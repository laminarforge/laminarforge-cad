use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Full Device Assembly Visualization ───
//
// Combines simplified versions of all LAMP device v1 parts at their
// correct relative positions for visualization. This is NOT for printing
// — it exists solely to verify that all components fit together correctly.
//
// Structure:
// - Enclosure at origin (reference frame)
// - PCB on enclosure floor
// - Aluminum heating block on PCB (replaces copper spreader + PETG tube holder)
// - Lid on top of enclosure
// - 8 PCR tubes in heating block
// - Dispensing frame above enclosure
// - 2× syringe pump cradles on frame back wall
// - Linear rail near top of frame
// - Dispensing carriage on rail
//
// All parts are simplified (no internal features like bolt holes, channels,
// or ventilation slots) to keep the assembly lightweight.
//
// Export: output/assembly.stl

/// Simplified enclosure: hollow box with floor
fn build_enclosure() -> Part {
    let outer = centered_cube("enc_outer", OUTER_X, OUTER_Y, OUTER_Z);
    let inner = centered_cube("enc_inner", INNER_X, INNER_Y, WALL_HEIGHT)
        .translate(0.0, 0.0, ENCLOSURE_FLOOR / 2.0);
    outer - inner
}

/// Simplified PCB: flat rectangle representing the full circuit board
fn build_pcb() -> Part {
    centered_cube("pcb", PCB_LENGTH, PCB_WIDTH, PCB_THICKNESS)
}

/// Simplified aluminum heating block: body with tube wells
fn build_heating_block() -> Part {
    let body = centered_cube("block", BLOCK_LENGTH, BLOCK_WIDTH, BLOCK_HEIGHT);

    let first_x = first_slot_x();
    let well_z = (BLOCK_HEIGHT - BLOCK_WELL_DEPTH) / 2.0;
    let mut wells = Part::empty("block_wells");

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let well = centered_cylinder(
            &format!("well_{i}"),
            BLOCK_TUBE_DIAMETER / 2.0,
            BLOCK_WELL_DEPTH + 1.0,
            24,
        )
        .translate(x, 0.0, well_z);
        wells = wells + well;
    }
    body - wells
}

/// Simplified lid: flat plate with tube holes
fn build_lid() -> Part {
    let plate = centered_cube(
        "lid_plate",
        OUTER_X,
        OUTER_Y,
        LID_THICKNESS,
    );

    let first_x = first_slot_x();
    let mut holes = Part::empty("lid_holes");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("lid_hole_{i}"),
            LID_TUBE_HOLE_DIAMETER / 2.0,
            LID_THICKNESS + 2.0,
            24,
        )
        .translate(x, shelf_center_y(), 0.0);
        holes = holes + hole;
    }
    plate - holes
}

/// Simplified PCR tubes: thin cylinders representing 8 tubes in the holder
fn build_tubes() -> Part {
    let tube_length = 20.0; // visible portion above holder
    let first_x = first_slot_x();
    let mut tubes = Part::empty("tubes");

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let tube = centered_cylinder(
            &format!("tube_{i}"),
            TUBE_OD / 2.0,
            tube_length,
            24,
        )
        .translate(x, 0.0, 0.0);
        tubes = tubes + tube;
    }
    tubes
}

/// Simplified dispensing frame: open rectangular frame (no floor/ceiling)
fn build_dispensing_frame() -> Part {
    let frame_width = OUTER_X + 20.0; // 120mm
    let frame_depth = OUTER_Y + 4.0;  // 80mm
    let frame_height = 110.0;
    let wall = 4.0;

    let outer = centered_cube("frame_outer", frame_width, frame_depth, frame_height);
    let inner = centered_cube(
        "frame_inner",
        frame_width - wall * 2.0,
        frame_depth - wall * 2.0,
        frame_height + 2.0,
    );
    outer - inner
}

/// Simplified syringe pump cradle: small box
fn build_syringe_cradle() -> Part {
    let body_width = SYRINGE_FLANGE_WIDTH + 6.0; // 20mm
    let body_depth = SYRINGE_BARREL_OD + 6.0;    // 12.5mm
    let body_height = SYRINGE_BARREL_LENGTH + SYRINGE_FLANGE_THICKNESS + 5.0; // ~71.5mm

    centered_cube("cradle", body_width, body_depth, body_height)
}

/// Simplified linear rail: cylinder representing the 8mm rod
fn build_linear_rail(length: f64) -> Part {
    centered_cylinder("linear_rail", LINEAR_ROD_DIAMETER / 2.0, length, 24)
        .rotate(0.0, 90.0, 0.0) // align along X
}

/// Simplified dispensing carriage: small block on the rail
fn build_carriage() -> Part {
    centered_cube("carriage", 35.0, 25.0, 20.0)
}

fn main() {
    // ── Position all parts relative to enclosure at origin ──

    // Enclosure at origin
    let enclosure = build_enclosure();

    // PCB on enclosure floor (spans full enclosure width)
    let pcb_z = floor_z() + PCB_THICKNESS / 2.0;
    let pcb = build_pcb()
        .translate(0.0, 0.0, pcb_z);

    // Aluminum heating block on PCB, centered on shelf
    let block_z = pcb_z + PCB_THICKNESS / 2.0 + BLOCK_HEIGHT / 2.0;
    let heating_block = build_heating_block()
        .translate(0.0, shelf_center_y(), block_z);

    // Lid sits on top of the enclosure
    let lid_z = OUTER_Z / 2.0 + LID_THICKNESS / 2.0;
    let lid = build_lid()
        .translate(0.0, 0.0, lid_z);

    // Tubes extend upward from the heating block
    let tube_visible_length = 20.0;
    let tube_z = block_z + BLOCK_HEIGHT / 2.0 + tube_visible_length / 2.0;
    let tubes = build_tubes()
        .translate(0.0, shelf_center_y(), tube_z);

    // Dispensing frame sits above the enclosure
    let frame_height = 110.0;
    let frame_z = OUTER_Z / 2.0 + frame_height / 2.0;
    let dispensing_frame = build_dispensing_frame()
        .translate(0.0, 0.0, frame_z);

    // Syringe cradles on back wall of frame (2×, spaced 50mm apart)
    let syringe_spacing = 50.0;
    let frame_depth = OUTER_Y + 4.0;
    let cradle_depth = SYRINGE_BARREL_OD + 6.0;

    let cradle_1 = build_syringe_cradle()
        .translate(
            -(syringe_spacing / 2.0),
            frame_depth / 2.0 - cradle_depth / 2.0 - 4.0,
            frame_z - 5.0,
        );

    let cradle_2 = build_syringe_cradle()
        .translate(
            syringe_spacing / 2.0,
            frame_depth / 2.0 - cradle_depth / 2.0 - 4.0,
            frame_z - 5.0,
        );

    // Linear rail near top of frame
    let frame_width = OUTER_X + 20.0;
    let rail_length = frame_width - 20.0; // slightly shorter than frame
    let rail_z = frame_z; // mid-height of frame

    let linear_rail = build_linear_rail(rail_length)
        .translate(0.0, -(frame_depth / 4.0), rail_z);

    // Dispensing carriage on the rail (positioned at center)
    let carriage = build_carriage()
        .translate(0.0, -(frame_depth / 4.0), rail_z);

    // ── Union all parts ──

    let assembly = enclosure
        + pcb
        + heating_block
        + lid
        + tubes
        + dispensing_frame
        + cradle_1
        + cradle_2
        + linear_rail
        + carriage;

    // ── Export ──

    assembly
        .write_stl("output/assembly.stl")
        .unwrap();

    println!("Exported: output/assembly.stl");
    println!();
    println!("── LAMP Device v1 Assembly ──");
    println!();
    println!("  Component             Position (X, Y, Z center)");
    println!("  ─────────────────────────────────────────────────");
    println!("  Enclosure             (0, 0, 0)");
    println!("  PCB                   (0, 0, {pcb_z:.1})");
    println!("  Heating block         (0, {:.1}, {block_z:.1})", shelf_center_y());
    println!("  Lid                   (0, 0, {lid_z:.1})");
    println!("  Tubes (8x)            (0, {:.1}, {tube_z:.1})", shelf_center_y());
    println!("  Dispensing frame      (0, 0, {frame_z:.1})");
    println!("  Syringe cradle L      ({:.1}, back wall, {:.1})", -(syringe_spacing / 2.0), frame_z - 5.0);
    println!("  Syringe cradle R      ({:.1}, back wall, {:.1})", syringe_spacing / 2.0, frame_z - 5.0);
    println!("  Linear rail           (0, {:.1}, {rail_z:.1})", -(frame_depth / 4.0));
    println!("  Carriage              (0, {:.1}, {rail_z:.1})", -(frame_depth / 4.0));
    println!();
    println!("── Key Dimensions ──");
    println!("  Enclosure:            {OUTER_X:.0}mm x {OUTER_Y:.0}mm x {OUTER_Z:.0}mm");
    println!("  PCB:                  {PCB_LENGTH:.0}mm x {PCB_WIDTH:.0}mm x {PCB_THICKNESS:.1}mm");
    println!("  Heating block:        {BLOCK_LENGTH:.0}mm x {BLOCK_WIDTH:.0}mm x {BLOCK_HEIGHT:.0}mm (aluminum, 8 wells)");
    println!("  Frame:                {frame_width:.0}mm x {frame_depth:.0}mm x {frame_height:.0}mm");
    println!("  Total height:         {:.0}mm (enclosure bottom to frame top)", OUTER_Z / 2.0 + frame_z + frame_height / 2.0);
    println!("  Detection:            Horizontal optical path (LED + photodiode above block)");
    println!("  Purpose:              Visualization only (not for printing)");
}
