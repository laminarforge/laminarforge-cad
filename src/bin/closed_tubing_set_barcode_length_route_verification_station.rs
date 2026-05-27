use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed tubing-set identity, route, and length verification station.
//
// Intent:
// - Verify closed tubing set identity before installation using barcode and RFID
//   evidence lands.
// - Confirm route and cut length mechanically with gauge channels, bend-radius
//   combs, connector endpoint nests, and misroute witness pockets.
// - Segregate clean and used tubing paths, contain drips in a leak tray, and
//   put process labels into the exported CSG geometry as raised block strokes.
//
// This models validation datums, witnesses, labels, and containment geometry.
// It is not pressure-rated tubing hardware.

const OUTPUTS: [&str; 12] = [
    "output/closed_tubing_set_barcode_length_route_verification_station_deck.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_scan_lands.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_length_gauge_channels.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_bend_radius_route_combs.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_connector_endpoint_nests.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_misroute_witness_pockets.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_clean_used_segregation.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_leak_tray.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_csg_labels.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_route_window_bridge.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_robot_service_keepouts.stl",
    "output/closed_tubing_set_barcode_length_route_verification_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "barcode_scan_land",
    "rfid_scan_land",
    "length_gauge_channels",
    "bend_radius_route_combs",
    "connector_endpoint_nests",
    "misroute_witness_pockets",
    "clean_used_segregation",
    "leak_tray",
    "csg_labels",
    "route_window_bridge",
    "assembly_export",
];

const DECK_X: f64 = 1260.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 18.0;
const DECK_CORNER_R: f64 = 18.0;

const SCAN_PANEL_X: f64 = 372.0;
const SCAN_PANEL_Y: f64 = 128.0;
const SCAN_PANEL_Z: f64 = 16.0;
const SCAN_CENTER_X: f64 = -390.0;
const SCAN_CENTER_Y: f64 = 300.0;
const BARCODE_LAND_X: f64 = 190.0;
const BARCODE_LAND_Y: f64 = 46.0;
const RFID_LAND_X: f64 = 116.0;
const RFID_LAND_Y: f64 = 74.0;

const ROUTE_COUNT: usize = 4;
const ROUTE_NAMES: [&str; ROUTE_COUNT] = ["media", "wash", "waste", "sample"];
const ROUTE_LABELS: [&str; ROUTE_COUNT] = ["MEDIA", "WASH", "WASTE", "SAMPLE"];
const ROUTE_LENGTHS_MM: [f64; ROUTE_COUNT] = [420.0, 520.0, 680.0, 840.0];
const ROUTE_OD_MM: [f64; ROUTE_COUNT] = [6.4, 6.4, 8.0, 4.8];

const LENGTH_PLATE_X: f64 = 1000.0;
const LENGTH_PLATE_Y: f64 = 244.0;
const LENGTH_PLATE_Z: f64 = 20.0;
const LENGTH_CENTER_X: f64 = -70.0;
const LENGTH_CENTER_Y: f64 = 106.0;
const LENGTH_LANE_PITCH_Y: f64 = 52.0;
const LENGTH_CHANNEL_CLEARANCE: f64 = 2.2;
const LENGTH_CHANNEL_DEPTH: f64 = 13.0;
#[cfg(test)]
const MIN_LENGTH_GAUGE_MM: f64 = 400.0;

const COMB_PLATE_X: f64 = 442.0;
const COMB_PLATE_Y: f64 = 336.0;
const COMB_PLATE_Z: f64 = 18.0;
const COMB_CENTER_X: f64 = 360.0;
const COMB_CENTER_Y: f64 = 84.0;
const BEND_RADII_MM: [f64; ROUTE_COUNT] = [32.0, 42.0, 58.0, 76.0];
const COMB_TOOTH_W: f64 = 9.0;
const COMB_TOOTH_Y: f64 = 62.0;
const COMB_TOOTH_Z: f64 = 38.0;

const NEST_PLATE_X: f64 = 430.0;
const NEST_PLATE_Y: f64 = 236.0;
const NEST_PLATE_Z: f64 = 24.0;
const NEST_CENTER_X: f64 = 397.0;
const NEST_CENTER_Y: f64 = -236.0;
const ENDPOINTS_PER_ROUTE: usize = 2;
const ENDPOINT_NEST_COUNT: usize = ROUTE_COUNT * ENDPOINTS_PER_ROUTE;
const NEST_PITCH_X: f64 = 92.0;
const NEST_PITCH_Y: f64 = 78.0;
const CONNECTOR_D: f64 = 21.0;

const WITNESS_PLATE_X: f64 = 330.0;
const WITNESS_PLATE_Y: f64 = 214.0;
const WITNESS_PLATE_Z: f64 = 18.0;
const WITNESS_CENTER_X: f64 = -440.0;
const WITNESS_CENTER_Y: f64 = -212.0;
const WITNESS_ROWS: usize = 2;
const WITNESS_COLS: usize = 4;
const WITNESS_POCKET_X: f64 = 52.0;
const WITNESS_POCKET_Y: f64 = 44.0;
const WITNESS_POCKET_DEPTH: f64 = 12.0;

const SEG_BASE_X: f64 = 342.0;
const SEG_BASE_Y: f64 = 192.0;
const SEG_BASE_Z: f64 = 22.0;
const SEG_CENTER_X: f64 = -44.0;
const SEG_CENTER_Y: f64 = -282.0;
const SEG_BIN_X: f64 = 136.0;
const SEG_BIN_Y: f64 = 142.0;
const SEG_WALL_Z: f64 = 58.0;
const SEGREGATION_GAP_MM: f64 = 42.0;

const LEAK_TRAY_X: f64 = 420.0;
const LEAK_TRAY_Y: f64 = 150.0;
const LEAK_TRAY_Z: f64 = 30.0;
const LEAK_TRAY_CENTER_X: f64 = 332.0;
const LEAK_TRAY_CENTER_Y: f64 = 302.0;
const LEAK_TRAY_WALL: f64 = 9.0;
const LEAK_SENSOR_WELLS: usize = 6;

const LABEL_PLATE_X: f64 = 566.0;
const LABEL_PLATE_Y: f64 = 88.0;
const LABEL_PLATE_Z: f64 = 10.0;
const LABEL_CENTER_X: f64 = -34.0;
const LABEL_CENTER_Y: f64 = -390.0;
const LABEL_STROKE_Z: f64 = 2.4;

const ROUTE_BRIDGE_X: f64 = 960.0;
const ROUTE_BRIDGE_Y: f64 = 68.0;
const ROUTE_BRIDGE_POST_X: f64 = 24.0;
const ROUTE_BRIDGE_POST_Y: f64 = 52.0;
const ROUTE_BRIDGE_CLEARANCE_Z: f64 = 104.0;
const ROUTE_BRIDGE_BEAM_Z: f64 = 20.0;
const ROUTE_WINDOWS: usize = 5;

const ROBOT_KEEP_OUT_X: f64 = 206.0;
const ROBOT_KEEP_OUT_Y: f64 = 616.0;
const ROBOT_KEEP_OUT_Z: f64 = 138.0;
const SERVICE_KEEP_OUT_X: f64 = 980.0;
const SERVICE_KEEP_OUT_Y: f64 = 88.0;
const SERVICE_KEEP_OUT_Z: f64 = 92.0;
const KEEP_OUT_RAIL: f64 = 6.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let scan = barcode_rfid_scan_lands();
    export(OUTPUTS[1], &scan);

    let length = length_gauge_channels();
    export(OUTPUTS[2], &length);

    let combs = bend_radius_route_combs();
    export(OUTPUTS[3], &combs);

    let nests = connector_endpoint_nests();
    export(OUTPUTS[4], &nests);

    let witnesses = misroute_witness_pockets();
    export(OUTPUTS[5], &witnesses);

    let segregation = clean_used_segregation();
    export(OUTPUTS[6], &segregation);

    let leak = leak_tray();
    export(OUTPUTS[7], &leak);

    let labels = csg_label_plate();
    export(OUTPUTS[8], &labels);

    let bridge = route_window_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        deck + scan.translate(
            SCAN_CENTER_X,
            SCAN_CENTER_Y,
            DECK_Z / 2.0 + SCAN_PANEL_Z / 2.0,
        ) + length.translate(
            LENGTH_CENTER_X,
            LENGTH_CENTER_Y,
            DECK_Z / 2.0 + LENGTH_PLATE_Z / 2.0,
        ) + combs.translate(
            COMB_CENTER_X,
            COMB_CENTER_Y,
            DECK_Z / 2.0 + COMB_PLATE_Z / 2.0,
        ) + nests.translate(
            NEST_CENTER_X,
            NEST_CENTER_Y,
            DECK_Z / 2.0 + NEST_PLATE_Z / 2.0,
        ) + witnesses.translate(
            WITNESS_CENTER_X,
            WITNESS_CENTER_Y,
            DECK_Z / 2.0 + WITNESS_PLATE_Z / 2.0,
        ) + segregation.translate(SEG_CENTER_X, SEG_CENTER_Y, DECK_Z / 2.0 + SEG_BASE_Z / 2.0)
            + leak.translate(
                LEAK_TRAY_CENTER_X,
                LEAK_TRAY_CENTER_Y,
                DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0,
            )
            + labels.translate(
                LABEL_CENTER_X,
                LABEL_CENTER_Y,
                DECK_Z / 2.0 + LABEL_PLATE_Z / 2.0,
            )
            + bridge.translate(
                LENGTH_CENTER_X,
                LENGTH_CENTER_Y,
                DECK_Z / 2.0 + ROUTE_BRIDGE_CLEARANCE_Z / 2.0,
            )
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!(
        "Closed tubing set barcode/length/route verification station: {:.0}mm x {:.0}mm deck, {} route length gauge channels ({:.0}-{:.0}mm), {} bend-radius combs ({:.0}-{:.0}mm radius), {} endpoint nests, {} misroute witness pockets, clean/used segregation gap {:.0}mm, {} leak sensor wells, and raised CSG labels.",
        DECK_X,
        DECK_Y,
        ROUTE_COUNT,
        shortest_route_length(),
        longest_route_length(),
        ROUTE_COUNT,
        smallest_bend_radius(),
        largest_bend_radius(),
        ENDPOINT_NEST_COUNT,
        WITNESS_ROWS * WITNESS_COLS,
        SEGREGATION_GAP_MM,
        LEAK_SENSOR_WELLS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_tubing_set_barcode_length_route_verification_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let corner_reliefs = corner_cylinders(
        "closed_tubing_set_barcode_length_route_deck_corner_relief",
        DECK_X - 2.0 * DECK_CORNER_R,
        DECK_Y - 2.0 * DECK_CORNER_R,
        DECK_CORNER_R / 2.0,
        DECK_Z + 4.0,
    );

    let sockets = deck_component_sockets();
    let mounts = deck_mount_holes();

    deck - corner_reliefs - sockets - mounts + deck_perimeter_lips() + deck_datum_pads()
}

fn deck_component_sockets() -> Part {
    let specs = [
        (
            "scan_panel",
            SCAN_CENTER_X,
            SCAN_CENTER_Y,
            SCAN_PANEL_X + 24.0,
            SCAN_PANEL_Y + 20.0,
            6.0,
        ),
        (
            "length_gauge",
            LENGTH_CENTER_X,
            LENGTH_CENTER_Y,
            LENGTH_PLATE_X + 28.0,
            LENGTH_PLATE_Y + 24.0,
            6.0,
        ),
        (
            "route_comb",
            COMB_CENTER_X,
            COMB_CENTER_Y,
            COMB_PLATE_X + 22.0,
            COMB_PLATE_Y + 22.0,
            5.0,
        ),
        (
            "endpoint_nests",
            NEST_CENTER_X,
            NEST_CENTER_Y,
            NEST_PLATE_X + 22.0,
            NEST_PLATE_Y + 22.0,
            6.0,
        ),
        (
            "witness_pockets",
            WITNESS_CENTER_X,
            WITNESS_CENTER_Y,
            WITNESS_PLATE_X + 22.0,
            WITNESS_PLATE_Y + 22.0,
            5.0,
        ),
        (
            "segregation_bin",
            SEG_CENTER_X,
            SEG_CENTER_Y,
            SEG_BASE_X + 22.0,
            SEG_BASE_Y + 20.0,
            6.0,
        ),
        (
            "leak_tray",
            LEAK_TRAY_CENTER_X,
            LEAK_TRAY_CENTER_Y,
            LEAK_TRAY_X + 22.0,
            LEAK_TRAY_Y + 20.0,
            7.0,
        ),
        (
            "csg_labels",
            LABEL_CENTER_X,
            LABEL_CENTER_Y,
            LABEL_PLATE_X + 18.0,
            LABEL_PLATE_Y + 14.0,
            4.0,
        ),
    ];

    let mut sockets = Part::empty("closed_tubing_set_verification_deck_component_sockets");
    for (name, x, y, sx, sy, depth) in specs {
        sockets = sockets
            + centered_cube(
                format!("closed_tubing_set_verification_deck_{name}_socket"),
                sx,
                sy,
                depth,
            )
            .translate(x, y, DECK_Z / 2.0 - depth / 2.0 + 0.2);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_tubing_set_verification_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_tubing_set_verification_m6_mount_hole_{i}"),
                6.6 / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "closed_tubing_set_verification_deck_rear_splash_lip",
        DECK_X - 110.0,
        16.0,
        26.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 30.0, DECK_Z / 2.0 + 13.0);
    let left = centered_cube(
        "closed_tubing_set_verification_deck_left_locator_lip",
        16.0,
        DECK_Y - 124.0,
        24.0,
    )
    .translate(-DECK_X / 2.0 + 30.0, -4.0, DECK_Z / 2.0 + 12.0);
    let front = centered_cube(
        "closed_tubing_set_verification_deck_front_low_stop",
        DECK_X - 260.0,
        10.0,
        13.0,
    )
    .translate(-20.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0 + 6.5);
    rear + left + front
}

fn deck_datum_pads() -> Part {
    let mut pads = Part::empty("closed_tubing_set_verification_deck_hardened_datum_pads");
    for (i, (x, y)) in [
        (
            LENGTH_CENTER_X - LENGTH_PLATE_X / 2.0 + 46.0,
            LENGTH_CENTER_Y + LENGTH_PLATE_Y / 2.0 - 32.0,
        ),
        (
            LENGTH_CENTER_X + LENGTH_PLATE_X / 2.0 - 46.0,
            LENGTH_CENTER_Y + LENGTH_PLATE_Y / 2.0 - 32.0,
        ),
        (
            NEST_CENTER_X - NEST_PLATE_X / 2.0 + 34.0,
            NEST_CENTER_Y - NEST_PLATE_Y / 2.0 + 34.0,
        ),
        (
            NEST_CENTER_X + NEST_PLATE_X / 2.0 - 34.0,
            NEST_CENTER_Y - NEST_PLATE_Y / 2.0 + 34.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        pads = pads
            + centered_cylinder(
                format!("closed_tubing_set_verification_datum_pad_{i}"),
                18.0 / 2.0,
                5.0,
                32,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 2.5);
    }
    pads
}

fn barcode_rfid_scan_lands() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_barcode_rfid_scan_land_plate",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    );

    let barcode_land = centered_cube(
        "closed_tubing_set_barcode_label_flat_land",
        BARCODE_LAND_X,
        BARCODE_LAND_Y,
        4.0,
    )
    .translate(-58.0, -20.0, SCAN_PANEL_Z / 2.0 + 2.0);
    let barcode_slot = centered_cube(
        "closed_tubing_set_barcode_scan_window_cut",
        BARCODE_LAND_X - 28.0,
        16.0,
        SCAN_PANEL_Z + 4.0,
    )
    .translate(-58.0, -20.0, 1.0);
    let rfid_land = centered_cube(
        "closed_tubing_set_rfid_antenna_flat_land",
        RFID_LAND_X,
        RFID_LAND_Y,
        4.0,
    )
    .translate(SCAN_PANEL_X / 2.0 - 76.0, 5.0, SCAN_PANEL_Z / 2.0 + 2.0);
    let rfid_pocket = centered_cube(
        "closed_tubing_set_rfid_antenna_relief_pocket",
        RFID_LAND_X - 28.0,
        RFID_LAND_Y - 24.0,
        7.0,
    )
    .translate(SCAN_PANEL_X / 2.0 - 76.0, 5.0, SCAN_PANEL_Z / 2.0 - 2.5);

    let barcode_fences = side_fence_pair(
        "closed_tubing_set_barcode",
        BARCODE_LAND_X + 24.0,
        BARCODE_LAND_Y + 18.0,
        7.0,
        20.0,
    )
    .translate(-58.0, -20.0, SCAN_PANEL_Z / 2.0 + 10.0);
    let rfid_coil_marks = rfid_coil_markers();
    let label = csg_label("SCAN RFID", 6.0, 1.15, LABEL_STROKE_Z).translate(
        -SCAN_PANEL_X / 2.0 + 22.0,
        SCAN_PANEL_Y / 2.0 - 26.0,
        SCAN_PANEL_Z / 2.0 + 3.2,
    );

    plate + barcode_land + rfid_land + barcode_fences + rfid_coil_marks + label
        - barcode_slot
        - rfid_pocket
}

fn rfid_coil_markers() -> Part {
    let mut coil = Part::empty("closed_tubing_set_rfid_nested_coil_markers");
    for i in 0..4 {
        let sx = RFID_LAND_X - 20.0 - i as f64 * 18.0;
        let sy = RFID_LAND_Y - 18.0 - i as f64 * 12.0;
        coil = coil
            + rectangular_outline(
                format!("closed_tubing_set_rfid_coil_outline_{i}"),
                sx,
                sy,
                2.0,
                2.1,
            )
            .translate(SCAN_PANEL_X / 2.0 - 76.0, 5.0, SCAN_PANEL_Z / 2.0 + 4.2);
    }
    coil
}

fn length_gauge_channels() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_length_gauge_channel_plate",
        LENGTH_PLATE_X,
        LENGTH_PLATE_Y,
        LENGTH_PLATE_Z,
    );

    let mut channel_cuts = Part::empty("closed_tubing_set_length_gauge_channel_cuts");
    let mut stops = Part::empty("closed_tubing_set_length_gauge_end_stops");
    let mut ticks = Part::empty("closed_tubing_set_length_gauge_tick_marks");
    let mut route_labels = Part::empty("closed_tubing_set_length_gauge_route_labels");

    for route in 0..ROUTE_COUNT {
        let y = route_y(route);
        let length = ROUTE_LENGTHS_MM[route];
        let od = ROUTE_OD_MM[route] + LENGTH_CHANNEL_CLEARANCE;
        let channel = centered_cylinder(
            format!(
                "closed_tubing_set_{}_length_gauge_half_round_channel",
                ROUTE_NAMES[route]
            ),
            od / 2.0,
            length,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, LENGTH_PLATE_Z / 2.0 - LENGTH_CHANNEL_DEPTH / 2.0);
        let top_slot = centered_cube(
            format!(
                "closed_tubing_set_{}_length_top_access_slot",
                ROUTE_NAMES[route]
            ),
            length,
            od * 0.88,
            LENGTH_CHANNEL_DEPTH + 6.0,
        )
        .translate(
            0.0,
            y,
            LENGTH_PLATE_Z / 2.0 - LENGTH_CHANNEL_DEPTH / 2.0 + 3.0,
        );
        channel_cuts = channel_cuts + channel + top_slot;

        let left_stop = centered_cube(
            format!(
                "closed_tubing_set_{}_length_left_zero_stop",
                ROUTE_NAMES[route]
            ),
            10.0,
            od + 14.0,
            28.0,
        )
        .translate(-length / 2.0 - 8.0, y, LENGTH_PLATE_Z / 2.0 + 14.0);
        let right_stop = centered_cube(
            format!(
                "closed_tubing_set_{}_length_right_cut_stop",
                ROUTE_NAMES[route]
            ),
            10.0,
            od + 14.0,
            28.0,
        )
        .translate(length / 2.0 + 8.0, y, LENGTH_PLATE_Z / 2.0 + 14.0);
        stops = stops + left_stop + right_stop;

        ticks = ticks + length_tick_train(route, length, y, od);
        route_labels = route_labels
            + csg_label(ROUTE_LABELS[route], 4.8, 0.9, LABEL_STROKE_Z).translate(
                -LENGTH_PLATE_X / 2.0 + 26.0,
                y - 13.0,
                LENGTH_PLATE_Z / 2.0 + 3.0,
            );
    }

    let title = csg_label("LENGTH GAUGE", 7.0, 1.2, LABEL_STROKE_Z).translate(
        -LENGTH_PLATE_X / 2.0 + 34.0,
        LENGTH_PLATE_Y / 2.0 - 28.0,
        LENGTH_PLATE_Z / 2.0 + 3.2,
    );

    plate + stops + ticks + route_labels + title - channel_cuts
}

fn length_tick_train(route: usize, length: f64, y: f64, od: f64) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_tubing_set_{}_length_gauge_ticks",
        ROUTE_NAMES[route]
    ));
    let divisions = 5;
    for i in 0..=divisions {
        let x = -length / 2.0 + i as f64 * length / divisions as f64;
        let tick_h = if i == 0 || i == divisions { 15.0 } else { 10.0 };
        ticks = ticks
            + centered_cube(
                format!("closed_tubing_set_{}_length_tick_{}", ROUTE_NAMES[route], i),
                3.0,
                od + tick_h,
                3.2,
            )
            .translate(x, y, LENGTH_PLATE_Z / 2.0 + 1.8);
    }
    ticks
}

fn bend_radius_route_combs() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_bend_radius_route_comb_plate",
        COMB_PLATE_X,
        COMB_PLATE_Y,
        COMB_PLATE_Z,
    );

    let mut radius_disks = Part::empty("closed_tubing_set_bend_radius_reference_disks");
    let mut route_teeth = Part::empty("closed_tubing_set_bend_radius_route_comb_teeth");
    let mut channel_cuts = Part::empty("closed_tubing_set_bend_radius_tube_lane_cuts");
    let mut labels = Part::empty("closed_tubing_set_bend_radius_route_comb_labels");

    for route in 0..ROUTE_COUNT {
        let x = -((ROUTE_COUNT as f64 - 1.0) * 96.0) / 2.0 + route as f64 * 96.0;
        let radius = BEND_RADII_MM[route];
        let disk = centered_cylinder(
            format!(
                "closed_tubing_set_{}_minimum_bend_radius_reference_disk",
                ROUTE_NAMES[route]
            ),
            radius,
            8.0,
            54,
        )
        .translate(x, 54.0, COMB_PLATE_Z / 2.0 + 4.0);
        let relief = centered_cylinder(
            format!(
                "closed_tubing_set_{}_bend_radius_inner_visual_relief",
                ROUTE_NAMES[route]
            ),
            radius - 9.0,
            10.0,
            54,
        )
        .translate(x, 54.0, COMB_PLATE_Z / 2.0 + 4.5);
        radius_disks = radius_disks + (disk - relief);

        for tooth in 0..4 {
            let tx = x - 30.0 + tooth as f64 * 20.0;
            route_teeth = route_teeth
                + centered_cube(
                    format!(
                        "closed_tubing_set_{}_comb_tooth_{}",
                        ROUTE_NAMES[route], tooth
                    ),
                    COMB_TOOTH_W,
                    COMB_TOOTH_Y,
                    COMB_TOOTH_Z,
                )
                .translate(tx, -86.0, COMB_PLATE_Z / 2.0 + COMB_TOOTH_Z / 2.0);
        }

        let lane = centered_cylinder(
            format!(
                "closed_tubing_set_{}_comb_lane_radius_clearance_cut",
                ROUTE_NAMES[route]
            ),
            (ROUTE_OD_MM[route] + 2.0) / 2.0,
            112.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -78.0, COMB_PLATE_Z / 2.0 + 13.0);
        channel_cuts = channel_cuts + lane;

        labels = labels
            + csg_label(ROUTE_LABELS[route], 4.4, 0.8, LABEL_STROKE_Z).translate(
                x - 34.0,
                -COMB_PLATE_Y / 2.0 + 26.0,
                COMB_PLATE_Z / 2.0 + 2.8,
            );
    }

    let title = csg_label("ROUTE RADIUS", 6.3, 1.1, LABEL_STROKE_Z).translate(
        -COMB_PLATE_X / 2.0 + 28.0,
        COMB_PLATE_Y / 2.0 - 24.0,
        COMB_PLATE_Z / 2.0 + 3.0,
    );
    let spine = centered_cube(
        "closed_tubing_set_bend_radius_comb_shared_back_spine",
        COMB_PLATE_X - 42.0,
        12.0,
        36.0,
    )
    .translate(0.0, -116.0, COMB_PLATE_Z / 2.0 + 18.0);

    plate + radius_disks + route_teeth + labels + title + spine - channel_cuts
}

fn connector_endpoint_nests() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_connector_endpoint_nest_plate",
        NEST_PLATE_X,
        NEST_PLATE_Y,
        NEST_PLATE_Z,
    );

    let mut cutouts = Part::empty("closed_tubing_set_connector_endpoint_nest_cutouts");
    let mut bosses = Part::empty("closed_tubing_set_connector_endpoint_nest_bosses");
    let mut latch_flags = Part::empty("closed_tubing_set_connector_endpoint_latch_flags");
    let mut labels = Part::empty("closed_tubing_set_connector_endpoint_nest_csg_labels");

    for route in 0..ROUTE_COUNT {
        for endpoint in 0..ENDPOINTS_PER_ROUTE {
            let idx = route * ENDPOINTS_PER_ROUTE + endpoint;
            let (x, y) = endpoint_center(idx);
            let boss = centered_cylinder(
                format!(
                    "closed_tubing_set_{}_endpoint_{}_nest_boss",
                    ROUTE_NAMES[route], endpoint
                ),
                34.0 / 2.0,
                8.0,
                36,
            )
            .translate(x, y, NEST_PLATE_Z / 2.0 + 4.0);
            let keyed_socket = keyed_connector_socket(route, endpoint).translate(x, y, 2.0);
            let flag = centered_cube(
                format!(
                    "closed_tubing_set_{}_endpoint_{}_orientation_flag",
                    ROUTE_NAMES[route], endpoint
                ),
                12.0,
                26.0,
                24.0,
            )
            .translate(
                x + if endpoint == 0 { -26.0 } else { 26.0 },
                y,
                NEST_PLATE_Z / 2.0 + 12.0,
            );
            let text = if endpoint == 0 { "A" } else { "B" };
            let label = csg_label(text, 5.4, 1.0, LABEL_STROKE_Z).translate(
                x - 7.0,
                y + 27.0,
                NEST_PLATE_Z / 2.0 + 3.2,
            );
            bosses = bosses + boss;
            cutouts = cutouts + keyed_socket;
            latch_flags = latch_flags + flag;
            labels = labels + label;
        }
    }

    let title = csg_label("ENDPOINT NESTS", 5.8, 1.05, LABEL_STROKE_Z).translate(
        -NEST_PLATE_X / 2.0 + 28.0,
        NEST_PLATE_Y / 2.0 - 25.0,
        NEST_PLATE_Z / 2.0 + 3.0,
    );

    plate + bosses + latch_flags + labels + title - cutouts
}

fn keyed_connector_socket(route: usize, endpoint: usize) -> Part {
    let round = centered_cylinder(
        format!(
            "closed_tubing_set_{}_endpoint_{}_connector_round_socket",
            ROUTE_NAMES[route], endpoint
        ),
        CONNECTOR_D / 2.0,
        NEST_PLATE_Z + 8.0,
        32,
    );
    let key_x = match route {
        0 => -11.0,
        1 => -4.0,
        2 => 5.0,
        _ => 12.0,
    };
    let key = centered_cube(
        format!(
            "closed_tubing_set_{}_endpoint_{}_asymmetric_keyway",
            ROUTE_NAMES[route], endpoint
        ),
        8.0,
        20.0,
        NEST_PLATE_Z + 10.0,
    )
    .translate(key_x, if endpoint == 0 { -10.0 } else { 10.0 }, 0.0);
    round + key
}

fn misroute_witness_pockets() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_misroute_witness_pocket_plate",
        WITNESS_PLATE_X,
        WITNESS_PLATE_Y,
        WITNESS_PLATE_Z,
    );

    let mut pocket_cuts = Part::empty("closed_tubing_set_misroute_witness_pocket_cuts");
    let mut witness_rims = Part::empty("closed_tubing_set_misroute_witness_raised_rims");
    let mut reset_pins = Part::empty("closed_tubing_set_misroute_witness_reset_pins");
    let mut labels = Part::empty("closed_tubing_set_misroute_witness_labels");

    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let idx = row * WITNESS_COLS + col;
            let x = -((WITNESS_COLS as f64 - 1.0) * 70.0) / 2.0 + col as f64 * 70.0;
            let y = 34.0 - row as f64 * 76.0;
            let cut = centered_cube(
                format!("closed_tubing_set_misroute_witness_pocket_cut_{idx}"),
                WITNESS_POCKET_X,
                WITNESS_POCKET_Y,
                WITNESS_POCKET_DEPTH + 1.0,
            )
            .translate(
                x,
                y,
                WITNESS_PLATE_Z / 2.0 - WITNESS_POCKET_DEPTH / 2.0 + 0.3,
            );
            let rim = rectangular_outline(
                format!("closed_tubing_set_misroute_witness_pocket_rim_{idx}"),
                WITNESS_POCKET_X + 10.0,
                WITNESS_POCKET_Y + 10.0,
                4.0,
                5.0,
            )
            .translate(x, y, WITNESS_PLATE_Z / 2.0 + 2.5);
            let pin = centered_cylinder(
                format!("closed_tubing_set_misroute_witness_reset_pin_{idx}"),
                4.0,
                9.0,
                24,
            )
            .translate(x + 18.0, y - 14.0, WITNESS_PLATE_Z / 2.0 + 4.5);
            pocket_cuts = pocket_cuts + cut;
            witness_rims = witness_rims + rim;
            reset_pins = reset_pins + pin;
        }
    }

    labels = labels
        + csg_label("MISROUTE", 5.4, 1.0, LABEL_STROKE_Z).translate(
            -WITNESS_PLATE_X / 2.0 + 24.0,
            WITNESS_PLATE_Y / 2.0 - 24.0,
            WITNESS_PLATE_Z / 2.0 + 3.0,
        )
        + csg_label("WITNESS", 5.4, 1.0, LABEL_STROKE_Z).translate(
            -WITNESS_PLATE_X / 2.0 + 24.0,
            WITNESS_PLATE_Y / 2.0 - 44.0,
            WITNESS_PLATE_Z / 2.0 + 3.0,
        );

    plate + witness_rims + reset_pins + labels - pocket_cuts
}

fn clean_used_segregation() -> Part {
    let base = centered_cube(
        "closed_tubing_set_clean_used_segregation_base",
        SEG_BASE_X,
        SEG_BASE_Y,
        SEG_BASE_Z,
    );

    let clean_bin = segregation_bin("clean", -SEG_BASE_X / 4.0);
    let used_bin = segregation_bin("used", SEG_BASE_X / 4.0);
    let wall = centered_cube(
        "closed_tubing_set_clean_used_full_height_divider_wall",
        14.0,
        SEG_BASE_Y - 18.0,
        SEG_WALL_Z,
    )
    .translate(0.0, 0.0, SEG_BASE_Z / 2.0 + SEG_WALL_Z / 2.0);
    let one_way_gate = centered_cube(
        "closed_tubing_set_used_side_one_way_tube_drop_gate",
        72.0,
        12.0,
        34.0,
    )
    .translate(
        SEG_BASE_X / 4.0,
        SEG_BASE_Y / 2.0 - 24.0,
        SEG_BASE_Z / 2.0 + 17.0,
    );
    let clean_label = csg_label("CLEAN", 6.0, 1.1, LABEL_STROKE_Z).translate(
        -SEG_BASE_X / 2.0 + 26.0,
        SEG_BASE_Y / 2.0 - 32.0,
        SEG_BASE_Z / 2.0 + 3.0,
    );
    let used_label = csg_label("USED", 6.0, 1.1, LABEL_STROKE_Z).translate(
        SEG_BASE_X / 2.0 - 94.0,
        SEG_BASE_Y / 2.0 - 32.0,
        SEG_BASE_Z / 2.0 + 3.0,
    );

    base + clean_bin + used_bin + wall + one_way_gate + clean_label + used_label
}

fn segregation_bin(name: &str, x: f64) -> Part {
    let basin_cut = centered_cube(
        format!("closed_tubing_set_{name}_segregation_bin_basin_cut"),
        SEG_BIN_X,
        SEG_BIN_Y,
        SEG_BASE_Z + 4.0,
    )
    .translate(x, -8.0, SEG_BASE_Z / 2.0 + 2.0);
    let rim = rectangular_outline(
        format!("closed_tubing_set_{name}_segregation_bin_raised_rim"),
        SEG_BIN_X + 16.0,
        SEG_BIN_Y + 16.0,
        7.0,
        18.0,
    )
    .translate(x, -8.0, SEG_BASE_Z / 2.0 + 9.0);
    rim - basin_cut
}

fn leak_tray() -> Part {
    let tray = centered_cube(
        "closed_tubing_set_leak_tray_outer_body",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let basin = centered_cube(
        "closed_tubing_set_leak_tray_sloped_basin_relief",
        LEAK_TRAY_X - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Y - 2.0 * LEAK_TRAY_WALL,
        LEAK_TRAY_Z + 4.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0 - 8.0);
    let drain = centered_cylinder(
        "closed_tubing_set_leak_tray_drain_sump_cut",
        15.0,
        LEAK_TRAY_Z + 8.0,
        36,
    )
    .translate(LEAK_TRAY_X / 2.0 - 46.0, -LEAK_TRAY_Y / 2.0 + 34.0, 0.0);

    let mut wells = Part::empty("closed_tubing_set_leak_tray_sensor_wells");
    let mut channels = Part::empty("closed_tubing_set_leak_tray_capillary_channels");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = -((LEAK_SENSOR_WELLS as f64 - 1.0) * 54.0) / 2.0 + i as f64 * 54.0;
        wells = wells
            + centered_cylinder(
                format!("closed_tubing_set_leak_sensor_well_{i}"),
                8.0,
                8.0,
                28,
            )
            .translate(x, 28.0, LEAK_TRAY_Z / 2.0 + 4.0);
        channels = channels
            + centered_cube(
                format!("closed_tubing_set_leak_tray_capillary_channel_{i}"),
                6.0,
                72.0,
                5.0,
            )
            .translate(x, -24.0, LEAK_TRAY_Z / 2.0 - 1.5);
    }

    let label = csg_label("LEAK TRAY", 6.2, 1.1, LABEL_STROKE_Z).translate(
        -LEAK_TRAY_X / 2.0 + 28.0,
        LEAK_TRAY_Y / 2.0 - 28.0,
        LEAK_TRAY_Z / 2.0 + 3.0,
    );

    tray + wells + label - basin - drain - channels
}

fn csg_label_plate() -> Part {
    let plate = centered_cube(
        "closed_tubing_set_csg_label_plate",
        LABEL_PLATE_X,
        LABEL_PLATE_Y,
        LABEL_PLATE_Z,
    );
    let title = csg_label("TUBING SET VERIFY", 6.6, 1.05, LABEL_STROKE_Z).translate(
        -LABEL_PLATE_X / 2.0 + 24.0,
        20.0,
        LABEL_PLATE_Z / 2.0 + 3.0,
    );
    let subtitle = csg_label("SCAN LENGTH ROUTE", 5.2, 0.9, LABEL_STROKE_Z).translate(
        -LABEL_PLATE_X / 2.0 + 26.0,
        -22.0,
        LABEL_PLATE_Z / 2.0 + 3.0,
    );
    let witness_strip = rectangular_outline(
        "closed_tubing_set_csg_label_plate_wipeable_frame",
        LABEL_PLATE_X - 26.0,
        LABEL_PLATE_Y - 20.0,
        4.0,
        4.0,
    )
    .translate(0.0, 0.0, LABEL_PLATE_Z / 2.0 + 2.0);

    plate + title + subtitle + witness_strip
}

fn route_window_bridge() -> Part {
    let post_z = ROUTE_BRIDGE_CLEARANCE_Z + ROUTE_BRIDGE_BEAM_Z;
    let left_post = centered_cube(
        "closed_tubing_set_route_window_bridge_left_post",
        ROUTE_BRIDGE_POST_X,
        ROUTE_BRIDGE_POST_Y,
        post_z,
    )
    .translate(-ROUTE_BRIDGE_X / 2.0 + ROUTE_BRIDGE_POST_X / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_tubing_set_route_window_bridge_right_post",
        ROUTE_BRIDGE_POST_X,
        ROUTE_BRIDGE_POST_Y,
        post_z,
    )
    .translate(ROUTE_BRIDGE_X / 2.0 - ROUTE_BRIDGE_POST_X / 2.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_tubing_set_route_window_bridge_camera_beam",
        ROUTE_BRIDGE_X,
        ROUTE_BRIDGE_Y,
        ROUTE_BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, post_z / 2.0 - ROUTE_BRIDGE_BEAM_Z / 2.0);

    let mut windows = Part::empty("closed_tubing_set_route_window_bridge_view_windows");
    for i in 0..ROUTE_WINDOWS {
        let x = -((ROUTE_WINDOWS as f64 - 1.0) * 156.0) / 2.0 + i as f64 * 156.0;
        windows = windows
            + centered_cube(
                format!("closed_tubing_set_route_window_bridge_view_window_{i}"),
                92.0,
                ROUTE_BRIDGE_Y + 6.0,
                ROUTE_BRIDGE_BEAM_Z + 4.0,
            )
            .translate(x, 0.0, post_z / 2.0 - ROUTE_BRIDGE_BEAM_Z / 2.0);
    }

    let label = csg_label("ROUTE OK", 5.2, 0.9, LABEL_STROKE_Z)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -ROUTE_BRIDGE_X / 2.0 + 48.0,
            -ROUTE_BRIDGE_Y / 2.0 - 2.2,
            post_z / 2.0 - 10.0,
        );

    left_post + right_post + (beam - windows) + label
}

fn robot_service_keepouts() -> Part {
    let left_robot = keepout_outline(
        "closed_tubing_set_left_robot_pick_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + 126.0, -10.0, 0.0);
    let rear_service = keepout_outline(
        "closed_tubing_set_rear_service_scanner_cable_keepout",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(42.0, DECK_Y / 2.0 - 70.0, -18.0);
    let scanner_clearance = keepout_outline(
        "closed_tubing_set_overhead_scanner_sweep_keepout",
        760.0,
        190.0,
        86.0,
    )
    .translate(-50.0, 100.0, 34.0);
    left_robot + rear_service + scanner_clearance
}

fn keepout_outline(name: &str, sx: f64, sy: f64, sz: f64) -> Part {
    let base_z = KEEP_OUT_RAIL;
    let front = centered_cube(format!("{name}_front_rail"), sx, KEEP_OUT_RAIL, base_z).translate(
        0.0,
        -sy / 2.0,
        -sz / 2.0 + base_z / 2.0,
    );
    let back = centered_cube(format!("{name}_back_rail"), sx, KEEP_OUT_RAIL, base_z).translate(
        0.0,
        sy / 2.0,
        -sz / 2.0 + base_z / 2.0,
    );
    let left = centered_cube(format!("{name}_left_rail"), KEEP_OUT_RAIL, sy, base_z).translate(
        -sx / 2.0,
        0.0,
        -sz / 2.0 + base_z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), KEEP_OUT_RAIL, sy, base_z).translate(
        sx / 2.0,
        0.0,
        -sz / 2.0 + base_z / 2.0,
    );
    let uprights = corner_cylinders(
        format!("{name}_upright"),
        sx - KEEP_OUT_RAIL,
        sy - KEEP_OUT_RAIL,
        KEEP_OUT_RAIL / 2.0,
        sz,
    );

    front + back + left + right + uprights
}

fn rectangular_outline(name: impl Into<String>, sx: f64, sy: f64, rail: f64, z: f64) -> Part {
    let name = name.into();
    let front = centered_cube(format!("{name}_front"), sx, rail, z).translate(
        0.0,
        -sy / 2.0 + rail / 2.0,
        0.0,
    );
    let back = centered_cube(format!("{name}_back"), sx, rail, z).translate(
        0.0,
        sy / 2.0 - rail / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left"), rail, sy, z).translate(
        -sx / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail, sy, z).translate(
        sx / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    front + back + left + right
}

fn side_fence_pair(name: &str, sx: f64, sy: f64, fence_w: f64, fence_z: f64) -> Part {
    let left = centered_cube(format!("{name}_left_side_fence"), fence_w, sy, fence_z).translate(
        -sx / 2.0 + fence_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_side_fence"), fence_w, sy, fence_z).translate(
        sx / 2.0 - fence_w / 2.0,
        0.0,
        0.0,
    );
    left + right
}

fn csg_label(text: &str, scale: f64, stroke: f64, z: f64) -> Part {
    let mut label = Part::empty(format!("csg_label_{}", sanitize_name(text)));
    let mut cursor = 0.0;
    let step = scale + stroke * 2.5;
    for (i, ch) in text.chars().enumerate() {
        if ch == ' ' {
            cursor += step * 0.75;
            continue;
        }
        label = label + glyph(ch, scale, stroke, z).translate(cursor, 0.0, 0.0);
        cursor += if i + 1 == text.len() { scale } else { step };
    }
    label
}

fn glyph(ch: char, scale: f64, stroke: f64, z: f64) -> Part {
    let pattern = glyph_pattern(ch);
    let cell = scale / 5.0;
    let mut part = Part::empty(format!("csg_glyph_{ch}"));
    for (row, line) in pattern.iter().enumerate() {
        for (col, bit) in line.chars().enumerate() {
            if bit == '1' {
                let x = col as f64 * cell;
                let y = (6 - row) as f64 * cell;
                part = part
                    + centered_cube(
                        format!("csg_glyph_{}_{}_{}", ch, row, col),
                        cell + stroke,
                        cell + stroke,
                        z,
                    )
                    .translate(x, y, 0.0);
            }
        }
    }
    part
}

fn glyph_pattern(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'B' => [
            "11110", "10001", "10001", "11110", "10001", "10001", "11110",
        ],
        'C' => [
            "01111", "10000", "10000", "10000", "10000", "10000", "01111",
        ],
        'D' => [
            "11110", "10001", "10001", "10001", "10001", "10001", "11110",
        ],
        'E' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "11111",
        ],
        'F' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "10000",
        ],
        'G' => [
            "01111", "10000", "10000", "10111", "10001", "10001", "01111",
        ],
        'H' => [
            "10001", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'I' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        'K' => [
            "10001", "10010", "10100", "11000", "10100", "10010", "10001",
        ],
        'L' => [
            "10000", "10000", "10000", "10000", "10000", "10000", "11111",
        ],
        'M' => [
            "10001", "11011", "10101", "10101", "10001", "10001", "10001",
        ],
        'N' => [
            "10001", "11001", "10101", "10011", "10001", "10001", "10001",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'P' => [
            "11110", "10001", "10001", "11110", "10000", "10000", "10000",
        ],
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'S' => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'V' => [
            "10001", "10001", "10001", "10001", "10001", "01010", "00100",
        ],
        'W' => [
            "10001", "10001", "10001", "10101", "10101", "10101", "01010",
        ],
        'Y' => [
            "10001", "10001", "01010", "00100", "00100", "00100", "00100",
        ],
        '0' => [
            "01110", "10001", "10011", "10101", "11001", "10001", "01110",
        ],
        '1' => [
            "00100", "01100", "00100", "00100", "00100", "00100", "01110",
        ],
        '2' => [
            "01110", "10001", "00001", "00010", "00100", "01000", "11111",
        ],
        '3' => [
            "11110", "00001", "00001", "01110", "00001", "00001", "11110",
        ],
        '4' => [
            "10010", "10010", "10010", "11111", "00010", "00010", "00010",
        ],
        '5' => [
            "11111", "10000", "10000", "11110", "00001", "00001", "11110",
        ],
        '6' => [
            "01111", "10000", "10000", "11110", "10001", "10001", "01110",
        ],
        '7' => [
            "11111", "00001", "00010", "00100", "01000", "01000", "01000",
        ],
        '8' => [
            "01110", "10001", "10001", "01110", "10001", "10001", "01110",
        ],
        '9' => [
            "01110", "10001", "10001", "01111", "00001", "00001", "11110",
        ],
        _ => [
            "11111", "10001", "00010", "00100", "00100", "00000", "00100",
        ],
    }
}

fn sanitize_name(text: &str) -> String {
    text.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

fn corner_cylinders(name: impl Into<String>, sx: f64, sy: f64, radius: f64, z: f64) -> Part {
    let name = name.into();
    let mut cylinders = Part::empty(format!("{name}_set"));
    for (i, (x, y)) in [
        (-sx / 2.0, -sy / 2.0),
        (sx / 2.0, -sy / 2.0),
        (-sx / 2.0, sy / 2.0),
        (sx / 2.0, sy / 2.0),
    ]
    .iter()
    .enumerate()
    {
        cylinders = cylinders
            + centered_cylinder(format!("{name}_{i}"), radius, z, 24).translate(*x, *y, 0.0);
    }
    cylinders
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (-DECK_X / 2.0 + 56.0, 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
        (-110.0, DECK_Y / 2.0 - 56.0),
        (110.0, -DECK_Y / 2.0 + 56.0),
    ]
}

fn route_y(route: usize) -> f64 {
    ((ROUTE_COUNT as f64 - 1.0) * LENGTH_LANE_PITCH_Y) / 2.0 - route as f64 * LENGTH_LANE_PITCH_Y
}

fn endpoint_center(index: usize) -> (f64, f64) {
    let col = index % 4;
    let row = index / 4;
    (
        -((4.0 - 1.0) * NEST_PITCH_X) / 2.0 + col as f64 * NEST_PITCH_X,
        NEST_PITCH_Y / 2.0 - row as f64 * NEST_PITCH_Y,
    )
}

fn shortest_route_length() -> f64 {
    ROUTE_LENGTHS_MM
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}

fn longest_route_length() -> f64 {
    ROUTE_LENGTHS_MM
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

fn smallest_bend_radius() -> f64 {
    BEND_RADII_MM.iter().copied().fold(f64::INFINITY, f64::min)
}

fn largest_bend_radius() -> f64 {
    BEND_RADII_MM
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_tubing_set_barcode_length_route_verification_station_"
                ),
                "{path}"
            );
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn feature_set_covers_ticket_scope() {
        assert!(REQUIRED_FEATURES.contains(&"barcode_scan_land"));
        assert!(REQUIRED_FEATURES.contains(&"rfid_scan_land"));
        assert!(REQUIRED_FEATURES.contains(&"length_gauge_channels"));
        assert!(REQUIRED_FEATURES.contains(&"bend_radius_route_combs"));
        assert!(REQUIRED_FEATURES.contains(&"connector_endpoint_nests"));
        assert!(REQUIRED_FEATURES.contains(&"misroute_witness_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"csg_labels"));
    }

    #[test]
    fn route_length_and_radius_arrays_align() {
        assert_eq!(ROUTE_NAMES.len(), ROUTE_COUNT);
        assert_eq!(ROUTE_LABELS.len(), ROUTE_COUNT);
        assert_eq!(ROUTE_LENGTHS_MM.len(), ROUTE_COUNT);
        assert_eq!(ROUTE_OD_MM.len(), ROUTE_COUNT);
        assert_eq!(BEND_RADII_MM.len(), ROUTE_COUNT);
        assert!(shortest_route_length() >= MIN_LENGTH_GAUGE_MM);
        assert!(longest_route_length() <= LENGTH_PLATE_X - 120.0);
        assert!(smallest_bend_radius() >= 5.0 * ROUTE_OD_MM[0]);
    }

    #[test]
    fn endpoint_nests_cover_both_ends_for_every_route() {
        assert_eq!(ENDPOINTS_PER_ROUTE, 2);
        assert_eq!(ENDPOINT_NEST_COUNT, ROUTE_COUNT * ENDPOINTS_PER_ROUTE);
        assert_eq!(ENDPOINT_NEST_COUNT, 8);
        let (left_x, top_y) = endpoint_center(0);
        let (right_x, bottom_y) = endpoint_center(ENDPOINT_NEST_COUNT - 1);
        assert!(left_x.abs() + CONNECTOR_D < NEST_PLATE_X / 2.0);
        assert!(right_x.abs() + CONNECTOR_D < NEST_PLATE_X / 2.0);
        assert!(top_y.abs() + CONNECTOR_D < NEST_PLATE_Y / 2.0);
        assert!(bottom_y.abs() + CONNECTOR_D < NEST_PLATE_Y / 2.0);
    }

    #[test]
    fn segregation_and_leak_controls_are_present() {
        assert!(SEGREGATION_GAP_MM >= 40.0);
        assert!(SEG_WALL_Z > SEG_BASE_Z * 2.0);
        assert!(LEAK_TRAY_WALL >= 8.0);
        assert_eq!(LEAK_SENSOR_WELLS, 6);
        assert_eq!(WITNESS_ROWS * WITNESS_COLS, 8);
    }

    #[test]
    fn component_footprints_fit_on_deck() {
        assert!(SCAN_CENTER_X - SCAN_PANEL_X / 2.0 > -DECK_X / 2.0 + 40.0);
        assert!(LEAK_TRAY_CENTER_Y + LEAK_TRAY_Y / 2.0 < DECK_Y / 2.0 - 20.0);
        assert!(LABEL_CENTER_Y - LABEL_PLATE_Y / 2.0 > -DECK_Y / 2.0 + 8.0);
        assert!(COMB_CENTER_X + COMB_PLATE_X / 2.0 < DECK_X / 2.0 - 22.0);
        assert!(WITNESS_CENTER_X - WITNESS_PLATE_X / 2.0 > -DECK_X / 2.0 + 20.0);
    }
}
