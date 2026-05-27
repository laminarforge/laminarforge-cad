use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic cassette seeding angle and dwell station.
//
// Validation fixture intent:
// - Make robotic cassette seeding orientation, dwell timing, and post-seed
//   settling consistency visible across many chips while the cassette remains
//   represented as a sealed module.
// - Package angle witness saddles, timed dwell lanes, gripper datum and
//   clearance gauges, isolation pads, dye/optical distribution witnesses,
//   custody surfaces, and closed connector handoff geometry in one station.
// - Keep this as deterministic mechanical CAD for fixture review and printing.
//   It does not encode clinical acceptance thresholds, incubation release
//   criteria, seeding recipes, or validated robot motion programs.

const PART_PREFIX: &str = "closed_robotic_cassette_seeding_angle_and_dwell_station";
const DESIGN_REVISION: &str = "T-864F0959-seeding-angle-dwell-r1";
const UNITS: &str = "millimeters";
const GEOMETRY_SEED: u64 = 0;
const FLOAT_GRID_MM: f64 = 0.5;
const CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED: bool = false;
const REPRODUCIBILITY_CONTROLS: [&str; 5] = [
    "fixed_feature_counts",
    "stable_output_manifest_order",
    "no_random_sampling",
    "millimeter_grid_constants",
    "no_clinical_acceptance_thresholds",
];

const OUTPUTS: [&str; 12] = [
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_closed_base_tray.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_sealed_cassette_nest.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_angle_witness_saddles.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_timed_dwell_lanes.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_robot_gripper_datum_clearance_envelope.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_vibration_settling_isolation_pads.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_optical_dye_distribution_witness_slots.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_barcode_custody_status_surfaces.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_closed_connector_handoff_bulkhead.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_evidence_bridge.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_service_keepout_gauges.stl",
    "output/closed_robotic_cassette_seeding_angle_and_dwell_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "closed_base_tray",
    "sealed_cassette_nest",
    "angle_witness_saddles",
    "timed_dwell_lanes",
    "robot_gripper_datum_clearance_envelope",
    "vibration_settling_isolation_pads",
    "optical_dye_distribution_witness_slots",
    "barcode_custody_status_surfaces",
    "closed_connector_handoff_bulkhead",
    "evidence_bridge",
    "service_keepout_gauges",
    "assembly_export",
];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 930.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 5.5;
const MODULE_GAP: f64 = 24.0;
const DRAIN_D: f64 = 17.0;
const MOUNT_BOSSES: usize = 8;

const NEST_CENTER: (f64, f64) = (-480.0, 250.0);
const NEST_X: f64 = 420.0;
const NEST_Y: f64 = 280.0;
const NEST_Z: f64 = 62.0;
const CASSETTE_BAYS: usize = 2;
const CASSETTE_RECESS_X: f64 = 150.0;
const CASSETTE_RECESS_Y: f64 = 206.0;
const CASSETTE_RECESS_DEPTH: f64 = 20.0;
const CASSETTE_BAY_PITCH_X: f64 = 202.0;
const CHIPS_PER_CASSETTE: usize = 12;
const SEAL_LATCH_LANDS: usize = 8;
const ORIENTATION_DATUM_KEYS: usize = 6;

const ANGLE_CENTER: (f64, f64) = (0.0, 250.0);
const ANGLE_X: f64 = 420.0;
const ANGLE_Y: f64 = 280.0;
const ANGLE_Z: f64 = 46.0;
const ANGLE_SADDLES: usize = 4;
const ANGLE_SADDLE_ANGLES_DEG: [f64; ANGLE_SADDLES] = [-6.0, -3.0, 3.0, 6.0];
const ANGLE_SADDLE_X: f64 = 72.0;
const ANGLE_SADDLE_Y: f64 = 204.0;
const ANGLE_SADDLE_PITCH_X: f64 = 92.0;
const ANGLE_DATUM_TICKS_PER_SADDLE: usize = 5;

const OPTICAL_CENTER: (f64, f64) = (480.0, 250.0);
const OPTICAL_X: f64 = 430.0;
const OPTICAL_Y: f64 = 280.0;
const OPTICAL_Z: f64 = 42.0;
const WITNESS_COLS: usize = 8;
const WITNESS_ROWS: usize = 6;
const WITNESS_SLOTS: usize = WITNESS_COLS * WITNESS_ROWS;
const WITNESS_SLOT_X: f64 = 31.0;
const WITNESS_SLOT_Y: f64 = 19.0;
const WITNESS_SLOT_DEPTH: f64 = 13.0;
const WITNESS_PITCH_X: f64 = 46.0;
const WITNESS_PITCH_Y: f64 = 36.0;

const DWELL_CENTER: (f64, f64) = (-350.0, -80.0);
const DWELL_X: f64 = 650.0;
const DWELL_Y: f64 = 220.0;
const DWELL_Z: f64 = 36.0;
const DWELL_LANES: usize = 4;
const DWELL_STOPS_PER_LANE: usize = 8;
const DWELL_STOP_SLOTS: usize = DWELL_LANES * DWELL_STOPS_PER_LANE;
const DWELL_SLOT_X: f64 = 54.0;
const DWELL_SLOT_Y: f64 = 24.0;
const DWELL_SLOT_DEPTH: f64 = 10.0;
const DWELL_LANE_PITCH_Y: f64 = 42.0;
const DWELL_STOP_PITCH_X: f64 = 68.0;
const DWELL_TICK_MARKS_PER_LANE: usize = DWELL_STOPS_PER_LANE + 1;

const SETTLING_CENTER: (f64, f64) = (360.0, -85.0);
const SETTLING_X: f64 = 520.0;
const SETTLING_Y: f64 = 220.0;
const SETTLING_Z: f64 = 40.0;
const ISOLATION_PADS: usize = 8;
const ISOLATION_PAD_D: f64 = 42.0;
const SETTLING_CASSETTE_DOCKS: usize = 4;
const SETTLING_RIBS_PER_DOCK: usize = 5;

const CUSTODY_CENTER: (f64, f64) = (-500.0, -330.0);
const CUSTODY_X: f64 = 360.0;
const CUSTODY_Y: f64 = 135.0;
const CUSTODY_Z: f64 = 16.0;
const BARCODE_LANDS: usize = 16;
const STATUS_LANES: usize = 4;
const STATUS_SLOTS_PER_LANE: usize = 4;
const CUSTODY_PUNCH_LANDS: usize = 8;

const CONNECTOR_CENTER: (f64, f64) = (0.0, -330.0);
const CONNECTOR_X: f64 = 460.0;
const CONNECTOR_Y: f64 = 160.0;
const CONNECTOR_Z: f64 = 92.0;
const CLOSED_CONNECTOR_PORTS: usize = 4;
const CONNECTOR_PORT_D: f64 = 30.0;
const CAP_PARKING_WELLS: usize = CLOSED_CONNECTOR_PORTS;
const GASKET_FRAME_W: f64 = 18.0;

const ROBOT_CENTER: (f64, f64) = (500.0, -330.0);
const ROBOT_X: f64 = 360.0;
const ROBOT_Y: f64 = 160.0;
const ROBOT_Z: f64 = 52.0;
const GRIPPER_DATUM_PINS: usize = 6;
const CLEARANCE_RAILS: usize = 4;
const GRIPPER_FINGER_CLEAR_X: f64 = 268.0;
const GRIPPER_FINGER_CLEAR_Y: f64 = 92.0;

const BRIDGE_SPAN_X: f64 = 1280.0;
const BRIDGE_SPAN_Y: f64 = 700.0;
const BRIDGE_POST_W: f64 = 22.0;
const BRIDGE_UNDERSIDE_Z: f64 = 286.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;
const FIDUCIAL_DISCS: usize = 16;

const FRONT_ROBOT_CLEARANCE_Y: f64 = 430.0;
const REAR_INCUBATOR_CLEARANCE_Y: f64 = 310.0;
const LEFT_CASSETTE_SERVICE_X: f64 = 250.0;
const RIGHT_MANIFOLD_SERVICE_X: f64 = 260.0;
const OVERHEAD_GRIPPER_CLEARANCE_Z: f64 = 390.0;
const KEEP_OUT_GAUGES: usize = 8;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = closed_base_tray();
    export(&base, OUTPUTS[0]);

    let nest = sealed_cassette_nest();
    export(&nest, OUTPUTS[1]);

    let angles = angle_witness_saddles();
    export(&angles, OUTPUTS[2]);

    let dwell = timed_dwell_lanes();
    export(&dwell, OUTPUTS[3]);

    let robot = robot_gripper_datum_clearance_envelope();
    export(&robot, OUTPUTS[4]);

    let settling = vibration_settling_isolation_pads();
    export(&settling, OUTPUTS[5]);

    let witnesses = optical_dye_distribution_witness_slots();
    export(&witnesses, OUTPUTS[6]);

    let custody = barcode_custody_status_surfaces();
    export(&custody, OUTPUTS[7]);

    let connectors = closed_connector_handoff_bulkhead();
    export(&connectors, OUTPUTS[8]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let keepouts = service_keepout_gauges();
    export(&keepouts, OUTPUTS[10]);

    let assembly = station_assembly();
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed robotic cassette seeding angle and dwell station:");
    println!("  Generator:                 {PART_PREFIX}");
    println!("  Design revision:           {DESIGN_REVISION}");
    println!(
        "  Station deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm closed tray with socketed modules, sump, drain, datum bosses"
    );
    println!(
        "  Cassette validation:       {CASSETTE_BAYS} sealed cassette bays covering {} chip positions with {SEAL_LATCH_LANDS} latch witness lands",
        total_chip_positions()
    );
    println!(
        "  Orientation/dwell:         {ANGLE_SADDLES} angle witness saddles {:?}, {DWELL_LANES} timed dwell lanes, {DWELL_STOP_SLOTS} cassette stop slots",
        ANGLE_SADDLE_ANGLES_DEG
    );
    println!(
        "  Settling/witness:          {ISOLATION_PADS} isolation pads, {SETTLING_CASSETTE_DOCKS} settling docks, {WITNESS_SLOTS} optical/dye witness slots"
    );
    println!(
        "  Handoff/custody:           {CLOSED_CONNECTOR_PORTS} closed connector ports, {BARCODE_LANDS} barcode lands, {} status pockets",
        STATUS_LANES * STATUS_SLOTS_PER_LANE
    );
    println!(
        "  Robot/evidence gauges:     {GRIPPER_DATUM_PINS} datum pins, {CLEARANCE_RAILS} clearance rails, {CAMERA_PODS} camera pods, {KEEP_OUT_GAUGES} keep-out gauges"
    );
    println!(
        "  Reproducibility controls:  units={UNITS}, geometry_seed={GEOMETRY_SEED}, grid={FLOAT_GRID_MM:.1}mm, thresholds_encoded={CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED}"
    );
    println!(
        "  Control list:              {}",
        REPRODUCIBILITY_CONTROLS.join(", ")
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    closed_base_tray()
        + sealed_cassette_nest().translate(NEST_CENTER.0, NEST_CENTER.1, insert_z())
        + angle_witness_saddles().translate(ANGLE_CENTER.0, ANGLE_CENTER.1, insert_z())
        + optical_dye_distribution_witness_slots().translate(
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
            insert_z(),
        )
        + timed_dwell_lanes().translate(DWELL_CENTER.0, DWELL_CENTER.1, insert_z())
        + vibration_settling_isolation_pads().translate(
            SETTLING_CENTER.0,
            SETTLING_CENTER.1,
            insert_z(),
        )
        + barcode_custody_status_surfaces().translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            insert_z(),
        )
        + closed_connector_handoff_bulkhead().translate(
            CONNECTOR_CENTER.0,
            CONNECTOR_CENTER.1,
            insert_z(),
        )
        + robot_gripper_datum_clearance_envelope().translate(
            ROBOT_CENTER.0,
            ROBOT_CENTER.1,
            insert_z(),
        )
        + evidence_bridge().translate(0.0, 0.0, BASE_Z)
        + service_keepout_gauges()
}

fn insert_z() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len());
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(CASSETTE_BAYS * CHIPS_PER_CASSETTE, total_chip_positions());
    assert_eq!(WITNESS_SLOTS, WITNESS_COLS * WITNESS_ROWS);
    assert_eq!(DWELL_STOP_SLOTS, DWELL_LANES * DWELL_STOPS_PER_LANE);
    assert_eq!(DWELL_TICK_MARKS_PER_LANE, DWELL_STOPS_PER_LANE + 1);
    assert_eq!(CAP_PARKING_WELLS, CLOSED_CONNECTOR_PORTS);
    assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, BARCODE_LANDS);
    assert_eq!(ANGLE_SADDLE_ANGLES_DEG.len(), ANGLE_SADDLES);
    assert!(!CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED);
    assert!(GEOMETRY_SEED == 0);
    assert!(FLOAT_GRID_MM > 0.0);
    assert!(highest_fixture_feature_z() < OVERHEAD_GRIPPER_CLEARANCE_Z);
    assert!(
        BRIDGE_SPAN_X + 2.0 * BRIDGE_POST_W < DECK_X - 2.0 * RIM_W
            && BRIDGE_SPAN_Y + 2.0 * BRIDGE_POST_W < DECK_Y - 2.0 * RIM_W,
        "evidence bridge must land inside the closed station rim"
    );

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            fits_on_deck(footprint),
            "{} exceeds the socketed tray bounds",
            footprint.name
        );
    }

    for (left_index, left) in footprints.iter().enumerate() {
        for right in footprints.iter().skip(left_index + 1) {
            assert!(
                footprints_clear(*left, *right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn highest_fixture_feature_z() -> f64 {
    BASE_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z
}

fn total_chip_positions() -> usize {
    CASSETTE_BAYS * CHIPS_PER_CASSETTE
}

fn module_footprints() -> [Footprint; 8] {
    [
        fp("sealed_cassette_nest", NEST_CENTER, NEST_X, NEST_Y),
        fp("angle_witness_saddles", ANGLE_CENTER, ANGLE_X, ANGLE_Y),
        fp(
            "optical_dye_distribution_witness_slots",
            OPTICAL_CENTER,
            OPTICAL_X,
            OPTICAL_Y,
        ),
        fp("timed_dwell_lanes", DWELL_CENTER, DWELL_X, DWELL_Y),
        fp(
            "vibration_settling_isolation_pads",
            SETTLING_CENTER,
            SETTLING_X,
            SETTLING_Y,
        ),
        fp(
            "barcode_custody_status_surfaces",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        fp(
            "closed_connector_handoff_bulkhead",
            CONNECTOR_CENTER,
            CONNECTOR_X,
            CONNECTOR_Y,
        ),
        fp(
            "robot_gripper_datum_clearance_envelope",
            ROBOT_CENTER,
            ROBOT_X,
            ROBOT_Y,
        ),
    ]
}

fn fp(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_on_deck(footprint: Footprint) -> bool {
    let usable_x = DECK_X / 2.0 - RIM_W - 10.0;
    let usable_y = DECK_Y / 2.0 - RIM_W - 10.0;
    footprint.center.0.abs() + footprint.x / 2.0 <= usable_x
        && footprint.center.1.abs() + footprint.y / 2.0 <= usable_y
}

fn footprints_clear(left: Footprint, right: Footprint) -> bool {
    let dx = (left.center.0 - right.center.0).abs();
    let dy = (left.center.1 - right.center.1).abs();
    dx >= left.x / 2.0 + right.x / 2.0 + MODULE_GAP
        || dy >= left.y / 2.0 + right.y / 2.0 + MODULE_GAP
}

fn closed_base_tray() -> Part {
    let deck = centered_cube(
        format!("{PART_PREFIX}_closed_base_tray_floor"),
        DECK_X,
        DECK_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let wipe_basin = centered_cube(
        format!("{PART_PREFIX}_wipeable_recessed_validation_floor"),
        DECK_X - 150.0,
        DECK_Y - 132.0,
        7.0,
    )
    .translate(0.0, -6.0, BASE_Z - 3.5);
    let drain = centered_cylinder(
        format!("{PART_PREFIX}_closed_tray_low_point_drain"),
        DRAIN_D / 2.0,
        54.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 90.0, -DECK_Y / 2.0 + 38.0, BASE_Z - 7.0);

    deck - wipe_basin - drain - module_socket_recesses() - mount_holes()
        + perimeter_rims()
        + deck_zone_lands()
        + deck_datum_bosses()
        + transfer_lips()
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{PART_PREFIX}_module_socket_recesses"));
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{PART_PREFIX}_{}_socket_recess", footprint.name),
                footprint.x + 10.0,
                footprint.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{PART_PREFIX}_mount_holes"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 62.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 2.0 - 62.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 2.0 + 62.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 62.0),
        (0.0, -DECK_Y / 2.0 + 62.0),
        (0.0, DECK_Y / 2.0 - 62.0),
        (-DECK_X / 2.0 + 62.0, 0.0),
        (DECK_X / 2.0 - 62.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PART_PREFIX}_m6_mount_clearance_{i}"),
                3.4,
                BASE_Z + 8.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("{PART_PREFIX}_mount_slot_clocking_relief_{i}"),
                28.0,
                7.0,
                BASE_Z + 8.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let z = BASE_Z + RIM_Z / 2.0;
    centered_cube(
        format!("{PART_PREFIX}_front_closed_tray_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, z)
        + centered_cube(
            format!("{PART_PREFIX}_rear_closed_tray_rim"),
            DECK_X,
            RIM_W,
            RIM_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, z)
        + centered_cube(
            format!("{PART_PREFIX}_left_closed_tray_rim"),
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, z)
        + centered_cube(
            format!("{PART_PREFIX}_right_closed_tray_rim"),
            RIM_W,
            DECK_Y,
            RIM_Z,
        )
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, z)
}

fn deck_zone_lands() -> Part {
    let cassette_land = centered_cube(
        format!("{PART_PREFIX}_cassette_orientation_zone_land"),
        1325.0,
        332.0,
        3.0,
    )
    .translate(0.0, 250.0, BASE_Z + 1.5);
    let dwell_land = centered_cube(
        format!("{PART_PREFIX}_dwell_settling_zone_land"),
        1240.0,
        268.0,
        3.0,
    )
    .translate(5.0, -82.0, BASE_Z + 1.5);
    let handoff_land = centered_cube(
        format!("{PART_PREFIX}_custody_handoff_robot_zone_land"),
        1240.0,
        198.0,
        3.0,
    )
    .translate(0.0, -330.0, BASE_Z + 1.5);
    cassette_land + dwell_land + handoff_land
}

fn deck_datum_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PART_PREFIX}_deck_datum_bosses"));
    for index in 0..MOUNT_BOSSES {
        let x = if index % 2 == 0 {
            -DECK_X / 2.0 + 104.0
        } else {
            DECK_X / 2.0 - 104.0
        };
        let y = index_offset(index / 2, MOUNT_BOSSES / 2, 230.0);
        let boss = centered_cylinder(
            format!("{PART_PREFIX}_robot_deck_datum_boss_{index}"),
            14.0,
            8.0,
            36,
        )
        .translate(x, y, BASE_Z + 4.0);
        let center = centered_cylinder(
            format!("{PART_PREFIX}_robot_deck_datum_center_mark_{index}"),
            2.4,
            9.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - center);
    }
    bosses
}

fn transfer_lips() -> Part {
    let cassette_lip = centered_cube(
        format!("{PART_PREFIX}_cassette_input_transfer_lip"),
        NEST_X - 38.0,
        12.0,
        16.0,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - NEST_Y / 2.0 - 10.0,
        BASE_Z + 8.0,
    );
    let connector_lip = centered_cube(
        format!("{PART_PREFIX}_closed_connector_handoff_transfer_lip"),
        CONNECTOR_X - 58.0,
        12.0,
        18.0,
    )
    .translate(
        CONNECTOR_CENTER.0,
        CONNECTOR_CENTER.1 + CONNECTOR_Y / 2.0 + 10.0,
        BASE_Z + 9.0,
    );
    let robot_lip = centered_cube(
        format!("{PART_PREFIX}_robot_gripper_approach_lip"),
        ROBOT_X - 36.0,
        12.0,
        16.0,
    )
    .translate(
        ROBOT_CENTER.0,
        ROBOT_CENTER.1 + ROBOT_Y / 2.0 + 10.0,
        BASE_Z + 8.0,
    );
    cassette_lip + connector_lip + robot_lip
}

fn sealed_cassette_nest() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_sealed_cassette_nest_body"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    body - cassette_recess_cuts()
        + cassette_guide_rails()
        + cassette_gasket_lands()
        + cassette_latch_lands()
        + cassette_orientation_keys()
        + cassette_chip_index_ticks()
        + pick_fiducials("sealed_cassette_nest", NEST_X, NEST_Y, NEST_Z)
}

fn cassette_recess_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_cassette_recess_cuts"));
    for bay in 0..CASSETTE_BAYS {
        let x = cassette_bay_x(bay);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_cassette_bay_{bay}_sealed_module_recess"),
                CASSETTE_RECESS_X,
                CASSETTE_RECESS_Y,
                CASSETTE_RECESS_DEPTH + 1.0,
            )
            .translate(x, 8.0, NEST_Z - CASSETTE_RECESS_DEPTH / 2.0 + 0.5);
    }
    cuts
}

fn cassette_guide_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_cassette_guide_rails"));
    for bay in 0..CASSETTE_BAYS {
        let x = cassette_bay_x(bay);
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_cassette_bay_{bay}_left_wipeable_guide_rail"),
                12.0,
                CASSETTE_RECESS_Y + 52.0,
                36.0,
            )
            .translate(x - CASSETTE_RECESS_X / 2.0 - 18.0, 8.0, NEST_Z + 18.0)
            + centered_cube(
                format!("{PART_PREFIX}_cassette_bay_{bay}_right_wipeable_guide_rail"),
                12.0,
                CASSETTE_RECESS_Y + 52.0,
                36.0,
            )
            .translate(x + CASSETTE_RECESS_X / 2.0 + 18.0, 8.0, NEST_Z + 18.0)
            + centered_cube(
                format!("{PART_PREFIX}_cassette_bay_{bay}_rear_orientation_stop"),
                CASSETTE_RECESS_X + 58.0,
                14.0,
                34.0,
            )
            .translate(x, CASSETTE_RECESS_Y / 2.0 + 38.0, NEST_Z + 17.0);
    }
    rails
}

fn cassette_gasket_lands() -> Part {
    let mut frames = Part::empty(format!("{PART_PREFIX}_cassette_gasket_lands"));
    for bay in 0..CASSETTE_BAYS {
        frames = frames
            + rectangular_frame(
                &format!("{PART_PREFIX}_cassette_bay_{bay}_sealed_gasket_witness_land"),
                CASSETTE_RECESS_X + 50.0,
                CASSETTE_RECESS_Y + 56.0,
                7.0,
                9.0,
            )
            .translate(cassette_bay_x(bay), 8.0, NEST_Z + 3.5);
    }
    frames
}

fn cassette_latch_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_cassette_latch_witness_lands"));
    for index in 0..SEAL_LATCH_LANDS {
        let bay = index / (SEAL_LATCH_LANDS / CASSETTE_BAYS);
        let local = index % (SEAL_LATCH_LANDS / CASSETTE_BAYS);
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_cassette_latch_witness_land_{index}"),
                38.0,
                16.0,
                7.0,
            )
            .translate(
                cassette_bay_x(bay) + index_offset(local, SEAL_LATCH_LANDS / CASSETTE_BAYS, 43.0),
                -NEST_Y / 2.0 + 26.0,
                NEST_Z + 3.5,
            );
    }
    lands
}

fn cassette_orientation_keys() -> Part {
    let mut keys = Part::empty(format!("{PART_PREFIX}_cassette_orientation_keys"));
    for index in 0..ORIENTATION_DATUM_KEYS {
        let bay = index / (ORIENTATION_DATUM_KEYS / CASSETTE_BAYS);
        let local = index % (ORIENTATION_DATUM_KEYS / CASSETTE_BAYS);
        keys = keys
            + centered_cylinder(
                format!("{PART_PREFIX}_cassette_bay_{bay}_orientation_key_{local}"),
                5.8,
                16.0,
                24,
            )
            .translate(
                cassette_bay_x(bay)
                    + index_offset(local, ORIENTATION_DATUM_KEYS / CASSETTE_BAYS, 36.0),
                NEST_Y / 2.0 - 34.0,
                NEST_Z + 8.0,
            );
    }
    keys
}

fn cassette_chip_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_cassette_chip_index_ticks"));
    for bay in 0..CASSETTE_BAYS {
        for chip in 0..CHIPS_PER_CASSETTE {
            ticks = ticks
                + centered_cube(
                    format!("{PART_PREFIX}_cassette_bay_{bay}_chip_position_tick_{chip}"),
                    4.0,
                    16.0,
                    4.0,
                )
                .translate(
                    cassette_bay_x(bay) - CASSETTE_RECESS_X / 2.0 + 14.0 + chip as f64 * 11.0,
                    -NEST_Y / 2.0 + 58.0,
                    NEST_Z + 2.0,
                );
        }
    }
    ticks
}

fn cassette_bay_x(bay: usize) -> f64 {
    index_offset(bay, CASSETTE_BAYS, CASSETTE_BAY_PITCH_X)
}

fn angle_witness_saddles() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_angle_witness_saddle_body"),
        ANGLE_X,
        ANGLE_Y,
        ANGLE_Z,
    );
    body - angle_saddle_recesses()
        + angle_saddle_tilt_rails()
        + angle_datum_ticks()
        + angle_witness_stop_combs()
        + pick_fiducials("angle_witness_saddles", ANGLE_X, ANGLE_Y, ANGLE_Z)
}

fn angle_saddle_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PART_PREFIX}_angle_saddle_recesses"));
    for saddle in 0..ANGLE_SADDLES {
        recesses = recesses
            + centered_cube(
                format!("{PART_PREFIX}_angle_saddle_{saddle}_cassette_footprint_recess"),
                ANGLE_SADDLE_X,
                ANGLE_SADDLE_Y,
                14.0,
            )
            .translate(angle_saddle_x(saddle), 0.0, ANGLE_Z - 6.5);
    }
    recesses
}

fn angle_saddle_tilt_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_angle_saddle_tilt_rails"));
    for saddle in 0..ANGLE_SADDLES {
        let x = angle_saddle_x(saddle);
        let angle = ANGLE_SADDLE_ANGLES_DEG[saddle];
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_angle_saddle_{saddle}_left_tilt_witness_rail"),
                10.0,
                ANGLE_SADDLE_Y + 14.0,
                18.0,
            )
            .rotate(angle, 0.0, 0.0)
            .translate(x - ANGLE_SADDLE_X / 2.0 - 8.0, 0.0, ANGLE_Z + 9.0)
            + centered_cube(
                format!("{PART_PREFIX}_angle_saddle_{saddle}_right_tilt_witness_rail"),
                10.0,
                ANGLE_SADDLE_Y + 14.0,
                18.0,
            )
            .rotate(angle, 0.0, 0.0)
            .translate(x + ANGLE_SADDLE_X / 2.0 + 8.0, 0.0, ANGLE_Z + 9.0)
            + centered_cube(
                format!("{PART_PREFIX}_angle_saddle_{saddle}_front_settle_stop"),
                ANGLE_SADDLE_X + 40.0,
                7.0,
                18.0,
            )
            .translate(x, -ANGLE_SADDLE_Y / 2.0 - 13.0, ANGLE_Z + 9.0)
            + centered_cube(
                format!("{PART_PREFIX}_angle_saddle_{saddle}_rear_settle_stop"),
                ANGLE_SADDLE_X + 40.0,
                7.0,
                18.0,
            )
            .translate(x, ANGLE_SADDLE_Y / 2.0 + 13.0, ANGLE_Z + 9.0);
    }
    rails
}

fn angle_datum_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_angle_datum_ticks"));
    for saddle in 0..ANGLE_SADDLES {
        let x = angle_saddle_x(saddle);
        for tick in 0..ANGLE_DATUM_TICKS_PER_SADDLE {
            ticks = ticks
                + centered_cube(
                    format!("{PART_PREFIX}_angle_saddle_{saddle}_datum_tick_{tick}"),
                    28.0,
                    3.0,
                    4.0,
                )
                .translate(
                    x,
                    index_offset(tick, ANGLE_DATUM_TICKS_PER_SADDLE, 38.0),
                    ANGLE_Z + 2.0,
                );
        }
    }
    ticks
}

fn angle_witness_stop_combs() -> Part {
    let mut combs = Part::empty(format!("{PART_PREFIX}_angle_witness_stop_combs"));
    for saddle in 0..ANGLE_SADDLES {
        let x = angle_saddle_x(saddle);
        for tooth in 0..4 {
            combs = combs
                + centered_cube(
                    format!("{PART_PREFIX}_angle_saddle_{saddle}_settling_comb_tooth_{tooth}"),
                    8.0,
                    22.0,
                    7.0 + tooth as f64,
                )
                .translate(
                    x + index_offset(tooth, 4, 18.0),
                    ANGLE_Y / 2.0 - 28.0,
                    ANGLE_Z + 3.5 + tooth as f64 / 2.0,
                );
        }
    }
    combs
}

fn angle_saddle_x(saddle: usize) -> f64 {
    index_offset(saddle, ANGLE_SADDLES, ANGLE_SADDLE_PITCH_X)
}

fn timed_dwell_lanes() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_timed_dwell_lane_body"),
        DWELL_X,
        DWELL_Y,
        DWELL_Z,
    );
    body - dwell_stop_recesses()
        + dwell_lane_rails()
        + dwell_tick_marks()
        + dwell_token_parking()
        + dwell_lane_gate_flags()
        + pick_fiducials("timed_dwell_lanes", DWELL_X, DWELL_Y, DWELL_Z)
}

fn dwell_stop_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PART_PREFIX}_dwell_stop_recesses"));
    for lane in 0..DWELL_LANES {
        for stop in 0..DWELL_STOPS_PER_LANE {
            let i = lane * DWELL_STOPS_PER_LANE + stop;
            recesses = recesses
                + centered_cube(
                    format!("{PART_PREFIX}_dwell_lane_{lane}_stop_recess_{stop}"),
                    DWELL_SLOT_X,
                    DWELL_SLOT_Y,
                    DWELL_SLOT_DEPTH + 1.0,
                )
                .translate(
                    index_offset(stop, DWELL_STOPS_PER_LANE, DWELL_STOP_PITCH_X),
                    dwell_lane_y(lane),
                    DWELL_Z - DWELL_SLOT_DEPTH / 2.0 + 0.5,
                )
                + centered_cylinder(
                    format!("{PART_PREFIX}_dwell_lane_{lane}_token_pin_clearance_{stop}"),
                    3.0,
                    DWELL_Z + 1.0,
                    20,
                )
                .translate(
                    index_offset(stop, DWELL_STOPS_PER_LANE, DWELL_STOP_PITCH_X),
                    dwell_lane_y(lane),
                    DWELL_Z / 2.0,
                );
            assert!(i < DWELL_STOP_SLOTS);
        }
    }
    recesses
}

fn dwell_lane_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_dwell_lane_rails"));
    for lane in 0..DWELL_LANES {
        let y = dwell_lane_y(lane);
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_dwell_lane_{lane}_front_timer_rail"),
                DWELL_X - 62.0,
                5.0,
                12.0,
            )
            .translate(0.0, y - DWELL_SLOT_Y / 2.0 - 12.0, DWELL_Z + 6.0)
            + centered_cube(
                format!("{PART_PREFIX}_dwell_lane_{lane}_rear_timer_rail"),
                DWELL_X - 62.0,
                5.0,
                12.0,
            )
            .translate(0.0, y + DWELL_SLOT_Y / 2.0 + 12.0, DWELL_Z + 6.0);
    }
    rails
}

fn dwell_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_dwell_tick_marks"));
    for lane in 0..DWELL_LANES {
        for tick in 0..DWELL_TICK_MARKS_PER_LANE {
            ticks = ticks
                + centered_cube(
                    format!("{PART_PREFIX}_dwell_lane_{lane}_elapsed_marker_tick_{tick}"),
                    4.0,
                    30.0,
                    5.0,
                )
                .translate(
                    index_offset(tick, DWELL_TICK_MARKS_PER_LANE, DWELL_STOP_PITCH_X),
                    dwell_lane_y(lane),
                    DWELL_Z + 2.5,
                );
        }
    }
    ticks
}

fn dwell_token_parking() -> Part {
    let mut tokens = Part::empty(format!("{PART_PREFIX}_dwell_token_parking"));
    for lane in 0..DWELL_LANES {
        tokens = tokens
            + annular_collar(
                &format!("{PART_PREFIX}_dwell_lane_{lane}_start_token_collar"),
                34.0,
                21.0,
                8.0,
            )
            .translate(-DWELL_X / 2.0 + 34.0, dwell_lane_y(lane), DWELL_Z + 4.0)
            + annular_collar(
                &format!("{PART_PREFIX}_dwell_lane_{lane}_finish_token_collar"),
                34.0,
                21.0,
                8.0,
            )
            .translate(DWELL_X / 2.0 - 34.0, dwell_lane_y(lane), DWELL_Z + 4.0);
    }
    tokens
}

fn dwell_lane_gate_flags() -> Part {
    let mut flags = Part::empty(format!("{PART_PREFIX}_dwell_lane_gate_flags"));
    for lane in 0..DWELL_LANES {
        flags = flags
            + centered_cube(
                format!("{PART_PREFIX}_dwell_lane_{lane}_status_flag_land"),
                54.0,
                15.0,
                5.0,
            )
            .translate(DWELL_X / 2.0 - 86.0, dwell_lane_y(lane), DWELL_Z + 2.5);
    }
    flags
}

fn dwell_lane_y(lane: usize) -> f64 {
    index_offset(lane, DWELL_LANES, DWELL_LANE_PITCH_Y)
}

fn robot_gripper_datum_clearance_envelope() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_robot_gripper_datum_plate"),
        ROBOT_X,
        ROBOT_Y,
        ROBOT_Z,
    );
    body - gripper_clearance_window()
        + gripper_datum_pins()
        + clearance_rails()
        + gripper_finger_shadow_gauges()
        + pick_fiducials("robot_gripper_datum", ROBOT_X, ROBOT_Y, ROBOT_Z)
}

fn gripper_clearance_window() -> Part {
    centered_cube(
        format!("{PART_PREFIX}_robot_gripper_finger_clearance_window"),
        GRIPPER_FINGER_CLEAR_X,
        GRIPPER_FINGER_CLEAR_Y,
        18.0,
    )
    .translate(0.0, 0.0, ROBOT_Z - 8.5)
}

fn gripper_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{PART_PREFIX}_gripper_datum_pins"));
    for pin in 0..GRIPPER_DATUM_PINS {
        pins = pins
            + centered_cylinder(
                format!("{PART_PREFIX}_gripper_datum_pin_{pin}"),
                5.0,
                24.0,
                28,
            )
            .translate(
                index_offset(pin % 3, 3, 92.0),
                index_offset(pin / 3, 2, 82.0),
                ROBOT_Z + 12.0,
            );
    }
    pins
}

fn clearance_rails() -> Part {
    let mut rails = Part::empty(format!("{PART_PREFIX}_gripper_clearance_rails"));
    for rail in 0..CLEARANCE_RAILS {
        let side = if rail % 2 == 0 { -1.0 } else { 1.0 };
        let tier = rail / 2;
        rails = rails
            + centered_cube(
                format!("{PART_PREFIX}_gripper_clearance_rail_{rail}"),
                GRIPPER_FINGER_CLEAR_X + 34.0,
                5.0,
                16.0 + tier as f64 * 8.0,
            )
            .translate(
                0.0,
                side * (GRIPPER_FINGER_CLEAR_Y / 2.0 + 18.0 + tier as f64 * 12.0),
                ROBOT_Z + 8.0 + tier as f64 * 4.0,
            );
    }
    rails
}

fn gripper_finger_shadow_gauges() -> Part {
    let left = centered_cube(
        format!("{PART_PREFIX}_left_gripper_finger_shadow_gauge"),
        18.0,
        ROBOT_Y - 36.0,
        12.0,
    )
    .translate(-GRIPPER_FINGER_CLEAR_X / 2.0 - 22.0, 0.0, ROBOT_Z + 6.0);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_gripper_finger_shadow_gauge"),
        18.0,
        ROBOT_Y - 36.0,
        12.0,
    )
    .translate(GRIPPER_FINGER_CLEAR_X / 2.0 + 22.0, 0.0, ROBOT_Z + 6.0);
    left + right
}

fn vibration_settling_isolation_pads() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_vibration_settling_pad_body"),
        SETTLING_X,
        SETTLING_Y,
        SETTLING_Z,
    );
    body - settling_dock_recesses()
        + isolation_pad_collars()
        + settling_rib_fields()
        + vibration_reference_masses()
        + pick_fiducials(
            "vibration_settling_isolation",
            SETTLING_X,
            SETTLING_Y,
            SETTLING_Z,
        )
}

fn settling_dock_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PART_PREFIX}_settling_dock_recesses"));
    for dock in 0..SETTLING_CASSETTE_DOCKS {
        recesses = recesses
            + centered_cube(
                format!("{PART_PREFIX}_settling_dock_{dock}_cassette_rest_recess"),
                104.0,
                116.0,
                12.0,
            )
            .translate(
                index_offset(dock, SETTLING_CASSETTE_DOCKS, 122.0),
                0.0,
                SETTLING_Z - 5.5,
            );
    }
    recesses
}

fn isolation_pad_collars() -> Part {
    let mut pads = Part::empty(format!("{PART_PREFIX}_isolation_pad_collars"));
    for pad in 0..ISOLATION_PADS {
        pads = pads
            + annular_collar(
                &format!("{PART_PREFIX}_settling_isolation_pad_{pad}"),
                ISOLATION_PAD_D,
                ISOLATION_PAD_D - 18.0,
                10.0,
            )
            .translate(
                index_offset(pad % 4, 4, 124.0),
                index_offset(pad / 4, 2, 142.0),
                SETTLING_Z + 5.0,
            );
    }
    pads
}

fn settling_rib_fields() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_settling_rib_fields"));
    for dock in 0..SETTLING_CASSETTE_DOCKS {
        let dock_x = index_offset(dock, SETTLING_CASSETTE_DOCKS, 122.0);
        for rib in 0..SETTLING_RIBS_PER_DOCK {
            ribs = ribs
                + centered_cube(
                    format!("{PART_PREFIX}_settling_dock_{dock}_soft_contact_rib_{rib}"),
                    5.0,
                    104.0,
                    6.0,
                )
                .translate(
                    dock_x + index_offset(rib, SETTLING_RIBS_PER_DOCK, 17.0),
                    0.0,
                    SETTLING_Z + 3.0,
                );
        }
    }
    ribs
}

fn vibration_reference_masses() -> Part {
    let mut masses = Part::empty(format!("{PART_PREFIX}_vibration_reference_masses"));
    for dock in 0..SETTLING_CASSETTE_DOCKS {
        masses = masses
            + centered_cube(
                format!("{PART_PREFIX}_settling_dock_{dock}_reference_mass_land"),
                56.0,
                16.0,
                8.0,
            )
            .translate(
                index_offset(dock, SETTLING_CASSETTE_DOCKS, 122.0),
                -SETTLING_Y / 2.0 + 24.0,
                SETTLING_Z + 4.0,
            );
    }
    masses
}

fn optical_dye_distribution_witness_slots() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_optical_dye_witness_body"),
        OPTICAL_X,
        OPTICAL_Y,
        OPTICAL_Z,
    );
    body - optical_witness_slot_cuts()
        + witness_slot_lips()
        + backlight_channel_ribs()
        + dye_uniformity_index_ticks()
        + pick_fiducials("optical_dye_witness", OPTICAL_X, OPTICAL_Y, OPTICAL_Z)
}

fn optical_witness_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_optical_witness_slot_cuts"));
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let index = row * WITNESS_COLS + col;
            cuts = cuts
                + centered_cube(
                    format!("{PART_PREFIX}_dye_distribution_witness_slot_{index}"),
                    WITNESS_SLOT_X,
                    WITNESS_SLOT_Y,
                    WITNESS_SLOT_DEPTH + 1.0,
                )
                .translate(
                    index_offset(col, WITNESS_COLS, WITNESS_PITCH_X),
                    index_offset(row, WITNESS_ROWS, WITNESS_PITCH_Y),
                    OPTICAL_Z - WITNESS_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }
    cuts
}

fn witness_slot_lips() -> Part {
    let mut lips = Part::empty(format!("{PART_PREFIX}_witness_slot_lips"));
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let index = row * WITNESS_COLS + col;
            let x = index_offset(col, WITNESS_COLS, WITNESS_PITCH_X);
            let y = index_offset(row, WITNESS_ROWS, WITNESS_PITCH_Y);
            lips = lips
                + centered_cube(
                    format!("{PART_PREFIX}_witness_slot_{index}_lower_locator_lip"),
                    WITNESS_SLOT_X + 7.0,
                    3.0,
                    5.0,
                )
                .translate(x, y - WITNESS_SLOT_Y / 2.0 - 4.0, OPTICAL_Z + 2.5)
                + centered_cube(
                    format!("{PART_PREFIX}_witness_slot_{index}_barcode_edge_lip"),
                    3.0,
                    WITNESS_SLOT_Y + 7.0,
                    5.0,
                )
                .translate(x + WITNESS_SLOT_X / 2.0 + 4.0, y, OPTICAL_Z + 2.5);
        }
    }
    lips
}

fn backlight_channel_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PART_PREFIX}_backlight_channel_ribs"));
    for row in 0..WITNESS_ROWS {
        ribs = ribs
            + centered_cube(
                format!("{PART_PREFIX}_witness_row_{row}_backlight_rib"),
                OPTICAL_X - 58.0,
                3.0,
                6.0,
            )
            .translate(
                0.0,
                index_offset(row, WITNESS_ROWS, WITNESS_PITCH_Y) + WITNESS_SLOT_Y / 2.0 + 7.0,
                OPTICAL_Z + 3.0,
            );
    }
    ribs
}

fn dye_uniformity_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_dye_uniformity_index_ticks"));
    for col in 0..WITNESS_COLS {
        ticks = ticks
            + centered_cube(
                format!("{PART_PREFIX}_dye_witness_column_tick_{col}"),
                22.0,
                4.0,
                4.0,
            )
            .translate(
                index_offset(col, WITNESS_COLS, WITNESS_PITCH_X),
                OPTICAL_Y / 2.0 - 20.0,
                OPTICAL_Z + 2.0,
            );
    }
    for row in 0..WITNESS_ROWS {
        ticks = ticks
            + centered_cube(
                format!("{PART_PREFIX}_dye_witness_row_tick_{row}"),
                4.0,
                18.0,
                4.0,
            )
            .translate(
                -OPTICAL_X / 2.0 + 22.0,
                index_offset(row, WITNESS_ROWS, WITNESS_PITCH_Y),
                OPTICAL_Z + 2.0,
            );
    }
    ticks
}

fn barcode_custody_status_surfaces() -> Part {
    let body = module_panel(
        &format!("{PART_PREFIX}_barcode_custody_status_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    body + barcode_lands() + custody_punch_lands() + status_lane_pockets() + scan_alignment_ticks()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_barcode_lands"));
    for land in 0..BARCODE_LANDS {
        let x = index_offset(land % 4, 4, 66.0);
        let y = index_offset(land / 4, 4, 23.0);
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_barcode_land_{land}"),
                48.0,
                14.0,
                3.0,
            )
            .translate(x, y, CUSTODY_Z + 1.5)
            + barcode_stripes(
                &format!("{PART_PREFIX}_barcode_land"),
                land,
                x,
                y,
                CUSTODY_Z + 3.2,
            );
    }
    lands
}

fn custody_punch_lands() -> Part {
    let mut punches = Part::empty(format!("{PART_PREFIX}_custody_punch_lands"));
    for punch in 0..CUSTODY_PUNCH_LANDS {
        punches = punches
            + centered_cylinder(
                format!("{PART_PREFIX}_custody_punch_land_{punch}"),
                4.0,
                3.0,
                20,
            )
            .translate(
                CUSTODY_X / 2.0 - 28.0,
                index_offset(punch, CUSTODY_PUNCH_LANDS, 12.0),
                CUSTODY_Z + 1.5,
            );
    }
    punches
}

fn status_lane_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PART_PREFIX}_status_lane_pockets"));
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            pockets = pockets
                + centered_cube(
                    format!("{PART_PREFIX}_custody_status_lane_{lane}_slot_{slot}"),
                    32.0,
                    9.0,
                    4.0,
                )
                .translate(
                    -CUSTODY_X / 2.0 + 26.0 + slot as f64 * 38.0,
                    index_offset(lane, STATUS_LANES, 25.0),
                    CUSTODY_Z + 2.0,
                );
        }
    }
    pockets
}

fn scan_alignment_ticks() -> Part {
    centered_cube(
        format!("{PART_PREFIX}_custody_scan_left_alignment_tick"),
        5.0,
        CUSTODY_Y - 26.0,
        4.0,
    )
    .translate(-CUSTODY_X / 2.0 + 20.0, 0.0, CUSTODY_Z + 2.0)
        + centered_cube(
            format!("{PART_PREFIX}_custody_scan_right_alignment_tick"),
            5.0,
            CUSTODY_Y - 26.0,
            4.0,
        )
        .translate(CUSTODY_X / 2.0 - 20.0, 0.0, CUSTODY_Z + 2.0)
}

fn closed_connector_handoff_bulkhead() -> Part {
    let base = module_panel(
        &format!("{PART_PREFIX}_closed_connector_handoff_base"),
        CONNECTOR_X,
        CONNECTOR_Y,
        CONNECTOR_Z / 2.0,
    );
    let wall = centered_cube(
        format!("{PART_PREFIX}_incubator_seeding_manifold_handoff_bulkhead"),
        CONNECTOR_X - 64.0,
        22.0,
        CONNECTOR_Z,
    )
    .translate(0.0, 0.0, CONNECTOR_Z / 2.0);
    base + (wall - connector_port_cuts())
        + connector_gasket_frames()
        + connector_cap_parking()
        + connector_route_key_blocks()
}

fn connector_port_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PART_PREFIX}_connector_port_cuts"));
    for port in 0..CLOSED_CONNECTOR_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_closed_connector_port_cut_{port}"),
                CONNECTOR_PORT_D / 2.0,
                34.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                index_offset(port, CLOSED_CONNECTOR_PORTS, 84.0),
                0.0,
                CONNECTOR_Z / 2.0 + 7.0,
            );
    }
    cuts
}

fn connector_gasket_frames() -> Part {
    let mut frames = Part::empty(format!("{PART_PREFIX}_connector_gasket_frames"));
    for port in 0..CLOSED_CONNECTOR_PORTS {
        frames = frames
            + annular_collar(
                &format!("{PART_PREFIX}_closed_connector_port_{port}_gasket_land"),
                CONNECTOR_PORT_D + GASKET_FRAME_W,
                CONNECTOR_PORT_D + 3.0,
                7.0,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                index_offset(port, CLOSED_CONNECTOR_PORTS, 84.0),
                -15.0,
                CONNECTOR_Z / 2.0 + 7.0,
            );
    }
    frames
}

fn connector_cap_parking() -> Part {
    let mut parks = Part::empty(format!("{PART_PREFIX}_connector_cap_parking"));
    for well in 0..CAP_PARKING_WELLS {
        parks = parks
            + annular_collar(
                &format!("{PART_PREFIX}_closed_connector_cap_parking_well_{well}"),
                30.0,
                18.0,
                8.0,
            )
            .translate(
                index_offset(well, CAP_PARKING_WELLS, 84.0),
                -CONNECTOR_Y / 2.0 + 28.0,
                CONNECTOR_Z / 2.0 + 4.0,
            );
    }
    parks
}

fn connector_route_key_blocks() -> Part {
    let incubator_key = centered_cube(
        format!("{PART_PREFIX}_incubator_handoff_route_key"),
        112.0,
        16.0,
        22.0,
    )
    .translate(-110.0, CONNECTOR_Y / 2.0 - 24.0, CONNECTOR_Z / 2.0 + 11.0);
    let manifold_key = centered_cube(
        format!("{PART_PREFIX}_seeding_manifold_handoff_route_key"),
        112.0,
        16.0,
        22.0,
    )
    .translate(110.0, CONNECTOR_Y / 2.0 - 24.0, CONNECTOR_Z / 2.0 + 11.0);
    incubator_key + manifold_key
}

fn evidence_bridge() -> Part {
    let mut bridge = Part::empty(format!("{PART_PREFIX}_evidence_bridge"));
    for (index, (x, y)) in [
        (-BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (-BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("{PART_PREFIX}_evidence_bridge_post_{index}"),
                BRIDGE_POST_W,
                BRIDGE_POST_W,
                BRIDGE_UNDERSIDE_Z,
            )
            .translate(x, y, BRIDGE_UNDERSIDE_Z / 2.0);
    }
    bridge
        + centered_cube(
            format!("{PART_PREFIX}_front_evidence_bridge_beam"),
            BRIDGE_SPAN_X + BRIDGE_POST_W,
            BRIDGE_POST_W,
            BRIDGE_BEAM_Z,
        )
        .translate(
            0.0,
            -BRIDGE_SPAN_Y / 2.0,
            BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
        )
        + centered_cube(
            format!("{PART_PREFIX}_rear_evidence_bridge_beam"),
            BRIDGE_SPAN_X + BRIDGE_POST_W,
            BRIDGE_POST_W,
            BRIDGE_BEAM_Z,
        )
        .translate(
            0.0,
            BRIDGE_SPAN_Y / 2.0,
            BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
        )
        + centered_cube(
            format!("{PART_PREFIX}_left_evidence_bridge_beam"),
            BRIDGE_POST_W,
            BRIDGE_SPAN_Y + BRIDGE_POST_W,
            BRIDGE_BEAM_Z,
        )
        .translate(
            -BRIDGE_SPAN_X / 2.0,
            0.0,
            BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
        )
        + centered_cube(
            format!("{PART_PREFIX}_right_evidence_bridge_beam"),
            BRIDGE_POST_W,
            BRIDGE_SPAN_Y + BRIDGE_POST_W,
            BRIDGE_BEAM_Z,
        )
        .translate(
            BRIDGE_SPAN_X / 2.0,
            0.0,
            BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
        )
        + camera_pods()
        + bridge_light_bars()
        + bridge_fiducials()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_camera_pods"));
    for pod in 0..CAMERA_PODS {
        let x = index_offset(pod, CAMERA_PODS, 230.0);
        let pod_body = centered_cube(
            format!("{PART_PREFIX}_camera_pod_{pod}_body"),
            74.0,
            42.0,
            22.0,
        )
        .translate(x, 0.0, BRIDGE_UNDERSIDE_Z - 13.0);
        let lens = centered_cylinder(
            format!("{PART_PREFIX}_camera_pod_{pod}_lens_clearance"),
            12.0,
            24.0,
            32,
        )
        .translate(x, 0.0, BRIDGE_UNDERSIDE_Z - 13.0);
        pods = pods + (pod_body - lens);
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{PART_PREFIX}_bridge_light_bars"));
    for bar in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{PART_PREFIX}_evidence_light_bar_{bar}"),
                210.0,
                11.0,
                10.0,
            )
            .translate(
                index_offset(bar, LIGHT_BARS, 300.0),
                -BRIDGE_SPAN_Y / 2.0 + 42.0,
                BRIDGE_UNDERSIDE_Z - 10.0,
            );
    }
    bars
}

fn bridge_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PART_PREFIX}_bridge_fiducials"));
    for index in 0..FIDUCIAL_DISCS {
        let x = index_offset(index % 8, 8, 150.0);
        let y = if index < 8 { -1.0 } else { 1.0 } * (BRIDGE_SPAN_Y / 2.0 - 72.0);
        fiducials = fiducials
            + fiducial_disc(&format!("{PART_PREFIX}_bridge_fiducial_{index}")).translate(x, y, 3.0);
    }
    fiducials
}

fn service_keepout_gauges() -> Part {
    let front = keepout_plate(
        "front_robot_approach_clearance",
        DECK_X - 190.0,
        10.0,
        FRONT_ROBOT_CLEARANCE_Y,
    )
    .translate(0.0, -DECK_Y / 2.0 - 22.0, 5.0);
    let rear = keepout_plate(
        "rear_incubator_return_clearance",
        DECK_X - 190.0,
        10.0,
        REAR_INCUBATOR_CLEARANCE_Y,
    )
    .translate(0.0, DECK_Y / 2.0 + 22.0, 5.0);
    let left = keepout_plate(
        "left_cassette_service_clearance",
        10.0,
        DECK_Y - 180.0,
        LEFT_CASSETTE_SERVICE_X,
    )
    .translate(-DECK_X / 2.0 - 22.0, 0.0, 5.0);
    let right = keepout_plate(
        "right_manifold_service_clearance",
        10.0,
        DECK_Y - 180.0,
        RIGHT_MANIFOLD_SERVICE_X,
    )
    .translate(DECK_X / 2.0 + 22.0, 0.0, 5.0);
    front + rear + left + right + keepout_posts()
}

fn keepout_plate(name: &str, x: f64, y: f64, z: f64) -> Part {
    centered_cube(format!("{PART_PREFIX}_{name}_gauge"), x, y, 10.0).translate(0.0, 0.0, 0.0)
        + centered_cube(format!("{PART_PREFIX}_{name}_height_post"), 12.0, 12.0, z).translate(
            0.0,
            0.0,
            z / 2.0,
        )
}

fn keepout_posts() -> Part {
    let mut posts = Part::empty(format!("{PART_PREFIX}_keepout_posts"));
    for post in 0..KEEP_OUT_GAUGES {
        let side_x = if post % 2 == 0 { -1.0 } else { 1.0 };
        let y = index_offset(post / 2, KEEP_OUT_GAUGES / 2, 170.0);
        posts = posts
            + centered_cylinder(
                format!("{PART_PREFIX}_robot_service_keepout_post_{post}"),
                7.0,
                118.0,
                24,
            )
            .translate(side_x * (DECK_X / 2.0 + 42.0), y, 59.0);
    }
    posts
}

fn module_panel(name: &str, x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y, z).translate(0.0, 0.0, z / 2.0)
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_opening"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 0.2,
    );
    outer - inner
}

fn annular_collar(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(
            format!("{name}_inner_clearance"),
            inner_d / 2.0,
            z + 0.2,
            36,
        )
}

fn pick_fiducials(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    fiducial_disc(&format!("{PART_PREFIX}_{prefix}_left_pick_fiducial")).translate(
        -x / 2.0 + 24.0,
        -y / 2.0 + 22.0,
        z + 1.5,
    ) + fiducial_disc(&format!("{PART_PREFIX}_{prefix}_right_pick_fiducial")).translate(
        x / 2.0 - 24.0,
        -y / 2.0 + 22.0,
        z + 1.5,
    )
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 7.0, 3.0, 32);
    let center = centered_cylinder(format!("{name}_center_mark"), 2.0, 4.0, 20);
    (outer - center).translate(0.0, 0.0, 1.5)
}

fn barcode_stripes(prefix: &str, index: usize, x: f64, y: f64, z: f64) -> Part {
    let mut stripes = Part::empty(format!("{prefix}_{index}_stripes"));
    for (bar, dx) in [-15.0, -10.0, -6.0, -1.0, 5.0, 11.0, 16.0]
        .into_iter()
        .enumerate()
    {
        let width = if bar % 2 == 0 { 2.0 } else { 1.0 };
        stripes =
            stripes
                + centered_cube(format!("{prefix}_{index}_stripe_{bar}"), width, 10.0, 1.2)
                    .translate(x + dx, y, z);
    }
    stripes
}

fn index_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_stable_and_station_scoped() {
        let manifest = OUTPUTS.join("\n");
        assert_eq!(
            manifest,
            "output/closed_robotic_cassette_seeding_angle_and_dwell_station_closed_base_tray.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_sealed_cassette_nest.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_angle_witness_saddles.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_timed_dwell_lanes.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_robot_gripper_datum_clearance_envelope.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_vibration_settling_isolation_pads.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_optical_dye_distribution_witness_slots.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_barcode_custody_status_surfaces.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_closed_connector_handoff_bulkhead.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_evidence_bridge.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_service_keepout_gauges.stl\n\
output/closed_robotic_cassette_seeding_angle_and_dwell_station_assembly.stl"
        );
        assert_eq!(
            OUTPUTS.last().copied(),
            Some("output/closed_robotic_cassette_seeding_angle_and_dwell_station_assembly.stl")
        );
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_robotic_cassette_seeding_angle_and_dwell_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_counts_cover_many_chip_seeding_validation() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert_eq!(total_chip_positions(), 24);
        assert!(WITNESS_SLOTS >= 2 * total_chip_positions());
        assert!(DWELL_STOP_SLOTS >= total_chip_positions());
        assert_eq!(ANGLE_SADDLES, ANGLE_SADDLE_ANGLES_DEG.len());
        assert!(ISOLATION_PADS >= SETTLING_CASSETTE_DOCKS * 2);
        assert_eq!(CLOSED_CONNECTOR_PORTS, CAP_PARKING_WELLS);
    }

    #[test]
    fn station_bounds_and_major_modules_do_not_overlap() {
        assert_layout();
        assert!(DECK_X <= 1600.0);
        assert!(DECK_Y <= 1000.0);
        assert!(highest_fixture_feature_z() < OVERHEAD_GRIPPER_CLEARANCE_Z);
    }

    #[test]
    fn reproducibility_controls_are_explicit() {
        assert_eq!(DESIGN_REVISION, "T-864F0959-seeding-angle-dwell-r1");
        assert_eq!(UNITS, "millimeters");
        assert_eq!(GEOMETRY_SEED, 0);
        assert_eq!(FLOAT_GRID_MM, 0.5);
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"stable_output_manifest_order"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"no_random_sampling"));
    }

    #[test]
    fn fixture_intent_avoids_clinical_acceptance_thresholds() {
        assert!(!CLINICAL_ACCEPTANCE_THRESHOLDS_ENCODED);
        assert_eq!(ANGLE_SADDLE_ANGLES_DEG, [-6.0, -3.0, 3.0, 6.0]);
        assert!(FRONT_ROBOT_CLEARANCE_Y > 0.0);
        assert!(REAR_INCUBATOR_CLEARANCE_Y > 0.0);
        assert!(LEFT_CASSETTE_SERVICE_X > 0.0);
        assert!(RIGHT_MANIFOLD_SERVICE_X > 0.0);
    }
}
