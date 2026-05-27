use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion pressure-sensor, bubble, and wetness crosstalk station.
//
// Intent:
// - Package a closed, article-free validation fixture that separates pressure
//   transducer drift/crosstalk from optical bubble and wetness sensor events.
// - Keep the twenty route identities, pressure pulses, controlled bubble slug
//   challenges, restriction references, wetness coupons, isolation valves, and
//   timestamped evidence capture mechanically visible on one leak-tray deck.
// - Represent sensor docks and challenge hardware as placeholders and witness
//   features only; purchased sensors, tubing, couplers, valves, and electronics
//   define actual wetted and electrical performance outside this CAD file.
//
// This is mechanical validation packaging only. It is not a pressure-rated
// wetted design, not a software-validation package, and not a release method.

const PREFIX: &str = "closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station";

const OUTPUTS: &[&str] = &[
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_base_leak_tray_deck.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_twenty_lane_perfusion_surrogate_comb.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_pressure_transducer_dock_bank.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_optical_bubble_sensor_fork_bank.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_wetness_probe_coupon_slots.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_controlled_bubble_slug_challenge_manifold.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_pressure_pulse_manifold.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_line_isolation_valve_bank.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_route_identity_token_rail.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_reference_restriction_coupon_rack.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_event_timestamp_beacon_camera_bridge.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_robot_service_keepouts.stl",
    "output/closed_perfusion_pressure_sensor_bubble_wetness_crosstalk_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_leak_tray_deck",
    "twenty_lane_perfusion_surrogate_comb",
    "pressure_transducer_dock_bank",
    "optical_bubble_sensor_fork_bank",
    "wetness_probe_coupon_slots",
    "controlled_bubble_slug_challenge_manifold",
    "pressure_pulse_manifold",
    "line_isolation_valve_bank",
    "route_identity_token_rail",
    "reference_restriction_coupon_rack",
    "release_hold_reject_lanes",
    "event_timestamp_beacon_camera_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: &[&str] = &[
    "mechanical_validation_packaging_only",
    "not_pressure_rated_wetted_design",
    "not_software_validation_package",
    "not_release_method",
    "purchased_sensors_valves_tubing_and_electronics_are_placeholders",
];

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 1060.0;
const BASE_Z: f64 = 24.0;
const LEAK_BASIN_X: f64 = STATION_X - 120.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 120.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_SLOTS: usize = 10;
const BASE_DATUMS: usize = 6;

const LANES: usize = 20;
const LANE_ROWS: usize = 4;
const LANE_COLS: usize = 5;
const LANE_PITCH_Y: f64 = 30.0;
const LANE_CENTER: (f64, f64) = (-64.0, 98.0);
const LANE_RUN_X: f64 = 920.0;
const LANE_BANK_X: f64 = LANE_RUN_X + 116.0;
const LANE_BANK_Y: f64 = (LANES as f64 - 1.0) * LANE_PITCH_Y + 84.0;
const LANE_PLATE_Z: f64 = 18.0;
const LANE_TRACE_W: f64 = 7.0;
const LANE_TRACE_Z: f64 = 6.0;
const LANE_INLET_X: f64 = LANE_CENTER.0 - LANE_RUN_X / 2.0;
const LANE_OUTLET_X: f64 = LANE_CENTER.0 + LANE_RUN_X / 2.0;
const LANE_PORT_D: f64 = 13.0;
const LANE_BORE_D: f64 = 5.8;
const PRESSURE_TAP_PAIRS: usize = LANES * 2;

const PRESSURE_DOCK_POS: (f64, f64) = (-64.0, -370.0);
const PRESSURE_DOCK_X: f64 = 960.0;
const PRESSURE_DOCK_Y: f64 = 118.0;
const PRESSURE_DOCK_Z: f64 = 46.0;
const PRESSURE_TRANSDUCERS: usize = LANES;
const PRESSURE_DOCK_COLS: usize = 10;
const PRESSURE_DOCK_ROWS: usize = 2;
const PRESSURE_PITCH_X: f64 = 86.0;
const PRESSURE_PITCH_Y: f64 = 42.0;
const PRESSURE_POCKET_X: f64 = 48.0;
const PRESSURE_POCKET_Y: f64 = 25.0;
const PRESSURE_ZERO_REFERENCES: usize = 4;
const DRIFT_WITNESS_TABS: usize = LANES;

const BUBBLE_FORK_POS: (f64, f64) = (-64.0, 98.0);
const BUBBLE_FORK_X: f64 = LANE_RUN_X + 80.0;
const BUBBLE_FORK_Y: f64 = LANE_BANK_Y + 40.0;
const BUBBLE_FORK_Z: f64 = 28.0;
const BUBBLE_SENSOR_FORKS: usize = LANES;
const FORK_WINDOW_X: f64 = 54.0;
const FORK_GAP_Y: f64 = 18.0;
const FORK_TINE_X: f64 = 12.0;
const BUBBLE_REFERENCE_SLUG_WINDOWS: usize = LANES;

const WETNESS_POS: (f64, f64) = (592.0, 98.0);
const WETNESS_X: f64 = 178.0;
const WETNESS_Y: f64 = LANE_BANK_Y + 42.0;
const WETNESS_Z: f64 = 34.0;
const WETNESS_COUPON_SLOTS: usize = LANES;
const WETNESS_COUPON_X: f64 = 76.0;
const WETNESS_COUPON_Y: f64 = 18.0;
const WETNESS_PROBE_PAIRS: usize = LANES;
const DRY_WET_REFERENCE_TABS: usize = 4;

const BUBBLE_MANIFOLD_POS: (f64, f64) = (-620.0, 98.0);
const BUBBLE_MANIFOLD_X: f64 = 210.0;
const BUBBLE_MANIFOLD_Y: f64 = LANE_BANK_Y + 58.0;
const BUBBLE_MANIFOLD_Z: f64 = 54.0;
const BUBBLE_SLUG_INJECTORS: usize = LANES;
const BUBBLE_SLUG_VOLUME_STEPS: usize = 4;
const SLUG_SEPTUM_D: f64 = 9.0;
const SLUG_VOLUME_UL: [usize; BUBBLE_SLUG_VOLUME_STEPS] = [5, 10, 25, 50];

const PULSE_POS: (f64, f64) = (-368.0, -156.0);
const PULSE_X: f64 = 430.0;
const PULSE_Y: f64 = 154.0;
const PULSE_Z: f64 = 58.0;
const PULSE_CHAMBERS: usize = 5;
const PULSE_OUTPUTS: usize = LANES;
const PULSE_LEVELS_KPA: [usize; PULSE_CHAMBERS] = [5, 10, 20, 35, 50];
const PULSE_CHAMBER_D: f64 = 34.0;

const VALVE_POS: (f64, f64) = (200.0, -156.0);
const VALVE_X: f64 = 650.0;
const VALVE_Y: f64 = 154.0;
const VALVE_Z: f64 = 46.0;
const ISOLATION_VALVES: usize = LANES;
const VALVE_ROWS: usize = 2;
const VALVE_COLS: usize = LANES / VALVE_ROWS;
const VALVE_PITCH_X: f64 = 60.0;
const VALVE_PITCH_Y: f64 = 52.0;
const VALVE_BODY_D: f64 = 24.0;

const TOKEN_POS: (f64, f64) = (-515.0, 430.0);
const TOKEN_X: f64 = 430.0;
const TOKEN_Y: f64 = 82.0;
const TOKEN_Z: f64 = 18.0;
const ROUTE_IDENTITY_TOKENS: usize = LANES;
const TOKEN_COLS: usize = 10;
const TOKEN_ROWS: usize = 2;
const TOKEN_D: f64 = 16.0;

const RESTRICTION_POS: (f64, f64) = (40.0, 430.0);
const RESTRICTION_X: f64 = 560.0;
const RESTRICTION_Y: f64 = 82.0;
const RESTRICTION_Z: f64 = 38.0;
const REFERENCE_RESTRICTION_COUPONS: usize = LANES;
const RESTRICTION_COUPON_X: f64 = 24.0;
const RESTRICTION_COUPON_Y: f64 = 42.0;
const RESTRICTION_GROUPS: usize = 5;

const DISPOSITION_POS: (f64, f64) = (484.0, -370.0);
const DISPOSITION_X: f64 = 420.0;
const DISPOSITION_Y: f64 = 118.0;
const DISPOSITION_Z: f64 = 30.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_NAMES: [&str; DISPOSITION_LANES] = ["release", "hold", "reject"];
const DISPOSITION_SLOTS_PER_LANE: usize = LANES;
const DISPOSITION_SLOT_X: f64 = 15.0;
const DISPOSITION_SLOT_Y: f64 = 20.0;
const DISPOSITION_PITCH_X: f64 = 18.0;
const DISPOSITION_PITCH_Y: f64 = 32.0;

const CAMERA_POS: (f64, f64) = (0.0, 98.0);
const CAMERA_X: f64 = 1240.0;
const CAMERA_Y: f64 = 56.0;
const CAMERA_Z: f64 = 228.0;
const CAMERA_COUNT: usize = 4;
const TIMESTAMP_BEACONS: usize = 6;
const CAMERA_PITCH_X: f64 = 330.0;
const CAMERA_WINDOW_X: f64 = 118.0;
const CAMERA_WINDOW_Y: f64 = 28.0;

const KEEP_OUT_X: f64 = 1460.0;
const KEEP_OUT_Y: f64 = 940.0;
const KEEP_OUT_Z: f64 = 170.0;
const KEEP_OUT_GAUGES: usize = 6;
const ROBOT_Z_CLEARANCE: f64 = 150.0;
const FRONT_SERVICE_CLEARANCE: f64 = 250.0;
const REAR_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_CHALLENGE_SERVICE_CLEARANCE: f64 = 165.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 170.0;
const CAMERA_LIFT_CLEARANCE: f64 = 260.0;

const LABEL_BAR_COUNT: usize = 8;
const FORBIDDEN_CLAIM_TERMS: &[&str] = &[
    "patient",
    "therapy",
    "therapeutic",
    "diagnosis",
    "diagnostic",
    "clinical",
    "sterility assurance",
    "biological release",
];

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_rim(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray_deck();
    export(&base, OUTPUTS[0]);

    let comb = twenty_lane_perfusion_surrogate_comb();
    export(&comb, OUTPUTS[1]);

    let pressure = pressure_transducer_dock_bank();
    export(&pressure, OUTPUTS[2]);

    let bubble = optical_bubble_sensor_fork_bank();
    export(&bubble, OUTPUTS[3]);

    let wetness = wetness_probe_coupon_slots();
    export(&wetness, OUTPUTS[4]);

    let slug = controlled_bubble_slug_challenge_manifold();
    export(&slug, OUTPUTS[5]);

    let pulse = pressure_pulse_manifold();
    export(&pulse, OUTPUTS[6]);

    let valves = line_isolation_valve_bank();
    export(&valves, OUTPUTS[7]);

    let tokens = route_identity_token_rail();
    export(&tokens, OUTPUTS[8]);

    let restrictions = reference_restriction_coupon_rack();
    export(&restrictions, OUTPUTS[9]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[10]);

    let evidence = event_timestamp_beacon_camera_bridge();
    export(&evidence, OUTPUTS[11]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[12]);

    let assembly = base
        + comb
        + pressure
        + bubble
        + wetness
        + slug
        + pulse
        + valves
        + tokens
        + restrictions
        + disposition
        + evidence
        + keepouts;
    export(&assembly, OUTPUTS[13]);

    println!(
        "Closed perfusion pressure/bubble/wetness crosstalk station: {STATION_X:.0}mm x {STATION_Y:.0}mm leak-tray deck, {LANES} surrogate lanes, {PRESSURE_TRANSDUCERS} pressure transducer docks, {BUBBLE_SENSOR_FORKS} optical bubble forks, and {WETNESS_COUPON_SLOTS} wetness coupon slots."
    );
    println!(
        "Challenge and evidence coverage: {BUBBLE_SLUG_INJECTORS} bubble slug injectors, {PULSE_CHAMBERS} pulse chambers feeding {PULSE_OUTPUTS} lane outputs, {ISOLATION_VALVES} isolation valves, {ROUTE_IDENTITY_TOKENS} route tokens, {REFERENCE_RESTRICTION_COUPONS} restriction coupons, {CAMERA_COUNT} evidence cameras, {TIMESTAMP_BEACONS} timestamp beacons, {} limitation markers, and {} STL outputs.",
        LIMITATIONS.len(),
        OUTPUTS.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(LANES, 20);
    assert_eq!(LANE_ROWS * LANE_COLS, LANES);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(PRESSURE_TRANSDUCERS, LANES);
    assert_eq!(BUBBLE_SENSOR_FORKS, LANES);
    assert_eq!(BUBBLE_REFERENCE_SLUG_WINDOWS, LANES);
    assert_eq!(WETNESS_COUPON_SLOTS, LANES);
    assert_eq!(BUBBLE_SLUG_INJECTORS, LANES);
    assert_eq!(PULSE_OUTPUTS, LANES);
    assert_eq!(ISOLATION_VALVES, LANES);
    assert_eq!(ROUTE_IDENTITY_TOKENS, LANES);
    assert_eq!(REFERENCE_RESTRICTION_COUPONS, LANES);
    assert_eq!(DISPOSITION_SLOTS_PER_LANE, LANES);
    assert_eq!(PRESSURE_TAP_PAIRS, LANES * 2);
    assert_eq!(BASE_DATUMS, 6);
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert!(LANE_BORE_D < LANE_PORT_D);
    assert!(CAMERA_Z > KEEP_OUT_Z);
    assert!(CAMERA_LIFT_CLEARANCE > CAMERA_Z);
    assert!(FRONT_SERVICE_CLEARANCE >= 240.0);
    assert!(REAR_SERVICE_CLEARANCE >= 200.0);
    assert!(LEFT_CHALLENGE_SERVICE_CLEARANCE >= 160.0);
    assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 160.0);
    assert_no_scope_claim_terms();

    for spec in module_specs() {
        assert!(spec.fits_inside_rim(), "{} exceeds station rim", spec.name);
    }
}

fn assert_no_scope_claim_terms() {
    let searchable = format!(
        "{} {} {} {}",
        REQUIRED_FEATURES.join(" "),
        LIMITATIONS.join(" "),
        OUTPUTS.join(" "),
        PREFIX
    )
    .to_lowercase();
    for forbidden in FORBIDDEN_CLAIM_TERMS {
        assert!(
            !searchable.contains(forbidden),
            "claim term should not be present: {forbidden}"
        );
    }
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(format!("{PREFIX}_base_plate"), STATION_X, STATION_Y, BASE_Z);
    let basin = centered_cube(
        format!("{PREFIX}_sumped_leak_basin_recess"),
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.8);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_right_tray_drain"),
        DRAIN_D / 2.0,
        60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 18.0), 0.0);

    deck - basin - drain
        + containment_rims()
        + mounting_slots()
        + datum_targets()
        + module_socket_rails()
}

fn containment_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(format!("{PREFIX}_left_leak_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        z,
    );
    let right = centered_cube(format!("{PREFIX}_right_leak_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(format!("{PREFIX}_rear_leak_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        z,
    );
    let front = centered_cube(
        format!("{PREFIX}_front_low_access_leak_rim"),
        STATION_X,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z * 0.31,
    );
    left + right + rear + front
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_m6_mounting_slot_bosses"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_mount_slot_boss_{i}"), 44.0, 24.0, 8.0)
            .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        let bore = centered_cylinder(format!("{PREFIX}_mount_slot_bore_{i}"), 3.5, 14.0, 24)
            .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        slots = slots + (boss - bore);
    }
    slots
}

fn datum_targets() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_station_datum_targets"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 72.0), -(STATION_Y / 2.0 - 64.0)),
        (STATION_X / 2.0 - 72.0, -(STATION_Y / 2.0 - 64.0)),
        (-(STATION_X / 2.0 - 72.0), STATION_Y / 2.0 - 64.0),
        (STATION_X / 2.0 - 72.0, STATION_Y / 2.0 - 64.0),
        (0.0, STATION_Y / 2.0 - 64.0),
        (0.0, -(STATION_Y / 2.0 - 64.0)),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_target(format!("{PREFIX}_datum_{i}")).translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    datums
}

fn module_socket_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_module_socket_rails"));
    for (i, footprint) in module_specs_without_keepout().iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_module_socket_{i}_front_rail"),
                footprint.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1 - footprint.y / 2.0 - 4.0,
                BASE_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("{PREFIX}_module_socket_{i}_rear_rail"),
                footprint.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1 + footprint.y / 2.0 + 4.0,
                BASE_Z / 2.0 + 4.0,
            );
    }
    rails
}

fn twenty_lane_perfusion_surrogate_comb() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_twenty_lane_surrogate_comb_backer"),
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_PLATE_Z,
    )
    .translate(
        LANE_CENTER.0,
        LANE_CENTER.1,
        BASE_Z / 2.0 + LANE_PLATE_Z / 2.0,
    );
    let relief = centered_cube(
        format!("{PREFIX}_twenty_lane_surrogate_comb_recess"),
        LANE_BANK_X - 52.0,
        LANE_BANK_Y - 46.0,
        5.0,
    )
    .translate(
        LANE_CENTER.0,
        LANE_CENTER.1,
        BASE_Z / 2.0 + LANE_PLATE_Z - 2.0,
    );

    plate - relief + surrogate_lane_traces() + pressure_tap_marks() + inlet_outlet_port_rings()
}

fn surrogate_lane_traces() -> Part {
    let mut traces = Part::empty(format!("{PREFIX}_twenty_surrogate_lane_traces"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + LANE_TRACE_Z / 2.0;
    for lane in 0..LANES {
        let y = lane_y(lane);
        let run = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_straight_perfusion_surrogate_trace"),
            LANE_RUN_X,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_CENTER.0, y, z);
        let upstream_witness = crosstalk_gap_witness(
            format!("{PREFIX}_lane_{lane:02}_upstream_separation_witness"),
            64.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_INLET_X + 168.0, y, z);
        let downstream_witness = crosstalk_gap_witness(
            format!("{PREFIX}_lane_{lane:02}_downstream_separation_witness"),
            64.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_OUTLET_X - 168.0, y, z);
        traces = traces + run + upstream_witness + downstream_witness;
    }
    traces
}

fn pressure_tap_marks() -> Part {
    let mut taps = Part::empty(format!("{PREFIX}_upstream_downstream_pressure_tap_marks"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + 4.0;
    for lane in 0..LANES {
        let y = lane_y(lane);
        for (side, x) in [LANE_INLET_X + 282.0, LANE_OUTLET_X - 282.0]
            .iter()
            .enumerate()
        {
            taps = taps
                + port_ring(
                    format!("{PREFIX}_lane_{lane:02}_pressure_tap_{side}"),
                    16.0,
                    5.0,
                    6.0,
                )
                .translate(*x, y, z);
        }
    }
    taps
}

fn inlet_outlet_port_rings() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_inlet_outlet_port_rings"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + 4.0;
    for lane in 0..LANES {
        let y = lane_y(lane);
        ports = ports
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_inlet_port_ring"),
                LANE_PORT_D,
                LANE_BORE_D,
                7.0,
            )
            .translate(LANE_INLET_X, y, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_outlet_port_ring"),
                LANE_PORT_D,
                LANE_BORE_D,
                7.0,
            )
            .translate(LANE_OUTLET_X, y, z);
    }
    ports
}

fn pressure_transducer_dock_bank() -> Part {
    let bank = centered_cube(
        format!("{PREFIX}_pressure_transducer_dock_bank_body"),
        PRESSURE_DOCK_X,
        PRESSURE_DOCK_Y,
        PRESSURE_DOCK_Z,
    )
    .translate(
        PRESSURE_DOCK_POS.0,
        PRESSURE_DOCK_POS.1,
        BASE_Z / 2.0 + PRESSURE_DOCK_Z / 2.0,
    );
    let label = csg_label_plaque(
        format!("{PREFIX}_pressure_drift_crosstalk_reference_label"),
        196.0,
        18.0,
        4.0,
        31,
    )
    .translate(
        PRESSURE_DOCK_POS.0 - PRESSURE_DOCK_X / 2.0 + 132.0,
        PRESSURE_DOCK_POS.1 + PRESSURE_DOCK_Y / 2.0 - 18.0,
        BASE_Z / 2.0 + PRESSURE_DOCK_Z + 2.0,
    );

    bank - pressure_transducer_pocket_cuts()
        + pressure_transducer_datum_tabs()
        + zero_reference_ports()
        + label
}

fn pressure_transducer_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_transducer_pocket_cuts"));
    for lane in 0..PRESSURE_TRANSDUCERS {
        let (x, y) = pressure_dock_position(lane);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_pressure_transducer_lane_{lane:02}_pocket_cut"),
                PRESSURE_POCKET_X,
                PRESSURE_POCKET_Y,
                PRESSURE_DOCK_Z + 8.0,
            )
            .translate(x, y, BASE_Z / 2.0 + PRESSURE_DOCK_Z / 2.0 - 8.0);
    }
    cuts
}

fn pressure_transducer_datum_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_pressure_transducer_drift_witness_tabs"));
    for lane in 0..DRIFT_WITNESS_TABS {
        let (x, y) = pressure_dock_position(lane);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_pressure_lane_{lane:02}_drift_witness_tab"),
                22.0,
                4.0,
                5.0,
            )
            .translate(
                x,
                y + PRESSURE_POCKET_Y / 2.0 + 6.0,
                BASE_Z / 2.0 + PRESSURE_DOCK_Z + 2.5,
            );
    }
    tabs
}

fn zero_reference_ports() -> Part {
    let mut refs = Part::empty(format!("{PREFIX}_pressure_zero_reference_ports"));
    for i in 0..PRESSURE_ZERO_REFERENCES {
        refs = refs
            + port_ring(
                format!("{PREFIX}_pressure_zero_reference_port_{i}"),
                24.0,
                10.0,
                6.0,
            )
            .translate(
                PRESSURE_DOCK_POS.0 + centered_index(i, PRESSURE_ZERO_REFERENCES, 48.0),
                PRESSURE_DOCK_POS.1 - PRESSURE_DOCK_Y / 2.0 + 17.0,
                BASE_Z / 2.0 + PRESSURE_DOCK_Z + 3.0,
            );
    }
    refs
}

fn optical_bubble_sensor_fork_bank() -> Part {
    let frame = rectangular_frame_xy(
        format!("{PREFIX}_optical_bubble_sensor_fork_alignment_frame"),
        BUBBLE_FORK_X,
        BUBBLE_FORK_Y,
        18.0,
        BUBBLE_FORK_Z,
    )
    .translate(
        BUBBLE_FORK_POS.0,
        BUBBLE_FORK_POS.1,
        BASE_Z / 2.0 + LANE_PLATE_Z + BUBBLE_FORK_Z / 2.0 + 20.0,
    );
    frame + bubble_fork_array()
}

fn bubble_fork_array() -> Part {
    let mut forks = Part::empty(format!("{PREFIX}_twenty_optical_bubble_sensor_forks"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + BUBBLE_FORK_Z + 22.0;
    for lane in 0..BUBBLE_SENSOR_FORKS {
        let y = lane_y(lane);
        let left_tine = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_bubble_fork_left_tine"),
            FORK_TINE_X,
            FORK_GAP_Y + 12.0,
            18.0,
        )
        .translate(BUBBLE_FORK_POS.0 - FORK_WINDOW_X / 2.0, y, z);
        let right_tine = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_bubble_fork_right_tine"),
            FORK_TINE_X,
            FORK_GAP_Y + 12.0,
            18.0,
        )
        .translate(BUBBLE_FORK_POS.0 + FORK_WINDOW_X / 2.0, y, z);
        let bridge = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_bubble_fork_back_bridge"),
            FORK_WINDOW_X + FORK_TINE_X,
            5.0,
            18.0,
        )
        .translate(BUBBLE_FORK_POS.0, y + FORK_GAP_Y / 2.0 + 6.0, z);
        let slug_window = port_ring(
            format!("{PREFIX}_lane_{lane:02}_bubble_slug_reference_window"),
            17.0,
            6.0,
            5.0,
        )
        .translate(BUBBLE_FORK_POS.0 + 88.0, y, z + 9.0);
        forks = forks + left_tine + right_tine + bridge + slug_window;
    }
    forks
}

fn wetness_probe_coupon_slots() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_wetness_probe_coupon_slot_rack"),
        WETNESS_X,
        WETNESS_Y,
        WETNESS_Z,
    )
    .translate(WETNESS_POS.0, WETNESS_POS.1, BASE_Z / 2.0 + WETNESS_Z / 2.0);
    rack - wetness_coupon_slot_cuts() + wetness_probe_pair_lands() + dry_wet_reference_tabs()
}

fn wetness_coupon_slot_cuts() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_wetness_coupon_slot_cuts"));
    for lane in 0..WETNESS_COUPON_SLOTS {
        let y = lane_y(lane);
        slots = slots
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_wetness_coupon_slot"),
                WETNESS_COUPON_X,
                WETNESS_COUPON_Y,
                WETNESS_Z + 8.0,
            )
            .translate(
                WETNESS_POS.0 - 26.0,
                y,
                BASE_Z / 2.0 + WETNESS_Z / 2.0 - 5.0,
            );
    }
    slots
}

fn wetness_probe_pair_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_wetness_probe_pair_lands"));
    for lane in 0..WETNESS_PROBE_PAIRS {
        let y = lane_y(lane);
        for side in 0..2 {
            lands = lands
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane:02}_wetness_probe_pair_{side}"),
                    3.2,
                    8.0,
                    20,
                )
                .translate(
                    WETNESS_POS.0 + 42.0 + side as f64 * 16.0,
                    y,
                    BASE_Z / 2.0 + WETNESS_Z + 4.0,
                );
        }
    }
    lands
}

fn dry_wet_reference_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_dry_wet_reference_tabs"));
    for i in 0..DRY_WET_REFERENCE_TABS {
        tabs = tabs
            + csg_label_plaque(
                format!("{PREFIX}_dry_wet_reference_tab_{i}"),
                28.0,
                14.0,
                4.0,
                50 + i,
            )
            .translate(
                WETNESS_POS.0 + centered_index(i, DRY_WET_REFERENCE_TABS, 34.0),
                WETNESS_POS.1 + WETNESS_Y / 2.0 - 18.0,
                BASE_Z / 2.0 + WETNESS_Z + 2.0,
            );
    }
    tabs
}

fn controlled_bubble_slug_challenge_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_controlled_bubble_slug_manifold_body"),
        BUBBLE_MANIFOLD_X,
        BUBBLE_MANIFOLD_Y,
        BUBBLE_MANIFOLD_Z,
    )
    .translate(
        BUBBLE_MANIFOLD_POS.0,
        BUBBLE_MANIFOLD_POS.1,
        BASE_Z / 2.0 + BUBBLE_MANIFOLD_Z / 2.0,
    );
    body + slug_injector_array() + slug_volume_step_lands() + bubble_manifold_header()
}

fn slug_injector_array() -> Part {
    let mut injectors = Part::empty(format!("{PREFIX}_twenty_controlled_bubble_slug_injectors"));
    for lane in 0..BUBBLE_SLUG_INJECTORS {
        let y = lane_y(lane);
        let septum = centered_cylinder(
            format!("{PREFIX}_lane_{lane:02}_bubble_slug_septum_land"),
            SLUG_SEPTUM_D / 2.0,
            8.0,
            24,
        )
        .translate(
            BUBBLE_MANIFOLD_POS.0 + 34.0,
            y,
            BASE_Z / 2.0 + BUBBLE_MANIFOLD_Z + 4.0,
        );
        let feed_tick = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_bubble_slug_feed_tick"),
            44.0,
            4.0,
            5.0,
        )
        .translate(
            BUBBLE_MANIFOLD_POS.0 + 4.0,
            y,
            BASE_Z / 2.0 + BUBBLE_MANIFOLD_Z + 6.0,
        );
        injectors = injectors + septum + feed_tick;
    }
    injectors
}

fn slug_volume_step_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_bubble_slug_volume_step_lands"));
    for (i, volume) in SLUG_VOLUME_UL.iter().enumerate() {
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_bubble_slug_{volume}_ul_step_land"),
                38.0,
                15.0,
                4.0,
                *volume,
            )
            .translate(
                BUBBLE_MANIFOLD_POS.0 - BUBBLE_MANIFOLD_X / 2.0 + 30.0,
                BUBBLE_MANIFOLD_POS.1 + centered_index(i, BUBBLE_SLUG_VOLUME_STEPS, 38.0),
                BASE_Z / 2.0 + BUBBLE_MANIFOLD_Z + 2.0,
            );
    }
    lands
}

fn bubble_manifold_header() -> Part {
    centered_cube(
        format!("{PREFIX}_bubble_slug_common_header_witness"),
        12.0,
        BUBBLE_MANIFOLD_Y - 70.0,
        7.0,
    )
    .translate(
        BUBBLE_MANIFOLD_POS.0 - 38.0,
        BUBBLE_MANIFOLD_POS.1,
        BASE_Z / 2.0 + BUBBLE_MANIFOLD_Z + 3.5,
    )
}

fn pressure_pulse_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_pressure_pulse_manifold_body"),
        PULSE_X,
        PULSE_Y,
        PULSE_Z,
    )
    .translate(PULSE_POS.0, PULSE_POS.1, BASE_Z / 2.0 + PULSE_Z / 2.0);
    body - pulse_chamber_cuts() + pulse_chamber_lands() + pulse_lane_output_ticks()
}

fn pulse_chamber_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_pulse_chamber_cuts"));
    for chamber in 0..PULSE_CHAMBERS {
        cuts = cuts
            + centered_cylinder(
                format!(
                    "{PREFIX}_pulse_level_{}_kpa_chamber_cut",
                    PULSE_LEVELS_KPA[chamber]
                ),
                PULSE_CHAMBER_D / 2.0,
                PULSE_Z + 8.0,
                32,
            )
            .translate(
                PULSE_POS.0 + centered_index(chamber, PULSE_CHAMBERS, 74.0),
                PULSE_POS.1 + 26.0,
                BASE_Z / 2.0 + PULSE_Z / 2.0,
            );
    }
    cuts
}

fn pulse_chamber_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pressure_pulse_chamber_level_lands"));
    for chamber in 0..PULSE_CHAMBERS {
        let x = PULSE_POS.0 + centered_index(chamber, PULSE_CHAMBERS, 74.0);
        lands = lands
            + port_ring(
                format!(
                    "{PREFIX}_pulse_level_{}_kpa_land",
                    PULSE_LEVELS_KPA[chamber]
                ),
                44.0,
                PULSE_CHAMBER_D,
                6.0,
            )
            .translate(x, PULSE_POS.1 + 26.0, BASE_Z / 2.0 + PULSE_Z + 3.0)
            + csg_label_plaque(
                format!(
                    "{PREFIX}_pulse_level_{}_kpa_label",
                    PULSE_LEVELS_KPA[chamber]
                ),
                44.0,
                13.0,
                3.0,
                PULSE_LEVELS_KPA[chamber],
            )
            .translate(x, PULSE_POS.1 - 42.0, BASE_Z / 2.0 + PULSE_Z + 1.5);
    }
    lands
}

fn pulse_lane_output_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_pressure_pulse_twenty_output_ticks"));
    for lane in 0..PULSE_OUTPUTS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_pressure_pulse_output_lane_{lane:02}"),
                14.0,
                5.0,
                5.0,
            )
            .translate(
                PULSE_POS.0 + centered_index(lane % 10, 10, 32.0),
                PULSE_POS.1 - 64.0 + (lane / 10) as f64 * 14.0,
                BASE_Z / 2.0 + PULSE_Z + 2.5,
            );
    }
    ticks
}

fn line_isolation_valve_bank() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_line_isolation_valve_bank_body"),
        VALVE_X,
        VALVE_Y,
        VALVE_Z,
    )
    .translate(VALVE_POS.0, VALVE_POS.1, BASE_Z / 2.0 + VALVE_Z / 2.0);
    body - valve_body_cuts() + valve_handle_lands() + valve_common_headers()
}

fn valve_body_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_line_isolation_valve_body_cuts"));
    for valve in 0..ISOLATION_VALVES {
        let (x, y) = valve_position(valve);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_lane_{valve:02}_isolation_valve_body_cut"),
                VALVE_BODY_D / 2.0,
                VALVE_Z + 8.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0 + VALVE_Z / 2.0);
    }
    cuts
}

fn valve_handle_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_line_isolation_valve_handle_lands"));
    for valve in 0..ISOLATION_VALVES {
        let (x, y) = valve_position(valve);
        let angle = if valve % 2 == 0 { 6.0 } else { -6.0 };
        lands = lands
            + centered_cube(
                format!("{PREFIX}_lane_{valve:02}_isolation_valve_handle_witness"),
                38.0,
                8.0,
                6.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(x, y, BASE_Z / 2.0 + VALVE_Z + 3.0);
    }
    lands
}

fn valve_common_headers() -> Part {
    let upper = centered_cube(
        format!("{PREFIX}_line_isolation_upper_common_header_witness"),
        VALVE_X - 60.0,
        5.0,
        6.0,
    )
    .translate(
        VALVE_POS.0,
        VALVE_POS.1 + VALVE_PITCH_Y / 2.0,
        BASE_Z / 2.0 + VALVE_Z + 3.0,
    );
    let lower = centered_cube(
        format!("{PREFIX}_line_isolation_lower_common_header_witness"),
        VALVE_X - 60.0,
        5.0,
        6.0,
    )
    .translate(
        VALVE_POS.0,
        VALVE_POS.1 - VALVE_PITCH_Y / 2.0,
        BASE_Z / 2.0 + VALVE_Z + 3.0,
    );
    upper + lower
}

fn route_identity_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_route_identity_token_rail_base"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z / 2.0 + TOKEN_Z / 2.0);
    rail + route_token_array() + route_scanner_window()
}

fn route_token_array() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_twenty_route_identity_tokens"));
    for token in 0..ROUTE_IDENTITY_TOKENS {
        let row = token / TOKEN_COLS;
        let col = token % TOKEN_COLS;
        tokens = tokens
            + port_ring(
                format!("{PREFIX}_route_identity_token_{token:02}"),
                TOKEN_D,
                TOKEN_D - 7.0,
                4.0,
            )
            .translate(
                TOKEN_POS.0 + centered_index(col, TOKEN_COLS, 38.0),
                TOKEN_POS.1 + centered_index(row, TOKEN_ROWS, 30.0),
                BASE_Z / 2.0 + TOKEN_Z + 2.0,
            );
    }
    tokens
}

fn route_scanner_window() -> Part {
    rectangular_frame_xy(
        format!("{PREFIX}_route_identity_scanner_window"),
        TOKEN_X - 40.0,
        20.0,
        5.0,
        5.0,
    )
    .translate(
        TOKEN_POS.0,
        TOKEN_POS.1 + TOKEN_Y / 2.0 - 14.0,
        BASE_Z / 2.0 + TOKEN_Z + 2.5,
    )
}

fn reference_restriction_coupon_rack() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_reference_restriction_coupon_rack_body"),
        RESTRICTION_X,
        RESTRICTION_Y,
        RESTRICTION_Z,
    )
    .translate(
        RESTRICTION_POS.0,
        RESTRICTION_POS.1,
        BASE_Z / 2.0 + RESTRICTION_Z / 2.0,
    );
    rack - restriction_coupon_slot_cuts() + restriction_group_labels()
}

fn restriction_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_reference_restriction_coupon_slot_cuts"));
    for coupon in 0..REFERENCE_RESTRICTION_COUPONS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_restriction_coupon_{coupon:02}_slot"),
                RESTRICTION_COUPON_X,
                RESTRICTION_COUPON_Y,
                RESTRICTION_Z + 8.0,
            )
            .translate(
                RESTRICTION_POS.0 + centered_index(coupon, REFERENCE_RESTRICTION_COUPONS, 25.0),
                RESTRICTION_POS.1 - 4.0,
                BASE_Z / 2.0 + RESTRICTION_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn restriction_group_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_reference_restriction_group_labels"));
    for group in 0..RESTRICTION_GROUPS {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_restriction_group_{group}_label"),
                74.0,
                15.0,
                3.0,
                60 + group,
            )
            .translate(
                RESTRICTION_POS.0 + centered_index(group, RESTRICTION_GROUPS, 104.0),
                RESTRICTION_POS.1 + RESTRICTION_Y / 2.0 - 15.0,
                BASE_Z / 2.0 + RESTRICTION_Z + 1.5,
            );
    }
    labels
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_panel"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1,
        BASE_Z / 2.0 + DISPOSITION_Z / 2.0,
    );
    panel + disposition_slot_array() + disposition_lane_labels()
}

fn disposition_slot_array() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_release_hold_reject_lane_slots"));
    for lane in 0..DISPOSITION_LANES {
        let y = DISPOSITION_POS.1 + centered_index(lane, DISPOSITION_LANES, DISPOSITION_PITCH_Y);
        let rail = centered_cube(
            format!("{PREFIX}_disposition_lane_{lane}_guide_rail"),
            DISPOSITION_X - 46.0,
            4.0,
            6.0,
        )
        .translate(DISPOSITION_POS.0, y, BASE_Z / 2.0 + DISPOSITION_Z + 3.0);
        slots = slots + rail;
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{lane}_slot_{slot:02}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    5.0,
                )
                .translate(
                    DISPOSITION_POS.0
                        + centered_index(slot, DISPOSITION_SLOTS_PER_LANE, DISPOSITION_PITCH_X),
                    y,
                    BASE_Z / 2.0 + DISPOSITION_Z + 7.0,
                );
        }
    }
    slots
}

fn disposition_lane_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_release_hold_reject_lane_labels"));
    for (lane, name) in DISPOSITION_NAMES.iter().enumerate() {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_{name}_lane_label"),
                66.0,
                15.0,
                4.0,
                80 + lane,
            )
            .translate(
                DISPOSITION_POS.0 - DISPOSITION_X / 2.0 + 48.0,
                DISPOSITION_POS.1 + centered_index(lane, DISPOSITION_LANES, DISPOSITION_PITCH_Y),
                BASE_Z / 2.0 + DISPOSITION_Z + 8.0,
            );
    }
    labels
}

fn event_timestamp_beacon_camera_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_event_camera_bridge_left_post"),
        34.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(
        CAMERA_POS.0 - CAMERA_X / 2.0 + 38.0,
        CAMERA_POS.1,
        BASE_Z + CAMERA_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_event_camera_bridge_right_post"),
        34.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(
        CAMERA_POS.0 + CAMERA_X / 2.0 - 38.0,
        CAMERA_POS.1,
        BASE_Z + CAMERA_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_event_camera_bridge_overhead_beam"),
        CAMERA_X,
        34.0,
        28.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1, BASE_Z + CAMERA_Z - 14.0);
    left_post + right_post + beam + camera_mounts_and_windows() + timestamp_beacon_lands()
}

fn camera_mounts_and_windows() -> Part {
    let mut mounts = Part::empty(format!("{PREFIX}_event_camera_mounts"));
    let mut windows = Part::empty(format!("{PREFIX}_event_camera_view_windows"));
    for camera in 0..CAMERA_COUNT {
        let x = CAMERA_POS.0 + centered_index(camera, CAMERA_COUNT, CAMERA_PITCH_X);
        mounts = mounts
            + centered_cube(
                format!("{PREFIX}_event_camera_{camera}_mount_land"),
                86.0,
                34.0,
                10.0,
            )
            .translate(x, CAMERA_POS.1, BASE_Z + CAMERA_Z - 36.0);
        windows = windows
            + centered_cube(
                format!("{PREFIX}_event_camera_{camera}_view_window"),
                CAMERA_WINDOW_X,
                CAMERA_WINDOW_Y,
                12.0,
            )
            .translate(x, CAMERA_POS.1, BASE_Z + CAMERA_Z - 14.0);
    }
    mounts - windows
}

fn timestamp_beacon_lands() -> Part {
    let mut beacons = Part::empty(format!("{PREFIX}_timestamp_beacon_lands"));
    for beacon in 0..TIMESTAMP_BEACONS {
        beacons = beacons
            + port_ring(
                format!("{PREFIX}_timestamp_beacon_{beacon}"),
                22.0,
                8.0,
                5.0,
            )
            .translate(
                CAMERA_POS.0 + centered_index(beacon, TIMESTAMP_BEACONS, 160.0),
                CAMERA_POS.1 - CAMERA_Y / 2.0 - 18.0,
                BASE_Z + CAMERA_Z - 52.0,
            );
    }
    beacons
}

fn robot_service_keepouts() -> Part {
    let frame = keepout_frame(
        format!("{PREFIX}_robot_keepout_outer_frame"),
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z + KEEP_OUT_Z / 2.0);
    frame + keepout_corner_posts() + service_keepout_gauges() + robot_z_clearance_gauge()
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_robot_keepout_corner_posts"));
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_robot_keepout_post_{i}"),
                12.0,
                12.0,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + KEEP_OUT_Z / 2.0);
    }
    posts
}

fn service_keepout_gauges() -> Part {
    let z = BASE_Z + 14.0;
    let front = gauge_bar(
        format!("{PREFIX}_front_service_clearance_gauge"),
        380.0,
        18.0,
        12.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 60.0, z);
    let rear = gauge_bar(
        format!("{PREFIX}_rear_service_clearance_gauge"),
        380.0,
        18.0,
        12.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 60.0, z);
    let left = gauge_bar(
        format!("{PREFIX}_left_bubble_challenge_service_gauge"),
        18.0,
        260.0,
        12.0,
    )
    .translate(-STATION_X / 2.0 + 62.0, BUBBLE_MANIFOLD_POS.1, z);
    let right = gauge_bar(
        format!("{PREFIX}_right_wetness_sensor_service_gauge"),
        18.0,
        260.0,
        12.0,
    )
    .translate(STATION_X / 2.0 - 62.0, WETNESS_POS.1, z);
    let pressure = gauge_bar(
        format!("{PREFIX}_pressure_transducer_service_gauge"),
        250.0,
        16.0,
        12.0,
    )
    .translate(
        PRESSURE_DOCK_POS.0,
        PRESSURE_DOCK_POS.1 - PRESSURE_DOCK_Y / 2.0 - 24.0,
        z,
    );
    let camera = gauge_bar(
        format!("{PREFIX}_camera_lift_service_gauge"),
        260.0,
        16.0,
        12.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1 + CAMERA_Y / 2.0 + 34.0, z);
    front + rear + left + right + pressure + camera
}

fn robot_z_clearance_gauge() -> Part {
    let post = centered_cube(
        format!("{PREFIX}_robot_z_clearance_reference_post"),
        28.0,
        28.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 56.0,
        -(KEEP_OUT_Y / 2.0 - 56.0),
        BASE_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    let plaque = csg_label_plaque(
        format!("{PREFIX}_robot_z_clearance_label"),
        78.0,
        18.0,
        4.0,
        ROBOT_Z_CLEARANCE as usize,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 56.0,
        -(KEEP_OUT_Y / 2.0 - 96.0),
        BASE_Z + 8.0,
    );
    post + plaque
}

fn lane_y(lane: usize) -> f64 {
    LANE_CENTER.1 + centered_index(lane, LANES, LANE_PITCH_Y)
}

fn pressure_dock_position(lane: usize) -> (f64, f64) {
    let row = lane / PRESSURE_DOCK_COLS;
    let col = lane % PRESSURE_DOCK_COLS;
    (
        PRESSURE_DOCK_POS.0 + centered_index(col, PRESSURE_DOCK_COLS, PRESSURE_PITCH_X),
        PRESSURE_DOCK_POS.1 + centered_index(row, PRESSURE_DOCK_ROWS, PRESSURE_PITCH_Y),
    )
}

fn valve_position(valve: usize) -> (f64, f64) {
    let row = valve / VALVE_COLS;
    let col = valve % VALVE_COLS;
    (
        VALVE_POS.0 + centered_index(col, VALVE_COLS, VALVE_PITCH_X),
        VALVE_POS.1 + centered_index(row, VALVE_ROWS, VALVE_PITCH_Y),
    )
}

fn module_specs() -> [Footprint; 12] {
    [
        Footprint {
            name: "twenty_lane_perfusion_surrogate_comb",
            center: LANE_CENTER,
            x: LANE_BANK_X,
            y: LANE_BANK_Y,
        },
        Footprint {
            name: "pressure_transducer_dock_bank",
            center: PRESSURE_DOCK_POS,
            x: PRESSURE_DOCK_X,
            y: PRESSURE_DOCK_Y,
        },
        Footprint {
            name: "optical_bubble_sensor_fork_bank",
            center: BUBBLE_FORK_POS,
            x: BUBBLE_FORK_X,
            y: BUBBLE_FORK_Y,
        },
        Footprint {
            name: "wetness_probe_coupon_slots",
            center: WETNESS_POS,
            x: WETNESS_X,
            y: WETNESS_Y,
        },
        Footprint {
            name: "controlled_bubble_slug_challenge_manifold",
            center: BUBBLE_MANIFOLD_POS,
            x: BUBBLE_MANIFOLD_X,
            y: BUBBLE_MANIFOLD_Y,
        },
        Footprint {
            name: "pressure_pulse_manifold",
            center: PULSE_POS,
            x: PULSE_X,
            y: PULSE_Y,
        },
        Footprint {
            name: "line_isolation_valve_bank",
            center: VALVE_POS,
            x: VALVE_X,
            y: VALVE_Y,
        },
        Footprint {
            name: "route_identity_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "reference_restriction_coupon_rack",
            center: RESTRICTION_POS,
            x: RESTRICTION_X,
            y: RESTRICTION_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Footprint {
            name: "event_timestamp_beacon_camera_bridge",
            center: CAMERA_POS,
            x: CAMERA_X,
            y: CAMERA_Y,
        },
        Footprint {
            name: "robot_service_keepouts",
            center: (0.0, 0.0),
            x: KEEP_OUT_X,
            y: KEEP_OUT_Y,
        },
    ]
}

fn module_specs_without_keepout() -> [Footprint; 11] {
    let specs = module_specs();
    [
        specs[0], specs[1], specs[2], specs[3], specs[4], specs[5], specs[6], specs[7], specs[8],
        specs[9], specs[10],
    ]
}

fn mount_points() -> [(f64, f64); MOUNT_SLOTS] {
    [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 56.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 56.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 56.0),
        (0.0, -(STATION_Y / 2.0 - 56.0)),
        (0.0, STATION_Y / 2.0 - 56.0),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
        (-STATION_X / 4.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 56.0),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn crosstalk_gap_witness(name: impl Into<String>, run_x: f64, width: f64, z: f64) -> Part {
    let name = name.into();
    let left = centered_cube(format!("{name}_left_gap_edge"), run_x / 2.0 - 8.0, width, z)
        .translate(-(run_x / 4.0 + 4.0), 0.0, 0.0);
    let right = centered_cube(
        format!("{name}_right_gap_edge"),
        run_x / 2.0 - 8.0,
        width,
        z,
    )
    .translate(run_x / 4.0 + 4.0, 0.0, 0.0);
    let bridge_tick = centered_cube(
        format!("{name}_event_alignment_tick"),
        4.0,
        width * 2.2,
        z + 2.0,
    );
    left + right + bridge_tick
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    rail: f64,
    z: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner_clearance"),
            outer_x - 2.0 * rail,
            outer_y - 2.0 * rail,
            z + 1.0,
        )
}

fn keepout_frame(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn port_ring(name: impl Into<String>, outer_d: f64, inner_d: f64, z: f64) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 28)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let ring = port_ring(format!("{name}_ring"), 22.0, 8.0, 4.0);
    let crosshair = centered_cube(format!("{name}_crosshair_x"), 26.0, 2.0, 3.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 26.0, 3.0);
    ring + crosshair
}

fn gauge_bar(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let bar = centered_cube(format!("{name}_bar"), x, y, z);
    let tick_a = centered_cube(format!("{name}_tick_a"), 14.0, 14.0, z + 6.0).translate(
        -x / 2.0,
        -y / 2.0,
        0.0,
    );
    let tick_b = centered_cube(format!("{name}_tick_b"), 14.0, 14.0, z + 6.0).translate(
        x / 2.0,
        y / 2.0,
        0.0,
    );
    bar + tick_a + tick_b
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_barcode_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 1.8 + ((seed + i) % 4) as f64 * 1.1;
        let height = (y - 5.0 - (i % 3) as f64).max(3.0);
        let x_offset = -x / 2.0 + 7.0 + i as f64 * ((x - 16.0) / LABEL_BAR_COUNT as f64);
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 1.2).translate(
                x_offset,
                0.0,
                z / 2.0 + 0.6,
            );
    }
    let corner = centered_cube(format!("{name}_orientation_corner"), 7.0, 3.0, z + 1.4).translate(
        x / 2.0 - 6.0,
        y / 2.0 - 4.0,
        z / 2.0 + 0.7,
    );
    sheet + bars + corner
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for output in OUTPUTS {
            assert!(output.starts_with(&format!("output/{PREFIX}_")), "{output}");
            assert!(output.ends_with(".stl"), "{output}");
        }
        assert!(OUTPUTS[13].ends_with("_assembly.stl"));
    }

    #[test]
    fn lane_count_and_lane_geometry_are_pinned() {
        assert_eq!(LANES, 20);
        assert_eq!(LANE_ROWS, 4);
        assert_eq!(LANE_COLS, 5);
        assert_eq!(LANE_ROWS * LANE_COLS, LANES);
        assert_eq!(lane_y(0) + lane_y(LANES - 1), 2.0 * LANE_CENTER.1);
        for lane in 1..LANES {
            assert_eq!(lane_y(lane) - lane_y(lane - 1), LANE_PITCH_Y);
        }
        assert!(LANE_BANK_Y > (LANES as f64 - 1.0) * LANE_PITCH_Y);
        assert_eq!(PRESSURE_TAP_PAIRS, 40);
    }

    #[test]
    fn sensor_and_coupon_counts_match_twenty_routes() {
        assert_eq!(PRESSURE_TRANSDUCERS, LANES);
        assert_eq!(
            PRESSURE_DOCK_ROWS * PRESSURE_DOCK_COLS,
            PRESSURE_TRANSDUCERS
        );
        assert_eq!(BUBBLE_SENSOR_FORKS, LANES);
        assert_eq!(BUBBLE_REFERENCE_SLUG_WINDOWS, LANES);
        assert_eq!(WETNESS_COUPON_SLOTS, LANES);
        assert_eq!(WETNESS_PROBE_PAIRS, LANES);
        assert_eq!(BUBBLE_SLUG_INJECTORS, LANES);
        assert_eq!(PULSE_OUTPUTS, LANES);
        assert_eq!(ISOLATION_VALVES, LANES);
        assert_eq!(ROUTE_IDENTITY_TOKENS, LANES);
        assert_eq!(TOKEN_ROWS * TOKEN_COLS, ROUTE_IDENTITY_TOKENS);
        assert_eq!(REFERENCE_RESTRICTION_COUPONS, LANES);
    }

    #[test]
    fn feature_list_covers_requested_station_scope() {
        for feature in [
            "base_leak_tray_deck",
            "twenty_lane_perfusion_surrogate_comb",
            "pressure_transducer_dock_bank",
            "optical_bubble_sensor_fork_bank",
            "wetness_probe_coupon_slots",
            "controlled_bubble_slug_challenge_manifold",
            "pressure_pulse_manifold",
            "line_isolation_valve_bank",
            "route_identity_token_rail",
            "reference_restriction_coupon_rack",
            "release_hold_reject_lanes",
            "event_timestamp_beacon_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 13);
    }

    #[test]
    fn station_bounds_and_keepouts_are_declared() {
        assert_design_constraints();
        for footprint in module_specs() {
            assert!(
                footprint.fits_inside_rim(),
                "{} footprint should remain inside leak tray rim",
                footprint.name
            );
        }
        assert!(KEEP_OUT_X < STATION_X);
        assert!(KEEP_OUT_Y < STATION_Y);
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(FRONT_SERVICE_CLEARANCE >= 240.0);
        assert!(REAR_SERVICE_CLEARANCE >= 200.0);
        assert!(LEFT_CHALLENGE_SERVICE_CLEARANCE >= 160.0);
        assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 160.0);
        assert!(CAMERA_LIFT_CLEARANCE >= 250.0);
    }

    #[test]
    fn evidence_disposition_and_challenge_capacity_are_pinned() {
        assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE, LANES);
        assert_eq!(DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE, 60);
        assert_eq!(PULSE_CHAMBERS, PULSE_LEVELS_KPA.len());
        assert_eq!(BUBBLE_SLUG_VOLUME_STEPS, SLUG_VOLUME_UL.len());
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(TIMESTAMP_BEACONS, 6);
        assert_eq!(BASE_DATUMS, 6);
        assert_eq!(MOUNT_SLOTS, 10);
    }

    #[test]
    fn no_biological_or_clinical_claim_terms_are_present() {
        assert_no_scope_claim_terms();
    }
}
