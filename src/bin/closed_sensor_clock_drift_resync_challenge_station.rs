use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sensor clock-drift resync challenge station.
//
// Intent:
// - Package a no-cell validation station that challenges independent sensor
//   logger clocks with controlled drift, power-cycle, and resync events before
//   culture-run data is accepted.
// - Keep reference clock custody, local logger docks, drift injection controls,
//   PPS/PTP/NTP resync fanout, timestamp skew witnesses, temperature/voltage
//   stress pockets, event replay tokens, image evidence targets, and
//   quarantine/release gates visible as deterministic mechanical interfaces.
//
// Research assumptions encoded in the fixture:
// - Reproducible tissue-chip runs need sensor observations to be attributable
//   to the correct time window, not only to the correct chip lane.
// - Time synchronization practice separates a trusted reference clock, clock
//   distribution, local-device drift challenge, and post-run record custody.
// - Drift and resync must be tested independently from biology, so this station
//   is no-cell validation packaging rather than a culture protocol.
//
// This is validation fixture/interface CAD only. It does not define a clock
// synchronization algorithm, acceptance limits, regulatory data-integrity
// claims, sterile processing instructions, sensor calibration methods, or
// biological performance claims.

const OUTPUT_PREFIX: &str = "output/closed_sensor_clock_drift_resync_challenge_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_sensor_clock_drift_resync_challenge_station_base_challenge_deck.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_reference_clock_dock_bank.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_drift_injection_clock_emulator_panel.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_sensor_logger_challenge_lanes.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_resync_pulse_fanout_manifold.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_timestamp_skew_witness_ruler.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_temperature_voltage_drift_stress_pocket.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_event_replay_token_rail.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_camera_evidence_clock_target.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_resync_quarantine_release_gates.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_data_custody_evidence_lands.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_robot_service_keepouts.stl",
    "output/closed_sensor_clock_drift_resync_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_challenge_deck",
    "reference_clock_dock_bank",
    "drift_injection_clock_emulator_panel",
    "sensor_logger_challenge_lanes",
    "resync_pulse_fanout_manifold",
    "timestamp_skew_witness_ruler",
    "temperature_voltage_drift_stress_pocket",
    "event_replay_token_rail",
    "camera_evidence_clock_target",
    "resync_quarantine_release_gates",
    "data_custody_evidence_lands",
    "robot_service_keepouts",
];

const DESIGN_ASSUMPTIONS: [&str; 6] = [
    "no_cell_clock_validation_before_culture_runs",
    "trusted_reference_clock_separate_from_local_loggers",
    "per_stream_logger_clock_drift_must_be_challenged",
    "resync_events_need_physical_event_tokens",
    "unaligned_streams_are_quarantined_before_release",
    "raw_logs_and_evidence_stay_traceable_to_run_record",
];

const LIMITATIONS: [&str; 7] = [
    "validation_fixture_only",
    "no_clock_sync_algorithm",
    "no_acceptance_thresholds",
    "no_regulatory_data_integrity_claim",
    "no_sterile_barrier_claim",
    "no_sensor_calibration_method",
    "no_biological_performance_claim",
];

const STREAM_COUNT: usize = 8;
const STREAM_NAMES: [&str; STREAM_COUNT] = [
    "pressure",
    "flow",
    "ph_do",
    "oxygen",
    "co2_humidity",
    "imaging",
    "scale_mass",
    "robot_events",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.8;
const TUBE_BORE_D: f64 = 6.4;

const REFERENCE_POS: (f64, f64) = (-525.0, 250.0);
const REFERENCE_X: f64 = 320.0;
const REFERENCE_Y: f64 = 190.0;
const REFERENCE_Z: f64 = 48.0;
const REFERENCE_CLOCK_DOCKS: usize = 3;
const REFERENCE_PPS_CHECK_JACKS: usize = STREAM_COUNT + 1;
const REFERENCE_DOCK_D: f64 = 38.0;
const REFERENCE_PPS_D: f64 = 9.6;

const DRIFT_POS: (f64, f64) = (-170.0, 250.0);
const DRIFT_X: f64 = 330.0;
const DRIFT_Y: f64 = 190.0;
const DRIFT_Z: f64 = 52.0;
const DRIFT_EMULATOR_CHANNELS: usize = STREAM_COUNT;
const DRIFT_RANGE_STOPS: usize = 5;
const DRIFT_DIAL_D: f64 = 23.0;
const DRIFT_CHANNEL_PITCH: f64 = 36.0;

const FANOUT_POS: (f64, f64) = (185.0, 250.0);
const FANOUT_X: f64 = 340.0;
const FANOUT_Y: f64 = 190.0;
const FANOUT_Z: f64 = 56.0;
const RESYNC_INPUT_PORTS: usize = 2;
const RESYNC_OUTPUT_PORTS: usize = STREAM_COUNT;
const RESYNC_MONITOR_PORTS: usize = STREAM_COUNT;
const FANOUT_PORT_D: f64 = 9.6;
const FANOUT_PORT_PITCH: f64 = 36.0;

const STRESS_POS: (f64, f64) = (545.0, 250.0);
const STRESS_X: f64 = 250.0;
const STRESS_Y: f64 = 190.0;
const STRESS_Z: f64 = 58.0;
const TEMPERATURE_WELLS: usize = 4;
const VOLTAGE_RAIL_SLOTS: usize = 4;
const STRESS_SENSOR_WELLS: usize = 3;
const TEMPERATURE_WELL_D: f64 = 29.0;
const VOLTAGE_SLOT_X: f64 = 42.0;
const VOLTAGE_SLOT_Y: f64 = 22.0;

const LOGGER_POS: (f64, f64) = (-500.0, 0.0);
const LOGGER_X: f64 = 350.0;
const LOGGER_Y: f64 = 220.0;
const LOGGER_Z: f64 = 44.0;
const LOGGER_LANE_PITCH: f64 = 25.0;
const LOGGER_POCKET_X: f64 = 82.0;
const LOGGER_POCKET_Y: f64 = 18.0;
const LOGGER_RESET_PIN_D: f64 = 4.2;

const WITNESS_POS: (f64, f64) = (-105.0, 0.0);
const WITNESS_X: f64 = 390.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 34.0;
const SKEW_WITNESS_LANES: usize = STREAM_COUNT;
const SKEW_TICK_STATIONS: usize = 7;
const SKEW_TICK_PITCH: f64 = 45.0;
const SKEW_LANE_PITCH: f64 = 24.0;
const SKEW_WITNESS_SLOTS: usize = SKEW_WITNESS_LANES * SKEW_TICK_STATIONS;

const EVIDENCE_POS: (f64, f64) = (335.0, 0.0);
const EVIDENCE_X: f64 = 420.0;
const EVIDENCE_Y: f64 = 210.0;
const EVIDENCE_Z: f64 = 36.0;
const CAMERA_FIDUCIALS: usize = 4;
const CLOCK_FACE_D: f64 = 112.0;
const CLOCK_TICK_MARKS: usize = 12;
const LED_EVIDENCE_WINDOWS: usize = STREAM_COUNT;

const TOKEN_POS: (f64, f64) = (-515.0, -270.0);
const TOKEN_X: f64 = 330.0;
const TOKEN_Y: f64 = 170.0;
const TOKEN_Z: f64 = 30.0;
const EVENT_TOKEN_TYPES: [&str; 10] = [
    "run_start",
    "pps_loss",
    "ntp_dropout",
    "ptp_relock",
    "power_cycle",
    "temp_step",
    "voltage_sag",
    "manual_clock_set",
    "resync_accept",
    "run_stop",
];
const EVENT_TOKEN_ROWS: usize = 2;
const EVENT_TOKEN_COLS: usize = 5;
const EVENT_TOKEN_D: f64 = 21.0;
const EVENT_TOKEN_PITCH_X: f64 = 58.0;
const EVENT_TOKEN_PITCH_Y: f64 = 58.0;

const GATE_POS: (f64, f64) = (-125.0, -270.0);
const GATE_X: f64 = 380.0;
const GATE_Y: f64 = 170.0;
const GATE_Z: f64 = 42.0;
const GATE_NAMES: [&str; 3] = ["release", "resync", "quarantine"];
const DECISION_GATES: usize = GATE_NAMES.len();
const GATE_TOKEN_SLOTS_PER_GATE: usize = STREAM_COUNT;
const GATE_SLOT_X: f64 = 26.0;
const GATE_SLOT_Y: f64 = 18.0;
const GATE_PITCH_X: f64 = 112.0;

const CUSTODY_POS: (f64, f64) = (335.0, -270.0);
const CUSTODY_X: f64 = 420.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 5;
const RAW_CLOCK_LOG_LANDS: usize = STREAM_COUNT;
const EVIDENCE_SEAL_WELLS: usize = 4;
const AUDIT_EXPORT_SLOTS: usize = 4;

const KEEP_OUT_X: f64 = 1410.0;
const KEEP_OUT_Y: f64 = 820.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 405.0;
const REAR_CLOCK_SERVICE_CLEARANCE: f64 = 175.0;
const LEFT_CABLE_SERVICE_CLEARANCE: f64 = 165.0;
const RIGHT_DATA_SERVICE_CLEARANCE: f64 = 180.0;
const LOGGER_LIFT_CLEARANCE_Z: f64 = 140.0;

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

    let deck = base_challenge_deck();
    export(OUTPUTS[0], &deck);

    let reference = reference_clock_dock_bank();
    export(OUTPUTS[1], &reference);

    let drift = drift_injection_clock_emulator_panel();
    export(OUTPUTS[2], &drift);

    let loggers = sensor_logger_challenge_lanes();
    export(OUTPUTS[3], &loggers);

    let fanout = resync_pulse_fanout_manifold();
    export(OUTPUTS[4], &fanout);

    let witness = timestamp_skew_witness_ruler();
    export(OUTPUTS[5], &witness);

    let stress = temperature_voltage_drift_stress_pocket();
    export(OUTPUTS[6], &stress);

    let tokens = event_replay_token_rail();
    export(OUTPUTS[7], &tokens);

    let evidence = camera_evidence_clock_target();
    export(OUTPUTS[8], &evidence);

    let gates = resync_quarantine_release_gates();
    export(OUTPUTS[9], &gates);

    let custody = data_custody_evidence_lands();
    export(OUTPUTS[10], &custody);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + reference.translate(REFERENCE_POS.0, REFERENCE_POS.1, on_deck_z(REFERENCE_Z))
        + drift.translate(DRIFT_POS.0, DRIFT_POS.1, on_deck_z(DRIFT_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_deck_z(LOGGER_Z))
        + fanout.translate(FANOUT_POS.0, FANOUT_POS.1, on_deck_z(FANOUT_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + stress.translate(STRESS_POS.0, STRESS_POS.1, on_deck_z(STRESS_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_deck_z(EVIDENCE_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed sensor clock-drift resync challenge station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm no-cell timing validation deck"
    );
    println!(
        "  Stream coverage:           {STREAM_COUNT} streams ({})",
        STREAM_NAMES.join(", ")
    );
    println!(
        "  Clock challenge:           {REFERENCE_CLOCK_DOCKS} reference docks, {DRIFT_EMULATOR_CHANNELS} drift channels, {DRIFT_RANGE_STOPS} drift range stops"
    );
    println!(
        "  Resync fanout:             {RESYNC_INPUT_PORTS} reference inputs, {RESYNC_OUTPUT_PORTS} stream outputs, {RESYNC_MONITOR_PORTS} monitor returns"
    );
    println!(
        "  Skew witnesses:            {SKEW_WITNESS_LANES} lanes, {SKEW_TICK_STATIONS} tick stations, {SKEW_WITNESS_SLOTS} timestamp witness slots"
    );
    println!(
        "  Stress/evidence:           {TEMPERATURE_WELLS} temperature wells, {VOLTAGE_RAIL_SLOTS} voltage slots, {CAMERA_FIDUCIALS} camera fiducials, {LED_EVIDENCE_WINDOWS} LED windows"
    );
    println!(
        "  Run record controls:       {} event tokens, {DECISION_GATES} decision gates, {RUN_RECORD_LANDS} run-record lands, {RAW_CLOCK_LOG_LANDS} raw clock-log lands",
        EVENT_TOKEN_TYPES.len()
    );
    println!(
        "  Scope controls:            {KEEP_OUT_ZONE_COUNT} keepout gauges, {} required feature groups, {} explicit non-scope notes",
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

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "reference_clock_dock_bank",
            center: REFERENCE_POS,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
        Footprint {
            name: "drift_injection_clock_emulator_panel",
            center: DRIFT_POS,
            x: DRIFT_X,
            y: DRIFT_Y,
        },
        Footprint {
            name: "resync_pulse_fanout_manifold",
            center: FANOUT_POS,
            x: FANOUT_X,
            y: FANOUT_Y,
        },
        Footprint {
            name: "temperature_voltage_drift_stress_pocket",
            center: STRESS_POS,
            x: STRESS_X,
            y: STRESS_Y,
        },
        Footprint {
            name: "sensor_logger_challenge_lanes",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Footprint {
            name: "timestamp_skew_witness_ruler",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "camera_evidence_clock_target",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
        Footprint {
            name: "event_replay_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "resync_quarantine_release_gates",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Footprint {
            name: "data_custody_evidence_lands",
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
        "reference_clock_dock_bank",
        "drift_injection_clock_emulator_panel",
        "sensor_logger_challenge_lanes",
        "resync_pulse_fanout_manifold",
        "timestamp_skew_witness_ruler",
        "temperature_voltage_drift_stress_pocket",
        "event_replay_token_rail",
        "camera_evidence_clock_target",
        "resync_quarantine_release_gates",
        "data_custody_evidence_lands",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for assumption in [
        "trusted_reference_clock_separate_from_local_loggers",
        "per_stream_logger_clock_drift_must_be_challenged",
        "resync_events_need_physical_event_tokens",
        "unaligned_streams_are_quarantined_before_release",
        "raw_logs_and_evidence_stay_traceable_to_run_record",
    ] {
        assert!(DESIGN_ASSUMPTIONS.contains(&assumption));
    }

    assert_eq!(STREAM_COUNT, STREAM_NAMES.len());
    assert_eq!(DRIFT_EMULATOR_CHANNELS, STREAM_COUNT);
    assert_eq!(REFERENCE_PPS_CHECK_JACKS, STREAM_COUNT + 1);
    assert_eq!(RESYNC_OUTPUT_PORTS, STREAM_COUNT);
    assert_eq!(RESYNC_MONITOR_PORTS, STREAM_COUNT);
    assert_eq!(SKEW_WITNESS_LANES, STREAM_COUNT);
    assert_eq!(SKEW_WITNESS_SLOTS, SKEW_WITNESS_LANES * SKEW_TICK_STATIONS);
    assert_eq!(EVENT_TOKEN_TYPES.len(), EVENT_TOKEN_ROWS * EVENT_TOKEN_COLS);
    assert_eq!(DECISION_GATES, GATE_NAMES.len());
    assert_eq!(GATE_TOKEN_SLOTS_PER_GATE, STREAM_COUNT);
    assert_eq!(RAW_CLOCK_LOG_LANDS, STREAM_COUNT);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);

    assert!(REFERENCE_DOCK_D + 18.0 < REFERENCE_X / REFERENCE_CLOCK_DOCKS as f64);
    assert!(DRIFT_DIAL_D + 8.0 < DRIFT_CHANNEL_PITCH);
    assert!(FANOUT_PORT_D + 14.0 < FANOUT_PORT_PITCH);
    assert!(SKEW_TICK_PITCH * (SKEW_TICK_STATIONS as f64 - 1.0) < WITNESS_X - 80.0);
    assert!(SKEW_LANE_PITCH < LOGGER_LANE_PITCH + 4.0);
    assert!(CLOCK_FACE_D + 90.0 < EVIDENCE_X);
    assert!(LOGGER_LIFT_CLEARANCE_Z > LOGGER_Z + 80.0);

    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds clock drift resync deck",
            footprint.name
        );
    }

    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(
                !a.overlaps_with_clearance(*b, 10.0),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }
}

fn base_challenge_deck() -> Part {
    let deck = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wipe_recess = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_recessed_wipe_pan",
        STATION_X - 120.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.5);
    let front_log_sump = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_front_evidence_sump",
        STATION_X - 230.0,
        112.0,
        8.0,
    )
    .translate(0.0, -300.0, BASE_Z / 2.0 - 4.0);

    deck - wipe_recess - front_log_sump - deck_module_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + row_divider_rails()
        + time_direction_ribs()
        + robot_datum_targets()
}

fn deck_module_sockets() -> Part {
    let mut sockets =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_module_sockets");
    for footprint in station_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{}_registration_socket",
                    footprint.name
                ),
                footprint.x + 10.0,
                footprint.y + 10.0,
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
    let mut holes =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_mounting_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_sensor_clock_drift_resync_challenge_station_m6_mount_{i}"),
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
        "closed_sensor_clock_drift_resync_challenge_station_front_low_robot_lip",
        STATION_X - 190.0,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_rear_clock_service_rim",
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
        "closed_sensor_clock_drift_resync_challenge_station_left_cable_rim",
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
        "closed_sensor_clock_drift_resync_challenge_station_right_data_service_rim",
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
    let top_mid = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_clock_to_witness_divider",
        STATION_X - 220.0,
        10.0,
        28.0,
    )
    .translate(0.0, 125.0, BASE_Z / 2.0 + 14.0);
    let mid_bottom = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_witness_to_record_divider",
        STATION_X - 230.0,
        10.0,
        28.0,
    )
    .translate(0.0, -145.0, BASE_Z / 2.0 + 14.0);
    let left_mid = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_logger_to_skew_divider",
        10.0,
        232.0,
        26.0,
    )
    .translate(-305.0, 0.0, BASE_Z / 2.0 + 13.0);
    let right_mid = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_skew_to_evidence_divider",
        10.0,
        232.0,
        26.0,
    )
    .translate(112.0, 0.0, BASE_Z / 2.0 + 13.0);
    top_mid + mid_bottom + left_mid + right_mid
}

fn time_direction_ribs() -> Part {
    let mut ribs =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_time_direction_ribs");
    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, 150.0);
        ribs = ribs
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_time_arrow_stem_{stream}"
                ),
                82.0,
                5.0,
                6.0,
            )
            .translate(x, 126.0, BASE_Z / 2.0 + 3.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_time_arrow_head_{stream}"
                ),
                16.0,
                16.0,
                6.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 48.0, 126.0, BASE_Z / 2.0 + 3.0);
    }
    ribs
}

fn robot_datum_targets() -> Part {
    let mut targets =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_robot_datum_targets");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 86.0), STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 86.0, STATION_Y / 2.0 - 86.0),
        (-(STATION_X / 2.0 - 86.0), -(STATION_Y / 2.0 - 86.0)),
        (STATION_X / 2.0 - 86.0, -(STATION_Y / 2.0 - 86.0)),
    ]
    .into_iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_sensor_clock_drift_resync_challenge_station_robot_datum_ring_{i}"),
            17.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("closed_sensor_clock_drift_resync_challenge_station_robot_datum_bore_{i}"),
            3.4,
            8.0,
            24,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        targets = targets + (ring - bore);
    }
    targets
}

fn reference_clock_dock_bank() -> Part {
    let bank = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_reference_clock_bank_body",
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_reference_clock_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_reference_clock_features");

    for source in 0..REFERENCE_CLOCK_DOCKS {
        let x = centered_index(source, REFERENCE_CLOCK_DOCKS, 86.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_source_{source}_dock_recess"
                ),
                REFERENCE_DOCK_D / 2.0,
                REFERENCE_Z - 8.0,
                44,
            )
            .translate(x, 42.0, 4.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_source_{source}_certificate_land"
                ),
                68.0,
                16.0,
                5.0,
            )
            .translate(x, 78.0, REFERENCE_Z / 2.0 + 2.5)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_source_{source}_anti_swap_key"
                ),
                10.0,
                28.0,
                12.0,
            )
            .translate(x + 27.0, 42.0, REFERENCE_Z / 2.0 + 6.0);
    }

    for jack in 0..REFERENCE_PPS_CHECK_JACKS {
        let x = centered_index(jack, REFERENCE_PPS_CHECK_JACKS, 30.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_pps_check_jack_{jack}"
                ),
                REFERENCE_PPS_D / 2.0,
                REFERENCE_Z + 8.0,
                24,
            )
            .translate(x, -58.0, 0.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_pps_jack_{jack}_label_land"
                ),
                23.0,
                12.0,
                4.0,
            )
            .translate(x, -80.0, REFERENCE_Z / 2.0 + 2.0);
    }

    bank - cuts + features + reference_guard_ribs()
}

fn reference_guard_ribs() -> Part {
    let top = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_reference_top_guard_rib",
        REFERENCE_X - 30.0,
        8.0,
        14.0,
    )
    .translate(0.0, REFERENCE_Y / 2.0 - 20.0, REFERENCE_Z / 2.0 + 7.0);
    let bottom = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_reference_bottom_guard_rib",
        REFERENCE_X - 30.0,
        8.0,
        14.0,
    )
    .translate(0.0, -REFERENCE_Y / 2.0 + 20.0, REFERENCE_Z / 2.0 + 7.0);
    top + bottom
}

fn drift_injection_clock_emulator_panel() -> Part {
    let panel = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_drift_emulator_panel_body",
        DRIFT_X,
        DRIFT_Y,
        DRIFT_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_drift_emulator_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_drift_emulator_features");

    for stream in 0..DRIFT_EMULATOR_CHANNELS {
        let x = centered_index(stream, DRIFT_EMULATOR_CHANNELS, DRIFT_CHANNEL_PITCH);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_drift_dial_recess"
                ),
                DRIFT_DIAL_D / 2.0,
                DRIFT_Z - 8.0,
                36,
            )
            .translate(x, 40.0, 4.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_drift_sign_selector_slot"
                ),
                22.0,
                7.0,
                8.0,
            )
            .translate(x, -3.0, DRIFT_Z / 2.0 - 3.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_drift_value_land"
                ),
                29.0,
                14.0,
                4.0,
            )
            .translate(x, 72.0, DRIFT_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_drift_state_flag"
                ),
                6.0,
                44.0,
                6.0,
            )
            .translate(x, -42.0, DRIFT_Z / 2.0 + 3.0);
    }

    for stop in 0..DRIFT_RANGE_STOPS {
        let x = centered_index(stop, DRIFT_RANGE_STOPS, 48.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_drift_range_stop_{stop}"
                ),
                10.0,
                24.0,
                12.0,
            )
            .translate(x, -76.0, DRIFT_Z / 2.0 + 6.0);
    }

    panel - cuts + features
}

fn sensor_logger_challenge_lanes() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_sensor_logger_lane_body",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_logger_lane_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_logger_lane_features");

    for stream in 0..STREAM_COUNT {
        let y = centered_index(stream, STREAM_COUNT, LOGGER_LANE_PITCH);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_logger_pocket"
                ),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                24.0,
            )
            .translate(-88.0, y, LOGGER_Z / 2.0 - 10.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_local_clock_reset_pin"
                ),
                LOGGER_RESET_PIN_D / 2.0,
                LOGGER_Z + 8.0,
                18,
            )
            .translate(32.0, y, 0.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_sensor_signal_input_bore"
                ),
                TUBE_BORE_D / 2.0,
                LOGGER_X + 10.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(LOGGER_X / 2.0 - 12.0, y, 1.0);
        features = features
            + centered_cube(
                format!("closed_sensor_clock_drift_resync_challenge_station_{label}_lane_rib"),
                LOGGER_X - 34.0,
                3.5,
                5.0,
            )
            .translate(0.0, y + LOGGER_LANE_PITCH / 2.0 - 3.0, LOGGER_Z / 2.0 + 2.5)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_logger_id_land"
                ),
                52.0,
                13.0,
                4.0,
            )
            .translate(-150.0, y, LOGGER_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_reset_state_land"
                ),
                40.0,
                13.0,
                4.0,
            )
            .translate(86.0, y, LOGGER_Z / 2.0 + 2.0);
    }

    body - cuts + features
}

fn resync_pulse_fanout_manifold() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_resync_fanout_body",
        FANOUT_X,
        FANOUT_Y,
        FANOUT_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_resync_fanout_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_resync_fanout_features");

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, FANOUT_PORT_PITCH);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_resync_output_port"
                ),
                FANOUT_PORT_D / 2.0,
                FANOUT_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, FANOUT_Y / 2.0 - 18.0, 0.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_monitor_return_port"
                ),
                FANOUT_PORT_D / 2.0,
                FANOUT_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -FANOUT_Y / 2.0 + 18.0, 0.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_isolation_valve_pocket"
                ),
                12.0,
                FANOUT_Z + 8.0,
                30,
            )
            .translate(x, 0.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_sensor_clock_drift_resync_challenge_station_{label}_fanout_rib"),
                4.0,
                FANOUT_Y - 50.0,
                5.0,
            )
            .translate(x, 0.0, FANOUT_Z / 2.0 + 2.5)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_fanout_status_land"
                ),
                30.0,
                15.0,
                4.0,
            )
            .translate(x, 0.0, FANOUT_Z / 2.0 + 2.0);
    }

    for input in 0..RESYNC_INPUT_PORTS {
        let x = centered_index(input, RESYNC_INPUT_PORTS, 54.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                "closed_sensor_clock_drift_resync_challenge_station_reference_resync_input_{input}"
            ),
                FANOUT_PORT_D / 2.0,
                FANOUT_Z + 8.0,
                24,
            )
            .translate(x, 64.0, 0.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_reference_resync_input_{input}_tag"
                ),
                38.0,
                14.0,
                4.0,
            )
            .translate(x, 38.0, FANOUT_Z / 2.0 + 2.0);
    }

    body - cuts + features + fanout_common_bus_ribs()
}

fn fanout_common_bus_ribs() -> Part {
    let upper = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_fanout_upper_common_bus",
        FANOUT_X - 56.0,
        7.0,
        6.0,
    )
    .translate(0.0, 48.0, FANOUT_Z / 2.0 + 3.0);
    let lower = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_fanout_lower_return_bus",
        FANOUT_X - 56.0,
        7.0,
        6.0,
    )
    .translate(0.0, -48.0, FANOUT_Z / 2.0 + 3.0);
    upper + lower
}

fn timestamp_skew_witness_ruler() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_skew_witness_ruler_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_skew_witness_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_skew_witness_features");

    for stream in 0..SKEW_WITNESS_LANES {
        let y = centered_index(stream, SKEW_WITNESS_LANES, SKEW_LANE_PITCH);
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!("closed_sensor_clock_drift_resync_challenge_station_{label}_skew_lane"),
                WITNESS_X - 54.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, WITNESS_Z / 2.0 + 2.5);

        for tick in 0..SKEW_TICK_STATIONS {
            let x = centered_index(tick, SKEW_TICK_STATIONS, SKEW_TICK_PITCH);
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_sensor_clock_drift_resync_challenge_station_{label}_skew_tick_{tick}_coupon_slot"
                    ),
                    16.0,
                    10.0,
                    8.0,
                )
                .translate(x, y, WITNESS_Z / 2.0 - 3.0);
            features = features
                + centered_cube(
                    format!(
                        "closed_sensor_clock_drift_resync_challenge_station_{label}_skew_tick_{tick}_index_land"
                    ),
                    10.0,
                    16.0,
                    4.0,
                )
                .translate(x, y + 8.0, WITNESS_Z / 2.0 + 2.0);
        }
    }

    body - cuts + features
}

fn temperature_voltage_drift_stress_pocket() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_temperature_voltage_stress_body",
        STRESS_X,
        STRESS_Y,
        STRESS_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_stress_pocket_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_stress_pocket_features");

    for well in 0..TEMPERATURE_WELLS {
        let x = centered_index(well, TEMPERATURE_WELLS, 48.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                "closed_sensor_clock_drift_resync_challenge_station_temperature_step_well_{well}"
            ),
                TEMPERATURE_WELL_D / 2.0,
                STRESS_Z - 10.0,
                36,
            )
            .translate(x, 48.0, 5.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_temperature_step_{well}_land"
                ),
                40.0,
                14.0,
                4.0,
            )
            .translate(x, 76.0, STRESS_Z / 2.0 + 2.0);
    }

    for slot in 0..VOLTAGE_RAIL_SLOTS {
        let x = centered_index(slot, VOLTAGE_RAIL_SLOTS, 50.0);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_voltage_rail_slot_{slot}"
                ),
                VOLTAGE_SLOT_X,
                VOLTAGE_SLOT_Y,
                12.0,
            )
            .translate(x, -12.0, STRESS_Z / 2.0 - 4.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_voltage_rail_{slot}_guard"
                ),
                VOLTAGE_SLOT_X + 10.0,
                4.0,
                6.0,
            )
            .translate(x, -34.0, STRESS_Z / 2.0 + 3.0);
    }

    for well in 0..STRESS_SENSOR_WELLS {
        let x = centered_index(well, STRESS_SENSOR_WELLS, 58.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_stress_monitor_probe_well_{well}"
                ),
                7.0,
                STRESS_Z + 8.0,
                24,
            )
            .translate(x, -66.0, 0.0);
    }

    body - cuts + features + stress_thermal_ribs()
}

fn stress_thermal_ribs() -> Part {
    let mut ribs =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_stress_thermal_ribs");
    for rib in 0..6 {
        let x = centered_index(rib, 6, 38.0);
        ribs = ribs
            + centered_cube(
                format!(
                "closed_sensor_clock_drift_resync_challenge_station_stress_thermal_mass_rib_{rib}"
            ),
                5.0,
                STRESS_Y - 34.0,
                6.0,
            )
            .translate(x, 0.0, STRESS_Z / 2.0 + 3.0);
    }
    ribs
}

fn event_replay_token_rail() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_event_replay_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_event_token_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_event_token_features");

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
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_event_token_well"
                ),
                EVENT_TOKEN_D / 2.0,
                TOKEN_Z - 6.0,
                36,
            )
            .translate(x, y, 4.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_event_label_land"
                ),
                44.0,
                13.0,
                4.0,
            )
            .translate(x, y - 23.0, TOKEN_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_event_anti_swap_key"
                ),
                6.0,
                16.0,
                7.0,
            )
            .translate(x + 15.0, y, TOKEN_Z / 2.0 + 3.5);
    }

    body - cuts + features + event_row_ridges()
}

fn event_row_ridges() -> Part {
    let mut ridges =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_event_row_ridges");
    for row in 0..=EVENT_TOKEN_ROWS {
        let y = -((EVENT_TOKEN_ROWS as f64) * EVENT_TOKEN_PITCH_Y) / 2.0
            + row as f64 * EVENT_TOKEN_PITCH_Y;
        ridges = ridges
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_event_row_separator_{row}"
                ),
                TOKEN_X - 36.0,
                5.0,
                6.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 3.0);
    }
    ridges
}

fn camera_evidence_clock_target() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_camera_evidence_target_body",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_camera_evidence_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_camera_evidence_features");

    cuts = cuts
        + centered_cylinder(
            "closed_sensor_clock_drift_resync_challenge_station_clock_face_recess",
            CLOCK_FACE_D / 2.0,
            9.0,
            64,
        )
        .translate(-118.0, 0.0, EVIDENCE_Z / 2.0 - 3.5);
    features = features
        + centered_cylinder(
            "closed_sensor_clock_drift_resync_challenge_station_clock_face_outer_index_ring",
            CLOCK_FACE_D / 2.0 + 6.0,
            4.0,
            64,
        )
        .translate(-118.0, 0.0, EVIDENCE_Z / 2.0 + 2.0)
        - centered_cylinder(
            "closed_sensor_clock_drift_resync_challenge_station_clock_face_inner_opening",
            CLOCK_FACE_D / 2.0 - 2.0,
            6.0,
            64,
        )
        .translate(-118.0, 0.0, EVIDENCE_Z / 2.0 + 2.0);

    for tick in 0..CLOCK_TICK_MARKS {
        let angle = tick as f64 * 30.0;
        features = features
            + centered_cube(
                format!("closed_sensor_clock_drift_resync_challenge_station_clock_tick_{tick}"),
                5.0,
                22.0,
                5.0,
            )
            .translate(0.0, CLOCK_FACE_D / 2.0 - 13.0, EVIDENCE_Z / 2.0 + 2.5)
            .rotate(0.0, 0.0, angle)
            .translate(-118.0, 0.0, 0.0);
    }

    for fiducial in 0..CAMERA_FIDUCIALS {
        let (x, y) = match fiducial {
            0 => (-EVIDENCE_X / 2.0 + 36.0, -EVIDENCE_Y / 2.0 + 34.0),
            1 => (EVIDENCE_X / 2.0 - 36.0, -EVIDENCE_Y / 2.0 + 34.0),
            2 => (-EVIDENCE_X / 2.0 + 36.0, EVIDENCE_Y / 2.0 - 34.0),
            _ => (EVIDENCE_X / 2.0 - 36.0, EVIDENCE_Y / 2.0 - 34.0),
        };
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_camera_fiducial_bore_{fiducial}"
                ),
                8.0,
                EVIDENCE_Z + 8.0,
                32,
            )
            .translate(x, y, 0.0);
        features = features
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_camera_fiducial_ring_{fiducial}"
                ),
                15.0,
                4.0,
                36,
            )
            .translate(x, y, EVIDENCE_Z / 2.0 + 2.0);
    }

    for window in 0..LED_EVIDENCE_WINDOWS {
        let x = centered_index(window % 4, 4, 52.0) + 110.0;
        let y = centered_index(window / 4, 2, 54.0);
        let label = stream_label(window);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_led_evidence_window"
                ),
                32.0,
                18.0,
                8.0,
            )
            .translate(x, y, EVIDENCE_Z / 2.0 - 3.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_led_status_land"
                ),
                34.0,
                6.0,
                4.0,
            )
            .translate(x, y - 18.0, EVIDENCE_Z / 2.0 + 2.0);
    }

    body - cuts + features
}

fn resync_quarantine_release_gates() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_decision_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_decision_gate_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_decision_gate_features");

    for (gate, label) in GATE_NAMES.iter().enumerate() {
        let x = centered_index(gate, DECISION_GATES, GATE_PITCH_X);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_decision_lane_divider"
                ),
                GATE_PITCH_X - 16.0,
                7.0,
                8.0 + gate as f64 * 8.0,
            )
            .translate(x, 58.0, GATE_Z / 2.0 + 4.0 + gate as f64 * 4.0)
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_decision_label_land"
                ),
                70.0,
                17.0,
                4.0,
            )
            .translate(x, 76.0, GATE_Z / 2.0 + 2.0);

        for stream in 0..GATE_TOKEN_SLOTS_PER_GATE {
            let y = centered_index(stream, GATE_TOKEN_SLOTS_PER_GATE, 16.0) - 12.0;
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_sensor_clock_drift_resync_challenge_station_{label}_{}_decision_token_slot",
                        stream_label(stream)
                    ),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    10.0,
                )
                .translate(x, y, GATE_Z / 2.0 - 4.0);
        }
    }

    body - cuts + features
}

fn data_custody_evidence_lands() -> Part {
    let body = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_data_custody_land_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut cuts = Part::empty("closed_sensor_clock_drift_resync_challenge_station_custody_cuts");
    let mut features =
        Part::empty("closed_sensor_clock_drift_resync_challenge_station_custody_features");

    for land in 0..RUN_RECORD_LANDS {
        let x = centered_index(land, RUN_RECORD_LANDS, 70.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_run_record_land_{land}"
                ),
                56.0,
                24.0,
                5.0,
            )
            .translate(x, 52.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for land in 0..RAW_CLOCK_LOG_LANDS {
        let x = centered_index(land % 4, 4, 78.0);
        let y = centered_index(land / 4, 2, 42.0) - 24.0;
        let label = stream_label(land);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_{label}_raw_clock_log_land"
                ),
                62.0,
                20.0,
                5.0,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.5);
    }

    for seal in 0..EVIDENCE_SEAL_WELLS {
        let x = centered_index(seal, EVIDENCE_SEAL_WELLS, 72.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_evidence_seal_well_{seal}"
                ),
                10.0,
                CUSTODY_Z + 8.0,
                30,
            )
            .translate(x, -70.0, 0.0);
    }

    for slot in 0..AUDIT_EXPORT_SLOTS {
        let x = centered_index(slot, AUDIT_EXPORT_SLOTS, 74.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_clock_drift_resync_challenge_station_audit_export_slot_{slot}"
                ),
                48.0,
                9.0,
                5.0,
            )
            .translate(x, -48.0, CUSTODY_Z / 2.0 + 2.5);
    }

    body - cuts + features
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_front_robot_sweep_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -FRONT_ROBOT_SWEEP_CLEARANCE, BASE_Z + KEEP_OUT_Z / 2.0);
    let rear_clock = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_rear_clock_service_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, REAR_CLOCK_SERVICE_CLEARANCE, BASE_Z + KEEP_OUT_Z / 2.0);
    let left_cable = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_left_cable_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -LEFT_CABLE_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let right_data = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_right_data_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(RIGHT_DATA_SERVICE_CLEARANCE, 0.0, BASE_Z + KEEP_OUT_Z / 2.0);
    let logger_lift = centered_cube(
        "closed_sensor_clock_drift_resync_challenge_station_logger_lift_clearance_gauge",
        LOGGER_X,
        LOGGER_Y,
        8.0,
    )
    .translate(LOGGER_POS.0, LOGGER_POS.1, BASE_Z + LOGGER_LIFT_CLEARANCE_Z);

    front_robot + rear_clock + left_cable + right_data + logger_lift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_scoped_and_complete() {
        assert_design_constraints();
        for output in OUTPUTS {
            assert!(output.starts_with(OUTPUT_PREFIX));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn all_streams_have_drift_resync_and_witness_capacity() {
        assert_eq!(STREAM_COUNT, 8);
        assert_eq!(DRIFT_EMULATOR_CHANNELS, STREAM_COUNT);
        assert_eq!(RESYNC_OUTPUT_PORTS, STREAM_COUNT);
        assert_eq!(RESYNC_MONITOR_PORTS, STREAM_COUNT);
        assert_eq!(SKEW_WITNESS_SLOTS, STREAM_COUNT * SKEW_TICK_STATIONS);
        assert_eq!(GATE_TOKEN_SLOTS_PER_GATE, STREAM_COUNT);
    }

    #[test]
    fn validation_scope_is_explicitly_limited() {
        for limitation in [
            "no_clock_sync_algorithm",
            "no_acceptance_thresholds",
            "no_sterile_barrier_claim",
            "no_biological_performance_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }
    }
}
