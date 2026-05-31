use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Aluminum cartridge heat platen.
//
// Replaces the PCR tube well block with a reusable flat thermal interface for a
// sealed disposable cartridge. The disposable cartridge owns the wet path; this
// block only provides heat, registration, and clamp reaction surfaces.

fn main() {
    let body = centered_cube(
        "cartridge_heat_platen_body",
        BLOCK_LENGTH,
        BLOCK_WIDTH,
        BLOCK_HEIGHT,
    );

    let top_z = BLOCK_HEIGHT / 2.0;

    let cartridge_pocket = centered_cube(
        "cartridge_shallow_registration_pocket",
        CARTRIDGE_LENGTH + CARTRIDGE_CLEARANCE_X,
        CARTRIDGE_WIDTH + CARTRIDGE_CLEARANCE_Y,
        BLOCK_CARTRIDGE_POCKET_DEPTH + 0.2,
    )
    .translate(0.0, 0.0, top_z - BLOCK_CARTRIDGE_POCKET_DEPTH / 2.0 + 0.1);

    let thermal_pad_recess = centered_cube(
        "bottom_film_thermal_pad_recess",
        HEATER_ZONE_LENGTH,
        HEATER_ZONE_WIDTH,
        BLOCK_THERMAL_PAD_RECESS_DEPTH + 0.2,
    )
    .translate(
        0.0,
        REACTION_CHAMBER_CENTER_Y,
        top_z - BLOCK_THERMAL_PAD_RECESS_DEPTH / 2.0 + 0.12,
    );

    let heater_z = -(BLOCK_HEIGHT / 2.0) + HEATER_BORE_Z_OFFSET;
    let heater_center_x = -(BLOCK_LENGTH / 2.0) + HEATER_BORE_DEPTH / 2.0;
    let heater_bore = centered_cylinder(
        "cartridge_heater_bore",
        HEATER_BORE_DIAMETER / 2.0,
        HEATER_BORE_DEPTH + 1.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(heater_center_x, REACTION_CHAMBER_CENTER_Y, heater_z);

    let therm_center_y = -(BLOCK_WIDTH / 2.0) + THERMISTOR_BORE_DEPTH / 2.0;
    let therm_pocket = centered_cylinder(
        "cartridge_thermistor_pocket",
        THERMISTOR_BORE_DIAMETER / 2.0,
        THERMISTOR_BORE_DEPTH + 1.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, heater_z);

    let mut cutouts = cartridge_pocket + thermal_pad_recess + heater_bore + therm_pocket;

    for (i, &(mx, my)) in mount_positions().iter().enumerate() {
        let hole = centered_cylinder(
            format!("platen_m3_mount_{i}"),
            BLOCK_MOUNT_HOLE_DIAMETER / 2.0,
            BLOCK_HEIGHT + 2.0,
            24,
        )
        .translate(mx, my, 0.0);
        cutouts = cutouts + hole;
    }

    for (i, &(px, py)) in alignment_pin_positions().iter().enumerate() {
        let pin_hole = centered_cylinder(
            format!("cartridge_alignment_pin_socket_{i}"),
            CARTRIDGE_ALIGNMENT_HOLE_DIAMETER / 2.0,
            5.0,
            24,
        )
        .translate(px, py, top_z - 2.2);
        cutouts = cutouts + pin_hole;
    }

    let platen = body - cutouts + side_rails() + front_stop() + clamp_lands();

    platen.write_stl("output/heating_block.stl").unwrap();

    println!("Exported: output/heating_block.stl");
    println!();
    println!("-- Cartridge Heat Platen Specs --");
    println!(
        "  Body:                 {BLOCK_LENGTH:.0}mm x {BLOCK_WIDTH:.0}mm x {BLOCK_HEIGHT:.0}mm"
    );
    println!("  Material:             6061-T6 aluminum");
    println!(
        "  Cartridge pocket:     {:.1}mm x {:.1}mm x {:.1}mm deep",
        CARTRIDGE_LENGTH + CARTRIDGE_CLEARANCE_X,
        CARTRIDGE_WIDTH + CARTRIDGE_CLEARANCE_Y,
        BLOCK_CARTRIDGE_POCKET_DEPTH
    );
    println!("  Thermal pad recess:   {HEATER_ZONE_LENGTH:.0}mm x {HEATER_ZONE_WIDTH:.0}mm x {BLOCK_THERMAL_PAD_RECESS_DEPTH:.1}mm");
    println!(
        "  Reaction lanes:       {NUM_SLOTS} sealed cartridge windows at {SLOT_SPACING:.0}mm pitch"
    );
    println!(
        "  Heater bore:          {HEATER_BORE_DIAMETER:.1}mm dia x {HEATER_BORE_DEPTH:.0}mm deep"
    );
    println!("  Thermistor pocket:    {THERMISTOR_BORE_DIAMETER:.0}mm dia x {THERMISTOR_BORE_DEPTH:.0}mm deep");
    println!("  Wet path:             disposable cartridge only; no reusable wet plumbing");
}

fn mount_positions() -> [(f64, f64); 4] {
    [
        (-BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
    ]
}

fn alignment_pin_positions() -> [(f64, f64); 4] {
    [
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
    ]
}

fn side_rails() -> Part {
    let rail_z = BLOCK_HEIGHT / 2.0 + CARTRIDGE_RAIL_HEIGHT / 2.0;
    let left = centered_cube(
        "left_cartridge_side_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(
        0.0,
        -(CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH / 2.0 + 1.0),
        rail_z,
    );
    let right = centered_cube(
        "right_cartridge_side_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH / 2.0 + 1.0,
        rail_z,
    );
    left + right
}

fn front_stop() -> Part {
    centered_cube(
        "front_cartridge_insertion_stop",
        CARTRIDGE_LENGTH + 8.0,
        3.0,
        CARTRIDGE_INSERTION_STOP_HEIGHT,
    )
    .translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH + 2.0,
        BLOCK_HEIGHT / 2.0 + CARTRIDGE_INSERTION_STOP_HEIGHT / 2.0,
    )
}

fn clamp_lands() -> Part {
    let land_z = BLOCK_HEIGHT / 2.0 + 1.0;
    let rear = centered_cube("rear_lid_clamp_land", CARTRIDGE_LENGTH + 6.0, 3.0, 2.0).translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 - 2.0,
        land_z,
    );
    let front = centered_cube("front_lid_clamp_land", CARTRIDGE_LENGTH + 6.0, 3.0, 2.0).translate(
        0.0,
        -CARTRIDGE_WIDTH / 2.0 + 2.0,
        land_z,
    );
    rear + front
}
