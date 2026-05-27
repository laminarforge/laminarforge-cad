use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sensor data timestamp-alignment station.
//
// Intent:
// - Validate that independent pressure, flow, pH/DO, imaging, environmental,
//   scale/load-cell, and robot-event data streams can be aligned to a common
//   run clock before records are released for culture-run review.
// - Keep the timing interfaces visible as named CSG geometry: timestamp beacon
//   panel, sync pulse manifold, event token rail, camera fiducial clock target,
//   logger docks, cable/connector route witnesses, drift comparison lanes,
//   quarantine/release decision gates, and run-record custody lands.
//
// Research assumptions encoded in the fixture:
// - FDA/WHO data-integrity guidance expects records to be attributable,
//   legible, contemporaneous, original, accurate, complete, consistent,
//   enduring, and available. The custody lands, labeled logger docks, and
//   event tokens are mechanical reminders that raw logs, image frames, and
//   robot events must stay traceable to the run record.
// - PAT/data-historian practice correlates process measurements from multiple
//   instruments on a common timebase. The design therefore gives every stream
//   its own physical logger dock and drift lane while sharing the same beacon
//   and pulse manifold.
// - NIST/IEEE 1588/PTP/NTP/PPS time-sync practice separates reference clock
//   distribution from verification. The station models that separation with
//   reference beacon docks, sync-pulse fanout, visual camera clock target, and
//   per-stream drift witnesses. It does not claim clock accuracy.
//
// This is validation fixture/interface CAD only. It does not define software
// time-sync algorithms, regulatory acceptance limits, sterile processing
// instructions, sensor calibration methods, or metrology performance claims.

const OUTPUT_PREFIX: &str = "output/closed_sensor_data_timestamp_alignment_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_sensor_data_timestamp_alignment_station_base_timebase_deck.stl",
    "output/closed_sensor_data_timestamp_alignment_station_timestamp_beacon_panel.stl",
    "output/closed_sensor_data_timestamp_alignment_station_sync_pulse_manifold.stl",
    "output/closed_sensor_data_timestamp_alignment_station_event_token_rail.stl",
    "output/closed_sensor_data_timestamp_alignment_station_camera_fiducial_clock_target.stl",
    "output/closed_sensor_data_timestamp_alignment_station_logger_dock_array.stl",
    "output/closed_sensor_data_timestamp_alignment_station_cable_connector_route_witnesses.stl",
    "output/closed_sensor_data_timestamp_alignment_station_drift_comparison_lanes.stl",
    "output/closed_sensor_data_timestamp_alignment_station_quarantine_release_decision_gates.stl",
    "output/closed_sensor_data_timestamp_alignment_station_run_record_custody_lands.stl",
    "output/closed_sensor_data_timestamp_alignment_station_robot_service_keepouts.stl",
    "output/closed_sensor_data_timestamp_alignment_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "timestamp_beacon_panel",
    "sync_pulse_manifold",
    "event_token_rail",
    "camera_fiducial_clock_target",
    "logger_docks",
    "cable_connector_route_witnesses",
    "drift_comparison_lanes",
    "quarantine_release_decision_gates",
    "run_record_custody_lands",
    "robot_service_keepouts",
    "standalone_stl_exports",
];

const LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "no_time_sync_algorithm",
    "no_regulatory_acceptance_limits",
    "no_sterile_barrier_claim",
    "no_sensor_calibration_method",
    "no_metrology_performance_claim",
];

const DESIGN_ASSUMPTIONS: [&str; 5] = [
    "alcoa_plus_contemporaneous_traceable_records",
    "common_reference_timebase_for_all_streams",
    "separate_sync_distribution_from_verification",
    "camera_frames_need_visual_clock_fiducial",
    "quarantine_unaligned_streams_before_release",
];

const STREAM_COUNT: usize = 7;
const STREAM_NAMES: [&str; STREAM_COUNT] = [
    "pressure",
    "flow",
    "ph_do",
    "imaging",
    "environmental",
    "scale_load_cell",
    "robot_events",
];

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 960.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.8;

const BEACON_POS: (f64, f64) = (-575.0, 255.0);
const BEACON_X: f64 = 350.0;
const BEACON_Y: f64 = 220.0;
const BEACON_Z: f64 = 48.0;
const TIMEBASE_SOURCE_COUNT: usize = 3;
const BEACON_WINDOW_COUNT: usize = STREAM_COUNT;
const PPS_JACK_COUNT: usize = STREAM_COUNT + 1;
const BEACON_DOCK_D: f64 = 38.0;
const BEACON_WINDOW_X: f64 = 34.0;
const BEACON_WINDOW_Y: f64 = 22.0;

const MANIFOLD_POS: (f64, f64) = (-165.0, 255.0);
const MANIFOLD_X: f64 = 360.0;
const MANIFOLD_Y: f64 = 220.0;
const MANIFOLD_Z: f64 = 56.0;
const SYNC_INPUT_PORTS: usize = STREAM_COUNT;
const SYNC_OUTPUT_PORTS: usize = STREAM_COUNT;
const REFERENCE_PULSE_PORTS: usize = 2;
const SYNC_TOTAL_PORTS: usize = SYNC_INPUT_PORTS + SYNC_OUTPUT_PORTS + REFERENCE_PULSE_PORTS;
const SYNC_PORT_D: f64 = 10.4;
const SYNC_PORT_PITCH_X: f64 = 44.0;
const MANIFOLD_VALVE_COUNT: usize = STREAM_COUNT;

const CAMERA_POS: (f64, f64) = (250.0, 255.0);
const CAMERA_X: f64 = 320.0;
const CAMERA_Y: f64 = 220.0;
const CAMERA_Z: f64 = 34.0;
const CAMERA_FIDUCIALS: usize = 4;
const CLOCK_TICK_MARKS: usize = 12;
const VISUAL_BEACON_WINDOWS: usize = STREAM_COUNT;
const FIDUCIAL_D: f64 = 28.0;
const CLOCK_FACE_D: f64 = 118.0;

const LOGGER_POS: (f64, f64) = (600.0, 255.0);
const LOGGER_X: f64 = 300.0;
const LOGGER_Y: f64 = 220.0;
const LOGGER_Z: f64 = 42.0;
const LOGGER_DOCKS: usize = STREAM_COUNT + 1;
const LOGGER_ROWS: usize = 2;
const LOGGER_COLS: usize = 4;
const LOGGER_DOCK_X: f64 = 56.0;
const LOGGER_DOCK_Y: f64 = 54.0;
const LOGGER_DOCK_DEPTH: f64 = 22.0;
const LOGGER_PITCH_X: f64 = 66.0;
const LOGGER_PITCH_Y: f64 = 82.0;

const ROUTE_POS: (f64, f64) = (-530.0, 0.0);
const ROUTE_X: f64 = 400.0;
const ROUTE_Y: f64 = 200.0;
const ROUTE_Z: f64 = 38.0;
const ROUTE_WITNESS_LANES: usize = STREAM_COUNT;
const CONNECTOR_DOCKS: usize = STREAM_COUNT * 2;
const ROUTE_TIE_POINTS_PER_LANE: usize = 3;
const ROUTE_CHANNEL_D: f64 = 8.4;
const ROUTE_LANE_PITCH_Y: f64 = 24.0;

const DRIFT_POS: (f64, f64) = (-45.0, 0.0);
const DRIFT_X: f64 = 520.0;
const DRIFT_Y: f64 = 200.0;
const DRIFT_Z: f64 = 34.0;
const DRIFT_LANES: usize = STREAM_COUNT;
const DRIFT_TICK_STATIONS: usize = 5;
const DRIFT_WITNESS_SLOTS: usize = DRIFT_LANES * DRIFT_TICK_STATIONS;
const DRIFT_LANE_PITCH_Y: f64 = 24.0;
const DRIFT_TICK_PITCH_X: f64 = 86.0;

const GATE_POS: (f64, f64) = (515.0, 0.0);
const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 200.0;
const GATE_Z: f64 = 44.0;
const DECISION_GATE_COUNT: usize = 3;
const DECISION_GATE_LABELS: [&str; DECISION_GATE_COUNT] = ["release", "quarantine", "resync"];
const GATE_TOKEN_SLOTS_PER_GATE: usize = STREAM_COUNT;
const GATE_LANE_X: f64 = 118.0;
const GATE_LANE_Y: f64 = 152.0;
const GATE_PITCH_X: f64 = 136.0;

const TOKEN_POS: (f64, f64) = (-540.0, -290.0);
const TOKEN_X: f64 = 380.0;
const TOKEN_Y: f64 = 190.0;
const TOKEN_Z: f64 = 30.0;
const EVENT_TOKEN_TYPES: [&str; 9] = [
    "run_start",
    "pump_step",
    "valve_change",
    "image_frame",
    "ph_do_sample",
    "scale_tare",
    "environment_alarm",
    "robot_pick_place",
    "run_stop",
];
const EVENT_TOKEN_SLOTS: usize = EVENT_TOKEN_TYPES.len();
const EVENT_TOKEN_ROWS: usize = 3;
const EVENT_TOKEN_COLS: usize = 3;
const EVENT_TOKEN_D: f64 = 23.0;
const EVENT_TOKEN_PITCH_X: f64 = 82.0;
const EVENT_TOKEN_PITCH_Y: f64 = 52.0;

const CUSTODY_POS: (f64, f64) = (-75.0, -290.0);
const CUSTODY_X: f64 = 500.0;
const CUSTODY_Y: f64 = 190.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 6;
const RAW_LOG_CUSTODY_LANDS: usize = STREAM_COUNT;
const CUSTODY_SEAL_WELLS: usize = 4;
const AUDIT_EXPORT_SLOTS: usize = 3;
const CUSTODY_LAND_X: f64 = 70.0;
const CUSTODY_LAND_Y: f64 = 26.0;

const KEEP_OUT_X: f64 = 1490.0;
const KEEP_OUT_Y: f64 = 880.0;
const KEEP_OUT_Z: f64 = 7.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 405.0;
const REAR_CLOCK_SERVICE_CLEARANCE: f64 = 150.0;
const LEFT_CABLE_SERVICE_CLEARANCE: f64 = 165.0;
const RIGHT_LOGGER_SERVICE_CLEARANCE: f64 = 170.0;
const LOGGER_LIFT_CLEARANCE_Z: f64 = 138.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps_with_clearance(self, other: Footprint, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_timebase_deck();
    export(OUTPUTS[0], &deck);

    let beacon = timestamp_beacon_panel();
    export(OUTPUTS[1], &beacon);

    let manifold = sync_pulse_manifold();
    export(OUTPUTS[2], &manifold);

    let tokens = event_token_rail();
    export(OUTPUTS[3], &tokens);

    let camera = camera_fiducial_clock_target();
    export(OUTPUTS[4], &camera);

    let loggers = logger_dock_array();
    export(OUTPUTS[5], &loggers);

    let routes = cable_connector_route_witnesses();
    export(OUTPUTS[6], &routes);

    let drift = drift_comparison_lanes();
    export(OUTPUTS[7], &drift);

    let gates = quarantine_release_decision_gates();
    export(OUTPUTS[8], &gates);

    let custody = run_record_custody_lands();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + beacon.translate(BEACON_POS.0, BEACON_POS.1, on_deck_z(BEACON_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_deck_z(MANIFOLD_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_deck_z(LOGGER_Z))
        + routes.translate(ROUTE_POS.0, ROUTE_POS.1, on_deck_z(ROUTE_Z))
        + drift.translate(DRIFT_POS.0, DRIFT_POS.1, on_deck_z(DRIFT_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed sensor data timestamp-alignment station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm timebase validation deck"
    );
    println!(
        "  Stream coverage:           {STREAM_COUNT} streams ({})",
        STREAM_NAMES.join(", ")
    );
    println!(
        "  Timebase interfaces:       {TIMEBASE_SOURCE_COUNT} reference beacon docks, {PPS_JACK_COUNT} PPS/check jacks, {SYNC_TOTAL_PORTS} sync pulse ports"
    );
    println!(
        "  Evidence capture:          {CAMERA_FIDUCIALS} camera fiducials, {CLOCK_TICK_MARKS} clock ticks, {VISUAL_BEACON_WINDOWS} visual beacon windows"
    );
    println!(
        "  Logger and routing:        {LOGGER_DOCKS} logger docks, {ROUTE_WITNESS_LANES} routed witness lanes, {CONNECTOR_DOCKS} connector docks"
    );
    println!(
        "  Drift/disposition:         {DRIFT_LANES} drift lanes with {DRIFT_WITNESS_SLOTS} tick witness slots, {} decision gates",
        DECISION_GATE_LABELS.join("/")
    );
    println!(
        "  Run record custody:        {RUN_RECORD_LANDS} run-record lands, {RAW_LOG_CUSTODY_LANDS} raw-log custody lands, {CUSTODY_SEAL_WELLS} seal wells, {AUDIT_EXPORT_SLOTS} audit export slots"
    );
    println!(
        "  Scope controls:            {KEEP_OUT_ZONE_COUNT} keepout gauges, {} required feature groups and {} explicit non-scope notes",
        REQUIRED_FEATURES.len(),
        LIMITATIONS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_position(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn stream_label(index: usize) -> &'static str {
    STREAM_NAMES.get(index).copied().unwrap_or("unknown_stream")
}

fn logger_label(index: usize) -> &'static str {
    if index < STREAM_COUNT {
        stream_label(index)
    } else {
        "reference_clock"
    }
}

fn station_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "timestamp_beacon_panel",
            center: BEACON_POS,
            x: BEACON_X,
            y: BEACON_Y,
        },
        Footprint {
            name: "sync_pulse_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Footprint {
            name: "camera_fiducial_clock_target",
            center: CAMERA_POS,
            x: CAMERA_X,
            y: CAMERA_Y,
        },
        Footprint {
            name: "logger_dock_array",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Footprint {
            name: "cable_connector_route_witnesses",
            center: ROUTE_POS,
            x: ROUTE_X,
            y: ROUTE_Y,
        },
        Footprint {
            name: "drift_comparison_lanes",
            center: DRIFT_POS,
            x: DRIFT_X,
            y: DRIFT_Y,
        },
        Footprint {
            name: "quarantine_release_decision_gates",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Footprint {
            name: "event_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "run_record_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));

    for feature in [
        "timestamp_beacon_panel",
        "sync_pulse_manifold",
        "event_token_rail",
        "camera_fiducial_clock_target",
        "logger_docks",
        "cable_connector_route_witnesses",
        "drift_comparison_lanes",
        "quarantine_release_decision_gates",
        "run_record_custody_lands",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for assumption in [
        "alcoa_plus_contemporaneous_traceable_records",
        "common_reference_timebase_for_all_streams",
        "separate_sync_distribution_from_verification",
        "camera_frames_need_visual_clock_fiducial",
        "quarantine_unaligned_streams_before_release",
    ] {
        assert!(DESIGN_ASSUMPTIONS.contains(&assumption));
    }

    assert_eq!(STREAM_COUNT, STREAM_NAMES.len());
    assert_eq!(TIMEBASE_SOURCE_COUNT, 3);
    assert_eq!(BEACON_WINDOW_COUNT, STREAM_COUNT);
    assert_eq!(PPS_JACK_COUNT, STREAM_COUNT + 1);
    assert_eq!(SYNC_INPUT_PORTS, STREAM_COUNT);
    assert_eq!(SYNC_OUTPUT_PORTS, STREAM_COUNT);
    assert_eq!(
        SYNC_TOTAL_PORTS,
        SYNC_INPUT_PORTS + SYNC_OUTPUT_PORTS + REFERENCE_PULSE_PORTS
    );
    assert_eq!(MANIFOLD_VALVE_COUNT, STREAM_COUNT);
    assert_eq!(VISUAL_BEACON_WINDOWS, STREAM_COUNT);
    assert_eq!(LOGGER_DOCKS, LOGGER_ROWS * LOGGER_COLS);
    assert_eq!(LOGGER_DOCKS, STREAM_COUNT + 1);
    assert_eq!(ROUTE_WITNESS_LANES, STREAM_COUNT);
    assert_eq!(CONNECTOR_DOCKS, STREAM_COUNT * 2);
    assert_eq!(DRIFT_LANES, STREAM_COUNT);
    assert_eq!(DRIFT_WITNESS_SLOTS, DRIFT_LANES * DRIFT_TICK_STATIONS);
    assert_eq!(EVENT_TOKEN_SLOTS, EVENT_TOKEN_ROWS * EVENT_TOKEN_COLS);
    assert_eq!(GATE_TOKEN_SLOTS_PER_GATE, STREAM_COUNT);
    assert_eq!(RAW_LOG_CUSTODY_LANDS, STREAM_COUNT);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);

    assert!(BEACON_DOCK_D + 14.0 < BEACON_X / TIMEBASE_SOURCE_COUNT as f64);
    assert!(SYNC_PORT_D + 16.0 < SYNC_PORT_PITCH_X);
    assert!(ROUTE_CHANNEL_D < ROUTE_LANE_PITCH_Y - 8.0);
    assert!(DRIFT_TICK_PITCH_X * (DRIFT_TICK_STATIONS as f64 - 1.0) < DRIFT_X - 95.0);
    assert!(CLOCK_FACE_D + 60.0 < CAMERA_X);
    assert!(LOGGER_LIFT_CLEARANCE_Z > LOGGER_Z + 80.0);

    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds timestamp alignment deck",
            footprint.name
        );
    }

    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(
                !a.overlaps_with_clearance(*b, 8.0),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }
}

fn base_timebase_deck() -> Part {
    let deck = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_base_timebase_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_recessed_wipe_pan",
        STATION_X - 118.0,
        STATION_Y - 116.0,
        7.0,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - 3.5);
    let front_custody_sump = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_front_custody_sump",
        STATION_X - 220.0,
        132.0,
        8.0,
    )
    .translate(0.0, -298.0, BASE_Z / 2.0 - 4.0);

    deck - washdown_recess - front_custody_sump - deck_module_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + row_divider_rails()
        + deck_timebase_direction_ticks()
        + robot_datum_targets()
}

fn deck_module_sockets() -> Part {
    let mut sockets = Part::empty("closed_sensor_data_timestamp_alignment_station_module_sockets");
    for footprint in station_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{}_registration_socket",
                    footprint.name
                ),
                footprint.x + 12.0,
                footprint.y + 12.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mounting_holes() -> Part {
    let mut holes = Part::empty("closed_sensor_data_timestamp_alignment_station_mounting_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 56.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 56.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 56.0),
        (0.0, -(STATION_Y / 2.0 - 56.0)),
        (0.0, STATION_Y / 2.0 - 56.0),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_front_low_custody_lip",
        STATION_X - 190.0,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_rear_clock_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_left_cable_rim",
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
        "closed_sensor_data_timestamp_alignment_station_right_logger_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn row_divider_rails() -> Part {
    let top_to_mid = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_clock_to_drift_divider",
        STATION_X - 210.0,
        10.0,
        28.0,
    )
    .translate(0.0, 124.0, BASE_Z / 2.0 + 14.0);
    let mid_to_custody = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_drift_to_custody_divider",
        STATION_X - 230.0,
        10.0,
        26.0,
    )
    .translate(0.0, -154.0, BASE_Z / 2.0 + 13.0);
    let route_to_drift = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_route_to_drift_divider",
        10.0,
        212.0,
        26.0,
    )
    .translate(-300.0, 0.0, BASE_Z / 2.0 + 13.0);
    let drift_to_gate = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_drift_to_gate_divider",
        10.0,
        212.0,
        26.0,
    )
    .translate(250.0, 0.0, BASE_Z / 2.0 + 13.0);
    top_to_mid + mid_to_custody + route_to_drift + drift_to_gate
}

fn deck_timebase_direction_ticks() -> Part {
    let mut ticks =
        Part::empty("closed_sensor_data_timestamp_alignment_station_timebase_direction_ticks");
    for i in 0..STREAM_COUNT {
        let x = centered_index(i, STREAM_COUNT, 165.0);
        ticks = ticks
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_time_arrow_stem_{i}"),
                96.0,
                5.0,
                6.0,
            )
            .translate(x, 126.0, BASE_Z / 2.0 + 3.0)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_time_arrow_head_{i}"),
                18.0,
                18.0,
                6.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 55.0, 126.0, BASE_Z / 2.0 + 3.0);
    }
    ticks
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_sensor_data_timestamp_alignment_station_datum_targets");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 88.0), STATION_Y / 2.0 - 88.0),
        (STATION_X / 2.0 - 88.0, STATION_Y / 2.0 - 88.0),
        (-(STATION_X / 2.0 - 88.0), -(STATION_Y / 2.0 - 88.0)),
        (STATION_X / 2.0 - 88.0, -(STATION_Y / 2.0 - 88.0)),
    ]
    .into_iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_sensor_data_timestamp_alignment_station_robot_datum_ring_{i}"),
            17.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("closed_sensor_data_timestamp_alignment_station_robot_datum_bore_{i}"),
            3.4,
            8.0,
            24,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        targets = targets + (ring - bore);
    }
    targets
}

fn timestamp_beacon_panel() -> Part {
    let panel = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_beacon_panel_body",
        BEACON_X,
        BEACON_Y,
        BEACON_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_beacon_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_beacon_features");

    for source in 0..TIMEBASE_SOURCE_COUNT {
        let x = centered_index(source, TIMEBASE_SOURCE_COUNT, 92.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_timebase_source_{source}_dock_recess"),
                BEACON_DOCK_D / 2.0,
                BEACON_Z - 10.0,
                44,
            )
            .translate(x, 46.0, 4.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_timebase_source_{source}_certificate_land"),
                70.0,
                16.0,
                5.0,
            )
            .translate(x, 82.0, BEACON_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_timebase_source_{source}_keyed_backstop"),
                58.0,
                9.0,
                20.0,
            )
            .translate(x, 18.0, BEACON_Z / 2.0 + 10.0);
    }

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, 42.0);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_beacon_window_recess"
                ),
                BEACON_WINDOW_X,
                BEACON_WINDOW_Y,
                10.0,
            )
            .translate(x, -20.0, BEACON_Z / 2.0 - 4.0)
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_pps_jack_bore"),
                SYNC_PORT_D / 2.0,
                BEACON_Z + 8.0,
                24,
            )
            .translate(x, -67.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_beacon_window_index_land"),
                BEACON_WINDOW_X + 6.0,
                5.0,
                5.0,
            )
            .translate(x, -4.0, BEACON_Z / 2.0 + 2.5);
    }

    cuts = cuts
        + centered_cylinder(
            "closed_sensor_data_timestamp_alignment_station_reference_pps_check_jack",
            SYNC_PORT_D / 2.0,
            BEACON_Z + 8.0,
            24,
        )
        .translate(BEACON_X / 2.0 - 34.0, -67.0, 0.0);

    panel - cuts + features + beacon_guard_ribs()
}

fn beacon_guard_ribs() -> Part {
    let top = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_beacon_top_guard_rib",
        BEACON_X - 34.0,
        8.0,
        15.0,
    )
    .translate(0.0, BEACON_Y / 2.0 - 20.0, BEACON_Z / 2.0 + 7.5);
    let bottom = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_beacon_bottom_guard_rib",
        BEACON_X - 34.0,
        8.0,
        15.0,
    )
    .translate(0.0, -BEACON_Y / 2.0 + 20.0, BEACON_Z / 2.0 + 7.5);
    top + bottom
}

fn sync_pulse_manifold() -> Part {
    let body = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_sync_pulse_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_sync_manifold_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_sync_manifold_features");

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, SYNC_PORT_PITCH_X);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_sync_input_port"),
                SYNC_PORT_D / 2.0,
                MANIFOLD_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -MANIFOLD_Y / 2.0 + 18.0, 2.0)
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_sync_output_port"),
                SYNC_PORT_D / 2.0,
                MANIFOLD_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, MANIFOLD_Y / 2.0 - 18.0, 2.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_isolator_valve_pocket"
                ),
                14.0,
                MANIFOLD_Z + 8.0,
                36,
            )
            .translate(x, 0.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_sync_state_land"),
                40.0,
                18.0,
                5.0,
            )
            .translate(x, 0.0, MANIFOLD_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_pulse_trace_rib"),
                5.0,
                MANIFOLD_Y - 54.0,
                5.0,
            )
            .translate(x, 0.0, MANIFOLD_Z / 2.0 + 2.5);
    }

    for (i, x) in [-MANIFOLD_X / 2.0 + 34.0, MANIFOLD_X / 2.0 - 34.0]
        .into_iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_reference_pulse_port_{i}"),
                SYNC_PORT_D / 2.0,
                MANIFOLD_Z + 8.0,
                24,
            )
            .translate(x, 0.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_reference_pulse_port_{i}_tag_land"),
                42.0,
                18.0,
                5.0,
            )
            .translate(x, -36.0, MANIFOLD_Z / 2.0 + 2.5);
    }

    body - cuts + features + manifold_flow_ribs()
}

fn manifold_flow_ribs() -> Part {
    let upper = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_sync_manifold_upper_common_bus_rib",
        MANIFOLD_X - 62.0,
        8.0,
        7.0,
    )
    .translate(0.0, 50.0, MANIFOLD_Z / 2.0 + 3.5);
    let lower = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_sync_manifold_lower_common_bus_rib",
        MANIFOLD_X - 62.0,
        8.0,
        7.0,
    )
    .translate(0.0, -50.0, MANIFOLD_Z / 2.0 + 3.5);
    upper + lower
}

fn event_token_rail() -> Part {
    let rail = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_event_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_event_token_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_event_token_features");

    for (event, label) in EVENT_TOKEN_TYPES.iter().enumerate() {
        let (x, y) = grid_position(
            event,
            EVENT_TOKEN_COLS,
            EVENT_TOKEN_ROWS,
            EVENT_TOKEN_PITCH_X,
            EVENT_TOKEN_PITCH_Y,
        );
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_token_well"),
                EVENT_TOKEN_D / 2.0,
                TOKEN_Z - 6.0,
                36,
            )
            .translate(x, y, 4.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_token_label_land"),
                58.0,
                15.0,
                4.0,
            )
            .translate(x, y - 24.0, TOKEN_Z / 2.0 + 2.0)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_anti_swap_key"),
                7.0,
                18.0,
                8.0,
            )
            .translate(x + 18.0, y, TOKEN_Z / 2.0 + 4.0);
    }

    rail - cuts + features + event_rail_ridges()
}

fn event_rail_ridges() -> Part {
    let mut ridges =
        Part::empty("closed_sensor_data_timestamp_alignment_station_event_rail_ridges");
    for row in 0..=EVENT_TOKEN_ROWS {
        let y = -((EVENT_TOKEN_ROWS as f64) * EVENT_TOKEN_PITCH_Y) / 2.0
            + row as f64 * EVENT_TOKEN_PITCH_Y;
        ridges = ridges
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_event_row_separator_{row}"),
                TOKEN_X - 44.0,
                5.0,
                6.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 3.0);
    }
    ridges
}

fn camera_fiducial_clock_target() -> Part {
    let plate = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_camera_clock_target_plate",
        CAMERA_X,
        CAMERA_Y,
        CAMERA_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_camera_clock_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_camera_clock_features");

    cuts = cuts
        + centered_cylinder(
            "closed_sensor_data_timestamp_alignment_station_clock_face_recess",
            CLOCK_FACE_D / 2.0,
            10.0,
            72,
        )
        .translate(0.0, 22.0, CAMERA_Z / 2.0 - 4.0);
    features = features
        + centered_cylinder(
            "closed_sensor_data_timestamp_alignment_station_clock_face_outer_ring",
            CLOCK_FACE_D / 2.0 + 5.0,
            5.0,
            72,
        )
        .translate(0.0, 22.0, CAMERA_Z / 2.0 + 2.5)
        - centered_cylinder(
            "closed_sensor_data_timestamp_alignment_station_clock_face_inner_opening",
            CLOCK_FACE_D / 2.0 - 5.0,
            7.0,
            72,
        )
        .translate(0.0, 22.0, CAMERA_Z / 2.0 + 2.5);

    for tick in 0..CLOCK_TICK_MARKS {
        let angle = tick as f64 * 30.0;
        let radius = CLOCK_FACE_D / 2.0 - 8.0;
        let x = angle.to_radians().sin() * radius;
        let y = 22.0 + angle.to_radians().cos() * radius;
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_clock_tick_{tick:02}"),
                5.0,
                18.0,
                5.0,
            )
            .rotate(0.0, 0.0, -angle)
            .translate(x, y, CAMERA_Z / 2.0 + 2.5);
    }

    for (i, (x, y)) in [
        (-(CAMERA_X / 2.0 - 42.0), -(CAMERA_Y / 2.0 - 42.0)),
        (CAMERA_X / 2.0 - 42.0, -(CAMERA_Y / 2.0 - 42.0)),
        (-(CAMERA_X / 2.0 - 42.0), CAMERA_Y / 2.0 - 42.0),
        (CAMERA_X / 2.0 - 42.0, CAMERA_Y / 2.0 - 42.0),
    ]
    .into_iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_sensor_data_timestamp_alignment_station_camera_fiducial_ring_{i}"),
            FIDUCIAL_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y, CAMERA_Z / 2.0 + 2.5);
        let dot = centered_cylinder(
            format!("closed_sensor_data_timestamp_alignment_station_camera_fiducial_center_{i}"),
            4.0,
            7.0,
            20,
        )
        .translate(x, y, CAMERA_Z / 2.0 + 2.5);
        features = features + (ring - dot);
    }

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, 36.0);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_visual_beacon_window"
                ),
                24.0,
                14.0,
                8.0,
            )
            .translate(x, -74.0, CAMERA_Z / 2.0 - 3.0);
    }

    plate - cuts + features
}

fn logger_dock_array() -> Part {
    let tray = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_logger_dock_tray",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_logger_dock_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_logger_dock_features");

    for dock in 0..LOGGER_DOCKS {
        let (x, y) = grid_position(
            dock,
            LOGGER_COLS,
            LOGGER_ROWS,
            LOGGER_PITCH_X,
            LOGGER_PITCH_Y,
        );
        let label = logger_label(dock);
        cuts = cuts
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_logger_recess"),
                LOGGER_DOCK_X,
                LOGGER_DOCK_Y,
                LOGGER_DOCK_DEPTH,
            )
            .translate(x, y, LOGGER_Z / 2.0 - LOGGER_DOCK_DEPTH / 2.0 + 1.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_logger_connector_bore"
                ),
                5.2,
                LOGGER_DOCK_Y + 10.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - LOGGER_DOCK_Y / 2.0, 3.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_logger_latch_land_left"),
                8.0,
                LOGGER_DOCK_Y + 14.0,
                11.0,
            )
            .translate(x - LOGGER_DOCK_X / 2.0 - 7.0, y, LOGGER_Z / 2.0 + 5.5)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_logger_latch_land_right"),
                8.0,
                LOGGER_DOCK_Y + 14.0,
                11.0,
            )
            .translate(x + LOGGER_DOCK_X / 2.0 + 7.0, y, LOGGER_Z / 2.0 + 5.5)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_logger_barcode_land"),
                46.0,
                12.0,
                4.0,
            )
            .translate(x, y + LOGGER_DOCK_Y / 2.0 + 13.0, LOGGER_Z / 2.0 + 2.0);
    }

    tray - cuts + features + logger_dock_row_keys()
}

fn logger_dock_row_keys() -> Part {
    let upper = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_logger_upper_row_key",
        LOGGER_X - 40.0,
        6.0,
        7.0,
    )
    .translate(0.0, LOGGER_PITCH_Y / 2.0, LOGGER_Z / 2.0 + 3.5);
    let lower = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_logger_lower_row_key",
        LOGGER_X - 40.0,
        6.0,
        7.0,
    )
    .translate(0.0, -LOGGER_PITCH_Y / 2.0, LOGGER_Z / 2.0 + 3.5);
    upper + lower
}

fn cable_connector_route_witnesses() -> Part {
    let base = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_route_witness_base",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_route_witness_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_route_witness_features");

    for stream in 0..STREAM_COUNT {
        let y = centered_index(stream, STREAM_COUNT, ROUTE_LANE_PITCH_Y);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_cable_route_channel"
                ),
                ROUTE_CHANNEL_D / 2.0,
                ROUTE_X - 64.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 2.0);
        for side in [-1.0, 1.0] {
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_sensor_data_timestamp_alignment_station_{label}_{}_connector_dock",
                        if side < 0.0 { "source" } else { "logger" }
                    ),
                    13.0,
                    ROUTE_Z + 8.0,
                    30,
                )
                .translate(side * (ROUTE_X / 2.0 - 36.0), y, 0.0);
        }
        for tie in 0..ROUTE_TIE_POINTS_PER_LANE {
            let x = centered_index(tie, ROUTE_TIE_POINTS_PER_LANE, 116.0);
            features = features
                + centered_cube(
                    format!("closed_sensor_data_timestamp_alignment_station_{label}_route_tie_witness_{tie}"),
                    16.0,
                    8.0,
                    8.0,
                )
                .translate(x, y + 10.0, ROUTE_Z / 2.0 + 4.0);
        }
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_route_label_land"),
                58.0,
                11.0,
                4.0,
            )
            .translate(-ROUTE_X / 2.0 + 78.0, y - 10.0, ROUTE_Z / 2.0 + 2.0);
    }

    base - cuts + features + route_lane_separators()
}

fn route_lane_separators() -> Part {
    let mut separators =
        Part::empty("closed_sensor_data_timestamp_alignment_station_route_lane_separators");
    for lane in 0..=STREAM_COUNT {
        let y =
            -((STREAM_COUNT as f64) * ROUTE_LANE_PITCH_Y) / 2.0 + lane as f64 * ROUTE_LANE_PITCH_Y;
        separators = separators
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_route_separator_{lane}"),
                ROUTE_X - 44.0,
                3.0,
                5.0,
            )
            .translate(0.0, y, ROUTE_Z / 2.0 + 2.5);
    }
    separators
}

fn drift_comparison_lanes() -> Part {
    let panel = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_drift_comparison_panel",
        DRIFT_X,
        DRIFT_Y,
        DRIFT_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_drift_lane_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_drift_lane_features");

    for stream in 0..DRIFT_LANES {
        let y = centered_index(stream, DRIFT_LANES, DRIFT_LANE_PITCH_Y);
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_drift_lane_rail"),
                DRIFT_X - 52.0,
                5.0,
                6.0,
            )
            .translate(0.0, y, DRIFT_Z / 2.0 + 3.0);
        for tick in 0..DRIFT_TICK_STATIONS {
            let x = centered_index(tick, DRIFT_TICK_STATIONS, DRIFT_TICK_PITCH_X);
            cuts = cuts
                + centered_cube(
                    format!("closed_sensor_data_timestamp_alignment_station_{label}_drift_tick_{tick}_source_slot"),
                    28.0,
                    8.0,
                    8.0,
                )
                .translate(x, y - 6.0, DRIFT_Z / 2.0 - 3.0)
                + centered_cube(
                    format!("closed_sensor_data_timestamp_alignment_station_{label}_drift_tick_{tick}_reference_slot"),
                    28.0,
                    8.0,
                    8.0,
                )
                .translate(x, y + 6.0, DRIFT_Z / 2.0 - 3.0);
            features = features
                + centered_cylinder(
                    format!("closed_sensor_data_timestamp_alignment_station_{label}_drift_tick_{tick}_witness_pin"),
                    3.2,
                    6.0,
                    18,
                )
                .translate(x, y, DRIFT_Z / 2.0 + 3.0);
        }
    }

    panel - cuts + features + drift_reference_rulers()
}

fn drift_reference_rulers() -> Part {
    let top = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_drift_upper_reference_ruler",
        DRIFT_X - 64.0,
        7.0,
        6.0,
    )
    .translate(0.0, DRIFT_Y / 2.0 - 22.0, DRIFT_Z / 2.0 + 3.0);
    let bottom = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_drift_lower_reference_ruler",
        DRIFT_X - 64.0,
        7.0,
        6.0,
    )
    .translate(0.0, -DRIFT_Y / 2.0 + 22.0, DRIFT_Z / 2.0 + 3.0);
    top + bottom
}

fn quarantine_release_decision_gates() -> Part {
    let base = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_decision_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut cuts = Part::empty("closed_sensor_data_timestamp_alignment_station_decision_gate_cuts");
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_decision_gate_features");

    for gate in 0..DECISION_GATE_COUNT {
        let x = centered_index(gate, DECISION_GATE_COUNT, GATE_PITCH_X);
        let label = DECISION_GATE_LABELS[gate];
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_decision_lane_recess"
                ),
                GATE_LANE_X,
                GATE_LANE_Y,
                18.0,
            )
            .translate(x, 0.0, GATE_Z / 2.0 - 7.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_decision_gate_high_back"),
                GATE_LANE_X,
                10.0,
                36.0 + gate as f64 * 8.0,
            )
            .translate(x, GATE_LANE_Y / 2.0 - 5.0, GATE_Z / 2.0 + 18.0 + gate as f64 * 4.0)
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_{label}_decision_label_land"),
                GATE_LANE_X - 28.0,
                18.0,
                5.0,
            )
            .translate(x, -GATE_LANE_Y / 2.0 + 24.0, GATE_Z / 2.0 + 2.5);
        for slot in 0..GATE_TOKEN_SLOTS_PER_GATE {
            let y = centered_index(slot, GATE_TOKEN_SLOTS_PER_GATE, 16.5);
            cuts = cuts
                + centered_cube(
                    format!("closed_sensor_data_timestamp_alignment_station_{label}_stream_{slot}_decision_token_slot"),
                    26.0,
                    9.0,
                    10.0,
                )
                .translate(x, y, GATE_Z / 2.0 - 4.0);
        }
    }

    base - cuts + features + decision_gate_lock_bar()
}

fn decision_gate_lock_bar() -> Part {
    let release_lock = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_release_gate_green_key_bar",
        62.0,
        GATE_Y - 44.0,
        10.0,
    )
    .translate(
        centered_index(0, DECISION_GATE_COUNT, GATE_PITCH_X),
        0.0,
        GATE_Z / 2.0 + 5.0,
    );
    let quarantine_lock = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_quarantine_gate_red_key_bar",
        78.0,
        GATE_Y - 30.0,
        16.0,
    )
    .translate(
        centered_index(1, DECISION_GATE_COUNT, GATE_PITCH_X),
        0.0,
        GATE_Z / 2.0 + 8.0,
    );
    let resync_lock = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_resync_gate_blue_key_bar",
        54.0,
        GATE_Y - 60.0,
        12.0,
    )
    .translate(
        centered_index(2, DECISION_GATE_COUNT, GATE_PITCH_X),
        0.0,
        GATE_Z / 2.0 + 6.0,
    );
    release_lock + quarantine_lock + resync_lock
}

fn run_record_custody_lands() -> Part {
    let plate = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_run_record_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut features =
        Part::empty("closed_sensor_data_timestamp_alignment_station_run_record_custody_features");
    let mut cuts =
        Part::empty("closed_sensor_data_timestamp_alignment_station_run_record_custody_cuts");

    for land in 0..RUN_RECORD_LANDS {
        let x = centered_index(land, RUN_RECORD_LANDS, 78.0);
        features = features
            + centered_cube(
                format!("closed_sensor_data_timestamp_alignment_station_run_record_land_{land}"),
                CUSTODY_LAND_X,
                CUSTODY_LAND_Y,
                5.0,
            )
            .translate(x, 58.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for stream in 0..RAW_LOG_CUSTODY_LANDS {
        let x = centered_index(stream, RAW_LOG_CUSTODY_LANDS, 62.0);
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_{label}_raw_log_custody_land"
                ),
                50.0,
                18.0,
                4.0,
            )
            .translate(x, 12.0, CUSTODY_Z / 2.0 + 2.0);
    }

    for well in 0..CUSTODY_SEAL_WELLS {
        let x = centered_index(well, CUSTODY_SEAL_WELLS, 96.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_data_timestamp_alignment_station_custody_seal_well_{well}"),
                14.0,
                CUSTODY_Z + 5.0,
                36,
            )
            .translate(x, -44.0, 0.0);
    }

    for slot in 0..AUDIT_EXPORT_SLOTS {
        let x = centered_index(slot, AUDIT_EXPORT_SLOTS, 128.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_data_timestamp_alignment_station_audit_export_slot_land_{slot}"
                ),
                86.0,
                22.0,
                4.0,
            )
            .translate(x, -80.0, CUSTODY_Z / 2.0 + 2.0);
    }

    plate - cuts + features
}

fn robot_service_keepouts() -> Part {
    let base_gauge = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_keepout_outer_footprint_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let front_robot = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_front_robot_sweep_keepout",
        KEEP_OUT_X - 150.0,
        10.0,
        34.0,
    )
    .translate(0.0, -FRONT_ROBOT_SWEEP_CLEARANCE, BASE_Z / 2.0 + 17.0);
    let rear_clock = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_rear_clock_service_keepout",
        KEEP_OUT_X - 190.0,
        10.0,
        32.0,
    )
    .translate(0.0, REAR_CLOCK_SERVICE_CLEARANCE, BASE_Z / 2.0 + 16.0);
    let left_cable = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_left_cable_service_keepout",
        10.0,
        KEEP_OUT_Y - 160.0,
        28.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_CABLE_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + 14.0,
    );
    let right_logger = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_right_logger_service_keepout",
        10.0,
        KEEP_OUT_Y - 160.0,
        28.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_LOGGER_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + 14.0,
    );
    let logger_lift = centered_cube(
        "closed_sensor_data_timestamp_alignment_station_logger_lift_clearance_gauge",
        LOGGER_X + 70.0,
        12.0,
        18.0,
    )
    .translate(
        LOGGER_POS.0,
        LOGGER_POS.1 + LOGGER_Y / 2.0 + 26.0,
        BASE_Z / 2.0 + LOGGER_LIFT_CLEARANCE_Z,
    );

    base_gauge + front_robot + rear_clock + left_cable + right_logger + logger_lift
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_station_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn every_required_data_stream_has_alignment_hardware() {
        assert_eq!(
            STREAM_NAMES,
            [
                "pressure",
                "flow",
                "ph_do",
                "imaging",
                "environmental",
                "scale_load_cell",
                "robot_events",
            ]
        );
        assert_eq!(BEACON_WINDOW_COUNT, STREAM_COUNT);
        assert_eq!(SYNC_INPUT_PORTS, STREAM_COUNT);
        assert_eq!(SYNC_OUTPUT_PORTS, STREAM_COUNT);
        assert_eq!(LOGGER_DOCKS, STREAM_COUNT + 1);
        assert_eq!(ROUTE_WITNESS_LANES, STREAM_COUNT);
        assert_eq!(DRIFT_LANES, STREAM_COUNT);
        assert_eq!(RAW_LOG_CUSTODY_LANDS, STREAM_COUNT);
    }

    #[test]
    fn pulse_and_drift_counts_are_parametric() {
        assert_eq!(SYNC_TOTAL_PORTS, 16);
        assert_eq!(MANIFOLD_VALVE_COUNT, STREAM_COUNT);
        assert_eq!(DRIFT_WITNESS_SLOTS, 35);
        assert_eq!(CONNECTOR_DOCKS, 14);
        assert_eq!(EVENT_TOKEN_SLOTS, 9);
        assert_eq!(EVENT_TOKEN_SLOTS, EVENT_TOKEN_ROWS * EVENT_TOKEN_COLS);
        assert_eq!(GATE_TOKEN_SLOTS_PER_GATE * DECISION_GATE_COUNT, 21);
    }

    #[test]
    fn layout_footprints_fit_without_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn dimensions_leave_physical_service_clearance() {
        assert!(STATION_X >= 1500.0);
        assert!(LOGGER_LIFT_CLEARANCE_Z > LOGGER_Z + 80.0);
        assert!(FRONT_ROBOT_SWEEP_CLEARANCE > 380.0);
        assert!(BEACON_DOCK_D + 14.0 < BEACON_X / TIMEBASE_SOURCE_COUNT as f64);
        assert!(SYNC_PORT_D + 16.0 < SYNC_PORT_PITCH_X);
        assert!(ROUTE_CHANNEL_D < ROUTE_LANE_PITCH_Y - 8.0);
    }

    #[test]
    fn data_integrity_scope_is_explicit() {
        for feature in [
            "timestamp_beacon_panel",
            "sync_pulse_manifold",
            "event_token_rail",
            "camera_fiducial_clock_target",
            "logger_docks",
            "cable_connector_route_witnesses",
            "drift_comparison_lanes",
            "quarantine_release_decision_gates",
            "run_record_custody_lands",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "missing {feature}");
        }
        assert!(DESIGN_ASSUMPTIONS.contains(&"alcoa_plus_contemporaneous_traceable_records"));
        assert!(DESIGN_ASSUMPTIONS.contains(&"common_reference_timebase_for_all_streams"));
        assert!(DESIGN_ASSUMPTIONS.contains(&"quarantine_unaligned_streams_before_release"));
        assert!(LIMITATIONS.contains(&"no_time_sync_algorithm"));
        assert!(LIMITATIONS.contains(&"no_metrology_performance_claim"));
    }
}
