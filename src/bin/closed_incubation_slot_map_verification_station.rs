use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed rack slot-map verification station.
//
// This generator models the mechanical interface for a no-cell rack map process:
// cassette surrogate dock, six-slot rack reference comb, barcode/position scan
// lands, logger pockets, token map, condensate witness lands, transfer datum,
// decision lanes, segregation features, robot/service keepouts, and assembly.

const OUTPUTS: [&str; 12] = [
    "output/closed_incubation_slot_map_verification_station_deck.stl",
    "output/closed_incubation_slot_map_verification_station_cassette_surrogate_dock.stl",
    "output/closed_incubation_slot_map_verification_station_six_slot_rack_comb.stl",
    "output/closed_incubation_slot_map_verification_station_scan_lands.stl",
    "output/closed_incubation_slot_map_verification_station_logger_pockets.stl",
    "output/closed_incubation_slot_map_verification_station_token_map.stl",
    "output/closed_incubation_slot_map_verification_station_condensate_witness_lands.stl",
    "output/closed_incubation_slot_map_verification_station_transfer_tray_datum.stl",
    "output/closed_incubation_slot_map_verification_station_release_hold_reject_lanes.stl",
    "output/closed_incubation_slot_map_verification_station_clean_used_segregation.stl",
    "output/closed_incubation_slot_map_verification_station_robot_service_keepouts.stl",
    "output/closed_incubation_slot_map_verification_station_assembly.stl",
];

const RACK_SLOTS: usize = 6;
const SLOT_TOKEN_COUNT: usize = RACK_SLOTS;
const EDGE_TOKEN_COUNT: usize = 4;
const CENTER_TOKEN_COUNT: usize = RACK_SLOTS - EDGE_TOKEN_COUNT;

const DECK_X: f64 = 980.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;

const CASSETTE_SURROGATE_X: f64 = REVC_CHIP_LENGTH + 74.0;
const CASSETTE_SURROGATE_Y: f64 = REVC_CHIP_WIDTH + 64.0;
const CASSETTE_SURROGATE_Z: f64 = 34.0;
const CASSETTE_DOCK_X: f64 = CASSETTE_SURROGATE_X + 58.0;
const CASSETTE_DOCK_Y: f64 = CASSETTE_SURROGATE_Y + 58.0;
const CASSETTE_DOCK_Z: f64 = 32.0;
const CASSETTE_DOCK_CENTER_X: f64 = -280.0;
const CASSETTE_DOCK_CENTER_Y: f64 = 118.0;
const CASSETTE_CLEARANCE: f64 = 1.0;
const DOCK_RAIL_W: f64 = 14.0;
const DOCK_RAIL_Z: f64 = 24.0;
const DOCK_DATUM_PIN_D: f64 = 7.0;

const RACK_SLOT_X: f64 = 66.0;
const RACK_SLOT_Y: f64 = 132.0;
const RACK_SLOT_Z: f64 = 24.0;
const RACK_SLOT_PITCH_X: f64 = 82.0;
const RACK_COMB_X: f64 = RACK_SLOTS as f64 * RACK_SLOT_PITCH_X + 52.0;
const RACK_COMB_Y: f64 = 190.0;
const RACK_COMB_Z: f64 = 30.0;
const RACK_COMB_CENTER_X: f64 = 42.0;
const RACK_COMB_CENTER_Y: f64 = 142.0;

const SCAN_LAND_X: f64 = 86.0;
const SCAN_LAND_Y: f64 = 34.0;
const SCAN_LAND_Z: f64 = 3.0;
const POSITION_SCAN_D: f64 = 18.0;
const BARCODE_LAND_X: f64 = 108.0;
const BARCODE_LAND_Y: f64 = 28.0;
const SCAN_STRIP_CENTER_Y: f64 = -92.0;

const LOGGER_POCKET_COUNT: usize = 4;
const LOGGER_POCKET_X: f64 = 92.0;
const LOGGER_POCKET_Y: f64 = 54.0;
const LOGGER_POCKET_Z: f64 = 18.0;
const LOGGER_RECESS_DEPTH: f64 = 7.0;
const LOGGER_POCKET_CENTER_X: f64 = -326.0;
const LOGGER_POCKET_CENTER_Y: f64 = -222.0;
const LOGGER_POCKET_PITCH_X: f64 = 108.0;

const TOKEN_MAP_X: f64 = 238.0;
const TOKEN_MAP_Y: f64 = 164.0;
const TOKEN_MAP_Z: f64 = 12.0;
const TOKEN_MAP_CENTER_X: f64 = 322.0;
const TOKEN_MAP_CENTER_Y: f64 = 168.0;
const TOKEN_D: f64 = 21.0;
const TOKEN_EDGE_RIM_D: f64 = 30.0;
const TOKEN_CENTER_RIM_D: f64 = 24.0;

const WITNESS_LAND_COUNT: usize = RACK_SLOTS;
const WITNESS_LAND_X: f64 = 58.0;
const WITNESS_LAND_Y: f64 = 36.0;
const WITNESS_LAND_Z: f64 = 4.0;
const WITNESS_GUTTER_W: f64 = 5.0;
const WITNESS_CENTER_Y: f64 = -178.0;

const TRANSFER_TRAY_X: f64 = 338.0;
const TRANSFER_TRAY_Y: f64 = 154.0;
const TRANSFER_TRAY_Z: f64 = 26.0;
const TRANSFER_TRAY_CENTER_X: f64 = -44.0;
const TRANSFER_TRAY_CENTER_Y: f64 = -274.0;
const TRANSFER_DATUM_BALL_D: f64 = 10.0;
const TRANSFER_FORK_SLOT_X: f64 = 42.0;
const TRANSFER_FORK_SLOT_Y: f64 = 126.0;

const DECISION_LANE_COUNT: usize = 3;
const DECISION_LANE_X: f64 = 146.0;
const DECISION_LANE_Y: f64 = 84.0;
const DECISION_LANE_Z: f64 = 20.0;
const DECISION_LANE_CENTER_X: f64 = 300.0;
const DECISION_LANE_CENTER_Y: f64 = -266.0;
const DECISION_LANE_PITCH_X: f64 = 164.0;
const LANE_WALL_W: f64 = 8.0;

const CLEAN_ZONE_X: f64 = 318.0;
const CLEAN_ZONE_Y: f64 = 118.0;
const USED_ZONE_X: f64 = 318.0;
const USED_ZONE_Y: f64 = 118.0;
const SEGREGATION_RIB_W: f64 = 20.0;
const SEGREGATION_RIB_Z: f64 = 32.0;
const CLEAN_USED_GAP: f64 = 76.0;

const ROBOT_KEEP_OUT_X: f64 = 760.0;
const ROBOT_KEEP_OUT_Y: f64 = 118.0;
const ROBOT_KEEP_OUT_Z: f64 = 16.0;
const SERVICE_KEEP_OUT_X: f64 = 122.0;
const SERVICE_KEEP_OUT_Y: f64 = 520.0;
const SERVICE_KEEP_OUT_Z: f64 = 42.0;
const SCANNER_CLEARANCE_Z: f64 = 122.0;

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let dock = cassette_surrogate_dock();
    export(OUTPUTS[1], &dock);

    let comb = six_slot_rack_reference_comb();
    export(OUTPUTS[2], &comb);

    let scans = barcode_and_position_scan_lands();
    export(OUTPUTS[3], &scans);

    let loggers = environmental_logger_pockets();
    export(OUTPUTS[4], &loggers);

    let tokens = edge_center_token_map();
    export(OUTPUTS[5], &tokens);

    let witness = condensate_witness_lands();
    export(OUTPUTS[6], &witness);

    let transfer = transfer_tray_datum();
    export(OUTPUTS[7], &transfer);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_and_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        deck + dock.translate(
            CASSETTE_DOCK_CENTER_X,
            CASSETTE_DOCK_CENTER_Y,
            DECK_Z / 2.0 + CASSETTE_DOCK_Z / 2.0,
        ) + comb.translate(
            RACK_COMB_CENTER_X,
            RACK_COMB_CENTER_Y,
            DECK_Z / 2.0 + RACK_COMB_Z / 2.0,
        ) + scans.translate(0.0, SCAN_STRIP_CENTER_Y, DECK_Z / 2.0 + SCAN_LAND_Z / 2.0)
            + loggers.translate(0.0, 0.0, DECK_Z / 2.0 + LOGGER_POCKET_Z / 2.0)
            + tokens.translate(
                TOKEN_MAP_CENTER_X,
                TOKEN_MAP_CENTER_Y,
                DECK_Z / 2.0 + TOKEN_MAP_Z / 2.0,
            )
            + witness.translate(0.0, WITNESS_CENTER_Y, DECK_Z / 2.0 + WITNESS_LAND_Z / 2.0)
            + transfer.translate(
                TRANSFER_TRAY_CENTER_X,
                TRANSFER_TRAY_CENTER_Y,
                DECK_Z / 2.0 + TRANSFER_TRAY_Z / 2.0,
            )
            + lanes.translate(
                DECISION_LANE_CENTER_X,
                DECISION_LANE_CENTER_Y,
                DECK_Z / 2.0 + DECISION_LANE_Z / 2.0,
            )
            + segregation.translate(0.0, 0.0, DECK_Z / 2.0 + SEGREGATION_RIB_Z / 2.0)
            + keepouts.translate(0.0, 0.0, DECK_Z + ROBOT_KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubation slot-map verification station:");
    println!(
        "  No-cell process: cassette surrogate dock plus {RACK_SLOTS}-slot rack reference comb for rack/slot identity verification"
    );
    println!(
        "  Scan interface: {RACK_SLOTS} barcode lands, {RACK_SLOTS} position targets, {LOGGER_POCKET_COUNT} environmental logger pockets"
    );
    println!(
        "  Token map: {EDGE_TOKEN_COUNT} edge tokens and {CENTER_TOKEN_COUNT} center tokens; witness lands: {WITNESS_LAND_COUNT}"
    );
    println!(
        "  Decision flow: released/hold/reject lanes with {:.0}mm clean-used segregation gap and {:.0}mm scanner clearance",
        clean_used_gap(),
        scanner_clearance()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(RACK_SLOTS, 6, "rack reference comb must have six slots");
    assert_eq!(
        SLOT_TOKEN_COUNT,
        EDGE_TOKEN_COUNT + CENTER_TOKEN_COUNT,
        "edge/center token map must cover every rack slot"
    );
    assert_eq!(
        OUTPUTS.len(),
        12,
        "closed slot-map station export count changed"
    );
    assert!(
        CASSETTE_SURROGATE_Z > REVC_TOTAL_HEIGHT,
        "cassette surrogate dock must clear the existing cassette article height"
    );
    assert!(
        RACK_COMB_X < DECK_X - 120.0,
        "six-slot rack comb exceeds deck width allowance"
    );
    assert!(
        clean_used_gap() >= CLEAN_USED_GAP,
        "clean/used segregation gap is below target"
    );
    assert!(
        !rects_overlap(clean_rect(), used_rect()),
        "clean and used zones overlap"
    );
    assert!(
        scanner_clearance() >= 90.0,
        "scanner and robot clearance is too low"
    );
}

fn station_deck() -> Part {
    let deck = centered_cube("closed_slot_map_station_deck", DECK_X, DECK_Y, DECK_Z);

    let dock_recess = centered_cube(
        "closed_slot_map_cassette_dock_recess",
        CASSETTE_DOCK_X + 22.0,
        CASSETTE_DOCK_Y + 22.0,
        5.0,
    )
    .translate(
        CASSETTE_DOCK_CENTER_X,
        CASSETTE_DOCK_CENTER_Y,
        DECK_Z / 2.0 - 2.0,
    );

    let comb_recess = centered_cube(
        "closed_slot_map_rack_comb_recess",
        RACK_COMB_X + 18.0,
        RACK_COMB_Y + 18.0,
        5.0,
    )
    .translate(RACK_COMB_CENTER_X, RACK_COMB_CENTER_Y, DECK_Z / 2.0 - 2.0);

    let transfer_recess = centered_cube(
        "closed_slot_map_transfer_tray_recess",
        TRANSFER_TRAY_X + 20.0,
        TRANSFER_TRAY_Y + 20.0,
        5.0,
    )
    .translate(
        TRANSFER_TRAY_CENTER_X,
        TRANSFER_TRAY_CENTER_Y,
        DECK_Z / 2.0 - 2.0,
    );

    deck - dock_recess - comb_recess - transfer_recess - mounting_holes()
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("closed_slot_map_station_mounting_holes");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_slot_map_station_m6_mount_{i}"),
                3.3,
                DECK_Z + 2.0,
                28,
            )
            .translate(sx * (DECK_X / 2.0 - 36.0), sy * (DECK_Y / 2.0 - 36.0), 0.0);
    }
    holes
}

fn cassette_surrogate_dock() -> Part {
    let base = centered_cube(
        "closed_slot_map_cassette_surrogate_dock_base",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let surrogate_relief = centered_cube(
        "closed_slot_map_cassette_surrogate_relief",
        CASSETTE_SURROGATE_X + 2.0 * CASSETTE_CLEARANCE,
        CASSETTE_SURROGATE_Y + 2.0 * CASSETTE_CLEARANCE,
        12.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 - 4.8);
    let barcode_window = centered_cube(
        "closed_slot_map_cassette_surrogate_barcode_window",
        96.0,
        24.0,
        13.0,
    )
    .translate(
        0.0,
        -(CASSETTE_DOCK_Y / 2.0 - 28.0),
        CASSETTE_DOCK_Z / 2.0 - 5.0,
    );

    base - surrogate_relief - barcode_window + dock_rails() + dock_datum_pins()
}

fn dock_rails() -> Part {
    let left = centered_cube(
        "closed_slot_map_cassette_dock_left_datum_rail",
        DOCK_RAIL_W,
        CASSETTE_DOCK_Y,
        DOCK_RAIL_Z,
    )
    .translate(
        -(CASSETTE_DOCK_X / 2.0 - DOCK_RAIL_W / 2.0),
        0.0,
        CASSETTE_DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
    );
    let back = centered_cube(
        "closed_slot_map_cassette_dock_back_datum_rail",
        CASSETTE_DOCK_X,
        DOCK_RAIL_W,
        DOCK_RAIL_Z,
    )
    .translate(
        0.0,
        CASSETTE_DOCK_Y / 2.0 - DOCK_RAIL_W / 2.0,
        CASSETTE_DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
    );
    let right_soft = centered_cube(
        "closed_slot_map_cassette_dock_right_soft_capture_rail",
        DOCK_RAIL_W,
        CASSETTE_DOCK_Y * 0.72,
        DOCK_RAIL_Z * 0.65,
    )
    .translate(
        CASSETTE_DOCK_X / 2.0 - DOCK_RAIL_W / 2.0,
        -8.0,
        CASSETTE_DOCK_Z / 2.0 + DOCK_RAIL_Z * 0.325,
    );

    left + back + right_soft
}

fn dock_datum_pins() -> Part {
    let mut pins = Part::empty("closed_slot_map_cassette_dock_datum_pins");
    for (i, (x, y)) in dock_pin_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_slot_map_cassette_dock_datum_boss_{i}"),
            DOCK_DATUM_PIN_D,
            9.0,
            32,
        )
        .translate(*x, *y, CASSETTE_DOCK_Z / 2.0 + 4.5);
        let pilot = centered_cylinder(
            format!("closed_slot_map_cassette_dock_datum_pilot_{i}"),
            DOCK_DATUM_PIN_D * 0.42,
            13.0,
            28,
        )
        .translate(*x, *y, CASSETTE_DOCK_Z / 2.0 + 6.5);
        pins = pins + boss + pilot;
    }
    pins
}

fn six_slot_rack_reference_comb() -> Part {
    let base = centered_cube(
        "closed_slot_map_six_slot_rack_reference_comb_base",
        RACK_COMB_X,
        RACK_COMB_Y,
        RACK_COMB_Z,
    );
    let mut slots = Part::empty("closed_slot_map_rack_slot_reliefs");
    let mut slot_rims = Part::empty("closed_slot_map_rack_slot_reference_rims");

    for slot in 0..RACK_SLOTS {
        let x = slot_center_x(slot);
        let relief = centered_cube(
            format!("closed_slot_map_rack_slot_{slot}_surrogate_relief"),
            RACK_SLOT_X,
            RACK_SLOT_Y,
            RACK_SLOT_Z + 2.0,
        )
        .translate(x, 0.0, RACK_COMB_Z / 2.0 - RACK_SLOT_Z / 2.0 + 1.0);
        let front_key = centered_cube(
            format!("closed_slot_map_rack_slot_{slot}_front_position_key"),
            RACK_SLOT_X * 0.42,
            8.0,
            8.0,
        )
        .translate(x, -(RACK_SLOT_Y / 2.0 + 12.0), RACK_COMB_Z / 2.0 + 4.0);
        let rear_key = centered_cube(
            format!("closed_slot_map_rack_slot_{slot}_rear_position_key"),
            RACK_SLOT_X * 0.28,
            8.0,
            8.0,
        )
        .translate(x, RACK_SLOT_Y / 2.0 + 12.0, RACK_COMB_Z / 2.0 + 4.0);
        slots = slots + relief;
        slot_rims = slot_rims + front_key + rear_key;
    }

    base - slots + slot_rims
}

fn barcode_and_position_scan_lands() -> Part {
    let mut lands = Part::empty("closed_slot_map_barcode_position_scan_lands");
    for slot in 0..RACK_SLOTS {
        let x = slot_center_x(slot) + RACK_COMB_CENTER_X;
        let barcode = centered_cube(
            format!("closed_slot_map_slot_{slot}_barcode_scan_land"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            SCAN_LAND_Z,
        )
        .translate(x, 0.0, 0.0);
        let position = centered_cylinder(
            format!("closed_slot_map_slot_{slot}_position_scan_target"),
            POSITION_SCAN_D / 2.0,
            SCAN_LAND_Z + 1.0,
            36,
        )
        .translate(x, 42.0, 0.5);
        let human_label_land = centered_cube(
            format!("closed_slot_map_slot_{slot}_human_readable_position_land"),
            SCAN_LAND_X * 0.62,
            SCAN_LAND_Y * 0.58,
            SCAN_LAND_Z,
        )
        .translate(x, -39.0, 0.0);
        lands = lands + barcode + position + human_label_land;
    }
    lands
}

fn environmental_logger_pockets() -> Part {
    let mut pockets = Part::empty("closed_slot_map_environmental_logger_pockets");
    for i in 0..LOGGER_POCKET_COUNT {
        let x = LOGGER_POCKET_CENTER_X + i as f64 * LOGGER_POCKET_PITCH_X;
        let y = LOGGER_POCKET_CENTER_Y;
        let pocket = centered_cube(
            format!("closed_slot_map_environmental_logger_pocket_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_Z,
        );
        let recess = centered_cube(
            format!("closed_slot_map_environmental_logger_recess_{i}"),
            LOGGER_POCKET_X - 18.0,
            LOGGER_POCKET_Y - 16.0,
            LOGGER_RECESS_DEPTH + 1.0,
        )
        .translate(
            0.0,
            0.0,
            LOGGER_POCKET_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0 + 0.6,
        );
        let cable_notch = centered_cube(
            format!("closed_slot_map_environmental_logger_cable_notch_{i}"),
            14.0,
            LOGGER_POCKET_Y + 2.0,
            LOGGER_RECESS_DEPTH + 2.0,
        )
        .translate(
            LOGGER_POCKET_X / 2.0 - 16.0,
            0.0,
            LOGGER_POCKET_Z / 2.0 - 2.0,
        );
        pockets = pockets + (pocket - recess - cable_notch).translate(x, y, 0.0);
    }
    pockets
}

fn edge_center_token_map() -> Part {
    let plate = centered_cube(
        "closed_slot_map_edge_center_token_map_plate",
        TOKEN_MAP_X,
        TOKEN_MAP_Y,
        TOKEN_MAP_Z,
    );
    let mut tokens = Part::empty("closed_slot_map_edge_center_tokens");
    for slot in 0..RACK_SLOTS {
        let (x, y) = token_center(slot);
        let edge = is_edge_slot(slot);
        let rim_d = if edge {
            TOKEN_EDGE_RIM_D
        } else {
            TOKEN_CENTER_RIM_D
        };
        let rim = centered_cylinder(
            format!(
                "closed_slot_map_slot_{slot}_{}_token_rim",
                token_label(edge)
            ),
            rim_d / 2.0,
            4.0,
            36,
        )
        .translate(x, y, TOKEN_MAP_Z / 2.0 + 2.0);
        let token = centered_cylinder(
            format!("closed_slot_map_slot_{slot}_{}_token", token_label(edge)),
            TOKEN_D / 2.0,
            7.0,
            36,
        )
        .translate(x, y, TOKEN_MAP_Z / 2.0 + 3.5);
        tokens = tokens + rim + token;
    }
    plate + tokens
}

fn condensate_witness_lands() -> Part {
    let mut lands = Part::empty("closed_slot_map_condensate_witness_lands");
    for slot in 0..WITNESS_LAND_COUNT {
        let x = slot_center_x(slot) + RACK_COMB_CENTER_X;
        let pad = centered_cube(
            format!("closed_slot_map_slot_{slot}_condensate_witness_land"),
            WITNESS_LAND_X,
            WITNESS_LAND_Y,
            WITNESS_LAND_Z,
        );
        let gutter = centered_cube(
            format!("closed_slot_map_slot_{slot}_condensate_witness_gutter"),
            WITNESS_LAND_X + 10.0,
            WITNESS_GUTTER_W,
            WITNESS_LAND_Z + 1.0,
        )
        .translate(0.0, -(WITNESS_LAND_Y / 2.0 - 6.0), 0.0);
        lands = lands + (pad - gutter).translate(x, 0.0, 0.0);
    }
    lands
}

fn transfer_tray_datum() -> Part {
    let tray = centered_cube(
        "closed_slot_map_transfer_tray_datum_base",
        TRANSFER_TRAY_X,
        TRANSFER_TRAY_Y,
        TRANSFER_TRAY_Z,
    );
    let left_fork = centered_cube(
        "closed_slot_map_transfer_tray_left_fork_clearance",
        TRANSFER_FORK_SLOT_X,
        TRANSFER_FORK_SLOT_Y,
        TRANSFER_TRAY_Z + 2.0,
    )
    .translate(-72.0, 0.0, 0.0);
    let right_fork = centered_cube(
        "closed_slot_map_transfer_tray_right_fork_clearance",
        TRANSFER_FORK_SLOT_X,
        TRANSFER_FORK_SLOT_Y,
        TRANSFER_TRAY_Z + 2.0,
    )
    .translate(72.0, 0.0, 0.0);
    let stop = centered_cube(
        "closed_slot_map_transfer_tray_hard_stop",
        TRANSFER_TRAY_X,
        12.0,
        18.0,
    )
    .translate(
        0.0,
        TRANSFER_TRAY_Y / 2.0 - 6.0,
        TRANSFER_TRAY_Z / 2.0 + 9.0,
    );

    tray - left_fork - right_fork + stop + transfer_datum_balls()
}

fn transfer_datum_balls() -> Part {
    let mut datums = Part::empty("closed_slot_map_transfer_tray_datum_balls");
    for (i, (x, y)) in [(-126.0, 42.0), (126.0, 42.0), (-126.0, -42.0)]
        .iter()
        .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_slot_map_transfer_tray_datum_ball_surrogate_{i}"),
                TRANSFER_DATUM_BALL_D / 2.0,
                8.0,
                32,
            )
            .translate(*x, *y, TRANSFER_TRAY_Z / 2.0 + 4.0);
    }
    datums
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("closed_slot_map_release_hold_reject_lanes");
    for lane in 0..DECISION_LANE_COUNT {
        let x = decision_lane_x(lane);
        let base = centered_cube(
            format!("closed_slot_map_{}_lane_base", lane_label(lane)),
            DECISION_LANE_X,
            DECISION_LANE_Y,
            DECISION_LANE_Z,
        );
        let pocket = centered_cube(
            format!("closed_slot_map_{}_lane_recess", lane_label(lane)),
            DECISION_LANE_X - 22.0,
            DECISION_LANE_Y - 22.0,
            8.0,
        )
        .translate(0.0, 0.0, DECISION_LANE_Z / 2.0 - 3.2);
        let front_wall = centered_cube(
            format!("closed_slot_map_{}_lane_front_wall", lane_label(lane)),
            DECISION_LANE_X,
            LANE_WALL_W,
            18.0,
        )
        .translate(
            0.0,
            -(DECISION_LANE_Y / 2.0 - LANE_WALL_W / 2.0),
            DECISION_LANE_Z / 2.0,
        );
        lanes = lanes + (base - pocket + front_wall).translate(x, 0.0, 0.0);
    }
    lanes
}

fn clean_used_segregation() -> Part {
    let clean_zone = centered_cube(
        "closed_slot_map_clean_inbound_zone_land",
        CLEAN_ZONE_X,
        CLEAN_ZONE_Y,
        5.0,
    )
    .translate(
        -CLEAN_ZONE_X / 2.0 - CLEAN_USED_GAP / 2.0,
        0.0,
        -SEGREGATION_RIB_Z / 2.0,
    );
    let used_zone = centered_cube(
        "closed_slot_map_used_outbound_zone_land",
        USED_ZONE_X,
        USED_ZONE_Y,
        5.0,
    )
    .translate(
        USED_ZONE_X / 2.0 + CLEAN_USED_GAP / 2.0,
        0.0,
        -SEGREGATION_RIB_Z / 2.0,
    );
    let rib = centered_cube(
        "closed_slot_map_clean_used_segregation_rib",
        SEGREGATION_RIB_W,
        CLEAN_ZONE_Y + 42.0,
        SEGREGATION_RIB_Z,
    );
    let wipe_channel = centered_cube(
        "closed_slot_map_clean_used_wipe_channel",
        SEGREGATION_RIB_W + 8.0,
        18.0,
        SEGREGATION_RIB_Z + 2.0,
    )
    .translate(0.0, 0.0, 0.0);

    clean_zone + used_zone + (rib - wipe_channel)
}

fn robot_and_service_keepouts() -> Part {
    let robot = centered_cube(
        "closed_slot_map_robot_handoff_keepout_gauge",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - ROBOT_KEEP_OUT_Y / 2.0 - 24.0, 0.0);
    let service_left = centered_cube(
        "closed_slot_map_left_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        -(DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 24.0),
        0.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "closed_slot_map_right_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 24.0,
        0.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let scanner_bridge = centered_cube(
        "closed_slot_map_scan_bridge_clearance_gauge",
        RACK_COMB_X + 72.0,
        42.0,
        10.0,
    )
    .translate(
        RACK_COMB_CENTER_X,
        SCAN_STRIP_CENTER_Y,
        SCANNER_CLEARANCE_Z - ROBOT_KEEP_OUT_Z / 2.0,
    );

    robot + service_left + service_right + scanner_bridge
}

fn slot_center_x(slot: usize) -> f64 {
    (slot as f64 - (RACK_SLOTS as f64 - 1.0) / 2.0) * RACK_SLOT_PITCH_X
}

fn dock_pin_points() -> [(f64, f64); 3] {
    [
        (
            -(CASSETTE_SURROGATE_X / 2.0 + 14.0),
            CASSETTE_SURROGATE_Y / 2.0 + 14.0,
        ),
        (
            CASSETTE_SURROGATE_X / 2.0 + 14.0,
            CASSETTE_SURROGATE_Y / 2.0 + 14.0,
        ),
        (
            -(CASSETTE_SURROGATE_X / 2.0 + 14.0),
            -(CASSETTE_SURROGATE_Y / 2.0 + 14.0),
        ),
    ]
}

fn token_center(slot: usize) -> (f64, f64) {
    let col = slot % 3;
    let row = slot / 3;
    ((col as f64 - 1.0) * 72.0, (row as f64 - 0.5) * 62.0)
}

fn is_edge_slot(slot: usize) -> bool {
    slot == 0 || slot == 2 || slot == 3 || slot == 5
}

fn token_label(edge: bool) -> &'static str {
    if edge {
        "edge"
    } else {
        "center"
    }
}

fn decision_lane_x(lane: usize) -> f64 {
    (lane as f64 - 1.0) * DECISION_LANE_PITCH_X
}

fn lane_label(lane: usize) -> &'static str {
    match lane {
        0 => "released",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn clean_rect() -> Rect {
    Rect {
        x: -CLEAN_ZONE_X / 2.0 - CLEAN_USED_GAP / 2.0,
        y: 0.0,
        w: CLEAN_ZONE_X,
        h: CLEAN_ZONE_Y,
    }
}

fn used_rect() -> Rect {
    Rect {
        x: USED_ZONE_X / 2.0 + CLEAN_USED_GAP / 2.0,
        y: 0.0,
        w: USED_ZONE_X,
        h: USED_ZONE_Y,
    }
}

fn clean_used_gap() -> f64 {
    horizontal_gap(clean_rect(), used_rect())
}

fn scanner_clearance() -> f64 {
    SCANNER_CLEARANCE_Z - REVC_TOTAL_HEIGHT
}

fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rack_geometry_constants_match_closed_slot_map_process() {
        assert_eq!(RACK_SLOTS, 6);
        assert_eq!(WITNESS_LAND_COUNT, RACK_SLOTS);
        assert_eq!(SLOT_TOKEN_COUNT, RACK_SLOTS);
        assert_eq!(EDGE_TOKEN_COUNT, 4);
        assert_eq!(CENTER_TOKEN_COUNT, 2);
        assert!(RACK_SLOT_PITCH_X > RACK_SLOT_X);
        assert!(CASSETTE_DOCK_X > CASSETTE_SURROGATE_X + 2.0 * CASSETTE_CLEARANCE);
    }

    #[test]
    fn token_map_covers_edge_and_center_slots() {
        let mut edge = 0;
        let mut center = 0;
        for slot in 0..RACK_SLOTS {
            let (x, y) = token_center(slot);
            assert!(x.abs() < TOKEN_MAP_X / 2.0 - TOKEN_EDGE_RIM_D / 2.0);
            assert!(y.abs() < TOKEN_MAP_Y / 2.0 - TOKEN_EDGE_RIM_D / 2.0);
            if is_edge_slot(slot) {
                edge += 1;
            } else {
                center += 1;
            }
        }
        assert_eq!(edge, EDGE_TOKEN_COUNT);
        assert_eq!(center, CENTER_TOKEN_COUNT);
    }

    #[test]
    fn clean_used_and_service_spaces_are_segregated() {
        assert!(clean_used_gap() >= CLEAN_USED_GAP);
        assert!(!rects_overlap(clean_rect(), used_rect()));
        assert!(SEGREGATION_RIB_Z > DECK_Z);
        assert!(SERVICE_KEEP_OUT_X + SERVICE_KEEP_OUT_Y > ROBOT_KEEP_OUT_Y);
        assert!(scanner_clearance() >= 90.0);
    }

    #[test]
    fn output_manifest_exports_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS[0].ends_with("_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }
}
