use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed pump tubing occlusion/endurance validation station.
//
// Intent:
// - Validate closed pump tubing sets before live perfusion by packaging
//   purchased pump cassettes, tubing loops, calibrated restriction coupons,
//   pressure sensing, bubble/wetness witnessing, gravimetric collection pads,
//   overpressure relief/waste routing, traceability, status/custody lanes,
//   evidence imaging, and robot/service keepout envelopes.
// - Model interface and fixture CAD only. This does not model sterile barriers,
//   validated pump firmware, calibrated sensors, relief-valve internals, or the
//   endurance/occlusion acceptance algorithm.

const OUTPUT_PREFIX: &str = "output/closed_pump_tubing_occlusion_endurance_station";
const OUTPUTS: [&str; 13] = [
    "output/closed_pump_tubing_occlusion_endurance_station_base_leak_tray.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_pump_cassette_nest.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_tubing_loop_strain_relief_combs.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_calibrated_restriction_coupon_slots.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_pressure_sensor_pockets.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_optical_bubble_wetness_witness.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_gravimetric_collection_pad_placeholders.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_overpressure_relief_waste_route.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_cycle_counter_lot_barcode_lands.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_pass_fail_quarantine_lanes.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_evidence_camera_bridge.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_robot_service_keepouts.stl",
    "output/closed_pump_tubing_occlusion_endurance_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 15] = [
    "closed_leak_tray",
    "pump_cassette_nest",
    "tubing_loop_strain_relief_combs",
    "calibrated_restriction_coupon_slots",
    "pressure_sensor_pockets",
    "optical_bubble_wetness_witness",
    "gravimetric_collection_pad_placeholders",
    "overpressure_relief_waste_route",
    "cycle_counter_land",
    "lot_barcode_lands",
    "pass_lane",
    "fail_lane",
    "quarantine_lane",
    "evidence_camera_bridge",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const LEAK_BASIN_X: f64 = STATION_X - 108.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 94.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const DRAIN_D: f64 = 16.0;
const DRAIN_CENTER_X: f64 = STATION_X / 2.0 - 82.0;
const DRAIN_CENTER_Y: f64 = -(STATION_Y / 2.0 - 46.0);
const MOUNT_HOLE_D: f64 = 7.0;

const PUMP_CASSETTES: usize = 4;
const PUMP_NEST_X: f64 = 760.0;
const PUMP_NEST_Y: f64 = 174.0;
const PUMP_NEST_Z: f64 = 46.0;
const PUMP_NEST_CENTER_Y: f64 = 222.0;
const PUMP_CASSETTE_X: f64 = 118.0;
const PUMP_CASSETTE_Y: f64 = 132.0;
const PUMP_CASSETTE_Z: f64 = 30.0;
const PUMP_CASSETTE_PITCH_X: f64 = 174.0;
const CASSETTE_DATUM_PIN_D: f64 = 10.0;

const TUBING_LANES: usize = 8;
const LOOP_COMB_X: f64 = 930.0;
const LOOP_COMB_Y: f64 = 126.0;
const LOOP_COMB_Z: f64 = 26.0;
const LOOP_COMB_CENTER_Y: f64 = 82.0;
const LOOP_LANE_PITCH_X: f64 = 108.0;
const COMB_TOOTH_X: f64 = 14.0;
const COMB_TOOTH_Y: f64 = 42.0;
const COMB_TOOTH_Z: f64 = 40.0;
const TUBE_OD_MAX: f64 = 6.4;
const TUBE_CLEARANCE: f64 = 1.4;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;

const RESTRICTION_COUPONS: usize = 8;
const RESTRICTION_TRAY_X: f64 = 498.0;
const RESTRICTION_TRAY_Y: f64 = 166.0;
const RESTRICTION_TRAY_Z: f64 = 28.0;
const RESTRICTION_CENTER_X: f64 = -322.0;
const RESTRICTION_CENTER_Y: f64 = -70.0;
const COUPON_SLOT_X: f64 = 46.0;
const COUPON_SLOT_Y: f64 = 104.0;
const COUPON_SLOT_Z: f64 = 24.0;
const COUPON_PITCH_X: f64 = 58.0;
const RESTRICTION_ID_LANDS: usize = RESTRICTION_COUPONS;

const PRESSURE_SENSORS: usize = 8;
const SENSOR_BAR_X: f64 = 520.0;
const SENSOR_BAR_Y: f64 = 114.0;
const SENSOR_BAR_Z: f64 = 42.0;
const SENSOR_CENTER_X: f64 = 216.0;
const SENSOR_CENTER_Y: f64 = -72.0;
const SENSOR_PITCH_X: f64 = 62.0;
const PRESSURE_SENSOR_POCKET_X: f64 = 38.0;
const PRESSURE_SENSOR_POCKET_Y: f64 = 42.0;
const PRESSURE_SENSOR_POCKET_Z: f64 = 28.0;
const PRESSURE_TAP_D: f64 = 8.0;

const WITNESS_X: f64 = 266.0;
const WITNESS_Y: f64 = 150.0;
const WITNESS_Z: f64 = 86.0;
const WITNESS_CENTER_X: f64 = -455.0;
const WITNESS_CENTER_Y: f64 = -260.0;
const WITNESS_CHANNELS: usize = 4;
const WITNESS_CHANNEL_PITCH_X: f64 = 52.0;
const WETNESS_PAD_X: f64 = 44.0;
const WETNESS_PAD_Y: f64 = 34.0;

const COLLECTION_PADS: usize = 6;
const COLLECTION_RACK_X: f64 = 548.0;
const COLLECTION_RACK_Y: f64 = 150.0;
const COLLECTION_RACK_Z: f64 = 24.0;
const COLLECTION_CENTER_X: f64 = 22.0;
const COLLECTION_CENTER_Y: f64 = -272.0;
const COLLECTION_PITCH_X: f64 = 82.0;
const SCALE_PAD_X: f64 = 66.0;
const SCALE_PAD_Y: f64 = 86.0;
const COLLECTION_VIAL_D: f64 = 42.0;

const WASTE_ROUTE_X: f64 = 132.0;
const WASTE_ROUTE_Y: f64 = 612.0;
const WASTE_ROUTE_Z: f64 = 44.0;
const WASTE_ROUTE_CENTER_X: f64 = 522.0;
const WASTE_ROUTE_CENTER_Y: f64 = -18.0;
const WASTE_CHANNELS: usize = TUBING_LANES;
const WASTE_PORT_D: f64 = 10.0;
const RELIEF_TOWER_D: f64 = 52.0;
const RELIEF_TOWER_Z: f64 = 104.0;
const RELIEF_SETPOINT_LANDS: usize = 3;

const TRACEABILITY_PANEL_X: f64 = 358.0;
const TRACEABILITY_PANEL_Y: f64 = 92.0;
const TRACEABILITY_PANEL_Z: f64 = 14.0;
const TRACEABILITY_CENTER_X: f64 = -416.0;
const TRACEABILITY_CENTER_Y: f64 = 336.0;
const BARCODE_LANDS: usize = 10;
const CYCLE_COUNTER_WINDOWS: usize = 2;

const STATUS_LANES: usize = 3;
const STATUS_PANEL_X: f64 = 382.0;
const STATUS_PANEL_Y: f64 = 186.0;
const STATUS_PANEL_Z: f64 = 30.0;
const STATUS_CENTER_X: f64 = 382.0;
const STATUS_CENTER_Y: f64 = 256.0;
const STATUS_LANE_Y: f64 = 46.0;
const STATUS_LANE_PITCH_Y: f64 = 62.0;
const STATUS_CUSTODY_GAP: f64 = 74.0;
const MIN_CUSTODY_GAP: f64 = 60.0;
const QUARANTINE_WALL_Z: f64 = 78.0;

const CAMERA_BRIDGE_X: f64 = 880.0;
const CAMERA_BRIDGE_Y: f64 = 70.0;
const CAMERA_BRIDGE_Z: f64 = 190.0;
const CAMERA_BRIDGE_CENTER_X: f64 = -12.0;
const CAMERA_BRIDGE_CENTER_Y: f64 = 348.0;
const CAMERA_BLOCK_X: f64 = 74.0;
const CAMERA_BLOCK_Y: f64 = 50.0;
const CAMERA_BLOCK_Z: f64 = 44.0;
const RING_LIGHT_D: f64 = 68.0;
const EVIDENCE_TARGETS: usize = 5;

const ROBOT_KEEP_OUT_X: f64 = 1040.0;
const ROBOT_KEEP_OUT_Y: f64 = 212.0;
const ROBOT_KEEP_OUT_Z: f64 = 176.0;
const ROBOT_KEEP_OUT_CENTER_Y: f64 = 56.0;
const SERVICE_KEEP_OUT_FRONT_Y: f64 = 248.0;
const SERVICE_KEEP_OUT_REAR_Y: f64 = 178.0;
const SERVICE_KEEP_OUT_Z: f64 = 118.0;
const WASTE_SERVICE_KEEP_OUT_X: f64 = 190.0;
const SENSOR_SERVICE_KEEP_OUT_X: f64 = 560.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    write_part(base_leak_tray(), OUTPUTS[0]);
    write_part(pump_cassette_nest(), OUTPUTS[1]);
    write_part(tubing_loop_strain_relief_combs(), OUTPUTS[2]);
    write_part(calibrated_restriction_coupon_slots(), OUTPUTS[3]);
    write_part(pressure_sensor_pockets(), OUTPUTS[4]);
    write_part(optical_bubble_wetness_witness(), OUTPUTS[5]);
    write_part(gravimetric_collection_pad_placeholders(), OUTPUTS[6]);
    write_part(overpressure_relief_waste_route(), OUTPUTS[7]);
    write_part(cycle_counter_lot_barcode_lands(), OUTPUTS[8]);
    write_part(pass_fail_quarantine_lanes(), OUTPUTS[9]);
    write_part(evidence_camera_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed pump tubing occlusion/endurance station: {:.0}mm x {:.0}mm leak-tray deck, {} pump cassette nests, {} tubing lanes, {} restriction coupons, {} pressure pockets, {} witness channels, and {} gravimetric collection pads.",
        STATION_X,
        STATION_Y,
        PUMP_CASSETTES,
        TUBING_LANES,
        RESTRICTION_COUPONS,
        PRESSURE_SENSORS,
        WITNESS_CHANNELS,
        COLLECTION_PADS
    );
    println!(
        "Traceability and custody: {} cycle-counter windows, {} lot/barcode lands, {} restriction ID lands, pass/fail/quarantine separation gap {:.0}mm (minimum {:.0}mm), {} relief setpoint lands, {} waste channels, {} evidence targets, {} required feature groups, and output prefix {OUTPUT_PREFIX}.",
        CYCLE_COUNTER_WINDOWS,
        BARCODE_LANDS,
        RESTRICTION_ID_LANDS,
        STATUS_CUSTODY_GAP,
        MIN_CUSTODY_GAP,
        RELIEF_SETPOINT_LANDS,
        WASTE_CHANNELS,
        EVIDENCE_TARGETS,
        REQUIRED_FEATURES.len()
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray()
        + pump_cassette_nest()
        + tubing_loop_strain_relief_combs()
        + calibrated_restriction_coupon_slots()
        + pressure_sensor_pockets()
        + optical_bubble_wetness_witness()
        + gravimetric_collection_pad_placeholders()
        + overpressure_relief_waste_route()
        + cycle_counter_lot_barcode_lands()
        + pass_fail_quarantine_lanes()
        + evidence_camera_bridge()
        + robot_service_keepouts()
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_pump_tubing_station_deck_plate",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        "closed_pump_tubing_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.5);
    let drain = centered_cylinder(
        "closed_pump_tubing_station_leak_tray_drain_bore",
        DRAIN_D / 2.0,
        DECK_Z + 20.0,
        32,
    )
    .translate(DRAIN_CENTER_X, DRAIN_CENTER_Y, 0.0);

    deck - basin - drain + tray_rims() + mounting_bosses() + datum_fiducials()
}

fn tray_rims() -> Part {
    let z = DECK_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        "closed_pump_tubing_station_left_leak_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        "closed_pump_tubing_station_right_leak_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        "closed_pump_tubing_station_rear_leak_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        "closed_pump_tubing_station_front_low_leak_rim",
        STATION_X,
        RIM_W,
        RIM_Z * 0.66,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.33,
    );

    left + right + rear + front
}

fn mounting_bosses() -> Part {
    let mut bosses = Part::empty("closed_pump_tubing_station_mounting_bosses");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_pump_tubing_station_mounting_boss_{i}"),
            18.0,
            8.0,
            32,
        )
        .translate(x, y, DECK_Z / 2.0 + 4.0);
        let hole = centered_cylinder(
            format!("closed_pump_tubing_station_mounting_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            16.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_pump_tubing_station_datum_fiducials");
    for i in 0..4 {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_pump_tubing_station_floor_fiducial_{i}"),
                9.0,
                2.5,
                32,
            )
            .translate(
                lane_x(i, 4, 324.0),
                STATION_Y / 2.0 - 90.0,
                DECK_Z / 2.0 + 1.25,
            );
    }
    fiducials
}

fn pump_cassette_nest() -> Part {
    let plate = centered_cube(
        "closed_pump_tubing_station_pump_cassette_nest_plate",
        PUMP_NEST_X,
        PUMP_NEST_Y,
        PUMP_NEST_Z,
    )
    .translate(0.0, PUMP_NEST_CENTER_Y, DECK_Z / 2.0 + PUMP_NEST_Z / 2.0);
    let mut cutouts = Part::empty("closed_pump_tubing_station_pump_cassette_recesses");
    let mut features = Part::empty("closed_pump_tubing_station_pump_cassette_features");

    for i in 0..PUMP_CASSETTES {
        let x = lane_x(i, PUMP_CASSETTES, PUMP_CASSETTE_PITCH_X);
        cutouts = cutouts
            + centered_cube(
                format!("closed_pump_tubing_station_pump_cassette_recess_{i}"),
                PUMP_CASSETTE_X,
                PUMP_CASSETTE_Y,
                PUMP_CASSETTE_Z,
            )
            .translate(
                x,
                PUMP_NEST_CENTER_Y,
                DECK_Z / 2.0 + PUMP_NEST_Z / 2.0 + 8.0,
            );

        for (pin_i, dx) in [-46.0, 46.0].iter().copied().enumerate() {
            features = features
                + centered_cylinder(
                    format!("closed_pump_tubing_station_cassette_datum_pin_{i}_{pin_i}"),
                    CASSETTE_DATUM_PIN_D / 2.0,
                    22.0,
                    24,
                )
                .translate(
                    x + dx,
                    PUMP_NEST_CENTER_Y - PUMP_CASSETTE_Y / 2.0 - 18.0,
                    DECK_Z / 2.0 + PUMP_NEST_Z + 11.0,
                );
        }

        for (port_i, dy) in [-48.0, 48.0].iter().copied().enumerate() {
            cutouts = cutouts
                + centered_cylinder(
                    format!("closed_pump_tubing_station_cassette_tube_channel_{i}_{port_i}"),
                    TUBE_CHANNEL_D / 2.0,
                    PUMP_CASSETTE_Y + 52.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x + if port_i == 0 { -31.0 } else { 31.0 },
                    PUMP_NEST_CENTER_Y + dy * 0.12,
                    DECK_Z / 2.0 + PUMP_NEST_Z + 2.0,
                );
        }

        features = features
            + centered_cube(
                format!("closed_pump_tubing_station_cassette_latch_land_{i}"),
                PUMP_CASSETTE_X + 20.0,
                12.0,
                18.0,
            )
            .translate(
                x,
                PUMP_NEST_CENTER_Y + PUMP_CASSETTE_Y / 2.0 + 13.0,
                DECK_Z / 2.0 + PUMP_NEST_Z + 9.0,
            );
    }

    plate - cutouts + features
}

fn tubing_loop_strain_relief_combs() -> Part {
    let backbone = centered_cube(
        "closed_pump_tubing_station_loop_comb_backbone",
        LOOP_COMB_X,
        LOOP_COMB_Y,
        LOOP_COMB_Z,
    )
    .translate(0.0, LOOP_COMB_CENTER_Y, DECK_Z / 2.0 + LOOP_COMB_Z / 2.0);
    let mut teeth = Part::empty("closed_pump_tubing_station_loop_comb_teeth");
    let mut troughs = Part::empty("closed_pump_tubing_station_loop_comb_tube_troughs");

    for i in 0..TUBING_LANES {
        let x = lane_x(i, TUBING_LANES, LOOP_LANE_PITCH_X);
        for (side_i, y) in [
            LOOP_COMB_CENTER_Y - LOOP_COMB_Y / 2.0 + 22.0,
            LOOP_COMB_CENTER_Y + LOOP_COMB_Y / 2.0 - 22.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            teeth = teeth
                + centered_cube(
                    format!("closed_pump_tubing_station_strain_relief_comb_tooth_{i}_{side_i}"),
                    COMB_TOOTH_X,
                    COMB_TOOTH_Y,
                    COMB_TOOTH_Z,
                )
                .translate(x, y, DECK_Z / 2.0 + LOOP_COMB_Z + COMB_TOOTH_Z / 2.0);
        }

        troughs = troughs
            + centered_cylinder(
                format!("closed_pump_tubing_station_loop_tubing_trough_{i}"),
                TUBE_CHANNEL_D / 2.0,
                LOOP_COMB_Y + 22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, LOOP_COMB_CENTER_Y, DECK_Z / 2.0 + LOOP_COMB_Z + 4.0);
    }

    backbone - troughs + teeth + loop_end_posts()
}

fn loop_end_posts() -> Part {
    let mut posts = Part::empty("closed_pump_tubing_station_loop_end_posts");
    for i in 0..TUBING_LANES {
        let x = lane_x(i, TUBING_LANES, LOOP_LANE_PITCH_X);
        for (post_i, y) in [
            LOOP_COMB_CENTER_Y - LOOP_COMB_Y / 2.0 - 20.0,
            LOOP_COMB_CENTER_Y + LOOP_COMB_Y / 2.0 + 20.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            posts = posts
                + centered_cylinder(
                    format!("closed_pump_tubing_station_loop_end_post_{i}_{post_i}"),
                    8.0,
                    54.0,
                    24,
                )
                .translate(x, y, DECK_Z / 2.0 + 27.0);
        }
    }
    posts
}

fn calibrated_restriction_coupon_slots() -> Part {
    let tray = centered_cube(
        "closed_pump_tubing_station_restriction_coupon_tray",
        RESTRICTION_TRAY_X,
        RESTRICTION_TRAY_Y,
        RESTRICTION_TRAY_Z,
    )
    .translate(
        RESTRICTION_CENTER_X,
        RESTRICTION_CENTER_Y,
        DECK_Z / 2.0 + RESTRICTION_TRAY_Z / 2.0,
    );
    let mut slots = Part::empty("closed_pump_tubing_station_restriction_coupon_slots");
    let mut lands = Part::empty("closed_pump_tubing_station_restriction_coupon_id_lands");

    for i in 0..RESTRICTION_COUPONS {
        let x = RESTRICTION_CENTER_X + lane_x(i, RESTRICTION_COUPONS, COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_pump_tubing_station_calibrated_restriction_slot_{i}"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_SLOT_Z,
            )
            .translate(
                x,
                RESTRICTION_CENTER_Y + 12.0,
                DECK_Z / 2.0 + RESTRICTION_TRAY_Z / 2.0 + 6.0,
            );
        lands = lands
            + centered_cube(
                format!("closed_pump_tubing_station_restriction_coupon_laser_id_land_{i}"),
                COUPON_SLOT_X,
                18.0,
                3.0,
            )
            .translate(
                x,
                RESTRICTION_CENTER_Y - RESTRICTION_TRAY_Y / 2.0 + 18.0,
                DECK_Z / 2.0 + RESTRICTION_TRAY_Z + 1.5,
            );
    }

    tray - slots + lands
}

fn pressure_sensor_pockets() -> Part {
    let bar = centered_cube(
        "closed_pump_tubing_station_pressure_sensor_bar",
        SENSOR_BAR_X,
        SENSOR_BAR_Y,
        SENSOR_BAR_Z,
    )
    .translate(
        SENSOR_CENTER_X,
        SENSOR_CENTER_Y,
        DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0,
    );
    let mut pockets = Part::empty("closed_pump_tubing_station_pressure_sensor_recesses");
    let mut fittings = Part::empty("closed_pump_tubing_station_pressure_sensor_fittings");

    for i in 0..PRESSURE_SENSORS {
        let x = SENSOR_CENTER_X + lane_x(i, PRESSURE_SENSORS, SENSOR_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!("closed_pump_tubing_station_pressure_sensor_pocket_{i}"),
                PRESSURE_SENSOR_POCKET_X,
                PRESSURE_SENSOR_POCKET_Y,
                PRESSURE_SENSOR_POCKET_Z,
            )
            .translate(x, SENSOR_CENTER_Y, DECK_Z / 2.0 + SENSOR_BAR_Z / 2.0 + 6.0);
        fittings = fittings
            + centered_cylinder(
                format!("closed_pump_tubing_station_pressure_tap_boss_{i}"),
                PRESSURE_TAP_D,
                18.0,
                24,
            )
            .translate(
                x,
                SENSOR_CENTER_Y - SENSOR_BAR_Y / 2.0 - 12.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z + 9.0,
            )
            + centered_cube(
                format!("closed_pump_tubing_station_pressure_sensor_cable_slot_{i}"),
                24.0,
                8.0,
                12.0,
            )
            .translate(
                x,
                SENSOR_CENTER_Y + SENSOR_BAR_Y / 2.0 + 6.0,
                DECK_Z / 2.0 + SENSOR_BAR_Z + 6.0,
            );
    }

    bar - pockets + fittings
}

fn optical_bubble_wetness_witness() -> Part {
    let base = centered_cube(
        "closed_pump_tubing_station_optical_witness_base",
        WITNESS_X,
        WITNESS_Y,
        26.0,
    )
    .translate(WITNESS_CENTER_X, WITNESS_CENTER_Y, DECK_Z / 2.0 + 13.0);
    let rear_wall = centered_cube(
        "closed_pump_tubing_station_optical_witness_rear_wall",
        WITNESS_X,
        14.0,
        WITNESS_Z,
    )
    .translate(
        WITNESS_CENTER_X,
        WITNESS_CENTER_Y + WITNESS_Y / 2.0 - 7.0,
        DECK_Z / 2.0 + WITNESS_Z / 2.0,
    );
    let front_gate = centered_cube(
        "closed_pump_tubing_station_optical_witness_front_gate",
        WITNESS_X,
        10.0,
        WITNESS_Z * 0.72,
    )
    .translate(
        WITNESS_CENTER_X,
        WITNESS_CENTER_Y - WITNESS_Y / 2.0 + 5.0,
        DECK_Z / 2.0 + WITNESS_Z * 0.36,
    );
    let mut windows = Part::empty("closed_pump_tubing_station_optical_bubble_windows");
    let mut pads = Part::empty("closed_pump_tubing_station_wetness_witness_pad_lands");

    for i in 0..WITNESS_CHANNELS {
        let x = WITNESS_CENTER_X + lane_x(i, WITNESS_CHANNELS, WITNESS_CHANNEL_PITCH_X);
        windows = windows
            + centered_cube(
                format!("closed_pump_tubing_station_bubble_sight_window_{i}"),
                30.0,
                18.0,
                44.0,
            )
            .translate(
                x,
                WITNESS_CENTER_Y + WITNESS_Y / 2.0 - 6.0,
                DECK_Z / 2.0 + 54.0,
            );
        pads = pads
            + centered_cube(
                format!("closed_pump_tubing_station_wetness_indicator_pad_{i}"),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                4.0,
            )
            .translate(
                x,
                WITNESS_CENTER_Y - WITNESS_Y / 2.0 + 30.0,
                DECK_Z / 2.0 + 28.0,
            );
    }

    base + (rear_wall - windows) + front_gate + pads + witness_led_detector_bosses()
}

fn witness_led_detector_bosses() -> Part {
    let mut bosses = Part::empty("closed_pump_tubing_station_witness_led_detector_bosses");
    for i in 0..WITNESS_CHANNELS {
        let x = WITNESS_CENTER_X + lane_x(i, WITNESS_CHANNELS, WITNESS_CHANNEL_PITCH_X);
        for (side_i, y) in [
            WITNESS_CENTER_Y - WITNESS_Y / 2.0 + 54.0,
            WITNESS_CENTER_Y + WITNESS_Y / 2.0 - 34.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            bosses = bosses
                + centered_cylinder(
                    format!("closed_pump_tubing_station_witness_optic_boss_{i}_{side_i}"),
                    10.0,
                    14.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, DECK_Z / 2.0 + 52.0);
        }
    }
    bosses
}

fn gravimetric_collection_pad_placeholders() -> Part {
    let rack = centered_cube(
        "closed_pump_tubing_station_gravimetric_collection_rack",
        COLLECTION_RACK_X,
        COLLECTION_RACK_Y,
        COLLECTION_RACK_Z,
    )
    .translate(
        COLLECTION_CENTER_X,
        COLLECTION_CENTER_Y,
        DECK_Z / 2.0 + COLLECTION_RACK_Z / 2.0,
    );
    let mut pads = Part::empty("closed_pump_tubing_station_collection_balance_pads");

    for i in 0..COLLECTION_PADS {
        let x = COLLECTION_CENTER_X + lane_x(i, COLLECTION_PADS, COLLECTION_PITCH_X);
        let pad = centered_cube(
            format!("closed_pump_tubing_station_gravimetric_scale_pad_{i}"),
            SCALE_PAD_X,
            SCALE_PAD_Y,
            5.0,
        )
        .translate(
            x,
            COLLECTION_CENTER_Y,
            DECK_Z / 2.0 + COLLECTION_RACK_Z + 2.5,
        );
        let vial_ring = centered_cylinder(
            format!("closed_pump_tubing_station_collection_vial_outer_ring_{i}"),
            COLLECTION_VIAL_D / 2.0 + 4.0,
            10.0,
            32,
        )
        .translate(
            x,
            COLLECTION_CENTER_Y,
            DECK_Z / 2.0 + COLLECTION_RACK_Z + 10.0,
        ) - centered_cylinder(
            format!("closed_pump_tubing_station_collection_vial_inner_clearance_{i}"),
            COLLECTION_VIAL_D / 2.0,
            12.0,
            32,
        )
        .translate(
            x,
            COLLECTION_CENTER_Y,
            DECK_Z / 2.0 + COLLECTION_RACK_Z + 10.0,
        );
        pads = pads + pad + vial_ring;
    }

    rack + pads
}

fn overpressure_relief_waste_route() -> Part {
    let spine = centered_cube(
        "closed_pump_tubing_station_overpressure_waste_spine",
        WASTE_ROUTE_X,
        WASTE_ROUTE_Y,
        WASTE_ROUTE_Z,
    )
    .translate(
        WASTE_ROUTE_CENTER_X,
        WASTE_ROUTE_CENTER_Y,
        DECK_Z / 2.0 + WASTE_ROUTE_Z / 2.0,
    );
    let mut channels = Part::empty("closed_pump_tubing_station_waste_route_channels");
    let mut ports = Part::empty("closed_pump_tubing_station_waste_route_ports");

    for i in 0..WASTE_CHANNELS {
        let y = WASTE_ROUTE_CENTER_Y + lane_x(i, WASTE_CHANNELS, 62.0);
        channels = channels
            + centered_cylinder(
                format!("closed_pump_tubing_station_waste_route_channel_{i}"),
                TUBE_CHANNEL_D / 2.0,
                WASTE_ROUTE_X + 30.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(WASTE_ROUTE_CENTER_X, y, DECK_Z / 2.0 + WASTE_ROUTE_Z + 2.0);
        ports = ports
            + centered_cylinder(
                format!("closed_pump_tubing_station_waste_bulkhead_port_{i}"),
                WASTE_PORT_D / 2.0,
                18.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                WASTE_ROUTE_CENTER_X + WASTE_ROUTE_X / 2.0 + 9.0,
                y,
                DECK_Z / 2.0 + WASTE_ROUTE_Z / 2.0,
            );
    }

    spine - channels + ports + relief_valve_tower() + relief_setpoint_lands()
}

fn relief_valve_tower() -> Part {
    let tower = centered_cylinder(
        "closed_pump_tubing_station_overpressure_relief_tower",
        RELIEF_TOWER_D / 2.0,
        RELIEF_TOWER_Z,
        40,
    )
    .translate(
        WASTE_ROUTE_CENTER_X,
        WASTE_ROUTE_CENTER_Y + WASTE_ROUTE_Y / 2.0 - 58.0,
        DECK_Z / 2.0 + WASTE_ROUTE_Z + RELIEF_TOWER_Z / 2.0,
    );
    let vent = centered_cylinder(
        "closed_pump_tubing_station_relief_vent_bore",
        12.0,
        RELIEF_TOWER_Z + 12.0,
        32,
    )
    .translate(
        WASTE_ROUTE_CENTER_X,
        WASTE_ROUTE_CENTER_Y + WASTE_ROUTE_Y / 2.0 - 58.0,
        DECK_Z / 2.0 + WASTE_ROUTE_Z + RELIEF_TOWER_Z / 2.0,
    );
    tower - vent
}

fn relief_setpoint_lands() -> Part {
    let mut lands = Part::empty("closed_pump_tubing_station_relief_setpoint_lands");
    for i in 0..RELIEF_SETPOINT_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_pump_tubing_station_relief_setpoint_label_land_{i}"),
                78.0,
                24.0,
                3.0,
            )
            .translate(
                WASTE_ROUTE_CENTER_X,
                WASTE_ROUTE_CENTER_Y + WASTE_ROUTE_Y / 2.0 - 128.0 - i as f64 * 32.0,
                DECK_Z / 2.0 + WASTE_ROUTE_Z + 1.5,
            );
    }
    lands
}

fn cycle_counter_lot_barcode_lands() -> Part {
    let panel = centered_cube(
        "closed_pump_tubing_station_traceability_panel",
        TRACEABILITY_PANEL_X,
        TRACEABILITY_PANEL_Y,
        TRACEABILITY_PANEL_Z,
    )
    .translate(
        TRACEABILITY_CENTER_X,
        TRACEABILITY_CENTER_Y,
        DECK_Z / 2.0 + TRACEABILITY_PANEL_Z / 2.0,
    );
    let mut lands = Part::empty("closed_pump_tubing_station_lot_barcode_lands");

    for i in 0..BARCODE_LANDS {
        let row = i / 5;
        let col = i % 5;
        lands = lands
            + centered_cube(
                format!("closed_pump_tubing_station_lot_barcode_land_{i}"),
                54.0,
                20.0,
                3.0,
            )
            .translate(
                TRACEABILITY_CENTER_X - 116.0 + col as f64 * 58.0,
                TRACEABILITY_CENTER_Y - 20.0 + row as f64 * 36.0,
                DECK_Z / 2.0 + TRACEABILITY_PANEL_Z + 1.5,
            );
    }

    for i in 0..CYCLE_COUNTER_WINDOWS {
        lands = lands
            + centered_cube(
                format!("closed_pump_tubing_station_cycle_counter_window_{i}"),
                92.0,
                26.0,
                4.0,
            )
            .translate(
                TRACEABILITY_CENTER_X + 92.0,
                TRACEABILITY_CENTER_Y - 18.0 + i as f64 * 36.0,
                DECK_Z / 2.0 + TRACEABILITY_PANEL_Z + 2.0,
            );
    }

    panel + lands
}

fn pass_fail_quarantine_lanes() -> Part {
    let panel = centered_cube(
        "closed_pump_tubing_station_status_custody_panel",
        STATUS_PANEL_X,
        STATUS_PANEL_Y,
        STATUS_PANEL_Z,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y,
        DECK_Z / 2.0 + STATUS_PANEL_Z / 2.0,
    );
    let mut lanes = Part::empty("closed_pump_tubing_station_pass_fail_quarantine_lanes");

    for (i, lane) in ["pass", "fail", "quarantine"].iter().enumerate() {
        let y = STATUS_CENTER_Y + lane_x(i, STATUS_LANES, STATUS_LANE_PITCH_Y);
        lanes = lanes
            + centered_cube(
                format!("closed_pump_tubing_station_{lane}_custody_lane"),
                STATUS_PANEL_X - 44.0,
                STATUS_LANE_Y,
                6.0,
            )
            .translate(
                x_status_lane_center(),
                y,
                DECK_Z / 2.0 + STATUS_PANEL_Z + 3.0,
            );
    }

    panel + lanes + custody_separation_walls()
}

fn custody_separation_walls() -> Part {
    let pass_fail_wall = centered_cube(
        "closed_pump_tubing_station_pass_fail_custody_wall",
        STATUS_PANEL_X - 28.0,
        12.0,
        48.0,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y - STATUS_LANE_PITCH_Y / 2.0,
        DECK_Z / 2.0 + STATUS_PANEL_Z + 24.0,
    );
    let quarantine_wall = centered_cube(
        "closed_pump_tubing_station_quarantine_custody_wall",
        STATUS_PANEL_X - 28.0,
        16.0,
        QUARANTINE_WALL_Z,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y + STATUS_LANE_PITCH_Y / 2.0,
        DECK_Z / 2.0 + STATUS_PANEL_Z + QUARANTINE_WALL_Z / 2.0,
    );
    let quarantine_gate = centered_cube(
        "closed_pump_tubing_station_quarantine_sealed_gate_land",
        82.0,
        20.0,
        24.0,
    )
    .translate(
        STATUS_CENTER_X + STATUS_PANEL_X / 2.0 - 68.0,
        STATUS_CENTER_Y + STATUS_LANE_PITCH_Y,
        DECK_Z / 2.0 + STATUS_PANEL_Z + 12.0,
    );
    pass_fail_wall + quarantine_wall + quarantine_gate
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_pump_tubing_station_evidence_bridge_left_post",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_CENTER_X - CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "closed_pump_tubing_station_evidence_bridge_right_post",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_CENTER_X + CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let crossbar = centered_cube(
        "closed_pump_tubing_station_evidence_bridge_crossbar",
        CAMERA_BRIDGE_X,
        32.0,
        32.0,
    )
    .translate(
        CAMERA_BRIDGE_CENTER_X,
        CAMERA_BRIDGE_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 16.0,
    );
    let camera_block = centered_cube(
        "closed_pump_tubing_station_evidence_camera_block",
        CAMERA_BLOCK_X,
        CAMERA_BLOCK_Y,
        CAMERA_BLOCK_Z,
    )
    .translate(
        CAMERA_BRIDGE_CENTER_X,
        CAMERA_BRIDGE_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 54.0,
    );
    let ring_light = centered_cylinder(
        "closed_pump_tubing_station_evidence_ring_light_outer",
        RING_LIGHT_D / 2.0,
        8.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        CAMERA_BRIDGE_CENTER_X,
        CAMERA_BRIDGE_CENTER_Y - 30.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 82.0,
    ) - centered_cylinder(
        "closed_pump_tubing_station_evidence_ring_light_inner",
        RING_LIGHT_D / 2.0 - 12.0,
        10.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        CAMERA_BRIDGE_CENTER_X,
        CAMERA_BRIDGE_CENTER_Y - 30.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 82.0,
    );

    left_post + right_post + crossbar + camera_block + ring_light + evidence_targets()
}

fn evidence_targets() -> Part {
    let mut targets = Part::empty("closed_pump_tubing_station_evidence_targets");
    for i in 0..EVIDENCE_TARGETS {
        targets = targets
            + centered_cube(
                format!("closed_pump_tubing_station_evidence_scale_target_{i}"),
                42.0,
                24.0,
                3.0,
            )
            .translate(
                CAMERA_BRIDGE_CENTER_X + lane_x(i, EVIDENCE_TARGETS, 122.0),
                CAMERA_BRIDGE_CENTER_Y - 34.0,
                DECK_Z / 2.0 + 3.0,
            );
    }
    targets
}

fn robot_service_keepouts() -> Part {
    let robot_corridor = centered_cube(
        "closed_pump_tubing_station_robot_pick_sweep_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        ROBOT_KEEP_OUT_CENTER_Y,
        DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0,
    );
    let front_service = centered_cube(
        "closed_pump_tubing_station_front_service_keepout",
        STATION_X - 140.0,
        SERVICE_KEEP_OUT_FRONT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + SERVICE_KEEP_OUT_FRONT_Y / 2.0),
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_pump_tubing_station_rear_service_keepout",
        STATION_X - 180.0,
        SERVICE_KEEP_OUT_REAR_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + SERVICE_KEEP_OUT_REAR_Y / 2.0,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let waste_service = centered_cube(
        "closed_pump_tubing_station_waste_bag_service_keepout",
        WASTE_SERVICE_KEEP_OUT_X,
        WASTE_ROUTE_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        WASTE_ROUTE_CENTER_X,
        WASTE_ROUTE_CENTER_Y,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );
    let sensor_service = centered_cube(
        "closed_pump_tubing_station_pressure_sensor_service_keepout",
        SENSOR_SERVICE_KEEP_OUT_X,
        116.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        SENSOR_CENTER_X,
        SENSOR_CENTER_Y,
        DECK_Z / 2.0 + SERVICE_KEEP_OUT_Z / 2.0,
    );

    robot_corridor + front_service + rear_service + waste_service + sensor_service
}

fn x_status_lane_center() -> f64 {
    STATUS_CENTER_X
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
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn includes_required_occlusion_endurance_features() {
        for feature in [
            "pump_cassette_nest",
            "tubing_loop_strain_relief_combs",
            "calibrated_restriction_coupon_slots",
            "pressure_sensor_pockets",
            "optical_bubble_wetness_witness",
            "gravimetric_collection_pad_placeholders",
            "overpressure_relief_waste_route",
            "cycle_counter_land",
            "lot_barcode_lands",
            "pass_lane",
            "fail_lane",
            "quarantine_lane",
            "evidence_camera_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 15);
    }

    #[test]
    fn channel_counts_match_closed_tubing_validation_scope() {
        assert_eq!(PUMP_CASSETTES * 2, TUBING_LANES);
        assert_eq!(PRESSURE_SENSORS, TUBING_LANES);
        assert_eq!(WASTE_CHANNELS, TUBING_LANES);
        assert_eq!(RESTRICTION_ID_LANDS, RESTRICTION_COUPONS);
        assert!(TUBE_CHANNEL_D > TUBE_OD_MAX);
        assert!(PRESSURE_TAP_D > TUBE_CHANNEL_D / 2.0);
    }

    #[test]
    fn keepout_geometry_protects_robot_and_service_access() {
        assert!(ROBOT_KEEP_OUT_X > LOOP_COMB_X);
        assert!(ROBOT_KEEP_OUT_Y > PUMP_NEST_Y);
        assert!(ROBOT_KEEP_OUT_Z > CAMERA_BLOCK_Z + WASTE_ROUTE_Z);
        assert!(SERVICE_KEEP_OUT_FRONT_Y > COLLECTION_RACK_Y);
        assert!(SERVICE_KEEP_OUT_REAR_Y > TRACEABILITY_PANEL_Y);
        assert!(WASTE_SERVICE_KEEP_OUT_X > WASTE_ROUTE_X);
        assert!(SENSOR_SERVICE_KEEP_OUT_X > SENSOR_BAR_X);
    }

    #[test]
    fn pass_fail_quarantine_lanes_have_status_and_custody_separation() {
        assert_eq!(STATUS_LANES, 3);
        assert!(STATUS_CUSTODY_GAP >= MIN_CUSTODY_GAP);
        assert!(QUARANTINE_WALL_Z > STATUS_PANEL_Z * 2.0);
        assert!(STATUS_LANE_PITCH_Y > STATUS_LANE_Y);
        assert!(BARCODE_LANDS >= PUMP_CASSETTES + COLLECTION_PADS);
        assert!(CYCLE_COUNTER_WINDOWS >= 2);
    }

    #[test]
    fn major_modules_fit_inside_station_deck() {
        for (x, y, sx, sy) in [
            (0.0, PUMP_NEST_CENTER_Y, PUMP_NEST_X, PUMP_NEST_Y),
            (0.0, LOOP_COMB_CENTER_Y, LOOP_COMB_X, LOOP_COMB_Y),
            (
                RESTRICTION_CENTER_X,
                RESTRICTION_CENTER_Y,
                RESTRICTION_TRAY_X,
                RESTRICTION_TRAY_Y,
            ),
            (SENSOR_CENTER_X, SENSOR_CENTER_Y, SENSOR_BAR_X, SENSOR_BAR_Y),
            (WITNESS_CENTER_X, WITNESS_CENTER_Y, WITNESS_X, WITNESS_Y),
            (
                COLLECTION_CENTER_X,
                COLLECTION_CENTER_Y,
                COLLECTION_RACK_X,
                COLLECTION_RACK_Y,
            ),
            (
                WASTE_ROUTE_CENTER_X,
                WASTE_ROUTE_CENTER_Y,
                WASTE_ROUTE_X,
                WASTE_ROUTE_Y,
            ),
            (
                TRACEABILITY_CENTER_X,
                TRACEABILITY_CENTER_Y,
                TRACEABILITY_PANEL_X,
                TRACEABILITY_PANEL_Y,
            ),
            (
                STATUS_CENTER_X,
                STATUS_CENTER_Y,
                STATUS_PANEL_X,
                STATUS_PANEL_Y,
            ),
            (
                CAMERA_BRIDGE_CENTER_X,
                CAMERA_BRIDGE_CENTER_Y,
                CAMERA_BRIDGE_X,
                CAMERA_BRIDGE_Y,
            ),
        ] {
            assert!(x.abs() + sx / 2.0 < STATION_X / 2.0);
            assert!(y.abs() + sy / 2.0 < STATION_Y / 2.0);
        }
    }

    #[test]
    fn station_parts_are_constructible() {
        let parts = [
            base_leak_tray(),
            pump_cassette_nest(),
            tubing_loop_strain_relief_combs(),
            calibrated_restriction_coupon_slots(),
            pressure_sensor_pockets(),
            optical_bubble_wetness_witness(),
            gravimetric_collection_pad_placeholders(),
            overpressure_relief_waste_route(),
            cycle_counter_lot_barcode_lands(),
            pass_fail_quarantine_lanes(),
            evidence_camera_bridge(),
            robot_service_keepouts(),
            station_assembly(),
        ];
        assert_eq!(parts.len(), OUTPUTS.len());
    }
}
