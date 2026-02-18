use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Syringe Motor Mount ───
//
// 3D-printed (PETG) bracket that sits on top of the dispensing frame
// back wall. Holds a NEMA17 stepper motor (shaft pointing down) that
// drives the T8 lead screw for the syringe plunger pusher.
//
// Also holds the top end of the guide rod (press-fit bore) for
// anti-rotation of the plunger pusher.
//
// The motor shaft couples to the T8 lead screw via a 5mm-to-8mm
// flexible coupler below the plate.
//
// Exports: output/syringe_motor_mount.stl

fn main() {
    // ── Dimensions ──

    // Plate sized to fit NEMA17 bolt pattern + frame mounting
    let plate_width = NEMA17_BODY + 4.0; // 46.3mm
    let plate_depth = 30.0; // spans back wall + overhang
    let plate_thickness = 5.0;

    // Motor bolt pattern (top face)
    let motor_hole_spacing = NEMA17_HOLE_SPACING; // 31mm
    let motor_hole_d = NEMA17_HOLE_DIAMETER; // M3 clearance

    // Boss recess (bottom face, for NEMA17 locating boss)
    let boss_d = NEMA17_BOSS_DIAMETER + 0.5; // 22.5mm
    let boss_depth = NEMA17_BOSS_HEIGHT + 0.5; // 2.5mm

    // Shaft through-hole (center)
    let shaft_hole_d = 7.0; // clearance for 5mm shaft + coupler

    // Guide rod bore (press-fit, holds top of anti-rotation rod)
    let guide_rod_d = SYRINGE_GUIDE_ROD_DIAMETER; // 4mm
    let guide_bore_d = guide_rod_d + 0.1; // tight press-fit
    let guide_offset = SYRINGE_GUIDE_ROD_OFFSET; // 12mm from center

    // Frame mounting holes (bottom face, bolt down into back wall top edge)
    let frame_hole_d = NEMA17_HOLE_DIAMETER; // M3
    let frame_hole_spacing = 25.0; // X spacing between frame bolts

    // ── Build plate ──

    let plate = centered_cube("plate", plate_width, plate_depth, plate_thickness);

    // ── Motor bolt holes (4× M3, from top, through plate) ──

    let half_bolt = motor_hole_spacing / 2.0;
    let bolt_positions: [(f64, f64); 4] = [
        (-half_bolt, -half_bolt),
        (-half_bolt, half_bolt),
        (half_bolt, -half_bolt),
        (half_bolt, half_bolt),
    ];

    let mut motor_bolts = centered_cube("mb_init", 0.01, 0.01, 0.01);
    for (i, (bx, by)) in bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("motor_bolt_{i}"),
            motor_hole_d / 2.0,
            plate_thickness + 2.0,
            24,
        )
        // Motor bolts are in X-Z plane when plate is horizontal
        // but plate is modeled flat (X-Y), so Y here = Z in assembly
        .translate(*bx, *by, 0.0);
        motor_bolts = motor_bolts + hole;
    }

    // ── Boss recess (from bottom face) ──

    let boss_recess = centered_cylinder(
        "boss_recess",
        boss_d / 2.0,
        boss_depth,
        48,
    )
    .translate(0.0, 0.0, -(plate_thickness / 2.0) + boss_depth / 2.0);

    // ── Shaft through-hole (center) ──

    let shaft_hole = centered_cylinder(
        "shaft_hole",
        shaft_hole_d / 2.0,
        plate_thickness + 2.0,
        32,
    );

    // ── Guide rod bore (press-fit, offset from center) ──

    let guide_bore = centered_cylinder(
        "guide_bore",
        guide_bore_d / 2.0,
        plate_thickness + 2.0,
        24,
    )
    .translate(guide_offset, 0.0, 0.0);

    // ── Frame mounting holes (2× M3, through bottom) ──

    let frame_hole_1 = centered_cylinder(
        "frame_hole_0",
        frame_hole_d / 2.0,
        plate_thickness + 2.0,
        24,
    )
    .translate(-(frame_hole_spacing / 2.0), 0.0, 0.0);

    let frame_hole_2 = centered_cylinder(
        "frame_hole_1",
        frame_hole_d / 2.0,
        plate_thickness + 2.0,
        24,
    )
    .translate(frame_hole_spacing / 2.0, 0.0, 0.0);

    // ── Assemble ──

    let mount = plate
        - motor_bolts
        - boss_recess
        - shaft_hole
        - guide_bore
        - frame_hole_1
        - frame_hole_2;

    // ── Export ──

    mount
        .write_stl("output/syringe_motor_mount.stl")
        .unwrap();

    println!("Exported: output/syringe_motor_mount.stl");
    println!();
    println!("── Syringe Motor Mount Specs ──");
    println!("  Plate:          {plate_width:.1}mm × {plate_depth:.1}mm × {plate_thickness:.1}mm");
    println!("  Motor pattern:  4× M3 at {motor_hole_spacing:.0}mm square (NEMA17)");
    println!("  Boss recess:    {boss_d:.1}mm dia × {boss_depth:.1}mm");
    println!("  Shaft hole:     {shaft_hole_d:.1}mm dia (5mm shaft + coupler clearance)");
    println!("  Guide rod bore: {guide_bore_d:.1}mm dia at {guide_offset:.0}mm offset (press-fit for {guide_rod_d:.0}mm rod)");
    println!("  Frame bolts:    2× M3 at {frame_hole_spacing:.0}mm spacing");
    println!("  Mounts:         on top of dispensing frame back wall, motor shaft points down");
    println!("  Material:       PETG");
}
