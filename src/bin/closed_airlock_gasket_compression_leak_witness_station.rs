use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed automated incubator door/airlock gasket compression and leak witness
// station.
//
// This standalone generator models a validation fixture for contained door and
// airlock gasket coupons. It packages mechanical witness geometry for repeatable
// compression checks, hinge/latch repeatability observations, smoke/dye leak
// path visualization, pressure-decay tap routing, condensate capture, door-cycle
// trace tokens, clean/used segregation, evidence imaging, and robot/service
// keepouts. It is validation CAD only: biological protocols, leak acceptance
// thresholds, pressure recipes, and release decisions remain external controls.

const OUTPUT_PREFIX: &str = "closed_airlock_gasket_compression_leak_witness_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_airlock_gasket_compression_leak_witness_station_base_containment_tray.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_gasket_coupon_nest_bank.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_compression_height_witness_gauges.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_hinge_latch_repeatability_pads.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_smoke_dye_leak_path_witness_slots.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_pressure_decay_tap_manifold.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_condensate_gutter.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_door_cycle_token_rail.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_barcode_status_lands.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_clean_used_segregation.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_evidence_camera_bridge.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_robot_service_keepouts.stl",
    "output/closed_airlock_gasket_compression_leak_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "gasket_coupon_nest_bank",
    "compression_height_witness_gauges",
    "hinge_latch_repeatability_pads",
    "smoke_dye_leak_path_witness_slots",
    "pressure_decay_tap_manifold",
    "condensate_gutter",
    "door_cycle_token_rail",
    "barcode_status_lands",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 26.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_RECESS_Z: f64 = 8.0;
const MOUNT_SLOT_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const COUPON_BANK_X: f64 = 700.0;
const COUPON_BANK_Y: f64 = 300.0;
const COUPON_BANK_Z: f64 = 44.0;
const COUPON_COLS: usize = 4;
const COUPON_ROWS: usize = 3;
const GASKET_COUPON_COUNT: usize = COUPON_COLS * COUPON_ROWS;
const COUPON_PITCH_X: f64 = 154.0;
const COUPON_PITCH_Y: f64 = 82.0;
const COUPON_NEST_X: f64 = 120.0;
const COUPON_NEST_Y: f64 = 50.0;
const COUPON_NEST_DEPTH: f64 = 13.0;
const GASKET_FRAME_W: f64 = 8.0;
const COUPON_BANK_POS: (f64, f64) = (-310.0, 210.0);

const GAUGE_PANEL_X: f64 = 440.0;
const GAUGE_PANEL_Y: f64 = 210.0;
const GAUGE_PANEL_Z: f64 = 40.0;
const GAUGE_COUNT: usize = 8;
const GAUGE_STEP_COUNT: usize = 5;
const GAUGE_PITCH_X: f64 = 48.0;
const GAUGE_STEP_PITCH_Y: f64 = 30.0;
const GAUGE_MIN_HEIGHT: f64 = 2.0;
const GAUGE_STEP_DELTA: f64 = 0.55;
const GAUGE_PANEL_POS: (f64, f64) = (420.0, 250.0);

const REPEATABILITY_X: f64 = 520.0;
const REPEATABILITY_Y: f64 = 170.0;
const REPEATABILITY_Z: f64 = 38.0;
const HINGE_PAD_COUNT: usize = 4;
const LATCH_PAD_COUNT: usize = 4;
const REPEATABILITY_POS: (f64, f64) = (-400.0, -45.0);

const WITNESS_SLOT_X: f64 = 520.0;
const WITNESS_SLOT_Y: f64 = 190.0;
const WITNESS_SLOT_Z: f64 = 30.0;
const LEAK_PATH_LANES: usize = 6;
const WITNESS_SLOT_COUNT: usize = LEAK_PATH_LANES * 2;
const SMOKE_PORT_COUNT: usize = LEAK_PATH_LANES;
const DYE_CUP_COUNT: usize = LEAK_PATH_LANES;
const WITNESS_POS: (f64, f64) = (-400.0, -230.0);

const MANIFOLD_X: f64 = 430.0;
const MANIFOLD_Y: f64 = 176.0;
const MANIFOLD_Z: f64 = 52.0;
const TAP_COUNT: usize = GASKET_COUPON_COUNT;
const TAP_ROWS: usize = 3;
const TAP_COLS: usize = 4;
const TAP_PITCH_X: f64 = 88.0;
const TAP_PITCH_Y: f64 = 42.0;
const TAP_BORE_D: f64 = 7.2;
const HEADER_BORE_D: f64 = 18.0;
const MANIFOLD_POS: (f64, f64) = (430.0, 45.0);

const GUTTER_X: f64 = 430.0;
const GUTTER_Y: f64 = 150.0;
const GUTTER_Z: f64 = 34.0;
const GUTTER_CHANNELS: usize = 5;
const GUTTER_CUP_COUNT: usize = 3;
const GUTTER_POS: (f64, f64) = (430.0, -150.0);

const TOKEN_RAIL_X: f64 = 430.0;
const TOKEN_RAIL_Y: f64 = 92.0;
const TOKEN_RAIL_Z: f64 = 22.0;
const DOOR_CYCLE_TOKEN_COUNT: usize = 10;
const TOKEN_PITCH_X: f64 = 38.0;
const TOKEN_D: f64 = 24.0;
const TOKEN_POS: (f64, f64) = (430.0, -283.0);

const TRACE_X: f64 = 520.0;
const TRACE_Y: f64 = 96.0;
const TRACE_Z: f64 = 10.0;
const BARCODE_LAND_COUNT: usize = GASKET_COUPON_COUNT + 4;
const STATUS_LANE_COUNT: usize = 3;
const CERTIFICATE_LAND_COUNT: usize = 4;
const TRACE_POS: (f64, f64) = (-400.0, -386.0);

const SEGREGATION_X: f64 = 430.0;
const SEGREGATION_Y: f64 = 104.0;
const SEGREGATION_Z: f64 = 54.0;
const CLEAN_BIN_COUNT: usize = 6;
const USED_BIN_COUNT: usize = 6;
const CLEAN_USED_MIN_GAP: f64 = 84.0;
const SEGREGATION_POS: (f64, f64) = (430.0, -382.0);

const EVIDENCE_X: f64 = 1180.0;
const EVIDENCE_Y: f64 = 66.0;
const EVIDENCE_BEAM_Z: f64 = 32.0;
const EVIDENCE_CLEARANCE_Z: f64 = 220.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 2;
const EVIDENCE_POS: (f64, f64) = (0.0, 392.0);

const KEEP_OUT_X: f64 = 1340.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 36.0;
const REAR_CAMERA_SERVICE_CLEARANCE: f64 = 44.0;
const SIDE_SERVICE_CLEARANCE: f64 = 70.0;
const DOOR_SWING_CLEARANCE_Z: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_tray(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let coupons = gasket_coupon_nest_bank();
    export(OUTPUTS[1], &coupons);

    let gauges = compression_height_witness_gauges();
    export(OUTPUTS[2], &gauges);

    let repeatability = hinge_latch_repeatability_pads();
    export(OUTPUTS[3], &repeatability);

    let witness_slots = smoke_dye_leak_path_witness_slots();
    export(OUTPUTS[4], &witness_slots);

    let manifold = pressure_decay_tap_manifold();
    export(OUTPUTS[5], &manifold);

    let gutter = condensate_gutter();
    export(OUTPUTS[6], &gutter);

    let tokens = door_cycle_token_rail();
    export(OUTPUTS[7], &tokens);

    let trace = barcode_status_lands();
    export(OUTPUTS[8], &trace);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let evidence = evidence_camera_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        base + coupons.translate(
            COUPON_BANK_POS.0,
            COUPON_BANK_POS.1,
            insert_z(COUPON_BANK_Z),
        ) + gauges.translate(
            GAUGE_PANEL_POS.0,
            GAUGE_PANEL_POS.1,
            insert_z(GAUGE_PANEL_Z),
        ) + repeatability.translate(
            REPEATABILITY_POS.0,
            REPEATABILITY_POS.1,
            insert_z(REPEATABILITY_Z),
        ) + witness_slots.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_SLOT_Z))
            + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, insert_z(MANIFOLD_Z))
            + gutter.translate(GUTTER_POS.0, GUTTER_POS.1, insert_z(GUTTER_Z))
            + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_RAIL_Z))
            + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
            + segregation.translate(
                SEGREGATION_POS.0,
                SEGREGATION_POS.1,
                insert_z(SEGREGATION_Z),
            )
            + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, BASE_Z)
            + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed airlock gasket compression leak witness station:");
    println!(
        "  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm containment tray with {RIM_Z:.0}mm rim"
    );
    println!(
        "  Coupon bank:           {COUPON_COLS}x{COUPON_ROWS} gasket coupon nests, {GASKET_COUPON_COUNT} compression frames"
    );
    println!(
        "  Compression witness:   {GAUGE_COUNT} gauge blocks with {GAUGE_STEP_COUNT} height steps each"
    );
    println!(
        "  Leak witnesses:        {WITNESS_SLOT_COUNT} smoke/dye witness slots, {SMOKE_PORT_COUNT} smoke ports, {DYE_CUP_COUNT} dye cups"
    );
    println!(
        "  Pressure decay:        {TAP_COUNT} tap placeholders on {TAP_ROWS}x{TAP_COLS} manifold with routed witness channels"
    );
    println!(
        "  Traceability:          {DOOR_CYCLE_TOKEN_COUNT} door-cycle tokens, {BARCODE_LAND_COUNT} barcode lands, {STATUS_LANE_COUNT} status lanes"
    );
    println!(
        "  Evidence/handling:     {CAMERA_COUNT} camera lands, {LIGHT_BAR_COUNT} light bars, {KEEP_OUT_GAUGE_COUNT} robot/service keepout gauges"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(GASKET_COUPON_COUNT, COUPON_COLS * COUPON_ROWS);
    assert_eq!(TAP_COUNT, TAP_ROWS * TAP_COLS);
    assert_eq!(WITNESS_SLOT_COUNT, LEAK_PATH_LANES * 2);
    assert_eq!(STATUS_LANE_COUNT, 3);
    assert_eq!(CLEAN_BIN_COUNT, USED_BIN_COUNT);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(gauge_height_range_mm() >= 2.0);
    assert!(condensate_capture_volume_ml() >= 140.0);
    assert!(clean_used_gap_mm() >= CLEAN_USED_MIN_GAP);
    assert!(front_robot_clearance_mm() >= FRONT_ROBOT_CLEARANCE);
    assert!(rear_camera_service_clearance_mm() >= REAR_CAMERA_SERVICE_CLEARANCE);
    assert!(side_service_clearance_mm() >= SIDE_SERVICE_CLEARANCE);
    assert!(DOOR_SWING_CLEARANCE_Z >= EVIDENCE_CLEARANCE_Z);

    let modules = module_rects();
    for module in modules {
        assert!(module.fits_inside_tray(), "{} exceeds tray", module.name);
    }

    for (left_index, left) in modules.iter().enumerate() {
        for right in modules.iter().skip(left_index + 1) {
            if left.name == "evidence camera bridge" || right.name == "evidence camera bridge" {
                continue;
            }
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn module_rects() -> [Rect; 10] {
    [
        Rect {
            name: "gasket coupon nest bank",
            center: COUPON_BANK_POS,
            x: COUPON_BANK_X,
            y: COUPON_BANK_Y,
        },
        Rect {
            name: "compression height witness gauges",
            center: GAUGE_PANEL_POS,
            x: GAUGE_PANEL_X,
            y: GAUGE_PANEL_Y,
        },
        Rect {
            name: "hinge latch repeatability pads",
            center: REPEATABILITY_POS,
            x: REPEATABILITY_X,
            y: REPEATABILITY_Y,
        },
        Rect {
            name: "smoke dye leak path witness slots",
            center: WITNESS_POS,
            x: WITNESS_SLOT_X,
            y: WITNESS_SLOT_Y,
        },
        Rect {
            name: "pressure decay tap manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Rect {
            name: "condensate gutter",
            center: GUTTER_POS,
            x: GUTTER_X,
            y: GUTTER_Y,
        },
        Rect {
            name: "door cycle token rail",
            center: TOKEN_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Rect {
            name: "barcode status lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "clean used segregation",
            center: SEGREGATION_POS,
            x: SEGREGATION_X,
            y: SEGREGATION_Y,
        },
        Rect {
            name: "evidence camera bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

fn gauge_height_range_mm() -> f64 {
    (GAUGE_STEP_COUNT as f64 - 1.0) * GAUGE_STEP_DELTA
}

fn condensate_capture_volume_ml() -> f64 {
    let trough_x = GUTTER_X - 78.0;
    let trough_y = 36.0;
    let trough_depth = 18.0;
    trough_x * trough_y * trough_depth / 1000.0
}

fn clean_used_gap_mm() -> f64 {
    150.0
}

fn front_robot_clearance_mm() -> f64 {
    STATION_Y / 2.0 - (SEGREGATION_POS.1.abs() + SEGREGATION_Y / 2.0)
}

fn rear_camera_service_clearance_mm() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_Y / 2.0)
}

fn side_service_clearance_mm() -> f64 {
    STATION_X / 2.0 - (GAUGE_PANEL_POS.0 + GAUGE_PANEL_X / 2.0)
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "airlock_gasket_compression_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "airlock_gasket_compression_secondary_basin_recess",
        STATION_X - 132.0,
        STATION_Y - 124.0,
        BASIN_RECESS_Z + 1.0,
    )
    .translate(0.0, -16.0, BASE_Z / 2.0 - BASIN_RECESS_Z / 2.0 + 0.5);

    deck - basin - mounting_slots() - module_locator_sockets()
        + tray_rims()
        + datum_targets()
        + zone_dividers()
        + containment_drain_bores()
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airlock_gasket_compression_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let round = centered_cylinder(
            format!("airlock_gasket_compression_m6_mount_round_{i}"),
            7.2 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(x, y, 0.0);
        let slot = centered_cube(
            format!("airlock_gasket_compression_m6_mount_slot_{i}"),
            30.0,
            7.2,
            BASE_Z + 4.0,
        )
        .translate(x, y, 0.0);
        slots = slots + round + slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-650.0, -410.0),
        (-325.0, -410.0),
        (0.0, -410.0),
        (325.0, -410.0),
        (650.0, -410.0),
        (-650.0, 410.0),
        (-325.0, 410.0),
        (0.0, 410.0),
        (325.0, 410.0),
        (650.0, 410.0),
    ]
}

fn module_locator_sockets() -> Part {
    let mut sockets = Part::empty("airlock_gasket_compression_module_locator_sockets");
    for module in module_rects()
        .into_iter()
        .filter(|rect| rect.name != "evidence camera bridge")
    {
        sockets = sockets
            + centered_cube(
                format!(
                    "airlock_gasket_compression_{}_locator_socket",
                    module.name.replace(' ', "_")
                ),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "airlock_gasket_compression_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "airlock_gasket_compression_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "airlock_gasket_compression_rear_camera_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "airlock_gasket_compression_front_robot_low_lip",
        STATION_X - 180.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn datum_targets() -> Part {
    let mut datums = Part::empty("airlock_gasket_compression_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("airlock_gasket_compression_datum_ring_outer_{i}"),
                15.0,
                3.0,
                36,
            )
            .translate(x, y, BASE_Z / 2.0 + 1.5)
            - centered_cylinder(
                format!("airlock_gasket_compression_datum_ring_inner_{i}"),
                6.0,
                3.6,
                24,
            )
            .translate(x, y, BASE_Z / 2.0 + 1.8);
    }
    datums
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-610.0, 350.0),
        (610.0, 350.0),
        (-610.0, -350.0),
        (610.0, -350.0),
    ]
}

fn zone_dividers() -> Part {
    let vertical = centered_cube(
        "airlock_gasket_compression_clean_dirty_center_divider",
        10.0,
        STATION_Y - 170.0,
        26.0,
    )
    .translate(52.0, -32.0, BASE_Z / 2.0 + 13.0);
    let witness_row = centered_cube(
        "airlock_gasket_compression_witness_row_divider",
        STATION_X - 190.0,
        10.0,
        24.0,
    )
    .translate(0.0, -112.0, BASE_Z / 2.0 + 12.0);
    let trace_row = centered_cube(
        "airlock_gasket_compression_traceability_row_divider",
        STATION_X - 220.0,
        8.0,
        20.0,
    )
    .translate(0.0, -352.0, BASE_Z / 2.0 + 10.0);

    vertical + witness_row + trace_row
}

fn containment_drain_bores() -> Part {
    let left = centered_cylinder(
        "airlock_gasket_compression_left_containment_drain_bore",
        7.0,
        58.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-620.0, -STATION_Y / 2.0 - 1.0, 0.0);
    let right = centered_cylinder(
        "airlock_gasket_compression_right_containment_drain_bore",
        7.0,
        58.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(620.0, -STATION_Y / 2.0 - 1.0, 0.0);

    left + right
}

fn gasket_coupon_nest_bank() -> Part {
    let plate = centered_cube(
        "airlock_gasket_coupon_nest_bank_plate",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    );
    plate - coupon_nest_reliefs()
        + gasket_coupon_frames()
        + coupon_index_tabs()
        + coupon_edge_compression_lips()
}

fn coupon_nest_reliefs() -> Part {
    let mut reliefs = Part::empty("airlock_gasket_coupon_nest_reliefs");
    for index in 0..GASKET_COUPON_COUNT {
        let (x, y) = coupon_center(index);
        reliefs = reliefs
            + centered_cube(
                format!("airlock_gasket_coupon_nest_relief_{index}"),
                COUPON_NEST_X,
                COUPON_NEST_Y,
                COUPON_NEST_DEPTH + 1.0,
            )
            .translate(x, y, COUPON_BANK_Z / 2.0 - COUPON_NEST_DEPTH / 2.0 + 0.5);
    }
    reliefs
}

fn gasket_coupon_frames() -> Part {
    let mut frames = Part::empty("airlock_gasket_coupon_witness_frames");
    for index in 0..GASKET_COUPON_COUNT {
        let (x, y) = coupon_center(index);
        let outer = centered_cube(
            format!("airlock_gasket_coupon_frame_outer_{index}"),
            COUPON_NEST_X + GASKET_FRAME_W * 2.0,
            COUPON_NEST_Y + GASKET_FRAME_W * 2.0,
            6.0,
        );
        let inner = centered_cube(
            format!("airlock_gasket_coupon_frame_inner_relief_{index}"),
            COUPON_NEST_X - 16.0,
            COUPON_NEST_Y - 14.0,
            7.0,
        );
        frames = frames + (outer - inner).translate(x, y, COUPON_BANK_Z / 2.0 + 3.0);
    }
    frames
}

fn coupon_index_tabs() -> Part {
    let mut tabs = Part::empty("airlock_gasket_coupon_index_tabs");
    for index in 0..GASKET_COUPON_COUNT {
        let (x, y) = coupon_center(index);
        tabs = tabs
            + centered_cube(
                format!("airlock_gasket_coupon_index_tab_{index}"),
                26.0,
                8.0,
                8.0,
            )
            .translate(
                x - COUPON_NEST_X / 2.0 + 18.0,
                y + COUPON_NEST_Y / 2.0 + 11.0,
                COUPON_BANK_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn coupon_edge_compression_lips() -> Part {
    let upper = centered_cube(
        "airlock_gasket_coupon_upper_edge_compression_lip",
        COUPON_BANK_X - 50.0,
        8.0,
        16.0,
    )
    .translate(0.0, COUPON_BANK_Y / 2.0 - 28.0, COUPON_BANK_Z / 2.0 + 8.0);
    let lower = centered_cube(
        "airlock_gasket_coupon_lower_edge_compression_lip",
        COUPON_BANK_X - 50.0,
        8.0,
        16.0,
    )
    .translate(0.0, -COUPON_BANK_Y / 2.0 + 28.0, COUPON_BANK_Z / 2.0 + 8.0);

    upper + lower
}

fn coupon_center(index: usize) -> (f64, f64) {
    let col = index % COUPON_COLS;
    let row = index / COUPON_COLS;
    (
        centered_index(col, COUPON_COLS, COUPON_PITCH_X),
        centered_index(row, COUPON_ROWS, COUPON_PITCH_Y),
    )
}

fn compression_height_witness_gauges() -> Part {
    let panel = centered_cube(
        "airlock_gasket_compression_height_gauge_panel",
        GAUGE_PANEL_X,
        GAUGE_PANEL_Y,
        GAUGE_PANEL_Z,
    );
    panel - gauge_storage_pockets() + gauge_step_blocks() + gauge_reference_rails()
}

fn gauge_storage_pockets() -> Part {
    let mut pockets = Part::empty("airlock_gasket_compression_gauge_storage_pockets");
    for i in 0..GAUGE_COUNT {
        pockets = pockets
            + centered_cube(
                format!("airlock_gasket_compression_gauge_storage_pocket_{i}"),
                34.0,
                156.0,
                12.0,
            )
            .translate(
                centered_index(i, GAUGE_COUNT, GAUGE_PITCH_X),
                0.0,
                GAUGE_PANEL_Z / 2.0 - 5.5,
            );
    }
    pockets
}

fn gauge_step_blocks() -> Part {
    let mut steps = Part::empty("airlock_gasket_compression_height_witness_steps");
    for gauge in 0..GAUGE_COUNT {
        let x = centered_index(gauge, GAUGE_COUNT, GAUGE_PITCH_X);
        for step in 0..GAUGE_STEP_COUNT {
            let height = GAUGE_MIN_HEIGHT + step as f64 * GAUGE_STEP_DELTA;
            steps = steps
                + centered_cube(
                    format!("airlock_gasket_compression_gauge_{gauge}_step_{step}"),
                    28.0,
                    18.0,
                    height,
                )
                .translate(
                    x,
                    centered_index(step, GAUGE_STEP_COUNT, GAUGE_STEP_PITCH_Y),
                    GAUGE_PANEL_Z / 2.0 + height / 2.0,
                );
        }
    }
    steps
}

fn gauge_reference_rails() -> Part {
    let top = centered_cube(
        "airlock_gasket_compression_gauge_top_reference_rail",
        GAUGE_PANEL_X - 44.0,
        7.0,
        12.0,
    )
    .translate(0.0, GAUGE_PANEL_Y / 2.0 - 24.0, GAUGE_PANEL_Z / 2.0 + 6.0);
    let bottom = centered_cube(
        "airlock_gasket_compression_gauge_bottom_reference_rail",
        GAUGE_PANEL_X - 44.0,
        7.0,
        12.0,
    )
    .translate(0.0, -GAUGE_PANEL_Y / 2.0 + 24.0, GAUGE_PANEL_Z / 2.0 + 6.0);

    top + bottom
}

fn hinge_latch_repeatability_pads() -> Part {
    let plate = centered_cube(
        "airlock_gasket_hinge_latch_repeatability_plate",
        REPEATABILITY_X,
        REPEATABILITY_Y,
        REPEATABILITY_Z,
    );
    plate - repeatability_pad_reliefs()
        + hinge_repeatability_pad_targets()
        + latch_repeatability_pad_targets()
        + closing_force_witness_ribs()
}

fn repeatability_pad_reliefs() -> Part {
    let mut reliefs = Part::empty("airlock_gasket_repeatability_pad_reliefs");
    for i in 0..(HINGE_PAD_COUNT + LATCH_PAD_COUNT) {
        let (x, y) = repeatability_pad_center(i);
        reliefs = reliefs
            + centered_cube(
                format!("airlock_gasket_repeatability_pad_relief_{i}"),
                62.0,
                34.0,
                8.0,
            )
            .translate(x, y, REPEATABILITY_Z / 2.0 - 3.5);
    }
    reliefs
}

fn hinge_repeatability_pad_targets() -> Part {
    let mut pads = Part::empty("airlock_gasket_hinge_repeatability_pad_targets");
    for i in 0..HINGE_PAD_COUNT {
        let (x, y) = repeatability_pad_center(i);
        pads = pads
            + centered_cube(
                format!("airlock_gasket_hinge_repeatability_pad_{i}"),
                54.0,
                26.0,
                8.0,
            )
            .translate(x, y, REPEATABILITY_Z / 2.0 + 4.0)
            + centered_cylinder(
                format!("airlock_gasket_hinge_pad_datum_disc_{i}"),
                8.0,
                5.0,
                24,
            )
            .translate(x - 16.0, y, REPEATABILITY_Z / 2.0 + 10.5);
    }
    pads
}

fn latch_repeatability_pad_targets() -> Part {
    let mut pads = Part::empty("airlock_gasket_latch_repeatability_pad_targets");
    for i in 0..LATCH_PAD_COUNT {
        let index = i + HINGE_PAD_COUNT;
        let (x, y) = repeatability_pad_center(index);
        pads = pads
            + centered_cube(
                format!("airlock_gasket_latch_repeatability_pad_{i}"),
                54.0,
                26.0,
                8.0,
            )
            .translate(x, y, REPEATABILITY_Z / 2.0 + 4.0)
            + centered_cube(
                format!("airlock_gasket_latch_pad_strike_line_{i}"),
                5.0,
                34.0,
                6.0,
            )
            .translate(x + 16.0, y, REPEATABILITY_Z / 2.0 + 11.0);
    }
    pads
}

fn repeatability_pad_center(index: usize) -> (f64, f64) {
    let col = index % 4;
    let row = index / 4;
    (centered_index(col, 4, 118.0), centered_index(row, 2, 74.0))
}

fn closing_force_witness_ribs() -> Part {
    let mut ribs = Part::empty("airlock_gasket_closing_force_witness_ribs");
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("airlock_gasket_closing_force_witness_rib_{i}"),
                14.0,
                REPEATABILITY_Y - 34.0,
                9.0,
            )
            .translate(centered_index(i, 5, 92.0), 0.0, REPEATABILITY_Z / 2.0 + 4.5);
    }
    ribs
}

fn smoke_dye_leak_path_witness_slots() -> Part {
    let plate = centered_cube(
        "airlock_gasket_smoke_dye_witness_slot_plate",
        WITNESS_SLOT_X,
        WITNESS_SLOT_Y,
        WITNESS_SLOT_Z,
    );
    plate - leak_path_slot_reliefs() - smoke_port_bores() - dye_cup_reliefs()
        + witness_slot_label_lands()
        + leak_path_divider_ribs()
}

fn leak_path_slot_reliefs() -> Part {
    let mut slots = Part::empty("airlock_gasket_leak_path_slot_reliefs");
    for lane in 0..LEAK_PATH_LANES {
        let y = centered_index(lane, LEAK_PATH_LANES, 27.0);
        slots = slots
            + centered_cube(
                format!("airlock_gasket_smoke_witness_slot_{lane}"),
                178.0,
                7.0,
                WITNESS_SLOT_Z + 2.0,
            )
            .translate(-112.0, y, 0.0)
            + centered_cube(
                format!("airlock_gasket_dye_witness_slot_{lane}"),
                178.0,
                7.0,
                WITNESS_SLOT_Z + 2.0,
            )
            .translate(112.0, y, 0.0);
    }
    slots
}

fn smoke_port_bores() -> Part {
    let mut ports = Part::empty("airlock_gasket_smoke_port_bores");
    for lane in 0..SMOKE_PORT_COUNT {
        ports = ports
            + centered_cylinder(
                format!("airlock_gasket_smoke_port_bore_{lane}"),
                5.0,
                WITNESS_SLOT_Z + 4.0,
                20,
            )
            .translate(-230.0, centered_index(lane, SMOKE_PORT_COUNT, 27.0), 0.0);
    }
    ports
}

fn dye_cup_reliefs() -> Part {
    let mut cups = Part::empty("airlock_gasket_dye_cup_reliefs");
    for lane in 0..DYE_CUP_COUNT {
        cups = cups
            + centered_cylinder(
                format!("airlock_gasket_dye_cup_relief_{lane}"),
                12.0,
                12.0,
                28,
            )
            .translate(
                232.0,
                centered_index(lane, DYE_CUP_COUNT, 27.0),
                WITNESS_SLOT_Z / 2.0 - 5.5,
            );
    }
    cups
}

fn witness_slot_label_lands() -> Part {
    let smoke = centered_cube("airlock_gasket_smoke_witness_label_land", 190.0, 18.0, 4.0)
        .translate(
            -112.0,
            WITNESS_SLOT_Y / 2.0 - 20.0,
            WITNESS_SLOT_Z / 2.0 + 2.0,
        );
    let dye = centered_cube("airlock_gasket_dye_witness_label_land", 190.0, 18.0, 4.0).translate(
        112.0,
        WITNESS_SLOT_Y / 2.0 - 20.0,
        WITNESS_SLOT_Z / 2.0 + 2.0,
    );

    smoke + dye
}

fn leak_path_divider_ribs() -> Part {
    let mut ribs = Part::empty("airlock_gasket_leak_path_divider_ribs");
    for lane in 0..(LEAK_PATH_LANES + 1) {
        ribs = ribs
            + centered_cube(
                format!("airlock_gasket_leak_path_divider_rib_{lane}"),
                WITNESS_SLOT_X - 42.0,
                4.0,
                7.0,
            )
            .translate(
                0.0,
                -((LEAK_PATH_LANES as f64) * 27.0) / 2.0 + lane as f64 * 27.0,
                WITNESS_SLOT_Z / 2.0 + 3.5,
            );
    }
    ribs
}

fn pressure_decay_tap_manifold() -> Part {
    let body = centered_cube(
        "airlock_gasket_pressure_decay_tap_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    body - pressure_tap_bores() - pressure_header_bore()
        + tap_pad_lands()
        + route_witness_grooves()
        + reference_volume_placeholders()
}

fn pressure_tap_bores() -> Part {
    let mut taps = Part::empty("airlock_gasket_pressure_decay_tap_bores");
    for tap in 0..TAP_COUNT {
        let (x, y) = tap_center(tap);
        taps = taps
            + centered_cylinder(
                format!("airlock_gasket_pressure_decay_tap_bore_{tap}"),
                TAP_BORE_D / 2.0,
                MANIFOLD_Z + 4.0,
                20,
            )
            .translate(x, y, 0.0);
    }
    taps
}

fn pressure_header_bore() -> Part {
    centered_cylinder(
        "airlock_gasket_pressure_decay_header_bore",
        HEADER_BORE_D / 2.0,
        MANIFOLD_X + 12.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -MANIFOLD_Y / 2.0 + 26.0, 0.0)
}

fn tap_pad_lands() -> Part {
    let mut pads = Part::empty("airlock_gasket_pressure_decay_tap_pad_lands");
    for tap in 0..TAP_COUNT {
        let (x, y) = tap_center(tap);
        pads = pads
            + centered_cylinder(
                format!("airlock_gasket_pressure_decay_tap_pad_{tap}"),
                14.0,
                6.0,
                24,
            )
            .translate(x, y, MANIFOLD_Z / 2.0 + 3.0);
    }
    pads
}

fn route_witness_grooves() -> Part {
    let mut grooves = Part::empty("airlock_gasket_pressure_route_witness_grooves");
    for row in 0..TAP_ROWS {
        grooves = grooves
            + centered_cube(
                format!("airlock_gasket_pressure_route_witness_groove_row_{row}"),
                MANIFOLD_X - 74.0,
                5.0,
                5.0,
            )
            .translate(
                0.0,
                centered_index(row, TAP_ROWS, TAP_PITCH_Y),
                MANIFOLD_Z / 2.0 + 2.5,
            );
    }
    grooves
}

fn reference_volume_placeholders() -> Part {
    let mut volumes = Part::empty("airlock_gasket_pressure_reference_volume_placeholders");
    for col in 0..TAP_COLS {
        volumes = volumes
            + centered_cube(
                format!("airlock_gasket_pressure_reference_volume_block_{col}"),
                58.0,
                24.0,
                24.0,
            )
            .translate(
                centered_index(col, TAP_COLS, TAP_PITCH_X),
                MANIFOLD_Y / 2.0 - 28.0,
                MANIFOLD_Z / 2.0 + 12.0,
            );
    }
    volumes
}

fn tap_center(index: usize) -> (f64, f64) {
    let col = index % TAP_COLS;
    let row = index / TAP_COLS;
    (
        centered_index(col, TAP_COLS, TAP_PITCH_X),
        centered_index(row, TAP_ROWS, TAP_PITCH_Y) - 18.0,
    )
}

fn condensate_gutter() -> Part {
    let body = centered_cube(
        "airlock_gasket_condensate_gutter_body",
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    );
    body - gutter_trough_relief() - gutter_cup_reliefs() + gutter_lane_ribs() + drip_witness_posts()
}

fn gutter_trough_relief() -> Part {
    centered_cube(
        "airlock_gasket_condensate_gutter_trough_relief",
        GUTTER_X - 76.0,
        38.0,
        19.0,
    )
    .translate(0.0, -12.0, GUTTER_Z / 2.0 - 8.5)
}

fn gutter_cup_reliefs() -> Part {
    let mut cups = Part::empty("airlock_gasket_condensate_gutter_cup_reliefs");
    for i in 0..GUTTER_CUP_COUNT {
        cups = cups
            + centered_cylinder(
                format!("airlock_gasket_condensate_collection_cup_relief_{i}"),
                22.0,
                20.0,
                32,
            )
            .translate(
                centered_index(i, GUTTER_CUP_COUNT, 114.0),
                GUTTER_Y / 2.0 - 42.0,
                GUTTER_Z / 2.0 - 9.0,
            );
    }
    cups
}

fn gutter_lane_ribs() -> Part {
    let mut ribs = Part::empty("airlock_gasket_condensate_gutter_lane_ribs");
    for i in 0..GUTTER_CHANNELS {
        ribs = ribs
            + centered_cube(
                format!("airlock_gasket_condensate_gutter_lane_rib_{i}"),
                6.0,
                GUTTER_Y - 28.0,
                10.0,
            )
            .translate(
                centered_index(i, GUTTER_CHANNELS, 72.0),
                0.0,
                GUTTER_Z / 2.0 + 5.0,
            );
    }
    ribs
}

fn drip_witness_posts() -> Part {
    let mut posts = Part::empty("airlock_gasket_condensate_drip_witness_posts");
    for i in 0..GUTTER_CHANNELS {
        posts = posts
            + centered_cylinder(
                format!("airlock_gasket_condensate_drip_witness_post_{i}"),
                5.0,
                18.0,
                18,
            )
            .translate(
                centered_index(i, GUTTER_CHANNELS, 72.0),
                -GUTTER_Y / 2.0 + 28.0,
                GUTTER_Z / 2.0 + 9.0,
            );
    }
    posts
}

fn door_cycle_token_rail() -> Part {
    let rail = centered_cube(
        "airlock_gasket_door_cycle_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    rail - token_socket_reliefs() + cycle_token_posts() + token_stop_tabs()
}

fn token_socket_reliefs() -> Part {
    let mut sockets = Part::empty("airlock_gasket_door_cycle_token_socket_reliefs");
    for i in 0..DOOR_CYCLE_TOKEN_COUNT {
        sockets = sockets
            + centered_cylinder(
                format!("airlock_gasket_door_cycle_token_socket_{i}"),
                TOKEN_D / 2.0,
                9.0,
                28,
            )
            .translate(
                centered_index(i, DOOR_CYCLE_TOKEN_COUNT, TOKEN_PITCH_X),
                0.0,
                TOKEN_RAIL_Z / 2.0 - 4.0,
            );
    }
    sockets
}

fn cycle_token_posts() -> Part {
    let mut posts = Part::empty("airlock_gasket_door_cycle_token_posts");
    for i in 0..DOOR_CYCLE_TOKEN_COUNT {
        posts = posts
            + centered_cylinder(
                format!("airlock_gasket_door_cycle_token_post_{i}"),
                6.0,
                8.0,
                18,
            )
            .translate(
                centered_index(i, DOOR_CYCLE_TOKEN_COUNT, TOKEN_PITCH_X),
                0.0,
                TOKEN_RAIL_Z / 2.0 + 4.0,
            );
    }
    posts
}

fn token_stop_tabs() -> Part {
    let left = centered_cube(
        "airlock_gasket_door_cycle_token_left_stop_tab",
        10.0,
        TOKEN_RAIL_Y - 20.0,
        18.0,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 24.0, 0.0, TOKEN_RAIL_Z / 2.0 + 9.0);
    let right = centered_cube(
        "airlock_gasket_door_cycle_token_right_stop_tab",
        10.0,
        TOKEN_RAIL_Y - 20.0,
        18.0,
    )
    .translate(TOKEN_RAIL_X / 2.0 - 24.0, 0.0, TOKEN_RAIL_Z / 2.0 + 9.0);

    left + right
}

fn barcode_status_lands() -> Part {
    let panel = centered_cube(
        "airlock_gasket_barcode_status_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    panel + barcode_lands() + status_lanes() + certificate_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("airlock_gasket_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 8;
        let col = i % 8;
        lands = lands
            + centered_cube(format!("airlock_gasket_barcode_land_{i}"), 48.0, 18.0, 3.0).translate(
                centered_index(col, 8, 58.0),
                14.0 + centered_index(row, 2, 28.0),
                TRACE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn status_lanes() -> Part {
    let mut lanes = Part::empty("airlock_gasket_status_lanes");
    for (i, name) in ["release", "hold", "reject"].into_iter().enumerate() {
        lanes = lanes
            + centered_cube(
                format!("airlock_gasket_{name}_status_lane"),
                130.0,
                16.0,
                5.0,
            )
            .translate(
                -TRACE_X / 2.0 + 86.0 + i as f64 * 150.0,
                -TRACE_Y / 2.0 + 18.0,
                TRACE_Z / 2.0 + 2.5,
            );
    }
    lanes
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("airlock_gasket_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("airlock_gasket_certificate_land_{i}"),
                72.0,
                20.0,
                3.0,
            )
            .translate(
                TRACE_X / 2.0 - 46.0,
                centered_index(i, CERTIFICATE_LAND_COUNT, 22.0),
                TRACE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn clean_used_segregation() -> Part {
    let base = centered_cube(
        "airlock_gasket_clean_used_segregation_base",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    base - clean_used_bin_reliefs()
        + clean_used_center_wall()
        + clean_coupon_posts()
        + used_coupon_cups()
}

fn clean_used_bin_reliefs() -> Part {
    let mut reliefs = Part::empty("airlock_gasket_clean_used_bin_reliefs");
    for i in 0..(CLEAN_BIN_COUNT + USED_BIN_COUNT) {
        let clean_side = i < CLEAN_BIN_COUNT;
        let local = if clean_side { i } else { i - CLEAN_BIN_COUNT };
        let x = if clean_side { -108.0 } else { 108.0 } + centered_index(local % 3, 3, 48.0);
        let y = centered_index(local / 3, 2, 42.0);
        reliefs = reliefs
            + centered_cube(
                format!("airlock_gasket_clean_used_bin_relief_{i}"),
                34.0,
                30.0,
                18.0,
            )
            .translate(x, y, SEGREGATION_Z / 2.0 - 8.0);
    }
    reliefs
}

fn clean_used_center_wall() -> Part {
    centered_cube(
        "airlock_gasket_clean_used_segregation_center_wall",
        12.0,
        SEGREGATION_Y,
        SEGREGATION_Z + 34.0,
    )
    .translate(0.0, 0.0, 17.0)
}

fn clean_coupon_posts() -> Part {
    let mut posts = Part::empty("airlock_gasket_clean_coupon_posts");
    for i in 0..CLEAN_BIN_COUNT {
        posts = posts
            + centered_cylinder(
                format!("airlock_gasket_clean_coupon_post_{i}"),
                5.0,
                18.0,
                18,
            )
            .translate(
                -108.0 + centered_index(i % 3, 3, 48.0),
                centered_index(i / 3, 2, 42.0),
                SEGREGATION_Z / 2.0 + 9.0,
            );
    }
    posts
}

fn used_coupon_cups() -> Part {
    let mut cups = Part::empty("airlock_gasket_used_coupon_cups");
    for i in 0..USED_BIN_COUNT {
        cups = cups
            + centered_cylinder(
                format!("airlock_gasket_used_coupon_cup_lip_{i}"),
                15.0,
                8.0,
                24,
            )
            .translate(
                108.0 + centered_index(i % 3, 3, 48.0),
                centered_index(i / 3, 2, 42.0),
                SEGREGATION_Z / 2.0 + 4.0,
            );
    }
    cups
}

fn evidence_camera_bridge() -> Part {
    let beam = centered_cube(
        "airlock_gasket_evidence_camera_bridge_beam",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z);
    let left_upright = centered_cube(
        "airlock_gasket_evidence_camera_bridge_left_upright",
        34.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 38.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let right_upright = centered_cube(
        "airlock_gasket_evidence_camera_bridge_right_upright",
        34.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 38.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);

    beam + left_upright + right_upright + camera_mount_lands() + light_bar_lands()
}

fn camera_mount_lands() -> Part {
    let mut mounts = Part::empty("airlock_gasket_evidence_camera_mount_lands");
    for i in 0..CAMERA_COUNT {
        mounts = mounts
            + centered_cube(
                format!("airlock_gasket_evidence_camera_mount_land_{i}"),
                86.0,
                28.0,
                8.0,
            )
            .translate(
                centered_index(i, CAMERA_COUNT, 310.0),
                0.0,
                EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0 + 4.0,
            )
            + centered_cylinder(
                format!("airlock_gasket_evidence_camera_lens_clearance_{i}"),
                14.0,
                EVIDENCE_BEAM_Z + 10.0,
                28,
            )
            .translate(
                centered_index(i, CAMERA_COUNT, 310.0),
                0.0,
                EVIDENCE_CLEARANCE_Z,
            );
    }
    mounts
}

fn light_bar_lands() -> Part {
    let mut bars = Part::empty("airlock_gasket_evidence_light_bar_lands");
    for i in 0..LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("airlock_gasket_evidence_light_bar_land_{i}"),
                EVIDENCE_X - 180.0,
                10.0,
                7.0,
            )
            .translate(
                0.0,
                centered_index(i, LIGHT_BAR_COUNT, 36.0),
                EVIDENCE_CLEARANCE_Z - 20.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let footprint = centered_cube(
        "airlock_gasket_robot_service_keepout_footprint",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    footprint + keepout_gauge_posts() + door_swing_clearance_arch()
}

fn keepout_gauge_posts() -> Part {
    let mut gauges = Part::empty("airlock_gasket_robot_service_keepout_gauges");
    let positions = [
        (-KEEP_OUT_X / 2.0 + 34.0, -KEEP_OUT_Y / 2.0 + 34.0),
        (0.0, -KEEP_OUT_Y / 2.0 + 34.0),
        (KEEP_OUT_X / 2.0 - 34.0, -KEEP_OUT_Y / 2.0 + 34.0),
        (-KEEP_OUT_X / 2.0 + 34.0, KEEP_OUT_Y / 2.0 - 34.0),
        (0.0, KEEP_OUT_Y / 2.0 - 34.0),
        (KEEP_OUT_X / 2.0 - 34.0, KEEP_OUT_Y / 2.0 - 34.0),
    ];
    for (i, (x, y)) in positions.into_iter().enumerate() {
        gauges = gauges
            + centered_cube(
                format!("airlock_gasket_keepout_gauge_post_{i}"),
                22.0,
                22.0,
                56.0,
            )
            .translate(x, y, KEEP_OUT_Z / 2.0 + 28.0);
    }
    gauges
}

fn door_swing_clearance_arch() -> Part {
    let arc = centered_cylinder(
        "airlock_gasket_door_swing_clearance_reference_arc",
        250.0,
        5.0,
        64,
    )
    .translate(-170.0, 0.0, KEEP_OUT_Z / 2.0 + 2.5);
    let inner = centered_cylinder(
        "airlock_gasket_door_swing_clearance_reference_arc_inner",
        224.0,
        6.0,
        64,
    )
    .translate(-170.0, 0.0, KEEP_OUT_Z / 2.0 + 3.0);
    arc - inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_complete_and_scoped() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn required_feature_coverage_matches_design_intent() {
        for feature in [
            "gasket_coupon_nest_bank",
            "compression_height_witness_gauges",
            "hinge_latch_repeatability_pads",
            "smoke_dye_leak_path_witness_slots",
            "pressure_decay_tap_manifold",
            "condensate_gutter",
            "door_cycle_token_rail",
            "barcode_status_lands",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepouts",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn feature_counts_are_deterministic() {
        assert_eq!(GASKET_COUPON_COUNT, 12);
        assert_eq!(TAP_COUNT, GASKET_COUPON_COUNT);
        assert_eq!(WITNESS_SLOT_COUNT, 12);
        assert_eq!(DOOR_CYCLE_TOKEN_COUNT, 10);
        assert_eq!(BARCODE_LAND_COUNT, 16);
        assert_eq!(CLEAN_BIN_COUNT + USED_BIN_COUNT, GASKET_COUPON_COUNT);
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 6);
    }

    #[test]
    fn modules_fit_inside_bounds_without_overlap() {
        let modules = module_rects();
        for module in modules {
            assert!(
                module.fits_inside_tray(),
                "{} exceeds station bounds",
                module.name
            );
        }
        for (i, left) in modules.iter().enumerate() {
            for right in modules.iter().skip(i + 1) {
                if left.name == "evidence camera bridge" || right.name == "evidence camera bridge" {
                    continue;
                }
                assert!(
                    !left.overlaps(*right),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn witness_geometry_has_clearances_and_capacity() {
        assert!(gauge_height_range_mm() >= 2.0);
        assert!(condensate_capture_volume_ml() >= 140.0);
        assert!(clean_used_gap_mm() >= CLEAN_USED_MIN_GAP);
        assert!(front_robot_clearance_mm() >= FRONT_ROBOT_CLEARANCE);
        assert!(rear_camera_service_clearance_mm() >= REAR_CAMERA_SERVICE_CLEARANCE);
        assert!(side_service_clearance_mm() >= SIDE_SERVICE_CLEARANCE);
    }
}
