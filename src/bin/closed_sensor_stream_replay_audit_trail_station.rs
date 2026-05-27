use std::fs;

use vcad::{centered_cube, centered_cylinder as vcad_centered_cylinder, Part};

// Closed sensor stream replay audit-trail station.
//
// Intent:
// - Package a no-cell validation fixture that replays captured sensor streams
//   through the closed-system data path before tissue-chip runs depend on the
//   resulting record.
// - Keep raw stream cartridges, replay controller identity, per-stream patching,
//   event-order/timebase references, expected/observed comparison lanes,
//   checksum/hash seals, anomaly-injection tokens, audit-trail custody lands,
//   release/quarantine gates, and camera evidence targets visible as physical
//   interfaces.
//
// Research assumptions encoded in the fixture:
// - Reproducibility depends on proving the data trail, not only the fluidic
//   hardware. A replayable raw stream helps separate true culture drift from
//   logger, parser, transport, or analysis drift.
// - Data-integrity practice expects records to remain attributable, complete,
//   contemporaneous, original, accurate, and traceable. The design therefore
//   separates raw capture custody, replay execution evidence, checksum
//   witnesses, and final release decisions.
// - Replay validation is a no-cell challenge. It should run with synthetic or
//   archived streams before a culture batch is accepted, then quarantine any
//   stream whose replay no longer matches the expected event order or hash.
//
// This is validation fixture/interface CAD only. It does not define data
// historian software, cryptographic implementation, regulatory acceptance
// limits, sterile processing instructions, cybersecurity controls, sensor
// calibration methods, or biological performance claims.

const OUTPUT_PREFIX: &str = "output/closed_sensor_stream_replay_audit_trail_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_sensor_stream_replay_audit_trail_station_base_replay_audit_deck.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_raw_stream_cartridge_vault.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_replay_controller_sled.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_stream_lane_patch_panel.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_timebase_event_order_bridge.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_expected_observed_comparator_lanes.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_checksum_hash_seal_strip.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_anomaly_injection_token_tray.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_audit_trail_custody_lands.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_release_quarantine_disposition_gates.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_evidence_camera_status_target.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_robot_service_keepouts.stl",
    "output/closed_sensor_stream_replay_audit_trail_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_replay_audit_deck",
    "raw_stream_cartridge_vault",
    "replay_controller_sled",
    "stream_lane_patch_panel",
    "timebase_event_order_bridge",
    "expected_observed_comparator_lanes",
    "checksum_hash_seal_strip",
    "anomaly_injection_token_tray",
    "audit_trail_custody_lands",
    "release_quarantine_disposition_gates",
    "evidence_camera_status_target",
    "robot_service_keepouts",
];

const DESIGN_ASSUMPTIONS: [&str; 7] = [
    "no_cell_replay_validation_before_culture_release",
    "raw_stream_custody_separate_from_replay_execution",
    "expected_observed_comparison_per_sensor_stream",
    "hash_manifest_and_event_order_evidence_stay_with_run_record",
    "parser_model_firmware_identity_must_be_replay_visible",
    "mismatched_replay_streams_are_quarantined_before_release",
    "robot_events_share_the_same_audit_trail",
];

const LIMITATIONS: [&str; 8] = [
    "validation_fixture_only",
    "no_data_historian_software",
    "no_cryptographic_implementation",
    "no_regulatory_acceptance_limits",
    "no_cybersecurity_control_claim",
    "no_sterile_barrier_claim",
    "no_sensor_calibration_method",
    "no_biological_performance_claim",
];

const STREAM_COUNT: usize = 8;
const STREAM_NAMES: [&str; STREAM_COUNT] = [
    "pressure",
    "flow",
    "ph_do",
    "dissolved_oxygen",
    "co2_humidity",
    "imaging",
    "scale_mass",
    "robot_events",
];

const REPLAY_MODES: [&str; 6] = [
    "baseline",
    "offset_time",
    "dropped_frame",
    "duplicated_event",
    "stale_calibration",
    "reordered_packet",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 930.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.8;
const DATA_PORT_D: f64 = 9.2;

const VAULT_POS: (f64, f64) = (-540.0, 250.0);
const VAULT_X: f64 = 320.0;
const VAULT_Y: f64 = 200.0;
const VAULT_Z: f64 = 52.0;
const RAW_STREAM_CARTRIDGES: usize = STREAM_COUNT;
const CARTRIDGE_COLS: usize = 4;
const CARTRIDGE_ROWS: usize = 2;
const CARTRIDGE_POCKET_X: f64 = 58.0;
const CARTRIDGE_POCKET_Y: f64 = 42.0;
const CARTRIDGE_PITCH_X: f64 = 72.0;
const CARTRIDGE_PITCH_Y: f64 = 72.0;
const WRITE_PROTECT_TABS: usize = STREAM_COUNT;

const CONTROLLER_POS: (f64, f64) = (-165.0, 250.0);
const CONTROLLER_X: f64 = 330.0;
const CONTROLLER_Y: f64 = 200.0;
const CONTROLLER_Z: f64 = 50.0;
const CONTROLLER_DRIVE_BAYS: usize = 3;
const PARSER_VERSION_KEYS: usize = 4;
const FIRMWARE_ID_LANDS: usize = 4;
const EXECUTION_TOKEN_WELLS: usize = 5;

const PATCH_POS: (f64, f64) = (205.0, 250.0);
const PATCH_X: f64 = 360.0;
const PATCH_Y: f64 = 200.0;
const PATCH_Z: f64 = 44.0;
const PATCH_INPUT_PORTS: usize = STREAM_COUNT;
const PATCH_OUTPUT_PORTS: usize = STREAM_COUNT;
const PATCH_LOOPBACK_GAUGES: usize = STREAM_COUNT;
const PATCH_PORT_PITCH_X: f64 = 38.0;
const PATCH_LOOPBACK_SLOT_X: f64 = 30.0;
const PATCH_LOOPBACK_SLOT_Y: f64 = 14.0;

const TIMEBASE_POS: (f64, f64) = (555.0, 250.0);
const TIMEBASE_X: f64 = 250.0;
const TIMEBASE_Y: f64 = 200.0;
const TIMEBASE_Z: f64 = 72.0;
const REFERENCE_TIME_PORTS: usize = 3;
const EVENT_ORDER_CHANNELS: usize = STREAM_COUNT;
const ORDER_TICK_MARKS: usize = 16;
const TIMEBASE_STATUS_WINDOWS: usize = STREAM_COUNT;
const TIMEBASE_CLOCK_D: f64 = 88.0;

const COMPARATOR_POS: (f64, f64) = (-465.0, 0.0);
const COMPARATOR_X: f64 = 450.0;
const COMPARATOR_Y: f64 = 220.0;
const COMPARATOR_Z: f64 = 36.0;
const COMPARATOR_LANES: usize = STREAM_COUNT;
const EXPECTED_OBSERVED_SLOT_PAIRS: usize = STREAM_COUNT;
const COMPARATOR_TICK_STATIONS: usize = 6;
const COMPARATOR_LANE_PITCH_Y: f64 = 23.0;
const COMPARATOR_TICK_PITCH_X: f64 = 58.0;

const CHECKSUM_POS: (f64, f64) = (40.0, 0.0);
const CHECKSUM_X: f64 = 500.0;
const CHECKSUM_Y: f64 = 220.0;
const CHECKSUM_Z: f64 = 24.0;
const RAW_HASH_LANDS: usize = STREAM_COUNT;
const REPLAY_HASH_LANDS: usize = STREAM_COUNT;
const MANIFEST_KEY_SLOTS: usize = 5;
const HASH_SEAL_WELLS: usize = 6;
const HASH_LAND_X: f64 = 48.0;
const HASH_LAND_Y: f64 = 20.0;

const TOKEN_POS: (f64, f64) = (520.0, 0.0);
const TOKEN_X: f64 = 330.0;
const TOKEN_Y: f64 = 220.0;
const TOKEN_Z: f64 = 30.0;
const MODE_TOKEN_WELLS: usize = REPLAY_MODES.len();
const SEED_TOKEN_WELLS: usize = 8;
const BLIND_BATCH_TOKEN_SLOTS: usize = 6;
const TOKEN_WELL_D: f64 = 21.0;
const TOKEN_PITCH_X: f64 = 52.0;
const TOKEN_PITCH_Y: f64 = 48.0;

const CUSTODY_POS: (f64, f64) = (-480.0, -280.0);
const CUSTODY_X: f64 = 380.0;
const CUSTODY_Y: f64 = 180.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 5;
const RAW_STREAM_CUSTODY_LANDS: usize = STREAM_COUNT;
const REPLAY_MANIFEST_LANDS: usize = 4;
const OPERATOR_EQUIPMENT_VERSION_LANDS: usize = 4;

const GATE_POS: (f64, f64) = (-55.0, -280.0);
const GATE_X: f64 = 410.0;
const GATE_Y: f64 = 180.0;
const GATE_Z: f64 = 42.0;
const GATE_NAMES: [&str; 3] = ["release", "rerun", "quarantine"];
const DISPOSITION_GATES: usize = GATE_NAMES.len();
const GATE_TOKEN_SLOTS_PER_GATE: usize = STREAM_COUNT;
const GATE_PITCH_X: f64 = 118.0;
const GATE_SLOT_X: f64 = 24.0;
const GATE_SLOT_Y: f64 = 16.0;

const CAMERA_POS: (f64, f64) = (390.0, -280.0);
const CAMERA_X: f64 = 340.0;
const CAMERA_Y: f64 = 180.0;
const CAMERA_Z: f64 = 34.0;
const CAMERA_FIDUCIALS: usize = 4;
const CAMERA_STATUS_WINDOWS: usize = STREAM_COUNT;
const CAMERA_CLOCK_TICKS: usize = 12;
const CAMERA_TARGET_D: f64 = 88.0;

const KEEP_OUT_X: f64 = 1480.0;
const KEEP_OUT_Y: f64 = 850.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 405.0;
const REAR_DATA_SERVICE_CLEARANCE: f64 = 170.0;
const LEFT_CARTRIDGE_SERVICE_CLEARANCE: f64 = 165.0;
const RIGHT_AUDIT_SERVICE_CLEARANCE: f64 = 175.0;
const CONTROLLER_LIFT_CLEARANCE_Z: f64 = 135.0;

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

    let deck = base_replay_audit_deck();
    export(OUTPUTS[0], &deck);

    let vault = raw_stream_cartridge_vault();
    export(OUTPUTS[1], &vault);

    let controller = replay_controller_sled();
    export(OUTPUTS[2], &controller);

    let patch = stream_lane_patch_panel();
    export(OUTPUTS[3], &patch);

    let timebase = timebase_event_order_bridge();
    export(OUTPUTS[4], &timebase);

    let comparator = expected_observed_comparator_lanes();
    export(OUTPUTS[5], &comparator);

    let checksum = checksum_hash_seal_strip();
    export(OUTPUTS[6], &checksum);

    let tokens = anomaly_injection_token_tray();
    export(OUTPUTS[7], &tokens);

    let custody = audit_trail_custody_lands();
    export(OUTPUTS[8], &custody);

    let gates = release_quarantine_disposition_gates();
    export(OUTPUTS[9], &gates);

    let camera = evidence_camera_status_target();
    export(OUTPUTS[10], &camera);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + vault.translate(VAULT_POS.0, VAULT_POS.1, on_deck_z(VAULT_Z))
        + controller.translate(CONTROLLER_POS.0, CONTROLLER_POS.1, on_deck_z(CONTROLLER_Z))
        + patch.translate(PATCH_POS.0, PATCH_POS.1, on_deck_z(PATCH_Z))
        + timebase.translate(TIMEBASE_POS.0, TIMEBASE_POS.1, on_deck_z(TIMEBASE_Z))
        + comparator.translate(COMPARATOR_POS.0, COMPARATOR_POS.1, on_deck_z(COMPARATOR_Z))
        + checksum.translate(CHECKSUM_POS.0, CHECKSUM_POS.1, on_deck_z(CHECKSUM_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed sensor stream replay audit-trail station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm no-cell replay validation deck"
    );
    println!(
        "  Stream coverage:           {STREAM_COUNT} streams ({})",
        STREAM_NAMES.join(", ")
    );
    println!(
        "  Raw custody:               {RAW_STREAM_CARTRIDGES} stream cartridges, {WRITE_PROTECT_TABS} write-protect witnesses, {RAW_STREAM_CUSTODY_LANDS} custody lands"
    );
    println!(
        "  Replay controls:           {CONTROLLER_DRIVE_BAYS} drive bays, {PARSER_VERSION_KEYS} parser keys, {FIRMWARE_ID_LANDS} firmware/equipment ID lands"
    );
    println!(
        "  Comparison controls:       {COMPARATOR_LANES} lanes, {COMPARATOR_TICK_STATIONS} event ticks, {RAW_HASH_LANDS}+{REPLAY_HASH_LANDS} hash lands"
    );
    println!(
        "  Challenge tokens:          {} replay modes, {SEED_TOKEN_WELLS} seed wells, {BLIND_BATCH_TOKEN_SLOTS} blind batch slots",
        REPLAY_MODES.join("/")
    );
    println!(
        "  Disposition controls:      {} gates, {GATE_TOKEN_SLOTS_PER_GATE} stream tokens per gate",
        GATE_NAMES.join("/")
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

fn centered_cylinder(name: impl Into<String>, diameter: f64, height: f64) -> Part {
    vcad_centered_cylinder(name, diameter / 2.0, height, 32)
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_position(index: usize, cols: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, (STREAM_COUNT + cols - 1) / cols, pitch_y),
    )
}

fn stream_label(index: usize) -> &'static str {
    STREAM_NAMES.get(index).copied().unwrap_or("unknown_stream")
}

fn replay_mode_label(index: usize) -> &'static str {
    REPLAY_MODES
        .get(index)
        .copied()
        .unwrap_or("unknown_replay_mode")
}

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "raw_stream_cartridge_vault",
            center: VAULT_POS,
            x: VAULT_X,
            y: VAULT_Y,
        },
        Footprint {
            name: "replay_controller_sled",
            center: CONTROLLER_POS,
            x: CONTROLLER_X,
            y: CONTROLLER_Y,
        },
        Footprint {
            name: "stream_lane_patch_panel",
            center: PATCH_POS,
            x: PATCH_X,
            y: PATCH_Y,
        },
        Footprint {
            name: "timebase_event_order_bridge",
            center: TIMEBASE_POS,
            x: TIMEBASE_X,
            y: TIMEBASE_Y,
        },
        Footprint {
            name: "expected_observed_comparator_lanes",
            center: COMPARATOR_POS,
            x: COMPARATOR_X,
            y: COMPARATOR_Y,
        },
        Footprint {
            name: "checksum_hash_seal_strip",
            center: CHECKSUM_POS,
            x: CHECKSUM_X,
            y: CHECKSUM_Y,
        },
        Footprint {
            name: "anomaly_injection_token_tray",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "audit_trail_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "release_quarantine_disposition_gates",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Footprint {
            name: "evidence_camera_status_target",
            center: CAMERA_POS,
            x: CAMERA_X,
            y: CAMERA_Y,
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
        "raw_stream_cartridge_vault",
        "replay_controller_sled",
        "stream_lane_patch_panel",
        "timebase_event_order_bridge",
        "expected_observed_comparator_lanes",
        "checksum_hash_seal_strip",
        "anomaly_injection_token_tray",
        "audit_trail_custody_lands",
        "release_quarantine_disposition_gates",
        "evidence_camera_status_target",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for assumption in [
        "no_cell_replay_validation_before_culture_release",
        "raw_stream_custody_separate_from_replay_execution",
        "expected_observed_comparison_per_sensor_stream",
        "hash_manifest_and_event_order_evidence_stay_with_run_record",
        "mismatched_replay_streams_are_quarantined_before_release",
    ] {
        assert!(DESIGN_ASSUMPTIONS.contains(&assumption));
    }

    assert_eq!(STREAM_COUNT, STREAM_NAMES.len());
    assert_eq!(RAW_STREAM_CARTRIDGES, STREAM_COUNT);
    assert_eq!(RAW_STREAM_CARTRIDGES, CARTRIDGE_COLS * CARTRIDGE_ROWS);
    assert_eq!(WRITE_PROTECT_TABS, STREAM_COUNT);
    assert_eq!(PATCH_INPUT_PORTS, STREAM_COUNT);
    assert_eq!(PATCH_OUTPUT_PORTS, STREAM_COUNT);
    assert_eq!(PATCH_LOOPBACK_GAUGES, STREAM_COUNT);
    assert_eq!(EVENT_ORDER_CHANNELS, STREAM_COUNT);
    assert_eq!(TIMEBASE_STATUS_WINDOWS, STREAM_COUNT);
    assert_eq!(COMPARATOR_LANES, STREAM_COUNT);
    assert_eq!(EXPECTED_OBSERVED_SLOT_PAIRS, STREAM_COUNT);
    assert_eq!(RAW_HASH_LANDS, STREAM_COUNT);
    assert_eq!(REPLAY_HASH_LANDS, STREAM_COUNT);
    assert_eq!(MODE_TOKEN_WELLS, REPLAY_MODES.len());
    assert_eq!(RAW_STREAM_CUSTODY_LANDS, STREAM_COUNT);
    assert_eq!(DISPOSITION_GATES, GATE_NAMES.len());
    assert_eq!(GATE_TOKEN_SLOTS_PER_GATE, STREAM_COUNT);
    assert_eq!(CAMERA_FIDUCIALS, 4);
    assert_eq!(CAMERA_STATUS_WINDOWS, STREAM_COUNT);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);

    assert!(CARTRIDGE_POCKET_X + 10.0 < CARTRIDGE_PITCH_X);
    assert!(DATA_PORT_D + 10.0 < PATCH_PORT_PITCH_X);
    assert!(
        COMPARATOR_TICK_PITCH_X * (COMPARATOR_TICK_STATIONS as f64 - 1.0) < COMPARATOR_X - 90.0
    );
    assert!(HASH_LAND_X * 4.0 + 84.0 < CHECKSUM_X);
    assert!(TOKEN_WELL_D + 12.0 < TOKEN_PITCH_X);
    assert!(TIMEBASE_CLOCK_D + 70.0 < TIMEBASE_X);
    assert!(CONTROLLER_LIFT_CLEARANCE_Z > CONTROLLER_Z + 75.0);

    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds replay audit deck",
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

fn base_replay_audit_deck() -> Part {
    let deck = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wipe_recess = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_recessed_wipe_pan",
        STATION_X - 124.0,
        STATION_Y - 116.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.5);
    let front_cable_sump = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_front_data_cable_sump",
        STATION_X - 260.0,
        118.0,
        8.0,
    )
    .translate(0.0, -316.0, BASE_Z / 2.0 - 4.0);

    deck - wipe_recess - front_cable_sump - deck_module_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + row_divider_rails()
        + replay_flow_direction_ribs()
        + robot_datum_targets()
}

fn deck_module_sockets() -> Part {
    let footprints = station_footprints();
    let mut sockets = module_socket(footprints[0], 0);
    for (index, footprint) in footprints.iter().enumerate().skip(1) {
        sockets = sockets + module_socket(*footprint, index);
    }
    sockets
}

fn module_socket(footprint: Footprint, index: usize) -> Part {
    centered_cube(
        format!(
            "closed_sensor_stream_replay_audit_trail_station_{}_socket_{index}",
            footprint.name
        ),
        footprint.x + 10.0,
        footprint.y + 10.0,
        SOCKET_DEPTH,
    )
    .translate(
        footprint.center.0,
        footprint.center.1,
        BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
    )
}

fn deck_mounting_holes() -> Part {
    let positions = [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
    ];
    let mut holes = centered_cylinder(
        "closed_sensor_stream_replay_audit_trail_station_m6_mount_0",
        MOUNT_HOLE_D,
        BASE_Z + 2.0,
    )
    .translate(positions[0].0, positions[0].1, 0.0);
    for (index, (x, y)) in positions.iter().enumerate().skip(1) {
        holes = holes
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_m6_mount_{index}"),
                MOUNT_HOLE_D,
                BASE_Z + 2.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_front_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, on_deck_z(RIM_Z));
    let rear = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, on_deck_z(RIM_Z));
    let left = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, on_deck_z(RIM_Z));
    let right = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, on_deck_z(RIM_Z));

    front + rear + left + right
}

fn row_divider_rails() -> Part {
    let upper = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_top_middle_row_divider",
        STATION_X - 130.0,
        7.0,
        16.0,
    )
    .translate(0.0, 130.0, BASE_Z + 8.0);
    let lower = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_middle_bottom_row_divider",
        STATION_X - 130.0,
        7.0,
        16.0,
    )
    .translate(0.0, -160.0, BASE_Z + 8.0);
    upper + lower
}

fn replay_flow_direction_ribs() -> Part {
    let mut ribs = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_replay_flow_rib_0",
        72.0,
        6.0,
        9.0,
    )
    .translate(-340.0, 132.0, BASE_Z + 4.5);
    for index in 1..7 {
        ribs = ribs
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_replay_flow_rib_{index}"),
                72.0,
                6.0,
                9.0,
            )
            .translate(-340.0 + index as f64 * 112.0, 132.0, BASE_Z + 4.5);
    }
    ribs
}

fn robot_datum_targets() -> Part {
    let positions = [
        (-STATION_X / 2.0 + 92.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 92.0),
        (-STATION_X / 2.0 + 92.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
    ];
    let mut targets = centered_cylinder(
        "closed_sensor_stream_replay_audit_trail_station_robot_datum_target_0",
        22.0,
        5.0,
    )
    .translate(positions[0].0, positions[0].1, BASE_Z + 2.5);
    for (index, (x, y)) in positions.iter().enumerate().skip(1) {
        targets = targets
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_robot_datum_target_{index}"
                ),
                22.0,
                5.0,
            )
            .translate(*x, *y, BASE_Z + 2.5);
    }
    targets
}

fn raw_stream_cartridge_vault() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_raw_stream_vault_body",
        VAULT_X,
        VAULT_Y,
        VAULT_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_raw_stream_vault_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_raw_stream_vault_features");

    for cartridge in 0..RAW_STREAM_CARTRIDGES {
        let (x, y) = grid_position(
            cartridge,
            CARTRIDGE_COLS,
            CARTRIDGE_PITCH_X,
            CARTRIDGE_PITCH_Y,
        );
        let label = stream_label(cartridge);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_raw_stream_cartridge_pocket"
                ),
                CARTRIDGE_POCKET_X,
                CARTRIDGE_POCKET_Y,
                18.0,
            )
            .translate(x, y, VAULT_Z / 2.0 - 9.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_write_protect_tab"
                ),
                34.0,
                8.0,
                6.0,
            )
            .translate(x, y + 30.0, VAULT_Z / 2.0 + 3.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_custody_seal_boss"
                ),
                15.0,
                6.0,
            )
            .translate(x + 24.0, y - 28.0, VAULT_Z / 2.0 + 3.0);
    }

    for index in 0..4 {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_raw_vault_chain_of_custody_flag_{index}"
                ),
                52.0,
                12.0,
                6.0,
            )
            .translate(centered_index(index, 4, 70.0), -86.0, VAULT_Z / 2.0 + 3.0);
    }

    body - cuts + features
}

fn replay_controller_sled() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_replay_controller_sled_body",
        CONTROLLER_X,
        CONTROLLER_Y,
        CONTROLLER_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_replay_controller_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_replay_controller_features");

    for bay in 0..CONTROLLER_DRIVE_BAYS {
        cuts = cuts
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_replay_drive_bay_{bay}"),
                76.0,
                42.0,
                18.0,
            )
            .translate(
                centered_index(bay, CONTROLLER_DRIVE_BAYS, 94.0),
                48.0,
                CONTROLLER_Z / 2.0 - 9.0,
            );
    }

    for key in 0..PARSER_VERSION_KEYS {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_parser_version_key_land_{key}"
                ),
                54.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(key, PARSER_VERSION_KEYS, 68.0),
                -8.0,
                CONTROLLER_Z / 2.0 + 2.5,
            );
    }

    for land in 0..FIRMWARE_ID_LANDS {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_firmware_equipment_id_land_{land}"
                ),
                48.0,
                18.0,
                5.0,
            )
            .translate(centered_index(land, FIRMWARE_ID_LANDS, 68.0), -44.0, CONTROLLER_Z / 2.0 + 2.5);
    }

    for token in 0..EXECUTION_TOKEN_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_replay_execution_token_well_{token}"
                ),
                18.0,
                14.0,
            )
            .translate(centered_index(token, EXECUTION_TOKEN_WELLS, 52.0), -78.0, CONTROLLER_Z / 2.0 - 7.0);
    }

    body - cuts + features
}

fn stream_lane_patch_panel() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_stream_lane_patch_panel_body",
        PATCH_X,
        PATCH_Y,
        PATCH_Z,
    );
    let mut cuts = Part::empty("closed_sensor_stream_replay_audit_trail_station_patch_panel_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_patch_panel_features");

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream, STREAM_COUNT, PATCH_PORT_PITCH_X);
        let label = stream_label(stream);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_raw_input_port"),
                DATA_PORT_D,
                PATCH_Z + 8.0,
            )
            .translate(x, 50.0, 0.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_replay_output_port"
                ),
                DATA_PORT_D,
                PATCH_Z + 8.0,
            )
            .translate(x, -50.0, 0.0);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_loopback_gauge_slot"
                ),
                PATCH_LOOPBACK_SLOT_X,
                PATCH_LOOPBACK_SLOT_Y,
                5.0,
            )
            .translate(x, 0.0, PATCH_Z / 2.0 + 2.5);
    }

    for row in 0..3 {
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_patch_panel_row_label_land_{row}"),
                PATCH_X - 42.0,
                6.0,
                5.0,
            )
            .translate(0.0, centered_index(row, 3, 50.0), PATCH_Z / 2.0 + 2.5);
    }

    body - cuts + features
}

fn timebase_event_order_bridge() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_timebase_event_order_bridge_body",
        TIMEBASE_X,
        TIMEBASE_Y,
        TIMEBASE_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_timebase_bridge_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_timebase_bridge_features");

    features = features
        + centered_cylinder(
            "closed_sensor_stream_replay_audit_trail_station_reference_time_clock_face",
            TIMEBASE_CLOCK_D,
            6.0,
        )
        .translate(-55.0, 30.0, TIMEBASE_Z / 2.0 + 3.0);

    for port in 0..REFERENCE_TIME_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_reference_time_port_{port}"
                ),
                10.0,
                TIMEBASE_Z + 8.0,
            )
            .translate(
                centered_index(port, REFERENCE_TIME_PORTS, 34.0) - 55.0,
                -38.0,
                0.0,
            );
    }

    for tick in 0..ORDER_TICK_MARKS {
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_event_order_tick_{tick}"),
                4.0,
                20.0,
                5.0,
            )
            .translate(
                centered_index(tick, ORDER_TICK_MARKS, 12.0) + 56.0,
                52.0,
                TIMEBASE_Z / 2.0 + 2.5,
            );
    }

    for stream in 0..EVENT_ORDER_CHANNELS {
        let y = centered_index(stream, EVENT_ORDER_CHANNELS, 17.0) - 28.0;
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_event_order_channel_land"
                ),
                86.0,
                7.0,
                5.0,
            )
            .translate(58.0, y, TIMEBASE_Z / 2.0 + 2.5);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_timebase_status_window"
                ),
                16.0,
                9.0,
                12.0,
            )
            .translate(112.0, y, TIMEBASE_Z / 2.0 - 6.0);
    }

    body - cuts + features
}

fn expected_observed_comparator_lanes() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_expected_observed_comparator_body",
        COMPARATOR_X,
        COMPARATOR_Y,
        COMPARATOR_Z,
    );
    let mut cuts = Part::empty("closed_sensor_stream_replay_audit_trail_station_comparator_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_comparator_features");

    for lane in 0..COMPARATOR_LANES {
        let y = centered_index(lane, COMPARATOR_LANES, COMPARATOR_LANE_PITCH_Y);
        let label = stream_label(lane);
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_expected_lane"),
                166.0,
                6.0,
                5.0,
            )
            .translate(-104.0, y + 5.0, COMPARATOR_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_observed_lane"),
                166.0,
                6.0,
                5.0,
            )
            .translate(104.0, y - 5.0, COMPARATOR_Z / 2.0 + 2.5);

        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_expected_observed_delta_slot"
                ),
                32.0,
                14.0,
                10.0,
            )
            .translate(0.0, y, COMPARATOR_Z / 2.0 - 5.0);
    }

    for tick in 0..COMPARATOR_TICK_STATIONS {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_comparator_event_tick_station_{tick}"
                ),
                8.0,
                COMPARATOR_Y - 52.0,
                6.0,
            )
            .translate(centered_index(tick, COMPARATOR_TICK_STATIONS, COMPARATOR_TICK_PITCH_X), 0.0, COMPARATOR_Z / 2.0 + 3.0);
    }

    body - cuts + features
}

fn checksum_hash_seal_strip() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_checksum_hash_seal_strip_body",
        CHECKSUM_X,
        CHECKSUM_Y,
        CHECKSUM_Z,
    );
    let mut cuts = Part::empty("closed_sensor_stream_replay_audit_trail_station_hash_strip_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_hash_strip_features");

    for stream in 0..STREAM_COUNT {
        let x = centered_index(stream % 4, 4, 72.0);
        let y = centered_index(stream / 4, 2, 52.0) + 34.0;
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_raw_hash_land"),
                HASH_LAND_X,
                HASH_LAND_Y,
                5.0,
            )
            .translate(x - 76.0, y, CHECKSUM_Z / 2.0 + 2.5)
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_replay_hash_land"),
                HASH_LAND_X,
                HASH_LAND_Y,
                5.0,
            )
            .translate(x + 76.0, y, CHECKSUM_Z / 2.0 + 2.5);
    }

    for slot in 0..MANIFEST_KEY_SLOTS {
        cuts = cuts
            + centered_cube(
                format!(
                "closed_sensor_stream_replay_audit_trail_station_replay_manifest_key_slot_{slot}"
            ),
                44.0,
                12.0,
                9.0,
            )
            .translate(
                centered_index(slot, MANIFEST_KEY_SLOTS, 70.0),
                -36.0,
                CHECKSUM_Z / 2.0 - 4.5,
            );
    }

    for seal in 0..HASH_SEAL_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_hash_seal_well_{seal}"),
                13.0,
                CHECKSUM_Z + 6.0,
            )
            .translate(centered_index(seal, HASH_SEAL_WELLS, 62.0), -78.0, 0.0);
    }

    body - cuts + features
}

fn anomaly_injection_token_tray() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_anomaly_injection_token_tray_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts = Part::empty("closed_sensor_stream_replay_audit_trail_station_token_tray_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_token_tray_features");

    for mode in 0..MODE_TOKEN_WELLS {
        let label = replay_mode_label(mode);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_mode_token_well"),
                TOKEN_WELL_D,
                12.0,
            )
            .translate(
                centered_index(mode, MODE_TOKEN_WELLS, TOKEN_PITCH_X),
                58.0,
                TOKEN_Z / 2.0 - 6.0,
            );
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_mode_label_land"),
                38.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(mode, MODE_TOKEN_WELLS, TOKEN_PITCH_X),
                30.0,
                TOKEN_Z / 2.0 + 2.0,
            );
    }

    for seed in 0..SEED_TOKEN_WELLS {
        let (x, y) = grid_position(seed, 4, TOKEN_PITCH_X, TOKEN_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_seed_token_well_{seed}"),
                16.0,
                12.0,
            )
            .translate(x, y - 34.0, TOKEN_Z / 2.0 - 6.0);
    }

    for slot in 0..BLIND_BATCH_TOKEN_SLOTS {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_blind_batch_token_slot_{slot}"
                ),
                38.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(slot, BLIND_BATCH_TOKEN_SLOTS, 48.0),
                -86.0,
                TOKEN_Z / 2.0 + 2.5,
            );
    }

    body - cuts + features
}

fn audit_trail_custody_lands() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_audit_trail_custody_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_audit_custody_features");

    for land in 0..RUN_RECORD_LANDS {
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_run_record_land_{land}"),
                54.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(land, RUN_RECORD_LANDS, 66.0),
                56.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }

    for stream in 0..RAW_STREAM_CUSTODY_LANDS {
        let (x, y) = grid_position(stream, 4, 78.0, 42.0);
        let label = stream_label(stream);
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_raw_stream_custody_land"
                ),
                58.0,
                18.0,
                5.0,
            )
            .translate(x, y - 10.0, CUSTODY_Z / 2.0 + 2.5);
    }

    for land in 0..REPLAY_MANIFEST_LANDS {
        features = features
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_replay_manifest_land_{land}"
                ),
                56.0,
                16.0,
                5.0,
            )
            .translate(
                centered_index(land, REPLAY_MANIFEST_LANDS, 72.0),
                -58.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }

    for land in 0..OPERATOR_EQUIPMENT_VERSION_LANDS {
        features = features
            + centered_cylinder(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_operator_equipment_version_seal_{land}"
                ),
                16.0,
                5.0,
            )
            .translate(centered_index(land, OPERATOR_EQUIPMENT_VERSION_LANDS, 74.0), -82.0, CUSTODY_Z / 2.0 + 2.5);
    }

    body + features
}

fn release_quarantine_disposition_gates() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_disposition_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_disposition_gate_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_disposition_gate_features");

    for (gate, label) in GATE_NAMES.iter().enumerate() {
        let x = centered_index(gate, DISPOSITION_GATES, GATE_PITCH_X);
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_{label}_decision_lane"),
                GATE_PITCH_X - 18.0,
                8.0,
                8.0 + gate as f64 * 8.0,
            )
            .translate(x, 62.0, GATE_Z / 2.0 + 4.0 + gate as f64 * 4.0)
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_decision_label_land"
                ),
                72.0,
                17.0,
                4.0,
            )
            .translate(x, 78.0, GATE_Z / 2.0 + 2.0);

        for stream in 0..GATE_TOKEN_SLOTS_PER_GATE {
            let y = centered_index(stream, GATE_TOKEN_SLOTS_PER_GATE, 16.0) - 12.0;
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_sensor_stream_replay_audit_trail_station_{label}_{}_stream_disposition_token_slot",
                        stream_label(stream)
                    ),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    10.0,
                )
                .translate(x, y, GATE_Z / 2.0 - 5.0);
        }
    }

    body - cuts + features
}

fn evidence_camera_status_target() -> Part {
    let body = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_camera_status_target_body",
        CAMERA_X,
        CAMERA_Y,
        CAMERA_Z,
    );
    let mut cuts =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_camera_target_cuts");
    let mut features =
        Part::empty("closed_sensor_stream_replay_audit_trail_station_camera_target_features");

    let fiducials = [
        (-CAMERA_X / 2.0 + 34.0, -CAMERA_Y / 2.0 + 34.0),
        (CAMERA_X / 2.0 - 34.0, -CAMERA_Y / 2.0 + 34.0),
        (-CAMERA_X / 2.0 + 34.0, CAMERA_Y / 2.0 - 34.0),
        (CAMERA_X / 2.0 - 34.0, CAMERA_Y / 2.0 - 34.0),
    ];
    for (index, (x, y)) in fiducials.iter().enumerate() {
        features = features
            + centered_cylinder(
                format!("closed_sensor_stream_replay_audit_trail_station_camera_fiducial_{index}"),
                22.0,
                5.0,
            )
            .translate(*x, *y, CAMERA_Z / 2.0 + 2.5);
    }

    features = features
        + centered_cylinder(
            "closed_sensor_stream_replay_audit_trail_station_replay_status_clock_target",
            CAMERA_TARGET_D,
            5.0,
        )
        .translate(-48.0, 8.0, CAMERA_Z / 2.0 + 2.5);

    for tick in 0..CAMERA_CLOCK_TICKS {
        features = features
            + centered_cube(
                format!("closed_sensor_stream_replay_audit_trail_station_camera_clock_tick_{tick}"),
                4.0,
                18.0,
                5.0,
            )
            .translate(-103.0 + tick as f64 * 10.0, 8.0, CAMERA_Z / 2.0 + 5.0);
    }

    for stream in 0..CAMERA_STATUS_WINDOWS {
        let label = stream_label(stream);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_sensor_stream_replay_audit_trail_station_{label}_camera_status_window"
                ),
                17.0,
                11.0,
                10.0,
            )
            .translate(
                centered_index(stream, CAMERA_STATUS_WINDOWS, 26.0) + 66.0,
                -60.0,
                CAMERA_Z / 2.0 - 5.0,
            );
    }

    features = features
        + centered_cube(
            "closed_sensor_stream_replay_audit_trail_station_audit_hash_camera_land",
            104.0,
            34.0,
            5.0,
        )
        .translate(86.0, 42.0, CAMERA_Z / 2.0 + 2.5);

    body - cuts + features
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_front_robot_sweep_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -FRONT_ROBOT_SWEEP_CLEARANCE, BASE_Z + KEEP_OUT_Z / 2.0);
    let rear_data = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_rear_data_service_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, REAR_DATA_SERVICE_CLEARANCE, BASE_Z + KEEP_OUT_Z / 2.0);
    let left_cartridge = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_left_cartridge_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -LEFT_CARTRIDGE_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let right_audit = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_right_audit_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        RIGHT_AUDIT_SERVICE_CLEARANCE,
        0.0,
        BASE_Z + KEEP_OUT_Z / 2.0,
    );
    let controller_lift = centered_cube(
        "closed_sensor_stream_replay_audit_trail_station_controller_lift_clearance_gauge",
        CONTROLLER_X,
        CONTROLLER_Y,
        8.0,
    )
    .translate(
        CONTROLLER_POS.0,
        CONTROLLER_POS.1,
        BASE_Z + CONTROLLER_LIFT_CLEARANCE_Z,
    );

    front_robot + rear_data + left_cartridge + right_audit + controller_lift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_are_scoped_and_complete() {
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn replay_controls_cover_every_stream() {
        assert_eq!(RAW_STREAM_CARTRIDGES, STREAM_COUNT);
        assert_eq!(PATCH_INPUT_PORTS, STREAM_COUNT);
        assert_eq!(PATCH_OUTPUT_PORTS, STREAM_COUNT);
        assert_eq!(PATCH_LOOPBACK_GAUGES, STREAM_COUNT);
        assert_eq!(COMPARATOR_LANES, STREAM_COUNT);
        assert_eq!(RAW_HASH_LANDS, STREAM_COUNT);
        assert_eq!(REPLAY_HASH_LANDS, STREAM_COUNT);
        assert_eq!(RAW_STREAM_CUSTODY_LANDS, STREAM_COUNT);
    }

    #[test]
    fn custody_and_replay_are_separate_controls() {
        assert!(REQUIRED_FEATURES.contains(&"raw_stream_cartridge_vault"));
        assert!(REQUIRED_FEATURES.contains(&"replay_controller_sled"));
        assert!(REQUIRED_FEATURES.contains(&"audit_trail_custody_lands"));
        assert!(DESIGN_ASSUMPTIONS.contains(&"raw_stream_custody_separate_from_replay_execution"));
    }

    #[test]
    fn deck_modules_fit_without_overlap() {
        let footprints = station_footprints();
        for footprint in footprints {
            assert!(
                footprint.fits_inside_deck(),
                "{} does not fit",
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

    #[test]
    fn validation_scope_is_explicitly_limited() {
        for limitation in [
            "validation_fixture_only",
            "no_data_historian_software",
            "no_cryptographic_implementation",
            "no_cybersecurity_control_claim",
            "no_biological_performance_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }
    }

    #[test]
    fn full_design_constraint_passes() {
        assert_design_constraints();
    }
}
