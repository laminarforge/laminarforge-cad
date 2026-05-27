use std::collections::BTreeSet;
use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system perfusion pump cross-channel crosstalk validation station.
//
// Intent:
// - Package purchased multi-channel pump heads, parallel closed tubing lanes,
//   pressure/flow sensor ports, lane-selective occlusion valves, and reference
//   reservoirs into a repeatable fixture for measuring cross-channel coupling.
// - Provide witness bypasses, restrictor coupons, drain capture, traceability
//   lands, and service/robot keepouts without modeling pump internals, sterile
//   barriers, sensors, controller firmware, or acceptance algorithms.

const OUTPUT_PREFIX: &str = "output/closed_perfusion_pump_cross_channel_crosstalk_station";
const OUTPUTS: [&str; 11] = [
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_base_leak_tray.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_multi_channel_pump_mounts.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_parallel_tubing_lanes.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_pressure_flow_sensor_ports.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_occlusion_valve_bridge.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_reference_reservoir_rack.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_crosstalk_witness_manifold.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_restrictor_coupon_bank.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_traceability_label_lands.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_service_keepouts.stl",
    "output/closed_perfusion_pump_cross_channel_crosstalk_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "multi_channel_pump_mounting",
    "parallel_tubing_lanes",
    "pressure_flow_sensor_ports",
    "occlusion_valves",
    "reference_reservoirs",
    "crosstalk_witness_manifold",
    "restrictor_coupon_bank",
    "leak_capture_base",
    "traceability_label_lands",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const DECK_Z: f64 = 20.0;
const LEAK_BASIN_X: f64 = STATION_X - 108.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 94.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;

const PUMP_MODULES: usize = 4;
const LANES: usize = 8;
const LANES_PER_PUMP: usize = LANES / PUMP_MODULES;
const LANE_PITCH_X: f64 = 118.0;
const PUMP_PITCH_X: f64 = 214.0;

const PUMP_RAIL_X: f64 = 914.0;
const PUMP_RAIL_Y: f64 = 176.0;
const PUMP_RAIL_Z: f64 = 38.0;
const PUMP_RAIL_CENTER_Y: f64 = 246.0;
const PUMP_ENVELOPE_X: f64 = 148.0;
const PUMP_ENVELOPE_Y: f64 = 118.0;
const PUMP_ENVELOPE_Z: f64 = 72.0;
const PUMP_DATUM_PIN_D: f64 = 10.0;

const TUBING_DECK_X: f64 = 1004.0;
const TUBING_DECK_Y: f64 = 134.0;
const TUBING_DECK_Z: f64 = 26.0;
const TUBING_DECK_CENTER_Y: f64 = 92.0;
const TUBE_OD_MAX: f64 = 6.4;
const TUBE_CLEARANCE: f64 = 1.2;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const STRAIN_RELIEF_TOOTH_X: f64 = 15.0;
const STRAIN_RELIEF_TOOTH_Y: f64 = 44.0;
const STRAIN_RELIEF_TOOTH_Z: f64 = 36.0;

const SENSOR_BAR_X: f64 = 996.0;
const SENSOR_BAR_Y: f64 = 104.0;
const SENSOR_BAR_Z: f64 = 46.0;
const SENSOR_BAR_CENTER_Y: f64 = -62.0;
const PRESSURE_SENSOR_X: f64 = 44.0;
const PRESSURE_SENSOR_Y: f64 = 32.0;
const PRESSURE_SENSOR_Z: f64 = 28.0;
const FLOW_SENSOR_X: f64 = 56.0;
const FLOW_SENSOR_Y: f64 = 34.0;
const FLOW_SENSOR_Z: f64 = 28.0;
const SENSOR_TAP_D: f64 = 8.0;

const OCCLUSION_BRIDGE_X: f64 = 1000.0;
const OCCLUSION_BRIDGE_Y: f64 = 122.0;
const OCCLUSION_BRIDGE_Z: f64 = 44.0;
const OCCLUSION_BRIDGE_CENTER_Y: f64 = -184.0;
const VALVE_ACTUATOR_D: f64 = 28.0;
const VALVE_ACTUATOR_Z: f64 = 42.0;
const BYPASS_PINCH_D: f64 = 18.0;

const RESERVOIR_RACK_X: f64 = 544.0;
const RESERVOIR_RACK_Y: f64 = 178.0;
const RESERVOIR_RACK_Z: f64 = 32.0;
const RESERVOIR_RACK_CENTER_X: f64 = -296.0;
const RESERVOIR_RACK_CENTER_Y: f64 = -306.0;
const RESERVOIRS: usize = 6;
const RESERVOIR_PITCH_X: f64 = 82.0;
const RESERVOIR_WELL_D: f64 = 54.0;
const RESERVOIR_ID_LANDS: usize = RESERVOIRS;

const WITNESS_MANIFOLD_X: f64 = 404.0;
const WITNESS_MANIFOLD_Y: f64 = 166.0;
const WITNESS_MANIFOLD_Z: f64 = 54.0;
const WITNESS_MANIFOLD_CENTER_X: f64 = 356.0;
const WITNESS_MANIFOLD_CENTER_Y: f64 = -304.0;
const WITNESS_CHANNELS: usize = 4;
const WITNESS_PITCH_X: f64 = 86.0;
const WITNESS_WINDOW_X: f64 = 42.0;
const WITNESS_WINDOW_Y: f64 = 14.0;
const WITNESS_WINDOW_Z: f64 = 32.0;

const COUPON_BANK_X: f64 = 386.0;
const COUPON_BANK_Y: f64 = 124.0;
const COUPON_BANK_Z: f64 = 30.0;
const COUPON_BANK_CENTER_X: f64 = -388.0;
const COUPON_BANK_CENTER_Y: f64 = -168.0;
const RESTRICTOR_COUPONS: usize = 6;
const COUPON_SLOT_X: f64 = 48.0;
const COUPON_SLOT_Y: f64 = 82.0;
const COUPON_SLOT_Z: f64 = 22.0;
const COUPON_PITCH_X: f64 = 58.0;

const TRACE_PANEL_X: f64 = 742.0;
const TRACE_PANEL_Y: f64 = 76.0;
const TRACE_PANEL_Z: f64 = 12.0;
const TRACE_PANEL_CENTER_Y: f64 = STATION_Y / 2.0 - 54.0;
const LANE_LABEL_LANDS: usize = LANES;
const RUN_LABEL_LANDS: usize = 5;

const KEEP_OUT_X: f64 = 1060.0;
const KEEP_OUT_Y: f64 = 168.0;
const KEEP_OUT_Z: f64 = 154.0;
const FRONT_SERVICE_Y: f64 = 226.0;
const REAR_SERVICE_Y: f64 = 182.0;
const SENSOR_SERVICE_X: f64 = 1040.0;
const RESERVOIR_SERVICE_X: f64 = 612.0;
const SERVICE_CLEARANCE_MIN: f64 = 150.0;

fn main() {
    assert_design_constraints();
    fs::create_dir_all("output").unwrap();

    let base = base_leak_tray();
    let pumps = multi_channel_pump_mounts();
    let tubing = parallel_tubing_lanes();
    let sensors = pressure_flow_sensor_ports();
    let occlusion = occlusion_valve_bridge();
    let reservoirs = reference_reservoir_rack();
    let witness = crosstalk_witness_manifold();
    let coupons = restrictor_coupon_bank();
    let labels = traceability_label_lands();
    let keepouts = service_keepouts();

    export(&base, OUTPUTS[0]);
    export(&pumps, OUTPUTS[1]);
    export(&tubing, OUTPUTS[2]);
    export(&sensors, OUTPUTS[3]);
    export(&occlusion, OUTPUTS[4]);
    export(&reservoirs, OUTPUTS[5]);
    export(&witness, OUTPUTS[6]);
    export(&coupons, OUTPUTS[7]);
    export(&labels, OUTPUTS[8]);
    export(&keepouts, OUTPUTS[9]);

    let assembly = base
        + pumps
        + tubing
        + sensors
        + occlusion
        + reservoirs
        + witness
        + coupons
        + labels
        + keepouts;
    export(&assembly, OUTPUTS[10]);

    println!(
        "Closed perfusion pump crosstalk station: {STATION_X:.0}mm x {STATION_Y:.0}mm leak-tray deck, {PUMP_MODULES} pump modules, {LANES} parallel tubing lanes, {LANES_PER_PUMP} lanes per pump module, {LANES} pressure/flow sensor pairs, {LANES} occlusion valve positions, and {RESERVOIRS} reference reservoir wells."
    );
    println!(
        "Crosstalk challenge coverage: {WITNESS_CHANNELS} witness bypass windows, {RESTRICTOR_COUPONS} restrictor coupon slots, {LANE_LABEL_LANDS} lane label lands, {RUN_LABEL_LANDS} run-record lands, {} required feature groups, and output prefix {OUTPUT_PREFIX}.",
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_perfusion_crosstalk_station_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        "closed_perfusion_crosstalk_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.5);
    let drain = centered_cylinder(
        "closed_perfusion_crosstalk_station_drain_bore",
        DRAIN_D / 2.0,
        DECK_Z + 18.0,
        32,
    )
    .translate(STATION_X / 2.0 - 80.0, -(STATION_Y / 2.0 - 44.0), 0.0);

    deck - basin - drain + leak_tray_rims() + base_mounting_bosses() + station_datum_fiducials()
}

fn leak_tray_rims() -> Part {
    let side_z = DECK_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        "closed_perfusion_crosstalk_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, side_z);
    let right = centered_cube(
        "closed_perfusion_crosstalk_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, side_z);
    let rear = centered_cube(
        "closed_perfusion_crosstalk_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, side_z);
    let front = centered_cube(
        "closed_perfusion_crosstalk_station_front_low_rim",
        STATION_X,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn base_mounting_bosses() -> Part {
    let mut bosses = Part::empty("closed_perfusion_crosstalk_station_mounting_bosses");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_perfusion_crosstalk_station_mount_boss_{i}"),
            18.0,
            8.0,
            32,
        )
        .translate(x, y, DECK_Z / 2.0 + 4.0);
        let hole = centered_cylinder(
            format!("closed_perfusion_crosstalk_station_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            16.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn station_datum_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_perfusion_crosstalk_station_datum_fiducials");
    for i in 0..4 {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_floor_fiducial_{i}"),
                9.0,
                2.5,
                32,
            )
            .translate(
                lane_x(i, 4, 282.0),
                STATION_Y / 2.0 - 88.0,
                DECK_Z / 2.0 + 1.25,
            );
    }
    fiducials
}

fn multi_channel_pump_mounts() -> Part {
    let rail = centered_cube(
        "closed_perfusion_crosstalk_station_pump_mount_rail",
        PUMP_RAIL_X,
        PUMP_RAIL_Y,
        PUMP_RAIL_Z,
    )
    .translate(0.0, PUMP_RAIL_CENTER_Y, DECK_Z / 2.0 + PUMP_RAIL_Z / 2.0);
    let mut cutouts = Part::empty("closed_perfusion_crosstalk_station_pump_recesses");
    let mut features = Part::empty("closed_perfusion_crosstalk_station_pump_mount_features");

    for pump in 0..PUMP_MODULES {
        let x = lane_x(pump, PUMP_MODULES, PUMP_PITCH_X);
        cutouts = cutouts
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_pump_envelope_recess_{pump}"),
                PUMP_ENVELOPE_X,
                PUMP_ENVELOPE_Y,
                24.0,
            )
            .translate(
                x,
                PUMP_RAIL_CENTER_Y,
                DECK_Z / 2.0 + PUMP_RAIL_Z / 2.0 + 6.0,
            );

        features = features
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_pump_module_envelope_{pump}"),
                PUMP_ENVELOPE_X - 18.0,
                PUMP_ENVELOPE_Y - 18.0,
                PUMP_ENVELOPE_Z,
            )
            .translate(
                x,
                PUMP_RAIL_CENTER_Y,
                DECK_Z / 2.0 + PUMP_RAIL_Z + PUMP_ENVELOPE_Z / 2.0,
            );

        for (pin_i, dx) in [-54.0, 54.0].iter().copied().enumerate() {
            features = features
                + centered_cylinder(
                    format!("closed_perfusion_crosstalk_station_pump_datum_pin_{pump}_{pin_i}"),
                    PUMP_DATUM_PIN_D / 2.0,
                    24.0,
                    24,
                )
                .translate(
                    x + dx,
                    PUMP_RAIL_CENTER_Y - PUMP_ENVELOPE_Y / 2.0 - 15.0,
                    DECK_Z / 2.0 + PUMP_RAIL_Z + 12.0,
                );
        }

        for lane in 0..LANES_PER_PUMP {
            let lane_x = x + (lane as f64 - 0.5) * 44.0;
            cutouts = cutouts
                + centered_cylinder(
                    format!("closed_perfusion_crosstalk_station_pump_tube_exit_{pump}_{lane}"),
                    TUBE_CHANNEL_D / 2.0,
                    PUMP_RAIL_Y + 24.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    lane_x,
                    PUMP_RAIL_CENTER_Y,
                    DECK_Z / 2.0 + PUMP_RAIL_Z + 6.0,
                );
        }
    }

    rail - cutouts + features + pump_pair_isolation_fences()
}

fn pump_pair_isolation_fences() -> Part {
    let mut fences = Part::empty("closed_perfusion_crosstalk_station_pump_isolation_fences");
    for (i, x) in [-107.0, 107.0].iter().copied().enumerate() {
        fences = fences
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_pump_pair_isolation_fence_{i}"),
                10.0,
                PUMP_RAIL_Y + 34.0,
                64.0,
            )
            .translate(x, PUMP_RAIL_CENTER_Y, DECK_Z / 2.0 + PUMP_RAIL_Z + 32.0);
    }
    fences
}

fn parallel_tubing_lanes() -> Part {
    let deck = centered_cube(
        "closed_perfusion_crosstalk_station_tubing_lane_deck",
        TUBING_DECK_X,
        TUBING_DECK_Y,
        TUBING_DECK_Z,
    )
    .translate(
        0.0,
        TUBING_DECK_CENTER_Y,
        DECK_Z / 2.0 + TUBING_DECK_Z / 2.0,
    );
    let mut troughs = Part::empty("closed_perfusion_crosstalk_station_parallel_tube_troughs");
    let mut teeth = Part::empty("closed_perfusion_crosstalk_station_strain_relief_teeth");
    let mut lane_dividers = Part::empty("closed_perfusion_crosstalk_station_lane_dividers");

    for lane in 0..LANES {
        let x = lane_center(lane);
        troughs = troughs
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_lane_tube_trough_{lane}"),
                TUBE_CHANNEL_D / 2.0,
                TUBING_DECK_Y + 30.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, TUBING_DECK_CENTER_Y, DECK_Z / 2.0 + TUBING_DECK_Z + 3.0);

        for (tooth_i, y) in [
            TUBING_DECK_CENTER_Y - TUBING_DECK_Y / 2.0 + 24.0,
            TUBING_DECK_CENTER_Y + TUBING_DECK_Y / 2.0 - 24.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            teeth = teeth
                + centered_cube(
                    format!(
                        "closed_perfusion_crosstalk_station_lane_strain_relief_tooth_{lane}_{tooth_i}"
                    ),
                    STRAIN_RELIEF_TOOTH_X,
                    STRAIN_RELIEF_TOOTH_Y,
                    STRAIN_RELIEF_TOOTH_Z,
                )
                .translate(
                    x,
                    y,
                    DECK_Z / 2.0 + TUBING_DECK_Z + STRAIN_RELIEF_TOOTH_Z / 2.0,
                );
        }

        if lane + 1 < LANES {
            lane_dividers = lane_dividers
                + centered_cube(
                    format!("closed_perfusion_crosstalk_station_lane_separation_wall_{lane}"),
                    5.0,
                    TUBING_DECK_Y + 20.0,
                    54.0,
                )
                .translate(
                    x + LANE_PITCH_X / 2.0,
                    TUBING_DECK_CENTER_Y,
                    DECK_Z / 2.0 + TUBING_DECK_Z + 27.0,
                );
        }
    }

    deck - troughs + teeth + lane_dividers + tubing_lane_direction_markers()
}

fn tubing_lane_direction_markers() -> Part {
    let mut markers = Part::empty("closed_perfusion_crosstalk_station_lane_direction_markers");
    for lane in 0..LANES {
        markers = markers
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_lane_flow_arrow_land_{lane}"),
                46.0,
                12.0,
                3.0,
            )
            .translate(
                lane_center(lane),
                TUBING_DECK_CENTER_Y - TUBING_DECK_Y / 2.0 - 15.0,
                DECK_Z / 2.0 + 3.0,
            );
    }
    markers
}

fn pressure_flow_sensor_ports() -> Part {
    let bar = centered_cube(
        "closed_perfusion_crosstalk_station_pressure_flow_sensor_bar",
        SENSOR_BAR_X,
        SENSOR_BAR_Y,
        SENSOR_BAR_Z,
    )
    .translate(0.0, SENSOR_BAR_CENTER_Y, DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0);
    let mut pockets = Part::empty("closed_perfusion_crosstalk_station_sensor_recesses");
    let mut ports = Part::empty("closed_perfusion_crosstalk_station_sensor_ports");

    for lane in 0..LANES {
        let x = lane_center(lane);
        pockets = pockets
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_pressure_sensor_pocket_{lane}"),
                PRESSURE_SENSOR_X,
                PRESSURE_SENSOR_Y,
                PRESSURE_SENSOR_Z,
            )
            .translate(
                x - 20.0,
                SENSOR_BAR_CENTER_Y + 22.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0 + 6.0,
            )
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_flow_sensor_pocket_{lane}"),
                FLOW_SENSOR_X,
                FLOW_SENSOR_Y,
                FLOW_SENSOR_Z,
            )
            .translate(
                x + 20.0,
                SENSOR_BAR_CENTER_Y - 22.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0 + 6.0,
            );

        ports = ports
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_pressure_tap_port_{lane}"),
                SENSOR_TAP_D / 2.0,
                22.0,
                24,
            )
            .translate(
                x - 20.0,
                SENSOR_BAR_CENTER_Y + SENSOR_BAR_Y / 2.0 + 11.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0,
            )
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_flow_tap_port_{lane}"),
                SENSOR_TAP_D / 2.0,
                22.0,
                24,
            )
            .translate(
                x + 20.0,
                SENSOR_BAR_CENTER_Y - SENSOR_BAR_Y / 2.0 - 11.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0,
            )
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_sensor_cable_exit_{lane}"),
                34.0,
                8.0,
                12.0,
            )
            .translate(x, SENSOR_BAR_CENTER_Y, DECK_Z / 2.0 + SENSOR_BAR_Z + 6.0);
    }

    bar - pockets + ports + sensor_pair_label_lands()
}

fn sensor_pair_label_lands() -> Part {
    let mut lands = Part::empty("closed_perfusion_crosstalk_station_sensor_pair_labels");
    for lane in 0..LANES {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_sensor_pair_label_land_{lane}"),
                58.0,
                16.0,
                3.0,
            )
            .translate(
                lane_center(lane),
                SENSOR_BAR_CENTER_Y,
                DECK_Z / 2.0 + SENSOR_BAR_Z + 1.5,
            );
    }
    lands
}

fn occlusion_valve_bridge() -> Part {
    let bridge = centered_cube(
        "closed_perfusion_crosstalk_station_occlusion_valve_bridge",
        OCCLUSION_BRIDGE_X,
        OCCLUSION_BRIDGE_Y,
        OCCLUSION_BRIDGE_Z,
    )
    .translate(
        0.0,
        OCCLUSION_BRIDGE_CENTER_Y,
        DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z / 2.0,
    );
    let mut valve_windows = Part::empty("closed_perfusion_crosstalk_station_occlusion_windows");
    let mut valve_features = Part::empty("closed_perfusion_crosstalk_station_occlusion_features");

    for lane in 0..LANES {
        let x = lane_center(lane);
        valve_windows = valve_windows
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_pinch_tube_window_{lane}"),
                TUBE_CHANNEL_D / 2.0,
                OCCLUSION_BRIDGE_Y + 28.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                OCCLUSION_BRIDGE_CENTER_Y,
                DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z + 3.0,
            );

        valve_features = valve_features
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_occlusion_actuator_boss_{lane}"),
                VALVE_ACTUATOR_D / 2.0,
                VALVE_ACTUATOR_Z,
                32,
            )
            .translate(
                x,
                OCCLUSION_BRIDGE_CENTER_Y + 24.0,
                DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z + VALVE_ACTUATOR_Z / 2.0,
            )
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_bypass_pinch_reference_{lane}"),
                BYPASS_PINCH_D / 2.0,
                18.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x,
                OCCLUSION_BRIDGE_CENTER_Y - OCCLUSION_BRIDGE_Y / 2.0 - 8.0,
                DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z / 2.0,
            )
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_occlusion_state_land_{lane}"),
                46.0,
                18.0,
                3.0,
            )
            .translate(
                x,
                OCCLUSION_BRIDGE_CENTER_Y + OCCLUSION_BRIDGE_Y / 2.0 - 16.0,
                DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z + 1.5,
            );
    }

    bridge - valve_windows + valve_features + occlusion_group_barriers()
}

fn occlusion_group_barriers() -> Part {
    let mut barriers = Part::empty("closed_perfusion_crosstalk_station_occlusion_group_barriers");
    for (i, x) in [-236.0, 0.0, 236.0].iter().copied().enumerate() {
        barriers = barriers
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_occlusion_group_barrier_{i}"),
                8.0,
                OCCLUSION_BRIDGE_Y + 28.0,
                70.0,
            )
            .translate(
                x,
                OCCLUSION_BRIDGE_CENTER_Y,
                DECK_Z / 2.0 + OCCLUSION_BRIDGE_Z + 35.0,
            );
    }
    barriers
}

fn reference_reservoir_rack() -> Part {
    let rack = centered_cube(
        "closed_perfusion_crosstalk_station_reference_reservoir_rack",
        RESERVOIR_RACK_X,
        RESERVOIR_RACK_Y,
        RESERVOIR_RACK_Z,
    )
    .translate(
        RESERVOIR_RACK_CENTER_X,
        RESERVOIR_RACK_CENTER_Y,
        DECK_Z / 2.0 + RESERVOIR_RACK_Z / 2.0,
    );
    let mut wells = Part::empty("closed_perfusion_crosstalk_station_reservoir_well_cutouts");
    let mut collars = Part::empty("closed_perfusion_crosstalk_station_reservoir_well_collars");
    let mut labels = Part::empty("closed_perfusion_crosstalk_station_reservoir_id_lands");

    for reservoir in 0..RESERVOIRS {
        let x = RESERVOIR_RACK_CENTER_X + lane_x(reservoir, RESERVOIRS, RESERVOIR_PITCH_X);
        wells = wells
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_reservoir_well_{reservoir}"),
                RESERVOIR_WELL_D / 2.0,
                RESERVOIR_RACK_Z + 8.0,
                40,
            )
            .translate(
                x,
                RESERVOIR_RACK_CENTER_Y + 22.0,
                DECK_Z / 2.0 + RESERVOIR_RACK_Z / 2.0,
            );
        collars = collars
            + reservoir_collar(reservoir).translate(
                x,
                RESERVOIR_RACK_CENTER_Y + 22.0,
                DECK_Z / 2.0 + RESERVOIR_RACK_Z + 9.0,
            );
        labels = labels
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_reservoir_barcode_land_{reservoir}"),
                52.0,
                18.0,
                3.0,
            )
            .translate(
                x,
                RESERVOIR_RACK_CENTER_Y - RESERVOIR_RACK_Y / 2.0 + 20.0,
                DECK_Z / 2.0 + RESERVOIR_RACK_Z + 1.5,
            );
    }

    rack - wells + collars + labels + reservoir_headspace_port_bar()
}

fn reservoir_collar(index: usize) -> Part {
    centered_cylinder(
        format!("closed_perfusion_crosstalk_station_reservoir_outer_collar_{index}"),
        RESERVOIR_WELL_D / 2.0 + 5.0,
        18.0,
        40,
    ) - centered_cylinder(
        format!("closed_perfusion_crosstalk_station_reservoir_inner_clearance_{index}"),
        RESERVOIR_WELL_D / 2.0,
        20.0,
        40,
    )
}

fn reservoir_headspace_port_bar() -> Part {
    let bar = centered_cube(
        "closed_perfusion_crosstalk_station_reservoir_headspace_port_bar",
        RESERVOIR_RACK_X - 34.0,
        28.0,
        28.0,
    )
    .translate(
        RESERVOIR_RACK_CENTER_X,
        RESERVOIR_RACK_CENTER_Y + RESERVOIR_RACK_Y / 2.0 + 22.0,
        DECK_Z / 2.0 + RESERVOIR_RACK_Z + 14.0,
    );
    let mut ports = Part::empty("closed_perfusion_crosstalk_station_reservoir_headspace_ports");
    for reservoir in 0..RESERVOIRS {
        ports = ports
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_headspace_reference_port_{reservoir}"),
                SENSOR_TAP_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                RESERVOIR_RACK_CENTER_X + lane_x(reservoir, RESERVOIRS, RESERVOIR_PITCH_X),
                RESERVOIR_RACK_CENTER_Y + RESERVOIR_RACK_Y / 2.0 + 22.0,
                DECK_Z / 2.0 + RESERVOIR_RACK_Z + 14.0,
            );
    }
    bar - ports
}

fn crosstalk_witness_manifold() -> Part {
    let body = centered_cube(
        "closed_perfusion_crosstalk_station_witness_manifold_body",
        WITNESS_MANIFOLD_X,
        WITNESS_MANIFOLD_Y,
        WITNESS_MANIFOLD_Z,
    )
    .translate(
        WITNESS_MANIFOLD_CENTER_X,
        WITNESS_MANIFOLD_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_MANIFOLD_Z / 2.0,
    );
    let mut windows = Part::empty("closed_perfusion_crosstalk_station_witness_windows");
    let mut bypass_ports = Part::empty("closed_perfusion_crosstalk_station_witness_bypass_ports");
    let mut witness_lands = Part::empty("closed_perfusion_crosstalk_station_witness_result_lands");

    for witness in 0..WITNESS_CHANNELS {
        let x = WITNESS_MANIFOLD_CENTER_X + lane_x(witness, WITNESS_CHANNELS, WITNESS_PITCH_X);
        windows = windows
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_crosstalk_sight_window_{witness}"),
                WITNESS_WINDOW_X,
                WITNESS_WINDOW_Y,
                WITNESS_WINDOW_Z,
            )
            .translate(
                x,
                WITNESS_MANIFOLD_CENTER_Y + WITNESS_MANIFOLD_Y / 2.0 - 7.0,
                DECK_Z / 2.0 + WITNESS_MANIFOLD_Z / 2.0,
            );

        for (port_i, y) in [
            WITNESS_MANIFOLD_CENTER_Y - WITNESS_MANIFOLD_Y / 2.0 + 24.0,
            WITNESS_MANIFOLD_CENTER_Y + WITNESS_MANIFOLD_Y / 2.0 - 24.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            bypass_ports = bypass_ports
                + centered_cylinder(
                    format!(
                        "closed_perfusion_crosstalk_station_witness_bypass_port_{witness}_{port_i}"
                    ),
                    TUBE_CHANNEL_D / 2.0,
                    28.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, DECK_Z / 2.0 + WITNESS_MANIFOLD_Z / 2.0);
        }

        witness_lands = witness_lands
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_delta_pressure_label_land_{witness}"),
                68.0,
                20.0,
                3.0,
            )
            .translate(
                x,
                WITNESS_MANIFOLD_CENTER_Y,
                DECK_Z / 2.0 + WITNESS_MANIFOLD_Z + 1.5,
            );
    }

    body - windows - bypass_ports + witness_lands + witness_optic_bosses()
}

fn witness_optic_bosses() -> Part {
    let mut bosses = Part::empty("closed_perfusion_crosstalk_station_witness_optic_bosses");
    for witness in 0..WITNESS_CHANNELS {
        let x = WITNESS_MANIFOLD_CENTER_X + lane_x(witness, WITNESS_CHANNELS, WITNESS_PITCH_X);
        for (boss_i, y) in [
            WITNESS_MANIFOLD_CENTER_Y - 40.0,
            WITNESS_MANIFOLD_CENTER_Y + 40.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            bosses = bosses
                + centered_cylinder(
                    format!(
                        "closed_perfusion_crosstalk_station_witness_optic_boss_{witness}_{boss_i}"
                    ),
                    10.0,
                    14.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, DECK_Z / 2.0 + WITNESS_MANIFOLD_Z + 8.0);
        }
    }
    bosses
}

fn restrictor_coupon_bank() -> Part {
    let tray = centered_cube(
        "closed_perfusion_crosstalk_station_restrictor_coupon_bank",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(
        COUPON_BANK_CENTER_X,
        COUPON_BANK_CENTER_Y,
        DECK_Z / 2.0 + COUPON_BANK_Z / 2.0,
    );
    let mut slots = Part::empty("closed_perfusion_crosstalk_station_restrictor_coupon_slots");
    let mut lands = Part::empty("closed_perfusion_crosstalk_station_restrictor_coupon_labels");

    for coupon in 0..RESTRICTOR_COUPONS {
        let x = COUPON_BANK_CENTER_X + lane_x(coupon, RESTRICTOR_COUPONS, COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_restrictor_slot_{coupon}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_SLOT_Z,
            )
            .translate(
                x,
                COUPON_BANK_CENTER_Y + 10.0,
                DECK_Z / 2.0 + COUPON_BANK_Z / 2.0 + 5.0,
            );
        lands = lands
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_restrictor_id_land_{coupon}"),
                COUPON_SLOT_X,
                18.0,
                3.0,
            )
            .translate(
                x,
                COUPON_BANK_CENTER_Y - COUPON_BANK_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + COUPON_BANK_Z + 1.5,
            );
    }

    tray - slots + lands + restrictor_bypass_port_row()
}

fn restrictor_bypass_port_row() -> Part {
    let row = centered_cube(
        "closed_perfusion_crosstalk_station_restrictor_bypass_port_row",
        COUPON_BANK_X - 34.0,
        24.0,
        26.0,
    )
    .translate(
        COUPON_BANK_CENTER_X,
        COUPON_BANK_CENTER_Y + COUPON_BANK_Y / 2.0 + 18.0,
        DECK_Z / 2.0 + COUPON_BANK_Z + 13.0,
    );
    let mut bores = Part::empty("closed_perfusion_crosstalk_station_restrictor_bypass_bores");
    for coupon in 0..RESTRICTOR_COUPONS {
        bores = bores
            + centered_cylinder(
                format!("closed_perfusion_crosstalk_station_restrictor_bypass_bore_{coupon}"),
                TUBE_CHANNEL_D / 2.0,
                32.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                COUPON_BANK_CENTER_X + lane_x(coupon, RESTRICTOR_COUPONS, COUPON_PITCH_X),
                COUPON_BANK_CENTER_Y + COUPON_BANK_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + COUPON_BANK_Z + 13.0,
            );
    }
    row - bores
}

fn traceability_label_lands() -> Part {
    let panel = centered_cube(
        "closed_perfusion_crosstalk_station_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(
        0.0,
        TRACE_PANEL_CENTER_Y,
        DECK_Z / 2.0 + TRACE_PANEL_Z / 2.0,
    );
    let mut lands = Part::empty("closed_perfusion_crosstalk_station_traceability_lands");

    for lane in 0..LANE_LABEL_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_lane_barcode_land_{lane}"),
                54.0,
                20.0,
                3.0,
            )
            .translate(
                lane_x(lane, LANE_LABEL_LANDS, 76.0),
                TRACE_PANEL_CENTER_Y - 18.0,
                DECK_Z / 2.0 + TRACE_PANEL_Z + 1.5,
            );
    }

    for run in 0..RUN_LABEL_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_perfusion_crosstalk_station_run_record_land_{run}"),
                74.0,
                22.0,
                3.0,
            )
            .translate(
                -166.0 + run as f64 * 83.0,
                TRACE_PANEL_CENTER_Y + 22.0,
                DECK_Z / 2.0 + TRACE_PANEL_Z + 1.5,
            );
    }

    panel + lands + label_guard_rail()
}

fn label_guard_rail() -> Part {
    centered_cube(
        "closed_perfusion_crosstalk_station_label_guard_rail",
        TRACE_PANEL_X + 30.0,
        10.0,
        30.0,
    )
    .translate(
        0.0,
        TRACE_PANEL_CENTER_Y + TRACE_PANEL_Y / 2.0 + 10.0,
        DECK_Z / 2.0 + TRACE_PANEL_Z + 15.0,
    )
}

fn service_keepouts() -> Part {
    let robot_bridge = centered_cube(
        "closed_perfusion_crosstalk_station_robot_keepout_bridge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        10.0,
    )
    .translate(0.0, 22.0, DECK_Z / 2.0 + KEEP_OUT_Z);
    let front_service = centered_cube(
        "closed_perfusion_crosstalk_station_front_service_keepout",
        KEEP_OUT_X,
        10.0,
        82.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_Y / 2.0),
        DECK_Z / 2.0 + 41.0,
    );
    let rear_service = centered_cube(
        "closed_perfusion_crosstalk_station_rear_service_keepout",
        KEEP_OUT_X,
        10.0,
        82.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_Y / 2.0,
        DECK_Z / 2.0 + 41.0,
    );
    let sensor_service = centered_cube(
        "closed_perfusion_crosstalk_station_sensor_service_keepout",
        SENSOR_SERVICE_X,
        8.0,
        62.0,
    )
    .translate(0.0, SENSOR_BAR_CENTER_Y, DECK_Z / 2.0 + SENSOR_BAR_Z + 31.0);
    let reservoir_service = centered_cube(
        "closed_perfusion_crosstalk_station_reservoir_service_keepout",
        RESERVOIR_SERVICE_X,
        8.0,
        82.0,
    )
    .translate(
        RESERVOIR_RACK_CENTER_X,
        RESERVOIR_RACK_CENTER_Y,
        DECK_Z / 2.0 + RESERVOIR_RACK_Z + 41.0,
    );

    robot_bridge + front_service + rear_service + sensor_service + reservoir_service
}

fn lane_center(lane: usize) -> f64 {
    lane_x(lane, LANES, LANE_PITCH_X)
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(LANES, PUMP_MODULES * LANES_PER_PUMP);
    assert_eq!(LANES, LANE_LABEL_LANDS);
    assert_eq!(RESERVOIRS, RESERVOIR_ID_LANDS);
    assert_eq!(WITNESS_CHANNELS, LANES / 2);
    assert!(TUBE_CHANNEL_D > TUBE_OD_MAX);
    assert!(SERVICE_CLEARANCE_MIN <= FRONT_SERVICE_Y);
    assert!(SERVICE_CLEARANCE_MIN <= REAR_SERVICE_Y);
    assert!(LEAK_BASIN_X < STATION_X);
    assert!(LEAK_BASIN_Y < STATION_Y);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX));
        assert!(path.ends_with(".stl"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_labeled_and_unique() {
        assert_design_constraints();
        assert!(OUTPUTS[0].contains("base_leak_tray"));
        assert!(OUTPUTS[1].contains("multi_channel_pump_mounts"));
        assert!(OUTPUTS[2].contains("parallel_tubing_lanes"));
        assert!(OUTPUTS[3].contains("pressure_flow_sensor_ports"));
        assert!(OUTPUTS[4].contains("occlusion_valve_bridge"));
        assert!(OUTPUTS[5].contains("reference_reservoir_rack"));
        assert!(OUTPUTS[10].ends_with("_assembly.stl"));
    }

    #[test]
    fn lane_counts_match_cross_channel_plan() {
        assert_eq!(LANES, 8);
        assert_eq!(PUMP_MODULES, 4);
        assert_eq!(LANES_PER_PUMP, 2);
        assert_eq!(WITNESS_CHANNELS, 4);
        assert_eq!(RESTRICTOR_COUPONS, 6);
        assert_eq!(LANE_LABEL_LANDS, LANES);
    }

    #[test]
    fn service_clearance_stays_outside_fixture() {
        assert!(FRONT_SERVICE_Y >= SERVICE_CLEARANCE_MIN);
        assert!(REAR_SERVICE_Y >= SERVICE_CLEARANCE_MIN);
        assert!(KEEP_OUT_X < STATION_X);
        assert!(RESERVOIR_RACK_CENTER_Y < OCCLUSION_BRIDGE_CENTER_Y);
    }
}
