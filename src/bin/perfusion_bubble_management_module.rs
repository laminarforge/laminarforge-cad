use vcad::{centered_cube, centered_cylinder, Part};

// Automated perfusion bubble-management module for scaled tissue-chip culture.
//
// Intent:
// - Preserve closed sterile media paths through bulkhead and cartridge interfaces.
// - Remove or divert bubbles before tissue-chip inlets using upstream debubbler
//   chambers, optical bubble forks, pressure-relief bypass routing, and waste
//   diversion.
// - Present equal-length tubing datum combs, low-dead-volume service valves, a
//   leak tray, and robotic service keepout gauges for integration into a larger
//   culture workcell.
//
// This is a packaging/architecture model. Wetted material selection, membrane
// selection, optical calibration, sterility validation, and biological testing
// remain separate gates.

const OUTPUTS: &[&str] = &[
    "output/perfusion_bubble_management_module_base_leak_tray.stl",
    "output/perfusion_bubble_management_module_sterile_interface_manifold.stl",
    "output/perfusion_bubble_management_module_upstream_debubbler.stl",
    "output/perfusion_bubble_management_module_optical_sensor_blocks.stl",
    "output/perfusion_bubble_management_module_pressure_relief_bypass.stl",
    "output/perfusion_bubble_management_module_tubing_datum_comb.stl",
    "output/perfusion_bubble_management_module_valve_service_area.stl",
    "output/perfusion_bubble_management_module_waste_diversion.stl",
    "output/perfusion_bubble_management_module_robot_service_keepouts.stl",
    "output/perfusion_bubble_management_module_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "closed_sterile_fluid_path_interfaces",
    "upstream_bubble_trap_debubbler",
    "optical_bubble_sensor_blocks",
    "pressure_relief_bypass",
    "equal_length_tubing_datum_comb",
    "low_dead_volume_valve_service_area",
    "waste_diversion",
    "leak_tray",
    "robotic_service_keepouts",
];

const LANES: usize = 4;
const CLOSED_PATH_PORTS_PER_LANE: usize = 2;
const OPTICAL_SENSOR_BLOCKS_PER_LANE: usize = 2;
const RELIEF_VALVES: usize = LANES;
const LOW_DEAD_VOLUME_VALVES: usize = LANES * 2;
const TUBING_DATUM_SLOTS: usize = LANES * 2;
const ROBOT_KEEP_OUT_WINDOWS: usize = 3;

const MODULE_X: f64 = 640.0;
const MODULE_Y: f64 = 340.0;
const BASE_Z: f64 = 18.0;
const LEAK_BASIN_X: f64 = MODULE_X - 64.0;
const LEAK_BASIN_Y: f64 = MODULE_Y - 64.0;
const LANE_PITCH_X: f64 = 118.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const INTERFACE_X: f64 = 552.0;
const INTERFACE_Y: f64 = 64.0;
const INTERFACE_Z: f64 = 42.0;
const INTERFACE_PORT_ROW_Y: f64 = 18.0;

const DEBUBBLER_X: f64 = 510.0;
const DEBUBBLER_Y: f64 = 96.0;
const DEBUBBLER_Z: f64 = 84.0;
const DEBUBBLER_CHAMBER_D: f64 = 36.0;
const DEBUBBLER_CHAMBER_Z: f64 = 76.0;
const MEMBRANE_WINDOW_D: f64 = 28.0;

const SENSOR_BLOCK_X: f64 = 58.0;
const SENSOR_BLOCK_Y: f64 = 42.0;
const SENSOR_BLOCK_Z: f64 = 58.0;
const SENSOR_STAGE_PITCH_Y: f64 = 52.0;

const BYPASS_X: f64 = 526.0;
const BYPASS_Y: f64 = 82.0;
const BYPASS_Z: f64 = 46.0;
const BYPASS_OFFSET_Y: f64 = -23.0;

const COMB_X: f64 = 566.0;
const COMB_Y: f64 = 70.0;
const COMB_Z: f64 = 34.0;
const COMB_SLOT_PITCH_X: f64 = 61.0;
const LOOP_POST_D: f64 = 12.0;

const VALVE_AREA_X: f64 = 500.0;
const VALVE_AREA_Y: f64 = 120.0;
const VALVE_AREA_Z: f64 = 48.0;
const VALVE_ROW_Y: f64 = 24.0;

const WASTE_X: f64 = 330.0;
const WASTE_Y: f64 = 120.0;
const WASTE_Z: f64 = 58.0;

const ROBOT_KEEP_OUT_X: f64 = 570.0;
const ROBOT_KEEP_OUT_Y: f64 = 162.0;
const ROBOT_KEEP_OUT_Z: f64 = 118.0;
const ROBOT_GRIPPER_CLEARANCE_Z: f64 = 86.0;

fn main() {
    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let interface = sterile_interface_manifold();
    export(&interface, OUTPUTS[1]);

    let debubbler = upstream_debubbler();
    export(&debubbler, OUTPUTS[2]);

    let sensors = optical_sensor_blocks();
    export(&sensors, OUTPUTS[3]);

    let bypass = pressure_relief_bypass();
    export(&bypass, OUTPUTS[4]);

    let comb = equal_length_tubing_datum_comb();
    export(&comb, OUTPUTS[5]);

    let valves = low_dead_volume_valve_service_area();
    export(&valves, OUTPUTS[6]);

    let waste = waste_diversion();
    export(&waste, OUTPUTS[7]);

    let keepouts = robotic_service_keepouts();
    export(&keepouts, OUTPUTS[8]);

    let assembly = base
        + interface.translate(0.0, 103.0, BASE_Z / 2.0 + INTERFACE_Z / 2.0)
        + debubbler.translate(0.0, 42.0, BASE_Z / 2.0 + DEBUBBLER_Z / 2.0)
        + sensors.translate(0.0, -44.0, BASE_Z / 2.0 + SENSOR_BLOCK_Z / 2.0)
        + bypass.translate(0.0, -102.0, BASE_Z / 2.0 + BYPASS_Z / 2.0)
        + comb.translate(0.0, -154.0, BASE_Z / 2.0 + COMB_Z / 2.0)
        + valves.translate(0.0, -7.0, BASE_Z / 2.0 + VALVE_AREA_Z / 2.0)
        + waste.translate(144.0, -83.0, BASE_Z / 2.0 + WASTE_Z / 2.0)
        + keepouts.translate(0.0, -9.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);

    export(&assembly, OUTPUTS[9]);

    println!(
        "Perfusion bubble-management module: {:.0}mm x {:.0}mm leak-tray footprint, {} tissue-chip lanes, {} sterile ports, {} upstream debubbler chambers, {} optical forks, {} relief valves, {} equal-length tubing datum slots, {} low-dead-volume valves, {} robot keepout windows, and {} required feature groups.",
        MODULE_X,
        MODULE_Y,
        LANES,
        LANES * CLOSED_PATH_PORTS_PER_LANE,
        LANES,
        LANES * OPTICAL_SENSOR_BLOCKS_PER_LANE,
        RELIEF_VALVES,
        TUBING_DATUM_SLOTS,
        LOW_DEAD_VOLUME_VALVES,
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
        "perfusion_bubble_module_base_deck",
        MODULE_X,
        MODULE_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "perfusion_bubble_module_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder(
        "perfusion_bubble_module_leak_tray_drain",
        8.0 / 2.0,
        48.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MODULE_X / 2.0 - 72.0, -(MODULE_Y / 2.0 - 20.0), 0.0);
    let waste_pocket = centered_cube(
        "perfusion_bubble_module_waste_diversion_pocket",
        WASTE_X + 22.0,
        WASTE_Y + 28.0,
        8.0,
    )
    .translate(144.0, -83.0, BASE_Z / 2.0 - 3.0);

    let mut mount_slots = Part::empty("perfusion_bubble_module_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let bore = centered_cylinder(
            format!("perfusion_bubble_module_m5_mount_bore_{i}"),
            5.4 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("perfusion_bubble_module_m5_mount_slot_{i}"),
            18.0,
            5.6,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        mount_slots = mount_slots + bore + slot;
    }

    deck - basin - drain - waste_pocket - mount_slots
        + leak_tray_rims()
        + module_locator_bosses()
        + cassette_hardstops()
}

fn leak_tray_rims() -> Part {
    let left = centered_cube(
        "perfusion_bubble_module_left_leak_tray_rim",
        16.0,
        MODULE_Y,
        28.0,
    )
    .translate(-(MODULE_X / 2.0 - 8.0), 0.0, BASE_Z / 2.0 + 14.0);
    let right = centered_cube(
        "perfusion_bubble_module_right_leak_tray_rim",
        16.0,
        MODULE_Y,
        28.0,
    )
    .translate(MODULE_X / 2.0 - 8.0, 0.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "perfusion_bubble_module_rear_leak_tray_rim",
        MODULE_X,
        16.0,
        28.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 8.0, BASE_Z / 2.0 + 14.0);
    let front = centered_cube(
        "perfusion_bubble_module_front_low_spill_lip",
        MODULE_X - 96.0,
        12.0,
        18.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 - 6.0), BASE_Z / 2.0 + 9.0);

    left + right + rear + front
}

fn module_locator_bosses() -> Part {
    let mut bosses = Part::empty("perfusion_bubble_module_locator_bosses");
    for (i, x) in [-250.0, -84.0, 84.0, 250.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("perfusion_bubble_module_locator_boss_{i}"),
            9.0,
            7.0,
            28,
        )
        .translate(*x, 142.0, BASE_Z / 2.0 + 3.5);
        let pilot = centered_cylinder(
            format!("perfusion_bubble_module_locator_pilot_{i}"),
            3.0 / 2.0,
            9.0,
            20,
        )
        .translate(*x, 142.0, BASE_Z / 2.0 + 3.5);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn cassette_hardstops() -> Part {
    let mut stops = Part::empty("perfusion_bubble_module_cassette_hardstops");
    for (i, x) in [-282.0, -188.0, 188.0, 282.0].iter().enumerate() {
        stops = stops
            + centered_cube(
                format!("perfusion_bubble_module_cassette_hardstop_{i}"),
                18.0,
                24.0,
                24.0,
            )
            .translate(*x, -149.0, BASE_Z / 2.0 + 12.0);
    }
    stops
}

fn sterile_interface_manifold() -> Part {
    let body = centered_cube(
        "perfusion_bubble_module_sterile_interface_body",
        INTERFACE_X,
        INTERFACE_Y,
        INTERFACE_Z,
    );

    let mut bores = Part::empty("perfusion_bubble_module_sterile_interface_bores");
    let mut bosses = Part::empty("perfusion_bubble_module_sterile_interface_bulkhead_bosses");
    for lane in 0..LANES {
        let x = lane_x(lane);
        for (row, y) in [-INTERFACE_PORT_ROW_Y, INTERFACE_PORT_ROW_Y]
            .iter()
            .enumerate()
        {
            let port_index = lane * CLOSED_PATH_PORTS_PER_LANE + row;
            bores = bores
                + centered_cylinder(
                    format!("perfusion_bubble_module_closed_path_port_bore_{port_index}"),
                    FLUID_BORE_D / 2.0,
                    INTERFACE_Y + 18.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, *y, 0.0)
                + centered_cylinder(
                    format!("perfusion_bubble_module_bulkhead_face_relief_{port_index}"),
                    10.0 / 2.0,
                    8.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y.signum() * (INTERFACE_Y / 2.0 + 1.0), 0.0);
            bosses = bosses
                + centered_cylinder(
                    format!("perfusion_bubble_module_closed_sterile_bulkhead_boss_{port_index}"),
                    13.0 / 2.0,
                    10.0,
                    32,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y.signum() * (INTERFACE_Y / 2.0 + 5.0), 0.0);
        }
    }

    let mut clamp_tabs = Part::empty("perfusion_bubble_module_interface_clamp_tabs");
    for (i, x) in [-245.0, 245.0].iter().enumerate() {
        let tab = centered_cube(
            format!("perfusion_bubble_module_interface_clamp_tab_{i}"),
            38.0,
            30.0,
            16.0,
        )
        .translate(*x, 0.0, -(INTERFACE_Z / 2.0 + 5.0));
        let screw = centered_cylinder(
            format!("perfusion_bubble_module_interface_clamp_screw_{i}"),
            4.4 / 2.0,
            18.0,
            22,
        )
        .translate(*x, 0.0, -(INTERFACE_Z / 2.0 + 5.0));
        clamp_tabs = clamp_tabs + (tab - screw);
    }

    body + bosses + clamp_tabs - bores - interface_lane_windows()
}

fn interface_lane_windows() -> Part {
    let mut windows = Part::empty("perfusion_bubble_module_interface_lane_windows");
    for lane in 0..LANES {
        let x = lane_x(lane);
        windows = windows
            + centered_cube(
                format!("perfusion_bubble_module_interface_lane_trace_window_{lane}"),
                34.0,
                8.0,
                12.0,
            )
            .translate(x, 0.0, INTERFACE_Z / 2.0 - 7.0);
    }
    windows
}

fn upstream_debubbler() -> Part {
    let spine = centered_cube(
        "perfusion_bubble_module_debubbler_spine",
        DEBUBBLER_X,
        DEBUBBLER_Y,
        DEBUBBLER_Z,
    );
    let inlet_gallery = centered_cylinder(
        "perfusion_bubble_module_debubbler_inlet_gallery",
        FLUID_BORE_D / 2.0,
        DEBUBBLER_X + 12.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -26.0, -18.0);
    let outlet_gallery = centered_cylinder(
        "perfusion_bubble_module_debubbler_outlet_gallery",
        FLUID_BORE_D / 2.0,
        DEBUBBLER_X + 12.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 26.0, -18.0);

    let mut chamber_cuts = Part::empty("perfusion_bubble_module_debubbler_chamber_cuts");
    let mut visible_cartridges = Part::empty("perfusion_bubble_module_debubbler_chamber_shells");
    for lane in 0..LANES {
        let x = lane_x(lane);
        chamber_cuts = chamber_cuts
            + stepped_chamber_cut(lane).translate(x, 0.0, 0.0)
            + centered_cylinder(
                format!("perfusion_bubble_module_debubbler_inlet_riser_{lane}"),
                FLUID_BORE_D / 2.0,
                66.0,
                26,
            )
            .translate(x, -26.0, -18.0)
            + centered_cylinder(
                format!("perfusion_bubble_module_debubbler_outlet_riser_{lane}"),
                FLUID_BORE_D / 2.0,
                66.0,
                26,
            )
            .translate(x, 26.0, -18.0)
            + centered_cylinder(
                format!("perfusion_bubble_module_debubbler_membrane_window_{lane}"),
                MEMBRANE_WINDOW_D / 2.0,
                12.0,
                40,
            )
            .translate(x, 0.0, DEBUBBLER_Z / 2.0 - 3.0);
        visible_cartridges = visible_cartridges
            + centered_cylinder(
                format!("perfusion_bubble_module_debubbler_transparent_cartridge_land_{lane}"),
                DEBUBBLER_CHAMBER_D / 2.0 + 5.0,
                8.0,
                44,
            )
            .translate(x, 0.0, DEBUBBLER_Z / 2.0 + 4.0)
            + centered_cube(
                format!("perfusion_bubble_module_debubbler_internal_partition_rib_{lane}"),
                5.0,
                52.0,
                48.0,
            )
            .translate(x, 0.0, 3.0);
    }

    spine + visible_cartridges - inlet_gallery - outlet_gallery - chamber_cuts
        + debubbler_mount_lugs()
}

fn stepped_chamber_cut(lane: usize) -> Part {
    let lower = centered_cylinder(
        format!("perfusion_bubble_module_debubbler_lower_cavity_{lane}"),
        DEBUBBLER_CHAMBER_D / 2.0,
        DEBUBBLER_CHAMBER_Z * 0.54,
        48,
    )
    .translate(0.0, 0.0, -8.0);
    let upper = centered_cylinder(
        format!("perfusion_bubble_module_debubbler_upper_cavity_{lane}"),
        DEBUBBLER_CHAMBER_D / 2.0 - 4.0,
        DEBUBBLER_CHAMBER_Z * 0.36,
        48,
    )
    .translate(0.0, 0.0, 16.0);
    let crown = centered_cylinder(
        format!("perfusion_bubble_module_debubbler_crown_cavity_{lane}"),
        DEBUBBLER_CHAMBER_D / 2.0 - 8.0,
        DEBUBBLER_CHAMBER_Z * 0.20,
        40,
    )
    .translate(0.0, 0.0, 32.0);

    lower + upper + crown
}

fn debubbler_mount_lugs() -> Part {
    let mut lugs = Part::empty("perfusion_bubble_module_debubbler_mount_lugs");
    for (i, x) in [-242.0, 242.0].iter().enumerate() {
        let lug = centered_cube(
            format!("perfusion_bubble_module_debubbler_mount_lug_{i}"),
            34.0,
            24.0,
            16.0,
        )
        .translate(*x, 0.0, -(DEBUBBLER_Z / 2.0 - 8.0));
        let screw = centered_cylinder(
            format!("perfusion_bubble_module_debubbler_mount_screw_{i}"),
            4.4 / 2.0,
            18.0,
            22,
        )
        .translate(*x, 0.0, -(DEBUBBLER_Z / 2.0 - 8.0));
        lugs = lugs + (lug - screw);
    }
    lugs
}

fn optical_sensor_blocks() -> Part {
    let mut sensors = Part::empty("perfusion_bubble_module_optical_sensor_blocks");
    for lane in 0..LANES {
        let x = lane_x(lane);
        for stage in 0..OPTICAL_SENSOR_BLOCKS_PER_LANE {
            let y = centered_index(stage, OPTICAL_SENSOR_BLOCKS_PER_LANE, SENSOR_STAGE_PITCH_Y);
            sensors = sensors + optical_sensor_fork(lane, stage).translate(x, y, 0.0);
        }
    }
    sensors + sensor_cable_backplane()
}

fn optical_sensor_fork(lane: usize, stage: usize) -> Part {
    let base = centered_cube(
        format!("perfusion_bubble_module_optical_fork_base_{lane}_{stage}"),
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
        10.0,
    )
    .translate(0.0, 0.0, -(SENSOR_BLOCK_Z / 2.0 - 5.0));
    let led_arm = centered_cube(
        format!("perfusion_bubble_module_optical_fork_led_arm_{lane}_{stage}"),
        12.0,
        12.0,
        SENSOR_BLOCK_Z,
    )
    .translate(0.0, -16.0, 0.0);
    let detector_arm = centered_cube(
        format!("perfusion_bubble_module_optical_fork_detector_arm_{lane}_{stage}"),
        12.0,
        12.0,
        SENSOR_BLOCK_Z,
    )
    .translate(0.0, 16.0, 0.0);
    let tube_gap = centered_cylinder(
        format!("perfusion_bubble_module_optical_fork_tube_gap_{lane}_{stage}"),
        (TUBE_OD + 2.0) / 2.0,
        SENSOR_BLOCK_X + 8.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 4.0);
    let optical_axis = centered_cylinder(
        format!("perfusion_bubble_module_optical_fork_light_path_{lane}_{stage}"),
        2.8 / 2.0,
        SENSOR_BLOCK_Y + 8.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 4.0);
    let wire_slot = centered_cube(
        format!("perfusion_bubble_module_optical_fork_wire_slot_{lane}_{stage}"),
        9.0,
        SENSOR_BLOCK_Y + 8.0,
        9.0,
    )
    .translate(0.0, 0.0, SENSOR_BLOCK_Z / 2.0 - 10.0);

    base + led_arm + detector_arm - tube_gap - optical_axis - wire_slot
}

fn sensor_cable_backplane() -> Part {
    let body = centered_cube(
        "perfusion_bubble_module_sensor_cable_backplane",
        INTERFACE_X,
        16.0,
        28.0,
    )
    .translate(0.0, SENSOR_STAGE_PITCH_Y / 2.0 + 34.0, -10.0);
    let mut cable_slots = Part::empty("perfusion_bubble_module_sensor_cable_slots");
    for i in 0..LANES * OPTICAL_SENSOR_BLOCKS_PER_LANE {
        let x = centered_index(i, LANES * OPTICAL_SENSOR_BLOCKS_PER_LANE, 56.0);
        cable_slots = cable_slots
            + centered_cube(
                format!("perfusion_bubble_module_sensor_cable_slot_{i}"),
                8.0,
                18.0,
                12.0,
            )
            .translate(x, SENSOR_STAGE_PITCH_Y / 2.0 + 34.0, -2.0);
    }
    body - cable_slots
}

fn pressure_relief_bypass() -> Part {
    let body = centered_cube(
        "perfusion_bubble_module_pressure_relief_bypass_body",
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    let bypass_gallery = centered_cylinder(
        "perfusion_bubble_module_pressure_relief_waste_gallery",
        FLUID_BORE_D / 2.0,
        BYPASS_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, BYPASS_OFFSET_Y, -8.0);

    let mut channels = Part::empty("perfusion_bubble_module_pressure_relief_channel_cuts");
    let mut valve_caps = Part::empty("perfusion_bubble_module_pressure_relief_valve_caps");
    for lane in 0..LANES {
        let x = lane_x(lane);
        channels = channels
            + centered_cylinder(
                format!("perfusion_bubble_module_pressure_main_lane_{lane}"),
                FLUID_BORE_D / 2.0,
                BYPASS_Y + 12.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -8.0)
            + centered_cylinder(
                format!("perfusion_bubble_module_pressure_relief_bridge_{lane}"),
                FLUID_BORE_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BYPASS_OFFSET_Y / 2.0, -8.0)
            + centered_cylinder(
                format!("perfusion_bubble_module_pressure_relief_stem_bore_{lane}"),
                4.0 / 2.0,
                38.0,
                24,
            )
            .translate(x, BYPASS_OFFSET_Y, 8.0);
        valve_caps = valve_caps
            + centered_cylinder(
                format!("perfusion_bubble_module_pressure_relief_spring_cap_{lane}"),
                13.0,
                9.0,
                32,
            )
            .translate(x, BYPASS_OFFSET_Y, BYPASS_Z / 2.0 + 4.5);
    }

    body + valve_caps + relief_direction_tabs() - bypass_gallery - channels
}

fn relief_direction_tabs() -> Part {
    let mut tabs = Part::empty("perfusion_bubble_module_pressure_relief_direction_tabs");
    for lane in 0..LANES {
        let x = lane_x(lane);
        tabs = tabs
            + centered_cube(
                format!("perfusion_bubble_module_relief_to_waste_tab_{lane}"),
                26.0,
                8.0,
                6.0,
            )
            .translate(x, BYPASS_OFFSET_Y - 22.0, BYPASS_Z / 2.0 + 3.0);
    }
    tabs
}

fn equal_length_tubing_datum_comb() -> Part {
    let body = centered_cube(
        "perfusion_bubble_module_equal_length_comb_body",
        COMB_X,
        COMB_Y,
        COMB_Z,
    );
    let mut tube_slots = Part::empty("perfusion_bubble_module_equal_length_tube_slots");
    let mut loop_posts = Part::empty("perfusion_bubble_module_equal_length_loop_posts");
    for slot in 0..TUBING_DATUM_SLOTS {
        let x = comb_slot_x(slot);
        tube_slots = tube_slots
            + centered_cylinder(
                format!("perfusion_bubble_module_equal_length_comb_round_slot_{slot}"),
                (TUBE_OD + 1.6) / 2.0,
                COMB_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 4.0)
            + centered_cube(
                format!("perfusion_bubble_module_equal_length_comb_top_entry_{slot}"),
                TUBE_OD + 2.2,
                COMB_Y + 8.0,
                COMB_Z / 2.0,
            )
            .translate(x, 0.0, COMB_Z / 4.0);
        loop_posts = loop_posts
            + datum_loop_post(slot, x, -24.0)
            + datum_loop_post(slot + TUBING_DATUM_SLOTS, x, 24.0);
    }

    body + loop_posts + comb_mount_tabs() - tube_slots
}

fn datum_loop_post(index: usize, x: f64, y: f64) -> Part {
    let post = centered_cylinder(
        format!("perfusion_bubble_module_datum_loop_post_{index}"),
        LOOP_POST_D / 2.0,
        18.0,
        28,
    )
    .translate(x, y, COMB_Z / 2.0 + 9.0);
    let pilot = centered_cylinder(
        format!("perfusion_bubble_module_datum_loop_post_pilot_{index}"),
        2.0 / 2.0,
        20.0,
        18,
    )
    .translate(x, y, COMB_Z / 2.0 + 9.0);
    post - pilot
}

fn comb_mount_tabs() -> Part {
    let mut tabs = Part::empty("perfusion_bubble_module_comb_mount_tabs");
    for (i, x) in [-252.0, 252.0].iter().enumerate() {
        let tab = centered_cube(
            format!("perfusion_bubble_module_comb_mount_tab_{i}"),
            34.0,
            28.0,
            14.0,
        )
        .translate(*x, 0.0, -(COMB_Z / 2.0 + 5.0));
        let screw = centered_cylinder(
            format!("perfusion_bubble_module_comb_mount_screw_{i}"),
            4.4 / 2.0,
            16.0,
            22,
        )
        .translate(*x, 0.0, -(COMB_Z / 2.0 + 5.0));
        tabs = tabs + (tab - screw);
    }
    tabs
}

fn low_dead_volume_valve_service_area() -> Part {
    let tray = centered_cube(
        "perfusion_bubble_module_low_dead_volume_valve_service_tray",
        VALVE_AREA_X,
        VALVE_AREA_Y,
        VALVE_AREA_Z,
    );
    let service_cover_recess = centered_cube(
        "perfusion_bubble_module_valve_service_cover_recess",
        VALVE_AREA_X - 54.0,
        VALVE_AREA_Y - 30.0,
        8.0,
    )
    .translate(0.0, 0.0, VALVE_AREA_Z / 2.0 - 4.0);

    let mut valve_pockets = Part::empty("perfusion_bubble_module_low_dead_volume_valve_pockets");
    let mut valve_caps = Part::empty("perfusion_bubble_module_low_dead_volume_valve_cap_bosses");
    let mut flow_cuts = Part::empty("perfusion_bubble_module_low_dead_volume_flow_cuts");
    for lane in 0..LANES {
        let x = lane_x(lane);
        for row in 0..2 {
            let y = centered_index(row, 2, VALVE_ROW_Y * 2.0);
            let index = lane * 2 + row;
            valve_pockets = valve_pockets
                + centered_cube(
                    format!("perfusion_bubble_module_microvalve_service_pocket_{index}"),
                    30.0,
                    26.0,
                    16.0,
                )
                .translate(x, y, VALVE_AREA_Z / 2.0 - 8.0)
                + centered_cylinder(
                    format!("perfusion_bubble_module_microvalve_stem_clearance_{index}"),
                    4.0 / 2.0,
                    54.0,
                    24,
                )
                .translate(x, y, 6.0);
            valve_caps = valve_caps
                + centered_cylinder(
                    format!("perfusion_bubble_module_low_dead_volume_valve_cap_{index}"),
                    12.0,
                    8.0,
                    28,
                )
                .translate(x, y, VALVE_AREA_Z / 2.0 + 4.0);
            flow_cuts = flow_cuts
                + centered_cylinder(
                    format!("perfusion_bubble_module_low_dead_volume_lane_flow_{index}"),
                    FLUID_BORE_D / 2.0,
                    VALVE_AREA_Y + 12.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, -6.0);
        }
    }

    tray + valve_caps + valve_cover_latches() - service_cover_recess - valve_pockets - flow_cuts
}

fn valve_cover_latches() -> Part {
    let mut latches = Part::empty("perfusion_bubble_module_valve_cover_latches");
    for (i, x) in [-222.0, -74.0, 74.0, 222.0].iter().enumerate() {
        let bridge = centered_cube(
            format!("perfusion_bubble_module_valve_cover_latch_bridge_{i}"),
            34.0,
            10.0,
            8.0,
        )
        .translate(*x, VALVE_AREA_Y / 2.0 - 10.0, VALVE_AREA_Z / 2.0 + 4.0);
        let slot = centered_cube(
            format!("perfusion_bubble_module_valve_cover_latch_slot_{i}"),
            22.0,
            12.0,
            3.0,
        )
        .translate(*x, VALVE_AREA_Y / 2.0 - 10.0, VALVE_AREA_Z / 2.0 + 4.0);
        latches = latches + (bridge - slot);
    }
    latches
}

fn waste_diversion() -> Part {
    let tray = centered_cube(
        "perfusion_bubble_module_waste_diversion_secondary_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let sump = centered_cube(
        "perfusion_bubble_module_waste_diversion_sump",
        WASTE_X - 38.0,
        WASTE_Y - 34.0,
        18.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 9.0);
    let diversion_manifold = centered_cube(
        "perfusion_bubble_module_waste_diversion_manifold",
        WASTE_X - 70.0,
        34.0,
        34.0,
    )
    .translate(0.0, 8.0, -WASTE_Z / 2.0 + 30.0);
    let waste_gallery = centered_cylinder(
        "perfusion_bubble_module_waste_diversion_gallery",
        FLUID_BORE_D / 2.0,
        WASTE_X - 54.0,
        30,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 8.0, -WASTE_Z / 2.0 + 30.0);
    let waste_bulkhead = centered_cylinder(
        "perfusion_bubble_module_waste_bulkhead_port",
        9.5 / 2.0,
        WASTE_Y + 12.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_X / 2.0 - 44.0, 0.0, -WASTE_Z / 2.0 + 30.0);

    let mut branch_ports = Part::empty("perfusion_bubble_module_waste_diversion_branch_ports");
    for lane in 0..LANES {
        let x = centered_index(lane, LANES, 56.0) - 38.0;
        branch_ports = branch_ports
            + centered_cylinder(
                format!("perfusion_bubble_module_waste_diversion_lane_branch_{lane}"),
                FLUID_BORE_D / 2.0,
                44.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -12.0, -WASTE_Z / 2.0 + 30.0);
    }

    tray + diversion_manifold + waste_tray_grip_tabs()
        - sump
        - waste_gallery
        - waste_bulkhead
        - branch_ports
}

fn waste_tray_grip_tabs() -> Part {
    let left = centered_cube(
        "perfusion_bubble_module_waste_tray_left_pull_tab",
        44.0,
        14.0,
        18.0,
    )
    .translate(-(WASTE_X / 2.0 - 44.0), -(WASTE_Y / 2.0 - 7.0), 0.0);
    let right = centered_cube(
        "perfusion_bubble_module_waste_tray_right_pull_tab",
        44.0,
        14.0,
        18.0,
    )
    .translate(WASTE_X / 2.0 - 44.0, -(WASTE_Y / 2.0 - 7.0), 0.0);

    left + right
}

fn robotic_service_keepouts() -> Part {
    let top_frame = keepout_window_frame(
        "perfusion_bubble_module_robot_top_access_keepout",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        10.0,
    )
    .translate(0.0, 0.0, ROBOT_KEEP_OUT_Z / 2.0 - 5.0);
    let gripper_lane = keepout_window_frame(
        "perfusion_bubble_module_robot_gripper_lane_keepout",
        ROBOT_KEEP_OUT_X - 86.0,
        62.0,
        8.0,
    )
    .translate(0.0, -60.0, ROBOT_GRIPPER_CLEARANCE_Z);
    let pipette_lane = keepout_window_frame(
        "perfusion_bubble_module_robot_pipette_lane_keepout",
        ROBOT_KEEP_OUT_X - 140.0,
        42.0,
        8.0,
    )
    .translate(0.0, 54.0, ROBOT_GRIPPER_CLEARANCE_Z + 16.0);

    top_frame + gripper_lane + pipette_lane + keepout_corner_posts() + service_nozzle_gauge()
}

fn keepout_window_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);

    front + rear + left + right
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty("perfusion_bubble_module_robot_keepout_corner_posts");
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
                format!("perfusion_bubble_module_robot_keepout_post_{i}"),
                10.0,
                10.0,
                ROBOT_KEEP_OUT_Z,
            )
            .translate(*x, *y, 0.0);
    }
    posts
}

fn service_nozzle_gauge() -> Part {
    let mut gauges = Part::empty("perfusion_bubble_module_robot_service_nozzle_gauges");
    for lane in 0..LANES {
        let x = lane_x(lane);
        let collar = centered_cylinder(
            format!("perfusion_bubble_module_robot_service_nozzle_collar_{lane}"),
            18.0 / 2.0,
            6.0,
            30,
        )
        .translate(x, 58.0, ROBOT_GRIPPER_CLEARANCE_Z + 34.0);
        let clearance = centered_cylinder(
            format!("perfusion_bubble_module_robot_service_nozzle_clearance_{lane}"),
            9.0 / 2.0,
            8.0,
            24,
        )
        .translate(x, 58.0, ROBOT_GRIPPER_CLEARANCE_Z + 34.0);
        gauges = gauges + (collar - clearance);
    }
    gauges
}

fn base_mount_points() -> [(f64, f64); 6] {
    [
        (-(MODULE_X / 2.0 - 34.0), -(MODULE_Y / 2.0 - 34.0)),
        (MODULE_X / 2.0 - 34.0, -(MODULE_Y / 2.0 - 34.0)),
        (-(MODULE_X / 2.0 - 34.0), MODULE_Y / 2.0 - 34.0),
        (MODULE_X / 2.0 - 34.0, MODULE_Y / 2.0 - 34.0),
        (0.0, -(MODULE_Y / 2.0 - 34.0)),
        (0.0, MODULE_Y / 2.0 - 34.0),
    ]
}

fn lane_x(index: usize) -> f64 {
    centered_index(index, LANES, LANE_PITCH_X)
}

fn comb_slot_x(index: usize) -> f64 {
    centered_index(index, TUBING_DATUM_SLOTS, COMB_SLOT_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/perfusion_bubble_management_module_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_coverage_matches_module_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 9);
        assert!(REQUIRED_FEATURES.contains(&"closed_sterile_fluid_path_interfaces"));
        assert!(REQUIRED_FEATURES.contains(&"upstream_bubble_trap_debubbler"));
        assert!(REQUIRED_FEATURES.contains(&"optical_bubble_sensor_blocks"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_relief_bypass"));
        assert!(REQUIRED_FEATURES.contains(&"equal_length_tubing_datum_comb"));
        assert!(REQUIRED_FEATURES.contains(&"low_dead_volume_valve_service_area"));
        assert!(REQUIRED_FEATURES.contains(&"waste_diversion"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"robotic_service_keepouts"));
    }

    #[test]
    fn lane_arrays_are_symmetric_and_inside_packaging() {
        assert_eq!(lane_x(0), -lane_x(LANES - 1));
        assert_eq!(lane_x(1), -lane_x(LANES - 2));
        assert!(lane_x(0).abs() < INTERFACE_X / 2.0 - 54.0);
        assert!(lane_x(LANES - 1).abs() < DEBUBBLER_X / 2.0 - 54.0);
        assert!(lane_x(LANES - 1).abs() < BYPASS_X / 2.0 - 54.0);
    }

    #[test]
    fn closed_path_and_detection_counts_match_scaled_tissue_chip_lanes() {
        assert_eq!(LANES, 4);
        assert_eq!(LANES * CLOSED_PATH_PORTS_PER_LANE, 8);
        assert_eq!(LANES * OPTICAL_SENSOR_BLOCKS_PER_LANE, 8);
        assert_eq!(RELIEF_VALVES, LANES);
        assert_eq!(LOW_DEAD_VOLUME_VALVES, LANES * 2);
        assert_eq!(TUBING_DATUM_SLOTS, LANES * 2);
    }

    #[test]
    fn equal_length_comb_slots_fit_and_preserve_centerline_symmetry() {
        assert_eq!(comb_slot_x(0), -comb_slot_x(TUBING_DATUM_SLOTS - 1));
        assert_eq!(comb_slot_x(1), -comb_slot_x(TUBING_DATUM_SLOTS - 2));
        assert!(comb_slot_x(0).abs() < COMB_X / 2.0 - 42.0);
        assert!(COMB_SLOT_PITCH_X > TUBE_OD * 9.0);
    }

    #[test]
    fn service_geometry_preserves_robot_and_leak_clearances() {
        assert!(VALVE_AREA_X <= MODULE_X - 100.0);
        assert!(DEBUBBLER_X <= MODULE_X - 100.0);
        assert!(ROBOT_KEEP_OUT_X <= MODULE_X - 60.0);
        assert!(ROBOT_KEEP_OUT_Y <= MODULE_Y - 120.0);
        assert!(ROBOT_GRIPPER_CLEARANCE_Z > DEBUBBLER_Z);
        assert!(LEAK_BASIN_X >= INTERFACE_X);
        assert!(LEAK_BASIN_Y >= VALVE_AREA_Y + WASTE_Y);
    }
}
