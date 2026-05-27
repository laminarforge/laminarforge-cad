use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed pump/valve manifold calibration station for pre-cell-run checks.
//
// Intent:
// - Package purchased peristaltic/diaphragm pumps, pinch/selector valves,
//   pressure sensors, flow sensors, and balances as a repeatable station used
//   before live-cell perfusion runs.
// - Provide physical references for gravimetric flow checks, pressure/flow
//   sensor mapping, valve actuation verification, calibrated restriction loops,
//   bubble challenge injection, leak capture, waste/flush routing, traceability,
//   clean/used segregation, and robot/service keepouts.
//
// This is interface/packaging CAD only. It does not model safety-critical pump,
// valve, sensor, controller, sterile barrier, calibration algorithm, or wetted
// internals.

const OUTPUTS: &[&str] = &[
    "output/closed_pump_valve_manifold_calibration_station_base_leak_tray.stl",
    "output/closed_pump_valve_manifold_calibration_station_pump_reference_bays.stl",
    "output/closed_pump_valve_manifold_calibration_station_valve_actuation_map_plate.stl",
    "output/closed_pump_valve_manifold_calibration_station_restriction_loop_holders.stl",
    "output/closed_pump_valve_manifold_calibration_station_gravimetric_collection_nests.stl",
    "output/closed_pump_valve_manifold_calibration_station_pressure_flow_sensor_pockets.stl",
    "output/closed_pump_valve_manifold_calibration_station_bubble_challenge_inlet.stl",
    "output/closed_pump_valve_manifold_calibration_station_waste_flush_routing.stl",
    "output/closed_pump_valve_manifold_calibration_station_barcode_run_record_lands.stl",
    "output/closed_pump_valve_manifold_calibration_station_clean_used_segregation.stl",
    "output/closed_pump_valve_manifold_calibration_station_robot_service_keepouts.stl",
    "output/closed_pump_valve_manifold_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_leak_tray",
    "pump_reference_bays",
    "valve_actuation_map_plate",
    "calibrated_restriction_loop_holders",
    "gravimetric_collection_nests",
    "pressure_flow_sensor_pockets",
    "bubble_challenge_inlet",
    "waste_flush_routing",
    "barcode_run_record_lands",
    "clean_used_segregation",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 980.0;
const STATION_Y: f64 = 640.0;
const BASE_Z: f64 = 22.0;
const LEAK_BASIN_X: f64 = STATION_X - 96.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 86.0;
const RIM_Z: f64 = 38.0;
const RIM_W: f64 = 18.0;
const DRAIN_D: f64 = 14.0;

const PUMP_BAY_COUNT: usize = 4;
const PUMP_BAY_X: f64 = 136.0;
const PUMP_BAY_Y: f64 = 164.0;
const PUMP_BAY_Z: f64 = 48.0;
const PUMP_BAY_PITCH_X: f64 = 184.0;
const PUMP_BAY_CENTER_Y: f64 = 196.0;
const PUMP_ENVELOPE_X: f64 = 108.0;
const PUMP_ENVELOPE_Y: f64 = 126.0;
const PUMP_ENVELOPE_Z: f64 = 56.0;

const VALVE_LANES: usize = 8;
const VALVE_PLATE_X: f64 = 826.0;
const VALVE_PLATE_Y: f64 = 94.0;
const VALVE_PLATE_Z: f64 = 18.0;
const VALVE_PLATE_CENTER_Y: f64 = 84.0;
const VALVE_PITCH_X: f64 = 92.0;
const VALVE_ACTUATOR_D: f64 = 26.0;
const VALVE_LABEL_X: f64 = 48.0;
const VALVE_LABEL_Y: f64 = 22.0;

const RESTRICTION_LOOP_COUNT: usize = 6;
const LOOP_HOLDER_X: f64 = 760.0;
const LOOP_HOLDER_Y: f64 = 112.0;
const LOOP_HOLDER_Z: f64 = 34.0;
const LOOP_HOLDER_CENTER_Y: f64 = -36.0;
const LOOP_PITCH_X: f64 = 118.0;
const LOOP_POST_D: f64 = 14.0;
const LOOP_POST_Z: f64 = 52.0;
const LOOP_SPAN_Y: f64 = 54.0;

const COLLECTION_NEST_COUNT: usize = 6;
const NEST_RACK_X: f64 = 820.0;
const NEST_RACK_Y: f64 = 142.0;
const NEST_RACK_Z: f64 = 30.0;
const NEST_RACK_CENTER_Y: f64 = -202.0;
const NEST_PITCH_X: f64 = 122.0;
const BEAKER_D: f64 = 62.0;
const BALANCE_PAD_X: f64 = 92.0;
const BALANCE_PAD_Y: f64 = 96.0;

const SENSOR_POCKET_COUNT: usize = 8;
const SENSOR_BAR_X: f64 = 826.0;
const SENSOR_BAR_Y: f64 = 82.0;
const SENSOR_BAR_Z: f64 = 44.0;
const SENSOR_BAR_CENTER_Y: f64 = -112.0;
const SENSOR_PITCH_X: f64 = 92.0;
const PRESSURE_POCKET_X: f64 = 46.0;
const PRESSURE_POCKET_Y: f64 = 28.0;
const FLOW_POCKET_X: f64 = 54.0;
const FLOW_POCKET_Y: f64 = 32.0;

const BUBBLE_INLET_COUNT: usize = 2;
const BUBBLE_BLOCK_X: f64 = 176.0;
const BUBBLE_BLOCK_Y: f64 = 104.0;
const BUBBLE_BLOCK_Z: f64 = 74.0;
const BUBBLE_BLOCK_CENTER_X: f64 = -(STATION_X / 2.0 - 132.0);
const BUBBLE_BLOCK_CENTER_Y: f64 = -286.0;
const SYRINGE_PORT_D: f64 = 18.0;
const SIGHT_WINDOW_X: f64 = 92.0;
const SIGHT_WINDOW_Y: f64 = 8.0;
const SIGHT_WINDOW_Z: f64 = 34.0;

const WASTE_CHANNEL_COUNT: usize = 8;
const FLUSH_PORT_COUNT: usize = 8;
const ROUTING_BAR_X: f64 = 214.0;
const ROUTING_BAR_Y: f64 = 532.0;
const ROUTING_BAR_Z: f64 = 42.0;
const ROUTING_BAR_CENTER_X: f64 = STATION_X / 2.0 - 118.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const TUBE_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const BARCODE_LAND_COUNT: usize = 12;
const RUN_RECORD_LAND_COUNT: usize = 4;
const LAND_PLATE_X: f64 = 782.0;
const LAND_PLATE_Y: f64 = 72.0;
const LAND_PLATE_Z: f64 = 8.0;
const LAND_PLATE_CENTER_Y: f64 = STATION_Y / 2.0 - 52.0;

const SEGREGATION_RIB_X: f64 = 18.0;
const SEGREGATION_RIB_Y: f64 = STATION_Y - 88.0;
const SEGREGATION_RIB_Z: f64 = 96.0;
const CLEAN_ZONE_CENTER_X: f64 = -246.0;
const USED_ZONE_CENTER_X: f64 = 246.0;
const CLEAN_USED_MIN_GAP: f64 = 72.0;

const ROBOT_KEEP_OUT_X: f64 = 900.0;
const ROBOT_KEEP_OUT_Y: f64 = 168.0;
const ROBOT_KEEP_OUT_Z: f64 = 138.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 3;
const FRONT_SERVICE_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const SENSOR_SERVICE_CLEARANCE: f64 = 150.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let pumps = pump_reference_bays();
    export(&pumps, OUTPUTS[1]);

    let valves = valve_actuation_map_plate();
    export(&valves, OUTPUTS[2]);

    let loops = calibrated_restriction_loop_holders();
    export(&loops, OUTPUTS[3]);

    let collection = gravimetric_collection_nests();
    export(&collection, OUTPUTS[4]);

    let sensors = pressure_flow_sensor_pockets();
    export(&sensors, OUTPUTS[5]);

    let bubble = bubble_challenge_inlet();
    export(&bubble, OUTPUTS[6]);

    let routing = waste_flush_routing();
    export(&routing, OUTPUTS[7]);

    let traceability = barcode_run_record_lands();
    export(&traceability, OUTPUTS[8]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + pumps
        + valves
        + loops
        + collection
        + sensors
        + bubble
        + routing
        + traceability
        + segregation
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed pump/valve manifold calibration station: {:.0}mm x {:.0}mm leak-tray base, {} pump reference bays, {} valve lanes, {} calibrated restriction loop holders, {} gravimetric collection nests, {} pressure/flow sensor pockets, {} bubble challenge inlets, {} flush ports, and {} waste channels.",
        STATION_X,
        STATION_Y,
        PUMP_BAY_COUNT,
        VALVE_LANES,
        RESTRICTION_LOOP_COUNT,
        COLLECTION_NEST_COUNT,
        SENSOR_POCKET_COUNT,
        BUBBLE_INLET_COUNT,
        FLUSH_PORT_COUNT,
        WASTE_CHANNEL_COUNT
    );
    println!(
        "Traceability and operation: {} barcode/run-record lands, clean/used segregation gap {:.0}mm, {:.0}mm front service clearance, {:.0}mm rear service clearance, {} robot keepout windows, and {} required feature groups.",
        BARCODE_LAND_COUNT + RUN_RECORD_LAND_COUNT,
        CLEAN_USED_MIN_GAP,
        FRONT_SERVICE_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        ROBOT_KEEP_OUT_WINDOWS,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_pump_valve_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_pump_valve_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder(
        "closed_pump_valve_station_leak_tray_drain",
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -(STATION_Y / 2.0 - 18.0), 0.0);

    deck - basin - drain + tray_rims() + mounting_slots() + datum_fiducials()
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "closed_pump_valve_station_left_leak_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_pump_valve_station_right_leak_rim",
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
        "closed_pump_valve_station_rear_leak_rim",
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
        "closed_pump_valve_station_front_low_leak_lip",
        STATION_X - 118.0,
        12.0,
        22.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 18.0), BASE_Z / 2.0 + 11.0);

    left + right + rear + front
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_pump_valve_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_pump_valve_station_m6_mount_bore_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_pump_valve_station_m6_mount_slot_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_pump_valve_station_datum_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 84.0), STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 84.0, STATION_Y / 2.0 - 86.0),
        (-(STATION_X / 2.0 - 84.0), -(STATION_Y / 2.0 - 86.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_pump_valve_station_robot_datum_disc_{i}"),
                14.0,
                4.0,
                32,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0)
            - centered_cylinder(
                format!("closed_pump_valve_station_robot_datum_center_{i}"),
                3.0,
                6.0,
                20,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn pump_reference_bays() -> Part {
    let mut bays = Part::empty("closed_pump_valve_station_pump_reference_bays");
    for lane in 0..PUMP_BAY_COUNT {
        let x = lane_x(lane, PUMP_BAY_COUNT, PUMP_BAY_PITCH_X);
        let tray = centered_cube(
            format!("closed_pump_valve_station_pump_bay_tray_{lane}"),
            PUMP_BAY_X,
            PUMP_BAY_Y,
            PUMP_BAY_Z,
        )
        .translate(x, PUMP_BAY_CENTER_Y, BASE_Z / 2.0 + PUMP_BAY_Z / 2.0);
        let pump_envelope = centered_cube(
            format!("closed_pump_valve_station_purchased_pump_envelope_{lane}"),
            PUMP_ENVELOPE_X,
            PUMP_ENVELOPE_Y,
            PUMP_ENVELOPE_Z,
        )
        .translate(
            x,
            PUMP_BAY_CENTER_Y,
            BASE_Z / 2.0 + PUMP_BAY_Z + PUMP_ENVELOPE_Z / 2.0,
        );
        let service_pull = centered_cube(
            format!("closed_pump_valve_station_pump_service_pull_arrow_{lane}"),
            PUMP_BAY_X - 24.0,
            10.0,
            10.0,
        )
        .translate(
            x,
            PUMP_BAY_CENTER_Y - PUMP_BAY_Y / 2.0 - 14.0,
            BASE_Z / 2.0 + PUMP_BAY_Z + 9.0,
        );
        let cable_relief = centered_cube(
            format!("closed_pump_valve_station_pump_cable_relief_{lane}"),
            18.0,
            38.0,
            16.0,
        )
        .translate(
            x + PUMP_BAY_X / 2.0 - 18.0,
            PUMP_BAY_CENTER_Y + PUMP_BAY_Y / 2.0 - 16.0,
            BASE_Z / 2.0 + PUMP_BAY_Z + 8.0,
        );
        let screw_pattern = pump_mount_pattern(lane, x, PUMP_BAY_CENTER_Y);
        bays = bays + tray - pump_envelope - screw_pattern + service_pull + cable_relief;
    }
    bays
}

fn pump_mount_pattern(lane: usize, x: f64, y: f64) -> Part {
    let mut pattern = Part::empty(format!(
        "closed_pump_valve_station_pump_mount_pattern_{lane}"
    ));
    for (i, (dx, dy)) in [(-44.0, -52.0), (44.0, -52.0), (-44.0, 52.0), (44.0, 52.0)]
        .iter()
        .enumerate()
    {
        pattern = pattern
            + centered_cylinder(
                format!("closed_pump_valve_station_pump_m4_clearance_{lane}_{i}"),
                4.5 / 2.0,
                PUMP_BAY_Z + 6.0,
                20,
            )
            .translate(x + dx, y + dy, BASE_Z / 2.0 + PUMP_BAY_Z / 2.0);
    }
    pattern
}

fn valve_actuation_map_plate() -> Part {
    let plate = centered_cube(
        "closed_pump_valve_station_valve_actuation_map_plate",
        VALVE_PLATE_X,
        VALVE_PLATE_Y,
        VALVE_PLATE_Z,
    )
    .translate(
        0.0,
        VALVE_PLATE_CENTER_Y,
        BASE_Z / 2.0 + VALVE_PLATE_Z / 2.0,
    );
    let mut features = Part::empty("closed_pump_valve_station_valve_map_features");
    for lane in 0..VALVE_LANES {
        let x = lane_x(lane, VALVE_LANES, VALVE_PITCH_X);
        features = features
            + centered_cylinder(
                format!("closed_pump_valve_station_valve_actuator_pocket_{lane}"),
                VALVE_ACTUATOR_D / 2.0,
                VALVE_PLATE_Z + 8.0,
                32,
            )
            .translate(
                x,
                VALVE_PLATE_CENTER_Y - 20.0,
                BASE_Z / 2.0 + VALVE_PLATE_Z / 2.0,
            )
            + centered_cube(
                format!("closed_pump_valve_station_valve_state_label_land_{lane}"),
                VALVE_LABEL_X,
                VALVE_LABEL_Y,
                5.0,
            )
            .translate(
                x,
                VALVE_PLATE_CENTER_Y + 24.0,
                BASE_Z / 2.0 + VALVE_PLATE_Z + 2.5,
            )
            + centered_cube(
                format!("closed_pump_valve_station_valve_tubing_trace_{lane}"),
                6.0,
                VALVE_PLATE_Y - 20.0,
                4.0,
            )
            .translate(x, VALVE_PLATE_CENTER_Y, BASE_Z / 2.0 + VALVE_PLATE_Z + 2.0);
    }
    plate - features + valve_lane_index_ticks()
}

fn valve_lane_index_ticks() -> Part {
    let mut ticks = Part::empty("closed_pump_valve_station_valve_lane_index_ticks");
    for lane in 0..=VALVE_LANES {
        let x = -((VALVE_LANES as f64) * VALVE_PITCH_X) / 2.0 + lane as f64 * VALVE_PITCH_X;
        ticks = ticks
            + centered_cube(
                format!("closed_pump_valve_station_valve_lane_tick_{lane}"),
                4.0,
                VALVE_PLATE_Y,
                6.0,
            )
            .translate(x, VALVE_PLATE_CENTER_Y, BASE_Z / 2.0 + VALVE_PLATE_Z + 3.0);
    }
    ticks
}

fn calibrated_restriction_loop_holders() -> Part {
    let rail = centered_cube(
        "closed_pump_valve_station_restriction_loop_holder_rail",
        LOOP_HOLDER_X,
        LOOP_HOLDER_Y,
        LOOP_HOLDER_Z,
    )
    .translate(
        0.0,
        LOOP_HOLDER_CENTER_Y,
        BASE_Z / 2.0 + LOOP_HOLDER_Z / 2.0,
    );
    let mut holders = Part::empty("closed_pump_valve_station_restriction_loop_holders");
    for lane in 0..RESTRICTION_LOOP_COUNT {
        let x = lane_x(lane, RESTRICTION_LOOP_COUNT, LOOP_PITCH_X);
        let posts = centered_cylinder(
            format!("closed_pump_valve_station_loop_front_post_{lane}"),
            LOOP_POST_D / 2.0,
            LOOP_POST_Z,
            24,
        )
        .translate(
            x,
            LOOP_HOLDER_CENTER_Y - LOOP_SPAN_Y / 2.0,
            BASE_Z / 2.0 + LOOP_HOLDER_Z + LOOP_POST_Z / 2.0,
        ) + centered_cylinder(
            format!("closed_pump_valve_station_loop_rear_post_{lane}"),
            LOOP_POST_D / 2.0,
            LOOP_POST_Z,
            24,
        )
        .translate(
            x,
            LOOP_HOLDER_CENTER_Y + LOOP_SPAN_Y / 2.0,
            BASE_Z / 2.0 + LOOP_HOLDER_Z + LOOP_POST_Z / 2.0,
        );
        let capillary_land = centered_cube(
            format!("closed_pump_valve_station_calibrated_capillary_land_{lane}"),
            78.0,
            12.0,
            8.0,
        )
        .translate(x, LOOP_HOLDER_CENTER_Y, BASE_Z / 2.0 + LOOP_HOLDER_Z + 8.0);
        let id_tag = centered_cube(
            format!("closed_pump_valve_station_restriction_id_tag_{lane}"),
            58.0,
            18.0,
            5.0,
        )
        .translate(
            x,
            LOOP_HOLDER_CENTER_Y + LOOP_HOLDER_Y / 2.0 - 16.0,
            BASE_Z / 2.0 + LOOP_HOLDER_Z + 2.5,
        );
        holders = holders + posts + capillary_land + id_tag;
    }
    rail + holders - loop_tubing_grooves()
}

fn loop_tubing_grooves() -> Part {
    let mut grooves = Part::empty("closed_pump_valve_station_restriction_loop_tubing_grooves");
    for lane in 0..RESTRICTION_LOOP_COUNT {
        let x = lane_x(lane, RESTRICTION_LOOP_COUNT, LOOP_PITCH_X);
        grooves = grooves
            + centered_cube(
                format!("closed_pump_valve_station_loop_groove_front_{lane}"),
                82.0,
                TUBE_BORE_D,
                LOOP_HOLDER_Z + 6.0,
            )
            .translate(
                x,
                LOOP_HOLDER_CENTER_Y - LOOP_SPAN_Y / 2.0,
                BASE_Z / 2.0 + LOOP_HOLDER_Z / 2.0,
            )
            + centered_cube(
                format!("closed_pump_valve_station_loop_groove_rear_{lane}"),
                82.0,
                TUBE_BORE_D,
                LOOP_HOLDER_Z + 6.0,
            )
            .translate(
                x,
                LOOP_HOLDER_CENTER_Y + LOOP_SPAN_Y / 2.0,
                BASE_Z / 2.0 + LOOP_HOLDER_Z / 2.0,
            );
    }
    grooves
}

fn gravimetric_collection_nests() -> Part {
    let rack = centered_cube(
        "closed_pump_valve_station_gravimetric_collection_rack",
        NEST_RACK_X,
        NEST_RACK_Y,
        NEST_RACK_Z,
    )
    .translate(0.0, NEST_RACK_CENTER_Y, BASE_Z / 2.0 + NEST_RACK_Z / 2.0);
    let mut nests = Part::empty("closed_pump_valve_station_gravimetric_collection_nests");
    for lane in 0..COLLECTION_NEST_COUNT {
        let x = lane_x(lane, COLLECTION_NEST_COUNT, NEST_PITCH_X);
        let beaker_recess = centered_cylinder(
            format!("closed_pump_valve_station_collection_beaker_recess_{lane}"),
            BEAKER_D / 2.0,
            NEST_RACK_Z + 8.0,
            48,
        )
        .translate(
            x,
            NEST_RACK_CENTER_Y - 18.0,
            BASE_Z / 2.0 + NEST_RACK_Z / 2.0,
        );
        let balance_pad = centered_cube(
            format!("closed_pump_valve_station_balance_pad_land_{lane}"),
            BALANCE_PAD_X,
            BALANCE_PAD_Y,
            6.0,
        )
        .translate(
            x,
            NEST_RACK_CENTER_Y + 12.0,
            BASE_Z / 2.0 + NEST_RACK_Z + 3.0,
        );
        let drip_fence = centered_cube(
            format!("closed_pump_valve_station_collection_drip_fence_{lane}"),
            86.0,
            8.0,
            28.0,
        )
        .translate(
            x,
            NEST_RACK_CENTER_Y - NEST_RACK_Y / 2.0 + 12.0,
            BASE_Z / 2.0 + NEST_RACK_Z + 14.0,
        );
        nests = nests - beaker_recess + balance_pad + drip_fence;
    }
    rack + nests + collection_route_comb()
}

fn collection_route_comb() -> Part {
    let mut comb = Part::empty("closed_pump_valve_station_collection_route_comb");
    for lane in 0..COLLECTION_NEST_COUNT {
        let x = lane_x(lane, COLLECTION_NEST_COUNT, NEST_PITCH_X);
        comb =
            comb + centered_cube(
                format!("closed_pump_valve_station_collection_tube_clip_{lane}"),
                18.0,
                42.0,
                24.0,
            )
            .translate(
                x,
                NEST_RACK_CENTER_Y + NEST_RACK_Y / 2.0 - 16.0,
                BASE_Z / 2.0 + NEST_RACK_Z + 12.0,
            ) - centered_cylinder(
                format!("closed_pump_valve_station_collection_tube_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                24.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                NEST_RACK_CENTER_Y + NEST_RACK_Y / 2.0 - 16.0,
                BASE_Z / 2.0 + NEST_RACK_Z + 12.0,
            );
    }
    comb
}

fn pressure_flow_sensor_pockets() -> Part {
    let bar = centered_cube(
        "closed_pump_valve_station_pressure_flow_sensor_bar",
        SENSOR_BAR_X,
        SENSOR_BAR_Y,
        SENSOR_BAR_Z,
    )
    .translate(0.0, SENSOR_BAR_CENTER_Y, BASE_Z / 2.0 + SENSOR_BAR_Z / 2.0);
    let mut cuts = Part::empty("closed_pump_valve_station_pressure_flow_sensor_cutouts");
    let mut details = Part::empty("closed_pump_valve_station_pressure_flow_sensor_details");
    for lane in 0..SENSOR_POCKET_COUNT {
        let x = lane_x(lane, SENSOR_POCKET_COUNT, SENSOR_PITCH_X);
        let pressure = centered_cube(
            format!("closed_pump_valve_station_pressure_sensor_pocket_{lane}"),
            PRESSURE_POCKET_X,
            PRESSURE_POCKET_Y,
            SENSOR_BAR_Z + 8.0,
        )
        .translate(
            x,
            SENSOR_BAR_CENTER_Y - 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z / 2.0,
        );
        let flow = centered_cube(
            format!("closed_pump_valve_station_flow_sensor_pocket_{lane}"),
            FLOW_POCKET_X,
            FLOW_POCKET_Y,
            SENSOR_BAR_Z + 8.0,
        )
        .translate(
            x,
            SENSOR_BAR_CENTER_Y + 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z / 2.0,
        );
        let cable_land = centered_cube(
            format!("closed_pump_valve_station_sensor_cable_strain_land_{lane}"),
            40.0,
            10.0,
            6.0,
        )
        .translate(
            x,
            SENSOR_BAR_CENTER_Y + SENSOR_BAR_Y / 2.0 + 8.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z + 3.0,
        );
        let pressure_lip = centered_cube(
            format!("closed_pump_valve_station_pressure_sensor_lip_{lane}"),
            PRESSURE_POCKET_X + 14.0,
            PRESSURE_POCKET_Y + 12.0,
            8.0,
        )
        .translate(
            x,
            SENSOR_BAR_CENTER_Y - 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z + 4.0,
        );
        let flow_lip = centered_cube(
            format!("closed_pump_valve_station_flow_sensor_lip_{lane}"),
            FLOW_POCKET_X + 14.0,
            FLOW_POCKET_Y + 12.0,
            8.0,
        )
        .translate(
            x,
            SENSOR_BAR_CENTER_Y + 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z + 4.0,
        );
        let pressure_boss = centered_cylinder(
            format!("closed_pump_valve_station_pressure_sensor_round_datum_{lane}"),
            8.0,
            10.0,
            32,
        )
        .translate(
            x - 18.0,
            SENSOR_BAR_CENTER_Y - 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z + 10.0,
        );
        let flow_boss = centered_cylinder(
            format!("closed_pump_valve_station_flow_sensor_round_datum_{lane}"),
            8.0,
            10.0,
            32,
        )
        .translate(
            x + 18.0,
            SENSOR_BAR_CENTER_Y + 20.0,
            BASE_Z / 2.0 + SENSOR_BAR_Z + 10.0,
        );
        cuts = cuts + pressure + flow;
        details = details + cable_land + pressure_lip + flow_lip + pressure_boss + flow_boss;
    }
    bar - cuts + details
}

fn bubble_challenge_inlet() -> Part {
    let block = centered_cube(
        "closed_pump_valve_station_bubble_challenge_block",
        BUBBLE_BLOCK_X,
        BUBBLE_BLOCK_Y,
        BUBBLE_BLOCK_Z,
    )
    .translate(
        BUBBLE_BLOCK_CENTER_X,
        BUBBLE_BLOCK_CENTER_Y,
        BASE_Z / 2.0 + BUBBLE_BLOCK_Z / 2.0,
    );
    let sight_window = centered_cube(
        "closed_pump_valve_station_bubble_sight_window",
        SIGHT_WINDOW_X,
        SIGHT_WINDOW_Y,
        SIGHT_WINDOW_Z,
    )
    .translate(
        BUBBLE_BLOCK_CENTER_X,
        BUBBLE_BLOCK_CENTER_Y - BUBBLE_BLOCK_Y / 2.0 - 1.0,
        BASE_Z / 2.0 + BUBBLE_BLOCK_Z / 2.0,
    );
    let mut inlets = Part::empty("closed_pump_valve_station_bubble_challenge_inlets");
    for i in 0..BUBBLE_INLET_COUNT {
        let x = BUBBLE_BLOCK_CENTER_X + lane_x(i, BUBBLE_INLET_COUNT, 58.0);
        inlets = inlets
            + centered_cylinder(
                format!("closed_pump_valve_station_luer_syringe_inlet_{i}"),
                SYRINGE_PORT_D / 2.0,
                BUBBLE_BLOCK_Z + 8.0,
                32,
            )
            .translate(
                x,
                BUBBLE_BLOCK_CENTER_Y,
                BASE_Z / 2.0 + BUBBLE_BLOCK_Z / 2.0,
            )
            + centered_cube(
                format!("closed_pump_valve_station_bubble_inlet_clip_land_{i}"),
                46.0,
                18.0,
                8.0,
            )
            .translate(
                x,
                BUBBLE_BLOCK_CENTER_Y + BUBBLE_BLOCK_Y / 2.0 - 18.0,
                BASE_Z / 2.0 + BUBBLE_BLOCK_Z + 4.0,
            );
    }
    block - inlets - sight_window + bubble_bypass_label_lands()
}

fn bubble_bypass_label_lands() -> Part {
    centered_cube(
        "closed_pump_valve_station_bubble_challenge_label_land",
        132.0,
        28.0,
        5.0,
    )
    .translate(
        BUBBLE_BLOCK_CENTER_X,
        BUBBLE_BLOCK_CENTER_Y - BUBBLE_BLOCK_Y / 2.0 + 18.0,
        BASE_Z / 2.0 + BUBBLE_BLOCK_Z + 2.5,
    )
}

fn waste_flush_routing() -> Part {
    let bar = centered_cube(
        "closed_pump_valve_station_waste_flush_routing_bar",
        ROUTING_BAR_X,
        ROUTING_BAR_Y,
        ROUTING_BAR_Z,
    )
    .translate(
        ROUTING_BAR_CENTER_X,
        0.0,
        BASE_Z / 2.0 + ROUTING_BAR_Z / 2.0,
    );
    let mut routing = Part::empty("closed_pump_valve_station_waste_flush_routing_features");
    for lane in 0..WASTE_CHANNEL_COUNT {
        let y = lane_x(lane, WASTE_CHANNEL_COUNT, 56.0);
        routing = routing
            + centered_cube(
                format!("closed_pump_valve_station_waste_channel_groove_{lane}"),
                ROUTING_BAR_X + 8.0,
                TUBE_BORE_D,
                10.0,
            )
            .translate(ROUTING_BAR_CENTER_X, y, BASE_Z / 2.0 + ROUTING_BAR_Z - 7.0)
            + centered_cylinder(
                format!("closed_pump_valve_station_flush_port_socket_{lane}"),
                10.0 / 2.0,
                ROUTING_BAR_Z + 8.0,
                24,
            )
            .translate(
                ROUTING_BAR_CENTER_X - ROUTING_BAR_X / 2.0 + 38.0,
                y,
                BASE_Z / 2.0 + ROUTING_BAR_Z / 2.0,
            )
            + centered_cylinder(
                format!("closed_pump_valve_station_waste_return_socket_{lane}"),
                12.0 / 2.0,
                ROUTING_BAR_Z + 8.0,
                24,
            )
            .translate(
                ROUTING_BAR_CENTER_X + ROUTING_BAR_X / 2.0 - 38.0,
                y,
                BASE_Z / 2.0 + ROUTING_BAR_Z / 2.0,
            );
    }
    bar - routing + waste_header_lands()
}

fn waste_header_lands() -> Part {
    centered_cube(
        "closed_pump_valve_station_clean_flush_header_land",
        72.0,
        ROUTING_BAR_Y - 44.0,
        8.0,
    )
    .translate(
        ROUTING_BAR_CENTER_X - ROUTING_BAR_X / 2.0 + 38.0,
        0.0,
        BASE_Z / 2.0 + ROUTING_BAR_Z + 4.0,
    ) + centered_cube(
        "closed_pump_valve_station_used_waste_header_land",
        72.0,
        ROUTING_BAR_Y - 44.0,
        8.0,
    )
    .translate(
        ROUTING_BAR_CENTER_X + ROUTING_BAR_X / 2.0 - 38.0,
        0.0,
        BASE_Z / 2.0 + ROUTING_BAR_Z + 4.0,
    )
}

fn barcode_run_record_lands() -> Part {
    let plate = centered_cube(
        "closed_pump_valve_station_barcode_run_record_plate",
        LAND_PLATE_X,
        LAND_PLATE_Y,
        LAND_PLATE_Z,
    )
    .translate(
        -42.0,
        LAND_PLATE_CENTER_Y,
        BASE_Z / 2.0 + LAND_PLATE_Z / 2.0,
    );
    let mut lands = Part::empty("closed_pump_valve_station_barcode_run_record_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = -42.0 + lane_x(i, BARCODE_LAND_COUNT, 58.0);
        lands = lands
            + centered_cube(
                format!("closed_pump_valve_station_barcode_land_{i}"),
                42.0,
                24.0,
                4.0,
            )
            .translate(
                x,
                LAND_PLATE_CENTER_Y - 18.0,
                BASE_Z / 2.0 + LAND_PLATE_Z + 2.0,
            );
    }
    for i in 0..RUN_RECORD_LAND_COUNT {
        let x = -310.0 + i as f64 * 126.0;
        lands = lands
            + centered_cube(
                format!("closed_pump_valve_station_run_record_land_{i}"),
                108.0,
                28.0,
                4.0,
            )
            .translate(
                x,
                LAND_PLATE_CENTER_Y + 20.0,
                BASE_Z / 2.0 + LAND_PLATE_Z + 2.0,
            );
    }
    plate + lands
}

fn clean_used_segregation() -> Part {
    let rib = centered_cube(
        "closed_pump_valve_station_clean_used_segregation_rib",
        SEGREGATION_RIB_X,
        SEGREGATION_RIB_Y,
        SEGREGATION_RIB_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + SEGREGATION_RIB_Z / 2.0);
    let clean_land = centered_cube(
        "closed_pump_valve_station_clean_parts_landing_zone",
        348.0,
        74.0,
        8.0,
    )
    .translate(
        CLEAN_ZONE_CENTER_X,
        -(STATION_Y / 2.0 - 74.0),
        BASE_Z / 2.0 + 4.0,
    );
    let used_land = centered_cube(
        "closed_pump_valve_station_used_parts_quarantine_zone",
        348.0,
        74.0,
        8.0,
    )
    .translate(
        USED_ZONE_CENTER_X,
        -(STATION_Y / 2.0 - 74.0),
        BASE_Z / 2.0 + 4.0,
    );
    let pass_through_gates = centered_cube(
        "closed_pump_valve_station_segregation_tubing_gate_lower",
        SEGREGATION_RIB_X + 8.0,
        62.0,
        30.0,
    )
    .translate(0.0, -174.0, BASE_Z / 2.0 + 34.0)
        + centered_cube(
            "closed_pump_valve_station_segregation_tubing_gate_upper",
            SEGREGATION_RIB_X + 8.0,
            62.0,
            30.0,
        )
        .translate(0.0, 164.0, BASE_Z / 2.0 + 34.0);

    rib - pass_through_gates + clean_land + used_land
}

fn robot_service_keepouts() -> Part {
    let robot_sweep = centered_cube(
        "closed_pump_valve_station_robot_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(0.0, 24.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    let front_service = centered_cube(
        "closed_pump_valve_station_front_balance_service_keepout",
        STATION_X - 150.0,
        FRONT_SERVICE_CLEARANCE,
        72.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        BASE_Z + 36.0,
    );
    let rear_service = centered_cube(
        "closed_pump_valve_station_rear_pump_service_keepout",
        STATION_X - 210.0,
        REAR_SERVICE_CLEARANCE,
        84.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 42.0,
    );
    let sensor_service = centered_cube(
        "closed_pump_valve_station_right_sensor_service_keepout",
        SENSOR_SERVICE_CLEARANCE,
        STATION_Y - 164.0,
        78.0,
    )
    .translate(
        STATION_X / 2.0 + SENSOR_SERVICE_CLEARANCE / 2.0,
        0.0,
        BASE_Z + 39.0,
    );

    robot_sweep - keepout_windows() + front_service + rear_service + sensor_service
}

fn keepout_windows() -> Part {
    let mut windows = Part::empty("closed_pump_valve_station_robot_keepout_windows");
    for i in 0..ROBOT_KEEP_OUT_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_pump_valve_station_robot_keepout_window_{i}"),
                190.0,
                ROBOT_KEEP_OUT_Y + 8.0,
                70.0,
            )
            .translate(
                lane_x(i, ROBOT_KEEP_OUT_WINDOWS, 252.0),
                24.0,
                BASE_Z + ROBOT_KEEP_OUT_Z / 2.0,
            );
    }
    windows
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
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
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_pump_valve_manifold_calibration_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn purchased_component_interfaces_are_explicit() {
        assert_eq!(PUMP_BAY_COUNT, 4);
        assert_eq!(VALVE_LANES, 8);
        assert_eq!(SENSOR_POCKET_COUNT, 8);
        assert_eq!(PUMP_BAY_COUNT * 2, VALVE_LANES);
        assert_eq!(VALVE_LANES, SENSOR_POCKET_COUNT);
        assert!(PUMP_ENVELOPE_X < PUMP_BAY_X);
        assert!(PUMP_ENVELOPE_Y < PUMP_BAY_Y);
    }

    #[test]
    fn calibration_capacity_matches_pre_run_fixture_scope() {
        assert_eq!(RESTRICTION_LOOP_COUNT, 6);
        assert_eq!(COLLECTION_NEST_COUNT, 6);
        assert_eq!(BUBBLE_INLET_COUNT, 2);
        assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
        assert!(LOOP_POST_Z > LOOP_HOLDER_Z);
        assert!(BEAKER_D < BALANCE_PAD_X);
    }

    #[test]
    fn arrays_fit_inside_the_leak_tray_footprint() {
        let pump_edge =
            lane_x(PUMP_BAY_COUNT - 1, PUMP_BAY_COUNT, PUMP_BAY_PITCH_X).abs() + PUMP_BAY_X / 2.0;
        let valve_edge =
            lane_x(VALVE_LANES - 1, VALVE_LANES, VALVE_PITCH_X).abs() + VALVE_LABEL_X / 2.0;
        let loop_edge = lane_x(
            RESTRICTION_LOOP_COUNT - 1,
            RESTRICTION_LOOP_COUNT,
            LOOP_PITCH_X,
        )
        .abs()
            + 44.0;
        let nest_edge = lane_x(
            COLLECTION_NEST_COUNT - 1,
            COLLECTION_NEST_COUNT,
            NEST_PITCH_X,
        )
        .abs()
            + BALANCE_PAD_X / 2.0;
        assert!(pump_edge < STATION_X / 2.0 - RIM_W);
        assert!(valve_edge < STATION_X / 2.0 - RIM_W);
        assert!(loop_edge < STATION_X / 2.0 - RIM_W);
        assert!(nest_edge < STATION_X / 2.0 - RIM_W);
    }

    #[test]
    fn clean_used_zones_have_physical_separation() {
        let clean_right_edge = CLEAN_ZONE_CENTER_X + 348.0 / 2.0;
        let used_left_edge = USED_ZONE_CENTER_X - 348.0 / 2.0;
        assert!(used_left_edge - clean_right_edge >= CLEAN_USED_MIN_GAP);
        assert!(SEGREGATION_RIB_Z > RIM_Z);
        assert!(SEGREGATION_RIB_Y < STATION_Y);
    }

    #[test]
    fn service_and_robot_keepouts_are_visible_and_large_enough() {
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 3);
        assert!(ROBOT_KEEP_OUT_X < STATION_X);
        assert!(ROBOT_KEEP_OUT_Z >= 120.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 240.0);
        assert!(SENSOR_SERVICE_CLEARANCE >= 140.0);
    }

    #[test]
    fn traceability_lands_cover_each_lane_and_run_record() {
        assert_eq!(BARCODE_LAND_COUNT, 12);
        assert_eq!(RUN_RECORD_LAND_COUNT, 4);
        assert!(BARCODE_LAND_COUNT >= VALVE_LANES);
        assert!(LAND_PLATE_CENTER_Y + LAND_PLATE_Y / 2.0 < STATION_Y / 2.0);
    }
}
