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
    use laminarforge_cad::{sixteen_slot_cassette_a0::*, REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
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

    pub fn main() {
        validate_contract().expect("invalid 16-slot A0 interface contract");
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
        println!("  Carrier base body:    {CARRIER_X:.1} x {CARRIER_Y:.1} x {CARRIER_Z:.1} mm");
        println!("  Lid/clamp base body:  {LID_X:.1} x {LID_Y:.1} x {LID_Z:.1} mm");
        println!("  Dock plate base body: {DOCK_X:.1} x {DOCK_Y:.1} x {DOCK_Z:.1} mm");
        println!("  Bulkhead:             {BULKHEAD_X:.1} x {BULKHEAD_Y:.1} x {BULKHEAD_Z:.1} mm");
        println!(
            "  Pocket clearance:     {CHIP_CLEARANCE:.2} mm/side STEP draft; {DRAWING_TARGET_CHIP_CLEARANCE:.2} mm/side drawing target after chip lot measurement"
        );
        println!(
            "  Gasket compression:   {GASKET_FREE_HEIGHT:.2} mm free -> {GASKET_COMPRESSED_HEIGHT:.2} mm target"
        );
        println!("  Closure plane:        {CLOSURE_PLANE_ABOVE_CARRIER:.2} mm above carrier top");
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
                    top_face_cut_z(CARRIER_Z, CHIP_POCKET_DEPTH),
                    REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
                    top_face_cut_height(CHIP_POCKET_DEPTH),
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
                            PER_SLOT_GASKET_OUTER_X,
                            PER_SLOT_GASKET_OUTER_Y,
                            GASKET_LAND_Z,
                            GASKET_LAND_W,
                        ),
                        x,
                        y,
                        CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0,
                    ))
                    .into();

                carrier = carrier.union(&slot_label_land(row, col)).into();
            }
        }

        let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
        for (x, y) in inter_slot_stop_points() {
            carrier = carrier
                .union(&centered_cyl_z(
                    x,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    INTERNAL_STOP_DIAMETER / 2.0,
                    stop_z,
                ))
                .into();
        }

        carrier = carrier
            .subtract(&translate(
                rectangular_frame(
                    LEAK_GUTTER_OUTER_X,
                    LEAK_GUTTER_OUTER_Y,
                    top_face_cut_height(LEAK_GUTTER_DEPTH),
                    LEAK_GUTTER_W,
                ),
                0.0,
                0.0,
                top_face_cut_z(CARRIER_Z, LEAK_GUTTER_DEPTH),
            ))
            .into();
        carrier = carrier
            .subtract(&centered_cyl_y(
                CARRIER_DRAIN_X,
                CARRIER_DRAIN_Y,
                CARRIER_DRAIN_Z,
                CARRIER_DRAIN_DIAMETER / 2.0,
                CARRIER_DRAIN_LENGTH,
            ))
            .into();

        carrier = carrier
            .union(&translate(
                rectangular_frame(
                    PERIMETER_GASKET_OUTER_X,
                    PERIMETER_GASKET_OUTER_Y,
                    GASKET_LAND_Z,
                    PERIMETER_GASKET_W,
                ),
                0.0,
                0.0,
                CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0,
            ))
            .into();

        let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
        let x_edge = PERIMETER_GASKET_OUTER_X / 2.0 + PERIMETER_STOP_CENTER_OFFSET;
        let y_edge = PERIMETER_GASKET_OUTER_Y / 2.0 + PERIMETER_STOP_CENTER_OFFSET;
        for x in [-240.0, -120.0, 0.0, 120.0, 240.0] {
            carrier = carrier
                .union(&centered_box(
                    x,
                    -y_edge,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    18.0,
                    PERIMETER_STOP_W,
                    stop_z,
                ))
                .into();
            carrier = carrier
                .union(&centered_box(
                    x,
                    y_edge,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    18.0,
                    PERIMETER_STOP_W,
                    stop_z,
                ))
                .into();
        }
        for y in [-176.0, -88.0, 0.0, 88.0, 176.0] {
            carrier = carrier
                .union(&centered_box(
                    -x_edge,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    PERIMETER_STOP_W,
                    18.0,
                    stop_z,
                ))
                .into();
            carrier = carrier
                .union(&centered_box(
                    x_edge,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0,
                    PERIMETER_STOP_W,
                    18.0,
                    stop_z,
                ))
                .into();
        }

        for datum in datum_features() {
            let boss = centered_cyl_z(
                datum.x,
                datum.y,
                CARRIER_Z / 2.0 + DATUM_BOSS_Z / 2.0,
                DATUM_BOSS_DIAMETER / 2.0,
                DATUM_BOSS_Z,
            );
            let bore = datum_bore(datum);
            carrier = carrier.union(&boss.subtract(&bore).into()).into();
        }

        carrier = carrier.union(&carrier_condition_id_land()).into();
        carrier = carrier.union(&carrier_orientation_marker()).into();
        carrier = carrier.union(&side_service_reliefs()).into();
        carrier = carrier
            .subtract(&perimeter_mount_holes(
                CARRIER_X,
                CARRIER_Y,
                CARRIER_Z + 2.0,
            ))
            .into();
        carrier = carrier.subtract(&carrier_lid_receiver_holes()).into();
        carrier = carrier.subtract(&datum_bore_cuts()).into();
        carrier
    }

    fn datum_bore(datum: DatumFeature) -> Shape {
        let z = CARRIER_Z / 2.0 + DATUM_BOSS_Z / 2.0;
        let height = DATUM_BOSS_Z + 2.0;
        match datum.role {
            DatumRole::RoundLocator => {
                centered_cyl_z(datum.x, datum.y, z, DATUM_D1_BORE_DIAMETER / 2.0, height)
            }
            DatumRole::RelievedLocator => {
                let relief = (DATUM_D2_SLOT_LENGTH - DATUM_D2_SLOT_WIDTH) / 2.0;
                centered_cyl_z(
                    datum.x - relief,
                    datum.y,
                    z,
                    DATUM_D2_SLOT_WIDTH / 2.0,
                    height,
                )
                .union(&centered_box(
                    datum.x,
                    datum.y,
                    z,
                    relief * 2.0,
                    DATUM_D2_SLOT_WIDTH,
                    height,
                ))
                .union(&centered_cyl_z(
                    datum.x + relief,
                    datum.y,
                    z,
                    DATUM_D2_SLOT_WIDTH / 2.0,
                    height,
                ))
                .into()
            }
            DatumRole::ClearanceWitness => centered_cyl_z(
                datum.x,
                datum.y,
                z,
                DATUM_WITNESS_BORE_DIAMETER / 2.0,
                height,
            ),
        }
    }

    fn datum_bore_cuts() -> Shape {
        let datums = datum_features();
        let mut bores = datum_bore(datums[0]);
        for datum in datums.into_iter().skip(1) {
            bores = bores.union(&datum_bore(datum)).into();
        }
        bores
    }

    fn lid_clamp() -> Shape {
        let seal_skin = centered_box(
            0.0,
            0.0,
            -LID_Z / 2.0 + LID_UNDERSIDE_SEAL_SKIN_Z / 2.0,
            LID_X,
            LID_Y,
            LID_UNDERSIDE_SEAL_SKIN_Z,
        );
        let upper_relief = centered_box(
            0.0,
            0.0,
            top_face_cut_z(LID_Z, LID_UPPER_FRAME_Z),
            LID_UPPER_RELIEF_X,
            LID_UPPER_RELIEF_Y,
            top_face_cut_height(LID_UPPER_FRAME_Z),
        );
        let upper_frame: Shape = centered_box(
            0.0,
            0.0,
            LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0,
            LID_X,
            LID_Y,
            LID_UPPER_FRAME_Z,
        )
        .subtract(&upper_relief)
        .into();
        let mut lid: Shape = seal_skin.union(&upper_frame).into();

        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                lid = lid
                    .subtract(&centered_box(
                        x,
                        y,
                        0.0,
                        LID_SLOT_VIEW_OPENING_X,
                        LID_SLOT_VIEW_OPENING_Y,
                        LID_Z + 2.0,
                    ))
                    .into();
                lid = lid
                    .subtract(&centered_box(
                        x,
                        y,
                        bottom_face_cut_z(LID_Z, LID_CHIP_TOP_RELIEF_DEPTH),
                        LID_CHIP_TOP_RELIEF_X,
                        LID_CHIP_TOP_RELIEF_Y,
                        bottom_face_cut_height(LID_CHIP_TOP_RELIEF_DEPTH),
                    ))
                    .into();
            }
        }

        for (x, y) in fastener_points() {
            let retainer =
                centered_cyl_z(x, y, LID_Z / 2.0 + 0.8, LID_RETAINER_DIAMETER / 2.0, 1.6)
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
                .union(&centered_box(
                    x,
                    0.0,
                    LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0,
                    10.0,
                    SLOT_ARRAY_Y + 38.0,
                    LID_UPPER_FRAME_Z,
                ))
                .into();
        }
        for row in 1..ROWS {
            let y = centered_index(row, ROWS, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
            lid = lid
                .union(&centered_box(
                    0.0,
                    y,
                    LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0,
                    SLOT_ARRAY_X + 42.0,
                    10.0,
                    LID_UPPER_FRAME_Z,
                ))
                .into();
        }

        for (x, y) in datum_points() {
            lid = lid
                .union(&centered_box(
                    x,
                    y,
                    LID_Z / 2.0 - LID_UPPER_FRAME_Z / 2.0,
                    34.0,
                    22.0,
                    LID_UPPER_FRAME_Z,
                ))
                .into();
        }

        lid = lid
            .union(&translate(
                rectangular_frame(SLOT_ARRAY_X + 96.0, SLOT_ARRAY_Y + 88.0, 1.6, 6.0),
                0.0,
                0.0,
                LID_Z / 2.0 + 0.8,
            ))
            .into();

        lid = lid.subtract(&lid_fastener_hole_cuts()).into();
        lid = lid.subtract(&lid_alignment_pin_seat_cuts()).into();
        lid.subtract(&lid_gasket_groove_cuts()).into()
    }

    fn lid_alignment_pin_seat_cuts() -> Shape {
        let datums = datum_features();
        let locating = datums.into_iter().filter(|datum| {
            matches!(
                datum.role,
                DatumRole::RoundLocator | DatumRole::RelievedLocator
            )
        });
        let mut seats: Option<Shape> = None;
        for datum in locating {
            let seat = centered_cyl_z(
                datum.x,
                datum.y,
                bottom_face_cut_z(LID_Z, LID_DATUM_PIN_SEAT_DEPTH),
                LID_DATUM_PIN_SEAT_DIAMETER / 2.0,
                bottom_face_cut_height(LID_DATUM_PIN_SEAT_DEPTH),
            );
            seats = Some(match seats {
                Some(existing) => existing.union(&seat).into(),
                None => seat,
            });
        }
        seats.expect("D1/D2 replaceable alignment pin seats must exist")
    }

    fn lid_alignment_pin_surrogates() -> Shape {
        let datums = datum_features();
        let locating = datums.into_iter().filter(|datum| {
            matches!(
                datum.role,
                DatumRole::RoundLocator | DatumRole::RelievedLocator
            )
        });
        let mut pins: Option<Shape> = None;
        for datum in locating {
            let pin = centered_cyl_z(
                datum.x,
                datum.y,
                -LID_Z / 2.0 + (LID_DATUM_PIN_EMBEDMENT - LID_DATUM_PIN_EXTENSION) / 2.0,
                LID_DATUM_PIN_DIAMETER / 2.0,
                LID_DATUM_PIN_TOTAL_Z,
            );
            pins = Some(match pins {
                Some(existing) => existing.union(&pin).into(),
                None => pin,
            });
        }
        pins.expect("D1/D2 replaceable alignment pin surrogates must exist")
    }

    fn lid_fastener_hole_cuts() -> Shape {
        let points = fastener_points();
        let mut holes = centered_cyl_z(
            points[0].0,
            points[0].1,
            0.0,
            LID_FASTENER_CLEARANCE_DIAMETER / 2.0,
            LID_Z + 2.0,
        );
        for (x, y) in points.into_iter().skip(1) {
            holes = holes
                .union(&centered_cyl_z(
                    x,
                    y,
                    0.0,
                    LID_FASTENER_CLEARANCE_DIAMETER / 2.0,
                    LID_Z + 2.0,
                ))
                .into();
        }
        holes
    }

    fn lid_gasket_groove_cuts() -> Shape {
        let mut grooves: Option<Shape> = None;
        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                let groove = translate(
                    rectangular_frame(
                        PER_SLOT_GASKET_GROOVE_OUTER_X,
                        PER_SLOT_GASKET_GROOVE_OUTER_Y,
                        GASKET_GROOVE_CUT_HEIGHT,
                        GASKET_GROOVE_W,
                    ),
                    x,
                    y,
                    bottom_face_gasket_groove_cut_z(LID_Z),
                );
                grooves = Some(match grooves {
                    Some(existing) => existing.union(&groove).into(),
                    None => groove,
                });
            }
        }

        let perimeter = translate(
            rectangular_frame(
                PERIMETER_GASKET_GROOVE_OUTER_X,
                PERIMETER_GASKET_GROOVE_OUTER_Y,
                GASKET_GROOVE_CUT_HEIGHT,
                GASKET_GROOVE_W,
            ),
            0.0,
            0.0,
            bottom_face_gasket_groove_cut_z(LID_Z),
        );
        grooves
            .expect("16-slot gasket groove set must not be empty")
            .union(&perimeter)
            .into()
    }

    fn window_placeholder() -> Shape {
        let panel_x = WINDOW_X;
        let panel_y = WINDOW_Y;
        let mut window = centered_box(0.0, 0.0, 0.0, panel_x, panel_y, WINDOW_Z);

        for row in 0..ROWS {
            for col in 0..COLS {
                let (x, y) = slot_center(row, col);
                window = window
                    .union(&translate(
                        rectangular_frame(
                            REVC_CHIP_LENGTH - 18.0,
                            REVC_CHIP_WIDTH - 18.0,
                            WINDOW_WITNESS_FRAME_BODY_Z,
                            3.0,
                        ),
                        x,
                        y,
                        WINDOW_WITNESS_FRAME_CENTER_Z,
                    ))
                    .into();
            }
        }

        for (x, y) in window_fiducial_points() {
            let fiducial =
                centered_cyl_z(x, y, WINDOW_FIDUCIAL_CENTER_Z, 5.0, WINDOW_FIDUCIAL_BODY_Z)
                    .subtract(&centered_cyl_z(
                        x,
                        y,
                        WINDOW_FIDUCIAL_CENTER_Z,
                        1.2,
                        WINDOW_FIDUCIAL_HOLE_CUT_Z,
                    ));
            window = window.union(&fiducial.into()).into();
        }

        for x in [-210.0, -70.0, 70.0, 210.0] {
            window = window
                .union(&centered_box(
                    x,
                    -panel_y / 2.0 + 8.0,
                    WINDOW_RETENTION_TAB_CENTER_Z,
                    34.0,
                    9.0,
                    WINDOW_RETENTION_TAB_BODY_Z,
                ))
                .into();
            window = window
                .union(&centered_box(
                    x,
                    panel_y / 2.0 - 8.0,
                    WINDOW_RETENTION_TAB_CENTER_Z,
                    34.0,
                    9.0,
                    WINDOW_RETENTION_TAB_BODY_Z,
                ))
                .into();
        }
        for y in [-150.0, -50.0, 50.0, 150.0] {
            window = window
                .union(&centered_box(
                    -panel_x / 2.0 + 8.0,
                    y,
                    WINDOW_RETENTION_TAB_CENTER_Z,
                    9.0,
                    34.0,
                    WINDOW_RETENTION_TAB_BODY_Z,
                ))
                .into();
            window = window
                .union(&centered_box(
                    panel_x / 2.0 - 8.0,
                    y,
                    WINDOW_RETENTION_TAB_CENTER_Z,
                    9.0,
                    34.0,
                    WINDOW_RETENTION_TAB_BODY_Z,
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
                COUPON_SAMPLE_SLOT_CENTER_Y,
                0.0,
                COUPON_SAMPLE_SLOT_X,
                COUPON_SAMPLE_SLOT_Y,
                COUPON_Z + 2.0,
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
                    COUPON_SQUEEZE_STEP_CENTER_Y,
                    COUPON_Z / 2.0 + h / 2.0,
                    COUPON_SQUEEZE_STEP_X,
                    COUPON_SQUEEZE_STEP_Y,
                    *h,
                ))
                .into();
        }

        for (x, y) in coupon_stop_points() {
            coupon = coupon
                .union(&centered_cyl_z(
                    x,
                    y,
                    COUPON_Z / 2.0 + GASKET_COMPRESSED_HEIGHT / 2.0,
                    INTERNAL_STOP_DIAMETER / 2.0,
                    GASKET_COMPRESSED_HEIGHT,
                ))
                .into();
        }
        coupon = coupon
            .union(&centered_box(
                -70.0,
                COUPON_LABEL_CENTER_Y,
                COUPON_Z / 2.0 + 1.0,
                84.0,
                COUPON_LABEL_LAND_Y,
                2.0,
            ))
            .into();
        coupon = coupon
            .union(&centered_box(
                62.0,
                COUPON_LABEL_CENTER_Y,
                COUPON_Z / 2.0 + 1.0,
                102.0,
                COUPON_LABEL_LAND_Y,
                2.0,
            ))
            .into();

        coupon.subtract(&coupon_gasket_groove_cuts()).into()
    }

    fn coupon_gasket_groove_cuts() -> Shape {
        translate(
            rectangular_frame(
                COUPON_LEAK_LOOP_X,
                COUPON_LEAK_LOOP_Y,
                GASKET_GROOVE_CUT_HEIGHT,
                GASKET_GROOVE_W,
            ),
            COUPON_LEAK_LOOP_CENTER_X,
            COUPON_LOOP_CENTER_Y,
            top_face_gasket_groove_cut_z(COUPON_Z),
        )
        .union(&translate(
            rectangular_frame(
                COUPON_RECONNECTION_LOOP_X,
                COUPON_RECONNECTION_LOOP_Y,
                GASKET_GROOVE_CUT_HEIGHT,
                GASKET_GROOVE_W,
            ),
            COUPON_RECONNECTION_LOOP_CENTER_X,
            COUPON_LOOP_CENTER_Y,
            top_face_gasket_groove_cut_z(COUPON_Z),
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
                        dock_slot_recess_cut_z(),
                        DOCK_SLOT_RECESS_X,
                        DOCK_SLOT_RECESS_Y,
                        dock_slot_recess_cut_height(),
                    ))
                    .into();
            }
        }

        for y in dock_air_bypass_center_ys() {
            dock = dock
                .subtract(&centered_box(
                    0.0,
                    y,
                    DOCK_THROUGH_CUT_CENTER_Z,
                    DOCK_AIR_BYPASS_X,
                    DOCK_AIR_BYPASS_Y,
                    DOCK_THROUGH_CUT_HEIGHT,
                ))
                .into();
        }

        dock = dock
            .subtract(&centered_box(
                0.0,
                DOCK_FRONT_DRAIN_OPENING_CENTER_Y,
                DOCK_THROUGH_CUT_CENTER_Z,
                DOCK_FRONT_DRAIN_OPENING_X,
                DOCK_FRONT_DRAIN_OPENING_Y,
                DOCK_THROUGH_CUT_HEIGHT,
            ))
            .into();
        dock = dock
            .subtract(&centered_box(
                DOCK_SIDE_DRAIN_OPENING_CENTER_X,
                0.0,
                DOCK_THROUGH_CUT_CENTER_Z,
                DOCK_SIDE_DRAIN_OPENING_X,
                DOCK_SIDE_DRAIN_OPENING_Y,
                DOCK_THROUGH_CUT_HEIGHT,
            ))
            .into();
        dock = dock
            .subtract(&centered_box(
                DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X,
                DOCK_DRAIN_VISIBILITY_OPENING_CENTER_Y,
                DOCK_THROUGH_CUT_CENTER_Z,
                DOCK_DRAIN_VISIBILITY_OPENING_X,
                DOCK_DRAIN_VISIBILITY_OPENING_Y,
                DOCK_THROUGH_CUT_HEIGHT,
            ))
            .into();

        dock = dock
            .subtract(&perimeter_mount_holes(
                DOCK_X,
                DOCK_Y,
                DOCK_THROUGH_CUT_HEIGHT,
            ))
            .into();
        for row in 0..ROWS {
            for col in 0..COLS {
                let (token_x, token_y) = dock_position_token_point(row, col);
                dock = dock
                    .union(&centered_box(
                        token_x,
                        token_y,
                        dock_top_feature_center_z(DOCK_POSITION_TOKEN_Z),
                        DOCK_POSITION_TOKEN_X,
                        DOCK_POSITION_TOKEN_Y,
                        DOCK_POSITION_TOKEN_Z,
                    ))
                    .into();
            }
        }
        dock = dock.union(&dock_reference_rails()).into();
        dock = dock.union(&logger_reservation_lands()).into();
        dock = dock.union(&robot_lift_lands()).into();
        dock.union(&dock_leveling_pad_lands()).into()
    }

    fn service_bulkhead_test_block() -> Shape {
        let mut block = centered_box(0.0, 0.0, 0.0, BULKHEAD_X, BULKHEAD_Y, BULKHEAD_Z);
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
        let cable_shelf = centered_box(
            SENSOR_CONNECTOR_CENTER_X,
            -BULKHEAD_Y / 2.0 - 13.0,
            -20.0,
            120.0,
            26.0,
            14.0,
        )
        .subtract(&centered_box(
            SENSOR_CONNECTOR_CENTER_X,
            -BULKHEAD_Y / 2.0 - 13.0,
            -20.0,
            92.0,
            8.0,
            16.0,
        ));
        block = block.union(&cable_shelf.into()).into();

        for x in GAS_PORT_XS {
            block = block
                .subtract(&centered_cyl_y(
                    x,
                    0.0,
                    18.0,
                    GAS_PORT_DIAMETER / 2.0,
                    BULKHEAD_Y + 4.0,
                ))
                .into();
        }
        for x in MEDIA_PORT_XS {
            block = block
                .subtract(&centered_cyl_y(
                    x,
                    0.0,
                    0.0,
                    MEDIA_WASTE_PORT_DIAMETER / 2.0,
                    BULKHEAD_Y + 4.0,
                ))
                .into();
        }
        for x in WASTE_PORT_XS {
            block = block
                .subtract(&centered_cyl_y(
                    x,
                    0.0,
                    0.0,
                    MEDIA_WASTE_PORT_DIAMETER / 2.0,
                    BULKHEAD_Y + 4.0,
                ))
                .into();
        }
        block
            .subtract(&centered_box(
                SENSOR_CONNECTOR_CENTER_X,
                0.0,
                SENSOR_CONNECTOR_CENTER_Z,
                SENSOR_CONNECTOR_X,
                BULKHEAD_Y + 4.0,
                SENSOR_CONNECTOR_Z,
            ))
            .into()
    }

    fn stackup_reference() -> Shape {
        let carrier_center_z = DOCK_SUPPORT_PLANE_Z + CARRIER_Z / 2.0;
        let carrier_top_z = carrier_center_z + CARRIER_Z / 2.0;
        let lid_center_z = carrier_top_z + CLOSURE_PLANE_ABOVE_CARRIER + LID_Z / 2.0;
        let window_center_z = lid_center_z + LID_Z / 2.0 + WINDOW_Z / 2.0 + 2.0;
        let mut stack = incubator_dock_plate();
        stack = stack
            .union(&translate(lower_carrier(), 0.0, 0.0, carrier_center_z))
            .into();
        stack = stack
            .union(&translate(lid_clamp(), 0.0, 0.0, lid_center_z))
            .into();
        stack = stack
            .union(&translate(
                lid_alignment_pin_surrogates(),
                0.0,
                0.0,
                lid_center_z,
            ))
            .into();
        stack = stack
            .union(&translate(window_placeholder(), 0.0, 0.0, window_center_z))
            .into();
        stack = stack
            .union(&translate(
                service_bulkhead_test_block(),
                0.0,
                BULKHEAD_OFFSET_Y,
                DOCK_SUPPORT_PLANE_Z + BULKHEAD_Z / 2.0,
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
            GLOBAL_BARCODE_CENTER_X,
            GLOBAL_BARCODE_CENTER_Y,
            CARRIER_Z / 2.0 + 1.0,
            GLOBAL_BARCODE_LAND_X,
            GLOBAL_BARCODE_LAND_Y,
            2.0,
        )
        .union(&centered_box(
            GLOBAL_TEXT_CENTER_X,
            GLOBAL_TEXT_CENTER_Y,
            CARRIER_Z / 2.0 + 0.75,
            GLOBAL_TEXT_LAND_X,
            GLOBAL_TEXT_LAND_Y,
            1.5,
        ))
        .into()
    }

    fn carrier_orientation_marker() -> Shape {
        centered_box(
            -CARRIER_X / 2.0 + 16.0,
            GLOBAL_BARCODE_CENTER_Y,
            CARRIER_Z / 2.0 + 1.5,
            18.0,
            4.0,
            3.0,
        )
        .union(&centered_box(
            -CARRIER_X / 2.0 + 9.0,
            GLOBAL_BARCODE_CENTER_Y + 7.0,
            CARRIER_Z / 2.0 + 1.5,
            4.0,
            18.0,
            3.0,
        ))
        .into()
    }

    fn side_service_reliefs() -> Shape {
        let reliefs: Shape = centered_box(
            -(CARRIER_X / 2.0 - 7.0),
            0.0,
            CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
            12.0,
            LEAK_GUTTER_INNER_Y - 16.0,
            SIDE_SERVICE_RELIEF_Z,
        )
        .union(&centered_box(
            CARRIER_X / 2.0 - 7.0,
            0.0,
            CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
            12.0,
            LEAK_GUTTER_INNER_Y - 16.0,
            SIDE_SERVICE_RELIEF_Z,
        ))
        .into();
        reliefs.subtract(&side_service_datum_keepouts()).into()
    }

    fn side_service_datum_keepouts() -> Shape {
        let datums = datum_features();
        let mut keepouts = centered_cyl_z(
            datums[0].x,
            datums[0].y,
            CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
            DATUM_BOSS_DIAMETER / 2.0 + SIDE_SERVICE_DATUM_CLEARANCE,
            SIDE_SERVICE_RELIEF_Z + 2.0,
        );
        for datum in datums.into_iter().skip(1) {
            keepouts = keepouts
                .union(&centered_cyl_z(
                    datum.x,
                    datum.y,
                    CARRIER_Z / 2.0 + SIDE_SERVICE_RELIEF_Z / 2.0,
                    DATUM_BOSS_DIAMETER / 2.0 + SIDE_SERVICE_DATUM_CLEARANCE,
                    SIDE_SERVICE_RELIEF_Z + 2.0,
                ))
                .into();
        }
        keepouts
    }

    fn carrier_lid_receiver_holes() -> Shape {
        let points = fastener_points();
        let mut holes = centered_cyl_z(
            points[0].0,
            points[0].1,
            0.0,
            CARRIER_LID_RECEIVER_DIAMETER / 2.0,
            CARRIER_Z + 2.0,
        );
        for (x, y) in points.into_iter().skip(1) {
            holes = holes
                .union(&centered_cyl_z(
                    x,
                    y,
                    0.0,
                    CARRIER_LID_RECEIVER_DIAMETER / 2.0,
                    CARRIER_Z + 2.0,
                ))
                .into();
        }
        holes
    }

    fn slot_label_land(row: usize, col: usize) -> Shape {
        let slot = slot_number(row, col);
        centered_box(
            centered_index(slot - 1, SLOT_COUNT, SLOT_LABEL_PITCH_X),
            SLOT_LABEL_CENTER_Y,
            CARRIER_Z / 2.0 + 1.0,
            SLOT_LABEL_LAND_X,
            SLOT_LABEL_LAND_Y,
            2.0,
        )
    }

    fn dock_reference_rails() -> Shape {
        let mut rails = centered_box(
            0.0,
            DOCK_REAR_RAIL_CENTER_Y,
            dock_top_feature_center_z(DOCK_RAIL_Z),
            DOCK_REAR_RAIL_X,
            DOCK_RAIL_W,
            DOCK_RAIL_Z,
        );
        rails = rails
            .union(&centered_box(
                DOCK_LEFT_RAIL_CENTER_X,
                0.0,
                dock_top_feature_center_z(DOCK_RAIL_Z),
                DOCK_RAIL_W,
                DOCK_LEFT_RAIL_Y,
                DOCK_RAIL_Z,
            ))
            .into();
        rails
            .union(&centered_box(
                0.0,
                DOCK_FRONT_LIP_CENTER_Y,
                dock_top_feature_center_z(DOCK_FRONT_LIP_Z),
                DOCK_FRONT_LIP_X,
                DOCK_FRONT_LIP_W,
                DOCK_FRONT_LIP_Z,
            ))
            .into()
    }

    fn logger_reservation_lands() -> Shape {
        let positions = dock_logger_reservation_land_points();
        let mut lands = centered_box(
            positions[0].0,
            positions[0].1,
            dock_top_feature_center_z(DOCK_LOGGER_RESERVATION_LAND_Z),
            DOCK_LOGGER_RESERVATION_LAND_X,
            DOCK_LOGGER_RESERVATION_LAND_Y,
            DOCK_LOGGER_RESERVATION_LAND_Z,
        );
        for (x, y) in positions.into_iter().skip(1) {
            lands = lands
                .union(&centered_box(
                    x,
                    y,
                    dock_top_feature_center_z(DOCK_LOGGER_RESERVATION_LAND_Z),
                    DOCK_LOGGER_RESERVATION_LAND_X,
                    DOCK_LOGGER_RESERVATION_LAND_Y,
                    DOCK_LOGGER_RESERVATION_LAND_Z,
                ))
                .into();
        }
        lands
    }

    fn robot_lift_lands() -> Shape {
        let points = dock_robot_lift_points();
        centered_box(
            points[0].0,
            points[0].1,
            dock_top_feature_center_z(DOCK_ROBOT_LIFT_Z),
            DOCK_ROBOT_LIFT_X,
            DOCK_ROBOT_LIFT_Y,
            DOCK_ROBOT_LIFT_Z,
        )
        .union(&centered_box(
            points[1].0,
            points[1].1,
            dock_top_feature_center_z(DOCK_ROBOT_LIFT_Z),
            DOCK_ROBOT_LIFT_X,
            DOCK_ROBOT_LIFT_Y,
            DOCK_ROBOT_LIFT_Z,
        ))
        .into()
    }

    fn dock_leveling_pad_lands() -> Shape {
        let positions = dock_leveling_pad_points();
        let mut pads = centered_cyl_z(
            positions[0].0,
            positions[0].1,
            dock_top_feature_center_z(DOCK_LEVELING_PAD_Z),
            DOCK_LEVELING_PAD_RADIUS,
            DOCK_LEVELING_PAD_Z,
        );
        for (x, y) in positions.into_iter().skip(1) {
            pads = pads
                .union(&centered_cyl_z(
                    x,
                    y,
                    dock_top_feature_center_z(DOCK_LEVELING_PAD_Z),
                    DOCK_LEVELING_PAD_RADIUS,
                    DOCK_LEVELING_PAD_Z,
                ))
                .into();
        }
        pads
    }

    fn perimeter_mount_holes(x: f64, y: f64, height: f64) -> Shape {
        let points = perimeter_mount_points(x, y);
        let radius = PERIMETER_MOUNT_HOLE_DIAMETER / 2.0;
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
