use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// No-cell perfusion shear/pressure surrogate station for tissue-chip runs.
//
// Intent:
// - Validate shear-lane pressure drop, flow sensor mapping, bubble challenge
//   response, tracer visibility, and waste capture before live cells enter the
//   workcell.
// - Provide dummy restriction chips and transparent observation-lane fixtures
//   that make shear/pressure behavior visible without consuming seeded chips.
// - Keep release, hold, and reject paths physically distinct with per-position
//   barcode/fiducial lands and robot/service keepout gauges.
//
// This is a mechanical surrogate station. Biological acceptance criteria,
// sensor calibration, and wetted-material validation remain separate gates.

const OUTPUTS: &[&str] = &[
    "output/closed_perfusion_shear_stress_surrogate_chip_station_base_leak_tray.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_dummy_restriction_chip_carrier.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_transparent_shear_observation_lanes.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_pressure_tap_manifold.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_flow_sensor_docks.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_bubble_challenge_inlet.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_dye_tracer_ports.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_waste_capture.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_barcode_fiducial_lands.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_robot_service_keepouts.stl",
    "output/closed_perfusion_shear_stress_surrogate_chip_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "dummy_restriction_chips",
    "transparent_shear_observation_lanes",
    "pressure_taps",
    "flow_sensor_docks",
    "bubble_challenge_inlet",
    "dye_tracer_ports",
    "waste_capture",
    "per_position_barcode_fiducials",
    "release_hold_reject_lanes",
    "robot_service_keepouts",
];

const COLS: usize = 3;
const ROWS: usize = 2;
const POSITIONS: usize = COLS * ROWS;
const PRESSURE_TAPS_PER_POSITION: usize = 3;
const FLOW_SENSOR_DOCKS_PER_POSITION: usize = 2;
const DYE_TRACER_PORTS_PER_POSITION: usize = 2;
const FIDUCIALS_PER_POSITION: usize = 2;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = POSITIONS;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;

const BASE_X: f64 = 820.0;
const BASE_Y: f64 = 560.0;
const BASE_Z: f64 = 18.0;
const LEAK_BASIN_X: f64 = BASE_X - 82.0;
const LEAK_BASIN_Y: f64 = BASE_Y - 86.0;

const CHIP_X: f64 = REVC_CHIP_LENGTH + 18.0;
const CHIP_Y: f64 = REVC_CHIP_WIDTH + 18.0;
const CHIP_PITCH_X: f64 = CHIP_X + 28.0;
const CHIP_PITCH_Y: f64 = CHIP_Y + 34.0;
const ARRAY_X: f64 = (COLS as f64 - 1.0) * CHIP_PITCH_X + CHIP_X;
const ARRAY_Y: f64 = (ROWS as f64 - 1.0) * CHIP_PITCH_Y + CHIP_Y;

const CARRIER_X: f64 = ARRAY_X + 86.0;
const CARRIER_Y: f64 = ARRAY_Y + 84.0;
const CARRIER_Z: f64 = 24.0;
const COUPON_X: f64 = REVC_CHIP_LENGTH + 8.0;
const COUPON_Y: f64 = REVC_CHIP_WIDTH + 8.0;
const COUPON_Z: f64 = 10.0;

const OBS_LANE_X: f64 = REVC_CHIP_LENGTH + 24.0;
const OBS_LANE_Y: f64 = 31.0;
const OBS_LANE_Z: f64 = 18.0;
const SHEAR_WINDOW_X: f64 = REVC_CHIP_LENGTH - 24.0;
const SHEAR_WINDOW_Y: f64 = 14.0;
const SHEAR_CHANNEL_Y: f64 = 3.2;

const PRESSURE_MANIFOLD_X: f64 = 650.0;
const PRESSURE_MANIFOLD_Y: f64 = 76.0;
const PRESSURE_MANIFOLD_Z: f64 = 58.0;
const PRESSURE_TAP_D: f64 = 3.2;

const FLOW_DOCK_X: f64 = 74.0;
const FLOW_DOCK_Y: f64 = 42.0;
const FLOW_DOCK_Z: f64 = 36.0;
const FLOW_DOCK_PITCH_X: f64 = 86.0;
const SENSOR_WINDOW_X: f64 = 42.0;
const SENSOR_WINDOW_Y: f64 = 26.0;
const SENSOR_WINDOW_Z: f64 = 18.0;

const BUBBLE_INLET_X: f64 = 238.0;
const BUBBLE_INLET_Y: f64 = 118.0;
const BUBBLE_INLET_Z: f64 = 68.0;
const BUBBLE_CHAMBER_D: f64 = 34.0;

const DYE_BAR_X: f64 = 612.0;
const DYE_BAR_Y: f64 = 58.0;
const DYE_BAR_Z: f64 = 38.0;
const SEPTUM_D: f64 = 9.0;

const WASTE_X: f64 = 386.0;
const WASTE_Y: f64 = 118.0;
const WASTE_Z: f64 = 44.0;
const WASTE_BAYS: usize = 3;

const TRACE_X: f64 = 598.0;
const TRACE_Y: f64 = 76.0;
const TRACE_Z: f64 = 8.0;
const BARCODE_LAND_X: f64 = 62.0;
const BARCODE_LAND_Y: f64 = 16.0;

const STATUS_X: f64 = 612.0;
const STATUS_Y: f64 = 118.0;
const STATUS_Z: f64 = 28.0;

const ROBOT_KEEP_OUT_X: f64 = 704.0;
const ROBOT_KEEP_OUT_Y: f64 = 420.0;
const ROBOT_KEEP_OUT_Z: f64 = 142.0;
const ROBOT_Z_CLEARANCE: f64 = 118.0;
const REAR_SERVICE_ACCESS: f64 = 96.0;
const FRONT_ROBOT_APPROACH: f64 = 120.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let carrier = dummy_restriction_chip_carrier();
    export(OUTPUTS[1], &carrier);

    let observation = transparent_shear_observation_lanes();
    export(OUTPUTS[2], &observation);

    let pressure = pressure_tap_manifold();
    export(OUTPUTS[3], &pressure);

    let flow = flow_sensor_docks();
    export(OUTPUTS[4], &flow);

    let bubble = bubble_challenge_inlet();
    export(OUTPUTS[5], &bubble);

    let dye = dye_tracer_ports();
    export(OUTPUTS[6], &dye);

    let waste = waste_capture();
    export(OUTPUTS[7], &waste);

    let traceability = barcode_fiducial_lands();
    export(OUTPUTS[8], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + waste.translate(170.0, -198.0, BASE_Z / 2.0 + WASTE_Z / 2.0)
        + carrier.translate(38.0, 18.0, BASE_Z / 2.0 + CARRIER_Z / 2.0)
        + observation.translate(
            38.0,
            18.0,
            BASE_Z / 2.0 + CARRIER_Z + OBS_LANE_Z / 2.0 + 6.0,
        )
        + pressure.translate(34.0, 216.0, BASE_Z / 2.0 + PRESSURE_MANIFOLD_Z / 2.0)
        + flow.translate(38.0, -132.0, BASE_Z / 2.0 + FLOW_DOCK_Z / 2.0)
        + bubble.translate(-266.0, -206.0, BASE_Z / 2.0 + BUBBLE_INLET_Z / 2.0)
        + dye.translate(38.0, 158.0, BASE_Z / 2.0 + DYE_BAR_Z / 2.0)
        + traceability.translate(38.0, -254.0, BASE_Z / 2.0 + TRACE_Z / 2.0)
        + status.translate(38.0, -244.0, BASE_Z / 2.0 + STATUS_Z / 2.0)
        + keepouts.translate(0.0, 0.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!(
        "Closed perfusion shear surrogate station: {:.0}mm x {:.0}mm leak-tray footprint, {} no-cell dummy restriction chips in a {}x{} grid, {} pressure taps, {} flow sensor docks, {} dye/tracer ports, {} barcode lands, {} fiducials, release/hold/reject lanes, bubble challenge inlet, waste capture, {} robot/service keepout windows, and {} required feature groups.",
        BASE_X,
        BASE_Y,
        POSITIONS,
        COLS,
        ROWS,
        POSITIONS * PRESSURE_TAPS_PER_POSITION,
        POSITIONS * FLOW_SENSOR_DOCKS_PER_POSITION,
        POSITIONS * DYE_TRACER_PORTS_PER_POSITION,
        POSITIONS,
        POSITIONS * FIDUCIALS_PER_POSITION,
        ROBOT_KEEP_OUT_WINDOWS,
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("closed_perfusion_shear_base_deck", BASE_X, BASE_Y, BASE_Z);
    let basin = centered_cube(
        "closed_perfusion_shear_base_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        8.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder(
        "closed_perfusion_shear_base_leak_tray_drain",
        9.0 / 2.0,
        46.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 66.0, -BASE_Y / 2.0 + 12.0, 0.0);
    let station_socket = centered_cube(
        "closed_perfusion_shear_carrier_socket_relief",
        CARRIER_X + 34.0,
        CARRIER_Y + 28.0,
        7.0,
    )
    .translate(38.0, 18.0, BASE_Z / 2.0 - 2.5);

    deck - basin - drain - station_socket
        + perimeter_lips()
        + base_mount_slots()
        + carrier_locator_bosses()
        + leak_witness_ribs()
}

fn perimeter_lips() -> Part {
    let left = centered_cube(
        "closed_perfusion_shear_left_leak_lip",
        18.0,
        BASE_Y - 62.0,
        30.0,
    )
    .translate(-(BASE_X / 2.0 - 31.0), 0.0, BASE_Z / 2.0 + 15.0);
    let right = centered_cube(
        "closed_perfusion_shear_right_leak_lip",
        18.0,
        BASE_Y - 62.0,
        30.0,
    )
    .translate(BASE_X / 2.0 - 31.0, 0.0, BASE_Z / 2.0 + 15.0);
    let rear = centered_cube(
        "closed_perfusion_shear_rear_service_lip",
        BASE_X - 72.0,
        18.0,
        32.0,
    )
    .translate(0.0, BASE_Y / 2.0 - 31.0, BASE_Z / 2.0 + 16.0);
    let front = centered_cube(
        "closed_perfusion_shear_front_robot_low_lip",
        BASE_X - 126.0,
        12.0,
        18.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 28.0, BASE_Z / 2.0 + 9.0);

    left + right + rear + front
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("closed_perfusion_shear_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("closed_perfusion_shear_m6_mount_hole_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_perfusion_shear_m6_mount_slot_{i}"),
                22.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn carrier_locator_bosses() -> Part {
    let mut bosses = Part::empty("closed_perfusion_shear_carrier_locator_bosses");
    for (i, (x, y)) in [
        (38.0 - CARRIER_X / 2.0 + 32.0, 18.0 - CARRIER_Y / 2.0 + 30.0),
        (38.0 + CARRIER_X / 2.0 - 32.0, 18.0 - CARRIER_Y / 2.0 + 30.0),
        (38.0 - CARRIER_X / 2.0 + 32.0, 18.0 + CARRIER_Y / 2.0 - 30.0),
        (38.0 + CARRIER_X / 2.0 - 32.0, 18.0 + CARRIER_Y / 2.0 - 30.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_perfusion_shear_carrier_locator_boss_{i}"),
            9.0,
            8.0,
            32,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        let pin_clearance = centered_cylinder(
            format!("closed_perfusion_shear_carrier_locator_pin_clearance_{i}"),
            3.2,
            10.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        bosses = bosses + (boss - pin_clearance);
    }
    bosses
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_perfusion_shear_leak_witness_ribs");
    for i in 0..8 {
        let x = centered_index(i, 8, 72.0);
        ribs = ribs
            + centered_cube(
                format!("closed_perfusion_shear_leak_witness_rib_{i}"),
                8.0,
                LEAK_BASIN_Y - 74.0,
                5.0,
            )
            .translate(x, -4.0, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn dummy_restriction_chip_carrier() -> Part {
    let tray = centered_cube(
        "closed_perfusion_shear_dummy_restriction_carrier_tray",
        CARRIER_X,
        CARRIER_Y,
        CARRIER_Z,
    );
    let cleanout = centered_cube(
        "closed_perfusion_shear_dummy_restriction_carrier_center_window",
        ARRAY_X + 22.0,
        ARRAY_Y + 22.0,
        CARRIER_Z + 4.0,
    );

    let mut pocket_cuts = Part::empty("closed_perfusion_shear_dummy_chip_pocket_cuts");
    let mut coupons = Part::empty("closed_perfusion_shear_dummy_restriction_chips");
    for position in 0..POSITIONS {
        let (x, y) = chip_position(position);
        pocket_cuts = pocket_cuts
            + centered_cube(
                format!("closed_perfusion_shear_dummy_chip_pocket_{position}"),
                COUPON_X + 5.0,
                COUPON_Y + 5.0,
                CARRIER_Z + 6.0,
            )
            .translate(x, y, 0.0);
        coupons = coupons
            + dummy_restriction_chip(position).translate(x, y, CARRIER_Z / 2.0 + COUPON_Z / 2.0);
    }

    tray - cleanout - pocket_cuts + carrier_datum_rails() + carrier_spring_clips() + coupons
}

fn dummy_restriction_chip(position: usize) -> Part {
    let body = centered_cube(
        format!("closed_perfusion_shear_dummy_restriction_chip_{position}"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let inlet = centered_cylinder(
        format!("closed_perfusion_shear_dummy_chip_{position}_inlet_bore"),
        FLUID_BORE_D / 2.0,
        COUPON_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -COUPON_Y / 4.0, 0.0);
    let outlet = centered_cylinder(
        format!("closed_perfusion_shear_dummy_chip_{position}_outlet_bore"),
        FLUID_BORE_D / 2.0,
        COUPON_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, COUPON_Y / 4.0, 0.0);
    let shear_channel = centered_cube(
        format!("closed_perfusion_shear_dummy_chip_{position}_known_shear_channel"),
        COUPON_X - 36.0,
        SHEAR_CHANNEL_Y,
        COUPON_Z + 2.0,
    );
    let observation_window = centered_cube(
        format!("closed_perfusion_shear_dummy_chip_{position}_clear_observation_window"),
        SHEAR_WINDOW_X,
        SHEAR_WINDOW_Y,
        COUPON_Z + 2.0,
    );
    let restriction_slot = centered_cube(
        format!("closed_perfusion_shear_dummy_chip_{position}_restriction_neckdown"),
        restriction_slot_length(position),
        1.4 + (position % 3) as f64 * 0.5,
        COUPON_Z + 2.0,
    )
    .translate(0.0, 0.0, 0.0);

    body - inlet - outlet - shear_channel - observation_window - restriction_slot
        + restriction_index_tabs(position)
        + chip_gripper_ears(position)
}

fn restriction_index_tabs(position: usize) -> Part {
    let mut tabs = Part::empty(format!(
        "closed_perfusion_shear_dummy_chip_{position}_restriction_index_tabs"
    ));
    for tab in 0..=position % 3 {
        tabs = tabs
            + centered_cube(
                format!("closed_perfusion_shear_dummy_chip_{position}_restriction_tab_{tab}"),
                7.0,
                3.0,
                3.0,
            )
            .translate(
                -(COUPON_X / 2.0 - 14.0 - tab as f64 * 10.0),
                COUPON_Y / 2.0 - 8.0,
                COUPON_Z / 2.0 + 1.5,
            );
    }
    tabs
}

fn chip_gripper_ears(position: usize) -> Part {
    let left = centered_cube(
        format!("closed_perfusion_shear_dummy_chip_{position}_left_gripper_ear"),
        12.0,
        26.0,
        6.0,
    )
    .translate(-(COUPON_X / 2.0 + 6.0), 0.0, -COUPON_Z / 2.0 + 3.0);
    let right = centered_cube(
        format!("closed_perfusion_shear_dummy_chip_{position}_right_gripper_ear"),
        12.0,
        26.0,
        6.0,
    )
    .translate(COUPON_X / 2.0 + 6.0, 0.0, -COUPON_Z / 2.0 + 3.0);
    left + right
}

fn carrier_datum_rails() -> Part {
    let rear = centered_cube(
        "closed_perfusion_shear_dummy_carrier_rear_hard_datum",
        ARRAY_X + 48.0,
        14.0,
        24.0,
    )
    .translate(0.0, ARRAY_Y / 2.0 + 34.0, CARRIER_Z / 2.0 + 12.0);
    let left = centered_cube(
        "closed_perfusion_shear_dummy_carrier_left_datum",
        14.0,
        ARRAY_Y + 54.0,
        24.0,
    )
    .translate(-(ARRAY_X / 2.0 + 34.0), 0.0, CARRIER_Z / 2.0 + 12.0);
    let right = centered_cube(
        "closed_perfusion_shear_dummy_carrier_right_spring_datum",
        12.0,
        ARRAY_Y + 54.0,
        18.0,
    )
    .translate(ARRAY_X / 2.0 + 34.0, 0.0, CARRIER_Z / 2.0 + 9.0);
    let front = centered_cube(
        "closed_perfusion_shear_dummy_carrier_front_low_access_lip",
        ARRAY_X + 40.0,
        10.0,
        14.0,
    )
    .translate(0.0, -(ARRAY_Y / 2.0 + 34.0), CARRIER_Z / 2.0 + 7.0);

    rear + left + right + front
}

fn carrier_spring_clips() -> Part {
    let mut clips = Part::empty("closed_perfusion_shear_dummy_carrier_spring_clips");
    for row in 0..ROWS {
        let y = centered_index(row, ROWS, CHIP_PITCH_Y);
        clips = clips
            + centered_cube(
                format!("closed_perfusion_shear_dummy_carrier_row_{row}_spring_clip"),
                16.0,
                42.0,
                18.0,
            )
            .translate(ARRAY_X / 2.0 + 50.0, y, CARRIER_Z / 2.0 + 9.0);
    }
    clips
}

fn transparent_shear_observation_lanes() -> Part {
    let mut lanes = Part::empty("closed_perfusion_shear_transparent_observation_lanes");
    for position in 0..POSITIONS {
        let (x, y) = chip_position(position);
        lanes = lanes + observation_lane(position).translate(x, y, 0.0);
    }
    lanes + observation_alignment_bridge()
}

fn observation_lane(position: usize) -> Part {
    let frame = centered_cube(
        format!("closed_perfusion_shear_observation_lane_{position}_clear_acrylic_frame"),
        OBS_LANE_X,
        OBS_LANE_Y,
        OBS_LANE_Z,
    );
    let window = centered_cube(
        format!("closed_perfusion_shear_observation_lane_{position}_transparent_window_cutout"),
        SHEAR_WINDOW_X,
        SHEAR_WINDOW_Y,
        OBS_LANE_Z + 3.0,
    );
    let flow_bore = centered_cylinder(
        format!("closed_perfusion_shear_observation_lane_{position}_flow_bore"),
        FLUID_BORE_D / 2.0,
        OBS_LANE_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0);
    let inlet_o_ring = o_ring_land(
        &format!("closed_perfusion_shear_observation_lane_{position}_inlet_o_ring"),
        18.0,
        8.0,
    )
    .translate(-(OBS_LANE_X / 2.0 - 18.0), 0.0, 0.0);
    let outlet_o_ring = o_ring_land(
        &format!("closed_perfusion_shear_observation_lane_{position}_outlet_o_ring"),
        18.0,
        8.0,
    )
    .translate(OBS_LANE_X / 2.0 - 18.0, 0.0, 0.0);

    frame - window - flow_bore + inlet_o_ring + outlet_o_ring + shear_scale_ticks(position)
}

fn shear_scale_ticks(position: usize) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_perfusion_shear_observation_lane_{position}_shear_scale_ticks"
    ));
    for tick in 0..7 {
        let height = if tick == 3 { 6.0 } else { 3.5 };
        ticks = ticks
            + centered_cube(
                format!("closed_perfusion_shear_observation_lane_{position}_scale_tick_{tick}"),
                1.2,
                height,
                2.0,
            )
            .translate(
                centered_index(tick, 7, 15.0),
                OBS_LANE_Y / 2.0 - 3.0,
                OBS_LANE_Z / 2.0 + 1.0,
            );
    }
    ticks
}

fn observation_alignment_bridge() -> Part {
    let rear = centered_cube(
        "closed_perfusion_shear_observation_rear_alignment_bridge",
        ARRAY_X + 52.0,
        10.0,
        10.0,
    )
    .translate(0.0, ARRAY_Y / 2.0 + 28.0, 1.0);
    let front = centered_cube(
        "closed_perfusion_shear_observation_front_alignment_bridge",
        ARRAY_X + 52.0,
        10.0,
        10.0,
    )
    .translate(0.0, -(ARRAY_Y / 2.0 + 28.0), 1.0);
    rear + front
}

fn pressure_tap_manifold() -> Part {
    let bar = centered_cube(
        "closed_perfusion_shear_pressure_tap_manifold_body",
        PRESSURE_MANIFOLD_X,
        PRESSURE_MANIFOLD_Y,
        PRESSURE_MANIFOLD_Z,
    );
    let supply_header = centered_cylinder(
        "closed_perfusion_shear_pressure_supply_header",
        7.0 / 2.0,
        PRESSURE_MANIFOLD_X + 10.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -18.0, 10.0);
    let return_header = centered_cylinder(
        "closed_perfusion_shear_pressure_return_header",
        7.0 / 2.0,
        PRESSURE_MANIFOLD_X + 10.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 18.0, -8.0);

    let mut cuts = Part::empty("closed_perfusion_shear_pressure_tap_cuts");
    let mut bosses = Part::empty("closed_perfusion_shear_pressure_tap_luer_bosses");
    for position in 0..POSITIONS {
        let x = pressure_position_x(position);
        for tap in 0..PRESSURE_TAPS_PER_POSITION {
            let y = tap_y(tap);
            cuts = cuts
                + centered_cylinder(
                    format!("closed_perfusion_shear_position_{position}_pressure_tap_{tap}"),
                    PRESSURE_TAP_D / 2.0,
                    PRESSURE_MANIFOLD_Y + 8.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, 8.0 - tap as f64 * 8.0);
            bosses = bosses
                + luer_boss(
                    &format!("closed_perfusion_shear_position_{position}_pressure_tap_{tap}_boss"),
                    12.0,
                    7.0,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x,
                    -PRESSURE_MANIFOLD_Y / 2.0 - 2.0,
                    8.0 - tap as f64 * 8.0,
                );
        }
    }

    let reference_port = luer_boss(
        "closed_perfusion_shear_pressure_reference_calibrator_port",
        17.0,
        10.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        PRESSURE_MANIFOLD_X / 2.0 - 32.0,
        PRESSURE_MANIFOLD_Y / 2.0 + 4.0,
        0.0,
    );

    bar - supply_header - return_header - cuts
        + bosses
        + pressure_sensor_label_lands()
        + reference_port
}

fn pressure_sensor_label_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_shear_pressure_sensor_label_lands");
    for position in 0..POSITIONS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_shear_position_{position}_pressure_label_land"),
                42.0,
                12.0,
                3.0,
            )
            .translate(
                pressure_position_x(position),
                PRESSURE_MANIFOLD_Y / 2.0 - 8.0,
                PRESSURE_MANIFOLD_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn flow_sensor_docks() -> Part {
    let mut docks = Part::empty("closed_perfusion_shear_flow_sensor_docks");
    for position in 0..POSITIONS {
        let x = pressure_position_x(position);
        docks = docks
            + flow_sensor_dock(position, "supply").translate(
                x - FLOW_DOCK_PITCH_X / 4.0,
                -18.0,
                0.0,
            )
            + flow_sensor_dock(position, "return").translate(
                x + FLOW_DOCK_PITCH_X / 4.0,
                24.0,
                0.0,
            );
    }
    docks + flow_sensor_common_cable_comb()
}

fn flow_sensor_dock(position: usize, side: &str) -> Part {
    let body = centered_cube(
        format!("closed_perfusion_shear_position_{position}_{side}_flow_sensor_dock_body"),
        FLOW_DOCK_X,
        FLOW_DOCK_Y,
        FLOW_DOCK_Z,
    );
    let sensor_pocket = centered_cube(
        format!("closed_perfusion_shear_position_{position}_{side}_flow_sensor_window"),
        SENSOR_WINDOW_X,
        SENSOR_WINDOW_Y,
        SENSOR_WINDOW_Z,
    )
    .translate(0.0, 0.0, 5.0);
    let tube_bore = centered_cylinder(
        format!("closed_perfusion_shear_position_{position}_{side}_flow_sensor_tube_bore"),
        FLUID_BORE_D / 2.0,
        FLOW_DOCK_X + 6.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -7.0);
    let latch_left = centered_cube(
        format!("closed_perfusion_shear_position_{position}_{side}_flow_sensor_left_latch"),
        8.0,
        FLOW_DOCK_Y + 8.0,
        8.0,
    )
    .translate(-(FLOW_DOCK_X / 2.0 - 8.0), 0.0, FLOW_DOCK_Z / 2.0 + 4.0);
    let latch_right = centered_cube(
        format!("closed_perfusion_shear_position_{position}_{side}_flow_sensor_right_latch"),
        8.0,
        FLOW_DOCK_Y + 8.0,
        8.0,
    )
    .translate(FLOW_DOCK_X / 2.0 - 8.0, 0.0, FLOW_DOCK_Z / 2.0 + 4.0);

    body - sensor_pocket - tube_bore + latch_left + latch_right
}

fn flow_sensor_common_cable_comb() -> Part {
    let rail = centered_cube(
        "closed_perfusion_shear_flow_sensor_common_cable_comb",
        PRESSURE_MANIFOLD_X - 44.0,
        18.0,
        20.0,
    )
    .translate(0.0, -64.0, 0.0);
    let mut teeth = Part::empty("closed_perfusion_shear_flow_sensor_cable_comb_teeth");
    for position in 0..POSITIONS {
        teeth = teeth
            + centered_cube(
                format!("closed_perfusion_shear_position_{position}_flow_sensor_cable_tooth"),
                8.0,
                24.0,
                22.0,
            )
            .translate(pressure_position_x(position), -80.0, 1.0);
    }
    rail + teeth
}

fn bubble_challenge_inlet() -> Part {
    let body = centered_cube(
        "closed_perfusion_shear_bubble_challenge_inlet_body",
        BUBBLE_INLET_X,
        BUBBLE_INLET_Y,
        BUBBLE_INLET_Z,
    );
    let inlet = centered_cylinder(
        "closed_perfusion_shear_bubble_challenge_bulk_inlet",
        FLUID_BORE_D / 2.0,
        BUBBLE_INLET_Y + 8.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-74.0, 0.0, -14.0);
    let challenge_chamber = centered_cylinder(
        "closed_perfusion_shear_bubble_challenge_visibility_chamber",
        BUBBLE_CHAMBER_D / 2.0,
        BUBBLE_INLET_X - 58.0,
        42,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 6.0);
    let outlet = centered_cylinder(
        "closed_perfusion_shear_bubble_challenge_conditioned_outlet",
        FLUID_BORE_D / 2.0,
        BUBBLE_INLET_Y + 8.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(74.0, 0.0, -14.0);
    let high_point_vent = centered_cylinder(
        "closed_perfusion_shear_bubble_challenge_high_point_vent",
        3.2 / 2.0,
        BUBBLE_INLET_Z + 4.0,
        20,
    )
    .translate(0.0, 0.0, 0.0);
    let optical_window = centered_cube(
        "closed_perfusion_shear_bubble_challenge_optical_window",
        BUBBLE_INLET_X - 92.0,
        BUBBLE_INLET_Y + 4.0,
        18.0,
    )
    .translate(0.0, 0.0, 5.0);

    body - inlet - challenge_chamber - outlet - high_point_vent - optical_window
        + bubble_metering_ports()
        + bubble_bypass_valve_land()
}

fn bubble_metering_ports() -> Part {
    let mut ports = Part::empty("closed_perfusion_shear_bubble_metering_ports");
    for (i, diameter) in [4.0, 7.0, 10.0].iter().enumerate() {
        ports = ports
            + luer_boss(
                &format!("closed_perfusion_shear_bubble_metering_port_{i}"),
                diameter + 10.0,
                7.0,
            )
            .translate(
                centered_index(i, 3, 42.0),
                -(BUBBLE_INLET_Y / 2.0 + 4.0),
                BUBBLE_INLET_Z / 2.0 - 5.0,
            );
    }
    ports
}

fn bubble_bypass_valve_land() -> Part {
    let land = centered_cube(
        "closed_perfusion_shear_bubble_challenge_bypass_valve_land",
        64.0,
        34.0,
        8.0,
    )
    .translate(
        BUBBLE_INLET_X / 2.0 - 42.0,
        BUBBLE_INLET_Y / 2.0 - 24.0,
        BUBBLE_INLET_Z / 2.0 + 4.0,
    );
    let groove = centered_cylinder(
        "closed_perfusion_shear_bubble_challenge_bypass_valve_groove",
        FLUID_BORE_D / 2.0,
        70.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        BUBBLE_INLET_X / 2.0 - 42.0,
        BUBBLE_INLET_Y / 2.0 - 24.0,
        BUBBLE_INLET_Z / 2.0 + 4.0,
    );
    land - groove
}

fn dye_tracer_ports() -> Part {
    let bar = centered_cube(
        "closed_perfusion_shear_dye_tracer_port_bar",
        DYE_BAR_X,
        DYE_BAR_Y,
        DYE_BAR_Z,
    );
    let mut cuts = Part::empty("closed_perfusion_shear_dye_tracer_port_cuts");
    let mut bosses = Part::empty("closed_perfusion_shear_dye_tracer_septum_bosses");
    for position in 0..POSITIONS {
        let x = pressure_position_x(position);
        for port in 0..DYE_TRACER_PORTS_PER_POSITION {
            let y = if port == 0 { -14.0 } else { 14.0 };
            cuts = cuts
                + centered_cylinder(
                    format!("closed_perfusion_shear_position_{position}_dye_tracer_port_{port}"),
                    3.4 / 2.0,
                    DYE_BAR_Z + 6.0,
                    20,
                )
                .translate(x, y, 0.0);
            bosses = bosses
                + luer_boss(
                    &format!("closed_perfusion_shear_position_{position}_septum_boss_{port}"),
                    SEPTUM_D + 7.0,
                    5.0,
                )
                .translate(x, y, DYE_BAR_Z / 2.0 + 2.5);
        }
    }
    let purge_slot = centered_cube(
        "closed_perfusion_shear_dye_tracer_common_purge_slot",
        DYE_BAR_X - 44.0,
        6.0,
        8.0,
    )
    .translate(0.0, 0.0, -DYE_BAR_Z / 2.0 + 6.0);

    bar - cuts - purge_slot + bosses + dye_witness_strip()
}

fn dye_witness_strip() -> Part {
    let strip = centered_cube(
        "closed_perfusion_shear_dye_witness_transparent_strip",
        DYE_BAR_X - 78.0,
        8.0,
        5.0,
    )
    .translate(0.0, DYE_BAR_Y / 2.0 + 2.0, 0.0);
    let mut windows = Part::empty("closed_perfusion_shear_dye_witness_window_marks");
    for position in 0..POSITIONS {
        windows = windows
            + centered_cube(
                format!("closed_perfusion_shear_position_{position}_dye_witness_window"),
                34.0,
                10.0,
                7.0,
            )
            .translate(pressure_position_x(position), DYE_BAR_Y / 2.0 + 2.0, 0.0);
    }
    strip - windows
}

fn waste_capture() -> Part {
    let tray = centered_cube(
        "closed_perfusion_shear_waste_capture_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let sump = centered_cube(
        "closed_perfusion_shear_waste_capture_absorbent_pad_recess",
        WASTE_X - 42.0,
        WASTE_Y - 34.0,
        WASTE_Z / 2.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 4.0);
    let drain = centered_cylinder(
        "closed_perfusion_shear_waste_capture_closed_drain_bulkhead",
        8.0 / 2.0,
        WASTE_Y + 8.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_X / 2.0 - 42.0, 0.0, -8.0);
    let mut bay_cuts = Part::empty("closed_perfusion_shear_waste_capture_bay_cuts");
    for bay in 0..WASTE_BAYS {
        bay_cuts = bay_cuts
            + centered_cube(
                format!("closed_perfusion_shear_waste_capture_bay_{bay}"),
                82.0,
                WASTE_Y - 50.0,
                WASTE_Z + 4.0,
            )
            .translate(centered_index(bay, WASTE_BAYS, 102.0), 0.0, 8.0);
    }

    tray - sump - drain - bay_cuts + waste_bay_dividers() + waste_bottle_necks()
}

fn waste_bay_dividers() -> Part {
    let mut dividers = Part::empty("closed_perfusion_shear_waste_capture_dividers");
    for i in 0..2 {
        dividers = dividers
            + centered_cube(
                format!("closed_perfusion_shear_waste_capture_weir_divider_{i}"),
                8.0,
                WASTE_Y - 24.0,
                WASTE_Z,
            )
            .translate(centered_index(i, 2, 102.0), 0.0, 0.0);
    }
    dividers
}

fn waste_bottle_necks() -> Part {
    let mut necks = Part::empty("closed_perfusion_shear_waste_bottle_neck_docks");
    for bay in 0..WASTE_BAYS {
        necks = necks
            + o_ring_land(
                &format!("closed_perfusion_shear_waste_bottle_neck_dock_{bay}"),
                34.0,
                8.0,
            )
            .translate(
                centered_index(bay, WASTE_BAYS, 102.0),
                -(WASTE_Y / 2.0 - 18.0),
                WASTE_Z / 2.0 + 4.0,
            );
    }
    necks
}

fn barcode_fiducial_lands() -> Part {
    let spine = centered_cube(
        "closed_perfusion_shear_traceability_spine",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("closed_perfusion_shear_per_position_barcode_fiducial_lands");
    for position in 0..POSITIONS {
        let x = pressure_position_x(position);
        lands = lands
            + barcode_land(position).translate(x, -15.0, TRACE_Z / 2.0 + 2.0)
            + fiducial_disc(&format!(
                "closed_perfusion_shear_position_{position}_left_fiducial"
            ))
            .translate(x - 38.0, 22.0, TRACE_Z / 2.0 + 2.0)
            + fiducial_disc(&format!(
                "closed_perfusion_shear_position_{position}_right_fiducial"
            ))
            .translate(x + 38.0, 22.0, TRACE_Z / 2.0 + 2.0);
    }
    spine + lands
}

fn barcode_land(position: usize) -> Part {
    let land = centered_cube(
        format!("closed_perfusion_shear_position_{position}_barcode_land"),
        BARCODE_LAND_X,
        BARCODE_LAND_Y,
        4.0,
    );
    let scan_slot = centered_cube(
        format!("closed_perfusion_shear_position_{position}_barcode_scan_slot"),
        BARCODE_LAND_X - 12.0,
        3.2,
        5.0,
    );
    land - scan_slot
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_perfusion_shear_release_hold_reject_lane_panel",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut cuts = Part::empty("closed_perfusion_shear_release_hold_reject_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("closed_perfusion_shear_status_lane_{lane}_slot_{slot}"),
                    42.0,
                    20.0,
                    STATUS_Z + 4.0,
                )
                .translate(status_slot_x(slot), status_lane_y(lane), 0.0);
        }
    }
    panel - cuts + status_lane_dividers() + status_lane_label_lands()
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("closed_perfusion_shear_status_lane_dividers");
    for lane in 0..=STATUS_LANES {
        dividers = dividers
            + centered_cube(
                format!("closed_perfusion_shear_status_lane_divider_{lane}"),
                STATUS_X - 34.0,
                5.0,
                STATUS_Z + 8.0,
            )
            .translate(
                0.0,
                -STATUS_Y / 2.0 + lane as f64 * (STATUS_Y / STATUS_LANES as f64),
                4.0,
            );
    }
    dividers
}

fn status_lane_label_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_shear_status_lane_label_lands");
    for (lane, name) in ["release", "hold", "reject"].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_shear_{name}_lane_label_land"),
                58.0,
                16.0,
                4.0,
            )
            .translate(
                -(STATUS_X / 2.0 - 42.0),
                status_lane_y(lane),
                STATUS_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let top = keepout_frame(
        "closed_perfusion_shear_top_robot_service_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        10.0,
    )
    .translate(0.0, 0.0, ROBOT_KEEP_OUT_Z / 2.0 - 5.0);
    let front_robot = keepout_frame(
        "closed_perfusion_shear_front_robot_approach_keepout",
        ROBOT_KEEP_OUT_X - 80.0,
        FRONT_ROBOT_APPROACH,
        8.0,
    )
    .translate(
        0.0,
        -(ROBOT_KEEP_OUT_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0),
        ROBOT_Z_CLEARANCE,
    );
    let rear_service = keepout_frame(
        "closed_perfusion_shear_rear_pressure_service_keepout",
        ROBOT_KEEP_OUT_X - 96.0,
        REAR_SERVICE_ACCESS,
        8.0,
    )
    .translate(
        0.0,
        ROBOT_KEEP_OUT_Y / 2.0 - REAR_SERVICE_ACCESS / 2.0,
        ROBOT_Z_CLEARANCE + 8.0,
    );
    let side_service = keepout_frame(
        "closed_perfusion_shear_left_waste_service_keepout",
        112.0,
        ROBOT_KEEP_OUT_Y - 120.0,
        8.0,
    )
    .translate(
        -(ROBOT_KEEP_OUT_X / 2.0 - 56.0),
        -12.0,
        ROBOT_Z_CLEARANCE - 10.0,
    );

    top + front_robot + rear_service + side_service + keepout_posts() + keepout_height_gauge()
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn keepout_posts() -> Part {
    let mut posts = Part::empty("closed_perfusion_shear_robot_service_keepout_posts");
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
                format!("closed_perfusion_shear_robot_service_keepout_post_{i}"),
                10.0,
                10.0,
                ROBOT_KEEP_OUT_Z,
            )
            .translate(*x, *y, 0.0);
    }
    posts
}

fn keepout_height_gauge() -> Part {
    let gauge = centered_cube(
        "closed_perfusion_shear_robot_z_clearance_gauge",
        26.0,
        26.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        ROBOT_KEEP_OUT_X / 2.0 - 42.0,
        -(ROBOT_KEEP_OUT_Y / 2.0 - 42.0),
        -ROBOT_KEEP_OUT_Z / 2.0 + ROBOT_Z_CLEARANCE / 2.0,
    );
    let label = centered_cube(
        "closed_perfusion_shear_robot_z_clearance_label_land",
        48.0,
        18.0,
        4.0,
    )
    .translate(
        ROBOT_KEEP_OUT_X / 2.0 - 42.0,
        -(ROBOT_KEEP_OUT_Y / 2.0 - 76.0),
        0.0,
    );
    gauge + label
}

fn o_ring_land(name: &str, outer_d: f64, thickness: f64) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), outer_d / 2.0, thickness, 32);
    let inner = centered_cylinder(
        format!("{name}_inner_clearance"),
        FLUID_BORE_D / 2.0,
        thickness + 2.0,
        24,
    );
    outer - inner
}

fn luer_boss(name: &str, outer_d: f64, thickness: f64) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), outer_d / 2.0, thickness, 32);
    let inner = centered_cylinder(
        format!("{name}_inner_luer_clearance"),
        3.4 / 2.0,
        thickness + 2.0,
        20,
    );
    outer - inner
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 8.0, 3.0, 36);
    let inner = centered_cylinder(format!("{name}_center_dot"), 3.2, 4.0, 24);
    outer - inner
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 44.0), -(BASE_Y / 2.0 - 42.0)),
        (BASE_X / 2.0 - 44.0, -(BASE_Y / 2.0 - 42.0)),
        (-(BASE_X / 2.0 - 44.0), BASE_Y / 2.0 - 42.0),
        (BASE_X / 2.0 - 44.0, BASE_Y / 2.0 - 42.0),
        (0.0, -(BASE_Y / 2.0 - 42.0)),
        (0.0, BASE_Y / 2.0 - 42.0),
        (-(BASE_X / 2.0 - 44.0), 0.0),
        (BASE_X / 2.0 - 44.0, 0.0),
    ]
}

fn chip_position(position: usize) -> (f64, f64) {
    let row = position / COLS;
    let col = position % COLS;
    (
        centered_index(col, COLS, CHIP_PITCH_X),
        centered_index(row, ROWS, CHIP_PITCH_Y),
    )
}

fn pressure_position_x(position: usize) -> f64 {
    centered_index(position, POSITIONS, 96.0)
}

fn tap_y(tap: usize) -> f64 {
    centered_index(tap, PRESSURE_TAPS_PER_POSITION, 22.0)
}

fn status_lane_y(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, STATUS_Y / STATUS_LANES as f64)
}

fn status_slot_x(slot: usize) -> f64 {
    centered_index(slot, STATUS_SLOTS_PER_LANE, 82.0)
}

fn restriction_slot_length(position: usize) -> f64 {
    28.0 + (position % 3) as f64 * 14.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
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
        for output in OUTPUTS {
            assert!(
                output.starts_with("output/closed_perfusion_shear_stress_surrogate_chip_station_")
            );
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_list_covers_surrogate_station_intent() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"dummy_restriction_chips"));
        assert!(REQUIRED_FEATURES.contains(&"transparent_shear_observation_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_taps"));
        assert!(REQUIRED_FEATURES.contains(&"flow_sensor_docks"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_challenge_inlet"));
        assert!(REQUIRED_FEATURES.contains(&"dye_tracer_ports"));
        assert!(REQUIRED_FEATURES.contains(&"waste_capture"));
        assert!(REQUIRED_FEATURES.contains(&"per_position_barcode_fiducials"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn no_cell_chip_grid_is_symmetric_and_fits_carrier() {
        assert_eq!(POSITIONS, COLS * ROWS);
        let (first_x, first_y) = chip_position(0);
        let (last_x, last_y) = chip_position(POSITIONS - 1);
        assert_eq!(first_x, -last_x);
        assert_eq!(first_y, -last_y);
        assert!(ARRAY_X + 64.0 < CARRIER_X);
        assert!(ARRAY_Y + 62.0 < CARRIER_Y);
        assert!(COUPON_X > REVC_CHIP_LENGTH);
        assert!(COUPON_Y > REVC_CHIP_WIDTH);
    }

    #[test]
    fn instrumentation_counts_match_every_surrogate_position() {
        assert_eq!(POSITIONS * PRESSURE_TAPS_PER_POSITION, 18);
        assert_eq!(POSITIONS * FLOW_SENSOR_DOCKS_PER_POSITION, 12);
        assert_eq!(POSITIONS * DYE_TRACER_PORTS_PER_POSITION, 12);
        assert_eq!(POSITIONS * FIDUCIALS_PER_POSITION, 12);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 18);
    }

    #[test]
    fn transparent_observation_geometry_clears_shear_lanes() {
        assert!(SHEAR_WINDOW_X < OBS_LANE_X - 24.0);
        assert!(SHEAR_WINDOW_Y > SHEAR_CHANNEL_Y * 3.0);
        assert!(OBS_LANE_Z > COUPON_Z);
        assert!(restriction_slot_length(0) < restriction_slot_length(1));
        assert!(restriction_slot_length(1) < restriction_slot_length(2));
    }

    #[test]
    fn pressure_flow_and_traceability_arrays_stay_inside_station_envelope() {
        assert!(pressure_position_x(0).abs() < PRESSURE_MANIFOLD_X / 2.0 - 50.0);
        assert!(pressure_position_x(POSITIONS - 1).abs() < DYE_BAR_X / 2.0 - 34.0);
        assert!(status_slot_x(0).abs() < STATUS_X / 2.0 - 46.0);
        assert!(status_slot_x(STATUS_SLOTS_PER_LANE - 1).abs() < STATUS_X / 2.0 - 46.0);
        assert!(TRACE_X <= STATUS_X);
        assert!(BARCODE_LAND_X * POSITIONS as f64 <= TRACE_X);
    }

    #[test]
    fn leak_waste_and_keepout_clearances_cover_service_workflow() {
        assert!(LEAK_BASIN_X >= CARRIER_X);
        assert!(LEAK_BASIN_Y >= CARRIER_Y);
        assert_eq!(WASTE_BAYS, 3);
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 4);
        assert!(ROBOT_Z_CLEARANCE > BUBBLE_INLET_Z);
        assert!(ROBOT_KEEP_OUT_X < BASE_X);
        assert!(ROBOT_KEEP_OUT_Y < BASE_Y);
        assert!(FRONT_ROBOT_APPROACH > REAR_SERVICE_ACCESS);
    }
}
