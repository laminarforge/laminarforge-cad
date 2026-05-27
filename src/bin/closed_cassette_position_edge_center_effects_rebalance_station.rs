use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette position edge/center effects rebalance station.
//
// Design assumptions from the research scan:
// - Cell-culture plate edge effects are treated as a coupled evaporation,
//   humidity, gas, and thermal boundary problem, so every cassette position has
//   mass-loss evidence and edge/center coupon witnesses.
// - Position-bias mitigation uses randomized or blocked layouts; this fixture
//   uses a deterministic re-slotting permutation that moves all six center
//   homes to edge positions and six edge homes into center positions.
// - Tissue-chip reproducibility work increasingly logs oxygen beside CO2,
//   temperature, and RH, so the logger dock is split into four dedicated
//   sensor pockets instead of a generic environmental logger bay.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_position_edge_center_effects_rebalance_station_deck.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_cassette_surrogate_grid.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_edge_center_environmental_coupons.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_dummy_load_equalizers.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_randomized_reslotting_token_rail.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_rh_temp_co2_o2_logger_docks.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_evaporation_mass_pads.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_flow_thermal_equalizer_witnesses.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_custody_lands.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_position_edge_center_effects_rebalance_station_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;
const REBALANCE_ASSIGNMENT: [usize; POSITION_COUNT] = [
    5, 12, 6, 15, 10, 0, 17, 13, 14, 2, 19, 9, 1, 8, 3, 18, 7, 11, 4, 16,
];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 960.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 30.0;
const MOUNT_HOLE_D: f64 = 6.6;
const REGISTRATION_RECESS_DEPTH: f64 = 5.0;

const GRID_CENTER: (f64, f64) = (-120.0, 120.0);
const CHIP_GUTTER: f64 = 6.0;
const GRID_MARGIN_X: f64 = 34.0;
const GRID_MARGIN_Y: f64 = 34.0;
const GRID_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER;
const GRID_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER;
const GRID_X: f64 = GRID_ARRAY_X + 2.0 * GRID_MARGIN_X;
const GRID_Y: f64 = GRID_ARRAY_Y + 2.0 * GRID_MARGIN_Y;
const GRID_Z: f64 = 40.0;
const GRID_SLOT_X: f64 = REVC_CHIP_LENGTH + 8.0;
const GRID_SLOT_Y: f64 = REVC_CHIP_WIDTH + 8.0;
const GRID_RECESS_DEPTH: f64 = 15.0;
const GRID_RAIL_W: f64 = 14.0;
const GRID_RAIL_Z: f64 = 24.0;
const EDGE_MARKER_D: f64 = 16.0;
const CENTER_MARKER_D: f64 = 23.0;

const COUPON_CENTER: (f64, f64) = (-575.0, 225.0);
const COUPON_PANEL_X: f64 = 260.0;
const COUPON_PANEL_Y: f64 = 310.0;
const COUPON_PANEL_Z: f64 = 18.0;
const COUPON_EDGE_D: f64 = 17.0;
const COUPON_CENTER_D: f64 = 23.0;
const COUPON_Z: f64 = 5.0;
const COUPON_COL_PITCH: f64 = 32.0;
const COUPON_ROW_PITCH: f64 = 42.0;

const DUMMY_CENTER: (f64, f64) = (-575.0, -130.0);
const DUMMY_TRAY_X: f64 = 268.0;
const DUMMY_TRAY_Y: f64 = 260.0;
const DUMMY_TRAY_Z: f64 = 30.0;
const DUMMY_POCKET_D: f64 = 24.0;
const DUMMY_EDGE_RIM_D: f64 = 34.0;
const DUMMY_CENTER_RIM_D: f64 = 28.0;
const DUMMY_PITCH_X: f64 = 56.0;
const DUMMY_PITCH_Y: f64 = 42.0;
const DUMMY_RECESS_DEPTH: f64 = 10.0;

const TOKEN_CENTER: (f64, f64) = (500.0, 260.0);
const TOKEN_PANEL_X: f64 = 440.0;
const TOKEN_PANEL_Y: f64 = 200.0;
const TOKEN_PANEL_Z: f64 = 24.0;
const TOKEN_SLOT_X: f64 = 34.0;
const TOKEN_SLOT_Y: f64 = 20.0;
const TOKEN_PITCH_X: f64 = 40.0;
const TOKEN_PITCH_Y: f64 = 46.0;
const TOKEN_STOP_D: f64 = 8.0;

const LOGGER_CENTER: (f64, f64) = (505.0, 20.0);
const LOGGER_PANEL_X: f64 = 390.0;
const LOGGER_PANEL_Y: f64 = 210.0;
const LOGGER_PANEL_Z: f64 = 32.0;
const LOGGER_POCKET_COUNT: usize = 4;
const LOGGER_POCKET_X: f64 = 76.0;
const LOGGER_POCKET_Y: f64 = 50.0;
const LOGGER_RECESS_DEPTH: f64 = 13.0;
const LOGGER_PITCH_X: f64 = 92.0;
const LOGGER_PITCH_Y: f64 = 76.0;

const MASS_CENTER: (f64, f64) = (500.0, -220.0);
const MASS_PANEL_X: f64 = 390.0;
const MASS_PANEL_Y: f64 = 230.0;
const MASS_PANEL_Z: f64 = 16.0;
const MASS_PAD_X: f64 = 36.0;
const MASS_PAD_Y: f64 = 22.0;
const MASS_PAD_Z: f64 = 5.0;
const MASS_PAD_PITCH_X: f64 = 50.0;
const MASS_PAD_PITCH_Y: f64 = 36.0;
const MASS_WELLS_PER_POSITION: usize = 2;
const MASS_WELL_D: f64 = 8.0;
const MASS_WELL_DEPTH: f64 = 4.0;

const WITNESS_CENTER: (f64, f64) = (-120.0, -325.0);
const WITNESS_PANEL_X: f64 = 610.0;
const WITNESS_PANEL_Y: f64 = 170.0;
const WITNESS_PANEL_Z: f64 = 18.0;
const EQUALIZER_PAIR_COUNT: usize = 8;
const FLOW_WITNESS_X: f64 = 58.0;
const FLOW_WITNESS_Y: f64 = 14.0;
const THERMAL_WITNESS_D: f64 = 18.0;
const WITNESS_PITCH_X: f64 = 68.0;

const CUSTODY_CENTER: (f64, f64) = (-575.0, -360.0);
const CUSTODY_PANEL_X: f64 = 280.0;
const CUSTODY_PANEL_Y: f64 = 170.0;
const CUSTODY_PANEL_Z: f64 = 16.0;
const POSITION_BARCODE_COUNT: usize = POSITION_COUNT;
const LOGGER_BARCODE_COUNT: usize = LOGGER_POCKET_COUNT;
const STUDY_CARD_COUNT: usize = 4;
const BARCODE_LAND_X: f64 = 44.0;
const BARCODE_LAND_Y: f64 = 14.0;
const BARCODE_LAND_Z: f64 = 3.0;

const LANE_CENTER: (f64, f64) = (500.0, -395.0);
const LANE_COUNT: usize = 3;
const LANE_PANEL_X: f64 = 430.0;
const LANE_PANEL_Y: f64 = 120.0;
const LANE_PANEL_Z: f64 = 28.0;
const LANE_X: f64 = 122.0;
const LANE_Y: f64 = 82.0;
const LANE_WALL_W: f64 = 8.0;
const LANE_PITCH_X: f64 = 140.0;
const RELEASE_CAPACITY: usize = 8;
const HOLD_CAPACITY: usize = 8;
const REJECT_CAPACITY: usize = 4;

const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_FRONT_KEEP_OUT_Y: f64 = 118.0;
const ROBOT_REAR_KEEP_OUT_Y: f64 = 96.0;
const SERVICE_SIDE_KEEP_OUT_X: f64 = 126.0;
const SERVICE_CLEARANCE_Z: f64 = 132.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoggerKind {
    Rh,
    Temperature,
    Co2,
    O2,
}

impl LoggerKind {
    fn all() -> [LoggerKind; LOGGER_POCKET_COUNT] {
        [
            LoggerKind::Rh,
            LoggerKind::Temperature,
            LoggerKind::Co2,
            LoggerKind::O2,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            LoggerKind::Rh => "rh",
            LoggerKind::Temperature => "temperature",
            LoggerKind::Co2 => "co2",
            LoggerKind::O2 => "o2",
        }
    }

    fn index(self) -> usize {
        match self {
            LoggerKind::Rh => 0,
            LoggerKind::Temperature => 1,
            LoggerKind::Co2 => 2,
            LoggerKind::O2 => 3,
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
    fn all() -> [DispositionLane; LANE_COUNT] {
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
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
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
    assert_layout();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let grid = cassette_surrogate_grid();
    export(OUTPUTS[1], &grid);

    let coupons = edge_center_environmental_coupons();
    export(OUTPUTS[2], &coupons);

    let dummy = dummy_load_equalizers();
    export(OUTPUTS[3], &dummy);

    let tokens = randomized_reslotting_token_rail();
    export(OUTPUTS[4], &tokens);

    let loggers = rh_temp_co2_o2_logger_docks();
    export(OUTPUTS[5], &loggers);

    let mass = evaporation_mass_pads();
    export(OUTPUTS[6], &mass);

    let witnesses = flow_thermal_equalizer_witnesses();
    export(OUTPUTS[7], &witnesses);

    let custody = custody_lands();
    export(OUTPUTS[8], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette position edge/center effects rebalance station:");
    println!(
        "  Cassette surrogate:      {CASSETTE_COLS}x{CASSETTE_ROWS} grid, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center positions"
    );
    println!(
        "  Re-slotting controls:    {POSITION_COUNT} deterministic tokens, {} edge/center zone changes",
        assignment_cross_zone_count()
    );
    println!(
        "  Environmental evidence:  {LOGGER_POCKET_COUNT} logger docks for RH, temperature, CO2, and O2 plus {EDGE_POSITION_COUNT}/{CENTER_POSITION_COUNT} coupon split"
    );
    println!(
        "  Load/evaporation checks: {POSITION_COUNT} dummy load equalizers and {} evaporation wells",
        total_mass_well_count()
    );
    println!(
        "  Equalizer witnesses:     {EQUALIZER_PAIR_COUNT} flow strips and {EQUALIZER_PAIR_COUNT} thermal coupons"
    );
    println!(
        "  Disposition capacity:    release/hold/reject lanes cover {} tokens",
        total_lane_capacity()
    );
    println!("  STL outputs:             {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    station_deck()
        + cassette_surrogate_grid()
        + edge_center_environmental_coupons()
        + dummy_load_equalizers()
        + randomized_reslotting_token_rail()
        + rh_temp_co2_o2_logger_docks()
        + evaporation_mass_pads()
        + flow_thermal_equalizer_witnesses()
        + custody_lands()
        + release_hold_reject_lanes()
        + robot_service_keepout_gauges()
}

fn assert_layout() {
    assert_eq!(POSITION_COUNT, 20);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(CENTER_POSITION_COUNT, POSITION_COUNT - EDGE_POSITION_COUNT);
    assert_eq!(LoggerKind::all().len(), LOGGER_POCKET_COUNT);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(total_lane_capacity(), POSITION_COUNT);
    assert_eq!(
        total_mass_well_count(),
        POSITION_COUNT * MASS_WELLS_PER_POSITION
    );
    assert!(is_assignment_permutation());
    assert_eq!(assignment_cross_zone_count(), 12);
    assert!(GRID_Z > REVC_TOTAL_HEIGHT + 20.0);
    assert!(SERVICE_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 90.0);
    assert!(module_footprints_are_inside_deck());
    assert!(!critical_modules_overlap());
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "edge_center_rebalance_station_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - deck_registration_recesses()
        + deck_perimeter_rims()
        + deck_flow_direction_ribs()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("edge_center_rebalance_station_deck_mount_holes");
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
                format!("edge_center_rebalance_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                36,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_registration_recesses() -> Part {
    let mut recesses = Part::empty("edge_center_rebalance_station_registration_recesses");
    for footprint in process_footprints() {
        recesses = recesses
            + centered_cube(
                format!(
                    "edge_center_rebalance_station_{}_registration_recess",
                    footprint.name
                ),
                footprint.x + 18.0,
                footprint.y + 18.0,
                REGISTRATION_RECESS_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                DECK_Z - REGISTRATION_RECESS_DEPTH / 2.0 + 0.2,
            );
    }
    recesses
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        "edge_center_rebalance_station_front_wipe_rim",
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
        "edge_center_rebalance_station_rear_wipe_rim",
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
        "edge_center_rebalance_station_left_wipe_rim",
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
        "edge_center_rebalance_station_right_wipe_rim",
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

fn deck_flow_direction_ribs() -> Part {
    let mut ribs = Part::empty("edge_center_rebalance_station_process_flow_direction_ribs");
    for (index, (x, y)) in [(-410.0, -36.0), (188.0, -42.0), (500.0, -105.0)]
        .into_iter()
        .enumerate()
    {
        let shaft = centered_cube(
            format!("edge_center_rebalance_station_flow_rib_{index}_shaft"),
            86.0,
            7.0,
            7.0,
        )
        .translate(x, y, DECK_Z + 3.5);
        let head = centered_cube(
            format!("edge_center_rebalance_station_flow_rib_{index}_head"),
            18.0,
            22.0,
            7.0,
        )
        .translate(x + 52.0, y, DECK_Z + 3.5);
        ribs = ribs + shaft + head;
    }
    ribs
}

fn cassette_surrogate_grid() -> Part {
    let base = centered_cube(
        "edge_center_rebalance_4x5_cassette_surrogate_grid_base",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let mut reliefs = Part::empty("edge_center_rebalance_position_slot_reliefs");
    let mut slot_features = Part::empty("edge_center_rebalance_position_slot_features");

    for position in 0..POSITION_COUNT {
        let (x, y) = grid_local_position(position);
        let zone = position_zone_label(position);
        reliefs = reliefs
            + centered_cube(
                format!("edge_center_rebalance_position_{position:02}_{zone}_chip_relief"),
                GRID_SLOT_X,
                GRID_SLOT_Y,
                GRID_RECESS_DEPTH + 1.0,
            )
            .translate(x, y, GRID_Z / 2.0 - GRID_RECESS_DEPTH / 2.0 + 0.5);

        let balance_marker_d = if is_edge_position(position) {
            EDGE_MARKER_D
        } else {
            CENTER_MARKER_D
        };
        let marker = centered_cylinder(
            format!("edge_center_rebalance_position_{position:02}_{zone}_balance_marker"),
            balance_marker_d / 2.0,
            5.0,
            36,
        )
        .translate(
            x - GRID_SLOT_X / 2.0 + 18.0,
            y + GRID_SLOT_Y / 2.0 - 16.0,
            GRID_Z / 2.0 + 2.5,
        );
        let front_land = centered_cube(
            format!("edge_center_rebalance_position_{position:02}_{zone}_front_identity_land"),
            GRID_SLOT_X * 0.42,
            7.0,
            8.0,
        )
        .translate(x, y - GRID_SLOT_Y / 2.0 - 8.0, GRID_Z / 2.0 + 4.0);
        slot_features = slot_features + marker + front_land;
    }

    let left_rail = centered_cube(
        "edge_center_rebalance_grid_left_datum_rail",
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
        "edge_center_rebalance_grid_rear_datum_rail",
        GRID_X,
        GRID_RAIL_W,
        GRID_RAIL_Z,
    )
    .translate(
        0.0,
        GRID_Y / 2.0 - GRID_RAIL_W / 2.0,
        GRID_Z / 2.0 + GRID_RAIL_Z / 2.0,
    );
    let center_band = centered_cube(
        "edge_center_rebalance_grid_center_position_band",
        REVC_CHIP_LENGTH * 2.0 + CHIP_GUTTER,
        REVC_CHIP_WIDTH * 3.0 + CHIP_GUTTER * 2.0,
        3.0,
    )
    .translate(0.0, 0.0, GRID_Z / 2.0 + 1.5);

    (base - reliefs + slot_features + left_rail + rear_rail + center_band).translate(
        GRID_CENTER.0,
        GRID_CENTER.1,
        DECK_Z + GRID_Z / 2.0,
    )
}

fn edge_center_environmental_coupons() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_environmental_coupon_panel",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );
    let mut coupons = Part::empty("edge_center_rebalance_environmental_coupons");

    for index in 0..EDGE_POSITION_COUNT {
        let (x, y) = edge_coupon_position(index);
        coupons = coupons + environmental_coupon(index, true).translate(x, y, coupon_top_z());
    }
    for index in 0..CENTER_POSITION_COUNT {
        let (x, y) = center_coupon_position(index);
        coupons = coupons + environmental_coupon(index, false).translate(x, y, coupon_top_z());
    }

    let divider = centered_cube(
        "edge_center_rebalance_coupon_edge_center_divider",
        COUPON_PANEL_X - 30.0,
        5.0,
        9.0,
    )
    .translate(0.0, 28.0, COUPON_PANEL_Z / 2.0 + 4.5);
    let humidity_gutter = centered_cube(
        "edge_center_rebalance_coupon_humidity_equalization_gutter",
        COUPON_PANEL_X - 40.0,
        10.0,
        5.0,
    )
    .translate(
        0.0,
        -COUPON_PANEL_Y / 2.0 + 28.0,
        COUPON_PANEL_Z / 2.0 + 2.5,
    );

    (panel + coupons + divider + humidity_gutter).translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        DECK_Z + COUPON_PANEL_Z / 2.0,
    )
}

fn environmental_coupon(index: usize, edge: bool) -> Part {
    let label = if edge { "edge" } else { "center" };
    let radius = if edge {
        COUPON_EDGE_D / 2.0
    } else {
        COUPON_CENTER_D / 2.0
    };
    let puck = centered_cylinder(
        format!("edge_center_rebalance_{label}_environmental_coupon_{index}"),
        radius,
        COUPON_Z,
        36,
    );
    let orientation_tick = centered_cube(
        format!("edge_center_rebalance_{label}_coupon_orientation_tick_{index}"),
        3.0,
        radius * 1.35,
        2.0,
    )
    .translate(radius * 0.45, 0.0, COUPON_Z / 2.0);
    puck + orientation_tick
}

fn dummy_load_equalizers() -> Part {
    let tray = centered_cube(
        "edge_center_rebalance_dummy_load_equalizer_tray",
        DUMMY_TRAY_X,
        DUMMY_TRAY_Y,
        DUMMY_TRAY_Z,
    );
    let mut reliefs = Part::empty("edge_center_rebalance_dummy_load_pocket_reliefs");
    let mut rims = Part::empty("edge_center_rebalance_dummy_load_equalizer_rims");

    for position in 0..POSITION_COUNT {
        let (x, y) = dummy_load_position(position);
        let zone = position_zone_label(position);
        reliefs = reliefs
            + centered_cylinder(
                format!("edge_center_rebalance_position_{position:02}_{zone}_dummy_load_relief"),
                DUMMY_POCKET_D / 2.0,
                DUMMY_RECESS_DEPTH,
                36,
            )
            .translate(x, y, DUMMY_TRAY_Z / 2.0 - DUMMY_RECESS_DEPTH / 2.0 + 0.4);

        let rim_d = if is_edge_position(position) {
            DUMMY_EDGE_RIM_D
        } else {
            DUMMY_CENTER_RIM_D
        };
        rims = rims
            + centered_cylinder(
                format!("edge_center_rebalance_position_{position:02}_{zone}_dummy_load_rim"),
                rim_d / 2.0,
                4.0,
                36,
            )
            .translate(x, y, DUMMY_TRAY_Z / 2.0 + 2.0);
    }

    let balance_spine = centered_cube(
        "edge_center_rebalance_dummy_load_balance_spine",
        DUMMY_TRAY_X - 32.0,
        8.0,
        10.0,
    )
    .translate(0.0, 0.0, DUMMY_TRAY_Z / 2.0 + 5.0);

    (tray - reliefs + rims + balance_spine).translate(
        DUMMY_CENTER.0,
        DUMMY_CENTER.1,
        DECK_Z + DUMMY_TRAY_Z / 2.0,
    )
}

fn randomized_reslotting_token_rail() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_randomized_reslotting_token_rail_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    );
    let mut slot_reliefs = Part::empty("edge_center_rebalance_reslotting_token_slot_reliefs");
    let mut stops = Part::empty("edge_center_rebalance_reslotting_token_stops");

    for home in 0..POSITION_COUNT {
        let assigned = REBALANCE_ASSIGNMENT[home];
        let (x, y) = token_position(home);
        let transition = assignment_transition_label(home, assigned);
        slot_reliefs = slot_reliefs
            + centered_cube(
                format!(
                    "edge_center_rebalance_home_{home:02}_assigned_{assigned:02}_{transition}_token_slot"
                ),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                9.0,
            )
            .translate(x, y, TOKEN_PANEL_Z / 2.0 - 3.0);
        stops = stops
            + centered_cylinder(
                format!(
                    "edge_center_rebalance_home_{home:02}_assigned_{assigned:02}_{transition}_stop"
                ),
                TOKEN_STOP_D / 2.0,
                5.0,
                24,
            )
            .translate(x + TOKEN_SLOT_X / 2.0 + 8.0, y, TOKEN_PANEL_Z / 2.0 + 2.5);
    }

    let edge_to_center_rail = transition_rail("edge_to_center", 35.0);
    let center_to_edge_rail = transition_rail("center_to_edge", -35.0);

    (panel - slot_reliefs + stops + edge_to_center_rail + center_to_edge_rail).translate(
        TOKEN_CENTER.0,
        TOKEN_CENTER.1,
        DECK_Z + TOKEN_PANEL_Z / 2.0,
    )
}

fn transition_rail(label: &str, y: f64) -> Part {
    centered_cube(
        format!("edge_center_rebalance_{label}_audit_rail"),
        TOKEN_PANEL_X - 40.0,
        7.0,
        9.0,
    )
    .translate(0.0, y, TOKEN_PANEL_Z / 2.0 + 4.5)
}

fn rh_temp_co2_o2_logger_docks() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_rh_temp_co2_o2_logger_dock_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let mut docks = Part::empty("edge_center_rebalance_logger_docks");

    for kind in LoggerKind::all() {
        let (x, y) = logger_position(kind);
        docks = docks + logger_dock(kind).translate(x, y, 0.0);
    }

    let cable_comb = logger_cable_comb();
    (panel + docks + cable_comb).translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1,
        DECK_Z + LOGGER_PANEL_Z / 2.0,
    )
}

fn logger_dock(kind: LoggerKind) -> Part {
    let label = kind.label();
    let pocket = centered_cube(
        format!("edge_center_rebalance_{label}_logger_pocket_body"),
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        LOGGER_PANEL_Z,
    );
    let recess = centered_cube(
        format!("edge_center_rebalance_{label}_logger_recess"),
        LOGGER_POCKET_X - 16.0,
        LOGGER_POCKET_Y - 14.0,
        LOGGER_RECESS_DEPTH + 1.0,
    )
    .translate(
        0.0,
        0.0,
        LOGGER_PANEL_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0 + 0.5,
    );
    let cable = centered_cube(
        format!("edge_center_rebalance_{label}_logger_cable_exit"),
        9.0,
        LOGGER_POCKET_Y + 4.0,
        LOGGER_RECESS_DEPTH + 2.0,
    )
    .translate(
        LOGGER_POCKET_X / 2.0 - 13.0,
        0.0,
        LOGGER_PANEL_Z / 2.0 - 2.0,
    );
    let sensor_access = match kind {
        LoggerKind::Rh => rh_louver_windows(),
        LoggerKind::Temperature => temperature_probe_cradle(),
        LoggerKind::Co2 | LoggerKind::O2 => gas_diffusion_windows(label),
    };

    pocket - recess - cable - sensor_access
}

fn rh_louver_windows() -> Part {
    let mut louvers = Part::empty("edge_center_rebalance_rh_logger_louver_windows");
    for index in 0..4 {
        louvers = louvers
            + centered_cube(
                format!("edge_center_rebalance_rh_louver_window_{index}"),
                44.0,
                2.5,
                LOGGER_RECESS_DEPTH + 2.0,
            )
            .translate(0.0, -15.0 + index as f64 * 9.0, LOGGER_PANEL_Z / 2.0);
    }
    louvers
}

fn temperature_probe_cradle() -> Part {
    let probe = centered_cylinder(
        "edge_center_rebalance_temperature_probe_cradle",
        2.4,
        58.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(4.0, 0.0, LOGGER_PANEL_Z / 2.0);
    let bead = centered_cylinder(
        "edge_center_rebalance_temperature_bead_access_cup",
        5.0,
        LOGGER_RECESS_DEPTH + 2.0,
        28,
    )
    .translate(-22.0, 0.0, LOGGER_PANEL_Z / 2.0);
    probe + bead
}

fn gas_diffusion_windows(label: &str) -> Part {
    let mut windows = Part::empty(format!(
        "edge_center_rebalance_{label}_logger_gas_diffusion_windows"
    ));
    for index in 0..3 {
        windows = windows
            + centered_cylinder(
                format!("edge_center_rebalance_{label}_diffusion_window_{index}"),
                4.2,
                LOGGER_RECESS_DEPTH + 2.0,
                28,
            )
            .translate(-18.0 + index as f64 * 18.0, -12.0, LOGGER_PANEL_Z / 2.0);
    }
    windows
}

fn logger_cable_comb() -> Part {
    let mut comb = Part::empty("edge_center_rebalance_logger_cable_comb");
    for index in 0..LOGGER_POCKET_COUNT {
        let x = centered_index(index, LOGGER_POCKET_COUNT, 58.0);
        let clamp = centered_cube(
            format!("edge_center_rebalance_logger_cable_clamp_{index}"),
            28.0,
            12.0,
            12.0,
        )
        .translate(x, LOGGER_PANEL_Y / 2.0 - 26.0, LOGGER_PANEL_Z / 2.0 + 6.0);
        let slot = centered_cylinder(
            format!("edge_center_rebalance_logger_cable_passage_{index}"),
            3.0,
            30.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LOGGER_PANEL_Y / 2.0 - 26.0, LOGGER_PANEL_Z / 2.0 + 6.0);
        comb = comb + (clamp - slot);
    }
    comb
}

fn evaporation_mass_pads() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_evaporation_mass_pad_panel",
        MASS_PANEL_X,
        MASS_PANEL_Y,
        MASS_PANEL_Z,
    );
    let mut pads = Part::empty("edge_center_rebalance_evaporation_mass_pads");

    for position in 0..POSITION_COUNT {
        let (x, y) = mass_pad_position(position);
        let zone = position_zone_label(position);
        let pad = centered_cube(
            format!("edge_center_rebalance_position_{position:02}_{zone}_evaporation_mass_pad"),
            MASS_PAD_X,
            MASS_PAD_Y,
            MASS_PAD_Z,
        )
        .translate(x, y, MASS_PANEL_Z / 2.0 + MASS_PAD_Z / 2.0);
        let wells = mass_pad_wells(position, zone).translate(x, y, MASS_PANEL_Z / 2.0);
        pads = pads + (pad - wells);
    }

    let scale_datum_left = centered_cube(
        "edge_center_rebalance_evaporation_scale_left_datum",
        8.0,
        MASS_PANEL_Y - 28.0,
        8.0,
    )
    .translate(-MASS_PANEL_X / 2.0 + 26.0, 0.0, MASS_PANEL_Z / 2.0 + 4.0);
    let scale_datum_rear = centered_cube(
        "edge_center_rebalance_evaporation_scale_rear_datum",
        MASS_PANEL_X - 42.0,
        8.0,
        8.0,
    )
    .translate(0.0, MASS_PANEL_Y / 2.0 - 26.0, MASS_PANEL_Z / 2.0 + 4.0);

    (panel + pads + scale_datum_left + scale_datum_rear).translate(
        MASS_CENTER.0,
        MASS_CENTER.1,
        DECK_Z + MASS_PANEL_Z / 2.0,
    )
}

fn mass_pad_wells(position: usize, zone: &str) -> Part {
    let mut wells = Part::empty(format!(
        "edge_center_rebalance_position_{position:02}_{zone}_mass_wells"
    ));
    for well in 0..MASS_WELLS_PER_POSITION {
        wells = wells
            + centered_cylinder(
                format!("edge_center_rebalance_position_{position:02}_{zone}_mass_well_{well}"),
                MASS_WELL_D / 2.0,
                MASS_WELL_DEPTH + 1.0,
                24,
            )
            .translate(
                centered_index(well, MASS_WELLS_PER_POSITION, 14.0),
                0.0,
                MASS_PAD_Z / 2.0,
            );
    }
    wells
}

fn flow_thermal_equalizer_witnesses() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_flow_thermal_equalizer_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let mut witnesses = Part::empty("edge_center_rebalance_flow_thermal_equalizer_witnesses");

    for pair in 0..EQUALIZER_PAIR_COUNT {
        let x = witness_pair_x(pair);
        let flow = centered_cube(
            format!("edge_center_rebalance_pair_{pair}_laminar_flow_witness_strip"),
            FLOW_WITNESS_X,
            FLOW_WITNESS_Y,
            5.0,
        )
        .translate(x, 34.0, WITNESS_PANEL_Z / 2.0 + 2.5);
        let thermal = centered_cylinder(
            format!("edge_center_rebalance_pair_{pair}_thermal_equalizer_coupon"),
            THERMAL_WITNESS_D / 2.0,
            5.0,
            36,
        )
        .translate(x, -34.0, WITNESS_PANEL_Z / 2.0 + 2.5);
        let bridge = centered_cube(
            format!("edge_center_rebalance_pair_{pair}_flow_to_thermal_bridge_land"),
            6.0,
            54.0,
            4.0,
        )
        .translate(x, 0.0, WITNESS_PANEL_Z / 2.0 + 2.0);
        witnesses = witnesses + flow + thermal + bridge;
    }

    let centerline = centered_cube(
        "edge_center_rebalance_witness_edge_center_centerline",
        WITNESS_PANEL_X - 42.0,
        5.0,
        8.0,
    )
    .translate(0.0, 0.0, WITNESS_PANEL_Z / 2.0 + 4.0);

    (panel + witnesses + centerline).translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        DECK_Z + WITNESS_PANEL_Z / 2.0,
    )
}

fn custody_lands() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_custody_land_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    let mut lands = Part::empty("edge_center_rebalance_custody_lands");

    for index in 0..POSITION_BARCODE_COUNT {
        let (x, y) = custody_position_land(index);
        lands = lands
            + centered_cube(
                format!("edge_center_rebalance_position_{index:02}_custody_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                BARCODE_LAND_Z,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + BARCODE_LAND_Z / 2.0);
    }
    for logger in 0..LOGGER_BARCODE_COUNT {
        let x = centered_index(logger, LOGGER_BARCODE_COUNT, 54.0);
        lands = lands
            + centered_cube(
                format!("edge_center_rebalance_logger_{logger}_custody_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                BARCODE_LAND_Z,
            )
            .translate(
                x,
                -CUSTODY_PANEL_Y / 2.0 + 34.0,
                CUSTODY_PANEL_Z / 2.0 + 1.5,
            );
    }
    for card in 0..STUDY_CARD_COUNT {
        let x = centered_index(card, STUDY_CARD_COUNT, 58.0);
        let seal = centered_cylinder(
            format!("edge_center_rebalance_study_card_{card}_seal_boss"),
            7.0,
            5.0,
            28,
        )
        .translate(x, CUSTODY_PANEL_Y / 2.0 - 28.0, CUSTODY_PANEL_Z / 2.0 + 2.5);
        lands = lands + seal;
    }

    (panel + lands).translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        DECK_Z + CUSTODY_PANEL_Z / 2.0,
    )
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "edge_center_rebalance_release_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut reliefs = Part::empty("edge_center_rebalance_release_hold_reject_lane_reliefs");
    let mut stops = Part::empty("edge_center_rebalance_release_hold_reject_lane_stops");

    for lane in DispositionLane::all() {
        let x = lane_x(lane);
        let pocket = centered_cube(
            format!("edge_center_rebalance_{}_lane_pocket_relief", lane.label()),
            LANE_X,
            LANE_Y,
            13.0,
        )
        .translate(x, 0.0, LANE_PANEL_Z / 2.0 - 5.0);
        let front_wall = centered_cube(
            format!("edge_center_rebalance_{}_lane_front_stop", lane.label()),
            LANE_X,
            LANE_WALL_W,
            20.0,
        )
        .translate(x, -LANE_Y / 2.0, LANE_PANEL_Z / 2.0 + 10.0);
        reliefs = reliefs + pocket;
        stops = stops + front_wall + lane_capacity_posts(lane);
    }

    (panel - reliefs + stops).translate(LANE_CENTER.0, LANE_CENTER.1, DECK_Z + LANE_PANEL_Z / 2.0)
}

fn lane_capacity_posts(lane: DispositionLane) -> Part {
    let mut posts = Part::empty(format!(
        "edge_center_rebalance_{}_lane_capacity_posts",
        lane.label()
    ));
    let x = lane_x(lane);
    for index in 0..lane.capacity() {
        let y = centered_index(index, lane.capacity(), 8.0);
        posts = posts
            + centered_cylinder(
                format!(
                    "edge_center_rebalance_{}_lane_capacity_post_{index}",
                    lane.label()
                ),
                2.4,
                5.0,
                16,
            )
            .translate(x + LANE_X / 2.0 - 12.0, y, LANE_PANEL_Z / 2.0 + 2.5);
    }
    posts
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "edge_center_rebalance_front_robot_keepout_gauge",
        DECK_X - 160.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 92.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let rear_robot = centered_cube(
        "edge_center_rebalance_rear_robot_keepout_gauge",
        DECK_X - 160.0,
        ROBOT_REAR_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 86.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let left_service = centered_cube(
        "edge_center_rebalance_left_service_keepout_gauge",
        SERVICE_SIDE_KEEP_OUT_X,
        DECK_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + 82.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let right_service = centered_cube(
        "edge_center_rebalance_right_service_keepout_gauge",
        SERVICE_SIDE_KEEP_OUT_X,
        DECK_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 - 82.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let vertical_clearance = centered_cube(
        "edge_center_rebalance_cassette_vertical_pick_clearance_gauge",
        42.0,
        GRID_Y + 82.0,
        SERVICE_CLEARANCE_Z,
    )
    .translate(
        GRID_CENTER.0 + GRID_X / 2.0 + 48.0,
        GRID_CENTER.1,
        DECK_Z + SERVICE_CLEARANCE_Z / 2.0,
    );

    front_robot + rear_robot + left_service + right_service + vertical_clearance
}

fn process_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "cassette_grid",
            center: GRID_CENTER,
            x: GRID_X,
            y: GRID_Y,
        },
        Footprint {
            name: "coupon_bank",
            center: COUPON_CENTER,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        Footprint {
            name: "dummy_loads",
            center: DUMMY_CENTER,
            x: DUMMY_TRAY_X,
            y: DUMMY_TRAY_Y,
        },
        Footprint {
            name: "token_rail",
            center: TOKEN_CENTER,
            x: TOKEN_PANEL_X,
            y: TOKEN_PANEL_Y,
        },
        Footprint {
            name: "logger_docks",
            center: LOGGER_CENTER,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Footprint {
            name: "mass_pads",
            center: MASS_CENTER,
            x: MASS_PANEL_X,
            y: MASS_PANEL_Y,
        },
        Footprint {
            name: "equalizer_witnesses",
            center: WITNESS_CENTER,
            x: WITNESS_PANEL_X,
            y: WITNESS_PANEL_Y,
        },
        Footprint {
            name: "custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Footprint {
            name: "disposition_lanes",
            center: LANE_CENTER,
            x: LANE_PANEL_X,
            y: LANE_PANEL_Y,
        },
    ]
}

fn grid_local_position(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, REVC_CHIP_LENGTH + CHIP_GUTTER),
        -centered_index(row, CASSETTE_ROWS, REVC_CHIP_WIDTH + CHIP_GUTTER),
    )
}

fn dummy_load_position(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, DUMMY_PITCH_X),
        -centered_index(row, CASSETTE_ROWS, DUMMY_PITCH_Y),
    )
}

fn token_position(home: usize) -> (f64, f64) {
    let col = home % 10;
    let row = home / 10;
    (
        centered_index(col, 10, TOKEN_PITCH_X),
        centered_index(row, 2, TOKEN_PITCH_Y),
    )
}

fn logger_position(kind: LoggerKind) -> (f64, f64) {
    let col = kind.index() % 2;
    let row = kind.index() / 2;
    (
        centered_index(col, 2, LOGGER_PITCH_X),
        -centered_index(row, 2, LOGGER_PITCH_Y),
    )
}

fn mass_pad_position(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, MASS_PAD_PITCH_X),
        -centered_index(row, CASSETTE_ROWS, MASS_PAD_PITCH_Y),
    )
}

fn edge_coupon_position(index: usize) -> (f64, f64) {
    let col = index % 7;
    let row = index / 7;
    (
        centered_index(col, 7, COUPON_COL_PITCH),
        78.0 - row as f64 * COUPON_ROW_PITCH,
    )
}

fn center_coupon_position(index: usize) -> (f64, f64) {
    (
        centered_index(index, CENTER_POSITION_COUNT, COUPON_COL_PITCH),
        -74.0,
    )
}

fn coupon_top_z() -> f64 {
    COUPON_PANEL_Z / 2.0 + COUPON_Z / 2.0
}

fn witness_pair_x(pair: usize) -> f64 {
    centered_index(pair, EQUALIZER_PAIR_COUNT, WITNESS_PITCH_X)
}

fn custody_position_land(index: usize) -> (f64, f64) {
    let col = index % 5;
    let row = index / 5;
    (centered_index(col, 5, 52.0), centered_index(row, 4, 28.0))
}

fn lane_x(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn is_edge_position(position: usize) -> bool {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1
}

fn position_zone_label(position: usize) -> &'static str {
    if is_edge_position(position) {
        "edge"
    } else {
        "center"
    }
}

fn edge_position_count() -> usize {
    (0..POSITION_COUNT)
        .filter(|position| is_edge_position(*position))
        .count()
}

fn assignment_transition_label(home: usize, assigned: usize) -> &'static str {
    match (is_edge_position(home), is_edge_position(assigned)) {
        (true, false) => "edge_to_center",
        (false, true) => "center_to_edge",
        (true, true) => "edge_to_edge",
        (false, false) => "center_to_center",
    }
}

fn assignment_cross_zone_count() -> usize {
    REBALANCE_ASSIGNMENT
        .iter()
        .enumerate()
        .filter(|(home, assigned)| is_edge_position(*home) != is_edge_position(**assigned))
        .count()
}

fn is_assignment_permutation() -> bool {
    let mut seen = [false; POSITION_COUNT];
    for assigned in REBALANCE_ASSIGNMENT {
        if assigned >= POSITION_COUNT || seen[assigned] {
            return false;
        }
        seen[assigned] = true;
    }
    seen.into_iter().all(|present| present)
}

fn total_mass_well_count() -> usize {
    POSITION_COUNT * MASS_WELLS_PER_POSITION
}

fn total_lane_capacity() -> usize {
    DispositionLane::all()
        .iter()
        .map(|lane| lane.capacity())
        .sum()
}

fn module_footprints_are_inside_deck() -> bool {
    process_footprints().into_iter().all(|footprint| {
        let rect = footprint_rect(footprint);
        rect.x - rect.w / 2.0 > -DECK_X / 2.0 + DECK_RIM_W
            && rect.x + rect.w / 2.0 < DECK_X / 2.0 - DECK_RIM_W
            && rect.y - rect.h / 2.0 > -DECK_Y / 2.0 + DECK_RIM_W
            && rect.y + rect.h / 2.0 < DECK_Y / 2.0 - DECK_RIM_W
    })
}

fn critical_modules_overlap() -> bool {
    let footprints = process_footprints();
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            if rects_overlap(
                footprint_rect(footprints[left]),
                footprint_rect(footprints[right]),
            ) {
                return true;
            }
        }
    }
    false
}

fn footprint_rect(footprint: Footprint) -> Rect {
    Rect {
        x: footprint.center.0,
        y: footprint.center.1,
        w: footprint.x,
        h: footprint.y,
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
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_station_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for output in OUTPUTS {
            assert!(output.starts_with(
                "output/closed_cassette_position_edge_center_effects_rebalance_station_"
            ));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn cassette_surrogate_has_expected_edge_center_topology() {
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(POSITION_COUNT, 20);
        assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
        assert_eq!(CENTER_POSITION_COUNT, 6);
        assert!(GRID_SLOT_X > REVC_CHIP_LENGTH);
        assert!(GRID_SLOT_Y > REVC_CHIP_WIDTH);
        assert!(GRID_Z > REVC_TOTAL_HEIGHT + 20.0);
    }

    #[test]
    fn rebalance_assignment_is_permutation_and_crosses_zones() {
        assert!(is_assignment_permutation());
        assert_eq!(assignment_cross_zone_count(), 12);
        let center_to_edge = REBALANCE_ASSIGNMENT
            .iter()
            .enumerate()
            .filter(|(home, assigned)| !is_edge_position(*home) && is_edge_position(**assigned))
            .count();
        let edge_to_center = REBALANCE_ASSIGNMENT
            .iter()
            .enumerate()
            .filter(|(home, assigned)| is_edge_position(*home) && !is_edge_position(**assigned))
            .count();
        assert_eq!(center_to_edge, CENTER_POSITION_COUNT);
        assert_eq!(edge_to_center, CENTER_POSITION_COUNT);
    }

    #[test]
    fn environmental_and_evaporation_evidence_counts_match_positions() {
        assert_eq!(LoggerKind::all().len(), LOGGER_POCKET_COUNT);
        assert_eq!(LOGGER_POCKET_COUNT, 4);
        assert_eq!(total_mass_well_count(), 40);
        assert_eq!(POSITION_BARCODE_COUNT, POSITION_COUNT);
        assert_eq!(LOGGER_BARCODE_COUNT, LOGGER_POCKET_COUNT);
        assert_eq!(EDGE_POSITION_COUNT + CENTER_POSITION_COUNT, POSITION_COUNT);
    }

    #[test]
    fn dummy_loads_and_equalizer_witnesses_have_balanced_capacity() {
        assert!(DUMMY_TRAY_X > 0.0);
        assert_eq!(EQUALIZER_PAIR_COUNT, 8);
        assert!(DUMMY_EDGE_RIM_D > DUMMY_CENTER_RIM_D);
        assert!(WITNESS_PANEL_X > EQUALIZER_PAIR_COUNT as f64 * WITNESS_PITCH_X);
        assert!(MASS_PANEL_X > CASSETTE_COLS as f64 * MASS_PAD_PITCH_X);
    }

    #[test]
    fn disposition_and_layout_clearances_are_valid() {
        assert_eq!(DispositionLane::all().len(), LANE_COUNT);
        assert_eq!(total_lane_capacity(), POSITION_COUNT);
        assert_eq!(DispositionLane::Release.capacity(), 8);
        assert_eq!(DispositionLane::Hold.capacity(), 8);
        assert_eq!(DispositionLane::Reject.capacity(), 4);
        assert!(module_footprints_are_inside_deck());
        assert!(!critical_modules_overlap());
        assert!(SERVICE_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 90.0);
    }
}
