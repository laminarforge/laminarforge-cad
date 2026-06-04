#[cfg(not(feature = "step"))]
fn main() {
    eprintln!(
        "This binary requires the 'step' feature. Build with: \
         CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo run --features step --bin sixteen_slot_cassette_first_article_step"
    );
    std::process::exit(1);
}

#[cfg(feature = "step")]
mod step_export {
    use std::fs;
    use std::path::Path;

    use glam::dvec3;
    use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
    use opencascade::primitives::Shape;
    use opencascade::workplane::Workplane;

    const OUTPUT_DIR: &str = "output/rfq";
    const OUTPUTS: [&str; 7] = [
        "output/rfq/sixteen_slot_cassette_lower_carrier.step",
        "output/rfq/sixteen_slot_cassette_lid_clamp.step",
        "output/rfq/sixteen_slot_cassette_window_placeholder.step",
        "output/rfq/sixteen_slot_cassette_gasket_witness_coupon.step",
        "output/rfq/sixteen_slot_incubator_dock_plate.step",
        "output/rfq/sixteen_slot_service_bulkhead_test_block.step",
        "output/rfq/sixteen_slot_cassette_stackup_reference.step",
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

    pub fn main() {
        assert_layout();
        fs::create_dir_all(OUTPUT_DIR).expect("failed to create output/rfq");

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

        let stackup = stackup_reference();
        export(OUTPUTS[6], &stackup);

        for path in OUTPUTS {
            assert!(
                Path::new(path).exists(),
                "STEP export did not create required output: {path}"
            );
        }

        println!();
        println!("16-slot cassette first-article STEP draft:");
        println!("  Slot map:             {COLS} x {ROWS} ({SLOT_COUNT} slots)");
        println!("  Carrier:              {CARRIER_X:.1} x {CARRIER_Y:.1} x {CARRIER_Z:.1} mm");
        println!("  Lid/clamp:            {LID_X:.1} x {LID_Y:.1} x {LID_Z:.1} mm");
        println!("  Dock plate:           {DOCK_X:.1} x {DOCK_Y:.1} x {DOCK_Z:.1} mm");
        println!("  Bulkhead:             {BULKHEAD_X:.1} x {BULKHEAD_Y:.1} x {BULKHEAD_Z:.1} mm");
        println!(
            "  Pocket clearance:     {CHIP_CLEARANCE:.2} mm/side STEP draft; {DRAWING_TARGET_CHIP_CLEARANCE:.2} mm/side drawing target after chip lot measurement"
        );
        println!(
            "  Gasket compression:   {GASKET_FREE_HEIGHT:.2} mm free -> {GASKET_COMPRESSED_HEIGHT:.2} mm target"
        );
        println!("  RFQ status:           STEP draft only; D0-D9 drawings still control release");
        println!("  Biology status:       no sterile, live-cell, AAV, or containment claim");
    }

    fn export(path: &str, shape: &Shape) {
        shape.write_step(path).unwrap_or_else(|error| {
            panic!("failed to write STEP output {path}: {error:?}");
        });
        println!("Exported: {path}");
    }

    fn lower_carrier() -> Shape {
        let mut carrier = centered_box(0.0, 0.0, 0.0, CARRIER_X, CARRIER_Y, CARRIER_Z);

        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                let pocket = centered_box(
                    x,
                    y,
                    CARRIER_Z / 2.0 - CHIP_POCKET_DEPTH / 2.0 + 0.2,
                    REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
                    CHIP_POCKET_DEPTH,
                );
                carrier = carrier.subtract(&pocket).into();

                let window = centered_box(
                    x,
                    y,
                    0.0,
                    REVC_CHIP_LENGTH - OPTICAL_WINDOW_MARGIN,
                    REVC_CHIP_WIDTH - OPTICAL_WINDOW_MARGIN,
                    CARRIER_Z + 2.0,
                );
                carrier = carrier.subtract(&window).into();

                carrier = carrier
                    .union(&translate(
                        rectangular_frame(
                            REVC_CHIP_LENGTH + 14.0,
                            REVC_CHIP_WIDTH + 14.0,
                            PER_SLOT_GASKET_LAND_Z,
                            GASKET_LAND_W,
                        ),
                        x,
                        y,
                        CARRIER_Z / 2.0 + PER_SLOT_GASKET_LAND_Z / 2.0,
                    ))
                    .into();

                carrier = carrier.union(&slot_label_land(row, col)).into();

                let stop_z = PER_SLOT_GASKET_LAND_Z + GASKET_COMPRESSED_HEIGHT;
                let offset_x = REVC_CHIP_LENGTH / 2.0 + 3.0;
                let offset_y = REVC_CHIP_WIDTH / 2.0 + 3.0;
                for (sx, sy) in [
                    (-offset_x, -offset_y),
                    (offset_x, -offset_y),
                    (-offset_x, offset_y),
                    (offset_x, offset_y),
                ] {
                    carrier = carrier
                        .union(&centered_cyl_z(
                            x + sx,
                            y + sy,
                            CARRIER_Z / 2.0 + stop_z / 2.0,
                            2.5,
                            stop_z,
                        ))
                        .into();
                }
            }
        }

        carrier = carrier
            .subtract(&translate(
                rectangular_frame(CARRIER_X - 28.0, CARRIER_Y - 28.0, CARRIER_Z + 2.0, 7.0),
                0.0,
                0.0,
                CARRIER_Z / 2.0 - 4.0,
            ))
            .into();

        carrier = carrier
            .union(&translate(
                rectangular_frame(
                    SLOT_ARRAY_X + 72.0,
                    SLOT_ARRAY_Y + 66.0,
                    PERIMETER_GASKET_LAND_Z,
                    PERIMETER_GASKET_W,
                ),
                0.0,
                0.0,
                CARRIER_Z / 2.0 + PERIMETER_GASKET_LAND_Z / 2.0,
            ))
            .into();

        let stop_z = PERIMETER_GASKET_LAND_Z + GASKET_COMPRESSED_HEIGHT;
        let x_edge = (SLOT_ARRAY_X + 72.0) / 2.0 + 10.0;
        let y_edge = (SLOT_ARRAY_Y + 66.0) / 2.0 + 10.0;
        for x in [-220.0, -110.0, 0.0, 110.0, 220.0] {
            carrier = carrier
                .union(&centered_box(
                    x,
                    -y_edge,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    18.0,
                    7.0,
                    stop_z,
                ))
                .into();
            carrier = carrier
                .union(&centered_box(
                    x,
                    y_edge,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    18.0,
                    7.0,
                    stop_z,
                ))
                .into();
        }
        for y in [-150.0, -75.0, 0.0, 75.0, 150.0] {
            carrier = carrier
                .union(&centered_box(
                    -x_edge,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    7.0,
                    18.0,
                    stop_z,
                ))
                .into();
            carrier = carrier
                .union(&centered_box(
                    x_edge,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    7.0,
                    18.0,
                    stop_z,
                ))
                .into();
        }

        for (x, y) in datum_points() {
            let boss = centered_cyl_z(x, y, CARRIER_Z / 2.0 + 3.0, 9.0, 6.0);
            let bore = centered_cyl_z(x, y, CARRIER_Z / 2.0 + 3.0, 3.0, 8.0);
            carrier = carrier.union(&boss.subtract(&bore).into()).into();
        }

        carrier = carrier.union(&carrier_condition_id_land()).into();
        carrier = carrier.union(&carrier_orientation_marker()).into();
        carrier = carrier.union(&carrier_handling_lands()).into();
        carrier = carrier.union(&side_service_reliefs()).into();
        carrier = carrier
            .subtract(&perimeter_mount_holes(
                CARRIER_X,
                CARRIER_Y,
                CARRIER_Z + 2.0,
                2.7,
            ))
            .into();
        carrier
    }

    fn lid_clamp() -> Shape {
        let mut lid = centered_box(0.0, 0.0, 0.0, LID_X, LID_Y, LID_Z);
        lid = lid
            .subtract(&centered_box(
                0.0,
                0.0,
                0.0,
                SLOT_ARRAY_X + 52.0,
                SLOT_ARRAY_Y + 44.0,
                LID_Z + 2.0,
            ))
            .into();

        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                lid = lid
                    .subtract(&centered_box(
                        x,
                        y,
                        0.0,
                        REVC_CHIP_LENGTH - 14.0,
                        REVC_CHIP_WIDTH - 14.0,
                        LID_Z + 2.0,
                    ))
                    .into();
                lid = lid
                    .subtract(&translate(
                        rectangular_frame(
                            REVC_CHIP_LENGTH + 14.0 - (GASKET_LAND_W - GASKET_GROOVE_W),
                            REVC_CHIP_WIDTH + 14.0 - (GASKET_LAND_W - GASKET_GROOVE_W),
                            GASKET_GROOVE_DEPTH,
                            GASKET_GROOVE_W,
                        ),
                        x,
                        y,
                        -LID_Z / 2.0 + GASKET_GROOVE_DEPTH / 2.0 - 0.05,
                    ))
                    .into();
            }
        }

        lid = lid
            .subtract(&translate(
                rectangular_frame(
                    SLOT_ARRAY_X + 72.0 - (PERIMETER_GASKET_W - GASKET_GROOVE_W),
                    SLOT_ARRAY_Y + 66.0 - (PERIMETER_GASKET_W - GASKET_GROOVE_W),
                    GASKET_GROOVE_DEPTH,
                    GASKET_GROOVE_W,
                ),
                0.0,
                0.0,
                -LID_Z / 2.0 + GASKET_GROOVE_DEPTH / 2.0 - 0.05,
            ))
            .into();

        for (x, y) in fastener_points() {
            lid = lid
                .subtract(&centered_cyl_z(x, y, 0.0, 2.4, LID_Z + 2.0))
                .into();
            let retainer = centered_cyl_z(x, y, LID_Z / 2.0 + 0.8, 5.4, 1.6)
                .subtract(&centered_cyl_z(x, y, LID_Z / 2.0 + 0.8, 2.6, 2.0));
            lid = lid.union(&retainer.into()).into();
            lid = lid
                .union(&centered_box(
                    x,
                    y + 11.0,
                    LID_Z / 2.0 + 0.5,
                    16.0,
                    6.0,
                    1.0,
                ))
                .into();
        }

        for col in 1..COLS {
            let x = centered_index(col, COLS, SLOT_PITCH_X) - SLOT_PITCH_X / 2.0;
            lid = lid
                .union(&centered_box(x, 0.0, 0.0, 10.0, SLOT_ARRAY_Y + 38.0, LID_Z))
                .into();
        }
        for row in 1..ROWS {
            let y = centered_index(row, ROWS, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
            lid = lid
                .union(&centered_box(0.0, y, 0.0, SLOT_ARRAY_X + 42.0, 10.0, LID_Z))
                .into();
        }

        for (x, y) in datum_points() {
            lid = lid
                .union(&centered_box(x, y, 0.0, 34.0, 22.0, LID_Z))
                .into();
        }

        lid.union(&translate(
            rectangular_frame(SLOT_ARRAY_X + 96.0, SLOT_ARRAY_Y + 88.0, 1.6, 6.0),
            0.0,
            0.0,
            LID_Z / 2.0 + 0.8,
        ))
        .into()
    }

    fn window_placeholder() -> Shape {
        let panel_x = SLOT_ARRAY_X + 84.0;
        let panel_y = SLOT_ARRAY_Y + 76.0;
        let mut window = centered_box(0.0, 0.0, 0.0, panel_x, panel_y, WINDOW_Z);

        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                window = window
                    .union(&translate(
                        rectangular_frame(
                            REVC_CHIP_LENGTH - 18.0,
                            REVC_CHIP_WIDTH - 18.0,
                            1.2,
                            3.0,
                        ),
                        x,
                        y,
                        WINDOW_Z / 2.0 + 0.6,
                    ))
                    .into();
            }
        }

        for (x, y) in [
            (-(SLOT_ARRAY_X / 2.0 + 24.0), SLOT_ARRAY_Y / 2.0 + 22.0),
            (SLOT_ARRAY_X / 2.0 + 24.0, SLOT_ARRAY_Y / 2.0 + 22.0),
            (-(SLOT_ARRAY_X / 2.0 + 24.0), -(SLOT_ARRAY_Y / 2.0 + 22.0)),
        ] {
            let fiducial = centered_cyl_z(x, y, WINDOW_Z / 2.0 + 1.2, 5.0, 1.2)
                .subtract(&centered_cyl_z(x, y, WINDOW_Z / 2.0 + 1.2, 1.2, 1.4));
            window = window.union(&fiducial.into()).into();
        }

        for x in [-210.0, -70.0, 70.0, 210.0] {
            window = window
                .union(&centered_box(
                    x,
                    -panel_y / 2.0 + 8.0,
                    WINDOW_Z / 2.0 + 0.7,
                    34.0,
                    9.0,
                    1.4,
                ))
                .into();
            window = window
                .union(&centered_box(
                    x,
                    panel_y / 2.0 - 8.0,
                    WINDOW_Z / 2.0 + 0.7,
                    34.0,
                    9.0,
                    1.4,
                ))
                .into();
        }
        for y in [-150.0, -50.0, 50.0, 150.0] {
            window = window
                .union(&centered_box(
                    -panel_x / 2.0 + 8.0,
                    y,
                    WINDOW_Z / 2.0 + 0.7,
                    9.0,
                    34.0,
                    1.4,
                ))
                .into();
            window = window
                .union(&centered_box(
                    panel_x / 2.0 - 8.0,
                    y,
                    WINDOW_Z / 2.0 + 0.7,
                    9.0,
                    34.0,
                    1.4,
                ))
                .into();
        }

        window
    }

    fn gasket_witness_coupon() -> Shape {
        let mut coupon = centered_box(0.0, 0.0, 0.0, COUPON_X, COUPON_Y, COUPON_Z);
        coupon = coupon
            .subtract(&centered_box(
                0.0,
                -34.0,
                0.0,
                COUPON_X - 34.0,
                22.0,
                COUPON_Z + 2.0,
            ))
            .into();
        coupon = coupon
            .subtract(&translate(
                rectangular_frame(COUPON_X - 42.0, 46.0, GASKET_GROOVE_DEPTH, GASKET_GROOVE_W),
                0.0,
                20.0,
                COUPON_Z / 2.0 - GASKET_GROOVE_DEPTH / 2.0 + 0.1,
            ))
            .into();
        coupon = coupon
            .subtract(&translate(
                rectangular_frame(86.0, 34.0, GASKET_GROOVE_DEPTH, GASKET_GROOVE_W),
                74.0,
                -30.0,
                COUPON_Z / 2.0 - GASKET_GROOVE_DEPTH / 2.0 + 0.1,
            ))
            .into();

        for (i, h) in [
            GASKET_GUARD_MAX_COMPRESSED_HEIGHT,
            GASKET_COMPRESSED_HEIGHT,
            GASKET_GUARD_MIN_COMPRESSED_HEIGHT,
        ]
        .iter()
        .enumerate()
        {
            coupon = coupon
                .union(&centered_box(
                    centered_index(i, 3, 72.0),
                    28.0,
                    COUPON_Z / 2.0 + h / 2.0,
                    54.0,
                    26.0,
                    *h,
                ))
                .into();
        }

        for (x, y) in [(-96.0, 44.0), (96.0, 44.0), (-96.0, -4.0), (96.0, -4.0)] {
            coupon = coupon
                .union(&centered_cyl_z(
                    x,
                    y,
                    COUPON_Z / 2.0 + GASKET_COMPRESSED_HEIGHT / 2.0,
                    5.0,
                    GASKET_COMPRESSED_HEIGHT,
                ))
                .into();
        }
        coupon = coupon
            .union(&centered_box(
                -70.0,
                -COUPON_Y / 2.0 + 18.0,
                COUPON_Z / 2.0 + 1.0,
                84.0,
                18.0,
                2.0,
            ))
            .into();
        coupon
            .union(&centered_box(
                62.0,
                -COUPON_Y / 2.0 + 18.0,
                COUPON_Z / 2.0 + 1.0,
                102.0,
                18.0,
                2.0,
            ))
            .into()
    }

    fn incubator_dock_plate() -> Shape {
        let mut dock = centered_box(0.0, 0.0, 0.0, DOCK_X, DOCK_Y, DOCK_Z);
        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                dock = dock
                    .subtract(&centered_box(
                        x,
                        y,
                        DOCK_Z / 2.0 - SLOT_RECESS_DEPTH / 2.0 + 0.2,
                        REVC_CHIP_LENGTH + 10.0,
                        REVC_CHIP_WIDTH + 10.0,
                        SLOT_RECESS_DEPTH,
                    ))
                    .into();
                dock = dock
                    .union(&centered_box(
                        x,
                        y - REVC_CHIP_WIDTH / 2.0 - 16.0,
                        DOCK_Z / 2.0 + 1.5,
                        24.0,
                        10.0,
                        3.0,
                    ))
                    .into();
            }
        }

        for row in 0..=ROWS {
            let y = centered_index(row, ROWS + 1, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
            dock = dock
                .subtract(&centered_box(
                    0.0,
                    y,
                    0.0,
                    SLOT_ARRAY_X + 74.0,
                    8.0,
                    DOCK_Z + 2.0,
                ))
                .into();
        }

        dock = dock
            .subtract(&centered_box(
                0.0,
                -DOCK_Y / 2.0 + 38.0,
                0.0,
                DOCK_X - 70.0,
                10.0,
                DOCK_Z + 2.0,
            ))
            .into();
        dock = dock
            .subtract(&centered_box(
                DOCK_X / 2.0 - 42.0,
                0.0,
                0.0,
                10.0,
                DOCK_Y - 76.0,
                DOCK_Z + 2.0,
            ))
            .into();
        dock = dock
            .subtract(&centered_box(
                DOCK_X / 2.0 - 58.0,
                -DOCK_Y / 2.0 + 58.0,
                0.0,
                58.0,
                38.0,
                DOCK_Z + 2.0,
            ))
            .into();

        dock = dock
            .subtract(&perimeter_mount_holes(DOCK_X, DOCK_Y, DOCK_Z + 2.0, 2.7))
            .into();
        dock = dock.union(&dock_reference_rails()).into();
        dock = dock.union(&logger_pockets()).into();
        dock = dock.union(&robot_lift_lands()).into();
        dock.union(&dock_leveling_pad_lands()).into()
    }

    fn service_bulkhead_test_block() -> Shape {
        let mut block = centered_box(0.0, 0.0, 0.0, BULKHEAD_X, BULKHEAD_Y, BULKHEAD_Z);
        for x in [-240.0, -210.0, -180.0, -150.0] {
            block = block
                .subtract(&centered_cyl_y(x, 0.0, 18.0, 4.0, BULKHEAD_Y + 4.0))
                .into();
        }
        for x in [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0] {
            block = block
                .subtract(&centered_cyl_y(x, 0.0, 0.0, 3.2, BULKHEAD_Y + 4.0))
                .into();
        }
        for x in [150.0, 176.0, 202.0, 228.0, 254.0] {
            block = block
                .subtract(&centered_cyl_y(x, 0.0, 0.0, 3.2, BULKHEAD_Y + 4.0))
                .into();
        }
        block = block
            .subtract(&centered_box(
                300.0,
                0.0,
                18.0,
                88.0,
                BULKHEAD_Y + 4.0,
                18.0,
            ))
            .into();

        block = block
            .union(&centered_box(
                0.0,
                -BULKHEAD_Y / 2.0 - 1.0,
                BULKHEAD_Z / 2.0 - 14.0,
                BULKHEAD_X - 72.0,
                2.0,
                8.0,
            ))
            .into();

        for i in 0..12 {
            block = block
                .union(&centered_box(
                    centered_index(i, 12, 22.0),
                    -BULKHEAD_Y / 2.0 - 12.0,
                    -20.0,
                    5.0,
                    24.0,
                    18.0,
                ))
                .into();
        }
        let cable_shelf =
            centered_box(300.0, -BULKHEAD_Y / 2.0 - 13.0, -20.0, 120.0, 26.0, 14.0).subtract(
                &centered_box(300.0, -BULKHEAD_Y / 2.0 - 13.0, -20.0, 92.0, 8.0, 16.0),
            );
        block.union(&cable_shelf.into()).into()
    }

    fn stackup_reference() -> Shape {
        let mut stack = incubator_dock_plate();
        stack = stack
            .union(&translate(
                lower_carrier(),
                0.0,
                0.0,
                DOCK_Z / 2.0 + CARRIER_Z / 2.0 + 8.0,
            ))
            .into();
        stack = stack
            .union(&translate(
                lid_clamp(),
                0.0,
                0.0,
                DOCK_Z / 2.0 + CARRIER_Z + LID_Z / 2.0 + 11.0,
            ))
            .into();
        stack = stack
            .union(&translate(
                window_placeholder(),
                0.0,
                0.0,
                DOCK_Z / 2.0 + CARRIER_Z + LID_Z + WINDOW_Z / 2.0 + 12.5,
            ))
            .into();
        stack = stack
            .union(&translate(
                service_bulkhead_test_block(),
                0.0,
                BULKHEAD_OFFSET_Y,
                DOCK_Z / 2.0 + BULKHEAD_Z / 2.0,
            ))
            .into();
        stack
            .union(&translate(
                gasket_witness_coupon(),
                -(DOCK_X / 2.0 - COUPON_X / 2.0 - 30.0),
                -(DOCK_Y / 2.0 + COUPON_Y / 2.0 + 38.0),
                COUPON_Z / 2.0,
            ))
            .into()
    }

    fn carrier_condition_id_land() -> Shape {
        centered_box(
            -CARRIER_X / 2.0 + 78.0,
            -CARRIER_Y / 2.0 + 28.0,
            CARRIER_Z / 2.0 + 1.0,
            96.0,
            24.0,
            2.0,
        )
        .union(&centered_box(
            -CARRIER_X / 2.0 + 185.0,
            -CARRIER_Y / 2.0 + 27.0,
            CARRIER_Z / 2.0 + 0.75,
            118.0,
            14.0,
            1.5,
        ))
        .into()
    }

    fn carrier_orientation_marker() -> Shape {
        centered_box(
            -CARRIER_X / 2.0 + 37.0,
            -CARRIER_Y / 2.0 + 54.0,
            CARRIER_Z / 2.0 + 1.5,
            34.0,
            5.0,
            3.0,
        )
        .union(&centered_box(
            -CARRIER_X / 2.0 + 22.5,
            -CARRIER_Y / 2.0 + 69.0,
            CARRIER_Z / 2.0 + 1.5,
            5.0,
            34.0,
            3.0,
        ))
        .into()
    }

    fn carrier_handling_lands() -> Shape {
        let positions = [
            (-112.0, -CARRIER_Y / 2.0 + 58.0),
            (112.0, -CARRIER_Y / 2.0 + 58.0),
            (-112.0, CARRIER_Y / 2.0 - 58.0),
            (112.0, CARRIER_Y / 2.0 - 58.0),
        ];
        let mut lands = centered_box(
            positions[0].0,
            positions[0].1,
            CARRIER_Z / 2.0 + 0.75,
            58.0,
            16.0,
            1.5,
        );
        for (x, y) in positions.into_iter().skip(1) {
            lands = lands
                .union(&centered_box(x, y, CARRIER_Z / 2.0 + 0.75, 58.0, 16.0, 1.5))
                .into();
        }
        lands
    }

    fn side_service_reliefs() -> Shape {
        centered_box(
            -(CARRIER_X / 2.0 - 18.0),
            0.0,
            CARRIER_Z / 2.0 + 4.0,
            24.0,
            SLOT_ARRAY_Y + 42.0,
            8.0,
        )
        .union(&centered_box(
            CARRIER_X / 2.0 - 18.0,
            0.0,
            CARRIER_Z / 2.0 + 4.0,
            24.0,
            SLOT_ARRAY_Y + 42.0,
            8.0,
        ))
        .into()
    }

    fn slot_label_land(row: usize, col: usize) -> Shape {
        let (x, y) = slot_center(row, col);
        centered_box(
            x - REVC_CHIP_LENGTH / 2.0 + 22.0,
            y + REVC_CHIP_WIDTH / 2.0 - 14.0,
            CARRIER_Z / 2.0 + 1.0,
            26.0,
            10.0,
            2.0,
        )
    }

    fn dock_reference_rails() -> Shape {
        let mut rails = centered_box(
            0.0,
            CARRIER_Y / 2.0 + 17.0,
            DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
            CARRIER_X + 44.0,
            DOCK_RAIL_W,
            DOCK_RAIL_Z,
        );
        rails = rails
            .union(&centered_box(
                -CARRIER_X / 2.0 - 17.0,
                0.0,
                DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0,
                DOCK_RAIL_W,
                CARRIER_Y + 46.0,
                DOCK_RAIL_Z,
            ))
            .into();
        rails
            .union(&centered_box(
                0.0,
                -CARRIER_Y / 2.0 - 14.0,
                DOCK_Z / 2.0 + 5.0,
                CARRIER_X + 44.0,
                10.0,
                10.0,
            ))
            .into()
    }

    fn logger_pockets() -> Shape {
        let positions = [
            (-(CARRIER_X / 2.0 + 34.0), CARRIER_Y / 2.0 - 30.0),
            (CARRIER_X / 2.0 + 34.0, CARRIER_Y / 2.0 - 30.0),
            (-(CARRIER_X / 2.0 + 34.0), -CARRIER_Y / 2.0 + 30.0),
            (CARRIER_X / 2.0 + 34.0, -CARRIER_Y / 2.0 + 30.0),
        ];
        let mut pockets = centered_box(
            positions[0].0,
            positions[0].1,
            DOCK_Z / 2.0 + 4.0,
            48.0,
            32.0,
            8.0,
        );
        for (x, y) in positions.into_iter().skip(1) {
            pockets = pockets
                .union(&centered_box(x, y, DOCK_Z / 2.0 + 4.0, 48.0, 32.0, 8.0))
                .into();
        }
        pockets
    }

    fn robot_lift_lands() -> Shape {
        centered_box(
            0.0,
            -DOCK_Y / 2.0 + 74.0,
            DOCK_Z / 2.0 + 3.5,
            160.0,
            20.0,
            7.0,
        )
        .union(&centered_box(
            0.0,
            DOCK_Y / 2.0 - 74.0,
            DOCK_Z / 2.0 + 3.5,
            160.0,
            20.0,
            7.0,
        ))
        .into()
    }

    fn dock_leveling_pad_lands() -> Shape {
        let positions = [
            (-(DOCK_X / 2.0 - 42.0), -(DOCK_Y / 2.0 - 42.0)),
            (DOCK_X / 2.0 - 42.0, -(DOCK_Y / 2.0 - 42.0)),
            (-(DOCK_X / 2.0 - 42.0), DOCK_Y / 2.0 - 42.0),
            (DOCK_X / 2.0 - 42.0, DOCK_Y / 2.0 - 42.0),
        ];
        let mut pads = centered_cyl_z(
            positions[0].0,
            positions[0].1,
            DOCK_Z / 2.0 + 1.5,
            16.0,
            3.0,
        );
        for (x, y) in positions.into_iter().skip(1) {
            pads = pads
                .union(&centered_cyl_z(x, y, DOCK_Z / 2.0 + 1.5, 16.0, 3.0))
                .into();
        }
        pads
    }

    fn perimeter_mount_holes(x: f64, y: f64, height: f64, radius: f64) -> Shape {
        let points = [
            (-(x / 2.0 - 22.0), -(y / 2.0 - 22.0)),
            (x / 2.0 - 22.0, -(y / 2.0 - 22.0)),
            (-(x / 2.0 - 22.0), y / 2.0 - 22.0),
            (x / 2.0 - 22.0, y / 2.0 - 22.0),
            (0.0, -(y / 2.0 - 22.0)),
            (0.0, y / 2.0 - 22.0),
        ];
        let mut holes = centered_cyl_z(points[0].0, points[0].1, 0.0, radius, height);
        for (px, py) in points.into_iter().skip(1) {
            holes = holes
                .union(&centered_cyl_z(px, py, 0.0, radius, height))
                .into();
        }
        holes
    }

    fn rectangular_frame(x: f64, y: f64, z: f64, wall: f64) -> Shape {
        centered_box(0.0, 0.0, 0.0, x, y, z)
            .subtract(&centered_box(
                0.0,
                0.0,
                0.0,
                x - wall * 2.0,
                y - wall * 2.0,
                z + 2.0,
            ))
            .into()
    }

    fn centered_box(cx: f64, cy: f64, cz: f64, x: f64, y: f64, z: f64) -> Shape {
        Workplane::xy()
            .translated(dvec3(cx, cy, cz - z / 2.0))
            .rect(x, y)
            .to_face()
            .extrude(dvec3(0.0, 0.0, z))
            .into()
    }

    fn centered_cyl_z(cx: f64, cy: f64, cz: f64, radius: f64, height: f64) -> Shape {
        Workplane::xy()
            .translated(dvec3(cx, cy, cz - height / 2.0))
            .circle(0.0, 0.0, radius)
            .to_face()
            .extrude(dvec3(0.0, 0.0, height))
            .into()
    }

    fn centered_cyl_y(cx: f64, cy: f64, cz: f64, radius: f64, height: f64) -> Shape {
        Workplane::xz()
            .translated(dvec3(cx, cy - height / 2.0, cz))
            .circle(0.0, 0.0, radius)
            .to_face()
            .extrude(dvec3(0.0, height, 0.0))
            .into()
    }

    fn translate(mut shape: Shape, x: f64, y: f64, z: f64) -> Shape {
        shape.set_global_translation(dvec3(x, y, z));
        shape
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

    fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
        (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
    }

    fn assert_layout() {
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(OUTPUTS.len(), 7);
        assert!(CARRIER_X > SLOT_ARRAY_X);
        assert!(CARRIER_Y > SLOT_ARRAY_Y);
        assert!(DOCK_X > CARRIER_X + 100.0);
        assert!(DOCK_Y > CARRIER_Y + 100.0);
        assert!(BULKHEAD_X > SLOT_ARRAY_X);
        assert!(LID_X > CARRIER_X);
        assert!(LID_Y > CARRIER_Y);
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/rfq/sixteen_slot_")));
    }
}

#[cfg(feature = "step")]
fn main() {
    step_export::main();
}
