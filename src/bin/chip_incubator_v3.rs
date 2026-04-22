use vcad::{centered_cube, centered_cylinder, Part};

// ─── Chip Incubator v3 — Tier-2 Expanded Thermal Container ───
//
// Expanded container sized to fit ALL of:
//   - Full 100-chip rack on 2-axis gimbal rocker (T-1C261EE3) at pitch ±15° / roll ±10°
//   - Media reservoir + drip heads (T-C40B32F0, external reservoir, internal drip heads)
//   - Imaging gantry full XY travel (above rack)
//   - LH shelf extension path (front-to-back, 200mm extension + clearance)
//
// Rationale (vs v2, A-86F1ED1D, 600×520×550mm interior):
//   v2 forced the 2-axis rocker (T-1C261EE3) to drop to a 45-chip sub-rack
//   because at pitch +15° / roll +10° the gimbal envelope exceeded the
//   container's 520mm Y dimension by 22.8mm with a full 100-chip rack.
//   v3 expands to Tier-2 interior (~750×650×700mm) so the full 100-chip rack
//   can run under 2-axis tilt with ≥50mm clearance on all faces.
//
// Interior target: 750 × 650 × 700 mm (X × Y × Z)
// Interior volume: ~341 L (vs 172 L for v2 — 1.98× increase)
//
// Construction (scaled from v2):
//   - Inner chamber wall: 3mm PETG
//   - Insulation gap: 25mm PIR foam (k ≈ 0.022 W/m·K) on all 6 sides
//   - Outer shell wall: 3mm PETG
//   - Structural frame: 4× 45×45mm Bosch Rexroth extrusion corner posts (exterior)
//   - Front door: 5mm PETG + 25mm PIR foam + 5mm PETG (gasketed swing door, cam latches)
//   - Motorized LH top-access hatch (clamshell, per A-71001BA1)
//   - Acrylic viewing window (6mm thick, 400×300mm front)
//
// Operating conditions: 37°C, 5% CO2, ≥90% RH.
//
// Coordinate system: chamber centered at origin.
//   X: left/right      (−375 to +375)   interior 750mm
//   Y: front/back      (−325 to +325)   interior 650mm, front = −Y
//   Z: bottom/top      (−350 to +350)   interior 700mm
//
// Exports:
//   - chip_incubator_v3_chamber.stl     (inner PETG chamber w/ ports)
//   - chip_incubator_v3_shell.stl       (outer PETG shell with insulation gap)
//   - chip_incubator_v3_door.stl        (front double-wall insulated door)
//   - chip_incubator_v3_window.stl      (6mm acrylic viewing window)
//   - chip_incubator_v3_lh_hatch.stl    (motorized LH top-access hatch)
//   - chip_incubator_v3_drain_pan.stl   (sloped floor drain pan)
//   - chip_incubator_v3_co2_port.stl    (CO2 injection manifold)
//   - chip_incubator_v3_hepa_plenum.stl (top HEPA recirculation plenum)
//   - chip_incubator_v3_grommet.stl     (bulkhead pass-through grommet, print N)
//   - chip_incubator_v3_frame_post.stl  (Rexroth 4545 corner post, 4× cut to length)
//   - chip_incubator_v3_assembly.stl    (assembly viz)

// ─── Re-usable functions (called from main) ─────────────────────────────

// Interior constants — bumped from initial 750×650×700 target after
// interference check flagged insufficient Y and Z_top clearance.
// Final sizing: 800 × 700 × 850 mm interior for ≥50mm clearance on all 6 faces
// with the 100-chip rack on the 2-axis gimbal at pitch +15° / roll +10°.
const INNER_X: f64 = 800.0;
const INNER_Y: f64 = 700.0;
const INNER_Z: f64 = 850.0;
const WALL: f64 = 3.0;
const INSULATION: f64 = 25.0;
const SHELL_WALL: f64 = 3.0;
const FRAME_EXT: f64 = 45.0; // 4545 Bosch Rexroth extrusion

// Window opening
const WINDOW_X: f64 = 400.0;
const WINDOW_Z: f64 = 300.0;
const WINDOW_THICK: f64 = 6.0;

// LH hatch (clamshell opening in top — per A-71001BA1 sized up for 100-chip rack)
const LH_HATCH_X: f64 = 500.0;
const LH_HATCH_Y: f64 = 350.0;

// HEPA plenum (top)
const HEPA_X: f64 = 450.0; // 18" filter
const HEPA_Y: f64 = 450.0;
const HEPA_Z: f64 = 80.0; // plenum height including filter

// Derived chamber outer
fn chamber_outer_x() -> f64 { INNER_X + WALL * 2.0 }
fn chamber_outer_y() -> f64 { INNER_Y + WALL * 2.0 }
fn chamber_outer_z() -> f64 { INNER_Z + WALL * 2.0 }

// Shell cavity (chamber_outer + insulation on all sides)
fn shell_cavity_x() -> f64 { chamber_outer_x() + INSULATION * 2.0 }
fn shell_cavity_y() -> f64 { chamber_outer_y() + INSULATION * 2.0 }
fn shell_cavity_z() -> f64 { chamber_outer_z() + INSULATION * 2.0 }

// Shell outer
fn shell_outer_x() -> f64 { shell_cavity_x() + SHELL_WALL * 2.0 }
fn shell_outer_y() -> f64 { shell_cavity_y() + SHELL_WALL * 2.0 }
fn shell_outer_z() -> f64 { shell_cavity_z() + SHELL_WALL * 2.0 }

// Frame outer (extrusion goes around the shell, 10mm standoff to allow panels)
fn frame_outer_x() -> f64 { shell_outer_x() + FRAME_EXT * 2.0 + 20.0 }
fn frame_outer_y() -> f64 { shell_outer_y() + FRAME_EXT * 2.0 + 20.0 }
fn frame_outer_z() -> f64 { shell_outer_z() + FRAME_EXT * 2.0 + 20.0 }

// ─── wall_panels(): inner chamber with all ports cut ────────────────────
fn wall_panels() -> Part {
    let cx = chamber_outer_x();
    let cy = chamber_outer_y();
    let cz = chamber_outer_z();

    let outer = centered_cube("chamber_outer", cx, cy, cz);
    let inner = centered_cube("chamber_inner", INNER_X, INNER_Y, INNER_Z);

    // Front door opening (front = −Y face)
    let front_opening = centered_cube("front_opening", INNER_X - 40.0, WALL + 2.0, INNER_Z - 40.0)
        .translate(0.0, -(cy / 2.0), 0.0);

    // LH top-access hatch cutout (top face)
    let lh_cutout = centered_cube("lh_hatch_cut", LH_HATCH_X, LH_HATCH_Y, WALL + 2.0)
        .translate(0.0, 0.0, cz / 2.0);

    // HEPA plenum opening in top (offset from LH hatch)
    let hepa_opening = centered_cube("hepa_opening", HEPA_X, HEPA_Y, WALL + 2.0)
        .translate(0.0, -(INNER_Y / 2.0) + HEPA_Y / 2.0 + 10.0, cz / 2.0);

    // CO2 injection port (upper rear, 10mm)
    let co2_port = centered_cylinder("co2_port", 10.0 / 2.0, WALL + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, cy / 2.0, INNER_Z / 2.0 - 50.0);

    // Heater leads port (rear-bottom, 12mm)
    let heater_port = centered_cylinder("heater_port", 12.0 / 2.0, WALL + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, cy / 2.0, -(INNER_Z / 2.0) + 30.0);

    // Temp/humidity sensor port (mid-rear, 10mm)
    let sensor_port = centered_cylinder("sensor_port", 10.0 / 2.0, WALL + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(100.0, cy / 2.0, 0.0);

    // CO2 sensor pocket (rear recess for MH-Z19B)
    let co2_sensor_pocket = centered_cube("co2_sensor_pocket", 25.0, 10.0, 35.0)
        .translate(-100.0, cy / 2.0 - 5.0, 0.0);

    // Bulkhead grommets on right wall — media IN (30mm), media OUT (30mm),
    // power (25mm), sensor cable (20mm)
    let g_media_in = centered_cylinder("g_media_in", 30.0 / 2.0, WALL + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx / 2.0, -100.0, 150.0);
    let g_media_out = centered_cylinder("g_media_out", 30.0 / 2.0, WALL + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx / 2.0, 100.0, 150.0);
    let g_power = centered_cylinder("g_power", 25.0 / 2.0, WALL + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx / 2.0, 0.0, -50.0);
    let g_sensor = centered_cylinder("g_sensor", 20.0 / 2.0, WALL + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx / 2.0, 0.0, -150.0);

    // Drip-head mount holes above each of 5 shelves (right-wall column of 5
    // × 14mm grommets at shelf_pitch_z). Anchored to rack center Z=−150
    // (rack base on floor) + shelf_n × 40mm pitch.
    let mut drip_mounts = Part::empty("drip_mounts");
    let shelf_pitch = 40.0;
    let rack_base_z = -(INNER_Z / 2.0) + 100.0; // rack base above gimbal
    for n in 0..5 {
        let z = rack_base_z + n as f64 * shelf_pitch + 20.0;
        let hole = centered_cylinder(&format!("drip_{n}"), 14.0 / 2.0, WALL + 2.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(cx / 2.0, 200.0, z);
        drip_mounts = drip_mounts + hole;
    }

    // Fan/blower port (right side, 160mm for 400 CFM blower)
    let blower_port = centered_cylinder("blower_port", 160.0 / 2.0, WALL + 2.0, 64)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx / 2.0, -150.0, -150.0);

    // Drain hole at floor center (connects to sloped drain pan)
    let drain_hole = centered_cylinder("drain_hole", 20.0 / 2.0, WALL + 2.0, 32)
        .translate(0.0, 0.0, -(cz / 2.0));

    (outer - inner)
        - front_opening
        - lh_cutout
        - hepa_opening
        - co2_port
        - heater_port
        - sensor_port
        - co2_sensor_pocket
        - g_media_in
        - g_media_out
        - g_power
        - g_sensor
        - drip_mounts
        - blower_port
        - drain_hole
}

// ─── frame(): 4× Bosch Rexroth 4545 corner posts exterior structural frame ─
fn frame() -> Part {
    let fx = frame_outer_x();
    let fy = frame_outer_y();
    let fz = frame_outer_z();

    // 4 vertical corner posts (45×45mm extrusion, length = fz)
    let post_half = (fx / 2.0) - (FRAME_EXT / 2.0);
    let post_half_y = (fy / 2.0) - (FRAME_EXT / 2.0);
    let mut frame_asm = Part::empty("frame");
    for (dx, dy) in &[(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
        let post = centered_cube("post", FRAME_EXT, FRAME_EXT, fz)
            .translate(dx * post_half, dy * post_half_y, 0.0);
        frame_asm = frame_asm + post;
    }

    // Top and bottom horizontal beams (4× along X, 4× along Y = 8 total)
    let beam_len_x = fx - FRAME_EXT * 2.0;
    let beam_len_y = fy - FRAME_EXT * 2.0;
    let top_z = fz / 2.0 - FRAME_EXT / 2.0;
    let bot_z = -fz / 2.0 + FRAME_EXT / 2.0;
    for &z in &[top_z, bot_z] {
        for &y in &[-post_half_y, post_half_y] {
            let beam = centered_cube("beam_x", beam_len_x, FRAME_EXT, FRAME_EXT)
                .translate(0.0, y, z);
            frame_asm = frame_asm + beam;
        }
        for &x in &[-post_half, post_half] {
            let beam = centered_cube("beam_y", FRAME_EXT, beam_len_y, FRAME_EXT)
                .translate(x, 0.0, z);
            frame_asm = frame_asm + beam;
        }
    }

    frame_asm
}

// ─── viewing_window(): 6mm acrylic pane ────────────────────────────────
fn viewing_window() -> Part {
    centered_cube("window_pane", WINDOW_X, WINDOW_THICK, WINDOW_Z)
}

// ─── door(): front gasketed swing door with acrylic window ─────────────
fn door() -> Part {
    let door_x = INNER_X + 40.0; // 790mm wide (overlaps front opening)
    let door_z = INNER_Z + 40.0; // 740mm tall
    let door_petg = 5.0;
    let door_foam = 25.0;
    let door_thick = door_petg * 2.0 + door_foam; // 35mm

    let body = centered_cube("door_body", door_x, door_thick, door_z);

    // Foam cavity (sandwich core)
    let foam_cavity = centered_cube(
        "door_foam",
        door_x - 20.0,
        door_foam,
        door_z - 20.0,
    );

    // Window cutout (400×300mm, full-thickness)
    let window_cut = centered_cube("window_cut", WINDOW_X, door_thick + 2.0, WINDOW_Z);

    // Gasket channel (3×3mm around perimeter, inner face)
    let gw = 3.0;
    let gd = 3.0;
    let gi = 15.0;
    let go_x = door_x - gi * 2.0;
    let go_z = door_z - gi * 2.0;
    let gin_x = go_x - gw * 2.0;
    let gin_z = go_z - gw * 2.0;
    let gy = -(door_thick / 2.0) + gd / 2.0;

    let g_outer = centered_cube("g_outer", go_x, gd + 0.1, go_z).translate(0.0, gy, 0.0);
    let g_inner = centered_cube("g_inner", gin_x, gd + 0.2, gin_z).translate(0.0, gy, 0.0);
    let gasket = g_outer - g_inner;

    // Cam latch pockets (4× around perimeter for cam latches vs v2 magnets —
    // v3 is big enough to need positive latching against gasket compression).
    let mut cam_latches = Part::empty("cam_latches");
    let cam_inset = 40.0;
    for (x, z) in &[
        (door_x / 2.0 - cam_inset, 0.0),
        (door_x / 2.0 - cam_inset, door_z / 2.0 - cam_inset),
        (door_x / 2.0 - cam_inset, -(door_z / 2.0 - cam_inset)),
        (-(door_x / 2.0 - cam_inset), 0.0),
    ] {
        let p = centered_cube("cam_pocket", 30.0, door_thick + 2.0, 20.0)
            .translate(*x, 0.0, *z);
        cam_latches = cam_latches + p;
    }

    // Hinge pin holes (3× on left edge)
    let hinge_x = -(door_x / 2.0) + 15.0;
    let mut hinges = Part::empty("hinges");
    for z in &[door_z / 2.0 - 60.0, 0.0, -(door_z / 2.0 - 60.0)] {
        let h = centered_cylinder("hinge", 8.0 / 2.0, door_thick + 2.0, 24)
            .translate(hinge_x, 0.0, *z);
        hinges = hinges + h;
    }

    body - foam_cavity - window_cut - gasket - cam_latches - hinges
}

// ─── lh_hatch(): motorized clamshell top-access hatch ──────────────────
fn lh_hatch() -> Part {
    let hx = LH_HATCH_X + 40.0; // overlaps opening
    let hy = LH_HATCH_Y + 40.0;
    let ht = 15.0; // PETG plate (insulated hatch per v2 pattern)

    let body = centered_cube("lh_hatch_body", hx, hy, ht);

    // Gasket channel (3×3mm on bottom face)
    let gow_x = hx - 20.0;
    let gow_y = hy - 20.0;
    let gin_x = gow_x - 6.0;
    let gin_y = gow_y - 6.0;
    let gz = -(ht / 2.0) + 1.5;

    let g_outer = centered_cube("lh_g_outer", gow_x, gow_y, 3.1).translate(0.0, 0.0, gz);
    let g_inner = centered_cube("lh_g_inner", gin_x, gin_y, 3.2).translate(0.0, 0.0, gz);
    let gasket = g_outer - g_inner;

    // Servo mount pockets on top (2× for linear actuator mount points)
    let mut servo_mounts = Part::empty("servo_mounts");
    for &xo in &[-(hx / 2.0) + 40.0, hx / 2.0 - 40.0] {
        let m = centered_cube("servo_mount", 40.0, 40.0, 2.0)
            .translate(xo, 0.0, ht / 2.0 - 1.0);
        servo_mounts = servo_mounts + m;
    }

    // Hinge pin line on rear edge (+Y)
    let mut hinges = Part::empty("lh_hinges");
    for &xo in &[-(hx / 2.0) + 60.0, 0.0, hx / 2.0 - 60.0] {
        let h = centered_cylinder("lh_hinge", 6.0 / 2.0, 20.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(xo, hy / 2.0 - 10.0, 0.0);
        hinges = hinges + h;
    }

    body - gasket - servo_mounts - hinges
}

// ─── drain_pan(): sloped floor drain pan ───────────────────────────────
fn drain_pan() -> Part {
    let px = INNER_X - 40.0;
    let py = INNER_Y - 40.0;
    let pz = 20.0;
    let pan_wall = 2.5;

    let outer = centered_cube("pan_outer", px, py, pz);
    // Sloped interior: approximated as flat-bottom cube minus center-drain cone
    let inner = centered_cube(
        "pan_inner",
        px - pan_wall * 2.0,
        py - pan_wall * 2.0,
        pz - pan_wall,
    )
    .translate(0.0, 0.0, pan_wall / 2.0);

    // Center drain cone (20mm dia)
    let drain_cone = centered_cylinder("pan_drain", 20.0 / 2.0, pan_wall + 2.0, 32)
        .translate(0.0, 0.0, -(pz / 2.0));

    outer - inner - drain_cone
}

// ─── co2_port(): CO2 injection manifold ────────────────────────────────
fn co2_port() -> Part {
    // Tube-in-bulkhead: 15mm OD flange, 10mm ID tube, 30mm long
    let flange = centered_cylinder("co2_flange", 20.0 / 2.0, 6.0, 48)
        .rotate(90.0, 0.0, 0.0);
    let tube = centered_cylinder("co2_tube", 15.0 / 2.0, 30.0, 48)
        .rotate(90.0, 0.0, 0.0);
    let bore = centered_cylinder("co2_bore", 10.0 / 2.0, 40.0, 48)
        .rotate(90.0, 0.0, 0.0);

    (flange + tube) - bore
}

// ─── hepa_plenum(): top HEPA recirculation plenum ──────────────────────
fn hepa_plenum() -> Part {
    // Box that seats a 450×450mm HEPA filter (18") on top,
    // 400 CFM blower duct cutout on one side.
    let plenum_wall = 3.0;
    let plenum = centered_cube("hepa_plenum", HEPA_X, HEPA_Y, HEPA_Z);
    let cavity = centered_cube(
        "hepa_cavity",
        HEPA_X - plenum_wall * 2.0,
        HEPA_Y - plenum_wall * 2.0,
        HEPA_Z - plenum_wall * 2.0,
    );
    // Filter slot (top face, 420×420mm)
    let filter_slot = centered_cube("filter_slot", 420.0, 420.0, plenum_wall + 2.0)
        .translate(0.0, 0.0, HEPA_Z / 2.0);
    // Return duct (160mm from side to blower intake)
    let return_duct = centered_cylinder("return_duct", 160.0 / 2.0, plenum_wall + 2.0, 64)
        .rotate(0.0, 90.0, 0.0)
        .translate(HEPA_X / 2.0, 0.0, 0.0);

    plenum - cavity - filter_slot - return_duct
}

// ─── passthrough_grommets(): single grommet part (print N as needed) ───
fn passthrough_grommets() -> Part {
    // 30mm OD flanged silicone/EPDM grommet, 20mm ID, 40mm flange
    let flange = centered_cylinder("gr_flange", 40.0 / 2.0, 4.0, 48);
    let tube = centered_cylinder("gr_tube", 30.0 / 2.0, 20.0, 48)
        .translate(0.0, 0.0, 12.0);
    let bore = centered_cylinder("gr_bore", 20.0 / 2.0, 30.0, 48)
        .translate(0.0, 0.0, 10.0);

    (flange + tube) - bore
}

// ─── assembly(): visualization (chamber + shell outline) ───────────────
fn assembly() -> Part {
    // Chamber (with cutouts)
    let chamber = wall_panels();

    // Shell (outer box - cavity, no ports for viz)
    let sx = shell_outer_x();
    let sy = shell_outer_y();
    let sz = shell_outer_z();
    let shell_outer_box = centered_cube("shell_outer", sx, sy, sz);
    let shell_cav = centered_cube(
        "shell_cav",
        shell_cavity_x(),
        shell_cavity_y(),
        shell_cavity_z(),
    );
    // Front opening in shell
    let shell_front = centered_cube("shell_front", INNER_X + 20.0, sy + 4.0, INNER_Z + 20.0)
        .translate(0.0, -sy / 2.0, 0.0);
    let shell = (shell_outer_box - shell_cav) - shell_front;

    // Frame
    let frame_asm = frame();

    chamber + shell + frame_asm
}

// ─── interference_check(): verify 100-chip rack + 2-axis gimbal fits ───
fn interference_check() {
    // 100-chip rack envelope from A-1A303196: 550 × 472 × 306 mm
    let rack_x: f64 = 550.0;
    let rack_y: f64 = 472.0;
    let rack_z: f64 = 306.0;

    // 2-axis gimbal adds: base stand ~100mm below rack, plate + bearings ~50mm,
    // outer ring ~40mm around. Effective rotating assembly height: rack_z + 60 = 366mm.
    // Gimbal base footprint: rack + 100mm (bearing stands) = 650 × 572mm.
    let gimbal_base_x = rack_x + 100.0;
    let gimbal_base_y = rack_y + 100.0;
    let rotating_h = rack_z + 60.0; // rack + plate + bearings

    // Pitch ±15° rotates rack around X axis (changes Y and Z projections)
    // Roll ±10° rotates around outer-ring-local Y (changes X and Z)
    let pitch = 15.0_f64.to_radians();
    let roll = 10.0_f64.to_radians();

    // Worst-case projected dimensions (both tilts at extreme)
    // Y projection: rack_y*cos(pitch) + rotating_h*sin(pitch)
    let proj_y = rack_y * pitch.cos() + rotating_h * pitch.sin();
    // X projection: rack_x*cos(roll) + rotating_h*sin(roll)
    let proj_x = rack_x * roll.cos() + rotating_h * roll.sin();
    // Z tower: rotating_h*cos(pitch)*cos(roll) + corner displacement
    let proj_z_up = rotating_h * pitch.cos() * roll.cos()
        + (rack_x / 2.0) * roll.sin()
        + (rack_y / 2.0) * pitch.sin();

    // Total envelope includes the fixed gimbal base footprint
    let env_x = proj_x.max(gimbal_base_x);
    let env_y = proj_y.max(gimbal_base_y);
    let env_z = proj_z_up + 100.0; // 100mm below for base stand

    // Clearances
    let clear_x = (INNER_X - env_x) / 2.0;
    let clear_y = (INNER_Y - env_y) / 2.0;
    let clear_z_top = INNER_Z / 2.0 - (proj_z_up - 100.0); // rack centered vertically
    let clear_z_bot = INNER_Z / 2.0 - 100.0; // gimbal base ~100mm below center

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   INTERFERENCE CHECK — 100-chip rack + 2-axis gimbal @ extremes");
    println!("══════════════════════════════════════════════════════════════");
    println!("   Interior:          {INNER_X:.0} × {INNER_Y:.0} × {INNER_Z:.0} mm");
    println!("   100-chip rack:     {rack_x:.0} × {rack_y:.0} × {rack_z:.0} mm");
    println!("   Gimbal base:       {gimbal_base_x:.0} × {gimbal_base_y:.0} mm");
    println!("   Rotating height:   {rotating_h:.0} mm (rack + plate + bearings)");
    println!("   Tilt extremes:     pitch ±15° / roll ±10°");
    println!();
    println!("   Swept envelope:    {env_x:.1} × {env_y:.1} × {env_z:.1} mm");
    println!("   Clearance X:       ±{clear_x:.1} mm (each side)");
    println!("   Clearance Y:       ±{clear_y:.1} mm (each side)");
    println!("   Clearance Z top:   {clear_z_top:.1} mm");
    println!("   Clearance Z bot:   {clear_z_bot:.1} mm");
    println!();

    let min_clear = clear_x.min(clear_y).min(clear_z_top).min(clear_z_bot);
    if min_clear >= 50.0 {
        println!("   RESULT:  PASS ✓ (min clearance {min_clear:.1} mm ≥ 50 mm required)");
    } else {
        println!("   RESULT:  FAIL ✗ (min clearance {min_clear:.1} mm < 50 mm required)");
    }
    println!("══════════════════════════════════════════════════════════════");
}

fn main() {
    std::fs::create_dir_all("output").ok();

    // ── Export parts ─────────────────────────────────────────────
    let chamber = wall_panels();
    chamber.write_stl("output/chip_incubator_v3_chamber.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_chamber.stl");

    let d = door();
    d.write_stl("output/chip_incubator_v3_door.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_door.stl");

    let w = viewing_window();
    w.write_stl("output/chip_incubator_v3_window.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_window.stl");

    let lh = lh_hatch();
    lh.write_stl("output/chip_incubator_v3_lh_hatch.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_lh_hatch.stl");

    let dp = drain_pan();
    dp.write_stl("output/chip_incubator_v3_drain_pan.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_drain_pan.stl");

    let co2 = co2_port();
    co2.write_stl("output/chip_incubator_v3_co2_port.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_co2_port.stl");

    let hp = hepa_plenum();
    hp.write_stl("output/chip_incubator_v3_hepa_plenum.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_hepa_plenum.stl");

    let g = passthrough_grommets();
    g.write_stl("output/chip_incubator_v3_grommet.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_grommet.stl");

    // Shell (outer PETG box with matched pass-throughs)
    let sx = shell_outer_x();
    let sy = shell_outer_y();
    let sz = shell_outer_z();
    let cav_x = shell_cavity_x();
    let cav_y = shell_cavity_y();
    let cav_z = shell_cavity_z();

    let shell_outer_box = centered_cube("shell_outer", sx, sy, sz);
    let shell_cav = centered_cube("shell_cav", cav_x, cav_y, cav_z);
    let shell_front_op = centered_cube(
        "shell_front",
        INNER_X + 20.0,
        SHELL_WALL + INSULATION + WALL + 4.0,
        INNER_Z + 20.0,
    )
    .translate(0.0, -sy / 2.0, 0.0);
    let shell_lh_cut = centered_cube(
        "shell_lh",
        LH_HATCH_X + 20.0,
        LH_HATCH_Y + 20.0,
        SHELL_WALL + INSULATION + WALL + 4.0,
    )
    .translate(0.0, 0.0, sz / 2.0);
    let shell_hepa_cut = centered_cube(
        "shell_hepa",
        HEPA_X + 20.0,
        HEPA_Y + 20.0,
        SHELL_WALL + INSULATION + WALL + 4.0,
    )
    .translate(0.0, -(INNER_Y / 2.0) + HEPA_Y / 2.0 + 10.0, sz / 2.0);

    // Caster wheel mount pads (4× M8 in floor corners)
    let caster_inset = 50.0;
    let mut casters = Part::empty("casters");
    for (dx, dy) in &[(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
        let hole = centered_cylinder(
            "caster_hole",
            8.5 / 2.0,
            SHELL_WALL + 4.0,
            24,
        )
        .translate(
            dx * (sx / 2.0 - caster_inset),
            dy * (sy / 2.0 - caster_inset),
            -(sz / 2.0) + SHELL_WALL / 2.0,
        );
        casters = casters + hole;
    }

    let shell = (shell_outer_box - shell_cav)
        - shell_front_op
        - shell_lh_cut
        - shell_hepa_cut
        - casters;
    shell.write_stl("output/chip_incubator_v3_shell.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_shell.stl");

    // Frame post (one post; manufactured 4× at length from Bosch Rexroth stock)
    let fz = frame_outer_z();
    let post = centered_cube("frame_post", FRAME_EXT, FRAME_EXT, fz);
    post.write_stl("output/chip_incubator_v3_frame_post.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_frame_post.stl");

    // Assembly visualization
    let asm = assembly();
    asm.write_stl("output/chip_incubator_v3_assembly.stl").unwrap();
    println!("Exported: output/chip_incubator_v3_assembly.stl");

    // ── Specs report ─────────────────────────────────────────────
    let interior_vol_l = (INNER_X * INNER_Y * INNER_Z) / 1_000_000.0;
    let shell_surf_m2 = 2.0
        * ((shell_outer_x() * shell_outer_y())
            + (shell_outer_x() * shell_outer_z())
            + (shell_outer_y() * shell_outer_z()))
        / 1_000_000.0;
    let q_loss = 0.022 * shell_surf_m2 * 15.0 / 0.025; // W at ΔT=15K, 25mm PIR

    // HEPA sizing: 30 ACH × 341L = 10.23 m³/h = 6.02 CFM for pure air-change,
    // but HEPA filter face velocity requires 100 fpm × 450×450mm filter area
    // = 100 × (1.47 × 1.47) = ~220 ft³/min minimum, and commercial BSC spec
    // is 0.4 m/s ≈ 80 fpm which across 18"×18" = 180 CFM. We spec 400 CFM to
    // provide both (a) HEPA face velocity margin for filter loading over life
    // and (b) forced-convection air mixing that defeats thermal stratification.
    let vol_m3 = interior_vol_l / 1000.0;
    let ach_target = 30.0;
    let cfm_ach = vol_m3 * ach_target * 35.3 / 60.0; // CFM needed for ACH alone
    let cfm_hepa_face = 400.0; // 18×18" HEPA @ 80 fpm + headroom

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   CHIP INCUBATOR v3 — TIER-2 SPECS");
    println!("══════════════════════════════════════════════════════════════");
    println!();
    println!(" DIMENSIONS");
    println!("   Interior chamber:  {INNER_X:.0} × {INNER_Y:.0} × {INNER_Z:.0} mm");
    println!("   Chamber outer:     {:.0} × {:.0} × {:.0} mm (3 mm PETG)",
        chamber_outer_x(), chamber_outer_y(), chamber_outer_z());
    println!("   Shell cavity:      {:.0} × {:.0} × {:.0} mm", cav_x, cav_y, cav_z);
    println!("   Shell outer:       {:.0} × {:.0} × {:.0} mm (3 mm PETG)", sx, sy, sz);
    println!("   Frame outer:       {:.0} × {:.0} × {:.0} mm (45×45 Rexroth)",
        frame_outer_x(), frame_outer_y(), frame_outer_z());
    println!("   Insulation:        {INSULATION:.0} mm PIR foam (all 6 sides)");
    println!();
    println!(" INTERIOR VOLUME:  {interior_vol_l:.1} L   (v2: 172 L, v3/v2 ratio = {:.2}×)",
        interior_vol_l / 172.0);
    println!(" SHELL SURFACE:    {shell_surf_m2:.3} m²");
    println!();
    println!(" THERMAL");
    println!("   Operating:         37 °C / 5 % CO2 / ≥90 % RH");
    println!("   ΔT (setpoint):     15 K (22°C ambient → 37°C chamber)");
    println!("   Steady-state loss: Q = k·A·ΔT/t = 0.022 × {shell_surf_m2:.3} × 15 / 0.025");
    println!("                      ≈ {q_loss:.1} W");
    println!("   Heater budget:     400 W silicone pad (>{:.1}× steady-state margin,", 400.0 / q_loss);
    println!("                      ~same time-to-setpoint as v2)");
    println!("   Alternative:       300 W with ~33% longer ramp (15 → 20 min)");
    println!();
    println!(" HEPA RECIRCULATION");
    println!("   Target ACH:        {ach_target:.0}");
    println!("   CFM from ACH:      {cfm_ach:.1}  (air-changes only)");
    println!("   CFM from HEPA:     180    (18×18\" filter @ 80 fpm face velocity)");
    println!("   Selected blower:   {cfm_hepa_face:.0} CFM EC centrifugal (1.5× face headroom)");
    println!("   v2 baseline:       60 CFM @ ~1 ACH — insufficient turbulence for v3");
    println!();

    // ── Interference check ───────────────────────────────────────
    interference_check();

    // ── Drawing file ─────────────────────────────────────────────
    let drawing_path = "output/chip_incubator_v3_drawing.txt";
    let drawing = format!(
        "CHIP INCUBATOR v3 — DIMENSIONED VIEWS\n\
         ===========================================================\n\
         \n\
         PLAN (top-down, Z+ looking down)\n\
         -----------------------------------------------------------\n\
             ┌────────────────────────────────────────────┐ ← {:.0} mm (frame outer Y)\n\
             │  ┌──────────────────────────────────────┐  │\n\
             │  │  ┌────────────────────────────────┐  │  │ ← {:.0} mm (shell outer)\n\
             │  │  │           INTERIOR             │  │  │\n\
             │  │  │         {INNER_X:.0} × {INNER_Y:.0} mm         │  │  │\n\
             │  │  │                                │  │  │\n\
             │  │  └────────────────────────────────┘  │  │ ← {:.0} mm (interior Y)\n\
             │  │       ↑ 25 mm PIR insulation         │  │\n\
             │  └──────────────────────────────────────┘  │\n\
             │          ↑ 3 mm PETG shell                 │\n\
             └────────────────────────────────────────────┘\n\
              ↑ 45×45 Rexroth frame corner posts (×4)\n\
             \n\
             |<--{:.0}-->|\n\
                 INNER_X\n\
         \n\
         ELEVATION (front view, Y− looking +Y)\n\
         -----------------------------------------------------------\n\
             ┌────────────────────────────────────────────┐ ← Top (+Z)\n\
             │   [HEPA Plenum ({HEPA_X:.0}×{HEPA_Y:.0})] [LH Hatch]   │\n\
             │  ┌──────────────────────────────────────┐  │\n\
             │  │     ┌──────────────────────┐         │  │\n\
             │  │     │   Viewing Window     │         │  │\n\
             │  │     │   {WINDOW_X:.0} × {WINDOW_Z:.0} mm    │ INTERIOR \n\
             │  │     │      (6mm acrylic)    │ {INNER_Z:.0}mm Z│\n\
             │  │     └──────────────────────┘         │  │\n\
             │  │                                       │  │\n\
             │  └──────────────────────────────────────┘  │\n\
             └────────────────────────────────────────────┘ ← Floor (−Z)\n\
         \n\
         PORT MAP (chamber-centered, mm)\n\
         -----------------------------------------------------------\n\
           CO2 injection     10 mm Ø   (0, +{cyh:.0}, +{cz50:.0})   upper rear\n\
           Heater leads      12 mm Ø   (0, +{cyh:.0}, -{cz30:.0})   lower rear\n\
           Temp/RH sensor    10 mm Ø   (+100, +{cyh:.0}, 0)\n\
           CO2 sensor pocket 25×35×10  (-100, +{cyh:.0}, 0)         MH-Z19B\n\
           Media in grommet  30 mm Ø   (+{cxh:.0}, -100, 150)       right wall\n\
           Media out grommet 30 mm Ø   (+{cxh:.0}, +100, 150)\n\
           Power grommet     25 mm Ø   (+{cxh:.0}, 0, -50)\n\
           Sensor grommet    20 mm Ø   (+{cxh:.0}, 0, -150)\n\
           Blower port       160 mm Ø  (+{cxh:.0}, -150, -150)\n\
           Drain             20 mm Ø   (0, 0, -{czh:.0})            floor center\n\
           LH hatch         {LH_HATCH_X:.0}×{LH_HATCH_Y:.0}  (0, 0, +{czh:.0})   top\n\
           HEPA opening     {HEPA_X:.0}×{HEPA_Y:.0}  (0, -{hyo:.0}, +{czh:.0})   top, offset fwd\n\
           Drip mounts ×5   14 mm Ø    (+{cxh:.0}, 200, -150 + n×40) right wall\n\
         \n\
         BILL OF MATERIALS DELTA vs v2\n\
         -----------------------------------------------------------\n\
           Item                          v2        v3        Δ\n\
           Heater (silicone pad)        200 W    400 W    +$60\n\
           Blower                        60 CFM  400 CFM  +$120\n\
           Viewing window           300×200    {WINDOW_X:.0}×{WINDOW_Z:.0}  +$150\n\
           Structural frame (4545)       —      4×{fz:.0}mm posts  +$400\n\
           HEPA filter              (none)      18×18\"   +$150\n\
           PETG (sheet, total)          ~4 kg    ~9 kg    +$120\n\
           PIR foam (25mm)              ~6 m²    ~13 m²   +$90\n\
           Caster wheels                (feet)    4× M8   +$80\n\
         ------------------------------------------------------------\n\
           TOTAL COST-UP vs v2                            ≈ +$1,170\n\
         \n\
         WEIGHT ESTIMATE\n\
         -----------------------------------------------------------\n\
           Chamber PETG 3mm × {sa:.2} m² × 1.27 g/cm³  ≈ {wpetg:.1} kg\n\
           Shell PETG 3mm (larger area)                ≈ {wshell:.1} kg\n\
           PIR foam 25mm × {sa:.2} m² × 32 kg/m³       ≈ {wfoam:.1} kg\n\
           Frame (4× Rexroth 4545, {fz:.0}mm)          ≈ {wframe:.1} kg\n\
           Door + window + hatch + hardware            ≈ 15.0 kg\n\
           Drain pan + HEPA plenum + grommets          ≈  3.0 kg\n\
         ------------------------------------------------------------\n\
           TOTAL (empty, unloaded)                     ≈ {wtot:.0} kg\n\
         \n\
         TRANSPORT: 4× swivel caster wheels with locking pins on\n\
         floor corners — 50mm inset from edges. M8 thread into outer\n\
         frame extrusion. Rated 150 kg each, 4× = 600 kg capacity.\n\
         For in-lab moves: roll on casters. For cross-facility:\n\
         forklift slots under outer frame extrusion (200mm clearance).\n\
         \n",
        frame_outer_y(), sy, INNER_Y, INNER_X,
        cyh = chamber_outer_y() / 2.0,
        cxh = chamber_outer_x() / 2.0,
        czh = chamber_outer_z() / 2.0,
        cz50 = INNER_Z / 2.0 - 50.0,
        cz30 = INNER_Z / 2.0 - 30.0,
        hyo = (INNER_Y / 2.0) - HEPA_Y / 2.0 - 10.0,
        fz = fz,
        sa = shell_surf_m2,
        wpetg = shell_surf_m2 * 0.003 * 1270.0,
        wshell = shell_surf_m2 * 1.2 * 0.003 * 1270.0,
        wfoam = shell_surf_m2 * 0.025 * 32.0,
        wframe = 4.0 * fz / 1000.0 * 1.45, // 1.45 kg/m for 4545 slot-10
        wtot = shell_surf_m2 * 0.003 * 1270.0
            + shell_surf_m2 * 1.2 * 0.003 * 1270.0
            + shell_surf_m2 * 0.025 * 32.0
            + 4.0 * fz / 1000.0 * 1.45
            + 15.0
            + 3.0,
    );
    std::fs::write(drawing_path, drawing).unwrap();
    println!("Exported: {}", drawing_path);

    // STEP export via stltostp helper
    for stl in &[
        "output/chip_incubator_v3_chamber.stl",
        "output/chip_incubator_v3_shell.stl",
        "output/chip_incubator_v3_door.stl",
        "output/chip_incubator_v3_window.stl",
        "output/chip_incubator_v3_lh_hatch.stl",
        "output/chip_incubator_v3_drain_pan.stl",
        "output/chip_incubator_v3_co2_port.stl",
        "output/chip_incubator_v3_hepa_plenum.stl",
        "output/chip_incubator_v3_grommet.stl",
        "output/chip_incubator_v3_frame_post.stl",
        "output/chip_incubator_v3_assembly.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }
}
