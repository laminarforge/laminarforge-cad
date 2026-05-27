use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed module thermal fault injection station.
//
// Intent:
// - Hold sealed, cell-free culture-module dummies in a contained validation
//   fixture so module/incubator thermal safety interfaces can be characterized.
// - Reserve packaging envelopes for heater-fault coupons, independent
//   thermistors/loggers, thermal fuse or overtemp switch lands, isolation
//   shields, emergency disconnect/service bulkhead placeholders, traceability,
//   disposition lanes, clean/used segregation, evidence imaging, and robot or
//   service keepout gauges.
//
// This is packaging/interface CAD only. It intentionally does not define
// energized heater control, hazardous operating steps, setpoints, acceptance
// criteria, biological use, or validation release decisions.

const OUTPUT_PREFIX: &str = "output/closed_module_thermal_runaway_fault_injection_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_module_thermal_runaway_fault_injection_station_base_containment_deck.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_sealed_module_dummy_nest.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_heater_fault_coupon_placeholder_pockets.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_independent_thermistor_logger_pockets.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_thermal_fuse_overtemp_switch_lands.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_isolation_shield_panels.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_emergency_disconnect_service_bulkhead.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_barcode_certificate_lands.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_release_hold_reject_lanes.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_clean_used_segregation.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_evidence_camera_bridge.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_robot_service_keepout_gauges.stl",
    "output/closed_module_thermal_runaway_fault_injection_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_containment_deck",
    "sealed_module_dummy_nest",
    "heater_fault_coupon_placeholder_pockets",
    "independent_thermistor_logger_pockets",
    "thermal_fuse_overtemp_switch_lands",
    "isolation_shield_panels",
    "emergency_disconnect_service_bulkhead",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 6;

const MODULE_COLS: usize = 3;
const MODULE_ROWS: usize = 2;
const MODULE_GUTTER: f64 = 10.0;
const MODULE_ARRAY_X: f64 =
    MODULE_COLS as f64 * REVC_CHIP_LENGTH + (MODULE_COLS as f64 - 1.0) * MODULE_GUTTER;
const MODULE_ARRAY_Y: f64 =
    MODULE_ROWS as f64 * REVC_CHIP_WIDTH + (MODULE_ROWS as f64 - 1.0) * MODULE_GUTTER;
const MODULE_DUMMY_X: f64 = MODULE_ARRAY_X + 96.0;
const MODULE_DUMMY_Y: f64 = MODULE_ARRAY_Y + 92.0;
const MODULE_DUMMY_Z: f64 = REVC_TOTAL_HEIGHT + 34.0;
const MODULE_NEST_X: f64 = MODULE_DUMMY_X + 70.0;
const MODULE_NEST_Y: f64 = MODULE_DUMMY_Y + 58.0;
const MODULE_NEST_Z: f64 = 38.0;
const MODULE_NEST_POS: (f64, f64) = (-270.0, 85.0);
const MODULE_CLEARANCE: f64 = 1.6;
const NEST_RAIL_W: f64 = 18.0;
const NEST_DATUM_PIN_COUNT: usize = 4;

const COUPON_BLOCK_X: f64 = 420.0;
const COUPON_BLOCK_Y: f64 = 180.0;
const COUPON_BLOCK_Z: f64 = 44.0;
const COUPON_BLOCK_POS: (f64, f64) = (250.0, 230.0);
const HEATER_COUPON_COUNT: usize = 6;
const COUPON_POCKET_X: f64 = 86.0;
const COUPON_POCKET_Y: f64 = 44.0;
const COUPON_POCKET_DEPTH: f64 = 18.0;
const COUPON_PITCH_X: f64 = 118.0;
const COUPON_PITCH_Y: f64 = 68.0;
const COUPON_KEY_W: f64 = 8.0;

const SENSOR_BLOCK_X: f64 = 370.0;
const SENSOR_BLOCK_Y: f64 = 150.0;
const SENSOR_BLOCK_Z: f64 = 42.0;
const SENSOR_BLOCK_POS: (f64, f64) = (280.0, 35.0);
const THERMISTOR_POCKET_COUNT: usize = 8;
const THERMISTOR_PITCH_X: f64 = 82.0;
const THERMISTOR_PITCH_Y: f64 = 48.0;
const THERMISTOR_WELL_D: f64 = 9.0;
const LOGGER_POCKET_COUNT: usize = 2;
const LOGGER_POCKET_X: f64 = 112.0;
const LOGGER_POCKET_Y: f64 = 44.0;

const FUSE_BLOCK_X: f64 = 340.0;
const FUSE_BLOCK_Y: f64 = 130.0;
const FUSE_BLOCK_Z: f64 = 26.0;
const FUSE_BLOCK_POS: (f64, f64) = (280.0, -145.0);
const THERMAL_FUSE_LANDS: usize = 4;
const OVERTEMP_SWITCH_LANDS: usize = 3;
const FUSE_LAND_X: f64 = 64.0;
const FUSE_LAND_Y: f64 = 22.0;
const SWITCH_LAND_D: f64 = 34.0;

const SHIELD_X: f64 = 1010.0;
const SHIELD_Y: f64 = 500.0;
const SHIELD_Z: f64 = 174.0;
const SHIELD_POS: (f64, f64) = (-18.0, 108.0);
const SHIELD_PANEL_W: f64 = 12.0;
const SHIELD_WINDOW_COUNT: usize = 10;
const SHIELD_POST_COUNT: usize = 6;

const BULKHEAD_X: f64 = 900.0;
const BULKHEAD_Y: f64 = 34.0;
const BULKHEAD_Z: f64 = 136.0;
const BULKHEAD_POS: (f64, f64) = (0.0, 384.0);
const SERVICE_PORT_COUNT: usize = 10;
const SERVICE_PORT_D: f64 = 18.0;
const DISCONNECT_LAND_D: f64 = 58.0;

const TRACE_PLATE_X: f64 = 430.0;
const TRACE_PLATE_Y: f64 = 100.0;
const TRACE_PLATE_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (-140.0, -315.0);
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 4;

const LANE_PLATE_X: f64 = 430.0;
const LANE_PLATE_Y: f64 = 142.0;
const LANE_PLATE_Z: f64 = 28.0;
const LANE_POS: (f64, f64) = (330.0, -315.0);
const DISPOSITION_LANES: usize = 3;
const LANE_SLOT_COUNT: usize = 9;
const LANE_PITCH_X: f64 = 136.0;
const LANE_SLOT_PITCH_Y: f64 = 36.0;
const LANE_WALL_W: f64 = 8.0;

const SEGREGATION_X: f64 = 135.0;
const SEGREGATION_Y: f64 = 260.0;
const SEGREGATION_Z: f64 = 58.0;
const SEGREGATION_POS: (f64, f64) = (-490.0, -260.0);
const CLEAN_USED_BARRIER_Z: f64 = 92.0;
const SEGREGATION_GAP_MIN: f64 = 54.0;

const CAMERA_BRIDGE_X: f64 = 610.0;
const CAMERA_BRIDGE_Y: f64 = 250.0;
const CAMERA_BRIDGE_Z: f64 = 220.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (-285.0, 68.0);
const CAMERA_PAD_COUNT: usize = 4;
const CAMERA_PAD_X: f64 = 78.0;
const CAMERA_PAD_Y: f64 = 48.0;
const CAMERA_FIELD_X: f64 = 480.0;
const CAMERA_FIELD_Y: f64 = 158.0;

const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONE_COUNT: usize = 7;
const FRONT_ROBOT_SWEEP: f64 = 360.0;
const REAR_SERVICE_SWEEP: f64 = 250.0;
const FRONT_ROBOT_CLEARANCE_MIN: f64 = 36.0;
const REAR_SERVICE_CLEARANCE_MIN: f64 = 24.0;
const LEFT_MODULE_LOAD_SWEEP: f64 = 150.0;
const RIGHT_SERVICE_SWEEP: f64 = 160.0;
const VERTICAL_SHIELD_LIFT_CLEARANCE: f64 = 230.0;

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = sealed_module_dummy_nest();
    export(OUTPUTS[1], &nest);

    let coupons = heater_fault_coupon_placeholder_pockets();
    export(OUTPUTS[2], &coupons);

    let sensors = independent_thermistor_logger_pockets();
    export(OUTPUTS[3], &sensors);

    let fuses = thermal_fuse_overtemp_switch_lands();
    export(OUTPUTS[4], &fuses);

    let shields = isolation_shield_panels();
    export(OUTPUTS[5], &shields);

    let bulkhead = emergency_disconnect_service_bulkhead();
    export(OUTPUTS[6], &bulkhead);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        deck + nest.translate(
            MODULE_NEST_POS.0,
            MODULE_NEST_POS.1,
            on_deck_z(MODULE_NEST_Z),
        ) + coupons.translate(
            COUPON_BLOCK_POS.0,
            COUPON_BLOCK_POS.1,
            on_deck_z(COUPON_BLOCK_Z),
        ) + sensors.translate(
            SENSOR_BLOCK_POS.0,
            SENSOR_BLOCK_POS.1,
            on_deck_z(SENSOR_BLOCK_Z),
        ) + fuses.translate(FUSE_BLOCK_POS.0, FUSE_BLOCK_POS.1, on_deck_z(FUSE_BLOCK_Z))
            + shields.translate(SHIELD_POS.0, SHIELD_POS.1, on_deck_z(SHIELD_Z))
            + bulkhead.translate(BULKHEAD_POS.0, BULKHEAD_POS.1, on_deck_z(BULKHEAD_Z))
            + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_PLATE_Z))
            + lanes.translate(LANE_POS.0, LANE_POS.1, on_deck_z(LANE_PLATE_Z))
            + segregation.translate(
                SEGREGATION_POS.0,
                SEGREGATION_POS.1,
                on_deck_z(SEGREGATION_Z),
            )
            + bridge.translate(
                CAMERA_BRIDGE_POS.0,
                CAMERA_BRIDGE_POS.1,
                on_deck_z(CAMERA_BRIDGE_Z),
            )
            + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed module thermal fault injection station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck with raised rim and locator sockets"
    );
    println!(
        "  Cell-free module nest:  {MODULE_COLS}x{MODULE_ROWS} sealed module dummy envelope with {MODULE_CLEARANCE:.1}mm nominal pocket clearance"
    );
    println!(
        "  Fault placeholders:     {HEATER_COUPON_COUNT} heater coupon placeholder pockets, {THERMISTOR_POCKET_COUNT} independent thermistor wells, {LOGGER_POCKET_COUNT} logger pockets"
    );
    println!(
        "  Safety interfaces:      {THERMAL_FUSE_LANDS} fuse lands, {OVERTEMP_SWITCH_LANDS} overtemp switch lands, isolation shield panels, emergency disconnect/service bulkhead placeholders"
    );
    println!(
        "  Evidence/disposition:   {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, release/hold/reject lanes, clean/used segregation, evidence camera bridge"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(HEATER_COUPON_COUNT, 6);
    assert_eq!(THERMISTOR_POCKET_COUNT, 8);
    assert_eq!(NEST_DATUM_PIN_COUNT, 4);
    assert_eq!(DATUM_TARGET_COUNT, 6);
    assert_eq!(SHIELD_POST_COUNT, 6);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 7);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));

    for (name, rect) in non_overlapping_fixture_rects() {
        assert!(
            rect_inside(rect, station_inner_rect(), 8.0),
            "{name} exceeds contained deck envelope"
        );
    }

    let checked = non_overlapping_fixture_rects();
    for i in 0..checked.len() {
        for j in (i + 1)..checked.len() {
            assert!(
                !rects_overlap(checked[i].1, checked[j].1),
                "{} overlaps {}",
                checked[i].0,
                checked[j].0
            );
        }
    }

    assert!(MODULE_DUMMY_X > MODULE_ARRAY_X + 90.0);
    assert!(MODULE_DUMMY_Y > MODULE_ARRAY_Y + 88.0);
    assert!(MODULE_DUMMY_Z > REVC_TOTAL_HEIGHT + 30.0);
    assert!(MODULE_NEST_X > MODULE_DUMMY_X + 60.0);
    assert!(MODULE_NEST_Y > MODULE_DUMMY_Y + 50.0);
    assert!(COUPON_POCKET_DEPTH < COUPON_BLOCK_Z / 2.0);
    assert!(shield_clearance_above_module() > 80.0);
    assert!(segregation_gap_to_traceability() >= SEGREGATION_GAP_MIN);
    assert!(VERTICAL_SHIELD_LIFT_CLEARANCE > SHIELD_Z);
    assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE_MIN);
    assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE_MIN);
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "thermal_fault_station_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let spill_basin = centered_cube(
        "thermal_fault_station_secondary_containment_recess_cut",
        STATION_X - 118.0,
        STATION_Y - 116.0,
        SOCKET_DEPTH + 2.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.4);
    let front_drain = centered_cylinder(
        "thermal_fault_station_front_service_drain_placeholder_cut",
        10.0 / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - spill_basin - front_drain - locator_sockets() - mounting_holes()
        + containment_rims()
        + deck_zone_dividers()
        + robot_datum_targets()
        + deck_witness_ribs()
}

fn locator_sockets() -> Part {
    let mut sockets = Part::empty("thermal_fault_station_locator_sockets");
    for (name, rect) in fixture_socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("thermal_fault_station_{name}_locator_socket"),
                rect.w + 8.0,
                rect.h + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(rect.x, rect.y, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3);
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("thermal_fault_station_mounting_holes");
    for (i, (x, y)) in mount_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("thermal_fault_station_m6_mount_clearance_{i}"),
                6.8 / 2.0,
                BASE_Z + 5.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("thermal_fault_station_m6_service_slot_{i}"),
                26.0,
                7.2,
                BASE_Z + 5.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn mount_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 2.0 - 64.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 2.0 + 64.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 60.0),
        (-STATION_X / 4.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 4.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 4.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 60.0),
    ]
}

fn containment_rims() -> Part {
    let left = centered_cube(
        "thermal_fault_station_left_high_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "thermal_fault_station_right_high_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "thermal_fault_station_rear_service_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "thermal_fault_station_front_robot_low_containment_lip",
        STATION_X - 160.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 12.0);

    left + right + rear + front
}

fn deck_zone_dividers() -> Part {
    let rear_row = centered_cube(
        "thermal_fault_station_rear_fault_coupon_row_divider",
        STATION_X - 160.0,
        10.0,
        28.0,
    )
    .translate(0.0, 130.0, BASE_Z / 2.0 + 14.0);
    let sensor_row = centered_cube(
        "thermal_fault_station_sensor_safety_row_divider",
        520.0,
        10.0,
        25.0,
    )
    .translate(270.0, -55.0, BASE_Z / 2.0 + 12.5);
    let disposition_row = centered_cube(
        "thermal_fault_station_front_disposition_row_divider",
        STATION_X - 180.0,
        8.0,
        22.0,
    )
    .translate(0.0, -235.0, BASE_Z / 2.0 + 11.0);
    let clean_used_column = centered_cube(
        "thermal_fault_station_clean_used_column_divider",
        10.0,
        260.0,
        28.0,
    )
    .translate(-410.0, -260.0, BASE_Z / 2.0 + 14.0);
    rear_row + sensor_row + disposition_row + clean_used_column
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("thermal_fault_station_robot_datum_targets");
    for (i, (x, y)) in [
        (-485.0, 300.0),
        (485.0, 300.0),
        (-485.0, -365.0),
        (485.0, -365.0),
        (-72.0, -365.0),
        (0.0, 300.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("thermal_fault_station_robot_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.5,
            );
    }
    targets
}

fn deck_witness_ribs() -> Part {
    let mut ribs = Part::empty("thermal_fault_station_containment_witness_ribs");
    for (i, y) in [-182.0, -98.0, -14.0, 70.0, 154.0, 238.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("thermal_fault_station_witness_rib_{i}"),
                STATION_X - 180.0,
                4.0,
                5.0,
            )
            .translate(0.0, *y, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn sealed_module_dummy_nest() -> Part {
    let body = centered_cube(
        "thermal_fault_station_sealed_module_dummy_nest_body",
        MODULE_NEST_X,
        MODULE_NEST_Y,
        MODULE_NEST_Z,
    );
    let module_relief = centered_cube(
        "thermal_fault_station_cell_free_sealed_module_dummy_relief_cut",
        MODULE_DUMMY_X + MODULE_CLEARANCE * 2.0,
        MODULE_DUMMY_Y + MODULE_CLEARANCE * 2.0,
        MODULE_NEST_Z + 4.0,
    )
    .translate(0.0, 0.0, MODULE_NEST_Z / 2.0 - 10.0);
    let underside_window = centered_cube(
        "thermal_fault_station_nest_underside_air_gap_window_cut",
        MODULE_DUMMY_X - 92.0,
        MODULE_DUMMY_Y - 78.0,
        MODULE_NEST_Z + 6.0,
    )
    .translate(0.0, 0.0, -8.0);

    body - module_relief - underside_window
        + nest_datum_rails()
        + nest_datum_pins()
        + module_dummy_envelope_markers()
}

fn nest_datum_rails() -> Part {
    let left = centered_cube(
        "thermal_fault_station_nest_left_hard_datum_rail",
        NEST_RAIL_W,
        MODULE_NEST_Y - 42.0,
        MODULE_NEST_Z + 20.0,
    )
    .translate(
        -MODULE_DUMMY_X / 2.0 - MODULE_CLEARANCE - NEST_RAIL_W / 2.0,
        0.0,
        10.0,
    );
    let rear = centered_cube(
        "thermal_fault_station_nest_rear_hard_datum_rail",
        MODULE_DUMMY_X + 36.0,
        NEST_RAIL_W,
        MODULE_NEST_Z + 18.0,
    )
    .translate(
        0.0,
        MODULE_DUMMY_Y / 2.0 + MODULE_CLEARANCE + NEST_RAIL_W / 2.0,
        9.0,
    );
    let right_soft = centered_cube(
        "thermal_fault_station_nest_right_soft_capture_rail",
        12.0,
        MODULE_NEST_Y - 88.0,
        MODULE_NEST_Z + 12.0,
    )
    .translate(MODULE_DUMMY_X / 2.0 + 23.0, -8.0, 6.0);
    let front_entry = centered_cube(
        "thermal_fault_station_nest_front_robot_entry_lip",
        MODULE_DUMMY_X + 18.0,
        12.0,
        18.0,
    )
    .translate(0.0, -MODULE_NEST_Y / 2.0 + 14.0, MODULE_NEST_Z / 2.0 + 9.0);
    left + rear + right_soft + front_entry
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("thermal_fault_station_nest_datum_pin_bosses");
    for (i, (x, y)) in [
        (-MODULE_DUMMY_X / 2.0 - 28.0, -MODULE_DUMMY_Y / 2.0 - 12.0),
        (-MODULE_DUMMY_X / 2.0 - 28.0, MODULE_DUMMY_Y / 2.0 + 12.0),
        (MODULE_DUMMY_X / 2.0 + 28.0, -MODULE_DUMMY_Y / 2.0 - 12.0),
        (MODULE_DUMMY_X / 2.0 + 28.0, MODULE_DUMMY_Y / 2.0 + 12.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("thermal_fault_station_nest_datum_pin_boss_{i}"),
                17.0 / 2.0,
                8.0,
                32,
            )
            .translate(*x, *y, MODULE_NEST_Z / 2.0 + 4.0)
            - centered_cylinder(
                format!("thermal_fault_station_nest_datum_pin_bore_{i}"),
                5.2 / 2.0,
                12.0,
                24,
            )
            .translate(*x, *y, MODULE_NEST_Z / 2.0 + 4.0);
    }
    pins
}

fn module_dummy_envelope_markers() -> Part {
    let mut markers = Part::empty("thermal_fault_station_cell_free_module_dummy_markers");
    for row in 0..MODULE_ROWS {
        for col in 0..MODULE_COLS {
            let x = centered_index(col, MODULE_COLS, REVC_CHIP_LENGTH + MODULE_GUTTER);
            let y = centered_index(row, MODULE_ROWS, REVC_CHIP_WIDTH + MODULE_GUTTER);
            markers = markers
                + centered_cube(
                    format!("thermal_fault_station_module_dummy_outline_{row}_{col}"),
                    REVC_CHIP_LENGTH - 26.0,
                    3.0,
                    4.0,
                )
                .translate(
                    x,
                    y - REVC_CHIP_WIDTH / 2.0 + 15.0,
                    MODULE_NEST_Z / 2.0 + 2.0,
                )
                + centered_cube(
                    format!("thermal_fault_station_module_dummy_cross_line_{row}_{col}"),
                    3.0,
                    REVC_CHIP_WIDTH - 24.0,
                    4.0,
                )
                .translate(x, y, MODULE_NEST_Z / 2.0 + 2.0);
        }
    }
    markers
}

fn heater_fault_coupon_placeholder_pockets() -> Part {
    let body = centered_cube(
        "thermal_fault_station_heater_fault_coupon_placeholder_body",
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
        COUPON_BLOCK_Z,
    );
    let mut cuts = Part::empty("thermal_fault_station_heater_coupon_placeholder_cuts");
    let mut ledges = Part::empty("thermal_fault_station_heater_coupon_retainer_ledges");
    for index in 0..HEATER_COUPON_COUNT {
        let col = index % 3;
        let row = index / 3;
        let x = centered_index(col, 3, COUPON_PITCH_X);
        let y = centered_index(row, 2, COUPON_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("thermal_fault_station_heater_coupon_pocket_cut_{index}"),
                COUPON_POCKET_X,
                COUPON_POCKET_Y,
                COUPON_POCKET_DEPTH + 0.8,
            )
            .translate(x, y, COUPON_BLOCK_Z / 2.0 - COUPON_POCKET_DEPTH / 2.0 + 0.4)
            + centered_cube(
                format!("thermal_fault_station_heater_coupon_key_slot_cut_{index}"),
                COUPON_KEY_W,
                COUPON_POCKET_Y + 18.0,
                COUPON_POCKET_DEPTH + 1.0,
            )
            .translate(
                x - COUPON_POCKET_X / 2.0 + 10.0,
                y,
                COUPON_BLOCK_Z / 2.0 - COUPON_POCKET_DEPTH / 2.0 + 0.5,
            );
        ledges = ledges
            + centered_cube(
                format!("thermal_fault_station_heater_coupon_retainer_land_{index}"),
                COUPON_POCKET_X + 16.0,
                8.0,
                6.0,
            )
            .translate(
                x,
                y + COUPON_POCKET_Y / 2.0 + 8.0,
                COUPON_BLOCK_Z / 2.0 + 3.0,
            );
    }
    body - cuts + ledges + coupon_boundary_fence()
}

fn coupon_boundary_fence() -> Part {
    let rear = centered_cube(
        "thermal_fault_station_heater_coupon_rear_boundary_fence",
        COUPON_BLOCK_X,
        10.0,
        34.0,
    )
    .translate(0.0, COUPON_BLOCK_Y / 2.0 - 5.0, COUPON_BLOCK_Z / 2.0 + 17.0);
    let left = centered_cube(
        "thermal_fault_station_heater_coupon_left_boundary_fence",
        10.0,
        COUPON_BLOCK_Y,
        30.0,
    )
    .translate(
        -COUPON_BLOCK_X / 2.0 + 5.0,
        0.0,
        COUPON_BLOCK_Z / 2.0 + 15.0,
    );
    let right = centered_cube(
        "thermal_fault_station_heater_coupon_right_boundary_fence",
        10.0,
        COUPON_BLOCK_Y,
        30.0,
    )
    .translate(COUPON_BLOCK_X / 2.0 - 5.0, 0.0, COUPON_BLOCK_Z / 2.0 + 15.0);
    rear + left + right
}

fn independent_thermistor_logger_pockets() -> Part {
    let body = centered_cube(
        "thermal_fault_station_independent_thermistor_logger_block",
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    );
    let mut cuts = Part::empty("thermal_fault_station_independent_sensor_pocket_cuts");
    for index in 0..THERMISTOR_POCKET_COUNT {
        let col = index % 4;
        let row = index / 4;
        let x = centered_index(col, 4, THERMISTOR_PITCH_X);
        let y = centered_index(row, 2, THERMISTOR_PITCH_Y) + 18.0;
        cuts = cuts
            + centered_cylinder(
                format!("thermal_fault_station_independent_thermistor_well_cut_{index}"),
                THERMISTOR_WELL_D / 2.0,
                SENSOR_BLOCK_Z + 1.0,
                24,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("thermal_fault_station_independent_thermistor_cable_groove_{index}"),
                7.0,
                62.0,
                9.0,
            )
            .translate(x, y - 34.0, SENSOR_BLOCK_Z / 2.0 - 4.0);
    }

    for index in 0..LOGGER_POCKET_COUNT {
        let x = centered_index(index, LOGGER_POCKET_COUNT, 170.0);
        let y = -SENSOR_BLOCK_Y / 2.0 + 34.0;
        cuts = cuts
            + centered_cube(
                format!("thermal_fault_station_independent_logger_sled_cut_{index}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                18.0,
            )
            .translate(x, y, SENSOR_BLOCK_Z / 2.0 - 9.0);
    }

    body - cuts + sensor_number_tabs() + logger_retainer_clips()
}

fn sensor_number_tabs() -> Part {
    let mut tabs = Part::empty("thermal_fault_station_independent_sensor_number_tabs");
    for index in 0..THERMISTOR_POCKET_COUNT {
        let col = index % 4;
        let row = index / 4;
        let x = centered_index(col, 4, THERMISTOR_PITCH_X);
        let y = centered_index(row, 2, THERMISTOR_PITCH_Y) + 42.0;
        tabs = tabs
            + centered_cube(
                format!("thermal_fault_station_sensor_number_land_{index}"),
                24.0,
                8.0,
                4.0,
            )
            .translate(x, y, SENSOR_BLOCK_Z / 2.0 + 2.0);
    }
    tabs
}

fn logger_retainer_clips() -> Part {
    let mut clips = Part::empty("thermal_fault_station_logger_retainer_clips");
    for index in 0..LOGGER_POCKET_COUNT {
        let x = centered_index(index, LOGGER_POCKET_COUNT, 170.0);
        let y = -SENSOR_BLOCK_Y / 2.0 + 62.0;
        clips = clips
            + centered_cube(
                format!("thermal_fault_station_logger_retainer_clip_{index}"),
                LOGGER_POCKET_X + 24.0,
                8.0,
                9.0,
            )
            .translate(x, y, SENSOR_BLOCK_Z / 2.0 + 4.5);
    }
    clips
}

fn thermal_fuse_overtemp_switch_lands() -> Part {
    let plate = centered_cube(
        "thermal_fault_station_thermal_fuse_overtemp_switch_plate",
        FUSE_BLOCK_X,
        FUSE_BLOCK_Y,
        FUSE_BLOCK_Z,
    );
    let mut lands = Part::empty("thermal_fault_station_thermal_fuse_overtemp_switch_lands");
    for index in 0..THERMAL_FUSE_LANDS {
        let x = centered_index(index, THERMAL_FUSE_LANDS, 74.0);
        lands = lands
            + centered_cube(
                format!("thermal_fault_station_thermal_fuse_land_{index}"),
                FUSE_LAND_X,
                FUSE_LAND_Y,
                7.0,
            )
            .translate(x, 32.0, FUSE_BLOCK_Z / 2.0 + 3.5)
            - centered_cube(
                format!("thermal_fault_station_thermal_fuse_tie_relief_{index}"),
                FUSE_LAND_X - 18.0,
                6.0,
                8.0,
            )
            .translate(x, 32.0, FUSE_BLOCK_Z / 2.0 + 4.0);
    }
    for index in 0..OVERTEMP_SWITCH_LANDS {
        let x = centered_index(index, OVERTEMP_SWITCH_LANDS, 90.0);
        lands = lands
            + centered_cylinder(
                format!("thermal_fault_station_overtemp_switch_land_{index}"),
                SWITCH_LAND_D / 2.0,
                8.0,
                40,
            )
            .translate(x, -32.0, FUSE_BLOCK_Z / 2.0 + 4.0)
            - centered_cylinder(
                format!("thermal_fault_station_overtemp_switch_center_relief_{index}"),
                11.0 / 2.0,
                10.0,
                24,
            )
            .translate(x, -32.0, FUSE_BLOCK_Z / 2.0 + 5.0);
    }
    plate + lands + fuse_guard_rails()
}

fn fuse_guard_rails() -> Part {
    let front = centered_cube(
        "thermal_fault_station_fuse_plate_front_guard_rail",
        FUSE_BLOCK_X - 34.0,
        8.0,
        26.0,
    )
    .translate(0.0, -FUSE_BLOCK_Y / 2.0 + 8.0, FUSE_BLOCK_Z / 2.0 + 13.0);
    let rear = centered_cube(
        "thermal_fault_station_fuse_plate_rear_guard_rail",
        FUSE_BLOCK_X - 34.0,
        8.0,
        26.0,
    )
    .translate(0.0, FUSE_BLOCK_Y / 2.0 - 8.0, FUSE_BLOCK_Z / 2.0 + 13.0);
    front + rear
}

fn isolation_shield_panels() -> Part {
    let rear_panel = centered_cube(
        "thermal_fault_station_rear_isolation_shield_panel",
        SHIELD_X,
        SHIELD_PANEL_W,
        SHIELD_Z,
    )
    .translate(0.0, SHIELD_Y / 2.0 - SHIELD_PANEL_W / 2.0, 0.0);
    let front_panel = centered_cube(
        "thermal_fault_station_front_isolation_shield_panel",
        SHIELD_X,
        SHIELD_PANEL_W,
        SHIELD_Z * 0.58,
    )
    .translate(
        0.0,
        -SHIELD_Y / 2.0 + SHIELD_PANEL_W / 2.0,
        -SHIELD_Z * 0.21,
    );
    let left_panel = centered_cube(
        "thermal_fault_station_left_isolation_shield_panel",
        SHIELD_PANEL_W,
        SHIELD_Y,
        SHIELD_Z,
    )
    .translate(-SHIELD_X / 2.0 + SHIELD_PANEL_W / 2.0, 0.0, 0.0);
    let right_panel = centered_cube(
        "thermal_fault_station_right_isolation_shield_panel",
        SHIELD_PANEL_W,
        SHIELD_Y,
        SHIELD_Z,
    )
    .translate(SHIELD_X / 2.0 - SHIELD_PANEL_W / 2.0, 0.0, 0.0);

    rear_panel + front_panel + left_panel + right_panel + shield_posts()
        - shield_observation_windows()
}

fn shield_posts() -> Part {
    let mut posts = Part::empty("thermal_fault_station_isolation_shield_posts");
    for (i, (x, y)) in [
        (-SHIELD_X / 2.0 + 42.0, -SHIELD_Y / 2.0 + 42.0),
        (SHIELD_X / 2.0 - 42.0, -SHIELD_Y / 2.0 + 42.0),
        (-SHIELD_X / 2.0 + 42.0, SHIELD_Y / 2.0 - 42.0),
        (SHIELD_X / 2.0 - 42.0, SHIELD_Y / 2.0 - 42.0),
        (-120.0, SHIELD_Y / 2.0 - 42.0),
        (250.0, SHIELD_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("thermal_fault_station_isolation_shield_post_{i}"),
                22.0,
                22.0,
                SHIELD_Z + 16.0,
            )
            .translate(*x, *y, 8.0);
    }
    posts
}

fn shield_observation_windows() -> Part {
    let mut windows = Part::empty("thermal_fault_station_isolation_shield_windows");
    for index in 0..SHIELD_WINDOW_COUNT {
        let x = centered_index(index % 5, 5, 166.0);
        let y = if index < 5 {
            SHIELD_Y / 2.0 - SHIELD_PANEL_W / 2.0
        } else {
            -SHIELD_Y / 2.0 + SHIELD_PANEL_W / 2.0
        };
        windows = windows
            + centered_cube(
                format!("thermal_fault_station_isolation_shield_observation_window_cut_{index}"),
                94.0,
                SHIELD_PANEL_W + 3.0,
                46.0,
            )
            .translate(x, y, 8.0);
    }
    windows
}

fn emergency_disconnect_service_bulkhead() -> Part {
    let panel = centered_cube(
        "thermal_fault_station_emergency_disconnect_service_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let disconnect_land = centered_cylinder(
        "thermal_fault_station_emergency_disconnect_land_placeholder",
        DISCONNECT_LAND_D / 2.0,
        9.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-BULKHEAD_X / 2.0 + 95.0, -BULKHEAD_Y / 2.0 - 4.5, 24.0);
    let service_handle = centered_cube(
        "thermal_fault_station_bulkhead_service_handle_land",
        160.0,
        12.0,
        28.0,
    )
    .translate(BULKHEAD_X / 2.0 - 135.0, -BULKHEAD_Y / 2.0 - 6.0, 20.0);
    let cable_trough = centered_cube(
        "thermal_fault_station_bulkhead_strain_relief_trough",
        BULKHEAD_X - 190.0,
        18.0,
        18.0,
    )
    .translate(50.0, -BULKHEAD_Y / 2.0 - 9.0, -36.0);
    panel - bulkhead_port_cuts() + disconnect_land + service_handle + cable_trough
}

fn bulkhead_port_cuts() -> Part {
    let mut cuts = Part::empty("thermal_fault_station_service_bulkhead_port_cuts");
    for index in 0..SERVICE_PORT_COUNT {
        let x = centered_index(index, SERVICE_PORT_COUNT, 68.0) + 48.0;
        cuts = cuts
            + centered_cylinder(
                format!("thermal_fault_station_service_bulkhead_port_cut_{index}"),
                SERVICE_PORT_D / 2.0,
                BULKHEAD_Y + 5.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 22.0);
    }
    cuts
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "thermal_fault_station_barcode_certificate_traceability_plate",
        TRACE_PLATE_X,
        TRACE_PLATE_Y,
        TRACE_PLATE_Z,
    );
    let mut lands = Part::empty("thermal_fault_station_barcode_certificate_lands");
    for index in 0..BARCODE_LANDS {
        let x = centered_index(index % 4, 4, 86.0);
        let y = 18.0 + centered_index(index / 4, 2, 34.0);
        lands = lands
            + centered_cube(
                format!("thermal_fault_station_barcode_land_{index}"),
                68.0,
                20.0,
                3.5,
            )
            .translate(x, y, TRACE_PLATE_Z / 2.0 + 1.75);
    }
    for index in 0..CERTIFICATE_LANDS {
        let x = centered_index(index, CERTIFICATE_LANDS, 88.0);
        lands = lands
            + centered_cube(
                format!("thermal_fault_station_certificate_land_{index}"),
                66.0,
                22.0,
                3.5,
            )
            .translate(x, -34.0, TRACE_PLATE_Z / 2.0 + 1.75);
    }
    plate + lands + traceability_edge_fence()
}

fn traceability_edge_fence() -> Part {
    let rear = centered_cube(
        "thermal_fault_station_traceability_rear_edge_fence",
        TRACE_PLATE_X,
        6.0,
        16.0,
    )
    .translate(0.0, TRACE_PLATE_Y / 2.0 - 3.0, TRACE_PLATE_Z / 2.0 + 8.0);
    let left = centered_cube(
        "thermal_fault_station_traceability_left_edge_fence",
        6.0,
        TRACE_PLATE_Y,
        16.0,
    )
    .translate(-TRACE_PLATE_X / 2.0 + 3.0, 0.0, TRACE_PLATE_Z / 2.0 + 8.0);
    rear + left
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "thermal_fault_station_release_hold_reject_lane_plate",
        LANE_PLATE_X,
        LANE_PLATE_Y,
        LANE_PLATE_Z,
    );
    let mut lane_parts = Part::empty("thermal_fault_station_release_hold_reject_lanes");
    for lane in 0..DISPOSITION_LANES {
        let x = centered_index(lane, DISPOSITION_LANES, LANE_PITCH_X);
        lane_parts = lane_parts + single_disposition_lane(lane, x);
    }
    plate + lane_parts
}

fn single_disposition_lane(lane: usize, x: f64) -> Part {
    let lane_base = centered_cube(
        format!("thermal_fault_station_disposition_lane_{lane}_base"),
        112.0,
        LANE_PLATE_Y - 22.0,
        10.0,
    )
    .translate(x, 0.0, LANE_PLATE_Z / 2.0 + 5.0);
    let left_wall = centered_cube(
        format!("thermal_fault_station_disposition_lane_{lane}_left_wall"),
        LANE_WALL_W,
        LANE_PLATE_Y - 16.0,
        28.0,
    )
    .translate(x - 60.0, 0.0, LANE_PLATE_Z / 2.0 + 14.0);
    let right_wall = centered_cube(
        format!("thermal_fault_station_disposition_lane_{lane}_right_wall"),
        LANE_WALL_W,
        LANE_PLATE_Y - 16.0,
        28.0,
    )
    .translate(x + 60.0, 0.0, LANE_PLATE_Z / 2.0 + 14.0);
    let mut token_slots = Part::empty(format!(
        "thermal_fault_station_disposition_lane_{lane}_token_slots"
    ));
    for slot in 0..(LANE_SLOT_COUNT / DISPOSITION_LANES) {
        let y = centered_index(slot, LANE_SLOT_COUNT / DISPOSITION_LANES, LANE_SLOT_PITCH_Y);
        token_slots = token_slots
            - centered_cube(
                format!("thermal_fault_station_disposition_lane_{lane}_token_slot_{slot}"),
                64.0,
                18.0,
                12.0,
            )
            .translate(x, y, LANE_PLATE_Z / 2.0 + 7.0);
    }
    lane_base + left_wall + right_wall + token_slots
}

fn clean_used_segregation() -> Part {
    let clean_side = centered_cube(
        "thermal_fault_station_clean_side_shadow_board",
        SEGREGATION_X,
        SEGREGATION_Y / 2.0 - 14.0,
        SEGREGATION_Z,
    )
    .translate(0.0, SEGREGATION_Y / 4.0 + 7.0, 0.0);
    let used_side = centered_cube(
        "thermal_fault_station_used_side_lidded_pocket",
        SEGREGATION_X,
        SEGREGATION_Y / 2.0 - 14.0,
        SEGREGATION_Z,
    )
    .translate(0.0, -SEGREGATION_Y / 4.0 - 7.0, 0.0);
    let barrier = centered_cube(
        "thermal_fault_station_clean_used_segregation_tall_barrier",
        SEGREGATION_X + 22.0,
        12.0,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(0.0, 0.0, CLEAN_USED_BARRIER_Z / 2.0 - SEGREGATION_Z / 2.0);
    let used_lid_land = centered_cube(
        "thermal_fault_station_used_fixture_lid_parking_land",
        SEGREGATION_X - 22.0,
        32.0,
        8.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 26.0, SEGREGATION_Z / 2.0 + 4.0);
    clean_side + used_side + barrier + used_lid_land - clean_used_recess_cuts()
}

fn clean_used_recess_cuts() -> Part {
    let clean_shadow_cut = centered_cube(
        "thermal_fault_station_clean_shadow_tool_outline_cut",
        SEGREGATION_X - 42.0,
        76.0,
        16.0,
    )
    .translate(0.0, 62.0, SEGREGATION_Z / 2.0 - 8.0);
    let used_pocket_cut = centered_cube(
        "thermal_fault_station_used_coupon_pocket_cut",
        SEGREGATION_X - 38.0,
        82.0,
        22.0,
    )
    .translate(0.0, -64.0, SEGREGATION_Z / 2.0 - 11.0);
    clean_shadow_cut + used_pocket_cut
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "thermal_fault_station_camera_bridge_left_post",
        28.0,
        52.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 24.0, 0.0, 0.0);
    let right_post = centered_cube(
        "thermal_fault_station_camera_bridge_right_post",
        28.0,
        52.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 24.0, 0.0, 0.0);
    let beam = centered_cube(
        "thermal_fault_station_camera_bridge_overhead_beam",
        CAMERA_BRIDGE_X,
        32.0,
        24.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 12.0);
    let field_frame = centered_cube(
        "thermal_fault_station_camera_bridge_field_of_view_frame",
        CAMERA_FIELD_X,
        CAMERA_FIELD_Y,
        8.0,
    )
    .translate(0.0, -70.0, -CAMERA_BRIDGE_Z / 2.0 + 44.0)
        - centered_cube(
            "thermal_fault_station_camera_bridge_field_of_view_window_cut",
            CAMERA_FIELD_X - 32.0,
            CAMERA_FIELD_Y - 32.0,
            10.0,
        )
        .translate(0.0, -70.0, -CAMERA_BRIDGE_Z / 2.0 + 44.0);

    left_post + right_post + beam + camera_lands() + field_frame
}

fn camera_lands() -> Part {
    let mut lands = Part::empty("thermal_fault_station_evidence_camera_lands");
    for index in 0..CAMERA_PAD_COUNT {
        let x = centered_index(index, CAMERA_PAD_COUNT, 128.0);
        lands = lands
            + centered_cube(
                format!("thermal_fault_station_evidence_camera_pad_{index}"),
                CAMERA_PAD_X,
                CAMERA_PAD_Y,
                10.0,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 5.0)
            - centered_cylinder(
                format!("thermal_fault_station_evidence_camera_lens_relief_{index}"),
                16.0 / 2.0,
                12.0,
                32,
            )
            .translate(x, 0.0, CAMERA_BRIDGE_Z / 2.0 + 6.0);
    }
    lands
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "thermal_fault_station_front_robot_sweep_keepout_gauge",
        STATION_X - 170.0,
        34.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_SWEEP, 0.0);
    let rear_service = centered_cube(
        "thermal_fault_station_rear_service_sweep_keepout_gauge",
        STATION_X - 210.0,
        30.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_SWEEP, 0.0);
    let left_load = centered_cube(
        "thermal_fault_station_left_module_load_keepout_gauge",
        32.0,
        STATION_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 + LEFT_MODULE_LOAD_SWEEP, -15.0, 0.0);
    let right_service = centered_cube(
        "thermal_fault_station_right_service_keepout_gauge",
        32.0,
        STATION_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(STATION_X / 2.0 - RIGHT_SERVICE_SWEEP, -10.0, 0.0);
    let lift = centered_cube(
        "thermal_fault_station_vertical_shield_lift_keepout_gauge",
        SHIELD_X,
        24.0,
        KEEP_OUT_Z,
    )
    .translate(SHIELD_POS.0, SHIELD_POS.1 + SHIELD_Y / 2.0 + 28.0, 0.0);
    front_robot + rear_service + left_load + right_service + lift + keepout_corner_markers()
}

fn keepout_corner_markers() -> Part {
    let mut markers = Part::empty("thermal_fault_station_keepout_corner_markers");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 126.0, -STATION_Y / 2.0 + 98.0),
        (STATION_X / 2.0 - 126.0, -STATION_Y / 2.0 + 98.0),
        (-STATION_X / 2.0 + 126.0, STATION_Y / 2.0 - 142.0),
        (STATION_X / 2.0 - 126.0, STATION_Y / 2.0 - 142.0),
        (
            SHIELD_POS.0 - SHIELD_X / 2.0 + 44.0,
            SHIELD_POS.1 + SHIELD_Y / 2.0 + 28.0,
        ),
        (
            SHIELD_POS.0 + SHIELD_X / 2.0 - 44.0,
            SHIELD_POS.1 + SHIELD_Y / 2.0 + 28.0,
        ),
        (0.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + centered_cube(
                format!("thermal_fault_station_keepout_marker_{i}"),
                28.0,
                28.0,
                KEEP_OUT_Z + 3.0,
            )
            .translate(*x, *y, 1.5);
    }
    markers
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 20.0 / 2.0, 5.0, 36)
        - centered_cylinder(format!("{name}_center_relief"), 8.0 / 2.0, 6.0, 24)
}

fn fixture_socket_rects() -> [(&'static str, Rect); 9] {
    [
        ("sealed_module_dummy_nest", module_rect()),
        ("heater_fault_coupon_pockets", coupon_rect()),
        ("independent_thermistor_logger_pockets", sensor_rect()),
        ("thermal_fuse_overtemp_switch_lands", fuse_rect()),
        ("barcode_certificate_lands", trace_rect()),
        ("release_hold_reject_lanes", lanes_rect()),
        ("clean_used_segregation", segregation_rect()),
        ("service_bulkhead", bulkhead_rect()),
        ("evidence_camera_bridge", camera_bridge_rect()),
    ]
}

fn non_overlapping_fixture_rects() -> [(&'static str, Rect); 7] {
    [
        ("sealed_module_dummy_nest", module_rect()),
        ("heater_fault_coupon_pockets", coupon_rect()),
        ("independent_thermistor_logger_pockets", sensor_rect()),
        ("thermal_fuse_overtemp_switch_lands", fuse_rect()),
        ("barcode_certificate_lands", trace_rect()),
        ("release_hold_reject_lanes", lanes_rect()),
        ("clean_used_segregation", segregation_rect()),
    ]
}

fn station_inner_rect() -> Rect {
    Rect::new(0.0, 0.0, STATION_X - 2.0 * RIM_W, STATION_Y - 2.0 * RIM_W)
}

fn module_rect() -> Rect {
    Rect::new(
        MODULE_NEST_POS.0,
        MODULE_NEST_POS.1,
        MODULE_NEST_X,
        MODULE_NEST_Y,
    )
}

fn coupon_rect() -> Rect {
    Rect::new(
        COUPON_BLOCK_POS.0,
        COUPON_BLOCK_POS.1,
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
    )
}

fn sensor_rect() -> Rect {
    Rect::new(
        SENSOR_BLOCK_POS.0,
        SENSOR_BLOCK_POS.1,
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
    )
}

fn fuse_rect() -> Rect {
    Rect::new(
        FUSE_BLOCK_POS.0,
        FUSE_BLOCK_POS.1,
        FUSE_BLOCK_X,
        FUSE_BLOCK_Y,
    )
}

fn trace_rect() -> Rect {
    Rect::new(TRACE_POS.0, TRACE_POS.1, TRACE_PLATE_X, TRACE_PLATE_Y)
}

fn lanes_rect() -> Rect {
    Rect::new(LANE_POS.0, LANE_POS.1, LANE_PLATE_X, LANE_PLATE_Y)
}

fn segregation_rect() -> Rect {
    Rect::new(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1,
        SEGREGATION_X,
        SEGREGATION_Y,
    )
}

fn bulkhead_rect() -> Rect {
    Rect::new(BULKHEAD_POS.0, BULKHEAD_POS.1, BULKHEAD_X, BULKHEAD_Y)
}

fn camera_bridge_rect() -> Rect {
    Rect::new(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
    )
}

fn rect_inside(inner: Rect, outer: Rect, margin: f64) -> bool {
    inner.x - inner.w / 2.0 >= outer.x - outer.w / 2.0 + margin
        && inner.x + inner.w / 2.0 <= outer.x + outer.w / 2.0 - margin
        && inner.y - inner.h / 2.0 >= outer.y - outer.h / 2.0 + margin
        && inner.y + inner.h / 2.0 <= outer.y + outer.h / 2.0 - margin
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

fn shield_clearance_above_module() -> f64 {
    SHIELD_Z - MODULE_DUMMY_Z
}

fn segregation_gap_to_traceability() -> f64 {
    horizontal_gap(segregation_rect(), trace_rect())
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_PLATE_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (BULKHEAD_POS.1 + BULKHEAD_Y / 2.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_stable() {
        assert_layout_constraints();

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS[0].ends_with("_base_containment_deck.stl"));
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "unscoped path: {path}");
            assert!(path.ends_with(".stl"), "non-STL output: {path}");
        }
    }

    #[test]
    fn required_feature_groups_cover_requested_station_scope() {
        for feature in [
            "base_containment_deck",
            "sealed_module_dummy_nest",
            "heater_fault_coupon_placeholder_pockets",
            "independent_thermistor_logger_pockets",
            "thermal_fuse_overtemp_switch_lands",
            "isolation_shield_panels",
            "emergency_disconnect_service_bulkhead",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn sealed_module_dummy_nest_clears_cell_free_module_envelope() {
        assert_eq!(MODULE_COLS * MODULE_ROWS, 6);
        assert!(MODULE_DUMMY_X > MODULE_ARRAY_X + 90.0);
        assert!(MODULE_DUMMY_Y > MODULE_ARRAY_Y + 88.0);
        assert!(MODULE_DUMMY_Z > REVC_TOTAL_HEIGHT);
        assert!(MODULE_NEST_X > MODULE_DUMMY_X + 60.0);
        assert!(MODULE_NEST_Y > MODULE_DUMMY_Y + 50.0);
        assert_eq!(NEST_DATUM_PIN_COUNT, 4);
    }

    #[test]
    fn placeholder_pockets_and_independent_sensors_are_explicit() {
        assert_eq!(HEATER_COUPON_COUNT, 6);
        assert_eq!(THERMISTOR_POCKET_COUNT, 8);
        assert_eq!(LOGGER_POCKET_COUNT, 2);
        assert!(COUPON_POCKET_DEPTH < COUPON_BLOCK_Z / 2.0);
        assert!(THERMISTOR_WELL_D < 12.0);
        assert!(LOGGER_POCKET_X < SENSOR_BLOCK_X / 2.0);
    }

    #[test]
    fn safety_interface_lands_are_placeholders_not_controls() {
        assert_eq!(THERMAL_FUSE_LANDS, 4);
        assert_eq!(OVERTEMP_SWITCH_LANDS, 3);
        assert_eq!(SERVICE_PORT_COUNT, 10);
        assert!(DISCONNECT_LAND_D > SERVICE_PORT_D * 2.0);
        assert!(FUSE_LAND_X > FUSE_LAND_Y);
    }

    #[test]
    fn layout_fits_contained_deck_without_unintended_plan_overlap() {
        assert_layout_constraints();
        assert!(rect_inside(module_rect(), station_inner_rect(), 8.0));
        assert!(rect_inside(coupon_rect(), station_inner_rect(), 8.0));
        assert!(rect_inside(lanes_rect(), station_inner_rect(), 8.0));
        assert!(!rects_overlap(module_rect(), coupon_rect()));
        assert!(!rects_overlap(sensor_rect(), fuse_rect()));
        assert!(!rects_overlap(trace_rect(), lanes_rect()));
        assert!(!rects_overlap(segregation_rect(), trace_rect()));
    }

    #[test]
    fn traceability_disposition_and_segregation_capacity_are_present() {
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(CERTIFICATE_LANDS, 4);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(LANE_SLOT_COUNT, 9);
        assert!(segregation_gap_to_traceability() >= SEGREGATION_GAP_MIN);
    }

    #[test]
    fn evidence_shield_and_keepout_geometry_is_sized_for_interface_review() {
        assert_eq!(CAMERA_PAD_COUNT, 4);
        assert_eq!(SHIELD_POST_COUNT, 6);
        assert_eq!(SHIELD_WINDOW_COUNT, 10);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 7);
        assert!(shield_clearance_above_module() > 80.0);
        assert!(VERTICAL_SHIELD_LIFT_CLEARANCE > SHIELD_Z);
        assert!(FRONT_ROBOT_SWEEP > 300.0);
        assert!(REAR_SERVICE_SWEEP > 200.0);
        assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE_MIN);
        assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE_MIN);
    }
}
