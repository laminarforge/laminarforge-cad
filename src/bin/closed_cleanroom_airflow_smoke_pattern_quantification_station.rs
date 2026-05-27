use std::{f64::consts::PI, fs};

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleanroom airflow smoke-pattern quantification station.
//
// This standalone generator models a no-cell cleanroom airflow visualization
// station for repeatable smoke-pattern studies across a closed transfer zone.
// It separates smoke generation from capture coupons, references each vane and
// flag to camera fiducials, records transfer-door events, and gives operators
// explicit release/hold/reject gates without opening the process boundary.
// Smoke chemistry, particle limits, video analysis, exposure recipes,
// calibration certificates, and final disposition criteria remain protocol
// controls outside this CAD model.

const PREFIX: &str = "closed_cleanroom_airflow_smoke_pattern_quantification_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_base_containment_deck.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_smoke_injection_manifold.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_calibrated_vane_flag_grid.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_particle_smoke_capture_coupon_rail.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_pressure_cascade_sensor_towers.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_transfer_door_event_markers.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_camera_fiducial_bridge.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_airflow_shadow_blocker_coupons.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_timed_recovery_token_lane.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_release_hold_reject_status_gates.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_barcode_custody_lands.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_robot_service_keepouts.stl",
    "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "smoke_injection_manifold",
    "calibrated_vane_flag_grid",
    "particle_smoke_capture_coupon_rail",
    "pressure_cascade_sensor_towers",
    "transfer_door_event_markers",
    "camera_fiducial_bridge",
    "airflow_shadow_blocker_coupons",
    "timed_recovery_token_lane",
    "release_hold_reject_status_gates",
    "barcode_custody_lands",
    "robot_service_keepouts",
    "assembly",
];

const DECK_X: f64 = 1680.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_SLOT_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const SMOKE_POS: (f64, f64) = (-500.0, 220.0);
const SMOKE_X: f64 = 420.0;
const SMOKE_Y: f64 = 220.0;
const SMOKE_Z: f64 = 34.0;
const SMOKE_SOURCE_CUP_COUNT: usize = 2;
const SMOKE_BRANCH_COUNT: usize = 4;
const SMOKE_PORTS_PER_BRANCH: usize = 5;
const SMOKE_PORT_COUNT: usize = SMOKE_BRANCH_COUNT * SMOKE_PORTS_PER_BRANCH;
const SMOKE_PORT_D: f64 = 7.2;
const MANIFOLD_MIXING_VANE_COUNT: usize = 8;
const SMOKE_CHECK_FLAG_COUNT: usize = 4;

const VANE_POS: (f64, f64) = (-500.0, -110.0);
const VANE_X: f64 = 500.0;
const VANE_Y: f64 = 320.0;
const VANE_Z: f64 = 26.0;
const VANE_COLS: usize = 5;
const VANE_ROWS: usize = 4;
const VANE_COUNT: usize = VANE_COLS * VANE_ROWS;
const VANE_PITCH_X: f64 = 84.0;
const VANE_PITCH_Y: f64 = 66.0;
const FLAG_COUNT: usize = VANE_COUNT;
const VANE_ANGLES_DEG: [f64; VANE_COUNT] = [
    -24.0, -12.0, 0.0, 12.0, 24.0, -18.0, -6.0, 6.0, 18.0, 30.0, -30.0, -18.0, -6.0, 6.0, 18.0,
    -24.0, -12.0, 0.0, 12.0, 24.0,
];

const COUPON_POS: (f64, f64) = (-430.0, -390.0);
const COUPON_X: f64 = 620.0;
const COUPON_Y: f64 = 130.0;
const COUPON_Z: f64 = 18.0;
const CAPTURE_COUPON_COUNT: usize = 12;
const UPSTREAM_COUPON_COUNT: usize = 6;
const DOWNSTREAM_COUPON_COUNT: usize = 6;
const COUPON_LAND_X: f64 = 46.0;
const COUPON_LAND_Y: f64 = 34.0;
const COUPON_PITCH_X: f64 = 46.0;
const COUPON_CAPTURE_DEPTH: f64 = 8.0;
const CAPTURE_SLOT_COUNT: usize = 9;

const SENSOR_POS: (f64, f64) = (90.0, 245.0);
const SENSOR_X: f64 = 360.0;
const SENSOR_Y: f64 = 300.0;
const SENSOR_BASE_Z: f64 = 24.0;
const PRESSURE_ZONE_COUNT: usize = 4;
const PRESSURE_SETPOINTS_PA: [f64; PRESSURE_ZONE_COUNT] = [30.0, 20.0, 10.0, 0.0];
const MIN_PRESSURE_STEP_PA: f64 = 10.0;
const SENSOR_TOWER_COUNT: usize = 8;
const SENSOR_TOWER_HEIGHTS: [f64; SENSOR_TOWER_COUNT] =
    [76.0, 96.0, 116.0, 136.0, 136.0, 116.0, 96.0, 76.0];
const SENSOR_PORTS_PER_TOWER: usize = 3;
const SENSOR_FLAG_COUNT: usize = SENSOR_TOWER_COUNT;
const DIFFERENTIAL_PRESSURE_PAIR_COUNT: usize = PRESSURE_ZONE_COUNT - 1;

const DOOR_POS: (f64, f64) = (500.0, 245.0);
const DOOR_X: f64 = 280.0;
const DOOR_Y: f64 = 260.0;
const DOOR_Z: f64 = 28.0;
const TRANSFER_DOOR_LEAF_COUNT: usize = 2;
const EVENT_MARKER_COUNT: usize = 8;
const INTERLOCK_PIN_COUNT: usize = 4;
const DOOR_GASKET_TICK_COUNT: usize = 12;
const EVENT_CARD_COUNT: usize = 4;

const BRIDGE_POS: (f64, f64) = (0.0, 430.0);
const BRIDGE_SPAN_X: f64 = 1380.0;
const BRIDGE_Y: f64 = 70.0;
const BRIDGE_ANCHOR_Z: f64 = 14.0;
const BRIDGE_POST_Z: f64 = 210.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_POD_COUNT: usize = 5;
const BRIDGE_FIDUCIAL_COUNT: usize = 9;
const LIGHT_BAR_COUNT: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 204.0;

const BLOCKER_POS: (f64, f64) = (90.0, -90.0);
const BLOCKER_X: f64 = 320.0;
const BLOCKER_Y: f64 = 200.0;
const BLOCKER_Z: f64 = 22.0;
const BLOCKER_COUPON_COUNT: usize = 9;
const BLOCKER_ROWS: usize = 3;
const BLOCKER_COLS: usize = 3;
const BLOCKER_HEIGHTS: [f64; BLOCKER_ROWS] = [18.0, 38.0, 64.0];

const TOKEN_POS: (f64, f64) = (470.0, -80.0);
const TOKEN_X: f64 = 340.0;
const TOKEN_Y: f64 = 110.0;
const TOKEN_Z: f64 = 14.0;
const RECOVERY_TOKEN_COUNT: usize = 10;
const RECOVERY_TIME_SECONDS: [usize; RECOVERY_TOKEN_COUNT] =
    [0, 15, 30, 45, 60, 90, 120, 180, 240, 300];
const TOKEN_SLOT_X: f64 = 24.0;
const TOKEN_SLOT_Y: f64 = 28.0;

const GATE_POS: (f64, f64) = (470.0, -250.0);
const GATE_X: f64 = 360.0;
const GATE_Y: f64 = 110.0;
const GATE_Z: f64 = 22.0;
const STATUS_GATE_COUNT: usize = 3;
const RELEASE_CAPACITY: usize = 6;
const HOLD_CAPACITY: usize = 3;
const REJECT_CAPACITY: usize = 1;

const TRACE_POS: (f64, f64) = (60.0, -390.0);
const TRACE_X: f64 = 320.0;
const TRACE_Y: f64 = 110.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 16;
const CUSTODY_SEAL_LAND_COUNT: usize = 6;
const PROTOCOL_CARD_LAND_COUNT: usize = 4;

const KEEP_OUT_X: f64 = 1540.0;
const KEEP_OUT_Y: f64 = 940.0;
const KEEP_OUT_Z: f64 = 8.0;
const ROBOT_FRONT_CLEARANCE: f64 = 340.0;
const SERVICE_REAR_CLEARANCE: f64 = 110.0;
const SIDE_SERVICE_CLEARANCE: f64 = 130.0;
const OVERHEAD_SERVICE_CLEARANCE_Z: f64 = 300.0;

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

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StatusGate {
    Release,
    Hold,
    Reject,
}

impl StatusGate {
    fn all() -> [StatusGate; STATUS_GATE_COUNT] {
        [StatusGate::Release, StatusGate::Hold, StatusGate::Reject]
    }

    fn index(self) -> usize {
        match self {
            StatusGate::Release => 0,
            StatusGate::Hold => 1,
            StatusGate::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            StatusGate::Release => "release",
            StatusGate::Hold => "hold",
            StatusGate::Reject => "reject",
        }
    }

    fn capacity(self) -> usize {
        match self {
            StatusGate::Release => RELEASE_CAPACITY,
            StatusGate::Hold => HOLD_CAPACITY,
            StatusGate::Reject => REJECT_CAPACITY,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let smoke = smoke_injection_manifold();
    export(OUTPUTS[1], &smoke);

    let vanes = calibrated_vane_flag_grid();
    export(OUTPUTS[2], &vanes);

    let coupons = particle_smoke_capture_coupon_rail();
    export(OUTPUTS[3], &coupons);

    let sensors = pressure_cascade_sensor_towers();
    export(OUTPUTS[4], &sensors);

    let door = transfer_door_event_markers();
    export(OUTPUTS[5], &door);

    let bridge = camera_fiducial_bridge();
    export(OUTPUTS[6], &bridge);

    let blockers = airflow_shadow_blocker_coupons();
    export(OUTPUTS[7], &blockers);

    let tokens = timed_recovery_token_lane();
    export(OUTPUTS[8], &tokens);

    let gates = release_hold_reject_status_gates();
    export(OUTPUTS[9], &gates);

    let trace = barcode_custody_lands();
    export(OUTPUTS[10], &trace);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + smoke.translate(SMOKE_POS.0, SMOKE_POS.1, DECK_Z)
        + vanes.translate(VANE_POS.0, VANE_POS.1, DECK_Z)
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, DECK_Z)
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, DECK_Z)
        + door.translate(DOOR_POS.0, DOOR_POS.1, DECK_Z)
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z)
        + blockers.translate(BLOCKER_POS.0, BLOCKER_POS.1, DECK_Z)
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, DECK_Z)
        + gates.translate(GATE_POS.0, GATE_POS.1, DECK_Z)
        + trace.translate(TRACE_POS.0, TRACE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cleanroom airflow smoke-pattern quantification station:");
    println!(
        "  Smoke:          {SMOKE_PORT_COUNT} calibrated ports, {SMOKE_BRANCH_COUNT} branches, {MANIFOLD_MIXING_VANE_COUNT} manifold mixing vanes"
    );
    println!(
        "  Quantification: {VANE_COUNT} vane cells, {FLAG_COUNT} flags, {BRIDGE_FIDUCIAL_COUNT} bridge fiducials, {CAMERA_POD_COUNT} camera pods"
    );
    println!(
        "  Capture:        {CAPTURE_COUPON_COUNT} capture coupons, {CAPTURE_SLOT_COUNT} recovery slots, {:.1}x capture/injection area",
        capture_to_injection_area_ratio()
    );
    println!(
        "  Cascade:        {PRESSURE_ZONE_COUNT} pressure zones, {:.0} Pa total drop, {:.0} Pa minimum step, {SENSOR_TOWER_COUNT} towers, {} sensor sample ports",
        pressure_total_drop_pa(),
        minimum_adjacent_pressure_step_pa(),
        pressure_sensor_port_count()
    );
    println!(
        "  Events:         {EVENT_MARKER_COUNT} transfer-door markers, {RECOVERY_TOKEN_COUNT} timed recovery tokens, {STATUS_GATE_COUNT} status gates"
    );
    println!(
        "  Custody:        {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_SEAL_LAND_COUNT} custody seal lands, {PROTOCOL_CARD_LAND_COUNT} protocol cards"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(PREFIX)));

    for feature in REQUIRED_FEATURES {
        assert!(
            OUTPUTS.iter().any(|path| path.contains(feature)),
            "missing output for required feature: {feature}"
        );
    }

    assert_eq!(
        SMOKE_PORT_COUNT,
        SMOKE_BRANCH_COUNT * SMOKE_PORTS_PER_BRANCH
    );
    assert_eq!(VANE_COUNT, VANE_COLS * VANE_ROWS);
    assert_eq!(FLAG_COUNT, VANE_COUNT);
    assert_eq!(
        CAPTURE_COUPON_COUNT,
        UPSTREAM_COUPON_COUNT + DOWNSTREAM_COUPON_COUNT
    );
    assert_eq!(PRESSURE_SETPOINTS_PA.len(), PRESSURE_ZONE_COUNT);
    assert_eq!(SENSOR_FLAG_COUNT, SENSOR_TOWER_COUNT);
    assert_eq!(
        pressure_sensor_port_count(),
        SENSOR_TOWER_COUNT * SENSOR_PORTS_PER_TOWER
    );
    assert_eq!(DIFFERENTIAL_PRESSURE_PAIR_COUNT, PRESSURE_ZONE_COUNT - 1);
    assert_eq!(TRANSFER_DOOR_LEAF_COUNT, 2);
    assert_eq!(EVENT_CARD_COUNT, PRESSURE_ZONE_COUNT);
    assert_eq!(BLOCKER_COUPON_COUNT, BLOCKER_ROWS * BLOCKER_COLS);
    assert_eq!(RECOVERY_TIME_SECONDS.len(), RECOVERY_TOKEN_COUNT);
    assert_eq!(StatusGate::all().len(), STATUS_GATE_COUNT);
    assert_eq!(total_status_gate_capacity(), RECOVERY_TOKEN_COUNT);

    for pair in PRESSURE_SETPOINTS_PA.windows(2) {
        assert!(
            pair[0] - pair[1] >= MIN_PRESSURE_STEP_PA,
            "pressure cascade step below minimum"
        );
    }
    for pair in RECOVERY_TIME_SECONDS.windows(2) {
        assert!(pair[1] > pair[0], "recovery times must be increasing");
    }

    assert!(capture_to_injection_area_ratio() > 3.5);
    assert!(maximum_vane_angle_abs_deg() <= 30.0);
    assert!(tallest_sensor_tower_height() + SENSOR_BASE_Z + 40.0 < CAMERA_CLEARANCE_Z);
    assert!(OVERHEAD_SERVICE_CLEARANCE_Z > CAMERA_CLEARANCE_Z + BRIDGE_BEAM_Z);

    let rects = layout_rects();
    for item in rects {
        assert!(item.fits_inside_deck(), "{} exceeds deck rim", item.name);
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn layout_rects() -> [Rect; 10] {
    [
        rect("smoke_injection_manifold", SMOKE_POS, SMOKE_X, SMOKE_Y),
        rect("calibrated_vane_flag_grid", VANE_POS, VANE_X, VANE_Y),
        rect(
            "particle_smoke_capture_coupon_rail",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        rect(
            "pressure_cascade_sensor_towers",
            SENSOR_POS,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect("transfer_door_event_markers", DOOR_POS, DOOR_X, DOOR_Y),
        rect(
            "camera_fiducial_bridge",
            BRIDGE_POS,
            BRIDGE_SPAN_X,
            BRIDGE_Y,
        ),
        rect(
            "airflow_shadow_blocker_coupons",
            BLOCKER_POS,
            BLOCKER_X,
            BLOCKER_Y,
        ),
        rect("timed_recovery_token_lane", TOKEN_POS, TOKEN_X, TOKEN_Y),
        rect("release_hold_reject_status_gates", GATE_POS, GATE_X, GATE_Y),
        rect("barcode_custody_lands", TRACE_POS, TRACE_X, TRACE_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "airflow_smoke_quantification_base_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "airflow_smoke_quantification_wipeable_basin_cut",
        DECK_X - 156.0,
        DECK_Y - 134.0,
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.3);
    let condensate_gutter = centered_cube(
        "airflow_smoke_quantification_front_smoke_condensate_gutter_cut",
        DECK_X - 240.0,
        16.0,
        9.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 58.0, DECK_Z - 4.0);
    let capture_drain = centered_cylinder(
        "airflow_smoke_quantification_capture_drain_cut",
        8.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 44.0, DECK_Z / 2.0);

    deck - basin - condensate_gutter - capture_drain - insert_sockets() - mounting_slots()
        + perimeter_lips()
        + flow_axis_reference_spines()
        + datum_targets()
        + bridge_anchor_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("airflow_smoke_quantification_insert_socket_cuts");
    for item in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("airflow_smoke_quantification_{}_socket_cut", item.name),
                item.x + 8.0,
                item.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                item.center.0,
                item.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airflow_smoke_quantification_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().iter().enumerate() {
        let round = centered_cylinder(
            format!("airflow_smoke_quantification_m6_mount_round_{i}"),
            3.4,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("airflow_smoke_quantification_m6_mount_slot_{i}"),
            30.0,
            7.4,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        slots = slots + round + slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-DECK_X / 2.0 + 66.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 2.0 - 66.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 2.0 + 66.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 66.0, DECK_Y / 2.0 - 62.0),
        (0.0, -DECK_Y / 2.0 + 62.0),
        (0.0, DECK_Y / 2.0 - 62.0),
        (-DECK_X / 2.0 + 66.0, 0.0),
        (DECK_X / 2.0 - 66.0, 0.0),
        (COUPON_POS.0, COUPON_POS.1 - COUPON_Y / 2.0 + 34.0),
        (DOOR_POS.0, DOOR_POS.1 + DOOR_Y / 2.0 - 36.0),
    ]
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        "airflow_smoke_quantification_rear_service_high_lip",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "airflow_smoke_quantification_left_clean_zone_lip",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "airflow_smoke_quantification_right_service_lip",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "airflow_smoke_quantification_front_robot_low_lip",
        DECK_X - 176.0,
        14.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 36.0, DECK_Z + 12.0);

    rear + left + right + front
}

fn flow_axis_reference_spines() -> Part {
    let smoke_to_vanes = centered_cube(
        "airflow_smoke_quantification_smoke_to_vane_axis_spine",
        12.0,
        320.0,
        18.0,
    )
    .translate(SMOKE_POS.0, 52.0, DECK_Z + 9.0);
    let vane_to_capture = centered_cube(
        "airflow_smoke_quantification_vane_to_capture_axis_spine",
        12.0,
        220.0,
        18.0,
    )
    .translate(COUPON_POS.0, -270.0, DECK_Z + 9.0);
    let transfer_to_tokens = centered_cube(
        "airflow_smoke_quantification_transfer_to_recovery_axis_spine",
        10.0,
        280.0,
        16.0,
    )
    .translate(TOKEN_POS.0, 78.0, DECK_Z + 8.0);

    smoke_to_vanes + vane_to_capture + transfer_to_tokens
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("airflow_smoke_quantification_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().iter().enumerate() {
        targets =
            targets
                + fiducial_target(&format!("airflow_smoke_quantification_datum_target_{i}"))
                    .translate(*x, *y, DECK_Z + 1.0);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-DECK_X / 2.0 + 100.0, -DECK_Y / 2.0 + 100.0),
        (DECK_X / 2.0 - 100.0, -DECK_Y / 2.0 + 100.0),
        (-DECK_X / 2.0 + 100.0, DECK_Y / 2.0 - 100.0),
        (DECK_X / 2.0 - 100.0, DECK_Y / 2.0 - 100.0),
    ]
}

fn bridge_anchor_lands() -> Part {
    let left = centered_cube(
        "airflow_smoke_quantification_left_camera_bridge_anchor_land",
        88.0,
        58.0,
        8.0,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, BRIDGE_POS.1, DECK_Z + 4.0);
    let right = centered_cube(
        "airflow_smoke_quantification_right_camera_bridge_anchor_land",
        88.0,
        58.0,
        8.0,
    )
    .translate(BRIDGE_SPAN_X / 2.0, BRIDGE_POS.1, DECK_Z + 4.0);
    left + right
}

fn smoke_injection_manifold() -> Part {
    let plate = centered_cube(
        "airflow_smoke_quantification_smoke_manifold_base_plate",
        SMOKE_X,
        SMOKE_Y,
        SMOKE_Z,
    )
    .translate(0.0, 0.0, SMOKE_Z / 2.0);
    let generator_pocket = centered_cube(
        "airflow_smoke_quantification_smoke_generator_pocket_cut",
        132.0,
        74.0,
        12.0,
    )
    .translate(-SMOKE_X / 2.0 + 92.0, -SMOKE_Y / 2.0 + 56.0, SMOKE_Z - 6.0);

    plate - generator_pocket
        + smoke_source_cups()
        + smoke_distribution_tubes()
        + smoke_port_array()
        + manifold_mixing_vanes()
        + smoke_check_flags()
        + smoke_coupon_priming_lands()
}

fn smoke_source_cups() -> Part {
    let mut cups = Part::empty("airflow_smoke_quantification_smoke_source_cups");
    for i in 0..SMOKE_SOURCE_CUP_COUNT {
        let y = centered_index(i, SMOKE_SOURCE_CUP_COUNT, 46.0);
        let cup = centered_cylinder(
            format!("airflow_smoke_quantification_smoke_source_cup_{i}"),
            22.0,
            22.0,
            44,
        )
        .translate(-SMOKE_X / 2.0 + 72.0, y, SMOKE_Z + 11.0);
        let bore = centered_cylinder(
            format!("airflow_smoke_quantification_smoke_source_cup_bore_{i}"),
            14.0,
            24.0,
            36,
        )
        .translate(-SMOKE_X / 2.0 + 72.0, y, SMOKE_Z + 11.0);
        cups = cups + (cup - bore);
    }
    cups
}

fn smoke_distribution_tubes() -> Part {
    let main = centered_cylinder(
        "airflow_smoke_quantification_smoke_main_distribution_tube",
        12.0,
        SMOKE_X - 126.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(38.0, 0.0, SMOKE_Z + 22.0);

    let mut branches = Part::empty("airflow_smoke_quantification_smoke_branch_tubes");
    for branch in 0..SMOKE_BRANCH_COUNT {
        let x = centered_index(branch, SMOKE_BRANCH_COUNT, 72.0) + 70.0;
        let branch_tube = centered_cylinder(
            format!("airflow_smoke_quantification_smoke_branch_tube_{branch}"),
            6.5,
            SMOKE_Y - 70.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, SMOKE_Z + 18.0);
        branches = branches + branch_tube;
    }

    main + branches
}

fn smoke_port_array() -> Part {
    let mut ports = Part::empty("airflow_smoke_quantification_smoke_port_array");
    for branch in 0..SMOKE_BRANCH_COUNT {
        let x = centered_index(branch, SMOKE_BRANCH_COUNT, 72.0) + 70.0;
        for port in 0..SMOKE_PORTS_PER_BRANCH {
            let y = centered_index(port, SMOKE_PORTS_PER_BRANCH, 32.0);
            let collar = centered_cylinder(
                format!("airflow_smoke_quantification_smoke_port_collar_{branch}_{port}"),
                13.0,
                8.0,
                32,
            )
            .translate(x, y, SMOKE_Z + 4.0);
            let bore = centered_cylinder(
                format!("airflow_smoke_quantification_smoke_port_bore_{branch}_{port}"),
                SMOKE_PORT_D / 2.0,
                10.0,
                24,
            )
            .translate(x, y, SMOKE_Z + 4.0);
            ports = ports + (collar - bore);
        }
    }
    ports
}

fn manifold_mixing_vanes() -> Part {
    let mut vanes = Part::empty("airflow_smoke_quantification_manifold_mixing_vanes");
    for i in 0..MANIFOLD_MIXING_VANE_COUNT {
        let vane = centered_cube(
            format!("airflow_smoke_quantification_manifold_mixing_vane_{i}"),
            6.0,
            34.0,
            30.0,
        )
        .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
        .translate(
            centered_index(i, MANIFOLD_MIXING_VANE_COUNT, 34.0) + 78.0,
            0.0,
            SMOKE_Z + 15.0,
        );
        vanes = vanes + vane;
    }
    vanes
}

fn smoke_check_flags() -> Part {
    let mut flags = Part::empty("airflow_smoke_quantification_smoke_check_flags");
    for i in 0..SMOKE_CHECK_FLAG_COUNT {
        flags = flags
            + centered_cube(
                format!("airflow_smoke_quantification_smoke_flow_check_flag_{i}"),
                38.0,
                8.0,
                26.0,
            )
            .translate(
                centered_index(i, SMOKE_CHECK_FLAG_COUNT, 74.0) + 70.0,
                SMOKE_Y / 2.0 - 26.0,
                SMOKE_Z + 13.0,
            );
    }
    flags
}

fn smoke_coupon_priming_lands() -> Part {
    let mut lands = Part::empty("airflow_smoke_quantification_smoke_coupon_priming_lands");
    for i in 0..SMOKE_BRANCH_COUNT {
        lands = lands
            + centered_cube(
                format!("airflow_smoke_quantification_smoke_branch_prime_land_{i}"),
                52.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(i, SMOKE_BRANCH_COUNT, 72.0) + 70.0,
                -SMOKE_Y / 2.0 + 26.0,
                SMOKE_Z + 2.5,
            );
    }
    lands
}

fn calibrated_vane_flag_grid() -> Part {
    let base = centered_cube(
        "airflow_smoke_quantification_vane_flag_grid_base",
        VANE_X,
        VANE_Y,
        VANE_Z,
    )
    .translate(0.0, 0.0, VANE_Z / 2.0);
    let recess = centered_cube(
        "airflow_smoke_quantification_vane_grid_floor_recess_cut",
        VANE_X - 50.0,
        VANE_Y - 44.0,
        8.0,
    )
    .translate(0.0, 0.0, VANE_Z - 4.0);

    base - recess + vane_cells() + vane_angle_flags() + grid_index_ticks() + flag_zero_line()
}

fn vane_cells() -> Part {
    let mut cells = Part::empty("airflow_smoke_quantification_calibrated_vane_cells");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let i = row * VANE_COLS + col;
            let (x, y) = vane_cell_center(col, row);
            let rail = rectangular_frame_xy(
                &format!("airflow_smoke_quantification_vane_cell_frame_{i}"),
                62.0,
                44.0,
                6.0,
                14.0,
            )
            .translate(x, y, VANE_Z + 7.0);
            let vane = centered_cube(
                format!("airflow_smoke_quantification_calibrated_vane_blade_{i}"),
                5.0,
                42.0,
                48.0,
            )
            .rotate(0.0, 0.0, VANE_ANGLES_DEG[i])
            .translate(x, y, VANE_Z + 24.0);
            cells = cells + rail + vane;
        }
    }
    cells
}

fn vane_angle_flags() -> Part {
    let mut flags = Part::empty("airflow_smoke_quantification_vane_angle_flags");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let i = row * VANE_COLS + col;
            let (x, y) = vane_cell_center(col, row);
            let flag_height = 10.0 + VANE_ANGLES_DEG[i].abs() / 2.0;
            let flag = centered_cube(
                format!("airflow_smoke_quantification_vane_angle_flag_{i}"),
                24.0,
                5.0,
                flag_height,
            )
            .translate(x + 24.0, y - 24.0, VANE_Z + flag_height / 2.0);
            flags = flags + flag;
        }
    }
    flags
}

fn grid_index_ticks() -> Part {
    let mut ticks = Part::empty("airflow_smoke_quantification_vane_grid_index_ticks");
    for col in 0..VANE_COLS {
        ticks = ticks
            + centered_cube(
                format!("airflow_smoke_quantification_vane_grid_column_tick_{col}"),
                34.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(col, VANE_COLS, VANE_PITCH_X),
                VANE_Y / 2.0 - 18.0,
                VANE_Z + 2.5,
            );
    }
    for row in 0..VANE_ROWS {
        ticks = ticks
            + centered_cube(
                format!("airflow_smoke_quantification_vane_grid_row_tick_{row}"),
                5.0,
                34.0,
                5.0,
            )
            .translate(
                -VANE_X / 2.0 + 18.0,
                centered_index(row, VANE_ROWS, VANE_PITCH_Y),
                VANE_Z + 2.5,
            );
    }
    ticks
}

fn flag_zero_line() -> Part {
    centered_cube(
        "airflow_smoke_quantification_vane_zero_degree_reference_line",
        VANE_X - 64.0,
        4.0,
        5.0,
    )
    .translate(0.0, 0.0, VANE_Z + 2.5)
}

fn vane_cell_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, VANE_COLS, VANE_PITCH_X),
        centered_index(row, VANE_ROWS, VANE_PITCH_Y),
    )
}

fn particle_smoke_capture_coupon_rail() -> Part {
    let rail = centered_cube(
        "airflow_smoke_quantification_particle_smoke_capture_coupon_rail",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0);
    let trough = centered_cube(
        "airflow_smoke_quantification_capture_coupon_center_trough_cut",
        COUPON_X - 64.0,
        42.0,
        COUPON_CAPTURE_DEPTH,
    )
    .translate(0.0, 0.0, COUPON_Z - COUPON_CAPTURE_DEPTH / 2.0);

    rail - trough + capture_coupon_lands() + capture_slot_lips() + upstream_downstream_tabs()
}

fn capture_coupon_lands() -> Part {
    let mut lands = Part::empty("airflow_smoke_quantification_capture_coupon_lands");
    for i in 0..CAPTURE_COUPON_COUNT {
        let x = centered_index(i, CAPTURE_COUPON_COUNT, COUPON_PITCH_X);
        let y = if i < UPSTREAM_COUPON_COUNT {
            -COUPON_Y / 2.0 + 32.0
        } else {
            COUPON_Y / 2.0 - 32.0
        };
        let land = centered_cube(
            format!("airflow_smoke_quantification_capture_coupon_land_{i}"),
            COUPON_LAND_X,
            COUPON_LAND_Y,
            8.0,
        )
        .translate(x, y, COUPON_Z + 4.0);
        let window = centered_cube(
            format!("airflow_smoke_quantification_capture_coupon_window_cut_{i}"),
            COUPON_LAND_X - 16.0,
            COUPON_LAND_Y - 14.0,
            10.0,
        )
        .translate(x, y, COUPON_Z + 4.0);
        lands = lands + (land - window);
    }
    lands
}

fn capture_slot_lips() -> Part {
    let mut lips = Part::empty("airflow_smoke_quantification_capture_recovery_slot_lips");
    for i in 0..CAPTURE_SLOT_COUNT {
        let x = centered_index(i, CAPTURE_SLOT_COUNT, 58.0);
        let lip = centered_cube(
            format!("airflow_smoke_quantification_capture_recovery_slot_lip_{i}"),
            44.0,
            7.0,
            8.0,
        )
        .translate(x, 0.0, COUPON_Z + 4.0);
        lips = lips + lip;
    }
    lips
}

fn upstream_downstream_tabs() -> Part {
    let upstream = centered_cube(
        "airflow_smoke_quantification_upstream_capture_label_land",
        150.0,
        16.0,
        5.0,
    )
    .translate(-160.0, -COUPON_Y / 2.0 + 14.0, COUPON_Z + 2.5);
    let downstream = centered_cube(
        "airflow_smoke_quantification_downstream_capture_label_land",
        150.0,
        16.0,
        5.0,
    )
    .translate(160.0, COUPON_Y / 2.0 - 14.0, COUPON_Z + 2.5);
    upstream + downstream
}

fn pressure_cascade_sensor_towers() -> Part {
    let base = centered_cube(
        "airflow_smoke_quantification_pressure_cascade_sensor_tower_base",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_BASE_Z,
    )
    .translate(0.0, 0.0, SENSOR_BASE_Z / 2.0);

    base + sensor_tower_masts()
        + differential_pressure_pair_rails()
        + pressure_zone_reference_steps()
        + sensor_cable_comb()
}

fn sensor_tower_masts() -> Part {
    let mut towers = Part::empty("airflow_smoke_quantification_pressure_sensor_towers");
    for i in 0..SENSOR_TOWER_COUNT {
        let col = i % 4;
        let row = i / 4;
        let x = centered_index(col, 4, 82.0);
        let y = centered_index(row, 2, 108.0);
        let height = SENSOR_TOWER_HEIGHTS[i];
        let mast = centered_cylinder(
            format!("airflow_smoke_quantification_pressure_sensor_tower_mast_{i}"),
            6.5,
            height,
            28,
        )
        .translate(x, y, SENSOR_BASE_Z + height / 2.0);
        let pod = centered_cube(
            format!("airflow_smoke_quantification_pressure_sensor_pod_{i}"),
            38.0,
            30.0,
            22.0,
        )
        .translate(x, y, SENSOR_BASE_Z + height + 11.0);
        let ports = sensor_sample_ports(i, x, y, height);
        let flag = centered_cube(
            format!("airflow_smoke_quantification_pressure_sensor_height_flag_{i}"),
            30.0,
            7.0,
            12.0,
        )
        .translate(x + 26.0, y, SENSOR_BASE_Z + height);
        towers = towers + mast + pod + ports + flag;
    }
    towers
}

fn sensor_sample_ports(index: usize, x: f64, y: f64, height: f64) -> Part {
    let mut ports = Part::empty(format!(
        "airflow_smoke_quantification_pressure_sensor_sample_ports_{index}"
    ));
    for port in 0..SENSOR_PORTS_PER_TOWER {
        let z = SENSOR_BASE_Z + height * (0.28 + port as f64 * 0.24);
        ports = ports
            + centered_cylinder(
                format!("airflow_smoke_quantification_sensor_sample_port_{index}_{port}"),
                4.5,
                16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - 17.0, z);
    }
    ports
}

fn differential_pressure_pair_rails() -> Part {
    let mut rails = Part::empty("airflow_smoke_quantification_differential_pressure_pair_rails");
    for i in 0..DIFFERENTIAL_PRESSURE_PAIR_COUNT {
        let x = centered_index(i, DIFFERENTIAL_PRESSURE_PAIR_COUNT, 86.0);
        let block = centered_cube(
            format!("airflow_smoke_quantification_dp_pair_bridge_{i}"),
            72.0,
            22.0,
            18.0,
        )
        .translate(x, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        let tap_a = centered_cylinder(
            format!("airflow_smoke_quantification_dp_pair_tap_a_{i}"),
            4.5,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 18.0, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        let tap_b = centered_cylinder(
            format!("airflow_smoke_quantification_dp_pair_tap_b_{i}"),
            4.5,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 18.0, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        rails = rails + (block - tap_a - tap_b);
    }
    rails
}

fn pressure_zone_reference_steps() -> Part {
    let mut steps = Part::empty("airflow_smoke_quantification_pressure_zone_reference_steps");
    for i in 0..PRESSURE_ZONE_COUNT {
        let x = centered_index(i, PRESSURE_ZONE_COUNT, 78.0);
        let h = 10.0 + PRESSURE_SETPOINTS_PA[i] * 0.8;
        steps = steps
            + centered_cube(
                format!("airflow_smoke_quantification_pressure_setpoint_step_{i}"),
                38.0,
                26.0,
                h,
            )
            .translate(x, -SENSOR_Y / 2.0 + 38.0, SENSOR_BASE_Z + h / 2.0);
    }
    steps
}

fn sensor_cable_comb() -> Part {
    let comb = centered_cube(
        "airflow_smoke_quantification_sensor_cable_comb",
        SENSOR_X - 54.0,
        30.0,
        18.0,
    )
    .translate(0.0, -SENSOR_Y / 2.0 + 16.0, SENSOR_BASE_Z + 9.0);
    let mut slots = Part::empty("airflow_smoke_quantification_sensor_cable_comb_slots");
    for i in 0..SENSOR_TOWER_COUNT {
        slots = slots
            + centered_cube(
                format!("airflow_smoke_quantification_sensor_cable_comb_slot_{i}"),
                10.0,
                34.0,
                20.0,
            )
            .translate(
                centered_index(i, SENSOR_TOWER_COUNT, 36.0),
                -SENSOR_Y / 2.0 + 16.0,
                SENSOR_BASE_Z + 9.0,
            );
    }
    comb - slots
}

fn transfer_door_event_markers() -> Part {
    let base = centered_cube(
        "airflow_smoke_quantification_transfer_door_event_marker_base",
        DOOR_X,
        DOOR_Y,
        DOOR_Z,
    )
    .translate(0.0, 0.0, DOOR_Z / 2.0);

    base + transfer_door_leaves()
        + transfer_event_marker_lane()
        + interlock_pin_bank()
        + door_gasket_witness_ticks()
        + event_protocol_cards()
}

fn transfer_door_leaves() -> Part {
    let mut leaves = Part::empty("airflow_smoke_quantification_transfer_door_leaves");
    for i in 0..TRANSFER_DOOR_LEAF_COUNT {
        let x = centered_index(i, TRANSFER_DOOR_LEAF_COUNT, 86.0) - 24.0;
        let leaf = centered_cube(
            format!("airflow_smoke_quantification_transfer_door_leaf_{i}"),
            78.0,
            118.0,
            16.0,
        )
        .translate(x, 0.0, DOOR_Z + 8.0);
        let gasket = rectangular_frame_xy(
            &format!("airflow_smoke_quantification_transfer_door_gasket_land_{i}"),
            66.0,
            104.0,
            8.0,
            8.0,
        )
        .translate(x, 0.0, DOOR_Z + 16.0);
        let handle = centered_cube(
            format!("airflow_smoke_quantification_transfer_door_handle_{i}"),
            10.0,
            46.0,
            16.0,
        )
        .translate(x + 24.0, 0.0, DOOR_Z + 26.0);
        leaves = leaves + leaf + gasket + handle;
    }
    leaves
}

fn transfer_event_marker_lane() -> Part {
    let mut markers = Part::empty("airflow_smoke_quantification_transfer_door_event_markers");
    for i in 0..EVENT_MARKER_COUNT {
        let x = centered_index(i, EVENT_MARKER_COUNT, 30.0);
        let marker = centered_cube(
            format!("airflow_smoke_quantification_transfer_event_marker_{i}"),
            22.0,
            18.0,
            4.0 + i as f64,
        )
        .translate(x, -DOOR_Y / 2.0 + 34.0, DOOR_Z + 2.0 + i as f64 / 2.0);
        markers = markers + marker;
    }
    markers
}

fn interlock_pin_bank() -> Part {
    let mut pins = Part::empty("airflow_smoke_quantification_transfer_interlock_pins");
    for i in 0..INTERLOCK_PIN_COUNT {
        let x = centered_index(i, INTERLOCK_PIN_COUNT, 42.0) - 48.0;
        let pin = centered_cylinder(
            format!("airflow_smoke_quantification_transfer_interlock_pin_{i}"),
            5.0,
            42.0,
            24,
        )
        .translate(x, DOOR_Y / 2.0 - 38.0, DOOR_Z + 21.0);
        let flag = centered_cube(
            format!("airflow_smoke_quantification_transfer_interlock_flag_{i}"),
            24.0,
            8.0,
            12.0,
        )
        .translate(x + 10.0, DOOR_Y / 2.0 - 38.0, DOOR_Z + 38.0);
        pins = pins + pin + flag;
    }
    pins
}

fn door_gasket_witness_ticks() -> Part {
    let mut ticks = Part::empty("airflow_smoke_quantification_door_gasket_witness_ticks");
    for i in 0..DOOR_GASKET_TICK_COUNT {
        ticks = ticks
            + centered_cube(
                format!("airflow_smoke_quantification_door_gasket_witness_tick_{i}"),
                11.0,
                5.0,
                7.0,
            )
            .translate(
                centered_index(i, DOOR_GASKET_TICK_COUNT, 20.0),
                DOOR_Y / 2.0 - 16.0,
                DOOR_Z + 3.5,
            );
    }
    ticks
}

fn event_protocol_cards() -> Part {
    let mut cards = Part::empty("airflow_smoke_quantification_transfer_event_protocol_cards");
    for i in 0..EVENT_CARD_COUNT {
        cards = cards
            + centered_cube(
                format!("airflow_smoke_quantification_transfer_event_card_land_{i}"),
                54.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, EVENT_CARD_COUNT, 62.0),
                -DOOR_Y / 2.0 + 72.0,
                DOOR_Z + 2.0,
            );
    }
    cards
}

fn camera_fiducial_bridge() -> Part {
    let left_anchor = bridge_anchor("left", -BRIDGE_SPAN_X / 2.0 + 52.0);
    let right_anchor = bridge_anchor("right", BRIDGE_SPAN_X / 2.0 - 52.0);
    let left_post = centered_cube(
        "airflow_smoke_quantification_camera_fiducial_bridge_left_post",
        30.0,
        30.0,
        BRIDGE_POST_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0 + 52.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "airflow_smoke_quantification_camera_fiducial_bridge_right_post",
        30.0,
        30.0,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0 - 52.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "airflow_smoke_quantification_camera_fiducial_bridge_crossbeam",
        BRIDGE_SPAN_X - 92.0,
        34.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_anchor
        + right_anchor
        + left_post
        + right_post
        + beam
        + camera_pods()
        + bridge_fiducial_targets()
        + bridge_light_bars()
}

fn bridge_anchor(label: &str, x: f64) -> Part {
    let pad = centered_cube(
        format!("airflow_smoke_quantification_camera_bridge_{label}_anchor_pad"),
        104.0,
        BRIDGE_Y,
        BRIDGE_ANCHOR_Z,
    );
    let screw_1 = centered_cylinder(
        format!("airflow_smoke_quantification_camera_bridge_{label}_front_screw_clearance"),
        3.2,
        BRIDGE_ANCHOR_Z + 2.0,
        24,
    )
    .translate(-24.0, -14.0, 0.0);
    let screw_2 = centered_cylinder(
        format!("airflow_smoke_quantification_camera_bridge_{label}_rear_screw_clearance"),
        3.2,
        BRIDGE_ANCHOR_Z + 2.0,
        24,
    )
    .translate(24.0, 14.0, 0.0);

    (pad - screw_1 - screw_2).translate(x, 0.0, 0.0)
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("airflow_smoke_quantification_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let x = centered_index(i, CAMERA_POD_COUNT, BRIDGE_SPAN_X / 5.6);
        let pod = centered_cube(
            format!("airflow_smoke_quantification_camera_pod_{i}"),
            58.0,
            38.0,
            22.0,
        )
        .translate(x, -BRIDGE_Y / 2.0 - 18.0, CAMERA_CLEARANCE_Z);
        let lens = centered_cylinder(
            format!("airflow_smoke_quantification_camera_lens_{i}"),
            12.0,
            8.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BRIDGE_Y / 2.0 - 40.0, CAMERA_CLEARANCE_Z);
        pods = pods + pod + lens;
    }
    pods
}

fn bridge_fiducial_targets() -> Part {
    let mut targets = Part::empty("airflow_smoke_quantification_bridge_fiducial_targets");
    for i in 0..BRIDGE_FIDUCIAL_COUNT {
        targets = targets
            + fiducial_target(&format!(
                "airflow_smoke_quantification_camera_bridge_fiducial_{i}"
            ))
            .translate(
                centered_index(i, BRIDGE_FIDUCIAL_COUNT, BRIDGE_SPAN_X / 10.0),
                BRIDGE_Y / 2.0 - 14.0,
                BRIDGE_ANCHOR_Z / 2.0 + BRIDGE_POST_Z + BRIDGE_BEAM_Z + 1.0,
            );
    }
    targets
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty("airflow_smoke_quantification_bridge_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("airflow_smoke_quantification_bridge_light_bar_{i}"),
                170.0,
                10.0,
                10.0,
            )
            .translate(
                centered_index(i, LIGHT_BAR_COUNT, BRIDGE_SPAN_X / 4.8),
                -BRIDGE_Y / 2.0 + 12.0,
                CAMERA_CLEARANCE_Z - 22.0,
            );
    }
    bars
}

fn airflow_shadow_blocker_coupons() -> Part {
    let tray = centered_cube(
        "airflow_smoke_quantification_airflow_shadow_blocker_coupon_tray",
        BLOCKER_X,
        BLOCKER_Y,
        BLOCKER_Z,
    )
    .translate(0.0, 0.0, BLOCKER_Z / 2.0);
    let pocket = centered_cube(
        "airflow_smoke_quantification_shadow_blocker_tray_recess_cut",
        BLOCKER_X - 46.0,
        BLOCKER_Y - 42.0,
        8.0,
    )
    .translate(0.0, 0.0, BLOCKER_Z - 4.0);

    tray - pocket + shadow_blocker_cards() + blocker_height_ticks() + blocker_parking_comb()
}

fn shadow_blocker_cards() -> Part {
    let mut cards = Part::empty("airflow_smoke_quantification_shadow_blocker_cards");
    for row in 0..BLOCKER_ROWS {
        for col in 0..BLOCKER_COLS {
            let i = row * BLOCKER_COLS + col;
            let x = centered_index(col, BLOCKER_COLS, 86.0);
            let y = centered_index(row, BLOCKER_ROWS, 52.0);
            let height = BLOCKER_HEIGHTS[row];
            let card = centered_cube(
                format!("airflow_smoke_quantification_airflow_shadow_blocker_coupon_{i}"),
                54.0,
                12.0,
                height,
            )
            .translate(x, y, BLOCKER_Z + height / 2.0);
            let notch = centered_cube(
                format!("airflow_smoke_quantification_shadow_blocker_index_notch_{i}"),
                14.0,
                14.0,
                12.0,
            )
            .translate(x + 16.0, y, BLOCKER_Z + height - 6.0);
            cards = cards + (card - notch);
        }
    }
    cards
}

fn blocker_height_ticks() -> Part {
    let mut ticks = Part::empty("airflow_smoke_quantification_shadow_blocker_height_ticks");
    for row in 0..BLOCKER_ROWS {
        ticks = ticks
            + centered_cube(
                format!("airflow_smoke_quantification_shadow_blocker_height_tick_{row}"),
                22.0,
                5.0,
                6.0,
            )
            .translate(
                -BLOCKER_X / 2.0 + 24.0,
                centered_index(row, BLOCKER_ROWS, 52.0),
                BLOCKER_Z + 3.0,
            );
    }
    ticks
}

fn blocker_parking_comb() -> Part {
    let comb = centered_cube(
        "airflow_smoke_quantification_shadow_blocker_parking_comb",
        BLOCKER_X - 66.0,
        24.0,
        14.0,
    )
    .translate(0.0, -BLOCKER_Y / 2.0 + 22.0, BLOCKER_Z + 7.0);
    let mut slots = Part::empty("airflow_smoke_quantification_shadow_blocker_parking_slots");
    for i in 0..BLOCKER_COUPON_COUNT {
        slots = slots
            + centered_cube(
                format!("airflow_smoke_quantification_shadow_blocker_parking_slot_{i}"),
                8.0,
                28.0,
                16.0,
            )
            .translate(
                centered_index(i, BLOCKER_COUPON_COUNT, 25.0),
                -BLOCKER_Y / 2.0 + 22.0,
                BLOCKER_Z + 7.0,
            );
    }
    comb - slots
}

fn timed_recovery_token_lane() -> Part {
    let lane = centered_cube(
        "airflow_smoke_quantification_timed_recovery_token_lane",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0);
    let trough = centered_cube(
        "airflow_smoke_quantification_recovery_token_trough_cut",
        TOKEN_X - 44.0,
        TOKEN_Y - 48.0,
        6.0,
    )
    .translate(0.0, 4.0, TOKEN_Z - 3.0);

    lane - trough + recovery_token_slots() + recovery_time_tick_lands() + recovery_reference_stop()
}

fn recovery_token_slots() -> Part {
    let mut slots = Part::empty("airflow_smoke_quantification_recovery_token_slots");
    for i in 0..RECOVERY_TOKEN_COUNT {
        let x = centered_index(i, RECOVERY_TOKEN_COUNT, 30.0);
        let slot = centered_cube(
            format!("airflow_smoke_quantification_recovery_token_slot_{i}"),
            TOKEN_SLOT_X,
            TOKEN_SLOT_Y,
            6.0,
        )
        .translate(x, 4.0, TOKEN_Z + 3.0);
        let token = centered_cylinder(
            format!("airflow_smoke_quantification_recovery_time_token_{i}"),
            9.0,
            5.0,
            28,
        )
        .translate(x, 4.0, TOKEN_Z + 8.5);
        slots = slots + slot + token;
    }
    slots
}

fn recovery_time_tick_lands() -> Part {
    let mut ticks = Part::empty("airflow_smoke_quantification_recovery_time_tick_lands");
    for i in 0..RECOVERY_TOKEN_COUNT {
        ticks = ticks
            + centered_cube(
                format!(
                    "airflow_smoke_quantification_recovery_time_tick_{}s",
                    RECOVERY_TIME_SECONDS[i]
                ),
                20.0,
                5.0,
                4.0,
            )
            .translate(
                centered_index(i, RECOVERY_TOKEN_COUNT, 30.0),
                -TOKEN_Y / 2.0 + 18.0,
                TOKEN_Z + 2.0,
            );
    }
    ticks
}

fn recovery_reference_stop() -> Part {
    centered_cube(
        "airflow_smoke_quantification_recovery_token_zero_reference_stop",
        12.0,
        TOKEN_Y - 28.0,
        28.0,
    )
    .translate(-TOKEN_X / 2.0 + 28.0, 0.0, TOKEN_Z + 14.0)
}

fn release_hold_reject_status_gates() -> Part {
    let base = centered_cube(
        "airflow_smoke_quantification_release_hold_reject_status_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(0.0, 0.0, GATE_Z / 2.0);

    base + status_gate_lanes() + status_gate_stop_blocks() + status_gate_token_capacity_tabs()
}

fn status_gate_lanes() -> Part {
    let mut lanes = Part::empty("airflow_smoke_quantification_status_gate_lanes");
    for gate in StatusGate::all() {
        let x = centered_index(gate.index(), STATUS_GATE_COUNT, 108.0);
        let lane = centered_cube(
            format!("airflow_smoke_quantification_{}_status_lane", gate.name()),
            92.0,
            58.0,
            8.0,
        )
        .translate(x, 0.0, GATE_Z + 4.0);
        let label = centered_cube(
            format!(
                "airflow_smoke_quantification_{}_status_label_land",
                gate.name()
            ),
            72.0,
            14.0,
            4.0,
        )
        .translate(x, -GATE_Y / 2.0 + 16.0, GATE_Z + 2.0);
        lanes = lanes + lane + label;
    }
    lanes
}

fn status_gate_stop_blocks() -> Part {
    let release = centered_cube(
        "airflow_smoke_quantification_release_status_gate_low_stop",
        92.0,
        10.0,
        28.0,
    )
    .translate(
        centered_index(0, STATUS_GATE_COUNT, 108.0),
        GATE_Y / 2.0 - 24.0,
        GATE_Z + 14.0,
    );
    let hold = centered_cube(
        "airflow_smoke_quantification_hold_status_gate_mid_stop",
        92.0,
        10.0,
        42.0,
    )
    .translate(
        centered_index(1, STATUS_GATE_COUNT, 108.0),
        GATE_Y / 2.0 - 24.0,
        GATE_Z + 21.0,
    );
    let reject = centered_cube(
        "airflow_smoke_quantification_reject_status_gate_high_stop",
        92.0,
        10.0,
        58.0,
    )
    .translate(
        centered_index(2, STATUS_GATE_COUNT, 108.0),
        GATE_Y / 2.0 - 24.0,
        GATE_Z + 29.0,
    );
    release + hold + reject
}

fn status_gate_token_capacity_tabs() -> Part {
    let mut tabs = Part::empty("airflow_smoke_quantification_status_gate_capacity_tabs");
    for gate in StatusGate::all() {
        let x = centered_index(gate.index(), STATUS_GATE_COUNT, 108.0);
        for i in 0..gate.capacity() {
            tabs = tabs
                + centered_cube(
                    format!(
                        "airflow_smoke_quantification_{}_capacity_token_tab_{i}",
                        gate.name()
                    ),
                    10.0,
                    8.0,
                    5.0,
                )
                .translate(
                    x + centered_index(i, gate.capacity(), 12.0),
                    GATE_Y / 2.0 - 42.0,
                    GATE_Z + 2.5,
                );
        }
    }
    tabs
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "airflow_smoke_quantification_barcode_custody_land_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);

    panel + barcode_lands() + custody_seal_lands() + protocol_card_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("airflow_smoke_quantification_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("airflow_smoke_quantification_barcode_land_{i}"),
                44.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i % 4, 4, 62.0),
                centered_index(i / 4, 4, 20.0) + 8.0,
                TRACE_Z + 2.0,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty("airflow_smoke_quantification_custody_seal_lands");
    for i in 0..CUSTODY_SEAL_LAND_COUNT {
        seals = seals
            + centered_cylinder(
                format!("airflow_smoke_quantification_custody_seal_land_{i}"),
                9.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LAND_COUNT, 38.0),
                -TRACE_Y / 2.0 + 16.0,
                TRACE_Z + 2.0,
            );
    }
    seals
}

fn protocol_card_lands() -> Part {
    let mut cards = Part::empty("airflow_smoke_quantification_protocol_card_lands");
    for i in 0..PROTOCOL_CARD_LAND_COUNT {
        cards = cards
            + centered_cube(
                format!("airflow_smoke_quantification_protocol_card_land_{i}"),
                62.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, PROTOCOL_CARD_LAND_COUNT, 72.0),
                TRACE_Y / 2.0 - 16.0,
                TRACE_Z + 2.0,
            );
    }
    cards
}

fn robot_service_keepouts() -> Part {
    let perimeter = rectangular_frame_xy(
        "airflow_smoke_quantification_robot_service_keepout_perimeter",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        8.0,
        KEEP_OUT_Z,
    );
    let front_robot = centered_cube(
        "airflow_smoke_quantification_front_robot_approach_keepout",
        KEEP_OUT_X - 160.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE, 0.0);
    let rear_service = centered_cube(
        "airflow_smoke_quantification_rear_service_sweep_keepout",
        KEEP_OUT_X - 180.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE, 0.0);
    let left_service = centered_cube(
        "airflow_smoke_quantification_left_service_probe_keepout",
        12.0,
        KEEP_OUT_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE, 0.0, 0.0);
    let right_robot = centered_cube(
        "airflow_smoke_quantification_right_robot_transfer_keepout",
        12.0,
        KEEP_OUT_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE, 0.0, 0.0);
    let overhead = centered_cube(
        "airflow_smoke_quantification_overhead_camera_service_keepout",
        180.0,
        120.0,
        10.0,
    )
    .translate(0.0, BRIDGE_POS.1, OVERHEAD_SERVICE_CLEARANCE_Z);

    perimeter + front_robot + rear_service + left_service + right_robot + overhead
}

fn rectangular_frame_xy(name: &str, outer_x: f64, outer_y: f64, rail_w: f64, z: f64) -> Part {
    let outer =
        centered_cube(format!("{name}_outer"), outer_x, outer_y, z).translate(0.0, 0.0, z / 2.0);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        outer_x - 2.0 * rail_w,
        outer_y - 2.0 * rail_w,
        z + 2.0,
    )
    .translate(0.0, 0.0, z / 2.0);
    outer - inner
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 15.0, 2.0, 40).translate(0.0, 0.0, 1.0);
    let inner =
        centered_cylinder(format!("{name}_inner_clearance"), 7.0, 3.0, 32).translate(0.0, 0.0, 1.0);
    let cross_x = centered_cube(format!("{name}_cross_x"), 30.0, 3.0, 2.4).translate(0.0, 0.0, 1.2);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 30.0, 2.4).translate(0.0, 0.0, 1.2);
    outer - inner + cross_x + cross_y
}

fn injection_open_area_mm2() -> f64 {
    SMOKE_PORT_COUNT as f64 * PI * (SMOKE_PORT_D / 2.0).powi(2)
}

fn capture_open_area_mm2() -> f64 {
    CAPTURE_SLOT_COUNT as f64 * 44.0 * 7.0
        + CAPTURE_COUPON_COUNT as f64 * (COUPON_LAND_X - 16.0) * (COUPON_LAND_Y - 14.0)
}

fn capture_to_injection_area_ratio() -> f64 {
    capture_open_area_mm2() / injection_open_area_mm2()
}

fn pressure_sensor_port_count() -> usize {
    SENSOR_TOWER_COUNT * SENSOR_PORTS_PER_TOWER
}

fn tallest_sensor_tower_height() -> f64 {
    SENSOR_TOWER_HEIGHTS.into_iter().fold(0.0, f64::max)
}

fn maximum_vane_angle_abs_deg() -> f64 {
    VANE_ANGLES_DEG
        .into_iter()
        .map(f64::abs)
        .fold(0.0, f64::max)
}

fn pressure_total_drop_pa() -> f64 {
    PRESSURE_SETPOINTS_PA[0] - PRESSURE_SETPOINTS_PA[PRESSURE_ZONE_COUNT - 1]
}

fn minimum_adjacent_pressure_step_pa() -> f64 {
    PRESSURE_SETPOINTS_PA
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .fold(f64::INFINITY, f64::min)
}

fn total_status_gate_capacity() -> usize {
    StatusGate::all()
        .into_iter()
        .map(StatusGate::capacity)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cleanroom_airflow_smoke_pattern_quantification_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_cleanroom_features_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for expected in [
            "smoke_injection_manifold",
            "calibrated_vane_flag_grid",
            "particle_smoke_capture_coupon_rail",
            "pressure_cascade_sensor_towers",
            "transfer_door_event_markers",
            "camera_fiducial_bridge",
            "airflow_shadow_blocker_coupons",
            "timed_recovery_token_lane",
            "release_hold_reject_status_gates",
            "barcode_custody_lands",
            "robot_service_keepouts",
            "assembly",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
            assert!(OUTPUTS.iter().any(|path| path.contains(expected)));
        }
    }

    #[test]
    fn layout_modules_fit_without_floor_overlap() {
        assert_design_constraints();
        let rects = layout_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} exceeds deck", rect.name);
        }
        for i in 0..rects.len() {
            for j in (i + 1)..rects.len() {
                assert!(
                    !rects[i].overlaps(rects[j]),
                    "{} overlaps {}",
                    rects[i].name,
                    rects[j].name
                );
            }
        }
    }

    #[test]
    fn smoke_capture_capacity_has_margin() {
        assert_eq!(SMOKE_PORT_COUNT, 20);
        assert_eq!(CAPTURE_COUPON_COUNT, 12);
        assert_eq!(UPSTREAM_COUPON_COUNT, 6);
        assert_eq!(DOWNSTREAM_COUPON_COUNT, 6);
        assert!(capture_to_injection_area_ratio() > 3.5);
    }

    #[test]
    fn vane_flag_grid_is_calibrated_and_counted() {
        assert_eq!(VANE_COUNT, 20);
        assert_eq!(FLAG_COUNT, VANE_COUNT);
        assert_eq!(VANE_ANGLES_DEG.len(), VANE_COUNT);
        assert!(maximum_vane_angle_abs_deg() <= 30.0);
    }

    #[test]
    fn pressure_cascade_and_sensor_ports_are_defensible() {
        assert_eq!(PRESSURE_SETPOINTS_PA, [30.0, 20.0, 10.0, 0.0]);
        assert_eq!(pressure_total_drop_pa(), 30.0);
        assert_eq!(minimum_adjacent_pressure_step_pa(), MIN_PRESSURE_STEP_PA);
        assert_eq!(SENSOR_TOWER_COUNT, 8);
        assert_eq!(pressure_sensor_port_count(), 24);
        assert_eq!(DIFFERENTIAL_PRESSURE_PAIR_COUNT, 3);
    }

    #[test]
    fn events_tokens_gates_and_custody_are_traceable() {
        assert_eq!(EVENT_MARKER_COUNT, 8);
        assert_eq!(INTERLOCK_PIN_COUNT, 4);
        assert_eq!(RECOVERY_TOKEN_COUNT, RECOVERY_TIME_SECONDS.len());
        assert_eq!(RECOVERY_TIME_SECONDS[0], 0);
        assert_eq!(RECOVERY_TIME_SECONDS[RECOVERY_TOKEN_COUNT - 1], 300);
        assert_eq!(STATUS_GATE_COUNT, 3);
        assert_eq!(total_status_gate_capacity(), RECOVERY_TOKEN_COUNT);
        assert_eq!(BARCODE_LAND_COUNT, 16);
        assert_eq!(CUSTODY_SEAL_LAND_COUNT, 6);
    }

    #[test]
    fn bridge_clearances_clear_sensor_towers() {
        assert!(tallest_sensor_tower_height() + SENSOR_BASE_Z + 40.0 < CAMERA_CLEARANCE_Z);
        assert!(OVERHEAD_SERVICE_CLEARANCE_Z > CAMERA_CLEARANCE_Z + BRIDGE_BEAM_Z);
        assert_eq!(CAMERA_POD_COUNT, 5);
        assert_eq!(BRIDGE_FIDUCIAL_COUNT, 9);
    }
}
