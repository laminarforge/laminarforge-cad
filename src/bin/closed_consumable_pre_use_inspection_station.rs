use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed consumable pre-use inspection station for the automated tissue-chip workcell.
//
// Research assumptions used for this concept geometry:
// - Single-use assemblies are commonly checked at point of use with identity,
//   packaging/seal inspection, and pressure or vacuum decay style handoffs.
// - Sterile barrier systems can look intact while having small channels or leaks,
//   so the station exposes both optical inspection and a small integrity-test
//   fixture rather than relying on visual checks alone.
// - Released/hold/reject segregation, barcode/RFID identity capture, and sample
//   coupon staging should be physically obvious before consumables enter the cell.
//
// This is product-concept mechanical CAD only. It is not a sterility, package
// integrity, cleanroom, GMP, ISO, ASTM, or validation claim.

const OUTPUTS: [&str; 11] = [
    "output/closed_consumable_pre_use_inspection_station_base_leak_tray.stl",
    "output/closed_consumable_pre_use_inspection_station_clean_incoming_kit_datum.stl",
    "output/closed_consumable_pre_use_inspection_station_optical_inspection_window_camera_bridge.stl",
    "output/closed_consumable_pre_use_inspection_station_barcode_rfid_identity_lands.stl",
    "output/closed_consumable_pre_use_inspection_station_seal_pouch_integrity_fixture.stl",
    "output/closed_consumable_pre_use_inspection_station_released_hold_reject_lanes.stl",
    "output/closed_consumable_pre_use_inspection_station_leak_test_pressure_decay_handoff_ports.stl",
    "output/closed_consumable_pre_use_inspection_station_particle_wipe_coupon_staging.stl",
    "output/closed_consumable_pre_use_inspection_station_robot_pick_datums.stl",
    "output/closed_consumable_pre_use_inspection_station_clean_used_separation_service_keepouts.stl",
    "output/closed_consumable_pre_use_inspection_station_assembly.stl",
];

const DECK_X: f64 = 1360.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 20.0;
const DECK_RIM_W: f64 = 22.0;
const DECK_RIM_Z: f64 = 36.0;
const SUMP_X: f64 = 1210.0;
const SUMP_Y: f64 = 710.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;

const CLEAN_DATUM_CENTER: (f64, f64) = (-425.0, 130.0);
const CLEAN_DATUM_X: f64 = 450.0;
const CLEAN_DATUM_Y: f64 = 536.0;
const CLEAN_DATUM_Z: f64 = 42.0;
const TUBING_KIT_SLOTS: usize = 3;
const TUBING_SLOT_X: f64 = 364.0;
const TUBING_SLOT_Y: f64 = 38.0;
const MANIFOLD_POCKETS: usize = 4;
const FILTER_SADDLES: usize = 6;
const CAP_WELLS: usize = 32;
const CHIP_CASSETTE_NESTS: usize = 2;
const ARCHIVE_ARTICLE_POCKETS: usize = 4;

const OPTICAL_CENTER: (f64, f64) = (-48.0, 162.0);
const OPTICAL_FRAME_X: f64 = 720.0;
const OPTICAL_FRAME_Y: f64 = 306.0;
const OPTICAL_FRAME_Z: f64 = 22.0;
const INSPECTION_WINDOW_X: f64 = 610.0;
const INSPECTION_WINDOW_Y: f64 = 214.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 212.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_COUNT: usize = 4;
const CAMERA_PITCH_X: f64 = 150.0;
const LED_BAR_COUNT: usize = 2;

const IDENTITY_LANDS: usize = 14;
const RFID_LANDS: usize = 8;
const IDENTITY_LAND_X: f64 = 88.0;
const IDENTITY_LAND_Y: f64 = 30.0;
const RFID_LAND_X: f64 = 62.0;
const RFID_LAND_Y: f64 = 46.0;

const SEAL_FIXTURE_CENTER: (f64, f64) = (400.0, 180.0);
const SEAL_FIXTURE_X: f64 = 486.0;
const SEAL_FIXTURE_Y: f64 = 264.0;
const SEAL_FIXTURE_Z: f64 = 54.0;
const POUCH_BAYS: usize = 2;
const POUCH_BAY_X: f64 = 196.0;
const POUCH_BAY_Y: f64 = 158.0;
const SEAL_INSPECTION_RAILS: usize = 6;
const VACUUM_GASKET_LANES: usize = 4;

const STATUS_CENTER: (f64, f64) = (410.0, -226.0);
const STATUS_PANEL_X: f64 = 474.0;
const STATUS_PANEL_Y: f64 = 330.0;
const STATUS_PANEL_Z: f64 = 38.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 118.0;
const STATUS_SLOT_Y: f64 = 52.0;
const STATUS_LANE_PITCH_X: f64 = 146.0;
const STATUS_SLOT_PITCH_Y: f64 = 66.0;
const STATUS_DIVIDER_Z: f64 = 58.0;

const LEAK_HANDOFF_CENTER: (f64, f64) = (20.0, -302.0);
const LEAK_HANDOFF_X: f64 = 458.0;
const LEAK_HANDOFF_Y: f64 = 118.0;
const LEAK_HANDOFF_Z: f64 = 40.0;
const LEAK_PORT_COUNT: usize = 8;
const LEAK_PORT_D: f64 = 8.0;
const LEAK_PORT_PITCH_X: f64 = 48.0;
const PRESSURE_SENSOR_POCKETS: usize = 4;

const COUPON_CENTER: (f64, f64) = (-420.0, -274.0);
const COUPON_TRAY_X: f64 = 410.0;
const COUPON_TRAY_Y: f64 = 224.0;
const COUPON_TRAY_Z: f64 = 34.0;
const PARTICLE_COUPON_SLOTS: usize = 8;
const WIPE_COUPON_SLOTS: usize = 12;
const SWAB_TUBE_WELLS: usize = 6;

const ROBOT_PICK_PADS: usize = 12;
const DATUM_PIN_COUNT: usize = 6;
const FIDUCIAL_COUNT: usize = 10;
const PICK_PAD_D: f64 = 24.0;
const DATUM_PIN_D: f64 = 8.0;

const SEPARATION_WALL_X: f64 = 154.0;
const SEPARATION_WALL_Y: f64 = 690.0;
const SEPARATION_WALL_W: f64 = 24.0;
const SEPARATION_WALL_Z: f64 = 82.0;
const CLEAN_TO_USED_AIR_GAP: f64 = 36.0;
const FRONT_ROBOT_CLEARANCE: f64 = 460.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 224.0;
const CAMERA_TOP_CLEARANCE: f64 = 292.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(base_leak_tray(), OUTPUTS[0]);
    write_part(clean_incoming_kit_datum(), OUTPUTS[1]);
    write_part(optical_inspection_window_camera_bridge(), OUTPUTS[2]);
    write_part(barcode_rfid_identity_lands(), OUTPUTS[3]);
    write_part(seal_pouch_integrity_fixture(), OUTPUTS[4]);
    write_part(released_hold_reject_lanes(), OUTPUTS[5]);
    write_part(leak_test_pressure_decay_handoff_ports(), OUTPUTS[6]);
    write_part(particle_wipe_coupon_staging(), OUTPUTS[7]);
    write_part(robot_pick_datums(), OUTPUTS[8]);
    write_part(clean_used_separation_service_keepouts(), OUTPUTS[9]);
    write_part(station_assembly(), OUTPUTS[10]);

    println!(
        "Closed consumable pre-use inspection station: {:.0}mm x {:.0}mm deck, {} output STL parts, {} incoming consumable datum positions, {} barcode lands, {} RFID lands, {} pouch integrity bays, {} pressure-decay handoff ports, and {} released/hold/reject status slots.",
        DECK_X,
        DECK_Y,
        OUTPUTS.len(),
        incoming_consumable_positions(),
        IDENTITY_LANDS,
        RFID_LANDS,
        POUCH_BAYS,
        LEAK_PORT_COUNT,
        status_slot_count()
    );
    println!(
        "Inspection bridge: {:.0}mm x {:.0}mm window frame, {} camera pods, {} LED bars, {:.0}mm camera underside, {:.0}mm top service clearance.",
        OPTICAL_FRAME_X,
        OPTICAL_FRAME_Y,
        CAMERA_COUNT,
        LED_BAR_COUNT,
        CAMERA_BRIDGE_UNDERSIDE_Z,
        CAMERA_TOP_CLEARANCE
    );
    println!(
        "Workflow controls: {} particle coupons, {} wipe coupons, {} swab wells, {} robot pick pads, {} datum pins, clean/used wall at X={:.0}mm with {:.0}mm minimum air gap.",
        PARTICLE_COUPON_SLOTS,
        WIPE_COUPON_SLOTS,
        SWAB_TUBE_WELLS,
        ROBOT_PICK_PADS,
        DATUM_PIN_COUNT,
        SEPARATION_WALL_X,
        CLEAN_TO_USED_AIR_GAP
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray()
        + clean_incoming_kit_datum().translate(CLEAN_DATUM_CENTER.0, CLEAN_DATUM_CENTER.1, DECK_Z)
        + optical_inspection_window_camera_bridge().translate(
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
            DECK_Z,
        )
        + barcode_rfid_identity_lands().translate(0.0, 0.0, DECK_Z + 1.0)
        + seal_pouch_integrity_fixture().translate(
            SEAL_FIXTURE_CENTER.0,
            SEAL_FIXTURE_CENTER.1,
            DECK_Z,
        )
        + released_hold_reject_lanes().translate(STATUS_CENTER.0, STATUS_CENTER.1, DECK_Z)
        + leak_test_pressure_decay_handoff_ports().translate(
            LEAK_HANDOFF_CENTER.0,
            LEAK_HANDOFF_CENTER.1,
            DECK_Z,
        )
        + particle_wipe_coupon_staging().translate(COUPON_CENTER.0, COUPON_CENTER.1, DECK_Z)
        + robot_pick_datums().translate(0.0, 0.0, DECK_Z)
        + clean_used_separation_service_keepouts().translate(0.0, 0.0, DECK_Z)
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "consumable_pre_use_inspection_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let sump = centered_cube(
        "consumable_pre_use_inspection_recessed_leak_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -12.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        "consumable_pre_use_inspection_front_sump_drain",
        DRAIN_PORT_D / 2.0,
        DECK_RIM_W + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 90.0, -DECK_Y / 2.0 + 14.0, DECK_Z - 5.0);

    deck - sump - drain - deck_mount_holes()
        + deck_perimeter_curbs()
        + module_recess_ribs()
        + zone_witness_grooves()
}

fn deck_perimeter_curbs() -> Part {
    let front = centered_cube(
        "consumable_pre_use_inspection_front_leak_curb",
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
        "consumable_pre_use_inspection_rear_leak_curb",
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
        "consumable_pre_use_inspection_left_leak_curb",
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
        "consumable_pre_use_inspection_right_leak_curb",
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

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("consumable_pre_use_inspection_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("consumable_pre_use_inspection_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn module_recess_ribs() -> Part {
    let mut ribs = Part::empty("consumable_pre_use_inspection_module_recess_ribs");
    for (i, (center, x, y)) in [
        (
            CLEAN_DATUM_CENTER,
            CLEAN_DATUM_X + 18.0,
            CLEAN_DATUM_Y + 18.0,
        ),
        (
            OPTICAL_CENTER,
            OPTICAL_FRAME_X + 20.0,
            OPTICAL_FRAME_Y + 20.0,
        ),
        (
            SEAL_FIXTURE_CENTER,
            SEAL_FIXTURE_X + 18.0,
            SEAL_FIXTURE_Y + 18.0,
        ),
        (STATUS_CENTER, STATUS_PANEL_X + 18.0, STATUS_PANEL_Y + 18.0),
        (
            LEAK_HANDOFF_CENTER,
            LEAK_HANDOFF_X + 18.0,
            LEAK_HANDOFF_Y + 18.0,
        ),
        (COUPON_CENTER, COUPON_TRAY_X + 18.0, COUPON_TRAY_Y + 18.0),
    ]
    .iter()
    .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("consumable_pre_use_inspection_module_recess_front_rail_{i}"),
                *x,
                7.0,
                6.0,
            )
            .translate(center.0, center.1 - y / 2.0, DECK_Z + 3.0)
            + centered_cube(
                format!("consumable_pre_use_inspection_module_recess_rear_rail_{i}"),
                *x,
                7.0,
                6.0,
            )
            .translate(center.0, center.1 + y / 2.0, DECK_Z + 3.0)
            + centered_cube(
                format!("consumable_pre_use_inspection_module_recess_left_rail_{i}"),
                7.0,
                *y,
                6.0,
            )
            .translate(center.0 - x / 2.0, center.1, DECK_Z + 3.0)
            + centered_cube(
                format!("consumable_pre_use_inspection_module_recess_right_rail_{i}"),
                7.0,
                *y,
                6.0,
            )
            .translate(center.0 + x / 2.0, center.1, DECK_Z + 3.0);
    }
    ribs
}

fn zone_witness_grooves() -> Part {
    let clean_flow_arrow = centered_cube(
        "consumable_pre_use_inspection_clean_zone_flow_witness",
        420.0,
        8.0,
        5.0,
    )
    .translate(-360.0, -92.0, DECK_Z + 2.5);
    let used_flow_arrow = centered_cube(
        "consumable_pre_use_inspection_used_zone_flow_witness",
        364.0,
        8.0,
        5.0,
    )
    .translate(420.0, -62.0, DECK_Z + 2.5);
    let front_cleanout = centered_cube(
        "consumable_pre_use_inspection_front_cleanout_gutter",
        DECK_X - 180.0,
        10.0,
        5.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 58.0, DECK_Z + 2.5);

    clean_flow_arrow + used_flow_arrow + front_cleanout
}

fn clean_incoming_kit_datum() -> Part {
    let tray = centered_cube(
        "consumable_pre_use_clean_incoming_kit_tray",
        CLEAN_DATUM_X,
        CLEAN_DATUM_Y,
        CLEAN_DATUM_Z,
    )
    .translate(0.0, 0.0, CLEAN_DATUM_Z / 2.0);

    let basin = centered_cube(
        "consumable_pre_use_clean_incoming_recessed_basin",
        CLEAN_DATUM_X - 34.0,
        CLEAN_DATUM_Y - 34.0,
        9.0,
    )
    .translate(0.0, 0.0, CLEAN_DATUM_Z - 4.0);

    tray - basin - incoming_consumable_pocket_cuts()
        + incoming_tray_dividers()
        + incoming_datum_pins()
        + incoming_label_lands()
}

fn incoming_consumable_pocket_cuts() -> Part {
    tubing_kit_slot_cuts()
        + manifold_pocket_cuts()
        + filter_saddle_cuts()
        + cap_well_cuts()
        + chip_cassette_nest_cuts()
        + archive_article_pocket_cuts()
}

fn tubing_kit_slot_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_tubing_kit_slot_cuts");
    for i in 0..TUBING_KIT_SLOTS {
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_tubing_kit_slot_{i}"),
                TUBING_SLOT_X,
                TUBING_SLOT_Y,
                12.0,
                -4.0,
                202.0 - i as f64 * 62.0,
                CLEAN_DATUM_Z,
            );
    }
    cuts
}

fn manifold_pocket_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_manifold_pocket_cuts");
    for i in 0..MANIFOLD_POCKETS {
        let x = if i % 2 == 0 { -98.0 } else { 98.0 };
        let y = 16.0 - (i / 2) as f64 * 62.0;
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_single_use_manifold_pocket_{i}"),
                158.0,
                44.0,
                10.0,
                x,
                y,
                CLEAN_DATUM_Z,
            );
    }
    cuts
}

fn filter_saddle_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_filter_saddle_cuts");
    for i in 0..FILTER_SADDLES {
        let x = -168.0 + i as f64 * 67.2;
        cuts =
            cuts + top_cut(
                format!("consumable_pre_use_filter_saddle_slot_{i}"),
                42.0,
                28.0,
                9.0,
                x,
                -116.0,
                CLEAN_DATUM_Z,
            ) + centered_cylinder(
                format!("consumable_pre_use_filter_round_relief_{i}"),
                15.0,
                10.5,
                24,
            )
            .translate(x, -116.0, CLEAN_DATUM_Z - 4.5);
    }
    cuts
}

fn cap_well_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_cap_well_cuts");
    for i in 0..CAP_WELLS {
        let col = i % 8;
        let row = i / 8;
        let x = -126.0 + col as f64 * 36.0;
        let y = -172.0 - row as f64 * 28.0;
        cuts = cuts
            + centered_cylinder(
                format!("consumable_pre_use_cap_well_{i}"),
                14.0 / 2.0,
                10.0,
                24,
            )
            .translate(x, y, CLEAN_DATUM_Z - 4.5);
    }
    cuts
}

fn chip_cassette_nest_cuts() -> Part {
    let cassette_x = REVC_CHIP_LENGTH + 92.0;
    let cassette_y = REVC_CHIP_WIDTH + 28.0;
    let mut cuts = Part::empty("consumable_pre_use_chip_cassette_nest_cuts");
    for i in 0..CHIP_CASSETTE_NESTS {
        let x = if i == 0 { -96.0 } else { 96.0 };
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_chip_cassette_nest_{i}"),
                cassette_x,
                cassette_y,
                8.0,
                x,
                -60.0,
                CLEAN_DATUM_Z,
            );
    }
    cuts
}

fn archive_article_pocket_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_archive_article_pockets");
    for i in 0..ARCHIVE_ARTICLE_POCKETS {
        let x = -150.0 + i as f64 * 100.0;
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_sample_archive_article_pocket_{i}"),
                72.0,
                28.0,
                8.0,
                x,
                -242.0,
                CLEAN_DATUM_Z,
            );
    }
    cuts
}

fn incoming_tray_dividers() -> Part {
    let horizontal_1 = centered_cube(
        "consumable_pre_use_incoming_tubing_manifold_divider",
        CLEAN_DATUM_X - 54.0,
        8.0,
        22.0,
    )
    .translate(0.0, 62.0, CLEAN_DATUM_Z + 11.0);
    let horizontal_2 = centered_cube(
        "consumable_pre_use_incoming_manifold_filter_divider",
        CLEAN_DATUM_X - 54.0,
        8.0,
        22.0,
    )
    .translate(0.0, -92.0, CLEAN_DATUM_Z + 11.0);
    let vertical = centered_cube(
        "consumable_pre_use_incoming_archive_cap_divider",
        8.0,
        132.0,
        18.0,
    )
    .translate(0.0, -191.0, CLEAN_DATUM_Z + 9.0);

    horizontal_1 + horizontal_2 + vertical
}

fn incoming_datum_pins() -> Part {
    let mut pins = Part::empty("consumable_pre_use_incoming_datum_pin_bosses");
    for (i, (x, y)) in [
        (-194.0, 236.0),
        (194.0, 236.0),
        (-194.0, -236.0),
        (194.0, -236.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("consumable_pre_use_incoming_datum_boss_{i}"),
                12.0,
                8.0,
                32,
            )
            .translate(*x, *y, CLEAN_DATUM_Z + 4.0)
            - centered_cylinder(
                format!("consumable_pre_use_incoming_datum_pin_cut_{i}"),
                DATUM_PIN_D / 2.0,
                10.0,
                24,
            )
            .translate(*x, *y, CLEAN_DATUM_Z + 4.0);
    }
    pins
}

fn incoming_label_lands() -> Part {
    let mut lands = Part::empty("consumable_pre_use_incoming_article_label_lands");
    for i in 0..6 {
        lands = lands
            + centered_cube(
                format!("consumable_pre_use_incoming_article_label_land_{i}"),
                74.0,
                18.0,
                3.0,
            )
            .translate(-174.0 + i as f64 * 70.0, 248.0, CLEAN_DATUM_Z + 1.5);
    }
    lands
}

fn optical_inspection_window_camera_bridge() -> Part {
    inspection_window_frame() + camera_bridge() + illumination_bars() + field_calibration_tokens()
}

fn inspection_window_frame() -> Part {
    let left = centered_cube(
        "consumable_pre_use_optical_window_left_frame",
        24.0,
        OPTICAL_FRAME_Y,
        OPTICAL_FRAME_Z,
    )
    .translate(-OPTICAL_FRAME_X / 2.0 + 12.0, 0.0, OPTICAL_FRAME_Z / 2.0);
    let right = centered_cube(
        "consumable_pre_use_optical_window_right_frame",
        24.0,
        OPTICAL_FRAME_Y,
        OPTICAL_FRAME_Z,
    )
    .translate(OPTICAL_FRAME_X / 2.0 - 12.0, 0.0, OPTICAL_FRAME_Z / 2.0);
    let front = centered_cube(
        "consumable_pre_use_optical_window_front_frame",
        OPTICAL_FRAME_X,
        24.0,
        OPTICAL_FRAME_Z,
    )
    .translate(0.0, -OPTICAL_FRAME_Y / 2.0 + 12.0, OPTICAL_FRAME_Z / 2.0);
    let rear = centered_cube(
        "consumable_pre_use_optical_window_rear_frame",
        OPTICAL_FRAME_X,
        24.0,
        OPTICAL_FRAME_Z,
    )
    .translate(0.0, OPTICAL_FRAME_Y / 2.0 - 12.0, OPTICAL_FRAME_Z / 2.0);

    let datum_glass_land = centered_cube(
        "consumable_pre_use_optical_window_clear_panel_envelope",
        INSPECTION_WINDOW_X,
        INSPECTION_WINDOW_Y,
        4.0,
    )
    .translate(0.0, 0.0, OPTICAL_FRAME_Z + 2.0);

    left + right + front + rear + datum_glass_land
}

fn camera_bridge() -> Part {
    let post_z = CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z;
    let left_post = centered_cube(
        "consumable_pre_use_camera_bridge_left_post",
        34.0,
        70.0,
        post_z,
    )
    .translate(-OPTICAL_FRAME_X / 2.0 + 44.0, 0.0, post_z / 2.0);
    let right_post = centered_cube(
        "consumable_pre_use_camera_bridge_right_post",
        34.0,
        70.0,
        post_z,
    )
    .translate(OPTICAL_FRAME_X / 2.0 - 44.0, 0.0, post_z / 2.0);
    let beam = centered_cube(
        "consumable_pre_use_camera_bridge_crossbeam",
        OPTICAL_FRAME_X - 68.0,
        42.0,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );

    let mut pods = Part::empty("consumable_pre_use_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = (i as f64 - (CAMERA_COUNT as f64 - 1.0) / 2.0) * CAMERA_PITCH_X;
        pods = pods
            + centered_cube(
                format!("consumable_pre_use_camera_pod_{i}"),
                58.0,
                50.0,
                34.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_UNDERSIDE_Z - 17.0)
            - centered_cylinder(
                format!("consumable_pre_use_camera_lens_cutout_{i}"),
                12.0,
                8.0,
                32,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_UNDERSIDE_Z - 34.0);
    }

    left_post + right_post + beam + pods
}

fn illumination_bars() -> Part {
    let left = centered_cube(
        "consumable_pre_use_left_low_angle_led_bar",
        INSPECTION_WINDOW_X,
        20.0,
        18.0,
    )
    .translate(0.0, -INSPECTION_WINDOW_Y / 2.0 - 22.0, 52.0);
    let right = centered_cube(
        "consumable_pre_use_right_low_angle_led_bar",
        INSPECTION_WINDOW_X,
        20.0,
        18.0,
    )
    .translate(0.0, INSPECTION_WINDOW_Y / 2.0 + 22.0, 52.0);
    left + right
}

fn field_calibration_tokens() -> Part {
    let mut tokens = Part::empty("consumable_pre_use_optical_field_calibration_tokens");
    for i in 0..6 {
        tokens = tokens
            + centered_cylinder(
                format!("consumable_pre_use_optical_gray_card_token_{i}"),
                10.0,
                3.0,
                28,
            )
            .translate(-250.0 + i as f64 * 100.0, -128.0, OPTICAL_FRAME_Z + 4.0);
    }
    tokens
}

fn barcode_rfid_identity_lands() -> Part {
    let mut lands = Part::empty("consumable_pre_use_identity_lands");
    for i in 0..IDENTITY_LANDS {
        let (x, y) = barcode_land_position(i);
        lands = lands
            + centered_cube(
                format!("consumable_pre_use_barcode_lot_land_{i}"),
                IDENTITY_LAND_X,
                IDENTITY_LAND_Y,
                4.0,
            )
            .translate(x, y, 2.0);
    }
    for i in 0..RFID_LANDS {
        let (x, y) = rfid_land_position(i);
        lands = lands
            + centered_cube(
                format!("consumable_pre_use_rfid_antenna_land_{i}"),
                RFID_LAND_X,
                RFID_LAND_Y,
                4.0,
            )
            .translate(x, y, 2.0)
            + centered_cube(
                format!("consumable_pre_use_rfid_antenna_slot_{i}"),
                RFID_LAND_X - 18.0,
                5.0,
                3.0,
            )
            .translate(x, y, 5.5);
    }
    lands
}

fn seal_pouch_integrity_fixture() -> Part {
    let platen = centered_cube(
        "consumable_pre_use_seal_integrity_fixture_platen",
        SEAL_FIXTURE_X,
        SEAL_FIXTURE_Y,
        SEAL_FIXTURE_Z,
    )
    .translate(0.0, 0.0, SEAL_FIXTURE_Z / 2.0);

    platen - pouch_bay_recess_cuts() - vacuum_port_cuts()
        + pouch_gasket_lands()
        + seal_inspection_rails()
        + pouch_clamp_fingers()
        + pressure_reference_blocks()
}

fn pouch_bay_recess_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_pouch_bay_recess_cuts");
    for i in 0..POUCH_BAYS {
        let x = if i == 0 { -118.0 } else { 118.0 };
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_pouch_vacuum_bay_{i}"),
                POUCH_BAY_X,
                POUCH_BAY_Y,
                12.0,
                x,
                12.0,
                SEAL_FIXTURE_Z,
            );
    }
    cuts
}

fn vacuum_port_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_pouch_vacuum_port_cuts");
    for i in 0..POUCH_BAYS {
        let x = if i == 0 { -118.0 } else { 118.0 };
        cuts = cuts
            + centered_cylinder(
                format!("consumable_pre_use_pouch_vacuum_port_{i}"),
                8.0,
                SEAL_FIXTURE_Y + 10.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -SEAL_FIXTURE_Y / 2.0 + 12.0, SEAL_FIXTURE_Z - 14.0);
    }
    cuts
}

fn pouch_gasket_lands() -> Part {
    let mut lands = Part::empty("consumable_pre_use_pouch_gasket_lands");
    for i in 0..POUCH_BAYS {
        let x = if i == 0 { -118.0 } else { 118.0 };
        lands = lands + rectangular_gasket(format!("consumable_pre_use_pouch_gasket_{i}"), x, 12.0);
    }
    lands
}

fn rectangular_gasket(name: String, center_x: f64, center_y: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), POUCH_BAY_X, 6.0, 5.0).translate(
        center_x,
        center_y + POUCH_BAY_Y / 2.0,
        SEAL_FIXTURE_Z + 2.5,
    );
    let bottom = centered_cube(format!("{name}_bottom"), POUCH_BAY_X, 6.0, 5.0).translate(
        center_x,
        center_y - POUCH_BAY_Y / 2.0,
        SEAL_FIXTURE_Z + 2.5,
    );
    let left = centered_cube(format!("{name}_left"), 6.0, POUCH_BAY_Y, 5.0).translate(
        center_x - POUCH_BAY_X / 2.0,
        center_y,
        SEAL_FIXTURE_Z + 2.5,
    );
    let right = centered_cube(format!("{name}_right"), 6.0, POUCH_BAY_Y, 5.0).translate(
        center_x + POUCH_BAY_X / 2.0,
        center_y,
        SEAL_FIXTURE_Z + 2.5,
    );
    top + bottom + left + right
}

fn seal_inspection_rails() -> Part {
    let mut rails = Part::empty("consumable_pre_use_seal_visual_inspection_rails");
    for i in 0..SEAL_INSPECTION_RAILS {
        rails = rails
            + centered_cube(
                format!("consumable_pre_use_seal_inspection_rail_{i}"),
                SEAL_FIXTURE_X - 72.0,
                5.0,
                8.0,
            )
            .translate(
                0.0,
                -SEAL_FIXTURE_Y / 2.0 + 36.0 + i as f64 * 36.0,
                SEAL_FIXTURE_Z + 4.0,
            );
    }
    rails
}

fn pouch_clamp_fingers() -> Part {
    let mut fingers = Part::empty("consumable_pre_use_pouch_clamp_fingers");
    for i in 0..VACUUM_GASKET_LANES {
        fingers = fingers
            + centered_cube(
                format!("consumable_pre_use_pouch_clamp_finger_{i}"),
                30.0,
                118.0,
                18.0,
            )
            .translate(-176.0 + i as f64 * 117.0, -96.0, SEAL_FIXTURE_Z + 9.0);
    }
    fingers
}

fn pressure_reference_blocks() -> Part {
    let mut blocks = Part::empty("consumable_pre_use_pressure_reference_blocks");
    for i in 0..PRESSURE_SENSOR_POCKETS {
        blocks = blocks
            + centered_cube(
                format!("consumable_pre_use_pressure_reference_pocket_{i}"),
                58.0,
                28.0,
                16.0,
            )
            .translate(
                -174.0 + i as f64 * 116.0,
                SEAL_FIXTURE_Y / 2.0 - 26.0,
                SEAL_FIXTURE_Z + 8.0,
            );
    }
    blocks
}

fn released_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "consumable_pre_use_status_lane_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(0.0, 0.0, STATUS_PANEL_Z / 2.0);

    panel - status_slot_cuts()
        + status_lane_dividers()
        + lane_status_flag_tabs()
        + reject_high_lip()
}

fn status_slot_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_status_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = (lane as f64 - 1.0) * STATUS_LANE_PITCH_X;
            let y = (slot as f64 - 1.5) * STATUS_SLOT_PITCH_Y;
            cuts = cuts
                + top_cut(
                    format!("consumable_pre_use_status_lane_{lane}_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    11.0,
                    x,
                    y,
                    STATUS_PANEL_Z,
                );
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let left = centered_cube(
        "consumable_pre_use_released_hold_lane_divider",
        10.0,
        STATUS_PANEL_Y - 28.0,
        STATUS_DIVIDER_Z,
    )
    .translate(
        -STATUS_LANE_PITCH_X / 2.0,
        0.0,
        STATUS_PANEL_Z + STATUS_DIVIDER_Z / 2.0,
    );
    let right = centered_cube(
        "consumable_pre_use_hold_reject_lane_divider",
        10.0,
        STATUS_PANEL_Y - 28.0,
        STATUS_DIVIDER_Z,
    )
    .translate(
        STATUS_LANE_PITCH_X / 2.0,
        0.0,
        STATUS_PANEL_Z + STATUS_DIVIDER_Z / 2.0,
    );
    left + right
}

fn lane_status_flag_tabs() -> Part {
    let mut tabs = Part::empty("consumable_pre_use_status_flag_tabs");
    for lane in 0..STATUS_LANES {
        let x = (lane as f64 - 1.0) * STATUS_LANE_PITCH_X;
        tabs = tabs
            + centered_cube(
                format!("consumable_pre_use_status_lane_flag_tab_{lane}"),
                104.0,
                22.0,
                14.0,
            )
            .translate(x, STATUS_PANEL_Y / 2.0 - 24.0, STATUS_PANEL_Z + 7.0);
    }
    tabs
}

fn reject_high_lip() -> Part {
    centered_cube(
        "consumable_pre_use_reject_lane_high_lip",
        STATUS_SLOT_X + 30.0,
        STATUS_PANEL_Y - 44.0,
        34.0,
    )
    .translate(STATUS_LANE_PITCH_X, 0.0, STATUS_PANEL_Z + 17.0)
}

fn leak_test_pressure_decay_handoff_ports() -> Part {
    let manifold = centered_cube(
        "consumable_pre_use_pressure_decay_handoff_manifold",
        LEAK_HANDOFF_X,
        LEAK_HANDOFF_Y,
        LEAK_HANDOFF_Z,
    )
    .translate(0.0, 0.0, LEAK_HANDOFF_Z / 2.0);

    manifold - leak_port_cuts()
        + leak_port_collars()
        + pressure_decay_sensor_blocks()
        + tubing_comb_guides()
}

fn leak_port_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_leak_handoff_port_cuts");
    for i in 0..LEAK_PORT_COUNT {
        let x = leak_port_x(i);
        cuts = cuts
            + centered_cylinder(
                format!("consumable_pre_use_pressure_decay_port_bore_{i}"),
                LEAK_PORT_D / 2.0,
                LEAK_HANDOFF_Z + 8.0,
                28,
            )
            .translate(x, 18.0, LEAK_HANDOFF_Z / 2.0);
    }
    cuts
}

fn leak_port_collars() -> Part {
    let mut collars = Part::empty("consumable_pre_use_leak_handoff_port_collars");
    for i in 0..LEAK_PORT_COUNT {
        let x = leak_port_x(i);
        collars = collars
            + centered_cylinder(
                format!("consumable_pre_use_pressure_decay_port_collar_{i}"),
                14.0,
                8.0,
                32,
            )
            .translate(x, 18.0, LEAK_HANDOFF_Z + 4.0);
    }
    collars
}

fn pressure_decay_sensor_blocks() -> Part {
    let mut blocks = Part::empty("consumable_pre_use_pressure_decay_sensor_blocks");
    for i in 0..PRESSURE_SENSOR_POCKETS {
        blocks = blocks
            + centered_cube(
                format!("consumable_pre_use_pressure_decay_sensor_pocket_{i}"),
                74.0,
                30.0,
                20.0,
            )
            .translate(-156.0 + i as f64 * 104.0, -34.0, LEAK_HANDOFF_Z + 10.0);
    }
    blocks
}

fn tubing_comb_guides() -> Part {
    let mut combs = Part::empty("consumable_pre_use_leak_handoff_tubing_combs");
    for i in 0..LEAK_PORT_COUNT {
        let x = leak_port_x(i);
        combs = combs
            + centered_cube(
                format!("consumable_pre_use_leak_handoff_tubing_comb_{i}"),
                14.0,
                46.0,
                16.0,
            )
            .translate(x, LEAK_HANDOFF_Y / 2.0 - 24.0, LEAK_HANDOFF_Z + 8.0);
    }
    combs
}

fn particle_wipe_coupon_staging() -> Part {
    let tray = centered_cube(
        "consumable_pre_use_particle_wipe_coupon_tray",
        COUPON_TRAY_X,
        COUPON_TRAY_Y,
        COUPON_TRAY_Z,
    )
    .translate(0.0, 0.0, COUPON_TRAY_Z / 2.0);

    tray - coupon_slot_cuts()
        + coupon_retainer_tabs()
        + swab_tube_well_collars()
        + coupon_chain_of_custody_lands()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("consumable_pre_use_coupon_slot_cuts");
    for i in 0..PARTICLE_COUPON_SLOTS {
        let col = i % 4;
        let row = i / 4;
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_particle_coupon_slot_{i}"),
                72.0,
                28.0,
                8.0,
                -126.0 + col as f64 * 84.0,
                58.0 - row as f64 * 42.0,
                COUPON_TRAY_Z,
            );
    }
    for i in 0..WIPE_COUPON_SLOTS {
        let col = i % 6;
        let row = i / 6;
        cuts = cuts
            + top_cut(
                format!("consumable_pre_use_wipe_coupon_slot_{i}"),
                48.0,
                24.0,
                7.0,
                -132.0 + col as f64 * 54.0,
                -48.0 - row as f64 * 38.0,
                COUPON_TRAY_Z,
            );
    }
    for i in 0..SWAB_TUBE_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("consumable_pre_use_swab_tube_well_{i}"),
                12.0,
                9.0,
                24,
            )
            .translate(-150.0 + i as f64 * 60.0, -96.0, COUPON_TRAY_Z - 4.0);
    }
    cuts
}

fn coupon_retainer_tabs() -> Part {
    let mut tabs = Part::empty("consumable_pre_use_coupon_retainer_tabs");
    for i in 0..4 {
        tabs = tabs
            + centered_cube(
                format!("consumable_pre_use_coupon_retainer_tab_{i}"),
                38.0,
                12.0,
                10.0,
            )
            .translate(-162.0 + i as f64 * 108.0, 96.0, COUPON_TRAY_Z + 5.0);
    }
    tabs
}

fn swab_tube_well_collars() -> Part {
    let mut collars = Part::empty("consumable_pre_use_swab_tube_well_collars");
    for i in 0..SWAB_TUBE_WELLS {
        collars = collars
            + centered_cylinder(
                format!("consumable_pre_use_swab_tube_well_collar_{i}"),
                16.0,
                5.0,
                28,
            )
            .translate(-150.0 + i as f64 * 60.0, -96.0, COUPON_TRAY_Z + 2.5);
    }
    collars
}

fn coupon_chain_of_custody_lands() -> Part {
    let mut lands = Part::empty("consumable_pre_use_coupon_chain_of_custody_lands");
    for i in 0..6 {
        lands = lands
            + centered_cube(
                format!("consumable_pre_use_coupon_custody_land_{i}"),
                54.0,
                18.0,
                4.0,
            )
            .translate(
                -150.0 + i as f64 * 60.0,
                COUPON_TRAY_Y / 2.0 - 20.0,
                COUPON_TRAY_Z + 2.0,
            );
    }
    lands
}

fn robot_pick_datums() -> Part {
    let mut datums = Part::empty("consumable_pre_use_robot_pick_datums");
    for (i, (x, y)) in robot_pick_pad_positions().iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("consumable_pre_use_robot_pick_pad_{i}"),
                PICK_PAD_D / 2.0,
                6.0,
                32,
            )
            .translate(*x, *y, 3.0);
    }
    for (i, (x, y)) in datum_pin_positions().iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("consumable_pre_use_robot_datum_pin_boss_{i}"),
                13.0,
                10.0,
                32,
            )
            .translate(*x, *y, 5.0)
            - centered_cylinder(
                format!("consumable_pre_use_robot_datum_pin_bore_{i}"),
                DATUM_PIN_D / 2.0,
                12.0,
                24,
            )
            .translate(*x, *y, 5.0);
    }
    for i in 0..FIDUCIAL_COUNT {
        let (x, y) = fiducial_position(i);
        datums = datums
            + centered_cylinder(
                format!("consumable_pre_use_vision_fiducial_{i}"),
                6.0,
                2.0,
                24,
            )
            .translate(x, y, 1.0);
    }
    datums
}

fn clean_used_separation_service_keepouts() -> Part {
    clean_used_wall()
        + clean_to_used_air_gap_gauge()
        + service_keepout_gauges()
        + camera_top_clearance_gauge()
}

fn clean_used_wall() -> Part {
    let lower = centered_cube(
        "consumable_pre_use_clean_used_separation_lower_wall",
        SEPARATION_WALL_W,
        250.0,
        SEPARATION_WALL_Z,
    )
    .translate(SEPARATION_WALL_X, -244.0, SEPARATION_WALL_Z / 2.0);
    let upper = centered_cube(
        "consumable_pre_use_clean_used_separation_upper_wall",
        SEPARATION_WALL_W,
        255.0,
        SEPARATION_WALL_Z,
    )
    .translate(SEPARATION_WALL_X, 257.5, SEPARATION_WALL_Z / 2.0);
    let bridge_marker = centered_cube(
        "consumable_pre_use_clean_used_controlled_transfer_gap_marker",
        72.0,
        32.0,
        28.0,
    )
    .translate(SEPARATION_WALL_X, 0.0, 14.0);

    lower + upper + bridge_marker
}

fn clean_to_used_air_gap_gauge() -> Part {
    centered_cube(
        "consumable_pre_use_clean_to_used_air_gap_gauge",
        CLEAN_TO_USED_AIR_GAP,
        SEPARATION_WALL_Y,
        6.0,
    )
    .translate(
        SEPARATION_WALL_X + CLEAN_TO_USED_AIR_GAP / 2.0 + 18.0,
        0.0,
        3.0,
    )
}

fn service_keepout_gauges() -> Part {
    let front = centered_cube(
        "consumable_pre_use_front_robot_approach_keepout",
        DECK_X - 120.0,
        FRONT_ROBOT_CLEARANCE,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0, 4.0);
    let rear = centered_cube(
        "consumable_pre_use_rear_service_keepout",
        DECK_X - 180.0,
        REAR_SERVICE_CLEARANCE,
        8.0,
    )
    .translate(20.0, DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 4.0);
    let right = centered_cube(
        "consumable_pre_use_right_integrity_fixture_service_keepout",
        RIGHT_SERVICE_CLEARANCE,
        DECK_Y - 140.0,
        8.0,
    )
    .translate(DECK_X / 2.0 + RIGHT_SERVICE_CLEARANCE / 2.0, 20.0, 4.0);

    front + rear + right
}

fn camera_top_clearance_gauge() -> Part {
    let z = CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z + CAMERA_TOP_CLEARANCE / 2.0;
    centered_cube(
        "consumable_pre_use_camera_top_service_clearance_gauge",
        OPTICAL_FRAME_X - 100.0,
        70.0,
        CAMERA_TOP_CLEARANCE,
    )
    .translate(OPTICAL_CENTER.0, OPTICAL_CENTER.1, z)
}

fn top_cut(name: impl Into<String>, x: f64, y: f64, depth: f64, cx: f64, cy: f64, z: f64) -> Part {
    centered_cube(name, x, y, depth + 0.4).translate(cx, cy, z - depth / 2.0 + 0.2)
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 82.0, -DECK_Y / 2.0 + 82.0),
        (0.0, -DECK_Y / 2.0 + 82.0),
        (DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 82.0),
        (-DECK_X / 2.0 + 82.0, 0.0),
        (DECK_X / 2.0 - 82.0, 0.0),
        (-DECK_X / 2.0 + 82.0, DECK_Y / 2.0 - 82.0),
        (0.0, DECK_Y / 2.0 - 82.0),
        (DECK_X / 2.0 - 82.0, DECK_Y / 2.0 - 82.0),
    ]
}

fn barcode_land_position(index: usize) -> (f64, f64) {
    match index {
        0..=5 => (-626.0 + index as f64 * 102.0, 372.0),
        6..=9 => (316.0 + (index - 6) as f64 * 98.0, 358.0),
        _ => (-616.0 + (index - 10) as f64 * 112.0, -382.0),
    }
}

fn rfid_land_position(index: usize) -> (f64, f64) {
    if index < 4 {
        (-620.0 + index as f64 * 122.0, 318.0)
    } else {
        (292.0 + (index - 4) as f64 * 112.0, 310.0)
    }
}

fn leak_port_x(index: usize) -> f64 {
    (index as f64 - (LEAK_PORT_COUNT as f64 - 1.0) / 2.0) * LEAK_PORT_PITCH_X
}

fn robot_pick_pad_positions() -> [(f64, f64); ROBOT_PICK_PADS] {
    [
        (-614.0, 342.0),
        (-430.0, 342.0),
        (-246.0, 342.0),
        (-614.0, -98.0),
        (-430.0, -98.0),
        (-246.0, -98.0),
        (238.0, 326.0),
        (442.0, 326.0),
        (596.0, 326.0),
        (238.0, -382.0),
        (442.0, -382.0),
        (596.0, -382.0),
    ]
}

fn datum_pin_positions() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-610.0, 226.0),
        (-250.0, 226.0),
        (-610.0, -336.0),
        (232.0, 34.0),
        (636.0, 34.0),
        (636.0, -346.0),
    ]
}

fn fiducial_position(index: usize) -> (f64, f64) {
    let positions = [
        (-632.0, 392.0),
        (-232.0, 392.0),
        (-632.0, -392.0),
        (-232.0, -392.0),
        (-246.0, 112.0),
        (118.0, 112.0),
        (238.0, 384.0),
        (620.0, 384.0),
        (238.0, -390.0),
        (620.0, -390.0),
    ];
    positions[index]
}

fn incoming_consumable_positions() -> usize {
    TUBING_KIT_SLOTS
        + MANIFOLD_POCKETS
        + FILTER_SADDLES
        + CAP_WELLS
        + CHIP_CASSETTE_NESTS
        + ARCHIVE_ARTICLE_POCKETS
}

fn status_slot_count() -> usize {
    STATUS_LANES * STATUS_SLOTS_PER_LANE
}

fn assert_layout() {
    assert!(fits_on_deck(
        CLEAN_DATUM_CENTER,
        CLEAN_DATUM_X,
        CLEAN_DATUM_Y,
        28.0
    ));
    assert!(fits_on_deck(
        OPTICAL_CENTER,
        OPTICAL_FRAME_X,
        OPTICAL_FRAME_Y,
        28.0
    ));
    assert!(fits_on_deck(
        SEAL_FIXTURE_CENTER,
        SEAL_FIXTURE_X,
        SEAL_FIXTURE_Y,
        28.0
    ));
    assert!(fits_on_deck(
        STATUS_CENTER,
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        28.0
    ));
    assert!(fits_on_deck(
        LEAK_HANDOFF_CENTER,
        LEAK_HANDOFF_X,
        LEAK_HANDOFF_Y,
        28.0
    ));
    assert!(fits_on_deck(
        COUPON_CENTER,
        COUPON_TRAY_X,
        COUPON_TRAY_Y,
        28.0
    ));
    assert!(
        rect(CLEAN_DATUM_CENTER, CLEAN_DATUM_X, CLEAN_DATUM_Y).1
            < SEPARATION_WALL_X - CLEAN_TO_USED_AIR_GAP,
        "incoming clean kit datum crosses into used/reject side"
    );
    assert!(
        rect(STATUS_CENTER, STATUS_PANEL_X, STATUS_PANEL_Y).0
            > SEPARATION_WALL_X + CLEAN_TO_USED_AIR_GAP / 2.0,
        "status lanes are too close to the clean side boundary"
    );
    assert!(
        !rects_overlap(
            rect(CLEAN_DATUM_CENTER, CLEAN_DATUM_X, CLEAN_DATUM_Y),
            rect(SEAL_FIXTURE_CENTER, SEAL_FIXTURE_X, SEAL_FIXTURE_Y)
        ),
        "incoming datum and seal fixture collide"
    );
    assert!(
        CAMERA_BRIDGE_UNDERSIDE_Z > CLEAN_DATUM_Z + 120.0,
        "camera bridge does not clear staged consumables"
    );
    assert!(status_slot_count() >= 12);
    assert!(incoming_consumable_positions() >= 40);
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0 - x / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - y / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + y / 2.0 <= DECK_Y / 2.0 - margin
}

fn rect(center: (f64, f64), x: f64, y: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - x / 2.0,
        center.0 + x / 2.0,
        center.1 - y / 2.0,
        center.1 + y / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.1 && a.1 > b.0 && a.2 < b.3 && a.3 > b.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_named_station_parts_and_assembly() {
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/closed_consumable_pre_use_inspection_station_")));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn layout_keeps_major_modules_on_deck() {
        assert_layout();
    }

    #[test]
    fn incoming_datum_covers_required_consumable_types() {
        assert_eq!(TUBING_KIT_SLOTS, 3);
        assert_eq!(MANIFOLD_POCKETS, 4);
        assert_eq!(FILTER_SADDLES, 6);
        assert_eq!(CAP_WELLS, 32);
        assert_eq!(CHIP_CASSETTE_NESTS, 2);
        assert_eq!(ARCHIVE_ARTICLE_POCKETS, 4);
        assert_eq!(incoming_consumable_positions(), 51);
    }

    #[test]
    fn inspection_and_integrity_features_are_represented() {
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(LED_BAR_COUNT, 2);
        assert_eq!(POUCH_BAYS, 2);
        assert_eq!(SEAL_INSPECTION_RAILS, 6);
        assert_eq!(LEAK_PORT_COUNT, 8);
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z >= 200.0);
    }

    #[test]
    fn traceability_and_status_lanes_are_sized_for_batch_flow() {
        assert_eq!(IDENTITY_LANDS, 14);
        assert_eq!(RFID_LANDS, 8);
        assert_eq!(status_slot_count(), 12);
        assert_eq!(STATUS_LANES, 3);
        assert!(STATUS_LANE_PITCH_X - STATUS_SLOT_X > 24.0);
    }

    #[test]
    fn coupon_and_robot_datums_support_automated_handling() {
        assert_eq!(PARTICLE_COUPON_SLOTS, 8);
        assert_eq!(WIPE_COUPON_SLOTS, 12);
        assert_eq!(SWAB_TUBE_WELLS, 6);
        assert_eq!(ROBOT_PICK_PADS, 12);
        assert_eq!(DATUM_PIN_COUNT, 6);
        assert_eq!(FIDUCIAL_COUNT, 10);
    }

    #[test]
    fn clean_and_used_sides_have_explicit_physical_separation() {
        let clean_right = rect(CLEAN_DATUM_CENTER, CLEAN_DATUM_X, CLEAN_DATUM_Y).1;
        let status_left = rect(STATUS_CENTER, STATUS_PANEL_X, STATUS_PANEL_Y).0;
        assert!(SEPARATION_WALL_X - clean_right >= CLEAN_TO_USED_AIR_GAP);
        assert!(status_left - SEPARATION_WALL_X >= CLEAN_TO_USED_AIR_GAP / 2.0);
        assert!(SEPARATION_WALL_Z >= 80.0);
    }
}
