use std::collections::BTreeSet;
use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent/cell-media cold-chain unload, warmup-excursion, and release station.
//
// Design intent:
// - Dock sealed chilled totes at the clean-enclosure unload boundary without
//   exposing reagent or cell-media containers to open-bench handling.
// - Make warmup excursion, logger custody, lot identity, CoA evidence,
//   connector wetness, quarantine/release status, and robot/service clearances
//   physically visible before any material is released.
// - Keep chilled, suspect, warmup, clean-release, and used-witness regions
//   mechanically segregated for validation review.
//
// Mechanical concept CAD only. This file models fixture geometry for planning
// and validation discussion; it is not a cold-chain release rule, GMP procedure,
// sterility claim, thermal qualification, or biological acceptance criterion.

const OUTPUT_PREFIX: &str = "output/closed_reagent_cold_chain_unload_warmup_excursion_station";
const PART_PREFIX: &str = "closed_reagent_cold_chain_unload_warmup_excursion_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_containment_drain_deck.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_chilled_tote_docks.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_temperature_logger_pockets.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_warmup_clock_token_rails.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_quarantine_release_lane_gate.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_barcode_coa_custody_plate.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_connector_wetness_witness_pads.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_clean_enclosure_transfer_port_gauge.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_release_audit_token_caddy.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_robot_service_gauges.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_evidence_camera_bridge.stl",
    "output/closed_reagent_cold_chain_unload_warmup_excursion_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 10] = [
    "chilled_tote_docks",
    "temperature_logger_pockets",
    "warmup_clock_token_rails",
    "quarantine_release_lanes",
    "barcode_coa_custody_plate",
    "connector_wetness_witness_pads",
    "clean_enclosure_transfer_port_gauge",
    "release_audit_token_caddy",
    "evidence_camera_bridge",
    "robot_service_gauges",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "no_release_rule",
    "no_biological_acceptance_criteria",
    "not_a_sterile_barrier_design",
    "purchased_wetted_components_external",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 46.0;
const SUMP_X: f64 = 1360.0;
const SUMP_Y: f64 = 768.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_BOSSES: usize = 8;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.8;

const TOTE_POS: (f64, f64) = (-430.0, 205.0);
const TOTE_PANEL_X: f64 = 560.0;
const TOTE_PANEL_Y: f64 = 320.0;
const TOTE_PANEL_Z: f64 = 58.0;
const CHILLED_TOTE_DOCKS: usize = 2;
const TOTE_DOCK_X: f64 = 206.0;
const TOTE_DOCK_Y: f64 = 236.0;
const TOTE_DOCK_DEPTH: f64 = 18.0;
const TOTE_DOCK_PITCH_X: f64 = 260.0;
const TOTE_RAIL_Z: f64 = 46.0;
const TOTE_LATCH_LANDS: usize = 8;
const THERMAL_CONTACT_RIBS_PER_DOCK: usize = 5;
const COLD_REFERENCE_PUCKS: usize = 4;

const LOGGER_POS: (f64, f64) = (-535.0, -250.0);
const LOGGER_PANEL_X: f64 = 360.0;
const LOGGER_PANEL_Y: f64 = 170.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_POCKETS: usize = 6;
const LOGGER_SLOT_X: f64 = 48.0;
const LOGGER_SLOT_Y: f64 = 92.0;
const LOGGER_SLOT_DEPTH: f64 = 11.0;
const LOGGER_SLOT_PITCH_X: f64 = 56.0;
const LOGGER_SEAL_WELLS: usize = 8;
const LOGGER_POGO_PINS_PER_POCKET: usize = 3;
const LOGGER_POGO_PINS: usize = LOGGER_POCKETS * LOGGER_POGO_PINS_PER_POCKET;

const WARMUP_POS: (f64, f64) = (0.0, -374.0);
const WARMUP_RAIL_X: f64 = 680.0;
const WARMUP_RAIL_Y: f64 = 96.0;
const WARMUP_RAIL_Z: f64 = 30.0;
const WARMUP_LANES: usize = 3;
const WARMUP_TOKEN_SLOTS_PER_LANE: usize = 4;
const WARMUP_TOKEN_SLOTS: usize = WARMUP_LANES * WARMUP_TOKEN_SLOTS_PER_LANE;
const WARMUP_SLOT_X: f64 = 58.0;
const WARMUP_SLOT_Y: f64 = 22.0;
const WARMUP_SLOT_DEPTH: f64 = 8.0;
const WARMUP_SLOT_PITCH_X: f64 = 128.0;
const WARMUP_LANE_PITCH_Y: f64 = 28.0;
const WARMUP_TICK_MARKS: usize = WARMUP_TOKEN_SLOTS + 1;
const EXCURSION_DIALS: usize = 6;

const LANE_POS: (f64, f64) = (200.0, 170.0);
const LANE_PANEL_X: f64 = 560.0;
const LANE_PANEL_Y: f64 = 320.0;
const LANE_PANEL_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["quarantine", "warmup_hold", "release"];
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 100.0;
const STATUS_SLOT_Y: f64 = 48.0;
const STATUS_SLOT_DEPTH: f64 = 8.0;
const STATUS_LANE_PITCH_X: f64 = 158.0;
const STATUS_SLOT_PITCH_Y: f64 = 64.0;
const LANE_DIVIDER_W: f64 = 12.0;
const LANE_INTERLOCK_SHUTTERS: usize = STATUS_LANES;
const LANE_MIN_GAP: f64 = STATUS_LANE_PITCH_X - STATUS_SLOT_X;

const CUSTODY_POS: (f64, f64) = (545.0, -205.0);
const CUSTODY_PLATE_X: f64 = 340.0;
const CUSTODY_PLATE_Y: f64 = 220.0;
const CUSTODY_PLATE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 8;
const COA_CARD_LANDS: usize = 4;
const RFID_CHIP_WELLS: usize = 4;
const LOT_TOKEN_WELLS: usize = 6;

const WETNESS_POS: (f64, f64) = (-125.0, -130.0);
const WETNESS_BLOCK_X: f64 = 360.0;
const WETNESS_BLOCK_Y: f64 = 170.0;
const WETNESS_BLOCK_Z: f64 = 34.0;
const WETNESS_PAD_COUNT: usize = 10;
const WETNESS_PAD_COLS: usize = 5;
const WETNESS_PAD_ROWS: usize = 2;
const WETNESS_PAD_X: f64 = 42.0;
const WETNESS_PAD_Y: f64 = 34.0;
const WETNESS_PAD_DEPTH: f64 = 10.0;
const DRY_CONTROL_PADS: usize = 2;
const CONNECTOR_SADDLES: usize = 6;

const TRANSFER_POS: (f64, f64) = (600.0, 115.0);
const TRANSFER_PANEL_X: f64 = 240.0;
const TRANSFER_PANEL_Y: f64 = 260.0;
const TRANSFER_PANEL_Z: f64 = 28.0;
const TRANSFER_GATE_X: f64 = 218.0;
const TRANSFER_GATE_Y: f64 = 30.0;
const TRANSFER_GATE_Z: f64 = 162.0;
const PASS_WINDOW_X: f64 = 132.0;
const PASS_WINDOW_Z: f64 = 76.0;
const TRANSFER_LATCHES: usize = 6;
const PRESSURE_EQUALIZATION_PORTS: usize = 4;

const AUDIT_POS: (f64, f64) = (520.0, 380.0);
const AUDIT_CADDY_X: f64 = 390.0;
const AUDIT_CADDY_Y: f64 = 80.0;
const AUDIT_CADDY_Z: f64 = 26.0;
const RELEASE_AUDIT_TOKENS: usize = 9;
const RELEASE_LOCK_PINS: usize = 6;

const BRIDGE_POS: (f64, f64) = (0.0, 392.0);
const BRIDGE_SPAN_X: f64 = 1220.0;
const BRIDGE_POST_X: f64 = 28.0;
const BRIDGE_POST_Y: f64 = 40.0;
const BRIDGE_POST_Z: f64 = 190.0;
const BRIDGE_BEAM_Y: f64 = 42.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 138.0;

const ROBOT_FRONT_APPROACH_Y: f64 = 430.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 270.0;
const LEFT_TOTE_SERVICE_X: f64 = 250.0;
const RIGHT_RELEASE_SERVICE_X: f64 = 235.0;
const TOP_BRIDGE_SERVICE_Z: f64 = 330.0;
const ROBOT_SWEEP_X: f64 = 1180.0;
const ROBOT_SWEEP_Y: f64 = 760.0;
const ROBOT_SWEEP_Z: f64 = 172.0;
const GRIPPER_CLEARANCE_POSTS: usize = 6;
const KEEP_OUT_RAIL_Z: f64 = 8.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_drain_deck();
    export(&deck, OUTPUTS[0]);

    let totes = chilled_tote_docks();
    export(&totes, OUTPUTS[1]);

    let loggers = temperature_logger_pockets();
    export(&loggers, OUTPUTS[2]);

    let warmup = warmup_clock_token_rails();
    export(&warmup, OUTPUTS[3]);

    let lanes = quarantine_release_lane_gate();
    export(&lanes, OUTPUTS[4]);

    let custody = barcode_coa_custody_plate();
    export(&custody, OUTPUTS[5]);

    let wetness = connector_wetness_witness_pads();
    export(&wetness, OUTPUTS[6]);

    let transfer = clean_enclosure_transfer_port_gauge();
    export(&transfer, OUTPUTS[7]);

    let audit = release_audit_token_caddy();
    export(&audit, OUTPUTS[8]);

    let gauges = robot_service_gauges();
    export(&gauges, OUTPUTS[9]);

    let bridge = evidence_camera_bridge();
    export(&bridge, OUTPUTS[10]);

    let assembly = station_assembly();
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed reagent cold-chain unload warmup excursion station:");
    println!(
        "  Containment deck:        {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm sump, {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Chilled tote docks:      {CHILLED_TOTE_DOCKS} sealed tote docks, {TOTE_LATCH_LANDS} latch lands, {COLD_REFERENCE_PUCKS} cold-reference puck pockets"
    );
    println!(
        "  Temperature custody:     {LOGGER_POCKETS} logger pockets, {LOGGER_SEAL_WELLS} seal wells, {LOGGER_POGO_PINS} pogo/contact pins"
    );
    println!(
        "  Warmup excursion proof:  {WARMUP_LANES} clock lanes, {WARMUP_TOKEN_SLOTS} token slots, {WARMUP_TICK_MARKS} tick marks, {EXCURSION_DIALS} excursion dial wells"
    );
    println!(
        "  Quarantine/release:      {:?} lanes with {STATUS_SLOTS_PER_LANE} closed-material slots per lane and {LANE_INTERLOCK_SHUTTERS} status shutters",
        STATUS_LANE_NAMES
    );
    println!(
        "  Identity evidence:       {BARCODE_LANDS} barcode lands, {COA_CARD_LANDS} CoA lands, {RFID_CHIP_WELLS} RFID wells, {LOT_TOKEN_WELLS} lot token wells"
    );
    println!(
        "  Connector wetness:       {WETNESS_PAD_COUNT} witness pads, {DRY_CONTROL_PADS} dry controls, {CONNECTOR_SADDLES} connector saddles"
    );
    println!(
        "  Clean transfer gate:     {PASS_WINDOW_X:.0}mm x {PASS_WINDOW_Z:.0}mm pass window, {TRANSFER_LATCHES} latch lands, {PRESSURE_EQUALIZATION_PORTS} pressure equalization ports"
    );
    println!(
        "  Robot/service gauges:    front robot {ROBOT_FRONT_APPROACH_Y:.0}mm, rear service {REAR_SERVICE_CLEARANCE_Y:.0}mm, left tote {LEFT_TOTE_SERVICE_X:.0}mm, right release {RIGHT_RELEASE_SERVICE_X:.0}mm, top bridge {TOP_BRIDGE_SERVICE_Z:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_drain_deck()
        + chilled_tote_docks().translate(TOTE_POS.0, TOTE_POS.1, BASE_Z)
        + temperature_logger_pockets().translate(LOGGER_POS.0, LOGGER_POS.1, BASE_Z)
        + warmup_clock_token_rails().translate(WARMUP_POS.0, WARMUP_POS.1, BASE_Z)
        + quarantine_release_lane_gate().translate(LANE_POS.0, LANE_POS.1, BASE_Z)
        + barcode_coa_custody_plate().translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z)
        + connector_wetness_witness_pads().translate(WETNESS_POS.0, WETNESS_POS.1, BASE_Z)
        + clean_enclosure_transfer_port_gauge().translate(TRANSFER_POS.0, TRANSFER_POS.1, BASE_Z)
        + release_audit_token_caddy().translate(AUDIT_POS.0, AUDIT_POS.1, BASE_Z)
        + evidence_camera_bridge().translate(BRIDGE_POS.0, BRIDGE_POS.1, BASE_Z)
        + robot_service_gauges()
}

fn containment_drain_deck() -> Part {
    let deck = centered_cube(
        format!("{PART_PREFIX}_containment_drain_deck_floor"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        format!("{PART_PREFIX}_warmup_condensate_sump_cut"),
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain_channel = centered_cube(
        format!("{PART_PREFIX}_front_condensate_drain_channel"),
        SUMP_X - 120.0,
        DRAIN_CHANNEL_W,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 80.0, BASE_Z - SUMP_DEPTH / 2.0);
    let drain = centered_cylinder(
        format!("{PART_PREFIX}_bulkhead_drain_port"),
        DRAIN_D / 2.0,
        CURB_W + 36.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 14.0,
        BASE_Z - 6.0,
    );

    deck - sump - drain_channel - drain - module_socket_recesses()
        + containment_curbs()
        + mount_bosses()
        + floor_zone_lands()
        + condensate_flow_ribs()
}

fn containment_curbs() -> Part {
    let z = BASE_Z + CURB_Z / 2.0;
    let front = centered_cube(
        format!("{PART_PREFIX}_front_high_condensate_curb"),
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, z);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_clean_enclosure_curb"),
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, z);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_chilled_tote_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_release_curb"),
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{PART_PREFIX}_module_socket_recesses"));
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{PART_PREFIX}_{}_socket_recess", footprint.name),
                footprint.x + 8.0,
                footprint.y + 8.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.25,
            );
    }
    sockets
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PART_PREFIX}_mount_bosses"));
    for index in 0..MOUNT_BOSSES {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let y_index = index / 2;
        let x = side * (STATION_X / 2.0 - 82.0);
        let y = centered_index(y_index, MOUNT_BOSSES / 2, 238.0);
        let boss = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_boss_{index}"),
            MOUNT_BOSS_D / 2.0,
            10.0,
            40,
        )
        .translate(x, y, BASE_Z + 5.0);
        let hole = centered_cylinder(
            format!("{PART_PREFIX}_deck_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            14.0,
            28,
        )
        .translate(x, y, BASE_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn floor_zone_lands() -> Part {
    let chilled = centered_cube(
        format!("{PART_PREFIX}_incoming_chilled_zone_floor_land"),
        TOTE_PANEL_X + 48.0,
        TOTE_PANEL_Y + 36.0,
        3.0,
    )
    .translate(TOTE_POS.0, TOTE_POS.1, BASE_Z + 1.5);
    let evidence = centered_cube(
        format!("{PART_PREFIX}_logger_warmup_evidence_zone_floor_land"),
        1030.0,
        322.0,
        3.0,
    )
    .translate(-170.0, -275.0, BASE_Z + 1.5);
    let release = centered_cube(
        format!("{PART_PREFIX}_quarantine_release_decision_zone_floor_land"),
        754.0,
        456.0,
        3.0,
    )
    .translate(330.0, 214.0, BASE_Z + 1.5);

    chilled + evidence + release
}

fn condensate_flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_condensate_flow_ribs"));
    for index in 0..7 {
        ribs = ribs
            + centered_cube(
                format!("{PART_PREFIX}_condensate_flow_rib_{index}"),
                14.0,
                SUMP_Y - 170.0,
                5.0,
            )
            .translate(centered_index(index, 7, 160.0), -48.0, BASE_Z + 2.5);
    }
    ribs
}

fn chilled_tote_docks() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_chilled_tote_dock_plate"),
        TOTE_PANEL_X,
        TOTE_PANEL_Y,
        TOTE_PANEL_Z,
    )
    .translate(0.0, 0.0, TOTE_PANEL_Z / 2.0);

    let mut cuts = Part::empty(format!("{PART_PREFIX}_chilled_tote_dock_cuts"));
    for dock in 0..CHILLED_TOTE_DOCKS {
        let x = tote_dock_x(dock);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_tote_{dock}_sealed_floor_recess"),
                TOTE_DOCK_X,
                TOTE_DOCK_Y,
                TOTE_DOCK_DEPTH + 1.0,
            )
            .translate(x, 12.0, TOTE_PANEL_Z - TOTE_DOCK_DEPTH / 2.0 + 0.5)
            + centered_cube(
                format!("{PART_PREFIX}_tote_{dock}_front_latch_sensor_trough"),
                TOTE_DOCK_X - 32.0,
                18.0,
                10.0,
            )
            .translate(x, -TOTE_DOCK_Y / 2.0 - 24.0, TOTE_PANEL_Z - 5.0);
    }

    body - cuts
        + tote_side_rails()
        + tote_latch_lands()
        + thermal_contact_ribs()
        + cold_reference_puck_wells()
}

fn tote_side_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_tote_side_rails"));
    for dock in 0..CHILLED_TOTE_DOCKS {
        let x = tote_dock_x(dock);
        let left = centered_cube(
            format!("{PART_PREFIX}_tote_{dock}_left_insulated_guide_rail"),
            16.0,
            TOTE_DOCK_Y + 56.0,
            TOTE_RAIL_Z,
        )
        .translate(
            x - TOTE_DOCK_X / 2.0 - 22.0,
            12.0,
            TOTE_PANEL_Z + TOTE_RAIL_Z / 2.0,
        );
        let right = centered_cube(
            format!("{PART_PREFIX}_tote_{dock}_right_insulated_guide_rail"),
            16.0,
            TOTE_DOCK_Y + 56.0,
            TOTE_RAIL_Z,
        )
        .translate(
            x + TOTE_DOCK_X / 2.0 + 22.0,
            12.0,
            TOTE_PANEL_Z + TOTE_RAIL_Z / 2.0,
        );
        let rear = centered_cube(
            format!("{PART_PREFIX}_tote_{dock}_rear_cold_stop"),
            TOTE_DOCK_X + 62.0,
            18.0,
            TOTE_RAIL_Z,
        )
        .translate(
            x,
            TOTE_DOCK_Y / 2.0 + 42.0,
            TOTE_PANEL_Z + TOTE_RAIL_Z / 2.0,
        );
        rails = rails + left + right + rear;
    }
    rails
}

fn tote_latch_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_tote_latch_and_seal_lands"));
    for index in 0..TOTE_LATCH_LANDS {
        let dock = index / (TOTE_LATCH_LANDS / CHILLED_TOTE_DOCKS);
        let local = index % (TOTE_LATCH_LANDS / CHILLED_TOTE_DOCKS);
        let x = tote_dock_x(dock) + centered_index(local, 4, 54.0);
        let y = -TOTE_PANEL_Y / 2.0 + 32.0;
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_tote_latch_tamper_land_{index}"),
                40.0,
                22.0,
                10.0,
            )
            .translate(x, y, TOTE_PANEL_Z + 5.0);
    }
    lands
}

fn thermal_contact_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_tote_thermal_contact_ribs"));
    for dock in 0..CHILLED_TOTE_DOCKS {
        let x0 = tote_dock_x(dock);
        for rib in 0..THERMAL_CONTACT_RIBS_PER_DOCK {
            ribs = ribs
                + centered_cube(
                    format!("{PART_PREFIX}_tote_{dock}_thermal_contact_rib_{rib}"),
                    14.0,
                    TOTE_DOCK_Y - 46.0,
                    7.0,
                )
                .translate(
                    x0 + centered_index(rib, THERMAL_CONTACT_RIBS_PER_DOCK, 34.0),
                    12.0,
                    TOTE_PANEL_Z + 3.5,
                );
        }
    }
    ribs
}

fn cold_reference_puck_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_cold_reference_puck_wells"));
    for index in 0..COLD_REFERENCE_PUCKS {
        let x = centered_index(index, COLD_REFERENCE_PUCKS, 72.0);
        let outer = centered_cylinder(
            format!("{PART_PREFIX}_cold_reference_puck_ring_{index}"),
            18.0,
            8.0,
            36,
        )
        .translate(x, -TOTE_PANEL_Y / 2.0 + 64.0, TOTE_PANEL_Z + 4.0);
        let inner = centered_cylinder(
            format!("{PART_PREFIX}_cold_reference_puck_recess_{index}"),
            12.0,
            10.0,
            36,
        )
        .translate(x, -TOTE_PANEL_Y / 2.0 + 64.0, TOTE_PANEL_Z + 4.0);
        wells = wells + (outer - inner);
    }
    wells
}

fn temperature_logger_pockets() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_temperature_logger_pocket_panel"),
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);

    let mut cuts = Part::empty(format!("{PART_PREFIX}_logger_pocket_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_logger_{index}_custody_pocket_cut"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH + 1.0,
            )
            .translate(
                logger_x(index),
                -10.0,
                LOGGER_PANEL_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.5,
            );
    }

    body - cuts + logger_cable_comb() + logger_seal_wells() + logger_pogo_pin_lands()
}

fn logger_cable_comb() -> Part {
    let comb = centered_cube(
        format!("{PART_PREFIX}_logger_cable_comb"),
        LOGGER_PANEL_X - 46.0,
        22.0,
        18.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 + 16.0, LOGGER_PANEL_Z - 8.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_logger_cable_comb_cuts"));
    for index in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_logger_{index}_cable_channel"),
                3.4,
                54.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                logger_x(index),
                LOGGER_PANEL_Y / 2.0 + 16.0,
                LOGGER_PANEL_Z - 8.0,
            );
    }
    comb - cuts
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_logger_tamper_seal_wells"));
    for index in 0..LOGGER_SEAL_WELLS {
        let x = centered_index(index, LOGGER_SEAL_WELLS, 38.0);
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_logger_seal_ring_{index}"),
            8.0,
            7.0,
            28,
        )
        .translate(x, -LOGGER_PANEL_Y / 2.0 + 22.0, LOGGER_PANEL_Z + 3.5);
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_logger_seal_well_cut_{index}"),
            4.8,
            9.0,
            28,
        )
        .translate(x, -LOGGER_PANEL_Y / 2.0 + 22.0, LOGGER_PANEL_Z + 3.5);
        wells = wells + (ring - cut);
    }
    wells
}

fn logger_pogo_pin_lands() -> Part {
    let mut pins = Part::empty(format!("{PART_PREFIX}_logger_pogo_pin_lands"));
    for pocket in 0..LOGGER_POCKETS {
        for pin in 0..LOGGER_POGO_PINS_PER_POCKET {
            pins = pins
                + centered_cylinder(
                    format!("{PART_PREFIX}_logger_{pocket}_pogo_pin_land_{pin}"),
                    2.4,
                    3.0,
                    18,
                )
                .translate(
                    logger_x(pocket) + centered_index(pin, LOGGER_POGO_PINS_PER_POCKET, 10.0),
                    48.0,
                    LOGGER_PANEL_Z + 1.5,
                );
        }
    }
    pins
}

fn warmup_clock_token_rails() -> Part {
    let rail = centered_cube(
        format!("{PART_PREFIX}_warmup_clock_token_rail_body"),
        WARMUP_RAIL_X,
        WARMUP_RAIL_Y,
        WARMUP_RAIL_Z,
    )
    .translate(0.0, 0.0, WARMUP_RAIL_Z / 2.0);

    let mut cuts = Part::empty(format!("{PART_PREFIX}_warmup_clock_token_slot_cuts"));
    for lane in 0..WARMUP_LANES {
        for slot in 0..WARMUP_TOKEN_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{PART_PREFIX}_warmup_lane_{lane}_token_slot_{slot}"),
                    WARMUP_SLOT_X,
                    WARMUP_SLOT_Y,
                    WARMUP_SLOT_DEPTH + 1.0,
                )
                .translate(
                    warmup_slot_x(slot),
                    warmup_lane_y(lane),
                    WARMUP_RAIL_Z - WARMUP_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    rail - cuts + warmup_tick_marks() + excursion_dial_wells() + warmup_lane_guard_rails()
}

fn warmup_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_warmup_clock_tick_marks"));
    for index in 0..WARMUP_TICK_MARKS {
        ticks = ticks
            + centered_cube(
                format!("{PART_PREFIX}_warmup_clock_tick_{index}"),
                4.0,
                WARMUP_RAIL_Y - 10.0,
                8.0,
            )
            .translate(
                centered_index(index, WARMUP_TICK_MARKS, 42.0),
                0.0,
                WARMUP_RAIL_Z + 4.0,
            );
    }
    ticks
}

fn excursion_dial_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_excursion_dial_wells"));
    for index in 0..EXCURSION_DIALS {
        let x = WARMUP_RAIL_X / 2.0 - 92.0 + centered_index(index, EXCURSION_DIALS, 24.0);
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_excursion_dial_ring_{index}"),
            10.0,
            7.0,
            28,
        )
        .translate(x, WARMUP_RAIL_Y / 2.0 - 22.0, WARMUP_RAIL_Z + 3.5);
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_excursion_dial_recess_{index}"),
            6.0,
            9.0,
            28,
        )
        .translate(x, WARMUP_RAIL_Y / 2.0 - 22.0, WARMUP_RAIL_Z + 3.5);
        wells = wells + (ring - cut);
    }
    wells
}

fn warmup_lane_guard_rails() -> Part {
    let mut guards = Part::empty(format!("{PART_PREFIX}_warmup_lane_guard_rails"));
    for lane in 0..WARMUP_LANES {
        guards = guards
            + centered_cube(
                format!("{PART_PREFIX}_warmup_lane_{lane}_raised_guard"),
                WARMUP_RAIL_X - 38.0,
                5.0,
                10.0,
            )
            .translate(0.0, warmup_lane_y(lane) + 14.0, WARMUP_RAIL_Z + 5.0);
    }
    guards
}

fn quarantine_release_lane_gate() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_quarantine_release_lane_panel"),
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_quarantine_release_lane_slot_cuts"));
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "{PART_PREFIX}_{}_closed_material_slot_{slot}",
                        STATUS_LANE_NAMES[lane]
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_SLOT_DEPTH + 1.0,
                )
                .translate(
                    status_lane_x(lane),
                    centered_index(slot, STATUS_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y),
                    LANE_PANEL_Z - STATUS_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    panel - cuts + lane_dividers() + lane_interlock_shutters() + status_token_posts()
}

fn lane_dividers() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_quarantine_warmup_hold_divider"),
        LANE_DIVIDER_W,
        LANE_PANEL_Y - 30.0,
        34.0,
    )
    .translate(-STATUS_LANE_PITCH_X / 2.0, 0.0, LANE_PANEL_Z + 17.0);
    let right = centered_cube(
        format!("{PART_PREFIX}_warmup_hold_release_divider"),
        LANE_DIVIDER_W,
        LANE_PANEL_Y - 30.0,
        34.0,
    )
    .translate(STATUS_LANE_PITCH_X / 2.0, 0.0, LANE_PANEL_Z + 17.0);
    let front_stop = centered_cube(
        format!("{PART_PREFIX}_lane_front_locked_stop_bar"),
        LANE_PANEL_X - 36.0,
        12.0,
        28.0,
    )
    .translate(0.0, -LANE_PANEL_Y / 2.0 + 28.0, LANE_PANEL_Z + 14.0);
    let rear_stop = centered_cube(
        format!("{PART_PREFIX}_lane_rear_release_datum_bar"),
        LANE_PANEL_X - 36.0,
        12.0,
        28.0,
    )
    .translate(0.0, LANE_PANEL_Y / 2.0 - 28.0, LANE_PANEL_Z + 14.0);
    left + right + front_stop + rear_stop
}

fn lane_interlock_shutters() -> Part {
    let mut shutters = Part::empty(format!("{PART_PREFIX}_lane_interlock_shutters"));
    for lane in 0..LANE_INTERLOCK_SHUTTERS {
        shutters = shutters
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_interlock_shutter_land",
                    STATUS_LANE_NAMES[lane]
                ),
                116.0,
                22.0,
                18.0,
            )
            .translate(
                status_lane_x(lane),
                LANE_PANEL_Y / 2.0 + 18.0,
                LANE_PANEL_Z + 9.0,
            );
    }
    shutters
}

fn status_token_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_status_lane_token_posts"));
    for lane in 0..STATUS_LANES {
        for token in 0..3 {
            posts = posts
                + centered_cylinder(
                    format!("{PART_PREFIX}_lane_{lane}_status_token_post_{token}"),
                    5.0,
                    10.0,
                    24,
                )
                .translate(
                    status_lane_x(lane) + centered_index(token, 3, 28.0),
                    -LANE_PANEL_Y / 2.0 + 54.0,
                    LANE_PANEL_Z + 5.0,
                );
        }
    }
    posts
}

fn barcode_coa_custody_plate() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_barcode_coa_custody_plate_body"),
        CUSTODY_PLATE_X,
        CUSTODY_PLATE_Y,
        CUSTODY_PLATE_Z,
    )
    .translate(0.0, 0.0, CUSTODY_PLATE_Z / 2.0);
    let plate = plate - coa_card_pocket_cuts() - rfid_chip_well_cuts();
    plate + barcode_lands() + lot_token_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_barcode_lands"));
    for index in 0..BARCODE_LANDS {
        let row = index / 4;
        let col = index % 4;
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_barcode_land_{index}"),
                62.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 76.0),
                -CUSTODY_PLATE_Y / 2.0 + 34.0 + row as f64 * 34.0,
                CUSTODY_PLATE_Z + 2.0,
            );
    }
    lands
}

fn coa_card_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_coa_card_pocket_cuts"));
    for index in 0..COA_CARD_LANDS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_coa_card_pocket_cut_{index}"),
                68.0,
                42.0,
                6.0,
            )
            .translate(
                centered_index(index, COA_CARD_LANDS, 78.0),
                32.0,
                CUSTODY_PLATE_Z - 3.0,
            );
    }
    cuts
}

fn rfid_chip_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_rfid_chip_well_cuts"));
    for index in 0..RFID_CHIP_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_rfid_chip_well_cut_{index}"),
                7.0,
                7.0,
                24,
            )
            .translate(
                centered_index(index, RFID_CHIP_WELLS, 62.0),
                CUSTODY_PLATE_Y / 2.0 - 34.0,
                CUSTODY_PLATE_Z - 3.0,
            );
    }
    cuts
}

fn lot_token_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_lot_identity_token_wells"));
    for index in 0..LOT_TOKEN_WELLS {
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_lot_token_ring_{index}"),
            9.0,
            6.0,
            28,
        )
        .translate(
            centered_index(index, LOT_TOKEN_WELLS, 42.0),
            CUSTODY_PLATE_Y / 2.0 + 18.0,
            CUSTODY_PLATE_Z + 3.0,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_lot_token_center_clearance_{index}"),
            4.8,
            8.0,
            28,
        )
        .translate(
            centered_index(index, LOT_TOKEN_WELLS, 42.0),
            CUSTODY_PLATE_Y / 2.0 + 18.0,
            CUSTODY_PLATE_Z + 3.0,
        );
        wells = wells + (ring - cut);
    }
    wells
}

fn connector_wetness_witness_pads() -> Part {
    let block = centered_cube(
        format!("{PART_PREFIX}_connector_wetness_witness_block"),
        WETNESS_BLOCK_X,
        WETNESS_BLOCK_Y,
        WETNESS_BLOCK_Z,
    )
    .translate(0.0, 0.0, WETNESS_BLOCK_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_wetness_pad_recess_cuts"));
    for index in 0..WETNESS_PAD_COUNT {
        let (x, y) = wetness_pad_xy(index);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_connector_wetness_pad_recess_{index}"),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                WETNESS_PAD_DEPTH + 1.0,
            )
            .translate(x, y, WETNESS_BLOCK_Z - WETNESS_PAD_DEPTH / 2.0 + 0.5);
    }

    block - cuts + dry_control_pads() + connector_saddles() + wetness_drain_gutters()
}

fn dry_control_pads() -> Part {
    let mut pads = Part::empty(format!("{PART_PREFIX}_dry_control_witness_pads"));
    for index in 0..DRY_CONTROL_PADS {
        pads = pads
            + centered_cube(
                format!("{PART_PREFIX}_dry_control_pad_{index}"),
                48.0,
                30.0,
                5.0,
            )
            .translate(
                WETNESS_BLOCK_X / 2.0 - 48.0,
                centered_index(index, DRY_CONTROL_PADS, 46.0),
                WETNESS_BLOCK_Z + 2.5,
            );
    }
    pads
}

fn connector_saddles() -> Part {
    let mut saddles = Part::empty(format!("{PART_PREFIX}_connector_saddles"));
    for index in 0..CONNECTOR_SADDLES {
        let saddle = centered_cube(
            format!("{PART_PREFIX}_connector_saddle_block_{index}"),
            34.0,
            18.0,
            14.0,
        )
        .translate(
            centered_index(index, CONNECTOR_SADDLES, 48.0),
            WETNESS_BLOCK_Y / 2.0 + 18.0,
            WETNESS_BLOCK_Z + 7.0,
        );
        let tube_cut = centered_cylinder(
            format!("{PART_PREFIX}_connector_saddle_tube_cut_{index}"),
            4.2,
            40.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, CONNECTOR_SADDLES, 48.0),
            WETNESS_BLOCK_Y / 2.0 + 18.0,
            WETNESS_BLOCK_Z + 8.0,
        );
        saddles = saddles + (saddle - tube_cut);
    }
    saddles
}

fn wetness_drain_gutters() -> Part {
    let front = centered_cube(
        format!("{PART_PREFIX}_wetness_front_drain_gutter"),
        WETNESS_BLOCK_X - 42.0,
        10.0,
        8.0,
    )
    .translate(0.0, -WETNESS_BLOCK_Y / 2.0 + 16.0, WETNESS_BLOCK_Z + 4.0);
    let rear = centered_cube(
        format!("{PART_PREFIX}_wetness_rear_drain_gutter"),
        WETNESS_BLOCK_X - 42.0,
        10.0,
        8.0,
    )
    .translate(0.0, WETNESS_BLOCK_Y / 2.0 - 16.0, WETNESS_BLOCK_Z + 4.0);
    front + rear
}

fn clean_enclosure_transfer_port_gauge() -> Part {
    let base = centered_cube(
        format!("{PART_PREFIX}_clean_transfer_gauge_base"),
        TRANSFER_PANEL_X,
        TRANSFER_PANEL_Y,
        TRANSFER_PANEL_Z,
    )
    .translate(0.0, 0.0, TRANSFER_PANEL_Z / 2.0);
    let upright = centered_cube(
        format!("{PART_PREFIX}_clean_transfer_port_upright"),
        TRANSFER_GATE_X,
        TRANSFER_GATE_Y,
        TRANSFER_GATE_Z,
    )
    .translate(0.0, 0.0, TRANSFER_PANEL_Z + TRANSFER_GATE_Z / 2.0);
    let window = centered_cube(
        format!("{PART_PREFIX}_clean_transfer_pass_window_cut"),
        PASS_WINDOW_X,
        TRANSFER_GATE_Y + 4.0,
        PASS_WINDOW_Z,
    )
    .translate(0.0, 0.0, TRANSFER_PANEL_Z + TRANSFER_GATE_Z / 2.0);

    base + (upright - window)
        + transfer_gasket_frame()
        + transfer_latch_lands()
        + pressure_equalization_ports()
}

fn transfer_gasket_frame() -> Part {
    let z = TRANSFER_PANEL_Z + TRANSFER_GATE_Z / 2.0;
    let top = centered_cube(
        format!("{PART_PREFIX}_transfer_gasket_top_land"),
        PASS_WINDOW_X + 44.0,
        8.0,
        12.0,
    )
    .translate(
        0.0,
        -TRANSFER_GATE_Y / 2.0 - 6.0,
        z + PASS_WINDOW_Z / 2.0 + 16.0,
    );
    let bottom = centered_cube(
        format!("{PART_PREFIX}_transfer_gasket_bottom_land"),
        PASS_WINDOW_X + 44.0,
        8.0,
        12.0,
    )
    .translate(
        0.0,
        -TRANSFER_GATE_Y / 2.0 - 6.0,
        z - PASS_WINDOW_Z / 2.0 - 16.0,
    );
    let left = centered_cube(
        format!("{PART_PREFIX}_transfer_gasket_left_land"),
        10.0,
        8.0,
        PASS_WINDOW_Z + 44.0,
    )
    .translate(-PASS_WINDOW_X / 2.0 - 20.0, -TRANSFER_GATE_Y / 2.0 - 6.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_transfer_gasket_right_land"),
        10.0,
        8.0,
        PASS_WINDOW_Z + 44.0,
    )
    .translate(PASS_WINDOW_X / 2.0 + 20.0, -TRANSFER_GATE_Y / 2.0 - 6.0, z);
    top + bottom + left + right
}

fn transfer_latch_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_transfer_latch_lands"));
    for index in 0..TRANSFER_LATCHES {
        let x = centered_index(index % 3, 3, 70.0);
        let z = TRANSFER_PANEL_Z + TRANSFER_GATE_Z / 2.0 + if index < 3 { 68.0 } else { -68.0 };
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_transfer_latch_land_{index}"),
                38.0,
                10.0,
                16.0,
            )
            .translate(x, TRANSFER_GATE_Y / 2.0 + 10.0, z);
    }
    lands
}

fn pressure_equalization_ports() -> Part {
    let mut ports = Part::empty(format!(
        "{PART_PREFIX}_transfer_pressure_equalization_ports"
    ));
    for index in 0..PRESSURE_EQUALIZATION_PORTS {
        let ring = centered_cylinder(
            format!("{PART_PREFIX}_pressure_equalization_port_ring_{index}"),
            8.0,
            8.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, PRESSURE_EQUALIZATION_PORTS, 34.0),
            TRANSFER_GATE_Y / 2.0 + 8.0,
            TRANSFER_PANEL_Z + 24.0,
        );
        let cut = centered_cylinder(
            format!("{PART_PREFIX}_pressure_equalization_port_clearance_{index}"),
            4.0,
            10.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, PRESSURE_EQUALIZATION_PORTS, 34.0),
            TRANSFER_GATE_Y / 2.0 + 8.0,
            TRANSFER_PANEL_Z + 24.0,
        );
        ports = ports + (ring - cut);
    }
    ports
}

fn release_audit_token_caddy() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_release_audit_token_caddy_body"),
        AUDIT_CADDY_X,
        AUDIT_CADDY_Y,
        AUDIT_CADDY_Z,
    )
    .translate(0.0, 0.0, AUDIT_CADDY_Z / 2.0);
    let mut cuts = Part::empty(format!("{PART_PREFIX}_release_audit_token_slot_cuts"));
    for token in 0..RELEASE_AUDIT_TOKENS {
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_release_audit_token_slot_{token}"),
                30.0,
                42.0,
                8.0,
            )
            .translate(
                centered_index(token, RELEASE_AUDIT_TOKENS, 38.0),
                -8.0,
                AUDIT_CADDY_Z - 4.0,
            );
    }

    body - cuts + release_lock_pin_posts() + audit_status_tabs()
}

fn release_lock_pin_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_release_lock_pin_posts"));
    for index in 0..RELEASE_LOCK_PINS {
        let post = centered_cylinder(
            format!("{PART_PREFIX}_release_lock_pin_post_{index}"),
            5.5,
            12.0,
            24,
        )
        .translate(
            centered_index(index, RELEASE_LOCK_PINS, 48.0),
            AUDIT_CADDY_Y / 2.0 + 14.0,
            AUDIT_CADDY_Z + 6.0,
        );
        posts = posts + post;
    }
    posts
}

fn audit_status_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PART_PREFIX}_audit_status_tabs"));
    for index in 0..STATUS_LANES {
        tabs = tabs
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_audit_status_tab",
                    STATUS_LANE_NAMES[index]
                ),
                88.0,
                16.0,
                12.0,
            )
            .translate(
                centered_index(index, STATUS_LANES, 112.0),
                -AUDIT_CADDY_Y / 2.0 - 10.0,
                AUDIT_CADDY_Z + 6.0,
            );
    }
    tabs
}

fn robot_service_gauges() -> Part {
    let z = BASE_Z + KEEP_OUT_RAIL_Z / 2.0;
    let front = centered_cube(
        format!("{PART_PREFIX}_front_robot_unload_sweep_gauge"),
        ROBOT_SWEEP_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + 62.0, z);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_clean_enclosure_service_gauge"),
        ROBOT_SWEEP_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 62.0, z);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_tote_service_gauge"),
        8.0,
        ROBOT_SWEEP_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-STATION_X / 2.0 + 64.0, 0.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_release_service_gauge"),
        8.0,
        ROBOT_SWEEP_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(STATION_X / 2.0 - 64.0, 0.0, z);

    front + rear + left + right + gripper_clearance_posts() + top_service_height_gauge()
}

fn gripper_clearance_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_robot_gripper_clearance_posts"));
    for index in 0..GRIPPER_CLEARANCE_POSTS {
        posts = posts
            + centered_cube(
                format!("{PART_PREFIX}_robot_gripper_clearance_post_{index}"),
                16.0,
                16.0,
                ROBOT_SWEEP_Z,
            )
            .translate(
                centered_index(index, GRIPPER_CLEARANCE_POSTS, 190.0),
                -STATION_Y / 2.0 + 118.0,
                BASE_Z + ROBOT_SWEEP_Z / 2.0,
            );
    }
    posts
}

fn top_service_height_gauge() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_top_bridge_left_height_gauge"),
        18.0,
        18.0,
        TOP_BRIDGE_SERVICE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + TOP_BRIDGE_SERVICE_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PART_PREFIX}_top_bridge_right_height_gauge"),
        18.0,
        18.0,
        TOP_BRIDGE_SERVICE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + TOP_BRIDGE_SERVICE_Z / 2.0,
    );
    let cap = centered_cube(
        format!("{PART_PREFIX}_top_bridge_service_cap_gauge"),
        BRIDGE_SPAN_X + 40.0,
        12.0,
        10.0,
    )
    .translate(0.0, BRIDGE_POS.1, BASE_Z + TOP_BRIDGE_SERVICE_Z + 5.0);
    left + right + cap
}

fn evidence_camera_bridge() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let right = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_POST_Z / 2.0);
    let beam = centered_cube(
        format!("{PART_PREFIX}_evidence_bridge_camera_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);

    left + right + beam + camera_pods() + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_evidence_camera_pods"));
    for index in 0..CAMERA_PODS {
        let pod = centered_cube(
            format!("{PART_PREFIX}_evidence_camera_pod_{index}"),
            54.0,
            38.0,
            28.0,
        )
        .translate(
            centered_index(index, CAMERA_PODS, 220.0),
            -BRIDGE_BEAM_Y / 2.0 - 16.0,
            BRIDGE_POST_Z - 18.0,
        );
        let lens = centered_cylinder(
            format!("{PART_PREFIX}_evidence_camera_lens_{index}"),
            9.0,
            12.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(index, CAMERA_PODS, 220.0),
            -BRIDGE_BEAM_Y / 2.0 - 34.0,
            BRIDGE_POST_Z - 18.0,
        );
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty(format!("{PART_PREFIX}_evidence_light_bars"));
    for index in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{PART_PREFIX}_evidence_light_bar_{index}"),
                170.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(index, LIGHT_BARS, 260.0),
                BRIDGE_BEAM_Y / 2.0 + 10.0,
                CAMERA_CLEARANCE_Z,
            );
    }
    bars
}

fn module_footprints() -> [Footprint; 8] {
    [
        footprint("chilled_tote_docks", TOTE_POS, TOTE_PANEL_X, TOTE_PANEL_Y),
        footprint(
            "temperature_logger_pockets",
            LOGGER_POS,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        ),
        footprint(
            "warmup_clock_token_rails",
            WARMUP_POS,
            WARMUP_RAIL_X,
            WARMUP_RAIL_Y,
        ),
        footprint(
            "quarantine_release_lane_gate",
            LANE_POS,
            LANE_PANEL_X,
            LANE_PANEL_Y,
        ),
        footprint(
            "barcode_coa_custody_plate",
            CUSTODY_POS,
            CUSTODY_PLATE_X,
            CUSTODY_PLATE_Y,
        ),
        footprint(
            "connector_wetness_witness_pads",
            WETNESS_POS,
            WETNESS_BLOCK_X,
            WETNESS_BLOCK_Y,
        ),
        footprint(
            "clean_enclosure_transfer_port_gauge",
            TRANSFER_POS,
            TRANSFER_PANEL_X,
            TRANSFER_PANEL_Y,
        ),
        footprint(
            "release_audit_token_caddy",
            AUDIT_POS,
            AUDIT_CADDY_X,
            AUDIT_CADDY_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_inside_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - CURB_W - 16.0;
    let usable_y = STATION_Y / 2.0 - CURB_W - 16.0;
    footprint.center.0 - footprint.x / 2.0 >= -usable_x
        && footprint.center.0 + footprint.x / 2.0 <= usable_x
        && footprint.center.1 - footprint.y / 2.0 >= -usable_y
        && footprint.center.1 + footprint.y / 2.0 <= usable_y
}

fn overlaps(first: Footprint, second: Footprint) -> bool {
    let dx = (first.center.0 - second.center.0).abs();
    let dy = (first.center.1 - second.center.1).abs();
    dx < (first.x + second.x) / 2.0 && dy < (first.y + second.y) / 2.0
}

fn tote_dock_x(index: usize) -> f64 {
    centered_index(index, CHILLED_TOTE_DOCKS, TOTE_DOCK_PITCH_X)
}

fn logger_x(index: usize) -> f64 {
    centered_index(index, LOGGER_POCKETS, LOGGER_SLOT_PITCH_X)
}

fn warmup_slot_x(index: usize) -> f64 {
    centered_index(index, WARMUP_TOKEN_SLOTS_PER_LANE, WARMUP_SLOT_PITCH_X)
}

fn warmup_lane_y(index: usize) -> f64 {
    centered_index(index, WARMUP_LANES, WARMUP_LANE_PITCH_Y)
}

fn status_lane_x(index: usize) -> f64 {
    centered_index(index, STATUS_LANES, STATUS_LANE_PITCH_X)
}

fn wetness_pad_xy(index: usize) -> (f64, f64) {
    let row = index / WETNESS_PAD_COLS;
    let col = index % WETNESS_PAD_COLS;
    (
        centered_index(col, WETNESS_PAD_COLS, 58.0),
        centered_index(row, WETNESS_PAD_ROWS, 58.0),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(CHILLED_TOTE_DOCKS, 2);
    assert_eq!(
        LOGGER_POGO_PINS,
        LOGGER_POCKETS * LOGGER_POGO_PINS_PER_POCKET
    );
    assert_eq!(
        WARMUP_TOKEN_SLOTS,
        WARMUP_LANES * WARMUP_TOKEN_SLOTS_PER_LANE
    );
    assert_eq!(WARMUP_TICK_MARKS, WARMUP_TOKEN_SLOTS + 1);
    assert_eq!(WETNESS_PAD_COUNT, WETNESS_PAD_COLS * WETNESS_PAD_ROWS);
    assert_eq!(STATUS_LANE_NAMES, ["quarantine", "warmup_hold", "release"]);
    assert!(LANE_MIN_GAP >= 50.0);
    assert!(PASS_WINDOW_X < TRANSFER_GATE_X);
    assert!(PASS_WINDOW_Z < TRANSFER_GATE_Z);
    assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(TOP_BRIDGE_SERVICE_Z > BRIDGE_POST_Z + BRIDGE_BEAM_Z + 90.0);
    assert!(ROBOT_SWEEP_X < STATION_X);
    assert!(ROBOT_SWEEP_Y < STATION_Y);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.ends_with(".stl"), "{path}");
    }

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            fits_inside_station(footprint),
            "{} exceeds station footprint",
            footprint.name
        );
    }

    for (index, first) in footprints.iter().enumerate() {
        for second in footprints.iter().skip(index + 1) {
            assert!(
                !overlaps(*first, *second),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        assert_layout();
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS[0].contains("containment_drain_deck"));
        assert!(OUTPUTS[1].contains("chilled_tote_docks"));
        assert!(OUTPUTS[2].contains("temperature_logger_pockets"));
        assert!(OUTPUTS[3].contains("warmup_clock_token_rails"));
        assert!(OUTPUTS[10].contains("evidence_camera_bridge"));
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_design_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        for feature in [
            "chilled_tote_docks",
            "temperature_logger_pockets",
            "warmup_clock_token_rails",
            "quarantine_release_lanes",
            "barcode_coa_custody_plate",
            "connector_wetness_witness_pads",
            "clean_enclosure_transfer_port_gauge",
            "release_audit_token_caddy",
            "evidence_camera_bridge",
            "robot_service_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn primary_modules_fit_without_footprint_overlap() {
        let footprints = module_footprints();
        for footprint in footprints {
            assert!(fits_inside_station(footprint), "{}", footprint.name);
        }
        for (index, first) in footprints.iter().enumerate() {
            for second in footprints.iter().skip(index + 1) {
                assert!(
                    !overlaps(*first, *second),
                    "{} overlaps {}",
                    first.name,
                    second.name
                );
            }
        }
    }

    #[test]
    fn cold_chain_warmup_and_logger_counts_are_explicit() {
        assert_eq!(CHILLED_TOTE_DOCKS, 2);
        assert_eq!(THERMAL_CONTACT_RIBS_PER_DOCK, 5);
        assert_eq!(COLD_REFERENCE_PUCKS, 4);
        assert_eq!(LOGGER_POCKETS, 6);
        assert_eq!(LOGGER_SEAL_WELLS, 8);
        assert_eq!(LOGGER_POGO_PINS, 18);
        assert_eq!(WARMUP_LANES, 3);
        assert_eq!(WARMUP_TOKEN_SLOTS, 12);
        assert_eq!(EXCURSION_DIALS, 6);
    }

    #[test]
    fn identity_wetness_and_release_capacity_are_sane() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 4);
        assert_eq!(STATUS_LANE_NAMES, ["quarantine", "warmup_hold", "release"]);
        assert!(LANE_MIN_GAP >= 50.0);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_CARD_LANDS, 4);
        assert_eq!(RFID_CHIP_WELLS, 4);
        assert_eq!(LOT_TOKEN_WELLS, 6);
        assert_eq!(WETNESS_PAD_COUNT, 10);
        assert_eq!(DRY_CONTROL_PADS, 2);
        assert_eq!(CONNECTOR_SADDLES, 6);
        assert!(STATUS_LANES * STATUS_SLOTS_PER_LANE >= WETNESS_PAD_COUNT);
    }

    #[test]
    fn transfer_gate_and_robot_service_gauges_are_declared() {
        assert!(PASS_WINDOW_X < TRANSFER_GATE_X);
        assert!(PASS_WINDOW_Z < TRANSFER_GATE_Z);
        assert_eq!(TRANSFER_LATCHES, 6);
        assert_eq!(PRESSURE_EQUALIZATION_PORTS, 4);
        assert_eq!(RELEASE_AUDIT_TOKENS, 9);
        assert_eq!(RELEASE_LOCK_PINS, 6);
        assert_eq!(CAMERA_PODS, 5);
        assert_eq!(LIGHT_BARS, 4);
        assert_eq!(GRIPPER_CLEARANCE_POSTS, 6);
        assert!(BRIDGE_SPAN_X > TOTE_PANEL_X + LANE_PANEL_X);
        assert!(ROBOT_FRONT_APPROACH_Y >= 400.0);
        assert!(TOP_BRIDGE_SERVICE_Z > BRIDGE_POST_Z + BRIDGE_BEAM_Z);
    }

    #[test]
    fn limitation_markers_prevent_protocol_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"no_release_rule"));
        assert!(LIMITATIONS.contains(&"no_biological_acceptance_criteria"));
        assert!(LIMITATIONS.contains(&"not_a_sterile_barrier_design"));
        assert!(LIMITATIONS.contains(&"purchased_wetted_components_external"));
    }
}
