use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental excursion root-cause evidence board for culture automation QA.
//
// Intent:
// - Package root-cause evidence from a closed environmental excursion in a
//   physical board: logger downloads, trend tokens, cassette position maps,
//   archive references, calibration certificates, photo evidence, decision
//   status, barcode/RFID identity, clean/used segregation, and robot/service
//   clearances.
// - Support evidence custody and QA investigation workflows after an excursion.
// - Avoid diagnostic claims: this is packaging/interface CAD, not biological
//   diagnosis, acceptance criteria, or release protocol.
//
// Research assumptions:
// - Environmental excursion procedures emphasize timely identification,
//   documentation, investigation, risk assessment, CAPA linkage, and recurrence
//   trending.
// - Chain-of-custody evidence needs aligned timestamps, independent logger files,
//   container identity, barcode/RFID scans, tamper-evident seal records, photos,
//   and traceable calibration state.
// - Investigation boards benefit from physically separated hold/release/reject
//   lanes and clean/used evidence handling so custody gaps are visible.

const OUTPUTS: [&str; 12] = [
    "output/closed_excursion_root_cause_evidence_board_base_tray.stl",
    "output/closed_excursion_root_cause_evidence_board_logger_download_slots.stl",
    "output/closed_excursion_root_cause_evidence_board_pressure_gas_trend_tokens.stl",
    "output/closed_excursion_root_cause_evidence_board_cassette_position_map_cards.stl",
    "output/closed_excursion_root_cause_evidence_board_media_sample_archive_references.stl",
    "output/closed_excursion_root_cause_evidence_board_calibration_certificate_pockets.stl",
    "output/closed_excursion_root_cause_evidence_board_photo_evidence_bridge.stl",
    "output/closed_excursion_root_cause_evidence_board_hold_release_reject_lanes.stl",
    "output/closed_excursion_root_cause_evidence_board_barcode_rfid_lands.stl",
    "output/closed_excursion_root_cause_evidence_board_clean_used_segregation.stl",
    "output/closed_excursion_root_cause_evidence_board_robot_service_keepouts.stl",
    "output/closed_excursion_root_cause_evidence_board_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "environmental_logger_download_slots",
    "pressure_gas_trend_tokens",
    "cassette_position_map_cards",
    "media_sample_archive_references",
    "sensor_calibration_certificate_pockets",
    "photo_evidence_bridge",
    "hold_release_reject_decision_lanes",
    "barcode_rfid_lands",
    "clean_used_segregation",
    "robot_service_keepouts",
    "assembly_export",
];

const BOARD_X: f64 = 1180.0;
const BOARD_Y: f64 = 760.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 32.0;
const SUMP_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 5.2;

const LOGGER_CENTER: (f64, f64) = (-405.0, 230.0);
const LOGGER_PANEL_X: f64 = 305.0;
const LOGGER_PANEL_Y: f64 = 180.0;
const LOGGER_PANEL_Z: f64 = 36.0;
const LOGGER_SLOTS: usize = 5;
const LOGGER_SLOT_X: f64 = 46.0;
const LOGGER_SLOT_Y: f64 = 110.0;
const LOGGER_SLOT_PITCH: f64 = 54.0;
const DOWNLOAD_DOCKS: usize = 3;

const TREND_CENTER: (f64, f64) = (-75.0, 235.0);
const TREND_PANEL_X: f64 = 330.0;
const TREND_PANEL_Y: f64 = 170.0;
const TREND_PANEL_Z: f64 = 30.0;
const PRESSURE_TOKEN_COUNT: usize = 8;
const GAS_TOKEN_COUNT: usize = 8;
const TREND_TOKEN_D: f64 = 22.0;
const TREND_TOKEN_PITCH: f64 = 38.0;

const MAP_CENTER: (f64, f64) = (300.0, 230.0);
const MAP_PANEL_X: f64 = 330.0;
const MAP_PANEL_Y: f64 = 186.0;
const MAP_PANEL_Z: f64 = 34.0;
const MAP_CARD_SLOTS: usize = 6;
const MAP_CARD_SLOT_X: f64 = 84.0;
const MAP_CARD_SLOT_Y: f64 = 44.0;
const MAP_CARD_PITCH_X: f64 = 102.0;
const MAP_CARD_PITCH_Y: f64 = 58.0;
const CASSETTE_POSITION_ROWS: usize = 4;
const CASSETTE_POSITION_COLS: usize = 5;
const CASSETTE_POSITION_COUNT: usize = CASSETTE_POSITION_ROWS * CASSETTE_POSITION_COLS;

const ARCHIVE_CENTER: (f64, f64) = (-390.0, 20.0);
const ARCHIVE_PANEL_X: f64 = 340.0;
const ARCHIVE_PANEL_Y: f64 = 170.0;
const ARCHIVE_PANEL_Z: f64 = 38.0;
const MEDIA_ARCHIVE_POCKETS: usize = 4;
const SAMPLE_ARCHIVE_POCKETS: usize = 4;
const ARCHIVE_POCKET_X: f64 = 62.0;
const ARCHIVE_POCKET_Y: f64 = 48.0;
const ARCHIVE_POCKET_PITCH_X: f64 = 78.0;
const ARCHIVE_POCKET_PITCH_Y: f64 = 62.0;

const CERT_CENTER: (f64, f64) = (-40.0, 10.0);
const CERT_PANEL_X: f64 = 330.0;
const CERT_PANEL_Y: f64 = 168.0;
const CERT_PANEL_Z: f64 = 30.0;
const CAL_CERT_POCKETS: usize = 6;
const CERT_SLOT_X: f64 = 88.0;
const CERT_SLOT_Y: f64 = 42.0;
const CERT_SLOT_PITCH_X: f64 = 104.0;
const CERT_SLOT_PITCH_Y: f64 = 58.0;
const SENSOR_CERT_LANDS: usize = 6;

const DECISION_CENTER: (f64, f64) = (342.0, -18.0);
const DECISION_PANEL_X: f64 = 360.0;
const DECISION_PANEL_Y: f64 = 245.0;
const DECISION_PANEL_Z: f64 = 34.0;
const DECISION_LANES: usize = 3;
const DECISION_SLOTS_PER_LANE: usize = 4;
const DECISION_TOTAL_SLOTS: usize = DECISION_LANES * DECISION_SLOTS_PER_LANE;
const DECISION_SLOT_X: f64 = 82.0;
const DECISION_SLOT_Y: f64 = 44.0;
const DECISION_LANE_PITCH_X: f64 = 112.0;
const DECISION_SLOT_PITCH_Y: f64 = 55.0;
const DECISION_LANE_GAP_MIN: f64 = 22.0;

const PHOTO_CENTER: (f64, f64) = (-40.0, -105.0);
const PHOTO_SPAN_X: f64 = 1010.0;
const PHOTO_POST_X: f64 = 26.0;
const PHOTO_POST_Y: f64 = 38.0;
const PHOTO_UNDERSIDE_Z: f64 = 164.0;
const PHOTO_BEAM_Z: f64 = 26.0;
const PHOTO_CAMERA_COUNT: usize = 3;
const PHOTO_LIGHT_SEGMENTS: usize = 8;
const EVIDENCE_WINDOW_COUNT: usize = 5;

const TRACE_CENTER: (f64, f64) = (-260.0, -255.0);
const TRACE_PANEL_X: f64 = 460.0;
const TRACE_PANEL_Y: f64 = 96.0;
const TRACE_PANEL_Z: f64 = 8.0;
const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 4;
const SEAL_ID_LANDS: usize = 6;

const SEG_CENTER: (f64, f64) = (260.0, -260.0);
const SEG_PANEL_X: f64 = 310.0;
const SEG_PANEL_Y: f64 = 108.0;
const SEG_PANEL_Z: f64 = 40.0;
const CLEAN_EVIDENCE_BAYS: usize = 4;
const USED_EVIDENCE_BAYS: usize = 4;
const SEG_BAY_X: f64 = 58.0;
const SEG_BAY_Y: f64 = 36.0;
const SEG_BAY_PITCH: f64 = 68.0;
const SEG_DIVIDER_Z: f64 = 72.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 340.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 235.0;
const LEFT_DATA_SERVICE_KEEP_OUT_X: f64 = 180.0;
const RIGHT_QA_REVIEW_KEEP_OUT_X: f64 = 245.0;
const TOP_PHOTO_BRIDGE_KEEP_OUT_Z: f64 = 210.0;
const KEEP_OUT_GAUGE_Z: f64 = 9.0;
const KEEP_OUT_ZONES: usize = 5;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let loggers = environmental_logger_download_slots();
    export(OUTPUTS[1], &loggers);

    let trend_tokens = pressure_gas_trend_tokens();
    export(OUTPUTS[2], &trend_tokens);

    let map_cards = cassette_position_map_cards();
    export(OUTPUTS[3], &map_cards);

    let archives = media_sample_archive_references();
    export(OUTPUTS[4], &archives);

    let certs = sensor_calibration_certificate_pockets();
    export(OUTPUTS[5], &certs);

    let photo_bridge = photo_evidence_bridge();
    export(OUTPUTS[6], &photo_bridge);

    let decision_lanes = hold_release_reject_decision_lanes();
    export(OUTPUTS[7], &decision_lanes);

    let traceability = barcode_rfid_lands();
    export(OUTPUTS[8], &traceability);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + loggers.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, deck_z(LOGGER_PANEL_Z))
        + trend_tokens.translate(TREND_CENTER.0, TREND_CENTER.1, deck_z(TREND_PANEL_Z))
        + map_cards.translate(MAP_CENTER.0, MAP_CENTER.1, deck_z(MAP_PANEL_Z))
        + archives.translate(ARCHIVE_CENTER.0, ARCHIVE_CENTER.1, deck_z(ARCHIVE_PANEL_Z))
        + certs.translate(CERT_CENTER.0, CERT_CENTER.1, deck_z(CERT_PANEL_Z))
        + photo_bridge.translate(PHOTO_CENTER.0, PHOTO_CENTER.1, BASE_Z)
        + decision_lanes.translate(
            DECISION_CENTER.0,
            DECISION_CENTER.1,
            deck_z(DECISION_PANEL_Z),
        )
        + traceability.translate(TRACE_CENTER.0, TRACE_CENTER.1, BASE_Z + TRACE_PANEL_Z / 2.0)
        + segregation.translate(SEG_CENTER.0, SEG_CENTER.1, deck_z(SEG_PANEL_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z + KEEP_OUT_GAUGE_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed excursion root-cause evidence board:");
    println!(
        "  Board/tray:                  {BOARD_X:.0}mm x {BOARD_Y:.0}mm x {BASE_Z:.0}mm with recessed custody sump"
    );
    println!(
        "  Logger evidence:             {LOGGER_SLOTS} logger download slots and {DOWNLOAD_DOCKS} cable/download docks"
    );
    println!(
        "  Trend evidence:              {PRESSURE_TOKEN_COUNT} pressure tokens and {GAS_TOKEN_COUNT} gas-trend tokens"
    );
    println!(
        "  Cassette map cards:           {MAP_CARD_SLOTS} card slots covering {CASSETTE_POSITION_COUNT} cassette positions"
    );
    println!(
        "  Archive references:           {MEDIA_ARCHIVE_POCKETS} media pockets and {SAMPLE_ARCHIVE_POCKETS} sample pockets"
    );
    println!(
        "  Sensor certificates:          {CAL_CERT_POCKETS} certificate pockets and {SENSOR_CERT_LANDS} sensor-calibration lands"
    );
    println!(
        "  Photo bridge:                 {PHOTO_CAMERA_COUNT} cameras, {PHOTO_LIGHT_SEGMENTS} light segments, {EVIDENCE_WINDOW_COUNT} evidence windows"
    );
    println!(
        "  Decision lanes:               hold/release/reject, {DECISION_SLOTS_PER_LANE} slots each ({DECISION_TOTAL_SLOTS} total)"
    );
    println!(
        "  Traceability/segregation:     {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {SEAL_ID_LANDS} seal-ID lands, clean/used bays"
    );
    println!(
        "  Keepouts:                     {KEEP_OUT_ZONES} robot/service/photo bridge keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected output part count");
    assert_eq!(DECISION_LANES, 3, "hold/release/reject lane count changed");
    assert_eq!(
        CASSETTE_POSITION_COUNT, 20,
        "cassette position map assumes a 20-position QA map"
    );

    for (name, center, width, depth) in insert_specs() {
        assert!(
            fits_on_board(center, width, depth, 12.0),
            "{name} exceeds evidence board envelope"
        );
    }

    assert!(
        decision_lane_gap() >= DECISION_LANE_GAP_MIN,
        "decision lanes are not visibly segregated"
    );
    assert!(
        !rects_overlap(
            rect(LOGGER_CENTER, LOGGER_PANEL_X, LOGGER_PANEL_Y),
            rect(TREND_CENTER, TREND_PANEL_X, TREND_PANEL_Y)
        ),
        "logger download slots overlap trend-token panel"
    );
    assert!(
        !rects_overlap(
            rect(ARCHIVE_CENTER, ARCHIVE_PANEL_X, ARCHIVE_PANEL_Y),
            rect(CERT_CENTER, CERT_PANEL_X, CERT_PANEL_Y)
        ),
        "archive references overlap certificate pockets"
    );
    assert!(
        !rects_overlap(
            rect(DECISION_CENTER, DECISION_PANEL_X, DECISION_PANEL_Y),
            rect(SEG_CENTER, SEG_PANEL_X, SEG_PANEL_Y)
        ),
        "decision lanes overlap clean/used segregation"
    );
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "environmental_logger_download_slots",
            LOGGER_CENTER,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        ),
        (
            "pressure_gas_trend_tokens",
            TREND_CENTER,
            TREND_PANEL_X,
            TREND_PANEL_Y,
        ),
        (
            "cassette_position_map_cards",
            MAP_CENTER,
            MAP_PANEL_X,
            MAP_PANEL_Y,
        ),
        (
            "media_sample_archive_references",
            ARCHIVE_CENTER,
            ARCHIVE_PANEL_X,
            ARCHIVE_PANEL_Y,
        ),
        (
            "sensor_calibration_certificate_pockets",
            CERT_CENTER,
            CERT_PANEL_X,
            CERT_PANEL_Y,
        ),
        (
            "hold_release_reject_decision_lanes",
            DECISION_CENTER,
            DECISION_PANEL_X,
            DECISION_PANEL_Y,
        ),
        (
            "barcode_rfid_lands",
            TRACE_CENTER,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "clean_used_segregation",
            SEG_CENTER,
            SEG_PANEL_X,
            SEG_PANEL_Y,
        ),
        (
            "photo_evidence_bridge",
            PHOTO_CENTER,
            PHOTO_SPAN_X,
            PHOTO_POST_Y,
        ),
    ]
}

fn fits_on_board(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0.abs() + width / 2.0 <= BOARD_X / 2.0 - RIM_W - margin
        && center.1.abs() + depth / 2.0 <= BOARD_Y / 2.0 - RIM_W - margin
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_excursion_evidence_board_deck",
        BOARD_X,
        BOARD_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sump = centered_cube(
        "closed_excursion_evidence_board_recessed_document_sump",
        BOARD_X - 110.0,
        BOARD_Y - 104.0,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z - SUMP_DEPTH / 2.0);

    deck - sump + tray_rim() + drain_gutters() + mounting_bosses()
}

fn tray_rim() -> Part {
    let left = centered_cube(
        "closed_excursion_evidence_board_left_raised_lip",
        RIM_W,
        BOARD_Y,
        RIM_Z,
    )
    .translate(-BOARD_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_excursion_evidence_board_right_raised_lip",
        RIM_W,
        BOARD_Y,
        RIM_Z,
    )
    .translate(BOARD_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_excursion_evidence_board_rear_raised_lip",
        BOARD_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, BOARD_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_excursion_evidence_board_front_low_review_lip",
        BOARD_X - 190.0,
        10.0,
        18.0,
    )
    .translate(0.0, -BOARD_Y / 2.0 + 8.0, BASE_Z + 9.0);

    left + right + rear + front
}

fn drain_gutters() -> Part {
    let mut gutters = Part::empty("closed_excursion_evidence_board_drain_gutters");
    for (i, y) in [-250.0, -125.0, 0.0, 125.0, 250.0].into_iter().enumerate() {
        gutters = gutters
            + centered_cube(
                format!("closed_excursion_evidence_board_cross_gutter_{i}"),
                BOARD_X - 180.0,
                6.0,
                4.0,
            )
            .translate(0.0, y, BASE_Z + 2.0);
    }

    gutters
        + centered_cube(
            "closed_excursion_evidence_board_front_evidence_drain",
            250.0,
            9.0,
            6.0,
        )
        .translate(BOARD_X / 2.0 - 185.0, -BOARD_Y / 2.0 + 48.0, BASE_Z + 3.0)
}

fn mounting_bosses() -> Part {
    let mut bosses = Part::empty("closed_excursion_evidence_board_mounting_bosses");
    for (i, (x, y)) in [
        (-505.0, 318.0),
        (505.0, 318.0),
        (-505.0, -318.0),
        (505.0, -318.0),
        (0.0, 318.0),
        (0.0, -318.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_excursion_evidence_board_mount_boss_{i}"),
            14.0,
            9.0,
            32,
        )
        .translate(x, y, BASE_Z + 4.5);
        let hole = centered_cylinder(
            format!("closed_excursion_evidence_board_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            11.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn environmental_logger_download_slots() -> Part {
    let body = centered_cube(
        "closed_excursion_evidence_logger_download_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );

    let mut cuts = Part::empty("closed_excursion_evidence_logger_slot_cuts");
    let mut retainers = Part::empty("closed_excursion_evidence_logger_retainer_lips");
    for i in 0..LOGGER_SLOTS {
        let x = centered_index(i, LOGGER_SLOTS, LOGGER_SLOT_PITCH);
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_evidence_logger_download_slot_{i}"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_PANEL_Z + 2.0,
            )
            .translate(x, -10.0, 0.0);
        retainers = retainers
            + centered_cube(
                format!("closed_excursion_evidence_logger_slot_top_retainer_{i}"),
                LOGGER_SLOT_X + 8.0,
                8.0,
                12.0,
            )
            .translate(x, LOGGER_SLOT_Y / 2.0 - 8.0, LOGGER_PANEL_Z / 2.0 + 6.0);
    }

    let mut docks = Part::empty("closed_excursion_evidence_download_cable_docks");
    for i in 0..DOWNLOAD_DOCKS {
        docks = docks
            + centered_cube(
                format!("closed_excursion_evidence_usb_download_dock_{i}"),
                44.0,
                16.0,
                12.0,
            )
            .translate(
                centered_index(i, DOWNLOAD_DOCKS, 72.0),
                -LOGGER_PANEL_Y / 2.0 + 20.0,
                6.0,
            );
    }

    body - cuts + retainers + docks
}

fn pressure_gas_trend_tokens() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_pressure_gas_trend_panel",
        TREND_PANEL_X,
        TREND_PANEL_Y,
        TREND_PANEL_Z,
    );

    let mut token_cuts = Part::empty("closed_excursion_evidence_trend_token_cuts");
    let mut lane_rails = Part::empty("closed_excursion_evidence_trend_lane_rails");
    for i in 0..PRESSURE_TOKEN_COUNT {
        token_cuts = token_cuts
            + centered_cylinder(
                format!("closed_excursion_evidence_pressure_token_socket_{i}"),
                TREND_TOKEN_D / 2.0,
                TREND_PANEL_Z + 2.0,
                32,
            )
            .translate(
                centered_index(i, PRESSURE_TOKEN_COUNT, TREND_TOKEN_PITCH),
                34.0,
                0.0,
            );
    }
    for i in 0..GAS_TOKEN_COUNT {
        token_cuts = token_cuts
            + centered_cylinder(
                format!("closed_excursion_evidence_gas_token_socket_{i}"),
                TREND_TOKEN_D / 2.0,
                TREND_PANEL_Z + 2.0,
                32,
            )
            .translate(
                centered_index(i, GAS_TOKEN_COUNT, TREND_TOKEN_PITCH),
                -34.0,
                0.0,
            );
    }
    for (i, y) in [62.0, 6.0, -62.0].into_iter().enumerate() {
        lane_rails = lane_rails
            + centered_cube(
                format!("closed_excursion_evidence_trend_lane_rail_{i}"),
                TREND_PANEL_X - 34.0,
                7.0,
                13.0,
            )
            .translate(0.0, y, TREND_PANEL_Z / 2.0 + 6.5);
    }

    panel - token_cuts + lane_rails
}

fn cassette_position_map_cards() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_cassette_map_card_panel",
        MAP_PANEL_X,
        MAP_PANEL_Y,
        MAP_PANEL_Z,
    );

    let mut card_cuts = Part::empty("closed_excursion_evidence_cassette_map_card_cuts");
    for i in 0..MAP_CARD_SLOTS {
        let col = i % 3;
        let row = i / 3;
        card_cuts = card_cuts
            + centered_cube(
                format!("closed_excursion_evidence_cassette_map_card_slot_{i}"),
                MAP_CARD_SLOT_X,
                MAP_CARD_SLOT_Y,
                MAP_PANEL_Z + 2.0,
            )
            .translate(
                centered_index(col, 3, MAP_CARD_PITCH_X),
                centered_index(row, 2, MAP_CARD_PITCH_Y),
                0.0,
            );
    }

    let mut position_grid = Part::empty("closed_excursion_evidence_cassette_position_grid");
    for row in 0..CASSETTE_POSITION_ROWS {
        for col in 0..CASSETTE_POSITION_COLS {
            let index = row * CASSETTE_POSITION_COLS + col;
            position_grid = position_grid
                + centered_cube(
                    format!("closed_excursion_evidence_cassette_position_land_{index}"),
                    20.0,
                    12.0,
                    4.0,
                )
                .translate(
                    -MAP_PANEL_X / 2.0 + 38.0 + col as f64 * 25.0,
                    -MAP_PANEL_Y / 2.0 + 18.0 + row as f64 * 18.0,
                    MAP_PANEL_Z / 2.0 + 2.0,
                );
        }
    }

    panel - card_cuts + position_grid
}

fn media_sample_archive_references() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_archive_reference_panel",
        ARCHIVE_PANEL_X,
        ARCHIVE_PANEL_Y,
        ARCHIVE_PANEL_Z,
    );

    let mut cuts = Part::empty("closed_excursion_evidence_archive_pocket_cuts");
    let mut flags = Part::empty("closed_excursion_evidence_archive_reference_flags");
    for i in 0..(MEDIA_ARCHIVE_POCKETS + SAMPLE_ARCHIVE_POCKETS) {
        let row = i / 4;
        let col = i % 4;
        let x = centered_index(col, 4, ARCHIVE_POCKET_PITCH_X);
        let y = centered_index(row, 2, ARCHIVE_POCKET_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_evidence_archive_reference_pocket_{i}"),
                ARCHIVE_POCKET_X,
                ARCHIVE_POCKET_Y,
                ARCHIVE_PANEL_Z + 2.0,
            )
            .translate(x, y, 0.0);
        flags = flags
            + centered_cube(
                format!("closed_excursion_evidence_archive_reference_flag_{i}"),
                ARCHIVE_POCKET_X - 12.0,
                7.0,
                12.0,
            )
            .translate(
                x,
                y + ARCHIVE_POCKET_Y / 2.0 + 8.0,
                ARCHIVE_PANEL_Z / 2.0 + 6.0,
            );
    }

    panel - cuts + flags
}

fn sensor_calibration_certificate_pockets() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_sensor_calibration_certificate_panel",
        CERT_PANEL_X,
        CERT_PANEL_Y,
        CERT_PANEL_Z,
    );

    let mut cuts = Part::empty("closed_excursion_evidence_calibration_certificate_cuts");
    for i in 0..CAL_CERT_POCKETS {
        let col = i % 3;
        let row = i / 3;
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_evidence_calibration_certificate_pocket_{i}"),
                CERT_SLOT_X,
                CERT_SLOT_Y,
                CERT_PANEL_Z + 2.0,
            )
            .translate(
                centered_index(col, 3, CERT_SLOT_PITCH_X),
                centered_index(row, 2, CERT_SLOT_PITCH_Y),
                0.0,
            );
    }

    let mut lands = Part::empty("closed_excursion_evidence_sensor_certificate_lands");
    for i in 0..SENSOR_CERT_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_excursion_evidence_sensor_certificate_status_land_{i}"),
                38.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(i, SENSOR_CERT_LANDS, 48.0),
                CERT_PANEL_Y / 2.0 - 18.0,
                CERT_PANEL_Z / 2.0 + 2.5,
            );
    }

    panel - cuts + lands
}

fn photo_evidence_bridge() -> Part {
    let mut bridge = Part::empty("closed_excursion_evidence_photo_bridge");
    for (i, x) in [
        -PHOTO_SPAN_X / 2.0 + PHOTO_POST_X / 2.0,
        PHOTO_SPAN_X / 2.0 - PHOTO_POST_X / 2.0,
    ]
    .into_iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("closed_excursion_evidence_photo_bridge_post_{i}"),
                PHOTO_POST_X,
                PHOTO_POST_Y,
                PHOTO_UNDERSIDE_Z,
            )
            .translate(x, 0.0, PHOTO_UNDERSIDE_Z / 2.0);
    }

    bridge = bridge
        + centered_cube(
            "closed_excursion_evidence_photo_bridge_beam",
            PHOTO_SPAN_X,
            PHOTO_POST_Y,
            PHOTO_BEAM_Z,
        )
        .translate(0.0, 0.0, PHOTO_UNDERSIDE_Z + PHOTO_BEAM_Z / 2.0);

    for i in 0..PHOTO_CAMERA_COUNT {
        bridge = bridge
            + centered_cube(
                format!("closed_excursion_evidence_camera_mount_{i}"),
                46.0,
                28.0,
                14.0,
            )
            .translate(
                centered_index(i, PHOTO_CAMERA_COUNT, PHOTO_SPAN_X / 4.0),
                -PHOTO_POST_Y / 2.0 - 12.0,
                PHOTO_UNDERSIDE_Z - 10.0,
            );
    }

    for i in 0..PHOTO_LIGHT_SEGMENTS {
        bridge = bridge
            + centered_cube(
                format!("closed_excursion_evidence_light_bar_segment_{i}"),
                72.0,
                10.0,
                8.0,
            )
            .translate(
                centered_index(i, PHOTO_LIGHT_SEGMENTS, 95.0),
                PHOTO_POST_Y / 2.0 + 6.0,
                PHOTO_UNDERSIDE_Z - 12.0,
            );
    }

    for i in 0..EVIDENCE_WINDOW_COUNT {
        bridge = bridge
            + centered_cube(
                format!("closed_excursion_evidence_photo_target_window_{i}"),
                86.0,
                5.0,
                6.0,
            )
            .translate(centered_index(i, EVIDENCE_WINDOW_COUNT, 132.0), 0.0, 12.0);
    }

    bridge
}

fn hold_release_reject_decision_lanes() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_hold_release_reject_panel",
        DECISION_PANEL_X,
        DECISION_PANEL_Y,
        DECISION_PANEL_Z,
    );

    let mut slot_cuts = Part::empty("closed_excursion_evidence_decision_slot_cuts");
    let mut lane_walls = Part::empty("closed_excursion_evidence_decision_lane_walls");
    for lane in 0..DECISION_LANES {
        let x = decision_lane_x(lane);
        for slot in 0..DECISION_SLOTS_PER_LANE {
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("closed_excursion_evidence_decision_lane_{lane}_slot_{slot}"),
                    DECISION_SLOT_X,
                    DECISION_SLOT_Y,
                    DECISION_PANEL_Z + 2.0,
                )
                .translate(
                    x,
                    centered_index(slot, DECISION_SLOTS_PER_LANE, DECISION_SLOT_PITCH_Y),
                    0.0,
                );
        }
        lane_walls = lane_walls
            + centered_cube(
                format!("closed_excursion_evidence_decision_lane_backstop_{lane}"),
                DECISION_SLOT_X + 18.0,
                8.0,
                22.0,
            )
            .translate(
                x,
                DECISION_PANEL_Y / 2.0 - 17.0,
                DECISION_PANEL_Z / 2.0 + 11.0,
            );
    }

    for lane in 0..DECISION_LANES - 1 {
        lane_walls = lane_walls
            + centered_cube(
                format!("closed_excursion_evidence_decision_lane_separator_{lane}"),
                8.0,
                DECISION_PANEL_Y - 32.0,
                34.0,
            )
            .translate(
                (decision_lane_x(lane) + decision_lane_x(lane + 1)) / 2.0,
                0.0,
                DECISION_PANEL_Z / 2.0 + 17.0,
            );
    }

    let reject_cage = centered_cube(
        "closed_excursion_evidence_reject_lane_high_custody_wall",
        DECISION_SLOT_X + 30.0,
        DECISION_PANEL_Y - 34.0,
        52.0,
    )
    .translate(decision_lane_x(2), 0.0, DECISION_PANEL_Z / 2.0 + 26.0);
    let reject_view_cut = centered_cube(
        "closed_excursion_evidence_reject_lane_visible_window",
        DECISION_SLOT_X + 12.0,
        DECISION_PANEL_Y - 72.0,
        46.0,
    )
    .translate(decision_lane_x(2), 0.0, DECISION_PANEL_Z / 2.0 + 26.0);

    panel - slot_cuts + lane_walls + (reject_cage - reject_view_cut)
}

fn barcode_rfid_lands() -> Part {
    let mut panel = centered_cube(
        "closed_excursion_evidence_traceability_land_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );

    for i in 0..BARCODE_LANDS {
        panel = panel
            + centered_cube(
                format!("closed_excursion_evidence_barcode_scan_land_{i}"),
                36.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 42.0),
                TRACE_PANEL_Y / 2.0 - 22.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }

    for i in 0..RFID_LANDS {
        panel = panel
            + centered_cube(
                format!("closed_excursion_evidence_rfid_antenna_land_{i}"),
                72.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(i, RFID_LANDS, 104.0),
                0.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }

    for i in 0..SEAL_ID_LANDS {
        panel = panel
            + centered_cube(
                format!("closed_excursion_evidence_tamper_seal_id_land_{i}"),
                50.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(i, SEAL_ID_LANDS, 68.0),
                -TRACE_PANEL_Y / 2.0 + 20.0,
                TRACE_PANEL_Z / 2.0 + 2.0,
            );
    }

    panel
}

fn clean_used_segregation() -> Part {
    let panel = centered_cube(
        "closed_excursion_evidence_clean_used_segregation_panel",
        SEG_PANEL_X,
        SEG_PANEL_Y,
        SEG_PANEL_Z,
    );

    let mut cuts = Part::empty("closed_excursion_evidence_clean_used_bay_cuts");
    for i in 0..CLEAN_EVIDENCE_BAYS {
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_evidence_clean_evidence_bay_{i}"),
                SEG_BAY_X,
                SEG_BAY_Y,
                SEG_PANEL_Z + 2.0,
            )
            .translate(
                centered_index(i, CLEAN_EVIDENCE_BAYS, SEG_BAY_PITCH),
                25.0,
                0.0,
            );
    }
    for i in 0..USED_EVIDENCE_BAYS {
        cuts = cuts
            + centered_cube(
                format!("closed_excursion_evidence_used_evidence_bay_{i}"),
                SEG_BAY_X,
                SEG_BAY_Y,
                SEG_PANEL_Z + 2.0,
            )
            .translate(
                centered_index(i, USED_EVIDENCE_BAYS, SEG_BAY_PITCH),
                -25.0,
                0.0,
            );
    }

    let divider = centered_cube(
        "closed_excursion_evidence_clean_used_high_divider",
        SEG_PANEL_X - 24.0,
        8.0,
        SEG_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEG_PANEL_Z / 2.0 + SEG_DIVIDER_Z / 2.0);

    panel - cuts + divider
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_excursion_evidence_front_robot_keepout_gauge",
        BOARD_X - 120.0,
        10.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -BOARD_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_excursion_evidence_rear_service_keepout_gauge",
        BOARD_X - 120.0,
        10.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        BOARD_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left = centered_cube(
        "closed_excursion_evidence_left_data_service_keepout_gauge",
        10.0,
        BOARD_Y - 140.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -BOARD_X / 2.0 + LEFT_DATA_SERVICE_KEEP_OUT_X,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "closed_excursion_evidence_right_qa_review_keepout_gauge",
        10.0,
        BOARD_Y - 140.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        BOARD_X / 2.0 - RIGHT_QA_REVIEW_KEEP_OUT_X,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top = centered_cube(
        "closed_excursion_evidence_photo_bridge_top_clearance_gauge",
        210.0,
        16.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_PHOTO_BRIDGE_KEEP_OUT_Z);

    front + rear + left + right + top
}

fn decision_lane_x(lane: usize) -> f64 {
    centered_index(lane, DECISION_LANES, DECISION_LANE_PITCH_X)
}

fn decision_lane_gap() -> f64 {
    DECISION_LANE_PITCH_X - DECISION_SLOT_X
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.1 && a.1 > b.0 && a.2 < b.3 && a.3 > b.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_count_matches_export_plan() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn feature_inventory_covers_requested_evidence_interfaces() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"environmental_logger_download_slots"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_gas_trend_tokens"));
        assert!(REQUIRED_FEATURES.contains(&"cassette_position_map_cards"));
        assert!(REQUIRED_FEATURES.contains(&"media_sample_archive_references"));
        assert!(REQUIRED_FEATURES.contains(&"sensor_calibration_certificate_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"photo_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"hold_release_reject_decision_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"assembly_export"));
    }

    #[test]
    fn geometry_constants_preserve_evidence_board_capacity() {
        assert_eq!(LOGGER_SLOTS, 5);
        assert_eq!(PRESSURE_TOKEN_COUNT + GAS_TOKEN_COUNT, 16);
        assert_eq!(CASSETTE_POSITION_COUNT, 20);
        assert_eq!(MEDIA_ARCHIVE_POCKETS + SAMPLE_ARCHIVE_POCKETS, 8);
        assert_eq!(CAL_CERT_POCKETS, 6);
        assert_eq!(DECISION_TOTAL_SLOTS, 12);
        assert_eq!(BARCODE_LANDS + RFID_LANDS + SEAL_ID_LANDS, 20);
        assert_eq!(CLEAN_EVIDENCE_BAYS, USED_EVIDENCE_BAYS);
    }

    #[test]
    fn all_modules_fit_with_visible_lane_separation() {
        assert_layout();
        assert!(decision_lane_gap() >= DECISION_LANE_GAP_MIN);
    }
}
