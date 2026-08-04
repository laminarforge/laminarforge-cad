use std::fs;

use laminarforge_cad::{sixteen_slot_cassette_a0::*, REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
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

const CHIP_POCKET_INTERNAL_RADIUS_NOTE: f64 = 1.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    validate_contract().expect("invalid 16-slot A0 interface contract");
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

    let carrier_center_z = DOCK_SUPPORT_PLANE_Z + CARRIER_Z / 2.0;
    let carrier_top_z = carrier_center_z + CARRIER_Z / 2.0;
    let lid_center_z = carrier_top_z + CLOSURE_PLANE_ABOVE_CARRIER + LID_Z / 2.0;
    let window_center_z = lid_center_z + LID_Z / 2.0 + WINDOW_Z / 2.0 + 2.0;
    let assembly = dock
        + carrier.translate(0.0, 0.0, carrier_center_z)
        + lid.translate(0.0, 0.0, lid_center_z)
        + lid_alignment_pin_surrogates().translate(0.0, 0.0, lid_center_z)
        + window.translate(0.0, 0.0, window_center_z)
        + bulkhead.translate(
            0.0,
            BULKHEAD_OFFSET_Y,
            DOCK_SUPPORT_PLANE_Z + BULKHEAD_Z / 2.0,
        )
        + coupon.translate(
            -(DOCK_X / 2.0 - COUPON_X / 2.0 - 30.0),
            -(DOCK_Y / 2.0 + COUPON_Y / 2.0 + 38.0),
            COUPON_Z / 2.0,
        );
    export(OUTPUTS[6], &assembly);

    println!();
    println!("16-slot cassette/incubator first article:");
    println!("  Slot map:                {COLS} x {ROWS} ({SLOT_COUNT} slots)");
    println!("  Cassette carrier body:   {CARRIER_X:.1}mm x {CARRIER_Y:.1}mm x {CARRIER_Z:.1}mm");
    println!("  Carrier overall Z:       {CARRIER_OVERALL_Z:.2}mm through the seal/closure lands");
    println!("  Lid/clamp base body:     {LID_X:.1}mm x {LID_Y:.1}mm x {LID_Z:.1}mm");
    println!("  Incubator dock body:     {DOCK_X:.1}mm x {DOCK_Y:.1}mm x {DOCK_Z:.1}mm");
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
        "  Closure plane:           {CLOSURE_PLANE_ABOVE_CARRIER:.2}mm above carrier top, matching nominal Rev C chip top"
    );
    println!(
        "  Gasket groove note:      {GASKET_GROOVE_DEPTH:.2}mm depth x {GASKET_GROOVE_W:.2}mm width; seal bands Ra {SEAL_BAND_RA_TARGET_UM:.1}um target / {SEAL_BAND_RA_MAX_UM:.1}um provisional max"
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
        LEAK_GUTTER_OUTER_X,
        LEAK_GUTTER_OUTER_Y,
        top_face_cut_height(LEAK_GUTTER_DEPTH),
        LEAK_GUTTER_W,
    )
    .translate(0.0, 0.0, top_face_cut_z(CARRIER_Z, LEAK_GUTTER_DEPTH));
    let drain = centered_cylinder(
        "sixteen_slot_lower_carrier_drain_port",
        CARRIER_DRAIN_DIAMETER / 2.0,
        CARRIER_DRAIN_LENGTH,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(CARRIER_DRAIN_X, CARRIER_DRAIN_Y, CARRIER_DRAIN_Z);

    let gross = body
        - chip_pockets()
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
        + inter_slot_compression_stops()
        + perimeter_compression_stops()
        + datum_pin_bosses()
        + slot_label_lands()
        + carrier_condition_id_land()
        + carrier_orientation_marker()
        + side_service_reliefs();

    gross - carrier_lid_receiver_holes() - datum_bore_cuts()
}

fn lid_clamp() -> Part {
    let seal_skin = centered_cube(
        "sixteen_slot_lid_continuous_underside_seal_skin",
        LID_X,
        LID_Y,
        LID_UNDERSIDE_SEAL_SKIN_Z,
    )
    .translate(0.0, 0.0, -LID_Z / 2.0 + LID_UNDERSIDE_SEAL_SKIN_Z / 2.0);
    let upper_outer = centered_cube(
        "sixteen_slot_lid_upper_frame_outer",
        LID_X,
        LID_Y,
        LID_UPPER_FRAME_Z,
    )
    .translate(0.0, 0.0, LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0);
    let upper_relief = centered_cube(
        "sixteen_slot_lid_upper_center_lightening_relief",
        LID_UPPER_RELIEF_X,
        LID_UPPER_RELIEF_Y,
        top_face_cut_height(LID_UPPER_FRAME_Z),
    )
    .translate(0.0, 0.0, top_face_cut_z(LID_Z, LID_UPPER_FRAME_Z));
    let frame = seal_skin + (upper_outer - upper_relief);
    let gross = frame - slot_view_openings(LID_Z + 2.0) - lid_chip_top_relief_cuts()
        + lid_crossbars()
        + lid_alignment_ears()
        + captive_fastener_retainers()
        + lid_window_retention_lip()
        + lid_torque_sequence_tabs();

    gross - lid_fastener_holes() - lid_alignment_pin_seat_cuts() - lid_gasket_groove_cuts()
}

fn window_placeholder() -> Part {
    let panel = centered_cube(
        "sixteen_slot_retained_window_placeholder_panel",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    panel + window_slot_witness_frames() + calibration_fiducials() + window_retention_tabs()
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
        COUPON_SAMPLE_SLOT_X,
        COUPON_SAMPLE_SLOT_Y,
        COUPON_Z + 2.0,
    )
    .translate(0.0, COUPON_SAMPLE_SLOT_CENTER_Y, 0.0);

    let gross = base - retain_slot
        + gasket_squeeze_steps()
        + coupon_label_lands()
        + coupon_compression_stop_lands();
    gross - coupon_gasket_groove_cuts()
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
        - dock_drain_openings()
        - perimeter_mount_holes(
            "sixteen_slot_incubator_dock_m5",
            DOCK_X,
            DOCK_Y,
            DOCK_THROUGH_CUT_HEIGHT,
        )
        + dock_reference_rails()
        + slot_position_tokens()
        + logger_reservation_lands()
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
    let gross =
        body + bulkhead_label_strip() + tubing_strain_relief_comb() + cable_strain_relief_comb();
    gross - gas_port_cuts() - media_port_cuts() - waste_port_cuts() - sensor_connector_cut()
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
                    top_face_cut_height(CHIP_POCKET_DEPTH),
                )
                .translate(x, y, top_face_cut_z(CARRIER_Z, CHIP_POCKET_DEPTH));
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
                    LID_SLOT_VIEW_OPENING_X,
                    LID_SLOT_VIEW_OPENING_Y,
                    height,
                )
                .translate(x, y, 0.0);
        }
    }
    openings
}

fn lid_chip_top_relief_cuts() -> Part {
    let mut reliefs = Part::empty("sixteen_slot_lid_chip_top_clearance_reliefs");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            reliefs = reliefs
                + centered_cube(
                    format!("sixteen_slot_lid_slot_{slot:02}_chip_top_relief"),
                    LID_CHIP_TOP_RELIEF_X,
                    LID_CHIP_TOP_RELIEF_Y,
                    bottom_face_cut_height(LID_CHIP_TOP_RELIEF_DEPTH),
                )
                .translate(
                    x,
                    y,
                    bottom_face_cut_z(LID_Z, LID_CHIP_TOP_RELIEF_DEPTH),
                );
        }
    }
    reliefs
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
                    PER_SLOT_GASKET_OUTER_X,
                    PER_SLOT_GASKET_OUTER_Y,
                    GASKET_LAND_Z,
                    GASKET_LAND_W,
                )
                .translate(x, y, CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0);
        }
    }
    lands
}

fn perimeter_gasket_land() -> Part {
    rectangular_frame(
        "sixteen_slot_lower_carrier_perimeter_gasket_land",
        PERIMETER_GASKET_OUTER_X,
        PERIMETER_GASKET_OUTER_Y,
        GASKET_LAND_Z,
        PERIMETER_GASKET_W,
    )
    .translate(0.0, 0.0, CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0)
}

fn inter_slot_compression_stops() -> Part {
    let mut stops = Part::empty("sixteen_slot_inter_slot_25pct_compression_stops");
    let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
    for (index, (x, y)) in inter_slot_stop_points().into_iter().enumerate() {
        stops = stops
            + centered_cylinder(
                format!("sixteen_slot_inter_slot_hard_stop_{index:02}_25pct"),
                INTERNAL_STOP_DIAMETER / 2.0,
                stop_z,
                24,
            )
            .translate(x, y, CARRIER_Z / 2.0 + stop_z / 2.0);
    }
    stops
}

fn perimeter_compression_stops() -> Part {
    let mut stops = Part::empty("sixteen_slot_perimeter_25pct_compression_stops");
    let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
    let x_edge = PERIMETER_GASKET_OUTER_X / 2.0 + PERIMETER_STOP_CENTER_OFFSET;
    let y_edge = PERIMETER_GASKET_OUTER_Y / 2.0 + PERIMETER_STOP_CENTER_OFFSET;
    for (i, x) in [-240.0, -120.0, 0.0, 120.0, 240.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_front_stop_{i}_25pct"),
                18.0,
                PERIMETER_STOP_W,
                stop_z,
            )
            .translate(*x, -y_edge, CARRIER_Z / 2.0 + stop_z / 2.0);
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_rear_stop_{i}_25pct"),
                18.0,
                PERIMETER_STOP_W,
                stop_z,
            )
            .translate(*x, y_edge, CARRIER_Z / 2.0 + stop_z / 2.0);
    }
    for (i, y) in [-176.0, -88.0, 0.0, 88.0, 176.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_left_stop_{i}_25pct"),
                PERIMETER_STOP_W,
                18.0,
                stop_z,
            )
            .translate(-x_edge, *y, CARRIER_Z / 2.0 + stop_z / 2.0);
        stops = stops
            + centered_cube(
                format!("sixteen_slot_perimeter_right_stop_{i}_25pct"),
                PERIMETER_STOP_W,
                18.0,
                stop_z,
            )
            .translate(x_edge, *y, CARRIER_Z / 2.0 + stop_z / 2.0);
    }
    stops
}

fn datum_pin_bosses() -> Part {
    let mut pins = Part::empty("sixteen_slot_lower_carrier_datum_pin_bosses");
    for datum in datum_features() {
        let boss = centered_cylinder(
            format!("sixteen_slot_datum_{}_boss", datum.id),
            DATUM_BOSS_DIAMETER / 2.0,
            DATUM_BOSS_Z,
            32,
        )
        .translate(datum.x, datum.y, CARRIER_Z / 2.0 + DATUM_BOSS_Z / 2.0);
        let bore = datum_bore(datum);
        pins = pins + (boss - bore);
    }
    pins
}

fn datum_bore(datum: DatumFeature) -> Part {
    let z = CARRIER_Z / 2.0 + DATUM_BOSS_Z / 2.0;
    let height = DATUM_BOSS_Z + 2.0;
    match datum.role {
        DatumRole::RoundLocator => centered_cylinder(
            "sixteen_slot_datum_D1_round_locator_bore",
            DATUM_D1_BORE_DIAMETER / 2.0,
            height,
            28,
        )
        .translate(datum.x, datum.y, z),
        DatumRole::RelievedLocator => {
            let relief = (DATUM_D2_SLOT_LENGTH - DATUM_D2_SLOT_WIDTH) / 2.0;
            centered_cylinder(
                "sixteen_slot_datum_D2_relief_left",
                DATUM_D2_SLOT_WIDTH / 2.0,
                height,
                28,
            )
            .translate(datum.x - relief, datum.y, z)
                + centered_cube(
                    "sixteen_slot_datum_D2_relief_web",
                    relief * 2.0,
                    DATUM_D2_SLOT_WIDTH,
                    height,
                )
                .translate(datum.x, datum.y, z)
                + centered_cylinder(
                    "sixteen_slot_datum_D2_relief_right",
                    DATUM_D2_SLOT_WIDTH / 2.0,
                    height,
                    28,
                )
                .translate(datum.x + relief, datum.y, z)
        }
        DatumRole::ClearanceWitness => centered_cylinder(
            format!("sixteen_slot_datum_{}_clearance_witness_bore", datum.id),
            DATUM_WITNESS_BORE_DIAMETER / 2.0,
            height,
            28,
        )
        .translate(datum.x, datum.y, z),
    }
}

fn datum_bore_cuts() -> Part {
    let mut bores = Part::empty("sixteen_slot_lower_carrier_final_datum_bore_cuts");
    for datum in datum_features() {
        bores = bores + datum_bore(datum);
    }
    bores
}

fn slot_label_lands() -> Part {
    let mut lands = Part::empty("sixteen_slot_slot_label_lands");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let x = centered_index(slot - 1, SLOT_COUNT, SLOT_LABEL_PITCH_X);
            lands = lands
                + centered_cube(
                    format!("sixteen_slot_slot_{slot:02}_label_land"),
                    SLOT_LABEL_LAND_X,
                    SLOT_LABEL_LAND_Y,
                    2.0,
                )
                .translate(x, SLOT_LABEL_CENTER_Y, CARRIER_Z / 2.0 + 1.0);
        }
    }
    lands
}

fn carrier_condition_id_land() -> Part {
    let barcode = centered_cube(
        "sixteen_slot_carrier_global_condition_id_barcode_land",
        GLOBAL_BARCODE_LAND_X,
        GLOBAL_BARCODE_LAND_Y,
        2.0,
    )
    .translate(
        GLOBAL_BARCODE_CENTER_X,
        GLOBAL_BARCODE_CENTER_Y,
        CARRIER_Z / 2.0 + 1.0,
    );
    let human = centered_cube(
        "sixteen_slot_carrier_global_condition_id_text_land",
        GLOBAL_TEXT_LAND_X,
        GLOBAL_TEXT_LAND_Y,
        1.5,
    )
    .translate(
        GLOBAL_TEXT_CENTER_X,
        GLOBAL_TEXT_CENTER_Y,
        CARRIER_Z / 2.0 + 0.75,
    );
    barcode + human
}

fn carrier_orientation_marker() -> Part {
    let x_leg = centered_cube(
        "sixteen_slot_carrier_slot_01_orientation_x_leg",
        18.0,
        4.0,
        3.0,
    )
    .translate(
        -CARRIER_X / 2.0 + 16.0,
        GLOBAL_BARCODE_CENTER_Y,
        CARRIER_Z / 2.0 + 1.5,
    );
    let y_leg = centered_cube(
        "sixteen_slot_carrier_slot_01_orientation_y_leg",
        4.0,
        18.0,
        3.0,
    )
    .translate(
        -CARRIER_X / 2.0 + 9.0,
        GLOBAL_BARCODE_CENTER_Y + 7.0,
        CARRIER_Z / 2.0 + 1.5,
    );
    x_leg + y_leg
}

fn side_service_reliefs() -> Part {
    let left = centered_cube(
        "sixteen_slot_left_tubing_service_relief_land",
        12.0,
        LEAK_GUTTER_INNER_Y - 16.0,
        SIDE_SERVICE_RELIEF_Z,
    )
    .translate(
        -(CARRIER_X / 2.0 - 7.0),
        0.0,
        CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
    );
    let right = centered_cube(
        "sixteen_slot_right_tubing_service_relief_land",
        12.0,
        LEAK_GUTTER_INNER_Y - 16.0,
        SIDE_SERVICE_RELIEF_Z,
    )
    .translate(
        CARRIER_X / 2.0 - 7.0,
        0.0,
        CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
    );
    left + right - side_service_datum_keepouts()
}

fn side_service_datum_keepouts() -> Part {
    let mut keepouts = Part::empty("sixteen_slot_side_service_datum_keepouts");
    for datum in datum_features() {
        keepouts = keepouts
            + centered_cylinder(
                format!("sixteen_slot_side_service_datum_{}_keepout", datum.id),
                DATUM_BOSS_DIAMETER / 2.0 + SIDE_SERVICE_DATUM_CLEARANCE,
                SIDE_SERVICE_RELIEF_Z + 2.0,
                32,
            )
            .translate(
                datum.x,
                datum.y,
                CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
            );
    }
    keepouts
}

fn carrier_lid_receiver_holes() -> Part {
    let mut holes = Part::empty("sixteen_slot_carrier_m4_receiver_pilot_holes");
    for (index, (x, y)) in fastener_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("sixteen_slot_carrier_m4_receiver_pilot_{index:02}"),
                CARRIER_LID_RECEIVER_DIAMETER / 2.0,
                CARRIER_Z + 2.0,
                28,
            )
            .translate(x, y, 0.0);
    }
    holes
}

fn lid_fastener_holes() -> Part {
    let mut holes = Part::empty("sixteen_slot_lid_fastener_holes");
    for (i, (x, y)) in fastener_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("sixteen_slot_lid_m4_clearance_{i}"),
                LID_FASTENER_CLEARANCE_DIAMETER / 2.0,
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
            LID_RETAINER_DIAMETER / 2.0,
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
                    PER_SLOT_GASKET_GROOVE_OUTER_X,
                    PER_SLOT_GASKET_GROOVE_OUTER_Y,
                    GASKET_GROOVE_CUT_HEIGHT,
                    GASKET_GROOVE_W,
                )
                .translate(x, y, bottom_face_gasket_groove_cut_z(LID_Z));
        }
    }
    grooves
        + rectangular_frame(
            "sixteen_slot_lid_perimeter_gasket_groove_2p4mm",
            PERIMETER_GASKET_GROOVE_OUTER_X,
            PERIMETER_GASKET_GROOVE_OUTER_Y,
            GASKET_GROOVE_CUT_HEIGHT,
            GASKET_GROOVE_W,
        )
        .translate(0.0, 0.0, bottom_face_gasket_groove_cut_z(LID_Z))
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
                LID_UPPER_FRAME_Z,
            )
            .translate(x, 0.0, LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0);
    }
    for row in 1..ROWS {
        let y = centered_index(row, ROWS, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
        bars = bars
            + centered_cube(
                format!("sixteen_slot_lid_horizontal_clamp_bar_{row}"),
                SLOT_ARRAY_X + 42.0,
                10.0,
                LID_UPPER_FRAME_Z,
            )
            .translate(0.0, y, LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0);
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
                LID_UPPER_FRAME_Z,
            )
            .translate(*x, *y, LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0);
    }
    ears
}

fn lid_alignment_pin_seat_cuts() -> Part {
    let mut seats = Part::empty("sixteen_slot_lid_replaceable_alignment_pin_seats");
    for datum in datum_features().into_iter().filter(|datum| {
        matches!(
            datum.role,
            DatumRole::RoundLocator | DatumRole::RelievedLocator
        )
    }) {
        seats = seats
            + centered_cylinder(
                format!("sixteen_slot_lid_datum_{}_replaceable_pin_seat", datum.id),
                LID_DATUM_PIN_SEAT_DIAMETER / 2.0,
                bottom_face_cut_height(LID_DATUM_PIN_SEAT_DEPTH),
                28,
            )
            .translate(
                datum.x,
                datum.y,
                bottom_face_cut_z(LID_Z, LID_DATUM_PIN_SEAT_DEPTH),
            );
    }
    seats
}

fn lid_alignment_pin_surrogates() -> Part {
    let mut pins = Part::empty("sixteen_slot_replaceable_alignment_pin_surrogates");
    for datum in datum_features().into_iter().filter(|datum| {
        matches!(
            datum.role,
            DatumRole::RoundLocator | DatumRole::RelievedLocator
        )
    }) {
        pins = pins
            + centered_cylinder(
                format!("sixteen_slot_datum_{}_replaceable_pin_surrogate", datum.id),
                LID_DATUM_PIN_DIAMETER / 2.0,
                LID_DATUM_PIN_TOTAL_Z,
                28,
            )
            .translate(
                datum.x,
                datum.y,
                -LID_Z / 2.0 + (LID_DATUM_PIN_EMBEDMENT - LID_DATUM_PIN_EXTENSION) / 2.0,
            );
    }
    pins
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
                    WINDOW_WITNESS_FRAME_BODY_Z,
                    3.0,
                )
                .translate(x, y, WINDOW_WITNESS_FRAME_CENTER_Z);
        }
    }
    frames
}

fn window_retention_tabs() -> Part {
    let panel_x = WINDOW_X;
    let panel_y = WINDOW_Y;
    let mut tabs = Part::empty("sixteen_slot_window_mechanical_retention_tabs");
    for (i, x) in [-210.0, -70.0, 70.0, 210.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_front_retention_tab_{i}"),
                34.0,
                9.0,
                WINDOW_RETENTION_TAB_BODY_Z,
            )
            .translate(*x, -panel_y / 2.0 + 8.0, WINDOW_RETENTION_TAB_CENTER_Z);
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_rear_retention_tab_{i}"),
                34.0,
                9.0,
                WINDOW_RETENTION_TAB_BODY_Z,
            )
            .translate(*x, panel_y / 2.0 - 8.0, WINDOW_RETENTION_TAB_CENTER_Z);
    }
    for (i, y) in [-150.0, -50.0, 50.0, 150.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_left_retention_tab_{i}"),
                9.0,
                34.0,
                WINDOW_RETENTION_TAB_BODY_Z,
            )
            .translate(-panel_x / 2.0 + 8.0, *y, WINDOW_RETENTION_TAB_CENTER_Z);
        tabs = tabs
            + centered_cube(
                format!("sixteen_slot_window_right_retention_tab_{i}"),
                9.0,
                34.0,
                WINDOW_RETENTION_TAB_BODY_Z,
            )
            .translate(panel_x / 2.0 - 8.0, *y, WINDOW_RETENTION_TAB_CENTER_Z);
    }
    tabs
}

fn calibration_fiducials() -> Part {
    let mut targets = Part::empty("sixteen_slot_window_calibration_fiducials");
    for (i, (x, y)) in window_fiducial_points().iter().enumerate() {
        let disc = centered_cylinder(
            format!("sixteen_slot_window_fiducial_disc_{i}"),
            5.0,
            WINDOW_FIDUCIAL_BODY_Z,
            32,
        )
        .translate(*x, *y, WINDOW_FIDUCIAL_CENTER_Z);
        let center = centered_cylinder(
            format!("sixteen_slot_window_fiducial_center_{i}"),
            1.2,
            WINDOW_FIDUCIAL_HOLE_CUT_Z,
            20,
        )
        .translate(*x, *y, WINDOW_FIDUCIAL_CENTER_Z);
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
                COUPON_SQUEEZE_STEP_X,
                COUPON_SQUEEZE_STEP_Y,
                *h,
            )
            .translate(
                centered_index(i, 3, 72.0),
                COUPON_SQUEEZE_STEP_CENTER_Y,
                COUPON_Z / 2.0 + h / 2.0,
            );
    }
    steps
}

fn coupon_gasket_groove_cuts() -> Part {
    let long_loop = rectangular_frame(
        "sixteen_slot_gasket_coupon_leak_loop_groove_2p4mm",
        COUPON_LEAK_LOOP_X,
        COUPON_LEAK_LOOP_Y,
        GASKET_GROOVE_CUT_HEIGHT,
        GASKET_GROOVE_W,
    )
    .translate(
        COUPON_LEAK_LOOP_CENTER_X,
        COUPON_LOOP_CENTER_Y,
        top_face_gasket_groove_cut_z(COUPON_Z),
    );
    let short_loop = rectangular_frame(
        "sixteen_slot_gasket_coupon_reconnection_loop_groove_2p4mm",
        COUPON_RECONNECTION_LOOP_X,
        COUPON_RECONNECTION_LOOP_Y,
        GASKET_GROOVE_CUT_HEIGHT,
        GASKET_GROOVE_W,
    )
    .translate(
        COUPON_RECONNECTION_LOOP_CENTER_X,
        COUPON_LOOP_CENTER_Y,
        top_face_gasket_groove_cut_z(COUPON_Z),
    );
    long_loop + short_loop
}

fn coupon_compression_stop_lands() -> Part {
    let mut stops = Part::empty("sixteen_slot_gasket_coupon_25pct_hard_stop_lands");
    for (i, (x, y)) in coupon_stop_points().iter().enumerate() {
        stops = stops
            + centered_cylinder(
                format!("sixteen_slot_gasket_coupon_stop_{i}_25pct"),
                INTERNAL_STOP_DIAMETER / 2.0,
                GASKET_COMPRESSED_HEIGHT,
                24,
            )
            .translate(*x, *y, COUPON_Z / 2.0 + GASKET_COMPRESSED_HEIGHT / 2.0);
    }
    stops
}

fn coupon_label_lands() -> Part {
    let lot = centered_cube(
        "sixteen_slot_gasket_coupon_lot_label_land",
        84.0,
        COUPON_LABEL_LAND_Y,
        2.0,
    )
    .translate(-70.0, COUPON_LABEL_CENTER_Y, COUPON_Z / 2.0 + 1.0);
    let witness = centered_cube(
        "sixteen_slot_gasket_coupon_witness_label_land",
        102.0,
        COUPON_LABEL_LAND_Y,
        2.0,
    )
    .translate(62.0, COUPON_LABEL_CENTER_Y, COUPON_Z / 2.0 + 1.0);
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
                    DOCK_SLOT_RECESS_X,
                    DOCK_SLOT_RECESS_Y,
                    dock_slot_recess_cut_height(),
                )
                .translate(x, y, dock_slot_recess_cut_z());
        }
    }
    recesses
}

fn dock_air_bypass_windows() -> Part {
    let mut windows = Part::empty("sixteen_slot_dock_air_bypass_windows");
    for (row, y) in dock_air_bypass_center_ys().into_iter().enumerate() {
        windows = windows
            + centered_cube(
                format!("sixteen_slot_dock_row_{row}_air_bypass_window"),
                DOCK_AIR_BYPASS_X,
                DOCK_AIR_BYPASS_Y,
                DOCK_THROUGH_CUT_HEIGHT,
            )
            .translate(0.0, y, DOCK_THROUGH_CUT_CENTER_Z);
    }
    windows
}

fn dock_drain_openings() -> Part {
    let front = centered_cube(
        "sixteen_slot_dock_front_through_drain_opening",
        DOCK_FRONT_DRAIN_OPENING_X,
        DOCK_FRONT_DRAIN_OPENING_Y,
        DOCK_THROUGH_CUT_HEIGHT,
    )
    .translate(
        0.0,
        DOCK_FRONT_DRAIN_OPENING_CENTER_Y,
        DOCK_THROUGH_CUT_CENTER_Z,
    );
    let side = centered_cube(
        "sixteen_slot_dock_right_through_drain_opening",
        DOCK_SIDE_DRAIN_OPENING_X,
        DOCK_SIDE_DRAIN_OPENING_Y,
        DOCK_THROUGH_CUT_HEIGHT,
    )
    .translate(
        DOCK_SIDE_DRAIN_OPENING_CENTER_X,
        0.0,
        DOCK_THROUGH_CUT_CENTER_Z,
    );
    let visibility = centered_cube(
        "sixteen_slot_dock_drain_visibility_opening",
        DOCK_DRAIN_VISIBILITY_OPENING_X,
        DOCK_DRAIN_VISIBILITY_OPENING_Y,
        DOCK_THROUGH_CUT_HEIGHT,
    )
    .translate(
        DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X,
        DOCK_DRAIN_VISIBILITY_OPENING_CENTER_Y,
        DOCK_THROUGH_CUT_CENTER_Z,
    );
    front + side + visibility
}

fn dock_reference_rails() -> Part {
    let rear = centered_cube(
        "sixteen_slot_dock_rear_primary_datum_rail",
        DOCK_REAR_RAIL_X,
        DOCK_RAIL_W,
        DOCK_RAIL_Z,
    )
    .translate(
        0.0,
        DOCK_REAR_RAIL_CENTER_Y,
        dock_top_feature_center_z(DOCK_RAIL_Z),
    );
    let left = centered_cube(
        "sixteen_slot_dock_left_secondary_datum_rail",
        DOCK_RAIL_W,
        DOCK_LEFT_RAIL_Y,
        DOCK_RAIL_Z,
    )
    .translate(
        DOCK_LEFT_RAIL_CENTER_X,
        0.0,
        dock_top_feature_center_z(DOCK_RAIL_Z),
    );
    let front = centered_cube(
        "sixteen_slot_dock_front_low_retention_lip",
        DOCK_FRONT_LIP_X,
        DOCK_FRONT_LIP_W,
        DOCK_FRONT_LIP_Z,
    )
    .translate(
        0.0,
        DOCK_FRONT_LIP_CENTER_Y,
        dock_top_feature_center_z(DOCK_FRONT_LIP_Z),
    );
    rear + left + front
}

fn slot_position_tokens() -> Part {
    let mut tokens = Part::empty("sixteen_slot_dock_position_tokens");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = dock_position_token_point(row, col);
            let edge = row == 0 || row == ROWS - 1 || col == 0 || col == COLS - 1;
            let name = if edge { "edge" } else { "center" };
            tokens = tokens
                + centered_cube(
                    format!("sixteen_slot_dock_slot_{slot:02}_{name}_position_token"),
                    DOCK_POSITION_TOKEN_X,
                    DOCK_POSITION_TOKEN_Y,
                    DOCK_POSITION_TOKEN_Z,
                )
                .translate(x, y, dock_top_feature_center_z(DOCK_POSITION_TOKEN_Z));
        }
    }
    tokens
}

fn logger_reservation_lands() -> Part {
    let mut lands = Part::empty("sixteen_slot_dock_logger_reservation_lands");
    for (i, (x, y)) in dock_logger_reservation_land_points()
        .into_iter()
        .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("sixteen_slot_dock_logger_reservation_land_{i}"),
                DOCK_LOGGER_RESERVATION_LAND_X,
                DOCK_LOGGER_RESERVATION_LAND_Y,
                DOCK_LOGGER_RESERVATION_LAND_Z,
            )
            .translate(
                x,
                y,
                dock_top_feature_center_z(DOCK_LOGGER_RESERVATION_LAND_Z),
            );
    }
    lands
}

fn robot_lift_lands() -> Part {
    let points = dock_robot_lift_points();
    let front = centered_cube(
        "sixteen_slot_dock_front_robot_lift_land",
        DOCK_ROBOT_LIFT_X,
        DOCK_ROBOT_LIFT_Y,
        DOCK_ROBOT_LIFT_Z,
    )
    .translate(
        points[0].0,
        points[0].1,
        dock_top_feature_center_z(DOCK_ROBOT_LIFT_Z),
    );
    let rear = centered_cube(
        "sixteen_slot_dock_rear_robot_lift_land",
        DOCK_ROBOT_LIFT_X,
        DOCK_ROBOT_LIFT_Y,
        DOCK_ROBOT_LIFT_Z,
    )
    .translate(
        points[1].0,
        points[1].1,
        dock_top_feature_center_z(DOCK_ROBOT_LIFT_Z),
    );
    front + rear
}

fn dock_leveling_pad_lands() -> Part {
    let mut pads = Part::empty("sixteen_slot_dock_leveling_pad_lands");
    for (i, (x, y)) in dock_leveling_pad_points().into_iter().enumerate() {
        pads = pads
            + centered_cylinder(
                format!("sixteen_slot_dock_leveling_pad_land_{i}"),
                DOCK_LEVELING_PAD_RADIUS,
                DOCK_LEVELING_PAD_Z,
                36,
            )
            .translate(x, y, dock_top_feature_center_z(DOCK_LEVELING_PAD_Z));
    }
    pads
}

fn gas_port_cuts() -> Part {
    let mut cuts = Part::empty("sixteen_slot_bulkhead_gas_port_cuts");
    for (i, x) in GAS_PORT_XS.iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_gas_port_{i}"),
                GAS_PORT_DIAMETER / 2.0,
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
    for (i, x) in MEDIA_PORT_XS.iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_media_port_{i}"),
                MEDIA_WASTE_PORT_DIAMETER / 2.0,
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
    for (i, x) in WASTE_PORT_XS.iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("sixteen_slot_bulkhead_waste_port_{i}"),
                MEDIA_WASTE_PORT_DIAMETER / 2.0,
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
        SENSOR_CONNECTOR_X,
        BULKHEAD_Y + 4.0,
        SENSOR_CONNECTOR_Z,
    )
    .translate(SENSOR_CONNECTOR_CENTER_X, 0.0, SENSOR_CONNECTOR_CENTER_Z)
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
    .translate(SENSOR_CONNECTOR_CENTER_X, -BULKHEAD_Y / 2.0 - 13.0, -20.0);
    let slot = centered_cube("sixteen_slot_bulkhead_cable_zip_slot", 92.0, 8.0, 16.0).translate(
        SENSOR_CONNECTOR_CENTER_X,
        -BULKHEAD_Y / 2.0 - 13.0,
        -20.0,
    );
    body - slot
}

fn perimeter_mount_holes(prefix: &str, x: f64, y: f64, height: f64) -> Part {
    let mut holes = Part::empty(format!("{prefix}_mount_holes"));
    for (i, (px, py)) in perimeter_mount_points(x, y).into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{prefix}_{i}"),
                PERIMETER_MOUNT_HOLE_DIAMETER / 2.0,
                height,
                28,
            )
            .translate(px, py, 0.0);
    }
    holes
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
        assert!((GASKET_GROOVE_DEPTH - 1.80).abs() < TOL);
        assert!((GASKET_GROOVE_W - 3.20).abs() < TOL);
        assert!((SEAL_BAND_RA_TARGET_UM - 0.8).abs() < TOL);
        assert!((SEAL_BAND_RA_MAX_UM - 1.6).abs() < TOL);
        assert_eq!(fastener_points().len(), 16);
        assert_eq!(inter_slot_stop_points().len(), 9);
        assert_eq!(datum_points().len(), 4);
    }
}
