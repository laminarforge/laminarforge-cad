use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette position environmental equivalence station.
//
// This is workflow geometry for comparing edge, center, and rack-slot effects
// across sealed 20-chip cassette surrogates and incubator rack positions. It
// provides a 4x5 cassette nest, removable sensor puck sockets, edge/center
// reference coupons, gas/RH/temp logger pockets, barcode tokens, disposition
// lanes, camera evidence bridge, robot/service keepouts, and clean/used
// segregation. Analytical equivalence criteria stay in the study protocol.

// Stable STL outputs for parent integration:
const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_position_environmental_equivalence_station_deck.stl",
    "output/closed_cassette_position_environmental_equivalence_station_cassette_surrogate_nest.stl",
    "output/closed_cassette_position_environmental_equivalence_station_incubator_rack_slot_reference_comb.stl",
    "output/closed_cassette_position_environmental_equivalence_station_removable_sensor_puck_positions.stl",
    "output/closed_cassette_position_environmental_equivalence_station_edge_center_reference_coupons.stl",
    "output/closed_cassette_position_environmental_equivalence_station_gas_rh_temp_logger_pockets.stl",
    "output/closed_cassette_position_environmental_equivalence_station_barcode_position_tokens.stl",
    "output/closed_cassette_position_environmental_equivalence_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_position_environmental_equivalence_station_evidence_camera_bridge.stl",
    "output/closed_cassette_position_environmental_equivalence_station_robot_service_keepouts.stl",
    "output/closed_cassette_position_environmental_equivalence_station_clean_used_segregation.stl",
    "output/closed_cassette_position_environmental_equivalence_station_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = CASSETTE_POSITION_COUNT - EDGE_POSITION_COUNT;
const RACK_SLOT_COUNT: usize = 6;
const LOGGER_POCKET_COUNT: usize = 4;
const DISPOSITION_LANE_COUNT: usize = 3;

const CHIP_GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const CASSETTE_SURROGATE_Z: f64 = 36.0;
const ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER;
const ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const DECK_X: f64 = 1340.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;
const PERIMETER_RIM_W: f64 = 18.0;
const PERIMETER_RIM_Z: f64 = 26.0;
const MOUNT_HOLE_D: f64 = 6.4;

const NEST_CENTER_X: f64 = -130.0;
const NEST_CENTER_Y: f64 = 75.0;
const NEST_SOCKET_DEPTH: f64 = 5.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 24.0;
const NEST_GRID_RIB_W: f64 = 4.0;
const NEST_GRID_RIB_Z: f64 = 9.0;
const NEST_LEDGE_W: f64 = 18.0;
const NEST_LEDGE_Z: f64 = 7.0;
const DATUM_PIN_D: f64 = 7.0;

const RACK_COMB_CENTER_X: f64 = 425.0;
const RACK_COMB_CENTER_Y: f64 = 250.0;
const RACK_SLOT_X: f64 = 58.0;
const RACK_SLOT_Y: f64 = 118.0;
const RACK_SLOT_Z: f64 = 22.0;
const RACK_SLOT_PITCH_X: f64 = 68.0;
const RACK_COMB_X: f64 = RACK_SLOT_COUNT as f64 * RACK_SLOT_PITCH_X + 58.0;
const RACK_COMB_Y: f64 = 172.0;
const RACK_COMB_Z: f64 = 28.0;
const RACK_EDGE_SLOT_COUNT: usize = 2;
const RACK_CENTER_SLOT_COUNT: usize = RACK_SLOT_COUNT - RACK_EDGE_SLOT_COUNT;

const PUCK_SOCKET_D: f64 = 38.0;
const PUCK_RECESS_D: f64 = 24.0;
const PUCK_SOCKET_Z: f64 = 7.0;
const PUCK_KEY_W: f64 = 8.0;
const PUCK_PULL_TAB_X: f64 = 14.0;
const PUCK_PULL_TAB_Y: f64 = 18.0;

const COUPON_BANK_CENTER_X: f64 = -520.0;
const COUPON_BANK_CENTER_Y: f64 = 168.0;
const COUPON_BANK_X: f64 = 218.0;
const COUPON_BANK_Y: f64 = 300.0;
const COUPON_BANK_Z: f64 = 18.0;
const COUPON_EDGE_D: f64 = 18.0;
const COUPON_CENTER_D: f64 = 22.0;
const COUPON_Z: f64 = 5.0;
const COUPON_ROW_PITCH: f64 = 36.0;
const COUPON_COL_PITCH: f64 = 64.0;

const LOGGER_BANK_CENTER_X: f64 = -520.0;
const LOGGER_BANK_CENTER_Y: f64 = -188.0;
const LOGGER_BANK_X: f64 = 220.0;
const LOGGER_BANK_Y: f64 = 228.0;
const LOGGER_BANK_Z: f64 = 32.0;
const LOGGER_POCKET_X: f64 = 76.0;
const LOGGER_POCKET_Y: f64 = 48.0;
const LOGGER_POCKET_Z: f64 = 18.0;
const LOGGER_RECESS_DEPTH: f64 = 8.0;
const LOGGER_CABLE_SLOT_W: f64 = 9.0;
const LOGGER_PITCH_X: f64 = 92.0;
const LOGGER_PITCH_Y: f64 = 76.0;

const TOKEN_BOARD_CENTER_X: f64 = 430.0;
const TOKEN_BOARD_CENTER_Y: f64 = 24.0;
const TOKEN_BOARD_X: f64 = 386.0;
const TOKEN_BOARD_Y: f64 = 188.0;
const TOKEN_BOARD_Z: f64 = 16.0;
const POSITION_TOKEN_D: f64 = 16.0;
const POSITION_TOKEN_Z: f64 = 4.0;
const BARCODE_LAND_X: f64 = 52.0;
const BARCODE_LAND_Y: f64 = 16.0;
const BARCODE_LAND_Z: f64 = 3.0;
const POSITION_TOKEN_COUNT: usize = CASSETTE_POSITION_COUNT + RACK_SLOT_COUNT + LOGGER_POCKET_COUNT;

const LANE_BANK_CENTER_X: f64 = 425.0;
const LANE_BANK_CENTER_Y: f64 = -270.0;
const LANE_BANK_X: f64 = 452.0;
const LANE_BANK_Y: f64 = 216.0;
const LANE_BANK_Z: f64 = 28.0;
const LANE_W: f64 = 122.0;
const LANE_Y: f64 = 170.0;
const LANE_WALL_W: f64 = 8.0;
const LANE_RECESS_DEPTH: f64 = 12.0;
const RELEASE_CAPACITY: usize = 8;
const HOLD_CAPACITY: usize = 8;
const REJECT_CAPACITY: usize = 4;

const CAMERA_POST_W: f64 = 28.0;
const CAMERA_POST_Z: f64 = 168.0;
const CAMERA_BRIDGE_X: f64 = CASSETTE_X + 126.0;
const CAMERA_BRIDGE_Y: f64 = 34.0;
const CAMERA_BRIDGE_Z: f64 = 24.0;
const CAMERA_CARRIAGE_X: f64 = 136.0;
const CAMERA_CARRIAGE_Y: f64 = 88.0;
const CAMERA_CARRIAGE_Z: f64 = 18.0;
const CAMERA_LENS_D: f64 = 32.0;
const EVIDENCE_LIGHT_BAR_X: f64 = 16.0;
const EVIDENCE_LIGHT_BAR_Y: f64 = CASSETTE_Y + 70.0;
const CAMERA_CLEARANCE_Z: f64 = CAMERA_POST_Z - CASSETTE_SURROGATE_Z;

const ROBOT_FRONT_KEEP_OUT_Y: f64 = 260.0;
const ROBOT_SIDE_KEEP_OUT_X: f64 = 92.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 128.0;
const SERVICE_REAR_KEEP_OUT_Y: f64 = 176.0;
const SERVICE_LOGGER_PULL_X: f64 = 124.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;

const CLEAN_USED_CENTER_X: f64 = -285.0;
const CLEAN_USED_CENTER_Y: f64 = -312.0;
const CLEAN_ZONE_X: f64 = 190.0;
const USED_ZONE_X: f64 = 190.0;
const CLEAN_USED_ZONE_Y: f64 = 142.0;
const CLEAN_USED_ZONE_Z: f64 = 22.0;
const CLEAN_USED_GAP: f64 = 72.0;
const SEGREGATION_RIB_W: f64 = 18.0;
const SEGREGATION_RIB_Z: f64 = 46.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoggerKind {
    Co2,
    O2,
    Rh,
    Temp,
}

impl LoggerKind {
    fn all() -> [LoggerKind; LOGGER_POCKET_COUNT] {
        [
            LoggerKind::Co2,
            LoggerKind::O2,
            LoggerKind::Rh,
            LoggerKind::Temp,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            LoggerKind::Co2 => "co2_gas",
            LoggerKind::O2 => "o2_gas",
            LoggerKind::Rh => "rh",
            LoggerKind::Temp => "temperature",
        }
    }

    fn index(self) -> usize {
        match self {
            LoggerKind::Co2 => 0,
            LoggerKind::O2 => 1,
            LoggerKind::Rh => 2,
            LoggerKind::Temp => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANE_COUNT] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_CAPACITY,
            DispositionLane::Hold => HOLD_CAPACITY,
            DispositionLane::Reject => REJECT_CAPACITY,
        }
    }
}

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

    let nest = cassette_surrogate_nest();
    export(OUTPUTS[1], &nest);

    let rack = incubator_rack_slot_reference_comb();
    export(OUTPUTS[2], &rack);

    let pucks = removable_sensor_puck_positions();
    export(OUTPUTS[3], &pucks);

    let coupons = edge_center_reference_coupons();
    export(OUTPUTS[4], &coupons);

    let loggers = gas_rh_temp_logger_pockets();
    export(OUTPUTS[5], &loggers);

    let tokens = barcode_position_tokens();
    export(OUTPUTS[6], &tokens);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[8], &camera);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let segregation = clean_used_segregation();
    export(OUTPUTS[10], &segregation);

    let assembly = deck
        + nest
        + rack
        + pucks
        + coupons
        + loggers
        + tokens
        + lanes
        + camera
        + keepouts
        + segregation;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette position environmental equivalence station:");
    println!(
        "  Cassette map:                {CASSETTE_COLS}x{CASSETTE_ROWS} surrogate nest, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center positions"
    );
    println!(
        "  Rack comparison:             {RACK_SLOT_COUNT} incubator slot references, {RACK_EDGE_SLOT_COUNT} outer-slot controls and {RACK_CENTER_SLOT_COUNT} center-slot controls"
    );
    println!(
        "  Environmental instrumentation:{CASSETTE_POSITION_COUNT} removable sensor puck positions plus {LOGGER_POCKET_COUNT} gas/RH/temp logger pockets"
    );
    println!(
        "  Traceability:                {POSITION_TOKEN_COUNT} barcode position tokens for cassette, rack, and logger records"
    );
    println!(
        "  Disposition lanes:           release/hold/reject capacity {} tokens",
        total_lane_capacity()
    );
    println!(
        "  Evidence capture:            {CAMERA_CLEARANCE_Z:.0}mm camera bridge clearance over {CASSETTE_SURROGATE_Z:.0}mm surrogate height"
    );
    println!(
        "  Clearances:                  {ROBOT_PICK_CLEARANCE_Z:.0}mm robot pick clearance, {SERVICE_REAR_KEEP_OUT_Y:.0}mm rear service zone, {:.0}mm clean-used gap",
        clean_used_gap()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(CASSETTE_POSITION_COUNT, 20);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(LoggerKind::all().len(), LOGGER_POCKET_COUNT);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert_eq!(total_lane_capacity(), CASSETTE_POSITION_COUNT);
    assert_eq!(
        POSITION_TOKEN_COUNT,
        CASSETTE_POSITION_COUNT + RACK_SLOT_COUNT + LOGGER_POCKET_COUNT
    );
    assert_eq!(OUTPUTS.len(), 12);
    assert!(CASSETTE_SURROGATE_Z > REVC_TOTAL_HEIGHT + 18.0);
    assert!(CASSETTE_X > ARRAY_X && CASSETTE_Y > ARRAY_Y);
    assert!(PUCK_RECESS_D < PUCK_SOCKET_D - 8.0);
    assert!(LOGGER_RECESS_DEPTH < LOGGER_POCKET_Z);
    assert!(CAMERA_CLEARANCE_Z >= 120.0);
    assert!(ROBOT_PICK_CLEARANCE_Z > CASSETTE_SURROGATE_Z + REVC_TOTAL_HEIGHT + 60.0);
    assert!(SERVICE_LOGGER_PULL_X > LOGGER_POCKET_X + 40.0);
    assert!(clean_used_gap() >= CLEAN_USED_GAP);
    assert!(!rects_overlap(clean_rect(), used_rect()));
    assert!(rack_slot_center_x(0) < rack_slot_center_x(RACK_SLOT_COUNT - 1));
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_position_equivalence_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - cassette_nest_recess()
        - rack_comb_recess()
        - coupon_bank_recess()
        - logger_bank_recess()
        - token_board_recess()
        - lane_bank_recess()
        - clean_used_recess()
        - runoff_channels()
        - deck_mount_holes()
        + perimeter_rims()
        + deck_datum_strips()
}

fn cassette_nest_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_nest_recess",
        CASSETTE_X + 48.0,
        CASSETTE_Y + 48.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn rack_comb_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_rack_comb_recess",
        RACK_COMB_X + 18.0,
        RACK_COMB_Y + 18.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        RACK_COMB_CENTER_X,
        RACK_COMB_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn coupon_bank_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_coupon_bank_recess",
        COUPON_BANK_X + 14.0,
        COUPON_BANK_Y + 14.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        COUPON_BANK_CENTER_X,
        COUPON_BANK_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn logger_bank_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_logger_bank_recess",
        LOGGER_BANK_X + 14.0,
        LOGGER_BANK_Y + 14.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn token_board_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_token_board_recess",
        TOKEN_BOARD_X + 14.0,
        TOKEN_BOARD_Y + 14.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        TOKEN_BOARD_CENTER_X,
        TOKEN_BOARD_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn lane_bank_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_lane_bank_recess",
        LANE_BANK_X + 16.0,
        LANE_BANK_Y + 16.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        LANE_BANK_CENTER_X,
        LANE_BANK_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn clean_used_recess() -> Part {
    centered_cube(
        "closed_cassette_position_equivalence_clean_used_recess",
        clean_used_total_x() + 24.0,
        CLEAN_USED_ZONE_Y + 24.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        CLEAN_USED_CENTER_X,
        CLEAN_USED_CENTER_Y,
        DECK_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn runoff_channels() -> Part {
    let front = centered_cube(
        "closed_cassette_position_equivalence_front_runoff_channel",
        DECK_X - 132.0,
        12.0,
        6.0,
    )
    .translate(0.0, deck_front_y() + 54.0, DECK_Z - 3.0);
    let nest_left = centered_cube(
        "closed_cassette_position_equivalence_left_nest_runoff_channel",
        10.0,
        CASSETTE_Y + 78.0,
        6.0,
    )
    .translate(cassette_left_edge() - 34.0, NEST_CENTER_Y, DECK_Z - 3.0);
    let nest_right = centered_cube(
        "closed_cassette_position_equivalence_right_nest_runoff_channel",
        10.0,
        CASSETTE_Y + 78.0,
        6.0,
    )
    .translate(cassette_right_edge() + 34.0, NEST_CENTER_Y, DECK_Z - 3.0);

    front + nest_left + nest_right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_cassette_position_equivalence_deck_mount_holes");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_position_equivalence_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                32,
            )
            .translate(
                sx * (DECK_X / 2.0 - 44.0),
                sy * (DECK_Y / 2.0 - 44.0),
                DECK_Z / 2.0,
            );
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "closed_cassette_position_equivalence_rear_service_rim",
        DECK_X,
        PERIMETER_RIM_W,
        PERIMETER_RIM_Z,
    )
    .translate(
        0.0,
        deck_rear_y() - PERIMETER_RIM_W / 2.0,
        DECK_Z + PERIMETER_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_cassette_position_equivalence_left_stop_rim",
        PERIMETER_RIM_W,
        DECK_Y,
        PERIMETER_RIM_Z,
    )
    .translate(
        deck_left_x() + PERIMETER_RIM_W / 2.0,
        0.0,
        DECK_Z + PERIMETER_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_cassette_position_equivalence_right_robot_rim",
        PERIMETER_RIM_W,
        DECK_Y - 120.0,
        PERIMETER_RIM_Z,
    )
    .translate(
        deck_right_x() - PERIMETER_RIM_W / 2.0,
        42.0,
        DECK_Z + PERIMETER_RIM_Z / 2.0,
    );
    let front_low = centered_cube(
        "closed_cassette_position_equivalence_front_low_clean_lip",
        DECK_X - 148.0,
        12.0,
        18.0,
    )
    .translate(0.0, deck_front_y() + 24.0, DECK_Z + 9.0);

    rear + left + right + front_low
}

fn deck_datum_strips() -> Part {
    let rear_strip = centered_cube(
        "closed_cassette_position_equivalence_rear_datum_strip",
        DECK_X - 220.0,
        10.0,
        5.0,
    )
    .translate(0.0, deck_rear_y() - 68.0, DECK_Z + 2.5);
    let left_strip = centered_cube(
        "closed_cassette_position_equivalence_left_datum_strip",
        10.0,
        DECK_Y - 180.0,
        5.0,
    )
    .translate(deck_left_x() + 68.0, 0.0, DECK_Z + 2.5);

    rear_strip + left_strip
}

fn cassette_surrogate_nest() -> Part {
    cassette_nest_rails()
        + cassette_position_grid()
        + cassette_position_floor_pads()
        + cassette_nest_datum_pins()
        + cassette_loading_fiducials()
}

fn cassette_nest_rails() -> Part {
    let left = centered_cube(
        "closed_cassette_position_equivalence_left_x_datum_rail",
        NEST_RAIL_W,
        CASSETTE_Y + 30.0,
        NEST_RAIL_Z,
    )
    .translate(
        cassette_left_edge() - NEST_RAIL_W / 2.0 - 3.0,
        NEST_CENTER_Y,
        DECK_Z + NEST_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_cassette_position_equivalence_rear_y_datum_rail",
        CASSETTE_X + 34.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        NEST_CENTER_X,
        cassette_rear_edge() + NEST_RAIL_W / 2.0 + 3.0,
        DECK_Z + NEST_RAIL_Z / 2.0,
    );
    let right_soft = centered_cube(
        "closed_cassette_position_equivalence_right_soft_capture_rail",
        NEST_RAIL_W,
        CASSETTE_Y * 0.76,
        NEST_RAIL_Z * 0.62,
    )
    .translate(
        cassette_right_edge() + NEST_RAIL_W / 2.0 + 3.0,
        NEST_CENTER_Y - 10.0,
        DECK_Z + NEST_RAIL_Z * 0.31,
    );
    let front_low = centered_cube(
        "closed_cassette_position_equivalence_front_low_loading_lip",
        CASSETTE_X + 36.0,
        10.0,
        15.0,
    )
    .translate(NEST_CENTER_X, cassette_front_edge() - 9.0, DECK_Z + 7.5);
    let left_ledge = centered_cube(
        "closed_cassette_position_equivalence_left_support_ledge",
        NEST_LEDGE_W,
        CASSETTE_Y - 44.0,
        NEST_LEDGE_Z,
    )
    .translate(
        cassette_left_edge() + 36.0,
        NEST_CENTER_Y,
        DECK_Z + NEST_LEDGE_Z / 2.0,
    );
    let right_ledge = centered_cube(
        "closed_cassette_position_equivalence_right_support_ledge",
        NEST_LEDGE_W,
        CASSETTE_Y - 44.0,
        NEST_LEDGE_Z,
    )
    .translate(
        cassette_right_edge() - 36.0,
        NEST_CENTER_Y,
        DECK_Z + NEST_LEDGE_Z / 2.0,
    );

    left + rear + right_soft + front_low + left_ledge + right_ledge
}

fn cassette_position_grid() -> Part {
    let mut grid = Part::empty("closed_cassette_position_equivalence_4x5_grid_ribs");
    for col in 1..CASSETTE_COLS {
        let x = cassette_left_edge()
            + CASSETTE_MARGIN_X
            + col as f64 * REVC_CHIP_LENGTH
            + (col as f64 - 0.5) * CHIP_GUTTER;
        grid = grid
            + centered_cube(
                format!("closed_cassette_position_equivalence_col_separator_{col}"),
                NEST_GRID_RIB_W,
                ARRAY_Y + 18.0,
                NEST_GRID_RIB_Z,
            )
            .translate(x, NEST_CENTER_Y, DECK_Z + NEST_GRID_RIB_Z / 2.0);
    }
    for row in 1..CASSETTE_ROWS {
        let y = cassette_front_edge()
            + CASSETTE_MARGIN_Y
            + row as f64 * REVC_CHIP_WIDTH
            + (row as f64 - 0.5) * CHIP_GUTTER;
        grid = grid
            + centered_cube(
                format!("closed_cassette_position_equivalence_row_separator_{row}"),
                ARRAY_X + 18.0,
                NEST_GRID_RIB_W,
                NEST_GRID_RIB_Z,
            )
            .translate(NEST_CENTER_X, y, DECK_Z + NEST_GRID_RIB_Z / 2.0);
    }
    grid
}

fn cassette_position_floor_pads() -> Part {
    let mut pads = Part::empty("closed_cassette_position_equivalence_position_floor_pads");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = position_index(col, row);
            let (x, y) = cassette_position_center(col, row);
            let zone = if is_edge_position(col, row) {
                "edge"
            } else {
                "center"
            };
            let pad = centered_cube(
                format!("closed_cassette_position_equivalence_{zone}_position_floor_pad_{index}"),
                REVC_CHIP_LENGTH - 26.0,
                REVC_CHIP_WIDTH - 24.0,
                4.0,
            )
            .translate(x, y, DECK_Z + 2.0);
            let witness = centered_cylinder(
                format!("closed_cassette_position_equivalence_{zone}_condensate_witness_{index}"),
                7.0,
                2.0,
                28,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 24.0,
                y + REVC_CHIP_WIDTH / 2.0 - 18.0,
                DECK_Z + 5.0,
            );
            pads = pads + pad + witness;
        }
    }
    pads
}

fn cassette_nest_datum_pins() -> Part {
    let mut pins = Part::empty("closed_cassette_position_equivalence_nest_datum_pins");
    for (i, (x, y)) in nest_datum_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_cassette_position_equivalence_nest_datum_boss_{i}"),
            DATUM_PIN_D,
            8.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        let pilot = centered_cylinder(
            format!("closed_cassette_position_equivalence_nest_datum_pilot_{i}"),
            DATUM_PIN_D * 0.42,
            12.0,
            28,
        )
        .translate(*x, *y, DECK_Z + 6.0);
        pins = pins + boss + pilot;
    }
    pins
}

fn cassette_loading_fiducials() -> Part {
    let mut marks = Part::empty("closed_cassette_position_equivalence_loading_fiducials");
    for (i, (x, y)) in [
        (cassette_left_edge() + 26.0, cassette_rear_edge() - 26.0),
        (cassette_right_edge() - 26.0, cassette_rear_edge() - 26.0),
        (cassette_left_edge() + 26.0, cassette_front_edge() + 26.0),
    ]
    .iter()
    .enumerate()
    {
        marks = marks
            + centered_cylinder(
                format!("closed_cassette_position_equivalence_robot_fiducial_{i}"),
                8.0,
                3.0,
                36,
            )
            .translate(*x, *y, DECK_Z + 1.5);
    }
    marks
}

fn incubator_rack_slot_reference_comb() -> Part {
    let base = centered_cube(
        "closed_cassette_position_equivalence_rack_slot_comb_base",
        RACK_COMB_X,
        RACK_COMB_Y,
        RACK_COMB_Z,
    )
    .translate(
        RACK_COMB_CENTER_X,
        RACK_COMB_CENTER_Y,
        DECK_Z + RACK_COMB_Z / 2.0,
    );

    let mut reliefs = Part::empty("closed_cassette_position_equivalence_rack_slot_reliefs");
    let mut keys = Part::empty("closed_cassette_position_equivalence_rack_slot_keys");
    for slot in 0..RACK_SLOT_COUNT {
        let x = RACK_COMB_CENTER_X + rack_slot_center_x(slot);
        let y = RACK_COMB_CENTER_Y;
        let relief = centered_cube(
            format!("closed_cassette_position_equivalence_rack_slot_{slot}_cassette_relief"),
            RACK_SLOT_X,
            RACK_SLOT_Y,
            RACK_SLOT_Z + 2.0,
        )
        .translate(x, y, DECK_Z + RACK_COMB_Z - RACK_SLOT_Z / 2.0 + 1.0);
        reliefs = reliefs + relief;

        let label = if is_outer_rack_slot(slot) {
            "outer"
        } else {
            "center"
        };
        let front_key = centered_cube(
            format!("closed_cassette_position_equivalence_{label}_rack_slot_{slot}_front_key"),
            RACK_SLOT_X * 0.44,
            8.0,
            7.0,
        )
        .translate(x, y - RACK_SLOT_Y / 2.0 - 12.0, DECK_Z + RACK_COMB_Z + 3.5);
        let rear_key = centered_cube(
            format!("closed_cassette_position_equivalence_{label}_rack_slot_{slot}_rear_key"),
            RACK_SLOT_X * 0.28,
            8.0,
            7.0,
        )
        .translate(x, y + RACK_SLOT_Y / 2.0 + 12.0, DECK_Z + RACK_COMB_Z + 3.5);
        let slot_token = centered_cylinder(
            format!("closed_cassette_position_equivalence_{label}_rack_slot_{slot}_token"),
            if is_outer_rack_slot(slot) { 9.5 } else { 7.0 },
            4.0,
            32,
        )
        .translate(x, y, DECK_Z + RACK_COMB_Z + 2.0);
        keys = keys + front_key + rear_key + slot_token;
    }

    base - reliefs + keys
}

fn removable_sensor_puck_positions() -> Part {
    let mut pucks = Part::empty("closed_cassette_position_equivalence_removable_sensor_pucks");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = position_index(col, row);
            let (x, y) = cassette_position_center(col, row);
            let label = if is_edge_position(col, row) {
                "edge"
            } else {
                "center"
            };
            pucks = pucks
                + sensor_puck_socket(index, label).translate(x, y, DECK_Z + PUCK_SOCKET_Z / 2.0);
        }
    }
    pucks + spare_puck_garage()
}

fn sensor_puck_socket(index: usize, label: &str) -> Part {
    let outer = centered_cylinder(
        format!("closed_cassette_position_equivalence_{label}_sensor_puck_outer_socket_{index}"),
        PUCK_SOCKET_D / 2.0,
        PUCK_SOCKET_Z,
        48,
    );
    let recess = centered_cylinder(
        format!("closed_cassette_position_equivalence_{label}_sensor_puck_recess_{index}"),
        PUCK_RECESS_D / 2.0,
        PUCK_SOCKET_Z + 1.0,
        42,
    )
    .translate(0.0, 0.0, 0.5);
    let key = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_sensor_puck_keyway_{index}"),
        PUCK_KEY_W,
        PUCK_SOCKET_D,
        PUCK_SOCKET_Z + 1.0,
    )
    .translate(PUCK_SOCKET_D / 2.0 - PUCK_KEY_W / 2.0, 0.0, 0.5);
    let tab = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_sensor_puck_pull_tab_{index}"),
        PUCK_PULL_TAB_X,
        PUCK_PULL_TAB_Y,
        4.0,
    )
    .translate(
        -(PUCK_SOCKET_D / 2.0 + PUCK_PULL_TAB_X / 2.0 - 2.0),
        0.0,
        1.5,
    );
    let airflow_notch = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_sensor_puck_air_notch_{index}"),
        7.0,
        PUCK_SOCKET_D + 6.0,
        2.4,
    )
    .translate(0.0, 0.0, PUCK_SOCKET_Z / 2.0);

    outer - recess - key - airflow_notch + tab
}

fn spare_puck_garage() -> Part {
    let mut garage = Part::empty("closed_cassette_position_equivalence_spare_puck_garage");
    let x0 = LOGGER_BANK_CENTER_X - 52.0;
    let y0 = LOGGER_BANK_CENTER_Y - LOGGER_BANK_Y / 2.0 - 44.0;
    for i in 0..LOGGER_POCKET_COUNT {
        let socket = sensor_puck_socket(i, "spare").translate(
            x0 + i as f64 * 34.0,
            y0,
            DECK_Z + PUCK_SOCKET_Z / 2.0,
        );
        garage = garage + socket;
    }
    garage
}

fn edge_center_reference_coupons() -> Part {
    let bank = centered_cube(
        "closed_cassette_position_equivalence_edge_center_coupon_bank",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(
        COUPON_BANK_CENTER_X,
        COUPON_BANK_CENTER_Y,
        DECK_Z + COUPON_BANK_Z / 2.0,
    );

    let mut coupons = Part::empty("closed_cassette_position_equivalence_reference_coupons");
    for i in 0..EDGE_POSITION_COUNT {
        let (x, y) = coupon_center(i, EDGE_POSITION_COUNT, 0);
        coupons = coupons
            + reference_coupon(i, true).translate(x, y, DECK_Z + COUPON_BANK_Z + COUPON_Z / 2.0);
    }
    for i in 0..CENTER_POSITION_COUNT {
        let (x, y) = coupon_center(i, CENTER_POSITION_COUNT, 1);
        coupons = coupons
            + reference_coupon(i, false).translate(x, y, DECK_Z + COUPON_BANK_Z + COUPON_Z / 2.0);
    }

    let divider = centered_cube(
        "closed_cassette_position_equivalence_coupon_edge_center_divider",
        COUPON_BANK_X - 28.0,
        5.0,
        9.0,
    )
    .translate(
        COUPON_BANK_CENTER_X,
        COUPON_BANK_CENTER_Y,
        DECK_Z + COUPON_BANK_Z + 4.5,
    );

    bank + coupons + divider
}

fn reference_coupon(index: usize, edge: bool) -> Part {
    let label = if edge { "edge" } else { "center" };
    let radius = if edge {
        COUPON_EDGE_D / 2.0
    } else {
        COUPON_CENTER_D / 2.0
    };
    let coupon = centered_cylinder(
        format!("closed_cassette_position_equivalence_{label}_reference_coupon_{index}"),
        radius,
        COUPON_Z,
        36,
    );
    let orientation_tick = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_coupon_orientation_tick_{index}"),
        3.0,
        radius * 1.3,
        2.0,
    )
    .translate(radius * 0.45, 0.0, COUPON_Z / 2.0);

    coupon + orientation_tick
}

fn gas_rh_temp_logger_pockets() -> Part {
    let base = centered_cube(
        "closed_cassette_position_equivalence_logger_bank_base",
        LOGGER_BANK_X,
        LOGGER_BANK_Y,
        LOGGER_BANK_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        DECK_Z + LOGGER_BANK_Z / 2.0,
    );

    let mut pockets = Part::empty("closed_cassette_position_equivalence_logger_pockets");
    for kind in LoggerKind::all() {
        let (x, y) = logger_center(kind);
        pockets = pockets + logger_pocket(kind).translate(x, y, DECK_Z + LOGGER_BANK_Z / 2.0);
    }

    let cable_comb = logger_cable_comb();
    base + pockets + cable_comb
}

fn logger_pocket(kind: LoggerKind) -> Part {
    let label = kind.label();
    let body = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_logger_pocket_body"),
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        LOGGER_POCKET_Z,
    );
    let recess = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_logger_recess"),
        LOGGER_POCKET_X - 16.0,
        LOGGER_POCKET_Y - 14.0,
        LOGGER_RECESS_DEPTH + 1.0,
    )
    .translate(
        0.0,
        0.0,
        LOGGER_POCKET_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0 + 0.6,
    );
    let cable = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_logger_cable_slot"),
        LOGGER_CABLE_SLOT_W,
        LOGGER_POCKET_Y + 4.0,
        LOGGER_RECESS_DEPTH + 2.0,
    )
    .translate(
        LOGGER_POCKET_X / 2.0 - 14.0,
        0.0,
        LOGGER_POCKET_Z / 2.0 - 2.0,
    );
    let diffusion = match kind {
        LoggerKind::Co2 | LoggerKind::O2 => gas_diffusion_windows(label),
        LoggerKind::Rh => rh_louver_windows(),
        LoggerKind::Temp => temp_probe_cradle(),
    };

    body - recess - cable - diffusion
}

fn gas_diffusion_windows(label: &str) -> Part {
    let mut windows = Part::empty(format!(
        "closed_cassette_position_equivalence_{label}_diffusion_windows"
    ));
    for i in 0..3 {
        windows = windows
            + centered_cylinder(
                format!("closed_cassette_position_equivalence_{label}_diffusion_window_{i}"),
                4.0,
                LOGGER_RECESS_DEPTH + 2.0,
                28,
            )
            .translate(-18.0 + i as f64 * 18.0, -12.0, LOGGER_POCKET_Z / 2.0);
    }
    windows
}

fn rh_louver_windows() -> Part {
    let mut louvers = Part::empty("closed_cassette_position_equivalence_rh_louver_windows");
    for i in 0..4 {
        louvers = louvers
            + centered_cube(
                format!("closed_cassette_position_equivalence_rh_louver_window_{i}"),
                44.0,
                2.5,
                LOGGER_RECESS_DEPTH + 2.0,
            )
            .translate(0.0, -15.0 + i as f64 * 9.0, LOGGER_POCKET_Z / 2.0);
    }
    louvers
}

fn temp_probe_cradle() -> Part {
    let probe = centered_cylinder(
        "closed_cassette_position_equivalence_temperature_probe_cradle",
        2.3,
        58.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(4.0, 0.0, LOGGER_POCKET_Z / 2.0);
    let bead = centered_cylinder(
        "closed_cassette_position_equivalence_temperature_bead_cup",
        5.0,
        LOGGER_RECESS_DEPTH + 2.0,
        28,
    )
    .translate(-22.0, 0.0, LOGGER_POCKET_Z / 2.0);

    probe + bead
}

fn logger_cable_comb() -> Part {
    let mut comb = Part::empty("closed_cassette_position_equivalence_logger_cable_comb");
    let comb_y = LOGGER_BANK_CENTER_Y + LOGGER_BANK_Y / 2.0 - 24.0;
    for i in 0..LOGGER_POCKET_COUNT {
        let x = LOGGER_BANK_CENTER_X - 68.0 + i as f64 * 46.0;
        let clamp = centered_cube(
            format!("closed_cassette_position_equivalence_logger_cable_clamp_{i}"),
            26.0,
            12.0,
            12.0,
        )
        .translate(x, comb_y, DECK_Z + LOGGER_BANK_Z + 6.0);
        let slot = centered_cylinder(
            format!("closed_cassette_position_equivalence_logger_cable_passage_{i}"),
            3.0,
            30.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, comb_y, DECK_Z + LOGGER_BANK_Z + 6.0);
        comb = comb + (clamp - slot);
    }
    comb
}

fn barcode_position_tokens() -> Part {
    let board = centered_cube(
        "closed_cassette_position_equivalence_barcode_token_board",
        TOKEN_BOARD_X,
        TOKEN_BOARD_Y,
        TOKEN_BOARD_Z,
    )
    .translate(
        TOKEN_BOARD_CENTER_X,
        TOKEN_BOARD_CENTER_Y,
        DECK_Z + TOKEN_BOARD_Z / 2.0,
    );

    let mut tokens = Part::empty("closed_cassette_position_equivalence_barcode_position_tokens");
    for i in 0..POSITION_TOKEN_COUNT {
        let (x, y) = token_center(i);
        let zone = token_zone_label(i);
        let puck = centered_cylinder(
            format!("closed_cassette_position_equivalence_{zone}_barcode_token_{i}"),
            POSITION_TOKEN_D / 2.0,
            POSITION_TOKEN_Z,
            32,
        )
        .translate(x, y + 18.0, DECK_Z + TOKEN_BOARD_Z + POSITION_TOKEN_Z / 2.0);
        let land = centered_cube(
            format!("closed_cassette_position_equivalence_{zone}_barcode_land_{i}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            BARCODE_LAND_Z,
        )
        .translate(x, y - 12.0, DECK_Z + TOKEN_BOARD_Z + BARCODE_LAND_Z / 2.0);
        tokens = tokens + puck + land;
    }

    board + tokens
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_cassette_position_equivalence_disposition_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(
        LANE_BANK_CENTER_X,
        LANE_BANK_CENTER_Y,
        DECK_Z + LANE_BANK_Z / 2.0,
    );

    let mut lane_reliefs = Part::empty("closed_cassette_position_equivalence_lane_reliefs");
    let mut lane_features = Part::empty("closed_cassette_position_equivalence_lane_features");
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        let relief = centered_cube(
            format!(
                "closed_cassette_position_equivalence_{}_lane_recess",
                lane.label()
            ),
            LANE_W,
            LANE_Y,
            LANE_RECESS_DEPTH + 1.0,
        )
        .translate(
            x,
            LANE_BANK_CENTER_Y,
            DECK_Z + LANE_BANK_Z - LANE_RECESS_DEPTH / 2.0 + 0.5,
        );
        lane_reliefs = lane_reliefs + relief;

        let left_wall = centered_cube(
            format!(
                "closed_cassette_position_equivalence_{}_lane_left_wall",
                lane.label()
            ),
            LANE_WALL_W,
            LANE_Y,
            18.0,
        )
        .translate(
            x - LANE_W / 2.0,
            LANE_BANK_CENTER_Y,
            DECK_Z + LANE_BANK_Z + 9.0,
        );
        let right_wall = centered_cube(
            format!(
                "closed_cassette_position_equivalence_{}_lane_right_wall",
                lane.label()
            ),
            LANE_WALL_W,
            LANE_Y,
            18.0,
        )
        .translate(
            x + LANE_W / 2.0,
            LANE_BANK_CENTER_Y,
            DECK_Z + LANE_BANK_Z + 9.0,
        );
        lane_features = lane_features + left_wall + right_wall + lane_token_stops(lane);
    }

    base - lane_reliefs + lane_features
}

fn lane_token_stops(lane: DispositionLane) -> Part {
    let mut stops = Part::empty(format!(
        "closed_cassette_position_equivalence_{}_lane_token_stops",
        lane.label()
    ));
    let x = disposition_lane_x(lane);
    for i in 0..lane.capacity() {
        let y = LANE_BANK_CENTER_Y + centered_index(i, lane.capacity(), 18.0);
        let stop = centered_cylinder(
            format!(
                "closed_cassette_position_equivalence_{}_lane_token_stop_{i}",
                lane.label()
            ),
            4.0,
            5.0,
            20,
        )
        .translate(x, y, DECK_Z + LANE_BANK_Z + 2.5);
        stops = stops + stop;
    }
    stops
}

fn evidence_camera_bridge() -> Part {
    let post_y_front = cassette_front_edge() - 34.0;
    let post_y_rear = cassette_rear_edge() + 34.0;
    let post_x_left = NEST_CENTER_X - CAMERA_BRIDGE_X / 2.0;
    let post_x_right = NEST_CENTER_X + CAMERA_BRIDGE_X / 2.0;

    let mut posts = Part::empty("closed_cassette_position_equivalence_camera_bridge_posts");
    for (i, (x, y)) in [
        (post_x_left, post_y_front),
        (post_x_right, post_y_front),
        (post_x_left, post_y_rear),
        (post_x_right, post_y_rear),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("closed_cassette_position_equivalence_camera_post_{i}"),
            CAMERA_POST_W,
            CAMERA_POST_W,
            CAMERA_POST_Z,
        )
        .translate(*x, *y, DECK_Z + CAMERA_POST_Z / 2.0);
        posts = posts + post;
    }

    let bridge = centered_cube(
        "closed_cassette_position_equivalence_camera_cross_bridge",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z + CAMERA_POST_Z + CAMERA_BRIDGE_Z / 2.0,
    );
    let carriage = centered_cube(
        "closed_cassette_position_equivalence_camera_carriage_plate",
        CAMERA_CARRIAGE_X,
        CAMERA_CARRIAGE_Y,
        CAMERA_CARRIAGE_Z,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z + CAMERA_POST_Z + CAMERA_BRIDGE_Z + CAMERA_CARRIAGE_Z / 2.0,
    );
    let lens_bore = centered_cylinder(
        "closed_cassette_position_equivalence_camera_lens_bore",
        CAMERA_LENS_D / 2.0,
        CAMERA_CARRIAGE_Z + 2.0,
        48,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z + CAMERA_POST_Z + CAMERA_BRIDGE_Z + CAMERA_CARRIAGE_Z / 2.0,
    );
    let light_left = centered_cube(
        "closed_cassette_position_equivalence_left_evidence_light_bar",
        EVIDENCE_LIGHT_BAR_X,
        EVIDENCE_LIGHT_BAR_Y,
        12.0,
    )
    .translate(
        NEST_CENTER_X - CASSETTE_X / 2.0 - 28.0,
        NEST_CENTER_Y,
        DECK_Z + CAMERA_POST_Z - 22.0,
    );
    let light_right = centered_cube(
        "closed_cassette_position_equivalence_right_evidence_light_bar",
        EVIDENCE_LIGHT_BAR_X,
        EVIDENCE_LIGHT_BAR_Y,
        12.0,
    )
    .translate(
        NEST_CENTER_X + CASSETTE_X / 2.0 + 28.0,
        NEST_CENTER_Y,
        DECK_Z + CAMERA_POST_Z - 22.0,
    );
    let field_frame = evidence_field_of_view_frame();

    posts + bridge + (carriage - lens_bore) + light_left + light_right + field_frame
}

fn evidence_field_of_view_frame() -> Part {
    let z = DECK_Z + CAMERA_POST_Z - 52.0;
    let front = centered_cube(
        "closed_cassette_position_equivalence_evidence_fov_front_edge",
        CASSETTE_X + 18.0,
        4.0,
        6.0,
    )
    .translate(NEST_CENTER_X, cassette_front_edge() - 8.0, z);
    let rear = centered_cube(
        "closed_cassette_position_equivalence_evidence_fov_rear_edge",
        CASSETTE_X + 18.0,
        4.0,
        6.0,
    )
    .translate(NEST_CENTER_X, cassette_rear_edge() + 8.0, z);
    let left = centered_cube(
        "closed_cassette_position_equivalence_evidence_fov_left_edge",
        4.0,
        CASSETTE_Y + 18.0,
        6.0,
    )
    .translate(cassette_left_edge() - 8.0, NEST_CENTER_Y, z);
    let right = centered_cube(
        "closed_cassette_position_equivalence_evidence_fov_right_edge",
        4.0,
        CASSETTE_Y + 18.0,
        6.0,
    )
    .translate(cassette_right_edge() + 8.0, NEST_CENTER_Y, z);

    front + rear + left + right
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_cassette_position_equivalence_front_robot_pick_keepout",
        DECK_X - 170.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(
        0.0,
        deck_front_y() + ROBOT_FRONT_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_PAD_Z / 2.0,
    );
    let right_robot = centered_cube(
        "closed_cassette_position_equivalence_right_robot_side_keepout",
        ROBOT_SIDE_KEEP_OUT_X,
        DECK_Y - 150.0,
        KEEP_OUT_PAD_Z,
    )
    .translate(
        deck_right_x() - ROBOT_SIDE_KEEP_OUT_X / 2.0,
        32.0,
        DECK_Z + KEEP_OUT_PAD_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_cassette_position_equivalence_rear_service_keepout",
        DECK_X - 210.0,
        SERVICE_REAR_KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(
        0.0,
        deck_rear_y() - SERVICE_REAR_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_PAD_Z / 2.0,
    );
    let logger_pull = centered_cube(
        "closed_cassette_position_equivalence_logger_pull_service_keepout",
        SERVICE_LOGGER_PULL_X,
        LOGGER_BANK_Y + 70.0,
        KEEP_OUT_PAD_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X - LOGGER_BANK_X / 2.0 - SERVICE_LOGGER_PULL_X / 2.0,
        LOGGER_BANK_CENTER_Y,
        DECK_Z + KEEP_OUT_PAD_Z / 2.0,
    );
    let vertical_pick = centered_cube(
        "closed_cassette_position_equivalence_vertical_robot_pick_clearance_marker",
        CASSETTE_X + 92.0,
        CASSETTE_Y + 92.0,
        10.0,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z + ROBOT_PICK_CLEARANCE_Z,
    );

    front_robot + right_robot + rear_service + logger_pull + vertical_pick
}

fn clean_used_segregation() -> Part {
    let clean = centered_cube(
        "closed_cassette_position_equivalence_clean_sensor_puck_staging_zone",
        CLEAN_ZONE_X,
        CLEAN_USED_ZONE_Y,
        CLEAN_USED_ZONE_Z,
    )
    .translate(
        clean_rect().x,
        clean_rect().y,
        DECK_Z + CLEAN_USED_ZONE_Z / 2.0,
    );
    let used = centered_cube(
        "closed_cassette_position_equivalence_used_sensor_puck_return_zone",
        USED_ZONE_X,
        CLEAN_USED_ZONE_Y,
        CLEAN_USED_ZONE_Z,
    )
    .translate(
        used_rect().x,
        used_rect().y,
        DECK_Z + CLEAN_USED_ZONE_Z / 2.0,
    );
    let rib = centered_cube(
        "closed_cassette_position_equivalence_clean_used_segregation_rib",
        SEGREGATION_RIB_W,
        CLEAN_USED_ZONE_Y + 40.0,
        SEGREGATION_RIB_Z,
    )
    .translate(
        CLEAN_USED_CENTER_X,
        CLEAN_USED_CENTER_Y,
        DECK_Z + SEGREGATION_RIB_Z / 2.0,
    );
    let transfer_gate = centered_cube(
        "closed_cassette_position_equivalence_one_way_clean_used_transfer_gate",
        SEGREGATION_RIB_W + 10.0,
        42.0,
        12.0,
    )
    .translate(
        CLEAN_USED_CENTER_X,
        CLEAN_USED_CENTER_Y - CLEAN_USED_ZONE_Y / 2.0 + 36.0,
        DECK_Z + SEGREGATION_RIB_Z + 6.0,
    );
    let clean_token_rail = token_rail("clean", clean_rect().x, clean_rect().y);
    let used_token_rail = token_rail("used", used_rect().x, used_rect().y);

    clean + used + rib + transfer_gate + clean_token_rail + used_token_rail
}

fn token_rail(label: &str, x: f64, y: f64) -> Part {
    let rail = centered_cube(
        format!("closed_cassette_position_equivalence_{label}_zone_token_rail"),
        142.0,
        10.0,
        9.0,
    )
    .translate(
        x,
        y + CLEAN_USED_ZONE_Y / 2.0 - 24.0,
        DECK_Z + CLEAN_USED_ZONE_Z + 4.5,
    );
    let mut stops = Part::empty(format!(
        "closed_cassette_position_equivalence_{label}_zone_token_stops"
    ));
    for i in 0..4 {
        let stop = centered_cylinder(
            format!("closed_cassette_position_equivalence_{label}_zone_token_stop_{i}"),
            4.0,
            5.0,
            20,
        )
        .translate(
            x + centered_index(i, 4, 34.0),
            y + CLEAN_USED_ZONE_Y / 2.0 - 24.0,
            DECK_Z + CLEAN_USED_ZONE_Z + 11.5,
        );
        stops = stops + stop;
    }
    rail + stops
}

fn cassette_position_center(col: usize, row: usize) -> (f64, f64) {
    (
        cassette_left_edge()
            + CASSETTE_MARGIN_X
            + REVC_CHIP_LENGTH / 2.0
            + col as f64 * (REVC_CHIP_LENGTH + CHIP_GUTTER),
        cassette_front_edge()
            + CASSETTE_MARGIN_Y
            + REVC_CHIP_WIDTH / 2.0
            + row as f64 * (REVC_CHIP_WIDTH + CHIP_GUTTER),
    )
}

fn cassette_left_edge() -> f64 {
    NEST_CENTER_X - CASSETTE_X / 2.0
}

fn cassette_right_edge() -> f64 {
    NEST_CENTER_X + CASSETTE_X / 2.0
}

fn cassette_front_edge() -> f64 {
    NEST_CENTER_Y - CASSETTE_Y / 2.0
}

fn cassette_rear_edge() -> f64 {
    NEST_CENTER_Y + CASSETTE_Y / 2.0
}

fn deck_left_x() -> f64 {
    -DECK_X / 2.0
}

fn deck_right_x() -> f64 {
    DECK_X / 2.0
}

fn deck_front_y() -> f64 {
    -DECK_Y / 2.0
}

fn deck_rear_y() -> f64 {
    DECK_Y / 2.0
}

fn position_index(col: usize, row: usize) -> usize {
    row * CASSETTE_COLS + col
}

fn is_edge_position(col: usize, row: usize) -> bool {
    col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            if is_edge_position(col, row) {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    CASSETTE_POSITION_COUNT - edge_position_count()
}

fn nest_datum_points() -> [(f64, f64); 4] {
    [
        (cassette_left_edge() + 24.0, cassette_front_edge() + 24.0),
        (cassette_right_edge() - 24.0, cassette_front_edge() + 24.0),
        (cassette_left_edge() + 24.0, cassette_rear_edge() - 24.0),
        (cassette_right_edge() - 24.0, cassette_rear_edge() - 24.0),
    ]
}

fn rack_slot_center_x(slot: usize) -> f64 {
    centered_index(slot, RACK_SLOT_COUNT, RACK_SLOT_PITCH_X)
}

fn is_outer_rack_slot(slot: usize) -> bool {
    slot == 0 || slot == RACK_SLOT_COUNT - 1
}

fn coupon_center(index: usize, count: usize, row: usize) -> (f64, f64) {
    let cols = if count > 7 { 7 } else { count };
    let col = index % cols;
    let subrow = index / cols;
    let x = COUPON_BANK_CENTER_X + centered_index(col, cols, COUPON_COL_PITCH);
    let y = COUPON_BANK_CENTER_Y + if row == 0 { 62.0 } else { -72.0 }
        - subrow as f64 * COUPON_ROW_PITCH;
    (x, y)
}

fn logger_center(kind: LoggerKind) -> (f64, f64) {
    let col = kind.index() % 2;
    let row = kind.index() / 2;
    (
        LOGGER_BANK_CENTER_X + centered_index(col, 2, LOGGER_PITCH_X),
        LOGGER_BANK_CENTER_Y + centered_index(row, 2, LOGGER_PITCH_Y),
    )
}

fn token_center(index: usize) -> (f64, f64) {
    let cols = 6;
    let col = index % cols;
    let row = index / cols;
    (
        TOKEN_BOARD_CENTER_X + centered_index(col, cols, 58.0),
        TOKEN_BOARD_CENTER_Y + centered_index(row, 5, 34.0),
    )
}

fn token_zone_label(index: usize) -> &'static str {
    if index < CASSETTE_POSITION_COUNT {
        "cassette_position"
    } else if index < CASSETTE_POSITION_COUNT + RACK_SLOT_COUNT {
        "rack_slot"
    } else {
        "logger"
    }
}

fn disposition_lane_x(lane: DispositionLane) -> f64 {
    LANE_BANK_CENTER_X + centered_index(lane.index(), DISPOSITION_LANE_COUNT, LANE_W + 22.0)
}

fn total_lane_capacity() -> usize {
    DispositionLane::all()
        .iter()
        .map(|lane| lane.capacity())
        .sum()
}

fn clean_rect() -> Rect {
    Rect {
        x: CLEAN_USED_CENTER_X - (CLEAN_ZONE_X / 2.0 + CLEAN_USED_GAP / 2.0),
        y: CLEAN_USED_CENTER_Y,
        w: CLEAN_ZONE_X,
        h: CLEAN_USED_ZONE_Y,
    }
}

fn used_rect() -> Rect {
    Rect {
        x: CLEAN_USED_CENTER_X + (USED_ZONE_X / 2.0 + CLEAN_USED_GAP / 2.0),
        y: CLEAN_USED_CENTER_Y,
        w: USED_ZONE_X,
        h: CLEAN_USED_ZONE_Y,
    }
}

fn clean_used_total_x() -> f64 {
    CLEAN_ZONE_X + CLEAN_USED_GAP + USED_ZONE_X
}

fn clean_used_gap() -> f64 {
    horizontal_gap(clean_rect(), used_rect())
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
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
    fn cassette_surrogate_map_has_expected_edge_and_center_positions() {
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(CASSETTE_POSITION_COUNT, 20);
        assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
        assert_eq!(center_position_count(), CENTER_POSITION_COUNT);

        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                let (x, y) = cassette_position_center(col, row);
                assert!(x > cassette_left_edge());
                assert!(x < cassette_right_edge());
                assert!(y > cassette_front_edge());
                assert!(y < cassette_rear_edge());
            }
        }
    }

    #[test]
    fn rack_logger_and_token_maps_cover_equivalence_records() {
        assert_eq!(RACK_SLOT_COUNT, 6);
        assert_eq!(
            RACK_EDGE_SLOT_COUNT + RACK_CENTER_SLOT_COUNT,
            RACK_SLOT_COUNT
        );
        assert_eq!(LoggerKind::all().len(), LOGGER_POCKET_COUNT);
        assert_eq!(
            POSITION_TOKEN_COUNT,
            CASSETTE_POSITION_COUNT + RACK_SLOT_COUNT + LOGGER_POCKET_COUNT
        );
        assert!(
            token_center(POSITION_TOKEN_COUNT - 1).0 < TOKEN_BOARD_CENTER_X + TOKEN_BOARD_X / 2.0
        );
        assert!(LOGGER_CABLE_SLOT_W >= 8.0);
    }

    #[test]
    fn disposition_and_segregation_capacity_is_sized_for_twenty_positions() {
        assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
        assert_eq!(total_lane_capacity(), CASSETTE_POSITION_COUNT);
        assert_eq!(DispositionLane::Release.capacity(), 8);
        assert_eq!(DispositionLane::Hold.capacity(), 8);
        assert_eq!(DispositionLane::Reject.capacity(), 4);
        assert!(clean_used_gap() >= CLEAN_USED_GAP);
        assert!(!rects_overlap(clean_rect(), used_rect()));
    }

    #[test]
    fn evidence_bridge_and_keepouts_clear_surrogate_handling() {
        assert!(CAMERA_CLEARANCE_Z >= 120.0);
        assert!(ROBOT_PICK_CLEARANCE_Z > CASSETTE_SURROGATE_Z + REVC_TOTAL_HEIGHT + 60.0);
        assert!(SERVICE_LOGGER_PULL_X > LOGGER_POCKET_X + 40.0);
        assert!(ROBOT_SIDE_KEEP_OUT_X >= 90.0);
        assert!(SERVICE_REAR_KEEP_OUT_Y >= 170.0);
    }

    #[test]
    fn output_manifest_lists_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS[0].ends_with("_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }
}
