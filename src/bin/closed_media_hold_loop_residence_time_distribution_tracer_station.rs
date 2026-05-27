use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media hold-loop residence-time distribution tracer station.
//
// This generator models validation-fixture packaging for closed media RTD
// tracer work: equalized hold-loop coil coupon nests, tracer injection ports,
// timed fraction collection, recirculation pump bypass witnessing, thermal
// jacket witnesses, bubble/dead-volume windows, pH/osmolality sample points,
// run token custody, release/hold/reject lanes, evidence capture, and
// robot/service clearance gauges. It is architecture CAD for planning and
// fixture fabrication, not a sterile wetted-path drawing, analytical method,
// biological release criterion, or pressure-rated fluidic design.

const OUTPUT_PREFIX: &str =
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_containment_deck.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_hold_loop_coil_coupon_nests.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_tracer_injection_port_bank.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_timed_fraction_collection_well_rack.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_recirculation_pump_bypass_gauge.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_thermal_jacket_witness_blocks.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_bubble_dead_volume_window_bridge.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_ph_osmolality_sample_points.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_barcode_run_token_and_decision_rail.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_evidence_bridge.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_robot_service_keepout_gauges.stl",
    "output/closed_media_hold_loop_residence_time_distribution_tracer_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "hold_loop_coil_coupon_nests",
    "tracer_injection_ports",
    "timed_fraction_collection_wells",
    "recirculation_pump_bypass_gauge",
    "thermal_jacket_witness_blocks",
    "bubble_windows",
    "dead_volume_windows",
    "ph_sample_points",
    "osmolality_sample_points",
    "barcode_run_token_rail",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "not_a_sterile_wetted_product_design",
    "not_a_pressure_rated_loop",
    "not_an_analytical_acceptance_method",
    "not_a_biological_release_protocol",
    "purchased_pumps_tubing_sensors_and_windows_are_surrogates",
];

const REPRODUCIBILITY_CONTROLS: [&str; 6] = [
    "fixed_output_manifest",
    "millimeter_units",
    "no_random_inputs",
    "named_deterministic_geometry",
    "static_feature_counts",
    "stable_module_rectangles",
];

const PARAMETRIC_REVISION: &str =
    "closed_media_hold_loop_residence_time_distribution_tracer_station_v1";
const UNITS: &str = "millimeters";
const GRID_STEP_MM: f64 = 2.0;
const SEGMENTS: u32 = 32;

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const SUMP_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_TARGET_D: f64 = 18.0;
const LAYOUT_CLEARANCE: f64 = 16.0;

const INJECT_CENTER: (f64, f64) = (-550.0, 220.0);
const INJECT_X: f64 = 300.0;
const INJECT_Y: f64 = 220.0;
const INJECT_Z: f64 = 64.0;
const TRACER_PORTS: usize = HOLD_LOOP_LANES;
const FLUSH_CHASE_PORTS: usize = 2;
const INJECTION_MIXING_TEES: usize = HOLD_LOOP_LANES;
const INJECT_PORT_D: f64 = 11.0;
const INJECT_GUARD_D: f64 = 27.0;

const COIL_CENTER: (f64, f64) = (-140.0, 220.0);
const COIL_X: f64 = 480.0;
const COIL_Y: f64 = 220.0;
const COIL_Z: f64 = 58.0;
const HOLD_LOOP_LANES: usize = 6;
const COUPON_NESTS_PER_LANE: usize = 3;
const COUPON_NESTS: usize = HOLD_LOOP_LANES * COUPON_NESTS_PER_LANE;
const COIL_SERPENTINE_SEGMENTS: usize = 8;
const LOOP_COIL_TARGET_ML: f64 = 120.0;
const LOOP_LENGTH_EQUALITY_TOLERANCE_MM: f64 = 0.05;
const COIL_LANE_PITCH_Y: f64 = 30.0;
const COUPON_PITCH_X: f64 = 118.0;
const COUPON_SOCKET_X: f64 = 70.0;
const COUPON_SOCKET_Y: f64 = 22.0;

const FRACTION_CENTER: (f64, f64) = (435.0, 220.0);
const FRACTION_X: f64 = 500.0;
const FRACTION_Y: f64 = 220.0;
const FRACTION_Z: f64 = 42.0;
const FRACTION_TIMEPOINTS: usize = 6;
const FRACTION_WELLS: usize = HOLD_LOOP_LANES * FRACTION_TIMEPOINTS;
const FRACTION_WELL_D: f64 = 16.0;
const FRACTION_CLEARANCE_D: f64 = 18.4;
const FRACTION_RIM_D: f64 = 25.0;
const FRACTION_PITCH_X: f64 = 68.0;
const FRACTION_PITCH_Y: f64 = 30.0;

const THERMAL_CENTER: (f64, f64) = (-550.0, -70.0);
const THERMAL_X: f64 = 300.0;
const THERMAL_Y: f64 = 220.0;
const THERMAL_Z: f64 = 54.0;
const THERMAL_BLOCKS: usize = 3;
const THERMAL_WELLS_PER_BLOCK: usize = 4;
const THERMAL_WITNESS_WELLS: usize = THERMAL_BLOCKS * THERMAL_WELLS_PER_BLOCK;
const JACKET_CHANNELS: usize = 4;
const THERMAL_WELL_D: f64 = 17.0;

const WINDOW_CENTER: (f64, f64) = (-145.0, -70.0);
const WINDOW_X: f64 = 380.0;
const WINDOW_Y: f64 = 220.0;
const WINDOW_Z: f64 = 26.0;
const BUBBLE_WINDOWS: usize = HOLD_LOOP_LANES;
const DEAD_VOLUME_WINDOWS: usize = HOLD_LOOP_LANES;
const WINDOW_TICKS_PER_LANE: usize = 5;
const BUBBLE_WINDOW_X: f64 = 84.0;
const DEAD_VOLUME_WINDOW_X: f64 = 62.0;
const WINDOW_SLOT_Y: f64 = 18.0;

const PUMP_CENTER: (f64, f64) = (445.0, -70.0);
const PUMP_X: f64 = 450.0;
const PUMP_Y: f64 = 220.0;
const PUMP_Z: f64 = 60.0;
const PUMP_CRADLES: usize = 2;
const BYPASS_GAUGE_WINDOWS: usize = 4;
const BYPASS_VALVE_PADS: usize = 3;
const BYPASS_PRESSURE_PORTS: usize = 4;

const SAMPLE_CENTER: (f64, f64) = (490.0, -325.0);
const SAMPLE_X: f64 = 360.0;
const SAMPLE_Y: f64 = 120.0;
const SAMPLE_Z: f64 = 42.0;
const SAMPLE_ANALYTES: usize = 2;
const SAMPLE_REPLICATES: usize = HOLD_LOOP_LANES;
const SAMPLE_POINTS: usize = SAMPLE_ANALYTES * SAMPLE_REPLICATES;
const SAMPLE_WELL_D: f64 = 15.0;
const SAMPLE_RIM_D: f64 = 22.0;
const SAMPLE_PITCH_X: f64 = 48.0;
const SAMPLE_PITCH_Y: f64 = 42.0;

const TRACE_DECISION_CENTER: (f64, f64) = (-240.0, -325.0);
const TRACE_DECISION_X: f64 = 680.0;
const TRACE_DECISION_Y: f64 = 120.0;
const TRACE_DECISION_Z: f64 = 28.0;
const BARCODE_LANDS: usize = 12;
const RUN_TOKEN_SLOTS: usize = 8;
const DECISION_LANES: usize = 3;
const DECISION_SLOTS_PER_LANE: usize = 4;
const DECISION_SLOTS: usize = DECISION_LANES * DECISION_SLOTS_PER_LANE;
const DECISION_SLOT_X: f64 = 52.0;
const DECISION_SLOT_Y: f64 = 26.0;

const EVIDENCE_BRIDGE_SPAN_X: f64 = 1360.0;
const EVIDENCE_BRIDGE_SPAN_Y: f64 = 650.0;
const EVIDENCE_POST_X: f64 = 30.0;
const EVIDENCE_POST_Y: f64 = 38.0;
const EVIDENCE_CLEARANCE_Z: f64 = 205.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const EVIDENCE_LED_SEGMENTS: usize = 10;

const ROBOT_KEEP_OUT_X: f64 = 1320.0;
const ROBOT_KEEP_OUT_Y: f64 = 720.0;
const ROBOT_KEEP_OUT_Z: f64 = 180.0;
const SERVICE_KEEPOUTS: usize = 5;
const ROBOT_DATUMS: usize = 8;
const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_COIL_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_THERMAL_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_FRACTION_SERVICE_CLEARANCE: f64 = 230.0;
const TOP_EVIDENCE_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - LAYOUT_CLEARANCE
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - LAYOUT_CLEARANCE
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let coil = hold_loop_coil_coupon_nests();
    export(OUTPUTS[1], &coil);

    let injection = tracer_injection_port_bank();
    export(OUTPUTS[2], &injection);

    let fractions = timed_fraction_collection_well_rack();
    export(OUTPUTS[3], &fractions);

    let bypass = recirculation_pump_bypass_gauge();
    export(OUTPUTS[4], &bypass);

    let thermal = thermal_jacket_witness_blocks();
    export(OUTPUTS[5], &thermal);

    let windows = bubble_dead_volume_window_bridge();
    export(OUTPUTS[6], &windows);

    let samples = ph_osmolality_sample_points();
    export(OUTPUTS[7], &samples);

    let trace = barcode_run_token_and_decision_rail();
    export(OUTPUTS[8], &trace);

    let evidence = evidence_bridge();
    export(OUTPUTS[9], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + coil
        + injection
        + fractions
        + bypass
        + thermal
        + windows
        + samples
        + trace
        + evidence
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media hold-loop RTD tracer station:");
    println!("  Revision/units:              {PARAMETRIC_REVISION} / {UNITS}");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Hold-loop lanes:             {HOLD_LOOP_LANES} equalized lanes, {COUPON_NESTS} coupon nests, {COIL_SERPENTINE_SEGMENTS} visible coil route segments"
    );
    println!(
        "  RTD tracer workflow:         {TRACER_PORTS} tracer injection ports, {FRACTION_WELLS} timed fraction wells, {SAMPLE_POINTS} pH/osmolality sample points"
    );
    println!(
        "  Witnessing:                  {BYPASS_GAUGE_WINDOWS} pump bypass windows, {THERMAL_WITNESS_WELLS} thermal wells, {BUBBLE_WINDOWS} bubble windows, {DEAD_VOLUME_WINDOWS} dead-volume windows"
    );
    println!(
        "  Traceability/release:        {BARCODE_LANDS} barcode lands, {RUN_TOKEN_SLOTS} run-token slots, {DECISION_SLOTS} release/hold/reject slots"
    );
    println!(
        "  Evidence/service:            {EVIDENCE_CAMERA_COUNT} camera lands, {EVIDENCE_LED_SEGMENTS} LED witness segments, {SERVICE_KEEPOUTS} service keepouts"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z
}

fn place_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn lane_y(lane: usize) -> f64 {
    COIL_CENTER.1 + centered_index(lane, HOLD_LOOP_LANES, COIL_LANE_PITCH_Y)
}

fn fraction_position(lane: usize, timepoint: usize) -> (f64, f64) {
    (
        FRACTION_CENTER.0 + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_X),
        FRACTION_CENTER.1 + centered_index(lane, HOLD_LOOP_LANES, FRACTION_PITCH_Y),
    )
}

fn sample_position(analyte: usize, replicate: usize) -> (f64, f64) {
    (
        SAMPLE_CENTER.0 + centered_index(replicate, SAMPLE_REPLICATES, SAMPLE_PITCH_X),
        SAMPLE_CENTER.1 + centered_index(analyte, SAMPLE_ANALYTES, SAMPLE_PITCH_Y),
    )
}

fn module_rects() -> [Rect; 8] {
    [
        rect(
            "tracer_injection_port_bank",
            INJECT_CENTER,
            INJECT_X,
            INJECT_Y,
        ),
        rect("hold_loop_coil_coupon_nests", COIL_CENTER, COIL_X, COIL_Y),
        rect(
            "timed_fraction_collection_well_rack",
            FRACTION_CENTER,
            FRACTION_X,
            FRACTION_Y,
        ),
        rect(
            "thermal_jacket_witness_blocks",
            THERMAL_CENTER,
            THERMAL_X,
            THERMAL_Y,
        ),
        rect(
            "bubble_dead_volume_window_bridge",
            WINDOW_CENTER,
            WINDOW_X,
            WINDOW_Y,
        ),
        rect(
            "recirculation_pump_bypass_gauge",
            PUMP_CENTER,
            PUMP_X,
            PUMP_Y,
        ),
        rect(
            "barcode_run_token_and_decision_rail",
            TRACE_DECISION_CENTER,
            TRACE_DECISION_X,
            TRACE_DECISION_Y,
        ),
        rect(
            "ph_osmolality_sample_points",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "rtd_tracer_station_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "rtd_tracer_station_recessed_leak_sump",
        STATION_X - 150.0,
        STATION_Y - 132.0,
        SUMP_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, deck_top_z() - SUMP_DEPTH / 2.0);
    let drain = centered_cylinder(
        "rtd_tracer_station_front_low_point_drain",
        7.0,
        74.0,
        SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 88.0,
        -STATION_Y / 2.0 + 8.0,
        deck_top_z() - 5.0,
    );

    deck - sump - drain - module_insert_sockets() - mounting_holes()
        + perimeter_rims()
        + datum_targets()
        + wet_dry_zone_locator_rails()
}

fn module_insert_sockets() -> Part {
    let mut sockets = Part::empty("rtd_tracer_station_module_insert_sockets");
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("rtd_tracer_station_{}_socket_relief", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                deck_top_z() - SOCKET_DEPTH / 2.0 + 0.1,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("rtd_tracer_station_mounting_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("rtd_tracer_station_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "rtd_tracer_station_left_spill_retention_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "rtd_tracer_station_right_fraction_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "rtd_tracer_station_rear_tubing_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "rtd_tracer_station_low_front_robot_lip",
        STATION_X - 240.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 16.0, deck_top_z() + 10.0);
    left + right + rear + front
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("rtd_tracer_station_robot_datum_targets");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        targets = targets
            + ring_z(
                &format!("rtd_tracer_station_robot_datum_target_{i}"),
                DATUM_TARGET_D,
                8.0,
                4.0,
            )
            .translate(*x, *y, deck_top_z() + 2.0);
    }
    targets
}

fn wet_dry_zone_locator_rails() -> Part {
    let top_row = centered_cube(
        "rtd_tracer_station_rear_injection_coil_fraction_locator_rail",
        STATION_X - 230.0,
        9.0,
        18.0,
    )
    .translate(0.0, 80.0, deck_top_z() + 9.0);
    let witness_row = centered_cube(
        "rtd_tracer_station_middle_thermal_window_pump_locator_rail",
        STATION_X - 260.0,
        9.0,
        18.0,
    )
    .translate(0.0, -205.0, deck_top_z() + 9.0);
    let front_trace = centered_cube(
        "rtd_tracer_station_front_traceability_locator_rail",
        STATION_X - 330.0,
        8.0,
        16.0,
    )
    .translate(0.0, -250.0, deck_top_z() + 8.0);
    top_row + witness_row + front_trace
}

fn hold_loop_coil_coupon_nests() -> Part {
    let body = centered_cube(
        "rtd_tracer_hold_loop_coupon_nest_body",
        COIL_X,
        COIL_Y,
        COIL_Z,
    )
    .translate(COIL_CENTER.0, COIL_CENTER.1, place_z(COIL_Z));

    body - coupon_socket_cuts() - coil_lane_reliefs()
        + coupon_socket_rims()
        + visible_hold_loop_coil_route()
        + equal_length_gauge_ticks()
        + coil_hold_down_bridges()
}

fn coupon_socket_cuts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_hold_loop_coupon_socket_cuts");
    for lane in 0..HOLD_LOOP_LANES {
        for nest in 0..COUPON_NESTS_PER_LANE {
            let (x, y) = coupon_position(lane, nest);
            cuts = cuts
                + centered_cube(
                    format!("rtd_tracer_lane_{lane}_coupon_nest_{nest}_socket_cut"),
                    COUPON_SOCKET_X,
                    COUPON_SOCKET_Y,
                    COIL_Z + 4.0,
                )
                .translate(x, y, place_z(COIL_Z));
        }
    }
    cuts
}

fn coupon_socket_rims() -> Part {
    let mut rims = Part::empty("rtd_tracer_hold_loop_coupon_socket_rims");
    for lane in 0..HOLD_LOOP_LANES {
        for nest in 0..COUPON_NESTS_PER_LANE {
            let (x, y) = coupon_position(lane, nest);
            rims = rims
                + rectangular_frame(
                    &format!("rtd_tracer_lane_{lane}_coupon_nest_{nest}_raised_rim"),
                    COUPON_SOCKET_X + 18.0,
                    COUPON_SOCKET_Y + 14.0,
                    5.0,
                    5.0,
                )
                .translate(x, y, deck_top_z() + COIL_Z + 2.5);
        }
    }
    rims
}

fn coil_lane_reliefs() -> Part {
    let mut reliefs = Part::empty("rtd_tracer_hold_loop_coil_lane_reliefs");
    for lane in 0..HOLD_LOOP_LANES {
        reliefs = reliefs
            + centered_cylinder(
                format!("rtd_tracer_lane_{lane}_rear_loop_bore_relief"),
                3.8,
                COIL_X + 24.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(COIL_CENTER.0, lane_y(lane), place_z(COIL_Z) - 4.0);
    }
    reliefs
}

fn visible_hold_loop_coil_route() -> Part {
    let mut route = Part::empty("rtd_tracer_visible_hold_loop_coil_route");
    for segment in 0..COIL_SERPENTINE_SEGMENTS {
        let y = COIL_CENTER.1 + centered_index(segment, COIL_SERPENTINE_SEGMENTS, 18.0);
        let length = if segment % 2 == 0 {
            COIL_X - 74.0
        } else {
            COIL_X - 132.0
        };
        route = route
            + centered_cube(
                format!("rtd_tracer_serpentine_coil_segment_{segment}"),
                length,
                7.0,
                8.0,
            )
            .translate(COIL_CENTER.0, y, deck_top_z() + COIL_Z + 4.0);
    }

    for turn in 0..(COIL_SERPENTINE_SEGMENTS - 1) {
        let x = COIL_CENTER.0
            + if turn % 2 == 0 {
                COIL_X / 2.0 - 48.0
            } else {
                -COIL_X / 2.0 + 48.0
            };
        route = route
            + centered_cylinder(
                format!("rtd_tracer_serpentine_coil_u_turn_marker_{turn}"),
                11.0,
                8.0,
                SEGMENTS,
            )
            .translate(
                x,
                COIL_CENTER.1 + centered_index(turn, COIL_SERPENTINE_SEGMENTS - 1, 18.0),
                deck_top_z() + COIL_Z + 4.0,
            );
    }
    route
}

fn equal_length_gauge_ticks() -> Part {
    let mut ticks = Part::empty("rtd_tracer_equal_length_gauge_ticks");
    for lane in 0..HOLD_LOOP_LANES {
        let y = lane_y(lane) - 12.0;
        for tick in 0..5 {
            ticks = ticks
                + centered_cube(
                    format!("rtd_tracer_lane_{lane}_residence_time_tick_{tick}"),
                    3.0,
                    16.0,
                    4.0,
                )
                .translate(
                    COIL_CENTER.0 - COIL_X / 2.0 + 74.0 + tick as f64 * 80.0,
                    y,
                    deck_top_z() + COIL_Z + 6.0,
                );
        }
    }
    ticks
}

fn coil_hold_down_bridges() -> Part {
    let left = centered_cube(
        "rtd_tracer_hold_loop_left_tube_hold_down_bridge",
        24.0,
        COIL_Y - 48.0,
        18.0,
    )
    .translate(
        COIL_CENTER.0 - COIL_X / 2.0 + 30.0,
        COIL_CENTER.1,
        deck_top_z() + COIL_Z + 9.0,
    );
    let right = centered_cube(
        "rtd_tracer_hold_loop_right_tube_hold_down_bridge",
        24.0,
        COIL_Y - 48.0,
        18.0,
    )
    .translate(
        COIL_CENTER.0 + COIL_X / 2.0 - 30.0,
        COIL_CENTER.1,
        deck_top_z() + COIL_Z + 9.0,
    );
    left + right
}

fn tracer_injection_port_bank() -> Part {
    let body = centered_cube(
        "rtd_tracer_injection_port_bank_body",
        INJECT_X,
        INJECT_Y,
        INJECT_Z,
    )
    .translate(INJECT_CENTER.0, INJECT_CENTER.1, place_z(INJECT_Z));

    body - injection_port_cuts() - injection_header_bores()
        + injection_port_guard_rings()
        + tracer_syringe_plunger_reliefs()
        + mixing_tee_witness_blocks()
        + chase_flush_port_pair()
}

fn injection_port_cuts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_injection_port_cuts");
    for lane in 0..TRACER_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("rtd_tracer_lane_{lane}_tracer_injection_port_cut"),
                INJECT_PORT_D / 2.0,
                INJECT_Z + 8.0,
                28,
            )
            .translate(
                INJECT_CENTER.0 - 50.0,
                INJECT_CENTER.1 + centered_index(lane, TRACER_PORTS, 29.0),
                place_z(INJECT_Z),
            );
    }
    cuts
}

fn injection_port_guard_rings() -> Part {
    let mut rings = Part::empty("rtd_tracer_injection_port_guard_rings");
    for lane in 0..TRACER_PORTS {
        let x = INJECT_CENTER.0 - 50.0;
        let y = INJECT_CENTER.1 + centered_index(lane, TRACER_PORTS, 29.0);
        rings = rings
            + ring_z(
                &format!("rtd_tracer_lane_{lane}_injection_septum_guard_ring"),
                INJECT_GUARD_D,
                INJECT_PORT_D + 1.4,
                5.0,
            )
            .translate(x, y, deck_top_z() + INJECT_Z + 2.5);
    }
    rings
}

fn injection_header_bores() -> Part {
    let inlet_header = centered_cylinder(
        "rtd_tracer_injection_bank_equalized_inlet_header",
        3.6,
        INJECT_Y + 18.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        INJECT_CENTER.0 + 42.0,
        INJECT_CENTER.1,
        place_z(INJECT_Z) - 8.0,
    );
    let outlet_header = centered_cylinder(
        "rtd_tracer_injection_bank_chase_outlet_header",
        3.6,
        INJECT_Y + 18.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        INJECT_CENTER.0 + 86.0,
        INJECT_CENTER.1,
        place_z(INJECT_Z) - 8.0,
    );
    inlet_header + outlet_header
}

fn tracer_syringe_plunger_reliefs() -> Part {
    let mut reliefs = Part::empty("rtd_tracer_syringe_plunger_reliefs");
    for lane in 0..TRACER_PORTS {
        reliefs = reliefs
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_plunger_access_clearance"),
                74.0,
                10.0,
                8.0,
            )
            .translate(
                INJECT_CENTER.0 - 108.0,
                INJECT_CENTER.1 + centered_index(lane, TRACER_PORTS, 29.0),
                deck_top_z() + INJECT_Z + 4.0,
            );
    }
    reliefs
}

fn mixing_tee_witness_blocks() -> Part {
    let mut tees = Part::empty("rtd_tracer_injection_mixing_tee_witness_blocks");
    for lane in 0..INJECTION_MIXING_TEES {
        tees = tees
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_mixing_tee_witness_block"),
                38.0,
                16.0,
                18.0,
            )
            .translate(
                INJECT_CENTER.0 + 72.0,
                INJECT_CENTER.1 + centered_index(lane, INJECTION_MIXING_TEES, 29.0),
                deck_top_z() + INJECT_Z + 9.0,
            );
    }
    tees
}

fn chase_flush_port_pair() -> Part {
    let mut ports = Part::empty("rtd_tracer_chase_flush_port_pair");
    for i in 0..FLUSH_CHASE_PORTS {
        let x = INJECT_CENTER.0 + centered_index(i, FLUSH_CHASE_PORTS, 72.0);
        let y = INJECT_CENTER.1 - INJECT_Y / 2.0 + 28.0;
        ports =
            ports
                + low_dead_volume_port(&format!("rtd_tracer_chase_flush_port_{i}"), 10.0)
                    .translate(x, y, deck_top_z() + INJECT_Z + 8.0);
    }
    ports
}

fn timed_fraction_collection_well_rack() -> Part {
    let body = centered_cube(
        "rtd_tracer_timed_fraction_collection_rack_body",
        FRACTION_X,
        FRACTION_Y,
        FRACTION_Z,
    )
    .translate(FRACTION_CENTER.0, FRACTION_CENTER.1, place_z(FRACTION_Z));

    body - fraction_well_cuts()
        + fraction_well_rims()
        + fraction_timepoint_dividers()
        + fraction_lane_inlet_manifold()
        + fraction_clock_token_lands()
}

fn fraction_well_cuts() -> Part {
    let mut wells = Part::empty("rtd_tracer_timed_fraction_well_cuts");
    for lane in 0..HOLD_LOOP_LANES {
        for timepoint in 0..FRACTION_TIMEPOINTS {
            let (x, y) = fraction_position(lane, timepoint);
            wells = wells
                + centered_cylinder(
                    format!("rtd_tracer_lane_{lane}_timepoint_{timepoint}_fraction_well_cut"),
                    FRACTION_CLEARANCE_D / 2.0,
                    FRACTION_Z + 6.0,
                    30,
                )
                .translate(x, y, place_z(FRACTION_Z));
        }
    }
    wells
}

fn fraction_well_rims() -> Part {
    let mut rims = Part::empty("rtd_tracer_timed_fraction_well_rims");
    for lane in 0..HOLD_LOOP_LANES {
        for timepoint in 0..FRACTION_TIMEPOINTS {
            let (x, y) = fraction_position(lane, timepoint);
            rims = rims
                + ring_z(
                    &format!("rtd_tracer_lane_{lane}_timepoint_{timepoint}_fraction_well_rim"),
                    FRACTION_RIM_D,
                    FRACTION_WELL_D,
                    4.0,
                )
                .translate(x, y, deck_top_z() + FRACTION_Z + 2.0);
        }
    }
    rims
}

fn fraction_timepoint_dividers() -> Part {
    let mut dividers = Part::empty("rtd_tracer_fraction_timepoint_dividers");
    for timepoint in 0..(FRACTION_TIMEPOINTS - 1) {
        let x = FRACTION_CENTER.0
            + (centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_X)
                + centered_index(timepoint + 1, FRACTION_TIMEPOINTS, FRACTION_PITCH_X))
                / 2.0;
        dividers = dividers
            + centered_cube(
                format!("rtd_tracer_fraction_timepoint_separator_{timepoint}"),
                5.0,
                FRACTION_Y - 44.0,
                22.0,
            )
            .translate(x, FRACTION_CENTER.1, deck_top_z() + FRACTION_Z + 11.0);
    }
    dividers
}

fn fraction_lane_inlet_manifold() -> Part {
    let rear = centered_cube(
        "rtd_tracer_fraction_rack_rear_inlet_manifold_rail",
        FRACTION_X - 54.0,
        18.0,
        18.0,
    )
    .translate(
        FRACTION_CENTER.0,
        FRACTION_CENTER.1 + FRACTION_Y / 2.0 - 24.0,
        deck_top_z() + FRACTION_Z + 9.0,
    );
    let front = centered_cube(
        "rtd_tracer_fraction_rack_front_archive_manifold_rail",
        FRACTION_X - 54.0,
        14.0,
        14.0,
    )
    .translate(
        FRACTION_CENTER.0,
        FRACTION_CENTER.1 - FRACTION_Y / 2.0 + 22.0,
        deck_top_z() + FRACTION_Z + 7.0,
    );
    rear + front
}

fn fraction_clock_token_lands() -> Part {
    let mut lands = Part::empty("rtd_tracer_fraction_clock_token_lands");
    for timepoint in 0..FRACTION_TIMEPOINTS {
        lands = lands
            + centered_cube(
                format!("rtd_tracer_fraction_timepoint_{timepoint}_clock_token_land"),
                44.0,
                12.0,
                5.0,
            )
            .translate(
                FRACTION_CENTER.0
                    + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_X),
                FRACTION_CENTER.1 - FRACTION_Y / 2.0 + 46.0,
                deck_top_z() + FRACTION_Z + 2.5,
            );
    }
    lands
}

fn recirculation_pump_bypass_gauge() -> Part {
    let body = centered_cube(
        "rtd_tracer_recirculation_pump_bypass_gauge_body",
        PUMP_X,
        PUMP_Y,
        PUMP_Z,
    )
    .translate(PUMP_CENTER.0, PUMP_CENTER.1, place_z(PUMP_Z));

    body - pump_cradle_reliefs() - bypass_flow_bores()
        + pump_cradle_lips()
        + bypass_gauge_window_frames()
        + bypass_valve_pads()
        + pressure_port_rings()
        + recirculation_direction_tokens()
}

fn pump_cradle_reliefs() -> Part {
    let mut reliefs = Part::empty("rtd_tracer_pump_cradle_reliefs");
    for pump in 0..PUMP_CRADLES {
        reliefs = reliefs
            + centered_cube(
                format!("rtd_tracer_recirc_pump_{pump}_cradle_relief"),
                126.0,
                68.0,
                PUMP_Z + 5.0,
            )
            .translate(
                PUMP_CENTER.0 + centered_index(pump, PUMP_CRADLES, 150.0),
                PUMP_CENTER.1 + 28.0,
                place_z(PUMP_Z),
            );
    }
    reliefs
}

fn bypass_flow_bores() -> Part {
    let main = centered_cylinder(
        "rtd_tracer_pump_bypass_main_flow_bore",
        4.2,
        PUMP_X + 20.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(PUMP_CENTER.0, PUMP_CENTER.1 - 50.0, place_z(PUMP_Z) - 6.0);
    let bypass = centered_cylinder(
        "rtd_tracer_pump_bypass_reference_flow_bore",
        3.4,
        PUMP_X - 42.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(PUMP_CENTER.0, PUMP_CENTER.1 - 6.0, place_z(PUMP_Z) - 8.0);
    main + bypass
}

fn pump_cradle_lips() -> Part {
    let mut lips = Part::empty("rtd_tracer_pump_cradle_lips");
    for pump in 0..PUMP_CRADLES {
        let x = PUMP_CENTER.0 + centered_index(pump, PUMP_CRADLES, 150.0);
        lips = lips
            + rectangular_frame(
                &format!("rtd_tracer_recirc_pump_{pump}_cradle_lip"),
                142.0,
                84.0,
                8.0,
                8.0,
            )
            .translate(x, PUMP_CENTER.1 + 28.0, deck_top_z() + PUMP_Z + 4.0);
    }
    lips
}

fn bypass_gauge_window_frames() -> Part {
    let mut windows = Part::empty("rtd_tracer_bypass_gauge_window_frames");
    for i in 0..BYPASS_GAUGE_WINDOWS {
        windows = windows
            + rectangular_frame(
                &format!("rtd_tracer_bypass_gauge_flow_window_{i}"),
                58.0,
                22.0,
                5.0,
                5.0,
            )
            .translate(
                PUMP_CENTER.0 + centered_index(i, BYPASS_GAUGE_WINDOWS, 72.0),
                PUMP_CENTER.1 - 52.0,
                deck_top_z() + PUMP_Z + 2.5,
            );
    }
    windows
}

fn bypass_valve_pads() -> Part {
    let mut pads = Part::empty("rtd_tracer_bypass_valve_pads");
    for i in 0..BYPASS_VALVE_PADS {
        pads = pads
            + centered_cube(format!("rtd_tracer_bypass_valve_pad_{i}"), 54.0, 34.0, 14.0)
                .translate(
                    PUMP_CENTER.0 + centered_index(i, BYPASS_VALVE_PADS, 92.0),
                    PUMP_CENTER.1 + PUMP_Y / 2.0 - 34.0,
                    deck_top_z() + PUMP_Z + 7.0,
                );
    }
    pads
}

fn pressure_port_rings() -> Part {
    let mut rings = Part::empty("rtd_tracer_bypass_pressure_port_rings");
    for i in 0..BYPASS_PRESSURE_PORTS {
        rings = rings
            + low_dead_volume_port(&format!("rtd_tracer_bypass_pressure_port_{i}"), 9.0).translate(
                PUMP_CENTER.0 + centered_index(i, BYPASS_PRESSURE_PORTS, 82.0),
                PUMP_CENTER.1 - PUMP_Y / 2.0 + 28.0,
                deck_top_z() + PUMP_Z + 8.0,
            );
    }
    rings
}

fn recirculation_direction_tokens() -> Part {
    let mut tokens = Part::empty("rtd_tracer_recirculation_direction_tokens");
    for i in 0..8 {
        tokens = tokens
            + centered_cube(
                format!("rtd_tracer_pump_bypass_direction_marker_{i}"),
                30.0,
                5.0,
                5.0,
            )
            .translate(
                PUMP_CENTER.0 - PUMP_X / 2.0 + 66.0 + i as f64 * 45.0,
                PUMP_CENTER.1 - 92.0,
                deck_top_z() + PUMP_Z + 2.5,
            );
    }
    tokens
}

fn thermal_jacket_witness_blocks() -> Part {
    let body = centered_cube(
        "rtd_tracer_thermal_jacket_witness_body",
        THERMAL_X,
        THERMAL_Y,
        THERMAL_Z,
    )
    .translate(THERMAL_CENTER.0, THERMAL_CENTER.1, place_z(THERMAL_Z));

    body - thermal_well_cuts() - thermal_jacket_channel_cuts()
        + thermal_well_rims()
        + thermal_probe_bridge_sockets()
        + thermal_gradient_token_lands()
}

fn thermal_well_cuts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_thermal_witness_well_cuts");
    for block in 0..THERMAL_BLOCKS {
        for well in 0..THERMAL_WELLS_PER_BLOCK {
            let x = THERMAL_CENTER.0 + centered_index(block, THERMAL_BLOCKS, 82.0);
            let y = THERMAL_CENTER.1 + centered_index(well, THERMAL_WELLS_PER_BLOCK, 34.0);
            cuts = cuts
                + centered_cylinder(
                    format!("rtd_tracer_thermal_block_{block}_witness_well_{well}_cut"),
                    THERMAL_WELL_D / 2.0,
                    THERMAL_Z + 6.0,
                    28,
                )
                .translate(x, y, place_z(THERMAL_Z));
        }
    }
    cuts
}

fn thermal_well_rims() -> Part {
    let mut rims = Part::empty("rtd_tracer_thermal_witness_well_rims");
    for block in 0..THERMAL_BLOCKS {
        for well in 0..THERMAL_WELLS_PER_BLOCK {
            let x = THERMAL_CENTER.0 + centered_index(block, THERMAL_BLOCKS, 82.0);
            let y = THERMAL_CENTER.1 + centered_index(well, THERMAL_WELLS_PER_BLOCK, 34.0);
            rims = rims
                + ring_z(
                    &format!("rtd_tracer_thermal_block_{block}_witness_well_{well}_rim"),
                    THERMAL_WELL_D + 10.0,
                    THERMAL_WELL_D + 1.0,
                    4.0,
                )
                .translate(x, y, deck_top_z() + THERMAL_Z + 2.0);
        }
    }
    rims
}

fn thermal_jacket_channel_cuts() -> Part {
    let mut channels = Part::empty("rtd_tracer_thermal_jacket_channel_cuts");
    for channel in 0..JACKET_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("rtd_tracer_thermal_jacket_channel_{channel}_cut"),
                4.2,
                THERMAL_X + 18.0,
                22,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                THERMAL_CENTER.0,
                THERMAL_CENTER.1 + centered_index(channel, JACKET_CHANNELS, 38.0),
                place_z(THERMAL_Z) - 7.0,
            );
    }
    channels
}

fn thermal_probe_bridge_sockets() -> Part {
    let mut sockets = Part::empty("rtd_tracer_thermal_probe_bridge_sockets");
    for block in 0..THERMAL_BLOCKS {
        sockets = sockets
            + centered_cube(
                format!("rtd_tracer_thermal_block_{block}_probe_bridge_socket"),
                66.0,
                18.0,
                16.0,
            )
            .translate(
                THERMAL_CENTER.0 + centered_index(block, THERMAL_BLOCKS, 82.0),
                THERMAL_CENTER.1 + THERMAL_Y / 2.0 - 24.0,
                deck_top_z() + THERMAL_Z + 8.0,
            );
    }
    sockets
}

fn thermal_gradient_token_lands() -> Part {
    let mut lands = Part::empty("rtd_tracer_thermal_gradient_token_lands");
    for i in 0..6 {
        lands = lands
            + centered_cube(
                format!("rtd_tracer_thermal_gradient_token_land_{i}"),
                34.0,
                14.0,
                5.0,
            )
            .translate(
                THERMAL_CENTER.0 + centered_index(i, 6, 42.0),
                THERMAL_CENTER.1 - THERMAL_Y / 2.0 + 24.0,
                deck_top_z() + THERMAL_Z + 2.5,
            );
    }
    lands
}

fn bubble_dead_volume_window_bridge() -> Part {
    let bridge = centered_cube(
        "rtd_tracer_bubble_dead_volume_window_bridge_frame",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    )
    .translate(
        WINDOW_CENTER.0,
        WINDOW_CENTER.1,
        deck_top_z() + COIL_Z + 26.0,
    );

    bridge - bubble_window_cutouts() - dead_volume_window_cutouts()
        + bubble_window_ribs()
        + dead_volume_timing_ticks()
        + window_camera_lands()
}

fn bubble_window_cutouts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_bubble_window_cutouts");
    for lane in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_bubble_window_cutout"),
                BUBBLE_WINDOW_X,
                WINDOW_SLOT_Y,
                WINDOW_Z + 4.0,
            )
            .translate(
                WINDOW_CENTER.0 - 70.0,
                WINDOW_CENTER.1 + centered_index(lane, BUBBLE_WINDOWS, 29.0),
                deck_top_z() + COIL_Z + 26.0,
            );
    }
    cuts
}

fn dead_volume_window_cutouts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_dead_volume_window_cutouts");
    for lane in 0..DEAD_VOLUME_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_dead_volume_window_cutout"),
                DEAD_VOLUME_WINDOW_X,
                WINDOW_SLOT_Y,
                WINDOW_Z + 4.0,
            )
            .translate(
                WINDOW_CENTER.0 + 88.0,
                WINDOW_CENTER.1 + centered_index(lane, DEAD_VOLUME_WINDOWS, 29.0),
                deck_top_z() + COIL_Z + 26.0,
            );
    }
    cuts
}

fn bubble_window_ribs() -> Part {
    let mut ribs = Part::empty("rtd_tracer_bubble_dead_volume_window_ribs");
    for lane in 0..HOLD_LOOP_LANES {
        let y = WINDOW_CENTER.1 + centered_index(lane, HOLD_LOOP_LANES, 29.0);
        ribs = ribs
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_bubble_window_left_frame"),
                6.0,
                WINDOW_SLOT_Y + 14.0,
                WINDOW_Z + 4.0,
            )
            .translate(WINDOW_CENTER.0 - 116.0, y, deck_top_z() + COIL_Z + 28.0)
            + centered_cube(
                format!("rtd_tracer_lane_{lane}_dead_volume_window_right_frame"),
                6.0,
                WINDOW_SLOT_Y + 14.0,
                WINDOW_Z + 4.0,
            )
            .translate(WINDOW_CENTER.0 + 126.0, y, deck_top_z() + COIL_Z + 28.0);
    }
    ribs
}

fn dead_volume_timing_ticks() -> Part {
    let mut ticks = Part::empty("rtd_tracer_dead_volume_timing_ticks");
    for lane in 0..HOLD_LOOP_LANES {
        for tick in 0..WINDOW_TICKS_PER_LANE {
            ticks = ticks
                + centered_cube(
                    format!("rtd_tracer_lane_{lane}_dead_volume_tick_{tick}"),
                    2.0,
                    WINDOW_SLOT_Y + 8.0,
                    3.0,
                )
                .translate(
                    WINDOW_CENTER.0 + 58.0 + tick as f64 * 15.0,
                    WINDOW_CENTER.1 + centered_index(lane, HOLD_LOOP_LANES, 29.0),
                    deck_top_z() + COIL_Z + 43.0,
                );
        }
    }
    ticks
}

fn window_camera_lands() -> Part {
    let left = centered_cube("rtd_tracer_bubble_window_camera_land", 72.0, 28.0, 7.0).translate(
        WINDOW_CENTER.0 - WINDOW_X / 2.0 + 56.0,
        WINDOW_CENTER.1 + WINDOW_Y / 2.0 - 26.0,
        deck_top_z() + COIL_Z + 43.5,
    );
    let right = centered_cube("rtd_tracer_dead_volume_window_camera_land", 72.0, 28.0, 7.0)
        .translate(
            WINDOW_CENTER.0 + WINDOW_X / 2.0 - 56.0,
            WINDOW_CENTER.1 + WINDOW_Y / 2.0 - 26.0,
            deck_top_z() + COIL_Z + 43.5,
        );
    left + right
}

fn ph_osmolality_sample_points() -> Part {
    let body = centered_cube(
        "rtd_tracer_ph_osmolality_sample_point_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, place_z(SAMPLE_Z));

    body - sample_point_cuts()
        + sample_point_rims()
        + sample_analyte_lane_tags()
        + sample_chain_of_custody_strip()
}

fn sample_point_cuts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_ph_osmolality_sample_point_cuts");
    for analyte in 0..SAMPLE_ANALYTES {
        for replicate in 0..SAMPLE_REPLICATES {
            let (x, y) = sample_position(analyte, replicate);
            cuts = cuts
                + centered_cylinder(
                    format!("rtd_tracer_analyte_{analyte}_replicate_{replicate}_sample_cut"),
                    SAMPLE_WELL_D / 2.0,
                    SAMPLE_Z + 5.0,
                    28,
                )
                .translate(x, y, place_z(SAMPLE_Z));
        }
    }
    cuts
}

fn sample_point_rims() -> Part {
    let mut rims = Part::empty("rtd_tracer_ph_osmolality_sample_point_rims");
    for analyte in 0..SAMPLE_ANALYTES {
        for replicate in 0..SAMPLE_REPLICATES {
            let (x, y) = sample_position(analyte, replicate);
            rims = rims
                + ring_z(
                    &format!("rtd_tracer_analyte_{analyte}_replicate_{replicate}_sample_rim"),
                    SAMPLE_RIM_D,
                    SAMPLE_WELL_D + 1.0,
                    4.0,
                )
                .translate(x, y, deck_top_z() + SAMPLE_Z + 2.0);
        }
    }
    rims
}

fn sample_analyte_lane_tags() -> Part {
    let ph = centered_cube("rtd_tracer_ph_sample_lane_tag_land", 54.0, 14.0, 6.0).translate(
        SAMPLE_CENTER.0 - SAMPLE_X / 2.0 + 38.0,
        SAMPLE_CENTER.1 + SAMPLE_PITCH_Y / 2.0,
        deck_top_z() + SAMPLE_Z + 3.0,
    );
    let osmo = centered_cube(
        "rtd_tracer_osmolality_sample_lane_tag_land",
        54.0,
        14.0,
        6.0,
    )
    .translate(
        SAMPLE_CENTER.0 - SAMPLE_X / 2.0 + 38.0,
        SAMPLE_CENTER.1 - SAMPLE_PITCH_Y / 2.0,
        deck_top_z() + SAMPLE_Z + 3.0,
    );
    ph + osmo
}

fn sample_chain_of_custody_strip() -> Part {
    centered_cube(
        "rtd_tracer_sample_chain_of_custody_strip",
        SAMPLE_X - 46.0,
        12.0,
        6.0,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 + 18.0,
        deck_top_z() + SAMPLE_Z + 3.0,
    )
}

fn barcode_run_token_and_decision_rail() -> Part {
    let body = centered_cube(
        "rtd_tracer_barcode_run_token_decision_rail_body",
        TRACE_DECISION_X,
        TRACE_DECISION_Y,
        TRACE_DECISION_Z,
    )
    .translate(
        TRACE_DECISION_CENTER.0,
        TRACE_DECISION_CENTER.1,
        place_z(TRACE_DECISION_Z),
    );

    body - decision_slot_cuts()
        + barcode_lands()
        + run_token_slots()
        + release_hold_reject_lane_dividers()
        + decision_lane_status_windows()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("rtd_tracer_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("rtd_tracer_barcode_land_{i}"), 42.0, 16.0, 5.0).translate(
                TRACE_DECISION_CENTER.0 - TRACE_DECISION_X / 2.0 + 38.0 + (i % 6) as f64 * 52.0,
                TRACE_DECISION_CENTER.1 + 30.0 - (i / 6) as f64 * 30.0,
                deck_top_z() + TRACE_DECISION_Z + 2.5,
            );
    }
    lands
}

fn run_token_slots() -> Part {
    let mut tokens = Part::empty("rtd_tracer_run_token_slots");
    for i in 0..RUN_TOKEN_SLOTS {
        let x = TRACE_DECISION_CENTER.0 - 12.0 + centered_index(i, RUN_TOKEN_SLOTS, 36.0);
        let y = TRACE_DECISION_CENTER.1 - TRACE_DECISION_Y / 2.0 + 24.0;
        let slot = centered_cube(format!("rtd_tracer_run_token_slot_{i}"), 26.0, 18.0, 6.0)
            .translate(x, y, deck_top_z() + TRACE_DECISION_Z + 3.0);
        let relief = centered_cube(
            format!("rtd_tracer_run_token_slot_{i}_finger_relief"),
            16.0,
            10.0,
            7.0,
        )
        .translate(x, y, deck_top_z() + TRACE_DECISION_Z + 3.4);
        tokens = tokens + (slot - relief);
    }
    tokens
}

fn decision_slot_cuts() -> Part {
    let mut cuts = Part::empty("rtd_tracer_release_hold_reject_decision_slot_cuts");
    for lane in 0..DECISION_LANES {
        let x = TRACE_DECISION_CENTER.0 + 190.0 + centered_index(lane, DECISION_LANES, 78.0);
        for slot in 0..DECISION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("rtd_tracer_decision_lane_{lane}_slot_{slot}_cut"),
                    DECISION_SLOT_X,
                    DECISION_SLOT_Y,
                    TRACE_DECISION_Z + 4.0,
                )
                .translate(
                    x,
                    TRACE_DECISION_CENTER.1 + centered_index(slot, DECISION_SLOTS_PER_LANE, 27.0),
                    place_z(TRACE_DECISION_Z),
                );
        }
    }
    cuts
}

fn release_hold_reject_lane_dividers() -> Part {
    let mut dividers = Part::empty("rtd_tracer_release_hold_reject_lane_dividers");
    for lane in 0..DECISION_LANES {
        let x = TRACE_DECISION_CENTER.0 + 190.0 + centered_index(lane, DECISION_LANES, 78.0);
        dividers = dividers
            + centered_cube(
                format!("rtd_tracer_decision_lane_{lane}_rear_status_backstop"),
                DECISION_SLOT_X + 12.0,
                6.0,
                22.0,
            )
            .translate(
                x,
                TRACE_DECISION_CENTER.1 + TRACE_DECISION_Y / 2.0 - 14.0,
                deck_top_z() + TRACE_DECISION_Z + 11.0,
            );
    }

    for divider in 0..(DECISION_LANES - 1) {
        dividers = dividers
            + centered_cube(
                format!("rtd_tracer_decision_lane_separator_{divider}"),
                6.0,
                TRACE_DECISION_Y - 24.0,
                28.0,
            )
            .translate(
                TRACE_DECISION_CENTER.0
                    + 190.0
                    + (centered_index(divider, DECISION_LANES, 78.0)
                        + centered_index(divider + 1, DECISION_LANES, 78.0))
                        / 2.0,
                TRACE_DECISION_CENTER.1,
                deck_top_z() + TRACE_DECISION_Z + 14.0,
            );
    }
    dividers
}

fn decision_lane_status_windows() -> Part {
    let mut windows = Part::empty("rtd_tracer_decision_lane_status_windows");
    for (lane, name) in ["release", "hold", "reject"].iter().enumerate() {
        windows = windows
            + centered_cube(
                format!("rtd_tracer_{name}_lane_status_window_land"),
                56.0,
                14.0,
                6.0,
            )
            .translate(
                TRACE_DECISION_CENTER.0 + 190.0 + centered_index(lane, DECISION_LANES, 78.0),
                TRACE_DECISION_CENTER.1 - TRACE_DECISION_Y / 2.0 + 14.0,
                deck_top_z() + TRACE_DECISION_Z + 3.0,
            );
    }
    windows
}

fn evidence_bridge() -> Part {
    let mut posts = Part::empty("rtd_tracer_evidence_bridge_posts");
    for (i, (x, y)) in evidence_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("rtd_tracer_evidence_bridge_post_{i}"),
                EVIDENCE_POST_X,
                EVIDENCE_POST_Y,
                EVIDENCE_CLEARANCE_Z,
            )
            .translate(*x, *y, deck_top_z() + EVIDENCE_CLEARANCE_Z / 2.0);
    }

    let front_beam = centered_cube(
        "rtd_tracer_evidence_bridge_front_beam",
        EVIDENCE_BRIDGE_SPAN_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        0.0,
        -EVIDENCE_BRIDGE_SPAN_Y / 2.0,
        deck_top_z() + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        "rtd_tracer_evidence_bridge_rear_beam",
        EVIDENCE_BRIDGE_SPAN_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        0.0,
        EVIDENCE_BRIDGE_SPAN_Y / 2.0,
        deck_top_z() + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    let cross_beam = centered_cube(
        "rtd_tracer_evidence_bridge_longitudinal_camera_beam",
        EVIDENCE_POST_X,
        EVIDENCE_BRIDGE_SPAN_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        deck_top_z() + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0,
    );

    posts + front_beam + rear_beam + cross_beam + camera_lands() + evidence_led_segments()
}

fn camera_lands() -> Part {
    let mut lands = Part::empty("rtd_tracer_evidence_camera_lands");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        lands = lands
            + centered_cube(
                format!("rtd_tracer_evidence_camera_land_{i}"),
                86.0,
                48.0,
                10.0,
            )
            .translate(
                centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0),
                if i % 2 == 0 { 135.0 } else { -135.0 },
                deck_top_z() + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 5.0,
            );
    }
    lands
}

fn evidence_led_segments() -> Part {
    let mut leds = Part::empty("rtd_tracer_evidence_led_segments");
    for i in 0..EVIDENCE_LED_SEGMENTS {
        leds = leds
            + centered_cube(
                format!("rtd_tracer_evidence_led_segment_{i}"),
                62.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LED_SEGMENTS, 110.0),
                0.0,
                deck_top_z() + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 3.0,
            );
    }
    leds
}

fn robot_service_keepout_gauges() -> Part {
    let robot = rectangular_frame(
        "rtd_tracer_robot_pick_place_keepout_envelope",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        8.0,
        8.0,
    )
    .translate(0.0, 0.0, deck_top_z() + ROBOT_KEEP_OUT_Z);

    let mut service = Part::empty("rtd_tracer_service_keepout_gauges");
    for (i, (name, x, y, sx, sy)) in [
        (
            "front_fraction_collection_drawer_pull",
            0.0,
            -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0,
            STATION_X - 240.0,
            FRONT_ROBOT_CLEARANCE,
        ),
        (
            "rear_coil_tubing_lift",
            0.0,
            STATION_Y / 2.0 - REAR_COIL_SERVICE_CLEARANCE / 2.0,
            STATION_X - 250.0,
            REAR_COIL_SERVICE_CLEARANCE,
        ),
        (
            "left_thermal_jacket_service",
            -STATION_X / 2.0 + LEFT_THERMAL_SERVICE_CLEARANCE / 2.0,
            -70.0,
            LEFT_THERMAL_SERVICE_CLEARANCE,
            440.0,
        ),
        (
            "right_fraction_pump_service",
            STATION_X / 2.0 - RIGHT_FRACTION_SERVICE_CLEARANCE / 2.0,
            40.0,
            RIGHT_FRACTION_SERVICE_CLEARANCE,
            560.0,
        ),
        (
            "top_evidence_bridge_clearance",
            0.0,
            0.0,
            EVIDENCE_BRIDGE_SPAN_X - 100.0,
            EVIDENCE_BRIDGE_SPAN_Y - 90.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let z = if i == 4 {
            deck_top_z() + TOP_EVIDENCE_CLEARANCE
        } else {
            deck_top_z() + 8.0
        };
        service = service
            + rectangular_frame(
                &format!("rtd_tracer_service_keepout_{name}"),
                sx,
                sy,
                6.0,
                6.0,
            )
            .translate(x, y, z);
    }

    robot + service + keepout_height_posts()
}

fn keepout_height_posts() -> Part {
    let mut posts = Part::empty("rtd_tracer_keepout_height_posts");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        posts = posts
            + centered_cylinder(format!("rtd_tracer_keepout_height_post_{i}"), 5.0, 44.0, 20)
                .translate(*x, *y, deck_top_z() + 22.0);
    }
    posts
}

fn ring_z(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, SEGMENTS)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, SEGMENTS)
}

fn low_dead_volume_port(name: &str, bore_d: f64) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), bore_d / 2.0 + 8.0, 12.0, SEGMENTS);
    let bore = centered_cylinder(format!("{name}_bore"), bore_d / 2.0, 14.0, 24);
    let key = centered_cube(format!("{name}_clocking_key"), bore_d + 18.0, 4.0, 5.0).translate(
        0.0,
        bore_d / 2.0 + 7.0,
        4.5,
    );
    boss - bore + key
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, rail: f64, z: f64) -> Part {
    let left = centered_cube(format!("{name}_left_rail"), rail, outer_y, z).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, outer_y, z).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    let front = centered_cube(format!("{name}_front_rail"), outer_x, rail, z).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), outer_x, rail, z).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        0.0,
    );
    left + right + front + rear
}

fn coupon_position(lane: usize, nest: usize) -> (f64, f64) {
    (
        COIL_CENTER.0 + centered_index(nest, COUPON_NESTS_PER_LANE, COUPON_PITCH_X),
        lane_y(lane),
    )
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (-300.0, STATION_Y / 2.0 - 58.0),
        (300.0, STATION_Y / 2.0 - 58.0),
        (-300.0, -STATION_Y / 2.0 + 58.0),
        (300.0, -STATION_Y / 2.0 + 58.0),
    ]
}

fn datum_points() -> [(f64, f64); ROBOT_DATUMS] {
    [
        (-650.0, -382.0),
        (650.0, -382.0),
        (-650.0, 382.0),
        (650.0, 382.0),
        (-235.0, 382.0),
        (235.0, 382.0),
        (-235.0, -382.0),
        (235.0, -382.0),
    ]
}

fn evidence_post_points() -> [(f64, f64); 4] {
    [
        (-EVIDENCE_BRIDGE_SPAN_X / 2.0, -EVIDENCE_BRIDGE_SPAN_Y / 2.0),
        (EVIDENCE_BRIDGE_SPAN_X / 2.0, -EVIDENCE_BRIDGE_SPAN_Y / 2.0),
        (-EVIDENCE_BRIDGE_SPAN_X / 2.0, EVIDENCE_BRIDGE_SPAN_Y / 2.0),
        (EVIDENCE_BRIDGE_SPAN_X / 2.0, EVIDENCE_BRIDGE_SPAN_Y / 2.0),
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
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 6);
    assert_eq!(UNITS, "millimeters");
    assert_eq!(GRID_STEP_MM, 2.0);

    assert_eq!(TRACER_PORTS, HOLD_LOOP_LANES);
    assert_eq!(INJECTION_MIXING_TEES, HOLD_LOOP_LANES);
    assert_eq!(COUPON_NESTS, HOLD_LOOP_LANES * COUPON_NESTS_PER_LANE);
    assert_eq!(FRACTION_WELLS, HOLD_LOOP_LANES * FRACTION_TIMEPOINTS);
    assert_eq!(
        THERMAL_WITNESS_WELLS,
        THERMAL_BLOCKS * THERMAL_WELLS_PER_BLOCK
    );
    assert_eq!(BUBBLE_WINDOWS, HOLD_LOOP_LANES);
    assert_eq!(DEAD_VOLUME_WINDOWS, HOLD_LOOP_LANES);
    assert_eq!(SAMPLE_POINTS, SAMPLE_ANALYTES * SAMPLE_REPLICATES);
    assert_eq!(SAMPLE_REPLICATES, HOLD_LOOP_LANES);
    assert_eq!(DECISION_SLOTS, DECISION_LANES * DECISION_SLOTS_PER_LANE);
    assert_eq!(SERVICE_KEEPOUTS, 5);
    assert_eq!(ROBOT_DATUMS, 8);

    assert!(LOOP_COIL_TARGET_ML >= 100.0);
    assert!(LOOP_LENGTH_EQUALITY_TOLERANCE_MM <= 0.05);
    assert!(FRACTION_CLEARANCE_D > FRACTION_WELL_D);
    assert!(FRACTION_RIM_D > FRACTION_CLEARANCE_D);
    assert!(INJECT_GUARD_D > INJECT_PORT_D);
    assert!(TOP_EVIDENCE_CLEARANCE > EVIDENCE_CLEARANCE_Z);
    assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
    assert!(REAR_COIL_SERVICE_CLEARANCE >= 240.0);
    assert!(LEFT_THERMAL_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_FRACTION_SERVICE_CLEARANCE >= 220.0);

    for required in [
        "hold_loop_coil_coupon_nests",
        "tracer_injection_ports",
        "timed_fraction_collection_wells",
        "recirculation_pump_bypass_gauge",
        "thermal_jacket_witness_blocks",
        "bubble_windows",
        "dead_volume_windows",
        "ph_sample_points",
        "osmolality_sample_points",
        "barcode_run_token_rail",
        "release_hold_reject_lanes",
        "evidence_bridge",
        "robot_service_keepouts",
    ] {
        assert!(
            REQUIRED_FEATURES.contains(&required),
            "missing required feature {required}"
        );
    }

    for rect in module_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station envelope",
            rect.name
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_containment_deck.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_hold_loop_coil_coupon_nests.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_tracer_injection_port_bank.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_timed_fraction_collection_well_rack.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_recirculation_pump_bypass_gauge.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_thermal_jacket_witness_blocks.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_bubble_dead_volume_window_bridge.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_ph_osmolality_sample_points.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_barcode_run_token_and_decision_rail.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_evidence_bridge.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_robot_service_keepout_gauges.stl",
                "output/closed_media_hold_loop_residence_time_distribution_tracer_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_rtd_station_features_are_represented() {
        for feature in [
            "hold_loop_coil_coupon_nests",
            "tracer_injection_ports",
            "timed_fraction_collection_wells",
            "recirculation_pump_bypass_gauge",
            "thermal_jacket_witness_blocks",
            "bubble_windows",
            "dead_volume_windows",
            "ph_sample_points",
            "osmolality_sample_points",
            "barcode_run_token_rail",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn rtd_counts_cover_each_lane_and_timepoint() {
        assert_eq!(HOLD_LOOP_LANES, 6);
        assert_eq!(TRACER_PORTS, HOLD_LOOP_LANES);
        assert_eq!(COUPON_NESTS, 18);
        assert_eq!(FRACTION_TIMEPOINTS, 6);
        assert_eq!(FRACTION_WELLS, 36);
        assert_eq!(BUBBLE_WINDOWS, HOLD_LOOP_LANES);
        assert_eq!(DEAD_VOLUME_WINDOWS, HOLD_LOOP_LANES);
        assert_eq!(SAMPLE_POINTS, 12);
        assert_eq!(SAMPLE_ANALYTES, 2);
    }

    #[test]
    fn fraction_grid_maps_every_lane_timepoint_pair() {
        let mut pockets = BTreeSet::new();
        for lane in 0..HOLD_LOOP_LANES {
            for timepoint in 0..FRACTION_TIMEPOINTS {
                pockets.insert((lane, timepoint));
                let (x, y) = fraction_position(lane, timepoint);
                assert!(x.abs() < STATION_X / 2.0);
                assert!(y.abs() < STATION_Y / 2.0);
            }
        }
        assert_eq!(pockets.len(), FRACTION_WELLS);
        assert!(FRACTION_CLEARANCE_D > FRACTION_WELL_D);
        assert!(FRACTION_RIM_D > FRACTION_CLEARANCE_D);
    }

    #[test]
    fn sampling_decision_and_traceability_counts_match_release_workflow() {
        assert_eq!(SAMPLE_REPLICATES, HOLD_LOOP_LANES);
        assert_eq!(SAMPLE_POINTS, SAMPLE_ANALYTES * SAMPLE_REPLICATES);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RUN_TOKEN_SLOTS, 8);
        assert_eq!(DECISION_LANES, 3);
        assert_eq!(DECISION_SLOTS, 12);
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
    }

    #[test]
    fn layout_fits_station_without_module_overlap() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_station(), "{rect:?} should fit station");
        }
        for (i, left) in rects.iter().enumerate() {
            for right in rects.iter().skip(i + 1) {
                assert!(
                    !left.overlaps_with_clearance(*right, LAYOUT_CLEARANCE),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn service_and_evidence_clearances_are_explicit() {
        assert_eq!(EVIDENCE_CAMERA_COUNT, 4);
        assert_eq!(EVIDENCE_LED_SEGMENTS, 10);
        assert_eq!(SERVICE_KEEPOUTS, 5);
        assert_eq!(ROBOT_DATUMS, 8);
        assert!(ROBOT_KEEP_OUT_X < STATION_X);
        assert!(ROBOT_KEEP_OUT_Y < STATION_Y);
        assert!(ROBOT_KEEP_OUT_Z > PUMP_Z);
        assert!(TOP_EVIDENCE_CLEARANCE > EVIDENCE_CLEARANCE_Z);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
    }

    #[test]
    fn fixture_limitations_and_reproducibility_controls_are_declared() {
        assert!(LIMITATIONS.contains(&"validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"not_a_sterile_wetted_product_design"));
        assert!(LIMITATIONS.contains(&"not_a_pressure_rated_loop"));
        assert!(LIMITATIONS.contains(&"not_an_analytical_acceptance_method"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"fixed_output_manifest"));
        assert!(REPRODUCIBILITY_CONTROLS.contains(&"no_random_inputs"));
        assert_eq!(
            PARAMETRIC_REVISION,
            "closed_media_hold_loop_residence_time_distribution_tracer_station_v1"
        );
        assert_eq!(UNITS, "millimeters");
        assert_eq!(GRID_STEP_MM, 2.0);
    }
}
