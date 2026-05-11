use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Tube Holder (Simple Grip Block) ───
//
// 3D-printed (PETG) block that holds 8 PCR tubes upright over the
// copper spreader plate and PCB. All optical detection is handled by
// SMD components on the PCB surface below (vertical fluorescence).
//
// This piece is just a grip block:
// - 8 through-holes (6.45mm dia) for snug tube fit
// - 18mm tall for full tube stability (prevents wobble)
// - 2 M3 screw holes for securing to PCB/spreader
// - No optical features — LEDs and photodiodes are on the PCB
//
// Material: PETG (required — sits on 65°C copper spreader)
// Dimensions: 84mm x 22mm x 18mm
// Export: output/tube_holder.stl

fn main() {
    // ── Body ──

    let body = centered_cube("body", HOLDER_LENGTH, HOLDER_WIDTH, HOLDER_HEIGHT);

    let first_x = first_slot_x();

    // ── Tube through-holes (full height) ──
    let mut holes = centered_cube("h_init", 0.01, 0.01, 0.01);
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            format!("hole_{i}"),
            HOLDER_TUBE_DIAMETER / 2.0,
            HOLDER_HEIGHT + 2.0,
            32,
        )
        .translate(x, 0.0, 0.0);
        holes = holes + hole;
    }

    // ── M3 screw holes (for securing to PCB/spreader) ──
    let screw_hole_l = centered_cylinder(
        "screw_l",
        HOLDER_SCREW_DIAMETER / 2.0,
        HOLDER_HEIGHT + 2.0,
        24,
    )
    .translate(-HOLDER_SCREW_SPACING_X / 2.0, 0.0, 0.0);

    let screw_hole_r = centered_cylinder(
        "screw_r",
        HOLDER_SCREW_DIAMETER / 2.0,
        HOLDER_HEIGHT + 2.0,
        24,
    )
    .translate(HOLDER_SCREW_SPACING_X / 2.0, 0.0, 0.0);

    // ── Assemble ──

    let part = body - holes - screw_hole_l - screw_hole_r;

    // ── Export ──

    part.write_stl("output/tube_holder.stl").unwrap();

    println!("Exported: output/tube_holder.stl");
    println!();
    println!("── Tube Holder Specs ──");
    println!("  Body:         {HOLDER_LENGTH:.1}mm x {HOLDER_WIDTH:.1}mm x {HOLDER_HEIGHT:.1}mm");
    println!("  Type:         Simple grip block (no optical features)");
    println!("  Tube holes:   {NUM_SLOTS}x {HOLDER_TUBE_DIAMETER:.2}mm dia through-holes");
    println!("  Slot spacing: {SLOT_SPACING:.1}mm center-to-center");
    println!(
        "  Screw holes:  2x M3 at X = +/-{:.1}mm",
        HOLDER_SCREW_SPACING_X / 2.0
    );
    println!("  Detection:    Vertical fluorescence on PCB below (not in holder)");
    println!("  Material:     PETG (opaque black, heat resistant)");
}
