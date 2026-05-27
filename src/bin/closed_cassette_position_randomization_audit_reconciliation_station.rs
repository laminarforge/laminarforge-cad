use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette position-randomization audit/reconciliation station.
//
// No-cell validation geometry for confirming that a randomized chip-to-position
// assignment can be reconciled against physical cassette slot identity,
// barcode/RFID reads, blind operator/batch tokens, edge/center balance, and
// quarantine/rework paths before a multi-chip culture run is released.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_position_randomization_audit_reconciliation_station_audit_deck.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_twenty_position_cassette_surrogate_grid.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_randomized_assignment_token_tray.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_barcode_rfid_scan_lands.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_mismatch_quarantine_pocket.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_operator_batch_blind_code_token_rails.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_edge_center_balance_witness_markers.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_physical_slot_identity_keys.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_evidence_camera_bridge.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_position_randomization_audit_reconciliation_station_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;
const RANDOMIZED_ASSIGNMENT: [usize; POSITION_COUNT] = [
    11, 2, 18, 7, 14, 0, 9, 16, 5, 12, 3, 19, 8, 15, 1, 10, 17, 6, 13, 4,
];

const DECK_X: f64 = 1480.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 28.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CHIP_GUTTER: f64 = 6.0;
const GRID_CENTER: (f64, f64) = (-90.0, 92.0);
const GRID_MARGIN_X: f64 = 34.0;
const GRID_MARGIN_Y: f64 = 34.0;
const GRID_SLOT_X: f64 = REVC_CHIP_LENGTH + 8.0;
const GRID_SLOT_Y: f64 = REVC_CHIP_WIDTH + 8.0;
const GRID_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const GRID_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const GRID_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER;
const GRID_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER;
const GRID_X: f64 = GRID_ARRAY_X + 2.0 * GRID_MARGIN_X;
const GRID_Y: f64 = GRID_ARRAY_Y + 2.0 * GRID_MARGIN_Y;
const GRID_Z: f64 = 40.0;
const GRID_RECESS_DEPTH: f64 = 14.0;
const GRID_RAIL_W: f64 = 14.0;
const GRID_RAIL_Z: f64 = 24.0;

const TOKEN_TRAY_CENTER: (f64, f64) = (-585.0, 110.0);
const TOKEN_TRAY_X: f64 = 264.0;
const TOKEN_TRAY_Y: f64 = 360.0;
const TOKEN_TRAY_Z: f64 = 30.0;
const TOKEN_D: f64 = 18.0;
const TOKEN_RIM_D: f64 = 28.0;
const TOKEN_PITCH_X: f64 = 48.0;
const TOKEN_PITCH_Y: f64 = 54.0;
const TOKEN_HOME_COUNT: usize = POSITION_COUNT;

const SCAN_CENTER: (f64, f64) = (-210.0, -290.0);
const SCAN_PANEL_X: f64 = 650.0;
const SCAN_PANEL_Y: f64 = 130.0;
const SCAN_PANEL_Z: f64 = 16.0;
const BARCODE_LAND_X: f64 = 92.0;
const BARCODE_LAND_Y: f64 = 20.0;
const RFID_LAND_D: f64 = 26.0;
const SCAN_PAIR_COUNT: usize = POSITION_COUNT;

const QUARANTINE_CENTER: (f64, f64) = (480.0, -290.0);
const QUARANTINE_X: f64 = 300.0;
const QUARANTINE_Y: f64 = 162.0;
const QUARANTINE_Z: f64 = 54.0;
const QUARANTINE_POCKET_X: f64 = 116.0;
const QUARANTINE_POCKET_Y: f64 = 104.0;
const QUARANTINE_WALL_Z: f64 = 62.0;

const BLIND_RAIL_CENTER: (f64, f64) = (482.0, 228.0);
const BLIND_RAIL_PANEL_X: f64 = 420.0;
const BLIND_RAIL_PANEL_Y: f64 = 214.0;
const BLIND_RAIL_PANEL_Z: f64 = 24.0;
const BLIND_RAIL_COUNT: usize = 4;
const BLIND_TOKEN_SLOTS_PER_RAIL: usize = 5;
const BLIND_RAIL_SLOT_X: f64 = 54.0;
const BLIND_RAIL_SLOT_Y: f64 = 20.0;
const BLIND_RAIL_PITCH_Y: f64 = 44.0;

const LANE_CENTER: (f64, f64) = (482.0, -44.0);
const LANE_COUNT: usize = 3;
const LANE_PANEL_X: f64 = 438.0;
const LANE_PANEL_Y: f64 = 196.0;
const LANE_PANEL_Z: f64 = 28.0;
const LANE_X: f64 = 122.0;
const LANE_Y: f64 = 146.0;
const LANE_WALL_W: f64 = 8.0;
const LANE_PITCH_X: f64 = 142.0;

const WITNESS_CENTER: (f64, f64) = (-586.0, -234.0);
const WITNESS_PANEL_X: f64 = 266.0;
const WITNESS_PANEL_Y: f64 = 228.0;
const WITNESS_PANEL_Z: f64 = 16.0;
const EDGE_MARKER_D: f64 = 17.0;
const CENTER_MARKER_D: f64 = 24.0;
const WITNESS_MARKER_Z: f64 = 5.0;

const SLOT_KEY_COUNT: usize = POSITION_COUNT;
const SLOT_KEY_D: f64 = 8.0;
const SLOT_KEY_Z: f64 = 12.0;
const SLOT_KEY_RAIL_Z: f64 = 6.0;

const BRIDGE_CENTER: (f64, f64) = (-40.0, 90.0);
const BRIDGE_SPAN_X: f64 = 1260.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 58.0;
const BRIDGE_BEAM_Y: f64 = 70.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const BRIDGE_UNDERSIDE_Z: f64 = 188.0;
const CAMERA_POD_COUNT: usize = 4;
const CAMERA_POD_X: f64 = 92.0;
const CAMERA_POD_Y: f64 = 62.0;
const CAMERA_POD_Z: f64 = 42.0;
const CAMERA_LENS_D: f64 = 24.0;

const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_FRONT_KEEP_OUT_Y: f64 = 118.0;
const ROBOT_REAR_KEEP_OUT_Y: f64 = 96.0;
const SERVICE_SIDE_KEEP_OUT_X: f64 = 126.0;
const SERVICE_CLEARANCE_Z: f64 = 118.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = audit_deck();
    export(OUTPUTS[0], &deck);

    let grid = twenty_position_cassette_surrogate_grid();
    export(OUTPUTS[1], &grid);

    let tokens = randomized_assignment_token_tray();
    export(OUTPUTS[2], &tokens);

    let scans = barcode_rfid_scan_lands();
    export(OUTPUTS[3], &scans);

    let quarantine = mismatch_quarantine_pocket();
    export(OUTPUTS[4], &quarantine);

    let blind = operator_batch_blind_code_token_rails();
    export(OUTPUTS[5], &blind);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[6], &lanes);

    let witness = edge_center_balance_witness_markers();
    export(OUTPUTS[7], &witness);

    let keys = physical_slot_identity_keys();
    export(OUTPUTS[8], &keys);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette position-randomization audit reconciliation station:");
    println!(
        "  No-cell cassette audit:      {CASSETTE_COLS}x{CASSETTE_ROWS} surrogate grid ({POSITION_COUNT} physical slots)"
    );
    println!(
        "  Randomization controls:      {TOKEN_HOME_COUNT} assignment token homes with deterministic 20-position audit map"
    );
    println!(
        "  Identity reconciliation:     {SCAN_PAIR_COUNT} barcode lands and {SCAN_PAIR_COUNT} RFID scan lands"
    );
    println!(
        "  Bias controls:               {BLIND_RAIL_COUNT} blind-code rails, {BLIND_TOKEN_SLOTS_PER_RAIL} tokens per rail"
    );
    println!(
        "  Balance witnesses:           {EDGE_POSITION_COUNT} edge markers and {CENTER_POSITION_COUNT} center markers"
    );
    println!(
        "  Disposition paths:           release/hold/reject lanes plus isolated mismatch quarantine pocket"
    );
    println!(
        "  Evidence bridge clearance:   {:.0}mm above deck top with {CAMERA_POD_COUNT} camera pods",
        bridge_clearance_above_deck()
    );
    println!("  STL outputs:                 {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    audit_deck()
        + twenty_position_cassette_surrogate_grid()
        + randomized_assignment_token_tray()
        + barcode_rfid_scan_lands()
        + mismatch_quarantine_pocket()
        + operator_batch_blind_code_token_rails()
        + release_hold_reject_lanes()
        + edge_center_balance_witness_markers()
        + physical_slot_identity_keys()
        + evidence_camera_bridge()
        + robot_service_keepout_gauges()
}

fn assert_layout() {
    assert_eq!(POSITION_COUNT, 20);
    assert_eq!(EDGE_POSITION_COUNT + CENTER_POSITION_COUNT, POSITION_COUNT);
    assert!(is_assignment_permutation());
    assert_eq!(assignment_home_count(), POSITION_COUNT);
    assert_eq!(barcode_rfid_pair_count(), POSITION_COUNT);
    assert_eq!(physical_slot_key_count(), POSITION_COUNT);
    assert!(grid_left_edge() > token_tray_right_edge() + 36.0);
    assert!(lane_to_quarantine_gap() >= 48.0);
    assert!(bridge_clearance_above_deck() > REVC_TOTAL_HEIGHT + 120.0);
    assert!(SERVICE_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 58.0);
}

fn audit_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_randomization_audit_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - deck_recesses() + deck_perimeter_rim() + audit_flow_arrow_ribs()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_cassette_randomization_audit_deck_mount_holes");
    for (index, (x, y)) in [
        (-DECK_X / 2.0 + 44.0, -DECK_Y / 2.0 + 44.0),
        (DECK_X / 2.0 - 44.0, -DECK_Y / 2.0 + 44.0),
        (-DECK_X / 2.0 + 44.0, DECK_Y / 2.0 - 44.0),
        (DECK_X / 2.0 - 44.0, DECK_Y / 2.0 - 44.0),
        (0.0, -DECK_Y / 2.0 + 44.0),
        (0.0, DECK_Y / 2.0 - 44.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_randomization_audit_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                36,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_recesses() -> Part {
    let mut recesses = Part::empty("closed_cassette_randomization_audit_deck_recesses");
    for (name, (x, y), sx, sy) in [
        ("grid", GRID_CENTER, GRID_X + 24.0, GRID_Y + 24.0),
        (
            "token_tray",
            TOKEN_TRAY_CENTER,
            TOKEN_TRAY_X + 20.0,
            TOKEN_TRAY_Y + 20.0,
        ),
        (
            "scan_panel",
            SCAN_CENTER,
            SCAN_PANEL_X + 18.0,
            SCAN_PANEL_Y + 18.0,
        ),
        (
            "quarantine",
            QUARANTINE_CENTER,
            QUARANTINE_X + 18.0,
            QUARANTINE_Y + 18.0,
        ),
        (
            "blind_rails",
            BLIND_RAIL_CENTER,
            BLIND_RAIL_PANEL_X + 18.0,
            BLIND_RAIL_PANEL_Y + 18.0,
        ),
        (
            "lanes",
            LANE_CENTER,
            LANE_PANEL_X + 18.0,
            LANE_PANEL_Y + 18.0,
        ),
        (
            "witness",
            WITNESS_CENTER,
            WITNESS_PANEL_X + 18.0,
            WITNESS_PANEL_Y + 18.0,
        ),
    ] {
        recesses = recesses
            + centered_cube(
                format!("closed_cassette_randomization_audit_{name}_registration_recess"),
                sx,
                sy,
                5.2,
            )
            .translate(x, y, DECK_Z - 2.4);
    }
    recesses
}

fn deck_perimeter_rim() -> Part {
    let front = centered_cube(
        "closed_cassette_randomization_audit_front_wipe_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_cassette_randomization_audit_rear_wipe_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_cassette_randomization_audit_left_wipe_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_cassette_randomization_audit_right_wipe_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn audit_flow_arrow_ribs() -> Part {
    let mut ribs = Part::empty("closed_cassette_randomization_audit_flow_arrow_ribs");
    for (index, x) in [-470.0, -40.0, 270.0].into_iter().enumerate() {
        let shaft = centered_cube(
            format!("closed_cassette_randomization_audit_flow_rib_{index}_shaft"),
            82.0,
            7.0,
            7.0,
        )
        .translate(x, -118.0, DECK_Z + 3.5);
        let head = centered_cube(
            format!("closed_cassette_randomization_audit_flow_rib_{index}_head"),
            18.0,
            22.0,
            7.0,
        )
        .translate(x + 50.0, -118.0, DECK_Z + 3.5);
        ribs = ribs + shaft + head;
    }
    ribs
}

fn twenty_position_cassette_surrogate_grid() -> Part {
    let base = centered_cube(
        "closed_cassette_randomization_twenty_position_grid_base",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let mut slot_reliefs = Part::empty("closed_cassette_randomization_grid_slot_reliefs");
    let mut slot_rims = Part::empty("closed_cassette_randomization_grid_slot_rims");

    for pos in 0..POSITION_COUNT {
        let (x, y) = grid_local_position(pos);
        slot_reliefs = slot_reliefs
            + centered_cube(
                format!("closed_cassette_randomization_position_{pos:02}_closed_chip_relief"),
                GRID_SLOT_X,
                GRID_SLOT_Y,
                GRID_RECESS_DEPTH + 1.0,
            )
            .translate(x, y, GRID_Z / 2.0 - GRID_RECESS_DEPTH / 2.0 + 0.5);
        slot_rims = slot_rims
            + centered_cube(
                format!("closed_cassette_randomization_position_{pos:02}_front_identity_land"),
                GRID_SLOT_X * 0.42,
                7.0,
                8.0,
            )
            .translate(x, y - GRID_SLOT_Y / 2.0 - 8.0, GRID_Z / 2.0 + 4.0);
    }

    let left_rail = centered_cube(
        "closed_cassette_randomization_grid_left_datum_rail",
        GRID_RAIL_W,
        GRID_Y,
        GRID_RAIL_Z,
    )
    .translate(
        -GRID_X / 2.0 + GRID_RAIL_W / 2.0,
        0.0,
        GRID_Z / 2.0 + GRID_RAIL_Z / 2.0,
    );
    let rear_rail = centered_cube(
        "closed_cassette_randomization_grid_rear_datum_rail",
        GRID_X,
        GRID_RAIL_W,
        GRID_RAIL_Z,
    )
    .translate(
        0.0,
        GRID_Y / 2.0 - GRID_RAIL_W / 2.0,
        GRID_Z / 2.0 + GRID_RAIL_Z / 2.0,
    );

    (base - slot_reliefs + slot_rims + left_rail + rear_rail).translate(
        GRID_CENTER.0,
        GRID_CENTER.1,
        DECK_Z + GRID_Z / 2.0,
    )
}

fn randomized_assignment_token_tray() -> Part {
    let plate = centered_cube(
        "closed_cassette_randomization_assignment_token_tray_plate",
        TOKEN_TRAY_X,
        TOKEN_TRAY_Y,
        TOKEN_TRAY_Z,
    );
    let mut token_reliefs = Part::empty("closed_cassette_randomization_assignment_token_reliefs");
    let mut token_rims = Part::empty("closed_cassette_randomization_assignment_token_rims");

    for home in 0..TOKEN_HOME_COUNT {
        let assigned = RANDOMIZED_ASSIGNMENT[home];
        let (x, y) = token_home_position(home);
        token_reliefs = token_reliefs
            + centered_cylinder(
                format!(
                    "closed_cassette_randomization_home_{home:02}_assigned_position_{assigned:02}_token_relief"
                ),
                TOKEN_D / 2.0,
                8.0,
                36,
            )
            .translate(x, y, TOKEN_TRAY_Z / 2.0 - 3.0);
        token_rims = token_rims
            + centered_cylinder(
                format!(
                    "closed_cassette_randomization_home_{home:02}_assigned_position_{assigned:02}_token_rim"
                ),
                TOKEN_RIM_D / 2.0,
                4.0,
                36,
            )
            .translate(x, y, TOKEN_TRAY_Z / 2.0 + 2.0);
    }

    (plate - token_reliefs + token_rims).translate(
        TOKEN_TRAY_CENTER.0,
        TOKEN_TRAY_CENTER.1,
        DECK_Z + TOKEN_TRAY_Z / 2.0,
    )
}

fn barcode_rfid_scan_lands() -> Part {
    let panel = centered_cube(
        "closed_cassette_randomization_barcode_rfid_scan_panel",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    );
    let mut lands = Part::empty("closed_cassette_randomization_barcode_rfid_scan_lands");
    for pos in 0..POSITION_COUNT {
        let col = pos % 10;
        let row = pos / 10;
        let x = (col as f64 - 4.5) * 62.0;
        let y = (row as f64 - 0.5) * 58.0;
        let barcode = centered_cube(
            format!("closed_cassette_randomization_position_{pos:02}_barcode_scan_land"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            4.0,
        )
        .translate(x, y - 12.0, SCAN_PANEL_Z / 2.0 + 2.0);
        let rfid = centered_cylinder(
            format!("closed_cassette_randomization_position_{pos:02}_rfid_scan_land"),
            RFID_LAND_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y + 17.0, SCAN_PANEL_Z / 2.0 + 2.5);
        lands = lands + barcode + rfid;
    }
    (panel + lands).translate(SCAN_CENTER.0, SCAN_CENTER.1, DECK_Z + SCAN_PANEL_Z / 2.0)
}

fn mismatch_quarantine_pocket() -> Part {
    let base = centered_cube(
        "closed_cassette_randomization_mismatch_quarantine_base",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let left_pocket = centered_cube(
        "closed_cassette_randomization_barcode_mismatch_pocket_relief",
        QUARANTINE_POCKET_X,
        QUARANTINE_POCKET_Y,
        24.0,
    )
    .translate(-72.0, 0.0, QUARANTINE_Z / 2.0 - 10.0);
    let right_pocket = centered_cube(
        "closed_cassette_randomization_rfid_mismatch_pocket_relief",
        QUARANTINE_POCKET_X,
        QUARANTINE_POCKET_Y,
        24.0,
    )
    .translate(72.0, 0.0, QUARANTINE_Z / 2.0 - 10.0);
    let high_wall = centered_cube(
        "closed_cassette_randomization_quarantine_high_back_wall",
        QUARANTINE_X,
        12.0,
        QUARANTINE_WALL_Z,
    )
    .translate(
        0.0,
        QUARANTINE_Y / 2.0 - 6.0,
        QUARANTINE_Z / 2.0 + QUARANTINE_WALL_Z / 2.0,
    );
    let bridge_lock = centered_cube(
        "closed_cassette_randomization_quarantine_rework_lock_bar",
        42.0,
        QUARANTINE_Y,
        18.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0 + 9.0);
    (base - left_pocket - right_pocket + high_wall + bridge_lock).translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        DECK_Z + QUARANTINE_Z / 2.0,
    )
}

fn operator_batch_blind_code_token_rails() -> Part {
    let panel = centered_cube(
        "closed_cassette_randomization_operator_batch_blind_code_panel",
        BLIND_RAIL_PANEL_X,
        BLIND_RAIL_PANEL_Y,
        BLIND_RAIL_PANEL_Z,
    );
    let mut reliefs = Part::empty("closed_cassette_randomization_blind_code_slot_reliefs");
    let mut rails = Part::empty("closed_cassette_randomization_blind_code_rails");
    for rail in 0..BLIND_RAIL_COUNT {
        let y = (rail as f64 - (BLIND_RAIL_COUNT as f64 - 1.0) / 2.0) * BLIND_RAIL_PITCH_Y;
        rails = rails
            + centered_cube(
                format!(
                    "closed_cassette_randomization_{}_blind_code_rail",
                    blind_rail_label(rail)
                ),
                BLIND_RAIL_PANEL_X - 42.0,
                8.0,
                12.0,
            )
            .translate(0.0, y, BLIND_RAIL_PANEL_Z / 2.0 + 6.0);
        for slot in 0..BLIND_TOKEN_SLOTS_PER_RAIL {
            let x = (slot as f64 - 2.0) * 70.0;
            reliefs = reliefs
                + centered_cube(
                    format!(
                        "closed_cassette_randomization_{}_blind_code_slot_{slot}",
                        blind_rail_label(rail)
                    ),
                    BLIND_RAIL_SLOT_X,
                    BLIND_RAIL_SLOT_Y,
                    9.0,
                )
                .translate(x, y, BLIND_RAIL_PANEL_Z / 2.0 - 3.0);
        }
    }
    (panel - reliefs + rails).translate(
        BLIND_RAIL_CENTER.0,
        BLIND_RAIL_CENTER.1,
        DECK_Z + BLIND_RAIL_PANEL_Z / 2.0,
    )
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_cassette_randomization_release_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut reliefs = Part::empty("closed_cassette_randomization_release_hold_reject_reliefs");
    let mut stops = Part::empty("closed_cassette_randomization_release_hold_reject_front_stops");
    for lane in 0..LANE_COUNT {
        let x = (lane as f64 - 1.0) * LANE_PITCH_X;
        let pocket = centered_cube(
            format!(
                "closed_cassette_randomization_{}_lane_pocket_relief",
                lane_label(lane)
            ),
            LANE_X,
            LANE_Y,
            13.0,
        )
        .translate(x, 0.0, LANE_PANEL_Z / 2.0 - 5.0);
        let front_wall = centered_cube(
            format!(
                "closed_cassette_randomization_{}_lane_front_stop",
                lane_label(lane)
            ),
            LANE_X,
            LANE_WALL_W,
            20.0,
        )
        .translate(x, -LANE_Y / 2.0, LANE_PANEL_Z / 2.0 + 10.0);
        reliefs = reliefs + pocket;
        stops = stops + front_wall;
    }
    (panel - reliefs + stops).translate(LANE_CENTER.0, LANE_CENTER.1, DECK_Z + LANE_PANEL_Z / 2.0)
}

fn edge_center_balance_witness_markers() -> Part {
    let panel = centered_cube(
        "closed_cassette_randomization_edge_center_balance_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let mut markers = Part::empty("closed_cassette_randomization_edge_center_witness_markers");
    for pos in 0..POSITION_COUNT {
        let (x, y) = witness_marker_position(pos);
        let edge = is_edge_position(pos);
        let d = if edge { EDGE_MARKER_D } else { CENTER_MARKER_D };
        markers = markers
            + centered_cylinder(
                format!(
                    "closed_cassette_randomization_position_{pos:02}_{}_balance_marker",
                    balance_label(edge)
                ),
                d / 2.0,
                WITNESS_MARKER_Z,
                36,
            )
            .translate(x, y, WITNESS_PANEL_Z / 2.0 + WITNESS_MARKER_Z / 2.0);
    }
    (panel + markers).translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        DECK_Z + WITNESS_PANEL_Z / 2.0,
    )
}

fn physical_slot_identity_keys() -> Part {
    let mut keys = Part::empty("closed_cassette_randomization_physical_slot_identity_keys");
    for pos in 0..POSITION_COUNT {
        let (x, y) = absolute_grid_position(pos);
        let pin = centered_cylinder(
            format!("closed_cassette_randomization_position_{pos:02}_physical_identity_pin"),
            SLOT_KEY_D / 2.0,
            SLOT_KEY_Z,
            28,
        )
        .translate(
            x - GRID_SLOT_X / 2.0 + 12.0,
            y + GRID_SLOT_Y / 2.0 - 12.0,
            DECK_Z + GRID_Z + SLOT_KEY_Z / 2.0,
        );
        let rail = centered_cube(
            format!("closed_cassette_randomization_position_{pos:02}_physical_identity_key_rail"),
            24.0 + (pos % CASSETTE_COLS) as f64 * 5.0,
            5.0,
            SLOT_KEY_RAIL_Z,
        )
        .translate(
            x,
            y + GRID_SLOT_Y / 2.0 + 8.0,
            DECK_Z + GRID_Z + SLOT_KEY_RAIL_Z / 2.0,
        );
        keys = keys + pin + rail;
    }
    keys
}

fn evidence_camera_bridge() -> Part {
    let post_z = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
    let left_post = centered_cube(
        "closed_cassette_randomization_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + post_z / 2.0,
    );
    let right_post = centered_cube(
        "closed_cassette_randomization_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + post_z / 2.0,
    );
    let beam = centered_cube(
        "closed_cassette_randomization_evidence_bridge_cross_beam",
        BRIDGE_SPAN_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    left_post + right_post + beam + camera_pods() + evidence_card_rails()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("closed_cassette_randomization_evidence_camera_pods");
    for pod in 0..CAMERA_POD_COUNT {
        let x = BRIDGE_CENTER.0 + (pod as f64 - 1.5) * 252.0;
        let body = centered_cube(
            format!("closed_cassette_randomization_evidence_camera_pod_{pod}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("closed_cassette_randomization_evidence_camera_lens_{pod}"),
            CAMERA_LENS_D / 2.0,
            8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BRIDGE_CENTER.1 - CAMERA_POD_Y / 2.0 - 3.0,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
        );
        pods = pods + body + lens;
    }
    pods
}

fn evidence_card_rails() -> Part {
    let lower = centered_cube(
        "closed_cassette_randomization_evidence_card_lower_rail",
        BRIDGE_SPAN_X - 240.0,
        8.0,
        10.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1 + 48.0,
        DECK_Z + BRIDGE_UNDERSIDE_Z - 34.0,
    );
    let upper = centered_cube(
        "closed_cassette_randomization_evidence_card_upper_rail",
        BRIDGE_SPAN_X - 240.0,
        8.0,
        10.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1 + 78.0,
        DECK_Z + BRIDGE_UNDERSIDE_Z - 34.0,
    );
    lower + upper
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "closed_cassette_randomization_front_robot_keepout_gauge",
        DECK_X - 160.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 92.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let rear_robot = centered_cube(
        "closed_cassette_randomization_rear_robot_keepout_gauge",
        DECK_X - 160.0,
        ROBOT_REAR_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 86.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let left_service = centered_cube(
        "closed_cassette_randomization_left_service_keepout_gauge",
        SERVICE_SIDE_KEEP_OUT_X,
        DECK_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + 82.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let right_service = centered_cube(
        "closed_cassette_randomization_right_service_keepout_gauge",
        SERVICE_SIDE_KEEP_OUT_X,
        DECK_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 - 82.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let service_height_gauge = centered_cube(
        "closed_cassette_randomization_service_clearance_height_gauge",
        42.0,
        260.0,
        SERVICE_CLEARANCE_Z,
    )
    .translate(682.0, 0.0, DECK_Z + SERVICE_CLEARANCE_Z / 2.0);
    front_robot + rear_robot + left_service + right_service + service_height_gauge
}

fn grid_local_position(pos: usize) -> (f64, f64) {
    let col = pos % CASSETTE_COLS;
    let row = pos / CASSETTE_COLS;
    (
        (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * GRID_PITCH_X,
        ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * GRID_PITCH_Y,
    )
}

fn absolute_grid_position(pos: usize) -> (f64, f64) {
    let (x, y) = grid_local_position(pos);
    (GRID_CENTER.0 + x, GRID_CENTER.1 + y)
}

fn token_home_position(home: usize) -> (f64, f64) {
    let col = home % 4;
    let row = home / 4;
    (
        (col as f64 - 1.5) * TOKEN_PITCH_X,
        (2.0 - row as f64) * TOKEN_PITCH_Y,
    )
}

fn witness_marker_position(pos: usize) -> (f64, f64) {
    let col = pos % CASSETTE_COLS;
    let row = pos / CASSETTE_COLS;
    ((col as f64 - 1.5) * 52.0, (2.0 - row as f64) * 38.0)
}

fn is_edge_position(pos: usize) -> bool {
    let col = pos % CASSETTE_COLS;
    let row = pos / CASSETTE_COLS;
    col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1
}

fn balance_label(edge: bool) -> &'static str {
    if edge {
        "edge"
    } else {
        "center"
    }
}

fn blind_rail_label(rail: usize) -> &'static str {
    match rail {
        0 => "operator_a",
        1 => "operator_b",
        2 => "batch_a",
        3 => "batch_b",
        _ => "unknown",
    }
}

fn lane_label(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn assignment_home_count() -> usize {
    TOKEN_HOME_COUNT
}

fn barcode_rfid_pair_count() -> usize {
    SCAN_PAIR_COUNT
}

fn physical_slot_key_count() -> usize {
    SLOT_KEY_COUNT
}

fn is_assignment_permutation() -> bool {
    let mut seen = [false; POSITION_COUNT];
    for assigned in RANDOMIZED_ASSIGNMENT {
        if assigned >= POSITION_COUNT || seen[assigned] {
            return false;
        }
        seen[assigned] = true;
    }
    seen.into_iter().all(|present| present)
}

fn bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

fn grid_left_edge() -> f64 {
    GRID_CENTER.0 - GRID_X / 2.0
}

fn token_tray_right_edge() -> f64 {
    TOKEN_TRAY_CENTER.0 + TOKEN_TRAY_X / 2.0
}

fn lane_bottom_edge() -> f64 {
    LANE_CENTER.1 - LANE_PANEL_Y / 2.0
}

fn quarantine_top_edge() -> f64 {
    QUARANTINE_CENTER.1 + QUARANTINE_Y / 2.0
}

fn lane_to_quarantine_gap() -> f64 {
    lane_bottom_edge() - quarantine_top_edge()
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
        assert!(OUTPUTS[0].ends_with("_audit_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cassette_position_randomization_audit_reconciliation_station_"
            ));
        }
    }

    #[test]
    fn cassette_grid_and_identity_controls_cover_twenty_positions() {
        assert_eq!(POSITION_COUNT, 20);
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(assignment_home_count(), POSITION_COUNT);
        assert_eq!(barcode_rfid_pair_count(), POSITION_COUNT);
        assert_eq!(physical_slot_key_count(), POSITION_COUNT);
        assert!(GRID_SLOT_X > REVC_CHIP_LENGTH);
        assert!(GRID_SLOT_Y > REVC_CHIP_WIDTH);
    }

    #[test]
    fn randomized_assignment_is_a_complete_permutation() {
        assert!(is_assignment_permutation());
        assert_ne!(RANDOMIZED_ASSIGNMENT[0], 0);
        assert_ne!(
            RANDOMIZED_ASSIGNMENT[POSITION_COUNT - 1],
            POSITION_COUNT - 1
        );
    }

    #[test]
    fn edge_center_witnesses_match_cassette_topology() {
        let edge_count = (0..POSITION_COUNT)
            .filter(|position| is_edge_position(*position))
            .count();
        let center_count = POSITION_COUNT - edge_count;
        assert_eq!(edge_count, EDGE_POSITION_COUNT);
        assert_eq!(center_count, CENTER_POSITION_COUNT);
        assert!(CENTER_MARKER_D > EDGE_MARKER_D);
    }

    #[test]
    fn quarantine_lanes_and_keepouts_have_audit_clearance() {
        assert!(lane_to_quarantine_gap() >= 48.0);
        assert!(grid_left_edge() > token_tray_right_edge() + 36.0);
        assert!(bridge_clearance_above_deck() > REVC_TOTAL_HEIGHT + 120.0);
        assert!(SERVICE_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 58.0);
    }
}
