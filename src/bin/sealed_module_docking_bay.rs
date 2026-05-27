use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Docking bay for one sealed culture module inside the larger clean automation pod.
//
// Intent:
// - Define how the sealed module sits in the pod: rails, hard stops, latch points, and leak capture.
// - Keep utilities standardized at a rear service receiver instead of hand-routing every build.
// - Separate the clean pod boundary from the module fluid/sensor/thermal boundary.
// - Provide robot/technician datum features without exposing the culture path to room air.
//
// This is an architecture CAD model. Connector selection, sterilization, pressure testing,
// and biological validation remain separate gates.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;
const NEST_X: f64 = CASSETTE_X + 160.0;
const NEST_Y: f64 = CASSETTE_Y + 150.0;

const MODULE_X: f64 = NEST_X + 150.0;
const MODULE_Y: f64 = NEST_Y + 135.0;
const MODULE_Z: f64 = 88.0;

const BAY_X: f64 = MODULE_X + 130.0;
const BAY_Y: f64 = MODULE_Y + 170.0;
const BAY_Z: f64 = 52.0;
const RAIL_Z: f64 = 34.0;
const RECEIVER_Z: f64 = 128.0;
const RECEIVER_Y: f64 = 58.0;

fn main() {
    let tray = leak_tray();
    tray.write_stl("output/sealed_module_docking_bay_tray.stl")
        .unwrap();
    println!("Exported: output/sealed_module_docking_bay_tray.stl");

    let rails = datum_rails_and_stops();
    rails
        .write_stl("output/sealed_module_docking_bay_rails.stl")
        .unwrap();
    println!("Exported: output/sealed_module_docking_bay_rails.stl");

    let receiver = service_receiver();
    receiver
        .write_stl("output/sealed_module_docking_bay_service_receiver.stl")
        .unwrap();
    println!("Exported: output/sealed_module_docking_bay_service_receiver.stl");

    let latch = latch_and_present_sensors();
    latch
        .write_stl("output/sealed_module_docking_bay_latch_sensors.stl")
        .unwrap();
    println!("Exported: output/sealed_module_docking_bay_latch_sensors.stl");

    let assembly = tray
        + rails.translate(0.0, -18.0, BAY_Z / 2.0 + RAIL_Z / 2.0)
        + receiver.translate(0.0, BAY_Y / 2.0 - RECEIVER_Y / 2.0, BAY_Z / 2.0 + 36.0)
        + latch.translate(0.0, -BAY_Y / 2.0 + 72.0, BAY_Z / 2.0 + 28.0)
        + module_keepout().translate(0.0, -18.0, BAY_Z / 2.0 + MODULE_Z / 2.0 + 4.0);

    assembly
        .write_stl("output/sealed_module_docking_bay_assembly.stl")
        .unwrap();
    println!("Exported: output/sealed_module_docking_bay_assembly.stl");

    println!(
        "Sealed module docking bay: {:.0}mm x {:.0}mm x {:.0}mm tray, {:.0}mm module keepout, rear utility receiver, latch/sensor datum features.",
        BAY_X, BAY_Y, BAY_Z, MODULE_Z
    );
}

fn leak_tray() -> Part {
    let tray_outer = centered_cube("module_dock_tray_outer", BAY_X, BAY_Y, BAY_Z);
    let tray_sump = centered_cube("module_dock_tray_sump", BAY_X - 40.0, BAY_Y - 46.0, BAY_Z)
        .translate(0.0, -12.0, 14.0);

    let drain_channel = centered_cube("module_dock_drain_channel", BAY_X - 160.0, 20.0, 14.0)
        .translate(0.0, -BAY_Y / 2.0 + 78.0, -BAY_Z / 2.0 + 14.0);
    let drain_port = centered_cylinder("module_dock_drain_port", 8.0 / 2.0, 36.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(BAY_X / 2.0 - 78.0, -BAY_Y / 2.0 - 2.0, -BAY_Z / 2.0 + 15.0);

    let robot_fork_left = centered_cube("module_dock_robot_fork_left", 130.0, 28.0, 18.0)
        .translate(-220.0, -BAY_Y / 2.0 + 28.0, -BAY_Z / 2.0 + 10.0);
    let robot_fork_right = centered_cube("module_dock_robot_fork_right", 130.0, 28.0, 18.0)
        .translate(220.0, -BAY_Y / 2.0 + 28.0, -BAY_Z / 2.0 + 10.0);

    let mut mount_holes = Part::empty("module_dock_mount_holes");
    for (i, (x, y)) in tray_mount_points().iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("module_dock_mount_hole_{i}"),
                6.6 / 2.0,
                BAY_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    tray_outer
        - tray_sump
        - drain_channel
        - drain_port
        - robot_fork_left
        - robot_fork_right
        - mount_holes
        + tray_mount_bosses()
}

fn tray_mount_bosses() -> Part {
    let mut bosses = Part::empty("module_dock_mount_bosses");
    for (i, (x, y)) in tray_mount_points().iter().enumerate() {
        let boss = centered_cylinder(format!("module_dock_mount_boss_{i}"), 14.0, 12.0, 32)
            .translate(*x, *y, -BAY_Z / 2.0 + 8.0);
        let hole = centered_cylinder(
            format!("module_dock_mount_boss_hole_{i}"),
            6.6 / 2.0,
            14.0,
            24,
        )
        .translate(*x, *y, -BAY_Z / 2.0 + 8.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn datum_rails_and_stops() -> Part {
    let rail_y = MODULE_Y + 52.0;
    let rail_offset_x = MODULE_X / 2.0 + 28.0;

    let left_rail = rail("left").translate(-rail_offset_x, 0.0, 0.0);
    let right_rail = rail("right").translate(rail_offset_x, 0.0, 0.0);
    let rear_stop = centered_cube("module_dock_rear_hard_stop", MODULE_X + 72.0, 24.0, RAIL_Z)
        .translate(0.0, rail_y / 2.0 - 12.0, 0.0);

    let mut kinematic_stops = Part::empty("module_dock_kinematic_stops");
    for (i, (x, y)) in [
        (-(MODULE_X / 2.0 - 62.0), rail_y / 2.0 - 50.0),
        (MODULE_X / 2.0 - 62.0, rail_y / 2.0 - 50.0),
        (-(MODULE_X / 2.0 - 62.0), -(rail_y / 2.0 - 70.0)),
        (MODULE_X / 2.0 - 62.0, -(rail_y / 2.0 - 70.0)),
    ]
    .iter()
    .enumerate()
    {
        kinematic_stops =
            kinematic_stops
                + centered_cylinder(format!("module_dock_tapered_stop_{i}"), 11.0, 12.0, 32)
                    .translate(*x, *y, RAIL_Z / 2.0 + 6.0);
    }

    left_rail + right_rail + rear_stop + kinematic_stops
}

fn rail(name: &str) -> Part {
    let body = centered_cube(
        format!("{name}_module_dock_rail_body"),
        34.0,
        MODULE_Y + 52.0,
        RAIL_Z,
    );
    let top_clearance = centered_cube(
        format!("{name}_module_dock_rail_top_relief"),
        18.0,
        MODULE_Y + 20.0,
        RAIL_Z + 2.0,
    )
    .translate(0.0, 0.0, 12.0);

    let mut latch_slots = Part::empty(format!("{name}_module_dock_latch_slots"));
    for (i, y) in [-(MODULE_Y / 2.0 - 85.0), 0.0, MODULE_Y / 2.0 - 85.0]
        .iter()
        .enumerate()
    {
        latch_slots = latch_slots
            + centered_cube(
                format!("{name}_module_dock_latch_slot_{i}"),
                38.0,
                28.0,
                RAIL_Z + 2.0,
            )
            .translate(0.0, *y, 0.0);
    }

    body - top_clearance - latch_slots
}

fn service_receiver() -> Part {
    let body = centered_cube(
        "module_dock_service_receiver_body",
        MODULE_X - 60.0,
        RECEIVER_Y,
        RECEIVER_Z,
    );
    let mut cuts = Part::empty("module_dock_service_receiver_cuts");

    for (i, x) in [-315.0, -285.0, -255.0, -225.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("module_dock_gas_receiver_{i}"),
                10.0 / 2.0,
                70.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 34.0);
    }

    for (i, x) in [-160.0, -125.0, -90.0, -55.0, -20.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("module_dock_media_receiver_{i}"),
                8.0 / 2.0,
                70.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 8.0);
    }

    for (i, x) in [42.0, 77.0, 112.0, 147.0, 182.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("module_dock_waste_receiver_{i}"),
                8.0 / 2.0,
                70.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 8.0);
    }

    for (i, x) in [260.0, 302.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("module_dock_thermal_receiver_{i}"),
                12.0 / 2.0,
                70.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -28.0);
    }

    let sensor_backplane =
        centered_cube("module_dock_sensor_backplane_receiver", 118.0, 70.0, 22.0)
            .translate(300.0, 0.0, 20.0);
    let service_label_land = centered_cube(
        "module_dock_service_label_land",
        MODULE_X - 120.0,
        4.0,
        10.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 - 1.0, RECEIVER_Z / 2.0 - 24.0);

    body - cuts - sensor_backplane + service_label_land + receiver_alignment_pins()
}

fn receiver_alignment_pins() -> Part {
    let mut pins = Part::empty("module_dock_receiver_alignment_pins");
    for (i, x) in [-(MODULE_X / 2.0 - 92.0), MODULE_X / 2.0 - 92.0]
        .iter()
        .enumerate()
    {
        pins = pins
            + centered_cylinder(format!("module_dock_alignment_pin_{i}"), 7.5, 18.0, 32)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, -RECEIVER_Y / 2.0 - 9.0, 44.0);
    }
    pins
}

fn latch_and_present_sensors() -> Part {
    let mut features = Part::empty("module_dock_latch_sensor_features");

    for (i, x) in [-(MODULE_X / 2.0 - 120.0), MODULE_X / 2.0 - 120.0]
        .iter()
        .enumerate()
    {
        let latch_block = centered_cube(
            format!("module_dock_front_latch_block_{i}"),
            92.0,
            46.0,
            44.0,
        )
        .translate(*x, 0.0, 0.0);
        let cam_bore = centered_cylinder(
            format!("module_dock_front_cam_bore_{i}"),
            10.0 / 2.0,
            96.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, 0.0, 4.0);
        features = features + (latch_block - cam_bore);
    }

    let optical_flag_left = centered_cube("module_dock_present_sensor_left", 34.0, 20.0, 24.0)
        .translate(-(MODULE_X / 2.0 - 46.0), 34.0, 10.0);
    let optical_flag_right = centered_cube("module_dock_present_sensor_right", 34.0, 20.0, 24.0)
        .translate(MODULE_X / 2.0 - 46.0, 34.0, 10.0);
    let fiducial_a = fiducial_target("a").translate(-(MODULE_X / 2.0 - 75.0), -34.0, 26.0);
    let fiducial_b = fiducial_target("b").translate(MODULE_X / 2.0 - 75.0, -34.0, 26.0);

    features + optical_flag_left + optical_flag_right + fiducial_a + fiducial_b
}

fn fiducial_target(name: &str) -> Part {
    let disk = centered_cylinder(format!("module_dock_fiducial_{name}"), 9.0, 2.0, 40);
    let center = centered_cylinder(format!("module_dock_fiducial_{name}_center"), 2.0, 3.0, 20);
    disk - center
}

fn module_keepout() -> Part {
    let shell = centered_cube(
        "sealed_module_keepout_transparent_placeholder",
        MODULE_X,
        MODULE_Y,
        MODULE_Z,
    );
    let cavity = centered_cube(
        "sealed_module_keepout_inner_relief",
        MODULE_X - 34.0,
        MODULE_Y - 34.0,
        MODULE_Z + 2.0,
    )
    .translate(0.0, 0.0, 6.0);
    shell - cavity
}

fn tray_mount_points() -> [(f64, f64); 8] {
    [
        (-(BAY_X / 2.0 - 44.0), -(BAY_Y / 2.0 - 44.0)),
        (BAY_X / 2.0 - 44.0, -(BAY_Y / 2.0 - 44.0)),
        (-(BAY_X / 2.0 - 44.0), BAY_Y / 2.0 - 44.0),
        (BAY_X / 2.0 - 44.0, BAY_Y / 2.0 - 44.0),
        (0.0, -(BAY_Y / 2.0 - 44.0)),
        (0.0, BAY_Y / 2.0 - 44.0),
        (-(BAY_X / 2.0 - 44.0), 0.0),
        (BAY_X / 2.0 - 44.0, 0.0),
    ]
}
