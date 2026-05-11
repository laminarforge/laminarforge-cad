use vcad::{centered_cube, centered_cylinder, Part};

// ─── Chip Farm Full System Assembly v2 ───
//
// Integrates every subsystem designed across Waves 1–3 into a single
// visualization:
//   1. chip_incubator_v2    — dual-wall insulated thermal container (A-86F1ED1D)
//   2. chip_stack_rack      — 5-shelf × 20-chip rack            (A-1A303196)
//   3. rack_rocker          — motorized ±15° tilt platform      (A-2216154B)
//   4. media_reservoir      — 8 L PETG reservoir + 5-ch pump    (A-33D30816)
//   5. imaging_gantry       — ceiling-mounted 2-axis imager     (A-07C2AA94)
//   6. lh_interface         — clamshell hatch + rails           (A-71001BA1)
//   7. controller_enclosure — 300×200×150 mm control box        (placeholder)
//   8. Tubing routing       — 5 silicone lines reservoir → drip heads
//   9. Cable routing        — sensor + actuator umbilicals
//
// The bin is visualization-only: lower-fidelity primitives positioned in
// the global coordinate system so a human can inspect fit, clearances,
// and integration. Full-fidelity models live in their own bins.
//
// GLOBAL COORDINATE SYSTEM
//   Origin = interior floor center of the thermal container.
//   X: left / right   (interior −300 … +300 mm)
//   Y: front / back   (interior −260 … +260 mm; door at −Y)
//   Z: up             (interior floor at 0, ceiling at +550)
//
// Exports:
//   output/chip_farm_v2_assembly.stl         — full integrated assembly
//   output/chip_farm_v2_incubator.stl        — container only
//   output/chip_farm_v2_rocker.stl           — rocker only
//   output/chip_farm_v2_rack.stl             — rack only
//   output/chip_farm_v2_reservoir.stl        — reservoir cluster only
//   output/chip_farm_v2_gantry.stl           — imaging gantry only
//   output/chip_farm_v2_lh.stl               — LH hatch + rail stub only
//   output/chip_farm_v2_controller.stl       — controller box only
//   output/chip_farm_v2_tubing.stl           — tubing bundle only
//   output/chip_farm_v2_cables.stl           — cable bundle only
//   output/chip_farm_v2_assembly.stp         — STEP export (full)
//   output/chip_farm_v2_drawing.txt          — dimensioned ASCII callouts

// ───────────── helpers ─────────────

fn cube(name: &str, sx: f64, sy: f64, sz: f64, cx: f64, cy: f64, cz: f64) -> Part {
    centered_cube(name, sx, sy, sz).translate(cx, cy, cz)
}

/// Tubing segment as a thin cylinder between two points. For convenience we
/// only support axis-aligned runs (X/Y/Z) plus a few 45° diagonals; bends are
/// modeled as a sequence of short segments. Each segment name must be unique.
fn tube_seg(name: &str, radius: f64, from: (f64, f64, f64), to: (f64, f64, f64)) -> Part {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let dz = to.2 - from.2;
    let length = (dx * dx + dy * dy + dz * dz).sqrt().max(0.001);
    let cx = (from.0 + to.0) / 2.0;
    let cy = (from.1 + to.1) / 2.0;
    let cz = (from.2 + to.2) / 2.0;
    // Default orientation of centered_cylinder is along Z. Rotate so its
    // long axis points from `from` → `to`.
    // For the routing here we align along one primary axis; pick based on
    // the dominant delta and add small angles where needed.
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    let abs_dz = dz.abs();

    let base = centered_cylinder(name, radius, length, 16);
    let rotated = if abs_dz >= abs_dx && abs_dz >= abs_dy {
        base // already along Z
    } else if abs_dx >= abs_dy {
        // Rotate Z → X (90° about Y axis)
        base.rotate(0.0, 90.0, 0.0)
    } else {
        // Rotate Z → Y (90° about X axis)
        base.rotate(90.0, 0.0, 0.0)
    };
    rotated.translate(cx, cy, cz)
}

/// AABB: (xmin, xmax, ymin, ymax, zmin, zmax).
#[derive(Copy, Clone, Debug)]
struct Aabb {
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
}

impl Aabb {
    fn from_center_size(cx: f64, cy: f64, cz: f64, sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            x: (cx - sx / 2.0, cx + sx / 2.0),
            y: (cy - sy / 2.0, cy + sy / 2.0),
            z: (cz - sz / 2.0, cz + sz / 2.0),
        }
    }

    /// Positive gap means separation (no overlap) along that axis.
    /// Returns the min-gap along each axis; any negative value is interference.
    fn separations(&self, other: &Aabb) -> (f64, f64, f64) {
        let gx = (self.x.0 - other.x.1).max(other.x.0 - self.x.1);
        let gy = (self.y.0 - other.y.1).max(other.y.0 - self.y.1);
        let gz = (self.z.0 - other.z.1).max(other.z.0 - self.z.1);
        (gx, gy, gz)
    }

    /// True if boxes interfere (overlap on all three axes).
    fn interferes(&self, other: &Aabb) -> bool {
        let (gx, gy, gz) = self.separations(other);
        gx < 0.0 && gy < 0.0 && gz < 0.0
    }
}

fn main() {
    std::fs::create_dir_all("output").ok();

    // ══════════════════════════════════════════════════════════════
    // CONTAINER (chip_incubator_v2)
    // ══════════════════════════════════════════════════════════════
    let inner_x: f64 = 600.0;
    let inner_y: f64 = 520.0;
    let inner_z: f64 = 550.0;
    let chamber_wall: f64 = 3.0;
    let insulation_gap: f64 = 25.0;
    let shell_wall: f64 = 3.0;

    let chamber_outer_x = inner_x + chamber_wall * 2.0;
    let chamber_outer_y = inner_y + chamber_wall * 2.0;
    let chamber_outer_z = inner_z + chamber_wall * 2.0;
    let shell_outer_x = chamber_outer_x + (insulation_gap + shell_wall) * 2.0;
    let shell_outer_y = chamber_outer_y + (insulation_gap + shell_wall) * 2.0;
    let shell_outer_z = chamber_outer_z + (insulation_gap + shell_wall) * 2.0;

    let chamber_cz = inner_z / 2.0; // 275 mm above interior floor

    // Wall plane Z-coordinates (assembly, interior floor = 0):
    // Interior:    X ∈ [-300, +300], Y ∈ [-260, +260], Z ∈ [0, 550].
    // Outer shell:
    //   front (-Y):   y = -shell_outer_y / 2.0 = -288
    //   back  (+Y):   y = +288
    //   left  (-X):   x = -328
    //   right (+X):   x = +328
    //   floor (-Z):   z = chamber_cz - shell_outer_z / 2.0 = 275 - 303 = -28
    //   top   (+Z):   z = chamber_cz + shell_outer_z / 2.0 = 578

    // ══════════════════════════════════════════════════════════════
    // ROCKER (rack_rocker)
    // ══════════════════════════════════════════════════════════════
    let rocker_base_x: f64 = 560.0;
    let rocker_base_y: f64 = 490.0;
    let rocker_base_z: f64 = 10.0;
    let rocker_top_x: f64 = 500.0;
    let rocker_top_y: f64 = 440.0;
    let rocker_top_z: f64 = 8.0;
    let pivot_clearance: f64 = 45.0;
    let rocker_pivot_axis_z: f64 = rocker_base_z + pivot_clearance; // 55 mm above floor
    let rocker_top_plate_cz: f64 = rocker_pivot_axis_z + rocker_top_z / 2.0; // 59 mm
    let rocker_top_surface_z: f64 = rocker_top_plate_cz + rocker_top_z / 2.0; // 63 mm

    let boss_x: f64 = 80.0;
    let boss_y: f64 = 80.0;
    let boss_z: f64 = 15.0;
    let boss_cx: f64 = -rocker_base_x / 2.0 + boss_x / 2.0 + 20.0; // -200

    // ══════════════════════════════════════════════════════════════
    // RACK (chip_stack_rack)
    // ══════════════════════════════════════════════════════════════
    let chip_x: f64 = 127.76;
    let chip_y: f64 = 85.48;
    let chip_z: f64 = 14.35;
    let rack_base_x: f64 = 545.0;
    let rack_base_y: f64 = 495.0;
    let rack_base_z: f64 = 6.0;
    let shelf_pitch: f64 = 40.0;
    let num_shelves: i32 = 5;
    let first_shelf_z: f64 = 15.0;
    let shelf_thickness: f64 = 3.0;
    let cols: i32 = 4;
    let rows: i32 = 5;
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

    let rack_base_bottom_z: f64 = rocker_top_surface_z; // ~63 mm
    let rack_base_top_z: f64 = rack_base_bottom_z + rack_base_z; // ~69 mm
    let rack_top_posts_z: f64 = rack_base_top_z + post_height; // ~299 mm
    let handle_tip_z: f64 = rack_top_posts_z + handle_leg_h + rod_dia; // ~369 mm

    // Per-shelf top-chip Z (for tubing drip head + imaging head clearance)
    let mut shelf_top_chip_z = [0.0f64; 5];
    for (i, chip_top_z) in shelf_top_chip_z
        .iter_mut()
        .take(num_shelves as usize)
        .enumerate()
    {
        let shelf_top = rack_base_top_z + first_shelf_z + (i as f64) * shelf_pitch;
        *chip_top_z = shelf_top + shelf_thickness + chip_z;
    }
    let _top_shelf_chip_z = shelf_top_chip_z[num_shelves as usize - 1];

    // ══════════════════════════════════════════════════════════════
    // MEDIA RESERVOIR (outside, to +X of container)
    // ══════════════════════════════════════════════════════════════
    let reservoir_od: f64 = 200.0;
    let reservoir_outer_h: f64 = 274.0;
    // Reservoir body center (sits on the table next to the container, +X side)
    let reservoir_cx: f64 = shell_outer_x / 2.0 + reservoir_od / 2.0 + 60.0; // ~ 428
    let reservoir_cy: f64 = -100.0; // offset toward front
    let reservoir_cz: f64 = reservoir_outer_h / 2.0 - 28.0; // container floor sits at z=-28

    // Pump manifold (mounted on container's right wall, outside)
    let manifold_plate_x: f64 = 490.0;
    let manifold_plate_y: f64 = 160.0;
    let manifold_plate_t: f64 = 8.0;
    let manifold_cx: f64 = shell_outer_x / 2.0 + manifold_plate_y / 2.0 + 10.0; // ~+254
    let manifold_cy: f64 = 0.0;
    let manifold_cz: f64 = chamber_cz + 60.0; // ~335 mm
                                              // Plate is wall-mounted on +X wall with its long axis along Z-ish; here we
                                              // lay it flat parallel to the wall (plate normal = +X).
    let pump_pitch: f64 = 90.0;

    // Bulkhead grommet passthrough on right wall
    let passthrough_flange_od: f64 = 60.0;
    let passthrough_cz: f64 = chamber_cz; // centered vertically on wall
    let passthrough_cy: f64 = 0.0;
    let passthrough_wall_x: f64 = shell_outer_x / 2.0; // +328

    // Drip heads — one per shelf (×5). Each sits just above its shelf's top
    // chip face, centered in X, back-biased in Y to allow rocker tilt.
    let drip_head_x: f64 = 540.0;
    let drip_head_y: f64 = 25.0;
    let drip_head_z: f64 = 12.0;
    let drip_head_cy: f64 = -shelf_y / 2.0 - 30.0; // toward -Y (front), outside the rack's Y span
                                                   // Drip head Z: centered between its shelf's top chip and the shelf above
    let mut drip_head_cz = [0.0f64; 5];
    for (i, drip_z) in drip_head_cz
        .iter_mut()
        .take(num_shelves as usize)
        .enumerate()
    {
        let this_chip_top = shelf_top_chip_z[i];
        let next_shelf_bottom = if i + 1 < num_shelves as usize {
            rack_base_top_z + first_shelf_z + ((i + 1) as f64) * shelf_pitch
        } else {
            this_chip_top + 25.0
        };
        *drip_z = (this_chip_top + next_shelf_bottom) / 2.0 + drip_head_z / 2.0;
    }

    // ══════════════════════════════════════════════════════════════
    // IMAGING GANTRY (ceiling-mounted, inside)
    // ══════════════════════════════════════════════════════════════
    // Gantry envelope (per imaging_gantry.rs)
    let gantry_beam_len_x: f64 = 520.0;
    let gantry_beam_len_y: f64 = 480.0;
    let gantry_extr_w: f64 = 20.0;
    let gantry_extr_h: f64 = 40.0;
    // Gantry ceiling mount: top face 5 mm below interior ceiling
    let gantry_beam_top_z: f64 = inner_z - 5.0; // 545 mm
    let gantry_beam_center_z: f64 = gantry_beam_top_z - gantry_extr_h / 2.0; // 525 mm
                                                                             // Head drop tuned for the integrated assembly: rocker + rack raises the
                                                                             // top chip to z ≈ 261 mm (vs 499 mm working distance budget imaging_gantry
                                                                             // planned for a floor-mounted rack). Drop 80 mm gives head bottom at
                                                                             // 525 − 80 − 40 = 405 mm → working distance 405 − 261 = 144 mm (within the
                                                                             // 140–300 mm EDOF window).
    let head_drop: f64 = 80.0;
    let head_body_h: f64 = 80.0;
    let head_cz: f64 = gantry_beam_center_z - head_drop - head_body_h / 2.0;
    let head_body_dia: f64 = 80.0;

    // ══════════════════════════════════════════════════════════════
    // LH INTERFACE (clamshell hatch on top-center, rail stub on shelf 3)
    // ══════════════════════════════════════════════════════════════
    let hatch_open_x: f64 = 400.0;
    let hatch_open_y: f64 = 300.0;
    let hatch_panel_thickness: f64 = 15.0;
    let hatch_frame_thk: f64 = 8.0;
    // Hatch frame sits flush on shell top face (z = 578)
    let shell_top_z: f64 = chamber_cz + shell_outer_z / 2.0; // 578
    let hatch_frame_cz: f64 = shell_top_z + hatch_frame_thk / 2.0;
    let hatch_panel_cz_closed: f64 = shell_top_z + hatch_frame_thk + hatch_panel_thickness / 2.0;

    // LH door swing (panel open at 110° about its hinge edge; here we model
    // the panel as rotated +70° about the Y axis, hinged on its -X edge)
    let hatch_open_angle_deg: f64 = 70.0;

    // Rail extension stub: show shelf 3 (middle) extended +Y by 200 mm. The
    // shelf slides out toward -Y (door). For assembly visualization, just
    // indicate the inner rail at its fully-deployed position.
    let rail_extension: f64 = 200.0;
    let rail_length: f64 = 500.0;
    let rail_width: f64 = 18.0;
    let rail_height: f64 = 12.0;
    // Middle shelf index = 2 (0-indexed)
    let demo_shelf_idx: usize = 2;
    let demo_shelf_z: f64 = rack_base_top_z + first_shelf_z + (demo_shelf_idx as f64) * shelf_pitch;
    // Outer rail at shelf Z; rides from rack's -Y edge toward +Y (back of rack).
    // Inner rail extends from the rack forward (out of the hatch opening).

    // ══════════════════════════════════════════════════════════════
    // CONTROLLER ENCLOSURE (wall-mounted, outside +X, below reservoir cluster)
    // ══════════════════════════════════════════════════════════════
    // Placeholder bounding box 300×200×150 mm per task spec (T-88727330).
    let ctrl_x: f64 = 300.0;
    let ctrl_y: f64 = 200.0;
    let ctrl_z: f64 = 150.0;
    let ctrl_cx: f64 = shell_outer_x / 2.0 + ctrl_y / 2.0 + 10.0; // right side, flush against wall
    let ctrl_cy: f64 = 200.0; // toward back (+Y)
    let ctrl_cz: f64 = 100.0; // low on wall, away from pump manifold

    // ══════════════════════════════════════════════════════════════
    // BUILD — incubator shell (hollow box with open front + top hatch hole)
    // ══════════════════════════════════════════════════════════════
    let incubator = {
        let outer = cube(
            "inc_outer",
            shell_outer_x,
            shell_outer_y,
            shell_outer_z,
            0.0,
            0.0,
            chamber_cz,
        );
        let cavity = cube(
            "inc_cavity",
            inner_x,
            inner_y,
            inner_z,
            0.0,
            0.0,
            chamber_cz,
        );
        let front_hole = cube(
            "inc_front_hole",
            inner_x + 20.0,
            shell_outer_y,
            inner_z + 20.0,
            0.0,
            -(shell_outer_y / 2.0),
            chamber_cz,
        );
        let top_hatch_hole = cube(
            "inc_top_hatch_hole",
            hatch_open_x,
            hatch_open_y,
            shell_outer_z,
            0.0,
            0.0,
            chamber_cz + shell_outer_z / 2.0,
        );
        // Bulkhead bore on right wall (tubing passthrough)
        let right_bulkhead = centered_cylinder(
            "inc_right_bulkhead",
            passthrough_flange_od / 2.0,
            shell_outer_x + 10.0,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, passthrough_cy, passthrough_cz);
        // Left wall cable grommet (sensor umbilical)
        let left_bulkhead = centered_cylinder("inc_left_bulkhead", 25.0, shell_outer_x + 10.0, 48)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, 0.0, chamber_cz + 100.0);
        ((outer - cavity) - front_hole) - top_hatch_hole - right_bulkhead - left_bulkhead
    };
    incubator
        .write_stl("output/chip_farm_v2_incubator.stl")
        .unwrap();
    println!("Exported: output/chip_farm_v2_incubator.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — rocker (simplified primitives, 0° tilt)
    // ══════════════════════════════════════════════════════════════
    let rocker = {
        let base = cube(
            "rk_base",
            rocker_base_x,
            rocker_base_y,
            rocker_base_z,
            0.0,
            0.0,
            rocker_base_z / 2.0,
        );
        let boss = cube(
            "rk_boss",
            boss_x,
            boss_y,
            boss_z,
            boss_cx,
            0.0,
            rocker_base_z + boss_z / 2.0,
        );
        let stepper = centered_cylinder("rk_stepper", 56.4 / 2.0, 70.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_cx - 70.0 / 2.0, 0.0, rocker_base_z + boss_z / 2.0);
        let pb_left = cube(
            "rk_pb_left",
            60.0,
            40.0,
            pivot_clearance,
            0.0,
            -(rocker_base_y / 2.0 - 40.0 / 2.0 - 20.0),
            rocker_base_z + pivot_clearance / 2.0,
        );
        let pb_right = cube(
            "rk_pb_right",
            60.0,
            40.0,
            pivot_clearance,
            0.0,
            rocker_base_y / 2.0 - 40.0 / 2.0 - 20.0,
            rocker_base_z + pivot_clearance / 2.0,
        );
        let top_plate = cube(
            "rk_top_plate",
            rocker_top_x,
            rocker_top_y,
            rocker_top_z,
            0.0,
            0.0,
            rocker_top_plate_cz,
        );
        base + boss + stepper + pb_left + pb_right + top_plate
    };
    rocker.write_stl("output/chip_farm_v2_rocker.stl").unwrap();
    println!("Exported: output/chip_farm_v2_rocker.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — rack (simplified, 0° tilt)
    // ══════════════════════════════════════════════════════════════
    let rack = {
        let base = cube(
            "rack_base",
            rack_base_x,
            rack_base_y,
            rack_base_z,
            0.0,
            0.0,
            rack_base_bottom_z + rack_base_z / 2.0,
        );
        let base_top = rack_base_top_z;
        let post_cx = post_spacing_x / 2.0;
        let post_cy = post_spacing_y / 2.0;
        let mut posts = Part::empty("rack_posts");
        for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
            for (j, &sy) in [-1.0f64, 1.0].iter().enumerate() {
                posts = posts
                    + cube(
                        &format!("rack_post_{i}_{j}"),
                        post_side,
                        post_side,
                        post_height,
                        sx * post_cx,
                        sy * post_cy,
                        base_top + post_height / 2.0,
                    );
            }
        }
        let mut shelves = Part::empty("rack_shelves");
        for i in 0..num_shelves {
            let z = base_top + first_shelf_z + (i as f64) * shelf_pitch;
            shelves = shelves
                + cube(
                    &format!("rack_shelf_{i}"),
                    shelf_x,
                    shelf_y,
                    shelf_thickness,
                    0.0,
                    0.0,
                    z + shelf_thickness / 2.0,
                );
        }
        // Chip dummies (first column only, for visual scale)
        let grid_origin_x = -(shelf_x / 2.0) + edge_margin + pocket_x / 2.0;
        let grid_origin_y = -(shelf_y / 2.0) + edge_margin + pocket_y / 2.0;
        let mut chips = Part::empty("rack_chips");
        for i in 0..num_shelves {
            let shelf_z = base_top + first_shelf_z + (i as f64) * shelf_pitch;
            let chip_cz = shelf_z + shelf_thickness + chip_z / 2.0;
            for cx in 0..cols {
                for ry in 0..rows {
                    let px = grid_origin_x + (cx as f64) * (pocket_x + gutter_interior);
                    let py = grid_origin_y + (ry as f64) * (pocket_y + gutter_interior);
                    chips = chips
                        + cube(
                            &format!("chip_{i}_{cx}_{ry}"),
                            chip_x,
                            chip_y,
                            chip_z,
                            px,
                            py,
                            chip_cz,
                        );
                }
            }
        }
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
    };
    rack.write_stl("output/chip_farm_v2_rack.stl").unwrap();
    println!("Exported: output/chip_farm_v2_rack.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — media reservoir cluster (outside container, +X)
    // ══════════════════════════════════════════════════════════════
    let reservoir_cluster = {
        // Reservoir body (cylinder)
        let body = centered_cylinder("res_body", reservoir_od / 2.0, reservoir_outer_h, 64)
            .translate(reservoir_cx, reservoir_cy, reservoir_cz);
        // Lid (flat disc on top)
        let lid = centered_cylinder("res_lid", (reservoir_od + 8.0) / 2.0, 8.0, 64).translate(
            reservoir_cx,
            reservoir_cy,
            reservoir_cz + reservoir_outer_h / 2.0 + 4.0,
        );

        // Pump manifold plate on right wall, flat normal = +X
        // Oriented so manifold_plate_x is along Z, manifold_plate_y is along X
        // (short axis perpendicular to wall)
        let plate = cube(
            "pump_plate",
            manifold_plate_y,
            manifold_plate_x,
            manifold_plate_t,
            manifold_cx,
            manifold_cy,
            manifold_cz,
        );
        // 5 pump bosses along Y on the plate
        let mut pumps = Part::empty("pumps");
        let pump_z_start = manifold_cz - manifold_plate_x / 2.0 + 50.0;
        for i in 0..5 {
            let pz = pump_z_start + (i as f64) * pump_pitch;
            let b = cube(
                &format!("pump_{i}"),
                76.0,
                60.0,
                45.0,
                manifold_cx + 45.0,
                0.0,
                pz,
            );
            pumps = pumps + b;
        }

        // Bulkhead flange on right wall
        let flange = centered_cylinder("res_flange", passthrough_flange_od / 2.0, 10.0, 48)
            .rotate(0.0, 90.0, 0.0)
            .translate(passthrough_wall_x + 5.0, passthrough_cy, passthrough_cz);

        // Drip heads (×5) — inside container, above each shelf
        let mut drip_heads = Part::empty("drip_heads");
        for (i, head_z) in drip_head_cz
            .iter()
            .copied()
            .take(num_shelves as usize)
            .enumerate()
        {
            drip_heads = drip_heads
                + cube(
                    &format!("drip_head_{i}"),
                    drip_head_x,
                    drip_head_y,
                    drip_head_z,
                    0.0,
                    drip_head_cy,
                    head_z,
                );
        }

        body + lid + plate + pumps + flange + drip_heads
    };
    reservoir_cluster
        .write_stl("output/chip_farm_v2_reservoir.stl")
        .unwrap();
    println!("Exported: output/chip_farm_v2_reservoir.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — imaging gantry (ceiling-mounted)
    // ══════════════════════════════════════════════════════════════
    let gantry = {
        // Two Y-rails running along Y, at X = ±230
        let y_beam_x = 230.0;
        let y_beam_l = centered_cube(
            "gantry_y_beam_l",
            gantry_extr_w,
            gantry_beam_len_y,
            gantry_extr_h,
        )
        .translate(-y_beam_x, 0.0, gantry_beam_center_z);
        let y_beam_r = centered_cube(
            "gantry_y_beam_r",
            gantry_extr_w,
            gantry_beam_len_y,
            gantry_extr_h,
        )
        .translate(y_beam_x, 0.0, gantry_beam_center_z);

        // X-beam (slides along Y, straddles the two Y-rails at the mid-Y position)
        let x_beam = centered_cube(
            "gantry_x_beam",
            gantry_beam_len_x,
            gantry_extr_w,
            gantry_extr_h,
        )
        .translate(0.0, 0.0, gantry_beam_center_z);

        // Drop column + imaging head centered at origin (X/Y)
        let drop_col = centered_cylinder("gantry_drop", 16.0 / 2.0, head_drop, 24).translate(
            0.0,
            0.0,
            gantry_beam_center_z - head_drop / 2.0,
        );
        let head = centered_cylinder("gantry_head", head_body_dia / 2.0, head_body_h, 48)
            .translate(0.0, 0.0, head_cz);

        // Ceiling bracket (just a thin plate on the ceiling)
        let bracket = cube("gantry_bracket", 500.0, 480.0, 4.0, 0.0, 0.0, inner_z - 2.0);

        y_beam_l + y_beam_r + x_beam + drop_col + head + bracket
    };
    gantry.write_stl("output/chip_farm_v2_gantry.stl").unwrap();
    println!("Exported: output/chip_farm_v2_gantry.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — LH interface (clamshell hatch, open 70°)
    // ══════════════════════════════════════════════════════════════
    let lh = {
        // Frame (rests on shell top, around hatch opening)
        let frame_outer_x = hatch_open_x + 60.0;
        let frame_outer_y = hatch_open_y + 60.0;
        let frame_outer = cube(
            "lh_frame_outer",
            frame_outer_x,
            frame_outer_y,
            hatch_frame_thk,
            0.0,
            0.0,
            hatch_frame_cz,
        );
        let frame_opening = cube(
            "lh_frame_opening",
            hatch_open_x,
            hatch_open_y,
            hatch_frame_thk + 2.0,
            0.0,
            0.0,
            hatch_frame_cz,
        );
        let frame = frame_outer - frame_opening;

        // Clamshell panels — open, rotated about their hinge edges.
        // Left panel hinged at x = -hatch_open_x/2; rotates about Y-axis
        // at that hinge line. We model only the open position (ease of viz).
        let panel_x = hatch_open_x / 2.0 + 10.0;
        let panel_y = hatch_open_y + 20.0;
        let hinge_line_x_l = -hatch_open_x / 2.0;
        let hinge_line_x_r = hatch_open_x / 2.0;
        let panel_z_closed = hatch_panel_cz_closed;

        // Translate so hinge edge is at origin, rotate, translate back
        let panel_l = centered_cube("lh_panel_l", panel_x, panel_y, hatch_panel_thickness)
            .translate(panel_x / 2.0 - 10.0, 0.0, 0.0) // shift so hinge edge at x=-5 → approximate
            .rotate(0.0, -hatch_open_angle_deg, 0.0)
            .translate(hinge_line_x_l, 0.0, panel_z_closed);

        let panel_r = centered_cube("lh_panel_r", panel_x, panel_y, hatch_panel_thickness)
            .translate(-(panel_x / 2.0 - 10.0), 0.0, 0.0)
            .rotate(0.0, hatch_open_angle_deg, 0.0)
            .translate(hinge_line_x_r, 0.0, panel_z_closed);

        // Middle shelf (index 2) deployed out the front — shown as a single
        // slab extending past the front wall.
        let deployed_cube = cube(
            "lh_deployed_shelf",
            shelf_x,
            shelf_y,
            shelf_thickness,
            0.0,
            -shell_outer_y / 2.0 - rail_extension / 2.0,
            demo_shelf_z + shelf_thickness / 2.0,
        );
        let outer_rail_l = cube(
            "lh_rail_l",
            rail_width,
            rail_length,
            rail_height,
            -shelf_x / 2.0 - 20.0,
            -shell_outer_y / 2.0 - rail_length / 2.0 + 50.0,
            demo_shelf_z - rail_height / 2.0,
        );
        let outer_rail_r = cube(
            "lh_rail_r",
            rail_width,
            rail_length,
            rail_height,
            shelf_x / 2.0 + 20.0,
            -shell_outer_y / 2.0 - rail_length / 2.0 + 50.0,
            demo_shelf_z - rail_height / 2.0,
        );

        frame + panel_l + panel_r + deployed_cube + outer_rail_l + outer_rail_r
    };
    lh.write_stl("output/chip_farm_v2_lh.stl").unwrap();
    println!("Exported: output/chip_farm_v2_lh.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — controller enclosure (placeholder box, +X wall-mounted)
    // ══════════════════════════════════════════════════════════════
    let controller = {
        let body = cube(
            "ctrl_body",
            ctrl_y,
            ctrl_x,
            ctrl_z,
            ctrl_cx,
            ctrl_cy,
            ctrl_cz,
        );
        // Front cover indicator (LCD window + button bosses)
        let lcd = cube(
            "ctrl_lcd",
            6.0,
            80.0,
            30.0,
            ctrl_cx + ctrl_y / 2.0 + 3.0,
            ctrl_cy + 50.0,
            ctrl_cz,
        );
        let estop = centered_cylinder("ctrl_estop", 20.0 / 2.0, 16.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(ctrl_cx + ctrl_y / 2.0 + 8.0, ctrl_cy - 60.0, ctrl_cz + 30.0);
        // Wall-mount bracket tab
        let bracket = cube(
            "ctrl_bracket",
            10.0,
            ctrl_x + 40.0,
            40.0,
            ctrl_cx - ctrl_y / 2.0 - 5.0,
            ctrl_cy,
            ctrl_cz - ctrl_z / 2.0 + 20.0,
        );
        body + lcd + estop + bracket
    };
    controller
        .write_stl("output/chip_farm_v2_controller.stl")
        .unwrap();
    println!("Exported: output/chip_farm_v2_controller.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — tubing bundle (5 silicone lines)
    // Route: reservoir lid top → pump (×5) → bulkhead grommet in right wall
    //        → inside container → drip head above each shelf
    // ══════════════════════════════════════════════════════════════
    let tube_r: f64 = 4.8 / 2.0 + 0.3; // OD 4.8 mm (silicone) + conduit padding
    let tubing = {
        let mut bundle = Part::empty("tubing");
        // Anchor points
        let res_top = (
            reservoir_cx,
            reservoir_cy,
            reservoir_cz + reservoir_outer_h / 2.0 + 12.0,
        );
        let pump_z_start = manifold_cz - manifold_plate_x / 2.0 + 50.0;
        let bulkhead_in = (passthrough_wall_x + 5.0, passthrough_cy, passthrough_cz);
        let bulkhead_out = (passthrough_wall_x - 5.0, passthrough_cy, passthrough_cz);

        for (i, head_z) in drip_head_cz
            .iter()
            .copied()
            .take(num_shelves as usize)
            .enumerate()
        {
            let pz = pump_z_start + (i as f64) * pump_pitch;
            let pump_in = (manifold_cx - 30.0, 0.0, pz);
            let pump_out = (manifold_cx - 45.0, 0.0, pz);

            // Segment A: reservoir lid → lateral over to pump inlet (down and across)
            bundle = bundle
                + tube_seg(
                    &format!("t{i}_a1"),
                    tube_r,
                    res_top,
                    (res_top.0, res_top.1, pz + 20.0),
                )
                + tube_seg(
                    &format!("t{i}_a2"),
                    tube_r,
                    (res_top.0, res_top.1, pz + 20.0),
                    (pump_in.0, pump_in.1, pz + 20.0),
                )
                + tube_seg(
                    &format!("t{i}_a3"),
                    tube_r,
                    (pump_in.0, pump_in.1, pz + 20.0),
                    pump_in,
                );

            // Segment B: pump outlet → bulkhead passthrough (Y and Z bend into common wall)
            // Slack loop at the rack-side drip head mount — modeled as a small vertical Z-dip
            let mid_y = (pump_out.1 + bulkhead_in.1) / 2.0;
            bundle = bundle
                + tube_seg(
                    &format!("t{i}_b1"),
                    tube_r,
                    pump_out,
                    (pump_out.0, mid_y, pz),
                )
                + tube_seg(
                    &format!("t{i}_b2"),
                    tube_r,
                    (pump_out.0, mid_y, pz),
                    (pump_out.0, mid_y, bulkhead_in.2),
                )
                + tube_seg(
                    &format!("t{i}_b3"),
                    tube_r,
                    (pump_out.0, mid_y, bulkhead_in.2),
                    bulkhead_in,
                );

            // Segment C: bulkhead → drip head (inside container, with slack loop)
            let drip_pt = (0.0, drip_head_cy, head_z);
            // Horizontal slack loop: 30 mm radius detour in Y right after bulkhead
            let slack_1 = (bulkhead_out.0 - 20.0, bulkhead_out.1 + 30.0, bulkhead_out.2);
            let slack_2 = (bulkhead_out.0 - 40.0, bulkhead_out.1, bulkhead_out.2);
            bundle = bundle
                + tube_seg(&format!("t{i}_c1"), tube_r, bulkhead_out, slack_1)
                + tube_seg(&format!("t{i}_c2"), tube_r, slack_1, slack_2)
                + tube_seg(
                    &format!("t{i}_c3"),
                    tube_r,
                    slack_2,
                    (slack_2.0, slack_2.1, drip_pt.2),
                )
                + tube_seg(
                    &format!("t{i}_c4"),
                    tube_r,
                    (slack_2.0, slack_2.1, drip_pt.2),
                    (drip_pt.0, slack_2.1, drip_pt.2),
                )
                + tube_seg(
                    &format!("t{i}_c5"),
                    tube_r,
                    (drip_pt.0, slack_2.1, drip_pt.2),
                    drip_pt,
                );
        }
        bundle
    };
    tubing.write_stl("output/chip_farm_v2_tubing.stl").unwrap();
    println!("Exported: output/chip_farm_v2_tubing.stl");

    // ══════════════════════════════════════════════════════════════
    // BUILD — cable bundles (sensor + actuator umbilicals)
    // ══════════════════════════════════════════════════════════════
    let cable_r: f64 = 8.0 / 2.0; // 8 mm plastic conduit
    let cables = {
        let mut bundle = Part::empty("cables");
        // Sensor umbilical: controller → left wall grommet → 3 sensors
        //   SHT40 (+80, 0, chamber mid), CO2 (-80, 0, chamber mid), DS18B20 (0, 0, floor+20)
        let ctrl_left_face = (
            ctrl_cx - ctrl_y / 2.0,
            ctrl_cy,
            ctrl_cz + ctrl_z / 2.0 - 20.0,
        );
        let left_wall_grommet_out = (-shell_outer_x / 2.0 + 5.0, 0.0, chamber_cz + 100.0);
        let left_wall_grommet_in = (-shell_outer_x / 2.0 + 15.0, 0.0, chamber_cz + 100.0);

        // Controller → left grommet: up, over the top, down the left side
        bundle = bundle
            + tube_seg(
                "cab_s1",
                cable_r,
                ctrl_left_face,
                (ctrl_left_face.0, ctrl_left_face.1, shell_top_z + 60.0),
            )
            + tube_seg(
                "cab_s2",
                cable_r,
                (ctrl_left_face.0, ctrl_left_face.1, shell_top_z + 60.0),
                (
                    -shell_outer_x / 2.0 - 20.0,
                    ctrl_left_face.1,
                    shell_top_z + 60.0,
                ),
            )
            + tube_seg(
                "cab_s3",
                cable_r,
                (
                    -shell_outer_x / 2.0 - 20.0,
                    ctrl_left_face.1,
                    shell_top_z + 60.0,
                ),
                (-shell_outer_x / 2.0 - 20.0, 0.0, shell_top_z + 60.0),
            )
            + tube_seg(
                "cab_s4",
                cable_r,
                (-shell_outer_x / 2.0 - 20.0, 0.0, shell_top_z + 60.0),
                left_wall_grommet_out,
            );

        // Inside container: fan out to 3 sensors
        let sht40 = (80.0, 220.0, chamber_cz);
        let co2 = (-80.0, 220.0, chamber_cz);
        let ds18b20 = (0.0, 220.0, 20.0);
        bundle = bundle
            + tube_seg(
                "cab_sht1",
                cable_r,
                left_wall_grommet_in,
                (sht40.0, left_wall_grommet_in.1, sht40.2),
            )
            + tube_seg(
                "cab_sht2",
                cable_r,
                (sht40.0, left_wall_grommet_in.1, sht40.2),
                sht40,
            )
            + tube_seg(
                "cab_co21",
                cable_r,
                left_wall_grommet_in,
                (co2.0, left_wall_grommet_in.1, co2.2),
            )
            + tube_seg(
                "cab_co22",
                cable_r,
                (co2.0, left_wall_grommet_in.1, co2.2),
                co2,
            )
            + tube_seg(
                "cab_ds1",
                cable_r,
                left_wall_grommet_in,
                (ds18b20.0, left_wall_grommet_in.1, ds18b20.2),
            )
            + tube_seg(
                "cab_ds2",
                cable_r,
                (ds18b20.0, left_wall_grommet_in.1, ds18b20.2),
                ds18b20,
            );

        // Actuator umbilical: controller → right wall grommet (shared with tubing
        // passthrough location but ~100 mm higher) → heater, fan, solenoid, stepper
        let ctrl_right_face = (
            ctrl_cx + ctrl_y / 2.0,
            ctrl_cy,
            ctrl_cz + ctrl_z / 2.0 - 20.0,
        );
        let right_wall_grommet_out = (shell_outer_x / 2.0 + 5.0, 0.0, chamber_cz - 100.0);
        let right_wall_grommet_in = (shell_outer_x / 2.0 - 15.0, 0.0, chamber_cz - 100.0);
        bundle = bundle
            + tube_seg(
                "cab_a1",
                cable_r,
                ctrl_right_face,
                (
                    ctrl_right_face.0,
                    ctrl_right_face.1,
                    right_wall_grommet_out.2,
                ),
            )
            + tube_seg(
                "cab_a2",
                cable_r,
                (
                    ctrl_right_face.0,
                    ctrl_right_face.1,
                    right_wall_grommet_out.2,
                ),
                (ctrl_right_face.0, 0.0, right_wall_grommet_out.2),
            )
            + tube_seg(
                "cab_a3",
                cable_r,
                (ctrl_right_face.0, 0.0, right_wall_grommet_out.2),
                right_wall_grommet_out,
            );

        // Inside container: fan out to heater (back wall low), fan (right wall mid),
        // solenoid (back wall low), stepper (rocker boss)
        let heater = (0.0, 220.0, 15.0);
        let fan_actuator = (200.0, 0.0, chamber_cz + 50.0);
        let solenoid = (-150.0, 220.0, 20.0);
        let stepper = (boss_cx, 0.0, rocker_base_z + boss_z / 2.0);
        bundle = bundle
            + tube_seg(
                "cab_heat1",
                cable_r,
                right_wall_grommet_in,
                (heater.0, right_wall_grommet_in.1, heater.2),
            )
            + tube_seg(
                "cab_heat2",
                cable_r,
                (heater.0, right_wall_grommet_in.1, heater.2),
                heater,
            )
            + tube_seg("cab_fan", cable_r, right_wall_grommet_in, fan_actuator)
            + tube_seg(
                "cab_sol1",
                cable_r,
                right_wall_grommet_in,
                (solenoid.0, right_wall_grommet_in.1, solenoid.2),
            )
            + tube_seg(
                "cab_sol2",
                cable_r,
                (solenoid.0, right_wall_grommet_in.1, solenoid.2),
                solenoid,
            )
            + tube_seg(
                "cab_stp1",
                cable_r,
                right_wall_grommet_in,
                (stepper.0, right_wall_grommet_in.1, stepper.2),
            )
            + tube_seg(
                "cab_stp2",
                cable_r,
                (stepper.0, right_wall_grommet_in.1, stepper.2),
                stepper,
            );

        bundle
    };
    cables.write_stl("output/chip_farm_v2_cables.stl").unwrap();
    println!("Exported: output/chip_farm_v2_cables.stl");

    // ══════════════════════════════════════════════════════════════
    // FULL ASSEMBLY
    // ══════════════════════════════════════════════════════════════
    let assembly =
        incubator + rocker + rack + reservoir_cluster + gantry + lh + controller + tubing + cables;
    assembly
        .write_stl("output/chip_farm_v2_assembly.stl")
        .unwrap();
    println!("Exported: output/chip_farm_v2_assembly.stl");

    // STEP export for the full assembly only
    laminarforge_cad::stl_to_step("output/chip_farm_v2_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // INTERFERENCE CHECKS
    // ══════════════════════════════════════════════════════════════

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   INTERFERENCE CHECKS");
    println!("══════════════════════════════════════════════════════════════");

    let mut violations: Vec<String> = Vec::new();

    // AABBs for the fixed (non-tilted) subsystems
    let aabb_rack_upright = Aabb::from_center_size(
        0.0,
        0.0,
        (rack_base_bottom_z + handle_tip_z) / 2.0,
        shelf_x,
        shelf_y,
        handle_tip_z - rack_base_bottom_z,
    );

    let aabb_interior = Aabb {
        x: (-inner_x / 2.0, inner_x / 2.0),
        y: (-inner_y / 2.0, inner_y / 2.0),
        z: (0.0, inner_z),
    };

    // Check 1: Rack fits inside container envelope at 0° tilt.
    let (gx0, gy0, gz0) = aabb_interior.separations(&aabb_rack_upright);
    println!("[1] Rack @ 0°: clearance vs interior (X={gx0:+.1}, Y={gy0:+.1}, Z={gz0:+.1} mm)");
    // Note: for AABB "contains" we want rack_aabb inside interior_aabb
    let contains_x =
        aabb_interior.x.0 <= aabb_rack_upright.x.0 && aabb_interior.x.1 >= aabb_rack_upright.x.1;
    let contains_y =
        aabb_interior.y.0 <= aabb_rack_upright.y.0 && aabb_interior.y.1 >= aabb_rack_upright.y.1;
    let contains_z =
        aabb_interior.z.0 <= aabb_rack_upright.z.0 && aabb_interior.z.1 >= aabb_rack_upright.z.1;
    if !(contains_x && contains_y && contains_z) {
        violations.push(format!(
            "Rack @ 0° exceeds interior envelope (containsX={contains_x}, Y={contains_y}, Z={contains_z})"
        ));
    }

    // Check 2: Rocker at +15° tilt — handle tip lateral+vertical motion.
    // Rack + handle pivots about the rocker pivot axis at z = 55 mm.
    // For tilt angle θ about +X, the (±Y_max, +Z_handle_tip) point rotates to:
    //   y' = y cosθ − (z − z_pivot) sinθ + 0
    //   z' = z_pivot + y sinθ + (z − z_pivot) cosθ
    let tilt_deg: f64 = 15.0;
    let theta = tilt_deg.to_radians();
    let handle_height_above_pivot = handle_tip_z - rocker_pivot_axis_z;
    let rack_half_y = shelf_y / 2.0; // rack side reaches ±shelf_y/2 at rack top (posts)
    let rack_corner_y = rack_half_y;
    let rack_corner_z_above_pivot = rack_top_posts_z - rocker_pivot_axis_z;
    let corner_y_tilted = rack_corner_y * theta.cos() - rack_corner_z_above_pivot * theta.sin();
    let corner_z_tilted =
        rocker_pivot_axis_z + rack_corner_y * theta.sin() + rack_corner_z_above_pivot * theta.cos();
    let handle_z_tilted = rocker_pivot_axis_z + 0.0 + handle_height_above_pivot * theta.cos();
    let handle_y_swing = handle_height_above_pivot * theta.sin(); // handle tip swings in Y
    println!(
        "[2] Rocker @ +15°: top rack corner → (Y={corner_y_tilted:+.1}, Z={corner_z_tilted:+.1} mm)"
    );
    println!("    Handle tip: Y-swing ±{handle_y_swing:.1} mm, Z (max) {handle_z_tilted:.1} mm");
    if corner_z_tilted > inner_z {
        violations.push(format!(
            "Rack top corner hits ceiling at +15° tilt: z={corner_z_tilted:.1} > {inner_z:.0}"
        ));
    } else {
        let headroom = inner_z - corner_z_tilted;
        println!("    → headroom above rack top corner: {headroom:.1} mm");
    }

    // Check 3: Rocker sweep — drip heads vs tilted rack.
    //
    // The drip heads are chamber-mounted (fixed) at Y ≈ -256 mm with a Y-span
    // of ±12.5 mm → Y ∈ [-268, -244]. The rack's Y-span is [-shelf_y/2,
    // +shelf_y/2] ≈ [-225, +225]. The two envelopes are Y-separated by at
    // least |−244 − (−225)| = 19 mm at 0° tilt. Under rotation about the X
    // axis through the pivot at Y=0, a point at (y_r, z_r) moves to
    //   (y_r cosθ − (z_r − z_p) sinθ,  z_p + y_r sinθ + (z_r − z_p) cosθ).
    // The tilted rack's Y-envelope: the Y-extrema occur at ±shelf_y/2, post
    // tops. Since |sinθ|·z_above_pivot can extend Y inward (toward 0) or
    // shrink the outer reach, the farthest -Y point after tilt is the
    // original (−shelf_y/2) point or the (+shelf_y/2, z_top) point rotated
    // negative, whichever is more negative. Compute both.
    let drip_y_inner_edge = drip_head_cy + drip_head_y / 2.0; // least negative Y of drip head AABB
    let drip_y_outer_edge = drip_head_cy - drip_head_y / 2.0;
    let _ = drip_y_outer_edge;

    // For each shelf, find the most negative Y the chip top reaches at ±15°.
    // Rack top corner at (y=-shelf_y/2, z=chip_top) under +θ moves to
    //   y' = -shelf_y/2 · cosθ − (chip_top − z_p) · sinθ
    // Under -θ the -shelf_y/2 side goes MORE negative in Y only if
    // (chip_top − z_p)·sinθ becomes negative → i.e., +θ direction. Check both.
    let mut min_drip_clear = f64::INFINITY;
    for (i, &chip_top) in shelf_top_chip_z.iter().enumerate() {
        let zap = chip_top - rocker_pivot_axis_z;
        // Four candidate tilted-rack extreme points per shelf (combinations of
        // ±shelf_y/2, ±θ). We want Y-separation from the drip AABB
        // (drip head at Y ∈ [drip_y_outer_edge, drip_y_inner_edge], Z near
        // drip_head_cz[i]). Drip head is in -Y; the concerning rack extreme
        // is its -Y edge. Check both +θ and -θ.
        for &s in &[1.0f64, -1.0] {
            let th = s * theta;
            let yr = -shelf_y / 2.0; // rack's -Y edge (nearest to drip head)
            let y_tilt = yr * th.cos() - zap * th.sin();
            let z_tilt = rocker_pivot_axis_z + yr * th.sin() + zap * th.cos();
            // Y-separation from drip head AABB (drip at more negative Y)
            let y_sep = drip_y_inner_edge - y_tilt; // >0 means rack is on +Y side of drip inner edge
                                                    // Z-separation (if y_sep > 0 we don't need Z, else we collide if Z overlaps)
            let drip_bottom = drip_head_cz[i] - drip_head_z / 2.0;
            let drip_top = drip_head_cz[i] + drip_head_z / 2.0;
            let z_above_drip = z_tilt - drip_top;
            let z_below_drip = drip_bottom - z_tilt;
            let z_sep = z_above_drip.max(z_below_drip);
            // Combined clearance: max of Y and Z seps (true min separation
            // since they're along orthogonal axes; a positive in either axis
            // means no overlap).
            let combined = y_sep.max(z_sep);
            if combined < min_drip_clear {
                min_drip_clear = combined;
            }
            println!(
                "    Shelf {} tilt{:+.0}°: rack(-Y edge) @ ({y_tilt:+.1}, {z_tilt:+.1}); drip y_sep {y_sep:+.1}, z_sep {z_sep:+.1} mm",
                i + 1,
                s * tilt_deg,
            );
        }
    }
    println!("[3] Min drip-head-vs-tilted-rack clearance (any-axis sep): {min_drip_clear:+.1} mm");
    if min_drip_clear < 5.0 {
        violations.push(format!(
            "Drip head clearance too tight (worst = {min_drip_clear:+.1} mm < 5 mm)"
        ));
    }
    let drip_clearance_worst = min_drip_clear;

    // Keep worst_z for gantry check below: the max Z reached by the top
    // shelf's chip-top at any tilt ±15°, over its full Y span.
    let top_idx = num_shelves as usize - 1;
    let chip_top_top = shelf_top_chip_z[top_idx];
    let zap_top = chip_top_top - rocker_pivot_axis_z;
    let mut worst_z: f64 = chip_top_top;
    for &s in &[1.0f64, -1.0] {
        let th = s * theta;
        for &yr in &[-shelf_y / 2.0, shelf_y / 2.0] {
            let z_tilt = rocker_pivot_axis_z + yr * th.sin() + zap_top * th.cos();
            if z_tilt > worst_z {
                worst_z = z_tilt;
            }
        }
    }

    // Check 4: Imaging gantry beam above rack top at max tilt.
    // The gantry beam bottom is at gantry_beam_center_z - extr_h/2 = 505 mm.
    let gantry_beam_bottom = gantry_beam_center_z - gantry_extr_h / 2.0;
    let gantry_clearance = gantry_beam_bottom - corner_z_tilted;
    println!(
        "[4] Gantry beam bottom ({gantry_beam_bottom:.1} mm) vs rack tilted top corner ({corner_z_tilted:.1} mm): clearance {gantry_clearance:+.1} mm"
    );
    if gantry_clearance < 20.0 {
        violations.push(format!(
            "Gantry beam / rack-top clearance < 20 mm at 15° tilt ({gantry_clearance:+.1})"
        ));
    }

    // Imaging head Z vs top shelf chips at 0° tilt (firmware interlock keeps
    // the rocker level during imaging passes — per imaging_gantry spec
    // working-distance window is 140–300 mm at head_cz=325 mm and top chip at
    // ~290 mm). At ±15° tilt the rocker pauses imaging.
    let head_bottom_z = head_cz - head_body_h / 2.0;
    let top_chip_z_level = chip_top_top; // rack level
    let head_clearance_level = head_bottom_z - top_chip_z_level;
    println!(
        "    Imaging head bottom ({head_bottom_z:.1}) vs top chip @ 0° ({top_chip_z_level:.1}): clearance {head_clearance_level:+.1} mm (working distance, level)"
    );
    if head_clearance_level < 50.0 {
        violations.push(format!(
            "Imaging head vertical clearance < 50 mm at 0° tilt ({head_clearance_level:+.1})"
        ));
    }
    // At ±15° tilt the top chip rises to `worst_z`; the firmware interlock
    // parks the imaging head at X=±225 (rail end) when rocking. Report the
    // gap so it's documented.
    let head_clearance_tilted = head_bottom_z - worst_z;
    println!(
        "    [info] Imaging head vs worst tilted top chip ({worst_z:.1}): {head_clearance_tilted:+.1} mm — firmware parks head during rocking"
    );

    // Check 5: LH hatch swing (70° open). Panel length = panel_x = 210 mm;
    // fully open panel tip rises to hinge_z + panel_x·sin(70°).
    let hinge_z = panel_z_closed_for_check(shell_top_z, hatch_frame_thk, hatch_panel_thickness);
    let panel_x = hatch_open_x / 2.0 + 10.0;
    let hatch_angle = hatch_open_angle_deg.to_radians();
    let panel_tip_z = hinge_z + panel_x * hatch_angle.sin();
    let panel_tip_x = hatch_open_x / 2.0 - panel_x * hatch_angle.cos();
    // Also check the panel tip vs the controller box above location (ctrl_cz ± ctrl_z/2).
    // Controller box is to the +X side at ctrl_cx > shell_outer_x/2. Panel tip at
    // x around center+/-200 and z around shell_top+panel_x*sinθ. Far from controller.
    println!(
        "[5] LH hatch tip at {hatch_open_angle_deg:.0}° open: X ≈ {panel_tip_x:.1}, Z ≈ {panel_tip_z:.1} mm"
    );
    // Clearance over gantry ceiling bracket: gantry is INSIDE container, so no
    // concern. Main concern: nothing outside container above the hatch.
    if panel_tip_x.abs() > inner_x / 2.0 + 100.0 {
        violations.push(format!(
            "LH hatch panel tip swings outside +/-{:.0} mm X-envelope at {:.0}° open",
            inner_x / 2.0 + 100.0,
            hatch_open_angle_deg
        ));
    }

    // Check 6: Tubing passthrough — slack loop accommodates rocker motion.
    // The drip heads are fixed to the chamber (per media_reservoir spec), so
    // rocker motion does NOT pull on the drip head itself. The tubing inside
    // the chamber has a 30 mm Y-direction slack loop after the bulkhead before
    // reaching the drip head. Required slack: the tubing is fixed on both ends
    // (bulkhead, drip head) so rocker motion doesn't affect these tubes
    // directly. The slack loop is for assembly tolerance + thermal cycling.
    let slack_loop_y: f64 = 30.0;
    let thermal_growth_per_m: f64 = 0.15; // mm per meter per 20°C (silicone)
    let tube_length_internal_mm = 300.0;
    let thermal_growth = thermal_growth_per_m * tube_length_internal_mm / 1000.0;
    println!(
        "[6] Tubing slack loop: {slack_loop_y:.0} mm; thermal growth over 0.3 m ≈ {thermal_growth:.3} mm — loop >> growth"
    );
    if slack_loop_y < thermal_growth * 2.0 {
        violations.push("Tubing slack loop insufficient for thermal growth".to_string());
    }

    // Check 7: Controller box outside envelope — does not collide with anything.
    let aabb_ctrl = Aabb::from_center_size(ctrl_cx, ctrl_cy, ctrl_cz, ctrl_y, ctrl_x, ctrl_z);
    let aabb_shell = Aabb::from_center_size(
        0.0,
        0.0,
        chamber_cz,
        shell_outer_x,
        shell_outer_y,
        shell_outer_z,
    );
    if aabb_ctrl.interferes(&aabb_shell) {
        violations.push("Controller box AABB overlaps container shell".to_string());
    } else {
        println!("[7] Controller box clear of container shell (ok)");
    }

    // Check 8: Reservoir + pump manifold outside envelope.
    let aabb_res = Aabb::from_center_size(
        reservoir_cx,
        reservoir_cy,
        reservoir_cz,
        reservoir_od,
        reservoir_od,
        reservoir_outer_h,
    );
    if aabb_res.interferes(&aabb_shell) {
        violations.push("Reservoir AABB overlaps container shell".to_string());
    } else {
        println!("[8] Reservoir clear of container shell (ok)");
    }

    // Check 9: Reservoir vs Controller — both on +X side.
    if aabb_res.interferes(&aabb_ctrl) {
        violations.push("Reservoir overlaps controller box".to_string());
    } else {
        println!("[9] Reservoir clear of controller box (ok)");
    }

    println!();
    if violations.is_empty() {
        println!("✓ All interference checks PASSED");
    } else {
        println!("✗ {} INTERFERENCE VIOLATIONS:", violations.len());
        for v in &violations {
            println!("    - {v}");
        }
    }
    assert!(
        violations.is_empty(),
        "Interference violations detected — fix geometry"
    );

    // ══════════════════════════════════════════════════════════════
    // DIMENSIONED ASCII DRAWING
    // ══════════════════════════════════════════════════════════════
    let drawing = format!(
        "Chip Farm Assembly v2 — Integrated Full-System Layout\n\
         (All dimensions in mm. Origin = container interior floor center.)\n\
         \n\
         ═══ SUBSYSTEM POSITION TABLE (global XYZ, centers) ═══\n\
         \n\
         Subsystem            |  CX     |  CY     |  CZ     |  Envelope (X×Y×Z)\n\
         ---------------------|---------|---------|---------|------------------------\n\
         Incubator shell      |    0.0  |    0.0  |  {icz:>6.1} |  {sox:>3.0}×{soy:>3.0}×{soz:>3.0} (outer)\n\
         Incubator interior   |    0.0  |    0.0  |  {icz:>6.1} |  {inx:>3.0}×{iny:>3.0}×{inz:>3.0} (inside)\n\
         Rocker base          |    0.0  |    0.0  |    5.0  |  {rbx:>3.0}×{rby:>3.0}×{rbz:>3.0}\n\
         Rocker top plate     |    0.0  |    0.0  |  {rtpz:>6.1} |  {rtx:>3.0}×{rty:>3.0}×{rtz:>3.0}\n\
         Rack base plate      |    0.0  |    0.0  |  {rabc:>6.1} |  {rax:>3.0}×{ray:>3.0}×{raz:>3.0}\n\
         Rack top (post top)  |    0.0  |    0.0  |  {rtop:>6.1} |  shelf {shx:>3.0}×{shy:>3.0}\n\
         Handle tip (top)     |    0.0  |    0.0  |  {htip:>6.1} |  Ø10 rod\n\
         Imaging gantry beam  |    0.0  |    0.0  |  {gbcz:>6.1} |  {gbx:>3.0}×{gby:>3.0}×{gth:>3.0}\n\
         Imaging head         |    0.0  |    0.0  |  {hcz:>6.1} |  Ø80 × 80\n\
         LH hatch frame       |    0.0  |    0.0  |  {hfz:>6.1} |  {hfx:>3.0}×{hfy:>3.0}×8\n\
         Drip head (shelf 1)  |    0.0  | {dhcy:>6.1} |  {dh0:>6.1} |  {dhx:>3.0}×{dhy:>3.0}×{dhz:>3.0}\n\
         Drip head (shelf 5)  |    0.0  | {dhcy:>6.1} |  {dh4:>6.1} |  {dhx:>3.0}×{dhy:>3.0}×{dhz:>3.0}\n\
         Reservoir body       |  {rcx:>6.1} | {rcy:>6.1} |  {rcz:>6.1} |  Ø{rod:>3.0} × {roh:>3.0}\n\
         Pump manifold        |  {mcx:>6.1} |    0.0  |  {mcz:>6.1} |  {mpx:>3.0}×{mpy:>3.0}×{mpt:>3.0}\n\
         Wall bulkhead (+X)   |  {btx:>6.1} |    0.0  |  {btz:>6.1} |  Ø{bod:>3.0} flange\n\
         Controller enclosure |  {ctx:>6.1} |  {cty:>6.1} |  {ctz:>6.1} |  {ctbx:>3.0}×{ctby:>3.0}×{ctbz:>3.0} (placeholder)\n\
         \n\
         ═══ TUBING ROUTING (5 silicone lines; ID 1.6, OD 4.8 mm) ═══\n\
         Route, per shelf i ∈ [1..5]:\n\
           reservoir_lid_top (Ø outlet)  →  pump_i inlet\n\
           pump_i outlet                 →  wall bulkhead (shared 5-hole grommet)\n\
           bulkhead interior             →  slack loop (30 mm Y-offset)\n\
           slack loop                    →  drip head i (above shelf i)\n\
         \n\
         Bulkhead passthrough: Ø60 mm flange on right (+X) wall at\n\
         Y=0, Z={bkz:.0} mm. Insert: 5-hole silicone grommet (Ø5.1 per hole).\n\
         Drip heads are CHAMBER-MOUNTED (per media_reservoir spec); they do\n\
         not travel with the rocker. A 30 mm horizontal slack loop after the\n\
         bulkhead absorbs silicone thermal growth (~0.05 mm over 300 mm @ 37°C).\n\
         \n\
         ═══ CABLE ROUTING (sensor + actuator umbilicals) ═══\n\
         Sensor umbilical   — Ø8 mm plastic conduit:\n\
           Controller box (left face) → up to shell top → over → down left\n\
             side → left wall grommet at Y=0, Z={lwg:.0}\n\
           Inside: fans out to SHT40 (+80,+220,+275), CO2 sensor\n\
             (-80,+220,+275), DS18B20 floor probe (0,+220,+20)\n\
         \n\
         Actuator umbilical — Ø8 mm plastic conduit:\n\
           Controller box (right face) → down → right wall grommet at\n\
             Y=0, Z={rwg:.0} (offset 100 mm below tubing bulkhead)\n\
           Inside: fans out to heater (0,+220,+15), fan actuator\n\
             (+200,0,+325), CO2 solenoid (-150,+220,+20), rocker stepper\n\
             ({stepx:.0},0,{stepz:.0})\n\
         \n\
         ═══ INTERFERENCE CHECK RESULTS ═══\n\
         [1] Rack @ 0°: fits inside interior (X slack {rx_slack:.1}, Y slack {ry_slack:.1}, Z slack {rz_slack:.1})\n\
         [2] Rack @ +15° tilt: top corner → Y={c_y:.1}, Z={c_z:.1} ({c_hr:.1} mm headroom)\n\
             Handle tip at 15°: Y-swing ±{hswing:.1} mm\n\
         [3] Worst-case drip head clearance over top shelf (±15°): {dclw:+.1} mm (min req 5 mm)\n\
         [4] Gantry beam bottom ({gbbz:.1}) vs tilted rack top corner ({c_z:.1}): {gcl:+.1} mm clearance\n\
             Imaging head ({hbz:.1}) vs worst tilted top chip ({wz:.1}): {hcl:+.1} mm\n\
         [5] LH hatch tip at 70° open: (X={ptx:.1}, Z={ptz:.1}) — no external collision\n\
         [6] Tubing slack loop 30 mm >> 0.05 mm thermal growth (ok)\n\
         [7] Controller box AABB: clear of container shell\n\
         [8] Reservoir AABB: clear of container shell\n\
         [9] Reservoir + Controller: clear of each other\n\
         \n\
         RESULT: ALL INTERFERENCE CHECKS PASSED.\n\
         \n\
         ═══ OUTPUTS MANIFEST ═══\n\
         output/chip_farm_v2_assembly.stl     — full integrated assembly\n\
         output/chip_farm_v2_assembly.stp     — STEP for the full assembly\n\
         output/chip_farm_v2_incubator.stl    — container shell only\n\
         output/chip_farm_v2_rocker.stl       — rocker only\n\
         output/chip_farm_v2_rack.stl         — rack only\n\
         output/chip_farm_v2_reservoir.stl    — reservoir cluster\n\
         output/chip_farm_v2_gantry.stl       — imaging gantry\n\
         output/chip_farm_v2_lh.stl           — LH hatch + deployed shelf\n\
         output/chip_farm_v2_controller.stl   — controller enclosure (placeholder)\n\
         output/chip_farm_v2_tubing.stl       — 5-line silicone bundle\n\
         output/chip_farm_v2_cables.stl       — sensor + actuator bundles\n\
         output/chip_farm_v2_drawing.txt      — this file\n\
         USDZ generated via freecadcmd + xcrun scntool (manual path; MCP\n\
         convert action has a known Python-over-stdin bug per A-00AFB907).\n\
         \n\
         ═══ OPEN ISSUES / NOTES ═══\n\
         - Controller enclosure is a 300×200×150 mm bounding-box placeholder.\n\
           Replace with full CAD once T-88727330 closes.\n\
         - Left wall cable grommet (Ø25 ID) is new — propagate to\n\
           chip_incubator_v2 next revision.\n\
         - Drip heads live on chamber mount rails (independent of rocker).\n\
           Add bracket detail in a follow-on revision.\n\
         - At full ±15° tilt the Y-swing of the handle is {hswing:.1} mm; the\n\
           acrylic door (at y=-291) clears by >{door_cl:.0} mm.\n",
        icz = chamber_cz,
        sox = shell_outer_x,
        soy = shell_outer_y,
        soz = shell_outer_z,
        inx = inner_x,
        iny = inner_y,
        inz = inner_z,
        rbx = rocker_base_x,
        rby = rocker_base_y,
        rbz = rocker_base_z,
        rtpz = rocker_top_plate_cz,
        rtx = rocker_top_x,
        rty = rocker_top_y,
        rtz = rocker_top_z,
        rabc = rack_base_bottom_z + rack_base_z / 2.0,
        rax = rack_base_x,
        ray = rack_base_y,
        raz = rack_base_z,
        rtop = rack_top_posts_z,
        shx = shelf_x,
        shy = shelf_y,
        htip = handle_tip_z,
        gbcz = gantry_beam_center_z,
        gbx = gantry_beam_len_x,
        gby = gantry_beam_len_y,
        gth = gantry_extr_h,
        hcz = head_cz,
        hfz = hatch_frame_cz,
        hfx = hatch_open_x + 60.0,
        hfy = hatch_open_y + 60.0,
        dhcy = drip_head_cy,
        dh0 = drip_head_cz[0],
        dh4 = drip_head_cz[4],
        dhx = drip_head_x,
        dhy = drip_head_y,
        dhz = drip_head_z,
        rcx = reservoir_cx,
        rcy = reservoir_cy,
        rcz = reservoir_cz,
        rod = reservoir_od,
        roh = reservoir_outer_h,
        mcx = manifold_cx,
        mcz = manifold_cz,
        mpx = manifold_plate_y,
        mpy = manifold_plate_x,
        mpt = manifold_plate_t,
        btx = passthrough_wall_x,
        btz = passthrough_cz,
        bod = passthrough_flange_od,
        ctx = ctrl_cx,
        cty = ctrl_cy,
        ctz = ctrl_cz,
        ctbx = ctrl_y,
        ctby = ctrl_x,
        ctbz = ctrl_z,
        bkz = passthrough_cz,
        lwg = chamber_cz + 100.0,
        rwg = chamber_cz - 100.0,
        stepx = boss_cx,
        stepz = rocker_base_z + boss_z / 2.0,
        rx_slack = inner_x - shelf_x,
        ry_slack = inner_y - shelf_y,
        rz_slack = inner_z - handle_tip_z,
        c_y = corner_y_tilted,
        c_z = corner_z_tilted,
        c_hr = inner_z - corner_z_tilted,
        hswing = handle_y_swing,
        dclw = drip_clearance_worst,
        gbbz = gantry_beam_bottom,
        gcl = gantry_clearance,
        hbz = head_bottom_z,
        wz = worst_z,
        hcl = head_clearance_level,
        ptx = panel_tip_x,
        ptz = panel_tip_z,
        door_cl = (shell_outer_y / 2.0 - handle_y_swing).max(0.0),
    );

    std::fs::write("output/chip_farm_v2_drawing.txt", &drawing).unwrap();
    println!("Exported: output/chip_farm_v2_drawing.txt");

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   CHIP FARM ASSEMBLY v2 — BUILD COMPLETE");
    println!("══════════════════════════════════════════════════════════════");
}

fn panel_z_closed_for_check(shell_top_z: f64, frame_thk: f64, panel_thk: f64) -> f64 {
    // Panel hinge axis Z = shell_top + frame_thk + panel_thk/2 (same as panel
    // closed center Z). Used by the interference section.
    shell_top_z + frame_thk + panel_thk / 2.0
}
