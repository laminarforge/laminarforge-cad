use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic media-bag spike and prime sequence station.
//
// Design intent:
// - Hold a closed media bag in repeatable robot datums for automated spiking,
//   priming, cap custody, first-flow witness, and closed handoff before
//   perfusion.
// - Represent spike-port alignment/keying, sterile connector and cap parks,
//   prime-loop routing, waste diversion, bubble/wetness witness windows,
//   ordered route tokens, barcode/COA/status evidence surfaces, pressure/flow
//   sensor pockets, robot approach gauges, and a closed handoff adapter to the
//   media conditioning/perfusion rack.
// - This is validation fixture CAD only. It deliberately does not encode
//   clinical acceptance thresholds, media-release criteria, or a sterile
//   connection protocol.

const BIN_NAME: &str = "closed_robotic_media_bag_spike_and_prime_sequence_station";
const OUTPUT_PREFIX: &str = "output/closed_robotic_media_bag_spike_and_prime_sequence_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_base_leak_tray.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_media_bag_nest.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_spike_port_alignment_keying.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_sterile_connector_cap_parks.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_prime_loop_waste_diversion_manifold.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_bubble_wetness_witness_windows.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_route_order_token_rail.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_barcode_coa_status_surfaces.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_pressure_flow_sensor_pockets.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_robot_approach_gauges.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_closed_handoff_adapter.stl",
    "output/closed_robotic_media_bag_spike_and_prime_sequence_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "media_bag_nest",
    "spike_port_alignment_keying",
    "sterile_connector_cap_parks",
    "prime_loop",
    "waste_diversion",
    "bubble_wetness_witness_windows",
    "route_order_token_rail",
    "barcode_coa_status_surfaces",
    "pressure_flow_sensor_pockets",
    "robot_approach_gauges",
    "closed_handoff_to_conditioning_rack",
    "first_flow_release_witness",
];

const VALIDATION_LIMITATIONS: [&str; 5] = [
    "validation_fixture_intent_only",
    "no_clinical_acceptance_thresholds",
    "no_media_release_criteria",
    "no_sterile_connection_protocol",
    "no_cell_culture_performance_claim",
];

const REPRODUCIBILITY_CONTROLS: [&str; 5] = [
    "fixed_output_manifest_order",
    "integer_grid_feature_counts",
    "fixed_cylinder_segment_counts",
    "no_random_or_time_inputs",
    "parametric_constants_only",
];

const CYL_SEGMENTS: u32 = 32;
const SMALL_CYL_SEGMENTS: u32 = 24;

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const EDGE_MARGIN: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 14.0;

const BAG_CENTER: (f64, f64) = (-380.0, 245.0);
const BAG_X: f64 = 420.0;
const BAG_Y: f64 = 230.0;
const BAG_Z: f64 = 54.0;
const BAG_POCKET_X: f64 = 330.0;
const BAG_POCKET_Y: f64 = 146.0;
const BAG_POCKET_DEPTH: f64 = 22.0;
const BAG_SADDLE_RIBS: usize = 8;
const BAG_LOCATOR_PINS: usize = 6;
const BAG_STRAP_BRIDGES: usize = 4;
const BAG_OUTLET_FORKS: usize = 3;

const SPIKE_CENTER: (f64, f64) = (25.0, 245.0);
const SPIKE_X: f64 = 330.0;
const SPIKE_Y: f64 = 220.0;
const SPIKE_Z: f64 = 50.0;
const SPIKE_ROWS: usize = 2;
const SPIKE_COLS: usize = 3;
const SPIKE_PORTS: usize = SPIKE_ROWS * SPIKE_COLS;
const SPIKE_PITCH_X: f64 = 78.0;
const SPIKE_PITCH_Y: f64 = 78.0;
const SPIKE_SOCKET_D: f64 = 40.0;
const SPIKE_KEY_TABS_PER_PORT: usize = 2;
const SPIKE_AXIS_GAUGES: usize = SPIKE_PORTS;

const CAP_CENTER: (f64, f64) = (405.0, 245.0);
const CAP_X: f64 = 300.0;
const CAP_Y: f64 = 220.0;
const CAP_Z: f64 = 44.0;
const CAP_ROWS: usize = 2;
const CAP_COLS: usize = 4;
const CAP_PARKS: usize = CAP_ROWS * CAP_COLS;
const CONNECTOR_PARKS: usize = 6;
const CAP_PITCH_X: f64 = 52.0;
const CAP_PITCH_Y: f64 = 72.0;
const CAP_CUP_D: f64 = 28.0;
const CAP_QUARANTINE_SLOTS: usize = 3;

const PRIME_CENTER: (f64, f64) = (-360.0, -5.0);
const PRIME_X: f64 = 410.0;
const PRIME_Y: f64 = 220.0;
const PRIME_Z: f64 = 48.0;
const PRIME_LOOP_SEGMENTS: usize = 8;
const PRIME_VALVES: usize = 4;
const WASTE_DIVERSION_LANES: usize = 3;
const WASTE_BOTTLE_NESTS: usize = 3;
const LOOP_CHANNEL_W: f64 = 14.0;

const WITNESS_CENTER: (f64, f64) = (65.0, -5.0);
const WITNESS_X: f64 = 330.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 42.0;
const BUBBLE_WINDOWS: usize = 4;
const WETNESS_WINDOWS: usize = 4;
const FIRST_FLOW_WINDOWS: usize = 3;
const WITNESS_WINDOW_COUNT: usize = BUBBLE_WINDOWS + WETNESS_WINDOWS + FIRST_FLOW_WINDOWS;
const WINDOW_X: f64 = 58.0;
const WINDOW_Y: f64 = 34.0;

const SENSOR_CENTER: (f64, f64) = (425.0, -5.0);
const SENSOR_X: f64 = 270.0;
const SENSOR_Y: f64 = 220.0;
const SENSOR_Z: f64 = 48.0;
const PRESSURE_SENSOR_POCKETS: usize = 4;
const FLOW_SENSOR_POCKETS: usize = 3;
const SENSOR_POCKET_COUNT: usize = PRESSURE_SENSOR_POCKETS + FLOW_SENSOR_POCKETS;
const SENSOR_PORT_D: f64 = 24.0;
const SENSOR_PITCH_X: f64 = 58.0;
const SENSOR_PITCH_Y: f64 = 72.0;

const TOKEN_CENTER: (f64, f64) = (-430.0, -255.0);
const TOKEN_X: f64 = 340.0;
const TOKEN_Y: f64 = 150.0;
const TOKEN_Z: f64 = 36.0;
const ROUTE_TOKENS: usize = 7;
const TOKEN_DETENTS: usize = ROUTE_TOKENS;
const TOKEN_PITCH_X: f64 = 42.0;

const STATUS_CENTER: (f64, f64) = (-60.0, -255.0);
const STATUS_X: f64 = 320.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 28.0;
const BARCODE_LANDS: usize = 5;
const COA_CARD_SLOTS: usize = 3;
const STATUS_PUCKS: usize = 4;

const HANDOFF_CENTER: (f64, f64) = (365.0, -255.0);
const HANDOFF_X: f64 = 390.0;
const HANDOFF_Y: f64 = 150.0;
const HANDOFF_Z: f64 = 52.0;
const HANDOFF_CONNECTORS: usize = 4;
const HANDOFF_LATCHES: usize = 4;
const HANDOFF_RACK_KEYS: usize = 3;
const HANDOFF_GUIDE_RAILS: usize = 2;

const KEEP_OUT_X: f64 = 1240.0;
const KEEP_OUT_Y: f64 = 800.0;
const KEEP_OUT_RAIL_Z: f64 = 8.0;
const ROBOT_APPROACH_CLEARANCE: f64 = 430.0;
const BAG_LOAD_CLEARANCE: f64 = 250.0;
const CAP_PICK_CLEARANCE: f64 = 210.0;
const SENSOR_SERVICE_CLEARANCE: f64 = 230.0;
const HANDOFF_RACK_CLEARANCE: f64 = 280.0;
const KEEP_OUT_POSTS: usize = 6;
const ROBOT_DATUMS: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = STATION_X / 2.0 - RIM_W - EDGE_MARGIN;
        let half_y = STATION_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
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

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let bag = media_bag_nest();
    export(OUTPUTS[1], &bag);

    let spike = spike_port_alignment_keying();
    export(OUTPUTS[2], &spike);

    let caps = sterile_connector_cap_parks();
    export(OUTPUTS[3], &caps);

    let prime = prime_loop_waste_diversion_manifold();
    export(OUTPUTS[4], &prime);

    let witness = bubble_wetness_witness_windows();
    export(OUTPUTS[5], &witness);

    let tokens = route_order_token_rail();
    export(OUTPUTS[6], &tokens);

    let status = barcode_coa_status_surfaces();
    export(OUTPUTS[7], &status);

    let sensors = pressure_flow_sensor_pockets();
    export(OUTPUTS[8], &sensors);

    let gauges = robot_approach_gauges();
    export(OUTPUTS[9], &gauges);

    let handoff = closed_handoff_adapter();
    export(OUTPUTS[10], &handoff);

    let assembly = base
        + bag.translate(BAG_CENTER.0, BAG_CENTER.1, deck_insert_z(BAG_Z))
        + spike.translate(SPIKE_CENTER.0, SPIKE_CENTER.1, deck_insert_z(SPIKE_Z))
        + caps.translate(CAP_CENTER.0, CAP_CENTER.1, deck_insert_z(CAP_Z))
        + prime.translate(PRIME_CENTER.0, PRIME_CENTER.1, deck_insert_z(PRIME_Z))
        + witness.translate(WITNESS_CENTER.0, WITNESS_CENTER.1, deck_insert_z(WITNESS_Z))
        + sensors.translate(SENSOR_CENTER.0, SENSOR_CENTER.1, deck_insert_z(SENSOR_Z))
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_insert_z(TOKEN_Z))
        + status.translate(STATUS_CENTER.0, STATUS_CENTER.1, deck_insert_z(STATUS_Z))
        + handoff.translate(HANDOFF_CENTER.0, HANDOFF_CENTER.1, deck_insert_z(HANDOFF_Z))
        + gauges.translate(0.0, 0.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0 + 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed robotic media-bag spike and prime sequence station:");
    println!("  Output prefix:         {OUTPUT_PREFIX}");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!(
        "  Media bag nest:        {BAG_SADDLE_RIBS} saddle ribs, {BAG_LOCATOR_PINS} locator pins, {BAG_STRAP_BRIDGES} strap bridges, {BAG_OUTLET_FORKS} outlet forks"
    );
    println!(
        "  Spike/cap custody:     {SPIKE_PORTS} keyed spike sockets, {SPIKE_AXIS_GAUGES} axis gauges, {CAP_PARKS} cap parks, {CONNECTOR_PARKS} connector parks"
    );
    println!(
        "  Prime/waste sequence:  {PRIME_LOOP_SEGMENTS} prime-loop segments, {PRIME_VALVES} valve pockets, {WASTE_DIVERSION_LANES} waste diversion lanes, {WASTE_BOTTLE_NESTS} waste nests"
    );
    println!(
        "  Witnessing/evidence:   {WITNESS_WINDOW_COUNT} bubble/wetness/first-flow windows, {ROUTE_TOKENS} route tokens, {BARCODE_LANDS} barcode lands, {COA_CARD_SLOTS} COA slots, {STATUS_PUCKS} status pucks"
    );
    println!(
        "  Sensors/handoff:       {PRESSURE_SENSOR_POCKETS} pressure pockets, {FLOW_SENSOR_POCKETS} flow pockets, {HANDOFF_CONNECTORS} closed handoff connector nests, {HANDOFF_LATCHES} rack latch sockets"
    );
    println!(
        "  Robot gauges:          front approach {ROBOT_APPROACH_CLEARANCE:.0}mm, bag load {BAG_LOAD_CLEARANCE:.0}mm, cap pick {CAP_PICK_CLEARANCE:.0}mm, sensor service {SENSOR_SERVICE_CLEARANCE:.0}mm, handoff rack {HANDOFF_RACK_CLEARANCE:.0}mm"
    );
    println!(
        "  Limitations:           validation fixture intent only; no clinical acceptance thresholds, media-release criteria, sterile connection protocol, or cell-culture performance claim"
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
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
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

fn module_rects() -> [Rect; 9] {
    [
        rect("media_bag_nest", BAG_CENTER, BAG_X, BAG_Y),
        rect(
            "spike_port_alignment_keying",
            SPIKE_CENTER,
            SPIKE_X,
            SPIKE_Y,
        ),
        rect("sterile_connector_cap_parks", CAP_CENTER, CAP_X, CAP_Y),
        rect(
            "prime_loop_waste_diversion_manifold",
            PRIME_CENTER,
            PRIME_X,
            PRIME_Y,
        ),
        rect(
            "bubble_wetness_witness_windows",
            WITNESS_CENTER,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "pressure_flow_sensor_pockets",
            SENSOR_CENTER,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect("route_order_token_rail", TOKEN_CENTER, TOKEN_X, TOKEN_Y),
        rect(
            "barcode_coa_status_surfaces",
            STATUS_CENTER,
            STATUS_X,
            STATUS_Y,
        ),
        rect(
            "closed_handoff_adapter",
            HANDOFF_CENTER,
            HANDOFF_X,
            HANDOFF_Y,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));

    for (i, path) in OUTPUTS.iter().enumerate() {
        for other in OUTPUTS.iter().skip(i + 1) {
            assert_ne!(path, other, "duplicate output path");
        }
    }

    for feature in [
        "media_bag_nest",
        "spike_port_alignment_keying",
        "sterile_connector_cap_parks",
        "prime_loop",
        "waste_diversion",
        "bubble_wetness_witness_windows",
        "route_order_token_rail",
        "barcode_coa_status_surfaces",
        "pressure_flow_sensor_pockets",
        "robot_approach_gauges",
        "closed_handoff_to_conditioning_rack",
        "first_flow_release_witness",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    for limitation in [
        "validation_fixture_intent_only",
        "no_clinical_acceptance_thresholds",
        "no_media_release_criteria",
        "no_sterile_connection_protocol",
        "no_cell_culture_performance_claim",
    ] {
        assert!(VALIDATION_LIMITATIONS.contains(&limitation));
    }

    for control in [
        "fixed_output_manifest_order",
        "integer_grid_feature_counts",
        "fixed_cylinder_segment_counts",
        "no_random_or_time_inputs",
        "parametric_constants_only",
    ] {
        assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
    }

    assert_eq!(SPIKE_PORTS, SPIKE_ROWS * SPIKE_COLS);
    assert_eq!(SPIKE_KEY_TABS_PER_PORT, 2);
    assert_eq!(CAP_PARKS, CAP_ROWS * CAP_COLS);
    assert_eq!(
        WITNESS_WINDOW_COUNT,
        BUBBLE_WINDOWS + WETNESS_WINDOWS + FIRST_FLOW_WINDOWS
    );
    assert_eq!(
        SENSOR_POCKET_COUNT,
        PRESSURE_SENSOR_POCKETS + FLOW_SENSOR_POCKETS
    );
    assert_eq!(TOKEN_DETENTS, ROUTE_TOKENS);
    assert_eq!(CYL_SEGMENTS, 32);
    assert_eq!(SMALL_CYL_SEGMENTS, 24);
    assert!(BAG_POCKET_DEPTH < BAG_Z);
    assert!(SPIKE_SOCKET_D + 16.0 < SPIKE_PITCH_X);
    assert!(CAP_CUP_D + 14.0 < CAP_PITCH_X);
    assert!(LOOP_CHANNEL_W > 10.0);
    assert!(WINDOW_X + 12.0 < WITNESS_X / 3.0);
    assert!(SENSOR_PORT_D + 16.0 < SENSOR_PITCH_X);
    assert!(SENSOR_PITCH_Y >= 70.0);
    assert_eq!(HANDOFF_GUIDE_RAILS, 2);
    assert_eq!(KEEP_OUT_POSTS, 6);
    assert!(ROBOT_APPROACH_CLEARANCE >= 420.0);
    assert!(HANDOFF_RACK_CLEARANCE >= 260.0);

    let rects = module_rects();
    for module in rects {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds station usable deck envelope",
            module.name
        );
    }

    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(name("base_leak_tray_deck"), STATION_X, STATION_Y, BASE_Z).translate(
        0.0,
        0.0,
        BASE_Z / 2.0,
    );
    let sump = centered_cube(
        name("base_secondary_containment_sump_cut"),
        STATION_X - 2.0 * (RIM_W + 46.0),
        STATION_Y - 2.0 * (RIM_W + 54.0),
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z - 4.0);
    let front_gutter = centered_cube(
        name("base_prime_waste_front_gutter_cut"),
        STATION_X - 190.0,
        26.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 66.0, BASE_Z - 4.5);
    let drain = centered_cylinder(
        name("base_waste_low_point_drain_cut"),
        DRAIN_PORT_D / 2.0,
        64.0,
        CYL_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 82.0,
        -STATION_Y / 2.0 + 42.0,
        BASE_Z - 8.0,
    );

    deck - sump - front_gutter - drain - module_sockets() - mount_holes()
        + perimeter_rim()
        + base_workflow_lane_markers()
        + base_datum_targets()
        + base_zone_label_lands()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty(name("base_module_socket_cuts"));
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("base_socket_{}", module.name)),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(name("base_mount_hole_cuts"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                name(&format!("base_m6_clearance_hole_{i}")),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(name("base_front_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("base_rear_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(name("base_left_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(name("base_right_spill_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn base_workflow_lane_markers() -> Part {
    let top_to_prime = centered_cube(
        name("base_bag_spike_cap_to_prime_sequence_lane_marker"),
        STATION_X - 210.0,
        9.0,
        22.0,
    )
    .translate(0.0, 118.0, BASE_Z + 11.0);
    let prime_to_release = centered_cube(
        name("base_prime_sensor_to_release_sequence_lane_marker"),
        STATION_X - 240.0,
        9.0,
        22.0,
    )
    .translate(0.0, -142.0, BASE_Z + 11.0);
    let handoff_boundary = centered_cube(
        name("base_status_to_handoff_boundary_marker"),
        10.0,
        150.0,
        26.0,
    )
    .translate(135.0, -255.0, BASE_Z + 13.0);
    let clean_cap_boundary = centered_cube(
        name("base_cap_custody_clean_used_boundary_marker"),
        10.0,
        220.0,
        26.0,
    )
    .translate(240.0, 245.0, BASE_Z + 13.0);

    top_to_prime + prime_to_release + handoff_boundary + clean_cap_boundary
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty(name("base_robot_datum_targets"));
    for (i, (x, y)) in datum_points().iter().enumerate() {
        targets = targets
            + datum_disc(&format!("base_robot_datum_target_{i}")).translate(*x, *y, BASE_Z + 2.0);
    }
    targets
}

fn base_zone_label_lands() -> Part {
    raised_bar_land("base_media_bag_zone_label_land", 164.0, 24.0, 5).translate(
        BAG_CENTER.0,
        BAG_CENTER.1 + BAG_Y / 2.0 - 20.0,
        BASE_Z + 2.0,
    ) + raised_bar_land("base_spike_keying_zone_label_land", 166.0, 24.0, 6).translate(
        SPIKE_CENTER.0,
        SPIKE_CENTER.1 + SPIKE_Y / 2.0 - 20.0,
        BASE_Z + 2.0,
    ) + raised_bar_land("base_prime_loop_zone_label_land", 170.0, 24.0, 6).translate(
        PRIME_CENTER.0,
        PRIME_CENTER.1 - PRIME_Y / 2.0 + 20.0,
        BASE_Z + 2.0,
    ) + raised_bar_land("base_handoff_zone_label_land", 172.0, 24.0, 5).translate(
        HANDOFF_CENTER.0,
        HANDOFF_CENTER.1 - HANDOFF_Y / 2.0 + 20.0,
        BASE_Z + 2.0,
    )
}

fn media_bag_nest() -> Part {
    let body = centered_cube(name("media_bag_nest_body"), BAG_X, BAG_Y, BAG_Z);
    let bag_pocket = centered_cube(
        name("media_bag_nest_soft_bag_pocket_cut"),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH,
    )
    .translate(0.0, 4.0, BAG_Z / 2.0 - BAG_POCKET_DEPTH / 2.0 + 2.0);
    let label_recess = centered_cube(
        name("media_bag_nest_label_visibility_recess_cut"),
        134.0,
        42.0,
        7.0,
    )
    .translate(
        -BAG_POCKET_X / 2.0 + 94.0,
        BAG_POCKET_Y / 2.0 - 20.0,
        BAG_Z / 2.0,
    );
    let outlet_trough = centered_cube(
        name("media_bag_nest_outlet_tube_trough_cut"),
        160.0,
        22.0,
        20.0,
    )
    .translate(BAG_X / 2.0 - 80.0, -BAG_Y / 2.0 + 42.0, BAG_Z / 2.0 - 6.0);

    body - bag_pocket - label_recess - outlet_trough
        + media_bag_saddle_ribs()
        + media_bag_locator_pins()
        + media_bag_strap_bridges()
        + media_bag_outlet_forks()
        + media_bag_fill_edge_gauges()
}

fn media_bag_saddle_ribs() -> Part {
    let mut ribs = Part::empty(name("media_bag_saddle_ribs"));
    for i in 0..BAG_SADDLE_RIBS {
        let x = centered_index(i, BAG_SADDLE_RIBS, 40.0);
        ribs = ribs
            + centered_cube(
                name(&format!("media_bag_saddle_rib_{i}")),
                8.0,
                BAG_POCKET_Y - 18.0,
                10.0,
            )
            .translate(x, 6.0, BAG_Z / 2.0 + 5.0);
    }
    ribs
}

fn media_bag_locator_pins() -> Part {
    let mut pins = Part::empty(name("media_bag_locator_pins"));
    for (i, (x, y)) in [
        (-BAG_POCKET_X / 2.0 + 28.0, -BAG_POCKET_Y / 2.0 + 26.0),
        (BAG_POCKET_X / 2.0 - 28.0, -BAG_POCKET_Y / 2.0 + 26.0),
        (-BAG_POCKET_X / 2.0 + 28.0, BAG_POCKET_Y / 2.0 - 26.0),
        (BAG_POCKET_X / 2.0 - 28.0, BAG_POCKET_Y / 2.0 - 26.0),
        (-54.0, BAG_POCKET_Y / 2.0 - 24.0),
        (54.0, BAG_POCKET_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                name(&format!("media_bag_locator_pin_{i}")),
                7.0,
                18.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(*x, *y + 4.0, BAG_Z / 2.0 + 9.0);
    }
    pins
}

fn media_bag_strap_bridges() -> Part {
    let mut bridges = Part::empty(name("media_bag_retention_strap_bridges"));
    for i in 0..BAG_STRAP_BRIDGES {
        let x = centered_index(i, BAG_STRAP_BRIDGES, 86.0);
        let bridge = centered_cube(
            name(&format!("media_bag_retention_bridge_{i}")),
            46.0,
            12.0,
            22.0,
        )
        .translate(x, -BAG_POCKET_Y / 2.0 + 14.0, BAG_Z / 2.0 + 11.0);
        let strap_slot = centered_cube(
            name(&format!("media_bag_retention_bridge_slot_cut_{i}")),
            30.0,
            14.0,
            9.0,
        )
        .translate(x, -BAG_POCKET_Y / 2.0 + 14.0, BAG_Z / 2.0 + 14.0);
        bridges = bridges + (bridge - strap_slot);
    }
    bridges
}

fn media_bag_outlet_forks() -> Part {
    let mut forks = Part::empty(name("media_bag_outlet_alignment_forks"));
    for i in 0..BAG_OUTLET_FORKS {
        let y = centered_index(i, BAG_OUTLET_FORKS, 32.0) - BAG_Y / 2.0 + 70.0;
        forks = forks
            + side_fence_pair(&format!("media_bag_outlet_fork_{i}"), 64.0, 20.0, 7.0, 24.0)
                .translate(BAG_X / 2.0 - 68.0, y, BAG_Z / 2.0 + 12.0);
    }
    forks
}

fn media_bag_fill_edge_gauges() -> Part {
    let mut gauges = Part::empty(name("media_bag_fill_edge_gauges"));
    for i in 0..5 {
        gauges = gauges
            + centered_cube(
                name(&format!("media_bag_fill_edge_reference_tick_{i}")),
                46.0,
                4.0,
                5.0,
            )
            .translate(
                -BAG_X / 2.0 + 64.0 + i as f64 * 30.0,
                BAG_Y / 2.0 - 42.0,
                BAG_Z / 2.0 + 3.0,
            );
    }
    gauges
}

fn spike_port_alignment_keying() -> Part {
    let plate = centered_cube(
        name("spike_port_alignment_keying_plate"),
        SPIKE_X,
        SPIKE_Y,
        SPIKE_Z,
    );

    let mut socket_cuts = Part::empty(name("spike_port_keyed_socket_cuts"));
    let mut keys = Part::empty(name("spike_port_key_blocks"));
    let mut axes = Part::empty(name("spike_port_axis_gauge_marks"));
    let mut port_labels = Part::empty(name("spike_port_order_marker_bars"));

    for port in 0..SPIKE_PORTS {
        let (x, y) = grid_xy(port, SPIKE_COLS, SPIKE_ROWS, SPIKE_PITCH_X, SPIKE_PITCH_Y);
        socket_cuts =
            socket_cuts + keyed_spike_socket_cut(port).translate(x, y, SPIKE_Z / 2.0 - 12.0);
        keys = keys + spike_key_tabs(port).translate(x, y, SPIKE_Z / 2.0 + 8.0);
        axes = axes + spike_axis_gauge(port).translate(x, y, SPIKE_Z / 2.0 + 3.0);
        port_labels = port_labels
            + raised_bar_land(
                &format!("spike_port_order_land_{port}"),
                36.0,
                14.0,
                port + 2,
            )
            .translate(x, y - 33.0, SPIKE_Z / 2.0 + 3.0);
    }

    let approach_ramp = centered_cube(
        name("spike_port_robot_approach_flat_gauge"),
        SPIKE_X - 48.0,
        24.0,
        8.0,
    )
    .translate(0.0, -SPIKE_Y / 2.0 + 28.0, SPIKE_Z / 2.0 + 4.0);
    let anti_rotation_bar = centered_cube(
        name("spike_port_anti_rotation_reference_bar"),
        SPIKE_X - 64.0,
        8.0,
        12.0,
    )
    .translate(0.0, SPIKE_Y / 2.0 - 34.0, SPIKE_Z / 2.0 + 6.0);

    plate + keys + axes + port_labels + approach_ramp + anti_rotation_bar - socket_cuts
}

fn keyed_spike_socket_cut(port: usize) -> Part {
    let bore = centered_cylinder(
        name(&format!("spike_port_socket_round_clearance_cut_{port}")),
        SPIKE_SOCKET_D / 2.0,
        SPIKE_Z + 6.0,
        CYL_SEGMENTS,
    );
    let vertical_key = centered_cube(
        name(&format!("spike_port_socket_vertical_keyway_cut_{port}")),
        12.0,
        SPIKE_SOCKET_D + 18.0,
        SPIKE_Z + 7.0,
    );
    let side_key = centered_cube(
        name(&format!("spike_port_socket_side_keyway_cut_{port}")),
        SPIKE_SOCKET_D + 16.0,
        10.0,
        SPIKE_Z + 7.0,
    )
    .translate(0.0, 0.0, 1.0);
    bore + vertical_key + side_key
}

fn spike_key_tabs(port: usize) -> Part {
    let left = centered_cube(
        name(&format!("spike_port_left_key_tab_{port}")),
        9.0,
        28.0,
        16.0,
    )
    .translate(-SPIKE_SOCKET_D / 2.0 - 10.0, 0.0, 0.0);
    let right = centered_cube(
        name(&format!("spike_port_right_key_tab_{port}")),
        9.0,
        28.0,
        16.0,
    )
    .translate(SPIKE_SOCKET_D / 2.0 + 10.0, 0.0, 0.0);
    left + right
}

fn spike_axis_gauge(port: usize) -> Part {
    let x_bar = centered_cube(
        name(&format!("spike_port_axis_x_mark_{port}")),
        58.0,
        3.0,
        4.0,
    );
    let y_bar = centered_cube(
        name(&format!("spike_port_axis_y_mark_{port}")),
        3.0,
        58.0,
        4.0,
    );
    x_bar + y_bar
}

fn sterile_connector_cap_parks() -> Part {
    let plate = centered_cube(
        name("sterile_connector_cap_park_plate"),
        CAP_X,
        CAP_Y,
        CAP_Z,
    );
    let mut cap_cuts = Part::empty(name("sterile_connector_cap_cup_cuts"));
    let mut cap_rims = Part::empty(name("sterile_connector_cap_cup_rims"));

    for cap in 0..CAP_PARKS {
        let (x, y) = grid_xy(cap, CAP_COLS, CAP_ROWS, CAP_PITCH_X, CAP_PITCH_Y);
        cap_cuts = cap_cuts
            + centered_cylinder(
                name(&format!("sterile_connector_cap_cup_cut_{cap}")),
                CAP_CUP_D / 2.0,
                CAP_Z + 4.0,
                CYL_SEGMENTS,
            )
            .translate(x - 36.0, y, CAP_Z / 2.0 - 8.0);
        cap_rims = cap_rims
            + centered_cylinder(
                name(&format!("sterile_connector_cap_cup_rim_{cap}")),
                CAP_CUP_D / 2.0 + 5.0,
                7.0,
                CYL_SEGMENTS,
            )
            .translate(x - 36.0, y, CAP_Z / 2.0 + 3.5);
    }

    let mut connector_parks = Part::empty(name("sterile_connector_body_parks"));
    for park in 0..CONNECTOR_PARKS {
        let x = 70.0 + centered_index(park % 3, 3, 54.0);
        let y = centered_index(park / 3, 2, 70.0);
        let nest = centered_cube(
            name(&format!("sterile_connector_body_v_nest_{park}")),
            42.0,
            28.0,
            16.0,
        );
        let channel = centered_cylinder(
            name(&format!("sterile_connector_body_channel_cut_{park}")),
            10.0,
            48.0,
            SMALL_CYL_SEGMENTS,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, 3.0);
        connector_parks = connector_parks + (nest - channel).translate(x, y, CAP_Z / 2.0 + 8.0);
    }

    let divider = centered_cube(
        name("sterile_connector_clean_used_cap_custody_divider"),
        8.0,
        CAP_Y - 38.0,
        56.0,
    )
    .translate(22.0, 0.0, CAP_Z / 2.0 + 28.0);
    let quarantine = cap_quarantine_slots().translate(-CAP_X / 2.0 + 43.0, 0.0, CAP_Z / 2.0 + 3.0);
    let custody_ticks = cap_custody_tick_marks();

    plate + cap_rims + connector_parks + divider + quarantine + custody_ticks - cap_cuts
}

fn cap_quarantine_slots() -> Part {
    let mut slots = Part::empty(name("sterile_connector_lost_cap_quarantine_slots"));
    for i in 0..CAP_QUARANTINE_SLOTS {
        slots = slots
            + centered_cube(
                name(&format!("sterile_connector_lost_cap_quarantine_slot_{i}")),
                46.0,
                24.0,
                8.0,
            )
            .translate(0.0, centered_index(i, CAP_QUARANTINE_SLOTS, 42.0), 0.0);
    }
    slots
}

fn cap_custody_tick_marks() -> Part {
    let mut ticks = Part::empty(name("sterile_connector_cap_custody_tick_marks"));
    for i in 0..CAP_PARKS {
        let (x, y) = grid_xy(i, CAP_COLS, CAP_ROWS, CAP_PITCH_X, CAP_PITCH_Y);
        ticks = ticks
            + centered_cube(
                name(&format!("sterile_connector_cap_custody_tick_{i}")),
                22.0,
                3.0,
                4.0,
            )
            .translate(x - 36.0, y - 23.0, CAP_Z / 2.0 + 4.0);
    }
    ticks
}

fn prime_loop_waste_diversion_manifold() -> Part {
    let plate = centered_cube(
        name("prime_loop_waste_diversion_plate"),
        PRIME_X,
        PRIME_Y,
        PRIME_Z,
    );
    let loop_channel_cuts = prime_loop_channel_cuts();
    let loop_ribs = prime_loop_visible_ribs();
    let valves = prime_valve_pockets();
    let waste = waste_diversion_lanes();
    let waste_nests = waste_bottle_nests();
    let first_flow_gate = centered_cube(
        name("prime_loop_first_flow_release_gate_land"),
        118.0,
        32.0,
        10.0,
    )
    .translate(
        PRIME_X / 2.0 - 78.0,
        PRIME_Y / 2.0 - 40.0,
        PRIME_Z / 2.0 + 5.0,
    );

    plate + loop_ribs + valves + waste + waste_nests + first_flow_gate - loop_channel_cuts
}

fn prime_loop_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("prime_loop_channel_cuts"));
    for (i, (x, y, sx, sy)) in prime_loop_segments().iter().enumerate() {
        cuts = cuts
            + centered_cube(
                name(&format!("prime_loop_channel_rectangular_cut_{i}")),
                *sx,
                *sy,
                18.0,
            )
            .translate(*x, *y, PRIME_Z / 2.0 - 4.0);
    }
    for (i, (x, y)) in [
        (-138.0, 62.0),
        (-138.0, -56.0),
        (42.0, -56.0),
        (136.0, 42.0),
    ]
    .iter()
    .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                name(&format!("prime_loop_turn_radius_clearance_cut_{i}")),
                18.0,
                20.0,
                CYL_SEGMENTS,
            )
            .translate(*x, *y, PRIME_Z / 2.0 - 4.0);
    }
    cuts
}

fn prime_loop_visible_ribs() -> Part {
    let mut ribs = Part::empty(name("prime_loop_visible_closed_route_ribs"));
    for (i, (x, y, sx, sy)) in prime_loop_segments().iter().enumerate() {
        ribs = ribs
            + centered_cube(
                name(&format!("prime_loop_visible_route_rib_{i}")),
                if *sx > *sy { *sx } else { LOOP_CHANNEL_W },
                if *sy >= *sx { *sy } else { LOOP_CHANNEL_W },
                7.0,
            )
            .translate(*x, *y, PRIME_Z / 2.0 + 3.5);
    }
    ribs
}

fn prime_loop_segments() -> [(f64, f64, f64, f64); PRIME_LOOP_SEGMENTS] {
    [
        (-138.0, 62.0, 130.0, LOOP_CHANNEL_W),
        (-200.0, 4.0, LOOP_CHANNEL_W, 116.0),
        (-98.0, -56.0, 206.0, LOOP_CHANNEL_W),
        (0.0, 2.0, LOOP_CHANNEL_W, 116.0),
        (78.0, 62.0, 156.0, LOOP_CHANNEL_W),
        (154.0, 22.0, LOOP_CHANNEL_W, 80.0),
        (96.0, -18.0, 116.0, LOOP_CHANNEL_W),
        (42.0, -56.0, LOOP_CHANNEL_W, 76.0),
    ]
}

fn prime_valve_pockets() -> Part {
    let mut valves = Part::empty(name("prime_loop_valve_pockets"));
    for (i, (x, y)) in [(-202.0, 64.0), (-2.0, 64.0), (154.0, -18.0), (42.0, -58.0)]
        .iter()
        .enumerate()
    {
        let body = centered_cylinder(
            name(&format!("prime_loop_valve_pocket_body_{i}")),
            22.0,
            9.0,
            CYL_SEGMENTS,
        )
        .translate(*x, *y, PRIME_Z / 2.0 + 4.5);
        let handle = centered_cube(
            name(&format!("prime_loop_valve_handle_index_{i}")),
            44.0,
            6.0,
            7.0,
        )
        .translate(*x, *y, PRIME_Z / 2.0 + 12.0);
        valves = valves + body + handle;
    }
    valves
}

fn waste_diversion_lanes() -> Part {
    let mut lanes = Part::empty(name("prime_loop_waste_diversion_lanes"));
    for lane in 0..WASTE_DIVERSION_LANES {
        let y = -PRIME_Y / 2.0 + 34.0 + lane as f64 * 28.0;
        lanes = lanes
            + centered_cube(
                name(&format!("prime_loop_waste_diversion_lane_{lane}")),
                162.0,
                8.0,
                8.0,
            )
            .translate(PRIME_X / 2.0 - 108.0, y, PRIME_Z / 2.0 + 4.0);
    }
    lanes
}

fn waste_bottle_nests() -> Part {
    let mut nests = Part::empty(name("prime_loop_waste_bottle_nests"));
    for i in 0..WASTE_BOTTLE_NESTS {
        nests = nests
            + centered_cylinder(
                name(&format!("prime_loop_waste_bottle_nest_{i}")),
                19.0,
                10.0,
                CYL_SEGMENTS,
            )
            .translate(
                PRIME_X / 2.0 - 62.0,
                -PRIME_Y / 2.0 + 34.0 + i as f64 * 38.0,
                PRIME_Z / 2.0 + 5.0,
            );
    }
    nests
}

fn bubble_wetness_witness_windows() -> Part {
    let plate = centered_cube(
        name("bubble_wetness_witness_window_plate"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut window_cuts = Part::empty(name("bubble_wetness_witness_window_cuts"));
    let mut frames = Part::empty(name("bubble_wetness_witness_window_frames"));
    let mut labels = Part::empty(name("bubble_wetness_witness_window_order_bars"));

    for i in 0..BUBBLE_WINDOWS {
        let x = centered_index(i, BUBBLE_WINDOWS, 64.0);
        let y = 62.0;
        window_cuts = window_cuts
            + centered_cube(
                name(&format!("bubble_witness_optical_window_cut_{i}")),
                WINDOW_X,
                WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(x, y, 0.0);
        frames = frames
            + window_frame(
                &format!("bubble_witness_window_frame_{i}"),
                WINDOW_X + 16.0,
                WINDOW_Y + 14.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 3.0);
    }

    for i in 0..WETNESS_WINDOWS {
        let x = centered_index(i, WETNESS_WINDOWS, 64.0);
        let y = 0.0;
        window_cuts = window_cuts
            + centered_cube(
                name(&format!("wetness_witness_strip_window_cut_{i}")),
                WINDOW_X,
                WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(x, y, 0.0);
        frames = frames
            + window_frame(
                &format!("wetness_witness_window_frame_{i}"),
                WINDOW_X + 16.0,
                WINDOW_Y + 14.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 3.0);
    }

    for i in 0..FIRST_FLOW_WINDOWS {
        let x = centered_index(i, FIRST_FLOW_WINDOWS, 76.0);
        let y = -64.0;
        window_cuts = window_cuts
            + centered_cube(
                name(&format!("first_flow_release_window_cut_{i}")),
                WINDOW_X + 12.0,
                WINDOW_Y,
                WITNESS_Z + 4.0,
            )
            .translate(x, y, 0.0);
        frames = frames
            + window_frame(
                &format!("first_flow_release_window_frame_{i}"),
                WINDOW_X + 28.0,
                WINDOW_Y + 14.0,
            )
            .translate(x, y, WITNESS_Z / 2.0 + 3.0);
        labels = labels
            + raised_bar_land(
                &format!("first_flow_release_order_bar_{i}"),
                46.0,
                10.0,
                i + 3,
            )
            .translate(x, y - 29.0, WITNESS_Z / 2.0 + 4.0);
    }

    plate + frames + labels + wetness_probe_ticks() - window_cuts
}

fn wetness_probe_ticks() -> Part {
    let mut ticks = Part::empty(name("wetness_probe_tick_marks"));
    for i in 0..12 {
        ticks = ticks
            + centered_cube(
                name(&format!("wetness_probe_reference_tick_{i}")),
                3.0,
                18.0,
                4.0,
            )
            .translate(
                -WITNESS_X / 2.0 + 34.0 + i as f64 * 24.0,
                WITNESS_Y / 2.0 - 28.0,
                WITNESS_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn route_order_token_rail() -> Part {
    let rail = centered_cube(
        name("route_order_token_rail_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let front_lip = centered_cube(
        name("route_order_token_rail_front_lip"),
        TOKEN_X - 36.0,
        12.0,
        24.0,
    )
    .translate(0.0, -TOKEN_Y / 2.0 + 18.0, TOKEN_Z / 2.0 + 12.0);
    let rear_lip = centered_cube(
        name("route_order_token_rail_rear_lip"),
        TOKEN_X - 36.0,
        12.0,
        24.0,
    )
    .translate(0.0, TOKEN_Y / 2.0 - 18.0, TOKEN_Z / 2.0 + 12.0);

    let mut detent_cuts = Part::empty(name("route_order_token_detent_cuts"));
    let mut tokens = Part::empty(name("route_order_sequence_tokens"));
    for token in 0..ROUTE_TOKENS {
        let x = centered_index(token, ROUTE_TOKENS, TOKEN_PITCH_X);
        detent_cuts = detent_cuts
            + centered_cylinder(
                name(&format!("route_order_token_detent_cut_{token}")),
                14.0,
                TOKEN_Z + 6.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(x, 0.0, TOKEN_Z / 2.0);
        tokens = tokens
            + sequence_token(token).translate(x, 0.0, TOKEN_Z / 2.0 + 8.0)
            + centered_cube(
                name(&format!("route_order_token_index_tick_{token}")),
                4.0,
                42.0,
                5.0,
            )
            .translate(x, TOKEN_Y / 2.0 - 42.0, TOKEN_Z / 2.0 + 3.0);
    }

    let no_skip_gate = centered_cube(
        name("route_order_no_skip_gate_bridge"),
        TOKEN_X - 82.0,
        10.0,
        44.0,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0 + 22.0);

    rail + front_lip + rear_lip + tokens + no_skip_gate - detent_cuts
}

fn sequence_token(token: usize) -> Part {
    let base = centered_cylinder(
        name(&format!("route_order_sequence_token_{token}")),
        13.0,
        8.0,
        SMALL_CYL_SEGMENTS,
    );
    let mark = centered_cube(
        name(&format!("route_order_sequence_token_barcode_mark_{token}")),
        4.0 + token as f64,
        18.0,
        4.0,
    )
    .translate(0.0, 0.0, 6.0);
    base + mark
}

fn barcode_coa_status_surfaces() -> Part {
    let plate = centered_cube(
        name("barcode_coa_status_surface_plate"),
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut barcode_lands = Part::empty(name("barcode_coa_status_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        barcode_lands = barcode_lands
            + barcode_land(&format!("media_bag_barcode_land_{i}"), 48.0, 28.0, 7 + i).translate(
                -STATUS_X / 2.0 + 44.0 + i as f64 * 54.0,
                38.0,
                STATUS_Z / 2.0 + 3.0,
            );
    }

    let mut coa_slots = Part::empty(name("coa_card_status_slots"));
    let mut coa_slot_cuts = Part::empty(name("coa_card_status_slot_cuts"));
    for i in 0..COA_CARD_SLOTS {
        let x = centered_index(i, COA_CARD_SLOTS, 74.0);
        coa_slots = coa_slots
            + side_fence_pair(
                &format!("coa_card_slot_side_fences_{i}"),
                58.0,
                36.0,
                5.0,
                18.0,
            )
            .translate(x, -24.0, STATUS_Z / 2.0 + 9.0);
        coa_slot_cuts = coa_slot_cuts
            + centered_cube(
                name(&format!("coa_card_slot_recess_cut_{i}")),
                48.0,
                28.0,
                8.0,
            )
            .translate(x, -24.0, STATUS_Z / 2.0 - 2.0);
    }

    let mut status_pucks = Part::empty(name("status_surface_mechanical_status_pucks"));
    for i in 0..STATUS_PUCKS {
        status_pucks = status_pucks
            + centered_cylinder(
                name(&format!("mechanical_status_puck_land_{i}")),
                14.0,
                8.0,
                SMALL_CYL_SEGMENTS,
            )
            .translate(
                -STATUS_X / 2.0 + 42.0 + i as f64 * 42.0,
                -STATUS_Y / 2.0 + 28.0,
                STATUS_Z / 2.0 + 4.0,
            );
    }

    plate + barcode_lands + coa_slots + status_pucks - coa_slot_cuts
}

fn pressure_flow_sensor_pockets() -> Part {
    let plate = centered_cube(
        name("pressure_flow_sensor_pocket_plate"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let mut pocket_cuts = Part::empty(name("pressure_flow_sensor_pocket_cuts"));
    let mut rims = Part::empty(name("pressure_flow_sensor_pocket_rims"));
    let mut tap_ports = Part::empty(name("pressure_flow_sensor_tap_ports"));

    for i in 0..PRESSURE_SENSOR_POCKETS {
        let x = centered_index(i, PRESSURE_SENSOR_POCKETS, SENSOR_PITCH_X);
        let y = 42.0;
        pocket_cuts = pocket_cuts
            + centered_cylinder(
                name(&format!("pressure_sensor_pocket_cut_{i}")),
                SENSOR_PORT_D / 2.0,
                SENSOR_Z + 4.0,
                CYL_SEGMENTS,
            )
            .translate(x, y, SENSOR_Z / 2.0 - 6.0);
        rims = rims
            + centered_cylinder(
                name(&format!("pressure_sensor_pocket_rim_{i}")),
                SENSOR_PORT_D / 2.0 + 5.0,
                7.0,
                CYL_SEGMENTS,
            )
            .translate(x, y, SENSOR_Z / 2.0 + 3.5);
        tap_ports = tap_ports
            + centered_cube(
                name(&format!("pressure_sensor_tube_tap_channel_{i}")),
                42.0,
                8.0,
                6.0,
            )
            .translate(x, y - 28.0, SENSOR_Z / 2.0 + 3.0);
    }

    for i in 0..FLOW_SENSOR_POCKETS {
        let x = centered_index(i, FLOW_SENSOR_POCKETS, SENSOR_PITCH_X + 12.0);
        let y = -54.0;
        let flow_slot = centered_cube(
            name(&format!("flow_sensor_inline_slot_cut_{i}")),
            58.0,
            24.0,
            SENSOR_Z + 4.0,
        )
        .translate(x, y, SENSOR_Z / 2.0 - 5.0);
        let flow_rim = rectangular_outline(
            &format!("flow_sensor_inline_slot_rim_{i}"),
            70.0,
            36.0,
            5.0,
            7.0,
        )
        .translate(x, y, SENSOR_Z / 2.0 + 4.0);
        pocket_cuts = pocket_cuts + flow_slot;
        rims = rims + flow_rim;
    }

    let cable_strain_relief = centered_cube(
        name("pressure_flow_sensor_cable_strain_relief_comb"),
        SENSOR_X - 52.0,
        14.0,
        14.0,
    )
    .translate(0.0, -SENSOR_Y / 2.0 + 28.0, SENSOR_Z / 2.0 + 7.0);

    plate + rims + tap_ports + sensor_direction_ticks() + cable_strain_relief - pocket_cuts
}

fn sensor_direction_ticks() -> Part {
    let mut ticks = Part::empty(name("pressure_flow_sensor_direction_ticks"));
    for i in 0..8 {
        ticks = ticks
            + centered_cube(
                name(&format!("pressure_flow_sensor_direction_tick_{i}")),
                18.0,
                3.0,
                4.0,
            )
            .translate(
                -SENSOR_X / 2.0 + 36.0 + i as f64 * 28.0,
                SENSOR_Y / 2.0 - 28.0,
                SENSOR_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn robot_approach_gauges() -> Part {
    let front = centered_cube(
        name("robot_approach_front_sweep_clearance_gauge"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        name("robot_approach_rear_bag_load_clearance_gauge"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        name("robot_approach_left_cap_pick_clearance_gauge"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        name("robot_approach_right_handoff_rack_clearance_gauge"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);

    front + rear + left + right + robot_keepout_posts() + robot_tool_approach_targets()
}

fn robot_keepout_posts() -> Part {
    let mut posts = Part::empty(name("robot_approach_vertical_clearance_posts"));
    for (i, (x, y, h)) in [
        (
            -KEEP_OUT_X / 2.0 + 78.0,
            -KEEP_OUT_Y / 2.0 + 56.0,
            ROBOT_APPROACH_CLEARANCE,
        ),
        (
            KEEP_OUT_X / 2.0 - 78.0,
            -KEEP_OUT_Y / 2.0 + 56.0,
            HANDOFF_RACK_CLEARANCE,
        ),
        (
            -KEEP_OUT_X / 2.0 + 78.0,
            KEEP_OUT_Y / 2.0 - 56.0,
            BAG_LOAD_CLEARANCE,
        ),
        (
            KEEP_OUT_X / 2.0 - 78.0,
            KEEP_OUT_Y / 2.0 - 56.0,
            CAP_PICK_CLEARANCE,
        ),
        (
            SENSOR_CENTER.0,
            SENSOR_CENTER.1 + 100.0,
            SENSOR_SERVICE_CLEARANCE,
        ),
        (
            SPIKE_CENTER.0,
            SPIKE_CENTER.1 - 112.0,
            ROBOT_APPROACH_CLEARANCE,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                name(&format!("robot_approach_clearance_post_{i}")),
                7.0,
                *h,
                SMALL_CYL_SEGMENTS,
            )
            .translate(*x, *y, h / 2.0);
    }
    posts
}

fn robot_tool_approach_targets() -> Part {
    let mut targets = Part::empty(name("robot_tool_approach_targets"));
    for (i, (x, y)) in [
        BAG_CENTER,
        SPIKE_CENTER,
        CAP_CENTER,
        PRIME_CENTER,
        WITNESS_CENTER,
        SENSOR_CENTER,
        TOKEN_CENTER,
        HANDOFF_CENTER,
    ]
    .iter()
    .enumerate()
    {
        targets =
            targets + datum_disc(&format!("robot_tool_approach_target_{i}")).translate(*x, *y, 4.0);
    }
    targets
}

fn closed_handoff_adapter() -> Part {
    let plate = centered_cube(
        name("closed_handoff_conditioning_rack_adapter_plate"),
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let rack_tongue = centered_cube(
        name("closed_handoff_conditioning_rack_tongue"),
        84.0,
        HANDOFF_Y + 34.0,
        30.0,
    )
    .translate(HANDOFF_X / 2.0 - 26.0, 0.0, HANDOFF_Z / 2.0 + 15.0);
    let connector_nests = handoff_connector_nests();
    let latch_sockets = handoff_latch_sockets();
    let rack_keys = handoff_rack_keys();
    let guide_rails = handoff_guide_rails();
    let seal_windows = handoff_seal_witness_windows();
    let tube_exit_channel = centered_cube(
        name("closed_handoff_transfer_tube_exit_channel_cut"),
        HANDOFF_X - 112.0,
        24.0,
        18.0,
    )
    .translate(-30.0, -HANDOFF_Y / 2.0 + 34.0, HANDOFF_Z / 2.0 - 4.0);

    plate + rack_tongue + connector_nests + latch_sockets + rack_keys + guide_rails + seal_windows
        - tube_exit_channel
}

fn handoff_connector_nests() -> Part {
    let mut nests = Part::empty(name("closed_handoff_connector_nests"));
    for i in 0..HANDOFF_CONNECTORS {
        let x = centered_index(i, HANDOFF_CONNECTORS, 62.0) - 42.0;
        let nest = centered_cylinder(
            name(&format!("closed_handoff_connector_nest_rim_{i}")),
            18.0,
            10.0,
            CYL_SEGMENTS,
        )
        .translate(x, 18.0, HANDOFF_Z / 2.0 + 5.0);
        let key = centered_cube(
            name(&format!("closed_handoff_connector_key_block_{i}")),
            10.0,
            30.0,
            14.0,
        )
        .translate(x + 24.0, 18.0, HANDOFF_Z / 2.0 + 7.0);
        nests = nests + nest + key;
    }
    nests
}

fn handoff_latch_sockets() -> Part {
    let mut sockets = Part::empty(name("closed_handoff_rack_latch_sockets"));
    for (i, (x, y)) in [
        (-HANDOFF_X / 2.0 + 44.0, -HANDOFF_Y / 2.0 + 26.0),
        (HANDOFF_X / 2.0 - 86.0, -HANDOFF_Y / 2.0 + 26.0),
        (-HANDOFF_X / 2.0 + 44.0, HANDOFF_Y / 2.0 - 26.0),
        (HANDOFF_X / 2.0 - 86.0, HANDOFF_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        sockets = sockets
            + centered_cube(
                name(&format!("closed_handoff_latch_socket_land_{i}")),
                36.0,
                18.0,
                8.0,
            )
            .translate(*x, *y, HANDOFF_Z / 2.0 + 4.0);
    }
    sockets
}

fn handoff_rack_keys() -> Part {
    let mut keys = Part::empty(name("closed_handoff_conditioning_rack_keys"));
    for i in 0..HANDOFF_RACK_KEYS {
        keys = keys
            + centered_cube(
                name(&format!("closed_handoff_rack_key_{i}")),
                18.0,
                42.0,
                22.0,
            )
            .translate(
                HANDOFF_X / 2.0 - 70.0,
                centered_index(i, HANDOFF_RACK_KEYS, 44.0),
                HANDOFF_Z / 2.0 + 11.0,
            );
    }
    keys
}

fn handoff_guide_rails() -> Part {
    let upper = centered_cube(
        name("closed_handoff_upper_guide_rail"),
        HANDOFF_X - 92.0,
        10.0,
        18.0,
    )
    .translate(-28.0, HANDOFF_Y / 2.0 - 34.0, HANDOFF_Z / 2.0 + 9.0);
    let lower = centered_cube(
        name("closed_handoff_lower_guide_rail"),
        HANDOFF_X - 92.0,
        10.0,
        18.0,
    )
    .translate(-28.0, -HANDOFF_Y / 2.0 + 34.0, HANDOFF_Z / 2.0 + 9.0);
    upper + lower
}

fn handoff_seal_witness_windows() -> Part {
    let mut windows = Part::empty(name("closed_handoff_seal_witness_windows"));
    for i in 0..3 {
        windows = windows
            + window_frame(
                &format!("closed_handoff_seal_witness_window_{i}"),
                48.0,
                24.0,
            )
            .translate(
                -HANDOFF_X / 2.0 + 80.0 + i as f64 * 58.0,
                -16.0,
                HANDOFF_Z / 2.0 + 4.0,
            );
    }
    windows
}

fn raised_bar_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        let bar_w = if i % 2 == 0 { 3.0 } else { 5.0 };
        land = land
            + centered_cube(
                name(&format!("{id}_raised_bar_{i}")),
                bar_w,
                depth - 8.0,
                3.0,
            )
            .translate(x, 0.0, 3.5);
    }
    land
}

fn barcode_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        let bar_w = match i % 3 {
            0 => 2.0,
            1 => 4.0,
            _ => 6.0,
        };
        land = land
            + centered_cube(
                name(&format!("{id}_barcode_bar_{i}")),
                bar_w,
                depth - 7.0,
                3.0,
            )
            .translate(x, 0.0, 3.5);
    }
    land
}

fn side_fence_pair(id: &str, width: f64, depth: f64, fence_w: f64, fence_z: f64) -> Part {
    let left = centered_cube(name(&format!("{id}_left_fence")), fence_w, depth, fence_z).translate(
        -width / 2.0 + fence_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(name(&format!("{id}_right_fence")), fence_w, depth, fence_z)
        .translate(width / 2.0 - fence_w / 2.0, 0.0, 0.0);
    left + right
}

fn rectangular_outline(id: &str, width: f64, depth: f64, rail_w: f64, rail_z: f64) -> Part {
    let left = centered_cube(name(&format!("{id}_left_rail")), rail_w, depth, rail_z).translate(
        -width / 2.0 + rail_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(name(&format!("{id}_right_rail")), rail_w, depth, rail_z).translate(
        width / 2.0 - rail_w / 2.0,
        0.0,
        0.0,
    );
    let front = centered_cube(name(&format!("{id}_front_rail")), width, rail_w, rail_z).translate(
        0.0,
        -depth / 2.0 + rail_w / 2.0,
        0.0,
    );
    let rear = centered_cube(name(&format!("{id}_rear_rail")), width, rail_w, rail_z).translate(
        0.0,
        depth / 2.0 - rail_w / 2.0,
        0.0,
    );
    left + right + front + rear
}

fn window_frame(id: &str, width: f64, depth: f64) -> Part {
    rectangular_outline(id, width, depth, 5.0, 6.0)
}

fn datum_disc(id: &str) -> Part {
    let disc = centered_cylinder(name(id), 13.0, 4.0, SMALL_CYL_SEGMENTS);
    let x_bar =
        centered_cube(name(&format!("{id}_x_bar")), 21.0, 3.0, 2.0).translate(0.0, 0.0, 3.0);
    let y_bar =
        centered_cube(name(&format!("{id}_y_bar")), 3.0, 21.0, 2.0).translate(0.0, 0.0, 3.0);
    disc + x_bar + y_bar
}

fn datum_points() -> [(f64, f64); ROBOT_DATUMS] {
    [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-260.0, STATION_Y / 2.0 - 58.0),
        (260.0, STATION_Y / 2.0 - 58.0),
        (-260.0, -STATION_Y / 2.0 + 58.0),
        (260.0, -STATION_Y / 2.0 + 58.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn expected_output_manifest() -> [&'static str; 12] {
        [
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_base_leak_tray.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_media_bag_nest.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_spike_port_alignment_keying.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_sterile_connector_cap_parks.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_prime_loop_waste_diversion_manifold.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_bubble_wetness_witness_windows.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_route_order_token_rail.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_barcode_coa_status_surfaces.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_pressure_flow_sensor_pockets.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_robot_approach_gauges.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_closed_handoff_adapter.stl",
            "output/closed_robotic_media_bag_spike_and_prime_sequence_station_assembly.stl",
        ]
    }

    #[test]
    fn design_constraints_hold() {
        assert_design_constraints();
    }

    #[test]
    fn output_manifest_is_stable_scoped_and_unique() {
        assert_eq!(OUTPUTS, expected_output_manifest());
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_counts_cover_spike_prime_and_handoff_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert_eq!(SPIKE_PORTS, 6);
        assert_eq!(SPIKE_KEY_TABS_PER_PORT * SPIKE_PORTS, 12);
        assert_eq!(CAP_PARKS, 8);
        assert_eq!(CONNECTOR_PARKS, 6);
        assert_eq!(PRIME_LOOP_SEGMENTS, 8);
        assert_eq!(PRIME_VALVES, 4);
        assert_eq!(WASTE_DIVERSION_LANES, 3);
        assert_eq!(WITNESS_WINDOW_COUNT, 11);
        assert_eq!(ROUTE_TOKENS, 7);
        assert_eq!(SENSOR_POCKET_COUNT, 7);
        assert_eq!(HANDOFF_CONNECTORS, 4);
        assert_eq!(HANDOFF_RACK_KEYS, 3);
    }

    #[test]
    fn station_modules_fit_without_major_plan_view_overlap() {
        let rects = module_rects();
        assert_eq!(rects.len(), 9);

        for module in rects {
            assert!(
                module.fits_inside_deck(),
                "{} exceeds station usable deck envelope",
                module.name
            );
        }

        for (i, a) in rects.iter().enumerate() {
            for b in rects.iter().skip(i + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
    }

    #[test]
    fn explicit_reproducibility_controls_are_present() {
        assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 5);
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"fixed_output_manifest_order"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"integer_grid_feature_counts"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"fixed_cylinder_segment_counts"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"no_random_or_time_inputs"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"parametric_constants_only"));
        assert_eq!(CYL_SEGMENTS, 32);
        assert_eq!(SMALL_CYL_SEGMENTS, 24);
    }

    #[test]
    fn validation_limitations_avoid_thresholds_or_protocol_claims() {
        assert!(VALIDATION_LIMITATIONS.contains(&"validation_fixture_intent_only"));
        assert!(VALIDATION_LIMITATIONS.contains(&"no_clinical_acceptance_thresholds"));
        assert!(VALIDATION_LIMITATIONS.contains(&"no_media_release_criteria"));
        assert!(VALIDATION_LIMITATIONS.contains(&"no_sterile_connection_protocol"));
        assert!(VALIDATION_LIMITATIONS.contains(&"no_cell_culture_performance_claim"));
    }
}
