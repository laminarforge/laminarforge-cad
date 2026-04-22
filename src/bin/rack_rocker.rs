use vcad::{centered_cube, centered_cylinder, Part};

// ─── Motorized Rack Rocker — Tilt Platform ───
//
// Single-axis seesaw rocker that sits on the floor of chip_incubator_v2
// and gently tilts the 100-chip rack (chip_stack_rack) ±10–15° at ~1
// cycle/min (1 rpm). Promotes uniform gas/liquid exchange across all
// 100 chips without vibration that would disturb cell attachment.
//
// MECHANISM
//     A NEMA 23 stepper motor drives an 8 mm acme lead screw. A nut
//     block rides the lead screw and is coupled (via a slotted linkage
//     arm) to a lever on the back edge of the top tilting plate. Two
//     R3 radial ball bearings (19 mm OD × 9.525 mm ID × 5 mm thick,
//     shielded) press-fit into side pivot blocks and ride on 8 mm
//     shafts bonded to the top tilting plate, providing a clean,
//     low-friction rocking axis.
//
// WHY STEPPER + LEAD SCREW (not servo + crank)
//     - Research (A-F424355F) says <10 N·m required; NEMA 23 (≥1.8 N·m
//       holding) × 60 mm lever arm × lead-screw mechanical advantage
//       comfortably clears this for a 15 kg rack.
//     - Steppers are simpler to DIY-drive with TMC2209 / DRV8825 and
//       do not need a feedback loop for 1 rpm positioning.
//     - Lead screw is self-locking (rack can't back-drive when power
//       is off — chips stay level mid-cycle if the MCU reboots).
//
// TILT GEOMETRY
//     arm radius r = 60 mm (distance from rocker axis to linkage pin)
//     lead-screw travel ±31 mm → θ_max = ±asin(31/60) × (180/π)
//                              ≈ ±31.1°  (mechanically limited)
//     Firmware clamps travel to ±15.5 mm → θ ≈ ±14.9°.
//
// TORQUE BUDGET
//     rack mass m ≈ 13 kg (aluminium) or 6 kg (PETG)
//     moment arm ≈ 0.15 m (rack CoG ~50 mm above pivot, + 100 mm
//                          lateral shift at 15°)
//     τ_peak ≈ m · g · d ≈ 13 · 9.81 · 0.15 ≈ 19.1 N·m at rack
//     But the lead-screw / linkage reduces motor τ by
//     r_arm · (2π / lead_mm) ≈ 60 · (2π / 2) ≈ 188×
//     → motor τ ≈ 0.10 N·m, trivial for NEMA 23 (1.8 N·m).
//
// INTERFACES
//     - BELOW: base plate 560 × 490 × 10 mm with 2 pairs of rail tabs
//       that mate with the chip_incubator_v2 floor rail slots
//       (x = ±60, ±180 mm; 12 mm wide, 3 mm deep).
//     - ABOVE: top tilting plate 500 × 440 × 8 mm with 8× M6 mount
//       holes on a 450 × 400 mm rectangle pattern, matching the
//       chip_stack_rack base plate.
//
// MATERIALS
//     Base plate + top tilting plate : 6061-T6 aluminium (CNC mill)
//     Pivot blocks + linkage + nut block : PETG (FDM print)
//     Rubber feet  : Sorbothane 8 mm dia × 2 mm (vibration damping)
//
// Exports:
//     output/rack_rocker_base.stl        — base plate w/ stepper boss, rails, feet
//     output/rack_rocker_top.stl         — tilting plate w/ M6 holes, ribs, tabs
//     output/rack_rocker_pivot_block.stl — one side pivot block (print 2×)
//     output/rack_rocker_linkage.stl     — 120 mm linkage arm
//     output/rack_rocker_nut_block.stl   — lead-screw nut carrier
//     output/rack_rocker_assembly.stl    — everything at 0° tilt

fn main() {
    // ══════════════════════════════════════════════════════════════
    // GLOBAL PARAMETERS
    // ══════════════════════════════════════════════════════════════

    // Base plate (sits on incubator floor)
    let base_x: f64 = 560.0;
    let base_y: f64 = 490.0;
    let base_z: f64 = 10.0;

    // Top tilting plate (carries the 100-chip rack)
    let top_x: f64 = 500.0;
    let top_y: f64 = 440.0;
    let top_z: f64 = 8.0;

    // Vertical distance from base top face to top plate bottom face (at 0° tilt)
    let pivot_clearance: f64 = 45.0;

    // Pivot geometry
    let pivot_axis_z = base_z / 2.0 + pivot_clearance; // above origin
    let pivot_arm_r: f64 = 60.0;                        // linkage arm radius

    // R3 bearing (ABEC-1, shielded): 19 mm OD × 9.525 mm ID × 5 mm thick
    let bearing_od: f64 = 19.0;
    let bearing_id: f64 = 9.525;
    let bearing_thk: f64 = 5.0;
    let bearing_bore_d = bearing_od + 0.05; // press-fit into PETG pivot block
    let _ = bearing_id;

    // Pivot shaft (8 mm steel dowel, press-fit into bearing ID w/ 9.525 sleeve)
    let shaft_d: f64 = 8.0;

    // NEMA 23 stepper mount (flange 56.4 mm, bolt circle 47.14 mm,
    // commonly given as 58.4 × 58.4 mm bolt pattern — 4× M5)
    let nema23_bolt_spacing: f64 = 58.4;
    let nema23_body: f64 = 56.4;
    let nema23_shaft_d: f64 = 6.35; // 1/4" coupler clearance
    let m5_clear: f64 = 5.5;

    // Lead screw (8 mm acme, 2 mm lead) — spans base on X axis
    let lead_screw_d: f64 = 8.0;
    let lead_screw_bearing_od: f64 = 16.0; // F688ZZ flanged, 8×16×5
    let lead_screw_bearing_thk: f64 = 5.0;

    // M6 rack mount pattern (matches chip_stack_rack base)
    let m6_clear: f64 = 6.6;
    let rack_pattern_x: f64 = 450.0;
    let rack_pattern_y: f64 = 400.0;

    // Incubator floor rail slot spec (from chip_incubator_v2.rs)
    let rail_slot_width: f64 = 12.0;
    let rail_slot_depth: f64 = 3.0;
    let rail_tab_len: f64 = 300.0; // shorter than slot length
    let rail_x_offsets = [-180.0, -60.0, 60.0, 180.0];

    // ══════════════════════════════════════════════════════════════
    // 1. BASE PLATE
    // ══════════════════════════════════════════════════════════════

    let base_body = centered_cube("base_body", base_x, base_y, base_z);

    // Rail tabs on the underside (extend into the incubator floor slots)
    let mut rail_tabs = Part::empty("rail_tabs");
    for (i, &xo) in rail_x_offsets.iter().enumerate() {
        let tab = centered_cube(
            &format!("rail_tab_{i}"),
            rail_slot_width - 0.3, // 0.3 mm slide clearance
            rail_tab_len,
            rail_slot_depth,
        )
        .translate(xo, 0.0, -(base_z / 2.0) - rail_slot_depth / 2.0);
        rail_tabs = rail_tabs + tab;
    }

    // Rubber foot pockets (4 corners, 8 mm dia × 2 mm deep)
    let foot_d: f64 = 8.0;
    let foot_depth: f64 = 2.0;
    let foot_inset: f64 = 25.0;
    let mut foot_pockets = Part::empty("foot_pockets");
    for (i, (fx, fy)) in [
        (-base_x / 2.0 + foot_inset, -base_y / 2.0 + foot_inset),
        (-base_x / 2.0 + foot_inset,  base_y / 2.0 - foot_inset),
        ( base_x / 2.0 - foot_inset, -base_y / 2.0 + foot_inset),
        ( base_x / 2.0 - foot_inset,  base_y / 2.0 - foot_inset),
    ]
    .iter()
    .enumerate()
    {
        let pocket = centered_cylinder(&format!("foot_pocket_{i}"), foot_d / 2.0, foot_depth + 0.1, 32)
            .translate(*fx, *fy, -(base_z / 2.0) + foot_depth / 2.0);
        foot_pockets = foot_pockets + pocket;
    }

    // Stepper mount boss: raised 15 mm block on back-left of base for
    // NEMA 23 face mount (shaft pointed +X along the lead screw)
    let boss_x: f64 = 80.0;
    let boss_y: f64 = 80.0;
    let boss_z: f64 = 15.0;
    let boss_cx: f64 = -base_x / 2.0 + boss_x / 2.0 + 20.0;
    let boss_cy: f64 = 0.0;
    let boss_cz: f64 = base_z / 2.0 + boss_z / 2.0;

    let stepper_boss = centered_cube("stepper_boss", boss_x, boss_y, boss_z)
        .translate(boss_cx, boss_cy, boss_cz);

    // Clearance cutout for the stepper body (cylindrical ⌀ 56.4 mm)
    // Drilled into +X face of the boss. Shaft points +X.
    let stepper_body_hole = centered_cylinder("stepper_body_hole", nema23_body / 2.0, boss_x + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(boss_cx, boss_cy, boss_cz);

    // Shaft pass-through (6.35 mm) through the +X face
    let stepper_shaft_hole = centered_cylinder("stepper_shaft_hole", nema23_shaft_d / 2.0 + 0.5, boss_x + 2.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(boss_cx, boss_cy, boss_cz);

    // 4× M5 mounting holes through the -X (back) face of the boss
    let mut nema_bolts = Part::empty("nema_bolts");
    for (i, (dy, dz)) in [
        (-nema23_bolt_spacing / 2.0, -nema23_bolt_spacing / 2.0),
        (-nema23_bolt_spacing / 2.0,  nema23_bolt_spacing / 2.0),
        ( nema23_bolt_spacing / 2.0, -nema23_bolt_spacing / 2.0),
        ( nema23_bolt_spacing / 2.0,  nema23_bolt_spacing / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let bolt = centered_cylinder(&format!("nema_bolt_{i}"), m5_clear / 2.0, boss_x + 2.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_cx, boss_cy + *dy, boss_cz + *dz);
        nema_bolts = nema_bolts + bolt;
    }

    // Lead-screw end bearing mounts (F688ZZ) — two upstands on +Z surface
    // Lead screw runs along +X from stepper boss toward +X side of base.
    let ls_mount_x: f64 = 18.0;
    let ls_mount_y: f64 = 30.0;
    let ls_mount_z: f64 = 20.0;
    // Near end: just past stepper boss. Far end: near +X edge of base.
    let ls_mount_near_cx = boss_cx + boss_x / 2.0 + ls_mount_x / 2.0 + 5.0;
    let ls_mount_far_cx  =  base_x / 2.0 - ls_mount_x / 2.0 - 20.0;
    let ls_mount_cz = base_z / 2.0 + ls_mount_z / 2.0;

    let ls_mount_near = centered_cube("ls_mount_near", ls_mount_x, ls_mount_y, ls_mount_z)
        .translate(ls_mount_near_cx, 0.0, ls_mount_cz);
    let ls_mount_far = centered_cube("ls_mount_far", ls_mount_x, ls_mount_y, ls_mount_z)
        .translate(ls_mount_far_cx, 0.0, ls_mount_cz);

    // F688ZZ bores through each upstand (cylinder along X)
    let ls_axis_z = base_z / 2.0 + ls_mount_z / 2.0; // lead screw centre height
    let ls_bearing_near = centered_cylinder(
        "ls_bearing_near",
        lead_screw_bearing_od / 2.0 + 0.05,
        lead_screw_bearing_thk + 0.2,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(ls_mount_near_cx, 0.0, ls_axis_z);
    let ls_bearing_far = centered_cylinder(
        "ls_bearing_far",
        lead_screw_bearing_od / 2.0 + 0.05,
        lead_screw_bearing_thk + 0.2,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(ls_mount_far_cx, 0.0, ls_axis_z);

    // Through-bore for the 8 mm lead-screw shaft in both mounts
    let ls_through_near = centered_cylinder("ls_through_near", lead_screw_d / 2.0 + 0.3, ls_mount_x + 2.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(ls_mount_near_cx, 0.0, ls_axis_z);
    let ls_through_far = centered_cylinder("ls_through_far", lead_screw_d / 2.0 + 0.3, ls_mount_x + 2.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(ls_mount_far_cx, 0.0, ls_axis_z);

    // Cable management channel (20 × 10 mm) from stepper boss to rear grommet
    let cable_ch_w: f64 = 20.0;
    let cable_ch_d: f64 = 10.0;
    let cable_ch_len: f64 = base_y / 2.0 - (boss_cy) + 5.0; // from boss to +Y edge
    let cable_channel = centered_cube("cable_channel", cable_ch_w, cable_ch_len, cable_ch_d + 0.1)
        .translate(
            boss_cx + boss_x / 2.0 - cable_ch_w / 2.0 - 5.0,
            boss_cy + cable_ch_len / 2.0,
            base_z / 2.0 - cable_ch_d / 2.0,
        );

    // Pivot-block mounting bolt pattern (4× M5 per block, 2 blocks = 8 holes)
    // Blocks sit on ±Y mid-edges, centred on X = 0.
    let pivot_bolt_inset_y: f64 = 30.0; // block half-footprint
    let pivot_bolt_inset_x: f64 = 25.0;
    let mut pivot_bolts = Part::empty("pivot_bolts");
    for &sy in [-1.0f64, 1.0].iter() {
        for &sx in [-1.0f64, 1.0].iter() {
            let bx = sx * pivot_bolt_inset_x;
            let by = sy * (base_y / 2.0 - pivot_bolt_inset_y);
            let bolt = centered_cylinder("piv_bolt", m5_clear / 2.0, base_z + 2.0, 24)
                .translate(bx, by, 0.0);
            pivot_bolts = pivot_bolts + bolt;
        }
    }

    // Assemble base plate
    let base = (base_body + rail_tabs + stepper_boss + ls_mount_near + ls_mount_far)
        - foot_pockets
        - stepper_body_hole
        - stepper_shaft_hole
        - nema_bolts
        - ls_bearing_near
        - ls_bearing_far
        - ls_through_near
        - ls_through_far
        - cable_channel
        - pivot_bolts;

    base.write_stl("output/rack_rocker_base.stl").unwrap();
    println!("Exported: output/rack_rocker_base.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. TOP TILTING PLATE
    // ══════════════════════════════════════════════════════════════

    let top_body = centered_cube("top_body", top_x, top_y, top_z);

    // 8× M6 rack mount holes (450 × 400 mm rectangle pattern)
    // Pattern: 4 corners + 4 midpoints → matches chip_stack_rack base
    let mut rack_holes = Part::empty("rack_holes");
    let m6_positions: [(f64, f64); 8] = [
        (-rack_pattern_x / 2.0, -rack_pattern_y / 2.0),
        ( rack_pattern_x / 2.0, -rack_pattern_y / 2.0),
        (-rack_pattern_x / 2.0,  rack_pattern_y / 2.0),
        ( rack_pattern_x / 2.0,  rack_pattern_y / 2.0),
        (0.0,                   -rack_pattern_y / 2.0),
        (0.0,                    rack_pattern_y / 2.0),
        (-rack_pattern_x / 2.0,  0.0),
        ( rack_pattern_x / 2.0,  0.0),
    ];
    for (i, (hx, hy)) in m6_positions.iter().enumerate() {
        let hole = centered_cylinder(&format!("m6_{i}"), m6_clear / 2.0, top_z + 2.0, 24)
            .translate(*hx, *hy, 0.0);
        rack_holes = rack_holes + hole;
    }

    // Pivot brackets at left + right mid-edges. Shaft bore 8 mm horizontal (X axis).
    let bracket_x: f64 = 18.0;   // thickness along X
    let bracket_y: f64 = 40.0;   // depth along Y
    let bracket_z: f64 = 35.0;   // tall, reaches down to pivot axis
    // Bracket sits below top plate, on the ±X mid-edges of the top plate
    let bracket_cz = -(top_z / 2.0) - bracket_z / 2.0;
    let top_pivot_axis_z = bracket_cz - bracket_z / 2.0 + 12.0; // shaft 12 mm up from bracket bottom
    // Top plate is placed (later) so that its own pivot axis aligns with base pivot axis.
    // For the STL we model the bracket relative to the top plate origin.

    let mut pivot_brackets = Part::empty("pivot_brackets");
    for &sx in [-1.0f64, 1.0].iter() {
        let bracket = centered_cube(
            "pivot_bracket",
            bracket_x,
            bracket_y,
            bracket_z,
        )
        .translate(sx * (top_x / 2.0 - bracket_x / 2.0), 0.0, bracket_cz);
        // Shaft bore 8 mm (press-fit dowel)
        let shaft_bore = centered_cylinder("shaft_bore", shaft_d / 2.0 + 0.1, bracket_x + 4.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(sx * (top_x / 2.0 - bracket_x / 2.0), 0.0, top_pivot_axis_z);
        pivot_brackets = pivot_brackets + (bracket - shaft_bore);
    }

    // Linkage arm pivot point — 8 mm hole, 60 mm back from rocker axis,
    // through a small reinforcement boss on the back (+Y) edge.
    let link_boss_x: f64 = 40.0;
    let link_boss_y: f64 = 30.0;
    let link_boss_z: f64 = 20.0;
    let link_boss_cy = top_y / 2.0 - link_boss_y / 2.0; // flush with back edge
    let link_boss_cz = -(top_z / 2.0) - link_boss_z / 2.0;
    let link_boss = centered_cube("link_boss", link_boss_x, link_boss_y, link_boss_z)
        .translate(0.0, link_boss_cy, link_boss_cz);
    // Pivot hole (8 mm, horizontal along X)
    let link_pivot_hole = centered_cylinder("link_pivot", shaft_d / 2.0 + 0.3, link_boss_x + 4.0, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, link_boss_cy, link_boss_cz);

    // Central reinforcement ribs (3×, 10 mm thick, perpendicular to rocker axis = Y direction)
    // Ribs run along Y, spaced across X.
    let rib_x: f64 = 10.0;
    let rib_y: f64 = top_y - 40.0;
    let rib_z: f64 = 12.0;
    let rib_cz = -(top_z / 2.0) - rib_z / 2.0;
    let rib_x_positions = [-120.0, 0.0, 120.0];
    let mut ribs = Part::empty("ribs");
    for (i, rx) in rib_x_positions.iter().enumerate() {
        let rib = centered_cube(&format!("rib_{i}"), rib_x, rib_y, rib_z)
            .translate(*rx, 0.0, rib_cz);
        ribs = ribs + rib;
    }

    // 4 corner chip-rack retention tabs (2 mm tall) on +Z face
    let tab_x: f64 = 20.0;
    let tab_y: f64 = 20.0;
    let tab_z: f64 = 2.0;
    let tab_inset: f64 = 15.0;
    let mut retention_tabs = Part::empty("retention_tabs");
    for (i, (tx, ty)) in [
        (-top_x / 2.0 + tab_inset, -top_y / 2.0 + tab_inset),
        (-top_x / 2.0 + tab_inset,  top_y / 2.0 - tab_inset),
        ( top_x / 2.0 - tab_inset, -top_y / 2.0 + tab_inset),
        ( top_x / 2.0 - tab_inset,  top_y / 2.0 - tab_inset),
    ]
    .iter()
    .enumerate()
    {
        let tab = centered_cube(&format!("ret_tab_{i}"), tab_x, tab_y, tab_z)
            .translate(*tx, *ty, top_z / 2.0 + tab_z / 2.0);
        retention_tabs = retention_tabs + tab;
    }

    let top = (top_body + pivot_brackets + link_boss + ribs + retention_tabs)
        - rack_holes
        - link_pivot_hole;

    top.write_stl("output/rack_rocker_top.stl").unwrap();
    println!("Exported: output/rack_rocker_top.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. PIVOT BLOCK (mounts to base plate ±Y edges; holds R3 bearing)
    // ══════════════════════════════════════════════════════════════

    let pb_x: f64 = 60.0;
    let pb_y: f64 = 40.0;
    let pb_z: f64 = pivot_clearance; // reaches up to pivot axis
    let pb_cz = base_z / 2.0 + pb_z / 2.0;

    let pb_body = centered_cube("pb_body", pb_x, pb_y, pb_z).translate(0.0, 0.0, pb_cz);

    // Bearing bore (press-fit R3) — through-bore along X axis, at pivot axis height
    let pb_bearing_bore = centered_cylinder(
        "pb_bearing_bore",
        bearing_bore_d / 2.0,
        pb_x + 2.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, pivot_axis_z);
    // 8 mm through-hole for shaft (in case bearing sits on one side only)
    let pb_shaft_through = centered_cylinder(
        "pb_shaft_through",
        shaft_d / 2.0 + 0.3,
        pb_x + 4.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, pivot_axis_z);

    // 4× M5 mounting holes in the base (downward)
    let mut pb_bolts = Part::empty("pb_bolts");
    for &sy in [-1.0f64, 1.0].iter() {
        for &sx in [-1.0f64, 1.0].iter() {
            let bx = sx * (pb_x / 2.0 - 10.0);
            let by = sy * (pb_y / 2.0 - 8.0);
            let bolt = centered_cylinder("pb_bolt", m5_clear / 2.0, pb_z + 2.0, 24)
                .translate(bx, by, pb_cz);
            pb_bolts = pb_bolts + bolt;
        }
    }

    // Bearing seat recess (5 mm deep on +X face to seat the bearing flange)
    let pb_recess = centered_cylinder(
        "pb_recess",
        bearing_od / 2.0 + 0.1,
        bearing_thk + 0.2,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(pb_x / 2.0 - (bearing_thk + 0.2) / 2.0, 0.0, pivot_axis_z);

    let pivot_block = pb_body - pb_bearing_bore - pb_shaft_through - pb_bolts - pb_recess;
    pivot_block
        .write_stl("output/rack_rocker_pivot_block.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_pivot_block.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. LINKAGE ARM (120 × 20 × 5 mm flat bar w/ slot + hole)
    // ══════════════════════════════════════════════════════════════

    let la_x: f64 = 120.0;
    let la_y: f64 = 20.0;
    let la_z: f64 = 5.0;
    let la_body = centered_cube("la_body", la_x, la_y, la_z);

    // Pivot hole at top-plate end (8 mm, at -X side)
    let la_pivot = centered_cylinder("la_pivot", shaft_d / 2.0 + 0.2, la_z + 2.0, 32)
        .translate(-la_x / 2.0 + 12.0, 0.0, 0.0);
    // Slotted hole at lead-screw end (8 mm wide × 16 mm long slot
    // to accommodate angular swing). Built as hole + two semicircles.
    let slot_w: f64 = 8.0 + 0.4;
    let slot_l: f64 = 16.0;
    let slot_center_x = la_x / 2.0 - 14.0;
    let slot_rect = centered_cube("slot_rect", slot_l, slot_w, la_z + 2.0)
        .translate(slot_center_x, 0.0, 0.0);
    let slot_cap_a = centered_cylinder("slot_cap_a", slot_w / 2.0, la_z + 2.0, 32)
        .translate(slot_center_x - slot_l / 2.0, 0.0, 0.0);
    let slot_cap_b = centered_cylinder("slot_cap_b", slot_w / 2.0, la_z + 2.0, 32)
        .translate(slot_center_x + slot_l / 2.0, 0.0, 0.0);
    let slot = slot_rect + slot_cap_a + slot_cap_b;

    let linkage = la_body - la_pivot - slot;
    linkage
        .write_stl("output/rack_rocker_linkage.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_linkage.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. NUT BLOCK (carries 8 mm acme lead-screw nut, bolts to linkage)
    // ══════════════════════════════════════════════════════════════

    let nb_x: f64 = 34.0;
    let nb_y: f64 = 30.0;
    let nb_z: f64 = 20.0;
    let nb_body = centered_cube("nb_body", nb_x, nb_y, nb_z);

    // Lead screw through-hole (8 mm) along X
    let nb_ls_hole = centered_cylinder("nb_ls_hole", lead_screw_d / 2.0 + 0.4, nb_x + 2.0, 32)
        .rotate(0.0, 90.0, 0.0);

    // Brass acme nut pocket: flanged (22 mm dia, 10 mm long body, 4 mm flange)
    // Model as a 22 mm dia cylindrical pocket, 14 mm deep from -X face
    let nb_nut_pocket = centered_cylinder("nb_nut_pocket", 22.0 / 2.0 + 0.1, 14.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(-(nb_x / 2.0) + 14.0 / 2.0 - 0.1, 0.0, 0.0);

    // 4× M4 flange bolt holes (square 16 mm pattern on -X face)
    let mut nb_flange_bolts = Part::empty("nb_flange_bolts");
    for &sy in [-1.0f64, 1.0].iter() {
        for &sz in [-1.0f64, 1.0].iter() {
            let bolt = centered_cylinder("nb_fb", 4.5 / 2.0, nb_x + 2.0, 24)
                .rotate(0.0, 90.0, 0.0)
                .translate(0.0, sy * 8.0, sz * 8.0);
            nb_flange_bolts = nb_flange_bolts + bolt;
        }
    }

    // Linkage attachment: 8 mm pin hole through +Y face (for linkage arm slot)
    let nb_link_pin = centered_cylinder("nb_link_pin", shaft_d / 2.0 + 0.2, nb_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, nb_z / 2.0 - 5.0);

    let nut_block = nb_body - nb_ls_hole - nb_nut_pocket - nb_flange_bolts - nb_link_pin;
    nut_block
        .write_stl("output/rack_rocker_nut_block.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_nut_block.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. ASSEMBLY (simplified geometry at 0° tilt — for visualization)
    // ══════════════════════════════════════════════════════════════
    // Rebuild simplified shapes (Part doesn't impl Clone) so the assembly
    // stays a small CSG tree and exports quickly.

    // Base plate (simple slab — no feature cutouts in viz)
    let asm_base_plate = centered_cube("asm_base_plate", base_x, base_y, base_z);

    // Stepper boss + cylindrical body stub, for visual identification
    let asm_stepper_boss =
        centered_cube("asm_stepper_boss", boss_x, boss_y, boss_z).translate(boss_cx, boss_cy, boss_cz);
    let asm_stepper_body =
        centered_cylinder("asm_stepper_body", nema23_body / 2.0, 70.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_cx - 70.0 / 2.0, boss_cy, boss_cz);

    // Lead screw stand-ins (just the rail between two end mounts)
    let asm_ls_mount_near =
        centered_cube("asm_ls_mount_near", ls_mount_x, ls_mount_y, ls_mount_z)
            .translate(ls_mount_near_cx, 0.0, ls_mount_cz);
    let asm_ls_mount_far =
        centered_cube("asm_ls_mount_far", ls_mount_x, ls_mount_y, ls_mount_z)
            .translate(ls_mount_far_cx, 0.0, ls_mount_cz);
    let ls_span = ls_mount_far_cx - ls_mount_near_cx;
    let asm_lead_screw =
        centered_cylinder("asm_lead_screw", lead_screw_d / 2.0, ls_span, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate((ls_mount_near_cx + ls_mount_far_cx) / 2.0, 0.0, ls_axis_z);

    // Pivot blocks — just two simple cubes on the ±Y edges
    let pb_left_y = -(base_y / 2.0 - pb_y / 2.0 - 20.0);
    let pb_right_y = base_y / 2.0 - pb_y / 2.0 - 20.0;
    let asm_pb_left = centered_cube("asm_pb_left", pb_x, pb_y, pb_z).translate(0.0, pb_left_y, pb_cz);
    let asm_pb_right = centered_cube("asm_pb_right", pb_x, pb_y, pb_z).translate(0.0, pb_right_y, pb_cz);

    // Top tilting plate at 0° tilt: place bottom of plate at pivot_axis_z
    let asm_top_plate = centered_cube("asm_top_plate", top_x, top_y, top_z)
        .translate(0.0, 0.0, pivot_axis_z + top_z / 2.0);

    // Reinforcement rib stubs under the top plate
    let rib_world_cz = pivot_axis_z - rib_z / 2.0;
    let mut asm_ribs = Part::empty("asm_ribs");
    for (i, rx) in rib_x_positions.iter().enumerate() {
        let rib = centered_cube(&format!("asm_rib_{i}"), rib_x, rib_y, rib_z)
            .translate(*rx, 0.0, rib_world_cz);
        asm_ribs = asm_ribs + rib;
    }

    // Linkage arm + nut block at 0° tilt (lead-screw mid-travel)
    let ls_mid_x = (ls_mount_near_cx + ls_mount_far_cx) / 2.0;
    // Linkage runs from top-plate back link boss down to the nut block.
    // In viz, draw it as a thin angled bar — approximate as a short cube.
    let link_world_y = link_boss_cy;
    let link_world_z_world = pivot_axis_z + top_z / 2.0 + link_boss_cz + link_boss_z / 2.0;
    let _ = link_world_z_world;
    let asm_linkage_bar = centered_cube("asm_linkage_bar", la_z, la_y, la_x)
        .translate(
            ls_mid_x,
            link_world_y / 2.0,
            (ls_axis_z + pivot_axis_z) / 2.0,
        );
    let asm_nut = centered_cube("asm_nut", nb_x, nb_y, nb_z).translate(ls_mid_x, 0.0, ls_axis_z);

    let assembly = asm_base_plate
        + asm_stepper_boss
        + asm_stepper_body
        + asm_ls_mount_near
        + asm_ls_mount_far
        + asm_lead_screw
        + asm_pb_left
        + asm_pb_right
        + asm_top_plate
        + asm_ribs
        + asm_linkage_bar
        + asm_nut;

    assembly
        .write_stl("output/rack_rocker_assembly.stl")
        .unwrap();
    println!("Exported: output/rack_rocker_assembly.stl");

    // ══════════════════════════════════════════════════════════════
    // SPECS
    // ══════════════════════════════════════════════════════════════

    let tilt_max_deg = (31.0f64 / pivot_arm_r).asin().to_degrees();
    println!();
    println!("── Rack Rocker — Motorized Tilt Platform Specs ──");
    println!("  Base plate        : {base_x:.0} × {base_y:.0} × {base_z:.0} mm  (6061-T6 Al)");
    println!("  Top tilting plate : {top_x:.0} × {top_y:.0} × {top_z:.0} mm  (6061-T6 Al)");
    println!("  Pivot clearance   : {pivot_clearance:.0} mm (base top to tilting plate bottom)");
    println!("  Tilt range        : ±{tilt_max_deg:.1}° (mechanical), ±15° (firmware-limited)");
    println!("  Cycle time        : 60 s full period (1 rpm)");
    println!("  Torque at motor   : ~0.10 N·m (NEMA 23 holding 1.8 N·m → 18× margin)");
    println!("  Torque at rack    : ~19 N·m peak at 15 kg, 0.15 m moment arm");
    println!("  Rack mount pattern: {rack_pattern_x:.0} × {rack_pattern_y:.0} mm, 8× M6 clearance");
    println!("  Floor interface   : 4 rail tabs @ x = ±60, ±180 mm (12 mm × 3 mm)");
    println!();
    println!("── BOM ──");
    println!("  Stepper motor     : NEMA 23, 1.8°/step, 1.8–3 N·m holding");
    println!("                      (e.g. StepperOnline 23HS30-2804S, 56 mm)");
    println!("  Stepper driver    : TMC2209 or DRV8825, 24 V, 2.8 A phase");
    println!("  Bearings (pivot)  : R3-2RS × 2  (19 × 9.525 × 5 mm, shielded)");
    println!("  Bearings (screw)  : F688ZZ × 2  (8 × 16 × 5 mm, flanged)");
    println!("  Lead screw        : 8 mm acme, 2 mm lead, 300 mm long");
    println!("  Lead-screw nut    : POM/brass flanged, 22 mm OD, 4× M4 bolts");
    println!("  Coupler           : 6.35 → 8 mm flexible jaw, 20 mm");
    println!("  Dowel shafts      : 8 mm × 50 mm steel × 2 (press-fit through R3)");
    println!("  Rubber feet       : Sorbothane 8 × 2 mm × 4");
    println!("  Fasteners         : M5 × 16 × 8 (stepper + pivot blocks),");
    println!("                      M6 × 16 × 8 (rack mount),");
    println!("                      M4 × 20 × 4 (nut flange)");
    println!();
    println!("── Parts (exported) ──");
    println!("  rack_rocker_base.stl         — 6061-T6 Al, CNC");
    println!("  rack_rocker_top.stl          — 6061-T6 Al, CNC");
    println!("  rack_rocker_pivot_block.stl  — PETG FDM × 2");
    println!("  rack_rocker_linkage.stl      — PETG FDM");
    println!("  rack_rocker_nut_block.stl    — PETG FDM");
    println!("  rack_rocker_assembly.stl     — visualization at 0° tilt");
}
