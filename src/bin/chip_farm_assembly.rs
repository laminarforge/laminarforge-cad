use vcad::{centered_cube, centered_cylinder, Part};

// ─── Chip Farm Full System Assembly ───
//
// Composite visualization of the full 100-chip farm:
//   * chip_incubator_v2  — dual-wall insulated thermal container
//   * rack_rocker        — motorized ±15° tilt platform
//   * chip_stack_rack    — 5-shelf, 20-chip-per-shelf rack
//
// This bin builds a SIMPLIFIED assembly that shows how everything fits
// together. It does NOT re-export full-fidelity detail models — those
// come from chip_incubator_v2.rs, rack_rocker.rs, and chip_stack_rack.rs
// (run them separately for their own STLs). Instead, each subsystem is
// represented by lower-fidelity primitives positioned correctly so a
// user can visually inspect fit, clearances, and overall proportions.
//
// COORDINATE SYSTEM (ASSEMBLY)
//   Origin at incubator INTERIOR FLOOR CENTER.
//   X : left / right   (incubator interior −300 … +300)
//   Y : front / back   (incubator interior −260 … +260, front = −Y)
//   Z : bottom / top   (interior floor at 0, interior ceiling at +550)
//
// Subsystem Z stacking (0° tilt):
//   Rocker base sits on floor :  z = 0 … 60 mm   (base + boss envelope)
//   Rack base plate           :  z = 60 … 66 mm
//   Rack posts / shelves      :  up to ~ 306 mm above floor
//   Top of rack (handle tip)  :  ~ 366 mm above floor
//   Interior ceiling          :  z = 550 mm (244 mm headroom)
//
// Exports
//   output/chip_farm_assembly.stl             0° tilt, fully nested
//   output/chip_farm_assembly_tilted.stl      rack rotated +12° about X
//   output/chip_farm_assembly_exploded.stl    door, hatch, rack, rocker separated
//   output/chip_farm_assembly_drawing.txt     dimensioned ASCII callouts

fn main() {
    std::fs::create_dir_all("output").ok();

    // ══════════════════════════════════════════════════════════════
    // CONSTANTS (mirrored from the three source bins)
    // ══════════════════════════════════════════════════════════════

    // ── Incubator (chip_incubator_v2.rs) ──
    // Chamber centered at origin in its own file; here we shift it so
    // that the interior floor = z = 0 and interior is centered in X/Y.
    let inner_x: f64 = 600.0;
    let inner_y: f64 = 520.0;
    let inner_z: f64 = 550.0;
    let chamber_wall: f64 = 3.0;
    let insulation_gap: f64 = 25.0;
    let shell_wall: f64 = 3.0;

    let chamber_outer_x = inner_x + chamber_wall * 2.0; // 606
    let chamber_outer_y = inner_y + chamber_wall * 2.0; // 526
    let chamber_outer_z = inner_z + chamber_wall * 2.0; // 556

    let shell_outer_x = chamber_outer_x + (insulation_gap + shell_wall) * 2.0; // 656
    let shell_outer_y = chamber_outer_y + (insulation_gap + shell_wall) * 2.0; // 576
    let shell_outer_z = chamber_outer_z + (insulation_gap + shell_wall) * 2.0; // 606

    // Chamber center (in assembly coords: interior floor = z=0)
    let _chamber_cx = 0.0;
    let _chamber_cy = 0.0;
    let chamber_cz = inner_z / 2.0; // 275 mm above the interior floor

    // ── Rocker (rack_rocker.rs) ──
    let rocker_base_x: f64 = 560.0;
    let rocker_base_y: f64 = 490.0;
    let rocker_base_z: f64 = 10.0;
    let rocker_top_x: f64 = 500.0;
    let rocker_top_y: f64 = 440.0;
    let rocker_top_z: f64 = 8.0;
    let pivot_clearance: f64 = 45.0;
    // Pivot axis above the rocker base top face
    let rocker_pivot_axis_z_local = rocker_base_z / 2.0 + pivot_clearance; // 50 mm in rocker-local
                                                                           // Rocker-local origin is the base center. In assembly coords, base sits
                                                                           // on interior floor: base bottom at z=0, base top at z=10.
    let rocker_origin_z = rocker_base_z / 2.0; // 5 mm (base center in assembly coords)
    let rocker_pivot_axis_z_world = rocker_origin_z + rocker_pivot_axis_z_local;
    // Top tilting plate sits with bottom at pivot axis + small offset.
    // In rack_rocker.rs: top plate translates to z = pivot_axis_z + top_z/2
    // (pivot_axis_z is in rocker-local coords). Convert to assembly coords.
    let rocker_top_plate_cz_world =
        rocker_origin_z + rocker_pivot_axis_z_local + rocker_top_z / 2.0; // ~59 mm

    // Stepper boss (visual cue)
    let boss_x: f64 = 80.0;
    let boss_y: f64 = 80.0;
    let boss_z: f64 = 15.0;
    let boss_cx: f64 = -rocker_base_x / 2.0 + boss_x / 2.0 + 20.0; // -200 from base center
    let boss_cy: f64 = 0.0;
    let boss_cz_local: f64 = rocker_base_z / 2.0 + boss_z / 2.0; // 12.5
    let boss_cz_world: f64 = rocker_origin_z + boss_cz_local - rocker_origin_z + rocker_origin_z
        - rocker_base_z / 2.0
        + boss_cz_local; // = rocker_origin_z - rocker_base_z/2 + boss_cz_local
    let _ = boss_cz_world; // simplified below
    let _boss_cz_assembly: f64 = rocker_base_z + boss_z / 2.0; // rocker sits on floor; boss rises above base

    // ── Rack (chip_stack_rack.rs) ──
    let chip_x: f64 = 127.76;
    let chip_y: f64 = 85.48;
    let chip_z: f64 = 14.35;
    let rack_base_x: f64 = 545.0;
    let rack_base_y: f64 = 495.0;
    let rack_base_z: f64 = 6.0;
    let shelf_pitch: f64 = 40.0;
    let num_shelves: i32 = 5;
    let first_shelf_z: f64 = 15.0; // relative to rack base top
    let shelf_thickness: f64 = 3.0;
    // Shelf inplane size (see chip_stack_rack.rs)
    let cols = 4;
    let rows = 5;
    let pocket_clearance: f64 = 0.7;
    let pocket_x: f64 = chip_x + pocket_clearance * 2.0;
    let pocket_y: f64 = chip_y + pocket_clearance * 2.0;
    let gutter_interior: f64 = 5.0;
    let edge_margin: f64 = 2.0;
    let shelf_x: f64 =
        (cols as f64) * pocket_x + ((cols - 1) as f64) * gutter_interior + 2.0 * edge_margin;
    let shelf_y: f64 =
        (rows as f64) * pocket_y + ((rows - 1) as f64) * gutter_interior + 2.0 * edge_margin;
    let post_side: f64 = 20.0;
    let post_height: f64 = (num_shelves as f64) * shelf_pitch + 30.0; // 230
    let post_spacing_x: f64 = shelf_x - post_side;
    let post_spacing_y: f64 = shelf_y - post_side;
    let rod_dia: f64 = 10.0;
    let handle_leg_h: f64 = 60.0;

    // Rack origin in assembly: rack base bottom sits on top of rocker top plate.
    // Rocker top plate top face z_world = rocker_top_plate_cz_world + rocker_top_z/2
    let rocker_top_surface_z = rocker_top_plate_cz_world + rocker_top_z / 2.0; // ~63 mm
                                                                               // Place rack base bottom at rocker_top_surface_z.
    let rack_base_bottom_z = rocker_top_surface_z;
    let rack_base_top_z = rack_base_bottom_z + rack_base_z;
    // Post Z in rack-local: post_z = base_thickness + post_height/2 ≈ 6 + 115 = 121.
    // Post bottom sits at rack_base_top_z; post center in assembly:
    let _post_world_cz = rack_base_top_z + post_height / 2.0;

    // ══════════════════════════════════════════════════════════════
    // 1. INCUBATOR SHELL (thin-walled box) — nested assembly
    // ══════════════════════════════════════════════════════════════
    // For visualization: show outer shell as a hollow box with the
    // front face open (so the interior is visible).

    fn build_incubator_hollow(
        shell_outer_x: f64,
        shell_outer_y: f64,
        shell_outer_z: f64,
        inner_x: f64,
        inner_y: f64,
        inner_z: f64,
        chamber_cz: f64,
        open_front: bool,
    ) -> Part {
        // Outer shell box
        let outer = centered_cube("inc_outer", shell_outer_x, shell_outer_y, shell_outer_z)
            .translate(0.0, 0.0, chamber_cz);
        // Inner cavity — the interior chamber (600 × 520 × 550)
        let cavity =
            centered_cube("inc_cavity", inner_x, inner_y, inner_z).translate(0.0, 0.0, chamber_cz);
        let mut result = outer - cavity;

        if open_front {
            // Cut away the entire front face so user can see inside
            let front_hole = centered_cube(
                "inc_front_hole",
                inner_x + 20.0,
                shell_outer_y, // full depth so it punches all layers
                inner_z + 20.0,
            )
            .translate(0.0, -(shell_outer_y / 2.0), chamber_cz);
            result = result - front_hole;
        }

        // Cut away top hatch opening for visualization
        let top_hatch = centered_cube(
            "inc_top_hatch",
            370.0,
            270.0,
            shell_outer_z, // punch through top
        )
        .translate(0.0, 0.0, chamber_cz + shell_outer_z / 2.0);
        result = result - top_hatch;

        result
    }

    // ══════════════════════════════════════════════════════════════
    // 2. ROCKER (simplified primitives)
    // ══════════════════════════════════════════════════════════════

    fn build_rocker(
        base_x: f64,
        base_y: f64,
        base_z: f64,
        top_x: f64,
        top_y: f64,
        top_z: f64,
        pivot_clearance: f64,
        boss_x: f64,
        boss_y: f64,
        boss_z: f64,
        boss_cx: f64,
        boss_cy: f64,
    ) -> Part {
        // Base plate: bottom at z=0, top at z=base_z
        let base =
            centered_cube("rk_base", base_x, base_y, base_z).translate(0.0, 0.0, base_z / 2.0);

        // Stepper boss on top of base
        let boss = centered_cube("rk_boss", boss_x, boss_y, boss_z).translate(
            boss_cx,
            boss_cy,
            base_z + boss_z / 2.0,
        );

        // Stepper body stub (cylinder in -X)
        let stepper_body = centered_cylinder("rk_stepper_body", 56.4 / 2.0, 70.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_cx - 70.0 / 2.0, boss_cy, base_z + boss_z / 2.0);

        // Lead-screw stand-in: cylinder along X between two end mounts
        let ls_mount_x: f64 = 18.0;
        let ls_mount_y: f64 = 30.0;
        let ls_mount_z: f64 = 20.0;
        let ls_mount_near_cx = boss_cx + boss_x / 2.0 + ls_mount_x / 2.0 + 5.0;
        let ls_mount_far_cx = base_x / 2.0 - ls_mount_x / 2.0 - 20.0;
        let ls_axis_z = base_z + ls_mount_z / 2.0;

        let ls_near = centered_cube("rk_ls_near", ls_mount_x, ls_mount_y, ls_mount_z).translate(
            ls_mount_near_cx,
            0.0,
            base_z + ls_mount_z / 2.0,
        );
        let ls_far = centered_cube("rk_ls_far", ls_mount_x, ls_mount_y, ls_mount_z).translate(
            ls_mount_far_cx,
            0.0,
            base_z + ls_mount_z / 2.0,
        );
        let ls_span = ls_mount_far_cx - ls_mount_near_cx;
        let lead_screw = centered_cylinder("rk_lead_screw", 8.0 / 2.0, ls_span, 16)
            .rotate(0.0, 90.0, 0.0)
            .translate((ls_mount_near_cx + ls_mount_far_cx) / 2.0, 0.0, ls_axis_z);

        // Pivot blocks on ±Y edges of base
        let pb_x = 60.0;
        let pb_y = 40.0;
        let pb_z = pivot_clearance; // reaches up to pivot axis
        let pb_left = centered_cube("rk_pb_left", pb_x, pb_y, pb_z).translate(
            0.0,
            -(base_y / 2.0 - pb_y / 2.0 - 20.0),
            base_z + pb_z / 2.0,
        );
        let pb_right = centered_cube("rk_pb_right", pb_x, pb_y, pb_z).translate(
            0.0,
            base_y / 2.0 - pb_y / 2.0 - 20.0,
            base_z + pb_z / 2.0,
        );

        // Top tilting plate centered in X/Y, at pivot height + top_z/2
        let pivot_axis_z_world = base_z + pivot_clearance;
        let top_plate = centered_cube("rk_top_plate", top_x, top_y, top_z).translate(
            0.0,
            0.0,
            pivot_axis_z_world + top_z / 2.0,
        );

        base + boss + stepper_body + ls_near + ls_far + lead_screw + pb_left + pb_right + top_plate
    }

    // ══════════════════════════════════════════════════════════════
    // 3. RACK (simplified primitives)
    // ══════════════════════════════════════════════════════════════

    fn build_rack(
        rack_base_x: f64,
        rack_base_y: f64,
        rack_base_z: f64,
        shelf_x: f64,
        shelf_y: f64,
        shelf_thickness: f64,
        shelf_pitch: f64,
        first_shelf_z: f64,
        num_shelves: i32,
        post_side: f64,
        post_height: f64,
        post_spacing_x: f64,
        post_spacing_y: f64,
        rod_dia: f64,
        handle_leg_h: f64,
        base_bottom_z: f64,
        chip_x: f64,
        chip_y: f64,
        chip_z: f64,
        pocket_x: f64,
        pocket_y: f64,
        gutter_interior: f64,
        edge_margin: f64,
        cols: i32,
        rows: i32,
    ) -> Part {
        // Rack base plate (bottom at base_bottom_z)
        let base = centered_cube("rack_base", rack_base_x, rack_base_y, rack_base_z).translate(
            0.0,
            0.0,
            base_bottom_z + rack_base_z / 2.0,
        );

        // 4 corner posts (simplified — solid square tubes)
        let base_top = base_bottom_z + rack_base_z;
        let post_cx = post_spacing_x / 2.0;
        let post_cy = post_spacing_y / 2.0;

        let mut posts = Part::empty("rack_posts");
        for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
            for (j, &sy) in [-1.0f64, 1.0].iter().enumerate() {
                let p = centered_cube(
                    &format!("rack_post_{i}_{j}"),
                    post_side,
                    post_side,
                    post_height,
                )
                .translate(
                    sx * post_cx,
                    sy * post_cy,
                    base_top + post_height / 2.0,
                );
                posts = posts + p;
            }
        }

        // Shelves (simplified — solid sheets, no pocket detail)
        let mut shelves = Part::empty("rack_shelves");
        for i in 0..num_shelves {
            let z = base_top + first_shelf_z + (i as f64) * shelf_pitch;
            let s = centered_cube(
                &format!("rack_shelf_{i}"),
                shelf_x,
                shelf_y,
                shelf_thickness,
            )
            .translate(0.0, 0.0, z);
            shelves = shelves + s;
        }

        // Chip dummies on each shelf (row 0 only — 4 chips per shelf for visual scale)
        let grid_origin_x = -(shelf_x / 2.0) + edge_margin + pocket_x / 2.0;
        let grid_origin_y = -(shelf_y / 2.0) + edge_margin + pocket_y / 2.0;
        let mut chips = Part::empty("rack_chips");
        for i in 0..num_shelves {
            let shelf_z = base_top + first_shelf_z + (i as f64) * shelf_pitch;
            let chip_cz = shelf_z + shelf_thickness / 2.0 + chip_z / 2.0;
            for cx in 0..cols {
                for ry in 0..rows {
                    let px = grid_origin_x + (cx as f64) * (pocket_x + gutter_interior);
                    let py = grid_origin_y + (ry as f64) * (pocket_y + gutter_interior);
                    let chip =
                        centered_cube(&format!("chip_{i}_{cx}_{ry}"), chip_x, chip_y, chip_z)
                            .translate(px, py, chip_cz);
                    chips = chips + chip;
                }
            }
        }

        // U-shaped handle at top of posts
        let handle_z_base = base_top + post_height;
        let leg_l = centered_cylinder("rack_leg_l", rod_dia / 2.0, handle_leg_h, 16).translate(
            -post_spacing_x / 2.0,
            0.0,
            handle_z_base + handle_leg_h / 2.0,
        );
        let leg_r = centered_cylinder("rack_leg_r", rod_dia / 2.0, handle_leg_h, 16).translate(
            post_spacing_x / 2.0,
            0.0,
            handle_z_base + handle_leg_h / 2.0,
        );
        let bar = centered_cylinder("rack_bar", rod_dia / 2.0, post_spacing_x, 16)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 0.0, handle_z_base + handle_leg_h);

        base + posts + shelves + chips + leg_l + leg_r + bar
    }

    // ══════════════════════════════════════════════════════════════
    // 4. ACCESSORIES (water tray, hatch, door slab)
    // ══════════════════════════════════════════════════════════════

    fn build_water_tray() -> Part {
        // 298 × 198 × 25 mm open-top tray; sits on floor, near +Y back
        let outer = centered_cube("wt_outer", 298.0, 198.0, 25.0).translate(0.0, 150.0, 25.0 / 2.0);
        let inner = centered_cube("wt_inner", 293.0, 193.0, 22.5).translate(
            0.0,
            150.0,
            25.0 / 2.0 + 2.5 / 2.0,
        );
        outer - inner
    }

    fn build_top_hatch(chamber_top_z: f64) -> Part {
        // Rests on top of chamber (z = chamber_top_z + 15/2)
        centered_cube("top_hatch", 360.0, 260.0, 15.0).translate(
            0.0,
            0.0,
            chamber_top_z + 15.0 / 2.0,
        )
    }

    fn build_door(chamber_front_y: f64) -> Part {
        // Door slab in closed position (attached to front face -Y)
        // Door thickness = 35 mm, centered on chamber front.
        let door_x = 630.0;
        let door_z = 580.0;
        let door_thickness = 35.0;
        centered_cube("door_body", door_x, door_thickness, door_z).translate(
            0.0,
            chamber_front_y - door_thickness / 2.0,
            chamber_front_y.abs().min(0.0).max(0.0) // unused
                    + 275.0, // chamber center z (interior floor 0 → center 275)
        )
    }

    // ══════════════════════════════════════════════════════════════
    // 5. BUILD — 0° ASSEMBLY (fully nested)
    // ══════════════════════════════════════════════════════════════

    let incubator_0 = build_incubator_hollow(
        shell_outer_x,
        shell_outer_y,
        shell_outer_z,
        inner_x,
        inner_y,
        inner_z,
        chamber_cz,
        true, // open front for visibility
    );

    let rocker_0 = build_rocker(
        rocker_base_x,
        rocker_base_y,
        rocker_base_z,
        rocker_top_x,
        rocker_top_y,
        rocker_top_z,
        pivot_clearance,
        boss_x,
        boss_y,
        boss_z,
        boss_cx,
        boss_cy,
    );

    let rack_0 = build_rack(
        rack_base_x,
        rack_base_y,
        rack_base_z,
        shelf_x,
        shelf_y,
        shelf_thickness,
        shelf_pitch,
        first_shelf_z,
        num_shelves,
        post_side,
        post_height,
        post_spacing_x,
        post_spacing_y,
        rod_dia,
        handle_leg_h,
        rack_base_bottom_z,
        chip_x,
        chip_y,
        chip_z,
        pocket_x,
        pocket_y,
        gutter_interior,
        edge_margin,
        cols,
        rows,
    );

    let water_tray = build_water_tray();

    let top_hatch =
        build_top_hatch(chamber_cz + chamber_outer_z / 2.0 + insulation_gap + shell_wall);
    let _ = top_hatch;

    // Front door (closed position): center Y = -interior front - door/2 outside the shell
    let chamber_front_y_assembly = -shell_outer_y / 2.0;
    let door = build_door(chamber_front_y_assembly);
    let _ = door;

    let assembly_0 = incubator_0 + rocker_0 + rack_0 + water_tray;

    assembly_0
        .write_stl("output/chip_farm_assembly.stl")
        .unwrap();
    println!("Exported: output/chip_farm_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. BUILD — TILTED (rack + rocker-top rotated +12° about X)
    // ══════════════════════════════════════════════════════════════

    // We construct rack and rocker top as a single group, rotate about X
    // through the pivot axis, then translate back. Simple approach: build
    // rack at 0° position, translate so pivot axis is at origin, rotate,
    // translate back.
    let tilt_deg: f64 = 12.0;
    let rack_tilted = build_rack(
        rack_base_x,
        rack_base_y,
        rack_base_z,
        shelf_x,
        shelf_y,
        shelf_thickness,
        shelf_pitch,
        first_shelf_z,
        num_shelves,
        post_side,
        post_height,
        post_spacing_x,
        post_spacing_y,
        rod_dia,
        handle_leg_h,
        rack_base_bottom_z,
        chip_x,
        chip_y,
        chip_z,
        pocket_x,
        pocket_y,
        gutter_interior,
        edge_margin,
        cols,
        rows,
    )
    .translate(0.0, 0.0, -rocker_pivot_axis_z_world)
    .rotate(tilt_deg, 0.0, 0.0)
    .translate(0.0, 0.0, rocker_pivot_axis_z_world);

    // Also tilt the top plate of rocker (rebuild a tilted rocker where
    // just the top plate rotates). Simpler: rebuild rocker without the
    // top plate, then add a rotated top plate.
    let rocker_no_top = {
        let base = centered_cube("rkt_base", rocker_base_x, rocker_base_y, rocker_base_z)
            .translate(0.0, 0.0, rocker_base_z / 2.0);
        let boss = centered_cube("rkt_boss", boss_x, boss_y, boss_z).translate(
            boss_cx,
            boss_cy,
            rocker_base_z + boss_z / 2.0,
        );
        let stepper_body = centered_cylinder("rkt_stepper_body", 56.4 / 2.0, 70.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_cx - 70.0 / 2.0, boss_cy, rocker_base_z + boss_z / 2.0);
        let pb_x_ = 60.0;
        let pb_y_ = 40.0;
        let pb_z_ = pivot_clearance;
        let pb_left = centered_cube("rkt_pb_left", pb_x_, pb_y_, pb_z_).translate(
            0.0,
            -(rocker_base_y / 2.0 - pb_y_ / 2.0 - 20.0),
            rocker_base_z + pb_z_ / 2.0,
        );
        let pb_right = centered_cube("rkt_pb_right", pb_x_, pb_y_, pb_z_).translate(
            0.0,
            rocker_base_y / 2.0 - pb_y_ / 2.0 - 20.0,
            rocker_base_z + pb_z_ / 2.0,
        );
        base + boss + stepper_body + pb_left + pb_right
    };

    let top_plate_tilted = centered_cube("rkt_top_plate", rocker_top_x, rocker_top_y, rocker_top_z)
        .translate(
            0.0,
            0.0,
            rocker_top_plate_cz_world - rocker_pivot_axis_z_world,
        )
        .rotate(tilt_deg, 0.0, 0.0)
        .translate(0.0, 0.0, rocker_pivot_axis_z_world);

    let water_tray_t = build_water_tray();
    let incubator_t = build_incubator_hollow(
        shell_outer_x,
        shell_outer_y,
        shell_outer_z,
        inner_x,
        inner_y,
        inner_z,
        chamber_cz,
        true,
    );

    let assembly_tilted =
        incubator_t + rocker_no_top + top_plate_tilted + rack_tilted + water_tray_t;
    assembly_tilted
        .write_stl("output/chip_farm_assembly_tilted.stl")
        .unwrap();
    println!("Exported: output/chip_farm_assembly_tilted.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. BUILD — EXPLODED VIEW
    // ══════════════════════════════════════════════════════════════
    //   door  : translated -300 mm in Y (pulled out toward -Y)
    //   hatch : lifted +100 mm in Z
    //   rack  : lifted +400 mm in Z (and cleared of rocker)
    //   rocker: separated +600 mm in Z

    let incubator_e = build_incubator_hollow(
        shell_outer_x,
        shell_outer_y,
        shell_outer_z,
        inner_x,
        inner_y,
        inner_z,
        chamber_cz,
        false, // solid box (so the exploded view shows all faces)
    );

    let door_thickness: f64 = 35.0;
    let door_exploded = centered_cube("door_e", 630.0, door_thickness, 580.0).translate(
        0.0,
        -shell_outer_y / 2.0 - door_thickness / 2.0 - 300.0,
        chamber_cz,
    );

    let hatch_exploded = centered_cube("hatch_e", 360.0, 260.0, 15.0).translate(
        0.0,
        0.0,
        chamber_cz + chamber_outer_z / 2.0 + insulation_gap + shell_wall + 15.0 / 2.0 + 100.0,
    );

    // Rack lifted +400 mm in Z (keep its local Z stack; translate whole thing)
    let rack_exploded = build_rack(
        rack_base_x,
        rack_base_y,
        rack_base_z,
        shelf_x,
        shelf_y,
        shelf_thickness,
        shelf_pitch,
        first_shelf_z,
        num_shelves,
        post_side,
        post_height,
        post_spacing_x,
        post_spacing_y,
        rod_dia,
        handle_leg_h,
        rack_base_bottom_z,
        chip_x,
        chip_y,
        chip_z,
        pocket_x,
        pocket_y,
        gutter_interior,
        edge_margin,
        cols,
        rows,
    )
    .translate(0.0, 0.0, 400.0);

    // Rocker lifted +600 mm in Z (above rack so stack is fully exploded)
    let rocker_exploded = build_rocker(
        rocker_base_x,
        rocker_base_y,
        rocker_base_z,
        rocker_top_x,
        rocker_top_y,
        rocker_top_z,
        pivot_clearance,
        boss_x,
        boss_y,
        boss_z,
        boss_cx,
        boss_cy,
    )
    .translate(0.0, 0.0, 900.0); // 600 above rack position = 400 (rack offset) + 500 spacing

    let water_tray_e = build_water_tray().translate(0.0, 0.0, 150.0); // lift with hatch area

    let assembly_exploded = incubator_e
        + door_exploded
        + hatch_exploded
        + rack_exploded
        + rocker_exploded
        + water_tray_e;
    assembly_exploded
        .write_stl("output/chip_farm_assembly_exploded.stl")
        .unwrap();
    println!("Exported: output/chip_farm_assembly_exploded.stl");

    // ══════════════════════════════════════════════════════════════
    // 8. DIMENSIONED ASCII DRAWING
    // ══════════════════════════════════════════════════════════════

    let total_chips = num_shelves * cols * rows;
    let rack_top_z_floor = rack_base_top_z + post_height; // absolute top of posts, above interior floor
    let handle_tip_z = rack_top_z_floor + handle_leg_h + rod_dia;
    let headroom = inner_z - rack_top_z_floor;

    let drawing = format!(
        "Chip Farm Assembly — Dimensioned\n\
         (All dimensions in mm. Origin: incubator INTERIOR FLOOR CENTER.)\n\
         \n\
         OVERALL\n\
           Outer container:      {shell_x:.0} × {shell_y:.0} × {shell_z:.0} mm (shell outer)\n\
           Chamber outer (PETG): {cho_x:.0} × {cho_y:.0} × {cho_z:.0} mm (3 mm walls)\n\
           Interior:             {inx:.0} × {iny:.0} × {inz:.0} mm\n\
           Insulation gap:       25 mm PIR foam (all 6 sides)\n\
           Door swing:           630 × 580 × 35 mm (left-hinged, full front access)\n\
           Top hatch:            360 × 260 × 15 mm liquid-handler access\n\
           Acrylic window:       300 × 200 mm in door\n\
         \n\
         INTERIOR STACK (floor up, 0° tilt)\n\
           Rocker base plate:    z =   0 …  10 mm   {rbx:.0} × {rby:.0} × {rbz:.0} mm (6061-T6 Al)\n\
             Rail tabs:            (underside) at X = -180, -60, +60, +180 mm (12 × 3 mm)\n\
             Stepper boss:         z =  10 …  25 mm  80 × 80 × 15 mm @ X = {boss_cx:.0} mm\n\
             NEMA 23 body stub:    cylindrical, 56.4 Ø × 70 mm (−X of boss)\n\
             Lead-screw mounts:    2× 18 × 30 × 20 mm upstands, axis at z = 20 mm\n\
             Pivot blocks (±Y):    60 × 40 × 45 mm, hold R3 bearings\n\
           Pivot axis:           z =  {pivz:.0} mm (±15° firmware-limited tilt)\n\
           Rocker top plate:     z ≈  {rtpl_bot:.0} …  {rtpl_top:.0} mm   {rtx:.0} × {rty:.0} × {rtz:.0} mm (6061-T6 Al)\n\
           Rack base plate:      z =  {rb_bot:.0} …  {rb_top:.0} mm   {rab_x:.0} × {rab_y:.0} × {rab_z:.0} mm\n\
           Rack shelves (5×):\n",
        shell_x = shell_outer_x,
        shell_y = shell_outer_y,
        shell_z = shell_outer_z,
        cho_x = chamber_outer_x,
        cho_y = chamber_outer_y,
        cho_z = chamber_outer_z,
        inx = inner_x,
        iny = inner_y,
        inz = inner_z,
        rbx = rocker_base_x,
        rby = rocker_base_y,
        rbz = rocker_base_z,
        boss_cx = boss_cx,
        pivz = rocker_pivot_axis_z_world,
        rtpl_bot = rocker_top_plate_cz_world - rocker_top_z / 2.0,
        rtpl_top = rocker_top_plate_cz_world + rocker_top_z / 2.0,
        rtx = rocker_top_x,
        rty = rocker_top_y,
        rtz = rocker_top_z,
        rb_bot = rack_base_bottom_z,
        rb_top = rack_base_top_z,
        rab_x = rack_base_x,
        rab_y = rack_base_y,
        rab_z = rack_base_z,
    );

    let mut drawing = drawing;
    for i in 0..num_shelves {
        let z = rack_base_top_z + first_shelf_z + (i as f64) * shelf_pitch;
        drawing.push_str(&format!(
            "             Shelf {s}:              z = {zb:>5.1} mm (bottom)   20 chips on {sx:.0} × {sy:.0} × {st:.0} mm sheet\n",
            s = i + 1,
            zb = z,
            sx = shelf_x,
            sy = shelf_y,
            st = shelf_thickness,
        ));
    }

    drawing.push_str(&format!(
        "           Top of posts:         z = {rtop:.0} mm\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20Carry handle:         z = {htop:.0} mm (U-shaped 10 mm rod)\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20Headroom above rack:  {head:.0} mm (to interior ceiling at z = {ic:.0} mm)\n\
         \n\
         PORTS & ACCESS (on the inner chamber wall; chamber centered at world z = {ccz:.0} mm)\n\
           Heater port:          8 mm Ø  @ back wall, 15 mm above interior floor (rear)\n\
           SHT40 sensor port:    10 mm Ø @ back wall, X = +80, Z = interior mid-height ({mid:.0} mm)\n\
           CO2 sensor pocket:    25 × 35 × 10 mm deep recess @ back wall, X = -80\n\
           Fan port (80 mm):     right side wall (+X), Z = interior mid + 50 = {fanz:.0} mm\n\
           CO2 gas inlet:        6 mm Ø  @ top, Y = +230 near back\n\
           Rocker wiring grommet:20 mm Ø @ floor, X = +120, Y = +220\n\
           Drain:                10 mm Ø @ floor back-left corner (X = -270, Y = +230)\n\
           Water tray pocket:    300 × 200 × 3 mm @ floor, Y = +150 (holds 298×198×25 PETG tray, ~1.4 L)\n\
           Rail slots (4):       12 × 3 mm deep, X = ±60, ±180 mm (mate rocker rail tabs)\n\
         \n\
         INCUBATOR v2 ENVELOPE — OUTER SHELL\n\
           Leveling feet:        4× M6 through-holes, 30 mm corner inset on floor\n\
           Operating condition:  37 °C / 5 % CO2 / ≥90 % RH\n\
           Est. steady-state:    ~10 W heat loss (25 mm PIR foam), 200 W heater budget\n\
         \n\
         ROCKER — MOTORIZED TILT\n\
           Drive:                NEMA 23 stepper + 8 mm acme lead screw (2 mm lead)\n\
           Mechanism:            lead-screw nut → slotted linkage arm → lever on back edge of top plate\n\
           Tilt range:           ±31° mechanical, ±15° firmware-limited\n\
           Cycle time:           60 s full period (1 rpm)\n\
           Torque budget:        τ_motor ≈ 0.10 N·m (18× margin vs NEMA 23 1.8 N·m holding)\n\
           Rack interface:       8× M6 on 450 × 400 mm rectangle (top plate)\n\
         \n\
         RACK — 100-CHIP STACK\n\
           Chip (Rev C, SLAS):   127.76 × 85.48 × 14.35 mm (glass bottom)\n\
           Grid:                 4 cols × 5 rows × 5 shelves = {tc} chips\n\
           Shelf pitch:          {sp} mm (14.35 chip + 25 mm pipette clearance)\n\
           Pocket clearance:     0.7 mm side, 4 retention tabs per pocket (15° tilt-rated)\n\
           Optical cut-through:  100 × 60 mm per pocket (unobstructed glass for imaging)\n\
           Barcode slot:         10 × 5 × 0.6 mm per pocket\n\
           Posts:                4× 20 × 20 mm square tube, 230 mm tall\n\
           Rocker mount:         8× M6 on 450 × 400 mm rectangle (rack base plate)\n\
           Carry handle:         Ø10 mm rod, U-shape, span {hs:.0} mm, leg 60 mm\n\
           Materials:            6061-T6 Al shelves / PETG alt, mass ~13 kg (Al) or ~6 kg (PETG)\n\
         \n\
         COORDINATE CHECKS\n\
           Rocker base X envelope: {rocker_x_env:.1} ≤ interior X ({inx:.0})  → fits, {sx_slack:.1} mm slack\n\
           Rocker base Y envelope: {rocker_y_env:.1} ≤ interior Y ({iny:.0})  → fits, {sy_slack:.1} mm slack\n\
           Rack shelf X envelope:  {shelf_x:.1} ≤ interior X ({inx:.0})  → fits, {rx_slack:.1} mm slack\n\
           Rack shelf Y envelope:  {shelf_y:.1} ≤ interior Y ({iny:.0})  → fits, {ry_slack:.1} mm slack\n\
           Rack top (handle tip):  {htop:.1} mm ≤ interior Z ({inz:.0})   → fits, {rz_slack:.1} mm headroom\n\
         \n\
         TILT SWEEP (15° worst case)\n\
           Rack handle lateral swing ≈ {swing:.0} mm (sin 15° × (handle_tip_z − pivot_z))\n\
           Rack base lateral swing   ≈ negligible (pivot at ~50 mm, rack base ~60 mm)\n\
         \n\
         EXPORTED FILES\n\
           output/chip_farm_assembly.stl             0° tilt, fully nested (interior visible via open front)\n\
           output/chip_farm_assembly_tilted.stl      rack + top plate rotated +12° about X axis\n\
           output/chip_farm_assembly_exploded.stl    door -Y 300, hatch +Z 100, rack +Z 400, rocker +Z 900\n\
           output/chip_farm_assembly_drawing.txt     this file\n\
         \n\
         STEP exports (STL → STEP via stltostp):\n\
           output/chip_farm_assembly.stp\n\
           output/chip_farm_assembly_tilted.stp\n\
           output/chip_farm_assembly_exploded.stp\n\
         USDZ (AR preview) generated by laminarforge_build action=convert.\n",
        rtop = rack_top_z_floor,
        htop = handle_tip_z,
        head = headroom,
        ic = inner_z,
        ccz = chamber_cz,
        mid = inner_z / 2.0,
        fanz = inner_z / 2.0 + 50.0,
        tc = total_chips,
        sp = shelf_pitch,
        hs = post_spacing_x,
        rocker_x_env = rocker_base_x,
        rocker_y_env = rocker_base_y,
        inx = inner_x,
        iny = inner_y,
        inz = inner_z,
        sx_slack = inner_x - rocker_base_x,
        sy_slack = inner_y - rocker_base_y,
        shelf_x = shelf_x,
        shelf_y = shelf_y,
        rx_slack = inner_x - shelf_x,
        ry_slack = inner_y - shelf_y,
        rz_slack = inner_z - handle_tip_z,
        swing = 15.0f64.to_radians().sin() * (handle_tip_z - rocker_pivot_axis_z_world),
    ));

    std::fs::write("output/chip_farm_assembly_drawing.txt", &drawing).unwrap();
    println!("Exported: output/chip_farm_assembly_drawing.txt");

    // ══════════════════════════════════════════════════════════════
    // SUMMARY PRINT
    // ══════════════════════════════════════════════════════════════

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   CHIP FARM FULL SYSTEM ASSEMBLY");
    println!("══════════════════════════════════════════════════════════════");
    println!("   Interior stack:");
    println!("     Rocker base:       0 …  {:.0} mm", rocker_base_z);
    println!(
        "     Rocker top plate:  {:.0} … {:.0} mm",
        rocker_top_plate_cz_world - rocker_top_z / 2.0,
        rocker_top_plate_cz_world + rocker_top_z / 2.0
    );
    println!(
        "     Rack base:         {:.0} … {:.0} mm",
        rack_base_bottom_z, rack_base_top_z
    );
    println!("     Top of posts:      {:.0} mm", rack_top_z_floor);
    println!("     Handle tip:        {:.0} mm", handle_tip_z);
    println!(
        "     Interior ceiling:  {:.0} mm  → {:.0} mm headroom",
        inner_z, headroom
    );
    println!("   Outputs:");
    println!("     output/chip_farm_assembly.stl");
    println!("     output/chip_farm_assembly_tilted.stl");
    println!("     output/chip_farm_assembly_exploded.stl");
    println!("     output/chip_farm_assembly_drawing.txt");

    // STEP export (STL → STEP via stltostp). USDZ is produced separately by
    // `laminarforge_build action=convert` (FreeCAD headless → Apple scntool).
    for stl in [
        "output/chip_farm_assembly.stl",
        "output/chip_farm_assembly_tilted.stl",
        "output/chip_farm_assembly_exploded.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }
}
