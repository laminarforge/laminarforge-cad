use laminarforge_cad::*;
use vcad::{centered_cube, Part};

// ─── Workbench / Desk Surface Reference Model ───
//
// Simplified visualization model of the workbench that supports the
// workstation enclosure. NOT for 3D printing — this is a reference
// model for assembly visualization and space planning.
//
// Features:
// - Flat desk top surface: WS_TOTAL_WIDTH × WS_DEPTH × 25mm
// - 4 legs: 60×60×750mm (standard desk height ~775mm total)
// - Shelf rail: 1800×100×20mm at 300mm from floor (storage/cable management)
//
// The workstation enclosure sits on top of this desk.
//
// Export: output/workbench.stl

fn main() {
    // ── Dimensions ──

    let top_width = WS_TOTAL_WIDTH; // 1800mm
    let top_depth = WS_DEPTH; // 700mm
    let top_thickness = 25.0;

    let leg_width = 60.0;
    let leg_depth = 60.0;
    let leg_height = 750.0; // standard desk leg height

    let shelf_width = WS_TOTAL_WIDTH; // 1800mm
    let shelf_depth = 100.0;
    let shelf_height = 20.0;
    let shelf_z_from_floor = 300.0; // center of shelf above floor

    // Leg inset from desk edges
    let leg_inset_x = 50.0;
    let leg_inset_y = 50.0;

    // ── Build desk top ──
    // Desk top centered at origin in X/Y, positioned so bottom face
    // is at Z = 0 (legs hang below)

    let top = centered_cube("desk_top", top_width, top_depth, top_thickness).translate(
        0.0,
        0.0,
        top_thickness / 2.0,
    );

    // ── Build legs ──
    // Legs extend downward from the desk top bottom face (Z=0 down to Z=-leg_height)

    let leg_positions: [(f64, f64); 4] = [
        (
            -(top_width / 2.0 - leg_inset_x - leg_width / 2.0),
            -(top_depth / 2.0 - leg_inset_y - leg_depth / 2.0),
        ),
        (
            (top_width / 2.0 - leg_inset_x - leg_width / 2.0),
            -(top_depth / 2.0 - leg_inset_y - leg_depth / 2.0),
        ),
        (
            -(top_width / 2.0 - leg_inset_x - leg_width / 2.0),
            (top_depth / 2.0 - leg_inset_y - leg_depth / 2.0),
        ),
        (
            (top_width / 2.0 - leg_inset_x - leg_width / 2.0),
            (top_depth / 2.0 - leg_inset_y - leg_depth / 2.0),
        ),
    ];

    let mut legs = Part::empty("legs");
    for (i, (lx, ly)) in leg_positions.iter().enumerate() {
        let leg = centered_cube(&format!("leg_{i}"), leg_width, leg_depth, leg_height).translate(
            *lx,
            *ly,
            -(leg_height / 2.0),
        );
        legs = legs + leg;
    }

    // ── Build shelf rail ──
    // Horizontal rail spanning the full width, between front legs
    // Center Z is at -(leg_height) + shelf_z_from_floor (measuring up from floor)

    let floor_z = -leg_height;
    let shelf_center_z = floor_z + shelf_z_from_floor;

    let shelf = centered_cube("shelf_rail", shelf_width, shelf_depth, shelf_height).translate(
        0.0,
        -(top_depth / 2.0 - leg_inset_y - leg_depth / 2.0),
        shelf_center_z,
    );

    // ── Assemble ──

    let workbench = top + legs + shelf;

    // ── Export ──

    workbench.write_stl("output/workbench.stl").unwrap();

    let total_height = leg_height + top_thickness;

    println!("Exported: output/workbench.stl");
    println!();
    println!("── Workbench Specs ──");
    println!("  Desk top:       {top_width:.0}mm x {top_depth:.0}mm x {top_thickness:.0}mm");
    println!("  Total height:   {total_height:.0}mm (top surface above floor)");
    println!("  Legs:           4x {leg_width:.0}mm x {leg_depth:.0}mm x {leg_height:.0}mm");
    println!("  Leg inset:      {leg_inset_x:.0}mm from edges");
    println!("  Shelf rail:     {shelf_width:.0}mm x {shelf_depth:.0}mm x {shelf_height:.0}mm");
    println!("  Shelf height:   {shelf_z_from_floor:.0}mm above floor");
    println!("  Purpose:        Visualization reference (not for printing)");
}
