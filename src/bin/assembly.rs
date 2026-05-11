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
// - Optical mount on top of heating block (LED + OPT101 detection)
// - Lid on top of enclosure
// - 8 PCR tubes in heating block
//
// All parts are simplified (no internal features like bolt holes, channels,
// or ventilation slots) to keep the assembly lightweight.
//
// Export: output/assembly.stl

/// Simplified enclosure: hollow box with floor
fn build_enclosure() -> Part {
    let outer = centered_cube("enc_outer", OUTER_X, OUTER_Y, OUTER_Z);
    let inner = centered_cube("enc_inner", INNER_X, INNER_Y, WALL_HEIGHT).translate(
        0.0,
        0.0,
        ENCLOSURE_FLOOR / 2.0,
    );
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
            format!("well_{i}"),
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
    let plate = centered_cube("lid_plate", OUTER_X, OUTER_Y, LID_THICKNESS);

    let first_x = first_slot_x();
    let mut holes = Part::empty("lid_holes");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            format!("lid_hole_{i}"),
            LID_TUBE_HOLE_DIAMETER / 2.0,
            LID_THICKNESS + 2.0,
            24,
        )
        .translate(x, shelf_center_y(), 0.0);
        holes = holes + hole;
    }
    plate - holes
}

/// Simplified optical mount: rectangular bar above heating block
fn build_optical_mount() -> Part {
    centered_cube(
        "optical_mount",
        HEATER_ZONE_LENGTH,
        OPTICAL_MOUNT_WIDTH,
        OPTICAL_MOUNT_HEIGHT,
    )
}

/// Simplified PCR tubes: thin cylinders representing 8 tubes in the holder
fn build_tubes() -> Part {
    let tube_length = 20.0; // visible portion above holder
    let first_x = first_slot_x();
    let mut tubes = Part::empty("tubes");

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let tube = centered_cylinder(format!("tube_{i}"), TUBE_OD / 2.0, tube_length, 24)
            .translate(x, 0.0, 0.0);
        tubes = tubes + tube;
    }
    tubes
}

fn main() {
    // ── Position all parts relative to enclosure at origin ──

    // Enclosure at origin
    let enclosure = build_enclosure();

    // PCB on enclosure floor (spans full enclosure width)
    let pcb_z = floor_z() + PCB_THICKNESS / 2.0;
    let pcb = build_pcb().translate(0.0, 0.0, pcb_z);

    // Aluminum heating block on PCB, centered on shelf
    let block_z = pcb_z + PCB_THICKNESS / 2.0 + BLOCK_HEIGHT / 2.0;
    let heating_block = build_heating_block().translate(0.0, shelf_center_y(), block_z);

    // Optical mount sits on top of the heating block
    let optical_z = block_z + BLOCK_HEIGHT / 2.0 + OPTICAL_MOUNT_HEIGHT / 2.0;
    let optical_mount = build_optical_mount().translate(0.0, shelf_center_y(), optical_z);

    // Lid sits on top of the enclosure
    let lid_z = OUTER_Z / 2.0 + LID_THICKNESS / 2.0;
    let lid = build_lid().translate(0.0, 0.0, lid_z);

    // Tubes extend upward from the heating block (through optical mount)
    let tube_visible_length = 20.0;
    let tube_z = block_z + BLOCK_HEIGHT / 2.0 + tube_visible_length / 2.0;
    let tubes = build_tubes().translate(0.0, shelf_center_y(), tube_z);

    // ── Union all parts ──

    let assembly = enclosure + pcb + heating_block + optical_mount + lid + tubes;

    // ── Export ──

    assembly.write_stl("output/assembly.stl").unwrap();

    println!("Exported: output/assembly.stl");
    println!();
    println!("── LAMP Device v1 Assembly ──");
    println!();
    println!("  Component             Position (X, Y, Z center)");
    println!("  ─────────────────────────────────────────────────");
    println!("  Enclosure             (0, 0, 0)");
    println!("  PCB                   (0, 0, {pcb_z:.1})");
    println!(
        "  Heating block         (0, {:.1}, {block_z:.1})",
        shelf_center_y()
    );
    println!(
        "  Optical mount         (0, {:.1}, {optical_z:.1})",
        shelf_center_y()
    );
    println!("  Lid                   (0, 0, {lid_z:.1})");
    println!(
        "  Tubes (8x)            (0, {:.1}, {tube_z:.1})",
        shelf_center_y()
    );
    println!();
    println!("── Key Dimensions ──");
    println!("  Enclosure:            {OUTER_X:.0}mm x {OUTER_Y:.0}mm x {OUTER_Z:.0}mm");
    println!("  PCB:                  {PCB_LENGTH:.0}mm x {PCB_WIDTH:.0}mm x {PCB_THICKNESS:.1}mm");
    println!("  Heating block:        {BLOCK_LENGTH:.0}mm x {BLOCK_WIDTH:.0}mm x {BLOCK_HEIGHT:.0}mm (aluminum, 8 wells)");
    println!("  Optical mount:        {HEATER_ZONE_LENGTH:.0}mm x {OPTICAL_MOUNT_WIDTH:.0}mm x {OPTICAL_MOUNT_HEIGHT:.0}mm (PETG, LED+OPT101)");
    println!(
        "  Slot spacing:         {SLOT_SPACING:.0}mm center-to-center (12mm for OPT101P DIP-8)"
    );
    println!("  Detection:            Horizontal optical path (LED -> tube -> OPT101P)");
    println!("  Purpose:              Visualization only (not for printing)");
}
