use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed alarm-escalation run hold/release decision station.
//
// Intent:
// - Model a contained, cell-free validation fixture for checking that alarm
//   escalation, automated run-hold, and run-release decisions remain
//   deterministic before they affect tissue-chip perfusion lanes.
// - Keep independent alarm witnesses, severity tokens, an escalation rule-card
//   cassette, run-hold latch bridge, release authorization gate, quarantine /
//   retain splitter, dual-review interlocks, timestamped event docks, custody
//   lands, challenge coupons, and robot service keepouts mechanically visible.
// - Represent labels as CSG rails, keyed pockets, raised token lands, and
//   barcode-like custody plaques so the STL outputs stay self-describing
//   without decal files.
//
// Reproducibility assumptions encoded in the geometry:
// - Hold/release decisions should be exercised on a cell-free station before a
//   culture lane can be paused, diverted, or released back to service.
// - Escalation must preserve a traceable chain from raw alarm input through
//   severity, automated hold, review, and release/hold/escalate disposition.
// - Release is modeled as a dual-review gated action with independent witness
//   inputs and time-aligned event capture, not a loose manual reset.
//
// This is architecture CAD only. It is not a sterile barrier, pressure safety
// device, alarm algorithm, clinical release workflow, controller, software
// validation package, or cell-culture protocol.

const OUTPUT_PREFIX: &str = "output/closed_alarm_escalation_run_hold_release_decision_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_alarm_escalation_run_hold_release_decision_station_base_decision_deck.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_alarm_severity_input_matrix.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_escalation_rule_card_cassette.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_independent_witness_sensor_docks.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_event_timeline_logger_docks.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_run_hold_latch_bridge.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_quarantine_retain_decision_splitter.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_dual_review_release_interlock_panel.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_alarm_lifecycle_token_rail.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_run_record_custody_lands.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_robot_service_keepouts.stl",
    "output/closed_alarm_escalation_run_hold_release_decision_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "alarm_severity_input_matrix",
    "escalation_rule_card_cassette",
    "independent_witness_sensor_docks",
    "event_timeline_logger_docks",
    "run_hold_latch_bridge",
    "quarantine_retain_decision_splitter",
    "dual_review_release_interlock_panel",
    "alarm_lifecycle_token_rail",
    "run_record_custody_lands",
    "base_decision_deck",
    "robot_service_keepouts",
];

const REPRODUCIBILITY_CONTROLS: [&str; 9] = [
    "cell_free_alarm_escalation_challenge",
    "independent_witness_inputs",
    "severity_to_decision_token_chain",
    "operator_free_run_hold_latch",
    "dual_review_release_gate",
    "quarantine_retain_split_before_release",
    "time_aligned_event_record",
    "challenge_coupon_matrix",
    "run_record_custody_labels",
];

const LIMITATIONS: [&str; 7] = [
    "architecture_cad_only",
    "no_alarm_algorithm",
    "no_software_validation_claim",
    "no_pressure_safety_claim",
    "no_sterile_barrier_claim",
    "no_release_acceptance_limits",
    "no_cell_culture_protocol",
];

const ALARM_CHANNELS: usize = 6;
const ALARM_CHANNEL_NAMES: [&str; ALARM_CHANNELS] =
    ["pressure", "flow", "bubble", "ph", "do", "temperature"];
const SEVERITY_LEVELS: usize = 4;
const DECISION_STATES: usize = 3;

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;

const INPUT_POS: (f64, f64) = (-520.0, 260.0);
const INPUT_X: f64 = 320.0;
const INPUT_Y: f64 = 220.0;
const INPUT_Z: f64 = 54.0;
const INPUT_CHANNEL_PITCH_X: f64 = 46.0;
const INPUT_SEVERITY_PITCH_Y: f64 = 36.0;
const INPUT_SENSOR_PORT_D: f64 = 8.4;
const INPUT_STATUS_WINDOWS: usize = ALARM_CHANNELS * SEVERITY_LEVELS;

const RULE_POS: (f64, f64) = (-145.0, 260.0);
const RULE_X: f64 = 350.0;
const RULE_Y: f64 = 220.0;
const RULE_Z: f64 = 58.0;
const RULE_CARD_SLOTS: usize = SEVERITY_LEVELS;
const RULE_CARD_X: f64 = 56.0;
const RULE_CARD_Y: f64 = 132.0;
const RULE_CARD_PITCH_X: f64 = 74.0;

const WITNESS_POS: (f64, f64) = (250.0, 260.0);
const WITNESS_X: f64 = 330.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 52.0;
const PRIMARY_WITNESS_DOCKS: usize = ALARM_CHANNELS;
const INDEPENDENT_WITNESS_DOCKS: usize = ALARM_CHANNELS;
const WITNESS_COMPARATOR_WINDOWS: usize = ALARM_CHANNELS;
const WITNESS_PITCH_X: f64 = 46.0;

const LOGGER_POS: (f64, f64) = (575.0, 260.0);
const LOGGER_X: f64 = 260.0;
const LOGGER_Y: f64 = 220.0;
const LOGGER_Z: f64 = 46.0;
const EVENT_LOGGER_DOCKS: usize = 4;
const TIMEBASE_JACKS: usize = ALARM_CHANNELS + 2;
const LOGGER_DOCK_X: f64 = 52.0;
const LOGGER_DOCK_Y: f64 = 58.0;
const TIMEBASE_JACK_D: f64 = 7.8;

const HOLD_POS: (f64, f64) = (-450.0, 0.0);
const HOLD_X: f64 = 390.0;
const HOLD_Y: f64 = 210.0;
const HOLD_Z: f64 = 60.0;
const HOLD_LATCHES: usize = ALARM_CHANNELS;
const HOLD_BYPASS_CHANNELS: usize = ALARM_CHANNELS;
const HOLD_LATCH_PITCH_X: f64 = 54.0;
const HOLD_PORT_D: f64 = 9.0;

const SPLITTER_POS: (f64, f64) = (-80.0, 0.0);
const SPLITTER_X: f64 = 300.0;
const SPLITTER_Y: f64 = 210.0;
const SPLITTER_Z: f64 = 56.0;
const DECISION_SPLIT_PATHS: usize = DECISION_STATES;
const RETAIN_BAG_NESTS: usize = 2;
const QUARANTINE_BAG_NESTS: usize = 2;
const SPLITTER_VALVE_COUNT: usize = ALARM_CHANNELS;
const SPLITTER_PATH_PITCH_X: f64 = 82.0;

const REVIEW_POS: (f64, f64) = (330.0, 0.0);
const REVIEW_X: f64 = 430.0;
const REVIEW_Y: f64 = 210.0;
const REVIEW_Z: f64 = 50.0;
const REVIEWER_KEYS: usize = 2;
const RELEASE_AUTHORIZATION_KEYS: usize = 2;
const RELEASE_INTERLOCK_PINS: usize = ALARM_CHANNELS + REVIEWER_KEYS;
const REVIEW_PIN_D: f64 = 9.2;

const TOKEN_POS: (f64, f64) = (-500.0, -285.0);
const TOKEN_X: f64 = 360.0;
const TOKEN_Y: f64 = 210.0;
const TOKEN_Z: f64 = 32.0;
const TOKEN_WELLS: usize = ALARM_CHANNELS * (SEVERITY_LEVELS + DECISION_STATES);
const TOKEN_PITCH_X: f64 = 46.0;
const TOKEN_PITCH_Y: f64 = 30.0;
const TOKEN_D: f64 = 16.0;

const CUSTODY_POS: (f64, f64) = (-80.0, -285.0);
const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 210.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 6;
const RAW_EVENT_LANDS: usize = ALARM_CHANNELS;
const CUSTODY_SEAL_WELLS: usize = 4;
const LABEL_BAR_COUNT: usize = 8;

const GATE_POS: (f64, f64) = (390.0, -285.0);
const GATE_X: f64 = 360.0;
const GATE_Y: f64 = 210.0;
const GATE_Z: f64 = 44.0;
const DECISION_GATE_SLOTS_PER_STATE: usize = ALARM_CHANNELS;
const GATE_PITCH_X: f64 = 105.0;
const GATE_SLOT_X: f64 = 38.0;
const GATE_SLOT_Y: f64 = 22.0;

const KEEP_OUT_Z: f64 = 7.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const ROBOT_FIDUCIAL_COUNT: usize = 4;
const LEAK_WITNESS_RAILS: usize = 8;
const BASE_GUTTER_COUNT: usize = 5;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Severity {
    Advisory,
    Warning,
    Critical,
    Stop,
}

impl Severity {
    fn all() -> [Severity; SEVERITY_LEVELS] {
        [
            Severity::Advisory,
            Severity::Warning,
            Severity::Critical,
            Severity::Stop,
        ]
    }

    fn slug(self) -> &'static str {
        match self {
            Severity::Advisory => "advisory",
            Severity::Warning => "warning",
            Severity::Critical => "critical",
            Severity::Stop => "stop",
        }
    }

    fn height(self) -> f64 {
        match self {
            Severity::Advisory => 4.0,
            Severity::Warning => 6.0,
            Severity::Critical => 8.0,
            Severity::Stop => 10.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Decision {
    Release,
    Hold,
    Escalate,
}

impl Decision {
    fn all() -> [Decision; DECISION_STATES] {
        [Decision::Release, Decision::Hold, Decision::Escalate]
    }

    fn slug(self) -> &'static str {
        match self {
            Decision::Release => "release",
            Decision::Hold => "hold",
            Decision::Escalate => "escalate",
        }
    }

    fn gate_height(self) -> f64 {
        match self {
            Decision::Release => 20.0,
            Decision::Hold => 34.0,
            Decision::Escalate => 50.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    let base = base_decision_deck();
    export(OUTPUTS[0], &base);

    let inputs = alarm_severity_input_matrix();
    export(OUTPUTS[1], &inputs);

    let rules = escalation_rule_card_cassette();
    export(OUTPUTS[2], &rules);

    let witnesses = independent_witness_sensor_docks();
    export(OUTPUTS[3], &witnesses);

    let loggers = event_timeline_logger_docks();
    export(OUTPUTS[4], &loggers);

    let hold = run_hold_latch_bridge();
    export(OUTPUTS[5], &hold);

    let splitter = quarantine_retain_decision_splitter();
    export(OUTPUTS[6], &splitter);

    let review = dual_review_release_interlock_panel();
    export(OUTPUTS[7], &review);

    let tokens = alarm_lifecycle_token_rail();
    export(OUTPUTS[8], &tokens);

    let custody = run_record_custody_lands();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + inputs.translate(INPUT_POS.0, INPUT_POS.1, insert_z(INPUT_Z))
        + rules.translate(RULE_POS.0, RULE_POS.1, insert_z(RULE_Z))
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + hold.translate(HOLD_POS.0, HOLD_POS.1, insert_z(HOLD_Z))
        + splitter.translate(SPLITTER_POS.0, SPLITTER_POS.1, insert_z(SPLITTER_Z))
        + review.translate(REVIEW_POS.0, REVIEW_POS.1, insert_z(REVIEW_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + decision_gate_bank().translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_Z))
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed alarm escalation run hold/release decision station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm decision deck with {LEAK_WITNESS_RAILS} leak witness rails"
    );
    println!(
        "  Alarm coverage:         {ALARM_CHANNELS} channels x {SEVERITY_LEVELS} severity states, {INPUT_STATUS_WINDOWS} status windows"
    );
    println!(
        "  Decision chain:         {HOLD_LATCHES} run-hold latches, {DECISION_SPLIT_PATHS} split paths, {DECISION_STATES} disposition gates"
    );
    println!(
        "  Release controls:       {RELEASE_AUTHORIZATION_KEYS} release keys, {REVIEWER_KEYS} reviewer key docks, {RELEASE_INTERLOCK_PINS} interlock pins"
    );
    println!(
        "  Traceability:           {EVENT_LOGGER_DOCKS} event logger docks, {TIMEBASE_JACKS} timebase jacks, {TOKEN_WELLS} lifecycle token wells"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
    println!(
        "  Reproducibility controls: {} controls, {} explicit limitations",
        REPRODUCIBILITY_CONTROLS.len(),
        LIMITATIONS.len()
    );
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

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "alarm_severity_input_matrix",
            center: INPUT_POS,
            x: INPUT_X,
            y: INPUT_Y,
        },
        Footprint {
            name: "escalation_rule_card_cassette",
            center: RULE_POS,
            x: RULE_X,
            y: RULE_Y,
        },
        Footprint {
            name: "independent_witness_sensor_docks",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "event_timeline_logger_docks",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Footprint {
            name: "run_hold_latch_bridge",
            center: HOLD_POS,
            x: HOLD_X,
            y: HOLD_Y,
        },
        Footprint {
            name: "quarantine_retain_decision_splitter",
            center: SPLITTER_POS,
            x: SPLITTER_X,
            y: SPLITTER_Y,
        },
        Footprint {
            name: "dual_review_release_interlock_panel",
            center: REVIEW_POS,
            x: REVIEW_X,
            y: REVIEW_Y,
        },
        Footprint {
            name: "alarm_lifecycle_token_rail",
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
        Footprint {
            name: "decision_gate_bank",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn assert_design_constraints() {
    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds usable deck envelope",
            footprint.name
        );
    }

    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(
                !a.overlaps_with_clearance(*b, 14.0),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }

    assert_eq!(ALARM_CHANNEL_NAMES.len(), ALARM_CHANNELS);
    assert_eq!(Severity::all().len(), SEVERITY_LEVELS);
    assert_eq!(Decision::all().len(), DECISION_STATES);
    assert_eq!(INPUT_STATUS_WINDOWS, ALARM_CHANNELS * SEVERITY_LEVELS);
    assert_eq!(PRIMARY_WITNESS_DOCKS, INDEPENDENT_WITNESS_DOCKS);
    assert_eq!(WITNESS_COMPARATOR_WINDOWS, ALARM_CHANNELS);
    assert_eq!(HOLD_LATCHES, ALARM_CHANNELS);
    assert_eq!(HOLD_BYPASS_CHANNELS, ALARM_CHANNELS);
    assert_eq!(DECISION_SPLIT_PATHS, DECISION_STATES);
    assert_eq!(RELEASE_AUTHORIZATION_KEYS, REVIEWER_KEYS);
    assert_eq!(RELEASE_INTERLOCK_PINS, ALARM_CHANNELS + REVIEWER_KEYS);
    assert_eq!(
        TOKEN_WELLS,
        ALARM_CHANNELS * (SEVERITY_LEVELS + DECISION_STATES)
    );
    assert_eq!(ROBOT_FIDUCIAL_COUNT, 4);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
    assert!(EVENT_LOGGER_DOCKS >= REVIEWER_KEYS + 2);
    assert!(total_decision_gate_slots() >= ALARM_CHANNELS * DECISION_STATES);
}

fn base_decision_deck() -> Part {
    let deck = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wet_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_top_alarm_witness_recess",
        STATION_X - 150.0,
        236.0,
        8.0,
    )
    .translate(0.0, 260.0, BASE_Z / 2.0 - 4.0);
    let decision_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_middle_decision_recess",
        STATION_X - 180.0,
        220.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 4.0);
    let record_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_bottom_record_recess",
        STATION_X - 190.0,
        214.0,
        8.0,
    )
    .translate(0.0, -285.0, BASE_Z / 2.0 - 4.0);
    let drains = centered_cylinder(
        "closed_alarm_escalation_run_hold_release_decision_station_left_leak_drain",
        13.0,
        BASE_Z + 2.0,
        36,
    )
    .translate(-650.0, -390.0, 0.0)
        + centered_cylinder(
            "closed_alarm_escalation_run_hold_release_decision_station_right_leak_drain",
            13.0,
            BASE_Z + 2.0,
            36,
        )
        .translate(650.0, -390.0, 0.0);

    deck - wet_recess - decision_recess - record_recess - drains - base_mount_holes()
        + deck_rim()
        + base_insert_sockets()
        + leak_witness_rails()
        + base_gutters()
        + robot_fiducials()
}

fn base_mount_holes() -> Part {
    let mut holes =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_mount_holes");
    for (i, (x, y)) in [
        (-705.0, -415.0),
        (705.0, -415.0),
        (-705.0, 415.0),
        (705.0, 415.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_alarm_escalation_run_hold_release_decision_station_mount_hole_{i}"),
                MOUNT_HOLE_D,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn deck_rim() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_front_low_rim",
        STATION_X,
        RIM_W,
        RIM_Z * 0.7,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z - RIM_Z * 0.15);
    left + right + rear + front
}

fn base_insert_sockets() -> Part {
    let mut sockets =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_insert_sockets");
    for footprint in station_footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_socket",
                    footprint.name
                ),
                footprint.x + 8.0,
                footprint.y + 8.0,
                SOCKET_DEPTH,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn leak_witness_rails() -> Part {
    let mut rails =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_leak_witness_rails");
    for rail in 0..LEAK_WITNESS_RAILS {
        let x = centered_index(rail, LEAK_WITNESS_RAILS, 160.0);
        rails = rails
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_leak_witness_rail_{rail}"),
                112.0,
                4.0,
                5.0,
            )
            .translate(x, -405.0, BASE_Z / 2.0 + 2.5);
    }
    rails
}

fn base_gutters() -> Part {
    let mut gutters =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_base_gutters");
    for gutter in 0..BASE_GUTTER_COUNT {
        let y = centered_index(gutter, BASE_GUTTER_COUNT, 155.0);
        gutters = gutters
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_flow_gutter_{gutter}"),
                STATION_X - 180.0,
                4.0,
                4.0,
            )
            .translate(0.0, y, BASE_Z / 2.0 + 2.0);
    }
    gutters
}

fn robot_fiducials() -> Part {
    let mut fiducials =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_robot_fiducials");
    for (i, (x, y)) in [
        (-675.0, -395.0),
        (675.0, -395.0),
        (-675.0, 395.0),
        (675.0, 395.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(format!(
                "closed_alarm_escalation_run_hold_release_decision_station_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn alarm_severity_input_matrix() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_alarm_input_body",
        INPUT_X,
        INPUT_Y,
        INPUT_Z,
    );
    let relief = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_alarm_input_underside_relief",
        INPUT_X - 36.0,
        INPUT_Y - 36.0,
        16.0,
    )
    .translate(0.0, 0.0, -INPUT_Z / 2.0 + 7.0);

    let mut ports = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_alarm_channel_ports",
    );
    let mut windows =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_severity_windows");
    let mut labels = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_alarm_channel_labels",
    );
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, INPUT_CHANNEL_PITCH_X);
        ports = ports
            + centered_cylinder(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_sensor_port",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                INPUT_SENSOR_PORT_D,
                INPUT_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
        labels = labels
            + csg_label_plaque(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_label",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                34.0,
                12.0,
                3.0,
                channel,
            )
            .translate(x, INPUT_Y / 2.0 - 20.0, INPUT_Z / 2.0 + 1.5);

        for (level, severity) in Severity::all().iter().enumerate() {
            let severity = *severity;
            let y = centered_index(level, SEVERITY_LEVELS, INPUT_SEVERITY_PITCH_Y) - 12.0;
            windows = windows
                + centered_cube(
                    format!(
                        "closed_alarm_escalation_run_hold_release_decision_station_{}_{}_severity_window",
                        ALARM_CHANNEL_NAMES[channel],
                        severity.slug()
                    ),
                    24.0,
                    12.0,
                    10.0,
                )
                .translate(x, y, INPUT_Z / 2.0 - 4.0)
                + centered_cube(
                    format!(
                        "closed_alarm_escalation_run_hold_release_decision_station_{}_{}_raised_token_land",
                        ALARM_CHANNEL_NAMES[channel],
                        severity.slug()
                    ),
                    18.0,
                    8.0,
                    severity.height(),
                )
                .translate(x, y, INPUT_Z / 2.0 + severity.height() / 2.0);
        }
    }

    body - relief - ports - windows + labels
}

fn escalation_rule_card_cassette() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_rule_card_cassette_body",
        RULE_X,
        RULE_Y,
        RULE_Z,
    );
    let service_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_rule_card_service_recess",
        RULE_X - 30.0,
        RULE_Y - 30.0,
        18.0,
    )
    .translate(0.0, 0.0, RULE_Z / 2.0 - 7.0);

    let mut slots =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_rule_card_slots");
    let mut retainers = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_rule_card_retainers",
    );
    for (slot, severity) in Severity::all().iter().enumerate() {
        let severity = *severity;
        let x = centered_index(slot, RULE_CARD_SLOTS, RULE_CARD_PITCH_X);
        slots = slots
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_rule_card_slot",
                    severity.slug()
                ),
                RULE_CARD_X,
                RULE_CARD_Y,
                24.0,
            )
            .translate(x, 0.0, RULE_Z / 2.0 - 8.0);
        retainers = retainers
            + csg_label_plaque(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_rule_card_land",
                    severity.slug()
                ),
                RULE_CARD_X - 8.0,
                18.0,
                4.0,
                slot + 10,
            )
            .translate(x, -RULE_CARD_Y / 2.0 - 14.0, RULE_Z / 2.0 + 2.0)
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_rule_card_locking_rail",
                    severity.slug()
                ),
                8.0,
                RULE_CARD_Y + 18.0,
                8.0,
            )
            .translate(x + RULE_CARD_X / 2.0 + 8.0, 0.0, RULE_Z / 2.0 + 4.0);
    }

    body - service_recess - slots + retainers
}

fn independent_witness_sensor_docks() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_witness_dock_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_witness_service_recess",
        WITNESS_X - 28.0,
        WITNESS_Y - 28.0,
        16.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0 - 6.0);
    let mut docks =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_witness_dock_cuts");
    let mut comparators = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_witness_comparators",
    );
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, WITNESS_PITCH_X);
        docks = docks
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_primary_witness_dock",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                28.0,
                48.0,
                18.0,
            )
            .translate(x, 48.0, WITNESS_Z / 2.0 - 6.0)
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_independent_witness_dock",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                28.0,
                48.0,
                18.0,
            )
            .translate(x, -48.0, WITNESS_Z / 2.0 - 6.0);
        comparators = comparators
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_witness_comparator_window",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                26.0,
                16.0,
                5.0,
            )
            .translate(x, 0.0, WITNESS_Z / 2.0 + 2.5);
    }
    body - recess - docks + comparators + witness_pairing_rails()
}

fn witness_pairing_rails() -> Part {
    let mut rails = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_witness_pairing_rails",
    );
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, WITNESS_PITCH_X);
        rails = rails
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_witness_pairing_rail",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                7.0,
                112.0,
                6.0,
            )
            .translate(x, 0.0, WITNESS_Z / 2.0 + 3.0);
    }
    rails
}

fn event_timeline_logger_docks() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_logger_body",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_logger_dock_cuts");
    for dock in 0..EVENT_LOGGER_DOCKS {
        let x = centered_index(dock, EVENT_LOGGER_DOCKS, 58.0);
        let y = if dock % 2 == 0 { 42.0 } else { -42.0 };
        cuts = cuts
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_event_logger_dock_{dock}"),
                LOGGER_DOCK_X,
                LOGGER_DOCK_Y,
                18.0,
            )
            .translate(x, y, LOGGER_Z / 2.0 - 6.0);
    }
    let mut jacks =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_timebase_jacks");
    for jack in 0..TIMEBASE_JACKS {
        let x = centered_index(jack, TIMEBASE_JACKS, 27.0);
        jacks = jacks
            + centered_cylinder(
                format!("closed_alarm_escalation_run_hold_release_decision_station_timebase_jack_{jack}"),
                TIMEBASE_JACK_D,
                12.0,
                24,
            )
            .translate(x, 0.0, LOGGER_Z / 2.0 + 6.0);
    }
    body - cuts + jacks + timeline_state_rail()
}

fn timeline_state_rail() -> Part {
    let rail = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_timeline_state_rail_body",
        LOGGER_X - 28.0,
        24.0,
        8.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 24.0, LOGGER_Z / 2.0 + 4.0);
    let mut ticks =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_timeline_ticks");
    for tick in 0..(SEVERITY_LEVELS + DECISION_STATES + 2) {
        let x = centered_index(tick, SEVERITY_LEVELS + DECISION_STATES + 2, 25.0);
        ticks = ticks
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_timeline_tick_{tick}"),
                3.0,
                26.0,
                12.0,
            )
            .translate(x, -LOGGER_Y / 2.0 + 24.0, LOGGER_Z / 2.0 + 6.0);
    }
    rail + ticks
}

fn run_hold_latch_bridge() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_run_hold_bridge_body",
        HOLD_X,
        HOLD_Y,
        HOLD_Z,
    );
    let service_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_run_hold_service_recess",
        HOLD_X - 32.0,
        HOLD_Y - 32.0,
        18.0,
    )
    .translate(0.0, 0.0, HOLD_Z / 2.0 - 8.0);
    let mut latch_cuts =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_hold_latch_cuts");
    let mut bridges = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_hold_bypass_bridges",
    );
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, HOLD_LATCH_PITCH_X);
        latch_cuts = latch_cuts
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_hold_latch_slot",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                34.0,
                74.0,
                18.0,
            )
            .translate(x, 38.0, HOLD_Z / 2.0 - 6.0)
            + centered_cylinder(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_hold_port_bore",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                HOLD_PORT_D,
                HOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
        bridges = bridges
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_fail_hold_bypass_bridge",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                18.0,
                118.0,
                7.0,
            )
            .translate(x, -10.0, HOLD_Z / 2.0 + 3.5);
    }
    body - service_recess - latch_cuts + bridges + hold_state_flags()
}

fn hold_state_flags() -> Part {
    let mut flags =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_hold_state_flags");
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, HOLD_LATCH_PITCH_X);
        flags = flags
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_hold_asserted_flag",
                    ALARM_CHANNEL_NAMES[channel]
                ),
                26.0,
                8.0,
                12.0,
            )
            .translate(x, HOLD_Y / 2.0 - 24.0, HOLD_Z / 2.0 + 6.0);
    }
    flags
}

fn quarantine_retain_decision_splitter() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_decision_splitter_body",
        SPLITTER_X,
        SPLITTER_Y,
        SPLITTER_Z,
    );
    let relief = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_splitter_service_recess",
        SPLITTER_X - 34.0,
        SPLITTER_Y - 34.0,
        18.0,
    )
    .translate(0.0, 0.0, SPLITTER_Z / 2.0 - 7.0);
    let mut path_cuts =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_split_path_cuts");
    let mut labels =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_split_path_labels");
    for decision in Decision::all() {
        let x = centered_index(decision as usize, DECISION_STATES, SPLITTER_PATH_PITCH_X);
        path_cuts = path_cuts
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_split_path_recess",
                    decision.slug()
                ),
                56.0,
                152.0,
                16.0,
            )
            .translate(x, 0.0, SPLITTER_Z / 2.0 - 6.0);
        labels = labels
            + csg_label_plaque(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_split_label",
                    decision.slug()
                ),
                46.0,
                14.0,
                3.0,
                decision as usize + 30,
            )
            .translate(x, SPLITTER_Y / 2.0 - 18.0, SPLITTER_Z / 2.0 + 1.5);
    }
    let bag_nests = bag_nest_bank();
    let valves = splitter_valve_tokens();
    body - relief - path_cuts + labels + bag_nests + valves
}

fn bag_nest_bank() -> Part {
    let mut nests =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_bag_nest_bank");
    for nest in 0..(RETAIN_BAG_NESTS + QUARANTINE_BAG_NESTS) {
        let x = centered_index(nest, RETAIN_BAG_NESTS + QUARANTINE_BAG_NESTS, 54.0);
        let y = -SPLITTER_Y / 2.0 + 42.0;
        nests = nests
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_retain_quarantine_bag_nest_{nest}"),
                42.0,
                44.0,
                7.0,
            )
            .translate(x, y, SPLITTER_Z / 2.0 + 3.5)
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_bag_nest_{nest}_clamp"),
                34.0,
                4.0,
                10.0,
            )
            .translate(x, y + 24.0, SPLITTER_Z / 2.0 + 5.0);
    }
    nests
}

fn splitter_valve_tokens() -> Part {
    let mut valves =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_splitter_valves");
    for valve in 0..SPLITTER_VALVE_COUNT {
        let x = centered_index(valve, SPLITTER_VALVE_COUNT, 38.0);
        valves = valves
            + centered_cylinder(
                format!("closed_alarm_escalation_run_hold_release_decision_station_splitter_valve_{valve}"),
                18.0,
                8.0,
                28,
            )
            .translate(x, 28.0, SPLITTER_Z / 2.0 + 4.0);
    }
    valves
}

fn dual_review_release_interlock_panel() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_review_panel_body",
        REVIEW_X,
        REVIEW_Y,
        REVIEW_Z,
    );
    let service_recess = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_review_panel_service_recess",
        REVIEW_X - 34.0,
        REVIEW_Y - 34.0,
        14.0,
    )
    .translate(0.0, 0.0, REVIEW_Z / 2.0 - 5.0);
    let mut pin_cuts =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_review_pin_cuts");
    for pin in 0..RELEASE_INTERLOCK_PINS {
        let x = centered_index(pin, RELEASE_INTERLOCK_PINS, 42.0);
        pin_cuts = pin_cuts
            + centered_cylinder(
                format!("closed_alarm_escalation_run_hold_release_decision_station_release_interlock_pin_{pin}"),
                REVIEW_PIN_D,
                18.0,
                24,
            )
            .translate(x, 0.0, REVIEW_Z / 2.0 - 4.0);
    }
    body - service_recess - pin_cuts + reviewer_key_docks() + release_authorization_gates()
}

fn reviewer_key_docks() -> Part {
    let mut docks =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_reviewer_key_docks");
    for reviewer in 0..REVIEWER_KEYS {
        let x = centered_index(reviewer, REVIEWER_KEYS, 160.0);
        docks = docks
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_reviewer_{reviewer}_key_dock"),
                76.0,
                42.0,
                8.0,
            )
            .translate(x, REVIEW_Y / 2.0 - 40.0, REVIEW_Z / 2.0 + 4.0)
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_reviewer_{reviewer}_presence_flag"),
                12.0,
                54.0,
                14.0,
            )
            .translate(x + 46.0, REVIEW_Y / 2.0 - 40.0, REVIEW_Z / 2.0 + 7.0);
    }
    docks
}

fn release_authorization_gates() -> Part {
    let mut gates =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_release_auth_gates");
    for gate in 0..RELEASE_AUTHORIZATION_KEYS {
        let x = centered_index(gate, RELEASE_AUTHORIZATION_KEYS, 140.0);
        gates = gates
            + centered_cube(
                format!("closed_alarm_escalation_run_hold_release_decision_station_release_authorization_gate_{gate}"),
                104.0,
                22.0,
                12.0,
            )
            .translate(x, -REVIEW_Y / 2.0 + 34.0, REVIEW_Z / 2.0 + 6.0);
    }
    gates
}

fn alarm_lifecycle_token_rail() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut wells = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_lifecycle_token_wells",
    );
    for channel in 0..ALARM_CHANNELS {
        let x = centered_index(channel, ALARM_CHANNELS, TOKEN_PITCH_X);
        for level in 0..(SEVERITY_LEVELS + DECISION_STATES) {
            let y = centered_index(level, SEVERITY_LEVELS + DECISION_STATES, TOKEN_PITCH_Y);
            wells = wells
                + centered_cylinder(
                    format!(
                        "closed_alarm_escalation_run_hold_release_decision_station_{}_lifecycle_token_well_{level}",
                        ALARM_CHANNEL_NAMES[channel]
                    ),
                    TOKEN_D,
                    10.0,
                    24,
                )
                .translate(x, y, TOKEN_Z / 2.0 - 3.0);
        }
    }
    let headers = severity_and_decision_headers();
    body - wells + headers
}

fn severity_and_decision_headers() -> Part {
    let mut headers =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_token_headers");
    for (level, severity) in Severity::all().iter().enumerate() {
        let y = centered_index(level, SEVERITY_LEVELS + DECISION_STATES, TOKEN_PITCH_Y);
        headers = headers
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_severity_header",
                    severity.slug()
                ),
                TOKEN_X - 34.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 2.5);
    }
    for (offset, decision) in Decision::all().iter().enumerate() {
        let row = SEVERITY_LEVELS + offset;
        let y = centered_index(row, SEVERITY_LEVELS + DECISION_STATES, TOKEN_PITCH_Y);
        headers = headers
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_decision_header",
                    decision.slug()
                ),
                TOKEN_X - 54.0,
                4.0,
                7.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 3.5);
    }
    headers
}

fn run_record_custody_lands() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_custody_land_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_run_record_lands");
    for land in 0..RUN_RECORD_LANDS {
        let x = centered_index(land, RUN_RECORD_LANDS, 58.0);
        lands = lands
            + csg_label_plaque(
                format!("closed_alarm_escalation_run_hold_release_decision_station_run_record_land_{land}"),
                46.0,
                24.0,
                3.0,
                land + 50,
            )
            .translate(x, 52.0, CUSTODY_Z / 2.0 + 1.5);
    }

    for event in 0..RAW_EVENT_LANDS {
        let x = centered_index(event, RAW_EVENT_LANDS, 58.0);
        lands = lands
            + csg_label_plaque(
                format!("closed_alarm_escalation_run_hold_release_decision_station_raw_event_land_{event}"),
                46.0,
                22.0,
                3.0,
                event + 70,
            )
            .translate(x, -2.0, CUSTODY_Z / 2.0 + 1.5);
    }

    for seal in 0..CUSTODY_SEAL_WELLS {
        let x = centered_index(seal, CUSTODY_SEAL_WELLS, 84.0);
        lands = lands
            + centered_cylinder(
                format!("closed_alarm_escalation_run_hold_release_decision_station_custody_seal_well_{seal}"),
                18.0,
                6.0,
                28,
            )
            .translate(x, -70.0, CUSTODY_Z / 2.0 + 3.0);
    }

    body + lands + challenge_coupon_matrix()
}

fn challenge_coupon_matrix() -> Part {
    let mut coupons =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_challenge_coupons");
    for severity in 0..SEVERITY_LEVELS {
        for channel in 0..ALARM_CHANNELS {
            let x = centered_index(channel, ALARM_CHANNELS, 30.0);
            let y = -CUSTODY_Y / 2.0 + 18.0 + severity as f64 * 16.0;
            coupons = coupons
                + centered_cube(
                    format!(
                        "closed_alarm_escalation_run_hold_release_decision_station_{}_severity_{severity}_challenge_coupon",
                        ALARM_CHANNEL_NAMES[channel]
                    ),
                    18.0,
                    10.0,
                    4.0,
                )
                .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
        }
    }
    coupons
}

fn decision_gate_bank() -> Part {
    let body = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_decision_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut cuts =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_decision_gate_cuts");
    let mut features = Part::empty(
        "closed_alarm_escalation_run_hold_release_decision_station_decision_gate_features",
    );
    for decision in Decision::all() {
        let x = centered_index(decision as usize, DECISION_STATES, GATE_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!(
                "closed_alarm_escalation_run_hold_release_decision_station_{}_decision_lane_recess",
                decision.slug()
            ),
                GATE_PITCH_X - 18.0,
                152.0,
                16.0,
            )
            .translate(x, 0.0, GATE_Z / 2.0 - 6.0);
        features = features
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_decision_gate_header",
                    decision.slug()
                ),
                70.0,
                16.0,
                decision.gate_height() / 5.0,
            )
            .translate(x, GATE_Y / 2.0 - 25.0, GATE_Z / 2.0 + decision.gate_height() / 10.0);

        for slot in 0..DECISION_GATE_SLOTS_PER_STATE {
            let y = centered_index(slot, DECISION_GATE_SLOTS_PER_STATE, 22.0) - 10.0;
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_alarm_escalation_run_hold_release_decision_station_{}_channel_{slot}_decision_token_slot",
                        decision.slug()
                    ),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    12.0,
                )
                .translate(x, y, GATE_Z / 2.0 - 4.0);
        }
    }
    body - cuts + features + gate_locking_combs()
}

fn gate_locking_combs() -> Part {
    let mut combs =
        Part::empty("closed_alarm_escalation_run_hold_release_decision_station_gate_locking_combs");
    for decision in Decision::all() {
        let x = centered_index(decision as usize, DECISION_STATES, GATE_PITCH_X);
        combs = combs
            + centered_cube(
                format!(
                    "closed_alarm_escalation_run_hold_release_decision_station_{}_gate_locking_comb",
                    decision.slug()
                ),
                8.0,
                156.0,
                10.0,
            )
            .translate(x + 42.0, 0.0, GATE_Z / 2.0 + 5.0);
    }
    combs
}

fn total_decision_gate_slots() -> usize {
    DECISION_STATES * DECISION_GATE_SLOTS_PER_STATE
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_front_robot_sweep_keepout",
        STATION_X - 140.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -408.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_rear_service_keepout",
        STATION_X - 170.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, 408.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_left_bag_service_keepout",
        7.0,
        STATION_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(-700.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_right_logger_service_keepout",
        7.0,
        STATION_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(700.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let top = centered_cube(
        "closed_alarm_escalation_run_hold_release_decision_station_module_lift_keepout_gauge",
        118.0,
        118.0,
        132.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + 66.0);
    front + rear + left + right + top
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 2.0 + ((seed + i) % 4) as f64 * 1.2;
        let height = (y - 5.0 - (i % 3) as f64 * 1.4).max(3.0);
        let x_offset = -x / 2.0 + 7.0 + i as f64 * ((x - 14.0) / LABEL_BAR_COUNT as f64);
        bars =
            bars + centered_cube(format!("{name}_raised_bar_{i}"), width, height, z + 1.0)
                .translate(x_offset, 0.0, z / 2.0 + 0.5);
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 8.0, 3.0, z + 1.2)
        .translate(x / 2.0 - 8.0, y / 2.0 - 4.0, z / 2.0 + 0.6);
    base + bars + orientation_tab
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer_ring"), 14.0, 4.0, 36)
        - centered_cylinder(format!("{name}_inner_dot"), 5.0, 5.0, 24)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(OUTPUTS[11], format!("{OUTPUT_PREFIX}assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn required_feature_groups_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for expected in [
            "alarm_severity_input_matrix",
            "escalation_rule_card_cassette",
            "independent_witness_sensor_docks",
            "event_timeline_logger_docks",
            "run_hold_latch_bridge",
            "quarantine_retain_decision_splitter",
            "dual_review_release_interlock_panel",
            "alarm_lifecycle_token_rail",
            "run_record_custody_lands",
            "base_decision_deck",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn reproducibility_controls_exclude_manual_release_reset() {
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"cell_free_alarm_escalation_challenge"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"operator_free_run_hold_latch"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"dual_review_release_gate"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"time_aligned_event_record"));
        assert!(LIMITATIONS.contains(&"no_alarm_algorithm"));
        assert!(LIMITATIONS.contains(&"no_release_acceptance_limits"));
    }

    #[test]
    fn layout_footprints_fit_without_overlap() {
        assert_design_constraints();
        assert_eq!(station_footprints().len(), 10);
    }

    #[test]
    fn alarm_channels_cover_environment_and_perfusion_inputs() {
        assert_eq!(ALARM_CHANNEL_NAMES.len(), ALARM_CHANNELS);
        for expected in ["pressure", "flow", "bubble", "ph", "do", "temperature"] {
            assert!(ALARM_CHANNEL_NAMES.contains(&expected));
        }
        assert_eq!(INPUT_STATUS_WINDOWS, ALARM_CHANNELS * SEVERITY_LEVELS);
        assert_eq!(Severity::all().len(), SEVERITY_LEVELS);
    }

    #[test]
    fn hold_release_chain_has_independent_witness_and_review_capacity() {
        assert_eq!(PRIMARY_WITNESS_DOCKS, ALARM_CHANNELS);
        assert_eq!(INDEPENDENT_WITNESS_DOCKS, ALARM_CHANNELS);
        assert_eq!(WITNESS_COMPARATOR_WINDOWS, ALARM_CHANNELS);
        assert_eq!(HOLD_LATCHES, ALARM_CHANNELS);
        assert_eq!(HOLD_BYPASS_CHANNELS, ALARM_CHANNELS);
        assert_eq!(RELEASE_AUTHORIZATION_KEYS, REVIEWER_KEYS);
        assert_eq!(RELEASE_INTERLOCK_PINS, ALARM_CHANNELS + REVIEWER_KEYS);
    }

    #[test]
    fn lifecycle_tokens_and_decision_gates_cover_every_channel() {
        assert_eq!(
            TOKEN_WELLS,
            ALARM_CHANNELS * (SEVERITY_LEVELS + DECISION_STATES)
        );
        assert_eq!(Decision::all().len(), DECISION_STATES);
        assert_eq!(
            total_decision_gate_slots(),
            DECISION_STATES * DECISION_GATE_SLOTS_PER_STATE
        );
        assert!(total_decision_gate_slots() >= ALARM_CHANNELS * DECISION_STATES);
        assert_eq!(DECISION_SPLIT_PATHS, DECISION_STATES);
    }
}
