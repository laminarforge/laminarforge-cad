use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline sterile filter pre-use integrity and wetting station.
//
// Intent:
// - Wet and integrity-test inline sterile filter cartridges before they are
//   released into closed culture fluid paths.
// - Keep filter handling, pressure/hold testing, vent capture, traceability,
//   disposition lanes, spill containment, and robot/service keepouts in one
//   mechanically legible station envelope.
// - Provide packaging envelopes for purchased pumps, valves, gauges,
//   pressure transducers, barcode readers, and vent capture hardware.
//
// This is architecture CAD only. It is not a validated filter integrity test,
// sterile barrier definition, or process release specification.
//
// Exports:
//   output/closed_inline_filter_preuse_integrity_test_station_base_leak_tray.stl
//   output/closed_inline_filter_preuse_integrity_test_station_filter_cartridge_nests.stl
//   output/closed_inline_filter_preuse_integrity_test_station_wetting_loop_manifold.stl
//   output/closed_inline_filter_preuse_integrity_test_station_pressure_hold_test_ports.stl
//   output/closed_inline_filter_preuse_integrity_test_station_vent_capture_module.stl
//   output/closed_inline_filter_preuse_integrity_test_station_barcode_coa_lands.stl
//   output/closed_inline_filter_preuse_integrity_test_station_release_hold_reject_lanes.stl
//   output/closed_inline_filter_preuse_integrity_test_station_wetting_media_drainage_wells.stl
//   output/closed_inline_filter_preuse_integrity_test_station_service_bulkhead.stl
//   output/closed_inline_filter_preuse_integrity_test_station_robot_service_keepouts.stl
//   output/closed_inline_filter_preuse_integrity_test_station_assembly.stl

const OUTPUTS: [&str; 11] = [
    "output/closed_inline_filter_preuse_integrity_test_station_base_leak_tray.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_filter_cartridge_nests.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_wetting_loop_manifold.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_pressure_hold_test_ports.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_vent_capture_module.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_barcode_coa_lands.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_release_hold_reject_lanes.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_wetting_media_drainage_wells.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_service_bulkhead.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_robot_service_keepouts.stl",
    "output/closed_inline_filter_preuse_integrity_test_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "filter_cartridge_nests",
    "wetting_loop",
    "pressure_hold_test_ports",
    "vent_capture",
    "barcode_coa_lands",
    "release_hold_reject_lanes",
    "leak_tray",
    "wetting_media_drainage_wells",
    "service_bulkhead",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const SOCKET_DEPTH: f64 = 5.0;

const FILTER_ROWS: usize = 3;
const FILTER_COLS: usize = 4;
const FILTER_COUNT: usize = FILTER_ROWS * FILTER_COLS;
const FILTER_SLOT_X: f64 = 96.0;
const FILTER_SLOT_Y: f64 = 36.0;
const FILTER_SLOT_Z: f64 = 32.0;
const FILTER_PITCH_X: f64 = 122.0;
const FILTER_PITCH_Y: f64 = 58.0;
const FILTER_BODY_OD: f64 = 29.0;
const FILTER_TUBE_BORE_D: f64 = 6.4;

const NEST_X: f64 = 560.0;
const NEST_Y: f64 = 252.0;
const NEST_Z: f64 = 62.0;
const NEST_POS: (f64, f64) = (-280.0, 214.0);

const WETTING_X: f64 = 462.0;
const WETTING_Y: f64 = 236.0;
const WETTING_Z: f64 = 60.0;
const WETTING_POS: (f64, f64) = (304.0, 216.0);
const WETTING_BRANCHES: usize = FILTER_COUNT;
const LOOP_HEADER_D: f64 = 8.0;
const LOOP_BRANCH_D: f64 = 4.8;
const LOOP_FLOW_VIEWERS: usize = 4;

const PRESSURE_X: f64 = 510.0;
const PRESSURE_Y: f64 = 178.0;
const PRESSURE_Z: f64 = 78.0;
const PRESSURE_POS: (f64, f64) = (-300.0, -42.0);
const PRESSURE_TEST_PORTS: usize = FILTER_COUNT;
const HOLD_REFERENCE_VOLUMES: usize = 4;
const HOLD_MANIFOLD_D: f64 = 7.2;

const VENT_X: f64 = 366.0;
const VENT_Y: f64 = 176.0;
const VENT_Z: f64 = 74.0;
const VENT_POS: (f64, f64) = (226.0, -42.0);
const VENT_CAPTURE_PORTS: usize = FILTER_COUNT;
const VENT_TRAPS: usize = 4;

const TRACE_X: f64 = 340.0;
const TRACE_Y: f64 = 146.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-380.0, -266.0);
const BARCODE_LANDS: usize = FILTER_COUNT;
const COA_LANDS: usize = 4;

const STATUS_X: f64 = 392.0;
const STATUS_Y: f64 = 164.0;
const STATUS_Z: f64 = 46.0;
const STATUS_POS: (f64, f64) = (12.0, -270.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = FILTER_COUNT / STATUS_LANES + 1;
const DISPOSITION_SLOT_X: f64 = 42.0;
const DISPOSITION_SLOT_Y: f64 = 26.0;

const MEDIA_X: f64 = 314.0;
const MEDIA_Y: f64 = 166.0;
const MEDIA_Z: f64 = 56.0;
const MEDIA_POS: (f64, f64) = (400.0, -270.0);
const WETTING_MEDIA_WELLS: usize = 6;
const SPENT_WETTING_WELLS: usize = 6;

const BULKHEAD_X: f64 = 1030.0;
const BULKHEAD_Y: f64 = 28.0;
const BULKHEAD_Z: f64 = 116.0;
const BULKHEAD_POS: (f64, f64) = (0.0, STATION_Y / 2.0 - 50.0);
const BULKHEAD_TUBE_PORTS: usize = 14;

const LEAK_CHANNELS: usize = 6;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_D: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let nests = filter_cartridge_nests();
    export(OUTPUTS[1], &nests);

    let wetting = wetting_loop_manifold();
    export(OUTPUTS[2], &wetting);

    let pressure = pressure_hold_test_ports();
    export(OUTPUTS[3], &pressure);

    let vent = vent_capture_module();
    export(OUTPUTS[4], &vent);

    let traceability = barcode_coa_lands();
    export(OUTPUTS[5], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[6], &lanes);

    let media = wetting_media_drainage_wells();
    export(OUTPUTS[7], &media);

    let bulkhead = service_bulkhead();
    export(OUTPUTS[8], &bulkhead);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + nests.translate(NEST_POS.0, NEST_POS.1, insert_z(NEST_Z))
        + wetting.translate(WETTING_POS.0, WETTING_POS.1, insert_z(WETTING_Z))
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_Z))
        + vent.translate(VENT_POS.0, VENT_POS.1, insert_z(VENT_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + lanes.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + media.translate(MEDIA_POS.0, MEDIA_POS.1, insert_z(MEDIA_Z))
        + bulkhead.translate(BULKHEAD_POS.0, BULKHEAD_POS.1, insert_z(BULKHEAD_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + 3.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed inline filter pre-use integrity/wetting station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained leak tray");
    println!(
        "  Filter nests:           {FILTER_COUNT} inline cartridges in a {FILTER_ROWS}x{FILTER_COLS} keyed nest array"
    );
    println!(
        "  Wetting loop:           {WETTING_BRANCHES} branch returns, {LOOP_FLOW_VIEWERS} flow viewers, trapped wetting media and drain wells"
    );
    println!(
        "  Integrity testing:      {PRESSURE_TEST_PORTS} pressure/hold ports, {HOLD_REFERENCE_VOLUMES} reference hold volumes, vent capture for {VENT_CAPTURE_PORTS} cartridges"
    );
    println!(
        "  Traceability/status:    {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, release/hold/reject lanes"
    );
    println!(
        "  Containment/access:     {LEAK_CHANNELS} leak channels, {BULKHEAD_TUBE_PORTS} rear service ports, {KEEP_OUT_ZONE_COUNT} robot/service keepouts"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn filter_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, FILTER_COLS, FILTER_PITCH_X),
        centered_index(row, FILTER_ROWS, FILTER_PITCH_Y),
    )
}

fn pressure_port_center(index: usize) -> (f64, f64) {
    let row = index / FILTER_COLS;
    let col = index % FILTER_COLS;
    (
        centered_index(col, FILTER_COLS, 110.0),
        centered_index(row, FILTER_ROWS, 42.0),
    )
}

fn assert_layout() {
    assert_eq!(FILTER_COUNT, FILTER_ROWS * FILTER_COLS);
    assert_eq!(WETTING_BRANCHES, FILTER_COUNT);
    assert_eq!(PRESSURE_TEST_PORTS, FILTER_COUNT);
    assert_eq!(VENT_CAPTURE_PORTS, FILTER_COUNT);
    assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= FILTER_COUNT);

    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        ("filter_cartridge_nests", NEST_POS, NEST_X, NEST_Y),
        ("wetting_loop_manifold", WETTING_POS, WETTING_X, WETTING_Y),
        (
            "pressure_hold_test_ports",
            PRESSURE_POS,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        ("vent_capture_module", VENT_POS, VENT_X, VENT_Y),
        ("barcode_coa_lands", TRACE_POS, TRACE_X, TRACE_Y),
        ("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        ("wetting_media_drainage_wells", MEDIA_POS, MEDIA_X, MEDIA_Y),
        ("service_bulkhead", BULKHEAD_POS, BULKHEAD_X, BULKHEAD_Y),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_inline_filter_preuse_base_leak_tray_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_inline_filter_preuse_base_washdown_recess",
        STATION_X - 116.0,
        STATION_Y - 112.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let wet_zone_sump = centered_cube(
        "closed_inline_filter_preuse_wet_zone_sump_recess",
        1090.0,
        188.0,
        8.0,
    )
    .translate(0.0, 214.0, BASE_Z / 2.0 - 4.0);
    let lower_disposition_sump = centered_cube(
        "closed_inline_filter_preuse_disposition_zone_sump_recess",
        1010.0,
        116.0,
        8.0,
    )
    .translate(4.0, -270.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "closed_inline_filter_preuse_front_low_point_drain",
        DRAIN_D / 2.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 - 2.0, -1.0);
    let rear_drain = centered_cylinder(
        "closed_inline_filter_preuse_rear_wet_zone_drain",
        DRAIN_D / 2.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 130.0, STATION_Y / 2.0 - 96.0, -1.0);

    deck - washdown_recess
        - wet_zone_sump
        - lower_disposition_sump
        - front_drain
        - rear_drain
        - insert_sockets()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_channels()
        + filter_drip_shadows()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_inline_filter_preuse_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_inline_filter_preuse_{name}_registration_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_inline_filter_preuse_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_inline_filter_preuse_m6_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_inline_filter_preuse_m6_slot_relief_{i}"),
                24.0,
                MOUNT_HOLE_D + 0.4,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_inline_filter_preuse_datum_pin_holes");
    for (i, (x, y)) in [
        (-520.0, 322.0),
        (520.0, 322.0),
        (-520.0, -322.0),
        (520.0, -322.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_inline_filter_preuse_datum_pin_clearance_{i}"),
                5.0 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_inline_filter_preuse_left_containment_rim",
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
        "closed_inline_filter_preuse_right_containment_rim",
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
        "closed_inline_filter_preuse_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_lip = centered_cube(
        "closed_inline_filter_preuse_front_low_witness_lip",
        STATION_X - 170.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 25.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front_lip
}

fn zone_dividers() -> Part {
    let wet_test_divider = centered_cube(
        "closed_inline_filter_preuse_wet_to_test_divider",
        STATION_X - 156.0,
        12.0,
        30.0,
    )
    .translate(0.0, 82.0, BASE_Z / 2.0 + 15.0);
    let test_status_divider = centered_cube(
        "closed_inline_filter_preuse_test_to_disposition_divider",
        STATION_X - 166.0,
        10.0,
        28.0,
    )
    .translate(0.0, -164.0, BASE_Z / 2.0 + 14.0);
    let nest_loop_divider = centered_cube(
        "closed_inline_filter_preuse_nest_to_wetting_loop_divider",
        10.0,
        248.0,
        28.0,
    )
    .translate(-20.0, 214.0, BASE_Z / 2.0 + 14.0);
    let trace_release_divider = centered_cube(
        "closed_inline_filter_preuse_trace_to_status_divider",
        10.0,
        154.0,
        28.0,
    )
    .translate(-188.0, -270.0, BASE_Z / 2.0 + 14.0);
    let reject_media_divider = centered_cube(
        "closed_inline_filter_preuse_status_to_media_drain_divider",
        10.0,
        158.0,
        28.0,
    )
    .translate(226.0, -270.0, BASE_Z / 2.0 + 14.0);

    wet_test_divider
        + test_status_divider
        + nest_loop_divider
        + trace_release_divider
        + reject_media_divider
}

fn leak_witness_channels() -> Part {
    let mut channels = Part::empty("closed_inline_filter_preuse_leak_witness_channels");
    for i in 0..LEAK_CHANNELS {
        let x = centered_index(i, LEAK_CHANNELS, 178.0);
        channels = channels
            + centered_cube(
                format!("closed_inline_filter_preuse_leak_witness_channel_rib_{i}"),
                130.0,
                5.0,
                7.0,
            )
            .translate(x, -342.0, BASE_Z / 2.0 + 3.5);
    }
    channels
}

fn filter_drip_shadows() -> Part {
    let mut shadows = Part::empty("closed_inline_filter_preuse_filter_drip_shadow_ribs");
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let index = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            shadows = shadows
                + centered_cube(
                    format!("closed_inline_filter_preuse_drip_shadow_{index}"),
                    FILTER_SLOT_X + 10.0,
                    FILTER_SLOT_Y + 10.0,
                    5.0,
                )
                .translate(NEST_POS.0 + x, NEST_POS.1 + y, BASE_Z / 2.0 + 2.5);
        }
    }
    shadows
}

fn robot_fiducials() -> Part {
    let mut targets = Part::empty("closed_inline_filter_preuse_robot_fiducials");
    for (i, (x, y)) in [(-514.0, 318.0), (514.0, 318.0), (-514.0, -318.0)]
        .iter()
        .enumerate()
    {
        targets =
            targets
                + fiducial_disc(&format!("closed_inline_filter_preuse_robot_fiducial_{i}"))
                    .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    targets
}

fn filter_cartridge_nests() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_filter_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_inline_filter_preuse_filter_nest_rear_fence",
        NEST_X,
        14.0,
        NEST_Z + 34.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 7.0, 17.0);
    let left_fence = centered_cube(
        "closed_inline_filter_preuse_filter_nest_left_fence",
        14.0,
        NEST_Y - 20.0,
        NEST_Z + 20.0,
    )
    .translate(-(NEST_X / 2.0 - 7.0), 0.0, 10.0);

    let mut nest_cuts = Part::empty("closed_inline_filter_preuse_filter_nest_cuts");
    let mut clamp_lands = Part::empty("closed_inline_filter_preuse_filter_clamp_lands");
    let mut key_lands = Part::empty("closed_inline_filter_preuse_filter_key_lands");
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let index = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            nest_cuts = nest_cuts
                + filter_capsule_pocket(index, x, y)
                + filter_tube_socket_pair(index, x, y)
                + centered_cube(
                    format!("closed_inline_filter_preuse_filter_{index}_finger_relief"),
                    20.0,
                    FILTER_SLOT_Y + 12.0,
                    24.0,
                )
                .translate(x, y - 4.0, NEST_Z / 2.0 - 8.0);

            clamp_lands = clamp_lands
                + centered_cube(
                    format!("closed_inline_filter_preuse_filter_{index}_left_clamp_land"),
                    16.0,
                    FILTER_SLOT_Y + 14.0,
                    10.0,
                )
                .translate(x - FILTER_SLOT_X / 2.0 - 12.0, y, NEST_Z / 2.0 + 5.0)
                + centered_cube(
                    format!("closed_inline_filter_preuse_filter_{index}_right_clamp_land"),
                    16.0,
                    FILTER_SLOT_Y + 14.0,
                    10.0,
                )
                .translate(x + FILTER_SLOT_X / 2.0 + 12.0, y, NEST_Z / 2.0 + 5.0);

            key_lands = key_lands
                + centered_cube(
                    format!("closed_inline_filter_preuse_filter_{index}_flow_arrow_key_land"),
                    28.0,
                    6.0,
                    5.0,
                )
                .translate(x, y - FILTER_SLOT_Y / 2.0 - 15.0, NEST_Z / 2.0 + 2.5);
        }
    }

    body + rear_fence + left_fence + clamp_lands + key_lands - nest_cuts + nest_gripper_fiducials()
}

fn filter_capsule_pocket(index: usize, x: f64, y: f64) -> Part {
    let rectangular_clearance = centered_cube(
        format!("closed_inline_filter_preuse_filter_{index}_rectangular_clearance"),
        FILTER_SLOT_X,
        FILTER_SLOT_Y,
        FILTER_SLOT_Z,
    )
    .translate(x, y, NEST_Z / 2.0 - FILTER_SLOT_Z / 2.0 + 2.0);
    let cylindrical_cradle = centered_cylinder(
        format!("closed_inline_filter_preuse_filter_{index}_cylindrical_cradle"),
        FILTER_BODY_OD / 2.0,
        FILTER_SLOT_X + 8.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, y, NEST_Z / 2.0 - 12.0);
    rectangular_clearance + cylindrical_cradle
}

fn filter_tube_socket_pair(index: usize, x: f64, y: f64) -> Part {
    let inlet = centered_cylinder(
        format!("closed_inline_filter_preuse_filter_{index}_inlet_luer_socket"),
        FILTER_TUBE_BORE_D / 2.0,
        54.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x - FILTER_SLOT_X / 2.0 - 4.0, y, NEST_Z / 2.0 - 11.0);
    let outlet = centered_cylinder(
        format!("closed_inline_filter_preuse_filter_{index}_outlet_luer_socket"),
        FILTER_TUBE_BORE_D / 2.0,
        54.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x + FILTER_SLOT_X / 2.0 + 4.0, y, NEST_Z / 2.0 - 11.0);
    inlet + outlet
}

fn nest_gripper_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_inline_filter_preuse_nest_gripper_fiducials");
    for (i, x) in [-228.0, 228.0].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_inline_filter_preuse_nest_gripper_fiducial_{i}"
            ))
            .translate(*x, -(NEST_Y / 2.0 - 28.0), NEST_Z / 2.0 + 3.0);
    }
    fiducials
}

fn wetting_loop_manifold() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_wetting_loop_manifold_body",
        WETTING_X,
        WETTING_Y,
        WETTING_Z,
    );
    let recirculation_pump = centered_cube(
        "closed_inline_filter_preuse_wetting_loop_pump_envelope",
        96.0,
        66.0,
        58.0,
    )
    .translate(-(WETTING_X / 2.0 - 72.0), 0.0, WETTING_Z / 2.0 + 29.0);
    let bubble_trap = centered_cylinder(
        "closed_inline_filter_preuse_wetting_loop_bubble_trap_body",
        26.0,
        86.0,
        48,
    )
    .translate(WETTING_X / 2.0 - 68.0, 0.0, WETTING_Z / 2.0 + 38.0);
    let bypass_valve_block = centered_cube(
        "closed_inline_filter_preuse_wetting_loop_sanitize_bypass_valve_block",
        82.0,
        44.0,
        38.0,
    )
    .translate(0.0, WETTING_Y / 2.0 - 38.0, WETTING_Z / 2.0 + 19.0);

    body + recirculation_pump + bubble_trap + bypass_valve_block + flow_viewers()
        - wetting_loop_bores()
        - wetting_branch_ports()
        - bubble_trap_drain()
}

fn wetting_loop_bores() -> Part {
    let supply = centered_cylinder(
        "closed_inline_filter_preuse_wetting_supply_header_bore",
        LOOP_HEADER_D / 2.0,
        WETTING_X - 64.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(18.0, 50.0, WETTING_Z / 2.0 - 8.0);
    let return_header = centered_cylinder(
        "closed_inline_filter_preuse_wetting_return_header_bore",
        LOOP_HEADER_D / 2.0,
        WETTING_X - 64.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(18.0, -50.0, WETTING_Z / 2.0 - 8.0);
    let recirculation_cross = centered_cylinder(
        "closed_inline_filter_preuse_wetting_recirculation_cross_bore",
        LOOP_HEADER_D / 2.0,
        WETTING_Y - 46.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(WETTING_X / 2.0 - 72.0), 0.0, WETTING_Z / 2.0 - 8.0);
    supply + return_header + recirculation_cross
}

fn wetting_branch_ports() -> Part {
    let mut ports = Part::empty("closed_inline_filter_preuse_wetting_branch_ports");
    for index in 0..WETTING_BRANCHES {
        let row = index / FILTER_COLS;
        let col = index % FILTER_COLS;
        let x = centered_index(col, FILTER_COLS, 88.0) + 18.0;
        let y = centered_index(row, FILTER_ROWS, 34.0);
        ports = ports
            + centered_cylinder(
                format!("closed_inline_filter_preuse_wetting_branch_{index}_supply_port"),
                LOOP_BRANCH_D / 2.0,
                38.0,
                22,
            )
            .translate(x, y + 56.0, WETTING_Z / 2.0 - 8.0)
            + centered_cylinder(
                format!("closed_inline_filter_preuse_wetting_branch_{index}_return_port"),
                LOOP_BRANCH_D / 2.0,
                38.0,
                22,
            )
            .translate(x, y - 56.0, WETTING_Z / 2.0 - 8.0);
    }
    ports
}

fn bubble_trap_drain() -> Part {
    centered_cylinder(
        "closed_inline_filter_preuse_wetting_bubble_trap_low_drain",
        5.0 / 2.0,
        66.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WETTING_X / 2.0 - 68.0,
        -WETTING_Y / 2.0 + 18.0,
        WETTING_Z / 2.0,
    )
}

fn flow_viewers() -> Part {
    let mut viewers = Part::empty("closed_inline_filter_preuse_wetting_loop_flow_viewers");
    for i in 0..LOOP_FLOW_VIEWERS {
        let x = centered_index(i, LOOP_FLOW_VIEWERS, 82.0) + 18.0;
        viewers = viewers
            + centered_cube(
                format!("closed_inline_filter_preuse_flow_viewer_window_boss_{i}"),
                54.0,
                18.0,
                10.0,
            )
            .translate(x, 0.0, WETTING_Z / 2.0 + 5.0);
    }
    viewers
}

fn pressure_hold_test_ports() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_pressure_hold_port_bank_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let rear_connector_backer = centered_cube(
        "closed_inline_filter_preuse_pressure_hold_rear_connector_backer",
        PRESSURE_X - 38.0,
        14.0,
        PRESSURE_Z + 42.0,
    )
    .translate(0.0, PRESSURE_Y / 2.0 - 7.0, 21.0);
    let reference_volume_saddles = hold_reference_volume_saddles();
    let transducer_rail = pressure_transducer_rail();
    let regulator_gauge_cluster = pressure_regulator_gauge_cluster();

    body + rear_connector_backer
        + reference_volume_saddles
        + transducer_rail
        + regulator_gauge_cluster
        - pressure_hold_port_bores()
        - hold_manifold_bores()
}

fn pressure_hold_port_bores() -> Part {
    let mut bores = Part::empty("closed_inline_filter_preuse_pressure_hold_port_bores");
    for index in 0..PRESSURE_TEST_PORTS {
        let (x, y) = pressure_port_center(index);
        bores = bores
            + centered_cylinder(
                format!("closed_inline_filter_preuse_pressure_hold_port_{index}"),
                FILTER_TUBE_BORE_D / 2.0,
                PRESSURE_Z + 8.0,
                24,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("closed_inline_filter_preuse_pressure_hold_port_label_recess_{index}"),
                34.0,
                7.0,
                5.0,
            )
            .translate(x, y - 17.0, PRESSURE_Z / 2.0 - 2.5);
    }
    bores
}

fn hold_manifold_bores() -> Part {
    let common_pressure = centered_cylinder(
        "closed_inline_filter_preuse_common_pressure_hold_header",
        HOLD_MANIFOLD_D / 2.0,
        PRESSURE_X - 78.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, PRESSURE_Y / 2.0 - 34.0, PRESSURE_Z / 2.0 - 14.0);
    let vent_equalization = centered_cylinder(
        "closed_inline_filter_preuse_hold_equalization_header",
        HOLD_MANIFOLD_D / 2.0,
        PRESSURE_X - 78.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(PRESSURE_Y / 2.0 - 34.0), PRESSURE_Z / 2.0 - 14.0);
    common_pressure + vent_equalization
}

fn hold_reference_volume_saddles() -> Part {
    let mut saddles = Part::empty("closed_inline_filter_preuse_hold_reference_volume_saddles");
    for i in 0..HOLD_REFERENCE_VOLUMES {
        let x = centered_index(i, HOLD_REFERENCE_VOLUMES, 96.0);
        let cylinder = centered_cylinder(
            format!("closed_inline_filter_preuse_hold_reference_volume_{i}"),
            18.0,
            70.0,
            36,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, PRESSURE_Y / 2.0 - 44.0, PRESSURE_Z / 2.0 + 30.0);
        let saddle = centered_cube(
            format!("closed_inline_filter_preuse_hold_reference_volume_saddle_{i}"),
            76.0,
            20.0,
            18.0,
        )
        .translate(x, PRESSURE_Y / 2.0 - 44.0, PRESSURE_Z / 2.0 + 9.0);
        saddles = saddles + cylinder + saddle;
    }
    saddles
}

fn pressure_transducer_rail() -> Part {
    let rail = centered_cube(
        "closed_inline_filter_preuse_pressure_transducer_rail",
        PRESSURE_X - 76.0,
        22.0,
        20.0,
    )
    .translate(0.0, -(PRESSURE_Y / 2.0 - 32.0), PRESSURE_Z / 2.0 + 10.0);
    let mut pockets = Part::empty("closed_inline_filter_preuse_pressure_transducer_pockets");
    for i in 0..6 {
        pockets = pockets
            + centered_cube(
                format!("closed_inline_filter_preuse_pressure_transducer_pocket_{i}"),
                42.0,
                12.0,
                16.0,
            )
            .translate(
                centered_index(i, 6, 66.0),
                -(PRESSURE_Y / 2.0 - 32.0),
                PRESSURE_Z / 2.0 + 10.0,
            );
    }
    rail - pockets
}

fn pressure_regulator_gauge_cluster() -> Part {
    let mut cluster = Part::empty("closed_inline_filter_preuse_regulator_gauge_cluster");
    for (i, x) in [-200.0, -120.0, -40.0, 40.0, 120.0, 200.0]
        .iter()
        .enumerate()
    {
        let regulator = centered_cube(
            format!("closed_inline_filter_preuse_regulator_body_{i}"),
            42.0,
            28.0,
            34.0,
        )
        .translate(*x, 0.0, PRESSURE_Z / 2.0 + 17.0);
        let gauge = centered_cylinder(
            format!("closed_inline_filter_preuse_gauge_face_{i}"),
            14.0,
            7.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -20.0, PRESSURE_Z / 2.0 + 46.0);
        cluster = cluster + regulator + gauge;
    }
    cluster
}

fn vent_capture_module() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_vent_capture_body",
        VENT_X,
        VENT_Y,
        VENT_Z,
    );
    let hood = centered_cube(
        "closed_inline_filter_preuse_vent_capture_clear_hood_envelope",
        VENT_X - 40.0,
        VENT_Y - 36.0,
        42.0,
    )
    .translate(0.0, 0.0, VENT_Z / 2.0 + 21.0);
    let demister_shelf = centered_cube(
        "closed_inline_filter_preuse_vent_capture_demister_shelf",
        VENT_X - 58.0,
        16.0,
        16.0,
    )
    .translate(0.0, VENT_Y / 2.0 - 38.0, VENT_Z / 2.0 + 8.0);

    body + hood + demister_shelf + vent_trap_canisters() - vent_capture_bores() - vent_trap_wells()
}

fn vent_capture_bores() -> Part {
    let mut bores = Part::empty("closed_inline_filter_preuse_vent_capture_port_bores");
    for index in 0..VENT_CAPTURE_PORTS {
        let row = index / FILTER_COLS;
        let col = index % FILTER_COLS;
        let x = centered_index(col, FILTER_COLS, 82.0);
        let y = centered_index(row, FILTER_ROWS, 34.0);
        bores = bores
            + centered_cylinder(
                format!("closed_inline_filter_preuse_filter_{index}_vent_capture_port"),
                4.8 / 2.0,
                VENT_Z + 8.0,
                22,
            )
            .translate(x, y, 0.0);
    }
    bores
}

fn vent_trap_wells() -> Part {
    let mut wells = Part::empty("closed_inline_filter_preuse_vent_capture_condensate_wells");
    for i in 0..VENT_TRAPS {
        wells = wells
            + centered_cylinder(
                format!("closed_inline_filter_preuse_vent_capture_condensate_well_{i}"),
                16.0,
                VENT_Z + 6.0,
                36,
            )
            .translate(
                centered_index(i, VENT_TRAPS, 76.0),
                VENT_Y / 2.0 - 48.0,
                0.0,
            );
    }
    wells
}

fn vent_trap_canisters() -> Part {
    let mut canisters = Part::empty("closed_inline_filter_preuse_vent_hydrophobic_trap_canisters");
    for i in 0..VENT_TRAPS {
        let x = centered_index(i, VENT_TRAPS, 76.0);
        let body = centered_cylinder(
            format!("closed_inline_filter_preuse_hydrophobic_vent_filter_canister_{i}"),
            15.0,
            54.0,
            36,
        )
        .translate(x, -(VENT_Y / 2.0 - 36.0), VENT_Z / 2.0 + 28.0);
        let cap = centered_cylinder(
            format!("closed_inline_filter_preuse_hydrophobic_vent_filter_cap_{i}"),
            9.0,
            12.0,
            30,
        )
        .translate(x, -(VENT_Y / 2.0 - 36.0), VENT_Z / 2.0 + 61.0);
        canisters = canisters + body + cap;
    }
    canisters
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_inline_filter_preuse_barcode_coa_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut barcode_lands = Part::empty("closed_inline_filter_preuse_filter_barcode_lands");
    for index in 0..BARCODE_LANDS {
        let col = index % 4;
        let row = index / 4;
        barcode_lands = barcode_lands
            + centered_cube(
                format!("closed_inline_filter_preuse_filter_barcode_land_{index}"),
                62.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 74.0),
                44.0 - row as f64 * 31.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }

    let mut coa_lands = Part::empty("closed_inline_filter_preuse_certificate_of_analysis_lands");
    for i in 0..COA_LANDS {
        coa_lands = coa_lands
            + centered_cube(
                format!("closed_inline_filter_preuse_coa_land_{i}"),
                92.0,
                21.0,
                5.0,
            )
            .translate(
                centered_index(i, COA_LANDS, 82.0),
                -(TRACE_Y / 2.0 - 22.0),
                TRACE_Z / 2.0 + 2.5,
            );
    }
    let reader_datum = centered_cube(
        "closed_inline_filter_preuse_barcode_reader_edge_datum",
        TRACE_X - 42.0,
        8.0,
        9.0,
    )
    .translate(0.0, TRACE_Y / 2.0 - 12.0, TRACE_Z / 2.0 + 4.5);
    let coa_clip_rail = centered_cube(
        "closed_inline_filter_preuse_coa_clip_rail",
        TRACE_X - 56.0,
        8.0,
        12.0,
    )
    .translate(0.0, -(TRACE_Y / 2.0 - 48.0), TRACE_Z / 2.0 + 6.0);

    panel + barcode_lands + coa_lands + reader_datum + coa_clip_rail
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_release_hold_reject_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut lane_cuts = Part::empty("closed_inline_filter_preuse_disposition_lane_slots");
    let mut lane_labels = Part::empty("closed_inline_filter_preuse_disposition_lane_labels");
    for lane in 0..STATUS_LANES {
        let y = centered_index(lane, STATUS_LANES, 46.0);
        lane_labels = lane_labels
            + centered_cube(
                format!("closed_inline_filter_preuse_disposition_lane_{lane}_label_land"),
                74.0,
                8.0,
                5.0,
            )
            .translate(-(STATUS_X / 2.0 - 48.0), y, STATUS_Z / 2.0 + 2.5);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 54.0) + 56.0;
            lane_cuts = lane_cuts
                + centered_cube(
                    format!("closed_inline_filter_preuse_disposition_lane_{lane}_slot_{slot}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    26.0,
                )
                .translate(x, y, STATUS_Z / 2.0 - 11.0);
        }
    }

    let release_datum_rail = centered_cube(
        "closed_inline_filter_preuse_release_lane_green_datum_rail",
        STATUS_X - 76.0,
        7.0,
        8.0,
    )
    .translate(
        38.0,
        centered_index(2, STATUS_LANES, 46.0) + 20.0,
        STATUS_Z / 2.0 + 4.0,
    );
    let hold_quarantine_gate = centered_cube(
        "closed_inline_filter_preuse_hold_lane_quarantine_gate",
        STATUS_X - 96.0,
        8.0,
        22.0,
    )
    .translate(
        42.0,
        centered_index(1, STATUS_LANES, 46.0) + 21.0,
        STATUS_Z / 2.0 + 11.0,
    );
    let reject_cover_land = centered_cube(
        "closed_inline_filter_preuse_reject_lane_cover_land",
        STATUS_X - 96.0,
        10.0,
        18.0,
    )
    .translate(
        42.0,
        centered_index(0, STATUS_LANES, 46.0) - 22.0,
        STATUS_Z / 2.0 + 9.0,
    );

    body + lane_labels + release_datum_rail + hold_quarantine_gate + reject_cover_land - lane_cuts
}

fn wetting_media_drainage_wells() -> Part {
    let body = centered_cube(
        "closed_inline_filter_preuse_wetting_media_drainage_block",
        MEDIA_X,
        MEDIA_Y,
        MEDIA_Z,
    );
    let clean_media_label = centered_cube(
        "closed_inline_filter_preuse_clean_wetting_media_label_land",
        132.0,
        8.0,
        5.0,
    )
    .translate(-76.0, MEDIA_Y / 2.0 - 18.0, MEDIA_Z / 2.0 + 2.5);
    let spent_media_label = centered_cube(
        "closed_inline_filter_preuse_spent_wetting_media_label_land",
        132.0,
        8.0,
        5.0,
    )
    .translate(76.0, MEDIA_Y / 2.0 - 18.0, MEDIA_Z / 2.0 + 2.5);

    body + clean_media_label + spent_media_label - wetting_media_wells() - spent_wetting_wells()
        + low_point_drain_guard()
}

fn wetting_media_wells() -> Part {
    let mut wells = Part::empty("closed_inline_filter_preuse_clean_wetting_media_wells");
    for i in 0..WETTING_MEDIA_WELLS {
        let col = i % 3;
        let row = i / 3;
        wells = wells
            + centered_cylinder(
                format!("closed_inline_filter_preuse_clean_wetting_media_well_{i}"),
                16.0,
                MEDIA_Z + 4.0,
                36,
            )
            .translate(
                -76.0 + centered_index(col, 3, 42.0),
                -24.0 + row as f64 * 44.0,
                2.0,
            );
    }
    wells
}

fn spent_wetting_wells() -> Part {
    let mut wells = Part::empty("closed_inline_filter_preuse_spent_wetting_media_wells");
    for i in 0..SPENT_WETTING_WELLS {
        let col = i % 3;
        let row = i / 3;
        wells = wells
            + centered_cylinder(
                format!("closed_inline_filter_preuse_spent_wetting_well_{i}"),
                16.0,
                MEDIA_Z + 4.0,
                36,
            )
            .translate(
                76.0 + centered_index(col, 3, 42.0),
                -24.0 + row as f64 * 44.0,
                2.0,
            );
    }
    wells
}

fn low_point_drain_guard() -> Part {
    let guard = centered_cube(
        "closed_inline_filter_preuse_wetting_media_low_point_drain_guard",
        MEDIA_X - 52.0,
        18.0,
        18.0,
    )
    .translate(0.0, -(MEDIA_Y / 2.0 - 20.0), MEDIA_Z / 2.0 + 9.0);
    let drain = centered_cylinder(
        "closed_inline_filter_preuse_wetting_media_low_point_drain_port",
        8.0 / 2.0,
        MEDIA_X - 64.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(MEDIA_Y / 2.0 - 20.0), MEDIA_Z / 2.0 + 9.0);
    guard - drain
}

fn service_bulkhead() -> Part {
    let panel = centered_cube(
        "closed_inline_filter_preuse_rear_service_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let electrical_window = centered_cube(
        "closed_inline_filter_preuse_rear_service_electrical_window",
        154.0,
        BULKHEAD_Y + 4.0,
        52.0,
    )
    .translate(BULKHEAD_X / 2.0 - 126.0, 0.0, -BULKHEAD_Z / 2.0 + 40.0);
    let waste_window = centered_cube(
        "closed_inline_filter_preuse_rear_service_drain_window",
        118.0,
        BULKHEAD_Y + 4.0,
        42.0,
    )
    .translate(-(BULKHEAD_X / 2.0 - 118.0), 0.0, -BULKHEAD_Z / 2.0 + 34.0);

    panel - electrical_window - waste_window - service_tube_port_bores()
        + service_bulkhead_port_labels()
        + service_bulkhead_gussets()
}

fn service_tube_port_bores() -> Part {
    let mut ports = Part::empty("closed_inline_filter_preuse_service_tube_port_bores");
    for i in 0..BULKHEAD_TUBE_PORTS {
        ports = ports
            + centered_cylinder(
                format!("closed_inline_filter_preuse_service_tube_port_{i}"),
                7.2 / 2.0,
                BULKHEAD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, BULKHEAD_TUBE_PORTS, 64.0),
                0.0,
                centered_index(i % 2, 2, 32.0) + 18.0,
            );
    }
    ports
}

fn service_bulkhead_port_labels() -> Part {
    let mut labels = Part::empty("closed_inline_filter_preuse_service_bulkhead_port_labels");
    for i in 0..BULKHEAD_TUBE_PORTS {
        labels = labels
            + centered_cube(
                format!("closed_inline_filter_preuse_service_port_label_land_{i}"),
                42.0,
                5.0,
                4.0,
            )
            .translate(
                centered_index(i, BULKHEAD_TUBE_PORTS, 64.0),
                -(BULKHEAD_Y / 2.0 + 3.0),
                centered_index(i % 2, 2, 32.0) - 6.0,
            );
    }
    labels
}

fn service_bulkhead_gussets() -> Part {
    let mut gussets = Part::empty("closed_inline_filter_preuse_service_bulkhead_gussets");
    for (i, x) in [-450.0, -300.0, -150.0, 0.0, 150.0, 300.0, 450.0]
        .iter()
        .enumerate()
    {
        let web = centered_cube(
            format!("closed_inline_filter_preuse_service_bulkhead_gusset_web_{i}"),
            10.0,
            58.0,
            72.0,
        )
        .translate(*x, -(BULKHEAD_Y / 2.0 + 18.0), -8.0);
        let foot = centered_cube(
            format!("closed_inline_filter_preuse_service_bulkhead_gusset_foot_{i}"),
            44.0,
            48.0,
            10.0,
        )
        .translate(*x, -(BULKHEAD_Y / 2.0 + 18.0), -(BULKHEAD_Z / 2.0 - 5.0));
        gussets = gussets + web + foot;
    }
    gussets
}

fn robot_service_keepouts() -> Part {
    let mut zones = Part::empty("closed_inline_filter_preuse_robot_service_keepout_zones");
    for (i, (name, x, y, width, depth)) in [
        ("front_robot_pick_sweep", 0.0, -346.0, 1040.0, 10.0),
        ("rear_tube_service_sweep", 0.0, 342.0, 1080.0, 10.0),
        ("left_coa_scan_service_lane", -548.0, 0.0, 10.0, 620.0),
        ("right_wetting_media_change_lane", 548.0, 0.0, 10.0, 620.0),
        ("vent_hood_lift_envelope", 226.0, -42.0, VENT_X + 42.0, 10.0),
    ]
    .iter()
    .enumerate()
    {
        zones = zones
            + centered_cube(
                format!("closed_inline_filter_preuse_keepout_{i}_{name}"),
                *width,
                *depth,
                6.0,
            )
            .translate(*x, *y, 0.0);
    }

    let centerline = centered_cube(
        "closed_inline_filter_preuse_keepout_centerline_x",
        STATION_X - 140.0,
        4.0,
        6.0,
    ) + centered_cube(
        "closed_inline_filter_preuse_keepout_centerline_y",
        4.0,
        STATION_Y - 140.0,
        6.0,
    );
    zones + centerline
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center"), 1.2, 3.0, 18);
    disc - center
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_inline_filter_preuse_integrity_test_station_"),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn station_covers_required_user_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"filter_cartridge_nests"));
        assert!(REQUIRED_FEATURES.contains(&"wetting_loop"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_hold_test_ports"));
        assert!(REQUIRED_FEATURES.contains(&"vent_capture"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn filter_nest_and_status_capacity_matches_batch_size() {
        assert_eq!(FILTER_ROWS, 3);
        assert_eq!(FILTER_COLS, 4);
        assert_eq!(FILTER_COUNT, 12);
        assert_eq!(FILTER_COUNT, FILTER_ROWS * FILTER_COLS);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= FILTER_COUNT);
        assert_eq!(STATUS_LANES, 3);
    }

    #[test]
    fn wetting_pressure_and_vent_paths_are_one_to_one_with_filters() {
        assert_eq!(WETTING_BRANCHES, FILTER_COUNT);
        assert_eq!(PRESSURE_TEST_PORTS, FILTER_COUNT);
        assert_eq!(VENT_CAPTURE_PORTS, FILTER_COUNT);
        assert_eq!(VENT_TRAPS, 4);
        assert!(LOOP_HEADER_D > LOOP_BRANCH_D);
        assert!(HOLD_MANIFOLD_D > FILTER_TUBE_BORE_D);
    }

    #[test]
    fn all_modules_fit_inside_contained_station() {
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
        assert!(STATION_X <= 1200.0);
        assert!(STATION_Y <= 780.0);
        assert!(RIM_Z >= 40.0);
    }

    #[test]
    fn filter_positions_are_centered_in_nest_body() {
        let (left_x, low_y) = filter_center(0, 0);
        let (right_x, high_y) = filter_center(FILTER_ROWS - 1, FILTER_COLS - 1);
        assert!((left_x + right_x).abs() < 0.001);
        assert!((low_y + high_y).abs() < 0.001);
        assert!(right_x.abs() + FILTER_SLOT_X / 2.0 < NEST_X / 2.0 - 32.0);
        assert!(high_y.abs() + FILTER_SLOT_Y / 2.0 < NEST_Y / 2.0 - 28.0);
    }

    #[test]
    fn traceability_wetting_and_service_counts_cover_release() {
        assert_eq!(BARCODE_LANDS, FILTER_COUNT);
        assert_eq!(COA_LANDS, 4);
        assert_eq!(WETTING_MEDIA_WELLS, SPENT_WETTING_WELLS);
        assert!(WETTING_MEDIA_WELLS >= 6);
        assert!(BULKHEAD_TUBE_PORTS >= FILTER_COUNT);
    }

    #[test]
    fn containment_and_robot_access_are_explicit() {
        assert_eq!(LEAK_CHANNELS, 6);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
        assert!(DRAIN_D >= 10.0);
        assert!(MEDIA_Z >= 50.0);
    }
}
