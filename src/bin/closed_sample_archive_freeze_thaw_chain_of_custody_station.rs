use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sample/archive freeze-thaw chain-of-custody validation station.
//
// Design intent:
// - Validate archive tube identity, custody state, cold-chain transfer timing,
//   freeze-thaw count, cap/seal evidence, and carryover risk before endpoint
//   samples are trusted.
// - Keep the station no-cell and closed-handling focused: it models mechanical
//   nests, cold deck boundaries, witness pockets, evidence capture, and
//   segregation lanes, not a biological release protocol.
// - Make robot/service keepouts explicit so custody capture, freezer transfer,
//   and archive disposition cannot silently collide in the same envelope.

const OUTPUTS: [&str; 13] = [
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_containment_cold_deck.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_archive_tube_nest_array.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_freezer_transfer_dock.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_freeze_thaw_cycle_token_rail.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_cap_seal_witness_pockets.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_barcode_rfid_custody_lands.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_carryover_flush_witness_wells.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_temperature_logger_pockets.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_released_hold_reject_lanes.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_evidence_camera_bridge.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_robot_service_keepout_gauges.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_custody_mismatch_lockout_gates.stl",
    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "containment_cold_deck",
    "archive_tube_nest_array",
    "freezer_transfer_dock",
    "freeze_thaw_cycle_token_rail",
    "cap_seal_witness_pockets",
    "barcode_rfid_custody_lands",
    "carryover_flush_witness_wells",
    "temperature_logger_pockets",
    "released_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
    "custody_mismatch_lockout_gates",
];

const STATUS_LANES: [&str; 3] = ["released", "hold", "reject"];
const CUSTODY_STATES: [&str; 4] = ["received", "scanned", "verified", "released"];

const STATION_X: f64 = 1460.0;
const STATION_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const COLD_PLATE_X: f64 = 1010.0;
const COLD_PLATE_Y: f64 = 700.0;
const COLD_PLATE_Z: f64 = 10.0;
const CONDENSATE_CHANNEL_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 12.0;
const MOUNT_HOLES: usize = 8;

const NEST_CENTER: (f64, f64) = (-360.0, 120.0);
const NEST_X: f64 = 520.0;
const NEST_Y: f64 = 340.0;
const NEST_Z: f64 = 70.0;
const TUBE_ROWS: usize = 4;
const TUBE_COLS: usize = 8;
const TUBE_COUNT: usize = TUBE_ROWS * TUBE_COLS;
const TUBE_WELL_D: f64 = 17.4;
const TUBE_PITCH_X: f64 = 48.0;
const TUBE_PITCH_Y: f64 = 56.0;
const NEST_DATUM_PINS: usize = 4;

const DOCK_CENTER: (f64, f64) = (300.0, 250.0);
const DOCK_X: f64 = 420.0;
const DOCK_Y: f64 = 210.0;
const DOCK_Z: f64 = 76.0;
const FREEZER_CASSETTE_SLOTS: usize = 3;
const TRANSFER_TIMER_WINDOWS: usize = 4;

const TOKEN_CENTER: (f64, f64) = (-360.0, -210.0);
const TOKEN_X: f64 = 520.0;
const TOKEN_Y: f64 = 104.0;
const TOKEN_Z: f64 = 34.0;
const FREEZE_THAW_TOKEN_COUNT: usize = 10;
const TOKEN_PITCH: f64 = 46.0;

const SEAL_CENTER: (f64, f64) = (265.0, 70.0);
const SEAL_X: f64 = 350.0;
const SEAL_Y: f64 = 170.0;
const SEAL_Z: f64 = 42.0;
const CAP_WITNESS_POCKETS: usize = 8;
const SEAL_STRIP_LANDS: usize = 6;

const CUSTODY_CENTER: (f64, f64) = (295.0, -155.0);
const CUSTODY_X: f64 = 450.0;
const CUSTODY_Y: f64 = 190.0;
const CUSTODY_Z: f64 = 28.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 8;
const SIGNATURE_CARD_SLOTS: usize = 4;

const FLUSH_CENTER: (f64, f64) = (-40.0, -355.0);
const FLUSH_X: f64 = 440.0;
const FLUSH_Y: f64 = 150.0;
const FLUSH_Z: f64 = 52.0;
const FLUSH_WELLS: usize = 8;
const BLANK_WITNESS_WELLS: usize = 4;

const LOGGER_CENTER: (f64, f64) = (555.0, 115.0);
const LOGGER_X: f64 = 220.0;
const LOGGER_Y: f64 = 230.0;
const LOGGER_Z: f64 = 54.0;
const LOGGER_POCKETS: usize = 3;
const PROBE_CHANNELS: usize = 6;

const LANES_CENTER: (f64, f64) = (480.0, -325.0);
const LANES_X: f64 = 420.0;
const LANES_Y: f64 = 190.0;
const LANES_Z: f64 = 44.0;
const SLOTS_PER_STATUS: usize = 4;
const STATUS_SLOT_X: f64 = 88.0;
const STATUS_SLOT_Y: f64 = 34.0;
const STATUS_LANE_PITCH_X: f64 = 124.0;
#[cfg(test)]
const MIN_STATUS_GAP: f64 = 30.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 25.0);
const BRIDGE_SPAN_X: f64 = 1240.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_UNDERSIDE_Z: f64 = 250.0;
const BRIDGE_BEAM_Z: f64 = 36.0;
const CAMERA_PODS: usize = 4;
const EVIDENCE_LED_SEGMENTS: usize = 10;

const KEEP_OUT_Z: f64 = 92.0;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_FREEZER_SERVICE_CLEARANCE: f64 = 320.0;
const SIDE_ARCHIVE_CART_CLEARANCE: f64 = 260.0;
const OVERHEAD_CAMERA_CLEARANCE: f64 = 310.0;
const KEEP_OUT_GAUGES: usize = 4;

const LOCKOUT_CENTER: (f64, f64) = (-25.0, 370.0);
const LOCKOUT_X: f64 = 690.0;
const LOCKOUT_Y: f64 = 110.0;
const LOCKOUT_Z: f64 = 58.0;
const LOCKOUT_GATES: usize = 5;
const ERROR_FLAG_SLOTS: usize = 5;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = containment_cold_deck();
    export(OUTPUTS[0], &deck);

    let nest = archive_tube_nest_array();
    export(OUTPUTS[1], &nest);

    let dock = freezer_transfer_dock();
    export(OUTPUTS[2], &dock);

    let tokens = freeze_thaw_cycle_token_rail();
    export(OUTPUTS[3], &tokens);

    let seals = cap_seal_witness_pockets();
    export(OUTPUTS[4], &seals);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[5], &custody);

    let flush = carryover_flush_witness_wells();
    export(OUTPUTS[6], &flush);

    let logger = temperature_logger_pockets();
    export(OUTPUTS[7], &logger);

    let lanes = released_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let lockouts = custody_mismatch_lockout_gates();
    export(OUTPUTS[11], &lockouts);

    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, deck_mount_z(NEST_Z))
        + dock.translate(DOCK_CENTER.0, DOCK_CENTER.1, deck_mount_z(DOCK_Z))
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_mount_z(TOKEN_Z))
        + seals.translate(SEAL_CENTER.0, SEAL_CENTER.1, deck_mount_z(SEAL_Z))
        + custody.translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, deck_mount_z(CUSTODY_Z))
        + flush.translate(FLUSH_CENTER.0, FLUSH_CENTER.1, deck_mount_z(FLUSH_Z))
        + logger.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, deck_mount_z(LOGGER_Z))
        + lanes.translate(LANES_CENTER.0, LANES_CENTER.1, deck_mount_z(LANES_Z))
        + bridge.translate(
            BRIDGE_CENTER.0,
            BRIDGE_CENTER.1,
            deck_mount_z(BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z),
        )
        + keepouts.translate(0.0, 0.0, deck_mount_z(KEEP_OUT_Z))
        + lockouts.translate(LOCKOUT_CENTER.0, LOCKOUT_CENTER.1, deck_mount_z(LOCKOUT_Z));
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed sample/archive freeze-thaw chain-of-custody station:");
    println!(
        "  Containment/cold deck:       {STATION_X:.0}mm x {STATION_Y:.0}mm tray, {COLD_PLATE_X:.0}mm x {COLD_PLATE_Y:.0}mm cold plate, {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Archive nest:                {TUBE_ROWS} x {TUBE_COLS} tube wells ({TUBE_COUNT} total), {NEST_DATUM_PINS} datum pins"
    );
    println!(
        "  Freezer transfer dock:       {FREEZER_CASSETTE_SLOTS} cassette slots, {TRANSFER_TIMER_WINDOWS} timing evidence windows"
    );
    println!(
        "  Freeze-thaw controls:        {FREEZE_THAW_TOKEN_COUNT} cycle token positions with cap/seal witness pockets"
    );
    println!(
        "  Custody capture:             {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {SIGNATURE_CARD_SLOTS} custody card slots"
    );
    println!(
        "  Carryover evidence:          {FLUSH_WELLS} flush wells and {BLANK_WITNESS_WELLS} blank witness wells"
    );
    println!(
        "  Temperature evidence:        {LOGGER_POCKETS} logger pockets and {PROBE_CHANNELS} probe channels"
    );
    println!(
        "  Disposition lanes:           released/hold/reject with {SLOTS_PER_STATUS} positions each"
    );
    println!(
        "  Evidence/keepout envelope:   {CAMERA_PODS} camera pods, {EVIDENCE_LED_SEGMENTS} LED segments, {KEEP_OUT_GAUGES} keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_mount_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert_eq!(TUBE_COUNT, TUBE_ROWS * TUBE_COLS);
    assert!(FREEZE_THAW_TOKEN_COUNT >= 8);
    assert!(CAP_WITNESS_POCKETS >= TUBE_ROWS + TUBE_COLS / 2);
    assert!(BARCODE_LANDS >= RFID_LANDS);
    assert!(LOGGER_POCKETS >= 3);
    assert!(LOCKOUT_GATES == ERROR_FLAG_SLOTS);

    for (name, center, width, depth) in component_specs() {
        assert!(
            fits_on_station(center, width, depth),
            "{name} exceeds cold deck station envelope"
        );
    }
}

fn component_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        ("archive_tube_nest_array", NEST_CENTER, NEST_X, NEST_Y),
        ("freezer_transfer_dock", DOCK_CENTER, DOCK_X, DOCK_Y),
        (
            "freeze_thaw_cycle_token_rail",
            TOKEN_CENTER,
            TOKEN_X,
            TOKEN_Y,
        ),
        ("cap_seal_witness_pockets", SEAL_CENTER, SEAL_X, SEAL_Y),
        (
            "barcode_rfid_custody_lands",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        (
            "carryover_flush_witness_wells",
            FLUSH_CENTER,
            FLUSH_X,
            FLUSH_Y,
        ),
        (
            "temperature_logger_pockets",
            LOGGER_CENTER,
            LOGGER_X,
            LOGGER_Y,
        ),
        ("released_hold_reject_lanes", LANES_CENTER, LANES_X, LANES_Y),
        (
            "custody_mismatch_lockout_gates",
            LOCKOUT_CENTER,
            LOCKOUT_X,
            LOCKOUT_Y,
        ),
        (
            "evidence_camera_bridge",
            BRIDGE_CENTER,
            BRIDGE_SPAN_X,
            BRIDGE_POST_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn containment_cold_deck() -> Part {
    let deck = centered_cube(
        "archive_freeze_thaw_containment_deck_plate",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let cold_plate_recess = centered_cube(
        "archive_freeze_thaw_cold_plate_recess",
        COLD_PLATE_X,
        COLD_PLATE_Y,
        COLD_PLATE_Z + 2.0,
    )
    .translate(-70.0, 18.0, DECK_Z / 2.0 - COLD_PLATE_Z / 2.0 + 1.0);
    let condensate_channel = centered_cube(
        "archive_freeze_thaw_condensate_channel_cut",
        STATION_X - 190.0,
        26.0,
        CONDENSATE_CHANNEL_DEPTH + 2.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 72.0, DECK_Z / 2.0 - 3.0);
    let sump = centered_cube(
        "archive_freeze_thaw_condensate_sump_cut",
        118.0,
        76.0,
        CONDENSATE_CHANNEL_DEPTH + 4.0,
    )
    .translate(
        STATION_X / 2.0 - 126.0,
        -STATION_Y / 2.0 + 90.0,
        DECK_Z / 2.0 - 3.0,
    );
    let drain = centered_cylinder(
        "archive_freeze_thaw_condensate_drain_port",
        DRAIN_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 126.0,
        -STATION_Y / 2.0 + 30.0,
        DECK_Z / 2.0 - 3.0,
    );

    deck - cold_plate_recess - condensate_channel - sump - drain - mount_holes()
        + perimeter_rims()
        + cold_plate_zone()
        + component_socket_cuts()
        + custody_route_rails()
}

fn perimeter_rims() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    centered_cube(
        "archive_freeze_thaw_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            "archive_freeze_thaw_rear_spill_rim",
            STATION_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            "archive_freeze_thaw_left_spill_rim",
            RIM_W,
            STATION_Y,
            RIM_Z,
        )
        .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            "archive_freeze_thaw_right_spill_rim",
            RIM_W,
            STATION_Y,
            RIM_Z,
        )
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn cold_plate_zone() -> Part {
    let plate = centered_cube(
        "archive_freeze_thaw_removable_cold_plate_insert",
        COLD_PLATE_X - 30.0,
        COLD_PLATE_Y - 30.0,
        5.0,
    )
    .translate(-70.0, 18.0, DECK_Z / 2.0 + 2.5);
    let boundary = rectangular_frame_xy(
        "archive_freeze_thaw_blue_cold_zone_boundary",
        COLD_PLATE_X + 20.0,
        COLD_PLATE_Y + 20.0,
        8.0,
        COLD_PLATE_X - 20.0,
        COLD_PLATE_Y - 20.0,
    )
    .translate(-70.0, 18.0, DECK_Z / 2.0 + 7.0);
    plate + boundary
}

fn component_socket_cuts() -> Part {
    let mut sockets = Part::empty("archive_freeze_thaw_component_socket_cuts");
    for (name, center, width, depth) in component_specs().into_iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!("archive_freeze_thaw_{name}_locator_socket"),
                width + 10.0,
                depth + 10.0,
                5.0,
            )
            .translate(center.0, center.1, DECK_Z / 2.0 - 2.4);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("archive_freeze_thaw_m6_mount_holes");
    for i in 0..MOUNT_HOLES {
        let x = centered_index(i % 4, 4, 390.0);
        let y = if i < 4 {
            -STATION_Y / 2.0 + 48.0
        } else {
            STATION_Y / 2.0 - 48.0
        };
        holes = holes
            + centered_cylinder(
                format!("archive_freeze_thaw_m6_mount_hole_{i}"),
                3.3,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, 0.0);
    }
    holes
}

fn custody_route_rails() -> Part {
    centered_cube(
        "archive_freeze_thaw_freezer_to_scan_custody_route",
        470.0,
        10.0,
        14.0,
    )
    .translate(245.0, 155.0, DECK_Z / 2.0 + 7.0)
        + centered_cube(
            "archive_freeze_thaw_scan_to_hold_lane_route",
            380.0,
            10.0,
            14.0,
        )
        .translate(320.0, -245.0, DECK_Z / 2.0 + 7.0)
        + centered_cube(
            "archive_freeze_thaw_archive_nest_to_token_route",
            10.0,
            300.0,
            14.0,
        )
        .translate(-360.0, -45.0, DECK_Z / 2.0 + 7.0)
}

fn archive_tube_nest_array() -> Part {
    let body = centered_cube("archive_freeze_thaw_tube_nest_body", NEST_X, NEST_Y, NEST_Z);
    let mut wells = Part::empty("archive_freeze_thaw_tube_well_cuts");
    for row in 0..TUBE_ROWS {
        for col in 0..TUBE_COLS {
            wells = wells
                + centered_cylinder(
                    format!("archive_freeze_thaw_archive_tube_well_{row}_{col}"),
                    TUBE_WELL_D / 2.0,
                    NEST_Z + 4.0,
                    32,
                )
                .translate(
                    centered_index(col, TUBE_COLS, TUBE_PITCH_X),
                    centered_index(row, TUBE_ROWS, TUBE_PITCH_Y),
                    10.0,
                );
        }
    }

    body - wells + nest_row_identity_tabs() + nest_datum_features() + cold_finger_ribs()
}

fn nest_row_identity_tabs() -> Part {
    let mut tabs = Part::empty("archive_freeze_thaw_nest_row_identity_tabs");
    for row in 0..TUBE_ROWS {
        tabs = tabs
            + centered_cube(
                format!("archive_freeze_thaw_nest_row_{row}_identity_tab"),
                42.0,
                24.0,
                8.0,
            )
            .translate(
                -NEST_X / 2.0 + 32.0,
                centered_index(row, TUBE_ROWS, TUBE_PITCH_Y),
                NEST_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn nest_datum_features() -> Part {
    let mut datums = Part::empty("archive_freeze_thaw_nest_datum_features");
    for (i, (x, y)) in [
        (-NEST_X / 2.0 + 38.0, -NEST_Y / 2.0 + 38.0),
        (NEST_X / 2.0 - 38.0, -NEST_Y / 2.0 + 38.0),
        (-NEST_X / 2.0 + 38.0, NEST_Y / 2.0 - 38.0),
        (NEST_X / 2.0 - 38.0, NEST_Y / 2.0 - 38.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("archive_freeze_thaw_nest_datum_boss_{i}"),
            13.0,
            10.0,
            32,
        )
        .translate(x, y, NEST_Z / 2.0 + 5.0);
        let bore = centered_cylinder(
            format!("archive_freeze_thaw_nest_datum_bore_{i}"),
            4.2,
            12.0,
            24,
        )
        .translate(x, y, NEST_Z / 2.0 + 5.0);
        datums = datums + (boss - bore);
    }
    datums
}

fn cold_finger_ribs() -> Part {
    let mut ribs = Part::empty("archive_freeze_thaw_cold_finger_ribs");
    for col in 0..TUBE_COLS {
        ribs = ribs
            + centered_cube(
                format!("archive_freeze_thaw_cold_finger_rib_{col}"),
                7.0,
                NEST_Y - 54.0,
                14.0,
            )
            .translate(
                centered_index(col, TUBE_COLS, TUBE_PITCH_X),
                0.0,
                -NEST_Z / 2.0 - 7.0,
            );
    }
    ribs
}

fn freezer_transfer_dock() -> Part {
    let body = centered_cube(
        "archive_freeze_thaw_freezer_transfer_dock_body",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let mut cuts = Part::empty("archive_freeze_thaw_freezer_cassette_slot_cuts");
    for i in 0..FREEZER_CASSETTE_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("archive_freeze_thaw_freezer_cassette_slot_{i}"),
                94.0,
                DOCK_Y - 54.0,
                30.0,
            )
            .translate(centered_index(i, FREEZER_CASSETTE_SLOTS, 124.0), 0.0, 18.0);
    }

    body - cuts + dock_insulation_lips() + transfer_timer_windows() + dry_ice_guard_rail()
}

fn dock_insulation_lips() -> Part {
    centered_cube(
        "archive_freeze_thaw_freezer_dock_front_insulation_lip",
        DOCK_X + 24.0,
        14.0,
        28.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 - 7.0, 0.0)
        + centered_cube(
            "archive_freeze_thaw_freezer_dock_rear_insulation_lip",
            DOCK_X + 24.0,
            14.0,
            28.0,
        )
        .translate(0.0, DOCK_Y / 2.0 + 7.0, 0.0)
}

fn transfer_timer_windows() -> Part {
    let mut windows = Part::empty("archive_freeze_thaw_transfer_timer_windows");
    for i in 0..TRANSFER_TIMER_WINDOWS {
        windows = windows
            + centered_cube(
                format!("archive_freeze_thaw_transfer_timer_window_{i}"),
                58.0,
                8.0,
                28.0,
            )
            .translate(
                centered_index(i, TRANSFER_TIMER_WINDOWS, 78.0),
                -DOCK_Y / 2.0 - 18.0,
                16.0,
            );
    }
    windows
}

fn dry_ice_guard_rail() -> Part {
    rectangular_frame_xy(
        "archive_freeze_thaw_freezer_transfer_cold_guard_rail",
        DOCK_X + 38.0,
        DOCK_Y + 38.0,
        10.0,
        DOCK_X + 2.0,
        DOCK_Y + 2.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 5.0)
}

fn freeze_thaw_cycle_token_rail() -> Part {
    let rail = centered_cube(
        "archive_freeze_thaw_cycle_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut token_cuts = Part::empty("archive_freeze_thaw_cycle_token_socket_cuts");
    for i in 0..FREEZE_THAW_TOKEN_COUNT {
        token_cuts = token_cuts
            + centered_cylinder(
                format!("archive_freeze_thaw_cycle_token_socket_{i}"),
                13.0,
                TOKEN_Z + 4.0,
                30,
            )
            .translate(
                centered_index(i, FREEZE_THAW_TOKEN_COUNT, TOKEN_PITCH),
                0.0,
                4.0,
            );
    }
    rail - token_cuts + cycle_count_stop_tabs() + drift_flag_land()
}

fn cycle_count_stop_tabs() -> Part {
    let mut stops = Part::empty("archive_freeze_thaw_cycle_count_stop_tabs");
    for i in [0, FREEZE_THAW_TOKEN_COUNT - 1] {
        stops = stops
            + centered_cube(
                format!("archive_freeze_thaw_cycle_stop_tab_{i}"),
                16.0,
                TOKEN_Y + 26.0,
                20.0,
            )
            .translate(
                centered_index(i, FREEZE_THAW_TOKEN_COUNT, TOKEN_PITCH),
                0.0,
                TOKEN_Z / 2.0 + 10.0,
            );
    }
    stops
}

fn drift_flag_land() -> Part {
    centered_cube(
        "archive_freeze_thaw_cycle_drift_flag_land",
        130.0,
        24.0,
        8.0,
    )
    .translate(0.0, TOKEN_Y / 2.0 + 18.0, TOKEN_Z / 2.0 + 4.0)
}

fn cap_seal_witness_pockets() -> Part {
    let block = centered_cube(
        "archive_freeze_thaw_cap_seal_witness_block",
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let mut cuts = Part::empty("archive_freeze_thaw_cap_seal_witness_cuts");
    for i in 0..CAP_WITNESS_POCKETS {
        let row = i / 4;
        let col = i % 4;
        cuts = cuts
            + centered_cube(
                format!("archive_freeze_thaw_cap_witness_pocket_{i}"),
                46.0,
                30.0,
                SEAL_Z + 4.0,
            )
            .translate(
                centered_index(col, 4, 68.0),
                centered_index(row, 2, 54.0),
                8.0,
            );
    }

    block - cuts + seal_strip_lands() + failed_cap_quarantine_notch()
}

fn seal_strip_lands() -> Part {
    let mut lands = Part::empty("archive_freeze_thaw_tamper_seal_strip_lands");
    for i in 0..SEAL_STRIP_LANDS {
        lands = lands
            + centered_cube(
                format!("archive_freeze_thaw_tamper_seal_strip_land_{i}"),
                52.0,
                12.0,
                6.0,
            )
            .translate(
                centered_index(i, SEAL_STRIP_LANDS, 54.0),
                -SEAL_Y / 2.0 - 12.0,
                SEAL_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn failed_cap_quarantine_notch() -> Part {
    centered_cube(
        "archive_freeze_thaw_failed_cap_quarantine_notch",
        72.0,
        18.0,
        18.0,
    )
    .translate(SEAL_X / 2.0 - 56.0, SEAL_Y / 2.0 + 9.0, 0.0)
}

fn barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "archive_freeze_thaw_custody_capture_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel - barcode_land_recesses()
        + rfid_lands()
        + custody_signature_slots()
        + custody_state_ticks()
}

fn barcode_land_recesses() -> Part {
    let mut recesses = Part::empty("archive_freeze_thaw_barcode_land_recesses");
    for i in 0..BARCODE_LANDS {
        let row = i / 6;
        let col = i % 6;
        recesses = recesses
            + centered_cube(
                format!("archive_freeze_thaw_barcode_recess_{i}"),
                54.0,
                18.0,
                CUSTODY_Z + 2.0,
            )
            .translate(
                centered_index(col, 6, 62.0),
                centered_index(row, 2, 45.0),
                5.0,
            );
    }
    recesses
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("archive_freeze_thaw_rfid_lands");
    for i in 0..RFID_LANDS {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(
                format!("archive_freeze_thaw_rfid_land_{i}"),
                30.0,
                30.0,
                5.0,
            )
            .translate(
                centered_index(col, 4, 50.0),
                centered_index(row, 2, 54.0),
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn custody_signature_slots() -> Part {
    let mut slots = Part::empty("archive_freeze_thaw_custody_signature_card_slots");
    for i in 0..SIGNATURE_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("archive_freeze_thaw_custody_signature_card_slot_{i}"),
                18.0,
                72.0,
                18.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 42.0 + i as f64 * 30.0,
                CUSTODY_Y / 2.0 - 50.0,
                CUSTODY_Z / 2.0 + 9.0,
            );
    }
    slots
}

fn custody_state_ticks() -> Part {
    let mut ticks = Part::empty("archive_freeze_thaw_custody_state_ticks");
    for (i, state) in CUSTODY_STATES.iter().enumerate() {
        ticks = ticks
            + centered_cube(
                format!("archive_freeze_thaw_custody_state_{state}_tick"),
                44.0,
                8.0,
                8.0,
            )
            .translate(
                CUSTODY_X / 2.0 - 62.0,
                centered_index(i, CUSTODY_STATES.len(), 34.0),
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    ticks
}

fn carryover_flush_witness_wells() -> Part {
    let block = centered_cube(
        "archive_freeze_thaw_carryover_flush_witness_block",
        FLUSH_X,
        FLUSH_Y,
        FLUSH_Z,
    );
    let mut cuts = Part::empty("archive_freeze_thaw_flush_witness_well_cuts");
    for i in 0..FLUSH_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("archive_freeze_thaw_flush_witness_well_{i}"),
                11.0,
                FLUSH_Z + 4.0,
                32,
            )
            .translate(centered_index(i, FLUSH_WELLS, 43.0), -28.0, 8.0);
    }
    for i in 0..BLANK_WITNESS_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("archive_freeze_thaw_blank_witness_well_{i}"),
                13.0,
                FLUSH_Z + 4.0,
                32,
            )
            .translate(centered_index(i, BLANK_WITNESS_WELLS, 58.0), 42.0, 8.0);
    }

    block - cuts + flush_manifold_stub() + carryover_fail_flag_land()
}

fn flush_manifold_stub() -> Part {
    centered_cube(
        "archive_freeze_thaw_flush_manifold_closed_connector_land",
        FLUSH_X - 60.0,
        18.0,
        18.0,
    )
    .translate(0.0, -FLUSH_Y / 2.0 - 9.0, 0.0)
}

fn carryover_fail_flag_land() -> Part {
    centered_cube(
        "archive_freeze_thaw_carryover_fail_flag_land",
        112.0,
        24.0,
        8.0,
    )
    .translate(
        FLUSH_X / 2.0 - 72.0,
        FLUSH_Y / 2.0 + 14.0,
        FLUSH_Z / 2.0 + 4.0,
    )
}

fn temperature_logger_pockets() -> Part {
    let block = centered_cube(
        "archive_freeze_thaw_temperature_logger_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts = Part::empty("archive_freeze_thaw_temperature_logger_pocket_cuts");
    for i in 0..LOGGER_POCKETS {
        cuts = cuts
            + centered_cube(
                format!("archive_freeze_thaw_temperature_logger_pocket_{i}"),
                78.0,
                44.0,
                LOGGER_Z + 4.0,
            )
            .translate(0.0, centered_index(i, LOGGER_POCKETS, 64.0), 9.0);
    }
    for i in 0..PROBE_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                format!("archive_freeze_thaw_probe_channel_{i}"),
                3.2,
                LOGGER_X + 8.0,
                18,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, centered_index(i, PROBE_CHANNELS, 32.0), -8.0);
    }
    block - cuts + logger_pull_tabs()
}

fn logger_pull_tabs() -> Part {
    let mut tabs = Part::empty("archive_freeze_thaw_logger_pull_tabs");
    for i in 0..LOGGER_POCKETS {
        tabs = tabs
            + centered_cube(
                format!("archive_freeze_thaw_logger_pull_tab_{i}"),
                74.0,
                10.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, LOGGER_POCKETS, 64.0) - 27.0,
                LOGGER_Z / 2.0 + 5.0,
            );
    }
    tabs
}

fn released_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "archive_freeze_thaw_disposition_lane_base",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    let mut slot_cuts = Part::empty("archive_freeze_thaw_disposition_slot_cuts");
    let mut lane_flags = Part::empty("archive_freeze_thaw_disposition_lane_flags");
    for (lane, name) in STATUS_LANES.iter().enumerate() {
        let x = centered_index(lane, STATUS_LANES.len(), STATUS_LANE_PITCH_X);
        lane_flags = lane_flags
            + centered_cube(
                format!("archive_freeze_thaw_{name}_lane_header_flag"),
                STATUS_SLOT_X,
                12.0,
                10.0,
            )
            .translate(x, LANES_Y / 2.0 + 10.0, LANES_Z / 2.0 + 5.0);
        for slot in 0..SLOTS_PER_STATUS {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("archive_freeze_thaw_{name}_lane_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    LANES_Z + 4.0,
                )
                .translate(x, centered_index(slot, SLOTS_PER_STATUS, 40.0), 6.0);
        }
    }
    base - slot_cuts + lane_flags
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "archive_freeze_thaw_camera_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let right_post = centered_cube(
        "archive_freeze_thaw_camera_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let beam = centered_cube(
        "archive_freeze_thaw_camera_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    left_post + right_post + beam + camera_pods() + evidence_led_segments()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("archive_freeze_thaw_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        pods = pods
            + centered_cube(
                format!("archive_freeze_thaw_evidence_camera_pod_{i}"),
                64.0,
                54.0,
                24.0,
            )
            .translate(
                centered_index(i, CAMERA_PODS, 260.0),
                -BRIDGE_POST_Y / 2.0 - 18.0,
                BRIDGE_UNDERSIDE_Z / 2.0 - 32.0,
            );
    }
    pods
}

fn evidence_led_segments() -> Part {
    let mut leds = Part::empty("archive_freeze_thaw_evidence_led_segments");
    for i in 0..EVIDENCE_LED_SEGMENTS {
        leds = leds
            + centered_cube(
                format!("archive_freeze_thaw_evidence_led_segment_{i}"),
                72.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LED_SEGMENTS, 104.0),
                BRIDGE_POST_Y / 2.0 + 6.0,
                BRIDGE_UNDERSIDE_Z / 2.0 - 22.0,
            );
    }
    leds
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "archive_freeze_thaw_front_robot_keepout_gauge",
        STATION_X - 180.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE, 0.0);
    let rear = centered_cube(
        "archive_freeze_thaw_rear_freezer_service_keepout_gauge",
        STATION_X - 260.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_FREEZER_SERVICE_CLEARANCE, 0.0);
    let left = centered_cube(
        "archive_freeze_thaw_left_archive_cart_keepout_gauge",
        12.0,
        STATION_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 - SIDE_ARCHIVE_CART_CLEARANCE, 0.0, 0.0);
    let overhead = centered_cube(
        "archive_freeze_thaw_overhead_camera_keepout_gauge",
        STATION_X - 320.0,
        STATION_Y - 260.0,
        8.0,
    )
    .translate(0.0, 0.0, OVERHEAD_CAMERA_CLEARANCE);
    front + rear + left + overhead
}

fn custody_mismatch_lockout_gates() -> Part {
    let rail = centered_cube(
        "archive_freeze_thaw_custody_mismatch_lockout_rail",
        LOCKOUT_X,
        LOCKOUT_Y,
        LOCKOUT_Z,
    );
    let mut gate_cuts = Part::empty("archive_freeze_thaw_custody_lockout_gate_cuts");
    let mut flags = Part::empty("archive_freeze_thaw_custody_error_flag_slots");
    for i in 0..LOCKOUT_GATES {
        let x = centered_index(i, LOCKOUT_GATES, 126.0);
        gate_cuts = gate_cuts
            + centered_cube(
                format!("archive_freeze_thaw_lockout_gate_clearance_{i}"),
                82.0,
                44.0,
                LOCKOUT_Z + 4.0,
            )
            .translate(x, -18.0, 8.0);
        flags = flags
            + centered_cube(
                format!("archive_freeze_thaw_error_flag_slot_{i}"),
                72.0,
                10.0,
                12.0,
            )
            .translate(x, LOCKOUT_Y / 2.0 + 12.0, LOCKOUT_Z / 2.0 + 6.0);
    }
    rail - gate_cuts + flags + mismatch_return_chute()
}

fn mismatch_return_chute() -> Part {
    centered_cube(
        "archive_freeze_thaw_custody_mismatch_return_chute",
        LOCKOUT_X - 120.0,
        26.0,
        20.0,
    )
    .translate(0.0, -LOCKOUT_Y / 2.0 - 16.0, -LOCKOUT_Z / 2.0 + 10.0)
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    centered_cube(name.into(), outer_x, outer_y, z)
        - centered_cube(
            "archive_freeze_thaw_rectangular_frame_inner_cut",
            inner_x,
            inner_y,
            z + 2.0,
        )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_station_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_sample_archive_freeze_thaw_chain_of_custody_station_"
                ),
                "{path} is outside the station output namespace"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_subassemblies_match_archive_validation_station_request() {
        for feature in [
            "containment_cold_deck",
            "archive_tube_nest_array",
            "freezer_transfer_dock",
            "freeze_thaw_cycle_token_rail",
            "cap_seal_witness_pockets",
            "barcode_rfid_custody_lands",
            "carryover_flush_witness_wells",
            "temperature_logger_pockets",
            "released_hold_reject_lanes",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
            "custody_mismatch_lockout_gates",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn key_modules_fit_inside_containment_deck() {
        assert_layout();
        for (_, center, width, depth) in component_specs() {
            assert!(fits_on_station(center, width, depth));
        }
    }

    #[test]
    fn archive_tube_identity_and_freeze_thaw_counts_are_explicit() {
        assert_eq!(TUBE_ROWS, 4);
        assert_eq!(TUBE_COLS, 8);
        assert_eq!(TUBE_COUNT, 32);
        assert_eq!(NEST_DATUM_PINS, 4);
        assert_eq!(FREEZE_THAW_TOKEN_COUNT, 10);
        assert!(TOKEN_PITCH > TUBE_WELL_D * 2.0);
        assert!(TUBE_PITCH_X > TUBE_WELL_D * 2.0);
        assert!(TUBE_PITCH_Y > TUBE_WELL_D * 2.5);
    }

    #[test]
    fn cold_chain_and_temperature_evidence_are_dimensioned() {
        assert!(COLD_PLATE_X > NEST_X + DOCK_X / 2.0);
        assert!(COLD_PLATE_Y > NEST_Y + TOKEN_Y);
        assert_eq!(FREEZER_CASSETTE_SLOTS, 3);
        assert_eq!(TRANSFER_TIMER_WINDOWS, 4);
        assert_eq!(LOGGER_POCKETS, 3);
        assert_eq!(PROBE_CHANNELS, 6);
        assert!(CONDENSATE_CHANNEL_DEPTH >= 7.0);
        assert!(DRAIN_D >= 12.0);
    }

    #[test]
    fn custody_seal_carryover_and_disposition_controls_are_locked() {
        assert_eq!(CAP_WITNESS_POCKETS, 8);
        assert_eq!(SEAL_STRIP_LANDS, 6);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 8);
        assert_eq!(SIGNATURE_CARD_SLOTS, CUSTODY_STATES.len());
        assert_eq!(FLUSH_WELLS, 8);
        assert_eq!(BLANK_WITNESS_WELLS, 4);
        assert_eq!(STATUS_LANES, ["released", "hold", "reject"]);
        assert_eq!(SLOTS_PER_STATUS * STATUS_LANES.len(), 12);
        assert!(STATUS_LANE_PITCH_X - STATUS_SLOT_X >= MIN_STATUS_GAP);
    }

    #[test]
    fn evidence_bridge_lockouts_and_keepouts_are_reserved() {
        assert_eq!(CAMERA_PODS, 4);
        assert_eq!(EVIDENCE_LED_SEGMENTS, 10);
        assert!(BRIDGE_SPAN_X > NEST_X + DOCK_X);
        assert!(BRIDGE_UNDERSIDE_Z > NEST_Z + LOGGER_Z);
        assert_eq!(KEEP_OUT_GAUGES, 4);
        assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
        assert!(REAR_FREEZER_SERVICE_CLEARANCE >= 300.0);
        assert!(OVERHEAD_CAMERA_CLEARANCE >= BRIDGE_UNDERSIDE_Z);
        assert_eq!(LOCKOUT_GATES, 5);
        assert_eq!(ERROR_FLAG_SLOTS, LOCKOUT_GATES);
    }
}
