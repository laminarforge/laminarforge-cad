use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag lot, thaw-order, and pooling-sequence interlock station.
//
// Research notes encoded into the fixture:
// - Closed-system bagged media and reagents are commonly integrated through
//   sterile welds, Luer locks, MPC-style connectors, or similar closed
//   connections, so the station models connector/key custody instead of open
//   pouring.
// - ATMP/GMP guidance emphasizes documented traceability, rejected/recovered
//   material control, qualification/validation, and batch-release custody. The
//   station therefore separates lot/COA identity, event timestamp/read-point
//   evidence, thaw exposure records, retain samples, and release/hold/reject
//   gates.
// - Cold-chain and thaw operations are sensitive to temperature excursions and
//   workflow variation; this CAD locks physical thaw order and pooling order so
//   automation can validate reproducible cell-culture preparation.
//
// Validation CAD only. It is not a GMP batch-record implementation, thaw SOP,
// sterile connection protocol, release specification, or biological performance
// claim.

const BIN_NAME: &str = "closed_media_bag_lot_thaw_pooling_order_interlock_station";
const OUTPUT_PREFIX: &str = "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_base_containment_deck.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_incoming_lot_identity_bag_nests.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_thaw_order_interlock_ladder.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_pooling_sequence_manifold_panel.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_barcode_coa_custody_bridge.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_temperature_exposure_logger_rail.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_hold_release_reject_gate_lanes.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_sample_retain_split_rack.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_sterile_connector_cap_custody_parks.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_pooled_bag_receiver_load_cell_cradle.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_robot_service_keepouts.stl",
    "output/closed_media_bag_lot_thaw_pooling_order_interlock_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "closed_media_bag_identity_nests",
    "lot_barcode_capture",
    "coa_custody_slots",
    "thaw_order_interlock_ladder",
    "temperature_exposure_logger_rail",
    "pooling_sequence_manifold",
    "pooling_order_tokens",
    "sterile_connector_cap_custody",
    "hold_release_reject_gates",
    "sample_retain_split_rack",
    "pooled_bag_load_cell_cradle",
    "barcode_coa_custody_bridge",
    "robot_keepout_gauges",
    "service_keepout_gauges",
];

const VALIDATION_LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "no_gmp_batch_record_implementation",
    "no_thaw_sop",
    "no_sterile_connection_protocol",
    "no_release_specification",
    "no_biological_performance_claim",
];

const REPRODUCIBILITY_CONTROLS: [&str; 5] = [
    "fixed_output_manifest_order",
    "parametric_constants_only",
    "fixed_cylinder_segment_counts",
    "integer_feature_counts",
    "no_random_or_time_inputs",
];

const CYL_SEGMENTS: u32 = 32;
const SMALL_CYL_SEGMENTS: u32 = 24;

const STATION_X: f64 = 1640.0;
const STATION_Y: f64 = 1040.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const EDGE_MARGIN: f64 = 16.0;
const SOCKET_DEPTH: f64 = 6.0;
const PAN_RECESS_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_BOSSES: usize = 8;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.8;

const LOT_NEST_CENTER: (f64, f64) = (-510.0, 235.0);
const LOT_NEST_X: f64 = 430.0;
const LOT_NEST_Y: f64 = 300.0;
const LOT_NEST_Z: f64 = 58.0;
const MEDIA_BAG_LOTS: usize = 3;
const LOT_BAG_SLOT_X: f64 = 112.0;
const LOT_BAG_SLOT_Y: f64 = 184.0;
const LOT_BAG_POCKET_DEPTH: f64 = 22.0;
const LOT_NEST_PITCH_X: f64 = 132.0;
const LOT_DATUM_PINS_PER_BAG: usize = 4;
const LOT_BARCODE_LANDS_PER_BAG: usize = 2;
const LOT_COA_TAG_SLOTS_PER_BAG: usize = 1;
const LOT_KEY_RAILS_PER_BAG: usize = 2;

const POOL_RECEIVER_CENTER: (f64, f64) = (510.0, 235.0);
const POOL_RECEIVER_X: f64 = 430.0;
const POOL_RECEIVER_Y: f64 = 300.0;
const POOL_RECEIVER_Z: f64 = 62.0;
const POOL_BAG_POCKET_X: f64 = 318.0;
const POOL_BAG_POCKET_Y: f64 = 192.0;
const POOL_BAG_POCKET_DEPTH: f64 = 25.0;
const POOL_LOAD_CELLS: usize = 4;
const POOL_MIXING_RIBS: usize = 7;
const POOL_FINAL_ID_LANDS: usize = 3;
const POOL_BAG_STRAP_BRIDGES: usize = 4;

const THAW_LADDER_CENTER: (f64, f64) = (-510.0, -90.0);
const THAW_LADDER_X: f64 = 430.0;
const THAW_LADDER_Y: f64 = 210.0;
const THAW_LADDER_Z: f64 = 42.0;
const THAW_STEPS: usize = MEDIA_BAG_LOTS;
const THAW_STAGGER_MINUTES: [usize; THAW_STEPS] = [0, 12, 24];
const THAW_TOKEN_SLOTS_PER_STEP: usize = 3;
const THAW_START_END_WELLS_PER_STEP: usize = 2;
const THAW_STEP_PITCH_X: f64 = 128.0;
const THAW_KEY_HEIGHTS: [f64; THAW_STEPS] = [18.0, 30.0, 42.0];

const POOL_SEQUENCE_CENTER: (f64, f64) = (0.0, -90.0);
const POOL_SEQUENCE_X: f64 = 420.0;
const POOL_SEQUENCE_Y: f64 = 210.0;
const POOL_SEQUENCE_Z: f64 = 50.0;
const POOL_INLET_CONNECTORS: usize = MEDIA_BAG_LOTS;
const POOL_SEQUENCE_TOKENS: usize = MEDIA_BAG_LOTS;
const POOL_NONRETURN_VALVES: usize = MEDIA_BAG_LOTS;
const POOL_WETNESS_WINDOWS: usize = MEDIA_BAG_LOTS;
const POOL_MERGE_CHANNELS: usize = MEDIA_BAG_LOTS;
const POOL_INLET_PITCH_X: f64 = 118.0;
const CONNECTOR_SOCKET_D: f64 = 30.0;

const CUSTODY_BRIDGE_CENTER: (f64, f64) = (0.0, 430.0);
const CUSTODY_BRIDGE_SPAN_X: f64 = 1320.0;
const CUSTODY_BRIDGE_Y: f64 = 70.0;
const CUSTODY_BRIDGE_UNDERSIDE_Z: f64 = 214.0;
const CUSTODY_BRIDGE_BEAM_Z: f64 = 32.0;
const CUSTODY_POST_X: f64 = 30.0;
const CUSTODY_POST_Y: f64 = 46.0;
const BARCODE_SCAN_WINDOWS: usize = 8;
const COA_CARD_SLOTS: usize = 4;
const CUSTODY_SEAL_WELLS: usize = 6;
const EVENT_TIME_LANDS: usize = 3;
const READ_POINT_LANDS: usize = 3;
const CAMERA_PODS: usize = 4;
const LIGHT_BARS: usize = 4;

const TEMP_RAIL_CENTER: (f64, f64) = (-510.0, -355.0);
const TEMP_RAIL_X: f64 = 430.0;
const TEMP_RAIL_Y: f64 = 150.0;
const TEMP_RAIL_Z: f64 = 38.0;
const TEMPERATURE_LOGGER_POCKETS: usize = 4;
const EXPOSURE_TOKEN_SLOTS: usize = 8;
const EXCURSION_FLAG_WELLS: usize = 6;
const LOGGER_POCKET_X: f64 = 72.0;
const LOGGER_POCKET_Y: f64 = 42.0;
const LOGGER_POCKET_DEPTH: f64 = 18.0;

const GATE_CENTER: (f64, f64) = (510.0, -90.0);
const GATE_PANEL_X: f64 = 390.0;
const GATE_PANEL_Y: f64 = 210.0;
const GATE_PANEL_Z: f64 = 42.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;
const GATE_SLIDERS: usize = DISPOSITION_LANES;
const GATE_LANE_PITCH_X: f64 = 116.0;
const GATE_SLOT_X: f64 = 82.0;
const GATE_SLOT_Y: f64 = 42.0;
const GATE_SLOT_DEPTH: f64 = 8.0;
const MIN_DISPOSITION_LANE_GAP: f64 = 28.0;

const RETAIN_CENTER: (f64, f64) = (0.0, -355.0);
const RETAIN_RACK_X: f64 = 420.0;
const RETAIN_RACK_Y: f64 = 150.0;
const RETAIN_RACK_Z: f64 = 44.0;
const RETAIN_BRANCHES: usize = 4;
const RETAIN_VIAL_WELLS: usize = 12;
const RETAIN_SPLIT_VALVES: usize = 4;
const RETAIN_CHAIN_SEAL_SLOTS: usize = 6;
const RETAIN_VIAL_D: f64 = 20.0;

const CONNECTOR_PARK_CENTER: (f64, f64) = (510.0, -355.0);
const CONNECTOR_PARK_X: f64 = 390.0;
const CONNECTOR_PARK_Y: f64 = 150.0;
const CONNECTOR_PARK_Z: f64 = 42.0;
const STERILE_CONNECTOR_PARKS: usize = 8;
const CAP_CUSTODY_WELLS: usize = 12;
const USED_CAP_QUARANTINE_SLOTS: usize = 3;
const CAP_WELL_D: f64 = 23.0;
const CONNECTOR_PARK_D: f64 = 28.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 430.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 310.0;
const LEFT_THAW_SERVICE_KEEP_OUT_X: f64 = 285.0;
const RIGHT_POOL_SERVICE_KEEP_OUT_X: f64 = 285.0;
const OVERHEAD_BAG_LIFT_CLEARANCE_Z: f64 = 365.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;
const KEEP_OUT_GROUPS: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - EDGE_MARGIN;
        let usable_y = STATION_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }

    fn horizontal_gap(self, other: Rect) -> f64 {
        (self.center.0 - other.center.0).abs() - (self.x + other.x) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let lot_nests = incoming_lot_identity_bag_nests().translate(
        LOT_NEST_CENTER.0,
        LOT_NEST_CENTER.1,
        deck_insert_z(LOT_NEST_Z),
    );
    export(OUTPUTS[1], &lot_nests);

    let thaw_ladder = thaw_order_interlock_ladder().translate(
        THAW_LADDER_CENTER.0,
        THAW_LADDER_CENTER.1,
        deck_insert_z(THAW_LADDER_Z),
    );
    export(OUTPUTS[2], &thaw_ladder);

    let pooling_panel = pooling_sequence_manifold_panel().translate(
        POOL_SEQUENCE_CENTER.0,
        POOL_SEQUENCE_CENTER.1,
        deck_insert_z(POOL_SEQUENCE_Z),
    );
    export(OUTPUTS[3], &pooling_panel);

    let custody_bridge = barcode_coa_custody_bridge().translate(
        CUSTODY_BRIDGE_CENTER.0,
        CUSTODY_BRIDGE_CENTER.1,
        BASE_Z,
    );
    export(OUTPUTS[4], &custody_bridge);

    let temp_rail = temperature_exposure_logger_rail().translate(
        TEMP_RAIL_CENTER.0,
        TEMP_RAIL_CENTER.1,
        deck_insert_z(TEMP_RAIL_Z),
    );
    export(OUTPUTS[5], &temp_rail);

    let gate_lanes = hold_release_reject_gate_lanes().translate(
        GATE_CENTER.0,
        GATE_CENTER.1,
        deck_insert_z(GATE_PANEL_Z),
    );
    export(OUTPUTS[6], &gate_lanes);

    let retain_rack = sample_retain_split_rack().translate(
        RETAIN_CENTER.0,
        RETAIN_CENTER.1,
        deck_insert_z(RETAIN_RACK_Z),
    );
    export(OUTPUTS[7], &retain_rack);

    let connector_parks = sterile_connector_cap_custody_parks().translate(
        CONNECTOR_PARK_CENTER.0,
        CONNECTOR_PARK_CENTER.1,
        deck_insert_z(CONNECTOR_PARK_Z),
    );
    export(OUTPUTS[8], &connector_parks);

    let pool_receiver = pooled_bag_receiver_load_cell_cradle().translate(
        POOL_RECEIVER_CENTER.0,
        POOL_RECEIVER_CENTER.1,
        deck_insert_z(POOL_RECEIVER_Z),
    );
    export(OUTPUTS[9], &pool_receiver);

    let keepouts = robot_service_keepouts().translate(0.0, 0.0, BASE_Z + 8.0);
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + lot_nests
        + thaw_ladder
        + pooling_panel
        + custody_bridge
        + temp_rail
        + gate_lanes
        + retain_rack
        + connector_parks
        + pool_receiver
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media-bag lot/thaw/pooling interlock station:");
    println!("  Output prefix:          {OUTPUT_PREFIX}");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {RIM_Z:.0}mm rim"
    );
    println!(
        "  Lot identity:           {MEDIA_BAG_LOTS} bag nests, {} lot barcode lands, {} COA tag slots, {} datum pins",
        MEDIA_BAG_LOTS * LOT_BARCODE_LANDS_PER_BAG,
        MEDIA_BAG_LOTS * LOT_COA_TAG_SLOTS_PER_BAG,
        MEDIA_BAG_LOTS * LOT_DATUM_PINS_PER_BAG
    );
    println!(
        "  Thaw order:             {THAW_STEPS} keyed steps at T+{:02}/T+{:02}/T+{:02} min with {} token slots",
        THAW_STAGGER_MINUTES[0],
        THAW_STAGGER_MINUTES[1],
        THAW_STAGGER_MINUTES[2],
        THAW_STEPS * THAW_TOKEN_SLOTS_PER_STEP
    );
    println!(
        "  Pooling sequence:       {POOL_INLET_CONNECTORS} keyed inlet connectors, {POOL_SEQUENCE_TOKENS} order tokens, {POOL_NONRETURN_VALVES} non-return valve pockets"
    );
    println!(
        "  Custody evidence:       {BARCODE_SCAN_WINDOWS} scan windows, {COA_CARD_SLOTS} COA slots, {CUSTODY_SEAL_WELLS} seal wells, {} event/read-point lands",
        EVENT_TIME_LANDS + READ_POINT_LANDS
    );
    println!(
        "  Exposure and gates:     {TEMPERATURE_LOGGER_POCKETS} logger pockets, {EXPOSURE_TOKEN_SLOTS} exposure tokens, {DISPOSITION_LANES} hold/release/reject lanes"
    );
    println!(
        "  Retains/connectors:     {RETAIN_BRANCHES} retain branches, {RETAIN_VIAL_WELLS} retain vial wells, {STERILE_CONNECTOR_PARKS} connector parks, {CAP_CUSTODY_WELLS} cap custody wells"
    );
    println!(
        "  Robot/service keepouts: front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, side service {LEFT_THAW_SERVICE_KEEP_OUT_X:.0}/{RIGHT_POOL_SERVICE_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_BAG_LIFT_CLEARANCE_Z:.0}mm"
    );
    println!(
        "  Limitations:            validation fixture only; no GMP batch record, thaw SOP, sterile connection protocol, release specification, or biological performance claim"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn name(suffix: &str) -> String {
    format!("{BIN_NAME}_{suffix}")
}

fn deck_insert_z(part_z: f64) -> f64 {
    BASE_Z + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn grid_xy(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn deck_module_rects() -> [Rect; 8] {
    [
        rect(
            "incoming_lot_identity_bag_nests",
            LOT_NEST_CENTER,
            LOT_NEST_X,
            LOT_NEST_Y,
        ),
        rect(
            "pooled_bag_receiver_load_cell_cradle",
            POOL_RECEIVER_CENTER,
            POOL_RECEIVER_X,
            POOL_RECEIVER_Y,
        ),
        rect(
            "thaw_order_interlock_ladder",
            THAW_LADDER_CENTER,
            THAW_LADDER_X,
            THAW_LADDER_Y,
        ),
        rect(
            "pooling_sequence_manifold_panel",
            POOL_SEQUENCE_CENTER,
            POOL_SEQUENCE_X,
            POOL_SEQUENCE_Y,
        ),
        rect(
            "temperature_exposure_logger_rail",
            TEMP_RAIL_CENTER,
            TEMP_RAIL_X,
            TEMP_RAIL_Y,
        ),
        rect(
            "hold_release_reject_gate_lanes",
            GATE_CENTER,
            GATE_PANEL_X,
            GATE_PANEL_Y,
        ),
        rect(
            "sample_retain_split_rack",
            RETAIN_CENTER,
            RETAIN_RACK_X,
            RETAIN_RACK_Y,
        ),
        rect(
            "sterile_connector_cap_custody_parks",
            CONNECTOR_PARK_CENTER,
            CONNECTOR_PARK_X,
            CONNECTOR_PARK_Y,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));

    for (i, path) in OUTPUTS.iter().enumerate() {
        for other in OUTPUTS.iter().skip(i + 1) {
            assert_ne!(path, other, "duplicate output path");
        }
    }

    for feature in [
        "closed_media_bag_identity_nests",
        "lot_barcode_capture",
        "coa_custody_slots",
        "thaw_order_interlock_ladder",
        "temperature_exposure_logger_rail",
        "pooling_sequence_manifold",
        "pooling_order_tokens",
        "sterile_connector_cap_custody",
        "hold_release_reject_gates",
        "sample_retain_split_rack",
        "pooled_bag_load_cell_cradle",
        "barcode_coa_custody_bridge",
        "robot_keepout_gauges",
        "service_keepout_gauges",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "validation_fixture_only",
        "no_gmp_batch_record_implementation",
        "no_thaw_sop",
        "no_sterile_connection_protocol",
        "no_release_specification",
        "no_biological_performance_claim",
    ] {
        assert!(VALIDATION_LIMITATIONS.contains(&limitation));
    }

    for control in [
        "fixed_output_manifest_order",
        "parametric_constants_only",
        "fixed_cylinder_segment_counts",
        "integer_feature_counts",
        "no_random_or_time_inputs",
    ] {
        assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
    }

    assert_eq!(CYL_SEGMENTS, 32);
    assert_eq!(SMALL_CYL_SEGMENTS, 24);
    assert_eq!(MEDIA_BAG_LOTS, THAW_STEPS);
    assert_eq!(POOL_INLET_CONNECTORS, MEDIA_BAG_LOTS);
    assert_eq!(POOL_SEQUENCE_TOKENS, MEDIA_BAG_LOTS);
    assert_eq!(POOL_NONRETURN_VALVES, MEDIA_BAG_LOTS);
    assert_eq!(POOL_WETNESS_WINDOWS, MEDIA_BAG_LOTS);
    assert_eq!(POOL_MERGE_CHANNELS, MEDIA_BAG_LOTS);
    assert_eq!(MOUNT_BOSSES, 8);
    assert_eq!(GATE_SLIDERS, DISPOSITION_LANES);
    assert_eq!(THAW_START_END_WELLS_PER_STEP, 2);
    assert_eq!(RETAIN_SPLIT_VALVES, RETAIN_BRANCHES);
    assert!(THAW_STAGGER_MINUTES.windows(2).all(|w| w[0] < w[1]));
    assert!(THAW_KEY_HEIGHTS.windows(2).all(|w| w[0] < w[1]));
    assert!(GATE_LANE_PITCH_X - GATE_SLOT_X >= MIN_DISPOSITION_LANE_GAP);
    assert!(CUSTODY_BRIDGE_UNDERSIDE_Z > POOL_RECEIVER_Z + BASE_Z + 90.0);
    assert!(OVERHEAD_BAG_LIFT_CLEARANCE_Z > CUSTODY_BRIDGE_UNDERSIDE_Z);
    assert!(POOL_BAG_POCKET_DEPTH < POOL_RECEIVER_Z);
    assert!(LOT_BAG_POCKET_DEPTH < LOT_NEST_Z);

    let rects = deck_module_rects();
    assert!(
        rects[0].horizontal_gap(rects[1]) > 500.0,
        "incoming and pooled bag zones must stay physically separated"
    );
    for module in rects {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds usable station deck",
            module.name
        );
    }

    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }

    let custody_rect = rect(
        "barcode_coa_custody_bridge",
        CUSTODY_BRIDGE_CENTER,
        CUSTODY_BRIDGE_SPAN_X,
        CUSTODY_BRIDGE_Y,
    );
    assert!(custody_rect.fits_inside_deck());
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(name("base_containment_deck"), STATION_X, STATION_Y, BASE_Z)
        .translate(0.0, 0.0, BASE_Z / 2.0);
    let pan_recess = centered_cube(
        name("base_shallow_spill_pan_cut"),
        STATION_X - 130.0,
        STATION_Y - 136.0,
        PAN_RECESS_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - PAN_RECESS_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        name("base_front_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        64.0,
        CYL_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 96.0,
        -STATION_Y / 2.0 + 20.0,
        BASE_Z - 6.0,
    );

    deck - pan_recess - drain + perimeter_rim() + mount_bosses() + deck_workflow_lanes()
}

fn perimeter_rim() -> Part {
    let front = centered_cube(name("front_raised_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("rear_raised_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(name("left_raised_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(name("right_raised_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty(name("mount_bosses"));
    for (i, (x, y)) in mount_positions().iter().copied().enumerate() {
        let boss = centered_cylinder(
            name(&format!("mount_boss_{i}")),
            MOUNT_BOSS_D / 2.0,
            8.0,
            CYL_SEGMENTS,
        )
        .translate(x, y, BASE_Z + 4.0);
        let hole = centered_cylinder(
            name(&format!("mount_hole_cut_{i}")),
            MOUNT_HOLE_D / 2.0,
            14.0,
            SMALL_CYL_SEGMENTS,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn mount_positions() -> [(f64, f64); MOUNT_BOSSES] {
    [
        (-730.0, -450.0),
        (-730.0, 450.0),
        (730.0, -450.0),
        (730.0, 450.0),
        (-250.0, -450.0),
        (250.0, 450.0),
        (0.0, -450.0),
        (0.0, 450.0),
    ]
}

fn deck_workflow_lanes() -> Part {
    let mut lanes = Part::empty(name("deck_workflow_lanes"));
    for i in 0..7 {
        lanes = lanes
            + centered_cube(
                name(&format!("deck_front_to_back_lane_marker_{i}")),
                3.0,
                790.0,
                3.0,
            )
            .translate(centered_index(i, 7, 210.0), -20.0, BASE_Z + 1.5);
    }
    for (i, y) in [-250.0, 60.0, 395.0].iter().copied().enumerate() {
        lanes = lanes
            + centered_cube(
                name(&format!("deck_cross_stage_marker_{i}")),
                1380.0,
                4.0,
                3.0,
            )
            .translate(0.0, y, BASE_Z + 1.5);
    }
    lanes
}

fn incoming_lot_identity_bag_nests() -> Part {
    let mut panel = centered_cube(
        name("incoming_lot_identity_panel"),
        LOT_NEST_X,
        LOT_NEST_Y,
        LOT_NEST_Z,
    );
    let containment = centered_cube(
        name("incoming_lot_panel_drain_channel_cut"),
        LOT_NEST_X - 38.0,
        18.0,
        12.0,
    )
    .translate(0.0, -LOT_NEST_Y / 2.0 + 25.0, LOT_NEST_Z / 2.0 - 4.0);
    panel = panel - containment;

    for i in 0..MEDIA_BAG_LOTS {
        let x = centered_index(i, MEDIA_BAG_LOTS, LOT_NEST_PITCH_X);
        panel = panel + lot_bag_nest(i, x);
    }

    panel + lot_panel_rear_stop() + lot_panel_front_custody_strip()
}

fn lot_bag_nest(index: usize, x: f64) -> Part {
    let pocket = centered_cube(
        name(&format!("lot_{index}_bag_pocket_cut")),
        LOT_BAG_SLOT_X,
        LOT_BAG_SLOT_Y,
        LOT_BAG_POCKET_DEPTH,
    )
    .translate(x, 18.0, LOT_NEST_Z / 2.0 + 1.0);
    let tube_exit = centered_cube(
        name(&format!("lot_{index}_tube_exit_channel_cut")),
        22.0,
        LOT_NEST_Y / 2.0,
        16.0,
    )
    .translate(x, -LOT_NEST_Y / 2.0 + 55.0, LOT_NEST_Z / 2.0 + 1.5);
    let mut nest =
        Part::empty(name(&format!("lot_{index}_bag_nest_features"))) - pocket - tube_exit;

    for pin in 0..LOT_DATUM_PINS_PER_BAG {
        let (px, py) = grid_xy(pin, 2, 2, LOT_BAG_SLOT_X - 26.0, LOT_BAG_SLOT_Y - 44.0);
        nest = nest
            + centered_cylinder(
                name(&format!("lot_{index}_datum_pin_{pin}")),
                4.0,
                12.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(x + px, 18.0 + py, LOT_NEST_Z / 2.0 + 4.0);
    }

    for rail in 0..LOT_KEY_RAILS_PER_BAG {
        let rail_y = 18.0 + centered_index(rail, LOT_KEY_RAILS_PER_BAG, 72.0);
        nest = nest
            + centered_cube(
                name(&format!("lot_{index}_asymmetric_key_rail_{rail}")),
                LOT_BAG_SLOT_X - 18.0,
                6.0,
                10.0 + index as f64 * 5.0,
            )
            .translate(x, rail_y, LOT_NEST_Z / 2.0 + 5.0 + index as f64 * 2.5);
    }

    for land in 0..LOT_BARCODE_LANDS_PER_BAG {
        nest = nest
            + barcode_land(
                &format!("lot_{index}_barcode_land_{land}"),
                72.0,
                22.0,
                7 + index + land,
            )
            .translate(
                x,
                -LOT_NEST_Y / 2.0 + 30.0 + land as f64 * 30.0,
                LOT_NEST_Z / 2.0 + 4.0,
            );
    }

    for slot in 0..LOT_COA_TAG_SLOTS_PER_BAG {
        nest = nest
            + centered_cube(
                name(&format!("lot_{index}_coa_tag_capture_slot_{slot}")),
                78.0,
                16.0,
                7.0,
            )
            .translate(
                x,
                LOT_NEST_Y / 2.0 - 32.0 - slot as f64 * 24.0,
                LOT_NEST_Z / 2.0 + 4.0,
            );
    }

    nest
}

fn lot_panel_rear_stop() -> Part {
    centered_cube(
        name("incoming_lot_rear_scan_stop"),
        LOT_NEST_X - 26.0,
        12.0,
        38.0,
    )
    .translate(0.0, LOT_NEST_Y / 2.0 - 12.0, LOT_NEST_Z / 2.0 + 19.0)
}

fn lot_panel_front_custody_strip() -> Part {
    let mut strip = centered_cube(
        name("incoming_lot_front_lot_event_strip"),
        LOT_NEST_X - 34.0,
        24.0,
        9.0,
    )
    .translate(0.0, -LOT_NEST_Y / 2.0 + 16.0, LOT_NEST_Z / 2.0 + 4.5);
    for i in 0..MEDIA_BAG_LOTS {
        strip = strip
            + raised_label_land(&format!("incoming_lot_status_tick_{i}"), 42.0, 15.0, i + 2)
                .translate(
                    centered_index(i, MEDIA_BAG_LOTS, LOT_NEST_PITCH_X),
                    -LOT_NEST_Y / 2.0 + 16.0,
                    LOT_NEST_Z / 2.0 + 11.0,
                );
    }
    strip
}

fn pooled_bag_receiver_load_cell_cradle() -> Part {
    let mut cradle = centered_cube(
        name("pooled_bag_receiver_load_cell_cradle"),
        POOL_RECEIVER_X,
        POOL_RECEIVER_Y,
        POOL_RECEIVER_Z,
    );
    let pocket = centered_cube(
        name("pooled_bag_receiver_pocket_cut"),
        POOL_BAG_POCKET_X,
        POOL_BAG_POCKET_Y,
        POOL_BAG_POCKET_DEPTH,
    )
    .translate(0.0, 20.0, POOL_RECEIVER_Z / 2.0 + 1.0);
    let outlet_channel = centered_cube(
        name("pooled_bag_receiver_outlet_channel_cut"),
        34.0,
        POOL_RECEIVER_Y / 2.0,
        18.0,
    )
    .translate(
        0.0,
        -POOL_RECEIVER_Y / 2.0 + 56.0,
        POOL_RECEIVER_Z / 2.0 + 1.5,
    );
    cradle = cradle - pocket - outlet_channel;

    for i in 0..POOL_LOAD_CELLS {
        let (x, y) = grid_xy(i, 2, 2, POOL_BAG_POCKET_X - 72.0, POOL_BAG_POCKET_Y - 62.0);
        cradle = cradle
            + centered_cube(
                name(&format!("pooled_bag_load_cell_pad_{i}")),
                64.0,
                44.0,
                10.0,
            )
            .translate(x, 20.0 + y, POOL_RECEIVER_Z / 2.0 + 5.0);
    }

    for i in 0..POOL_MIXING_RIBS {
        cradle = cradle
            + centered_cube(
                name(&format!("pooled_bag_low_shear_mixing_rib_{i}")),
                5.0,
                164.0,
                9.0,
            )
            .rotate(0.0, 0.0, -18.0)
            .translate(
                centered_index(i, POOL_MIXING_RIBS, 42.0),
                20.0,
                POOL_RECEIVER_Z / 2.0 + 4.5,
            );
    }

    for i in 0..POOL_BAG_STRAP_BRIDGES {
        cradle = cradle
            + centered_cube(
                name(&format!("pooled_bag_strap_bridge_{i}")),
                POOL_BAG_POCKET_X + 40.0,
                8.0,
                18.0,
            )
            .translate(
                0.0,
                20.0 + centered_index(i, POOL_BAG_STRAP_BRIDGES, 48.0),
                POOL_RECEIVER_Z / 2.0 + 9.0,
            );
    }

    for i in 0..POOL_FINAL_ID_LANDS {
        cradle = cradle
            + barcode_land(
                &format!("pooled_bag_final_lot_identity_land_{i}"),
                82.0,
                24.0,
                8 + i,
            )
            .translate(
                centered_index(i, POOL_FINAL_ID_LANDS, 100.0),
                POOL_RECEIVER_Y / 2.0 - 28.0,
                POOL_RECEIVER_Z / 2.0 + 4.0,
            );
    }

    cradle
}

fn thaw_order_interlock_ladder() -> Part {
    let mut ladder = centered_cube(
        name("thaw_order_interlock_ladder_panel"),
        THAW_LADDER_X,
        THAW_LADDER_Y,
        THAW_LADDER_Z,
    );
    for i in 0..THAW_STEPS {
        let x = centered_index(i, THAW_STEPS, THAW_STEP_PITCH_X);
        ladder = ladder + thaw_step_interlock(i, x);
    }
    ladder + thaw_one_way_spine()
}

fn thaw_step_interlock(index: usize, x: f64) -> Part {
    let mut step = Part::empty(name(&format!("thaw_step_{index}_interlock")));
    let slot = centered_cube(
        name(&format!("thaw_step_{index}_token_lane_cut")),
        92.0,
        66.0,
        8.0,
    )
    .translate(x, 24.0, THAW_LADDER_Z / 2.0 + 1.0);
    step = step - slot;

    for token in 0..THAW_TOKEN_SLOTS_PER_STEP {
        step = step
            + centered_cube(
                name(&format!("thaw_step_{index}_order_token_detent_{token}")),
                24.0,
                36.0,
                7.0,
            )
            .translate(
                x + centered_index(token, THAW_TOKEN_SLOTS_PER_STEP, 30.0),
                25.0,
                THAW_LADDER_Z / 2.0 + 3.5,
            );
    }

    for well in 0..THAW_START_END_WELLS_PER_STEP {
        step = step
            + centered_cylinder(
                name(&format!("thaw_step_{index}_start_end_witness_well_{well}")),
                12.0,
                8.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(
                x + centered_index(well, THAW_START_END_WELLS_PER_STEP, 44.0),
                -55.0,
                THAW_LADDER_Z / 2.0 + 4.0,
            );
    }

    let key = centered_cube(
        name(&format!("thaw_step_{index}_height_coded_key_stop")),
        76.0,
        8.0,
        THAW_KEY_HEIGHTS[index],
    )
    .translate(
        x,
        THAW_LADDER_Y / 2.0 - 26.0,
        THAW_LADDER_Z / 2.0 + THAW_KEY_HEIGHTS[index] / 2.0,
    );
    let time_land = raised_label_land(
        &format!(
            "thaw_step_{index}_time_land_t_plus_{}",
            THAW_STAGGER_MINUTES[index]
        ),
        70.0,
        18.0,
        index + 3,
    )
    .translate(x, -THAW_LADDER_Y / 2.0 + 24.0, THAW_LADDER_Z / 2.0 + 4.0);

    step + key + time_land
}

fn thaw_one_way_spine() -> Part {
    let rail = centered_cube(
        name("thaw_order_one_way_spine"),
        THAW_LADDER_X - 58.0,
        10.0,
        18.0,
    )
    .translate(0.0, 0.0, THAW_LADDER_Z / 2.0 + 9.0);
    let mut pawls = Part::empty(name("thaw_order_pawl_indicators"));
    for i in 0..THAW_STEPS {
        pawls = pawls
            + centered_cube(name(&format!("thaw_order_pawl_{i}")), 28.0, 8.0, 20.0)
                .rotate(0.0, 0.0, 18.0)
                .translate(
                    centered_index(i, THAW_STEPS, THAW_STEP_PITCH_X),
                    0.0,
                    THAW_LADDER_Z / 2.0 + 10.0,
                );
    }
    rail + pawls
}

fn pooling_sequence_manifold_panel() -> Part {
    let mut panel = centered_cube(
        name("pooling_sequence_manifold_panel"),
        POOL_SEQUENCE_X,
        POOL_SEQUENCE_Y,
        POOL_SEQUENCE_Z,
    );
    let manifold_channel = centered_cube(
        name("pooling_sequence_common_manifold_channel_cut"),
        POOL_SEQUENCE_X - 90.0,
        18.0,
        16.0,
    )
    .translate(0.0, -22.0, POOL_SEQUENCE_Z / 2.0 + 1.0);
    panel = panel - manifold_channel;

    for i in 0..POOL_INLET_CONNECTORS {
        let x = centered_index(i, POOL_INLET_CONNECTORS, POOL_INLET_PITCH_X);
        panel = panel + pooling_inlet_station(i, x);
    }

    let outlet = centered_cylinder(
        name("pooling_sequence_pooled_outlet_socket"),
        19.0,
        16.0,
        CYL_SEGMENTS,
    )
    .translate(
        0.0,
        -POOL_SEQUENCE_Y / 2.0 + 34.0,
        POOL_SEQUENCE_Z / 2.0 + 8.0,
    );
    let outlet_key = centered_cube(
        name("pooling_sequence_outlet_asymmetric_key"),
        86.0,
        10.0,
        18.0,
    )
    .translate(
        0.0,
        -POOL_SEQUENCE_Y / 2.0 + 66.0,
        POOL_SEQUENCE_Z / 2.0 + 9.0,
    );

    panel + outlet + outlet_key
}

fn pooling_inlet_station(index: usize, x: f64) -> Part {
    let socket = centered_cylinder(
        name(&format!("pooling_inlet_{index}_connector_socket")),
        CONNECTOR_SOCKET_D / 2.0,
        12.0,
        CYL_SEGMENTS,
    )
    .translate(x, 58.0, POOL_SEQUENCE_Z / 2.0 + 6.0);
    let key = centered_cube(
        name(&format!("pooling_inlet_{index}_order_key_fin")),
        10.0,
        44.0,
        16.0 + index as f64 * 6.0,
    )
    .translate(
        x + CONNECTOR_SOCKET_D / 2.0 + 10.0,
        58.0,
        POOL_SEQUENCE_Z / 2.0 + 8.0 + index as f64 * 3.0,
    );
    let valve = centered_cube(
        name(&format!("pooling_inlet_{index}_nonreturn_valve_pocket")),
        46.0,
        28.0,
        9.0,
    )
    .translate(x, 2.0, POOL_SEQUENCE_Z / 2.0 + 4.5);
    let wetness = centered_cube(
        name(&format!("pooling_inlet_{index}_wetness_window")),
        54.0,
        20.0,
        6.0,
    )
    .translate(x, -50.0, POOL_SEQUENCE_Z / 2.0 + 3.0);
    let token = centered_cube(
        name(&format!("pooling_inlet_{index}_sequence_token_slot")),
        50.0,
        18.0,
        7.0,
    )
    .translate(x, POOL_SEQUENCE_Y / 2.0 - 24.0, POOL_SEQUENCE_Z / 2.0 + 3.5);

    socket + key + valve + wetness + token
}

fn barcode_coa_custody_bridge() -> Part {
    let left_post = centered_cube(
        name("custody_bridge_left_post"),
        CUSTODY_POST_X,
        CUSTODY_POST_Y,
        CUSTODY_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -CUSTODY_BRIDGE_SPAN_X / 2.0 + 52.0,
        0.0,
        CUSTODY_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        name("custody_bridge_right_post"),
        CUSTODY_POST_X,
        CUSTODY_POST_Y,
        CUSTODY_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        CUSTODY_BRIDGE_SPAN_X / 2.0 - 52.0,
        0.0,
        CUSTODY_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        name("custody_bridge_overhead_scan_beam"),
        CUSTODY_BRIDGE_SPAN_X,
        CUSTODY_BRIDGE_Y,
        CUSTODY_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        CUSTODY_BRIDGE_UNDERSIDE_Z + CUSTODY_BRIDGE_BEAM_Z / 2.0,
    );

    left_post
        + right_post
        + beam
        + custody_scan_windows()
        + custody_card_slots()
        + custody_camera_pods()
        + custody_light_bars()
}

fn custody_scan_windows() -> Part {
    let mut windows = Part::empty(name("custody_bridge_barcode_scan_windows"));
    for i in 0..BARCODE_SCAN_WINDOWS {
        windows = windows
            + barcode_land(
                &format!("custody_bridge_barcode_scan_window_{i}"),
                78.0,
                20.0,
                5 + i,
            )
            .translate(
                centered_index(i, BARCODE_SCAN_WINDOWS, 124.0),
                -20.0,
                CUSTODY_BRIDGE_UNDERSIDE_Z + 18.0,
            );
    }
    windows
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty(name("custody_bridge_coa_event_slots"));
    for i in 0..COA_CARD_SLOTS {
        slots = slots
            + centered_cube(
                name(&format!("custody_bridge_coa_card_slot_{i}")),
                86.0,
                18.0,
                6.0,
            )
            .translate(
                centered_index(i, COA_CARD_SLOTS, 120.0),
                18.0,
                CUSTODY_BRIDGE_UNDERSIDE_Z + 18.0,
            );
    }
    for i in 0..CUSTODY_SEAL_WELLS {
        slots = slots
            + centered_cylinder(
                name(&format!("custody_bridge_tamper_seal_well_{i}")),
                7.0,
                6.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_WELLS, 64.0),
                39.0,
                CUSTODY_BRIDGE_UNDERSIDE_Z + 18.0,
            );
    }
    for i in 0..EVENT_TIME_LANDS {
        slots = slots
            + raised_label_land(&format!("custody_event_time_land_{i}"), 58.0, 18.0, i + 3)
                .translate(
                    -520.0 + i as f64 * 72.0,
                    0.0,
                    CUSTODY_BRIDGE_UNDERSIDE_Z + 18.0,
                );
    }
    for i in 0..READ_POINT_LANDS {
        slots = slots
            + raised_label_land(&format!("custody_read_point_land_{i}"), 58.0, 18.0, i + 4)
                .translate(
                    376.0 + i as f64 * 72.0,
                    0.0,
                    CUSTODY_BRIDGE_UNDERSIDE_Z + 18.0,
                );
    }
    slots
}

fn custody_camera_pods() -> Part {
    let mut pods = Part::empty(name("custody_bridge_camera_pods"));
    for i in 0..CAMERA_PODS {
        let x = centered_index(i, CAMERA_PODS, 300.0);
        let body = centered_cube(
            name(&format!("custody_camera_pod_body_{i}")),
            58.0,
            34.0,
            24.0,
        )
        .translate(x, 0.0, CUSTODY_BRIDGE_UNDERSIDE_Z - 12.0);
        let lens = centered_cylinder(
            name(&format!("custody_camera_pod_lens_{i}")),
            9.0,
            10.0,
            SMALL_CYL_SEGMENTS,
        )
        .translate(x, 0.0, CUSTODY_BRIDGE_UNDERSIDE_Z - 30.0);
        pods = pods + body + lens;
    }
    pods
}

fn custody_light_bars() -> Part {
    let mut bars = Part::empty(name("custody_bridge_light_bars"));
    for i in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                name(&format!("custody_bridge_light_bar_{i}")),
                180.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(i, LIGHT_BARS, 260.0),
                -CUSTODY_BRIDGE_Y / 2.0 + 8.0,
                CUSTODY_BRIDGE_UNDERSIDE_Z - 7.0,
            );
    }
    bars
}

fn temperature_exposure_logger_rail() -> Part {
    let mut rail = centered_cube(
        name("temperature_exposure_logger_rail"),
        TEMP_RAIL_X,
        TEMP_RAIL_Y,
        TEMP_RAIL_Z,
    );
    for i in 0..TEMPERATURE_LOGGER_POCKETS {
        let x = centered_index(i, TEMPERATURE_LOGGER_POCKETS, 96.0);
        let pocket = centered_cube(
            name(&format!("temperature_logger_pocket_cut_{i}")),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_DEPTH,
        )
        .translate(x, 28.0, TEMP_RAIL_Z / 2.0 + 1.0);
        let retainer = centered_cube(
            name(&format!("temperature_logger_retainer_lip_{i}")),
            LOGGER_POCKET_X + 18.0,
            6.0,
            10.0,
        )
        .translate(x, 3.0, TEMP_RAIL_Z / 2.0 + 5.0);
        rail = rail - pocket + retainer;
    }
    for i in 0..EXPOSURE_TOKEN_SLOTS {
        rail = rail
            + centered_cube(
                name(&format!("temperature_exposure_token_slot_{i}")),
                36.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(i, EXPOSURE_TOKEN_SLOTS, 46.0),
                -34.0,
                TEMP_RAIL_Z / 2.0 + 3.0,
            );
    }
    for i in 0..EXCURSION_FLAG_WELLS {
        rail = rail
            + centered_cylinder(
                name(&format!("temperature_excursion_flag_well_{i}")),
                8.0,
                7.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(
                centered_index(i, EXCURSION_FLAG_WELLS, 58.0),
                -TEMP_RAIL_Y / 2.0 + 20.0,
                TEMP_RAIL_Z / 2.0 + 3.5,
            );
    }
    rail
}

fn hold_release_reject_gate_lanes() -> Part {
    let mut panel = centered_cube(
        name("hold_release_reject_gate_lanes_panel"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );
    for lane in 0..DISPOSITION_LANES {
        let x = centered_index(lane, DISPOSITION_LANES, GATE_LANE_PITCH_X);
        panel = panel + disposition_lane(lane, x);
    }
    panel
}

fn disposition_lane(lane: usize, x: f64) -> Part {
    let mut part = Part::empty(name(&format!("disposition_lane_{lane}")));
    for slot in 0..DISPOSITION_SLOTS_PER_LANE {
        let y = centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 40.0);
        let pocket = centered_cube(
            name(&format!("disposition_lane_{lane}_material_slot_{slot}")),
            GATE_SLOT_X,
            GATE_SLOT_Y,
            GATE_SLOT_DEPTH,
        )
        .translate(x, y, GATE_PANEL_Z / 2.0 + GATE_SLOT_DEPTH / 2.0);
        part = part + pocket;
    }
    let slider = centered_cube(
        name(&format!("disposition_lane_{lane}_mechanical_gate_slider")),
        26.0,
        GATE_PANEL_Y - 40.0,
        18.0 + lane as f64 * 8.0,
    )
    .translate(
        x + GATE_SLOT_X / 2.0 + 18.0,
        0.0,
        GATE_PANEL_Z / 2.0 + 9.0 + lane as f64 * 4.0,
    );
    let custody_flag = raised_label_land(
        &format!("disposition_lane_{lane}_custody_flag"),
        72.0,
        18.0,
        lane + 3,
    )
    .translate(x, GATE_PANEL_Y / 2.0 - 20.0, GATE_PANEL_Z / 2.0 + 4.0);

    part + slider + custody_flag
}

fn sample_retain_split_rack() -> Part {
    let mut rack = centered_cube(
        name("sample_retain_split_rack"),
        RETAIN_RACK_X,
        RETAIN_RACK_Y,
        RETAIN_RACK_Z,
    );
    let header = centered_cylinder(
        name("retain_split_common_header"),
        8.0,
        RETAIN_RACK_X - 82.0,
        CYL_SEGMENTS,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, RETAIN_RACK_Z / 2.0 + 12.0);
    rack = rack + header;

    for i in 0..RETAIN_BRANCHES {
        let x = centered_index(i, RETAIN_BRANCHES, 84.0);
        rack = rack
            + centered_cube(
                name(&format!("retain_split_valve_pocket_{i}")),
                46.0,
                28.0,
                10.0,
            )
            .translate(x, 18.0, RETAIN_RACK_Z / 2.0 + 5.0);
        rack = rack
            + centered_cylinder(
                name(&format!("retain_split_branch_line_{i}")),
                4.0,
                76.0,
                SMALL_CYL_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -22.0, RETAIN_RACK_Z / 2.0 + 12.0);
    }

    for i in 0..RETAIN_VIAL_WELLS {
        let (x, y) = grid_xy(i, 6, 2, 58.0, 42.0);
        let well = centered_cylinder(
            name(&format!("retain_vial_well_{i}")),
            RETAIN_VIAL_D / 2.0,
            12.0,
            SMALL_CYL_SEGMENTS,
        )
        .translate(x, y - 36.0, RETAIN_RACK_Z / 2.0 + 6.0);
        rack = rack + well;
    }

    for i in 0..RETAIN_CHAIN_SEAL_SLOTS {
        rack = rack
            + centered_cube(
                name(&format!("retain_chain_of_custody_seal_slot_{i}")),
                42.0,
                11.0,
                6.0,
            )
            .translate(
                centered_index(i, RETAIN_CHAIN_SEAL_SLOTS, 54.0),
                RETAIN_RACK_Y / 2.0 - 18.0,
                RETAIN_RACK_Z / 2.0 + 3.0,
            );
    }
    rack
}

fn sterile_connector_cap_custody_parks() -> Part {
    let mut parks = centered_cube(
        name("sterile_connector_cap_custody_parks_panel"),
        CONNECTOR_PARK_X,
        CONNECTOR_PARK_Y,
        CONNECTOR_PARK_Z,
    );

    for i in 0..STERILE_CONNECTOR_PARKS {
        let (x, y) = grid_xy(i, 4, 2, 74.0, 48.0);
        parks = parks
            + centered_cylinder(
                name(&format!("sterile_connector_park_{i}")),
                CONNECTOR_PARK_D / 2.0,
                10.0,
                CYL_SEGMENTS,
            )
            .translate(x, y + 18.0, CONNECTOR_PARK_Z / 2.0 + 5.0);
    }

    for i in 0..CAP_CUSTODY_WELLS {
        let (x, y) = grid_xy(i, 6, 2, 48.0, 42.0);
        parks = parks
            + centered_cylinder(
                name(&format!("cap_custody_well_{i}")),
                CAP_WELL_D / 2.0,
                9.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(x, y - 42.0, CONNECTOR_PARK_Z / 2.0 + 4.5);
    }

    for i in 0..USED_CAP_QUARANTINE_SLOTS {
        parks = parks
            + centered_cube(
                name(&format!("used_cap_quarantine_slot_{i}")),
                70.0,
                18.0,
                7.0,
            )
            .translate(
                centered_index(i, USED_CAP_QUARANTINE_SLOTS, 86.0),
                CONNECTOR_PARK_Y / 2.0 - 18.0,
                CONNECTOR_PARK_Z / 2.0 + 3.5,
            );
    }
    parks
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        name("front_robot_pick_place_keepout_gauge"),
        STATION_X - 180.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0, 0.0);
    let rear_service = centered_cube(
        name("rear_barcode_bridge_service_keepout_gauge"),
        STATION_X - 210.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0, 0.0);
    let left_thaw = centered_cube(
        name("left_thaw_bath_loader_service_keepout_gauge"),
        LEFT_THAW_SERVICE_KEEP_OUT_X,
        STATION_Y - 190.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_THAW_SERVICE_KEEP_OUT_X / 2.0,
        -40.0,
        0.0,
    );
    let right_pool = centered_cube(
        name("right_pool_bag_transfer_service_keepout_gauge"),
        RIGHT_POOL_SERVICE_KEEP_OUT_X,
        STATION_Y - 190.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_POOL_SERVICE_KEEP_OUT_X / 2.0,
        -40.0,
        0.0,
    );
    let overhead = centered_cube(
        name("overhead_bag_lift_and_camera_keepout_gauge"),
        CUSTODY_BRIDGE_SPAN_X,
        130.0,
        18.0,
    )
    .translate(
        CUSTODY_BRIDGE_CENTER.0,
        CUSTODY_BRIDGE_CENTER.1,
        OVERHEAD_BAG_LIFT_CLEARANCE_Z,
    );
    let mut tags = Part::empty(name("robot_service_keepout_group_tags"));
    for i in 0..KEEP_OUT_GROUPS {
        tags = tags
            + raised_label_land(&format!("robot_service_keepout_tag_{i}"), 70.0, 18.0, i + 2)
                .translate(
                    centered_index(i, KEEP_OUT_GROUPS, 160.0),
                    -STATION_Y / 2.0 - 28.0,
                    KEEP_OUT_GAUGE_Z + 5.0,
                );
    }

    front_robot + rear_service + left_thaw + right_pool + overhead + tags
}

fn barcode_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        let bar_w = if i % 2 == 0 { 2.2 } else { 4.2 };
        land = land
            + centered_cube(name(&format!("{id}_bar_{i}")), bar_w, depth - 6.0, 3.0)
                .translate(x, 0.0, 3.5);
    }
    land
}

fn raised_label_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        land = land
            + centered_cube(name(&format!("{id}_raised_bar_{i}")), 3.0, depth - 6.0, 3.0)
                .translate(x, 0.0, 3.5);
    }
    land
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_assembly_last() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_validation_controls_are_explicit() {
        for feature in [
            "closed_media_bag_identity_nests",
            "lot_barcode_capture",
            "coa_custody_slots",
            "thaw_order_interlock_ladder",
            "temperature_exposure_logger_rail",
            "pooling_sequence_manifold",
            "pooling_order_tokens",
            "hold_release_reject_gates",
            "sample_retain_split_rack",
            "robot_keepout_gauges",
            "service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert!(VALIDATION_LIMITATIONS.contains(&"no_gmp_batch_record_implementation"));
        assert!(VALIDATION_LIMITATIONS.contains(&"no_sterile_connection_protocol"));
    }

    #[test]
    fn deck_modules_fit_without_unplanned_overlap() {
        assert_design_constraints();
        let rects = deck_module_rects();
        for module in rects {
            assert!(module.fits_inside_deck(), "{} outside deck", module.name);
        }
        for (i, a) in rects.iter().enumerate() {
            for b in rects.iter().skip(i + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
        assert!(rects[0].horizontal_gap(rects[1]) > 500.0);
    }

    #[test]
    fn lot_identity_and_custody_capacity_match_three_bag_pooling() {
        assert_eq!(MEDIA_BAG_LOTS, 3);
        assert_eq!(LOT_BARCODE_LANDS_PER_BAG * MEDIA_BAG_LOTS, 6);
        assert_eq!(LOT_COA_TAG_SLOTS_PER_BAG * MEDIA_BAG_LOTS, 3);
        assert_eq!(LOT_DATUM_PINS_PER_BAG * MEDIA_BAG_LOTS, 12);
        assert_eq!(BARCODE_SCAN_WINDOWS, 8);
        assert_eq!(COA_CARD_SLOTS, 4);
        assert_eq!(CUSTODY_SEAL_WELLS, 6);
        assert_eq!(EVENT_TIME_LANDS + READ_POINT_LANDS, 6);
        assert!(CUSTODY_BRIDGE_UNDERSIDE_Z > LOT_NEST_Z + BASE_Z + 90.0);
    }

    #[test]
    fn thaw_and_pooling_orders_are_mechanically_keyed() {
        assert_eq!(THAW_STEPS, MEDIA_BAG_LOTS);
        assert_eq!(THAW_STAGGER_MINUTES, [0, 12, 24]);
        assert!(THAW_STAGGER_MINUTES.windows(2).all(|w| w[0] < w[1]));
        assert!(THAW_KEY_HEIGHTS.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(THAW_TOKEN_SLOTS_PER_STEP * THAW_STEPS, 9);
        assert_eq!(POOL_INLET_CONNECTORS, MEDIA_BAG_LOTS);
        assert_eq!(POOL_SEQUENCE_TOKENS, MEDIA_BAG_LOTS);
        assert_eq!(POOL_NONRETURN_VALVES, MEDIA_BAG_LOTS);
        assert_eq!(POOL_WETNESS_WINDOWS, MEDIA_BAG_LOTS);
    }

    #[test]
    fn exposure_gates_retains_and_keepouts_are_sized() {
        assert_eq!(TEMPERATURE_LOGGER_POCKETS, 4);
        assert_eq!(EXPOSURE_TOKEN_SLOTS, 8);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE * DISPOSITION_LANES, 12);
        assert_eq!(RETAIN_BRANCHES, 4);
        assert_eq!(RETAIN_VIAL_WELLS, 12);
        assert_eq!(STERILE_CONNECTOR_PARKS, 8);
        assert_eq!(CAP_CUSTODY_WELLS, 12);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 420.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 300.0);
        assert!(OVERHEAD_BAG_LIFT_CLEARANCE_Z > CUSTODY_BRIDGE_UNDERSIDE_Z);
    }

    #[test]
    fn deterministic_generation_controls_are_locked() {
        assert_eq!(CYL_SEGMENTS, 32);
        assert_eq!(SMALL_CYL_SEGMENTS, 24);
        for control in [
            "fixed_output_manifest_order",
            "parametric_constants_only",
            "fixed_cylinder_segment_counts",
            "integer_feature_counts",
            "no_random_or_time_inputs",
        ] {
            assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
        }
    }
}
