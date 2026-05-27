use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Bioreactor feed/harvest bag hotel for the closed cell-culture automation stack.
//
// Research assumptions used for the concept geometry:
// - Single-use bioprocess bag stations commonly combine rigid bag holders with
//   integrated load cells for gravimetric feed/harvest tracking.
// - Bag handling equipment often separates room-temperature media/feed holds
//   from chilled product/harvest holds, with protected hose compartments and
//   track-and-trace lands.
// - Commercial systems scale through modular racks, manifolds, tubing/connector
//   assemblies, and mobile frames rather than open manual pours.
//
// This is mechanical product-concept CAD only. It is not a biological protocol,
// sterile validation, GMP claim, or bag material compatibility certification.

const OUTPUTS: [&str; 11] = [
    "output/bioreactor_feed_harvest_bag_hotel_base_leak_tray.stl",
    "output/bioreactor_feed_harvest_bag_hotel_fresh_feed_bag_positions.stl",
    "output/bioreactor_feed_harvest_bag_hotel_harvest_collection_bag_positions.stl",
    "output/bioreactor_feed_harvest_bag_hotel_load_cell_pad_matrix.stl",
    "output/bioreactor_feed_harvest_bag_hotel_chilled_room_temp_bay_separation.stl",
    "output/bioreactor_feed_harvest_bag_hotel_sterile_connector_bulkhead.stl",
    "output/bioreactor_feed_harvest_bag_hotel_tubing_strain_relief_combs.stl",
    "output/bioreactor_feed_harvest_bag_hotel_barcode_lot_scan_lands.stl",
    "output/bioreactor_feed_harvest_bag_hotel_handoff_manifold.stl",
    "output/bioreactor_feed_harvest_bag_hotel_robot_service_keepouts.stl",
    "output/bioreactor_feed_harvest_bag_hotel_assembly.stl",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 34.0;
const LEAK_SUMP_X: f64 = 1160.0;
const LEAK_SUMP_Y: f64 = 600.0;
const LEAK_SUMP_DEPTH: f64 = 9.0;
const DRAIN_PORT_D: f64 = 18.0;

const FRESH_BAG_COUNT: usize = 6;
const FRESH_COLS: usize = 3;
const FRESH_BAG_LAND_X: f64 = 148.0;
const FRESH_BAG_LAND_Y: f64 = 210.0;
const FRESH_BAG_LAND_Z: f64 = 18.0;
const FRESH_BAG_PITCH_X: f64 = 176.0;
const FRESH_BAG_PITCH_Y: f64 = 250.0;
const FRESH_CENTER_X: f64 = -370.0;
const FRESH_CENTER_Y: f64 = -48.0;
const FRESH_BACKPLATE_X: f64 = 610.0;
const FRESH_BACKPLATE_Y: f64 = 36.0;
const FRESH_BACKPLATE_Z: f64 = 250.0;

const HARVEST_BAG_COUNT: usize = 4;
const HARVEST_COLS: usize = 2;
const HARVEST_BAG_LAND_X: f64 = 176.0;
const HARVEST_BAG_LAND_Y: f64 = 238.0;
const HARVEST_BAG_LAND_Z: f64 = 18.0;
const HARVEST_BAG_PITCH_X: f64 = 222.0;
const HARVEST_BAG_PITCH_Y: f64 = 280.0;
const HARVEST_CENTER_X: f64 = 330.0;
const HARVEST_CENTER_Y: f64 = -34.0;
const HARVEST_BACKPLATE_X: f64 = 520.0;
const HARVEST_BACKPLATE_Y: f64 = 40.0;
const HARVEST_BACKPLATE_Z: f64 = 270.0;

const TOTAL_BAG_POSITIONS: usize = FRESH_BAG_COUNT + HARVEST_BAG_COUNT;
const LOAD_CELL_PAD_X: f64 = 96.0;
const LOAD_CELL_PAD_Y: f64 = 74.0;
const LOAD_CELL_PAD_Z: f64 = 12.0;
const LOAD_CELL_BEAM_X: f64 = 124.0;
const LOAD_CELL_BEAM_Y: f64 = 18.0;
const LOAD_CELL_BEAM_Z: f64 = 16.0;

const DIVIDER_X: f64 = 34.0;
const DIVIDER_Y: f64 = 640.0;
const DIVIDER_Z: f64 = 188.0;
const DIVIDER_CENTER_X: f64 = 28.0;
const CHILLED_COLD_PLATE_X: f64 = 540.0;
const CHILLED_COLD_PLATE_Y: f64 = 620.0;
const CHILLED_COLD_PLATE_Z: f64 = 18.0;
const CHILLED_CENTER_X: f64 = 350.0;
const ROOM_TEMP_MARKER_X: f64 = 600.0;
const ROOM_TEMP_MARKER_Y: f64 = 20.0;
const ROOM_TEMP_MARKER_Z: f64 = 8.0;

const BULKHEAD_X: f64 = 1180.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 210.0;
const BULKHEAD_CENTER_Y: f64 = DECK_Y / 2.0 - 56.0;
const CONNECTOR_PORTS: usize = 20;
const CONNECTOR_PITCH_X: f64 = 56.0;
const CONNECTOR_PORT_D: f64 = 20.0;
const CONNECTOR_COLLAR_D: f64 = 34.0;
const VENT_FILTER_COUNT: usize = 4;
const SANITARY_CAP_PARKS: usize = 10;

const TUBING_COMB_COUNT: usize = 4;
const TUBING_SLOTS_PER_COMB: usize = 8;
const TUBING_SLOT_PITCH: f64 = 23.0;
const TUBING_SLOT_D: f64 = 7.2;
const TUBING_COMB_X: f64 = 240.0;
const TUBING_COMB_Y: f64 = 34.0;
const TUBING_COMB_Z: f64 = 34.0;

const BARCODE_LANDS: usize = TOTAL_BAG_POSITIONS + 4;
const LABEL_LAND_X: f64 = 72.0;
const LABEL_LAND_Y: f64 = 30.0;
const LABEL_LAND_Z: f64 = 4.0;
const LOT_CARD_X: f64 = 150.0;
const LOT_CARD_Y: f64 = 44.0;

const HANDOFF_DOCKS: usize = 4;
const HANDOFF_DOCK_X: f64 = 188.0;
const HANDOFF_DOCK_Y: f64 = 82.0;
const HANDOFF_DOCK_Z: f64 = 36.0;
const HANDOFF_CENTER_Y: f64 = -DECK_Y / 2.0 + 84.0;
const HANDOFF_PITCH_X: f64 = 250.0;
const HANDOFF_CONNECTORS_PER_DOCK: usize = 3;

const FRONT_ROBOT_APPROACH: f64 = 460.0;
const REAR_BULKHEAD_SERVICE: f64 = 280.0;
const SIDE_BAG_CHANGE_CLEARANCE: f64 = 260.0;
const TOP_BAG_HANG_CLEARANCE: f64 = 360.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let fresh = fresh_feed_bag_positions();
    export(OUTPUTS[1], &fresh);

    let harvest = harvest_collection_bag_positions();
    export(OUTPUTS[2], &harvest);

    let load_cells = load_cell_pad_matrix();
    export(OUTPUTS[3], &load_cells);

    let separation = chilled_room_temp_bay_separation();
    export(OUTPUTS[4], &separation);

    let bulkhead = sterile_connector_bulkhead();
    export(OUTPUTS[5], &bulkhead);

    let combs = tubing_strain_relief_combs();
    export(OUTPUTS[6], &combs);

    let labels = barcode_lot_scan_lands();
    export(OUTPUTS[7], &labels);

    let handoff = handoff_manifold();
    export(OUTPUTS[8], &handoff);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + fresh
        + harvest
        + load_cells
        + separation
        + bulkhead
        + combs
        + labels
        + handoff
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Bioreactor feed/harvest bag hotel:");
    println!(
        "  Deck and containment:       {DECK_X:.0}mm x {DECK_Y:.0}mm deck with recessed leak sump and {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Feed positions:             {FRESH_BAG_COUNT} room-temperature fresh/feed bag lands on {FRESH_BAG_PITCH_X:.0}mm x {FRESH_BAG_PITCH_Y:.0}mm pitch"
    );
    println!(
        "  Harvest positions:          {HARVEST_BAG_COUNT} chilled harvest/collection bag lands with separated cold-plate bay"
    );
    println!(
        "  Gravimetric placeholders:   {TOTAL_BAG_POSITIONS} load-cell pads and beams, one per bag position"
    );
    println!(
        "  Closed routing:             {CONNECTOR_PORTS} bulkhead connector ports, {VENT_FILTER_COUNT} filter placeholders, {} tubing strain-relief slots",
        TUBING_COMB_COUNT * TUBING_SLOTS_PER_COMB
    );
    println!(
        "  Traceability/handoff:       {BARCODE_LANDS} barcode lands, {HANDOFF_DOCKS} module handoff docks, {} handoff connector lands",
        HANDOFF_DOCKS * HANDOFF_CONNECTORS_PER_DOCK
    );
    println!(
        "  Service envelopes:          {FRONT_ROBOT_APPROACH:.0}mm front robot approach, {REAR_BULKHEAD_SERVICE:.0}mm rear service, {SIDE_BAG_CHANGE_CLEARANCE:.0}mm side bag-change, {TOP_BAG_HANG_CLEARANCE:.0}mm top clearance"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(TOTAL_BAG_POSITIONS, 10);
    assert!(fresh_right_edge() < DIVIDER_CENTER_X - DIVIDER_X / 2.0 - 20.0);
    assert!(harvest_left_edge() > DIVIDER_CENTER_X + DIVIDER_X / 2.0 + 18.0);
    assert!(connector_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
    assert!(handoff_span_x() + HANDOFF_DOCK_X < DECK_X - 160.0);
    assert!(
        FRESH_CENTER_Y - FRESH_BAG_PITCH_Y / 2.0 - FRESH_BAG_LAND_Y / 2.0 > -DECK_Y / 2.0 + 70.0
    );
    assert!(
        HARVEST_CENTER_Y + HARVEST_BAG_PITCH_Y / 2.0 + HARVEST_BAG_LAND_Y / 2.0
            < DECK_Y / 2.0 - 112.0
    );
    assert!(TUBING_COMB_COUNT * TUBING_SLOTS_PER_COMB >= CONNECTOR_PORTS);
    assert!(BARCODE_LANDS >= TOTAL_BAG_POSITIONS);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("bag_hotel_base_deck", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let sump = centered_cube(
        "bag_hotel_recessed_leak_sump_cut",
        LEAK_SUMP_X,
        LEAK_SUMP_Y,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -22.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder("bag_hotel_leak_drain_cut", DRAIN_PORT_D / 2.0, 72.0, 36)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 10.0, DECK_Z - 5.0);

    deck - sump - drain + leak_tray_rims() + zone_datum_pins()
}

fn leak_tray_rims() -> Part {
    let front = centered_cube("bag_hotel_front_leak_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("bag_hotel_rear_leak_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "bag_hotel_left_leak_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "bag_hotel_right_leak_rim",
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn zone_datum_pins() -> Part {
    let mut pins = Part::empty("bag_hotel_zone_datum_pins");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 70.0, -DECK_Y / 2.0 + 72.0),
        (DECK_X / 2.0 - 70.0, -DECK_Y / 2.0 + 72.0),
        (-DECK_X / 2.0 + 70.0, DECK_Y / 2.0 - 72.0),
        (DECK_X / 2.0 - 70.0, DECK_Y / 2.0 - 72.0),
    ]
    .iter()
    .enumerate()
    {
        pins =
            pins + centered_cylinder(format!("bag_hotel_zone_datum_pin_{i}"), 5.0, 14.0, 24)
                .translate(*x, *y, DECK_Z + 7.0);
    }
    pins
}

fn fresh_feed_bag_positions() -> Part {
    let backplate = centered_cube(
        "bag_hotel_fresh_feed_rear_hang_backplate",
        FRESH_BACKPLATE_X,
        FRESH_BACKPLATE_Y,
        FRESH_BACKPLATE_Z,
    )
    .translate(
        FRESH_CENTER_X,
        DECK_Y / 2.0 - 104.0,
        DECK_Z + FRESH_BACKPLATE_Z / 2.0,
    );

    let mut bay = backplate;
    for index in 0..FRESH_BAG_COUNT {
        let (x, y) = fresh_position_xy(index);
        bay = bay
            + bag_land(
                format!("bag_hotel_fresh_feed_bag_land_{index}"),
                FRESH_BAG_LAND_X,
                FRESH_BAG_LAND_Y,
                FRESH_BAG_LAND_Z,
            )
            .translate(x, y, DECK_Z + FRESH_BAG_LAND_Z / 2.0)
            + hang_hook_pair(
                format!("bag_hotel_fresh_feed_hang_hooks_{index}"),
                FRESH_BAG_LAND_X,
            )
            .translate(x, y + FRESH_BAG_LAND_Y / 2.0 + 20.0, DECK_Z + 154.0)
            + bag_guard_rails(
                format!("bag_hotel_fresh_feed_guard_rails_{index}"),
                FRESH_BAG_LAND_X,
                FRESH_BAG_LAND_Y,
            )
            .translate(x, y, DECK_Z + 34.0);
    }
    bay + fresh_zone_label_bar()
}

fn harvest_collection_bag_positions() -> Part {
    let backplate = centered_cube(
        "bag_hotel_harvest_rear_chilled_hang_backplate",
        HARVEST_BACKPLATE_X,
        HARVEST_BACKPLATE_Y,
        HARVEST_BACKPLATE_Z,
    )
    .translate(
        HARVEST_CENTER_X,
        DECK_Y / 2.0 - 104.0,
        DECK_Z + HARVEST_BACKPLATE_Z / 2.0,
    );

    let mut bay = backplate;
    for index in 0..HARVEST_BAG_COUNT {
        let (x, y) = harvest_position_xy(index);
        bay = bay
            + bag_land(
                format!("bag_hotel_harvest_collection_bag_land_{index}"),
                HARVEST_BAG_LAND_X,
                HARVEST_BAG_LAND_Y,
                HARVEST_BAG_LAND_Z,
            )
            .translate(x, y, DECK_Z + HARVEST_BAG_LAND_Z / 2.0)
            + hang_hook_pair(
                format!("bag_hotel_harvest_hang_hooks_{index}"),
                HARVEST_BAG_LAND_X,
            )
            .translate(x, y + HARVEST_BAG_LAND_Y / 2.0 + 22.0, DECK_Z + 168.0)
            + bag_guard_rails(
                format!("bag_hotel_harvest_guard_rails_{index}"),
                HARVEST_BAG_LAND_X,
                HARVEST_BAG_LAND_Y,
            )
            .translate(x, y, DECK_Z + 38.0);
    }
    bay + harvest_zone_label_bar()
}

fn bag_land(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let pad = centered_cube(format!("{name}_support_pad"), x, y, z);
    let soft_pocket = centered_cube(
        format!("{name}_bag_body_recess"),
        x - 22.0,
        y - 28.0,
        z + 2.0,
    )
    .translate(0.0, 0.0, 4.0);
    let lower_tube_exit =
        centered_cube(format!("{name}_lower_tube_exit_slot"), 42.0, 34.0, z + 6.0).translate(
            0.0,
            -(y / 2.0 - 12.0),
            3.0,
        );
    pad - soft_pocket - lower_tube_exit
}

fn hang_hook_pair(name: impl Into<String>, span_x: f64) -> Part {
    let name = name.into();
    let left = centered_cylinder(format!("{name}_left_hook_post"), 6.0, 28.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-span_x / 2.0 + 36.0, 0.0, 0.0);
    let right = centered_cylinder(format!("{name}_right_hook_post"), 6.0, 28.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(span_x / 2.0 - 36.0, 0.0, 0.0);
    let bridge = centered_cube(
        format!("{name}_hook_keeper_bridge"),
        span_x - 52.0,
        10.0,
        14.0,
    );
    left + right + bridge
}

fn bag_guard_rails(name: impl Into<String>, x: f64, y: f64) -> Part {
    let name = name.into();
    let left = centered_cube(format!("{name}_left_guard"), 10.0, y, 26.0).translate(
        -(x / 2.0 + 10.0),
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_guard"), 10.0, y, 26.0).translate(
        x / 2.0 + 10.0,
        0.0,
        0.0,
    );
    let bottom = centered_cube(format!("{name}_bottom_guard"), x + 30.0, 10.0, 22.0).translate(
        0.0,
        -(y / 2.0 + 6.0),
        0.0,
    );
    left + right + bottom
}

fn load_cell_pad_matrix() -> Part {
    let mut pads = Part::empty("bag_hotel_load_cell_pad_matrix");
    for index in 0..FRESH_BAG_COUNT {
        let (x, y) = fresh_position_xy(index);
        pads = pads
            + load_cell_pad(format!("bag_hotel_fresh_load_cell_{index}")).translate(
                x,
                y,
                DECK_Z + 7.0,
            );
    }
    for index in 0..HARVEST_BAG_COUNT {
        let (x, y) = harvest_position_xy(index);
        pads = pads
            + load_cell_pad(format!("bag_hotel_harvest_load_cell_{index}")).translate(
                x,
                y,
                DECK_Z + 7.0,
            );
    }
    pads
}

fn load_cell_pad(name: impl Into<String>) -> Part {
    let name = name.into();
    let pad = centered_cube(
        format!("{name}_pad"),
        LOAD_CELL_PAD_X,
        LOAD_CELL_PAD_Y,
        LOAD_CELL_PAD_Z,
    );
    let beam = centered_cube(
        format!("{name}_load_cell_beam_placeholder"),
        LOAD_CELL_BEAM_X,
        LOAD_CELL_BEAM_Y,
        LOAD_CELL_BEAM_Z,
    )
    .translate(0.0, 0.0, LOAD_CELL_PAD_Z / 2.0 + LOAD_CELL_BEAM_Z / 2.0);
    let cable_gland = centered_cylinder(
        format!("{name}_cable_gland_cut"),
        4.0,
        LOAD_CELL_PAD_X + 14.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOAD_CELL_PAD_Y / 2.0 - 12.0, LOAD_CELL_PAD_Z + 5.0);
    pad + beam - cable_gland
}

fn chilled_room_temp_bay_separation() -> Part {
    let divider = centered_cube(
        "bag_hotel_chilled_room_temp_vertical_divider",
        DIVIDER_X,
        DIVIDER_Y,
        DIVIDER_Z,
    )
    .translate(DIVIDER_CENTER_X, -12.0, DECK_Z + DIVIDER_Z / 2.0);
    let thermal_break = centered_cube(
        "bag_hotel_divider_thermal_break_channel",
        DIVIDER_X + 4.0,
        DIVIDER_Y - 80.0,
        24.0,
    )
    .translate(DIVIDER_CENTER_X, -20.0, DECK_Z + 80.0);
    let chilled_plate = centered_cube(
        "bag_hotel_harvest_chilled_cold_plate_placeholder",
        CHILLED_COLD_PLATE_X,
        CHILLED_COLD_PLATE_Y,
        CHILLED_COLD_PLATE_Z,
    )
    .translate(CHILLED_CENTER_X, -22.0, DECK_Z + CHILLED_COLD_PLATE_Z / 2.0);
    let chilled_drain_moat = centered_cube(
        "bag_hotel_chilled_bay_condensate_moat_cut",
        CHILLED_COLD_PLATE_X - 70.0,
        CHILLED_COLD_PLATE_Y - 72.0,
        CHILLED_COLD_PLATE_Z + 2.0,
    )
    .translate(
        CHILLED_CENTER_X,
        -22.0,
        DECK_Z + CHILLED_COLD_PLATE_Z / 2.0 + 2.0,
    );
    let room_marker = centered_cube(
        "bag_hotel_room_temp_feed_bay_marker_rail",
        ROOM_TEMP_MARKER_X,
        ROOM_TEMP_MARKER_Y,
        ROOM_TEMP_MARKER_Z,
    )
    .translate(FRESH_CENTER_X, -DECK_Y / 2.0 + 124.0, DECK_Z + 8.0);
    divider - thermal_break
        + (chilled_plate - chilled_drain_moat)
        + room_marker
        + chilled_cover_frame()
}

fn chilled_cover_frame() -> Part {
    let rear = centered_cube(
        "bag_hotel_chilled_bay_rear_condensation_lip",
        CHILLED_COLD_PLATE_X,
        14.0,
        34.0,
    )
    .translate(CHILLED_CENTER_X, DECK_Y / 2.0 - 154.0, DECK_Z + 36.0);
    let front = centered_cube(
        "bag_hotel_chilled_bay_front_condensation_lip",
        CHILLED_COLD_PLATE_X,
        14.0,
        34.0,
    )
    .translate(CHILLED_CENTER_X, -DECK_Y / 2.0 + 128.0, DECK_Z + 36.0);
    let side = centered_cube(
        "bag_hotel_chilled_bay_outer_condensation_lip",
        14.0,
        CHILLED_COLD_PLATE_Y,
        34.0,
    )
    .translate(DECK_X / 2.0 - 98.0, -22.0, DECK_Z + 36.0);
    rear + front + side
}

fn sterile_connector_bulkhead() -> Part {
    let panel = centered_cube(
        "bag_hotel_sterile_connector_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, DECK_Z + BULKHEAD_Z / 2.0);
    let mut cuts = Part::empty("bag_hotel_bulkhead_port_cuts");
    let mut collars = Part::empty("bag_hotel_bulkhead_port_collars");
    for i in 0..CONNECTOR_PORTS {
        let x = connector_x(i);
        let z = DECK_Z + 62.0 + ((i % 2) as f64) * 76.0;
        cuts = cuts
            + centered_cylinder(
                format!("bag_hotel_bulkhead_connector_port_cut_{i}"),
                CONNECTOR_PORT_D / 2.0,
                BULKHEAD_Y + 8.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y, z);
        collars = collars
            + centered_cylinder(
                format!("bag_hotel_bulkhead_connector_collar_{i}"),
                CONNECTOR_COLLAR_D / 2.0,
                8.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0, z);
    }
    (panel - cuts) + collars + filter_placeholders() + sanitary_cap_parks()
}

fn filter_placeholders() -> Part {
    let mut filters = Part::empty("bag_hotel_bulkhead_vent_filter_placeholders");
    for i in 0..VENT_FILTER_COUNT {
        let x = -BULKHEAD_X / 2.0 + 140.0 + i as f64 * 96.0;
        filters = filters
            + centered_cylinder(
                format!("bag_hotel_vent_filter_placeholder_{i}"),
                18.0,
                48.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 18.0,
                DECK_Z + 174.0,
            );
    }
    filters
}

fn sanitary_cap_parks() -> Part {
    let mut parks = Part::empty("bag_hotel_sanitary_cap_parks");
    for i in 0..SANITARY_CAP_PARKS {
        let x = -BULKHEAD_X / 2.0 + 120.0 + i as f64 * 104.0;
        parks = parks
            + centered_cube(format!("bag_hotel_cap_park_block_{i}"), 42.0, 20.0, 24.0).translate(
                x,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 18.0,
                DECK_Z + 22.0,
            );
    }
    parks
}

fn tubing_strain_relief_combs() -> Part {
    let mut combs = Part::empty("bag_hotel_tubing_strain_relief_combs");
    for i in 0..TUBING_COMB_COUNT {
        let x = -420.0 + i as f64 * 280.0;
        let y = if i % 2 == 0 {
            -DECK_Y / 2.0 + 174.0
        } else {
            DECK_Y / 2.0 - 184.0
        };
        combs = combs
            + tubing_comb(format!("bag_hotel_tubing_comb_{i}")).translate(x, y, DECK_Z + 22.0);
    }
    combs
}

fn tubing_comb(name: impl Into<String>) -> Part {
    let name = name.into();
    let block = centered_cube(
        format!("{name}_block"),
        TUBING_COMB_X,
        TUBING_COMB_Y,
        TUBING_COMB_Z,
    );
    let mut cuts = Part::empty(format!("{name}_slot_cuts"));
    for slot in 0..TUBING_SLOTS_PER_COMB {
        let x = comb_slot_x(slot);
        cuts = cuts
            + centered_cylinder(
                format!("{name}_slot_cut_{slot}"),
                TUBING_SLOT_D / 2.0,
                TUBING_COMB_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 3.0);
    }
    block - cuts
}

fn barcode_lot_scan_lands() -> Part {
    let mut lands = Part::empty("bag_hotel_barcode_lot_scan_lands");
    for i in 0..FRESH_BAG_COUNT {
        let (x, y) = fresh_position_xy(i);
        lands = lands
            + label_land(format!("bag_hotel_fresh_barcode_land_{i}")).translate(
                x,
                y - FRESH_BAG_LAND_Y / 2.0 - 24.0,
                DECK_Z + LABEL_LAND_Z / 2.0 + 1.0,
            );
    }
    for i in 0..HARVEST_BAG_COUNT {
        let (x, y) = harvest_position_xy(i);
        lands = lands
            + label_land(format!("bag_hotel_harvest_barcode_land_{i}")).translate(
                x,
                y - HARVEST_BAG_LAND_Y / 2.0 - 24.0,
                DECK_Z + LABEL_LAND_Z / 2.0 + 1.0,
            );
    }
    for i in 0..4 {
        lands = lands
            + centered_cube(
                format!("bag_hotel_module_lot_card_land_{i}"),
                LOT_CARD_X,
                LOT_CARD_Y,
                LABEL_LAND_Z,
            )
            .translate(
                -375.0 + i as f64 * 250.0,
                -DECK_Y / 2.0 + 34.0,
                DECK_Z + 4.0,
            );
    }
    lands
}

fn label_land(name: impl Into<String>) -> Part {
    centered_cube(name, LABEL_LAND_X, LABEL_LAND_Y, LABEL_LAND_Z)
}

fn handoff_manifold() -> Part {
    let base = centered_cube(
        "bag_hotel_handoff_common_drip_lip",
        handoff_span_x() + HANDOFF_DOCK_X + 60.0,
        HANDOFF_DOCK_Y + 40.0,
        18.0,
    )
    .translate(0.0, HANDOFF_CENTER_Y, DECK_Z + 10.0);
    let mut docks = Part::empty("bag_hotel_handoff_docks");
    for i in 0..HANDOFF_DOCKS {
        let x = handoff_x(i);
        docks = docks
            + handoff_dock(format!("bag_hotel_handoff_dock_{i}")).translate(
                x,
                HANDOFF_CENTER_Y,
                DECK_Z + HANDOFF_DOCK_Z / 2.0 + 18.0,
            );
    }
    base + docks
}

fn handoff_dock(name: impl Into<String>) -> Part {
    let name = name.into();
    let dock = centered_cube(
        format!("{name}_body"),
        HANDOFF_DOCK_X,
        HANDOFF_DOCK_Y,
        HANDOFF_DOCK_Z,
    );
    let mut ports = Part::empty(format!("{name}_port_cuts"));
    let mut collars = Part::empty(format!("{name}_connector_lands"));
    for i in 0..HANDOFF_CONNECTORS_PER_DOCK {
        let x = -44.0 + i as f64 * 44.0;
        ports = ports
            + centered_cylinder(
                format!("{name}_port_cut_{i}"),
                8.0,
                HANDOFF_DOCK_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 4.0);
        collars = collars
            + centered_cylinder(format!("{name}_connector_land_{i}"), 15.0, 7.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(x, -HANDOFF_DOCK_Y / 2.0 - 3.5, 4.0);
    }
    dock - ports + collars
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "bag_hotel_front_robot_approach_keepout",
        DECK_X - 110.0,
        FRONT_ROBOT_APPROACH,
        120.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + 60.0,
    );
    let rear = centered_cube(
        "bag_hotel_rear_bulkhead_service_keepout",
        BULKHEAD_X,
        REAR_BULKHEAD_SERVICE,
        BULKHEAD_Z + 90.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_BULKHEAD_SERVICE / 2.0,
        DECK_Z + (BULKHEAD_Z + 90.0) / 2.0,
    );
    let left = centered_cube(
        "bag_hotel_left_bag_change_service_keepout",
        SIDE_BAG_CHANGE_CLEARANCE,
        DECK_Y - 120.0,
        TOP_BAG_HANG_CLEARANCE,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_BAG_CHANGE_CLEARANCE / 2.0,
        -10.0,
        DECK_Z + TOP_BAG_HANG_CLEARANCE / 2.0,
    );
    let right = centered_cube(
        "bag_hotel_right_bag_change_service_keepout",
        SIDE_BAG_CHANGE_CLEARANCE,
        DECK_Y - 120.0,
        TOP_BAG_HANG_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 + SIDE_BAG_CHANGE_CLEARANCE / 2.0,
        -10.0,
        DECK_Z + TOP_BAG_HANG_CLEARANCE / 2.0,
    );
    let top = centered_cube("bag_hotel_top_bag_hang_lift_keepout", DECK_X, DECK_Y, 80.0).translate(
        0.0,
        0.0,
        DECK_Z + TOP_BAG_HANG_CLEARANCE + 40.0,
    );
    front + rear + left + right + top
}

fn fresh_zone_label_bar() -> Part {
    centered_cube(
        "bag_hotel_fresh_feed_room_temperature_zone_label_bar",
        540.0,
        12.0,
        28.0,
    )
    .translate(FRESH_CENTER_X, DECK_Y / 2.0 - 136.0, DECK_Z + 20.0)
}

fn harvest_zone_label_bar() -> Part {
    centered_cube(
        "bag_hotel_harvest_collection_chilled_zone_label_bar",
        460.0,
        12.0,
        28.0,
    )
    .translate(HARVEST_CENTER_X, DECK_Y / 2.0 - 136.0, DECK_Z + 20.0)
}

fn fresh_position_xy(index: usize) -> (f64, f64) {
    let col = index % FRESH_COLS;
    let row = index / FRESH_COLS;
    let x = FRESH_CENTER_X + (col as f64 - 1.0) * FRESH_BAG_PITCH_X;
    let y = FRESH_CENTER_Y + (row as f64 - 0.5) * FRESH_BAG_PITCH_Y;
    (x, y)
}

fn harvest_position_xy(index: usize) -> (f64, f64) {
    let col = index % HARVEST_COLS;
    let row = index / HARVEST_COLS;
    let x = HARVEST_CENTER_X + (col as f64 - 0.5) * HARVEST_BAG_PITCH_X;
    let y = HARVEST_CENTER_Y + (row as f64 - 0.5) * HARVEST_BAG_PITCH_Y;
    (x, y)
}

fn connector_x(index: usize) -> f64 {
    (index as f64 - (CONNECTOR_PORTS as f64 - 1.0) / 2.0) * CONNECTOR_PITCH_X
}

fn connector_span_x() -> f64 {
    (CONNECTOR_PORTS as f64 - 1.0) * CONNECTOR_PITCH_X
}

fn comb_slot_x(index: usize) -> f64 {
    (index as f64 - (TUBING_SLOTS_PER_COMB as f64 - 1.0) / 2.0) * TUBING_SLOT_PITCH
}

fn handoff_x(index: usize) -> f64 {
    (index as f64 - (HANDOFF_DOCKS as f64 - 1.0) / 2.0) * HANDOFF_PITCH_X
}

fn handoff_span_x() -> f64 {
    (HANDOFF_DOCKS as f64 - 1.0) * HANDOFF_PITCH_X
}

#[cfg(test)]
fn fresh_left_edge() -> f64 {
    FRESH_CENTER_X - FRESH_BAG_PITCH_X - FRESH_BAG_LAND_X / 2.0
}

fn fresh_right_edge() -> f64 {
    FRESH_CENTER_X + FRESH_BAG_PITCH_X + FRESH_BAG_LAND_X / 2.0
}

fn harvest_left_edge() -> f64 {
    HARVEST_CENTER_X - HARVEST_BAG_PITCH_X / 2.0 - HARVEST_BAG_LAND_X / 2.0
}

#[cfg(test)]
fn harvest_right_edge() -> f64 {
    HARVEST_CENTER_X + HARVEST_BAG_PITCH_X / 2.0 + HARVEST_BAG_LAND_X / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: HashSet<_> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/bioreactor_feed_harvest_bag_hotel_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feed_and_harvest_bays_are_segregated() {
        assert_eq!(FRESH_BAG_COUNT, 6);
        assert_eq!(HARVEST_BAG_COUNT, 4);
        assert!(fresh_left_edge() > -DECK_X / 2.0 + RIM_W + 10.0);
        assert!(fresh_right_edge() < DIVIDER_CENTER_X - DIVIDER_X / 2.0);
        assert!(harvest_left_edge() > DIVIDER_CENTER_X + DIVIDER_X / 2.0);
        assert!(harvest_right_edge() < DECK_X / 2.0 - RIM_W - 10.0);
        assert!(CHILLED_COLD_PLATE_X > HARVEST_BAG_PITCH_X + HARVEST_BAG_LAND_X);
    }

    #[test]
    fn gravimetric_and_traceability_counts_cover_bag_positions() {
        assert_eq!(TOTAL_BAG_POSITIONS, FRESH_BAG_COUNT + HARVEST_BAG_COUNT);
        assert_eq!(TOTAL_BAG_POSITIONS, 10);
        assert!(LOAD_CELL_PAD_X < FRESH_BAG_LAND_X);
        assert!(LOAD_CELL_PAD_Y < FRESH_BAG_LAND_Y);
        assert!(BARCODE_LANDS >= TOTAL_BAG_POSITIONS + HANDOFF_DOCKS);
    }

    #[test]
    fn closed_connector_capacity_covers_feed_harvest_and_handoffs() {
        assert!(CONNECTOR_PORTS >= TOTAL_BAG_POSITIONS * 2);
        assert_eq!(HANDOFF_DOCKS, 4);
        assert_eq!(HANDOFF_DOCKS * HANDOFF_CONNECTORS_PER_DOCK, 12);
        assert!(connector_span_x() + CONNECTOR_COLLAR_D < BULKHEAD_X);
        assert!(TUBING_COMB_COUNT * TUBING_SLOTS_PER_COMB >= CONNECTOR_PORTS);
    }

    #[test]
    fn handoff_and_service_envelopes_clear_the_deck() {
        assert!(handoff_span_x() + HANDOFF_DOCK_X < DECK_X - 120.0);
        assert!(HANDOFF_CENTER_Y - HANDOFF_DOCK_Y / 2.0 > -DECK_Y / 2.0 + 20.0);
        assert!(FRONT_ROBOT_APPROACH >= 420.0);
        assert!(REAR_BULKHEAD_SERVICE >= 240.0);
        assert!(SIDE_BAG_CHANGE_CLEARANCE >= 220.0);
        assert!(TOP_BAG_HANG_CLEARANCE >= 320.0);
    }
}
