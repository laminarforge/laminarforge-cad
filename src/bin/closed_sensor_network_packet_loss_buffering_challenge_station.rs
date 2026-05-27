use std::fs;

use vcad::{centered_cube, centered_cylinder as vcad_centered_cylinder, Part};

// Closed sensor network packet-loss buffering challenge station.
//
// Intent:
// - Package a no-cell validation fixture that challenges the sensor network
//   layer before tissue-chip runs depend on it for process evidence.
// - Keep controlled packet loss, jitter, gateway reboot, cable reseat, port
//   mirroring, local buffering, store-forward replay, timestamp/event beacons,
//   packet-gap witnesses, and quarantine/release decisions visible as physical
//   interfaces.
//
// Research assumptions encoded in the fixture:
// - Reproducible automated culture requires sensor data to remain attributable
//   to the correct chip lane and time window even when transport is imperfect.
// - Industrial monitoring practice separates transport challenge, local
//   buffering, replay verification, and final data-record release.
// - Packet loss should be validated without cells first, because network
//   failures can masquerade as biology drift after a run has started.
//
// This is validation fixture/interface CAD only. It does not define network
// firmware, cybersecurity controls, sterile processing instructions, packet
// loss acceptance limits, data-integrity certification, or biological
// performance claims.

const OUTPUT_PREFIX: &str = "output/closed_sensor_network_packet_loss_buffering_challenge_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_base_network_challenge_deck.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_deterministic_packet_loss_injector_panel.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_network_switch_port_mirror_bank.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_sensor_node_dock_array.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_buffer_queue_depth_cartridge_rack.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_store_forward_gateway_replay_sled.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_timestamp_event_beacon_bridge.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_packet_gap_witness_lanes.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_alarm_quarantine_release_gate.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_run_record_data_custody_lands.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_camera_evidence_status_target.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_robot_service_keepouts.stl",
    "output/closed_sensor_network_packet_loss_buffering_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_network_challenge_deck",
    "deterministic_packet_loss_injector_panel",
    "network_switch_port_mirror_bank",
    "sensor_node_dock_array",
    "buffer_queue_depth_cartridge_rack",
    "store_forward_gateway_replay_sled",
    "timestamp_event_beacon_bridge",
    "packet_gap_witness_lanes",
    "alarm_quarantine_release_gate",
    "run_record_data_custody_lands",
    "camera_evidence_status_target",
    "robot_service_keepouts",
];

const DESIGN_ASSUMPTIONS: [&str; 6] = [
    "no_cell_network_validation_before_culture_runs",
    "per_stream_sensor_transport_must_be_challenged",
    "local_buffering_separate_from_store_forward_gateway",
    "packet_gap_and_replay_evidence_stays_with_run_record",
    "late_or_missing_streams_are_quarantined_before_release",
    "robot_events_share_the_same_network_evidence_chain",
];

const LIMITATIONS: [&str; 7] = [
    "validation_fixture_only",
    "no_network_firmware",
    "no_cybersecurity_control_claim",
    "no_packet_loss_acceptance_thresholds",
    "no_sterile_barrier_claim",
    "no_data_integrity_certification",
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

const PACKET_LOSS_MODES: [&str; 6] = [
    "clean_link",
    "single_drop",
    "burst_drop",
    "jitter_delay",
    "gateway_reboot",
    "cable_reseat",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.5;
const MOUNT_HOLE_D: f64 = 6.8;
const PORT_BORE_D: f64 = 8.8;

const INJECTOR_POS: (f64, f64) = (-525.0, 250.0);
const INJECTOR_X: f64 = 300.0;
const INJECTOR_Y: f64 = 200.0;
const INJECTOR_Z: f64 = 52.0;
const DROP_MODE_DIALS: usize = PACKET_LOSS_MODES.len();
const RANDOM_SEED_TOKEN_WELLS: usize = 8;
const DROPOUT_WINDOW_SLOTS: usize = 5;
const MODE_DIAL_D: f64 = 24.0;
const DROPOUT_SLOT_X: f64 = 34.0;
const DROPOUT_SLOT_Y: f64 = 18.0;

const SWITCH_POS: (f64, f64) = (-170.0, 250.0);
const SWITCH_X: f64 = 300.0;
const SWITCH_Y: f64 = 200.0;
const SWITCH_Z: f64 = 48.0;
const SENSOR_PORTS: usize = STREAM_COUNT;
const MIRROR_PORTS: usize = 2;
const UPLINK_PORTS: usize = 2;
const TOTAL_SWITCH_PORTS: usize = SENSOR_PORTS + MIRROR_PORTS + UPLINK_PORTS;
const PORT_PITCH_X: f64 = 36.0;
const PORT_ROW_PITCH_Y: f64 = 54.0;

const NODE_POS: (f64, f64) = (225.0, 250.0);
const NODE_X: f64 = 450.0;
const NODE_Y: f64 = 200.0;
const NODE_Z: f64 = 44.0;
const NODE_DOCKS: usize = STREAM_COUNT;
const NODE_DOCK_X: f64 = 88.0;
const NODE_DOCK_Y: f64 = 30.0;
const NODE_DOCK_DEPTH: f64 = 18.0;
const NODE_RESET_PINS: usize = STREAM_COUNT;
const NODE_COLS: usize = 4;
const NODE_ROWS: usize = 2;

const BEACON_POS: (f64, f64) = (590.0, 250.0);
const BEACON_X: f64 = 230.0;
const BEACON_Y: f64 = 200.0;
const BEACON_Z: f64 = 80.0;
const TIME_REFERENCE_PORTS: usize = 3;
const EVENT_TRIGGER_PORTS: usize = STREAM_COUNT;
const STATUS_LED_WINDOWS: usize = STREAM_COUNT;
const BEACON_FACE_D: f64 = 92.0;

const BUFFER_POS: (f64, f64) = (-500.0, 0.0);
const BUFFER_X: f64 = 360.0;
const BUFFER_Y: f64 = 220.0;
const BUFFER_Z: f64 = 40.0;
const BUFFER_DEPTH_LEVELS: usize = 4;
const BUFFER_CARTRIDGES: usize = STREAM_COUNT;
const BUFFER_CARTRIDGE_X: f64 = 64.0;
const BUFFER_CARTRIDGE_Y: f64 = 30.0;
const BUFFER_CARTRIDGE_Z: f64 = 16.0;
const BUFFER_LEVEL_TICKS: usize = STREAM_COUNT * BUFFER_DEPTH_LEVELS;

const GATEWAY_POS: (f64, f64) = (-85.0, 0.0);
const GATEWAY_X: f64 = 430.0;
const GATEWAY_Y: f64 = 220.0;
const GATEWAY_Z: f64 = 46.0;
const GATEWAY_BAYS: usize = 3;
const REPLAY_TOKEN_SLOTS: usize = 6;
const STORE_FORWARD_DRIVE_SLOTS: usize = 4;
const REPLAY_BUTTON_WELLS: usize = 4;

const WITNESS_POS: (f64, f64) = (410.0, 0.0);
const WITNESS_X: f64 = 500.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 32.0;
const PACKET_GAP_LANES: usize = STREAM_COUNT;
const GAP_TICK_STATIONS: usize = 6;
const GAP_WITNESS_SLOTS: usize = PACKET_GAP_LANES * GAP_TICK_STATIONS;
const GAP_TICK_PITCH_X: f64 = 70.0;
const GAP_LANE_PITCH_Y: f64 = 23.0;

const GATE_POS: (f64, f64) = (-520.0, -270.0);
const GATE_X: f64 = 340.0;
const GATE_Y: f64 = 180.0;
const GATE_Z: f64 = 42.0;
const DECISION_GATES: usize = 3;
const GATE_NAMES: [&str; DECISION_GATES] = ["release", "retry", "quarantine"];
const GATE_TOKEN_SLOTS_PER_GATE: usize = STREAM_COUNT;
const GATE_PITCH_X: f64 = 96.0;
const GATE_SLOT_X: f64 = 22.0;
const GATE_SLOT_Y: f64 = 16.0;

const CUSTODY_POS: (f64, f64) = (-125.0, -270.0);
const CUSTODY_X: f64 = 410.0;
const CUSTODY_Y: f64 = 180.0;
const CUSTODY_Z: f64 = 18.0;
const RUN_RECORD_LANDS: usize = 5;
const RAW_PACKET_LOG_LANDS: usize = STREAM_COUNT;
const REPLAY_EVIDENCE_LANDS: usize = 4;
const AUDIT_EXPORT_SLOTS: usize = 4;

const CAMERA_POS: (f64, f64) = (300.0, -270.0);
const CAMERA_X: f64 = 300.0;
const CAMERA_Y: f64 = 180.0;
const CAMERA_Z: f64 = 34.0;
const CAMERA_FIDUCIALS: usize = 4;
const STATUS_TARGET_WINDOWS: usize = STREAM_COUNT;
const CAMERA_CLOCK_TICKS: usize = 12;
const CAMERA_TARGET_D: f64 = 84.0;

const KEEP_OUT_X: f64 = 1465.0;
const KEEP_OUT_Y: f64 = 840.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 405.0;
const REAR_NETWORK_SERVICE_CLEARANCE: f64 = 170.0;
const LEFT_CABLE_SERVICE_CLEARANCE: f64 = 165.0;
const RIGHT_DATA_SERVICE_CLEARANCE: f64 = 175.0;
const NODE_LIFT_CLEARANCE_Z: f64 = 135.0;

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

    let deck = base_network_challenge_deck();
    export(OUTPUTS[0], &deck);

    let injector = deterministic_packet_loss_injector_panel();
    export(OUTPUTS[1], &injector);

    let switch = network_switch_port_mirror_bank();
    export(OUTPUTS[2], &switch);

    let nodes = sensor_node_dock_array();
    export(OUTPUTS[3], &nodes);

    let buffers = buffer_queue_depth_cartridge_rack();
    export(OUTPUTS[4], &buffers);

    let gateway = store_forward_gateway_replay_sled();
    export(OUTPUTS[5], &gateway);

    let beacon = timestamp_event_beacon_bridge();
    export(OUTPUTS[6], &beacon);

    let witness = packet_gap_witness_lanes();
    export(OUTPUTS[7], &witness);

    let gates = alarm_quarantine_release_gate();
    export(OUTPUTS[8], &gates);

    let custody = run_record_data_custody_lands();
    export(OUTPUTS[9], &custody);

    let camera = camera_evidence_status_target();
    export(OUTPUTS[10], &camera);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + injector.translate(INJECTOR_POS.0, INJECTOR_POS.1, on_deck_z(INJECTOR_Z))
        + switch.translate(SWITCH_POS.0, SWITCH_POS.1, on_deck_z(SWITCH_Z))
        + nodes.translate(NODE_POS.0, NODE_POS.1, on_deck_z(NODE_Z))
        + buffers.translate(BUFFER_POS.0, BUFFER_POS.1, on_deck_z(BUFFER_Z))
        + gateway.translate(GATEWAY_POS.0, GATEWAY_POS.1, on_deck_z(GATEWAY_Z))
        + beacon.translate(BEACON_POS.0, BEACON_POS.1, on_deck_z(BEACON_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_Z))
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed sensor network packet-loss buffering challenge station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm no-cell network validation deck"
    );
    println!(
        "  Stream coverage:           {STREAM_COUNT} streams ({})",
        STREAM_NAMES.join(", ")
    );
    println!(
        "  Packet-loss challenge:     {DROP_MODE_DIALS} loss modes, {RANDOM_SEED_TOKEN_WELLS} seed wells, {DROPOUT_WINDOW_SLOTS} dropout window slots"
    );
    println!(
        "  Network fanout:            {SENSOR_PORTS} sensor ports, {MIRROR_PORTS} mirror ports, {UPLINK_PORTS} uplinks, {TOTAL_SWITCH_PORTS} total switch ports"
    );
    println!(
        "  Buffer/replay custody:     {BUFFER_CARTRIDGES} buffer cartridges, {BUFFER_DEPTH_LEVELS} queue-depth levels, {REPLAY_TOKEN_SLOTS} replay tokens, {STORE_FORWARD_DRIVE_SLOTS} drive slots"
    );
    println!(
        "  Gap witnesses:             {PACKET_GAP_LANES} lanes, {GAP_TICK_STATIONS} tick stations, {GAP_WITNESS_SLOTS} packet-gap witness slots"
    );
    println!(
        "  Release controls:          {} decision gates, {RUN_RECORD_LANDS} run-record lands, {RAW_PACKET_LOG_LANDS} raw packet-log lands",
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

fn packet_mode_label(index: usize) -> &'static str {
    PACKET_LOSS_MODES
        .get(index)
        .copied()
        .unwrap_or("unknown_packet_mode")
}

fn station_footprints() -> [Footprint; 10] {
    [
        Footprint {
            name: "deterministic_packet_loss_injector_panel",
            center: INJECTOR_POS,
            x: INJECTOR_X,
            y: INJECTOR_Y,
        },
        Footprint {
            name: "network_switch_port_mirror_bank",
            center: SWITCH_POS,
            x: SWITCH_X,
            y: SWITCH_Y,
        },
        Footprint {
            name: "sensor_node_dock_array",
            center: NODE_POS,
            x: NODE_X,
            y: NODE_Y,
        },
        Footprint {
            name: "timestamp_event_beacon_bridge",
            center: BEACON_POS,
            x: BEACON_X,
            y: BEACON_Y,
        },
        Footprint {
            name: "buffer_queue_depth_cartridge_rack",
            center: BUFFER_POS,
            x: BUFFER_X,
            y: BUFFER_Y,
        },
        Footprint {
            name: "store_forward_gateway_replay_sled",
            center: GATEWAY_POS,
            x: GATEWAY_X,
            y: GATEWAY_Y,
        },
        Footprint {
            name: "packet_gap_witness_lanes",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "alarm_quarantine_release_gate",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Footprint {
            name: "run_record_data_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "camera_evidence_status_target",
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
        "deterministic_packet_loss_injector_panel",
        "network_switch_port_mirror_bank",
        "sensor_node_dock_array",
        "buffer_queue_depth_cartridge_rack",
        "store_forward_gateway_replay_sled",
        "timestamp_event_beacon_bridge",
        "packet_gap_witness_lanes",
        "alarm_quarantine_release_gate",
        "run_record_data_custody_lands",
        "camera_evidence_status_target",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for assumption in [
        "no_cell_network_validation_before_culture_runs",
        "per_stream_sensor_transport_must_be_challenged",
        "local_buffering_separate_from_store_forward_gateway",
        "packet_gap_and_replay_evidence_stays_with_run_record",
        "late_or_missing_streams_are_quarantined_before_release",
    ] {
        assert!(DESIGN_ASSUMPTIONS.contains(&assumption));
    }

    assert_eq!(STREAM_COUNT, STREAM_NAMES.len());
    assert_eq!(DROP_MODE_DIALS, PACKET_LOSS_MODES.len());
    assert_eq!(SENSOR_PORTS, STREAM_COUNT);
    assert_eq!(
        TOTAL_SWITCH_PORTS,
        SENSOR_PORTS + MIRROR_PORTS + UPLINK_PORTS
    );
    assert_eq!(NODE_DOCKS, STREAM_COUNT);
    assert_eq!(NODE_DOCKS, NODE_COLS * NODE_ROWS);
    assert_eq!(NODE_RESET_PINS, STREAM_COUNT);
    assert_eq!(EVENT_TRIGGER_PORTS, STREAM_COUNT);
    assert_eq!(STATUS_LED_WINDOWS, STREAM_COUNT);
    assert_eq!(BUFFER_CARTRIDGES, STREAM_COUNT);
    assert_eq!(BUFFER_LEVEL_TICKS, STREAM_COUNT * BUFFER_DEPTH_LEVELS);
    assert_eq!(PACKET_GAP_LANES, STREAM_COUNT);
    assert_eq!(GAP_WITNESS_SLOTS, PACKET_GAP_LANES * GAP_TICK_STATIONS);
    assert_eq!(DECISION_GATES, GATE_NAMES.len());
    assert_eq!(GATE_TOKEN_SLOTS_PER_GATE, STREAM_COUNT);
    assert_eq!(RAW_PACKET_LOG_LANDS, STREAM_COUNT);
    assert_eq!(CAMERA_FIDUCIALS, 4);
    assert_eq!(STATUS_TARGET_WINDOWS, STREAM_COUNT);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);

    assert!(MODE_DIAL_D + 12.0 < INJECTOR_X / DROP_MODE_DIALS as f64);
    assert!(PORT_BORE_D + 12.0 < PORT_PITCH_X);
    assert!(NODE_DOCK_X * NODE_COLS as f64 <= NODE_X - 64.0);
    assert!(BUFFER_CARTRIDGE_X * 2.0 + 60.0 < BUFFER_X);
    assert!(GAP_TICK_PITCH_X * (GAP_TICK_STATIONS as f64 - 1.0) < WITNESS_X - 88.0);
    assert!(GAP_LANE_PITCH_Y < WITNESS_Y / PACKET_GAP_LANES as f64 + 3.0);
    assert!(CAMERA_TARGET_D + 70.0 < CAMERA_X);
    assert!(NODE_LIFT_CLEARANCE_Z > NODE_Z + 80.0);

    let footprints = station_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds packet-loss buffering deck",
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

fn base_network_challenge_deck() -> Part {
    let deck = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wipe_recess = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_recessed_wipe_pan",
        STATION_X - 122.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.5);
    let cable_sump = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_front_cable_sump",
        STATION_X - 240.0,
        120.0,
        8.0,
    )
    .translate(0.0, -306.0, BASE_Z / 2.0 - 4.0);

    deck - wipe_recess - cable_sump - deck_module_sockets() - deck_mounting_holes()
        + perimeter_rims()
        + row_divider_rails()
        + packet_flow_direction_ribs()
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
            "closed_sensor_network_packet_loss_buffering_challenge_station_{}_socket_{index}",
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
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (0.0, -STATION_Y / 2.0 + 52.0),
    ];
    let mut holes = centered_cylinder(
        "closed_sensor_network_packet_loss_buffering_challenge_station_m6_mount_0",
        MOUNT_HOLE_D,
        BASE_Z + 2.0,
    )
    .translate(positions[0].0, positions[0].1, 0.0);
    for (index, (x, y)) in positions.iter().enumerate().skip(1) {
        holes = holes
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_m6_mount_{index}"
                ),
                MOUNT_HOLE_D,
                BASE_Z + 2.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_front_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, on_deck_z(RIM_Z));
    let rear = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, on_deck_z(RIM_Z));
    let left = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, on_deck_z(RIM_Z));
    let right = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, on_deck_z(RIM_Z));
    front + rear + left + right
}

fn row_divider_rails() -> Part {
    let top = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_top_row_divider",
        STATION_X - 170.0,
        10.0,
        24.0,
    )
    .translate(0.0, 126.0, on_deck_z(24.0));
    let bottom = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_bottom_row_divider",
        STATION_X - 170.0,
        10.0,
        24.0,
    )
    .translate(0.0, -142.0, on_deck_z(24.0));
    let center = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_packet_flow_spine",
        12.0,
        STATION_Y - 172.0,
        22.0,
    )
    .translate(80.0, 0.0, on_deck_z(22.0));
    top + bottom + center
}

fn packet_flow_direction_ribs() -> Part {
    let mut ribs = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_flow_arrow_rib_0",
        74.0,
        8.0,
        10.0,
    )
    .translate(-600.0, 112.0, on_deck_z(10.0));
    for index in 1..7 {
        ribs = ribs
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_flow_arrow_rib_{index}"
                ),
                74.0,
                8.0,
                10.0,
            )
            .translate(-600.0 + index as f64 * 190.0, 112.0, on_deck_z(10.0));
    }
    ribs
}

fn robot_datum_targets() -> Part {
    let positions = [
        (-STATION_X / 2.0 + 92.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
        (-STATION_X / 2.0 + 92.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 92.0),
    ];
    let mut targets = centered_cylinder(
        "closed_sensor_network_packet_loss_buffering_challenge_station_robot_datum_ring_0",
        36.0,
        5.0,
    )
    .translate(positions[0].0, positions[0].1, BASE_Z / 2.0 + 2.5)
        - centered_cylinder(
            "closed_sensor_network_packet_loss_buffering_challenge_station_robot_datum_bore_0",
            13.0,
            7.0,
        )
        .translate(positions[0].0, positions[0].1, BASE_Z / 2.0 + 2.5);
    for (index, (x, y)) in positions.iter().enumerate().skip(1) {
        targets = targets
            + (centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_robot_datum_ring_{index}"
                ),
                36.0,
                5.0,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.5)
                - centered_cylinder(
                    format!(
                        "closed_sensor_network_packet_loss_buffering_challenge_station_robot_datum_bore_{index}"
                    ),
                    13.0,
                    7.0,
                )
                .translate(*x, *y, BASE_Z / 2.0 + 2.5));
    }
    targets
}

fn deterministic_packet_loss_injector_panel() -> Part {
    let mut panel = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_injector_body",
        INJECTOR_X,
        INJECTOR_Y,
        INJECTOR_Z,
    );
    for index in 0..DROP_MODE_DIALS {
        let x = centered_index(index, DROP_MODE_DIALS, 42.0);
        let label = packet_mode_label(index);
        panel = panel
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_mode_dial"
                ),
                MODE_DIAL_D,
                7.0,
            )
            .translate(x, 58.0, INJECTOR_Z / 2.0 + 3.5)
            - centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_detent_bore"
                ),
                MODE_DIAL_D - 10.0,
                9.0,
            )
            .translate(x, 58.0, INJECTOR_Z / 2.0 + 3.5);
    }
    for index in 0..RANDOM_SEED_TOKEN_WELLS {
        let (x, y) = grid_position(index, 4, 2, 58.0, 42.0);
        panel = panel
            - centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_seed_token_well_{index}"
                ),
                21.0,
                12.0,
            )
            .translate(x, y - 18.0, INJECTOR_Z / 2.0 - 6.0);
    }
    for index in 0..DROPOUT_WINDOW_SLOTS {
        panel = panel
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_dropout_window_slot_{index}"
                ),
                DROPOUT_SLOT_X,
                DROPOUT_SLOT_Y,
                12.0,
            )
            .translate(centered_index(index, DROPOUT_WINDOW_SLOTS, 47.0), -74.0, INJECTOR_Z / 2.0 - 6.0);
    }
    panel
}

fn network_switch_port_mirror_bank() -> Part {
    let mut bank = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_switch_body",
        SWITCH_X,
        SWITCH_Y,
        SWITCH_Z,
    );
    for index in 0..TOTAL_SWITCH_PORTS {
        let row = index / 6;
        let col = index % 6;
        let x = centered_index(col, 6, PORT_PITCH_X);
        let y = centered_index(row, 2, PORT_ROW_PITCH_Y);
        bank = bank
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_switch_port_bore_{index}"
                ),
                20.0,
                13.0,
                16.0,
            )
            .translate(x, y, SWITCH_Z / 2.0 - 8.0)
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_port_status_led_{index}"
                ),
                7.0,
                4.0,
            )
            .translate(x, y + 18.0, SWITCH_Z / 2.0 + 2.0);
    }
    for index in 0..MIRROR_PORTS {
        bank = bank
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_mirror_capture_label_land_{index}"
                ),
                52.0,
                16.0,
                5.0,
            )
            .translate(centered_index(index, MIRROR_PORTS, 74.0), -80.0, SWITCH_Z / 2.0 + 2.5);
    }
    bank
}

fn sensor_node_dock_array() -> Part {
    let mut docks = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_node_dock_base",
        NODE_X,
        NODE_Y,
        NODE_Z,
    );
    for index in 0..NODE_DOCKS {
        let (x, y) = grid_position(index, NODE_COLS, NODE_ROWS, 100.0, 76.0);
        let label = stream_label(index);
        docks = docks
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_node_pocket"
                ),
                NODE_DOCK_X,
                NODE_DOCK_Y,
                NODE_DOCK_DEPTH,
            )
            .translate(x, y, NODE_Z / 2.0 - NODE_DOCK_DEPTH / 2.0)
            - centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_reset_pin_access"
                ),
                5.2,
                10.0,
            )
            .translate(x + 34.0, y, NODE_Z / 2.0 - 5.0)
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_barcode_land"
                ),
                54.0,
                10.0,
                4.0,
            )
            .translate(x, y - 24.0, NODE_Z / 2.0 + 2.0);
    }
    docks
}

fn buffer_queue_depth_cartridge_rack() -> Part {
    let mut rack = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_buffer_rack_body",
        BUFFER_X,
        BUFFER_Y,
        BUFFER_Z,
    );
    for index in 0..BUFFER_CARTRIDGES {
        let (x, y) = grid_position(index, 4, 2, 78.0, 74.0);
        let label = stream_label(index);
        rack = rack
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_buffer_cartridge_socket"
                ),
                BUFFER_CARTRIDGE_X,
                BUFFER_CARTRIDGE_Y,
                BUFFER_CARTRIDGE_Z,
            )
            .translate(x, y, BUFFER_Z / 2.0 - BUFFER_CARTRIDGE_Z / 2.0);
        for level in 0..BUFFER_DEPTH_LEVELS {
            rack = rack
                + centered_cube(
                    format!(
                        "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_queue_depth_tick_{level}"
                    ),
                    8.0,
                    6.0,
                    5.0,
                )
                .translate(x - 28.0 + level as f64 * 18.0, y + 26.0, BUFFER_Z / 2.0 + 2.5);
        }
    }
    rack
}

fn store_forward_gateway_replay_sled() -> Part {
    let mut sled = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_gateway_replay_sled",
        GATEWAY_X,
        GATEWAY_Y,
        GATEWAY_Z,
    );
    for index in 0..GATEWAY_BAYS {
        sled = sled
            - centered_cube(
                format!(
                "closed_sensor_network_packet_loss_buffering_challenge_station_gateway_bay_{index}"
            ),
                98.0,
                70.0,
                22.0,
            )
            .translate(
                centered_index(index, GATEWAY_BAYS, 128.0),
                44.0,
                GATEWAY_Z / 2.0 - 11.0,
            );
    }
    for index in 0..REPLAY_TOKEN_SLOTS {
        sled = sled
            - centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_replay_token_slot_{index}"
                ),
                23.0,
                12.0,
            )
            .translate(centered_index(index, REPLAY_TOKEN_SLOTS, 54.0), -44.0, GATEWAY_Z / 2.0 - 6.0);
    }
    for index in 0..STORE_FORWARD_DRIVE_SLOTS {
        sled = sled
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_store_forward_drive_slot_{index}"
                ),
                58.0,
                24.0,
                14.0,
            )
            .translate(centered_index(index, STORE_FORWARD_DRIVE_SLOTS, 74.0), -84.0, GATEWAY_Z / 2.0 - 7.0);
    }
    for index in 0..REPLAY_BUTTON_WELLS {
        sled = sled
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_replay_button_guard_{index}"
                ),
                28.0,
                5.0,
            )
            .translate(centered_index(index, REPLAY_BUTTON_WELLS, 64.0), 92.0, GATEWAY_Z / 2.0 + 2.5);
    }
    sled
}

fn timestamp_event_beacon_bridge() -> Part {
    let left_post = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_beacon_left_post",
        24.0,
        24.0,
        BEACON_Z,
    )
    .translate(-BEACON_X / 2.0 + 30.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_beacon_right_post",
        24.0,
        24.0,
        BEACON_Z,
    )
    .translate(BEACON_X / 2.0 - 30.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_beacon_cross_beam",
        BEACON_X,
        24.0,
        22.0,
    )
    .translate(0.0, 0.0, BEACON_Z / 2.0 - 11.0);
    let mut bridge = left_post + right_post + beam;
    for index in 0..TIME_REFERENCE_PORTS {
        bridge = bridge
            - centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_time_reference_port_{index}"
                ),
                13.0,
                12.0,
            )
            .translate(centered_index(index, TIME_REFERENCE_PORTS, 54.0), 0.0, BEACON_Z / 2.0 - 11.0);
    }
    for index in 0..EVENT_TRIGGER_PORTS {
        bridge = bridge
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_event_trigger_window_{index}"
                ),
                16.0,
                8.0,
                10.0,
            )
            .translate(centered_index(index, EVENT_TRIGGER_PORTS, 24.0), -48.0, BEACON_Z / 2.0 - 10.0);
    }
    bridge
        + centered_cylinder(
            "closed_sensor_network_packet_loss_buffering_challenge_station_beacon_face",
            BEACON_FACE_D,
            6.0,
        )
        .translate(0.0, 58.0, BEACON_Z / 2.0)
}

fn packet_gap_witness_lanes() -> Part {
    let mut lanes = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_gap_witness_plate",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    for lane in 0..PACKET_GAP_LANES {
        let y = centered_index(lane, PACKET_GAP_LANES, GAP_LANE_PITCH_Y);
        let label = stream_label(lane);
        lanes = lanes
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_gap_lane_rib"
                ),
                WITNESS_X - 64.0,
                5.0,
                7.0,
            )
            .translate(0.0, y, WITNESS_Z / 2.0 + 3.5);
        for tick in 0..GAP_TICK_STATIONS {
            lanes = lanes
                - centered_cube(
                    format!(
                        "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_missing_packet_slot_{tick}"
                    ),
                    26.0,
                    13.0,
                    12.0,
                )
                .translate(centered_index(tick, GAP_TICK_STATIONS, GAP_TICK_PITCH_X), y, WITNESS_Z / 2.0 - 6.0);
        }
    }
    lanes
}

fn alarm_quarantine_release_gate() -> Part {
    let mut gates = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_alarm_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    for gate in 0..DECISION_GATES {
        let gate_x = centered_index(gate, DECISION_GATES, GATE_PITCH_X);
        let label = GATE_NAMES[gate];
        gates = gates
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_gate_label_land"
                ),
                70.0,
                18.0,
                5.0,
            )
            .translate(gate_x, 62.0, GATE_Z / 2.0 + 2.5);
        for slot in 0..GATE_TOKEN_SLOTS_PER_GATE {
            gates = gates
                - centered_cube(
                    format!(
                        "closed_sensor_network_packet_loss_buffering_challenge_station_{label}_stream_token_slot_{slot}"
                    ),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    12.0,
                )
                .translate(gate_x, centered_index(slot, GATE_TOKEN_SLOTS_PER_GATE, 18.0) - 8.0, GATE_Z / 2.0 - 6.0);
        }
    }
    gates
}

fn run_record_data_custody_lands() -> Part {
    let mut custody = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_custody_land_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    for index in 0..RUN_RECORD_LANDS {
        custody = custody
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_run_record_land_{index}"
                ),
                58.0,
                24.0,
                5.0,
            )
            .translate(centered_index(index, RUN_RECORD_LANDS, 70.0), 56.0, CUSTODY_Z / 2.0 + 2.5);
    }
    for index in 0..RAW_PACKET_LOG_LANDS {
        let (x, y) = grid_position(index, 4, 2, 82.0, 44.0);
        custody = custody
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_{}_raw_packet_log_land",
                    stream_label(index)
                ),
                64.0,
                20.0,
                5.0,
            )
            .translate(x, y - 12.0, CUSTODY_Z / 2.0 + 2.5);
    }
    for index in 0..REPLAY_EVIDENCE_LANDS {
        custody = custody
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_replay_evidence_seal_well_{index}"
                ),
                18.0,
                5.0,
            )
            .translate(centered_index(index, REPLAY_EVIDENCE_LANDS, 74.0), -72.0, CUSTODY_Z / 2.0 + 2.5);
    }
    for index in 0..AUDIT_EXPORT_SLOTS {
        custody = custody
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_audit_export_slot_{index}"
                ),
                42.0,
                11.0,
                8.0,
            )
            .translate(centered_index(index, AUDIT_EXPORT_SLOTS, 56.0), -42.0, CUSTODY_Z / 2.0 - 4.0);
    }
    custody
}

fn camera_evidence_status_target() -> Part {
    let mut target = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_camera_status_target_plate",
        CAMERA_X,
        CAMERA_Y,
        CAMERA_Z,
    );
    let fiducials = [
        (-CAMERA_X / 2.0 + 34.0, -CAMERA_Y / 2.0 + 34.0),
        (CAMERA_X / 2.0 - 34.0, -CAMERA_Y / 2.0 + 34.0),
        (-CAMERA_X / 2.0 + 34.0, CAMERA_Y / 2.0 - 34.0),
        (CAMERA_X / 2.0 - 34.0, CAMERA_Y / 2.0 - 34.0),
    ];
    for (index, (x, y)) in fiducials.iter().enumerate() {
        target = target
            + centered_cylinder(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_camera_fiducial_{index}"
                ),
                22.0,
                5.0,
            )
            .translate(*x, *y, CAMERA_Z / 2.0 + 2.5);
    }
    target = target
        + centered_cylinder(
            "closed_sensor_network_packet_loss_buffering_challenge_station_status_clock_target",
            CAMERA_TARGET_D,
            5.0,
        )
        .translate(0.0, 8.0, CAMERA_Z / 2.0 + 2.5);
    for index in 0..CAMERA_CLOCK_TICKS {
        let x = (index as f64 - 5.5) * 10.0;
        target = target
            + centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_camera_clock_tick_{index}"
                ),
                4.0,
                20.0,
                6.0,
            )
            .translate(x, 8.0, CAMERA_Z / 2.0 + 5.5);
    }
    for index in 0..STATUS_TARGET_WINDOWS {
        target = target
            - centered_cube(
                format!(
                    "closed_sensor_network_packet_loss_buffering_challenge_station_status_window_{index}"
                ),
                18.0,
                12.0,
                11.0,
            )
            .translate(centered_index(index, STATUS_TARGET_WINDOWS, 28.0), -62.0, CAMERA_Z / 2.0 - 5.5);
    }
    target
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_front_robot_sweep_keepout",
        KEEP_OUT_X,
        6.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -FRONT_ROBOT_SWEEP_CLEARANCE,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_rear_network_service_keepout",
        KEEP_OUT_X,
        6.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, REAR_NETWORK_SERVICE_CLEARANCE, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_left_cable_service_keepout",
        6.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -LEFT_CABLE_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_right_data_service_keepout",
        6.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        RIGHT_DATA_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + KEEP_OUT_Z / 2.0,
    );
    let lift = centered_cube(
        "closed_sensor_network_packet_loss_buffering_challenge_station_node_lift_clearance_gauge",
        NODE_X,
        NODE_Y,
        8.0,
    )
    .translate(NODE_POS.0, NODE_POS.1, BASE_Z / 2.0 + NODE_LIFT_CLEARANCE_Z);
    front + rear + left + right + lift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_are_feature_complete() {
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn packet_loss_controls_are_per_stream() {
        assert_eq!(SENSOR_PORTS, STREAM_COUNT);
        assert_eq!(NODE_DOCKS, STREAM_COUNT);
        assert_eq!(BUFFER_CARTRIDGES, STREAM_COUNT);
        assert_eq!(PACKET_GAP_LANES, STREAM_COUNT);
        assert_eq!(RAW_PACKET_LOG_LANDS, STREAM_COUNT);
        assert_eq!(STATUS_TARGET_WINDOWS, STREAM_COUNT);
    }

    #[test]
    fn buffering_and_replay_are_separate_controls() {
        assert!(REQUIRED_FEATURES.contains(&"buffer_queue_depth_cartridge_rack"));
        assert!(REQUIRED_FEATURES.contains(&"store_forward_gateway_replay_sled"));
        assert!(DESIGN_ASSUMPTIONS.contains(&"local_buffering_separate_from_store_forward_gateway"));
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
    fn design_scope_is_explicitly_limited() {
        for limitation in [
            "validation_fixture_only",
            "no_network_firmware",
            "no_cybersecurity_control_claim",
            "no_packet_loss_acceptance_thresholds",
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
