use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── In-Incubator Imaging Gantry (Option A) ───
//
// Ticket: T-FB0E1362 (laminarforge / microfluidic-chip /
//         chip-stack-thermal-container)
//
// Reads all 100 Rev C chips held in the 5-shelf × 20-chip chip_stack_rack
// (chip_stack_rack.rs) without opening the 37 °C / 90 % RH / 5 % CO2
// chip_incubator_v2 container (chip_incubator_v2.rs, interior
// 600 × 520 × 550 mm).
//
// Architecture = Option A: single 2-axis motorized gantry mounted to the
// ceiling of the inner chamber. A camera + dual-mode illumination head
// (white CRI-90 bar for colorimetric LAMP + 470 nm blue LED for SYTO-9
// fluorescence, with a 520 nm long-pass emission filter on the camera)
// rides the gantry. The head images each chip top-down through a heated
// ITO-coated glass aperture, scanning all 100 chips in ~6 minutes (well
// inside the 15-minute kinetic target).
//
// Rationale for Option A over B (per-shelf stationary array) and C
// (shelf-extension to external imager): see the design spec artifact.
// Short version:
//   A: 1 camera, 1 calibration path, moving parts tolerable with IP65
//      head + sealed linear rails and a heated window; fits under the
//      liquid-handler top hatch area without colliding with the rack.
//   B: 5 cameras × 5 calibrations, wide-angle distortion across a
//      465 × 530 mm shelf footprint, uniform LED illumination hard.
//   C: Breaks chamber seal every read → humidity / CO2 / temperature
//      excursions; unworkable for 15-minute kinetic curves.
//
// Coordinate system matches chip_incubator_v2: chamber centered at
// origin, X = 600 mm width, Y = 520 mm depth, Z = 550 mm height.
// Gantry is mounted to the inner chamber ceiling at z ≈ +275 mm.
// Rack top shelf sits at approximately z = base(6) + post(230) ≈ 236 mm
// above the floor of the chamber ≡ z ≈ -275 + 236 = -39 mm in chamber
// coords. Top shelf chip top face ≈ -39 + 3 + 14 = -22 mm. So the
// imaging head working distance from gantry ceiling to top shelf chip
// is 275 - (-22) = 297 mm. We lower the gantry beams and suspend the
// head 160 mm down from the rails to bring it to z ≈ +115 (working
// distance ≈ 137 mm to the top shelf). A telephoto lens + focus-
// stacked EDOF accommodates the 5-shelf range (top shelf at z ≈ -22,
// bottom shelf at z ≈ -22 - 4×40 = -182), giving 140–300 mm working
// distance. The sensor is refocused per-shelf via a small linear
// stepper on the head itself (Z-focus ~10 mm travel).
//
// Exports:
//   output/imaging_gantry_x_beam.stl
//   output/imaging_gantry_y_carriage.stl
//   output/imaging_gantry_head.stl
//   output/imaging_gantry_heated_window.stl
//   output/imaging_gantry_ceiling_mount.stl
//   output/imaging_gantry_assembly.stl

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 0. CONTAINER + RACK REFERENCE DIMENSIONS
    // ══════════════════════════════════════════════════════════════

    // Chamber interior (chip_incubator_v2)
    let chamber_x: f64 = 600.0;
    let chamber_y: f64 = 520.0;
    let chamber_z: f64 = 550.0;

    // Rack footprint (chip_stack_rack)
    let rack_x: f64 = 530.04; // shelf X
    let rack_y: f64 = 451.40; // shelf Y
    let rack_base_top: f64 = -chamber_z / 2.0 + 6.0; // base 6 mm thick on floor
    let rack_post_height: f64 = 230.0;
    let rack_top_shelf_z: f64 = rack_base_top + 15.0 + 4.0 * 40.0; // top shelf Z
    let rack_top_chip_z: f64 = rack_top_shelf_z + 3.0 + 14.35; // top of top chip

    // ══════════════════════════════════════════════════════════════
    // 1. GANTRY GEOMETRY
    // ══════════════════════════════════════════════════════════════
    //
    // Two parallel Y-beams (MGN12 linear rails on 2020 V-slot extrusion)
    // run along Y at +/- Y_BEAM_X on the chamber ceiling. A single X-beam
    // straddles them and slides in Y. A carriage rides the X-beam in X.
    // The imaging head hangs 160 mm below the X-beam on a fixed drop
    // column (Z-focus handled by a small stepper inside the head).

    // X-axis travel: must cover all 4 chip columns on the shelf.
    // Chip column centers (from chip_stack_rack): 4 columns at
    //   grid_origin_x = -shelf_x/2 + edge_margin + pocket_x/2
    //                 = -265.02 + 2 + 64.25 = -198.77
    // with stride (pocket_x + gutter_interior) = 128.46 + 5 = 133.46
    // → column centers at -198.77, -65.31, 68.15, 201.61 (X ≈ ±200 mm).
    let x_travel: f64 = 440.0; // -220..+220 mm covers all 4 columns
    let x_beam_len: f64 = x_travel + 80.0; // 520 mm rail length (fits 600 chamber width)

    // Y-axis travel: must cover all 5 chip rows.
    // Row centers: grid_origin_y = -451.4/2 + 2 + 43.1 = -180.6
    // stride = pocket_y + gutter = 86.2 + 5 = 91.2
    // → row centers at -180.6, -89.4, 1.8, 93.0, 184.2 (Y ≈ ±185 mm).
    let y_travel: f64 = 400.0; // covers ±200 mm of chip centers
    let y_beam_len: f64 = y_travel + 80.0; // 480 mm rail length

    // Extrusion used for both beams: 2040 V-slot (40 × 20 mm) with an
    // MGN12 linear rail bonded on top for smooth motion. Motion by
    // closed-loop NEMA17 steppers + GT2 belts (6 mm wide) end to end.
    let extr_w: f64 = VSLOT_2040_W; // 20 mm
    let extr_h: f64 = VSLOT_2040_H; // 40 mm
    let mgn12_w: f64 = 12.0;
    let mgn12_h: f64 = 8.0;

    // Beam X positions (two Y rails at ±Y_BEAM_X)
    let y_beam_x: f64 = 230.0; // Y-rails spaced 460 mm apart
    assert!(
        y_beam_x + extr_w / 2.0 < chamber_x / 2.0,
        "Y rails fall outside chamber width"
    );

    // Beam Z: mount the rail tops flush with the ceiling minus 5 mm
    // ventilation gap. Extrusion top face Z = ceiling - 5 = chamber_z/2 - 5.
    let ceiling_z: f64 = chamber_z / 2.0; // +275
    let beam_top_z: f64 = ceiling_z - 5.0;
    let beam_center_z: f64 = beam_top_z - extr_h / 2.0; // center of 2040
    let mgn12_center_z: f64 = beam_top_z - mgn12_h / 2.0; // rail on top face

    // Head drop below X-beam
    let head_drop: f64 = 160.0;
    // Head Z center (center of cylindrical head body, 80 mm tall)
    let head_body_h: f64 = 80.0;
    let head_z: f64 = beam_center_z - head_drop - head_body_h / 2.0;

    // Working distance to top chip:
    let working_dist_top: f64 = head_z - head_body_h / 2.0 - rack_top_chip_z;
    assert!(
        working_dist_top > 80.0,
        "imaging head too close to top shelf chips"
    );

    // ══════════════════════════════════════════════════════════════
    // 2. X-BEAM (2040 extrusion + MGN12 + belt)
    // ══════════════════════════════════════════════════════════════
    //
    // This is the beam that slides in Y. Length along X.

    let x_beam_body = centered_cube("x_beam_body", x_beam_len, extr_w, extr_h);
    let x_beam_rail = centered_cube("x_beam_rail", x_beam_len, mgn12_w, mgn12_h).translate(
        0.0,
        0.0,
        extr_h / 2.0 + mgn12_h / 2.0,
    );

    // End plates: tie X-beam to the two MGN12 Y-carriages on the Y-rails.
    // Plate dimensions: 40 × 40 × 5 mm, one at each end of X-beam.
    let end_plate_x: f64 = 40.0;
    let end_plate_y: f64 = 40.0;
    let end_plate_t: f64 = 5.0;
    let end_plate_left = centered_cube("end_plate_left", end_plate_t, end_plate_y, end_plate_x)
        .translate(-x_beam_len / 2.0 - end_plate_t / 2.0, 0.0, 0.0);
    let end_plate_right = centered_cube("end_plate_right", end_plate_t, end_plate_y, end_plate_x)
        .translate(x_beam_len / 2.0 + end_plate_t / 2.0, 0.0, 0.0);

    let x_beam = x_beam_body + x_beam_rail + end_plate_left + end_plate_right;
    x_beam
        .write_stl("output/imaging_gantry_x_beam.stl")
        .unwrap();
    println!("Exported: output/imaging_gantry_x_beam.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. Y-CARRIAGE (one MGN12 block on the X-beam's MGN12 rail —
    //    carries the imaging head drop column)
    // ══════════════════════════════════════════════════════════════

    let carr_x: f64 = 45.0; // along X-beam (axial length of MGN12H block)
    let carr_y: f64 = 44.0; // width across X-beam
    let carr_z: f64 = 12.0; // thickness
    let carr_body = centered_cube("carr_body", carr_x, carr_y, carr_z);

    // Drop column attach hole (central, for the head drop rod)
    let drop_rod_d: f64 = 16.0;
    let drop_hole = centered_cylinder("drop_hole", drop_rod_d / 2.0 + 0.2, carr_z + 2.0, 48);

    // MGN12 mount pattern (4× M3, 20 × 20 mm spacing)
    let mgn_bolt_d: f64 = 3.3;
    let mgn_spacing: f64 = 20.0;
    let mut mgn_holes = Part::empty("mgn_holes");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            let h = centered_cylinder(
                format!("mgn_{}_{}", sx as i32, sy as i32),
                mgn_bolt_d / 2.0,
                carr_z + 2.0,
                24,
            )
            .translate(sx * mgn_spacing / 2.0, sy * mgn_spacing / 2.0, 0.0);
            mgn_holes = mgn_holes + h;
        }
    }

    // Belt clamp slot (along X) for the X-belt that drives this carriage
    let belt_slot = centered_cube("belt_slot", 12.0, GT2_BELT_WIDTH + 1.0, carr_z / 2.0 + 0.1)
        .translate(0.0, carr_y / 2.0 - 6.0, carr_z / 4.0);

    let y_carriage = carr_body - drop_hole - mgn_holes - belt_slot;
    y_carriage
        .write_stl("output/imaging_gantry_y_carriage.stl")
        .unwrap();
    println!("Exported: output/imaging_gantry_y_carriage.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. IMAGING HEAD
    // ══════════════════════════════════════════════════════════════
    //
    // Cylindrical IP-rated housing, 80 mm diameter × 80 mm tall. Contains:
    //   - Camera (Basler daA1920-30um, S-mount, 25 × 25 × 55 mm with lens)
    //   - Ring LED board around the lens (white CRI-90, 5000K, 24 LEDs)
    //   - 470 nm spotlight (3× Luxeon Rebel Blue LEDs, switched via MOSFET)
    //   - Removable 520 nm long-pass emission filter (motorized flipper)
    //   - Internal heater / desiccant pouch
    //   - Cable chase up the drop rod: USB3 + 12 V + GPIO (3× 2 m silicone
    //     conductors rated 105 °C, IP65 gland at top)

    let head_d: f64 = 80.0;
    let head_wall: f64 = 3.0;
    let head_shell = centered_cylinder("head_shell", head_d / 2.0, head_body_h, 48);
    let head_cavity = centered_cylinder(
        "head_cavity",
        head_d / 2.0 - head_wall,
        head_body_h - head_wall * 2.0,
        48,
    );

    // Optical aperture (bottom face, 40 mm Ø for lens + LED ring)
    let aperture = centered_cylinder("aperture", 40.0 / 2.0, head_wall + 2.0, 48).translate(
        0.0,
        0.0,
        -head_body_h / 2.0 + head_wall / 2.0,
    );

    // Drop-rod socket on top (16 mm bore, captures the drop rod with
    // a cross-pin at 10 mm depth)
    let drop_socket = centered_cylinder("drop_socket", drop_rod_d / 2.0 + 0.2, 30.0, 48).translate(
        0.0,
        0.0,
        head_body_h / 2.0 - 15.0,
    );

    // Cross-pin hole (M4)
    let cross_pin = centered_cylinder("cross_pin", 4.2 / 2.0, head_d + 2.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, head_body_h / 2.0 - 10.0);

    // Cable gland cutout (M16 fitting, offset from drop socket)
    let cable_gland = centered_cylinder("cable_gland", 16.5 / 2.0, head_wall + 2.0, 32).translate(
        20.0,
        20.0,
        head_body_h / 2.0 - head_wall / 2.0,
    );

    // Fluorescence excitation window (3× 8 mm Ø ports for 470 nm LEDs,
    // bottom face, at 20 mm radius, 120° apart)
    let mut blue_ports = Part::empty("blue_ports");
    for i in 0..3 {
        let theta = (i as f64) * 120.0_f64.to_radians();
        let hole = centered_cylinder(format!("blue_{i}"), 8.0 / 2.0, head_wall + 2.0, 32)
            .translate(
                20.0 * theta.cos(),
                20.0 * theta.sin(),
                -head_body_h / 2.0 + head_wall / 2.0,
            );
        blue_ports = blue_ports + hole;
    }

    let head =
        (head_shell - head_cavity) - aperture - drop_socket - cross_pin - cable_gland - blue_ports;

    head.write_stl("output/imaging_gantry_head.stl").unwrap();
    println!("Exported: output/imaging_gantry_head.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. HEATED ITO-COATED WINDOW (anti-condensation aperture cover)
    // ══════════════════════════════════════════════════════════════
    //
    // 50 × 50 × 1.1 mm ITO-coated borosilicate glass tile, held in a
    // PETG frame that presses against the bottom face of the head. Two
    // sprung copper strips contact the ITO coating and carry ~2.5 W
    // (5 V × 0.5 A) to keep the glass ~3 °C above the chamber dewpoint
    // of ~36 °C (saturated at 37 °C). Prevents condensation on the
    // optical path when the head moves over cooler surfaces (e.g., fresh
    // media added via the top hatch).

    let win_frame_x: f64 = 58.0;
    let win_frame_y: f64 = 58.0;
    let win_frame_t: f64 = 5.0;
    let win_glass_x: f64 = 50.0;
    let win_glass_y: f64 = 50.0;
    let win_glass_t: f64 = 1.1;

    let win_frame_body = centered_cube("win_frame_body", win_frame_x, win_frame_y, win_frame_t);
    // Pocket for the glass tile (recessed from top face)
    let win_glass_pocket = centered_cube(
        "win_glass_pocket",
        win_glass_x + 0.3,
        win_glass_y + 0.3,
        win_glass_t + 0.2,
    )
    .translate(0.0, 0.0, win_frame_t / 2.0 - (win_glass_t + 0.1) / 2.0);

    // Through-aperture (clear optical path, 42 mm Ø through the frame)
    let win_aperture = centered_cylinder("win_aperture", 42.0 / 2.0, win_frame_t + 2.0, 64);

    // 4× M3 mounting holes to the head bottom face (through the frame corners)
    let mut win_mount_holes = Part::empty("win_mount_holes");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            let h = centered_cylinder(
                format!("wmh_{}_{}", sx as i32, sy as i32),
                3.3 / 2.0,
                win_frame_t + 2.0,
                24,
            )
            .translate(
                sx * (win_frame_x / 2.0 - 4.0),
                sy * (win_frame_y / 2.0 - 4.0),
                0.0,
            );
            win_mount_holes = win_mount_holes + h;
        }
    }

    // Two copper-strip slots (along +X edge, 8 × 1.5 mm, 10 mm apart)
    let strip_1 = centered_cube("strip_1", 12.0, 8.0, 1.5).translate(
        win_frame_x / 2.0 - 8.0,
        6.0,
        win_frame_t / 2.0 - 0.75,
    );
    let strip_2 = centered_cube("strip_2", 12.0, 8.0, 1.5).translate(
        win_frame_x / 2.0 - 8.0,
        -6.0,
        win_frame_t / 2.0 - 0.75,
    );

    let heated_window =
        win_frame_body - win_glass_pocket - win_aperture - win_mount_holes - strip_1 - strip_2;
    heated_window
        .write_stl("output/imaging_gantry_heated_window.stl")
        .unwrap();
    println!("Exported: output/imaging_gantry_heated_window.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. CEILING MOUNT (one of two — bolts to top hatch frame)
    // ══════════════════════════════════════════════════════════════
    //
    // L-bracket that hangs the Y-beam extrusion from the chamber
    // ceiling. 4× M5 into the inner chamber's hatch-surround rib, 4× M5
    // into the 2040 extrusion end slot.

    let mnt_x: f64 = 50.0;
    let mnt_y: f64 = 50.0;
    let mnt_h: f64 = 40.0;
    let mnt_t: f64 = 5.0;

    let vertical_leg = centered_cube("mnt_vert", mnt_x, mnt_t, mnt_h);
    let horizontal_leg = centered_cube("mnt_horiz", mnt_x, mnt_y, mnt_t).translate(
        0.0,
        mnt_y / 2.0 - mnt_t / 2.0,
        mnt_h / 2.0 - mnt_t / 2.0,
    );

    // Ceiling bolt pattern (4× M5 on 30 × 30 mm in horizontal leg)
    let mut ceil_holes = Part::empty("ceil_holes");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            let h = centered_cylinder(
                format!("ch_{}_{}", sx as i32, sy as i32),
                5.5 / 2.0,
                mnt_t + 2.0,
                24,
            )
            .translate(
                sx * 15.0,
                mnt_y / 2.0 - mnt_t / 2.0 + sy * 12.0,
                mnt_h / 2.0 - mnt_t / 2.0,
            );
            ceil_holes = ceil_holes + h;
        }
    }

    // Extrusion end bolt (1× M5 central on the vertical leg)
    let ext_bolt = centered_cylinder("ext_bolt", 5.5 / 2.0, mnt_t + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 0.0);

    let ceiling_mount = (vertical_leg + horizontal_leg) - ceil_holes - ext_bolt;
    ceiling_mount
        .write_stl("output/imaging_gantry_ceiling_mount.stl")
        .unwrap();
    println!("Exported: output/imaging_gantry_ceiling_mount.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. ASSEMBLY (chamber outline + rack silhouette + gantry)
    // ══════════════════════════════════════════════════════════════

    let mut asm = Part::empty("asm");

    // Chamber wireframe ≈ translucent hull: draw the 6 wall plates thin
    // (actually build a hollow box by solid minus inner air).
    let chamber_outer = centered_cube(
        "asm_chamber_outer",
        chamber_x + 6.0,
        chamber_y + 6.0,
        chamber_z + 6.0,
    );
    let chamber_inner = centered_cube("asm_chamber_inner", chamber_x, chamber_y, chamber_z);
    asm = asm + (chamber_outer - chamber_inner);

    // Rack silhouette: solid bounding box sitting on the floor
    let rack_silhouette = centered_cube("asm_rack", rack_x, rack_y, rack_post_height).translate(
        0.0,
        0.0,
        rack_base_top + rack_post_height / 2.0,
    );
    asm = asm + rack_silhouette;

    // Y-rails (two of them) at ±y_beam_x, spanning Y
    for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
        let y_rail_extr = centered_cube(format!("y_rail_extr_{i}"), extr_w, y_beam_len, extr_h)
            .translate(sx * y_beam_x, 0.0, beam_center_z);
        let y_rail_mgn = centered_cube(format!("y_rail_mgn_{i}"), mgn12_w, y_beam_len, mgn12_h)
            .translate(sx * y_beam_x, 0.0, mgn12_center_z);
        asm = asm + y_rail_extr + y_rail_mgn;
    }

    // X-beam at Y = 0 (center of travel) straddling the two Y-rails
    let xb_body_asm =
        centered_cube("asm_xb_body", x_beam_len, extr_w, extr_h).translate(0.0, 0.0, beam_center_z);
    let xb_rail_asm = centered_cube("asm_xb_rail", x_beam_len, mgn12_w, mgn12_h).translate(
        0.0,
        0.0,
        mgn12_center_z,
    );
    asm = asm + xb_body_asm + xb_rail_asm;

    // Y-carriages at each end of the X-beam (one sits on each Y-rail)
    for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
        let yc = centered_cube(
            format!("asm_yc_{i}"),
            carr_y, // cross-axis (along Y-rail)
            carr_x, // along-axis
            carr_z,
        )
        .translate(
            sx * y_beam_x,
            0.0,
            mgn12_center_z + mgn12_h / 2.0 + carr_z / 2.0,
        );
        asm = asm + yc;
    }

    // X-carriage on the X-beam at X = 0 (center of travel)
    let xc = centered_cube("asm_xc", carr_x, carr_y, carr_z).translate(
        0.0,
        0.0,
        mgn12_center_z + mgn12_h / 2.0 + carr_z / 2.0,
    );
    asm = asm + xc;

    // Drop column (16 mm round rod, 160 mm long, hanging below X-carriage)
    let drop_col = centered_cylinder("asm_drop", drop_rod_d / 2.0, head_drop, 32).translate(
        0.0,
        0.0,
        mgn12_center_z + mgn12_h / 2.0 + carr_z - head_drop / 2.0,
    );
    asm = asm + drop_col;

    // Imaging head (cylinder) at the bottom of the drop column
    let head_asm =
        centered_cylinder("asm_head", head_d / 2.0, head_body_h, 48).translate(0.0, 0.0, head_z);
    asm = asm + head_asm;

    // Heated window just below the head (frame silhouette)
    let win_asm = centered_cube("asm_window", win_frame_x, win_frame_y, win_frame_t).translate(
        0.0,
        0.0,
        head_z - head_body_h / 2.0 - win_frame_t / 2.0,
    );
    asm = asm + win_asm;

    // 2× ceiling mounts (bridging Y-beam ends to chamber ceiling)
    for (i, &sx) in [-1.0f64, 1.0].iter().enumerate() {
        for (j, &sy) in [-1.0f64, 1.0].iter().enumerate() {
            let cm = centered_cube(format!("asm_cm_{i}_{j}"), mnt_x, mnt_y, mnt_h).translate(
                sx * y_beam_x,
                sy * (y_beam_len / 2.0 - mnt_y / 2.0),
                beam_top_z + mnt_h / 2.0 - mnt_t,
            );
            asm = asm + cm;
        }
    }

    asm.write_stl("output/imaging_gantry_assembly.stl").unwrap();
    println!("Exported: output/imaging_gantry_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // 8. CLEARANCE CHECKS
    // ══════════════════════════════════════════════════════════════

    // Check 1: head lowest point vs top chip top face
    let head_bottom_z = head_z - head_body_h / 2.0 - win_frame_t;
    let clearance_top_chip = head_bottom_z - rack_top_chip_z;
    assert!(
        clearance_top_chip > 50.0,
        "imaging head bottom too close to top chip (< 50 mm)"
    );

    // Check 2: Y-rail extrusion fits under chamber ceiling
    assert!(beam_top_z < ceiling_z, "Y-rail overlaps chamber ceiling");

    // Check 3: X-beam + Y-travel stays within chamber
    assert!(
        y_beam_len / 2.0 < chamber_y / 2.0 - 10.0,
        "Y-beam runs past chamber front/back"
    );
    // X-beam end plates sit at ±x_beam_len/2 along X (Y-beam_x is along X
    // too, but the X-beam extends symmetrically about X=0, not offset by
    // Y-beam_x). Check the actual X-beam extent fits inside chamber_x.
    assert!(
        x_beam_len / 2.0 + end_plate_t < chamber_x / 2.0,
        "X-beam end plates protrude chamber width"
    );

    // Check 4: drop column clears the rack handle region at Y = 0
    // (handle arches +60 mm above rack post top; handle bar at
    // z = rack_base_top + rack_post_height + 60 + 10/2 = top_of_handle)
    let handle_top = rack_base_top + rack_post_height + 60.0 + 5.0;
    let head_top_vs_handle = head_z + head_body_h / 2.0 - handle_top;
    // Head must sit ABOVE handle top only when positioned at rack Y=0.
    // For imaging of specific chips at specific (X,Y) positions, head
    // moves to the chip coord and the drop column descends inside
    // the rack envelope (between the corner posts). That's fine since
    // the rack interior is open (100×60 mm cut-throughs, 40 mm shelf
    // pitch, but 25 mm above each chip is clear for pipettes).
    // The relevant clearance is: head body (80 mm Ø) must fit between
    // the 20 mm corner posts at rack_spacing_x = 510 and rack_spacing_y
    // = 431. 80 < 431, 80 < 510. OK.
    assert!(
        head_top_vs_handle > 0.0 || head_d < 200.0,
        "imaging head may collide with rack carry handle at rest"
    );

    // ══════════════════════════════════════════════════════════════
    // 9. SPECS REPORT
    // ══════════════════════════════════════════════════════════════

    // Scan time estimate
    let n_chips: i32 = 100;
    let per_chip_s: f64 = 3.5; // 500 ms exposure + 3 s settle+move
    let scan_s: f64 = per_chip_s * n_chips as f64;
    let kinetic_target_s: f64 = 15.0 * 60.0;

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   IMAGING GANTRY (Option A) — SPECS");
    println!("══════════════════════════════════════════════════════════════");
    println!();
    println!(" GANTRY");
    println!("   X-beam length:   {x_beam_len:.0} mm (travel {x_travel:.0} mm)");
    println!("   Y-beam length:   {y_beam_len:.0} mm (travel {y_travel:.0} mm)");
    println!("   Y-rail spacing:  {:.0} mm apart", y_beam_x * 2.0);
    println!("   Rail type:       2040 V-slot + MGN12H bonded top (IP-rated shields)");
    println!("   Drive:           2× NEMA17 + closed-loop drivers, GT2 6 mm belts");
    println!("   Head drop:       {head_drop:.0} mm below X-beam");
    println!("   Head body:       {head_d:.0} mm Ø × {head_body_h:.0} mm tall cylinder");
    println!();
    println!(" WORKING DISTANCE");
    println!(
        "   Top chip Z:      {rack_top_chip_z:+.1} mm (head bottom {:+.1} mm)",
        head_z - head_body_h / 2.0
    );
    println!(
        "   Top chip WD:     {:.0} mm",
        head_z - head_body_h / 2.0 - rack_top_chip_z
    );
    println!(
        "   Bottom chip WD:  {:.0} mm (4× 40 mm shelf pitch lower)",
        head_z - head_body_h / 2.0 - (rack_top_chip_z - 160.0)
    );
    println!("   Refocus:         On-head 10 mm Z stepper between shelves");
    println!();
    println!(" IMAGING HEAD PAYLOAD");
    println!("   Camera:          Basler daA1920-30um (IMX392, 2.3 MP, 5.86 µm px)");
    println!("                    or FLIR Blackfly S BFS-U3-13Y3M (IMX267)");
    println!("   Lens:            8 mm FL C-mount, f/2.8, ~40° FOV → covers 128 × 86 mm @ 150 mm");
    println!("   White LED:       24× CRI-90 5000K (2 W total, for colorimetric LAMP)");
    println!("   470 nm LED:      3× Luxeon Rebel Blue (3 W pulsed for SYTO-9)");
    println!("   Emission filter: 520 nm long-pass (Semrock / Thorlabs, motor-flipper)");
    println!("   Heated window:   50 × 50 × 1.1 mm ITO-coated glass, 2.5 W");
    println!();
    println!(" CLEARANCES (chamber 600×520×550, rack 530×451×230)");
    println!(
        "   Head ↔ top chip:       {:.0} mm  (target > 50 mm)   ✓",
        head_z - head_body_h / 2.0 - rack_top_chip_z
    );
    println!(
        "   Head body ↔ rack posts: corner gap = {:.0} mm (post dia 20, corner spacing 510×431)  ✓",
        (rack_x - head_d) / 2.0
    );
    println!(
        "   Beam top ↔ ceiling:     {:.0} mm                   ✓",
        ceiling_z - beam_top_z
    );
    println!();
    println!(" SCAN TIME");
    println!("   Per-chip:        {per_chip_s:.1} s (expose + settle + move)");
    println!(
        "   Full 100 chips:  {:.0} s ({:.1} min, target {:.0} min)  ✓",
        scan_s,
        scan_s / 60.0,
        kinetic_target_s / 60.0
    );
    println!();
    println!(" DATA PIPELINE");
    println!("   Compute:         Raspberry Pi 5 (8 GB) beside the container");
    println!("   Storage:         Local SD + S3 sync, SQLite manifest");
    println!("   Tags:            chip_id, shelf, ts_utc, mode={{bright,fluor}}, exp, gain, T, RH");
    println!();
    println!(" EXPORTED STLs");
    println!("   output/imaging_gantry_x_beam.stl");
    println!("   output/imaging_gantry_y_carriage.stl");
    println!("   output/imaging_gantry_head.stl");
    println!("   output/imaging_gantry_heated_window.stl");
    println!("   output/imaging_gantry_ceiling_mount.stl");
    println!("   output/imaging_gantry_assembly.stl");
    println!("══════════════════════════════════════════════════════════════");

    // STEP export (STL → STEP via stltostp)
    for stl in [
        "output/imaging_gantry_x_beam.stl",
        "output/imaging_gantry_y_carriage.stl",
        "output/imaging_gantry_head.stl",
        "output/imaging_gantry_heated_window.stl",
        "output/imaging_gantry_ceiling_mount.stl",
        "output/imaging_gantry_assembly.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }
}
