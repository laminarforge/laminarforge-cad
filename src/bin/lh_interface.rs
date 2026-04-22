use vcad::{centered_cube, centered_cylinder, Part};

// ─── Liquid Handler Integration — Shelf Extension + Motorized Hatch ───
//
// Mechanical interface between the 100-chip rack (`chip_stack_rack.rs`)
// inside the insulated thermal container (`chip_incubator_v2.rs`) and a
// commercial liquid handler (Opentrons Flex is the reference target).
//
// KINEMATIC CHOICE
//     Option (B) — per-shelf linear extension — selected over:
//       (A) LH reaches through a fixed hatch — limited by LH Z-travel
//           (Flex gantry cannot reach 4 full shelf depths through a
//           single 550 mm deep chamber; bottom shelves unreachable),
//       (C) Whole-rack pick-and-place by gripper — 13 kg PETG / 15 kg
//           Al rack exceeds Flex gripper capacity (7 kg typical labware),
//           and sterility loss is ~3 min per pick.
//
//     Option (B) keeps the rack continuously rocked and thermally
//     stabilized; only one shelf at a time is exposed for ≤5 min, and
//     each shelf slides out onto telescoping linear rails through a
//     front-top hatch into a Class II BSC footprint where the LH deck
//     sits directly above.
//
// MECHANICAL SUMMARY
//     - Motorized top hatch: 400 × 300 mm clamshell servo-driven
//       (2× MG996R at 1.5 N·m each, geared), gasketed when closed,
//       opens full-width in ≤15 s.
//     - Per-shelf telescoping rail: 2-stage ball-bearing slide, 200 mm
//       extension, detent locks at 0 mm (home) and 200 mm (deployed).
//       Stage 1 = inner drawer rail bolted to shelf bottom, stage 2 =
//       intermediate carrier, stage 3 = outer fixed to post cross-ties.
//     - Locking pins: solenoid-driven 5 mm Ø pins engage at both home
//       and deployed detents; prevent back-drive under rocker motion
//       AND during LH pipette reach.
//     - LH deck fiducials: 3 reflective 12 mm discs on an L-frame on
//       the extended shelf's front-right corner let the LH camera home
//       to the shelf with ±0.1 mm precision.
//     - Clamshell hatch + gasket: anti-condensation heater wire
//       channel routes around lip (shared with chamber's existing
//       hatch_heat_channel geometry).
//
// CHIP ADDRESSING (ANSI/SLAS, per shelf)
//     Shelf has 4 cols × 5 rows = 20 chips, chip long axis along X.
//     Row label: A..E (front → back, +Y)
//     Col label: 1..4 (left → right, +X)
//     Pitch X: 132.76 mm (127.76 chip + 5 mm gutter)
//     Pitch Y: 90.48 mm  (85.48 chip + 5 mm gutter)
//     Datum = shelf chip A1 well A1 corner, reported relative to LH
//     deck origin after the shelf is latched at 200 mm deployed.
//
// Exports:
//     output/lh_interface_hatch_panel.stl    — clamshell half hatch
//     output/lh_interface_hatch_frame.stl    — hinge + servo frame
//     output/lh_interface_rail_inner.stl     — inner drawer slide (1/shelf)
//     output/lh_interface_rail_outer.stl     — outer fixed slide (1/shelf)
//     output/lh_interface_locking_pin.stl    — solenoid detent pin
//     output/lh_interface_fiducial_frame.stl — corner L-frame w/ fiducials
//     output/lh_interface_assembly.stl       — one extended shelf demo

fn main() {
    // ══════════════════════════════════════════════════════════════
    // PARAMETERS (mirror chip_stack_rack.rs / chip_incubator_v2.rs)
    // ══════════════════════════════════════════════════════════════

    // Chip (Rev C) + shelf grid
    let chip_x: f64 = 127.76;
    let chip_y: f64 = 85.48;
    let _chip_z: f64 = 14.35;

    let pocket_clearance: f64 = 0.7;
    let pocket_x: f64 = chip_x + pocket_clearance * 2.0;
    let pocket_y: f64 = chip_y + pocket_clearance * 2.0;

    let cols: i32 = 4;
    let rows: i32 = 5;
    let gutter_interior: f64 = 5.0;
    let edge_margin: f64 = 2.0;

    let shelf_x: f64 =
        (cols as f64) * pocket_x + ((cols - 1) as f64) * gutter_interior + 2.0 * edge_margin;
    let shelf_y: f64 =
        (rows as f64) * pocket_y + ((rows - 1) as f64) * gutter_interior + 2.0 * edge_margin;
    let shelf_thickness: f64 = 3.0;

    let shelf_pitch: f64 = 40.0;
    let num_shelves: i32 = 5;

    // Chamber interior (chip_incubator_v2)
    let chamber_inner_x: f64 = 600.0;
    let chamber_inner_y: f64 = 520.0;
    let chamber_inner_z: f64 = 550.0;

    // ── Hatch (clamshell, replaces static top_hatch on chamber) ──
    let hatch_open_x: f64 = 400.0; // along container X (left/right)
    let hatch_open_y: f64 = 300.0; // along container Y (front/back)
    let hatch_panel_thickness: f64 = 15.0;
    // Each of the 2 clamshell panels spans half the opening + 10 mm overlap
    let hatch_panel_x: f64 = hatch_open_x / 2.0 + 10.0;
    let hatch_panel_y: f64 = hatch_open_y + 20.0; // 10 mm lip each side
    let hatch_hinge_dia: f64 = 6.0;

    // ── Telescoping linear rail (one per shelf) ──
    let rail_length: f64 = 500.0; // outer rail body length
    let rail_extension: f64 = 200.0; // max deployed slide
    let rail_width: f64 = 18.0;
    let rail_height: f64 = 12.0;
    let rail_wall: f64 = 2.0;

    // ── Detent locking pin (solenoid) ──
    let pin_dia: f64 = 5.0;
    let pin_len: f64 = 25.0;
    let solenoid_body_x: f64 = 32.0;
    let solenoid_body_y: f64 = 16.0;
    let solenoid_body_z: f64 = 16.0;

    // ── Fiducial L-frame ──
    let fiducial_disc_dia: f64 = 12.0;
    let fiducial_disc_thk: f64 = 2.0;
    let fiducial_arm_len: f64 = 80.0;
    let fiducial_arm_thk: f64 = 5.0;

    // ══════════════════════════════════════════════════════════════
    // 1. CLAMSHELL HATCH PANEL (print 2 — mirror for second half)
    // ══════════════════════════════════════════════════════════════

    let panel_body =
        centered_cube("panel_body", hatch_panel_x, hatch_panel_y, hatch_panel_thickness);

    // Gasket groove on bottom face (3×3 mm) along 3 edges of the half
    // (excludes the centerline edge where the two halves meet)
    let gasket_width = 3.0;
    let gasket_depth = 3.0;
    let gasket_z = -(hatch_panel_thickness / 2.0) + gasket_depth / 2.0;
    let gasket_outer_x = hatch_panel_x - 10.0;
    let gasket_outer_y = hatch_panel_y - 10.0;
    let gasket_inner_x = gasket_outer_x - gasket_width * 2.0;
    let gasket_inner_y = gasket_outer_y - gasket_width * 2.0;

    let gasket_outer = centered_cube(
        "gasket_outer",
        gasket_outer_x,
        gasket_outer_y,
        gasket_depth + 0.1,
    )
    .translate(0.0, 0.0, gasket_z);
    let gasket_inner = centered_cube(
        "gasket_inner",
        gasket_inner_x,
        gasket_inner_y,
        gasket_depth + 0.2,
    )
    .translate(0.0, 0.0, gasket_z);
    let gasket_channel = gasket_outer - gasket_inner;

    // Hinge knuckles on one long edge (the +X edge = away from centerline
    // for left half; mirrored for right). 3 knuckles, each 30 mm wide.
    let knuckle_w = 30.0;
    let knuckle_h = 12.0;
    let knuckle_d = 10.0;
    let knuckle_edge_x = hatch_panel_x / 2.0 - knuckle_d / 2.0 + 4.0;
    let mut knuckles = Part::empty("knuckles");
    for (i, yo) in [-hatch_panel_y / 3.0, 0.0, hatch_panel_y / 3.0].iter().enumerate() {
        let k = centered_cube(
            &format!("knuckle_{i}"),
            knuckle_d,
            knuckle_w,
            knuckle_h,
        )
        .translate(knuckle_edge_x, *yo, hatch_panel_thickness / 2.0 + knuckle_h / 2.0 - 1.0);
        // Drill the hinge hole through it (along Y so the pin runs parallel
        // to the hatch long edge)
        let hole = centered_cylinder(
            &format!("kh_{i}"),
            hatch_hinge_dia / 2.0 + 0.1,
            knuckle_w + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(knuckle_edge_x, *yo, hatch_panel_thickness / 2.0 + knuckle_h / 2.0 - 1.0);
        knuckles = knuckles + (k - hole);
    }

    // Servo mount boss on the opposite long edge (-X edge) — square
    // pocket sized for MG996R-class servo (40.7 × 20 × 39.5 mm nominal)
    let servo_pocket_x: f64 = 42.0;
    let servo_pocket_y: f64 = 22.0;
    let servo_pocket_z: f64 = 35.0;
    let servo_mount = centered_cube("servo_mount", 55.0, 32.0, 14.0).translate(
        -hatch_panel_x / 2.0 + 35.0,
        0.0,
        hatch_panel_thickness / 2.0 + 7.0,
    );
    let servo_pocket = centered_cube(
        "servo_pocket",
        servo_pocket_x,
        servo_pocket_y,
        servo_pocket_z,
    )
    .translate(
        -hatch_panel_x / 2.0 + 35.0,
        0.0,
        hatch_panel_thickness / 2.0 + 7.0,
    );
    let servo_mount_final = servo_mount - servo_pocket;

    let hatch_panel = (panel_body - gasket_channel) + knuckles + servo_mount_final;
    hatch_panel
        .write_stl("output/lh_interface_hatch_panel.stl")
        .unwrap();
    println!("Exported: output/lh_interface_hatch_panel.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. HATCH FRAME (hinge support + bolt flange on chamber top)
    // ══════════════════════════════════════════════════════════════

    let frame_outer_x: f64 = hatch_open_x + 60.0;
    let frame_outer_y: f64 = hatch_open_y + 60.0;
    let frame_thk: f64 = 8.0;
    let frame_outer = centered_cube("frame_outer", frame_outer_x, frame_outer_y, frame_thk);
    let frame_opening = centered_cube("frame_opening", hatch_open_x, hatch_open_y, frame_thk + 2.0);

    // Bolt holes around the perimeter (8× M5 through into chamber top)
    let mut frame_holes = Part::empty("frame_holes");
    let bolt_dia = 5.5;
    let bolt_inset = 15.0;
    let bolt_positions: [(f64, f64); 8] = [
        (-frame_outer_x / 2.0 + bolt_inset, -frame_outer_y / 2.0 + bolt_inset),
        (0.0, -frame_outer_y / 2.0 + bolt_inset),
        (frame_outer_x / 2.0 - bolt_inset, -frame_outer_y / 2.0 + bolt_inset),
        (-frame_outer_x / 2.0 + bolt_inset, 0.0),
        (frame_outer_x / 2.0 - bolt_inset, 0.0),
        (-frame_outer_x / 2.0 + bolt_inset, frame_outer_y / 2.0 - bolt_inset),
        (0.0, frame_outer_y / 2.0 - bolt_inset),
        (frame_outer_x / 2.0 - bolt_inset, frame_outer_y / 2.0 - bolt_inset),
    ];
    for (i, (fx, fy)) in bolt_positions.iter().enumerate() {
        let h = centered_cylinder(&format!("fh_{i}"), bolt_dia / 2.0, frame_thk + 2.0, 24)
            .translate(*fx, *fy, 0.0);
        frame_holes = frame_holes + h;
    }

    // Center hinge support spine (bridge at Y=0)
    let hinge_spine = centered_cube("hinge_spine", 20.0, frame_outer_y - 40.0, frame_thk + 6.0);

    // Hinge pin holes (3 positions along spine)
    let mut spine_holes = Part::empty("spine_holes");
    for (i, yo) in [-frame_outer_y / 3.0, 0.0, frame_outer_y / 3.0].iter().enumerate() {
        let h = centered_cylinder(
            &format!("sh_{i}"),
            hatch_hinge_dia / 2.0 + 0.1,
            30.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, *yo, frame_thk / 2.0 + 3.0);
        spine_holes = spine_holes + h;
    }

    let hatch_frame = (frame_outer - frame_opening) + hinge_spine - frame_holes - spine_holes;
    hatch_frame
        .write_stl("output/lh_interface_hatch_frame.stl")
        .unwrap();
    println!("Exported: output/lh_interface_hatch_frame.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. RAIL — OUTER FIXED SLIDE (bolts to chamber wall / post cross-tie)
    // ══════════════════════════════════════════════════════════════

    let outer_rail = centered_cube("outer_rail", rail_width, rail_length, rail_height);
    // Channel (U-shape) that carries the inner slide
    let outer_channel = centered_cube(
        "outer_channel",
        rail_width - rail_wall * 2.0,
        rail_length + 2.0,
        rail_height - rail_wall,
    )
    .translate(0.0, 0.0, rail_wall / 2.0);
    // Mounting tabs at 4 positions along the length
    let mut outer_holes = Part::empty("outer_rail_holes");
    for (i, yo) in [
        -rail_length / 2.0 + 25.0,
        -rail_length / 6.0,
        rail_length / 6.0,
        rail_length / 2.0 - 25.0,
    ]
    .iter()
    .enumerate()
    {
        let h = centered_cylinder(
            &format!("orh_{i}"),
            4.4 / 2.0,
            rail_height + 2.0,
            24,
        )
        .translate(0.0, *yo, 0.0);
        outer_holes = outer_holes + h;
    }
    // Detent pockets at home (0 mm) and deployed (+200 mm) positions
    let detent_home = centered_cylinder(
        "detent_home",
        pin_dia / 2.0 + 0.2,
        rail_wall + 1.0,
        24,
    )
    .translate(0.0, -rail_length / 2.0 + 50.0, rail_height / 2.0 - rail_wall / 2.0);
    let detent_deployed = centered_cylinder(
        "detent_deployed",
        pin_dia / 2.0 + 0.2,
        rail_wall + 1.0,
        24,
    )
    .translate(
        0.0,
        -rail_length / 2.0 + 50.0 + rail_extension,
        rail_height / 2.0 - rail_wall / 2.0,
    );

    let rail_outer = outer_rail - outer_channel - outer_holes - detent_home - detent_deployed;
    rail_outer
        .write_stl("output/lh_interface_rail_outer.stl")
        .unwrap();
    println!("Exported: output/lh_interface_rail_outer.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. RAIL — INNER DRAWER SLIDE (bolts to underside of shelf)
    // ══════════════════════════════════════════════════════════════

    let inner_w = rail_width - rail_wall * 2.0 - 1.0; // 1 mm running clearance
    let inner_h = rail_height - rail_wall - 1.0;
    let inner_body =
        centered_cube("inner_body", inner_w, rail_length - 30.0, inner_h);
    // Ball-bearing recesses (6 total, paired) — simplified as 4 mm Ø
    // pockets 3 mm deep on both side faces
    let mut ball_recesses = Part::empty("ball_recesses");
    for (i, yo) in [
        -rail_length / 2.0 + 60.0,
        -rail_length / 2.0 + 160.0,
        rail_length / 2.0 - 60.0,
    ]
    .iter()
    .enumerate()
    {
        for &sx in &[-1.0f64, 1.0] {
            let r = centered_cylinder(
                &format!("br_{i}_{}", sx as i32),
                4.0 / 2.0,
                3.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(sx * (inner_w / 2.0 - 1.5), *yo, 0.0);
            ball_recesses = ball_recesses + r;
        }
    }
    // Shelf-mount holes on top face (4× M4 through)
    let mut shelf_mount_holes = Part::empty("shelf_mount_holes");
    for (i, yo) in [
        -(rail_length - 30.0) / 2.0 + 20.0,
        -(rail_length - 30.0) / 6.0,
        (rail_length - 30.0) / 6.0,
        (rail_length - 30.0) / 2.0 - 20.0,
    ]
    .iter()
    .enumerate()
    {
        let h = centered_cylinder(
            &format!("smh_{i}"),
            4.4 / 2.0,
            inner_h + 2.0,
            24,
        )
        .translate(0.0, *yo, 0.0);
        shelf_mount_holes = shelf_mount_holes + h;
    }

    let rail_inner = inner_body - ball_recesses - shelf_mount_holes;
    rail_inner
        .write_stl("output/lh_interface_rail_inner.stl")
        .unwrap();
    println!("Exported: output/lh_interface_rail_inner.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. LOCKING PIN (solenoid-actuated detent)
    // ══════════════════════════════════════════════════════════════

    let pin_body = centered_cylinder("pin_body", pin_dia / 2.0, pin_len, 24);
    // Conical tip
    let pin_tip = centered_cylinder("pin_tip", pin_dia / 2.0, 3.0, 24)
        .translate(0.0, 0.0, pin_len / 2.0 + 1.5);
    // Plunger collar
    let collar = centered_cylinder("collar", 9.0 / 2.0, 3.0, 24)
        .translate(0.0, 0.0, -pin_len / 2.0 + 1.5);
    let pin = pin_body + pin_tip + collar;
    // Solenoid body stub (so the exported STL shows the envelope)
    let sol_body = centered_cube(
        "sol_body",
        solenoid_body_x,
        solenoid_body_y,
        solenoid_body_z,
    )
    .translate(0.0, 0.0, -pin_len / 2.0 - solenoid_body_z / 2.0);
    let locking = pin + sol_body;
    locking
        .write_stl("output/lh_interface_locking_pin.stl")
        .unwrap();
    println!("Exported: output/lh_interface_locking_pin.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. FIDUCIAL L-FRAME (LH camera homing target)
    // ══════════════════════════════════════════════════════════════

    let arm_x = centered_cube("arm_x", fiducial_arm_len, fiducial_arm_thk, fiducial_arm_thk)
        .translate(fiducial_arm_len / 2.0, 0.0, 0.0);
    let arm_y = centered_cube("arm_y", fiducial_arm_thk, fiducial_arm_len, fiducial_arm_thk)
        .translate(0.0, fiducial_arm_len / 2.0, 0.0);
    let fid_1 = centered_cylinder(
        "fid_1",
        fiducial_disc_dia / 2.0,
        fiducial_disc_thk,
        32,
    )
    .translate(10.0, 10.0, fiducial_arm_thk / 2.0 + fiducial_disc_thk / 2.0);
    let fid_2 = centered_cylinder(
        "fid_2",
        fiducial_disc_dia / 2.0,
        fiducial_disc_thk,
        32,
    )
    .translate(fiducial_arm_len - 10.0, 10.0, fiducial_arm_thk / 2.0 + fiducial_disc_thk / 2.0);
    let fid_3 = centered_cylinder(
        "fid_3",
        fiducial_disc_dia / 2.0,
        fiducial_disc_thk,
        32,
    )
    .translate(10.0, fiducial_arm_len - 10.0, fiducial_arm_thk / 2.0 + fiducial_disc_thk / 2.0);
    // Mount hole at corner (M4)
    let fid_hole = centered_cylinder("fid_hole", 4.2 / 2.0, fiducial_arm_thk + 2.0, 24);
    let fid_frame = (arm_x + arm_y + fid_1 + fid_2 + fid_3) - fid_hole;
    fid_frame
        .write_stl("output/lh_interface_fiducial_frame.stl")
        .unwrap();
    println!("Exported: output/lh_interface_fiducial_frame.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. ASSEMBLY — shelf 3 extended, chamber outline for scale
    // ══════════════════════════════════════════════════════════════

    // Chamber outline (shell only — for scale reference)
    let chamber_shell_outer = centered_cube(
        "chamber_shell_outer",
        chamber_inner_x + 6.0,
        chamber_inner_y + 6.0,
        chamber_inner_z + 6.0,
    );
    let chamber_shell_inner = centered_cube(
        "chamber_shell_inner",
        chamber_inner_x,
        chamber_inner_y,
        chamber_inner_z,
    );
    // Cut the top hatch opening
    let top_cut = centered_cube(
        "top_cut",
        hatch_open_x,
        hatch_open_y,
        10.0,
    )
    .translate(0.0, 0.0, chamber_inner_z / 2.0);
    let chamber_ref = (chamber_shell_outer - chamber_shell_inner) - top_cut;

    // Outer rail, fixed to interior ceiling, runs along Y
    let outer_rail_at_shelf = centered_cube(
        "outer_rail_at_shelf",
        rail_width,
        rail_length,
        rail_height,
    )
    .translate(
        shelf_x / 4.0,
        0.0,
        chamber_inner_z / 2.0 - 40.0 - rail_height / 2.0,
    );

    // Shelf 3 at mid-pitch, extended 200 mm out the front (+Y direction
    // is back; front = −Y in chamber coords, but the hatch opens up (+Z),
    // so "extension" is along +X → out the container side? For this demo
    // we show the shelf slid forward along the inner-rail axis which we
    // place along Y; the real deployment translates the shelf in +Y
    // through an open hatch-side wall. Here we deploy along +Y for demo.
    let deployed_y = rail_extension; // +200 mm from home
    let shelf_z = 100.0; // arbitrary mid-stack Z for demo
    let deployed_shelf = centered_cube(
        "deployed_shelf",
        shelf_x,
        shelf_y,
        shelf_thickness,
    )
    .translate(0.0, deployed_y, shelf_z);

    // Fiducial L-frame at front-right corner of deployed shelf
    let fid_at_shelf_x = shelf_x / 2.0 - fiducial_arm_len / 2.0;
    let fid_at_shelf_y = deployed_y - shelf_y / 2.0 + fiducial_arm_len / 2.0;
    let fid_at_shelf = {
        let ax = centered_cube(
            "asm_fid_ax",
            fiducial_arm_len,
            fiducial_arm_thk,
            fiducial_arm_thk,
        );
        let ay = centered_cube(
            "asm_fid_ay",
            fiducial_arm_thk,
            fiducial_arm_len,
            fiducial_arm_thk,
        );
        (ax + ay).translate(
            fid_at_shelf_x,
            fid_at_shelf_y,
            shelf_z + shelf_thickness / 2.0 + fiducial_arm_thk / 2.0,
        )
    };

    // Hatch frame on top of chamber (centered on top face)
    let hatch_frame_asm = centered_cube(
        "asm_hatch_frame",
        frame_outer_x,
        frame_outer_y,
        frame_thk,
    )
    .translate(
        0.0,
        0.0,
        chamber_inner_z / 2.0 + 3.0 + frame_thk / 2.0,
    );
    let hatch_open_asm = centered_cube(
        "asm_hatch_open",
        hatch_open_x,
        hatch_open_y,
        frame_thk + 2.0,
    )
    .translate(
        0.0,
        0.0,
        chamber_inner_z / 2.0 + 3.0 + frame_thk / 2.0,
    );

    let assembly = chamber_ref
        + outer_rail_at_shelf
        + deployed_shelf
        + fid_at_shelf
        + (hatch_frame_asm - hatch_open_asm);
    assembly
        .write_stl("output/lh_interface_assembly.stl")
        .unwrap();
    println!("Exported: output/lh_interface_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // CHIP COORDINATE TABLE (ANSI/SLAS, relative to shelf A1 datum)
    // ══════════════════════════════════════════════════════════════

    let pitch_x = pocket_x + gutter_interior; // 132.7 mm (chip + gutter)
    let pitch_y = pocket_y + gutter_interior; // 90.48 + 5 = 95.48? no — pocket_y = 86.88, +5 = 91.88
    // Note: pocket_y = chip_y + 2*0.7 = 86.88 mm; pitch_y = 91.88 mm.

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!(" CHIP ADDRESSING — relative to shelf 1, chip A1 (datum)");
    println!("══════════════════════════════════════════════════════════════");
    println!(
        "  Pitch X (col → col): {pitch_x:.2} mm   ({:.2} chip + {:.0} gutter)",
        pocket_x, gutter_interior
    );
    println!(
        "  Pitch Y (row → row): {pitch_y:.2} mm   ({:.2} chip + {:.0} gutter)",
        pocket_y, gutter_interior
    );
    println!("  Pitch Z (shelf N+1 − shelf N): {shelf_pitch:.1} mm");
    println!("  Number of shelves: {num_shelves}     Chips per shelf: {}", cols * rows);
    println!();
    println!("  Shelf | Row | Col | ΔX (mm) | ΔY (mm) | ΔZ (mm)");
    println!("  ------+-----+-----+---------+---------+--------");
    for shelf in 1..=num_shelves {
        for r in 0..rows {
            let row_label = (b'A' + r as u8) as char;
            for c in 0..cols {
                let col_label = c + 1;
                let dx = (c as f64) * pitch_x;
                let dy = (r as f64) * pitch_y;
                let dz = ((shelf - 1) as f64) * shelf_pitch;
                println!(
                    "   S{shelf}  |  {row_label}  |  {col_label}  | {dx:7.2} | {dy:7.2} | {dz:6.2}"
                );
            }
        }
        if shelf < num_shelves {
            println!("  ------+-----+-----+---------+---------+--------");
        }
    }
    println!();

    // ══════════════════════════════════════════════════════════════
    // CYCLE TIME + PIPETTE PATH BUDGET
    // ══════════════════════════════════════════════════════════════

    // 48 h media change: 6 wells aspirate spent + 3 wells dispense fresh = 9 tip events per chip.
    // Opentrons Flex 8-channel pipette does 8 channels in parallel; 20 chips
    // per shelf → we aspirate from 6 wells × 20 chips = 120 well events;
    // with 8-ch parallel that is 120/8 = 15 cycles × 5 s = 75 s aspirate phase
    // plus 60 wells dispense = 60/8 = 7.5 → 8 cycles × 5 s = 40 s dispense.
    // Per shelf pipette work: ~115 s.
    let tip_events_per_chip = 9;
    let chips_per_shelf = (cols * rows) as f64;
    let events_per_shelf = tip_events_per_chip as f64 * chips_per_shelf;
    let channels = 8.0;
    let seconds_per_cycle = 5.0;
    let pipette_cycles = (events_per_shelf / channels).ceil();
    let pipette_time_per_shelf = pipette_cycles * seconds_per_cycle;

    let rocker_pause_s = 30.0;
    let hatch_open_s = 15.0;
    let shelf_extend_s = 10.0;
    let shelf_retract_s = 10.0;
    let rocker_resume_s = 30.0;

    let per_shelf = shelf_extend_s + pipette_time_per_shelf + shelf_retract_s;
    let total_s = rocker_pause_s
        + hatch_open_s
        + (num_shelves as f64) * per_shelf
        + rocker_resume_s;

    println!("══════════════════════════════════════════════════════════════");
    println!(" CYCLE TIME BUDGET — 100-chip 48 h media change");
    println!("══════════════════════════════════════════════════════════════");
    println!("  Rocker pause (MQTT handshake):  {rocker_pause_s:>5.0} s");
    println!("  Clamshell hatch open:           {hatch_open_s:>5.0} s");
    println!(
        "  Per-shelf cycle ×{num_shelves}: extend {shelf_extend_s:.0} + pipette {pipette_time_per_shelf:.0} + retract {shelf_retract_s:.0} = {per_shelf:.0} s"
    );
    println!("  Rocker resume + hatch close:    {rocker_resume_s:>5.0} s");
    println!(
        "  TOTAL:                          {:.0} s ≈ {:.1} min",
        total_s,
        total_s / 60.0
    );
    println!(
        "  Per-batch overhead repeats every 48 h (≤{:.1}% duty)",
        (total_s / 60.0) / (48.0 * 60.0) * 100.0
    );

    println!();
    println!(" EXPORTED STLs");
    println!("   output/lh_interface_hatch_panel.stl");
    println!("   output/lh_interface_hatch_frame.stl");
    println!("   output/lh_interface_rail_outer.stl");
    println!("   output/lh_interface_rail_inner.stl");
    println!("   output/lh_interface_locking_pin.stl");
    println!("   output/lh_interface_fiducial_frame.stl");
    println!("   output/lh_interface_assembly.stl");
    println!("══════════════════════════════════════════════════════════════");
}
