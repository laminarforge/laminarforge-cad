use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed tissue-chip perfusion lane pressure-relief and burst-guard validation station.
//
// Intent:
// - Challenge each closed chip perfusion lane with visible restriction, relief,
//   burst-membrane, pressure-transducer, dye-recovery, isolate, and vent cues
//   before the lane is released for tissue-chip operation.
// - Prevent overpressure from rupturing chips or pushing dye/media across lanes
//   by keeping each lane mechanically separated through relief capture and
//   disposition routing.
// - Provide robot handling, service keepout, barcode custody, and camera witness
//   geometry in the same validation envelope.
//
// This is architecture CAD for fixture planning. It is not a pressure-rated
// device drawing, sterile fluid-path release drawing, or acceptance criterion.

const OUTPUTS: &[&str] = &[
    "output/closed_chip_lane_pressure_relief_burst_guard_station_base_containment_tray.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_lane_restrictor_coupon_sled.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_relief_valve_cartridge_nests.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_burst_membrane_witness_windows.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_calibrated_pressure_transducer_pockets.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_isolate_vent_routing_manifold.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_dye_recovery_well_rack.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_release_hold_reject_lanes.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_barcode_custody_panel.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_clean_used_segregation_cups.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_evidence_camera_bridge.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_robot_service_keepout_gauges.stl",
    "output/closed_chip_lane_pressure_relief_burst_guard_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "lane_restrictor_coupons",
    "relief_valve_cartridge_nests",
    "burst_membrane_witness_windows",
    "calibrated_pressure_transducer_pockets",
    "dye_recovery_wells",
    "isolate_vent_routing",
    "release_hold_reject_lanes",
    "barcode_custody",
    "robot_service_keepouts",
    "closed_leak_containment",
    "cross_lane_separation",
    "assembly_export",
];

const LIMITATIONS: &[&str] = &[
    "mechanical_validation_packaging_only",
    "not_pressure_rated",
    "not_sterile_barrier_definition",
    "not_release_acceptance_criterion",
];

const STATION_X: f64 = 1600.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 7.0;

const LANE_COUNT: usize = 8;
const LANE_PITCH_Y: f64 = 42.0;
const LANE_RUN_X: f64 = 590.0;
const LANE_TRACE_W: f64 = 7.2;
const LANE_TRACE_Z: f64 = 5.2;
const LANE_INLET_X: f64 = -320.0;
const LANE_OUTLET_X: f64 = 320.0;
const LANE_PRESSURE_LIMIT_KPA: f64 = 35.0;
const BURST_GUARD_TARGET_KPA: f64 = 48.0;
const CROSS_LANE_ISOLATION_GAP: f64 = 18.0;

const LANE_SLED_X: f64 = 700.0;
const LANE_SLED_Y: f64 = 360.0;
const LANE_SLED_Z: f64 = 38.0;
const LANE_SLED_POS: (f64, f64) = (-160.0, 154.0);
const RESTRICTOR_COUPON_X: f64 = 62.0;
const RESTRICTOR_COUPON_Y: f64 = 24.0;
const RESTRICTOR_COUPON_Z: f64 = 10.0;
const RESTRICTOR_COUPON_COUNT: usize = LANE_COUNT;
const RESTRICTOR_BORE_D: f64 = 4.2;
const LANE_PORT_D: f64 = 8.4;
const LANE_PORT_PAD_D: f64 = 20.0;

const RELIEF_X: f64 = 330.0;
const RELIEF_Y: f64 = 318.0;
const RELIEF_Z: f64 = 56.0;
const RELIEF_POS: (f64, f64) = (410.0, 176.0);
const RELIEF_ROWS: usize = 4;
const RELIEF_COLS: usize = 2;
const RELIEF_CARTRIDGE_COUNT: usize = RELIEF_ROWS * RELIEF_COLS;
const RELIEF_PITCH_X: f64 = 122.0;
const RELIEF_PITCH_Y: f64 = 64.0;
const RELIEF_CARTRIDGE_D: f64 = 27.0;
const RELIEF_CLEARANCE_D: f64 = 30.0;
const RELIEF_RETENTION_EARS: usize = RELIEF_CARTRIDGE_COUNT * 2;

const BURST_X: f64 = 690.0;
const BURST_Y: f64 = 138.0;
const BURST_Z: f64 = 30.0;
const BURST_POS: (f64, f64) = (-160.0, -122.0);
const BURST_WINDOW_COUNT: usize = LANE_COUNT;
const BURST_WINDOW_X: f64 = 54.0;
const BURST_WINDOW_Y: f64 = 30.0;
const BURST_MEMBRANE_D: f64 = 22.0;
const BURST_WITNESS_TICKS_PER_LANE: usize = 4;

const TRANSDUCER_X: f64 = 330.0;
const TRANSDUCER_Y: f64 = 150.0;
const TRANSDUCER_Z: f64 = 50.0;
const TRANSDUCER_POS: (f64, f64) = (410.0, -86.0);
const TRANSDUCER_ROWS: usize = 2;
const TRANSDUCER_COLS: usize = 4;
const TRANSDUCER_COUNT: usize = TRANSDUCER_ROWS * TRANSDUCER_COLS;
const TRANSDUCER_PITCH_X: f64 = 78.0;
const TRANSDUCER_PITCH_Y: f64 = 54.0;
const TRANSDUCER_POCKET_X: f64 = 42.0;
const TRANSDUCER_POCKET_Y: f64 = 26.0;
const TRANSDUCER_PORT_D: f64 = 8.0;
const CALIBRATION_TOKEN_COUNT: usize = 4;

const ROUTING_X: f64 = 340.0;
const ROUTING_Y: f64 = 150.0;
const ROUTING_Z: f64 = 44.0;
const ROUTING_POS: (f64, f64) = (-430.0, -300.0);
const ISOLATE_VALVE_COUNT: usize = LANE_COUNT;
const VENT_ROUTE_COUNT: usize = LANE_COUNT;
const VENT_FILTER_COUNT: usize = 4;
const ROUTE_CHANNEL_W: f64 = 5.8;
const ROUTE_HEADER_W: f64 = 9.0;

const DYE_X: f64 = 300.0;
const DYE_Y: f64 = 150.0;
const DYE_Z: f64 = 48.0;
const DYE_POS: (f64, f64) = (-90.0, -300.0);
const DYE_RECOVERY_WELLS: usize = LANE_COUNT;
const DYE_WELL_D: f64 = 24.0;
const DYE_OVERFLOW_WELLS: usize = 2;
const DYE_WELL_VOLUME_UL: f64 = 900.0;

const STATUS_X: f64 = 260.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 34.0;
const STATUS_POS: (f64, f64) = (220.0, -300.0);
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 34.0;
const STATUS_SLOT_Y: f64 = 24.0;

const CUSTODY_X: f64 = 220.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 14.0;
const CUSTODY_POS: (f64, f64) = (500.0, -276.0);
const BARCODE_LANDS: usize = LANE_COUNT;
const CUSTODY_TOKEN_SLOTS: usize = 6;
const TAMPER_SEAL_POSTS: usize = 4;
const BARCODE_LAND_X: f64 = 54.0;
const BARCODE_LAND_Y: f64 = 18.0;

const SEGREGATION_X: f64 = 170.0;
const SEGREGATION_Y: f64 = 80.0;
const SEGREGATION_Z: f64 = 34.0;
const SEGREGATION_POS: (f64, f64) = (680.0, 26.0);
const CLEAN_CAP_CUPS: usize = 4;
const USED_BURST_DISC_CUPS: usize = 4;

const CAMERA_X: f64 = 1010.0;
const CAMERA_Y: f64 = 52.0;
const CAMERA_Z: f64 = 216.0;
const CAMERA_POS: (f64, f64) = (-28.0, 24.0);
const CAMERA_COUNT: usize = 3;
const CAMERA_PITCH_X: f64 = 310.0;
const CAMERA_VIEW_WINDOW_X: f64 = 104.0;
const CAMERA_VIEW_WINDOW_Y: f64 = 28.0;

const KEEP_OUT_ZONE_COUNT: usize = 6;
const ROBOT_SWEEP_CLEARANCE: f64 = 360.0;
const FRONT_DYE_SERVICE_CLEARANCE: f64 = 180.0;
const REAR_TUBING_SERVICE_CLEARANCE: f64 = 230.0;
const RELIEF_CARTRIDGE_LIFT_CLEARANCE: f64 = 170.0;
const TRANSDUCER_CABLE_SERVICE_CLEARANCE: f64 = 190.0;
const CAMERA_LIFT_CLEARANCE: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_tray();
    export(&base, OUTPUTS[0]);

    let restrictors = lane_restrictor_coupon_sled();
    export(&restrictors, OUTPUTS[1]);

    let relief = relief_valve_cartridge_nests();
    export(&relief, OUTPUTS[2]);

    let burst = burst_membrane_witness_windows();
    export(&burst, OUTPUTS[3]);

    let transducers = calibrated_pressure_transducer_pockets();
    export(&transducers, OUTPUTS[4]);

    let routing = isolate_vent_routing_manifold();
    export(&routing, OUTPUTS[5]);

    let dye = dye_recovery_well_rack();
    export(&dye, OUTPUTS[6]);

    let status = release_hold_reject_lanes();
    export(&status, OUTPUTS[7]);

    let custody = barcode_custody_panel();
    export(&custody, OUTPUTS[8]);

    let segregation = clean_used_segregation_cups();
    export(&segregation, OUTPUTS[9]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[10]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + restrictors.translate(LANE_SLED_POS.0, LANE_SLED_POS.1, insert_z(LANE_SLED_Z))
        + relief.translate(RELIEF_POS.0, RELIEF_POS.1, insert_z(RELIEF_Z))
        + burst.translate(BURST_POS.0, BURST_POS.1, insert_z(BURST_Z))
        + transducers.translate(TRANSDUCER_POS.0, TRANSDUCER_POS.1, insert_z(TRANSDUCER_Z))
        + routing.translate(ROUTING_POS.0, ROUTING_POS.1, insert_z(ROUTING_Z))
        + dye.translate(DYE_POS.0, DYE_POS.1, insert_z(DYE_Z))
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, BASE_Z / 2.0)
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Closed chip-lane pressure-relief and burst-guard validation station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm closed containment tray with {RIM_Z:.0}mm rim"
    );
    println!(
        "  Perfusion lanes:        {LANE_COUNT} isolated lanes, {RESTRICTOR_COUPON_COUNT} restrictor coupons, {:.0} kPa nominal relief challenge, {:.0} kPa burst guard witness target",
        LANE_PRESSURE_LIMIT_KPA,
        BURST_GUARD_TARGET_KPA
    );
    println!(
        "  Relief instrumentation: {RELIEF_CARTRIDGE_COUNT} relief valve cartridge nests, {RELIEF_RETENTION_EARS} retention ears, {BURST_WINDOW_COUNT} burst membrane witness windows, {TRANSDUCER_COUNT} calibrated transducer pockets"
    );
    println!(
        "  Closed routing:         {ISOLATE_VALVE_COUNT} isolate toggles, {VENT_ROUTE_COUNT} vent routes, {VENT_FILTER_COUNT} vent filter placeholders, {DYE_RECOVERY_WELLS} dye recovery wells"
    );
    println!(
        "  Custody/status:         {} disposition lanes ({}, {}, {}), {BARCODE_LANDS} barcode lands, {CUSTODY_TOKEN_SLOTS} custody token slots",
        STATUS_LANES, STATUS_LANE_NAMES[0], STATUS_LANE_NAMES[1], STATUS_LANE_NAMES[2]
    );
    println!(
        "  Access controls:        {CAMERA_COUNT} evidence camera mounts, {KEEP_OUT_ZONE_COUNT} robot/service keepout gauges, {ROBOT_SWEEP_CLEARANCE:.0}mm robot sweep, {CROSS_LANE_ISOLATION_GAP:.0}mm minimum cross-lane guard gap"
    );
    println!(
        "  Limitations:            {} limitation markers; {} required feature groups",
        LIMITATIONS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn lane_y(index: usize) -> f64 {
    centered_index(index, LANE_COUNT, LANE_PITCH_Y)
}

fn assert_layout() {
    assert_eq!(LANE_COUNT, RESTRICTOR_COUPON_COUNT);
    assert_eq!(LANE_COUNT, BURST_WINDOW_COUNT);
    assert_eq!(LANE_COUNT, DYE_RECOVERY_WELLS);
    assert_eq!(LANE_COUNT, ISOLATE_VALVE_COUNT);
    assert_eq!(LANE_COUNT, VENT_ROUTE_COUNT);
    assert_eq!(LANE_COUNT, TRANSDUCER_COUNT);
    assert_eq!(LANE_COUNT, RELIEF_CARTRIDGE_COUNT);
    assert_eq!(RELIEF_CARTRIDGE_COUNT, RELIEF_ROWS * RELIEF_COLS);
    assert_eq!(TRANSDUCER_COUNT, TRANSDUCER_ROWS * TRANSDUCER_COLS);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= LANE_COUNT);
    assert!(
        BURST_GUARD_TARGET_KPA > LANE_PRESSURE_LIMIT_KPA,
        "burst guard challenge must sit above relief setpoint"
    );
    assert!(
        CROSS_LANE_ISOLATION_GAP >= 2.0 * LANE_TRACE_W,
        "cross-lane guard must exceed two lane trace widths"
    );
    assert!(
        RELIEF_CLEARANCE_D > RELIEF_CARTRIDGE_D,
        "relief cartridges need insertion clearance"
    );
    assert!(
        DYE_WELL_VOLUME_UL >= 750.0,
        "dye recovery wells should capture repeated challenge pulses"
    );

    for (name, pos, width, depth) in module_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station footprint"
        );
    }

    let restrictors = rect(LANE_SLED_POS, LANE_SLED_X, LANE_SLED_Y);
    let relief = rect(RELIEF_POS, RELIEF_X, RELIEF_Y);
    let burst = rect(BURST_POS, BURST_X, BURST_Y);
    let transducers = rect(TRANSDUCER_POS, TRANSDUCER_X, TRANSDUCER_Y);
    let routing = rect(ROUTING_POS, ROUTING_X, ROUTING_Y);
    let dye = rect(DYE_POS, DYE_X, DYE_Y);
    let status = rect(STATUS_POS, STATUS_X, STATUS_Y);
    let custody = rect(CUSTODY_POS, CUSTODY_X, CUSTODY_Y);
    let segregation = rect(SEGREGATION_POS, SEGREGATION_X, SEGREGATION_Y);

    assert!(!rects_overlap(restrictors, relief, 18.0));
    assert!(!rects_overlap(restrictors, burst, 12.0));
    assert!(!rects_overlap(relief, transducers, 16.0));
    assert!(!rects_overlap(burst, transducers, 16.0));
    assert!(!rects_overlap(routing, dye, 18.0));
    assert!(!rects_overlap(dye, status, 18.0));
    assert!(!rects_overlap(status, custody, 18.0));
    assert!(!rects_overlap(transducers, segregation, 16.0));
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "lane_restrictor_coupon_sled",
            LANE_SLED_POS,
            LANE_SLED_X,
            LANE_SLED_Y,
        ),
        (
            "relief_valve_cartridge_nests",
            RELIEF_POS,
            RELIEF_X,
            RELIEF_Y,
        ),
        (
            "burst_membrane_witness_windows",
            BURST_POS,
            BURST_X,
            BURST_Y,
        ),
        (
            "calibrated_pressure_transducer_pockets",
            TRANSDUCER_POS,
            TRANSDUCER_X,
            TRANSDUCER_Y,
        ),
        (
            "isolate_vent_routing_manifold",
            ROUTING_POS,
            ROUTING_X,
            ROUTING_Y,
        ),
        ("dye_recovery_well_rack", DYE_POS, DYE_X, DYE_Y),
        ("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        ("barcode_custody_panel", CUSTODY_POS, CUSTODY_X, CUSTODY_Y),
        (
            "clean_used_segregation_cups",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn rect(pos: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        pos.0 - width / 2.0,
        pos.0 + width / 2.0,
        pos.1 - depth / 2.0,
        pos.1 + depth / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64), margin: f64) -> bool {
    a.0 - margin < b.1 && a.1 + margin > b.0 && a.2 - margin < b.3 && a.3 + margin > b.2
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "closed_chip_lane_pressure_relief_base_containment_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_chip_lane_pressure_relief_sumped_basin_recess",
        STATION_X - 116.0,
        STATION_Y - 114.0,
        BASIN_DEPTH + 0.4,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - BASIN_DEPTH / 2.0 + 0.2);
    let front_drain = centered_cylinder(
        "closed_chip_lane_pressure_relief_front_low_point_drain",
        DRAIN_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 + 4.0, 0.0);
    let rear_drain_witness = centered_cylinder(
        "closed_chip_lane_pressure_relief_rear_drain_witness_boss",
        DRAIN_D / 2.0,
        5.0,
        32,
    )
    .translate(
        -(STATION_X / 2.0 - 88.0),
        STATION_Y / 2.0 - 72.0,
        BASE_Z / 2.0 + 2.5,
    );

    deck - basin - front_drain - insert_sockets() - mounting_holes()
        + perimeter_rims()
        + rear_drain_witness
        + lane_zone_dividers()
        + tray_fiducials()
        + cross_lane_leak_witness_channels()
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_chip_lane_pressure_relief_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_chip_lane_pressure_relief_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_chip_lane_pressure_relief_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_chip_lane_pressure_relief_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    left + right + front + rear
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_chip_lane_pressure_relief_insert_registration_sockets");
    for (name, pos, width, depth) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("closed_chip_lane_pressure_relief_mounting_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_mounting_hole_{i:02}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn lane_zone_dividers() -> Part {
    let wet_to_status = centered_cube(
        "closed_chip_lane_pressure_relief_wet_to_status_zone_divider",
        STATION_X - 150.0,
        8.0,
        22.0,
    )
    .translate(0.0, -214.0, BASE_Z / 2.0 + 11.0);
    let valve_to_lane = centered_cube(
        "closed_chip_lane_pressure_relief_valve_to_lane_guard_rail",
        8.0,
        330.0,
        26.0,
    )
    .translate(216.0, 168.0, BASE_Z / 2.0 + 13.0);
    wet_to_status + valve_to_lane
}

fn tray_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_chip_lane_pressure_relief_robot_fiducials");
    for (i, (x, y)) in [
        (-560.0, 350.0),
        (-500.0, -350.0),
        (560.0, 350.0),
        (560.0, -350.0),
        (-30.0, 348.0),
        (518.0, -164.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_fiducial_boss_{i:02}"),
                8.0,
                3.0,
                32,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5);
    }
    fiducials
}

fn cross_lane_leak_witness_channels() -> Part {
    let mut channels = Part::empty("closed_chip_lane_pressure_relief_cross_lane_leak_witness_ribs");
    for i in 0..=LANE_COUNT {
        let y = LANE_SLED_POS.1 - (LANE_COUNT as f64 * LANE_PITCH_Y) / 2.0
            + i as f64 * LANE_PITCH_Y
            - LANE_PITCH_Y / 2.0;
        channels = channels
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_cross_lane_isolation_rib_{i:02}"),
                LANE_SLED_X - 70.0,
                3.5,
                9.0,
            )
            .translate(LANE_SLED_POS.0, y, BASE_Z / 2.0 + 4.5);
    }
    channels
}

fn lane_restrictor_coupon_sled() -> Part {
    let sled = centered_cube(
        "closed_chip_lane_pressure_relief_lane_restrictor_coupon_sled_deck",
        LANE_SLED_X,
        LANE_SLED_Y,
        LANE_SLED_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_restrictor_sled_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_restrictor_sled_features");

    for lane in 0..LANE_COUNT {
        let y = lane_y(lane);
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_flow_trace_groove"),
                LANE_RUN_X,
                LANE_TRACE_W,
                LANE_TRACE_Z,
            )
            .translate(0.0, y, LANE_SLED_Z / 2.0 - LANE_TRACE_Z / 2.0 + 0.2)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_restrictor_coupon_pocket"),
                RESTRICTOR_COUPON_X,
                RESTRICTOR_COUPON_Y,
                RESTRICTOR_COUPON_Z + 0.4,
            )
            .translate(0.0, y, LANE_SLED_Z / 2.0 - RESTRICTOR_COUPON_Z / 2.0 + 0.2)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_inlet_bore"),
                LANE_PORT_D / 2.0,
                LANE_SLED_Z + 2.0,
                28,
            )
            .translate(LANE_INLET_X, y, 0.0)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_outlet_bore"),
                LANE_PORT_D / 2.0,
                LANE_SLED_Z + 2.0,
                28,
            )
            .translate(LANE_OUTLET_X, y, 0.0)
            + centered_cylinder(
                format!(
                    "closed_chip_lane_pressure_relief_lane_{lane:02}_restrictor_micro_bore_witness"
                ),
                RESTRICTOR_BORE_D / 2.0,
                RESTRICTOR_COUPON_X + 2.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, LANE_SLED_Z / 2.0 - 10.0);

        features = features
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_inlet_port_pad"),
                LANE_PORT_PAD_D / 2.0,
                4.0,
                32,
            )
            .translate(LANE_INLET_X, y, LANE_SLED_Z / 2.0 + 2.0)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_outlet_port_pad"),
                LANE_PORT_PAD_D / 2.0,
                4.0,
                32,
            )
            .translate(LANE_OUTLET_X, y, LANE_SLED_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_lane_{lane:02}_restrictor_coupon_key_rail_a"
                ),
                RESTRICTOR_COUPON_X + 20.0,
                3.0,
                7.0,
            )
            .translate(
                0.0,
                y - RESTRICTOR_COUPON_Y / 2.0 - 5.0,
                LANE_SLED_Z / 2.0 + 3.5,
            )
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_lane_{lane:02}_restrictor_coupon_key_rail_b"
                ),
                RESTRICTOR_COUPON_X + 20.0,
                3.0,
                7.0,
            )
            .translate(
                0.0,
                y + RESTRICTOR_COUPON_Y / 2.0 + 5.0,
                LANE_SLED_Z / 2.0 + 3.5,
            )
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_no_crossflow_guard"),
                LANE_RUN_X + 36.0,
                3.0,
                8.0,
            )
            .translate(
                0.0,
                y + LANE_PITCH_Y / 2.0 - CROSS_LANE_ISOLATION_GAP / 2.0,
                LANE_SLED_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_pressure_challenge_tick"),
                12.0,
                12.0,
                5.0,
            )
            .translate(-120.0 + lane as f64 * 8.0, y, LANE_SLED_Z / 2.0 + 2.5);
    }

    sled - cutouts + features + lane_sled_end_bulkheads()
}

fn lane_sled_end_bulkheads() -> Part {
    let inlet = centered_cube(
        "closed_chip_lane_pressure_relief_lane_sled_inlet_bulkhead",
        28.0,
        LANE_SLED_Y - 48.0,
        32.0,
    )
    .translate(LANE_INLET_X - 42.0, 0.0, LANE_SLED_Z / 2.0 + 16.0);
    let outlet = centered_cube(
        "closed_chip_lane_pressure_relief_lane_sled_outlet_relief_bulkhead",
        28.0,
        LANE_SLED_Y - 48.0,
        32.0,
    )
    .translate(LANE_OUTLET_X + 42.0, 0.0, LANE_SLED_Z / 2.0 + 16.0);
    inlet + outlet
}

fn relief_valve_cartridge_nests() -> Part {
    let plate = centered_cube(
        "closed_chip_lane_pressure_relief_valve_cartridge_nest_plate",
        RELIEF_X,
        RELIEF_Y,
        RELIEF_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_valve_nest_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_valve_nest_features");

    for idx in 0..RELIEF_CARTRIDGE_COUNT {
        let row = idx / RELIEF_COLS;
        let col = idx % RELIEF_COLS;
        let x = centered_index(col, RELIEF_COLS, RELIEF_PITCH_X);
        let y = centered_index(row, RELIEF_ROWS, RELIEF_PITCH_Y);

        cutouts = cutouts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_cartridge_clearance_bore"),
                RELIEF_CLEARANCE_D / 2.0,
                RELIEF_Z + 2.0,
                36,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_key_flat_cutout"),
                RELIEF_CLEARANCE_D,
                7.0,
                RELIEF_Z + 2.0,
            )
            .translate(x + RELIEF_CLEARANCE_D / 2.0 - 2.0, y, 0.0);

        features = features
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_seat_ring"),
                (RELIEF_CLEARANCE_D + 9.0) / 2.0,
                5.0,
                36,
            )
            .translate(x, y, RELIEF_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_retention_ear_a"),
                26.0,
                9.0,
                11.0,
            )
            .translate(x - 27.0, y, RELIEF_Z / 2.0 + 5.5)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_retention_ear_b"),
                26.0,
                9.0,
                11.0,
            )
            .translate(x + 27.0, y, RELIEF_Z / 2.0 + 5.5)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_valve_{idx:02}_relief_flow_arrow"),
                30.0,
                4.0,
                5.0,
            )
            .translate(x, y - 28.0, RELIEF_Z / 2.0 + 2.5);
    }

    plate - cutouts + features + relief_header_rails()
}

fn relief_header_rails() -> Part {
    let isolate_header = centered_cube(
        "closed_chip_lane_pressure_relief_valve_isolate_header_rail",
        RELIEF_X - 40.0,
        8.0,
        10.0,
    )
    .translate(0.0, RELIEF_Y / 2.0 - 28.0, RELIEF_Z / 2.0 + 5.0);
    let vent_header = centered_cube(
        "closed_chip_lane_pressure_relief_valve_vent_header_rail",
        RELIEF_X - 40.0,
        8.0,
        10.0,
    )
    .translate(0.0, -(RELIEF_Y / 2.0 - 28.0), RELIEF_Z / 2.0 + 5.0);
    let cartridge_lift_gauge = centered_cube(
        "closed_chip_lane_pressure_relief_valve_cartridge_lift_clearance_gauge",
        14.0,
        RELIEF_Y - 42.0,
        42.0,
    )
    .translate(RELIEF_X / 2.0 - 28.0, 0.0, RELIEF_Z / 2.0 + 21.0);
    isolate_header + vent_header + cartridge_lift_gauge
}

fn burst_membrane_witness_windows() -> Part {
    let bridge = centered_cube(
        "closed_chip_lane_pressure_relief_burst_membrane_window_bridge",
        BURST_X,
        BURST_Y,
        BURST_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_burst_window_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_burst_window_features");

    for lane in 0..LANE_COUNT {
        let x = centered_index(lane, LANE_COUNT, 82.0);
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_burst_lane_{lane:02}_transparent_window_aperture"),
                BURST_WINDOW_X,
                BURST_WINDOW_Y,
                BURST_Z + 2.0,
            )
            .translate(x, 0.0, 0.0)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_burst_lane_{lane:02}_membrane_disc_clearance"),
                BURST_MEMBRANE_D / 2.0,
                BURST_Z + 2.0,
                32,
            )
            .translate(x, -42.0, 0.0);

        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_burst_lane_{lane:02}_window_frame_top"),
                BURST_WINDOW_X + 12.0,
                4.0,
                7.0,
            )
            .translate(x, BURST_WINDOW_Y / 2.0 + 6.0, BURST_Z / 2.0 + 3.5)
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_burst_lane_{lane:02}_window_frame_bottom"
                ),
                BURST_WINDOW_X + 12.0,
                4.0,
                7.0,
            )
            .translate(x, -(BURST_WINDOW_Y / 2.0 + 6.0), BURST_Z / 2.0 + 3.5)
            + centered_cylinder(
                format!(
                    "closed_chip_lane_pressure_relief_burst_lane_{lane:02}_membrane_witness_ring"
                ),
                (BURST_MEMBRANE_D + 8.0) / 2.0,
                4.0,
                32,
            )
            .translate(x, -42.0, BURST_Z / 2.0 + 2.0);

        for tick in 0..BURST_WITNESS_TICKS_PER_LANE {
            features = features
                + centered_cube(
                    format!(
                        "closed_chip_lane_pressure_relief_burst_lane_{lane:02}_witness_tick_{tick:02}"
                    ),
                    3.0,
                    13.0 + tick as f64 * 4.0,
                    3.0,
                )
                .translate(
                    x - 20.0 + tick as f64 * 13.0,
                    46.0,
                    BURST_Z / 2.0 + 1.5,
                );
        }
    }

    bridge - cutouts + features + burst_overflow_gutter()
}

fn burst_overflow_gutter() -> Part {
    let front_gutter = centered_cube(
        "closed_chip_lane_pressure_relief_burst_dye_overflow_front_gutter",
        BURST_X - 40.0,
        9.0,
        10.0,
    )
    .translate(0.0, -(BURST_Y / 2.0 - 16.0), BURST_Z / 2.0 + 5.0);
    let rear_gutter = centered_cube(
        "closed_chip_lane_pressure_relief_burst_dye_overflow_rear_gutter",
        BURST_X - 40.0,
        9.0,
        10.0,
    )
    .translate(0.0, BURST_Y / 2.0 - 16.0, BURST_Z / 2.0 + 5.0);
    front_gutter + rear_gutter
}

fn calibrated_pressure_transducer_pockets() -> Part {
    let panel = centered_cube(
        "closed_chip_lane_pressure_relief_transducer_pocket_panel",
        TRANSDUCER_X,
        TRANSDUCER_Y,
        TRANSDUCER_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_transducer_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_transducer_features");

    for idx in 0..TRANSDUCER_COUNT {
        let row = idx / TRANSDUCER_COLS;
        let col = idx % TRANSDUCER_COLS;
        let x = centered_index(col, TRANSDUCER_COLS, TRANSDUCER_PITCH_X);
        let y = centered_index(row, TRANSDUCER_ROWS, TRANSDUCER_PITCH_Y);

        cutouts = cutouts
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_transducer_{idx:02}_calibrated_body_pocket"
                ),
                TRANSDUCER_POCKET_X,
                TRANSDUCER_POCKET_Y,
                16.0,
            )
            .translate(x, y, TRANSDUCER_Z / 2.0 - 8.0 + 0.2)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_transducer_{idx:02}_pressure_port_bore"),
                TRANSDUCER_PORT_D / 2.0,
                TRANSDUCER_Z + 2.0,
                26,
            )
            .translate(x - 18.0, y, 0.0)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_transducer_{idx:02}_cable_exit_slot"),
                36.0,
                6.0,
                10.0,
            )
            .translate(x + 22.0, y, TRANSDUCER_Z / 2.0 - 5.0 + 0.2);

        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_transducer_{idx:02}_keyed_pocket_ledge"),
                TRANSDUCER_POCKET_X + 10.0,
                4.0,
                6.0,
            )
            .translate(
                x,
                y - TRANSDUCER_POCKET_Y / 2.0 - 5.0,
                TRANSDUCER_Z / 2.0 + 3.0,
            )
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_transducer_{idx:02}_zero_span_tick"),
                10.0,
                8.0,
                4.0,
            )
            .translate(x + 14.0, y + 18.0, TRANSDUCER_Z / 2.0 + 2.0);
    }

    panel - cutouts + features + calibration_token_strip()
}

fn calibration_token_strip() -> Part {
    let mut tokens = Part::empty("closed_chip_lane_pressure_relief_calibration_token_strip");
    for i in 0..CALIBRATION_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_calibration_token_boss_{i:02}"),
                10.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, CALIBRATION_TOKEN_COUNT, 42.0),
                TRANSDUCER_Y / 2.0 - 20.0,
                TRANSDUCER_Z / 2.0 + 2.0,
            );
    }
    tokens
}

fn isolate_vent_routing_manifold() -> Part {
    let plate = centered_cube(
        "closed_chip_lane_pressure_relief_isolate_vent_routing_plate",
        ROUTING_X,
        ROUTING_Y,
        ROUTING_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_isolate_vent_route_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_isolate_vent_route_features");

    let isolate_header = centered_cube(
        "closed_chip_lane_pressure_relief_isolate_header_channel",
        ROUTING_X - 52.0,
        ROUTE_HEADER_W,
        8.0,
    )
    .translate(0.0, 34.0, ROUTING_Z / 2.0 - 4.0 + 0.2);
    let vent_header = centered_cube(
        "closed_chip_lane_pressure_relief_vent_header_channel",
        ROUTING_X - 52.0,
        ROUTE_HEADER_W,
        8.0,
    )
    .translate(0.0, -34.0, ROUTING_Z / 2.0 - 4.0 + 0.2);
    cutouts = cutouts + isolate_header + vent_header;

    for lane in 0..LANE_COUNT {
        let x = centered_index(lane, LANE_COUNT, 36.0);
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_isolate_drop_channel"),
                ROUTE_CHANNEL_W,
                48.0,
                8.0,
            )
            .translate(x, 10.0, ROUTING_Z / 2.0 - 4.0 + 0.2)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_vent_drop_channel"),
                ROUTE_CHANNEL_W,
                48.0,
                8.0,
            )
            .translate(x, -10.0, ROUTING_Z / 2.0 - 4.0 + 0.2);

        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_isolate_toggle_guard"),
                20.0,
                10.0,
                14.0,
            )
            .translate(x, 54.0, ROUTING_Z / 2.0 + 7.0)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_vent_toggle_guard"),
                20.0,
                10.0,
                14.0,
            )
            .translate(x, -54.0, ROUTING_Z / 2.0 + 7.0);
    }

    for i in 0..VENT_FILTER_COUNT {
        features = features
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_vent_filter_placeholder_{i:02}"),
                13.0,
                18.0,
                32,
            )
            .translate(
                centered_index(i, VENT_FILTER_COUNT, 54.0),
                -4.0,
                ROUTING_Z / 2.0 + 9.0,
            );
    }

    plate - cutouts + features + isolate_vent_bulkhead_barrier()
}

fn isolate_vent_bulkhead_barrier() -> Part {
    centered_cube(
        "closed_chip_lane_pressure_relief_isolate_vent_bulkhead_cross_contamination_barrier",
        ROUTING_X - 32.0,
        4.0,
        18.0,
    )
    .translate(0.0, 0.0, ROUTING_Z / 2.0 + 9.0)
}

fn dye_recovery_well_rack() -> Part {
    let rack = centered_cube(
        "closed_chip_lane_pressure_relief_dye_recovery_well_rack",
        DYE_X,
        DYE_Y,
        DYE_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_dye_recovery_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_dye_recovery_features");

    for lane in 0..DYE_RECOVERY_WELLS {
        let row = lane / 4;
        let col = lane % 4;
        let x = centered_index(col, 4, 62.0);
        let y = centered_index(row, 2, 58.0) + 18.0;
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_dye_lane_{lane:02}_recovery_well"),
                DYE_WELL_D / 2.0,
                DYE_Z + 2.0,
                36,
            )
            .translate(x, y, 0.0);
        features = features
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_dye_lane_{lane:02}_well_rim"),
                (DYE_WELL_D + 8.0) / 2.0,
                4.0,
                36,
            )
            .translate(x, y, DYE_Z / 2.0 + 2.0)
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_dye_lane_{lane:02}_closed_line_landing"),
                24.0,
                7.0,
                6.0,
            )
            .translate(x, y - 28.0, DYE_Z / 2.0 + 3.0);
    }

    for i in 0..DYE_OVERFLOW_WELLS {
        let x = centered_index(i, DYE_OVERFLOW_WELLS, 70.0);
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_dye_overflow_well_{i:02}"),
                18.0,
                DYE_Z + 2.0,
                36,
            )
            .translate(x, -(DYE_Y / 2.0 - 30.0), 0.0);
        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_dye_overflow_well_{i:02}_splash_guard"),
                54.0,
                4.0,
                12.0,
            )
            .translate(x, -(DYE_Y / 2.0 - 54.0), DYE_Z / 2.0 + 6.0);
    }

    rack - cutouts + features + dye_rack_drain_gutter()
}

fn dye_rack_drain_gutter() -> Part {
    centered_cube(
        "closed_chip_lane_pressure_relief_dye_recovery_drain_gutter",
        DYE_X - 34.0,
        8.0,
        9.0,
    )
    .translate(0.0, -(DYE_Y / 2.0 - 14.0), DYE_Z / 2.0 + 4.5)
}

fn release_hold_reject_lanes() -> Part {
    let block = centered_cube(
        "closed_chip_lane_pressure_relief_release_hold_reject_lane_block",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_disposition_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_disposition_features");

    for lane in 0..STATUS_LANES {
        let y = centered_index(lane, STATUS_LANES, 44.0);
        cutouts = cutouts
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_{}_lane_trough",
                    STATUS_LANE_NAMES[lane]
                ),
                STATUS_X - 34.0,
                STATUS_SLOT_Y + 8.0,
                12.0,
            )
            .translate(0.0, y, STATUS_Z / 2.0 - 6.0 + 0.2);

        features = features
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_{}_lane_front_lip",
                    STATUS_LANE_NAMES[lane]
                ),
                STATUS_X - 28.0,
                4.0,
                8.0,
            )
            .translate(0.0, y - 18.0, STATUS_Z / 2.0 + 4.0)
            + centered_cube(
                format!(
                    "closed_chip_lane_pressure_relief_{}_lane_rear_lip",
                    STATUS_LANE_NAMES[lane]
                ),
                STATUS_X - 28.0,
                4.0,
                8.0,
            )
            .translate(0.0, y + 18.0, STATUS_Z / 2.0 + 4.0);

        for slot in 0..STATUS_SLOTS_PER_LANE {
            features = features
                + centered_cube(
                    format!(
                        "closed_chip_lane_pressure_relief_{}_lane_slot_{slot:02}",
                        STATUS_LANE_NAMES[lane]
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    4.0,
                )
                .translate(
                    centered_index(slot, STATUS_SLOTS_PER_LANE, 50.0),
                    y,
                    STATUS_Z / 2.0 + 2.0,
                );
        }
    }

    block - cutouts + features + disposition_lane_gate_posts()
}

fn disposition_lane_gate_posts() -> Part {
    let mut posts = Part::empty("closed_chip_lane_pressure_relief_disposition_gate_posts");
    for i in 0..4 {
        let x = centered_index(i, 4, 58.0);
        posts = posts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_disposition_gate_post_{i:02}"),
                5.0,
                18.0,
                24,
            )
            .translate(x, STATUS_Y / 2.0 - 16.0, STATUS_Z / 2.0 + 9.0);
    }
    posts
}

fn barcode_custody_panel() -> Part {
    let panel = centered_cube(
        "closed_chip_lane_pressure_relief_barcode_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_barcode_custody_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_barcode_custody_features");

    for lane in 0..BARCODE_LANDS {
        let row = lane / 2;
        let col = lane % 2;
        let x = centered_index(col, 2, 82.0);
        let y = centered_index(row, 4, 34.0) + 14.0;
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_barcode_recess"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, CUSTODY_Z / 2.0 - 1.5 + 0.2);
        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_lane_{lane:02}_barcode_scan_guard"),
                BARCODE_LAND_X + 8.0,
                3.0,
                5.0,
            )
            .translate(x, y + BARCODE_LAND_Y / 2.0 + 4.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for i in 0..CUSTODY_TOKEN_SLOTS {
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_custody_token_slot_{i:02}"),
                18.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, CUSTODY_TOKEN_SLOTS, 28.0),
                -64.0,
                CUSTODY_Z / 2.0 - 2.0,
            );
    }

    for i in 0..TAMPER_SEAL_POSTS {
        features = features
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_tamper_seal_post_{i:02}"),
                4.5,
                12.0,
                24,
            )
            .translate(
                centered_index(i, TAMPER_SEAL_POSTS, 48.0),
                CUSTODY_Y / 2.0 - 18.0,
                CUSTODY_Z / 2.0 + 6.0,
            );
    }

    panel - cutouts + features
}

fn clean_used_segregation_cups() -> Part {
    let rack = centered_cube(
        "closed_chip_lane_pressure_relief_clean_used_segregation_rack",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_clean_used_cup_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_clean_used_cup_features");

    for i in 0..CLEAN_CAP_CUPS {
        let x = centered_index(i, CLEAN_CAP_CUPS, 42.0);
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_clean_cap_cup_{i:02}"),
                12.0,
                SEGREGATION_Z + 2.0,
                28,
            )
            .translate(x, 18.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_clean_cap_lane_{i:02}_guard"),
                30.0,
                4.0,
                8.0,
            )
            .translate(x, 39.0, SEGREGATION_Z / 2.0 + 4.0);
    }

    for i in 0..USED_BURST_DISC_CUPS {
        let x = centered_index(i, USED_BURST_DISC_CUPS, 42.0);
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_used_burst_disc_cup_{i:02}"),
                12.0,
                SEGREGATION_Z + 2.0,
                28,
            )
            .translate(x, -18.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_used_disc_lane_{i:02}_guard"),
                30.0,
                4.0,
                8.0,
            )
            .translate(x, -39.0, SEGREGATION_Z / 2.0 + 4.0);
    }

    rack - cutouts + features + clean_used_center_barrier()
}

fn clean_used_center_barrier() -> Part {
    centered_cube(
        "closed_chip_lane_pressure_relief_clean_used_center_barrier",
        SEGREGATION_X - 34.0,
        5.0,
        18.0,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 + 9.0)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_chip_lane_pressure_relief_evidence_camera_bridge_left_post",
        22.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(-CAMERA_X / 2.0 + 32.0, 0.0, CAMERA_Z / 2.0);
    let right_post = centered_cube(
        "closed_chip_lane_pressure_relief_evidence_camera_bridge_right_post",
        22.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(CAMERA_X / 2.0 - 32.0, 0.0, CAMERA_Z / 2.0);
    let beam = centered_cube(
        "closed_chip_lane_pressure_relief_evidence_camera_bridge_beam",
        CAMERA_X,
        CAMERA_Y,
        22.0,
    )
    .translate(0.0, 0.0, CAMERA_Z - 11.0);
    let mut cutouts = Part::empty("closed_chip_lane_pressure_relief_camera_bridge_view_cutouts");
    let mut features = Part::empty("closed_chip_lane_pressure_relief_camera_bridge_features");

    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, CAMERA_PITCH_X);
        cutouts = cutouts
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_camera_{i:02}_view_window"),
                CAMERA_VIEW_WINDOW_X,
                CAMERA_VIEW_WINDOW_Y,
                24.0,
            )
            .translate(x, 0.0, CAMERA_Z - 11.0);
        features = features
            + centered_cube(
                format!("closed_chip_lane_pressure_relief_camera_{i:02}_mount_plate"),
                74.0,
                8.0,
                12.0,
            )
            .translate(x, -CAMERA_Y / 2.0 - 4.0, CAMERA_Z - 11.0)
            + centered_cylinder(
                format!("closed_chip_lane_pressure_relief_camera_{i:02}_lens_axis_boss"),
                8.0,
                5.0,
                28,
            )
            .translate(x, -CAMERA_Y / 2.0 - 10.0, CAMERA_Z - 11.0);
    }

    left_post + right_post + beam - cutouts + features
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot_sweep = centered_cube(
        "closed_chip_lane_pressure_relief_front_robot_sweep_keepout_gauge",
        STATION_X - 180.0,
        6.0,
        120.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 96.0, BASE_Z / 2.0 + 60.0);
    let rear_tubing_service = centered_cube(
        "closed_chip_lane_pressure_relief_rear_tubing_service_keepout_gauge",
        STATION_X - 220.0,
        6.0,
        REAR_TUBING_SERVICE_CLEARANCE,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 108.0,
        BASE_Z / 2.0 + REAR_TUBING_SERVICE_CLEARANCE / 2.0,
    );
    let left_dye_service = centered_cube(
        "closed_chip_lane_pressure_relief_left_dye_service_keepout_gauge",
        6.0,
        FRONT_DYE_SERVICE_CLEARANCE,
        92.0,
    )
    .translate(-STATION_X / 2.0 + 96.0, -300.0, BASE_Z / 2.0 + 46.0);
    let relief_lift = centered_cube(
        "closed_chip_lane_pressure_relief_relief_cartridge_lift_keepout_gauge",
        RELIEF_X + 44.0,
        RELIEF_Y + 36.0,
        8.0,
    )
    .translate(
        RELIEF_POS.0,
        RELIEF_POS.1,
        BASE_Z / 2.0 + RELIEF_Z + RELIEF_CARTRIDGE_LIFT_CLEARANCE,
    );
    let transducer_cable = centered_cube(
        "closed_chip_lane_pressure_relief_transducer_cable_service_keepout_gauge",
        TRANSDUCER_CABLE_SERVICE_CLEARANCE,
        6.0,
        86.0,
    )
    .translate(
        TRANSDUCER_POS.0 + TRANSDUCER_X / 2.0 + 24.0,
        TRANSDUCER_POS.1,
        BASE_Z / 2.0 + 43.0,
    );
    let camera_lift = centered_cube(
        "closed_chip_lane_pressure_relief_camera_lift_keepout_gauge",
        CAMERA_X,
        CAMERA_Y + 28.0,
        8.0,
    )
    .translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        BASE_Z / 2.0 + CAMERA_Z + CAMERA_LIFT_CLEARANCE,
    );

    front_robot_sweep
        + rear_tubing_service
        + left_dye_service
        + relief_lift
        + transducer_cable
        + camera_lift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_invariants_hold() {
        assert_layout();
    }

    #[test]
    fn exported_manifest_covers_required_features() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
        assert!(OUTPUTS
            .iter()
            .all(|path| path
                .starts_with("output/closed_chip_lane_pressure_relief_burst_guard_station_")));
        assert!(REQUIRED_FEATURES.contains(&"lane_restrictor_coupons"));
        assert!(REQUIRED_FEATURES.contains(&"relief_valve_cartridge_nests"));
        assert!(REQUIRED_FEATURES.contains(&"burst_membrane_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"calibrated_pressure_transducer_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"dye_recovery_wells"));
        assert!(REQUIRED_FEATURES.contains(&"isolate_vent_routing"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_custody"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn lane_counts_align_to_closed_pressure_relief_workflow() {
        assert_eq!(LANE_COUNT, RELIEF_CARTRIDGE_COUNT);
        assert_eq!(LANE_COUNT, TRANSDUCER_COUNT);
        assert_eq!(LANE_COUNT, BURST_WINDOW_COUNT);
        assert_eq!(LANE_COUNT, DYE_RECOVERY_WELLS);
        assert_eq!(RELIEF_RETENTION_EARS, RELIEF_CARTRIDGE_COUNT * 2);
        assert!(BURST_GUARD_TARGET_KPA > LANE_PRESSURE_LIMIT_KPA);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= LANE_COUNT);
        assert_eq!(CLEAN_CAP_CUPS + USED_BURST_DISC_CUPS, LANE_COUNT);
    }
}
