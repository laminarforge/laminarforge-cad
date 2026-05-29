use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cryostorage inventory retrieval and transfer staging fixture.
//
// Design intent:
// - Stage cryovials and cryobags from LN2 or qualified vapor/shipping storage into a
//   closed cell-bank recovery workflow.
// - Preserve identity, temperature evidence, released/hold/reject segregation,
//   condensate control, tote docking, custody tokens, and robot/service clearances.
// - Keep every CAD feature as a mechanical planning envelope. This is not a
//   validated cryogenic handling, sterility, thawing, or chain-of-custody claim.

const OUTPUTS: &[&str] = &[
    "output/closed_cryostorage_inventory_transfer_fixture_frost_condensate_leak_tray.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_cryovial_rack_nest.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_cryobag_cassette_pocket.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_cold_vapor_shipping_envelope.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_inventory_scan_lands.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_release_hold_reject_lanes.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_cold_pack_placeholder_zone.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_ppe_tool_staging_pockets.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_transfer_tote_dock.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_custody_token_logger_slots.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_robot_service_keepouts.stl",
    "output/closed_cryostorage_inventory_transfer_fixture_assembly.stl",
];

const DECK_X: f64 = 1560.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 24.0;
const TRAY_RIM_H: f64 = 40.0;
const SUMP_X: f64 = 1380.0;
const SUMP_Y: f64 = 768.0;
const SUMP_DEPTH: f64 = 10.0;
const DRAIN_PORT_D: f64 = 18.0;
const FROST_GUTTER_COUNT: usize = 4;

const VIAL_NEST_X: f64 = 432.0;
const VIAL_NEST_Y: f64 = 306.0;
const VIAL_NEST_Z: f64 = 46.0;
const VIAL_CENTER_X: f64 = -456.0;
const VIAL_CENTER_Y: f64 = 170.0;
const VIAL_RACK_ROWS: usize = 8;
const VIAL_RACK_COLS: usize = 12;
const VIAL_POSITIONS: usize = VIAL_RACK_ROWS * VIAL_RACK_COLS;
const VIAL_PITCH: f64 = 25.0;
const VIAL_BORE_D: f64 = 15.4;
const VIAL_RACK_RECESS_DEPTH: f64 = 12.0;
const RACK_KEY_SLOT_X: f64 = 18.0;
const RACK_KEY_SLOT_Y: f64 = 74.0;

const CRYOBAG_POCKET_X: f64 = 386.0;
const CRYOBAG_POCKET_Y: f64 = 272.0;
const CRYOBAG_POCKET_Z: f64 = 58.0;
const CRYOBAG_CENTER_X: f64 = 15.0;
const CRYOBAG_CENTER_Y: f64 = 166.0;
const CRYOBAG_SLOTS: usize = 3;
const CRYOBAG_SLOT_X: f64 = 106.0;
const CRYOBAG_SLOT_Y: f64 = 214.0;
const CRYOBAG_SLOT_DEPTH: f64 = 20.0;

const VAPOR_ENVELOPE_X: f64 = 382.0;
const VAPOR_ENVELOPE_Y: f64 = 344.0;
const VAPOR_ENVELOPE_Z: f64 = 178.0;
const VAPOR_CENTER_X: f64 = 460.0;
const VAPOR_CENTER_Y: f64 = 142.0;
const SHIPPING_CASSETTE_X: f64 = 302.0;
const SHIPPING_CASSETTE_Y: f64 = 256.0;
const SHIPPING_CASSETTE_Z: f64 = 118.0;
const VAPOR_VENT_SLOTS: usize = 8;

const RELEASE_LANE_COUNT: usize = 3;
const LANE_X: f64 = 322.0;
const LANE_Y: f64 = 86.0;
const LANE_Z: f64 = 30.0;
const LANE_CENTER_X: f64 = -476.0;
const LANE_START_Y: f64 = -270.0;
const LANE_PITCH_Y: f64 = 112.0;
const LANE_STOP_BOSS_COUNT: usize = 2;

const COLD_PACK_ZONE_X: f64 = 316.0;
const COLD_PACK_ZONE_Y: f64 = 214.0;
const COLD_PACK_ZONE_Z: f64 = 42.0;
const COLD_PACK_CENTER_X: f64 = -52.0;
const COLD_PACK_CENTER_Y: f64 = -284.0;
const COLD_PACK_POCKET_COUNT: usize = 4;

const TOOL_STAGE_X: f64 = 286.0;
const TOOL_STAGE_Y: f64 = 240.0;
const TOOL_STAGE_Z: f64 = 36.0;
const TOOL_STAGE_CENTER_X: f64 = 582.0;
const TOOL_STAGE_CENTER_Y: f64 = -170.0;
const PPE_POCKETS: usize = 5;
const TOOL_POCKETS: usize = 6;

const TOTE_DOCK_X: f64 = 496.0;
const TOTE_DOCK_Y: f64 = 112.0;
const TOTE_DOCK_Z: f64 = 54.0;
const TOTE_DOCK_CENTER_X: f64 = 462.0;
const TOTE_DOCK_CENTER_Y: f64 = -360.0;
const TOTE_GASKET_X: f64 = 438.0;
const TOTE_GASKET_Y: f64 = 68.0;
const TOTE_GASKET_Z: f64 = 8.0;
const TOTE_DATUM_PIN_D: f64 = 12.0;
const TOTE_LATCH_COUNT: usize = 4;

const SCAN_LANDS: usize = 18;
const SCAN_LAND_X: f64 = 92.0;
const SCAN_LAND_Y: f64 = 34.0;
const SCAN_LAND_Z: f64 = 4.0;
const RFID_PAD_D: f64 = 22.0;

const CUSTODY_SLOT_COUNT: usize = 24;
const CUSTODY_SLOT_X: f64 = 32.0;
const CUSTODY_SLOT_Y: f64 = 16.0;
const CUSTODY_SLOT_Z: f64 = 12.0;
const LOGGER_POCKETS: usize = 4;
const LOGGER_POCKET_X: f64 = 94.0;
const LOGGER_POCKET_Y: f64 = 42.0;
const LOGGER_POCKET_Z: f64 = 20.0;

const FRONT_ROBOT_APPROACH: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_INVENTORY_CLEARANCE: f64 = 220.0;
const RIGHT_TRANSFER_CLEARANCE: f64 = 300.0;
const TOP_CRYO_TRANSFER_CLEARANCE: f64 = 380.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = frost_condensate_leak_tray();
    export(OUTPUTS[0], &tray);

    let vial_nest = cryovial_rack_nest();
    export(OUTPUTS[1], &vial_nest);

    let cryobag = cryobag_cassette_pocket();
    export(OUTPUTS[2], &cryobag);

    let vapor = cold_vapor_shipping_envelope();
    export(OUTPUTS[3], &vapor);

    let scans = inventory_scan_lands();
    export(OUTPUTS[4], &scans);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[5], &lanes);

    let cold_pack = cold_pack_placeholder_zone();
    export(OUTPUTS[6], &cold_pack);

    let tools = ppe_tool_staging_pockets();
    export(OUTPUTS[7], &tools);

    let tote = transfer_tote_dock();
    export(OUTPUTS[8], &tote);

    let custody = custody_token_logger_slots();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = tray
        + vial_nest
        + cryobag
        + vapor
        + scans
        + lanes
        + cold_pack
        + tools
        + tote
        + custody
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cryostorage inventory transfer fixture:");
    println!(
        "  Deck/leak control:         {DECK_X:.0}mm W x {DECK_Y:.0}mm D x {DECK_Z:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm frost/condensate sump, {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Inventory capacity:        {VIAL_POSITIONS} cryovial positions on one 12x8 rack nest, {CRYOBAG_SLOTS} cryobag cassette slots, vapor/shipping cassette envelope {SHIPPING_CASSETTE_X:.0}mm x {SHIPPING_CASSETTE_Y:.0}mm x {SHIPPING_CASSETTE_Z:.0}mm"
    );
    println!(
        "  Identity/custody:          {SCAN_LANDS} barcode/RFID scan lands, {CUSTODY_SLOT_COUNT} custody token slots, {LOGGER_POCKETS} temperature logger pockets"
    );
    println!(
        "  Segregation and cold mass: released/hold/reject lanes plus {COLD_PACK_POCKET_COUNT} dry-ice/cold-pack placeholder pockets"
    );
    println!(
        "  Handling/service:          transfer tote dock {TOTE_DOCK_X:.0}mm x {TOTE_DOCK_Y:.0}mm, {PPE_POCKETS} PPE pockets, {TOOL_POCKETS} tool pockets"
    );
    println!(
        "  Keepouts:                  front robot {FRONT_ROBOT_APPROACH:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, left inventory {LEFT_INVENTORY_CLEARANCE:.0}mm, right transfer {RIGHT_TRANSFER_CLEARANCE:.0}mm, top {TOP_CRYO_TRANSFER_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(VIAL_POSITIONS, 96);
    assert_eq!(RELEASE_LANE_COUNT, 3);
    assert_eq!(FROST_GUTTER_COUNT, 4);
    assert!(vial_right_edge() < cryobag_left_edge() - 34.0);
    assert!(cryobag_right_edge() < vapor_left_edge() - 34.0);
    assert!(release_lane_x_max() < cold_pack_left_edge() - 34.0);
    assert!(cold_pack_right_edge() < tool_stage_left_edge() - 28.0);
    assert!(TOTE_DOCK_CENTER_Y - TOTE_DOCK_Y / 2.0 > -DECK_Y / 2.0 + TRAY_RIM_W + 12.0);
    assert!(TOTE_DOCK_CENTER_X + TOTE_DOCK_X / 2.0 < DECK_X / 2.0 - TRAY_RIM_W - 12.0);
    assert!(scan_land_count() == SCAN_LANDS);
    assert!(custody_slot_count() == CUSTODY_SLOT_COUNT);
    assert!(logger_pocket_count() == LOGGER_POCKETS);
    assert!(vapor_vent_count() == VAPOR_VENT_SLOTS);
    assert!(cold_pack_pocket_count() == COLD_PACK_POCKET_COUNT);
}

fn frost_condensate_leak_tray() -> Part {
    let deck = centered_cube("cryostorage_transfer_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let sump = centered_cube(
        "cryostorage_transfer_recessed_frost_condensate_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, 10.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        "cryostorage_transfer_condensate_drain_cut",
        DRAIN_PORT_D / 2.0,
        TRAY_RIM_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 16.0, DECK_Z - 6.0);

    deck - sump - drain + leak_tray_rim() + frost_gutters() + datum_bosses()
}

fn leak_tray_rim() -> Part {
    let front = centered_cube(
        "cryostorage_transfer_front_condensate_rim",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_H,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_H / 2.0,
    );
    let rear = centered_cube(
        "cryostorage_transfer_rear_condensate_rim",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_H,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_H / 2.0,
    );
    let left = centered_cube(
        "cryostorage_transfer_left_condensate_rim",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_H,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_H / 2.0,
    );
    let right = centered_cube(
        "cryostorage_transfer_right_condensate_rim",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_H,
    )
    .translate(
        DECK_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_H / 2.0,
    );

    front + rear + left + right
}

fn frost_gutters() -> Part {
    let mut gutters = Part::empty("cryostorage_transfer_frost_flow_gutters");
    let x_span = DECK_X - 180.0;
    let ys = [-330.0, -104.0, 106.0, 322.0];
    for (i, y) in ys.iter().enumerate() {
        let gutter = centered_cube(
            format!("cryostorage_transfer_frost_gutter_{i}"),
            x_span,
            12.0,
            7.0,
        )
        .translate(0.0, *y, DECK_Z + 3.5);
        gutters = gutters + gutter;
    }
    gutters
}

fn datum_bosses() -> Part {
    let mut bosses = Part::empty("cryostorage_transfer_fixture_datum_bosses");
    let positions = [
        (-690.0, 390.0),
        (690.0, 390.0),
        (-690.0, -390.0),
        (690.0, -390.0),
        (-80.0, 392.0),
        (80.0, -392.0),
    ];
    for (i, (x, y)) in positions.iter().enumerate() {
        let boss = centered_cylinder(
            format!("cryostorage_transfer_datum_boss_{i}"),
            14.0,
            10.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        let pin_cut = centered_cylinder(
            format!("cryostorage_transfer_datum_pin_cut_{i}"),
            4.0,
            12.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 5.5);
        bosses = bosses + (boss - pin_cut);
    }
    bosses
}

fn cryovial_rack_nest() -> Part {
    let base = centered_cube(
        "cryostorage_transfer_cryovial_rack_nest_body",
        VIAL_NEST_X,
        VIAL_NEST_Y,
        VIAL_NEST_Z,
    )
    .translate(VIAL_CENTER_X, VIAL_CENTER_Y, DECK_Z + VIAL_NEST_Z / 2.0);

    let rack_recess = centered_cube(
        "cryostorage_transfer_sbs_cryovial_rack_recess",
        VIAL_NEST_X - 56.0,
        VIAL_NEST_Y - 54.0,
        VIAL_RACK_RECESS_DEPTH + 0.4,
    )
    .translate(
        VIAL_CENTER_X,
        VIAL_CENTER_Y,
        DECK_Z + VIAL_NEST_Z - VIAL_RACK_RECESS_DEPTH / 2.0 + 0.2,
    );

    let key_slot = centered_cube(
        "cryostorage_transfer_cryovial_rack_a1_key_slot",
        RACK_KEY_SLOT_X,
        RACK_KEY_SLOT_Y,
        VIAL_RACK_RECESS_DEPTH + 2.0,
    )
    .translate(
        VIAL_CENTER_X - VIAL_NEST_X / 2.0 + 42.0,
        VIAL_CENTER_Y + VIAL_NEST_Y / 2.0 - 72.0,
        DECK_Z + VIAL_NEST_Z - VIAL_RACK_RECESS_DEPTH / 2.0,
    );

    let mut wells = Part::empty("cryostorage_transfer_cryovial_well_cuts");
    for row in 0..VIAL_RACK_ROWS {
        for col in 0..VIAL_RACK_COLS {
            let x = VIAL_CENTER_X + (col as f64 - (VIAL_RACK_COLS as f64 - 1.0) / 2.0) * VIAL_PITCH;
            let y = VIAL_CENTER_Y + ((VIAL_RACK_ROWS as f64 - 1.0) / 2.0 - row as f64) * VIAL_PITCH;
            let well = centered_cylinder(
                format!("cryostorage_transfer_vial_well_r{row}_c{col}"),
                VIAL_BORE_D / 2.0,
                28.0,
                28,
            )
            .translate(x, y, DECK_Z + VIAL_NEST_Z - 14.0);
            wells = wells + well;
        }
    }

    base - rack_recess - key_slot - wells + rack_clamp_lips()
}

fn rack_clamp_lips() -> Part {
    let front = centered_cube(
        "cryostorage_transfer_vial_rack_front_clamp_lip",
        VIAL_NEST_X - 36.0,
        12.0,
        18.0,
    )
    .translate(
        VIAL_CENTER_X,
        VIAL_CENTER_Y - VIAL_NEST_Y / 2.0 + 18.0,
        DECK_Z + VIAL_NEST_Z + 9.0,
    );
    let rear = centered_cube(
        "cryostorage_transfer_vial_rack_rear_clamp_lip",
        VIAL_NEST_X - 36.0,
        12.0,
        18.0,
    )
    .translate(
        VIAL_CENTER_X,
        VIAL_CENTER_Y + VIAL_NEST_Y / 2.0 - 18.0,
        DECK_Z + VIAL_NEST_Z + 9.0,
    );
    let left = centered_cube(
        "cryostorage_transfer_vial_rack_left_clamp_lip",
        12.0,
        VIAL_NEST_Y - 36.0,
        18.0,
    )
    .translate(
        VIAL_CENTER_X - VIAL_NEST_X / 2.0 + 18.0,
        VIAL_CENTER_Y,
        DECK_Z + VIAL_NEST_Z + 9.0,
    );
    let right = centered_cube(
        "cryostorage_transfer_vial_rack_right_clamp_lip",
        12.0,
        VIAL_NEST_Y - 36.0,
        18.0,
    )
    .translate(
        VIAL_CENTER_X + VIAL_NEST_X / 2.0 - 18.0,
        VIAL_CENTER_Y,
        DECK_Z + VIAL_NEST_Z + 9.0,
    );
    front + rear + left + right
}

fn cryobag_cassette_pocket() -> Part {
    let pocket = centered_cube(
        "cryostorage_transfer_cryobag_cassette_pocket_body",
        CRYOBAG_POCKET_X,
        CRYOBAG_POCKET_Y,
        CRYOBAG_POCKET_Z,
    )
    .translate(
        CRYOBAG_CENTER_X,
        CRYOBAG_CENTER_Y,
        DECK_Z + CRYOBAG_POCKET_Z / 2.0,
    );

    let mut cuts = Part::empty("cryostorage_transfer_cryobag_slot_cuts");
    for i in 0..CRYOBAG_SLOTS {
        let x = CRYOBAG_CENTER_X + (i as f64 - 1.0) * (CRYOBAG_SLOT_X + 14.0);
        let slot = centered_cube(
            format!("cryostorage_transfer_cryobag_slot_cut_{i}"),
            CRYOBAG_SLOT_X,
            CRYOBAG_SLOT_Y,
            CRYOBAG_SLOT_DEPTH + 0.4,
        )
        .translate(
            x,
            CRYOBAG_CENTER_Y,
            DECK_Z + CRYOBAG_POCKET_Z - CRYOBAG_SLOT_DEPTH / 2.0 + 0.2,
        );
        let tube_exit = centered_cube(
            format!("cryostorage_transfer_cryobag_tube_exit_cut_{i}"),
            42.0,
            18.0,
            18.0,
        )
        .translate(
            x,
            CRYOBAG_CENTER_Y - CRYOBAG_POCKET_Y / 2.0 + 9.0,
            DECK_Z + CRYOBAG_POCKET_Z - 9.0,
        );
        cuts = cuts + slot + tube_exit;
    }

    pocket - cuts + cryobag_retainers() + cryobag_handle_lands()
}

fn cryobag_retainers() -> Part {
    let mut retainers = Part::empty("cryostorage_transfer_cryobag_cassette_retainers");
    for i in 0..CRYOBAG_SLOTS {
        let x = CRYOBAG_CENTER_X + (i as f64 - 1.0) * (CRYOBAG_SLOT_X + 14.0);
        let front = centered_cube(
            format!("cryostorage_transfer_cryobag_front_retainer_{i}"),
            CRYOBAG_SLOT_X + 18.0,
            10.0,
            24.0,
        )
        .translate(
            x,
            CRYOBAG_CENTER_Y - CRYOBAG_POCKET_Y / 2.0 + 26.0,
            DECK_Z + CRYOBAG_POCKET_Z + 12.0,
        );
        let rear = centered_cube(
            format!("cryostorage_transfer_cryobag_rear_retainer_{i}"),
            CRYOBAG_SLOT_X + 18.0,
            10.0,
            24.0,
        )
        .translate(
            x,
            CRYOBAG_CENTER_Y + CRYOBAG_POCKET_Y / 2.0 - 26.0,
            DECK_Z + CRYOBAG_POCKET_Z + 12.0,
        );
        retainers = retainers + front + rear;
    }
    retainers
}

fn cryobag_handle_lands() -> Part {
    let mut lands = Part::empty("cryostorage_transfer_cryobag_handle_lands");
    for i in 0..CRYOBAG_SLOTS {
        let x = CRYOBAG_CENTER_X + (i as f64 - 1.0) * (CRYOBAG_SLOT_X + 14.0);
        let land = centered_cube(
            format!("cryostorage_transfer_cryobag_handle_land_{i}"),
            66.0,
            20.0,
            5.0,
        )
        .translate(
            x,
            CRYOBAG_CENTER_Y + CRYOBAG_POCKET_Y / 2.0 - 48.0,
            DECK_Z + CRYOBAG_POCKET_Z + 2.5,
        );
        lands = lands + land;
    }
    lands
}

fn cold_vapor_shipping_envelope() -> Part {
    let back_wall = centered_cube(
        "cryostorage_transfer_cold_vapor_envelope_back_wall",
        VAPOR_ENVELOPE_X,
        16.0,
        VAPOR_ENVELOPE_Z,
    )
    .translate(
        VAPOR_CENTER_X,
        VAPOR_CENTER_Y + VAPOR_ENVELOPE_Y / 2.0 - 8.0,
        DECK_Z + VAPOR_ENVELOPE_Z / 2.0,
    );
    let left_wall = centered_cube(
        "cryostorage_transfer_cold_vapor_envelope_left_wall",
        16.0,
        VAPOR_ENVELOPE_Y,
        VAPOR_ENVELOPE_Z,
    )
    .translate(
        VAPOR_CENTER_X - VAPOR_ENVELOPE_X / 2.0 + 8.0,
        VAPOR_CENTER_Y,
        DECK_Z + VAPOR_ENVELOPE_Z / 2.0,
    );
    let right_wall = centered_cube(
        "cryostorage_transfer_cold_vapor_envelope_right_wall",
        16.0,
        VAPOR_ENVELOPE_Y,
        VAPOR_ENVELOPE_Z,
    )
    .translate(
        VAPOR_CENTER_X + VAPOR_ENVELOPE_X / 2.0 - 8.0,
        VAPOR_CENTER_Y,
        DECK_Z + VAPOR_ENVELOPE_Z / 2.0,
    );
    let floor = centered_cube(
        "cryostorage_transfer_cold_vapor_shipping_floor",
        VAPOR_ENVELOPE_X,
        VAPOR_ENVELOPE_Y,
        18.0,
    )
    .translate(VAPOR_CENTER_X, VAPOR_CENTER_Y, DECK_Z + 9.0);

    let cassette_shadow = centered_cube(
        "cryostorage_transfer_shipping_cassette_shadow",
        SHIPPING_CASSETTE_X,
        SHIPPING_CASSETTE_Y,
        SHIPPING_CASSETTE_Z,
    )
    .translate(
        VAPOR_CENTER_X,
        VAPOR_CENTER_Y - 10.0,
        DECK_Z + 18.0 + SHIPPING_CASSETTE_Z / 2.0,
    );

    let envelope = back_wall + left_wall + right_wall + floor + cassette_shadow;
    envelope + vapor_vent_lands() + shipping_cassette_datums()
}

fn vapor_vent_lands() -> Part {
    let mut vents = Part::empty("cryostorage_transfer_cold_vapor_vent_lands");
    for i in 0..VAPOR_VENT_SLOTS {
        let y = VAPOR_CENTER_Y - VAPOR_ENVELOPE_Y / 2.0 + 48.0 + i as f64 * 34.0;
        let left = centered_cube(
            format!("cryostorage_transfer_left_vapor_vent_land_{i}"),
            5.0,
            22.0,
            42.0,
        )
        .translate(
            VAPOR_CENTER_X - VAPOR_ENVELOPE_X / 2.0 + 19.0,
            y,
            DECK_Z + 90.0,
        );
        let right = centered_cube(
            format!("cryostorage_transfer_right_vapor_vent_land_{i}"),
            5.0,
            22.0,
            42.0,
        )
        .translate(
            VAPOR_CENTER_X + VAPOR_ENVELOPE_X / 2.0 - 19.0,
            y,
            DECK_Z + 90.0,
        );
        vents = vents + left + right;
    }
    vents
}

fn shipping_cassette_datums() -> Part {
    let mut datums = Part::empty("cryostorage_transfer_shipping_cassette_datums");
    let positions = [
        (
            -SHIPPING_CASSETTE_X / 2.0 + 42.0,
            -SHIPPING_CASSETTE_Y / 2.0 + 42.0,
        ),
        (
            SHIPPING_CASSETTE_X / 2.0 - 42.0,
            -SHIPPING_CASSETTE_Y / 2.0 + 42.0,
        ),
        (
            -SHIPPING_CASSETTE_X / 2.0 + 42.0,
            SHIPPING_CASSETTE_Y / 2.0 - 42.0,
        ),
        (
            SHIPPING_CASSETTE_X / 2.0 - 42.0,
            SHIPPING_CASSETTE_Y / 2.0 - 42.0,
        ),
    ];
    for (i, (dx, dy)) in positions.iter().enumerate() {
        let pin = centered_cylinder(
            format!("cryostorage_transfer_shipping_cassette_datum_pin_{i}"),
            6.0,
            22.0,
            28,
        )
        .translate(
            VAPOR_CENTER_X + dx,
            VAPOR_CENTER_Y - 10.0 + dy,
            DECK_Z + 29.0,
        );
        datums = datums + pin;
    }
    datums
}

fn inventory_scan_lands() -> Part {
    let mut lands = Part::empty("cryostorage_transfer_inventory_scan_lands");
    for i in 0..SCAN_LANDS {
        let (x, y) = scan_land_position(i);
        let label = centered_cube(
            format!("cryostorage_transfer_barcode_scan_land_{i}"),
            SCAN_LAND_X,
            SCAN_LAND_Y,
            SCAN_LAND_Z,
        )
        .translate(x, y, DECK_Z + SCAN_LAND_Z / 2.0 + 1.0);
        let rfid = centered_cylinder(
            format!("cryostorage_transfer_rfid_button_land_{i}"),
            RFID_PAD_D / 2.0,
            3.0,
            28,
        )
        .translate(x + SCAN_LAND_X / 2.0 - 18.0, y, DECK_Z + SCAN_LAND_Z + 4.0);
        lands = lands + label + rfid;
    }
    lands
}

fn scan_land_position(i: usize) -> (f64, f64) {
    match i {
        0..=5 => (-620.0 + i as f64 * 76.0, 378.0),
        6..=11 => (-154.0 + (i - 6) as f64 * 88.0, 378.0),
        _ => (404.0 + (i - 12) as f64 * 74.0, 378.0),
    }
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("cryostorage_transfer_release_hold_reject_lanes");
    for i in 0..RELEASE_LANE_COUNT {
        let y = LANE_START_Y + i as f64 * LANE_PITCH_Y;
        let lane = centered_cube(
            format!("cryostorage_transfer_release_hold_reject_lane_{i}"),
            LANE_X,
            LANE_Y,
            LANE_Z,
        )
        .translate(LANE_CENTER_X, y, DECK_Z + LANE_Z / 2.0);
        let lane_recess = centered_cube(
            format!("cryostorage_transfer_release_hold_reject_recess_{i}"),
            LANE_X - 38.0,
            LANE_Y - 30.0,
            12.0,
        )
        .translate(LANE_CENTER_X, y, DECK_Z + LANE_Z - 6.0);
        lanes = lanes + (lane - lane_recess) + lane_stop_bosses(i, y);
    }
    lanes
}

fn lane_stop_bosses(i: usize, y: f64) -> Part {
    let mut bosses = Part::empty(format!("cryostorage_transfer_lane_{i}_stop_bosses"));
    for side in 0..LANE_STOP_BOSS_COUNT {
        let x = LANE_CENTER_X
            + if side == 0 {
                -LANE_X / 2.0 + 30.0
            } else {
                LANE_X / 2.0 - 30.0
            };
        let boss = centered_cylinder(
            format!("cryostorage_transfer_lane_{i}_stop_boss_{side}"),
            11.0,
            14.0,
            24,
        )
        .translate(x, y + LANE_Y / 2.0 - 18.0, DECK_Z + LANE_Z + 7.0);
        bosses = bosses + boss;
    }
    bosses
}

fn cold_pack_placeholder_zone() -> Part {
    let zone = centered_cube(
        "cryostorage_transfer_dry_ice_cold_pack_zone",
        COLD_PACK_ZONE_X,
        COLD_PACK_ZONE_Y,
        COLD_PACK_ZONE_Z,
    )
    .translate(
        COLD_PACK_CENTER_X,
        COLD_PACK_CENTER_Y,
        DECK_Z + COLD_PACK_ZONE_Z / 2.0,
    );

    let mut pockets = Part::empty("cryostorage_transfer_cold_pack_pocket_cuts");
    for i in 0..COLD_PACK_POCKET_COUNT {
        let col = i % 2;
        let row = i / 2;
        let x = COLD_PACK_CENTER_X + (col as f64 - 0.5) * 142.0;
        let y = COLD_PACK_CENTER_Y + (row as f64 - 0.5) * 82.0;
        let pocket = centered_cube(
            format!("cryostorage_transfer_cold_pack_pocket_cut_{i}"),
            116.0,
            62.0,
            18.0,
        )
        .translate(x, y, DECK_Z + COLD_PACK_ZONE_Z - 9.0);
        pockets = pockets + pocket;
    }

    zone - pockets + cold_pack_retainer_lips()
}

fn cold_pack_retainer_lips() -> Part {
    let front = centered_cube(
        "cryostorage_transfer_cold_pack_front_retainer_lip",
        COLD_PACK_ZONE_X - 30.0,
        10.0,
        20.0,
    )
    .translate(
        COLD_PACK_CENTER_X,
        COLD_PACK_CENTER_Y - COLD_PACK_ZONE_Y / 2.0 + 16.0,
        DECK_Z + COLD_PACK_ZONE_Z + 10.0,
    );
    let rear = centered_cube(
        "cryostorage_transfer_cold_pack_rear_retainer_lip",
        COLD_PACK_ZONE_X - 30.0,
        10.0,
        20.0,
    )
    .translate(
        COLD_PACK_CENTER_X,
        COLD_PACK_CENTER_Y + COLD_PACK_ZONE_Y / 2.0 - 16.0,
        DECK_Z + COLD_PACK_ZONE_Z + 10.0,
    );
    front + rear
}

fn ppe_tool_staging_pockets() -> Part {
    let stage = centered_cube(
        "cryostorage_transfer_ppe_tool_staging_body",
        TOOL_STAGE_X,
        TOOL_STAGE_Y,
        TOOL_STAGE_Z,
    )
    .translate(
        TOOL_STAGE_CENTER_X,
        TOOL_STAGE_CENTER_Y,
        DECK_Z + TOOL_STAGE_Z / 2.0,
    );

    let mut cuts = Part::empty("cryostorage_transfer_ppe_tool_pocket_cuts");
    for i in 0..PPE_POCKETS {
        let x = TOOL_STAGE_CENTER_X - 104.0 + i as f64 * 52.0;
        let pocket = centered_cube(
            format!("cryostorage_transfer_ppe_pocket_cut_{i}"),
            38.0,
            58.0,
            18.0,
        )
        .translate(x, TOOL_STAGE_CENTER_Y + 64.0, DECK_Z + TOOL_STAGE_Z - 9.0);
        cuts = cuts + pocket;
    }
    for i in 0..TOOL_POCKETS {
        let x = TOOL_STAGE_CENTER_X - 118.0 + i as f64 * 47.2;
        let pocket = centered_cylinder(
            format!("cryostorage_transfer_tool_pocket_cut_{i}"),
            11.0,
            18.0,
            26,
        )
        .translate(x, TOOL_STAGE_CENTER_Y - 54.0, DECK_Z + TOOL_STAGE_Z - 9.0);
        cuts = cuts + pocket;
    }

    stage - cuts + tool_shadow_lands()
}

fn tool_shadow_lands() -> Part {
    let mut shadows = Part::empty("cryostorage_transfer_tool_shadow_lands");
    for i in 0..TOOL_POCKETS {
        let x = TOOL_STAGE_CENTER_X - 118.0 + i as f64 * 47.2;
        let land = centered_cube(
            format!("cryostorage_transfer_tool_shadow_land_{i}"),
            32.0,
            7.0,
            4.0,
        )
        .translate(x, TOOL_STAGE_CENTER_Y - 18.0, DECK_Z + TOOL_STAGE_Z + 2.0);
        shadows = shadows + land;
    }
    shadows
}

fn transfer_tote_dock() -> Part {
    let dock = centered_cube(
        "cryostorage_transfer_closed_transfer_tote_dock",
        TOTE_DOCK_X,
        TOTE_DOCK_Y,
        TOTE_DOCK_Z,
    )
    .translate(
        TOTE_DOCK_CENTER_X,
        TOTE_DOCK_CENTER_Y,
        DECK_Z + TOTE_DOCK_Z / 2.0,
    );

    let gasket_recess = centered_cube(
        "cryostorage_transfer_tote_gasket_recess",
        TOTE_GASKET_X,
        TOTE_GASKET_Y,
        TOTE_GASKET_Z + 0.4,
    )
    .translate(
        TOTE_DOCK_CENTER_X,
        TOTE_DOCK_CENTER_Y,
        DECK_Z + TOTE_DOCK_Z - TOTE_GASKET_Z / 2.0 + 0.2,
    );

    dock - gasket_recess + tote_datum_pins() + tote_latch_sensor_lands()
}

fn tote_datum_pins() -> Part {
    let mut pins = Part::empty("cryostorage_transfer_tote_datum_pins");
    let positions = [
        (-TOTE_GASKET_X / 2.0 + 42.0, -TOTE_GASKET_Y / 2.0 + 22.0),
        (TOTE_GASKET_X / 2.0 - 42.0, -TOTE_GASKET_Y / 2.0 + 22.0),
        (-TOTE_GASKET_X / 2.0 + 42.0, TOTE_GASKET_Y / 2.0 - 22.0),
        (TOTE_GASKET_X / 2.0 - 42.0, TOTE_GASKET_Y / 2.0 - 22.0),
    ];
    for (i, (dx, dy)) in positions.iter().enumerate() {
        let pin = centered_cylinder(
            format!("cryostorage_transfer_tote_datum_pin_{i}"),
            TOTE_DATUM_PIN_D / 2.0,
            22.0,
            28,
        )
        .translate(
            TOTE_DOCK_CENTER_X + dx,
            TOTE_DOCK_CENTER_Y + dy,
            DECK_Z + TOTE_DOCK_Z + 11.0,
        );
        pins = pins + pin;
    }
    pins
}

fn tote_latch_sensor_lands() -> Part {
    let mut lands = Part::empty("cryostorage_transfer_tote_latch_sensor_lands");
    for i in 0..TOTE_LATCH_COUNT {
        let x = TOTE_DOCK_CENTER_X - 180.0 + i as f64 * 120.0;
        let land = centered_cube(
            format!("cryostorage_transfer_tote_latch_sensor_land_{i}"),
            46.0,
            16.0,
            5.0,
        )
        .translate(
            x,
            TOTE_DOCK_CENTER_Y + TOTE_DOCK_Y / 2.0 - 18.0,
            DECK_Z + TOTE_DOCK_Z + 2.5,
        );
        lands = lands + land;
    }
    lands
}

fn custody_token_logger_slots() -> Part {
    let custody_body = centered_cube(
        "cryostorage_transfer_chain_of_custody_token_block",
        318.0,
        116.0,
        34.0,
    )
    .translate(-36.0, -112.0, DECK_Z + 17.0);

    let mut slots = Part::empty("cryostorage_transfer_chain_of_custody_slot_cuts");
    for i in 0..CUSTODY_SLOT_COUNT {
        let col = i % 8;
        let row = i / 8;
        let x = -36.0 + (col as f64 - 3.5) * 36.0;
        let y = -112.0 + (row as f64 - 1.0) * 28.0;
        let slot = centered_cube(
            format!("cryostorage_transfer_custody_token_slot_cut_{i}"),
            CUSTODY_SLOT_X,
            CUSTODY_SLOT_Y,
            CUSTODY_SLOT_Z,
        )
        .translate(x, y, DECK_Z + 34.0 - CUSTODY_SLOT_Z / 2.0);
        slots = slots + slot;
    }

    let logger_bank = logger_pocket_bank();
    custody_body - slots + logger_bank
}

fn logger_pocket_bank() -> Part {
    let mut bank = Part::empty("cryostorage_transfer_temperature_logger_pockets");
    for i in 0..LOGGER_POCKETS {
        let x = 286.0 + (i as f64 - 1.5) * 104.0;
        let body = centered_cube(
            format!("cryostorage_transfer_temperature_logger_pocket_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_Z,
        )
        .translate(x, -112.0, DECK_Z + LOGGER_POCKET_Z / 2.0);
        let recess = centered_cube(
            format!("cryostorage_transfer_temperature_logger_recess_{i}"),
            LOGGER_POCKET_X - 22.0,
            LOGGER_POCKET_Y - 14.0,
            8.0,
        )
        .translate(x, -112.0, DECK_Z + LOGGER_POCKET_Z - 4.0);
        bank = bank + (body - recess);
    }
    bank
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "cryostorage_transfer_front_robot_approach_keepout",
        DECK_X - 168.0,
        FRONT_ROBOT_APPROACH,
        4.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + 2.0,
    );
    let rear_service = centered_cube(
        "cryostorage_transfer_rear_service_keepout",
        DECK_X - 220.0,
        REAR_SERVICE_CLEARANCE,
        4.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        DECK_Z + 2.0,
    );
    let left_inventory = centered_cube(
        "cryostorage_transfer_left_inventory_cart_keepout",
        LEFT_INVENTORY_CLEARANCE,
        DECK_Y - 190.0,
        4.0,
    )
    .translate(
        -DECK_X / 2.0 - LEFT_INVENTORY_CLEARANCE / 2.0,
        42.0,
        DECK_Z + 2.0,
    );
    let right_transfer = centered_cube(
        "cryostorage_transfer_right_tote_transfer_keepout",
        RIGHT_TRANSFER_CLEARANCE,
        DECK_Y - 240.0,
        4.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_TRANSFER_CLEARANCE / 2.0,
        -26.0,
        DECK_Z + 2.0,
    );
    let top_clearance = centered_cube(
        "cryostorage_transfer_top_cryo_transfer_clearance",
        SHIPPING_CASSETTE_X + 96.0,
        SHIPPING_CASSETTE_Y + 96.0,
        4.0,
    )
    .translate(
        VAPOR_CENTER_X,
        VAPOR_CENTER_Y,
        DECK_Z + VAPOR_ENVELOPE_Z + TOP_CRYO_TRANSFER_CLEARANCE,
    );

    front_robot + rear_service + left_inventory + right_transfer + top_clearance
}

fn vial_right_edge() -> f64 {
    VIAL_CENTER_X + VIAL_NEST_X / 2.0
}

fn cryobag_left_edge() -> f64 {
    CRYOBAG_CENTER_X - CRYOBAG_POCKET_X / 2.0
}

fn cryobag_right_edge() -> f64 {
    CRYOBAG_CENTER_X + CRYOBAG_POCKET_X / 2.0
}

fn vapor_left_edge() -> f64 {
    VAPOR_CENTER_X - VAPOR_ENVELOPE_X / 2.0
}

fn release_lane_x_max() -> f64 {
    LANE_CENTER_X + LANE_X / 2.0
}

fn cold_pack_left_edge() -> f64 {
    COLD_PACK_CENTER_X - COLD_PACK_ZONE_X / 2.0
}

fn cold_pack_right_edge() -> f64 {
    COLD_PACK_CENTER_X + COLD_PACK_ZONE_X / 2.0
}

fn tool_stage_left_edge() -> f64 {
    TOOL_STAGE_CENTER_X - TOOL_STAGE_X / 2.0
}

fn scan_land_count() -> usize {
    SCAN_LANDS
}

fn custody_slot_count() -> usize {
    CUSTODY_SLOT_COUNT
}

fn logger_pocket_count() -> usize {
    LOGGER_POCKETS
}

fn vapor_vent_count() -> usize {
    VAPOR_VENT_SLOTS
}

fn cold_pack_pocket_count() -> usize {
    COLD_PACK_POCKET_COUNT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_reserves_required_inventory_capacity() {
        assert_eq!(VIAL_POSITIONS, 96);
        assert_eq!(CRYOBAG_SLOTS, 3);
        assert!(VIAL_PITCH >= VIAL_BORE_D + 8.0);
        assert!(CRYOBAG_SLOT_X * (CRYOBAG_SLOTS as f64) < CRYOBAG_POCKET_X);
    }

    #[test]
    fn cold_chain_and_custody_features_are_present() {
        assert_eq!(COLD_PACK_POCKET_COUNT, 4);
        assert_eq!(LOGGER_POCKETS, 4);
        assert_eq!(CUSTODY_SLOT_COUNT, 24);
        assert!(VAPOR_ENVELOPE_Z > SHIPPING_CASSETTE_Z + 40.0);
    }

    #[test]
    fn segregation_lanes_are_plausibly_spaced() {
        assert_eq!(RELEASE_LANE_COUNT, 3);
        assert!(LANE_PITCH_Y > LANE_Y + 20.0);
        assert!(LANE_START_Y + (RELEASE_LANE_COUNT - 1) as f64 * LANE_PITCH_Y < 0.0);
    }

    #[test]
    fn tote_dock_and_service_keepouts_fit_deck() {
        assert!(TOTE_DOCK_X < DECK_X * 0.38);
        assert!(TOTE_DOCK_Y < DECK_Y * 0.16);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(RIGHT_TRANSFER_CLEARANCE >= 300.0);
    }

    #[test]
    fn full_layout_assertions_pass() {
        assert_layout();
    }
}
