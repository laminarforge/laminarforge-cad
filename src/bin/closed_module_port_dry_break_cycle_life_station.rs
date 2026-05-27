use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module dry-break service-port cycle-life validation station.
//
// Intent:
// - Cycle male/female dry-break couplers for sealed culture module fluid, gas,
//   and electrical service ports without opening the sterile boundary.
// - Capture insertion force, cycle-count witness marks, leak/pressure-decay
//   checks, cap/plug custody, traceability, disposition, and evidence imaging
//   in one fixture envelope.
// - Keep clean and used handling zones, robot sweep volumes, service access, and
//   rear bulkhead interfaces explicit in the CAD.
//
// This is fixture/interface CAD only. It intentionally does not model final
// coupler internals, seal chemistry, electrical compliance, pressure controller
// logic, or biological release criteria.
//
// Exported STL parts:
// - output/closed_module_port_dry_break_cycle_life_station_base_leak_tray.stl
// - output/closed_module_port_dry_break_cycle_life_station_male_coupler_nests.stl
// - output/closed_module_port_dry_break_cycle_life_station_female_coupler_nests.stl
// - output/closed_module_port_dry_break_cycle_life_station_cycle_count_witness_dials.stl
// - output/closed_module_port_dry_break_cycle_life_station_insertion_force_gauge_pocket.stl
// - output/closed_module_port_dry_break_cycle_life_station_leak_pressure_decay_witness_ports.stl
// - output/closed_module_port_dry_break_cycle_life_station_cap_plug_parks.stl
// - output/closed_module_port_dry_break_cycle_life_station_clean_used_segregation.stl
// - output/closed_module_port_dry_break_cycle_life_station_barcode_coa_lands.stl
// - output/closed_module_port_dry_break_cycle_life_station_release_hold_reject_lanes.stl
// - output/closed_module_port_dry_break_cycle_life_station_evidence_camera_bridge.stl
// - output/closed_module_port_dry_break_cycle_life_station_robot_service_keepouts.stl
// - output/closed_module_port_dry_break_cycle_life_station_service_bulkhead.stl
// - output/closed_module_port_dry_break_cycle_life_station_assembly.stl

const OUTPUTS: &[&str] = &[
    "output/closed_module_port_dry_break_cycle_life_station_base_leak_tray.stl",
    "output/closed_module_port_dry_break_cycle_life_station_male_coupler_nests.stl",
    "output/closed_module_port_dry_break_cycle_life_station_female_coupler_nests.stl",
    "output/closed_module_port_dry_break_cycle_life_station_cycle_count_witness_dials.stl",
    "output/closed_module_port_dry_break_cycle_life_station_insertion_force_gauge_pocket.stl",
    "output/closed_module_port_dry_break_cycle_life_station_leak_pressure_decay_witness_ports.stl",
    "output/closed_module_port_dry_break_cycle_life_station_cap_plug_parks.stl",
    "output/closed_module_port_dry_break_cycle_life_station_clean_used_segregation.stl",
    "output/closed_module_port_dry_break_cycle_life_station_barcode_coa_lands.stl",
    "output/closed_module_port_dry_break_cycle_life_station_release_hold_reject_lanes.stl",
    "output/closed_module_port_dry_break_cycle_life_station_evidence_camera_bridge.stl",
    "output/closed_module_port_dry_break_cycle_life_station_robot_service_keepouts.stl",
    "output/closed_module_port_dry_break_cycle_life_station_service_bulkhead.stl",
    "output/closed_module_port_dry_break_cycle_life_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "male_coupler_nests",
    "female_coupler_nests",
    "cycle_count_witness_dial_lands",
    "insertion_force_gauge_pocket",
    "leak_pressure_decay_witness_ports",
    "cap_plug_parks",
    "clean_used_segregation",
    "barcode_coa_lands",
    "release_hold_reject_lanes",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "service_bulkhead",
];

const BASE_X: f64 = 1280.0;
const BASE_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const LEAK_BASIN_X: f64 = BASE_X - 118.0;
const LEAK_BASIN_Y: f64 = BASE_Y - 112.0;
const DRAIN_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.8;

const SERVICE_PORT_COUNT: usize = 8;
const SERVICE_PORT_PITCH: f64 = 44.0;
const FLUID_PORTS: usize = 4;
const GAS_PORTS: usize = 2;
const ELECTRICAL_PORTS: usize = 2;
const COUPLER_SOCKET_D: f64 = 23.0;
const COUPLER_BODY_D: f64 = 18.0;
const COUPLER_SEAL_WITNESS_D: f64 = 31.0;

const MALE_CENTER_X: f64 = -395.0;
const MALE_CENTER_Y: f64 = 145.0;
const MALE_NEST_X: f64 = 390.0;
const MALE_NEST_Y: f64 = 170.0;
const MALE_NEST_Z: f64 = 46.0;

const FEMALE_CENTER_X: f64 = -395.0;
const FEMALE_CENTER_Y: f64 = -65.0;
const FEMALE_NEST_X: f64 = 390.0;
const FEMALE_NEST_Y: f64 = 170.0;
const FEMALE_NEST_Z: f64 = 50.0;

const CYCLE_CENTER_X: f64 = -25.0;
const CYCLE_CENTER_Y: f64 = 250.0;
const CYCLE_PANEL_X: f64 = 330.0;
const CYCLE_PANEL_Y: f64 = 120.0;
const CYCLE_PANEL_Z: f64 = 14.0;
const CYCLE_DIAL_COUNT: usize = 4;
const CYCLE_DIAL_D: f64 = 58.0;
const CYCLE_DIAL_PITCH: f64 = 72.0;
const CYCLE_TICK_COUNT: usize = 12;
const CYCLE_DECADE_LANDS: usize = 10;

const FORCE_CENTER_X: f64 = 360.0;
const FORCE_CENTER_Y: f64 = 210.0;
const FORCE_BLOCK_X: f64 = 330.0;
const FORCE_BLOCK_Y: f64 = 170.0;
const FORCE_BLOCK_Z: f64 = 54.0;
const FORCE_GAUGE_ENV_X: f64 = 178.0;
const FORCE_GAUGE_ENV_Y: f64 = 82.0;
const FORCE_GAUGE_ENV_Z: f64 = 214.0;
const FORCE_RAM_BORE_D: f64 = 16.0;
const FORCE_SHIM_COUNT: usize = 6;

const PRESSURE_CENTER_X: f64 = 342.0;
const PRESSURE_CENTER_Y: f64 = 0.0;
const PRESSURE_BLOCK_X: f64 = 360.0;
const PRESSURE_BLOCK_Y: f64 = 160.0;
const PRESSURE_BLOCK_Z: f64 = 48.0;
const PRESSURE_PORT_COUNT: usize = SERVICE_PORT_COUNT;
const PRESSURE_PORT_PITCH: f64 = 38.0;
const PRESSURE_PORT_D: f64 = 8.0;
const LEAK_WELL_D: f64 = 24.0;

const CAP_CENTER_X: f64 = -405.0;
const CAP_CENTER_Y: f64 = -270.0;
const CAP_PARK_X: f64 = 380.0;
const CAP_PARK_Y: f64 = 150.0;
const CAP_PARK_Z: f64 = 42.0;
const CAP_PARK_COUNT: usize = SERVICE_PORT_COUNT;
const CAP_PARK_PITCH: f64 = 42.0;
const CAP_POCKET_D: f64 = 22.0;
const PLUG_POCKET_D: f64 = 18.0;

const TRACE_CENTER_X: f64 = -20.0;
const TRACE_CENTER_Y: f64 = -300.0;
const TRACE_PANEL_X: f64 = 330.0;
const TRACE_PANEL_Y: f64 = 130.0;
const TRACE_PANEL_Z: f64 = 9.0;
const BARCODE_LANDS: usize = 8;
const COA_LANDS: usize = 3;
const RFID_LANDS: usize = 4;

const STATUS_CENTER_X: f64 = 405.0;
const STATUS_CENTER_Y: f64 = -285.0;
const STATUS_X: f64 = 360.0;
const STATUS_Y: f64 = 160.0;
const STATUS_Z: f64 = 46.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_PITCH: f64 = 84.0;
const STATUS_SLOT_X: f64 = 72.0;
const STATUS_SLOT_Y: f64 = 28.0;

const BULKHEAD_CENTER_X: f64 = 0.0;
const BULKHEAD_CENTER_Y: f64 = 360.0;
const BULKHEAD_X: f64 = 1080.0;
const BULKHEAD_Y: f64 = 58.0;
const BULKHEAD_Z: f64 = 126.0;
const BULKHEAD_FLUID_PORTS: usize = FLUID_PORTS;
const BULKHEAD_GAS_PORTS: usize = GAS_PORTS;
const BULKHEAD_ELECTRICAL_PORTS: usize = ELECTRICAL_PORTS;

const CAMERA_CENTER_X: f64 = MALE_CENTER_X;
const CAMERA_CENTER_Y: f64 = 38.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 44.0;
const CAMERA_POST_Z: f64 = 188.0;
const CAMERA_SPAN_X: f64 = 462.0;
const CAMERA_SPAN_Y: f64 = 420.0;
const CAMERA_BEAM_Z: f64 = 28.0;
const CAMERA_UNDERSIDE_Z: f64 = 182.0;
const CAMERA_COUNT: usize = 3;

const SEGREGATION_RIB_Z: f64 = 72.0;
const CLEAN_USED_AIR_GAP: f64 = 62.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 98.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 86.0;
const LEFT_COUPLER_CART_KEEP_OUT_X: f64 = 92.0;
const RIGHT_GAUGE_SERVICE_KEEP_OUT_X: f64 = 112.0;
const KEEP_OUT_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let male = male_coupler_nests();
    export(&male, OUTPUTS[1]);

    let female = female_coupler_nests();
    export(&female, OUTPUTS[2]);

    let dials = cycle_count_witness_dials();
    export(&dials, OUTPUTS[3]);

    let force = insertion_force_gauge_pocket();
    export(&force, OUTPUTS[4]);

    let pressure = leak_pressure_decay_witness_ports();
    export(&pressure, OUTPUTS[5]);

    let parks = cap_plug_parks();
    export(&parks, OUTPUTS[6]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[7]);

    let traceability = barcode_coa_lands();
    export(&traceability, OUTPUTS[8]);

    let status = release_hold_reject_lanes();
    export(&status, OUTPUTS[9]);

    let bridge = evidence_camera_bridge();
    export(&bridge, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let bulkhead = service_bulkhead();
    export(&bulkhead, OUTPUTS[12]);

    let assembly = base
        + male
        + female
        + dials
        + force
        + pressure
        + parks
        + segregation
        + traceability
        + status
        + bridge
        + keepouts
        + bulkhead;
    export(&assembly, OUTPUTS[13]);

    println!(
        "Closed module port dry-break cycle-life station: {:.0}mm x {:.0}mm leak-tray deck, {} service-port lanes ({} fluid, {} gas, {} electrical), male/female coupler nests, {} cycle-count witness dials, insertion-force gauge pocket, pressure-decay ports, cap/plug parks, traceability, disposition lanes, camera bridge, keepouts, and service bulkhead.",
        BASE_X,
        BASE_Y,
        SERVICE_PORT_COUNT,
        FLUID_PORTS,
        GAS_PORTS,
        ELECTRICAL_PORTS,
        CYCLE_DIAL_COUNT,
    );
    println!(
        "Workflow controls: clean/used air gap {:.0}mm, {} release/hold/reject lanes with {} slots each, {} barcode lands, {} COA lands, {} RFID lands, and {} required feature groups.",
        CLEAN_USED_AIR_GAP,
        STATUS_LANES,
        STATUS_SLOTS_PER_LANE,
        BARCODE_LANDS,
        COA_LANDS,
        RFID_LANDS,
        REQUIRED_FEATURES.len(),
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "dry_break_cycle_life_station_base_deck",
        BASE_X,
        BASE_Y,
        DECK_Z,
    );
    let recessed_basin = centered_cube(
        "dry_break_cycle_life_station_recessed_leak_basin",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        8.0,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - 3.0);
    let front_wipe_gutter = centered_cube(
        "dry_break_cycle_life_station_front_wipe_gutter",
        BASE_X - 220.0,
        12.0,
        8.0,
    )
    .translate(20.0, -BASE_Y / 2.0 + 84.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "dry_break_cycle_life_station_leak_tray_drain",
        DRAIN_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 92.0, -BASE_Y / 2.0 + 36.0, 0.0);

    deck - recessed_basin - front_wipe_gutter - drain - deck_mount_holes()
        + perimeter_rims()
        + robot_datum_fiducials()
        + station_flow_lands()
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "dry_break_cycle_life_station_rear_service_rim",
        BASE_X - 78.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, BASE_Y / 2.0 - RIM_W / 2.0, deck_top_z() + RIM_Z / 2.0);
    let left = centered_cube(
        "dry_break_cycle_life_station_left_clean_cart_rim",
        RIM_W,
        BASE_Y - 94.0,
        RIM_Z,
    )
    .translate(
        -BASE_X / 2.0 + RIM_W / 2.0,
        -10.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "dry_break_cycle_life_station_right_gauge_service_rim",
        RIM_W,
        BASE_Y - 132.0,
        RIM_Z,
    )
    .translate(
        BASE_X / 2.0 - RIM_W / 2.0,
        -26.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let front_low = centered_cube(
        "dry_break_cycle_life_station_front_low_wipe_lip",
        BASE_X - 230.0,
        12.0,
        20.0,
    )
    .translate(20.0, -BASE_Y / 2.0 + 28.0, deck_top_z() + 10.0);

    rear + left + right + front_low
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("dry_break_cycle_life_station_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let bore = centered_cylinder(
            format!("dry_break_cycle_life_station_m6_mount_bore_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 6.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("dry_break_cycle_life_station_m6_mount_slot_{i}"),
            26.0,
            7.0,
            DECK_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + bore + slot;
    }
    holes
}

fn robot_datum_fiducials() -> Part {
    let mut fiducials = Part::empty("dry_break_cycle_life_station_robot_datum_fiducials");
    for (i, (x, y)) in [
        (-BASE_X / 2.0 + 74.0, BASE_Y / 2.0 - 86.0),
        (BASE_X / 2.0 - 74.0, BASE_Y / 2.0 - 86.0),
        (-BASE_X / 2.0 + 74.0, -BASE_Y / 2.0 + 86.0),
        (BASE_X / 2.0 - 74.0, -BASE_Y / 2.0 + 86.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("dry_break_cycle_life_station_robot_datum_disc_{i}"),
            15.0,
            4.0,
            36,
        )
        .translate(*x, *y, deck_top_z() + 2.0);
        let cross_a = centered_cube(
            format!("dry_break_cycle_life_station_robot_datum_cross_a_{i}"),
            32.0,
            2.4,
            5.0,
        )
        .translate(*x, *y, deck_top_z() + 2.5);
        let cross_b = centered_cube(
            format!("dry_break_cycle_life_station_robot_datum_cross_b_{i}"),
            2.4,
            32.0,
            5.0,
        )
        .translate(*x, *y, deck_top_z() + 2.5);
        fiducials = fiducials + disc - cross_a - cross_b;
    }
    fiducials
}

fn station_flow_lands() -> Part {
    let clean_load = centered_cube(
        "dry_break_cycle_life_station_clean_load_flow_land",
        360.0,
        7.0,
        5.0,
    )
    .translate(MALE_CENTER_X, 42.0, deck_top_z() + 2.5);
    let test_flow = centered_cube(
        "dry_break_cycle_life_station_test_flow_land",
        470.0,
        7.0,
        5.0,
    )
    .rotate(0.0, 0.0, -16.0)
    .translate(88.0, -88.0, deck_top_z() + 2.5);
    let disposition_flow = centered_cube(
        "dry_break_cycle_life_station_disposition_flow_land",
        335.0,
        7.0,
        5.0,
    )
    .translate(282.0, -190.0, deck_top_z() + 2.5);

    clean_load + test_flow + disposition_flow
}

fn male_coupler_nests() -> Part {
    let block = centered_cube(
        "dry_break_cycle_life_station_male_coupler_nest_block",
        MALE_NEST_X,
        MALE_NEST_Y,
        MALE_NEST_Z,
    )
    .translate(MALE_CENTER_X, MALE_CENTER_Y, feature_z(MALE_NEST_Z));

    let mut sockets = Part::empty("dry_break_cycle_life_station_male_coupler_socket_cuts");
    let mut bodies = Part::empty("dry_break_cycle_life_station_male_coupler_body_witnesses");
    let mut keys = Part::empty("dry_break_cycle_life_station_male_coupler_key_witnesses");
    for lane in 0..SERVICE_PORT_COUNT {
        let x = service_lane_x(MALE_CENTER_X, lane);
        let port_type_y = if lane < FLUID_PORTS {
            MALE_CENTER_Y - 28.0
        } else if lane < FLUID_PORTS + GAS_PORTS {
            MALE_CENTER_Y + 6.0
        } else {
            MALE_CENTER_Y + 40.0
        };

        sockets = sockets
            + centered_cylinder(
                format!("dry_break_cycle_life_station_male_socket_relief_{lane}"),
                COUPLER_SOCKET_D / 2.0,
                MALE_NEST_Z + 8.0,
                44,
            )
            .translate(x, port_type_y, feature_z(MALE_NEST_Z));

        let body = centered_cylinder(
            format!("dry_break_cycle_life_station_male_coupler_body_datum_{lane}"),
            COUPLER_BODY_D / 2.0,
            32.0,
            44,
        )
        .translate(x, port_type_y, deck_top_z() + MALE_NEST_Z + 16.0);
        let seal_witness = centered_cylinder(
            format!("dry_break_cycle_life_station_male_seal_witness_ring_{lane}"),
            COUPLER_SEAL_WITNESS_D / 2.0,
            4.0,
            44,
        )
        .translate(x, port_type_y, deck_top_z() + MALE_NEST_Z + 3.0)
            - centered_cylinder(
                format!("dry_break_cycle_life_station_male_seal_witness_inner_{lane}"),
                COUPLER_BODY_D / 2.0,
                5.0,
                36,
            )
            .translate(x, port_type_y, deck_top_z() + MALE_NEST_Z + 3.0);

        let key = centered_cube(
            format!("dry_break_cycle_life_station_male_anti_rotation_key_land_{lane}"),
            6.0,
            22.0,
            8.0,
        )
        .translate(x + 16.0, port_type_y, deck_top_z() + MALE_NEST_Z + 4.0);
        let pull_clearance = centered_cube(
            format!("dry_break_cycle_life_station_male_release_collar_sweep_{lane}"),
            28.0,
            8.0,
            6.0,
        )
        .translate(x, port_type_y + 25.0, deck_top_z() + MALE_NEST_Z + 3.0);

        bodies = bodies + body + seal_witness;
        keys = keys + key + pull_clearance;
    }

    let rear_hard_stop = centered_cube(
        "dry_break_cycle_life_station_male_nest_rear_hard_stop",
        MALE_NEST_X - 46.0,
        12.0,
        34.0,
    )
    .translate(
        MALE_CENTER_X,
        MALE_CENTER_Y + MALE_NEST_Y / 2.0 - 18.0,
        deck_top_z() + MALE_NEST_Z + 17.0,
    );
    let front_pick_cut = centered_cube(
        "dry_break_cycle_life_station_male_robot_pick_clearance",
        250.0,
        22.0,
        MALE_NEST_Z + 6.0,
    )
    .translate(
        MALE_CENTER_X,
        MALE_CENTER_Y - MALE_NEST_Y / 2.0 + 20.0,
        feature_z(MALE_NEST_Z),
    );

    block - sockets - front_pick_cut + bodies + keys + rear_hard_stop + coupler_lane_flags("male")
}

fn female_coupler_nests() -> Part {
    let block = centered_cube(
        "dry_break_cycle_life_station_female_coupler_nest_block",
        FEMALE_NEST_X,
        FEMALE_NEST_Y,
        FEMALE_NEST_Z,
    )
    .translate(FEMALE_CENTER_X, FEMALE_CENTER_Y, feature_z(FEMALE_NEST_Z));

    let mut cups = Part::empty("dry_break_cycle_life_station_female_coupler_cup_cuts");
    let mut latch_ears = Part::empty("dry_break_cycle_life_station_female_latch_ear_witnesses");
    let mut collars = Part::empty("dry_break_cycle_life_station_female_coupler_lip_collars");
    for lane in 0..SERVICE_PORT_COUNT {
        let x = service_lane_x(FEMALE_CENTER_X, lane);
        let port_type_y = if lane < FLUID_PORTS {
            FEMALE_CENTER_Y - 30.0
        } else if lane < FLUID_PORTS + GAS_PORTS {
            FEMALE_CENTER_Y + 7.0
        } else {
            FEMALE_CENTER_Y + 43.0
        };
        let cup_d = if lane < FLUID_PORTS {
            26.0
        } else if lane < FLUID_PORTS + GAS_PORTS {
            22.0
        } else {
            20.0
        };

        cups = cups
            + centered_cylinder(
                format!("dry_break_cycle_life_station_female_receiver_cup_{lane}"),
                cup_d / 2.0,
                FEMALE_NEST_Z + 10.0,
                48,
            )
            .translate(x, port_type_y, feature_z(FEMALE_NEST_Z))
            + centered_cube(
                format!("dry_break_cycle_life_station_female_keyway_slot_{lane}"),
                6.0,
                28.0,
                FEMALE_NEST_Z + 10.0,
            )
            .translate(x - 16.0, port_type_y, feature_z(FEMALE_NEST_Z));

        collars = collars
            + centered_cylinder(
                format!("dry_break_cycle_life_station_female_receiver_lip_{lane}"),
                (cup_d + 9.0) / 2.0,
                5.0,
                48,
            )
            .translate(x, port_type_y, deck_top_z() + FEMALE_NEST_Z + 2.5)
            - centered_cylinder(
                format!("dry_break_cycle_life_station_female_receiver_lip_center_{lane}"),
                cup_d / 2.0,
                6.0,
                44,
            )
            .translate(x, port_type_y, deck_top_z() + FEMALE_NEST_Z + 2.5);

        latch_ears = latch_ears
            + centered_cube(
                format!("dry_break_cycle_life_station_female_left_latch_ear_{lane}"),
                7.0,
                20.0,
                12.0,
            )
            .translate(x - 21.0, port_type_y, deck_top_z() + FEMALE_NEST_Z + 6.0)
            + centered_cube(
                format!("dry_break_cycle_life_station_female_right_latch_ear_{lane}"),
                7.0,
                20.0,
                12.0,
            )
            .translate(x + 21.0, port_type_y, deck_top_z() + FEMALE_NEST_Z + 6.0);
    }

    let drain_shelf = centered_cube(
        "dry_break_cycle_life_station_female_coupler_drip_shelf",
        FEMALE_NEST_X - 58.0,
        16.0,
        8.0,
    )
    .translate(
        FEMALE_CENTER_X,
        FEMALE_CENTER_Y - FEMALE_NEST_Y / 2.0 + 20.0,
        deck_top_z() + FEMALE_NEST_Z + 4.0,
    );
    let rear_stop = centered_cube(
        "dry_break_cycle_life_station_female_nest_rear_stop",
        FEMALE_NEST_X - 48.0,
        10.0,
        30.0,
    )
    .translate(
        FEMALE_CENTER_X,
        FEMALE_CENTER_Y + FEMALE_NEST_Y / 2.0 - 18.0,
        deck_top_z() + FEMALE_NEST_Z + 15.0,
    );

    block - cups + collars + latch_ears + drain_shelf + rear_stop + coupler_lane_flags("female")
}

fn coupler_lane_flags(prefix: &str) -> Part {
    let center_y = if prefix == "male" {
        MALE_CENTER_Y
    } else {
        FEMALE_CENTER_Y
    };
    let center_x = if prefix == "male" {
        MALE_CENTER_X
    } else {
        FEMALE_CENTER_X
    };
    let z = if prefix == "male" {
        deck_top_z() + MALE_NEST_Z + 3.0
    } else {
        deck_top_z() + FEMALE_NEST_Z + 3.0
    };

    let mut flags = Part::empty(format!("dry_break_cycle_life_station_{prefix}_lane_flags"));
    for lane in 0..SERVICE_PORT_COUNT {
        let x = service_lane_x(center_x, lane);
        let y = center_y - 72.0;
        let width = if lane < FLUID_PORTS {
            26.0
        } else if lane < FLUID_PORTS + GAS_PORTS {
            20.0
        } else {
            16.0
        };
        flags = flags
            + centered_cube(
                format!("dry_break_cycle_life_station_{prefix}_lane_type_land_{lane}"),
                width,
                12.0,
                6.0,
            )
            .translate(x, y, z);
    }
    flags
}

fn cycle_count_witness_dials() -> Part {
    let plate = centered_cube(
        "dry_break_cycle_life_station_cycle_count_witness_panel",
        CYCLE_PANEL_X,
        CYCLE_PANEL_Y,
        CYCLE_PANEL_Z,
    )
    .translate(CYCLE_CENTER_X, CYCLE_CENTER_Y, feature_z(CYCLE_PANEL_Z));
    let mut dials = Part::empty("dry_break_cycle_life_station_cycle_count_dial_lands");
    let mut holes = Part::empty("dry_break_cycle_life_station_cycle_count_dial_center_holes");
    let mut ticks = Part::empty("dry_break_cycle_life_station_cycle_count_dial_ticks");
    for dial in 0..CYCLE_DIAL_COUNT {
        let x = CYCLE_CENTER_X + lane_x(dial, CYCLE_DIAL_COUNT, CYCLE_DIAL_PITCH);
        let y = CYCLE_CENTER_Y + 6.0;
        dials = dials
            + centered_cylinder(
                format!("dry_break_cycle_life_station_cycle_dial_land_{dial}"),
                CYCLE_DIAL_D / 2.0,
                5.0,
                64,
            )
            .translate(x, y, deck_top_z() + CYCLE_PANEL_Z + 2.5);
        holes = holes
            + centered_cylinder(
                format!("dry_break_cycle_life_station_cycle_dial_center_pin_{dial}"),
                4.5,
                8.0,
                28,
            )
            .translate(x, y, deck_top_z() + CYCLE_PANEL_Z + 3.0);

        for tick in 0..CYCLE_TICK_COUNT {
            let angle = 360.0 * tick as f64 / CYCLE_TICK_COUNT as f64;
            let radius = CYCLE_DIAL_D / 2.0 - 5.5;
            let dx = radius * angle.to_radians().cos();
            let dy = radius * angle.to_radians().sin();
            ticks = ticks
                + centered_cube(
                    format!("dry_break_cycle_life_station_cycle_dial_tick_{dial}_{tick}"),
                    2.2,
                    9.0,
                    3.0,
                )
                .rotate(0.0, 0.0, angle)
                .translate(x + dx, y + dy, deck_top_z() + CYCLE_PANEL_Z + 6.5);
        }
    }

    let mut decade_lands =
        Part::empty("dry_break_cycle_life_station_cycle_count_decade_witness_lands");
    for i in 0..CYCLE_DECADE_LANDS {
        decade_lands = decade_lands
            + centered_cube(
                format!("dry_break_cycle_life_station_cycle_decade_coupon_land_{i}"),
                22.0,
                11.0,
                4.0,
            )
            .translate(
                CYCLE_CENTER_X - 126.0 + i as f64 * 28.0,
                CYCLE_CENTER_Y - 42.0,
                deck_top_z() + CYCLE_PANEL_Z + 2.0,
            );
    }

    let locked_counter_window = centered_cube(
        "dry_break_cycle_life_station_locked_counter_window_land",
        94.0,
        18.0,
        4.0,
    )
    .translate(
        CYCLE_CENTER_X,
        CYCLE_CENTER_Y + CYCLE_PANEL_Y / 2.0 - 18.0,
        deck_top_z() + CYCLE_PANEL_Z + 2.0,
    );

    plate + dials + ticks + decade_lands + locked_counter_window - holes
}

fn insertion_force_gauge_pocket() -> Part {
    let block = centered_cube(
        "dry_break_cycle_life_station_insertion_force_gauge_block",
        FORCE_BLOCK_X,
        FORCE_BLOCK_Y,
        FORCE_BLOCK_Z,
    )
    .translate(FORCE_CENTER_X, FORCE_CENTER_Y, feature_z(FORCE_BLOCK_Z));

    let gauge_pocket = centered_cube(
        "dry_break_cycle_life_station_force_gauge_body_pocket",
        FORCE_GAUGE_ENV_X + 18.0,
        FORCE_GAUGE_ENV_Y + 18.0,
        18.0,
    )
    .translate(
        FORCE_CENTER_X - 36.0,
        FORCE_CENTER_Y + 22.0,
        deck_top_z() + FORCE_BLOCK_Z - 4.0,
    );
    let gauge_envelope = centered_cube(
        "dry_break_cycle_life_station_force_gauge_envelope_placeholder",
        FORCE_GAUGE_ENV_X,
        FORCE_GAUGE_ENV_Y,
        FORCE_GAUGE_ENV_Z,
    )
    .translate(
        FORCE_CENTER_X - 36.0,
        FORCE_CENTER_Y + 22.0,
        deck_top_z() + FORCE_BLOCK_Z + FORCE_GAUGE_ENV_Z / 2.0,
    );
    let ram_bore = centered_cylinder(
        "dry_break_cycle_life_station_force_ram_bore",
        FORCE_RAM_BORE_D / 2.0,
        FORCE_BLOCK_Y + 18.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        FORCE_CENTER_X + 110.0,
        FORCE_CENTER_Y,
        deck_top_z() + FORCE_BLOCK_Z / 2.0,
    );
    let receiver_pocket = centered_cylinder(
        "dry_break_cycle_life_station_force_receiver_coupler_pocket",
        31.0 / 2.0,
        FORCE_BLOCK_Z + 8.0,
        44,
    )
    .translate(
        FORCE_CENTER_X + 110.0,
        FORCE_CENTER_Y - 46.0,
        feature_z(FORCE_BLOCK_Z),
    );

    let top_reaction_bridge = centered_cube(
        "dry_break_cycle_life_station_force_gauge_reaction_bridge",
        FORCE_BLOCK_X - 54.0,
        14.0,
        34.0,
    )
    .translate(
        FORCE_CENTER_X,
        FORCE_CENTER_Y + FORCE_BLOCK_Y / 2.0 - 20.0,
        deck_top_z() + FORCE_BLOCK_Z + 17.0,
    );
    let ram_guide_left = centered_cube(
        "dry_break_cycle_life_station_force_ram_left_guide",
        16.0,
        FORCE_BLOCK_Y - 42.0,
        22.0,
    )
    .translate(
        FORCE_CENTER_X + 88.0,
        FORCE_CENTER_Y - 8.0,
        deck_top_z() + FORCE_BLOCK_Z + 11.0,
    );
    let ram_guide_right = centered_cube(
        "dry_break_cycle_life_station_force_ram_right_guide",
        16.0,
        FORCE_BLOCK_Y - 42.0,
        22.0,
    )
    .translate(
        FORCE_CENTER_X + 132.0,
        FORCE_CENTER_Y - 8.0,
        deck_top_z() + FORCE_BLOCK_Z + 11.0,
    );

    block - gauge_pocket - ram_bore - receiver_pocket
        + gauge_envelope
        + top_reaction_bridge
        + ram_guide_left
        + ram_guide_right
        + force_reference_shim_parks()
        + force_zero_stop()
}

fn force_reference_shim_parks() -> Part {
    let mut parks = Part::empty("dry_break_cycle_life_station_force_reference_shim_parks");
    for i in 0..FORCE_SHIM_COUNT {
        parks = parks
            + centered_cube(
                format!("dry_break_cycle_life_station_force_shim_slot_{i}"),
                10.0,
                48.0,
                30.0,
            )
            .translate(
                FORCE_CENTER_X - 156.0 + i as f64 * 18.0,
                FORCE_CENTER_Y - 56.0,
                deck_top_z() + FORCE_BLOCK_Z + 15.0,
            );
    }
    parks
}

fn force_zero_stop() -> Part {
    let stop_bar = centered_cube(
        "dry_break_cycle_life_station_force_zero_stop_bar",
        104.0,
        10.0,
        28.0,
    )
    .translate(
        FORCE_CENTER_X + 108.0,
        FORCE_CENTER_Y - FORCE_BLOCK_Y / 2.0 + 18.0,
        deck_top_z() + FORCE_BLOCK_Z + 14.0,
    );
    let hard_stop_pad = centered_cube(
        "dry_break_cycle_life_station_force_hard_stop_pad",
        44.0,
        12.0,
        24.0,
    )
    .translate(
        FORCE_CENTER_X + 110.0,
        FORCE_CENTER_Y - FORCE_BLOCK_Y / 2.0 + 38.0,
        deck_top_z() + FORCE_BLOCK_Z + 12.0,
    );

    stop_bar + hard_stop_pad
}

fn leak_pressure_decay_witness_ports() -> Part {
    let block = centered_cube(
        "dry_break_cycle_life_station_pressure_decay_manifold_block",
        PRESSURE_BLOCK_X,
        PRESSURE_BLOCK_Y,
        PRESSURE_BLOCK_Z,
    )
    .translate(
        PRESSURE_CENTER_X,
        PRESSURE_CENTER_Y,
        feature_z(PRESSURE_BLOCK_Z),
    );

    let mut port_bores = Part::empty("dry_break_cycle_life_station_pressure_port_bores");
    let mut collars = Part::empty("dry_break_cycle_life_station_pressure_port_collars");
    let mut wells = Part::empty("dry_break_cycle_life_station_leak_witness_wells");
    for port in 0..PRESSURE_PORT_COUNT {
        let x = PRESSURE_CENTER_X + lane_x(port, PRESSURE_PORT_COUNT, PRESSURE_PORT_PITCH);
        let y = PRESSURE_CENTER_Y + 44.0;
        port_bores = port_bores
            + centered_cylinder(
                format!("dry_break_cycle_life_station_pressure_decay_port_bore_{port}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_BLOCK_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, deck_top_z() + PRESSURE_BLOCK_Z / 2.0);
        collars = collars
            + centered_cylinder(
                format!("dry_break_cycle_life_station_pressure_decay_port_collar_{port}"),
                15.0 / 2.0,
                6.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                y - PRESSURE_BLOCK_Y / 2.0 - 1.0,
                deck_top_z() + PRESSURE_BLOCK_Z / 2.0,
            );

        wells = wells
            + centered_cylinder(
                format!("dry_break_cycle_life_station_leak_witness_well_{port}"),
                LEAK_WELL_D / 2.0,
                10.0,
                36,
            )
            .translate(
                x,
                PRESSURE_CENTER_Y - 40.0,
                deck_top_z() + PRESSURE_BLOCK_Z - 3.0,
            );
    }

    let decay_sensor_pocket = centered_cube(
        "dry_break_cycle_life_station_pressure_decay_sensor_pocket",
        118.0,
        54.0,
        16.0,
    )
    .translate(
        PRESSURE_CENTER_X - 108.0,
        PRESSURE_CENTER_Y + 2.0,
        deck_top_z() + PRESSURE_BLOCK_Z - 4.0,
    );
    let reference_volume_land = centered_cube(
        "dry_break_cycle_life_station_reference_volume_land",
        132.0,
        22.0,
        8.0,
    )
    .translate(
        PRESSURE_CENTER_X + 102.0,
        PRESSURE_CENTER_Y - 4.0,
        deck_top_z() + PRESSURE_BLOCK_Z + 4.0,
    );
    let bleed_capture_channel = centered_cube(
        "dry_break_cycle_life_station_bleed_capture_channel",
        PRESSURE_BLOCK_X - 52.0,
        10.0,
        8.0,
    )
    .translate(
        PRESSURE_CENTER_X,
        PRESSURE_CENTER_Y - PRESSURE_BLOCK_Y / 2.0 + 18.0,
        deck_top_z() + PRESSURE_BLOCK_Z - 3.0,
    );

    block - port_bores - wells - decay_sensor_pocket - bleed_capture_channel
        + collars
        + reference_volume_land
        + pressure_decay_lane_tags()
}

fn pressure_decay_lane_tags() -> Part {
    let mut tags = Part::empty("dry_break_cycle_life_station_pressure_decay_lane_tags");
    for port in 0..PRESSURE_PORT_COUNT {
        tags = tags
            + centered_cube(
                format!("dry_break_cycle_life_station_pressure_decay_lane_tag_{port}"),
                24.0,
                10.0,
                4.0,
            )
            .translate(
                PRESSURE_CENTER_X + lane_x(port, PRESSURE_PORT_COUNT, PRESSURE_PORT_PITCH),
                PRESSURE_CENTER_Y + PRESSURE_BLOCK_Y / 2.0 - 14.0,
                deck_top_z() + PRESSURE_BLOCK_Z + 2.0,
            );
    }
    tags
}

fn cap_plug_parks() -> Part {
    let tray = centered_cube(
        "dry_break_cycle_life_station_cap_plug_park_tray",
        CAP_PARK_X,
        CAP_PARK_Y,
        CAP_PARK_Z,
    )
    .translate(CAP_CENTER_X, CAP_CENTER_Y, feature_z(CAP_PARK_Z));
    let mut cuts = Part::empty("dry_break_cycle_life_station_cap_plug_park_pocket_cuts");
    let mut anchors = Part::empty("dry_break_cycle_life_station_cap_plug_chain_anchors");
    for lane in 0..CAP_PARK_COUNT {
        let x = CAP_CENTER_X + lane_x(lane, CAP_PARK_COUNT, CAP_PARK_PITCH);
        cuts = cuts
            + centered_cylinder(
                format!("dry_break_cycle_life_station_clean_cap_park_pocket_{lane}"),
                CAP_POCKET_D / 2.0,
                CAP_PARK_Z + 8.0,
                36,
            )
            .translate(x, CAP_CENTER_Y + 28.0, feature_z(CAP_PARK_Z))
            + centered_cylinder(
                format!("dry_break_cycle_life_station_used_plug_park_pocket_{lane}"),
                PLUG_POCKET_D / 2.0,
                CAP_PARK_Z + 8.0,
                32,
            )
            .translate(x, CAP_CENTER_Y - 30.0, feature_z(CAP_PARK_Z));
        anchors = anchors
            + centered_cylinder(
                format!("dry_break_cycle_life_station_cap_plug_tether_anchor_{lane}"),
                5.0,
                10.0,
                24,
            )
            .translate(x, CAP_CENTER_Y, deck_top_z() + CAP_PARK_Z + 5.0);
    }

    let clean_front_rail = centered_cube(
        "dry_break_cycle_life_station_clean_cap_park_front_rail",
        CAP_PARK_X - 42.0,
        8.0,
        24.0,
    )
    .translate(
        CAP_CENTER_X,
        CAP_CENTER_Y + CAP_PARK_Y / 2.0 - 18.0,
        deck_top_z() + CAP_PARK_Z + 12.0,
    );
    let used_rear_rail = centered_cube(
        "dry_break_cycle_life_station_used_plug_park_rear_rail",
        CAP_PARK_X - 42.0,
        8.0,
        24.0,
    )
    .translate(
        CAP_CENTER_X,
        CAP_CENTER_Y - CAP_PARK_Y / 2.0 + 18.0,
        deck_top_z() + CAP_PARK_Z + 12.0,
    );

    tray - cuts + anchors + clean_front_rail + used_rear_rail + cap_plug_lane_tags()
}

fn cap_plug_lane_tags() -> Part {
    let mut tags = Part::empty("dry_break_cycle_life_station_cap_plug_lane_tags");
    for lane in 0..CAP_PARK_COUNT {
        tags = tags
            + centered_cube(
                format!("dry_break_cycle_life_station_cap_plug_lane_tag_{lane}"),
                24.0,
                8.0,
                4.0,
            )
            .translate(
                CAP_CENTER_X + lane_x(lane, CAP_PARK_COUNT, CAP_PARK_PITCH),
                CAP_CENTER_Y + CAP_PARK_Y / 2.0 - 12.0,
                deck_top_z() + CAP_PARK_Z + 2.0,
            );
    }
    tags
}

fn clean_used_segregation() -> Part {
    let center_air_gap = centered_cube(
        "dry_break_cycle_life_station_clean_used_center_air_gap_witness",
        CLEAN_USED_AIR_GAP,
        10.0,
        6.0,
    )
    .translate(-178.0, -170.0, deck_top_z() + 3.0);
    let vertical_rib = centered_cube(
        "dry_break_cycle_life_station_clean_used_vertical_segregation_rib",
        14.0,
        312.0,
        SEGREGATION_RIB_Z,
    )
    .translate(-188.0, -166.0, deck_top_z() + SEGREGATION_RIB_Z / 2.0);
    let transfer_gate = centered_cube(
        "dry_break_cycle_life_station_clean_used_transfer_gate_relief",
        18.0,
        66.0,
        SEGREGATION_RIB_Z + 4.0,
    )
    .translate(-188.0, -42.0, deck_top_z() + SEGREGATION_RIB_Z / 2.0);
    let clean_floor_land = centered_cube(
        "dry_break_cycle_life_station_clean_zone_floor_land",
        352.0,
        18.0,
        5.0,
    )
    .translate(-405.0, -184.0, deck_top_z() + 2.5);
    let used_floor_land = centered_cube(
        "dry_break_cycle_life_station_used_zone_floor_land",
        414.0,
        18.0,
        5.0,
    )
    .translate(214.0, -184.0, deck_top_z() + 2.5);
    let wipe_bridge = centered_cube(
        "dry_break_cycle_life_station_clean_used_wipe_bridge",
        78.0,
        26.0,
        12.0,
    )
    .translate(-188.0, -42.0, deck_top_z() + 6.0);

    vertical_rib - transfer_gate + center_air_gap + clean_floor_land + used_floor_land + wipe_bridge
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "dry_break_cycle_life_station_barcode_coa_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER_X, TRACE_CENTER_Y, feature_z(TRACE_PANEL_Z));

    let mut barcode_lands = Part::empty("dry_break_cycle_life_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 4;
        let row = i / 4;
        barcode_lands = barcode_lands
            + centered_cube(
                format!("dry_break_cycle_life_station_barcode_land_{i}"),
                64.0,
                18.0,
                4.0,
            )
            .translate(
                TRACE_CENTER_X - 108.0 + col as f64 * 72.0,
                TRACE_CENTER_Y + 34.0 - row as f64 * 28.0,
                deck_top_z() + TRACE_PANEL_Z + 2.0,
            );
    }

    let mut coa_lands = Part::empty("dry_break_cycle_life_station_coa_lands");
    for i in 0..COA_LANDS {
        coa_lands = coa_lands
            + centered_cube(
                format!("dry_break_cycle_life_station_coa_document_land_{i}"),
                78.0,
                28.0,
                4.0,
            )
            .translate(
                TRACE_CENTER_X - 88.0 + i as f64 * 88.0,
                TRACE_CENTER_Y - 44.0,
                deck_top_z() + TRACE_PANEL_Z + 2.0,
            );
    }

    let mut rfid = Part::empty("dry_break_cycle_life_station_rfid_lands");
    for i in 0..RFID_LANDS {
        rfid = rfid
            + centered_cylinder(
                format!("dry_break_cycle_life_station_rfid_disc_land_{i}"),
                11.0,
                4.0,
                28,
            )
            .translate(
                TRACE_CENTER_X + 122.0,
                TRACE_CENTER_Y + 40.0 - i as f64 * 28.0,
                deck_top_z() + TRACE_PANEL_Z + 2.0,
            );
    }

    let custody_ridge = centered_cube(
        "dry_break_cycle_life_station_traceability_custody_ridge",
        TRACE_PANEL_X - 36.0,
        7.0,
        10.0,
    )
    .translate(
        TRACE_CENTER_X,
        TRACE_CENTER_Y + TRACE_PANEL_Y / 2.0 - 14.0,
        deck_top_z() + TRACE_PANEL_Z + 5.0,
    );

    panel + barcode_lands + coa_lands + rfid + custody_ridge
}

fn release_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "dry_break_cycle_life_station_release_hold_reject_tray",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(STATUS_CENTER_X, STATUS_CENTER_Y, feature_z(STATUS_Z));
    let mut cuts = Part::empty("dry_break_cycle_life_station_release_hold_reject_slot_cuts");
    let mut lane_lands = Part::empty("dry_break_cycle_life_station_release_hold_reject_lane_lands");
    let status_names = ["release", "hold", "reject"];
    for lane in 0..STATUS_LANES {
        let y = STATUS_CENTER_Y + lane_y(lane, STATUS_LANES, STATUS_LANE_PITCH);
        let lane_name = status_names[lane];
        lane_lands = lane_lands
            + centered_cube(
                format!("dry_break_cycle_life_station_{lane_name}_lane_header_land"),
                STATUS_X - 42.0,
                14.0,
                5.0,
            )
            .translate(
                STATUS_CENTER_X,
                y + STATUS_LANE_PITCH / 2.0 - 18.0,
                deck_top_z() + STATUS_Z + 2.5,
            );

        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("dry_break_cycle_life_station_{lane_name}_lane_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_Z + 8.0,
                )
                .translate(
                    STATUS_CENTER_X - 126.0 + slot as f64 * 84.0,
                    y,
                    feature_z(STATUS_Z),
                );
        }
    }

    let divider_a = centered_cube(
        "dry_break_cycle_life_station_release_hold_lane_divider",
        STATUS_X - 28.0,
        8.0,
        34.0,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y - STATUS_LANE_PITCH / 2.0,
        deck_top_z() + STATUS_Z + 17.0,
    );
    let divider_b = centered_cube(
        "dry_break_cycle_life_station_hold_reject_lane_divider",
        STATUS_X - 28.0,
        8.0,
        34.0,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y + STATUS_LANE_PITCH / 2.0,
        deck_top_z() + STATUS_Z + 17.0,
    );
    let reject_lid_ledge = centered_cube(
        "dry_break_cycle_life_station_reject_lane_lid_ledge",
        STATUS_X - 52.0,
        9.0,
        8.0,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y - STATUS_Y / 2.0 + 18.0,
        deck_top_z() + STATUS_Z + 4.0,
    );
    let hold_timer_witness = centered_cube(
        "dry_break_cycle_life_station_hold_timer_witness_land",
        112.0,
        18.0,
        5.0,
    )
    .translate(
        STATUS_CENTER_X + 112.0,
        STATUS_CENTER_Y,
        deck_top_z() + STATUS_Z + 2.5,
    );

    tray - cuts + lane_lands + divider_a + divider_b + reject_lid_ledge + hold_timer_witness
}

fn evidence_camera_bridge() -> Part {
    let left_front = camera_post("left_front", -CAMERA_SPAN_X / 2.0, -CAMERA_SPAN_Y / 2.0);
    let left_rear = camera_post("left_rear", -CAMERA_SPAN_X / 2.0, CAMERA_SPAN_Y / 2.0);
    let right_front = camera_post("right_front", CAMERA_SPAN_X / 2.0, -CAMERA_SPAN_Y / 2.0);
    let right_rear = camera_post("right_rear", CAMERA_SPAN_X / 2.0, CAMERA_SPAN_Y / 2.0);
    let x_beam = centered_cube(
        "dry_break_cycle_life_station_evidence_camera_x_beam",
        CAMERA_SPAN_X + CAMERA_POST_X,
        28.0,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X,
        CAMERA_CENTER_Y,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let front_y_beam = centered_cube(
        "dry_break_cycle_life_station_evidence_camera_front_y_beam",
        26.0,
        CAMERA_SPAN_Y + CAMERA_POST_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X - CAMERA_SPAN_X / 2.0,
        CAMERA_CENTER_Y,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let rear_y_beam = centered_cube(
        "dry_break_cycle_life_station_evidence_camera_rear_y_beam",
        26.0,
        CAMERA_SPAN_Y + CAMERA_POST_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X + CAMERA_SPAN_X / 2.0,
        CAMERA_CENTER_Y,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );

    left_front
        + left_rear
        + right_front
        + right_rear
        + x_beam
        + front_y_beam
        + rear_y_beam
        + evidence_cameras()
        + bridge_lighting_lands()
}

fn camera_post(name: &str, dx: f64, dy: f64) -> Part {
    centered_cube(
        format!("dry_break_cycle_life_station_camera_post_{name}"),
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_POST_Z,
    )
    .translate(
        CAMERA_CENTER_X + dx,
        CAMERA_CENTER_Y + dy,
        deck_top_z() + CAMERA_POST_Z / 2.0,
    )
}

fn evidence_cameras() -> Part {
    let mut cameras = Part::empty("dry_break_cycle_life_station_evidence_camera_mounts");
    for i in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("dry_break_cycle_life_station_evidence_camera_mount_plate_{i}"),
                54.0,
                42.0,
                8.0,
            )
            .translate(
                CAMERA_CENTER_X - 124.0 + i as f64 * 124.0,
                CAMERA_CENTER_Y,
                CAMERA_UNDERSIDE_Z - 4.0,
            )
            + centered_cylinder(
                format!("dry_break_cycle_life_station_evidence_camera_lens_keepclear_{i}"),
                16.0,
                12.0,
                32,
            )
            .translate(
                CAMERA_CENTER_X - 124.0 + i as f64 * 124.0,
                CAMERA_CENTER_Y - 18.0,
                CAMERA_UNDERSIDE_Z - 12.0,
            );
    }
    cameras
}

fn bridge_lighting_lands() -> Part {
    let front_light = centered_cube(
        "dry_break_cycle_life_station_evidence_front_light_bar_land",
        CAMERA_SPAN_X - 72.0,
        12.0,
        6.0,
    )
    .translate(
        CAMERA_CENTER_X,
        CAMERA_CENTER_Y - CAMERA_SPAN_Y / 2.0 + 26.0,
        CAMERA_UNDERSIDE_Z - 3.0,
    );
    let rear_light = centered_cube(
        "dry_break_cycle_life_station_evidence_rear_light_bar_land",
        CAMERA_SPAN_X - 72.0,
        12.0,
        6.0,
    )
    .translate(
        CAMERA_CENTER_X,
        CAMERA_CENTER_Y + CAMERA_SPAN_Y / 2.0 - 26.0,
        CAMERA_UNDERSIDE_Z - 3.0,
    );
    front_light + rear_light
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "dry_break_cycle_life_station_front_robot_handoff_keepout",
        BASE_X - 220.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        10.0,
        -BASE_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        deck_top_z() + KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "dry_break_cycle_life_station_rear_service_keepout",
        BASE_X - 180.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
        deck_top_z() + KEEP_OUT_Z / 2.0,
    );
    let left_cart = centered_cube(
        "dry_break_cycle_life_station_left_coupler_cart_keepout",
        10.0,
        BASE_Y - 194.0,
        KEEP_OUT_Z,
    )
    .translate(
        -BASE_X / 2.0 + LEFT_COUPLER_CART_KEEP_OUT_X,
        -10.0,
        deck_top_z() + KEEP_OUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "dry_break_cycle_life_station_right_gauge_service_keepout",
        10.0,
        BASE_Y - 230.0,
        KEEP_OUT_Z,
    )
    .translate(
        BASE_X / 2.0 - RIGHT_GAUGE_SERVICE_KEEP_OUT_X,
        -28.0,
        deck_top_z() + KEEP_OUT_Z / 2.0,
    );
    let bridge_sweep = centered_cube(
        "dry_break_cycle_life_station_camera_bridge_service_sweep_keepout",
        CAMERA_SPAN_X + 82.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        CAMERA_CENTER_X,
        CAMERA_CENTER_Y + CAMERA_SPAN_Y / 2.0 + 28.0,
        deck_top_z() + KEEP_OUT_Z / 2.0,
    );

    front_robot + rear_service + left_cart + right_service + bridge_sweep
}

fn service_bulkhead() -> Part {
    let body = centered_cube(
        "dry_break_cycle_life_station_rear_service_bulkhead_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y,
        deck_top_z() + BULKHEAD_Z / 2.0,
    );
    let mut cuts = Part::empty("dry_break_cycle_life_station_service_bulkhead_cuts");
    let mut collars = Part::empty("dry_break_cycle_life_station_service_bulkhead_collars");

    for i in 0..BULKHEAD_FLUID_PORTS {
        let x = BULKHEAD_CENTER_X - 330.0 + i as f64 * 50.0;
        cuts = cuts
            + centered_cylinder(
                format!("dry_break_cycle_life_station_bulkhead_fluid_bore_{i}"),
                9.0 / 2.0,
                BULKHEAD_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y, deck_top_z() + 82.0);
        collars = collars
            + centered_cylinder(
                format!("dry_break_cycle_life_station_bulkhead_fluid_collar_{i}"),
                21.0 / 2.0,
                6.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 2.0,
                deck_top_z() + 82.0,
            );
    }

    for i in 0..BULKHEAD_GAS_PORTS {
        let x = BULKHEAD_CENTER_X - 90.0 + i as f64 * 58.0;
        cuts = cuts
            + centered_cylinder(
                format!("dry_break_cycle_life_station_bulkhead_gas_bore_{i}"),
                11.0 / 2.0,
                BULKHEAD_Y + 10.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y, deck_top_z() + 82.0);
        collars = collars
            + centered_cylinder(
                format!("dry_break_cycle_life_station_bulkhead_gas_collar_{i}"),
                25.0 / 2.0,
                6.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 2.0,
                deck_top_z() + 82.0,
            );
    }

    for i in 0..BULKHEAD_ELECTRICAL_PORTS {
        let x = BULKHEAD_CENTER_X + 118.0 + i as f64 * 82.0;
        cuts = cuts
            + centered_cube(
                format!("dry_break_cycle_life_station_bulkhead_electrical_keyed_cut_{i}"),
                38.0,
                BULKHEAD_Y + 10.0,
                20.0,
            )
            .translate(x, BULKHEAD_CENTER_Y, deck_top_z() + 84.0);
        collars = collars
            + centered_cube(
                format!("dry_break_cycle_life_station_bulkhead_electrical_face_land_{i}"),
                62.0,
                6.0,
                38.0,
            )
            .translate(
                x,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 2.0,
                deck_top_z() + 84.0,
            );
    }

    let service_cable_chase = centered_cube(
        "dry_break_cycle_life_station_bulkhead_rear_cable_chase",
        BULKHEAD_X - 116.0,
        12.0,
        20.0,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y + BULKHEAD_Y / 2.0 - 10.0,
        deck_top_z() + 28.0,
    );
    let drip_shelf = centered_cube(
        "dry_break_cycle_life_station_bulkhead_drip_shelf",
        BULKHEAD_X - 96.0,
        18.0,
        9.0,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 9.0,
        deck_top_z() + 30.0,
    );
    let coa_socket = centered_cube(
        "dry_break_cycle_life_station_bulkhead_service_record_socket",
        144.0,
        8.0,
        48.0,
    )
    .translate(
        BULKHEAD_CENTER_X + 384.0,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 2.0,
        deck_top_z() + 86.0,
    );

    body - cuts - service_cable_chase + collars + drip_shelf + coa_socket + bulkhead_lane_tags()
}

fn bulkhead_lane_tags() -> Part {
    let mut tags = Part::empty("dry_break_cycle_life_station_bulkhead_lane_tags");
    for lane in 0..SERVICE_PORT_COUNT {
        tags = tags
            + centered_cube(
                format!("dry_break_cycle_life_station_bulkhead_lane_tag_{lane}"),
                38.0,
                5.0,
                10.0,
            )
            .translate(
                BULKHEAD_CENTER_X - 330.0 + lane as f64 * 52.0,
                BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
                deck_top_z() + 116.0,
            );
    }
    tags
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-BASE_X / 2.0 + 42.0, -BASE_Y / 2.0 + 42.0),
        (BASE_X / 2.0 - 42.0, -BASE_Y / 2.0 + 42.0),
        (-BASE_X / 2.0 + 42.0, BASE_Y / 2.0 - 42.0),
        (BASE_X / 2.0 - 42.0, BASE_Y / 2.0 - 42.0),
        (0.0, -BASE_Y / 2.0 + 42.0),
        (0.0, BASE_Y / 2.0 - 42.0),
        (-BASE_X / 2.0 + 42.0, 0.0),
        (BASE_X / 2.0 - 42.0, 0.0),
    ]
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn feature_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn lane_y(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn service_lane_x(center_x: f64, lane: usize) -> f64 {
    center_x + lane_x(lane, SERVICE_PORT_COUNT, SERVICE_PORT_PITCH)
}

fn assert_layout() {
    for rect in [
        male_rect(),
        female_rect(),
        cycle_rect(),
        force_rect(),
        pressure_rect(),
        cap_rect(),
        trace_rect(),
        status_rect(),
        bulkhead_rect(),
    ] {
        assert!(
            rect.fits_inside(BASE_X, BASE_Y),
            "{} exceeds base footprint",
            rect.name
        );
    }

    assert!(!male_rect().overlaps(cycle_rect()));
    assert!(!female_rect().overlaps(pressure_rect()));
    assert!(!cap_rect().overlaps(trace_rect()));
    assert!(!trace_rect().overlaps(status_rect()));
    assert!(!force_rect().overlaps(pressure_rect()));
    assert!(!cycle_rect().overlaps(force_rect()));
    assert!(!cycle_rect().overlaps(bulkhead_rect()));
    assert!(CLEAN_USED_AIR_GAP >= 60.0);
    assert!(CAMERA_UNDERSIDE_Z > deck_top_z() + FEMALE_NEST_Z + 82.0);
    assert_eq!(
        SERVICE_PORT_COUNT,
        FLUID_PORTS + GAS_PORTS + ELECTRICAL_PORTS
    );
}

fn male_rect() -> Rect {
    Rect::new(
        "male coupler nests",
        MALE_CENTER_X,
        MALE_CENTER_Y,
        MALE_NEST_X,
        MALE_NEST_Y,
    )
}

fn female_rect() -> Rect {
    Rect::new(
        "female coupler nests",
        FEMALE_CENTER_X,
        FEMALE_CENTER_Y,
        FEMALE_NEST_X,
        FEMALE_NEST_Y,
    )
}

fn cycle_rect() -> Rect {
    Rect::new(
        "cycle count witness dials",
        CYCLE_CENTER_X,
        CYCLE_CENTER_Y,
        CYCLE_PANEL_X,
        CYCLE_PANEL_Y,
    )
}

fn force_rect() -> Rect {
    Rect::new(
        "insertion force gauge pocket",
        FORCE_CENTER_X,
        FORCE_CENTER_Y,
        FORCE_BLOCK_X,
        FORCE_BLOCK_Y,
    )
}

fn pressure_rect() -> Rect {
    Rect::new(
        "leak pressure decay ports",
        PRESSURE_CENTER_X,
        PRESSURE_CENTER_Y,
        PRESSURE_BLOCK_X,
        PRESSURE_BLOCK_Y,
    )
}

fn cap_rect() -> Rect {
    Rect::new(
        "cap plug parks",
        CAP_CENTER_X,
        CAP_CENTER_Y,
        CAP_PARK_X,
        CAP_PARK_Y,
    )
}

fn trace_rect() -> Rect {
    Rect::new(
        "barcode COA lands",
        TRACE_CENTER_X,
        TRACE_CENTER_Y,
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
    )
}

fn status_rect() -> Rect {
    Rect::new(
        "release hold reject lanes",
        STATUS_CENTER_X,
        STATUS_CENTER_Y,
        STATUS_X,
        STATUS_Y,
    )
}

fn bulkhead_rect() -> Rect {
    Rect::new(
        "service bulkhead",
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y,
        BULKHEAD_X,
        BULKHEAD_Y,
    )
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    cx: f64,
    cy: f64,
    x: f64,
    y: f64,
}

impl Rect {
    const fn new(name: &'static str, cx: f64, cy: f64, x: f64, y: f64) -> Self {
        Self { name, cx, cy, x, y }
    }

    fn left(self) -> f64 {
        self.cx - self.x / 2.0
    }

    fn right(self) -> f64 {
        self.cx + self.x / 2.0
    }

    fn bottom(self) -> f64 {
        self.cy - self.y / 2.0
    }

    fn top(self) -> f64 {
        self.cy + self.y / 2.0
    }

    fn fits_inside(self, max_x: f64, max_y: f64) -> bool {
        self.left() >= -max_x / 2.0
            && self.right() <= max_x / 2.0
            && self.bottom() >= -max_y / 2.0
            && self.top() <= max_y / 2.0
    }

    fn overlaps(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_plan_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_module_port_dry_break_cycle_life_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_workflow_features_are_declared() {
        assert!(REQUIRED_FEATURES.contains(&"male_coupler_nests"));
        assert!(REQUIRED_FEATURES.contains(&"female_coupler_nests"));
        assert!(REQUIRED_FEATURES.contains(&"cycle_count_witness_dial_lands"));
        assert!(REQUIRED_FEATURES.contains(&"insertion_force_gauge_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"leak_pressure_decay_witness_ports"));
        assert!(REQUIRED_FEATURES.contains(&"cap_plug_parks"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_bulkhead"));
    }

    #[test]
    fn layout_stays_on_deck_and_keeps_validation_clearances() {
        assert_layout();
    }
}
