use std::f64::consts::TAU;
use vcad::{centered_cube, centered_cylinder, Part};

// Isolator material-transfer hatch concept with RTP alpha-port placeholders,
// VHP service ports, double-door interlock hardware, tray rails, and service tabs.

const BODY_X: f64 = 720.0;
const BODY_Y: f64 = 360.0;
const BODY_Z: f64 = 470.0;
const OPENING_X: f64 = 600.0;
const OPENING_Z: f64 = 350.0;

const DOOR_X: f64 = 670.0;
const DOOR_Y: f64 = 28.0;
const DOOR_Z: f64 = 405.0;
const DOOR_GAP: f64 = 8.0;

const GASKET_Y: f64 = 8.0;
const GASKET_OUTER_X: f64 = 646.0;
const GASKET_OUTER_Z: f64 = 386.0;
const GASKET_INNER_X: f64 = 600.0;
const GASKET_INNER_Z: f64 = 340.0;

const RTP_190_CLEAR_DIA: f64 = 190.0;
const RTP_270_CLEAR_DIA: f64 = 270.0;

fn main() {
    let body = hatch_body();
    body.write_stl("output/aseptic_transfer_hatch_body.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_body.stl");

    let outer_door = door_leaf("outer", -1.0, front_door_y(), true);
    outer_door
        .write_stl("output/aseptic_transfer_hatch_outer_door.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_outer_door.stl");

    let inner_door = door_leaf("inner", 1.0, rear_door_y(), false);
    inner_door
        .write_stl("output/aseptic_transfer_hatch_inner_door.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_inner_door.stl");

    let rtp = rtp_alpha_placeholders();
    rtp.write_stl("output/aseptic_transfer_hatch_rtp_alpha_placeholders.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_rtp_alpha_placeholders.stl");

    let vhp_ports = vhp_service_ports();
    vhp_ports
        .write_stl("output/aseptic_transfer_hatch_vhp_service_ports.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_vhp_service_ports.stl");

    let tray = tray_rails();
    tray.write_stl("output/aseptic_transfer_hatch_tray_rails.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_tray_rails.stl");

    let assembly = hatch_body()
        + door_leaf("outer_assembly", -1.0, front_door_y(), true)
        + door_leaf("inner_assembly", 1.0, rear_door_y(), false)
        + vhp_service_ports()
        + tray_rails();
    assembly
        .write_stl("output/aseptic_transfer_hatch_assembly.stl")
        .unwrap();
    println!("Exported: output/aseptic_transfer_hatch_assembly.stl");

    println!(
        "Aseptic transfer hatch envelope: {BODY_X:.0}mm W x {BODY_Y:.0}mm D x {BODY_Z:.0}mm H"
    );
    println!(
        "Clear transfer tunnel: {OPENING_X:.0}mm W x {OPENING_Z:.0}mm H; doors: {DOOR_X:.0}mm W x {DOOR_Z:.0}mm H x {DOOR_Y:.0}mm thick"
    );
    println!(
        "RTP alpha placeholders: {RTP_190_CLEAR_DIA:.0}mm and {RTP_270_CLEAR_DIA:.0}mm clear diameters"
    );
    println!("VHP ports: inlet, exhaust, and catalyst cartridge service interface.");
}

fn hatch_body() -> Part {
    let shell = centered_cube("transfer_hatch_body_shell", BODY_X, BODY_Y, BODY_Z);
    let tunnel = centered_cube("transfer_hatch_tunnel", OPENING_X, BODY_Y + 4.0, OPENING_Z);

    let mut vhp_wall_cuts = Part::empty("vhp_wall_cuts");
    for (i, (x, y, dia)) in [
        (-110.0, -58.0, 18.0),
        (0.0, -58.0, 24.0),
        (118.0, -58.0, 32.0),
    ]
    .iter()
    .enumerate()
    {
        vhp_wall_cuts = vhp_wall_cuts
            + centered_cylinder(format!("vhp_roof_cut_{i}"), dia / 2.0, 42.0, 36).translate(
                *x,
                *y,
                BODY_Z / 2.0 - 12.0,
            );
    }

    let body_tunnel = shell - tunnel - vhp_wall_cuts;

    body_tunnel
        + body_end_gasket("front").translate(0.0, -BODY_Y / 2.0 - GASKET_Y / 2.0, 0.0)
        + body_end_gasket("rear").translate(0.0, BODY_Y / 2.0 + GASKET_Y / 2.0, 0.0)
        + service_mount_tabs()
        + interlock_spine()
}

fn body_end_gasket(name: &str) -> Part {
    rectangular_frame(
        format!("{name}_body_gasket_land"),
        GASKET_OUTER_X,
        GASKET_Y,
        GASKET_OUTER_Z,
        GASKET_INNER_X,
        GASKET_INNER_Z,
    ) + rectangular_frame(
        format!("{name}_body_gasket_witness_rib"),
        GASKET_OUTER_X - 14.0,
        GASKET_Y + 2.0,
        GASKET_OUTER_Z - 14.0,
        GASKET_INNER_X + 14.0,
        GASKET_INNER_Z + 14.0,
    )
}

fn service_mount_tabs() -> Part {
    let mut tabs = Part::empty("service_mount_tabs");

    for (i, (x_sign, y, z)) in [
        (-1.0, -118.0, -152.0),
        (-1.0, 118.0, 152.0),
        (1.0, -118.0, 152.0),
        (1.0, 118.0, -152.0),
    ]
    .iter()
    .enumerate()
    {
        let tab_x = BODY_X / 2.0 * x_sign + 22.0 * x_sign;
        let tab_body =
            centered_cube(format!("service_tab_{i}"), 44.0, 70.0, 54.0).translate(tab_x, *y, *z);
        let tab_hole = centered_cylinder(format!("service_tab_m6_{i}"), 6.6 / 2.0, 52.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(tab_x, *y, *z);
        tabs = tabs + (tab_body - tab_hole);
    }

    tabs
}

fn interlock_spine() -> Part {
    let spine = centered_cube(
        "double_door_mechanical_interlock_spine",
        34.0,
        BODY_Y + 2.0 * (DOOR_Y + DOOR_GAP) + 42.0,
        18.0,
    )
    .translate(BODY_X / 2.0 - 78.0, 0.0, BODY_Z / 2.0 + 9.0);

    let front_keeper = centered_cube("front_interlock_keeper", 54.0, 36.0, 30.0).translate(
        BODY_X / 2.0 - 78.0,
        front_door_y(),
        BODY_Z / 2.0 + 8.0,
    );
    let rear_keeper = centered_cube("rear_interlock_keeper", 54.0, 36.0, 30.0).translate(
        BODY_X / 2.0 - 78.0,
        rear_door_y(),
        BODY_Z / 2.0 + 8.0,
    );

    spine + front_keeper + rear_keeper
}

fn door_leaf(name: &str, face_sign: f64, y_center: f64, include_rtp: bool) -> Part {
    let plate = centered_cube(format!("{name}_door_plate"), DOOR_X, DOOR_Y, DOOR_Z)
        .translate(0.0, y_center, 0.0);

    let chamber_side = -face_sign;
    let gasket = rectangular_frame(
        format!("{name}_door_gasket_land"),
        GASKET_OUTER_X - 16.0,
        6.0,
        GASKET_OUTER_Z - 16.0,
        GASKET_INNER_X - 10.0,
        GASKET_INNER_Z - 10.0,
    )
    .translate(0.0, y_center + chamber_side * (DOOR_Y / 2.0 + 3.0), 0.0);

    let sight_slot = centered_cube(format!("{name}_sight_slot_cut"), 260.0, DOOR_Y + 4.0, 28.0)
        .translate(-42.0, y_center, -DOOR_Z / 2.0 + 54.0);

    let hinge = hinge_barrels(name, face_sign, y_center);
    let hardware = latch_and_sensor_blocks(name, face_sign, y_center);
    let rtp = if include_rtp {
        rtp_alpha_placeholders()
    } else {
        Part::empty(format!("{name}_no_rtp"))
    };

    plate - sight_slot + gasket + hinge + hardware + rtp
}

fn hinge_barrels(name: &str, face_sign: f64, y_center: f64) -> Part {
    let x = -DOOR_X / 2.0 + 32.0;
    let y = y_center + face_sign * (DOOR_Y / 2.0 + 11.0);
    let mut barrels = Part::empty(format!("{name}_hinge_barrels"));

    for (i, z) in [-136.0, 0.0, 136.0].iter().enumerate() {
        let barrel = centered_cylinder(format!("{name}_hinge_barrel_{i}"), 12.0, 76.0, 28)
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, *z);
        let pin_hole = centered_cylinder(format!("{name}_hinge_pin_{i}"), 3.2, 80.0, 18)
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, *z);
        barrels = barrels + (barrel - pin_hole);
    }

    barrels
}

fn latch_and_sensor_blocks(name: &str, face_sign: f64, y_center: f64) -> Part {
    let y = y_center + face_sign * (DOOR_Y / 2.0 + 9.0);

    let upper_latch = centered_cube(format!("{name}_upper_latch_block"), 74.0, 22.0, 38.0)
        .translate(DOOR_X / 2.0 - 76.0, y, 112.0);
    let lower_latch = centered_cube(format!("{name}_lower_latch_block"), 74.0, 22.0, 38.0)
        .translate(DOOR_X / 2.0 - 76.0, y, -112.0);
    let interlock_key = centered_cube(format!("{name}_interlock_key_block"), 50.0, 24.0, 28.0)
        .translate(BODY_X / 2.0 - 78.0, y, BODY_Z / 2.0 - 8.0);

    let sensor_body = centered_cube(format!("{name}_door_closed_sensor"), 40.0, 18.0, 28.0)
        .translate(DOOR_X / 2.0 - 142.0, y, DOOR_Z / 2.0 - 60.0);
    let sensor_flag = centered_cube(format!("{name}_magnet_flag"), 18.0, 8.0, 46.0).translate(
        DOOR_X / 2.0 - 112.0,
        y + face_sign * 15.0,
        DOOR_Z / 2.0 - 60.0,
    );

    let latch_bores = centered_cylinder(format!("{name}_upper_latch_bore"), 4.2 / 2.0, 28.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(DOOR_X / 2.0 - 76.0, y, 112.0)
        + centered_cylinder(format!("{name}_lower_latch_bore"), 4.2 / 2.0, 28.0, 20)
            .rotate(90.0, 0.0, 0.0)
            .translate(DOOR_X / 2.0 - 76.0, y, -112.0);

    upper_latch + lower_latch + interlock_key + sensor_body + sensor_flag - latch_bores
}

fn rtp_alpha_placeholders() -> Part {
    rtp_alpha_placeholder("rtp_alpha_190", RTP_190_CLEAR_DIA, -213.0, 12.0)
        + rtp_alpha_placeholder("rtp_alpha_270", RTP_270_CLEAR_DIA, 170.0, 0.0)
}

fn rtp_alpha_placeholder(name: &str, clear_dia: f64, x: f64, z: f64) -> Part {
    let face_sign = -1.0;
    let y_face = front_door_y() + face_sign * DOOR_Y / 2.0;
    let flange_depth = 16.0;
    let boss_depth = 12.0;
    let flange_od = clear_dia + 38.0;
    let boss_od = clear_dia + 18.0;
    let bolt_pitch = clear_dia + 24.0;

    let flange = centered_cylinder(format!("{name}_flange"), flange_od / 2.0, flange_depth, 72)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y_face - flange_depth / 2.0, z);

    let boss = centered_cylinder(format!("{name}_boss"), boss_od / 2.0, boss_depth, 72)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y_face - flange_depth - boss_depth / 2.0, z);

    let bore = centered_cylinder(format!("{name}_clear_bore"), clear_dia / 2.0, 34.0, 72)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y_face - 12.0, z);

    let mut bolt_holes = Part::empty(format!("{name}_bolt_holes"));
    for i in 0..12 {
        let angle = i as f64 / 12.0 * TAU;
        let bx = x + angle.cos() * bolt_pitch / 2.0;
        let bz = z + angle.sin() * bolt_pitch / 2.0;
        bolt_holes = bolt_holes
            + centered_cylinder(format!("{name}_m6_bolt_{i}"), 6.6 / 2.0, 36.0, 18)
                .rotate(90.0, 0.0, 0.0)
                .translate(bx, y_face - 12.0, bz);
    }

    flange + boss - bore - bolt_holes
}

fn vhp_service_ports() -> Part {
    let plate_z = BODY_Z / 2.0 + 8.0;
    let plate =
        centered_cube("vhp_service_mount_plate", 330.0, 118.0, 16.0).translate(4.0, -58.0, plate_z);

    let inlet = vertical_port("vhp_inlet", -110.0, -58.0, 18.0, 38.0, plate_z + 18.0);
    let exhaust = vertical_port("vhp_exhaust", 0.0, -58.0, 24.0, 50.0, plate_z + 18.0);
    let catalyst = catalyst_cartridge_mount();

    plate + inlet + exhaust + catalyst
}

fn vertical_port(name: &str, x: f64, y: f64, clear_dia: f64, flange_dia: f64, z: f64) -> Part {
    let flange = centered_cylinder(
        format!("{name}_tri_clamp_flange"),
        flange_dia / 2.0,
        12.0,
        40,
    )
    .translate(x, y, z);
    let neck = centered_cylinder(format!("{name}_hose_neck"), clear_dia / 2.0 + 5.0, 34.0, 40)
        .translate(x, y, z + 17.0);
    let bore = centered_cylinder(format!("{name}_bore"), clear_dia / 2.0, 50.0, 40).translate(
        x,
        y,
        z + 12.0,
    );

    flange + neck - bore
}

fn catalyst_cartridge_mount() -> Part {
    let x = 118.0;
    let y = -58.0;
    let z = BODY_Z / 2.0 + 28.0;
    let pedestal = centered_cube("catalyst_service_pedestal", 82.0, 72.0, 36.0).translate(x, y, z);
    let cartridge_saddle = centered_cylinder("catalyst_cartridge_saddle_cut", 20.0, 86.0, 36)
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, z + 10.0);
    let port_bore = centered_cylinder("catalyst_roof_bore", 32.0 / 2.0, 58.0, 36).translate(
        x,
        y,
        BODY_Z / 2.0 + 18.0,
    );

    let strap_left =
        centered_cube("catalyst_strap_left", 14.0, 86.0, 42.0).translate(x - 44.0, y, z + 18.0);
    let strap_right =
        centered_cube("catalyst_strap_right", 14.0, 86.0, 42.0).translate(x + 44.0, y, z + 18.0);

    pedestal + strap_left + strap_right - cartridge_saddle - port_bore
}

fn tray_rails() -> Part {
    let rail_y = BODY_Y - 74.0;
    let rail_z = -OPENING_Z / 2.0 + 58.0;
    let mut rails = Part::empty("tray_rails");

    for (i, x) in [-232.0, 232.0].iter().enumerate() {
        let rail = centered_cube(format!("tray_side_rail_{i}"), 30.0, rail_y, 16.0)
            .translate(*x, 0.0, rail_z);
        let lip = centered_cube(format!("tray_retainer_lip_{i}"), 18.0, rail_y, 18.0).translate(
            *x - x.signum() * 12.0,
            0.0,
            rail_z + 13.0,
        );
        let front_stop = centered_cube(format!("tray_front_stop_{i}"), 42.0, 18.0, 40.0).translate(
            *x,
            -rail_y / 2.0 + 9.0,
            rail_z + 12.0,
        );
        let rear_stop = centered_cube(format!("tray_rear_stop_{i}"), 42.0, 18.0, 40.0).translate(
            *x,
            rail_y / 2.0 - 9.0,
            rail_z + 12.0,
        );
        rails = rails + rail + lip + front_stop + rear_stop;
    }

    let removable_tray_floor = centered_cube(
        "removable_transfer_tray_reference",
        420.0,
        rail_y - 36.0,
        8.0,
    )
    .translate(0.0, 0.0, rail_z - 10.0);

    rails + removable_tray_floor
}

fn rectangular_frame(
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
    -BODY_Y / 2.0 - DOOR_GAP - DOOR_Y / 2.0
}

fn rear_door_y() -> f64 {
    BODY_Y / 2.0 + DOOR_GAP + DOOR_Y / 2.0
}
