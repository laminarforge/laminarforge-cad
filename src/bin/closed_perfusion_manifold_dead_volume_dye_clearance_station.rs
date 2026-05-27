use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system perfusion manifold dead-volume and dye-clearance validation station.
//
// Intent:
// - Map manifold hold-up volume, dye slug breakthrough, and flush clearance before
//   a tissue-chip cassette or live culture article is connected.
// - Keep every perfusion lane the same modeled length so lane-to-lane timing
//   differences come from the manifold/coupon setup rather than the fixture.
// - Provide dye slug reservoirs, transparent witness windows, bubble traps,
//   timed fraction pockets, waste capture, sensor coupon pockets, custody lands,
//   and robot/service keepout gauges on one closed validation deck.
//
// This is architecture/fit CAD for validation planning. It is not a wetted-path
// release drawing, analytical method, sterility claim, or acceptance criterion.

const OUTPUTS: &[&str] = &[
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_base_leak_tray.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_equal_length_perfusion_lanes.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_dye_slug_reservoir_bank.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_transparent_witness_windows.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_bubble_trap_array.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_pressure_flow_sensor_coupon_pockets.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_timed_fraction_collection_pockets.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_waste_capture_nests.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_barcode_custody_lands.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_flush_reference_token_bank.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_robot_service_keepout_gauges.stl",
    "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "closed_leak_tray",
    "equal_length_perfusion_lanes",
    "dye_slug_reservoirs",
    "transparent_witness_windows",
    "bubble_traps",
    "pressure_flow_sensor_coupon_pockets",
    "timed_fraction_collection_pockets",
    "waste_capture_nests",
    "barcode_custody_lands",
    "flush_reference_tokens",
    "robot_keepout_gauges",
    "service_keepout_gauges",
];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const LEAK_BASIN_X: f64 = STATION_X - 110.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 104.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_SLOT_COUNT: usize = 8;
const LEAK_SENSOR_WELLS: usize = 5;

const LANES: usize = 8;
const LANE_RUN_X: f64 = 640.0;
const LANE_PITCH_Y: f64 = 42.0;
const LANE_TRACE_W: f64 = 8.0;
const LANE_TRACE_Z: f64 = 6.0;
const LANE_CENTER_X: f64 = -110.0;
const LANE_CENTER_Y: f64 = 34.0;
const LANE_BANK_X: f64 = LANE_RUN_X + 96.0;
const LANE_BANK_Y: f64 = (LANES as f64 - 1.0) * LANE_PITCH_Y + 66.0;
const LANE_RAIL_Z: f64 = 18.0;
const LANE_DEAD_VOLUME_UL: f64 = 18.0;
const LANE_SWEPT_LENGTH_MM: f64 = 620.0;
const EQUAL_LENGTH_TOLERANCE_MM: f64 = 0.05;
const LANE_INLET_X: f64 = LANE_CENTER_X - LANE_RUN_X / 2.0;
const LANE_OUTLET_X: f64 = LANE_CENTER_X + LANE_RUN_X / 2.0;

const DYE_BANK_X: f64 = 215.0;
const DYE_BANK_Y: f64 = LANE_BANK_Y + 48.0;
const DYE_BANK_Z: f64 = 48.0;
const DYE_BANK_POS: (f64, f64) = (-484.0, 34.0);
const DYE_RESERVOIRS: usize = LANES;
const DYE_RESERVOIR_D: f64 = 22.0;
const DYE_SEPTUM_D: f64 = 9.0;
const DYE_SLUG_VOLUME_UL: f64 = 120.0;

const WINDOW_BRIDGE_X: f64 = LANE_RUN_X + 74.0;
const WINDOW_BRIDGE_Y: f64 = LANE_BANK_Y + 24.0;
const WINDOW_BRIDGE_Z: f64 = 22.0;
const WINDOW_POS: (f64, f64) = (LANE_CENTER_X, LANE_CENTER_Y);
const WITNESS_WINDOWS: usize = LANES;
const WINDOW_X: f64 = 118.0;
const WINDOW_Y: f64 = 24.0;
const WITNESS_TICKS_PER_LANE: usize = 5;

const BUBBLE_TRAP_X: f64 = 208.0;
const BUBBLE_TRAP_Y: f64 = LANE_BANK_Y + 44.0;
const BUBBLE_TRAP_Z: f64 = 76.0;
const BUBBLE_TRAP_POS: (f64, f64) = (60.0, 34.0);
const BUBBLE_TRAPS: usize = LANES;
const BUBBLE_CHAMBER_D: f64 = 28.0;
const BUBBLE_PURGE_D: f64 = 5.0;

const SENSOR_RACK_X: f64 = 704.0;
const SENSOR_RACK_Y: f64 = 118.0;
const SENSOR_RACK_Z: f64 = 34.0;
const SENSOR_POS: (f64, f64) = (120.0, -270.0);
const SENSOR_COUPON_POCKETS: usize = LANES;
const SENSOR_POCKET_X: f64 = 58.0;
const SENSOR_POCKET_Y: f64 = 68.0;
const SENSOR_PITCH_X: f64 = 82.0;
const PRESSURE_PORTS_PER_LANE: usize = 2;
const FLOW_WINDOWS_PER_LANE: usize = 1;
const SENSOR_COUPON_CLEARANCE: f64 = 0.8;

const FRACTION_RACK_X: f64 = 388.0;
const FRACTION_RACK_Y: f64 = 344.0;
const FRACTION_RACK_Z: f64 = 36.0;
const FRACTION_POS: (f64, f64) = (390.0, 178.0);
const FRACTION_TIMEPOINTS: usize = 6;
const FRACTION_POCKETS: usize = LANES * FRACTION_TIMEPOINTS;
const FRACTION_PITCH_X: f64 = 54.0;
const FRACTION_PITCH_Y: f64 = 39.0;
const FRACTION_VIAL_D: f64 = 16.4;
const FRACTION_CLEARANCE_D: f64 = 18.0;
const FRACTION_RIM_D: f64 = 24.0;

const WASTE_NEST_X: f64 = 282.0;
const WASTE_NEST_Y: f64 = 178.0;
const WASTE_NEST_Z: f64 = 50.0;
const WASTE_POS: (f64, f64) = (450.0, -100.0);
const WASTE_CAPTURE_NESTS: usize = 4;
const WASTE_BOTTLE_D: f64 = 44.0;
const WASTE_BOTTLE_CLEARANCE_D: f64 = 47.0;
const WASTE_CAPTURE_VOLUME_ML: f64 = 720.0;

const TRACE_PANEL_X: f64 = 330.0;
const TRACE_PANEL_Y: f64 = 118.0;
const TRACE_PANEL_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-424.0, -300.0);
const BARCODE_LANDS: usize = LANES;
const CUSTODY_LANDS: usize = 4;
const FIDUCIALS: usize = 6;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 18.0;

const TOKEN_BANK_X: f64 = 360.0;
const TOKEN_BANK_Y: f64 = 82.0;
const TOKEN_BANK_Z: f64 = 20.0;
const TOKEN_POS: (f64, f64) = (58.0, 316.0);
const FLUSH_REFERENCE_TOKENS: usize = 8;
const BLANK_REFERENCE_TOKENS: usize = 4;
const TOKEN_PITCH_X: f64 = 38.0;
const TOKEN_D: f64 = 24.0;

const ROBOT_KEEP_OUT_X: f64 = 1040.0;
const ROBOT_KEEP_OUT_Y: f64 = 610.0;
const ROBOT_KEEP_OUT_Z: f64 = 168.0;
const ROBOT_KEEPOUT_WINDOWS: usize = 4;
const SERVICE_GAUGES: usize = 5;
const FRONT_SERVICE_CLEARANCE: f64 = 340.0;
const REAR_TUBING_SERVICE_CLEARANCE: f64 = 230.0;
const RIGHT_FRACTION_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_DYE_SERVICE_CLEARANCE: f64 = 170.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let lanes = equal_length_perfusion_lanes();
    export(&lanes, OUTPUTS[1]);

    let dye = dye_slug_reservoir_bank();
    export(&dye, OUTPUTS[2]);

    let windows = transparent_witness_windows();
    export(&windows, OUTPUTS[3]);

    let bubbles = bubble_trap_array();
    export(&bubbles, OUTPUTS[4]);

    let sensors = pressure_flow_sensor_coupon_pockets();
    export(&sensors, OUTPUTS[5]);

    let fractions = timed_fraction_collection_pockets();
    export(&fractions, OUTPUTS[6]);

    let waste = waste_capture_nests();
    export(&waste, OUTPUTS[7]);

    let trace = barcode_custody_lands();
    export(&trace, OUTPUTS[8]);

    let tokens = flush_reference_token_bank();
    export(&tokens, OUTPUTS[9]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + lanes
        + dye
        + windows
        + bubbles
        + sensors
        + fractions
        + waste
        + trace
        + tokens
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed perfusion manifold dead-volume/dye-clearance station: {:.0}mm x {:.0}mm closed leak-tray deck with {} equal-length lanes ({:.1}mm swept length, {:.2}mm tolerance), {} dye slug reservoirs ({:.0} uL slug target), {} transparent witness windows, {} bubble traps, {} pressure/flow sensor coupon pockets, and {} flow witness windows.",
        STATION_X,
        STATION_Y,
        LANES,
        LANE_SWEPT_LENGTH_MM,
        EQUAL_LENGTH_TOLERANCE_MM,
        DYE_RESERVOIRS,
        DYE_SLUG_VOLUME_UL,
        WITNESS_WINDOWS,
        BUBBLE_TRAPS,
        SENSOR_COUPON_POCKETS,
        LANES * FLOW_WINDOWS_PER_LANE
    );
    println!(
        "Workflow features: {} timed fraction pockets, {} waste capture nests, {} barcode/custody lands, {} fiducials, {} flush/blank reference tokens, {} robot keepout windows, {} service gauges, and {} required feature groups.",
        FRACTION_POCKETS,
        WASTE_CAPTURE_NESTS,
        BARCODE_LANDS + CUSTODY_LANDS,
        FIDUCIALS,
        FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS,
        ROBOT_KEEPOUT_WINDOWS,
        SERVICE_GAUGES,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(LANES, DYE_RESERVOIRS);
    assert_eq!(LANES, BUBBLE_TRAPS);
    assert_eq!(LANES, WITNESS_WINDOWS);
    assert_eq!(LANES, SENSOR_COUPON_POCKETS);
    assert_eq!(FRACTION_POCKETS, LANES * FRACTION_TIMEPOINTS);
    assert!(
        FRACTION_CLEARANCE_D > FRACTION_VIAL_D,
        "fraction pockets need vial insertion clearance"
    );
    assert!(
        WASTE_BOTTLE_CLEARANCE_D > WASTE_BOTTLE_D,
        "waste nests need bottle insertion clearance"
    );
    assert!(
        LANE_DEAD_VOLUME_UL <= 20.0,
        "lane coupon envelope should stay low dead volume"
    );
    assert!(
        WASTE_CAPTURE_VOLUME_ML >= LANES as f64 * FRACTION_TIMEPOINTS as f64 * 12.0,
        "waste capture volume should cover repeated lane flushes"
    );

    for (name, center, width, depth) in module_specs() {
        assert!(
            fits_on_station(center, width, depth, 24.0),
            "{name} exceeds station footprint"
        );
    }

    let lanes = rect(LANE_CENTER(), LANE_BANK_X, LANE_BANK_Y);
    let dye = rect(DYE_BANK_POS, DYE_BANK_X, DYE_BANK_Y);
    let traps = rect(BUBBLE_TRAP_POS, BUBBLE_TRAP_X, BUBBLE_TRAP_Y);
    let fractions = rect(FRACTION_POS, FRACTION_RACK_X, FRACTION_RACK_Y);
    let waste = rect(WASTE_POS, WASTE_NEST_X, WASTE_NEST_Y);
    let sensors = rect(SENSOR_POS, SENSOR_RACK_X, SENSOR_RACK_Y);
    let trace = rect(TRACE_POS, TRACE_PANEL_X, TRACE_PANEL_Y);
    let tokens = rect(TOKEN_POS, TOKEN_BANK_X, TOKEN_BANK_Y);

    assert!(!rects_overlap(dye, fractions, 18.0));
    assert!(!rects_overlap(traps, fractions, 18.0));
    assert!(!rects_overlap(sensors, trace, 18.0));
    assert!(!rects_overlap(sensors, waste, 18.0));
    assert!(!rects_overlap(trace, tokens, 18.0));
    assert!(
        rect_gap_y(lanes, sensors) >= 26.0,
        "sensor rack needs dry-service separation from lane bank"
    );
}

#[allow(non_snake_case)]
fn LANE_CENTER() -> (f64, f64) {
    (LANE_CENTER_X, LANE_CENTER_Y)
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "equal_length_perfusion_lanes",
            LANE_CENTER(),
            LANE_BANK_X,
            LANE_BANK_Y,
        ),
        (
            "dye_slug_reservoir_bank",
            DYE_BANK_POS,
            DYE_BANK_X,
            DYE_BANK_Y,
        ),
        (
            "transparent_witness_windows",
            WINDOW_POS,
            WINDOW_BRIDGE_X,
            WINDOW_BRIDGE_Y,
        ),
        (
            "bubble_trap_array",
            BUBBLE_TRAP_POS,
            BUBBLE_TRAP_X,
            BUBBLE_TRAP_Y,
        ),
        (
            "pressure_flow_sensor_coupon_pockets",
            SENSOR_POS,
            SENSOR_RACK_X,
            SENSOR_RACK_Y,
        ),
        (
            "timed_fraction_collection_pockets",
            FRACTION_POS,
            FRACTION_RACK_X,
            FRACTION_RACK_Y,
        ),
        ("waste_capture_nests", WASTE_POS, WASTE_NEST_X, WASTE_NEST_Y),
        (
            "barcode_custody_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "flush_reference_token_bank",
            TOKEN_POS,
            TOKEN_BANK_X,
            TOKEN_BANK_Y,
        ),
    ]
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_perfusion_manifold_dye_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_perfusion_manifold_dye_station_recessed_leak_basin",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        "closed_perfusion_manifold_dye_station_low_point_drain",
        DRAIN_D / 2.0,
        BASE_Z + 6.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 + 10.0, 0.0);

    deck - basin - drain - module_socket_reliefs()
        + perimeter_rims()
        + deck_mount_slots()
        + leak_sensor_wells()
        + zone_divider_rails()
        + robot_datum_fiducials()
}

fn module_socket_reliefs() -> Part {
    let mut sockets = Part::empty("closed_perfusion_manifold_dye_station_module_socket_reliefs");
    for (name, center, width, depth) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_{name}_socket_relief"),
                width + 8.0,
                depth + 8.0,
                6.6,
            )
            .translate(center.0, center.1, BASE_Z / 2.0 - 2.6);
    }
    sockets
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_perfusion_manifold_dye_station_front_low_spill_lip",
        STATION_X - 134.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z / 2.0 + 12.0);
    let rear = centered_cube(
        "closed_perfusion_manifold_dye_station_rear_tubing_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_perfusion_manifold_dye_station_left_dye_service_rim",
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
        "closed_perfusion_manifold_dye_station_right_fraction_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("closed_perfusion_manifold_dye_station_deck_mount_slots");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(
            format!("closed_perfusion_manifold_dye_station_mount_boss_{i}"),
            58.0,
            28.0,
            5.0,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 2.5);
        let hole = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_m6_clearance_{i}"),
            3.4,
            BASE_Z + 8.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_perfusion_manifold_dye_station_mount_slot_relief_{i}"),
            25.0,
            7.4,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + boss - hole - slot;
    }
    slots
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_perfusion_manifold_dye_station_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 178.0);
        let boss = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_leak_sensor_boss_{i}"),
            15.0,
            6.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 + 3.0);
        let cup = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_leak_sensor_cup_{i}"),
            7.2,
            7.4,
            28,
        )
        .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 + 3.4);
        wells = wells + (boss - cup);
    }
    wells
}

fn zone_divider_rails() -> Part {
    let wet_to_dry = centered_cube(
        "closed_perfusion_manifold_dye_station_wet_to_trace_divider_rail",
        STATION_X - 156.0,
        8.0,
        30.0,
    )
    .translate(0.0, -190.0, BASE_Z / 2.0 + 15.0);
    let fraction_guard = centered_cube(
        "closed_perfusion_manifold_dye_station_fraction_rack_splash_guard",
        10.0,
        FRACTION_RACK_Y + 38.0,
        34.0,
    )
    .translate(
        FRACTION_POS.0 - FRACTION_RACK_X / 2.0 - 22.0,
        FRACTION_POS.1,
        BASE_Z / 2.0 + 17.0,
    );
    let waste_guard = centered_cube(
        "closed_perfusion_manifold_dye_station_waste_capture_backstop",
        WASTE_NEST_X + 34.0,
        10.0,
        38.0,
    )
    .translate(
        WASTE_POS.0,
        WASTE_POS.1 + WASTE_NEST_Y / 2.0 + 20.0,
        BASE_Z / 2.0 + 19.0,
    );
    wet_to_dry + fraction_guard + waste_guard
}

fn robot_datum_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_perfusion_manifold_dye_station_robot_datum_fiducials");
    for (i, (x, y)) in [
        (-540.0, 328.0),
        (-180.0, 328.0),
        (180.0, 328.0),
        (540.0, 328.0),
        (-540.0, -328.0),
        (540.0, -328.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!(
                "closed_perfusion_manifold_dye_station_robot_datum_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn equal_length_perfusion_lanes() -> Part {
    let carrier = centered_cube(
        "closed_perfusion_manifold_dye_station_lane_carrier_plate",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_RAIL_Z,
    )
    .translate(LANE_CENTER_X, LANE_CENTER_Y, insert_z(LANE_RAIL_Z));
    let center_window = centered_cube(
        "closed_perfusion_manifold_dye_station_lane_carrier_open_cleanout",
        LANE_RUN_X - 34.0,
        LANE_BANK_Y - 28.0,
        LANE_RAIL_Z + 2.0,
    )
    .translate(LANE_CENTER_X, LANE_CENTER_Y, insert_z(LANE_RAIL_Z));

    carrier - center_window
        + lane_traces()
        + lane_inlet_outlet_ports()
        + equal_length_loop_gauges()
        + lane_hold_down_clips()
        + manifold_equalization_headers()
}

fn lane_traces() -> Part {
    let mut traces = Part::empty("closed_perfusion_manifold_dye_station_equal_length_lane_traces");
    for lane in 0..LANES {
        let y = lane_y(lane);
        let trace = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_straight_trace"),
            LANE_RUN_X,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(
            LANE_CENTER_X,
            y,
            BASE_Z + LANE_RAIL_Z + LANE_TRACE_Z / 2.0 + 1.0,
        );
        let inlet_meter = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_inlet_metered_leg"),
            54.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .rotate(0.0, 0.0, 90.0)
        .translate(
            LANE_INLET_X + 18.0,
            y,
            BASE_Z + LANE_RAIL_Z + LANE_TRACE_Z / 2.0 + 1.0,
        );
        let outlet_meter = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_outlet_metered_leg"),
            54.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .rotate(0.0, 0.0, 90.0)
        .translate(
            LANE_OUTLET_X - 18.0,
            y,
            BASE_Z + LANE_RAIL_Z + LANE_TRACE_Z / 2.0 + 1.0,
        );
        traces = traces + trace + inlet_meter + outlet_meter + volume_tick_marks(lane);
    }
    traces
}

fn lane_inlet_outlet_ports() -> Part {
    let mut ports = Part::empty("closed_perfusion_manifold_dye_station_lane_inlet_outlet_ports");
    for lane in 0..LANES {
        let y = lane_y(lane);
        let inlet = low_dead_volume_port(
            &format!("closed_perfusion_manifold_dye_station_lane_{lane}_inlet_port"),
            DYE_SEPTUM_D,
        )
        .translate(LANE_INLET_X - 26.0, y, BASE_Z + LANE_RAIL_Z + 8.0);
        let outlet = low_dead_volume_port(
            &format!("closed_perfusion_manifold_dye_station_lane_{lane}_outlet_port"),
            DYE_SEPTUM_D,
        )
        .translate(LANE_OUTLET_X + 26.0, y, BASE_Z + LANE_RAIL_Z + 8.0);
        ports = ports + inlet + outlet;
    }
    ports
}

fn equal_length_loop_gauges() -> Part {
    let mut gauges = Part::empty("closed_perfusion_manifold_dye_station_equal_length_loop_gauges");
    for lane in 0..LANES {
        let y = lane_y(lane);
        let left = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_left_length_stop"),
            7.0,
            26.0,
            18.0,
        )
        .translate(LANE_INLET_X, y, BASE_Z + LANE_RAIL_Z / 2.0 + 9.0);
        let right = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_right_length_stop"),
            7.0,
            26.0,
            18.0,
        )
        .translate(LANE_OUTLET_X, y, BASE_Z + LANE_RAIL_Z / 2.0 + 9.0);
        let caliper_bar = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_caliper_length_bar"),
            LANE_SWEPT_LENGTH_MM,
            3.0,
            5.0,
        )
        .translate(LANE_CENTER_X, y + 13.0, BASE_Z + LANE_RAIL_Z + 5.0);
        gauges = gauges + left + right + caliper_bar;
    }
    gauges
}

fn lane_hold_down_clips() -> Part {
    let mut clips = Part::empty("closed_perfusion_manifold_dye_station_lane_hold_down_clips");
    for lane in 0..LANES {
        let y = lane_y(lane);
        for (j, x) in [
            LANE_INLET_X + 92.0,
            LANE_CENTER_X - 95.0,
            LANE_CENTER_X + 95.0,
            LANE_OUTLET_X - 92.0,
        ]
        .iter()
        .enumerate()
        {
            let clip = centered_cube(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_hold_down_clip_{j}"),
                18.0,
                16.0,
                14.0,
            )
            .translate(*x, y, BASE_Z + LANE_RAIL_Z + 7.0);
            let tube_relief = centered_cylinder(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_clip_{j}_tube_relief"),
                3.4,
                20.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, y, BASE_Z + LANE_RAIL_Z + 7.0);
            clips = clips + (clip - tube_relief);
        }
    }
    clips
}

fn manifold_equalization_headers() -> Part {
    let inlet_header = centered_cube(
        "closed_perfusion_manifold_dye_station_inlet_equalization_header",
        24.0,
        LANE_BANK_Y - 44.0,
        18.0,
    )
    .translate(
        LANE_INLET_X - 58.0,
        LANE_CENTER_Y,
        BASE_Z + LANE_RAIL_Z + 9.0,
    );
    let outlet_header = centered_cube(
        "closed_perfusion_manifold_dye_station_outlet_equalization_header",
        24.0,
        LANE_BANK_Y - 44.0,
        18.0,
    )
    .translate(
        LANE_OUTLET_X + 58.0,
        LANE_CENTER_Y,
        BASE_Z + LANE_RAIL_Z + 9.0,
    );
    let balance_crossbar = centered_cube(
        "closed_perfusion_manifold_dye_station_equal_length_balance_crossbar",
        LANE_RUN_X + 116.0,
        10.0,
        12.0,
    )
    .translate(
        LANE_CENTER_X,
        LANE_CENTER_Y + LANE_BANK_Y / 2.0 - 22.0,
        BASE_Z + LANE_RAIL_Z + 6.0,
    );
    inlet_header + outlet_header + balance_crossbar
}

fn volume_tick_marks(lane: usize) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_perfusion_manifold_dye_station_lane_{lane}_volume_ticks"
    ));
    for tick in 0..WITNESS_TICKS_PER_LANE {
        let x = LANE_INLET_X + 120.0 + tick as f64 * 102.0;
        ticks = ticks
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_volume_tick_{tick}"),
                3.0,
                16.0,
                3.0,
            )
            .translate(
                x,
                lane_y(lane) - 13.0,
                BASE_Z + LANE_RAIL_Z + LANE_TRACE_Z + 4.0,
            );
    }
    ticks
}

fn dye_slug_reservoir_bank() -> Part {
    let body = centered_cube(
        "closed_perfusion_manifold_dye_station_dye_slug_reservoir_bank_body",
        DYE_BANK_X,
        DYE_BANK_Y,
        DYE_BANK_Z,
    )
    .translate(DYE_BANK_POS.0, DYE_BANK_POS.1, insert_z(DYE_BANK_Z));

    body - dye_reservoir_pockets()
        + dye_reservoir_rims()
        + dye_slug_volume_steps()
        + dye_bank_latches()
}

fn dye_reservoir_pockets() -> Part {
    let mut pockets = Part::empty("closed_perfusion_manifold_dye_station_dye_reservoir_pockets");
    for lane in 0..DYE_RESERVOIRS {
        let y = lane_y(lane);
        let pocket = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_dye_reservoir_cup"),
            DYE_RESERVOIR_D / 2.0,
            DYE_BANK_Z + 4.0,
            40,
        )
        .translate(DYE_BANK_POS.0 - 24.0, y, insert_z(DYE_BANK_Z));
        let outlet = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_slug_outlet_bore"),
            2.0,
            72.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(DYE_BANK_POS.0 + 42.0, y, BASE_Z + DYE_BANK_Z * 0.42);
        pockets = pockets + pocket + outlet;
    }
    pockets
}

fn dye_reservoir_rims() -> Part {
    let mut rims = Part::empty("closed_perfusion_manifold_dye_station_dye_reservoir_rims");
    for lane in 0..DYE_RESERVOIRS {
        let y = lane_y(lane);
        let rim = ring_z(
            &format!("closed_perfusion_manifold_dye_station_lane_{lane}_dye_reservoir_rim"),
            DYE_RESERVOIR_D + 10.0,
            DYE_RESERVOIR_D + 1.0,
            7.0,
        )
        .translate(DYE_BANK_POS.0 - 24.0, y, BASE_Z + DYE_BANK_Z + 3.5);
        let septum = low_dead_volume_port(
            &format!("closed_perfusion_manifold_dye_station_lane_{lane}_septum_cap"),
            DYE_SEPTUM_D,
        )
        .translate(DYE_BANK_POS.0 - 24.0, y, BASE_Z + DYE_BANK_Z + 12.0);
        rims = rims + rim + septum;
    }
    rims
}

fn dye_slug_volume_steps() -> Part {
    let mut steps = Part::empty("closed_perfusion_manifold_dye_station_dye_slug_volume_steps");
    for lane in 0..DYE_RESERVOIRS {
        let y = lane_y(lane);
        for step in 0..3 {
            steps = steps
                + centered_cube(
                    format!(
                        "closed_perfusion_manifold_dye_station_lane_{lane}_slug_volume_step_{step}"
                    ),
                    26.0 + step as f64 * 14.0,
                    2.2,
                    3.0,
                )
                .translate(
                    DYE_BANK_POS.0 + 58.0,
                    y - 13.0 + step as f64 * 8.0,
                    BASE_Z + DYE_BANK_Z + 1.5,
                );
        }
    }
    steps
}

fn dye_bank_latches() -> Part {
    let top = centered_cube(
        "closed_perfusion_manifold_dye_station_dye_bank_top_latch_bar",
        DYE_BANK_X + 22.0,
        12.0,
        22.0,
    )
    .translate(
        DYE_BANK_POS.0,
        DYE_BANK_POS.1 + DYE_BANK_Y / 2.0 + 8.0,
        BASE_Z + DYE_BANK_Z / 2.0 + 11.0,
    );
    let bottom = centered_cube(
        "closed_perfusion_manifold_dye_station_dye_bank_bottom_latch_bar",
        DYE_BANK_X + 22.0,
        12.0,
        22.0,
    )
    .translate(
        DYE_BANK_POS.0,
        DYE_BANK_POS.1 - DYE_BANK_Y / 2.0 - 8.0,
        BASE_Z + DYE_BANK_Z / 2.0 + 11.0,
    );
    top + bottom
}

fn transparent_witness_windows() -> Part {
    let bridge = centered_cube(
        "closed_perfusion_manifold_dye_station_transparent_window_bridge_frame",
        WINDOW_BRIDGE_X,
        WINDOW_BRIDGE_Y,
        WINDOW_BRIDGE_Z,
    )
    .translate(WINDOW_POS.0, WINDOW_POS.1, BASE_Z + LANE_RAIL_Z + 28.0);
    let cutouts = witness_window_cutouts();
    let ribs = witness_window_cross_ribs();

    bridge - cutouts + ribs + witness_window_ticks() + witness_camera_lands()
}

fn witness_window_cutouts() -> Part {
    let mut cutouts = Part::empty("closed_perfusion_manifold_dye_station_witness_window_cutouts");
    for lane in 0..WITNESS_WINDOWS {
        let y = lane_y(lane);
        cutouts = cutouts
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_witness_window"),
                WINDOW_X,
                WINDOW_Y,
                WINDOW_BRIDGE_Z + 4.0,
            )
            .translate(LANE_CENTER_X - 26.0, y, BASE_Z + LANE_RAIL_Z + 28.0);
    }
    cutouts
}

fn witness_window_cross_ribs() -> Part {
    let mut ribs = Part::empty("closed_perfusion_manifold_dye_station_witness_window_cross_ribs");
    for lane in 0..WITNESS_WINDOWS {
        let y = lane_y(lane);
        let left = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_witness_left_frame"),
            8.0,
            WINDOW_Y + 18.0,
            WINDOW_BRIDGE_Z + 6.0,
        )
        .translate(
            LANE_CENTER_X - 26.0 - WINDOW_X / 2.0 - 8.0,
            y,
            BASE_Z + LANE_RAIL_Z + 31.0,
        );
        let right = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_witness_right_frame"),
            8.0,
            WINDOW_Y + 18.0,
            WINDOW_BRIDGE_Z + 6.0,
        )
        .translate(
            LANE_CENTER_X - 26.0 + WINDOW_X / 2.0 + 8.0,
            y,
            BASE_Z + LANE_RAIL_Z + 31.0,
        );
        ribs = ribs + left + right;
    }
    ribs
}

fn witness_window_ticks() -> Part {
    let mut ticks =
        Part::empty("closed_perfusion_manifold_dye_station_witness_window_timing_ticks");
    for lane in 0..WITNESS_WINDOWS {
        for tick in 0..WITNESS_TICKS_PER_LANE {
            let x = LANE_CENTER_X - 26.0 - WINDOW_X / 2.0 + 18.0 + tick as f64 * 21.0;
            ticks = ticks
                + centered_cube(
                    format!(
                        "closed_perfusion_manifold_dye_station_lane_{lane}_witness_tick_{tick}"
                    ),
                    2.0,
                    WINDOW_Y + 10.0,
                    2.6,
                )
                .translate(x, lane_y(lane), BASE_Z + LANE_RAIL_Z + 44.0);
        }
    }
    ticks
}

fn witness_camera_lands() -> Part {
    let left = centered_cube(
        "closed_perfusion_manifold_dye_station_witness_camera_left_land",
        68.0,
        38.0,
        8.0,
    )
    .translate(
        WINDOW_POS.0 - WINDOW_BRIDGE_X / 2.0 + 48.0,
        WINDOW_POS.1 + WINDOW_BRIDGE_Y / 2.0 - 26.0,
        BASE_Z + LANE_RAIL_Z + 44.0,
    );
    let right = centered_cube(
        "closed_perfusion_manifold_dye_station_witness_camera_right_land",
        68.0,
        38.0,
        8.0,
    )
    .translate(
        WINDOW_POS.0 + WINDOW_BRIDGE_X / 2.0 - 48.0,
        WINDOW_POS.1 + WINDOW_BRIDGE_Y / 2.0 - 26.0,
        BASE_Z + LANE_RAIL_Z + 44.0,
    );
    left + right
}

fn bubble_trap_array() -> Part {
    let base = centered_cube(
        "closed_perfusion_manifold_dye_station_bubble_trap_array_base",
        BUBBLE_TRAP_X,
        BUBBLE_TRAP_Y,
        BUBBLE_TRAP_Z,
    )
    .translate(
        BUBBLE_TRAP_POS.0,
        BUBBLE_TRAP_POS.1,
        insert_z(BUBBLE_TRAP_Z),
    );

    base - bubble_trap_chambers()
        + bubble_trap_transparent_caps()
        + bubble_trap_purge_ports()
        + bubble_trap_index_flags()
}

fn bubble_trap_chambers() -> Part {
    let mut chambers = Part::empty("closed_perfusion_manifold_dye_station_bubble_trap_chambers");
    for lane in 0..BUBBLE_TRAPS {
        let y = lane_y(lane);
        let chamber = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_chamber"),
            BUBBLE_CHAMBER_D / 2.0,
            BUBBLE_TRAP_Z + 5.0,
            44,
        )
        .translate(BUBBLE_TRAP_POS.0 - 22.0, y, insert_z(BUBBLE_TRAP_Z));
        let tangent_inlet = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_tangent_inlet"),
            2.8,
            62.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            BUBBLE_TRAP_POS.0 - 64.0,
            y + 6.0,
            BASE_Z + BUBBLE_TRAP_Z * 0.42,
        );
        let tangent_outlet = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_tangent_outlet"),
            2.8,
            62.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            BUBBLE_TRAP_POS.0 + 36.0,
            y - 6.0,
            BASE_Z + BUBBLE_TRAP_Z * 0.42,
        );
        chambers = chambers + chamber + tangent_inlet + tangent_outlet;
    }
    chambers
}

fn bubble_trap_transparent_caps() -> Part {
    let mut caps = Part::empty("closed_perfusion_manifold_dye_station_bubble_trap_clear_caps");
    for lane in 0..BUBBLE_TRAPS {
        let y = lane_y(lane);
        caps = caps
            + ring_z(
                &format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_clear_cap"),
                BUBBLE_CHAMBER_D + 12.0,
                BUBBLE_CHAMBER_D - 2.0,
                8.0,
            )
            .translate(BUBBLE_TRAP_POS.0 - 22.0, y, BASE_Z + BUBBLE_TRAP_Z + 4.0);
    }
    caps
}

fn bubble_trap_purge_ports() -> Part {
    let mut ports = Part::empty("closed_perfusion_manifold_dye_station_bubble_trap_purge_ports");
    for lane in 0..BUBBLE_TRAPS {
        let y = lane_y(lane);
        let purge = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_purge_port"),
            BUBBLE_PURGE_D / 2.0,
            18.0,
            22,
        )
        .translate(BUBBLE_TRAP_POS.0 - 22.0, y, BASE_Z + BUBBLE_TRAP_Z + 12.0);
        let handle = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_purge_handle_land"),
            24.0,
            7.0,
            5.0,
        )
        .translate(BUBBLE_TRAP_POS.0 - 22.0, y, BASE_Z + BUBBLE_TRAP_Z + 21.5);
        ports = ports + purge + handle;
    }
    ports
}

fn bubble_trap_index_flags() -> Part {
    let mut flags = Part::empty("closed_perfusion_manifold_dye_station_bubble_trap_index_flags");
    for lane in 0..BUBBLE_TRAPS {
        flags = flags
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_bubble_index_flag"),
                12.0 + lane as f64 * 2.0,
                5.0,
                4.0,
            )
            .translate(
                BUBBLE_TRAP_POS.0 + 72.0,
                lane_y(lane),
                BASE_Z + BUBBLE_TRAP_Z + 2.0,
            );
    }
    flags
}

fn pressure_flow_sensor_coupon_pockets() -> Part {
    let rack = centered_cube(
        "closed_perfusion_manifold_dye_station_sensor_coupon_rack_body",
        SENSOR_RACK_X,
        SENSOR_RACK_Y,
        SENSOR_RACK_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, insert_z(SENSOR_RACK_Z));

    rack - sensor_coupon_pocket_cuts() - sensor_tube_bores()
        + pressure_sensor_lands()
        + flow_sensor_windows()
        + sensor_coupon_latch_ears()
}

fn sensor_coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_perfusion_manifold_dye_station_sensor_coupon_pocket_cuts");
    for lane in 0..SENSOR_COUPON_POCKETS {
        let x = sensor_x(lane);
        let pocket = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_sensor_coupon_pocket"),
            SENSOR_POCKET_X + SENSOR_COUPON_CLEARANCE,
            SENSOR_POCKET_Y + SENSOR_COUPON_CLEARANCE,
            SENSOR_RACK_Z + 4.0,
        )
        .translate(x, SENSOR_POS.1, insert_z(SENSOR_RACK_Z));
        cuts = cuts + pocket;
    }
    cuts
}

fn sensor_tube_bores() -> Part {
    let mut bores = Part::empty("closed_perfusion_manifold_dye_station_sensor_tube_bores");
    for lane in 0..SENSOR_COUPON_POCKETS {
        let x = sensor_x(lane);
        let bore = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_sensor_inline_bore"),
            3.1,
            SENSOR_POCKET_X + 26.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, SENSOR_POS.1, BASE_Z + SENSOR_RACK_Z * 0.46);
        bores = bores + bore;
    }
    bores
}

fn pressure_sensor_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_manifold_dye_station_pressure_sensor_lands");
    for lane in 0..SENSOR_COUPON_POCKETS {
        let x = sensor_x(lane);
        for port in 0..PRESSURE_PORTS_PER_LANE {
            lands = lands
                + low_dead_volume_port(
                    &format!(
                        "closed_perfusion_manifold_dye_station_lane_{lane}_pressure_coupon_port_{port}"
                    ),
                    7.0,
                )
                .translate(
                    x - 14.0 + port as f64 * 28.0,
                    SENSOR_POS.1 + 22.0,
                    BASE_Z + SENSOR_RACK_Z + 5.0,
                );
        }
    }
    lands
}

fn flow_sensor_windows() -> Part {
    let mut windows = Part::empty("closed_perfusion_manifold_dye_station_flow_sensor_windows");
    for lane in 0..SENSOR_COUPON_POCKETS {
        let x = sensor_x(lane);
        let frame = rectangular_frame_xy(
            &format!("closed_perfusion_manifold_dye_station_lane_{lane}_flow_window_frame"),
            38.0,
            22.0,
            5.0,
            5.0,
        )
        .translate(x, SENSOR_POS.1 - 24.0, BASE_Z + SENSOR_RACK_Z + 2.5);
        windows = windows + frame;
    }
    windows
}

fn sensor_coupon_latch_ears() -> Part {
    let mut ears = Part::empty("closed_perfusion_manifold_dye_station_sensor_coupon_latch_ears");
    for lane in 0..SENSOR_COUPON_POCKETS {
        let x = sensor_x(lane);
        let left = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_sensor_left_latch"),
            12.0,
            18.0,
            14.0,
        )
        .translate(x - SENSOR_POCKET_X / 2.0 - 9.0, SENSOR_POS.1, BASE_Z + 7.0);
        let right = centered_cube(
            format!("closed_perfusion_manifold_dye_station_lane_{lane}_sensor_right_latch"),
            12.0,
            18.0,
            14.0,
        )
        .translate(x + SENSOR_POCKET_X / 2.0 + 9.0, SENSOR_POS.1, BASE_Z + 7.0);
        ears = ears + left + right;
    }
    ears
}

fn timed_fraction_collection_pockets() -> Part {
    let rack = centered_cube(
        "closed_perfusion_manifold_dye_station_timed_fraction_rack_body",
        FRACTION_RACK_X,
        FRACTION_RACK_Y,
        FRACTION_RACK_Z,
    )
    .translate(FRACTION_POS.0, FRACTION_POS.1, insert_z(FRACTION_RACK_Z));

    rack - fraction_pocket_cuts()
        + fraction_pocket_rims()
        + fraction_time_index_rails()
        + fraction_lane_bars()
}

fn fraction_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_perfusion_manifold_dye_station_fraction_pocket_cuts");
    for lane in 0..LANES {
        for timepoint in 0..FRACTION_TIMEPOINTS {
            let (x, y) = fraction_position(lane, timepoint);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_perfusion_manifold_dye_station_lane_{lane}_time_{timepoint}_fraction_pocket"
                    ),
                    FRACTION_CLEARANCE_D / 2.0,
                    FRACTION_RACK_Z + 4.0,
                    32,
                )
                .translate(x, y, insert_z(FRACTION_RACK_Z));
        }
    }
    cuts
}

fn fraction_pocket_rims() -> Part {
    let mut rims = Part::empty("closed_perfusion_manifold_dye_station_fraction_pocket_rims");
    for lane in 0..LANES {
        for timepoint in 0..FRACTION_TIMEPOINTS {
            let (x, y) = fraction_position(lane, timepoint);
            rims = rims
                + ring_z(
                    &format!(
                        "closed_perfusion_manifold_dye_station_lane_{lane}_time_{timepoint}_fraction_rim"
                    ),
                    FRACTION_RIM_D,
                    FRACTION_CLEARANCE_D + 1.0,
                    5.0,
                )
                .translate(x, y, BASE_Z + FRACTION_RACK_Z + 2.5);
        }
    }
    rims
}

fn fraction_time_index_rails() -> Part {
    let mut rails = Part::empty("closed_perfusion_manifold_dye_station_fraction_time_index_rails");
    for timepoint in 0..FRACTION_TIMEPOINTS {
        let x = FRACTION_POS.0 + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_X);
        rails = rails
            + centered_cube(
                format!(
                    "closed_perfusion_manifold_dye_station_fraction_timepoint_{timepoint}_index_rail"
                ),
                3.0,
                FRACTION_RACK_Y - 30.0,
                5.0,
            )
            .translate(x, FRACTION_POS.1, BASE_Z + FRACTION_RACK_Z + 2.5);
    }
    rails
}

fn fraction_lane_bars() -> Part {
    let mut bars = Part::empty("closed_perfusion_manifold_dye_station_fraction_lane_bars");
    for lane in 0..LANES {
        let y = FRACTION_POS.1 + centered_index(lane, LANES, FRACTION_PITCH_Y);
        bars = bars
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_fraction_lane_{lane}_bar"),
                FRACTION_RACK_X - 34.0,
                2.0,
                4.0,
            )
            .translate(FRACTION_POS.0, y, BASE_Z + FRACTION_RACK_Z + 4.0);
    }
    bars
}

fn waste_capture_nests() -> Part {
    let nest = centered_cube(
        "closed_perfusion_manifold_dye_station_waste_capture_nest_body",
        WASTE_NEST_X,
        WASTE_NEST_Y,
        WASTE_NEST_Z,
    )
    .translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_NEST_Z));

    nest - waste_bottle_pockets()
        + waste_bottle_rims()
        + waste_route_troughs()
        + waste_level_sight_gauges()
}

fn waste_bottle_pockets() -> Part {
    let mut pockets = Part::empty("closed_perfusion_manifold_dye_station_waste_bottle_pockets");
    for i in 0..WASTE_CAPTURE_NESTS {
        let x = WASTE_POS.0 + centered_index(i, WASTE_CAPTURE_NESTS, 60.0);
        pockets = pockets
            + centered_cylinder(
                format!("closed_perfusion_manifold_dye_station_waste_bottle_pocket_{i}"),
                WASTE_BOTTLE_CLEARANCE_D / 2.0,
                WASTE_NEST_Z + 5.0,
                40,
            )
            .translate(x, WASTE_POS.1, insert_z(WASTE_NEST_Z));
    }
    pockets
}

fn waste_bottle_rims() -> Part {
    let mut rims = Part::empty("closed_perfusion_manifold_dye_station_waste_bottle_rims");
    for i in 0..WASTE_CAPTURE_NESTS {
        let x = WASTE_POS.0 + centered_index(i, WASTE_CAPTURE_NESTS, 60.0);
        rims = rims
            + ring_z(
                &format!("closed_perfusion_manifold_dye_station_waste_bottle_rim_{i}"),
                WASTE_BOTTLE_CLEARANCE_D + 12.0,
                WASTE_BOTTLE_CLEARANCE_D + 1.0,
                7.0,
            )
            .translate(x, WASTE_POS.1, BASE_Z + WASTE_NEST_Z + 3.5);
    }
    rims
}

fn waste_route_troughs() -> Part {
    let inlet_trough = centered_cube(
        "closed_perfusion_manifold_dye_station_waste_route_inlet_trough",
        WASTE_NEST_X - 42.0,
        28.0,
        18.0,
    )
    .translate(WASTE_POS.0, WASTE_POS.1 + 58.0, BASE_Z + WASTE_NEST_Z + 9.0);
    let overflow_trough = centered_cube(
        "closed_perfusion_manifold_dye_station_waste_overflow_trough",
        WASTE_NEST_X - 72.0,
        18.0,
        14.0,
    )
    .translate(WASTE_POS.0, WASTE_POS.1 - 62.0, BASE_Z + WASTE_NEST_Z + 7.0);
    inlet_trough + overflow_trough
}

fn waste_level_sight_gauges() -> Part {
    let mut gauges = Part::empty("closed_perfusion_manifold_dye_station_waste_level_sight_gauges");
    for i in 0..WASTE_CAPTURE_NESTS {
        let x = WASTE_POS.0 + centered_index(i, WASTE_CAPTURE_NESTS, 60.0);
        let window = rectangular_frame_xy(
            &format!("closed_perfusion_manifold_dye_station_waste_level_window_{i}"),
            24.0,
            42.0,
            4.0,
            5.0,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, WASTE_POS.1 - WASTE_NEST_Y / 2.0 - 4.0, BASE_Z + 34.0);
        gauges = gauges + window;
    }
    gauges
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_perfusion_manifold_dye_station_barcode_custody_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z));

    panel + barcode_lands() + custody_lands() + trace_fiducials() + dry_erase_status_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_manifold_dye_station_lane_barcode_lands");
    for lane in 0..BARCODE_LANDS {
        let row = lane / 4;
        let col = lane % 4;
        let x = TRACE_POS.0 + centered_index(col, 4, 82.0);
        let y = TRACE_POS.1 + 18.0 - row as f64 * 34.0;
        lands = lands
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_lane_{lane}_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, BASE_Z + TRACE_PANEL_Z + 1.5);
    }
    lands
}

fn custody_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_manifold_dye_station_custody_lands");
    for i in 0..CUSTODY_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_custody_land_{i}"),
                74.0,
                18.0,
                3.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(i, CUSTODY_LANDS, 82.0),
                TRACE_POS.1 + 49.0,
                BASE_Z + TRACE_PANEL_Z + 1.5,
            );
    }
    lands
}

fn trace_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_perfusion_manifold_dye_station_trace_fiducials");
    for (i, (x, y)) in [
        (-152.0, -44.0),
        (-92.0, -44.0),
        (-32.0, -44.0),
        (32.0, -44.0),
        (92.0, -44.0),
        (152.0, -44.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!(
                "closed_perfusion_manifold_dye_station_trace_fiducial_{i}"
            ))
            .translate(
                TRACE_POS.0 + *x,
                TRACE_POS.1 + *y,
                BASE_Z + TRACE_PANEL_Z + 2.0,
            );
    }
    fiducials
}

fn dry_erase_status_slots() -> Part {
    let released = centered_cube(
        "closed_perfusion_manifold_dye_station_released_status_land",
        92.0,
        20.0,
        3.0,
    )
    .translate(
        TRACE_POS.0 - 106.0,
        TRACE_POS.1 - 53.0,
        BASE_Z + TRACE_PANEL_Z + 1.5,
    );
    let hold = centered_cube(
        "closed_perfusion_manifold_dye_station_hold_status_land",
        92.0,
        20.0,
        3.0,
    )
    .translate(
        TRACE_POS.0,
        TRACE_POS.1 - 53.0,
        BASE_Z + TRACE_PANEL_Z + 1.5,
    );
    let reject = centered_cube(
        "closed_perfusion_manifold_dye_station_reject_status_land",
        92.0,
        20.0,
        3.0,
    )
    .translate(
        TRACE_POS.0 + 106.0,
        TRACE_POS.1 - 53.0,
        BASE_Z + TRACE_PANEL_Z + 1.5,
    );
    released + hold + reject
}

fn flush_reference_token_bank() -> Part {
    let bank = centered_cube(
        "closed_perfusion_manifold_dye_station_flush_reference_token_bank",
        TOKEN_BANK_X,
        TOKEN_BANK_Y,
        TOKEN_BANK_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_BANK_Z));
    bank - token_pocket_cuts() + flush_reference_tokens() + blank_reference_tokens()
}

fn token_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_perfusion_manifold_dye_station_reference_token_pocket_cuts");
    for i in 0..(FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS) {
        let x = TOKEN_POS.0
            + centered_index(
                i,
                FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS,
                TOKEN_PITCH_X,
            );
        let pocket = centered_cylinder(
            format!("closed_perfusion_manifold_dye_station_reference_token_pocket_{i}"),
            TOKEN_D / 2.0,
            TOKEN_BANK_Z + 3.0,
            30,
        )
        .translate(x, TOKEN_POS.1, insert_z(TOKEN_BANK_Z));
        cuts = cuts + pocket;
    }
    cuts
}

fn flush_reference_tokens() -> Part {
    let mut tokens = Part::empty("closed_perfusion_manifold_dye_station_flush_reference_tokens");
    for i in 0..FLUSH_REFERENCE_TOKENS {
        let x = TOKEN_POS.0
            + centered_index(
                i,
                FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS,
                TOKEN_PITCH_X,
            );
        tokens = tokens
            + centered_cylinder(
                format!("closed_perfusion_manifold_dye_station_flush_reference_token_{i}"),
                TOKEN_D / 2.0 - 1.0,
                4.0,
                30,
            )
            .translate(x, TOKEN_POS.1, BASE_Z + TOKEN_BANK_Z + 2.0)
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_flush_reference_token_{i}_index"),
                3.0 + i as f64,
                16.0,
                2.0,
            )
            .translate(x, TOKEN_POS.1, BASE_Z + TOKEN_BANK_Z + 5.0);
    }
    tokens
}

fn blank_reference_tokens() -> Part {
    let mut tokens = Part::empty("closed_perfusion_manifold_dye_station_blank_reference_tokens");
    for i in 0..BLANK_REFERENCE_TOKENS {
        let index = FLUSH_REFERENCE_TOKENS + i;
        let x = TOKEN_POS.0
            + centered_index(
                index,
                FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS,
                TOKEN_PITCH_X,
            );
        tokens = tokens
            + centered_cylinder(
                format!("closed_perfusion_manifold_dye_station_blank_reference_token_{i}"),
                TOKEN_D / 2.0 - 1.0,
                4.0,
                30,
            )
            .translate(x, TOKEN_POS.1, BASE_Z + TOKEN_BANK_Z + 2.0)
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_blank_reference_token_{i}_flat"),
                18.0,
                3.0,
                2.0,
            )
            .translate(x, TOKEN_POS.1, BASE_Z + TOKEN_BANK_Z + 5.0);
    }
    tokens
}

fn robot_service_keepout_gauges() -> Part {
    let robot = rectangular_frame_xy(
        "closed_perfusion_manifold_dye_station_robot_plan_keepout_frame",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        10.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z + ROBOT_KEEP_OUT_Z);
    let robot_posts = keepout_corner_posts();
    let service = service_clearance_gauges();
    let height_gauge = centered_cube(
        "closed_perfusion_manifold_dye_station_robot_z_clearance_gauge",
        36.0,
        36.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + 68.0,
        STATION_Y / 2.0 - 72.0,
        BASE_Z + ROBOT_KEEP_OUT_Z / 2.0,
    );
    robot + robot_posts + service + height_gauge
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty("closed_perfusion_manifold_dye_station_keepout_corner_posts");
    for (i, (x, y)) in [
        (-ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (-ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_perfusion_manifold_dye_station_robot_keepout_post_{i}"),
                12.0,
                12.0,
                ROBOT_KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    }
    posts
}

fn service_clearance_gauges() -> Part {
    let front = gauge_bar(
        "closed_perfusion_manifold_dye_station_front_service_clearance_gauge",
        STATION_X - 180.0,
        8.0,
        48.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 24.0,
    );
    let rear = gauge_bar(
        "closed_perfusion_manifold_dye_station_rear_tubing_service_clearance_gauge",
        STATION_X - 240.0,
        8.0,
        48.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_TUBING_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 24.0,
    );
    let right = gauge_bar(
        "closed_perfusion_manifold_dye_station_right_fraction_service_clearance_gauge",
        8.0,
        STATION_Y - 180.0,
        48.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_FRACTION_SERVICE_CLEARANCE / 2.0,
        0.0,
        BASE_Z + 24.0,
    );
    let left = gauge_bar(
        "closed_perfusion_manifold_dye_station_left_dye_service_clearance_gauge",
        8.0,
        STATION_Y - 180.0,
        48.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_DYE_SERVICE_CLEARANCE / 2.0,
        0.0,
        BASE_Z + 24.0,
    );
    let waste_pull = gauge_bar(
        "closed_perfusion_manifold_dye_station_waste_bottle_pull_clearance_gauge",
        WASTE_NEST_X + 40.0,
        8.0,
        42.0,
    )
    .translate(
        WASTE_POS.0,
        WASTE_POS.1 - WASTE_NEST_Y / 2.0 - 62.0,
        BASE_Z + 21.0,
    );
    front + rear + right + left + waste_pull
}

fn low_dead_volume_port(name: &str, bore_d: f64) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), bore_d / 2.0 + 5.0, 12.0, 30);
    let bore = centered_cylinder(format!("{name}_bore"), bore_d / 2.0, 14.0, 24);
    boss - bore
}

fn ring_z(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 40)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 36)
}

fn rectangular_frame_xy(name: &str, outer_x: f64, outer_y: f64, rail: f64, z: f64) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner"),
            outer_x - 2.0 * rail,
            outer_y - 2.0 * rail,
            z + 1.0,
        )
}

fn fiducial_target(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_ring"), 10.0, 3.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot"), 3.2, 4.0, 24);
    let xhair = centered_cube(format!("{name}_crosshair_x"), 22.0, 2.0, 2.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 22.0, 2.0);
    disc - center + xhair
}

fn gauge_bar(name: &str, x: f64, y: f64, z: f64) -> Part {
    let bar = centered_cube(format!("{name}_bar"), x, y, z);
    let tick_a = centered_cube(format!("{name}_tick_a"), 14.0, 14.0, z + 8.0).translate(
        -x / 2.0,
        -y / 2.0,
        0.0,
    );
    let tick_b = centered_cube(format!("{name}_tick_b"), 14.0, 14.0, z + 8.0).translate(
        x / 2.0,
        y / 2.0,
        0.0,
    );
    bar + tick_a + tick_b
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 52.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 52.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 52.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (0.0, STATION_Y / 2.0 - 48.0),
        (-(STATION_X / 2.0 - 52.0), 0.0),
        (STATION_X / 2.0 - 52.0, 0.0),
    ]
}

fn lane_y(lane: usize) -> f64 {
    LANE_CENTER_Y + centered_index(lane, LANES, LANE_PITCH_Y)
}

fn sensor_x(lane: usize) -> f64 {
    SENSOR_POS.0 + centered_index(lane, SENSOR_COUPON_POCKETS, SENSOR_PITCH_X)
}

fn fraction_position(lane: usize, timepoint: usize) -> (f64, f64) {
    (
        FRACTION_POS.0 + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_X),
        FRACTION_POS.1 + centered_index(lane, LANES, FRACTION_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64), margin: f64) -> bool {
    !(a.1 + margin <= b.0 || b.1 + margin <= a.0 || a.3 + margin <= b.2 || b.3 + margin <= a.2)
}

fn rect_gap_y(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> f64 {
    if a.3 < b.2 {
        b.2 - a.3
    } else if b.3 < a.2 {
        a.2 - b.3
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_perfusion_manifold_dead_volume_dye_clearance_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn perfusion_lanes_are_equal_length_and_low_dead_volume() {
        assert_eq!(LANES, 8);
        assert_eq!(DYE_RESERVOIRS, LANES);
        assert_eq!(BUBBLE_TRAPS, LANES);
        assert_eq!(WITNESS_WINDOWS, LANES);
        assert_eq!(SENSOR_COUPON_POCKETS, LANES);
        assert!(LANE_SWEPT_LENGTH_MM <= LANE_RUN_X);
        assert!(LANE_DEAD_VOLUME_UL <= 20.0);
        assert!(EQUAL_LENGTH_TOLERANCE_MM <= 0.05);

        let first = lane_y(0);
        let last = lane_y(LANES - 1);
        assert_eq!(first + last, 2.0 * LANE_CENTER_Y);
        for lane in 1..LANES {
            let spacing = lane_y(lane) - lane_y(lane - 1);
            assert_eq!(spacing, LANE_PITCH_Y);
        }
    }

    #[test]
    fn timed_fraction_collection_maps_every_lane_and_timepoint() {
        let mut pockets = BTreeSet::new();
        for lane in 0..LANES {
            for timepoint in 0..FRACTION_TIMEPOINTS {
                pockets.insert((lane, timepoint));
                let (x, y) = fraction_position(lane, timepoint);
                assert!(x.abs() < STATION_X / 2.0);
                assert!(y.abs() < STATION_Y / 2.0);
            }
        }
        assert_eq!(pockets.len(), FRACTION_POCKETS);
        assert_eq!(FRACTION_POCKETS, 48);
        assert!(FRACTION_CLEARANCE_D > FRACTION_VIAL_D);
        assert!(FRACTION_RIM_D > FRACTION_CLEARANCE_D);
    }

    #[test]
    fn sensor_and_witness_counts_are_per_lane() {
        assert_eq!(PRESSURE_PORTS_PER_LANE * LANES, 16);
        assert_eq!(FLOW_WINDOWS_PER_LANE * LANES, 8);
        assert_eq!(WITNESS_TICKS_PER_LANE * LANES, 40);
        assert!(SENSOR_POCKET_X > 2.0 * DYE_SEPTUM_D);
        assert!(SENSOR_COUPON_CLEARANCE > 0.5);
        assert!(WINDOW_X > 4.0 * LANE_TRACE_W);
        assert!(WINDOW_Y > LANE_TRACE_W);
    }

    #[test]
    fn waste_traceability_and_reference_controls_are_explicit() {
        assert_eq!(WASTE_CAPTURE_NESTS, 4);
        assert!(WASTE_CAPTURE_VOLUME_ML >= 576.0);
        assert!(WASTE_BOTTLE_CLEARANCE_D > WASTE_BOTTLE_D);
        assert_eq!(BARCODE_LANDS, LANES);
        assert_eq!(CUSTODY_LANDS, 4);
        assert_eq!(FIDUCIALS, 6);
        assert_eq!(FLUSH_REFERENCE_TOKENS + BLANK_REFERENCE_TOKENS, 12);
        assert!(DYE_SLUG_VOLUME_UL >= 100.0);
    }

    #[test]
    fn module_rectangles_fit_station_without_core_overlaps() {
        for (name, center, width, depth) in module_specs() {
            assert!(
                fits_on_station(center, width, depth, 24.0),
                "{name} should fit with margin"
            );
        }

        let dye = rect(DYE_BANK_POS, DYE_BANK_X, DYE_BANK_Y);
        let fractions = rect(FRACTION_POS, FRACTION_RACK_X, FRACTION_RACK_Y);
        let traps = rect(BUBBLE_TRAP_POS, BUBBLE_TRAP_X, BUBBLE_TRAP_Y);
        let sensors = rect(SENSOR_POS, SENSOR_RACK_X, SENSOR_RACK_Y);
        let waste = rect(WASTE_POS, WASTE_NEST_X, WASTE_NEST_Y);
        let trace = rect(TRACE_POS, TRACE_PANEL_X, TRACE_PANEL_Y);
        let tokens = rect(TOKEN_POS, TOKEN_BANK_X, TOKEN_BANK_Y);

        assert!(!rects_overlap(dye, fractions, 18.0));
        assert!(!rects_overlap(traps, fractions, 18.0));
        assert!(!rects_overlap(sensors, waste, 18.0));
        assert!(!rects_overlap(sensors, trace, 18.0));
        assert!(!rects_overlap(trace, tokens, 18.0));
    }

    #[test]
    fn keepout_gauges_cover_robot_and_service_access() {
        assert_eq!(ROBOT_KEEPOUT_WINDOWS, 4);
        assert_eq!(SERVICE_GAUGES, 5);
        assert!(ROBOT_KEEP_OUT_X < STATION_X);
        assert!(ROBOT_KEEP_OUT_Y < STATION_Y);
        assert!(ROBOT_KEEP_OUT_Z > BUBBLE_TRAP_Z);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_TUBING_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_FRACTION_SERVICE_CLEARANCE >= 200.0);
        assert!(LEFT_DYE_SERVICE_CLEARANCE >= 160.0);
    }
}
