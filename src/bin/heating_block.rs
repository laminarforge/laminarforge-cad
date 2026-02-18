use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Aluminum Heating Block ───
//
// CNC-machined aluminum block that holds 8 PCR tubes at 65C for
// LAMP isothermal amplification. The block provides uniform heat
// distribution across all 8 slots.
//
// Features:
// - 8 blind holes for 0.2mL PCR tubes (6.5mm dia, 13mm deep)
//   spaced 10mm center-to-center along the top face
// - 1 heater cartridge bore (6.2mm dia, 70mm deep) from one end,
//   offset below center for optimal heat transfer to tubes
// - 1 thermistor bore (3.5mm dia, 50mm deep) from the same end,
//   on the opposite side of center for accurate temp sensing
//
// Material: 6061 aluminum
// Dimensions: 84mm x 16mm x 15mm

fn main() {
    // ── Block body ──

    let body = centered_cube("body", BLOCK_LENGTH, BLOCK_WIDTH, BLOCK_HEIGHT);

    // ── Tube holes (8x blind holes from top face) ──

    let first_x = first_slot_x();
    let mut tube_holes = centered_cube("th_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("tube_hole_{i}"),
            TUBE_HOLE_DIAMETER / 2.0,
            TUBE_HOLE_DEPTH,
            32,
        )
        .translate(x, 0.0, BLOCK_HEIGHT / 2.0 - TUBE_HOLE_DEPTH / 2.0);
        tube_holes = tube_holes + hole;
    }

    // ── Heater cartridge bore (horizontal, from -X end) ──
    // Center is BORE_Z_OFFSET_FROM_BOTTOM above the block bottom,
    // offset HEATER_Y_OFFSET from center in Y

    let heater_z = -(BLOCK_HEIGHT / 2.0) + BORE_Z_OFFSET_FROM_BOTTOM;

    let heater_bore = centered_cylinder(
        "heater_bore",
        HEATER_BORE_DIAMETER / 2.0,
        HEATER_BORE_DEPTH,
        32,
    )
    .rotate(0.0, 90.0, 0.0) // align along X axis
    .translate(
        -(BLOCK_LENGTH / 2.0) + HEATER_BORE_DEPTH / 2.0,
        HEATER_Y_OFFSET,
        heater_z,
    );

    // ── Thermistor bore (horizontal, from same -X end) ──
    // Same Z offset, opposite Y offset

    let thermistor_bore = centered_cylinder(
        "thermistor_bore",
        THERMISTOR_BORE_DIAMETER / 2.0,
        THERMISTOR_BORE_DEPTH,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -(BLOCK_LENGTH / 2.0) + THERMISTOR_BORE_DEPTH / 2.0,
        THERMISTOR_Y_OFFSET,
        heater_z,
    );

    // ── Assemble ──

    let block = body - tube_holes - heater_bore - thermistor_bore;

    // ── Export ──

    block
        .write_stl("output/heating_block.stl")
        .unwrap();

    println!("Exported: output/heating_block.stl");
    println!();
    println!("── Heating Block Specs ──");
    println!("  Body:           {BLOCK_LENGTH:.1}mm x {BLOCK_WIDTH:.1}mm x {BLOCK_HEIGHT:.1}mm");
    println!("  Tube holes:     {NUM_SLOTS}x {TUBE_HOLE_DIAMETER:.1}mm dia x {TUBE_HOLE_DEPTH:.1}mm deep");
    println!("  Slot spacing:   {SLOT_SPACING:.1}mm center-to-center");
    println!("  Heater bore:    {HEATER_BORE_DIAMETER:.1}mm dia x {HEATER_BORE_DEPTH:.1}mm deep (Y offset: {HEATER_Y_OFFSET:.1}mm)");
    println!("  Thermistor:     {THERMISTOR_BORE_DIAMETER:.1}mm dia x {THERMISTOR_BORE_DEPTH:.1}mm deep (Y offset: {THERMISTOR_Y_OFFSET:.1}mm)");
    println!("  Bore Z offset:  {BORE_Z_OFFSET_FROM_BOTTOM:.1}mm from bottom");
    println!("  Material:       6061 aluminum");
}
