use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed alarm-response false-positive fault-injection station.
//
// Intent:
// - Provide a contained, cell-free validation fixture for proving that alarm
//   response logic does not falsely divert, pause, or quarantine a culture lane
//   when injected sensor faults disagree with independent truth references.
// - Keep the no-fault truth loop, fault-injector cassette, sensor arbitration
//   panel, mock quarantine diverter path, event/timestamp docks, challenge
//   randomization matrix, operator-free reset interlock, custody lands, and
//   release/hold/reject gates mechanically visible.
// - Represent state labels as CSG token wells, raised rails, keyed pockets,
//   comparator windows, and run-record lands so the STL exports remain useful
//   without external decal files.
//
// Reproducibility assumptions encoded in the geometry:
// - False-positive control needs an independent ground-truth path. The station
//   physically separates no-fault reference sensors from injected fault lanes.
// - Alarm review is event based: generated, acknowledged, cleared/suppressed,
//   quarantined, and released states must be traceable to a timestamped run
//   record rather than handled through an untracked manual override.
// - Closed automated culture work should validate nuisance trips as explicitly
//   as true alarms because unnecessary media diversion, stop/start flow, and
//   reset actions can create biological variability across chip positions.
//
// This is architecture CAD only. It is not a validated sterile barrier,
// controller, alarm algorithm, acceptance criterion, software implementation,
// electrical design, pressure safety device, or cell-culture protocol.

const OUTPUT_PREFIX: &str = "output/closed_alarm_response_false_positive_fault_injection_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_alarm_response_false_positive_fault_injection_station_base_validation_deck.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_no_fault_truth_reference_loop.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_fault_injector_signal_cassette.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_sensor_arbitration_panel.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_event_recorder_timestamp_docks.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_quarantine_diverter_mock_path.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_alarm_disposition_token_rail.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_reset_interlock_sequence_panel.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_challenge_randomization_coupon_matrix.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_run_record_custody_lands.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_release_hold_reject_gates.stl",
    "output/closed_alarm_response_false_positive_fault_injection_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "no_fault_truth_reference_loop",
    "fault_injector_signal_cassette",
    "sensor_arbitration_panel",
    "event_recorder_timestamp_docks",
    "quarantine_diverter_mock_path",
    "alarm_disposition_token_rail",
    "reset_interlock_sequence_panel",
    "challenge_randomization_coupon_matrix",
    "run_record_custody_lands",
    "release_hold_reject_gates",
    "robot_service_keepouts",
];

const REPRODUCIBILITY_CONTROLS: [&str; 8] = [
    "independent_no_fault_truth_reference",
    "cell_free_fault_injection",
    "primary_reference_sensor_pairing",
    "randomized_challenge_coupons",
    "timestamped_alarm_lifecycle_tokens",
    "mock_quarantine_without_culture_exposure",
    "operator_free_reset_sequence",
    "release_hold_reject_custody",
];

const LIMITATIONS: [&str; 7] = [
    "architecture_cad_only",
    "no_alarm_algorithm",
    "no_software_validation_claim",
    "no_pressure_safety_claim",
    "no_sterile_barrier_claim",
    "no_acceptance_limits",
    "no_cell_culture_protocol",
];

const ALARM_CHANNELS: usize = 6;
const ALARM_CHANNEL_NAMES: [&str; ALARM_CHANNELS] =
    ["pressure", "flow", "bubble", "ph", "do", "temperature"];
const LIFECYCLE_STATES: [&str; 5] = ["normal", "injected", "suppressed", "escalated", "cleared"];
const DISPOSITION_STATES: [&str; 3] = ["release", "hold", "reject"];

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.6;

const TRUTH_POS: (f64, f64) = (-565.0, 260.0);
const TRUTH_X: f64 = 320.0;
const TRUTH_Y: f64 = 230.0;
const TRUTH_Z: f64 = 52.0;
const TRUTH_SENSOR_PAIRS: usize = ALARM_CHANNELS;
const TRUTH_LOOP_LANES: usize = ALARM_CHANNELS;
const TRUTH_LANE_PITCH_X: f64 = 42.0;
const TRUTH_PORT_D: f64 = 9.2;
const TRUTH_REFERENCE_CELLS: usize = 3;

const INJECTOR_POS: (f64, f64) = (-170.0, 260.0);
const INJECTOR_X: f64 = 390.0;
const INJECTOR_Y: f64 = 230.0;
const INJECTOR_Z: f64 = 58.0;
const INJECTION_LANES: usize = ALARM_CHANNELS;
const INJECTOR_PITCH_X: f64 = 54.0;
const INJECTOR_KEY_COUNT: usize = INJECTION_LANES;
const INJECTOR_PORT_D: f64 = 10.0;
const FAULT_LEVELS: usize = 3;

const ARBITER_POS: (f64, f64) = (250.0, 260.0);
const ARBITER_X: f64 = 390.0;
const ARBITER_Y: f64 = 230.0;
const ARBITER_Z: f64 = 50.0;
const PRIMARY_SENSOR_DOCKS: usize = ALARM_CHANNELS;
const REFERENCE_SENSOR_DOCKS: usize = ALARM_CHANNELS;
const COMPARATOR_WINDOWS: usize = ALARM_CHANNELS;
const ARBITER_PITCH_X: f64 = 54.0;

const LOGGER_POS: (f64, f64) = (590.0, 260.0);
const LOGGER_X: f64 = 270.0;
const LOGGER_Y: f64 = 230.0;
const LOGGER_Z: f64 = 46.0;
const EVENT_LOGGER_DOCKS: usize = 4;
const TIMEBASE_JACKS: usize = ALARM_CHANNELS + 1;
const LOGGER_DOCK_X: f64 = 52.0;
const LOGGER_DOCK_Y: f64 = 54.0;
const TIMEBASE_JACK_D: f64 = 8.4;

const DIVERTER_POS: (f64, f64) = (-465.0, 10.0);
const DIVERTER_X: f64 = 430.0;
const DIVERTER_Y: f64 = 210.0;
const DIVERTER_Z: f64 = 58.0;
const MOCK_VALVES: usize = ALARM_CHANNELS;
const MOCK_BAG_NESTS: usize = 4;
const DIVERTER_PORT_D: f64 = 8.0;
const DIVERTER_PITCH_X: f64 = 58.0;

const TOKEN_POS: (f64, f64) = (0.0, 10.0);
const TOKEN_X: f64 = 380.0;
const TOKEN_Y: f64 = 210.0;
const TOKEN_Z: f64 = 32.0;
const TOKEN_WELLS: usize = ALARM_CHANNELS * LIFECYCLE_STATES.len();
const TOKEN_D: f64 = 19.0;
const TOKEN_PITCH_X: f64 = 52.0;
const TOKEN_PITCH_Y: f64 = 36.0;

const RESET_POS: (f64, f64) = (430.0, 10.0);
const RESET_X: f64 = 340.0;
const RESET_Y: f64 = 210.0;
const RESET_Z: f64 = 48.0;
const RESET_SEQUENCE_STEPS: usize = 5;
const RESET_INTERLOCK_PINS: usize = ALARM_CHANNELS + 2;
const RESET_PIN_D: f64 = 10.0;
const RESET_STEP_PITCH_X: f64 = 58.0;

const CHALLENGE_POS: (f64, f64) = (-500.0, -285.0);
const CHALLENGE_X: f64 = 360.0;
const CHALLENGE_Y: f64 = 210.0;
const CHALLENGE_Z: f64 = 34.0;
const CHALLENGE_ROWS: usize = 3;
const CHALLENGE_COLS: usize = ALARM_CHANNELS;
const CHALLENGE_COUPONS: usize = CHALLENGE_ROWS * CHALLENGE_COLS;
const CHALLENGE_COUPON_X: f64 = 38.0;
const CHALLENGE_COUPON_Y: f64 = 36.0;
const CHALLENGE_PITCH_X: f64 = 48.0;
const CHALLENGE_PITCH_Y: f64 = 54.0;

const CUSTODY_POS: (f64, f64) = (-80.0, -285.0);
const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 210.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 6;
const RAW_EVENT_LANDS: usize = ALARM_CHANNELS;
const CUSTODY_SEAL_WELLS: usize = 4;
const CUSTODY_LAND_X: f64 = 66.0;
const CUSTODY_LAND_Y: f64 = 24.0;

const GATES_POS: (f64, f64) = (410.0, -285.0);
const GATES_X: f64 = 320.0;
const GATES_Y: f64 = 210.0;
const GATES_Z: f64 = 42.0;
const GATE_TOKEN_SLOTS_PER_STATE: usize = ALARM_CHANNELS;
const GATE_PITCH_X: f64 = 92.0;
const GATE_SLOT_X: f64 = 38.0;
const GATE_SLOT_Y: f64 = 22.0;

const KEEP_OUT_Z: f64 = 7.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 410.0;
const REAR_SERVICE_CLEARANCE: f64 = 150.0;
const LEFT_BAG_SERVICE_CLEARANCE: f64 = 160.0;
const RIGHT_LOGGER_SERVICE_CLEARANCE: f64 = 150.0;
const TOP_MODULE_LIFT_CLEARANCE: f64 = 132.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 10.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 10.0
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

    let deck = base_validation_deck();
    export(OUTPUTS[0], &deck);

    let truth = no_fault_truth_reference_loop();
    export(OUTPUTS[1], &truth);

    let injector = fault_injector_signal_cassette();
    export(OUTPUTS[2], &injector);

    let arbiter = sensor_arbitration_panel();
    export(OUTPUTS[3], &arbiter);

    let loggers = event_recorder_timestamp_docks();
    export(OUTPUTS[4], &loggers);

    let diverter = quarantine_diverter_mock_path();
    export(OUTPUTS[5], &diverter);

    let tokens = alarm_disposition_token_rail();
    export(OUTPUTS[6], &tokens);

    let reset = reset_interlock_sequence_panel();
    export(OUTPUTS[7], &reset);

    let challenges = challenge_randomization_coupon_matrix();
    export(OUTPUTS[8], &challenges);

    let custody = run_record_custody_lands();
    export(OUTPUTS[9], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[10], &gates);

    let keepouts = robot_service_keepouts();

    let assembly = deck
        + truth.translate(TRUTH_POS.0, TRUTH_POS.1, on_deck_z(TRUTH_Z))
        + injector.translate(INJECTOR_POS.0, INJECTOR_POS.1, on_deck_z(INJECTOR_Z))
        + arbiter.translate(ARBITER_POS.0, ARBITER_POS.1, on_deck_z(ARBITER_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_deck_z(LOGGER_Z))
        + diverter.translate(DIVERTER_POS.0, DIVERTER_POS.1, on_deck_z(DIVERTER_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + reset.translate(RESET_POS.0, RESET_POS.1, on_deck_z(RESET_Z))
        + challenges.translate(CHALLENGE_POS.0, CHALLENGE_POS.1, on_deck_z(CHALLENGE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + gates.translate(GATES_POS.0, GATES_POS.1, on_deck_z(GATES_Z))
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed alarm-response false-positive fault-injection station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained validation deck"
    );
    println!(
        "  Alarm channels:            {ALARM_CHANNELS} channels ({})",
        ALARM_CHANNEL_NAMES.join(", ")
    );
    println!(
        "  Ground truth controls:     {TRUTH_SENSOR_PAIRS} primary/reference pairs, {TRUTH_REFERENCE_CELLS} no-fault reference cells, {TRUTH_LOOP_LANES} truth lanes"
    );
    println!(
        "  Fault injection:           {INJECTION_LANES} keyed lanes, {FAULT_LEVELS} challenge levels, {CHALLENGE_COUPONS} randomized coupons"
    );
    println!(
        "  Alarm lifecycle evidence:  {TOKEN_WELLS} token wells across {} states, {EVENT_LOGGER_DOCKS} logger docks, {TIMEBASE_JACKS} timebase jacks",
        LIFECYCLE_STATES.len()
    );
    println!(
        "  Disposition controls:      {} gates with {GATE_TOKEN_SLOTS_PER_STATE} channel slots each, {RUN_RECORD_LANDS} run-record lands, {RAW_EVENT_LANDS} raw-event lands",
        DISPOSITION_STATES.join("/")
    );
    println!(
        "  Scope controls:            {} reproducibility controls, {} explicit non-scope notes, {KEEP_OUT_ZONE_COUNT} keepout gauges",
        REPRODUCIBILITY_CONTROLS.len(),
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

fn channel_label(index: usize) -> &'static str {
    ALARM_CHANNEL_NAMES.get(index).copied().unwrap_or("unknown")
}

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "no_fault_truth_reference_loop",
            center: TRUTH_POS,
            x: TRUTH_X,
            y: TRUTH_Y,
        },
        Footprint {
            name: "fault_injector_signal_cassette",
            center: INJECTOR_POS,
            x: INJECTOR_X,
            y: INJECTOR_Y,
        },
        Footprint {
            name: "sensor_arbitration_panel",
            center: ARBITER_POS,
            x: ARBITER_X,
            y: ARBITER_Y,
        },
        Footprint {
            name: "event_recorder_timestamp_docks",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Footprint {
            name: "quarantine_diverter_mock_path",
            center: DIVERTER_POS,
            x: DIVERTER_X,
            y: DIVERTER_Y,
        },
        Footprint {
            name: "alarm_disposition_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "reset_interlock_sequence_panel",
            center: RESET_POS,
            x: RESET_X,
            y: RESET_Y,
        },
        Footprint {
            name: "challenge_randomization_coupon_matrix",
            center: CHALLENGE_POS,
            x: CHALLENGE_X,
            y: CHALLENGE_Y,
        },
        Footprint {
            name: "run_record_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "release_hold_reject_gates",
            center: GATES_POS,
            x: GATES_X,
            y: GATES_Y,
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
        "no_fault_truth_reference_loop",
        "fault_injector_signal_cassette",
        "sensor_arbitration_panel",
        "event_recorder_timestamp_docks",
        "quarantine_diverter_mock_path",
        "alarm_disposition_token_rail",
        "reset_interlock_sequence_panel",
        "challenge_randomization_coupon_matrix",
        "run_record_custody_lands",
        "release_hold_reject_gates",
        "robot_service_keepouts",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for control in [
        "independent_no_fault_truth_reference",
        "cell_free_fault_injection",
        "primary_reference_sensor_pairing",
        "timestamped_alarm_lifecycle_tokens",
        "operator_free_reset_sequence",
        "release_hold_reject_custody",
    ] {
        assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
    }

    assert_eq!(TRUTH_SENSOR_PAIRS, ALARM_CHANNELS);
    assert_eq!(TRUTH_LOOP_LANES, ALARM_CHANNELS);
    assert_eq!(INJECTION_LANES, ALARM_CHANNELS);
    assert_eq!(INJECTOR_KEY_COUNT, INJECTION_LANES);
    assert_eq!(PRIMARY_SENSOR_DOCKS, ALARM_CHANNELS);
    assert_eq!(REFERENCE_SENSOR_DOCKS, ALARM_CHANNELS);
    assert_eq!(COMPARATOR_WINDOWS, ALARM_CHANNELS);
    assert_eq!(TIMEBASE_JACKS, ALARM_CHANNELS + 1);
    assert_eq!(MOCK_VALVES, ALARM_CHANNELS);
    assert_eq!(TOKEN_WELLS, ALARM_CHANNELS * LIFECYCLE_STATES.len());
    assert_eq!(RESET_INTERLOCK_PINS, ALARM_CHANNELS + 2);
    assert_eq!(CHALLENGE_COUPONS, CHALLENGE_ROWS * CHALLENGE_COLS);
    assert_eq!(CHALLENGE_COLS, ALARM_CHANNELS);
    assert_eq!(RAW_EVENT_LANDS, ALARM_CHANNELS);
    assert_eq!(GATE_TOKEN_SLOTS_PER_STATE, ALARM_CHANNELS);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);

    assert!(TRUTH_PORT_D + 12.0 < TRUTH_LANE_PITCH_X);
    assert!(INJECTOR_PORT_D + 18.0 < INJECTOR_PITCH_X);
    assert!(TIMEBASE_JACK_D + 12.0 < LOGGER_X / TIMEBASE_JACKS as f64);
    assert!(TOKEN_D + 10.0 < TOKEN_PITCH_X);
    assert!(TOKEN_D + 10.0 < TOKEN_PITCH_Y);
    assert!(CHALLENGE_COUPON_X + 8.0 <= CHALLENGE_PITCH_X);
    assert!(TOP_MODULE_LIFT_CLEARANCE > INJECTOR_Z + 60.0);

    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds validation deck",
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

fn base_validation_deck() -> Part {
    let deck = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_base_validation_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wipe_recess = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_recessed_wipe_pan",
        STATION_X - 128.0,
        STATION_Y - 128.0,
        7.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.5);
    let front_event_sump = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_front_event_sump",
        STATION_X - 250.0,
        112.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 118.0, BASE_Z / 2.0 - 4.0);

    deck - wipe_recess - front_event_sump - deck_module_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + row_divider_rails()
        + alarm_flow_direction_ticks()
        + robot_datum_targets()
}

fn deck_module_sockets() -> Part {
    let mut sockets =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_module_sockets");
    for footprint in station_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_alarm_response_false_positive_fault_injection_station_{}_registration_socket",
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
    let mut holes =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_mounting_holes");
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
                format!(
                    "closed_alarm_response_false_positive_fault_injection_station_m6_mount_{i}"
                ),
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
        "closed_alarm_response_false_positive_fault_injection_station_front_low_custody_lip",
        STATION_X - 210.0,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_rear_service_rim",
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
        "closed_alarm_response_false_positive_fault_injection_station_left_bag_service_rim",
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
        "closed_alarm_response_false_positive_fault_injection_station_right_logger_rim",
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
        "closed_alarm_response_false_positive_fault_injection_station_truth_to_alarm_divider",
        STATION_X - 230.0,
        10.0,
        28.0,
    )
    .translate(0.0, 140.0, BASE_Z / 2.0 + 14.0);
    let mid_to_custody = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_alarm_to_custody_divider",
        STATION_X - 240.0,
        10.0,
        28.0,
    )
    .translate(0.0, -145.0, BASE_Z / 2.0 + 14.0);
    let diverter_to_tokens = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_diverter_to_token_divider",
        10.0,
        220.0,
        26.0,
    )
    .translate(-235.0, 10.0, BASE_Z / 2.0 + 13.0);
    let tokens_to_reset = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_token_to_reset_divider",
        10.0,
        220.0,
        26.0,
    )
    .translate(220.0, 10.0, BASE_Z / 2.0 + 13.0);
    top_to_mid + mid_to_custody + diverter_to_tokens + tokens_to_reset
}

fn alarm_flow_direction_ticks() -> Part {
    let mut ticks =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_flow_ticks");
    for i in 0..ALARM_CHANNELS {
        let x = centered_index(i, ALARM_CHANNELS, 190.0);
        ticks = ticks
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_channel_{i}_truth_to_injector_arrow_stem"),
                96.0,
                5.0,
                6.0,
            )
            .translate(x, 143.0, BASE_Z / 2.0 + 3.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_channel_{i}_truth_to_injector_arrow_head"),
                18.0,
                18.0,
                6.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 55.0, 143.0, BASE_Z / 2.0 + 3.0);
    }
    ticks
}

fn robot_datum_targets() -> Part {
    let mut targets =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_datum_targets");
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
            format!(
                "closed_alarm_response_false_positive_fault_injection_station_robot_datum_ring_{i}"
            ),
            17.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!(
                "closed_alarm_response_false_positive_fault_injection_station_robot_datum_bore_{i}"
            ),
            3.4,
            8.0,
            24,
        )
        .translate(x, y, BASE_Z / 2.0 + 2.5);
        targets = targets + (ring - bore);
    }
    targets
}

fn no_fault_truth_reference_loop() -> Part {
    let body = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_truth_reference_loop_body",
        TRUTH_X,
        TRUTH_Y,
        TRUTH_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_truth_cuts");
    let mut features =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_truth_features");

    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, TRUTH_LANE_PITCH_X);
        let label = channel_label(channel);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_truth_inlet_port"),
                TRUTH_PORT_D / 2.0,
                TRUTH_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TRUTH_Y / 2.0 + 16.0, 0.0)
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_truth_outlet_port"),
                TRUTH_PORT_D / 2.0,
                TRUTH_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, TRUTH_Y / 2.0 - 16.0, 0.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_reference_sensor_pair_recess"),
                30.0,
                58.0,
                18.0,
            )
            .translate(x, 0.0, TRUTH_Z / 2.0 - 7.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_no_fault_lane_rib"),
                5.0,
                TRUTH_Y - 48.0,
                6.0,
            )
            .translate(x, 0.0, TRUTH_Z / 2.0 + 3.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_truth_label_land"),
                36.0,
                14.0,
                5.0,
            )
            .translate(x, -78.0, TRUTH_Z / 2.0 + 2.5);
    }

    for reference in 0..TRUTH_REFERENCE_CELLS {
        let x = centered_index(reference, TRUTH_REFERENCE_CELLS, 86.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_truth_reference_cell_{reference}_well"),
                17.0,
                TRUTH_Z - 8.0,
                36,
            )
            .translate(x, 74.0, 4.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_truth_reference_cell_{reference}_certificate_land"),
                58.0,
                15.0,
                5.0,
            )
            .translate(x, 100.0, TRUTH_Z / 2.0 + 2.5);
    }

    body - cuts + features + truth_loop_guard_rails()
}

fn truth_loop_guard_rails() -> Part {
    let upper = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_truth_loop_upper_guard_rail",
        TRUTH_X - 34.0,
        8.0,
        16.0,
    )
    .translate(0.0, TRUTH_Y / 2.0 - 22.0, TRUTH_Z / 2.0 + 8.0);
    let lower = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_truth_loop_lower_guard_rail",
        TRUTH_X - 34.0,
        8.0,
        16.0,
    )
    .translate(0.0, -TRUTH_Y / 2.0 + 22.0, TRUTH_Z / 2.0 + 8.0);
    upper + lower
}

fn fault_injector_signal_cassette() -> Part {
    let body = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_fault_injector_cassette_body",
        INJECTOR_X,
        INJECTOR_Y,
        INJECTOR_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_injector_cuts");
    let mut features = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_injector_features",
    );

    for channel in 0..INJECTION_LANES {
        let x = centered_index(channel, INJECTION_LANES, INJECTOR_PITCH_X);
        let label = channel_label(channel);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_fault_injection_port"),
                INJECTOR_PORT_D / 2.0,
                INJECTOR_Z + 8.0,
                24,
            )
            .translate(x, 48.0, 0.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_keyed_injector_recess"),
                34.0,
                52.0,
                18.0,
            )
            .translate(x, -4.0, INJECTOR_Z / 2.0 - 7.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_fault_signature_bar"),
                7.0,
                74.0,
                7.0,
            )
            .translate(x + 18.0, -8.0, INJECTOR_Z / 2.0 + 3.5)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_do_not_trip_label_land"),
                42.0,
                15.0,
                5.0,
            )
            .translate(x, -78.0, INJECTOR_Z / 2.0 + 2.5);

        for level in 0..FAULT_LEVELS {
            features = features
                + centered_cube(
                    format!("closed_alarm_response_false_positive_fault_injection_station_{label}_fault_level_{level}_witness_step"),
                    18.0,
                    9.0,
                    4.0 + level as f64 * 2.0,
                )
                .translate(
                    x - 17.0 + level as f64 * 17.0,
                    82.0,
                    INJECTOR_Z / 2.0 + 2.0 + level as f64,
                );
        }
    }

    body - cuts + features + injector_lockout_keys()
}

fn injector_lockout_keys() -> Part {
    let mut keys =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_lockout_keys");
    for key in 0..INJECTOR_KEY_COUNT {
        let x = centered_index(key, INJECTOR_KEY_COUNT, INJECTOR_PITCH_X);
        keys = keys
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_injector_lane_{key}_asymmetric_key"),
                9.0 + key as f64,
                22.0,
                13.0,
            )
            .translate(x - 18.0, 22.0, INJECTOR_Z / 2.0 + 6.5);
    }
    keys
}

fn sensor_arbitration_panel() -> Part {
    let panel = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_sensor_arbitration_panel_body",
        ARBITER_X,
        ARBITER_Y,
        ARBITER_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_arbiter_cuts");
    let mut features = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_arbiter_features",
    );

    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, ARBITER_PITCH_X);
        let label = channel_label(channel);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_primary_sensor_dock"),
                36.0,
                38.0,
                18.0,
            )
            .translate(x, 52.0, ARBITER_Z / 2.0 - 7.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_reference_sensor_dock"),
                36.0,
                38.0,
                18.0,
            )
            .translate(x, -52.0, ARBITER_Z / 2.0 - 7.0)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_comparator_window"),
                32.0,
                20.0,
                12.0,
            )
            .translate(x, 0.0, ARBITER_Z / 2.0 - 5.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_primary_to_reference_compare_rib"),
                5.0,
                78.0,
                5.0,
            )
            .translate(x, 0.0, ARBITER_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_mismatch_suppress_land"),
                40.0,
                14.0,
                5.0,
            )
            .translate(x, -92.0, ARBITER_Z / 2.0 + 2.5);
    }

    panel - cuts + features + arbitration_common_bus()
}

fn arbitration_common_bus() -> Part {
    let primary_bus = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_primary_sensor_bus_rib",
        ARBITER_X - 42.0,
        6.0,
        7.0,
    )
    .translate(0.0, 84.0, ARBITER_Z / 2.0 + 3.5);
    let reference_bus = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_reference_sensor_bus_rib",
        ARBITER_X - 42.0,
        6.0,
        7.0,
    )
    .translate(0.0, -84.0, ARBITER_Z / 2.0 + 3.5);
    let compare_bus = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_compare_bus_rib",
        ARBITER_X - 74.0,
        6.0,
        7.0,
    )
    .translate(0.0, 0.0, ARBITER_Z / 2.0 + 3.5);
    primary_bus + reference_bus + compare_bus
}

fn event_recorder_timestamp_docks() -> Part {
    let block = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_event_recorder_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_logger_cuts");
    let mut features =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_logger_features");

    for dock in 0..EVENT_LOGGER_DOCKS {
        let (x, y) = grid_position(dock, 2, 2, 72.0, 72.0);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_event_logger_dock_{dock}"),
                LOGGER_DOCK_X,
                LOGGER_DOCK_Y,
                18.0,
            )
            .translate(x, y + 16.0, LOGGER_Z / 2.0 - 7.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_event_logger_dock_{dock}_custody_tab"),
                42.0,
                12.0,
                5.0,
            )
            .translate(x, y - 20.0, LOGGER_Z / 2.0 + 2.5);
    }

    for jack in 0..TIMEBASE_JACKS {
        let x = centered_index(jack, TIMEBASE_JACKS, 31.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                "closed_alarm_response_false_positive_fault_injection_station_timebase_jack_{jack}"
            ),
                TIMEBASE_JACK_D / 2.0,
                LOGGER_Z + 8.0,
                24,
            )
            .translate(x, -84.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_timebase_jack_{jack}_index_tick"),
                5.0,
                18.0,
                5.0,
            )
            .translate(x, -64.0, LOGGER_Z / 2.0 + 2.5);
    }

    block - cuts + features + timestamp_lifecycle_strip()
}

fn timestamp_lifecycle_strip() -> Part {
    let mut strip = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_timestamp_lifecycle_strip",
    );
    for (state, label) in LIFECYCLE_STATES.iter().enumerate() {
        strip = strip
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_timestamp_state_land"),
                42.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(state, LIFECYCLE_STATES.len(), 49.0),
                102.0,
                LOGGER_Z / 2.0 + 2.5,
            );
    }
    strip
}

fn quarantine_diverter_mock_path() -> Part {
    let body = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_quarantine_diverter_mock_path_body",
        DIVERTER_X,
        DIVERTER_Y,
        DIVERTER_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_diverter_cuts");
    let mut features = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_diverter_features",
    );

    for channel in 0..MOCK_VALVES {
        let x = centered_index(channel, MOCK_VALVES, DIVERTER_PITCH_X);
        let label = channel_label(channel);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_mock_valve_pocket"),
                13.0,
                DIVERTER_Z + 8.0,
                36,
            )
            .translate(x, 34.0, 0.0)
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_mock_path_port"),
                DIVERTER_PORT_D / 2.0,
                DIVERTER_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -DIVERTER_Y / 2.0 + 18.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_blocked_culture_path_flag"),
                32.0,
                10.0,
                9.0,
            )
            .translate(x, -8.0, DIVERTER_Z / 2.0 + 4.5)
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_retain_path_rib"),
                5.0,
                84.0,
                6.0,
            )
            .translate(x, -30.0, DIVERTER_Z / 2.0 + 3.0);
    }

    for bag in 0..MOCK_BAG_NESTS {
        let (x, y) = grid_position(bag, 2, 2, 142.0, 56.0);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_mock_quarantine_bag_nest_{bag}"),
                108.0,
                34.0,
                16.0,
            )
            .translate(x, y - 58.0, DIVERTER_Z / 2.0 - 6.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_mock_quarantine_bag_{bag}_barcode_land"),
                72.0,
                10.0,
                5.0,
            )
            .translate(x, y - 84.0, DIVERTER_Z / 2.0 + 2.5);
    }

    body - cuts + features + diverter_bypass_bridge()
}

fn diverter_bypass_bridge() -> Part {
    let bridge = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_no_fault_bypass_bridge",
        DIVERTER_X - 64.0,
        8.0,
        9.0,
    )
    .translate(0.0, 78.0, DIVERTER_Z / 2.0 + 4.5);
    let retain_bus = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_retain_bag_bus_rib",
        DIVERTER_X - 112.0,
        8.0,
        7.0,
    )
    .translate(0.0, -38.0, DIVERTER_Z / 2.0 + 3.5);
    bridge + retain_bus
}

fn alarm_disposition_token_rail() -> Part {
    let rail = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_alarm_disposition_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_token_cuts");
    let mut features =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_token_features");

    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, TOKEN_PITCH_X);
        let label = channel_label(channel);
        for (state, state_label) in LIFECYCLE_STATES.iter().enumerate() {
            let y = centered_index(state, LIFECYCLE_STATES.len(), TOKEN_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("closed_alarm_response_false_positive_fault_injection_station_{label}_{state_label}_alarm_token_well"),
                    TOKEN_D / 2.0,
                    TOKEN_Z - 6.0,
                    32,
                )
                .translate(x, y, 4.0);
            features = features
                + centered_cube(
                    format!("closed_alarm_response_false_positive_fault_injection_station_{label}_{state_label}_token_key"),
                    5.0 + state as f64,
                    14.0,
                    6.0,
                )
                .translate(x + 18.0, y, TOKEN_Z / 2.0 + 3.0);
        }
    }

    for channel in 0..ALARM_CHANNELS {
        let label = channel_label(channel);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_token_column_label"),
                42.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(channel, ALARM_CHANNELS, TOKEN_PITCH_X),
                TOKEN_Y / 2.0 - 16.0,
                TOKEN_Z / 2.0 + 2.5,
            );
    }

    rail - cuts + features + token_state_row_ridges()
}

fn token_state_row_ridges() -> Part {
    let mut ridges =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_token_ridges");
    for row in 0..=LIFECYCLE_STATES.len() {
        let y =
            -((LIFECYCLE_STATES.len() as f64) * TOKEN_PITCH_Y) / 2.0 + row as f64 * TOKEN_PITCH_Y;
        ridges = ridges
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_token_row_separator_{row}"),
                TOKEN_X - 42.0,
                5.0,
                6.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 3.0);
    }
    ridges
}

fn reset_interlock_sequence_panel() -> Part {
    let panel = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_reset_interlock_panel_body",
        RESET_X,
        RESET_Y,
        RESET_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_reset_cuts");
    let mut features =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_reset_features");

    for pin in 0..RESET_INTERLOCK_PINS {
        let x = centered_index(pin, RESET_INTERLOCK_PINS, 36.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_reset_interlock_pin_{pin}"),
                RESET_PIN_D / 2.0,
                RESET_Z + 8.0,
                24,
            )
            .translate(x, 64.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_reset_pin_{pin}_armed_indicator"),
                16.0,
                9.0,
                5.0,
            )
            .translate(x, 42.0, RESET_Z / 2.0 + 2.5);
    }

    for step in 0..RESET_SEQUENCE_STEPS {
        let x = centered_index(step, RESET_SEQUENCE_STEPS, RESET_STEP_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_reset_sequence_step_{step}_token_slot"),
                42.0,
                24.0,
                13.0,
            )
            .translate(x, -36.0, RESET_Z / 2.0 - 5.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_reset_sequence_step_{step}_order_rib"),
                8.0,
                58.0,
                7.0,
            )
            .translate(x + 22.0, -36.0, RESET_Z / 2.0 + 3.5);
    }

    panel - cuts + features + no_manual_override_guard()
}

fn no_manual_override_guard() -> Part {
    let guard = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_no_manual_override_guard_bar",
        RESET_X - 58.0,
        10.0,
        18.0,
    )
    .translate(0.0, -RESET_Y / 2.0 + 24.0, RESET_Z / 2.0 + 9.0);
    let service_cover = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_reset_service_cover_keepout",
        RESET_X - 92.0,
        30.0,
        12.0,
    )
    .translate(0.0, -RESET_Y / 2.0 + 55.0, RESET_Z / 2.0 + 6.0);
    guard + service_cover
}

fn challenge_randomization_coupon_matrix() -> Part {
    let plate = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_challenge_matrix_plate",
        CHALLENGE_X,
        CHALLENGE_Y,
        CHALLENGE_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_challenge_cuts");
    let mut features = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_challenge_features",
    );

    for coupon in 0..CHALLENGE_COUPONS {
        let (x, y) = grid_position(
            coupon,
            CHALLENGE_COLS,
            CHALLENGE_ROWS,
            CHALLENGE_PITCH_X,
            CHALLENGE_PITCH_Y,
        );
        let channel = coupon % ALARM_CHANNELS;
        let label = channel_label(channel);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_challenge_coupon_{coupon}_pocket"),
                CHALLENGE_COUPON_X,
                CHALLENGE_COUPON_Y,
                13.0,
            )
            .translate(x, y, CHALLENGE_Z / 2.0 - 5.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_challenge_coupon_{coupon}_randomization_key"),
                7.0 + (coupon % FAULT_LEVELS) as f64 * 3.0,
                14.0,
                6.0,
            )
            .translate(x + 19.0, y, CHALLENGE_Z / 2.0 + 3.0);
    }

    plate - cuts + features + randomization_certificate_lands()
}

fn randomization_certificate_lands() -> Part {
    let left = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_randomization_seed_land",
        92.0,
        18.0,
        5.0,
    )
    .translate(
        -CHALLENGE_X / 2.0 + 64.0,
        CHALLENGE_Y / 2.0 - 24.0,
        CHALLENGE_Z / 2.0 + 2.5,
    );
    let right = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_blinded_challenge_land",
        92.0,
        18.0,
        5.0,
    )
    .translate(
        CHALLENGE_X / 2.0 - 64.0,
        CHALLENGE_Y / 2.0 - 24.0,
        CHALLENGE_Z / 2.0 + 2.5,
    );
    left + right
}

fn run_record_custody_lands() -> Part {
    let plate = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_run_record_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_custody_cuts");
    let mut features = Part::empty(
        "closed_alarm_response_false_positive_fault_injection_station_custody_features",
    );

    for land in 0..RUN_RECORD_LANDS {
        let x = centered_index(land, RUN_RECORD_LANDS, 58.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_run_record_land_{land}"),
                CUSTODY_LAND_X,
                CUSTODY_LAND_Y,
                5.0,
            )
            .translate(x, 52.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for land in 0..RAW_EVENT_LANDS {
        let x = centered_index(land, RAW_EVENT_LANDS, 58.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_raw_event_land_{land}"),
                CUSTODY_LAND_X - 12.0,
                CUSTODY_LAND_Y,
                5.0,
            )
            .translate(x, 8.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for seal in 0..CUSTODY_SEAL_WELLS {
        let x = centered_index(seal, CUSTODY_SEAL_WELLS, 88.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_alarm_response_false_positive_fault_injection_station_custody_seal_well_{seal}"),
                12.0,
                CUSTODY_Z + 4.0,
                32,
            )
            .translate(x, -62.0, 0.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_custody_seal_{seal}_tamper_tab"),
                42.0,
                11.0,
                5.0,
            )
            .translate(x, -91.0, CUSTODY_Z / 2.0 + 2.5);
    }

    plate - cuts + features + custody_barcode_strips()
}

fn custody_barcode_strips() -> Part {
    let mut strips =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_barcode_strips");
    for strip in 0..8 {
        strips = strips
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_barcode_strip_{strip}"),
                7.0 + (strip % 3) as f64 * 3.0,
                62.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 28.0 + strip as f64 * 18.0,
                -8.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    strips
}

fn release_hold_reject_gates() -> Part {
    let body = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_release_hold_reject_gate_body",
        GATES_X,
        GATES_Y,
        GATES_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_gate_cuts");
    let mut features =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_gate_features");

    for (state, label) in DISPOSITION_STATES.iter().enumerate() {
        let x = centered_index(state, DISPOSITION_STATES.len(), GATE_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_gate_lane_recess"),
                GATE_PITCH_X - 18.0,
                150.0,
                16.0,
            )
            .translate(x, 0.0, GATES_Z / 2.0 - 6.0);
        features = features
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_gate_header_land"),
                66.0,
                16.0,
                5.0,
            )
            .translate(x, 82.0, GATES_Z / 2.0 + 2.5);

        for slot in 0..GATE_TOKEN_SLOTS_PER_STATE {
            let y = centered_index(slot, GATE_TOKEN_SLOTS_PER_STATE, 22.0) - 8.0;
            cuts = cuts
                + centered_cube(
                    format!("closed_alarm_response_false_positive_fault_injection_station_{label}_channel_{slot}_gate_token_slot"),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    12.0,
                )
                .translate(x, y, GATES_Z / 2.0 - 4.0);
        }
    }

    body - cuts + features + gate_locking_combs()
}

fn gate_locking_combs() -> Part {
    let mut combs =
        Part::empty("closed_alarm_response_false_positive_fault_injection_station_gate_combs");
    for (state, label) in DISPOSITION_STATES.iter().enumerate() {
        let x = centered_index(state, DISPOSITION_STATES.len(), GATE_PITCH_X);
        combs = combs
            + centered_cube(
                format!("closed_alarm_response_false_positive_fault_injection_station_{label}_gate_locking_comb"),
                8.0,
                154.0,
                10.0,
            )
            .translate(x + 42.0, 0.0, GATES_Z / 2.0 + 5.0);
    }
    combs
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_front_robot_sweep_keepout",
        STATION_X - 140.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -FRONT_ROBOT_SWEEP_CLEARANCE,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_rear_service_keepout",
        STATION_X - 170.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, REAR_SERVICE_CLEARANCE, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_left_bag_service_keepout",
        7.0,
        STATION_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(
        -LEFT_BAG_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_right_logger_service_keepout",
        7.0,
        STATION_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(
        RIGHT_LOGGER_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let top = centered_cube(
        "closed_alarm_response_false_positive_fault_injection_station_module_lift_keepout_gauge",
        118.0,
        118.0,
        TOP_MODULE_LIFT_CLEARANCE,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + TOP_MODULE_LIFT_CLEARANCE / 2.0);
    front + rear + left + right + top
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_scoped_and_complete() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
        assert_eq!(OUTPUTS[11], format!("{OUTPUT_PREFIX}assembly.stl"));
    }

    #[test]
    fn false_positive_fixture_has_independent_truth_controls() {
        assert_eq!(TRUTH_SENSOR_PAIRS, ALARM_CHANNELS);
        assert_eq!(PRIMARY_SENSOR_DOCKS, REFERENCE_SENSOR_DOCKS);
        assert_eq!(PRIMARY_SENSOR_DOCKS, COMPARATOR_WINDOWS);
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"independent_no_fault_truth_reference"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"cell_free_fault_injection"));
    }

    #[test]
    fn alarm_lifecycle_is_traceable_without_manual_override() {
        assert_eq!(TOKEN_WELLS, ALARM_CHANNELS * LIFECYCLE_STATES.len());
        assert_eq!(TIMEBASE_JACKS, ALARM_CHANNELS + 1);
        assert_eq!(RESET_SEQUENCE_STEPS, LIFECYCLE_STATES.len());
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"operator_free_reset_sequence"));
        assert!(LIMITATIONS.contains(&"no_alarm_algorithm"));
    }

    #[test]
    fn layout_footprints_fit_without_overlap() {
        let footprints = station_footprints();
        for footprint in footprints {
            assert!(footprint.fits_inside_deck(), "{}", footprint.name);
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

    #[test]
    fn challenge_matrix_covers_every_channel_at_each_level() {
        assert_eq!(CHALLENGE_COLS, ALARM_CHANNELS);
        assert_eq!(CHALLENGE_ROWS, FAULT_LEVELS);
        assert_eq!(CHALLENGE_COUPONS, ALARM_CHANNELS * FAULT_LEVELS);
        assert_eq!(INJECTION_LANES, ALARM_CHANNELS);
    }
}
