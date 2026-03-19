use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Dispensing Frame ───
//
// 3D-printed (PETG) structural frame that mounts above the existing
// v1 device enclosure. Holds all dispensing system components:
// - Syringe pump cradles (2×, on back wall)
// - Outlet valve mounts (2×, on back wall)
// - Linear rail mounts (left + right walls)
// - Syringe motor mount plates (2×, on top of back wall)
// - Open front/top for tube access and carriage travel
//
// Bolts onto the existing enclosure through the top edges.

fn main() {
    // ── Dimensions ──

    // Frame sized to match and extend above the existing enclosure
    let frame_width = OUTER_X + 20.0; // 120mm — wider for syringe mounts
    let frame_depth = OUTER_Y + 4.0; // 80mm
    let frame_height = 110.0; // tall enough for syringe + rail + clearance

    let wall = 4.0;
    // Inner cavity (no floor — sits on top of existing enclosure, open bottom)
    let inner_width = frame_width - wall * 2.0;
    let inner_depth = frame_depth - wall * 2.0;

    // Enclosure mating: bolt holes that align with enclosure top edges
    let enclosure_bolt_spacing_x = OUTER_X - 10.0; // ~90mm
    let enclosure_bolt_spacing_y = OUTER_Y - 10.0; // ~66mm
    let bolt_d = NEMA17_HOLE_DIAMETER; // M3

    // Back wall: thicker to support syringe mounts
    let back_wall = 6.0;

    // Syringe spacing (center-to-center between two syringe positions)
    let syringe_spacing = 50.0;

    // Syringe cradle mount holes on back wall
    // Tab offset: cradle body 20mm wide, tabs 4mm thick → tab center at ±12mm from syringe center
    let tab_offset_x = 12.0;
    let syringe_z = -5.0; // centered lower in frame (cradle is ~71.5mm tall)
    let syringe_hole_half_spacing_z = 20.0; // ±20mm = 40mm apart, matching cradle tab holes

    // Motor mount holes on top of back wall (vertical, for syringe motor mount plates)
    let motor_mount_hole_spacing = 25.0; // X spacing per motor mount plate

    // Rail mount holes on side walls
    let rail_z = 0.0; // mid-height for rail mounts
    let rail_hole_spacing_z = 30.0;

    // Tubing pass-through holes in bottom edge (for PTFE to desk-level reservoirs)
    let tube_pass_d = 6.0; // clearance for barbed adapters
    let tube_pass_spacing = 20.0;

    // ── Build frame ──

    // Outer shell (open top, open bottom)
    let outer = centered_cube("outer", frame_width, frame_depth, frame_height);
    let inner = centered_cube("inner", inner_width, inner_depth, frame_height + 2.0);
    let shell = outer - inner;

    // Thicken back wall
    let back_reinforcement = centered_cube(
        "back_reinforce",
        inner_width,
        back_wall - wall,
        frame_height,
    )
    .translate(0.0, frame_depth / 2.0 - wall - (back_wall - wall) / 2.0, 0.0);

    // ── Open front face (large cutout for tube access) ──

    let front_opening = centered_cube(
        "front_opening",
        inner_width - 20.0,
        wall + 2.0,
        frame_height - 20.0,
    )
    .translate(0.0, -(frame_depth / 2.0), 5.0);

    // ── Enclosure mating bolt holes (4× M3, through bottom edge) ──

    let half_bx = enclosure_bolt_spacing_x / 2.0;
    let half_by = enclosure_bolt_spacing_y / 2.0;
    let bolt_z = -(frame_height / 2.0) + 4.0;

    let mut enclosure_bolts = centered_cube("eb_init", 0.01, 0.01, 0.01);
    let bolt_positions: [(f64, f64); 4] = [
        (-half_bx, -half_by),
        (-half_bx, half_by),
        (half_bx, -half_by),
        (half_bx, half_by),
    ];
    for (i, (bx, by)) in bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("enc_bolt_{i}"),
            bolt_d / 2.0,
            12.0,
            24,
        )
        .translate(*bx, *by, bolt_z);
        enclosure_bolts = enclosure_bolts + hole;
    }

    // ── Syringe cradle mount holes on back wall (4 per cradle, through back wall Y-axis) ──

    let back_y = frame_depth / 2.0 - back_wall / 2.0;
    let mut syringe_holes = centered_cube("sh_init", 0.01, 0.01, 0.01);
    for i in 0..NUM_FLUID_CHANNELS {
        let sx = -(syringe_spacing / 2.0) + (i as f64) * syringe_spacing;

        // 4 holes per cradle: left tab (top, bot) + right tab (top, bot)
        for tab_side in [-1.0_f64, 1.0] {
            let tx = sx + tab_side * tab_offset_x;
            for z_sign in [-1.0_f64, 1.0] {
                let sz = syringe_z + z_sign * syringe_hole_half_spacing_z;
                let hole = centered_cylinder(
                    &format!("syr_h_{i}_{}", if tab_side < 0.0 { "l" } else { "r" }),
                    bolt_d / 2.0,
                    back_wall + 2.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(tx, back_y, sz);
                syringe_holes = syringe_holes + hole;
            }
        }
    }

    // ── Motor mount holes on top of back wall (2 per motor, vertical through top edge) ──

    let mut motor_holes = centered_cube("mh_init", 0.01, 0.01, 0.01);
    for i in 0..NUM_FLUID_CHANNELS {
        let sx = -(syringe_spacing / 2.0) + (i as f64) * syringe_spacing;

        let h1 = centered_cylinder(
            &format!("motor_h_{i}_0"),
            bolt_d / 2.0,
            wall + 2.0,
            24,
        )
        .translate(sx - motor_mount_hole_spacing / 2.0, back_y, frame_height / 2.0 - wall / 2.0);

        let h2 = centered_cylinder(
            &format!("motor_h_{i}_1"),
            bolt_d / 2.0,
            wall + 2.0,
            24,
        )
        .translate(sx + motor_mount_hole_spacing / 2.0, back_y, frame_height / 2.0 - wall / 2.0);

        motor_holes = motor_holes + h1 + h2;
    }

    // ── Rail mount holes on side walls ──

    let side_x = frame_width / 2.0 - wall / 2.0;
    let mut rail_holes = centered_cube("rh_init", 0.01, 0.01, 0.01);

    for side in [-1.0_f64, 1.0] {
        let sx = side * side_x;
        for j in 0..2 {
            let rz = rail_z - rail_hole_spacing_z / 2.0 + (j as f64) * rail_hole_spacing_z;
            let hole = centered_cylinder(
                &format!("rail_h_{}_{j}", if side < 0.0 { "l" } else { "r" }),
                bolt_d / 2.0,
                wall + 2.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(sx, 0.0, rz);
            rail_holes = rail_holes + hole;
        }
    }

    // ── Tubing pass-through holes in bottom edge ──

    let mut tube_passes = centered_cube("tp_init", 0.01, 0.01, 0.01);
    for i in 0..NUM_FLUID_CHANNELS {
        let tx = -(tube_pass_spacing / 2.0) + (i as f64) * tube_pass_spacing;
        let hole = centered_cylinder(
            &format!("tube_pass_{i}"),
            tube_pass_d / 2.0,
            wall + 2.0,
            24,
        )
        .translate(tx, frame_depth / 2.0 - back_wall / 2.0, -(frame_height / 2.0) + 4.0);
        tube_passes = tube_passes + hole;
    }

    // ── Assemble ──

    let frame = (shell + back_reinforcement)
        - front_opening
        - enclosure_bolts
        - syringe_holes
        - motor_holes
        - rail_holes
        - tube_passes;

    // ── Export ──

    frame
        .write_stl("output/dispensing_frame.stl")
        .unwrap();

    println!("Exported: output/dispensing_frame.stl");
    println!();
    println!("── Dispensing Frame Specs ──");
    println!("  Frame:           {frame_width:.1}mm × {frame_depth:.1}mm × {frame_height:.1}mm");
    println!("  Wall:            {wall:.1}mm (back: {back_wall:.1}mm reinforced)");
    println!("  Enclosure bolts: 4× M3 at {enclosure_bolt_spacing_x:.0}mm × {enclosure_bolt_spacing_y:.0}mm");
    println!("  Syringe mounts:  {NUM_FLUID_CHANNELS}× on back wall at Z={syringe_z:.0}mm (4 holes each, {:.0}mm spacing)", syringe_hole_half_spacing_z * 2.0);
    println!("  Motor mounts:    {NUM_FLUID_CHANNELS}× on back wall top edge ({motor_mount_hole_spacing:.0}mm bolt spacing)");
    println!("  Rail mounts:     2× side walls at Z={rail_z:.0}mm");
    println!("  Tube pass:       {NUM_FLUID_CHANNELS}× {tube_pass_d:.0}mm holes in bottom");
    println!("  Material:        PETG");
}
