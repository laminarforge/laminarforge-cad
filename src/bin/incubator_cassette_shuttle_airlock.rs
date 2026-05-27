use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Incubator-to-isolator cassette shuttle/airlock concept.
//
// The model is intentionally architectural: it reserves the sterile cassette
// envelope, interlocked incubator/isolator doors, thermal and humidity control
// features, scan/datum references, and robot/service clearances.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const CASSETTE_CLEARANCE: f64 = 0.8;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const SEALED_CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const SEALED_CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;
const SEALED_CASSETTE_Z: f64 = 44.0;

const TRAY_WALL: f64 = 28.0;
const TRAY_X: f64 = SEALED_CASSETTE_X + TRAY_WALL * 2.0 + 46.0;
const TRAY_Y: f64 = SEALED_CASSETTE_Y + TRAY_WALL * 2.0 + 46.0;
const TRAY_Z: f64 = 36.0;
const BASIN_DEPTH: f64 = 14.0;
const GASKET_LAND_Z: f64 = 7.0;

const AIRLOCK_X: f64 = 900.0;
const AIRLOCK_Y: f64 = 860.0;
const AIRLOCK_Z: f64 = 370.0;
const TRANSFER_FLOOR_Z: f64 = -AIRLOCK_Z / 2.0 + 102.0;
const TRAY_CENTER_Z: f64 = TRANSFER_FLOOR_Z + TRAY_Z / 2.0;
const TUNNEL_X: f64 = TRAY_X + 62.0;
const TUNNEL_Z: f64 = SEALED_CASSETTE_Z + 150.0;
const TUNNEL_CENTER_Z: f64 = TRANSFER_FLOOR_Z + TUNNEL_Z / 2.0 - 12.0;

const DOOR_THICKNESS: f64 = 30.0;
const DOOR_GAP: f64 = 10.0;
const DOOR_X: f64 = TUNNEL_X + 82.0;
const DOOR_Z: f64 = TUNNEL_Z + 74.0;
const GASKET_Y: f64 = 7.0;

const THERMAL_PLATE_X: f64 = SEALED_CASSETTE_X + 94.0;
const THERMAL_PLATE_Y: f64 = 205.0;
const THERMAL_PLATE_Z: f64 = 18.0;
const THERMAL_PLATE_CENTER_Y: f64 = 0.0;
const THERMAL_PLATE_CENTER_Z: f64 = TRANSFER_FLOOR_Z - 12.0;
const THERMAL_CHANNEL_DIA: f64 = 9.5;
const THERMAL_CHANNEL_PITCH: f64 = 48.0;

const DRAIN_GUTTER_W: f64 = 18.0;
const DRAIN_SUMP_X: f64 = 92.0;
const DRAIN_SUMP_Y: f64 = 58.0;
const DRAIN_PORT_DIA: f64 = 11.0;
const CONDENSATE_CHANNEL_Z: f64 = 9.0;

const RAIL_Z: f64 = 28.0;
const RAIL_W: f64 = 18.0;
const MODULE_HANDOFF_CLEARANCE_Z: f64 = REVC_TOTAL_HEIGHT + 52.0;

const ROBOT_GRIPPER_SLOT_X: f64 = 42.0;
const ROBOT_GRIPPER_SLOT_Y: f64 = 252.0;
const ROBOT_GRIPPER_SLOT_Z: f64 = 42.0;
const SERVICE_KEEP_OUT_Y: f64 = 132.0;
const SERVICE_KEEP_OUT_Z: f64 = 98.0;

const HEPA_PORT_DIA: f64 = 76.0;
const VHP_PORT_DIA: f64 = 24.0;

fn main() {
    let shell = airlock_shell();
    shell
        .write_stl("output/incubator_cassette_shuttle_airlock_shell.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_shell.stl");

    let tray = sealed_cassette_tray();
    tray.write_stl("output/incubator_cassette_shuttle_airlock_sealed_tray.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_sealed_tray.stl");

    let interlock = dual_door_interlock();
    interlock
        .write_stl("output/incubator_cassette_shuttle_airlock_dual_door_interlock.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_dual_door_interlock.stl");

    let thermal = thermal_buffer_plate();
    thermal
        .write_stl("output/incubator_cassette_shuttle_airlock_thermal_buffer_plate.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_thermal_buffer_plate.stl");

    let drains = humidity_condensation_drain_features();
    drains
        .write_stl("output/incubator_cassette_shuttle_airlock_humidity_drain_features.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_humidity_drain_features.stl");

    let ports = hepa_vhp_port_placeholders();
    ports
        .write_stl("output/incubator_cassette_shuttle_airlock_hepa_vhp_ports.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_hepa_vhp_ports.stl");

    let datum = scan_land_and_datum_handoff();
    datum
        .write_stl("output/incubator_cassette_shuttle_airlock_scan_datum_handoff.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_scan_datum_handoff.stl");

    let keepouts = robot_and_service_keepouts();
    keepouts
        .write_stl("output/incubator_cassette_shuttle_airlock_robot_service_keepouts.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_robot_service_keepouts.stl");

    let assembly = shell + tray + interlock + thermal + drains + ports + datum + keepouts;
    assembly
        .write_stl("output/incubator_cassette_shuttle_airlock_assembly.stl")
        .unwrap();
    println!("Exported: output/incubator_cassette_shuttle_airlock_assembly.stl");

    println!(
        "Incubator cassette shuttle airlock: {:.0}mm W x {:.0}mm D x {:.0}mm H, {:.0}mm x {:.0}mm sealed cassette tray for {COLS}x{ROWS} Rev C chips.",
        AIRLOCK_X, AIRLOCK_Y, AIRLOCK_Z, TRAY_X, TRAY_Y
    );
    println!(
        "Door center separation: {:.0}mm; transfer tunnel: {:.0}mm W x {:.0}mm H; thermal buffer plate: {:.0}mm x {:.0}mm.",
        door_center_separation(),
        TUNNEL_X,
        TUNNEL_Z,
        THERMAL_PLATE_X,
        THERMAL_PLATE_Y
    );
    println!(
        "HEPA ports reserve {:.0}mm clear filters; VHP ports reserve {:.0}mm service bores; drain port {:.0}mm.",
        HEPA_PORT_DIA, VHP_PORT_DIA, DRAIN_PORT_DIA
    );
    println!(
        "Robot gripper side gap: {:.0}mm; sealed module handoff keepout height: {:.0}mm.",
        minimum_robot_gripper_gap(),
        MODULE_HANDOFF_CLEARANCE_Z
    );
}

fn airlock_shell() -> Part {
    let shell = centered_cube(
        "cassette_shuttle_airlock_outer_shell",
        AIRLOCK_X,
        AIRLOCK_Y,
        AIRLOCK_Z,
    );
    let tunnel = centered_cube(
        "cassette_shuttle_transfer_tunnel_cut",
        TUNNEL_X,
        AIRLOCK_Y + 4.0,
        TUNNEL_Z,
    )
    .translate(0.0, 0.0, TUNNEL_CENTER_Z);

    let tray_slide_clearance = centered_cube(
        "cassette_shuttle_tray_slide_clearance",
        TRAY_X + 36.0,
        AIRLOCK_Y + 6.0,
        TRAY_Z + 26.0,
    )
    .translate(0.0, 0.0, TRAY_CENTER_Z + 5.0);

    let roof_port_cuts = roof_port_cutouts();

    shell - tunnel - tray_slide_clearance - roof_port_cuts
        + end_gasket_land("incubator").translate(
            0.0,
            front_door_y() + DOOR_THICKNESS / 2.0,
            TUNNEL_CENTER_Z,
        )
        + end_gasket_land("isolator").translate(
            0.0,
            rear_door_y() - DOOR_THICKNESS / 2.0,
            TUNNEL_CENTER_Z,
        )
        + exterior_mounting_flanges()
}

fn sealed_cassette_tray() -> Part {
    let tray_body = centered_cube("sealed_cassette_tray_body", TRAY_X, TRAY_Y, TRAY_Z).translate(
        0.0,
        0.0,
        TRAY_CENTER_Z,
    );
    let cassette_recess = centered_cube(
        "sealed_cassette_tray_recess",
        SEALED_CASSETTE_X + CASSETTE_CLEARANCE * 2.0,
        SEALED_CASSETTE_Y + CASSETTE_CLEARANCE * 2.0,
        BASIN_DEPTH + 2.0,
    )
    .translate(
        0.0,
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 - BASIN_DEPTH / 2.0 + 1.0,
    );

    let leak_basin = centered_cube(
        "sealed_cassette_leak_basin_relief",
        SEALED_CASSETTE_X + 34.0,
        SEALED_CASSETTE_Y + 34.0,
        BASIN_DEPTH - 3.0,
    )
    .translate(
        0.0,
        -10.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 - (BASIN_DEPTH - 3.0) / 2.0 + 1.0,
    );

    let left_gripper_clearance = centered_cube(
        "left_robot_gripper_clearance_cut",
        ROBOT_GRIPPER_SLOT_X,
        ROBOT_GRIPPER_SLOT_Y,
        ROBOT_GRIPPER_SLOT_Z,
    )
    .translate(-(TRAY_X / 2.0 - 20.0), 0.0, TRAY_CENTER_Z + 3.0);
    let right_gripper_clearance = centered_cube(
        "right_robot_gripper_clearance_cut",
        ROBOT_GRIPPER_SLOT_X,
        ROBOT_GRIPPER_SLOT_Y,
        ROBOT_GRIPPER_SLOT_Z,
    )
    .translate(TRAY_X / 2.0 - 20.0, 0.0, TRAY_CENTER_Z + 3.0);

    let drain_slot = centered_cube(
        "sealed_cassette_tray_drain_slot",
        SEALED_CASSETTE_X + 12.0,
        DRAIN_GUTTER_W,
        CONDENSATE_CHANNEL_Z,
    )
    .translate(
        0.0,
        -(SEALED_CASSETTE_Y / 2.0 + 12.0),
        TRAY_CENTER_Z + TRAY_Z / 2.0 - 4.0,
    );

    tray_body
        - cassette_recess
        - leak_basin
        - left_gripper_clearance
        - right_gripper_clearance
        - drain_slot
        + tray_gasket_land()
        + cassette_retainer_lips()
        + latch_pockets()
}

fn dual_door_interlock() -> Part {
    let front = door_leaf("incubator", -1.0, front_door_y(), true);
    let rear = door_leaf("isolator", 1.0, rear_door_y(), false);

    let spine_z = TUNNEL_CENTER_Z + TUNNEL_Z / 2.0 + 26.0;
    let spine = centered_cube(
        "dual_door_mechanical_interlock_spine_placeholder",
        32.0,
        door_center_separation() + 104.0,
        28.0,
    )
    .translate(AIRLOCK_X / 2.0 - 72.0, 0.0, spine_z);

    let front_keeper = centered_cube("incubator_door_interlock_keeper", 70.0, 34.0, 42.0)
        .translate(AIRLOCK_X / 2.0 - 72.0, front_door_y(), spine_z - 4.0);
    let rear_keeper = centered_cube("isolator_door_interlock_keeper", 70.0, 34.0, 42.0).translate(
        AIRLOCK_X / 2.0 - 72.0,
        rear_door_y(),
        spine_z - 4.0,
    );

    front + rear + spine + front_keeper + rear_keeper
}

fn thermal_buffer_plate() -> Part {
    let plate = centered_cube(
        "thermal_buffer_plate_body",
        THERMAL_PLATE_X,
        THERMAL_PLATE_Y,
        THERMAL_PLATE_Z,
    )
    .translate(0.0, THERMAL_PLATE_CENTER_Y, THERMAL_PLATE_CENTER_Z);

    let mut coolant_channels = Part::empty("thermal_buffer_coolant_channel_cuts");
    for i in 0..5 {
        let x = thermal_channel_x(i);
        let channel = centered_cylinder(
            format!("thermal_buffer_channel_{i}"),
            THERMAL_CHANNEL_DIA / 2.0,
            THERMAL_PLATE_Y + 10.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, THERMAL_PLATE_CENTER_Y, THERMAL_PLATE_CENTER_Z);
        coolant_channels = coolant_channels + channel;
    }

    let inlet_boss = centered_cylinder("thermal_buffer_inlet_boss", 15.0, 18.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            -THERMAL_PLATE_X / 2.0 + 54.0,
            THERMAL_PLATE_CENTER_Y - THERMAL_PLATE_Y / 2.0 - 9.0,
            THERMAL_PLATE_CENTER_Z,
        );
    let outlet_boss = centered_cylinder("thermal_buffer_outlet_boss", 15.0, 18.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            THERMAL_PLATE_X / 2.0 - 54.0,
            THERMAL_PLATE_CENTER_Y + THERMAL_PLATE_Y / 2.0 + 9.0,
            THERMAL_PLATE_CENTER_Z,
        );

    let thermal_break_front = centered_cube(
        "thermal_break_slot_front",
        THERMAL_PLATE_X - 94.0,
        8.0,
        THERMAL_PLATE_Z + 2.0,
    )
    .translate(0.0, -THERMAL_PLATE_Y / 2.0 + 36.0, THERMAL_PLATE_CENTER_Z);
    let thermal_break_rear = centered_cube(
        "thermal_break_slot_rear",
        THERMAL_PLATE_X - 94.0,
        8.0,
        THERMAL_PLATE_Z + 2.0,
    )
    .translate(0.0, THERMAL_PLATE_Y / 2.0 - 36.0, THERMAL_PLATE_CENTER_Z);

    plate - coolant_channels - thermal_break_front - thermal_break_rear + inlet_boss + outlet_boss
}

fn humidity_condensation_drain_features() -> Part {
    let left_gutter = centered_cube(
        "left_condensation_side_gutter",
        DRAIN_GUTTER_W,
        SEALED_CASSETTE_Y + 66.0,
        CONDENSATE_CHANNEL_Z,
    )
    .translate(
        -(SEALED_CASSETTE_X / 2.0 + DRAIN_GUTTER_W / 2.0 + 12.0),
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + CONDENSATE_CHANNEL_Z / 2.0,
    );
    let right_gutter = centered_cube(
        "right_condensation_side_gutter",
        DRAIN_GUTTER_W,
        SEALED_CASSETTE_Y + 66.0,
        CONDENSATE_CHANNEL_Z,
    )
    .translate(
        SEALED_CASSETTE_X / 2.0 + DRAIN_GUTTER_W / 2.0 + 12.0,
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + CONDENSATE_CHANNEL_Z / 2.0,
    );
    let front_cross_gutter = centered_cube(
        "front_condensation_cross_gutter_to_sump",
        SEALED_CASSETTE_X + 72.0,
        DRAIN_GUTTER_W,
        CONDENSATE_CHANNEL_Z,
    )
    .translate(
        0.0,
        -(SEALED_CASSETTE_Y / 2.0 + 28.0),
        TRAY_CENTER_Z + TRAY_Z / 2.0 + CONDENSATE_CHANNEL_Z / 2.0,
    );

    let sump = centered_cube(
        "condensation_and_leak_sump",
        DRAIN_SUMP_X,
        DRAIN_SUMP_Y,
        24.0,
    )
    .translate(
        TRAY_X / 2.0 - DRAIN_SUMP_X / 2.0 - 28.0,
        -(TRAY_Y / 2.0 - DRAIN_SUMP_Y / 2.0 - 24.0),
        TRAY_CENTER_Z + TRAY_Z / 2.0 + 12.0,
    );

    let drain_port = centered_cylinder(
        "condensation_drain_port_placeholder",
        DRAIN_PORT_DIA / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        TRAY_X / 2.0 - DRAIN_SUMP_X / 2.0 - 28.0,
        -TRAY_Y / 2.0 - 8.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + 10.0,
    );

    left_gutter + right_gutter + front_cross_gutter + sump + drain_port
}

fn hepa_vhp_port_placeholders() -> Part {
    let top_z = AIRLOCK_Z / 2.0 + 10.0;
    let incubator_hepa = hepa_filter_placeholder("incubator_hepa_supply", -240.0, -174.0, top_z);
    let isolator_hepa = hepa_filter_placeholder("isolator_hepa_return", 240.0, 174.0, top_z);
    let vhp_inlet = vertical_port_placeholder("vhp_inlet", -92.0, -44.0, top_z, VHP_PORT_DIA);
    let vhp_exhaust =
        vertical_port_placeholder("vhp_exhaust", 0.0, 44.0, top_z, VHP_PORT_DIA + 8.0);
    let vhp_sample = vertical_port_placeholder("vhp_sample", 96.0, -44.0, top_z, 16.0);

    incubator_hepa + isolator_hepa + vhp_inlet + vhp_exhaust + vhp_sample
}

fn scan_land_and_datum_handoff() -> Part {
    let scan_land = centered_cube("rfid_barcode_scan_land", 168.0, 6.0, 54.0).translate(
        -AIRLOCK_X / 2.0 - 3.0,
        -90.0,
        TUNNEL_CENTER_Z + 8.0,
    );
    let barcode_strip = centered_cube("barcode_window_recess_placeholder", 112.0, 8.0, 18.0)
        .translate(-AIRLOCK_X / 2.0 - 8.0, -90.0, TUNNEL_CENTER_Z + 26.0);
    let rfid_patch = centered_cube("rfid_patch_land", 62.0, 8.0, 34.0).translate(
        -AIRLOCK_X / 2.0 - 8.0,
        -90.0,
        TUNNEL_CENTER_Z - 20.0,
    );

    scan_land - barcode_strip + rfid_patch + datum_rails_for_handoff() + fiducial_targets()
}

fn robot_and_service_keepouts() -> Part {
    let left_gripper = centered_cube(
        "left_robot_gripper_keepout",
        ROBOT_GRIPPER_SLOT_X,
        ROBOT_GRIPPER_SLOT_Y,
        ROBOT_GRIPPER_SLOT_Z,
    )
    .translate(-(TRAY_X / 2.0 + 18.0), 0.0, TRAY_CENTER_Z + 8.0);
    let right_gripper = centered_cube(
        "right_robot_gripper_keepout",
        ROBOT_GRIPPER_SLOT_X,
        ROBOT_GRIPPER_SLOT_Y,
        ROBOT_GRIPPER_SLOT_Z,
    )
    .translate(TRAY_X / 2.0 + 18.0, 0.0, TRAY_CENTER_Z + 8.0);

    let front_service = centered_cube(
        "incubator_side_service_keepout",
        AIRLOCK_X - 120.0,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        front_door_y() - DOOR_THICKNESS / 2.0 - SERVICE_KEEP_OUT_Y / 2.0,
        TUNNEL_CENTER_Z,
    );
    let rear_service = centered_cube(
        "isolator_side_service_keepout",
        AIRLOCK_X - 120.0,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        rear_door_y() + DOOR_THICKNESS / 2.0 + SERVICE_KEEP_OUT_Y / 2.0,
        TUNNEL_CENTER_Z,
    );
    let roof_service = centered_cube("roof_filter_vhp_service_keepout", 620.0, 380.0, 86.0)
        .translate(0.0, 0.0, AIRLOCK_Z / 2.0 + 58.0);
    let module_handoff = centered_cube(
        "sealed_culture_module_handoff_keepout",
        SEALED_CASSETTE_X + 16.0,
        SEALED_CASSETTE_Y + 16.0,
        MODULE_HANDOFF_CLEARANCE_Z,
    )
    .translate(
        0.0,
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + MODULE_HANDOFF_CLEARANCE_Z / 2.0,
    );

    left_gripper + right_gripper + front_service + rear_service + roof_service + module_handoff
}

fn end_gasket_land(name: &str) -> Part {
    rectangular_frame_xz(
        format!("{name}_door_body_gasket_land"),
        TUNNEL_X + 56.0,
        GASKET_Y,
        TUNNEL_Z + 46.0,
        TUNNEL_X + 10.0,
        TUNNEL_Z + 8.0,
    )
}

fn door_leaf(name: &str, face_sign: f64, y_center: f64, include_observation_window: bool) -> Part {
    let plate = centered_cube(format!("{name}_door_plate"), DOOR_X, DOOR_THICKNESS, DOOR_Z)
        .translate(0.0, y_center, TUNNEL_CENTER_Z);
    let chamber_side = -face_sign;
    let gasket = rectangular_frame_xz(
        format!("{name}_door_gasket_land"),
        TUNNEL_X + 44.0,
        6.0,
        TUNNEL_Z + 36.0,
        TUNNEL_X + 6.0,
        TUNNEL_Z + 6.0,
    )
    .translate(
        0.0,
        y_center + chamber_side * (DOOR_THICKNESS / 2.0 + 3.0),
        TUNNEL_CENTER_Z,
    );

    let observation_window = if include_observation_window {
        centered_cube(
            format!("{name}_condensation_observation_window_cut"),
            235.0,
            DOOR_THICKNESS + 4.0,
            54.0,
        )
        .translate(-120.0, y_center, TUNNEL_CENTER_Z + 18.0)
    } else {
        Part::empty(format!("{name}_no_observation_window"))
    };

    plate - observation_window
        + gasket
        + hinge_barrels(name, face_sign, y_center)
        + latch_and_door_sensor_blocks(name, face_sign, y_center)
}

fn hinge_barrels(name: &str, face_sign: f64, y_center: f64) -> Part {
    let x = -DOOR_X / 2.0 + 34.0;
    let y = y_center + face_sign * (DOOR_THICKNESS / 2.0 + 12.0);
    let mut barrels = Part::empty(format!("{name}_hinge_barrels"));

    for (i, z) in [
        TUNNEL_CENTER_Z - TUNNEL_Z / 2.0 + 45.0,
        TUNNEL_CENTER_Z,
        TUNNEL_CENTER_Z + TUNNEL_Z / 2.0 - 45.0,
    ]
    .iter()
    .enumerate()
    {
        let barrel = centered_cylinder(format!("{name}_hinge_barrel_{i}"), 13.0, 82.0, 30)
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, *z);
        let pin = centered_cylinder(format!("{name}_hinge_pin_clearance_{i}"), 3.4, 86.0, 20)
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, *z);
        barrels = barrels + (barrel - pin);
    }

    barrels
}

fn latch_and_door_sensor_blocks(name: &str, face_sign: f64, y_center: f64) -> Part {
    let y = y_center + face_sign * (DOOR_THICKNESS / 2.0 + 10.0);
    let latch_x = DOOR_X / 2.0 - 82.0;

    let upper_latch = centered_cube(format!("{name}_upper_latch_placeholder"), 78.0, 24.0, 42.0)
        .translate(latch_x, y, TUNNEL_CENTER_Z + 72.0);
    let lower_latch = centered_cube(format!("{name}_lower_latch_placeholder"), 78.0, 24.0, 42.0)
        .translate(latch_x, y, TUNNEL_CENTER_Z - 72.0);
    let closed_sensor = centered_cube(format!("{name}_door_closed_sensor_land"), 44.0, 18.0, 28.0)
        .translate(latch_x - 86.0, y, TUNNEL_CENTER_Z + TUNNEL_Z / 2.0 - 42.0);
    let lock_pin = centered_cylinder(format!("{name}_interlock_pin_placeholder"), 9.0, 44.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            AIRLOCK_X / 2.0 - 72.0,
            y,
            TUNNEL_CENTER_Z + TUNNEL_Z / 2.0 + 20.0,
        );

    upper_latch + lower_latch + closed_sensor + lock_pin
}

fn tray_gasket_land() -> Part {
    rectangular_frame_xy(
        "sealed_cassette_tray_gasket_land",
        SEALED_CASSETTE_X + 44.0,
        SEALED_CASSETTE_Y + 44.0,
        GASKET_LAND_Z,
        SEALED_CASSETTE_X + 12.0,
        SEALED_CASSETTE_Y + 12.0,
    )
    .translate(0.0, 0.0, TRAY_CENTER_Z + TRAY_Z / 2.0 + GASKET_LAND_Z / 2.0)
}

fn cassette_retainer_lips() -> Part {
    let rear_lip = centered_cube(
        "sealed_cassette_rear_retainer_lip",
        SEALED_CASSETTE_X + 58.0,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        SEALED_CASSETTE_Y / 2.0 + 22.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + 11.0,
    );
    let left_lip = centered_cube(
        "sealed_cassette_left_retainer_lip",
        18.0,
        SEALED_CASSETTE_Y + 34.0,
        20.0,
    )
    .translate(
        -(SEALED_CASSETTE_X / 2.0 + 22.0),
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + 10.0,
    );
    let right_soft_lip = centered_cube(
        "sealed_cassette_right_soft_retainer_lip",
        14.0,
        SEALED_CASSETTE_Y + 34.0,
        15.0,
    )
    .translate(
        SEALED_CASSETTE_X / 2.0 + 20.0,
        0.0,
        TRAY_CENTER_Z + TRAY_Z / 2.0 + 7.5,
    );

    rear_lip + left_lip + right_soft_lip
}

fn latch_pockets() -> Part {
    let mut pockets = Part::empty("sealed_cassette_tray_latch_pockets");
    for (i, (x, y)) in [
        (
            -(SEALED_CASSETTE_X / 2.0 - 46.0),
            -(SEALED_CASSETTE_Y / 2.0 - 42.0),
        ),
        (
            SEALED_CASSETTE_X / 2.0 - 46.0,
            -(SEALED_CASSETTE_Y / 2.0 - 42.0),
        ),
        (
            -(SEALED_CASSETTE_X / 2.0 - 46.0),
            SEALED_CASSETTE_Y / 2.0 - 42.0,
        ),
        (
            SEALED_CASSETTE_X / 2.0 - 46.0,
            SEALED_CASSETTE_Y / 2.0 - 42.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("sealed_cassette_latch_boss_{i}"), 10.0, 16.0, 30)
            .translate(*x, *y, TRAY_CENTER_Z + TRAY_Z / 2.0 + 8.0);
        let screw = centered_cylinder(
            format!("sealed_cassette_latch_screw_{i}"),
            3.2 / 2.0,
            18.0,
            20,
        )
        .translate(*x, *y, TRAY_CENTER_Z + TRAY_Z / 2.0 + 8.0);
        pockets = pockets + (boss - screw);
    }
    pockets
}

fn datum_rails_for_handoff() -> Part {
    let rail_y = SEALED_CASSETTE_Y + 56.0;
    let rail_center_z = TRAY_CENTER_Z + TRAY_Z / 2.0 + RAIL_Z / 2.0;
    let rail_offset_x = SEALED_CASSETTE_X / 2.0 + RAIL_W / 2.0 + 38.0;
    let left = datum_rail("left").translate(-rail_offset_x, 0.0, rail_center_z);
    let right = datum_rail("right").translate(rail_offset_x, 0.0, rail_center_z);
    let rear_stop = centered_cube(
        "sealed_module_handoff_rear_y_datum",
        SEALED_CASSETTE_X + 92.0,
        RAIL_W,
        RAIL_Z,
    )
    .translate(0.0, rail_y / 2.0 - RAIL_W / 2.0, rail_center_z);
    let front_low_stop = centered_cube(
        "sealed_module_handoff_front_low_datum",
        SEALED_CASSETTE_X + 92.0,
        10.0,
        RAIL_Z * 0.45,
    )
    .translate(0.0, -rail_y / 2.0 + 5.0, rail_center_z - RAIL_Z * 0.275);

    left + right + rear_stop + front_low_stop
}

fn datum_rail(name: &str) -> Part {
    let body = centered_cube(
        format!("{name}_sealed_module_handoff_datum_rail"),
        RAIL_W,
        SEALED_CASSETTE_Y + 56.0,
        RAIL_Z,
    );
    let top_relief = centered_cube(
        format!("{name}_sealed_module_handoff_top_relief"),
        RAIL_W + 2.0,
        SEALED_CASSETTE_Y - 18.0,
        10.0,
    )
    .translate(0.0, 0.0, RAIL_Z / 2.0 - 5.0);
    body - top_relief
}

fn fiducial_targets() -> Part {
    let mut targets = Part::empty("shuttle_airlock_fiducials");
    for (i, (x, y)) in [
        (-(TRAY_X / 2.0 - 42.0), TRAY_Y / 2.0 - 42.0),
        (TRAY_X / 2.0 - 42.0, TRAY_Y / 2.0 - 42.0),
        (-(TRAY_X / 2.0 - 42.0), -(TRAY_Y / 2.0 - 42.0)),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(format!("shuttle_airlock_fiducial_disc_{i}"), 8.0, 2.2, 40)
            .translate(*x, *y, TRAY_CENTER_Z + TRAY_Z / 2.0 + RAIL_Z + 1.1);
        let center =
            centered_cylinder(format!("shuttle_airlock_fiducial_center_{i}"), 1.6, 3.0, 20)
                .translate(*x, *y, TRAY_CENTER_Z + TRAY_Z / 2.0 + RAIL_Z + 1.1);
        targets = targets + (disc - center);
    }
    targets
}

fn hepa_filter_placeholder(name: &str, x: f64, y: f64, z: f64) -> Part {
    let filter_body = centered_cube(format!("{name}_rect_filter_cartridge"), 136.0, 118.0, 34.0)
        .translate(x, y, z + 17.0);
    let round_bore = centered_cylinder(
        format!("{name}_round_clear_bore"),
        HEPA_PORT_DIA / 2.0,
        52.0,
        64,
    )
    .translate(x, y, z + 4.0);
    let clamp_flange = centered_cylinder(
        format!("{name}_clamp_flange"),
        (HEPA_PORT_DIA + 34.0) / 2.0,
        10.0,
        64,
    )
    .translate(x, y, z);
    let media_grid = centered_cube(format!("{name}_media_grid_land"), 110.0, 92.0, 4.0).translate(
        x,
        y,
        z + 36.0,
    );

    filter_body + clamp_flange + media_grid - round_bore
}

fn vertical_port_placeholder(name: &str, x: f64, y: f64, z: f64, clear_dia: f64) -> Part {
    let flange = centered_cylinder(
        format!("{name}_tri_clamp_flange"),
        clear_dia / 2.0 + 13.0,
        12.0,
        44,
    )
    .translate(x, y, z);
    let neck = centered_cylinder(
        format!("{name}_neck_placeholder"),
        clear_dia / 2.0 + 5.0,
        34.0,
        44,
    )
    .translate(x, y, z + 20.0);
    let bore = centered_cylinder(format!("{name}_clear_bore"), clear_dia / 2.0, 48.0, 44)
        .translate(x, y, z + 10.0);
    flange + neck - bore
}

fn roof_port_cutouts() -> Part {
    let top_z = AIRLOCK_Z / 2.0 - 2.0;
    let mut cuts = Part::empty("shuttle_airlock_roof_port_cuts");
    for (i, (x, y, dia)) in [
        (-240.0, -174.0, HEPA_PORT_DIA),
        (240.0, 174.0, HEPA_PORT_DIA),
        (-92.0, -44.0, VHP_PORT_DIA),
        (0.0, 44.0, VHP_PORT_DIA + 8.0),
        (96.0, -44.0, 16.0),
    ]
    .iter()
    .enumerate()
    {
        cuts = cuts
            + centered_cylinder(format!("roof_port_cut_{i}"), dia / 2.0, 44.0, 44)
                .translate(*x, *y, top_z);
    }
    cuts
}

fn exterior_mounting_flanges() -> Part {
    let mut flanges = Part::empty("shuttle_airlock_exterior_mounting_flanges");
    for (i, y) in [-(AIRLOCK_Y / 2.0 + 18.0), AIRLOCK_Y / 2.0 + 18.0]
        .iter()
        .enumerate()
    {
        let flange = centered_cube(
            format!("door_frame_mounting_flange_{i}"),
            AIRLOCK_X + 74.0,
            28.0,
            96.0,
        )
        .translate(0.0, *y, TUNNEL_CENTER_Z);
        let opening = centered_cube(
            format!("door_frame_mounting_flange_opening_{i}"),
            TUNNEL_X + 34.0,
            30.0,
            TUNNEL_Z + 22.0,
        )
        .translate(0.0, *y, TUNNEL_CENTER_Z);
        flanges = flanges + (flange - opening);
    }
    flanges
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    height_z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, height_z);
    let inner = centered_cube(format!("{name}_inner"), inner_x, inner_y, height_z + 2.0);
    outer - inner
}

fn rectangular_frame_xz(
    name: impl Into<String>,
    outer_x: f64,
    depth_y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, depth_y, outer_z);
    let inner = centered_cube(format!("{name}_inner"), inner_x, depth_y + 2.0, inner_z);
    outer - inner
}

fn front_door_y() -> f64 {
    -AIRLOCK_Y / 2.0 - DOOR_GAP - DOOR_THICKNESS / 2.0
}

fn rear_door_y() -> f64 {
    AIRLOCK_Y / 2.0 + DOOR_GAP + DOOR_THICKNESS / 2.0
}

fn door_center_separation() -> f64 {
    rear_door_y() - front_door_y()
}

fn thermal_channel_x(index: usize) -> f64 {
    -2.0 * THERMAL_CHANNEL_PITCH + index as f64 * THERMAL_CHANNEL_PITCH
}

fn minimum_robot_gripper_gap() -> f64 {
    TRAY_X - SEALED_CASSETTE_X
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tray_capacity_matches_twenty_chip_sealed_cassette() {
        assert_eq!(COLS * ROWS, 20);
        assert!((ARRAY_X - 526.04).abs() < 0.01);
        assert!((ARRAY_Y - 447.40).abs() < 0.01);
        assert!(TRAY_X > SEALED_CASSETTE_X + 2.0 * TRAY_WALL);
        assert!(TRAY_Y > SEALED_CASSETTE_Y + 2.0 * TRAY_WALL);
        assert!(TUNNEL_X > TRAY_X + 40.0);
    }

    #[test]
    fn dual_door_interlock_separates_incubator_and_isolator_interfaces() {
        assert!(front_door_y() < -AIRLOCK_Y / 2.0);
        assert!(rear_door_y() > AIRLOCK_Y / 2.0);
        assert!(door_center_separation() > SEALED_CASSETTE_Y + 360.0);
        assert!(DOOR_X > TUNNEL_X + 70.0);
        assert!(DOOR_Z > TUNNEL_Z + 60.0);
    }

    #[test]
    fn thermal_plate_and_drain_geometry_cover_condensation_path() {
        assert!(THERMAL_PLATE_X > SEALED_CASSETTE_X + 80.0);
        assert!(THERMAL_PLATE_Y > DRAIN_SUMP_Y + 120.0);
        assert_eq!(thermal_channel_x(0), -thermal_channel_x(4));
        assert!(THERMAL_CHANNEL_DIA < THERMAL_PLATE_Z);
        assert!(DRAIN_PORT_DIA < DRAIN_SUMP_Y / 4.0);
        assert!(BASIN_DEPTH > CONDENSATE_CHANNEL_Z);
    }

    #[test]
    fn robot_and_service_clearances_stay_outside_transfer_path() {
        assert!(minimum_robot_gripper_gap() > ROBOT_GRIPPER_SLOT_X * 2.0);
        assert!(ROBOT_GRIPPER_SLOT_Y > SEALED_CASSETTE_Y / 2.0 - 12.0);
        assert!(MODULE_HANDOFF_CLEARANCE_Z > REVC_TOTAL_HEIGHT + RAIL_Z);
        assert!(SERVICE_KEEP_OUT_Y > DOOR_THICKNESS + 80.0);
        assert!(AIRLOCK_X - TUNNEL_X > 120.0);
        assert!(HEPA_PORT_DIA > VHP_PORT_DIA * 2.0);
    }
}
