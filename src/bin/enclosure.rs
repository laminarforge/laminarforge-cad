use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// PETG enclosure for the sealed-cartridge LAMP/CRISPR fluorescence device.
//
// The reusable enclosure holds the PCB, aluminum heat platen, optical bridge,
// and lid clamp. The sample and reagents stay sealed inside the disposable
// cartridge inserted through the front bay.

fn main() {
    let outer = centered_cube(
        "sealed_cartridge_enclosure_outer",
        OUTER_X,
        OUTER_Y,
        OUTER_Z,
    );
    let inner = centered_cube(
        "sealed_cartridge_enclosure_inner",
        INNER_X,
        INNER_Y,
        WALL_HEIGHT,
    )
    .translate(0.0, 0.0, ENCLOSURE_FLOOR / 2.0);

    let shell = outer - inner;

    let mut cutouts = Part::empty("sealed_cartridge_enclosure_cutouts");
    cutouts = cutouts + cartridge_bay_floor_pocket();
    cutouts = cutouts + front_cartridge_slot();
    cutouts = cutouts + pcb_mount_holes();
    cutouts = cutouts + rear_io_cutouts();
    cutouts = cutouts + front_indicator_holes();
    cutouts = cutouts + side_vent_slots();
    cutouts = cutouts + clamp_boss_holes();

    let enclosure =
        shell + print_brim() + cartridge_rails() + heat_platen_bosses() + sensor_lands() - cutouts;

    enclosure.write_stl("output/enclosure.stl").unwrap();

    println!("Exported: output/enclosure.stl");
    println!();
    println!("-- Sealed Cartridge Enclosure --");
    println!("  Outer:              {OUTER_X:.1}mm x {OUTER_Y:.1}mm x {OUTER_Z:.1}mm");
    println!("  Inner cavity:       {INNER_X:.1}mm x {INNER_Y:.1}mm x {WALL_HEIGHT:.1}mm");
    println!("  Cartridge bay:      {CARTRIDGE_BAY_LENGTH:.1}mm x {CARTRIDGE_BAY_WIDTH:.1}mm");
    println!(
        "  Cartridge slot:     {:.1}mm x {:.1}mm front opening",
        CARTRIDGE_WIDTH + 4.0,
        CARTRIDGE_BODY_HEIGHT + CARTRIDGE_CLEARANCE_Z + 2.0
    );
    println!("  Shelf depth:        {SHELF_DEPTH:.1}mm");
    println!("  Electronics depth:  {ELECTRONICS_DEPTH:.1}mm");
    println!("  Material:           PETG");
}

fn cartridge_bay_floor_pocket() -> Part {
    centered_cube(
        "cartridge_bay_platen_pocket",
        CARTRIDGE_BAY_LENGTH + 2.0,
        CARTRIDGE_BAY_WIDTH + 2.0,
        BLOCK_HEIGHT + PCB_THICKNESS + 2.0,
    )
    .translate(
        0.0,
        shelf_center_y(),
        floor_z() + PCB_THICKNESS + (BLOCK_HEIGHT + 2.0) / 2.0,
    )
}

fn front_cartridge_slot() -> Part {
    centered_cube(
        "front_cartridge_insert_slot",
        CARTRIDGE_WIDTH + 4.0,
        ENCLOSURE_WALL + 2.0,
        CARTRIDGE_BODY_HEIGHT + CARTRIDGE_CLEARANCE_Z + 2.0,
    )
    .translate(
        0.0,
        -OUTER_Y / 2.0,
        floor_z() + PCB_THICKNESS + BLOCK_HEIGHT + CARTRIDGE_BODY_HEIGHT / 2.0 + 1.0,
    )
}

fn pcb_mount_holes() -> Part {
    let mount_spacing_x = PCB_LENGTH - 10.0;
    let mount_spacing_y = PCB_WIDTH - 10.0;
    let mount_depth = ENCLOSURE_FLOOR + 2.0;
    let mut holes = Part::empty("pcb_mount_holes");

    for &dx in &[-mount_spacing_x / 2.0, mount_spacing_x / 2.0] {
        for &dy in &[-mount_spacing_y / 2.0, mount_spacing_y / 2.0] {
            let hole = centered_cylinder(
                format!(
                    "pcb_mount_{}_{}",
                    if dx < 0.0 { "left" } else { "right" },
                    if dy < 0.0 { "front" } else { "rear" }
                ),
                3.2 / 2.0,
                mount_depth,
                24,
            )
            .translate(dx, dy, -(OUTER_Z / 2.0) + mount_depth / 2.0 - 1.0);
            holes = holes + hole;
        }
    }

    holes
}

fn rear_io_cutouts() -> Part {
    let usb = centered_cube("usb_c_cutout", 12.0, ENCLOSURE_WALL + 2.0, 8.0).translate(
        12.0,
        OUTER_Y / 2.0,
        floor_z() + 11.0,
    );
    let barrel = centered_cylinder("barrel_jack_cutout", 6.0, ENCLOSURE_WALL + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-OUTER_X / 4.0, OUTER_Y / 2.0, floor_z() + 11.0);
    let service = centered_cube("debug_header_rear_slot", 18.0, ENCLOSURE_WALL + 2.0, 6.0)
        .translate(OUTER_X / 4.0, OUTER_Y / 2.0, floor_z() + 10.0);

    usb + barrel + service
}

fn front_indicator_holes() -> Part {
    let mut holes = Part::empty("front_indicator_holes");
    for (i, x) in [-18.0, 0.0, 18.0].iter().enumerate() {
        let hole = centered_cylinder(
            format!("front_indicator_{i}"),
            1.5,
            ENCLOSURE_WALL + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -OUTER_Y / 2.0, floor_z() + 23.0);
        holes = holes + hole;
    }
    holes
}

fn side_vent_slots() -> Part {
    let mut vents = Part::empty("side_vent_slots");
    for side in &[-1.0_f64, 1.0] {
        let vx = side * OUTER_X / 2.0;
        for i in 0..6 {
            let slot = centered_cube(
                format!(
                    "side_vent_{}_{}",
                    if *side < 0.0 { "left" } else { "right" },
                    i
                ),
                ENCLOSURE_WALL + 2.0,
                22.0,
                2.0,
            )
            .translate(vx, INNER_Y / 4.0, floor_z() + 10.0 + i as f64 * 4.0);
            vents = vents + slot;
        }
    }
    vents
}

fn clamp_boss_holes() -> Part {
    let mut holes = Part::empty("clamp_boss_holes");
    for &x in &[-42.0, 42.0] {
        for &y in &[
            shelf_center_y() - CARTRIDGE_BAY_WIDTH / 2.0 - 5.0,
            shelf_center_y() + CARTRIDGE_BAY_WIDTH / 2.0 + 5.0,
        ] {
            let hole = centered_cylinder(
                format!("clamp_boss_hole_{x:.0}_{y:.0}"),
                3.2 / 2.0,
                12.0,
                24,
            )
            .translate(x, y, floor_z() + 16.0);
            holes = holes + hole;
        }
    }
    holes
}

fn print_brim() -> Part {
    centered_cube("print_brim", OUTER_X + 10.0, OUTER_Y + 10.0, 0.4).translate(
        0.0,
        0.0,
        -OUTER_Z / 2.0 + 0.2,
    )
}

fn cartridge_rails() -> Part {
    let rail_z = floor_z() + PCB_THICKNESS + BLOCK_HEIGHT + CARTRIDGE_RAIL_HEIGHT / 2.0;
    let rail_y_offset = CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH / 2.0 + 1.5;
    let left = centered_cube(
        "enclosure_left_cartridge_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(0.0, shelf_center_y() - rail_y_offset, rail_z);
    let right = centered_cube(
        "enclosure_right_cartridge_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(0.0, shelf_center_y() + rail_y_offset, rail_z);
    left + right
}

fn heat_platen_bosses() -> Part {
    let mut bosses = Part::empty("heat_platen_mount_bosses");
    for &x in &[-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_X] {
        for &y in &[
            shelf_center_y() - BLOCK_MOUNT_HOLE_Y,
            shelf_center_y() + BLOCK_MOUNT_HOLE_Y,
        ] {
            let boss = centered_cylinder(format!("platen_boss_{x:.0}_{y:.0}"), 4.2, 8.0, 32)
                .translate(x, y, floor_z() + 4.0);
            bosses = bosses + boss;
        }
    }
    bosses
}

fn sensor_lands() -> Part {
    let cartridge_present = centered_cube("cartridge_present_switch_land", 16.0, 8.0, 4.0)
        .translate(
            CARTRIDGE_LENGTH / 2.0 - 12.0,
            shelf_center_y() - CARTRIDGE_WIDTH / 2.0 - 7.0,
            floor_z() + PCB_THICKNESS + BLOCK_HEIGHT + 2.0,
        );
    let lid_switch = centered_cube("lid_closed_switch_land", 14.0, 8.0, 4.0).translate(
        -CARTRIDGE_LENGTH / 2.0 + 12.0,
        shelf_center_y() + CARTRIDGE_WIDTH / 2.0 + 7.0,
        floor_z() + PCB_THICKNESS + BLOCK_HEIGHT + 2.0,
    );
    cartridge_present + lid_switch
}
