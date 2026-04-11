use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Heated Lid ───
//
// Heated lid assembly for v1 LAMP device. Replaces mineral oil overlay
// for evaporation prevention. Aluminum heated plate at ~105°C contacts
// 0.2mL PCR tube caps via a compressible silicone foam pad, preventing
// condensation on tube walls during 65°C LAMP incubation.
//
// Same principle as PCR thermal cycler heated lids: keeping the lid
// hotter than the reaction temperature forces condensation back into
// the liquid rather than collecting on the cap.
//
// Architecture:
//   - CNC aluminum plate with Kapton heater on top + thermistor pocket
//   - 2mm silicone foam pad on bottom face for compliant tube-cap contact
//   - Hinged to enclosure back wall via M3 pin
//   - Magnetic closure on front edge (2× 6×3mm neodymium disc magnets)
//   - Insulating PEEK or PTFE spacer ring prevents heat loss to enclosure
//
// Exports:
//   output/heated_lid_plate.stl — aluminum heated plate
//   output/heated_lid_frame.stl — 3D-printed hinge frame (PETG)
//
// Ticket: T-2BE16750

fn main() {
    build_heated_plate();
    build_hinge_frame();
}

/// Aluminum heated plate — the thermal core of the lid.
///
/// Kapton heater (25×50mm, 12V 15W) adheres to the top face recess.
/// Thermistor inserts from the front edge at plate mid-height.
/// Bottom face is flat — silicone foam pad (not modeled) adheres here
/// and provides compliant contact with PCR tube caps.
fn build_heated_plate() {
    // ── Plate dimensions ──
    // Sized to span the 8-tube array with margin for heat uniformity.
    // Matches heating block length/width for alignment.

    let plate_length = BLOCK_LENGTH; // 84mm (spans all 8 tube positions)
    let plate_width = BLOCK_WIDTH + 4.0; // 26mm (wider for edge heat uniformity)
    let plate_height = 6.0; // mm (enough thermal mass for stable 105°C)

    // ── Kapton heater recess (top face) ──
    // Shallow pocket on top face for adhesive Kapton heater pad.
    // 25×50mm heater, 0.3mm thick, centered on plate.
    let heater_pad_x = 50.0; // mm
    let heater_pad_y = 25.0; // mm
    let heater_recess_depth = 0.5; // mm (heater sits flush, held by adhesive)

    // ── Thermistor pocket (from front face) ──
    // 3mm NTC bead thermistor, measures plate temperature for PID control.
    // Drilled from front face (-Y), 10mm deep, at plate mid-height.
    let therm_d = 3.0; // mm
    let therm_depth = 10.0; // mm (from front face)
    let therm_z = 0.0; // plate mid-height

    // ── Magnet recesses (front edge, bottom face) ──
    // 2× 6mm dia × 3mm deep pockets for neodymium disc magnets.
    // Matching magnets in the enclosure front wall provide closure force.
    let magnet_d = 6.2; // mm (0.2mm clearance for 6mm magnet)
    let magnet_depth = 3.0; // mm
    let magnet_inset_x = 25.0; // mm from center (symmetric)
    let magnet_y = -(plate_width / 2.0) + magnet_d / 2.0 + 1.0; // near front edge

    // ── M3 mounting holes for hinge bracket ──
    // 2× through-holes on back edge (+Y) for attaching to hinge frame
    let hinge_hole_d = 3.2; // M3 clearance
    let hinge_hole_inset_x = 30.0; // mm from center (symmetric)
    let hinge_hole_y = plate_width / 2.0 - 4.0; // near back edge

    // ── Build body ──

    let body = centered_cube("plate_body", plate_length, plate_width, plate_height);

    // ── Kapton heater recess (from top face) ──

    let heater_recess_z = plate_height / 2.0 - heater_recess_depth / 2.0 + 0.1;
    let heater_recess = centered_cube(
        "heater_recess",
        heater_pad_x,
        heater_pad_y,
        heater_recess_depth + 0.2,
    )
    .translate(0.0, 0.0, heater_recess_z);

    // ── Thermistor pocket (from front face -Y) ──

    let therm_center_y = -(plate_width / 2.0) + therm_depth / 2.0;
    let therm_pocket = centered_cylinder(
        "therm_pocket",
        therm_d / 2.0,
        therm_depth + 1.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, therm_z);

    // ── Magnet recesses (bottom face) ──

    let magnet_z = -(plate_height / 2.0) + magnet_depth / 2.0 - 0.1;
    let magnet_1 = centered_cylinder("magnet_1", magnet_d / 2.0, magnet_depth + 0.2, 32)
        .translate(-magnet_inset_x, magnet_y, magnet_z);
    let magnet_2 = centered_cylinder("magnet_2", magnet_d / 2.0, magnet_depth + 0.2, 32)
        .translate(magnet_inset_x, magnet_y, magnet_z);

    // ── Hinge mounting holes (back edge, through-holes) ──

    let hinge_1 = centered_cylinder("hinge_hole_1", hinge_hole_d / 2.0, plate_height + 2.0, 24)
        .translate(-hinge_hole_inset_x, hinge_hole_y, 0.0);
    let hinge_2 = centered_cylinder("hinge_hole_2", hinge_hole_d / 2.0, plate_height + 2.0, 24)
        .translate(hinge_hole_inset_x, hinge_hole_y, 0.0);

    // ── Assemble ──

    let plate = body - heater_recess - therm_pocket - magnet_1 - magnet_2 - hinge_1 - hinge_2;

    // ── Export ──

    plate.write_stl("output/heated_lid_plate.stl").unwrap();

    println!("Exported: output/heated_lid_plate.stl");
    println!();
    println!("── Heated Lid Plate Specs ──");
    println!("  Body:            {plate_length:.0}mm × {plate_width:.0}mm × {plate_height:.0}mm");
    println!("  Material:        6061-T6 Aluminum");
    println!("  Heater recess:   {heater_pad_x:.0}mm × {heater_pad_y:.0}mm × {heater_recess_depth:.1}mm (top face, Kapton 12V 15W)");
    println!("  Thermistor:      {therm_d:.0}mm dia × {therm_depth:.0}mm deep (front face, NTC 10K)");
    println!("  Magnets:         2× {magnet_d:.1}mm dia × {magnet_depth:.0}mm deep (bottom face, front edge)");
    println!("  Hinge holes:     2× M3 ({hinge_hole_d:.1}mm) on back edge");
    println!("  Target temp:     105°C (prevents condensation at 65°C LAMP)");
    println!();
    println!("── Assembly Notes ──");
    println!("  1. Adhere Kapton heater (25×50mm, 12V 15W) into top recess");
    println!("  2. Insert NTC thermistor into front pocket, seal with kapton tape");
    println!("  3. Press-fit 6×3mm neodymium magnets into bottom-face pockets");
    println!("  4. Adhere 2mm silicone foam pad to bottom face (tube contact)");
    println!("     Foam compresses ~1mm under magnetic closure force");
    println!("  5. Bolt to hinge frame via M3 screws on back edge");
}

/// 3D-printed hinge frame — mounts to enclosure back wall, provides
/// pivot point for the heated plate to swing open for tube access.
///
/// Simple U-bracket with M3 hinge pin holes that align with the
/// plate's back-edge mounting holes. Bolts to enclosure with 2× M3.
fn build_hinge_frame() {
    // ── Frame dimensions ──
    let frame_width = 20.0; // mm (Y, depth along enclosure wall)
    let frame_height = 15.0; // mm (Z, tall enough for hinge clearance)
    let frame_length = 70.0; // mm (X, spans between hinge hole positions)
    let wall_t = 3.0; // mm wall thickness

    // ── Hinge pin holes ──
    // M3 holes at the top of each arm, aligned with plate back-edge holes
    let hinge_pin_d = 3.2; // M3 clearance
    let arm_spacing = 60.0; // center-to-center of the two arms
    let pin_z = frame_height / 2.0 - 4.0; // near top of frame

    // ── Enclosure mounting holes ──
    // 2× M3 through-holes at the base for bolting to enclosure back wall
    let mount_d = 3.2;
    let mount_spacing = 50.0;
    let mount_z = -(frame_height / 2.0) + 5.0; // near bottom

    // ── Build frame body (U-bracket) ──

    let outer = centered_cube("frame_outer", frame_length, frame_width, frame_height);
    let inner = centered_cube(
        "frame_inner",
        frame_length - wall_t * 2.0,
        frame_width + 2.0, // open front/back
        frame_height - wall_t,
    )
    .translate(0.0, 0.0, -(wall_t / 2.0));

    // ── Hinge pin holes (through arms, along Y) ──

    let pin_1 = centered_cylinder("hinge_pin_1", hinge_pin_d / 2.0, frame_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-arm_spacing / 2.0, 0.0, pin_z);
    let pin_2 = centered_cylinder("hinge_pin_2", hinge_pin_d / 2.0, frame_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(arm_spacing / 2.0, 0.0, pin_z);

    // ── Enclosure mounting holes (through base, along Y) ──

    let mount_1 = centered_cylinder("mount_1", mount_d / 2.0, frame_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-mount_spacing / 2.0, 0.0, mount_z);
    let mount_2 = centered_cylinder("mount_2", mount_d / 2.0, frame_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(mount_spacing / 2.0, 0.0, mount_z);

    // ── Assemble ──

    let frame = (outer - inner) - pin_1 - pin_2 - mount_1 - mount_2;

    // ── Export ──

    frame.write_stl("output/heated_lid_frame.stl").unwrap();

    println!();
    println!("Exported: output/heated_lid_frame.stl");
    println!();
    println!("── Hinge Frame Specs ──");
    println!("  Body:            {frame_length:.0}mm × {frame_width:.0}mm × {frame_height:.0}mm");
    println!("  Material:        PETG (3D printed)");
    println!("  Hinge pins:      2× M3 ({hinge_pin_d:.1}mm) at {arm_spacing:.0}mm spacing");
    println!("  Enclosure mount: 2× M3 ({mount_d:.1}mm) at {mount_spacing:.0}mm spacing");
    println!("  Wall thickness:  {wall_t:.0}mm");
    println!();
    println!("── Full Assembly ──");
    println!("  1. Bolt hinge frame to enclosure back wall (2× M3)");
    println!("  2. Thread M3×20mm screws through frame hinge holes + plate back holes");
    println!("  3. Plate pivots open for tube insertion, magnets snap it closed");
    println!("  4. Foam pad compresses onto tube caps under magnetic force");
    println!("  5. PID heats plate to 105°C in ~30s (15W into 84g aluminum)");
}
