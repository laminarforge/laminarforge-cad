use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Gantry Brackets: Motor Mounts, Belt Tensioners, Endstop Brackets ───
//
// All structural brackets for the V-slot gantry liquid handler.
// NEMA17 motors drive GT2 belts on X and Y axes.
//
// Ticket: T-A5B39FE0
// Spec: Motor mounts, GT2 belt idlers/tensioners, limit switch mounts. PETG.
//
// Exports:
//   output/gantry_motor_mount.stl — NEMA17 L-bracket for V-slot end
//   output/gantry_belt_tensioner.stl — GT2 idler tensioner
//   output/gantry_endstop_bracket.stl — Microswitch mount for V-slot
//
// All brackets mount to 2040 V-slot extrusion via M5 T-nuts.
// Material: PETG

fn main() {
    build_motor_mount();
    build_belt_tensioner();
    build_endstop_bracket();
}

/// NEMA17 motor mount — L-bracket for V-slot 2040 extrusion end
///
/// One face bolts to the end of a 2040 V-slot extrusion.
/// The perpendicular face holds a NEMA17 motor with GT2 pulley.
/// The motor shaft extends through the bracket into the V-slot channel
/// where the GT2 belt runs.
fn build_motor_mount() {
    // ── V-slot face (mounts to extrusion end) ──
    let vslot_face_w = VSLOT_2040_H + 10.0; // 50mm — wider than extrusion for rigidity
    let vslot_face_h = VSLOT_2040_H + 6.0; // 46mm
    let bracket_thickness = 5.0; // mm wall thickness

    // ── Motor face (perpendicular, holds NEMA17) ──
    let motor_face_w = NEMA17_BODY + 8.0; // 50.3mm
    let motor_face_h = NEMA17_BODY + 8.0; // 50.3mm

    // ── V-slot mounting face ──
    let vslot_face = centered_cube("vslot_face", vslot_face_w, bracket_thickness, vslot_face_h);

    // ── Motor face (extends from top of V-slot face in +Y direction) ──
    let motor_face = centered_cube("motor_face", motor_face_w, motor_face_h, bracket_thickness)
        .translate(
            0.0,
            motor_face_h / 2.0 - bracket_thickness / 2.0,
            vslot_face_h / 2.0 - bracket_thickness / 2.0,
        );

    // ── Gusset (triangular reinforcement between faces) ──
    let gusset_size = 15.0;
    let gusset = centered_cube("gusset", bracket_thickness, gusset_size, gusset_size).translate(
        -(vslot_face_w / 2.0 - bracket_thickness / 2.0),
        gusset_size / 2.0 - bracket_thickness / 2.0,
        vslot_face_h / 2.0 - gusset_size / 2.0 - bracket_thickness,
    );

    let gusset2 = centered_cube("gusset2", bracket_thickness, gusset_size, gusset_size).translate(
        vslot_face_w / 2.0 - bracket_thickness / 2.0,
        gusset_size / 2.0 - bracket_thickness / 2.0,
        vslot_face_h / 2.0 - gusset_size / 2.0 - bracket_thickness,
    );

    // ── V-slot M5 mounting holes (through V-slot face, into extrusion end) ──
    // 2× M5 holes aligned with V-slot channels (at ±10mm from center)
    let mut vslot_holes = Part::empty("vslot_holes");
    let vslot_hole_positions = [
        (0.0, -10.0), // lower channel
        (0.0, 10.0),  // upper channel
    ];

    for (i, (hx, hz)) in vslot_hole_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("vslot_hole_{i}"),
            VSLOT_M5_HOLE / 2.0,
            bracket_thickness + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*hx, 0.0, *hz);
        vslot_holes = vslot_holes + hole;
    }

    // ── NEMA17 bolt holes (4× M3 at 31mm square pattern) ──
    let half_bolt = NEMA17_HOLE_SPACING / 2.0;
    let motor_z = vslot_face_h / 2.0 - bracket_thickness / 2.0; // motor face Z center
    let motor_y = motor_face_h / 2.0 - bracket_thickness / 2.0; // motor face Y center

    let mut motor_holes = Part::empty("motor_holes");
    let bolt_positions = [
        (-half_bolt, -half_bolt),
        (-half_bolt, half_bolt),
        (half_bolt, -half_bolt),
        (half_bolt, half_bolt),
    ];

    for (i, (bx, by)) in bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("motor_bolt_{i}"),
            NEMA17_HOLE_DIAMETER / 2.0,
            bracket_thickness + 2.0,
            24,
        )
        .translate(*bx, motor_y + *by, motor_z);
        motor_holes = motor_holes + hole;
    }

    // ── Boss recess (NEMA17 locating boss) ──
    let boss_d = NEMA17_BOSS_DIAMETER + 0.5;
    let boss_depth = 2.5;
    let boss_recess = centered_cylinder("boss", boss_d / 2.0, boss_depth + 0.1, 48).translate(
        0.0,
        motor_y,
        motor_z - bracket_thickness / 2.0 + boss_depth / 2.0 - 0.05,
    );

    // ── Shaft through-hole ──
    let shaft_hole = centered_cylinder("shaft", 12.0 / 2.0, bracket_thickness + 2.0, 32)
        .translate(0.0, motor_y, motor_z);

    // ── Assemble motor mount ──

    let motor_mount = (vslot_face + motor_face + gusset + gusset2)
        - vslot_holes
        - motor_holes
        - boss_recess
        - shaft_hole;

    motor_mount
        .write_stl("output/gantry_motor_mount.stl")
        .unwrap();

    println!("Exported: output/gantry_motor_mount.stl");
    println!();
    println!("── Gantry Motor Mount ──");
    println!(
        "  V-slot face:    {vslot_face_w:.0}mm x {vslot_face_h:.0}mm x {bracket_thickness:.0}mm"
    );
    println!(
        "  Motor face:     {motor_face_w:.0}mm x {motor_face_h:.0}mm x {bracket_thickness:.0}mm"
    );
    println!("  V-slot holes:   2x M5 ({VSLOT_M5_HOLE:.1}mm)");
    println!("  NEMA17 pattern: 4x M3 at {NEMA17_HOLE_SPACING:.0}mm square");
    println!("  Boss recess:    {boss_d:.1}mm dia x {boss_depth:.1}mm");
    println!("  Shaft hole:     12mm dia (5mm shaft + pulley clearance)");
}

/// GT2 belt tensioner — F625ZZ idler bearing on adjustable mount
///
/// Mounts to V-slot via M5 T-nut. A slot (not hole) allows the idler
/// position to be adjusted for belt tension. The F625ZZ flanged bearing
/// keeps the belt centered.
fn build_belt_tensioner() {
    let body_length = 35.0; // X — long enough for adjustment slot
    let body_width = 20.0; // Y
    let body_height = 15.0; // Z

    // ── Idler bearing bore (F625ZZ) ──
    // Slot for tension adjustment (elongated hole)
    let slot_length = 10.0; // mm adjustment range
    let bearing_bore = F625ZZ_BORE + 0.2; // 5.2mm clearance for M5 bolt
    let flange_recess_d = F625ZZ_FLANGE_OD + 0.5; // 17.5mm
    let flange_recess_depth = 1.0;

    // Idler position: center of adjustment range
    let idler_x = body_length / 2.0 - 10.0; // near +X end
    let _idler_z = body_height / 2.0; // top surface

    // ── V-slot mounting ──
    let vslot_hole_x = -(body_length / 2.0) + 8.0; // near -X end

    // ── Build body ──

    let body = centered_cube("body", body_length, body_width, body_height);

    // Adjustment slot (elongated hole for idler bolt)
    // Create by unioning two cylinders at slot ends + a connecting cube
    let slot_end_1 = centered_cylinder("slot_1", bearing_bore / 2.0, body_height + 2.0, 24)
        .translate(idler_x - slot_length / 2.0, 0.0, 0.0);
    let slot_end_2 = centered_cylinder("slot_2", bearing_bore / 2.0, body_height + 2.0, 24)
        .translate(idler_x + slot_length / 2.0, 0.0, 0.0);
    let slot_bridge = centered_cube("slot_bridge", slot_length, bearing_bore, body_height + 2.0)
        .translate(idler_x, 0.0, 0.0);

    // Flange recess on top surface (so bearing flange sits flush)
    let flange_end_1 = centered_cylinder(
        "flange_1",
        flange_recess_d / 2.0,
        flange_recess_depth + 0.1,
        32,
    )
    .translate(
        idler_x - slot_length / 2.0,
        0.0,
        body_height / 2.0 - flange_recess_depth / 2.0,
    );
    let flange_end_2 = centered_cylinder(
        "flange_2",
        flange_recess_d / 2.0,
        flange_recess_depth + 0.1,
        32,
    )
    .translate(
        idler_x + slot_length / 2.0,
        0.0,
        body_height / 2.0 - flange_recess_depth / 2.0,
    );
    let flange_bridge = centered_cube(
        "flange_bridge",
        slot_length,
        flange_recess_d,
        flange_recess_depth + 0.1,
    )
    .translate(idler_x, 0.0, body_height / 2.0 - flange_recess_depth / 2.0);

    // V-slot mounting hole (M5 through bottom)
    let vslot_hole = centered_cylinder("vslot_mount", VSLOT_M5_HOLE / 2.0, body_height + 2.0, 24)
        .translate(vslot_hole_x, 0.0, 0.0);

    // ── Assemble ──

    let tensioner = body
        - slot_end_1
        - slot_end_2
        - slot_bridge
        - flange_end_1
        - flange_end_2
        - flange_bridge
        - vslot_hole;

    tensioner
        .write_stl("output/gantry_belt_tensioner.stl")
        .unwrap();

    println!();
    println!("Exported: output/gantry_belt_tensioner.stl");
    println!();
    println!("── Gantry Belt Tensioner ──");
    println!("  Body:           {body_length:.0}mm x {body_width:.0}mm x {body_height:.0}mm");
    println!("  Idler bore:     {bearing_bore:.1}mm (F625ZZ, {slot_length:.0}mm adjustment slot)");
    println!("  Flange recess:  {flange_recess_d:.1}mm dia x {flange_recess_depth:.0}mm");
    println!("  V-slot mount:   M5 ({VSLOT_M5_HOLE:.1}mm)");
}

/// Limit switch bracket — microswitch mount for V-slot extrusion
///
/// Small bracket that holds a standard microswitch (20×6.5×10.5mm)
/// on the side of a V-slot extrusion. The switch lever extends into
/// the path of the carriage for homing.
fn build_endstop_bracket() {
    let body_length = 28.0; // X
    let body_width = 15.0; // Y
    let body_height = 22.0; // Z

    // ── Microswitch pocket ──
    // Standard microswitch: ~20×6.5×10.5mm body
    let switch_w = 21.0; // X (with clearance)
    let switch_d = 7.0; // Y depth
    let switch_h = 11.0; // Z

    // Pocket on +Z face (switch lever points up or to the side)
    let switch_pocket = centered_cube("switch_pocket", switch_w, switch_d, switch_h + 0.1)
        .translate(
            0.0,
            -(body_width / 2.0 - switch_d / 2.0 + 0.1),
            body_height / 2.0 - switch_h / 2.0,
        );

    // ── Microswitch M2 mounting holes ──
    let switch_hole_d = 2.2; // M2 clearance
    let switch_hole_spacing = 10.0;
    let switch_z = body_height / 2.0 - switch_h / 2.0;

    let sw_h1 = centered_cylinder("sw_h1", switch_hole_d / 2.0, body_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-switch_hole_spacing / 2.0, 0.0, switch_z);
    let sw_h2 = centered_cylinder("sw_h2", switch_hole_d / 2.0, body_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(switch_hole_spacing / 2.0, 0.0, switch_z);

    // ── V-slot M5 mounting holes ──
    // 2× M5 holes for T-nut mounting to V-slot side channel
    let vslot_h1 = centered_cylinder("vs_h1", VSLOT_M5_HOLE / 2.0, body_width + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, -body_height / 2.0 + 6.0);

    // ── Build ──

    let body = centered_cube("body", body_length, body_width, body_height);

    let bracket = body - switch_pocket - sw_h1 - sw_h2 - vslot_h1;

    bracket
        .write_stl("output/gantry_endstop_bracket.stl")
        .unwrap();

    println!();
    println!("Exported: output/gantry_endstop_bracket.stl");
    println!();
    println!("── Gantry Endstop Bracket ──");
    println!("  Body:           {body_length:.0}mm x {body_width:.0}mm x {body_height:.0}mm");
    println!("  Switch pocket:  {switch_w:.0}mm x {switch_d:.0}mm x {switch_h:.0}mm");
    println!(
        "  Switch holes:   2x M2 ({switch_hole_d:.1}mm) at {switch_hole_spacing:.0}mm spacing"
    );
    println!("  V-slot mount:   1x M5 ({VSLOT_M5_HOLE:.1}mm)");
    println!("  Material:       PETG");
}
