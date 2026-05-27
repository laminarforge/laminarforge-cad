use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Cassette deviation/quarantine station for the closed automated tissue-chip workflow.
//
// Intent:
// - Pull suspect or deviated cassette lots out of normal incubator flow without
//   relying on an operator deciding where to set them down.
// - Physically separate released, hold, and reject cassette bays with hard
//   dividers, visible status/deviation placarding, identity scan lands, and
//   sample/archive handoff positions.
// - Represent cleanable leak containment, sealed transfer tote docking, an
//   environmental logger pocket, contamination-suspect cover envelope, and
//   robot/service keepout gauges. This is product-concept CAD only, not a
//   biological protocol or release procedure.

const OUTPUTS: [&str; 11] = [
    "output/cassette_deviation_quarantine_station_leak_tray_base.stl",
    "output/cassette_deviation_quarantine_station_status_bay_array.stl",
    "output/cassette_deviation_quarantine_station_identity_scan_lands.stl",
    "output/cassette_deviation_quarantine_station_deviation_tag_panel.stl",
    "output/cassette_deviation_quarantine_station_environmental_logger_pocket.stl",
    "output/cassette_deviation_quarantine_station_sealed_transfer_tote_interface.stl",
    "output/cassette_deviation_quarantine_station_isolation_cover_envelope.stl",
    "output/cassette_deviation_quarantine_station_sample_archive_handoff.stl",
    "output/cassette_deviation_quarantine_station_flow_separation_barrier.stl",
    "output/cassette_deviation_quarantine_station_robot_service_keepouts.stl",
    "output/cassette_deviation_quarantine_station_assembly.stl",
];

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_COUNT: usize = CHIP_COLS * CHIP_ROWS;

const CHIP_GUTTER: f64 = 6.0;
const CASSETTE_MARGIN_X: f64 = 38.0;
const CASSETTE_MARGIN_Y: f64 = 40.0;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_X: f64 = CHIP_ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = REVC_TOTAL_HEIGHT + 18.0;

const DECK_X: f64 = 2280.0;
const DECK_Y: f64 = 1900.0;
const DECK_Z: f64 = 20.0;
const TRAY_CURB_W: f64 = 18.0;
const TRAY_CURB_Z: f64 = 34.0;
const LEAK_SUMP_X: f64 = 2060.0;
const LEAK_SUMP_Y: f64 = 1680.0;
const LEAK_SUMP_DEPTH: f64 = 8.0;
const DRAIN_CHANNEL_W: f64 = 12.0;
const DRAIN_PORT_D: f64 = 18.0;

const BAY_COUNT: usize = 3;
const BAY_CAPACITY: usize = 2;
const TOTAL_CASSETTE_POSITIONS: usize = BAY_COUNT * BAY_CAPACITY;
const BAY_X: f64 = CASSETTE_X + 42.0;
const BAY_Y: f64 = CASSETTE_Y * 2.0 + 86.0;
const BAY_WALL: f64 = 16.0;
const BAY_RAIL_Z: f64 = 36.0;
const BAY_PITCH_X: f64 = BAY_X + 52.0;
const BAY_CENTER_Y: f64 = 38.0;
const BAY_START_X: f64 = -BAY_PITCH_X;
const BAY_SOCKET_DEPTH: f64 = 6.0;
const STATUS_FLAG_X: f64 = 132.0;
const STATUS_FLAG_Y: f64 = 10.0;
const STATUS_FLAG_Z: f64 = 42.0;

const SCAN_LAND_COUNT: usize = TOTAL_CASSETTE_POSITIONS + 4;
const SCAN_BAR_X: f64 = 1660.0;
const SCAN_BAR_Y: f64 = 108.0;
const SCAN_BAR_Z: f64 = 16.0;
const SCAN_LAND_X: f64 = 72.0;
const SCAN_LAND_Y: f64 = 44.0;
const RFID_LAND_X: f64 = 96.0;
const RFID_LAND_Y: f64 = 56.0;

const TAG_PANEL_X: f64 = 1640.0;
const TAG_PANEL_Y: f64 = 26.0;
const TAG_PANEL_Z: f64 = 210.0;
const TAG_CARD_COUNT: usize = 6;
const TAG_CARD_X: f64 = 118.0;
const TAG_CARD_Z: f64 = 70.0;

const LOGGER_POCKET_X: f64 = 238.0;
const LOGGER_POCKET_Y: f64 = 116.0;
const LOGGER_POCKET_Z: f64 = 44.0;
const LOGGER_SLOT_COUNT: usize = 4;
const LOGGER_SENSOR_PORTS: usize = 5;

const TOTE_DOCK_X: f64 = 430.0;
const TOTE_DOCK_Y: f64 = 190.0;
const TOTE_DOCK_Z: f64 = 92.0;
const TOTE_GASKET_X: f64 = 378.0;
const TOTE_GASKET_Y: f64 = 18.0;
const TOTE_GASKET_Z: f64 = 118.0;
const TOTE_LATCH_COUNT: usize = 4;

const COVER_X: f64 = BAY_X * 2.0 + BAY_PITCH_X + 72.0;
const COVER_Y: f64 = BAY_Y + 76.0;
const COVER_Z: f64 = 238.0;
const COVER_WALL: f64 = 8.0;
const COVER_Z_CLEARANCE: f64 = 72.0;
const COVER_FILTER_COUNT: usize = 4;

const SAMPLE_ARCHIVE_COUNT: usize = 8;
const SAMPLE_RACK_X: f64 = 520.0;
const SAMPLE_RACK_Y: f64 = 138.0;
const SAMPLE_RACK_Z: f64 = 38.0;
const SAMPLE_TUBE_D: f64 = 18.0;
const SAMPLE_VIAL_D: f64 = 13.0;

const FLOW_BARRIER_X: f64 = 2180.0;
const FLOW_BARRIER_Y: f64 = 28.0;
const FLOW_BARRIER_Z: f64 = 126.0;
const NORMAL_FLOW_KEEP_AWAY_Y: f64 = 210.0;
const SIDE_WALL_GAP: f64 = 60.0;

const ROBOT_CLEARANCE_X: f64 = DECK_X + 260.0;
const ROBOT_CLEARANCE_Y: f64 = DECK_Y + 220.0;
const ROBOT_CLEARANCE_Z: f64 = 360.0;
const FRONT_OPERATOR_CLEARANCE_Y: f64 = 440.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 260.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let bays = status_bay_array();
    export(OUTPUTS[1], &bays);

    let scan = identity_scan_lands();
    export(OUTPUTS[2], &scan);

    let tags = deviation_tag_panel();
    export(OUTPUTS[3], &tags);

    let logger = environmental_logger_pocket();
    export(OUTPUTS[4], &logger);

    let tote = sealed_transfer_tote_interface();
    export(OUTPUTS[5], &tote);

    let cover = contamination_suspect_isolation_cover();
    export(OUTPUTS[6], &cover);

    let sample = sample_archive_handoff_positions();
    export(OUTPUTS[7], &sample);

    let barrier = flow_separation_barrier();
    export(OUTPUTS[8], &barrier);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base + bays + scan + tags + logger + tote + cover + sample + barrier + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Cassette deviation/quarantine station:");
    println!("  Leak tray deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Status bays:                 released/hold/reject, {BAY_CAPACITY} cassette slots each, {TOTAL_CASSETTE_POSITIONS} total"
    );
    println!(
        "  Cassette envelope:           {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm x {CASSETTE_Z:.1}mm for 4x5 Rev C chip array"
    );
    println!(
        "  Traceability:                {SCAN_LAND_COUNT} barcode/RFID lands, {TAG_CARD_COUNT} deviation tag cards, {LOGGER_SLOT_COUNT} logger slots"
    );
    println!(
        "  Transfer/isolation:          {TOTE_DOCK_X:.0}mm sealed tote dock and {COVER_X:.0}mm x {COVER_Y:.0}mm x {COVER_Z:.0}mm suspect-cover envelope"
    );
    println!(
        "  Sample/archive handoff:      {SAMPLE_ARCHIVE_COUNT} paired sample/archive positions"
    );
    println!(
        "  Flow separation:             {FLOW_BARRIER_X:.0}mm hard barrier with {NORMAL_FLOW_KEEP_AWAY_Y:.0}mm normal-flow keep-away lane"
    );
    println!(
        "  Robot/service keepouts:      robot {ROBOT_CLEARANCE_X:.0}mm x {ROBOT_CLEARANCE_Y:.0}mm x {ROBOT_CLEARANCE_Z:.0}mm, front {FRONT_OPERATOR_CLEARANCE_Y:.0}mm, rear {REAR_SERVICE_CLEARANCE_Y:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(CHIP_COUNT, 20);
    assert_eq!(BAY_COUNT, 3);
    assert_eq!(TOTAL_CASSETTE_POSITIONS, 6);
    assert!(bay_span_x() < DECK_X - 160.0);
    assert!(BAY_Y < DECK_Y - 210.0);
    assert!(SCAN_LAND_COUNT >= TOTAL_CASSETTE_POSITIONS + 3);
    assert!(cover_clearance_z() >= CASSETTE_Z + COVER_Z_CLEARANCE);
    assert!(reject_bay_center_x() - hold_bay_center_x() >= BAY_PITCH_X);
    assert!(hold_bay_center_x() - released_bay_center_x() >= BAY_PITCH_X);
    assert!(quarantine_zone_rear_edge() <= barrier_front_edge() - 16.0);
    assert!(normal_incubator_flow_lane_width() >= NORMAL_FLOW_KEEP_AWAY_Y);
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "cassette_quarantine_leak_tray_cleanable_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        "cassette_quarantine_leak_sump_recess",
        LEAK_SUMP_X,
        LEAK_SUMP_Y,
        LEAK_SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -24.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0 + 0.5);
    let drain_channel = centered_cube(
        "cassette_quarantine_front_drain_channel",
        LEAK_SUMP_X - 80.0,
        DRAIN_CHANNEL_W,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 64.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0);
    let drain_port = centered_cylinder(
        "cassette_quarantine_bulkhead_drain_port",
        DRAIN_PORT_D / 2.0,
        TRAY_CURB_W + 18.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 26.0, DECK_Z - 9.0);

    deck - sump - drain_channel - drain_port + tray_curbs() + mount_holes_and_datum_rails()
}

fn tray_curbs() -> Part {
    let left = centered_cube(
        "cassette_quarantine_left_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 42.0,
        TRAY_CURB_Z,
    )
    .translate(
        -(DECK_X / 2.0 - TRAY_CURB_W / 2.0),
        12.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "cassette_quarantine_right_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 42.0,
        TRAY_CURB_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_CURB_W / 2.0,
        12.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let rear = centered_cube(
        "cassette_quarantine_rear_spill_curb",
        DECK_X - 36.0,
        TRAY_CURB_W,
        TRAY_CURB_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let front_left = centered_cube(
        "cassette_quarantine_front_low_lip_left",
        DECK_X * 0.42,
        12.0,
        20.0,
    )
    .translate(-(DECK_X * 0.23), -DECK_Y / 2.0 + 18.0, DECK_Z + 10.0);
    let front_right = centered_cube(
        "cassette_quarantine_front_low_lip_right",
        DECK_X * 0.42,
        12.0,
        20.0,
    )
    .translate(DECK_X * 0.23, -DECK_Y / 2.0 + 18.0, DECK_Z + 10.0);

    left + right + rear + front_left + front_right
}

fn mount_holes_and_datum_rails() -> Part {
    let mut holes = Part::empty("cassette_quarantine_mount_holes");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 48.0), -(DECK_Y / 2.0 - 46.0)),
        (DECK_X / 2.0 - 48.0, -(DECK_Y / 2.0 - 46.0)),
        (-(DECK_X / 2.0 - 48.0), DECK_Y / 2.0 - 46.0),
        (DECK_X / 2.0 - 48.0, DECK_Y / 2.0 - 46.0),
        (0.0, -(DECK_Y / 2.0 - 46.0)),
        (0.0, DECK_Y / 2.0 - 46.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cassette_quarantine_m6_clearance_{i}"),
                6.6 / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }

    let rear_datum = centered_cube(
        "cassette_quarantine_underside_rear_robot_datum_rail",
        DECK_X - 180.0,
        16.0,
        10.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 72.0, 5.0);
    let left_datum = centered_cube(
        "cassette_quarantine_underside_left_robot_datum_rail",
        16.0,
        DECK_Y - 160.0,
        10.0,
    )
    .translate(-(DECK_X / 2.0 - 78.0), 0.0, 5.0);
    let right_datum = centered_cube(
        "cassette_quarantine_underside_right_robot_datum_rail",
        16.0,
        DECK_Y - 160.0,
        10.0,
    )
    .translate(DECK_X / 2.0 - 78.0, 0.0, 5.0);

    rear_datum + left_datum + right_datum - holes
}

fn status_bay_array() -> Part {
    status_bay("released", released_bay_center_x(), "qa_released")
        + status_bay("hold", hold_bay_center_x(), "deviation_hold")
        + status_bay("reject", reject_bay_center_x(), "rejected")
        + inter_bay_hard_dividers()
}

fn status_bay(name: &str, center_x: f64, label: &str) -> Part {
    let platform = centered_cube(
        format!("cassette_quarantine_{name}_bay_recessed_platform"),
        BAY_X,
        BAY_Y,
        18.0,
    )
    .translate(center_x, BAY_CENTER_Y, DECK_Z + 9.0);
    let socket_cut_0 = cassette_socket_cut(name, 0, center_x);
    let socket_cut_1 = cassette_socket_cut(name, 1, center_x);
    let rear_stop = centered_cube(
        format!("cassette_quarantine_{name}_rear_cassette_stop"),
        BAY_X - 38.0,
        BAY_WALL,
        BAY_RAIL_Z,
    )
    .translate(
        center_x,
        BAY_CENTER_Y + BAY_Y / 2.0 - BAY_WALL / 2.0,
        DECK_Z + BAY_RAIL_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        format!("cassette_quarantine_{name}_front_low_lip"),
        BAY_X - 38.0,
        10.0,
        20.0,
    )
    .translate(center_x, BAY_CENTER_Y - BAY_Y / 2.0 + 16.0, DECK_Z + 10.0);
    let left_rail = centered_cube(
        format!("cassette_quarantine_{name}_left_side_rail"),
        BAY_WALL,
        BAY_Y - 42.0,
        BAY_RAIL_Z,
    )
    .translate(
        center_x - BAY_X / 2.0 + BAY_WALL / 2.0,
        BAY_CENTER_Y,
        DECK_Z + BAY_RAIL_Z / 2.0,
    );
    let right_rail = centered_cube(
        format!("cassette_quarantine_{name}_right_side_rail"),
        BAY_WALL,
        BAY_Y - 42.0,
        BAY_RAIL_Z,
    )
    .translate(
        center_x + BAY_X / 2.0 - BAY_WALL / 2.0,
        BAY_CENTER_Y,
        DECK_Z + BAY_RAIL_Z / 2.0,
    );
    let center_separator = centered_cube(
        format!("cassette_quarantine_{name}_cassette_pair_separator"),
        BAY_X - 52.0,
        12.0,
        26.0,
    )
    .translate(center_x, BAY_CENTER_Y, DECK_Z + 13.0);
    let flag = centered_cube(
        format!("cassette_quarantine_{label}_front_status_flag"),
        STATUS_FLAG_X,
        STATUS_FLAG_Y,
        STATUS_FLAG_Z,
    )
    .translate(
        center_x,
        BAY_CENTER_Y - BAY_Y / 2.0 - STATUS_FLAG_Y / 2.0 - 10.0,
        DECK_Z + 60.0,
    );
    let two_slot_numbers = slot_number_tabs(name, center_x);

    platform - socket_cut_0 - socket_cut_1
        + rear_stop
        + front_low_lip
        + left_rail
        + right_rail
        + center_separator
        + flag
        + two_slot_numbers
        + cassette_edge_datums(name, center_x)
}

fn cassette_socket_cut(name: &str, slot: usize, center_x: f64) -> Part {
    centered_cube(
        format!("cassette_quarantine_{name}_slot_{slot}_cassette_shadow_cut"),
        CASSETTE_X + 8.0,
        CASSETTE_Y + 8.0,
        BAY_SOCKET_DEPTH,
    )
    .translate(
        center_x,
        bay_slot_y(slot),
        DECK_Z + 18.0 - BAY_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn cassette_edge_datums(name: &str, center_x: f64) -> Part {
    let mut datums = Part::empty(format!("cassette_quarantine_{name}_cassette_edge_datums"));
    for slot in 0..BAY_CAPACITY {
        let y = bay_slot_y(slot);
        let left = centered_cube(
            format!("cassette_quarantine_{name}_slot_{slot}_left_x_datum"),
            8.0,
            CASSETTE_Y + 16.0,
            18.0,
        )
        .translate(center_x - CASSETTE_X / 2.0 - 8.0, y, DECK_Z + 18.0 + 9.0);
        let rear = centered_cube(
            format!("cassette_quarantine_{name}_slot_{slot}_rear_y_datum"),
            CASSETTE_X + 20.0,
            8.0,
            18.0,
        )
        .translate(center_x, y + CASSETTE_Y / 2.0 + 8.0, DECK_Z + 18.0 + 9.0);
        let datum_pin_left = centered_cylinder(
            format!("cassette_quarantine_{name}_slot_{slot}_left_datum_pin"),
            4.0,
            12.0,
            24,
        )
        .translate(
            center_x - CASSETTE_X / 2.0 + 30.0,
            y + CASSETTE_Y / 2.0 - 30.0,
            DECK_Z + 30.0,
        );
        let datum_pin_right = centered_cylinder(
            format!("cassette_quarantine_{name}_slot_{slot}_right_datum_pin"),
            4.0,
            12.0,
            24,
        )
        .translate(
            center_x + CASSETTE_X / 2.0 - 30.0,
            y + CASSETTE_Y / 2.0 - 30.0,
            DECK_Z + 30.0,
        );
        datums = datums + left + rear + datum_pin_left + datum_pin_right;
    }
    datums
}

fn slot_number_tabs(name: &str, center_x: f64) -> Part {
    let mut tabs = Part::empty(format!("cassette_quarantine_{name}_slot_number_tabs"));
    for slot in 0..BAY_CAPACITY {
        tabs = tabs
            + centered_cube(
                format!("cassette_quarantine_{name}_slot_{slot}_number_land"),
                48.0,
                6.0,
                26.0,
            )
            .translate(
                center_x + CASSETTE_X / 2.0 - 42.0,
                bay_slot_y(slot) - CASSETTE_Y / 2.0 - 18.0,
                DECK_Z + 42.0,
            );
    }
    tabs
}

fn inter_bay_hard_dividers() -> Part {
    let left_divider = centered_cube(
        "cassette_quarantine_released_to_hold_hard_divider",
        22.0,
        BAY_Y + 76.0,
        118.0,
    )
    .translate(
        (released_bay_center_x() + hold_bay_center_x()) / 2.0,
        BAY_CENTER_Y,
        DECK_Z + 59.0,
    );
    let right_divider = centered_cube(
        "cassette_quarantine_hold_to_reject_hard_divider",
        22.0,
        BAY_Y + 76.0,
        142.0,
    )
    .translate(
        (hold_bay_center_x() + reject_bay_center_x()) / 2.0,
        BAY_CENTER_Y,
        DECK_Z + 71.0,
    );
    let reject_back_plate = centered_cube(
        "cassette_quarantine_reject_red_status_back_plate",
        BAY_X - 26.0,
        16.0,
        96.0,
    )
    .translate(
        reject_bay_center_x(),
        BAY_CENTER_Y + BAY_Y / 2.0 + 18.0,
        DECK_Z + 66.0,
    );

    left_divider + right_divider + reject_back_plate
}

fn identity_scan_lands() -> Part {
    let scan_bar = centered_cube(
        "cassette_quarantine_identity_scan_front_bar",
        SCAN_BAR_X,
        SCAN_BAR_Y,
        SCAN_BAR_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 118.0, DECK_Z + SCAN_BAR_Z / 2.0);
    let mut lands = Part::empty("cassette_quarantine_identity_scan_lands");
    for i in 0..SCAN_LAND_COUNT {
        let x = scan_land_x(i);
        let is_rfid = i % 3 == 2;
        let land = if is_rfid {
            centered_cube(
                format!("cassette_quarantine_rfid_antenna_land_{i}"),
                RFID_LAND_X,
                RFID_LAND_Y,
                5.0,
            )
        } else {
            centered_cube(
                format!("cassette_quarantine_barcode_flat_land_{i}"),
                SCAN_LAND_X,
                SCAN_LAND_Y,
                5.0,
            )
        }
        .translate(x, -DECK_Y / 2.0 + 118.0, DECK_Z + SCAN_BAR_Z + 2.5);

        let fiducial = centered_cylinder(
            format!("cassette_quarantine_scan_fiducial_dot_{i}"),
            3.0,
            4.0,
            18,
        )
        .translate(x + 28.0, -DECK_Y / 2.0 + 82.0, DECK_Z + SCAN_BAR_Z + 4.0);

        lands = lands + land + fiducial;
    }

    let scanner_bridge = centered_cube(
        "cassette_quarantine_handsfree_scanner_bridge_clearance",
        SCAN_BAR_X + 80.0,
        22.0,
        116.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 176.0, DECK_Z + 82.0);
    let camera_body = centered_cube(
        "cassette_quarantine_barcode_camera_placeholder",
        142.0,
        60.0,
        48.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 148.0, DECK_Z + 142.0);
    let lens = centered_cylinder("cassette_quarantine_scan_lens_axis", 18.0, 28.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -DECK_Y / 2.0 + 112.0, DECK_Z + 142.0);

    scan_bar + lands + scanner_bridge + camera_body + lens
}

fn deviation_tag_panel() -> Part {
    let panel = centered_cube(
        "cassette_quarantine_visible_deviation_tag_panel",
        TAG_PANEL_X,
        TAG_PANEL_Y,
        TAG_PANEL_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 78.0, DECK_Z + 136.0);
    let top_cap = centered_cube(
        "cassette_quarantine_tag_panel_top_wipe_cap",
        TAG_PANEL_X + 36.0,
        TAG_PANEL_Y + 18.0,
        18.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 78.0, DECK_Z + 250.0);

    let mut cards = Part::empty("cassette_quarantine_deviation_tag_card_slots");
    for i in 0..TAG_CARD_COUNT {
        let x = tag_card_x(i);
        let pocket = centered_cube(
            format!("cassette_quarantine_deviation_tag_card_pocket_{i}"),
            TAG_CARD_X,
            TAG_PANEL_Y + 6.0,
            TAG_CARD_Z,
        )
        .translate(x, DECK_Y / 2.0 - 92.0, DECK_Z + 132.0);
        let label_strip = centered_cube(
            format!("cassette_quarantine_deviation_tag_label_strip_{i}"),
            TAG_CARD_X - 18.0,
            6.0,
            12.0,
        )
        .translate(x, DECK_Y / 2.0 - 110.0, DECK_Z + 180.0);
        cards = cards + pocket + label_strip;
    }

    let qa_lockout_rail = centered_cube(
        "cassette_quarantine_qa_lockout_badge_rail",
        TAG_PANEL_X - 80.0,
        16.0,
        22.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 116.0, DECK_Z + 48.0);

    panel + top_cap + cards + qa_lockout_rail
}

fn environmental_logger_pocket() -> Part {
    let body = centered_cube(
        "cassette_quarantine_environmental_logger_pocket_body",
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        LOGGER_POCKET_Z,
    )
    .translate(
        DECK_X / 2.0 - 196.0,
        DECK_Y / 2.0 - 206.0,
        DECK_Z + LOGGER_POCKET_Z / 2.0,
    );
    let cavity = centered_cube(
        "cassette_quarantine_environmental_logger_device_cavity",
        LOGGER_POCKET_X - 42.0,
        LOGGER_POCKET_Y - 34.0,
        LOGGER_POCKET_Z + 2.0,
    )
    .translate(
        DECK_X / 2.0 - 196.0,
        DECK_Y / 2.0 - 206.0,
        DECK_Z + LOGGER_POCKET_Z / 2.0 + 8.0,
    );
    let cable_exit = centered_cube(
        "cassette_quarantine_logger_cable_exit_slot",
        28.0,
        LOGGER_POCKET_Y + 8.0,
        16.0,
    )
    .translate(
        DECK_X / 2.0 - 88.0,
        DECK_Y / 2.0 - 206.0,
        DECK_Z + LOGGER_POCKET_Z - 8.0,
    );

    let mut slots = Part::empty("cassette_quarantine_logger_retention_slots");
    for i in 0..LOGGER_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_logger_clip_slot_{i}"),
                14.0,
                76.0,
                8.0,
            )
            .translate(
                DECK_X / 2.0 - 268.0 + i as f64 * 46.0,
                DECK_Y / 2.0 - 206.0,
                DECK_Z + LOGGER_POCKET_Z + 4.0,
            );
    }

    let mut ports = Part::empty("cassette_quarantine_logger_sensor_reference_ports");
    for i in 0..LOGGER_SENSOR_PORTS {
        ports = ports
            + centered_cylinder(
                format!("cassette_quarantine_logger_sensor_port_{i}"),
                5.0,
                20.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                DECK_X / 2.0 - 286.0 + i as f64 * 44.0,
                DECK_Y / 2.0 - 264.0,
                DECK_Z + 74.0,
            );
    }

    body - cavity - cable_exit + slots + ports
}

fn sealed_transfer_tote_interface() -> Part {
    let dock = centered_cube(
        "cassette_quarantine_sealed_transfer_tote_dock_plate",
        TOTE_DOCK_X,
        TOTE_DOCK_Y,
        TOTE_DOCK_Z,
    )
    .translate(
        -DECK_X / 2.0 + TOTE_DOCK_X / 2.0 + 54.0,
        -DECK_Y / 2.0 + TOTE_DOCK_Y / 2.0 + 42.0,
        DECK_Z + TOTE_DOCK_Z / 2.0,
    );
    let opening = centered_cube(
        "cassette_quarantine_tote_payload_window_cut",
        TOTE_DOCK_X - 96.0,
        TOTE_DOCK_Y + 6.0,
        TOTE_DOCK_Z - 34.0,
    )
    .translate(
        -DECK_X / 2.0 + TOTE_DOCK_X / 2.0 + 54.0,
        -DECK_Y / 2.0 + TOTE_DOCK_Y / 2.0 + 42.0,
        DECK_Z + TOTE_DOCK_Z / 2.0 + 6.0,
    );
    let gasket = gasket_frame_xz(
        "cassette_quarantine_tote_gasket_frame",
        TOTE_GASKET_X,
        TOTE_GASKET_Y,
        TOTE_GASKET_Z,
        18.0,
    )
    .translate(
        -DECK_X / 2.0 + TOTE_DOCK_X / 2.0 + 54.0,
        -DECK_Y / 2.0 + TOTE_DOCK_Y + 50.0,
        DECK_Z + 62.0,
    );

    let mut latches = Part::empty("cassette_quarantine_tote_latch_clamps");
    for i in 0..TOTE_LATCH_COUNT {
        let sx = if i % 2 == 0 { -1.0 } else { 1.0 };
        let sz = if i < 2 { -1.0 } else { 1.0 };
        latches = latches
            + centered_cube(
                format!("cassette_quarantine_tote_cam_latch_{i}"),
                54.0,
                32.0,
                30.0,
            )
            .translate(
                -DECK_X / 2.0 + TOTE_DOCK_X / 2.0 + 54.0 + sx * (TOTE_DOCK_X / 2.0 - 44.0),
                -DECK_Y / 2.0 + TOTE_DOCK_Y + 50.0,
                DECK_Z + 62.0 + sz * 48.0,
            );
    }

    let barcode_gate = centered_cube(
        "cassette_quarantine_tote_identity_interlock_scan_land",
        126.0,
        8.0,
        44.0,
    )
    .translate(
        -DECK_X / 2.0 + TOTE_DOCK_X + 4.0,
        -DECK_Y / 2.0 + TOTE_DOCK_Y + 58.0,
        DECK_Z + 130.0,
    );

    dock - opening + gasket + latches + barcode_gate
}

fn contamination_suspect_isolation_cover() -> Part {
    let cover_outer = centered_cube(
        "cassette_quarantine_contamination_suspect_cover_outer_envelope",
        COVER_X,
        COVER_Y,
        COVER_Z,
    )
    .translate(0.0, BAY_CENTER_Y, DECK_Z + COVER_Z / 2.0 + 34.0);
    let cover_inner = centered_cube(
        "cassette_quarantine_contamination_suspect_cover_inner_clearance",
        COVER_X - 2.0 * COVER_WALL,
        COVER_Y - 2.0 * COVER_WALL,
        COVER_Z - COVER_WALL,
    )
    .translate(0.0, BAY_CENTER_Y, DECK_Z + COVER_Z / 2.0 + 26.0);
    let front_view_cut = centered_cube(
        "cassette_quarantine_isolation_cover_front_view_cut",
        COVER_X - 160.0,
        COVER_WALL + 4.0,
        COVER_Z - 84.0,
    )
    .translate(0.0, BAY_CENTER_Y - COVER_Y / 2.0, DECK_Z + 152.0);
    let front_window_frame = gasket_frame_xz(
        "cassette_quarantine_isolation_cover_front_window_frame",
        COVER_X - 116.0,
        8.0,
        COVER_Z - 46.0,
        18.0,
    )
    .translate(0.0, BAY_CENTER_Y - COVER_Y / 2.0 - 8.0, DECK_Z + 152.0);
    let handle = centered_cube(
        "cassette_quarantine_isolation_cover_lift_handle",
        420.0,
        32.0,
        34.0,
    )
    .translate(0.0, BAY_CENTER_Y, DECK_Z + COVER_Z + 82.0);
    let seal_land = gasket_frame_xy(
        "cassette_quarantine_isolation_cover_lower_seal_land",
        COVER_X + 34.0,
        COVER_Y + 34.0,
        10.0,
        16.0,
    )
    .translate(0.0, BAY_CENTER_Y, DECK_Z + 37.0);

    cover_outer - cover_inner - front_view_cut
        + front_window_frame
        + handle
        + seal_land
        + cover_filter_lands()
}

fn cover_filter_lands() -> Part {
    let mut filters = Part::empty("cassette_quarantine_cover_filter_lands");
    for i in 0..COVER_FILTER_COUNT {
        filters = filters
            + centered_cube(
                format!("cassette_quarantine_cover_hydrophobic_vent_filter_land_{i}"),
                72.0,
                8.0,
                40.0,
            )
            .translate(
                -270.0 + i as f64 * 180.0,
                BAY_CENTER_Y + COVER_Y / 2.0 + 6.0,
                DECK_Z + COVER_Z - 18.0,
            );
    }
    filters
}

fn sample_archive_handoff_positions() -> Part {
    let rack = centered_cube(
        "cassette_quarantine_sample_archive_handoff_rack",
        SAMPLE_RACK_X,
        SAMPLE_RACK_Y,
        SAMPLE_RACK_Z,
    )
    .translate(
        DECK_X / 2.0 - SAMPLE_RACK_X / 2.0 - 64.0,
        -DECK_Y / 2.0 + 238.0,
        DECK_Z + SAMPLE_RACK_Z / 2.0,
    );

    let mut cuts = Part::empty("cassette_quarantine_sample_archive_handoff_cuts");
    let mut lands = Part::empty("cassette_quarantine_sample_archive_handoff_lands");
    for i in 0..SAMPLE_ARCHIVE_COUNT {
        let x = sample_archive_x(i);
        let y_sample = -DECK_Y / 2.0 + 210.0;
        let y_archive = -DECK_Y / 2.0 + 268.0;
        cuts = cuts
            + centered_cylinder(
                format!("cassette_quarantine_sample_tube_socket_cut_{i}"),
                SAMPLE_TUBE_D / 2.0,
                SAMPLE_RACK_Z + 2.0,
                28,
            )
            .translate(x, y_sample, DECK_Z + SAMPLE_RACK_Z / 2.0)
            + centered_cylinder(
                format!("cassette_quarantine_archive_vial_socket_cut_{i}"),
                SAMPLE_VIAL_D / 2.0,
                SAMPLE_RACK_Z + 2.0,
                24,
            )
            .translate(x, y_archive, DECK_Z + SAMPLE_RACK_Z / 2.0);
        lands = lands
            + centered_cube(
                format!("cassette_quarantine_sample_archive_pair_label_land_{i}"),
                42.0,
                5.0,
                20.0,
            )
            .translate(x, -DECK_Y / 2.0 + 302.0, DECK_Z + 52.0);
    }

    let covered_lid = centered_cube(
        "cassette_quarantine_archive_rack_hinged_lid_placeholder",
        SAMPLE_RACK_X + 24.0,
        16.0,
        46.0,
    )
    .translate(
        DECK_X / 2.0 - SAMPLE_RACK_X / 2.0 - 64.0,
        -DECK_Y / 2.0 + 326.0,
        DECK_Z + 62.0,
    );

    rack - cuts + lands + covered_lid
}

fn flow_separation_barrier() -> Part {
    let barrier = centered_cube(
        "cassette_quarantine_normal_incubator_flow_hard_barrier",
        FLOW_BARRIER_X,
        FLOW_BARRIER_Y,
        FLOW_BARRIER_Z,
    )
    .translate(0.0, barrier_center_y(), DECK_Z + FLOW_BARRIER_Z / 2.0);
    let redirection_arrow_land = centered_cube(
        "cassette_quarantine_not_normal_flow_label_land",
        420.0,
        8.0,
        36.0,
    )
    .translate(
        -DECK_X / 2.0 + 310.0,
        barrier_center_y() - FLOW_BARRIER_Y / 2.0 - 8.0,
        DECK_Z + 88.0,
    );
    let pass_gate_blocker = centered_cube(
        "cassette_quarantine_normal_flow_pass_gate_blocker",
        188.0,
        72.0,
        96.0,
    )
    .translate(
        DECK_X / 2.0 - 156.0,
        barrier_center_y() + 28.0,
        DECK_Z + 62.0,
    );
    let side_gap_gauge_left = centered_cube(
        "cassette_quarantine_left_side_wall_gap_gauge",
        SIDE_WALL_GAP,
        NORMAL_FLOW_KEEP_AWAY_Y,
        20.0,
    )
    .translate(
        -DECK_X / 2.0 + SIDE_WALL_GAP / 2.0,
        barrier_center_y() + NORMAL_FLOW_KEEP_AWAY_Y / 2.0 + 28.0,
        DECK_Z + 10.0,
    );
    let side_gap_gauge_right = centered_cube(
        "cassette_quarantine_right_side_wall_gap_gauge",
        SIDE_WALL_GAP,
        NORMAL_FLOW_KEEP_AWAY_Y,
        20.0,
    )
    .translate(
        DECK_X / 2.0 - SIDE_WALL_GAP / 2.0,
        barrier_center_y() + NORMAL_FLOW_KEEP_AWAY_Y / 2.0 + 28.0,
        DECK_Z + 10.0,
    );

    barrier
        + redirection_arrow_land
        + pass_gate_blocker
        + side_gap_gauge_left
        + side_gap_gauge_right
}

fn robot_service_keepouts() -> Part {
    let robot_volume = centered_cube(
        "cassette_quarantine_robot_overhead_service_keepout",
        ROBOT_CLEARANCE_X,
        ROBOT_CLEARANCE_Y,
        ROBOT_CLEARANCE_Z,
    )
    .translate(0.0, 0.0, DECK_Z + ROBOT_CLEARANCE_Z / 2.0 + 20.0);
    let front_operator = centered_cube(
        "cassette_quarantine_front_operator_handoff_keepout",
        DECK_X,
        FRONT_OPERATOR_CLEARANCE_Y,
        42.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_OPERATOR_CLEARANCE_Y / 2.0,
        DECK_Z + 21.0,
    );
    let rear_service = centered_cube(
        "cassette_quarantine_rear_quality_service_keepout",
        DECK_X,
        REAR_SERVICE_CLEARANCE_Y,
        72.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE_Y / 2.0,
        DECK_Z + 36.0,
    );
    let left_service = centered_cube(
        "cassette_quarantine_left_tote_cart_keepout",
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y,
        56.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE_X / 2.0,
        0.0,
        DECK_Z + 28.0,
    );
    let right_service = centered_cube(
        "cassette_quarantine_right_archive_service_keepout",
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y,
        56.0,
    )
    .translate(
        DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE_X / 2.0,
        0.0,
        DECK_Z + 28.0,
    );

    robot_volume + front_operator + rear_service + left_service + right_service
}

fn gasket_frame_xz(name: impl Into<String>, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - 2.0 * wall,
        y + 2.0,
        z - 2.0 * wall,
    );
    outer - inner
}

fn gasket_frame_xy(name: impl Into<String>, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

fn bay_slot_y(slot: usize) -> f64 {
    assert!(slot < BAY_CAPACITY);
    let pitch = CASSETTE_Y + 42.0;
    BAY_CENTER_Y + (slot as f64 - 0.5) * pitch
}

fn released_bay_center_x() -> f64 {
    BAY_START_X
}

fn hold_bay_center_x() -> f64 {
    BAY_START_X + BAY_PITCH_X
}

fn reject_bay_center_x() -> f64 {
    BAY_START_X + 2.0 * BAY_PITCH_X
}

fn bay_span_x() -> f64 {
    2.0 * BAY_PITCH_X + BAY_X
}

fn scan_land_x(i: usize) -> f64 {
    assert!(i < SCAN_LAND_COUNT);
    let usable = SCAN_BAR_X - 110.0;
    -usable / 2.0 + i as f64 * usable / (SCAN_LAND_COUNT as f64 - 1.0)
}

fn tag_card_x(i: usize) -> f64 {
    assert!(i < TAG_CARD_COUNT);
    let pitch = (TAG_PANEL_X - 160.0) / (TAG_CARD_COUNT as f64 - 1.0);
    -((TAG_PANEL_X - 160.0) / 2.0) + i as f64 * pitch
}

fn sample_archive_x(i: usize) -> f64 {
    assert!(i < SAMPLE_ARCHIVE_COUNT);
    let pitch = 56.0;
    DECK_X / 2.0 - SAMPLE_RACK_X + 40.0 + i as f64 * pitch
}

fn cover_clearance_z() -> f64 {
    COVER_Z - COVER_WALL
}

fn quarantine_zone_rear_edge() -> f64 {
    BAY_CENTER_Y + COVER_Y / 2.0
}

fn barrier_center_y() -> f64 {
    DECK_Y / 2.0 - NORMAL_FLOW_KEEP_AWAY_Y - FLOW_BARRIER_Y / 2.0
}

fn barrier_front_edge() -> f64 {
    barrier_center_y() - FLOW_BARRIER_Y / 2.0
}

fn normal_incubator_flow_lane_width() -> f64 {
    DECK_Y / 2.0 - (barrier_center_y() + FLOW_BARRIER_Y / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_has_named_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS
            .last()
            .unwrap()
            .ends_with("cassette_deviation_quarantine_station_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with("output/cassette_deviation_quarantine_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn status_bays_hold_expected_cassette_count() {
        assert_eq!(BAY_COUNT, 3);
        assert_eq!(BAY_CAPACITY, 2);
        assert_eq!(TOTAL_CASSETTE_POSITIONS, 6);
        assert_eq!(CHIP_COUNT, 20);
        assert!(BAY_X > CASSETTE_X + 36.0);
        assert!(BAY_Y > 2.0 * CASSETTE_Y + 70.0);
    }

    #[test]
    fn deck_and_cover_dimensions_leave_robot_clearance() {
        assert_eq!(DECK_X, 2280.0);
        assert_eq!(DECK_Y, 1900.0);
        assert!(bay_span_x() < DECK_X - 160.0);
        assert!(cover_clearance_z() >= CASSETTE_Z + COVER_Z_CLEARANCE);
        assert!(ROBOT_CLEARANCE_Z > COVER_Z + 90.0);
        assert!(ROBOT_CLEARANCE_X > DECK_X);
        assert!(ROBOT_CLEARANCE_Y > DECK_Y);
    }

    #[test]
    fn hold_and_reject_are_physically_segregated() {
        assert!(hold_bay_center_x() > released_bay_center_x());
        assert!(reject_bay_center_x() > hold_bay_center_x());
        assert!(hold_bay_center_x() - released_bay_center_x() >= BAY_X + 48.0);
        assert!(reject_bay_center_x() - hold_bay_center_x() >= BAY_X + 48.0);
        assert!(BAY_WALL >= 12.0);
    }

    #[test]
    fn traceability_and_sample_counts_match_workflow() {
        assert_eq!(SCAN_LAND_COUNT, 10);
        assert_eq!(TAG_CARD_COUNT, 6);
        assert_eq!(LOGGER_SLOT_COUNT, 4);
        assert_eq!(LOGGER_SENSOR_PORTS, 5);
        assert_eq!(SAMPLE_ARCHIVE_COUNT, 8);
        assert!(SCAN_LAND_COUNT >= TOTAL_CASSETTE_POSITIONS + TOTE_LATCH_COUNT);
    }

    #[test]
    fn flow_barrier_keeps_quarantine_out_of_normal_incubator_flow() {
        assert!(quarantine_zone_rear_edge() <= barrier_front_edge() - 16.0);
        assert!(normal_incubator_flow_lane_width() >= NORMAL_FLOW_KEEP_AWAY_Y);
        assert!(FLOW_BARRIER_X > DECK_X - 140.0);
        assert!(FLOW_BARRIER_Z >= 120.0);
    }

    #[test]
    fn tote_and_logger_interfaces_have_practical_service_space() {
        assert!(TOTE_GASKET_X < TOTE_DOCK_X);
        assert!(TOTE_GASKET_Z > TOTE_DOCK_Z);
        assert_eq!(TOTE_LATCH_COUNT, 4);
        assert!(LOGGER_POCKET_X > 200.0);
        assert!(LOGGER_POCKET_Y > 100.0);
        assert!(FRONT_OPERATOR_CLEARANCE_Y >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE_Y >= 240.0);
    }
}
