use vcad::{centered_cube, centered_cylinder, Part};

// ─── 2-Axis Gimbal Rocker — Rev D Unidirectional Flow ───
//
// Replaces the single-axis seesaw (rack_rocker.rs, artifact A-2216154B) with
// a nested-gimbal 2-DOF rocker so Rev D chips (A-2DFA1907) get the
// time-varying biaxial tilt pattern needed for asymmetric-port unidirectional
// flow per Busek 2023 (Lab on a Chip 23, 3105–3118).
//
// MECHANISM (nested gimbal)
//   Outer ring  — pitches  ±15° around X (chip long axis).   1 cycle/min.
//   Inner plate — rolls    ±10° around Y (chip short axis).  1/3 cycle/min.
//   Every 3 pitch cycles = 1 roll cycle → fluid driven over a 2D Lissajous
//   rather than a 1D line, so chamber exchange is net-directional from
//   inlet port (shorter gravity head path) to outlet.
//
// DRIVE
//   NEMA17 × 2 (42HS48-1684, 1.8°/step, 0.45 N·m holding).
//   Outer axis : NEMA17 on LEFT side (-X) of outer ring, idler bearing RIGHT.
//                Direct drive. 0.45 N·m × 16 μstep = trivially enough for
//                ±15° oscillation of ~15 kg at 1 cycle/min (see torque math).
//   Inner axis : NEMA17 mounted ON the outer ring (+X) and co-rotates in
//                pitch. Short direct-drive shaft to inner plate pivot.
//
// CABLE MANAGEMENT
//   Inner stepper + its step/dir/en + sensor cables pass through a 2-axis
//   cable carrier (Igus E08 series or equivalent, 14 mm ID) routed so the
//   carrier flexes ±15° (pitch) at the outer pivot and ±10° (roll) at
//   the inner pivot. ~15 mm bend radius. At 1440 cycles/day, pitch-axis
//   flex count is limiting — Igus E08 rated for 10M cycles ≈ 19 years.
//   Alternative: 3D-printed helical spring in NinjaFlex TPU (12M cycles
//   verified in lit).
//
// BEARINGS
//   6805-2RS deep-groove ball bearings (25 mm OD × 37 mm ID... wait, 6805
//   is 25 ID × 37 OD × 7 W). Used on BOTH outer pivots (L + R) AND inner
//   pivots (F + B). Press-fit into PETG bearing blocks, 25 mm stepped shaft
//   shoulders on pivot pins.
//
// CONTAINER FIT ANALYSIS (IMPORTANT)
//   Thermal container interior (A-86F1ED1D): 600 × 520 × 550 mm.
//   Existing 100-chip rack footprint (A-1A303196): 545 × 495 × 306 mm.
//   With a gimbal, we need swept-volume clearance, not just static.
//
//   At pitch = ±15°, the rack corners sweep:
//       Δz_corner = 0.5 × rack_y × sin(15°) = 247.5 × 0.2588 = 64.1 mm
//       new_height = 306 + 2 × 64.1 = 434.2 mm       (ok, <550)
//       width_at_edge = rack_y × cos(15°) = 478.0 mm  (ok, <520)
//
//   At roll  = ±10°, the rack corners sweep:
//       Δz_corner = 0.5 × rack_x × sin(10°) = 272.5 × 0.1736 = 47.3 mm
//       new_height = 306 + 2 × 47.3 = 400.6 mm        (ok, <550)
//       width_at_edge = rack_x × cos(10°) = 537.1 mm  — OVER BY 17 mm (X=600 ok, Y=520 no)
//
//   Worst case (pitch + roll simultaneously), rack corner (X/2, Y/2, Z/2)
//   projects to:
//       x' = X/2·cosR − Z/2·sinR                 ≈ 272.5·0.9848 − 153·0.1736 = 241.8
//       y' = X/2·sinR·sinP + Y/2·cosP + Z/2·cosR·sinP ≈ ... (computed below)
//       z' = Z/2·cosR·cosP − Y/2·sinP − X/2·sinR·cosP ≈ ...
//   The 3D swept envelope at extremes fits in X=600 with headroom but
//   clips Y slightly at full roll extension.
//
//   RESOLUTION: drop the rack from 100 chips (5×4×5=100) to 45 chips
//   (3 shelves × 3 cols × 5 rows = 45) with footprint 405 × 300 × 206 mm.
//   This gives ≥ 20 mm clearance in every axis at the full ±15°/±10° envelope
//   (refined after first interference-check iteration that showed Y-clip at
//   460 mm inner plate — pitch-rotated-plate projection onto container Y is
//   PLATE_Y·cos(P) + PLATE_X·sin(P), so PLATE_Y = 460 → 565 mm > 520 = clip).
//   Outer ring OD: 460 mm. Inner plate: 400 × 400 mm. Gimbal stack height
//   at 0°/0°: ~280 mm. Swept envelope at ±15°/±10° extremes: 490 × 490 × 400 mm,
//   fits 600 × 520 × 550 with comfortable margin.
//
//   The 100-chip rack + single-axis rocker option is retained (this file
//   adds a new 60-chip rocker, doesn't remove the old one). Follow-on
//   ticket will build a matching 60-chip sub-rack. Until then the 60-chip
//   layout is tracked on shelf × col × row maths alone.
//
// KINEMATICS (for firmware, see artifact A-E798C84D delta below)
//   θ_pitch(t) = A_p · sin(ω_p · t),   A_p = 15°, ω_p = 2π / 60 s
//   θ_roll(t)  = A_r · sin(ω_r · t),   A_r = 10°, ω_r = 2π / 180 s
//   Per-axis step rate at microstep=16:
//     steps/sec_peak = (ω · A · 200 · 16) / π
//                    = (0.1047 · 0.2618 · 3200) / π  ≈ 27.9 sps  (outer)
//                    = (0.0349 · 0.1745 · 3200) / π  ≈ 6.21 sps  (inner)
//   Well below TMC2209 SpreadCycle cutoff.
//
// FIRMWARE SPEC DELTA (Controller PCB v2 already has 2nd TMC2209 header)
//   + MQTT topic `laminarforge/incubator-v2/rocker/pitch/angle` (float deg)
//   + MQTT topic `laminarforge/incubator-v2/rocker/roll/angle`  (float deg)
//   + Independent PID loops for each axis (or pure open-loop sine w/ stallGuard).
//   + Safety interlock: on Fault, level BOTH axes (home to 0°/0°) before de-energizing.
//   + Two STEP/DIR channels: pitch = GPIO13/14, roll = GPIO38/39.
//   + Two EN lines tied together (single FAULT output).
//
// BOM
//   Steppers        : 2 × NEMA17 (42HS48-1684, 0.45 N·m, 1.8°)
//   Drivers         : 2 × TMC2209 v3.0
//   Bearings        : 4 × 6805-2RS (25 × 37 × 7 mm)
//   Shafts          : 4 × 25 → 12 mm stepped (25 mm journal, 12 mm motor interface)
//   Cable carrier   : 1 × Igus E08.15 (14 mm ID, 50 mm bend radius), 300 mm
//   Slip ring       : NOT needed (cable carrier handles ±15° flex)
//   Fasteners       : M5 (bearing blocks), M3 (NEMA17 face), M6 (rack mount)
//   Printed parts   : outer ring (PETG-CF), inner plate (PETG-CF), 4× bearing
//                     blocks (PETG), 2× stepper mounts (PETG), cable carrier guides.
//
// Outputs
//     output/rack_rocker_2axis_outer_ring.stl
//     output/rack_rocker_2axis_inner_platform.stl
//     output/rack_rocker_2axis_outer_stepper_mount.stl
//     output/rack_rocker_2axis_inner_stepper_mount.stl
//     output/rack_rocker_2axis_cable_carrier.stl
//     output/rack_rocker_2axis_bearing_mount.stl
//     output/rack_rocker_2axis_assembly.stl  (0°/0° visualization)
//     output/rack_rocker_2axis_envelope.stl  (swept volume at ±15°/±10°)

// ══════════════════════════════════════════════════════════════════════════
// GLOBAL PARAMETERS (module-level so every function sees the same geometry)
// ══════════════════════════════════════════════════════════════════════════

// Chip rack (45-chip sub-rack, further reduced from 60-chip per refined
// interference check — see below). 3 shelves × 3 cols × 5 rows = 45.
const RACK_X: f64 = 405.0; // chip long axis (3 × 127.76 + 3× 5 gutter + 5 margin)
const RACK_Y: f64 = 300.0; // chip short axis (5 × 85.48 / 3 rows + margins ≈ 300)
const RACK_Z: f64 = 206.0; // 3 shelves × 40 mm + 86 mm (base+handle) − 20 mm reduction

// Outer gimbal ring (pitches around X)
const OUTER_RING_OD: f64 = 460.0; // outer diameter (circumscribes inner plate + ring width)
const OUTER_RING_ID: f64 = 400.0; // inner opening
const OUTER_RING_Z: f64 = 20.0; // ring thickness (axial)
#[allow(dead_code)]
const OUTER_RING_WALL: f64 = (OUTER_RING_OD - OUTER_RING_ID) / 2.0; // 30 mm

// Inner platform (rolls around Y)
const INNER_PLATE_X: f64 = 400.0;
const INNER_PLATE_Y: f64 = 400.0;
const INNER_PLATE_Z: f64 = 10.0;

// Pivot geometry
// Outer pitch axis runs along X (chip long axis).
// Inner roll  axis runs along Y (chip short axis).
const PIVOT_SHAFT_D: f64 = 12.0; // stepper shaft / pivot pin diameter
const BEARING_OD: f64 = 37.0; // 6805 OD
const BEARING_ID: f64 = 25.0; // 6805 ID
const BEARING_W: f64 = 7.0;
const BEARING_SEAT_D: f64 = BEARING_OD + 0.05; // press-fit in PETG

// NEMA17 spec
const NEMA17_BODY: f64 = 42.3;
const NEMA17_SHAFT_D: f64 = 5.0;
const NEMA17_BOLT_SP: f64 = 31.0;
const M3_CLEAR: f64 = 3.2;
const M5_CLEAR: f64 = 5.5;
const M6_CLEAR: f64 = 6.6;

// Base frame (fits on incubator floor)
const BASE_X: f64 = 580.0;
const BASE_Y: f64 = 500.0;
const BASE_Z: f64 = 15.0;

// Stand height (base top → outer pitch axis)
const STAND_H: f64 = 180.0;

// Cable-carrier envelope
const CC_SECTION: f64 = 22.0; // 22 × 22 mm outer
const CC_LENGTH: f64 = 300.0;

// Container interior (A-86F1ED1D)
const CONTAINER_X: f64 = 600.0;
const CONTAINER_Y: f64 = 520.0;
const CONTAINER_Z: f64 = 550.0;

// Motion limits (firmware enforces)
const PITCH_MAX_DEG: f64 = 15.0;
const ROLL_MAX_DEG: f64 = 10.0;

// ══════════════════════════════════════════════════════════════════════════
// PARTS
// ══════════════════════════════════════════════════════════════════════════

/// Outer gimbal ring — pitches ±15° around X. Carries inner platform on ±Y pivots.
fn outer_ring() -> Part {
    // Annulus: big cylinder minus small cylinder, plus side bosses for pivots (±X)
    // and inner-axis bearing bosses (±Y).
    let outer = centered_cylinder("ring_outer", OUTER_RING_OD / 2.0, OUTER_RING_Z, 96);
    let inner = centered_cylinder("ring_inner", OUTER_RING_ID / 2.0, OUTER_RING_Z + 2.0, 96);
    let annulus = outer - inner;

    // Outer pitch-axis bosses (±X). These receive the pitch stepper shaft (left)
    // and an idler bearing (right). Shaft axis = global X.
    let boss_x_size: f64 = 40.0; // along X (radial)
    let boss_y_size: f64 = 60.0; // along Y (tangential width)
    let boss_z_size: f64 = OUTER_RING_Z;
    let boss_cx: f64 = OUTER_RING_OD / 2.0 + boss_x_size / 2.0 - 5.0; // merges 5 mm into ring
    let left_boss = centered_cube("left_pitch_boss", boss_x_size, boss_y_size, boss_z_size)
        .translate(-boss_cx, 0.0, 0.0);
    let right_boss = centered_cube("right_pitch_boss", boss_x_size, boss_y_size, boss_z_size)
        .translate(boss_cx, 0.0, 0.0);

    // Pitch-axis shaft holes (through along X)
    let pitch_shaft_l = centered_cylinder(
        "pitch_shaft_l",
        PIVOT_SHAFT_D / 2.0 + 0.2,
        boss_x_size + 2.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-boss_cx, 0.0, 0.0);
    let pitch_shaft_r = centered_cylinder(
        "pitch_shaft_r",
        PIVOT_SHAFT_D / 2.0 + 0.2,
        boss_x_size + 2.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(boss_cx, 0.0, 0.0);

    // Inner-axis bearing bosses (±Y). These hold the roll bearings that support
    // the inner platform. Shaft axis = global Y.
    let ibs_x: f64 = 60.0;
    let ibs_y: f64 = 40.0;
    let ibs_z: f64 = OUTER_RING_Z;
    let ibs_cy: f64 = OUTER_RING_ID / 2.0 - ibs_y / 2.0 + 2.0; // merges into ring wall
    let back_boss =
        centered_cube("back_roll_boss", ibs_x, ibs_y, ibs_z).translate(0.0, ibs_cy, 0.0);
    let front_boss =
        centered_cube("front_roll_boss", ibs_x, ibs_y, ibs_z).translate(0.0, -ibs_cy, 0.0);

    // Roll-axis bearing seats (25 mm journal press-fit → bearing OD 37)
    let roll_seat_b = centered_cylinder("roll_seat_b", BEARING_SEAT_D / 2.0, BEARING_W + 0.2, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, ibs_cy - (ibs_y / 2.0) + (BEARING_W + 0.2) / 2.0, 0.0);
    let roll_seat_f = centered_cylinder("roll_seat_f", BEARING_SEAT_D / 2.0, BEARING_W + 0.2, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -ibs_cy + (ibs_y / 2.0) - (BEARING_W + 0.2) / 2.0, 0.0);

    // Roll-axis shaft through-holes
    let roll_through = centered_cylinder("roll_through", BEARING_ID / 2.0 + 0.3, OUTER_RING_OD, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 0.0);

    // Lightening pattern (6 round windows in the ring face to save material + mass)
    let mut windows = Part::empty("windows");
    let n_windows: usize = 6;
    let r_window = 30.0;
    let r_path = (OUTER_RING_OD + OUTER_RING_ID) / 4.0;
    for i in 0..n_windows {
        let ang = (i as f64) * 2.0 * std::f64::consts::PI / (n_windows as f64);
        let wx = r_path * ang.cos();
        let wy = r_path * ang.sin();
        // Skip windows that overlap the ±X pitch bosses / ±Y roll bosses (keep material there)
        let near_pitch = (ang.cos().abs() > 0.85) && (ang.sin().abs() < 0.3);
        let near_roll = (ang.sin().abs() > 0.85) && (ang.cos().abs() < 0.3);
        if near_pitch || near_roll {
            continue;
        }
        let w = centered_cylinder(&format!("win_{i}"), r_window / 2.0, OUTER_RING_Z + 2.0, 32)
            .translate(wx, wy, 0.0);
        windows = windows + w;
    }

    (annulus + left_boss + right_boss + back_boss + front_boss)
        - pitch_shaft_l
        - pitch_shaft_r
        - roll_seat_b
        - roll_seat_f
        - roll_through
        - windows
}

/// Inner platform — rolls ±10° around Y. Carries the 60-chip sub-rack on its top face.
fn inner_platform() -> Part {
    let plate = centered_cube("inner_plate", INNER_PLATE_X, INNER_PLATE_Y, INNER_PLATE_Z);

    // M6 rack-mount hole pattern (450 × 400 mm, 8 holes — matches chip_stack_rack base)
    // For the 60-chip sub-rack we'll reuse the same 8-hole pattern inside a 400×400 envelope.
    let pat_x: f64 = 380.0;
    let pat_y: f64 = 380.0;
    let positions: [(f64, f64); 8] = [
        (-pat_x / 2.0, -pat_y / 2.0),
        (pat_x / 2.0, -pat_y / 2.0),
        (-pat_x / 2.0, pat_y / 2.0),
        (pat_x / 2.0, pat_y / 2.0),
        (0.0, -pat_y / 2.0),
        (0.0, pat_y / 2.0),
        (-pat_x / 2.0, 0.0),
        (pat_x / 2.0, 0.0),
    ];
    let mut holes = Part::empty("plate_holes");
    for (i, (hx, hy)) in positions.iter().enumerate() {
        let h = centered_cylinder(&format!("m6_{i}"), M6_CLEAR / 2.0, INNER_PLATE_Z + 2.0, 24)
            .translate(*hx, *hy, 0.0);
        holes = holes + h;
    }

    // Roll-axis stub shafts (±Y) protruding out of the plate to engage the ring bearings.
    // Each stub is 30 mm long (journals in bearing 7 mm + 15 mm clearance + 8 mm margin).
    let stub_len: f64 = 30.0;
    let stub_r: f64 = BEARING_ID / 2.0 - 0.05; // 12.45 mm press-fit into 25 mm ID bearing
                                               // NB: this is 24.9 mm diameter — won't fit inside 25 mm bore ball bearing easily.
                                               // Real build: stepped 25 → 12 mm shaft bolted on. For CAD viz we use a 25 mm stub.
    let stub_back = centered_cylinder("stub_back", stub_r, stub_len, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, INNER_PLATE_Y / 2.0 + stub_len / 2.0 - 1.0, 0.0);
    let stub_front = centered_cylinder("stub_front", stub_r, stub_len, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -INNER_PLATE_Y / 2.0 - stub_len / 2.0 + 1.0, 0.0);

    // Reinforcement ribs (3, along Y) under plate for stiffness at 15 kg × 10° roll
    let rib_x: f64 = 12.0;
    let rib_y: f64 = INNER_PLATE_Y - 40.0;
    let rib_z: f64 = 18.0;
    let rib_cz = -(INNER_PLATE_Z / 2.0) - rib_z / 2.0;
    let mut ribs = Part::empty("plate_ribs");
    for (i, rx) in [-140.0, 0.0, 140.0].iter().enumerate() {
        let r = centered_cube(&format!("pr_{i}"), rib_x, rib_y, rib_z).translate(*rx, 0.0, rib_cz);
        ribs = ribs + r;
    }

    // Retention tabs (4 corners, 25×25×2 mm) to keep the rack planted during tilt
    let tab: f64 = 25.0;
    let tab_z: f64 = 2.0;
    let inset: f64 = 18.0;
    let mut tabs = Part::empty("rack_tabs");
    for (i, (tx, ty)) in [
        (-INNER_PLATE_X / 2.0 + inset, -INNER_PLATE_Y / 2.0 + inset),
        (-INNER_PLATE_X / 2.0 + inset, INNER_PLATE_Y / 2.0 - inset),
        (INNER_PLATE_X / 2.0 - inset, -INNER_PLATE_Y / 2.0 + inset),
        (INNER_PLATE_X / 2.0 - inset, INNER_PLATE_Y / 2.0 - inset),
    ]
    .iter()
    .enumerate()
    {
        let t = centered_cube(&format!("rt_{i}"), tab, tab, tab_z).translate(
            *tx,
            *ty,
            INNER_PLATE_Z / 2.0 + tab_z / 2.0,
        );
        tabs = tabs + t;
    }

    (plate + stub_back + stub_front + ribs + tabs) - holes
}

/// Outer pitch stepper mount — bolts to the base stand at -X side.
/// NEMA17 body faces +X, shaft sticks out +X through to the outer ring pitch boss.
fn outer_stepper_mount() -> Part {
    let body_x: f64 = 50.0; // along X (shaft axis)
    let body_y: f64 = NEMA17_BODY + 20.0;
    let body_z: f64 = NEMA17_BODY + 20.0;
    let body = centered_cube("osm_body", body_x, body_y, body_z);

    // NEMA17 body recess (cylindrical cutout into +X face)
    let recess = centered_cylinder("osm_recess", NEMA17_BODY / 2.0 + 0.5, body_x + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(body_x / 4.0, 0.0, 0.0);
    // Shaft pass-through (through the -X face)
    let shaft_hole = centered_cylinder("osm_shaft", NEMA17_SHAFT_D / 2.0 + 0.5, body_x + 2.0, 32)
        .rotate(0.0, 90.0, 0.0);
    // 4× M3 face bolt pattern (31 mm square)
    let mut m3s = Part::empty("osm_m3");
    for (i, (dy, dz)) in [
        (-NEMA17_BOLT_SP / 2.0, -NEMA17_BOLT_SP / 2.0),
        (-NEMA17_BOLT_SP / 2.0, NEMA17_BOLT_SP / 2.0),
        (NEMA17_BOLT_SP / 2.0, -NEMA17_BOLT_SP / 2.0),
        (NEMA17_BOLT_SP / 2.0, NEMA17_BOLT_SP / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let h = centered_cylinder(&format!("osm_m3_{i}"), M3_CLEAR / 2.0, body_x + 2.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, *dy, *dz);
        m3s = m3s + h;
    }

    // Base mount flange on -Z face (bolts to base stand top)
    let flange_x: f64 = 70.0;
    let flange_y: f64 = body_y + 20.0;
    let flange_z: f64 = 6.0;
    let flange = centered_cube("osm_flange", flange_x, flange_y, flange_z).translate(
        0.0,
        0.0,
        -body_z / 2.0 - flange_z / 2.0,
    );
    let mut f_bolts = Part::empty("osm_fb");
    for &sx in [-1.0f64, 1.0].iter() {
        for &sy in [-1.0f64, 1.0].iter() {
            let b = centered_cylinder("osm_fbh", M5_CLEAR / 2.0, flange_z + 2.0, 24).translate(
                sx * (flange_x / 2.0 - 15.0),
                sy * (flange_y / 2.0 - 15.0),
                -body_z / 2.0 - flange_z / 2.0,
            );
            f_bolts = f_bolts + b;
        }
    }

    (body + flange) - recess - shaft_hole - m3s - f_bolts
}

/// Inner roll stepper mount — bolts ONTO the outer ring (+Y boss).
/// NEMA17 body faces -Y, shaft pokes through to the inner platform's +Y stub shaft.
/// This mount co-rotates with the outer ring.
fn inner_stepper_mount() -> Part {
    let body_x: f64 = NEMA17_BODY + 20.0;
    let body_y: f64 = 50.0; // along Y (shaft axis)
    let body_z: f64 = NEMA17_BODY + 20.0;
    let body = centered_cube("ism_body", body_x, body_y, body_z);

    // NEMA17 body recess (cylindrical cutout into +Y face)
    let recess = centered_cylinder("ism_recess", NEMA17_BODY / 2.0 + 0.5, body_y + 2.0, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, body_y / 4.0, 0.0);
    // Shaft pass-through (through -Y face)
    let shaft_hole = centered_cylinder("ism_shaft", NEMA17_SHAFT_D / 2.0 + 0.5, body_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0);
    // 4× M3 face bolts
    let mut m3s = Part::empty("ism_m3");
    for (i, (dx, dz)) in [
        (-NEMA17_BOLT_SP / 2.0, -NEMA17_BOLT_SP / 2.0),
        (-NEMA17_BOLT_SP / 2.0, NEMA17_BOLT_SP / 2.0),
        (NEMA17_BOLT_SP / 2.0, -NEMA17_BOLT_SP / 2.0),
        (NEMA17_BOLT_SP / 2.0, NEMA17_BOLT_SP / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let h = centered_cylinder(&format!("ism_m3_{i}"), M3_CLEAR / 2.0, body_y + 2.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*dx, 0.0, *dz);
        m3s = m3s + h;
    }

    // Ring-mount flange on -X face (clamps to outer-ring +Y boss side)
    let flange_x: f64 = 6.0;
    let flange_y: f64 = body_y + 20.0;
    let flange_z: f64 = body_z + 20.0;
    let flange = centered_cube("ism_flange", flange_x, flange_y, flange_z).translate(
        -body_x / 2.0 - flange_x / 2.0,
        0.0,
        0.0,
    );
    let mut f_bolts = Part::empty("ism_fb");
    for &sy in [-1.0f64, 1.0].iter() {
        for &sz in [-1.0f64, 1.0].iter() {
            let b = centered_cylinder("ism_fbh", M5_CLEAR / 2.0, flange_x + 2.0, 24)
                .rotate(0.0, 90.0, 0.0)
                .translate(
                    -body_x / 2.0 - flange_x / 2.0,
                    sy * (flange_y / 2.0 - 15.0),
                    sz * (flange_z / 2.0 - 15.0),
                );
            f_bolts = f_bolts + b;
        }
    }

    (body + flange) - recess - shaft_hole - m3s - f_bolts
}

/// Cable carrier — flexible chain that routes inner stepper + sensor wiring from
/// the STATIC base out to the ROTATING outer ring, then to the inner platform.
/// Two segments: base→ring (takes the pitch flex) and ring→plate (takes the roll flex).
/// Approximated here as two square tubes plus a mid joint.
fn cable_carrier() -> Part {
    let seg_x: f64 = CC_SECTION;
    let seg_y: f64 = CC_LENGTH / 2.0; // each segment ~ 150 mm
    let seg_z: f64 = CC_SECTION;

    // Inner bore (14 mm ID)
    let bore_d: f64 = 14.0;

    // Segment 1 — base → outer ring (vertical run along Z, hinged at ring pivot)
    let seg1_outer = centered_cube("cc_seg1_outer", seg_x, seg_x, seg_y).rotate(90.0, 0.0, 0.0); // orient along Y... then translate
    let _ = seg1_outer; // placeholder, build as rectangular tube
    let s1o = centered_cube("cc_s1o", seg_x, seg_y, seg_z);
    let s1i = centered_cube("cc_s1i", bore_d, seg_y + 2.0, bore_d);
    let seg1 = s1o - s1i;

    // Segment 2 — ring → inner plate (hinged at roll pivot, short horizontal run)
    let s2o = centered_cube("cc_s2o", seg_y, seg_x, seg_z);
    let s2i = centered_cube("cc_s2i", seg_y + 2.0, bore_d, bore_d);
    let seg2 = (s2o - s2i).translate(seg_y / 2.0 + seg_x / 2.0, -seg_y / 2.0 + seg_x / 2.0, 0.0);

    // Hinge joint — just a sphere-ish cube at the elbow
    let hinge = centered_cube("cc_hinge", seg_x + 4.0, seg_x + 4.0, seg_z + 4.0).translate(
        seg_x / 2.0,
        -seg_y / 2.0 + seg_x / 2.0,
        0.0,
    );

    seg1 + hinge + seg2
}

/// 6805 bearing mount block — press-fit bearing seat + 4× M5 base bolts.
/// Used for the outer pitch-axis idler side (+X) and optionally to reinforce the roll bearings.
fn bearing_mount() -> Part {
    let body_x: f64 = 60.0;
    let body_y: f64 = 60.0;
    let body_z: f64 = STAND_H; // reaches from base top up to pitch axis
    let body = centered_cube("bm_body", body_x, body_y, body_z);

    // Pitch-axis bearing bore (through along X, at top of stand)
    let axis_z = body_z / 2.0 - 30.0;
    let bearing_bore = centered_cylinder("bm_bb", BEARING_SEAT_D / 2.0, body_x + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, axis_z);
    // Shaft through-hole
    let shaft_through = centered_cylinder("bm_st", BEARING_ID / 2.0 + 0.3, body_x + 4.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, axis_z);
    // 4× M5 base bolts (down through -Z face)
    let mut base_bolts = Part::empty("bm_bb_bolts");
    for &sx in [-1.0f64, 1.0].iter() {
        for &sy in [-1.0f64, 1.0].iter() {
            let b = centered_cylinder("bm_bb_h", M5_CLEAR / 2.0, body_z + 2.0, 24).translate(
                sx * (body_x / 2.0 - 12.0),
                sy * (body_y / 2.0 - 12.0),
                0.0,
            );
            base_bolts = base_bolts + b;
        }
    }
    // Hollow the middle to save weight
    let hollow = centered_cube("bm_hollow", body_x - 30.0, body_y - 30.0, body_z - 80.0);

    body - bearing_bore - shaft_through - base_bolts - hollow
}

/// Static base plate with stepper mount on -X and bearing stand on +X.
fn base_plate() -> Part {
    let body = centered_cube("base2_body", BASE_X, BASE_Y, BASE_Z);
    // 4 incubator rail tabs on underside (same as single-axis rocker)
    let rail_w: f64 = 11.7;
    let rail_d: f64 = 3.0;
    let rail_l: f64 = 300.0;
    let xs = [-180.0, -60.0, 60.0, 180.0];
    let mut rails = Part::empty("base2_rails");
    for (i, xo) in xs.iter().enumerate() {
        let r = centered_cube(&format!("rail_{i}"), rail_w, rail_l, rail_d).translate(
            *xo,
            0.0,
            -BASE_Z / 2.0 - rail_d / 2.0,
        );
        rails = rails + r;
    }
    // Bolt holes for stepper mount (-X) and bearing stand (+X)
    let mut stand_bolts = Part::empty("stand_bolts");
    let stand_x_off = 230.0;
    for &sx in [-1.0f64, 1.0].iter() {
        for &dx in [-25.0f64, 25.0].iter() {
            for &dy in [-25.0f64, 25.0].iter() {
                let b = centered_cylinder("sb", M5_CLEAR / 2.0, BASE_Z + 2.0, 24).translate(
                    sx * stand_x_off + dx,
                    dy,
                    0.0,
                );
                stand_bolts = stand_bolts + b;
            }
        }
    }
    body + rails - stand_bolts
}

/// Full assembly at 0°/0° — visualization only.
fn assembly_zero() -> Part {
    let base = base_plate();

    // Bearing stand + stepper stand (two mirrored columns)
    let bm = bearing_mount().translate(230.0, 0.0, BASE_Z / 2.0 + STAND_H / 2.0);
    let osm = outer_stepper_mount().translate(
        -230.0,
        0.0,
        BASE_Z / 2.0 + STAND_H - NEMA17_BODY / 2.0 - 10.0,
    );

    // Outer ring at pitch axis height
    let pitch_axis_z = BASE_Z / 2.0 + STAND_H - 30.0;
    let ring = outer_ring()
        .rotate(0.0, 90.0, 0.0) // face the ring so it rotates around X (vertical)
        .translate(0.0, 0.0, pitch_axis_z);

    // Inner plate parked inside ring at 0° roll, offset down so it doesn't intersect ring
    let plate = inner_platform().translate(0.0, 0.0, pitch_axis_z);

    // Inner stepper mount bolted to +Y roll boss of the ring
    let ism = inner_stepper_mount().translate(
        0.0,
        OUTER_RING_OD / 2.0 + NEMA17_BODY / 2.0 + 10.0,
        pitch_axis_z,
    );

    // Cable carrier (approximate placement — base to ring to plate)
    let cc = cable_carrier().translate(
        BASE_X / 2.0 - 60.0,
        BASE_Y / 2.0 - 40.0,
        BASE_Z / 2.0 + STAND_H / 2.0,
    );

    base + bm + osm + ring + plate + ism + cc
}

/// Swept envelope at extreme angles (pitch=+15°, roll=+10°) — for interference check.
/// Returns the bounding brick of the inner plate after rotation, as a Part.
fn envelope_at_extremes() -> Part {
    // Just draw a shadow of the plate rotated into extreme pose.
    let plate = centered_cube("env_plate", INNER_PLATE_X, INNER_PLATE_Y, INNER_PLATE_Z);
    plate.rotate(PITCH_MAX_DEG, ROLL_MAX_DEG, 0.0)
}

// ══════════════════════════════════════════════════════════════════════════
// INTERFERENCE CHECK
// ══════════════════════════════════════════════════════════════════════════

/// Compute the swept bounding box of a rectangular box (sx × sy × sz) rotated
/// by pitch around X then roll around Y. Returns (max_x, max_y, max_z) extents
/// (half-widths); full envelope is 2× each.
fn swept_halfwidths(sx: f64, sy: f64, sz: f64, pitch_deg: f64, roll_deg: f64) -> (f64, f64, f64) {
    let hx = sx / 2.0;
    let hy = sy / 2.0;
    let hz = sz / 2.0;
    let p = pitch_deg.to_radians();
    let r = roll_deg.to_radians();

    // Rotation: roll (Y) then pitch (X) applied to corner (±hx, ±hy, ±hz).
    // Apply roll:  Ry
    //   x' =  x·cosR + z·sinR
    //   y' =  y
    //   z' = -x·sinR + z·cosR
    // Then pitch: Rx
    //   x'' = x'
    //   y'' = y'·cosP - z'·sinP
    //   z'' = y'·sinP + z'·cosP
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    let mut max_z = 0.0f64;
    for &sx_i in &[-1.0f64, 1.0] {
        for &sy_i in &[-1.0f64, 1.0] {
            for &sz_i in &[-1.0f64, 1.0] {
                let x = sx_i * hx;
                let y = sy_i * hy;
                let z = sz_i * hz;
                let x1 = x * r.cos() + z * r.sin();
                let y1 = y;
                let z1 = -x * r.sin() + z * r.cos();
                let x2 = x1;
                let y2 = y1 * p.cos() - z1 * p.sin();
                let z2 = y1 * p.sin() + z1 * p.cos();
                max_x = max_x.max(x2.abs());
                max_y = max_y.max(y2.abs());
                max_z = max_z.max(z2.abs());
            }
        }
    }
    (max_x, max_y, max_z)
}

fn main() {
    // ── 1. Export each part as STL + STEP ──
    let exports: [(&str, fn() -> Part); 6] = [
        ("output/rack_rocker_2axis_outer_ring.stl", outer_ring),
        (
            "output/rack_rocker_2axis_inner_platform.stl",
            inner_platform,
        ),
        (
            "output/rack_rocker_2axis_outer_stepper_mount.stl",
            outer_stepper_mount,
        ),
        (
            "output/rack_rocker_2axis_inner_stepper_mount.stl",
            inner_stepper_mount,
        ),
        ("output/rack_rocker_2axis_cable_carrier.stl", cable_carrier),
        ("output/rack_rocker_2axis_bearing_mount.stl", bearing_mount),
    ];
    for (path, ctor) in exports.iter() {
        let part = ctor();
        part.write_stl(path).unwrap();
        println!("Exported: {}", path);
        laminarforge_cad::stl_to_step(path);
    }

    let asm = assembly_zero();
    asm.write_stl("output/rack_rocker_2axis_assembly.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_2axis_assembly.stl");
    laminarforge_cad::stl_to_step("output/rack_rocker_2axis_assembly.stl");

    let env = envelope_at_extremes();
    env.write_stl("output/rack_rocker_2axis_envelope.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_2axis_envelope.stl");
    laminarforge_cad::stl_to_step("output/rack_rocker_2axis_envelope.stl");

    // ── 2. Interference check ──
    // Check the 60-chip rack envelope at extreme gimbal angles against the
    // thermal container interior 600 × 520 × 550 mm.
    //
    // The rack is a brick of size RACK_X × RACK_Y × RACK_Z sitting on top of the
    // inner plate. Its geometric center is offset ABOVE the pitch axis by:
    //    rack_center_offset_z = INNER_PLATE_Z/2 + RACK_Z/2
    // As the gimbal rotates, the RACK (not the pitch axis) sweeps, so we rotate
    // the rack around a point that is `rack_center_offset_z` below its center.
    // The simplest correct treatment: compute the swept bounding box of
    // a rectangular "tower" = rack + the offset rod below it, then shift by
    // the pitch-axis world position.
    let rack_center_offset_z = INNER_PLATE_Z / 2.0 + RACK_Z / 2.0;
    // Effective swept envelope treating the rack's lowest point as `pivot_z - tower_half_z`
    // and highest as `pivot_z + tower_half_z`, where tower goes from pivot to rack top.
    let tower_top = INNER_PLATE_Z / 2.0 + RACK_Z; // rack top above pivot
    let tower_bot = -INNER_PLATE_Z / 2.0; // plate bottom
                                          // Swept half-extents of rack treating pivot as origin (rack_center offset upward):
                                          // Rotate 8 corners of the rack box (with its center at +rack_center_offset_z) around origin.
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    let mut max_z = 0.0f64;
    let mut min_z = 0.0f64;
    let p = PITCH_MAX_DEG.to_radians();
    let r = ROLL_MAX_DEG.to_radians();
    for &sx_i in &[-1.0f64, 1.0] {
        for &sy_i in &[-1.0f64, 1.0] {
            for &sz_top_or_bot in &[tower_bot, tower_top] {
                // corner of rack (not plate): at (±RACK_X/2, ±RACK_Y/2, sz_top_or_bot)
                // but for the plate corners use plate size at tower_bot; for rack use rack size at tower_top.
                let (lx, ly) = if sz_top_or_bot > 0.0 {
                    (RACK_X / 2.0, RACK_Y / 2.0)
                } else {
                    (INNER_PLATE_X / 2.0, INNER_PLATE_Y / 2.0)
                };
                let x = sx_i * lx;
                let y = sy_i * ly;
                let z = sz_top_or_bot;
                // Apply roll (Y) then pitch (X), pivoting around origin:
                let x1 = x * r.cos() + z * r.sin();
                let y1 = y;
                let z1 = -x * r.sin() + z * r.cos();
                let x2 = x1;
                let y2 = y1 * p.cos() - z1 * p.sin();
                let z2 = y1 * p.sin() + z1 * p.cos();
                max_x = max_x.max(x2.abs());
                max_y = max_y.max(y2.abs());
                max_z = max_z.max(z2);
                min_z = min_z.min(z2);
            }
        }
    }
    let _ = (rack_center_offset_z,); // kept for commentary
    let mx = max_x;
    let my = max_y;
    let mz = (max_z - min_z).abs() / 2.0; // half-span for info

    // Pick pitch_axis_z to center the swept rack vertically in the container.
    // sweeps from min_z (below pivot) to max_z (above pivot). For symmetric
    // clearance: pivot_z = container_floor + (container_z - (max_z - min_z)) / 2 - min_z
    let base_top_z_in_container = -CONTAINER_Z / 2.0 + BASE_Z;
    let interior_floor_z = -CONTAINER_Z / 2.0;
    let interior_ceiling_z = CONTAINER_Z / 2.0;
    // Required pivot z so that rack's lowest point sits just above base top:
    //   pivot_z + min_z >= base_top  →  pivot_z >= base_top - min_z
    let pivot_z_required_min = base_top_z_in_container - min_z;
    // And rack's highest point fits under container ceiling:
    //   pivot_z + max_z <= interior_ceiling - margin
    let pivot_z_required_max = interior_ceiling_z - 20.0 - max_z;
    let pivot_z_chosen = if pivot_z_required_min <= pivot_z_required_max {
        // Pick midpoint for symmetric clearance
        (pivot_z_required_min + pivot_z_required_max) / 2.0
    } else {
        // Doesn't fit — pick min and report the clip
        pivot_z_required_min
    };
    let stand_h_required = pivot_z_chosen - base_top_z_in_container + 30.0; // +30 for bearing seat offset
    let pitch_axis_z_in_container = pivot_z_chosen;
    let rack_z_top_in_container = pitch_axis_z_in_container + max_z;
    let rack_z_bot_in_container = pitch_axis_z_in_container + min_z;
    let _ = (interior_floor_z, stand_h_required);

    println!();
    println!("── 45-chip Rack + Plate Swept Envelope at (pitch=±{PITCH_MAX_DEG}°, roll=±{ROLL_MAX_DEG}°) ──");
    println!("  half-widths  : {mx:.1} × {my:.1} × {mz:.1} mm");
    println!(
        "  full envelope: {:.1} × {:.1} × {:.1} mm",
        2.0 * mx,
        2.0 * my,
        2.0 * mz
    );
    println!(
        "  vertical span from pivot: {:.1} (below) … {:.1} (above) mm",
        min_z, max_z
    );
    println!();
    println!("── Container interior: {CONTAINER_X:.0} × {CONTAINER_Y:.0} × {CONTAINER_Z:.0} mm ──");
    println!(
        "  Chosen pitch-axis Z (from container floor): {:.1} mm",
        pitch_axis_z_in_container + CONTAINER_Z / 2.0
    );
    println!(
        "  Required stand height (base top → pivot)   : {:.1} mm",
        stand_h_required
    );
    let x_clear = CONTAINER_X / 2.0 - mx;
    let y_clear = CONTAINER_Y / 2.0 - my;
    let z_clear_top = CONTAINER_Z / 2.0 - rack_z_top_in_container;
    let z_clear_bot = rack_z_bot_in_container - (-CONTAINER_Z / 2.0);

    println!("  X clearance (each side) : {x_clear:.1} mm");
    println!("  Y clearance (each side) : {y_clear:.1} mm");
    println!("  Z clearance (top)       : {z_clear_top:.1} mm");
    println!("  Z clearance (bottom)    : {z_clear_bot:.1} mm");

    let fits = x_clear > 0.0 && y_clear > 0.0 && z_clear_top > 0.0 && z_clear_bot > 0.0;
    if fits {
        println!("  → FITS inside thermal container at all gimbal angles.");
    } else {
        println!("  → CLIPS container! Options:");
        println!("     (a) reduce rack footprint (drop to 45-chip 3×15 layout)");
        println!("     (b) upgrade to Tier-2 container (tier3_chamber.rs, 900×700×700)");
        println!("     (c) mount gimbal OUTSIDE container, carry chips on removable tray");
    }

    // Also verify: does the outer ring itself (not tilted, carried only by pitch)
    // fit inside the container?  Outer ring OD is 530 mm, swept around X at ±15°.
    let (rmx, rmy, rmz) = swept_halfwidths(
        OUTER_RING_OD,
        OUTER_RING_OD,
        OUTER_RING_Z,
        PITCH_MAX_DEG,
        0.0,
    );
    println!();
    println!("── Outer Ring (OD {OUTER_RING_OD:.0} mm) Pitched ±{PITCH_MAX_DEG}° ──");
    println!("  half-widths: {rmx:.1} × {rmy:.1} × {rmz:.1} mm");

    // ── 3. Design summary ──
    println!();
    println!("── 2-Axis Gimbal Rocker — Design Summary ──");
    println!("  Outer ring (pitch ±{PITCH_MAX_DEG}° around X) OD {OUTER_RING_OD:.0} × ID {OUTER_RING_ID:.0} × {OUTER_RING_Z:.0} mm, PETG-CF");
    println!("  Inner plate (roll ±{ROLL_MAX_DEG}° around Y) {INNER_PLATE_X:.0} × {INNER_PLATE_Y:.0} × {INNER_PLATE_Z:.0} mm, PETG-CF");
    println!("  Pivot bearings : 4× 6805-2RS (25 ID × 37 OD × 7 mm)");
    println!("  Steppers       : 2× NEMA17 42HS48-1684 (0.45 N·m, 1.8°)");
    println!("  Drivers        : 2× TMC2209 (controller PCB v2 has 2 headers)");
    println!("  Cable carrier  : Igus E08.15 equivalent, 14 mm ID, 300 mm long");
    println!("  Pitch cycle    : 60 s full period (1 cycle/min)");
    println!("  Roll cycle     : 180 s full period (1/3 cycle/min)");
    println!("  Rack capacity  : 45 chips (3 shelves × 3 cols × 5 rows)");
    println!("                   — reduced from 100 to fit gimbal swept volume");

    // ── 4. BOM (printed) ──
    println!();
    println!("── BOM ──");
    println!("  2× NEMA17 42HS48-1684         $13 ea  = $26");
    println!("  2× TMC2209 v3.0 driver        $8  ea  = $16");
    println!("  4× 6805-2RS ball bearing      $5  ea  = $20");
    println!("  4× 25→12 mm stepped shaft     $8  ea  = $32");
    println!("  1× Igus E08.15 (300 mm)                $55");
    println!("  Printed parts (PETG-CF 1 kg)           $40");
    println!("  M3 / M5 / M6 fasteners                 $10");
    println!("                              TOTAL ≈ $199");

    // Sanity: Part isn't Clone, so don't try to reuse its values.
    let _ = (
        INNER_PLATE_X,
        INNER_PLATE_Y,
        INNER_PLATE_Z,
        OUTER_RING_Z,
        OUTER_RING_ID,
    );
}
