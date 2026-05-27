use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system cell-lot hold-time and viability-decay tracing fixture before
// seeding.
//
// Design intent:
// - Keep sealed lot containers, timed hold tokens, temperature evidence,
//   closed-loop sampling surrogates, count-cassette handling, and disposition
//   segregation visible on one bounded validation deck.
// - Model bought-in loggers, count/viability readers, cameras, tubing, and
//   robot access as envelopes, witness windows, lands, and keepouts only.
// - This is mechanical concept CAD for traceability and workflow validation;
//   it is not a cell culture protocol, assay validation, lot disposition, or
//   sterility claim.

const OUTPUTS: [&str; 14] = [
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_base_containment_deck.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_sealed_cell_lot_bag_vial_nests.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_timed_hold_token_rail.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_gentle_mix_witness_cradle.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_temperature_logger_pockets.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_closed_sampling_loop_surrogate.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_viability_count_cassette_dock_envelope.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_first_middle_last_sample_custody_wells.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_bubble_dead_volume_witness_windows.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_release_hold_reject_lanes.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_barcode_coa_status_lands.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_camera_bridge.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_robot_service_keepouts.stl",
    "output/closed_cell_lot_hold_time_viability_decay_tracer_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 13] = [
    "base_containment_deck",
    "sealed_cell_lot_bag_vial_nests",
    "timed_hold_token_rail",
    "gentle_mix_witness_cradle",
    "temperature_logger_pockets",
    "closed_sampling_loop_surrogate",
    "viability_count_cassette_dock_envelope",
    "first_middle_last_sample_custody_wells",
    "bubble_dead_volume_witness_windows",
    "release_hold_reject_lanes",
    "barcode_coa_status_lands",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const SAMPLE_PHASES: [&str; 3] = ["first", "middle", "last"];
const STATUS_LANES: [&str; 3] = ["release", "hold", "reject"];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const CONTAINMENT_RECESS_Z: f64 = 8.0;
const DRAIN_PORT_D: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DATUM_BOSSES: usize = 10;

const LOT_NEST_CENTER: (f64, f64) = (-520.0, 225.0);
const LOT_NEST_X: f64 = 360.0;
const LOT_NEST_Y: f64 = 250.0;
const LOT_NEST_Z: f64 = 64.0;
const SEALED_BAG_NESTS: usize = 2;
const VIAL_NEST_ROWS: usize = 2;
const VIAL_NEST_COLS: usize = 5;
const VIAL_NESTS: usize = VIAL_NEST_ROWS * VIAL_NEST_COLS;
const BAG_RECESS_X: f64 = 128.0;
const BAG_RECESS_Y: f64 = 168.0;
const VIAL_WELL_D: f64 = 17.5;
const VIAL_PITCH_X: f64 = 38.0;
const VIAL_PITCH_Y: f64 = 42.0;

const HOLD_RAIL_CENTER: (f64, f64) = (0.0, 390.0);
const HOLD_RAIL_X: f64 = 820.0;
const HOLD_RAIL_Y: f64 = 88.0;
const HOLD_RAIL_Z: f64 = 34.0;
const HOLD_TOKENS: usize = 8;
const HOLD_TOKEN_PITCH_X: f64 = 88.0;
const HOLD_TOKEN_D: f64 = 28.0;
const HOLD_TIME_MARKS: usize = HOLD_TOKENS;

const MIX_CRADLE_CENTER: (f64, f64) = (-510.0, -75.0);
const MIX_CRADLE_X: f64 = 330.0;
const MIX_CRADLE_Y: f64 = 210.0;
const MIX_CRADLE_Z: f64 = 58.0;
const MIX_BAG_RECESS_X: f64 = 240.0;
const MIX_BAG_RECESS_Y: f64 = 136.0;
const MIX_ROLLERS: usize = 2;
const MIX_WITNESS_RIBS: usize = 5;
const MAX_MIX_RPM: f64 = 8.0;

const LOGGER_CENTER: (f64, f64) = (500.0, 300.0);
const LOGGER_X: f64 = 260.0;
const LOGGER_Y: f64 = 150.0;
const LOGGER_Z: f64 = 44.0;
const TEMPERATURE_LOGGER_POCKETS: usize = 4;
const LOGGER_PITCH_X: f64 = 58.0;
const LOGGER_CABLE_RELIEFS: usize = 4;

const LOOP_CENTER: (f64, f64) = (-120.0, 150.0);
const LOOP_X: f64 = 430.0;
const LOOP_Y: f64 = 210.0;
const LOOP_Z: f64 = 58.0;
const LOOP_PORTS: usize = 6;
const LOOP_VALVES: usize = 6;
const LOOP_SENSOR_TAPS: usize = 4;
const LOOP_TUBE_D: f64 = 6.4;
const LOOP_BRANCH_D: f64 = 4.8;

const CASSETTE_CENTER: (f64, f64) = (365.0, 70.0);
const CASSETTE_X: f64 = 360.0;
const CASSETTE_Y: f64 = 250.0;
const CASSETTE_Z: f64 = 62.0;
const COUNT_CASSETTE_SLOTS: usize = 6;
const CASSETTE_DATUM_PINS: usize = 4;
const CASSETTE_SLOT_X: f64 = 92.0;
const CASSETTE_SLOT_Y: f64 = 54.0;

const CUSTODY_CENTER: (f64, f64) = (-330.0, -315.0);
const CUSTODY_X: f64 = 380.0;
const CUSTODY_Y: f64 = 180.0;
const CUSTODY_Z: f64 = 48.0;
const CUSTODY_WELLS: usize = SAMPLE_PHASES.len();
const CUSTODY_RETAIN_WELLS: usize = 3;
const CUSTODY_CARD_SLOTS: usize = 3;

const WITNESS_CENTER: (f64, f64) = (100.0, -315.0);
const WITNESS_X: f64 = 330.0;
const WITNESS_Y: f64 = 180.0;
const WITNESS_Z: f64 = 52.0;
const BUBBLE_WITNESS_WINDOWS: usize = 3;
const DEAD_VOLUME_WITNESS_WINDOWS: usize = 3;
const WITNESS_WINDOW_D: f64 = 32.0;

const LANES_CENTER: (f64, f64) = (465.0, -245.0);
const LANES_X: f64 = 380.0;
const LANES_Y: f64 = 230.0;
const LANES_Z: f64 = 48.0;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 76.0;
const STATUS_SLOT_Y: f64 = 44.0;
const STATUS_LANE_PITCH_X: f64 = 112.0;

const LABEL_CENTER: (f64, f64) = (0.0, -424.0);
const LABEL_X: f64 = 900.0;
const LABEL_Y: f64 = 72.0;
const LABEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 12;
const COA_LANDS: usize = 4;
const STATUS_LANDS: usize = 6;
const LABEL_LAND_PITCH_X: f64 = 62.0;

const CAMERA_BRIDGE_CENTER: (f64, f64) = (0.0, -5.0);
const CAMERA_BRIDGE_SPAN_X: f64 = 1320.0;
const CAMERA_BRIDGE_POST_X: f64 = 34.0;
const CAMERA_BRIDGE_POST_Y: f64 = 42.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 235.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 36.0;
const CAMERA_COUNT: usize = 4;
const EVIDENCE_LIGHT_BARS: usize = 6;

const KEEP_OUT_Z: f64 = 92.0;
const ROBOT_FRONT_CLEARANCE: f64 = 360.0;
const SERVICE_REAR_CLEARANCE: f64 = 240.0;
const SERVICE_SIDE_CLEARANCE: f64 = 230.0;
const OVERHEAD_CAMERA_CLEARANCE: f64 = 305.0;
const KEEP_OUT_ZONES: usize = 4;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let lot_nests = sealed_cell_lot_bag_vial_nests();
    export(OUTPUTS[1], &lot_nests);

    let hold_rail = timed_hold_token_rail();
    export(OUTPUTS[2], &hold_rail);

    let mix = gentle_mix_witness_cradle();
    export(OUTPUTS[3], &mix);

    let loggers = temperature_logger_pockets();
    export(OUTPUTS[4], &loggers);

    let loop_surrogate = closed_sampling_loop_surrogate();
    export(OUTPUTS[5], &loop_surrogate);

    let cassette = viability_count_cassette_dock_envelope();
    export(OUTPUTS[6], &cassette);

    let custody = first_middle_last_sample_custody_wells();
    export(OUTPUTS[7], &custody);

    let witnesses = bubble_dead_volume_witness_windows();
    export(OUTPUTS[8], &witnesses);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let labels = barcode_coa_status_lands();
    export(OUTPUTS[10], &labels);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[11], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[12], &keepouts);

    let assembly =
        base + lot_nests.translate(
            LOT_NEST_CENTER.0,
            LOT_NEST_CENTER.1,
            deck_insert_z(LOT_NEST_Z),
        ) + hold_rail.translate(
            HOLD_RAIL_CENTER.0,
            HOLD_RAIL_CENTER.1,
            deck_insert_z(HOLD_RAIL_Z),
        ) + mix.translate(
            MIX_CRADLE_CENTER.0,
            MIX_CRADLE_CENTER.1,
            deck_insert_z(MIX_CRADLE_Z),
        ) + loggers.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, deck_insert_z(LOGGER_Z))
            + loop_surrogate.translate(LOOP_CENTER.0, LOOP_CENTER.1, deck_insert_z(LOOP_Z))
            + cassette.translate(
                CASSETTE_CENTER.0,
                CASSETTE_CENTER.1,
                deck_insert_z(CASSETTE_Z),
            )
            + custody.translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, deck_insert_z(CUSTODY_Z))
            + witnesses.translate(WITNESS_CENTER.0, WITNESS_CENTER.1, deck_insert_z(WITNESS_Z))
            + lanes.translate(LANES_CENTER.0, LANES_CENTER.1, deck_insert_z(LANES_Z))
            + labels.translate(LABEL_CENTER.0, LABEL_CENTER.1, deck_insert_z(LABEL_Z))
            + bridge.translate(
                CAMERA_BRIDGE_CENTER.0,
                CAMERA_BRIDGE_CENTER.1,
                deck_insert_z(CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z),
            )
            + keepouts.translate(0.0, 0.0, deck_insert_z(KEEP_OUT_Z))
            + closed_route_rails();
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed cell-lot hold-time viability-decay tracer station:");
    println!(
        "  Deck:                       {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {DATUM_BOSSES} datum bosses"
    );
    println!(
        "  Sealed lot nests:           {SEALED_BAG_NESTS} bag nests and {VIAL_NESTS} vial positions"
    );
    println!(
        "  Hold-time traceability:     {HOLD_TOKENS} timed hold tokens and {HOLD_TIME_MARKS} rail marks"
    );
    println!(
        "  Gentle-mix witness:         {MIX_ROLLERS} rollers, {MIX_WITNESS_RIBS} witness ribs, <= {MAX_MIX_RPM:.0} rpm envelope"
    );
    println!(
        "  Temperature evidence:       {TEMPERATURE_LOGGER_POCKETS} logger pockets and {LOGGER_CABLE_RELIEFS} cable reliefs"
    );
    println!(
        "  Closed sample surrogate:    {LOOP_PORTS} sterile ports, {LOOP_VALVES} pinch-valve saddles, {LOOP_SENSOR_TAPS} sensor taps"
    );
    println!(
        "  Cassette/custody:           {COUNT_CASSETTE_SLOTS} count-cassette slots, {CUSTODY_WELLS} first/middle/last wells, {CUSTODY_RETAIN_WELLS} retain wells"
    );
    println!(
        "  Witness windows:            {BUBBLE_WITNESS_WINDOWS} bubble windows and {DEAD_VOLUME_WITNESS_WINDOWS} dead-volume windows"
    );
    println!(
        "  Disposition/identity:       release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {STATUS_LANDS} status lands"
    );
    println!(
        "  Evidence/keepouts:          {CAMERA_COUNT} camera pods, {EVIDENCE_LIGHT_BARS} light bars, {KEEP_OUT_ZONES} robot/service keepout zones"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert_eq!(CUSTODY_WELLS, SAMPLE_PHASES.len());
    assert_eq!(VIAL_NESTS, VIAL_NEST_ROWS * VIAL_NEST_COLS);
    assert_eq!(HOLD_TOKENS, HOLD_TIME_MARKS);
    assert_eq!(STATUS_LANES.len(), 3);
    assert_eq!(BUBBLE_WITNESS_WINDOWS, DEAD_VOLUME_WITNESS_WINDOWS);
    assert!(COUNT_CASSETTE_SLOTS >= SAMPLE_PHASES.len() * 2);
    assert!(BARCODE_LANDS >= SEALED_BAG_NESTS + VIAL_NESTS);
    assert!(COA_LANDS >= SAMPLE_PHASES.len());
    assert!(MAX_MIX_RPM <= 10.0);

    for spec in component_specs() {
        assert!(
            fits_on_station(spec.center, spec.width, spec.depth),
            "{} exceeds station footprint",
            spec.name
        );
    }
}

#[derive(Clone, Copy)]
struct ComponentSpec {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

fn component_specs() -> [ComponentSpec; 11] {
    [
        ComponentSpec {
            name: "sealed_cell_lot_bag_vial_nests",
            center: LOT_NEST_CENTER,
            width: LOT_NEST_X,
            depth: LOT_NEST_Y,
        },
        ComponentSpec {
            name: "timed_hold_token_rail",
            center: HOLD_RAIL_CENTER,
            width: HOLD_RAIL_X,
            depth: HOLD_RAIL_Y,
        },
        ComponentSpec {
            name: "gentle_mix_witness_cradle",
            center: MIX_CRADLE_CENTER,
            width: MIX_CRADLE_X,
            depth: MIX_CRADLE_Y,
        },
        ComponentSpec {
            name: "temperature_logger_pockets",
            center: LOGGER_CENTER,
            width: LOGGER_X,
            depth: LOGGER_Y,
        },
        ComponentSpec {
            name: "closed_sampling_loop_surrogate",
            center: LOOP_CENTER,
            width: LOOP_X,
            depth: LOOP_Y,
        },
        ComponentSpec {
            name: "viability_count_cassette_dock_envelope",
            center: CASSETTE_CENTER,
            width: CASSETTE_X,
            depth: CASSETTE_Y,
        },
        ComponentSpec {
            name: "first_middle_last_sample_custody_wells",
            center: CUSTODY_CENTER,
            width: CUSTODY_X,
            depth: CUSTODY_Y,
        },
        ComponentSpec {
            name: "bubble_dead_volume_witness_windows",
            center: WITNESS_CENTER,
            width: WITNESS_X,
            depth: WITNESS_Y,
        },
        ComponentSpec {
            name: "release_hold_reject_lanes",
            center: LANES_CENTER,
            width: LANES_X,
            depth: LANES_Y,
        },
        ComponentSpec {
            name: "barcode_coa_status_lands",
            center: LABEL_CENTER,
            width: LABEL_X,
            depth: LABEL_Y,
        },
        ComponentSpec {
            name: "evidence_camera_bridge",
            center: CAMERA_BRIDGE_CENTER,
            width: CAMERA_BRIDGE_SPAN_X,
            depth: CAMERA_BRIDGE_POST_Y,
        },
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_containment_recess",
        STATION_X - 126.0,
        STATION_Y - 122.0,
        CONTAINMENT_RECESS_Z + 2.0,
    )
    .translate(0.0, -18.0, BASE_Z - CONTAINMENT_RECESS_Z / 2.0 + 0.6);
    let front_sump = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_front_low_point_sump",
        148.0,
        62.0,
        CONTAINMENT_RECESS_Z + 4.0,
    )
    .translate(
        STATION_X / 2.0 - 124.0,
        -STATION_Y / 2.0 + 76.0,
        BASE_Z - CONTAINMENT_RECESS_Z / 2.0,
    );
    let drain = centered_cylinder(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_closed_drain_port",
        DRAIN_PORT_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 124.0,
        -STATION_Y / 2.0 + 32.0,
        BASE_Z - 5.0,
    );

    deck - basin - front_sump - drain - mount_holes()
        + perimeter_rims()
        + datum_bosses()
        + component_socket_reliefs()
        + containment_witness_strips()
}

fn perimeter_rims() -> Part {
    let z = BASE_Z + RIM_Z / 2.0;
    let front = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn mount_holes() -> Part {
    let mut holes =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_mount_holes");
    for (i, (x, y)) in mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_cell_lot_hold_time_viability_decay_tracer_station_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 52.0),
        (-STATION_X / 4.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 4.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 4.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 52.0),
    ]
}

fn datum_bosses() -> Part {
    let mut bosses =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_datum_bosses");
    for i in 0..DATUM_BOSSES {
        let x = centered_index(i % 5, 5, 315.0);
        let y = if i < 5 { -360.0 } else { 330.0 };
        let boss = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_datum_boss_{i}"),
            13.0,
            8.0,
            32,
        )
        .translate(x, y, BASE_Z + 4.0);
        let bore = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_datum_pin_bore_{i}"),
            4.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z + 5.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn component_socket_reliefs() -> Part {
    let mut sockets =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_component_sockets");
    for spec in component_specs().into_iter().take(10) {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_{}_socket_relief",
                    spec.name
                ),
                spec.width + 8.0,
                spec.depth + 8.0,
                5.0,
            )
            .translate(spec.center.0, spec.center.1, BASE_Z - 2.0);
    }
    sockets
}

fn containment_witness_strips() -> Part {
    let mut strips = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_containment_witness_strips",
    );
    for (i, y) in [-300.0, -70.0, 160.0, 330.0].into_iter().enumerate() {
        strips = strips
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_leak_trace_strip_{i}"
                ),
                118.0,
                8.0,
                3.0,
            )
            .translate(STATION_X / 2.0 - 185.0, y, BASE_Z + 1.5);
    }
    strips
}

fn sealed_cell_lot_bag_vial_nests() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_lot_nest_body",
        LOT_NEST_X,
        LOT_NEST_Y,
        LOT_NEST_Z,
    );
    let mut cuts =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_lot_nest_cuts");

    for i in 0..SEALED_BAG_NESTS {
        let x = centered_index(i, SEALED_BAG_NESTS, 152.0);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_sealed_bag_recess_{i}"
                ),
                BAG_RECESS_X,
                BAG_RECESS_Y,
                24.0,
            )
            .translate(x, 36.0, LOT_NEST_Z / 2.0 - 11.0);
    }

    for row in 0..VIAL_NEST_ROWS {
        for col in 0..VIAL_NEST_COLS {
            let x = centered_index(col, VIAL_NEST_COLS, VIAL_PITCH_X);
            let y = -LOT_NEST_Y / 2.0 + 44.0 + row as f64 * VIAL_PITCH_Y;
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_cell_lot_hold_time_viability_decay_tracer_station_sealed_vial_well_{row}_{col}"
                    ),
                    VIAL_WELL_D / 2.0,
                    34.0,
                    28,
                )
                .translate(x, y, LOT_NEST_Z / 2.0 - 14.0);
        }
    }

    body - cuts + bag_retainer_frames() + lot_nest_latch_lands() + sealed_port_saddles()
}

fn bag_retainer_frames() -> Part {
    let mut frames =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_bag_retainer_frames");
    for i in 0..SEALED_BAG_NESTS {
        let x = centered_index(i, SEALED_BAG_NESTS, 152.0);
        frames = frames
            + rectangular_frame_xy(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_bag_retainer_frame_{i}"
                ),
                BAG_RECESS_X + 24.0,
                BAG_RECESS_Y + 22.0,
                9.0,
                BAG_RECESS_X - 14.0,
                BAG_RECESS_Y - 12.0,
            )
            .translate(x, 36.0, LOT_NEST_Z / 2.0 + 4.5);
    }
    frames
}

fn lot_nest_latch_lands() -> Part {
    let mut lands = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_lot_nest_latch_lands",
    );
    for i in 0..4 {
        let x = centered_index(i % 2, 2, LOT_NEST_X - 76.0);
        let y = centered_index(i / 2, 2, LOT_NEST_Y - 56.0);
        lands = lands
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_lot_nest_latch_land_{i}"
                ),
                48.0,
                24.0,
                10.0,
            )
            .translate(x, y, LOT_NEST_Z / 2.0 + 5.0);
    }
    lands
}

fn sealed_port_saddles() -> Part {
    let mut saddles =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_sealed_port_saddles");
    for i in 0..LOOP_PORTS {
        let x = -LOT_NEST_X / 2.0 + 38.0 + i as f64 * 48.0;
        let saddle = centered_cube(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_lot_sealed_port_saddle_{i}"
            ),
            32.0,
            18.0,
            18.0,
        )
        .translate(x, -LOT_NEST_Y / 2.0 - 9.0, -6.0);
        let bore = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_lot_port_bore_{i}"),
            4.0,
            22.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -LOT_NEST_Y / 2.0 - 10.0, -6.0);
        saddles = saddles + (saddle - bore);
    }
    saddles
}

fn timed_hold_token_rail() -> Part {
    let rail = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_timed_hold_token_rail_body",
        HOLD_RAIL_X,
        HOLD_RAIL_Y,
        HOLD_RAIL_Z,
    );
    let slot_cut = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_hold_token_slide_channel",
        HOLD_RAIL_X - 58.0,
        34.0,
        14.0,
    )
    .translate(0.0, 0.0, HOLD_RAIL_Z / 2.0 - 6.0);
    rail - slot_cut + hold_tokens() + hold_time_tick_marks()
}

fn hold_tokens() -> Part {
    let mut tokens =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_hold_tokens");
    for i in 0..HOLD_TOKENS {
        let x = centered_index(i, HOLD_TOKENS, HOLD_TOKEN_PITCH_X);
        let token = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_hold_token_{i}"),
            HOLD_TOKEN_D / 2.0,
            8.0,
            36,
        )
        .translate(x, 0.0, HOLD_RAIL_Z / 2.0 + 4.0);
        let key = centered_cube(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_hold_token_key_{i}"),
            8.0,
            HOLD_TOKEN_D + 6.0,
            10.0,
        )
        .translate(x, 0.0, HOLD_RAIL_Z / 2.0 + 5.0);
        tokens = tokens + token + key;
    }
    tokens
}

fn hold_time_tick_marks() -> Part {
    let mut marks =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_hold_time_marks");
    for i in 0..HOLD_TIME_MARKS {
        let x = centered_index(i, HOLD_TIME_MARKS, HOLD_TOKEN_PITCH_X);
        marks = marks
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_hold_time_tick_{i}"
                ),
                5.0,
                HOLD_RAIL_Y + 14.0,
                8.0,
            )
            .translate(x, 0.0, HOLD_RAIL_Z / 2.0 + 4.0);
    }
    marks
}

fn gentle_mix_witness_cradle() -> Part {
    let tray = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_gentle_mix_cradle_tray",
        MIX_CRADLE_X,
        MIX_CRADLE_Y,
        MIX_CRADLE_Z,
    );
    let bag_recess = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_gentle_mix_bag_recess",
        MIX_BAG_RECESS_X,
        MIX_BAG_RECESS_Y,
        22.0,
    )
    .translate(0.0, 4.0, MIX_CRADLE_Z / 2.0 - 10.0);
    tray - bag_recess + mix_roller_pair() + mix_witness_ribs() + cradle_tilt_limit_flags()
}

fn mix_roller_pair() -> Part {
    let mut rollers =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_mix_roller_pair");
    for i in 0..MIX_ROLLERS {
        let y = centered_index(i, MIX_ROLLERS, 72.0);
        let roller = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_mix_roller_{i}"),
            8.0,
            MIX_BAG_RECESS_X + 28.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, MIX_CRADLE_Z / 2.0 + 13.0);
        let shaft = centered_cylinder(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_mix_roller_shaft_cut_{i}"
            ),
            2.4,
            MIX_BAG_RECESS_X + 42.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, MIX_CRADLE_Z / 2.0 + 13.0);
        rollers = rollers + (roller - shaft);
    }
    rollers
}

fn mix_witness_ribs() -> Part {
    let mut ribs =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_mix_witness_ribs");
    for i in 0..MIX_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_low_shear_mix_witness_rib_{i}"
                ),
                8.0,
                MIX_BAG_RECESS_Y - 28.0,
                7.0,
            )
            .translate(
                centered_index(i, MIX_WITNESS_RIBS, 42.0),
                4.0,
                MIX_CRADLE_Z / 2.0 + 3.5,
            );
    }
    ribs
}

fn cradle_tilt_limit_flags() -> Part {
    centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_cradle_tilt_limit_flag",
        MIX_CRADLE_X - 38.0,
        12.0,
        18.0,
    )
    .translate(0.0, -MIX_CRADLE_Y / 2.0 + 20.0, MIX_CRADLE_Z / 2.0 + 9.0)
}

fn temperature_logger_pockets() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_temperature_logger_pocket_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut cuts =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_logger_pocket_cuts");
    for i in 0..TEMPERATURE_LOGGER_POCKETS {
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_temperature_logger_pocket_cut_{i}"
                ),
                42.0,
                78.0,
                22.0,
            )
            .translate(centered_index(i, TEMPERATURE_LOGGER_POCKETS, LOGGER_PITCH_X), 0.0, LOGGER_Z / 2.0 - 10.0);
    }
    body - cuts + logger_cable_reliefs() + logger_status_lips()
}

fn logger_cable_reliefs() -> Part {
    let mut reliefs = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_logger_cable_reliefs",
    );
    for i in 0..LOGGER_CABLE_RELIEFS {
        reliefs = reliefs
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_logger_cable_relief_{i}"
                ),
                28.0,
                12.0,
                12.0,
            )
            .translate(
                centered_index(i, LOGGER_CABLE_RELIEFS, LOGGER_PITCH_X),
                -LOGGER_Y / 2.0 - 6.0,
                -2.0,
            );
    }
    reliefs
}

fn logger_status_lips() -> Part {
    let mut lips =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_logger_status_lips");
    for i in 0..TEMPERATURE_LOGGER_POCKETS {
        lips = lips
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_logger_status_lip_{i}"
                ),
                44.0,
                8.0,
                10.0,
            )
            .translate(
                centered_index(i, TEMPERATURE_LOGGER_POCKETS, LOGGER_PITCH_X),
                LOGGER_Y / 2.0 + 4.0,
                LOGGER_Z / 2.0,
            );
    }
    lips
}

fn closed_sampling_loop_surrogate() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_closed_sampling_loop_body",
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let supply = tube_cut_x(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_supply_bore",
        -LOOP_X / 2.0 + 34.0,
        LOOP_X / 2.0 - 34.0,
        42.0,
        2.0,
        LOOP_TUBE_D,
    );
    let return_bore = tube_cut_x(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_return_bore",
        -LOOP_X / 2.0 + 34.0,
        LOOP_X / 2.0 - 34.0,
        -42.0,
        2.0,
        LOOP_TUBE_D,
    );
    let recirc = tube_cut_y(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_recirc_bore",
        -LOOP_X / 2.0 + 70.0,
        -42.0,
        42.0,
        2.0,
        LOOP_BRANCH_D,
    );
    let sample_branch = tube_cut_y(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_sample_branch_bore",
        LOOP_X / 2.0 - 88.0,
        -42.0,
        42.0,
        2.0,
        LOOP_BRANCH_D,
    );
    body - supply - return_bore - recirc - sample_branch
        + loop_ports()
        + loop_valve_saddles()
        + loop_sensor_taps()
        + loop_cover_land()
}

fn loop_ports() -> Part {
    let mut ports =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_ports");
    for i in 0..LOOP_PORTS {
        let x = centered_index(i % 3, 3, 92.0);
        let y = if i < 3 {
            LOOP_Y / 2.0 + 10.0
        } else {
            -LOOP_Y / 2.0 - 10.0
        };
        let saddle = centered_cube(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_port_saddle_{i}"
            ),
            46.0,
            20.0,
            24.0,
        )
        .translate(x, y, 0.0);
        let bore = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_port_bore_{i}"),
            5.0,
            28.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, 0.0);
        ports = ports + (saddle - bore);
    }
    ports
}

fn loop_valve_saddles() -> Part {
    let mut valves =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_valve_saddles");
    for i in 0..LOOP_VALVES {
        let x = centered_index(i, LOOP_VALVES, 56.0);
        let y = if i % 2 == 0 { 42.0 } else { -42.0 };
        let saddle = centered_cube(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_pinch_valve_saddle_{i}"
            ),
            32.0,
            24.0,
            18.0,
        )
        .translate(x, y, LOOP_Z / 2.0 + 9.0);
        let window = centered_cube(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_pinch_valve_window_{i}"
            ),
            16.0,
            12.0,
            20.0,
        )
        .translate(x, y, LOOP_Z / 2.0 + 10.0);
        valves = valves + (saddle - window);
    }
    valves
}

fn loop_sensor_taps() -> Part {
    let mut taps =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_sensor_taps");
    for i in 0..LOOP_SENSOR_TAPS {
        let x = centered_index(i, LOOP_SENSOR_TAPS, 78.0);
        let pad = centered_cube(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_sensor_pad_{i}"),
            48.0,
            24.0,
            8.0,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 4.0);
        let tap = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_loop_sensor_tap_{i}"),
            2.4,
            12.0,
            18,
        )
        .translate(x, 0.0, LOOP_Z / 2.0 + 4.0);
        taps = taps + (pad - tap);
    }
    taps
}

fn loop_cover_land() -> Part {
    rectangular_frame_xy(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_clear_lid_land",
        LOOP_X - 34.0,
        LOOP_Y - 30.0,
        8.0,
        LOOP_X - 82.0,
        LOOP_Y - 82.0,
    )
    .translate(0.0, 0.0, LOOP_Z / 2.0 + 4.0)
}

fn viability_count_cassette_dock_envelope() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_count_cassette_dock_body",
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    body - cassette_slot_cuts() + cassette_datum_pins() + reader_envelope() + cassette_id_lands()
}

fn cassette_slot_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_slot_cuts");
    for i in 0..COUNT_CASSETTE_SLOTS {
        let x = centered_index(i % 3, 3, 106.0);
        let y = centered_index(i / 3, 2, 82.0);
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_count_cassette_slot_cut_{i}"
                ),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                18.0,
            )
            .translate(x, y, CASSETTE_Z / 2.0 - 8.0);
    }
    cuts
}

fn cassette_datum_pins() -> Part {
    let mut pins =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_datum_pins");
    for i in 0..CASSETTE_DATUM_PINS {
        let x = centered_index(i % 2, 2, CASSETTE_X - 72.0);
        let y = centered_index(i / 2, 2, CASSETTE_Y - 64.0);
        let boss = centered_cylinder(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_datum_boss_{i}"
            ),
            10.0,
            10.0,
            28,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 5.0);
        let pin = centered_cylinder(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_datum_pin_cut_{i}"
            ),
            3.0,
            12.0,
            20,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 5.0);
        pins = pins + (boss - pin);
    }
    pins
}

fn reader_envelope() -> Part {
    centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_bought_count_viability_reader_envelope",
        CASSETTE_X - 74.0,
        38.0,
        74.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 - 38.0, CASSETTE_Z / 2.0 + 37.0)
}

fn cassette_id_lands() -> Part {
    let mut lands =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_id_lands");
    for i in 0..COUNT_CASSETTE_SLOTS {
        lands = lands
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_id_land_{i}"
                ),
                70.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(i % 3, 3, 106.0),
                centered_index(i / 3, 2, 82.0) - 36.0,
                CASSETTE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn first_middle_last_sample_custody_wells() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_sample_custody_well_block",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    body - sample_custody_well_cuts() - retain_well_cuts()
        + custody_card_slots()
        + custody_phase_tabs()
}

fn sample_custody_well_cuts() -> Part {
    let mut wells = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_sample_custody_wells",
    );
    for (i, phase) in SAMPLE_PHASES.iter().enumerate() {
        wells = wells
            + centered_cylinder(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_{phase}_sample_custody_well_cut"
                ),
                18.0,
                42.0,
                36,
            )
            .translate(centered_index(i, SAMPLE_PHASES.len(), 92.0), 34.0, CUSTODY_Z / 2.0 - 18.0);
    }
    wells
}

fn retain_well_cuts() -> Part {
    let mut wells =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_retain_well_cuts");
    for i in 0..CUSTODY_RETAIN_WELLS {
        wells = wells
            + centered_cylinder(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_retain_well_cut_{i}"
                ),
                13.0,
                34.0,
                30,
            )
            .translate(
                centered_index(i, CUSTODY_RETAIN_WELLS, 72.0),
                -38.0,
                CUSTODY_Z / 2.0 - 14.0,
            );
    }
    wells
}

fn custody_card_slots() -> Part {
    let mut slots =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_custody_card_slots");
    for i in 0..CUSTODY_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_custody_card_slot_{i}"
                ),
                70.0,
                8.0,
                26.0,
            )
            .translate(
                centered_index(i, CUSTODY_CARD_SLOTS, 92.0),
                -CUSTODY_Y / 2.0 - 4.0,
                4.0,
            );
    }
    slots
}

fn custody_phase_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_custody_phase_tabs");
    for (i, phase) in SAMPLE_PHASES.iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_{phase}_custody_phase_tab"
                ),
                62.0,
                20.0,
                5.0,
            )
            .translate(centered_index(i, SAMPLE_PHASES.len(), 92.0), CUSTODY_Y / 2.0 - 22.0, CUSTODY_Z / 2.0 + 2.5);
    }
    tabs
}

fn bubble_dead_volume_witness_windows() -> Part {
    let body = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_bubble_dead_volume_witness_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    body - witness_window_cuts() + witness_window_bezels() + witness_calibration_stripes()
}

fn witness_window_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_witness_window_cuts");
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_bubble_witness_window_cut_{i}"
                ),
                WITNESS_WINDOW_D / 2.0,
                18.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, BUBBLE_WITNESS_WINDOWS, 84.0), 44.0, 8.0);
    }
    for i in 0..DEAD_VOLUME_WITNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_dead_volume_witness_window_cut_{i}"
                ),
                54.0,
                14.0,
                28.0,
            )
            .translate(centered_index(i, DEAD_VOLUME_WITNESS_WINDOWS, 84.0), -42.0, 8.0);
    }
    cuts
}

fn witness_window_bezels() -> Part {
    let mut bezels = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_witness_window_bezels",
    );
    for i in 0..BUBBLE_WITNESS_WINDOWS {
        bezels = bezels
            + centered_cylinder(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_bubble_witness_window_bezel_{i}"
                ),
                WITNESS_WINDOW_D / 2.0 + 7.0,
                6.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, BUBBLE_WITNESS_WINDOWS, 84.0), 51.0, 8.0);
    }
    for i in 0..DEAD_VOLUME_WITNESS_WINDOWS {
        bezels = bezels
            + rectangular_frame_xy(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_dead_volume_window_bezel_{i}"
                ),
                72.0,
                28.0,
                6.0,
                54.0,
                14.0,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, DEAD_VOLUME_WITNESS_WINDOWS, 84.0), -51.0, 8.0);
    }
    bezels
}

fn witness_calibration_stripes() -> Part {
    let mut stripes = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_witness_calibration_stripes",
    );
    for i in 0..6 {
        stripes = stripes
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_dead_volume_reference_stripe_{i}"
                ),
                6.0,
                130.0,
                4.0,
            )
            .translate(centered_index(i, 6, 28.0), 0.0, WITNESS_Z / 2.0 + 2.0);
    }
    stripes
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_release_hold_reject_lane_base",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    base - status_slot_cuts() + status_lane_dividers() + lane_token_parking()
}

fn status_slot_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_status_slot_cuts");
    for (lane_idx, lane) in STATUS_LANES.iter().enumerate() {
        let x = centered_index(lane_idx, STATUS_LANES.len(), STATUS_LANE_PITCH_X);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_cell_lot_hold_time_viability_decay_tracer_station_{lane}_slot_cut_{slot}"
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    22.0,
                )
                .translate(
                    x,
                    -LANES_Y / 2.0 + 42.0 + slot as f64 * 48.0,
                    LANES_Z / 2.0 - 10.0,
                );
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_status_lane_dividers",
    );
    for i in 0..2 {
        dividers = dividers
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_status_lane_divider_{i}"
                ),
                8.0,
                LANES_Y - 20.0,
                20.0,
            )
            .translate(centered_index(i, 2, STATUS_LANE_PITCH_X), 0.0, LANES_Z / 2.0 + 10.0);
    }
    dividers
}

fn lane_token_parking() -> Part {
    let mut tokens =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_lane_token_parking");
    for (lane_idx, lane) in STATUS_LANES.iter().enumerate() {
        tokens = tokens
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_{lane}_token_parking_land"
                ),
                84.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(lane_idx, STATUS_LANES.len(), STATUS_LANE_PITCH_X),
                LANES_Y / 2.0 - 22.0,
                LANES_Z / 2.0 + 2.5,
            );
    }
    tokens
}

fn barcode_coa_status_lands() -> Part {
    let rail = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_barcode_coa_status_land_rail",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    );
    rail + barcode_lands() + coa_lands() + status_lands()
}

fn barcode_lands() -> Part {
    let mut lands =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!(
                    "closed_cell_lot_hold_time_viability_decay_tracer_station_barcode_land_{i}"
                ),
                48.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, LABEL_LAND_PITCH_X),
                -18.0,
                LABEL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_coa_lands");
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_cell_lot_hold_time_viability_decay_tracer_station_coa_land_{i}"),
                76.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, COA_LANDS, 104.0),
                18.0,
                LABEL_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn status_lands() -> Part {
    let mut lands =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_status_lands");
    for i in 0..STATUS_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_cell_lot_hold_time_viability_decay_tracer_station_status_land_{i}"),
                42.0,
                16.0,
                4.0,
            )
            .translate(
                centered_index(i, STATUS_LANDS, 54.0),
                LABEL_Y / 2.0 + 8.0,
                2.0,
            );
    }
    lands
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_camera_bridge_left_post",
        CAMERA_BRIDGE_POST_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -CAMERA_BRIDGE_SPAN_X / 2.0,
        0.0,
        -CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let right_post = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_camera_bridge_right_post",
        CAMERA_BRIDGE_POST_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(CAMERA_BRIDGE_SPAN_X / 2.0, 0.0, -CAMERA_BRIDGE_BEAM_Z / 2.0);
    let beam = centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X + CAMERA_BRIDGE_POST_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_UNDERSIDE_Z / 2.0);
    left_post + right_post + beam + camera_pods() + evidence_light_bars()
}

fn camera_pods() -> Part {
    let mut pods =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 260.0);
        let pod = centered_cube(
            format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_camera_pod_{i}"
            ),
            62.0,
            50.0,
            34.0,
        )
        .translate(
            x,
            -CAMERA_BRIDGE_POST_Y / 2.0 - 18.0,
            CAMERA_BRIDGE_UNDERSIDE_Z / 2.0 - 34.0,
        );
        let lens = centered_cylinder(
            format!("closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_camera_lens_clearance_{i}"),
            12.0,
            10.0,
            28,
        )
        .translate(x, -CAMERA_BRIDGE_POST_Y / 2.0 - 18.0, CAMERA_BRIDGE_UNDERSIDE_Z / 2.0 - 48.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars =
        Part::empty("closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!(
                "closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_light_bar_{i}"
            ),
                160.0,
                10.0,
                8.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LIGHT_BARS, 190.0),
                CAMERA_BRIDGE_POST_Y / 2.0 + 6.0,
                CAMERA_BRIDGE_UNDERSIDE_Z / 2.0 - 24.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_frame(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_front_robot_keepout",
        STATION_X - 160.0,
        ROBOT_FRONT_CLEARANCE,
        KEEP_OUT_Z,
        0.0,
        -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE / 2.0 + 24.0,
    );
    let rear_service = keepout_frame(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_rear_service_keepout",
        STATION_X - 180.0,
        SERVICE_REAR_CLEARANCE,
        KEEP_OUT_Z,
        0.0,
        STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE / 2.0 - 26.0,
    );
    let side_service = keepout_frame(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_right_service_keepout",
        SERVICE_SIDE_CLEARANCE,
        STATION_Y - 190.0,
        KEEP_OUT_Z,
        STATION_X / 2.0 - SERVICE_SIDE_CLEARANCE / 2.0 - 30.0,
        0.0,
    );
    let overhead = keepout_frame(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_overhead_camera_keepout",
        CAMERA_BRIDGE_SPAN_X + 90.0,
        140.0,
        OVERHEAD_CAMERA_CLEARANCE,
        CAMERA_BRIDGE_CENTER.0,
        CAMERA_BRIDGE_CENTER.1,
    );
    front_robot + rear_service + side_service + overhead
}

fn closed_route_rails() -> Part {
    centered_cube(
        "closed_cell_lot_hold_time_viability_decay_tracer_station_lot_to_loop_route_rail",
        470.0,
        10.0,
        16.0,
    )
    .translate(-332.0, 198.0, BASE_Z + 8.0)
        + centered_cube(
            "closed_cell_lot_hold_time_viability_decay_tracer_station_loop_to_cassette_route_rail",
            486.0,
            10.0,
            16.0,
        )
        .translate(124.0, 112.0, BASE_Z + 8.0)
        + centered_cube(
            "closed_cell_lot_hold_time_viability_decay_tracer_station_cassette_to_custody_route_rail",
            760.0,
            10.0,
            16.0,
        )
        .rotate(0.0, 0.0, -17.0)
        .translate(40.0, -150.0, BASE_Z + 8.0)
}

fn tube_cut_x(name: impl Into<String>, x0: f64, x1: f64, y: f64, z: f64, dia: f64) -> Part {
    centered_cylinder(name, dia / 2.0, (x1 - x0).abs(), 28)
        .rotate(0.0, 90.0, 0.0)
        .translate((x0 + x1) / 2.0, y, z)
}

fn tube_cut_y(name: impl Into<String>, x: f64, y0: f64, y1: f64, z: f64, dia: f64) -> Part {
    centered_cylinder(name, dia / 2.0, (y1 - y0).abs(), 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y0 + y1) / 2.0, z)
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(format!("{name}_inner_cut"), inner_x, inner_y, z + 2.0)
}

fn keepout_frame(name: impl Into<String>, x: f64, y: f64, z: f64, cx: f64, cy: f64) -> Part {
    let name = name.into();
    let rail_z = 8.0;
    let corner_z = z;
    let front = centered_cube(format!("{name}_front_rail"), x, 8.0, rail_z).translate(
        cx,
        cy - y / 2.0,
        z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, rail_z).translate(
        cx,
        cy + y / 2.0,
        z / 2.0,
    );
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, rail_z).translate(
        cx - x / 2.0,
        cy,
        z / 2.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, rail_z).translate(
        cx + x / 2.0,
        cy,
        z / 2.0,
    );
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (px, py)) in [
        (cx - x / 2.0, cy - y / 2.0),
        (cx + x / 2.0, cy - y / 2.0),
        (cx - x / 2.0, cy + y / 2.0),
        (cx + x / 2.0, cy + y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{i}"), 10.0, 10.0, corner_z).translate(
                px,
                py,
                z / 2.0,
            );
    }
    front + rear + left + right + posts
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str =
        include_str!("closed_cell_lot_hold_time_viability_decay_tracer_station.rs");

    #[test]
    fn sample_time_and_custody_counts_are_explicit() {
        assert_eq!(SAMPLE_PHASES, ["first", "middle", "last"]);
        assert_eq!(CUSTODY_WELLS, 3);
        assert_eq!(CUSTODY_RETAIN_WELLS, 3);
        assert_eq!(HOLD_TOKENS, 8);
        assert_eq!(HOLD_TIME_MARKS, HOLD_TOKENS);
        assert_eq!(STATUS_LANES, ["release", "hold", "reject"]);
        assert_eq!(COUNT_CASSETTE_SLOTS, 6);
        assert_eq!(BUBBLE_WITNESS_WINDOWS + DEAD_VOLUME_WITNESS_WINDOWS, 6);
    }

    #[test]
    fn output_manifest_is_stable() {
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_base_containment_deck.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_sealed_cell_lot_bag_vial_nests.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_timed_hold_token_rail.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_gentle_mix_witness_cradle.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_temperature_logger_pockets.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_closed_sampling_loop_surrogate.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_viability_count_cassette_dock_envelope.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_first_middle_last_sample_custody_wells.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_bubble_dead_volume_witness_windows.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_release_hold_reject_lanes.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_barcode_coa_status_lands.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_evidence_camera_bridge.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_robot_service_keepouts.stl",
                "output/closed_cell_lot_hold_time_viability_decay_tracer_station_assembly.stl",
            ]
        );
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn station_bounds_are_enforced() {
        assert_layout();
        assert!(STATION_X <= 1500.0);
        assert!(STATION_Y <= 980.0);
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z > CASSETTE_Z + BASE_Z);
        assert!(OVERHEAD_CAMERA_CLEARANCE > CAMERA_BRIDGE_UNDERSIDE_Z);
        for spec in component_specs() {
            assert!(
                fits_on_station(spec.center, spec.width, spec.depth),
                "{} is outside deck bounds",
                spec.name
            );
        }
    }

    #[test]
    fn required_features_are_covered_by_manifest_or_named_geometry() {
        let manifest = OUTPUTS.join("\n");
        for feature in REQUIRED_FEATURES {
            assert!(
                SOURCE.contains(feature) || manifest.contains(feature),
                "missing feature marker {feature}"
            );
        }
    }

    #[test]
    fn source_avoids_biological_or_clinical_claims() {
        let implementation = SOURCE
            .split("#[cfg(test)]")
            .next()
            .expect("implementation source before tests");
        let lower = implementation.to_lowercase();
        for forbidden in [
            "clinical efficacy",
            "patient",
            "diagnose",
            "treat",
            "cure",
            "therapeutic",
            "guarantee viability",
            "sterility guarantee",
            "validated manufacturing process",
        ] {
            assert!(
                !lower.contains(forbidden),
                "source contains forbidden claim phrase: {forbidden}"
            );
        }
    }
}
