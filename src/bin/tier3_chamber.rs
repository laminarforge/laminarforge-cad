use vcad::{centered_cube, centered_cylinder, Part};

// ─── Tier-3 Walk-In Environmental Chamber — 2000-Chip Scale ───
//
// Purpose-built reach-in environmental chamber for the Tier-3 pan-disease
// biodistribution panel. Houses 4 parallel rocker beds × 500 chips each
// = 2000-chip total capacity, per scale-up analysis (A-F288A11B).
//
// EXTERNAL ENVELOPE: ~2000 × 2000 × 2200 mm
// INTERIOR:          ~1850 × 1850 × 2000 mm (150 mm insulated panels)
// OPERATING:         37 °C / 5 % CO2 / ≥90 % RH
//
// COORDINATE SYSTEM (ASSEMBLY)
//     Origin at interior floor center.
//     X : left / right        (interior −925 … +925, 1850 mm)
//     Y : front / back        (interior −925 … +925, 1850 mm; front = −Y)
//     Z : bottom / top        (interior floor at 0, interior ceiling at +2000)
//
// STRUCTURE
//     - Bosch Rexroth 4545 (45 × 45 mm) extrusion frame, 8 vertical posts,
//       top/bottom perimeter rails, cross-bracing on 3 walls. ~60 m total.
//     - 150 mm PIR-foam insulated panels, 0.6 mm galv-steel skins
//       (food-grade interior, painted exterior). U ≈ 0.15 W/m²K.
//     - Double-door airlock on front face: 1000 × 2000 × 600 mm vestibule,
//       two gasketed interlocked doors.
//     - 4 rocker bed positions in a 2 × 2 grid, each 900 × 750 mm, with
//       600 mm service aisles.
//     - Zoned HVAC: 4 independent thermal zones (500 W PTC heater +
//       EBM-Papst blower + SHT45 ×3 + MH-Z19B ×3 per zone, local ESP32-S3
//       PID, MQTT orchestration).
//     - HEPA H14 recirculation plenum in ceiling (1200 CFM, 30× ACH).
//     - Sloped floor (1:100) + central drain, perimeter condensate channels.
//     - 4 × heated triple-pane view windows, one per wall.
//     - Right-wall service pass-through: 4 media lines, 4 waste lines,
//       CO2, 24 VDC, Cat6 ×4, compressed air, drain.
//     - External control box on right wall (600 × 400 × 300 mm).
//
// EXPORTS
//     output/tier3_frame.stl
//     output/tier3_panels.stl
//     output/tier3_airlock.stl
//     output/tier3_rocker_beds.stl
//     output/tier3_zone_hvac.stl
//     output/tier3_hepa_plenum.stl
//     output/tier3_floor_drain.stl
//     output/tier3_windows.stl
//     output/tier3_service_pass_through.stl
//     output/tier3_control_box.stl
//     output/tier3_assembly.stl

// ══════════════════════════════════════════════════════════════
// GLOBAL DIMENSIONS
// ══════════════════════════════════════════════════════════════

// Interior
const INT_X: f64 = 1850.0;
const INT_Y: f64 = 1850.0;
const INT_Z: f64 = 2000.0;

// Insulated panel thickness
const PANEL_T: f64 = 150.0;

// External envelope = interior + 2× panels
const EXT_X: f64 = INT_X + PANEL_T * 2.0; // 2150 (slightly over 2000 spec; flat-pack panel edges)
const EXT_Y: f64 = INT_Y + PANEL_T * 2.0;
const EXT_Z: f64 = INT_Z + PANEL_T * 2.0; // 2300 (includes ceiling panel + floor panel)

// Bosch Rexroth 4545 profile
const EXT_W: f64 = 45.0; // 45 × 45 mm extrusion

// Airlock
const AIRLOCK_W: f64 = 1000.0; // X (wide)
const AIRLOCK_H: f64 = 2000.0; // Z (tall, full interior)
const AIRLOCK_D: f64 = 600.0; // Y (deep into room)

// Rocker bed footprint
const BED_X: f64 = 900.0;
const BED_Y: f64 = 750.0;
#[allow(dead_code)]
const BED_Z: f64 = 900.0; // overall platform height incl. rocker + rack
const BED_AISLE: f64 = 600.0;

// HEPA plenum
const PLENUM_X: f64 = 2000.0;
const PLENUM_Y: f64 = 1800.0;
const PLENUM_Z: f64 = 200.0;

// Windows
const WIN_X: f64 = 1200.0;
const WIN_Z: f64 = 800.0;
const WIN_T: f64 = 35.0; // triple-pane tempered

// Service pass-through panel
const SVC_X: f64 = 150.0; // into wall thickness
const SVC_Y: f64 = 400.0;
const SVC_Z: f64 = 800.0;

// Control box
const CTRL_X: f64 = 600.0;
const CTRL_Y: f64 = 300.0;
const CTRL_Z: f64 = 400.0;

// ══════════════════════════════════════════════════════════════
// 1. FRAME — Bosch Rexroth 4545 extrusion skeleton
// ══════════════════════════════════════════════════════════════

fn frame() -> Part {
    // 8 vertical posts (4 corners + 4 midpoints on long walls)
    // Corners at (±(EXT_X/2 - EXT_W/2), ±(EXT_Y/2 - EXT_W/2))
    // Midpoints at (0, ±(EXT_Y/2 - EXT_W/2)) and (±(EXT_X/2 - EXT_W/2), 0)
    // Midpoints placed on long walls (±Y walls assumed long if square, symmetric).
    let post_h = EXT_Z;
    let post_z_center = post_h / 2.0 - PANEL_T / 2.0; // referenced to interior floor = 0

    let half_x = EXT_X / 2.0 - EXT_W / 2.0;
    let half_y = EXT_Y / 2.0 - EXT_W / 2.0;

    let post_positions: [(f64, f64); 8] = [
        (-half_x, -half_y),
        (half_x, -half_y),
        (-half_x, half_y),
        (half_x, half_y),
        (0.0, -half_y),
        (0.0, half_y),
        (-half_x, 0.0),
        (half_x, 0.0),
    ];

    let mut posts = Part::empty("t3_posts");
    for (i, &(px, py)) in post_positions.iter().enumerate() {
        let p = centered_cube(&format!("t3_post_{i}"), EXT_W, EXT_W, post_h).translate(
            px,
            py,
            post_z_center,
        );
        posts = posts + p;
    }

    // Top + bottom perimeter rails (horizontal beams along the 4 edges)
    // Rails sit inboard of posts, at floor level and ceiling level.
    let top_z = INT_Z + PANEL_T / 2.0; // just under ceiling panel center
    let bot_z = -PANEL_T / 2.0; // just above floor panel center

    let rail_x_len = EXT_X - EXT_W * 2.0;
    let rail_y_len = EXT_Y - EXT_W * 2.0;

    let mut rails = Part::empty("t3_rails");
    for &z in &[top_z, bot_z] {
        // front and back rails along X
        let rf =
            centered_cube("t3_rail_front", rail_x_len, EXT_W, EXT_W).translate(0.0, -half_y, z);
        let rb = centered_cube("t3_rail_back", rail_x_len, EXT_W, EXT_W).translate(0.0, half_y, z);
        // left and right rails along Y
        let rl = centered_cube("t3_rail_left", EXT_W, rail_y_len, EXT_W).translate(-half_x, 0.0, z);
        let rr = centered_cube("t3_rail_right", EXT_W, rail_y_len, EXT_W).translate(half_x, 0.0, z);
        rails = rails + rf + rb + rl + rr;
    }

    // Cross-bracing on 3 walls (back + left + right, skip front for door/airlock access)
    // Diagonal brace modeled as an inclined cube (rotation is ok on a brace stand-in).
    let brace_len = (rail_x_len.powi(2) + post_h.powi(2)).sqrt();
    let brace_angle_y = (post_h / rail_x_len).atan().to_degrees(); // around Y axis for back wall

    let back_brace = centered_cube("t3_brace_back", brace_len, EXT_W, EXT_W)
        .rotate(0.0, brace_angle_y, 0.0)
        .translate(0.0, half_y, post_z_center);

    let brace_len_y = (rail_y_len.powi(2) + post_h.powi(2)).sqrt();
    let brace_angle_x = (post_h / rail_y_len).atan().to_degrees();

    let left_brace = centered_cube("t3_brace_left", EXT_W, brace_len_y, EXT_W)
        .rotate(brace_angle_x, 0.0, 0.0)
        .translate(-half_x, 0.0, post_z_center);
    let right_brace = centered_cube("t3_brace_right", EXT_W, brace_len_y, EXT_W)
        .rotate(brace_angle_x, 0.0, 0.0)
        .translate(half_x, 0.0, post_z_center);

    posts + rails + back_brace + left_brace + right_brace
}

// ══════════════════════════════════════════════════════════════
// 2. PANELS — 150 mm PIR insulated panels (6 walls + floor + ceiling)
// ══════════════════════════════════════════════════════════════

fn panels() -> Part {
    // Model as an outer box − interior cavity − openings (front door, windows, service).
    let outer = centered_cube("t3_pan_outer", EXT_X, EXT_Y, EXT_Z).translate(0.0, 0.0, INT_Z / 2.0); // centered so interior floor = 0

    let cavity =
        centered_cube("t3_pan_cavity", INT_X, INT_Y, INT_Z).translate(0.0, 0.0, INT_Z / 2.0);

    // Front airlock opening (cut through front wall; the airlock body is separate)
    let front_opening = centered_cube(
        "t3_pan_front",
        AIRLOCK_W + 20.0,
        PANEL_T + 4.0,
        AIRLOCK_H + 20.0,
    )
    .translate(0.0, -(INT_Y / 2.0 + PANEL_T / 2.0), AIRLOCK_H / 2.0);

    // Window cutouts: 4× 1200 × 800 mm, one per wall, centered at 1200 mm above floor.
    let win_z_center = 1200.0;
    let win_front = centered_cube("t3_pan_win_f", WIN_X, PANEL_T + 4.0, WIN_Z).translate(
        -600.0,
        -(INT_Y / 2.0 + PANEL_T / 2.0),
        win_z_center,
    );
    // (front window offset so it doesn't collide with airlock; airlock occupies X = -500..+500)
    // Place front window on LEFT side of front face.
    let win_back = centered_cube("t3_pan_win_b", WIN_X, PANEL_T + 4.0, WIN_Z).translate(
        0.0,
        INT_Y / 2.0 + PANEL_T / 2.0,
        win_z_center,
    );
    let win_left = centered_cube("t3_pan_win_l", PANEL_T + 4.0, WIN_X, WIN_Z).translate(
        -(INT_X / 2.0 + PANEL_T / 2.0),
        0.0,
        win_z_center,
    );
    let win_right = centered_cube("t3_pan_win_r", PANEL_T + 4.0, WIN_X, WIN_Z).translate(
        INT_X / 2.0 + PANEL_T / 2.0,
        0.0,
        win_z_center,
    );

    // Service panel cutout (right wall, below window height)
    let svc_cut = centered_cube("t3_pan_svc", SVC_X + 4.0, SVC_Y, SVC_Z).translate(
        INT_X / 2.0 + PANEL_T / 2.0,
        500.0,
        400.0,
    );

    // HEPA plenum access cutout on ceiling (inset from edges, returns handled via floor slots)
    let plenum_cut = centered_cube(
        "t3_pan_plenum",
        PLENUM_X - 200.0,
        PLENUM_Y - 200.0,
        PANEL_T + 4.0,
    )
    .translate(0.0, 0.0, INT_Z + PANEL_T / 2.0);

    // Floor perimeter return slots (thin slits 30 mm wide inboard of walls)
    let slot_inset = 80.0;
    let slot_w = 30.0;
    let slot_len_x = INT_X - slot_inset * 2.0;
    let slot_len_y = INT_Y - slot_inset * 2.0;
    let slot_z = -PANEL_T + slot_w / 2.0;
    let floor_slot_front = centered_cube("t3_floor_slot_f", slot_len_x, slot_w, slot_w).translate(
        0.0,
        -(INT_Y / 2.0 - slot_inset),
        slot_z,
    );
    let floor_slot_back = centered_cube("t3_floor_slot_b", slot_len_x, slot_w, slot_w).translate(
        0.0,
        INT_Y / 2.0 - slot_inset,
        slot_z,
    );
    let floor_slot_left = centered_cube("t3_floor_slot_l", slot_w, slot_len_y, slot_w).translate(
        -(INT_X / 2.0 - slot_inset),
        0.0,
        slot_z,
    );
    let floor_slot_right = centered_cube("t3_floor_slot_r", slot_w, slot_len_y, slot_w).translate(
        INT_X / 2.0 - slot_inset,
        0.0,
        slot_z,
    );

    (outer - cavity)
        - front_opening
        - win_front
        - win_back
        - win_left
        - win_right
        - svc_cut
        - plenum_cut
        - floor_slot_front
        - floor_slot_back
        - floor_slot_left
        - floor_slot_right
}

// ══════════════════════════════════════════════════════════════
// 3. AIRLOCK — double-door vestibule
// ══════════════════════════════════════════════════════════════

fn airlock() -> Part {
    // Airlock vestibule sits in front wall; center in X, protrudes into interior.
    // Walls are 50 mm insulated; two doors at ±Y ends of vestibule.
    let wall_t = 50.0;

    // Outer shell (interior of chamber) — vestibule as a box
    let outer = centered_cube("t3_al_outer", AIRLOCK_W, AIRLOCK_D, AIRLOCK_H).translate(
        0.0,
        -(INT_Y / 2.0) + AIRLOCK_D / 2.0,
        AIRLOCK_H / 2.0,
    );

    let cavity = centered_cube(
        "t3_al_cavity",
        AIRLOCK_W - wall_t * 2.0,
        AIRLOCK_D - wall_t * 2.0,
        AIRLOCK_H - wall_t,
    )
    .translate(
        0.0,
        -(INT_Y / 2.0) + AIRLOCK_D / 2.0,
        AIRLOCK_H / 2.0 + wall_t / 2.0,
    );

    // Exterior door (through chamber front wall) — Y = -(INT_Y/2) - PANEL_T
    let door_w = AIRLOCK_W - 200.0;
    let door_h = AIRLOCK_H - 200.0;
    let ext_door_cut = centered_cube("t3_al_ext_cut", door_w, wall_t + 4.0, door_h).translate(
        0.0,
        -(INT_Y / 2.0) + wall_t / 2.0,
        door_h / 2.0 + 100.0,
    );

    // Interior door (into chamber) at Y = -(INT_Y/2) + AIRLOCK_D
    let int_door_cut = centered_cube("t3_al_int_cut", door_w, wall_t + 4.0, door_h).translate(
        0.0,
        -(INT_Y / 2.0) + AIRLOCK_D - wall_t / 2.0,
        door_h / 2.0 + 100.0,
    );

    // Gasket channels around each door (3 × 3 mm recessed, simplified as shallow cube)
    let gasket_t = 4.0;
    let gasket_ext = centered_cube("t3_al_gasket_ext", door_w + 20.0, gasket_t, door_h + 20.0)
        - centered_cube(
            "t3_al_gasket_ext_in",
            door_w + 10.0,
            gasket_t + 0.2,
            door_h + 10.0,
        );
    let gasket_ext =
        gasket_ext.translate(0.0, -(INT_Y / 2.0) + gasket_t / 2.0, door_h / 2.0 + 100.0);

    let gasket_int = centered_cube("t3_al_gasket_int", door_w + 20.0, gasket_t, door_h + 20.0)
        - centered_cube(
            "t3_al_gasket_int_in",
            door_w + 10.0,
            gasket_t + 0.2,
            door_h + 10.0,
        );
    let gasket_int = gasket_int.translate(
        0.0,
        -(INT_Y / 2.0) + AIRLOCK_D - gasket_t / 2.0,
        door_h / 2.0 + 100.0,
    );

    // Interlock sensor housing (small tab on the side — visual only)
    let interlock = centered_cube("t3_al_interlock", 60.0, 40.0, 80.0).translate(
        AIRLOCK_W / 2.0 - 30.0,
        -(INT_Y / 2.0) + AIRLOCK_D / 2.0,
        1500.0,
    );

    ((outer - cavity) - ext_door_cut - int_door_cut) + gasket_ext + gasket_int + interlock
}

// ══════════════════════════════════════════════════════════════
// 4. ROCKER BED STATION — single bed (X/Y are center coords)
// ══════════════════════════════════════════════════════════════

fn rocker_bed_station(cx: f64, cy: f64) -> Part {
    // Each station: 900 × 750 mm footprint. We model:
    //  - floor-mounted load cell pads (4× 80 × 80 × 25 mm)
    //  - rocker base plate 900 × 750 × 30 mm
    //  - 2 pivot blocks (60 × 40 × 100 mm)
    //  - top tilting plate 800 × 680 × 15 mm at pivot_z + 100
    //  - chip rack stand-in 780 × 660 × 600 mm (represents 500-chip stack)

    let load_cell_d = 25.0;
    let lc_w = 80.0;
    let lc_positions = [
        (cx - BED_X / 2.0 + 60.0, cy - BED_Y / 2.0 + 60.0),
        (cx + BED_X / 2.0 - 60.0, cy - BED_Y / 2.0 + 60.0),
        (cx - BED_X / 2.0 + 60.0, cy + BED_Y / 2.0 - 60.0),
        (cx + BED_X / 2.0 - 60.0, cy + BED_Y / 2.0 - 60.0),
    ];
    let mut load_cells = Part::empty("t3_bed_lc");
    for (i, &(lcx, lcy)) in lc_positions.iter().enumerate() {
        let lc = centered_cube(&format!("t3_lc_{i}"), lc_w, lc_w, load_cell_d).translate(
            lcx,
            lcy,
            load_cell_d / 2.0,
        );
        load_cells = load_cells + lc;
    }

    let base_z = 30.0;
    let base = centered_cube("t3_bed_base", BED_X, BED_Y, base_z).translate(
        cx,
        cy,
        load_cell_d + base_z / 2.0,
    );

    let pivot_h = 100.0;
    let pb_left = centered_cube("t3_bed_pb_l", 60.0, 40.0, pivot_h).translate(
        cx,
        cy - BED_Y / 2.0 + 40.0,
        load_cell_d + base_z + pivot_h / 2.0,
    );
    let pb_right = centered_cube("t3_bed_pb_r", 60.0, 40.0, pivot_h).translate(
        cx,
        cy + BED_Y / 2.0 - 40.0,
        load_cell_d + base_z + pivot_h / 2.0,
    );

    let top_z = 15.0;
    let top = centered_cube("t3_bed_top", BED_X - 100.0, BED_Y - 70.0, top_z).translate(
        cx,
        cy,
        load_cell_d + base_z + pivot_h + top_z / 2.0,
    );

    // 500-chip rack stand-in (dummy envelope)
    let rack_h = 600.0;
    let rack = centered_cube("t3_bed_rack", BED_X - 120.0, BED_Y - 90.0, rack_h).translate(
        cx,
        cy,
        load_cell_d + base_z + pivot_h + top_z + rack_h / 2.0,
    );

    // Stepper motor housing stub
    let motor = centered_cylinder("t3_bed_motor", 56.4 / 2.0, 70.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(cx - BED_X / 2.0 + 35.0, cy, load_cell_d + base_z + 40.0);

    load_cells + base + pb_left + pb_right + top + rack + motor
}

fn rocker_beds() -> Part {
    // 2 × 2 grid with BED_AISLE between beds, centered in interior floor.
    // X positions: -(BED_X + BED_AISLE)/2 and +(BED_X + BED_AISLE)/2
    // Y positions: same pattern (we are roughly square).
    let x_off = (BED_X + BED_AISLE) / 2.0; // 750
    let y_off = (BED_Y + BED_AISLE) / 2.0; // 675
    let mut beds = Part::empty("t3_beds");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            beds = beds + rocker_bed_station(sx * x_off, sy * y_off);
        }
    }
    beds
}

// ══════════════════════════════════════════════════════════════
// 5. ZONE HVAC — one per bed (4 total)
// ══════════════════════════════════════════════════════════════

fn zone_hvac_unit(cx: f64, cy: f64) -> Part {
    // Each zone HVAC module mounts above its bed, below the HEPA plenum.
    // Contents:
    //  - 500 W PTC heater box   120 × 120 × 60 mm
    //  - 200 mm EBM-Papst blower  200 Ø × 80 mm
    //  - SHT45 + MH-Z19B sensor trees (3 small blocks at different Z)
    //  - ESP32-S3 controller box 80 × 60 × 25 mm

    let z_base = 1700.0; // mounted near ceiling, below plenum return

    // PTC heater housing
    let heater = centered_cube("t3_z_heater", 120.0, 120.0, 60.0).translate(cx - 120.0, cy, z_base);

    // Blower
    let blower =
        centered_cylinder("t3_z_blower", 200.0 / 2.0, 80.0, 48).translate(cx + 120.0, cy, z_base);

    // ESP32-S3 box
    let esp = centered_cube("t3_z_esp", 80.0, 60.0, 25.0).translate(cx, cy + 200.0, z_base);

    // 3 sensor trees at Z = 500, 1100, 1700 (low/mid/high)
    let mut sensors = Part::empty("t3_z_sensors");
    for (i, &sz) in [500.0f64, 1100.0, 1700.0].iter().enumerate() {
        // SHT45 + MH-Z19B paired on a small PCB
        let s_combo =
            centered_cube(&format!("t3_z_s_{i}"), 40.0, 20.0, 10.0).translate(cx, cy - 200.0, sz);
        sensors = sensors + s_combo;
    }

    // Duct stub from heater/blower down to bed zone
    let duct =
        centered_cylinder("t3_z_duct", 100.0 / 2.0, 400.0, 24).translate(cx, cy, z_base - 250.0);

    heater + blower + esp + sensors + duct
}

fn zone_hvac() -> Part {
    let x_off = (BED_X + BED_AISLE) / 2.0;
    let y_off = (BED_Y + BED_AISLE) / 2.0;
    let mut hvac = Part::empty("t3_hvac_all");
    for &sx in &[-1.0f64, 1.0] {
        for &sy in &[-1.0f64, 1.0] {
            hvac = hvac + zone_hvac_unit(sx * x_off, sy * y_off);
        }
    }
    hvac
}

// ══════════════════════════════════════════════════════════════
// 6. HEPA PLENUM — ceiling-mounted recirculation
// ══════════════════════════════════════════════════════════════

fn hepa_plenum() -> Part {
    // 2000 × 1800 × 200 mm plenum box at top of chamber.
    // Inside: HEPA H14 filter (1830 × 1630 × 150 mm) + 1200 CFM blower + ducting.
    let plenum_z = INT_Z - PLENUM_Z / 2.0; // top of interior

    let shell = centered_cube("t3_plenum_shell", PLENUM_X, PLENUM_Y, PLENUM_Z)
        .translate(0.0, 0.0, plenum_z);

    let cavity = centered_cube(
        "t3_plenum_cavity",
        PLENUM_X - 40.0,
        PLENUM_Y - 40.0,
        PLENUM_Z - 20.0,
    )
    .translate(0.0, 0.0, plenum_z);

    // HEPA H14 filter (Camfil Megalam EN1822) — slightly smaller than cavity
    let hepa = centered_cube("t3_plenum_hepa", PLENUM_X - 60.0, PLENUM_Y - 60.0, 150.0)
        .translate(0.0, 0.0, plenum_z);

    // 1200 CFM centrifugal blower at one end (EBM-Papst K3G250)
    let blower = centered_cylinder("t3_plenum_blower", 250.0 / 2.0, 150.0, 48).translate(
        -PLENUM_X / 2.0 + 250.0,
        PLENUM_Y / 2.0 - 200.0,
        plenum_z,
    );

    // Return ducts down to floor slots (4 corners)
    let duct_h = 400.0;
    let mut ducts = Part::empty("t3_plenum_ducts");
    for (i, &(dx, dy)) in [
        (-PLENUM_X / 2.0 + 100.0, -PLENUM_Y / 2.0 + 100.0),
        (PLENUM_X / 2.0 - 100.0, -PLENUM_Y / 2.0 + 100.0),
        (-PLENUM_X / 2.0 + 100.0, PLENUM_Y / 2.0 - 100.0),
        (PLENUM_X / 2.0 - 100.0, PLENUM_Y / 2.0 - 100.0),
    ]
    .iter()
    .enumerate()
    {
        let d = centered_cylinder(&format!("t3_plenum_duct_{i}"), 100.0 / 2.0, duct_h, 24)
            .translate(dx, dy, plenum_z - PLENUM_Z / 2.0 - duct_h / 2.0);
        ducts = ducts + d;
    }

    (shell - cavity) + hepa + blower + ducts
}

// ══════════════════════════════════════════════════════════════
// 7. FLOOR DRAIN + condensate channels
// ══════════════════════════════════════════════════════════════

fn floor_drain() -> Part {
    // Floor slopes 1:100 toward a central drain (100 mm ø stainless grate).
    // Modeled as a circular recess at origin + p-trap stub below + perimeter
    // condensate channel around inner wall.

    let drain_d = 100.0;
    let drain_depth = 30.0;
    let drain_grate =
        centered_cylinder("t3_drain_grate", drain_d / 2.0, 10.0, 48).translate(0.0, 0.0, 5.0);

    let drain_body = centered_cylinder("t3_drain_body", drain_d / 2.0 - 5.0, drain_depth, 48)
        .translate(0.0, 0.0, -drain_depth / 2.0 + 5.0);

    // P-trap stub (goes below floor panel to exterior)
    let ptrap_h = 200.0;
    let ptrap = centered_cylinder("t3_drain_ptrap", 50.0 / 2.0, ptrap_h, 24).translate(
        0.0,
        100.0,
        -drain_depth - ptrap_h / 2.0,
    );

    // Perimeter condensate channel — 50 mm wide × 15 mm deep, 100 mm inboard from walls
    let inset = 100.0;
    let ch_w = 50.0;
    let ch_d = 15.0;
    let outer_x = INT_X - inset * 2.0;
    let outer_y = INT_Y - inset * 2.0;
    let inner_x = outer_x - ch_w * 2.0;
    let inner_y = outer_y - ch_w * 2.0;
    let ch_z = ch_d / 2.0;

    let ch_outer =
        centered_cube("t3_drain_ch_out", outer_x, outer_y, ch_d).translate(0.0, 0.0, ch_z);
    let ch_inner =
        centered_cube("t3_drain_ch_in", inner_x, inner_y, ch_d + 0.2).translate(0.0, 0.0, ch_z);
    let channel = ch_outer - ch_inner;

    drain_grate + drain_body + ptrap + channel
}

// ══════════════════════════════════════════════════════════════
// 8. WINDOW ASSEMBLIES (one per wall)
// ══════════════════════════════════════════════════════════════

fn window_assembly(wall: &str) -> Part {
    // Triple-pane tempered glass with ITO heating. Modeled as a single slab
    // in the wall opening; 35 mm thick. Frame is a lip around perimeter.
    let win_z_center = 1200.0;

    let glass = centered_cube("t3_win_glass", WIN_X, WIN_T, WIN_Z);
    let frame_outer = centered_cube("t3_win_frame_o", WIN_X + 60.0, WIN_T + 10.0, WIN_Z + 60.0);
    let frame_inner = centered_cube("t3_win_frame_i", WIN_X + 10.0, WIN_T + 20.0, WIN_Z + 10.0);
    let frame = frame_outer - frame_inner;

    let pane = glass + frame;

    // ITO heating bus bar strips (visual) — two thin strips at top & bottom edges
    let bus_top = centered_cube("t3_win_bus_t", WIN_X - 20.0, WIN_T + 2.0, 10.0).translate(
        0.0,
        0.0,
        WIN_Z / 2.0 - 15.0,
    );
    let bus_bot = centered_cube("t3_win_bus_b", WIN_X - 20.0, WIN_T + 2.0, 10.0).translate(
        0.0,
        0.0,
        -WIN_Z / 2.0 + 15.0,
    );

    let assembled = pane + bus_top + bus_bot;

    // Place on the named wall
    match wall {
        "front" => assembled.translate(-600.0, -(INT_Y / 2.0 + PANEL_T / 2.0), win_z_center),
        "back" => assembled.translate(0.0, INT_Y / 2.0 + PANEL_T / 2.0, win_z_center),
        "left" => assembled.rotate(0.0, 0.0, 90.0).translate(
            -(INT_X / 2.0 + PANEL_T / 2.0),
            0.0,
            win_z_center,
        ),
        "right" => assembled.rotate(0.0, 0.0, 90.0).translate(
            INT_X / 2.0 + PANEL_T / 2.0,
            0.0,
            win_z_center,
        ),
        _ => assembled,
    }
}

fn windows() -> Part {
    window_assembly("front")
        + window_assembly("back")
        + window_assembly("left")
        + window_assembly("right")
}

// ══════════════════════════════════════════════════════════════
// 9. SERVICE PASS-THROUGH (right wall)
// ══════════════════════════════════════════════════════════════

fn service_pass_through() -> Part {
    // Gasketed bulkhead panel 150 × 400 × 800 mm embedded in the right wall.
    // Houses:
    //  - 4× media line bulkheads (DN10 stainless, 16 mm Ø clearance each)
    //  - 4× waste line bulkheads (DN10 stainless)
    //  - 1× CO2 line (1/4" SS tube, ~7 mm Ø)
    //  - 1× 24 VDC power gland (20 mm Ø)
    //  - 4× Cat6 data glands (10 mm Ø each)
    //  - 1× compressed air gland (12 mm Ø)
    //  - 1× drain bulkhead (50 mm Ø)

    let panel = centered_cube("t3_svc_panel", SVC_X, SVC_Y, SVC_Z);

    let mut holes = Part::empty("t3_svc_holes");

    // 4 media lines (top row)
    for i in 0..4 {
        let y = -SVC_Y / 2.0 + 50.0 + (i as f64) * 100.0;
        let h = centered_cylinder(&format!("t3_svc_media_{i}"), 16.0 / 2.0, SVC_X + 4.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, SVC_Z / 2.0 - 80.0);
        holes = holes + h;
    }

    // 4 waste lines (second row)
    for i in 0..4 {
        let y = -SVC_Y / 2.0 + 50.0 + (i as f64) * 100.0;
        let h = centered_cylinder(&format!("t3_svc_waste_{i}"), 16.0 / 2.0, SVC_X + 4.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, SVC_Z / 2.0 - 200.0);
        holes = holes + h;
    }

    // CO2 (row 3)
    let co2 = centered_cylinder("t3_svc_co2", 7.0 / 2.0, SVC_X + 4.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, -150.0, SVC_Z / 2.0 - 320.0);
    holes = holes + co2;

    // 24VDC power gland
    let pwr = centered_cylinder("t3_svc_pwr", 20.0 / 2.0, SVC_X + 4.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, -50.0, SVC_Z / 2.0 - 320.0);
    holes = holes + pwr;

    // 4 Cat6 glands
    for i in 0..4 {
        let y = -SVC_Y / 2.0 + 80.0 + (i as f64) * 50.0;
        let h = centered_cylinder(&format!("t3_svc_cat6_{i}"), 10.0 / 2.0, SVC_X + 4.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, SVC_Z / 2.0 - 420.0);
        holes = holes + h;
    }

    // Compressed air
    let air = centered_cylinder("t3_svc_air", 12.0 / 2.0, SVC_X + 4.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 150.0, SVC_Z / 2.0 - 520.0);
    holes = holes + air;

    // Drain bulkhead
    let drain = centered_cylinder("t3_svc_drain", 50.0 / 2.0, SVC_X + 4.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, -SVC_Z / 2.0 + 80.0);
    holes = holes + drain;

    (panel - holes).translate(INT_X / 2.0 + PANEL_T / 2.0, 500.0, 400.0)
}

// ══════════════════════════════════════════════════════════════
// 10. EXTERNAL CONTROL BOX
// ══════════════════════════════════════════════════════════════

fn control_box() -> Part {
    // NEMA 4X enclosure mounted on the right exterior wall.
    // Contains: main controller (industrial PC), 24V PSUs, CO2 MFC,
    // MQTT broker (Raspberry Pi 5), breakers.
    let outer = centered_cube("t3_ctrl_outer", CTRL_X, CTRL_Y, CTRL_Z);
    let cavity = centered_cube("t3_ctrl_cavity", CTRL_X - 8.0, CTRL_Y - 8.0, CTRL_Z - 8.0);

    // Vent louvers (4 slots on the top)
    let mut vents = Part::empty("t3_ctrl_vents");
    for i in 0..4 {
        let x = -CTRL_X / 2.0 + 80.0 + (i as f64) * 140.0;
        let v = centered_cube(&format!("t3_ctrl_vent_{i}"), 100.0, 20.0, 10.0).translate(
            x,
            0.0,
            CTRL_Z / 2.0,
        );
        vents = vents + v;
    }

    let box_part = (outer - cavity) - vents;
    // Mount high on right wall exterior
    box_part
        .translate(INT_X / 2.0 + PANEL_T + CTRL_Y / 2.0, -400.0, 1500.0)
        .rotate(0.0, 0.0, 90.0) // rotate so CTRL_X dimension is along Y of chamber
}

// ══════════════════════════════════════════════════════════════
// MAIN
// ══════════════════════════════════════════════════════════════

fn main() {
    std::fs::create_dir_all("output").ok();

    let frame_p = frame();
    frame_p.write_stl("output/tier3_frame.stl").unwrap();
    println!("Exported: output/tier3_frame.stl");

    let panels_p = panels();
    panels_p.write_stl("output/tier3_panels.stl").unwrap();
    println!("Exported: output/tier3_panels.stl");

    let airlock_p = airlock();
    airlock_p.write_stl("output/tier3_airlock.stl").unwrap();
    println!("Exported: output/tier3_airlock.stl");

    let beds_p = rocker_beds();
    beds_p.write_stl("output/tier3_rocker_beds.stl").unwrap();
    println!("Exported: output/tier3_rocker_beds.stl");

    let hvac_p = zone_hvac();
    hvac_p.write_stl("output/tier3_zone_hvac.stl").unwrap();
    println!("Exported: output/tier3_zone_hvac.stl");

    let plenum_p = hepa_plenum();
    plenum_p.write_stl("output/tier3_hepa_plenum.stl").unwrap();
    println!("Exported: output/tier3_hepa_plenum.stl");

    let drain_p = floor_drain();
    drain_p.write_stl("output/tier3_floor_drain.stl").unwrap();
    println!("Exported: output/tier3_floor_drain.stl");

    let windows_p = windows();
    windows_p.write_stl("output/tier3_windows.stl").unwrap();
    println!("Exported: output/tier3_windows.stl");

    let svc_p = service_pass_through();
    svc_p
        .write_stl("output/tier3_service_pass_through.stl")
        .unwrap();
    println!("Exported: output/tier3_service_pass_through.stl");

    let ctrl_p = control_box();
    ctrl_p.write_stl("output/tier3_control_box.stl").unwrap();
    println!("Exported: output/tier3_control_box.stl");

    // Full assembly — rebuild parts (Part is not Clone-able, so rebuild)
    let assembly = frame()
        + panels()
        + airlock()
        + rocker_beds()
        + zone_hvac()
        + hepa_plenum()
        + floor_drain()
        + windows()
        + service_pass_through()
        + control_box();
    assembly.write_stl("output/tier3_assembly.stl").unwrap();
    println!("Exported: output/tier3_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // SPECS REPORT
    // ══════════════════════════════════════════════════════════════
    let interior_vol_m3 = (INT_X * INT_Y * INT_Z) / 1.0e9;
    let external_vol_m3 = (EXT_X * EXT_Y * EXT_Z) / 1.0e9;
    let chamber_area_m2 = 2.0 * ((EXT_X * EXT_Y) + (EXT_X * EXT_Z) + (EXT_Y * EXT_Z)) / 1.0e6;
    let u_value = 0.15; // W/m²K for 150 mm PIR + steel skins
    let delta_t: f64 = 37.0 - 22.0;
    let heat_loss_w = u_value * chamber_area_m2 * delta_t;

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   TIER-3 WALK-IN ENVIRONMENTAL CHAMBER — SPECS");
    println!("══════════════════════════════════════════════════════════════");
    println!();
    println!(" ENVELOPE");
    println!("   External:           {EXT_X:.0} × {EXT_Y:.0} × {EXT_Z:.0} mm");
    println!("   Interior:           {INT_X:.0} × {INT_Y:.0} × {INT_Z:.0} mm");
    println!("   Insulated panels:   {PANEL_T:.0} mm PIR foam, 0.6 mm galv steel skins");
    println!("   Interior volume:    {interior_vol_m3:.2} m³");
    println!("   External volume:    {external_vol_m3:.2} m³");
    println!("   External area:      {chamber_area_m2:.2} m²  (U = {u_value} W/m²K)");
    println!("   Steady-state heat loss @ ΔT = {delta_t:.0} K:   {heat_loss_w:.0} W");
    println!();
    println!(" FRAME (Bosch Rexroth 4545 extrusion)");
    println!("   8 vertical posts + top/bottom perimeter rails + 3-wall bracing");
    println!("   Estimated length:   ~60 m extrusion, ~320 M8 T-slot fasteners");
    println!();
    println!(" AIRLOCK (front face)");
    println!("   Envelope:           {AIRLOCK_W:.0} × {AIRLOCK_D:.0} × {AIRLOCK_H:.0} mm");
    println!("   Two interlocked gasketed doors (ext + int)");
    println!();
    println!(" 4 ROCKER BEDS (2×2 grid, 600 mm aisles)");
    println!("   Each bed footprint: {BED_X:.0} × {BED_Y:.0} mm");
    println!("   Chip capacity:      4 beds × 500 chips = 2000 total");
    println!("   Load cells:         4× per bed (S-type, 500 kg range, 16× total)");
    println!();
    println!(" ZONED HVAC (4 independent zones)");
    println!("   Heater per zone:    500 W PTC (4× 200 W of Tier-1 scaled for volume)");
    println!("   Blower per zone:    200 mm EBM-Papst, 600 CFM");
    println!("   Sensors per zone:   3× (SHT45 + MH-Z19B) @ low/mid/high Z");
    println!("   Controller:         ESP32-S3 × 4 local PID, MQTT orchestration");
    println!();
    println!(" HEPA RECIRCULATION PLENUM (ceiling)");
    println!("   Dimensions:         {PLENUM_X:.0} × {PLENUM_Y:.0} × {PLENUM_Z:.0} mm");
    println!("   Filter:             HEPA H14 (99.995 % @ 0.3 µm, Camfil Megalam EN1822)");
    println!("   Blower:             1200 CFM (EBM-Papst K3G250)");
    println!("   Air changes:        30 ACH at interior volume");
    println!();
    println!(" FLOOR DRAIN + CONDENSATE");
    println!("   Central drain:      100 mm Ø stainless grate, 1:100 sloped floor");
    println!("   Perimeter channel:  50 mm × 15 mm, 100 mm inboard from walls");
    println!("   P-trap:             routes to external waste tank");
    println!();
    println!(" WINDOWS (4× one per wall)");
    println!("   Dimensions:         {WIN_X:.0} × {WIN_Z:.0} × {WIN_T:.0} mm triple-pane tempered");
    println!("   ITO heating:        prevents condensation");
    println!();
    println!(" SERVICE PASS-THROUGH (right wall, {SVC_Y:.0} × {SVC_Z:.0} mm panel)");
    println!("   4× media lines (DN10 SS), 4× waste lines (DN10 SS)");
    println!("   CO2 gas (1/4\" SS), 24 VDC, Cat6 ×4, compressed air, drain");
    println!();
    println!(" EXTERNAL CONTROL BOX (right wall exterior)");
    println!("   Dimensions:         {CTRL_X:.0} × {CTRL_Y:.0} × {CTRL_Z:.0} mm");
    println!("   Houses:             main controller, 24V PSUs, CO2 MFC, Pi 5 broker");
    println!();
    println!(" EXPORTED STLs");
    println!("   output/tier3_frame.stl");
    println!("   output/tier3_panels.stl");
    println!("   output/tier3_airlock.stl");
    println!("   output/tier3_rocker_beds.stl");
    println!("   output/tier3_zone_hvac.stl");
    println!("   output/tier3_hepa_plenum.stl");
    println!("   output/tier3_floor_drain.stl");
    println!("   output/tier3_windows.stl");
    println!("   output/tier3_service_pass_through.stl");
    println!("   output/tier3_control_box.stl");
    println!("   output/tier3_assembly.stl");
    println!("══════════════════════════════════════════════════════════════");
}
