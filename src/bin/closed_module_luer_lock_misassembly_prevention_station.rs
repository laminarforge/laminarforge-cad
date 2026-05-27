use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module luer-lock/tubing misassembly prevention station.
//
// Intent:
// - Prevent wrong-route, wrong-orientation, capped/uncapped, and under/over
//   torque assembly errors before closed culture modules enter the cabinet.
// - Put keyed luer nests, go/no-go gauges, barcode/route verification pads,
//   leak witness containment, cap state segregation, and torque witnessing on
//   one traceable benchtop fixture.
// - Keep labels as CSG geometry so the printed station remains interpretable
//   even when exported as standalone STL parts.
//
// This is interface/fixture CAD only. It is not a sterile-barrier design,
// acceptance protocol, leak test method, or clinical/bioprocessing instruction.

const OUTPUTS: [&str; 11] = [
    "output/closed_module_luer_lock_misassembly_prevention_station_deck.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_leak_witness_tray.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_keyed_connector_nests.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_go_nogo_gauges.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_barcode_route_verification_pads.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_capped_uncapped_segregation.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_torque_witness_scale.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_route_harness_reference_frame.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_csg_labels.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_robot_service_keepouts.stl",
    "output/closed_module_luer_lock_misassembly_prevention_station_assembly.stl",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 18.0;
const DECK_RIM_W: f64 = 16.0;
const DECK_RIM_Z: f64 = 36.0;
const MOUNT_HOLE_D: f64 = 6.4;

const LEAK_TRAY_X: f64 = 1080.0;
const LEAK_TRAY_Y: f64 = 640.0;
const LEAK_TRAY_Z: f64 = 28.0;
const LEAK_TRAY_WALL: f64 = 14.0;
const LEAK_DRAIN_D: f64 = 14.0;
const WITNESS_PAD_COUNT: usize = 16;
const WITNESS_PAD_COLS: usize = 4;

const WORK_Z: f64 = DECK_Z + LEAK_TRAY_Z + 8.0;

const ROUTE_COUNT: usize = 4;
const NESTS_PER_ROUTE: usize = 5;
const NEST_BANK_X: f64 = 560.0;
const NEST_BANK_Y: f64 = 304.0;
const NEST_BANK_Z: f64 = 42.0;
const NEST_BANK_CENTER: (f64, f64) = (-300.0, 132.0);
const ROUTE_LANE_PITCH_Y: f64 = 64.0;
const NEST_PITCH_X: f64 = 92.0;
const LUER_POCKET_D: f64 = 17.0;
const LUER_POCKET_DEPTH: f64 = 23.0;
const KEY_SLOT_W: f64 = 7.2;
const KEY_SLOT_L: f64 = 21.0;
const KEY_BOSS_W: f64 = 8.0;
const KEY_BOSS_L: f64 = 30.0;
const KEY_BOSS_Z: f64 = 7.0;

const GAUGE_BANK_X: f64 = 360.0;
const GAUGE_BANK_Y: f64 = 232.0;
const GAUGE_BANK_Z: f64 = 44.0;
const GAUGE_BANK_CENTER: (f64, f64) = (350.0, 162.0);
const GAUGE_ROWS: usize = 4;
const GAUGE_COLS: usize = 3;
const GAUGE_PAIR_PITCH_X: f64 = 82.0;
const GAUGE_PAIR_PITCH_Y: f64 = 46.0;

const SCAN_PANEL_X: f64 = 860.0;
const SCAN_PANEL_Y: f64 = 118.0;
const SCAN_PANEL_Z: f64 = 20.0;
const SCAN_PANEL_CENTER: (f64, f64) = (0.0, -332.0);
const ROUTE_SCAN_LANDS: usize = ROUTE_COUNT * 3;
const OPERATOR_VERIFY_LANDS: usize = 4;
const SCAN_LAND_X: f64 = 84.0;
const SCAN_LAND_Y: f64 = 30.0;
const SCAN_LAND_Z: f64 = 4.0;

const SEG_BIN_X: f64 = 338.0;
const SEG_BIN_Y: f64 = 240.0;
const SEG_BIN_Z: f64 = 62.0;
const SEG_BIN_CENTER: (f64, f64) = (-366.0, -128.0);
const CAP_WELL_COUNT: usize = 24;
const CAP_WELL_COLS: usize = 6;
const UNCAPPED_SLOTS: usize = 12;
const BIN_WALL: f64 = 12.0;

const TORQUE_BASE_X: f64 = 374.0;
const TORQUE_BASE_Y: f64 = 252.0;
const TORQUE_BASE_Z: f64 = 32.0;
const TORQUE_CENTER: (f64, f64) = (366.0, -120.0);
const TORQUE_TICKS: usize = 13;
const TORQUE_RADIUS: f64 = 92.0;
const TORQUE_PIVOT_D: f64 = 36.0;
const TORQUE_TOOL_DOCK_D: f64 = 26.0;

const ROUTE_FRAME_X: f64 = 930.0;
const ROUTE_FRAME_Y: f64 = 120.0;
const ROUTE_FRAME_Z: f64 = 48.0;
const ROUTE_FRAME_CENTER: (f64, f64) = (0.0, 322.0);
const TUBING_CHANNEL_D: f64 = 8.6;
const TUBING_CHANNELS_PER_ROUTE: usize = 3;

const KEEP_OUT_RAIL_Z: f64 = 8.0;
const ROBOT_KEEP_OUT_X: f64 = 360.0;
const ROBOT_KEEP_OUT_Y: f64 = 146.0;
const SERVICE_KEEP_OUT_X: f64 = 220.0;
const SERVICE_KEEP_OUT_Y: f64 = 650.0;

const LABEL_PLATE_Z: f64 = 3.0;
const LABEL_TEXT_Z: f64 = 2.2;
const LABEL_PIXEL: f64 = 2.8;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let leak = leak_witness_tray();
    export(OUTPUTS[1], &leak);

    let nests = keyed_connector_nests();
    export(OUTPUTS[2], &nests);

    let gauges = go_nogo_gauges();
    export(OUTPUTS[3], &gauges);

    let scan = barcode_route_verification_pads();
    export(OUTPUTS[4], &scan);

    let segregation = capped_uncapped_segregation();
    export(OUTPUTS[5], &segregation);

    let torque = torque_witness_scale();
    export(OUTPUTS[6], &torque);

    let route_frame = route_harness_reference_frame();
    export(OUTPUTS[7], &route_frame);

    let labels = csg_labels();
    export(OUTPUTS[8], &labels);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = deck
        + leak.translate(0.0, 0.0, DECK_Z)
        + nests.translate(NEST_BANK_CENTER.0, NEST_BANK_CENTER.1, WORK_Z)
        + gauges.translate(GAUGE_BANK_CENTER.0, GAUGE_BANK_CENTER.1, WORK_Z)
        + scan.translate(SCAN_PANEL_CENTER.0, SCAN_PANEL_CENTER.1, WORK_Z)
        + segregation.translate(SEG_BIN_CENTER.0, SEG_BIN_CENTER.1, WORK_Z)
        + torque.translate(TORQUE_CENTER.0, TORQUE_CENTER.1, WORK_Z)
        + route_frame.translate(ROUTE_FRAME_CENTER.0, ROUTE_FRAME_CENTER.1, WORK_Z)
        + labels
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed-module luer-lock/tubing misassembly prevention station:");
    println!("  Footprint:                 {DECK_X:.0}mm x {DECK_Y:.0}mm deck with leak-witness containment");
    println!(
        "  Keyed connector nests:     {} route lanes x {} luer positions = {} keyed nests",
        ROUTE_COUNT,
        NESTS_PER_ROUTE,
        ROUTE_COUNT * NESTS_PER_ROUTE
    );
    println!(
        "  Go/no-go checks:           {} route rows x {} gauge families = {} paired gauge lands",
        GAUGE_ROWS,
        GAUGE_COLS,
        GAUGE_ROWS * GAUGE_COLS
    );
    println!(
        "  Verification pads:         {} route barcode/connector/tube lands plus {} operator/lot pads",
        ROUTE_SCAN_LANDS, OPERATOR_VERIFY_LANDS
    );
    println!(
        "  Segregation:               {} capped wells, {} uncapped luer slots, reject lane, and center divider",
        CAP_WELL_COUNT, UNCAPPED_SLOTS
    );
    println!(
        "  Torque witnessing:         {TORQUE_TICKS} tick scale, pivot ring, under/over stops, and tool dock");
    println!(
        "  Labels:                    raised CSG block labels for nests, gauges, scan route, leak witness, caps, uncapped, torque, wrong-way, and route A-D"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_deck() -> Part {
    let floor = centered_cube(
        "luer_misassembly_station_deck_floor",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let leak_socket = centered_cube(
        "luer_misassembly_station_leak_tray_socket",
        LEAK_TRAY_X + 28.0,
        LEAK_TRAY_Y + 24.0,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z - 3.5);
    let scan_socket = centered_cube(
        "luer_misassembly_station_scan_panel_socket",
        SCAN_PANEL_X + 24.0,
        SCAN_PANEL_Y + 20.0,
        6.0,
    )
    .translate(SCAN_PANEL_CENTER.0, SCAN_PANEL_CENTER.1, DECK_Z - 3.0);

    floor - leak_socket - scan_socket - deck_mount_holes()
        + deck_rim()
        + deck_corner_datums()
        + deck_zone_recess_lands()
}

fn deck_rim() -> Part {
    let front = centered_cube(
        "luer_misassembly_station_front_rim",
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
        "luer_misassembly_station_rear_rim",
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
        "luer_misassembly_station_left_rim",
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
        "luer_misassembly_station_right_rim",
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
    let mut holes = Part::empty("luer_misassembly_station_deck_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 46.0, -DECK_Y / 2.0 + 46.0),
        (DECK_X / 2.0 - 46.0, -DECK_Y / 2.0 + 46.0),
        (-DECK_X / 2.0 + 46.0, DECK_Y / 2.0 - 46.0),
        (DECK_X / 2.0 - 46.0, DECK_Y / 2.0 - 46.0),
        (0.0, -DECK_Y / 2.0 + 46.0),
        (0.0, DECK_Y / 2.0 - 46.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("luer_misassembly_station_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn deck_corner_datums() -> Part {
    let mut datums = Part::empty("luer_misassembly_station_corner_datums");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 82.0, -DECK_Y / 2.0 + 82.0),
        (DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 82.0),
        (-DECK_X / 2.0 + 82.0, DECK_Y / 2.0 - 82.0),
        (DECK_X / 2.0 - 82.0, DECK_Y / 2.0 - 82.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("luer_misassembly_station_corner_datum_boss_{i}"),
            22.0 / 2.0,
            7.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        let pin = centered_cylinder(
            format!("luer_misassembly_station_corner_datum_pin_clearance_{i}"),
            5.0 / 2.0,
            9.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        datums = datums + (boss - pin);
    }
    datums
}

fn deck_zone_recess_lands() -> Part {
    let nest_land = centered_cube(
        "luer_misassembly_station_keyed_nest_recess_land",
        NEST_BANK_X + 30.0,
        NEST_BANK_Y + 28.0,
        4.0,
    )
    .translate(NEST_BANK_CENTER.0, NEST_BANK_CENTER.1, DECK_Z + 2.0);
    let gauge_land = centered_cube(
        "luer_misassembly_station_gauge_bank_recess_land",
        GAUGE_BANK_X + 28.0,
        GAUGE_BANK_Y + 28.0,
        4.0,
    )
    .translate(GAUGE_BANK_CENTER.0, GAUGE_BANK_CENTER.1, DECK_Z + 2.0);
    let torque_land = centered_cube(
        "luer_misassembly_station_torque_scale_recess_land",
        TORQUE_BASE_X + 30.0,
        TORQUE_BASE_Y + 30.0,
        4.0,
    )
    .translate(TORQUE_CENTER.0, TORQUE_CENTER.1, DECK_Z + 2.0);
    nest_land + gauge_land + torque_land
}

fn leak_witness_tray() -> Part {
    let floor = centered_cube(
        "luer_misassembly_leak_witness_tray_floor",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0);
    let basin_cut = centered_cube(
        "luer_misassembly_leak_witness_tray_basin",
        LEAK_TRAY_X - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Y - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Z - 7.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z - (LEAK_TRAY_Z - 7.0) / 2.0);
    let drain = centered_cylinder(
        "luer_misassembly_leak_witness_tray_front_drain",
        LEAK_DRAIN_D / 2.0,
        46.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        LEAK_TRAY_X / 2.0 - 82.0,
        -LEAK_TRAY_Y / 2.0 + 8.0,
        LEAK_TRAY_Z - 10.0,
    );

    floor - basin_cut - drain + leak_tray_rim() + witness_pads() + witness_flow_channels()
}

fn leak_tray_rim() -> Part {
    let front = centered_cube(
        "luer_misassembly_leak_witness_front_high_rim",
        LEAK_TRAY_X,
        LEAK_TRAY_WALL,
        LEAK_TRAY_Z + 14.0,
    )
    .translate(
        0.0,
        -LEAK_TRAY_Y / 2.0 + LEAK_TRAY_WALL / 2.0,
        LEAK_TRAY_Z / 2.0 + 7.0,
    );
    let rear = centered_cube(
        "luer_misassembly_leak_witness_rear_rim",
        LEAK_TRAY_X,
        LEAK_TRAY_WALL,
        LEAK_TRAY_Z + 8.0,
    )
    .translate(
        0.0,
        LEAK_TRAY_Y / 2.0 - LEAK_TRAY_WALL / 2.0,
        LEAK_TRAY_Z / 2.0 + 4.0,
    );
    let left = centered_cube(
        "luer_misassembly_leak_witness_left_rim",
        LEAK_TRAY_WALL,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z + 8.0,
    )
    .translate(
        -LEAK_TRAY_X / 2.0 + LEAK_TRAY_WALL / 2.0,
        0.0,
        LEAK_TRAY_Z / 2.0 + 4.0,
    );
    let right = centered_cube(
        "luer_misassembly_leak_witness_right_rim",
        LEAK_TRAY_WALL,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z + 8.0,
    )
    .translate(
        LEAK_TRAY_X / 2.0 - LEAK_TRAY_WALL / 2.0,
        0.0,
        LEAK_TRAY_Z / 2.0 + 4.0,
    );
    front + rear + left + right
}

fn witness_pads() -> Part {
    let mut pads = Part::empty("luer_misassembly_leak_witness_absorbent_pads");
    for pad in 0..WITNESS_PAD_COUNT {
        let col = pad % WITNESS_PAD_COLS;
        let row = pad / WITNESS_PAD_COLS;
        pads = pads
            + centered_cube(
                format!("luer_misassembly_leak_witness_pad_{pad}"),
                112.0,
                44.0,
                3.0,
            )
            .translate(
                lane_position(col, WITNESS_PAD_COLS, 172.0) - 86.0,
                lane_position(row, WITNESS_PAD_COUNT / WITNESS_PAD_COLS, 108.0) + 8.0,
                LEAK_TRAY_Z + 1.5,
            );
    }
    pads
}

fn witness_flow_channels() -> Part {
    let mut channels = Part::empty("luer_misassembly_leak_witness_flow_channels");
    for route in 0..ROUTE_COUNT {
        channels = channels
            + centered_cube(
                format!("luer_misassembly_leak_witness_route_channel_{route}"),
                812.0,
                4.0,
                4.0,
            )
            .translate(
                0.0,
                lane_position(route, ROUTE_COUNT, 76.0),
                LEAK_TRAY_Z + 2.0,
            );
    }
    channels
}

fn keyed_connector_nests() -> Part {
    let base = centered_cube(
        "luer_misassembly_keyed_connector_nest_bank",
        NEST_BANK_X,
        NEST_BANK_Y,
        NEST_BANK_Z,
    )
    .translate(0.0, 0.0, NEST_BANK_Z / 2.0);
    let mut cuts = Part::empty("luer_misassembly_keyed_connector_nest_cuts");
    let mut features = Part::empty("luer_misassembly_keyed_connector_nest_features");

    for route in 0..ROUTE_COUNT {
        let y = lane_position(route, ROUTE_COUNT, ROUTE_LANE_PITCH_Y);
        features = features
            + route_lane_rail(route, y)
            + route_start_flag(route, y)
            + wrong_way_stop(route, y);
        for nest in 0..NESTS_PER_ROUTE {
            let x = lane_position(nest, NESTS_PER_ROUTE, NEST_PITCH_X);
            cuts = cuts
                + luer_pocket_cut(route, nest, x, y)
                + keyed_orientation_slot(route, nest, x, y)
                + cap_thread_relief_slot(route, nest, x, y);
            features = features
                + luer_socket_boss(route, nest, x, y)
                + key_clocking_witness(route, nest, x, y)
                + route_color_bar(route, nest, x, y);
        }
    }

    let front_fence = centered_cube(
        "luer_misassembly_keyed_nest_front_fence",
        NEST_BANK_X - 24.0,
        10.0,
        32.0,
    )
    .translate(0.0, -NEST_BANK_Y / 2.0 + 18.0, NEST_BANK_Z + 16.0);

    base - cuts + features + front_fence
}

fn route_lane_rail(route: usize, y: f64) -> Part {
    let rear = centered_cube(
        format!("luer_misassembly_route_{route}_rear_tubing_lane_rail"),
        NEST_BANK_X - 58.0,
        5.0,
        12.0,
    )
    .translate(0.0, y + 22.0, NEST_BANK_Z + 6.0);
    let front = centered_cube(
        format!("luer_misassembly_route_{route}_front_tubing_lane_rail"),
        NEST_BANK_X - 58.0,
        5.0,
        12.0,
    )
    .translate(0.0, y - 22.0, NEST_BANK_Z + 6.0);
    rear + front
}

fn route_start_flag(route: usize, y: f64) -> Part {
    centered_cube(
        format!("luer_misassembly_route_{route}_start_reference_flag"),
        20.0,
        44.0,
        30.0,
    )
    .translate(-NEST_BANK_X / 2.0 + 34.0, y, NEST_BANK_Z + 15.0)
}

fn wrong_way_stop(route: usize, y: f64) -> Part {
    let top = centered_cube(
        format!("luer_misassembly_route_{route}_wrong_way_top_stop"),
        28.0,
        12.0,
        26.0,
    )
    .translate(NEST_BANK_X / 2.0 - 36.0, y + 15.0, NEST_BANK_Z + 13.0);
    let bottom = centered_cube(
        format!("luer_misassembly_route_{route}_wrong_way_bottom_stop"),
        28.0,
        12.0,
        26.0,
    )
    .translate(NEST_BANK_X / 2.0 - 36.0, y - 15.0, NEST_BANK_Z + 13.0);
    top + bottom
}

fn luer_pocket_cut(route: usize, nest: usize, x: f64, y: f64) -> Part {
    centered_cylinder(
        format!("luer_misassembly_route_{route}_nest_{nest}_luer_pocket"),
        LUER_POCKET_D / 2.0,
        LUER_POCKET_DEPTH,
        36,
    )
    .translate(x, y, NEST_BANK_Z - LUER_POCKET_DEPTH / 2.0 + 1.0)
}

fn keyed_orientation_slot(route: usize, nest: usize, x: f64, y: f64) -> Part {
    centered_cube(
        format!("luer_misassembly_route_{route}_nest_{nest}_key_slot"),
        KEY_SLOT_W,
        KEY_SLOT_L,
        LUER_POCKET_DEPTH + 2.0,
    )
    .translate(
        0.0,
        LUER_POCKET_D / 2.0 + KEY_SLOT_L / 2.0 - 1.5,
        NEST_BANK_Z - LUER_POCKET_DEPTH / 2.0 + 1.0,
    )
    .rotate(0.0, 0.0, key_angle(route, nest))
    .translate(x, y, 0.0)
}

fn cap_thread_relief_slot(route: usize, nest: usize, x: f64, y: f64) -> Part {
    centered_cube(
        format!("luer_misassembly_route_{route}_nest_{nest}_thread_relief_slot"),
        23.0,
        5.4,
        12.0,
    )
    .rotate(0.0, 0.0, key_angle(route, nest) + 90.0)
    .translate(x, y, NEST_BANK_Z - 6.0)
}

fn luer_socket_boss(route: usize, nest: usize, x: f64, y: f64) -> Part {
    let outer = centered_cylinder(
        format!("luer_misassembly_route_{route}_nest_{nest}_socket_outer_boss"),
        27.0 / 2.0,
        6.0,
        40,
    )
    .translate(x, y, NEST_BANK_Z + 3.0);
    let inner = centered_cylinder(
        format!("luer_misassembly_route_{route}_nest_{nest}_socket_boss_center_clearance"),
        (LUER_POCKET_D + 1.0) / 2.0,
        8.0,
        36,
    )
    .translate(x, y, NEST_BANK_Z + 3.0);
    outer - inner
}

fn key_clocking_witness(route: usize, nest: usize, x: f64, y: f64) -> Part {
    centered_cube(
        format!("luer_misassembly_route_{route}_nest_{nest}_raised_key_witness"),
        KEY_BOSS_W,
        KEY_BOSS_L,
        KEY_BOSS_Z,
    )
    .translate(
        0.0,
        LUER_POCKET_D / 2.0 + KEY_BOSS_L / 2.0 + 5.0,
        NEST_BANK_Z + KEY_BOSS_Z / 2.0,
    )
    .rotate(0.0, 0.0, key_angle(route, nest))
    .translate(x, y, 0.0)
}

fn route_color_bar(route: usize, nest: usize, x: f64, y: f64) -> Part {
    let bar_y = if nest % 2 == 0 { -31.0 } else { 31.0 };
    centered_cube(
        format!("luer_misassembly_route_{route}_nest_{nest}_route_identity_bar"),
        42.0 - route as f64 * 4.0,
        5.0,
        4.0,
    )
    .translate(x, y + bar_y, NEST_BANK_Z + 2.0)
}

fn key_angle(route: usize, nest: usize) -> f64 {
    -90.0 + route as f64 * 45.0 + nest as f64 * 12.0
}

fn go_nogo_gauges() -> Part {
    let base = centered_cube(
        "luer_misassembly_go_nogo_gauge_bank",
        GAUGE_BANK_X,
        GAUGE_BANK_Y,
        GAUGE_BANK_Z,
    )
    .translate(0.0, 0.0, GAUGE_BANK_Z / 2.0);
    let mut cuts = Part::empty("luer_misassembly_go_nogo_gauge_cuts");
    let mut features = Part::empty("luer_misassembly_go_nogo_gauge_features");

    for route in 0..GAUGE_ROWS {
        let y = lane_position(route, GAUGE_ROWS, GAUGE_PAIR_PITCH_Y);
        features = features + gauge_route_spine(route, y);
        for family in 0..GAUGE_COLS {
            let x = lane_position(family, GAUGE_COLS, GAUGE_PAIR_PITCH_X);
            cuts = cuts + gauge_family_cuts(route, family, x, y);
            features = features + gauge_family_bosses(route, family, x, y);
        }
    }

    let master_slot = centered_cube(
        "luer_misassembly_go_nogo_gauge_master_slot",
        GAUGE_BANK_X - 52.0,
        18.0,
        10.0,
    )
    .translate(0.0, GAUGE_BANK_Y / 2.0 - 23.0, GAUGE_BANK_Z - 5.0);

    base - cuts - master_slot + features
}

fn gauge_route_spine(route: usize, y: f64) -> Part {
    centered_cube(
        format!("luer_misassembly_gauge_route_{route}_spine"),
        GAUGE_BANK_X - 36.0,
        3.0,
        5.0,
    )
    .translate(0.0, y - GAUGE_PAIR_PITCH_Y / 2.0 + 4.0, GAUGE_BANK_Z + 2.5)
}

fn gauge_family_cuts(route: usize, family: usize, x: f64, y: f64) -> Part {
    let go_d = 7.4 + family as f64 * 1.2;
    let nogo_d = go_d - 0.8;
    let go = centered_cylinder(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_go_bore"),
        go_d / 2.0,
        GAUGE_BANK_Z + 2.0,
        32,
    )
    .translate(x - 17.0, y, GAUGE_BANK_Z / 2.0);
    let nogo = centered_cylinder(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_nogo_bore"),
        nogo_d / 2.0,
        18.0,
        32,
    )
    .translate(x + 17.0, y, GAUGE_BANK_Z - 9.0);
    let thread_step = centered_cube(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_thread_height_step"),
        54.0,
        5.0,
        9.0,
    )
    .translate(x, y + 16.0, GAUGE_BANK_Z - 4.5);
    go + nogo + thread_step
}

fn gauge_family_bosses(route: usize, family: usize, x: f64, y: f64) -> Part {
    let go_ring = gauge_ring(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_go_ring"),
        22.0,
        8.0,
        x - 17.0,
        y,
    );
    let nogo_ring = gauge_ring(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_nogo_ring"),
        18.0,
        6.0,
        x + 17.0,
        y,
    );
    let stop_flag = centered_cube(
        format!("luer_misassembly_route_{route}_gauge_family_{family}_nogo_stop_flag"),
        11.0,
        28.0,
        13.0,
    )
    .translate(x + 35.0, y, GAUGE_BANK_Z + 6.5);
    go_ring + nogo_ring + stop_flag
}

fn gauge_ring(name: String, outer_d: f64, height: f64, x: f64, y: f64) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), outer_d / 2.0, height, 32).translate(
        x,
        y,
        GAUGE_BANK_Z + height / 2.0,
    );
    let inner = centered_cylinder(
        format!("{name}_inner_clearance"),
        outer_d / 3.0,
        height + 2.0,
        28,
    )
    .translate(x, y, GAUGE_BANK_Z + height / 2.0);
    outer - inner
}

fn barcode_route_verification_pads() -> Part {
    let panel = centered_cube(
        "luer_misassembly_barcode_route_verification_panel",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    )
    .translate(0.0, 0.0, SCAN_PANEL_Z / 2.0);
    let mut pads = Part::empty("luer_misassembly_route_verification_pads");

    for land in 0..ROUTE_SCAN_LANDS {
        let route = land / 3;
        let lane = land % 3;
        let x = lane_position(land, ROUTE_SCAN_LANDS, 66.0);
        let y = match lane {
            0 => 24.0,
            1 => 0.0,
            _ => -24.0,
        };
        pads = pads
            + centered_cube(
                format!("luer_misassembly_route_{route}_scan_land_{lane}"),
                SCAN_LAND_X,
                SCAN_LAND_Y,
                SCAN_LAND_Z,
            )
            .translate(x, y, SCAN_PANEL_Z + SCAN_LAND_Z / 2.0);
    }

    for land in 0..OPERATOR_VERIFY_LANDS {
        pads = pads
            + centered_cube(
                format!("luer_misassembly_operator_lot_verify_pad_{land}"),
                74.0,
                26.0,
                SCAN_LAND_Z,
            )
            .translate(
                lane_position(land, OPERATOR_VERIFY_LANDS, 86.0),
                SCAN_PANEL_Y / 2.0 - 18.0,
                SCAN_PANEL_Z + SCAN_LAND_Z / 2.0,
            );
    }

    let scanner_window = centered_cube(
        "luer_misassembly_route_verification_scanner_window_cutout",
        SCAN_PANEL_X - 76.0,
        12.0,
        SCAN_PANEL_Z + 2.0,
    )
    .translate(0.0, -SCAN_PANEL_Y / 2.0 + 24.0, SCAN_PANEL_Z / 2.0);

    panel - scanner_window + pads + scan_fences()
}

fn scan_fences() -> Part {
    let left = centered_cube(
        "luer_misassembly_route_verification_left_document_fence",
        8.0,
        SCAN_PANEL_Y + 18.0,
        22.0,
    )
    .translate(-SCAN_PANEL_X / 2.0 + 16.0, 0.0, SCAN_PANEL_Z + 11.0);
    let right = centered_cube(
        "luer_misassembly_route_verification_right_document_fence",
        8.0,
        SCAN_PANEL_Y + 18.0,
        22.0,
    )
    .translate(SCAN_PANEL_X / 2.0 - 16.0, 0.0, SCAN_PANEL_Z + 11.0);
    left + right
}

fn capped_uncapped_segregation() -> Part {
    let shell = centered_cube(
        "luer_misassembly_capped_uncapped_segregation_bin",
        SEG_BIN_X,
        SEG_BIN_Y,
        SEG_BIN_Z,
    )
    .translate(0.0, 0.0, SEG_BIN_Z / 2.0);
    let cap_basin = centered_cube(
        "luer_misassembly_capped_parts_basin",
        SEG_BIN_X / 2.0 - 28.0,
        SEG_BIN_Y - 42.0,
        SEG_BIN_Z - 12.0,
    )
    .translate(-SEG_BIN_X / 4.0, 0.0, SEG_BIN_Z - (SEG_BIN_Z - 12.0) / 2.0);
    let uncapped_basin = centered_cube(
        "luer_misassembly_uncapped_parts_basin",
        SEG_BIN_X / 2.0 - 28.0,
        SEG_BIN_Y - 42.0,
        SEG_BIN_Z - 12.0,
    )
    .translate(SEG_BIN_X / 4.0, 0.0, SEG_BIN_Z - (SEG_BIN_Z - 12.0) / 2.0);
    let divider = centered_cube(
        "luer_misassembly_capped_uncapped_high_center_divider",
        BIN_WALL,
        SEG_BIN_Y - 20.0,
        SEG_BIN_Z + 34.0,
    )
    .translate(0.0, 0.0, SEG_BIN_Z / 2.0 + 17.0);

    shell - cap_basin - uncapped_basin - cap_well_cuts() - uncapped_slot_cuts()
        + divider
        + reject_lane()
}

fn cap_well_cuts() -> Part {
    let mut cuts = Part::empty("luer_misassembly_cap_well_cuts");
    for well in 0..CAP_WELL_COUNT {
        let col = well % CAP_WELL_COLS;
        let row = well / CAP_WELL_COLS;
        cuts = cuts
            + centered_cylinder(
                format!("luer_misassembly_capped_state_cap_well_{well}"),
                8.2 / 2.0,
                22.0,
                24,
            )
            .translate(
                -SEG_BIN_X / 4.0 + lane_position(col, CAP_WELL_COLS, 22.0),
                lane_position(row, CAP_WELL_COUNT / CAP_WELL_COLS, 30.0),
                SEG_BIN_Z - 11.0,
            );
    }
    cuts
}

fn uncapped_slot_cuts() -> Part {
    let mut cuts = Part::empty("luer_misassembly_uncapped_luer_slot_cuts");
    for slot in 0..UNCAPPED_SLOTS {
        let col = slot % 3;
        let row = slot / 3;
        cuts = cuts
            + centered_cube(
                format!("luer_misassembly_uncapped_luer_slot_{slot}"),
                56.0,
                12.0,
                24.0,
            )
            .translate(
                SEG_BIN_X / 4.0 + lane_position(col, 3, 60.0),
                lane_position(row, 4, 34.0),
                SEG_BIN_Z - 12.0,
            );
    }
    cuts
}

fn reject_lane() -> Part {
    centered_cube(
        "luer_misassembly_cap_state_reject_quarantine_lane",
        SEG_BIN_X - 40.0,
        20.0,
        18.0,
    )
    .translate(0.0, -SEG_BIN_Y / 2.0 + 26.0, SEG_BIN_Z + 9.0)
}

fn torque_witness_scale() -> Part {
    let base = centered_cube(
        "luer_misassembly_torque_witness_scale_base",
        TORQUE_BASE_X,
        TORQUE_BASE_Y,
        TORQUE_BASE_Z,
    )
    .translate(0.0, 0.0, TORQUE_BASE_Z / 2.0);
    let pivot_clearance = centered_cylinder(
        "luer_misassembly_torque_witness_pivot_clearance",
        TORQUE_PIVOT_D / 2.0,
        TORQUE_BASE_Z + 4.0,
        40,
    )
    .translate(0.0, -22.0, TORQUE_BASE_Z / 2.0);
    let tool_dock = centered_cylinder(
        "luer_misassembly_torque_tool_dock_socket",
        TORQUE_TOOL_DOCK_D / 2.0,
        20.0,
        36,
    )
    .translate(
        TORQUE_BASE_X / 2.0 - 62.0,
        -TORQUE_BASE_Y / 2.0 + 54.0,
        TORQUE_BASE_Z - 10.0,
    );
    let handle_channel = centered_cube("luer_misassembly_torque_handle_channel", 210.0, 20.0, 12.0)
        .translate(24.0, -22.0, TORQUE_BASE_Z - 6.0);

    base - pivot_clearance - tool_dock - handle_channel
        + torque_arc_ticks()
        + torque_pivot_ring()
        + torque_stop_blocks()
        + torque_witness_pointer()
}

fn torque_arc_ticks() -> Part {
    let mut ticks = Part::empty("luer_misassembly_torque_witness_arc_ticks");
    for tick in 0..TORQUE_TICKS {
        let angle = -66.0 + tick as f64 * 11.0;
        let is_major = tick % 3 == 0;
        let tick_len = if is_major { 32.0 } else { 20.0 };
        let tick_w = if is_major { 5.0 } else { 3.0 };
        ticks = ticks
            + centered_cube(
                format!("luer_misassembly_torque_witness_tick_{tick}"),
                tick_w,
                tick_len,
                6.0,
            )
            .translate(0.0, TORQUE_RADIUS, TORQUE_BASE_Z + 3.0)
            .rotate(0.0, 0.0, angle);
    }
    ticks
}

fn torque_pivot_ring() -> Part {
    let outer = centered_cylinder(
        "luer_misassembly_torque_pivot_outer_ring",
        (TORQUE_PIVOT_D + 24.0) / 2.0,
        8.0,
        42,
    )
    .translate(0.0, -22.0, TORQUE_BASE_Z + 4.0);
    let inner = centered_cylinder(
        "luer_misassembly_torque_pivot_inner_clearance",
        TORQUE_PIVOT_D / 2.0,
        10.0,
        40,
    )
    .translate(0.0, -22.0, TORQUE_BASE_Z + 4.0);
    outer - inner
}

fn torque_stop_blocks() -> Part {
    let under = centered_cube(
        "luer_misassembly_torque_under_torque_stop",
        22.0,
        54.0,
        32.0,
    )
    .translate(-112.0, 34.0, TORQUE_BASE_Z + 16.0)
    .rotate(0.0, 0.0, -26.0);
    let over = centered_cube("luer_misassembly_torque_over_torque_stop", 22.0, 54.0, 32.0)
        .translate(112.0, 34.0, TORQUE_BASE_Z + 16.0)
        .rotate(0.0, 0.0, 26.0);
    under + over
}

fn torque_witness_pointer() -> Part {
    centered_cube(
        "luer_misassembly_torque_nominal_window_pointer",
        12.0,
        86.0,
        9.0,
    )
    .translate(0.0, 60.0, TORQUE_BASE_Z + 4.5)
}

fn route_harness_reference_frame() -> Part {
    let left_post = centered_cube(
        "luer_misassembly_route_frame_left_post",
        24.0,
        ROUTE_FRAME_Y,
        ROUTE_FRAME_Z,
    )
    .translate(-ROUTE_FRAME_X / 2.0 + 12.0, 0.0, ROUTE_FRAME_Z / 2.0);
    let right_post = centered_cube(
        "luer_misassembly_route_frame_right_post",
        24.0,
        ROUTE_FRAME_Y,
        ROUTE_FRAME_Z,
    )
    .translate(ROUTE_FRAME_X / 2.0 - 12.0, 0.0, ROUTE_FRAME_Z / 2.0);
    let rear_rail = centered_cube(
        "luer_misassembly_route_frame_rear_rail",
        ROUTE_FRAME_X,
        18.0,
        ROUTE_FRAME_Z,
    )
    .translate(0.0, ROUTE_FRAME_Y / 2.0 - 9.0, ROUTE_FRAME_Z / 2.0);
    let front_rail = centered_cube(
        "luer_misassembly_route_frame_front_low_rail",
        ROUTE_FRAME_X,
        12.0,
        20.0,
    )
    .translate(0.0, -ROUTE_FRAME_Y / 2.0 + 6.0, 10.0);

    left_post + right_post + rear_rail + front_rail + tubing_reference_channels()
}

fn tubing_reference_channels() -> Part {
    let mut channels = Part::empty("luer_misassembly_route_frame_tubing_reference_channels");
    for route in 0..ROUTE_COUNT {
        let y = lane_position(route, ROUTE_COUNT, 24.0);
        for channel in 0..TUBING_CHANNELS_PER_ROUTE {
            channels = channels
                + centered_cylinder(
                    format!("luer_misassembly_route_{route}_tube_reference_channel_{channel}"),
                    TUBING_CHANNEL_D / 2.0,
                    ROUTE_FRAME_X - 86.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(
                    0.0,
                    y + lane_position(channel, TUBING_CHANNELS_PER_ROUTE, 8.0),
                    ROUTE_FRAME_Z - 11.0,
                );
        }
    }
    channels
}

fn csg_labels() -> Part {
    let mut labels = Part::empty("luer_misassembly_station_csg_labels");

    labels = labels
        + label_at(
            "luer_label_keyed_nests",
            "KEYED NESTS",
            232.0,
            34.0,
            NEST_BANK_CENTER.0 - 94.0,
            NEST_BANK_CENTER.1 + NEST_BANK_Y / 2.0 - 28.0,
            WORK_Z + NEST_BANK_Z,
        )
        + label_at(
            "luer_label_go_no_go",
            "GO NO GO",
            188.0,
            34.0,
            GAUGE_BANK_CENTER.0,
            GAUGE_BANK_CENTER.1 + GAUGE_BANK_Y / 2.0 - 29.0,
            WORK_Z + GAUGE_BANK_Z,
        )
        + label_at(
            "luer_label_scan_route",
            "SCAN ROUTE",
            216.0,
            34.0,
            SCAN_PANEL_CENTER.0,
            SCAN_PANEL_CENTER.1 + SCAN_PANEL_Y / 2.0 - 20.0,
            WORK_Z + SCAN_PANEL_Z,
        )
        + label_at(
            "luer_label_leak_witness",
            "LEAK WITNESS",
            238.0,
            34.0,
            -36.0,
            16.0,
            DECK_Z + LEAK_TRAY_Z + 3.0,
        )
        + label_at(
            "luer_label_caps",
            "CAPS",
            94.0,
            32.0,
            SEG_BIN_CENTER.0 - 82.0,
            SEG_BIN_CENTER.1 + SEG_BIN_Y / 2.0 - 30.0,
            WORK_Z + SEG_BIN_Z,
        )
        + label_at(
            "luer_label_uncapped",
            "UNCAPPED",
            166.0,
            32.0,
            SEG_BIN_CENTER.0 + 88.0,
            SEG_BIN_CENTER.1 + SEG_BIN_Y / 2.0 - 30.0,
            WORK_Z + SEG_BIN_Z,
        )
        + label_at(
            "luer_label_torque",
            "TORQUE",
            142.0,
            32.0,
            TORQUE_CENTER.0,
            TORQUE_CENTER.1 + TORQUE_BASE_Y / 2.0 - 28.0,
            WORK_Z + TORQUE_BASE_Z,
        )
        + label_at(
            "luer_label_wrong_way",
            "WRONG WAY",
            190.0,
            32.0,
            NEST_BANK_CENTER.0 + NEST_BANK_X / 2.0 - 98.0,
            NEST_BANK_CENTER.1 - NEST_BANK_Y / 2.0 + 30.0,
            WORK_Z + NEST_BANK_Z,
        );

    for route in 0..ROUTE_COUNT {
        labels = labels
            + label_at(
                format!("luer_label_route_{}", route_letter(route)),
                format!("ROUTE {}", route_letter(route)),
                132.0,
                30.0,
                ROUTE_FRAME_CENTER.0 + lane_position(route, ROUTE_COUNT, 190.0),
                ROUTE_FRAME_CENTER.1,
                WORK_Z + ROUTE_FRAME_Z,
            );
    }

    labels
}

fn label_at<N, T>(name: N, text: T, plate_x: f64, plate_y: f64, x: f64, y: f64, z: f64) -> Part
where
    N: Into<String>,
    T: AsRef<str>,
{
    let name = name.into();
    let plate = centered_cube(format!("{name}_plate"), plate_x, plate_y, LABEL_PLATE_Z).translate(
        0.0,
        0.0,
        LABEL_PLATE_Z / 2.0,
    );
    let text = block_text(format!("{name}_block_text"), text.as_ref()).translate(
        0.0,
        0.0,
        LABEL_PLATE_Z + LABEL_TEXT_Z / 2.0,
    );
    (plate + text).translate(x, y, z)
}

fn block_text<N: Into<String>>(name: N, text: &str) -> Part {
    let name = name.into();
    let mut part = Part::empty(format!("{name}_glyphs"));
    let total_width = text_width(text);
    let mut cursor = -total_width / 2.0;

    for (char_index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            cursor += 3.0 * LABEL_PIXEL;
            continue;
        }

        let pattern = glyph_pattern(ch);
        for (row, line) in pattern.iter().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                if cell == '#' {
                    let x = cursor + col as f64 * LABEL_PIXEL + LABEL_PIXEL / 2.0;
                    let y = (3.0 - row as f64) * LABEL_PIXEL;
                    part = part
                        + centered_cube(
                            format!("{name}_{char_index}_{row}_{col}"),
                            LABEL_PIXEL * 0.84,
                            LABEL_PIXEL * 0.84,
                            LABEL_TEXT_Z,
                        )
                        .translate(x, y, 0.0);
                }
            }
        }
        cursor += 6.0 * LABEL_PIXEL;
    }

    part
}

fn text_width(text: &str) -> f64 {
    text.chars()
        .map(|ch| {
            if ch == ' ' {
                3.0 * LABEL_PIXEL
            } else {
                6.0 * LABEL_PIXEL
            }
        })
        .sum()
}

fn glyph_pattern(ch: char) -> [&'static str; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [
            ".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
        'B' => [
            "####.", "#...#", "#...#", "####.", "#...#", "#...#", "####.",
        ],
        'C' => [
            ".####", "#....", "#....", "#....", "#....", "#....", ".####",
        ],
        'D' => [
            "####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####.",
        ],
        'E' => [
            "#####", "#....", "#....", "####.", "#....", "#....", "#####",
        ],
        'G' => [
            ".####", "#....", "#....", "#.###", "#...#", "#...#", ".###.",
        ],
        'I' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "#####",
        ],
        'K' => [
            "#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#",
        ],
        'L' => [
            "#....", "#....", "#....", "#....", "#....", "#....", "#####",
        ],
        'N' => [
            "#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#",
        ],
        'O' => [
            ".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###.",
        ],
        'P' => [
            "####.", "#...#", "#...#", "####.", "#....", "#....", "#....",
        ],
        'Q' => [
            ".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#",
        ],
        'R' => [
            "####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#",
        ],
        'S' => [
            ".####", "#....", "#....", ".###.", "....#", "....#", "####.",
        ],
        'T' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#..",
        ],
        'U' => [
            "#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###.",
        ],
        'W' => [
            "#...#", "#...#", "#...#", "#.#.#", "#.#.#", "##.##", "#...#",
        ],
        'Y' => [
            "#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#..",
        ],
        _ => [
            "#####", "....#", "...#.", "..#..", ".#...", ".....", ".#...",
        ],
    }
}

fn robot_service_keepouts() -> Part {
    let robot_front = keepout_frame(
        "luer_misassembly_front_robot_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        -102.0,
        -DECK_Y / 2.0 + ROBOT_KEEP_OUT_Y / 2.0 + 36.0,
    );
    let right_service = keepout_frame(
        "luer_misassembly_right_service_drawer_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 36.0,
        0.0,
    );
    let scan_clearance = centered_cube(
        "luer_misassembly_scan_bridge_clearance_gauge",
        SCAN_PANEL_X - 90.0,
        14.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(
        SCAN_PANEL_CENTER.0,
        SCAN_PANEL_CENTER.1 - SCAN_PANEL_Y / 2.0 - 28.0,
        WORK_Z + SCAN_PANEL_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    robot_front + right_service + scan_clearance
}

fn keepout_frame(name: &str, x: f64, y: f64, cx: f64, cy: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, 6.0, KEEP_OUT_RAIL_Z).translate(
        cx,
        cy - y / 2.0,
        WORK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear"), x, 6.0, KEEP_OUT_RAIL_Z).translate(
        cx,
        cy + y / 2.0,
        WORK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let left = centered_cube(format!("{name}_left"), 6.0, y, KEEP_OUT_RAIL_Z).translate(
        cx - x / 2.0,
        cy,
        WORK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    let right = centered_cube(format!("{name}_right"), 6.0, y, KEEP_OUT_RAIL_Z).translate(
        cx + x / 2.0,
        cy,
        WORK_Z + KEEP_OUT_RAIL_Z / 2.0,
    );
    front + rear + left + right
}

fn route_letter(route: usize) -> char {
    match route {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        _ => 'D',
    }
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert!(LEAK_TRAY_X + 2.0 * DECK_RIM_W < DECK_X);
    assert!(LEAK_TRAY_Y + 2.0 * DECK_RIM_W < DECK_Y);
    assert!(ROUTE_COUNT * NESTS_PER_ROUTE >= 20);
    assert!(CAP_WELL_COUNT > UNCAPPED_SLOTS);
    assert!(TORQUE_TICKS % 2 == 1);
}
