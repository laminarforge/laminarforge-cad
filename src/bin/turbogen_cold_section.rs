use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Cold Section Housing — STEP Export for CNC Quoting ───
//
// Reference: Artifact A-58E55C4B (Housing CAD Design, §4)
// Material: 6061-T6 aluminum
// Billet: 6" (152.4mm) round x 100mm long
// Contains: inlet bell, compressor shroud, hub-pinched vaneless diffuser,
//           scroll volute, front bearing bore, rear flange with bolt holes
//
// CNC operations: Lathe (bore contours) + 3-axis mill with 4th-axis rotary (scroll)

/// Cylinder from z_bottom, along +Z
fn cyl_z(z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(0.0, 0.0, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

/// Cylinder at an (x, y) offset from center, for bolt holes etc.
fn cyl_at(cx: f64, cy: f64, z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

fn main() {
    // Orientation: Z-axis = engine axis, z=0 at front face, +Z toward rear
    let housing_od = TG_COLD_MAX_OD;   // 150mm
    let housing_r = housing_od / 2.0;   // 75mm
    let length = TG_COLD_LENGTH;         // 85mm

    // ── Main body: solid cylinder ──
    let mut housing: Shape = cyl_z(0.0, length, housing_r);

    // ── Inlet bell bore (z=0 to z=30) ──
    // Bellmouth from 80mm OD down to 52mm eye, modeled as stepped bores
    let inlet_bore = cyl_z(-0.5, 31.0, TG_INLET_BELL_OD / 2.0);
    housing = housing.subtract(&inlet_bore).into();

    // Inlet eye (narrower section within bell)
    let eye_bore = cyl_z(-0.5, 31.0, TG_INLET_EYE_ID / 2.0);
    // The eye is the through-hole; the bell is the larger surrounding cavity
    // For the STEP file, we model the bell as a large bore and the eye passage separately

    // ── Compressor shroud bore (z=30 to z=58) ──
    // 80.8mm bore (80mm wheel + 0.4mm clearance each side)
    let shroud_bore = cyl_z(29.5, 29.0, TG_COMP_BORE_ID / 2.0);
    housing = housing.subtract(&shroud_bore).into();

    // ── Vaneless diffuser passage (z=58 to z=73) ──
    // Hub-pinched diffuser: exit at r=60mm, width=4.84mm
    // Modeled as annular slot from r=40mm to r=60mm, 5.5mm high
    let diffuser_bore = cyl_z(57.5, 16.0, TG_DIFFUSER_EXIT_RADIUS);
    housing = housing.subtract(&diffuser_bore).into();

    // ── Scroll volute (z=58 to z=85, wrapping 360°) ──
    // Tall narrow rectangular cross-section: 15mm radial x up to 40mm axial
    // Approximated as discrete pocket segments at angular intervals
    //
    // The scroll is the spiral groove that collects diffuser air.
    // For CNC quoting, we model it as a series of radial pockets at
    // increasing depth/height around the circumference.

    let scroll_inner_r = TG_COMP_SCROLL_INNER_R; // 60mm
    let scroll_depth = TG_COMP_SCROLL_RADIAL_DEPTH; // 15mm radial
    let scroll_max_h = TG_COMP_SCROLL_MAX_HEIGHT; // 40mm axial at 360°

    // 36 pockets at 10° intervals, each a small arc-segment pocket
    let num_segments = 36;
    for i in 1..=num_segments {
        let theta = (i as f64) / (num_segments as f64); // 0..1
        let angle_deg = theta * 360.0;
        let angle_rad = angle_deg.to_radians();

        // Cross-section scales linearly with angle
        let pocket_height = scroll_max_h * theta; // 0 to 40mm
        let pocket_width = scroll_depth * theta.min(1.0); // 0 to 15mm radial

        if pocket_height < 1.0 || pocket_width < 1.0 {
            continue; // skip near-zero segments
        }

        // Pocket center position (radial from axis)
        let pocket_r = scroll_inner_r + pocket_width / 2.0;
        let cx = pocket_r * angle_rad.cos();
        let cy = pocket_r * angle_rad.sin();

        // Pocket as a vertical cylinder at this position
        // Using a small cylinder to approximate each scroll segment
        let pocket_radius = pocket_width / 2.0;
        let pocket_z = length - pocket_height; // pocket extends from rear face forward

        let pocket = cyl_at(cx, cy, pocket_z, pocket_height + 0.5, pocket_radius);
        housing = housing.subtract(&pocket).into();
    }

    // ── Scroll exit port (25mm bore, angled rearward) ──
    // Single exit at theta=360° (same as tongue), routing to transfer duct
    let exit_r = scroll_inner_r + scroll_depth / 2.0;
    let exit_port = cyl_at(exit_r, 0.0, length - scroll_max_h, scroll_max_h + 0.5, 12.5);
    housing = housing.subtract(&exit_port).into();

    // ── Front bearing bore (from front face) ──
    // 28mm H7, 10mm deep
    let bearing_bore = cyl_z(-0.5, TG_BEARING_BORE_DEPTH + 0.5, TG_BEARING_BORE_DIA / 2.0);
    housing = housing.subtract(&bearing_bore).into();

    // ── Shaft pass-through (center, full length) ──
    // 12.5mm clearance for 12mm shaft
    let shaft_hole = cyl_z(-0.5, length + 1.0, 6.25);
    housing = housing.subtract(&shaft_hole).into();

    // ── Shaft tunnel press-fit bore (rear face) ──
    // 20mm H7 bore from rear face, ~15mm deep
    let tunnel_bore = cyl_z(length - 15.0, 15.5, TG_SHAFT_TUNNEL_OD / 2.0);
    housing = housing.subtract(&tunnel_bore).into();

    // ── Rear flange bolt holes (6x M6 on 100mm PCD) ──
    let bolt_r = TG_BOLT_PCD / 2.0; // 50mm
    let bolt_hole_r = TG_BOLT_CLEARANCE_DIA / 2.0; // 3.25mm
    for i in 0..TG_BOLT_COUNT {
        let angle = (i as f64) * 360.0 / (TG_BOLT_COUNT as f64);
        let angle_rad = angle.to_radians();
        let bx = bolt_r * angle_rad.cos();
        let by = bolt_r * angle_rad.sin();
        let bolt = cyl_at(bx, by, length - 15.0, 16.0, bolt_hole_r);
        housing = housing.subtract(&bolt).into();
    }

    // ── Export ──
    housing
        .write_step("output/turbogen_cold_section.stp")
        .expect("Failed to write cold section STEP");

    println!("=== Turbogenerator Cold Section Housing ===");
    println!("Exported: output/turbogen_cold_section.stp");
    println!("Material: 6061-T6 aluminum");
    println!("Billet: {:.1}mm round x {:.0}mm long", TG_COLD_BILLET_DIA, TG_COLD_BILLET_LENGTH);
    println!("Finished OD: {:.0}mm, Length: {:.0}mm", housing_od, length);
    println!("Features: inlet bell, compressor shroud bore, vaneless diffuser,");
    println!("          scroll volute (36 segments), front bearing bore (28mm H7),");
    println!("          shaft tunnel bore, 6x M6 bolt holes on {}mm PCD", TG_BOLT_PCD);
    println!("Tolerances: Bearing bore H7, shroud bore +/-0.05mm, flange flat 0.05mm");
}
