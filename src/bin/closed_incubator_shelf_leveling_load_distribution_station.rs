use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator shelf leveling and load-distribution validation station.
//
// No-cell fixture for checking shelf datum level, loaded-shelf sag,
// cassette-slot pitch, edge/center cassette repeatability, and evidence capture
// before live closed culture chips are placed into the incubator.

const BIN_PREFIX: &str = "closed_incubator_shelf_leveling_load_distribution_station";

const OUTPUTS: &[&str] = &[
    "output/closed_incubator_shelf_leveling_load_distribution_station_deck.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_adjustable_shelf_datum_plate.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_load_cell_reference_mass_pockets.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_level_tilt_vial_pockets.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_rack_slot_surrogate_rails.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_deflection_witness_gauges.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_edge_center_cassette_blocks.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_barcode_status_lanes.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_camera_evidence_bridge.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_clean_used_segregation.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_robot_service_keepouts.stl",
    "output/closed_incubator_shelf_leveling_load_distribution_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "adjustable_shelf_datum_plate",
    "load_cell_reference_mass_pockets",
    "level_tilt_vial_pockets",
    "rack_slot_surrogate_rails",
    "deflection_witness_gauges",
    "edge_center_cassette_blocks",
    "barcode_status_lanes",
    "camera_evidence_bridge",
    "clean_used_segregation",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1120.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 30.0;
const MOUNT_HOLE_COUNT: usize = 8;

const DATUM_PLATE_X: f64 = 790.0;
const DATUM_PLATE_Y: f64 = 430.0;
const DATUM_PLATE_Z: f64 = 24.0;
const DATUM_PLATE_POS: (f64, f64) = (-58.0, 52.0);
const DATUM_RECESS_DEPTH: f64 = 4.0;
const LEVELING_FOOT_COUNT: usize = 4;
const LEVELING_FOOT_D: f64 = 42.0;
const LEVELING_SCREW_D: f64 = 12.0;
const DATUM_PIN_COUNT: usize = 6;
const DATUM_PIN_D: f64 = 8.0;

const LOAD_POCKET_COUNT: usize = 6;
const LOAD_CELL_POCKET_X: f64 = 84.0;
const LOAD_CELL_POCKET_Y: f64 = 72.0;
const LOAD_CELL_RECESS_Z: f64 = 8.0;
const REFERENCE_MASS_D: f64 = 36.0;
const MASS_TOKEN_COUNT: usize = 12;
const LOAD_GRID_COLS: usize = 3;
const LOAD_GRID_ROWS: usize = 2;
const LOAD_GRID_PITCH_X: f64 = 220.0;
const LOAD_GRID_PITCH_Y: f64 = 166.0;

const VIAL_POCKET_COUNT: usize = 5;
const LEVEL_VIAL_X: f64 = 112.0;
const LEVEL_VIAL_Y: f64 = 24.0;
const LEVEL_VIAL_RECESS_Z: f64 = 8.0;
const VIAL_BOSS_D: f64 = 14.0;

const RACK_SLOT_COUNT: usize = 6;
const SLOT_PITCH_X: f64 = 116.0;
const SLOT_RAIL_X: f64 = 94.0;
const SLOT_RAIL_Y: f64 = 238.0;
const SLOT_RAIL_Z: f64 = 18.0;
const SLOT_RAIL_GAP: f64 = 42.0;
const SLOT_PITCH_COMB_X: f64 = 724.0;
const SLOT_PITCH_COMB_Y: f64 = 82.0;
const SLOT_PITCH_COMB_Z: f64 = 16.0;

const DEFLECTION_GAUGE_COUNT: usize = 7;
const DEFLECTION_POST_D: f64 = 18.0;
const DEFLECTION_POST_Z: f64 = 84.0;
const WITNESS_FLAG_X: f64 = 34.0;
const WITNESS_FLAG_Y: f64 = 6.0;
const WITNESS_FLAG_Z: f64 = 28.0;
const SAG_SCALE_TICK_COUNT: usize = 5;

const CASSETTE_BLOCK_COUNT: usize = 5;
const CASSETTE_BLOCK_X: f64 = 138.0;
const CASSETTE_BLOCK_Y: f64 = 92.0;
const CASSETTE_BLOCK_Z: f64 = 30.0;
const EDGE_BLOCK_COUNT: usize = 4;
const CENTER_BLOCK_COUNT: usize = 1;

const BARCODE_LANE_COUNT: usize = 6;
const STATUS_LANE_COUNT: usize = 3;
const BARCODE_LANE_X: f64 = 118.0;
const BARCODE_LANE_Y: f64 = 28.0;
const BARCODE_LANE_Z: f64 = 5.0;
const STATUS_TOKEN_D: f64 = 20.0;

const CAMERA_BRIDGE_X: f64 = 900.0;
const CAMERA_BRIDGE_Y: f64 = 70.0;
const CAMERA_BRIDGE_Z: f64 = 168.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const CAMERA_WINDOW_X: f64 = 348.0;
const CAMERA_WINDOW_Y: f64 = 38.0;
const CAMERA_FIDUCIAL_COUNT: usize = 8;
const CAMERA_FIDUCIAL_D: f64 = 11.0;

const SEGREGATION_RIB_X: f64 = 22.0;
const SEGREGATION_RIB_Y: f64 = 610.0;
const SEGREGATION_RIB_Z: f64 = 46.0;
const SEGREGATION_GAP_X: f64 = 56.0;
const CLEAN_USED_TRAY_X: f64 = 210.0;
const CLEAN_USED_TRAY_Y: f64 = 132.0;
const CLEAN_USED_TRAY_Z: f64 = 22.0;

const ROBOT_APPROACH_KEEP_OUT_X: f64 = 830.0;
const ROBOT_APPROACH_KEEP_OUT_Y: f64 = 92.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 96.0;
const REAR_CAMERA_SERVICE_KEEP_OUT_Y: f64 = 96.0;
const OVERHEAD_CLEARANCE_Z: f64 = 245.0;
const HAND_SERVICE_CLEARANCE_Z: f64 = 76.0;
const KEEP_OUT_GAUGE_Z: f64 = 6.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

const COMPONENT_FOOTPRINTS: &[Footprint] = &[
    Footprint {
        name: "adjustable_shelf_datum_plate",
        center: DATUM_PLATE_POS,
        width: DATUM_PLATE_X,
        depth: DATUM_PLATE_Y,
    },
    Footprint {
        name: "rack_slot_surrogate_rails",
        center: (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1 + 2.0),
        width: SLOT_PITCH_COMB_X,
        depth: SLOT_RAIL_Y + 42.0,
    },
    Footprint {
        name: "barcode_status_lanes",
        center: (0.0, -306.0),
        width: 820.0,
        depth: 70.0,
    },
    Footprint {
        name: "clean_used_segregation",
        center: (386.0, 0.0),
        width: 292.0,
        depth: SEGREGATION_RIB_Y,
    },
    Footprint {
        name: "camera_evidence_bridge",
        center: (0.0, 244.0),
        width: CAMERA_BRIDGE_X,
        depth: CAMERA_BRIDGE_Y,
    },
];

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_sanity();

    let deck = deck();
    export(OUTPUTS[0], &deck);

    let datum = adjustable_shelf_datum_plate();
    export(OUTPUTS[1], &datum);

    let load = load_cell_reference_mass_pockets();
    export(OUTPUTS[2], &load);

    let vials = level_tilt_vial_pockets();
    export(OUTPUTS[3], &vials);

    let rails = rack_slot_surrogate_rails();
    export(OUTPUTS[4], &rails);

    let gauges = deflection_witness_gauges();
    export(OUTPUTS[5], &gauges);

    let cassette_blocks = edge_center_cassette_blocks();
    export(OUTPUTS[6], &cassette_blocks);

    let lanes = barcode_status_lanes();
    export(OUTPUTS[7], &lanes);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[8], &bridge);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + datum
        + load
        + vials
        + rails
        + gauges
        + cassette_blocks
        + lanes
        + bridge
        + segregation
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator shelf leveling/load-distribution station:");
    println!(
        "  Shelf datum:                 {DATUM_PLATE_X:.0}mm x {DATUM_PLATE_Y:.0}mm adjustable datum plate with {LEVELING_FOOT_COUNT} leveling feet and {DATUM_PIN_COUNT} datum pins"
    );
    println!(
        "  Load mapping:                {LOAD_POCKET_COUNT} load-cell pockets on a {LOAD_GRID_COLS}x{LOAD_GRID_ROWS} grid, {MASS_TOKEN_COUNT} reference-mass tokens"
    );
    println!(
        "  Level/sag evidence:          {VIAL_POCKET_COUNT} level/tilt vial pockets and {DEFLECTION_GAUGE_COUNT} deflection witness posts with {SAG_SCALE_TICK_COUNT} scale ticks each"
    );
    println!(
        "  Slot repeatability:          {RACK_SLOT_COUNT} rack-slot surrogate rails at {SLOT_PITCH_X:.0}mm pitch plus {EDGE_BLOCK_COUNT} edge and {CENTER_BLOCK_COUNT} center cassette blocks"
    );
    println!(
        "  Traceability/keepouts:       {BARCODE_LANE_COUNT} barcode lanes, {STATUS_LANE_COUNT} status lanes, {CAMERA_FIDUCIAL_COUNT} camera fiducials, and {} feature groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck() -> Part {
    let base = centered_cube(format!("{BIN_PREFIX}_deck_base"), DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let datum_recess = centered_cube(
        format!("{BIN_PREFIX}_datum_plate_recess"),
        DATUM_PLATE_X + 18.0,
        DATUM_PLATE_Y + 18.0,
        DATUM_RECESS_DEPTH,
    )
    .translate(
        DATUM_PLATE_POS.0,
        DATUM_PLATE_POS.1,
        DECK_Z - DATUM_RECESS_DEPTH / 2.0 + 0.1,
    );

    base - datum_recess + deck_rim() + deck_mount_holes() + lift_handle_bars()
}

fn deck_rim() -> Part {
    let front = centered_cube(
        format!("{BIN_PREFIX}_front_deck_rim"),
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
        format!("{BIN_PREFIX}_rear_deck_rim"),
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
        format!("{BIN_PREFIX}_left_deck_rim"),
        DECK_RIM_W,
        DECK_Y - 2.0 * DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{BIN_PREFIX}_right_deck_rim"),
        DECK_RIM_W,
        DECK_Y - 2.0 * DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{BIN_PREFIX}_deck_mount_hole_markers"));
    for (index, (x, y)) in deck_mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{BIN_PREFIX}_deck_m6_mount_marker_{index}"),
                5.0,
                4.0,
                30,
            )
            .translate(x, y, DECK_Z + 2.0);
    }
    holes
}

fn lift_handle_bars() -> Part {
    let mut handles = Part::empty(format!("{BIN_PREFIX}_lift_handle_bars"));
    for (index, y) in [-DECK_Y / 2.0 + 58.0, DECK_Y / 2.0 - 58.0]
        .into_iter()
        .enumerate()
    {
        let bridge = centered_cube(
            format!("{BIN_PREFIX}_lift_handle_bridge_{index}"),
            156.0,
            16.0,
            18.0,
        )
        .translate(-454.0, y, DECK_Z + 9.0);
        let left_standoff = centered_cube(
            format!("{BIN_PREFIX}_lift_handle_left_standoff_{index}"),
            18.0,
            22.0,
            18.0,
        )
        .translate(-526.0, y, DECK_Z + 9.0);
        let right_standoff = centered_cube(
            format!("{BIN_PREFIX}_lift_handle_right_standoff_{index}"),
            18.0,
            22.0,
            18.0,
        )
        .translate(-382.0, y, DECK_Z + 9.0);
        handles = handles + bridge + left_standoff + right_standoff;
    }
    handles
}

fn adjustable_shelf_datum_plate() -> Part {
    let plate = centered_cube(
        format!("{BIN_PREFIX}_adjustable_shelf_datum_plate"),
        DATUM_PLATE_X,
        DATUM_PLATE_Y,
        DATUM_PLATE_Z,
    )
    .translate(
        DATUM_PLATE_POS.0,
        DATUM_PLATE_POS.1,
        deck_top_z() + DATUM_PLATE_Z / 2.0 - DATUM_RECESS_DEPTH,
    );

    plate - leveling_screw_clearances() - load_cell_recess_cuts() - vial_recess_cuts()
        + leveling_foot_bosses()
        + datum_pin_bosses()
        + datum_edge_rails()
        + pitch_reference_crosshairs()
}

fn leveling_foot_bosses() -> Part {
    let mut bosses = Part::empty(format!("{BIN_PREFIX}_leveling_foot_bosses"));
    for (index, (x, y)) in leveling_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("{BIN_PREFIX}_leveling_foot_boss_{index}"),
            LEVELING_FOOT_D / 2.0,
            10.0,
            40,
        )
        .translate(x, y, datum_top_z() + 5.0);
        let adjuster_head = centered_cylinder(
            format!("{BIN_PREFIX}_leveling_foot_adjuster_head_{index}"),
            LEVELING_SCREW_D / 2.0 + 5.0,
            5.0,
            36,
        )
        .translate(x, y, datum_top_z() + 12.5);
        bosses = bosses + boss + adjuster_head;
    }
    bosses
}

fn leveling_screw_clearances() -> Part {
    let mut clearances = Part::empty(format!("{BIN_PREFIX}_leveling_screw_clearances"));
    for (index, (x, y)) in leveling_points().into_iter().enumerate() {
        clearances = clearances
            + centered_cylinder(
                format!("{BIN_PREFIX}_leveling_screw_clearance_{index}"),
                LEVELING_SCREW_D / 2.0,
                DATUM_PLATE_Z + 4.0,
                36,
            )
            .translate(
                x,
                y,
                deck_top_z() + DATUM_PLATE_Z / 2.0 - DATUM_RECESS_DEPTH,
            );
    }
    clearances
}

fn datum_pin_bosses() -> Part {
    let mut pins = Part::empty(format!("{BIN_PREFIX}_datum_pin_bosses"));
    for (index, (x, y)) in datum_pin_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("{BIN_PREFIX}_datum_pin_boss_{index}"),
            DATUM_PIN_D / 2.0 + 5.0,
            5.0,
            36,
        )
        .translate(x, y, datum_top_z() + 2.5);
        let pin = centered_cylinder(
            format!("{BIN_PREFIX}_datum_pin_{index}"),
            DATUM_PIN_D / 2.0,
            14.0,
            36,
        )
        .translate(x, y, datum_top_z() + 7.0);
        pins = pins + boss + pin;
    }
    pins
}

fn datum_edge_rails() -> Part {
    let left = centered_cube(
        format!("{BIN_PREFIX}_left_shelf_edge_datum_rail"),
        16.0,
        DATUM_PLATE_Y - 22.0,
        18.0,
    )
    .translate(
        DATUM_PLATE_POS.0 - DATUM_PLATE_X / 2.0 + 22.0,
        DATUM_PLATE_POS.1,
        datum_top_z() + 9.0,
    );
    let rear = centered_cube(
        format!("{BIN_PREFIX}_rear_shelf_edge_datum_rail"),
        DATUM_PLATE_X - 22.0,
        16.0,
        18.0,
    )
    .translate(
        DATUM_PLATE_POS.0,
        DATUM_PLATE_POS.1 + DATUM_PLATE_Y / 2.0 - 22.0,
        datum_top_z() + 9.0,
    );
    let right_soft = centered_cube(
        format!("{BIN_PREFIX}_right_soft_capture_rail"),
        12.0,
        DATUM_PLATE_Y * 0.56,
        10.0,
    )
    .translate(
        DATUM_PLATE_POS.0 + DATUM_PLATE_X / 2.0 - 28.0,
        DATUM_PLATE_POS.1 - 20.0,
        datum_top_z() + 5.0,
    );

    left + rear + right_soft
}

fn pitch_reference_crosshairs() -> Part {
    let x_axis = centered_cube(
        format!("{BIN_PREFIX}_datum_x_pitch_reference_line"),
        DATUM_PLATE_X - 86.0,
        3.0,
        2.4,
    )
    .translate(DATUM_PLATE_POS.0, DATUM_PLATE_POS.1, datum_top_z() + 1.2);
    let y_axis = centered_cube(
        format!("{BIN_PREFIX}_datum_y_pitch_reference_line"),
        3.0,
        DATUM_PLATE_Y - 76.0,
        2.4,
    )
    .translate(DATUM_PLATE_POS.0, DATUM_PLATE_POS.1, datum_top_z() + 1.2);

    x_axis + y_axis
}

fn load_cell_reference_mass_pockets() -> Part {
    let mut pockets = Part::empty(format!("{BIN_PREFIX}_load_cell_reference_mass_pockets"));
    for index in 0..LOAD_POCKET_COUNT {
        let (x, y) = load_pocket_center(index);
        let load_cell_land = centered_cube(
            format!("{BIN_PREFIX}_load_cell_land_{index}"),
            LOAD_CELL_POCKET_X + 14.0,
            LOAD_CELL_POCKET_Y + 14.0,
            5.0,
        )
        .translate(x, y, datum_top_z() + 2.5);
        let reference_mass_puck = centered_cylinder(
            format!("{BIN_PREFIX}_reference_mass_puck_{index}"),
            REFERENCE_MASS_D / 2.0,
            12.0,
            42,
        )
        .translate(x, y, datum_top_z() + 11.0);
        let cable_exit = centered_cube(
            format!("{BIN_PREFIX}_load_cell_{index}_cable_exit_channel"),
            12.0,
            54.0,
            5.0,
        )
        .translate(x + 46.0, y - 30.0, datum_top_z() + 2.5);
        pockets = pockets + load_cell_land + reference_mass_puck + cable_exit;
    }

    pockets + mass_token_storage()
}

fn load_cell_recess_cuts() -> Part {
    let mut cuts = Part::empty(format!("{BIN_PREFIX}_load_cell_recess_cuts"));
    for index in 0..LOAD_POCKET_COUNT {
        let (x, y) = load_pocket_center(index);
        cuts = cuts
            + centered_cube(
                format!("{BIN_PREFIX}_load_cell_recess_{index}"),
                LOAD_CELL_POCKET_X,
                LOAD_CELL_POCKET_Y,
                LOAD_CELL_RECESS_Z,
            )
            .translate(x, y, datum_top_z() - LOAD_CELL_RECESS_Z / 2.0 + 0.2);
    }
    cuts
}

fn mass_token_storage() -> Part {
    let mut tokens = Part::empty(format!("{BIN_PREFIX}_reference_mass_token_storage"));
    for index in 0..MASS_TOKEN_COUNT {
        let row = index / 6;
        let col = index % 6;
        let x = 352.0 + centered_index(col, 6, 30.0);
        let y = -102.0 + centered_index(row, 2, 34.0);
        tokens = tokens
            + centered_cylinder(
                format!("{BIN_PREFIX}_reference_mass_token_pocket_{index}"),
                12.0,
                8.0,
                32,
            )
            .translate(x, y, deck_top_z() + 4.0);
    }
    tokens
}

fn level_tilt_vial_pockets() -> Part {
    let mut vials = Part::empty(format!("{BIN_PREFIX}_level_tilt_vial_pockets"));
    for index in 0..VIAL_POCKET_COUNT {
        let (x, y, angle_z) = vial_pose(index);
        let vial = centered_cube(
            format!("{BIN_PREFIX}_level_tilt_vial_land_{index}"),
            LEVEL_VIAL_X + 18.0,
            LEVEL_VIAL_Y + 12.0,
            6.0,
        )
        .rotate(0.0, 0.0, angle_z)
        .translate(x, y, datum_top_z() + 3.0);
        let left_boss = centered_cylinder(
            format!("{BIN_PREFIX}_level_tilt_vial_{index}_left_boss"),
            VIAL_BOSS_D / 2.0,
            8.0,
            30,
        )
        .translate(x - 48.0, y, datum_top_z() + 4.0);
        let right_boss = centered_cylinder(
            format!("{BIN_PREFIX}_level_tilt_vial_{index}_right_boss"),
            VIAL_BOSS_D / 2.0,
            8.0,
            30,
        )
        .translate(x + 48.0, y, datum_top_z() + 4.0);
        vials = vials + vial + left_boss + right_boss;
    }
    vials
}

fn vial_recess_cuts() -> Part {
    let mut cuts = Part::empty(format!("{BIN_PREFIX}_level_tilt_vial_recess_cuts"));
    for index in 0..VIAL_POCKET_COUNT {
        let (x, y, angle_z) = vial_pose(index);
        cuts = cuts
            + centered_cube(
                format!("{BIN_PREFIX}_level_tilt_vial_recess_{index}"),
                LEVEL_VIAL_X,
                LEVEL_VIAL_Y,
                LEVEL_VIAL_RECESS_Z,
            )
            .rotate(0.0, 0.0, angle_z)
            .translate(x, y, datum_top_z() - LEVEL_VIAL_RECESS_Z / 2.0 + 0.2);
    }
    cuts
}

fn rack_slot_surrogate_rails() -> Part {
    let comb = centered_cube(
        format!("{BIN_PREFIX}_rack_slot_pitch_comb_base"),
        SLOT_PITCH_COMB_X,
        SLOT_PITCH_COMB_Y,
        SLOT_PITCH_COMB_Z,
    )
    .translate(
        DATUM_PLATE_POS.0,
        DATUM_PLATE_POS.1 - 2.0,
        datum_top_z() + SLOT_PITCH_COMB_Z / 2.0,
    );

    comb + slot_rail_pairs() + slot_pitch_witness_ticks()
}

fn slot_rail_pairs() -> Part {
    let mut rails = Part::empty(format!("{BIN_PREFIX}_rack_slot_surrogate_rail_pairs"));
    for slot in 0..RACK_SLOT_COUNT {
        let x = slot_center_x(slot);
        let y = DATUM_PLATE_POS.1 - 2.0;
        let left = centered_cube(
            format!("{BIN_PREFIX}_slot_{slot}_left_surrogate_rail"),
            SLOT_RAIL_X,
            8.0,
            SLOT_RAIL_Z,
        )
        .translate(
            x,
            y - SLOT_RAIL_GAP / 2.0,
            datum_top_z() + SLOT_PITCH_COMB_Z + SLOT_RAIL_Z / 2.0,
        );
        let right = centered_cube(
            format!("{BIN_PREFIX}_slot_{slot}_right_surrogate_rail"),
            SLOT_RAIL_X,
            8.0,
            SLOT_RAIL_Z,
        )
        .translate(
            x,
            y + SLOT_RAIL_GAP / 2.0,
            datum_top_z() + SLOT_PITCH_COMB_Z + SLOT_RAIL_Z / 2.0,
        );
        let nose_stop = centered_cube(
            format!("{BIN_PREFIX}_slot_{slot}_nose_stop"),
            SLOT_RAIL_X,
            10.0,
            SLOT_RAIL_Z,
        )
        .translate(
            x,
            y + SLOT_RAIL_Y / 2.0 - 16.0,
            datum_top_z() + SLOT_PITCH_COMB_Z + SLOT_RAIL_Z / 2.0,
        );
        rails = rails + left + right + nose_stop;
    }
    rails
}

fn slot_pitch_witness_ticks() -> Part {
    let mut ticks = Part::empty(format!("{BIN_PREFIX}_slot_pitch_witness_ticks"));
    for slot in 0..RACK_SLOT_COUNT {
        let x = slot_center_x(slot);
        let center_tick = centered_cube(
            format!("{BIN_PREFIX}_slot_{slot}_centerline_tick"),
            3.0,
            SLOT_PITCH_COMB_Y + 22.0,
            4.0,
        )
        .translate(
            x,
            DATUM_PLATE_POS.1 - 2.0,
            datum_top_z() + SLOT_PITCH_COMB_Z + 2.0,
        );
        ticks = ticks + center_tick;
    }
    ticks
}

fn deflection_witness_gauges() -> Part {
    let mut gauges = Part::empty(format!("{BIN_PREFIX}_deflection_witness_gauges"));
    for index in 0..DEFLECTION_GAUGE_COUNT {
        let (x, y) = deflection_gauge_center(index);
        let post = centered_cylinder(
            format!("{BIN_PREFIX}_deflection_post_{index}"),
            DEFLECTION_POST_D / 2.0,
            DEFLECTION_POST_Z,
            36,
        )
        .translate(x, y, deck_top_z() + DEFLECTION_POST_Z / 2.0);
        let flag = centered_cube(
            format!("{BIN_PREFIX}_deflection_witness_flag_{index}"),
            WITNESS_FLAG_X,
            WITNESS_FLAG_Y,
            WITNESS_FLAG_Z,
        )
        .translate(
            x + 20.0,
            y,
            deck_top_z() + DEFLECTION_POST_Z - WITNESS_FLAG_Z / 2.0,
        );
        gauges = gauges + post + flag + sag_scale_ticks(index);
    }
    gauges
}

fn sag_scale_ticks(gauge: usize) -> Part {
    let mut ticks = Part::empty(format!("{BIN_PREFIX}_sag_scale_ticks_{gauge}"));
    let (x, y) = deflection_gauge_center(gauge);
    for tick in 0..SAG_SCALE_TICK_COUNT {
        ticks = ticks
            + centered_cube(
                format!("{BIN_PREFIX}_deflection_gauge_{gauge}_scale_tick_{tick}"),
                22.0 - tick as f64 * 2.0,
                2.5,
                2.0,
            )
            .translate(x + 25.0, y, deck_top_z() + 22.0 + tick as f64 * 11.0);
    }
    ticks
}

fn edge_center_cassette_blocks() -> Part {
    let mut blocks = Part::empty(format!("{BIN_PREFIX}_edge_center_cassette_blocks"));
    for index in 0..CASSETTE_BLOCK_COUNT {
        let (x, y) = cassette_block_center(index);
        let kind = if index == 2 { "center" } else { "edge" };
        let body = centered_cube(
            format!("{BIN_PREFIX}_{kind}_cassette_block_{index}"),
            CASSETTE_BLOCK_X,
            CASSETTE_BLOCK_Y,
            CASSETTE_BLOCK_Z,
        )
        .translate(x, y, datum_top_z() + CASSETTE_BLOCK_Z / 2.0);
        let reference_target = centered_cylinder(
            format!("{BIN_PREFIX}_{kind}_cassette_block_{index}_repeatability_target"),
            14.0,
            3.0,
            36,
        )
        .translate(x, y, datum_top_z() + CASSETTE_BLOCK_Z + 1.5);
        let edge_key = centered_cube(
            format!("{BIN_PREFIX}_{kind}_cassette_block_{index}_edge_key"),
            12.0,
            CASSETTE_BLOCK_Y + 10.0,
            10.0,
        )
        .translate(
            x - CASSETTE_BLOCK_X / 2.0 + 10.0,
            y,
            datum_top_z() + CASSETTE_BLOCK_Z + 5.0,
        );
        blocks = blocks + body + reference_target + edge_key;
    }
    blocks
}

fn barcode_status_lanes() -> Part {
    let panel = centered_cube(
        format!("{BIN_PREFIX}_barcode_status_lane_panel"),
        850.0,
        86.0,
        10.0,
    )
    .translate(0.0, -306.0, deck_top_z() + 5.0);
    let mut lanes = Part::empty(format!("{BIN_PREFIX}_barcode_status_lanes"));
    for index in 0..BARCODE_LANE_COUNT {
        let x = centered_index(index, BARCODE_LANE_COUNT, 132.0);
        lanes = lanes
            + centered_cube(
                format!("{BIN_PREFIX}_slot_{index}_barcode_lane"),
                BARCODE_LANE_X,
                BARCODE_LANE_Y,
                BARCODE_LANE_Z,
            )
            .translate(x, -322.0, deck_top_z() + 10.0 + BARCODE_LANE_Z / 2.0);
    }
    for index in 0..STATUS_LANE_COUNT {
        let x = -260.0 + index as f64 * 260.0;
        let tray = centered_cube(
            format!("{BIN_PREFIX}_status_lane_{index}_tray"),
            178.0,
            24.0,
            5.0,
        )
        .translate(x, -280.0, deck_top_z() + 12.5);
        let token = centered_cylinder(
            format!("{BIN_PREFIX}_status_lane_{index}_token_marker"),
            STATUS_TOKEN_D / 2.0,
            7.0,
            32,
        )
        .translate(x - 68.0, -280.0, deck_top_z() + 16.5);
        lanes = lanes + tray + token;
    }

    panel + lanes
}

fn camera_evidence_bridge() -> Part {
    let left_upright = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_left_upright"),
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_X / 2.0,
        244.0,
        deck_top_z() + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_upright = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_right_upright"),
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_X / 2.0,
        244.0,
        deck_top_z() + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_top_beam"),
        CAMERA_BRIDGE_X + 28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        0.0,
        244.0,
        deck_top_z() + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );
    let window = centered_cube(
        format!("{BIN_PREFIX}_camera_bridge_evidence_window_frame"),
        CAMERA_WINDOW_X,
        CAMERA_WINDOW_Y,
        8.0,
    )
    .translate(
        0.0,
        244.0,
        deck_top_z() + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 4.0,
    );

    left_upright + right_upright + beam + window + camera_fiducials()
}

fn camera_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{BIN_PREFIX}_camera_fiducials"));
    for index in 0..CAMERA_FIDUCIAL_COUNT {
        let x = centered_index(index % 4, 4, 190.0);
        let y = 244.0 + if index < 4 { -24.0 } else { 24.0 };
        let disk = centered_cylinder(
            format!("{BIN_PREFIX}_camera_fiducial_disk_{index}"),
            CAMERA_FIDUCIAL_D / 2.0,
            3.0,
            32,
        )
        .translate(x, y, deck_top_z() + CAMERA_BRIDGE_Z - 10.0);
        let cross_x = centered_cube(
            format!("{BIN_PREFIX}_camera_fiducial_{index}_cross_x"),
            CAMERA_FIDUCIAL_D + 8.0,
            1.6,
            3.4,
        )
        .translate(x, y, deck_top_z() + CAMERA_BRIDGE_Z - 8.1);
        let cross_y = centered_cube(
            format!("{BIN_PREFIX}_camera_fiducial_{index}_cross_y"),
            1.6,
            CAMERA_FIDUCIAL_D + 8.0,
            3.4,
        )
        .translate(x, y, deck_top_z() + CAMERA_BRIDGE_Z - 8.1);
        fiducials = fiducials + disk + cross_x + cross_y;
    }
    fiducials
}

fn clean_used_segregation() -> Part {
    let rib = centered_cube(
        format!("{BIN_PREFIX}_clean_used_segregation_rib"),
        SEGREGATION_RIB_X,
        SEGREGATION_RIB_Y,
        SEGREGATION_RIB_Z,
    )
    .translate(386.0, 0.0, deck_top_z() + SEGREGATION_RIB_Z / 2.0);
    let clean_tray = segregation_tray("clean_inbound", 306.0, 104.0);
    let used_tray = segregation_tray("used_outbound", 466.0, -104.0);
    let wipe_channel = centered_cube(
        format!("{BIN_PREFIX}_segregation_wipe_channel"),
        SEGREGATION_GAP_X,
        SEGREGATION_RIB_Y - 64.0,
        4.0,
    )
    .translate(386.0, 0.0, deck_top_z() + 2.0);

    rib + clean_tray + used_tray + wipe_channel
}

fn segregation_tray(label: &str, x: f64, y: f64) -> Part {
    let body = centered_cube(
        format!("{BIN_PREFIX}_{label}_cassette_block_tray"),
        CLEAN_USED_TRAY_X,
        CLEAN_USED_TRAY_Y,
        CLEAN_USED_TRAY_Z,
    )
    .translate(x, y, deck_top_z() + CLEAN_USED_TRAY_Z / 2.0);
    let recess = centered_cube(
        format!("{BIN_PREFIX}_{label}_tray_recess"),
        CLEAN_USED_TRAY_X - 30.0,
        CLEAN_USED_TRAY_Y - 28.0,
        8.0,
    )
    .translate(x, y, deck_top_z() + CLEAN_USED_TRAY_Z - 4.0);

    body - recess
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        format!("{BIN_PREFIX}_front_robot_approach_keepout_gauge"),
        ROBOT_APPROACH_KEEP_OUT_X,
        ROBOT_APPROACH_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + ROBOT_APPROACH_KEEP_OUT_Y / 2.0,
        deck_top_z() + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left_service = centered_cube(
        format!("{BIN_PREFIX}_left_service_keepout_gauge"),
        SIDE_SERVICE_KEEP_OUT_X,
        DECK_Y - 142.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -DECK_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X / 2.0,
        0.0,
        deck_top_z() + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_camera = centered_cube(
        format!("{BIN_PREFIX}_rear_camera_service_keepout_gauge"),
        CAMERA_BRIDGE_X,
        REAR_CAMERA_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_CAMERA_SERVICE_KEEP_OUT_Y / 2.0,
        deck_top_z() + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let overhead = centered_cube(
        format!("{BIN_PREFIX}_overhead_robot_wrist_clearance_gauge"),
        218.0,
        92.0,
        12.0,
    )
    .translate(0.0, 66.0, deck_top_z() + OVERHEAD_CLEARANCE_Z);
    let hand_service = centered_cube(
        format!("{BIN_PREFIX}_manual_service_clearance_gauge"),
        250.0,
        48.0,
        8.0,
    )
    .translate(276.0, -230.0, deck_top_z() + HAND_SERVICE_CLEARANCE_Z);

    front_robot + left_service + rear_camera + overhead + hand_service
}

fn deck_mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-500.0, -322.0),
        (-250.0, -322.0),
        (250.0, -322.0),
        (500.0, -322.0),
        (-500.0, 322.0),
        (-250.0, 322.0),
        (250.0, 322.0),
        (500.0, 322.0),
    ]
}

fn leveling_points() -> [(f64, f64); LEVELING_FOOT_COUNT] {
    [
        (
            DATUM_PLATE_POS.0 - DATUM_PLATE_X / 2.0 + 58.0,
            DATUM_PLATE_POS.1 - DATUM_PLATE_Y / 2.0 + 58.0,
        ),
        (
            DATUM_PLATE_POS.0 + DATUM_PLATE_X / 2.0 - 58.0,
            DATUM_PLATE_POS.1 - DATUM_PLATE_Y / 2.0 + 58.0,
        ),
        (
            DATUM_PLATE_POS.0 - DATUM_PLATE_X / 2.0 + 58.0,
            DATUM_PLATE_POS.1 + DATUM_PLATE_Y / 2.0 - 58.0,
        ),
        (
            DATUM_PLATE_POS.0 + DATUM_PLATE_X / 2.0 - 58.0,
            DATUM_PLATE_POS.1 + DATUM_PLATE_Y / 2.0 - 58.0,
        ),
    ]
}

fn datum_pin_points() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (DATUM_PLATE_POS.0 - 320.0, DATUM_PLATE_POS.1 + 178.0),
        (DATUM_PLATE_POS.0 - 80.0, DATUM_PLATE_POS.1 + 178.0),
        (DATUM_PLATE_POS.0 + 320.0, DATUM_PLATE_POS.1 + 178.0),
        (DATUM_PLATE_POS.0 - 320.0, DATUM_PLATE_POS.1 - 178.0),
        (DATUM_PLATE_POS.0 - 80.0, DATUM_PLATE_POS.1 - 178.0),
        (DATUM_PLATE_POS.0 + 320.0, DATUM_PLATE_POS.1 - 178.0),
    ]
}

fn load_pocket_center(index: usize) -> (f64, f64) {
    let col = index % LOAD_GRID_COLS;
    let row = index / LOAD_GRID_COLS;
    (
        DATUM_PLATE_POS.0 + centered_index(col, LOAD_GRID_COLS, LOAD_GRID_PITCH_X),
        DATUM_PLATE_POS.1 + centered_index(row, LOAD_GRID_ROWS, LOAD_GRID_PITCH_Y),
    )
}

fn vial_pose(index: usize) -> (f64, f64, f64) {
    match index {
        0 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1, 0.0),
        1 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1, 90.0),
        2 => (DATUM_PLATE_POS.0 - 302.0, DATUM_PLATE_POS.1, 0.0),
        3 => (DATUM_PLATE_POS.0 + 302.0, DATUM_PLATE_POS.1, 0.0),
        4 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1 - 170.0, 90.0),
        _ => unreachable!("vial index outside declared count"),
    }
}

fn slot_center_x(slot: usize) -> f64 {
    DATUM_PLATE_POS.0 + centered_index(slot, RACK_SLOT_COUNT, SLOT_PITCH_X)
}

fn deflection_gauge_center(index: usize) -> (f64, f64) {
    match index {
        0 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1),
        1 => (DATUM_PLATE_POS.0 - 328.0, DATUM_PLATE_POS.1 - 170.0),
        2 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1 - 170.0),
        3 => (DATUM_PLATE_POS.0 + 328.0, DATUM_PLATE_POS.1 - 170.0),
        4 => (DATUM_PLATE_POS.0 - 328.0, DATUM_PLATE_POS.1 + 170.0),
        5 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1 + 170.0),
        6 => (DATUM_PLATE_POS.0 + 328.0, DATUM_PLATE_POS.1 + 170.0),
        _ => unreachable!("deflection gauge index outside declared count"),
    }
}

fn cassette_block_center(index: usize) -> (f64, f64) {
    match index {
        0 => (DATUM_PLATE_POS.0 - 298.0, DATUM_PLATE_POS.1 - 126.0),
        1 => (DATUM_PLATE_POS.0 + 298.0, DATUM_PLATE_POS.1 - 126.0),
        2 => (DATUM_PLATE_POS.0, DATUM_PLATE_POS.1),
        3 => (DATUM_PLATE_POS.0 - 298.0, DATUM_PLATE_POS.1 + 126.0),
        4 => (DATUM_PLATE_POS.0 + 298.0, DATUM_PLATE_POS.1 + 126.0),
        _ => unreachable!("cassette block index outside declared count"),
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn deck_top_z() -> f64 {
    DECK_Z
}

fn datum_top_z() -> f64 {
    deck_top_z() + DATUM_PLATE_Z - DATUM_RECESS_DEPTH
}

fn footprint_fits_on_deck(footprint: Footprint) -> bool {
    let min_x = footprint.center.0 - footprint.width / 2.0;
    let max_x = footprint.center.0 + footprint.width / 2.0;
    let min_y = footprint.center.1 - footprint.depth / 2.0;
    let max_y = footprint.center.1 + footprint.depth / 2.0;

    min_x > -DECK_X / 2.0 + DECK_RIM_W
        && max_x < DECK_X / 2.0 - DECK_RIM_W
        && min_y > -DECK_Y / 2.0 + DECK_RIM_W
        && max_y < DECK_Y / 2.0 - DECK_RIM_W
}

fn rack_slot_span_x() -> f64 {
    (RACK_SLOT_COUNT as f64 - 1.0) * SLOT_PITCH_X + SLOT_RAIL_X
}

fn required_feature_count() -> usize {
    REQUIRED_FEATURES.len()
}

fn assert_design_sanity() {
    assert_eq!(OUTPUTS.len(), 12, "station export count changed");
    assert_eq!(required_feature_count(), 10);
    assert_eq!(LOAD_POCKET_COUNT, LOAD_GRID_COLS * LOAD_GRID_ROWS);
    assert_eq!(LEVELING_FOOT_COUNT, leveling_points().len());
    assert_eq!(DATUM_PIN_COUNT, datum_pin_points().len());
    assert_eq!(EDGE_BLOCK_COUNT + CENTER_BLOCK_COUNT, CASSETTE_BLOCK_COUNT);
    assert!(rack_slot_span_x() <= SLOT_PITCH_COMB_X - 24.0);
    assert!(OVERHEAD_CLEARANCE_Z > CAMERA_BRIDGE_Z);
    assert!(HAND_SERVICE_CLEARANCE_Z >= 72.0);
    assert!(SEGREGATION_GAP_X > SEGREGATION_RIB_X * 2.0);

    for footprint in COMPONENT_FOOTPRINTS {
        assert!(
            footprint_fits_on_deck(*footprint),
            "{} footprint exceeds deck",
            footprint.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_are_named_for_the_closed_station() {
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output
                .starts_with("output/closed_incubator_shelf_leveling_load_distribution_station_"));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn load_slot_and_cassette_maps_are_complete() {
        assert_eq!(LOAD_POCKET_COUNT, 6);
        assert_eq!(RACK_SLOT_COUNT, 6);
        assert_eq!(CASSETTE_BLOCK_COUNT, 5);
        assert_eq!(EDGE_BLOCK_COUNT, 4);
        assert_eq!(CENTER_BLOCK_COUNT, 1);
    }

    #[test]
    fn datum_geometry_has_clearance_for_shelf_validation() {
        assert!(DATUM_PLATE_X < DECK_X - 180.0);
        assert!(DATUM_PLATE_Y < DECK_Y - 220.0);
        assert!(rack_slot_span_x() < DATUM_PLATE_X - 42.0);
        assert!(DEFLECTION_POST_Z > CASSETTE_BLOCK_Z + 40.0);
    }

    #[test]
    fn all_declared_component_footprints_fit_the_deck() {
        for footprint in COMPONENT_FOOTPRINTS {
            assert!(footprint_fits_on_deck(*footprint), "{}", footprint.name);
        }
    }
}
