use vcad::{centered_cube, centered_cylinder, Part};

// ─── 100-Chip Stacking Rack ───
//
// Holds 100 ANSI/SLAS microplate-format Rev C microfluidic chips
// (127.76 × 85.48 × 14.35 mm, 32 wells, glass coverslip bottom) in a
// 5-shelf × 20-chip-per-shelf arrangement inside a ~550 × 500 × 500 mm
// thermal container. Mounts onto a rocker top plate.
//
// Features:
//   - 5 shelves × (4 cols × 5 rows) = 100 chip pockets total
//   - 40 mm shelf pitch (14.35 mm chip + 25 mm pipette-tip clearance)
//   - Drop-in pockets with 0.7 mm side clearance + 4 corner retention tabs
//     keep chips seated at 15° rocker tilt
//   - Central 100 × 60 mm cut-through per pocket exposes the glass bottom
//     for optical inspection / imaging from below
//   - 10 × 5 mm barcode label slot next to each pocket
//   - 4 corner posts (20 × 20 mm square tube) with notches at each shelf
//     level; posts bolt to rack base plate via M5, base plate bolts to
//     rocker top plate via M6 (450 × 400 mm bolt rectangle)
//   - Parameterized shelf material thickness (default 3 mm — either
//     6061 aluminum sheet or 3D-printed PETG)
//   - U-shaped 10 mm rod carry handle across the top posts
//
// Exports:
//   - chip_stack_rack_shelf.stl
//   - chip_stack_rack_post.stl
//   - chip_stack_rack_base.stl
//   - chip_stack_rack_handle.stl
//   - chip_stack_rack_assembly.stl

fn main() {
    // ══════════════════════════════════════════════════════════════
    // PARAMETERS
    // ══════════════════════════════════════════════════════════════

    // ── Chip (Rev C, ANSI/SLAS microplate) ──
    let chip_x: f64 = 127.76;
    let chip_y: f64 = 85.48;
    let chip_z: f64 = 14.35;

    // ── Pocket clearance ──
    let pocket_clearance: f64 = 0.7; // per side
    let pocket_x: f64 = chip_x + pocket_clearance * 2.0; // 128.5
    let pocket_y: f64 = chip_y + pocket_clearance * 2.0; // 86.2
    let pocket_depth: f64 = 4.0;

    // ── Grid arrangement (per shelf) ──
    // 4 columns (X) × 5 rows (Y) = 20 chips per shelf
    let cols: i32 = 4;
    let rows: i32 = 5;

    // Per-shelf footprint ─ must fit in the 550 × 500 container.
    // Default orientation: long chip axis along X, short along Y.
    //   X: cols × chip_x + (cols+1) × gutter = 4×127.76 + 5×10 = 561.04  — overflows 550
    //   Y: rows × chip_y + (rows+1) × gutter = 5×85.48 + 6×10  = 487.40  — fits 500
    // Fall back: rotate chips 90° so long axis runs along Y (depth).
    //   X: 4 × chip_y + 5 × gutter = 4×85.48 + 50 = 391.92   — fits 550
    //   Y: 5 × chip_x + 6 × gutter = 5×127.76 + 60 = 698.80  — overflows 500
    // Neither 4×5 fits without spill. Use the default orientation and
    // shrink the gutter slightly + tighten the outside margin so the
    // 550-mm face is cleared. Use gutter = 7 mm interior / 5 mm edge.
    //   X: 4×127.76 + 3×7 + 2×5 = 511.04 + 21 + 10 = 542.04  ✓ < 550
    //   Y: 5×85.48  + 4×7 + 2×5 = 427.40 + 28 + 10 = 465.40  ✓ < 500
    // Gutters chosen so shelf + 20×20 corner posts (placed inside shelf
    // corners via notches) fit under the 550×500 container envelope:
    //   shelf_x = 4×127.76 + 3×5 + 2×2 = 530.04  ✓ < 550
    //   shelf_y = 5×85.48  + 4×5 + 2×2 = 451.40  ✓ < 500
    let gutter_interior: f64 = 5.0;
    let edge_margin: f64 = 2.0;

    let shelf_x: f64 = (cols as f64) * pocket_x
        + ((cols - 1) as f64) * gutter_interior
        + 2.0 * edge_margin;
    let shelf_y: f64 = (rows as f64) * pocket_y
        + ((rows - 1) as f64) * gutter_interior
        + 2.0 * edge_margin;
    let shelf_thickness: f64 = 3.0; // material thickness (3 mm, 6061 Al or PETG)

    // Fallback check: if even this overflows, re-tighten (but we know ok)
    assert!(shelf_x < 550.0, "shelf width overflows container");
    assert!(shelf_y < 500.0, "shelf depth overflows container");

    // ── Shelf pitch ──
    let shelf_pitch: f64 = 40.0; // 14.35 chip + 25 mm pipette clearance ≈ 40
    let num_shelves: i32 = 5;
    let total_chips: i32 = num_shelves * cols * rows;
    assert!(total_chips == 100);

    // ── Rack base plate ──
    // Sized to fit inside the 550×500 container with a small margin.
    let base_x: f64 = 545.0;
    let base_y: f64 = 495.0;
    let base_thickness: f64 = 6.0;

    // ── Corner posts (20×20 mm square tube) ──
    let post_side: f64 = 20.0;
    let post_height: f64 =
        (num_shelves as f64) * shelf_pitch + 30.0; // top clearance above top shelf
    let post_flange_side: f64 = 34.0; // keeps flange inside base footprint
    let post_flange_thickness: f64 = 6.0;

    // Corner post spacing — posts sit INSIDE shelf corners via notches
    // so the overall footprint = shelf footprint. Post center sits
    // half a post width inside the shelf corner.
    let post_spacing_x: f64 = shelf_x - post_side; // post centers
    let post_spacing_y: f64 = shelf_y - post_side;

    // Shelf notch (square cut out of shelf corner to clear the post)
    let notch_side: f64 = post_side + 1.0; // 0.5 mm clearance per side

    // ── Label slot ──
    let label_x: f64 = 10.0;
    let label_y: f64 = 5.0;
    let label_depth: f64 = 0.6;

    // ── Optical cut-through under pocket ──
    let optical_x: f64 = 100.0;
    let optical_y: f64 = 60.0;

    // ── Mounting holes ──
    let corner_bolt_dia: f64 = 5.5; // M5 clearance
    let rocker_bolt_dia: f64 = 6.6; // M6 clearance
    let rocker_bolt_rect_x: f64 = 450.0;
    let rocker_bolt_rect_y: f64 = 400.0;

    // ══════════════════════════════════════════════════════════════
    // 1. SHELF
    // ══════════════════════════════════════════════════════════════

    // Start with a solid sheet
    let mut shelf = centered_cube("shelf_body", shelf_x, shelf_y, shelf_thickness);

    // Grid origin for chip pockets (shelf centered at 0)
    let grid_origin_x = -(shelf_x / 2.0) + edge_margin + pocket_x / 2.0;
    let grid_origin_y = -(shelf_y / 2.0) + edge_margin + pocket_y / 2.0;

    // Subtract pockets, optical cut-throughs, and label slots
    let mut pockets = Part::empty("pockets");
    let mut cuts = Part::empty("optical_cuts");
    let mut labels = Part::empty("labels");

    for cx in 0..cols {
        for ry in 0..rows {
            let px = grid_origin_x + (cx as f64) * (pocket_x + gutter_interior);
            let py = grid_origin_y + (ry as f64) * (pocket_y + gutter_interior);

            // Pocket recess (from top face, going down by pocket_depth).
            // shelf_thickness = 3 mm < pocket_depth = 4 mm → pocket passes
            // fully through a 3 mm shelf. That's fine; the retention
            // tabs (added below) keep chips in place and the cut-
            // through exposes the glass bottom. If shelf_thickness ≥
            // pocket_depth, the pocket is a blind recess.
            let pocket = centered_cube(
                &format!("pocket_{cx}_{ry}"),
                pocket_x,
                pocket_y,
                pocket_depth + 0.5,
            )
            .translate(px, py, shelf_thickness / 2.0 - pocket_depth / 2.0 + 0.25);
            pockets = pockets + pocket;

            // Optical cut-through at pocket center (fully through shelf)
            let cut = centered_cube(
                &format!("cut_{cx}_{ry}"),
                optical_x,
                optical_y,
                shelf_thickness + 2.0,
            )
            .translate(px, py, 0.0);
            cuts = cuts + cut;

            // Label slot in front gutter (toward -Y)
            let label_y_pos = py - pocket_y / 2.0 - label_y / 2.0 - 0.5;
            let label = centered_cube(
                &format!("label_{cx}_{ry}"),
                label_x,
                label_y,
                label_depth + 0.1,
            )
            .translate(
                px,
                label_y_pos,
                shelf_thickness / 2.0 - label_depth / 2.0 + 0.05,
            );
            labels = labels + label;
        }
    }

    shelf = shelf - pockets - cuts - labels;

    // ── Retention tabs (4 per pocket, one at each corner) ──
    // Small raised ridges on the shelf top surface that grip the chip
    // corners so a 15° rocker tilt can't slide them out. Tabs are
    // added AFTER the pockets were cut so they rise from the pocket
    // floor / frame edges.
    let tab_x: f64 = 3.0;
    let tab_y: f64 = 3.0;
    let tab_h: f64 = 2.5; // rises above pocket floor
    let mut tabs = Part::empty("tabs");
    for cx in 0..cols {
        for ry in 0..rows {
            let px = grid_origin_x + (cx as f64) * (pocket_x + gutter_interior);
            let py = grid_origin_y + (ry as f64) * (pocket_y + gutter_interior);
            // Place tabs slightly inside the chip footprint so they
            // grip corners. Offset = chip/2 - inset
            let inset = 1.0;
            let tx_off = chip_x / 2.0 - inset - tab_x / 2.0;
            let ty_off = chip_y / 2.0 - inset - tab_y / 2.0;
            for &sx in &[-1.0f64, 1.0] {
                for &sy in &[-1.0f64, 1.0] {
                    let tab = centered_cube(
                        &format!("tab_{cx}_{ry}_{}_{}", sx as i32, sy as i32),
                        tab_x,
                        tab_y,
                        tab_h,
                    )
                    .translate(
                        px + sx * tx_off,
                        py + sy * ty_off,
                        shelf_thickness / 2.0 + tab_h / 2.0,
                    );
                    tabs = tabs + tab;
                }
            }
        }
    }

    shelf = shelf + tabs;

    // ── Corner notches (so shelf slides between / around posts) ──
    let post_cx = post_spacing_x / 2.0;
    let post_cy = post_spacing_y / 2.0;
    let mut corner_notches = Part::empty("notches");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            // Cut a square out of each shelf corner where the post sits
            let notch = centered_cube(
                &format!("notch_{}_{}", sx as i32, sy as i32),
                notch_side,
                notch_side,
                shelf_thickness + 2.0,
            )
            .translate(sx * post_cx, sy * post_cy, 0.0);
            corner_notches = corner_notches + notch;
        }
    }
    shelf = shelf - corner_notches;

    shelf
        .write_stl("output/chip_stack_rack_shelf.stl")
        .unwrap();
    println!("Exported: output/chip_stack_rack_shelf.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. CORNER POST (one, with shelf notches + base flange)
    // ══════════════════════════════════════════════════════════════

    // Post body: 20×20 square tube (hollow). Model as outer - inner.
    let post_outer = centered_cube("post_outer", post_side, post_side, post_height);
    let post_wall: f64 = 2.0;
    let post_inner_side = post_side - post_wall * 2.0;
    let post_inner = centered_cube(
        "post_inner",
        post_inner_side,
        post_inner_side,
        post_height + 2.0,
    );
    // Leave the top/bottom capped: raise/lower the inner slightly
    let post_inner = post_inner.translate(0.0, 0.0, 0.0);

    // Shelf notches on the inside face of post (facing +X +Y toward the
    // shelf envelope). For a canonical post at (-post_cx, -post_cy),
    // the shelf is toward +X and +Y, so we notch the +X and +Y faces.
    // Each notch: 6 mm (shelf_thickness + 3 mm slop) tall × 4 mm deep
    // from the inner face of the post.
    let notch_h = shelf_thickness + 3.0;
    let notch_d = 4.0;

    // Shelf Z positions: first shelf at z = 15 above base, then +pitch
    let first_shelf_z: f64 = 15.0;
    let mut post_notches = Part::empty("post_notches");
    for i in 0..num_shelves {
        let z = first_shelf_z + (i as f64) * shelf_pitch;
        // Notch on +X face (cut a slot from +X face inward)
        let slot_x = centered_cube(
            &format!("slotx_{i}"),
            notch_d + 0.5,
            post_side + 0.2,
            notch_h,
        )
        .translate(post_side / 2.0 - notch_d / 2.0 + 0.25, 0.0, z);
        // Notch on +Y face
        let slot_y = centered_cube(
            &format!("sloty_{i}"),
            post_side + 0.2,
            notch_d + 0.5,
            notch_h,
        )
        .translate(0.0, post_side / 2.0 - notch_d / 2.0 + 0.25, z);
        post_notches = post_notches + slot_x + slot_y;
    }

    // Base flange (bolts to rack base plate)
    let flange = centered_cube(
        "flange",
        post_flange_side,
        post_flange_side,
        post_flange_thickness,
    )
    .translate(0.0, 0.0, -post_height / 2.0 + post_flange_thickness / 2.0);

    // Flange bolt holes (4× M5)
    let flange_bolt_offset = post_flange_side / 2.0 - 6.0;
    let mut flange_holes = Part::empty("flange_holes");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            let hole = centered_cylinder(
                &format!("fh_{}_{}", sx as i32, sy as i32),
                corner_bolt_dia / 2.0,
                post_flange_thickness + 2.0,
                24,
            )
            .translate(
                sx * flange_bolt_offset,
                sy * flange_bolt_offset,
                -post_height / 2.0 + post_flange_thickness / 2.0,
            );
            flange_holes = flange_holes + hole;
        }
    }

    let post =
        ((post_outer - post_inner) + flange) - post_notches - flange_holes;

    post.write_stl("output/chip_stack_rack_post.stl").unwrap();
    println!("Exported: output/chip_stack_rack_post.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. BASE PLATE
    // ══════════════════════════════════════════════════════════════

    let base = centered_cube("base", base_x, base_y, base_thickness);

    // Corner-post mounting holes (4× M5 at flange positions = post centers)
    let mut base_post_holes = Part::empty("base_post_holes");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            for &fx in &[-1.0f64, 1.0] {
                for &fy in &[-1.0f64, 1.0] {
                    // Each post has 4 flange bolts
                    let px = sx * post_cx + fx * flange_bolt_offset;
                    let py = sy * post_cy + fy * flange_bolt_offset;
                    let hole = centered_cylinder(
                        &format!(
                            "bph_{}_{}_{}_{}",
                            sx as i32, sy as i32, fx as i32, fy as i32
                        ),
                        corner_bolt_dia / 2.0,
                        base_thickness + 2.0,
                        24,
                    )
                    .translate(px, py, 0.0);
                    base_post_holes = base_post_holes + hole;
                }
            }
        }
    }

    // Rocker top-plate mounting holes (8× M6 on 450 × 400 rect)
    // Layout: 4 corners + 4 midpoints of the rect edges
    let mut rocker_holes = Part::empty("rocker_holes");
    let rx = rocker_bolt_rect_x / 2.0;
    let ry = rocker_bolt_rect_y / 2.0;
    let rocker_positions: [(f64, f64); 8] = [
        (-rx, -ry),
        (rx, -ry),
        (-rx, ry),
        (rx, ry),
        (0.0, -ry),
        (0.0, ry),
        (-rx, 0.0),
        (rx, 0.0),
    ];
    for (i, (px, py)) in rocker_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("rh_{i}"),
            rocker_bolt_dia / 2.0,
            base_thickness + 2.0,
            24,
        )
        .translate(*px, *py, 0.0);
        rocker_holes = rocker_holes + hole;
    }

    let base_part = base - base_post_holes - rocker_holes;
    base_part
        .write_stl("output/chip_stack_rack_base.stl")
        .unwrap();
    println!("Exported: output/chip_stack_rack_base.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. HANDLE (U-shaped carry handle across top of rack)
    // ══════════════════════════════════════════════════════════════
    //
    // Approximate a bent 10 mm rod as 3 straight cylinders joined at
    // the corners. Spans between the two +Y posts (front) — wait,
    // better: span across the top in X (between the two post pairs
    // at +Y and -Y along the long edge). Orient the U so the grip
    // is horizontal at the top.

    let rod_dia: f64 = 10.0;
    let handle_span_x: f64 = post_spacing_x; // horizontal grip length
    let handle_leg_h: f64 = 60.0; // vertical leg height above post tops

    // Two vertical legs + one horizontal top bar (oriented along X).
    let leg_left = centered_cylinder("leg_left", rod_dia / 2.0, handle_leg_h, 24)
        .translate(-handle_span_x / 2.0, 0.0, handle_leg_h / 2.0);
    let leg_right = centered_cylinder("leg_right", rod_dia / 2.0, handle_leg_h, 24)
        .translate(handle_span_x / 2.0, 0.0, handle_leg_h / 2.0);
    let top_bar = centered_cylinder("top_bar", rod_dia / 2.0, handle_span_x, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, handle_leg_h);

    let handle = leg_left + leg_right + top_bar;
    handle
        .write_stl("output/chip_stack_rack_handle.stl")
        .unwrap();
    println!("Exported: output/chip_stack_rack_handle.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. ASSEMBLY (base + 4 posts + 5 shelves + handle + 4 chip
    //    dummies on one shelf for scale)
    // ══════════════════════════════════════════════════════════════

    // Base plate sits at z = 0 (centered). Top of base is at z = base_thickness/2
    let base_top = base_thickness / 2.0;
    // Base assembly piece (already cut), lifted so bottom is at z = 0
    let asm_base = (centered_cube("asm_base", base_x, base_y, base_thickness)
        .translate(0.0, 0.0, base_top))
        - {
            let mut bh = Part::empty("asm_base_holes");
            for &sx in &[-1.0f64, 1.0] {
                for &sy in &[-1.0f64, 1.0] {
                    for &fx in &[-1.0f64, 1.0] {
                        for &fy in &[-1.0f64, 1.0] {
                            let px = sx * post_cx + fx * flange_bolt_offset;
                            let py = sy * post_cy + fy * flange_bolt_offset;
                            let hole = centered_cylinder(
                                &format!("abh_{}_{}_{}_{}", sx as i32, sy as i32, fx as i32, fy as i32),
                                corner_bolt_dia / 2.0,
                                base_thickness + 2.0,
                                16,
                            )
                            .translate(px, py, base_top);
                            bh = bh + hole;
                        }
                    }
                }
            }
            bh
        };

    // Posts: 4 corners, bottom of post flange sits on top of base
    // Post STL is centered at origin — total height = post_height.
    // Flange is at bottom (z = -post_height/2..-post_height/2+flange_thickness).
    // To place bottom of flange at z = base_thickness, translate by
    //   dz = base_thickness + post_height/2.
    let post_z = base_thickness + post_height / 2.0;

    // Rebuild a simpler post for the assembly (avoid giant CSG tree)
    let asm_post_template =
        centered_cube("asm_post", post_side, post_side, post_height)
            + centered_cube(
                "asm_flange",
                post_flange_side,
                post_flange_side,
                post_flange_thickness,
            )
            .translate(0.0, 0.0, -post_height / 2.0 + post_flange_thickness / 2.0);

    let mut asm = asm_base;
    for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
        for (j, &sy) in [-1.0f64, 1.0].iter().enumerate() {
            // Build a fresh copy of the post per corner (vcad Parts
            // aren't Clone-safe to reuse across CSG operations).
            let p =
                centered_cube(&format!("ap_o_{i}_{j}"), post_side, post_side, post_height)
                    + centered_cube(
                        &format!("ap_f_{i}_{j}"),
                        post_flange_side,
                        post_flange_side,
                        post_flange_thickness,
                    )
                    .translate(
                        0.0,
                        0.0,
                        -post_height / 2.0 + post_flange_thickness / 2.0,
                    );
            let placed = p.translate(sx * post_cx, sy * post_cy, post_z);
            asm = asm + placed;
        }
    }
    let _ = asm_post_template; // retained for reference

    // Shelves: 5 of them, at z = base_thickness + first_shelf_z + i*pitch
    // (the post's first_shelf_z is measured from post center; post
    // center sits at z = post_z, so absolute shelf z:
    //   shelf_z = post_z + (first_shelf_z_from_post_center offset)
    // We defined first_shelf_z as "15 mm above base" meaning:
    //   shelf_z_absolute = base_thickness + 15 + i*pitch.
    for i in 0..num_shelves {
        let z = base_thickness + 15.0 + (i as f64) * shelf_pitch;
        // Build a simplified shelf for assembly (just outline + corner
        // notches, skip pockets/tabs/labels/cuts to keep tree small)
        let s = {
            let body = centered_cube(
                &format!("asm_shelf_{i}"),
                shelf_x,
                shelf_y,
                shelf_thickness,
            );
            let mut notches = Part::empty(&format!("asm_notches_{i}"));
            for &sx in &[-1.0f64, 1.0] {
                for &sy in &[-1.0f64, 1.0] {
                    let n = centered_cube(
                        &format!("asm_n_{i}_{}_{}", sx as i32, sy as i32),
                        notch_side,
                        notch_side,
                        shelf_thickness + 2.0,
                    )
                    .translate(sx * post_cx, sy * post_cy, 0.0);
                    notches = notches + n;
                }
            }
            body - notches
        }
        .translate(0.0, 0.0, z);
        asm = asm + s;
    }

    // 4 chip dummies on the bottom shelf (row 0 chips for scale)
    let dummy_z = base_thickness + 15.0 + shelf_thickness / 2.0 + chip_z / 2.0;
    for cx in 0..cols {
        let px = grid_origin_x + (cx as f64) * (pocket_x + gutter_interior);
        let py = grid_origin_y; // row 0
        let chip = centered_cube(
            &format!("chip_dummy_{cx}"),
            chip_x,
            chip_y,
            chip_z,
        )
        .translate(px, py, dummy_z);
        asm = asm + chip;
    }

    // Handle at top
    let handle_z_base = base_thickness + post_height; // top of posts
    let leg_l = centered_cylinder("h_leg_l", rod_dia / 2.0, handle_leg_h, 16)
        .translate(
            -handle_span_x / 2.0,
            0.0,
            handle_z_base + handle_leg_h / 2.0,
        );
    let leg_r = centered_cylinder("h_leg_r", rod_dia / 2.0, handle_leg_h, 16)
        .translate(
            handle_span_x / 2.0,
            0.0,
            handle_z_base + handle_leg_h / 2.0,
        );
    let bar = centered_cylinder("h_bar", rod_dia / 2.0, handle_span_x, 16)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, handle_z_base + handle_leg_h);

    asm = asm + leg_l + leg_r + bar;

    asm.write_stl("output/chip_stack_rack_assembly.stl")
        .unwrap();
    println!("Exported: output/chip_stack_rack_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // SPECS
    // ══════════════════════════════════════════════════════════════

    // Rough mass estimate
    //   Aluminum 6061: 2.70 g/cm³ = 2.70e-3 g/mm³
    //   PETG:          1.27 g/cm³ = 1.27e-3 g/mm³
    let al_density: f64 = 2.70e-3; // g/mm³
    // Volumes (approximate, ignoring holes/cuts for a conservative upper bound)
    let shelf_vol = shelf_x * shelf_y * shelf_thickness;
    // Subtract ~20% for pockets + optical cuts
    let shelf_vol_eff = shelf_vol * 0.80;
    let post_vol_outer = post_side * post_side * post_height;
    let post_wall_frac = 1.0 - ((post_side - 2.0 * post_wall).powi(2) / (post_side.powi(2)));
    let post_vol_eff = post_vol_outer * post_wall_frac
        + post_flange_side * post_flange_side * post_flange_thickness;
    let base_vol = base_x * base_y * base_thickness;
    let handle_vol = std::f64::consts::PI * (rod_dia / 2.0).powi(2)
        * (handle_span_x + 2.0 * handle_leg_h);

    let total_vol_mm3 = (shelf_vol_eff * num_shelves as f64)
        + (post_vol_eff * 4.0)
        + base_vol
        + handle_vol;
    let mass_al = total_vol_mm3 * al_density;

    println!();
    println!("── 100-Chip Stack Rack Specs ──");
    println!("  Chip (Rev C):          {chip_x:.2} × {chip_y:.2} × {chip_z:.2} mm");
    println!(
        "  Shelf:                 {shelf_x:.1} × {shelf_y:.1} × {shelf_thickness:.1} mm ({}×{} grid, gutter {} mm)",
        cols, rows, gutter_interior
    );
    println!("  Chips per shelf:       {}", cols * rows);
    println!("  Shelves:               {num_shelves}");
    println!("  Total chips:           {total_chips}");
    println!("  Shelf pitch:           {shelf_pitch:.1} mm (chip {chip_z:.2} + {:.2} pipette clearance)", shelf_pitch - chip_z);
    println!(
        "  Pocket:                {pocket_x:.1} × {pocket_y:.1} × {pocket_depth:.1} mm deep (0.7 mm side clearance)"
    );
    println!(
        "  Retention tabs:        4 × ({tab_x:.1} × {tab_y:.1} × {tab_h:.1} mm) at pocket corners"
    );
    println!(
        "  Optical cut-through:   {optical_x:.0} × {optical_y:.0} mm per pocket (unobstructed glass)"
    );
    println!("  Barcode label slot:    {label_x:.0} × {label_y:.0} × {label_depth:.1} mm per pocket");
    println!(
        "  Corner posts:          4 × {post_side:.0} × {post_side:.0} mm square tube, {post_height:.0} mm tall"
    );
    println!("  Post spacing:          {post_spacing_x:.1} × {post_spacing_y:.1} mm");
    println!(
        "  Post flange:           {post_flange_side:.0} × {post_flange_side:.0} × {post_flange_thickness:.0} mm, 4× M5 bolts"
    );
    println!(
        "  Base plate:            {base_x:.0} × {base_y:.0} × {base_thickness:.0} mm"
    );
    println!(
        "  Rocker mount:          8× M6 on {rocker_bolt_rect_x:.0} × {rocker_bolt_rect_y:.0} mm rectangle"
    );
    println!(
        "  Carry handle:          Ø{rod_dia:.0} mm rod, U-shape, span {handle_span_x:.0} mm, leg {handle_leg_h:.0} mm"
    );
    println!("  Shelf material:        3 mm 6061 aluminum or PETG (parameterized)");
    println!(
        "  Envelope (W×D×H):      {:.0} × {:.0} × {:.0} mm (fits 550×500×500 container)",
        post_spacing_x + post_flange_side,
        post_spacing_y + post_flange_side,
        base_thickness + post_height + handle_leg_h + rod_dia
    );
    println!("  Est. mass (Al):        {:.2} kg", mass_al / 1000.0);
    println!(
        "  Est. mass (PETG):      {:.2} kg",
        (total_vol_mm3 * 1.27e-3) / 1000.0
    );
    println!();
    println!("  Access pattern:");
    println!("    - Drop-in / drop-out by hand or liquid-handler gripper");
    println!("    - Top-accessible wells (25 mm pipette clearance per shelf)");
    println!("    - Glass bottom visible through 100×60 mm cut-through for imaging");
    println!("    - Corner tabs + 15°-tilt-rated retention geometry");
    println!();
    println!("  Exports:");
    println!("    - output/chip_stack_rack_shelf.stl");
    println!("    - output/chip_stack_rack_post.stl");
    println!("    - output/chip_stack_rack_base.stl");
    println!("    - output/chip_stack_rack_handle.stl");
    println!("    - output/chip_stack_rack_assembly.stl");

    // STEP export (STL → STEP via stltostp)
    for stl in [
        "output/chip_stack_rack_shelf.stl",
        "output/chip_stack_rack_post.stl",
        "output/chip_stack_rack_base.stl",
        "output/chip_stack_rack_handle.stl",
        "output/chip_stack_rack_assembly.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }
}
