use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette seed distribution fluorescent uniformity station.
//
// No-cell validation fixture for challenging a sealed 16/20-slot cassette with
// fluorescent beads or dye before live cells are introduced. Geometry covers a
// cassette datum nest, row/column inlet surrogate ports, witness coupon and
// slide slots, timed dwell token lands, wash/recovery wells, imaging fiducials,
// edge/center position labels, wetness/bubble witness pockets, leak tray,
// barcode/status lanes, and robot/service keepout gauges.

const OUTPUTS: [&str; 13] = [
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_leak_tray_deck.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_cassette_datum_nest.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_row_column_inlet_surrogate_ports.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_fluorescent_witness_coupon_slide_slots.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_timed_dwell_token_lands.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_wash_recovery_wells.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_imaging_fiducial_frame.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_edge_center_position_labels.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_bubble_wetness_witness_pockets.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_barcode_status_lanes.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_robot_service_keepouts.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_reference_controls.stl",
    "output/closed_cassette_seed_distribution_fluorescent_uniformity_station_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITIONS: usize = CASSETTE_COLS * CASSETTE_ROWS;
const ROW_PORTS: usize = CASSETTE_ROWS;
const COLUMN_PORTS: usize = CASSETTE_COLS;
const INLET_PORTS: usize = ROW_PORTS + COLUMN_PORTS;
const EDGE_POSITIONS: usize = 14;
const CENTER_POSITIONS: usize = CASSETTE_POSITIONS - EDGE_POSITIONS;
const WITNESS_COUPONS: usize = 10;
const SLIDE_SLOTS: usize = 4;
const DWELL_TOKEN_COUNT: usize = 8;
const WASH_WELLS: usize = 4;
const RECOVERY_WELLS: usize = 4;
const BUBBLE_WITNESS_COUNT: usize = CASSETTE_POSITIONS;
const STATUS_LANES: usize = 3;
const CONTROL_COUPONS: usize = 6;

const CHIP_GAP_X: f64 = 6.0;
const CHIP_GAP_Y: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const CASSETTE_X: f64 = CASSETTE_COLS as f64 * REVC_CHIP_LENGTH
    + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP_X
    + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH
    + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP_Y
    + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = 38.0;

const DECK_X: f64 = 1480.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const LEAK_TRAY_RIM_W: f64 = 20.0;
const LEAK_TRAY_RIM_Z: f64 = 34.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;

const NEST_CENTER: (f64, f64) = (-188.0, 72.0);
const NEST_X: f64 = CASSETTE_X + 48.0;
const NEST_Y: f64 = CASSETTE_Y + 46.0;
const NEST_Z: f64 = 34.0;
const NEST_SOCKET_DEPTH: f64 = 9.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 27.0;
const DATUM_PIN_D: f64 = 7.0;
const CHIP_RELIEF_Z: f64 = 8.0;

const PORT_MANIFOLD_CENTER: (f64, f64) = (402.0, 242.0);
const PORT_MANIFOLD_X: f64 = 464.0;
const PORT_MANIFOLD_Y: f64 = 250.0;
const PORT_MANIFOLD_Z: f64 = 32.0;
const PORT_BOSS_D: f64 = 28.0;
const PORT_BORE_D: f64 = 12.0;
const PORT_PITCH_X: f64 = 74.0;
const PORT_PITCH_Y: f64 = 54.0;
const PORT_CHANNEL_W: f64 = 10.0;
const PORT_LABEL_LAND_X: f64 = 42.0;
const PORT_LABEL_LAND_Y: f64 = 16.0;

const WITNESS_CENTER: (f64, f64) = (-520.0, -196.0);
const WITNESS_BANK_X: f64 = 320.0;
const WITNESS_BANK_Y: f64 = 294.0;
const WITNESS_BANK_Z: f64 = 28.0;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 72.0;
const COUPON_SLOT_Z: f64 = 12.0;
const COUPON_PITCH_X: f64 = 58.0;
const COUPON_PITCH_Y: f64 = 92.0;
const SLIDE_SLOT_X: f64 = 88.0;
const SLIDE_SLOT_Y: f64 = 28.0;
const SLIDE_PITCH_Y: f64 = 48.0;

const DWELL_CENTER: (f64, f64) = (420.0, 20.0);
const DWELL_BOARD_X: f64 = 396.0;
const DWELL_BOARD_Y: f64 = 154.0;
const DWELL_BOARD_Z: f64 = 16.0;
const DWELL_TOKEN_D: f64 = 24.0;
const DWELL_TOKEN_Z: f64 = 5.0;
const DWELL_BAR_X: f64 = 42.0;
const DWELL_BAR_Y: f64 = 12.0;

const WELL_CENTER: (f64, f64) = (426.0, -206.0);
const WELL_BANK_X: f64 = 424.0;
const WELL_BANK_Y: f64 = 220.0;
const WELL_BANK_Z: f64 = 34.0;
const WELL_D: f64 = 38.0;
const WELL_DEPTH: f64 = 18.0;
const WELL_PITCH_X: f64 = 74.0;
const WELL_ROW_PITCH_Y: f64 = 82.0;
const WELL_OVERFLOW_W: f64 = 10.0;

const FIDUCIAL_CENTER: (f64, f64) = (-188.0, 320.0);
const FIDUCIAL_FRAME_X: f64 = CASSETTE_X + 126.0;
const FIDUCIAL_FRAME_Y: f64 = 152.0;
const FIDUCIAL_FRAME_Z: f64 = 22.0;
const FIDUCIAL_RAIL_W: f64 = 13.0;
const FIDUCIAL_TARGET_D: f64 = 18.0;
const FIDUCIAL_LED_BAR_X: f64 = FIDUCIAL_FRAME_X - 80.0;
const FIDUCIAL_LED_BAR_Y: f64 = 12.0;

const POSITION_LABEL_CENTER: (f64, f64) = (-188.0, -266.0);
const POSITION_LABEL_BOARD_X: f64 = CASSETTE_X + 72.0;
const POSITION_LABEL_BOARD_Y: f64 = 126.0;
const POSITION_LABEL_BOARD_Z: f64 = 12.0;
const POSITION_LABEL_LAND_X: f64 = 30.0;
const POSITION_LABEL_LAND_Y: f64 = 14.0;
const EDGE_MARK_D: f64 = 13.0;
const CENTER_MARK_D: f64 = 17.0;

const BUBBLE_CENTER: (f64, f64) = (-188.0, -74.0);
const BUBBLE_BOARD_X: f64 = CASSETTE_X + 44.0;
const BUBBLE_BOARD_Y: f64 = 134.0;
const BUBBLE_BOARD_Z: f64 = 15.0;
const BUBBLE_POCKET_D: f64 = 14.0;
const BUBBLE_POCKET_DEPTH: f64 = 7.0;
const WETNESS_STRIP_X: f64 = 18.0;
const WETNESS_STRIP_Y: f64 = 52.0;

const STATUS_CENTER: (f64, f64) = (398.0, -362.0);
const STATUS_BOARD_X: f64 = 426.0;
const STATUS_BOARD_Y: f64 = 132.0;
const STATUS_BOARD_Z: f64 = 22.0;
const STATUS_LANE_X: f64 = 118.0;
const STATUS_LANE_Y: f64 = 86.0;
const STATUS_LANE_PITCH_X: f64 = 134.0;
const BARCODE_LAND_X: f64 = 92.0;
const BARCODE_LAND_Y: f64 = 20.0;

const KEEP_OUT_X: f64 = 1370.0;
const KEEP_OUT_Y: f64 = 828.0;
const ROBOT_FRONT_CLEARANCE: f64 = 320.0;
const SERVICE_REAR_CLEARANCE: f64 = 252.0;
const CAMERA_CLEARANCE_Z: f64 = 148.0;
const PORT_SERVICE_CLEARANCE: f64 = 188.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;

const CONTROL_CENTER: (f64, f64) = (-580.0, 164.0);
const CONTROL_BANK_X: f64 = 236.0;
const CONTROL_BANK_Y: f64 = 190.0;
const CONTROL_BANK_Z: f64 = 22.0;
const CONTROL_COUPON_D: f64 = 28.0;
const CONTROL_PITCH_X: f64 = 64.0;
const CONTROL_PITCH_Y: f64 = 58.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - LEAK_TRAY_RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - LEAK_TRAY_RIM_W - 12.0;

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
    assert_layout();

    let deck = leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let nest = cassette_datum_nest();
    export(OUTPUTS[1], &nest);

    let ports = row_column_inlet_surrogate_ports();
    export(OUTPUTS[2], &ports);

    let witnesses = fluorescent_witness_coupon_slide_slots();
    export(OUTPUTS[3], &witnesses);

    let dwell = timed_dwell_token_lands();
    export(OUTPUTS[4], &dwell);

    let wells = wash_recovery_wells();
    export(OUTPUTS[5], &wells);

    let fiducials = imaging_fiducial_frame();
    export(OUTPUTS[6], &fiducials);

    let position_labels = edge_center_position_labels();
    export(OUTPUTS[7], &position_labels);

    let bubble_wetness = bubble_wetness_witness_pockets();
    export(OUTPUTS[8], &bubble_wetness);

    let status = barcode_status_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let controls = reference_controls();
    export(OUTPUTS[11], &controls);

    let assembly = leak_tray_deck()
        + cassette_datum_nest().translate(
            NEST_CENTER.0,
            NEST_CENTER.1,
            DECK_Z / 2.0 + NEST_Z / 2.0,
        )
        + row_column_inlet_surrogate_ports().translate(
            PORT_MANIFOLD_CENTER.0,
            PORT_MANIFOLD_CENTER.1,
            DECK_Z / 2.0 + PORT_MANIFOLD_Z / 2.0,
        )
        + fluorescent_witness_coupon_slide_slots().translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            DECK_Z / 2.0 + WITNESS_BANK_Z / 2.0,
        )
        + timed_dwell_token_lands().translate(
            DWELL_CENTER.0,
            DWELL_CENTER.1,
            DECK_Z / 2.0 + DWELL_BOARD_Z / 2.0,
        )
        + wash_recovery_wells().translate(
            WELL_CENTER.0,
            WELL_CENTER.1,
            DECK_Z / 2.0 + WELL_BANK_Z / 2.0,
        )
        + imaging_fiducial_frame().translate(
            FIDUCIAL_CENTER.0,
            FIDUCIAL_CENTER.1,
            DECK_Z / 2.0 + FIDUCIAL_FRAME_Z / 2.0,
        )
        + edge_center_position_labels().translate(
            POSITION_LABEL_CENTER.0,
            POSITION_LABEL_CENTER.1,
            DECK_Z / 2.0 + POSITION_LABEL_BOARD_Z / 2.0,
        )
        + bubble_wetness_witness_pockets().translate(
            BUBBLE_CENTER.0,
            BUBBLE_CENTER.1,
            DECK_Z / 2.0 + BUBBLE_BOARD_Z / 2.0,
        )
        + barcode_status_lanes().translate(
            STATUS_CENTER.0,
            STATUS_CENTER.1,
            DECK_Z / 2.0 + STATUS_BOARD_Z / 2.0,
        )
        + robot_service_keepouts().translate(0.0, 0.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0)
        + reference_controls().translate(
            CONTROL_CENTER.0,
            CONTROL_CENTER.1,
            DECK_Z / 2.0 + CONTROL_BANK_Z / 2.0,
        );
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cassette seed distribution fluorescent uniformity station:");
    println!(
        "  Cassette map: {CASSETTE_ROWS} rows x {CASSETTE_COLS} columns = {CASSETTE_POSITIONS} surrogate seed positions"
    );
    println!(
        "  Inlet challenge: {ROW_PORTS} row ports + {COLUMN_PORTS} column ports = {INLET_PORTS} row/column surrogate ports"
    );
    println!(
        "  Evidence: {WITNESS_COUPONS} fluorescent coupons, {SLIDE_SLOTS} slide slots, {BUBBLE_WITNESS_COUNT} wetness/bubble pockets, {CONTROL_COUPONS} reference controls"
    );
    println!(
        "  Labels/status: {EDGE_POSITIONS} edge and {CENTER_POSITIONS} center position lands with {DWELL_TOKEN_COUNT} dwell tokens and {STATUS_LANES} status lanes"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(
        CASSETTE_POSITIONS, 20,
        "cassette map must cover 20 positions"
    );
    assert_eq!(INLET_PORTS, 9, "row/column inlet port count changed");
    assert_eq!(
        EDGE_POSITIONS + CENTER_POSITIONS,
        CASSETTE_POSITIONS,
        "edge and center labels must cover every cassette position"
    );
    assert_eq!(OUTPUTS.len(), 13, "expected stable STL export count");
    assert!(
        CASSETTE_Z > REVC_TOTAL_HEIGHT,
        "datum nest cassette surrogate must clear the Rev C chip height"
    );
    assert!(
        CAMERA_CLEARANCE_Z > CASSETTE_Z + 80.0,
        "imaging frame needs vertical clearance over the sealed cassette"
    );
    assert!(
        ROBOT_FRONT_CLEARANCE >= 300.0 && SERVICE_REAR_CLEARANCE >= 240.0,
        "robot and service clearances are below the validation station target"
    );
    assert!(
        PORT_SERVICE_CLEARANCE >= 180.0,
        "surrogate inlet manifold service clearance is too small"
    );

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside(),
            "{} does not fit on leak tray deck",
            rect.name
        );
    }
    assert!(
        keepout_outline_rect().fits_inside(),
        "robot/service keepout outline does not fit on leak tray deck"
    );

    assert!(
        !rect_by_name(&rects, "row_column_inlet_ports")
            .overlaps(rect_by_name(&rects, "wash_recovery_wells")),
        "surrogate inlet manifold overlaps wash/recovery wells"
    );
    assert!(
        !rect_by_name(&rects, "witness_coupon_slide_slots")
            .overlaps(rect_by_name(&rects, "barcode_status_lanes")),
        "witness coupon bank overlaps barcode/status lanes"
    );
    assert!(
        !rect_by_name(&rects, "reference_controls")
            .overlaps(rect_by_name(&rects, "witness_coupon_slide_slots")),
        "reference controls overlap witness coupon bank"
    );
}

fn layout_rects() -> [Rect; 10] {
    [
        Rect {
            name: "cassette_datum_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "row_column_inlet_ports",
            center: PORT_MANIFOLD_CENTER,
            x: PORT_MANIFOLD_X,
            y: PORT_MANIFOLD_Y,
        },
        Rect {
            name: "witness_coupon_slide_slots",
            center: WITNESS_CENTER,
            x: WITNESS_BANK_X,
            y: WITNESS_BANK_Y,
        },
        Rect {
            name: "timed_dwell_tokens",
            center: DWELL_CENTER,
            x: DWELL_BOARD_X,
            y: DWELL_BOARD_Y,
        },
        Rect {
            name: "wash_recovery_wells",
            center: WELL_CENTER,
            x: WELL_BANK_X,
            y: WELL_BANK_Y,
        },
        Rect {
            name: "imaging_fiducial_frame",
            center: FIDUCIAL_CENTER,
            x: FIDUCIAL_FRAME_X,
            y: FIDUCIAL_FRAME_Y,
        },
        Rect {
            name: "edge_center_position_labels",
            center: POSITION_LABEL_CENTER,
            x: POSITION_LABEL_BOARD_X,
            y: POSITION_LABEL_BOARD_Y,
        },
        Rect {
            name: "bubble_wetness_witness_pockets",
            center: BUBBLE_CENTER,
            x: BUBBLE_BOARD_X,
            y: BUBBLE_BOARD_Y,
        },
        Rect {
            name: "barcode_status_lanes",
            center: STATUS_CENTER,
            x: STATUS_BOARD_X,
            y: STATUS_BOARD_Y,
        },
        Rect {
            name: "reference_controls",
            center: CONTROL_CENTER,
            x: CONTROL_BANK_X,
            y: CONTROL_BANK_Y,
        },
    ]
}

fn keepout_outline_rect() -> Rect {
    Rect {
        name: "robot_service_keepout_outline",
        center: (0.0, 0.0),
        x: KEEP_OUT_X,
        y: KEEP_OUT_Y,
    }
}

fn rect_by_name(rects: &[Rect], name: &str) -> Rect {
    *rects
        .iter()
        .find(|rect| rect.name == name)
        .expect("named layout rect must exist")
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_seed_uniformity_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let rim_front = centered_cube(
        "closed_seed_uniformity_leak_tray_front_rim",
        DECK_X,
        LEAK_TRAY_RIM_W,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - LEAK_TRAY_RIM_W / 2.0),
        DECK_Z / 2.0 + LEAK_TRAY_RIM_Z / 2.0,
    );
    let rim_back = centered_cube(
        "closed_seed_uniformity_leak_tray_back_rim",
        DECK_X,
        LEAK_TRAY_RIM_W,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - LEAK_TRAY_RIM_W / 2.0,
        DECK_Z / 2.0 + LEAK_TRAY_RIM_Z / 2.0,
    );
    let rim_left = centered_cube(
        "closed_seed_uniformity_leak_tray_left_rim",
        LEAK_TRAY_RIM_W,
        DECK_Y,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        -(DECK_X / 2.0 - LEAK_TRAY_RIM_W / 2.0),
        0.0,
        DECK_Z / 2.0 + LEAK_TRAY_RIM_Z / 2.0,
    );
    let rim_right = centered_cube(
        "closed_seed_uniformity_leak_tray_right_rim",
        LEAK_TRAY_RIM_W,
        DECK_Y,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - LEAK_TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + LEAK_TRAY_RIM_Z / 2.0,
    );
    let drain = centered_cube(
        "closed_seed_uniformity_leak_tray_drain_channel",
        DECK_X - 110.0,
        DRAIN_CHANNEL_W,
        6.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 - 72.0), DECK_Z / 2.0 - 1.8);

    deck + rim_front + rim_back + rim_left + rim_right - drain - mounting_holes()
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("closed_seed_uniformity_station_mounting_holes");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_seed_uniformity_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 3.0,
                32,
            )
            .translate(sx * (DECK_X / 2.0 - 46.0), sy * (DECK_Y / 2.0 - 46.0), 0.0);
    }
    holes
}

fn cassette_datum_nest() -> Part {
    let base = centered_cube(
        "closed_seed_uniformity_cassette_datum_nest_base",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let socket = centered_cube(
        "closed_seed_uniformity_cassette_socket_relief",
        CASSETTE_X + 2.0,
        CASSETTE_Y + 2.0,
        NEST_SOCKET_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 - NEST_SOCKET_DEPTH / 2.0 + 0.5);

    base - socket + nest_datum_rails() + nest_datum_pins() + cassette_position_reliefs()
}

fn nest_datum_rails() -> Part {
    let left = centered_cube(
        "closed_seed_uniformity_left_hard_datum_rail",
        NEST_RAIL_W,
        NEST_Y,
        NEST_RAIL_Z,
    )
    .translate(
        -(NEST_X / 2.0 - NEST_RAIL_W / 2.0),
        0.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let back = centered_cube(
        "closed_seed_uniformity_back_hard_datum_rail",
        NEST_X,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        NEST_Y / 2.0 - NEST_RAIL_W / 2.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let front_soft = centered_cube(
        "closed_seed_uniformity_front_soft_capture_rail",
        NEST_X * 0.74,
        NEST_RAIL_W,
        NEST_RAIL_Z * 0.66,
    )
    .translate(
        18.0,
        -(NEST_Y / 2.0 - NEST_RAIL_W / 2.0),
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.33,
    );
    let right_soft = centered_cube(
        "closed_seed_uniformity_right_soft_capture_rail",
        NEST_RAIL_W,
        NEST_Y * 0.72,
        NEST_RAIL_Z * 0.66,
    )
    .translate(
        NEST_X / 2.0 - NEST_RAIL_W / 2.0,
        -10.0,
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.33,
    );

    left + back + front_soft + right_soft
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("closed_seed_uniformity_cassette_datum_pins");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 22.0), CASSETTE_Y / 2.0 - 24.0),
        (CASSETTE_X / 2.0 - 34.0, CASSETTE_Y / 2.0 - 24.0),
        (-(CASSETTE_X / 2.0 - 22.0), -(CASSETTE_Y / 2.0 - 28.0)),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_seed_uniformity_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0,
            10.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        let pilot = centered_cylinder(
            format!("closed_seed_uniformity_datum_pin_pilot_{i}"),
            DATUM_PIN_D * 0.24,
            15.0,
            28,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 7.5);
        pins = pins + boss + pilot;
    }
    pins
}

fn cassette_position_reliefs() -> Part {
    let mut reliefs = Part::empty("closed_seed_uniformity_twenty_position_chip_reliefs");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let idx = row * CASSETTE_COLS + col;
            let (x, y) = cassette_position_xy(
                row,
                col,
                REVC_CHIP_LENGTH + CHIP_GAP_X,
                REVC_CHIP_WIDTH + CHIP_GAP_Y,
            );
            let pocket = centered_cube(
                format!("closed_seed_uniformity_chip_position_{idx:02}_relief"),
                REVC_CHIP_LENGTH,
                REVC_CHIP_WIDTH,
                CHIP_RELIEF_Z,
            )
            .translate(x, y, NEST_Z / 2.0 + CHIP_RELIEF_Z / 2.0);
            let flow_cross = centered_cube(
                format!("closed_seed_uniformity_chip_position_{idx:02}_flow_cross"),
                REVC_CHIP_LENGTH * 0.58,
                5.0,
                4.0,
            )
            .translate(x, y, NEST_Z / 2.0 + CHIP_RELIEF_Z + 2.0);
            reliefs = reliefs + pocket + flow_cross;
        }
    }
    reliefs
}

fn row_column_inlet_surrogate_ports() -> Part {
    let base = centered_cube(
        "closed_seed_uniformity_row_column_inlet_manifold_base",
        PORT_MANIFOLD_X,
        PORT_MANIFOLD_Y,
        PORT_MANIFOLD_Z,
    );
    let mut bores = Part::empty("closed_seed_uniformity_inlet_surrogate_bores");
    let mut bosses = Part::empty("closed_seed_uniformity_inlet_surrogate_bosses");
    let mut channels = Part::empty("closed_seed_uniformity_row_column_channel_lands");

    for row in 0..ROW_PORTS {
        let y = (row as f64 - (ROW_PORTS as f64 - 1.0) / 2.0) * PORT_PITCH_Y;
        let x = -PORT_MANIFOLD_X / 2.0 + 70.0;
        let boss = port_boss(
            format!("closed_seed_uniformity_row_inlet_boss_r{row}"),
            x,
            y,
        );
        let bore = port_bore(
            format!("closed_seed_uniformity_row_inlet_bore_r{row}"),
            x,
            y,
        );
        let channel = centered_cube(
            format!("closed_seed_uniformity_row_inlet_label_land_r{row}"),
            PORT_LABEL_LAND_X,
            PORT_LABEL_LAND_Y,
            4.0,
        )
        .translate(x + 48.0, y, PORT_MANIFOLD_Z / 2.0 + 2.0);
        bosses = bosses + boss;
        bores = bores + bore;
        channels = channels + channel;
    }

    for col in 0..COLUMN_PORTS {
        let x = -40.0 + col as f64 * PORT_PITCH_X;
        let y = PORT_MANIFOLD_Y / 2.0 - 58.0;
        let boss = port_boss(
            format!("closed_seed_uniformity_column_inlet_boss_c{col}"),
            x,
            y,
        );
        let bore = port_bore(
            format!("closed_seed_uniformity_column_inlet_bore_c{col}"),
            x,
            y,
        );
        let channel = centered_cube(
            format!("closed_seed_uniformity_column_inlet_label_land_c{col}"),
            PORT_LABEL_LAND_X,
            PORT_LABEL_LAND_Y,
            4.0,
        )
        .translate(x, y - 42.0, PORT_MANIFOLD_Z / 2.0 + 2.0);
        bosses = bosses + boss;
        bores = bores + bore;
        channels = channels + channel;
    }

    let row_header = centered_cube(
        "closed_seed_uniformity_row_port_distribution_header",
        PORT_CHANNEL_W,
        PORT_MANIFOLD_Y - 52.0,
        5.0,
    )
    .translate(
        -PORT_MANIFOLD_X / 2.0 + 116.0,
        0.0,
        PORT_MANIFOLD_Z / 2.0 + 2.5,
    );
    let column_header = centered_cube(
        "closed_seed_uniformity_column_port_distribution_header",
        PORT_MANIFOLD_X - 154.0,
        PORT_CHANNEL_W,
        5.0,
    )
    .translate(
        54.0,
        PORT_MANIFOLD_Y / 2.0 - 104.0,
        PORT_MANIFOLD_Z / 2.0 + 2.5,
    );

    base + bosses + channels + row_header + column_header - bores
}

fn port_boss(name: String, x: f64, y: f64) -> Part {
    centered_cylinder(name, PORT_BOSS_D / 2.0, 12.0, 36).translate(
        x,
        y,
        PORT_MANIFOLD_Z / 2.0 + 6.0,
    )
}

fn port_bore(name: String, x: f64, y: f64) -> Part {
    centered_cylinder(name, PORT_BORE_D / 2.0, PORT_MANIFOLD_Z + 18.0, 32).translate(x, y, 0.0)
}

fn fluorescent_witness_coupon_slide_slots() -> Part {
    let base = centered_cube(
        "closed_seed_uniformity_fluorescent_witness_bank",
        WITNESS_BANK_X,
        WITNESS_BANK_Y,
        WITNESS_BANK_Z,
    );
    let mut cutouts = Part::empty("closed_seed_uniformity_witness_coupon_slide_cutouts");
    let mut clips = Part::empty("closed_seed_uniformity_witness_coupon_retention_clips");

    for i in 0..WITNESS_COUPONS {
        let col = i % 5;
        let row = i / 5;
        let x = (col as f64 - 2.0) * COUPON_PITCH_X;
        let y = 48.0 - row as f64 * COUPON_PITCH_Y;
        let slot = centered_cube(
            format!("closed_seed_uniformity_fluorescent_coupon_slot_{i:02}"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            COUPON_SLOT_Z + 1.0,
        )
        .translate(x, y, WITNESS_BANK_Z / 2.0 - COUPON_SLOT_Z / 2.0 + 0.5);
        let clip_a = centered_cube(
            format!("closed_seed_uniformity_fluorescent_coupon_{i:02}_left_clip"),
            6.0,
            COUPON_SLOT_Y + 10.0,
            8.0,
        )
        .translate(x - COUPON_SLOT_X / 2.0 - 5.0, y, WITNESS_BANK_Z / 2.0 + 4.0);
        let clip_b = centered_cube(
            format!("closed_seed_uniformity_fluorescent_coupon_{i:02}_right_clip"),
            6.0,
            COUPON_SLOT_Y + 10.0,
            8.0,
        )
        .translate(x + COUPON_SLOT_X / 2.0 + 5.0, y, WITNESS_BANK_Z / 2.0 + 4.0);
        cutouts = cutouts + slot;
        clips = clips + clip_a + clip_b;
    }

    for i in 0..SLIDE_SLOTS {
        let y = -WITNESS_BANK_Y / 2.0 + 38.0 + i as f64 * SLIDE_PITCH_Y;
        let slot = centered_cube(
            format!("closed_seed_uniformity_fluorescent_slide_slot_{i}"),
            SLIDE_SLOT_X,
            SLIDE_SLOT_Y,
            COUPON_SLOT_Z + 2.0,
        )
        .translate(WITNESS_BANK_X / 2.0 - 62.0, y, WITNESS_BANK_Z / 2.0 - 5.0);
        cutouts = cutouts + slot;
    }

    base - cutouts + clips
}

fn timed_dwell_token_lands() -> Part {
    let board = centered_cube(
        "closed_seed_uniformity_timed_dwell_token_board",
        DWELL_BOARD_X,
        DWELL_BOARD_Y,
        DWELL_BOARD_Z,
    );
    let mut tokens = Part::empty("closed_seed_uniformity_timed_dwell_token_lands");
    for i in 0..DWELL_TOKEN_COUNT {
        let x = (i as f64 - (DWELL_TOKEN_COUNT as f64 - 1.0) / 2.0) * 44.0;
        let token = centered_cylinder(
            format!("closed_seed_uniformity_dwell_token_land_{i}"),
            DWELL_TOKEN_D / 2.0,
            DWELL_TOKEN_Z,
            36,
        )
        .translate(x, 18.0, DWELL_BOARD_Z / 2.0 + DWELL_TOKEN_Z / 2.0);
        let time_bar = centered_cube(
            format!("closed_seed_uniformity_dwell_time_bar_{i}"),
            DWELL_BAR_X,
            DWELL_BAR_Y,
            3.0,
        )
        .translate(x, -34.0, DWELL_BOARD_Z / 2.0 + 1.5);
        tokens = tokens + token + time_bar;
    }
    board + tokens
}

fn wash_recovery_wells() -> Part {
    let base = centered_cube(
        "closed_seed_uniformity_wash_recovery_well_bank",
        WELL_BANK_X,
        WELL_BANK_Y,
        WELL_BANK_Z,
    );
    let mut wells = Part::empty("closed_seed_uniformity_wash_recovery_well_cutouts");
    let mut labels = Part::empty("closed_seed_uniformity_wash_recovery_well_label_lands");

    for i in 0..(WASH_WELLS + RECOVERY_WELLS) {
        let row = i / 4;
        let col = i % 4;
        let x = (col as f64 - 1.5) * WELL_PITCH_X;
        let y = (0.5 - row as f64) * WELL_ROW_PITCH_Y;
        let well = centered_cylinder(
            format!("closed_seed_uniformity_wash_recovery_well_cutout_{i}"),
            WELL_D / 2.0,
            WELL_DEPTH + 2.0,
            40,
        )
        .translate(x, y, WELL_BANK_Z / 2.0 - WELL_DEPTH / 2.0 + 1.0);
        let label = centered_cube(
            format!("closed_seed_uniformity_wash_recovery_well_label_land_{i}"),
            44.0,
            14.0,
            3.0,
        )
        .translate(x, y - 38.0, WELL_BANK_Z / 2.0 + 1.5);
        wells = wells + well;
        labels = labels + label;
    }

    let overflow = centered_cube(
        "closed_seed_uniformity_wash_recovery_overflow_trench",
        WELL_BANK_X - 72.0,
        WELL_OVERFLOW_W,
        7.0,
    )
    .translate(0.0, 0.0, WELL_BANK_Z / 2.0 - 2.0);

    base - wells - overflow + labels
}

fn imaging_fiducial_frame() -> Part {
    let front = centered_cube(
        "closed_seed_uniformity_imaging_fiducial_front_rail",
        FIDUCIAL_FRAME_X,
        FIDUCIAL_RAIL_W,
        FIDUCIAL_FRAME_Z,
    )
    .translate(0.0, -FIDUCIAL_FRAME_Y / 2.0 + FIDUCIAL_RAIL_W / 2.0, 0.0);
    let back = centered_cube(
        "closed_seed_uniformity_imaging_fiducial_back_rail",
        FIDUCIAL_FRAME_X,
        FIDUCIAL_RAIL_W,
        FIDUCIAL_FRAME_Z,
    )
    .translate(0.0, FIDUCIAL_FRAME_Y / 2.0 - FIDUCIAL_RAIL_W / 2.0, 0.0);
    let left = centered_cube(
        "closed_seed_uniformity_imaging_fiducial_left_rail",
        FIDUCIAL_RAIL_W,
        FIDUCIAL_FRAME_Y,
        FIDUCIAL_FRAME_Z,
    )
    .translate(-FIDUCIAL_FRAME_X / 2.0 + FIDUCIAL_RAIL_W / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "closed_seed_uniformity_imaging_fiducial_right_rail",
        FIDUCIAL_RAIL_W,
        FIDUCIAL_FRAME_Y,
        FIDUCIAL_FRAME_Z,
    )
    .translate(FIDUCIAL_FRAME_X / 2.0 - FIDUCIAL_RAIL_W / 2.0, 0.0, 0.0);
    let led_bar = centered_cube(
        "closed_seed_uniformity_imaging_excitation_led_bar_land",
        FIDUCIAL_LED_BAR_X,
        FIDUCIAL_LED_BAR_Y,
        6.0,
    )
    .translate(0.0, 0.0, FIDUCIAL_FRAME_Z / 2.0 + 3.0);

    front + back + left + right + led_bar + fiducial_targets()
}

fn fiducial_targets() -> Part {
    let mut targets = Part::empty("closed_seed_uniformity_imaging_fiducial_targets");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let outer = centered_cylinder(
            format!("closed_seed_uniformity_fiducial_outer_target_{i}"),
            FIDUCIAL_TARGET_D / 2.0,
            5.0,
            36,
        )
        .translate(
            sx * (FIDUCIAL_FRAME_X / 2.0 - 34.0),
            sy * (FIDUCIAL_FRAME_Y / 2.0 - 30.0),
            FIDUCIAL_FRAME_Z / 2.0 + 2.5,
        );
        let inner = centered_cylinder(
            format!("closed_seed_uniformity_fiducial_inner_target_{i}"),
            FIDUCIAL_TARGET_D * 0.22,
            7.0,
            28,
        )
        .translate(
            sx * (FIDUCIAL_FRAME_X / 2.0 - 34.0),
            sy * (FIDUCIAL_FRAME_Y / 2.0 - 30.0),
            FIDUCIAL_FRAME_Z / 2.0 + 3.5,
        );
        targets = targets + outer + inner;
    }
    targets
}

fn edge_center_position_labels() -> Part {
    let board = centered_cube(
        "closed_seed_uniformity_edge_center_position_label_board",
        POSITION_LABEL_BOARD_X,
        POSITION_LABEL_BOARD_Y,
        POSITION_LABEL_BOARD_Z,
    );
    let mut labels = Part::empty("closed_seed_uniformity_edge_center_position_label_lands");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let idx = row * CASSETTE_COLS + col;
            let (x, y) = cassette_position_xy(
                row,
                col,
                REVC_CHIP_LENGTH + CHIP_GAP_X,
                POSITION_LABEL_LAND_Y + 8.0,
            );
            let label = centered_cube(
                format!("closed_seed_uniformity_position_{idx:02}_label_land"),
                POSITION_LABEL_LAND_X,
                POSITION_LABEL_LAND_Y,
                3.0,
            )
            .translate(x, y, POSITION_LABEL_BOARD_Z / 2.0 + 1.5);
            let marker_d = if is_edge_position(row, col) {
                EDGE_MARK_D
            } else {
                CENTER_MARK_D
            };
            let marker = centered_cylinder(
                format!("closed_seed_uniformity_position_{idx:02}_edge_center_marker"),
                marker_d / 2.0,
                4.0,
                32,
            )
            .translate(x, y + 18.0, POSITION_LABEL_BOARD_Z / 2.0 + 2.0);
            labels = labels + label + marker;
        }
    }
    board + labels
}

fn bubble_wetness_witness_pockets() -> Part {
    let board = centered_cube(
        "closed_seed_uniformity_bubble_wetness_witness_board",
        BUBBLE_BOARD_X,
        BUBBLE_BOARD_Y,
        BUBBLE_BOARD_Z,
    );
    let mut pockets = Part::empty("closed_seed_uniformity_bubble_wetness_pocket_cutouts");
    let mut wetness = Part::empty("closed_seed_uniformity_wetness_strip_lands");

    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let idx = row * CASSETTE_COLS + col;
            let (x, y) = cassette_position_xy(
                row,
                col,
                REVC_CHIP_LENGTH + CHIP_GAP_X,
                BUBBLE_POCKET_D + 8.0,
            );
            let pocket = centered_cylinder(
                format!("closed_seed_uniformity_bubble_pocket_{idx:02}"),
                BUBBLE_POCKET_D / 2.0,
                BUBBLE_POCKET_DEPTH + 1.0,
                28,
            )
            .translate(x, y, BUBBLE_BOARD_Z / 2.0 - BUBBLE_POCKET_DEPTH / 2.0 + 0.5);
            let strip = centered_cube(
                format!("closed_seed_uniformity_wetness_strip_{idx:02}"),
                WETNESS_STRIP_X,
                WETNESS_STRIP_Y,
                3.0,
            )
            .translate(x + 21.0, y, BUBBLE_BOARD_Z / 2.0 + 1.5);
            pockets = pockets + pocket;
            wetness = wetness + strip;
        }
    }

    board - pockets + wetness
}

fn barcode_status_lanes() -> Part {
    let board = centered_cube(
        "closed_seed_uniformity_barcode_status_lane_board",
        STATUS_BOARD_X,
        STATUS_BOARD_Y,
        STATUS_BOARD_Z,
    );
    let mut lanes = Part::empty("closed_seed_uniformity_barcode_status_lanes");
    for lane in 0..STATUS_LANES {
        let x = (lane as f64 - 1.0) * STATUS_LANE_PITCH_X;
        let pocket = centered_cube(
            format!("closed_seed_uniformity_status_lane_{lane}_recess"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            9.0,
        )
        .translate(x, 0.0, STATUS_BOARD_Z / 2.0 - 4.0);
        let barcode = centered_cube(
            format!("closed_seed_uniformity_status_lane_{lane}_barcode_land"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            4.0,
        )
        .translate(x, -STATUS_LANE_Y / 2.0 + 20.0, STATUS_BOARD_Z / 2.0 + 2.0);
        let status_tab = centered_cube(
            format!("closed_seed_uniformity_status_lane_{lane}_pass_hold_fail_tab"),
            76.0,
            20.0,
            5.0,
        )
        .translate(x, STATUS_LANE_Y / 2.0 - 20.0, STATUS_BOARD_Z / 2.0 + 2.5);
        lanes = lanes - pocket + barcode + status_tab;
    }
    board + lanes
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "closed_seed_uniformity_robot_front_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, -(KEEP_OUT_Y / 2.0), 0.0);
    let robot_back = centered_cube(
        "closed_seed_uniformity_service_rear_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let side_left = centered_cube(
        "closed_seed_uniformity_robot_left_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let side_right = centered_cube(
        "closed_seed_uniformity_robot_right_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    let camera_clearance = centered_cube(
        "closed_seed_uniformity_camera_clearance_height_gauge",
        34.0,
        34.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, CAMERA_CLEARANCE_Z / 2.0);
    let port_service = centered_cube(
        "closed_seed_uniformity_port_service_pull_keepout_gauge",
        PORT_SERVICE_CLEARANCE,
        18.0,
        36.0,
    )
    .translate(PORT_MANIFOLD_CENTER.0, PORT_MANIFOLD_CENTER.1 - 164.0, 18.0);

    robot_front + robot_back + side_left + side_right + camera_clearance + port_service
}

fn reference_controls() -> Part {
    let base = centered_cube(
        "closed_seed_uniformity_positive_negative_reference_control_bank",
        CONTROL_BANK_X,
        CONTROL_BANK_Y,
        CONTROL_BANK_Z,
    );
    let mut controls = Part::empty("closed_seed_uniformity_reference_control_coupon_lands");
    for i in 0..CONTROL_COUPONS {
        let col = i % 3;
        let row = i / 3;
        let x = (col as f64 - 1.0) * CONTROL_PITCH_X;
        let y = (0.5 - row as f64) * CONTROL_PITCH_Y;
        let coupon = centered_cylinder(
            format!("closed_seed_uniformity_reference_control_coupon_land_{i}"),
            CONTROL_COUPON_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y, CONTROL_BANK_Z / 2.0 + 2.5);
        let barcode = centered_cube(
            format!("closed_seed_uniformity_reference_control_barcode_land_{i}"),
            42.0,
            12.0,
            3.0,
        )
        .translate(x, y - 30.0, CONTROL_BANK_Z / 2.0 + 1.5);
        controls = controls + coupon + barcode;
    }
    base + controls
}

fn cassette_position_xy(row: usize, col: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let x = (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * pitch_x;
    let y = ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * pitch_y;
    (x, y)
}

fn is_edge_position(row: usize, col: usize) -> bool {
    row == 0 || row == CASSETTE_ROWS - 1 || col == 0 || col == CASSETTE_COLS - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_all_twenty_positions() {
        assert_eq!(CASSETTE_POSITIONS, 20);
        assert_eq!(EDGE_POSITIONS, 14);
        assert_eq!(CENTER_POSITIONS, 6);

        let mut edge_count = 0;
        let mut center_count = 0;
        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                if is_edge_position(row, col) {
                    edge_count += 1;
                } else {
                    center_count += 1;
                }
            }
        }
        assert_eq!(edge_count, EDGE_POSITIONS);
        assert_eq!(center_count, CENTER_POSITIONS);
    }

    #[test]
    fn inlet_ports_cover_rows_and_columns() {
        assert_eq!(ROW_PORTS, CASSETTE_ROWS);
        assert_eq!(COLUMN_PORTS, CASSETTE_COLS);
        assert_eq!(INLET_PORTS, ROW_PORTS + COLUMN_PORTS);
    }

    #[test]
    fn layout_has_no_internal_overlaps() {
        assert_layout();
    }
}
