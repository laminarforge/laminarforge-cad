use vcad::{centered_cube, centered_cylinder, Part};

// ─── Aluminum Heating Block for LAMP Device v1 ───
//
// 8-slot block that holds standard 0.2mL PCR tubes.
// Blind holes (not through-holes) — tubes extend above the block
// for horizontal optical reading through clear tube walls.
//
// Includes thermistor hole for PID temperature sensing and
// heater mounting surface on the bottom.

fn make_tube_hole(diameter: f64, depth: f64, index: usize) -> Part {
    centered_cylinder(&format!("tube_hole_{index}"), diameter / 2.0, depth + 1.0, 32)
}

fn make_mount_hole(name: &str, diameter: f64, depth: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, depth + 1.0, 32)
}

fn main() {
    // ── Dimensions (mm) ──

    // PCR tube specs (standard 0.2mL thin-wall)
    let tube_od = 6.35; // outer diameter of tube body
    let tube_hole_diameter = 6.5; // slight clearance for easy insertion
    let num_slots: usize = 8;
    let slot_spacing = 10.0; // center-to-center between slots

    // Block dimensions
    let block_length = (num_slots as f64 - 1.0) * slot_spacing + 14.0; // slots + 7mm wall on each end
    let block_width = 16.0; // enough to surround tubes with thermal mass
    let block_height = 15.0; // tubes sit ~15mm deep, extend above for optical path

    // Tube hole depth (blind holes — don't go all the way through)
    let hole_depth = 13.0; // leaves 2mm of aluminum at the bottom

    // Thermistor hole (drilled from the side into the center of the block)
    let thermistor_hole_diameter = 3.5; // fits standard NTC 10K thermistor

    // Heater mounting holes (M3 screws on bottom face)
    let mount_hole_diameter = 3.2; // M3 clearance
    let mount_hole_depth = 6.0;

    // ── Build the block ──

    let block = centered_cube("block", block_length, block_width, block_height);

    // ── Drill tube holes ──
    // 8 holes in a row along the X axis, centered on the block

    let first_hole_x = -((num_slots as f64 - 1.0) * slot_spacing) / 2.0;

    let mut all_holes = Part::empty("all_tube_holes");
    for i in 0..num_slots {
        let x_offset = first_hole_x + (i as f64 * slot_spacing);
        let hole = make_tube_hole(tube_hole_diameter, hole_depth, i)
            .translate(x_offset, 0.0, (block_height - hole_depth) / 2.0 + 0.5);
        all_holes = all_holes + hole;
    }

    // ── Thermistor hole (from the side, centered vertically) ──
    // Drilled horizontally from one end into the center of the block

    let thermistor_hole = centered_cylinder(
        "thermistor",
        thermistor_hole_diameter / 2.0,
        block_length, // full length, clipped by block boundary
        32,
    )
    .rotate(0.0, 90.0, 0.0) // rotate to horizontal along X axis
    .translate(0.0, 0.0, -2.0); // offset down from center between tube bottoms

    // ── Heater mounting holes (2 holes on bottom face) ──

    let mount_left = make_mount_hole("mount_left", mount_hole_diameter, mount_hole_depth)
        .translate(-block_length / 4.0, 0.0, -block_height / 2.0 + mount_hole_depth / 2.0 - 0.5);

    let mount_right = make_mount_hole("mount_right", mount_hole_diameter, mount_hole_depth)
        .translate(block_length / 4.0, 0.0, -block_height / 2.0 + mount_hole_depth / 2.0 - 0.5);

    // ── Final part ──

    let heating_block = block - all_holes - thermistor_hole - mount_left - mount_right;

    // ── Export ──

    heating_block
        .write_stl("output/heating_block.stl")
        .unwrap();

    println!("Exported: output/heating_block.stl");
    println!();
    println!("── Heating Block Specs ──");
    println!("  Block:       {block_length:.1}mm x {block_width:.1}mm x {block_height:.1}mm");
    println!("  Material:    6061 Aluminum");
    println!("  Tube holes:  {num_slots} x {tube_hole_diameter:.1}mm diameter, {hole_depth:.1}mm deep (blind)");
    println!("  Spacing:     {slot_spacing:.1}mm center-to-center");
    println!(
        "  Tube OD:     {tube_od:.2}mm (holes have {:.2}mm clearance)",
        tube_hole_diameter - tube_od
    );
    println!("  Thermistor:  {thermistor_hole_diameter:.1}mm hole from side, reaches block center");
    println!("  Mounts:      2x M3 holes on bottom face for heater attachment");
}
