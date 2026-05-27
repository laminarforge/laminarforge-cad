use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Robot tool-change and end-effector storage rack for the closed culture workcell.
//
// Intent:
// - Provide a dockable clean-side rack for the cassette gripper, scanner/camera
//   tool, pipette-free tubing connector tool, and calibration probe tool.
// - Keep clean parking pockets physically separated from used/quarantine return
//   saddles while still giving the robot a single repeatable tool-change datum.
// - Reserve explicit lands for barcode/RFID/tool ID verification, force/torque
//   datum checks, VHP/washdown cleanability gaps, drip capture, and service or
//   robot collision keepouts.
//
// This is architecture packaging CAD. The pocket geometry, datum checks, and
// ID lands are placeholders for later mechanical selection, validation, and
// robot teach-point qualification.

const OUTPUTS: &[&str] = &[
    "output/robot_tool_change_and_end_effector_rack_drip_tray.stl",
    "output/robot_tool_change_and_end_effector_rack_clean_pocket_bank.stl",
    "output/robot_tool_change_and_end_effector_rack_cassette_gripper_pocket.stl",
    "output/robot_tool_change_and_end_effector_rack_scanner_camera_pocket.stl",
    "output/robot_tool_change_and_end_effector_rack_tubing_connector_pocket.stl",
    "output/robot_tool_change_and_end_effector_rack_calibration_probe_pocket.stl",
    "output/robot_tool_change_and_end_effector_rack_clean_used_segregation.stl",
    "output/robot_tool_change_and_end_effector_rack_tool_id_lands.stl",
    "output/robot_tool_change_and_end_effector_rack_force_torque_datums.stl",
    "output/robot_tool_change_and_end_effector_rack_vhp_cleanability_keepouts.stl",
    "output/robot_tool_change_and_end_effector_rack_collision_service_keepouts.stl",
    "output/robot_tool_change_and_end_effector_rack_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "cassette_gripper_docking_pocket",
    "scanner_camera_docking_pocket",
    "pipette_free_tubing_connector_pocket",
    "calibration_probe_tool_pocket",
    "clean_used_physical_segregation",
    "barcode_rfid_tool_id_lands",
    "force_torque_datum_checks",
    "drip_leak_capture_tray",
    "vhp_cleanability_clearances",
    "collision_service_keepouts",
];

const RACK_X: f64 = 900.0;
const RACK_Y: f64 = 460.0;
const DRIP_TRAY_Z: f64 = 46.0;
const TRAY_RIM_W: f64 = 20.0;
const TRAY_RIM_Z: f64 = 54.0;
const TRAY_SUMP_DROP: f64 = 16.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 7.0;

const CLEAN_ROW_Y: f64 = 72.0;
const USED_ROW_Y: f64 = -154.0;
const TOOL_CENTERLINE_Z: f64 = DRIP_TRAY_Z;
const CLEAN_USED_BARRIER_Y: f64 = -42.0;
const CLEAN_USED_BARRIER_Z: f64 = 124.0;
const CLEAN_USED_GAP: f64 = 62.0;
const CLEAN_USED_AIR_GAP_MIN: f64 = 46.0;

const TOOL_COUNT: usize = 4;
const TOOL_ID_LAND_COUNT: usize = TOOL_COUNT;
const RFID_LAND_COUNT: usize = TOOL_COUNT;
const FORCE_DATUM_COUNT: usize = TOOL_COUNT;
const TORQUE_PIN_PAIRS: usize = TOOL_COUNT;
const USED_RETURN_SADDLE_COUNT: usize = TOOL_COUNT;

const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 30.0;
const RFID_PAD_D: f64 = 24.0;
const FORCE_PAD_D: f64 = 16.0;
const TORQUE_PIN_D: f64 = 8.0;

const MIN_WASHDOWN_GAP: f64 = 32.0;
const MIN_VHP_NOZZLE_CLEARANCE: f64 = 80.0;
const VHP_CORRIDOR_Z: f64 = 240.0;
const VHP_CORRIDOR_Y: f64 = 154.0;
const ROBOT_WRIST_CLEARANCE_Z: f64 = 310.0;
const FRONT_SERVICE_CLEARANCE: f64 = 520.0;
const SIDE_SERVICE_CLEARANCE: f64 = 180.0;
const REAR_WORKCELL_OFFSET: f64 = 84.0;
const CLOSED_WORKCELL_DECK_X: f64 = 2400.0 - 140.0;
const CLOSED_WORKCELL_DECK_Y: f64 = 1120.0 - 150.0;

#[derive(Clone, Copy)]
struct ToolPocket {
    slug: &'static str,
    width: f64,
    depth: f64,
    height: f64,
    cavity_x: f64,
    cavity_y: f64,
    cavity_depth: f64,
    x: f64,
    y: f64,
}

const POCKETS: [ToolPocket; TOOL_COUNT] = [
    ToolPocket {
        slug: "cassette_gripper",
        width: 186.0,
        depth: 168.0,
        height: 78.0,
        cavity_x: 132.0,
        cavity_y: 112.0,
        cavity_depth: 54.0,
        x: -318.0,
        y: CLEAN_ROW_Y,
    },
    ToolPocket {
        slug: "scanner_camera",
        width: 164.0,
        depth: 150.0,
        height: 74.0,
        cavity_x: 112.0,
        cavity_y: 94.0,
        cavity_depth: 48.0,
        x: -106.0,
        y: CLEAN_ROW_Y,
    },
    ToolPocket {
        slug: "tubing_connector",
        width: 172.0,
        depth: 178.0,
        height: 84.0,
        cavity_x: 116.0,
        cavity_y: 128.0,
        cavity_depth: 60.0,
        x: 112.0,
        y: CLEAN_ROW_Y,
    },
    ToolPocket {
        slug: "calibration_probe",
        width: 148.0,
        depth: 158.0,
        height: 98.0,
        cavity_x: 72.0,
        cavity_y: 120.0,
        cavity_depth: 74.0,
        x: 314.0,
        y: CLEAN_ROW_Y,
    },
];

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_sanity();

    let tray = drip_leak_tray();
    export(OUTPUTS[0], &tray);

    let pocket_bank = clean_pocket_bank();
    export(OUTPUTS[1], &pocket_bank);

    let cassette =
        single_tool_pocket(POCKETS[0]).translate(POCKETS[0].x, POCKETS[0].y, TOOL_CENTERLINE_Z);
    export(OUTPUTS[2], &cassette);

    let scanner =
        single_tool_pocket(POCKETS[1]).translate(POCKETS[1].x, POCKETS[1].y, TOOL_CENTERLINE_Z);
    export(OUTPUTS[3], &scanner);

    let tubing =
        single_tool_pocket(POCKETS[2]).translate(POCKETS[2].x, POCKETS[2].y, TOOL_CENTERLINE_Z);
    export(OUTPUTS[4], &tubing);

    let probe =
        single_tool_pocket(POCKETS[3]).translate(POCKETS[3].x, POCKETS[3].y, TOOL_CENTERLINE_Z);
    export(OUTPUTS[5], &probe);

    let segregation = clean_used_segregation();
    export(OUTPUTS[6], &segregation);

    let id_lands = tool_id_lands();
    export(OUTPUTS[7], &id_lands);

    let datums = force_torque_datum_checks();
    export(OUTPUTS[8], &datums);

    let vhp = vhp_cleanability_clearances();
    export(OUTPUTS[9], &vhp);

    let keepouts = collision_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = tray + pocket_bank + segregation + id_lands + datums + vhp + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Robot tool-change and end-effector rack:");
    println!("  Rack footprint:              {RACK_X:.0}mm x {RACK_Y:.0}mm");
    println!(
        "  Clean docking pockets:       {TOOL_COUNT} tools: cassette gripper, scanner/camera, tubing connector, calibration probe"
    );
    println!(
        "  Used quarantine saddles:     {USED_RETURN_SADDLE_COUNT} front-row saddles behind {CLEAN_USED_BARRIER_Z:.0}mm segregation wall"
    );
    println!(
        "  Tool ID lands:               {TOOL_ID_LAND_COUNT} barcode plates + {RFID_LAND_COUNT} RFID puck lands"
    );
    println!(
        "  Datum checks:                {FORCE_DATUM_COUNT} normal-force pads + {TORQUE_PIN_PAIRS} torque pin pairs"
    );
    println!(
        "  Cleanability clearances:     {MIN_WASHDOWN_GAP:.0}mm washdown gaps, {MIN_VHP_NOZZLE_CLEARANCE:.0}mm VHP corridor clearance"
    );
    println!(
        "  Service keepouts:            front {FRONT_SERVICE_CLEARANCE:.0}mm, side {SIDE_SERVICE_CLEARANCE:.0}mm, robot wrist Z {ROBOT_WRIST_CLEARANCE_Z:.0}mm"
    );
    println!("  Feature groups covered:      {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_sanity() {
    for pocket in POCKETS {
        assert!(
            pocket_fits_on_rack(pocket),
            "{} pocket exceeds rack envelope",
            pocket.slug
        );
        assert!(
            pocket.height + TOOL_CENTERLINE_Z < ROBOT_WRIST_CLEARANCE_Z - 110.0,
            "{} pocket enters robot wrist keepout",
            pocket.slug
        );
    }
    assert!(
        clean_used_air_gap() >= CLEAN_USED_AIR_GAP_MIN,
        "clean and used rows do not preserve the required physical air gap"
    );
    assert!(
        clean_used_air_gap() >= CLEAN_USED_GAP,
        "clean and used rows do not preserve the modeled segregation gap"
    );
    assert!(
        rack_fits_adjacent_to_closed_workcell(),
        "rack exceeds closed isolator work deck adjacency assumption"
    );
}

fn drip_leak_tray() -> Part {
    let floor = centered_cube(
        "robot_tool_rack_drip_tray_sloped_floor",
        RACK_X,
        RACK_Y,
        DRIP_TRAY_Z,
    )
    .translate(0.0, 0.0, DRIP_TRAY_Z / 2.0);
    let sump = centered_cube(
        "robot_tool_rack_drip_tray_low_sump",
        RACK_X - 74.0,
        RACK_Y - 82.0,
        TRAY_SUMP_DROP,
    )
    .translate(0.0, -28.0, DRIP_TRAY_Z - TRAY_SUMP_DROP / 2.0 + 4.0);
    let drain_gutter = centered_cube(
        "robot_tool_rack_front_drain_gutter",
        RACK_X - 148.0,
        18.0,
        18.0,
    )
    .translate(0.0, -(RACK_Y / 2.0 - 54.0), DRIP_TRAY_Z - 9.0);
    let drain_port = centered_cylinder(
        "robot_tool_rack_drip_tray_drain_port",
        DRAIN_PORT_D / 2.0,
        44.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        RACK_X / 2.0 - 78.0,
        -(RACK_Y / 2.0 - 32.0),
        DRIP_TRAY_Z - 14.0,
    );

    floor - sump - drain_gutter - drain_port + perimeter_rim() + mount_bosses() + datum_feet()
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "robot_tool_rack_front_washdown_rim",
        RACK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(0.0, -(RACK_Y / 2.0 - TRAY_RIM_W / 2.0), TRAY_RIM_Z / 2.0);
    let rear = centered_cube(
        "robot_tool_rack_rear_workcell_dock_rim",
        RACK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(0.0, RACK_Y / 2.0 - TRAY_RIM_W / 2.0, TRAY_RIM_Z / 2.0);
    let left = centered_cube(
        "robot_tool_rack_left_washdown_rim",
        TRAY_RIM_W,
        RACK_Y,
        TRAY_RIM_Z,
    )
    .translate(-(RACK_X / 2.0 - TRAY_RIM_W / 2.0), 0.0, TRAY_RIM_Z / 2.0);
    let right = centered_cube(
        "robot_tool_rack_right_washdown_rim",
        TRAY_RIM_W,
        RACK_Y,
        TRAY_RIM_Z,
    )
    .translate(RACK_X / 2.0 - TRAY_RIM_W / 2.0, 0.0, TRAY_RIM_Z / 2.0);

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty("robot_tool_rack_mount_bosses");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cylinder(format!("robot_tool_rack_mount_boss_{i}"), 18.0, 12.0, 32)
            .translate(*x, *y, 10.0);
        let hole = centered_cylinder(
            format!("robot_tool_rack_mount_boss_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            18.0,
            24,
        )
        .translate(*x, *y, 10.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn datum_feet() -> Part {
    let mut feet = Part::empty("robot_tool_rack_three_point_datum_feet");
    for (i, (x, y)) in [
        (-(RACK_X / 2.0 - 94.0), RACK_Y / 2.0 + REAR_WORKCELL_OFFSET),
        (RACK_X / 2.0 - 94.0, RACK_Y / 2.0 + REAR_WORKCELL_OFFSET),
        (0.0, RACK_Y / 2.0 + REAR_WORKCELL_OFFSET),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("robot_tool_rack_rear_datum_foot_{i}"),
            18.0,
            18.0,
            40,
        )
        .translate(*x, *y, 9.0);
        let center = centered_cylinder(
            format!("robot_tool_rack_rear_datum_center_mark_{i}"),
            2.0,
            20.0,
            18,
        )
        .translate(*x, *y, 9.0);
        feet = feet + (pad - center);
    }
    feet
}

fn clean_pocket_bank() -> Part {
    let mut bank = Part::empty("robot_tool_rack_clean_pocket_bank");
    for pocket in POCKETS {
        bank = bank
            + single_tool_pocket(pocket).translate(pocket.x, pocket.y, TOOL_CENTERLINE_Z)
            + approach_funnel(pocket).translate(pocket.x, pocket.y, TOOL_CENTERLINE_Z);
    }
    bank + rear_datum_rail()
}

fn single_tool_pocket(pocket: ToolPocket) -> Part {
    let body = centered_cube(
        format!("robot_tool_rack_{}_pocket_body", pocket.slug),
        pocket.width,
        pocket.depth,
        pocket.height,
    )
    .translate(0.0, 0.0, pocket.height / 2.0);
    let top_recess = centered_cube(
        format!("robot_tool_rack_{}_tool_body_recess", pocket.slug),
        pocket.cavity_x,
        pocket.cavity_y,
        pocket.cavity_depth + 6.0,
    )
    .translate(0.0, 0.0, pocket.height - pocket.cavity_depth / 2.0 + 3.0);
    let mouth_relief = centered_cube(
        format!("robot_tool_rack_{}_wide_top_mouth_relief", pocket.slug),
        pocket.cavity_x + 34.0,
        pocket.cavity_y + 28.0,
        22.0,
    )
    .translate(0.0, -6.0, pocket.height + 1.0);
    let drain_slot = centered_cube(
        format!("robot_tool_rack_{}_pocket_drain_slot", pocket.slug),
        pocket.cavity_x * 0.62,
        10.0,
        14.0,
    )
    .translate(0.0, -(pocket.depth / 2.0 - 20.0), 14.0);
    let latch_slot_left = centered_cube(
        format!("robot_tool_rack_{}_left_latch_slot", pocket.slug),
        18.0,
        24.0,
        18.0,
    )
    .translate(-(pocket.width / 2.0 - 22.0), 0.0, pocket.height - 18.0);
    let latch_slot_right = centered_cube(
        format!("robot_tool_rack_{}_right_latch_slot", pocket.slug),
        18.0,
        24.0,
        18.0,
    )
    .translate(pocket.width / 2.0 - 22.0, 0.0, pocket.height - 18.0);

    let base = body - top_recess - mouth_relief - drain_slot - latch_slot_left - latch_slot_right;
    base + pocket_hard_stops(pocket) + pocket_id_tabs(pocket) + special_tool_interface(pocket)
}

fn pocket_hard_stops(pocket: ToolPocket) -> Part {
    let rear_stop = centered_cube(
        format!("robot_tool_rack_{}_rear_hard_stop", pocket.slug),
        pocket.cavity_x + 22.0,
        12.0,
        34.0,
    )
    .translate(0.0, pocket.depth / 2.0 - 20.0, pocket.height - 17.0);
    let left_rail = centered_cube(
        format!("robot_tool_rack_{}_left_v_rail", pocket.slug),
        12.0,
        pocket.depth - 42.0,
        28.0,
    )
    .translate(-(pocket.cavity_x / 2.0 + 16.0), -2.0, pocket.height - 14.0);
    let right_rail = centered_cube(
        format!("robot_tool_rack_{}_right_v_rail", pocket.slug),
        12.0,
        pocket.depth - 42.0,
        28.0,
    )
    .translate(pocket.cavity_x / 2.0 + 16.0, -2.0, pocket.height - 14.0);

    let mut pins = Part::empty(format!("robot_tool_rack_{}_kinematic_pins", pocket.slug));
    for (i, (x, y)) in [
        (
            -(pocket.cavity_x / 2.0 - 18.0),
            pocket.cavity_y / 2.0 - 18.0,
        ),
        (pocket.cavity_x / 2.0 - 18.0, pocket.cavity_y / 2.0 - 18.0),
        (0.0, -(pocket.cavity_y / 2.0 - 18.0)),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("robot_tool_rack_{}_kinematic_pin_{i}", pocket.slug),
                5.0,
                12.0,
                24,
            )
            .translate(*x, *y, pocket.height + 4.0);
    }

    rear_stop + left_rail + right_rail + pins
}

fn pocket_id_tabs(pocket: ToolPocket) -> Part {
    let barcode_tab = centered_cube(
        format!("robot_tool_rack_{}_local_barcode_tab", pocket.slug),
        50.0,
        8.0,
        18.0,
    )
    .translate(0.0, -(pocket.depth / 2.0 + 4.0), pocket.height - 18.0);
    let rfid_boss = centered_cylinder(
        format!("robot_tool_rack_{}_local_rfid_boss", pocket.slug),
        9.0,
        5.0,
        28,
    )
    .translate(
        pocket.width / 2.0 - 26.0,
        -(pocket.depth / 2.0 - 18.0),
        pocket.height + 2.5,
    );
    barcode_tab + rfid_boss
}

fn special_tool_interface(pocket: ToolPocket) -> Part {
    match pocket.slug {
        "cassette_gripper" => cassette_gripper_interface(pocket),
        "scanner_camera" => scanner_camera_interface(pocket),
        "tubing_connector" => tubing_connector_interface(pocket),
        "calibration_probe" => calibration_probe_interface(pocket),
        _ => Part::empty(format!("robot_tool_rack_{}_generic_interface", pocket.slug)),
    }
}

fn cassette_gripper_interface(pocket: ToolPocket) -> Part {
    let left_fork_relief = centered_cube(
        "robot_tool_rack_cassette_gripper_left_finger_shadow",
        24.0,
        pocket.depth - 42.0,
        12.0,
    )
    .translate(-42.0, -4.0, pocket.height + 6.0);
    let right_fork_relief = centered_cube(
        "robot_tool_rack_cassette_gripper_right_finger_shadow",
        24.0,
        pocket.depth - 42.0,
        12.0,
    )
    .translate(42.0, -4.0, pocket.height + 6.0);
    let palm_stop = centered_cube(
        "robot_tool_rack_cassette_gripper_palm_stop",
        110.0,
        16.0,
        42.0,
    )
    .translate(0.0, pocket.depth / 2.0 - 34.0, pocket.height + 15.0);
    left_fork_relief + right_fork_relief + palm_stop
}

fn scanner_camera_interface(pocket: ToolPocket) -> Part {
    let lens_window = centered_cylinder(
        "robot_tool_rack_scanner_camera_lens_safe_window",
        25.0,
        8.0,
        48,
    )
    .translate(0.0, -12.0, pocket.height + 4.0);
    let light_baffle = frame_xy(
        "robot_tool_rack_scanner_camera_light_baffle_frame",
        104.0,
        76.0,
        10.0,
        18.0,
    )
    .translate(0.0, -4.0, pocket.height + 12.0);
    let focus_target = centered_cube(
        "robot_tool_rack_scanner_camera_focus_target_card",
        74.0,
        3.0,
        44.0,
    )
    .translate(0.0, pocket.depth / 2.0 - 26.0, pocket.height + 12.0);
    lens_window + light_baffle + focus_target
}

fn tubing_connector_interface(pocket: ToolPocket) -> Part {
    let wet_cup = centered_cylinder(
        "robot_tool_rack_tubing_connector_drip_cup_outer",
        36.0,
        28.0,
        48,
    )
    .translate(0.0, -8.0, pocket.height + 14.0);
    let wet_cup_hollow = centered_cylinder(
        "robot_tool_rack_tubing_connector_drip_cup_inner",
        25.0,
        30.0,
        48,
    )
    .translate(0.0, -8.0, pocket.height + 18.0);
    let port_block = centered_cube(
        "robot_tool_rack_tubing_connector_media_port_block",
        118.0,
        16.0,
        46.0,
    )
    .translate(0.0, pocket.depth / 2.0 - 26.0, pocket.height + 10.0);
    let mut ports = Part::empty("robot_tool_rack_tubing_connector_media_ports");
    for (i, x) in [-36.0, 0.0, 36.0].iter().enumerate() {
        ports = ports
            + centered_cylinder(
                format!("robot_tool_rack_tubing_connector_luerless_port_{i}"),
                6.0,
                20.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, pocket.depth / 2.0 - 27.0, pocket.height + 10.0);
    }
    (wet_cup - wet_cup_hollow) + (port_block - ports)
}

fn calibration_probe_interface(pocket: ToolPocket) -> Part {
    let sleeve = centered_cylinder(
        "robot_tool_rack_calibration_probe_sleeve_outer",
        25.0,
        96.0,
        48,
    )
    .translate(0.0, -6.0, pocket.height + 40.0);
    let bore = centered_cylinder(
        "robot_tool_rack_calibration_probe_sleeve_bore",
        12.0,
        102.0,
        40,
    )
    .translate(0.0, -6.0, pocket.height + 42.0);
    let probe_tip_guard = centered_cube(
        "robot_tool_rack_calibration_probe_tip_guard",
        70.0,
        34.0,
        34.0,
    )
    .translate(0.0, -(pocket.depth / 2.0 - 28.0), pocket.height + 8.0);
    (sleeve - bore) + probe_tip_guard
}

fn approach_funnel(pocket: ToolPocket) -> Part {
    let left = centered_cube(
        format!("robot_tool_rack_{}_left_approach_funnel", pocket.slug),
        14.0,
        72.0,
        34.0,
    )
    .rotate(0.0, 0.0, 10.0)
    .translate(-(pocket.width / 2.0 + 22.0), -58.0, pocket.height + 17.0);
    let right = centered_cube(
        format!("robot_tool_rack_{}_right_approach_funnel", pocket.slug),
        14.0,
        72.0,
        34.0,
    )
    .rotate(0.0, 0.0, -10.0)
    .translate(pocket.width / 2.0 + 22.0, -58.0, pocket.height + 17.0);
    left + right
}

fn rear_datum_rail() -> Part {
    let rail = centered_cube(
        "robot_tool_rack_rear_iso9409_style_datum_rail",
        RACK_X - 124.0,
        18.0,
        48.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 50.0, DRIP_TRAY_Z + 24.0);
    let mut datum_marks = Part::empty("robot_tool_rack_rear_datum_rail_marks");
    for (i, pocket) in POCKETS.iter().enumerate() {
        let mark = centered_cylinder(format!("robot_tool_rack_rear_datum_mark_{i}"), 7.0, 4.0, 32)
            .translate(pocket.x, RACK_Y / 2.0 - 60.0, DRIP_TRAY_Z + 50.0);
        let center = centered_cylinder(
            format!("robot_tool_rack_rear_datum_mark_center_{i}"),
            1.8,
            5.0,
            18,
        )
        .translate(pocket.x, RACK_Y / 2.0 - 60.0, DRIP_TRAY_Z + 50.0);
        datum_marks = datum_marks + (mark - center);
    }
    rail + datum_marks
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "robot_tool_rack_clean_used_segregation_wall",
        RACK_X - 118.0,
        24.0,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(
        0.0,
        CLEAN_USED_BARRIER_Y,
        DRIP_TRAY_Z + CLEAN_USED_BARRIER_Z / 2.0,
    );
    let pass_window = centered_cube(
        "robot_tool_rack_clean_used_locked_pass_window",
        78.0,
        30.0,
        58.0,
    )
    .translate(0.0, CLEAN_USED_BARRIER_Y, DRIP_TRAY_Z + 62.0);
    let wall = wall - pass_window + barrier_top_drip_edge();

    let mut used = Part::empty("robot_tool_rack_used_return_saddles");
    for (i, pocket) in POCKETS.iter().enumerate() {
        used = used + used_return_saddle(i, *pocket);
    }

    wall + used + used_tool_spill_gutter()
}

fn barrier_top_drip_edge() -> Part {
    centered_cube(
        "robot_tool_rack_clean_used_barrier_sloped_drip_edge",
        RACK_X - 150.0,
        14.0,
        18.0,
    )
    .translate(
        0.0,
        CLEAN_USED_BARRIER_Y - 8.0,
        DRIP_TRAY_Z + CLEAN_USED_BARRIER_Z + 9.0,
    )
}

fn used_return_saddle(index: usize, pocket: ToolPocket) -> Part {
    let saddle_x = pocket.width - 34.0;
    let saddle_y = 74.0;
    let saddle_z = 32.0;
    let body = centered_cube(
        format!(
            "robot_tool_rack_used_{}_return_saddle_body_{index}",
            pocket.slug
        ),
        saddle_x,
        saddle_y,
        saddle_z,
    )
    .translate(pocket.x, USED_ROW_Y, DRIP_TRAY_Z + saddle_z / 2.0);
    let recess = centered_cube(
        format!(
            "robot_tool_rack_used_{}_return_shadow_recess_{index}",
            pocket.slug
        ),
        saddle_x - 34.0,
        saddle_y - 24.0,
        saddle_z,
    )
    .translate(pocket.x, USED_ROW_Y, DRIP_TRAY_Z + saddle_z / 2.0 + 9.0);
    let drip_hole = centered_cylinder(
        format!(
            "robot_tool_rack_used_{}_return_drip_hole_{index}",
            pocket.slug
        ),
        5.0,
        saddle_z + 4.0,
        24,
    )
    .translate(pocket.x, USED_ROW_Y - 18.0, DRIP_TRAY_Z + saddle_z / 2.0);
    body - recess - drip_hole
}

fn used_tool_spill_gutter() -> Part {
    let gutter = centered_cube(
        "robot_tool_rack_used_return_spill_gutter",
        RACK_X - 164.0,
        18.0,
        18.0,
    )
    .translate(0.0, USED_ROW_Y - 58.0, DRIP_TRAY_Z + 9.0);
    let drain = centered_cylinder("robot_tool_rack_used_return_gutter_drain", 7.0, 26.0, 28)
        .translate(RACK_X / 2.0 - 102.0, USED_ROW_Y - 58.0, DRIP_TRAY_Z + 9.0);
    gutter - drain
}

fn tool_id_lands() -> Part {
    let mut lands = Part::empty("robot_tool_rack_tool_id_lands");
    for (i, pocket) in POCKETS.iter().enumerate() {
        let barcode = centered_cube(
            format!("robot_tool_rack_{}_barcode_land_{i}", pocket.slug),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            4.0,
        )
        .translate(pocket.x - 34.0, 6.0, DRIP_TRAY_Z + 4.0);
        let barcode_clip_holes = barcode_clip_holes(i, *pocket);
        let rfid = centered_cylinder(
            format!("robot_tool_rack_{}_rfid_puck_land_{i}", pocket.slug),
            RFID_PAD_D / 2.0,
            5.0,
            36,
        )
        .translate(pocket.x + 46.0, 6.0, DRIP_TRAY_Z + 4.5);
        let spring_contact_land = centered_cube(
            format!(
                "robot_tool_rack_{}_spring_contact_code_land_{i}",
                pocket.slug
            ),
            36.0,
            16.0,
            5.0,
        )
        .translate(pocket.x + 86.0, 6.0, DRIP_TRAY_Z + 4.5);
        lands = lands + (barcode - barcode_clip_holes) + rfid + spring_contact_land;
    }
    lands
}

fn barcode_clip_holes(index: usize, pocket: ToolPocket) -> Part {
    let mut holes = Part::empty(format!("robot_tool_rack_barcode_clip_holes_{index}"));
    for (j, dx) in [-24.0, 24.0].iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!(
                    "robot_tool_rack_{}_barcode_clip_hole_{index}_{j}",
                    pocket.slug
                ),
                1.8,
                8.0,
                16,
            )
            .translate(pocket.x - 34.0 + dx, 6.0, DRIP_TRAY_Z + 4.0);
    }
    holes
}

fn force_torque_datum_checks() -> Part {
    let mut datums = Part::empty("robot_tool_rack_force_torque_datum_checks");
    for (i, pocket) in POCKETS.iter().enumerate() {
        datums = datums + force_torque_station(i, *pocket);
    }
    datums
}

fn force_torque_station(index: usize, pocket: ToolPocket) -> Part {
    let base = centered_cube(
        format!(
            "robot_tool_rack_{}_force_torque_station_base_{index}",
            pocket.slug
        ),
        94.0,
        42.0,
        16.0,
    )
    .translate(pocket.x, RACK_Y / 2.0 - 96.0, DRIP_TRAY_Z + 8.0);
    let force_pad = centered_cylinder(
        format!("robot_tool_rack_{}_normal_force_pad_{index}", pocket.slug),
        FORCE_PAD_D / 2.0,
        7.0,
        36,
    )
    .translate(pocket.x, RACK_Y / 2.0 - 96.0, DRIP_TRAY_Z + 19.5);
    let left_torque_pin = centered_cylinder(
        format!("robot_tool_rack_{}_left_torque_pin_{index}", pocket.slug),
        TORQUE_PIN_D / 2.0,
        24.0,
        24,
    )
    .translate(pocket.x - 28.0, RACK_Y / 2.0 - 96.0, DRIP_TRAY_Z + 28.0);
    let right_torque_pin = centered_cylinder(
        format!("robot_tool_rack_{}_right_torque_pin_{index}", pocket.slug),
        TORQUE_PIN_D / 2.0,
        24.0,
        24,
    )
    .translate(pocket.x + 28.0, RACK_Y / 2.0 - 96.0, DRIP_TRAY_Z + 28.0);
    let datum_arrow = centered_cube(
        format!("robot_tool_rack_{}_datum_vector_arrow_{index}", pocket.slug),
        34.0,
        6.0,
        4.0,
    )
    .translate(pocket.x, RACK_Y / 2.0 - 124.0, DRIP_TRAY_Z + 22.0);

    base + force_pad + left_torque_pin + right_torque_pin + datum_arrow
}

fn vhp_cleanability_clearances() -> Part {
    let clearance_bridge = frame_xz(
        "robot_tool_rack_vhp_nozzle_clearance_bridge",
        RACK_X - 150.0,
        16.0,
        VHP_CORRIDOR_Z,
        18.0,
    )
    .translate(
        0.0,
        RACK_Y / 2.0 - VHP_CORRIDOR_Y,
        DRIP_TRAY_Z + VHP_CORRIDOR_Z / 2.0,
    );
    let washdown_gap_gauges = washdown_gap_gauges();
    let coved_corner_standins = cleanability_cove_standins();
    clearance_bridge + washdown_gap_gauges + coved_corner_standins
}

fn washdown_gap_gauges() -> Part {
    let mut gauges = Part::empty("robot_tool_rack_washdown_gap_gauges");
    for (i, pocket) in POCKETS.iter().enumerate() {
        gauges = gauges
            + centered_cube(
                format!(
                    "robot_tool_rack_{}_minimum_washdown_gap_gauge_{i}",
                    pocket.slug
                ),
                MIN_WASHDOWN_GAP,
                10.0,
                42.0,
            )
            .translate(
                pocket.x,
                pocket.y - pocket.depth / 2.0 - MIN_WASHDOWN_GAP / 2.0,
                DRIP_TRAY_Z + 42.0,
            );
    }
    gauges
}

fn cleanability_cove_standins() -> Part {
    let mut coves = Part::empty("robot_tool_rack_cleanability_cove_standins");
    for (i, (x, y)) in [
        (-(RACK_X / 2.0 - 44.0), -(RACK_Y / 2.0 - 44.0)),
        (RACK_X / 2.0 - 44.0, -(RACK_Y / 2.0 - 44.0)),
        (-(RACK_X / 2.0 - 44.0), RACK_Y / 2.0 - 44.0),
        (RACK_X / 2.0 - 44.0, RACK_Y / 2.0 - 44.0),
    ]
    .iter()
    .enumerate()
    {
        coves = coves
            + centered_cylinder(
                format!("robot_tool_rack_washdown_cove_radius_standin_{i}"),
                16.0,
                74.0,
                32,
            )
            .translate(*x, *y, DRIP_TRAY_Z + 37.0);
    }
    coves
}

fn collision_service_keepouts() -> Part {
    let robot_wrist = frame_xy(
        "robot_tool_rack_robot_wrist_swept_keepout",
        RACK_X - 120.0,
        210.0,
        14.0,
        12.0,
    )
    .translate(0.0, CLEAN_ROW_Y, ROBOT_WRIST_CLEARANCE_Z);
    let front_service = frame_xy(
        "robot_tool_rack_front_service_pullout_keepout",
        RACK_X - 80.0,
        FRONT_SERVICE_CLEARANCE,
        18.0,
        12.0,
    )
    .translate(
        0.0,
        -(RACK_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        DRIP_TRAY_Z + 80.0,
    );
    let left_side_service = frame_xy(
        "robot_tool_rack_left_side_service_keepout",
        SIDE_SERVICE_CLEARANCE,
        RACK_Y - 70.0,
        14.0,
        10.0,
    )
    .translate(
        -(RACK_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0),
        0.0,
        DRIP_TRAY_Z + 80.0,
    );
    let right_side_service = frame_xy(
        "robot_tool_rack_right_side_service_keepout",
        SIDE_SERVICE_CLEARANCE,
        RACK_Y - 70.0,
        14.0,
        10.0,
    )
    .translate(
        RACK_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0,
        0.0,
        DRIP_TRAY_Z + 80.0,
    );
    let rear_workcell_interface = centered_cube(
        "robot_tool_rack_rear_closed_workcell_collision_plane",
        RACK_X - 140.0,
        10.0,
        250.0,
    )
    .translate(
        0.0,
        RACK_Y / 2.0 + REAR_WORKCELL_OFFSET,
        DRIP_TRAY_Z + 125.0,
    );

    robot_wrist + front_service + left_side_service + right_side_service + rear_workcell_interface
}

fn frame_xy(name: impl Into<String>, outer_x: f64, outer_y: f64, rail: f64, z_t: f64) -> Part {
    let base = name.into();
    let front = centered_cube(format!("{base}_front"), outer_x, rail, z_t).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{base}_rear"), outer_x, rail, z_t).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{base}_left"), rail, outer_y, z_t).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{base}_right"), rail, outer_y, z_t).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn frame_xz(name: impl Into<String>, outer_x: f64, y_t: f64, outer_z: f64, rail: f64) -> Part {
    let base = name.into();
    let top = centered_cube(format!("{base}_top"), outer_x, y_t, rail).translate(
        0.0,
        0.0,
        outer_z / 2.0 - rail / 2.0,
    );
    let bottom = centered_cube(format!("{base}_bottom"), outer_x, y_t, rail).translate(
        0.0,
        0.0,
        -outer_z / 2.0 + rail / 2.0,
    );
    let left = centered_cube(format!("{base}_left"), rail, y_t, outer_z).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{base}_right"), rail, y_t, outer_z).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    top + bottom + left + right
}

fn pocket_fits_on_rack(pocket: ToolPocket) -> bool {
    pocket.x.abs() + pocket.width / 2.0 <= RACK_X / 2.0 - TRAY_RIM_W - 8.0
        && pocket.y.abs() + pocket.depth / 2.0 <= RACK_Y / 2.0 - TRAY_RIM_W - 8.0
}

fn clean_used_air_gap() -> f64 {
    let clean_front = CLEAN_ROW_Y - max_pocket_depth() / 2.0;
    let used_rear = USED_ROW_Y + 74.0 / 2.0;
    clean_front - used_rear
}

fn max_pocket_depth() -> f64 {
    POCKETS
        .iter()
        .map(|pocket| pocket.depth)
        .fold(0.0, f64::max)
}

fn rack_fits_adjacent_to_closed_workcell() -> bool {
    RACK_X <= CLOSED_WORKCELL_DECK_X - 120.0 && RACK_Y <= CLOSED_WORKCELL_DECK_Y - 420.0
}

fn mount_points() -> [(f64, f64); 6] {
    [
        (-(RACK_X / 2.0 - 72.0), -(RACK_Y / 2.0 - 62.0)),
        (RACK_X / 2.0 - 72.0, -(RACK_Y / 2.0 - 62.0)),
        (-(RACK_X / 2.0 - 72.0), RACK_Y / 2.0 - 62.0),
        (RACK_X / 2.0 - 72.0, RACK_Y / 2.0 - 62.0),
        (-86.0, RACK_Y / 2.0 - 62.0),
        (86.0, RACK_Y / 2.0 - 62.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/robot_tool_change_and_end_effector_rack_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_robot_tool_change_rack() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"cassette_gripper_docking_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"scanner_camera_docking_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"pipette_free_tubing_connector_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"calibration_probe_tool_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_physical_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_tool_id_lands"));
        assert!(REQUIRED_FEATURES.contains(&"force_torque_datum_checks"));
        assert!(REQUIRED_FEATURES.contains(&"drip_leak_capture_tray"));
        assert!(REQUIRED_FEATURES.contains(&"vhp_cleanability_clearances"));
        assert!(REQUIRED_FEATURES.contains(&"collision_service_keepouts"));
    }

    #[test]
    fn all_tool_pockets_fit_the_rack_and_stay_in_clean_row() {
        assert_eq!(POCKETS.len(), TOOL_COUNT);
        for pocket in POCKETS {
            assert!(pocket_fits_on_rack(pocket));
            assert!(pocket.y > CLEAN_USED_BARRIER_Y);
            assert!(pocket.height >= 70.0);
            assert!(pocket.cavity_x < pocket.width);
            assert!(pocket.cavity_y < pocket.depth);
            assert!(pocket.cavity_depth < pocket.height);
        }
    }

    #[test]
    fn clean_and_used_tool_flows_are_physically_segregated() {
        assert_eq!(USED_RETURN_SADDLE_COUNT, TOOL_COUNT);
        assert!(CLEAN_USED_BARRIER_Z >= 110.0);
        assert!(CLEAN_USED_GAP >= CLEAN_USED_AIR_GAP_MIN);
        assert!(clean_used_air_gap() >= CLEAN_USED_AIR_GAP_MIN);
        assert!(USED_ROW_Y < CLEAN_USED_BARRIER_Y);
    }

    #[test]
    fn id_and_datum_counts_track_tool_count() {
        assert_eq!(TOOL_ID_LAND_COUNT, TOOL_COUNT);
        assert_eq!(RFID_LAND_COUNT, TOOL_COUNT);
        assert_eq!(FORCE_DATUM_COUNT, TOOL_COUNT);
        assert_eq!(TORQUE_PIN_PAIRS, TOOL_COUNT);
        assert!(BARCODE_LAND_X >= 70.0);
        assert!(RFID_PAD_D >= 20.0);
        assert!(FORCE_PAD_D > TORQUE_PIN_D);
    }

    #[test]
    fn cleanability_and_service_clearances_are_explicit() {
        assert!(MIN_WASHDOWN_GAP >= 30.0);
        assert!(MIN_VHP_NOZZLE_CLEARANCE >= 75.0);
        assert!(VHP_CORRIDOR_Z >= 220.0);
        assert!(ROBOT_WRIST_CLEARANCE_Z >= 300.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 500.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 160.0);
    }

    #[test]
    fn drip_tray_and_workcell_adjacency_are_sane() {
        assert!(DRIP_TRAY_Z >= 40.0);
        assert!(TRAY_RIM_Z > DRIP_TRAY_Z);
        assert!(TRAY_SUMP_DROP >= 12.0);
        assert!(DRAIN_PORT_D >= 16.0);
        assert_eq!(mount_points().len(), 6);
        assert!(rack_fits_adjacent_to_closed_workcell());
    }
}
