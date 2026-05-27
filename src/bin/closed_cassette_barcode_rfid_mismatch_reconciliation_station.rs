use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette barcode/RFID mismatch reconciliation station.
//
// Intent:
// - Present a 16-position closed cassette tray to barcode camera and RFID checks
//   without opening the cassette path.
// - Physically separate release, hold, and reject decisions while preserving
//   scan/certificate evidence lands for review.
// - Model mechanical datums, camera/RFID envelopes, service indicators,
//   reject/hold pockets, and evidence bridge geometry only; identity matching,
//   Part 11 records, and instrument qualification remain outside this CAD file.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_cleanable_deck.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_sixteen_slot_cassette_tray.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_barcode_camera_window.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_rfid_antenna_zone.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_reject_hold_pocket.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_alignment_fiducials.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_service_indicator_blocks.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_barcode_certificate_lands.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_evidence_bridge.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_robot_service_keepouts.stl",
    "output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 10] = [
    "sixteen_slot_cassette_tray",
    "barcode_camera_window",
    "rfid_antenna_zone",
    "reject_hold_pocket",
    "alignment_fiducials",
    "service_indicator_blocks",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const SLOT_COLS: usize = 4;
const SLOT_ROWS: usize = 4;
const SLOT_COUNT: usize = SLOT_COLS * SLOT_ROWS;
const CERTIFICATE_LAND_COUNT: usize = 4;
const LANE_COUNT: usize = 3;
const LANE_SLOTS_PER_LANE: usize = 4;
const SERVICE_INDICATOR_COUNT: usize = 5;
const FIDUCIAL_COUNT: usize = 12;
const EVIDENCE_CAMERA_COUNT: usize = 3;

const DECK_X: f64 = 1400.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 20.0;
const DECK_RIM_W: f64 = 16.0;
const DECK_RIM_Z: f64 = 28.0;
const WIPE_GUTTER_W: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const TRAY_CENTER: (f64, f64) = (-318.0, 104.0);
const SLOT_X: f64 = REVC_CHIP_LENGTH + 12.0;
const SLOT_Y: f64 = REVC_CHIP_WIDTH + 12.0;
const SLOT_DEPTH: f64 = 13.0;
const SLOT_PITCH_X: f64 = SLOT_X + 18.0;
const SLOT_PITCH_Y: f64 = SLOT_Y + 16.0;
const TRAY_MARGIN: f64 = 28.0;
const TRAY_X: f64 = (SLOT_COLS as f64 - 1.0) * SLOT_PITCH_X + SLOT_X + 2.0 * TRAY_MARGIN;
const TRAY_Y: f64 = (SLOT_ROWS as f64 - 1.0) * SLOT_PITCH_Y + SLOT_Y + 2.0 * TRAY_MARGIN;
const TRAY_Z: f64 = 38.0;
const TRAY_RAIL_W: f64 = 12.0;
const TRAY_RAIL_Z: f64 = 24.0;
const SLOT_LABEL_LAND_X: f64 = 78.0;
const SLOT_LABEL_LAND_Y: f64 = 18.0;

const BARCODE_WINDOW_CENTER: (f64, f64) = (-318.0, -244.0);
const BARCODE_WINDOW_X: f64 = 360.0;
const BARCODE_WINDOW_Y: f64 = 118.0;
const BARCODE_WINDOW_Z: f64 = 18.0;
const CAMERA_APERTURE_X: f64 = 254.0;
const CAMERA_APERTURE_Y: f64 = 54.0;
const CAMERA_DATUM_PINS: usize = 4;

const RFID_CENTER: (f64, f64) = (352.0, 232.0);
const RFID_PANEL_X: f64 = 342.0;
const RFID_PANEL_Y: f64 = 262.0;
const RFID_PANEL_Z: f64 = 16.0;
const RFID_COIL_OUTER_X: f64 = 252.0;
const RFID_COIL_OUTER_Y: f64 = 172.0;
const RFID_COIL_TRACE_W: f64 = 9.0;
const RFID_REFERENCE_TAGS: usize = 4;

const REJECT_HOLD_CENTER: (f64, f64) = (382.0, -288.0);
const REJECT_HOLD_X: f64 = 352.0;
const REJECT_HOLD_Y: f64 = 168.0;
const REJECT_HOLD_Z: f64 = 50.0;
const REJECT_HOLD_POCKET_X: f64 = 134.0;
const REJECT_HOLD_POCKET_Y: f64 = 104.0;
const REJECT_HOLD_WALL: f64 = 12.0;
const REJECT_HIGH_WALL_Z: f64 = 64.0;

const BARCODE_CERT_CENTER: (f64, f64) = (-318.0, -344.0);
const BARCODE_CERT_PANEL_X: f64 = 612.0;
const BARCODE_CERT_PANEL_Y: f64 = 112.0;
const BARCODE_CERT_PANEL_Z: f64 = 10.0;
const BARCODE_LAND_X: f64 = 116.0;
const BARCODE_LAND_Y: f64 = 18.0;
const CERTIFICATE_LAND_X: f64 = 126.0;
const CERTIFICATE_LAND_Y: f64 = 38.0;

const LANE_CENTER: (f64, f64) = (360.0, -44.0);
const LANE_PANEL_X: f64 = 432.0;
const LANE_PANEL_Y: f64 = 248.0;
const LANE_PANEL_Z: f64 = 28.0;
const LANE_SLOT_X: f64 = 104.0;
const LANE_SLOT_Y: f64 = 34.0;
const LANE_SLOT_PITCH_X: f64 = 132.0;
const LANE_SLOT_PITCH_Y: f64 = 84.0;
const LANE_DIVIDER_W: f64 = 10.0;
const RELEASE_HOLD_REJECT_GAP_MIN: f64 = 46.0;

const SERVICE_CENTER: (f64, f64) = (306.0, 396.0);
const SERVICE_BLOCK_X: f64 = 72.0;
const SERVICE_BLOCK_Y: f64 = 42.0;
const SERVICE_BLOCK_Z: f64 = 30.0;
const SERVICE_BLOCK_PITCH_X: f64 = 88.0;

const BRIDGE_CENTER: (f64, f64) = (-30.0, 128.0);
const BRIDGE_SPAN_X: f64 = 1320.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 52.0;
const BRIDGE_BEAM_Y: f64 = 72.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_UNDERSIDE_Z: f64 = 176.0;
const BRIDGE_POST_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const CAMERA_POD_X: f64 = 94.0;
const CAMERA_POD_Y: f64 = 62.0;
const CAMERA_POD_Z: f64 = 48.0;
const CAMERA_PITCH_X: f64 = 204.0;
const LED_SEGMENTS: usize = 8;
const EVIDENCE_CARD_RAIL_X: f64 = 520.0;
const EVIDENCE_CARD_RAIL_Y: f64 = 26.0;

const KEEP_OUT_GAUGE_Z: f64 = 8.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 108.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 84.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 122.0;
const LEFT_TRAY_LOAD_KEEP_OUT_X: f64 = 86.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_deck();
    export(OUTPUTS[0], &deck);

    let tray = sixteen_slot_cassette_tray();
    export(OUTPUTS[1], &tray);

    let barcode_window = barcode_camera_window();
    export(OUTPUTS[2], &barcode_window);

    let rfid = rfid_antenna_zone();
    export(OUTPUTS[3], &rfid);

    let reject_hold = reject_hold_pocket();
    export(OUTPUTS[4], &reject_hold);

    let fiducials = alignment_fiducials();
    export(OUTPUTS[5], &fiducials);

    let indicators = service_indicator_blocks();
    export(OUTPUTS[6], &indicators);

    let barcode_lands = barcode_certificate_lands();
    export(OUTPUTS[7], &barcode_lands);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette barcode/RFID mismatch reconciliation station:");
    println!("  Cleanable deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Cassette tray:               {SLOT_ROWS} x {SLOT_COLS} closed cassette slots ({SLOT_COUNT} total), slot relief {:.1}mm x {:.1}mm x {:.1}mm",
        SLOT_X, SLOT_Y, SLOT_DEPTH
    );
    println!(
        "  Barcode camera window:       {:.0}mm x {:.0}mm aperture frame with {CAMERA_DATUM_PINS} camera datum pins",
        CAMERA_APERTURE_X, CAMERA_APERTURE_Y
    );
    println!(
        "  RFID antenna zone:           {:.0}mm x {:.0}mm panel, {:.0}mm x {:.0}mm coil envelope, {RFID_REFERENCE_TAGS} reference tag lands",
        RFID_PANEL_X, RFID_PANEL_Y, RFID_COIL_OUTER_X, RFID_COIL_OUTER_Y
    );
    println!(
        "  Decision handling:           release/hold/reject lanes, {LANE_SLOTS_PER_LANE} slots per lane, reject/hold pocket gap {:.0}mm",
        release_hold_reject_gap()
    );
    println!(
        "  Traceability lands:          {SLOT_COUNT} barcode lands plus {CERTIFICATE_LAND_COUNT} certificate lands and {FIDUCIAL_COUNT} alignment fiducials"
    );
    println!(
        "  Evidence bridge:             {EVIDENCE_CAMERA_COUNT} camera pods, {LED_SEGMENTS} LED segments, underside {:.0}mm above deck top",
        evidence_bridge_clearance_above_deck()
    );
    println!("  Labeled STL outputs:         {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    cleanable_deck()
        + sixteen_slot_cassette_tray()
        + barcode_camera_window()
        + rfid_antenna_zone()
        + reject_hold_pocket()
        + alignment_fiducials()
        + service_indicator_blocks()
        + barcode_certificate_lands()
        + release_hold_reject_lanes()
        + evidence_bridge()
        + robot_service_keepouts()
}

fn cleanable_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_reconciliation_cleanable_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - module_recesses() - wipe_gutters()
        + deck_perimeter_lips()
        + rear_cable_label_strip()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_cassette_reconciliation_deck_mount_holes");
    for (index, (x, y)) in deck_mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_reconciliation_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn module_recesses() -> Part {
    let mut recesses = Part::empty("closed_cassette_reconciliation_module_recesses");
    for (name, center, x, y) in module_specs() {
        recesses = recesses + deck_top_recess(name, center, x + 18.0, y + 18.0, 5.0);
    }
    recesses
}

fn deck_top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(
        format!("closed_cassette_reconciliation_{name}_deck_recess"),
        x,
        y,
        depth + 0.4,
    )
    .translate(center.0, center.1, DECK_Z - depth / 2.0 + 0.2)
}

fn wipe_gutters() -> Part {
    let cross = centered_cube(
        "closed_cassette_reconciliation_cross_wipe_gutter",
        DECK_X - 186.0,
        WIPE_GUTTER_W,
        6.0,
    )
    .translate(0.0, -194.0, DECK_Z - 2.4);
    let tray_front = centered_cube(
        "closed_cassette_reconciliation_tray_front_wipe_gutter",
        TRAY_X + 60.0,
        WIPE_GUTTER_W,
        6.0,
    )
    .translate(
        TRAY_CENTER.0,
        TRAY_CENTER.1 - TRAY_Y / 2.0 - 28.0,
        DECK_Z - 2.4,
    );
    let right_sump = centered_cube(
        "closed_cassette_reconciliation_right_decision_sump",
        18.0,
        DECK_Y - 210.0,
        7.0,
    )
    .translate(DECK_X / 2.0 - 58.0, -14.0, DECK_Z - 2.8);
    let drain = centered_cylinder(
        "closed_cassette_reconciliation_front_drain_port",
        5.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(486.0, -DECK_Y / 2.0 + 16.0, DECK_Z - 7.0);

    cross + tray_front + right_sump + drain
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "closed_cassette_reconciliation_rear_cleanable_lip",
        DECK_X - 90.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 28.0, DECK_Z + DECK_RIM_Z / 2.0);
    let left = centered_cube(
        "closed_cassette_reconciliation_left_tray_loading_lip",
        DECK_RIM_W,
        DECK_Y - 120.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 28.0, 0.0, DECK_Z + DECK_RIM_Z / 2.0);
    let right = centered_cube(
        "closed_cassette_reconciliation_right_service_lip",
        DECK_RIM_W,
        DECK_Y - 190.0,
        22.0,
    )
    .translate(DECK_X / 2.0 - 28.0, -18.0, DECK_Z + 11.0);
    let front = centered_cube(
        "closed_cassette_reconciliation_front_low_robot_lip",
        DECK_X - 334.0,
        10.0,
        14.0,
    )
    .translate(-84.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 7.0);

    rear + left + right + front
}

fn rear_cable_label_strip() -> Part {
    let strip = centered_cube(
        "closed_cassette_reconciliation_rear_barcode_rfid_cable_label_strip",
        770.0,
        6.0,
        24.0,
    )
    .translate(92.0, DECK_Y / 2.0 - 66.0, DECK_Z + 12.0);
    let mut glands = Part::empty("closed_cassette_reconciliation_rear_service_cable_glands");
    for index in 0..7 {
        glands = glands
            + centered_cylinder(
                format!("closed_cassette_reconciliation_rear_cable_gland_{index}"),
                6.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -226.0 + index as f64 * 92.0,
                DECK_Y / 2.0 - 66.0,
                DECK_Z + 12.0,
            );
    }
    strip - glands
}

fn sixteen_slot_cassette_tray() -> Part {
    let tray = centered_cube(
        "closed_cassette_reconciliation_sixteen_slot_tray_body",
        TRAY_X,
        TRAY_Y,
        TRAY_Z,
    );

    let mut cuts = Part::empty("closed_cassette_reconciliation_slot_relief_cuts");
    for index in 0..SLOT_COUNT {
        let (x, y) = slot_center(index);
        let pocket = centered_cube(
            format!("closed_cassette_reconciliation_cassette_slot_{index:02}_pocket"),
            SLOT_X,
            SLOT_Y,
            SLOT_DEPTH + 1.0,
        )
        .translate(x, y, TRAY_Z / 2.0 - SLOT_DEPTH / 2.0 + 0.6);
        let front_barcode_notch = centered_cube(
            format!("closed_cassette_reconciliation_cassette_slot_{index:02}_barcode_view_notch"),
            SLOT_X * 0.52,
            10.0,
            SLOT_DEPTH + 2.0,
        )
        .translate(
            x,
            y - SLOT_Y / 2.0 + 8.0,
            TRAY_Z / 2.0 - SLOT_DEPTH / 2.0 + 1.0,
        );
        cuts = cuts + pocket + front_barcode_notch;
    }

    let tray_with_slots = tray - cuts + tray_perimeter_rails() + tray_slot_index_lands();
    place_on_deck(tray_with_slots, TRAY_CENTER, TRAY_Z)
}

fn tray_perimeter_rails() -> Part {
    let rear = centered_cube(
        "closed_cassette_reconciliation_tray_rear_datum_rail",
        TRAY_X,
        TRAY_RAIL_W,
        TRAY_RAIL_Z,
    )
    .translate(
        0.0,
        TRAY_Y / 2.0 - TRAY_RAIL_W / 2.0,
        TRAY_Z / 2.0 + TRAY_RAIL_Z / 2.0,
    );
    let left = centered_cube(
        "closed_cassette_reconciliation_tray_left_datum_rail",
        TRAY_RAIL_W,
        TRAY_Y,
        TRAY_RAIL_Z,
    )
    .translate(
        -TRAY_X / 2.0 + TRAY_RAIL_W / 2.0,
        0.0,
        TRAY_Z / 2.0 + TRAY_RAIL_Z / 2.0,
    );
    let right_soft = centered_cube(
        "closed_cassette_reconciliation_tray_right_soft_capture_rail",
        TRAY_RAIL_W,
        TRAY_Y * 0.72,
        TRAY_RAIL_Z * 0.66,
    )
    .translate(
        TRAY_X / 2.0 - TRAY_RAIL_W / 2.0,
        -22.0,
        TRAY_Z / 2.0 + TRAY_RAIL_Z * 0.33,
    );
    let front_low = centered_cube(
        "closed_cassette_reconciliation_tray_front_low_robot_access_rail",
        TRAY_X - 120.0,
        TRAY_RAIL_W * 0.72,
        14.0,
    )
    .translate(20.0, -TRAY_Y / 2.0 + TRAY_RAIL_W / 2.0, TRAY_Z / 2.0 + 7.0);

    rear + left + right_soft + front_low
}

fn tray_slot_index_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_reconciliation_slot_index_lands");
    for index in 0..SLOT_COUNT {
        let (x, y) = slot_center(index);
        let land = centered_cube(
            format!("closed_cassette_reconciliation_slot_{index:02}_index_land"),
            SLOT_LABEL_LAND_X,
            SLOT_LABEL_LAND_Y,
            4.0,
        )
        .translate(x, y + SLOT_Y / 2.0 + 11.0, TRAY_Z / 2.0 + 2.0);
        let witness_dot = centered_cylinder(
            format!("closed_cassette_reconciliation_slot_{index:02}_orientation_witness_dot"),
            4.0,
            5.0,
            24,
        )
        .translate(
            x - SLOT_X / 2.0 + 16.0,
            y - SLOT_Y / 2.0 + 16.0,
            TRAY_Z / 2.0 + 2.5,
        );
        lands = lands + land + witness_dot;
    }
    lands
}

fn barcode_camera_window() -> Part {
    let frame = centered_cube(
        "closed_cassette_reconciliation_barcode_camera_window_frame",
        BARCODE_WINDOW_X,
        BARCODE_WINDOW_Y,
        BARCODE_WINDOW_Z,
    );
    let aperture = centered_cube(
        "closed_cassette_reconciliation_barcode_camera_clear_aperture",
        CAMERA_APERTURE_X,
        CAMERA_APERTURE_Y,
        BARCODE_WINDOW_Z + 2.0,
    );
    let glass_land = centered_cube(
        "closed_cassette_reconciliation_barcode_window_replaceable_glass_land",
        CAMERA_APERTURE_X + 30.0,
        CAMERA_APERTURE_Y + 26.0,
        4.0,
    )
    .translate(0.0, 0.0, BARCODE_WINDOW_Z / 2.0 + 2.0);
    let anti_glare_baffles = barcode_window_baffles();
    let camera_datums = barcode_window_camera_datums();

    place_on_deck(
        frame - aperture + glass_land + anti_glare_baffles + camera_datums,
        BARCODE_WINDOW_CENTER,
        BARCODE_WINDOW_Z,
    )
}

fn barcode_window_baffles() -> Part {
    let mut baffles =
        Part::empty("closed_cassette_reconciliation_barcode_window_anti_glare_baffles");
    for index in 0..5 {
        baffles = baffles
            + centered_cube(
                format!("closed_cassette_reconciliation_barcode_window_baffle_{index}"),
                5.0,
                CAMERA_APERTURE_Y + 24.0,
                18.0,
            )
            .translate(
                -CAMERA_APERTURE_X / 2.0 + 42.0 + index as f64 * 42.0,
                0.0,
                BARCODE_WINDOW_Z / 2.0 + 9.0,
            );
    }
    baffles
}

fn barcode_window_camera_datums() -> Part {
    let mut datums = Part::empty("closed_cassette_reconciliation_barcode_window_camera_datums");
    for (index, (x, y)) in [
        (
            -BARCODE_WINDOW_X / 2.0 + 32.0,
            -BARCODE_WINDOW_Y / 2.0 + 28.0,
        ),
        (
            BARCODE_WINDOW_X / 2.0 - 32.0,
            -BARCODE_WINDOW_Y / 2.0 + 28.0,
        ),
        (
            -BARCODE_WINDOW_X / 2.0 + 32.0,
            BARCODE_WINDOW_Y / 2.0 - 28.0,
        ),
        (BARCODE_WINDOW_X / 2.0 - 32.0, BARCODE_WINDOW_Y / 2.0 - 28.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_cassette_reconciliation_barcode_window_camera_datum_boss_{index}"),
            7.5,
            8.0,
            32,
        )
        .translate(x, y, BARCODE_WINDOW_Z / 2.0 + 4.0);
        let pilot = centered_cylinder(
            format!("closed_cassette_reconciliation_barcode_window_camera_datum_pilot_{index}"),
            2.2,
            9.0,
            24,
        )
        .translate(x, y, BARCODE_WINDOW_Z / 2.0 + 4.5);
        datums = datums + (boss - pilot);
    }
    datums
}

fn rfid_antenna_zone() -> Part {
    let panel = centered_cube(
        "closed_cassette_reconciliation_rfid_antenna_zone_panel",
        RFID_PANEL_X,
        RFID_PANEL_Y,
        RFID_PANEL_Z,
    );
    let coil = rectangular_frame(
        "closed_cassette_reconciliation_rfid_main_antenna_coil",
        RFID_COIL_OUTER_X,
        RFID_COIL_OUTER_Y,
        RFID_COIL_TRACE_W,
        5.0,
    )
    .translate(0.0, 4.0, RFID_PANEL_Z / 2.0 + 2.5);
    let guard = rectangular_frame(
        "closed_cassette_reconciliation_rfid_field_guard_ring",
        RFID_COIL_OUTER_X + 48.0,
        RFID_COIL_OUTER_Y + 42.0,
        5.0,
        5.0,
    )
    .translate(0.0, 4.0, RFID_PANEL_Z / 2.0 + 2.6);
    let tag_lands = rfid_reference_tag_lands();
    let cable_exit = centered_cube(
        "closed_cassette_reconciliation_rfid_shielded_cable_exit_land",
        84.0,
        26.0,
        18.0,
    )
    .translate(0.0, -RFID_PANEL_Y / 2.0 - 10.0, RFID_PANEL_Z / 2.0 + 9.0);
    let field_boundary = rfid_field_boundary_posts();

    place_on_deck(
        panel + coil + guard + tag_lands + cable_exit + field_boundary,
        RFID_CENTER,
        RFID_PANEL_Z,
    )
}

fn rfid_reference_tag_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_reconciliation_rfid_reference_tag_lands");
    for (index, (x, y)) in [
        (-104.0, -72.0),
        (104.0, -72.0),
        (-104.0, 82.0),
        (104.0, 82.0),
    ]
    .into_iter()
    .enumerate()
    {
        let land = centered_cube(
            format!("closed_cassette_reconciliation_rfid_reference_tag_land_{index}"),
            58.0,
            34.0,
            4.0,
        )
        .translate(x, y, RFID_PANEL_Z / 2.0 + 2.0);
        let dot = centered_cylinder(
            format!("closed_cassette_reconciliation_rfid_reference_tag_center_dot_{index}"),
            5.0,
            5.0,
            24,
        )
        .translate(x, y, RFID_PANEL_Z / 2.0 + 2.5);
        lands = lands + land + dot;
    }
    lands
}

fn rfid_field_boundary_posts() -> Part {
    let mut posts = Part::empty("closed_cassette_reconciliation_rfid_field_boundary_posts");
    for (index, (x, y)) in [
        (-RFID_PANEL_X / 2.0 + 18.0, -RFID_PANEL_Y / 2.0 + 18.0),
        (RFID_PANEL_X / 2.0 - 18.0, -RFID_PANEL_Y / 2.0 + 18.0),
        (-RFID_PANEL_X / 2.0 + 18.0, RFID_PANEL_Y / 2.0 - 18.0),
        (RFID_PANEL_X / 2.0 - 18.0, RFID_PANEL_Y / 2.0 - 18.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_cassette_reconciliation_rfid_field_boundary_post_{index}"),
                6.0,
                22.0,
                28,
            )
            .translate(x, y, RFID_PANEL_Z / 2.0 + 11.0);
    }
    posts
}

fn reject_hold_pocket() -> Part {
    let panel = centered_cube(
        "closed_cassette_reconciliation_reject_hold_pocket_panel",
        REJECT_HOLD_X,
        REJECT_HOLD_Y,
        REJECT_HOLD_Z,
    );
    let hold_cut = centered_cube(
        "closed_cassette_reconciliation_hold_pocket_recess",
        REJECT_HOLD_POCKET_X,
        REJECT_HOLD_POCKET_Y,
        22.0,
    )
    .translate(-82.0, 0.0, REJECT_HOLD_Z / 2.0 - 10.0);
    let reject_cut = centered_cube(
        "closed_cassette_reconciliation_reject_pocket_recess",
        REJECT_HOLD_POCKET_X,
        REJECT_HOLD_POCKET_Y,
        22.0,
    )
    .translate(82.0, 0.0, REJECT_HOLD_Z / 2.0 - 10.0);
    let divider = centered_cube(
        "closed_cassette_reconciliation_hold_reject_hard_divider",
        REJECT_HOLD_WALL,
        REJECT_HOLD_Y + 18.0,
        REJECT_HIGH_WALL_Z,
    )
    .translate(0.0, 0.0, REJECT_HOLD_Z / 2.0 + REJECT_HIGH_WALL_Z / 2.0);
    let reject_wall = centered_cube(
        "closed_cassette_reconciliation_reject_pocket_high_backstop",
        REJECT_HOLD_POCKET_X + 38.0,
        REJECT_HOLD_WALL,
        REJECT_HIGH_WALL_Z,
    )
    .translate(
        82.0,
        REJECT_HOLD_Y / 2.0 - REJECT_HOLD_WALL / 2.0,
        REJECT_HOLD_Z / 2.0 + REJECT_HIGH_WALL_Z / 2.0,
    );
    let hold_status_land = centered_cube(
        "closed_cassette_reconciliation_hold_review_status_land",
        116.0,
        24.0,
        5.0,
    )
    .translate(
        -82.0,
        -REJECT_HOLD_Y / 2.0 + 24.0,
        REJECT_HOLD_Z / 2.0 + 2.5,
    );
    let reject_status_land = centered_cube(
        "closed_cassette_reconciliation_reject_review_status_land",
        116.0,
        24.0,
        5.0,
    )
    .translate(82.0, -REJECT_HOLD_Y / 2.0 + 24.0, REJECT_HOLD_Z / 2.0 + 2.5);

    place_on_deck(
        panel - hold_cut - reject_cut
            + divider
            + reject_wall
            + hold_status_land
            + reject_status_land,
        REJECT_HOLD_CENTER,
        REJECT_HOLD_Z,
    )
}

fn alignment_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_cassette_reconciliation_alignment_fiducials");
    for (index, (x, y)) in fiducial_points().into_iter().enumerate() {
        let disk = centered_cylinder(
            format!("closed_cassette_reconciliation_alignment_fiducial_disk_{index}"),
            10.0,
            3.0,
            40,
        )
        .translate(x, y, DECK_Z + 1.5);
        let center_dot = centered_cylinder(
            format!("closed_cassette_reconciliation_alignment_fiducial_dot_{index}"),
            3.0,
            4.0,
            28,
        )
        .translate(x, y, DECK_Z + 2.0);
        let cross_x = centered_cube(
            format!("closed_cassette_reconciliation_alignment_fiducial_{index}_cross_x"),
            24.0,
            2.0,
            3.4,
        )
        .translate(x, y, DECK_Z + 1.7);
        let cross_y = centered_cube(
            format!("closed_cassette_reconciliation_alignment_fiducial_{index}_cross_y"),
            2.0,
            24.0,
            3.4,
        )
        .translate(x, y, DECK_Z + 1.7);
        fiducials = fiducials + disk + center_dot + cross_x + cross_y;
    }
    fiducials
}

fn service_indicator_blocks() -> Part {
    let mut blocks = Part::empty("closed_cassette_reconciliation_service_indicator_blocks");
    for index in 0..SERVICE_INDICATOR_COUNT {
        let x = SERVICE_CENTER.0
            - (SERVICE_INDICATOR_COUNT as f64 - 1.0) * SERVICE_BLOCK_PITCH_X / 2.0
            + index as f64 * SERVICE_BLOCK_PITCH_X;
        let block = centered_cube(
            format!("closed_cassette_reconciliation_service_indicator_block_{index}"),
            SERVICE_BLOCK_X,
            SERVICE_BLOCK_Y,
            SERVICE_BLOCK_Z,
        )
        .translate(x, SERVICE_CENTER.1, DECK_Z + SERVICE_BLOCK_Z / 2.0);
        let light = centered_cylinder(
            format!("closed_cassette_reconciliation_service_indicator_light_pipe_{index}"),
            9.0,
            7.0,
            32,
        )
        .translate(x, SERVICE_CENTER.1, DECK_Z + SERVICE_BLOCK_Z + 3.5);
        let label_land = centered_cube(
            format!("closed_cassette_reconciliation_service_indicator_label_land_{index}"),
            SERVICE_BLOCK_X - 16.0,
            6.0,
            8.0,
        )
        .translate(
            x,
            SERVICE_CENTER.1 - SERVICE_BLOCK_Y / 2.0 - 3.0,
            DECK_Z + SERVICE_BLOCK_Z / 2.0,
        );
        blocks = blocks + block + light + label_land;
    }
    blocks
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_cassette_reconciliation_barcode_certificate_land_panel",
        BARCODE_CERT_PANEL_X,
        BARCODE_CERT_PANEL_Y,
        BARCODE_CERT_PANEL_Z,
    );
    let mut lands = Part::empty("closed_cassette_reconciliation_barcode_certificate_lands");
    for index in 0..SLOT_COUNT {
        let col = index % SLOT_COLS;
        let row = index / SLOT_COLS;
        let x = -230.0 + col as f64 * 132.0;
        let y = 30.0 - row as f64 * 20.0;
        lands = lands
            + centered_cube(
                format!("closed_cassette_reconciliation_slot_{index:02}_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, y, BARCODE_CERT_PANEL_Z / 2.0 + 2.0);
    }

    for index in 0..CERTIFICATE_LAND_COUNT {
        let x = -210.0 + index as f64 * 140.0;
        let certificate = centered_cube(
            format!("closed_cassette_reconciliation_certificate_land_{index}"),
            CERTIFICATE_LAND_X,
            CERTIFICATE_LAND_Y,
            4.0,
        )
        .translate(x, -42.0, BARCODE_CERT_PANEL_Z / 2.0 + 2.0);
        let seal = centered_cylinder(
            format!("closed_cassette_reconciliation_certificate_land_{index}_seal_witness"),
            8.0,
            5.0,
            32,
        )
        .translate(
            x + CERTIFICATE_LAND_X / 2.0 - 18.0,
            -42.0,
            BARCODE_CERT_PANEL_Z / 2.0 + 2.5,
        );
        lands = lands + certificate + seal;
    }

    let master = centered_cube(
        "closed_cassette_reconciliation_master_lot_certificate_land",
        160.0,
        30.0,
        5.0,
    )
    .translate(214.0, 42.0, BARCODE_CERT_PANEL_Z / 2.0 + 2.5);

    place_on_deck(
        panel + lands + master,
        BARCODE_CERT_CENTER,
        BARCODE_CERT_PANEL_Z,
    )
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_cassette_reconciliation_release_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut cuts = Part::empty("closed_cassette_reconciliation_release_hold_reject_lane_slot_cuts");
    let mut walls = Part::empty("closed_cassette_reconciliation_release_hold_reject_lane_walls");
    let mut labels =
        Part::empty("closed_cassette_reconciliation_release_hold_reject_lane_label_lands");

    for lane in 0..LANE_COUNT {
        let lane_y = lane_y(lane);
        let backstop = centered_cube(
            format!("closed_cassette_reconciliation_decision_lane_{lane}_backstop"),
            LANE_PANEL_X - 42.0,
            LANE_DIVIDER_W,
            32.0,
        )
        .translate(
            0.0,
            lane_y + LANE_SLOT_Y / 2.0 + 14.0,
            LANE_PANEL_Z / 2.0 + 16.0,
        );
        walls = walls + backstop;

        let lane_label = centered_cube(
            format!("closed_cassette_reconciliation_decision_lane_{lane}_status_label_land"),
            74.0,
            18.0,
            5.0,
        )
        .translate(-LANE_PANEL_X / 2.0 + 48.0, lane_y, LANE_PANEL_Z / 2.0 + 2.5);
        labels = labels + lane_label;

        for slot in 0..LANE_SLOTS_PER_LANE {
            let x = -((LANE_SLOTS_PER_LANE as f64 - 1.0) * LANE_SLOT_PITCH_X) / 2.0
                + slot as f64 * LANE_SLOT_PITCH_X;
            cuts = cuts
                + centered_cube(
                    format!("closed_cassette_reconciliation_decision_lane_{lane}_slot_{slot}"),
                    LANE_SLOT_X,
                    LANE_SLOT_Y,
                    13.0,
                )
                .translate(x, lane_y, LANE_PANEL_Z / 2.0 - 5.5);
            labels = labels
                + centered_cube(
                    format!("closed_cassette_reconciliation_decision_lane_{lane}_slot_{slot}_barcode_land"),
                    68.0,
                    10.0,
                    4.0,
                )
                .translate(x, lane_y - LANE_SLOT_Y / 2.0 - 12.0, LANE_PANEL_Z / 2.0 + 2.0);
        }
    }

    let separators = lane_separators();
    place_on_deck(
        panel - cuts + walls + labels + separators,
        LANE_CENTER,
        LANE_PANEL_Z,
    )
}

fn lane_separators() -> Part {
    let mut separators =
        Part::empty("closed_cassette_reconciliation_release_hold_reject_lane_separators");
    for (index, y) in [
        lane_y(0) + LANE_SLOT_PITCH_Y / 2.0,
        lane_y(1) + LANE_SLOT_PITCH_Y / 2.0,
    ]
    .into_iter()
    .enumerate()
    {
        separators = separators
            + centered_cube(
                format!("closed_cassette_reconciliation_decision_lane_separator_{index}"),
                LANE_PANEL_X - 26.0,
                LANE_DIVIDER_W,
                40.0,
            )
            .translate(0.0, y, LANE_PANEL_Z / 2.0 + 20.0);
    }
    separators
}

fn evidence_bridge() -> Part {
    let left_x = BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0;
    let right_x = BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0;
    let front_y = BRIDGE_CENTER.1 - TRAY_Y / 2.0 + 28.0;
    let rear_y = BRIDGE_CENTER.1 + TRAY_Y / 2.0 - 28.0;

    let mut posts = Part::empty("closed_cassette_reconciliation_evidence_bridge_posts");
    for (index, (x, y)) in [
        (left_x, front_y),
        (right_x, front_y),
        (left_x, rear_y),
        (right_x, rear_y),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_cassette_reconciliation_evidence_bridge_post_{index}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_POST_Z,
            )
            .translate(x, y, DECK_Z + BRIDGE_POST_Z / 2.0);
    }

    let beam = centered_cube(
        "closed_cassette_reconciliation_evidence_bridge_camera_beam",
        BRIDGE_SPAN_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let camera_pods = evidence_camera_pods();
    let leds = evidence_bridge_led_segments();
    let rail = evidence_card_capture_rail();

    posts + beam + camera_pods + leds + rail
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("closed_cassette_reconciliation_evidence_camera_pods");
    for index in 0..EVIDENCE_CAMERA_COUNT {
        let x = BRIDGE_CENTER.0 - (EVIDENCE_CAMERA_COUNT as f64 - 1.0) * CAMERA_PITCH_X / 2.0
            + index as f64 * CAMERA_PITCH_X;
        let body = centered_cube(
            format!("closed_cassette_reconciliation_evidence_camera_pod_{index}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("closed_cassette_reconciliation_evidence_camera_lens_{index}"),
            14.0,
            10.0,
            40,
        )
        .translate(
            x,
            BRIDGE_CENTER.1,
            DECK_Z + BRIDGE_UNDERSIDE_Z - CAMERA_POD_Z - 5.0,
        );
        let focus_gauge = centered_cube(
            format!("closed_cassette_reconciliation_evidence_camera_focus_gauge_{index}"),
            60.0,
            5.0,
            12.0,
        )
        .translate(
            x,
            BRIDGE_CENTER.1 - CAMERA_POD_Y / 2.0 - 3.0,
            DECK_Z + BRIDGE_UNDERSIDE_Z - 28.0,
        );
        pods = pods + body + lens + focus_gauge;
    }
    pods
}

fn evidence_bridge_led_segments() -> Part {
    let mut leds = Part::empty("closed_cassette_reconciliation_evidence_bridge_led_segments");
    for index in 0..LED_SEGMENTS {
        let x = BRIDGE_CENTER.0 - 350.0 + index as f64 * 100.0;
        let front = centered_cube(
            format!("closed_cassette_reconciliation_evidence_bridge_front_led_segment_{index}"),
            56.0,
            8.0,
            8.0,
        )
        .translate(
            x,
            BRIDGE_CENTER.1 - BRIDGE_BEAM_Y / 2.0 - 5.0,
            DECK_Z + BRIDGE_UNDERSIDE_Z + 9.0,
        );
        let rear = centered_cube(
            format!("closed_cassette_reconciliation_evidence_bridge_rear_led_segment_{index}"),
            56.0,
            8.0,
            8.0,
        )
        .translate(
            x,
            BRIDGE_CENTER.1 + BRIDGE_BEAM_Y / 2.0 + 5.0,
            DECK_Z + BRIDGE_UNDERSIDE_Z + 9.0,
        );
        leds = leds + front + rear;
    }
    leds
}

fn evidence_card_capture_rail() -> Part {
    let rail = centered_cube(
        "closed_cassette_reconciliation_evidence_card_capture_rail",
        EVIDENCE_CARD_RAIL_X,
        EVIDENCE_CARD_RAIL_Y,
        22.0,
    )
    .translate(
        TRAY_CENTER.0,
        TRAY_CENTER.1 + TRAY_Y / 2.0 + 36.0,
        DECK_Z + 56.0,
    );
    let mut clip_lands = Part::empty("closed_cassette_reconciliation_evidence_card_clip_lands");
    for index in 0..6 {
        clip_lands = clip_lands
            + centered_cube(
                format!("closed_cassette_reconciliation_evidence_card_clip_land_{index}"),
                42.0,
                10.0,
                14.0,
            )
            .translate(
                TRAY_CENTER.0 - 210.0 + index as f64 * 84.0,
                TRAY_CENTER.1 + TRAY_Y / 2.0 + 54.0,
                DECK_Z + 63.0,
            );
    }
    rail + clip_lands
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_cassette_reconciliation_front_robot_sweep_keepout_gauge",
        DECK_X - 210.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_cassette_reconciliation_rear_scanner_service_keepout_gauge",
        DECK_X - 240.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        28.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0,
        DECK_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "closed_cassette_reconciliation_right_rfid_service_keepout_gauge",
        RIGHT_SERVICE_KEEP_OUT_X,
        DECK_Y - 240.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_KEEP_OUT_X / 2.0,
        -18.0,
        DECK_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left = centered_cube(
        "closed_cassette_reconciliation_left_tray_load_keepout_gauge",
        LEFT_TRAY_LOAD_KEEP_OUT_X,
        DECK_Y - 232.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_TRAY_LOAD_KEEP_OUT_X / 2.0,
        0.0,
        DECK_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let bridge_z = centered_cube(
        "closed_cassette_reconciliation_bridge_z_service_envelope_gauge",
        BRIDGE_SPAN_X,
        BRIDGE_BEAM_Y + 42.0,
        12.0,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 44.0,
    );

    front + rear + right + left + bridge_z
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_relief"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn place_on_deck(part: Part, center: (f64, f64), height: f64) -> Part {
    part.translate(center.0, center.1, DECK_Z + height / 2.0)
}

fn slot_center(index: usize) -> (f64, f64) {
    let col = index % SLOT_COLS;
    let row = index / SLOT_COLS;
    let x = (col as f64 - (SLOT_COLS as f64 - 1.0) / 2.0) * SLOT_PITCH_X;
    let y = ((SLOT_ROWS as f64 - 1.0) / 2.0 - row as f64) * SLOT_PITCH_Y;
    (x, y)
}

fn lane_y(lane: usize) -> f64 {
    ((LANE_COUNT as f64 - 1.0) / 2.0 - lane as f64) * LANE_SLOT_PITCH_Y
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 42.0, -DECK_Y / 2.0 + 42.0),
        (DECK_X / 2.0 - 42.0, -DECK_Y / 2.0 + 42.0),
        (-DECK_X / 2.0 + 42.0, DECK_Y / 2.0 - 42.0),
        (DECK_X / 2.0 - 42.0, DECK_Y / 2.0 - 42.0),
        (TRAY_CENTER.0 - TRAY_X / 2.0 + 28.0, TRAY_CENTER.1),
        (TRAY_CENTER.0 + TRAY_X / 2.0 - 28.0, TRAY_CENTER.1),
        (RFID_CENTER.0, RFID_CENTER.1 + RFID_PANEL_Y / 2.0 + 30.0),
        (
            REJECT_HOLD_CENTER.0,
            REJECT_HOLD_CENTER.1 - REJECT_HOLD_Y / 2.0 - 30.0,
        ),
    ]
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 6] {
    [
        ("cassette_tray", TRAY_CENTER, TRAY_X, TRAY_Y),
        (
            "barcode_camera_window",
            BARCODE_WINDOW_CENTER,
            BARCODE_WINDOW_X,
            BARCODE_WINDOW_Y,
        ),
        ("rfid_antenna_zone", RFID_CENTER, RFID_PANEL_X, RFID_PANEL_Y),
        (
            "reject_hold_pocket",
            REJECT_HOLD_CENTER,
            REJECT_HOLD_X,
            REJECT_HOLD_Y,
        ),
        (
            "barcode_certificate_lands",
            BARCODE_CERT_CENTER,
            BARCODE_CERT_PANEL_X,
            BARCODE_CERT_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            LANE_CENTER,
            LANE_PANEL_X,
            LANE_PANEL_Y,
        ),
    ]
}

fn fiducial_points() -> [(f64, f64); FIDUCIAL_COUNT] {
    [
        (
            TRAY_CENTER.0 - TRAY_X / 2.0 + 28.0,
            TRAY_CENTER.1 - TRAY_Y / 2.0 + 28.0,
        ),
        (
            TRAY_CENTER.0 + TRAY_X / 2.0 - 28.0,
            TRAY_CENTER.1 - TRAY_Y / 2.0 + 28.0,
        ),
        (
            TRAY_CENTER.0 - TRAY_X / 2.0 + 28.0,
            TRAY_CENTER.1 + TRAY_Y / 2.0 - 28.0,
        ),
        (
            TRAY_CENTER.0 + TRAY_X / 2.0 - 28.0,
            TRAY_CENTER.1 + TRAY_Y / 2.0 - 28.0,
        ),
        (
            BARCODE_WINDOW_CENTER.0 - BARCODE_WINDOW_X / 2.0 + 24.0,
            BARCODE_WINDOW_CENTER.1,
        ),
        (
            BARCODE_WINDOW_CENTER.0 + BARCODE_WINDOW_X / 2.0 - 24.0,
            BARCODE_WINDOW_CENTER.1,
        ),
        (
            RFID_CENTER.0 - RFID_PANEL_X / 2.0 + 28.0,
            RFID_CENTER.1 - RFID_PANEL_Y / 2.0 + 28.0,
        ),
        (
            RFID_CENTER.0 + RFID_PANEL_X / 2.0 - 28.0,
            RFID_CENTER.1 + RFID_PANEL_Y / 2.0 - 28.0,
        ),
        (
            LANE_CENTER.0 - LANE_PANEL_X / 2.0 + 30.0,
            LANE_CENTER.1 - LANE_PANEL_Y / 2.0 + 30.0,
        ),
        (
            LANE_CENTER.0 + LANE_PANEL_X / 2.0 - 30.0,
            LANE_CENTER.1 + LANE_PANEL_Y / 2.0 - 30.0,
        ),
        (
            REJECT_HOLD_CENTER.0 - REJECT_HOLD_X / 2.0 + 28.0,
            REJECT_HOLD_CENTER.1,
        ),
        (
            REJECT_HOLD_CENTER.0 + REJECT_HOLD_X / 2.0 - 28.0,
            REJECT_HOLD_CENTER.1,
        ),
    ]
}

fn tray_left_edge() -> f64 {
    TRAY_CENTER.0 - TRAY_X / 2.0
}

fn tray_right_edge() -> f64 {
    TRAY_CENTER.0 + TRAY_X / 2.0
}

fn tray_front_edge() -> f64 {
    TRAY_CENTER.1 - TRAY_Y / 2.0
}

fn rfid_right_edge() -> f64 {
    RFID_CENTER.0 + RFID_PANEL_X / 2.0
}

fn bridge_left_edge() -> f64 {
    BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0
}

fn bridge_right_edge() -> f64 {
    BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0
}

fn evidence_bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

fn release_hold_reject_gap() -> f64 {
    LANE_SLOT_PITCH_Y - LANE_SLOT_Y
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(SLOT_COUNT, 16, "cassette tray must expose exactly 16 slots");
    assert_eq!(
        LANE_COUNT, 3,
        "decision lanes must be release, hold, reject"
    );
    assert_eq!(FIDUCIAL_COUNT, fiducial_points().len());
    assert!(OUTPUTS.iter().all(|path| path
        .starts_with("output/closed_cassette_barcode_rfid_mismatch_reconciliation_station_")));
    assert!(TRAY_CENTER.0.abs() + TRAY_X / 2.0 < DECK_X / 2.0 - 24.0);
    assert!(TRAY_CENTER.1.abs() + TRAY_Y / 2.0 < DECK_Y / 2.0 - 58.0);
    assert!(tray_right_edge() + 118.0 < RFID_CENTER.0 - RFID_PANEL_X / 2.0);
    assert!(RFID_CENTER.0.abs() + RFID_PANEL_X / 2.0 < DECK_X / 2.0 - 72.0);
    assert!(REJECT_HOLD_CENTER.1 - REJECT_HOLD_Y / 2.0 > -DECK_Y / 2.0 + 42.0);
    assert!(BARCODE_CERT_CENTER.1 - BARCODE_CERT_PANEL_Y / 2.0 > -DECK_Y / 2.0 + 28.0);
    assert!(release_hold_reject_gap() >= RELEASE_HOLD_REJECT_GAP_MIN);
    assert!(evidence_bridge_clearance_above_deck() > REVC_TOTAL_HEIGHT + TRAY_Z + 90.0);
    assert!(bridge_left_edge() < tray_left_edge() - 32.0);
    assert!(bridge_right_edge() > rfid_right_edge() + 18.0);
    assert!(tray_front_edge() > BARCODE_WINDOW_CENTER.1 + BARCODE_WINDOW_Y / 2.0 - 24.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_exports_named_feature_groups() {
        assert_layout();
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        for path in OUTPUTS {
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cassette_slots_and_decision_lanes_are_complete() {
        assert_eq!(SLOT_ROWS * SLOT_COLS, SLOT_COUNT);
        assert_eq!(LANE_COUNT * LANE_SLOTS_PER_LANE, 12);
        assert!(release_hold_reject_gap() >= RELEASE_HOLD_REJECT_GAP_MIN);
    }
}
