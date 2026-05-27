use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Position-randomization and loading tray for a 20-chip culture cassette.
//
// Intent:
// - Randomize incoming chip/cartridge lots against the 4x5 cassette datum before
//   loading so cassette position effects can be separated from chip lot effects.
// - Keep clean chip staging, cartridge/card handling, and rejected/used handling
//   physically separated on one robot-accessible tray.
// - Provide barcode/lot label lands, position fiducials, and explicit robot pick
//   clearance envelopes for integration into the closed workcell deck.
//
// This is mechanical workflow geometry only. The randomized assignment sequence,
// lot release rules, and acceptance criteria remain separate lab records/SOPs.

const OUTPUTS: [&str; 11] = [
    "output/chip_cassette_position_randomization_tray_base_tray.stl",
    "output/chip_cassette_position_randomization_tray_cassette_datum.stl",
    "output/chip_cassette_position_randomization_tray_assignment_card_slots.stl",
    "output/chip_cassette_position_randomization_tray_clean_chip_staging_pockets.stl",
    "output/chip_cassette_position_randomization_tray_cartridge_staging_pockets.stl",
    "output/chip_cassette_position_randomization_tray_barcode_lot_label_lands.stl",
    "output/chip_cassette_position_randomization_tray_rejected_chip_pocket.stl",
    "output/chip_cassette_position_randomization_tray_position_fiducials.stl",
    "output/chip_cassette_position_randomization_tray_robot_pick_clearances.stl",
    "output/chip_cassette_position_randomization_tray_clean_used_segregation.stl",
    "output/chip_cassette_position_randomization_tray_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const POSITION_COUNT: usize = COLS * ROWS;
const RANDOMIZED_ASSIGNMENT: [usize; POSITION_COUNT] = [
    7, 12, 1, 18, 4, 15, 0, 9, 16, 3, 11, 6, 19, 2, 14, 5, 17, 8, 13, 10,
];

const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;
const CASSETTE_CENTER_X: f64 = 148.0;
const CASSETTE_CENTER_Y: f64 = 18.0;

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const TRAY_X: f64 = 980.0;
const TRAY_Y: f64 = 680.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 26.0;
const SOCKET_DEPTH: f64 = 4.5;

const DATUM_RAIL_W: f64 = 14.0;
const DATUM_RAIL_Z: f64 = 30.0;
const POSITION_FIDUCIAL_D: f64 = 9.0;

const CLEAN_RACK_CENTER_X: f64 = -310.0;
const CLEAN_RACK_CENTER_Y: f64 = 52.0;
const CLEAN_RACK_X: f64 = 300.0;
const CLEAN_RACK_Y: f64 = 250.0;
const CLEAN_RACK_Z: f64 = 32.0;
const CLEAN_STAGING_COLS: usize = 4;
const CLEAN_STAGING_ROWS: usize = 5;
const CLEAN_POCKET_X: f64 = 54.0;
const CLEAN_POCKET_Y: f64 = 36.0;
const CLEAN_POCKET_DEPTH: f64 = 20.0;
const CLEAN_PITCH_X: f64 = 68.0;
const CLEAN_PITCH_Y: f64 = 48.0;

const CARD_RACK_CENTER_X: f64 = -310.0;
const CARD_RACK_CENTER_Y: f64 = -257.0;
const CARD_RACK_X: f64 = 300.0;
const CARD_RACK_Y: f64 = 108.0;
const CARD_RACK_Z: f64 = 36.0;
const ASSIGNMENT_CARD_COLS: usize = 10;
const ASSIGNMENT_CARD_ROWS: usize = 2;
const CARD_SLOT_PITCH_X: f64 = 26.0;
const CARD_SLOT_PITCH_Y: f64 = 42.0;
const CARD_SLOT_W: f64 = 4.2;
const CARD_SLOT_Y: f64 = 30.0;

const CARTRIDGE_RACK_CENTER_X: f64 = -310.0;
const CARTRIDGE_RACK_CENTER_Y: f64 = 260.0;
const CARTRIDGE_RACK_X: f64 = 300.0;
const CARTRIDGE_RACK_Y: f64 = 116.0;
const CARTRIDGE_RACK_Z: f64 = 34.0;
const CARTRIDGE_SLOT_COLS: usize = 10;
const CARTRIDGE_SLOT_ROWS: usize = 2;
const CARTRIDGE_SLOT_X: f64 = 16.0;
const CARTRIDGE_SLOT_Y: f64 = 36.0;
const CARTRIDGE_SLOT_PITCH_X: f64 = 26.0;
const CARTRIDGE_SLOT_PITCH_Y: f64 = 44.0;

const REJECT_CENTER_X: f64 = 390.0;
const REJECT_CENTER_Y: f64 = -295.0;
const REJECT_POCKET_X: f64 = 150.0;
const REJECT_POCKET_Y: f64 = 78.0;
const REJECT_POCKET_Z: f64 = 40.0;

const LABEL_LAND_Z: f64 = 4.0;
const LABEL_LAND_Y: f64 = 24.0;
const LABEL_LAND_COUNT: usize = POSITION_COUNT + 5;

const SEGREGATION_DIVIDER_W: f64 = 18.0;
const SEGREGATION_AIR_GAP: f64 = 28.0;
const SEGREGATION_WALL_Z: f64 = 52.0;

const ROBOT_PICK_CLEARANCE_Z: f64 = 92.0;
const ROBOT_GRIPPER_SIDE_CLEARANCE: f64 = 12.0;
const ROBOT_CASSETTE_CLEARANCE_MARGIN: f64 = 34.0;
const ROBOT_STAGING_CLEARANCE_MARGIN: f64 = 24.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let cassette = cassette_datum();
    export(OUTPUTS[1], &cassette);

    let cards = assignment_card_slots();
    export(OUTPUTS[2], &cards);

    let clean_staging = clean_chip_staging_pockets();
    export(OUTPUTS[3], &clean_staging);

    let cartridge_staging = cartridge_staging_pockets();
    export(OUTPUTS[4], &cartridge_staging);

    let labels = barcode_lot_label_lands();
    export(OUTPUTS[5], &labels);

    let reject = rejected_chip_pocket();
    export(OUTPUTS[6], &reject);

    let fiducials = position_fiducials();
    export(OUTPUTS[7], &fiducials);

    let clearances = robot_pick_clearances();
    export(OUTPUTS[8], &clearances);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let assembly = base
        + cassette
        + cards
        + clean_staging
        + cartridge_staging
        + labels
        + reject
        + fiducials
        + clearances
        + segregation;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Chip cassette position randomization tray:");
    println!("  Tray footprint:              {TRAY_X:.0}mm x {TRAY_Y:.0}mm x {BASE_Z:.0}mm base");
    println!(
        "  Cassette datum:              {COLS}x{ROWS} positions, {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm datum envelope"
    );
    println!(
        "  Assignment capacity:         {POSITION_COUNT} randomized card slots and {POSITION_COUNT} cartridge staging slots"
    );
    println!(
        "  Clean chip staging:          {POSITION_COUNT} pockets at {CLEAN_PITCH_X:.0}mm x {CLEAN_PITCH_Y:.0}mm pitch"
    );
    println!(
        "  Rejected-chip handling:      {REJECT_POCKET_X:.0}mm x {REJECT_POCKET_Y:.0}mm isolated pocket"
    );
    println!(
        "  Robot clearance envelopes:   {ROBOT_PICK_CLEARANCE_Z:.0}mm Z clearance with {ROBOT_GRIPPER_SIDE_CLEARANCE:.0}mm minimum side clearance"
    );
    println!(
        "  Traceability:                {LABEL_LAND_COUNT} barcode/lot label lands plus 20 position fiducials"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(POSITION_COUNT, 20);
    assert!(cassette_left_edge() > clean_zone_right_edge());
    assert!(reject_left_edge() - clean_zone_right_edge() >= SEGREGATION_AIR_GAP);
    assert!(assignment_capacity() >= POSITION_COUNT);
    assert!(cartridge_capacity() >= POSITION_COUNT);
    assert!(ROBOT_PICK_CLEARANCE_Z > BASE_Z + REVC_TOTAL_HEIGHT + 45.0);
}

fn base_tray() -> Part {
    let deck = centered_cube("chip_randomization_tray_base_deck", TRAY_X, TRAY_Y, BASE_Z)
        .translate(0.0, 0.0, BASE_Z / 2.0);

    let cassette_socket = centered_cube(
        "chip_randomization_tray_cassette_socket_recess",
        CASSETTE_X + 22.0,
        CASSETTE_Y + 22.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let clean_socket = centered_cube(
        "chip_randomization_tray_clean_staging_socket",
        CLEAN_RACK_X + 8.0,
        CLEAN_RACK_Y + 8.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CLEAN_RACK_CENTER_X,
        CLEAN_RACK_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let card_socket = centered_cube(
        "chip_randomization_tray_assignment_card_socket",
        CARD_RACK_X + 8.0,
        CARD_RACK_Y + 8.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CARD_RACK_CENTER_X,
        CARD_RACK_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let cartridge_socket = centered_cube(
        "chip_randomization_tray_cartridge_staging_socket",
        CARTRIDGE_RACK_X + 8.0,
        CARTRIDGE_RACK_Y + 8.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CARTRIDGE_RACK_CENTER_X,
        CARTRIDGE_RACK_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let reject_socket = centered_cube(
        "chip_randomization_tray_reject_pocket_socket",
        REJECT_POCKET_X + 8.0,
        REJECT_POCKET_Y + 8.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    let drain_channel = centered_cube(
        "chip_randomization_tray_front_washdown_channel",
        TRAY_X - 94.0,
        8.0,
        6.0,
    )
    .translate(0.0, -TRAY_Y / 2.0 + 42.0, BASE_Z - 3.0);

    let drain_port = centered_cylinder("chip_randomization_tray_drain_port", 5.0, 34.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(TRAY_X / 2.0 - 52.0, -TRAY_Y / 2.0 + 22.0, BASE_Z - 7.0);

    deck - cassette_socket
        - clean_socket
        - card_socket
        - cartridge_socket
        - reject_socket
        - drain_channel
        - drain_port
        - tray_mount_holes()
        + perimeter_rims()
        + underside_robot_datum_rails()
}

fn tray_mount_holes() -> Part {
    let mut holes = Part::empty("chip_randomization_tray_mount_holes");
    for (i, (x, y)) in [
        (-(TRAY_X / 2.0 - 36.0), -(TRAY_Y / 2.0 - 34.0)),
        (TRAY_X / 2.0 - 36.0, -(TRAY_Y / 2.0 - 34.0)),
        (-(TRAY_X / 2.0 - 36.0), TRAY_Y / 2.0 - 34.0),
        (TRAY_X / 2.0 - 36.0, TRAY_Y / 2.0 - 34.0),
        (0.0, -(TRAY_Y / 2.0 - 34.0)),
        (0.0, TRAY_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("chip_randomization_tray_m5_clearance_{i}"),
                5.4 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "chip_randomization_tray_left_rim",
        RIM_W,
        TRAY_Y - 58.0,
        RIM_Z,
    )
    .translate(-(TRAY_X / 2.0 - RIM_W / 2.0), 8.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "chip_randomization_tray_right_rim",
        RIM_W,
        TRAY_Y - 58.0,
        RIM_Z,
    )
    .translate(TRAY_X / 2.0 - RIM_W / 2.0, 8.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "chip_randomization_tray_rear_rim",
        TRAY_X - 32.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, TRAY_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front_left = centered_cube(
        "chip_randomization_tray_front_clean_low_lip",
        TRAY_X * 0.46,
        12.0,
        18.0,
    )
    .translate(-(TRAY_X * 0.22), -TRAY_Y / 2.0 + 20.0, BASE_Z + 9.0);
    let front_right = centered_cube(
        "chip_randomization_tray_front_reject_low_lip",
        TRAY_X * 0.32,
        12.0,
        22.0,
    )
    .translate(TRAY_X * 0.28, -TRAY_Y / 2.0 + 20.0, BASE_Z + 11.0);

    left + right + rear + front_left + front_right
}

fn underside_robot_datum_rails() -> Part {
    let left = centered_cube(
        "chip_randomization_tray_underside_left_robot_datum_rail",
        18.0,
        TRAY_Y - 94.0,
        10.0,
    )
    .translate(-(TRAY_X / 2.0 - 64.0), 0.0, 5.0);
    let right = centered_cube(
        "chip_randomization_tray_underside_right_robot_datum_rail",
        18.0,
        TRAY_Y - 94.0,
        10.0,
    )
    .translate(TRAY_X / 2.0 - 64.0, 0.0, 5.0);
    let rear_stop = centered_cube(
        "chip_randomization_tray_underside_rear_robot_stop",
        TRAY_X - 156.0,
        18.0,
        10.0,
    )
    .translate(0.0, TRAY_Y / 2.0 - 58.0, 5.0);

    left + right + rear_stop
}

fn cassette_datum() -> Part {
    let left_stop = centered_cube(
        "chip_randomization_cassette_left_x_datum",
        DATUM_RAIL_W,
        CASSETTE_Y + 30.0,
        DATUM_RAIL_Z,
    )
    .translate(
        CASSETTE_CENTER_X - CASSETTE_X / 2.0 - DATUM_RAIL_W / 2.0 - 2.0,
        CASSETTE_CENTER_Y,
        BASE_Z + DATUM_RAIL_Z / 2.0,
    );

    let rear_stop = centered_cube(
        "chip_randomization_cassette_rear_y_datum",
        CASSETTE_X + 36.0,
        DATUM_RAIL_W,
        DATUM_RAIL_Z,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 + DATUM_RAIL_W / 2.0 + 2.0,
        BASE_Z + DATUM_RAIL_Z / 2.0,
    );

    let right_soft_rail = centered_cube(
        "chip_randomization_cassette_right_soft_rail",
        DATUM_RAIL_W,
        CASSETTE_Y + 18.0,
        DATUM_RAIL_Z * 0.6,
    )
    .translate(
        CASSETTE_CENTER_X + CASSETTE_X / 2.0 + DATUM_RAIL_W / 2.0 + 2.0,
        CASSETTE_CENTER_Y,
        BASE_Z + DATUM_RAIL_Z * 0.3,
    );

    let front_low_lip = centered_cube(
        "chip_randomization_cassette_front_low_lip",
        CASSETTE_X + 36.0,
        10.0,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 - 7.0,
        BASE_Z + 9.0,
    );

    left_stop + rear_stop + right_soft_rail + front_low_lip + cassette_position_grid_ribs()
        - cassette_datum_pin_holes()
}

fn cassette_position_grid_ribs() -> Part {
    let mut ribs = Part::empty("chip_randomization_cassette_position_grid_ribs");

    for col in 1..COLS {
        let x = cassette_left_edge()
            + CASSETTE_MARGIN_X
            + col as f64 * REVC_CHIP_LENGTH
            + (col as f64 - 0.5) * GUTTER;
        ribs = ribs
            + centered_cube(
                format!("chip_randomization_cassette_column_separator_{col}"),
                3.0,
                ARRAY_Y + 8.0,
                10.0,
            )
            .translate(x, CASSETTE_CENTER_Y, BASE_Z + 5.0);
    }

    for row in 1..ROWS {
        let y = cassette_bottom_edge()
            + CASSETTE_MARGIN_Y
            + row as f64 * REVC_CHIP_WIDTH
            + (row as f64 - 0.5) * GUTTER;
        ribs = ribs
            + centered_cube(
                format!("chip_randomization_cassette_row_separator_{row}"),
                ARRAY_X + 8.0,
                3.0,
                10.0,
            )
            .translate(CASSETTE_CENTER_X, y, BASE_Z + 5.0);
    }

    ribs
}

fn cassette_datum_pin_holes() -> Part {
    let mut holes = Part::empty("chip_randomization_cassette_datum_pin_holes");
    for (i, (x, y)) in [
        (
            cassette_left_edge() + 24.0,
            cassette_bottom_edge() + CASSETTE_Y - 24.0,
        ),
        (
            cassette_left_edge() + CASSETTE_X - 24.0,
            cassette_bottom_edge() + CASSETTE_Y - 24.0,
        ),
        (cassette_left_edge() + 24.0, cassette_bottom_edge() + 24.0),
        (
            cassette_left_edge() + CASSETTE_X - 24.0,
            cassette_bottom_edge() + 24.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("chip_randomization_cassette_datum_pin_clearance_{i}"),
                6.0 / 2.0,
                DATUM_RAIL_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z + DATUM_RAIL_Z / 2.0);
    }
    holes
}

fn assignment_card_slots() -> Part {
    let body = centered_cube(
        "chip_randomization_assignment_card_rack_body",
        CARD_RACK_X,
        CARD_RACK_Y,
        CARD_RACK_Z,
    )
    .translate(
        CARD_RACK_CENTER_X,
        CARD_RACK_CENTER_Y,
        BASE_Z + CARD_RACK_Z / 2.0,
    );

    let mut cuts = Part::empty("chip_randomization_assignment_card_slot_cuts");
    let mut tabs = Part::empty("chip_randomization_assignment_randomized_tabs");
    for slot in 0..POSITION_COUNT {
        let (x, y) = assignment_slot_center(slot);
        let assigned = RANDOMIZED_ASSIGNMENT[slot] + 1;
        cuts = cuts
            + centered_cube(
                format!("chip_randomization_assignment_slot_{slot}_to_position_{assigned}"),
                CARD_SLOT_W,
                CARD_SLOT_Y,
                CARD_RACK_Z + 4.0,
            )
            .translate(x, y, BASE_Z + CARD_RACK_Z / 2.0)
            + centered_cube(
                format!("chip_randomization_assignment_card_finger_relief_{slot}"),
                12.0,
                9.0,
                12.0,
            )
            .translate(x, y - CARD_SLOT_Y / 2.0 - 2.0, BASE_Z + CARD_RACK_Z - 5.0);

        tabs = tabs
            + centered_cube(
                format!("chip_randomization_assignment_slot_{slot}_position_{assigned}_tab"),
                12.0,
                6.0,
                8.0,
            )
            .translate(x, y + CARD_SLOT_Y / 2.0 + 7.0, BASE_Z + CARD_RACK_Z + 4.0);
    }

    body - cuts + tabs + rack_gripper_ears("assignment_card", CARD_RACK_CENTER_X, CARD_RACK_Y)
}

fn clean_chip_staging_pockets() -> Part {
    let body = centered_cube(
        "chip_randomization_clean_chip_staging_body",
        CLEAN_RACK_X,
        CLEAN_RACK_Y,
        CLEAN_RACK_Z,
    )
    .translate(
        CLEAN_RACK_CENTER_X,
        CLEAN_RACK_CENTER_Y,
        BASE_Z + CLEAN_RACK_Z / 2.0,
    );

    let mut pockets = Part::empty("chip_randomization_clean_chip_staging_pocket_cuts");
    let mut pocket_lips = Part::empty("chip_randomization_clean_chip_staging_pocket_lips");
    for row in 0..CLEAN_STAGING_ROWS {
        for col in 0..CLEAN_STAGING_COLS {
            let index = row * CLEAN_STAGING_COLS + col;
            let (x, y) = clean_staging_center(col, row);
            pockets = pockets
                + centered_cube(
                    format!("chip_randomization_clean_chip_pocket_{index}"),
                    CLEAN_POCKET_X,
                    CLEAN_POCKET_Y,
                    CLEAN_POCKET_DEPTH,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + CLEAN_RACK_Z - CLEAN_POCKET_DEPTH / 2.0 + 0.2,
                )
                + centered_cube(
                    format!("chip_randomization_clean_chip_pocket_robot_finger_gap_{index}"),
                    CLEAN_POCKET_X + 10.0,
                    9.0,
                    CLEAN_RACK_Z + 2.0,
                )
                .translate(
                    x,
                    y - CLEAN_POCKET_Y / 2.0,
                    BASE_Z + CLEAN_RACK_Z / 2.0,
                );

            pocket_lips = pocket_lips
                + centered_cube(
                    format!("chip_randomization_clean_chip_pocket_rear_lip_{index}"),
                    CLEAN_POCKET_X + 6.0,
                    5.0,
                    9.0,
                )
                .translate(
                    x,
                    y + CLEAN_POCKET_Y / 2.0 + 4.0,
                    BASE_Z + CLEAN_RACK_Z + 4.5,
                );
        }
    }

    body - pockets
        + pocket_lips
        + rack_gripper_ears("clean_chip_staging", CLEAN_RACK_CENTER_X, CLEAN_RACK_Y)
}

fn cartridge_staging_pockets() -> Part {
    let body = centered_cube(
        "chip_randomization_cartridge_staging_body",
        CARTRIDGE_RACK_X,
        CARTRIDGE_RACK_Y,
        CARTRIDGE_RACK_Z,
    )
    .translate(
        CARTRIDGE_RACK_CENTER_X,
        CARTRIDGE_RACK_CENTER_Y,
        BASE_Z + CARTRIDGE_RACK_Z / 2.0,
    );

    let mut cuts = Part::empty("chip_randomization_cartridge_staging_slot_cuts");
    let mut keepers = Part::empty("chip_randomization_cartridge_staging_slot_keepers");
    for slot in 0..POSITION_COUNT {
        let (x, y) = cartridge_slot_center(slot);
        cuts = cuts
            + centered_cube(
                format!("chip_randomization_cartridge_slot_{slot}"),
                CARTRIDGE_SLOT_X,
                CARTRIDGE_SLOT_Y,
                CARTRIDGE_RACK_Z + 4.0,
            )
            .translate(x, y, BASE_Z + CARTRIDGE_RACK_Z / 2.0)
            + centered_cube(
                format!("chip_randomization_cartridge_key_relief_{slot}"),
                8.0,
                10.0,
                CARTRIDGE_RACK_Z + 6.0,
            )
            .translate(
                x + 7.0,
                y + CARTRIDGE_SLOT_Y / 2.0 - 5.0,
                BASE_Z + CARTRIDGE_RACK_Z / 2.0,
            );

        keepers = keepers
            + centered_cube(
                format!("chip_randomization_cartridge_slot_front_keeper_{slot}"),
                CARTRIDGE_SLOT_X + 8.0,
                5.0,
                12.0,
            )
            .translate(
                x,
                y - CARTRIDGE_SLOT_Y / 2.0 - 4.0,
                BASE_Z + CARTRIDGE_RACK_Z + 6.0,
            );
    }

    body - cuts
        + keepers
        + rack_gripper_ears(
            "cartridge_staging",
            CARTRIDGE_RACK_CENTER_X,
            CARTRIDGE_RACK_Y,
        )
}

fn barcode_lot_label_lands() -> Part {
    let mut lands = Part::empty("chip_randomization_barcode_lot_label_lands");

    for slot in 0..POSITION_COUNT {
        let (slot_x, slot_y) = assignment_slot_center(slot);
        lands = lands
            + centered_cube(
                format!("chip_randomization_assignment_card_barcode_land_{slot}"),
                18.0,
                10.0,
                LABEL_LAND_Z,
            )
            .translate(
                slot_x,
                slot_y - 19.0,
                BASE_Z + CARD_RACK_Z + LABEL_LAND_Z / 2.0,
            );
    }

    for (i, (name, x, y, width)) in [
        ("tray_lot", 0.0, -TRAY_Y / 2.0 + 62.0, 190.0),
        (
            "cassette_lot",
            CASSETTE_CENTER_X,
            CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 + 38.0,
            210.0,
        ),
        (
            "clean_staging_lot",
            CLEAN_RACK_CENTER_X,
            CLEAN_RACK_CENTER_Y - CLEAN_RACK_Y / 2.0 + 22.0,
            194.0,
        ),
        (
            "cartridge_lot",
            CARTRIDGE_RACK_CENTER_X,
            CARTRIDGE_RACK_CENTER_Y + CARTRIDGE_RACK_Y / 2.0 - 20.0,
            194.0,
        ),
        (
            "reject_lot",
            REJECT_CENTER_X,
            REJECT_CENTER_Y - REJECT_POCKET_Y / 2.0 - 20.0,
            132.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("chip_randomization_{name}_barcode_label_land_{i}"),
                *width,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(*x, *y, BASE_Z + LABEL_LAND_Z / 2.0);
    }

    lands
}

fn rejected_chip_pocket() -> Part {
    let body = centered_cube(
        "chip_randomization_rejected_chip_pocket_body",
        REJECT_POCKET_X,
        REJECT_POCKET_Y,
        REJECT_POCKET_Z,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y,
        BASE_Z + REJECT_POCKET_Z / 2.0,
    );

    let basin = centered_cube(
        "chip_randomization_rejected_chip_recessed_basin",
        REJECT_POCKET_X - 30.0,
        REJECT_POCKET_Y - 26.0,
        REJECT_POCKET_Z - 10.0,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y,
        BASE_Z + REJECT_POCKET_Z / 2.0 + 6.0,
    );

    let front_drop_slot = centered_cube(
        "chip_randomization_rejected_chip_front_drop_slot",
        REJECT_POCKET_X - 42.0,
        12.0,
        REJECT_POCKET_Z + 4.0,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y - REJECT_POCKET_Y / 2.0 + 8.0,
        BASE_Z + REJECT_POCKET_Z / 2.0,
    );

    let rear_guard = centered_cube(
        "chip_randomization_rejected_chip_rear_guard",
        REJECT_POCKET_X + 10.0,
        14.0,
        REJECT_POCKET_Z + 24.0,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y + REJECT_POCKET_Y / 2.0 + 5.0,
        BASE_Z + REJECT_POCKET_Z / 2.0 + 12.0,
    );
    let side_guard = centered_cube(
        "chip_randomization_rejected_chip_inboard_guard",
        12.0,
        REJECT_POCKET_Y + 22.0,
        REJECT_POCKET_Z + 12.0,
    )
    .translate(
        REJECT_CENTER_X - REJECT_POCKET_X / 2.0 - 4.0,
        REJECT_CENTER_Y,
        BASE_Z + REJECT_POCKET_Z / 2.0 + 6.0,
    );

    body - basin - front_drop_slot + rear_guard + side_guard
}

fn position_fiducials() -> Part {
    let mut fiducials = Part::empty("chip_randomization_position_fiducials");

    for row in 0..ROWS {
        for col in 0..COLS {
            let position = position_index(col, row);
            let randomized_slot = randomized_slot_for_position(position);
            let (x, y) = cassette_position_center(col, row);
            let ring = centered_cylinder(
                format!(
                    "chip_randomization_position_{position}_fiducial_ring_slot_{randomized_slot}"
                ),
                POSITION_FIDUCIAL_D / 2.0,
                3.0,
                28,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 14.0,
                y + REVC_CHIP_WIDTH / 2.0 - 14.0,
                BASE_Z + 1.5,
            );
            let center = centered_cylinder(
                format!(
                    "chip_randomization_position_{position}_fiducial_center_slot_{randomized_slot}"
                ),
                2.4 / 2.0,
                4.0,
                18,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 14.0,
                y + REVC_CHIP_WIDTH / 2.0 - 14.0,
                BASE_Z + 1.5,
            );
            fiducials = fiducials + (ring - center);
        }
    }

    for (i, (x, y)) in [
        (cassette_left_edge() + 22.0, cassette_bottom_edge() + 22.0),
        (
            cassette_left_edge() + CASSETTE_X - 22.0,
            cassette_bottom_edge() + 22.0,
        ),
        (
            cassette_left_edge() + 22.0,
            cassette_bottom_edge() + CASSETTE_Y - 22.0,
        ),
        (
            cassette_left_edge() + CASSETTE_X - 22.0,
            cassette_bottom_edge() + CASSETTE_Y - 22.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        fiducials =
            fiducials
                + fiducial_crosshair(format!("chip_randomization_outer_datum_fiducial_{i}"))
                    .translate(*x, *y, BASE_Z + 2.0);
    }

    fiducials
}

fn fiducial_crosshair(name: String) -> Part {
    let ring = centered_cylinder(format!("{name}_ring"), 7.0, 3.0, 32);
    let bore = centered_cylinder(format!("{name}_bore"), 2.2, 4.0, 24);
    let x_bar = centered_cube(format!("{name}_x_bar"), 22.0, 2.0, 2.0);
    let y_bar = centered_cube(format!("{name}_y_bar"), 2.0, 22.0, 2.0);
    ring - bore + x_bar + y_bar
}

fn robot_pick_clearances() -> Part {
    let cassette = clearance_box(
        "chip_randomization_robot_cassette_pick_clearance",
        CASSETTE_X + 2.0 * ROBOT_CASSETTE_CLEARANCE_MARGIN,
        CASSETTE_Y + 2.0 * ROBOT_CASSETTE_CLEARANCE_MARGIN,
        ROBOT_PICK_CLEARANCE_Z,
        8.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        BASE_Z + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );

    let clean = clearance_box(
        "chip_randomization_robot_clean_staging_pick_clearance",
        CLEAN_RACK_X + 2.0 * ROBOT_STAGING_CLEARANCE_MARGIN,
        CLEAN_RACK_Y + 2.0 * ROBOT_STAGING_CLEARANCE_MARGIN,
        ROBOT_PICK_CLEARANCE_Z * 0.78,
        7.0,
    )
    .translate(
        CLEAN_RACK_CENTER_X,
        CLEAN_RACK_CENTER_Y,
        BASE_Z + ROBOT_PICK_CLEARANCE_Z * 0.39,
    );

    let reject = clearance_box(
        "chip_randomization_robot_reject_drop_clearance",
        REJECT_POCKET_X + 56.0,
        REJECT_POCKET_Y + 54.0,
        ROBOT_PICK_CLEARANCE_Z * 0.72,
        7.0,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y,
        BASE_Z + ROBOT_PICK_CLEARANCE_Z * 0.36,
    );

    cassette + clean + reject
}

fn clean_used_segregation() -> Part {
    let clean_lane_marker = centered_cube(
        "chip_randomization_clean_lane_floor_marker",
        CLEAN_RACK_X - 26.0,
        CLEAN_RACK_Y + CARD_RACK_Y + CARTRIDGE_RACK_Y + 26.0,
        3.0,
    )
    .translate(CLEAN_RACK_CENTER_X, 30.0, BASE_Z + 1.5);

    let front_used_guard = centered_cube(
        "chip_randomization_used_reject_lane_front_guard",
        REJECT_POCKET_X + 42.0,
        SEGREGATION_DIVIDER_W,
        SEGREGATION_WALL_Z,
    )
    .translate(
        REJECT_CENTER_X,
        REJECT_CENTER_Y - REJECT_POCKET_Y / 2.0 - SEGREGATION_DIVIDER_W / 2.0 - 4.0,
        BASE_Z + SEGREGATION_WALL_Z / 2.0,
    );

    let inboard_used_wall = centered_cube(
        "chip_randomization_clean_used_inboard_segregation_wall",
        SEGREGATION_DIVIDER_W,
        REJECT_POCKET_Y + 118.0,
        SEGREGATION_WALL_Z,
    )
    .translate(
        REJECT_CENTER_X - REJECT_POCKET_X / 2.0 - SEGREGATION_AIR_GAP - SEGREGATION_DIVIDER_W / 2.0,
        REJECT_CENTER_Y + 8.0,
        BASE_Z + SEGREGATION_WALL_Z / 2.0,
    );

    let reject_lane_marker = centered_cube(
        "chip_randomization_used_reject_floor_marker",
        REJECT_POCKET_X + 50.0,
        REJECT_POCKET_Y + 56.0,
        3.0,
    )
    .translate(REJECT_CENTER_X, REJECT_CENTER_Y, BASE_Z + 1.5);

    let transfer_gap_marker = centered_cube(
        "chip_randomization_clean_used_air_gap_marker",
        SEGREGATION_AIR_GAP,
        REJECT_POCKET_Y + 94.0,
        5.0,
    )
    .translate(
        REJECT_CENTER_X - REJECT_POCKET_X / 2.0 - SEGREGATION_AIR_GAP / 2.0,
        REJECT_CENTER_Y + 8.0,
        BASE_Z + 2.5,
    );

    clean_lane_marker + front_used_guard + inboard_used_wall + reject_lane_marker
        - transfer_gap_marker
}

fn rack_gripper_ears(name: &str, center_x: f64, rack_y: f64) -> Part {
    let left = centered_cube(
        format!("chip_randomization_{name}_left_robot_gripper_ear"),
        34.0,
        14.0,
        14.0,
    )
    .translate(
        center_x - 96.0,
        CARD_RACK_CENTER_Y.min(CLEAN_RACK_CENTER_Y) - rack_y / 2.0 + 18.0,
        BASE_Z + 28.0,
    );
    let right = centered_cube(
        format!("chip_randomization_{name}_right_robot_gripper_ear"),
        34.0,
        14.0,
        14.0,
    )
    .translate(
        center_x + 96.0,
        CARD_RACK_CENTER_Y.min(CLEAN_RACK_CENTER_Y) - rack_y / 2.0 + 18.0,
        BASE_Z + 28.0,
    );
    left + right
}

fn clearance_box(name: &str, x: f64, y: f64, z: f64, rib: f64) -> Part {
    let x_front = centered_cube(format!("{name}_front_x_rib"), x, rib, rib).translate(
        0.0,
        -y / 2.0,
        -z / 2.0,
    );
    let x_rear =
        centered_cube(format!("{name}_rear_x_rib"), x, rib, rib).translate(0.0, y / 2.0, -z / 2.0);
    let x_top_front = centered_cube(format!("{name}_top_front_x_rib"), x, rib, rib).translate(
        0.0,
        -y / 2.0,
        z / 2.0,
    );
    let x_top_rear = centered_cube(format!("{name}_top_rear_x_rib"), x, rib, rib).translate(
        0.0,
        y / 2.0,
        z / 2.0,
    );

    let y_left =
        centered_cube(format!("{name}_left_y_rib"), rib, y, rib).translate(-x / 2.0, 0.0, -z / 2.0);
    let y_right =
        centered_cube(format!("{name}_right_y_rib"), rib, y, rib).translate(x / 2.0, 0.0, -z / 2.0);
    let y_top_left = centered_cube(format!("{name}_top_left_y_rib"), rib, y, rib).translate(
        -x / 2.0,
        0.0,
        z / 2.0,
    );
    let y_top_right = centered_cube(format!("{name}_top_right_y_rib"), rib, y, rib).translate(
        x / 2.0,
        0.0,
        z / 2.0,
    );

    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (px, py)) in [
        (-x / 2.0, -y / 2.0),
        (x / 2.0, -y / 2.0),
        (-x / 2.0, y / 2.0),
        (x / 2.0, y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{i}"), rib, rib, z)
                .translate(*px, *py, 0.0);
    }

    x_front
        + x_rear
        + x_top_front
        + x_top_rear
        + y_left
        + y_right
        + y_top_left
        + y_top_right
        + posts
}

fn cassette_position_center(col: usize, row: usize) -> (f64, f64) {
    (
        cassette_left_edge() + CASSETTE_MARGIN_X + REVC_CHIP_LENGTH / 2.0 + col as f64 * PITCH_X,
        cassette_bottom_edge() + CASSETTE_MARGIN_Y + REVC_CHIP_WIDTH / 2.0 + row as f64 * PITCH_Y,
    )
}

fn assignment_slot_center(slot: usize) -> (f64, f64) {
    let row = slot / ASSIGNMENT_CARD_COLS;
    let col = slot % ASSIGNMENT_CARD_COLS;
    (
        CARD_RACK_CENTER_X + lane_position(col, ASSIGNMENT_CARD_COLS, CARD_SLOT_PITCH_X),
        CARD_RACK_CENTER_Y + lane_position(row, ASSIGNMENT_CARD_ROWS, CARD_SLOT_PITCH_Y),
    )
}

fn cartridge_slot_center(slot: usize) -> (f64, f64) {
    let row = slot / CARTRIDGE_SLOT_COLS;
    let col = slot % CARTRIDGE_SLOT_COLS;
    (
        CARTRIDGE_RACK_CENTER_X + lane_position(col, CARTRIDGE_SLOT_COLS, CARTRIDGE_SLOT_PITCH_X),
        CARTRIDGE_RACK_CENTER_Y + lane_position(row, CARTRIDGE_SLOT_ROWS, CARTRIDGE_SLOT_PITCH_Y),
    )
}

fn clean_staging_center(col: usize, row: usize) -> (f64, f64) {
    (
        CLEAN_RACK_CENTER_X + lane_position(col, CLEAN_STAGING_COLS, CLEAN_PITCH_X),
        CLEAN_RACK_CENTER_Y + lane_position(row, CLEAN_STAGING_ROWS, CLEAN_PITCH_Y),
    )
}

fn randomized_slot_for_position(position: usize) -> usize {
    RANDOMIZED_ASSIGNMENT
        .iter()
        .position(|assigned| *assigned == position)
        .expect("position must be present in randomized assignment")
}

fn position_index(col: usize, row: usize) -> usize {
    row * COLS + col
}

fn assignment_capacity() -> usize {
    ASSIGNMENT_CARD_COLS * ASSIGNMENT_CARD_ROWS
}

fn cartridge_capacity() -> usize {
    CARTRIDGE_SLOT_COLS * CARTRIDGE_SLOT_ROWS
}

fn cassette_left_edge() -> f64 {
    CASSETTE_CENTER_X - CASSETTE_X / 2.0
}

fn cassette_bottom_edge() -> f64 {
    CASSETTE_CENTER_Y - CASSETTE_Y / 2.0
}

fn clean_zone_right_edge() -> f64 {
    CLEAN_RACK_CENTER_X + CLEAN_RACK_X / 2.0
}

fn reject_left_edge() -> f64 {
    REJECT_CENTER_X - REJECT_POCKET_X / 2.0
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn cassette_datum_has_twenty_positions() {
        let mut positions = BTreeSet::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                positions.insert(position_index(col, row));
                let (x, y) = cassette_position_center(col, row);
                assert!(x > cassette_left_edge());
                assert!(y > cassette_bottom_edge());
            }
        }
        assert_eq!(positions.len(), 20);
        assert_eq!(POSITION_COUNT, 20);
    }

    #[test]
    fn randomized_assignment_has_capacity_and_unique_targets() {
        let unique: BTreeSet<usize> = RANDOMIZED_ASSIGNMENT.iter().copied().collect();
        assert_eq!(unique.len(), POSITION_COUNT);
        assert!(unique.iter().all(|position| *position < POSITION_COUNT));
        assert!(assignment_capacity() >= POSITION_COUNT);
        assert!(cartridge_capacity() >= POSITION_COUNT);
    }

    #[test]
    fn clean_and_rejected_chip_zones_are_segregated() {
        assert!(SEGREGATION_DIVIDER_W >= 18.0);
        assert!(SEGREGATION_AIR_GAP >= 28.0);
        assert!(clean_zone_right_edge() < cassette_left_edge());
        assert!(reject_left_edge() - clean_zone_right_edge() >= SEGREGATION_AIR_GAP);
        assert!(REJECT_CENTER_X > CASSETTE_CENTER_X);
    }

    #[test]
    fn robot_pick_clearances_cover_chip_and_staging_geometry() {
        assert!(ROBOT_PICK_CLEARANCE_Z > BASE_Z + REVC_TOTAL_HEIGHT + 45.0);
        assert!(ROBOT_GRIPPER_SIDE_CLEARANCE >= 12.0);
        assert!(CLEAN_PITCH_X - CLEAN_POCKET_X >= ROBOT_GRIPPER_SIDE_CLEARANCE);
        assert!(CLEAN_PITCH_Y - CLEAN_POCKET_Y >= ROBOT_GRIPPER_SIDE_CLEARANCE);
        assert!(ROBOT_CASSETTE_CLEARANCE_MARGIN >= 2.0 * ROBOT_GRIPPER_SIDE_CLEARANCE);
    }
}
