use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Run-record material scan station for electronic batch record execution.
//
// Intent:
// - Present incoming material lots to barcode/RFID, camera, and scale checks
//   before the batch record can accept them into a run.
// - Keep reagent, media, chip, and connector lots physically staged by type
//   with released/quarantine segregation and a dedicated reject/mismatch pocket.
// - Model cleanable mechanical datums, scan targets, calibration pockets,
//   purchased scanner/camera/scale envelopes, and robot service keepouts only.
//   eBR recipes, Part 11 controls, and actual instrument qualification remain
//   software/validation work outside this fixture model.

const OUTPUTS: [&str; 10] = [
    "output/run_record_material_scan_station_cleanable_deck.stl",
    "output/run_record_material_scan_station_barcode_rfid_scanner_bridge.stl",
    "output/run_record_material_scan_station_lot_staging_pockets.stl",
    "output/run_record_material_scan_station_released_quarantine_lanes.stl",
    "output/run_record_material_scan_station_calibration_standard_scan_pockets.stl",
    "output/run_record_material_scan_station_rejected_mismatch_pocket.stl",
    "output/run_record_material_scan_station_weigh_scale_load_cell_placeholder.stl",
    "output/run_record_material_scan_station_camera_illumination_bar.stl",
    "output/run_record_material_scan_station_robot_service_keepouts.stl",
    "output/run_record_material_scan_station_assembly.stl",
];

const REQUIRED_EBR_FEATURES: [&str; 9] = [
    "barcode_rfid_scanner_bridge",
    "reagent_lot_staging_pockets",
    "media_lot_staging_pockets",
    "chip_lot_staging_pockets",
    "connector_lot_staging_pockets",
    "released_quarantine_status_lanes",
    "calibration_standard_scan_pockets",
    "rejected_mismatch_pocket",
    "weigh_scale_load_cell_placeholder",
];

const DECK_X: f64 = 1040.0;
const DECK_Y: f64 = 720.0;
const DECK_Z: f64 = 18.0;
const DECK_RIM_W: f64 = 14.0;
const DECK_RIM_Z: f64 = 20.0;
const WIPE_GUTTER_W: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 5.4;

const STAGING_CENTER: (f64, f64) = (-230.0, 78.0);
const STAGING_X: f64 = 540.0;
const STAGING_Y: f64 = 490.0;
const STAGING_Z: f64 = 30.0;
const STAGING_RIM_W: f64 = 10.0;
const LOT_POCKET_DEPTH: f64 = 8.0;

const REAGENT_POCKET_COUNT: usize = 8;
const REAGENT_COLS: usize = 4;
const REAGENT_POCKET_X: f64 = 70.0;
const REAGENT_POCKET_Y: f64 = 42.0;
const REAGENT_PITCH_X: f64 = 86.0;
const REAGENT_PITCH_Y: f64 = 58.0;
const REAGENT_ORIGIN: (f64, f64) = (0.0, 176.0);

const MEDIA_POCKET_COUNT: usize = 6;
const MEDIA_COLS: usize = 3;
const MEDIA_POCKET_X: f64 = 90.0;
const MEDIA_POCKET_Y: f64 = 48.0;
const MEDIA_PITCH_X: f64 = 112.0;
const MEDIA_PITCH_Y: f64 = 58.0;
const MEDIA_ORIGIN: (f64, f64) = (0.0, 62.0);

const CHIP_POCKET_COUNT: usize = 4;
const CHIP_COLS: usize = 2;
const CHIP_POCKET_X: f64 = REVC_CHIP_LENGTH + 8.0;
const CHIP_POCKET_Y: f64 = REVC_CHIP_WIDTH * 0.58;
const CHIP_PITCH_X: f64 = CHIP_POCKET_X + 32.0;
const CHIP_PITCH_Y: f64 = CHIP_POCKET_Y + 22.0;
const CHIP_ORIGIN: (f64, f64) = (0.0, -66.0);

const CONNECTOR_POCKET_COUNT: usize = 12;
const CONNECTOR_COLS: usize = 6;
const CONNECTOR_POCKET_X: f64 = 34.0;
const CONNECTOR_POCKET_Y: f64 = 34.0;
const CONNECTOR_PITCH_X: f64 = 62.0;
const CONNECTOR_PITCH_Y: f64 = 58.0;
const CONNECTOR_ORIGIN: (f64, f64) = (0.0, -178.0);

const TOTAL_LOT_POCKETS: usize =
    REAGENT_POCKET_COUNT + MEDIA_POCKET_COUNT + CHIP_POCKET_COUNT + CONNECTOR_POCKET_COUNT;

const LANE_CENTER: (f64, f64) = (280.0, 130.0);
const LANE_PANEL_X: f64 = 360.0;
const LANE_PANEL_Y: f64 = 300.0;
const LANE_PANEL_Z: f64 = 28.0;
const RELEASED_LANE_COUNT: usize = 5;
const QUARANTINE_LANE_COUNT: usize = 5;
const STATUS_LANE_SLOT_X: f64 = 124.0;
const STATUS_LANE_SLOT_Y: f64 = 36.0;
const STATUS_LANE_PITCH_Y: f64 = 48.0;
const RELEASED_LANE_X: f64 = -92.0;
const QUARANTINE_LANE_X: f64 = 92.0;
const STATUS_LANE_SEGREGATION_MIN: f64 = 48.0;

const CAL_CENTER: (f64, f64) = (-310.0, -250.0);
const CAL_PANEL_X: f64 = 370.0;
const CAL_PANEL_Y: f64 = 126.0;
const CAL_PANEL_Z: f64 = 24.0;
const CAL_STANDARD_COUNT: usize = 4;
const CAL_STANDARD_PITCH: f64 = 82.0;
const CAL_STANDARD_POCKET_D: f64 = 32.0;

const SCALE_CENTER: (f64, f64) = (55.0, -248.0);
const SCALE_X: f64 = 176.0;
const SCALE_Y: f64 = 142.0;
const SCALE_Z: f64 = 26.0;
const SCALE_PAN_D: f64 = 98.0;
const LOAD_CELL_BEAM_X: f64 = 118.0;
const LOAD_CELL_BEAM_Y: f64 = 28.0;

const REJECT_CENTER: (f64, f64) = (320.0, -244.0);
const REJECT_X: f64 = 190.0;
const REJECT_Y: f64 = 138.0;
const REJECT_Z: f64 = 48.0;
const REJECT_WALL: f64 = 8.0;
const REJECT_SEGREGATION_MIN: f64 = 58.0;

const BRIDGE_SPAN_X: f64 = STAGING_X + 92.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 58.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const BRIDGE_UNDERSIDE_Z: f64 = 134.0;
const BRIDGE_POST_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const SCANNER_SLED_X: f64 = 116.0;
const SCANNER_SLED_Y: f64 = 62.0;
const RFID_ANTENNA_X: f64 = 154.0;
const RFID_ANTENNA_Y: f64 = 84.0;

const CAMERA_BAR_CENTER: (f64, f64) = (42.0, 20.0);
const CAMERA_BAR_X: f64 = 790.0;
const CAMERA_BAR_Y: f64 = 34.0;
const CAMERA_BAR_Z: f64 = 26.0;
const CAMERA_BAR_UNDERSIDE_Z: f64 = 162.0;
const CAMERA_COUNT: usize = 3;
const CAMERA_PITCH_X: f64 = 240.0;
const LED_SEGMENTS: usize = 8;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 104.0;
const REAR_CABLE_KEEP_OUT_Y: f64 = 76.0;
const LEFT_ROBOT_KEEP_OUT_X: f64 = 86.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 126.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_deck();
    export(OUTPUTS[0], &deck);

    let scanner_bridge = barcode_rfid_scanner_bridge();
    export(OUTPUTS[1], &scanner_bridge);

    let staging = material_lot_staging_pockets();
    export(OUTPUTS[2], &staging);

    let lanes = released_quarantine_lanes();
    export(OUTPUTS[3], &lanes);

    let calibration = calibration_standard_scan_pockets();
    export(OUTPUTS[4], &calibration);

    let rejected = rejected_mismatch_pocket();
    export(OUTPUTS[5], &rejected);

    let scale = weigh_scale_load_cell_placeholder();
    export(OUTPUTS[6], &scale);

    let camera_bar = camera_illumination_bar();
    export(OUTPUTS[7], &camera_bar);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[8], &keepouts);

    let assembly = deck
        + staging.translate(STAGING_CENTER.0, STAGING_CENTER.1, deck_insert_z(STAGING_Z))
        + lanes.translate(LANE_CENTER.0, LANE_CENTER.1, deck_insert_z(LANE_PANEL_Z))
        + calibration.translate(CAL_CENTER.0, CAL_CENTER.1, deck_insert_z(CAL_PANEL_Z))
        + scale.translate(SCALE_CENTER.0, SCALE_CENTER.1, deck_insert_z(SCALE_Z))
        + rejected.translate(REJECT_CENTER.0, REJECT_CENTER.1, deck_insert_z(REJECT_Z))
        + scanner_bridge.translate(STAGING_CENTER.0, STAGING_CENTER.1, DECK_Z / 2.0)
        + camera_bar.translate(CAMERA_BAR_CENTER.0, CAMERA_BAR_CENTER.1, DECK_Z / 2.0)
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_GAUGE_Z / 2.0);
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Run-record material scan station:");
    println!("  Cleanable deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Lot staging pockets:         {REAGENT_POCKET_COUNT} reagent, {MEDIA_POCKET_COUNT} media, {CHIP_POCKET_COUNT} chip, {CONNECTOR_POCKET_COUNT} connector ({TOTAL_LOT_POCKETS} total)"
    );
    println!(
        "  Status lanes:                {RELEASED_LANE_COUNT} released and {QUARANTINE_LANE_COUNT} quarantine slots with {:.0}mm minimum lane gap",
        status_lane_gap()
    );
    println!(
        "  Calibration scan pockets:    {CAL_STANDARD_COUNT} standard/token pockets plus barcode/RFID label lands"
    );
    println!(
        "  Weigh scale placeholder:     {SCALE_X:.0}mm x {SCALE_Y:.0}mm base, {SCALE_PAN_D:.0}mm pan, {LOAD_CELL_BEAM_X:.0}mm load-cell beam"
    );
    println!(
        "  Scanner/camera clearance:    {:.0}mm scanner bridge underside, {:.0}mm camera bar underside above deck top",
        bridge_clearance_above_deck(),
        camera_bar_clearance_above_deck()
    );
    println!(
        "  eBR feature groups covered:  {}",
        REQUIRED_EBR_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert!(fits_on_deck(STAGING_CENTER, STAGING_X, STAGING_Y, 10.0));
    assert!(fits_on_deck(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y, 10.0));
    assert!(fits_on_deck(CAL_CENTER, CAL_PANEL_X, CAL_PANEL_Y, 10.0));
    assert!(fits_on_deck(SCALE_CENTER, SCALE_X, SCALE_Y, 10.0));
    assert!(fits_on_deck(REJECT_CENTER, REJECT_X, REJECT_Y, 10.0));
    assert!(
        status_lane_gap() >= STATUS_LANE_SEGREGATION_MIN,
        "released/quarantine lanes are too close for status segregation"
    );
    assert!(
        horizontal_gap(
            rect(REJECT_CENTER, REJECT_X, REJECT_Y),
            rect(SCALE_CENTER, SCALE_X, SCALE_Y)
        ) >= REJECT_SEGREGATION_MIN,
        "reject/mismatch pocket is too close to the scale datum"
    );
    assert!(
        vertical_gap(
            rect(STAGING_CENTER, STAGING_X, STAGING_Y),
            rect(CAL_CENTER, CAL_PANEL_X, CAL_PANEL_Y)
        ) >= 14.0,
        "calibration standards collide with staging tray"
    );
    assert!(
        !rects_overlap(
            rect(STAGING_CENTER, STAGING_X, STAGING_Y),
            rect(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y)
        ),
        "status lanes collide with staging tray"
    );
}

fn cleanable_deck() -> Part {
    let deck = centered_cube(
        "run_record_material_scan_station_cleanable_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let staging_recess = top_recess(
        "run_record_material_scan_station_staging_module_recess",
        STAGING_CENTER,
        STAGING_X + 18.0,
        STAGING_Y + 18.0,
        5.0,
    );
    let lane_recess = top_recess(
        "run_record_material_scan_station_status_lane_recess",
        LANE_CENTER,
        LANE_PANEL_X + 18.0,
        LANE_PANEL_Y + 18.0,
        5.0,
    );
    let calibration_recess = top_recess(
        "run_record_material_scan_station_calibration_recess",
        CAL_CENTER,
        CAL_PANEL_X + 18.0,
        CAL_PANEL_Y + 18.0,
        4.0,
    );
    let scale_recess = top_recess(
        "run_record_material_scan_station_scale_recess",
        SCALE_CENTER,
        SCALE_X + 22.0,
        SCALE_Y + 22.0,
        5.0,
    );
    let reject_recess = top_recess(
        "run_record_material_scan_station_reject_bin_recess",
        REJECT_CENTER,
        REJECT_X + 22.0,
        REJECT_Y + 22.0,
        5.0,
    );

    let gutters = cleanable_wipe_gutters();
    let drains = drain_and_mount_holes();

    deck - staging_recess
        - lane_recess
        - calibration_recess
        - scale_recess
        - reject_recess
        - gutters
        - drains
        + deck_perimeter_lips()
        + rear_cable_datum_rail()
}

fn top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - depth / 2.0 + 0.1,
    )
}

fn cleanable_wipe_gutters() -> Part {
    let left_gutter = centered_cube(
        "run_record_material_scan_station_left_wipe_gutter",
        WIPE_GUTTER_W,
        DECK_Y - 112.0,
        6.0,
    )
    .translate(-82.0, 18.0, DECK_Z / 2.0 - 2.4);
    let center_gutter = centered_cube(
        "run_record_material_scan_station_center_wipe_gutter",
        DECK_X - 146.0,
        WIPE_GUTTER_W,
        6.0,
    )
    .translate(8.0, -84.0, DECK_Z / 2.0 - 2.4);
    let front_sump = centered_cube(
        "run_record_material_scan_station_front_cleanout_sump",
        DECK_X - 190.0,
        18.0,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 52.0, DECK_Z / 2.0 - 2.6);

    left_gutter + center_gutter + front_sump
}

fn drain_and_mount_holes() -> Part {
    let drain = centered_cylinder(
        "run_record_material_scan_station_front_sump_drain",
        12.0,
        DECK_Z + 4.0,
        36,
    )
    .translate(DECK_X / 2.0 - 76.0, -DECK_Y / 2.0 + 52.0, 0.0);

    let mut mounts = Part::empty("run_record_material_scan_station_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        mounts = mounts
            + centered_cylinder(
                format!("run_record_material_scan_station_m5_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    drain + mounts
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "run_record_material_scan_station_rear_cleanable_lip",
        DECK_X - 96.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 34.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);
    let left = centered_cube(
        "run_record_material_scan_station_left_cleanable_lip",
        DECK_RIM_W,
        DECK_Y - 124.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 34.0, 0.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);
    let front_low = centered_cube(
        "run_record_material_scan_station_front_low_retaining_lip",
        DECK_X - 240.0,
        10.0,
        12.0,
    )
    .translate(-20.0, -DECK_Y / 2.0 + 31.0, DECK_Z / 2.0 + 6.0);

    rear + left + front_low
}

fn rear_cable_datum_rail() -> Part {
    let rail = centered_cube(
        "run_record_material_scan_station_rear_scanner_cable_datum_rail",
        DECK_X - 240.0,
        16.0,
        22.0,
    )
    .translate(36.0, DECK_Y / 2.0 - 72.0, DECK_Z / 2.0 + 11.0);

    let mut glands = Part::empty("run_record_material_scan_station_rear_cable_gland_cuts");
    for i in 0..6 {
        glands = glands
            + centered_cylinder(
                format!("run_record_material_scan_station_rear_cable_gland_{i}"),
                7.0,
                20.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -248.0 + i as f64 * 96.0,
                DECK_Y / 2.0 - 72.0,
                DECK_Z / 2.0 + 11.0,
            );
    }

    rail - glands
}

fn material_lot_staging_pockets() -> Part {
    let tray = centered_cube(
        "run_record_material_scan_station_lot_staging_tray",
        STAGING_X,
        STAGING_Y,
        STAGING_Z,
    );
    let basin = centered_cube(
        "run_record_material_scan_station_lot_staging_washdown_basin",
        STAGING_X - 34.0,
        STAGING_Y - 34.0,
        7.0,
    )
    .translate(0.0, 0.0, STAGING_Z / 2.0 - 3.0);

    tray - basin - lot_pocket_cuts()
        + staging_rims()
        + lot_label_lands()
        + staging_group_dividers()
        + staging_gripper_fiducials()
}

fn lot_pocket_cuts() -> Part {
    grid_pocket_cuts(
        "reagent",
        REAGENT_POCKET_COUNT,
        REAGENT_COLS,
        REAGENT_POCKET_X,
        REAGENT_POCKET_Y,
        REAGENT_PITCH_X,
        REAGENT_PITCH_Y,
        REAGENT_ORIGIN,
    ) + grid_pocket_cuts(
        "media",
        MEDIA_POCKET_COUNT,
        MEDIA_COLS,
        MEDIA_POCKET_X,
        MEDIA_POCKET_Y,
        MEDIA_PITCH_X,
        MEDIA_PITCH_Y,
        MEDIA_ORIGIN,
    ) + grid_pocket_cuts(
        "chip",
        CHIP_POCKET_COUNT,
        CHIP_COLS,
        CHIP_POCKET_X,
        CHIP_POCKET_Y,
        CHIP_PITCH_X,
        CHIP_PITCH_Y,
        CHIP_ORIGIN,
    ) + grid_pocket_cuts(
        "connector",
        CONNECTOR_POCKET_COUNT,
        CONNECTOR_COLS,
        CONNECTOR_POCKET_X,
        CONNECTOR_POCKET_Y,
        CONNECTOR_PITCH_X,
        CONNECTOR_PITCH_Y,
        CONNECTOR_ORIGIN,
    )
}

fn grid_pocket_cuts(
    name: &str,
    count: usize,
    cols: usize,
    pocket_x: f64,
    pocket_y: f64,
    pitch_x: f64,
    pitch_y: f64,
    origin: (f64, f64),
) -> Part {
    let mut cuts = Part::empty(format!(
        "run_record_material_scan_station_{name}_pocket_cuts"
    ));
    for index in 0..count {
        let (x, y) = grid_position(index, count, cols, pitch_x, pitch_y);
        cuts = cuts
            + centered_cube(
                format!("run_record_material_scan_station_{name}_lot_pocket_{index}"),
                pocket_x,
                pocket_y,
                LOT_POCKET_DEPTH + 0.2,
            )
            .translate(
                origin.0 + x,
                origin.1 + y,
                STAGING_Z / 2.0 - LOT_POCKET_DEPTH / 2.0 + 0.1,
            );
    }
    cuts
}

fn staging_rims() -> Part {
    let front = centered_cube(
        "run_record_material_scan_station_staging_front_rim",
        STAGING_X,
        STAGING_RIM_W,
        26.0,
    )
    .translate(
        0.0,
        -STAGING_Y / 2.0 + STAGING_RIM_W / 2.0,
        STAGING_Z / 2.0 + 13.0,
    );
    let rear = centered_cube(
        "run_record_material_scan_station_staging_rear_rim",
        STAGING_X,
        STAGING_RIM_W,
        30.0,
    )
    .translate(
        0.0,
        STAGING_Y / 2.0 - STAGING_RIM_W / 2.0,
        STAGING_Z / 2.0 + 15.0,
    );
    let left = centered_cube(
        "run_record_material_scan_station_staging_left_rim",
        STAGING_RIM_W,
        STAGING_Y,
        28.0,
    )
    .translate(
        -STAGING_X / 2.0 + STAGING_RIM_W / 2.0,
        0.0,
        STAGING_Z / 2.0 + 14.0,
    );
    let right = centered_cube(
        "run_record_material_scan_station_staging_right_low_rim",
        STAGING_RIM_W,
        STAGING_Y - 92.0,
        22.0,
    )
    .translate(
        STAGING_X / 2.0 - STAGING_RIM_W / 2.0,
        18.0,
        STAGING_Z / 2.0 + 11.0,
    );

    front + rear + left + right
}

fn lot_label_lands() -> Part {
    let mut lands = Part::empty("run_record_material_scan_station_lot_label_lands");
    for (label, y) in [
        ("reagent", REAGENT_ORIGIN.1 + 38.0),
        ("media", MEDIA_ORIGIN.1 + 35.0),
        ("chip", CHIP_ORIGIN.1 + 49.0),
        ("connector", CONNECTOR_ORIGIN.1 + 35.0),
    ] {
        lands = lands
            + centered_cube(
                format!("run_record_material_scan_station_{label}_group_barcode_land"),
                STAGING_X - 98.0,
                13.0,
                3.0,
            )
            .translate(0.0, y, STAGING_Z / 2.0 + 1.5);
    }
    lands
}

fn staging_group_dividers() -> Part {
    let divider_1 = centered_cube(
        "run_record_material_scan_station_reagent_media_divider",
        STAGING_X - 74.0,
        8.0,
        20.0,
    )
    .translate(0.0, 116.0, STAGING_Z / 2.0 + 10.0);
    let divider_2 = centered_cube(
        "run_record_material_scan_station_media_chip_divider",
        STAGING_X - 74.0,
        8.0,
        20.0,
    )
    .translate(0.0, -2.0, STAGING_Z / 2.0 + 10.0);
    let divider_3 = centered_cube(
        "run_record_material_scan_station_chip_connector_divider",
        STAGING_X - 74.0,
        8.0,
        20.0,
    )
    .translate(0.0, -126.0, STAGING_Z / 2.0 + 10.0);

    divider_1 + divider_2 + divider_3
}

fn staging_gripper_fiducials() -> Part {
    let mut fiducials = Part::empty("run_record_material_scan_station_staging_gripper_fiducials");
    for (i, (x, y)) in [
        (-(STAGING_X / 2.0 - 42.0), -(STAGING_Y / 2.0 - 42.0)),
        (STAGING_X / 2.0 - 42.0, -(STAGING_Y / 2.0 - 42.0)),
        (-(STAGING_X / 2.0 - 42.0), STAGING_Y / 2.0 - 42.0),
        (STAGING_X / 2.0 - 42.0, STAGING_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("run_record_material_scan_station_staging_fiducial_disc_{i}"),
            7.0,
            3.0,
            32,
        )
        .translate(*x, *y, STAGING_Z / 2.0 + 1.5);
        let center = centered_cylinder(
            format!("run_record_material_scan_station_staging_fiducial_center_{i}"),
            1.5,
            4.0,
            18,
        )
        .translate(*x, *y, STAGING_Z / 2.0 + 1.5);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn released_quarantine_lanes() -> Part {
    let panel = centered_cube(
        "run_record_material_scan_station_status_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let wash_basin = centered_cube(
        "run_record_material_scan_station_status_lane_wash_basin",
        LANE_PANEL_X - 44.0,
        LANE_PANEL_Y - 42.0,
        6.0,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0 - 2.5);

    let released_slots = status_lane_slots("released", RELEASED_LANE_X, RELEASED_LANE_COUNT);
    let quarantine_slots =
        status_lane_slots("quarantine", QUARANTINE_LANE_X, QUARANTINE_LANE_COUNT);

    panel - wash_basin - released_slots - quarantine_slots
        + status_lane_dividers()
        + status_lane_label_strips()
        + status_lane_gate_flags()
}

fn status_lane_slots(name: &str, x: f64, count: usize) -> Part {
    let mut slots = Part::empty(format!(
        "run_record_material_scan_station_{name}_lane_slots"
    ));
    for i in 0..count {
        let y = centered_index(i, count, STATUS_LANE_PITCH_Y);
        slots = slots
            + centered_cube(
                format!("run_record_material_scan_station_{name}_lane_slot_{i}"),
                STATUS_LANE_SLOT_X,
                STATUS_LANE_SLOT_Y,
                7.0,
            )
            .translate(x, y, LANE_PANEL_Z / 2.0 - 3.0);
    }
    slots
}

fn status_lane_dividers() -> Part {
    let center_bar = centered_cube(
        "run_record_material_scan_station_status_lane_hard_divider",
        24.0,
        LANE_PANEL_Y - 46.0,
        36.0,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0 + 18.0);
    let released_outer = centered_cube(
        "run_record_material_scan_station_released_outer_lane_fence",
        10.0,
        LANE_PANEL_Y - 64.0,
        24.0,
    )
    .translate(
        RELEASED_LANE_X - STATUS_LANE_SLOT_X / 2.0 - 12.0,
        0.0,
        LANE_PANEL_Z / 2.0 + 12.0,
    );
    let quarantine_outer = centered_cube(
        "run_record_material_scan_station_quarantine_outer_lane_fence",
        10.0,
        LANE_PANEL_Y - 64.0,
        24.0,
    )
    .translate(
        QUARANTINE_LANE_X + STATUS_LANE_SLOT_X / 2.0 + 12.0,
        0.0,
        LANE_PANEL_Z / 2.0 + 12.0,
    );

    center_bar + released_outer + quarantine_outer
}

fn status_lane_label_strips() -> Part {
    let released = centered_cube(
        "run_record_material_scan_station_released_lane_barcode_strip",
        132.0,
        16.0,
        4.0,
    )
    .translate(
        RELEASED_LANE_X,
        LANE_PANEL_Y / 2.0 - 28.0,
        LANE_PANEL_Z / 2.0 + 2.0,
    );
    let quarantine = centered_cube(
        "run_record_material_scan_station_quarantine_lane_barcode_strip",
        132.0,
        16.0,
        4.0,
    )
    .translate(
        QUARANTINE_LANE_X,
        LANE_PANEL_Y / 2.0 - 28.0,
        LANE_PANEL_Z / 2.0 + 2.0,
    );

    released + quarantine
}

fn status_lane_gate_flags() -> Part {
    let released_flag = centered_cube(
        "run_record_material_scan_station_released_green_gate_flag_placeholder",
        76.0,
        10.0,
        36.0,
    )
    .translate(
        RELEASED_LANE_X,
        -LANE_PANEL_Y / 2.0 + 29.0,
        LANE_PANEL_Z / 2.0 + 18.0,
    );
    let quarantine_flag = centered_cube(
        "run_record_material_scan_station_quarantine_hold_gate_flag_placeholder",
        76.0,
        10.0,
        36.0,
    )
    .translate(
        QUARANTINE_LANE_X,
        -LANE_PANEL_Y / 2.0 + 29.0,
        LANE_PANEL_Z / 2.0 + 18.0,
    );

    released_flag + quarantine_flag
}

fn calibration_standard_scan_pockets() -> Part {
    let panel = centered_cube(
        "run_record_material_scan_station_calibration_standard_panel",
        CAL_PANEL_X,
        CAL_PANEL_Y,
        CAL_PANEL_Z,
    );
    let relief = centered_cube(
        "run_record_material_scan_station_calibration_panel_wipe_relief",
        CAL_PANEL_X - 38.0,
        CAL_PANEL_Y - 34.0,
        5.0,
    )
    .translate(0.0, 0.0, CAL_PANEL_Z / 2.0 - 2.0);

    let mut standard_pockets =
        Part::empty("run_record_material_scan_station_calibration_standard_pockets");
    let mut scan_label_windows =
        Part::empty("run_record_material_scan_station_calibration_standard_scan_windows");
    for i in 0..CAL_STANDARD_COUNT {
        let x = centered_index(i, CAL_STANDARD_COUNT, CAL_STANDARD_PITCH);
        let pocket = centered_cylinder(
            format!("run_record_material_scan_station_cal_standard_round_pocket_{i}"),
            CAL_STANDARD_POCKET_D / 2.0,
            CAL_PANEL_Z + 2.0,
            32,
        )
        .translate(x, 16.0, 0.0);
        let scan_label_window = centered_cube(
            format!("run_record_material_scan_station_cal_standard_barcode_window_{i}"),
            58.0,
            13.0,
            4.0,
        )
        .translate(x, -34.0, CAL_PANEL_Z / 2.0 + 2.0);
        standard_pockets = standard_pockets + pocket;
        scan_label_windows = scan_label_windows + scan_label_window;
    }

    let datum_bar = centered_cube(
        "run_record_material_scan_station_calibration_rear_datum_bar",
        CAL_PANEL_X - 48.0,
        10.0,
        24.0,
    )
    .translate(0.0, CAL_PANEL_Y / 2.0 - 18.0, CAL_PANEL_Z / 2.0 + 12.0);

    panel - relief - standard_pockets
        + scan_label_windows
        + datum_bar
        + calibration_token_fiducials()
}

fn calibration_token_fiducials() -> Part {
    let mut fiducials = Part::empty("run_record_material_scan_station_calibration_fiducials");
    for (i, x) in [-144.0, 144.0].iter().enumerate() {
        let disc = centered_cylinder(
            format!("run_record_material_scan_station_calibration_fiducial_disc_{i}"),
            5.0,
            2.0,
            28,
        )
        .translate(*x, -50.0, CAL_PANEL_Z / 2.0 + 1.0);
        let center = centered_cylinder(
            format!("run_record_material_scan_station_calibration_fiducial_center_{i}"),
            1.2,
            3.0,
            18,
        )
        .translate(*x, -50.0, CAL_PANEL_Z / 2.0 + 1.0);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn rejected_mismatch_pocket() -> Part {
    let base = centered_cube(
        "run_record_material_scan_station_reject_mismatch_bin_base",
        REJECT_X,
        REJECT_Y,
        REJECT_Z,
    );
    let cavity = centered_cube(
        "run_record_material_scan_station_reject_mismatch_bin_cavity",
        REJECT_X - 2.0 * REJECT_WALL,
        REJECT_Y - 2.0 * REJECT_WALL,
        REJECT_Z - 10.0,
    )
    .translate(0.0, 0.0, 7.0);
    let front_dump_slot = centered_cube(
        "run_record_material_scan_station_reject_front_dump_slot",
        REJECT_X - 56.0,
        REJECT_WALL + 4.0,
        22.0,
    )
    .translate(0.0, -REJECT_Y / 2.0, 2.0);
    let mismatch_flag = centered_cube(
        "run_record_material_scan_station_reject_mismatch_status_flag",
        REJECT_X - 52.0,
        7.0,
        20.0,
    )
    .translate(0.0, REJECT_Y / 2.0 + 4.0, REJECT_Z / 2.0 + 10.0);
    let drain = centered_cylinder(
        "run_record_material_scan_station_reject_bin_wipe_drain",
        6.0,
        REJECT_Z + 4.0,
        24,
    )
    .translate(REJECT_X / 2.0 - 28.0, -REJECT_Y / 2.0 + 24.0, 0.0);

    base - cavity - front_dump_slot - drain + mismatch_flag + rejected_bin_datum_tabs()
}

fn rejected_bin_datum_tabs() -> Part {
    let left = centered_cube(
        "run_record_material_scan_station_reject_left_datum_tab",
        28.0,
        14.0,
        12.0,
    )
    .translate(
        -REJECT_X / 2.0 + 30.0,
        REJECT_Y / 2.0 - 12.0,
        -REJECT_Z / 2.0 - 6.0,
    );
    let right = centered_cube(
        "run_record_material_scan_station_reject_right_datum_tab",
        28.0,
        14.0,
        12.0,
    )
    .translate(
        REJECT_X / 2.0 - 30.0,
        REJECT_Y / 2.0 - 12.0,
        -REJECT_Z / 2.0 - 6.0,
    );
    left + right
}

fn weigh_scale_load_cell_placeholder() -> Part {
    let scale_base = centered_cube(
        "run_record_material_scan_station_scale_base_plate",
        SCALE_X,
        SCALE_Y,
        SCALE_Z,
    );
    let load_cell = centered_cube(
        "run_record_material_scan_station_load_cell_beam_placeholder",
        LOAD_CELL_BEAM_X,
        LOAD_CELL_BEAM_Y,
        18.0,
    )
    .translate(0.0, -22.0, SCALE_Z / 2.0 + 9.0);
    let pan = centered_cylinder(
        "run_record_material_scan_station_scale_round_pan_placeholder",
        SCALE_PAN_D / 2.0,
        8.0,
        64,
    )
    .translate(0.0, 18.0, SCALE_Z / 2.0 + 26.0);
    let pan_recess = centered_cylinder(
        "run_record_material_scan_station_scale_pan_recess",
        SCALE_PAN_D / 2.0 + 8.0,
        5.0,
        64,
    )
    .translate(0.0, 18.0, SCALE_Z / 2.0 - 2.0);
    let cable_gland = centered_cylinder(
        "run_record_material_scan_station_scale_cable_gland_cut",
        6.0,
        SCALE_Y + 8.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SCALE_X / 2.0 - 28.0, 0.0, 0.0);
    let ebr_label_land = centered_cube(
        "run_record_material_scan_station_scale_ebr_label_land",
        92.0,
        16.0,
        3.0,
    )
    .translate(0.0, -SCALE_Y / 2.0 + 22.0, SCALE_Z / 2.0 + 1.5);

    scale_base - pan_recess - cable_gland + load_cell + pan + ebr_label_land
}

fn barcode_rfid_scanner_bridge() -> Part {
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
        "run_record_material_scan_station_scanner_bridge_crossbeam",
        BRIDGE_SPAN_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let scanner_sled = centered_cube(
        "run_record_material_scan_station_hybrid_barcode_scanner_sled",
        SCANNER_SLED_X,
        SCANNER_SLED_Y,
        38.0,
    )
    .translate(-96.0, -34.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 19.0);
    let scanner_window = centered_cube(
        "run_record_material_scan_station_scanner_downward_read_window",
        72.0,
        12.0,
        18.0,
    )
    .translate(-96.0, -64.0, BRIDGE_UNDERSIDE_Z - 9.0);
    let rfid_antenna = frame_xy(
        "run_record_material_scan_station_rfid_antenna_loop",
        RFID_ANTENNA_X,
        RFID_ANTENNA_Y,
        8.0,
        8.0,
    )
    .translate(96.0, 24.0, BRIDGE_UNDERSIDE_Z - 4.0);
    let operator_badge_pad = centered_cube(
        "run_record_material_scan_station_operator_badge_pad",
        78.0,
        52.0,
        6.0,
    )
    .translate(214.0, -30.0, BRIDGE_UNDERSIDE_Z + 3.0);
    let usb_strain_relief = centered_cube(
        "run_record_material_scan_station_scanner_usb_strain_relief",
        116.0,
        16.0,
        18.0,
    )
    .translate(-96.0, BRIDGE_POST_Y / 2.0 + 12.0, BRIDGE_UNDERSIDE_Z + 18.0);

    left_post
        + right_post
        + beam
        + scanner_sled
        + scanner_window
        + rfid_antenna
        + operator_badge_pad
        + usb_strain_relief
}

fn bridge_post(name: &str) -> Part {
    let post = centered_cube(
        format!("run_record_material_scan_station_scanner_bridge_{name}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    );
    let service_slot = centered_cube(
        format!("run_record_material_scan_station_scanner_bridge_{name}_cable_service_slot"),
        13.0,
        BRIDGE_POST_Y + 4.0,
        86.0,
    )
    .translate(0.0, 0.0, 58.0);
    let foot = centered_cube(
        format!("run_record_material_scan_station_scanner_bridge_{name}_foot"),
        74.0,
        82.0,
        10.0,
    )
    .translate(0.0, 0.0, 5.0);
    let mounting_holes = centered_cylinder(
        format!("run_record_material_scan_station_scanner_bridge_{name}_m5_pair_cut_a"),
        2.7,
        14.0,
        22,
    )
    .translate(-22.0, -24.0, 5.0)
        + centered_cylinder(
            format!("run_record_material_scan_station_scanner_bridge_{name}_m5_pair_cut_b"),
            2.7,
            14.0,
            22,
        )
        .translate(22.0, 24.0, 5.0);

    post - service_slot + (foot - mounting_holes)
}

fn camera_illumination_bar() -> Part {
    let beam = centered_cube(
        "run_record_material_scan_station_camera_illumination_bar",
        CAMERA_BAR_X,
        CAMERA_BAR_Y,
        CAMERA_BAR_Z,
    )
    .translate(0.0, 0.0, CAMERA_BAR_UNDERSIDE_Z + CAMERA_BAR_Z / 2.0);

    let mut camera_blocks = Part::empty("run_record_material_scan_station_camera_blocks");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, CAMERA_PITCH_X);
        camera_blocks = camera_blocks
            + centered_cube(
                format!("run_record_material_scan_station_camera_module_{i}"),
                68.0,
                44.0,
                32.0,
            )
            .translate(x, -4.0, CAMERA_BAR_UNDERSIDE_Z - 16.0)
            + centered_cylinder(
                format!("run_record_material_scan_station_camera_lens_clearance_{i}"),
                10.0,
                14.0,
                32,
            )
            .translate(x, -4.0, CAMERA_BAR_UNDERSIDE_Z - 36.0);
    }

    let mut led_segments = Part::empty("run_record_material_scan_station_led_light_segments");
    for i in 0..LED_SEGMENTS {
        let x = centered_index(i, LED_SEGMENTS, CAMERA_BAR_X / LED_SEGMENTS as f64);
        led_segments = led_segments
            + centered_cube(
                format!("run_record_material_scan_station_diffuse_led_segment_{i}"),
                64.0,
                10.0,
                5.0,
            )
            .translate(x, CAMERA_BAR_Y / 2.0 + 7.0, CAMERA_BAR_UNDERSIDE_Z - 3.0);
    }

    beam + camera_blocks + led_segments + camera_bar_stanchions()
}

fn camera_bar_stanchions() -> Part {
    let left = centered_cube(
        "run_record_material_scan_station_camera_bar_left_stanchion",
        26.0,
        52.0,
        CAMERA_BAR_UNDERSIDE_Z,
    )
    .translate(
        -CAMERA_BAR_X / 2.0 + 28.0,
        0.0,
        CAMERA_BAR_UNDERSIDE_Z / 2.0,
    );
    let right = centered_cube(
        "run_record_material_scan_station_camera_bar_right_stanchion",
        26.0,
        52.0,
        CAMERA_BAR_UNDERSIDE_Z,
    )
    .translate(CAMERA_BAR_X / 2.0 - 28.0, 0.0, CAMERA_BAR_UNDERSIDE_Z / 2.0);
    let left_foot = centered_cube(
        "run_record_material_scan_station_camera_bar_left_foot",
        88.0,
        76.0,
        10.0,
    )
    .translate(-CAMERA_BAR_X / 2.0 + 28.0, 0.0, 5.0);
    let right_foot = centered_cube(
        "run_record_material_scan_station_camera_bar_right_foot",
        88.0,
        76.0,
        10.0,
    )
    .translate(CAMERA_BAR_X / 2.0 - 28.0, 0.0, 5.0);

    left + right + left_foot + right_foot
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "run_record_material_scan_station_front_robot_handoff_keepout",
        DECK_X - 80.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 10.0,
        0.0,
    );
    let rear_cable = centered_cube(
        "run_record_material_scan_station_rear_cable_service_keepout",
        DECK_X - 126.0,
        REAR_CABLE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_CABLE_KEEP_OUT_Y / 2.0 - 8.0, 0.0);
    let left_robot = centered_cube(
        "run_record_material_scan_station_left_robot_sweep_keepout",
        LEFT_ROBOT_KEEP_OUT_X,
        DECK_Y - 140.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(-DECK_X / 2.0 + LEFT_ROBOT_KEEP_OUT_X / 2.0 + 8.0, 4.0, 0.0);
    let right_service = centered_cube(
        "run_record_material_scan_station_right_scale_service_keepout",
        RIGHT_SERVICE_KEEP_OUT_X,
        DECK_Y - 190.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_KEEP_OUT_X / 2.0 - 8.0,
        -10.0,
        0.0,
    );

    front_robot + rear_cable + left_robot + right_service
}

fn frame_xy(name: impl Into<String>, outer_x: f64, outer_y: f64, rail: f64, z_t: f64) -> Part {
    let base = name.into();
    let front = centered_cube(format!("{base}_front"), outer_x, rail, z_t).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{base}_rear"), outer_x, rail, z_t).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{base}_left"), rail, outer_y, z_t).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{base}_right"), rail, outer_y, z_t).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn grid_position(
    index: usize,
    count: usize,
    cols: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    let rows = count.div_ceil(cols);
    let row = index / cols;
    let col = index % cols;
    let used_cols = if row == rows - 1 && count % cols != 0 {
        count % cols
    } else {
        cols
    };
    let x = -((used_cols as f64 - 1.0) * pitch_x) / 2.0 + col as f64 * pitch_x;
    let y = ((rows as f64 - 1.0) * pitch_y) / 2.0 - row as f64 * pitch_y;
    (x, y)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
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

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

fn status_lane_gap() -> f64 {
    (QUARANTINE_LANE_X - RELEASED_LANE_X).abs() - STATUS_LANE_SLOT_X
}

fn bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

fn camera_bar_clearance_above_deck() -> f64 {
    CAMERA_BAR_UNDERSIDE_Z
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn rect(center: (f64, f64), w: f64, h: f64) -> Rect {
    Rect {
        x: center.0,
        y: center.1,
        w,
        h,
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

fn vertical_gap(a: Rect, b: Rect) -> f64 {
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    if ay1 < by0 {
        by0 - ay1
    } else if by1 < ay0 {
        ay0 - by1
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped_to_station() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/run_record_material_scan_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_covers_required_ebr_material_controls() {
        assert_eq!(REQUIRED_EBR_FEATURES.len(), 9);
        assert!(REQUIRED_EBR_FEATURES.contains(&"barcode_rfid_scanner_bridge"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"reagent_lot_staging_pockets"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"media_lot_staging_pockets"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"chip_lot_staging_pockets"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"connector_lot_staging_pockets"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"released_quarantine_status_lanes"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"calibration_standard_scan_pockets"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"rejected_mismatch_pocket"));
        assert!(REQUIRED_EBR_FEATURES.contains(&"weigh_scale_load_cell_placeholder"));
    }

    #[test]
    fn material_pocket_counts_match_batch_record_inputs() {
        assert_eq!(REAGENT_POCKET_COUNT, 8);
        assert_eq!(MEDIA_POCKET_COUNT, 6);
        assert_eq!(CHIP_POCKET_COUNT, 4);
        assert_eq!(CONNECTOR_POCKET_COUNT, 12);
        assert_eq!(
            TOTAL_LOT_POCKETS,
            REAGENT_POCKET_COUNT + MEDIA_POCKET_COUNT + CHIP_POCKET_COUNT + CONNECTOR_POCKET_COUNT
        );
        assert!(TOTAL_LOT_POCKETS >= 30);
    }

    #[test]
    fn main_modules_fit_on_cleanable_deck_without_overlap() {
        assert!(fits_on_deck(STAGING_CENTER, STAGING_X, STAGING_Y, 10.0));
        assert!(fits_on_deck(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y, 10.0));
        assert!(fits_on_deck(CAL_CENTER, CAL_PANEL_X, CAL_PANEL_Y, 10.0));
        assert!(fits_on_deck(SCALE_CENTER, SCALE_X, SCALE_Y, 10.0));
        assert!(fits_on_deck(REJECT_CENTER, REJECT_X, REJECT_Y, 10.0));

        let staging = rect(STAGING_CENTER, STAGING_X, STAGING_Y);
        let lanes = rect(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y);
        let calibration = rect(CAL_CENTER, CAL_PANEL_X, CAL_PANEL_Y);
        let scale = rect(SCALE_CENTER, SCALE_X, SCALE_Y);
        let reject = rect(REJECT_CENTER, REJECT_X, REJECT_Y);
        assert!(!rects_overlap(staging, lanes));
        assert!(!rects_overlap(staging, calibration));
        assert!(!rects_overlap(lanes, reject));
        assert!(!rects_overlap(calibration, scale));
        assert!(!rects_overlap(scale, reject));
    }

    #[test]
    fn released_quarantine_and_reject_paths_are_segregated() {
        assert_eq!(RELEASED_LANE_COUNT, QUARANTINE_LANE_COUNT);
        assert_eq!(RELEASED_LANE_COUNT + QUARANTINE_LANE_COUNT, 10);
        assert!(status_lane_gap() >= STATUS_LANE_SEGREGATION_MIN);
        assert!(REJECT_WALL >= 8.0);

        let reject = rect(REJECT_CENTER, REJECT_X, REJECT_Y);
        let lanes = rect(LANE_CENTER, LANE_PANEL_X, LANE_PANEL_Y);
        assert!(!rects_overlap(reject, lanes));
        assert!(vertical_gap(reject, lanes) >= REJECT_SEGREGATION_MIN);
    }

    #[test]
    fn scan_bridge_camera_bar_and_scale_clear_robot_access() {
        assert!(bridge_clearance_above_deck() >= 125.0);
        assert!(camera_bar_clearance_above_deck() >= 150.0);
        assert!(BRIDGE_SPAN_X > STAGING_X + 80.0);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LED_SEGMENTS, 8);
        assert!(SCALE_PAN_D < SCALE_X - 42.0);
        assert!(LOAD_CELL_BEAM_X < SCALE_X - 40.0);
    }

    #[test]
    fn staging_grid_extents_remain_inside_tray_rims() {
        assert!(grid_fits(
            REAGENT_POCKET_COUNT,
            REAGENT_COLS,
            REAGENT_POCKET_X,
            REAGENT_POCKET_Y,
            REAGENT_PITCH_X,
            REAGENT_PITCH_Y,
            REAGENT_ORIGIN
        ));
        assert!(grid_fits(
            MEDIA_POCKET_COUNT,
            MEDIA_COLS,
            MEDIA_POCKET_X,
            MEDIA_POCKET_Y,
            MEDIA_PITCH_X,
            MEDIA_PITCH_Y,
            MEDIA_ORIGIN
        ));
        assert!(grid_fits(
            CHIP_POCKET_COUNT,
            CHIP_COLS,
            CHIP_POCKET_X,
            CHIP_POCKET_Y,
            CHIP_PITCH_X,
            CHIP_PITCH_Y,
            CHIP_ORIGIN
        ));
        assert!(grid_fits(
            CONNECTOR_POCKET_COUNT,
            CONNECTOR_COLS,
            CONNECTOR_POCKET_X,
            CONNECTOR_POCKET_Y,
            CONNECTOR_PITCH_X,
            CONNECTOR_PITCH_Y,
            CONNECTOR_ORIGIN
        ));
    }

    fn grid_fits(
        count: usize,
        cols: usize,
        pocket_x: f64,
        pocket_y: f64,
        pitch_x: f64,
        pitch_y: f64,
        origin: (f64, f64),
    ) -> bool {
        for index in 0..count {
            let (x, y) = grid_position(index, count, cols, pitch_x, pitch_y);
            let abs_x = origin.0 + x;
            let abs_y = origin.1 + y;
            if abs_x.abs() + pocket_x / 2.0 > STAGING_X / 2.0 - STAGING_RIM_W - 8.0 {
                return false;
            }
            if abs_y.abs() + pocket_y / 2.0 > STAGING_Y / 2.0 - STAGING_RIM_W - 8.0 {
                return false;
            }
        }
        true
    }
}
