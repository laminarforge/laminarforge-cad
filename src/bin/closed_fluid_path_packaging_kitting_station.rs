use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed fluid-path packaging and kitting station for the automated tissue-chip workflow.
//
// Intent:
// - Kit sterile tubing harnesses, disposable manifold inserts, filters,
//   connectors, caps, and pouch/tote interfaces as traceable closed-path sets.
// - Keep clean, used, and rejected material physically separated on the deck.
// - Provide robot pick datums, scan lands, inspection camera clearance, and
//   leak-test-ready handoff ports before material enters the culture workcell.
//
// This is product-concept CAD only. It is not a packaging validation protocol,
// sterility claim, or biological handling procedure.

const OUTPUTS: &[&str] = &[
    "output/closed_fluid_path_packaging_kitting_station_deck_tray.stl",
    "output/closed_fluid_path_packaging_kitting_station_sterile_tubing_harness_kit_trays.stl",
    "output/closed_fluid_path_packaging_kitting_station_single_use_manifold_insert_pockets.stl",
    "output/closed_fluid_path_packaging_kitting_station_filter_connector_cap_staging.stl",
    "output/closed_fluid_path_packaging_kitting_station_barcode_rfid_lands.stl",
    "output/closed_fluid_path_packaging_kitting_station_clean_used_reject_segregation.stl",
    "output/closed_fluid_path_packaging_kitting_station_sealed_pouch_tote_interface.stl",
    "output/closed_fluid_path_packaging_kitting_station_leak_test_handoff_ports.stl",
    "output/closed_fluid_path_packaging_kitting_station_robot_pick_datums.stl",
    "output/closed_fluid_path_packaging_kitting_station_inspection_window_camera_bridge.stl",
    "output/closed_fluid_path_packaging_kitting_station_service_keepouts.stl",
    "output/closed_fluid_path_packaging_kitting_station_assembly.stl",
];

const DECK_X: f64 = 1240.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 38.0;

const CLEAN_ZONE_X: f64 = -330.0;
const WORK_ZONE_X: f64 = 30.0;
const STAGING_ZONE_X: f64 = 365.0;
const UPPER_ZONE_Y: f64 = 118.0;
const FRONT_ZONE_Y: f64 = -284.0;

const HARNESS_TRAY_COUNT: usize = 6;
const HARNESS_TRAY_X: f64 = 330.0;
const HARNESS_TRAY_Y: f64 = 246.0;
const HARNESS_TRAY_Z: f64 = 34.0;
const HARNESS_TUBE_OD: f64 = 6.35;
const HARNESS_TUBE_CLEARANCE: f64 = 1.0;

const MANIFOLD_POCKET_COUNT: usize = 8;
const MANIFOLD_POCKET_COLS: usize = 4;
const MANIFOLD_POCKET_ROWS: usize = 2;
const MANIFOLD_BLOCK_X: f64 = 344.0;
const MANIFOLD_BLOCK_Y: f64 = 244.0;
const MANIFOLD_BLOCK_Z: f64 = 42.0;

const FILTER_COUNT: usize = 8;
const CONNECTOR_COUNT: usize = 16;
const CAP_WELL_COUNT: usize = 32;
const STAGING_BLOCK_X: f64 = 332.0;
const STAGING_BLOCK_Y: f64 = 250.0;
const STAGING_BLOCK_Z: f64 = 40.0;

const BARCODE_LANDS: usize = 14;
const RFID_LANDS: usize = 8;
const LABEL_LAND_X: f64 = 96.0;
const LABEL_LAND_Y: f64 = 30.0;
const LABEL_LAND_Z: f64 = 5.0;

const CLEAN_LANE_W: f64 = 410.0;
const USED_LANE_W: f64 = 262.0;
const REJECT_LANE_W: f64 = 214.0;
const SEGREGATION_SPINE_Z: f64 = 92.0;
const STATUS_BIN_Y: f64 = 198.0;

const POUCH_DOCK_X: f64 = 520.0;
const POUCH_DOCK_Y: f64 = 118.0;
const POUCH_DOCK_Z: f64 = 36.0;
const TOTE_INTERFACE_X: f64 = 430.0;
const TOTE_INTERFACE_Y: f64 = 74.0;
const TOTE_INTERFACE_Z: f64 = 56.0;

const LEAK_PORT_COUNT: usize = 12;
const LEAK_PORT_BAR_X: f64 = 610.0;
const LEAK_PORT_BAR_Y: f64 = 78.0;
const LEAK_PORT_BAR_Z: f64 = 30.0;
const LEAK_PORT_D: f64 = 7.2;

const ROBOT_PICK_PADS: usize = 8;
const ROBOT_DATUM_PINS: usize = 6;
const PICK_PAD_D: f64 = 28.0;
const PICK_PAD_Z: f64 = 10.0;

const INSPECTION_WINDOW_X: f64 = 650.0;
const INSPECTION_WINDOW_Y: f64 = 278.0;
const CAMERA_BRIDGE_SPAN_X: f64 = 760.0;
const CAMERA_BRIDGE_POST_Y: f64 = 44.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 182.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 28.0;

const FRONT_ROBOT_KEEP_OUT: f64 = 420.0;
const REAR_SERVICE_KEEP_OUT: f64 = 240.0;
const SIDE_TOTE_SWING_KEEP_OUT: f64 = 180.0;
const CAMERA_SERVICE_KEEP_OUT_Z: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = deck_tray();
    export(&deck, OUTPUTS[0]);

    let harness = sterile_tubing_harness_kit_trays();
    export(&harness, OUTPUTS[1]);

    let manifolds = single_use_manifold_insert_pockets();
    export(&manifolds, OUTPUTS[2]);

    let staging = filter_connector_cap_staging();
    export(&staging, OUTPUTS[3]);

    let traceability = barcode_rfid_lands();
    export(&traceability, OUTPUTS[4]);

    let segregation = clean_used_reject_segregation();
    export(&segregation, OUTPUTS[5]);

    let pouch = sealed_pouch_tote_interface();
    export(&pouch, OUTPUTS[6]);

    let leak_ports = leak_test_handoff_ports();
    export(&leak_ports, OUTPUTS[7]);

    let robot_datums = robot_pick_datums();
    export(&robot_datums, OUTPUTS[8]);

    let inspection = inspection_window_camera_bridge();
    export(&inspection, OUTPUTS[9]);

    let keepouts = service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = deck
        + harness.translate(CLEAN_ZONE_X, UPPER_ZONE_Y, DECK_Z + HARNESS_TRAY_Z / 2.0)
        + manifolds.translate(WORK_ZONE_X, UPPER_ZONE_Y, DECK_Z + MANIFOLD_BLOCK_Z / 2.0)
        + staging.translate(STAGING_ZONE_X, UPPER_ZONE_Y, DECK_Z + STAGING_BLOCK_Z / 2.0)
        + traceability
        + segregation
        + pouch.translate(0.0, FRONT_ZONE_Y, DECK_Z + POUCH_DOCK_Z / 2.0)
        + leak_ports.translate(0.0, FRONT_ZONE_Y + 112.0, DECK_Z + LEAK_PORT_BAR_Z / 2.0)
        + robot_datums
        + inspection
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed fluid-path packaging/kitting station: {:.0} x {:.0} mm deck, {} harness trays, {} manifold pockets, {} filters, {} connectors, {} cap wells, {} leak-test ports.",
        DECK_X,
        DECK_Y,
        HARNESS_TRAY_COUNT,
        MANIFOLD_POCKET_COUNT,
        FILTER_COUNT,
        CONNECTOR_COUNT,
        CAP_WELL_COUNT,
        LEAK_PORT_COUNT
    );
    println!(
        "Traceability and inspection: {} barcode/lot lands, {} RFID lands, {:.0} x {:.0} mm inspection window, {:.0} mm bridge underside clearance.",
        BARCODE_LANDS,
        RFID_LANDS,
        INSPECTION_WINDOW_X,
        INSPECTION_WINDOW_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z
    );
    println!(
        "Keepouts: front robot {:.0} mm, rear service {:.0} mm, side tote swing {:.0} mm, camera service {:.0} mm Z.",
        FRONT_ROBOT_KEEP_OUT,
        REAR_SERVICE_KEEP_OUT,
        SIDE_TOTE_SWING_KEEP_OUT,
        CAMERA_SERVICE_KEEP_OUT_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_tray() -> Part {
    let deck = centered_cube("packaging_kitting_deck_pan", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    let basin = centered_cube(
        "packaging_kitting_recessed_spill_basin",
        DECK_X - 2.0 * (RIM_W + 38.0),
        DECK_Y - 2.0 * (RIM_W + 42.0),
        9.0,
    )
    .translate(0.0, 8.0, DECK_Z - 4.5);
    let drain_sump = centered_cube(
        "packaging_kitting_front_right_drain_sump",
        82.0,
        46.0,
        DECK_Z + 2.0,
    )
    .translate(DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 + 52.0, DECK_Z / 2.0);
    let drain_port = centered_cylinder("packaging_kitting_drain_port", 6.0 / 2.0, 38.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 + 20.0, DECK_Z - 8.0);

    deck - basin - drain_sump - drain_port - deck_mount_holes()
        + deck_rim()
        + deck_zone_floor_lands()
}

fn deck_rim() -> Part {
    let front = centered_cube("packaging_kitting_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("packaging_kitting_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("packaging_kitting_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("packaging_kitting_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("packaging_kitting_deck_mount_holes");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 42.0), -(DECK_Y / 2.0 - 42.0)),
        (DECK_X / 2.0 - 42.0, -(DECK_Y / 2.0 - 42.0)),
        (-(DECK_X / 2.0 - 42.0), DECK_Y / 2.0 - 42.0),
        (DECK_X / 2.0 - 42.0, DECK_Y / 2.0 - 42.0),
        (0.0, -(DECK_Y / 2.0 - 42.0)),
        (0.0, DECK_Y / 2.0 - 42.0),
        (-(DECK_X / 2.0 - 42.0), 0.0),
        (DECK_X / 2.0 - 42.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("packaging_kitting_deck_m6_mount_{i}"),
                6.6 / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn deck_zone_floor_lands() -> Part {
    let clean = centered_cube("packaging_kitting_clean_zone_floor_land", 402.0, 276.0, 4.0)
        .translate(CLEAN_ZONE_X, UPPER_ZONE_Y, DECK_Z + 2.0);
    let work = centered_cube("packaging_kitting_work_zone_floor_land", 338.0, 276.0, 4.0)
        .translate(WORK_ZONE_X, UPPER_ZONE_Y, DECK_Z + 2.0);
    let staging = centered_cube(
        "packaging_kitting_staging_zone_floor_land",
        340.0,
        276.0,
        4.0,
    )
    .translate(STAGING_ZONE_X, UPPER_ZONE_Y, DECK_Z + 2.0);
    let pouch = centered_cube("packaging_kitting_pouch_zone_floor_land", 620.0, 154.0, 4.0)
        .translate(0.0, FRONT_ZONE_Y, DECK_Z + 2.0);
    clean + work + staging + pouch
}

fn sterile_tubing_harness_kit_trays() -> Part {
    let body = centered_cube(
        "packaging_kitting_harness_kit_tray_body",
        HARNESS_TRAY_X,
        HARNESS_TRAY_Y,
        HARNESS_TRAY_Z,
    );
    let mut cuts = Part::empty("packaging_kitting_harness_tray_cuts");
    let mut ribs = Part::empty("packaging_kitting_harness_tray_ribs");
    let channel_d = HARNESS_TUBE_OD + HARNESS_TUBE_CLEARANCE;

    for i in 0..HARNESS_TRAY_COUNT {
        let y = lane_position(i, HARNESS_TRAY_COUNT, 34.0);
        cuts = cuts
            + centered_cylinder(
                format!("packaging_kitting_harness_long_tube_channel_{i}"),
                channel_d / 2.0,
                HARNESS_TRAY_X - 52.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -3.0)
            + centered_cube(
                format!("packaging_kitting_harness_top_loading_slot_{i}"),
                HARNESS_TRAY_X - 46.0,
                channel_d + 1.8,
                HARNESS_TRAY_Z,
            )
            .translate(0.0, y, HARNESS_TRAY_Z / 2.0 - 8.0)
            + centered_cube(
                format!("packaging_kitting_harness_left_connector_window_{i}"),
                34.0,
                20.0,
                HARNESS_TRAY_Z + 2.0,
            )
            .translate(-(HARNESS_TRAY_X / 2.0 - 42.0), y, 0.0)
            + centered_cube(
                format!("packaging_kitting_harness_right_connector_window_{i}"),
                34.0,
                20.0,
                HARNESS_TRAY_Z + 2.0,
            )
            .translate(HARNESS_TRAY_X / 2.0 - 42.0, y, 0.0);

        ribs = ribs
            + centered_cube(
                format!("packaging_kitting_harness_lot_clip_rib_{i}"),
                52.0,
                6.0,
                14.0,
            )
            .translate(0.0, y + 13.0, HARNESS_TRAY_Z / 2.0 + 7.0);
    }

    let keyed_corner = centered_cube(
        "packaging_kitting_harness_asymmetric_key_relief",
        46.0,
        24.0,
        HARNESS_TRAY_Z + 2.0,
    )
    .translate(
        -(HARNESS_TRAY_X / 2.0 - 34.0),
        HARNESS_TRAY_Y / 2.0 - 18.0,
        0.0,
    );

    body - cuts - keyed_corner
        + ribs
        + caddy_gripper_ears(
            "packaging_kitting_harness_tray",
            HARNESS_TRAY_X,
            HARNESS_TRAY_Z,
        )
}

fn single_use_manifold_insert_pockets() -> Part {
    let body = centered_cube(
        "packaging_kitting_manifold_insert_pocket_block",
        MANIFOLD_BLOCK_X,
        MANIFOLD_BLOCK_Y,
        MANIFOLD_BLOCK_Z,
    );
    let mut pockets = Part::empty("packaging_kitting_manifold_insert_pocket_cuts");
    let mut lips = Part::empty("packaging_kitting_manifold_insert_retention_lips");

    for row in 0..MANIFOLD_POCKET_ROWS {
        for col in 0..MANIFOLD_POCKET_COLS {
            let i = row * MANIFOLD_POCKET_COLS + col;
            let x = lane_position(col, MANIFOLD_POCKET_COLS, 76.0);
            let y = lane_position(row, MANIFOLD_POCKET_ROWS, 88.0);
            pockets = pockets
                + centered_cube(
                    format!("packaging_kitting_manifold_pocket_cut_{i}"),
                    54.0,
                    70.0,
                    28.0,
                )
                .translate(x, y, MANIFOLD_BLOCK_Z / 2.0 - 14.0)
                + centered_cube(
                    format!("packaging_kitting_manifold_key_cut_{i}"),
                    18.0,
                    28.0,
                    MANIFOLD_BLOCK_Z + 2.0,
                )
                .translate(x + 20.0, y + 20.0, 0.0);

            lips = lips
                + centered_cube(
                    format!("packaging_kitting_manifold_front_lip_{i}"),
                    64.0,
                    8.0,
                    14.0,
                )
                .translate(x, y - 39.0, MANIFOLD_BLOCK_Z / 2.0 + 7.0)
                + centered_cube(
                    format!("packaging_kitting_manifold_rear_lip_{i}"),
                    64.0,
                    8.0,
                    14.0,
                )
                .translate(x, y + 39.0, MANIFOLD_BLOCK_Z / 2.0 + 7.0);
        }
    }

    body - pockets
        + lips
        + caddy_gripper_ears(
            "packaging_kitting_manifold_pockets",
            MANIFOLD_BLOCK_X,
            MANIFOLD_BLOCK_Z,
        )
}

fn filter_connector_cap_staging() -> Part {
    let body = centered_cube(
        "packaging_kitting_filter_connector_cap_staging_block",
        STAGING_BLOCK_X,
        STAGING_BLOCK_Y,
        STAGING_BLOCK_Z,
    );
    let mut cuts = Part::empty("packaging_kitting_staging_cuts");
    let mut pegs = Part::empty("packaging_kitting_staging_positive_pegs");

    for i in 0..FILTER_COUNT {
        let x = lane_position(i, FILTER_COUNT, 36.0);
        cuts = cuts
            + centered_cylinder(
                format!("packaging_kitting_filter_saddle_cut_{i}"),
                12.0,
                74.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, 76.0, STAGING_BLOCK_Z / 2.0 - 6.0)
            + centered_cube(
                format!("packaging_kitting_filter_top_access_{i}"),
                28.0,
                38.0,
                STAGING_BLOCK_Z,
            )
            .translate(x, 76.0, STAGING_BLOCK_Z / 2.0 - 6.0);
    }

    for i in 0..CONNECTOR_COUNT {
        let col = i % 8;
        let row = i / 8;
        let x = lane_position(col, 8, 36.0);
        let y = -14.0 + row as f64 * 38.0;
        cuts = cuts
            + centered_cylinder(
                format!("packaging_kitting_connector_socket_cut_{i}"),
                8.0,
                STAGING_BLOCK_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);
        pegs = pegs
            + centered_cylinder(
                format!("packaging_kitting_connector_orientation_post_{i}"),
                2.8,
                8.0,
                20,
            )
            .translate(x + 12.0, y + 9.0, STAGING_BLOCK_Z / 2.0 + 4.0);
    }

    for i in 0..CAP_WELL_COUNT {
        let col = i % 16;
        let row = i / 16;
        let x = lane_position(col, 16, 18.0);
        let y = -88.0 + row as f64 * 26.0;
        cuts = cuts
            + centered_cylinder(
                format!("packaging_kitting_cap_well_cut_{i}"),
                4.6,
                STAGING_BLOCK_Z + 4.0,
                24,
            )
            .translate(x, y, 0.0);
    }

    body - cuts
        + pegs
        + caddy_gripper_ears(
            "packaging_kitting_staging",
            STAGING_BLOCK_X,
            STAGING_BLOCK_Z,
        )
}

fn barcode_rfid_lands() -> Part {
    let mut lands = Part::empty("packaging_kitting_barcode_rfid_lands");

    for i in 0..BARCODE_LANDS {
        let (x, y) = barcode_land_position(i);
        lands = lands
            + centered_cube(
                format!("packaging_kitting_barcode_lot_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(x, y, DECK_Z + LABEL_LAND_Z / 2.0 + 4.0);
    }

    for i in 0..RFID_LANDS {
        let (x, y) = rfid_land_position(i);
        let plate = centered_cube(
            format!("packaging_kitting_rfid_inlay_land_{i}"),
            42.0,
            30.0,
            LABEL_LAND_Z,
        )
        .translate(x, y, DECK_Z + LABEL_LAND_Z / 2.0 + 11.0);
        let antenna_mark = centered_cube(
            format!("packaging_kitting_rfid_antenna_notch_{i}"),
            30.0,
            3.0,
            LABEL_LAND_Z + 2.0,
        )
        .translate(x, y, DECK_Z + LABEL_LAND_Z / 2.0 + 11.0);
        lands = lands + (plate - antenna_mark);
    }

    lands
}

fn clean_used_reject_segregation() -> Part {
    let clean_lane = segregation_lane(
        "clean",
        CLEAN_ZONE_X,
        -92.0,
        CLEAN_LANE_W,
        STATUS_BIN_Y,
        42.0,
    );
    let used_lane = segregation_lane(
        "used",
        STAGING_ZONE_X - 46.0,
        -98.0,
        USED_LANE_W,
        STATUS_BIN_Y,
        54.0,
    );
    let reject = reject_bin().translate(DECK_X / 2.0 - 136.0, -DECK_Y / 2.0 + 146.0, DECK_Z + 34.0);
    let divider_1 = centered_cube(
        "packaging_kitting_clean_to_work_status_divider",
        16.0,
        STATUS_BIN_Y + 40.0,
        SEGREGATION_SPINE_Z,
    )
    .translate(-126.0, -92.0, DECK_Z + SEGREGATION_SPINE_Z / 2.0);
    let divider_2 = centered_cube(
        "packaging_kitting_work_to_used_status_divider",
        16.0,
        STATUS_BIN_Y + 40.0,
        SEGREGATION_SPINE_Z,
    )
    .translate(214.0, -92.0, DECK_Z + SEGREGATION_SPINE_Z / 2.0);

    clean_lane + used_lane + reject + divider_1 + divider_2
}

fn segregation_lane(name: &str, x: f64, y: f64, width: f64, depth: f64, wall_z: f64) -> Part {
    let rear = centered_cube(
        format!("packaging_kitting_{name}_segregation_rear_wall"),
        width,
        10.0,
        wall_z,
    )
    .translate(x, y + depth / 2.0, DECK_Z + wall_z / 2.0);
    let left = centered_cube(
        format!("packaging_kitting_{name}_segregation_left_wall"),
        10.0,
        depth,
        wall_z,
    )
    .translate(x - width / 2.0, y, DECK_Z + wall_z / 2.0);
    let right = centered_cube(
        format!("packaging_kitting_{name}_segregation_right_wall"),
        10.0,
        depth,
        wall_z,
    )
    .translate(x + width / 2.0, y, DECK_Z + wall_z / 2.0);
    let low_floor = centered_cube(
        format!("packaging_kitting_{name}_segregation_low_floor_marker"),
        width - 22.0,
        depth - 20.0,
        4.0,
    )
    .translate(x, y, DECK_Z + 8.0);

    rear + left + right + low_floor
}

fn reject_bin() -> Part {
    let outer = centered_cube(
        "packaging_kitting_reject_mismatch_outer_bin",
        REJECT_LANE_W,
        174.0,
        68.0,
    );
    let cavity = centered_cube(
        "packaging_kitting_reject_mismatch_open_cavity",
        REJECT_LANE_W - 24.0,
        150.0,
        60.0,
    )
    .translate(0.0, 0.0, 12.0);
    let notch = centered_cube(
        "packaging_kitting_reject_mismatch_robot_finger_notch",
        72.0,
        20.0,
        74.0,
    )
    .translate(0.0, -87.0, 0.0);
    outer - cavity - notch
}

fn sealed_pouch_tote_interface() -> Part {
    let pouch_clamp = centered_cube(
        "packaging_kitting_sealed_pouch_clamp_base",
        POUCH_DOCK_X,
        POUCH_DOCK_Y,
        POUCH_DOCK_Z,
    );
    let clamp_channel = centered_cube(
        "packaging_kitting_pouch_heatseal_edge_channel",
        POUCH_DOCK_X - 70.0,
        24.0,
        POUCH_DOCK_Z + 2.0,
    )
    .translate(0.0, 8.0, 0.0);
    let pouch_window = centered_cube(
        "packaging_kitting_pouch_visual_inspection_window",
        POUCH_DOCK_X - 126.0,
        42.0,
        POUCH_DOCK_Z + 2.0,
    )
    .translate(0.0, -35.0, 0.0);

    let tote = centered_cube(
        "packaging_kitting_sealed_tote_datum_tongue",
        TOTE_INTERFACE_X,
        TOTE_INTERFACE_Y,
        TOTE_INTERFACE_Z,
    )
    .translate(
        0.0,
        -(POUCH_DOCK_Y / 2.0 + TOTE_INTERFACE_Y / 2.0 - 4.0),
        10.0,
    );
    let tote_key = centered_cube(
        "packaging_kitting_tote_center_key_slot",
        92.0,
        TOTE_INTERFACE_Y + 4.0,
        16.0,
    )
    .translate(
        0.0,
        -(POUCH_DOCK_Y / 2.0 + TOTE_INTERFACE_Y / 2.0 - 4.0),
        28.0,
    );

    let gasket = rectangular_frame(
        "packaging_kitting_pouch_gasket_frame",
        POUCH_DOCK_X - 48.0,
        8.0,
        64.0,
        POUCH_DOCK_X - 128.0,
        18.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, POUCH_DOCK_Y / 2.0 + 4.0, 10.0);

    pouch_clamp - clamp_channel - pouch_window + (tote - tote_key) + gasket
}

fn leak_test_handoff_ports() -> Part {
    let body = centered_cube(
        "packaging_kitting_leak_test_handoff_port_bar",
        LEAK_PORT_BAR_X,
        LEAK_PORT_BAR_Y,
        LEAK_PORT_BAR_Z,
    );
    let mut cuts = Part::empty("packaging_kitting_leak_test_port_cuts");
    let mut blanking_cap_docks = Part::empty("packaging_kitting_leak_test_blanking_cap_docks");

    for i in 0..LEAK_PORT_COUNT {
        let x = leak_port_x(i);
        cuts = cuts
            + centered_cylinder(
                format!("packaging_kitting_leak_test_port_bore_{i}"),
                LEAK_PORT_D / 2.0,
                LEAK_PORT_BAR_Y + 8.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 2.0);
        blanking_cap_docks = blanking_cap_docks
            + centered_cylinder(
                format!("packaging_kitting_blanking_cap_dock_{i}"),
                7.6,
                8.0,
                26,
            )
            .translate(x, LEAK_PORT_BAR_Y / 2.0 - 14.0, LEAK_PORT_BAR_Z / 2.0 + 4.0);
    }

    let sensor_land = centered_cube(
        "packaging_kitting_leak_test_pressure_sensor_land",
        150.0,
        28.0,
        10.0,
    )
    .translate(
        0.0,
        -(LEAK_PORT_BAR_Y / 2.0 - 18.0),
        LEAK_PORT_BAR_Z / 2.0 + 5.0,
    );

    body - cuts + blanking_cap_docks + sensor_land
}

fn robot_pick_datums() -> Part {
    let mut datums = Part::empty("packaging_kitting_robot_pick_datums");

    for (i, (x, y)) in pick_pad_positions().iter().enumerate() {
        let pad = centered_cylinder(
            format!("packaging_kitting_robot_pick_pad_{i}"),
            PICK_PAD_D / 2.0,
            PICK_PAD_Z,
            40,
        )
        .translate(*x, *y, DECK_Z + PICK_PAD_Z / 2.0 + 6.0);
        let cross_a = centered_cube(
            format!("packaging_kitting_robot_pick_cross_a_{i}"),
            PICK_PAD_D + 7.0,
            3.0,
            2.0,
        )
        .translate(*x, *y, DECK_Z + PICK_PAD_Z + 12.0);
        let cross_b = centered_cube(
            format!("packaging_kitting_robot_pick_cross_b_{i}"),
            3.0,
            PICK_PAD_D + 7.0,
            2.0,
        )
        .translate(*x, *y, DECK_Z + PICK_PAD_Z + 12.0);
        datums = datums + pad + cross_a + cross_b;
    }

    for (i, (x, y)) in datum_pin_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("packaging_kitting_robot_datum_boss_{i}"),
            12.0,
            10.0,
            34,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        let pin_hole = centered_cylinder(
            format!("packaging_kitting_robot_datum_pin_hole_{i}"),
            3.2,
            12.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        datums = datums + (boss - pin_hole);
    }

    datums + gripper_finger_lands()
}

fn gripper_finger_lands() -> Part {
    let left = centered_cube(
        "packaging_kitting_left_robot_gripper_finger_land",
        32.0,
        168.0,
        18.0,
    )
    .translate(-(DECK_X / 2.0 - 72.0), UPPER_ZONE_Y, DECK_Z + 18.0);
    let right = centered_cube(
        "packaging_kitting_right_robot_gripper_finger_land",
        32.0,
        168.0,
        18.0,
    )
    .translate(DECK_X / 2.0 - 72.0, UPPER_ZONE_Y, DECK_Z + 18.0);
    let left_slot = centered_cube(
        "packaging_kitting_left_robot_gripper_slot",
        18.0,
        126.0,
        8.0,
    )
    .translate(-(DECK_X / 2.0 - 72.0), UPPER_ZONE_Y, DECK_Z + 18.0);
    let right_slot = centered_cube(
        "packaging_kitting_right_robot_gripper_slot",
        18.0,
        126.0,
        8.0,
    )
    .translate(DECK_X / 2.0 - 72.0, UPPER_ZONE_Y, DECK_Z + 18.0);
    (left - left_slot) + (right - right_slot)
}

fn inspection_window_camera_bridge() -> Part {
    let window_frame = rectangular_frame(
        "packaging_kitting_inspection_window_frame",
        INSPECTION_WINDOW_X,
        14.0,
        INSPECTION_WINDOW_Y,
        INSPECTION_WINDOW_X - 72.0,
        INSPECTION_WINDOW_Y - 68.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WORK_ZONE_X, UPPER_ZONE_Y, DECK_Z + 10.0);

    let left_post = centered_cube(
        "packaging_kitting_camera_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        WORK_ZONE_X - CAMERA_BRIDGE_SPAN_X / 2.0,
        UPPER_ZONE_Y,
        (CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z) / 2.0,
    );
    let right_post = centered_cube(
        "packaging_kitting_camera_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        WORK_ZONE_X + CAMERA_BRIDGE_SPAN_X / 2.0,
        UPPER_ZONE_Y,
        (CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z) / 2.0,
    );
    let beam = centered_cube(
        "packaging_kitting_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X + 34.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        WORK_ZONE_X,
        UPPER_ZONE_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let camera_mount = centered_cube("packaging_kitting_camera_mount_plate", 128.0, 74.0, 16.0)
        .translate(
            WORK_ZONE_X,
            UPPER_ZONE_Y - 4.0,
            CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z + 8.0,
        );
    let illumination_bar_front = centered_cube(
        "packaging_kitting_front_illumination_bar",
        CAMERA_BRIDGE_SPAN_X - 96.0,
        12.0,
        16.0,
    )
    .translate(
        WORK_ZONE_X,
        UPPER_ZONE_Y - 80.0,
        CAMERA_BRIDGE_UNDERSIDE_Z - 24.0,
    );
    let illumination_bar_rear = centered_cube(
        "packaging_kitting_rear_illumination_bar",
        CAMERA_BRIDGE_SPAN_X - 96.0,
        12.0,
        16.0,
    )
    .translate(
        WORK_ZONE_X,
        UPPER_ZONE_Y + 80.0,
        CAMERA_BRIDGE_UNDERSIDE_Z - 24.0,
    );

    window_frame
        + left_post
        + right_post
        + beam
        + camera_mount
        + illumination_bar_front
        + illumination_bar_rear
}

fn service_keepouts() -> Part {
    let front = keepout_cage(
        "front_robot_approach",
        DECK_X - 150.0,
        FRONT_ROBOT_KEEP_OUT,
        120.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT / 2.0), 78.0);
    let rear = keepout_cage(
        "rear_service_access",
        DECK_X - 220.0,
        REAR_SERVICE_KEEP_OUT,
        150.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT / 2.0, 92.0);
    let side = keepout_cage("right_tote_swing", SIDE_TOTE_SWING_KEEP_OUT, 380.0, 130.0).translate(
        DECK_X / 2.0 + SIDE_TOTE_SWING_KEEP_OUT / 2.0,
        -70.0,
        82.0,
    );
    let camera = keepout_cage(
        "camera_service_z",
        CAMERA_BRIDGE_SPAN_X - 80.0,
        170.0,
        CAMERA_SERVICE_KEEP_OUT_Z,
    )
    .translate(WORK_ZONE_X, UPPER_ZONE_Y, CAMERA_SERVICE_KEEP_OUT_Z / 2.0);

    front + rear + side + camera
}

fn keepout_cage(name: &str, x: f64, y: f64, z: f64) -> Part {
    let rail = 8.0;
    let bottom_front = centered_cube(format!("{name}_bottom_front"), x, rail, rail).translate(
        0.0,
        -y / 2.0,
        -z / 2.0,
    );
    let bottom_rear = centered_cube(format!("{name}_bottom_rear"), x, rail, rail).translate(
        0.0,
        y / 2.0,
        -z / 2.0,
    );
    let bottom_left = centered_cube(format!("{name}_bottom_left"), rail, y, rail).translate(
        -x / 2.0,
        0.0,
        -z / 2.0,
    );
    let bottom_right = centered_cube(format!("{name}_bottom_right"), rail, y, rail).translate(
        x / 2.0,
        0.0,
        -z / 2.0,
    );
    let top_front =
        centered_cube(format!("{name}_top_front"), x, rail, rail).translate(0.0, -y / 2.0, z / 2.0);
    let top_rear =
        centered_cube(format!("{name}_top_rear"), x, rail, rail).translate(0.0, y / 2.0, z / 2.0);
    let top_left =
        centered_cube(format!("{name}_top_left"), rail, y, rail).translate(-x / 2.0, 0.0, z / 2.0);
    let top_right =
        centered_cube(format!("{name}_top_right"), rail, y, rail).translate(x / 2.0, 0.0, z / 2.0);

    let mut posts = Part::empty(format!("{name}_vertical_posts"));
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
            + centered_cube(format!("{name}_vertical_post_{i}"), rail, rail, z)
                .translate(*px, *py, 0.0);
    }

    bottom_front
        + bottom_rear
        + bottom_left
        + bottom_right
        + top_front
        + top_rear
        + top_left
        + top_right
        + posts
}

fn caddy_gripper_ears(name: &str, body_x: f64, body_z: f64) -> Part {
    let left = centered_cube(format!("{name}_left_pick_ear"), 20.0, 86.0, 18.0).translate(
        -(body_x / 2.0 + 10.0),
        0.0,
        body_z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_pick_ear"), 20.0, 86.0, 18.0).translate(
        body_x / 2.0 + 10.0,
        0.0,
        body_z / 2.0,
    );
    let left_slot = centered_cube(format!("{name}_left_pick_slot"), 8.0, 58.0, 8.0).translate(
        -(body_x / 2.0 + 10.0),
        0.0,
        body_z / 2.0,
    );
    let right_slot = centered_cube(format!("{name}_right_pick_slot"), 8.0, 58.0, 8.0).translate(
        body_x / 2.0 + 10.0,
        0.0,
        body_z / 2.0,
    );
    (left - left_slot) + (right - right_slot)
}

fn rectangular_frame(
    name: &str,
    outer_x: f64,
    y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, y, outer_z);
    let inner = centered_cube(format!("{name}_inner_cut"), inner_x, y + 2.0, inner_z);
    outer - inner
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn leak_port_x(index: usize) -> f64 {
    lane_position(index, LEAK_PORT_COUNT, 46.0)
}

fn barcode_land_position(index: usize) -> (f64, f64) {
    let positions = [
        (CLEAN_ZONE_X - 110.0, UPPER_ZONE_Y + 162.0),
        (CLEAN_ZONE_X + 8.0, UPPER_ZONE_Y + 162.0),
        (CLEAN_ZONE_X + 126.0, UPPER_ZONE_Y + 162.0),
        (WORK_ZONE_X - 112.0, UPPER_ZONE_Y + 162.0),
        (WORK_ZONE_X + 6.0, UPPER_ZONE_Y + 162.0),
        (WORK_ZONE_X + 124.0, UPPER_ZONE_Y + 162.0),
        (STAGING_ZONE_X - 112.0, UPPER_ZONE_Y + 162.0),
        (STAGING_ZONE_X + 6.0, UPPER_ZONE_Y + 162.0),
        (STAGING_ZONE_X + 124.0, UPPER_ZONE_Y + 162.0),
        (-250.0, FRONT_ZONE_Y + 88.0),
        (-128.0, FRONT_ZONE_Y + 88.0),
        (0.0, FRONT_ZONE_Y + 88.0),
        (128.0, FRONT_ZONE_Y + 88.0),
        (250.0, FRONT_ZONE_Y + 88.0),
    ];
    positions[index]
}

fn rfid_land_position(index: usize) -> (f64, f64) {
    let positions = [
        (CLEAN_ZONE_X - 126.0, UPPER_ZONE_Y - 164.0),
        (CLEAN_ZONE_X + 126.0, UPPER_ZONE_Y - 164.0),
        (WORK_ZONE_X - 126.0, UPPER_ZONE_Y - 164.0),
        (WORK_ZONE_X + 126.0, UPPER_ZONE_Y - 164.0),
        (STAGING_ZONE_X - 126.0, UPPER_ZONE_Y - 164.0),
        (STAGING_ZONE_X + 126.0, UPPER_ZONE_Y - 164.0),
        (-214.0, FRONT_ZONE_Y - 86.0),
        (214.0, FRONT_ZONE_Y - 86.0),
    ];
    positions[index]
}

fn pick_pad_positions() -> [(f64, f64); ROBOT_PICK_PADS] {
    [
        (CLEAN_ZONE_X - 166.0, UPPER_ZONE_Y - 154.0),
        (CLEAN_ZONE_X + 166.0, UPPER_ZONE_Y - 154.0),
        (WORK_ZONE_X - 170.0, UPPER_ZONE_Y - 154.0),
        (WORK_ZONE_X + 170.0, UPPER_ZONE_Y - 154.0),
        (STAGING_ZONE_X - 164.0, UPPER_ZONE_Y - 154.0),
        (STAGING_ZONE_X + 164.0, UPPER_ZONE_Y - 154.0),
        (-214.0, FRONT_ZONE_Y - 86.0),
        (214.0, FRONT_ZONE_Y - 86.0),
    ]
}

fn datum_pin_positions() -> [(f64, f64); ROBOT_DATUM_PINS] {
    [
        (CLEAN_ZONE_X - 164.0, UPPER_ZONE_Y + 144.0),
        (CLEAN_ZONE_X + 164.0, UPPER_ZONE_Y + 144.0),
        (WORK_ZONE_X, UPPER_ZONE_Y + 144.0),
        (STAGING_ZONE_X - 164.0, UPPER_ZONE_Y + 144.0),
        (STAGING_ZONE_X + 164.0, UPPER_ZONE_Y + 144.0),
        (0.0, FRONT_ZONE_Y - 104.0),
    ]
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct PartSpec {
    path: &'static str,
    min_size: [f64; 3],
}

#[cfg(test)]
fn output_specs() -> [PartSpec; 12] {
    [
        PartSpec {
            path: OUTPUTS[0],
            min_size: [DECK_X, DECK_Y, RIM_Z],
        },
        PartSpec {
            path: OUTPUTS[1],
            min_size: [HARNESS_TRAY_X, HARNESS_TRAY_Y, HARNESS_TRAY_Z],
        },
        PartSpec {
            path: OUTPUTS[2],
            min_size: [MANIFOLD_BLOCK_X, MANIFOLD_BLOCK_Y, MANIFOLD_BLOCK_Z],
        },
        PartSpec {
            path: OUTPUTS[3],
            min_size: [STAGING_BLOCK_X, STAGING_BLOCK_Y, STAGING_BLOCK_Z],
        },
        PartSpec {
            path: OUTPUTS[4],
            min_size: [LABEL_LAND_X, LABEL_LAND_Y, LABEL_LAND_Z],
        },
        PartSpec {
            path: OUTPUTS[5],
            min_size: [
                CLEAN_LANE_W + USED_LANE_W,
                STATUS_BIN_Y,
                SEGREGATION_SPINE_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[6],
            min_size: [
                POUCH_DOCK_X,
                POUCH_DOCK_Y + TOTE_INTERFACE_Y,
                TOTE_INTERFACE_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[7],
            min_size: [LEAK_PORT_BAR_X, LEAK_PORT_BAR_Y, LEAK_PORT_BAR_Z],
        },
        PartSpec {
            path: OUTPUTS[8],
            min_size: [DECK_X - 160.0, DECK_Y - 250.0, PICK_PAD_Z],
        },
        PartSpec {
            path: OUTPUTS[9],
            min_size: [
                CAMERA_BRIDGE_SPAN_X,
                INSPECTION_WINDOW_Y,
                CAMERA_BRIDGE_UNDERSIDE_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[10],
            min_size: [
                DECK_X,
                DECK_Y + FRONT_ROBOT_KEEP_OUT,
                CAMERA_SERVICE_KEEP_OUT_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[11],
            min_size: [
                DECK_X,
                DECK_Y + FRONT_ROBOT_KEEP_OUT,
                CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z,
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_contract_lists_named_parts_and_assembly() {
        let specs = output_specs();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(specs.len(), OUTPUTS.len());
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/closed_fluid_path_packaging_kitting_station_")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
        assert!(specs.iter().all(|spec| spec.path.ends_with(".stl")));
        assert!(specs.iter().all(|spec| spec.min_size[0] > 0.0
            && spec.min_size[1] > 0.0
            && spec.min_size[2] > 0.0));
    }

    #[test]
    fn kit_capacity_covers_closed_fluid_path_materials() {
        assert_eq!(HARNESS_TRAY_COUNT, 6);
        assert_eq!(
            MANIFOLD_POCKET_COUNT,
            MANIFOLD_POCKET_COLS * MANIFOLD_POCKET_ROWS
        );
        assert_eq!(FILTER_COUNT, 8);
        assert_eq!(CONNECTOR_COUNT, 16);
        assert_eq!(CAP_WELL_COUNT, 32);
        assert!(CAP_WELL_COUNT >= CONNECTOR_COUNT * 2);
        assert!(BARCODE_LANDS >= HARNESS_TRAY_COUNT + MANIFOLD_POCKET_ROWS + 4);
        assert!(RFID_LANDS >= MANIFOLD_POCKET_ROWS + 4);
    }

    #[test]
    fn station_dimensions_keep_major_features_inside_deck() {
        assert!(HARNESS_TRAY_X < CLEAN_LANE_W);
        assert!(MANIFOLD_BLOCK_X < 360.0);
        assert!(STAGING_BLOCK_X < 360.0);
        assert!(POUCH_DOCK_X < DECK_X - 2.0 * RIM_W);
        assert!(UPPER_ZONE_Y + HARNESS_TRAY_Y / 2.0 < DECK_Y / 2.0 - RIM_W);
        assert!(FRONT_ZONE_Y - (POUCH_DOCK_Y + TOTE_INTERFACE_Y) / 2.0 > -DECK_Y / 2.0 + RIM_W);
    }

    #[test]
    fn leak_test_port_array_has_spacing_and_symmetry() {
        let first = leak_port_x(0);
        let last = leak_port_x(LEAK_PORT_COUNT - 1);
        assert_eq!(LEAK_PORT_COUNT, 12);
        assert!((first + last).abs() < 0.01);
        assert!(last - first < LEAK_PORT_BAR_X - 72.0);
        assert!((leak_port_x(1) - leak_port_x(0) - 46.0).abs() < 0.01);
    }

    #[test]
    fn robot_pick_datums_are_counted_and_on_deck() {
        let pads = pick_pad_positions();
        let pins = datum_pin_positions();
        assert_eq!(pads.len(), ROBOT_PICK_PADS);
        assert_eq!(pins.len(), ROBOT_DATUM_PINS);

        for (x, y) in pads.iter().chain(pins.iter()) {
            assert!(x.abs() < DECK_X / 2.0 - RIM_W);
            assert!(y.abs() < DECK_Y / 2.0 - RIM_W);
        }
    }

    #[test]
    fn inspection_bridge_clears_kitting_blocks_and_service_zone() {
        let tallest_block = HARNESS_TRAY_Z.max(MANIFOLD_BLOCK_Z).max(STAGING_BLOCK_Z) + DECK_Z;
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z > tallest_block + 110.0);
        assert!(CAMERA_BRIDGE_SPAN_X > MANIFOLD_BLOCK_X + STAGING_BLOCK_X);
        assert!(CAMERA_SERVICE_KEEP_OUT_Z > CAMERA_BRIDGE_UNDERSIDE_Z);
    }
}
