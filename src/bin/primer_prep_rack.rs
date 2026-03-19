use vcad::{centered_cube, centered_cylinder, Part};

// ─── Primer Prep Rack ───
//
// 3D-printed (PETG) snap-on adapter rack that positions 1.5mL
// microcentrifuge tubes at X-axis positions traversed by the
// dispensing carriage. Used for automated primer resuspension
// and LAMP primer mix preparation.
//
// Layout (8 positions along X):
//   [P1] [P2] [P3] [P4] [P5] [P6] [TE] [OUT]
//    F3   B3  FIP  BIP  LPF  LPB  buf  reservoir
//
// Positions 1-6: 1.5mL microcentrifuge tubes (~10mm OD, snap-fit)
// Position 7: TE buffer source vial (1.5mL tube)
// Position 8: 5mL output reservoir bottle (~16mm OD)
//
// Clips onto the dispensing frame above the tube holder area.
// Removable — swap back to PCR tube mode for testing.

fn main() {
    // ── Dimensions ──

    // Rack body
    let num_small_slots = 7; // positions 1-7: 1.5mL microcentrifuge tubes
    let num_large_slots = 1; // position 8: 5mL reservoir bottle
    let total_slots = num_small_slots + num_large_slots;

    let slot_spacing = 15.0; // center-to-center (wider than PCR tube spacing)
    let rack_length = (total_slots as f64) * slot_spacing + 10.0; // ~130mm
    let rack_width = 25.0; // Y
    let rack_height = 30.0; // Z — tubes sit deep for stability

    // 1.5mL microcentrifuge tube: ~10mm OD, conical bottom
    let micro_tube_od = 10.0;
    let micro_tube_clearance = 0.4; // press-fit but removable
    let micro_hole_d = micro_tube_od + micro_tube_clearance; // 10.4mm
    let micro_hole_depth = 25.0; // tube sits 25mm deep

    // 5mL bottle: ~16mm OD
    let bottle_od = 16.0;
    let bottle_clearance = 0.5;
    let bottle_hole_d = bottle_od + bottle_clearance; // 16.5mm
    let bottle_hole_depth = 25.0;

    // Snap-fit clips for frame attachment
    let clip_width = 8.0;
    let clip_height = 10.0;
    let clip_thickness = 3.0;
    let clip_hook_depth = 2.0;
    let clip_hook_height = 3.0;

    // Label recess on front face
    let label_width = rack_length - 10.0;
    let label_height = 8.0;
    let label_depth = 0.8;

    // ── Build rack body ──

    let body = centered_cube("body", rack_length, rack_width, rack_height);

    // ── Tube holes ──

    let first_slot_x = -((total_slots as f64 - 1.0) * slot_spacing) / 2.0;
    let mut holes = Part::empty("holes");

    // Positions 1-7: microcentrifuge tube holes
    for i in 0..num_small_slots {
        let x = first_slot_x + (i as f64) * slot_spacing;
        let hole = centered_cylinder(
            &format!("micro_hole_{i}"),
            micro_hole_d / 2.0,
            micro_hole_depth + 1.0,
            32,
        )
        .translate(x, 0.0, (rack_height - micro_hole_depth) / 2.0 + 0.5);
        holes = holes + hole;

        // Conical bottom taper for microcentrifuge tube shape
        let cone_top_r = micro_hole_d / 2.0;
        let cone_bottom_r = 2.0; // tapers to ~4mm at bottom
        let cone_height = 8.0;
        // Approximate cone with a cylinder at smaller radius
        let cone = centered_cylinder(
            &format!("micro_cone_{i}"),
            (cone_top_r + cone_bottom_r) / 2.0,
            cone_height,
            32,
        )
        .translate(
            x,
            0.0,
            (rack_height - micro_hole_depth) / 2.0 - cone_height / 2.0 + 0.5,
        );
        holes = holes + cone;
    }

    // Position 8: large bottle hole
    let bottle_x = first_slot_x + (num_small_slots as f64) * slot_spacing;
    let bottle_hole = centered_cylinder(
        "bottle_hole",
        bottle_hole_d / 2.0,
        bottle_hole_depth + 1.0,
        32,
    )
    .translate(
        bottle_x,
        0.0,
        (rack_height - bottle_hole_depth) / 2.0 + 0.5,
    );
    holes = holes + bottle_hole;

    // ── Snap-fit clips (2x, on bottom edges, along Y) ──

    let clip_offset_x = rack_length / 2.0 - 15.0;
    let clip_z = -(rack_height / 2.0) - clip_height / 2.0;

    let clip_body_1 = centered_cube("clip_1", clip_thickness, clip_width, clip_height)
        .translate(clip_offset_x, 0.0, clip_z);
    let clip_hook_1 = centered_cube(
        "clip_hook_1",
        clip_thickness + clip_hook_depth,
        clip_width,
        clip_hook_height,
    )
    .translate(clip_offset_x, 0.0, clip_z - clip_height / 2.0 + clip_hook_height / 2.0);

    let clip_body_2 = centered_cube("clip_2", clip_thickness, clip_width, clip_height)
        .translate(-clip_offset_x, 0.0, clip_z);
    let clip_hook_2 = centered_cube(
        "clip_hook_2",
        clip_thickness + clip_hook_depth,
        clip_width,
        clip_hook_height,
    )
    .translate(
        -clip_offset_x,
        0.0,
        clip_z - clip_height / 2.0 + clip_hook_height / 2.0,
    );

    // ── Label slot on front face ──

    let label_slot = centered_cube("label_slot", label_width, label_depth + 0.1, label_height)
        .translate(
            0.0,
            -(rack_width / 2.0) + label_depth / 2.0,
            -(rack_height / 2.0) + label_height / 2.0 + 3.0,
        );

    // ── Assemble ──

    let rack = (body + clip_body_1 + clip_hook_1 + clip_body_2 + clip_hook_2)
        - holes
        - label_slot;

    // ── Export ──

    rack.write_stl("output/primer_prep_rack.stl").unwrap();

    println!("Exported: output/primer_prep_rack.stl");
    println!();
    println!("── Primer Prep Rack Specs ──");
    println!(
        "  Body:              {rack_length:.0}mm x {rack_width:.0}mm x {rack_height:.0}mm"
    );
    println!("  Small tube slots:  {num_small_slots}x {micro_hole_d:.1}mm dia x {micro_hole_depth:.0}mm deep (1.5mL microcentrifuge)");
    println!(
        "  Bottle slot:       1x {bottle_hole_d:.1}mm dia x {bottle_hole_depth:.0}mm deep (5mL reservoir)"
    );
    println!("  Slot spacing:      {slot_spacing:.0}mm center-to-center");
    println!("  Attachment:        2x snap-fit clips (bottom edges)");
    println!("  Material:          PETG");
    println!();
    println!("  Layout: [F3] [B3] [FIP] [BIP] [LPF] [LPB] [TE] [OUT]");
}
