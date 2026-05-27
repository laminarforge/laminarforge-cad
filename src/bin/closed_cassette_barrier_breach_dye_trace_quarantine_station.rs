use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette barrier breach dye-trace quarantine station.
//
// Standalone validation fixture for checking sealed cassette barrier integrity
// with dye-trace witness paths before live chips are loaded. The geometry
// models a sealed cassette nest grid, dye injection manifold, breach witness
// windows, quarantine gate, pressure-decay coupon pockets, waste wells,
// custody/barcode lands, clean/dirty segregation, camera bridge, and robot
// service keepout gauges.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_deck.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_sealed_cassette_nest_grid.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_dye_trace_injection_ports.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_breach_witness_windows.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_quarantine_lane_gate.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_pressure_decay_coupon_pockets.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_waste_capture_wells.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_barcode_custody_lands.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_clean_dirty_segregation.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_camera_evidence_bridge.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_cassette_nest_grid",
    "dye_trace_injection_ports",
    "breach_witness_windows",
    "quarantine_lane_gate",
    "pressure_decay_coupon_pockets",
    "waste_capture_wells",
    "barcode_custody_lands",
    "clean_dirty_segregation",
    "camera_evidence_bridge",
    "robot_service_keepouts",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITIONS: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITIONS: usize = 14;
const CENTER_POSITIONS: usize = CASSETTE_POSITIONS - EDGE_POSITIONS;
const INJECTION_PORTS: usize = CASSETTE_ROWS + CASSETTE_COLS + 1;
const WITNESS_WINDOWS: usize = CASSETTE_POSITIONS;
const PRESSURE_COUPONS: usize = 8;
const PRESSURE_REFERENCE_COUPONS: usize = 4;
const WASTE_WELLS: usize = 6;
const CUSTODY_LANDS: usize = CASSETTE_POSITIONS + PRESSURE_REFERENCE_COUPONS;
const DISPOSITION_LANES: usize = 3;
const QUARANTINE_CAPACITY: usize = CASSETTE_POSITIONS;
const KEEP_OUT_GAUGES: usize = 6;

const CHIP_GAP_X: f64 = 6.5;
const CHIP_GAP_Y: f64 = 5.5;
const CASSETTE_MARGIN_X: f64 = 36.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const CHIP_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP_X;
const CHIP_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP_Y;
const CASSETTE_X: f64 = CHIP_ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 36.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.6;
const INSERT_SOCKET_DEPTH: f64 = 6.0;

const NEST_CENTER: (f64, f64) = (-90.0, 88.0);
const NEST_X: f64 = CASSETTE_X + 50.0;
const NEST_Y: f64 = CASSETTE_Y + 42.0;
const NEST_Z: f64 = 38.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 30.0;
const GASKET_RIB_W: f64 = 5.0;
const GASKET_RIB_Z: f64 = 7.0;
const NEST_SOCKET_DEPTH: f64 = 9.0;
const POSITION_POCKET_Z: f64 = REVC_TOTAL_HEIGHT + 5.0;
const DATUM_PIN_D: f64 = 7.0;

const PORT_CENTER: (f64, f64) = (448.0, 260.0);
const PORT_BLOCK_X: f64 = 400.0;
const PORT_BLOCK_Y: f64 = 232.0;
const PORT_BLOCK_Z: f64 = 34.0;
const PORT_BOSS_D: f64 = 28.0;
const PORT_BORE_D: f64 = 10.0;
const PORT_PITCH_X: f64 = 70.0;
const PORT_PITCH_Y: f64 = 54.0;
const TRACE_CHANNEL_W: f64 = 8.0;
const TRACE_CHANNEL_Z: f64 = 6.0;
const PURGE_TRAP_D: f64 = 34.0;

const WITNESS_CENTER: (f64, f64) = (-560.0, -250.0);
const WITNESS_BANK_X: f64 = 276.0;
const WITNESS_BANK_Y: f64 = 258.0;
const WITNESS_BANK_Z: f64 = 24.0;
const WINDOW_FRAME_X: f64 = 48.0;
const WINDOW_FRAME_Y: f64 = 30.0;
const WINDOW_CUT_X: f64 = 34.0;
const WINDOW_CUT_Y: f64 = 18.0;
const WINDOW_PITCH_X: f64 = 58.0;
const WINDOW_PITCH_Y: f64 = 42.0;

const PRESSURE_CENTER: (f64, f64) = (-560.0, 168.0);
const PRESSURE_BANK_X: f64 = 276.0;
const PRESSURE_BANK_Y: f64 = 300.0;
const PRESSURE_BANK_Z: f64 = 30.0;
const COUPON_POCKET_X: f64 = 62.0;
const COUPON_POCKET_Y: f64 = 38.0;
const COUPON_POCKET_Z: f64 = 14.0;
const COUPON_PITCH_X: f64 = 82.0;
const COUPON_PITCH_Y: f64 = 58.0;
const REFERENCE_VOLUME_D: f64 = 24.0;

const WASTE_CENTER: (f64, f64) = (500.0, -210.0);
const WASTE_BANK_X: f64 = 364.0;
const WASTE_BANK_Y: f64 = 236.0;
const WASTE_BANK_Z: f64 = 34.0;
const WASTE_WELL_D: f64 = 42.0;
const WASTE_WELL_DEPTH: f64 = 20.0;
const WASTE_PITCH_X: f64 = 78.0;
const WASTE_PITCH_Y: f64 = 74.0;

const QUARANTINE_CENTER: (f64, f64) = (54.0, -360.0);
const QUARANTINE_BANK_X: f64 = 374.0;
const QUARANTINE_BANK_Y: f64 = 134.0;
const QUARANTINE_BANK_Z: f64 = 28.0;
const LANE_X: f64 = 102.0;
const LANE_Y: f64 = 92.0;
const LANE_PITCH_X: f64 = 120.0;
const LANE_WALL_W: f64 = 8.0;
const GATE_BAR_X: f64 = 322.0;
const GATE_BAR_Y: f64 = 12.0;
const TOKEN_SLOT_X: f64 = 26.0;
const TOKEN_SLOT_Y: f64 = 18.0;

const CUSTODY_CENTER: (f64, f64) = (500.0, 18.0);
const CUSTODY_BOARD_X: f64 = 360.0;
const CUSTODY_BOARD_Y: f64 = 166.0;
const CUSTODY_BOARD_Z: f64 = 16.0;
const BARCODE_LAND_X: f64 = 48.0;
const BARCODE_LAND_Y: f64 = 15.0;
const CUSTODY_TOKEN_D: f64 = 16.0;

const SEGREGATION_WALL_Y: f64 = -70.0;
const SEGREGATION_WALL_X: f64 = 1180.0;
const SEGREGATION_WALL_W: f64 = 16.0;
const SEGREGATION_WALL_Z: f64 = 58.0;
const CLEAN_ZONE_CENTER: (f64, f64) = (-292.0, -356.0);
const DIRTY_ZONE_CENTER: (f64, f64) = (286.0, -356.0);
const SEG_ZONE_X: f64 = 210.0;
const SEG_ZONE_Y: f64 = 118.0;
const SEG_ZONE_Z: f64 = 20.0;
const TRANSFER_GATE_X: f64 = 98.0;

const CAMERA_POST_W: f64 = 26.0;
const CAMERA_POST_Z: f64 = 168.0;
const CAMERA_BRIDGE_X: f64 = NEST_X + 132.0;
const CAMERA_BRIDGE_Y: f64 = 38.0;
const CAMERA_BRIDGE_Z: f64 = 22.0;
const CAMERA_CARRIAGE_X: f64 = 126.0;
const CAMERA_CARRIAGE_Y: f64 = 78.0;
const CAMERA_CARRIAGE_Z: f64 = 18.0;
const CAMERA_LENS_D: f64 = 30.0;
const CAMERA_CLEARANCE_Z: f64 = CAMERA_POST_Z - (NEST_Z + REVC_TOTAL_HEIGHT);

const KEEP_OUT_X: f64 = 1410.0;
const KEEP_OUT_Y: f64 = 842.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 136.0;
const SERVICE_PORT_CLEARANCE_X: f64 = 210.0;
const SERVICE_WASTE_CLEARANCE_X: f64 = 190.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;

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

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let nest = sealed_cassette_nest_grid();
    export(OUTPUTS[1], &nest);

    let ports = dye_trace_injection_ports();
    export(OUTPUTS[2], &ports);

    let windows = breach_witness_windows();
    export(OUTPUTS[3], &windows);

    let quarantine = quarantine_lane_gate();
    export(OUTPUTS[4], &quarantine);

    let pressure = pressure_decay_coupon_pockets();
    export(OUTPUTS[5], &pressure);

    let waste = waste_capture_wells();
    export(OUTPUTS[6], &waste);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let segregation = clean_dirty_segregation();
    export(OUTPUTS[8], &segregation);

    let camera = camera_evidence_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + nest
        + ports
        + windows
        + quarantine
        + pressure
        + waste
        + custody
        + segregation
        + camera
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette barrier breach dye-trace quarantine station:");
    println!(
        "  Cassette nest:          {CASSETTE_COLS}x{CASSETTE_ROWS} sealed grid, {EDGE_POSITIONS} edge and {CENTER_POSITIONS} center positions"
    );
    println!(
        "  Dye trace challenge:    {INJECTION_PORTS} injection/purge ports feeding {WITNESS_WINDOWS} breach witness windows"
    );
    println!(
        "  Pressure checks:        {PRESSURE_COUPONS} coupon pockets plus {PRESSURE_REFERENCE_COUPONS} reference volumes"
    );
    println!(
        "  Quarantine handling:    {DISPOSITION_LANES} disposition lanes with capacity for {QUARANTINE_CAPACITY} cassette tokens"
    );
    println!(
        "  Capture/traceability:   {WASTE_WELLS} waste wells and {CUSTODY_LANDS} barcode/custody lands"
    );
    println!(
        "  Evidence/clearance:     {CAMERA_CLEARANCE_Z:.0}mm camera clearance, {KEEP_OUT_GAUGES} robot/service keepout gauges"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(CASSETTE_POSITIONS, 20);
    assert_eq!(EDGE_POSITIONS + CENTER_POSITIONS, CASSETTE_POSITIONS);
    assert_eq!(edge_position_count(), EDGE_POSITIONS);
    assert_eq!(INJECTION_PORTS, CASSETTE_ROWS + CASSETTE_COLS + 1);
    assert_eq!(WITNESS_WINDOWS, CASSETTE_POSITIONS);
    assert_eq!(
        CUSTODY_LANDS,
        CASSETTE_POSITIONS + PRESSURE_REFERENCE_COUPONS
    );
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert!(NEST_Z > REVC_TOTAL_HEIGHT + 18.0);
    assert!(POSITION_POCKET_Z > REVC_TOTAL_HEIGHT);
    assert!(CAMERA_CLEARANCE_Z >= 110.0);
    assert!(ROBOT_PICK_CLEARANCE_Z > NEST_Z + REVC_TOTAL_HEIGHT + 70.0);
    assert!(SERVICE_PORT_CLEARANCE_X >= 200.0);
    assert!(SERVICE_WASTE_CLEARANCE_X >= 180.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(rect.fits_inside(), "{} exceeds deck envelope", rect.name);
    }

    assert!(!rect_by_name(&rects, "sealed_cassette_nest")
        .overlaps(rect_by_name(&rects, "dye_trace_injection_ports")));
    assert!(!rect_by_name(&rects, "sealed_cassette_nest")
        .overlaps(rect_by_name(&rects, "breach_witness_windows")));
    assert!(!rect_by_name(&rects, "sealed_cassette_nest")
        .overlaps(rect_by_name(&rects, "pressure_decay_coupon_pockets")));
    assert!(!rect_by_name(&rects, "waste_capture_wells")
        .overlaps(rect_by_name(&rects, "quarantine_lane_gate")));
}

fn layout_rects() -> [Rect; 7] {
    [
        Rect {
            name: "sealed_cassette_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "dye_trace_injection_ports",
            center: PORT_CENTER,
            x: PORT_BLOCK_X,
            y: PORT_BLOCK_Y,
        },
        Rect {
            name: "breach_witness_windows",
            center: WITNESS_CENTER,
            x: WITNESS_BANK_X,
            y: WITNESS_BANK_Y,
        },
        Rect {
            name: "pressure_decay_coupon_pockets",
            center: PRESSURE_CENTER,
            x: PRESSURE_BANK_X,
            y: PRESSURE_BANK_Y,
        },
        Rect {
            name: "waste_capture_wells",
            center: WASTE_CENTER,
            x: WASTE_BANK_X,
            y: WASTE_BANK_Y,
        },
        Rect {
            name: "quarantine_lane_gate",
            center: QUARANTINE_CENTER,
            x: QUARANTINE_BANK_X,
            y: QUARANTINE_BANK_Y,
        },
        Rect {
            name: "barcode_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_BOARD_X,
            y: CUSTODY_BOARD_Y,
        },
    ]
}

fn rect_by_name(rects: &[Rect], name: &str) -> Rect {
    *rects
        .iter()
        .find(|rect| rect.name == name)
        .expect("named layout rectangle must exist")
}

fn station_deck() -> Part {
    let deck = centered_cube("closed_barrier_breach_deck_floor", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );

    deck + deck_rims() + deck_zone_labels()
        - insert_sockets()
        - deck_drain_channels()
        - deck_mount_holes()
        - low_point_drain()
}

fn deck_rims() -> Part {
    let rear = centered_cube(
        "closed_barrier_breach_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_barrier_breach_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_barrier_breach_right_containment_rim",
        RIM_W,
        DECK_Y - 120.0,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 34.0, DECK_Z + RIM_Z / 2.0);
    let front_low = centered_cube(
        "closed_barrier_breach_front_low_capture_lip",
        DECK_X - 160.0,
        14.0,
        20.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 - 28.0), DECK_Z + 10.0);

    rear + left + right + front_low
}

fn deck_zone_labels() -> Part {
    let clean = centered_cube(
        "closed_barrier_breach_deck_clean_side_label_land",
        180.0,
        18.0,
        3.0,
    )
    .translate(-350.0, -92.0, DECK_Z + 1.5);
    let dirty = centered_cube(
        "closed_barrier_breach_deck_dirty_side_label_land",
        180.0,
        18.0,
        3.0,
    )
    .translate(350.0, -92.0, DECK_Z + 1.5);
    let quarantine = centered_cube(
        "closed_barrier_breach_deck_quarantine_label_land",
        220.0,
        18.0,
        3.0,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1 + 80.0,
        DECK_Z + 1.5,
    );

    clean + dirty + quarantine
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_barrier_breach_deck_insert_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("closed_barrier_breach_{}_registration_socket", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                INSERT_SOCKET_DEPTH + 0.5,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - INSERT_SOCKET_DEPTH / 2.0 + 0.25,
            );
    }
    sockets
}

fn deck_drain_channels() -> Part {
    let front = centered_cube(
        "closed_barrier_breach_front_drain_channel",
        DECK_X - 132.0,
        DRAIN_CHANNEL_W,
        7.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 - 72.0), DECK_Z - 3.0);
    let nest_to_waste = centered_cube(
        "closed_barrier_breach_nest_to_waste_trace_runoff_channel",
        12.0,
        324.0,
        7.0,
    )
    .rotate(0.0, 0.0, -32.0)
    .translate(250.0, -78.0, DECK_Z - 3.0);
    let witness_to_front = centered_cube(
        "closed_barrier_breach_witness_to_front_capture_channel",
        12.0,
        236.0,
        7.0,
    )
    .translate(WITNESS_CENTER.0 + 112.0, -358.0, DECK_Z - 3.0);

    front + nest_to_waste + witness_to_front
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_barrier_breach_deck_mount_holes");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_barrier_breach_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                32,
            )
            .translate(
                sx * (DECK_X / 2.0 - 50.0),
                sy * (DECK_Y / 2.0 - 48.0),
                DECK_Z / 2.0,
            );
    }
    holes
}

fn low_point_drain() -> Part {
    centered_cylinder("closed_barrier_breach_low_point_waste_drain", 6.0, 54.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(DECK_X / 2.0 - 122.0, -(DECK_Y / 2.0 + 2.0), DECK_Z - 5.0)
}

fn sealed_cassette_nest_grid() -> Part {
    let base = centered_cube(
        "closed_barrier_breach_sealed_cassette_nest_base",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, insert_z(NEST_Z));

    base - cassette_socket_cut() - cassette_position_pockets()
        + cassette_hard_datum_rails()
        + cassette_gasket_ribs()
        + cassette_hold_down_latches()
        + cassette_datum_pins()
        + cassette_flow_trace_index_marks()
}

fn cassette_socket_cut() -> Part {
    centered_cube(
        "closed_barrier_breach_cassette_outer_socket_relief",
        CASSETTE_X + 4.0,
        CASSETTE_Y + 4.0,
        NEST_SOCKET_DEPTH + 1.0,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z + NEST_Z - NEST_SOCKET_DEPTH / 2.0 + 0.5,
    )
}

fn cassette_position_pockets() -> Part {
    let mut pockets = Part::empty("closed_barrier_breach_cassette_position_pockets");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = row * CASSETTE_COLS + col;
            let (x, y) = cassette_position_xy(row, col);
            let pocket = centered_cube(
                format!("closed_barrier_breach_cassette_position_{index}_sealed_chip_relief"),
                REVC_CHIP_LENGTH + 2.2,
                REVC_CHIP_WIDTH + 2.2,
                POSITION_POCKET_Z,
            )
            .translate(
                NEST_CENTER.0 + x,
                NEST_CENTER.1 + y,
                DECK_Z + NEST_Z - POSITION_POCKET_Z / 2.0 + 0.8,
            );
            let witness_notch = centered_cube(
                format!("closed_barrier_breach_cassette_position_{index}_dye_witness_notch"),
                18.0,
                8.0,
                8.0,
            )
            .translate(
                NEST_CENTER.0 + x,
                NEST_CENTER.1 + y - REVC_CHIP_WIDTH / 2.0 - 5.0,
                DECK_Z + NEST_Z - 3.0,
            );
            pockets = pockets + pocket + witness_notch;
        }
    }
    pockets
}

fn cassette_hard_datum_rails() -> Part {
    let left = centered_cube(
        "closed_barrier_breach_left_hard_datum_rail",
        NEST_RAIL_W,
        CASSETTE_Y + 30.0,
        NEST_RAIL_Z,
    )
    .translate(
        NEST_CENTER.0 - CASSETTE_X / 2.0 - NEST_RAIL_W / 2.0,
        NEST_CENTER.1,
        DECK_Z + NEST_Z + NEST_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_barrier_breach_rear_hard_datum_rail",
        CASSETTE_X + 30.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + CASSETTE_Y / 2.0 + NEST_RAIL_W / 2.0,
        DECK_Z + NEST_Z + NEST_RAIL_Z / 2.0,
    );
    let right_soft = centered_cube(
        "closed_barrier_breach_right_soft_capture_rail",
        NEST_RAIL_W,
        CASSETTE_Y * 0.72,
        NEST_RAIL_Z * 0.66,
    )
    .translate(
        NEST_CENTER.0 + CASSETTE_X / 2.0 + NEST_RAIL_W / 2.0,
        NEST_CENTER.1 - 18.0,
        DECK_Z + NEST_Z + NEST_RAIL_Z * 0.33,
    );
    let front_soft = centered_cube(
        "closed_barrier_breach_front_soft_capture_rail",
        CASSETTE_X * 0.68,
        NEST_RAIL_W,
        NEST_RAIL_Z * 0.66,
    )
    .translate(
        NEST_CENTER.0 + 24.0,
        NEST_CENTER.1 - CASSETTE_Y / 2.0 - NEST_RAIL_W / 2.0,
        DECK_Z + NEST_Z + NEST_RAIL_Z * 0.33,
    );

    left + rear + right_soft + front_soft
}

fn cassette_gasket_ribs() -> Part {
    let front = centered_cube(
        "closed_barrier_breach_front_gasket_compression_rib",
        CASSETTE_X + 12.0,
        GASKET_RIB_W,
        GASKET_RIB_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - CASSETTE_Y / 2.0 - 10.0,
        DECK_Z + NEST_Z + GASKET_RIB_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_barrier_breach_rear_gasket_compression_rib",
        CASSETTE_X + 12.0,
        GASKET_RIB_W,
        GASKET_RIB_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + CASSETTE_Y / 2.0 + 10.0,
        DECK_Z + NEST_Z + GASKET_RIB_Z / 2.0,
    );
    let left = centered_cube(
        "closed_barrier_breach_left_gasket_compression_rib",
        GASKET_RIB_W,
        CASSETTE_Y + 12.0,
        GASKET_RIB_Z,
    )
    .translate(
        NEST_CENTER.0 - CASSETTE_X / 2.0 - 10.0,
        NEST_CENTER.1,
        DECK_Z + NEST_Z + GASKET_RIB_Z / 2.0,
    );
    let right = centered_cube(
        "closed_barrier_breach_right_gasket_compression_rib",
        GASKET_RIB_W,
        CASSETTE_Y + 12.0,
        GASKET_RIB_Z,
    )
    .translate(
        NEST_CENTER.0 + CASSETTE_X / 2.0 + 10.0,
        NEST_CENTER.1,
        DECK_Z + NEST_Z + GASKET_RIB_Z / 2.0,
    );

    front + rear + left + right
}

fn cassette_hold_down_latches() -> Part {
    let mut latches = Part::empty("closed_barrier_breach_cassette_hold_down_latches");
    for (i, (x, y, rot)) in [
        (
            NEST_CENTER.0 - CASSETTE_X / 2.0 + 96.0,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 - 22.0,
            0.0,
        ),
        (
            NEST_CENTER.0 + CASSETTE_X / 2.0 - 96.0,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 - 22.0,
            0.0,
        ),
        (
            NEST_CENTER.0 - CASSETTE_X / 2.0 - 22.0,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 - 90.0,
            90.0,
        ),
        (
            NEST_CENTER.0 + CASSETTE_X / 2.0 + 22.0,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 - 90.0,
            90.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        latches = latches
            + centered_cube(
                format!("closed_barrier_breach_toggle_hold_down_latch_{i}"),
                56.0,
                18.0,
                12.0,
            )
            .rotate(0.0, 0.0, *rot)
            .translate(*x, *y, DECK_Z + NEST_Z + 6.0);
    }
    latches
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("closed_barrier_breach_cassette_datum_pins");
    for (i, (x, y)) in [
        (
            NEST_CENTER.0 - CASSETTE_X / 2.0 + 22.0,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 - 24.0,
        ),
        (
            NEST_CENTER.0 + CASSETTE_X / 2.0 - 34.0,
            NEST_CENTER.1 + CASSETTE_Y / 2.0 - 24.0,
        ),
        (
            NEST_CENTER.0 - CASSETTE_X / 2.0 + 22.0,
            NEST_CENTER.1 - CASSETTE_Y / 2.0 + 28.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_barrier_breach_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0,
            12.0,
            32,
        )
        .translate(*x, *y, DECK_Z + NEST_Z + 6.0);
        let pilot = centered_cylinder(
            format!("closed_barrier_breach_datum_pin_pilot_hole_{i}"),
            1.6,
            14.0,
            20,
        )
        .translate(*x, *y, DECK_Z + NEST_Z + 6.0);
        pins = pins + boss - pilot;
    }
    pins
}

fn cassette_flow_trace_index_marks() -> Part {
    let mut marks = Part::empty("closed_barrier_breach_flow_trace_index_marks");
    for row in 0..CASSETTE_ROWS {
        let y = NEST_CENTER.1 + centered_index(row, CASSETTE_ROWS, REVC_CHIP_WIDTH + CHIP_GAP_Y);
        marks = marks
            + centered_cube(
                format!("closed_barrier_breach_row_{row}_trace_index_land"),
                36.0,
                8.0,
                4.0,
            )
            .translate(
                NEST_CENTER.0 + CASSETTE_X / 2.0 - 44.0,
                y,
                DECK_Z + NEST_Z + 2.0,
            );
    }
    for col in 0..CASSETTE_COLS {
        let x = NEST_CENTER.0 + centered_index(col, CASSETTE_COLS, REVC_CHIP_LENGTH + CHIP_GAP_X);
        marks = marks
            + centered_cube(
                format!("closed_barrier_breach_col_{col}_trace_index_land"),
                8.0,
                36.0,
                4.0,
            )
            .translate(
                x,
                NEST_CENTER.1 - CASSETTE_Y / 2.0 + 44.0,
                DECK_Z + NEST_Z + 2.0,
            );
    }
    marks
}

fn dye_trace_injection_ports() -> Part {
    let base = centered_cube(
        "closed_barrier_breach_dye_trace_injection_manifold_body",
        PORT_BLOCK_X,
        PORT_BLOCK_Y,
        PORT_BLOCK_Z,
    )
    .translate(PORT_CENTER.0, PORT_CENTER.1, insert_z(PORT_BLOCK_Z));

    base + port_bosses() + dye_trace_headers() + purge_trap() + manifold_label_lands()
        - port_bores()
        - service_tube_slots()
}

fn port_bosses() -> Part {
    let mut bosses = Part::empty("closed_barrier_breach_dye_trace_port_bosses");
    for i in 0..INJECTION_PORTS {
        let (x, y) = injection_port_xy(i);
        let boss = centered_cylinder(
            format!("closed_barrier_breach_dye_injection_port_boss_{i}"),
            PORT_BOSS_D / 2.0,
            10.0,
            36,
        )
        .translate(
            PORT_CENTER.0 + x,
            PORT_CENTER.1 + y,
            DECK_Z + PORT_BLOCK_Z + 5.0,
        );
        let key = centered_cube(
            format!("closed_barrier_breach_dye_injection_port_{i}_orientation_key"),
            26.0,
            5.0,
            4.0,
        )
        .translate(
            PORT_CENTER.0 + x,
            PORT_CENTER.1 + y - 20.0,
            DECK_Z + PORT_BLOCK_Z + 2.0,
        );
        bosses = bosses + boss + key;
    }
    bosses
}

fn port_bores() -> Part {
    let mut bores = Part::empty("closed_barrier_breach_dye_trace_port_bores");
    for i in 0..INJECTION_PORTS {
        let (x, y) = injection_port_xy(i);
        bores = bores
            + centered_cylinder(
                format!("closed_barrier_breach_dye_injection_port_bore_{i}"),
                PORT_BORE_D / 2.0,
                PORT_BLOCK_Z + 18.0,
                28,
            )
            .translate(PORT_CENTER.0 + x, PORT_CENTER.1 + y, insert_z(PORT_BLOCK_Z));
    }
    bores
}

fn dye_trace_headers() -> Part {
    let row_header = centered_cube(
        "closed_barrier_breach_row_port_dye_trace_header_channel",
        294.0,
        TRACE_CHANNEL_W,
        TRACE_CHANNEL_Z,
    )
    .translate(
        PORT_CENTER.0 - 14.0,
        PORT_CENTER.1 + 40.0,
        DECK_Z + PORT_BLOCK_Z + 3.0,
    );
    let column_header = centered_cube(
        "closed_barrier_breach_column_port_dye_trace_header_channel",
        TRACE_CHANNEL_W,
        146.0,
        TRACE_CHANNEL_Z,
    )
    .translate(
        PORT_CENTER.0 + 142.0,
        PORT_CENTER.1 - 10.0,
        DECK_Z + PORT_BLOCK_Z + 3.0,
    );
    let witness_return = centered_cube(
        "closed_barrier_breach_witness_return_trace_channel",
        260.0,
        TRACE_CHANNEL_W,
        TRACE_CHANNEL_Z,
    )
    .rotate(0.0, 0.0, -21.0)
    .translate(
        PORT_CENTER.0 - 166.0,
        PORT_CENTER.1 - 96.0,
        DECK_Z + PORT_BLOCK_Z + 3.0,
    );

    row_header + column_header + witness_return
}

fn purge_trap() -> Part {
    let trap = centered_cylinder(
        "closed_barrier_breach_dye_trace_purge_bubble_trap_body",
        PURGE_TRAP_D / 2.0,
        18.0,
        40,
    )
    .translate(
        PORT_CENTER.0 - PORT_BLOCK_X / 2.0 + 48.0,
        PORT_CENTER.1 - PORT_BLOCK_Y / 2.0 + 46.0,
        DECK_Z + PORT_BLOCK_Z + 9.0,
    );
    let vent_land = centered_cube(
        "closed_barrier_breach_purge_trap_vent_barcode_land",
        44.0,
        12.0,
        4.0,
    )
    .translate(
        PORT_CENTER.0 - PORT_BLOCK_X / 2.0 + 48.0,
        PORT_CENTER.1 - PORT_BLOCK_Y / 2.0 + 78.0,
        DECK_Z + PORT_BLOCK_Z + 2.0,
    );
    trap + vent_land
}

fn manifold_label_lands() -> Part {
    let mut labels = Part::empty("closed_barrier_breach_dye_trace_manifold_labels");
    for i in 0..INJECTION_PORTS {
        let (x, y) = injection_port_xy(i);
        labels = labels
            + centered_cube(
                format!("closed_barrier_breach_dye_trace_port_{i}_label_land"),
                38.0,
                12.0,
                3.0,
            )
            .translate(
                PORT_CENTER.0 + x,
                PORT_CENTER.1 + y + 23.0,
                DECK_Z + PORT_BLOCK_Z + 1.5,
            );
    }
    labels
}

fn service_tube_slots() -> Part {
    let mut slots = Part::empty("closed_barrier_breach_service_tube_exit_slots");
    for i in 0..4 {
        slots = slots
            + centered_cube(
                format!("closed_barrier_breach_service_tube_exit_slot_{i}"),
                56.0,
                10.0,
                18.0,
            )
            .translate(
                PORT_CENTER.0 - 108.0 + i as f64 * 72.0,
                PORT_CENTER.1 + PORT_BLOCK_Y / 2.0 - 8.0,
                DECK_Z + PORT_BLOCK_Z - 7.0,
            );
    }
    slots
}

fn breach_witness_windows() -> Part {
    let board = centered_cube(
        "closed_barrier_breach_witness_window_bank_body",
        WITNESS_BANK_X,
        WITNESS_BANK_Y,
        WITNESS_BANK_Z,
    )
    .translate(WITNESS_CENTER.0, WITNESS_CENTER.1, insert_z(WITNESS_BANK_Z));

    board + witness_window_frames() + witness_flow_scallops() + witness_fail_reference_bar()
        - witness_window_cutouts()
}

fn witness_window_frames() -> Part {
    let mut frames = Part::empty("closed_barrier_breach_witness_window_frames");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = row * CASSETTE_COLS + col;
            let (x, y) = witness_window_xy(row, col);
            let frame = centered_cube(
                format!("closed_barrier_breach_witness_window_{index}_raised_frame"),
                WINDOW_FRAME_X,
                WINDOW_FRAME_Y,
                5.0,
            )
            .translate(
                WITNESS_CENTER.0 + x,
                WITNESS_CENTER.1 + y,
                DECK_Z + WITNESS_BANK_Z + 2.5,
            );
            let trace_tick = centered_cube(
                format!("closed_barrier_breach_witness_window_{index}_capillary_trace_tick"),
                24.0,
                4.0,
                4.0,
            )
            .translate(
                WITNESS_CENTER.0 + x,
                WITNESS_CENTER.1 + y - WINDOW_FRAME_Y / 2.0 - 6.0,
                DECK_Z + WITNESS_BANK_Z + 2.0,
            );
            frames = frames + frame + trace_tick;
        }
    }
    frames
}

fn witness_window_cutouts() -> Part {
    let mut cutouts = Part::empty("closed_barrier_breach_witness_window_cutouts");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = row * CASSETTE_COLS + col;
            let (x, y) = witness_window_xy(row, col);
            cutouts = cutouts
                + centered_cube(
                    format!("closed_barrier_breach_witness_window_{index}_clear_view_cutout"),
                    WINDOW_CUT_X,
                    WINDOW_CUT_Y,
                    WITNESS_BANK_Z + 8.0,
                )
                .translate(
                    WITNESS_CENTER.0 + x,
                    WITNESS_CENTER.1 + y,
                    insert_z(WITNESS_BANK_Z),
                );
        }
    }
    cutouts
}

fn witness_flow_scallops() -> Part {
    let mut scallops = Part::empty("closed_barrier_breach_witness_flow_scallops");
    for row in 0..CASSETTE_ROWS {
        let y = WITNESS_CENTER.1 + witness_window_xy(row, 0).1;
        scallops = scallops
            + centered_cube(
                format!("closed_barrier_breach_witness_row_{row}_dye_scallop"),
                WITNESS_BANK_X - 34.0,
                5.0,
                4.0,
            )
            .translate(WITNESS_CENTER.0, y, DECK_Z + WITNESS_BANK_Z + 2.0);
    }
    scallops
}

fn witness_fail_reference_bar() -> Part {
    centered_cube(
        "closed_barrier_breach_known_fail_reference_witness_bar",
        WITNESS_BANK_X - 42.0,
        14.0,
        5.0,
    )
    .translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1 - WITNESS_BANK_Y / 2.0 + 20.0,
        DECK_Z + WITNESS_BANK_Z + 2.5,
    )
}

fn quarantine_lane_gate() -> Part {
    let body = centered_cube(
        "closed_barrier_breach_quarantine_lane_gate_body",
        QUARANTINE_BANK_X,
        QUARANTINE_BANK_Y,
        QUARANTINE_BANK_Z,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        insert_z(QUARANTINE_BANK_Z),
    );

    body + quarantine_lane_walls() + quarantine_gate_bar() + quarantine_token_index_lands()
        - quarantine_lane_recesses()
}

fn quarantine_lane_walls() -> Part {
    let mut walls = Part::empty("closed_barrier_breach_quarantine_lane_walls");
    for i in 0..=DISPOSITION_LANES {
        let x = QUARANTINE_CENTER.0 + centered_index(i, DISPOSITION_LANES + 1, LANE_PITCH_X)
            - LANE_PITCH_X / 2.0;
        walls = walls
            + centered_cube(
                format!("closed_barrier_breach_quarantine_lane_wall_{i}"),
                LANE_WALL_W,
                LANE_Y + 28.0,
                26.0,
            )
            .translate(x, QUARANTINE_CENTER.1, DECK_Z + QUARANTINE_BANK_Z + 13.0);
    }
    let rear_stop = centered_cube(
        "closed_barrier_breach_quarantine_rear_gate_stop",
        QUARANTINE_BANK_X - 34.0,
        LANE_WALL_W,
        28.0,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1 + LANE_Y / 2.0 + 11.0,
        DECK_Z + QUARANTINE_BANK_Z + 14.0,
    );
    walls + rear_stop
}

fn quarantine_lane_recesses() -> Part {
    let mut recesses = Part::empty("closed_barrier_breach_quarantine_lane_recesses");
    for lane in 0..DISPOSITION_LANES {
        let x = QUARANTINE_CENTER.0 + centered_index(lane, DISPOSITION_LANES, LANE_PITCH_X);
        recesses = recesses
            + centered_cube(
                format!("closed_barrier_breach_quarantine_lane_{lane}_token_recess"),
                LANE_X,
                LANE_Y,
                10.0,
            )
            .translate(x, QUARANTINE_CENTER.1, DECK_Z + QUARANTINE_BANK_Z - 5.0);
    }
    recesses
}

fn quarantine_gate_bar() -> Part {
    let gate = centered_cube(
        "closed_barrier_breach_sliding_quarantine_gate_bar",
        GATE_BAR_X,
        GATE_BAR_Y,
        16.0,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1 - LANE_Y / 2.0 - 14.0,
        DECK_Z + QUARANTINE_BANK_Z + 8.0,
    );
    let hinge_left = centered_cylinder(
        "closed_barrier_breach_quarantine_gate_left_hinge_pin",
        8.0,
        26.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        QUARANTINE_CENTER.0 - GATE_BAR_X / 2.0 + 18.0,
        QUARANTINE_CENTER.1 - LANE_Y / 2.0 - 14.0,
        DECK_Z + QUARANTINE_BANK_Z + 8.0,
    );
    let hinge_right = centered_cylinder(
        "closed_barrier_breach_quarantine_gate_right_hinge_pin",
        8.0,
        26.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        QUARANTINE_CENTER.0 + GATE_BAR_X / 2.0 - 18.0,
        QUARANTINE_CENTER.1 - LANE_Y / 2.0 - 14.0,
        DECK_Z + QUARANTINE_BANK_Z + 8.0,
    );

    gate + hinge_left + hinge_right
}

fn quarantine_token_index_lands() -> Part {
    let mut lands = Part::empty("closed_barrier_breach_quarantine_token_index_lands");
    for i in 0..QUARANTINE_CAPACITY {
        let lane = i % DISPOSITION_LANES;
        let row = i / DISPOSITION_LANES;
        let x = QUARANTINE_CENTER.0
            + centered_index(lane, DISPOSITION_LANES, LANE_PITCH_X)
            + centered_index(row % 2, 2, 30.0);
        let y = QUARANTINE_CENTER.1 - 28.0 + row as f64 * 10.0;
        lands = lands
            + centered_cube(
                format!("closed_barrier_breach_quarantine_token_land_{i}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                3.0,
            )
            .translate(x, y, DECK_Z + QUARANTINE_BANK_Z + 1.5);
    }
    lands
}

fn pressure_decay_coupon_pockets() -> Part {
    let bank = centered_cube(
        "closed_barrier_breach_pressure_decay_coupon_bank_body",
        PRESSURE_BANK_X,
        PRESSURE_BANK_Y,
        PRESSURE_BANK_Z,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1,
        insert_z(PRESSURE_BANK_Z),
    );

    bank + pressure_reference_volumes() + pressure_luer_bosses() + pressure_coupon_labels()
        - pressure_coupon_recesses()
}

fn pressure_coupon_recesses() -> Part {
    let mut recesses = Part::empty("closed_barrier_breach_pressure_coupon_recesses");
    for i in 0..PRESSURE_COUPONS {
        let (x, y) = pressure_coupon_xy(i);
        recesses = recesses
            + centered_cube(
                format!("closed_barrier_breach_pressure_decay_coupon_pocket_{i}"),
                COUPON_POCKET_X,
                COUPON_POCKET_Y,
                COUPON_POCKET_Z,
            )
            .translate(
                PRESSURE_CENTER.0 + x,
                PRESSURE_CENTER.1 + y,
                DECK_Z + PRESSURE_BANK_Z - COUPON_POCKET_Z / 2.0 + 1.0,
            );
    }
    recesses
}

fn pressure_reference_volumes() -> Part {
    let mut volumes = Part::empty("closed_barrier_breach_pressure_reference_volumes");
    for i in 0..PRESSURE_REFERENCE_COUPONS {
        let x = PRESSURE_CENTER.0 + centered_index(i, PRESSURE_REFERENCE_COUPONS, 56.0);
        let y = PRESSURE_CENTER.1 - PRESSURE_BANK_Y / 2.0 + 42.0;
        volumes = volumes
            + centered_cylinder(
                format!("closed_barrier_breach_pressure_reference_volume_{i}"),
                REFERENCE_VOLUME_D / 2.0,
                18.0,
                34,
            )
            .translate(x, y, DECK_Z + PRESSURE_BANK_Z + 9.0);
    }
    volumes
}

fn pressure_luer_bosses() -> Part {
    let mut bosses = Part::empty("closed_barrier_breach_pressure_decay_luer_bosses");
    for i in 0..PRESSURE_COUPONS {
        let (x, y) = pressure_coupon_xy(i);
        bosses = bosses
            + centered_cylinder(
                format!("closed_barrier_breach_pressure_coupon_{i}_luer_boss"),
                8.0,
                9.0,
                28,
            )
            .translate(
                PRESSURE_CENTER.0 + x + COUPON_POCKET_X / 2.0 - 10.0,
                PRESSURE_CENTER.1 + y + COUPON_POCKET_Y / 2.0 + 12.0,
                DECK_Z + PRESSURE_BANK_Z + 4.5,
            );
    }
    bosses
}

fn pressure_coupon_labels() -> Part {
    let mut labels = Part::empty("closed_barrier_breach_pressure_coupon_labels");
    for i in 0..PRESSURE_COUPONS {
        let (x, y) = pressure_coupon_xy(i);
        labels = labels
            + centered_cube(
                format!("closed_barrier_breach_pressure_coupon_{i}_barcode_label_land"),
                42.0,
                10.0,
                3.0,
            )
            .translate(
                PRESSURE_CENTER.0 + x,
                PRESSURE_CENTER.1 + y - COUPON_POCKET_Y / 2.0 - 12.0,
                DECK_Z + PRESSURE_BANK_Z + 1.5,
            );
    }
    labels
}

fn waste_capture_wells() -> Part {
    let body = centered_cube(
        "closed_barrier_breach_waste_capture_well_bank_body",
        WASTE_BANK_X,
        WASTE_BANK_Y,
        WASTE_BANK_Z,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, insert_z(WASTE_BANK_Z));

    body + waste_overflow_weirs() + waste_well_labels() + waste_secondary_lip()
        - waste_well_cutouts()
        - waste_drain_slots()
}

fn waste_well_cutouts() -> Part {
    let mut cutouts = Part::empty("closed_barrier_breach_waste_capture_well_cutouts");
    for i in 0..WASTE_WELLS {
        let col = i % 3;
        let row = i / 3;
        let x = WASTE_CENTER.0 + centered_index(col, 3, WASTE_PITCH_X);
        let y = WASTE_CENTER.1 + centered_index(row, 2, WASTE_PITCH_Y);
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_barrier_breach_waste_capture_well_{i}_cutout"),
                WASTE_WELL_D / 2.0,
                WASTE_WELL_DEPTH,
                38,
            )
            .translate(x, y, DECK_Z + WASTE_BANK_Z - WASTE_WELL_DEPTH / 2.0 + 1.0);
    }
    cutouts
}

fn waste_overflow_weirs() -> Part {
    let mut weirs = Part::empty("closed_barrier_breach_waste_overflow_weirs");
    for row in 0..2 {
        let y = WASTE_CENTER.1 + centered_index(row, 2, WASTE_PITCH_Y);
        weirs = weirs
            + centered_cube(
                format!("closed_barrier_breach_waste_row_{row}_overflow_weir"),
                252.0,
                8.0,
                8.0,
            )
            .translate(WASTE_CENTER.0, y - 34.0, DECK_Z + WASTE_BANK_Z + 4.0);
    }
    weirs
}

fn waste_well_labels() -> Part {
    let mut labels = Part::empty("closed_barrier_breach_waste_well_barcode_labels");
    for i in 0..WASTE_WELLS {
        let col = i % 3;
        let row = i / 3;
        labels = labels
            + centered_cube(
                format!("closed_barrier_breach_waste_capture_well_{i}_label_land"),
                44.0,
                11.0,
                3.0,
            )
            .translate(
                WASTE_CENTER.0 + centered_index(col, 3, WASTE_PITCH_X),
                WASTE_CENTER.1 + centered_index(row, 2, WASTE_PITCH_Y) + 34.0,
                DECK_Z + WASTE_BANK_Z + 1.5,
            );
    }
    labels
}

fn waste_secondary_lip() -> Part {
    let front = centered_cube(
        "closed_barrier_breach_waste_front_secondary_lip",
        WASTE_BANK_X - 24.0,
        10.0,
        18.0,
    )
    .translate(
        WASTE_CENTER.0,
        WASTE_CENTER.1 - WASTE_BANK_Y / 2.0 + 14.0,
        DECK_Z + WASTE_BANK_Z + 9.0,
    );
    let right = centered_cube(
        "closed_barrier_breach_waste_right_secondary_lip",
        10.0,
        WASTE_BANK_Y - 20.0,
        18.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_BANK_X / 2.0 - 14.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_BANK_Z + 9.0,
    );
    front + right
}

fn waste_drain_slots() -> Part {
    centered_cube(
        "closed_barrier_breach_waste_bank_low_point_drain_slot",
        86.0,
        12.0,
        14.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_BANK_X / 2.0 - 66.0,
        WASTE_CENTER.1 - WASTE_BANK_Y / 2.0 + 20.0,
        DECK_Z + WASTE_BANK_Z - 5.0,
    )
}

fn barcode_custody_lands() -> Part {
    let board = centered_cube(
        "closed_barrier_breach_barcode_custody_board",
        CUSTODY_BOARD_X,
        CUSTODY_BOARD_Y,
        CUSTODY_BOARD_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_BOARD_Z),
    );

    board + barcode_lands() + custody_token_lands() + custody_chain_rail()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_barrier_breach_cassette_barcode_lands");
    for i in 0..CASSETTE_POSITIONS {
        let col = i % 5;
        let row = i / 5;
        lands = lands
            + centered_cube(
                format!("closed_barrier_breach_cassette_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                CUSTODY_CENTER.0 + centered_index(col, 5, 62.0),
                CUSTODY_CENTER.1 + centered_index(row, 4, 28.0) + 20.0,
                DECK_Z + CUSTODY_BOARD_Z + 1.5,
            );
    }
    lands
}

fn custody_token_lands() -> Part {
    let mut lands = Part::empty("closed_barrier_breach_pressure_reference_custody_tokens");
    for i in 0..PRESSURE_REFERENCE_COUPONS {
        lands = lands
            + centered_cylinder(
                format!("closed_barrier_breach_pressure_reference_custody_token_{i}"),
                CUSTODY_TOKEN_D / 2.0,
                4.0,
                26,
            )
            .translate(
                CUSTODY_CENTER.0 + centered_index(i, PRESSURE_REFERENCE_COUPONS, 48.0),
                CUSTODY_CENTER.1 - CUSTODY_BOARD_Y / 2.0 + 28.0,
                DECK_Z + CUSTODY_BOARD_Z + 2.0,
            );
    }
    lands
}

fn custody_chain_rail() -> Part {
    let rail = centered_cube(
        "closed_barrier_breach_custody_chain_clip_rail",
        CUSTODY_BOARD_X - 42.0,
        8.0,
        10.0,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1 + CUSTODY_BOARD_Y / 2.0 - 18.0,
        DECK_Z + CUSTODY_BOARD_Z + 5.0,
    );
    let scanner_datum = centered_cube(
        "closed_barrier_breach_hand_scanner_datum_land",
        74.0,
        32.0,
        5.0,
    )
    .translate(
        CUSTODY_CENTER.0 + CUSTODY_BOARD_X / 2.0 - 54.0,
        CUSTODY_CENTER.1 - CUSTODY_BOARD_Y / 2.0 + 36.0,
        DECK_Z + CUSTODY_BOARD_Z + 2.5,
    );
    rail + scanner_datum
}

fn clean_dirty_segregation() -> Part {
    let wall = centered_cube(
        "closed_barrier_breach_clean_dirty_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_W,
        SEGREGATION_WALL_Z,
    )
    .translate(0.0, SEGREGATION_WALL_Y, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let transfer_gate = centered_cube(
        "closed_barrier_breach_clean_dirty_transfer_gate_cut_marker",
        TRANSFER_GATE_X,
        SEGREGATION_WALL_W + 8.0,
        SEGREGATION_WALL_Z + 2.0,
    )
    .translate(0.0, SEGREGATION_WALL_Y, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let clean = segregation_zone(
        "closed_barrier_breach_clean_unchallenged_cassette_parking_zone",
        CLEAN_ZONE_CENTER,
    );
    let dirty = segregation_zone(
        "closed_barrier_breach_dirty_challenged_cassette_quarantine_zone",
        DIRTY_ZONE_CENTER,
    );
    let gate_rail = centered_cube(
        "closed_barrier_breach_transfer_gate_slide_rail",
        TRANSFER_GATE_X + 42.0,
        8.0,
        14.0,
    )
    .translate(0.0, SEGREGATION_WALL_Y - 22.0, DECK_Z + 7.0);

    wall - transfer_gate + clean + dirty + gate_rail
}

fn segregation_zone(name: &str, center: (f64, f64)) -> Part {
    let tray = centered_cube(format!("{name}_tray"), SEG_ZONE_X, SEG_ZONE_Y, SEG_ZONE_Z).translate(
        center.0,
        center.1,
        insert_z(SEG_ZONE_Z),
    );
    let rear_lip = centered_cube(format!("{name}_rear_lip"), SEG_ZONE_X, 8.0, 18.0).translate(
        center.0,
        center.1 + SEG_ZONE_Y / 2.0 - 8.0,
        DECK_Z + SEG_ZONE_Z + 9.0,
    );
    let left_lip = centered_cube(format!("{name}_left_lip"), 8.0, SEG_ZONE_Y, 18.0).translate(
        center.0 - SEG_ZONE_X / 2.0 + 8.0,
        center.1,
        DECK_Z + SEG_ZONE_Z + 9.0,
    );
    let right_lip = centered_cube(format!("{name}_right_lip"), 8.0, SEG_ZONE_Y, 18.0).translate(
        center.0 + SEG_ZONE_X / 2.0 - 8.0,
        center.1,
        DECK_Z + SEG_ZONE_Z + 9.0,
    );
    tray + rear_lip + left_lip + right_lip
}

fn camera_evidence_bridge() -> Part {
    camera_posts()
        + camera_crossbeam()
        + camera_carriage()
        + camera_light_bars()
        + evidence_scale_strip()
}

fn camera_posts() -> Part {
    let mut posts = Part::empty("closed_barrier_breach_camera_bridge_posts");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_barrier_breach_camera_bridge_post_{i}"),
                CAMERA_POST_W,
                CAMERA_POST_W,
                CAMERA_POST_Z,
            )
            .translate(
                NEST_CENTER.0 + sx * (CAMERA_BRIDGE_X / 2.0 - CAMERA_POST_W / 2.0),
                NEST_CENTER.1 + sy * (NEST_Y / 2.0 + 36.0),
                DECK_Z + CAMERA_POST_Z / 2.0,
            );
    }
    posts
}

fn camera_crossbeam() -> Part {
    let front = centered_cube(
        "closed_barrier_breach_camera_bridge_front_crossbeam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - NEST_Y / 2.0 - 36.0,
        DECK_Z + CAMERA_POST_Z - CAMERA_BRIDGE_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_barrier_breach_camera_bridge_rear_crossbeam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + NEST_Y / 2.0 + 36.0,
        DECK_Z + CAMERA_POST_Z - CAMERA_BRIDGE_Z / 2.0,
    );
    let spine = centered_cube(
        "closed_barrier_breach_camera_bridge_center_spine",
        36.0,
        NEST_Y + 72.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z + CAMERA_POST_Z - CAMERA_BRIDGE_Z / 2.0,
    );
    front + rear + spine
}

fn camera_carriage() -> Part {
    let carriage = centered_cube(
        "closed_barrier_breach_camera_evidence_carriage",
        CAMERA_CARRIAGE_X,
        CAMERA_CARRIAGE_Y,
        CAMERA_CARRIAGE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z + CAMERA_POST_Z - CAMERA_BRIDGE_Z - CAMERA_CARRIAGE_Z / 2.0 - 6.0,
    );
    let lens = centered_cylinder(
        "closed_barrier_breach_camera_lens_clearance_ring",
        CAMERA_LENS_D / 2.0,
        8.0,
        40,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z + CAMERA_POST_Z - CAMERA_BRIDGE_Z - CAMERA_CARRIAGE_Z - 10.0,
    );
    carriage + lens
}

fn camera_light_bars() -> Part {
    let left = centered_cube(
        "closed_barrier_breach_left_oblique_witness_light_bar",
        12.0,
        NEST_Y + 40.0,
        10.0,
    )
    .translate(
        NEST_CENTER.0 - NEST_X / 2.0 - 28.0,
        NEST_CENTER.1,
        DECK_Z + CAMERA_POST_Z - 56.0,
    );
    let right = centered_cube(
        "closed_barrier_breach_right_oblique_witness_light_bar",
        12.0,
        NEST_Y + 40.0,
        10.0,
    )
    .translate(
        NEST_CENTER.0 + NEST_X / 2.0 + 28.0,
        NEST_CENTER.1,
        DECK_Z + CAMERA_POST_Z - 56.0,
    );
    left + right
}

fn evidence_scale_strip() -> Part {
    centered_cube(
        "closed_barrier_breach_camera_evidence_scale_strip",
        NEST_X - 86.0,
        8.0,
        5.0,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - NEST_Y / 2.0 + 28.0,
        DECK_Z + NEST_Z + 5.0,
    )
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "closed_barrier_breach_robot_front_sweep_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);
    let rear = centered_cube(
        "closed_barrier_breach_service_rear_sweep_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);
    let left = centered_cube(
        "closed_barrier_breach_left_robot_side_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);
    let right = centered_cube(
        "closed_barrier_breach_right_service_side_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);
    let vertical_pick = centered_cube(
        "closed_barrier_breach_robot_vertical_pick_clearance_gauge",
        32.0,
        32.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        DECK_Z + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let port_service = centered_cube(
        "closed_barrier_breach_dye_port_service_pull_keepout_gauge",
        SERVICE_PORT_CLEARANCE_X,
        14.0,
        28.0,
    )
    .translate(
        PORT_CENTER.0,
        PORT_CENTER.1 - PORT_BLOCK_Y / 2.0 - 28.0,
        DECK_Z + 14.0,
    );
    let waste_service = centered_cube(
        "closed_barrier_breach_waste_well_service_pull_keepout_gauge",
        SERVICE_WASTE_CLEARANCE_X,
        14.0,
        28.0,
    )
    .translate(
        WASTE_CENTER.0,
        WASTE_CENTER.1 - WASTE_BANK_Y / 2.0 - 24.0,
        DECK_Z + 14.0,
    );

    front + rear + left + right + vertical_pick + port_service + waste_service
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn cassette_position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, CASSETTE_COLS, REVC_CHIP_LENGTH + CHIP_GAP_X),
        centered_index(row, CASSETTE_ROWS, REVC_CHIP_WIDTH + CHIP_GAP_Y),
    )
}

fn witness_window_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, CASSETTE_COLS, WINDOW_PITCH_X),
        centered_index(row, CASSETTE_ROWS, WINDOW_PITCH_Y),
    )
}

fn injection_port_xy(index: usize) -> (f64, f64) {
    if index < CASSETTE_ROWS {
        (
            centered_index(index, CASSETTE_ROWS, PORT_PITCH_X) - 20.0,
            PORT_PITCH_Y / 2.0,
        )
    } else if index < CASSETTE_ROWS + CASSETTE_COLS {
        (
            PORT_BLOCK_X / 2.0 - 58.0,
            centered_index(index - CASSETTE_ROWS, CASSETTE_COLS, PORT_PITCH_Y) - 24.0,
        )
    } else {
        (-PORT_BLOCK_X / 2.0 + 52.0, -PORT_BLOCK_Y / 2.0 + 46.0)
    }
}

fn pressure_coupon_xy(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (
        centered_index(col, 2, COUPON_PITCH_X),
        centered_index(row, 4, COUPON_PITCH_Y) + 24.0,
    )
}

fn is_edge_position(row: usize, col: usize) -> bool {
    row == 0 || row == CASSETTE_ROWS - 1 || col == 0 || col == CASSETTE_COLS - 1
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            if is_edge_position(row, col) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_cassette_barrier_breach_dye_trace_quarantine_station_"
                ),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_groups_match_design_intent() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"sealed_cassette_nest_grid"));
        assert!(REQUIRED_FEATURES.contains(&"dye_trace_injection_ports"));
        assert!(REQUIRED_FEATURES.contains(&"breach_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"quarantine_lane_gate"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_coupon_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"waste_capture_wells"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clean_dirty_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn cassette_map_covers_twenty_positions_with_edge_center_split() {
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(CASSETTE_POSITIONS, 20);
        assert_eq!(edge_position_count(), EDGE_POSITIONS);
        assert_eq!(CENTER_POSITIONS, CASSETTE_POSITIONS - EDGE_POSITIONS);

        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                let (x, y) = cassette_position_xy(row, col);
                assert!(x.abs() + REVC_CHIP_LENGTH / 2.0 < CASSETTE_X / 2.0);
                assert!(y.abs() + REVC_CHIP_WIDTH / 2.0 < CASSETTE_Y / 2.0);
            }
        }
    }

    #[test]
    fn dye_trace_and_witness_counts_cover_cassette_barrier_paths() {
        assert_eq!(INJECTION_PORTS, CASSETTE_ROWS + CASSETTE_COLS + 1);
        assert_eq!(WITNESS_WINDOWS, CASSETTE_POSITIONS);
        assert_eq!(WITNESS_WINDOWS, CASSETTE_ROWS * CASSETTE_COLS);
        assert_eq!(PORT_BOSS_D - PORT_BORE_D, 18.0);
        assert!(TRACE_CHANNEL_W >= 8.0);
    }

    #[test]
    fn quarantine_pressure_waste_and_custody_capacity_are_sized_for_preuse_release() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(QUARANTINE_CAPACITY, CASSETTE_POSITIONS);
        assert_eq!(PRESSURE_COUPONS, 8);
        assert_eq!(PRESSURE_REFERENCE_COUPONS, 4);
        assert_eq!(WASTE_WELLS, 6);
        assert_eq!(
            CUSTODY_LANDS,
            CASSETTE_POSITIONS + PRESSURE_REFERENCE_COUPONS
        );
        assert!(WASTE_WELL_DEPTH < WASTE_BANK_Z);
    }

    #[test]
    fn layout_modules_fit_and_do_not_collide_at_fixture_level() {
        assert_layout();
    }

    #[test]
    fn evidence_bridge_and_robot_keepouts_clear_nest_and_service_paths() {
        assert!(CAMERA_CLEARANCE_Z >= 110.0);
        assert!(ROBOT_PICK_CLEARANCE_Z > NEST_Z + REVC_TOTAL_HEIGHT + 70.0);
        assert!(KEEP_OUT_X <= DECK_X - 70.0);
        assert!(KEEP_OUT_Y <= DECK_Y - 70.0);
        assert_eq!(KEEP_OUT_GAUGES, 6);
    }
}
