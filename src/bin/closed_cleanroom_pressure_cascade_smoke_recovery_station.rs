use std::{f64::consts::PI, fs};

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleanroom pressure cascade smoke recovery station.
//
// Design basis:
// - FDA aseptic processing guidance treats clean area separation, HEPA-filtered
//   air, and airflow visualization as core contamination-control evidence.
// - EU GMP Annex 1 emphasizes a contamination control strategy, barrier
//   technology/closed systems, isolator qualification, pressure differentials,
//   and ongoing environmental/process monitoring.
// - ISO 14644-3:2019 is the public cleanroom test-method reference for
//   controlled-condition performance tests, including air-cleanliness,
//   airflow-related checks, and recovery-style verification.
//
// This CAD is a validation fixture, not a pressure vessel or sterile boundary.
// Smoke/aerosol chemistry, particle limits, video evidence, recovery timing,
// leak challenge acceptance, and calibration certificates remain protocol
// controls. Geometry is sized to keep tracer introduction on the non-product
// side, visibly recover tracer through return/exhaust features, and expose
// reversible sample cassettes without opening the culture module boundary.

const PREFIX: &str = "closed_cleanroom_pressure_cascade_smoke_recovery_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_base_cascade_deck.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_pressure_step_plenums.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_smoke_injection_manifold.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_return_exhaust_recovery_path.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_sensor_tower_array.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_door_transfer_port_leak_challenge.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_service_line_penetration_bulkhead.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_sample_cassette_exposure_rack.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_recovery_filter_coupon_lane.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_barcode_protocol_lands.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_evidence_camera_bridge.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_robot_service_keepout_gauges.stl",
    "output/closed_cleanroom_pressure_cascade_smoke_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "pressure_step_plenums",
    "smoke_injection_manifold",
    "return_exhaust_recovery_path",
    "sensor_tower_array",
    "door_transfer_port_leak_challenge",
    "service_line_penetration_bulkhead",
    "sample_cassette_exposure_rack",
    "recovery_filter_coupon_lane",
    "barcode_protocol_lands",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
    "assembly",
];

const DECK_X: f64 = 1620.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_SLOT_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const PLENUM_POS: (f64, f64) = (-385.0, 70.0);
const PLENUM_X: f64 = 620.0;
const PLENUM_Y: f64 = 320.0;
const PLENUM_Z: f64 = 38.0;
const PRESSURE_STEP_COUNT: usize = 4;
const PRESSURE_SETPOINTS_PA: [f64; PRESSURE_STEP_COUNT] = [30.0, 20.0, 10.0, 0.0];
const MIN_PRESSURE_STEP_PA: f64 = 10.0;
const PLENUM_CELL_X: f64 = 136.0;
const PLENUM_CELL_Y: f64 = 244.0;
const PLENUM_PITCH_X: f64 = 146.0;
const PLENUM_WALL_W: f64 = 9.0;
const PLENUM_WALL_Z: f64 = 46.0;
const DIFFUSER_SLOTS_PER_PLENUM: usize = 5;
const TRANSITION_GATE_COUNT: usize = PRESSURE_STEP_COUNT - 1;
const PRESSURE_TAP_COUNT: usize = PRESSURE_STEP_COUNT * 2;

const SMOKE_POS: (f64, f64) = (300.0, 220.0);
const SMOKE_X: f64 = 420.0;
const SMOKE_Y: f64 = 220.0;
const SMOKE_Z: f64 = 34.0;
const SMOKE_BRANCH_COUNT: usize = PRESSURE_STEP_COUNT;
const SMOKE_PORTS_PER_BRANCH: usize = 4;
const SMOKE_PORT_COUNT: usize = SMOKE_BRANCH_COUNT * SMOKE_PORTS_PER_BRANCH;
const SMOKE_PORT_D: f64 = 7.6;
const SMOKE_SOURCE_CUP_COUNT: usize = 2;
const SMOKE_COUPON_SOCKET_COUNT: usize = 8;
const MIXING_VANE_COUNT: usize = 8;

const RECOVERY_POS: (f64, f64) = (390.0, -110.0);
const RECOVERY_X: f64 = 500.0;
const RECOVERY_Y: f64 = 230.0;
const RECOVERY_Z: f64 = 42.0;
const RETURN_SLOT_COUNT: usize = 10;
const RETURN_SLOT_X: f64 = 34.0;
const RETURN_SLOT_Y: f64 = 14.0;
const RECOVERY_TRAP_COUNT: usize = 3;
const HEPA_RECOVERY_COUPON_COUNT: usize = 3;
const EXHAUST_STACK_COUNT: usize = 2;

const SENSOR_POS: (f64, f64) = (-500.0, -260.0);
const SENSOR_X: f64 = 500.0;
const SENSOR_Y: f64 = 230.0;
const SENSOR_BASE_Z: f64 = 24.0;
const SENSOR_TOWER_COUNT: usize = 6;
const SENSOR_TOWER_HEIGHTS: [f64; SENSOR_TOWER_COUNT] = [72.0, 96.0, 120.0, 144.0, 120.0, 96.0];
const SENSOR_PORTS_PER_TOWER: usize = 2;
const SENSOR_FLAG_COUNT: usize = SENSOR_TOWER_COUNT;
const DIFFERENTIAL_PRESSURE_PAIR_COUNT: usize = PRESSURE_STEP_COUNT - 1;
const PARTICLE_SMOKE_REFERENCE_COUNT: usize = 3;

const DOOR_POS: (f64, f64) = (-45.0, -210.0);
const DOOR_X: f64 = 280.0;
const DOOR_Y: f64 = 190.0;
const DOOR_Z: f64 = 32.0;
const DOOR_LEAF_COUNT: usize = 2;
const TRANSFER_PORT_COUNT: usize = 1;
const LEAK_SHIM_COUNT: usize = 6;
const INTERLOCK_PIN_COUNT: usize = 4;
const DOOR_GASKET_W: f64 = 10.0;
const TRANSFER_PORT_OD: f64 = 92.0;
const TRANSFER_PORT_ID: f64 = 58.0;

const SERVICE_POS: (f64, f64) = (635.0, 210.0);
const SERVICE_X: f64 = 210.0;
const SERVICE_Y: f64 = 300.0;
const SERVICE_PANEL_Z: f64 = 250.0;
const SERVICE_PENETRATION_COUNT: usize = 8;
const SERVICE_ROWS: usize = 2;
const SERVICE_COLS: usize = 4;
const SERVICE_PORT_D: f64 = 14.0;
const SERVICE_COLLAR_D: f64 = 32.0;
const SERVICE_DRIP_TRAY_COUNT: usize = 2;
const SERVICE_STRAIN_RELIEF_SLOTS: usize = SERVICE_PENETRATION_COUNT;

const SAMPLE_POS: (f64, f64) = (-385.0, 350.0);
const SAMPLE_X: f64 = 620.0;
const SAMPLE_Y: f64 = 190.0;
const SAMPLE_Z: f64 = 24.0;
const SAMPLE_COLS: usize = 4;
const SAMPLE_ROWS: usize = 2;
const SAMPLE_CASSETTE_COUNT: usize = SAMPLE_COLS * SAMPLE_ROWS;
const SAMPLE_CASSETTE_X: f64 = REVC_CHIP_LENGTH + 22.0;
const SAMPLE_CASSETTE_Y: f64 = REVC_CHIP_WIDTH + 14.0;
const SAMPLE_CASSETTE_Z: f64 = REVC_TOTAL_HEIGHT + 10.0;
const SAMPLE_PITCH_X: f64 = 150.0;
const SAMPLE_PITCH_Y: f64 = 104.0;
const SAMPLE_WITNESS_STRIP_COUNT: usize = SAMPLE_CASSETTE_COUNT;

const FILTER_POS: (f64, f64) = (390.0, -360.0);
const FILTER_X: f64 = 500.0;
const FILTER_Y: f64 = 130.0;
const FILTER_Z: f64 = 18.0;
const FILTER_COUPON_COUNT: usize = 6;
const FILTER_COUPON_D: f64 = 38.0;
const FILTER_UPSTREAM_COUNT: usize = 3;
const FILTER_DOWNSTREAM_COUNT: usize = 3;

const TRACE_POS: (f64, f64) = (-80.0, -410.0);
const TRACE_X: f64 = 320.0;
const TRACE_Y: f64 = 80.0;
const TRACE_Z: f64 = 10.0;
const BARCODE_LAND_COUNT: usize = 8;
const PROTOCOL_CARD_COUNT: usize = 3;
const WITNESS_SEAL_LAND_COUNT: usize = 4;

const BRIDGE_POS: (f64, f64) = (0.0, 0.0);
const BRIDGE_SPAN_X: f64 = 1340.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_UNDERSIDE_Z: f64 = 250.0;
const BRIDGE_BEAM_Z: f64 = 26.0;
const CAMERA_POD_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 4;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 8.0;
const KEEP_OUT_FRAME_COUNT: usize = 4;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 340.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 220.0;
const RIGHT_SERVICE_WITHDRAWAL_X: f64 = 250.0;
const TOP_FILTER_LIFT_CLEARANCE_Z: f64 = 330.0;

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
enum CascadeZone {
    GradeAProcess,
    GradeBBuffer,
    GradeCTransfer,
    GradeDService,
}

impl CascadeZone {
    fn all() -> [CascadeZone; PRESSURE_STEP_COUNT] {
        [
            CascadeZone::GradeAProcess,
            CascadeZone::GradeBBuffer,
            CascadeZone::GradeCTransfer,
            CascadeZone::GradeDService,
        ]
    }

    fn index(self) -> usize {
        match self {
            CascadeZone::GradeAProcess => 0,
            CascadeZone::GradeBBuffer => 1,
            CascadeZone::GradeCTransfer => 2,
            CascadeZone::GradeDService => 3,
        }
    }

    fn label(self) -> &'static str {
        match self {
            CascadeZone::GradeAProcess => "grade_a_process",
            CascadeZone::GradeBBuffer => "grade_b_buffer",
            CascadeZone::GradeCTransfer => "grade_c_transfer",
            CascadeZone::GradeDService => "grade_d_service",
        }
    }

    fn setpoint_pa(self) -> f64 {
        PRESSURE_SETPOINTS_PA[self.index()]
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_cascade_deck();
    export(OUTPUTS[0], &deck);

    let plenums = pressure_step_plenums();
    export(OUTPUTS[1], &plenums);

    let smoke = smoke_injection_manifold();
    export(OUTPUTS[2], &smoke);

    let recovery = return_exhaust_recovery_path();
    export(OUTPUTS[3], &recovery);

    let sensors = sensor_tower_array();
    export(OUTPUTS[4], &sensors);

    let door = door_transfer_port_leak_challenge();
    export(OUTPUTS[5], &door);

    let service = service_line_penetration_bulkhead();
    export(OUTPUTS[6], &service);

    let samples = sample_cassette_exposure_rack();
    export(OUTPUTS[7], &samples);

    let filters = recovery_filter_coupon_lane();
    export(OUTPUTS[8], &filters);

    let trace = barcode_protocol_lands();
    export(OUTPUTS[9], &trace);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + plenums.translate(PLENUM_POS.0, PLENUM_POS.1, DECK_Z)
        + smoke.translate(SMOKE_POS.0, SMOKE_POS.1, DECK_Z)
        + recovery.translate(RECOVERY_POS.0, RECOVERY_POS.1, DECK_Z)
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, DECK_Z)
        + door.translate(DOOR_POS.0, DOOR_POS.1, DECK_Z)
        + service.translate(SERVICE_POS.0, SERVICE_POS.1, DECK_Z)
        + samples.translate(SAMPLE_POS.0, SAMPLE_POS.1, DECK_Z)
        + filters.translate(FILTER_POS.0, FILTER_POS.1, DECK_Z)
        + trace.translate(TRACE_POS.0, TRACE_POS.1, DECK_Z)
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cleanroom pressure cascade smoke recovery station:");
    println!(
        "  Cascade:        {PRESSURE_STEP_COUNT} pressure zones, {:.0} Pa total drop, {:.0} Pa minimum adjacent step",
        cascade_total_drop_pa(),
        minimum_adjacent_pressure_step_pa()
    );
    println!(
        "  Smoke path:     {SMOKE_PORT_COUNT} injection ports, {MIXING_VANE_COUNT} mixing vanes, {RETURN_SLOT_COUNT} return slots, {HEPA_RECOVERY_COUPON_COUNT} HEPA recovery coupons"
    );
    println!(
        "  Monitoring:     {SENSOR_TOWER_COUNT} sensor towers, {DIFFERENTIAL_PRESSURE_PAIR_COUNT} differential pressure pairs, {PARTICLE_SMOKE_REFERENCE_COUNT} particle/smoke reference pockets"
    );
    println!(
        "  Challenge:      {DOOR_LEAF_COUNT} door leaves, {TRANSFER_PORT_COUNT} RTP ring, {LEAK_SHIM_COUNT} calibrated leak shims, {SERVICE_PENETRATION_COUNT} service penetrations"
    );
    println!(
        "  Samples:        {SAMPLE_CASSETTE_COUNT} Rev-C cassette surrogates, {SAMPLE_WITNESS_STRIP_COUNT} witness strips, {FILTER_COUPON_COUNT} recovery filter coupons"
    );
    println!(
        "  Evidence:       {BARCODE_LAND_COUNT} barcode lands, {PROTOCOL_CARD_COUNT} protocol cards, {CAMERA_POD_COUNT} camera pods, {LIGHT_BAR_COUNT} light bars"
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

    assert_eq!(PRESSURE_SETPOINTS_PA.len(), PRESSURE_STEP_COUNT);
    assert_eq!(TRANSITION_GATE_COUNT, PRESSURE_STEP_COUNT - 1);
    assert_eq!(PRESSURE_TAP_COUNT, PRESSURE_STEP_COUNT * 2);
    assert_eq!(
        SMOKE_PORT_COUNT,
        SMOKE_BRANCH_COUNT * SMOKE_PORTS_PER_BRANCH
    );
    assert_eq!(SERVICE_PENETRATION_COUNT, SERVICE_ROWS * SERVICE_COLS);
    assert_eq!(SAMPLE_CASSETTE_COUNT, SAMPLE_COLS * SAMPLE_ROWS);
    assert_eq!(
        FILTER_COUPON_COUNT,
        FILTER_UPSTREAM_COUNT + FILTER_DOWNSTREAM_COUNT
    );
    assert_eq!(SENSOR_FLAG_COUNT, SENSOR_TOWER_COUNT);
    assert_eq!(KEEP_OUT_FRAME_COUNT, 4);
    assert_eq!(
        SENSOR_PORTS_PER_TOWER * SENSOR_TOWER_COUNT,
        pressure_sensor_port_count()
    );

    for pair in PRESSURE_SETPOINTS_PA.windows(2) {
        assert!(
            pair[0] - pair[1] >= MIN_PRESSURE_STEP_PA,
            "pressure cascade step below minimum"
        );
    }

    assert!(cascade_total_drop_pa() >= 30.0);
    assert!(minimum_adjacent_pressure_step_pa() >= MIN_PRESSURE_STEP_PA);
    assert!(recovery_open_area_mm2() > smoke_injection_open_area_mm2() * 5.0);
    assert!(PLENUM_CELL_X * (PRESSURE_STEP_COUNT as f64) < PLENUM_X);
    assert!(SAMPLE_CASSETTE_X > REVC_CHIP_LENGTH + 18.0);
    assert!(SAMPLE_CASSETTE_Y > REVC_CHIP_WIDTH + 10.0);
    assert!(SAMPLE_CASSETTE_Z > REVC_TOTAL_HEIGHT + 8.0);
    assert!(SAMPLE_CASSETTE_COUNT >= PRESSURE_STEP_COUNT * 2);
    assert!(BRIDGE_UNDERSIDE_Z > tallest_sensor_tower_height() + SENSOR_BASE_Z + 70.0);
    assert!(TOP_FILTER_LIFT_CLEARANCE_Z > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 40.0);

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

fn layout_rects() -> [Rect; 9] {
    [
        rect("pressure_step_plenums", PLENUM_POS, PLENUM_X, PLENUM_Y),
        rect("smoke_injection_manifold", SMOKE_POS, SMOKE_X, SMOKE_Y),
        rect(
            "return_exhaust_recovery_path",
            RECOVERY_POS,
            RECOVERY_X,
            RECOVERY_Y,
        ),
        rect("sensor_tower_array", SENSOR_POS, SENSOR_X, SENSOR_Y),
        rect(
            "door_transfer_port_leak_challenge",
            DOOR_POS,
            DOOR_X,
            DOOR_Y,
        ),
        rect(
            "service_line_penetration_bulkhead",
            SERVICE_POS,
            SERVICE_X,
            SERVICE_Y,
        ),
        rect(
            "sample_cassette_exposure_rack",
            SAMPLE_POS,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "recovery_filter_coupon_lane",
            FILTER_POS,
            FILTER_X,
            FILTER_Y,
        ),
        rect("barcode_protocol_lands", TRACE_POS, TRACE_X, TRACE_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_cascade_deck() -> Part {
    let deck = centered_cube(
        "cleanroom_pressure_cascade_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "cleanroom_pressure_cascade_wipeable_basin_cut",
        DECK_X - 150.0,
        DECK_Y - 132.0,
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.3);
    let smoke_drain_gutter = centered_cube(
        "cleanroom_pressure_cascade_front_smoke_condensate_gutter_cut",
        DECK_X - 240.0,
        16.0,
        9.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 58.0, DECK_Z - 4.0);
    let recovery_sump = centered_cylinder(
        "cleanroom_pressure_cascade_recovery_sump_drain_cut",
        8.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 82.0, -DECK_Y / 2.0 + 42.0, DECK_Z / 2.0);

    deck - basin - smoke_drain_gutter - recovery_sump - insert_sockets() - mounting_slots()
        + perimeter_lips()
        + cascade_flow_axis_spines()
        + datum_targets()
        + pressure_direction_ticks()
        + evidence_bridge_anchor_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("cleanroom_pressure_cascade_insert_socket_cuts");
    for item in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("cleanroom_pressure_cascade_{}_socket_cut", item.name),
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
    let mut slots = Part::empty("cleanroom_pressure_cascade_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().iter().enumerate() {
        let round = centered_cylinder(
            format!("cleanroom_pressure_cascade_m6_mount_round_{i}"),
            3.4,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("cleanroom_pressure_cascade_m6_mount_slot_{i}"),
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
        (-DECK_X / 2.0 + 64.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 2.0 - 64.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 2.0 + 64.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 64.0, DECK_Y / 2.0 - 62.0),
        (0.0, -DECK_Y / 2.0 + 62.0),
        (0.0, DECK_Y / 2.0 - 62.0),
        (-DECK_X / 2.0 + 64.0, 0.0),
        (DECK_X / 2.0 - 64.0, 0.0),
        (PLENUM_POS.0, PLENUM_POS.1 - PLENUM_Y / 2.0 + 42.0),
        (RECOVERY_POS.0, RECOVERY_POS.1 + RECOVERY_Y / 2.0 - 38.0),
    ]
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        "cleanroom_pressure_cascade_rear_service_high_lip",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "cleanroom_pressure_cascade_left_clean_zone_lip",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "cleanroom_pressure_cascade_right_recovery_service_lip",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "cleanroom_pressure_cascade_front_robot_low_lip",
        DECK_X - 160.0,
        14.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 36.0, DECK_Z + 12.0);

    rear + left + right + front
}

fn cascade_flow_axis_spines() -> Part {
    let clean_to_recovery = centered_cube(
        "cleanroom_pressure_cascade_clean_to_recovery_axis_spine",
        12.0,
        610.0,
        20.0,
    )
    .translate(28.0, 22.0, DECK_Z + 10.0);
    let sample_to_plenum = centered_cube(
        "cleanroom_pressure_cascade_sample_to_plenum_axis_spine",
        650.0,
        10.0,
        18.0,
    )
    .translate(-382.0, 248.0, DECK_Z + 9.0);
    let smoke_to_exhaust = centered_cube(
        "cleanroom_pressure_cascade_smoke_to_exhaust_axis_spine",
        10.0,
        360.0,
        18.0,
    )
    .translate(132.0, 30.0, DECK_Z + 9.0);

    clean_to_recovery + sample_to_plenum + smoke_to_exhaust
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("cleanroom_pressure_cascade_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().iter().enumerate() {
        targets =
            targets
                + fiducial_target(&format!("cleanroom_pressure_cascade_datum_target_{i}"))
                    .translate(*x, *y, DECK_Z + 1.0);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-DECK_X / 2.0 + 96.0, -DECK_Y / 2.0 + 96.0),
        (DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 96.0),
        (-DECK_X / 2.0 + 96.0, DECK_Y / 2.0 - 96.0),
        (DECK_X / 2.0 - 96.0, DECK_Y / 2.0 - 96.0),
    ]
}

fn pressure_direction_ticks() -> Part {
    let mut ticks = Part::empty("cleanroom_pressure_cascade_pressure_direction_ticks");
    for i in 0..PRESSURE_STEP_COUNT {
        let x = PLENUM_POS.0 + centered_index(i, PRESSURE_STEP_COUNT, PLENUM_PITCH_X);
        let tick = centered_cube(
            format!("cleanroom_pressure_cascade_pressure_tick_{i}"),
            38.0,
            6.0,
            4.0,
        )
        .translate(x, PLENUM_POS.1 + PLENUM_Y / 2.0 - 18.0, DECK_Z + 2.0);
        let step_flag = centered_cube(
            format!("cleanroom_pressure_cascade_pressure_setpoint_flag_{i}"),
            9.0,
            18.0,
            6.0 + CascadeZone::all()[i].setpoint_pa() / 5.0,
        )
        .translate(x, PLENUM_POS.1 + PLENUM_Y / 2.0 - 34.0, DECK_Z + 3.0);
        ticks = ticks + tick + step_flag;
    }
    ticks
}

fn evidence_bridge_anchor_lands() -> Part {
    let left = centered_cube(
        "cleanroom_pressure_cascade_left_bridge_anchor_land",
        80.0,
        60.0,
        8.0,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, DECK_Z + 4.0);
    let right = centered_cube(
        "cleanroom_pressure_cascade_right_bridge_anchor_land",
        80.0,
        60.0,
        8.0,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, DECK_Z + 4.0);
    left + right
}

fn pressure_step_plenums() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_pressure_step_plenum_base",
        PLENUM_X,
        PLENUM_Y,
        PLENUM_Z,
    )
    .translate(0.0, 0.0, PLENUM_Z / 2.0);
    let recess = centered_cube(
        "cleanroom_pressure_cascade_pressure_step_plenum_floor_recess_cut",
        PLENUM_X - 52.0,
        PLENUM_Y - 54.0,
        8.0,
    )
    .translate(0.0, 0.0, PLENUM_Z - 4.0);

    let mut cells = Part::empty("cleanroom_pressure_cascade_pressure_cells");
    for zone in CascadeZone::all() {
        cells = cells + pressure_plenum_cell(zone);
    }

    base - recess + cells + transition_gates() + pressure_tap_rails() + plenum_zone_labels()
}

fn pressure_plenum_cell(zone: CascadeZone) -> Part {
    let i = zone.index();
    let x = centered_index(i, PRESSURE_STEP_COUNT, PLENUM_PITCH_X);
    let wall_z = PLENUM_WALL_Z + zone.setpoint_pa() * 0.35;

    let floor = centered_cube(
        format!(
            "cleanroom_pressure_cascade_{}_perforated_floor",
            zone.label()
        ),
        PLENUM_CELL_X,
        PLENUM_CELL_Y,
        16.0,
    )
    .translate(x, 0.0, 8.0);
    let mut slots = Part::empty(format!(
        "cleanroom_pressure_cascade_{}_diffuser_slot_cuts",
        zone.label()
    ));
    for slot in 0..DIFFUSER_SLOTS_PER_PLENUM {
        slots = slots
            + centered_cube(
                format!(
                    "cleanroom_pressure_cascade_{}_diffuser_slot_{slot}",
                    zone.label()
                ),
                PLENUM_CELL_X - 40.0,
                8.0,
                20.0,
            )
            .translate(
                x,
                centered_index(slot, DIFFUSER_SLOTS_PER_PLENUM, 34.0),
                8.0,
            );
    }

    let walls = rectangular_frame_xy(
        &format!(
            "cleanroom_pressure_cascade_{}_raised_pressure_wall",
            zone.label()
        ),
        PLENUM_CELL_X + 18.0,
        PLENUM_CELL_Y + 18.0,
        PLENUM_WALL_W,
        wall_z,
    )
    .translate(x, 0.0, 16.0);
    let reference_riser = centered_cube(
        format!(
            "cleanroom_pressure_cascade_{}_setpoint_height_riser",
            zone.label()
        ),
        28.0,
        42.0,
        18.0 + zone.setpoint_pa(),
    )
    .translate(
        x,
        PLENUM_CELL_Y / 2.0 - 38.0,
        16.0 + (18.0 + zone.setpoint_pa()) / 2.0,
    );

    floor - slots + walls + reference_riser
}

fn transition_gates() -> Part {
    let mut gates = Part::empty("cleanroom_pressure_cascade_transition_gates");
    for i in 0..TRANSITION_GATE_COUNT {
        let x = (centered_index(i, PRESSURE_STEP_COUNT, PLENUM_PITCH_X)
            + centered_index(i + 1, PRESSURE_STEP_COUNT, PLENUM_PITCH_X))
            / 2.0;
        let gate = centered_cube(
            format!("cleanroom_pressure_cascade_zone_transition_gate_{i}"),
            28.0,
            PLENUM_CELL_Y + 34.0,
            58.0,
        )
        .translate(x, 0.0, 16.0 + 29.0);
        let slot = centered_cube(
            format!("cleanroom_pressure_cascade_zone_transition_bleed_slot_{i}"),
            30.0,
            74.0,
            24.0,
        )
        .translate(x, 0.0, 16.0 + 26.0);
        let damper_flag = centered_cube(
            format!("cleanroom_pressure_cascade_transition_damper_flag_{i}"),
            44.0,
            8.0,
            24.0,
        )
        .translate(x, -PLENUM_CELL_Y / 2.0 - 22.0, 16.0 + 58.0);
        gates = gates + (gate - slot) + damper_flag;
    }
    gates
}

fn pressure_tap_rails() -> Part {
    let mut rails = Part::empty("cleanroom_pressure_cascade_pressure_tap_rails");
    for zone in CascadeZone::all() {
        let i = zone.index();
        let x = centered_index(i, PRESSURE_STEP_COUNT, PLENUM_PITCH_X);
        for side in 0..2 {
            let y = if side == 0 {
                -PLENUM_CELL_Y / 2.0 - 28.0
            } else {
                PLENUM_CELL_Y / 2.0 + 28.0
            };
            let boss = centered_cylinder(
                format!(
                    "cleanroom_pressure_cascade_{}_pressure_tap_boss_{side}",
                    zone.label()
                ),
                15.0,
                12.0,
                36,
            )
            .translate(x, y, PLENUM_Z + 6.0);
            let bore = centered_cylinder(
                format!(
                    "cleanroom_pressure_cascade_{}_pressure_tap_bore_{side}",
                    zone.label()
                ),
                4.0,
                16.0,
                24,
            )
            .translate(x, y, PLENUM_Z + 6.0);
            rails = rails + (boss - bore);
        }
    }
    rails
}

fn plenum_zone_labels() -> Part {
    let mut labels = Part::empty("cleanroom_pressure_cascade_zone_label_lands");
    for zone in CascadeZone::all() {
        let i = zone.index();
        let x = centered_index(i, PRESSURE_STEP_COUNT, PLENUM_PITCH_X);
        labels = labels
            + centered_cube(
                format!("cleanroom_pressure_cascade_{}_label_land", zone.label()),
                92.0,
                18.0,
                4.0,
            )
            .translate(x, -PLENUM_CELL_Y / 2.0 - 50.0, PLENUM_Z + 2.0);
    }
    labels
}

fn smoke_injection_manifold() -> Part {
    let plate = centered_cube(
        "cleanroom_pressure_cascade_smoke_manifold_base_plate",
        SMOKE_X,
        SMOKE_Y,
        SMOKE_Z,
    )
    .translate(0.0, 0.0, SMOKE_Z / 2.0);
    let recessed_pocket = centered_cube(
        "cleanroom_pressure_cascade_smoke_non_product_generator_pocket_cut",
        138.0,
        74.0,
        12.0,
    )
    .translate(-SMOKE_X / 2.0 + 96.0, -SMOKE_Y / 2.0 + 54.0, SMOKE_Z - 6.0);

    plate - recessed_pocket
        + smoke_source_cups()
        + smoke_branch_tubes()
        + smoke_port_array()
        + smoke_coupon_sockets()
        + smoke_mixing_vanes()
        + smoke_flow_check_flags()
}

fn smoke_source_cups() -> Part {
    let mut cups = Part::empty("cleanroom_pressure_cascade_smoke_source_cups");
    for i in 0..SMOKE_SOURCE_CUP_COUNT {
        let y = centered_index(i, SMOKE_SOURCE_CUP_COUNT, 46.0);
        let cup = centered_cylinder(
            format!("cleanroom_pressure_cascade_smoke_source_cup_{i}"),
            22.0,
            22.0,
            44,
        )
        .translate(-SMOKE_X / 2.0 + 76.0, y, SMOKE_Z + 11.0);
        let bore = centered_cylinder(
            format!("cleanroom_pressure_cascade_smoke_source_cup_bore_{i}"),
            14.0,
            24.0,
            36,
        )
        .translate(-SMOKE_X / 2.0 + 76.0, y, SMOKE_Z + 11.0);
        cups = cups + (cup - bore);
    }
    cups
}

fn smoke_branch_tubes() -> Part {
    let main = centered_cylinder(
        "cleanroom_pressure_cascade_smoke_main_distribution_tube",
        12.0,
        SMOKE_X - 128.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(34.0, 0.0, SMOKE_Z + 22.0);

    let mut branches = Part::empty("cleanroom_pressure_cascade_smoke_branch_tubes");
    for branch in 0..SMOKE_BRANCH_COUNT {
        let x = centered_index(branch, SMOKE_BRANCH_COUNT, 74.0) + 70.0;
        let branch_tube = centered_cylinder(
            format!("cleanroom_pressure_cascade_smoke_branch_tube_{branch}"),
            7.0,
            SMOKE_Y - 72.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, SMOKE_Z + 18.0);
        branches = branches + branch_tube;
    }

    main + branches
}

fn smoke_port_array() -> Part {
    let mut ports = Part::empty("cleanroom_pressure_cascade_smoke_port_array");
    for branch in 0..SMOKE_BRANCH_COUNT {
        let x = centered_index(branch, SMOKE_BRANCH_COUNT, 74.0) + 70.0;
        for port in 0..SMOKE_PORTS_PER_BRANCH {
            let y = centered_index(port, SMOKE_PORTS_PER_BRANCH, 38.0);
            let collar = centered_cylinder(
                format!("cleanroom_pressure_cascade_smoke_port_collar_{branch}_{port}"),
                13.0,
                8.0,
                32,
            )
            .translate(x, y, SMOKE_Z + 4.0);
            let bore = centered_cylinder(
                format!("cleanroom_pressure_cascade_smoke_port_bore_{branch}_{port}"),
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

fn smoke_coupon_sockets() -> Part {
    let mut sockets = Part::empty("cleanroom_pressure_cascade_smoke_coupon_sockets");
    for i in 0..SMOKE_COUPON_SOCKET_COUNT {
        let x = centered_index(i % 4, 4, 62.0) + 74.0;
        let y = if i < 4 { -82.0 } else { 82.0 };
        let socket = centered_cylinder(
            format!("cleanroom_pressure_cascade_smoke_coupon_socket_{i}"),
            18.0,
            8.0,
            36,
        )
        .translate(x, y, SMOKE_Z + 4.0);
        let relief = centered_cylinder(
            format!("cleanroom_pressure_cascade_smoke_coupon_relief_{i}"),
            11.0,
            10.0,
            28,
        )
        .translate(x, y, SMOKE_Z + 4.0);
        sockets = sockets + (socket - relief);
    }
    sockets
}

fn smoke_mixing_vanes() -> Part {
    let mut vanes = Part::empty("cleanroom_pressure_cascade_smoke_mixing_vanes");
    for i in 0..MIXING_VANE_COUNT {
        let x = centered_index(i, MIXING_VANE_COUNT, 34.0) + 76.0;
        let vane = centered_cube(
            format!("cleanroom_pressure_cascade_smoke_mixing_vane_{i}"),
            6.0,
            34.0,
            32.0,
        )
        .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
        .translate(x, 0.0, SMOKE_Z + 16.0);
        vanes = vanes + vane;
    }
    vanes
}

fn smoke_flow_check_flags() -> Part {
    let upstream = centered_cube(
        "cleanroom_pressure_cascade_smoke_upstream_check_flag",
        48.0,
        10.0,
        36.0,
    )
    .translate(
        -SMOKE_X / 2.0 + 152.0,
        -SMOKE_Y / 2.0 + 26.0,
        SMOKE_Z + 18.0,
    );
    let downstream = centered_cube(
        "cleanroom_pressure_cascade_smoke_downstream_check_flag",
        48.0,
        10.0,
        36.0,
    )
    .translate(SMOKE_X / 2.0 - 46.0, SMOKE_Y / 2.0 - 26.0, SMOKE_Z + 18.0);
    upstream + downstream
}

fn return_exhaust_recovery_path() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_return_recovery_base",
        RECOVERY_X,
        RECOVERY_Y,
        RECOVERY_Z,
    )
    .translate(0.0, 0.0, RECOVERY_Z / 2.0);
    let channel_cut = centered_cube(
        "cleanroom_pressure_cascade_return_recovery_center_channel_cut",
        RECOVERY_X - 86.0,
        62.0,
        18.0,
    )
    .translate(0.0, 0.0, RECOVERY_Z - 9.0);

    base - channel_cut
        + return_slot_bank()
        + recovery_trap_cups()
        + hepa_recovery_coupon_cassettes()
        + exhaust_stack_surrogates()
        + recovery_flow_arrow_tabs()
}

fn return_slot_bank() -> Part {
    let mut bank = Part::empty("cleanroom_pressure_cascade_return_slot_bank");
    for i in 0..RETURN_SLOT_COUNT {
        let x = centered_index(i, RETURN_SLOT_COUNT, 42.0);
        let lip = centered_cube(
            format!("cleanroom_pressure_cascade_return_slot_lip_{i}"),
            RETURN_SLOT_X + 12.0,
            RETURN_SLOT_Y + 8.0,
            6.0,
        )
        .translate(x, -RECOVERY_Y / 2.0 + 42.0, RECOVERY_Z + 3.0);
        let slot = centered_cube(
            format!("cleanroom_pressure_cascade_return_slot_clearance_{i}"),
            RETURN_SLOT_X,
            RETURN_SLOT_Y,
            8.0,
        )
        .translate(x, -RECOVERY_Y / 2.0 + 42.0, RECOVERY_Z + 3.0);
        bank = bank + (lip - slot);
    }
    bank
}

fn recovery_trap_cups() -> Part {
    let mut cups = Part::empty("cleanroom_pressure_cascade_recovery_trap_cups");
    for i in 0..RECOVERY_TRAP_COUNT {
        let x = centered_index(i, RECOVERY_TRAP_COUNT, 86.0) - 126.0;
        let cup = centered_cylinder(
            format!("cleanroom_pressure_cascade_recovery_trap_cup_{i}"),
            24.0,
            26.0,
            44,
        )
        .translate(x, RECOVERY_Y / 2.0 - 52.0, RECOVERY_Z + 13.0);
        let bore = centered_cylinder(
            format!("cleanroom_pressure_cascade_recovery_trap_cup_bore_{i}"),
            15.0,
            28.0,
            36,
        )
        .translate(x, RECOVERY_Y / 2.0 - 52.0, RECOVERY_Z + 13.0);
        cups = cups + (cup - bore);
    }
    cups
}

fn hepa_recovery_coupon_cassettes() -> Part {
    let mut cassettes = Part::empty("cleanroom_pressure_cascade_hepa_recovery_coupon_cassettes");
    for i in 0..HEPA_RECOVERY_COUPON_COUNT {
        let x = centered_index(i, HEPA_RECOVERY_COUPON_COUNT, 92.0) + 120.0;
        let nest = centered_cube(
            format!("cleanroom_pressure_cascade_hepa_recovery_coupon_nest_{i}"),
            74.0,
            62.0,
            18.0,
        )
        .translate(x, RECOVERY_Y / 2.0 - 56.0, RECOVERY_Z + 9.0);
        let coupon = centered_cube(
            format!("cleanroom_pressure_cascade_hepa_recovery_coupon_shadow_{i}"),
            52.0,
            42.0,
            8.0,
        )
        .translate(x, RECOVERY_Y / 2.0 - 56.0, RECOVERY_Z + 20.0);
        cassettes = cassettes + nest + coupon;
    }
    cassettes
}

fn exhaust_stack_surrogates() -> Part {
    let mut stacks = Part::empty("cleanroom_pressure_cascade_exhaust_stack_surrogates");
    for i in 0..EXHAUST_STACK_COUNT {
        let y = centered_index(i, EXHAUST_STACK_COUNT, 66.0);
        let stack = centered_cylinder(
            format!("cleanroom_pressure_cascade_exhaust_stack_{i}"),
            30.0,
            88.0,
            48,
        )
        .translate(RECOVERY_X / 2.0 - 58.0, y, RECOVERY_Z + 44.0);
        let bore = centered_cylinder(
            format!("cleanroom_pressure_cascade_exhaust_stack_bore_{i}"),
            20.0,
            90.0,
            40,
        )
        .translate(RECOVERY_X / 2.0 - 58.0, y, RECOVERY_Z + 44.0);
        stacks = stacks + (stack - bore);
    }
    stacks
}

fn recovery_flow_arrow_tabs() -> Part {
    let mut tabs = Part::empty("cleanroom_pressure_cascade_recovery_flow_arrow_tabs");
    for i in 0..5 {
        let tab = centered_cube(
            format!("cleanroom_pressure_cascade_recovery_flow_arrow_tab_{i}"),
            38.0,
            8.0,
            8.0,
        )
        .rotate(0.0, 0.0, -18.0)
        .translate(centered_index(i, 5, 72.0), 0.0, RECOVERY_Z + 4.0);
        tabs = tabs + tab;
    }
    tabs
}

fn sensor_tower_array() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_sensor_tower_base",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_BASE_Z,
    )
    .translate(0.0, 0.0, SENSOR_BASE_Z / 2.0);

    base + sensor_towers()
        + differential_pressure_pair_bridge()
        + particle_smoke_reference_pockets()
        + sensor_cable_comb()
}

fn sensor_towers() -> Part {
    let mut towers = Part::empty("cleanroom_pressure_cascade_sensor_towers");
    for i in 0..SENSOR_TOWER_COUNT {
        let col = i % 3;
        let row = i / 3;
        let x = centered_index(col, 3, 150.0);
        let y = centered_index(row, 2, 96.0);
        let height = SENSOR_TOWER_HEIGHTS[i];
        let mast = centered_cylinder(
            format!("cleanroom_pressure_cascade_sensor_tower_mast_{i}"),
            7.0,
            height,
            28,
        )
        .translate(x, y, SENSOR_BASE_Z + height / 2.0);
        let pod = centered_cube(
            format!("cleanroom_pressure_cascade_sensor_tower_pod_{i}"),
            42.0,
            30.0,
            24.0,
        )
        .translate(x, y, SENSOR_BASE_Z + height + 12.0);
        let low_port = centered_cylinder(
            format!("cleanroom_pressure_cascade_sensor_low_sample_port_{i}"),
            5.0,
            16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y - 18.0, SENSOR_BASE_Z + height * 0.38);
        let high_port = centered_cylinder(
            format!("cleanroom_pressure_cascade_sensor_high_sample_port_{i}"),
            5.0,
            16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y - 18.0, SENSOR_BASE_Z + height * 0.78);
        let flag = centered_cube(
            format!("cleanroom_pressure_cascade_sensor_height_flag_{i}"),
            30.0,
            8.0,
            12.0,
        )
        .translate(x + 28.0, y, SENSOR_BASE_Z + height);
        towers = towers + mast + pod + low_port + high_port + flag;
    }
    towers
}

fn differential_pressure_pair_bridge() -> Part {
    let mut bridges = Part::empty("cleanroom_pressure_cascade_differential_pressure_pair_bridges");
    for i in 0..DIFFERENTIAL_PRESSURE_PAIR_COUNT {
        let x = centered_index(i, DIFFERENTIAL_PRESSURE_PAIR_COUNT, 132.0);
        let block = centered_cube(
            format!("cleanroom_pressure_cascade_dp_pair_bridge_{i}"),
            96.0,
            22.0,
            18.0,
        )
        .translate(x, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        let tap_a = centered_cylinder(
            format!("cleanroom_pressure_cascade_dp_pair_tap_a_{i}"),
            5.0,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 24.0, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        let tap_b = centered_cylinder(
            format!("cleanroom_pressure_cascade_dp_pair_tap_b_{i}"),
            5.0,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 24.0, SENSOR_Y / 2.0 - 34.0, SENSOR_BASE_Z + 9.0);
        bridges = bridges + (block - tap_a - tap_b);
    }
    bridges
}

fn particle_smoke_reference_pockets() -> Part {
    let mut pockets = Part::empty("cleanroom_pressure_cascade_particle_smoke_reference_pockets");
    for i in 0..PARTICLE_SMOKE_REFERENCE_COUNT {
        let x = centered_index(i, PARTICLE_SMOKE_REFERENCE_COUNT, 116.0);
        let pocket = centered_cube(
            format!("cleanroom_pressure_cascade_particle_smoke_reference_pocket_{i}"),
            74.0,
            46.0,
            16.0,
        )
        .translate(x, -SENSOR_Y / 2.0 + 42.0, SENSOR_BASE_Z + 8.0);
        let target = fiducial_target(&format!(
            "cleanroom_pressure_cascade_particle_smoke_reference_target_{i}"
        ))
        .translate(x, -SENSOR_Y / 2.0 + 42.0, SENSOR_BASE_Z + 17.0);
        pockets = pockets + pocket + target;
    }
    pockets
}

fn sensor_cable_comb() -> Part {
    let comb = centered_cube(
        "cleanroom_pressure_cascade_sensor_cable_comb",
        SENSOR_X - 74.0,
        34.0,
        18.0,
    )
    .translate(0.0, -SENSOR_Y / 2.0 + 14.0, SENSOR_BASE_Z + 9.0);
    let mut slots = Part::empty("cleanroom_pressure_cascade_sensor_cable_comb_slots");
    for i in 0..SENSOR_TOWER_COUNT {
        slots = slots
            + centered_cube(
                format!("cleanroom_pressure_cascade_sensor_cable_comb_slot_{i}"),
                12.0,
                38.0,
                20.0,
            )
            .translate(
                centered_index(i, SENSOR_TOWER_COUNT, 64.0),
                -SENSOR_Y / 2.0 + 14.0,
                SENSOR_BASE_Z + 9.0,
            );
    }
    comb - slots
}

fn door_transfer_port_leak_challenge() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_door_transfer_port_base",
        DOOR_X,
        DOOR_Y,
        DOOR_Z,
    )
    .translate(0.0, 0.0, DOOR_Z / 2.0);

    base + door_leaf_pair()
        + round_transfer_port_ring()
        + calibrated_leak_shim_lane()
        + interlock_challenge_pins()
        + door_gasket_witness_ticks()
}

fn door_leaf_pair() -> Part {
    let mut leaves = Part::empty("cleanroom_pressure_cascade_door_leaf_pair");
    for i in 0..DOOR_LEAF_COUNT {
        let x = centered_index(i, DOOR_LEAF_COUNT, 86.0) - 56.0;
        let leaf = centered_cube(
            format!("cleanroom_pressure_cascade_door_leaf_{i}"),
            78.0,
            120.0,
            16.0,
        )
        .translate(x, 0.0, DOOR_Z + 8.0);
        let gasket = rectangular_frame_xy(
            &format!("cleanroom_pressure_cascade_door_leaf_gasket_land_{i}"),
            64.0,
            104.0,
            DOOR_GASKET_W,
            8.0,
        )
        .translate(x, 0.0, DOOR_Z + 16.0);
        let handle = centered_cube(
            format!("cleanroom_pressure_cascade_door_leaf_handle_{i}"),
            10.0,
            48.0,
            16.0,
        )
        .translate(x + 24.0, 0.0, DOOR_Z + 26.0);
        leaves = leaves + leaf + gasket + handle;
    }
    leaves
}

fn round_transfer_port_ring() -> Part {
    let ring = centered_cylinder(
        "cleanroom_pressure_cascade_transfer_port_outer_ring",
        TRANSFER_PORT_OD / 2.0,
        18.0,
        56,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DOOR_X / 2.0 - 66.0, 0.0, DOOR_Z + TRANSFER_PORT_OD / 2.0);
    let bore = centered_cylinder(
        "cleanroom_pressure_cascade_transfer_port_bore",
        TRANSFER_PORT_ID / 2.0,
        22.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DOOR_X / 2.0 - 66.0, 0.0, DOOR_Z + TRANSFER_PORT_OD / 2.0);
    let index_lug = centered_cube(
        "cleanroom_pressure_cascade_transfer_port_index_lug",
        28.0,
        18.0,
        16.0,
    )
    .translate(DOOR_X / 2.0 - 66.0, -18.0, DOOR_Z + TRANSFER_PORT_OD + 5.0);
    (ring - bore) + index_lug
}

fn calibrated_leak_shim_lane() -> Part {
    let mut shims = Part::empty("cleanroom_pressure_cascade_calibrated_leak_shim_lane");
    for i in 0..LEAK_SHIM_COUNT {
        let x = centered_index(i, LEAK_SHIM_COUNT, 34.0);
        let shim = centered_cube(
            format!("cleanroom_pressure_cascade_calibrated_leak_shim_{i}"),
            24.0,
            18.0,
            4.0 + i as f64,
        )
        .translate(x, -DOOR_Y / 2.0 + 34.0, DOOR_Z + 2.0 + i as f64 / 2.0);
        shims = shims + shim;
    }
    shims
}

fn interlock_challenge_pins() -> Part {
    let mut pins = Part::empty("cleanroom_pressure_cascade_interlock_challenge_pins");
    for i in 0..INTERLOCK_PIN_COUNT {
        let x = centered_index(i, INTERLOCK_PIN_COUNT, 42.0) - 52.0;
        let pin = centered_cylinder(
            format!("cleanroom_pressure_cascade_interlock_pin_{i}"),
            5.0,
            42.0,
            24,
        )
        .translate(x, DOOR_Y / 2.0 - 34.0, DOOR_Z + 21.0);
        let flag = centered_cube(
            format!("cleanroom_pressure_cascade_interlock_pin_flag_{i}"),
            24.0,
            8.0,
            12.0,
        )
        .translate(x + 10.0, DOOR_Y / 2.0 - 34.0, DOOR_Z + 38.0);
        pins = pins + pin + flag;
    }
    pins
}

fn door_gasket_witness_ticks() -> Part {
    let mut ticks = Part::empty("cleanroom_pressure_cascade_door_gasket_witness_ticks");
    for i in 0..8 {
        let x = centered_index(i, 8, 28.0);
        let tick = centered_cube(
            format!("cleanroom_pressure_cascade_door_gasket_witness_tick_{i}"),
            12.0,
            5.0,
            8.0,
        )
        .translate(x, DOOR_Y / 2.0 - 16.0, DOOR_Z + 4.0);
        ticks = ticks + tick;
    }
    ticks
}

fn service_line_penetration_bulkhead() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_service_bulkhead_base_tray",
        SERVICE_X,
        SERVICE_Y,
        22.0,
    )
    .translate(0.0, 0.0, 11.0);
    let panel = centered_cube(
        "cleanroom_pressure_cascade_service_penetration_panel",
        SERVICE_X - 24.0,
        26.0,
        SERVICE_PANEL_Z,
    )
    .translate(0.0, SERVICE_Y / 2.0 - 42.0, 22.0 + SERVICE_PANEL_Z / 2.0);

    base + (panel - service_line_bores())
        + service_port_collars()
        + service_strain_relief_comb()
        + service_drip_trays()
        + service_class_label_lands()
}

fn service_line_bores() -> Part {
    let mut bores = Part::empty("cleanroom_pressure_cascade_service_line_bores");
    for i in 0..SERVICE_PENETRATION_COUNT {
        let (x, z) = service_penetration_pose(i);
        bores = bores
            + centered_cylinder(
                format!("cleanroom_pressure_cascade_service_line_bore_{i}"),
                SERVICE_PORT_D / 2.0,
                32.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, SERVICE_Y / 2.0 - 42.0, z);
    }
    bores
}

fn service_port_collars() -> Part {
    let mut collars = Part::empty("cleanroom_pressure_cascade_service_port_collars");
    for i in 0..SERVICE_PENETRATION_COUNT {
        let (x, z) = service_penetration_pose(i);
        let collar = centered_cylinder(
            format!("cleanroom_pressure_cascade_service_port_collar_{i}"),
            SERVICE_COLLAR_D / 2.0,
            10.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SERVICE_Y / 2.0 - 58.0, z);
        let bore = centered_cylinder(
            format!("cleanroom_pressure_cascade_service_port_collar_bore_{i}"),
            SERVICE_PORT_D / 2.0,
            12.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SERVICE_Y / 2.0 - 58.0, z);
        collars = collars + (collar - bore);
    }
    collars
}

fn service_penetration_pose(index: usize) -> (f64, f64) {
    let col = index % SERVICE_COLS;
    let row = index / SERVICE_COLS;
    (
        centered_index(col, SERVICE_COLS, 44.0),
        22.0 + 72.0 + row as f64 * 68.0,
    )
}

fn service_strain_relief_comb() -> Part {
    let comb = centered_cube(
        "cleanroom_pressure_cascade_service_strain_relief_comb",
        SERVICE_X - 46.0,
        36.0,
        28.0,
    )
    .translate(0.0, -SERVICE_Y / 2.0 + 54.0, 22.0 + 14.0);
    let mut slots = Part::empty("cleanroom_pressure_cascade_service_strain_relief_slots");
    for i in 0..SERVICE_STRAIN_RELIEF_SLOTS {
        slots = slots
            + centered_cube(
                format!("cleanroom_pressure_cascade_service_strain_relief_slot_{i}"),
                12.0,
                40.0,
                30.0,
            )
            .translate(
                centered_index(i, SERVICE_STRAIN_RELIEF_SLOTS, 20.0),
                -SERVICE_Y / 2.0 + 54.0,
                22.0 + 14.0,
            );
    }
    comb - slots
}

fn service_drip_trays() -> Part {
    let mut trays = Part::empty("cleanroom_pressure_cascade_service_drip_trays");
    for i in 0..SERVICE_DRIP_TRAY_COUNT {
        let y = -SERVICE_Y / 2.0 + 106.0 + i as f64 * 52.0;
        let tray = centered_cube(
            format!("cleanroom_pressure_cascade_service_drip_tray_{i}"),
            SERVICE_X - 48.0,
            24.0,
            10.0,
        )
        .translate(0.0, y, 22.0 + 5.0);
        let gutter = centered_cube(
            format!("cleanroom_pressure_cascade_service_drip_tray_gutter_cut_{i}"),
            SERVICE_X - 78.0,
            10.0,
            6.0,
        )
        .translate(0.0, y, 22.0 + 8.0);
        trays = trays + (tray - gutter);
    }
    trays
}

fn service_class_label_lands() -> Part {
    let mut lands = Part::empty("cleanroom_pressure_cascade_service_class_label_lands");
    for i in 0..SERVICE_PENETRATION_COUNT {
        let (x, z) = service_penetration_pose(i);
        lands = lands
            + centered_cube(
                format!("cleanroom_pressure_cascade_service_class_label_land_{i}"),
                34.0,
                6.0,
                12.0,
            )
            .translate(x, SERVICE_Y / 2.0 - 74.0, z - 26.0);
    }
    lands
}

fn sample_cassette_exposure_rack() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_sample_cassette_rack_base",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);
    let recess = centered_cube(
        "cleanroom_pressure_cascade_sample_cassette_rack_recess_cut",
        SAMPLE_X - 54.0,
        SAMPLE_Y - 42.0,
        8.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z - 4.0);

    base - recess + sample_cassette_nests() + sample_witness_strips() + sample_zone_dividers()
}

fn sample_cassette_nests() -> Part {
    let mut nests = Part::empty("cleanroom_pressure_cascade_sample_cassette_nests");
    for i in 0..SAMPLE_CASSETTE_COUNT {
        let col = i % SAMPLE_COLS;
        let row = i / SAMPLE_COLS;
        let x = centered_index(col, SAMPLE_COLS, SAMPLE_PITCH_X);
        let y = centered_index(row, SAMPLE_ROWS, SAMPLE_PITCH_Y);
        let nest = centered_cube(
            format!("cleanroom_pressure_cascade_sample_cassette_nest_{i}"),
            SAMPLE_CASSETTE_X,
            SAMPLE_CASSETTE_Y,
            SAMPLE_CASSETTE_Z,
        )
        .translate(x, y, SAMPLE_Z + SAMPLE_CASSETTE_Z / 2.0);
        let pocket = centered_cube(
            format!("cleanroom_pressure_cascade_sample_cassette_pocket_cut_{i}"),
            REVC_CHIP_LENGTH + 3.0,
            REVC_CHIP_WIDTH + 3.0,
            REVC_TOTAL_HEIGHT + 4.0,
        )
        .translate(x, y, SAMPLE_Z + SAMPLE_CASSETTE_Z / 2.0 + 2.0);
        let inlet_flag = centered_cube(
            format!("cleanroom_pressure_cascade_sample_cassette_inlet_flag_{i}"),
            26.0,
            8.0,
            12.0,
        )
        .translate(
            x - SAMPLE_CASSETTE_X / 2.0 + 22.0,
            y + SAMPLE_CASSETTE_Y / 2.0 - 10.0,
            SAMPLE_Z + SAMPLE_CASSETTE_Z + 6.0,
        );
        nests = nests + (nest - pocket) + inlet_flag;
    }
    nests
}

fn sample_witness_strips() -> Part {
    let mut strips = Part::empty("cleanroom_pressure_cascade_sample_witness_strips");
    for i in 0..SAMPLE_WITNESS_STRIP_COUNT {
        let col = i % SAMPLE_COLS;
        let row = i / SAMPLE_COLS;
        let x = centered_index(col, SAMPLE_COLS, SAMPLE_PITCH_X);
        let y = centered_index(row, SAMPLE_ROWS, SAMPLE_PITCH_Y) - SAMPLE_CASSETTE_Y / 2.0 - 12.0;
        let strip = centered_cube(
            format!("cleanroom_pressure_cascade_sample_witness_strip_{i}"),
            92.0,
            10.0,
            5.0,
        )
        .translate(x, y, SAMPLE_Z + 2.5);
        strips = strips + strip;
    }
    strips
}

fn sample_zone_dividers() -> Part {
    let mut dividers = Part::empty("cleanroom_pressure_cascade_sample_zone_dividers");
    for i in 1..PRESSURE_STEP_COUNT {
        let x = centered_index(i, PRESSURE_STEP_COUNT, SAMPLE_PITCH_X) - SAMPLE_PITCH_X / 2.0;
        dividers = dividers
            + centered_cube(
                format!("cleanroom_pressure_cascade_sample_zone_divider_{i}"),
                8.0,
                SAMPLE_Y - 38.0,
                34.0,
            )
            .translate(x, 0.0, SAMPLE_Z + 17.0);
    }
    dividers
}

fn recovery_filter_coupon_lane() -> Part {
    let lane = centered_cube(
        "cleanroom_pressure_cascade_recovery_filter_coupon_lane",
        FILTER_X,
        FILTER_Y,
        FILTER_Z,
    )
    .translate(0.0, 0.0, FILTER_Z / 2.0);

    lane + filter_coupon_pockets() + upstream_downstream_labels() + filter_disposition_stops()
}

fn filter_coupon_pockets() -> Part {
    let mut pockets = Part::empty("cleanroom_pressure_cascade_filter_coupon_pockets");
    for i in 0..FILTER_COUPON_COUNT {
        let x = centered_index(i, FILTER_COUPON_COUNT, 72.0);
        let pocket = centered_cylinder(
            format!("cleanroom_pressure_cascade_filter_coupon_pocket_{i}"),
            FILTER_COUPON_D / 2.0,
            10.0,
            40,
        )
        .translate(x, 0.0, FILTER_Z + 5.0);
        let center = centered_cylinder(
            format!("cleanroom_pressure_cascade_filter_coupon_center_relief_{i}"),
            FILTER_COUPON_D / 2.0 - 8.0,
            12.0,
            36,
        )
        .translate(x, 0.0, FILTER_Z + 5.0);
        pockets = pockets + (pocket - center);
    }
    pockets
}

fn upstream_downstream_labels() -> Part {
    let upstream = centered_cube(
        "cleanroom_pressure_cascade_filter_upstream_label_land",
        190.0,
        20.0,
        5.0,
    )
    .translate(-110.0, FILTER_Y / 2.0 - 22.0, FILTER_Z + 2.5);
    let downstream = centered_cube(
        "cleanroom_pressure_cascade_filter_downstream_label_land",
        190.0,
        20.0,
        5.0,
    )
    .translate(110.0, FILTER_Y / 2.0 - 22.0, FILTER_Z + 2.5);
    upstream + downstream
}

fn filter_disposition_stops() -> Part {
    let mut stops = Part::empty("cleanroom_pressure_cascade_filter_disposition_stops");
    for i in 0..4 {
        let x = centered_index(i, 4, 132.0);
        stops = stops
            + centered_cube(
                format!("cleanroom_pressure_cascade_filter_coupon_stop_{i}"),
                10.0,
                FILTER_Y - 24.0,
                22.0,
            )
            .translate(x, 0.0, FILTER_Z + 11.0);
    }
    stops
}

fn barcode_protocol_lands() -> Part {
    let base = centered_cube(
        "cleanroom_pressure_cascade_barcode_protocol_base",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);

    base + barcode_lands() + protocol_card_lands() + witness_seal_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("cleanroom_pressure_cascade_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = centered_index(i % 4, 4, 60.0);
        let y = centered_index(i / 4, 2, 30.0);
        lands = lands
            + centered_cube(
                format!("cleanroom_pressure_cascade_barcode_land_{i}"),
                46.0,
                14.0,
                4.0,
            )
            .translate(x, y, TRACE_Z + 2.0);
    }
    lands
}

fn protocol_card_lands() -> Part {
    let mut cards = Part::empty("cleanroom_pressure_cascade_protocol_card_lands");
    for i in 0..PROTOCOL_CARD_COUNT {
        cards = cards
            + centered_cube(
                format!("cleanroom_pressure_cascade_protocol_card_land_{i}"),
                74.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(i, PROTOCOL_CARD_COUNT, 90.0),
                -TRACE_Y / 2.0 + 14.0,
                TRACE_Z + 2.0,
            );
    }
    cards
}

fn witness_seal_lands() -> Part {
    let mut seals = Part::empty("cleanroom_pressure_cascade_witness_seal_lands");
    for i in 0..WITNESS_SEAL_LAND_COUNT {
        seals = seals
            + centered_cylinder(
                format!("cleanroom_pressure_cascade_witness_seal_land_{i}"),
                10.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, WITNESS_SEAL_LAND_COUNT, 42.0),
                TRACE_Y / 2.0 - 14.0,
                TRACE_Z + 2.0,
            );
    }
    seals
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "cleanroom_pressure_cascade_evidence_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        "cleanroom_pressure_cascade_evidence_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        "cleanroom_pressure_cascade_evidence_camera_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    left_post + right_post + beam + camera_pods() + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("cleanroom_pressure_cascade_evidence_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let x = centered_index(i, CAMERA_POD_COUNT, BRIDGE_SPAN_X / 5.5);
        let pod = centered_cube(
            format!("cleanroom_pressure_cascade_evidence_camera_pod_{i}"),
            56.0,
            38.0,
            24.0,
        )
        .translate(x, -BRIDGE_POST_Y / 2.0 - 18.0, BRIDGE_UNDERSIDE_Z - 12.0);
        let lens = centered_cylinder(
            format!("cleanroom_pressure_cascade_evidence_camera_lens_{i}"),
            12.0,
            8.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BRIDGE_POST_Y / 2.0 - 40.0, BRIDGE_UNDERSIDE_Z - 12.0);
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty("cleanroom_pressure_cascade_evidence_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        let x = centered_index(i, LIGHT_BAR_COUNT, BRIDGE_SPAN_X / 4.8);
        bars = bars
            + centered_cube(
                format!("cleanroom_pressure_cascade_evidence_light_bar_{i}"),
                180.0,
                12.0,
                10.0,
            )
            .translate(x, BRIDGE_POST_Y / 2.0 + 12.0, BRIDGE_UNDERSIDE_Z - 16.0);
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "cleanroom_pressure_cascade_front_robot_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "cleanroom_pressure_cascade_rear_service_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "cleanroom_pressure_cascade_right_filter_withdrawal_keepout_gauge",
        10.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_WITHDRAWAL_X,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let top = centered_cube(
        "cleanroom_pressure_cascade_top_filter_lift_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, TOP_FILTER_LIFT_CLEARANCE_Z);

    front + rear + right + top + keepout_corner_posts()
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty("cleanroom_pressure_cascade_keepout_corner_posts");
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
            + centered_cylinder(
                format!("cleanroom_pressure_cascade_keepout_corner_post_{i}"),
                6.0,
                42.0,
                20,
            )
            .translate(*x, *y, 21.0);
    }
    posts
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

fn cascade_total_drop_pa() -> f64 {
    PRESSURE_SETPOINTS_PA[0] - PRESSURE_SETPOINTS_PA[PRESSURE_STEP_COUNT - 1]
}

fn minimum_adjacent_pressure_step_pa() -> f64 {
    PRESSURE_SETPOINTS_PA
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .fold(f64::INFINITY, f64::min)
}

fn smoke_injection_open_area_mm2() -> f64 {
    SMOKE_PORT_COUNT as f64 * PI * (SMOKE_PORT_D / 2.0).powi(2)
}

fn recovery_open_area_mm2() -> f64 {
    RETURN_SLOT_COUNT as f64 * RETURN_SLOT_X * RETURN_SLOT_Y
}

fn pressure_sensor_port_count() -> usize {
    SENSOR_TOWER_COUNT * SENSOR_PORTS_PER_TOWER
}

fn tallest_sensor_tower_height() -> f64 {
    SENSOR_TOWER_HEIGHTS.into_iter().fold(0.0, f64::max)
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
            assert!(path
                .starts_with("output/closed_cleanroom_pressure_cascade_smoke_recovery_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_cleanroom_features_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for expected in [
            "pressure_step_plenums",
            "smoke_injection_manifold",
            "return_exhaust_recovery_path",
            "sensor_tower_array",
            "door_transfer_port_leak_challenge",
            "service_line_penetration_bulkhead",
            "sample_cassette_exposure_rack",
            "recovery_filter_coupon_lane",
            "barcode_protocol_lands",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
            "assembly",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn pressure_cascade_steps_are_defensible() {
        assert_design_constraints();
        assert_eq!(PRESSURE_STEP_COUNT, 4);
        assert_eq!(PRESSURE_SETPOINTS_PA, [30.0, 20.0, 10.0, 0.0]);
        assert_eq!(cascade_total_drop_pa(), 30.0);
        assert_eq!(minimum_adjacent_pressure_step_pa(), 10.0);
        assert_eq!(TRANSITION_GATE_COUNT, 3);
        assert_eq!(PRESSURE_TAP_COUNT, 8);
    }

    #[test]
    fn smoke_recovery_capacity_exceeds_injection_capacity() {
        assert_eq!(SMOKE_PORT_COUNT, 16);
        assert_eq!(RETURN_SLOT_COUNT, 10);
        assert!(recovery_open_area_mm2() > smoke_injection_open_area_mm2() * 5.0);
        assert_eq!(HEPA_RECOVERY_COUPON_COUNT, 3);
        assert_eq!(RECOVERY_TRAP_COUNT, 3);
    }

    #[test]
    fn sensors_service_lines_and_leak_challenges_are_counted() {
        assert_eq!(SENSOR_TOWER_COUNT, 6);
        assert_eq!(pressure_sensor_port_count(), 12);
        assert_eq!(DIFFERENTIAL_PRESSURE_PAIR_COUNT, 3);
        assert_eq!(DOOR_LEAF_COUNT, 2);
        assert_eq!(TRANSFER_PORT_COUNT, 1);
        assert_eq!(LEAK_SHIM_COUNT, 6);
        assert_eq!(INTERLOCK_PIN_COUNT, 4);
        assert_eq!(SERVICE_PENETRATION_COUNT, SERVICE_ROWS * SERVICE_COLS);
    }

    #[test]
    fn sample_and_filter_cassettes_match_cascade_plan() {
        assert_eq!(SAMPLE_CASSETTE_COUNT, 8);
        assert_eq!(SAMPLE_WITNESS_STRIP_COUNT, SAMPLE_CASSETTE_COUNT);
        assert_eq!(FILTER_COUPON_COUNT, 6);
        assert_eq!(FILTER_UPSTREAM_COUNT, 3);
        assert_eq!(FILTER_DOWNSTREAM_COUNT, 3);
        assert!(SAMPLE_CASSETTE_X > REVC_CHIP_LENGTH + 18.0);
        assert!(SAMPLE_CASSETTE_Y > REVC_CHIP_WIDTH + 10.0);
        assert!(SAMPLE_CASSETTE_Z > REVC_TOTAL_HEIGHT + 8.0);
    }

    #[test]
    fn layout_rectangles_fit_without_floor_overlap() {
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
        assert!(BRIDGE_UNDERSIDE_Z > tallest_sensor_tower_height() + SENSOR_BASE_Z + 70.0);
    }
}
