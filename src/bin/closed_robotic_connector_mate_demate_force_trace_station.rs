use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic connector mate/demate force trace station.
//
// Intent:
// - Qualify automated mate/demate of closed fluid, gas, and electrical service
//   connectors with measured force/torque and insertion-depth evidence instead
//   of operator feel.
// - Keep male/female connector nests, cap/plug custody, leak/continuity witness
//   ports, cycle tokens, barcode custody, release/hold/reject lanes, and
//   evidence capture physically separated on one metrology deck.
// - Expose robot/service keepouts and misalignment challenge wedges for parent
//   workcell planning without claiming a validated connector or test method.

const OUTPUTS: [&str; 13] = [
    "output/closed_robotic_connector_mate_demate_force_trace_station_base_metrology_deck.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_male_female_connector_nests.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_force_torque_sensor_pocket.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_insertion_depth_gauge.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_angular_misalignment_wedges.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_cycle_count_token_rail.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_leak_continuity_witness_ports.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_cap_plug_custody_parks.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_release_hold_reject_lanes.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_barcode_custody_lands.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_evidence_camera_bridge.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_robot_service_keepout_gauges.stl",
    "output/closed_robotic_connector_mate_demate_force_trace_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "male_female_connector_nests",
    "force_torque_sensor_pocket",
    "insertion_depth_gauge",
    "angular_misalignment_wedges",
    "cycle_count_token_rail",
    "leak_continuity_witness_ports",
    "cap_plug_custody_parks",
    "release_hold_reject_lanes",
    "barcode_custody_lands",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;

const NEST_X: f64 = 520.0;
const NEST_Y: f64 = 250.0;
const NEST_Z: f64 = 58.0;
const NEST_POS: (f64, f64) = (-300.0, 220.0);
const CONNECTOR_PAIR_ROWS: usize = 2;
const CONNECTOR_PAIR_COLS: usize = 3;
const CONNECTOR_PAIR_COUNT: usize = CONNECTOR_PAIR_ROWS * CONNECTOR_PAIR_COLS;
const CONNECTORS_PER_PAIR: usize = 2;
const CONNECTOR_COUNT: usize = CONNECTOR_PAIR_COUNT * CONNECTORS_PER_PAIR;
const PAIR_PITCH_X: f64 = 158.0;
const PAIR_PITCH_Y: f64 = 92.0;
const MALE_SOCKET_D: f64 = 18.0;
const FEMALE_SOCKET_D: f64 = 24.0;
const STEM_BORE_D: f64 = 7.0;
const KEY_SLOT_X: f64 = 9.0;
const INSERTION_AXIS_Y: f64 = 12.0;

const SENSOR_X: f64 = 340.0;
const SENSOR_Y: f64 = 220.0;
const SENSOR_Z: f64 = 70.0;
const SENSOR_POS: (f64, f64) = (250.0, 220.0);
const SENSOR_RECESS_X: f64 = 132.0;
const SENSOR_RECESS_Y: f64 = 132.0;
const SENSOR_RECESS_Z: f64 = 34.0;
const SENSOR_BOLT_COUNT: usize = 4;
const TORQUE_REACTION_DOGS: usize = 4;
const CALIBRATION_MASS_POCKETS: usize = 4;

const DEPTH_X: f64 = 496.0;
const DEPTH_Y: f64 = 110.0;
const DEPTH_Z: f64 = 40.0;
const DEPTH_POS: (f64, f64) = (-315.0, -45.0);
const DEPTH_WINDOWS: usize = 8;
const DEPTH_REFERENCE_STOPS: usize = 2;
const DEPTH_SCALE_PITCH: f64 = 52.0;

const WEDGE_X: f64 = 300.0;
const WEDGE_Y: f64 = 120.0;
const WEDGE_Z: f64 = 38.0;
const WEDGE_POS: (f64, f64) = (190.0, -62.0);
const WEDGE_COUNT: usize = 5;
const WEDGE_STEPS: usize = 5;
const MAX_MISALIGN_DEG: f64 = 4.0;

const CYCLE_X: f64 = 500.0;
const CYCLE_Y: f64 = 120.0;
const CYCLE_Z: f64 = 34.0;
const CYCLE_POS: (f64, f64) = (-310.0, -210.0);
const CYCLE_TOKEN_COUNT: usize = 32;
const CYCLE_TOKEN_D: f64 = 15.0;
const CYCLE_COUNTER_WINDOWS: usize = 8;
const TOKEN_RAILS: usize = 2;

const WITNESS_X: f64 = 390.0;
const WITNESS_Y: f64 = 140.0;
const WITNESS_Z: f64 = 62.0;
const WITNESS_POS: (f64, f64) = (210.0, -245.0);
const LEAK_PORTS: usize = 12;
const CONTINUITY_PAD_PAIRS: usize = 12;
const WITNESS_WELLS: usize = 12;
const LEAK_PORT_D: f64 = 8.0;
const CONTINUITY_PAD_D: f64 = 5.0;
const WITNESS_WELL_D: f64 = 12.0;

const CUSTODY_X: f64 = 190.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 42.0;
const CUSTODY_POS: (f64, f64) = (468.0, -55.0);
const CAP_WELLS: usize = 12;
const PLUG_WELLS: usize = 12;
const CUSTODY_TAG_LANDS: usize = 6;
const CAP_WELL_D: f64 = 10.0;
const PLUG_WELL_D: f64 = 8.0;

const STATUS_LANES: usize = 3;
const STATUS_X: f64 = 126.0;
const STATUS_Y: f64 = 210.0;
const STATUS_Z: f64 = 30.0;
const STATUS_POS: (f64, f64) = (496.0, 260.0);
const STATUS_SLOT_COUNT: usize = 4;
const STATUS_LANE_Y: f64 = 58.0;
const STATUS_LANE_PITCH_Y: f64 = 68.0;

const BARCODE_X: f64 = 500.0;
const BARCODE_Y: f64 = 60.0;
const BARCODE_Z: f64 = 12.0;
const BARCODE_POS: (f64, f64) = (0.0, -362.0);
const BARCODE_LANDS: usize = 6;
const LOT_LANDS: usize = 6;
const RFID_LANDS: usize = 3;

const BRIDGE_POST_SPAN_X: f64 = 980.0;
const BRIDGE_POST_SPAN_Y: f64 = 704.0;
const BRIDGE_POST_X: f64 = 26.0;
const BRIDGE_POST_Y: f64 = 40.0;
const BRIDGE_POST_Z: f64 = 190.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const BRIDGE_BEAM_Y: f64 = 40.0;
const CAMERA_CLEARANCE_Z: f64 = 142.0;
const CAMERA_PODS: usize = 3;
const LIGHT_BARS: usize = 2;

const KEEP_OUT_RAIL_Z: f64 = 8.0;
const KEEP_OUT_FLAGS: usize = 6;
const ROBOT_SWEEP_Y: f64 = -394.0;
const SERVICE_SWEEP_Y: f64 = 390.0;
const SERVICE_FLAG_Z: f64 = 62.0;
const INSERTION_AXIS_CLEARANCE: f64 = 86.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_metrology_deck();
    export(OUTPUTS[0], &base);

    let nests = male_female_connector_nests();
    export(OUTPUTS[1], &nests);

    let sensor = force_torque_sensor_pocket();
    export(OUTPUTS[2], &sensor);

    let depth = insertion_depth_gauge();
    export(OUTPUTS[3], &depth);

    let wedges = angular_misalignment_wedges();
    export(OUTPUTS[4], &wedges);

    let cycles = cycle_count_token_rail();
    export(OUTPUTS[5], &cycles);

    let witnesses = leak_continuity_witness_ports();
    export(OUTPUTS[6], &witnesses);

    let custody = cap_plug_custody_parks();
    export(OUTPUTS[7], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let barcode = barcode_custody_lands();
    export(OUTPUTS[9], &barcode);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + nests.translate(NEST_POS.0, NEST_POS.1, insert_z(NEST_Z))
        + sensor.translate(SENSOR_POS.0, SENSOR_POS.1, insert_z(SENSOR_Z))
        + depth.translate(DEPTH_POS.0, DEPTH_POS.1, insert_z(DEPTH_Z))
        + wedges.translate(WEDGE_POS.0, WEDGE_POS.1, insert_z(WEDGE_Z))
        + cycles.translate(CYCLE_POS.0, CYCLE_POS.1, insert_z(CYCLE_Z))
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + lanes.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + barcode.translate(BARCODE_POS.0, BARCODE_POS.1, insert_z(BARCODE_Z))
        + bridge.translate(0.0, 0.0, BASE_Z / 2.0 + BRIDGE_POST_Z / 2.0 + 8.0)
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_RAIL_Z / 2.0 + 4.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed robotic connector mate/demate force trace station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained metrology deck"
    );
    println!(
        "  Connector qualification:   {CONNECTOR_PAIR_COUNT} male/female pairs, {CONNECTOR_COUNT} connector nests, keyed insertion axes"
    );
    println!(
        "  Force trace hardware:      6-axis sensor pocket, {SENSOR_BOLT_COUNT} bolt holes, {TORQUE_REACTION_DOGS} torque reaction dogs, insertion-depth gauge"
    );
    println!(
        "  Misalignment challenge:    {WEDGE_COUNT} stepped angular wedges spanning +/-{MAX_MISALIGN_DEG:.0} deg"
    );
    println!(
        "  Cycle/custody evidence:    {CYCLE_TOKEN_COUNT} cycle tokens, {CAP_WELLS} cap wells, {PLUG_WELLS} plug wells, {BARCODE_LANDS} barcode lands"
    );
    println!(
        "  Witness ports:             {LEAK_PORTS} leak ports, {CONTINUITY_PAD_PAIRS} continuity pad pairs, {WITNESS_WELLS} witness wells"
    );
    println!(
        "  Disposition/automation:    release-hold-reject lanes, {CAMERA_PODS} camera pods, {KEEP_OUT_FLAGS} robot/service keepout flags"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0 + 6.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13, "export count changed");
    assert_eq!(REQUIRED_FEATURES.len(), 11, "required feature list changed");
    assert_eq!(
        CONNECTOR_PAIR_COUNT,
        CONNECTOR_PAIR_ROWS * CONNECTOR_PAIR_COLS
    );
    assert_eq!(CONNECTOR_COUNT, CONNECTOR_PAIR_COUNT * CONNECTORS_PER_PAIR);
    assert!(CYCLE_TOKEN_COUNT >= CONNECTOR_COUNT * 2);
    assert!(CAP_WELLS >= CONNECTOR_COUNT);
    assert!(PLUG_WELLS >= CONNECTOR_COUNT);
    assert_eq!(LEAK_PORTS, WITNESS_WELLS);
    assert_eq!(CONTINUITY_PAD_PAIRS, LEAK_PORTS);
    assert!(FEMALE_SOCKET_D > MALE_SOCKET_D);
    assert!(SENSOR_RECESS_X < SENSOR_X);
    assert!(SENSOR_RECESS_Y < SENSOR_Y);
    assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(INSERTION_AXIS_CLEARANCE > FEMALE_SOCKET_D + MALE_SOCKET_D);

    for (name, rect) in insert_rects() {
        assert!(
            fits_on_station(rect),
            "{name} exceeds station envelope: {rect:?}"
        );
    }

    let rects = insert_rects();
    for (index, (left_name, left)) in rects.iter().enumerate() {
        for (right_name, right) in rects.iter().skip(index + 1) {
            assert!(
                !rects_overlap(*left, *right),
                "{left_name} overlaps {right_name}"
            );
        }
    }
}

fn base_metrology_deck() -> Part {
    let floor = centered_cube(
        "connector_force_trace_base_metrology_deck_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let shallow_sump = centered_cube(
        "connector_force_trace_base_shallow_witness_sump",
        STATION_X - 132.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let front_drain_slot = centered_cube(
        "connector_force_trace_base_front_low_point_drain_slot",
        120.0,
        14.0,
        9.0,
    )
    .translate(
        STATION_X / 2.0 - 142.0,
        -STATION_Y / 2.0 + 30.0,
        BASE_Z / 2.0 - 4.0,
    );

    floor - shallow_sump - front_drain_slot - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + deck_fiducials()
        + leak_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("connector_force_trace_insert_sockets");
    for (name, rect) in insert_rects() {
        sockets = sockets
            + centered_cube(
                format!("connector_force_trace_{name}_deck_socket"),
                rect.w + 10.0,
                rect.h + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(rect.x, rect.y, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("connector_force_trace_base_mounting_slots");
    for (index, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("connector_force_trace_base_m6_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("connector_force_trace_base_m6_mount_slot_relief_{index}"),
            28.0,
            MOUNT_HOLE_D + 0.4,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "connector_force_trace_left_containment_rim",
        RIM_W,
        STATION_Y - 48.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "connector_force_trace_right_containment_rim",
        RIM_W,
        STATION_Y - 48.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "connector_force_trace_rear_service_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "connector_force_trace_front_low_robot_access_lip",
        STATION_X - 184.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 26.0, BASE_Z / 2.0 + 12.0);
    left + right + rear + front
}

fn deck_fiducials() -> Part {
    let mut fiducials = Part::empty("connector_force_trace_deck_fiducials");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 66.0, -STATION_Y / 2.0 + 66.0),
        (STATION_X / 2.0 - 66.0, -STATION_Y / 2.0 + 66.0),
        (-STATION_X / 2.0 + 66.0, STATION_Y / 2.0 - 66.0),
        (STATION_X / 2.0 - 66.0, STATION_Y / 2.0 - 66.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("connector_force_trace_deck_fiducial_{index}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("connector_force_trace_base_leak_witness_ribs");
    for index in 0..10 {
        ribs = ribs
            + centered_cube(
                format!("connector_force_trace_base_leak_witness_rib_{index}"),
                82.0,
                5.0,
                7.0,
            )
            .translate(centered_index(index, 10, 96.0), -396.0, BASE_Z / 2.0 + 3.5);
    }
    ribs
}

fn male_female_connector_nests() -> Part {
    let body = centered_cube(
        "connector_force_trace_male_female_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let rear_fence = centered_cube(
        "connector_force_trace_male_female_rear_datum_fence",
        NEST_X,
        14.0,
        NEST_Z + 34.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 7.0, 17.0);
    let left_fence = centered_cube(
        "connector_force_trace_male_female_left_robot_datum_fence",
        14.0,
        NEST_Y - 24.0,
        NEST_Z + 24.0,
    )
    .translate(-(NEST_X / 2.0 - 7.0), 0.0, 12.0);
    let right_fence = centered_cube(
        "connector_force_trace_male_female_right_connector_family_fence",
        10.0,
        NEST_Y - 62.0,
        NEST_Z + 18.0,
    )
    .translate(NEST_X / 2.0 - 18.0, -14.0, 9.0);

    body + rear_fence + left_fence + right_fence - connector_nest_cuts()
        + connector_clamp_lands()
        + insertion_axis_guides()
        + nest_family_witness_tabs()
        + nest_robot_fiducials()
}

fn connector_nest_cuts() -> Part {
    let mut cuts = Part::empty("connector_force_trace_connector_nest_cuts");
    for pair in 0..CONNECTOR_PAIR_COUNT {
        let (x, y) = connector_pair_center(pair);
        cuts = cuts
            + male_connector_pocket(pair, x - 34.0, y)
            + female_connector_pocket(pair, x + 34.0, y)
            + centered_cube(
                format!("connector_force_trace_pair_{pair}_insertion_axis_clearance_slot"),
                INSERTION_AXIS_CLEARANCE,
                INSERTION_AXIS_Y,
                24.0,
            )
            .translate(x, y, NEST_Z / 2.0 - 8.0);
    }
    cuts
}

fn male_connector_pocket(pair: usize, x: f64, y: f64) -> Part {
    let cradle = centered_cylinder(
        format!("connector_force_trace_pair_{pair}_male_round_cradle"),
        MALE_SOCKET_D / 2.0,
        58.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, y, NEST_Z / 2.0 - 11.0);
    let stem = centered_cylinder(
        format!("connector_force_trace_pair_{pair}_male_stem_bore"),
        STEM_BORE_D / 2.0,
        42.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x - 28.0, y, NEST_Z / 2.0 - 11.0);
    let thumb_relief = centered_cube(
        format!("connector_force_trace_pair_{pair}_male_thumb_relief"),
        24.0,
        34.0,
        20.0,
    )
    .translate(x, y - 3.0, NEST_Z / 2.0 - 7.0);
    cradle + stem + thumb_relief
}

fn female_connector_pocket(pair: usize, x: f64, y: f64) -> Part {
    let cup = centered_cylinder(
        format!("connector_force_trace_pair_{pair}_female_cup_socket"),
        FEMALE_SOCKET_D / 2.0,
        40.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, y, NEST_Z / 2.0 - 11.0);
    let key = centered_cube(
        format!("connector_force_trace_pair_{pair}_female_keyed_flat_slot"),
        KEY_SLOT_X,
        FEMALE_SOCKET_D + 10.0,
        22.0,
    )
    .translate(x + 24.0, y, NEST_Z / 2.0 - 10.0);
    let latch_relief = centered_cube(
        format!("connector_force_trace_pair_{pair}_female_latch_window_relief"),
        28.0,
        18.0,
        20.0,
    )
    .translate(x, y + 22.0, NEST_Z / 2.0 - 8.0);
    cup + key + latch_relief
}

fn connector_clamp_lands() -> Part {
    let mut lands = Part::empty("connector_force_trace_connector_clamp_lands");
    for pair in 0..CONNECTOR_PAIR_COUNT {
        let (x, y) = connector_pair_center(pair);
        for (side, dx) in [("male", -64.0), ("female", 64.0)] {
            lands = lands
                + centered_cube(
                    format!("connector_force_trace_pair_{pair}_{side}_spring_clamp_land"),
                    26.0,
                    46.0,
                    10.0,
                )
                .translate(x + dx, y, NEST_Z / 2.0 + 5.0);
        }
    }
    lands
}

fn insertion_axis_guides() -> Part {
    let mut guides = Part::empty("connector_force_trace_insertion_axis_guides");
    for pair in 0..CONNECTOR_PAIR_COUNT {
        let (x, y) = connector_pair_center(pair);
        let upper = centered_cube(
            format!("connector_force_trace_pair_{pair}_upper_insertion_axis_rail"),
            INSERTION_AXIS_CLEARANCE + 22.0,
            5.0,
            9.0,
        )
        .translate(x, y + 20.0, NEST_Z / 2.0 + 4.5);
        let lower = centered_cube(
            format!("connector_force_trace_pair_{pair}_lower_insertion_axis_rail"),
            INSERTION_AXIS_CLEARANCE + 22.0,
            5.0,
            9.0,
        )
        .translate(x, y - 20.0, NEST_Z / 2.0 + 4.5);
        guides = guides + upper + lower;
    }
    guides
}

fn nest_family_witness_tabs() -> Part {
    let mut tabs = Part::empty("connector_force_trace_nest_family_witness_tabs");
    for (index, x) in [-176.0, 0.0, 176.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("connector_force_trace_family_{index}_fluid_gas_electrical_witness_tab"),
                82.0,
                12.0,
                6.0,
            )
            .translate(*x, -(NEST_Y / 2.0 - 18.0), NEST_Z / 2.0 + 3.0);
    }
    tabs
}

fn nest_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("connector_force_trace_nest_robot_fiducials");
    for (index, (x, y)) in [
        (-226.0, 100.0),
        (226.0, 100.0),
        (-226.0, -100.0),
        (226.0, -100.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "connector_force_trace_nest_robot_fiducial_{index}"
            ))
            .translate(*x, *y, NEST_Z / 2.0 + 3.0);
    }
    fiducials
}

fn force_torque_sensor_pocket() -> Part {
    let body = centered_cube(
        "connector_force_trace_force_torque_sensor_pocket_body",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let sensor_recess = centered_cube(
        "connector_force_trace_six_axis_force_torque_sensor_recess",
        SENSOR_RECESS_X,
        SENSOR_RECESS_Y,
        SENSOR_RECESS_Z,
    )
    .translate(-44.0, 0.0, SENSOR_Z / 2.0 - SENSOR_RECESS_Z / 2.0 + 1.0);
    let center_load_bore = centered_cylinder(
        "connector_force_trace_center_load_pin_bore",
        14.0,
        SENSOR_Z + 6.0,
        48,
    )
    .translate(-44.0, 0.0, 0.0);
    let cable_exit = centered_cube(
        "connector_force_trace_force_torque_sensor_cable_exit_notch",
        54.0,
        28.0,
        28.0,
    )
    .translate(-(SENSOR_X / 2.0 - 22.0), 0.0, SENSOR_Z / 2.0 - 12.0);
    let mate_force_anvil = centered_cylinder(
        "connector_force_trace_mate_force_anvil_face",
        22.0,
        18.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SENSOR_X / 2.0 - 48.0, -(SENSOR_Y / 2.0 + 9.0), 8.0);
    let demate_pull_eye =
        centered_cylinder("connector_force_trace_demate_pull_eye", 18.0, 16.0, 40)
            .rotate(90.0, 0.0, 0.0)
            .translate(SENSOR_X / 2.0 - 48.0, SENSOR_Y / 2.0 + 8.0, 8.0);
    let trace_land = centered_cube(
        "connector_force_trace_force_trace_scale_land",
        SENSOR_X - 68.0,
        12.0,
        5.0,
    )
    .translate(18.0, SENSOR_Y / 2.0 - 20.0, SENSOR_Z / 2.0 + 2.5);

    body - sensor_recess - center_load_bore - cable_exit - sensor_bolt_holes()
        + torque_reaction_dogs()
        + mate_force_anvil
        + demate_pull_eye
        + trace_land
        + calibration_mass_pockets()
}

fn sensor_bolt_holes() -> Part {
    let mut holes = Part::empty("connector_force_trace_sensor_bolt_holes");
    for (index, (x, y)) in [(-92.0, -48.0), (4.0, -48.0), (-92.0, 48.0), (4.0, 48.0)]
        .iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("connector_force_trace_sensor_bolt_hole_{index}"),
                3.4,
                SENSOR_Z + 8.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn torque_reaction_dogs() -> Part {
    let mut dogs = Part::empty("connector_force_trace_torque_reaction_dogs");
    for (index, (x, y)) in [(-118.0, 74.0), (30.0, 74.0), (-118.0, -74.0), (30.0, -74.0)]
        .iter()
        .enumerate()
    {
        let block = centered_cube(
            format!("connector_force_trace_torque_reaction_dog_{index}"),
            44.0,
            14.0,
            30.0,
        )
        .translate(*x, *y, SENSOR_Z / 2.0 + 15.0);
        let witness_slot = centered_cube(
            format!("connector_force_trace_torque_reaction_dog_{index}_witness_slot"),
            24.0,
            5.0,
            10.0,
        )
        .translate(*x, *y, SENSOR_Z / 2.0 + 20.0);
        dogs = dogs + (block - witness_slot);
    }
    dogs
}

fn calibration_mass_pockets() -> Part {
    let mut pockets = Part::empty("connector_force_trace_calibration_mass_pockets");
    for index in 0..CALIBRATION_MASS_POCKETS {
        pockets = pockets
            + centered_cylinder(
                format!("connector_force_trace_calibration_mass_pocket_{index}"),
                13.0,
                16.0,
                32,
            )
            .translate(
                98.0 + centered_index(index, CALIBRATION_MASS_POCKETS, 36.0),
                -(SENSOR_Y / 2.0 - 34.0),
                SENSOR_Z / 2.0 - 7.0,
            );
    }
    pockets
}

fn insertion_depth_gauge() -> Part {
    let body = centered_cube(
        "connector_force_trace_insertion_depth_gauge_body",
        DEPTH_X,
        DEPTH_Y,
        DEPTH_Z,
    );
    let probe_channel = centered_cube(
        "connector_force_trace_insertion_depth_linear_probe_channel",
        DEPTH_X - 78.0,
        18.0,
        18.0,
    )
    .translate(0.0, -8.0, DEPTH_Z / 2.0 - 7.0);
    let indicator_window = centered_cube(
        "connector_force_trace_depth_indicator_readout_window",
        84.0,
        38.0,
        14.0,
    )
    .translate(DEPTH_X / 2.0 - 78.0, 18.0, DEPTH_Z / 2.0 - 5.0);

    body - probe_channel - indicator_window - depth_reference_windows()
        + depth_scale_ticks()
        + depth_reference_stops()
        + depth_probe_datum_pins()
}

fn depth_reference_windows() -> Part {
    let mut windows = Part::empty("connector_force_trace_depth_reference_windows");
    for index in 0..DEPTH_WINDOWS {
        windows = windows
            + centered_cube(
                format!("connector_force_trace_depth_reference_window_{index}"),
                26.0,
                18.0,
                14.0,
            )
            .translate(
                centered_index(index, DEPTH_WINDOWS, DEPTH_SCALE_PITCH) - 22.0,
                30.0,
                DEPTH_Z / 2.0 - 5.0,
            );
    }
    windows
}

fn depth_scale_ticks() -> Part {
    let mut ticks = Part::empty("connector_force_trace_depth_scale_ticks");
    for index in 0..=DEPTH_WINDOWS {
        let height = if index % 2 == 0 { 14.0 } else { 9.0 };
        ticks = ticks
            + centered_cube(
                format!("connector_force_trace_depth_scale_tick_{index}"),
                5.0,
                height,
                6.0,
            )
            .translate(
                centered_index(index, DEPTH_WINDOWS + 1, DEPTH_SCALE_PITCH) - 22.0,
                DEPTH_Y / 2.0 - 14.0,
                DEPTH_Z / 2.0 + 3.0,
            );
    }
    ticks
}

fn depth_reference_stops() -> Part {
    let mut stops = Part::empty("connector_force_trace_depth_reference_stops");
    for (index, x) in [-(DEPTH_X / 2.0 - 34.0), DEPTH_X / 2.0 - 34.0]
        .iter()
        .enumerate()
    {
        stops = stops
            + centered_cube(
                format!("connector_force_trace_depth_reference_stop_{index}"),
                20.0,
                DEPTH_Y - 28.0,
                30.0,
            )
            .translate(*x, 0.0, DEPTH_Z / 2.0 + 15.0);
    }
    stops
}

fn depth_probe_datum_pins() -> Part {
    let mut pins = Part::empty("connector_force_trace_depth_probe_datum_pins");
    for index in 0..DEPTH_REFERENCE_STOPS {
        pins = pins
            + centered_cylinder(
                format!("connector_force_trace_depth_probe_datum_pin_{index}"),
                4.0,
                12.0,
                24,
            )
            .translate(
                centered_index(index, DEPTH_REFERENCE_STOPS, DEPTH_X - 96.0),
                -DEPTH_Y / 2.0 + 20.0,
                DEPTH_Z / 2.0 + 6.0,
            );
    }
    pins
}

fn angular_misalignment_wedges() -> Part {
    let panel = centered_cube(
        "connector_force_trace_angular_misalignment_wedge_panel",
        WEDGE_X,
        WEDGE_Y,
        WEDGE_Z,
    );
    panel - wedge_datum_sockets() + wedge_steps() + wedge_hold_down_lips()
}

fn wedge_datum_sockets() -> Part {
    let mut sockets = Part::empty("connector_force_trace_wedge_datum_sockets");
    for index in 0..WEDGE_COUNT {
        sockets = sockets
            + centered_cube(
                format!("connector_force_trace_wedge_{index}_datum_socket"),
                42.0,
                64.0,
                10.0,
            )
            .translate(
                centered_index(index, WEDGE_COUNT, 56.0),
                0.0,
                WEDGE_Z / 2.0 - 5.0,
            );
    }
    sockets
}

fn wedge_steps() -> Part {
    let mut wedges = Part::empty("connector_force_trace_angular_misalignment_wedge_steps");
    for index in 0..WEDGE_COUNT {
        let center_x = centered_index(index, WEDGE_COUNT, 56.0);
        let reverse = index < WEDGE_COUNT / 2;
        let name = match index {
            0 => "minus_4_deg",
            1 => "minus_2_deg",
            2 => "zero_deg",
            3 => "plus_2_deg",
            _ => "plus_4_deg",
        };
        for step in 0..WEDGE_STEPS {
            let step_order = if reverse {
                WEDGE_STEPS - 1 - step
            } else {
                step
            };
            let height = 5.0 + step_order as f64 * 3.2;
            wedges = wedges
                + centered_cube(
                    format!("connector_force_trace_{name}_wedge_step_{step}"),
                    44.0,
                    11.0,
                    height,
                )
                .translate(
                    center_x,
                    centered_index(step, WEDGE_STEPS, 12.0),
                    WEDGE_Z / 2.0 + height / 2.0,
                );
        }
        wedges = wedges
            + centered_cube(
                format!("connector_force_trace_{name}_angle_witness_tab"),
                48.0,
                10.0,
                6.0,
            )
            .translate(center_x, -(WEDGE_Y / 2.0 - 12.0), WEDGE_Z / 2.0 + 3.0);
    }
    wedges
}

fn wedge_hold_down_lips() -> Part {
    let front = centered_cube(
        "connector_force_trace_wedge_front_hold_down_lip",
        WEDGE_X - 32.0,
        8.0,
        18.0,
    )
    .translate(0.0, -WEDGE_Y / 2.0 + 8.0, WEDGE_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "connector_force_trace_wedge_rear_hold_down_lip",
        WEDGE_X - 32.0,
        8.0,
        18.0,
    )
    .translate(0.0, WEDGE_Y / 2.0 - 8.0, WEDGE_Z / 2.0 + 9.0);
    front + rear
}

fn cycle_count_token_rail() -> Part {
    let body = centered_cube(
        "connector_force_trace_cycle_count_token_rail_body",
        CYCLE_X,
        CYCLE_Y,
        CYCLE_Z,
    );
    let spent_chute = centered_cube(
        "connector_force_trace_spent_cycle_token_chute",
        CYCLE_X - 68.0,
        16.0,
        12.0,
    )
    .translate(0.0, -(CYCLE_Y / 2.0 - 24.0), CYCLE_Z / 2.0 - 5.0);
    body - token_pockets() - spent_chute + counter_windows() + token_click_rails()
}

fn token_pockets() -> Part {
    let mut pockets = Part::empty("connector_force_trace_cycle_token_pockets");
    for index in 0..CYCLE_TOKEN_COUNT {
        let col = index % 16;
        let row = index / 16;
        pockets = pockets
            + centered_cylinder(
                format!("connector_force_trace_cycle_token_pocket_{index}"),
                CYCLE_TOKEN_D / 2.0,
                17.0,
                28,
            )
            .translate(
                centered_index(col, 16, 28.0),
                26.0 + centered_index(row, TOKEN_RAILS, 30.0),
                CYCLE_Z / 2.0 - 7.0,
            );
    }
    pockets
}

fn counter_windows() -> Part {
    let mut windows = Part::empty("connector_force_trace_cycle_counter_window_lands");
    for index in 0..CYCLE_COUNTER_WINDOWS {
        windows = windows
            + centered_cube(
                format!("connector_force_trace_cycle_counter_window_land_{index}"),
                42.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(index, CYCLE_COUNTER_WINDOWS, 54.0),
                -(CYCLE_Y / 2.0 - 52.0),
                CYCLE_Z / 2.0 + 2.5,
            );
    }
    windows
}

fn token_click_rails() -> Part {
    let mut rails = Part::empty("connector_force_trace_token_click_rails");
    for index in 0..TOKEN_RAILS {
        rails = rails
            + centered_cube(
                format!("connector_force_trace_token_click_rail_{index}"),
                CYCLE_X - 70.0,
                5.0,
                7.0,
            )
            .translate(
                0.0,
                centered_index(index, TOKEN_RAILS, 32.0) + 10.0,
                CYCLE_Z / 2.0 + 3.5,
            );
    }
    rails
}

fn leak_continuity_witness_ports() -> Part {
    let body = centered_cube(
        "connector_force_trace_leak_continuity_witness_port_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let drip_lip = centered_cube(
        "connector_force_trace_leak_continuity_front_drip_lip",
        WITNESS_X - 34.0,
        9.0,
        14.0,
    )
    .translate(0.0, -(WITNESS_Y / 2.0 - 8.0), WITNESS_Z / 2.0 + 7.0);
    body - leak_port_bores() - continuity_pad_reliefs() - witness_well_cuts()
        + witness_tube_guides()
        + continuity_reference_bridge()
        + drip_lip
}

fn leak_port_bores() -> Part {
    let mut bores = Part::empty("connector_force_trace_leak_port_bores");
    for index in 0..LEAK_PORTS {
        bores = bores
            + centered_cylinder(
                format!("connector_force_trace_leak_port_bore_{index}"),
                LEAK_PORT_D / 2.0,
                WITNESS_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, LEAK_PORTS, 28.0),
                0.0,
                WITNESS_Z / 2.0 - 11.0,
            );
    }
    bores
}

fn continuity_pad_reliefs() -> Part {
    let mut reliefs = Part::empty("connector_force_trace_continuity_pad_reliefs");
    for index in 0..CONTINUITY_PAD_PAIRS {
        let x = centered_index(index, CONTINUITY_PAD_PAIRS, 28.0);
        for (side, y) in [("positive", 16.0), ("negative", 34.0)] {
            reliefs = reliefs
                + centered_cylinder(
                    format!("connector_force_trace_continuity_{side}_pad_relief_{index}"),
                    CONTINUITY_PAD_D / 2.0,
                    9.0,
                    20,
                )
                .translate(x, y, WITNESS_Z / 2.0 - 3.0);
        }
    }
    reliefs
}

fn witness_well_cuts() -> Part {
    let mut wells = Part::empty("connector_force_trace_leak_continuity_witness_well_cuts");
    for index in 0..WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("connector_force_trace_witness_well_{index}"),
                WITNESS_WELL_D / 2.0,
                18.0,
                28,
            )
            .translate(
                centered_index(index, WITNESS_WELLS, 28.0),
                -(WITNESS_Y / 2.0 - 34.0),
                WITNESS_Z / 2.0 - 7.0,
            );
    }
    wells
}

fn witness_tube_guides() -> Part {
    let mut guides = Part::empty("connector_force_trace_witness_tube_guides");
    for index in 0..LEAK_PORTS {
        guides = guides
            + centered_cylinder(
                format!("connector_force_trace_leak_tube_guide_{index}"),
                5.2,
                20.0,
                24,
            )
            .translate(
                centered_index(index, LEAK_PORTS, 28.0),
                WITNESS_Y / 2.0 + 9.0,
                WITNESS_Z / 2.0 - 10.0,
            );
    }
    guides
}

fn continuity_reference_bridge() -> Part {
    centered_cube(
        "connector_force_trace_continuity_reference_bridge_land",
        WITNESS_X - 52.0,
        8.0,
        8.0,
    )
    .translate(0.0, 26.0, WITNESS_Z / 2.0 + 4.0)
}

fn cap_plug_custody_parks() -> Part {
    let body = centered_cube(
        "connector_force_trace_cap_plug_custody_body",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let divider = centered_cube(
        "connector_force_trace_cap_plug_one_way_custody_divider",
        CUSTODY_X - 28.0,
        7.0,
        20.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0 + 10.0);
    let cap_label = centered_cube(
        "connector_force_trace_clean_cap_custody_label_land",
        CUSTODY_X - 32.0,
        12.0,
        5.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 - 13.0, CUSTODY_Z / 2.0 + 2.5);
    let plug_label = centered_cube(
        "connector_force_trace_used_plug_custody_label_land",
        CUSTODY_X - 32.0,
        12.0,
        5.0,
    )
    .translate(0.0, -(CUSTODY_Y / 2.0 - 13.0), CUSTODY_Z / 2.0 + 2.5);

    body - cap_well_cuts() - plug_well_cuts()
        + divider
        + cap_label
        + plug_label
        + custody_tag_tabs()
}

fn cap_well_cuts() -> Part {
    let mut wells = Part::empty("connector_force_trace_cap_well_cuts");
    for index in 0..CAP_WELLS {
        let col = index % 6;
        let row = index / 6;
        wells = wells
            + centered_cylinder(
                format!("connector_force_trace_clean_cap_well_{index}"),
                CAP_WELL_D / 2.0,
                16.0,
                24,
            )
            .translate(
                centered_index(col, 6, 26.0),
                28.0 + centered_index(row, 2, 24.0),
                CUSTODY_Z / 2.0 - 7.0,
            );
    }
    wells
}

fn plug_well_cuts() -> Part {
    let mut wells = Part::empty("connector_force_trace_plug_well_cuts");
    for index in 0..PLUG_WELLS {
        let col = index % 6;
        let row = index / 6;
        wells = wells
            + centered_cylinder(
                format!("connector_force_trace_used_plug_well_{index}"),
                PLUG_WELL_D / 2.0,
                16.0,
                24,
            )
            .translate(
                centered_index(col, 6, 26.0),
                -30.0 + centered_index(row, 2, 22.0),
                CUSTODY_Z / 2.0 - 7.0,
            );
    }
    wells
}

fn custody_tag_tabs() -> Part {
    let mut tabs = Part::empty("connector_force_trace_cap_plug_custody_tag_tabs");
    for index in 0..CUSTODY_TAG_LANDS {
        tabs = tabs
            + centered_cube(
                format!("connector_force_trace_cap_plug_custody_tag_land_{index}"),
                20.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(index, CUSTODY_TAG_LANDS, 26.0),
                0.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn release_hold_reject_lanes() -> Part {
    let backer = centered_cube(
        "connector_force_trace_release_hold_reject_lane_backer",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    backer - disposition_lane_slots() + disposition_lane_rails() + disposition_gate_tabs()
}

fn disposition_lane_slots() -> Part {
    let mut slots = Part::empty("connector_force_trace_release_hold_reject_lane_slots");
    for lane in 0..STATUS_LANES {
        let lane_name = disposition_name(lane);
        let y = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y);
        for slot in 0..STATUS_SLOT_COUNT {
            slots = slots
                + centered_cube(
                    format!("connector_force_trace_{lane_name}_custody_slot_{slot}"),
                    24.0,
                    20.0,
                    14.0,
                )
                .translate(
                    centered_index(slot, STATUS_SLOT_COUNT, 28.0),
                    y,
                    STATUS_Z / 2.0 - 5.0,
                );
        }
    }
    slots
}

fn disposition_lane_rails() -> Part {
    let mut rails = Part::empty("connector_force_trace_release_hold_reject_lane_rails");
    for lane in 0..STATUS_LANES {
        let lane_name = disposition_name(lane);
        let y = centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("connector_force_trace_{lane_name}_raised_header_rail"),
                STATUS_X - 16.0,
                7.0,
                10.0,
            )
            .translate(0.0, y + STATUS_LANE_Y / 2.0 - 8.0, STATUS_Z / 2.0 + 5.0);
    }
    rails
}

fn disposition_gate_tabs() -> Part {
    let mut tabs = Part::empty("connector_force_trace_disposition_gate_tabs");
    for lane in 0..STATUS_LANES {
        tabs = tabs
            + centered_cube(
                format!("connector_force_trace_{}_gate_tab", disposition_name(lane)),
                12.0,
                STATUS_LANE_Y - 12.0,
                18.0,
            )
            .translate(
                -(STATUS_X / 2.0 - 13.0),
                centered_index(lane, STATUS_LANES, STATUS_LANE_PITCH_Y),
                STATUS_Z / 2.0 + 9.0,
            );
    }
    tabs
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "connector_force_trace_barcode_custody_panel",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    panel
        + barcode_scan_lands()
        + lot_scan_lands()
        + rfid_custody_lands()
        + scan_tunnel_rails()
        + custody_chain_arrow_tabs()
}

fn barcode_scan_lands() -> Part {
    let mut lands = Part::empty("connector_force_trace_barcode_scan_lands");
    for index in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("connector_force_trace_connector_barcode_land_{index}"),
                44.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(index, BARCODE_LANDS, 56.0) - 80.0,
                15.0,
                BARCODE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn lot_scan_lands() -> Part {
    let mut lands = Part::empty("connector_force_trace_lot_scan_lands");
    for index in 0..LOT_LANDS {
        lands = lands
            + centered_cube(
                format!("connector_force_trace_connector_lot_land_{index}"),
                44.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(index, LOT_LANDS, 56.0) - 80.0,
                -15.0,
                BARCODE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn rfid_custody_lands() -> Part {
    let mut lands = Part::empty("connector_force_trace_rfid_custody_lands");
    for index in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("connector_force_trace_rfid_custody_land_{index}"),
                54.0,
                38.0,
                5.0,
            )
            .translate(
                BARCODE_X / 2.0 - 144.0 + index as f64 * 62.0,
                0.0,
                BARCODE_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn scan_tunnel_rails() -> Part {
    let front = centered_cube(
        "connector_force_trace_barcode_scan_tunnel_front_rail",
        BARCODE_X - 26.0,
        5.0,
        10.0,
    )
    .translate(0.0, -BARCODE_Y / 2.0 + 7.0, BARCODE_Z / 2.0 + 5.0);
    let rear = centered_cube(
        "connector_force_trace_barcode_scan_tunnel_rear_rail",
        BARCODE_X - 26.0,
        5.0,
        10.0,
    )
    .translate(0.0, BARCODE_Y / 2.0 - 7.0, BARCODE_Z / 2.0 + 5.0);
    front + rear
}

fn custody_chain_arrow_tabs() -> Part {
    let mut tabs = Part::empty("connector_force_trace_custody_chain_arrow_tabs");
    for index in 0..5 {
        tabs = tabs
            + centered_cube(
                format!("connector_force_trace_custody_chain_arrow_tab_{index}"),
                18.0,
                8.0,
                5.0,
            )
            .translate(
                -BARCODE_X / 2.0 + 42.0 + index as f64 * 34.0,
                0.0,
                BARCODE_Z / 2.0 + 2.5,
            );
    }
    tabs
}

fn evidence_camera_bridge() -> Part {
    let mut bridge = Part::empty("connector_force_trace_evidence_camera_bridge");
    for (index, (x, y)) in [
        (-BRIDGE_POST_SPAN_X / 2.0, -BRIDGE_POST_SPAN_Y / 2.0),
        (BRIDGE_POST_SPAN_X / 2.0, -BRIDGE_POST_SPAN_Y / 2.0),
        (-BRIDGE_POST_SPAN_X / 2.0, BRIDGE_POST_SPAN_Y / 2.0),
        (BRIDGE_POST_SPAN_X / 2.0, BRIDGE_POST_SPAN_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("connector_force_trace_camera_bridge_post_{index}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_POST_Z,
            )
            .translate(*x, *y, 0.0);
    }

    let front_beam = centered_cube(
        "connector_force_trace_camera_bridge_front_beam",
        BRIDGE_POST_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_POST_SPAN_Y / 2.0,
        BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        "connector_force_trace_camera_bridge_rear_beam",
        BRIDGE_POST_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_POST_SPAN_Y / 2.0,
        BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0,
    );
    let center_beam = centered_cube(
        "connector_force_trace_camera_bridge_center_crossbeam",
        BRIDGE_POST_SPAN_X - 120.0,
        24.0,
        18.0,
    )
    .translate(0.0, 0.0, CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0);

    bridge + front_beam + rear_beam + center_beam + camera_pods() + bridge_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("connector_force_trace_evidence_camera_pods");
    for index in 0..CAMERA_PODS {
        let x = centered_index(index, CAMERA_PODS, 260.0);
        let pod = centered_cube(
            format!("connector_force_trace_evidence_camera_pod_{index}"),
            94.0,
            34.0,
            18.0,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0 - 18.0);
        let lens_cut = centered_cylinder(
            format!("connector_force_trace_evidence_camera_lens_cut_{index}"),
            10.0,
            24.0,
            32,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0 - 18.0);
        pods = pods + (pod - lens_cut);
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty("connector_force_trace_evidence_light_bars");
    for index in 0..LIGHT_BARS {
        let y = centered_index(index, LIGHT_BARS, 130.0);
        bars = bars
            + centered_cube(
                format!("connector_force_trace_evidence_light_bar_{index}"),
                BRIDGE_POST_SPAN_X - 220.0,
                12.0,
                10.0,
            )
            .translate(0.0, y, CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0 - 38.0);
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let mut gauges = Part::empty("connector_force_trace_robot_service_keepout_gauges");
    for (index, (name, x, y, w, h)) in [
        (
            "front_robot_gripper_sweep",
            0.0,
            ROBOT_SWEEP_Y,
            STATION_X - 160.0,
            10.0,
        ),
        (
            "rear_service_cable_pressure_sweep",
            0.0,
            SERVICE_SWEEP_Y,
            STATION_X - 160.0,
            10.0,
        ),
        (
            "left_robot_entry_keepout",
            -552.0,
            0.0,
            10.0,
            STATION_Y - 168.0,
        ),
        ("right_service_keepout", 552.0, 0.0, 10.0, STATION_Y - 168.0),
        (
            "camera_bridge_lift_keepout",
            0.0,
            0.0,
            BRIDGE_POST_SPAN_X - 80.0,
            10.0,
        ),
        (
            "connector_insertion_axis_keepout",
            NEST_POS.0,
            NEST_POS.1,
            NEST_X - 80.0,
            10.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let rail = centered_cube(
            format!("connector_force_trace_keepout_{index}_{name}_rail"),
            *w,
            *h,
            KEEP_OUT_RAIL_Z,
        )
        .translate(*x, *y, 0.0);
        let flag = centered_cube(
            format!("connector_force_trace_keepout_{index}_{name}_height_flag"),
            28.0,
            12.0,
            SERVICE_FLAG_Z,
        )
        .translate(*x + *w / 2.0 - 28.0, *y, SERVICE_FLAG_Z / 2.0);
        gauges = gauges + rail + flag;
    }
    gauges
}

fn disposition_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn connector_pair_center(pair: usize) -> (f64, f64) {
    let row = pair / CONNECTOR_PAIR_COLS;
    let col = pair % CONNECTOR_PAIR_COLS;
    (
        centered_index(col, CONNECTOR_PAIR_COLS, PAIR_PITCH_X),
        centered_index(row, CONNECTOR_PAIR_ROWS, PAIR_PITCH_Y),
    )
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center_bore"), 1.2, 3.0, 18);
    disc - center
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn insert_rects() -> [(&'static str, Rect); 9] {
    [
        (
            "male_female_connector_nests",
            Rect {
                x: NEST_POS.0,
                y: NEST_POS.1,
                w: NEST_X,
                h: NEST_Y,
            },
        ),
        (
            "force_torque_sensor_pocket",
            Rect {
                x: SENSOR_POS.0,
                y: SENSOR_POS.1,
                w: SENSOR_X,
                h: SENSOR_Y,
            },
        ),
        (
            "insertion_depth_gauge",
            Rect {
                x: DEPTH_POS.0,
                y: DEPTH_POS.1,
                w: DEPTH_X,
                h: DEPTH_Y,
            },
        ),
        (
            "angular_misalignment_wedges",
            Rect {
                x: WEDGE_POS.0,
                y: WEDGE_POS.1,
                w: WEDGE_X,
                h: WEDGE_Y,
            },
        ),
        (
            "cycle_count_token_rail",
            Rect {
                x: CYCLE_POS.0,
                y: CYCLE_POS.1,
                w: CYCLE_X,
                h: CYCLE_Y,
            },
        ),
        (
            "leak_continuity_witness_ports",
            Rect {
                x: WITNESS_POS.0,
                y: WITNESS_POS.1,
                w: WITNESS_X,
                h: WITNESS_Y,
            },
        ),
        (
            "cap_plug_custody_parks",
            Rect {
                x: CUSTODY_POS.0,
                y: CUSTODY_POS.1,
                w: CUSTODY_X,
                h: CUSTODY_Y,
            },
        ),
        (
            "release_hold_reject_lanes",
            Rect {
                x: STATUS_POS.0,
                y: STATUS_POS.1,
                w: STATUS_X,
                h: STATUS_Y,
            },
        ),
        (
            "barcode_custody_lands",
            Rect {
                x: BARCODE_POS.0,
                y: BARCODE_POS.1,
                w: BARCODE_X,
                h: BARCODE_Y,
            },
        ),
    ]
}

fn fits_on_station(rect: Rect) -> bool {
    rect.x.abs() + rect.w / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && rect.y.abs() + rect.h / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let a_left = a.x - a.w / 2.0;
    let a_right = a.x + a.w / 2.0;
    let a_bottom = a.y - a.h / 2.0;
    let a_top = a.y + a.h / 2.0;
    let b_left = b.x - b.w / 2.0;
    let b_right = b.x + b.w / 2.0;
    let b_bottom = b.y - b.h / 2.0;
    let b_top = b.y + b.h / 2.0;

    !(a_right <= b_left || b_right <= a_left || a_top <= b_bottom || b_top <= a_bottom)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_stable() {
        assert_design_constraints();

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_robotic_connector_mate_demate_force_trace_station_"
                ),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert_eq!(
            OUTPUTS[0],
            "output/closed_robotic_connector_mate_demate_force_trace_station_base_metrology_deck.stl"
        );
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_mechanical_cues_are_explicit_features() {
        for feature in [
            "male_female_connector_nests",
            "force_torque_sensor_pocket",
            "insertion_depth_gauge",
            "angular_misalignment_wedges",
            "cycle_count_token_rail",
            "leak_continuity_witness_ports",
            "cap_plug_custody_parks",
            "release_hold_reject_lanes",
            "barcode_custody_lands",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn connector_nests_cover_male_female_closed_service_pairs() {
        assert_eq!(CONNECTOR_PAIR_ROWS, 2);
        assert_eq!(CONNECTOR_PAIR_COLS, 3);
        assert_eq!(CONNECTOR_PAIR_COUNT, 6);
        assert_eq!(CONNECTOR_COUNT, 12);
        assert!(FEMALE_SOCKET_D > MALE_SOCKET_D);
        let first = connector_pair_center(0);
        let last = connector_pair_center(CONNECTOR_PAIR_COUNT - 1);
        assert!((first.0 + last.0).abs() < 0.001);
        assert!((first.1 + last.1).abs() < 0.001);
        assert!(last.0.abs() + INSERTION_AXIS_CLEARANCE / 2.0 < NEST_X / 2.0 - 20.0);
    }

    #[test]
    fn force_depth_and_misalignment_trace_controls_are_sized() {
        assert_eq!(SENSOR_BOLT_COUNT, 4);
        assert_eq!(TORQUE_REACTION_DOGS, 4);
        assert_eq!(CALIBRATION_MASS_POCKETS, 4);
        assert!(SENSOR_RECESS_X < SENSOR_X);
        assert!(SENSOR_RECESS_Y < SENSOR_Y);
        assert_eq!(DEPTH_WINDOWS, 8);
        assert_eq!(WEDGE_COUNT, 5);
        assert_eq!(WEDGE_STEPS, 5);
        assert!(MAX_MISALIGN_DEG >= 4.0);
    }

    #[test]
    fn custody_witness_and_disposition_capacity_cover_each_connector() {
        assert!(CYCLE_TOKEN_COUNT >= CONNECTOR_COUNT * 2);
        assert!(CAP_WELLS >= CONNECTOR_COUNT);
        assert!(PLUG_WELLS >= CONNECTOR_COUNT);
        assert_eq!(LEAK_PORTS, CONNECTOR_COUNT);
        assert_eq!(CONTINUITY_PAD_PAIRS, CONNECTOR_COUNT);
        assert_eq!(WITNESS_WELLS, CONNECTOR_COUNT);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOT_COUNT * STATUS_LANES, CONNECTOR_COUNT);
    }

    #[test]
    fn station_modules_fit_and_remain_separated() {
        let rects = insert_rects();
        for (_name, rect) in rects {
            assert!(fits_on_station(rect));
        }
        for (index, (_left_name, left)) in rects.iter().enumerate() {
            for (_right_name, right) in rects.iter().skip(index + 1) {
                assert!(!rects_overlap(*left, *right));
            }
        }
        assert!(STATION_X <= 1200.0);
        assert!(STATION_Y <= 880.0);
        assert!(RIM_Z >= 44.0);
    }

    #[test]
    fn evidence_and_keepout_assumptions_are_visible() {
        assert_eq!(CAMERA_PODS, 3);
        assert_eq!(LIGHT_BARS, 2);
        assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
        assert_eq!(KEEP_OUT_FLAGS, 6);
        assert!(ROBOT_SWEEP_Y.abs() < STATION_Y / 2.0);
        assert!(SERVICE_SWEEP_Y.abs() < STATION_Y / 2.0);
        assert!(BRIDGE_POST_SPAN_X < STATION_X - 2.0 * RIM_W);
        assert!(BRIDGE_POST_SPAN_Y < STATION_Y - 2.0 * RIM_W);
    }
}
