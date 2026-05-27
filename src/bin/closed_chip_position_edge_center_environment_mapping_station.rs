use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed chip position edge/center environment mapping station.
//
// This no-cell validation fixture maps whether a scaled chip carrier creates
// position-dependent temperature, humidity, CO2, O2, evaporation, and flow-shadow
// differences before live tissue-chip runs. The modeled station uses a 4x4 chip
// array because the current automation direction is a 16-position board, with
// edge/center tokens, swappable sensor surrogate chips, dummy thermal masses,
// witness coupons, and evidence capture features. Acceptance limits, sensor
// calibration, batch statistics, and biological release rules remain protocol
// controls outside this CAD generator.

const OUTPUT_PREFIX: &str = "closed_chip_position_edge_center_environment_mapping_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_chip_position_edge_center_environment_mapping_station_base_containment_deck.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_sixteen_chip_map_nest.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_swappable_sensor_chip_surrogates.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_edge_center_witness_coupon_bank.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_evaporation_mass_balance_pads.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_flow_shadow_and_thermal_dummy_loads.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_logger_calibration_dock_array.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_position_map_token_board.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_release_hold_reject_lanes.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_evidence_camera_bridge.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_robot_service_keepout_gauges.stl",
    "output/closed_chip_position_edge_center_environment_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sixteen_chip_map_nest",
    "swappable_sensor_chip_surrogates",
    "edge_center_witness_coupon_bank",
    "evaporation_mass_balance_pads",
    "flow_shadow_and_thermal_dummy_loads",
    "logger_calibration_dock_array",
    "position_map_token_board",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 4;
const CHIP_POSITION_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const EDGE_POSITION_COUNT: usize = 12;
const CENTER_POSITION_COUNT: usize = CHIP_POSITION_COUNT - EDGE_POSITION_COUNT;
const SENSOR_KIND_COUNT: usize = 6;
const LOGGER_DOCK_COUNT: usize = 6;
const LANE_COUNT: usize = 3;

const DECK_X: f64 = 1860.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DATUM_TARGET_COUNT: usize = 4;

const MAP_NEST_POS: (f64, f64) = (-255.0, 135.0);
const CHIP_GUTTER: f64 = 9.0;
const NEST_MARGIN_X: f64 = 38.0;
const NEST_MARGIN_Y: f64 = 36.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const MAP_NEST_X: f64 = CHIP_ARRAY_X + 2.0 * NEST_MARGIN_X;
const MAP_NEST_Y: f64 = CHIP_ARRAY_Y + 2.0 * NEST_MARGIN_Y;
const MAP_NEST_Z: f64 = 38.0;
const CHIP_RECESS_X: f64 = REVC_CHIP_LENGTH + 8.0;
const CHIP_RECESS_Y: f64 = REVC_CHIP_WIDTH + 8.0;
const CHIP_RECESS_DEPTH: f64 = 13.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 24.0;
const EDGE_MARKER_D: f64 = 17.0;
const CENTER_MARKER_D: f64 = 23.0;

const SENSOR_POS: (f64, f64) = (-255.0, -250.0);
const SENSOR_TRAY_X: f64 = 720.0;
const SENSOR_TRAY_Y: f64 = 205.0;
const SENSOR_TRAY_Z: f64 = 24.0;
const SENSOR_CARD_X: f64 = 82.0;
const SENSOR_CARD_Y: f64 = 58.0;
const SENSOR_CARD_Z: f64 = 10.0;
const SENSOR_CARD_PITCH_X: f64 = 92.0;
const SENSOR_EDGE_COUNT: usize = EDGE_POSITION_COUNT;
const SENSOR_CENTER_COUNT: usize = CENTER_POSITION_COUNT;

const COUPON_POS: (f64, f64) = (-710.0, 170.0);
const COUPON_BANK_X: f64 = 230.0;
const COUPON_BANK_Y: f64 = 320.0;
const COUPON_BANK_Z: f64 = 18.0;
const EDGE_COUPON_D: f64 = 17.0;
const CENTER_COUPON_D: f64 = 23.0;
const COUPON_Z: f64 = 5.0;
const COUPON_COL_PITCH: f64 = 45.0;
const COUPON_ROW_PITCH: f64 = 38.0;

const MASS_POS: (f64, f64) = (375.0, 260.0);
const MASS_PANEL_X: f64 = 410.0;
const MASS_PANEL_Y: f64 = 215.0;
const MASS_PANEL_Z: f64 = 16.0;
const MASS_PAD_X: f64 = 42.0;
const MASS_PAD_Y: f64 = 26.0;
const MASS_PAD_Z: f64 = 5.0;
const MASS_PAD_PITCH_X: f64 = 52.0;
const MASS_PAD_PITCH_Y: f64 = 42.0;
const MASS_WELLS_PER_POSITION: usize = 2;
const MASS_WELL_D: f64 = 8.0;
const MASS_WELL_DEPTH: f64 = 4.0;

const SHADOW_POS: (f64, f64) = (385.0, 30.0);
const SHADOW_PANEL_X: f64 = 430.0;
const SHADOW_PANEL_Y: f64 = 210.0;
const SHADOW_PANEL_Z: f64 = 26.0;
const FLOW_RIBBON_COUNT: usize = 8;
const FLOW_RIBBON_X: f64 = 168.0;
const FLOW_RIBBON_Y: f64 = 10.0;
const FLOW_RIBBON_Z: f64 = 5.0;
const THERMAL_LOAD_COUNT: usize = 8;
const THERMAL_LOAD_D: f64 = 24.0;
const THERMAL_LOAD_Z: f64 = 11.0;

const LOGGER_POS: (f64, f64) = (395.0, -205.0);
const LOGGER_PANEL_X: f64 = 420.0;
const LOGGER_PANEL_Y: f64 = 185.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_DOCK_X: f64 = 82.0;
const LOGGER_DOCK_Y: f64 = 50.0;
const LOGGER_DOCK_Z: f64 = 18.0;
const LOGGER_DOCK_PITCH_X: f64 = 122.0;
const LOGGER_DOCK_PITCH_Y: f64 = 72.0;

const TOKEN_POS: (f64, f64) = (-750.0, -168.0);
const TOKEN_BOARD_X: f64 = 245.0;
const TOKEN_BOARD_Y: f64 = 245.0;
const TOKEN_BOARD_Z: f64 = 14.0;
const POSITION_TOKEN_D: f64 = 16.0;
const TOKEN_BARCODE_X: f64 = 44.0;
const TOKEN_BARCODE_Y: f64 = 13.0;
const TOKEN_BARCODE_Z: f64 = 3.0;

const LANE_POS: (f64, f64) = (395.0, -375.0);
const LANE_PANEL_X: f64 = 430.0;
const LANE_PANEL_Y: f64 = 110.0;
const LANE_PANEL_Z: f64 = 28.0;
const LANE_SLOT_X: f64 = 116.0;
const LANE_SLOT_Y: f64 = 70.0;
const LANE_WALL_W: f64 = 8.0;
const LANE_PITCH_X: f64 = 138.0;
const RELEASE_CAPACITY: usize = 8;
const HOLD_CAPACITY: usize = 4;
const REJECT_CAPACITY: usize = 4;

const CAMERA_POS: (f64, f64) = (-40.0, 405.0);
const CAMERA_BRIDGE_X: f64 = 1240.0;
const CAMERA_BRIDGE_Y: f64 = 70.0;
const CAMERA_POST_W: f64 = 28.0;
const CAMERA_POST_Z: f64 = 190.0;
const CAMERA_CROSSBAR_Z: f64 = 24.0;
const CAMERA_CARRIAGE_X: f64 = 140.0;
const CAMERA_CARRIAGE_Y: f64 = 84.0;
const CAMERA_CARRIAGE_Z: f64 = 18.0;
const CAMERA_TARGET_COUNT: usize = 5;
const CAMERA_CLEARANCE_Z: f64 = 160.0;

const KEEP_OUT_X: f64 = 1460.0;
const KEEP_OUT_Y: f64 = 870.0;
const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_FRONT_CLEARANCE: f64 = 390.0;
const SERVICE_REAR_CLEARANCE: f64 = 180.0;
const SERVICE_SIDE_CLEARANCE: f64 = 116.0;
const ROBOT_Z_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChipZone {
    Edge,
    Center,
}

impl ChipZone {
    fn label(self) -> &'static str {
        match self {
            ChipZone::Edge => "edge",
            ChipZone::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SensorKind {
    Temperature,
    Co2,
    O2,
    Rh,
    Dewpoint,
    Pressure,
}

impl SensorKind {
    fn all() -> [SensorKind; SENSOR_KIND_COUNT] {
        [
            SensorKind::Temperature,
            SensorKind::Co2,
            SensorKind::O2,
            SensorKind::Rh,
            SensorKind::Dewpoint,
            SensorKind::Pressure,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            SensorKind::Temperature => "temperature",
            SensorKind::Co2 => "co2",
            SensorKind::O2 => "o2",
            SensorKind::Rh => "rh",
            SensorKind::Dewpoint => "dewpoint",
            SensorKind::Pressure => "pressure",
        }
    }

    fn index(self) -> usize {
        match self {
            SensorKind::Temperature => 0,
            SensorKind::Co2 => 1,
            SensorKind::O2 => 2,
            SensorKind::Rh => 3,
            SensorKind::Dewpoint => 4,
            SensorKind::Pressure => 5,
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
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 16.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 16.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = sixteen_chip_map_nest();
    export(OUTPUTS[1], &nest);

    let sensor_surrogates = swappable_sensor_chip_surrogates();
    export(OUTPUTS[2], &sensor_surrogates);

    let coupons = edge_center_witness_coupon_bank();
    export(OUTPUTS[3], &coupons);

    let mass = evaporation_mass_balance_pads();
    export(OUTPUTS[4], &mass);

    let loads = flow_shadow_and_thermal_dummy_loads();
    export(OUTPUTS[5], &loads);

    let loggers = logger_calibration_dock_array();
    export(OUTPUTS[6], &loggers);

    let tokens = position_map_token_board();
    export(OUTPUTS[7], &tokens);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed chip position edge/center environment mapping station:");
    println!(
        "  Chip map:                 {CHIP_COLS}x{CHIP_ROWS} board, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center positions"
    );
    println!(
        "  Sensor surrogates:        {SENSOR_EDGE_COUNT} edge cards, {SENSOR_CENTER_COUNT} center cards, {} sensor modes",
        SensorKind::all().len()
    );
    println!(
        "  Environmental evidence:   {} coupons, {} evaporation wells, {LOGGER_DOCK_COUNT} logger docks",
        EDGE_POSITION_COUNT + CENTER_POSITION_COUNT,
        total_mass_well_count()
    );
    println!(
        "  Flow/thermal challenge:   {FLOW_RIBBON_COUNT} flow-shadow ribbons and {THERMAL_LOAD_COUNT} dummy thermal loads"
    );
    println!(
        "  Disposition controls:     release/hold/reject lanes cover {} position tokens",
        total_lane_capacity()
    );
    println!(
        "  Evidence clearance:       {CAMERA_CLEARANCE_Z:.1}mm camera clearance over {:.1}mm nest height",
        DECK_Z + MAP_NEST_Z
    );
    println!("  STL outputs:              {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_containment_deck()
        + sixteen_chip_map_nest()
        + swappable_sensor_chip_surrogates()
        + edge_center_witness_coupon_bank()
        + evaporation_mass_balance_pads()
        + flow_shadow_and_thermal_dummy_loads()
        + logger_calibration_dock_array()
        + position_map_token_board()
        + release_hold_reject_lanes()
        + evidence_camera_bridge()
        + robot_service_keepout_gauges()
}

fn assert_design_constraints() {
    assert_eq!(CHIP_POSITION_COUNT, 16);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(SensorKind::all().len(), SENSOR_KIND_COUNT);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert_eq!(total_lane_capacity(), CHIP_POSITION_COUNT);
    assert_eq!(
        total_mass_well_count(),
        CHIP_POSITION_COUNT * MASS_WELLS_PER_POSITION
    );
    assert!(
        MAP_NEST_Z >= REVC_TOTAL_HEIGHT + 20.0,
        "map nest should exceed chip surrogate height with handling clearance"
    );
    assert!(
        CHIP_RECESS_X < CHIP_PITCH_X && CHIP_RECESS_Y < CHIP_PITCH_Y,
        "chip recesses overlap adjacent map positions"
    );
    assert!(
        CAMERA_CLEARANCE_Z > MAP_NEST_Z + REVC_TOTAL_HEIGHT + 80.0,
        "evidence bridge does not clear chip map nest"
    );
    assert!(
        required_feature_count() == REQUIRED_FEATURES.len(),
        "feature registry drifted from exported parts"
    );
    assert!(module_footprints_fit_deck());
    assert!(!critical_footprints_overlap());
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - module_socket_recesses()
        + deck_perimeter_rims()
        + datum_targets()
        + flow_direction_ribs()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mount_holes"));
    let points = [
        (-DECK_X / 2.0 + 46.0, -DECK_Y / 2.0 + 46.0),
        (DECK_X / 2.0 - 46.0, -DECK_Y / 2.0 + 46.0),
        (-DECK_X / 2.0 + 46.0, DECK_Y / 2.0 - 46.0),
        (DECK_X / 2.0 - 46.0, DECK_Y / 2.0 - 46.0),
        (0.0, -DECK_Y / 2.0 + 46.0),
        (0.0, DECK_Y / 2.0 - 46.0),
        (-DECK_X / 2.0 + 46.0, 0.0),
        (DECK_X / 2.0 - 46.0, 0.0),
    ];
    for (index, (x, y)) in points.into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                36,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_module_socket_recesses"));
    for rect in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{}_{}_socket_recess", OUTPUT_PREFIX, rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_spill_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_spill_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_spill_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_spill_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_deck_datum_targets"));
    let points = [
        (
            MAP_NEST_POS.0 - MAP_NEST_X / 2.0 - 26.0,
            MAP_NEST_POS.1 - MAP_NEST_Y / 2.0,
        ),
        (
            MAP_NEST_POS.0 + MAP_NEST_X / 2.0 + 26.0,
            MAP_NEST_POS.1 - MAP_NEST_Y / 2.0,
        ),
        (
            MAP_NEST_POS.0 - MAP_NEST_X / 2.0 - 26.0,
            MAP_NEST_POS.1 + MAP_NEST_Y / 2.0,
        ),
        (
            MAP_NEST_POS.0 + MAP_NEST_X / 2.0 + 26.0,
            MAP_NEST_POS.1 + MAP_NEST_Y / 2.0,
        ),
    ];
    for (index, (x, y)) in points.into_iter().enumerate() {
        targets = targets
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_optical_datum_target_{index}"),
                9.0,
                3.0,
                40,
            )
            .translate(x, y, DECK_Z + 1.5);
    }
    targets
}

fn flow_direction_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_process_flow_direction_ribs"));
    for (index, (x, y, length)) in [
        (-545.0, -38.0, 110.0),
        (-70.0, -70.0, 160.0),
        (330.0, -92.0, 110.0),
    ]
    .into_iter()
    .enumerate()
    {
        let shaft = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_direction_rib_{index}_shaft"),
            length,
            7.0,
            7.0,
        )
        .translate(x, y, DECK_Z + 3.5);
        let head = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_direction_rib_{index}_head"),
            18.0,
            22.0,
            7.0,
        )
        .translate(x + length / 2.0 + 8.0, y, DECK_Z + 3.5);
        ribs = ribs + shaft + head;
    }
    ribs
}

fn sixteen_chip_map_nest() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_sixteen_chip_map_nest_base"),
        MAP_NEST_X,
        MAP_NEST_Y,
        MAP_NEST_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_chip_position_recesses"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_chip_position_features"));

    for position in 0..CHIP_POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        let zone = chip_zone(position);
        cuts = cuts
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_chip_recess",
                    zone.label()
                ),
                CHIP_RECESS_X,
                CHIP_RECESS_Y,
                CHIP_RECESS_DEPTH + 1.0,
            )
            .translate(x, y, MAP_NEST_Z / 2.0 - CHIP_RECESS_DEPTH / 2.0 + 0.5);

        let marker_d = match zone {
            ChipZone::Edge => EDGE_MARKER_D,
            ChipZone::Center => CENTER_MARKER_D,
        };
        let marker = centered_cylinder(
            format!(
                "{OUTPUT_PREFIX}_position_{position:02}_{}_zone_marker",
                zone.label()
            ),
            marker_d / 2.0,
            4.0,
            36,
        )
        .translate(
            x - CHIP_RECESS_X / 2.0 + 18.0,
            y + CHIP_RECESS_Y / 2.0 - 18.0,
            MAP_NEST_Z / 2.0 + 2.0,
        );
        let front_land = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_identity_land"),
            CHIP_RECESS_X * 0.42,
            7.0,
            6.0,
        )
        .translate(x, y - CHIP_RECESS_Y / 2.0 - 8.0, MAP_NEST_Z / 2.0 + 3.0);
        features = features + marker + front_land;
    }

    let left_rail = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_left_hard_datum_rail"),
        NEST_RAIL_W,
        MAP_NEST_Y,
        NEST_RAIL_Z,
    )
    .translate(
        -MAP_NEST_X / 2.0 + NEST_RAIL_W / 2.0,
        0.0,
        MAP_NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let rear_rail = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_rear_hard_datum_rail"),
        MAP_NEST_X,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        MAP_NEST_Y / 2.0 - NEST_RAIL_W / 2.0,
        MAP_NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let center_zone_outline = centered_cube(
        format!("{OUTPUT_PREFIX}_center_zone_outline_frame"),
        CHIP_PITCH_X + REVC_CHIP_LENGTH,
        CHIP_PITCH_Y + REVC_CHIP_WIDTH,
        3.0,
    )
    .translate(0.0, 0.0, MAP_NEST_Z / 2.0 + 1.5);

    (base - cuts + features + left_rail + rear_rail + center_zone_outline).translate(
        MAP_NEST_POS.0,
        MAP_NEST_POS.1,
        DECK_Z + MAP_NEST_Z / 2.0,
    )
}

fn swappable_sensor_chip_surrogates() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_swappable_sensor_chip_surrogate_tray"),
        SENSOR_TRAY_X,
        SENSOR_TRAY_Y,
        SENSOR_TRAY_Z,
    );
    let mut cards = Part::empty(format!("{OUTPUT_PREFIX}_sensor_surrogate_cards"));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sensor_surrogate_recesses"));

    for position in 0..CHIP_POSITION_COUNT {
        let zone = chip_zone(position);
        let sensor = SensorKind::all()[position % SENSOR_KIND_COUNT];
        let x = -SENSOR_TRAY_X / 2.0 + 70.0 + (position % 8) as f64 * SENSOR_CARD_PITCH_X;
        let y = if position < 8 { 48.0 } else { -48.0 };
        let body = sensor_card(position, zone, sensor).translate(
            x,
            y,
            SENSOR_TRAY_Z / 2.0 + SENSOR_CARD_Z / 2.0,
        );
        let recess = centered_cube(
            format!("{OUTPUT_PREFIX}_sensor_card_{position:02}_parking_recess"),
            SENSOR_CARD_X + 8.0,
            SENSOR_CARD_Y + 8.0,
            7.0,
        )
        .translate(x, y, SENSOR_TRAY_Z - 3.2);
        cards = cards + body;
        cuts = cuts + recess;
    }

    (tray - cuts + cards + sensor_tray_edge_center_divider()).translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        DECK_Z + SENSOR_TRAY_Z / 2.0,
    )
}

fn sensor_card(position: usize, zone: ChipZone, sensor: SensorKind) -> Part {
    let label = sensor.label();
    let base = centered_cube(
        format!(
            "{OUTPUT_PREFIX}_position_{position:02}_{}_{}_surrogate_card",
            zone.label(),
            label
        ),
        SENSOR_CARD_X,
        SENSOR_CARD_Y,
        SENSOR_CARD_Z,
    );
    let sensor_cut = match sensor {
        SensorKind::Temperature => centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_temperature_probe_cup"),
            6.0,
            5.0,
            28,
        )
        .translate(-18.0, 0.0, SENSOR_CARD_Z / 2.0),
        SensorKind::Co2 => centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_co2_ndir_module_relief"),
            42.0,
            25.0,
            5.0,
        )
        .translate(0.0, 0.0, SENSOR_CARD_Z / 2.0),
        SensorKind::O2 => centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_o2_membrane_cup"),
            11.0,
            5.0,
            36,
        )
        .translate(-12.0, 0.0, SENSOR_CARD_Z / 2.0),
        SensorKind::Rh => centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_rh_louver_relief"),
            46.0,
            19.0,
            5.0,
        )
        .translate(0.0, 0.0, SENSOR_CARD_Z / 2.0),
        SensorKind::Dewpoint => centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_dewpoint_mirror_cup"),
            13.0,
            5.0,
            40,
        )
        .translate(0.0, 0.0, SENSOR_CARD_Z / 2.0),
        SensorKind::Pressure => centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_pressure_port_relief"),
            22.0,
            36.0,
            5.0,
        )
        .translate(0.0, 0.0, SENSOR_CARD_Z / 2.0),
    };
    let cable_key = centered_cube(
        format!("{OUTPUT_PREFIX}_position_{position:02}_sensor_card_cable_key"),
        9.0,
        24.0,
        5.2,
    )
    .translate(SENSOR_CARD_X / 2.0 - 10.0, 0.0, SENSOR_CARD_Z / 2.0);
    let zone_key_d = match zone {
        ChipZone::Edge => 4.5,
        ChipZone::Center => 7.0,
    };
    let zone_key = centered_cylinder(
        format!(
            "{OUTPUT_PREFIX}_position_{position:02}_{}_zone_key",
            zone.label()
        ),
        zone_key_d,
        3.0,
        24,
    )
    .translate(
        -SENSOR_CARD_X / 2.0 + 12.0,
        SENSOR_CARD_Y / 2.0 - 12.0,
        SENSOR_CARD_Z / 2.0 + 1.5,
    );

    base - sensor_cut - cable_key + zone_key
}

fn sensor_tray_edge_center_divider() -> Part {
    let long_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_sensor_tray_edge_center_divider"),
        SENSOR_TRAY_X - 40.0,
        5.0,
        11.0,
    )
    .translate(0.0, 0.0, SENSOR_TRAY_Z / 2.0 + 5.5);
    let edge_label = centered_cube(
        format!("{OUTPUT_PREFIX}_sensor_tray_edge_label_land"),
        128.0,
        14.0,
        3.0,
    )
    .translate(-250.0, 82.0, SENSOR_TRAY_Z / 2.0 + 1.5);
    let center_label = centered_cube(
        format!("{OUTPUT_PREFIX}_sensor_tray_center_label_land"),
        128.0,
        14.0,
        3.0,
    )
    .translate(-250.0, -82.0, SENSOR_TRAY_Z / 2.0 + 1.5);
    long_divider + edge_label + center_label
}

fn edge_center_witness_coupon_bank() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_center_witness_coupon_panel"),
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    );
    let mut coupons = Part::empty(format!("{OUTPUT_PREFIX}_edge_center_witness_coupons"));

    for index in 0..EDGE_POSITION_COUNT {
        let (x, y) = edge_coupon_xy(index);
        coupons = coupons
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_edge_witness_coupon_{index:02}"),
                EDGE_COUPON_D / 2.0,
                COUPON_Z,
                36,
            )
            .translate(x, y, COUPON_BANK_Z / 2.0 + COUPON_Z / 2.0);
    }
    for index in 0..CENTER_POSITION_COUNT {
        let (x, y) = center_coupon_xy(index);
        coupons = coupons
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_center_witness_coupon_{index:02}"),
                CENTER_COUPON_D / 2.0,
                COUPON_Z,
                36,
            )
            .translate(x, y, COUPON_BANK_Z / 2.0 + COUPON_Z / 2.0);
    }

    let divider = centered_cube(
        format!("{OUTPUT_PREFIX}_coupon_edge_center_divider"),
        COUPON_BANK_X - 34.0,
        5.0,
        10.0,
    )
    .translate(0.0, 42.0, COUPON_BANK_Z / 2.0 + 5.0);
    let soak_gutter = centered_cube(
        format!("{OUTPUT_PREFIX}_coupon_humidity_soak_gutter"),
        COUPON_BANK_X - 44.0,
        10.0,
        5.0,
    )
    .translate(0.0, -COUPON_BANK_Y / 2.0 + 30.0, COUPON_BANK_Z / 2.0 + 2.5);

    (panel + coupons + divider + soak_gutter).translate(
        COUPON_POS.0,
        COUPON_POS.1,
        DECK_Z + COUPON_BANK_Z / 2.0,
    )
}

fn evaporation_mass_balance_pads() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_evaporation_mass_balance_panel"),
        MASS_PANEL_X,
        MASS_PANEL_Y,
        MASS_PANEL_Z,
    );
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_evaporation_mass_balance_pads"));

    for position in 0..CHIP_POSITION_COUNT {
        let row = position / CHIP_COLS;
        let col = position % CHIP_COLS;
        let x = -MASS_PAD_PITCH_X * 1.5 + col as f64 * MASS_PAD_PITCH_X;
        let y = MASS_PAD_PITCH_Y * 1.5 - row as f64 * MASS_PAD_PITCH_Y;
        let zone = chip_zone(position);
        let pad = centered_cube(
            format!(
                "{OUTPUT_PREFIX}_position_{position:02}_{}_evaporation_pad",
                zone.label()
            ),
            MASS_PAD_X,
            MASS_PAD_Y,
            MASS_PAD_Z,
        )
        .translate(x, y, MASS_PANEL_Z / 2.0 + MASS_PAD_Z / 2.0);
        let well_a = centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_mass_well_a"),
            MASS_WELL_D / 2.0,
            MASS_WELL_DEPTH + 0.4,
            24,
        )
        .translate(
            x - 10.0,
            y,
            MASS_PANEL_Z / 2.0 + MASS_PAD_Z - MASS_WELL_DEPTH / 2.0,
        );
        let well_b = centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_mass_well_b"),
            MASS_WELL_D / 2.0,
            MASS_WELL_DEPTH + 0.4,
            24,
        )
        .translate(
            x + 10.0,
            y,
            MASS_PANEL_Z / 2.0 + MASS_PAD_Z - MASS_WELL_DEPTH / 2.0,
        );
        pads = pads + (pad - well_a - well_b);
    }

    let draft_shield = centered_cube(
        format!("{OUTPUT_PREFIX}_mass_pad_draft_shield_low_wall"),
        MASS_PANEL_X - 32.0,
        8.0,
        22.0,
    )
    .translate(0.0, MASS_PANEL_Y / 2.0 - 20.0, MASS_PANEL_Z / 2.0 + 11.0);

    (panel + pads + draft_shield).translate(MASS_POS.0, MASS_POS.1, DECK_Z + MASS_PANEL_Z / 2.0)
}

fn flow_shadow_and_thermal_dummy_loads() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_flow_shadow_thermal_load_panel"),
        SHADOW_PANEL_X,
        SHADOW_PANEL_Y,
        SHADOW_PANEL_Z,
    );
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_flow_shadow_thermal_load_features"));

    for index in 0..FLOW_RIBBON_COUNT {
        let y = -78.0 + index as f64 * 22.0;
        let ribbon = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_shadow_ribbon_{index:02}"),
            FLOW_RIBBON_X,
            FLOW_RIBBON_Y,
            FLOW_RIBBON_Z,
        )
        .translate(-88.0, y, SHADOW_PANEL_Z / 2.0 + FLOW_RIBBON_Z / 2.0);
        let witness = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_shadow_witness_land_{index:02}"),
            38.0,
            14.0,
            4.0,
        )
        .translate(38.0, y, SHADOW_PANEL_Z / 2.0 + 2.0);
        features = features + ribbon + witness;
    }

    for index in 0..THERMAL_LOAD_COUNT {
        let x = 104.0 + (index % 2) as f64 * 50.0;
        let y = -72.0 + (index / 2) as f64 * 48.0;
        let load = centered_cylinder(
            format!("{OUTPUT_PREFIX}_thermal_dummy_load_{index:02}"),
            THERMAL_LOAD_D / 2.0,
            THERMAL_LOAD_Z,
            36,
        )
        .translate(x, y, SHADOW_PANEL_Z / 2.0 + THERMAL_LOAD_Z / 2.0);
        features = features + load;
    }

    let baffle_reference = centered_cube(
        format!("{OUTPUT_PREFIX}_removable_baffle_reference_slot"),
        28.0,
        SHADOW_PANEL_Y - 42.0,
        10.0,
    )
    .translate(-186.0, 0.0, SHADOW_PANEL_Z / 2.0 + 5.0);

    (panel + features + baffle_reference).translate(
        SHADOW_POS.0,
        SHADOW_POS.1,
        DECK_Z + SHADOW_PANEL_Z / 2.0,
    )
}

fn logger_calibration_dock_array() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_logger_calibration_dock_panel"),
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let mut docks = Part::empty(format!("{OUTPUT_PREFIX}_logger_calibration_docks"));
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_logger_calibration_dock_recesses"));

    for sensor in SensorKind::all() {
        let index = sensor.index();
        let x = -LOGGER_DOCK_PITCH_X + (index % 3) as f64 * LOGGER_DOCK_PITCH_X;
        let y = if index < 3 {
            LOGGER_DOCK_PITCH_Y / 2.0
        } else {
            -LOGGER_DOCK_PITCH_Y / 2.0
        };
        let dock = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_logger_dock_wall", sensor.label()),
            LOGGER_DOCK_X,
            LOGGER_DOCK_Y,
            LOGGER_DOCK_Z,
        )
        .translate(x, y, LOGGER_PANEL_Z / 2.0 + LOGGER_DOCK_Z / 2.0);
        let recess = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_logger_dock_recess", sensor.label()),
            LOGGER_DOCK_X - 14.0,
            LOGGER_DOCK_Y - 14.0,
            LOGGER_DOCK_Z + 0.6,
        )
        .translate(x, y, LOGGER_PANEL_Z / 2.0 + LOGGER_DOCK_Z / 2.0);
        let cal_gas_pin = centered_cylinder(
            format!("{OUTPUT_PREFIX}_{}_calibration_port_pin", sensor.label()),
            4.0,
            8.0,
            24,
        )
        .translate(
            x - LOGGER_DOCK_X / 2.0 + 13.0,
            y,
            LOGGER_PANEL_Z / 2.0 + 4.0,
        );
        docks = docks + dock + cal_gas_pin;
        cuts = cuts + recess;
    }

    (panel + docks - cuts).translate(LOGGER_POS.0, LOGGER_POS.1, DECK_Z + LOGGER_PANEL_Z / 2.0)
}

fn position_map_token_board() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_position_map_token_board"),
        TOKEN_BOARD_X,
        TOKEN_BOARD_Y,
        TOKEN_BOARD_Z,
    );
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_position_map_tokens"));
    for position in 0..CHIP_POSITION_COUNT {
        let (x, y) = token_xy(position);
        let zone = chip_zone(position);
        let token = centered_cylinder(
            format!(
                "{OUTPUT_PREFIX}_position_{position:02}_{}_map_token",
                zone.label()
            ),
            POSITION_TOKEN_D / 2.0,
            4.0,
            28,
        )
        .translate(x, y, TOKEN_BOARD_Z / 2.0 + 2.0);
        let barcode = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_barcode_land"),
            TOKEN_BARCODE_X,
            TOKEN_BARCODE_Y,
            TOKEN_BARCODE_Z,
        )
        .translate(x + 38.0, y, TOKEN_BOARD_Z / 2.0 + TOKEN_BARCODE_Z / 2.0);
        tokens = tokens + token + barcode;
    }
    let run_card = centered_cube(
        format!("{OUTPUT_PREFIX}_run_record_card_land"),
        TOKEN_BOARD_X - 34.0,
        24.0,
        4.0,
    )
    .translate(0.0, -TOKEN_BOARD_Y / 2.0 + 26.0, TOKEN_BOARD_Z / 2.0 + 2.0);

    (board + tokens + run_card).translate(TOKEN_POS.0, TOKEN_POS.1, DECK_Z + TOKEN_BOARD_Z / 2.0)
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_lane_panel"),
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_recesses"));
    let mut walls = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_walls"));

    for lane in DispositionLane::all() {
        let x = (lane.index() as f64 - 1.0) * LANE_PITCH_X;
        let label = lane.label();
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_lane_token_recess"),
                LANE_SLOT_X,
                LANE_SLOT_Y,
                LANE_PANEL_Z + 0.6,
            )
            .translate(x, 0.0, LANE_PANEL_Z / 2.0);
        walls = walls
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_lane_front_wall"),
                LANE_SLOT_X + LANE_WALL_W,
                LANE_WALL_W,
                15.0,
            )
            .translate(
                x,
                -LANE_SLOT_Y / 2.0 - LANE_WALL_W / 2.0,
                LANE_PANEL_Z / 2.0 + 7.5,
            );
        for token_index in 0..lane.capacity() {
            let row = token_index / 4;
            let col = token_index % 4;
            let token = centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_lane_capacity_marker_{token_index:02}"),
                12.0,
                8.0,
                3.0,
            )
            .translate(
                x - 42.0 + col as f64 * 28.0,
                -18.0 + row as f64 * 22.0,
                LANE_PANEL_Z / 2.0 + 1.5,
            );
            walls = walls + token;
        }
    }

    (panel - cuts + walls).translate(LANE_POS.0, LANE_POS.1, DECK_Z + LANE_PANEL_Z / 2.0)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_left_post"),
        CAMERA_POST_W,
        CAMERA_POST_W,
        CAMERA_POST_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0 + CAMERA_POST_W / 2.0,
        0.0,
        CAMERA_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_right_post"),
        CAMERA_POST_W,
        CAMERA_POST_W,
        CAMERA_POST_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0 - CAMERA_POST_W / 2.0,
        0.0,
        CAMERA_POST_Z / 2.0,
    );
    let crossbar = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_crossbar"),
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_CROSSBAR_Z,
    )
    .translate(0.0, 0.0, CAMERA_POST_Z - CAMERA_CROSSBAR_Z / 2.0);
    let carriage = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_camera_xy_carriage"),
        CAMERA_CARRIAGE_X,
        CAMERA_CARRIAGE_Y,
        CAMERA_CARRIAGE_Z,
    )
    .translate(
        MAP_NEST_POS.0 - CAMERA_POS.0,
        0.0,
        CAMERA_POST_Z - CAMERA_CROSSBAR_Z - CAMERA_CARRIAGE_Z / 2.0,
    );
    let lens = centered_cylinder(
        format!("{OUTPUT_PREFIX}_camera_lens_clearance_ring"),
        18.0,
        7.0,
        40,
    )
    .translate(
        MAP_NEST_POS.0 - CAMERA_POS.0,
        0.0,
        CAMERA_POST_Z - CAMERA_CROSSBAR_Z - CAMERA_CARRIAGE_Z - 3.5,
    );

    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_camera_evidence_targets"));
    for index in 0..CAMERA_TARGET_COUNT {
        let x = -430.0 + index as f64 * 215.0;
        targets = targets
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_camera_calibration_target_{index}"),
                8.0,
                3.0,
                28,
            )
            .translate(x, -CAMERA_BRIDGE_Y / 2.0 + 12.0, 6.0);
    }

    (left_post + right_post + crossbar + carriage + lens + targets).translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        DECK_Z,
    )
}

fn robot_service_keepout_gauges() -> Part {
    let robot_front = centered_cube(
        format!("{OUTPUT_PREFIX}_robot_front_sweep_keepout_gauge"),
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -ROBOT_FRONT_CLEARANCE, DECK_Z + KEEP_OUT_Z / 2.0);
    let service_rear = centered_cube(
        format!("{OUTPUT_PREFIX}_service_rear_pull_keepout_gauge"),
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, SERVICE_REAR_CLEARANCE, DECK_Z + KEEP_OUT_Z / 2.0);
    let left_service = centered_cube(
        format!("{OUTPUT_PREFIX}_left_side_service_keepout_gauge"),
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + SERVICE_SIDE_CLEARANCE,
        0.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let right_service = centered_cube(
        format!("{OUTPUT_PREFIX}_right_side_service_keepout_gauge"),
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - SERVICE_SIDE_CLEARANCE,
        0.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let z_goalpost = centered_cube(
        format!("{OUTPUT_PREFIX}_robot_z_clearance_goalpost"),
        28.0,
        28.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        MAP_NEST_POS.0 + MAP_NEST_X / 2.0 + 44.0,
        MAP_NEST_POS.1,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );

    robot_front + service_rear + left_service + right_service + z_goalpost
}

fn module_footprints() -> [Rect; 8] {
    [
        Rect {
            name: "map_nest",
            center: MAP_NEST_POS,
            x: MAP_NEST_X,
            y: MAP_NEST_Y,
        },
        Rect {
            name: "sensor_surrogates",
            center: SENSOR_POS,
            x: SENSOR_TRAY_X,
            y: SENSOR_TRAY_Y,
        },
        Rect {
            name: "coupon_bank",
            center: COUPON_POS,
            x: COUPON_BANK_X,
            y: COUPON_BANK_Y,
        },
        Rect {
            name: "mass_balance",
            center: MASS_POS,
            x: MASS_PANEL_X,
            y: MASS_PANEL_Y,
        },
        Rect {
            name: "flow_shadow",
            center: SHADOW_POS,
            x: SHADOW_PANEL_X,
            y: SHADOW_PANEL_Y,
        },
        Rect {
            name: "logger_docks",
            center: LOGGER_POS,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Rect {
            name: "token_board",
            center: TOKEN_POS,
            x: TOKEN_BOARD_X,
            y: TOKEN_BOARD_Y,
        },
        Rect {
            name: "disposition_lanes",
            center: LANE_POS,
            x: LANE_PANEL_X,
            y: LANE_PANEL_Y,
        },
    ]
}

fn module_footprints_fit_deck() -> bool {
    module_footprints()
        .iter()
        .copied()
        .all(Rect::fits_inside_deck)
}

fn critical_footprints_overlap() -> bool {
    let footprints = module_footprints();
    for i in 0..footprints.len() {
        for j in i + 1..footprints.len() {
            if footprints[i].overlaps(footprints[j]) {
                return true;
            }
        }
    }
    false
}

fn chip_position_xy(position: usize) -> (f64, f64) {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    let x = (col as f64 - (CHIP_COLS as f64 - 1.0) / 2.0) * CHIP_PITCH_X;
    let y = ((CHIP_ROWS as f64 - 1.0) / 2.0 - row as f64) * CHIP_PITCH_Y;
    (x, y)
}

fn chip_zone(position: usize) -> ChipZone {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    if col == 0 || col == CHIP_COLS - 1 || row == 0 || row == CHIP_ROWS - 1 {
        ChipZone::Edge
    } else {
        ChipZone::Center
    }
}

fn edge_position_count() -> usize {
    (0..CHIP_POSITION_COUNT)
        .filter(|position| chip_zone(*position) == ChipZone::Edge)
        .count()
}

fn center_position_count() -> usize {
    CHIP_POSITION_COUNT - edge_position_count()
}

fn edge_coupon_xy(index: usize) -> (f64, f64) {
    let col = index % 4;
    let row = index / 4;
    (
        -COUPON_COL_PITCH * 1.5 + col as f64 * COUPON_COL_PITCH,
        108.0 - row as f64 * COUPON_ROW_PITCH,
    )
}

fn center_coupon_xy(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (
        -COUPON_COL_PITCH / 2.0 + col as f64 * COUPON_COL_PITCH,
        -74.0 - row as f64 * COUPON_ROW_PITCH,
    )
}

fn token_xy(position: usize) -> (f64, f64) {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    (
        -TOKEN_BOARD_X / 2.0 + 34.0 + col as f64 * 49.0,
        TOKEN_BOARD_Y / 2.0 - 42.0 - row as f64 * 38.0,
    )
}

fn total_mass_well_count() -> usize {
    CHIP_POSITION_COUNT * MASS_WELLS_PER_POSITION
}

fn total_lane_capacity() -> usize {
    DispositionLane::all()
        .into_iter()
        .map(DispositionLane::capacity)
        .sum()
}

fn required_feature_count() -> usize {
    REQUIRED_FEATURES.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_position_map_has_expected_edge_center_split() {
        assert_eq!(CHIP_POSITION_COUNT, 16);
        assert_eq!(edge_position_count(), 12);
        assert_eq!(center_position_count(), 4);
        assert_eq!(chip_zone(5), ChipZone::Center);
        assert_eq!(chip_zone(10), ChipZone::Center);
        assert_eq!(chip_zone(0), ChipZone::Edge);
        assert_eq!(chip_zone(15), ChipZone::Edge);
    }

    #[test]
    fn all_modules_fit_without_overlap() {
        for rect in module_footprints() {
            assert!(
                rect.fits_inside_deck(),
                "{} footprint is outside deck",
                rect.name
            );
        }
        assert!(!critical_footprints_overlap());
    }

    #[test]
    fn outputs_are_named_for_parent_manifest_integration() {
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output.starts_with(
                "output/closed_chip_position_edge_center_environment_mapping_station_"
            ));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn sensor_and_logger_counts_cover_environmental_variables() {
        assert_eq!(SensorKind::all().len(), SENSOR_KIND_COUNT);
        assert_eq!(LOGGER_DOCK_COUNT, SENSOR_KIND_COUNT);
        assert!(SensorKind::all().contains(&SensorKind::Co2));
        assert!(SensorKind::all().contains(&SensorKind::O2));
        assert!(SensorKind::all().contains(&SensorKind::Rh));
    }

    #[test]
    fn disposition_lanes_cover_every_chip_position_token() {
        assert_eq!(total_lane_capacity(), CHIP_POSITION_COUNT);
    }

    #[test]
    fn mass_balance_has_duplicate_wells_per_chip_position() {
        assert_eq!(
            total_mass_well_count(),
            CHIP_POSITION_COUNT * MASS_WELLS_PER_POSITION
        );
        assert_eq!(total_mass_well_count(), 32);
    }
}
