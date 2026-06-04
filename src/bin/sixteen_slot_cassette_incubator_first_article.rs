use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Production-intent first article for the June 30 equipment sprint.
//
// This generator narrows the broad validation CAD set to the custom mechanical
// parts needed for a 16-slot cassette/incubator dock package. It keeps sterile
// wetted paths disposable and treats connector, gasket, sensor, and material
// choices as purchasable interfaces to be finalized by the RFQ/BOM ticket.
//
// A5 integration note: this model now carries the A1-A4 mechanical stack
// decisions as named first-article geometry. It remains an STL fit-check and
// interface package, not a vendor-ready drawing or biological validation claim.

const OUTPUTS: [&str; 7] = [
    "output/sixteen_slot_cassette_lower_carrier.stl",
    "output/sixteen_slot_cassette_lid_clamp.stl",
    "output/sixteen_slot_cassette_window_placeholder.stl",
    "output/sixteen_slot_cassette_gasket_witness_coupon.stl",
    "output/sixteen_slot_incubator_dock_plate.stl",
    "output/sixteen_slot_service_bulkhead_test_block.stl",
    "output/sixteen_slot_cassette_incubator_first_article_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 4;
const SLOT_COUNT: usize = COLS * ROWS;

const CHIP_GUTTER_X: f64 = 7.0;
const CHIP_GUTTER_Y: f64 = 7.0;
const SLOT_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER_X;
const SLOT_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER_Y;
const SLOT_ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * CHIP_GUTTER_X;
const SLOT_ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * CHIP_GUTTER_Y;

const CARRIER_MARGIN_X: f64 = 58.0;
const CARRIER_MARGIN_Y: f64 = 52.0;
const CARRIER_X: f64 = SLOT_ARRAY_X + CARRIER_MARGIN_X * 2.0;
const CARRIER_Y: f64 = SLOT_ARRAY_Y + CARRIER_MARGIN_Y * 2.0;
const CARRIER_Z: f64 = 24.0;
const CHIP_CLEARANCE: f64 = 1.2;
const DRAWING_TARGET_CHIP_CLEARANCE: f64 = 0.8;
const CHIP_POCKET_DEPTH: f64 = 7.0;
const OPTICAL_WINDOW_MARGIN: f64 = 24.0;
const GASKET_LAND_W: f64 = 8.0;
const PERIMETER_GASKET_W: f64 = 12.0;
const PER_SLOT_GASKET_LAND_Z: f64 = 3.0;
const PERIMETER_GASKET_LAND_Z: f64 = 4.0;
const GASKET_FREE_HEIGHT: f64 = 2.40;
const GASKET_TARGET_SQUEEZE: f64 = 0.25;
const GASKET_COMPRESSED_HEIGHT: f64 = GASKET_FREE_HEIGHT * (1.0 - GASKET_TARGET_SQUEEZE);
const GASKET_GUARD_MIN_SQUEEZE: f64 = 0.20;
const GASKET_GUARD_MAX_SQUEEZE: f64 = 0.30;
const GASKET_GUARD_MAX_COMPRESSED_HEIGHT: f64 =
    GASKET_FREE_HEIGHT * (1.0 - GASKET_GUARD_MIN_SQUEEZE);
const GASKET_GUARD_MIN_COMPRESSED_HEIGHT: f64 =
    GASKET_FREE_HEIGHT * (1.0 - GASKET_GUARD_MAX_SQUEEZE);
const GASKET_GROOVE_DEPTH: f64 = 1.82;
const GASKET_GROOVE_W: f64 = 3.20;
const GASKET_ENTRY_BREAK_RADIUS_NOTE: f64 = 0.20;
const SEAL_BAND_RA_MAX_UM: f64 = 1.6;
const CHIP_POCKET_INTERNAL_RADIUS_NOTE: f64 = 1.0;

const LID_X: f64 = CARRIER_X + 18.0;
const LID_Y: f64 = CARRIER_Y + 18.0;
const LID_Z: f64 = 10.0;
const WINDOW_Z: f64 = 3.0;

const DOCK_X: f64 = CARRIER_X + 170.0;
const DOCK_Y: f64 = CARRIER_Y + 150.0;
const DOCK_Z: f64 = 22.0;
const DOCK_RAIL_Z: f64 = 18.0;
const DOCK_RAIL_W: f64 = 16.0;
const SLOT_RECESS_DEPTH: f64 = 5.5;

const BULKHEAD_X: f64 = CARRIER_X + 90.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 76.0;
const BULKHEAD_OFFSET_Y: f64 = DOCK_Y / 2.0 + BULKHEAD_Y / 2.0 + 18.0;

const COUPON_X: f64 = 250.0;
const COUPON_Y: f64 = 118.0;
const COUPON_Z: f64 = 12.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let carrier = lower_carrier();
    export(OUTPUTS[0], &carrier);

    let lid = lid_clamp();
    export(OUTPUTS[1], &lid);

    let window = window_placeholder();
    export(OUTPUTS[2], &window);

    let coupon = gasket_witness_coupon();
    export(OUTPUTS[3], &coupon);

    let dock = incubator_dock_plate();
    export(OUTPUTS[4], &dock);

    let bulkhead = service_bulkhead_test_block();
    export(OUTPUTS[5], &bulkhead);

    let assembly = dock
        + carrier.translate(0.0, 0.0, DOCK_Z / 2.0 + CARRIER_Z / 2.0 + 8.0)
        + lid.translate(0.0, 0.0, DOCK_Z / 2.0 + CARRIER_Z + LID_Z / 2.0 + 11.0)
        + window.translate(
            0.0,
            0.0,
            DOCK_Z / 2.0 + CARRIER_Z + LID_Z + WINDOW_Z / 2.0 + 12.5,
        )
        + bulkhead.translate(0.0, BULKHEAD_OFFSET_Y, DOCK_Z / 2.0 + BULKHEAD_Z / 2.0)
        + coupon.translate(
            -(DOCK_X / 2.0 - COUPON_X / 2.0 - 30.0),
            -(DOCK_Y / 2.0 + COUPON_Y / 2.0 + 38.0),
            COUPON_Z / 2.0,
        );
    export(OUTPUTS[6], &assembly);

    println!();
    println!("16-slot cassette/incubator first article:");
    println!("  Slot map:                {COLS} x {ROWS} ({SLOT_COUNT} slots)");
    println!("  Cassette lower carrier:  {CARRIER_X:.1}mm x {CARRIER_Y:.1}mm x {CARRIER_Z:.1}mm");
    println!("  Lid/clamp frame:         {LID_X:.1}mm x {LID_Y:.1}mm x {LID_Z:.1}mm");
    println!("  Incubator dock plate:    {DOCK_X:.1}mm x {DOCK_Y:.1}mm x {DOCK_Z:.1}mm");
    println!(
        "  Service bulkhead block:  {BULKHEAD_X:.1}mm x {BULKHEAD_Y:.1}mm x {BULKHEAD_Z:.1}mm"
    );
    println!(
        "  Pocket clearance:        {CHIP_CLEARANCE:.2}mm/side CAD fit-check; {DRAWING_TARGET_CHIP_CLEARANCE:.2}mm/side drawing target after chip lot measurement"
    );
    println!(
        "  Gasket compression:      {GASKET_FREE_HEIGHT:.2}mm free height -> {GASKET_COMPRESSED_HEIGHT:.2}mm target ({:.0}% squeeze)",
        GASKET_TARGET_SQUEEZE * 100.0
    );
    println!(
        "  Gasket groove note:      {GASKET_GROOVE_DEPTH:.2}mm depth x {GASKET_GROOVE_W:.2}mm width; seal bands Ra <= {SEAL_BAND_RA_MAX_UM:.1}um"
    );
    println!(
        "  DFM notes:               6061-T651/T6 dry structure, R{CHIP_POCKET_INTERNAL_RADIUS_NOTE:.1}mm+ pocket corners, R{GASKET_ENTRY_BREAK_RADIUS_NOTE:.1}mm+ gasket entry break"
    );
    println!("  Wetted path policy:      disposable tubing/connectors; structural parts are dry fixtures");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn lower_carrier() -> Part {
    let body = centered_cube(
        "sixteen_slot_lower_carrier_body",
        CARRIER_X,
        CARRIER_Y,
        CARRIER_Z,
    );
    let leak_gutter = rectangular_frame(
        "sixteen_slot_lower_carrier_perimeter_leak_gutter",
        CARRIER_X - 28.0,
        CARRIER_Y - 28.0,
        CARRIER_Z + 2.0,
        7.0,
    )
    .translate(0.0, 0.0, CARRIER_Z / 2.0 - 4.0);
    let drain = centered_cylinder("sixteen_slot_lower_carrier_drain_port", 4.0, 32.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(CARRIER_X / 2.0 - 34.0, -CARRIER_Y / 2.0 + 12.0, -2.0);

    body - chip_pockets()
        - optical_windows(CARRIER_Z + 2.0)
        - leak_gutter
        - drain
        - perimeter_mount_holes(
            "sixteen_slot_lower_carrier_m5",
            CARRIER_X,
            CARRIER_Y,
            CARRIER_Z + 2.0,
        )
        + chip_gasket_lands()
        + perimeter_gasket_land()
        + per_slot_compression_stops()
        + perimeter_compression_stops()
        + datum_pin_bosses()
        + slot_label_lands()
        + carrier_condition_id_land()
        + carrier_orientation_marker()
        + carrier_handling_keepout_lands()
        + side_service_reliefs()
}

fn lid_clamp() -> Part {
    let frame_outer = centered_cube("sixteen_slot_lid_clamp_outer_frame", LID_X, LID_Y, LID_Z);
    let center_relief = centered_cube(
        "sixteen_slot_lid_clamp_inner_lightening_relief",
        SLOT_ARRAY_X + 52.0,
        SLOT_ARRAY_Y + 44.0,
        LID_Z + 2.0,
    );

    let gross =
        frame_outer - center_relief - slot_view_openings(LID_Z + 2.0) - lid_fastener_holes()
            + lid_crossbars()
            + lid_alignment_ears()
            + captive_fastener_retainers()
            + lid_window_retention_lip()
            + lid_torque_sequence_tabs();

    gross - lid_gasket_groove_cuts()
}

fn window_placeholder() -> Part {
    let panel = centered_cube(
        "sixteen_slot_retained_window_placeholder_panel",
        SLOT_ARRAY_X + 84.0,
        SLOT_ARRAY_Y + 76.0,
        WINDOW_Z,
    );
    panel
        + window_slot_witness_frames()
        + calibration_fiducials(WINDOW_Z / 2.0 + 1.2)
        + window_retention_tabs()
}

fn gasket_witness_coupon() -> Part {
    let base = centered_cube(
        "sixteen_slot_gasket_witness_coupon_base",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let retain_slot = centered_cube(
        "sixteen_slot_gasket_coupon_retain_sample_slot",
        COUPON_X - 34.0,
        22.0,
        COUPON_Z + 2.0,
    )
    .translate(0.0, -34.0, 0.0);

    base - retain_slot - coupon_gasket_groove_cuts()
        + gasket_squeeze_steps()
        + coupon_label_lands()
        + coupon_compression_stop_lands()
}

fn incubator_dock_plate() -> Part {
    let deck = centered_cube(
        "sixteen_slot_incubator_dock_plate_body",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    deck - slot_recesses()
        - dock_air_bypass_windows()
        - dock_drain_gutters()
        - perimeter_mount_holes(
            "sixteen_slot_incubator_dock_m6",
            DOCK_X,
            DOCK_Y,
            DOCK_Z + 2.0,
        )
        + dock_reference_rails()
        + slot_position_tokens()
        + logger_pockets()
        + robot_lift_lands()
        + dock_leveling_pad_lands()
}

fn service_bulkhead_test_block() -> Part {
    let body = centered_cube(
        "sixteen_slot_service_bulkhead_test_block_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    body - gas_port_cuts() - media_port_cuts() - waste_port_cuts() - sensor_connector_cut()
        + bulkhead_label_strip()
        + tubing_strain_relief_comb()
        + cable_strain_relief_comb()
}

fn chip_pockets() -> Part {
    let mut cuts = Part::empty("sixteen_slot_lower_carrier_chip_pocket_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            cuts = cuts
                + centered_cube(
                    format!("sixteen_slot_lower_carrier_slot_{slot:02}_chip_pocket"),
                    REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
                    CHIP_POCKET_DEPTH,
                )
                .translate(x, y, CARRIER_Z / 2.0 - CHIP_POCKET_DEPTH / 2.0 + 0.2);
        }
    }
    cuts
}

fn optical_windows(height: f64) -> Part {
    let mut cuts = Part::empty("sixteen_slot_optical_window_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            cuts = cuts
                + centered_cube(
                    format!("sixteen_slot_slot_{slot:02}_optical_window_cut"),
                    REVC_CHIP_LENGTH - OPTICAL_WINDOW_MARGIN,
                    REVC_CHIP_WIDTH - OPTICAL_WINDOW_MARGIN,
                    height,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn slot_view_openings(height: f64) -> Part {
    let mut openings = Part::empty("sixteen_slot_lid_slot_view_openings");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            openings = openings
                + centered_cube(
                    format!("sixteen_slot_lid_slot_{slot:02}_view_opening"),
                    REVC_CHIP_LENGTH - 14.0,
                    REVC_CHIP_WIDTH - 14.0,
                    height,
                )
                .translate(x, y, 0.0);
        }
    }
    openings
}

fn chip_gasket_lands() -> Part {
    let mut lands = Part::empty("sixteen_slot_lower_carrier_chip_gasket_lands");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            lands = lands
                + rectangular_frame(
                    &format!("sixteen_slot_slot_{slot:02}_raised_gasket_land"),
                    REVC_CHIP_LENGTH + 14.0,
                    REVC_CHIP_WIDTH + 14.0,
                    PER_SLOT_GASKET_LAND_Z,
                    GASKET_LAND_W,
                )
                .translate(x, y, CARRIER_Z / 2.0 + PER_SLOT_GASKET_LAND_Z / 2.0);
        }
    }
    lands
}

fn perimeter_gasket_land() -> Part {
    rectangular_frame(
        "sixteen_slot_lower_carrier_perimeter_gasket_land",
        SLOT_ARRAY_X + 72.0,
        SLOT_ARRAY_Y + 66.0,
        PERIMETER_GASKET_LAND_Z,
        PERIMETER_GASKET_W,
    )
    .translate(0.0, 0.0, CARRIER_Z / 2.0 + PERIMETER_GASKET_LAND_Z / 2.0)
}

fn per_slot_compression_stops() -> Part {
    let mut stops = Part::empty("sixteen_slot_per_slot_25pct_compression_stops");
    let stop_z = PER_SLOT_GASKET_LAND_Z + GASKET_COMPRESSED_HEIGHT;
    let offset_x = REVC_CHIP_LENGTH / 2.0 + 3.0;
    let offset_y = REVC_CHIP_WIDTH / 2.0 + 3.0;
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            for (corner, sx, sy) in [
                ("front_left", -offset_x, -offset_y),
                ("front_right", offset_x, -offset_y),
                ("rear_left", -offset_x, offset_y),
                ("rear_right", offset_x, offset_y),
            ] {
                stops = stops
                    + centered_cylinder(
                        format!("sixteen_slot_slot_{slot:02}_{corner}_hard_stop_25pct"),
                        2.5,
                        stop_z,
                        24,
                    )
                    .translate(x + sx, y + sy, CARRIER_Z / 2.0 + stop_z / 2.0);
            }
        }
    }
    stops
}

fn perimeter_compression_stops() -> Part {
    let mut stops = Part::empty("sixteen_slot_perimeter_25pct_compression_stops");
    let stop_z = PERIMETER_GASKET_LAND_Z + GASKET_COMPRESSED_HEIGHT;
    let x_edge = (SLOT_ARRAY_X + 72.0) / 2.0 + 10.0;
    let y_edge = (SLOT_ARRAY_Y + 66.0) / 2.0 + 10.0;
    for (i, x) in [-220.0, -110.0, 0.0, 110.0, 220.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_front_stop_{i}_25pct"),
                18.0,
                7.0,
                stop_z,
            )
            .translate(*x, -y_edge, CARRIER_Z / 2.0 + stop_z / 2.0);
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_rear_stop_{i}_25pct"),
                18.0,
                7.0,
                stop_z,
            )
            .translate(*x, y_edge, CARRIER_Z / 2.0 + stop_z / 2.0);
    }
    for (i, y) in [-150.0, -75.0, 0.0, 75.0, 150.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_left_stop_{i}_25pct"),
                7.0,
                18.0,
                stop_z,
            )
            .translate(-x_edge, *y, CARRIER_Z / 2.0 + stop_z / 2.0);
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_right_stop_{i}_25pct"),
                7.0,
                18.0,
                stop_z,
            )
            .translate(x_edge, *y, CARRIER_Z / 2.0 + stop_z / 2.0);
    }
    stops
}

fn datum_pin_bosses() -> Part {
    let mut pins = Part::empty("sixteen_slot_lower_carrier_datum_pin_bosses");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        let boss = centered_cylinder(format!("sixteen_slot_datum_pin_boss_{i}"), 9.0, 6.0, 32)
            .translate(*x, *y, CARRIER_Z / 2.0 + 3.0);
        let bore = centered_cylinder(format!("sixteen_slot_datum_pin_bore_{i}"), 3.0, 8.0, 28)
            .translate(*x, *y, CARRIER_Z / 2.0 + 3.0);
        pins = pins + (boss - bore);
    }
    pins
}

fn slot_label_lands() -> Part {
    let mut lands = Part::empty("sixteen_slot_slot_label_lands");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            lands = lands
                + centered_cube(
                    format!("sixteen_slot_slot_{slot:02}_label_land"),
                    26.0,
                    10.0,
                    2.0,
                )
                .translate(
                    x - REVC_CHIP_LENGTH / 2.0 + 22.0,
                    y + REVC_CHIP_WIDTH / 2.0 - 14.0,
                    CARRIER_Z / 2.0 + 1.0,
                );
        }
    }
    lands
}

fn carrier_condition_id_land() -> Part {
    let barcode = centered_cube(
        "sixteen_slot_carrier_global_condition_id_barcode_land",
        96.0,
        24.0,
        2.0,
    )
    .translate(
        -CARRIER_X / 2.0 + 78.0,
        -CARRIER_Y / 2.0 + 28.0,
        CARRIER_Z / 2.0 + 1.0,
    );
    let human = centered_cube(
        "sixteen_slot_carrier_global_condition_id_text_land",
        118.0,
        14.0,
        1.5,
    )
    .translate(
        -CARRIER_X / 2.0 + 185.0,
        -CARRIER_Y / 2.0 + 27.0,
        CARRIER_Z / 2.0 + 0.75,
    );
    barcode + human
}

fn carrier_orientation_marker() -> Part {
    let x_leg = centered_cube(
        "sixteen_slot_carrier_slot_01_orientation_x_leg",
        34.0,
        5.0,
        3.0,
    )
    .translate(
        -CARRIER_X / 2.0 + 37.0,
        -CARRIER_Y / 2.0 + 54.0,
        CARRIER_Z / 2.0 + 1.5,
    );
    let y_leg = centered_cube(
        "sixteen_slot_carrier_slot_01_orientation_y_leg",
        5.0,
        34.0,
        3.0,
    )
    .translate(
        -CARRIER_X / 2.0 + 22.5,
        -CARRIER_Y / 2.0 + 69.0,
        CARRIER_Z / 2.0 + 1.5,
    );
    x_leg + y_leg
}

fn carrier_handling_keepout_lands() -> Part {
    let front_left = centered_cube(
        "sixteen_slot_carrier_front_left_robot_pickup_keepout_land",
        58.0,
        16.0,
        1.5,
    )
    .translate(-112.0, -CARRIER_Y / 2.0 + 58.0, CARRIER_Z / 2.0 + 0.75);
    let front_right = centered_cube(
        "sixteen_slot_carrier_front_right_robot_pickup_keepout_land",
        58.0,
        16.0,
        1.5,
    )
    .translate(112.0, -CARRIER_Y / 2.0 + 58.0, CARRIER_Z / 2.0 + 0.75);
    let rear_left = centered_cube(
        "sixteen_slot_carrier_rear_left_robot_pickup_keepout_land",
        58.0,
        16.0,
        1.5,
    )
    .translate(-112.0, CARRIER_Y / 2.0 - 58.0, CARRIER_Z / 2.0 + 0.75);
    let rear_right = centered_cube(
        "sixteen_slot_carrier_rear_right_robot_pickup_keepout_land",
        58.0,
        16.0,
        1.5,
    )
    .translate(112.0, CARRIER_Y / 2.0 - 58.0, CARRIER_Z / 2.0 + 0.75);
    front_left + front_right + rear_left + rear_right
}

fn side_service_reliefs() -> Part {
    let left = centered_cube(
        "sixteen_slot_left_tubing_service_relief_land",
        24.0,
        SLOT_ARRAY_Y + 42.0,
        8.0,
    )
    .translate(-(CARRIER_X / 2.0 - 18.0), 0.0, CARRIER_Z / 2.0 + 4.0);
    let right = centered_cube(
        "sixteen_slot_right_tubing_service_relief_land",
        24.0,
        SLOT_ARRAY_Y + 42.0,
        8.0,
    )
    .translate(CARRIER_X / 2.0 - 18.0, 0.0, CARRIER_Z / 2.0 + 4.0);
    left + right
}

fn lid_fastener_holes() -> Part {
    let mut holes = Part::empty("sixteen_slot_lid_fastener_holes");
    for (i, (x, y)) in fastener_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("sixteen_slot_lid_m4_clearance_{i}"),
                2.4,
                LID_Z + 2.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn captive_fastener_retainers() -> Part {
    let mut retainers = Part::empty("sixteen_slot_lid_captive_m4_retainers");
    for (i, (x, y)) in fastener_points().iter().enumerate() {
        let retainer = centered_cylinder(
            format!("sixteen_slot_lid_captive_m4_retainer_witness_{i}"),
            5.4,
            1.6,
            32,
        ) - centered_cylinder(
            format!("sixteen_slot_lid_captive_m4_retainer_clearance_{i}"),
            2.6,
            2.0,
            28,
        );
        retainers = retainers + retainer.translate(*x, *y, LID_Z / 2.0 + 0.8);
    }
    retainers
}

fn lid_gasket_groove_cuts() -> Part {
    let mut grooves = Part::empty("sixteen_slot_lid_axial_face_gasket_groove_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            grooves = grooves
                + rectangular_frame(
                    &format!("sixteen_slot_lid_slot_{slot:02}_gasket_groove_2p4mm"),
                    REVC_CHIP_LENGTH + 14.0 - (GASKET_LAND_W - GASKET_GROOVE_W),
                    REVC_CHIP_WIDTH + 14.0 - (GASKET_LAND_W - GASKET_GROOVE_W),
                    GASKET_GROOVE_DEPTH,
                    GASKET_GROOVE_W,
                )
                .translate(x, y, -LID_Z / 2.0 + GASKET_GROOVE_DEPTH / 2.0 - 0.05);
        }
    }
    grooves
        + rectangular_frame(
            "sixteen_slot_lid_perimeter_gasket_groove_2p4mm",
            SLOT_ARRAY_X + 72.0 - (PERIMETER_GASKET_W - GASKET_GROOVE_W),
            SLOT_ARRAY_Y + 66.0 - (PERIMETER_GASKET_W - GASKET_GROOVE_W),
            GASKET_GROOVE_DEPTH,
            GASKET_GROOVE_W,
        )
        .translate(0.0, 0.0, -LID_Z / 2.0 + GASKET_GROOVE_DEPTH / 2.0 - 0.05)
}

fn lid_window_retention_lip() -> Part {
    rectangular_frame(
        "sixteen_slot_lid_full_panel_window_retention_lip",
        SLOT_ARRAY_X + 96.0,
        SLOT_ARRAY_Y + 88.0,
        1.6,
        6.0,
    )
    .translate(0.0, 0.0, LID_Z / 2.0 + 0.8)
}

fn lid_torque_sequence_tabs() -> Part {
    let mut tabs = Part::empty("sixteen_slot_lid_torque_sequence_tab_lands");
    for (i, (x, y)) in fastener_points().iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_lid_torque_sequence_land_{:02}", i + 1),
                16.0,
                6.0,
                1.0,
            )
            .translate(*x, *y + 11.0, LID_Z / 2.0 + 0.5);
    }
    tabs
}

fn lid_crossbars() -> Part {
    let mut bars = Part::empty("sixteen_slot_lid_clamp_crossbars");
    for col in 1..COLS {
        let x = centered_index(col, COLS, SLOT_PITCH_X) - SLOT_PITCH_X / 2.0;
        bars = bars
            + centered_cube(
                format!("sixteen_slot_lid_vertical_clamp_bar_{col}"),
                10.0,
                SLOT_ARRAY_Y + 38.0,
                LID_Z,
            )
            .translate(x, 0.0, 0.0);
    }
    for row in 1..ROWS {
        let y = centered_index(row, ROWS, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
        bars = bars
            + centered_cube(
                format!("sixteen_slot_lid_horizontal_clamp_bar_{row}"),
                SLOT_ARRAY_X + 42.0,
                10.0,
                LID_Z,
            )
            .translate(0.0, y, 0.0);
    }
    bars
}

fn lid_alignment_ears() -> Part {
    let mut ears = Part::empty("sixteen_slot_lid_alignment_ears");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        ears = ears
            + centered_cube(
                format!("sixteen_slot_lid_alignment_ear_{i}"),
                34.0,
                22.0,
                LID_Z,
            )
            .translate(*x, *y, 0.0);
    }
    ears
}

fn window_slot_witness_frames() -> Part {
    let mut frames = Part::empty("sixteen_slot_window_slot_witness_frames");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            frames = frames
                + rectangular_frame(
                    &format!("sixteen_slot_window_slot_{slot:02}_etched_witness_frame"),
                    REVC_CHIP_LENGTH - 18.0,
                    REVC_CHIP_WIDTH - 18.0,
                    1.2,
                    3.0,
                )
                .translate(x, y, WINDOW_Z / 2.0 + 0.6);
        }
    }
    frames
}

fn window_retention_tabs() -> Part {
    let panel_x = SLOT_ARRAY_X + 84.0;
    let panel_y = SLOT_ARRAY_Y + 76.0;
    let mut tabs = Part::empty("sixteen_slot_window_mechanical_retention_tabs");
    for (i, x) in [-210.0, -70.0, 70.0, 210.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_front_retention_tab_{i}"),
                34.0,
                9.0,
                1.4,
            )
            .translate(*x, -panel_y / 2.0 + 8.0, WINDOW_Z / 2.0 + 0.7);
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_rear_retention_tab_{i}"),
                34.0,
                9.0,
                1.4,
            )
            .translate(*x, panel_y / 2.0 - 8.0, WINDOW_Z / 2.0 + 0.7);
    }
    for (i, y) in [-150.0, -50.0, 50.0, 150.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_left_retention_tab_{i}"),
                9.0,
                34.0,
                1.4,
            )
            .translate(-panel_x / 2.0 + 8.0, *y, WINDOW_Z / 2.0 + 0.7);
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_right_retention_tab_{i}"),
                9.0,
                34.0,
                1.4,
            )
            .translate(panel_x / 2.0 - 8.0, *y, WINDOW_Z / 2.0 + 0.7);
    }
    tabs
}

fn calibration_fiducials(z: f64) -> Part {
    let mut targets = Part::empty("sixteen_slot_window_calibration_fiducials");
    for (i, (x, y)) in [
        (-(SLOT_ARRAY_X / 2.0 + 24.0), SLOT_ARRAY_Y / 2.0 + 22.0),
        (SLOT_ARRAY_X / 2.0 + 24.0, SLOT_ARRAY_Y / 2.0 + 22.0),
        (-(SLOT_ARRAY_X / 2.0 + 24.0), -(SLOT_ARRAY_Y / 2.0 + 22.0)),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("sixteen_slot_window_fiducial_disc_{i}"),
            5.0,
            1.2,
            32,
        )
        .translate(*x, *y, z);
        let center = centered_cylinder(
            format!("sixteen_slot_window_fiducial_center_{i}"),
            1.2,
            1.4,
            20,
        )
        .translate(*x, *y, z);
        targets = targets + (disc - center);
    }
    targets
}

fn gasket_squeeze_steps() -> Part {
    let mut steps = Part::empty("sixteen_slot_gasket_coupon_squeeze_steps");
    for (i, (label, h)) in [
        ("20pct", GASKET_GUARD_MAX_COMPRESSED_HEIGHT),
        ("25pct", GASKET_COMPRESSED_HEIGHT),
        ("30pct", GASKET_GUARD_MIN_COMPRESSED_HEIGHT),
    ]
    .iter()
    .enumerate()
    {
        steps = steps
            + centered_cube(
                format!("sixteen_slot_gasket_coupon_{label}_squeeze_step"),
                54.0,
                26.0,
                *h,
            )
            .translate(centered_index(i, 3, 72.0), 28.0, COUPON_Z / 2.0 + h / 2.0);
    }
    steps
}

fn coupon_gasket_groove_cuts() -> Part {
    let long_loop = rectangular_frame(
        "sixteen_slot_gasket_coupon_leak_loop_groove_2p4mm",
        COUPON_X - 42.0,
        46.0,
        GASKET_GROOVE_DEPTH,
        GASKET_GROOVE_W,
    )
    .translate(0.0, 20.0, COUPON_Z / 2.0 - GASKET_GROOVE_DEPTH / 2.0 + 0.1);
    let short_loop = rectangular_frame(
        "sixteen_slot_gasket_coupon_reconnection_loop_groove_2p4mm",
        86.0,
        34.0,
        GASKET_GROOVE_DEPTH,
        GASKET_GROOVE_W,
    )
    .translate(
        74.0,
        -30.0,
        COUPON_Z / 2.0 - GASKET_GROOVE_DEPTH / 2.0 + 0.1,
    );
    long_loop + short_loop
}

fn coupon_compression_stop_lands() -> Part {
    let mut stops = Part::empty("sixteen_slot_gasket_coupon_25pct_hard_stop_lands");
    for (i, (x, y)) in [(-96.0, 44.0), (96.0, 44.0), (-96.0, -4.0), (96.0, -4.0)]
        .iter()
        .enumerate()
    {
        stops = stops
            + centered_cylinder(
                format!("sixteen_slot_gasket_coupon_stop_{i}_25pct"),
                5.0,
                GASKET_COMPRESSED_HEIGHT,
                24,
            )
            .translate(*x, *y, COUPON_Z / 2.0 + GASKET_COMPRESSED_HEIGHT / 2.0);
    }
    stops
}

fn coupon_label_lands() -> Part {
    let lot = centered_cube("sixteen_slot_gasket_coupon_lot_label_land", 84.0, 18.0, 2.0)
        .translate(-70.0, -COUPON_Y / 2.0 + 18.0, COUPON_Z / 2.0 + 1.0);
    let witness = centered_cube(
        "sixteen_slot_gasket_coupon_witness_label_land",
        102.0,
        18.0,
        2.0,
    )
    .translate(62.0, -COUPON_Y / 2.0 + 18.0, COUPON_Z / 2.0 + 1.0);
    lot + witness
}

fn slot_recesses() -> Part {
    let mut recesses = Part::empty("sixteen_slot_dock_slot_recesses");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            recesses = recesses
                + centered_cube(
                    format!("sixteen_slot_dock_slot_{slot:02}_carrier_support_recess"),
                    REVC_CHIP_LENGTH + 10.0,
                    REVC_CHIP_WIDTH + 10.0,
                    SLOT_RECESS_DEPTH,
                )
                .translate(x, y, DOCK_Z / 2.0 - SLOT_RECESS_DEPTH / 2.0 + 0.2);
        }
    }
    recesses
}

fn dock_air_bypass_windows() -> Part {
    let mut windows = Part::empty("sixteen_slot_dock_air_bypass_windows");
    for row in 0..=ROWS {
        let y = centered_index(row, ROWS + 1, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
        windows = windows
            + centered_cube(
                format!("sixteen_slot_dock_row_{row}_air_bypass_window"),
                SLOT_ARRAY_X + 74.0,
                8.0,
                DOCK_Z + 2.0,
            )
            .translate(0.0, y, 0.0);
    }
    windows
}

fn dock_drain_gutters() -> Part {
    let front = centered_cube(
        "sixteen_slot_dock_front_condensate_gutter",
        DOCK_X - 70.0,
        10.0,
        DOCK_Z + 2.0,
    )
    .translate(0.0, -DOCK_Y / 2.0 + 38.0, 0.0);
    let side = centered_cube(
        "sixteen_slot_dock_right_drain_gutter",
        10.0,
        DOCK_Y - 76.0,
        DOCK_Z + 2.0,
    )
    .translate(DOCK_X / 2.0 - 42.0, 0.0, 0.0);
    let sump = centered_cube(
        "sixteen_slot_dock_visible_drain_sump",
        58.0,
        38.0,
        DOCK_Z + 2.0,
    )
    .translate(DOCK_X / 2.0 - 58.0, -DOCK_Y / 2.0 + 58.0, 0.0);
    front + side + sump
}

fn dock_reference_rails() -> Part {
    let rear = centered_cube(
        "sixteen_slot_dock_rear_primary_datum_rail",
        CARRIER_X + 44.0,
        DOCK_RAIL_W,
        DOCK_RAIL_Z,
    )
    .translate(
        0.0,
        CARRIER_Y / 2.0 + 17.0,
        DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
    );
    let left = centered_cube(
        "sixteen_slot_dock_left_secondary_datum_rail",
        DOCK_RAIL_W,
        CARRIER_Y + 46.0,
        DOCK_RAIL_Z,
    )
    .translate(
        -CARRIER_X / 2.0 - 17.0,
        0.0,
        DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
    );
    let front = centered_cube(
        "sixteen_slot_dock_front_low_retention_lip",
        CARRIER_X + 44.0,
        10.0,
        10.0,
    )
    .translate(0.0, -CARRIER_Y / 2.0 - 14.0, DOCK_Z / 2.0 + 5.0);
    rear + left + front
}

fn slot_position_tokens() -> Part {
    let mut tokens = Part::empty("sixteen_slot_dock_position_tokens");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            let edge = row == 0 || row == ROWS - 1 || col == 0 || col == COLS - 1;
            let name = if edge { "edge" } else { "center" };
            tokens = tokens
                + centered_cube(
                    format!("sixteen_slot_dock_slot_{slot:02}_{name}_position_token"),
                    24.0,
                    10.0,
                    3.0,
                )
                .translate(x, y - REVC_CHIP_WIDTH / 2.0 - 16.0, DOCK_Z / 2.0 + 1.5);
        }
    }
    tokens
}

fn logger_pockets() -> Part {
    let mut pockets = Part::empty("sixteen_slot_dock_logger_pockets");
    for (i, (x, y)) in [
        (-(CARRIER_X / 2.0 + 34.0), CARRIER_Y / 2.0 - 30.0),
        (CARRIER_X / 2.0 + 34.0, CARRIER_Y / 2.0 - 30.0),
        (-(CARRIER_X / 2.0 + 34.0), -CARRIER_Y / 2.0 + 30.0),
        (CARRIER_X / 2.0 + 34.0, -CARRIER_Y / 2.0 + 30.0),
    ]
    .iter()
    .enumerate()
    {
        pockets = pockets
            + centered_cube(
                format!("sixteen_slot_dock_logger_pocket_{i}"),
                48.0,
                32.0,
                8.0,
            )
            .translate(*x, *y, DOCK_Z / 2.0 + 4.0);
    }
    pockets
}

fn robot_lift_lands() -> Part {
    let front = centered_cube("sixteen_slot_dock_front_robot_lift_land", 160.0, 20.0, 7.0)
        .translate(0.0, -DOCK_Y / 2.0 + 74.0, DOCK_Z / 2.0 + 3.5);
    let rear = centered_cube("sixteen_slot_dock_rear_robot_lift_land", 160.0, 20.0, 7.0).translate(
        0.0,
        DOCK_Y / 2.0 - 74.0,
        DOCK_Z / 2.0 + 3.5,
    );
    front + rear
}

fn dock_leveling_pad_lands() -> Part {
    let mut pads = Part::empty("sixteen_slot_dock_leveling_pad_lands");
    for (i, (x, y)) in [
        (-(DOCK_X / 2.0 - 42.0), -(DOCK_Y / 2.0 - 42.0)),
        (DOCK_X / 2.0 - 42.0, -(DOCK_Y / 2.0 - 42.0)),
        (-(DOCK_X / 2.0 - 42.0), DOCK_Y / 2.0 - 42.0),
        (DOCK_X / 2.0 - 42.0, DOCK_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        pads = pads
            + centered_cylinder(
                format!("sixteen_slot_dock_leveling_pad_land_{i}"),
                16.0,
                3.0,
                36,
            )
            .translate(*x, *y, DOCK_Z / 2.0 + 1.5);
    }
    pads
}

fn gas_port_cuts() -> Part {
    let mut cuts = Part::empty("sixteen_slot_bulkhead_gas_port_cuts");
    for (i, x) in [-240.0, -210.0, -180.0, -150.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_gas_port_{i}"),
                4.0,
                BULKHEAD_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 18.0);
    }
    cuts
}

fn media_port_cuts() -> Part {
    let mut cuts = Part::empty("sixteen_slot_bulkhead_media_port_cuts");
    for (i, x) in [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0]
        .iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_media_port_{i}"),
                3.2,
                BULKHEAD_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0);
    }
    cuts
}

fn waste_port_cuts() -> Part {
    let mut cuts = Part::empty("sixteen_slot_bulkhead_waste_port_cuts");
    for (i, x) in [150.0, 176.0, 202.0, 228.0, 254.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_waste_port_{i}"),
                3.2,
                BULKHEAD_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0);
    }
    cuts
}

fn sensor_connector_cut() -> Part {
    centered_cube(
        "sixteen_slot_bulkhead_sensor_backplane_connector_cut",
        88.0,
        BULKHEAD_Y + 4.0,
        18.0,
    )
    .translate(300.0, 0.0, 18.0)
}

fn bulkhead_label_strip() -> Part {
    centered_cube(
        "sixteen_slot_bulkhead_service_label_strip",
        BULKHEAD_X - 72.0,
        2.0,
        8.0,
    )
    .translate(0.0, -BULKHEAD_Y / 2.0 - 1.0, BULKHEAD_Z / 2.0 - 14.0)
}

fn tubing_strain_relief_comb() -> Part {
    let mut comb = Part::empty("sixteen_slot_bulkhead_tubing_strain_relief_comb");
    for i in 0..12 {
        comb = comb
            + centered_cube(
                format!("sixteen_slot_bulkhead_tubing_comb_tooth_{i}"),
                5.0,
                24.0,
                18.0,
            )
            .translate(centered_index(i, 12, 22.0), -BULKHEAD_Y / 2.0 - 12.0, -20.0);
    }
    comb
}

fn cable_strain_relief_comb() -> Part {
    let body = centered_cube(
        "sixteen_slot_bulkhead_cable_strain_relief_shelf",
        120.0,
        26.0,
        14.0,
    )
    .translate(300.0, -BULKHEAD_Y / 2.0 - 13.0, -20.0);
    let slot = centered_cube("sixteen_slot_bulkhead_cable_zip_slot", 92.0, 8.0, 16.0).translate(
        300.0,
        -BULKHEAD_Y / 2.0 - 13.0,
        -20.0,
    );
    body - slot
}

fn perimeter_mount_holes(prefix: &str, x: f64, y: f64, height: f64) -> Part {
    let mut holes = Part::empty(format!("{prefix}_mount_holes"));
    for (i, (px, py)) in [
        (-(x / 2.0 - 22.0), -(y / 2.0 - 22.0)),
        (x / 2.0 - 22.0, -(y / 2.0 - 22.0)),
        (-(x / 2.0 - 22.0), y / 2.0 - 22.0),
        (x / 2.0 - 22.0, y / 2.0 - 22.0),
        (0.0, -(y / 2.0 - 22.0)),
        (0.0, y / 2.0 - 22.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(format!("{prefix}_{i}"), 2.7, height, 28).translate(*px, *py, 0.0);
    }
    holes
}

fn fastener_points() -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    for row in 0..ROWS {
        let y = centered_index(row, ROWS, SLOT_PITCH_Y);
        points.push((-(SLOT_ARRAY_X / 2.0 + 24.0), y));
        points.push((SLOT_ARRAY_X / 2.0 + 24.0, y));
    }
    for col in 0..COLS {
        let x = centered_index(col, COLS, SLOT_PITCH_X);
        points.push((x, -(SLOT_ARRAY_Y / 2.0 + 24.0)));
        points.push((x, SLOT_ARRAY_Y / 2.0 + 24.0));
    }
    points
}

fn datum_points() -> [(f64, f64); 4] {
    [
        (-(CARRIER_X / 2.0 - 35.0), -(CARRIER_Y / 2.0 - 35.0)),
        (CARRIER_X / 2.0 - 35.0, -(CARRIER_Y / 2.0 - 35.0)),
        (-(CARRIER_X / 2.0 - 35.0), CARRIER_Y / 2.0 - 35.0),
        (CARRIER_X / 2.0 - 35.0, CARRIER_Y / 2.0 - 35.0),
    ]
}

fn slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COLS, SLOT_PITCH_X),
        centered_index(row, ROWS, SLOT_PITCH_Y),
    )
}

fn slot_number(row: usize, col: usize) -> usize {
    row * COLS + col + 1
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - wall * 2.0,
        y - wall * 2.0,
        z + 2.0,
    );
    outer - inner
}

fn assert_layout() {
    assert_eq!(SLOT_COUNT, 16);
    assert!(CARRIER_X > SLOT_ARRAY_X);
    assert!(CARRIER_Y > SLOT_ARRAY_Y);
    assert!(DOCK_X > CARRIER_X + 100.0);
    assert!(DOCK_Y > CARRIER_Y + 100.0);
    assert!(BULKHEAD_X > SLOT_ARRAY_X);
    assert!(LID_X > CARRIER_X);
    assert!(LID_Y > CARRIER_Y);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with("output/sixteen_slot_")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_article_has_sixteen_slots() {
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(COLS, 4);
        assert_eq!(ROWS, 4);
    }

    #[test]
    fn custom_part_outputs_are_stable() {
        assert_eq!(OUTPUTS.len(), 7);
        assert!(OUTPUTS.contains(&"output/sixteen_slot_cassette_lower_carrier.stl"));
        assert!(OUTPUTS.contains(&"output/sixteen_slot_incubator_dock_plate.stl"));
        assert!(OUTPUTS.contains(&"output/sixteen_slot_service_bulkhead_test_block.stl"));
    }

    #[test]
    fn dock_envelope_contains_carrier_and_service_margins() {
        assert!(DOCK_X - CARRIER_X >= 150.0);
        assert!(DOCK_Y - CARRIER_Y >= 140.0);
        assert!(BULKHEAD_OFFSET_Y > DOCK_Y / 2.0);
    }

    #[test]
    fn a5_interface_targets_are_explicit() {
        const TOL: f64 = 1e-9;
        assert!((CHIP_CLEARANCE - 1.2).abs() < TOL);
        assert!((DRAWING_TARGET_CHIP_CLEARANCE - 0.8).abs() < TOL);
        assert!((GASKET_FREE_HEIGHT - 2.4).abs() < TOL);
        assert!((GASKET_COMPRESSED_HEIGHT - 1.8).abs() < TOL);
        assert!((GASKET_GROOVE_DEPTH - 1.82).abs() < TOL);
        assert!((GASKET_GROOVE_W - 3.20).abs() < TOL);
        assert_eq!(fastener_points().len(), 16);
        assert_eq!(datum_points().len(), 4);
    }
}
