use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed module oxygen-tension gradient mapping station.
//
// Intent:
// - Package a closed-module validation fixture for mapping per-chip and
//   per-slot oxygen tension drift, inlet/outlet gradients, optical O2 coupon
//   response, flow-balance restrictor behavior, gasket witness condition, and
//   purge/step-response behavior before cell-culture modules are released.
// - Keep bought oxygen sensors, fibers, gas mixers, restrictors, tubing, and
//   sealed chips as deterministic placeholder envelopes only. The printed CAD
//   is fixture, witness, custody, and evidence-capture geometry.
// - Avoid protocol claims: this is mechanical validation packaging, not an
//   oxygen calibration standard, pressure-rated closed-loop design, biological
//   acceptance protocol, or process release method.

const PREFIX: &str = "closed_module_oxygen_tension_gradient_mapping_station";
const OUTPUT_PREFIX: &str = "output/closed_module_oxygen_tension_gradient_mapping_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_module_oxygen_tension_gradient_mapping_station_containment_deck.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_reference_chip_nest_matrix.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_gas_inlet_outlet_witness_zones.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_optical_oxygen_sensor_coupon_windows.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_flow_balance_restrictor_coupon_rack.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_seal_gasket_witness_lands.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_purge_step_response_path.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_calibration_coupon_dark_storage.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_custody_release_gate_panel.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_evidence_camera_keepout_frame.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_slot_gradient_drift_token_strip.stl",
    "output/closed_module_oxygen_tension_gradient_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "reference_chip_nests",
    "gas_inlet_outlet_witness_zones",
    "optical_oxygen_sensor_coupon_windows",
    "flow_balance_restrictor_coupons",
    "seal_gasket_witness_lands",
    "purge_step_response_path",
    "calibration_coupon_storage",
    "custody_release_gates",
    "per_slot_gradient_drift_mapping",
    "assembly_stl",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_packaging_only",
    "not_an_oxygen_calibration_standard",
    "not_pressure_rated_wetted_design",
    "not_biological_acceptance_protocol",
    "not_process_release_method",
    "external_sensors_gas_mixers_tubing_and_optics",
];

const REPRODUCIBILITY_CONTROLS: [&str; 6] = [
    "fixed_output_manifest",
    "millimeter_units",
    "no_random_inputs",
    "named_deterministic_geometry",
    "static_feature_counts",
    "stable_layout_rectangles",
];

const FORBIDDEN_CLAIM_TERMS: [&str; 6] = [
    "patient",
    "therapy",
    "therapeutic",
    "diagnosis",
    "clinical",
    "sterility assurance",
];

const PARAMETRIC_REVISION: &str = "closed_module_oxygen_tension_gradient_mapping_station_v1";
const UNITS: &str = "millimeters";
const DEFAULT_TESSELLATION_SEGMENTS: u32 = 32;
const LAYOUT_CLEARANCE: f64 = 8.0;

const DECK_X: f64 = 1700.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_Z: f64 = 8.0;
const MOUNT_HOLES: usize = 10;
const DATUM_TARGETS: usize = 6;

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 3;
const CHIP_SLOTS: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_MATRIX_POS: (f64, f64) = (-420.0, 150.0);
const CHIP_MATRIX_X: f64 = 740.0;
const CHIP_MATRIX_Y: f64 = 500.0;
const CHIP_MATRIX_Z: f64 = 46.0;
const CHIP_SLOT_PITCH_X: f64 = 170.0;
const CHIP_SLOT_PITCH_Y: f64 = 145.0;
const CHIP_NEST_X_EXTRA: f64 = 30.0;
const CHIP_NEST_Y_EXTRA: f64 = 28.0;
const CHIP_NEST_DEPTH: f64 = 16.0;
const SLOT_EDGE_POSITIONS: usize = 10;
const SLOT_CENTER_POSITIONS: usize = CHIP_SLOTS - SLOT_EDGE_POSITIONS;
const DRIFT_WELLS_PER_SLOT: usize = 3;
const SLOT_DRIFT_WELLS: usize = CHIP_SLOTS * DRIFT_WELLS_PER_SLOT;
const SLOT_GRADIENT_TICKS_PER_SLOT: usize = 5;

const GAS_POS: (f64, f64) = (390.0, 345.0);
const GAS_PANEL_X: f64 = 560.0;
const GAS_PANEL_Y: f64 = 170.0;
const GAS_PANEL_Z: f64 = 52.0;
const GAS_LANES: usize = CHIP_ROWS;
const GAS_WITNESS_ZONES: usize = GAS_LANES * 2;
const GAS_PORT_RADIUS: f64 = 8.0;
const GAS_LANE_PITCH_Y: f64 = 46.0;
const GAS_GRADIENT_TICKS: usize = 9;

const SENSOR_POS: (f64, f64) = (390.0, 110.0);
const SENSOR_PANEL_X: f64 = 560.0;
const SENSOR_PANEL_Y: f64 = 210.0;
const SENSOR_PANEL_Z: f64 = 38.0;
const SENSOR_WINDOW_COLS: usize = 4;
const SENSOR_WINDOW_ROWS: usize = 4;
const SENSOR_WINDOWS: usize = SENSOR_WINDOW_COLS * SENSOR_WINDOW_ROWS;
const SENSOR_CONTROL_WINDOWS: usize = 4;
const SENSOR_WINDOW_X: f64 = 78.0;
const SENSOR_WINDOW_Y: f64 = 34.0;
const SENSOR_WINDOW_PITCH_X: f64 = 118.0;
const SENSOR_WINDOW_PITCH_Y: f64 = 46.0;
const FIBER_DOCKS_PER_WINDOW: usize = 2;

const RESTRICTOR_POS: (f64, f64) = (500.0, -175.0);
const RESTRICTOR_X: f64 = 360.0;
const RESTRICTOR_Y: f64 = 140.0;
const RESTRICTOR_Z: f64 = 42.0;
const RESTRICTOR_COUPONS: usize = 8;
const RESTRICTOR_PITCH_X: f64 = 40.0;
const RESTRICTOR_SLOT_X: f64 = 28.0;
const RESTRICTOR_SLOT_Y: f64 = 86.0;
const RESTRICTOR_ORIFICE_RADII_MM: [f64; RESTRICTOR_COUPONS] =
    [0.25, 0.32, 0.40, 0.50, 0.63, 0.80, 1.00, 1.25];

const GASKET_POS: (f64, f64) = (-480.0, -405.0);
const GASKET_X: f64 = 520.0;
const GASKET_Y: f64 = 120.0;
const GASKET_Z: f64 = 18.0;
const SLOT_GASKET_LANDS: usize = CHIP_SLOTS;
const MODULE_GASKET_WITNESS_LANDS: usize = 4;
const GASKET_WITNESS_LANDS: usize = SLOT_GASKET_LANDS + MODULE_GASKET_WITNESS_LANDS;

const PURGE_POS: (f64, f64) = (-40.0, -230.0);
const PURGE_X: f64 = 700.0;
const PURGE_Y: f64 = 160.0;
const PURGE_Z: f64 = 40.0;
const PURGE_STEPS: usize = 5;
const PURGE_STEP_LABELS: [&str; PURGE_STEPS] = ["purge", "low_o2", "mid_o2", "normoxic", "high_o2"];
const PURGE_TAPS_PER_STEP: usize = 2;
const PURGE_TAPS: usize = PURGE_STEPS * PURGE_TAPS_PER_STEP;
const STEP_RESPONSE_BAFFLES: usize = 6;

const CAL_POS: (f64, f64) = (130.0, -410.0);
const CAL_X: f64 = 420.0;
const CAL_Y: f64 = 120.0;
const CAL_Z: f64 = 48.0;
const CALIBRATION_COUPONS: usize = 10;
const CALIBRATION_DARK_LID_Z: f64 = 64.0;
const CALIBRATION_COUPON_PITCH_X: f64 = 38.0;

const GATE_POS: (f64, f64) = (565.0, -405.0);
const GATE_X: f64 = 350.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 32.0;
const DISPOSITION_GATES: usize = 3;
const GATE_NAMES: [&str; DISPOSITION_GATES] = ["release", "hold", "reject"];
const BARCODE_LANDS: usize = CHIP_SLOTS;
const CUSTODY_TOKEN_WELLS: usize = CHIP_SLOTS;

const TOKEN_POS: (f64, f64) = (-40.0, -125.0);
const TOKEN_X: f64 = 680.0;
const TOKEN_Y: f64 = 30.0;
const TOKEN_Z: f64 = 16.0;
const SLOT_TOKENS: usize = CHIP_SLOTS;
const DRIFT_RANGE_TOKENS: usize = 7;

const BRIDGE_POS: (f64, f64) = (-20.0, 15.0);
const BRIDGE_X: f64 = 1480.0;
const BRIDGE_Y: f64 = 56.0;
const BRIDGE_Z: f64 = 220.0;
const CAMERA_TARGETS: usize = 5;
const KEEP_OUT_GAUGES: usize = 6;
const ROBOT_FRONT_CLEARANCE: f64 = 340.0;
const REAR_GAS_SERVICE_CLEARANCE: f64 = 250.0;
const OPTICAL_HEAD_CLEARANCE_Z: f64 = 185.0;
const CHIP_LIFT_CLEARANCE_Z: f64 = 145.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 14.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 14.0;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SlotZone {
    Edge,
    Center,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let chip_nests = reference_chip_nest_matrix();
    export(OUTPUTS[1], &chip_nests);

    let gas_witness = gas_inlet_outlet_witness_zones();
    export(OUTPUTS[2], &gas_witness);

    let sensor_windows = optical_oxygen_sensor_coupon_windows();
    export(OUTPUTS[3], &sensor_windows);

    let restrictors = flow_balance_restrictor_coupon_rack();
    export(OUTPUTS[4], &restrictors);

    let gasket_lands = seal_gasket_witness_lands();
    export(OUTPUTS[5], &gasket_lands);

    let purge_path = purge_step_response_path();
    export(OUTPUTS[6], &purge_path);

    let cal_storage = calibration_coupon_dark_storage();
    export(OUTPUTS[7], &cal_storage);

    let gates = custody_release_gate_panel();
    export(OUTPUTS[8], &gates);

    let camera_frame = evidence_camera_keepout_frame();
    export(OUTPUTS[9], &camera_frame);

    let tokens = slot_gradient_drift_token_strip();
    export(OUTPUTS[10], &tokens);

    let assembly = deck
        + chip_nests
        + gas_witness
        + sensor_windows
        + restrictors
        + gasket_lands
        + purge_path
        + cal_storage
        + gates
        + camera_frame
        + tokens;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed module oxygen-tension gradient mapping station:");
    println!(
        "  Mapping deck:          {DECK_X:.0}mm x {DECK_Y:.0}mm closed-module validation deck with {CHIP_SLOTS} reference chip slots"
    );
    println!(
        "  Oxygen witnesses:      {GAS_WITNESS_ZONES} inlet/outlet gas witness zones, {SENSOR_WINDOWS} optical O2 coupon windows, and {SLOT_DRIFT_WELLS} slot drift wells"
    );
    println!(
        "  Flow and seals:        {RESTRICTOR_COUPONS} flow-balance restrictor coupons, {GASKET_WITNESS_LANDS} gasket witness lands, and {PURGE_TAPS} purge/step taps"
    );
    println!(
        "  Traceability:          {CALIBRATION_COUPONS} dark-stored calibration coupons, {DISPOSITION_GATES} custody/release gates, {BARCODE_LANDS} barcode lands, and {} STL outputs",
        OUTPUTS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    DECK_Z + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn chip_nest_x() -> f64 {
    REVC_CHIP_LENGTH + CHIP_NEST_X_EXTRA
}

fn chip_nest_y() -> f64 {
    REVC_CHIP_WIDTH + CHIP_NEST_Y_EXTRA
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "reference_chip_nest_matrix",
            CHIP_MATRIX_POS,
            CHIP_MATRIX_X,
            CHIP_MATRIX_Y,
        ),
        rect(
            "gas_inlet_outlet_witness_zones",
            GAS_POS,
            GAS_PANEL_X,
            GAS_PANEL_Y,
        ),
        rect(
            "optical_oxygen_sensor_coupon_windows",
            SENSOR_POS,
            SENSOR_PANEL_X,
            SENSOR_PANEL_Y,
        ),
        rect(
            "flow_balance_restrictor_coupon_rack",
            RESTRICTOR_POS,
            RESTRICTOR_X,
            RESTRICTOR_Y,
        ),
        rect("seal_gasket_witness_lands", GASKET_POS, GASKET_X, GASKET_Y),
        rect("purge_step_response_path", PURGE_POS, PURGE_X, PURGE_Y),
        rect("calibration_coupon_dark_storage", CAL_POS, CAL_X, CAL_Y),
        rect("custody_release_gate_panel", GATE_POS, GATE_X, GATE_Y),
        rect(
            "slot_gradient_drift_token_strip",
            TOKEN_POS,
            TOKEN_X,
            TOKEN_Y,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(
        OUTPUTS[OUTPUTS.len() - 1],
        format!("{OUTPUT_PREFIX}assembly.stl")
    );
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 6);
    assert_eq!(PARAMETRIC_REVISION, format!("{PREFIX}_v1"));
    assert_eq!(UNITS, "millimeters");
    assert_eq!(CHIP_SLOTS, CHIP_COLS * CHIP_ROWS);
    assert_eq!(SLOT_DRIFT_WELLS, CHIP_SLOTS * DRIFT_WELLS_PER_SLOT);
    assert_eq!(
        SLOT_EDGE_POSITIONS + SLOT_CENTER_POSITIONS,
        CHIP_SLOTS,
        "slot zones must cover every chip nest"
    );
    assert_eq!(SLOT_EDGE_POSITIONS, edge_slot_count());
    assert_eq!(SLOT_CENTER_POSITIONS, center_slot_count());
    assert_eq!(GAS_WITNESS_ZONES, GAS_LANES * 2);
    assert_eq!(SENSOR_WINDOWS, SENSOR_WINDOW_COLS * SENSOR_WINDOW_ROWS);
    assert_eq!(SENSOR_CONTROL_WINDOWS, SENSOR_WINDOWS - CHIP_SLOTS);
    assert_eq!(RESTRICTOR_COUPONS, RESTRICTOR_ORIFICE_RADII_MM.len());
    assert_eq!(
        GASKET_WITNESS_LANDS,
        SLOT_GASKET_LANDS + MODULE_GASKET_WITNESS_LANDS
    );
    assert_eq!(PURGE_STEPS, PURGE_STEP_LABELS.len());
    assert_eq!(PURGE_TAPS, PURGE_STEPS * PURGE_TAPS_PER_STEP);
    assert_eq!(DISPOSITION_GATES, GATE_NAMES.len());
    assert_eq!(BARCODE_LANDS, CHIP_SLOTS);
    assert_eq!(CUSTODY_TOKEN_WELLS, CHIP_SLOTS);
    assert_eq!(MOUNT_HOLES, mount_hole_positions().len());
    assert_eq!(DATUM_TARGETS, datum_positions().len());
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert!(chip_nest_x() > REVC_CHIP_LENGTH);
    assert!(chip_nest_y() > REVC_CHIP_WIDTH);
    assert!(CHIP_SLOT_PITCH_X > chip_nest_x());
    assert!(CHIP_SLOT_PITCH_Y > chip_nest_y());
    assert!(SENSOR_WINDOW_X < SENSOR_WINDOW_PITCH_X);
    assert!(SENSOR_WINDOW_Y < SENSOR_WINDOW_PITCH_Y);
    assert!(RESTRICTOR_ORIFICE_RADII_MM
        .windows(2)
        .all(|pair| pair[0] < pair[1]));
    assert!(ROBOT_FRONT_CLEARANCE >= 320.0);
    assert!(REAR_GAS_SERVICE_CLEARANCE >= 240.0);
    assert!(OPTICAL_HEAD_CLEARANCE_Z > SENSOR_PANEL_Z + DECK_Z);
    assert!(CHIP_LIFT_CLEARANCE_Z > CHIP_MATRIX_Z + DECK_Z);
    assert_no_scope_claim_terms();

    for item in module_rects() {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds deck envelope",
            item.name
        );
    }

    let rects = module_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], LAYOUT_CLEARANCE),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn assert_no_scope_claim_terms() {
    let searchable = format!(
        "{} {} {} {} {}",
        REQUIRED_FEATURES.join(" "),
        LIMITATIONS.join(" "),
        REPRODUCIBILITY_CONTROLS.join(" "),
        OUTPUTS.join(" "),
        PREFIX
    )
    .to_lowercase();
    for term in FORBIDDEN_CLAIM_TERMS {
        assert!(
            !searchable.contains(term),
            "claim term should not be present: {term}"
        );
    }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_deck"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_wipeable_closed_module_sump_recess"),
        DECK_X - 2.0 * (RIM_W + 54.0),
        DECK_Y - 2.0 * (RIM_W + 54.0),
        BASIN_Z,
    )
    .translate(0.0, -6.0, DECK_Z - BASIN_Z / 2.0 + 0.4);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_low_point_drain_placeholder"),
        7.0,
        70.0,
        DEFAULT_TESSELLATION_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 9.0);

    deck - basin - drain - deck_module_sockets() - mounting_holes()
        + perimeter_rims()
        + datum_targets()
        + station_workflow_spines()
}

fn deck_module_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_sockets"));
    for item in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                item.center.0,
                item.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_low_profile_robot_access_rim"),
        DECK_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + 14.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_gas_service_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_chip_custody_rim"),
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_sensor_restrictor_rim"),
        RIM_W,
        DECK_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_deck_mounting_holes"));
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-DECK_X / 2.0 + 62.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 2.0 - 62.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 2.0 + 62.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 62.0),
        (0.0, -DECK_Y / 2.0 + 62.0),
        (0.0, DECK_Y / 2.0 - 62.0),
        (-DECK_X / 2.0 + 62.0, 0.0),
        (DECK_X / 2.0 - 62.0, 0.0),
        (-DECK_X / 4.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 4.0, DECK_Y / 2.0 - 62.0),
    ]
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_robot_and_optics_datum_targets"));
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        targets = targets
            + fiducial_target(&format!("{PREFIX}_datum_target_{i}")).translate(
                *x,
                *y,
                DECK_Z + 2.0,
            );
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGETS] {
    [
        (-DECK_X / 2.0 + 112.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 112.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 112.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 112.0, DECK_Y / 2.0 - 104.0),
        (0.0, -DECK_Y / 2.0 + 104.0),
        (0.0, DECK_Y / 2.0 - 104.0),
    ]
}

fn station_workflow_spines() -> Part {
    let chip_to_purge = centered_cube(
        format!("{PREFIX}_chip_nest_to_purge_workflow_spine"),
        910.0,
        9.0,
        18.0,
    )
    .translate(-235.0, -120.0, DECK_Z + 9.0);
    let gas_to_sensor = centered_cube(
        format!("{PREFIX}_gas_witness_to_optical_coupon_workflow_spine"),
        10.0,
        250.0,
        18.0,
    )
    .translate(95.0, 225.0, DECK_Z + 9.0);
    let evidence_gate_spine = centered_cube(
        format!("{PREFIX}_calibration_to_release_gate_workflow_spine"),
        910.0,
        9.0,
        18.0,
    )
    .translate(125.0, -330.0, DECK_Z + 9.0);
    chip_to_purge + gas_to_sensor + evidence_gate_spine
}

fn reference_chip_nest_matrix() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_reference_chip_nest_matrix_body"),
        CHIP_MATRIX_X,
        CHIP_MATRIX_Y,
        CHIP_MATRIX_Z,
    );
    let mut cuts = Part::empty(format!("{PREFIX}_reference_chip_nest_cuts"));
    let mut rails = Part::empty(format!("{PREFIX}_chip_nest_retainers"));
    let mut drift_wells = Part::empty(format!("{PREFIX}_per_slot_oxygen_drift_wells"));
    let mut gradient_ticks = Part::empty(format!("{PREFIX}_per_slot_gradient_ticks"));

    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let slot = slot_index(col, row);
            let (x, y) = chip_slot_center(col, row);
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_slot_{slot:02}_reference_chip_relief_cut"),
                    chip_nest_x(),
                    chip_nest_y(),
                    CHIP_NEST_DEPTH + 0.8,
                )
                .translate(x, y, CHIP_MATRIX_Z / 2.0 - CHIP_NEST_DEPTH / 2.0 + 0.4);

            rails = rails
                + chip_slot_retainer(
                    &format!("{PREFIX}_slot_{slot:02}_chip_retainer"),
                    chip_nest_x(),
                    chip_nest_y(),
                )
                .translate(x, y, CHIP_MATRIX_Z / 2.0 + 3.0);

            for well in 0..DRIFT_WELLS_PER_SLOT {
                drift_wells = drift_wells
                    + centered_cylinder(
                        format!("{PREFIX}_slot_{slot:02}_drift_coupon_well_{well}"),
                        4.2,
                        5.0,
                        24,
                    )
                    .translate(
                        x + centered_index(well, DRIFT_WELLS_PER_SLOT, 18.0),
                        y - chip_nest_y() / 2.0 + 12.0,
                        CHIP_MATRIX_Z / 2.0 + 2.5,
                    );
            }

            for tick in 0..SLOT_GRADIENT_TICKS_PER_SLOT {
                gradient_ticks = gradient_ticks
                    + centered_cube(
                        format!("{PREFIX}_slot_{slot:02}_gradient_tick_{tick}"),
                        3.5 + tick as f64 * 1.0,
                        12.0,
                        4.0,
                    )
                    .translate(
                        x - chip_nest_x() / 2.0 + 18.0 + tick as f64 * 24.0,
                        y + chip_nest_y() / 2.0 - 12.0,
                        CHIP_MATRIX_Z / 2.0 + 2.0,
                    );
            }
        }
    }

    (body - cuts + rails + drift_wells + gradient_ticks + slot_zone_index_lands()).translate(
        CHIP_MATRIX_POS.0,
        CHIP_MATRIX_POS.1,
        on_deck_z(CHIP_MATRIX_Z),
    )
}

fn chip_slot_retainer(name: &str, x: f64, y: f64) -> Part {
    let front = centered_cube(format!("{name}_front_lip"), x + 16.0, 5.0, 6.0).translate(
        0.0,
        -y / 2.0 - 4.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear_lip"), x + 16.0, 5.0, 6.0).translate(
        0.0,
        y / 2.0 + 4.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left_lip"), 5.0, y + 16.0, 6.0).translate(
        -x / 2.0 - 4.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_lip"), 5.0, y + 16.0, 6.0).translate(
        x / 2.0 + 4.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn slot_zone_index_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_slot_edge_center_zone_index_lands"));
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let slot = slot_index(col, row);
            let (x, y) = chip_slot_center(col, row);
            let (land_x, land_y) = match slot_zone(col, row) {
                SlotZone::Edge => (28.0, 10.0),
                SlotZone::Center => (20.0, 20.0),
            };
            lands = lands
                + centered_cube(
                    format!("{PREFIX}_slot_{slot:02}_zone_index_land"),
                    land_x,
                    land_y,
                    3.0,
                )
                .translate(
                    x + chip_nest_x() / 2.0 - 18.0,
                    y - chip_nest_y() / 2.0 + 16.0,
                    CHIP_MATRIX_Z / 2.0 + 1.5,
                );
        }
    }
    lands
}

fn gas_inlet_outlet_witness_zones() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_gas_inlet_outlet_witness_panel"),
        GAS_PANEL_X,
        GAS_PANEL_Y,
        GAS_PANEL_Z,
    );
    let mut bores = Part::empty(format!("{PREFIX}_gas_witness_port_bores"));
    let mut pads = Part::empty(format!("{PREFIX}_gas_witness_pads"));
    let mut ticks = Part::empty(format!("{PREFIX}_gas_gradient_tick_ladders"));

    for lane in 0..GAS_LANES {
        let y = centered_index(lane, GAS_LANES, GAS_LANE_PITCH_Y);
        for side in 0..2 {
            let x = if side == 0 {
                -GAS_PANEL_X / 2.0 + 64.0
            } else {
                GAS_PANEL_X / 2.0 - 64.0
            };
            let name = if side == 0 { "inlet" } else { "outlet" };
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane}_{name}_gas_witness_port_bore"),
                    GAS_PORT_RADIUS,
                    GAS_PANEL_Z + 4.0,
                    DEFAULT_TESSELLATION_SEGMENTS,
                )
                .translate(x, y, 0.0);
            pads = pads
                + annular_disc(
                    &format!("{PREFIX}_lane_{lane}_{name}_witness_zone_land"),
                    40.0,
                    20.0,
                    5.0,
                )
                .translate(x, y, GAS_PANEL_Z / 2.0 + 2.5);
        }

        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_oxygen_gradient_reference_rail"),
                GAS_PANEL_X - 180.0,
                7.0,
                5.0,
            )
            .translate(0.0, y, GAS_PANEL_Z / 2.0 + 2.5);

        for tick in 0..GAS_GRADIENT_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_gas_gradient_tick_{tick}"),
                    4.0 + tick as f64,
                    18.0,
                    7.0,
                )
                .translate(
                    centered_index(tick, GAS_GRADIENT_TICKS, 38.0),
                    y,
                    GAS_PANEL_Z / 2.0 + 6.5,
                );
        }
    }

    (body - bores + pads + ticks + gas_lane_keying_teeth()).translate(
        GAS_POS.0,
        GAS_POS.1,
        on_deck_z(GAS_PANEL_Z),
    )
}

fn gas_lane_keying_teeth() -> Part {
    let mut teeth = Part::empty(format!("{PREFIX}_gas_lane_keying_teeth"));
    for lane in 0..GAS_LANES {
        teeth = teeth
            + centered_cube(
                format!("{PREFIX}_gas_lane_{lane}_asymmetric_key_tooth"),
                18.0 + lane as f64 * 5.0,
                8.0,
                10.0,
            )
            .translate(
                -GAS_PANEL_X / 2.0 + 26.0,
                centered_index(lane, GAS_LANES, GAS_LANE_PITCH_Y),
                GAS_PANEL_Z / 2.0 + 5.0,
            );
    }
    teeth
}

fn optical_oxygen_sensor_coupon_windows() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_optical_oxygen_coupon_window_panel"),
        SENSOR_PANEL_X,
        SENSOR_PANEL_Y,
        SENSOR_PANEL_Z,
    );
    let mut windows = Part::empty(format!("{PREFIX}_optical_coupon_window_cuts"));
    let mut bezels = Part::empty(format!("{PREFIX}_optical_coupon_bezels"));
    let mut fiber_docks = Part::empty(format!("{PREFIX}_fiber_probe_alignment_docks"));

    for row in 0..SENSOR_WINDOW_ROWS {
        for col in 0..SENSOR_WINDOW_COLS {
            let window = sensor_window_index(col, row);
            let (x, y) = sensor_window_position(col, row);
            windows = windows
                + centered_cube(
                    format!("{PREFIX}_optical_o2_coupon_window_{window:02}_clear_aperture"),
                    SENSOR_WINDOW_X,
                    SENSOR_WINDOW_Y,
                    SENSOR_PANEL_Z + 2.0,
                )
                .translate(x, y, 0.0);
            bezels = bezels
                + gasket_frame_xy(
                    &format!("{PREFIX}_optical_o2_coupon_window_{window:02}_bezel"),
                    SENSOR_WINDOW_X + 16.0,
                    SENSOR_WINDOW_Y + 14.0,
                    5.0,
                    5.0,
                )
                .translate(x, y, SENSOR_PANEL_Z / 2.0 + 2.5);

            for dock in 0..FIBER_DOCKS_PER_WINDOW {
                fiber_docks = fiber_docks
                    + centered_cylinder(
                        format!("{PREFIX}_window_{window:02}_fiber_probe_dock_{dock}"),
                        4.0,
                        10.0,
                        20,
                    )
                    .rotate(90.0, 0.0, 0.0)
                    .translate(
                        x + centered_index(dock, FIBER_DOCKS_PER_WINDOW, 42.0),
                        y + SENSOR_WINDOW_Y / 2.0 + 12.0,
                        SENSOR_PANEL_Z / 2.0 + 6.0,
                    );
            }
        }
    }

    (body - windows + bezels + fiber_docks + optical_control_window_flags()).translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        on_deck_z(SENSOR_PANEL_Z),
    )
}

fn optical_control_window_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_optical_control_window_flags"));
    for control in 0..SENSOR_CONTROL_WINDOWS {
        flags = flags
            + centered_cube(
                format!("{PREFIX}_optical_dark_air_zero_control_flag_{control}"),
                40.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(control, SENSOR_CONTROL_WINDOWS, SENSOR_WINDOW_PITCH_X),
                -SENSOR_PANEL_Y / 2.0 + 13.0,
                SENSOR_PANEL_Z / 2.0 + 2.0,
            );
    }
    flags
}

fn flow_balance_restrictor_coupon_rack() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_flow_balance_restrictor_coupon_rack_body"),
        RESTRICTOR_X,
        RESTRICTOR_Y,
        RESTRICTOR_Z,
    );
    let mut slots = Part::empty(format!("{PREFIX}_restrictor_coupon_slots"));
    let mut orifices = Part::empty(format!("{PREFIX}_restrictor_orifice_witness_bores"));
    let mut keys = Part::empty(format!("{PREFIX}_restrictor_flow_rank_keys"));

    for coupon in 0..RESTRICTOR_COUPONS {
        let x = centered_index(coupon, RESTRICTOR_COUPONS, RESTRICTOR_PITCH_X);
        slots = slots
            + centered_cube(
                format!("{PREFIX}_restrictor_coupon_{coupon}_slot_cut"),
                RESTRICTOR_SLOT_X,
                RESTRICTOR_SLOT_Y,
                RESTRICTOR_Z + 2.0,
            )
            .translate(x, 0.0, 0.0);
        orifices = orifices
            + centered_cylinder(
                format!(
                    "{PREFIX}_restrictor_coupon_{coupon}_orifice_radius_{:.2}_mm_witness",
                    RESTRICTOR_ORIFICE_RADII_MM[coupon]
                ),
                RESTRICTOR_ORIFICE_RADII_MM[coupon] + 1.8,
                RESTRICTOR_SLOT_Y + 6.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 2.0);
        keys = keys
            + centered_cube(
                format!("{PREFIX}_restrictor_coupon_{coupon}_flow_rank_key"),
                20.0,
                7.0,
                4.0 + coupon as f64 * 1.2,
            )
            .translate(
                x,
                -RESTRICTOR_Y / 2.0 + 14.0,
                RESTRICTOR_Z / 2.0 + 2.0 + coupon as f64 * 0.6,
            );
    }

    (body - slots - orifices + keys + restrictor_cover_bosses()).translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1,
        on_deck_z(RESTRICTOR_Z),
    )
}

fn restrictor_cover_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PREFIX}_restrictor_coupon_cover_bosses"));
    for (i, x) in [
        -RESTRICTOR_X / 2.0 + 34.0,
        -RESTRICTOR_X / 6.0,
        RESTRICTOR_X / 6.0,
        RESTRICTOR_X / 2.0 - 34.0,
    ]
    .iter()
    .enumerate()
    {
        bosses = bosses
            + centered_cylinder(format!("{PREFIX}_restrictor_cover_boss_{i}"), 8.0, 7.0, 24)
                .translate(*x, RESTRICTOR_Y / 2.0 - 18.0, RESTRICTOR_Z / 2.0 + 3.5);
    }
    bosses
}

fn seal_gasket_witness_lands() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_seal_gasket_witness_land_plate"),
        GASKET_X,
        GASKET_Y,
        GASKET_Z,
    );
    let mut lands = Part::empty(format!("{PREFIX}_slot_gasket_compression_witness_lands"));
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let slot = slot_index(col, row);
            lands = lands
                + gasket_frame_xy(
                    &format!("{PREFIX}_slot_{slot:02}_gasket_witness_land"),
                    92.0,
                    26.0,
                    4.0,
                    4.0,
                )
                .translate(
                    centered_index(col, CHIP_COLS, 120.0),
                    centered_index(row, CHIP_ROWS, 38.0),
                    GASKET_Z / 2.0 + 2.0,
                );
        }
    }

    (plate + lands + module_gasket_corner_lands()).translate(
        GASKET_POS.0,
        GASKET_POS.1,
        on_deck_z(GASKET_Z),
    )
}

fn module_gasket_corner_lands() -> Part {
    let mut corners = Part::empty(format!("{PREFIX}_module_gasket_corner_witness_lands"));
    for (i, (x, y)) in [
        (-GASKET_X / 2.0 + 30.0, -GASKET_Y / 2.0 + 24.0),
        (GASKET_X / 2.0 - 30.0, -GASKET_Y / 2.0 + 24.0),
        (-GASKET_X / 2.0 + 30.0, GASKET_Y / 2.0 - 24.0),
        (GASKET_X / 2.0 - 30.0, GASKET_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        corners = corners
            + annular_disc(
                &format!("{PREFIX}_module_gasket_corner_land_{i}"),
                30.0,
                12.0,
                5.0,
            )
            .translate(*x, *y, GASKET_Z / 2.0 + 2.5);
    }
    corners
}

fn purge_step_response_path() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_purge_step_response_path_body"),
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    );
    let mut channels = Part::empty(format!("{PREFIX}_purge_step_response_channels"));
    let mut chambers = Part::empty(format!("{PREFIX}_purge_step_response_chambers"));
    let mut taps = Part::empty(format!("{PREFIX}_purge_step_response_taps"));

    channels = channels
        + centered_cube(
            format!("{PREFIX}_purge_step_response_main_serpentine_cut"),
            PURGE_X - 96.0,
            13.0,
            12.0,
        )
        .translate(0.0, 0.0, PURGE_Z / 2.0 - 8.0);

    for step in 0..PURGE_STEPS {
        let x = centered_index(step, PURGE_STEPS, 124.0);
        chambers = chambers
            + annular_disc(
                &format!("{PREFIX}_{}_step_response_chamber", PURGE_STEP_LABELS[step]),
                52.0,
                26.0,
                8.0,
            )
            .translate(x, 0.0, PURGE_Z / 2.0 + 4.0);
        for tap in 0..PURGE_TAPS_PER_STEP {
            taps = taps
                + centered_cylinder(
                    format!(
                        "{PREFIX}_{}_step_response_tap_{tap}",
                        PURGE_STEP_LABELS[step]
                    ),
                    4.0,
                    PURGE_Z + 4.0,
                    20,
                )
                .translate(
                    x + centered_index(tap, PURGE_TAPS_PER_STEP, 32.0),
                    PURGE_Y / 2.0 - 24.0,
                    0.0,
                );
        }
    }

    (body - channels - taps + chambers + purge_baffle_teeth()).translate(
        PURGE_POS.0,
        PURGE_POS.1,
        on_deck_z(PURGE_Z),
    )
}

fn purge_baffle_teeth() -> Part {
    let mut baffles = Part::empty(format!("{PREFIX}_purge_step_response_baffle_teeth"));
    for baffle in 0..STEP_RESPONSE_BAFFLES {
        baffles = baffles
            + centered_cube(
                format!("{PREFIX}_purge_step_response_baffle_{baffle}"),
                7.0,
                54.0,
                12.0,
            )
            .translate(
                centered_index(baffle, STEP_RESPONSE_BAFFLES, 86.0),
                if baffle % 2 == 0 { -24.0 } else { 24.0 },
                PURGE_Z / 2.0 + 6.0,
            );
    }
    baffles
}

fn calibration_coupon_dark_storage() -> Part {
    let tray = centered_cube(
        format!("{PREFIX}_calibration_coupon_dark_storage_tray"),
        CAL_X,
        CAL_Y,
        CAL_Z,
    );
    let mut wells = Part::empty(format!("{PREFIX}_calibration_coupon_storage_wells"));
    for coupon in 0..CALIBRATION_COUPONS {
        wells = wells
            + centered_cube(
                format!("{PREFIX}_calibration_coupon_{coupon}_dark_storage_well"),
                26.0,
                68.0,
                CAL_Z + 2.0,
            )
            .translate(
                centered_index(coupon, CALIBRATION_COUPONS, CALIBRATION_COUPON_PITCH_X),
                0.0,
                0.0,
            );
    }
    (tray - wells + dark_storage_lid_ridge() + calibration_state_lands()).translate(
        CAL_POS.0,
        CAL_POS.1,
        on_deck_z(CAL_Z),
    )
}

fn dark_storage_lid_ridge() -> Part {
    let rear = centered_cube(
        format!("{PREFIX}_calibration_coupon_dark_lid_rear_ridge"),
        CAL_X - 36.0,
        10.0,
        CALIBRATION_DARK_LID_Z,
    )
    .translate(
        0.0,
        CAL_Y / 2.0 - 16.0,
        CAL_Z / 2.0 + CALIBRATION_DARK_LID_Z / 2.0,
    );
    let front = centered_cube(
        format!("{PREFIX}_calibration_coupon_dark_lid_front_ridge"),
        CAL_X - 36.0,
        10.0,
        30.0,
    )
    .translate(0.0, -CAL_Y / 2.0 + 16.0, CAL_Z / 2.0 + 15.0);
    rear + front
}

fn calibration_state_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_calibration_coupon_state_lands"));
    for state in 0..3 {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_calibration_state_land_{state}"),
                82.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(state, 3, 120.0),
                -CAL_Y / 2.0 + 14.0,
                CAL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_release_gate_panel() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_custody_release_gate_panel_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gate_slots = Part::empty(format!("{PREFIX}_release_hold_reject_gate_slots"));
    let mut barcodes = Part::empty(format!("{PREFIX}_slot_barcode_lands"));
    let mut custody = Part::empty(format!("{PREFIX}_custody_token_wells"));

    for gate in 0..DISPOSITION_GATES {
        gate_slots = gate_slots
            + centered_cube(
                format!("{PREFIX}_{}_gate_slot", GATE_NAMES[gate]),
                78.0,
                24.0,
                GATE_Z + 2.0,
            )
            .translate(
                centered_index(gate, DISPOSITION_GATES, 96.0),
                GATE_Y / 2.0 - 28.0,
                0.0,
            );
    }

    for slot in 0..BARCODE_LANDS {
        let col = slot % CHIP_COLS;
        let row = slot / CHIP_COLS;
        barcodes = barcodes
            + centered_cube(
                format!("{PREFIX}_slot_{slot:02}_barcode_land"),
                58.0,
                11.0,
                3.0,
            )
            .translate(
                centered_index(col, CHIP_COLS, 74.0),
                -GATE_Y / 2.0 + 18.0 + row as f64 * 20.0,
                GATE_Z / 2.0 + 1.5,
            );
        custody = custody
            + centered_cylinder(
                format!("{PREFIX}_slot_{slot:02}_custody_token_well"),
                5.2,
                4.0,
                20,
            )
            .translate(
                centered_index(col, CHIP_COLS, 74.0) + 28.0,
                -GATE_Y / 2.0 + 18.0 + row as f64 * 20.0,
                GATE_Z / 2.0 + 2.0,
            );
    }

    (panel - gate_slots + barcodes + custody).translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
}

fn evidence_camera_keepout_frame() -> Part {
    let bridge = centered_cube(
        format!("{PREFIX}_evidence_camera_crossbar"),
        BRIDGE_X,
        BRIDGE_Y,
        18.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z + BRIDGE_Z);
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_camera_left_post"),
        34.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0 + 48.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_camera_right_post"),
        34.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0 - 48.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_Z / 2.0,
    );

    bridge + left_post + right_post + camera_targets() + keepout_gauges()
}

fn camera_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_camera_field_targets"));
    for target in 0..CAMERA_TARGETS {
        targets = targets
            + fiducial_target(&format!("{PREFIX}_camera_field_target_{target}")).translate(
                BRIDGE_POS.0 + centered_index(target, CAMERA_TARGETS, 245.0),
                BRIDGE_POS.1,
                DECK_Z + BRIDGE_Z + 12.0,
            );
    }
    targets
}

fn keepout_gauges() -> Part {
    let mut gauges = Part::empty(format!("{PREFIX}_robot_optical_head_keepout_gauges"));
    for gauge in 0..KEEP_OUT_GAUGES {
        gauges = gauges
            + centered_cube(
                format!("{PREFIX}_keepout_gauge_{gauge}"),
                22.0,
                100.0 + gauge as f64 * 18.0,
                10.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(gauge, KEEP_OUT_GAUGES, 90.0),
                BRIDGE_POS.1 - 125.0,
                DECK_Z + 80.0 + gauge as f64 * 16.0,
            );
    }
    gauges
}

fn slot_gradient_drift_token_strip() -> Part {
    let strip = centered_cube(
        format!("{PREFIX}_slot_gradient_drift_token_strip_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut slot_tokens = Part::empty(format!("{PREFIX}_per_slot_gradient_drift_tokens"));
    for slot in 0..SLOT_TOKENS {
        slot_tokens = slot_tokens
            + centered_cylinder(
                format!("{PREFIX}_slot_{slot:02}_gradient_drift_token_land"),
                8.0,
                5.0,
                24,
            )
            .translate(
                centered_index(slot, SLOT_TOKENS, 46.0),
                -TOKEN_Y / 2.0 + 18.0,
                TOKEN_Z / 2.0 + 2.5,
            );
    }

    let mut range_tokens = Part::empty(format!("{PREFIX}_oxygen_tension_range_tokens"));
    for token in 0..DRIFT_RANGE_TOKENS {
        range_tokens = range_tokens
            + centered_cube(
                format!("{PREFIX}_oxygen_tension_range_token_{token}"),
                34.0,
                11.0 + token as f64 * 2.0,
                4.0,
            )
            .translate(
                centered_index(token, DRIFT_RANGE_TOKENS, 60.0),
                TOKEN_Y / 2.0 - 18.0,
                TOKEN_Z / 2.0 + 2.0,
            );
    }

    (strip + slot_tokens + range_tokens).translate(TOKEN_POS.0, TOKEN_POS.1, on_deck_z(TOKEN_Z))
}

fn slot_index(col: usize, row: usize) -> usize {
    row * CHIP_COLS + col
}

fn slot_zone(col: usize, row: usize) -> SlotZone {
    if col == 0 || row == 0 || col == CHIP_COLS - 1 || row == CHIP_ROWS - 1 {
        SlotZone::Edge
    } else {
        SlotZone::Center
    }
}

fn edge_slot_count() -> usize {
    let mut count = 0;
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            if slot_zone(col, row) == SlotZone::Edge {
                count += 1;
            }
        }
    }
    count
}

fn center_slot_count() -> usize {
    CHIP_SLOTS - edge_slot_count()
}

fn chip_slot_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, CHIP_COLS, CHIP_SLOT_PITCH_X),
        centered_index(row, CHIP_ROWS, CHIP_SLOT_PITCH_Y),
    )
}

fn sensor_window_index(col: usize, row: usize) -> usize {
    row * SENSOR_WINDOW_COLS + col
}

fn sensor_window_position(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, SENSOR_WINDOW_COLS, SENSOR_WINDOW_PITCH_X),
        centered_index(row, SENSOR_WINDOW_ROWS, SENSOR_WINDOW_PITCH_Y),
    )
}

fn gasket_frame_xy(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

fn annular_disc(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(
        format!("{name}_outer"),
        outer_d / 2.0,
        z,
        DEFAULT_TESSELLATION_SEGMENTS,
    ) - centered_cylinder(
        format!("{name}_inner_cut"),
        inner_d / 2.0,
        z + 2.0,
        DEFAULT_TESSELLATION_SEGMENTS,
    )
}

fn fiducial_target(name: &str) -> Part {
    let disc = centered_cylinder(
        format!("{name}_disc"),
        10.0,
        4.0,
        DEFAULT_TESSELLATION_SEGMENTS,
    );
    let slot_x = centered_cube(format!("{name}_slot_x_cut"), 18.0, 2.4, 5.0);
    let slot_y = centered_cube(format!("{name}_slot_y_cut"), 2.4, 18.0, 5.0);
    disc - slot_x - slot_y
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output.starts_with(&format!("output/{PREFIX}_")), "{output}");
            assert!(output.ends_with(".stl"), "{output}");
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_module_oxygen_tension_gradient_mapping_station_containment_deck.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_reference_chip_nest_matrix.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_gas_inlet_outlet_witness_zones.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_optical_oxygen_sensor_coupon_windows.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_flow_balance_restrictor_coupon_rack.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_seal_gasket_witness_lands.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_purge_step_response_path.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_calibration_coupon_dark_storage.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_custody_release_gate_panel.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_evidence_camera_keepout_frame.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_slot_gradient_drift_token_strip.stl",
                "output/closed_module_oxygen_tension_gradient_mapping_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_features_are_explicitly_represented() {
        for feature in [
            "reference_chip_nests",
            "gas_inlet_outlet_witness_zones",
            "optical_oxygen_sensor_coupon_windows",
            "flow_balance_restrictor_coupons",
            "seal_gasket_witness_lands",
            "purge_step_response_path",
            "calibration_coupon_storage",
            "custody_release_gates",
            "per_slot_gradient_drift_mapping",
            "assembly_stl",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn chip_slot_gradient_mapping_capacity_is_pinned() {
        assert_eq!(CHIP_COLS, 4);
        assert_eq!(CHIP_ROWS, 3);
        assert_eq!(CHIP_SLOTS, 12);
        assert_eq!(edge_slot_count(), SLOT_EDGE_POSITIONS);
        assert_eq!(center_slot_count(), SLOT_CENTER_POSITIONS);
        assert_eq!(SLOT_DRIFT_WELLS, 36);
        assert_eq!(SLOT_GRADIENT_TICKS_PER_SLOT, 5);
        assert!(chip_nest_x() > REVC_CHIP_LENGTH);
        assert!(chip_nest_y() > REVC_CHIP_WIDTH);

        for row in 0..CHIP_ROWS {
            for col in 0..CHIP_COLS {
                let (x, y) = chip_slot_center(col, row);
                assert!(x.abs() + chip_nest_x() / 2.0 < CHIP_MATRIX_X / 2.0);
                assert!(y.abs() + chip_nest_y() / 2.0 < CHIP_MATRIX_Y / 2.0);
            }
        }
    }

    #[test]
    fn gas_sensor_restrictor_and_purge_counts_match_station_plan() {
        assert_eq!(GAS_LANES, CHIP_ROWS);
        assert_eq!(GAS_WITNESS_ZONES, 6);
        assert_eq!(SENSOR_WINDOWS, 16);
        assert_eq!(SENSOR_CONTROL_WINDOWS, 4);
        assert_eq!(FIBER_DOCKS_PER_WINDOW, 2);
        assert_eq!(RESTRICTOR_COUPONS, 8);
        assert_eq!(RESTRICTOR_ORIFICE_RADII_MM[0], 0.25);
        assert_eq!(RESTRICTOR_ORIFICE_RADII_MM[RESTRICTOR_COUPONS - 1], 1.25);
        assert_eq!(PURGE_STEPS, 5);
        assert_eq!(PURGE_STEP_LABELS[0], "purge");
        assert_eq!(PURGE_STEP_LABELS[4], "high_o2");
        assert_eq!(PURGE_TAPS, 10);
    }

    #[test]
    fn gasket_calibration_and_custody_capacity_cover_every_slot() {
        assert_eq!(SLOT_GASKET_LANDS, CHIP_SLOTS);
        assert_eq!(MODULE_GASKET_WITNESS_LANDS, 4);
        assert_eq!(GASKET_WITNESS_LANDS, 16);
        assert_eq!(CALIBRATION_COUPONS, 10);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(GATE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(BARCODE_LANDS, CHIP_SLOTS);
        assert_eq!(CUSTODY_TOKEN_WELLS, CHIP_SLOTS);
        assert_eq!(SLOT_TOKENS, CHIP_SLOTS);
        assert_eq!(DRIFT_RANGE_TOKENS, 7);
    }

    #[test]
    fn layout_fits_and_preserves_service_clearances() {
        assert_design_constraints();
        for item in module_rects() {
            assert!(item.fits_inside_deck(), "{item:?} outside deck");
        }
        assert!(ROBOT_FRONT_CLEARANCE >= 340.0);
        assert!(REAR_GAS_SERVICE_CLEARANCE >= 250.0);
        assert!(OPTICAL_HEAD_CLEARANCE_Z > SENSOR_PANEL_Z + DECK_Z);
        assert!(CHIP_LIFT_CLEARANCE_Z > CHIP_MATRIX_Z + DECK_Z);
    }

    #[test]
    fn reproducibility_controls_and_limitations_are_explicit() {
        for control in [
            "fixed_output_manifest",
            "millimeter_units",
            "no_random_inputs",
            "named_deterministic_geometry",
            "static_feature_counts",
            "stable_layout_rectangles",
        ] {
            assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
        }
        assert_eq!(
            PARAMETRIC_REVISION,
            "closed_module_oxygen_tension_gradient_mapping_station_v1"
        );
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
        assert!(LIMITATIONS.contains(&"not_an_oxygen_calibration_standard"));
        assert!(LIMITATIONS.contains(&"not_process_release_method"));
        assert_no_scope_claim_terms();
    }
}
