use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Turbine Wheel — STEP for Investment Casting ────────────
//
// Artifact A-EE749EE3: GG Turbine Wheel Aerodynamic Design
// Material: Inconel 713C (investment cast — designed for casting & 950°C)
//
// 75mm tip dia · 13 radial-fibred blades · radial inflow → axial exit
// Blade path follows meridional channel contour H1-H5 / S1-S5
// Blade angle: 0° (radial) at inlet → 42.5° (from axial) at RMS exit
//
// Each blade is traced through the meridional channel using the blade angle
// schedule. Wrap angle computed by integrating dθ/dm = tan(β)/r along the
// mid-streamline. Blade elements placed at each station using Z-aligned
// cylinders (inlet/turn where passage is axially-oriented) or axis-aligned
// bounding boxes (exit where passage is radially-oriented).
//
// Foundry workflow: receive STEP → apply ~2% Inconel 713C shrinkage →
//   design gating & risers → produce wax/resin pattern → invest → cast

/// Z-aligned cylinder at origin
fn cyl_z(z0: f64, h: f64, r: f64) -> Shape {
    Workplane::xy()
        .translated(dvec3(0.0, 0.0, z0))
        .circle(0.0, 0.0, r)
        .to_face()
        .extrude(dvec3(0.0, 0.0, h))
        .into()
}

/// Z-aligned cylinder offset to (cx, cy)
fn cyl_at(cx: f64, cy: f64, z0: f64, h: f64, r: f64) -> Shape {
    Workplane::xy()
        .translated(dvec3(cx, cy, z0))
        .circle(0.0, 0.0, r)
        .to_face()
        .extrude(dvec3(0.0, 0.0, h))
        .into()
}

/// Axis-aligned box centered at (cx, cy) in XY, from z0 upward
fn box_at(cx: f64, cy: f64, z0: f64, w: f64, d: f64, h: f64) -> Shape {
    Workplane::xy()
        .translated(dvec3(cx, cy, z0))
        .rect(w, d)
        .to_face()
        .extrude(dvec3(0.0, 0.0, h))
        .into()
}

/// Piecewise-linear interpolation over (z, r) contour points; t ∈ [0, 1]
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
    // (meridional fraction, β degrees from local meridional direction)
    // β=0 at LE means blade is aligned with meridional (radial at inlet)
    // β=42.5° at TE means blade sweeps 42.5° from axial toward tangential
    let beta_sched: [(f64, f64); 6] = [
        (0.0, 0.0),
        (0.2, 8.0),
        (0.4, 18.0),
        (0.6, 28.0),
        (0.8, 36.0),
        (1.0, 42.5),
    ];

    let n_blades = TG_TURB_BLADE_COUNT; // 13
    let blade_r = TG_TURB_BLADE_THICKNESS / 2.0; // 0.75mm
    let le_r = TG_TURB_BLADE_LE_THICKNESS / 2.0; // 0.50mm
    let blade_thick = TG_TURB_BLADE_THICKNESS; // 1.5mm
    let depth = TG_TURB_AXIAL_LENGTH; // 45mm
    let back_t = TG_TURB_BACK_DISK_THICKNESS; // 8mm

    // ═══════════════════════════════════════════════════════════════════
    // §1 — Hub disc body (stepped revolution along actual hub contour)
    // ═══════════════════════════════════════════════════════════════════
    //
    // The hub body is the solid disc from which blades protrude. It follows
    // the hub contour from the aerodynamic spec: large at inlet (r=37.5),
    // tapering to small at exit (r=10).

    let n_hub = 15;
    let dz = depth / n_hub as f64; // 3mm per step
    let mut wheel: Shape = cyl_z(0.0, dz, hub_pts[0].1);
    for i in 1..n_hub {
        let t = i as f64 / n_hub as f64;
        let (_, r) = interp(&hub_pts, t);
        wheel = wheel.union(&cyl_z(t * depth, dz, r)).into();
    }

    // Back disc (structural, connects wheel to shaft via nut-clamp)
    wheel = wheel.union(&cyl_z(depth, back_t, TG_TURB_TIP_RADIUS)).into();

    // Shaft bore (12mm, matches S6001 bearing)
    wheel = wheel
        .subtract(&cyl_z(-0.5, depth + back_t + 1.0, TG_SHAFT_DIA / 2.0))
        .into();

    println!("Hub disc built: {} steps + back disc + bore", n_hub);

    // ═══════════════════════════════════════════════════════════════════
    // §2 — Compute blade station data with wrap angles
    // ═══════════════════════════════════════════════════════════════════
    //
    // At each meridional station, compute:
    //   - Hub/shroud positions (z, r)
    //   - Mid-streamline position
    //   - Accumulated wrap angle from blade angle integration
    //
    // Wrap angle Δθ is computed by integrating:
    //   dθ/dm = tan(β) / r
    // where m is the meridional arc length and r is the mid-stream radius.

    let n_sta: usize = 10;

    // Station data
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
    // §3 — Build blades: trace each blade through meridional channel
    // ═══════════════════════════════════════════════════════════════════
    //
    // Strategy: at each meridional station, place a blade element that
    // captures the blade's cross-section at that location.
    //
    // - Z-dominated stations (inlet/turn where passage height is in Z):
    //   Use a thin Z-aligned cylinder at the blade's radial position.
    //   The cylinder spans the full Z-extent of the passage at that station.
    //
    // - R-dominated stations (exit where passage height is in R):
    //   Use an axis-aligned bounding box of the blade cross-section.
    //   The box captures the radial extent (hub to shroud) and is thin
    //   in the tangential direction.
    //
    // Transition threshold: when radial passage span exceeds 4× blade
    // thickness, switch from cylinder to box representation.

    let tau = 2.0 * std::f64::consts::PI;
    let r_threshold = blade_thick * 4.0; // 6mm

    for bi in 0..n_blades {
        let base = (bi as f64) * tau / n_blades as f64;

        for si in 0..n_sta {
            let theta = base + (sta_wrap[si] + sta_wrap[si + 1]) / 2.0;

            // Mid-segment contour positions
            let tm = ((si as f64) + 0.5) / n_sta as f64;
            let (_zh, rh) = interp(&hub_pts, tm);
            let (_zs, rs) = interp(&shr_pts, tm);
            let r_span = (rs - rh).abs();

            // Z-extent: covers both this station and next (with passage height)
            let z_lo = sta_zh[si]
                .min(sta_zs[si])
                .min(sta_zh[si + 1])
                .min(sta_zs[si + 1]);
            let z_hi = sta_zh[si]
                .max(sta_zs[si])
                .max(sta_zh[si + 1])
                .max(sta_zs[si + 1]);
            let el_h = (z_hi - z_lo).max(2.0);

            // LE taper: thinner at leading edge station
            let r_elem = if si == 0 { le_r } else { blade_r };
            let thick = r_elem * 2.0;

            if r_span <= r_threshold {
                // Z-dominated (inlet/turn): thin cylinder at mid-radius
                let rm = (rh + rs) / 2.0;
                let cx = rm * theta.cos();
                let cy = rm * theta.sin();
                wheel = wheel.union(&cyl_at(cx, cy, z_lo, el_h, r_elem)).into();
            } else {
                // R-dominated (exit): axis-aligned bounding box of blade section
                // The blade cross-section is a thin rectangle from r_hub to r_shroud,
                // rotated by angle θ. The bounding box is:
                //   dx = |cos θ| × r_range + |sin θ| × thickness
                //   dy = |sin θ| × r_range + |cos θ| × thickness
                let r_lo_val = rh.min(rs);
                let r_hi_val = rh.max(rs);
                let r_mid = (r_lo_val + r_hi_val) / 2.0;
                let r_range = r_hi_val - r_lo_val;

                let dx = theta.cos().abs() * r_range + theta.sin().abs() * thick;
                let dy = theta.sin().abs() * r_range + theta.cos().abs() * thick;

                let cx = r_mid * theta.cos();
                let cy = r_mid * theta.sin();

                wheel = wheel
                    .union(&box_at(cx, cy, z_lo, dx.max(thick), dy.max(thick), el_h))
                    .into();
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // Export STEP
    // ═══════════════════════════════════════════════════════════════════

    wheel
        .write_step("output/turbogen_turbine_wheel.stp")
        .expect("Failed to write turbine wheel STEP");

    println!();
    println!("=== Turbogenerator Turbine Wheel (Investment Cast) ===");
    println!("Exported: output/turbogen_turbine_wheel.stp");
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
        "Blade thickness: {}mm LE → {}mm body",
        TG_TURB_BLADE_LE_THICKNESS, TG_TURB_BLADE_THICKNESS
    );
    println!("Hub:    H1(0,37.5) → H2(8,32) → H3(20,22) → H4(35,14) → H5(45,10)");
    println!("Shroud: S1(12,37.5) → S2(18,34) → S3(28,30) → S4(38,27.5) → S5(45,27.5)");
    println!();
    println!("Manufacturing: Investment casting (NOT CNC).");
    println!("  Material: Inconel 713C (γ' strengthened, designed for casting, 950°C capable)");
    println!("  Foundry applies ~2% shrinkage compensation for 713C.");
    println!("  Pattern: foundry SLA or wax injection from this geometry.");
    println!("  Post-cast: HIP + solution treat + age, then balance to G2.5.");
}
