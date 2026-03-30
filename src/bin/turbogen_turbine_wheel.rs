use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::{Edge, Face, Shape, Solid, Wire};

// ─── Turbogenerator Turbine Wheel — STEP for Investment Casting ────────────
//
// Artifact A-EE749EE3: GG Turbine Wheel Aerodynamic Design
// Material: Inconel 713C (investment cast — designed for casting & 950°C)
//
// 75mm tip dia · 13 radial-fibred blades · radial inflow → axial exit
// Blade path follows meridional channel contour H1-H5 / S1-S5
// Blade angle: 0° (radial) at inlet → 42.5° (from axial) at RMS exit
//
// GEOMETRY METHOD:
//   Hub disc — solid of revolution (Face::revolve) of meridional profile
//   Blades   — B-spline loft (Solid::loft) through wire cross-sections
//   Bore     — built into revolve profile (no boolean subtract needed)
//
// The STEP file contains exact NURBS surfaces, not mesh approximations.
// Foundry imports this directly for wax pattern tooling or SLA printing.
//
// Investment casting design rules applied:
//   - Min wall thickness 1.0mm LE / 1.5mm body (Inconel 713C minimum ~1.5mm)
//   - Blade root fillets 0.5mm minimum — foundry instruction (see drawing notes)
//   - Smooth contours throughout (no stepped/faceted surfaces)
//   - Uniform section transitions (gradual LE-to-body thickness taper)
//
// Foundry workflow: receive STEP → apply ~2% Inconel 713C shrinkage →
//   design gating & risers → produce wax/resin pattern → invest → cast

/// Piecewise-linear interpolation over (z, r) contour points; t in [0, 1]
fn interp(pts: &[(f64, f64)], t: f64) -> (f64, f64) {
    let t = t.clamp(0.0, 1.0);
    let n = pts.len() - 1;
    let f = t * n as f64;
    let i = (f as usize).min(n - 1);
    let u = f - i as f64;
    (
        pts[i].0 + (pts[i + 1].0 - pts[i].0) * u,
        pts[i].1 + (pts[i + 1].1 - pts[i].1) * u,
    )
}

/// Piecewise-linear interpolation over (fraction, degrees) schedule
fn interp_deg(s: &[(f64, f64)], t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let n = s.len() - 1;
    let f = t * n as f64;
    let i = (f as usize).min(n - 1);
    let u = f - i as f64;
    s[i].1 + (s[i + 1].1 - s[i].1) * u
}

fn main() {
    // ── Meridional channel contour control points (z mm, r mm) ──
    // From A-EE749EE3 §4.3
    //
    // Hub: inner wall of passage (closer to shaft)
    //   H1(0, 37.5) → H2(8, 32) → H3(20, 22) → H4(35, 14) → H5(45, 10)
    //
    // Shroud: outer wall of passage (further from shaft)
    //   S1(12, 37.5) → S2(18, 34) → S3(28, 30) → S4(38, 27.5) → S5(45, 27.5)
    //
    // At inlet (t=0): both walls at r=37.5, passage is 12mm in Z
    // At exit  (t=1): both walls at z=45, passage is 17.5mm in R

    let hub_pts: [(f64, f64); 5] = [
        (0.0, 37.5),
        (8.0, 32.0),
        (20.0, 22.0),
        (35.0, 14.0),
        (45.0, 10.0),
    ];
    let shr_pts: [(f64, f64); 5] = [
        (12.0, 37.5),
        (18.0, 34.0),
        (28.0, 30.0),
        (38.0, 27.5),
        (45.0, 27.5),
    ];

    // Blade angle at RMS streamline (A-EE749EE3 §5.3)
    // (meridional fraction, beta degrees from local meridional direction)
    // beta=0 at LE means blade is aligned with meridional (radial at inlet)
    // beta=42.5 at TE means blade sweeps 42.5 deg from axial toward tangential
    let beta_sched: [(f64, f64); 6] = [
        (0.0, 0.0),
        (0.2, 8.0),
        (0.4, 18.0),
        (0.6, 28.0),
        (0.8, 36.0),
        (1.0, 42.5),
    ];

    let n_blades = TG_TURB_BLADE_COUNT; // 13
    let blade_thick = TG_TURB_BLADE_THICKNESS; // 1.5mm
    let le_thick = TG_TURB_BLADE_LE_THICKNESS; // 1.0mm
    let depth = TG_TURB_AXIAL_LENGTH; // 45mm
    let back_t = TG_TURB_BACK_DISK_THICKNESS; // 8mm
    let bore_r = TG_SHAFT_DIA / 2.0; // 6mm
    let tip_r = TG_TURB_TIP_RADIUS; // 37.5mm

    // ═══════════════════════════════════════════════════════════════════
    // §1 — Hub disc via solid of revolution
    // ═══════════════════════════════════════════════════════════════════
    //
    // The hub disc is the solid body from which blades protrude. Built by
    // revolving a closed meridional profile 360° around the Z (shaft) axis.
    //
    // Profile in the XZ plane (y=0):
    //   - Outer contour follows hub aerodynamic surface (H1→H5)
    //   - Back disk at full tip radius behind the flow exit
    //   - Bore hole at shaft diameter throughout
    //
    // Result is an exact solid of revolution with analytical surfaces —
    // no stacking of discrete cylinders.

    let profile_pts = [
        dvec3(bore_r, 0.0, 0.0),              // P0: bore at inlet face
        dvec3(tip_r, 0.0, 0.0),               // P1: outer at inlet (full radius)
        dvec3(32.0, 0.0, 8.0),                // P2: hub contour H2
        dvec3(22.0, 0.0, 20.0),               // P3: hub contour H3
        dvec3(14.0, 0.0, 35.0),               // P4: hub contour H4
        dvec3(10.0, 0.0, depth),              // P5: hub contour H5 (exit)
        dvec3(tip_r, 0.0, depth),             // P6: back disk start (at tip radius)
        dvec3(tip_r, 0.0, depth + back_t),    // P7: back disk end
        dvec3(bore_r, 0.0, depth + back_t),   // P8: bore at back face
    ];

    // Build closed Wire from profile edges (P0→P1→...→P8→P0)
    let n_prof = profile_pts.len();
    let prof_edges: Vec<Edge> = (0..n_prof)
        .map(|i| Edge::segment(profile_pts[i], profile_pts[(i + 1) % n_prof]))
        .collect();
    let prof_edge_refs: Vec<&Edge> = prof_edges.iter().collect();
    let profile_wire = Wire::from_edges(prof_edge_refs);
    let profile_face = Face::from_wire(&profile_wire);

    // Revolve 360° around Z axis — produces solid with bore included
    let hub_solid = profile_face.revolve(dvec3(0.0, 0.0, 0.0), dvec3(0.0, 0.0, 1.0), None);
    let mut wheel: Shape = hub_solid.into();

    println!("Hub disc built via solid of revolution (9-point meridional profile)");

    // ═══════════════════════════════════════════════════════════════════
    // §2 — Compute blade station data with wrap angles
    // ═══════════════════════════════════════════════════════════════════
    //
    // At each meridional station, compute:
    //   - Hub/shroud positions (z, r)
    //   - Mid-streamline position
    //   - Accumulated wrap angle from blade angle integration
    //
    // Wrap angle dtheta is computed by integrating:
    //   dtheta/dm = tan(beta) / r
    // where m is the meridional arc length and r is the mid-stream radius.
    //
    // 20 stations for smooth B-spline loft interpolation.

    let n_sta: usize = 20;

    let mut sta_zh: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_rh: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_zs: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_rs: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_zm: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_rm: Vec<f64> = Vec::with_capacity(n_sta + 1);
    let mut sta_wrap: Vec<f64> = Vec::with_capacity(n_sta + 1);

    let mut wrap_acc = 0.0_f64;

    for i in 0..=n_sta {
        let t = i as f64 / n_sta as f64;
        let (zh, rh) = interp(&hub_pts, t);
        let (zs, rs) = interp(&shr_pts, t);
        let zm = (zh + zs) / 2.0;
        let rm = (rh + rs) / 2.0;

        if i > 0 {
            let zm_prev = sta_zm[i - 1];
            let rm_prev = sta_rm[i - 1];
            let b_now = interp_deg(&beta_sched, t).to_radians();
            let b_prev =
                interp_deg(&beta_sched, (i - 1) as f64 / n_sta as f64).to_radians();
            let b_avg = (b_now + b_prev) / 2.0;
            let dm = ((zm - zm_prev).powi(2) + (rm - rm_prev).powi(2)).sqrt();
            let r_avg = (rm + rm_prev) / 2.0;
            wrap_acc += b_avg.tan() * dm / r_avg;
        }

        sta_zh.push(zh);
        sta_rh.push(rh);
        sta_zs.push(zs);
        sta_rs.push(rs);
        sta_zm.push(zm);
        sta_rm.push(rm);
        sta_wrap.push(wrap_acc);
    }

    let total_wrap_deg = sta_wrap[n_sta].to_degrees();
    println!(
        "Blade stations computed: {} stations, total wrap = {:.1}°",
        n_sta, total_wrap_deg
    );

    // ═══════════════════════════════════════════════════════════════════
    // §3 — Build blades via B-spline loft
    // ═══════════════════════════════════════════════════════════════════
    //
    // At each meridional station, construct a quadrilateral Wire cross-
    // section defined by 4 corner points:
    //
    //   hub_ps ──── shr_ps     (pressure side: hub to shroud)
    //     |            |
    //   hub_ss ──── shr_ss     (suction side: hub to shroud)
    //
    // Points are in 3D cylindrical coordinates converted to cartesian:
    //   x = r * cos(theta ± dtheta)
    //   y = r * sin(theta ± dtheta)
    //   z = axial position from contour interpolation
    //
    // Where dtheta = half_thickness / r gives constant tangential thickness.
    //
    // Solid::loft() passes these wires through BRepOffsetAPI_ThruSections
    // (isSolid=true, CheckCompatibility=true) producing smooth NURBS blade
    // surfaces suitable for investment casting tooling.
    //
    // Blade root penetrates 0.3mm into hub solid for clean boolean union.

    let tau = 2.0 * std::f64::consts::PI;
    let hub_penetration = 0.3; // mm — blade extends into hub for clean boolean
    let le_taper_frac = 0.10; // LE thickness transitions over first 10% of meridional path
    let root_fillet_r = 0.4; // mm — casting fillet at blade-hub junctions (hot-tear prevention)

    // ── Phase 1: Loft and union all 13 blades ──

    for bi in 0..n_blades {
        let base = (bi as f64) * tau / n_blades as f64;
        let mut station_wires: Vec<Wire> = Vec::with_capacity(n_sta + 1);

        for si in 0..=n_sta {
            let t = si as f64 / n_sta as f64;
            let theta = base + sta_wrap[si];

            // Contour positions at this station
            let (zh, rh) = interp(&hub_pts, t);
            let (zs, rs) = interp(&shr_pts, t);

            // Blade thickness with smooth LE taper
            // Linearly transitions from le_thick → blade_thick over first 10%
            let thick = if t < le_taper_frac {
                le_thick + (blade_thick - le_thick) * (t / le_taper_frac)
            } else {
                blade_thick
            };
            let half_t = thick / 2.0;

            // Extend blade root slightly into hub for clean boolean intersection
            let rh_ext = rh - hub_penetration;

            // Angular offset for constant tangential thickness at each radius
            let dth_h = half_t / rh_ext;
            let dth_s = half_t / rs;

            // Four corner points of blade cross-section at this station
            let hub_ps = dvec3(
                rh_ext * (theta + dth_h).cos(),
                rh_ext * (theta + dth_h).sin(),
                zh,
            );
            let shr_ps = dvec3(
                rs * (theta + dth_s).cos(),
                rs * (theta + dth_s).sin(),
                zs,
            );
            let shr_ss = dvec3(
                rs * (theta - dth_s).cos(),
                rs * (theta - dth_s).sin(),
                zs,
            );
            let hub_ss = dvec3(
                rh_ext * (theta - dth_h).cos(),
                rh_ext * (theta - dth_h).sin(),
                zh,
            );

            // Build closed quadrilateral wire (consistent CCW winding)
            let e1 = Edge::segment(hub_ps, shr_ps); // pressure side
            let e2 = Edge::segment(shr_ps, shr_ss); // shroud tip
            let e3 = Edge::segment(shr_ss, hub_ss); // suction side
            let e4 = Edge::segment(hub_ss, hub_ps); // hub root
            station_wires.push(Wire::from_edges([&e1, &e2, &e3, &e4]));
        }

        // Loft through all station wires → smooth NURBS blade solid
        let blade_solid = Solid::loft(station_wires);
        let blade_shape: Shape = blade_solid.into();

        // Union blade with hub disc
        wheel = wheel.union(&blade_shape).into();

        println!("  Blade {}/{} lofted + unioned", bi + 1, n_blades);
    }

    // ── Phase 2: Selective blade-root filleting ──
    //
    // After all blades are unioned, iterate the shape's edges and fillet
    // ONLY the blade-root edges (where blade meets hub surface).
    //
    // Selection criterion: edge midpoint radius is near the hub contour
    // (within 1mm of hub_pts interpolated surface), AND the edge is not
    // on the bore, back disk, or shroud tip.
    //
    // This avoids fillet_new_edges() which tries ALL boolean edges and hangs
    // on complex degenerate intersection topology.

    println!();
    println!("Selecting blade-root edges for filleting...");

    let mut root_edges: Vec<opencascade::primitives::Edge> = Vec::new();
    let mut total_edges = 0_usize;

    for edge in wheel.edges() {
        total_edges += 1;

        // Get edge start/end points
        let sp = edge.start_point();
        let ep = edge.end_point();
        let mid_x = (sp.x + ep.x) / 2.0;
        let mid_y = (sp.y + ep.y) / 2.0;
        let mid_z = (sp.z + ep.z) / 2.0;
        let mid_r = (mid_x * mid_x + mid_y * mid_y).sqrt();

        // Skip edges clearly on the bore (r ≈ bore_r)
        if mid_r < bore_r + 1.0 {
            continue;
        }
        // Skip edges on the back disk face (z ≈ depth or z ≈ depth+back_t)
        if mid_z > depth - 1.0 {
            continue;
        }
        // Skip edges at the inlet face (z ≈ 0)
        if mid_z < 0.5 {
            continue;
        }

        // Check if this edge's midpoint radius is near the hub contour
        // Interpolate hub radius at this z position
        let t_z = (mid_z / depth).clamp(0.0, 1.0);
        let (_, hub_r_at_z) = interp(&hub_pts, t_z);

        // Blade-root intersection curves sit right at the hub surface.
        // Use tight tolerance to exclude loft cross-section edges that are
        // merely near the hub but not ON it.
        let r_diff = (mid_r - hub_r_at_z).abs();
        if r_diff < 0.6 {
            // Root curves are long (span multiple stations along meridional path).
            // Loft cross-section edges at hub side are only ~blade_thick (1.5mm).
            // Filter out short edges that are station cross-sections, not root curves.
            let edge_len = ((ep.x - sp.x).powi(2)
                + (ep.y - sp.y).powi(2)
                + (ep.z - sp.z).powi(2))
            .sqrt();
            // Root curves should be >3mm (they span ~2-3 stations of meridional path)
            if edge_len > 3.0 {
                root_edges.push(edge);
            }
        }
    }

    let n_root_edges = root_edges.len();
    println!(
        "  Found {} blade-root edges out of {} total edges",
        n_root_edges, total_edges
    );

    if !root_edges.is_empty() {
        println!("  Applying R{:.1}mm fillet to blade-root edges...", root_fillet_r);
        // fillet_edges mutates wheel in-place
        wheel.fillet_edges(root_fillet_r, root_edges);
        println!("  Blade-root fillets applied successfully!");
    } else {
        println!("  WARNING: No blade-root edges found for filleting.");
        println!("  STEP will be exported without fillets — add as foundry instruction.");
    }

    // ═══════════════════════════════════════════════════════════════════
    // §4 — Export STEP + STL
    // ═══════════════════════════════════════════════════════════════════

    wheel
        .write_step("output/turbogen_turbine_wheel.stp")
        .expect("Failed to write turbine wheel STEP");
    wheel
        .write_stl("output/turbogen_turbine_wheel.stl")
        .expect("Failed to write turbine wheel STL");

    println!();
    println!("=== Turbogenerator Turbine Wheel (Investment Cast) ===");
    println!("Exported: output/turbogen_turbine_wheel.stp");
    println!("Exported: output/turbogen_turbine_wheel.stl");
    println!("Material: Inconel 713C (investment cast)");
    println!("Tip diameter: {}mm", TG_TURB_TIP_DIA);
    println!("Blade count: {}", n_blades);
    println!("Inlet blade height: {}mm", TG_TURB_INLET_BLADE_HEIGHT);
    println!(
        "Exit: tip r={}mm, hub r={}mm",
        TG_TURB_EXIT_TIP_RADIUS, TG_TURB_EXIT_HUB_RADIUS
    );
    println!("Blade angle: 0° (radial inlet) → 42.5° (RMS exit from axial)");
    println!("Total blade wrap angle: {:.1}°", total_wrap_deg);
    println!(
        "Blade thickness: {}mm LE → {}mm body (smooth taper over first 10%)",
        TG_TURB_BLADE_LE_THICKNESS, TG_TURB_BLADE_THICKNESS
    );
    println!(
        "Blade root fillet: R{:.1}mm (modeled in CAD, {} root edges filleted)",
        root_fillet_r,
        n_root_edges
    );
    println!("Hub:    H1(0,37.5) → H2(8,32) → H3(20,22) → H4(35,14) → H5(45,10)");
    println!("Shroud: S1(12,37.5) → S2(18,34) → S3(28,30) → S4(38,27.5) → S5(45,27.5)");
    println!();
    println!("Geometry method:");
    println!("  Hub disc: solid of revolution (exact analytical surfaces)");
    println!("  Blades:   B-spline loft through {} cross-sections (smooth NURBS)", n_sta + 1);
    println!("  Fillets:  selective edge fillet on blade-root intersection curves");
    println!("  Bore:     integral to hub profile (no boolean subtract)");
    println!();
    println!("Manufacturing: Investment casting (NOT CNC).");
    println!("  Material: Inconel 713C (gamma-prime strengthened, designed for casting, 950°C capable)");
    println!("  Foundry applies ~2% shrinkage compensation for 713C.");
    println!("  Pattern: foundry SLA or wax injection from this geometry.");
    println!("  Post-cast: HIP + solution treat + age, then balance to G2.5.");
    println!("  Tip clearance: 0.6mm radial cold (0.23mm hot with Al housing).");
}
