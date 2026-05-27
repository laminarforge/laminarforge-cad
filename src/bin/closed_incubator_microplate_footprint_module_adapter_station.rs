use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator microplate-footprint module adapter station.
//
// Intent:
// - Provide a no-cell validation fixture for ANSI/SLAS-style microplate
//   footprint modules moving through the LaminarForge closed tissue-chip
//   incubator and perfusion handoff path.
// - Represent fixed footprint datums, swappable module nests, blind-mate
//   fluidic/electrical connector gauges, port witness lanes, leak/pressure
//   witness pockets, custody/status surfaces, robot gripper approach gauges,
//   and closed handoff features for an incubator rack/perfusion manifold.
//
// This is validation-fixture CAD only. It models mechanical datums, gauge
// envelopes, witness surfaces, and traceability lands, not clinical acceptance
// thresholds, live-cell criteria, or release specifications.

const OUTPUT_PREFIX: &str = "closed_incubator_microplate_footprint_module_adapter_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_microplate_footprint_module_adapter_station_base_deck.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_standard_footprint_datum_plate.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_swappable_module_nests.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_fluidic_electrical_blind_mate_connector_gauge.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_port_alignment_witness_lanes.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_leak_pressure_witness_pockets.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_barcode_custody_status_surfaces.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_robot_gripper_approach_gauges.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_incubator_rack_handoff_dock.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_perfusion_manifold_handoff_gauge.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_closed_handoff_envelope_keepouts.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_evidence_camera_fiducial_bridge.stl",
    "output/closed_incubator_microplate_footprint_module_adapter_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "standard_footprint_datum_plate",
    "swappable_module_nests",
    "fluidic_blind_mate_connector_gauge",
    "electrical_blind_mate_connector_gauge",
    "port_alignment_witness_lanes",
    "leak_pressure_witness_pockets",
    "barcode_custody_status_surfaces",
    "robot_gripper_approach_gauges",
    "incubator_rack_handoff_dock",
    "perfusion_manifold_handoff_gauge",
    "closed_handoff_envelope_keepouts",
];

const UNIT_SYSTEM: &str = "millimeter";
const REPRODUCIBILITY_TAG: &str = "slas_microplate_footprint_adapter_station_v1_fixed_mm";
const RANDOMIZED_GEOMETRY: bool = false;
const SMALL_CYLINDER_SEGMENTS: u32 = 24;
const STANDARD_CYLINDER_SEGMENTS: u32 = 36;
const ROUND_CYLINDER_SEGMENTS: u32 = 48;

const SLAS_FOOTPRINT_X: f64 = 127.76;
const SLAS_FOOTPRINT_Y: f64 = 85.48;
const MODULE_CLEARANCE: f64 = 1.0;
const DATUM_PIN_DIAMETER: f64 = 5.0;

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 20.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 24.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DECK_DATUM_TARGETS: usize = 6;

const DATUM_PLATE_X: f64 = 250.0;
const DATUM_PLATE_Y: f64 = 190.0;
const DATUM_PLATE_Z: f64 = 18.0;
const DATUM_PLATE_CENTER: (f64, f64) = (-470.0, 150.0);
const DATUM_RAIL_W: f64 = 8.0;
const DATUM_RAIL_Z: f64 = 24.0;
const FOOTPRINT_RECESS_Z: f64 = 4.0;
const FOOTPRINT_CORNER_TARGETS: usize = 4;

const SWAP_NEST_COUNT: usize = 3;
const NEST_PANEL_X: f64 = 500.0;
const NEST_PANEL_Y: f64 = 250.0;
const NEST_PANEL_Z: f64 = 24.0;
const NEST_PANEL_CENTER: (f64, f64) = (-55.0, 150.0);
const NEST_FRAME_X: f64 = 152.0;
const NEST_FRAME_Y: f64 = 122.0;
const NEST_FRAME_Z: f64 = 28.0;
const NEST_PITCH_X: f64 = 156.0;
const NEST_DATUM_PINS_PER_NEST: usize = 3;
const SWAP_KEY_COUNT: usize = SWAP_NEST_COUNT;

const CONNECTOR_GAUGE_X: f64 = 330.0;
const CONNECTOR_GAUGE_Y: f64 = 220.0;
const CONNECTOR_GAUGE_Z: f64 = 52.0;
const CONNECTOR_GAUGE_CENTER: (f64, f64) = (395.0, 150.0);
const FLUIDIC_PORT_COUNT: usize = 6;
const ELECTRICAL_PIN_ROWS: usize = 2;
const ELECTRICAL_PIN_COLS: usize = 12;
const ELECTRICAL_PIN_COUNT: usize = ELECTRICAL_PIN_ROWS * ELECTRICAL_PIN_COLS;
const CONNECTOR_KEY_COUNT: usize = 5;
const FLUIDIC_PORT_PITCH: f64 = 38.0;
const ELECTRICAL_PIN_PITCH_X: f64 = 15.0;
const ELECTRICAL_PIN_PITCH_Y: f64 = 32.0;

const PORT_LANE_X: f64 = 390.0;
const PORT_LANE_Y: f64 = 130.0;
const PORT_LANE_Z: f64 = 16.0;
const PORT_LANE_CENTER: (f64, f64) = (-360.0, -80.0);
const PORT_WITNESS_LANE_COUNT: usize = 8;
const PORT_LANE_PITCH: f64 = 41.0;
const WITNESS_TICK_COUNT: usize = 5;

const LEAK_POCKET_X: f64 = 330.0;
const LEAK_POCKET_Y: f64 = 140.0;
const LEAK_POCKET_Z: f64 = 26.0;
const LEAK_POCKET_CENTER: (f64, f64) = (50.0, -80.0);
const LEAK_POCKET_COUNT: usize = 6;
const LEAK_POCKET_COLS: usize = 3;
const LEAK_POCKET_PITCH_X: f64 = 82.0;
const LEAK_POCKET_PITCH_Y: f64 = 54.0;

const STATUS_SURFACE_X: f64 = 320.0;
const STATUS_SURFACE_Y: f64 = 140.0;
const STATUS_SURFACE_Z: f64 = 12.0;
const STATUS_SURFACE_CENTER: (f64, f64) = (410.0, -80.0);
const BARCODE_LAND_COUNT: usize = 6;
const CUSTODY_TOKEN_COUNT: usize = 4;
const STATUS_LANE_COUNT: usize = 3;

const GRIPPER_GAUGE_X: f64 = 420.0;
const GRIPPER_GAUGE_Y: f64 = 130.0;
const GRIPPER_GAUGE_Z: f64 = 34.0;
const GRIPPER_GAUGE_CENTER: (f64, f64) = (-370.0, -300.0);
const GRIPPER_APPROACH_CORRIDORS: usize = 2;
const GRIPPER_GAUGE_POST_COUNT: usize = 4;
const GRIPPER_FINGER_CLEARANCE_X: f64 = 38.0;

const PERFUSION_GAUGE_X: f64 = 390.0;
const PERFUSION_GAUGE_Y: f64 = 130.0;
const PERFUSION_GAUGE_Z: f64 = 42.0;
const PERFUSION_GAUGE_CENTER: (f64, f64) = (50.0, -300.0);
const MANIFOLD_PORT_COUNT: usize = 8;
const MANIFOLD_PORT_PITCH: f64 = 36.0;
const MANIFOLD_STRAIN_RELIEF_COUNT: usize = 4;

const INCUBATOR_DOCK_X: f64 = 310.0;
const INCUBATOR_DOCK_Y: f64 = 130.0;
const INCUBATOR_DOCK_Z: f64 = 48.0;
const INCUBATOR_DOCK_CENTER: (f64, f64) = (430.0, -300.0);
const RACK_RAIL_COUNT: usize = 2;
const RACK_SLOT_ID_COUNT: usize = 5;

const KEEP_OUT_RAIL_Z: f64 = 8.0;
const KEEP_OUT_POST_COUNT: usize = 8;
const SERVICE_KEEP_OUT_X: f64 = STATION_X - 112.0;
const SERVICE_KEEP_OUT_Y: f64 = STATION_Y - 112.0;

const BRIDGE_X: f64 = 1000.0;
const BRIDGE_Y: f64 = 64.0;
const BRIDGE_CENTER: (f64, f64) = (0.0, 342.0);
const BRIDGE_POST_X: f64 = 26.0;
const BRIDGE_POST_Y: f64 = 52.0;
const BRIDGE_POST_Z: f64 = 152.0;
const BRIDGE_BEAM_Z: f64 = 22.0;
const CAMERA_LAND_COUNT: usize = 4;
const BRIDGE_FIDUCIAL_COUNT: usize = 8;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn inside_station(self, margin: f64) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - margin
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - margin
    }

    fn overlaps_with_margin(self, other: Rect, margin: f64) -> bool {
        let x_overlap =
            (self.center.0 - other.center.0).abs() < self.x / 2.0 + other.x / 2.0 + margin;
        let y_overlap =
            (self.center.1 - other.center.1).abs() < self.y / 2.0 + other.y / 2.0 + margin;
        x_overlap && y_overlap
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let datum_plate = standard_footprint_datum_plate();
    export(OUTPUTS[1], &datum_plate);

    let nests = swappable_module_nests();
    export(OUTPUTS[2], &nests);

    let connector_gauge = fluidic_electrical_blind_mate_connector_gauge();
    export(OUTPUTS[3], &connector_gauge);

    let port_lanes = port_alignment_witness_lanes();
    export(OUTPUTS[4], &port_lanes);

    let leak_pockets = leak_pressure_witness_pockets();
    export(OUTPUTS[5], &leak_pockets);

    let custody = barcode_custody_status_surfaces();
    export(OUTPUTS[6], &custody);

    let gripper = robot_gripper_approach_gauges();
    export(OUTPUTS[7], &gripper);

    let incubator_dock = incubator_rack_handoff_dock();
    export(OUTPUTS[8], &incubator_dock);

    let perfusion = perfusion_manifold_handoff_gauge();
    export(OUTPUTS[9], &perfusion);

    let keepouts = closed_handoff_envelope_keepouts();
    export(OUTPUTS[10], &keepouts);

    let bridge = evidence_camera_fiducial_bridge();
    export(OUTPUTS[11], &bridge);

    let assembly =
        base + datum_plate.translate(
            DATUM_PLATE_CENTER.0,
            DATUM_PLATE_CENTER.1,
            deck_insert_z(DATUM_PLATE_Z),
        ) + nests.translate(
            NEST_PANEL_CENTER.0,
            NEST_PANEL_CENTER.1,
            deck_insert_z(NEST_PANEL_Z),
        ) + connector_gauge.translate(
            CONNECTOR_GAUGE_CENTER.0,
            CONNECTOR_GAUGE_CENTER.1,
            deck_insert_z(CONNECTOR_GAUGE_Z),
        ) + port_lanes.translate(
            PORT_LANE_CENTER.0,
            PORT_LANE_CENTER.1,
            deck_insert_z(PORT_LANE_Z),
        ) + leak_pockets.translate(
            LEAK_POCKET_CENTER.0,
            LEAK_POCKET_CENTER.1,
            deck_insert_z(LEAK_POCKET_Z),
        ) + custody.translate(
            STATUS_SURFACE_CENTER.0,
            STATUS_SURFACE_CENTER.1,
            deck_insert_z(STATUS_SURFACE_Z),
        ) + gripper.translate(
            GRIPPER_GAUGE_CENTER.0,
            GRIPPER_GAUGE_CENTER.1,
            deck_insert_z(GRIPPER_GAUGE_Z),
        ) + incubator_dock.translate(
            INCUBATOR_DOCK_CENTER.0,
            INCUBATOR_DOCK_CENTER.1,
            deck_insert_z(INCUBATOR_DOCK_Z),
        ) + perfusion.translate(
            PERFUSION_GAUGE_CENTER.0,
            PERFUSION_GAUGE_CENTER.1,
            deck_insert_z(PERFUSION_GAUGE_Z),
        ) + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_RAIL_Z / 2.0)
            + bridge.translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator microplate-footprint module adapter station:");
    println!(
        "  Station deck:              {STATION_X:.0}mm x {STATION_Y:.0}mm closed-system validation fixture"
    );
    println!(
        "  ANSI/SLAS footprint datum: {SLAS_FOOTPRINT_X:.2}mm x {SLAS_FOOTPRINT_Y:.2}mm with {DATUM_PIN_DIAMETER:.1}mm datum pin gauge references"
    );
    println!(
        "  Swappable nests:           {SWAP_NEST_COUNT} module nests with {NEST_DATUM_PINS_PER_NEST} datum pins each and {SWAP_KEY_COUNT} swap keys"
    );
    println!(
        "  Blind-mate gauge:          {FLUIDIC_PORT_COUNT} fluidic ports, {ELECTRICAL_PIN_COUNT} electrical pin positions, {CONNECTOR_KEY_COUNT} key gauges"
    );
    println!(
        "  Witness/control surfaces:  {PORT_WITNESS_LANE_COUNT} port lanes, {LEAK_POCKET_COUNT} leak/pressure pockets, {BARCODE_LAND_COUNT} barcode lands, {STATUS_LANE_COUNT} status lanes"
    );
    println!(
        "  Closed handoff:            incubator rack dock plus {MANIFOLD_PORT_COUNT} perfusion manifold gauge ports and {GRIPPER_GAUGE_POST_COUNT} gripper approach posts"
    );
    println!(
        "  Reproducibility:           {UNIT_SYSTEM}, {REPRODUCIBILITY_TAG}, randomized geometry = {RANDOMIZED_GEOMETRY}"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(component_z: f64) -> f64 {
    BASE_Z / 2.0 + component_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(
        ELECTRICAL_PIN_COUNT,
        ELECTRICAL_PIN_ROWS * ELECTRICAL_PIN_COLS
    );
    assert_eq!(MOUNT_SLOT_COUNT, mount_slot_points().len());
    assert_eq!(DECK_DATUM_TARGETS, deck_datum_target_points().len());
    assert_eq!(FOOTPRINT_CORNER_TARGETS, footprint_corner_targets().len());
    assert_eq!(SWAP_NEST_COUNT, 3);
    assert_eq!(KEEP_OUT_POST_COUNT, 8);
    assert!(SLAS_FOOTPRINT_X > SLAS_FOOTPRINT_Y);
    assert!(NEST_FRAME_X > SLAS_FOOTPRINT_X + 2.0 * MODULE_CLEARANCE);
    assert!(NEST_FRAME_Y > SLAS_FOOTPRINT_Y + 2.0 * MODULE_CLEARANCE);
    assert!(!RANDOMIZED_GEOMETRY);

    for rect in collision_sensitive_rects() {
        assert!(
            rect.inside_station(8.0),
            "{} exceeds station deck",
            rect.name
        );
    }

    let rects = collision_sensitive_rects();
    let non_overlay_count = rects.len() - 1;
    for i in 0..non_overlay_count {
        for j in (i + 1)..non_overlay_count {
            assert!(
                !rects[i].overlaps_with_margin(rects[j], 12.0),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "microplate_adapter_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let front_rim = centered_cube(
        "microplate_adapter_station_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear_rim = centered_cube(
        "microplate_adapter_station_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left_rim = centered_cube(
        "microplate_adapter_station_left_containment_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right_rim = centered_cube(
        "microplate_adapter_station_right_containment_rim",
        RIM_W,
        STATION_Y - 2.0 * RIM_W,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    deck - mount_slot_reliefs()
        + front_rim
        + rear_rim
        + left_rim
        + right_rim
        + deck_datum_targets()
        + deck_flow_witness_ribs()
}

fn mount_slot_reliefs() -> Part {
    let mut slots = Part::empty("microplate_adapter_station_mount_slot_reliefs");
    for (index, (x, y)) in mount_slot_points().into_iter().enumerate() {
        let slot = centered_cube(
            format!("microplate_adapter_station_mount_slot_center_cut_{index}"),
            28.0,
            7.0,
            BASE_Z + 4.0,
        )
        .translate(x, y, 0.0);
        let left_end = centered_cylinder(
            format!("microplate_adapter_station_mount_slot_left_radius_{index}"),
            3.5,
            BASE_Z + 5.0,
            SMALL_CYLINDER_SEGMENTS,
        )
        .translate(x - 14.0, y, 0.0);
        let right_end = centered_cylinder(
            format!("microplate_adapter_station_mount_slot_right_radius_{index}"),
            3.5,
            BASE_Z + 5.0,
            SMALL_CYLINDER_SEGMENTS,
        )
        .translate(x + 14.0, y, 0.0);
        slots = slots + slot + left_end + right_end;
    }
    slots
}

fn mount_slot_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-540.0, -350.0),
        (-360.0, -350.0),
        (360.0, -350.0),
        (540.0, -350.0),
        (-540.0, 350.0),
        (-360.0, 350.0),
        (360.0, 350.0),
        (540.0, 350.0),
    ]
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("microplate_adapter_station_deck_datum_targets");
    for (index, (x, y)) in deck_datum_target_points().into_iter().enumerate() {
        let target = fiducial_disc(format!(
            "microplate_adapter_station_deck_datum_target_{index}"
        ))
        .translate(x, y, BASE_Z / 2.0 + 2.0);
        targets = targets + target;
    }
    targets
}

fn deck_datum_target_points() -> [(f64, f64); DECK_DATUM_TARGETS] {
    [
        (-555.0, -300.0),
        (-555.0, 300.0),
        (0.0, -360.0),
        (0.0, 300.0),
        (555.0, -300.0),
        (555.0, 300.0),
    ]
}

fn deck_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("microplate_adapter_station_closed_flow_witness_ribs");
    for (index, y) in [-198.0, -2.0, 304.0].into_iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("microplate_adapter_station_lane_boundary_witness_rib_{index}"),
                STATION_X - 180.0,
                3.5,
                4.0,
            )
            .translate(0.0, y, BASE_Z / 2.0 + 2.0);
    }
    ribs
}

fn standard_footprint_datum_plate() -> Part {
    let body = centered_cube(
        "microplate_adapter_standard_footprint_datum_plate_body",
        DATUM_PLATE_X,
        DATUM_PLATE_Y,
        DATUM_PLATE_Z,
    );
    let footprint_recess = centered_cube(
        "microplate_adapter_slas_footprint_recess",
        SLAS_FOOTPRINT_X + 2.0 * MODULE_CLEARANCE,
        SLAS_FOOTPRINT_Y + 2.0 * MODULE_CLEARANCE,
        FOOTPRINT_RECESS_Z + 1.0,
    )
    .translate(0.0, 0.0, DATUM_PLATE_Z / 2.0 - FOOTPRINT_RECESS_Z / 2.0);
    let barcode_access = centered_cube(
        "microplate_adapter_datum_plate_front_barcode_access_relief",
        84.0,
        16.0,
        FOOTPRINT_RECESS_Z + 2.0,
    )
    .translate(
        0.0,
        -SLAS_FOOTPRINT_Y / 2.0 - 16.0,
        DATUM_PLATE_Z / 2.0 - 2.0,
    );

    body - footprint_recess - barcode_access
        + datum_plate_rails()
        + datum_plate_pins()
        + datum_plate_corner_targets()
        + datum_plate_origin_arrow()
}

fn datum_plate_rails() -> Part {
    let left_rail = centered_cube(
        "microplate_adapter_slas_left_x_hard_datum_rail",
        DATUM_RAIL_W,
        SLAS_FOOTPRINT_Y + 34.0,
        DATUM_RAIL_Z,
    )
    .translate(
        -SLAS_FOOTPRINT_X / 2.0 - DATUM_RAIL_W / 2.0 - 8.0,
        0.0,
        DATUM_PLATE_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let rear_rail = centered_cube(
        "microplate_adapter_slas_rear_y_hard_datum_rail",
        SLAS_FOOTPRINT_X + 48.0,
        DATUM_RAIL_W,
        DATUM_RAIL_Z,
    )
    .translate(
        0.0,
        SLAS_FOOTPRINT_Y / 2.0 + DATUM_RAIL_W / 2.0 + 8.0,
        DATUM_PLATE_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let compliant_side = centered_cube(
        "microplate_adapter_slas_right_soft_capture_rail",
        DATUM_RAIL_W,
        SLAS_FOOTPRINT_Y * 0.72,
        DATUM_RAIL_Z * 0.62,
    )
    .translate(
        SLAS_FOOTPRINT_X / 2.0 + DATUM_RAIL_W / 2.0 + 8.0,
        -8.0,
        DATUM_PLATE_Z / 2.0 + DATUM_RAIL_Z * 0.31,
    );
    let front_entry_lip = centered_cube(
        "microplate_adapter_slas_front_low_entry_lip",
        SLAS_FOOTPRINT_X + 42.0,
        8.0,
        12.0,
    )
    .translate(
        0.0,
        -SLAS_FOOTPRINT_Y / 2.0 - 12.0,
        DATUM_PLATE_Z / 2.0 + 6.0,
    );

    left_rail + rear_rail + compliant_side + front_entry_lip
}

fn datum_plate_pins() -> Part {
    let mut pins = Part::empty("microplate_adapter_slas_datum_pin_gauge_set");
    for (index, (x, y)) in [
        (
            -SLAS_FOOTPRINT_X / 2.0 - 18.0,
            SLAS_FOOTPRINT_Y / 2.0 + 18.0,
        ),
        (SLAS_FOOTPRINT_X / 2.0 + 18.0, SLAS_FOOTPRINT_Y / 2.0 + 18.0),
        (
            -SLAS_FOOTPRINT_X / 2.0 - 18.0,
            -SLAS_FOOTPRINT_Y / 2.0 - 18.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("microplate_adapter_slas_datum_pin_boss_{index}"),
            10.0,
            7.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, y, DATUM_PLATE_Z / 2.0 + 3.5);
        let pin = centered_cylinder(
            format!("microplate_adapter_slas_datum_pin_gauge_{index}"),
            DATUM_PIN_DIAMETER / 2.0,
            18.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, y, DATUM_PLATE_Z / 2.0 + 13.0);
        pins = pins + boss + pin;
    }
    pins
}

fn datum_plate_corner_targets() -> Part {
    let mut targets = Part::empty("microplate_adapter_slas_footprint_corner_targets");
    for (index, (x, y)) in footprint_corner_targets().into_iter().enumerate() {
        targets =
            targets
                + fiducial_disc(format!("microplate_adapter_slas_corner_target_{index}"))
                    .translate(x, y, DATUM_PLATE_Z / 2.0 + 2.0);
    }
    targets
}

fn footprint_corner_targets() -> [(f64, f64); FOOTPRINT_CORNER_TARGETS] {
    [
        (-SLAS_FOOTPRINT_X / 2.0, -SLAS_FOOTPRINT_Y / 2.0),
        (SLAS_FOOTPRINT_X / 2.0, -SLAS_FOOTPRINT_Y / 2.0),
        (-SLAS_FOOTPRINT_X / 2.0, SLAS_FOOTPRINT_Y / 2.0),
        (SLAS_FOOTPRINT_X / 2.0, SLAS_FOOTPRINT_Y / 2.0),
    ]
}

fn datum_plate_origin_arrow() -> Part {
    let stem = centered_cube(
        "microplate_adapter_slas_a1_origin_arrow_stem",
        46.0,
        5.0,
        5.0,
    )
    .translate(-50.0, -78.0, DATUM_PLATE_Z / 2.0 + 2.5);
    let head = centered_cube(
        "microplate_adapter_slas_a1_origin_arrow_head",
        14.0,
        14.0,
        5.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-25.0, -78.0, DATUM_PLATE_Z / 2.0 + 2.5);
    stem + head
}

fn swappable_module_nests() -> Part {
    let panel = centered_cube(
        "microplate_adapter_swappable_nest_panel",
        NEST_PANEL_X,
        NEST_PANEL_Y,
        NEST_PANEL_Z,
    );
    let swap_key_backbone = centered_cube(
        "microplate_adapter_swappable_nest_key_backbone",
        NEST_PANEL_X - 42.0,
        12.0,
        18.0,
    )
    .translate(0.0, NEST_PANEL_Y / 2.0 - 24.0, NEST_PANEL_Z / 2.0 + 9.0);

    panel - nest_socket_reliefs() + nest_frames() + swap_key_backbone + nest_lane_labels()
}

fn nest_socket_reliefs() -> Part {
    let mut reliefs = Part::empty("microplate_adapter_swappable_nest_socket_reliefs");
    for index in 0..SWAP_NEST_COUNT {
        reliefs = reliefs
            + centered_cube(
                format!("microplate_adapter_swappable_nest_socket_relief_{index}"),
                NEST_FRAME_X + 12.0,
                NEST_FRAME_Y + 12.0,
                7.0,
            )
            .translate(nest_center_x(index), -18.0, NEST_PANEL_Z / 2.0 - 3.0);
    }
    reliefs
}

fn nest_frames() -> Part {
    let mut nests = Part::empty("microplate_adapter_swappable_module_nest_frames");
    for index in 0..SWAP_NEST_COUNT {
        let frame = swappable_nest(index).translate(nest_center_x(index), -18.0, 0.0);
        nests = nests + frame;
    }
    nests
}

fn swappable_nest(index: usize) -> Part {
    let body = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_body"),
        NEST_FRAME_X,
        NEST_FRAME_Y,
        NEST_FRAME_Z,
    )
    .translate(0.0, 0.0, NEST_PANEL_Z / 2.0 + NEST_FRAME_Z / 2.0 - 2.0);
    let module_recess = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_slas_module_recess"),
        SLAS_FOOTPRINT_X + 2.0 * MODULE_CLEARANCE,
        SLAS_FOOTPRINT_Y + 2.0 * MODULE_CLEARANCE,
        NEST_FRAME_Z + 4.0,
    )
    .translate(0.0, 0.0, NEST_PANEL_Z / 2.0 + NEST_FRAME_Z / 2.0 - 2.0);
    let front_finger_cut = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_robot_front_finger_cut"),
        58.0,
        16.0,
        NEST_FRAME_Z + 6.0,
    )
    .translate(
        0.0,
        -NEST_FRAME_Y / 2.0 + 10.0,
        NEST_PANEL_Z / 2.0 + NEST_FRAME_Z / 2.0 - 2.0,
    );
    let side_key_cut = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_asymmetric_swap_key_relief"),
        16.0 + index as f64 * 4.0,
        10.0,
        NEST_FRAME_Z + 8.0,
    )
    .translate(
        NEST_FRAME_X / 2.0 - 22.0,
        NEST_FRAME_Y / 2.0 - 18.0,
        NEST_PANEL_Z / 2.0 + NEST_FRAME_Z / 2.0 - 2.0,
    );

    body - module_recess - front_finger_cut - side_key_cut
        + nest_datum_rails(index)
        + nest_datum_pins(index)
        + nest_latch_witness_tabs(index)
}

fn nest_datum_rails(index: usize) -> Part {
    let fixed_x = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_fixed_x_datum_rail"),
        7.0,
        SLAS_FOOTPRINT_Y + 20.0,
        18.0,
    )
    .translate(
        -SLAS_FOOTPRINT_X / 2.0 - 10.0,
        0.0,
        NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 7.0,
    );
    let fixed_y = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_fixed_y_datum_rail"),
        SLAS_FOOTPRINT_X + 22.0,
        7.0,
        18.0,
    )
    .translate(
        0.0,
        SLAS_FOOTPRINT_Y / 2.0 + 10.0,
        NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 7.0,
    );
    let soft_capture = centered_cube(
        format!("microplate_adapter_swappable_nest_{index}_soft_capture_side_rail"),
        7.0,
        SLAS_FOOTPRINT_Y * 0.58,
        12.0,
    )
    .translate(
        SLAS_FOOTPRINT_X / 2.0 + 10.0,
        -6.0,
        NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 4.0,
    );
    fixed_x + fixed_y + soft_capture
}

fn nest_datum_pins(index: usize) -> Part {
    let mut pins = Part::empty(format!(
        "microplate_adapter_swappable_nest_{index}_datum_pins"
    ));
    for (pin_index, (x, y)) in [
        (
            -SLAS_FOOTPRINT_X / 2.0 - 16.0,
            SLAS_FOOTPRINT_Y / 2.0 + 14.0,
        ),
        (SLAS_FOOTPRINT_X / 2.0 + 16.0, SLAS_FOOTPRINT_Y / 2.0 + 14.0),
        (
            -SLAS_FOOTPRINT_X / 2.0 - 16.0,
            -SLAS_FOOTPRINT_Y / 2.0 - 14.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("microplate_adapter_swappable_nest_{index}_datum_pin_{pin_index}"),
                2.8,
                16.0,
                SMALL_CYLINDER_SEGMENTS,
            )
            .translate(x, y, NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 8.0);
    }
    pins
}

fn nest_latch_witness_tabs(index: usize) -> Part {
    let mut tabs = Part::empty(format!(
        "microplate_adapter_swappable_nest_{index}_latch_witness_tabs"
    ));
    for (tab_index, x) in [-42.0, 0.0, 42.0].into_iter().enumerate() {
        let tab = centered_cube(
            format!("microplate_adapter_swappable_nest_{index}_latch_tab_{tab_index}"),
            24.0,
            7.0,
            8.0,
        )
        .translate(
            x,
            -SLAS_FOOTPRINT_Y / 2.0 - 15.0,
            NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 4.0,
        );
        let witness_cut = centered_cube(
            format!("microplate_adapter_swappable_nest_{index}_latch_tab_witness_cut_{tab_index}"),
            12.0,
            2.5,
            9.0,
        )
        .translate(
            x,
            -SLAS_FOOTPRINT_Y / 2.0 - 15.0,
            NEST_PANEL_Z / 2.0 + NEST_FRAME_Z + 4.0,
        );
        tabs = tabs + (tab - witness_cut);
    }
    tabs
}

fn nest_lane_labels() -> Part {
    let mut labels = Part::empty("microplate_adapter_swappable_nest_lane_labels");
    for index in 0..SWAP_NEST_COUNT {
        labels = labels
            + centered_cube(
                format!("microplate_adapter_swappable_nest_lane_status_land_{index}"),
                64.0,
                20.0,
                4.0,
            )
            .translate(
                nest_center_x(index),
                -NEST_PANEL_Y / 2.0 + 28.0,
                NEST_PANEL_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn nest_center_x(index: usize) -> f64 {
    centered_index(index, SWAP_NEST_COUNT, NEST_PITCH_X)
}

fn fluidic_electrical_blind_mate_connector_gauge() -> Part {
    let body = centered_cube(
        "microplate_adapter_blind_mate_connector_gauge_body",
        CONNECTOR_GAUGE_X,
        CONNECTOR_GAUGE_Y,
        CONNECTOR_GAUGE_Z,
    );
    let family_divider = centered_cube(
        "microplate_adapter_blind_mate_fluidic_electrical_family_divider",
        CONNECTOR_GAUGE_X - 36.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, CONNECTOR_GAUGE_Z / 2.0 + 9.0);

    body - fluidic_port_bores() - electrical_pin_bores() - connector_key_reliefs()
        + fluidic_port_collars()
        + electrical_pin_lands()
        + connector_key_gauge_comb()
        + family_divider
}

fn fluidic_port_bores() -> Part {
    let mut bores = Part::empty("microplate_adapter_blind_mate_fluidic_port_bores");
    for port in 0..FLUIDIC_PORT_COUNT {
        let x = centered_index(port, FLUIDIC_PORT_COUNT, FLUIDIC_PORT_PITCH);
        bores = bores
            + centered_cylinder(
                format!("microplate_adapter_blind_mate_fluidic_port_bore_{port}"),
                6.0,
                CONNECTOR_GAUGE_Z + 6.0,
                STANDARD_CYLINDER_SEGMENTS,
            )
            .translate(x, 58.0, 0.0);
    }
    bores
}

fn fluidic_port_collars() -> Part {
    let mut collars = Part::empty("microplate_adapter_blind_mate_fluidic_port_collars");
    for port in 0..FLUIDIC_PORT_COUNT {
        let x = centered_index(port, FLUIDIC_PORT_COUNT, FLUIDIC_PORT_PITCH);
        let collar = centered_cylinder(
            format!("microplate_adapter_blind_mate_fluidic_port_collar_{port}"),
            12.0,
            5.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, 58.0, CONNECTOR_GAUGE_Z / 2.0 + 2.5);
        let witness_slot = centered_cube(
            format!("microplate_adapter_blind_mate_fluidic_port_witness_slot_{port}"),
            18.0,
            3.0,
            6.0,
        )
        .translate(x, 58.0, CONNECTOR_GAUGE_Z / 2.0 + 2.5);
        collars = collars + (collar - witness_slot);
    }
    collars
}

fn electrical_pin_bores() -> Part {
    let mut bores = Part::empty("microplate_adapter_blind_mate_electrical_pin_bores");
    for row in 0..ELECTRICAL_PIN_ROWS {
        for col in 0..ELECTRICAL_PIN_COLS {
            let pin = row * ELECTRICAL_PIN_COLS + col;
            let x = centered_index(col, ELECTRICAL_PIN_COLS, ELECTRICAL_PIN_PITCH_X);
            let y = -58.0 + centered_index(row, ELECTRICAL_PIN_ROWS, ELECTRICAL_PIN_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("microplate_adapter_blind_mate_electrical_pin_bore_{pin}"),
                    1.5,
                    CONNECTOR_GAUGE_Z + 6.0,
                    SMALL_CYLINDER_SEGMENTS,
                )
                .translate(x, y, 0.0);
        }
    }
    bores
}

fn electrical_pin_lands() -> Part {
    let mut lands = Part::empty("microplate_adapter_blind_mate_electrical_pin_lands");
    for row in 0..ELECTRICAL_PIN_ROWS {
        for col in 0..ELECTRICAL_PIN_COLS {
            let pin = row * ELECTRICAL_PIN_COLS + col;
            let x = centered_index(col, ELECTRICAL_PIN_COLS, ELECTRICAL_PIN_PITCH_X);
            let y = -58.0 + centered_index(row, ELECTRICAL_PIN_ROWS, ELECTRICAL_PIN_PITCH_Y);
            lands = lands
                + centered_cylinder(
                    format!("microplate_adapter_blind_mate_electrical_pin_land_{pin}"),
                    3.4,
                    3.0,
                    SMALL_CYLINDER_SEGMENTS,
                )
                .translate(x, y, CONNECTOR_GAUGE_Z / 2.0 + 1.5);
        }
    }
    lands
}

fn connector_key_reliefs() -> Part {
    let mut keys = Part::empty("microplate_adapter_blind_mate_key_reliefs");
    for key in 0..CONNECTOR_KEY_COUNT {
        let x = centered_index(key, CONNECTOR_KEY_COUNT, 48.0);
        keys = keys
            + centered_cube(
                format!("microplate_adapter_blind_mate_asymmetric_key_relief_{key}"),
                12.0 + key as f64 * 2.5,
                16.0,
                CONNECTOR_GAUGE_Z + 4.0,
            )
            .translate(x, CONNECTOR_GAUGE_Y / 2.0 - 28.0, 0.0);
    }
    keys
}

fn connector_key_gauge_comb() -> Part {
    let mut comb = Part::empty("microplate_adapter_blind_mate_connector_key_gauge_comb");
    for key in 0..CONNECTOR_KEY_COUNT {
        let x = centered_index(key, CONNECTOR_KEY_COUNT, 48.0);
        comb = comb
            + centered_cube(
                format!("microplate_adapter_blind_mate_no_go_key_blade_{key}"),
                8.0 + key as f64 * 2.0,
                24.0,
                22.0,
            )
            .translate(
                x,
                CONNECTOR_GAUGE_Y / 2.0 - 66.0,
                CONNECTOR_GAUGE_Z / 2.0 + 11.0,
            );
    }
    comb
}

fn port_alignment_witness_lanes() -> Part {
    let plate = centered_cube(
        "microplate_adapter_port_alignment_witness_lane_plate",
        PORT_LANE_X,
        PORT_LANE_Y,
        PORT_LANE_Z,
    );
    plate - port_lane_grooves() + port_lane_centerlines() + port_lane_witness_ticks()
}

fn port_lane_grooves() -> Part {
    let mut grooves = Part::empty("microplate_adapter_port_alignment_lane_grooves");
    for lane in 0..PORT_WITNESS_LANE_COUNT {
        grooves = grooves
            + centered_cube(
                format!("microplate_adapter_port_alignment_lane_groove_{lane}"),
                25.0,
                PORT_LANE_Y - 34.0,
                5.0,
            )
            .translate(
                centered_index(lane, PORT_WITNESS_LANE_COUNT, PORT_LANE_PITCH),
                0.0,
                PORT_LANE_Z / 2.0 - 2.0,
            );
    }
    grooves
}

fn port_lane_centerlines() -> Part {
    let mut centerlines = Part::empty("microplate_adapter_port_alignment_lane_centerlines");
    for lane in 0..PORT_WITNESS_LANE_COUNT {
        let x = centered_index(lane, PORT_WITNESS_LANE_COUNT, PORT_LANE_PITCH);
        centerlines = centerlines
            + centered_cube(
                format!("microplate_adapter_port_alignment_lane_centerline_{lane}"),
                3.0,
                PORT_LANE_Y - 22.0,
                4.0,
            )
            .translate(x, 0.0, PORT_LANE_Z / 2.0 + 2.0)
            + centered_cylinder(
                format!("microplate_adapter_port_alignment_lane_port_target_{lane}"),
                8.0,
                4.0,
                STANDARD_CYLINDER_SEGMENTS,
            )
            .translate(x, PORT_LANE_Y / 2.0 - 22.0, PORT_LANE_Z / 2.0 + 2.0);
    }
    centerlines
}

fn port_lane_witness_ticks() -> Part {
    let mut ticks = Part::empty("microplate_adapter_port_alignment_witness_ticks");
    for lane in 0..PORT_WITNESS_LANE_COUNT {
        let x = centered_index(lane, PORT_WITNESS_LANE_COUNT, PORT_LANE_PITCH);
        for tick in 0..WITNESS_TICK_COUNT {
            ticks = ticks
                + centered_cube(
                    format!("microplate_adapter_port_lane_{lane}_witness_tick_{tick}"),
                    17.0,
                    3.0,
                    5.0,
                )
                .translate(
                    x,
                    centered_index(tick, WITNESS_TICK_COUNT, 18.0),
                    PORT_LANE_Z / 2.0 + 2.5,
                );
        }
    }
    ticks
}

fn leak_pressure_witness_pockets() -> Part {
    let body = centered_cube(
        "microplate_adapter_leak_pressure_witness_pocket_body",
        LEAK_POCKET_X,
        LEAK_POCKET_Y,
        LEAK_POCKET_Z,
    );
    body - leak_pressure_pocket_recesses()
        + leak_pressure_pocket_rims()
        + leak_pressure_trace_lanes()
        + leak_pressure_reference_land()
}

fn leak_pressure_pocket_recesses() -> Part {
    let mut recesses = Part::empty("microplate_adapter_leak_pressure_pocket_recesses");
    for pocket in 0..LEAK_POCKET_COUNT {
        let (x, y) = leak_pocket_center(pocket);
        recesses = recesses
            + centered_cylinder(
                format!("microplate_adapter_leak_pressure_witness_pocket_recess_{pocket}"),
                18.0,
                10.0,
                ROUND_CYLINDER_SEGMENTS,
            )
            .translate(x, y, LEAK_POCKET_Z / 2.0 - 4.0);
    }
    recesses
}

fn leak_pressure_pocket_rims() -> Part {
    let mut rims = Part::empty("microplate_adapter_leak_pressure_pocket_rims");
    for pocket in 0..LEAK_POCKET_COUNT {
        let (x, y) = leak_pocket_center(pocket);
        let rim = centered_cylinder(
            format!("microplate_adapter_leak_pressure_witness_pocket_rim_{pocket}"),
            25.0,
            5.0,
            ROUND_CYLINDER_SEGMENTS,
        )
        .translate(x, y, LEAK_POCKET_Z / 2.0 + 2.5);
        let clear_center = centered_cylinder(
            format!("microplate_adapter_leak_pressure_witness_pocket_open_center_{pocket}"),
            17.0,
            6.0,
            ROUND_CYLINDER_SEGMENTS,
        )
        .translate(x, y, LEAK_POCKET_Z / 2.0 + 2.5);
        rims = rims + (rim - clear_center);
    }
    rims
}

fn leak_pressure_trace_lanes() -> Part {
    let mut traces = Part::empty("microplate_adapter_leak_pressure_capillary_trace_lanes");
    for pocket in 0..LEAK_POCKET_COUNT {
        let (x, y) = leak_pocket_center(pocket);
        traces = traces
            + centered_cube(
                format!("microplate_adapter_leak_pressure_trace_lane_{pocket}"),
                58.0,
                4.0,
                4.0,
            )
            .translate(x + 42.0, y, LEAK_POCKET_Z / 2.0 + 2.0);
    }
    traces
}

fn leak_pressure_reference_land() -> Part {
    centered_cube(
        "microplate_adapter_leak_pressure_reference_coupon_land",
        78.0,
        24.0,
        4.0,
    )
    .translate(
        -LEAK_POCKET_X / 2.0 + 52.0,
        -LEAK_POCKET_Y / 2.0 + 20.0,
        LEAK_POCKET_Z / 2.0 + 2.0,
    )
}

fn leak_pocket_center(pocket: usize) -> (f64, f64) {
    let col = pocket % LEAK_POCKET_COLS;
    let row = pocket / LEAK_POCKET_COLS;
    (
        centered_index(col, LEAK_POCKET_COLS, LEAK_POCKET_PITCH_X),
        centered_index(row, 2, LEAK_POCKET_PITCH_Y),
    )
}

fn barcode_custody_status_surfaces() -> Part {
    let plate = centered_cube(
        "microplate_adapter_barcode_custody_status_surface_plate",
        STATUS_SURFACE_X,
        STATUS_SURFACE_Y,
        STATUS_SURFACE_Z,
    );
    plate + barcode_lands() + custody_token_wells() + status_lanes() + qr_reference_grid()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("microplate_adapter_barcode_lands");
    for land in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("microplate_adapter_barcode_land_{land}"),
                78.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(land % 3, 3, 94.0),
                38.0 - (land / 3) as f64 * 30.0,
                STATUS_SURFACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty("microplate_adapter_custody_token_wells");
    for token in 0..CUSTODY_TOKEN_COUNT {
        let x = centered_index(token, CUSTODY_TOKEN_COUNT, 38.0);
        let rim = centered_cylinder(
            format!("microplate_adapter_custody_token_well_rim_{token}"),
            13.0,
            5.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, -44.0, STATUS_SURFACE_Z / 2.0 + 2.5);
        let recess = centered_cylinder(
            format!("microplate_adapter_custody_token_well_recess_{token}"),
            8.0,
            6.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, -44.0, STATUS_SURFACE_Z / 2.0 + 2.5);
        wells = wells + (rim - recess);
    }
    wells
}

fn status_lanes() -> Part {
    let mut lanes = Part::empty("microplate_adapter_status_lane_surfaces");
    for lane in 0..STATUS_LANE_COUNT {
        lanes = lanes
            + centered_cube(
                format!("microplate_adapter_status_lane_surface_{lane}"),
                88.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(lane, STATUS_LANE_COUNT, 102.0),
                4.0,
                STATUS_SURFACE_Z / 2.0 + 2.5,
            );
    }
    lanes
}

fn qr_reference_grid() -> Part {
    let mut grid = Part::empty("microplate_adapter_qr_reference_grid");
    for row in 0..4 {
        for col in 0..4 {
            grid = grid
                + centered_cube(
                    format!("microplate_adapter_qr_reference_cell_{row}_{col}"),
                    5.0,
                    5.0,
                    3.0,
                )
                .translate(
                    STATUS_SURFACE_X / 2.0 - 38.0 + col as f64 * 8.0,
                    STATUS_SURFACE_Y / 2.0 - 38.0 + row as f64 * 8.0,
                    STATUS_SURFACE_Z / 2.0 + 1.5,
                );
        }
    }
    grid
}

fn robot_gripper_approach_gauges() -> Part {
    let plate = centered_cube(
        "microplate_adapter_robot_gripper_approach_gauge_plate",
        GRIPPER_GAUGE_X,
        GRIPPER_GAUGE_Y,
        GRIPPER_GAUGE_Z,
    );
    plate - gripper_finger_clearance_cuts()
        + gripper_approach_corridor_rails()
        + gripper_approach_posts()
        + gripper_centerline_flags()
}

fn gripper_finger_clearance_cuts() -> Part {
    let mut cuts = Part::empty("microplate_adapter_gripper_finger_clearance_cuts");
    for corridor in 0..GRIPPER_APPROACH_CORRIDORS {
        cuts = cuts
            + centered_cube(
                format!("microplate_adapter_gripper_corridor_{corridor}_finger_clearance_cut"),
                GRIPPER_FINGER_CLEARANCE_X,
                GRIPPER_GAUGE_Y + 4.0,
                GRIPPER_GAUGE_Z + 4.0,
            )
            .translate(
                centered_index(corridor, GRIPPER_APPROACH_CORRIDORS, 128.0),
                0.0,
                0.0,
            );
    }
    cuts
}

fn gripper_approach_corridor_rails() -> Part {
    let mut rails = Part::empty("microplate_adapter_gripper_approach_corridor_rails");
    for corridor in 0..GRIPPER_APPROACH_CORRIDORS {
        let x = centered_index(corridor, GRIPPER_APPROACH_CORRIDORS, 128.0);
        rails = rails
            + centered_cube(
                format!("microplate_adapter_gripper_corridor_{corridor}_left_rail"),
                7.0,
                GRIPPER_GAUGE_Y - 20.0,
                30.0,
            )
            .translate(
                x - GRIPPER_FINGER_CLEARANCE_X / 2.0 - 8.0,
                0.0,
                GRIPPER_GAUGE_Z / 2.0 + 15.0,
            )
            + centered_cube(
                format!("microplate_adapter_gripper_corridor_{corridor}_right_rail"),
                7.0,
                GRIPPER_GAUGE_Y - 20.0,
                30.0,
            )
            .translate(
                x + GRIPPER_FINGER_CLEARANCE_X / 2.0 + 8.0,
                0.0,
                GRIPPER_GAUGE_Z / 2.0 + 15.0,
            );
    }
    rails
}

fn gripper_approach_posts() -> Part {
    let mut posts = Part::empty("microplate_adapter_gripper_approach_posts");
    for (index, (x, y)) in [
        (-168.0, -44.0),
        (-168.0, 44.0),
        (168.0, -44.0),
        (168.0, 44.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("microplate_adapter_gripper_approach_post_{index}"),
                22.0,
                22.0,
                80.0,
            )
            .translate(x, y, GRIPPER_GAUGE_Z / 2.0 + 40.0);
    }
    posts
}

fn gripper_centerline_flags() -> Part {
    let mut flags = Part::empty("microplate_adapter_gripper_centerline_flags");
    for corridor in 0..GRIPPER_APPROACH_CORRIDORS {
        flags = flags
            + centered_cube(
                format!("microplate_adapter_gripper_corridor_{corridor}_centerline_flag"),
                90.0,
                3.0,
                6.0,
            )
            .translate(
                centered_index(corridor, GRIPPER_APPROACH_CORRIDORS, 128.0),
                0.0,
                GRIPPER_GAUGE_Z / 2.0 + 3.0,
            );
    }
    flags
}

fn incubator_rack_handoff_dock() -> Part {
    let plate = centered_cube(
        "microplate_adapter_incubator_rack_handoff_dock_plate",
        INCUBATOR_DOCK_X,
        INCUBATOR_DOCK_Y,
        INCUBATOR_DOCK_Z,
    );
    plate - incubator_dock_entry_reliefs()
        + incubator_rack_rails()
        + incubator_closed_gate_witness()
        + incubator_slot_identity_tabs()
}

fn incubator_dock_entry_reliefs() -> Part {
    centered_cube(
        "microplate_adapter_incubator_dock_module_entry_relief",
        SLAS_FOOTPRINT_X + 34.0,
        44.0,
        INCUBATOR_DOCK_Z + 4.0,
    )
    .translate(0.0, -INCUBATOR_DOCK_Y / 2.0 + 24.0, 0.0)
}

fn incubator_rack_rails() -> Part {
    let mut rails = Part::empty("microplate_adapter_incubator_rack_handoff_rails");
    for rail in 0..RACK_RAIL_COUNT {
        rails = rails
            + centered_cube(
                format!("microplate_adapter_incubator_rack_rail_{rail}"),
                INCUBATOR_DOCK_X - 56.0,
                10.0,
                24.0,
            )
            .translate(
                0.0,
                centered_index(rail, RACK_RAIL_COUNT, 74.0),
                INCUBATOR_DOCK_Z / 2.0 + 12.0,
            );
    }
    rails
}

fn incubator_closed_gate_witness() -> Part {
    let gate_bar = centered_cube(
        "microplate_adapter_incubator_closed_handoff_gate_bar",
        INCUBATOR_DOCK_X - 64.0,
        10.0,
        56.0,
    )
    .translate(
        0.0,
        INCUBATOR_DOCK_Y / 2.0 - 20.0,
        INCUBATOR_DOCK_Z / 2.0 + 28.0,
    );
    let witness_tabs = centered_cube(
        "microplate_adapter_incubator_gate_witness_tab_left",
        34.0,
        5.0,
        28.0,
    )
    .translate(
        -104.0,
        INCUBATOR_DOCK_Y / 2.0 - 10.0,
        INCUBATOR_DOCK_Z / 2.0 + 14.0,
    ) + centered_cube(
        "microplate_adapter_incubator_gate_witness_tab_right",
        34.0,
        5.0,
        28.0,
    )
    .translate(
        104.0,
        INCUBATOR_DOCK_Y / 2.0 - 10.0,
        INCUBATOR_DOCK_Z / 2.0 + 14.0,
    );
    gate_bar + witness_tabs
}

fn incubator_slot_identity_tabs() -> Part {
    let mut tabs = Part::empty("microplate_adapter_incubator_slot_identity_tabs");
    for tab in 0..RACK_SLOT_ID_COUNT {
        tabs = tabs
            + centered_cube(
                format!("microplate_adapter_incubator_slot_identity_tab_{tab}"),
                34.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(tab, RACK_SLOT_ID_COUNT, 48.0),
                -INCUBATOR_DOCK_Y / 2.0 + 16.0,
                INCUBATOR_DOCK_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn perfusion_manifold_handoff_gauge() -> Part {
    let body = centered_cube(
        "microplate_adapter_perfusion_manifold_handoff_gauge_body",
        PERFUSION_GAUGE_X,
        PERFUSION_GAUGE_Y,
        PERFUSION_GAUGE_Z,
    );
    body - manifold_port_bores()
        + manifold_port_collars()
        + manifold_strain_reliefs()
        + manifold_dry_break_key_bar()
}

fn manifold_port_bores() -> Part {
    let mut bores = Part::empty("microplate_adapter_perfusion_manifold_port_bores");
    for port in 0..MANIFOLD_PORT_COUNT {
        bores = bores
            + centered_cylinder(
                format!("microplate_adapter_perfusion_manifold_port_bore_{port}"),
                5.4,
                PERFUSION_GAUGE_Z + 4.0,
                STANDARD_CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(port, MANIFOLD_PORT_COUNT, MANIFOLD_PORT_PITCH),
                22.0,
                0.0,
            );
    }
    bores
}

fn manifold_port_collars() -> Part {
    let mut collars = Part::empty("microplate_adapter_perfusion_manifold_port_collars");
    for port in 0..MANIFOLD_PORT_COUNT {
        collars = collars
            + centered_cylinder(
                format!("microplate_adapter_perfusion_manifold_port_collar_{port}"),
                10.5,
                5.0,
                STANDARD_CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(port, MANIFOLD_PORT_COUNT, MANIFOLD_PORT_PITCH),
                22.0,
                PERFUSION_GAUGE_Z / 2.0 + 2.5,
            );
    }
    collars
}

fn manifold_strain_reliefs() -> Part {
    let mut reliefs = Part::empty("microplate_adapter_perfusion_manifold_strain_reliefs");
    for index in 0..MANIFOLD_STRAIN_RELIEF_COUNT {
        reliefs = reliefs
            + centered_cube(
                format!("microplate_adapter_perfusion_manifold_strain_relief_{index}"),
                62.0,
                12.0,
                18.0,
            )
            .translate(
                centered_index(index, MANIFOLD_STRAIN_RELIEF_COUNT, 82.0),
                -PERFUSION_GAUGE_Y / 2.0 + 24.0,
                PERFUSION_GAUGE_Z / 2.0 + 9.0,
            );
    }
    reliefs
}

fn manifold_dry_break_key_bar() -> Part {
    centered_cube(
        "microplate_adapter_perfusion_manifold_asymmetric_dry_break_key_bar",
        PERFUSION_GAUGE_X - 60.0,
        8.0,
        14.0,
    )
    .translate(
        0.0,
        PERFUSION_GAUGE_Y / 2.0 - 22.0,
        PERFUSION_GAUGE_Z / 2.0 + 7.0,
    )
}

fn closed_handoff_envelope_keepouts() -> Part {
    let front = centered_cube(
        "microplate_adapter_closed_handoff_front_robot_keepout_rail",
        SERVICE_KEEP_OUT_X,
        6.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -SERVICE_KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        "microplate_adapter_closed_handoff_rear_incubator_keepout_rail",
        SERVICE_KEEP_OUT_X,
        6.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, SERVICE_KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        "microplate_adapter_closed_handoff_left_service_keepout_rail",
        6.0,
        SERVICE_KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-SERVICE_KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "microplate_adapter_closed_handoff_right_manifold_keepout_rail",
        6.0,
        SERVICE_KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(SERVICE_KEEP_OUT_X / 2.0, 0.0, 0.0);
    front + rear + left + right + keepout_corner_posts()
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty("microplate_adapter_closed_handoff_keepout_posts");
    for (index, (x, y)) in [
        (-SERVICE_KEEP_OUT_X / 2.0, -SERVICE_KEEP_OUT_Y / 2.0),
        (-SERVICE_KEEP_OUT_X / 2.0, SERVICE_KEEP_OUT_Y / 2.0),
        (SERVICE_KEEP_OUT_X / 2.0, -SERVICE_KEEP_OUT_Y / 2.0),
        (SERVICE_KEEP_OUT_X / 2.0, SERVICE_KEEP_OUT_Y / 2.0),
        (-SERVICE_KEEP_OUT_X / 2.0, 0.0),
        (SERVICE_KEEP_OUT_X / 2.0, 0.0),
        (0.0, -SERVICE_KEEP_OUT_Y / 2.0),
        (0.0, SERVICE_KEEP_OUT_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("microplate_adapter_closed_handoff_keepout_post_{index}"),
                7.0,
                36.0,
                SMALL_CYLINDER_SEGMENTS,
            )
            .translate(x, y, 18.0);
    }
    posts
}

fn evidence_camera_fiducial_bridge() -> Part {
    let left_post = bridge_post("left").translate(
        -BRIDGE_X / 2.0 + BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let right_post = bridge_post("right").translate(
        BRIDGE_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "microplate_adapter_evidence_camera_fiducial_bridge_beam",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);
    left_post + right_post + beam + bridge_camera_lands() + bridge_fiducials()
}

fn bridge_post(side: &str) -> Part {
    centered_cube(
        format!("microplate_adapter_evidence_bridge_{side}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
}

fn bridge_camera_lands() -> Part {
    let mut lands = Part::empty("microplate_adapter_evidence_bridge_camera_lands");
    for camera in 0..CAMERA_LAND_COUNT {
        let x = centered_index(camera, CAMERA_LAND_COUNT, 230.0);
        let land = centered_cube(
            format!("microplate_adapter_evidence_camera_land_{camera}"),
            92.0,
            34.0,
            5.0,
        )
        .translate(x, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z + 2.5);
        let lens_bore = centered_cylinder(
            format!("microplate_adapter_evidence_camera_lens_bore_{camera}"),
            10.0,
            6.0,
            STANDARD_CYLINDER_SEGMENTS,
        )
        .translate(x, 0.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z + 2.5);
        lands = lands + (land - lens_bore);
    }
    lands
}

fn bridge_fiducials() -> Part {
    let mut fiducials = Part::empty("microplate_adapter_evidence_bridge_fiducials");
    for index in 0..BRIDGE_FIDUCIAL_COUNT {
        let x = centered_index(index, BRIDGE_FIDUCIAL_COUNT, 118.0);
        let y = if index % 2 == 0 {
            -BRIDGE_Y / 2.0 + 12.0
        } else {
            BRIDGE_Y / 2.0 - 12.0
        };
        fiducials = fiducials
            + fiducial_disc(format!(
                "microplate_adapter_evidence_bridge_fiducial_{index}"
            ))
            .translate(x, y, BRIDGE_POST_Z + BRIDGE_BEAM_Z + 3.0);
    }
    fiducials
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    let disc = centered_cylinder(format!("{name}_disc"), 9.0, 4.0, STANDARD_CYLINDER_SEGMENTS);
    let x_cut = centered_cube(format!("{name}_x_cut"), 16.0, 2.0, 5.0);
    let y_cut = centered_cube(format!("{name}_y_cut"), 2.0, 16.0, 5.0);
    disc - x_cut - y_cut
}

fn collision_sensitive_rects() -> [Rect; 11] {
    [
        Rect {
            name: "standard_footprint_datum_plate",
            center: DATUM_PLATE_CENTER,
            x: DATUM_PLATE_X,
            y: DATUM_PLATE_Y,
        },
        Rect {
            name: "swappable_module_nests",
            center: NEST_PANEL_CENTER,
            x: NEST_PANEL_X,
            y: NEST_PANEL_Y,
        },
        Rect {
            name: "blind_mate_connector_gauge",
            center: CONNECTOR_GAUGE_CENTER,
            x: CONNECTOR_GAUGE_X,
            y: CONNECTOR_GAUGE_Y,
        },
        Rect {
            name: "port_alignment_witness_lanes",
            center: PORT_LANE_CENTER,
            x: PORT_LANE_X,
            y: PORT_LANE_Y,
        },
        Rect {
            name: "leak_pressure_witness_pockets",
            center: LEAK_POCKET_CENTER,
            x: LEAK_POCKET_X,
            y: LEAK_POCKET_Y,
        },
        Rect {
            name: "barcode_custody_status_surfaces",
            center: STATUS_SURFACE_CENTER,
            x: STATUS_SURFACE_X,
            y: STATUS_SURFACE_Y,
        },
        Rect {
            name: "robot_gripper_approach_gauges",
            center: GRIPPER_GAUGE_CENTER,
            x: GRIPPER_GAUGE_X,
            y: GRIPPER_GAUGE_Y,
        },
        Rect {
            name: "perfusion_manifold_handoff_gauge",
            center: PERFUSION_GAUGE_CENTER,
            x: PERFUSION_GAUGE_X,
            y: PERFUSION_GAUGE_Y,
        },
        Rect {
            name: "incubator_rack_handoff_dock",
            center: INCUBATOR_DOCK_CENTER,
            x: INCUBATOR_DOCK_X,
            y: INCUBATOR_DOCK_Y,
        },
        Rect {
            name: "evidence_camera_fiducial_bridge",
            center: BRIDGE_CENTER,
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
        Rect {
            name: "closed_handoff_envelope_keepouts",
            center: (0.0, 0.0),
            x: SERVICE_KEEP_OUT_X,
            y: SERVICE_KEEP_OUT_Y,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_and_scoped() {
        let expected = [
            "output/closed_incubator_microplate_footprint_module_adapter_station_base_deck.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_standard_footprint_datum_plate.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_swappable_module_nests.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_fluidic_electrical_blind_mate_connector_gauge.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_port_alignment_witness_lanes.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_leak_pressure_witness_pockets.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_barcode_custody_status_surfaces.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_robot_gripper_approach_gauges.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_incubator_rack_handoff_dock.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_perfusion_manifold_handoff_gauge.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_closed_handoff_envelope_keepouts.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_evidence_camera_fiducial_bridge.stl",
            "output/closed_incubator_microplate_footprint_module_adapter_station_assembly.stl",
        ];
        assert_eq!(OUTPUTS, expected);

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for feature in [
            "standard_footprint_datum_plate",
            "swappable_module_nests",
            "fluidic_blind_mate_connector_gauge",
            "electrical_blind_mate_connector_gauge",
            "port_alignment_witness_lanes",
            "leak_pressure_witness_pockets",
            "barcode_custody_status_surfaces",
            "robot_gripper_approach_gauges",
            "incubator_rack_handoff_dock",
            "perfusion_manifold_handoff_gauge",
            "closed_handoff_envelope_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn feature_counts_cover_station_interfaces() {
        assert_eq!(SWAP_NEST_COUNT, 3);
        assert_eq!(NEST_DATUM_PINS_PER_NEST, 3);
        assert_eq!(FLUIDIC_PORT_COUNT, 6);
        assert_eq!(ELECTRICAL_PIN_COUNT, 24);
        assert_eq!(PORT_WITNESS_LANE_COUNT, 8);
        assert_eq!(LEAK_POCKET_COUNT, 6);
        assert_eq!(BARCODE_LAND_COUNT, 6);
        assert_eq!(CUSTODY_TOKEN_COUNT, 4);
        assert_eq!(STATUS_LANE_COUNT, 3);
        assert_eq!(GRIPPER_APPROACH_CORRIDORS, 2);
        assert_eq!(MANIFOLD_PORT_COUNT, 8);
    }

    #[test]
    fn microplate_footprint_and_nests_have_clear_datums() {
        assert!(SLAS_FOOTPRINT_X > 127.0);
        assert!(SLAS_FOOTPRINT_Y > 85.0);
        assert!(DATUM_PLATE_X > SLAS_FOOTPRINT_X + 110.0);
        assert!(DATUM_PLATE_Y > SLAS_FOOTPRINT_Y + 90.0);
        assert!(NEST_FRAME_X > SLAS_FOOTPRINT_X + 2.0 * MODULE_CLEARANCE);
        assert!(NEST_FRAME_Y > SLAS_FOOTPRINT_Y + 2.0 * MODULE_CLEARANCE);
        assert_eq!(FOOTPRINT_CORNER_TARGETS, 4);
        assert_eq!(SWAP_KEY_COUNT, SWAP_NEST_COUNT);
    }

    #[test]
    fn station_regions_fit_without_major_overlaps() {
        assert_design_constraints();

        let rects = collision_sensitive_rects();
        for rect in rects {
            assert!(rect.inside_station(8.0), "{} outside station", rect.name);
        }

        let non_overlay_count = rects.len() - 1;
        for i in 0..non_overlay_count {
            for j in (i + 1)..non_overlay_count {
                assert!(
                    !rects[i].overlaps_with_margin(rects[j], 12.0),
                    "{} overlaps {}",
                    rects[i].name,
                    rects[j].name
                );
            }
        }
    }

    #[test]
    fn closed_handoff_interfaces_are_segregated() {
        let perfusion = Rect {
            name: "perfusion",
            center: PERFUSION_GAUGE_CENTER,
            x: PERFUSION_GAUGE_X,
            y: PERFUSION_GAUGE_Y,
        };
        let incubator = Rect {
            name: "incubator",
            center: INCUBATOR_DOCK_CENTER,
            x: INCUBATOR_DOCK_X,
            y: INCUBATOR_DOCK_Y,
        };
        assert!(!perfusion.overlaps_with_margin(incubator, 12.0));
        assert_eq!(RACK_RAIL_COUNT, 2);
        assert_eq!(RACK_SLOT_ID_COUNT, 5);
        assert_eq!(MANIFOLD_STRAIN_RELIEF_COUNT, 4);
        assert!(SERVICE_KEEP_OUT_X < STATION_X);
        assert!(SERVICE_KEEP_OUT_Y < STATION_Y);
    }

    #[test]
    fn reproducibility_controls_are_explicit() {
        assert_eq!(UNIT_SYSTEM, "millimeter");
        assert_eq!(
            REPRODUCIBILITY_TAG,
            "slas_microplate_footprint_adapter_station_v1_fixed_mm"
        );
        assert!(!RANDOMIZED_GEOMETRY);
        assert_eq!(SMALL_CYLINDER_SEGMENTS, 24);
        assert_eq!(STANDARD_CYLINDER_SEGMENTS, 36);
        assert_eq!(ROUND_CYLINDER_SEGMENTS, 48);
    }
}
