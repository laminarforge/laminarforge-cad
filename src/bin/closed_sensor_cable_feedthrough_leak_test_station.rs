use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sensor/electrical cable feedthrough leak and strain-relief test station.
//
// Intent:
// - Validate sealed sensor and electrical cable feedthrough coupons used on
//   incubator/module walls without opening the closed module boundary.
// - Fixture wall coupons, strain-relief combs, pressure-decay annulus ports,
//   dye/water witness wells, bend-radius gauges, isolation placeholders,
//   evidence capture, traceability, disposition lanes, and segregation in one
//   bench-scale validation station.
// - Model purchased pressure hardware, insulation testers, cameras, labels, and
//   final certified feedthroughs as service envelopes only. This CAD is not a
//   pressure vessel, electrical safety certification, or sterility claim.
//
// Exports:
//   output/closed_sensor_cable_feedthrough_leak_test_station_base_containment_tray.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_wall_feedthrough_coupon_panel.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_cable_strain_relief_comb.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_pressure_decay_port_manifold.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_dye_water_ingress_witness_wells.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_bend_radius_gauge.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_electrical_isolation_placeholder_pads.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_barcode_certificate_lands.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_release_hold_reject_lanes.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_clean_used_segregation.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_evidence_camera_bridge.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_robot_service_keepout_gauges.stl
//   output/closed_sensor_cable_feedthrough_leak_test_station_assembly.stl

const OUTPUTS: &[&str] = &[
    "output/closed_sensor_cable_feedthrough_leak_test_station_base_containment_tray.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_wall_feedthrough_coupon_panel.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_cable_strain_relief_comb.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_pressure_decay_port_manifold.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_dye_water_ingress_witness_wells.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_bend_radius_gauge.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_electrical_isolation_placeholder_pads.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_barcode_certificate_lands.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_release_hold_reject_lanes.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_clean_used_segregation.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_evidence_camera_bridge.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_robot_service_keepout_gauges.stl",
    "output/closed_sensor_cable_feedthrough_leak_test_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_containment_tray",
    "wall_feedthrough_coupon_panel",
    "cable_strain_relief_comb",
    "pressure_decay_port_manifold",
    "dye_water_ingress_witness_wells",
    "bend_radius_gauge",
    "electrical_isolation_placeholder_pads",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const CABLE_CLASSES: [&str; FEEDTHROUGH_COUNT] = [
    "co2_sensor",
    "humidity_sensor",
    "temperature_probe",
    "heater_power",
    "fan_power_data",
    "spare_ground",
];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 22.0;
const BASIN_X: f64 = STATION_X - 116.0;
const BASIN_Y: f64 = STATION_Y - 108.0;
const BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;

const PANEL_X: f64 = 760.0;
const PANEL_Y: f64 = 28.0;
const PANEL_Z: f64 = 310.0;
const PANEL_CENTER_Y: f64 = 292.0;
const PANEL_CENTER_Z: f64 = BASE_Z + PANEL_Z / 2.0;
const PANEL_MAIN_FEEDTHROUGH_Z: f64 = BASE_Z + 168.0;
const PANEL_ANNULUS_PORT_Z: f64 = BASE_Z + 232.0;
const PANEL_DRAIN_PORT_Z: f64 = BASE_Z + 92.0;
const FEEDTHROUGH_COUNT: usize = 6;
const FEEDTHROUGH_PITCH_X: f64 = 112.0;
const FEEDTHROUGH_BORE_D: f64 = 18.0;
const GROMMET_LAND_D: f64 = 42.0;
const ANNULUS_PORT_D: f64 = 6.4;
const COUPON_CLAMP_TABS: usize = FEEDTHROUGH_COUNT * 2;
const WALL_COUPON_GASKET_LANDS: usize = FEEDTHROUGH_COUNT;

const COMB_CENTER_X: f64 = 0.0;
const COMB_CENTER_Y: f64 = 176.0;
const COMB_X: f64 = 790.0;
const COMB_Y: f64 = 92.0;
const COMB_Z: f64 = 64.0;
const CABLE_SLOT_D: f64 = 11.0;
const COMB_FINGER_W: f64 = 12.0;
const COMB_FINGER_Z: f64 = 78.0;
const STRAIN_RELIEF_LANES: usize = FEEDTHROUGH_COUNT;
const STRAIN_RELIEF_CLAMP_POINTS: usize = FEEDTHROUGH_COUNT * 2;

const MANIFOLD_CENTER_X: f64 = 0.0;
const MANIFOLD_CENTER_Y: f64 = 64.0;
const MANIFOLD_X: f64 = 780.0;
const MANIFOLD_Y: f64 = 78.0;
const MANIFOLD_Z: f64 = 56.0;
const PRESSURE_PORTS_PER_FEEDTHROUGH: usize = 2;
const PRESSURE_DECAY_PORTS: usize = FEEDTHROUGH_COUNT * PRESSURE_PORTS_PER_FEEDTHROUGH;
const PRESSURE_PORT_D: f64 = 7.0;
const SENSOR_POCKET_X: f64 = 42.0;
const SENSOR_POCKET_Y: f64 = 17.0;
const SENSOR_POCKET_Z: f64 = 20.0;
const REFERENCE_PRESSURE_PORTS: usize = 2;
const DECAY_REFERENCE_VOLUME_ML: f64 = 75.0;

const WELL_CENTER_X: f64 = 405.0;
const WELL_CENTER_Y: f64 = -118.0;
const WELL_PLATE_X: f64 = 270.0;
const WELL_PLATE_Y: f64 = 230.0;
const WELL_PLATE_Z: f64 = 22.0;
const WITNESS_WELL_COUNT: usize = FEEDTHROUGH_COUNT + 2;
const WITNESS_WELL_COLS: usize = 4;
const WITNESS_WELL_ROWS: usize = 2;
const WITNESS_WELL_D: f64 = 26.0;
const WITNESS_WELL_DEPTH: f64 = 15.0;
const DYE_CHANNEL_W: f64 = 5.0;
const DYE_REFERENCE_WELLS: usize = 2;

const BEND_CENTER_X: f64 = -378.0;
const BEND_CENTER_Y: f64 = -128.0;
const BEND_GAUGE_X: f64 = 338.0;
const BEND_GAUGE_Y: f64 = 190.0;
const BEND_GAUGE_Z: f64 = 18.0;
const BEND_MANDREL_RADII: [f64; 3] = [25.0, 40.0, 60.0];
const BEND_MANDREL_COUNT: usize = BEND_MANDREL_RADII.len();
const MIN_BEND_RADIUS: f64 = BEND_MANDREL_RADII[0];
const CABLE_NOMINAL_D: f64 = 6.0;

const ISOLATION_CENTER_X: f64 = 0.0;
const ISOLATION_CENTER_Y: f64 = -136.0;
const ISOLATION_PANEL_X: f64 = 360.0;
const ISOLATION_PANEL_Y: f64 = 130.0;
const ISOLATION_PANEL_Z: f64 = 16.0;
const ISOLATION_PAD_COUNT: usize = FEEDTHROUGH_COUNT;
const ISOLATION_PAD_X: f64 = 42.0;
const ISOLATION_PAD_Y: f64 = 34.0;
const ISOLATION_GUARD_GAP: f64 = 8.0;
const ISOLATION_TEST_CLEARANCE_MM: f64 = 16.0;

const TRACE_CENTER_X: f64 = -490.0;
const TRACE_CENTER_Y: f64 = 260.0;
const TRACE_PANEL_X: f64 = 220.0;
const TRACE_PANEL_Y: f64 = 110.0;
const TRACE_PANEL_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 4;
const CERTIFICATE_LANDS: usize = 2;
const FIDUCIALS: usize = 4;

const LANE_CENTER_X: f64 = 250.0;
const LANE_CENTER_Y: f64 = -326.0;
const LANE_BANK_X: f64 = 470.0;
const LANE_BANK_Y: f64 = 126.0;
const LANE_BANK_Z: f64 = 22.0;
const DISPOSITION_LANES: usize = 3;
const LANE_PITCH_X: f64 = 146.0;
const LANE_WIDTH_X: f64 = 126.0;
const LANE_RAIL_Z: f64 = 34.0;
const RELEASE_LANE_INDEX: usize = 0;
const HOLD_LANE_INDEX: usize = 1;
const REJECT_LANE_INDEX: usize = 2;

const SEGREGATION_CENTER_X: f64 = -345.0;
const SEGREGATION_CENTER_Y: f64 = -326.0;
const SEGREGATION_X: f64 = 360.0;
const SEGREGATION_Y: f64 = 126.0;
const SEGREGATION_Z: f64 = 24.0;
const CLEAN_LANES: usize = 2;
const USED_LANES: usize = 2;
const CLEAN_USED_DIVIDER_Z: f64 = 68.0;
const CLEAN_USED_GAP: f64 = 72.0;

const BRIDGE_CENTER_X: f64 = 0.0;
const BRIDGE_CENTER_Y: f64 = 226.0;
const BRIDGE_SPAN_X: f64 = 880.0;
const BRIDGE_POST_SPAN_Y: f64 = 190.0;
const BRIDGE_POST_X: f64 = 28.0;
const BRIDGE_POST_Y: f64 = 28.0;
const BRIDGE_UNDERSIDE_Z: f64 = BASE_Z + 250.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_COUNT: usize = 3;
const LIGHT_BAR_COUNT: usize = 2;

const ROBOT_APPROACH_KEEP_OUT_X: f64 = 820.0;
const ROBOT_APPROACH_KEEP_OUT_Y: f64 = 560.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 350.0;
const REAR_PANEL_SERVICE_KEEP_OUT_Y: f64 = 190.0;
const SIDE_CABLE_SERVICE_KEEP_OUT_X: f64 = 210.0;
const TOP_BRIDGE_SERVICE_KEEP_OUT_Z: f64 = 300.0;
const KEEP_OUT_FRAME_COUNT: usize = 4;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_geometry_assumptions();

    let base = base_containment_tray();
    export(&base, OUTPUTS[0]);

    let panel = wall_feedthrough_coupon_panel();
    export(&panel, OUTPUTS[1]);

    let comb = cable_strain_relief_comb();
    export(&comb, OUTPUTS[2]);

    let manifold = pressure_decay_port_manifold();
    export(&manifold, OUTPUTS[3]);

    let wells = dye_water_ingress_witness_wells();
    export(&wells, OUTPUTS[4]);

    let bend = bend_radius_gauge();
    export(&bend, OUTPUTS[5]);

    let isolation = electrical_isolation_placeholder_pads();
    export(&isolation, OUTPUTS[6]);

    let trace = barcode_certificate_lands();
    export(&trace, OUTPUTS[7]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[8]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[9]);

    let bridge = evidence_camera_bridge();
    export(&bridge, OUTPUTS[10]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + panel
        + comb
        + manifold
        + wells
        + bend
        + isolation
        + trace
        + lanes
        + segregation
        + bridge
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed sensor/electrical cable feedthrough leak test station:");
    println!(
        "  Footprint:                    {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray"
    );
    println!(
        "  Feedthrough coupon panel:     {FEEDTHROUGH_COUNT} wall feedthrough positions for {:?}",
        CABLE_CLASSES
    );
    println!(
        "  Leak test features:           {PRESSURE_DECAY_PORTS} pressure-decay ports, {REFERENCE_PRESSURE_PORTS} reference ports, {WITNESS_WELL_COUNT} dye/water witness wells"
    );
    println!(
        "  Strain-relief features:       {STRAIN_RELIEF_LANES} comb lanes, {STRAIN_RELIEF_CLAMP_POINTS} clamp points, {BEND_MANDREL_COUNT} bend-radius mandrels"
    );
    println!(
        "  Evidence and disposition:     {} traceability lands, {CAMERA_COUNT} camera mounts, {LIGHT_BAR_COUNT} light bars, release/hold/reject lanes, clean/used segregation, and {KEEP_OUT_FRAME_COUNT} keepout gauges",
        BARCODE_LANDS + CERTIFICATE_LANDS
    );
    println!(
        "  Service envelope:             {FRONT_ROBOT_KEEP_OUT_Y:.0}mm front robot clearance, {REAR_PANEL_SERVICE_KEEP_OUT_Y:.0}mm rear panel access, {SIDE_CABLE_SERVICE_KEEP_OUT_X:.0}mm side cable service bay, and {} required feature groups.",
        REQUIRED_FEATURES.len()
    );
    println!(
        "  Limitation:                   CAD captures fixture geometry and envelopes only; pressure, electrical, and sterility validation remain separate controlled procedures."
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_geometry_assumptions() {
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(CABLE_CLASSES.len(), FEEDTHROUGH_COUNT);
    assert_eq!(COUPON_CLAMP_TABS, FEEDTHROUGH_COUNT * 2);
    assert_eq!(WALL_COUPON_GASKET_LANDS, FEEDTHROUGH_COUNT);
    assert_eq!(STRAIN_RELIEF_LANES, FEEDTHROUGH_COUNT);
    assert_eq!(PRESSURE_DECAY_PORTS, FEEDTHROUGH_COUNT * 2);
    assert_eq!(ISOLATION_PAD_COUNT, FEEDTHROUGH_COUNT);
    assert_eq!(DYE_REFERENCE_WELLS, 2);
    assert_eq!(WITNESS_WELL_COUNT, WITNESS_WELL_COLS * WITNESS_WELL_ROWS);
    assert_eq!(LIGHT_BAR_COUNT, 2);
    assert!(GROMMET_LAND_D > FEEDTHROUGH_BORE_D);
    assert!(CABLE_SLOT_D > CABLE_NOMINAL_D + 3.0);
    assert!(MIN_BEND_RADIUS >= CABLE_NOMINAL_D * 4.0);
    assert!(DECAY_REFERENCE_VOLUME_ML >= 50.0);
    assert!(ISOLATION_TEST_CLEARANCE_MM >= 2.0 * ISOLATION_GUARD_GAP);
    assert!(CLEAN_USED_GAP >= 60.0);
    assert!(BRIDGE_UNDERSIDE_Z > PANEL_CENTER_Z + 80.0);
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "closed_sensor_cable_feedthrough_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "closed_sensor_cable_feedthrough_recessed_spill_basin",
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "closed_sensor_cable_feedthrough_low_point_drain",
        DRAIN_D / 2.0,
        BASE_Z + 8.0,
        36,
    )
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 74.0,
        BASE_Z / 2.0,
    );
    let panel_socket = centered_cube(
        "closed_sensor_cable_feedthrough_wall_panel_socket",
        PANEL_X + 52.0,
        18.0,
        7.0,
    )
    .translate(0.0, PANEL_CENTER_Y, BASE_Z - 3.0);

    deck - basin - drain - panel_socket
        + containment_rims()
        + fixture_datum_rails()
        + deck_mount_slots()
        + leak_sense_bosses()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "closed_sensor_cable_feedthrough_front_low_robot_lip",
        STATION_X - 132.0,
        RIM_W,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 34.0, BASE_Z + 12.0);
    let rear = centered_cube(
        "closed_sensor_cable_feedthrough_rear_wall_splash_rim",
        STATION_X - 92.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 34.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_sensor_cable_feedthrough_left_clean_side_rim",
        RIM_W,
        STATION_Y - 120.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + 34.0, 8.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_sensor_cable_feedthrough_right_used_side_rim",
        RIM_W,
        STATION_Y - 120.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - 34.0, 8.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn fixture_datum_rails() -> Part {
    let rear_panel_heel = centered_cube(
        "closed_sensor_cable_feedthrough_rear_panel_heel_datum",
        PANEL_X + 78.0,
        14.0,
        22.0,
    )
    .translate(0.0, PANEL_CENTER_Y - 32.0, BASE_Z + 11.0);
    let comb_left = centered_cube(
        "closed_sensor_cable_feedthrough_comb_left_datum_rail",
        16.0,
        160.0,
        18.0,
    )
    .translate(-COMB_X / 2.0 - 30.0, COMB_CENTER_Y - 20.0, BASE_Z + 9.0);
    let comb_right = centered_cube(
        "closed_sensor_cable_feedthrough_comb_right_datum_rail",
        16.0,
        160.0,
        18.0,
    )
    .translate(COMB_X / 2.0 + 30.0, COMB_CENTER_Y - 20.0, BASE_Z + 9.0);
    let manifold_stop = centered_cube(
        "closed_sensor_cable_feedthrough_pressure_manifold_front_stop",
        MANIFOLD_X + 42.0,
        12.0,
        18.0,
    )
    .translate(
        0.0,
        MANIFOLD_CENTER_Y - MANIFOLD_Y / 2.0 - 18.0,
        BASE_Z + 9.0,
    );

    rear_panel_heel + comb_left + comb_right + manifold_stop
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("closed_sensor_cable_feedthrough_mount_slots");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let boss = centered_cube(
            format!("closed_sensor_cable_feedthrough_mount_slot_boss_{i}"),
            58.0,
            28.0,
            5.0,
        )
        .translate(*x, *y, BASE_Z + 2.5);
        let clearance = centered_cylinder(
            format!("closed_sensor_cable_feedthrough_mount_hole_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            8.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 2.5);
        let slot = centered_cube(
            format!("closed_sensor_cable_feedthrough_mount_slot_clearance_{i}"),
            34.0,
            MOUNT_HOLE_D + 2.0,
            8.0,
        )
        .translate(*x, *y, BASE_Z + 2.5);
        slots = slots + (boss - clearance - slot);
    }
    slots
}

fn leak_sense_bosses() -> Part {
    let mut bosses = Part::empty("closed_sensor_cable_feedthrough_base_leak_sense_bosses");
    for (i, (x, y)) in [
        (-548.0, -348.0),
        (548.0, -348.0),
        (-548.0, 340.0),
        (548.0, 340.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cube(
            format!("closed_sensor_cable_feedthrough_leak_sensor_boss_{i}"),
            44.0,
            34.0,
            8.0,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        let cup = centered_cylinder(
            format!("closed_sensor_cable_feedthrough_leak_sensor_cup_{i}"),
            8.0,
            10.0,
            28,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        bosses = bosses + (boss - cup);
    }
    bosses
}

fn wall_feedthrough_coupon_panel() -> Part {
    let panel = centered_cube(
        "closed_sensor_cable_feedthrough_wall_coupon_panel",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(0.0, PANEL_CENTER_Y, PANEL_CENTER_Z);

    let mut cuts = Part::empty("closed_sensor_cable_feedthrough_wall_coupon_cuts");
    let mut lands = Part::empty("closed_sensor_cable_feedthrough_wall_coupon_lands");

    for i in 0..FEEDTHROUGH_COUNT {
        let x = feedthrough_x(i);
        cuts =
            cuts + through_panel_round_cut(
                format!(
                    "closed_sensor_cable_feedthrough_{}_main_bore",
                    CABLE_CLASSES[i]
                ),
                FEEDTHROUGH_BORE_D,
                x,
                PANEL_MAIN_FEEDTHROUGH_Z,
            ) + through_panel_round_cut(
                format!(
                    "closed_sensor_cable_feedthrough_{}_annulus_pressure_port",
                    CABLE_CLASSES[i]
                ),
                ANNULUS_PORT_D,
                x - 18.0,
                PANEL_ANNULUS_PORT_Z,
            ) + through_panel_round_cut(
                format!(
                    "closed_sensor_cable_feedthrough_{}_annulus_return_port",
                    CABLE_CLASSES[i]
                ),
                ANNULUS_PORT_D,
                x + 18.0,
                PANEL_ANNULUS_PORT_Z,
            ) + through_panel_round_cut(
                format!(
                    "closed_sensor_cable_feedthrough_{}_witness_drain_port",
                    CABLE_CLASSES[i]
                ),
                ANNULUS_PORT_D,
                x,
                PANEL_DRAIN_PORT_Z,
            );

        lands = lands
            + feedthrough_grommet_land(CABLE_CLASSES[i], x)
            + coupon_clamp_tabs(CABLE_CLASSES[i], x)
            + coupon_label_ticks(CABLE_CLASSES[i], x);
    }

    let gasket = panel_gasket_land();
    let witness_slope = centered_cube(
        "closed_sensor_cable_feedthrough_panel_lower_witness_slope",
        PANEL_X - 96.0,
        6.0,
        16.0,
    )
    .translate(
        0.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 4.0,
        PANEL_DRAIN_PORT_Z - 38.0,
    );

    panel - cuts + lands + gasket + witness_slope
}

fn through_panel_round_cut(name: impl Into<String>, diameter: f64, x: f64, z: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, PANEL_Y + 10.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PANEL_CENTER_Y, z)
}

fn feedthrough_grommet_land(class_name: &str, x: f64) -> Part {
    let outer = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_{class_name}_front_grommet_land"),
        GROMMET_LAND_D / 2.0,
        8.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0);
    let inner = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_{class_name}_front_grommet_opening"),
        FEEDTHROUGH_BORE_D / 2.0,
        10.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0);
    let witness_notch = centered_cube(
        format!("closed_sensor_cable_feedthrough_{class_name}_grommet_orientation_notch"),
        8.0,
        10.0,
        20.0,
    )
    .translate(GROMMET_LAND_D / 2.0 - 4.0, 0.0, 0.0);

    (outer - inner - witness_notch).translate(
        x,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 4.0,
        PANEL_MAIN_FEEDTHROUGH_Z,
    )
}

fn coupon_clamp_tabs(class_name: &str, x: f64) -> Part {
    let left = centered_cube(
        format!("closed_sensor_cable_feedthrough_{class_name}_left_coupon_clamp_tab"),
        14.0,
        12.0,
        52.0,
    )
    .translate(
        x - GROMMET_LAND_D / 2.0 - 18.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 6.0,
        PANEL_MAIN_FEEDTHROUGH_Z,
    );
    let right = centered_cube(
        format!("closed_sensor_cable_feedthrough_{class_name}_right_coupon_clamp_tab"),
        14.0,
        12.0,
        52.0,
    )
    .translate(
        x + GROMMET_LAND_D / 2.0 + 18.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 6.0,
        PANEL_MAIN_FEEDTHROUGH_Z,
    );
    let left_screw = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_{class_name}_left_clamp_screw"),
        3.3,
        16.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        x - GROMMET_LAND_D / 2.0 - 18.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 6.0,
        PANEL_MAIN_FEEDTHROUGH_Z,
    );
    let right_screw = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_{class_name}_right_clamp_screw"),
        3.3,
        16.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        x + GROMMET_LAND_D / 2.0 + 18.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 6.0,
        PANEL_MAIN_FEEDTHROUGH_Z,
    );

    (left - left_screw) + (right - right_screw)
}

fn coupon_label_ticks(class_name: &str, x: f64) -> Part {
    let strip = centered_cube(
        format!("closed_sensor_cable_feedthrough_{class_name}_coupon_id_land"),
        62.0,
        5.0,
        11.0,
    )
    .translate(
        x,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 7.0,
        PANEL_MAIN_FEEDTHROUGH_Z - 54.0,
    );
    let mut ticks = Part::empty(format!(
        "closed_sensor_cable_feedthrough_{class_name}_coupon_id_ticks"
    ));
    for tick in 0..5 {
        ticks = ticks
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_{class_name}_coupon_id_tick_{tick}"),
                3.0,
                7.0,
                16.0,
            )
            .translate(
                x + centered_position(tick, 5, 12.0),
                PANEL_CENTER_Y - PANEL_Y / 2.0 - 8.0,
                PANEL_MAIN_FEEDTHROUGH_Z - 54.0,
            );
    }
    strip + ticks
}

fn panel_gasket_land() -> Part {
    let top = centered_cube(
        "closed_sensor_cable_feedthrough_panel_top_gasket_land",
        PANEL_X - 66.0,
        6.0,
        8.0,
    )
    .translate(
        0.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
        PANEL_CENTER_Z + PANEL_Z / 2.0 - 32.0,
    );
    let bottom = centered_cube(
        "closed_sensor_cable_feedthrough_panel_bottom_gasket_land",
        PANEL_X - 66.0,
        6.0,
        8.0,
    )
    .translate(
        0.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
        PANEL_CENTER_Z - PANEL_Z / 2.0 + 32.0,
    );
    let left = centered_cube(
        "closed_sensor_cable_feedthrough_panel_left_gasket_land",
        8.0,
        6.0,
        PANEL_Z - 78.0,
    )
    .translate(
        -PANEL_X / 2.0 + 32.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
        PANEL_CENTER_Z,
    );
    let right = centered_cube(
        "closed_sensor_cable_feedthrough_panel_right_gasket_land",
        8.0,
        6.0,
        PANEL_Z - 78.0,
    )
    .translate(
        PANEL_X / 2.0 - 32.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
        PANEL_CENTER_Z,
    );

    top + bottom + left + right
}

fn cable_strain_relief_comb() -> Part {
    let body = centered_cube(
        "closed_sensor_cable_feedthrough_strain_relief_comb_body",
        COMB_X,
        COMB_Y,
        COMB_Z,
    );
    let mut cuts = Part::empty("closed_sensor_cable_feedthrough_comb_cable_slot_cuts");
    let mut fingers = Part::empty("closed_sensor_cable_feedthrough_comb_fingers");
    let mut clamps = Part::empty("closed_sensor_cable_feedthrough_comb_clamp_points");

    for i in 0..FEEDTHROUGH_COUNT {
        let x = feedthrough_x(i);
        let cable_round = centered_cylinder(
            format!(
                "closed_sensor_cable_feedthrough_comb_{}_cable_radius",
                CABLE_CLASSES[i]
            ),
            CABLE_SLOT_D / 2.0,
            COMB_Y + 8.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, COMB_Z / 2.0 - 18.0);
        let top_entry = centered_cube(
            format!(
                "closed_sensor_cable_feedthrough_comb_{}_top_entry_slot",
                CABLE_CLASSES[i]
            ),
            CABLE_SLOT_D,
            COMB_Y + 8.0,
            COMB_Z,
        )
        .translate(x, 0.0, COMB_Z / 2.0);
        cuts = cuts + cable_round + top_entry;

        fingers = fingers
            + centered_cube(
                format!(
                    "closed_sensor_cable_feedthrough_comb_{}_left_resilient_finger",
                    CABLE_CLASSES[i]
                ),
                COMB_FINGER_W,
                COMB_Y + 18.0,
                COMB_FINGER_Z,
            )
            .translate(x - CABLE_SLOT_D / 2.0 - 9.0, 0.0, 8.0)
            + centered_cube(
                format!(
                    "closed_sensor_cable_feedthrough_comb_{}_right_resilient_finger",
                    CABLE_CLASSES[i]
                ),
                COMB_FINGER_W,
                COMB_Y + 18.0,
                COMB_FINGER_Z,
            )
            .translate(x + CABLE_SLOT_D / 2.0 + 9.0, 0.0, 8.0);

        for side in 0..2 {
            clamps = clamps
                + centered_cylinder(
                    format!(
                        "closed_sensor_cable_feedthrough_comb_{}_clamp_screw_{side}",
                        CABLE_CLASSES[i]
                    ),
                    4.2,
                    9.0,
                    22,
                )
                .translate(
                    x + if side == 0 { -26.0 } else { 26.0 },
                    -COMB_Y / 2.0 - 6.0,
                    COMB_Z / 2.0 + 7.0,
                );
        }
    }

    let entry_arrow_land = centered_cube(
        "closed_sensor_cable_feedthrough_comb_clean_side_entry_arrow_land",
        COMB_X - 96.0,
        8.0,
        6.0,
    )
    .translate(0.0, COMB_Y / 2.0 + 6.0, COMB_Z / 2.0 + 3.0);
    let exit_arrow_land = centered_cube(
        "closed_sensor_cable_feedthrough_comb_used_side_exit_arrow_land",
        COMB_X - 96.0,
        8.0,
        6.0,
    )
    .translate(0.0, -COMB_Y / 2.0 - 6.0, COMB_Z / 2.0 + 3.0);

    (body - cuts + fingers + clamps + entry_arrow_land + exit_arrow_land).translate(
        COMB_CENTER_X,
        COMB_CENTER_Y,
        BASE_Z + COMB_Z / 2.0,
    )
}

fn pressure_decay_port_manifold() -> Part {
    let body = centered_cube(
        "closed_sensor_cable_feedthrough_pressure_decay_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut cuts = Part::empty("closed_sensor_cable_feedthrough_pressure_decay_manifold_cuts");
    let mut bosses = Part::empty("closed_sensor_cable_feedthrough_pressure_decay_port_bosses");
    let mut pockets = Part::empty("closed_sensor_cable_feedthrough_pressure_sensor_pockets");

    for i in 0..FEEDTHROUGH_COUNT {
        let x = feedthrough_x(i);
        for port in 0..PRESSURE_PORTS_PER_FEEDTHROUGH {
            let y = pressure_port_y(port);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_sensor_cable_feedthrough_{}_pressure_decay_port_cut_{port}",
                        CABLE_CLASSES[i]
                    ),
                    PRESSURE_PORT_D / 2.0,
                    MANIFOLD_Z + 8.0,
                    24,
                )
                .translate(x, y, 0.0);
            bosses = bosses
                + centered_cylinder(
                    format!(
                        "closed_sensor_cable_feedthrough_{}_pressure_decay_luer_boss_{port}",
                        CABLE_CLASSES[i]
                    ),
                    12.0,
                    12.0,
                    28,
                )
                .translate(x, y, MANIFOLD_Z / 2.0 + 6.0);
        }
        pockets = pockets
            + centered_cube(
                format!(
                    "closed_sensor_cable_feedthrough_{}_pressure_sensor_service_pocket",
                    CABLE_CLASSES[i]
                ),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                SENSOR_POCKET_Z,
            )
            .translate(
                x,
                MANIFOLD_Y / 2.0 - 12.0,
                MANIFOLD_Z / 2.0 + SENSOR_POCKET_Z / 2.0,
            );
    }

    let reference_a = reference_port("reference_leak_standard_low", -MANIFOLD_X / 2.0 + 42.0);
    let reference_b = reference_port("reference_leak_standard_high", MANIFOLD_X / 2.0 - 42.0);
    let pump_header = centered_cube(
        "closed_sensor_cable_feedthrough_pressure_decay_pump_header_envelope",
        MANIFOLD_X - 90.0,
        13.0,
        12.0,
    )
    .translate(0.0, -MANIFOLD_Y / 2.0 + 10.0, MANIFOLD_Z / 2.0 + 6.0);

    (body - cuts + bosses + pockets + reference_a + reference_b + pump_header).translate(
        MANIFOLD_CENTER_X,
        MANIFOLD_CENTER_Y,
        BASE_Z + MANIFOLD_Z / 2.0,
    )
}

fn reference_port(name: &str, x: f64) -> Part {
    let boss = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_pressure_{name}_boss"),
        15.0,
        14.0,
        30,
    )
    .translate(x, 0.0, MANIFOLD_Z / 2.0 + 7.0);
    let bore = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_pressure_{name}_bore"),
        PRESSURE_PORT_D / 2.0,
        18.0,
        24,
    )
    .translate(x, 0.0, MANIFOLD_Z / 2.0 + 7.0);
    boss - bore
}

fn dye_water_ingress_witness_wells() -> Part {
    let plate = centered_cube(
        "closed_sensor_cable_feedthrough_dye_water_witness_well_plate",
        WELL_PLATE_X,
        WELL_PLATE_Y,
        WELL_PLATE_Z,
    );
    let mut cups = Part::empty("closed_sensor_cable_feedthrough_dye_water_witness_cups");
    let mut rims = Part::empty("closed_sensor_cable_feedthrough_dye_water_witness_rims");
    let mut channels = Part::empty("closed_sensor_cable_feedthrough_dye_water_trace_channels");

    for i in 0..WITNESS_WELL_COUNT {
        let (x, y) = witness_well_center(i);
        cups = cups
            + centered_cylinder(
                format!("closed_sensor_cable_feedthrough_witness_well_cup_{i}"),
                WITNESS_WELL_D / 2.0,
                WITNESS_WELL_DEPTH,
                36,
            )
            .translate(x, y, WELL_PLATE_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 0.5);
        rims = rims
            + witness_well_rim(i, x, y)
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_witness_well_index_land_{i}"),
                26.0,
                6.0,
                4.0,
            )
            .translate(x, y - 28.0, WELL_PLATE_Z / 2.0 + 2.0);

        if i < FEEDTHROUGH_COUNT {
            channels = channels
                + centered_cube(
                    format!("closed_sensor_cable_feedthrough_dye_trace_channel_{i}"),
                    DYE_CHANNEL_W,
                    74.0,
                    4.0,
                )
                .translate(x, y + 34.0, WELL_PLATE_Z / 2.0 + 2.0);
        }
    }

    let reference_land = centered_cube(
        "closed_sensor_cable_feedthrough_dye_water_reference_land_pair",
        WELL_PLATE_X - 52.0,
        18.0,
        5.0,
    )
    .translate(0.0, WELL_PLATE_Y / 2.0 - 25.0, WELL_PLATE_Z / 2.0 + 2.5);

    (plate - cups + rims + channels + reference_land).translate(
        WELL_CENTER_X,
        WELL_CENTER_Y,
        BASE_Z + WELL_PLATE_Z / 2.0,
    )
}

fn witness_well_rim(index: usize, x: f64, y: f64) -> Part {
    let outer = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_witness_well_outer_rim_{index}"),
        WITNESS_WELL_D / 2.0 + 4.0,
        5.0,
        36,
    )
    .translate(x, y, WELL_PLATE_Z / 2.0 + 2.5);
    let inner = centered_cylinder(
        format!("closed_sensor_cable_feedthrough_witness_well_inner_opening_{index}"),
        WITNESS_WELL_D / 2.0,
        6.0,
        36,
    )
    .translate(x, y, WELL_PLATE_Z / 2.0 + 2.5);
    outer - inner
}

fn bend_radius_gauge() -> Part {
    let plate = centered_cube(
        "closed_sensor_cable_feedthrough_bend_radius_gauge_plate",
        BEND_GAUGE_X,
        BEND_GAUGE_Y,
        BEND_GAUGE_Z,
    );
    let mut mandrels = Part::empty("closed_sensor_cable_feedthrough_bend_radius_mandrels");
    for (i, radius) in BEND_MANDREL_RADII.iter().enumerate() {
        let x = centered_position(i, BEND_MANDREL_COUNT, 106.0);
        mandrels = mandrels
            + centered_cylinder(
                format!(
                    "closed_sensor_cable_feedthrough_bend_radius_{:.0}mm_mandrel",
                    radius
                ),
                *radius,
                30.0,
                48,
            )
            .translate(x, 8.0, BEND_GAUGE_Z / 2.0 + 15.0)
            + centered_cube(
                format!(
                    "closed_sensor_cable_feedthrough_bend_radius_{:.0}mm_tangent_gauge",
                    radius
                ),
                58.0,
                8.0,
                9.0,
            )
            .translate(x, -BEND_GAUGE_Y / 2.0 + 26.0, BEND_GAUGE_Z / 2.0 + 4.5);
    }

    let mut cable_size_slots = Part::empty("closed_sensor_cable_feedthrough_bend_cable_size_slots");
    for i in 0..FEEDTHROUGH_COUNT {
        cable_size_slots = cable_size_slots
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_bend_cable_size_slot_{i}"),
                34.0,
                CABLE_NOMINAL_D + i as f64,
                7.0,
            )
            .translate(
                centered_position(i, FEEDTHROUGH_COUNT, 48.0),
                BEND_GAUGE_Y / 2.0 - 24.0,
                BEND_GAUGE_Z / 2.0 + 3.5,
            );
    }

    let no_kink_bar = centered_cube(
        "closed_sensor_cable_feedthrough_no_kink_witness_bar",
        BEND_GAUGE_X - 44.0,
        10.0,
        8.0,
    )
    .translate(0.0, -BEND_GAUGE_Y / 2.0 + 50.0, BEND_GAUGE_Z / 2.0 + 4.0);

    (plate + mandrels + cable_size_slots + no_kink_bar).translate(
        BEND_CENTER_X,
        BEND_CENTER_Y,
        BASE_Z + BEND_GAUGE_Z / 2.0,
    )
}

fn electrical_isolation_placeholder_pads() -> Part {
    let panel = centered_cube(
        "closed_sensor_cable_feedthrough_isolation_placeholder_panel",
        ISOLATION_PANEL_X,
        ISOLATION_PANEL_Y,
        ISOLATION_PANEL_Z,
    );
    let mut pads = Part::empty("closed_sensor_cable_feedthrough_isolation_placeholder_pads");
    for i in 0..ISOLATION_PAD_COUNT {
        let x = centered_position(i, ISOLATION_PAD_COUNT, 54.0);
        let dielectric = centered_cube(
            format!("closed_sensor_cable_feedthrough_isolation_dielectric_pad_{i}"),
            ISOLATION_PAD_X,
            ISOLATION_PAD_Y,
            6.0,
        )
        .translate(x, -18.0, ISOLATION_PANEL_Z / 2.0 + 3.0);
        let electrode_a = centered_cube(
            format!("closed_sensor_cable_feedthrough_isolation_electrode_a_{i}"),
            ISOLATION_PAD_X - 12.0,
            5.0,
            3.0,
        )
        .translate(
            x,
            -18.0 - ISOLATION_GUARD_GAP,
            ISOLATION_PANEL_Z / 2.0 + 7.5,
        );
        let electrode_b = centered_cube(
            format!("closed_sensor_cable_feedthrough_isolation_electrode_b_{i}"),
            ISOLATION_PAD_X - 12.0,
            5.0,
            3.0,
        )
        .translate(
            x,
            -18.0 + ISOLATION_GUARD_GAP,
            ISOLATION_PANEL_Z / 2.0 + 7.5,
        );
        let guarded_via = centered_cylinder(
            format!("closed_sensor_cable_feedthrough_isolation_guard_ring_{i}"),
            9.0,
            3.0,
            28,
        )
        .translate(x, 34.0, ISOLATION_PANEL_Z / 2.0 + 7.5);
        let guarded_cut = centered_cylinder(
            format!("closed_sensor_cable_feedthrough_isolation_guard_opening_{i}"),
            4.0,
            4.0,
            24,
        )
        .translate(x, 34.0, ISOLATION_PANEL_Z / 2.0 + 7.5);
        pads = pads + dielectric + electrode_a + electrode_b + (guarded_via - guarded_cut);
    }

    let warning_land = centered_cube(
        "closed_sensor_cable_feedthrough_isolation_certified_tester_placeholder_land",
        ISOLATION_PANEL_X - 54.0,
        12.0,
        5.0,
    )
    .translate(
        0.0,
        ISOLATION_PANEL_Y / 2.0 - 18.0,
        ISOLATION_PANEL_Z / 2.0 + 2.5,
    );

    (panel + pads + warning_land).translate(
        ISOLATION_CENTER_X,
        ISOLATION_CENTER_Y,
        BASE_Z + ISOLATION_PANEL_Z / 2.0,
    )
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_sensor_cable_feedthrough_barcode_certificate_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_sensor_cable_feedthrough_barcode_certificate_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_barcode_land_{i}"),
                76.0,
                16.0,
                4.0,
            )
            .translate(
                centered_position(i % 2, 2, 88.0),
                centered_position(i / 2, 2, 32.0),
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_certificate_land_{i}"),
                88.0,
                24.0,
                4.0,
            )
            .translate(
                centered_position(i, CERTIFICATE_LANDS, 98.0),
                -TRACE_PANEL_Y / 2.0 + 18.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }
    for i in 0..FIDUCIALS {
        let (x, y) = trace_fiducial_point(i);
        lands = lands
            + centered_cylinder(
                format!("closed_sensor_cable_feedthrough_trace_fiducial_{i}"),
                5.0,
                4.0,
                24,
            )
            .translate(x, y, TRACE_PANEL_Z / 2.0 + 2.0);
    }

    (panel + lands).translate(TRACE_CENTER_X, TRACE_CENTER_Y, BASE_Z + TRACE_PANEL_Z / 2.0)
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "closed_sensor_cable_feedthrough_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    );
    let mut lanes = Part::empty("closed_sensor_cable_feedthrough_disposition_lanes");
    for i in 0..DISPOSITION_LANES {
        let x = disposition_lane_x(i);
        let lane_floor = centered_cube(
            format!(
                "closed_sensor_cable_feedthrough_{}_lane_floor",
                disposition_lane_name(i)
            ),
            LANE_WIDTH_X,
            LANE_BANK_Y - 26.0,
            5.0,
        )
        .translate(x, 0.0, LANE_BANK_Z / 2.0 + 2.5);
        let left_rail = centered_cube(
            format!(
                "closed_sensor_cable_feedthrough_{}_lane_left_rail",
                disposition_lane_name(i)
            ),
            6.0,
            LANE_BANK_Y - 26.0,
            LANE_RAIL_Z,
        )
        .translate(
            x - LANE_WIDTH_X / 2.0,
            0.0,
            LANE_BANK_Z / 2.0 + LANE_RAIL_Z / 2.0,
        );
        let right_rail = centered_cube(
            format!(
                "closed_sensor_cable_feedthrough_{}_lane_right_rail",
                disposition_lane_name(i)
            ),
            6.0,
            LANE_BANK_Y - 26.0,
            LANE_RAIL_Z,
        )
        .translate(
            x + LANE_WIDTH_X / 2.0,
            0.0,
            LANE_BANK_Z / 2.0 + LANE_RAIL_Z / 2.0,
        );
        let gate = centered_cube(
            format!(
                "closed_sensor_cable_feedthrough_{}_lane_gate",
                disposition_lane_name(i)
            ),
            LANE_WIDTH_X - 12.0,
            8.0,
            42.0,
        )
        .translate(x, LANE_BANK_Y / 2.0 - 16.0, LANE_BANK_Z / 2.0 + 21.0);
        lanes = lanes + lane_floor + left_rail + right_rail + gate;
    }

    (bank + lanes).translate(LANE_CENTER_X, LANE_CENTER_Y, BASE_Z + LANE_BANK_Z / 2.0)
}

fn clean_used_segregation() -> Part {
    let base = centered_cube(
        "closed_sensor_cable_feedthrough_clean_used_segregation_base",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let divider = centered_cube(
        "closed_sensor_cable_feedthrough_clean_used_center_divider",
        10.0,
        SEGREGATION_Y,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 + CLEAN_USED_DIVIDER_Z / 2.0);
    let clean_header = centered_cube(
        "closed_sensor_cable_feedthrough_clean_incoming_header_land",
        SEGREGATION_X / 2.0 - CLEAN_USED_GAP / 2.0,
        14.0,
        8.0,
    )
    .translate(
        -(SEGREGATION_X / 4.0 + CLEAN_USED_GAP / 4.0),
        SEGREGATION_Y / 2.0 - 18.0,
        SEGREGATION_Z / 2.0 + 4.0,
    );
    let used_header = centered_cube(
        "closed_sensor_cable_feedthrough_used_quarantine_header_land",
        SEGREGATION_X / 2.0 - CLEAN_USED_GAP / 2.0,
        14.0,
        8.0,
    )
    .translate(
        SEGREGATION_X / 4.0 + CLEAN_USED_GAP / 4.0,
        SEGREGATION_Y / 2.0 - 18.0,
        SEGREGATION_Z / 2.0 + 4.0,
    );

    let mut pockets = Part::empty("closed_sensor_cable_feedthrough_clean_used_pockets");
    for i in 0..CLEAN_LANES {
        pockets = pockets
            + segregation_pocket(
                &format!("closed_sensor_cable_feedthrough_clean_coupon_pocket_{i}"),
                -(SEGREGATION_X / 4.0 + CLEAN_USED_GAP / 4.0),
                centered_position(i, CLEAN_LANES, 42.0) - 18.0,
            );
    }
    for i in 0..USED_LANES {
        pockets = pockets
            + segregation_pocket(
                &format!("closed_sensor_cable_feedthrough_used_coupon_pocket_{i}"),
                SEGREGATION_X / 4.0 + CLEAN_USED_GAP / 4.0,
                centered_position(i, USED_LANES, 42.0) - 18.0,
            );
    }

    (base + divider + clean_header + used_header + pockets).translate(
        SEGREGATION_CENTER_X,
        SEGREGATION_CENTER_Y,
        BASE_Z + SEGREGATION_Z / 2.0,
    )
}

fn segregation_pocket(name: &str, x: f64, y: f64) -> Part {
    let rim = centered_cube(format!("{name}_rim"), 118.0, 32.0, 16.0).translate(x, y, 14.0);
    let recess = centered_cube(format!("{name}_recess"), 98.0, 18.0, 18.0).translate(x, y, 14.0);
    rim - recess
}

fn evidence_camera_bridge() -> Part {
    let mut posts = Part::empty("closed_sensor_cable_feedthrough_camera_bridge_posts");
    for (i, (x, y)) in bridge_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_camera_bridge_post_{i}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_UNDERSIDE_Z - BASE_Z,
            )
            .translate(*x, *y, (BRIDGE_UNDERSIDE_Z + BASE_Z) / 2.0);
    }

    let cross_beam = centered_cube(
        "closed_sensor_cable_feedthrough_camera_bridge_cross_beam",
        BRIDGE_SPAN_X,
        26.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER_X,
        BRIDGE_CENTER_Y,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let fore_aft_beam = centered_cube(
        "closed_sensor_cable_feedthrough_camera_bridge_fore_aft_beam",
        26.0,
        BRIDGE_POST_SPAN_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER_X,
        BRIDGE_CENTER_Y,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    let mut cameras = Part::empty("closed_sensor_cable_feedthrough_camera_mounts");
    for i in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("closed_sensor_cable_feedthrough_evidence_camera_mount_{i}"),
                48.0,
                36.0,
                18.0,
            )
            .translate(
                centered_position(i, CAMERA_COUNT, 170.0),
                BRIDGE_CENTER_Y - 8.0,
                BRIDGE_UNDERSIDE_Z - 12.0,
            )
            + centered_cylinder(
                format!("closed_sensor_cable_feedthrough_evidence_camera_lens_clearance_{i}"),
                11.0,
                20.0,
                28,
            )
            .translate(
                centered_position(i, CAMERA_COUNT, 170.0),
                BRIDGE_CENTER_Y - 8.0,
                BRIDGE_UNDERSIDE_Z - 22.0,
            );
    }

    let light_bars = centered_cube(
        "closed_sensor_cable_feedthrough_front_evidence_light_bar",
        BRIDGE_SPAN_X - 96.0,
        8.0,
        8.0,
    )
    .translate(
        BRIDGE_CENTER_X,
        BRIDGE_CENTER_Y - BRIDGE_POST_SPAN_Y / 2.0 + 22.0,
        BRIDGE_UNDERSIDE_Z - 26.0,
    ) + centered_cube(
        "closed_sensor_cable_feedthrough_rear_evidence_light_bar",
        BRIDGE_SPAN_X - 96.0,
        8.0,
        8.0,
    )
    .translate(
        BRIDGE_CENTER_X,
        BRIDGE_CENTER_Y + BRIDGE_POST_SPAN_Y / 2.0 - 22.0,
        BRIDGE_UNDERSIDE_Z - 26.0,
    );

    posts + cross_beam + fore_aft_beam + cameras + light_bars
}

fn robot_service_keepout_gauges() -> Part {
    let robot_frame = keepout_frame(
        "closed_sensor_cable_feedthrough_robot_approach_keepout",
        ROBOT_APPROACH_KEEP_OUT_X,
        ROBOT_APPROACH_KEEP_OUT_Y,
        0.0,
        -44.0,
        BASE_Z + 9.0,
    );
    let rear_panel = keepout_frame(
        "closed_sensor_cable_feedthrough_rear_panel_service_keepout",
        PANEL_X + 92.0,
        REAR_PANEL_SERVICE_KEEP_OUT_Y,
        0.0,
        PANEL_CENTER_Y,
        BASE_Z + 12.0,
    );
    let side_cable = keepout_frame(
        "closed_sensor_cable_feedthrough_side_cable_service_keepout",
        SIDE_CABLE_SERVICE_KEEP_OUT_X,
        360.0,
        STATION_X / 2.0 - SIDE_CABLE_SERVICE_KEEP_OUT_X / 2.0 - 42.0,
        18.0,
        BASE_Z + 15.0,
    );
    let top_bridge = centered_cube(
        "closed_sensor_cable_feedthrough_top_bridge_service_height_gauge",
        42.0,
        42.0,
        TOP_BRIDGE_SERVICE_KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + 78.0,
        STATION_Y / 2.0 - 92.0,
        BASE_Z + TOP_BRIDGE_SERVICE_KEEP_OUT_Z / 2.0,
    );

    robot_frame + rear_panel + side_cable + top_bridge
}

fn keepout_frame(name: &str, width: f64, depth: f64, x: f64, y: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front_bar"), width, 8.0, 8.0).translate(
        x,
        y - depth / 2.0,
        z,
    );
    let rear =
        centered_cube(format!("{name}_rear_bar"), width, 8.0, 8.0).translate(x, y + depth / 2.0, z);
    let left =
        centered_cube(format!("{name}_left_bar"), 8.0, depth, 8.0).translate(x - width / 2.0, y, z);
    let right = centered_cube(format!("{name}_right_bar"), 8.0, depth, 8.0).translate(
        x + width / 2.0,
        y,
        z,
    );
    front + rear + left + right
}

fn feedthrough_x(index: usize) -> f64 {
    centered_position(index, FEEDTHROUGH_COUNT, FEEDTHROUGH_PITCH_X)
}

fn pressure_port_y(index: usize) -> f64 {
    centered_position(index, PRESSURE_PORTS_PER_FEEDTHROUGH, 36.0)
}

fn witness_well_center(index: usize) -> (f64, f64) {
    let col = index % WITNESS_WELL_COLS;
    let row = index / WITNESS_WELL_COLS;
    (
        centered_position(col, WITNESS_WELL_COLS, 58.0),
        centered_position(row, WITNESS_WELL_ROWS, 76.0),
    )
}

fn disposition_lane_x(index: usize) -> f64 {
    centered_position(index, DISPOSITION_LANES, LANE_PITCH_X)
}

fn disposition_lane_name(index: usize) -> &'static str {
    match index {
        RELEASE_LANE_INDEX => "release",
        HOLD_LANE_INDEX => "hold",
        REJECT_LANE_INDEX => "reject",
        _ => "unknown",
    }
}

fn trace_fiducial_point(index: usize) -> (f64, f64) {
    match index {
        0 => (-TRACE_PANEL_X / 2.0 + 18.0, -TRACE_PANEL_Y / 2.0 + 18.0),
        1 => (TRACE_PANEL_X / 2.0 - 18.0, -TRACE_PANEL_Y / 2.0 + 18.0),
        2 => (-TRACE_PANEL_X / 2.0 + 18.0, TRACE_PANEL_Y / 2.0 - 18.0),
        3 => (TRACE_PANEL_X / 2.0 - 18.0, TRACE_PANEL_Y / 2.0 - 18.0),
        _ => (0.0, 0.0),
    }
}

fn bridge_post_points() -> [(f64, f64); 4] {
    [
        (
            BRIDGE_CENTER_X - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
            BRIDGE_CENTER_Y - BRIDGE_POST_SPAN_Y / 2.0 + BRIDGE_POST_Y / 2.0,
        ),
        (
            BRIDGE_CENTER_X + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
            BRIDGE_CENTER_Y - BRIDGE_POST_SPAN_Y / 2.0 + BRIDGE_POST_Y / 2.0,
        ),
        (
            BRIDGE_CENTER_X - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
            BRIDGE_CENTER_Y + BRIDGE_POST_SPAN_Y / 2.0 - BRIDGE_POST_Y / 2.0,
        ),
        (
            BRIDGE_CENTER_X + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
            BRIDGE_CENTER_Y + BRIDGE_POST_SPAN_Y / 2.0 - BRIDGE_POST_Y / 2.0,
        ),
    ]
}

fn centered_position(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-548.0, -354.0),
        (-348.0, -354.0),
        (-148.0, -354.0),
        (148.0, -354.0),
        (348.0, -354.0),
        (548.0, -354.0),
        (-548.0, 350.0),
        (548.0, 350.0),
    ]
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[cfg(test)]
impl Rect {
    const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

#[cfg(test)]
fn station_rect() -> Rect {
    Rect::new(0.0, 0.0, STATION_X, STATION_Y)
}

#[cfg(test)]
fn rect_inside(inner: Rect, outer: Rect, margin: f64) -> bool {
    inner.x - inner.w / 2.0 >= outer.x - outer.w / 2.0 + margin
        && inner.x + inner.w / 2.0 <= outer.x + outer.w / 2.0 - margin
        && inner.y - inner.h / 2.0 >= outer.y - outer.h / 2.0 + margin
        && inner.y + inner.h / 2.0 <= outer.y + outer.h / 2.0 - margin
}

#[cfg(test)]
fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

#[cfg(test)]
fn panel_rect() -> Rect {
    Rect::new(PANEL_CENTER_Y * 0.0, PANEL_CENTER_Y, PANEL_X, PANEL_Y)
}

#[cfg(test)]
fn comb_rect() -> Rect {
    Rect::new(COMB_CENTER_X, COMB_CENTER_Y, COMB_X, COMB_Y)
}

#[cfg(test)]
fn manifold_rect() -> Rect {
    Rect::new(MANIFOLD_CENTER_X, MANIFOLD_CENTER_Y, MANIFOLD_X, MANIFOLD_Y)
}

#[cfg(test)]
fn witness_rect() -> Rect {
    Rect::new(WELL_CENTER_X, WELL_CENTER_Y, WELL_PLATE_X, WELL_PLATE_Y)
}

#[cfg(test)]
fn bend_rect() -> Rect {
    Rect::new(BEND_CENTER_X, BEND_CENTER_Y, BEND_GAUGE_X, BEND_GAUGE_Y)
}

#[cfg(test)]
fn isolation_rect() -> Rect {
    Rect::new(
        ISOLATION_CENTER_X,
        ISOLATION_CENTER_Y,
        ISOLATION_PANEL_X,
        ISOLATION_PANEL_Y,
    )
}

#[cfg(test)]
fn lane_rect() -> Rect {
    Rect::new(LANE_CENTER_X, LANE_CENTER_Y, LANE_BANK_X, LANE_BANK_Y)
}

#[cfg(test)]
fn segregation_rect() -> Rect {
    Rect::new(
        SEGREGATION_CENTER_X,
        SEGREGATION_CENTER_Y,
        SEGREGATION_X,
        SEGREGATION_Y,
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sensor_cable_feedthrough_leak_test_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS[0],
            "output/closed_sensor_cable_feedthrough_leak_test_station_base_containment_tray.stl"
        );
        assert_eq!(
            OUTPUTS[12],
            "output/closed_sensor_cable_feedthrough_leak_test_station_assembly.stl"
        );
    }

    #[test]
    fn required_feature_groups_cover_requested_validation_station() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"base_containment_tray"));
        assert!(REQUIRED_FEATURES.contains(&"wall_feedthrough_coupon_panel"));
        assert!(REQUIRED_FEATURES.contains(&"cable_strain_relief_comb"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_port_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"dye_water_ingress_witness_wells"));
        assert!(REQUIRED_FEATURES.contains(&"bend_radius_gauge"));
        assert!(REQUIRED_FEATURES.contains(&"electrical_isolation_placeholder_pads"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepout_gauges"));
    }

    #[test]
    fn feedthrough_counts_match_leak_and_strain_relief_interfaces() {
        assert_eq!(FEEDTHROUGH_COUNT, 6);
        assert_eq!(CABLE_CLASSES.len(), FEEDTHROUGH_COUNT);
        assert_eq!(WALL_COUPON_GASKET_LANDS, FEEDTHROUGH_COUNT);
        assert_eq!(COUPON_CLAMP_TABS, FEEDTHROUGH_COUNT * 2);
        assert_eq!(STRAIN_RELIEF_LANES, FEEDTHROUGH_COUNT);
        assert_eq!(STRAIN_RELIEF_CLAMP_POINTS, FEEDTHROUGH_COUNT * 2);
        assert!(GROMMET_LAND_D >= FEEDTHROUGH_BORE_D + 20.0);
    }

    #[test]
    fn pressure_decay_and_ingress_witness_features_are_complete() {
        assert_eq!(PRESSURE_DECAY_PORTS, 12);
        assert_eq!(PRESSURE_DECAY_PORTS, FEEDTHROUGH_COUNT * 2);
        assert_eq!(REFERENCE_PRESSURE_PORTS, 2);
        assert_eq!(DYE_REFERENCE_WELLS, 2);
        assert_eq!(WITNESS_WELL_COUNT, FEEDTHROUGH_COUNT + DYE_REFERENCE_WELLS);
        assert_eq!(WITNESS_WELL_COUNT, WITNESS_WELL_COLS * WITNESS_WELL_ROWS);
        assert!(PRESSURE_PORT_D > ANNULUS_PORT_D);
        assert!(WITNESS_WELL_DEPTH > 2.0 * BASIN_DEPTH);
    }

    #[test]
    fn bend_and_electrical_placeholders_have_safe_spacing() {
        assert_eq!(BEND_MANDREL_COUNT, 3);
        assert_eq!(BEND_MANDREL_RADII, [25.0, 40.0, 60.0]);
        assert!(MIN_BEND_RADIUS >= CABLE_NOMINAL_D * 4.0);
        assert!(CABLE_SLOT_D > CABLE_NOMINAL_D + 3.0);
        assert_eq!(ISOLATION_PAD_COUNT, FEEDTHROUGH_COUNT);
        assert!(ISOLATION_TEST_CLEARANCE_MM >= 2.0 * ISOLATION_GUARD_GAP);
        assert!(ISOLATION_PAD_X > ISOLATION_PAD_Y);
    }

    #[test]
    fn layout_modules_fit_the_containment_tray() {
        for rect in [
            panel_rect(),
            comb_rect(),
            manifold_rect(),
            witness_rect(),
            bend_rect(),
            isolation_rect(),
            lane_rect(),
            segregation_rect(),
        ] {
            assert!(rect_inside(rect, station_rect(), 18.0));
        }
        assert!(!rects_overlap(witness_rect(), lane_rect()));
        assert!(!rects_overlap(bend_rect(), segregation_rect()));
        assert!(!rects_overlap(isolation_rect(), witness_rect()));
        assert!(!rects_overlap(isolation_rect(), bend_rect()));
    }

    #[test]
    fn disposition_traceability_and_segregation_are_explicit() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(disposition_lane_name(RELEASE_LANE_INDEX), "release");
        assert_eq!(disposition_lane_name(HOLD_LANE_INDEX), "hold");
        assert_eq!(disposition_lane_name(REJECT_LANE_INDEX), "reject");
        assert_eq!(BARCODE_LANDS, 4);
        assert_eq!(CERTIFICATE_LANDS, 2);
        assert_eq!(FIDUCIALS, 4);
        assert_eq!(CLEAN_LANES, 2);
        assert_eq!(USED_LANES, 2);
        assert!(CLEAN_USED_DIVIDER_Z >= 60.0);
        assert!(CLEAN_USED_GAP >= 72.0);
    }

    #[test]
    fn evidence_bridge_and_service_keepouts_clear_the_panel() {
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LIGHT_BAR_COUNT, 2);
        assert!(BRIDGE_SPAN_X > PANEL_X);
        assert!(BRIDGE_UNDERSIDE_Z > PANEL_CENTER_Z + 80.0);
        assert_eq!(KEEP_OUT_FRAME_COUNT, 4);
        assert!(ROBOT_APPROACH_KEEP_OUT_X > COMB_X);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 330.0);
        assert!(REAR_PANEL_SERVICE_KEEP_OUT_Y >= 180.0);
        assert!(SIDE_CABLE_SERVICE_KEEP_OUT_X >= 200.0);
        assert!(TOP_BRIDGE_SERVICE_KEEP_OUT_Z >= 280.0);
    }

    #[test]
    fn feedthrough_positions_stay_inside_the_coupon_panel() {
        for i in 0..FEEDTHROUGH_COUNT {
            assert!(
                feedthrough_x(i).abs() + GROMMET_LAND_D / 2.0 + 42.0 < PANEL_X / 2.0,
                "feedthrough {i} does not leave enough clamp land"
            );
        }
        assert!(PANEL_MAIN_FEEDTHROUGH_Z > BASE_Z + 110.0);
        assert!(PANEL_ANNULUS_PORT_Z > PANEL_MAIN_FEEDTHROUGH_Z);
        assert!(PANEL_DRAIN_PORT_Z < PANEL_MAIN_FEEDTHROUGH_Z);
    }
}
