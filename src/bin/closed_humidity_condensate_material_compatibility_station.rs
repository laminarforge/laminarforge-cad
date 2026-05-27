use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed humidity/condensate material compatibility validation station.
//
// This is mechanical validation packaging CAD only. It provides controlled
// locations for candidate coupons, condensate drip exposure hardware, cleaning
// residue witness lanes, dimensional/mass inspection nests, gasket witness
// pockets, traceability lands, disposition lanes, evidence capture geometry, and
// robot/service keepout gauges. It does not define a chemical compatibility
// protocol, cleaning chemistry, live-run release rule, or acceptance criterion.

const OUTPUTS: [&str; 13] = [
    "output/closed_humidity_condensate_material_compatibility_station_base_containment_tray.stl",
    "output/closed_humidity_condensate_material_compatibility_station_coupon_rack_cassettes.stl",
    "output/closed_humidity_condensate_material_compatibility_station_condensate_drip_challenge_manifold.stl",
    "output/closed_humidity_condensate_material_compatibility_station_heated_humidity_pocket_placeholders.stl",
    "output/closed_humidity_condensate_material_compatibility_station_cleaning_residue_coupon_lanes.stl",
    "output/closed_humidity_condensate_material_compatibility_station_mass_thickness_measurement_nests.stl",
    "output/closed_humidity_condensate_material_compatibility_station_seal_gasket_witness_pockets.stl",
    "output/closed_humidity_condensate_material_compatibility_station_barcode_certificate_lands.stl",
    "output/closed_humidity_condensate_material_compatibility_station_release_hold_reject_lanes.stl",
    "output/closed_humidity_condensate_material_compatibility_station_clean_used_segregation_bulkhead.stl",
    "output/closed_humidity_condensate_material_compatibility_station_evidence_camera_bridge.stl",
    "output/closed_humidity_condensate_material_compatibility_station_robot_service_keepout_gauges.stl",
    "output/closed_humidity_condensate_material_compatibility_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_containment_tray",
    "coupon_rack_cassettes",
    "condensate_drip_challenge_manifold",
    "heated_humidity_pocket_placeholders",
    "cleaning_residue_coupon_lanes",
    "mass_thickness_measurement_nests",
    "seal_gasket_witness_pockets",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const CLEAN_SIDE_LIMIT_X: f64 = -54.0;
const USED_SIDE_LIMIT_X: f64 = 54.0;

const RACK_X: f64 = 430.0;
const RACK_Y: f64 = 250.0;
const RACK_Z: f64 = 56.0;
const CASSETTE_COUNT: usize = 4;
const COUPONS_PER_CASSETTE: usize = 6;
const MATERIAL_COUPON_COUNT: usize = CASSETTE_COUNT * COUPONS_PER_CASSETTE;
const CASSETTE_X: f64 = 84.0;
const CASSETTE_Y: f64 = 210.0;
const CASSETTE_Z: f64 = 38.0;
const CASSETTE_PITCH_X: f64 = 96.0;
const COUPON_SLOT_X: f64 = 62.0;
const COUPON_SLOT_Y: f64 = 22.0;
const COUPON_SLOT_Z: f64 = 15.0;
const COUPON_SLOT_PITCH_Y: f64 = 30.0;
const RACK_POS: (f64, f64) = (-310.0, 190.0);

const MANIFOLD_X: f64 = 520.0;
const MANIFOLD_Y: f64 = 108.0;
const MANIFOLD_Z: f64 = 74.0;
const MANIFOLD_HEADER_D: f64 = 28.0;
const DRIP_NOZZLE_COUNT: usize = MATERIAL_COUPON_COUNT;
const NOZZLE_D: f64 = 5.0;
const NOZZLE_Z: f64 = 38.0;
const MANIFOLD_POS: (f64, f64) = (-270.0, 292.0);

const HUMIDITY_POCKET_X: f64 = 310.0;
const HUMIDITY_POCKET_Y: f64 = 180.0;
const HUMIDITY_POCKET_Z: f64 = 52.0;
const HUMIDITY_POCKET_COUNT: usize = 4;
const HUMIDITY_WELL_D: f64 = 58.0;
const HUMIDITY_WELL_DEPTH: f64 = 34.0;
const HUMIDITY_POS: (f64, f64) = (310.0, 205.0);

const RESIDUE_LANE_X: f64 = 430.0;
const RESIDUE_LANE_Y: f64 = 150.0;
const RESIDUE_LANE_Z: f64 = 34.0;
const RESIDUE_LANE_COUNT: usize = 8;
const RESIDUE_LANE_PITCH_X: f64 = 48.0;
const RESIDUE_POS: (f64, f64) = (-310.0, 20.0);

const MEASURE_X: f64 = 330.0;
const MEASURE_Y: f64 = 150.0;
const MEASURE_Z: f64 = 38.0;
const MASS_NEST_COUNT: usize = 4;
const THICKNESS_NEST_COUNT: usize = 4;
const MEASUREMENT_NEST_COUNT: usize = MASS_NEST_COUNT + THICKNESS_NEST_COUNT;
const MEASURE_POS: (f64, f64) = (310.0, 20.0);

const SEAL_X: f64 = 380.0;
const SEAL_Y: f64 = 135.0;
const SEAL_Z: f64 = 34.0;
const GASKET_WITNESS_COUNT: usize = 10;
const GASKET_POCKET_D: f64 = 34.0;
const GASKET_POCKET_DEPTH: f64 = 18.0;
const SEAL_POS: (f64, f64) = (-310.0, -170.0);

const TRACE_X: f64 = 300.0;
const TRACE_Y: f64 = 120.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 8;
const CERTIFICATE_LAND_COUNT: usize = 3;
const TRACE_POS: (f64, f64) = (310.0, -170.0);

const LANE_PLATE_X: f64 = 500.0;
const LANE_PLATE_Y: f64 = 132.0;
const LANE_PLATE_Z: f64 = 22.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const LANE_POS: (f64, f64) = (-150.0, -284.0);

const BULKHEAD_X: f64 = 18.0;
const BULKHEAD_Y: f64 = 650.0;
const BULKHEAD_Z: f64 = 82.0;
const BULKHEAD_PASS_THROUGH_COUNT: usize = 3;
const BULKHEAD_POS: (f64, f64) = (0.0, 0.0);

const CAMERA_BRIDGE_X: f64 = 920.0;
const CAMERA_BRIDGE_Y: f64 = 70.0;
const CAMERA_BRIDGE_Z: f64 = 176.0;
const CAMERA_POST_COUNT: usize = 4;
const CAMERA_LAND_COUNT: usize = 5;
const CAMERA_POS: (f64, f64) = (0.0, -78.0);

const KEEP_OUT_X: f64 = 1130.0;
const KEEP_OUT_Y: f64 = 710.0;
const KEEP_OUT_Z: f64 = 5.0;
const FRONT_ROBOT_APPROACH_CLEARANCE: f64 = 28.0;
const REAR_SERVICE_SWEEP_CLEARANCE: f64 = 12.0;
const SIDE_SERVICE_SWEEP_CLEARANCE: f64 = 30.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_tray();
    export(&base, OUTPUTS[0]);

    let rack = coupon_rack_cassettes();
    export(&rack, OUTPUTS[1]);

    let manifold = condensate_drip_challenge_manifold();
    export(&manifold, OUTPUTS[2]);

    let humidity = heated_humidity_pocket_placeholders();
    export(&humidity, OUTPUTS[3]);

    let residue = cleaning_residue_coupon_lanes();
    export(&residue, OUTPUTS[4]);

    let measurement = mass_thickness_measurement_nests();
    export(&measurement, OUTPUTS[5]);

    let seals = seal_gasket_witness_pockets();
    export(&seals, OUTPUTS[6]);

    let traceability = barcode_certificate_lands();
    export(&traceability, OUTPUTS[7]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[8]);

    let bulkhead = clean_used_segregation_bulkhead();
    export(&bulkhead, OUTPUTS[9]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[10]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, on_base_z(RACK_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_base_z(MANIFOLD_Z))
        + humidity.translate(HUMIDITY_POS.0, HUMIDITY_POS.1, on_base_z(HUMIDITY_POCKET_Z))
        + residue.translate(RESIDUE_POS.0, RESIDUE_POS.1, on_base_z(RESIDUE_LANE_Z))
        + measurement.translate(MEASURE_POS.0, MEASURE_POS.1, on_base_z(MEASURE_Z))
        + seals.translate(SEAL_POS.0, SEAL_POS.1, on_base_z(SEAL_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_base_z(TRACE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, on_base_z(LANE_PLATE_Z))
        + bulkhead.translate(BULKHEAD_POS.0, BULKHEAD_POS.1, on_base_z(BULKHEAD_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_base_z(CAMERA_BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed humidity/condensate material compatibility validation station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray");
    println!(
        "  Coupon exposure:        {MATERIAL_COUPON_COUNT} candidate material coupon slots across {CASSETTE_COUNT} removable cassettes with {DRIP_NOZZLE_COUNT} indexed drip nozzles"
    );
    println!(
        "  Humidity/residue:       {HUMIDITY_POCKET_COUNT} heated humidity pocket placeholders and {RESIDUE_LANE_COUNT} cleaning-residue coupon lanes"
    );
    println!(
        "  Inspection packaging:   {MEASUREMENT_NEST_COUNT} mass/thickness nests, {GASKET_WITNESS_COUNT} gasket witness pockets, {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands"
    );
    println!(
        "  Disposition/evidence:   {DISPOSITION_LANE_COUNT} release/hold/reject lanes, clean/used bulkhead, {CAMERA_LAND_COUNT} camera lands, and robot/service keepout gauges"
    );
    println!("  Required features:      {}", REQUIRED_FEATURES.len());
    println!("  Limitation:             Mechanical fixture CAD only; no chemistry, protocol, or acceptance criterion is encoded.");
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_base_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(MATERIAL_COUPON_COUNT, 24);
    assert_eq!(DRIP_NOZZLE_COUNT, MATERIAL_COUPON_COUNT);
    assert_eq!(MEASUREMENT_NEST_COUNT, 8);
    assert_eq!(DISPOSITION_LANE_COUNT, 3);
    assert_eq!(CAMERA_POST_COUNT, 4);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert!(containment_freeboard_volume_ml() > maximum_challenge_hold_up_ml());
    assert!(front_robot_approach_clearance() >= FRONT_ROBOT_APPROACH_CLEARANCE);
    assert!(rear_service_sweep_clearance() >= REAR_SERVICE_SWEEP_CLEARANCE);
    assert!(side_service_sweep_clearance() >= SIDE_SERVICE_SWEEP_CLEARANCE);

    for (name, pos, width, depth) in module_specs() {
        assert!(
            fits_inside_containment(pos, width, depth),
            "{name} exceeds containment tray"
        );
    }
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        ("coupon_rack_cassettes", RACK_POS, RACK_X, RACK_Y),
        (
            "condensate_drip_challenge_manifold",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        (
            "heated_humidity_pocket_placeholders",
            HUMIDITY_POS,
            HUMIDITY_POCKET_X,
            HUMIDITY_POCKET_Y,
        ),
        (
            "cleaning_residue_coupon_lanes",
            RESIDUE_POS,
            RESIDUE_LANE_X,
            RESIDUE_LANE_Y,
        ),
        (
            "mass_thickness_measurement_nests",
            MEASURE_POS,
            MEASURE_X,
            MEASURE_Y,
        ),
        ("seal_gasket_witness_pockets", SEAL_POS, SEAL_X, SEAL_Y),
        ("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        (
            "release_hold_reject_lanes",
            LANE_POS,
            LANE_PLATE_X,
            LANE_PLATE_Y,
        ),
        (
            "clean_used_segregation_bulkhead",
            BULKHEAD_POS,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        (
            "evidence_camera_bridge",
            CAMERA_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_inside_containment(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn front_robot_approach_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_PLATE_Y / 2.0)
}

fn rear_service_sweep_clearance() -> f64 {
    STATION_Y / 2.0 - (MANIFOLD_POS.1 + MANIFOLD_Y / 2.0)
}

fn side_service_sweep_clearance() -> f64 {
    STATION_X / 2.0 - (HUMIDITY_POS.0 + HUMIDITY_POCKET_X / 2.0)
}

fn containment_freeboard_volume_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let usable_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * usable_z / 1000.0
}

fn maximum_challenge_hold_up_ml() -> f64 {
    let coupon_wetting = MATERIAL_COUPON_COUNT as f64 * 8.0;
    let lane_hold_up = RESIDUE_LANE_COUNT as f64 * 12.0;
    let gasket_hold_up = GASKET_WITNESS_COUNT as f64 * 6.0;
    coupon_wetting + lane_hold_up + gasket_hold_up
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "humidity_condensate_material_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let shallow_basin = centered_cube(
        "humidity_condensate_material_base_secondary_basin_cut",
        STATION_X - 122.0,
        STATION_Y - 112.0,
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -4.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.3);
    let front_drain = centered_cylinder(
        "humidity_condensate_material_base_front_drain_bore",
        12.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 8.0);

    deck - shallow_basin - front_drain - base_locator_sockets() - base_mount_slots()
        + containment_rims()
        + station_zone_dividers()
        + station_datum_targets()
        + base_condensate_witness_ribs()
}

fn base_locator_sockets() -> Part {
    let mut sockets = Part::empty("humidity_condensate_material_base_locator_sockets");
    for (name, pos, width, depth) in module_specs() {
        if name == "evidence_camera_bridge" {
            continue;
        }
        sockets = sockets
            + centered_cube(
                format!("humidity_condensate_material_{name}_locator_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(pos.0, pos.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.3);
    }
    sockets
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("humidity_condensate_material_base_mount_slots");
    for (i, (x, y)) in mount_slot_positions().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("humidity_condensate_material_base_m6_clearance_{i}"),
                6.8 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0)
            + centered_cube(
                format!("humidity_condensate_material_base_service_slot_{i}"),
                30.0,
                7.5,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-196.0, STATION_Y / 2.0 - 58.0),
        (196.0, STATION_Y / 2.0 - 58.0),
        (-196.0, -(STATION_Y / 2.0 - 58.0)),
        (196.0, -(STATION_Y / 2.0 - 58.0)),
    ]
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "humidity_condensate_material_base_left_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "humidity_condensate_material_base_right_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "humidity_condensate_material_base_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "humidity_condensate_material_base_front_robot_low_lip",
        STATION_X - 176.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn station_zone_dividers() -> Part {
    let exposure_row = centered_cube(
        "humidity_condensate_material_exposure_row_divider",
        STATION_X - 172.0,
        9.0,
        24.0,
    )
    .translate(-10.0, 98.0, BASE_Z / 2.0 + 12.0);
    let inspection_row = centered_cube(
        "humidity_condensate_material_inspection_row_divider",
        STATION_X - 190.0,
        9.0,
        22.0,
    )
    .translate(-10.0, -92.0, BASE_Z / 2.0 + 11.0);
    let disposition_row = centered_cube(
        "humidity_condensate_material_disposition_row_divider",
        STATION_X - 220.0,
        8.0,
        18.0,
    )
    .translate(-20.0, -224.0, BASE_Z / 2.0 + 9.0);
    let clean_used_datum = centered_cube(
        "humidity_condensate_material_clean_used_center_datum_rib",
        8.0,
        STATION_Y - 142.0,
        20.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + 10.0);

    exposure_row + inspection_row + disposition_row + clean_used_datum
}

fn station_datum_targets() -> Part {
    let mut targets = Part::empty("humidity_condensate_material_robot_datum_targets");
    for (i, (x, y)) in [
        (-500.0, 302.0),
        (500.0, 302.0),
        (-500.0, -304.0),
        (500.0, -304.0),
    ]
    .iter()
    .enumerate()
    {
        targets =
            targets
                + fiducial_disc(&format!("humidity_condensate_material_base_fiducial_{i}"))
                    .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn base_condensate_witness_ribs() -> Part {
    let mut ribs = Part::empty("humidity_condensate_material_base_flow_witness_ribs");
    for (i, y) in [-198.0, -146.0, -44.0, 52.0, 146.0, 236.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("humidity_condensate_material_base_witness_rib_{i}"),
                STATION_X - 210.0,
                4.0,
                5.0,
            )
            .translate(-18.0, *y, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn coupon_rack_cassettes() -> Part {
    let base = centered_cube(
        "humidity_condensate_material_coupon_rack_base_plate",
        RACK_X,
        RACK_Y,
        16.0,
    )
    .translate(0.0, 0.0, -RACK_Z / 2.0 + 8.0);
    let rear_rail = centered_cube(
        "humidity_condensate_material_coupon_rack_rear_cassette_rail",
        RACK_X - 24.0,
        12.0,
        26.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 16.0, -RACK_Z / 2.0 + 29.0);
    let front_rail = centered_cube(
        "humidity_condensate_material_coupon_rack_front_cassette_rail",
        RACK_X - 24.0,
        12.0,
        22.0,
    )
    .translate(0.0, -RACK_Y / 2.0 + 16.0, -RACK_Z / 2.0 + 27.0);

    let mut cassettes = Part::empty("humidity_condensate_material_candidate_coupon_cassettes");
    let mut coupon_cuts = Part::empty("humidity_condensate_material_coupon_slot_cuts");
    let mut tabs = Part::empty("humidity_condensate_material_coupon_cassette_trace_tabs");
    for cassette in 0..CASSETTE_COUNT {
        let x = centered_index(cassette, CASSETTE_COUNT, CASSETTE_PITCH_X);
        cassettes = cassettes
            + centered_cube(
                format!("humidity_condensate_material_coupon_cassette_body_{cassette}"),
                CASSETTE_X,
                CASSETTE_Y,
                CASSETTE_Z,
            )
            .translate(x, 0.0, -RACK_Z / 2.0 + 16.0 + CASSETTE_Z / 2.0);

        for slot in 0..COUPONS_PER_CASSETTE {
            let y = centered_index(slot, COUPONS_PER_CASSETTE, COUPON_SLOT_PITCH_Y);
            let global_slot = cassette * COUPONS_PER_CASSETTE + slot;
            coupon_cuts = coupon_cuts
                + centered_cube(
                    format!("humidity_condensate_material_coupon_slot_cut_{global_slot}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_Z,
                )
                .translate(x, y, RACK_Z / 2.0 - COUPON_SLOT_Z / 2.0 + 0.5);
            tabs = tabs
                + centered_cube(
                    format!("humidity_condensate_material_coupon_serial_land_{global_slot}"),
                    52.0,
                    8.0,
                    3.0,
                )
                .translate(x, y + 11.0, RACK_Z / 2.0 + 1.5);
        }
    }

    base + rear_rail + front_rail + cassettes - coupon_cuts
        + tabs
        + rack_side_handles()
        + gripper_fiducials("coupon_rack_cassettes")
}

fn rack_side_handles() -> Part {
    let left = centered_cube(
        "humidity_condensate_material_coupon_rack_left_robot_handle",
        18.0,
        118.0,
        24.0,
    )
    .translate(-RACK_X / 2.0 + 18.0, 0.0, RACK_Z / 2.0 - 12.0);
    let right = centered_cube(
        "humidity_condensate_material_coupon_rack_right_robot_handle",
        18.0,
        118.0,
        24.0,
    )
    .translate(RACK_X / 2.0 - 18.0, 0.0, RACK_Z / 2.0 - 12.0);
    left + right
}

fn condensate_drip_challenge_manifold() -> Part {
    let header = centered_cylinder(
        "humidity_condensate_material_drip_manifold_header",
        MANIFOLD_HEADER_D / 2.0,
        MANIFOLD_X - 64.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, MANIFOLD_Z / 2.0 - 28.0);
    let supply_port = centered_cylinder(
        "humidity_condensate_material_drip_manifold_supply_port",
        11.0 / 2.0,
        72.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -MANIFOLD_X / 2.0 + 42.0,
        MANIFOLD_Y / 2.0 - 10.0,
        MANIFOLD_Z / 2.0 - 28.0,
    );
    let support_rail = centered_cube(
        "humidity_condensate_material_drip_manifold_support_rail",
        MANIFOLD_X,
        22.0,
        18.0,
    )
    .translate(0.0, -MANIFOLD_Y / 2.0 + 22.0, -MANIFOLD_Z / 2.0 + 18.0);
    let splash_shield = centered_cube(
        "humidity_condensate_material_drip_manifold_rear_splash_shield",
        MANIFOLD_X - 40.0,
        8.0,
        58.0,
    )
    .translate(0.0, MANIFOLD_Y / 2.0 - 14.0, -2.0);

    header + supply_port + support_rail + splash_shield + drip_nozzle_array() + manifold_feet()
}

fn drip_nozzle_array() -> Part {
    let mut nozzles = Part::empty("humidity_condensate_material_indexed_drip_nozzles");
    for cassette in 0..CASSETTE_COUNT {
        let x_base = centered_index(cassette, CASSETTE_COUNT, CASSETTE_PITCH_X);
        for slot in 0..COUPONS_PER_CASSETTE {
            let y = -32.0 + centered_index(slot, COUPONS_PER_CASSETTE, 12.0);
            let index = cassette * COUPONS_PER_CASSETTE + slot;
            nozzles = nozzles
                + centered_cylinder(
                    format!("humidity_condensate_material_drip_nozzle_stub_{index}"),
                    NOZZLE_D / 2.0,
                    NOZZLE_Z,
                    20,
                )
                .translate(x_base, y, MANIFOLD_Z / 2.0 - NOZZLE_Z / 2.0)
                + centered_cube(
                    format!("humidity_condensate_material_drip_witness_flag_{index}"),
                    14.0,
                    4.0,
                    12.0,
                )
                .translate(x_base, y + 8.0, MANIFOLD_Z / 2.0 - 10.0);
        }
    }
    nozzles
}

fn manifold_feet() -> Part {
    let mut feet = Part::empty("humidity_condensate_material_drip_manifold_feet");
    for (i, x) in [
        -MANIFOLD_X / 2.0 + 44.0,
        -MANIFOLD_X / 6.0,
        MANIFOLD_X / 6.0,
        MANIFOLD_X / 2.0 - 44.0,
    ]
    .iter()
    .enumerate()
    {
        feet = feet
            + centered_cube(
                format!("humidity_condensate_material_drip_manifold_mount_foot_{i}"),
                58.0,
                36.0,
                14.0,
            )
            .translate(*x, -MANIFOLD_Y / 2.0 + 25.0, -MANIFOLD_Z / 2.0 + 7.0);
    }
    feet
}

fn heated_humidity_pocket_placeholders() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_heated_humidity_placeholder_plate",
        HUMIDITY_POCKET_X,
        HUMIDITY_POCKET_Y,
        HUMIDITY_POCKET_Z,
    );
    let mut wells = Part::empty("humidity_condensate_material_heated_humidity_well_cuts");
    let mut lands = Part::empty("humidity_condensate_material_heated_humidity_lands");
    for i in 0..HUMIDITY_POCKET_COUNT {
        let (x, y) = humidity_pocket_center(i);
        wells = wells
            + centered_cylinder(
                format!("humidity_condensate_material_humidity_pocket_cut_{i}"),
                HUMIDITY_WELL_D / 2.0,
                HUMIDITY_WELL_DEPTH + 1.0,
                48,
            )
            .translate(
                x,
                y,
                HUMIDITY_POCKET_Z / 2.0 - HUMIDITY_WELL_DEPTH / 2.0 + 0.5,
            )
            + centered_cube(
                format!("humidity_condensate_material_humidity_sensor_keyway_{i}"),
                14.0,
                HUMIDITY_WELL_D + 12.0,
                HUMIDITY_WELL_DEPTH + 2.0,
            )
            .translate(
                x,
                y,
                HUMIDITY_POCKET_Z / 2.0 - HUMIDITY_WELL_DEPTH / 2.0 + 0.5,
            );
        lands = lands
            + centered_cylinder(
                format!("humidity_condensate_material_heated_pocket_witness_ring_outer_{i}"),
                HUMIDITY_WELL_D / 2.0 + 6.0,
                4.0,
                48,
            )
            .translate(x, y, HUMIDITY_POCKET_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("humidity_condensate_material_heated_pocket_witness_ring_inner_{i}"),
                HUMIDITY_WELL_D / 2.0 + 1.0,
                5.0,
                48,
            )
            .translate(x, y, HUMIDITY_POCKET_Z / 2.0 + 2.0)
            + centered_cube(
                format!("humidity_condensate_material_heater_blind_connector_land_{i}"),
                48.0,
                16.0,
                4.0,
            )
            .translate(
                x,
                y - HUMIDITY_WELL_D / 2.0 - 16.0,
                HUMIDITY_POCKET_Z / 2.0 + 2.0,
            );
    }

    plate - wells + lands + humidity_module_feet() + gripper_fiducials("heated_humidity_pockets")
}

fn humidity_pocket_center(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (centered_index(col, 2, 118.0), centered_index(row, 2, 82.0))
}

fn humidity_module_feet() -> Part {
    let mut feet = Part::empty("humidity_condensate_material_heated_humidity_thermal_break_feet");
    for (i, (x, y)) in [
        (
            -HUMIDITY_POCKET_X / 2.0 + 34.0,
            -HUMIDITY_POCKET_Y / 2.0 + 28.0,
        ),
        (
            HUMIDITY_POCKET_X / 2.0 - 34.0,
            -HUMIDITY_POCKET_Y / 2.0 + 28.0,
        ),
        (
            -HUMIDITY_POCKET_X / 2.0 + 34.0,
            HUMIDITY_POCKET_Y / 2.0 - 28.0,
        ),
        (
            HUMIDITY_POCKET_X / 2.0 - 34.0,
            HUMIDITY_POCKET_Y / 2.0 - 28.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        feet = feet
            + centered_cube(
                format!("humidity_condensate_material_humidity_thermal_break_foot_{i}"),
                42.0,
                28.0,
                12.0,
            )
            .translate(*x, *y, -HUMIDITY_POCKET_Z / 2.0 + 6.0);
    }
    feet
}

fn cleaning_residue_coupon_lanes() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_cleaning_residue_lane_plate",
        RESIDUE_LANE_X,
        RESIDUE_LANE_Y,
        RESIDUE_LANE_Z,
    );
    let mut channel_cuts = Part::empty("humidity_condensate_material_residue_lane_channel_cuts");
    let mut lane_tags = Part::empty("humidity_condensate_material_residue_lane_tags");
    for i in 0..RESIDUE_LANE_COUNT {
        let x = centered_index(i, RESIDUE_LANE_COUNT, RESIDUE_LANE_PITCH_X);
        channel_cuts = channel_cuts
            + centered_cube(
                format!("humidity_condensate_material_cleaning_residue_coupon_lane_cut_{i}"),
                34.0,
                RESIDUE_LANE_Y - 38.0,
                15.0,
            )
            .translate(x, 2.0, RESIDUE_LANE_Z / 2.0 - 7.0)
            + centered_cylinder(
                format!("humidity_condensate_material_residue_lane_low_point_well_{i}"),
                10.0,
                17.0,
                32,
            )
            .translate(x, -RESIDUE_LANE_Y / 2.0 + 26.0, RESIDUE_LANE_Z / 2.0 - 7.0);
        lane_tags = lane_tags
            + centered_cube(
                format!("humidity_condensate_material_residue_lane_barcode_tab_{i}"),
                34.0,
                12.0,
                3.0,
            )
            .translate(x, RESIDUE_LANE_Y / 2.0 - 18.0, RESIDUE_LANE_Z / 2.0 + 1.5);
    }

    plate - channel_cuts + residue_lane_dividers() + lane_tags + gripper_fiducials("residue_lanes")
}

fn residue_lane_dividers() -> Part {
    let mut dividers = Part::empty("humidity_condensate_material_residue_lane_dividers");
    for i in 0..=RESIDUE_LANE_COUNT {
        let x = -((RESIDUE_LANE_COUNT as f64) * RESIDUE_LANE_PITCH_X) / 2.0
            + i as f64 * RESIDUE_LANE_PITCH_X;
        dividers = dividers
            + centered_cube(
                format!("humidity_condensate_material_residue_lane_divider_{i}"),
                4.0,
                RESIDUE_LANE_Y - 26.0,
                16.0,
            )
            .translate(x, 0.0, RESIDUE_LANE_Z / 2.0 + 8.0);
    }
    dividers
}

fn mass_thickness_measurement_nests() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_measurement_nest_plate",
        MEASURE_X,
        MEASURE_Y,
        MEASURE_Z,
    );
    let mut cuts = Part::empty("humidity_condensate_material_measurement_nest_cuts");
    let mut anvils = Part::empty("humidity_condensate_material_measurement_nest_anvils");
    for i in 0..MASS_NEST_COUNT {
        let x = centered_index(i, MASS_NEST_COUNT, 70.0);
        cuts = cuts
            + centered_cube(
                format!("humidity_condensate_material_mass_balance_pan_recess_{i}"),
                52.0,
                42.0,
                14.0,
            )
            .translate(x, 34.0, MEASURE_Z / 2.0 - 6.5);
        anvils = anvils
            + centered_cube(
                format!("humidity_condensate_material_mass_nest_serial_flat_{i}"),
                48.0,
                12.0,
                3.0,
            )
            .translate(x, 65.0, MEASURE_Z / 2.0 + 1.5);
    }
    for i in 0..THICKNESS_NEST_COUNT {
        let x = centered_index(i, THICKNESS_NEST_COUNT, 70.0);
        cuts = cuts
            + centered_cube(
                format!("humidity_condensate_material_thickness_gauge_coupon_recess_{i}"),
                54.0,
                24.0,
                12.0,
            )
            .translate(x, -34.0, MEASURE_Z / 2.0 - 5.5);
        anvils = anvils
            + centered_cylinder(
                format!("humidity_condensate_material_thickness_anvil_land_{i}"),
                12.0,
                4.0,
                32,
            )
            .translate(x - 18.0, -34.0, MEASURE_Z / 2.0 + 2.0)
            + centered_cylinder(
                format!("humidity_condensate_material_thickness_probe_land_{i}"),
                8.0,
                4.0,
                32,
            )
            .translate(x + 18.0, -34.0, MEASURE_Z / 2.0 + 2.0);
    }

    plate - cuts + anvils + measurement_scale_guard() + gripper_fiducials("measurement_nests")
}

fn measurement_scale_guard() -> Part {
    centered_cube(
        "humidity_condensate_material_measurement_balance_air_guard",
        MEASURE_X - 48.0,
        8.0,
        30.0,
    )
    .translate(0.0, 0.0, MEASURE_Z / 2.0 + 15.0)
}

fn seal_gasket_witness_pockets() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_seal_gasket_witness_plate",
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let mut cuts = Part::empty("humidity_condensate_material_gasket_witness_pocket_cuts");
    let mut witness_rings = Part::empty("humidity_condensate_material_gasket_witness_rings");
    for i in 0..GASKET_WITNESS_COUNT {
        let (x, y) = gasket_pocket_center(i);
        cuts = cuts
            + centered_cylinder(
                format!("humidity_condensate_material_gasket_witness_pocket_cut_{i}"),
                GASKET_POCKET_D / 2.0,
                GASKET_POCKET_DEPTH + 1.0,
                40,
            )
            .translate(x, y, SEAL_Z / 2.0 - GASKET_POCKET_DEPTH / 2.0 + 0.5);
        witness_rings = witness_rings
            + centered_cylinder(
                format!("humidity_condensate_material_gasket_witness_ring_outer_{i}"),
                GASKET_POCKET_D / 2.0 + 5.0,
                4.0,
                40,
            )
            .translate(x, y, SEAL_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("humidity_condensate_material_gasket_witness_ring_inner_{i}"),
                GASKET_POCKET_D / 2.0 + 1.0,
                5.0,
                40,
            )
            .translate(x, y, SEAL_Z / 2.0 + 2.0)
            + centered_cube(
                format!("humidity_condensate_material_gasket_compression_flag_{i}"),
                30.0,
                7.0,
                3.0,
            )
            .translate(x, y + GASKET_POCKET_D / 2.0 + 9.0, SEAL_Z / 2.0 + 1.5);
    }

    plate - cuts + witness_rings + seal_lane_rails() + gripper_fiducials("seal_gasket_pockets")
}

fn gasket_pocket_center(index: usize) -> (f64, f64) {
    let col = index % 5;
    let row = index / 5;
    (centered_index(col, 5, 64.0), centered_index(row, 2, 58.0))
}

fn seal_lane_rails() -> Part {
    let upper = centered_cube(
        "humidity_condensate_material_seal_witness_upper_retainer_rail",
        SEAL_X - 36.0,
        8.0,
        18.0,
    )
    .translate(0.0, SEAL_Y / 2.0 - 14.0, SEAL_Z / 2.0 + 9.0);
    let lower = centered_cube(
        "humidity_condensate_material_seal_witness_lower_retainer_rail",
        SEAL_X - 36.0,
        8.0,
        18.0,
    )
    .translate(0.0, -SEAL_Y / 2.0 + 14.0, SEAL_Z / 2.0 + 9.0);
    upper + lower
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_barcode_certificate_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("humidity_condensate_material_barcode_certificate_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(
                format!("humidity_condensate_material_barcode_land_{i}"),
                54.0,
                20.0,
                3.0,
            )
            .translate(
                centered_index(col, 4, 68.0),
                24.0 - row as f64 * 34.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_condensate_material_certificate_card_land_{i}"),
                82.0,
                28.0,
                3.5,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 92.0),
                -TRACE_Y / 2.0 + 18.0,
                TRACE_Z / 2.0 + 1.75,
            );
    }

    plate + lands + traceability_fiducials()
}

fn traceability_fiducials() -> Part {
    fiducial_disc("humidity_condensate_material_traceability_left_fiducial").translate(
        -TRACE_X / 2.0 + 24.0,
        TRACE_Y / 2.0 - 20.0,
        TRACE_Z / 2.0 + 2.5,
    ) + fiducial_disc("humidity_condensate_material_traceability_right_fiducial").translate(
        TRACE_X / 2.0 - 24.0,
        TRACE_Y / 2.0 - 20.0,
        TRACE_Z / 2.0 + 2.5,
    )
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "humidity_condensate_material_release_hold_reject_lane_plate",
        LANE_PLATE_X,
        LANE_PLATE_Y,
        LANE_PLATE_Z,
    );
    let mut trough_cuts = Part::empty("humidity_condensate_material_disposition_lane_cuts");
    let mut dividers = Part::empty("humidity_condensate_material_disposition_lane_dividers");
    let mut flags = Part::empty("humidity_condensate_material_disposition_lane_flags");
    for i in 0..DISPOSITION_LANE_COUNT {
        let x = centered_index(i, DISPOSITION_LANE_COUNT, 156.0);
        trough_cuts = trough_cuts
            + centered_cube(
                format!("humidity_condensate_material_disposition_lane_recess_{i}"),
                130.0,
                LANE_PLATE_Y - 36.0,
                12.0,
            )
            .translate(x, 0.0, LANE_PLATE_Z / 2.0 - 5.5);
        flags = flags
            + centered_cube(disposition_flag_name(i), 92.0, 18.0, 4.0).translate(
                x,
                LANE_PLATE_Y / 2.0 - 18.0,
                LANE_PLATE_Z / 2.0 + 2.0,
            );
    }
    for i in 0..=DISPOSITION_LANE_COUNT {
        let x = -(DISPOSITION_LANE_COUNT as f64) * 156.0 / 2.0 + i as f64 * 156.0;
        dividers = dividers
            + centered_cube(
                format!("humidity_condensate_material_disposition_lane_wall_{i}"),
                6.0,
                LANE_PLATE_Y,
                28.0,
            )
            .translate(x, 0.0, LANE_PLATE_Z / 2.0 + 14.0);
    }

    plate - trough_cuts + dividers + flags + gripper_fiducials("release_hold_reject_lanes")
}

fn disposition_flag_name(index: usize) -> &'static str {
    match index {
        0 => "humidity_condensate_material_release_lane_land",
        1 => "humidity_condensate_material_hold_lane_land",
        _ => "humidity_condensate_material_reject_lane_land",
    }
}

fn clean_used_segregation_bulkhead() -> Part {
    let wall = centered_cube(
        "humidity_condensate_material_clean_used_bulkhead_wall",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let drip_skirt = centered_cube(
        "humidity_condensate_material_used_side_drip_skirt",
        36.0,
        BULKHEAD_Y - 48.0,
        30.0,
    )
    .translate(USED_SIDE_LIMIT_X / 2.0, 0.0, -BULKHEAD_Z / 2.0 + 15.0);
    let clean_rail = centered_cube(
        "humidity_condensate_material_clean_side_positive_stop_rail",
        28.0,
        BULKHEAD_Y - 78.0,
        22.0,
    )
    .translate(CLEAN_SIDE_LIMIT_X / 2.0, 0.0, -BULKHEAD_Z / 2.0 + 36.0);

    wall - bulkhead_pass_through_cuts() + drip_skirt + clean_rail + bulkhead_trace_lands()
}

fn bulkhead_pass_through_cuts() -> Part {
    let mut cuts = Part::empty("humidity_condensate_material_bulkhead_pass_through_cuts");
    for i in 0..BULKHEAD_PASS_THROUGH_COUNT {
        cuts = cuts
            + centered_cube(
                format!("humidity_condensate_material_bulkhead_coupon_pass_through_{i}"),
                BULKHEAD_X + 2.0,
                92.0,
                26.0,
            )
            .translate(
                0.0,
                centered_index(i, BULKHEAD_PASS_THROUGH_COUNT, 164.0),
                7.0,
            );
    }
    cuts
}

fn bulkhead_trace_lands() -> Part {
    let mut lands = Part::empty("humidity_condensate_material_bulkhead_trace_lands");
    for (i, y) in [-250.0, 250.0].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("humidity_condensate_material_bulkhead_clean_used_status_land_{i}"),
                20.0,
                86.0,
                4.0,
            )
            .translate(0.0, *y, BULKHEAD_Z / 2.0 + 2.0);
    }
    lands
}

fn evidence_camera_bridge() -> Part {
    let top_beam = centered_cube(
        "humidity_condensate_material_evidence_camera_bridge_top_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        24.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 12.0);
    let rear_light_bar = centered_cube(
        "humidity_condensate_material_evidence_bridge_light_bar_placeholder",
        CAMERA_BRIDGE_X - 100.0,
        12.0,
        14.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_Y / 2.0 - 10.0,
        CAMERA_BRIDGE_Z / 2.0 - 42.0,
    );
    let front_scale_bar = centered_cube(
        "humidity_condensate_material_evidence_bridge_front_scale_reference",
        CAMERA_BRIDGE_X - 140.0,
        8.0,
        10.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 + 10.0,
        -CAMERA_BRIDGE_Z / 2.0 + 54.0,
    );

    top_beam + rear_light_bar + front_scale_bar + camera_bridge_posts() + camera_mount_lands()
}

fn camera_bridge_posts() -> Part {
    let mut posts = Part::empty("humidity_condensate_material_evidence_camera_bridge_posts");
    for (i, (x, y)) in [
        (-CAMERA_BRIDGE_X / 2.0 + 36.0, -CAMERA_BRIDGE_Y / 2.0 + 16.0),
        (CAMERA_BRIDGE_X / 2.0 - 36.0, -CAMERA_BRIDGE_Y / 2.0 + 16.0),
        (-CAMERA_BRIDGE_X / 2.0 + 36.0, CAMERA_BRIDGE_Y / 2.0 - 16.0),
        (CAMERA_BRIDGE_X / 2.0 - 36.0, CAMERA_BRIDGE_Y / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("humidity_condensate_material_camera_bridge_post_{i}"),
                28.0,
                28.0,
                CAMERA_BRIDGE_Z,
            )
            .translate(*x, *y, 0.0);
    }
    posts
}

fn camera_mount_lands() -> Part {
    let mut lands = Part::empty("humidity_condensate_material_evidence_camera_mount_lands");
    for i in 0..CAMERA_LAND_COUNT {
        let x = centered_index(i, CAMERA_LAND_COUNT, 174.0);
        lands = lands
            + centered_cube(
                format!("humidity_condensate_material_camera_land_{i}"),
                74.0,
                44.0,
                8.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 4.0)
            - centered_cylinder(
                format!("humidity_condensate_material_camera_mount_bore_{i}"),
                5.0 / 2.0,
                10.0,
                24,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 4.0);
    }
    lands
}

fn robot_service_keepout_gauges() -> Part {
    let perimeter = centered_cube(
        "humidity_condensate_material_robot_service_keepout_perimeter_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let tray_cut = centered_cube(
        "humidity_condensate_material_keepout_open_work_area_cut",
        KEEP_OUT_X - 96.0,
        KEEP_OUT_Y - 96.0,
        KEEP_OUT_Z + 1.0,
    );
    let front_robot = centered_cube(
        "humidity_condensate_material_front_robot_approach_gauge",
        KEEP_OUT_X - 120.0,
        34.0,
        18.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 32.0, KEEP_OUT_Z / 2.0 + 9.0);
    let rear_service = centered_cube(
        "humidity_condensate_material_rear_service_sweep_gauge",
        KEEP_OUT_X - 140.0,
        34.0,
        18.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 32.0, KEEP_OUT_Z / 2.0 + 9.0);
    let left_service = centered_cube(
        "humidity_condensate_material_left_robot_gripper_keepout_gauge",
        34.0,
        KEEP_OUT_Y - 130.0,
        16.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 32.0, 0.0, KEEP_OUT_Z / 2.0 + 8.0);
    let right_service = centered_cube(
        "humidity_condensate_material_right_service_hand_keepout_gauge",
        34.0,
        KEEP_OUT_Y - 130.0,
        16.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 32.0, 0.0, KEEP_OUT_Z / 2.0 + 8.0);

    perimeter - tray_cut + front_robot + rear_service + left_service + right_service
}

fn gripper_fiducials(prefix: &str) -> Part {
    let left = fiducial_disc(&format!(
        "humidity_condensate_material_{prefix}_left_fiducial"
    ))
    .translate(-28.0, 28.0, 4.0);
    let right = fiducial_disc(&format!(
        "humidity_condensate_material_{prefix}_right_fiducial"
    ))
    .translate(28.0, 28.0, 4.0);
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 10.0, 4.0, 40)
        - centered_cylinder(format!("{name}_center_pip"), 2.4, 5.0, 20)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(
            OUTPUTS[0],
            "output/closed_humidity_condensate_material_compatibility_station_base_containment_tray.stl"
        );
        assert_eq!(
            OUTPUTS[12],
            "output/closed_humidity_condensate_material_compatibility_station_assembly.stl"
        );
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_humidity_condensate_material_compatibility_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn fixture_covers_requested_validation_packaging_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"base_containment_tray"));
        assert!(REQUIRED_FEATURES.contains(&"coupon_rack_cassettes"));
        assert!(REQUIRED_FEATURES.contains(&"condensate_drip_challenge_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"heated_humidity_pocket_placeholders"));
        assert!(REQUIRED_FEATURES.contains(&"cleaning_residue_coupon_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"mass_thickness_measurement_nests"));
        assert!(REQUIRED_FEATURES.contains(&"seal_gasket_witness_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepout_gauges"));
    }

    #[test]
    fn modules_fit_inside_containment_tray() {
        assert_design_constraints();
        for (_name, pos, width, depth) in module_specs() {
            assert!(fits_inside_containment(pos, width, depth));
        }
    }

    #[test]
    fn coupon_and_drip_arrays_are_index_matched() {
        assert_eq!(CASSETTE_COUNT, 4);
        assert_eq!(COUPONS_PER_CASSETTE, 6);
        assert_eq!(MATERIAL_COUPON_COUNT, 24);
        assert_eq!(DRIP_NOZZLE_COUNT, MATERIAL_COUPON_COUNT);
        assert!(COUPON_SLOT_X < CASSETTE_X);
        assert!(COUPON_SLOT_Y * (COUPONS_PER_CASSETTE as f64) < CASSETTE_Y);
        assert!(NOZZLE_D < COUPON_SLOT_Y / 3.0);
    }

    #[test]
    fn clean_and_used_work_areas_are_segregated_by_bulkhead() {
        let clean_modules = [
            (RACK_POS, RACK_X),
            (RESIDUE_POS, RESIDUE_LANE_X),
            (SEAL_POS, SEAL_X),
        ];
        for (pos, width) in clean_modules {
            assert!(pos.0 + width / 2.0 < CLEAN_SIDE_LIMIT_X);
        }

        let used_modules = [
            (HUMIDITY_POS, HUMIDITY_POCKET_X),
            (MEASURE_POS, MEASURE_X),
            (TRACE_POS, TRACE_X),
        ];
        for (pos, width) in used_modules {
            assert!(pos.0 - width / 2.0 > USED_SIDE_LIMIT_X);
        }

        assert_eq!(BULKHEAD_PASS_THROUGH_COUNT, 3);
        assert!(BULKHEAD_Z > RACK_Z);
    }

    #[test]
    fn inspection_capacity_is_explicit_without_acceptance_limits() {
        assert_eq!(RESIDUE_LANE_COUNT, 8);
        assert_eq!(HUMIDITY_POCKET_COUNT, 4);
        assert_eq!(MASS_NEST_COUNT, 4);
        assert_eq!(THICKNESS_NEST_COUNT, 4);
        assert_eq!(GASKET_WITNESS_COUNT, 10);
        assert_eq!(BARCODE_LAND_COUNT, 8);
        assert_eq!(CERTIFICATE_LAND_COUNT, 3);
        assert!(GASKET_POCKET_DEPTH < SEAL_Z);
        assert!(HUMIDITY_WELL_DEPTH < HUMIDITY_POCKET_Z);
    }

    #[test]
    fn containment_exceeds_fixture_hold_up_volume() {
        assert!(containment_freeboard_volume_ml() > maximum_challenge_hold_up_ml());
        assert!(containment_freeboard_volume_ml() > 20_000.0);
        assert!(maximum_challenge_hold_up_ml() < 500.0);
    }

    #[test]
    fn disposition_and_evidence_interfaces_are_counted() {
        assert_eq!(DISPOSITION_LANE_COUNT, 3);
        assert_eq!(CAMERA_POST_COUNT, 4);
        assert_eq!(CAMERA_LAND_COUNT, 5);
        assert!(CAMERA_BRIDGE_Z > BULKHEAD_Z);
        assert!(LANE_PLATE_Y < RESIDUE_LANE_Y);
    }

    #[test]
    fn robot_and_service_keepouts_have_minimum_clearance() {
        assert!(front_robot_approach_clearance() >= FRONT_ROBOT_APPROACH_CLEARANCE);
        assert!(rear_service_sweep_clearance() >= REAR_SERVICE_SWEEP_CLEARANCE);
        assert!(side_service_sweep_clearance() >= SIDE_SERVICE_SWEEP_CLEARANCE);
        assert!(KEEP_OUT_X > STATION_X - 80.0);
        assert!(KEEP_OUT_Y > STATION_Y - 80.0);
    }
}
