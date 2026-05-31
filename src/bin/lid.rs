use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Snap-on enclosure lid for the sealed-cartridge device.
//
// The lid no longer exposes tube holes. It provides a light-shielded cartridge
// service opening, clearance for the optical bridge, and latch/magnet features
// for the heated cartridge clamp.

fn main() {
    let plate = centered_cube(
        "sealed_cartridge_lid_plate",
        OUTER_X,
        OUTER_Y,
        LID_THICKNESS,
    );
    let lip = lid_lip();

    let mut cutouts = Part::empty("sealed_cartridge_lid_cutouts");
    cutouts = cutouts + cartridge_service_window();
    cutouts = cutouts + optical_bridge_clearance();
    cutouts = cutouts + finger_grips();
    cutouts = cutouts + magnet_recesses();

    let lid = plate + lip + light_trap_ribs() - cutouts;

    lid.write_stl("output/lid.stl").unwrap();

    println!("Exported: output/lid.stl");
    println!();
    println!("-- Sealed Cartridge Lid --");
    println!("  Plate:              {OUTER_X:.1}mm x {OUTER_Y:.1}mm x {LID_THICKNESS:.1}mm");
    println!("  Cartridge opening:  {LID_CARTRIDGE_WINDOW_LENGTH:.1}mm x {LID_CARTRIDGE_WINDOW_WIDTH:.1}mm");
    println!(
        "  Optical clearance:  {:.1}mm wide",
        LID_OPTICAL_BRIDGE_CLEARANCE
    );
    println!("  Tube holes:         removed");
    println!("  Material:           opaque PETG");
}

fn lid_lip() -> Part {
    let lip_outer_x = INNER_X - LID_LIP_CLEARANCE * 2.0;
    let lip_outer_y = INNER_Y - LID_LIP_CLEARANCE * 2.0;
    let lip_wall = ENCLOSURE_WALL;
    let lip_inner_x = lip_outer_x - lip_wall * 2.0;
    let lip_inner_y = lip_outer_y - lip_wall * 2.0;
    let lip_z = -(LID_THICKNESS / 2.0) - LID_LIP_DEPTH / 2.0;

    let outer = centered_cube("lid_lip_outer", lip_outer_x, lip_outer_y, LID_LIP_DEPTH)
        .translate(0.0, 0.0, lip_z);
    let inner = centered_cube(
        "lid_lip_inner",
        lip_inner_x,
        lip_inner_y,
        LID_LIP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, lip_z);

    outer - inner
}

fn cartridge_service_window() -> Part {
    centered_cube(
        "cartridge_service_window",
        LID_CARTRIDGE_WINDOW_LENGTH,
        LID_CARTRIDGE_WINDOW_WIDTH,
        LID_THICKNESS + LID_LIP_DEPTH + 2.0,
    )
    .translate(0.0, shelf_center_y(), -LID_LIP_DEPTH / 2.0)
}

fn optical_bridge_clearance() -> Part {
    centered_cube(
        "optical_bridge_clearance_slot",
        OPTICAL_MOUNT_LENGTH + 4.0,
        LID_OPTICAL_BRIDGE_CLEARANCE,
        LID_THICKNESS + 2.0,
    )
    .translate(0.0, shelf_center_y() + REACTION_CHAMBER_CENTER_Y, 0.0)
}

fn finger_grips() -> Part {
    let grip_width = 28.0;
    let grip_depth = 7.0;
    let grip_height = LID_THICKNESS + 2.0;

    let front = centered_cube("front_finger_grip", grip_width, grip_depth, grip_height).translate(
        0.0,
        -OUTER_Y / 2.0 + grip_depth / 2.0 - 1.0,
        0.0,
    );
    let front_round =
        centered_cylinder("front_finger_grip_round", grip_depth / 2.0, grip_width, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, -OUTER_Y / 2.0, 0.0);

    let rear = centered_cube("rear_finger_grip", grip_width, grip_depth, grip_height).translate(
        0.0,
        OUTER_Y / 2.0 - grip_depth / 2.0 + 1.0,
        0.0,
    );
    let rear_round = centered_cylinder("rear_finger_grip_round", grip_depth / 2.0, grip_width, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, OUTER_Y / 2.0, 0.0);

    front + front_round + rear + rear_round
}

fn magnet_recesses() -> Part {
    let mut recesses = Part::empty("lid_magnet_recesses");
    for &x in &[-34.0, 34.0] {
        let recess = centered_cylinder(
            format!(
                "lid_front_magnet_recess_{}",
                if x < 0.0 { "left" } else { "right" }
            ),
            6.2 / 2.0,
            LID_THICKNESS + 0.4,
            32,
        )
        .translate(x, shelf_center_y() - CARTRIDGE_BAY_WIDTH / 2.0 - 4.0, 0.0);
        recesses = recesses + recess;
    }
    recesses
}

fn light_trap_ribs() -> Part {
    let rib_z = -LID_THICKNESS / 2.0 - 1.0;
    let front = centered_cube(
        "front_light_trap_rib",
        LID_CARTRIDGE_WINDOW_LENGTH + 8.0,
        2.0,
        2.0,
    )
    .translate(
        0.0,
        shelf_center_y() - LID_CARTRIDGE_WINDOW_WIDTH / 2.0 - 3.0,
        rib_z,
    );
    let rear = centered_cube(
        "rear_light_trap_rib",
        LID_CARTRIDGE_WINDOW_LENGTH + 8.0,
        2.0,
        2.0,
    )
    .translate(
        0.0,
        shelf_center_y() + LID_CARTRIDGE_WINDOW_WIDTH / 2.0 + 3.0,
        rib_z,
    );
    front + rear
}
