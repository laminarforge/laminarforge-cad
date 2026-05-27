use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system perfusion bypass-valve fail-position witness station.
//
// Intent:
// - Validate normal, bypass, relief, and recovery routing before any live
//   tissue-chip cassette is connected.
// - Make fail-open, fail-closed, wrong-route, pressure-transient, bubble,
//   wetness, dye-recovery, custody, disposition, camera, and robot/service
//   keepout evidence visible as physical witness geometry.
// - Provide mechanical validation packaging only. Purchased valves, pressure
//   sensors, wetted materials, acceptance limits, and sterile procedures remain
//   external controlled items.

const PREFIX: &str = "closed_perfusion_bypass_valve_fail_position_witness_station";
const OUTPUT_PREFIX: &str = "output/closed_perfusion_bypass_valve_fail_position_witness_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_containment_leak_tray.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_valve_cartridge_nest_bank.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_normal_bypass_route_witness_lanes.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_fail_open_closed_token_rails.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_pressure_spike_sensor_pockets.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_relief_path_capture_wells.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_bubble_wetness_windows.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_dye_recovery_vial_nests.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_barcode_custody_lands.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_camera_bridge_robot_keepout_gauges.stl",
    "output/closed_perfusion_bypass_valve_fail_position_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "containment_leak_tray",
    "valve_cartridge_nest_bank",
    "normal_bypass_route_witness_lanes",
    "fail_open_closed_token_rails",
    "pressure_spike_sensor_pockets",
    "relief_path_capture_wells",
    "bubble_wetness_windows",
    "dye_recovery_vial_nests",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "camera_bridge_robot_keepout_gauges",
];

const LIMITATIONS: [&str; 5] = [
    "no_cell_validation_station_only",
    "not_pressure_rated_valve_body",
    "not_sterile_barrier_definition",
    "external_valves_sensors_and_vials",
    "acceptance_thresholds_defined_by_protocol",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_RECESS_Z: f64 = 7.0;
const DRAIN_D: f64 = 15.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_COUNT: usize = 5;

const VALVE_LANES: usize = 6;
const ROUTE_STATES: usize = 2;
const PRESSURE_POINTS_PER_LANE: usize = 3;
const PRESSURE_SENSOR_POCKETS: usize = VALVE_LANES * PRESSURE_POINTS_PER_LANE;
const RELIEF_WELLS: usize = VALVE_LANES;
const RECOVERY_VIALS: usize = VALVE_LANES;
const BUBBLE_WINDOWS: usize = VALVE_LANES;
const WETNESS_PADS: usize = VALVE_LANES * 2;
const TOKEN_RAILS: usize = 2;
const FAIL_TOKENS_PER_LANE: usize = 2;
const FAIL_STATE_TOKENS: usize = VALVE_LANES * FAIL_TOKENS_PER_LANE;
const BARCODE_LANDS: usize = VALVE_LANES;
const CUSTODY_LANDS: usize = 4;
const DISPOSITION_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const KEEP_OUT_GAUGES: usize = 6;
const CAMERA_TARGETS: usize = VALVE_LANES;

const VALVE_BANK_CENTER: (f64, f64) = (-330.0, 220.0);
const VALVE_BANK_X: f64 = 500.0;
const VALVE_BANK_Y: f64 = 210.0;
const VALVE_BANK_Z: f64 = 42.0;
const VALVE_PITCH_X: f64 = 74.0;
const VALVE_SLOT_X: f64 = 56.0;
const VALVE_SLOT_Y: f64 = 82.0;
const VALVE_SLOT_Z: f64 = 20.0;
const VALVE_BORE_D: f64 = 10.0;

const ROUTE_CENTER: (f64, f64) = (260.0, 220.0);
const ROUTE_X: f64 = 560.0;
const ROUTE_Y: f64 = 210.0;
const ROUTE_Z: f64 = 30.0;
const ROUTE_PITCH_X: f64 = 78.0;
const NORMAL_ROUTE_Y: f64 = 34.0;
const BYPASS_ROUTE_Y: f64 = -34.0;
const ROUTE_CHANNEL_D: f64 = 8.2;
const ROUTE_RIB_Z: f64 = 18.0;

const TOKEN_CENTER: (f64, f64) = (-354.0, 18.0);
const TOKEN_RAIL_X: f64 = 462.0;
const TOKEN_RAIL_Y: f64 = 150.0;
const TOKEN_RAIL_Z: f64 = 28.0;
const TOKEN_PITCH_X: f64 = 74.0;
const TOKEN_PUCK_D: f64 = 26.0;

const PRESSURE_CENTER: (f64, f64) = (172.0, 18.0);
const PRESSURE_X: f64 = 460.0;
const PRESSURE_Y: f64 = 150.0;
const PRESSURE_Z: f64 = 36.0;
const PRESSURE_PITCH_X: f64 = 70.0;
const PRESSURE_POINT_PITCH_Y: f64 = 42.0;
const SENSOR_POCKET_X: f64 = 36.0;
const SENSOR_POCKET_Y: f64 = 24.0;
const SENSOR_POCKET_Z: f64 = 14.0;

const RELIEF_CENTER: (f64, f64) = (-418.0, -184.0);
const RELIEF_X: f64 = 330.0;
const RELIEF_Y: f64 = 162.0;
const RELIEF_Z: f64 = 54.0;
const RELIEF_PITCH_X: f64 = 52.0;
const RELIEF_WELL_D: f64 = 38.0;

const WINDOW_CENTER: (f64, f64) = (-34.0, -184.0);
const WINDOW_X: f64 = 360.0;
const WINDOW_Y: f64 = 162.0;
const WINDOW_Z: f64 = 26.0;
const WINDOW_PITCH_X: f64 = 54.0;
const WINDOW_APERTURE_X: f64 = 34.0;
const WINDOW_APERTURE_Y: f64 = 56.0;

const RECOVERY_CENTER: (f64, f64) = (360.0, -184.0);
const RECOVERY_X: f64 = 330.0;
const RECOVERY_Y: f64 = 162.0;
const RECOVERY_Z: f64 = 48.0;
const RECOVERY_PITCH_X: f64 = 52.0;
const VIAL_NEST_D: f64 = 31.0;

const TRACE_CENTER: (f64, f64) = (-386.0, -332.0);
const TRACE_X: f64 = 360.0;
const TRACE_Y: f64 = 92.0;
const TRACE_Z: f64 = 12.0;

const DISPOSITION_CENTER: (f64, f64) = (54.0, -326.0);
const DISPOSITION_X: f64 = 404.0;
const DISPOSITION_Y: f64 = 108.0;
const DISPOSITION_Z: f64 = 30.0;

const BRIDGE_CENTER: (f64, f64) = (420.0, -326.0);
const BRIDGE_X: f64 = 252.0;
const BRIDGE_Y: f64 = 108.0;
const BRIDGE_Z: f64 = 118.0;
const CAMERA_BRIDGE_CLEARANCE_Z: f64 = 88.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = containment_leak_tray();
    export(&tray, OUTPUTS[0]);

    let valve_bank = valve_cartridge_nest_bank();
    export(&valve_bank, OUTPUTS[1]);

    let routes = normal_bypass_route_witness_lanes();
    export(&routes, OUTPUTS[2]);

    let tokens = fail_open_closed_token_rails();
    export(&tokens, OUTPUTS[3]);

    let pressure = pressure_spike_sensor_pockets();
    export(&pressure, OUTPUTS[4]);

    let relief = relief_path_capture_wells();
    export(&relief, OUTPUTS[5]);

    let windows = bubble_wetness_windows();
    export(&windows, OUTPUTS[6]);

    let recovery = dye_recovery_vial_nests();
    export(&recovery, OUTPUTS[7]);

    let custody = barcode_custody_lands();
    export(&custody, OUTPUTS[8]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[9]);

    let bridge = camera_bridge_robot_keepout_gauges();
    export(&bridge, OUTPUTS[10]);

    let assembly = tray
        + valve_bank
        + routes
        + tokens
        + pressure
        + relief
        + windows
        + recovery
        + custody
        + disposition
        + bridge;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed perfusion bypass-valve fail-position witness station: {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray, {VALVE_LANES} no-cell valve lanes, {ROUTE_STATES} route states, {FAIL_STATE_TOKENS} fail-position tokens, {PRESSURE_SENSOR_POCKETS} pressure spike sensor pockets, {RELIEF_WELLS} relief capture wells, and {RECOVERY_VIALS} dye recovery nests."
    );
    println!(
        "Evidence features: {BUBBLE_WINDOWS} bubble windows, {WETNESS_PADS} wetness pads, {BARCODE_LANDS} barcode lands, {CUSTODY_LANDS} custody lands, {DISPOSITION_LANES} release/hold/reject lanes, {KEEP_OUT_GAUGES} keepout gauges, {CAMERA_TARGETS} camera targets, {} limitations, and {} required feature groups.",
        LIMITATIONS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn lane_x(lane: usize, pitch: f64) -> f64 {
    centered_index(lane, VALVE_LANES, pitch)
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert_eq!(
        PRESSURE_SENSOR_POCKETS,
        VALVE_LANES * PRESSURE_POINTS_PER_LANE
    );
    assert_eq!(FAIL_STATE_TOKENS, VALVE_LANES * FAIL_TOKENS_PER_LANE);
    assert_eq!(BUBBLE_WINDOWS, VALVE_LANES);
    assert_eq!(RELIEF_WELLS, VALVE_LANES);
    assert_eq!(RECOVERY_VIALS, VALVE_LANES);
    assert_eq!(DATUM_COUNT, 5);
    assert_eq!(TOKEN_RAILS, 2);
    assert!(DISPOSITION_LANES * STATUS_SLOTS_PER_LANE >= VALVE_LANES);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));

    for (name, center, width, depth) in insert_specs() {
        assert!(
            fits_on_station(center, width, depth),
            "{name} exceeds contained station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        (
            "valve_cartridge_nest_bank",
            VALVE_BANK_CENTER,
            VALVE_BANK_X,
            VALVE_BANK_Y,
        ),
        (
            "normal_bypass_route_witness_lanes",
            ROUTE_CENTER,
            ROUTE_X,
            ROUTE_Y,
        ),
        (
            "fail_open_closed_token_rails",
            TOKEN_CENTER,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
        (
            "pressure_spike_sensor_pockets",
            PRESSURE_CENTER,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        (
            "relief_path_capture_wells",
            RELIEF_CENTER,
            RELIEF_X,
            RELIEF_Y,
        ),
        ("bubble_wetness_windows", WINDOW_CENTER, WINDOW_X, WINDOW_Y),
        (
            "dye_recovery_vial_nests",
            RECOVERY_CENTER,
            RECOVERY_X,
            RECOVERY_Y,
        ),
        ("barcode_custody_lands", TRACE_CENTER, TRACE_X, TRACE_Y),
        (
            "release_hold_reject_lanes",
            DISPOSITION_CENTER,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        (
            "camera_bridge_robot_keepout_gauges",
            BRIDGE_CENTER,
            BRIDGE_X,
            BRIDGE_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn containment_leak_tray() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_containment_leak_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_shallow_wet_evidence_basin"),
        STATION_X - 116.0,
        STATION_Y - 116.0,
        BASIN_RECESS_Z,
    )
    .translate(0.0, -12.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.2);
    let valve_zone_recess = centered_cube(
        format!("{PREFIX}_upper_valve_route_zone_recess"),
        1120.0,
        236.0,
        7.0,
    )
    .translate(0.0, 220.0, BASE_Z - 3.3);
    let evidence_zone_recess = centered_cube(
        format!("{PREFIX}_middle_failure_evidence_zone_recess"),
        1120.0,
        174.0,
        7.0,
    )
    .translate(0.0, -84.0, BASE_Z - 3.3);
    let low_point_drain = centered_cylinder(
        format!("{PREFIX}_front_low_point_leak_drain"),
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 84.0,
        -(STATION_Y / 2.0 - 10.0),
        BASE_Z - 5.0,
    );

    deck - basin - valve_zone_recess - evidence_zone_recess - low_point_drain - mounting_holes()
        + containment_rims()
        + zone_dividers()
        + leak_witness_gutters()
        + datum_fiducials()
        + insertion_socket_lands()
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_holes"));
    for (i, (x, y)) in mount_points().iter().copied().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
}

fn containment_rims() -> Part {
    let z = BASE_Z + RIM_Z / 2.0;
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front_lip = centered_cube(
        format!("{PREFIX}_front_low_service_lip"),
        STATION_X - 150.0,
        RIM_W,
        RIM_Z * 0.55,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z + RIM_Z * 0.275,
    );

    left + right + rear + front_lip
}

fn zone_dividers() -> Part {
    let upper = centered_cube(
        format!("{PREFIX}_valve_to_evidence_zone_divider"),
        STATION_X - 160.0,
        12.0,
        30.0,
    )
    .translate(0.0, 96.0, BASE_Z + 15.0);
    let lower = centered_cube(
        format!("{PREFIX}_evidence_to_disposition_zone_divider"),
        STATION_X - 160.0,
        12.0,
        30.0,
    )
    .translate(0.0, -266.0, BASE_Z + 15.0);
    let valve_to_route = centered_cube(
        format!("{PREFIX}_valve_bank_to_route_lanes_divider"),
        10.0,
        220.0,
        28.0,
    )
    .translate(-42.0, 220.0, BASE_Z + 14.0);
    let token_to_pressure = centered_cube(
        format!("{PREFIX}_fail_token_to_pressure_divider"),
        10.0,
        154.0,
        28.0,
    )
    .translate(-92.0, 18.0, BASE_Z + 14.0);
    let relief_to_windows = centered_cube(
        format!("{PREFIX}_relief_to_window_divider"),
        10.0,
        158.0,
        28.0,
    )
    .translate(-226.0, -184.0, BASE_Z + 14.0);
    let windows_to_recovery = centered_cube(
        format!("{PREFIX}_window_to_recovery_divider"),
        10.0,
        158.0,
        28.0,
    )
    .translate(168.0, -184.0, BASE_Z + 14.0);

    upper + lower + valve_to_route + token_to_pressure + relief_to_windows + windows_to_recovery
}

fn leak_witness_gutters() -> Part {
    let normal_to_window = centered_cube(
        format!("{PREFIX}_normal_route_leak_gutter"),
        8.0,
        392.0,
        6.0,
    )
    .translate(122.0, 18.0, BASE_Z + 3.0);
    let bypass_to_relief = centered_cube(
        format!("{PREFIX}_bypass_route_leak_gutter"),
        420.0,
        8.0,
        6.0,
    )
    .translate(-142.0, -96.0, BASE_Z + 3.0);
    let recovery_to_front = centered_cube(
        format!("{PREFIX}_recovery_overflow_front_gutter"),
        8.0,
        188.0,
        6.0,
    )
    .translate(510.0, -286.0, BASE_Z + 3.0);
    let mut rails = Part::empty(format!("{PREFIX}_front_wetness_witness_rails"));
    for i in 0..8 {
        rails =
            rails
                + centered_cube(format!("{PREFIX}_front_witness_rail_{i}"), 112.0, 5.0, 7.0)
                    .translate(centered_index(i, 8, 138.0), -370.0, BASE_Z + 3.5);
    }
    normal_to_window + bypass_to_relief + recovery_to_front + rails
}

fn datum_fiducials() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_datum_and_robot_fiducials"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 82.0), STATION_Y / 2.0 - 78.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 78.0),
        (-(STATION_X / 2.0 - 82.0), -(STATION_Y / 2.0 - 78.0)),
        (STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 78.0)),
        (0.0, STATION_Y / 2.0 - 78.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        datums = datums
            + fiducial_disc(format!("{PREFIX}_datum_fiducial_{i}")).translate(x, y, BASE_Z + 2.5);
    }
    datums
}

fn insertion_socket_lands() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_insert_socket_lands"));
    for (name, center, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{name}_shallow_socket_land"),
                width + 10.0,
                depth + 10.0,
                3.0,
            )
            .translate(center.0, center.1, BASE_Z + 1.5);
    }
    sockets
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_disc"), 13.0, 4.0, 40)
        + centered_cube(format!("{name}_cross_x"), 22.0, 3.0, 5.0)
        + centered_cube(format!("{name}_cross_y"), 3.0, 22.0, 5.0)
}

fn valve_cartridge_nest_bank() -> Part {
    let bank = centered_cube(
        format!("{PREFIX}_valve_cartridge_nest_bank_body"),
        VALVE_BANK_X,
        VALVE_BANK_Y,
        VALVE_BANK_Z,
    )
    .translate(
        VALVE_BANK_CENTER.0,
        VALVE_BANK_CENTER.1,
        BASE_Z + VALVE_BANK_Z / 2.0,
    );
    let mut slots = Part::empty(format!("{PREFIX}_valve_cartridge_slots"));
    let mut keys = Part::empty(format!("{PREFIX}_valve_orientation_keys"));
    let mut tube_bores = Part::empty(format!("{PREFIX}_valve_port_bore_cuts"));

    for lane in 0..VALVE_LANES {
        let x = VALVE_BANK_CENTER.0 + lane_x(lane, VALVE_PITCH_X);
        slots = slots
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_valve_cartridge_pocket"),
                VALVE_SLOT_X,
                VALVE_SLOT_Y,
                VALVE_SLOT_Z,
            )
            .translate(
                x,
                VALVE_BANK_CENTER.1,
                BASE_Z + VALVE_BANK_Z - VALVE_SLOT_Z / 2.0 + 0.2,
            );
        keys = keys
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_normal_position_key_rib"),
                10.0,
                VALVE_SLOT_Y + 24.0,
                18.0,
            )
            .translate(x - 23.0, VALVE_BANK_CENTER.1, BASE_Z + VALVE_BANK_Z + 9.0)
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_bypass_position_key_rib"),
                10.0,
                VALVE_SLOT_Y + 24.0,
                12.0,
            )
            .translate(x + 23.0, VALVE_BANK_CENTER.1, BASE_Z + VALVE_BANK_Z + 6.0);
        for (port, y) in [("inlet", 58.0), ("outlet", -58.0)] {
            tube_bores = tube_bores
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_{port}_tube_bore"),
                    VALVE_BORE_D / 2.0,
                    44.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x,
                    VALVE_BANK_CENTER.1 + y,
                    BASE_Z + VALVE_BANK_Z * 0.58,
                );
        }
    }

    bank - slots - tube_bores + keys + valve_lane_id_lands()
}

fn valve_lane_id_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_valve_lane_id_lands"));
    for lane in 0..VALVE_LANES {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_valve_lane_{lane}_id_land"),
                46.0,
                14.0,
                4.0,
            )
            .translate(
                VALVE_BANK_CENTER.0 + lane_x(lane, VALVE_PITCH_X),
                VALVE_BANK_CENTER.1 - 90.0,
                BASE_Z + VALVE_BANK_Z + 2.0,
            );
    }
    lands
}

fn normal_bypass_route_witness_lanes() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_route_witness_lane_deck"),
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    )
    .translate(ROUTE_CENTER.0, ROUTE_CENTER.1, BASE_Z + ROUTE_Z / 2.0);
    let mut channels = Part::empty(format!("{PREFIX}_normal_bypass_route_channel_cuts"));
    let mut ribs = Part::empty(format!("{PREFIX}_normal_bypass_route_ribs"));
    let mut wrong_route_traps = Part::empty(format!("{PREFIX}_wrong_route_evidence_traps"));

    for lane in 0..VALVE_LANES {
        let x = ROUTE_CENTER.0 + lane_x(lane, ROUTE_PITCH_X);
        for (state, y_offset) in [("normal", NORMAL_ROUTE_Y), ("bypass", BYPASS_ROUTE_Y)] {
            let y = ROUTE_CENTER.1 + y_offset;
            channels = channels
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_{state}_route_visible_trough"),
                    ROUTE_CHANNEL_D / 2.0,
                    154.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, BASE_Z + ROUTE_Z + 1.0);
            ribs = ribs
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_{state}_flow_arrow_tail"),
                    8.0,
                    42.0,
                    ROUTE_RIB_Z,
                )
                .translate(x, y - 30.0, BASE_Z + ROUTE_Z + ROUTE_RIB_Z / 2.0)
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_{state}_flow_arrow_head"),
                    24.0,
                    12.0,
                    ROUTE_RIB_Z,
                )
                .translate(x, y + 42.0, BASE_Z + ROUTE_Z + ROUTE_RIB_Z / 2.0);
        }
        wrong_route_traps = wrong_route_traps
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_wrong_route_cross_over_trap"),
                40.0,
                20.0,
                9.0,
            )
            .translate(x, ROUTE_CENTER.1, BASE_Z + ROUTE_Z + 4.5);
    }

    deck - channels + ribs + wrong_route_traps + route_header_manifolds()
}

fn route_header_manifolds() -> Part {
    let normal_header = centered_cube(
        format!("{PREFIX}_normal_route_header_land"),
        ROUTE_X - 48.0,
        10.0,
        12.0,
    )
    .translate(
        ROUTE_CENTER.0,
        ROUTE_CENTER.1 + NORMAL_ROUTE_Y + 78.0,
        BASE_Z + ROUTE_Z + 6.0,
    );
    let bypass_header = centered_cube(
        format!("{PREFIX}_bypass_route_header_land"),
        ROUTE_X - 48.0,
        10.0,
        12.0,
    )
    .translate(
        ROUTE_CENTER.0,
        ROUTE_CENTER.1 + BYPASS_ROUTE_Y - 78.0,
        BASE_Z + ROUTE_Z + 6.0,
    );
    normal_header + bypass_header
}

fn fail_open_closed_token_rails() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_fail_open_closed_token_rail_body"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, BASE_Z + TOKEN_RAIL_Z / 2.0);
    let mut sockets = Part::empty(format!("{PREFIX}_fail_state_token_socket_cuts"));
    let mut legends = Part::empty(format!("{PREFIX}_fail_state_raised_legend_lands"));

    for lane in 0..VALVE_LANES {
        let x = TOKEN_CENTER.0 + lane_x(lane, TOKEN_PITCH_X);
        for (state_index, (state, y_offset)) in [("fail_open", 32.0), ("fail_closed", -32.0)]
            .iter()
            .copied()
            .enumerate()
        {
            sockets = sockets
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_{state}_token_socket"),
                    TOKEN_PUCK_D / 2.0,
                    11.0,
                    32,
                )
                .translate(
                    x,
                    TOKEN_CENTER.1 + y_offset,
                    BASE_Z + TOKEN_RAIL_Z - 4.8,
                );
            legends = legends
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_{state}_token_index_land_{state_index}"),
                    38.0,
                    10.0,
                    5.0,
                )
                .translate(
                    x,
                    TOKEN_CENTER.1 + y_offset + 24.0,
                    BASE_Z + TOKEN_RAIL_Z + 2.5,
                );
        }
    }

    let rail_label_top = centered_cube(
        format!("{PREFIX}_fail_open_rail_label_bar"),
        TOKEN_RAIL_X - 30.0,
        6.0,
        6.0,
    )
    .translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1 + 62.0,
        BASE_Z + TOKEN_RAIL_Z + 3.0,
    );
    let rail_label_bottom = centered_cube(
        format!("{PREFIX}_fail_closed_rail_label_bar"),
        TOKEN_RAIL_X - 30.0,
        6.0,
        6.0,
    )
    .translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1 - 62.0,
        BASE_Z + TOKEN_RAIL_Z + 3.0,
    );

    base - sockets + legends + rail_label_top + rail_label_bottom
}

fn pressure_spike_sensor_pockets() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_pressure_spike_sensor_panel"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1,
        BASE_Z + PRESSURE_Z / 2.0,
    );
    let mut pockets = Part::empty(format!("{PREFIX}_pressure_sensor_pocket_cuts"));
    let mut spike_bars = Part::empty(format!("{PREFIX}_pressure_transient_spike_marker_bars"));

    for lane in 0..VALVE_LANES {
        let x = PRESSURE_CENTER.0 + lane_x(lane, PRESSURE_PITCH_X);
        for point in 0..PRESSURE_POINTS_PER_LANE {
            let y = PRESSURE_CENTER.1
                + centered_index(point, PRESSURE_POINTS_PER_LANE, PRESSURE_POINT_PITCH_Y);
            pockets = pockets
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_pressure_point_{point}_sensor_pocket"),
                    SENSOR_POCKET_X,
                    SENSOR_POCKET_Y,
                    SENSOR_POCKET_Z,
                )
                .translate(x, y, BASE_Z + PRESSURE_Z - SENSOR_POCKET_Z / 2.0 + 0.2);
            spike_bars = spike_bars
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_pressure_point_{point}_spike_witness_bar"),
                    6.0 + point as f64 * 4.0,
                    6.0,
                    8.0,
                )
                .translate(x, y + 18.0, BASE_Z + PRESSURE_Z + 4.0);
        }
    }
    let transient_header = centered_cube(
        format!("{PREFIX}_pressure_transient_direction_header"),
        PRESSURE_X - 42.0,
        8.0,
        10.0,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1 + PRESSURE_Y / 2.0 - 20.0,
        BASE_Z + PRESSURE_Z + 5.0,
    );

    panel - pockets + spike_bars + transient_header
}

fn relief_path_capture_wells() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_relief_path_capture_well_block"),
        RELIEF_X,
        RELIEF_Y,
        RELIEF_Z,
    )
    .translate(RELIEF_CENTER.0, RELIEF_CENTER.1, BASE_Z + RELIEF_Z / 2.0);
    let mut wells = Part::empty(format!("{PREFIX}_relief_capture_well_cuts"));
    let mut overflow_lips = Part::empty(format!("{PREFIX}_relief_overflow_witness_lips"));
    for lane in 0..VALVE_LANES {
        let x = RELIEF_CENTER.0 + lane_x(lane, RELIEF_PITCH_X);
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_lane_{lane}_relief_capture_well"),
                RELIEF_WELL_D / 2.0,
                RELIEF_Z - 8.0,
                40,
            )
            .translate(x, RELIEF_CENTER.1, BASE_Z + RELIEF_Z / 2.0 + 5.0);
        overflow_lips = overflow_lips
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_relief_available_overflow_lip"),
                38.0,
                8.0,
                8.0,
            )
            .translate(x, RELIEF_CENTER.1 + 54.0, BASE_Z + RELIEF_Z + 4.0);
    }
    let relief_header = centered_cube(
        format!("{PREFIX}_relief_path_availability_header"),
        RELIEF_X - 36.0,
        10.0,
        10.0,
    )
    .translate(
        RELIEF_CENTER.0,
        RELIEF_CENTER.1 - 62.0,
        BASE_Z + RELIEF_Z + 5.0,
    );
    block - wells + overflow_lips + relief_header
}

fn bubble_wetness_windows() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_bubble_wetness_window_plate"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(WINDOW_CENTER.0, WINDOW_CENTER.1, BASE_Z + WINDOW_Z / 2.0);
    let mut apertures = Part::empty(format!("{PREFIX}_bubble_window_aperture_cuts"));
    let mut pads = Part::empty(format!("{PREFIX}_wetness_pad_lands"));
    for lane in 0..VALVE_LANES {
        let x = WINDOW_CENTER.0 + lane_x(lane, WINDOW_PITCH_X);
        apertures = apertures
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_bubble_window_aperture"),
                WINDOW_APERTURE_X,
                WINDOW_APERTURE_Y,
                WINDOW_Z + 4.0,
            )
            .translate(x, WINDOW_CENTER.1, BASE_Z + WINDOW_Z / 2.0);
        for (side, y_offset) in [("inlet", 50.0), ("outlet", -50.0)] {
            pads = pads
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_{side}_wetness_evidence_pad"),
                    36.0,
                    16.0,
                    5.0,
                )
                .translate(x, WINDOW_CENTER.1 + y_offset, BASE_Z + WINDOW_Z + 2.5);
        }
    }
    let backlight_land = centered_cube(
        format!("{PREFIX}_camera_backlight_alignment_land"),
        WINDOW_X - 32.0,
        8.0,
        7.0,
    )
    .translate(
        WINDOW_CENTER.0,
        WINDOW_CENTER.1 + WINDOW_Y / 2.0 - 14.0,
        BASE_Z + WINDOW_Z + 3.5,
    );
    plate - apertures + pads + backlight_land
}

fn dye_recovery_vial_nests() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_dye_recovery_vial_nest_block"),
        RECOVERY_X,
        RECOVERY_Y,
        RECOVERY_Z,
    )
    .translate(
        RECOVERY_CENTER.0,
        RECOVERY_CENTER.1,
        BASE_Z + RECOVERY_Z / 2.0,
    );
    let mut nests = Part::empty(format!("{PREFIX}_dye_recovery_vial_nest_cuts"));
    let mut recovery_route_flags = Part::empty(format!("{PREFIX}_recovery_routing_flags"));
    for lane in 0..VALVE_LANES {
        let x = RECOVERY_CENTER.0 + lane_x(lane, RECOVERY_PITCH_X);
        nests = nests
            + centered_cylinder(
                format!("{PREFIX}_lane_{lane}_dye_recovery_vial_socket"),
                VIAL_NEST_D / 2.0,
                RECOVERY_Z - 7.0,
                40,
            )
            .translate(x, RECOVERY_CENTER.1, BASE_Z + RECOVERY_Z / 2.0 + 4.0);
        recovery_route_flags = recovery_route_flags
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_recovery_route_verified_flag"),
                28.0,
                10.0,
                7.0,
            )
            .translate(x, RECOVERY_CENTER.1 + 56.0, BASE_Z + RECOVERY_Z + 3.5);
    }
    let recovery_header = centered_cube(
        format!("{PREFIX}_post_fault_recovery_route_header"),
        RECOVERY_X - 36.0,
        8.0,
        9.0,
    )
    .translate(
        RECOVERY_CENTER.0,
        RECOVERY_CENTER.1 - 60.0,
        BASE_Z + RECOVERY_Z + 4.5,
    );
    block - nests + recovery_route_flags + recovery_header
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_barcode_custody_panel"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, BASE_Z + TRACE_Z / 2.0);
    let mut lands = Part::empty(format!("{PREFIX}_barcode_and_custody_lands"));
    for lane in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_barcode_land"),
                42.0,
                18.0,
                4.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(lane, BARCODE_LANDS, 48.0),
                TRACE_CENTER.1 + 20.0,
                BASE_Z + TRACE_Z + 2.0,
            );
    }
    for i in 0..CUSTODY_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_custody_signature_land_{i}"),
                70.0,
                18.0,
                4.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(i, CUSTODY_LANDS, 82.0),
                TRACE_CENTER.1 - 24.0,
                BASE_Z + TRACE_Z + 2.0,
            );
    }
    panel + lands
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_panel"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1,
        BASE_Z + DISPOSITION_Z / 2.0,
    );
    let mut slots = Part::empty(format!("{PREFIX}_disposition_slot_cuts"));
    let mut labels = Part::empty(format!("{PREFIX}_disposition_label_lands"));
    for (lane_index, name) in ["release", "hold", "reject"].iter().enumerate() {
        let y = DISPOSITION_CENTER.1 + centered_index(lane_index, DISPOSITION_LANES, 32.0);
        labels = labels
            + centered_cube(format!("{PREFIX}_{name}_lane_label_land"), 72.0, 16.0, 5.0).translate(
                DISPOSITION_CENTER.0 - 156.0,
                y,
                BASE_Z + DISPOSITION_Z + 2.5,
            );
        for slot in 0..STATUS_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("{PREFIX}_{name}_lane_slot_{slot}"),
                    46.0,
                    18.0,
                    12.0,
                )
                .translate(
                    DISPOSITION_CENTER.0 - 44.0 + slot as f64 * 58.0,
                    y,
                    BASE_Z + DISPOSITION_Z - 5.8,
                );
        }
    }
    panel - slots + labels
}

fn camera_bridge_robot_keepout_gauges() -> Part {
    let left_pier = centered_cube(
        format!("{PREFIX}_camera_bridge_left_pier"),
        18.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 - BRIDGE_X / 2.0 + 18.0,
        BRIDGE_CENTER.1,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let right_pier = centered_cube(
        format!("{PREFIX}_camera_bridge_right_pier"),
        18.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_CENTER.0 + BRIDGE_X / 2.0 - 18.0,
        BRIDGE_CENTER.1,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let top = centered_cube(
        format!("{PREFIX}_camera_evidence_bridge_top_beam"),
        BRIDGE_X,
        18.0,
        18.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        BASE_Z + CAMERA_BRIDGE_CLEARANCE_Z + 9.0,
    );
    let mut targets = Part::empty(format!("{PREFIX}_camera_evidence_targets"));
    for target in 0..CAMERA_TARGETS {
        targets = targets
            + centered_cube(
                format!("{PREFIX}_camera_target_lane_{target}"),
                24.0,
                6.0,
                6.0,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(target, CAMERA_TARGETS, 34.0),
                BRIDGE_CENTER.1 + 40.0,
                BASE_Z + CAMERA_BRIDGE_CLEARANCE_Z + 22.0,
            );
    }
    let mut gauges = Part::empty(format!("{PREFIX}_robot_service_keepout_gauges"));
    for i in 0..KEEP_OUT_GAUGES {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_robot_service_keepout_gauge_{i}"),
                10.0 + i as f64 * 6.0,
                7.0,
                42.0,
            )
            .translate(
                BRIDGE_CENTER.0 - 108.0 + i as f64 * 42.0,
                BRIDGE_CENTER.1 - 42.0,
                BASE_Z + 21.0,
            );
    }
    left_pier + right_pier + top + targets + gauges
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_table_has_expected_count_prefix_and_unique_paths() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
    }

    #[test]
    fn feature_groups_match_requested_witness_station_scope() {
        for required in [
            "containment_leak_tray",
            "valve_cartridge_nest_bank",
            "normal_bypass_route_witness_lanes",
            "fail_open_closed_token_rails",
            "pressure_spike_sensor_pockets",
            "relief_path_capture_wells",
            "bubble_wetness_windows",
            "dye_recovery_vial_nests",
            "barcode_custody_lands",
            "release_hold_reject_lanes",
            "camera_bridge_robot_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&required));
        }
    }

    #[test]
    fn geometry_intent_preserves_no_cell_fail_position_validation_counts() {
        assert_eq!(VALVE_LANES, 6);
        assert_eq!(ROUTE_STATES, 2);
        assert_eq!(TOKEN_RAILS, 2);
        assert_eq!(FAIL_STATE_TOKENS, 12);
        assert_eq!(PRESSURE_SENSOR_POCKETS, 18);
        assert_eq!(RELIEF_WELLS, VALVE_LANES);
        assert_eq!(RECOVERY_VIALS, VALVE_LANES);
        assert_eq!(BUBBLE_WINDOWS, VALVE_LANES);
        assert_eq!(WETNESS_PADS, VALVE_LANES * 2);
        assert!(LIMITATIONS.contains(&"no_cell_validation_station_only"));
    }

    #[test]
    fn all_subassemblies_fit_inside_containment_rim() {
        assert_layout();
        assert!(BRIDGE_Z > CAMERA_BRIDGE_CLEARANCE_Z);
        assert!(STATION_X > ROUTE_X + VALVE_BANK_X);
        assert!(STATION_Y > 760.0);
    }
}
