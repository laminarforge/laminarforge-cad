use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette gasket installation torque/compression station.
//
// Intent:
// - Make cassette gasket installation repeatable before a closed 16-slot chip
//   carrier is released to culture work.
// - Keep gasket lot identity, torque-limited screw sequencing, compression
//   witness gauge blocks, force/height sensor docking, pressure leak-test
//   handoff, reject quarantine, and robot access visibly separated.
// - Model station-level mechanical interfaces only. Acceptance limits, torque
//   recipes, leak pressure setpoints, sterile-barrier claims, and release
//   procedures remain external quality controls.
//
// Research assumptions behind the geometry:
// - Parker face-seal gland guidance gives typical static O-ring squeeze bands
//   around 19-32% for small cross sections. This fixture therefore encodes
//   witness shims around a 25% nominal compression target, with 20% and 30%
//   visible guard steps rather than a hidden single stop.
// - ASTM D395 frames compression set as a rubber property under static
//   compressive stress; the station stores lot scan/retain evidence and
//   compression witness artifacts but does not claim material qualification.
// - ASTM E3244/E3336 single-use-system integrity practice/test-method
//   summaries distinguish pressure-based and tracer-gas methods and call for
//   method validation against the article. This model provides pressure ports,
//   witness strips, and reference-volume placeholders, not an acceptance limit.
// - Fastener torque-auditing practice for medical devices records applied
//   torque plus preload/force evidence because under-torque weakens seals and
//   over-torque can collapse gaskets. This station pairs torque-limited stages
//   with load-washer and height-probe envelopes for traceable installation.

const OUTPUT_PREFIX: &str = "closed_cassette_gasket_install_torque_compression_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_gasket_install_torque_compression_station_base_containment_deck.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_cassette_datum_nest_16_slot.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_torque_limited_screw_stages.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_compression_witness_gauge_bank.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_leak_test_port_manifold.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_force_height_sensor_dock.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_gasket_lot_scan_area.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_reject_chute_quarantine_tray.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_evidence_bridge_and_lighting.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_robot_access_datum_keepouts.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_removable_torque_bits_and_witness_shims.stl",
    "output/closed_cassette_gasket_install_torque_compression_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "cassette_datum_nest_16_slot",
    "torque_limited_screw_stages",
    "compression_witness_gauge_bank",
    "leak_test_port_manifold",
    "force_height_sensor_dock",
    "gasket_lot_scan_area",
    "reject_chute_quarantine_tray",
    "evidence_bridge_and_lighting",
    "robot_access_datum_keepouts",
    "removable_torque_bits_and_witness_shims",
    "named_stl_outputs",
];

const CARRIER_COLS: usize = 4;
const CARRIER_ROWS: usize = 4;
const CARRIER_SLOT_COUNT: usize = CARRIER_COLS * CARRIER_ROWS;
const CHIP_GUTTER_X: f64 = 7.0;
const CHIP_GUTTER_Y: f64 = 7.0;
const CHIP_ARRAY_X: f64 =
    CARRIER_COLS as f64 * REVC_CHIP_LENGTH + (CARRIER_COLS as f64 - 1.0) * CHIP_GUTTER_X;
const CHIP_ARRAY_Y: f64 =
    CARRIER_ROWS as f64 * REVC_CHIP_WIDTH + (CARRIER_ROWS as f64 - 1.0) * CHIP_GUTTER_Y;
const CARRIER_MARGIN_X: f64 = 58.0;
const CARRIER_MARGIN_Y: f64 = 52.0;
const CARRIER_X: f64 = CHIP_ARRAY_X + 2.0 * CARRIER_MARGIN_X;
const CARRIER_Y: f64 = CHIP_ARRAY_Y + 2.0 * CARRIER_MARGIN_Y;
const CARRIER_Z: f64 = REVC_TOTAL_HEIGHT + 32.0;

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;

const NEST_CENTER: (f64, f64) = (-330.0, 128.0);
const NEST_X: f64 = CARRIER_X + 86.0;
const NEST_Y: f64 = CARRIER_Y + 76.0;
const NEST_Z: f64 = 42.0;
const NEST_RAIL_W: f64 = 16.0;
const NEST_RAIL_Z: f64 = 34.0;
const CHIP_POCKET_CLEARANCE: f64 = 3.2;
const CHIP_POCKET_DEPTH: f64 = REVC_TOTAL_HEIGHT + 6.0;
const DATUM_PIN_COUNT: usize = 4;
const DATUM_PIN_D: f64 = 7.0;
const GASKET_LAND_W: f64 = 9.0;
const GASKET_NOMINAL_THICKNESS: f64 = 2.4;
const NOMINAL_SQUEEZE: f64 = 0.25;
const SQUEEZE_LOW: f64 = 0.20;
const SQUEEZE_HIGH: f64 = 0.30;

const TORQUE_CENTER: (f64, f64) = (460.0, 270.0);
const TORQUE_X: f64 = 430.0;
const TORQUE_Y: f64 = 238.0;
const TORQUE_Z: f64 = 48.0;
const TORQUE_STAGE_COUNT: usize = 8;
const TORQUE_STAGE_COLS: usize = 4;
const TORQUE_STAGE_ROWS: usize = 2;
const TORQUE_STAGE_PITCH_X: f64 = 90.0;
const TORQUE_STAGE_PITCH_Y: f64 = 72.0;
const TORQUE_DRIVER_D: f64 = 24.0;
const TORQUE_REACTION_DOGS: usize = TORQUE_STAGE_COUNT;
const TORQUE_SEQUENCE_TOKENS: usize = TORQUE_STAGE_COUNT;

const WITNESS_CENTER: (f64, f64) = (440.0, 42.0);
const WITNESS_X: f64 = 414.0;
const WITNESS_Y: f64 = 174.0;
const WITNESS_Z: f64 = 36.0;
const WITNESS_GAUGE_COUNT: usize = TORQUE_STAGE_COUNT;
const WITNESS_STEP_COUNT: usize = 3;
const WITNESS_STEP_PITCH_X: f64 = 46.0;
const WITNESS_STEP_PITCH_Y: f64 = 38.0;

const LEAK_CENTER: (f64, f64) = (-524.0, -318.0);
const LEAK_X: f64 = 352.0;
const LEAK_Y: f64 = 182.0;
const LEAK_Z: f64 = 42.0;
const LEAK_TEST_PORTS: usize = 4;
const LEAK_REFERENCE_VOLUMES: usize = 3;
const LEAK_WITNESS_STRIPS: usize = 8;
const LEAK_PORT_D: f64 = 8.0;

const SENSOR_CENTER: (f64, f64) = (440.0, -136.0);
const SENSOR_X: f64 = 414.0;
const SENSOR_Y: f64 = 156.0;
const SENSOR_Z: f64 = 40.0;
const FORCE_SENSOR_COUNT: usize = 4;
const HEIGHT_SENSOR_COUNT: usize = 4;
const LOAD_WASHER_D: f64 = 28.0;
const HEIGHT_PROBE_D: f64 = 11.0;

const SCAN_CENTER: (f64, f64) = (-130.0, -318.0);
const SCAN_X: f64 = 344.0;
const SCAN_Y: f64 = 182.0;
const SCAN_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 8;
const RFID_LANDS: usize = 4;
const COA_LANDS: usize = 2;
const LOT_RETAIN_WELLS: usize = 6;

const QUARANTINE_CENTER: (f64, f64) = (318.0, -318.0);
const QUARANTINE_X: f64 = 430.0;
const QUARANTINE_Y: f64 = 182.0;
const QUARANTINE_Z: f64 = 64.0;
const DISPOSITION_LANES: usize = 3;
const QUARANTINE_TOKEN_CAPACITY: usize = CARRIER_SLOT_COUNT;
const REJECT_BIN_COUNT: usize = 2;

const EVIDENCE_CENTER: (f64, f64) = (-330.0, 428.0);
const EVIDENCE_SPAN_X: f64 = 840.0;
const EVIDENCE_BEAM_Y: f64 = 44.0;
const EVIDENCE_BEAM_Z: f64 = 26.0;
const EVIDENCE_POST_Z: f64 = 206.0;
const CAMERA_COUNT: usize = 3;
const LIGHT_BAR_COUNT: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 170.0;

const ROBOT_KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 132.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 74.0;
const LEFT_GRIPPER_CLEARANCE_X: f64 = 118.0;
const RIGHT_SERVICE_CLEARANCE_X: f64 = 116.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 168.0;

const TOOL_SHIM_CENTER: (f64, f64) = (-52.0, 0.0);
const TOOL_BIT_WELLS: usize = TORQUE_STAGE_COUNT;
const WITNESS_SHIM_COUNT: usize = WITNESS_GAUGE_COUNT * WITNESS_STEP_COUNT;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
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

#[derive(Clone, Copy)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let nest = cassette_datum_nest_16_slot();
    export(OUTPUTS[1], &nest);

    let torque = torque_limited_screw_stages();
    export(OUTPUTS[2], &torque);

    let witness = compression_witness_gauge_bank();
    export(OUTPUTS[3], &witness);

    let leak = leak_test_port_manifold();
    export(OUTPUTS[4], &leak);

    let sensors = force_height_sensor_dock();
    export(OUTPUTS[5], &sensors);

    let scan = gasket_lot_scan_area();
    export(OUTPUTS[6], &scan);

    let quarantine = reject_chute_quarantine_tray();
    export(OUTPUTS[7], &quarantine);

    let bridge = evidence_bridge_and_lighting();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_access_datum_keepouts();
    export(OUTPUTS[9], &keepouts);

    let tools = removable_torque_bits_and_witness_shims();
    export(OUTPUTS[10], &tools);

    let assembly = base
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, deck_top_z())
        + torque.translate(TORQUE_CENTER.0, TORQUE_CENTER.1, deck_top_z())
        + witness.translate(WITNESS_CENTER.0, WITNESS_CENTER.1, deck_top_z())
        + leak.translate(LEAK_CENTER.0, LEAK_CENTER.1, deck_top_z())
        + sensors.translate(SENSOR_CENTER.0, SENSOR_CENTER.1, deck_top_z())
        + scan.translate(SCAN_CENTER.0, SCAN_CENTER.1, deck_top_z())
        + quarantine.translate(QUARANTINE_CENTER.0, QUARANTINE_CENTER.1, deck_top_z())
        + bridge.translate(EVIDENCE_CENTER.0, EVIDENCE_CENTER.1, deck_top_z())
        + keepouts
        + tools.translate(TOOL_SHIM_CENTER.0, TOOL_SHIM_CENTER.1, deck_top_z());
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette gasket installation torque/compression station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Cassette datum nest:    {CARRIER_COLS}x{CARRIER_ROWS} chip cassette carrier, {CARRIER_SLOT_COUNT} chip slots, {:.1}mm x {:.1}mm carrier envelope",
        CARRIER_X, CARRIER_Y
    );
    println!(
        "  Torque/compression:     {TORQUE_STAGE_COUNT} torque-limited screw stages, {WITNESS_GAUGE_COUNT} compression witness gauges, {WITNESS_SHIM_COUNT} witness shims"
    );
    println!(
        "  Sensors/leak test:      {FORCE_SENSOR_COUNT} force/load washer pockets, {HEIGHT_SENSOR_COUNT} height probe docks, {LEAK_TEST_PORTS} leak-test ports, {LEAK_REFERENCE_VOLUMES} reference volumes"
    );
    println!(
        "  Traceability:           {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {COA_LANDS} COA lands, {LOT_RETAIN_WELLS} lot retain wells"
    );
    println!(
        "  Disposition/robotics:   release/hold/reject lanes with {QUARANTINE_TOKEN_CAPACITY} cassette-token capacity, {ROBOT_KEEP_OUT_GAUGES} robot keepout gauges, {CAMERA_COUNT} cameras"
    );
    println!("  Required features:      {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z / 2.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(CARRIER_SLOT_COUNT, 16);
    assert_eq!(TORQUE_STAGE_COUNT, TORQUE_STAGE_ROWS * TORQUE_STAGE_COLS);
    assert_eq!(TORQUE_STAGE_COUNT, WITNESS_GAUGE_COUNT);
    assert_eq!(FORCE_SENSOR_COUNT, HEIGHT_SENSOR_COUNT);
    assert_eq!(DATUM_PIN_COUNT, 4);
    assert_eq!(DISPOSITION_LANES, DispositionLane::all().len());
    assert_eq!(WITNESS_SHIM_COUNT, WITNESS_GAUGE_COUNT * WITNESS_STEP_COUNT);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(CARRIER_X > CHIP_ARRAY_X);
    assert!(CARRIER_Y > CHIP_ARRAY_Y);
    assert!(CARRIER_Z > REVC_TOTAL_HEIGHT);
    assert!(target_compressed_height_mm() > 0.0);
    assert!(SQUEEZE_LOW < NOMINAL_SQUEEZE && NOMINAL_SQUEEZE < SQUEEZE_HIGH);
    assert!(ROBOT_PICK_CLEARANCE_Z > CARRIER_Z + BASE_Z + 90.0);
    assert!(CAMERA_CLEARANCE_Z > CARRIER_Z + 110.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside(),
            "{} exceeds station footprint",
            rect.name
        );
    }

    for left_index in 0..rects.len() {
        for right_index in (left_index + 1)..rects.len() {
            assert!(
                !rects[left_index].overlaps(rects[right_index]),
                "{} overlaps {}",
                rects[left_index].name,
                rects[right_index].name
            );
        }
    }
}

fn layout_rects() -> [Rect; 7] {
    [
        rect("cassette datum nest", NEST_CENTER, NEST_X, NEST_Y),
        rect("torque screw stages", TORQUE_CENTER, TORQUE_X, TORQUE_Y),
        rect(
            "compression witness bank",
            WITNESS_CENTER,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect("leak test manifold", LEAK_CENTER, LEAK_X, LEAK_Y),
        rect(
            "force height sensor dock",
            SENSOR_CENTER,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect("gasket lot scan area", SCAN_CENTER, SCAN_X, SCAN_Y),
        rect(
            "reject quarantine tray",
            QUARANTINE_CENTER,
            QUARANTINE_X,
            QUARANTINE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "gasket_install_torque_station_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let recessed_pan = centered_cube(
        "gasket_install_torque_station_recessed_leak_pan",
        STATION_X - 2.0 * (RIM_W + 42.0),
        STATION_Y - 2.0 * (RIM_W + 44.0),
        9.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 4.5);
    let front_gutter = centered_cube(
        "gasket_install_torque_station_front_witness_gutter",
        STATION_X - 170.0,
        30.0,
        10.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 - 5.0);
    let drain = centered_cylinder(
        "gasket_install_torque_station_closed_drain_port",
        10.0,
        52.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 34.0, 0.0);

    deck - recessed_pan - front_gutter - drain - module_sockets() - mount_holes()
        + perimeter_rim()
        + flow_lane_marks()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("gasket_install_torque_station_module_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("gasket_install_torque_station_{}_socket", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("gasket_install_torque_station_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("gasket_install_torque_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 5.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 10] {
    [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-STATION_X / 2.0 + 54.0, 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
        (-STATION_X / 2.0 + 54.0, -230.0),
        (STATION_X / 2.0 - 54.0, -230.0),
    ]
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "gasket_install_torque_station_front_low_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "gasket_install_torque_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "gasket_install_torque_station_left_cart_rim",
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
        "gasket_install_torque_station_right_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn flow_lane_marks() -> Part {
    let lot_to_nest = centered_cube(
        "gasket_install_torque_station_lot_to_nest_flow_land",
        360.0,
        7.0,
        4.0,
    )
    .rotate(0.0, 0.0, 26.0)
    .translate(-270.0, -102.0, BASE_Z / 2.0 + 2.0);
    let nest_to_torque = centered_cube(
        "gasket_install_torque_station_nest_to_torque_flow_land",
        420.0,
        7.0,
        4.0,
    )
    .translate(132.0, 184.0, BASE_Z / 2.0 + 2.0);
    let torque_to_leak = centered_cube(
        "gasket_install_torque_station_torque_to_leak_flow_land",
        520.0,
        7.0,
        4.0,
    )
    .rotate(0.0, 0.0, -34.0)
    .translate(18.0, -112.0, BASE_Z / 2.0 + 2.0);
    let reject_lane = centered_cube(
        "gasket_install_torque_station_reject_flow_land",
        260.0,
        7.0,
        4.0,
    )
    .translate(226.0, -238.0, BASE_Z / 2.0 + 2.0);
    lot_to_nest + nest_to_torque + torque_to_leak + reject_lane
}

fn cassette_datum_nest_16_slot() -> Part {
    let body = centered_cube(
        "gasket_install_cassette_datum_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let carrier_recess = centered_cube(
        "gasket_install_cassette_carrier_recess",
        CARRIER_X + 8.0,
        CARRIER_Y + 8.0,
        CHIP_POCKET_DEPTH,
    )
    .translate(0.0, 0.0, NEST_Z - CHIP_POCKET_DEPTH / 2.0 + 2.0);

    body - carrier_recess - chip_slot_pocket_cuts()
        + carrier_datum_rails()
        + chip_slot_gasket_lands()
        + perimeter_gasket_compression_land()
        + cassette_datum_pins()
        + nest_screw_stage_targets()
        + robot_finger_reliefs()
}

fn chip_slot_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gasket_install_cassette_chip_slot_pocket_cuts");
    for row in 0..CARRIER_ROWS {
        for col in 0..CARRIER_COLS {
            let index = chip_index(row, col);
            let (x, y) = chip_center(row, col);
            cuts = cuts
                + centered_cube(
                    format!("gasket_install_chip_slot_{index:02}_pocket_cut"),
                    REVC_CHIP_LENGTH + CHIP_POCKET_CLEARANCE,
                    REVC_CHIP_WIDTH + CHIP_POCKET_CLEARANCE,
                    CHIP_POCKET_DEPTH + 4.0,
                )
                .translate(x, y, NEST_Z - CHIP_POCKET_DEPTH / 2.0 + 2.0);
        }
    }
    cuts
}

fn carrier_datum_rails() -> Part {
    let front = centered_cube(
        "gasket_install_cassette_front_robot_low_rail",
        CARRIER_X + 22.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(0.0, -CARRIER_Y / 2.0 - 8.0, NEST_Z + NEST_RAIL_Z / 2.0);
    let rear = centered_cube(
        "gasket_install_cassette_rear_fixed_datum_rail",
        CARRIER_X + 22.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(0.0, CARRIER_Y / 2.0 + 8.0, NEST_Z + NEST_RAIL_Z / 2.0);
    let left = centered_cube(
        "gasket_install_cassette_left_fixed_datum_rail",
        NEST_RAIL_W,
        CARRIER_Y + 22.0,
        NEST_RAIL_Z,
    )
    .translate(-CARRIER_X / 2.0 - 8.0, 0.0, NEST_Z + NEST_RAIL_Z / 2.0);
    let right = centered_cube(
        "gasket_install_cassette_right_spring_access_rail",
        NEST_RAIL_W,
        CARRIER_Y + 22.0,
        NEST_RAIL_Z,
    )
    .translate(CARRIER_X / 2.0 + 8.0, 0.0, NEST_Z + NEST_RAIL_Z / 2.0);
    front + rear + left + right
}

fn chip_slot_gasket_lands() -> Part {
    let mut lands = Part::empty("gasket_install_chip_slot_gasket_lands");
    for row in 0..CARRIER_ROWS {
        for col in 0..CARRIER_COLS {
            let index = chip_index(row, col);
            let (x, y) = chip_center(row, col);
            let frame = rectangular_frame(
                &format!("gasket_install_chip_slot_{index:02}_raised_gasket_witness"),
                REVC_CHIP_LENGTH + 12.0,
                REVC_CHIP_WIDTH + 12.0,
                4.0,
                GASKET_LAND_W,
            )
            .translate(x, y, NEST_Z + 2.0);
            let index_land = centered_cube(
                format!("gasket_install_chip_slot_{index:02}_index_land"),
                28.0,
                12.0,
                4.0,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 26.0,
                y + REVC_CHIP_WIDTH / 2.0 - 18.0,
                NEST_Z + 2.0,
            );
            lands = lands + frame + index_land;
        }
    }
    lands
}

fn perimeter_gasket_compression_land() -> Part {
    rectangular_frame(
        "gasket_install_cassette_perimeter_gasket_compression_land",
        CARRIER_X + 34.0,
        CARRIER_Y + 34.0,
        5.0,
        12.0,
    )
    .translate(0.0, 0.0, NEST_Z + 2.5)
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("gasket_install_cassette_datum_pins");
    for (i, (x, y)) in datum_pin_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("gasket_install_cassette_datum_pin_boss_{i}"),
            13.0,
            9.0,
            32,
        )
        .translate(*x, *y, NEST_Z + 4.5);
        let pin = centered_cylinder(
            format!("gasket_install_cassette_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            28.0,
            28,
        )
        .translate(*x, *y, NEST_Z + 14.0);
        pins = pins + boss + pin;
    }
    pins
}

fn nest_screw_stage_targets() -> Part {
    let mut targets = Part::empty("gasket_install_cassette_nest_screw_stage_targets");
    for (i, (x, y)) in carrier_screw_points().iter().enumerate() {
        let outer = centered_cylinder(
            format!("gasket_install_cassette_screw_stage_target_outer_{i}"),
            20.0,
            5.0,
            40,
        );
        let inner = centered_cylinder(
            format!("gasket_install_cassette_screw_stage_target_clearance_{i}"),
            6.0,
            6.0,
            32,
        );
        let sequence_land = centered_cube(
            format!("gasket_install_cassette_screw_stage_sequence_land_{i}"),
            28.0,
            12.0,
            4.0,
        )
        .translate(*x, *y + 24.0, NEST_Z + 2.0);
        targets = targets + (outer - inner).translate(*x, *y, NEST_Z + 2.5) + sequence_land;
    }
    targets
}

fn robot_finger_reliefs() -> Part {
    let left = centered_cube(
        "gasket_install_cassette_left_robot_finger_relief_marker",
        36.0,
        150.0,
        6.0,
    )
    .translate(-CARRIER_X / 2.0 - 34.0, 0.0, NEST_Z + 3.0);
    let right = centered_cube(
        "gasket_install_cassette_right_robot_finger_relief_marker",
        36.0,
        150.0,
        6.0,
    )
    .translate(CARRIER_X / 2.0 + 34.0, 0.0, NEST_Z + 3.0);
    left + right
}

fn torque_limited_screw_stages() -> Part {
    let plate = centered_cube(
        "gasket_install_torque_limited_stage_plate",
        TORQUE_X,
        TORQUE_Y,
        TORQUE_Z,
    )
    .translate(0.0, 0.0, TORQUE_Z / 2.0);
    let top_relief = centered_cube(
        "gasket_install_torque_limited_stage_top_relief",
        TORQUE_X - 34.0,
        TORQUE_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, TORQUE_Z - 4.0);

    plate - top_relief - torque_stage_bores()
        + torque_stage_reaction_dogs()
        + torque_sequence_token_lands()
        + torque_driver_docks()
        + torque_cross_pattern_rail()
}

fn torque_stage_bores() -> Part {
    let mut bores = Part::empty("gasket_install_torque_stage_bores");
    for i in 0..TORQUE_STAGE_COUNT {
        let (x, y) = torque_stage_xy(i);
        let bore = centered_cylinder(
            format!("gasket_install_torque_stage_driver_bore_{i}"),
            TORQUE_DRIVER_D / 2.0,
            TORQUE_Z + 4.0,
            40,
        )
        .translate(x, y, TORQUE_Z / 2.0 + 2.0);
        let bit_slot = centered_cube(
            format!("gasket_install_torque_stage_bit_key_slot_{i}"),
            10.0,
            34.0,
            TORQUE_Z + 4.0,
        )
        .translate(x + 20.0, y, TORQUE_Z / 2.0 + 2.0);
        bores = bores + bore + bit_slot;
    }
    bores
}

fn torque_stage_reaction_dogs() -> Part {
    let mut dogs = Part::empty("gasket_install_torque_stage_reaction_dogs");
    for i in 0..TORQUE_REACTION_DOGS {
        let (x, y) = torque_stage_xy(i);
        let dog = centered_cube(
            format!("gasket_install_torque_stage_reaction_dog_{i}"),
            46.0,
            12.0,
            18.0,
        )
        .translate(x, y - 30.0, TORQUE_Z + 9.0);
        let witness_tick = centered_cube(
            format!("gasket_install_torque_stage_click_witness_tick_{i}"),
            4.0,
            48.0,
            5.0,
        )
        .rotate(0.0, 0.0, -25.0 + i as f64 * 7.0)
        .translate(x, y, TORQUE_Z + 2.5);
        dogs = dogs + dog + witness_tick;
    }
    dogs
}

fn torque_sequence_token_lands() -> Part {
    let mut lands = Part::empty("gasket_install_torque_sequence_token_lands");
    for i in 0..TORQUE_SEQUENCE_TOKENS {
        let (x, y) = torque_stage_xy(i);
        lands = lands
            + centered_cube(
                format!("gasket_install_torque_cross_pattern_token_{i}"),
                30.0,
                14.0,
                5.0,
            )
            .translate(x - 28.0, y + 30.0, TORQUE_Z + 2.5);
    }
    lands
}

fn torque_driver_docks() -> Part {
    let low_range = centered_cylinder(
        "gasket_install_torque_driver_low_range_dock",
        23.0,
        14.0,
        40,
    )
    .translate(TORQUE_X / 2.0 - 54.0, 0.0, TORQUE_Z + 7.0);
    let high_range = centered_cylinder(
        "gasket_install_torque_driver_high_range_dock",
        23.0,
        14.0,
        40,
    )
    .translate(TORQUE_X / 2.0 - 54.0, -58.0, TORQUE_Z + 7.0);
    let calibration_pad = centered_cube(
        "gasket_install_torque_driver_calibration_pad",
        72.0,
        26.0,
        5.0,
    )
    .translate(TORQUE_X / 2.0 - 58.0, 72.0, TORQUE_Z + 2.5);
    low_range + high_range + calibration_pad
}

fn torque_cross_pattern_rail() -> Part {
    let rail = centered_cube(
        "gasket_install_torque_cross_pattern_sequence_rail",
        TORQUE_X - 76.0,
        8.0,
        16.0,
    )
    .translate(-22.0, 0.0, TORQUE_Z + 8.0);
    let left_stop = centered_cube(
        "gasket_install_torque_sequence_left_stop",
        10.0,
        TORQUE_Y - 62.0,
        20.0,
    )
    .translate(-TORQUE_X / 2.0 + 32.0, 0.0, TORQUE_Z + 10.0);
    let right_stop = centered_cube(
        "gasket_install_torque_sequence_right_stop",
        10.0,
        TORQUE_Y - 62.0,
        20.0,
    )
    .translate(TORQUE_X / 2.0 - 108.0, 0.0, TORQUE_Z + 10.0);
    rail + left_stop + right_stop
}

fn compression_witness_gauge_bank() -> Part {
    let bank = centered_cube(
        "gasket_install_compression_witness_gauge_bank",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);
    let tray_recess = centered_cube(
        "gasket_install_compression_witness_gauge_tray_recess",
        WITNESS_X - 30.0,
        WITNESS_Y - 30.0,
        9.0,
    )
    .translate(0.0, 0.0, WITNESS_Z - 4.0);

    bank - tray_recess + witness_step_blocks() + witness_gauge_label_lands()
}

fn witness_step_blocks() -> Part {
    let mut steps = Part::empty("gasket_install_compression_witness_step_blocks");
    for gauge in 0..WITNESS_GAUGE_COUNT {
        let (x, y) = witness_gauge_xy(gauge);
        for (step, squeeze) in [SQUEEZE_LOW, NOMINAL_SQUEEZE, SQUEEZE_HIGH]
            .iter()
            .enumerate()
        {
            let height = compressed_height_for_squeeze(*squeeze) * 5.0;
            steps = steps
                + centered_cube(
                    format!("gasket_install_compression_witness_gauge_{gauge}_step_{step}"),
                    24.0,
                    20.0,
                    height,
                )
                .translate(
                    x + centered_index(step, WITNESS_STEP_COUNT, 26.0),
                    y,
                    WITNESS_Z + height / 2.0,
                );
        }
    }
    steps
}

fn witness_gauge_label_lands() -> Part {
    let mut lands = Part::empty("gasket_install_compression_witness_label_lands");
    for gauge in 0..WITNESS_GAUGE_COUNT {
        let (x, y) = witness_gauge_xy(gauge);
        lands = lands
            + centered_cube(
                format!("gasket_install_compression_witness_gauge_{gauge}_label_land"),
                70.0,
                12.0,
                4.0,
            )
            .translate(x, y + 24.0, WITNESS_Z + 2.0);
    }
    lands
}

fn leak_test_port_manifold() -> Part {
    let block = centered_cube(
        "gasket_install_leak_test_port_manifold_block",
        LEAK_X,
        LEAK_Y,
        LEAK_Z,
    )
    .translate(0.0, 0.0, LEAK_Z / 2.0);
    let trough = centered_cube(
        "gasket_install_leak_test_witness_trough",
        LEAK_X - 44.0,
        42.0,
        14.0,
    )
    .translate(0.0, -LEAK_Y / 2.0 + 50.0, LEAK_Z - 5.0);

    block - trough - leak_port_bores()
        + leak_port_bosses()
        + reference_volume_blocks()
        + leak_witness_strips()
        + pressure_sensor_pad()
}

fn leak_port_bores() -> Part {
    let mut bores = Part::empty("gasket_install_leak_test_port_bores");
    for i in 0..LEAK_TEST_PORTS {
        let x = centered_index(i, LEAK_TEST_PORTS, 64.0);
        bores = bores
            + centered_cylinder(
                format!("gasket_install_leak_test_port_bore_{i}"),
                LEAK_PORT_D / 2.0,
                LEAK_Z + 4.0,
                28,
            )
            .translate(x, 42.0, LEAK_Z / 2.0 + 2.0);
    }
    bores
}

fn leak_port_bosses() -> Part {
    let mut bosses = Part::empty("gasket_install_leak_test_port_bosses");
    for i in 0..LEAK_TEST_PORTS {
        let x = centered_index(i, LEAK_TEST_PORTS, 64.0);
        let boss = centered_cylinder(
            format!("gasket_install_leak_test_port_boss_{i}"),
            18.0,
            12.0,
            36,
        )
        .translate(x, 42.0, LEAK_Z + 6.0);
        let route = centered_cube(
            format!("gasket_install_leak_test_port_{i}_route_witness"),
            10.0,
            76.0,
            5.0,
        )
        .translate(x, -4.0, LEAK_Z + 2.5);
        bosses = bosses + boss + route;
    }
    bosses
}

fn reference_volume_blocks() -> Part {
    let mut volumes = Part::empty("gasket_install_leak_test_reference_volumes");
    for i in 0..LEAK_REFERENCE_VOLUMES {
        volumes = volumes
            + centered_cylinder(
                format!("gasket_install_leak_reference_volume_{i}"),
                22.0,
                30.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(-80.0 + i as f64 * 80.0, LEAK_Y / 2.0 - 32.0, LEAK_Z + 15.0);
    }
    volumes
}

fn leak_witness_strips() -> Part {
    let mut strips = Part::empty("gasket_install_leak_witness_strips");
    for i in 0..LEAK_WITNESS_STRIPS {
        strips = strips
            + centered_cube(
                format!("gasket_install_leak_witness_strip_{i}"),
                28.0,
                68.0,
                3.0,
            )
            .translate(
                centered_index(i, LEAK_WITNESS_STRIPS, 36.0),
                -LEAK_Y / 2.0 + 58.0,
                LEAK_Z + 1.5,
            );
    }
    strips
}

fn pressure_sensor_pad() -> Part {
    let pad = centered_cube("gasket_install_pressure_sensor_pad", 92.0, 42.0, 8.0).translate(
        LEAK_X / 2.0 - 70.0,
        4.0,
        LEAK_Z + 4.0,
    );
    let cable_relief = centered_cube(
        "gasket_install_pressure_sensor_cable_relief_land",
        78.0,
        10.0,
        5.0,
    )
    .translate(LEAK_X / 2.0 - 70.0, -34.0, LEAK_Z + 2.5);
    pad + cable_relief
}

fn force_height_sensor_dock() -> Part {
    let dock = centered_cube(
        "gasket_install_force_height_sensor_dock",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0);
    let top_basin = centered_cube(
        "gasket_install_force_height_sensor_top_basin",
        SENSOR_X - 28.0,
        SENSOR_Y - 28.0,
        9.0,
    )
    .translate(0.0, 0.0, SENSOR_Z - 4.0);

    dock - top_basin - force_sensor_pocket_cuts() - height_probe_pocket_cuts()
        + force_sensor_lands()
        + height_probe_bosses()
        + sensor_cable_comb()
}

fn force_sensor_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gasket_install_force_sensor_pocket_cuts");
    for (i, (x, y)) in sensor_corner_points().iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("gasket_install_load_washer_pocket_cut_{i}"),
                LOAD_WASHER_D / 2.0,
                SENSOR_Z + 4.0,
                40,
            )
            .translate(*x, *y, SENSOR_Z / 2.0 + 2.0);
    }
    cuts
}

fn height_probe_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gasket_install_height_probe_pocket_cuts");
    for (i, (x, y)) in sensor_corner_points().iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("gasket_install_height_probe_bore_{i}"),
                HEIGHT_PROBE_D / 2.0,
                SENSOR_Z + 4.0,
                28,
            )
            .translate(*x + 38.0, *y, SENSOR_Z / 2.0 + 2.0);
    }
    cuts
}

fn force_sensor_lands() -> Part {
    let mut lands = Part::empty("gasket_install_force_sensor_lands");
    for (i, (x, y)) in sensor_corner_points().iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("gasket_install_force_sensor_{i}_id_land"),
                58.0,
                14.0,
                4.0,
            )
            .translate(*x, *y + 28.0, SENSOR_Z + 2.0);
    }
    lands
}

fn height_probe_bosses() -> Part {
    let mut bosses = Part::empty("gasket_install_height_probe_bosses");
    for (i, (x, y)) in sensor_corner_points().iter().enumerate() {
        bosses = bosses
            + centered_cylinder(
                format!("gasket_install_height_probe_boss_{i}"),
                13.0,
                18.0,
                28,
            )
            .translate(*x + 38.0, *y, SENSOR_Z + 9.0);
    }
    bosses
}

fn sensor_cable_comb() -> Part {
    let rail = centered_cube(
        "gasket_install_sensor_cable_comb_rail",
        SENSOR_X - 52.0,
        18.0,
        22.0,
    )
    .translate(0.0, -SENSOR_Y / 2.0 + 24.0, SENSOR_Z + 11.0);
    let mut slots = Part::empty("gasket_install_sensor_cable_comb_slots");
    for i in 0..(FORCE_SENSOR_COUNT + HEIGHT_SENSOR_COUNT) {
        slots = slots
            + centered_cube(
                format!("gasket_install_sensor_cable_comb_slot_{i}"),
                9.0,
                20.0,
                16.0,
            )
            .translate(
                centered_index(i, 8, 32.0),
                -SENSOR_Y / 2.0 + 24.0,
                SENSOR_Z + 11.0,
            );
    }
    rail - slots
}

fn gasket_lot_scan_area() -> Part {
    let panel = centered_cube(
        "gasket_install_lot_scan_traceability_panel",
        SCAN_X,
        SCAN_Y,
        SCAN_Z,
    )
    .translate(0.0, 0.0, SCAN_Z / 2.0);
    panel + barcode_lands() + rfid_lands() + coa_lands() + lot_retain_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("gasket_install_lot_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("gasket_install_lot_barcode_land_{i}"),
                78.0,
                22.0,
                4.0,
            )
            .translate(
                -104.0 + (i % 3) as f64 * 86.0,
                56.0 - (i / 3) as f64 * 34.0,
                SCAN_Z + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("gasket_install_lot_rfid_lands");
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(format!("gasket_install_lot_rfid_land_{i}"), 13.0, 4.0, 28)
                .translate(-132.0 + i as f64 * 38.0, -SCAN_Y / 2.0 + 28.0, SCAN_Z + 2.0);
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("gasket_install_lot_coa_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(format!("gasket_install_lot_coa_land_{i}"), 116.0, 36.0, 4.0)
                .translate(SCAN_X / 2.0 - 72.0, 44.0 - i as f64 * 52.0, SCAN_Z + 2.0);
    }
    lands
}

fn lot_retain_wells() -> Part {
    let mut wells = Part::empty("gasket_install_lot_retain_wells");
    for i in 0..LOT_RETAIN_WELLS {
        let outer = centered_cylinder(
            format!("gasket_install_lot_retain_well_lip_{i}"),
            13.0,
            8.0,
            28,
        );
        let inner = centered_cylinder(
            format!("gasket_install_lot_retain_well_opening_{i}"),
            8.0,
            9.0,
            28,
        );
        wells = wells
            + (outer - inner).translate(
                -116.0 + i as f64 * 42.0,
                -SCAN_Y / 2.0 + 66.0,
                SCAN_Z + 4.0,
            );
    }
    wells
}

fn reject_chute_quarantine_tray() -> Part {
    let tray = centered_cube(
        "gasket_install_reject_quarantine_tray",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0);
    let basin = centered_cube(
        "gasket_install_reject_quarantine_basin",
        QUARANTINE_X - 34.0,
        QUARANTINE_Y - 34.0,
        20.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z - 8.0);

    tray - basin - quarantine_token_recesses()
        + disposition_lane_dividers()
        + reject_chute()
        + quarantine_bin_lips()
        + quarantine_label_lands()
}

fn quarantine_token_recesses() -> Part {
    let mut recesses = Part::empty("gasket_install_quarantine_token_recesses");
    for lane in DispositionLane::all() {
        for slot in 0..(QUARANTINE_TOKEN_CAPACITY / DISPOSITION_LANES + 1) {
            if lane.index() == 2 && slot >= QUARANTINE_TOKEN_CAPACITY / DISPOSITION_LANES {
                continue;
            }
            recesses = recesses
                + centered_cube(
                    format!("gasket_install_{}_lane_token_recess_{slot}", lane.label()),
                    36.0,
                    22.0,
                    QUARANTINE_Z + 4.0,
                )
                .translate(
                    -136.0 + slot as f64 * 50.0,
                    quarantine_lane_y(lane),
                    QUARANTINE_Z / 2.0 + 2.0,
                );
        }
    }
    recesses
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty("gasket_install_quarantine_lane_dividers");
    for i in 0..=DISPOSITION_LANES {
        dividers = dividers
            + centered_cube(
                format!("gasket_install_quarantine_lane_divider_{i}"),
                QUARANTINE_X - 54.0,
                8.0,
                28.0,
            )
            .translate(
                0.0,
                QUARANTINE_Y / 2.0 - 22.0 - i as f64 * 46.0,
                QUARANTINE_Z + 14.0,
            );
    }
    dividers
}

fn reject_chute() -> Part {
    let ramp = centered_cube(
        "gasket_install_reject_chute_sloped_witness_ramp",
        108.0,
        142.0,
        12.0,
    )
    .rotate(0.0, 0.0, -8.0)
    .translate(QUARANTINE_X / 2.0 - 70.0, -12.0, QUARANTINE_Z + 6.0);
    let throat = centered_cube("gasket_install_reject_chute_throat_guard", 92.0, 18.0, 46.0)
        .translate(
            QUARANTINE_X / 2.0 - 70.0,
            QUARANTINE_Y / 2.0 - 32.0,
            QUARANTINE_Z + 23.0,
        );
    ramp + throat
}

fn quarantine_bin_lips() -> Part {
    let mut lips = Part::empty("gasket_install_quarantine_bin_lips");
    for i in 0..REJECT_BIN_COUNT {
        let outer = centered_cube(
            format!("gasket_install_reject_bin_{i}_outer_lip"),
            92.0,
            62.0,
            9.0,
        );
        let inner = centered_cube(
            format!("gasket_install_reject_bin_{i}_inner_opening"),
            70.0,
            42.0,
            10.0,
        );
        lips = lips
            + (outer - inner).translate(
                QUARANTINE_X / 2.0 - 70.0,
                -50.0 - i as f64 * 66.0,
                QUARANTINE_Z + 4.5,
            );
    }
    lips
}

fn quarantine_label_lands() -> Part {
    let mut lands = Part::empty("gasket_install_quarantine_label_lands");
    for lane in DispositionLane::all() {
        lands = lands
            + centered_cube(
                format!("gasket_install_{}_lane_label_land", lane.label()),
                86.0,
                18.0,
                5.0,
            )
            .translate(
                -QUARANTINE_X / 2.0 + 58.0,
                quarantine_lane_y(lane),
                QUARANTINE_Z + 2.5,
            );
    }
    lands
}

fn evidence_bridge_and_lighting() -> Part {
    let left_post = centered_cube(
        "gasket_install_evidence_bridge_left_post",
        28.0,
        EVIDENCE_BEAM_Y,
        EVIDENCE_POST_Z,
    )
    .translate(-EVIDENCE_SPAN_X / 2.0, 0.0, EVIDENCE_POST_Z / 2.0);
    let right_post = centered_cube(
        "gasket_install_evidence_bridge_right_post",
        28.0,
        EVIDENCE_BEAM_Y,
        EVIDENCE_POST_Z,
    )
    .translate(EVIDENCE_SPAN_X / 2.0, 0.0, EVIDENCE_POST_Z / 2.0);
    let beam = centered_cube(
        "gasket_install_evidence_bridge_camera_beam",
        EVIDENCE_SPAN_X + 56.0,
        EVIDENCE_BEAM_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_CLEARANCE_Z);
    left_post + right_post + beam + camera_mount_lands() + light_bar_lands()
}

fn camera_mount_lands() -> Part {
    let mut lands = Part::empty("gasket_install_evidence_camera_mount_lands");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 210.0);
        let mount = centered_cube(
            format!("gasket_install_evidence_camera_mount_{i}"),
            86.0,
            58.0,
            10.0,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - EVIDENCE_BEAM_Z / 2.0 - 5.0);
        let lens = centered_cylinder(
            format!("gasket_install_evidence_camera_lens_clearance_{i}"),
            16.0,
            12.0,
            32,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - EVIDENCE_BEAM_Z / 2.0 - 11.0);
        lands = lands + mount - lens;
    }
    lands
}

fn light_bar_lands() -> Part {
    let mut lands = Part::empty("gasket_install_evidence_light_bar_lands");
    for i in 0..LIGHT_BAR_COUNT {
        lands = lands
            + centered_cube(
                format!("gasket_install_evidence_light_bar_{i}"),
                EVIDENCE_SPAN_X - 120.0,
                12.0,
                8.0,
            )
            .translate(
                0.0,
                -18.0 + i as f64 * 36.0,
                CAMERA_CLEARANCE_Z - EVIDENCE_BEAM_Z / 2.0 - 11.0,
            );
    }
    lands
}

fn robot_access_datum_keepouts() -> Part {
    let front_rail = centered_cube(
        "gasket_install_front_robot_access_keepout_rail",
        STATION_X - 200.0,
        10.0,
        32.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y,
        BASE_Z / 2.0 + 16.0,
    );
    let rear_rail = centered_cube(
        "gasket_install_rear_service_keepout_rail",
        STATION_X - 190.0,
        10.0,
        32.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
        BASE_Z / 2.0 + 16.0,
    );
    let left_rail = centered_cube(
        "gasket_install_left_gripper_clearance_rail",
        10.0,
        STATION_Y - 260.0,
        32.0,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_GRIPPER_CLEARANCE_X,
        -18.0,
        BASE_Z / 2.0 + 16.0,
    );
    let right_rail = centered_cube(
        "gasket_install_right_service_clearance_rail",
        10.0,
        STATION_Y - 260.0,
        32.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SERVICE_CLEARANCE_X,
        -18.0,
        BASE_Z / 2.0 + 16.0,
    );
    let pick_clearance = centered_cube(
        "gasket_install_robot_pick_clearance_bridge",
        STATION_X - 260.0,
        18.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 186.0, ROBOT_PICK_CLEARANCE_Z);
    front_rail + rear_rail + left_rail + right_rail + pick_clearance + robot_datum_targets()
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("gasket_install_robot_datum_targets");
    for (i, (x, y)) in [
        (-660.0, 386.0),
        (-158.0, 386.0),
        (-660.0, -426.0),
        (-158.0, -426.0),
        (328.0, 386.0),
        (660.0, -426.0),
    ]
    .iter()
    .enumerate()
    {
        let outer = centered_cylinder(
            format!("gasket_install_robot_datum_target_outer_{i}"),
            18.0,
            4.0,
            36,
        );
        let inner = centered_cylinder(
            format!("gasket_install_robot_datum_target_inner_{i}"),
            8.0,
            5.0,
            32,
        );
        targets = targets + (outer - inner).translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    targets
}

fn removable_torque_bits_and_witness_shims() -> Part {
    torque_bit_magazine() + loose_witness_shims() + gasket_go_no_go_rings()
}

fn torque_bit_magazine() -> Part {
    let body = centered_cube("gasket_install_torque_bit_magazine", 246.0, 54.0, 42.0)
        .translate(-64.0, 0.0, 21.0);
    let mut wells = Part::empty("gasket_install_torque_bit_magazine_wells");
    for i in 0..TOOL_BIT_WELLS {
        wells = wells
            + centered_cylinder(format!("gasket_install_torque_bit_well_{i}"), 8.0, 46.0, 24)
                .translate(-152.0 + i as f64 * 24.0, 0.0, 23.0);
    }
    body - wells
}

fn loose_witness_shims() -> Part {
    let mut shims = Part::empty("gasket_install_loose_witness_shims");
    for gauge in 0..WITNESS_GAUGE_COUNT {
        for step in 0..WITNESS_STEP_COUNT {
            let squeeze = [SQUEEZE_LOW, NOMINAL_SQUEEZE, SQUEEZE_HIGH][step];
            let height = compressed_height_for_squeeze(squeeze);
            shims = shims
                + centered_cube(
                    format!("gasket_install_witness_shim_g{gauge}_s{step}"),
                    20.0,
                    30.0,
                    height,
                )
                .translate(
                    -144.0 + gauge as f64 * 30.0,
                    -74.0 + step as f64 * 26.0,
                    height / 2.0,
                );
        }
    }
    shims
}

fn gasket_go_no_go_rings() -> Part {
    let nominal = CARRIER_X.min(CARRIER_Y) / 7.8;
    let go_outer = centered_cylinder("gasket_install_go_ring_outer", nominal / 2.0 + 2.0, 4.0, 64);
    let go_inner = centered_cylinder("gasket_install_go_ring_inner", nominal / 2.0 - 8.0, 5.0, 64);
    let nogo_outer = centered_cylinder(
        "gasket_install_no_go_ring_outer",
        nominal / 2.0 - 2.0,
        4.0,
        64,
    );
    let nogo_inner = centered_cylinder(
        "gasket_install_no_go_ring_inner",
        nominal / 2.0 - 5.0,
        5.0,
        64,
    );
    (go_outer - go_inner).translate(142.0, 34.0, 2.0)
        + (nogo_outer - nogo_inner).translate(214.0, 34.0, 2.0)
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn chip_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, CARRIER_COLS, REVC_CHIP_LENGTH + CHIP_GUTTER_X),
        centered_index(row, CARRIER_ROWS, REVC_CHIP_WIDTH + CHIP_GUTTER_Y),
    )
}

fn chip_index(row: usize, col: usize) -> usize {
    row * CARRIER_COLS + col
}

fn datum_pin_points() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-CARRIER_X / 2.0 - 26.0, -CARRIER_Y / 2.0 - 18.0),
        (CARRIER_X / 2.0 + 26.0, -CARRIER_Y / 2.0 - 18.0),
        (-CARRIER_X / 2.0 - 26.0, CARRIER_Y / 2.0 + 18.0),
        (CARRIER_X / 2.0 + 26.0, CARRIER_Y / 2.0 + 18.0),
    ]
}

fn carrier_screw_points() -> [(f64, f64); TORQUE_STAGE_COUNT] {
    let x_outer = CARRIER_X / 2.0 + 32.0;
    let y_outer = CARRIER_Y / 2.0 + 28.0;
    [
        (-x_outer, -y_outer),
        (0.0, -y_outer),
        (x_outer, -y_outer),
        (x_outer, 0.0),
        (x_outer, y_outer),
        (0.0, y_outer),
        (-x_outer, y_outer),
        (-x_outer, 0.0),
    ]
}

fn torque_stage_xy(index: usize) -> (f64, f64) {
    (
        centered_index(
            index % TORQUE_STAGE_COLS,
            TORQUE_STAGE_COLS,
            TORQUE_STAGE_PITCH_X,
        ) - 30.0,
        centered_index(
            index / TORQUE_STAGE_COLS,
            TORQUE_STAGE_ROWS,
            TORQUE_STAGE_PITCH_Y,
        ),
    )
}

fn witness_gauge_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % 4, 4, WITNESS_STEP_PITCH_X * 2.0),
        centered_index(index / 4, 2, WITNESS_STEP_PITCH_Y * 2.0),
    )
}

fn sensor_corner_points() -> [(f64, f64); FORCE_SENSOR_COUNT] {
    [
        (-132.0, -42.0),
        (132.0, -42.0),
        (-132.0, 42.0),
        (132.0, 42.0),
    ]
}

fn quarantine_lane_y(lane: DispositionLane) -> f64 {
    QUARANTINE_Y / 2.0 - 38.0 - lane.index() as f64 * 46.0
}

fn target_compressed_height_mm() -> f64 {
    compressed_height_for_squeeze(NOMINAL_SQUEEZE)
}

fn compressed_height_for_squeeze(squeeze: f64) -> f64 {
    GASKET_NOMINAL_THICKNESS * (1.0 - squeeze)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_scoped_and_stable() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn required_feature_list_covers_user_requested_station() {
        for feature in [
            "cassette_datum_nest_16_slot",
            "torque_limited_screw_stages",
            "compression_witness_gauge_bank",
            "leak_test_port_manifold",
            "force_height_sensor_dock",
            "gasket_lot_scan_area",
            "reject_chute_quarantine_tray",
            "robot_access_datum_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn carrier_and_station_counts_are_deterministic() {
        assert_eq!(CARRIER_SLOT_COUNT, 16);
        assert_eq!(CARRIER_ROWS, 4);
        assert_eq!(CARRIER_COLS, 4);
        assert_eq!(TORQUE_STAGE_COUNT, 8);
        assert_eq!(WITNESS_GAUGE_COUNT, TORQUE_STAGE_COUNT);
        assert_eq!(WITNESS_SHIM_COUNT, 24);
        assert_eq!(FORCE_SENSOR_COUNT, 4);
        assert_eq!(HEIGHT_SENSOR_COUNT, 4);
        assert_eq!(LEAK_TEST_PORTS, 4);
        assert_eq!(BARCODE_LANDS + RFID_LANDS + COA_LANDS, 14);
    }

    #[test]
    fn cassette_envelope_wraps_revc_chip_array() {
        assert!(CARRIER_X > CHIP_ARRAY_X + 100.0);
        assert!(CARRIER_Y > CHIP_ARRAY_Y + 96.0);
        assert!(CARRIER_Z > REVC_TOTAL_HEIGHT + 30.0);
        for row in 0..CARRIER_ROWS {
            for col in 0..CARRIER_COLS {
                let (x, y) = chip_center(row, col);
                assert!(x.abs() + REVC_CHIP_LENGTH / 2.0 < CARRIER_X / 2.0);
                assert!(y.abs() + REVC_CHIP_WIDTH / 2.0 < CARRIER_Y / 2.0);
            }
        }
    }

    #[test]
    fn squeeze_witnesses_encode_guard_band() {
        assert!(SQUEEZE_LOW < NOMINAL_SQUEEZE);
        assert!(NOMINAL_SQUEEZE < SQUEEZE_HIGH);
        assert!(compressed_height_for_squeeze(SQUEEZE_LOW) > target_compressed_height_mm());
        assert!(target_compressed_height_mm() > compressed_height_for_squeeze(SQUEEZE_HIGH));
        assert!((target_compressed_height_mm() - 1.8).abs() < 0.001);
    }

    #[test]
    fn layout_is_bounded_and_non_overlapping() {
        assert_design_constraints();
    }
}
