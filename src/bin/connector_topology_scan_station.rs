use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Connector/chip topology scan and verification station for electronic batch records.
//
// Intent:
// - Present a 20-chip cassette at hard datums while scanning chip, connector,
//   tubing, pump, valve, leak-test, and reject/mismatch identities.
// - Keep the barcode/RFID, camera/illumination bridge, leak-test placeholders,
//   quarantine pocket, and handoff/service keepouts in one repeatable station.
// - Model only mechanical datums, scan targets, and cable/tubing clearance
//   envelopes. The EBR software and purchased scanners/cameras are external.

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const CONNECTORS_PER_CHIP: usize = 4;
const CONNECTOR_ID_SLOTS: usize = CHIP_COUNT * CONNECTORS_PER_CHIP;

const GUTTER: f64 = 8.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const CHIP_ARRAY_X: f64 = CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * GUTTER;
const CHIP_ARRAY_Y: f64 = CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * GUTTER;

const CASSETTE_MARGIN_X: f64 = 44.0;
const CASSETTE_MARGIN_Y: f64 = 46.0;
const CASSETTE_X: f64 = CHIP_ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;
const CASSETTE_CENTER_X: f64 = -120.0;
const CASSETTE_CENTER_Y: f64 = 20.0;

const DECK_X: f64 = 1120.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;

const DATUM_RAIL_W: f64 = 16.0;
const DATUM_RAIL_Z: f64 = 24.0;
const CHIP_POCKET_CLEARANCE: f64 = 0.70;
const CHIP_POCKET_DEPTH: f64 = 5.0;

const SCAN_ZONE_X: f64 = 318.0;
const SCAN_ZONE_Y: f64 = 96.0;
const SCAN_ZONE_Z: f64 = 16.0;
const SCAN_ZONE_CENTER_X: f64 = -380.0;
const SCAN_ZONE_CENTER_Y: f64 = -338.0;

const COMB_X: f64 = 176.0;
const COMB_Y: f64 = 520.0;
const COMB_Z: f64 = 22.0;
const COMB_CENTER_X: f64 = CASSETTE_CENTER_X + CASSETTE_X / 2.0 + 94.0;
const COMB_CENTER_Y: f64 = CASSETTE_CENTER_Y;
const CONNECTOR_LANE_PITCH_X: f64 = 30.0;
const CONNECTOR_ROW_PITCH_Y: f64 = 23.0;
const CONNECTOR_SLOT_D: f64 = 5.8;

const CHANNEL_MAP_COLS: usize = 6;
const CHANNEL_MAP_ROWS: usize = 4;
const PUMP_VALVE_CHANNELS: usize = CHANNEL_MAP_COLS * CHANNEL_MAP_ROWS;
const CHANNEL_MAP_CAPACITY: usize = PUMP_VALVE_CHANNELS * CONNECTORS_PER_CHIP;
const MAP_PLATE_X: f64 = 292.0;
const MAP_PLATE_Y: f64 = 214.0;
const MAP_PLATE_Z: f64 = 12.0;
const MAP_CENTER_X: f64 = DECK_X / 2.0 - 150.0;
const MAP_CENTER_Y: f64 = DECK_Y / 2.0 - 150.0;

const LEAK_PORT_ROWS: usize = 2;
const LEAK_PORTS_PER_ROW: usize = CHIP_COUNT / LEAK_PORT_ROWS;
const LEAK_PORT_BAR_X: f64 = 560.0;
const LEAK_PORT_BAR_Y: f64 = 72.0;
const LEAK_PORT_BAR_Z: f64 = 26.0;
const LEAK_PORT_CENTER_X: f64 = CASSETTE_CENTER_X;
const LEAK_PORT_CENTER_Y: f64 = CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 - 54.0;
const LEAK_PORT_PITCH_X: f64 = 50.0;
const LEAK_PORT_PITCH_Y: f64 = 30.0;
const LEAK_PORT_D: f64 = 7.2;

const BRIDGE_SIDE_CLEARANCE: f64 = 44.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 58.0;
const BRIDGE_BEAM_Z: f64 = 22.0;
const SCAN_BRIDGE_CLEARANCE_Z: f64 = 86.0;
const BRIDGE_UNDERSIDE_Z: f64 = REVC_TOTAL_HEIGHT + SCAN_BRIDGE_CLEARANCE_Z;
const BRIDGE_POST_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const BRIDGE_SPAN_X: f64 = CASSETTE_X + BRIDGE_SIDE_CLEARANCE * 2.0;

const QUARANTINE_X: f64 = 190.0;
const QUARANTINE_Y: f64 = 156.0;
const QUARANTINE_Z: f64 = 42.0;
const QUARANTINE_CENTER_X: f64 = DECK_X / 2.0 - 122.0;
const QUARANTINE_CENTER_Y: f64 = -DECK_Y / 2.0 + 128.0;
const QUARANTINE_WALL: f64 = 8.0;
const SEGREGATION_GAP_MIN: f64 = 48.0;

const ROBOT_HANDOFF_CLEARANCE_X: f64 = 230.0;
const OPERATOR_HANDOFF_CLEARANCE_Y: f64 = 112.0;
const CABLE_KEEP_OUT_Y: f64 = 58.0;
const SERVICE_KEEP_OUT_X: f64 = 118.0;

fn main() {
    let deck = station_deck();
    deck.write_stl("output/connector_topology_scan_station_deck.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_deck.stl");

    let cassette = twenty_chip_cassette_datum();
    cassette
        .write_stl("output/connector_topology_scan_station_twenty_chip_cassette_datum.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_twenty_chip_cassette_datum.stl");

    let ebr_scan_zone = barcode_rfid_scan_zone();
    ebr_scan_zone
        .write_stl("output/connector_topology_scan_station_ebr_scan_zone.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_ebr_scan_zone.stl");

    let connector_comb = connector_id_scan_comb();
    connector_comb
        .write_stl("output/connector_topology_scan_station_connector_id_scan_comb.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_connector_id_scan_comb.stl");

    let channel_map = pump_valve_channel_map_plate();
    channel_map
        .write_stl("output/connector_topology_scan_station_pump_valve_channel_map_plate.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_pump_valve_channel_map_plate.stl");

    let leak_ports = leak_test_port_bar();
    leak_ports
        .write_stl("output/connector_topology_scan_station_leak_test_port_bar.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_leak_test_port_bar.stl");

    let scan_bridge = camera_illumination_scan_bridge();
    scan_bridge
        .write_stl("output/connector_topology_scan_station_camera_illumination_scan_bridge.stl")
        .unwrap();
    println!(
        "Exported: output/connector_topology_scan_station_camera_illumination_scan_bridge.stl"
    );

    let quarantine = rejected_mismatch_quarantine_pocket();
    quarantine
        .write_stl("output/connector_topology_scan_station_quarantine_pocket.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_quarantine_pocket.stl");

    let keepouts = handoff_and_service_keepout_gauges();
    keepouts
        .write_stl("output/connector_topology_scan_station_handoff_service_keepouts.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_handoff_service_keepouts.stl");

    let assembly =
        deck + cassette.translate(
            CASSETTE_CENTER_X,
            CASSETTE_CENTER_Y,
            DECK_Z / 2.0 + DATUM_RAIL_Z / 2.0,
        ) + ebr_scan_zone.translate(
            SCAN_ZONE_CENTER_X,
            SCAN_ZONE_CENTER_Y,
            DECK_Z / 2.0 + SCAN_ZONE_Z / 2.0,
        ) + connector_comb.translate(COMB_CENTER_X, COMB_CENTER_Y, DECK_Z / 2.0 + COMB_Z / 2.0)
            + channel_map.translate(MAP_CENTER_X, MAP_CENTER_Y, DECK_Z / 2.0 + MAP_PLATE_Z / 2.0)
            + leak_ports.translate(
                LEAK_PORT_CENTER_X,
                LEAK_PORT_CENTER_Y,
                DECK_Z / 2.0 + LEAK_PORT_BAR_Z / 2.0,
            )
            + scan_bridge.translate(CASSETTE_CENTER_X, CASSETTE_CENTER_Y, DECK_Z / 2.0)
            + quarantine.translate(
                QUARANTINE_CENTER_X,
                QUARANTINE_CENTER_Y,
                DECK_Z / 2.0 + QUARANTINE_Z / 2.0,
            )
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + 9.0);

    assembly
        .write_stl("output/connector_topology_scan_station_assembly.stl")
        .unwrap();
    println!("Exported: output/connector_topology_scan_station_assembly.stl");

    println!(
        "Connector topology scan station: {:.0}mm x {:.0}mm deck, {} Rev C chip positions, {} connector ID slots, {} connector map cells across {} pump/valve channels, {} leak-test placeholders, {:.0}mm scan-bridge clearance above chip top, and a segregated reject/mismatch pocket with >= {:.0}mm planned gap.",
        DECK_X,
        DECK_Y,
        CHIP_COUNT,
        CONNECTOR_ID_SLOTS,
        CHANNEL_MAP_CAPACITY,
        PUMP_VALVE_CHANNELS,
        CHIP_COUNT,
        bridge_underside_clearance(),
        SEGREGATION_GAP_MIN
    );
}

fn station_deck() -> Part {
    let deck = centered_cube("connector_topology_station_deck", DECK_X, DECK_Y, DECK_Z);

    let cassette_recess = centered_cube(
        "connector_topology_station_cassette_recess",
        CASSETTE_X + 28.0,
        CASSETTE_Y + 28.0,
        7.0,
    )
    .translate(CASSETTE_CENTER_X, CASSETTE_CENTER_Y, DECK_Z / 2.0 - 3.2);
    let scan_zone_recess = centered_cube(
        "connector_topology_station_barcode_rfid_recess",
        SCAN_ZONE_X + 18.0,
        SCAN_ZONE_Y + 18.0,
        5.0,
    )
    .translate(SCAN_ZONE_CENTER_X, SCAN_ZONE_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let comb_recess = centered_cube(
        "connector_topology_station_connector_comb_recess",
        COMB_X + 20.0,
        COMB_Y + 20.0,
        5.0,
    )
    .translate(COMB_CENTER_X, COMB_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let map_recess = centered_cube(
        "connector_topology_station_channel_map_recess",
        MAP_PLATE_X + 18.0,
        MAP_PLATE_Y + 18.0,
        4.0,
    )
    .translate(MAP_CENTER_X, MAP_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let quarantine_recess = centered_cube(
        "connector_topology_station_quarantine_recess",
        QUARANTINE_X + 24.0,
        QUARANTINE_Y + 24.0,
        6.0,
    )
    .translate(QUARANTINE_CENTER_X, QUARANTINE_CENTER_Y, DECK_Z / 2.0 - 2.5);

    let operator_handoff_clearance = centered_cube(
        "connector_topology_station_operator_handoff_clearance",
        CASSETTE_X + ROBOT_HANDOFF_CLEARANCE_X,
        OPERATOR_HANDOFF_CLEARANCE_Y,
        DECK_Z + 2.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        -DECK_Y / 2.0 + OPERATOR_HANDOFF_CLEARANCE_Y / 2.0 + 14.0,
        0.0,
    );
    let rear_cable_keepout = centered_cube(
        "connector_topology_station_rear_cable_keepout",
        DECK_X - 140.0,
        CABLE_KEEP_OUT_Y,
        DECK_Z + 2.0,
    )
    .translate(0.0, DECK_Y / 2.0 - CABLE_KEEP_OUT_Y / 2.0 - 10.0, 0.0);
    let right_service_keepout = centered_cube(
        "connector_topology_station_right_service_keepout",
        SERVICE_KEEP_OUT_X,
        CASSETTE_Y + 70.0,
        DECK_Z + 2.0,
    )
    .translate(DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 10.0, 18.0, 0.0);

    let mut mounts = Part::empty("connector_topology_station_mounts");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        mounts = mounts
            + centered_cylinder(
                format!("connector_topology_station_m5_mount_{i}"),
                5.3 / 2.0,
                DECK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    deck - cassette_recess
        - scan_zone_recess
        - comb_recess
        - map_recess
        - quarantine_recess
        - operator_handoff_clearance
        - rear_cable_keepout
        - right_service_keepout
        - mounts
        + deck_perimeter_rails()
}

fn deck_perimeter_rails() -> Part {
    let rear = centered_cube(
        "connector_topology_station_rear_locator_rail",
        DECK_X - 110.0,
        18.0,
        26.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 34.0, DECK_Z / 2.0 + 13.0);
    let left = centered_cube(
        "connector_topology_station_left_locator_rail",
        18.0,
        DECK_Y - 120.0,
        24.0,
    )
    .translate(-DECK_X / 2.0 + 34.0, 12.0, DECK_Z / 2.0 + 12.0);
    let front_low_lip = centered_cube(
        "connector_topology_station_front_low_lip",
        DECK_X - 320.0,
        12.0,
        14.0,
    )
    .translate(-90.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0 + 7.0);

    rear + left + front_low_lip
}

fn twenty_chip_cassette_datum() -> Part {
    let tray = centered_cube(
        "connector_topology_twenty_chip_cassette_datum_tray",
        CASSETTE_X,
        CASSETTE_Y,
        DATUM_RAIL_Z,
    );

    let basin = centered_cube(
        "connector_topology_twenty_chip_cassette_relief_basin",
        CHIP_ARRAY_X + 18.0,
        CHIP_ARRAY_Y + 18.0,
        DATUM_RAIL_Z + 2.0,
    )
    .translate(0.0, 0.0, 5.0);

    let mut chip_pockets = Part::empty("connector_topology_chip_pockets");
    let mut fiducials = Part::empty("connector_topology_chip_fiducials");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let (x, y) = chip_center(col, row);
            chip_pockets = chip_pockets
                + centered_cube(
                    format!("connector_topology_chip_pocket_{col}_{row}"),
                    REVC_CHIP_LENGTH + CHIP_POCKET_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + CHIP_POCKET_CLEARANCE * 2.0,
                    CHIP_POCKET_DEPTH + 0.2,
                )
                .translate(
                    x,
                    y,
                    DATUM_RAIL_Z / 2.0 - CHIP_POCKET_DEPTH / 2.0 + 0.1,
                );

            for (idx, (dx, dy)) in chip_fiducial_offsets().iter().enumerate() {
                fiducials = fiducials
                    + centered_cylinder(
                        format!("connector_topology_chip_fiducial_{col}_{row}_{idx}"),
                        3.0 / 2.0,
                        DATUM_RAIL_Z + 4.0,
                        20,
                    )
                    .translate(x + dx, y + dy, 0.0);
            }
        }
    }

    let back_stop = centered_cube(
        "connector_topology_cassette_back_y_datum",
        CHIP_ARRAY_X + 38.0,
        DATUM_RAIL_W,
        DATUM_RAIL_Z + 14.0,
    )
    .translate(0.0, CHIP_ARRAY_Y / 2.0 + DATUM_RAIL_W / 2.0 + 10.0, 7.0);
    let left_stop = centered_cube(
        "connector_topology_cassette_left_x_datum",
        DATUM_RAIL_W,
        CHIP_ARRAY_Y + 38.0,
        DATUM_RAIL_Z + 12.0,
    )
    .translate(-(CHIP_ARRAY_X / 2.0 + DATUM_RAIL_W / 2.0 + 10.0), 0.0, 6.0);
    let front_low_lip = centered_cube(
        "connector_topology_cassette_front_low_lip",
        CHIP_ARRAY_X + 38.0,
        10.0,
        12.0,
    )
    .translate(0.0, -(CHIP_ARRAY_Y / 2.0 + 15.0), -6.0);

    let datum_pin_bosses = cassette_datum_pin_bosses();

    tray - basin - chip_pockets - fiducials
        + back_stop
        + left_stop
        + front_low_lip
        + datum_pin_bosses
}

fn cassette_datum_pin_bosses() -> Part {
    let mut bosses = Part::empty("connector_topology_cassette_datum_pin_bosses");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 34.0), -(CASSETTE_Y / 2.0 - 34.0)),
        (CASSETTE_X / 2.0 - 34.0, -(CASSETTE_Y / 2.0 - 34.0)),
        (-(CASSETTE_X / 2.0 - 34.0), CASSETTE_Y / 2.0 - 34.0),
        (CASSETTE_X / 2.0 - 34.0, CASSETTE_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("connector_topology_cassette_datum_boss_{i}"),
            16.0 / 2.0,
            12.0,
            32,
        )
        .translate(*x, *y, DATUM_RAIL_Z / 2.0 + 6.0);
        let hole = centered_cylinder(
            format!("connector_topology_cassette_datum_pin_clearance_{i}"),
            5.0 / 2.0,
            15.0,
            24,
        )
        .translate(*x, *y, DATUM_RAIL_Z / 2.0 + 6.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn barcode_rfid_scan_zone() -> Part {
    let plate = centered_cube(
        "connector_topology_barcode_rfid_scan_zone_plate",
        SCAN_ZONE_X,
        SCAN_ZONE_Y,
        SCAN_ZONE_Z,
    );

    let barcode_slot = centered_cube(
        "connector_topology_barcode_window_slot",
        SCAN_ZONE_X - 72.0,
        24.0,
        SCAN_ZONE_Z + 2.0,
    )
    .translate(0.0, -20.0, 1.0);
    let rfid_antenna_pocket =
        centered_cube("connector_topology_rfid_antenna_pocket", 96.0, 54.0, 7.0).translate(
            -(SCAN_ZONE_X / 2.0 - 70.0),
            16.0,
            SCAN_ZONE_Z / 2.0 - 3.0,
        );
    let operator_badge_pad =
        centered_cube("connector_topology_operator_badge_pad", 68.0, 42.0, 5.0).translate(
            SCAN_ZONE_X / 2.0 - 62.0,
            18.0,
            SCAN_ZONE_Z / 2.0 + 2.5,
        );

    let left_fence = centered_cube(
        "connector_topology_barcode_left_fence",
        8.0,
        SCAN_ZONE_Y + 12.0,
        18.0,
    )
    .translate(-(SCAN_ZONE_X / 2.0 + 4.0), 0.0, SCAN_ZONE_Z / 2.0 + 9.0);
    let right_fence = centered_cube(
        "connector_topology_barcode_right_fence",
        8.0,
        SCAN_ZONE_Y + 12.0,
        18.0,
    )
    .translate(SCAN_ZONE_X / 2.0 + 4.0, 0.0, SCAN_ZONE_Z / 2.0 + 9.0);

    plate - barcode_slot - rfid_antenna_pocket + operator_badge_pad + left_fence + right_fence
}

fn connector_id_scan_comb() -> Part {
    let body = centered_cube(
        "connector_topology_connector_id_scan_comb_body",
        COMB_X,
        COMB_Y,
        COMB_Z,
    );

    let mut slots = Part::empty("connector_topology_connector_id_scan_comb_slots");
    for slot in 0..CONNECTOR_ID_SLOTS {
        let (x, y) = connector_slot_center(slot);
        let tube_channel = centered_cylinder(
            format!("connector_topology_connector_slot_round_{slot}"),
            CONNECTOR_SLOT_D / 2.0,
            COMB_X + 2.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, 0.0);
        let top_scan_slit = centered_cube(
            format!("connector_topology_connector_scan_slit_{slot}"),
            7.2,
            12.0,
            COMB_Z + 2.0,
        )
        .translate(x, y, 4.0);
        slots = slots + tube_channel + top_scan_slit;
    }

    let mut group_ticks = Part::empty("connector_topology_connector_comb_group_ticks");
    for chip in 0..CHIP_COUNT {
        let y = -((CHIP_COUNT as f64 - 1.0) * CONNECTOR_ROW_PITCH_Y) / 2.0
            + chip as f64 * CONNECTOR_ROW_PITCH_Y;
        group_ticks = group_ticks
            + centered_cube(
                format!("connector_topology_connector_comb_chip_tick_{chip}"),
                COMB_X + 10.0,
                1.4,
                3.0,
            )
            .translate(0.0, y - CONNECTOR_ROW_PITCH_Y / 2.0, COMB_Z / 2.0 + 1.5);
    }

    body - slots + group_ticks
}

fn pump_valve_channel_map_plate() -> Part {
    let plate = centered_cube(
        "connector_topology_pump_valve_channel_map_plate",
        MAP_PLATE_X,
        MAP_PLATE_Y,
        MAP_PLATE_Z,
    );
    let title_strip = centered_cube(
        "connector_topology_pump_valve_channel_title_strip",
        MAP_PLATE_X - 24.0,
        18.0,
        4.0,
    )
    .translate(0.0, MAP_PLATE_Y / 2.0 - 19.0, MAP_PLATE_Z / 2.0 + 2.0);

    let mut cuts = Part::empty("connector_topology_channel_map_cuts");
    let mut pads = Part::empty("connector_topology_channel_map_pads");
    let pitch_x = 40.0;
    let pitch_y = 38.0;
    for channel in 0..PUMP_VALVE_CHANNELS {
        let col = channel % CHANNEL_MAP_COLS;
        let row = channel / CHANNEL_MAP_COLS;
        let x = -((CHANNEL_MAP_COLS as f64 - 1.0) * pitch_x) / 2.0 + col as f64 * pitch_x;
        let y = -42.0 + ((CHANNEL_MAP_ROWS as f64 - 1.0) * pitch_y) / 2.0 - row as f64 * pitch_y;
        cuts = cuts
            + centered_cylinder(
                format!("connector_topology_channel_map_pump_port_{channel}"),
                5.2 / 2.0,
                MAP_PLATE_Z + 2.0,
                24,
            )
            .translate(x - 8.0, y, 0.0)
            + centered_cylinder(
                format!("connector_topology_channel_map_valve_port_{channel}"),
                4.2 / 2.0,
                MAP_PLATE_Z + 2.0,
                20,
            )
            .translate(x + 9.0, y, 0.0);
        pads = pads
            + centered_cube(
                format!("connector_topology_channel_map_label_pad_{channel}"),
                30.0,
                18.0,
                2.5,
            )
            .translate(x, y + 16.0, MAP_PLATE_Z / 2.0 + 1.25);
    }

    plate - cuts + pads + title_strip
}

fn leak_test_port_bar() -> Part {
    let bar = centered_cube(
        "connector_topology_leak_test_port_placeholder_bar",
        LEAK_PORT_BAR_X,
        LEAK_PORT_BAR_Y,
        LEAK_PORT_BAR_Z,
    );

    let mut ports = Part::empty("connector_topology_leak_test_ports");
    let mut bosses = Part::empty("connector_topology_leak_test_port_bosses");
    for port in 0..CHIP_COUNT {
        let (x, y) = leak_port_center(port);
        let boss = centered_cylinder(
            format!("connector_topology_leak_port_boss_{port}"),
            17.0 / 2.0,
            8.0,
            32,
        )
        .translate(x, y, LEAK_PORT_BAR_Z / 2.0 + 4.0);
        let clearance = centered_cylinder(
            format!("connector_topology_leak_port_placeholder_{port}"),
            LEAK_PORT_D / 2.0,
            LEAK_PORT_BAR_Z + 12.0,
            24,
        )
        .translate(x, y, 3.0);
        bosses = bosses + boss;
        ports = ports + clearance;
    }

    let waste_ridge = centered_cube(
        "connector_topology_leak_test_witness_ridge",
        LEAK_PORT_BAR_X - 38.0,
        6.0,
        5.0,
    )
    .translate(
        0.0,
        -LEAK_PORT_BAR_Y / 2.0 + 11.0,
        LEAK_PORT_BAR_Z / 2.0 + 2.5,
    );

    bar + bosses + waste_ridge - ports
}

fn camera_illumination_scan_bridge() -> Part {
    let left_post = bridge_post("left").translate(
        -(BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0),
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let right_post = bridge_post("right").translate(
        BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "connector_topology_scan_bridge_camera_crossbeam",
        BRIDGE_SPAN_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let camera_sled = centered_cube(
        "connector_topology_scan_bridge_camera_sled",
        92.0,
        52.0,
        34.0,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 17.0);
    let lens_placeholder = centered_cylinder(
        "connector_topology_scan_bridge_lens_placeholder",
        18.0 / 2.0,
        28.0,
        32,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z - 14.0);

    let front_led_bar = centered_cube(
        "connector_topology_scan_bridge_front_illumination_bar",
        CHIP_ARRAY_X + 38.0,
        12.0,
        14.0,
    )
    .translate(
        0.0,
        -(REVC_CHIP_WIDTH / 2.0 + 32.0),
        BRIDGE_UNDERSIDE_Z - 19.0,
    );
    let rear_led_bar = centered_cube(
        "connector_topology_scan_bridge_rear_illumination_bar",
        CHIP_ARRAY_X + 38.0,
        12.0,
        14.0,
    )
    .translate(0.0, REVC_CHIP_WIDTH / 2.0 + 32.0, BRIDGE_UNDERSIDE_Z - 19.0);
    let cable_chain = centered_cube(
        "connector_topology_scan_bridge_rear_cable_chain_keepout",
        BRIDGE_SPAN_X - 90.0,
        14.0,
        16.0,
    )
    .translate(0.0, BRIDGE_POST_Y / 2.0 + 18.0, BRIDGE_UNDERSIDE_Z + 6.0);

    left_post
        + right_post
        + beam
        + camera_sled
        + lens_placeholder
        + front_led_bar
        + rear_led_bar
        + cable_chain
}

fn bridge_post(name: &str) -> Part {
    let post = centered_cube(
        format!("connector_topology_scan_bridge_{name}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    );
    let lightening_slot = centered_cube(
        format!("connector_topology_scan_bridge_{name}_post_lightening_slot"),
        BRIDGE_POST_X + 2.0,
        BRIDGE_POST_Y - 22.0,
        BRIDGE_POST_Z - 42.0,
    )
    .translate(0.0, 0.0, 8.0);
    let mount_holes = centered_cylinder(
        format!("connector_topology_scan_bridge_{name}_front_m5"),
        5.3 / 2.0,
        BRIDGE_POST_Y + 2.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -14.0, -BRIDGE_POST_Z / 2.0 + 18.0)
        + centered_cylinder(
            format!("connector_topology_scan_bridge_{name}_rear_m5"),
            5.3 / 2.0,
            BRIDGE_POST_Y + 2.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 14.0, -BRIDGE_POST_Z / 2.0 + 18.0);
    post - lightening_slot - mount_holes
}

fn rejected_mismatch_quarantine_pocket() -> Part {
    let tray = centered_cube(
        "connector_topology_quarantine_outer_tray",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let rejected_basin = centered_cube(
        "connector_topology_rejected_chip_basin",
        QUARANTINE_X / 2.0 - QUARANTINE_WALL * 2.0,
        QUARANTINE_Y - QUARANTINE_WALL * 3.0,
        QUARANTINE_Z - QUARANTINE_WALL,
    )
    .translate(-(QUARANTINE_X / 4.0), 0.0, QUARANTINE_WALL / 2.0);
    let mismatch_basin = centered_cube(
        "connector_topology_mismatch_connector_basin",
        QUARANTINE_X / 2.0 - QUARANTINE_WALL * 2.0,
        QUARANTINE_Y - QUARANTINE_WALL * 3.0,
        QUARANTINE_Z - QUARANTINE_WALL,
    )
    .translate(QUARANTINE_X / 4.0, 0.0, QUARANTINE_WALL / 2.0);
    let divider = centered_cube(
        "connector_topology_quarantine_center_divider",
        QUARANTINE_WALL,
        QUARANTINE_Y - QUARANTINE_WALL,
        QUARANTINE_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);
    let barcode_tab = centered_cube(
        "connector_topology_quarantine_reject_reason_scan_tab",
        QUARANTINE_X - 34.0,
        20.0,
        5.0,
    )
    .translate(0.0, -QUARANTINE_Y / 2.0 + 18.0, QUARANTINE_Z / 2.0 + 2.5);

    tray - rejected_basin - mismatch_basin + divider + barcode_tab
}

fn handoff_and_service_keepout_gauges() -> Part {
    let operator_front_gauge = centered_cube(
        "connector_topology_operator_handoff_front_gauge",
        CASSETTE_X + ROBOT_HANDOFF_CLEARANCE_X,
        10.0,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        -DECK_Y / 2.0 + OPERATOR_HANDOFF_CLEARANCE_Y + 20.0,
        0.0,
    );
    let robot_left_gauge = centered_cube(
        "connector_topology_robot_handoff_left_gauge",
        10.0,
        CASSETTE_Y + 34.0,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X - CASSETTE_X / 2.0 - 52.0,
        CASSETTE_CENTER_Y,
        0.0,
    );
    let robot_right_gauge = centered_cube(
        "connector_topology_robot_handoff_right_gauge",
        10.0,
        CASSETTE_Y + 34.0,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X + CASSETTE_X / 2.0 + 52.0,
        CASSETTE_CENTER_Y,
        0.0,
    );
    let rear_cable_bar = centered_cube(
        "connector_topology_rear_service_cable_keepout_bar",
        DECK_X - 180.0,
        12.0,
        18.0,
    )
    .translate(0.0, DECK_Y / 2.0 - CABLE_KEEP_OUT_Y - 4.0, 0.0);
    let right_service_bar = centered_cube(
        "connector_topology_right_service_keepout_bar",
        12.0,
        CASSETTE_Y + 110.0,
        18.0,
    )
    .translate(DECK_X / 2.0 - SERVICE_KEEP_OUT_X - 4.0, 20.0, 0.0);

    operator_front_gauge + robot_left_gauge + robot_right_gauge + rear_cable_bar + right_service_bar
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    let x = -CHIP_ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * CHIP_PITCH_X;
    let y = -CHIP_ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * CHIP_PITCH_Y;
    (x, y)
}

fn chip_fiducial_offsets() -> [(f64, f64); 3] {
    [
        (
            -(REVC_CHIP_LENGTH / 2.0 + 2.4),
            -(REVC_CHIP_WIDTH / 2.0 + 2.4),
        ),
        (-(REVC_CHIP_LENGTH / 2.0 + 2.4), REVC_CHIP_WIDTH / 2.0 + 2.4),
        (REVC_CHIP_LENGTH / 2.0 + 2.4, -(REVC_CHIP_WIDTH / 2.0 + 2.4)),
    ]
}

fn connector_slot_center(slot: usize) -> (f64, f64) {
    let chip = slot / CONNECTORS_PER_CHIP;
    let connector = slot % CONNECTORS_PER_CHIP;
    let x = -((CONNECTORS_PER_CHIP as f64 - 1.0) * CONNECTOR_LANE_PITCH_X) / 2.0
        + connector as f64 * CONNECTOR_LANE_PITCH_X;
    let y = -((CHIP_COUNT as f64 - 1.0) * CONNECTOR_ROW_PITCH_Y) / 2.0
        + chip as f64 * CONNECTOR_ROW_PITCH_Y;
    (x, y)
}

fn leak_port_center(port: usize) -> (f64, f64) {
    let row = port / LEAK_PORTS_PER_ROW;
    let col = port % LEAK_PORTS_PER_ROW;
    let x = -((LEAK_PORTS_PER_ROW as f64 - 1.0) * LEAK_PORT_PITCH_X) / 2.0
        + col as f64 * LEAK_PORT_PITCH_X;
    let y =
        -((LEAK_PORT_ROWS as f64 - 1.0) * LEAK_PORT_PITCH_Y) / 2.0 + row as f64 * LEAK_PORT_PITCH_Y;
    (x, y)
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 34.0), -(DECK_Y / 2.0 - 34.0)),
        (DECK_X / 2.0 - 34.0, -(DECK_Y / 2.0 - 34.0)),
        (-(DECK_X / 2.0 - 34.0), DECK_Y / 2.0 - 34.0),
        (DECK_X / 2.0 - 34.0, DECK_Y / 2.0 - 34.0),
        (0.0, -(DECK_Y / 2.0 - 34.0)),
        (0.0, DECK_Y / 2.0 - 34.0),
        (-(DECK_X / 2.0 - 34.0), 0.0),
        (DECK_X / 2.0 - 34.0, 0.0),
    ]
}

fn bridge_underside_clearance() -> f64 {
    BRIDGE_UNDERSIDE_Z - REVC_TOTAL_HEIGHT
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[cfg(test)]
fn chip_positions() -> Vec<(f64, f64)> {
    let mut positions = Vec::with_capacity(CHIP_COUNT);
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            positions.push(chip_center(col, row));
        }
    }
    positions
}

#[cfg(test)]
fn cassette_rect() -> Rect {
    Rect {
        x: CASSETTE_CENTER_X,
        y: CASSETTE_CENTER_Y,
        w: CASSETTE_X,
        h: CASSETTE_Y,
    }
}

#[cfg(test)]
fn quarantine_rect() -> Rect {
    Rect {
        x: QUARANTINE_CENTER_X,
        y: QUARANTINE_CENTER_Y,
        w: QUARANTINE_X,
        h: QUARANTINE_Y,
    }
}

#[cfg(test)]
fn scan_zone_rect() -> Rect {
    Rect {
        x: SCAN_ZONE_CENTER_X,
        y: SCAN_ZONE_CENTER_Y,
        w: SCAN_ZONE_X,
        h: SCAN_ZONE_Y,
    }
}

#[cfg(test)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_position_coverage_matches_cassette_datum() {
        let positions = chip_positions();
        assert_eq!(positions.len(), CHIP_COUNT);
        assert_eq!(CHIP_COUNT, 20);
        assert_eq!(CHIP_COLS * CHIP_ROWS, 20);
        assert!(CHIP_ARRAY_X < CASSETTE_X - CASSETTE_MARGIN_X);
        assert!(CHIP_ARRAY_Y < CASSETTE_Y - CASSETTE_MARGIN_Y);

        let first = chip_center(0, 0);
        let last = chip_center(CHIP_COLS - 1, CHIP_ROWS - 1);
        assert!((last.0 - first.0 - (CHIP_COLS as f64 - 1.0) * CHIP_PITCH_X).abs() < 0.01);
        assert!((last.1 - first.1 - (CHIP_ROWS as f64 - 1.0) * CHIP_PITCH_Y).abs() < 0.01);
    }

    #[test]
    fn connector_map_capacity_covers_all_tubing_id_inputs() {
        assert_eq!(CONNECTOR_ID_SLOTS, CHIP_COUNT * CONNECTORS_PER_CHIP);
        assert!(CHANNEL_MAP_CAPACITY >= CONNECTOR_ID_SLOTS);
        assert!(PUMP_VALVE_CHANNELS >= CHIP_COUNT);

        let last_slot = connector_slot_center(CONNECTOR_ID_SLOTS - 1);
        assert!(last_slot.0.abs() < COMB_X / 2.0 - 20.0);
        assert!(last_slot.1.abs() < COMB_Y / 2.0 - 20.0);
    }

    #[test]
    fn scan_bridge_clearance_covers_camera_and_robot_path() {
        assert!(bridge_underside_clearance() >= 80.0);
        assert!(BRIDGE_SPAN_X > CASSETTE_X + BRIDGE_SIDE_CLEARANCE);
        assert!(BRIDGE_UNDERSIDE_Z > REVC_TOTAL_HEIGHT + 70.0);
        assert!(BRIDGE_POST_Z < 140.0);
    }

    #[test]
    fn rejected_items_are_segregated_from_scan_and_cassette_flow() {
        let quarantine = quarantine_rect();
        assert!(!rects_overlap(quarantine, cassette_rect()));
        assert!(!rects_overlap(quarantine, scan_zone_rect()));
        assert!(horizontal_gap(quarantine, cassette_rect()) >= SEGREGATION_GAP_MIN);
        assert!(QUARANTINE_WALL >= 8.0);
        assert!(QUARANTINE_CENTER_X > CASSETTE_CENTER_X + CASSETTE_X / 2.0);
    }
}
