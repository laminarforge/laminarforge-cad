use vcad::{centered_cube, centered_cylinder, Part};

// Pod/skid-side inline sensor service module for one sealed culture module.
//
// Intent:
// - Keep wetted sensing, pressure monitoring, bypass, purge, and cables outside
//   the sealed culture volume while preserving one direct service path.
// - Provide a removable flow cartridge, pressure tap/transducer pockets, bubble
//   optical fork, pH/DO optical reader bay, valve block, cable strain relief,
//   and mount features for the culture module service skid.
// - Model service geometry and packaging envelopes; material selection,
//   biocompatibility, sterilization, and sensor validation stay separate gates.
//
// Exports:
//   output/inline_sensor_service_module_baseplate.stl
//   output/inline_sensor_service_module_manifold.stl
//   output/inline_sensor_service_module_sensor_cartridge.stl
//   output/inline_sensor_service_module_optical_bay.stl
//   output/inline_sensor_service_module_valve_block.stl
//   output/inline_sensor_service_module_cable_bracket.stl
//   output/inline_sensor_service_module_assembly.stl

const MODULE_X: f64 = 420.0;
const MODULE_Y: f64 = 210.0;
const BASE_Z: f64 = 14.0;
const MANIFOLD_X: f64 = 330.0;
const MANIFOLD_Y: f64 = 48.0;
const MANIFOLD_Z: f64 = 38.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.6;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const BYPASS_OFFSET_Y: f64 = -17.0;
const SENSOR_RAIL_Y: f64 = 56.0;
const CABLE_RAIL_Y: f64 = -78.0;

fn main() {
    let baseplate = baseplate();
    baseplate
        .write_stl("output/inline_sensor_service_module_baseplate.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_baseplate.stl");

    let manifold = inline_manifold();
    manifold
        .write_stl("output/inline_sensor_service_module_manifold.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_manifold.stl");

    let cartridge = flow_sensor_cartridge();
    cartridge
        .write_stl("output/inline_sensor_service_module_sensor_cartridge.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_sensor_cartridge.stl");

    let optical_bay = optical_reader_bay();
    optical_bay
        .write_stl("output/inline_sensor_service_module_optical_bay.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_optical_bay.stl");

    let valve_block = valve_bypass_purge_block();
    valve_block
        .write_stl("output/inline_sensor_service_module_valve_block.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_valve_block.stl");

    let cable_bracket = cable_strain_relief_bracket();
    cable_bracket
        .write_stl("output/inline_sensor_service_module_cable_bracket.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_cable_bracket.stl");

    let assembly = baseplate
        + manifold.translate(0.0, 0.0, BASE_Z / 2.0 + MANIFOLD_Z / 2.0)
        + cartridge.translate(-66.0, SENSOR_RAIL_Y, BASE_Z / 2.0 + 18.0)
        + optical_bay.translate(86.0, SENSOR_RAIL_Y, BASE_Z / 2.0 + 22.0)
        + valve_block.translate(116.0, -18.0, BASE_Z / 2.0 + 22.0)
        + cable_bracket.translate(0.0, CABLE_RAIL_Y, BASE_Z / 2.0 + 18.0);

    assembly
        .write_stl("output/inline_sensor_service_module_assembly.stl")
        .unwrap();
    println!("Exported: output/inline_sensor_service_module_assembly.stl");

    println!(
        "Inline sensor service module: {:.0}mm x {:.0}mm skid footprint, {:.0}mm inline manifold, {:.1}mm tube OD path with pressure taps, flow cartridge, bubble fork, pH/DO bay, bypass/purge valves, and cable strain relief.",
        MODULE_X, MODULE_Y, MANIFOLD_X, TUBE_OD
    );
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "inline_sensor_service_module_baseplate",
        MODULE_X,
        MODULE_Y,
        BASE_Z,
    );

    let drip_sump = centered_cube(
        "inline_sensor_service_module_drip_sump",
        MANIFOLD_X + 24.0,
        18.0,
        8.0,
    )
    .translate(0.0, -34.0, BASE_Z / 2.0 - 3.0);
    let purge_drain = centered_cylinder("inline_sensor_service_module_purge_drain", 5.0, 34.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(MODULE_X / 2.0 - 58.0, -52.0, 0.0);

    let cartridge_slide_clearance = centered_cube(
        "inline_sensor_cartridge_slide_clearance",
        270.0,
        52.0,
        BASE_Z + 2.0,
    )
    .translate(4.0, SENSOR_RAIL_Y, 0.0);

    let mut mount_holes = Part::empty("inline_sensor_baseplate_mount_holes");
    for (i, (x, y)) in [
        (-(MODULE_X / 2.0 - 28.0), -(MODULE_Y / 2.0 - 26.0)),
        (MODULE_X / 2.0 - 28.0, -(MODULE_Y / 2.0 - 26.0)),
        (-(MODULE_X / 2.0 - 28.0), MODULE_Y / 2.0 - 26.0),
        (MODULE_X / 2.0 - 28.0, MODULE_Y / 2.0 - 26.0),
        (0.0, -(MODULE_Y / 2.0 - 26.0)),
        (0.0, MODULE_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("inline_sensor_baseplate_m5_slot_{i}"),
                5.4 / 2.0,
                18.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("inline_sensor_baseplate_m5_slot_relief_{i}"),
                18.0,
                5.4,
                18.0,
            )
            .translate(*x, *y, 0.0);
    }

    let mut locator_bosses = Part::empty("inline_sensor_module_locator_bosses");
    for (i, x) in [-170.0, 170.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("inline_sensor_module_locator_boss_{i}"),
            11.0,
            8.0,
            32,
        )
        .translate(*x, 0.0, BASE_Z / 2.0 + 4.0);
        let socket = centered_cylinder(
            format!("inline_sensor_module_locator_socket_{i}"),
            4.0 / 2.0,
            10.0,
            24,
        )
        .translate(*x, 0.0, BASE_Z / 2.0 + 4.0);
        locator_bosses = locator_bosses + (boss - socket);
    }

    deck - drip_sump - purge_drain - cartridge_slide_clearance - mount_holes
        + module_side_rails()
        + cartridge_slide_rails()
        + locator_bosses
        + inlet_outlet_bulkhead_tabs()
}

fn module_side_rails() -> Part {
    let left = centered_cube("inline_sensor_left_guard_rail", 16.0, MODULE_Y - 36.0, 24.0)
        .translate(-(MODULE_X / 2.0 - 20.0), 0.0, BASE_Z / 2.0 + 12.0);
    let right = centered_cube(
        "inline_sensor_right_guard_rail",
        16.0,
        MODULE_Y - 36.0,
        24.0,
    )
    .translate(MODULE_X / 2.0 - 20.0, 0.0, BASE_Z / 2.0 + 12.0);
    let rear = centered_cube(
        "inline_sensor_rear_locator_rail",
        MODULE_X - 60.0,
        16.0,
        24.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 20.0, BASE_Z / 2.0 + 12.0);

    left + right + rear
}

fn cartridge_slide_rails() -> Part {
    let left = centered_cube("inline_sensor_left_cartridge_slide_rail", 274.0, 10.0, 10.0)
        .translate(4.0, SENSOR_RAIL_Y - 31.0, BASE_Z / 2.0 + 5.0);
    let right = centered_cube(
        "inline_sensor_right_cartridge_slide_rail",
        274.0,
        10.0,
        10.0,
    )
    .translate(4.0, SENSOR_RAIL_Y + 31.0, BASE_Z / 2.0 + 5.0);
    let rear_stop = centered_cube("inline_sensor_cartridge_slide_rear_stop", 12.0, 72.0, 14.0)
        .translate(148.0, SENSOR_RAIL_Y, BASE_Z / 2.0 + 7.0);

    left + right + rear_stop
}

fn inlet_outlet_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("inline_sensor_inlet_outlet_bulkhead_tabs");
    for (i, x) in [-198.0, 198.0].iter().enumerate() {
        let tab = centered_cube(format!("inline_sensor_bulkhead_tab_{i}"), 34.0, 42.0, 18.0)
            .translate(*x, 0.0, BASE_Z / 2.0 + 9.0);
        let tube_clearance = centered_cylinder(
            format!("inline_sensor_bulkhead_tube_clearance_{i}"),
            8.0 / 2.0,
            46.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, BASE_Z / 2.0 + 9.0);
        let mount_hole = centered_cylinder(
            format!("inline_sensor_bulkhead_tab_mount_{i}"),
            3.4 / 2.0,
            20.0,
            20,
        )
        .translate(*x, 13.0, BASE_Z / 2.0 + 9.0);
        tabs = tabs + (tab - tube_clearance - mount_hole);
    }
    tabs
}

fn inline_manifold() -> Part {
    let body = centered_cube(
        "inline_sensor_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let top_land = centered_cube(
        "inline_sensor_manifold_top_label_land",
        MANIFOLD_X - 28.0,
        4.0,
        5.0,
    )
    .translate(0.0, -(MANIFOLD_Y / 2.0 + 2.0), MANIFOLD_Z / 2.0 - 5.0);

    let main_bore = centered_cylinder(
        "inline_sensor_main_fluid_bore",
        FLUID_BORE_D / 2.0,
        MANIFOLD_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 0.0);

    let bypass_bore = centered_cylinder(
        "inline_sensor_bypass_fluid_bore",
        FLUID_BORE_D / 2.0,
        176.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(18.0, BYPASS_OFFSET_Y, 0.0);

    let bypass_in = centered_cylinder(
        "inline_sensor_bypass_inlet_cross_bore",
        FLUID_BORE_D / 2.0,
        26.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-70.0, BYPASS_OFFSET_Y / 2.0, 0.0);
    let bypass_out = centered_cylinder(
        "inline_sensor_bypass_outlet_cross_bore",
        FLUID_BORE_D / 2.0,
        26.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(106.0, BYPASS_OFFSET_Y / 2.0, 0.0);

    let mut taps = Part::empty("inline_sensor_pressure_tap_cuts");
    let mut pockets = Part::empty("inline_sensor_pressure_transducer_pockets");
    for (i, x) in [-118.0, -8.0, 118.0].iter().enumerate() {
        let tap = centered_cylinder(
            format!("inline_sensor_pressure_tap_bore_{i}"),
            2.3 / 2.0,
            MANIFOLD_Z + 8.0,
            20,
        )
        .translate(*x, 0.0, 0.0);
        let pocket = centered_cube(
            format!("inline_sensor_pressure_transducer_pocket_{i}"),
            42.0,
            28.0,
            10.0,
        )
        .translate(*x, 4.0, MANIFOLD_Z / 2.0 - 4.0);
        let boss = centered_cylinder(
            format!("inline_sensor_pressure_transducer_boss_{i}"),
            14.0,
            8.0,
            40,
        )
        .translate(*x, 0.0, MANIFOLD_Z / 2.0 + 4.0);
        let boss_recess = centered_cylinder(
            format!("inline_sensor_pressure_transducer_o_ring_recess_{i}"),
            8.0,
            9.0,
            32,
        )
        .translate(*x, 0.0, MANIFOLD_Z / 2.0 + 4.0);

        taps = taps + tap + pocket + boss_recess;
        pockets = pockets + boss;
    }

    let cartridge_window = centered_cube(
        "inline_sensor_manifold_flow_cartridge_window",
        90.0,
        18.0,
        MANIFOLD_Z + 2.0,
    )
    .translate(-66.0, MANIFOLD_Y / 2.0 - 9.0, 0.0);
    let optical_window = centered_cube(
        "inline_sensor_manifold_optical_window",
        112.0,
        18.0,
        MANIFOLD_Z + 2.0,
    )
    .translate(86.0, MANIFOLD_Y / 2.0 - 9.0, 0.0);

    body + pockets + top_land
        - main_bore
        - bypass_bore
        - bypass_in
        - bypass_out
        - taps
        - cartridge_window
        - optical_window
}

fn flow_sensor_cartridge() -> Part {
    let body = centered_cube("inline_sensor_flow_cartridge_body", 106.0, 52.0, 30.0);
    let tube_channel = centered_cylinder(
        "inline_sensor_flow_cartridge_tube_channel",
        FLUID_BORE_D / 2.0,
        112.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 0.0);
    let sensor_pocket =
        centered_cube("inline_sensor_flow_meter_pocket", 52.0, 34.0, 18.0).translate(0.0, 0.0, 5.0);
    let clear_view_window =
        centered_cube("inline_sensor_flow_view_window", 42.0, 56.0, 12.0).translate(0.0, 0.0, 9.0);
    let keyed_notch = centered_cube("inline_sensor_flow_cartridge_key_notch", 18.0, 18.0, 32.0)
        .translate(-44.0, 20.0, 0.0);

    let latch_left =
        latch_ear("inline_sensor_flow_cartridge_latch_left").translate(-42.0, -36.0, 0.0);
    let latch_right =
        latch_ear("inline_sensor_flow_cartridge_latch_right").translate(42.0, -36.0, 0.0);

    body + latch_left + latch_right - tube_channel - sensor_pocket - clear_view_window - keyed_notch
}

fn latch_ear(name: &str) -> Part {
    let ear = centered_cube(format!("{name}_ear"), 24.0, 22.0, 12.0);
    let screw = centered_cylinder(format!("{name}_m3_clearance"), 3.4 / 2.0, 14.0, 20)
        .translate(0.0, 0.0, 0.0);
    ear - screw
}

fn optical_reader_bay() -> Part {
    let bay = centered_cube("inline_sensor_optical_reader_bay_body", 132.0, 64.0, 34.0);
    let tube_channel = centered_cylinder(
        "inline_sensor_optical_reader_tube_channel",
        FLUID_BORE_D / 2.0,
        140.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let bubble_fork = bubble_optical_fork().translate(-42.0, 0.0, 20.0);

    let ph_pocket = centered_cylinder("inline_sensor_ph_optical_puck_pocket", 18.0 / 2.0, 18.0, 40)
        .translate(18.0, -12.0, 12.0);
    let do_pocket = centered_cylinder("inline_sensor_do_optical_puck_pocket", 18.0 / 2.0, 18.0, 40)
        .translate(50.0, -12.0, 12.0);
    let reader_window = centered_cube("inline_sensor_ph_do_reader_window", 68.0, 22.0, 18.0)
        .translate(34.0, -12.0, 8.0);
    let fiber_trench = centered_cube("inline_sensor_ph_do_fiber_trench", 94.0, 7.0, 9.0)
        .translate(32.0, 27.0, 9.0);
    let cable_slot = centered_cube("inline_sensor_optical_bay_cable_slot", 38.0, 12.0, 20.0)
        .translate(58.0, 31.0, 6.0);

    let cover_land = centered_cube("inline_sensor_optical_bay_cover_land", 118.0, 4.0, 5.0)
        .translate(0.0, -34.0, 16.0);

    bay + bubble_fork + cover_land
        - tube_channel
        - ph_pocket
        - do_pocket
        - reader_window
        - fiber_trench
        - cable_slot
}

fn bubble_optical_fork() -> Part {
    let base = centered_cube("inline_sensor_bubble_fork_base", 58.0, 42.0, 10.0);
    let led_arm = centered_cube("inline_sensor_bubble_fork_led_arm", 14.0, 12.0, 42.0)
        .translate(0.0, -17.0, 21.0);
    let detector_arm = centered_cube("inline_sensor_bubble_fork_detector_arm", 14.0, 12.0, 42.0)
        .translate(0.0, 17.0, 21.0);
    let tube_gap = centered_cylinder("inline_sensor_bubble_fork_tube_gap", 7.2 / 2.0, 64.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, 18.0);
    let optical_path = centered_cylinder(
        "inline_sensor_bubble_fork_optical_path",
        3.2 / 2.0,
        42.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 18.0);
    let wire_slots = centered_cube("inline_sensor_bubble_fork_led_wire_slot", 8.0, 16.0, 8.0)
        .translate(0.0, -21.0, 38.0)
        + centered_cube(
            "inline_sensor_bubble_fork_detector_wire_slot",
            8.0,
            16.0,
            8.0,
        )
        .translate(0.0, 21.0, 38.0);

    base + led_arm + detector_arm - tube_gap - optical_path - wire_slots
}

fn valve_bypass_purge_block() -> Part {
    let body = centered_cube("inline_sensor_valve_bypass_purge_body", 118.0, 78.0, 34.0);
    let main_channel = centered_cylinder(
        "inline_sensor_valve_main_channel",
        FLUID_BORE_D / 2.0,
        126.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    let bypass_channel = centered_cylinder(
        "inline_sensor_valve_bypass_channel",
        FLUID_BORE_D / 2.0,
        96.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, BYPASS_OFFSET_Y, 0.0);

    let mut valve_seats = Part::empty("inline_sensor_valve_seat_cuts");
    let mut valve_caps = Part::empty("inline_sensor_valve_cap_bosses");
    for (i, (x, y)) in [(-38.0, 0.0), (0.0, BYPASS_OFFSET_Y), (38.0, 0.0)]
        .iter()
        .enumerate()
    {
        valve_seats = valve_seats
            + centered_cylinder(
                format!("inline_sensor_valve_stem_bore_{i}"),
                4.0 / 2.0,
                40.0,
                24,
            )
            .translate(*x, *y, 5.0)
            + centered_cube(
                format!("inline_sensor_valve_body_pocket_{i}"),
                24.0,
                24.0,
                12.0,
            )
            .translate(*x, *y, 14.0);
        valve_caps = valve_caps
            + centered_cylinder(format!("inline_sensor_valve_cap_boss_{i}"), 12.0, 8.0, 32)
                .translate(*x, *y, 21.0);
    }

    let purge_port = centered_cylinder("inline_sensor_purge_port", 4.0 / 2.0, 50.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(48.0, -35.0, -4.0);
    let purge_cup = centered_cube("inline_sensor_purge_cup_recess", 32.0, 20.0, 10.0)
        .translate(48.0, -28.0, -9.0);
    let bypass_bridge_a = centered_cylinder(
        "inline_sensor_valve_bypass_bridge_a",
        FLUID_BORE_D / 2.0,
        24.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-38.0, BYPASS_OFFSET_Y / 2.0, 0.0);
    let bypass_bridge_b = centered_cylinder(
        "inline_sensor_valve_bypass_bridge_b",
        FLUID_BORE_D / 2.0,
        24.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(38.0, BYPASS_OFFSET_Y / 2.0, 0.0);

    body + valve_caps
        - main_channel
        - bypass_channel
        - bypass_bridge_a
        - bypass_bridge_b
        - valve_seats
        - purge_port
        - purge_cup
}

fn cable_strain_relief_bracket() -> Part {
    let body = centered_cube("inline_sensor_cable_strain_relief_body", 286.0, 28.0, 28.0);
    let connector_bay = centered_cube("inline_sensor_board_connector_bay", 84.0, 32.0, 18.0)
        .translate(104.0, 0.0, 3.0);
    let service_loop_trough = centered_cube("inline_sensor_service_loop_trough", 150.0, 30.0, 12.0)
        .translate(-48.0, 0.0, 6.0);

    let mut cable_cuts = Part::empty("inline_sensor_cable_comb_cuts");
    for (i, x) in [-118.0, -92.0, -66.0, -40.0, -14.0, 12.0, 38.0, 64.0]
        .iter()
        .enumerate()
    {
        cable_cuts = cable_cuts
            + centered_cylinder(
                format!("inline_sensor_cable_gland_{i}"),
                4.2 / 2.0,
                32.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0)
            + centered_cube(format!("inline_sensor_cable_top_slot_{i}"), 5.4, 32.0, 16.0)
                .translate(*x, 0.0, 8.0);
    }

    let mut tie_mounts = Part::empty("inline_sensor_cable_tie_mounts");
    for (i, x) in [-128.0, -72.0, -16.0, 40.0, 96.0, 132.0].iter().enumerate() {
        let tie = centered_cube(
            format!("inline_sensor_cable_tie_bridge_{i}"),
            24.0,
            10.0,
            8.0,
        )
        .translate(*x, -21.0, 0.0);
        let tie_slot = centered_cube(format!("inline_sensor_cable_tie_slot_{i}"), 16.0, 12.0, 3.0)
            .translate(*x, -21.0, 0.0);
        tie_mounts = tie_mounts + (tie - tie_slot);
    }

    body + tie_mounts - connector_bay - service_loop_trough - cable_cuts
}
