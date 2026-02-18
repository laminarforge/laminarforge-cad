use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Standalone Horizontal Syringe Pump ───
//
// Open-source syringe pump for microfluidic chip work, based on
// Wijnen et al. (2014). Horizontal configuration with dual linear
// rods and a T8 lead screw driving a pusher carriage.
//
// Exports 4 STL files:
//   1. syringe_pump_base.stl     - Base plate with rod/screw channels
//   2. syringe_pump_motor_end.stl - NEMA17 motor mount end plate
//   3. syringe_pump_syringe_end.stl - Syringe holder end plate
//   4. syringe_pump_pusher.stl   - Linear carriage (pusher block)
//
// Layout (along X axis, horizontal):
//   [Motor End] ──rod──rod──screw── [Pusher] ──syringe── [Syringe End]
//
// Rod spacing: +/-20mm from center in Y
// Screw: centered at Y=0
// Material: PETG

fn main() {
    // ── Shared dimensions ──

    let rod_bore = LINEAR_ROD_DIAMETER + 0.1; // 8.1mm press-fit
    let screw_clearance = LEADSCREW_DIAMETER + 2.0; // 10mm
    let rod_y_offset = 20.0; // rods at +/-20mm from center

    let m3_hole = NEMA17_HOLE_DIAMETER; // 3.2mm

    // ════════════════════════════════════════════════════════════════════
    // 1. BASE PLATE
    // ════════════════════════════════════════════════════════════════════

    let base_length = 250.0; // X
    let base_width = 60.0; // Y
    let base_height = 6.0; // Z

    let base_body = centered_cube("base_body", base_length, base_width, base_height);

    // Rod channels: full-length bores for 8mm rods
    let rod_channel_1 = centered_cylinder(
        "rod_ch_1",
        rod_bore / 2.0,
        base_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, rod_y_offset, 0.0);

    let rod_channel_2 = centered_cylinder(
        "rod_ch_2",
        rod_bore / 2.0,
        base_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -rod_y_offset, 0.0);

    // Screw channel: center, full length
    let screw_channel = centered_cylinder(
        "screw_ch",
        screw_clearance / 2.0,
        base_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let base = base_body - rod_channel_1 - rod_channel_2 - screw_channel;

    base.write_stl("output/syringe_pump_base.stl").unwrap();

    // ════════════════════════════════════════════════════════════════════
    // 2. MOTOR END PLATE
    // ════════════════════════════════════════════════════════════════════

    let me_width = 60.0; // Y
    let me_height = 60.0; // Z
    let me_thickness = 8.0; // X

    let me_body = centered_cube("me_body", me_thickness, me_width, me_height);

    // NEMA17 bolt pattern: 4x M3 at 31mm spacing (square)
    let half_spacing = NEMA17_HOLE_SPACING / 2.0;
    let bolt_positions: [(f64, f64); 4] = [
        (half_spacing, half_spacing),
        (half_spacing, -half_spacing),
        (-half_spacing, half_spacing),
        (-half_spacing, -half_spacing),
    ];

    let mut me_cuts = Part::empty("me_cuts");

    for (i, &(y, z)) in bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("me_bolt_{i}"),
            m3_hole / 2.0,
            me_thickness + 2.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, z);
        me_cuts = me_cuts + hole;
    }

    // Boss recess (NEMA17 locating boss)
    let boss_recess = centered_cylinder(
        "boss_recess",
        NEMA17_BOSS_DIAMETER / 2.0,
        NEMA17_BOSS_HEIGHT + 0.5,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(me_thickness / 2.0) + NEMA17_BOSS_HEIGHT / 2.0, 0.0, 0.0);

    // Shaft hole (7mm clearance for 5mm shaft + coupler)
    let shaft_hole = centered_cylinder(
        "shaft_hole",
        7.0 / 2.0,
        me_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    // Rod bores (press-fit for 8mm rods)
    let me_rod_1 = centered_cylinder(
        "me_rod_1",
        rod_bore / 2.0,
        me_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, rod_y_offset, 0.0);

    let me_rod_2 = centered_cylinder(
        "me_rod_2",
        rod_bore / 2.0,
        me_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -rod_y_offset, 0.0);

    // Screw clearance hole
    let me_screw = centered_cylinder(
        "me_screw",
        screw_clearance / 2.0,
        me_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let motor_end = me_body
        - me_cuts
        - boss_recess
        - shaft_hole
        - me_rod_1
        - me_rod_2
        - me_screw;

    motor_end
        .write_stl("output/syringe_pump_motor_end.stl")
        .unwrap();

    // ════════════════════════════════════════════════════════════════════
    // 3. SYRINGE END PLATE
    // ════════════════════════════════════════════════════════════════════

    let se_width = 60.0; // Y
    let se_height = 40.0; // Z
    let se_thickness = 8.0; // X

    let se_body = centered_cube("se_body", se_thickness, se_width, se_height);

    // Syringe barrel channel: semicircular groove on the +X face
    // 6.8mm diameter (6.5mm barrel + 0.3mm clearance)
    let barrel_channel_d = SYRINGE_BARREL_OD + 0.3; // 6.8mm

    let barrel_channel = centered_cylinder(
        "barrel_channel",
        barrel_channel_d / 2.0,
        se_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    // Flange slot: horizontal slot for syringe finger flanges
    let flange_slot = centered_cube(
        "flange_slot",
        se_thickness + 2.0,
        SYRINGE_FLANGE_WIDTH + 0.3, // 14.3mm
        SYRINGE_FLANGE_THICKNESS + 0.3, // 1.8mm
    );

    // Rod bores (press-fit)
    let se_rod_1 = centered_cylinder(
        "se_rod_1",
        rod_bore / 2.0,
        se_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, rod_y_offset, 0.0);

    let se_rod_2 = centered_cylinder(
        "se_rod_2",
        rod_bore / 2.0,
        se_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -rod_y_offset, 0.0);

    // Screw clearance hole
    let se_screw = centered_cylinder(
        "se_screw",
        screw_clearance / 2.0,
        se_thickness + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let syringe_end = se_body
        - barrel_channel
        - flange_slot
        - se_rod_1
        - se_rod_2
        - se_screw;

    syringe_end
        .write_stl("output/syringe_pump_syringe_end.stl")
        .unwrap();

    // ════════════════════════════════════════════════════════════════════
    // 4. PUSHER CARRIAGE
    // ════════════════════════════════════════════════════════════════════

    let pusher_length = 40.0; // X
    let _pusher_width = 30.0; // Y (narrower than end plates, between rods isn't needed — just wide enough for bearings)
    let pusher_height = 20.0; // Z

    // Expand width to accommodate rod spacing
    let pusher_full_width = rod_y_offset * 2.0 + LM8UU_OD + 6.0; // ~51mm

    let pusher_body = centered_cube("pusher_body", pusher_length, pusher_full_width, pusher_height);

    // LM8UU bearing bores (2x, along X axis)
    let bearing_bore_d = LM8UU_OD + 0.2; // 15.2mm clearance
    let bearing_bore_len = LM8UU_LENGTH + 0.5; // 24.5mm

    let bearing_1 = centered_cylinder(
        "bearing_1",
        bearing_bore_d / 2.0,
        bearing_bore_len,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, rod_y_offset, 0.0);

    let bearing_2 = centered_cylinder(
        "bearing_2",
        bearing_bore_d / 2.0,
        bearing_bore_len,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -rod_y_offset, 0.0);

    // T8 nut pocket (centered, cylindrical recess from bottom)
    let nut_pocket_d = LEADSCREW_NUT_OD + 0.3; // 22.3mm
    let nut_pocket_h = LEADSCREW_NUT_HEIGHT; // 10mm

    let nut_pocket = centered_cylinder(
        "nut_pocket",
        nut_pocket_d / 2.0,
        nut_pocket_h,
        48,
    )
    .translate(0.0, 0.0, -(pusher_height / 2.0) + nut_pocket_h / 2.0);

    // Screw through-hole
    let pusher_screw = centered_cylinder(
        "pusher_screw",
        screw_clearance / 2.0,
        pusher_height + 2.0,
        32,
    );

    // Push pad: cylinder on +X face that contacts the syringe plunger
    let pad_diameter = 10.0;
    let pad_length = 5.0;

    let push_pad = centered_cylinder(
        "push_pad",
        pad_diameter / 2.0,
        pad_length,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(pusher_length / 2.0 + pad_length / 2.0, 0.0, 0.0);

    let pusher = (pusher_body + push_pad)
        - bearing_1
        - bearing_2
        - nut_pocket
        - pusher_screw;

    pusher
        .write_stl("output/syringe_pump_pusher.stl")
        .unwrap();

    // ── Print summary ──

    println!("Exported: output/syringe_pump_base.stl");
    println!("Exported: output/syringe_pump_motor_end.stl");
    println!("Exported: output/syringe_pump_syringe_end.stl");
    println!("Exported: output/syringe_pump_pusher.stl");
    println!();
    println!("── Standalone Syringe Pump Specs ──");
    println!();
    println!("  Base plate:     {base_length:.0}mm x {base_width:.0}mm x {base_height:.0}mm");
    println!("  Rod bores:      2x {rod_bore:.1}mm at +/-{rod_y_offset:.0}mm from center");
    println!("  Screw channel:  {screw_clearance:.0}mm dia (T8 clearance)");
    println!();
    println!("  Motor end:      {me_thickness:.0}mm x {me_width:.0}mm x {me_height:.0}mm");
    println!("  NEMA17 bolts:   4x M3 at {:.0}mm spacing", NEMA17_HOLE_SPACING);
    println!("  Boss recess:    {:.0}mm dia x {:.0}mm", NEMA17_BOSS_DIAMETER, NEMA17_BOSS_HEIGHT);
    println!();
    println!("  Syringe end:    {se_thickness:.0}mm x {se_width:.0}mm x {se_height:.0}mm");
    println!("  Barrel channel: {barrel_channel_d:.1}mm dia (for {:.1}mm syringe)", SYRINGE_BARREL_OD);
    println!("  Flange slot:    {:.1}mm x {:.1}mm", SYRINGE_FLANGE_WIDTH + 0.3, SYRINGE_FLANGE_THICKNESS + 0.3);
    println!();
    println!("  Pusher block:   {pusher_length:.0}mm x {pusher_full_width:.0}mm x {pusher_height:.0}mm");
    println!("  LM8UU bores:    2x {bearing_bore_d:.1}mm dia x {bearing_bore_len:.1}mm");
    println!("  Nut pocket:     {nut_pocket_d:.1}mm dia x {nut_pocket_h:.0}mm");
    println!("  Push pad:       {pad_diameter:.0}mm dia x {pad_length:.0}mm");
    println!();
    println!("  Material:       PETG");
    println!("  Reference:      Wijnen et al. (2014) open-source syringe pump");
}
