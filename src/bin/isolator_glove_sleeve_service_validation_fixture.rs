use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Isolator glove/sleeve service and validation fixture.
//
// Intent:
// - Service glove and sleeve assemblies against a sealed fixture instead of
//   opening the culture process space.
// - Combine repeatable port-ring datum checks, sleeve leak checks, pressure
//   manifold placeholders, clean/used segregation, and traceability lands in
//   one station-sized package.
// - Keep VHP/cleanability coupons, puncture/abrasion witness lands, drip
//   capture, and robot/service keepout gauges explicit in the CAD envelope.
//
// This is packaging/fixture CAD for product planning. It is not a validated
// glove integrity test method, isolator certification, or sterile barrier spec.

const OUTPUTS: [&str; 12] = [
    "output/isolator_glove_sleeve_service_validation_fixture_base_leak_tray.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_glove_port_ring_datum.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_cuff_stretch_gauge.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_sealed_sleeve_leak_test_plate.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_pressure_manifold_panel.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_sterile_cap_plug_staging.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_clean_used_glove_segregation.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_vhp_cleanability_clearance_coupons.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_puncture_abrasion_witness_lands.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_barcode_run_record_lands.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_robot_service_keepouts.stl",
    "output/isolator_glove_sleeve_service_validation_fixture_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "glove_port_ring_datum",
    "cuff_stretch_gauge",
    "sealed_sleeve_leak_test_plate",
    "positive_negative_pressure_manifold",
    "sterile_cap_plug_staging",
    "clean_used_glove_segregation",
    "vhp_cleanability_clearance_coupons",
    "puncture_abrasion_witness_lands",
    "barcode_run_record_scan_lands",
    "drip_leak_tray",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 720.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SOCKET_DEPTH: f64 = 5.0;

const PORT_COUNT: usize = 2;
const PORT_PAIR_PITCH: f64 = 430.0;
const PORT_RING_ID: f64 = 238.0;
const PORT_RING_OD: f64 = 322.0;
const PORT_RING_Z: f64 = 28.0;
const PORT_DATUM_PLATE_X: f64 = 850.0;
const PORT_DATUM_PLATE_Y: f64 = 305.0;
const PORT_DATUM_PLATE_Z: f64 = 18.0;
const PORT_BOLT_COUNT_PER_RING: usize = 12;

const CUFF_GAUGE_X: f64 = 760.0;
const CUFF_GAUGE_Y: f64 = 108.0;
const CUFF_GAUGE_Z: f64 = 24.0;
const CUFF_STRETCH_STEPS: usize = 7;

const LEAK_PLATE_X: f64 = 720.0;
const LEAK_PLATE_Y: f64 = 248.0;
const LEAK_PLATE_Z: f64 = 22.0;
const LEAK_PLATE_PORT_COUNT: usize = PORT_COUNT;
const LEAK_GASKET_GROOVE_OD: f64 = 286.0;
const LEAK_GASKET_GROOVE_ID: f64 = 250.0;
const SLEEVE_CLAMP_LUGS_PER_PORT: usize = 8;

const MANIFOLD_X: f64 = 1030.0;
const MANIFOLD_Y: f64 = 38.0;
const MANIFOLD_Z: f64 = 245.0;
const PRESSURE_PORT_COUNT: usize = 6;
const GAUGE_COUNT: usize = 4;
const VALVE_PLACEHOLDER_COUNT: usize = 8;

const CAP_TRAY_X: f64 = 352.0;
const CAP_TRAY_Y: f64 = 154.0;
const CAP_TRAY_Z: f64 = 34.0;
const STERILE_CAP_WELLS: usize = 16;
const STERILE_PLUG_POSTS: usize = 12;

const SEG_TRAY_X: f64 = 454.0;
const SEG_TRAY_Y: f64 = 156.0;
const SEG_TRAY_Z: f64 = 42.0;
const CLEAN_GLOVE_CRADLES: usize = 4;
const USED_GLOVE_BAG_WELLS: usize = 4;

const COUPON_RACK_X: f64 = 218.0;
const COUPON_RACK_Y: f64 = 222.0;
const COUPON_RACK_Z: f64 = 48.0;
const VHP_COUPON_COUNT: usize = 8;
const CLEANABILITY_GAP_COUPONS: usize = 4;

const WITNESS_PANEL_X: f64 = 218.0;
const WITNESS_PANEL_Y: f64 = 222.0;
const WITNESS_PANEL_Z: f64 = 22.0;
const PUNCTURE_WITNESS_LANDS: usize = 6;
const ABRASION_WITNESS_LANDS: usize = 6;

const LABEL_PANEL_X: f64 = 330.0;
const LABEL_PANEL_Y: f64 = 124.0;
const LABEL_PANEL_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 8;
const RUN_RECORD_LANDS: usize = 2;

const KEEP_OUT_Z: f64 = 265.0;
const KEEP_OUT_ZONE_COUNT: usize = 4;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 10.0;
const TUBE_BORE_D: f64 = 6.0;

const RING_POS: (f64, f64) = (0.0, 106.0);
const LEAK_PLATE_POS: (f64, f64) = (0.0, -84.0);
const MANIFOLD_POS: (f64, f64) = (0.0, 312.0);
const CAP_POS: (f64, f64) = (-344.0, -252.0);
const SEG_POS: (f64, f64) = (84.0, -252.0);
const COUPON_POS: (f64, f64) = (452.0, -72.0);
const WITNESS_POS: (f64, f64) = (-452.0, -72.0);
const LABEL_POS: (f64, f64) = (392.0, -252.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let ring_datum = glove_port_ring_datum();
    export(&ring_datum, OUTPUTS[1]);

    let cuff_gauge = cuff_stretch_gauge();
    export(&cuff_gauge, OUTPUTS[2]);

    let leak_plate = sealed_sleeve_leak_test_plate();
    export(&leak_plate, OUTPUTS[3]);

    let manifold = pressure_manifold_panel();
    export(&manifold, OUTPUTS[4]);

    let cap_staging = sterile_cap_plug_staging();
    export(&cap_staging, OUTPUTS[5]);

    let segregation = clean_used_glove_segregation();
    export(&segregation, OUTPUTS[6]);

    let coupons = vhp_cleanability_clearance_coupons();
    export(&coupons, OUTPUTS[7]);

    let witness = puncture_abrasion_witness_lands();
    export(&witness, OUTPUTS[8]);

    let labels = barcode_run_record_lands();
    export(&labels, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + ring_datum.translate(RING_POS.0, RING_POS.1, deck_top_z())
        + cuff_gauge.translate(RING_POS.0, RING_POS.1 - 164.0, deck_top_z() + 2.0)
        + leak_plate.translate(LEAK_PLATE_POS.0, LEAK_PLATE_POS.1, deck_top_z() + 5.0)
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, deck_top_z())
        + cap_staging.translate(CAP_POS.0, CAP_POS.1, deck_top_z() + 2.0)
        + segregation.translate(SEG_POS.0, SEG_POS.1, deck_top_z() + 2.0)
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, deck_top_z() + 2.0)
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, deck_top_z() + 2.0)
        + labels.translate(LABEL_POS.0, LABEL_POS.1, deck_top_z() + 2.0)
        + keepouts.translate(0.0, 0.0, deck_top_z());
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Isolator glove/sleeve service validation fixture:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm deck");
    println!(
        "  Glove port datum:       {PORT_COUNT} ports, {PORT_RING_ID:.0}mm ID / {PORT_RING_OD:.0}mm OD rings, {PORT_BOLT_COUNT_PER_RING} bolt datum points per port"
    );
    println!(
        "  Sleeve test plate:      {LEAK_PLATE_PORT_COUNT} sealed sleeves with gasket grooves and {} clamp lugs",
        LEAK_PLATE_PORT_COUNT * SLEEVE_CLAMP_LUGS_PER_PORT
    );
    println!(
        "  Pressure controls:      {PRESSURE_PORT_COUNT} quick-connect ports, {GAUGE_COUNT} gauge pockets, {VALVE_PLACEHOLDER_COUNT} valve placeholders"
    );
    println!(
        "  Consumable handling:    {STERILE_CAP_WELLS} sterile cap wells, {STERILE_PLUG_POSTS} plug posts, {CLEAN_GLOVE_CRADLES} clean glove cradles, {USED_GLOVE_BAG_WELLS} used glove bag wells"
    );
    println!(
        "  Evidence capture:       {VHP_COUPON_COUNT} VHP/cleanability coupons, {CLEANABILITY_GAP_COUPONS} gap coupons, {PUNCTURE_WITNESS_LANDS} puncture lands, {ABRASION_WITNESS_LANDS} abrasion lands, {BARCODE_LANDS} barcode lands, {RUN_RECORD_LANDS} run-record lands"
    );
    println!(
        "  Covered feature groups: {} including drip/leak capture and {KEEP_OUT_ZONE_COUNT} robot/service keepout gauges",
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 7] {
    [
        (
            "glove_port_ring_datum",
            RING_POS,
            PORT_DATUM_PLATE_X,
            PORT_DATUM_PLATE_Y,
        ),
        (
            "sealed_sleeve_leak_test_plate",
            LEAK_PLATE_POS,
            LEAK_PLATE_X,
            LEAK_PLATE_Y,
        ),
        (
            "pressure_manifold_panel",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        ("cap_plug_staging", CAP_POS, CAP_TRAY_X, CAP_TRAY_Y),
        ("clean_used_segregation", SEG_POS, SEG_TRAY_X, SEG_TRAY_Y),
        ("coupon_rack", COUPON_POS, COUPON_RACK_X, COUPON_RACK_Y),
        (
            "witness_panel",
            WITNESS_POS,
            WITNESS_PANEL_X,
            WITNESS_PANEL_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "isolator_glove_fixture_base_leak_tray_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let shallow_basin = centered_cube(
        "isolator_glove_fixture_washdown_basin_recess",
        STATION_X - 118.0,
        STATION_Y - 116.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.0);
    let front_drain = centered_cylinder(
        "isolator_glove_fixture_front_drain_to_closed_waste",
        DRAIN_D / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 - 3.0, -1.0);

    deck - shallow_basin - front_drain - module_sockets() - mounting_slots()
        + perimeter_rims()
        + clean_dirty_lane_ribs()
        + base_robot_fiducials()
        + leak_sensor_wells()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("isolator_glove_fixture_module_registration_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("isolator_glove_fixture_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("isolator_glove_fixture_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (0.0, STATION_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("isolator_glove_fixture_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("isolator_glove_fixture_mount_slot_relief_{i}"),
            26.0,
            MOUNT_HOLE_D + 0.5,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "isolator_glove_fixture_left_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "isolator_glove_fixture_right_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "isolator_glove_fixture_rear_manifold_rim",
        STATION_X - 64.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 34.0, BASE_Z / 2.0 + RIM_Z / 2.0);
    let front = centered_cube(
        "isolator_glove_fixture_front_low_drip_lip",
        STATION_X - 150.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 30.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front
}

fn clean_dirty_lane_ribs() -> Part {
    let center = centered_cube(
        "isolator_glove_fixture_clean_dirty_center_lane_rib",
        12.0,
        260.0,
        28.0,
    )
    .translate(-132.0, -250.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "isolator_glove_fixture_manifold_zone_rib",
        STATION_X - 140.0,
        10.0,
        24.0,
    )
    .translate(0.0, 235.0, BASE_Z / 2.0 + 12.0);
    let service = centered_cube(
        "isolator_glove_fixture_leak_plate_service_rib",
        STATION_X - 180.0,
        8.0,
        20.0,
    )
    .translate(0.0, -198.0, BASE_Z / 2.0 + 10.0);

    center + rear + service
}

fn base_robot_fiducials() -> Part {
    let mut targets = Part::empty("isolator_glove_fixture_robot_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 78.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 78.0),
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 78.0)),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(&format!("isolator_glove_fixture_robot_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    targets
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("isolator_glove_fixture_leak_sensor_wells");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 92.0), -STATION_Y / 2.0 + 70.0),
        (0.0, -STATION_Y / 2.0 + 70.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 70.0),
    ]
    .iter()
    .enumerate()
    {
        let rim = centered_cube(
            format!("isolator_glove_fixture_leak_sensor_well_rim_{i}"),
            54.0,
            34.0,
            8.0,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        let pocket = centered_cube(
            format!("isolator_glove_fixture_leak_sensor_pocket_{i}"),
            38.0,
            20.0,
            10.0,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        wells = wells + (rim - pocket);
    }
    wells
}

fn glove_port_ring_datum() -> Part {
    let plate = centered_cube(
        "isolator_glove_fixture_port_datum_backing_plate",
        PORT_DATUM_PLATE_X,
        PORT_DATUM_PLATE_Y,
        PORT_DATUM_PLATE_Z,
    )
    .translate(0.0, 0.0, PORT_DATUM_PLATE_Z / 2.0);
    let mut datum = plate - port_apertures(PORT_DATUM_PLATE_Z + 4.0);

    for port in 0..PORT_COUNT {
        let x = centered_index(port, PORT_COUNT, PORT_PAIR_PITCH);
        datum = datum
            + single_port_ring(port).translate(x, 0.0, PORT_DATUM_PLATE_Z)
            + datum_flat_blocks(port).translate(x, 0.0, PORT_DATUM_PLATE_Z + 2.0)
            + sleeve_retainer_lips(port).translate(x, 0.0, PORT_DATUM_PLATE_Z + 8.0);
    }

    datum + port_side_datum_rails() + ring_id_tags()
}

fn port_apertures(height: f64) -> Part {
    let mut apertures = Part::empty("isolator_glove_fixture_port_apertures");
    for port in 0..PORT_COUNT {
        let x = centered_index(port, PORT_COUNT, PORT_PAIR_PITCH);
        apertures = apertures
            + centered_cylinder(
                format!("isolator_glove_fixture_port_aperture_{port}"),
                PORT_RING_ID / 2.0,
                height,
                96,
            )
            .translate(x, 0.0, height / 2.0 - 2.0);
    }
    apertures
}

fn single_port_ring(port: usize) -> Part {
    let ring = centered_cylinder(
        format!("isolator_glove_fixture_port_{port}_outer_ring"),
        PORT_RING_OD / 2.0,
        PORT_RING_Z,
        128,
    )
    .translate(0.0, 0.0, PORT_RING_Z / 2.0);
    let bore = centered_cylinder(
        format!("isolator_glove_fixture_port_{port}_operator_bore"),
        PORT_RING_ID / 2.0,
        PORT_RING_Z + 4.0,
        128,
    )
    .translate(0.0, 0.0, PORT_RING_Z / 2.0);
    let gasket_groove = centered_cylinder(
        format!("isolator_glove_fixture_port_{port}_gasket_groove"),
        PORT_RING_ID / 2.0 + 14.0,
        6.0,
        128,
    )
    .translate(0.0, 0.0, PORT_RING_Z - 2.0);
    let inner_land = centered_cylinder(
        format!("isolator_glove_fixture_port_{port}_gasket_inner_keep"),
        PORT_RING_ID / 2.0 + 5.0,
        8.0,
        128,
    )
    .translate(0.0, 0.0, PORT_RING_Z - 2.0);
    let mut bolt_holes = Part::empty(format!("isolator_glove_fixture_port_{port}_bolt_holes"));
    for i in 0..PORT_BOLT_COUNT_PER_RING {
        let angle = i as f64 * 360.0 / PORT_BOLT_COUNT_PER_RING as f64;
        let (x, y) = polar_point(142.0, angle);
        bolt_holes = bolt_holes
            + centered_cylinder(
                format!("isolator_glove_fixture_port_{port}_m5_datum_hole_{i}"),
                5.4 / 2.0,
                PORT_RING_Z + 6.0,
                20,
            )
            .translate(x, y, PORT_RING_Z / 2.0);
    }

    ring - bore - (gasket_groove - inner_land) - bolt_holes
}

fn datum_flat_blocks(port: usize) -> Part {
    let top_flat = centered_cube(
        format!("isolator_glove_fixture_port_{port}_top_clocking_flat"),
        118.0,
        16.0,
        18.0,
    )
    .translate(0.0, PORT_RING_OD / 2.0 - 16.0, 9.0);
    let bottom_flat = centered_cube(
        format!("isolator_glove_fixture_port_{port}_bottom_clocking_flat"),
        118.0,
        16.0,
        18.0,
    )
    .translate(0.0, -(PORT_RING_OD / 2.0 - 16.0), 9.0);
    let left_witness = centered_cube(
        format!("isolator_glove_fixture_port_{port}_left_witness_tab"),
        22.0,
        76.0,
        14.0,
    )
    .translate(-(PORT_RING_OD / 2.0 - 8.0), 0.0, 7.0);
    let right_witness = centered_cube(
        format!("isolator_glove_fixture_port_{port}_right_witness_tab"),
        22.0,
        76.0,
        14.0,
    )
    .translate(PORT_RING_OD / 2.0 - 8.0, 0.0, 7.0);

    top_flat + bottom_flat + left_witness + right_witness
}

fn sleeve_retainer_lips(port: usize) -> Part {
    let upper = centered_cube(
        format!("isolator_glove_fixture_port_{port}_upper_sleeve_retainer_lip"),
        240.0,
        18.0,
        20.0,
    )
    .translate(0.0, PORT_RING_OD / 2.0 + 20.0, 10.0);
    let lower = centered_cube(
        format!("isolator_glove_fixture_port_{port}_lower_sleeve_retainer_lip"),
        240.0,
        18.0,
        20.0,
    )
    .translate(0.0, -(PORT_RING_OD / 2.0 + 20.0), 10.0);
    upper + lower
}

fn port_side_datum_rails() -> Part {
    let rear_rail = centered_cube(
        "isolator_glove_fixture_port_datum_rear_seal_rail",
        PORT_DATUM_PLATE_X - 80.0,
        16.0,
        34.0,
    )
    .translate(
        0.0,
        PORT_DATUM_PLATE_Y / 2.0 - 16.0,
        PORT_DATUM_PLATE_Z + 17.0,
    );
    let front_rail = centered_cube(
        "isolator_glove_fixture_port_datum_front_drip_rail",
        PORT_DATUM_PLATE_X - 140.0,
        12.0,
        24.0,
    )
    .translate(
        0.0,
        -(PORT_DATUM_PLATE_Y / 2.0 - 18.0),
        PORT_DATUM_PLATE_Z + 12.0,
    );
    rear_rail + front_rail
}

fn ring_id_tags() -> Part {
    let mut tags = Part::empty("isolator_glove_fixture_ring_id_tags");
    for port in 0..PORT_COUNT {
        let x = centered_index(port, PORT_COUNT, PORT_PAIR_PITCH);
        tags = tags
            + centered_cube(
                format!("isolator_glove_fixture_ring_{port}_engrave_land"),
                86.0,
                28.0,
                4.0,
            )
            .translate(
                x,
                -(PORT_DATUM_PLATE_Y / 2.0 - 36.0),
                PORT_DATUM_PLATE_Z + 2.0,
            );
    }
    tags
}

fn cuff_stretch_gauge() -> Part {
    let board = centered_cube(
        "isolator_glove_fixture_cuff_stretch_gauge_board",
        CUFF_GAUGE_X,
        CUFF_GAUGE_Y,
        CUFF_GAUGE_Z,
    )
    .translate(0.0, 0.0, CUFF_GAUGE_Z / 2.0);
    let mut steps = Part::empty("isolator_glove_fixture_cuff_stretch_steps");
    for i in 0..CUFF_STRETCH_STEPS {
        let width = 42.0 + i as f64 * 12.0;
        steps = steps
            + centered_cube(
                format!("isolator_glove_fixture_cuff_stretch_step_slot_{i}"),
                width,
                62.0,
                CUFF_GAUGE_Z + 4.0,
            )
            .translate(
                centered_index(i, CUFF_STRETCH_STEPS, 92.0),
                -18.0,
                CUFF_GAUGE_Z / 2.0,
            );
    }

    let mut mandrels = Part::empty("isolator_glove_fixture_cuff_round_mandrels");
    for (i, d) in [164.0, 188.0, 212.0].iter().enumerate() {
        mandrels = mandrels
            + (centered_cylinder(
                format!("isolator_glove_fixture_cuff_round_mandrel_{i}"),
                *d / 2.0,
                16.0,
                80,
            ) - centered_cylinder(
                format!("isolator_glove_fixture_cuff_round_mandrel_bore_{i}"),
                (*d - 24.0) / 2.0,
                18.0,
                80,
            ))
            .translate(centered_index(i, 3, 210.0), 30.0, CUFF_GAUGE_Z + 8.0);
    }

    let witness_line = centered_cube(
        "isolator_glove_fixture_cuff_nominal_extension_witness_line",
        CUFF_GAUGE_X - 72.0,
        4.0,
        6.0,
    )
    .translate(0.0, -CUFF_GAUGE_Y / 2.0 + 18.0, CUFF_GAUGE_Z + 3.0);

    board - steps + mandrels + witness_line
}

fn sealed_sleeve_leak_test_plate() -> Part {
    let plate = centered_cube(
        "isolator_glove_fixture_sleeve_leak_test_plate",
        LEAK_PLATE_X,
        LEAK_PLATE_Y,
        LEAK_PLATE_Z,
    )
    .translate(0.0, 0.0, LEAK_PLATE_Z / 2.0);
    let mut body = plate - leak_plate_apertures();

    for port in 0..LEAK_PLATE_PORT_COUNT {
        let x = centered_index(port, LEAK_PLATE_PORT_COUNT, 330.0);
        body = body
            + leak_plate_clamp_lugs(port).translate(x, 0.0, LEAK_PLATE_Z)
            + leak_test_port_bosses(port).translate(x, 0.0, LEAK_PLATE_Z);
    }

    body + leak_plate_drain_channels() + leak_plate_handles()
}

fn leak_plate_apertures() -> Part {
    let mut cuts = Part::empty("isolator_glove_fixture_sleeve_plate_apertures");
    for port in 0..LEAK_PLATE_PORT_COUNT {
        let x = centered_index(port, LEAK_PLATE_PORT_COUNT, 330.0);
        let bore = centered_cylinder(
            format!("isolator_glove_fixture_sleeve_plate_bore_{port}"),
            LEAK_GASKET_GROOVE_ID / 2.0,
            LEAK_PLATE_Z + 4.0,
            96,
        )
        .translate(x, 0.0, LEAK_PLATE_Z / 2.0);
        let groove = centered_cylinder(
            format!("isolator_glove_fixture_sleeve_plate_gasket_outer_groove_{port}"),
            LEAK_GASKET_GROOVE_OD / 2.0,
            6.0,
            96,
        )
        .translate(x, 0.0, LEAK_PLATE_Z - 2.0);
        let groove_keep = centered_cylinder(
            format!("isolator_glove_fixture_sleeve_plate_gasket_inner_keep_{port}"),
            LEAK_GASKET_GROOVE_ID / 2.0 + 8.0,
            8.0,
            96,
        )
        .translate(x, 0.0, LEAK_PLATE_Z - 2.0);
        cuts = cuts + bore + (groove - groove_keep);
    }
    cuts
}

fn leak_plate_clamp_lugs(port: usize) -> Part {
    let mut lugs = Part::empty(format!(
        "isolator_glove_fixture_sleeve_plate_clamp_lugs_{port}"
    ));
    for i in 0..SLEEVE_CLAMP_LUGS_PER_PORT {
        let angle = i as f64 * 360.0 / SLEEVE_CLAMP_LUGS_PER_PORT as f64 + 22.5;
        let (x, y) = polar_point(158.0, angle);
        let lug = centered_cube(
            format!("isolator_glove_fixture_sleeve_plate_clamp_lug_{port}_{i}"),
            34.0,
            22.0,
            18.0,
        )
        .translate(x, y, 9.0);
        let screw = centered_cylinder(
            format!("isolator_glove_fixture_sleeve_plate_clamp_lug_screw_{port}_{i}"),
            5.2 / 2.0,
            22.0,
            20,
        )
        .translate(x, y, 9.0);
        lugs = lugs + (lug - screw);
    }
    lugs
}

fn leak_test_port_bosses(port: usize) -> Part {
    let pressure_boss = centered_cube(
        format!("isolator_glove_fixture_sleeve_plate_pressure_boss_{port}"),
        64.0,
        30.0,
        22.0,
    )
    .translate(-72.0, -LEAK_PLATE_Y / 2.0 + 28.0, 11.0);
    let vacuum_boss = centered_cube(
        format!("isolator_glove_fixture_sleeve_plate_vacuum_boss_{port}"),
        64.0,
        30.0,
        22.0,
    )
    .translate(72.0, -LEAK_PLATE_Y / 2.0 + 28.0, 11.0);
    let pressure_bore = centered_cylinder(
        format!("isolator_glove_fixture_sleeve_plate_pressure_bore_{port}"),
        TUBE_BORE_D / 2.0,
        36.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-72.0, -LEAK_PLATE_Y / 2.0 + 28.0, 11.0);
    let vacuum_bore = centered_cylinder(
        format!("isolator_glove_fixture_sleeve_plate_vacuum_bore_{port}"),
        TUBE_BORE_D / 2.0,
        36.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(72.0, -LEAK_PLATE_Y / 2.0 + 28.0, 11.0);

    (pressure_boss - pressure_bore) + (vacuum_boss - vacuum_bore)
}

fn leak_plate_drain_channels() -> Part {
    let left = centered_cube(
        "isolator_glove_fixture_sleeve_plate_left_drip_channel",
        250.0,
        10.0,
        8.0,
    )
    .translate(-165.0, -LEAK_PLATE_Y / 2.0 + 22.0, LEAK_PLATE_Z + 4.0);
    let right = centered_cube(
        "isolator_glove_fixture_sleeve_plate_right_drip_channel",
        250.0,
        10.0,
        8.0,
    )
    .translate(165.0, -LEAK_PLATE_Y / 2.0 + 22.0, LEAK_PLATE_Z + 4.0);
    left + right
}

fn leak_plate_handles() -> Part {
    let mut handles = Part::empty("isolator_glove_fixture_sleeve_plate_handles");
    for (i, x) in [-LEAK_PLATE_X / 2.0 + 42.0, LEAK_PLATE_X / 2.0 - 42.0]
        .iter()
        .enumerate()
    {
        let handle = centered_cube(
            format!("isolator_glove_fixture_sleeve_plate_handle_{i}"),
            24.0,
            96.0,
            24.0,
        )
        .translate(*x, 0.0, LEAK_PLATE_Z + 12.0);
        let grip = centered_cube(
            format!("isolator_glove_fixture_sleeve_plate_handle_cut_{i}"),
            12.0,
            58.0,
            28.0,
        )
        .translate(*x, 0.0, LEAK_PLATE_Z + 12.0);
        handles = handles + (handle - grip);
    }
    handles
}

fn pressure_manifold_panel() -> Part {
    let backplate = centered_cube(
        "isolator_glove_fixture_pressure_manifold_backplate",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(0.0, 0.0, MANIFOLD_Z / 2.0);
    let positive_header = manifold_header("positive_pressure", -120.0, 164.0);
    let negative_header = manifold_header("negative_pressure", -120.0, 96.0);
    let purge_header = manifold_header("purge_reference", -120.0, 44.0);

    backplate - manifold_service_bores()
        + positive_header
        + negative_header
        + purge_header
        + pressure_gauge_pockets()
        + valve_placeholder_bank()
        + manifold_label_strips()
}

fn manifold_header(name: &str, x0: f64, z: f64) -> Part {
    let block = centered_cube(
        format!("isolator_glove_fixture_{name}_header_block"),
        690.0,
        26.0,
        28.0,
    )
    .translate(x0, -MANIFOLD_Y / 2.0 - 10.0, z);
    let bore = centered_cylinder(
        format!("isolator_glove_fixture_{name}_header_bore"),
        TUBE_BORE_D / 2.0,
        710.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x0, -MANIFOLD_Y / 2.0 - 10.0, z);
    block - bore
}

fn manifold_service_bores() -> Part {
    let mut bores = Part::empty("isolator_glove_fixture_manifold_service_bores");
    for i in 0..PRESSURE_PORT_COUNT {
        let x = centered_index(i, PRESSURE_PORT_COUNT, 88.0) - 120.0;
        bores = bores
            + centered_cylinder(
                format!("isolator_glove_fixture_manifold_quick_connect_bore_{i}"),
                9.0 / 2.0,
                MANIFOLD_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 166.0);
    }
    bores
}

fn pressure_gauge_pockets() -> Part {
    let mut gauges = Part::empty("isolator_glove_fixture_manifold_gauge_pockets");
    for i in 0..GAUGE_COUNT {
        let x = 260.0 + centered_index(i, GAUGE_COUNT, 72.0);
        let gauge_bezel = centered_cylinder(
            format!("isolator_glove_fixture_pressure_gauge_bezel_{i}"),
            28.0,
            12.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -MANIFOLD_Y / 2.0 - 7.0, 146.0);
        let gauge_face = centered_cylinder(
            format!("isolator_glove_fixture_pressure_gauge_face_recess_{i}"),
            21.0,
            14.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -MANIFOLD_Y / 2.0 - 7.0, 146.0);
        gauges = gauges + (gauge_bezel - gauge_face);
    }
    gauges
}

fn valve_placeholder_bank() -> Part {
    let mut valves = Part::empty("isolator_glove_fixture_manifold_valve_placeholders");
    for i in 0..VALVE_PLACEHOLDER_COUNT {
        let x = centered_index(i, VALVE_PLACEHOLDER_COUNT, 64.0) - 100.0;
        valves = valves
            + centered_cube(
                format!("isolator_glove_fixture_manifold_valve_block_{i}"),
                42.0,
                42.0,
                34.0,
            )
            .translate(x, -MANIFOLD_Y / 2.0 - 18.0, 72.0)
            - centered_cylinder(
                format!("isolator_glove_fixture_manifold_valve_stem_clearance_{i}"),
                6.0,
                48.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -MANIFOLD_Y / 2.0 - 18.0, 72.0);
    }
    valves
}

fn manifold_label_strips() -> Part {
    let positive = centered_cube(
        "isolator_glove_fixture_manifold_positive_label_land",
        198.0,
        8.0,
        20.0,
    )
    .translate(-418.0, -MANIFOLD_Y / 2.0 - 6.0, 202.0);
    let negative = centered_cube(
        "isolator_glove_fixture_manifold_negative_label_land",
        198.0,
        8.0,
        20.0,
    )
    .translate(-418.0, -MANIFOLD_Y / 2.0 - 6.0, 118.0);
    let relief = centered_cube(
        "isolator_glove_fixture_manifold_relief_label_land",
        198.0,
        8.0,
        20.0,
    )
    .translate(342.0, -MANIFOLD_Y / 2.0 - 6.0, 52.0);
    positive + negative + relief
}

fn sterile_cap_plug_staging() -> Part {
    let tray = centered_cube(
        "isolator_glove_fixture_sterile_cap_plug_staging_tray",
        CAP_TRAY_X,
        CAP_TRAY_Y,
        CAP_TRAY_Z,
    )
    .translate(0.0, 0.0, CAP_TRAY_Z / 2.0);
    let basin = centered_cube(
        "isolator_glove_fixture_cap_staging_recess",
        CAP_TRAY_X - 32.0,
        CAP_TRAY_Y - 30.0,
        14.0,
    )
    .translate(0.0, 0.0, CAP_TRAY_Z - 5.0);
    let mut wells = Part::empty("isolator_glove_fixture_sterile_cap_wells");
    for row in 0..4 {
        for col in 0..4 {
            let i = row * 4 + col;
            wells = wells
                + centered_cylinder(
                    format!("isolator_glove_fixture_sterile_cap_well_{i}"),
                    13.0,
                    18.0,
                    28,
                )
                .translate(
                    centered_index(col, 4, 42.0) - 78.0,
                    centered_index(row, 4, 32.0),
                    CAP_TRAY_Z - 5.0,
                );
        }
    }
    let mut posts = Part::empty("isolator_glove_fixture_sterile_plug_posts");
    for i in 0..STERILE_PLUG_POSTS {
        posts = posts
            + centered_cylinder(
                format!("isolator_glove_fixture_sterile_plug_post_{i}"),
                7.5,
                28.0,
                24,
            )
            .translate(
                94.0 + centered_index(i % 3, 3, 34.0),
                centered_index(i / 3, 4, 30.0),
                CAP_TRAY_Z + 14.0,
            );
    }
    let clean_cover_lip = centered_cube(
        "isolator_glove_fixture_staging_clean_cover_lip",
        CAP_TRAY_X - 24.0,
        10.0,
        18.0,
    )
    .translate(0.0, CAP_TRAY_Y / 2.0 - 14.0, CAP_TRAY_Z + 9.0);

    tray - basin - wells + posts + clean_cover_lip
}

fn clean_used_glove_segregation() -> Part {
    let tray = centered_cube(
        "isolator_glove_fixture_clean_used_glove_segregation_tray",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    )
    .translate(0.0, 0.0, SEG_TRAY_Z / 2.0);
    let clean_recess = centered_cube(
        "isolator_glove_fixture_clean_glove_recess",
        SEG_TRAY_X / 2.0 - 36.0,
        SEG_TRAY_Y - 34.0,
        16.0,
    )
    .translate(-(SEG_TRAY_X / 4.0), 0.0, SEG_TRAY_Z - 6.0);
    let used_recess = centered_cube(
        "isolator_glove_fixture_used_glove_recess",
        SEG_TRAY_X / 2.0 - 36.0,
        SEG_TRAY_Y - 34.0,
        18.0,
    )
    .translate(SEG_TRAY_X / 4.0, 0.0, SEG_TRAY_Z - 6.0);
    let bulkhead = centered_cube(
        "isolator_glove_fixture_clean_used_physical_bulkhead",
        18.0,
        SEG_TRAY_Y - 16.0,
        72.0,
    )
    .translate(0.0, 0.0, 36.0);

    tray - clean_recess - used_recess + bulkhead + clean_glove_cradles() + used_glove_bag_wells()
}

fn clean_glove_cradles() -> Part {
    let mut cradles = Part::empty("isolator_glove_fixture_clean_glove_cradles");
    for i in 0..CLEAN_GLOVE_CRADLES {
        let cradle = centered_cylinder(
            format!("isolator_glove_fixture_clean_glove_cradle_{i}"),
            23.0,
            148.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -124.0 + centered_index(i, CLEAN_GLOVE_CRADLES, 46.0),
            0.0,
            SEG_TRAY_Z + 9.0,
        );
        let flat = centered_cube(
            format!("isolator_glove_fixture_clean_glove_cradle_flat_{i}"),
            52.0,
            154.0,
            23.0,
        )
        .translate(
            -124.0 + centered_index(i, CLEAN_GLOVE_CRADLES, 46.0),
            0.0,
            SEG_TRAY_Z + 20.0,
        );
        cradles = cradles + (cradle - flat);
    }
    cradles
}

fn used_glove_bag_wells() -> Part {
    let mut wells = Part::empty("isolator_glove_fixture_used_glove_bag_wells");
    for i in 0..USED_GLOVE_BAG_WELLS {
        wells = wells
            + centered_cube(
                format!("isolator_glove_fixture_used_glove_sealed_bag_well_{i}"),
                62.0,
                44.0,
                18.0,
            )
            .translate(
                110.0 + centered_index(i % 2, 2, 78.0),
                centered_index(i / 2, 2, 54.0),
                SEG_TRAY_Z - 4.0,
            );
    }
    wells
}

fn vhp_cleanability_clearance_coupons() -> Part {
    let base = centered_cube(
        "isolator_glove_fixture_vhp_cleanability_coupon_rack_base",
        COUPON_RACK_X,
        COUPON_RACK_Y,
        COUPON_RACK_Z,
    )
    .translate(0.0, 0.0, COUPON_RACK_Z / 2.0);
    let mut coupon_slots = Part::empty("isolator_glove_fixture_vhp_coupon_slots");
    for i in 0..VHP_COUPON_COUNT {
        coupon_slots = coupon_slots
            + centered_cube(
                format!("isolator_glove_fixture_vhp_coupon_slot_{i}"),
                18.0,
                72.0,
                34.0,
            )
            .translate(
                centered_index(i % 4, 4, 42.0),
                centered_index(i / 4, 2, 86.0),
                COUPON_RACK_Z - 8.0,
            );
    }
    let mut gap_coupons = Part::empty("isolator_glove_fixture_cleanability_gap_coupons");
    for i in 0..CLEANABILITY_GAP_COUPONS {
        gap_coupons = gap_coupons
            + centered_cube(
                format!("isolator_glove_fixture_cleanability_gap_coupon_{i}"),
                34.0,
                10.0 + i as f64 * 4.0,
                58.0,
            )
            .translate(
                centered_index(i, CLEANABILITY_GAP_COUPONS, 42.0),
                -COUPON_RACK_Y / 2.0 + 26.0,
                COUPON_RACK_Z + 29.0,
            );
    }
    let coved_corner_coupon = centered_cylinder(
        "isolator_glove_fixture_coved_corner_cleanability_coupon",
        42.0,
        68.0,
        40,
    )
    .translate(0.0, COUPON_RACK_Y / 2.0 - 42.0, COUPON_RACK_Z + 34.0);

    base - coupon_slots + gap_coupons + coved_corner_coupon
}

fn puncture_abrasion_witness_lands() -> Part {
    let panel = centered_cube(
        "isolator_glove_fixture_puncture_abrasion_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    )
    .translate(0.0, 0.0, WITNESS_PANEL_Z / 2.0);
    let mut puncture = Part::empty("isolator_glove_fixture_puncture_witness_lands");
    for i in 0..PUNCTURE_WITNESS_LANDS {
        puncture = puncture
            + centered_cube(
                format!("isolator_glove_fixture_puncture_witness_pad_{i}"),
                42.0,
                28.0,
                6.0,
            )
            .translate(
                centered_index(i % 3, 3, 56.0),
                54.0 + centered_index(i / 3, 2, 36.0),
                WITNESS_PANEL_Z + 3.0,
            );
    }
    let mut abrasion = Part::empty("isolator_glove_fixture_abrasion_witness_lands");
    for i in 0..ABRASION_WITNESS_LANDS {
        abrasion = abrasion
            + centered_cube(
                format!("isolator_glove_fixture_abrasion_witness_lane_{i}"),
                150.0,
                7.0,
                5.0,
            )
            .translate(0.0, -72.0 + i as f64 * 18.0, WITNESS_PANEL_Z + 2.5);
    }
    let witness_clip_rail = centered_cube(
        "isolator_glove_fixture_witness_coupon_clip_rail",
        WITNESS_PANEL_X - 28.0,
        12.0,
        18.0,
    )
    .translate(0.0, -WITNESS_PANEL_Y / 2.0 + 18.0, WITNESS_PANEL_Z + 9.0);
    panel + puncture + abrasion + witness_clip_rail
}

fn barcode_run_record_lands() -> Part {
    let panel = centered_cube(
        "isolator_glove_fixture_barcode_run_record_panel",
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    )
    .translate(0.0, 0.0, LABEL_PANEL_Z / 2.0);
    let mut labels = Part::empty("isolator_glove_fixture_barcode_lands");
    for i in 0..BARCODE_LANDS {
        labels = labels
            + centered_cube(
                format!("isolator_glove_fixture_barcode_land_{i}"),
                68.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(i % 4, 4, 76.0),
                28.0 + centered_index(i / 4, 2, 34.0),
                LABEL_PANEL_Z + 2.0,
            );
    }
    let run_record_slot = centered_cube(
        "isolator_glove_fixture_run_record_tablet_slot",
        186.0,
        44.0,
        7.0,
    )
    .translate(-46.0, -34.0, LABEL_PANEL_Z + 3.5);
    let certificate_land = centered_cube(
        "isolator_glove_fixture_calibration_certificate_land",
        86.0,
        44.0,
        4.0,
    )
    .translate(108.0, -34.0, LABEL_PANEL_Z + 2.0);

    panel + labels + run_record_slot + certificate_land
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        "isolator_glove_fixture_front_robot_approach_keepout",
        STATION_X - 160.0,
        150.0,
        KEEP_OUT_Z,
        0.0,
        -278.0,
        KEEP_OUT_Z / 2.0,
    );
    let port_face = keepout_box(
        "isolator_glove_fixture_port_face_service_keepout",
        PORT_DATUM_PLATE_X + 70.0,
        PORT_DATUM_PLATE_Y + 78.0,
        160.0,
        RING_POS.0,
        RING_POS.1,
        80.0,
    );
    let rear_manifold = keepout_box(
        "isolator_glove_fixture_rear_manifold_service_keepout",
        MANIFOLD_X - 42.0,
        112.0,
        MANIFOLD_Z + 90.0,
        MANIFOLD_POS.0,
        MANIFOLD_POS.1 - 26.0,
        (MANIFOLD_Z + 90.0) / 2.0,
    );
    let overhead_camera = keepout_box(
        "isolator_glove_fixture_overhead_camera_clearance_keepout",
        560.0,
        420.0,
        44.0,
        0.0,
        -36.0,
        312.0,
    );
    front_robot + port_face + rear_manifold + overhead_camera
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Part {
    let front = centered_cube(format!("{name}_front_rail"), x, 8.0, 8.0).translate(
        dx,
        dy - y / 2.0,
        dz - z / 2.0 + 4.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, 8.0).translate(
        dx,
        dy + y / 2.0,
        dz - z / 2.0 + 4.0,
    );
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, 8.0).translate(
        dx - x / 2.0,
        dy,
        dz - z / 2.0 + 4.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, 8.0).translate(
        dx + x / 2.0,
        dy,
        dz - z / 2.0 + 4.0,
    );
    let posts = keepout_posts(name, x, y, z, dx, dy, dz);
    front + rear + left + right + posts
}

fn keepout_posts(name: &str, x: f64, y: f64, z: f64, dx: f64, dy: f64, dz: f64) -> Part {
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{i}"), 8.0, 8.0, z).translate(
                dx + sx * x / 2.0,
                dy + sy * y / 2.0,
                dz,
            );
    }
    posts
}

fn fiducial_target(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 18.0, 4.0, 36)
        - centered_cylinder(format!("{name}_center_dot"), 6.0, 6.0, 24)
        - centered_cube(format!("{name}_cross_x"), 32.0, 3.0, 6.0)
        - centered_cube(format!("{name}_cross_y"), 3.0, 32.0, 6.0)
}

fn polar_point(radius: f64, degrees: f64) -> (f64, f64) {
    let radians = degrees.to_radians();
    (radius * radians.cos(), radius * radians.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS
            .iter()
            .all(|path| path.contains("isolator_glove_sleeve_service_validation_fixture")));
    }

    #[test]
    fn required_feature_set_covers_service_risks() {
        for feature in [
            "glove_port_ring_datum",
            "sealed_sleeve_leak_test_plate",
            "positive_negative_pressure_manifold",
            "sterile_cap_plug_staging",
            "clean_used_glove_segregation",
            "vhp_cleanability_clearance_coupons",
            "puncture_abrasion_witness_lands",
            "barcode_run_record_scan_lands",
            "drip_leak_tray",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn glove_port_dimensions_are_plausible_for_isolator_service() {
        assert_eq!(PORT_COUNT, 2);
        assert!(PORT_RING_ID >= 220.0 && PORT_RING_ID <= 260.0);
        assert!(PORT_RING_OD - PORT_RING_ID >= 70.0);
        assert!(PORT_PAIR_PITCH > PORT_RING_OD);
        assert_eq!(PORT_BOLT_COUNT_PER_RING, 12);
    }

    #[test]
    fn consumable_and_evidence_capacity_is_nontrivial() {
        assert!(STERILE_CAP_WELLS >= 16);
        assert!(STERILE_PLUG_POSTS >= 12);
        assert!(VHP_COUPON_COUNT >= 8);
        assert!(PUNCTURE_WITNESS_LANDS + ABRASION_WITNESS_LANDS >= 12);
        assert!(BARCODE_LANDS >= 8);
    }

    #[test]
    fn all_major_modules_fit_inside_the_tray() {
        assert_layout();
    }

    #[test]
    fn pressure_panel_has_positive_negative_and_purge_capacity() {
        assert!(PRESSURE_PORT_COUNT >= 6);
        assert!(GAUGE_COUNT >= 4);
        assert!(VALVE_PLACEHOLDER_COUNT >= 8);
        assert!(MANIFOLD_Z >= 220.0);
    }
}
