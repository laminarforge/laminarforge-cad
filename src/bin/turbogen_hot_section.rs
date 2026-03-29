use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Hot Section Housing — STEP Export for CNC Quoting ───
//
// Reference: Artifact A-58E55C4B (Housing CAD Design, §6)
// Material: 6061-T6 aluminum
// Billet: 5" (127mm) round x 70mm long
// Contains: turbine scroll/nozzle, turbine shroud bore, thermally-isolated
//           rear bearing support, exhaust cone, front flange with bolt holes
//
// CNC operations: Lathe (bore contours) + 3-axis mill with 4th-axis rotary (scroll)

/// Cylinder from z_bottom, along +Z
fn cyl_z(z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(0.0, 0.0, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

/// Cylinder at an (x, y) offset from center
fn cyl_at(cx: f64, cy: f64, z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

fn main() {
    // Orientation: Z-axis = engine axis, z=0 at front flange face, +Z toward rear
    let housing_od = TG_HOT_MAX_OD;    // 120mm
    let housing_r = housing_od / 2.0;   // 60mm
    let length = TG_HOT_LENGTH;          // 55mm

    // ── Main body: solid cylinder ──
    let mut housing: Shape = cyl_z(0.0, length, housing_r);

    // ── Turbine scroll (z=0 to z=20, wrapping 360°) ──
    // Circular cross-sections, max diameter 31.3mm at 360°
    // A/R = 1.71 cm
    let scroll_inner_r = TG_TURB_SCROLL_INNER_R; // 40.1mm
    let scroll_max_dia = TG_TURB_SCROLL_MAX_DIA; // 31.3mm

    let num_segments = 36;
    for i in 1..=num_segments {
        let theta = (i as f64) / (num_segments as f64);
        let angle_deg = theta * 360.0;
        let angle_rad = angle_deg.to_radians();

        // Cross-section diameter scales linearly (circular sections)
        let pocket_dia = scroll_max_dia * theta;
        if pocket_dia < 2.0 {
            continue;
        }
        let pocket_r = pocket_dia / 2.0;

        // Scroll centroid at r ~ 45mm from axis (inner wall + half diameter)
        let centroid_r = scroll_inner_r + pocket_r;
        let cx = centroid_r * angle_rad.cos();
        let cy = centroid_r * angle_rad.sin();

        // Scroll passage centered axially at z=10mm (front region of housing)
        let pocket_z = 10.0 - pocket_r; // center at z=10
        let pocket_height = pocket_dia;

        let pocket = cyl_at(cx, cy, pocket_z.max(0.0), pocket_height.min(20.0), pocket_r);
        housing = housing.subtract(&pocket).into();
    }

    // ── Turbine shroud bore (z=20 to z=35) ──
    // 76mm bore (75mm wheel + 0.5mm clearance each side)
    let turbine_bore = cyl_z(19.5, 16.0, TG_TURB_BORE_ID / 2.0);
    housing = housing.subtract(&turbine_bore).into();

    // ── Vaneless gap (z=10 to z=22, r=37.5 to r=40.1) ──
    // Annular slot connecting scroll to turbine inlet, 12mm axial height
    let vaneless_bore = cyl_z(9.5, 13.0, TG_TURB_SCROLL_INNER_R);
    housing = housing.subtract(&vaneless_bore).into();

    // ── Exhaust cone (z=35 to z=55) ──
    // 55mm ID at turbine exit, expanding to 65mm at exhaust
    // Simplified as stepped cylindrical bores
    let exhaust_bore_1 = cyl_z(34.5, 10.0, 27.5); // inner radius at turbine exit
    housing = housing.subtract(&exhaust_bore_1).into();
    let exhaust_bore_2 = cyl_z(44.5, 11.0, TG_EXHAUST_CONE_EXIT_DIA / 2.0);
    housing = housing.subtract(&exhaust_bore_2).into();

    // ── Rear bearing bore (from rear face) ──
    // 28mm H7, 10mm deep, with thermal isolation
    let bearing_bore = cyl_z(length - TG_BEARING_BORE_DEPTH, TG_BEARING_BORE_DEPTH + 0.5, TG_BEARING_BORE_DIA / 2.0);
    housing = housing.subtract(&bearing_bore).into();

    // ── Thermal isolation pockets around rear bearing ──
    // 3 thin webs at 120° spacing, 2mm circumferential air gap
    // Model as 3 large pockets between the webs
    let isolation_inner_r = TG_BEARING_BORE_DIA / 2.0 + 1.0; // 15mm
    let isolation_outer_r = isolation_inner_r + 8.0; // 23mm
    for i in 0..3 {
        // Each pocket spans ~100° (leaving 20° webs)
        let center_angle = (i as f64) * 120.0 + 60.0; // offset from web centers
        let angle_rad = center_angle.to_radians();
        let pocket_cx = ((isolation_inner_r + isolation_outer_r) / 2.0) * angle_rad.cos();
        let pocket_cy = ((isolation_inner_r + isolation_outer_r) / 2.0) * angle_rad.sin();

        let pocket = cyl_at(
            pocket_cx, pocket_cy,
            length - TG_BEARING_BORE_DEPTH - 2.0,
            TG_BEARING_BORE_DEPTH + 2.5,
            4.0,
        );
        housing = housing.subtract(&pocket).into();
    }

    // ── Shaft pass-through (center, full length) ──
    let shaft_hole = cyl_z(-0.5, length + 1.0, 6.25);
    housing = housing.subtract(&shaft_hole).into();

    // ── Shaft tunnel press-fit bore (front face) ──
    let tunnel_bore = cyl_z(-0.5, 15.5, TG_SHAFT_TUNNEL_OD / 2.0);
    housing = housing.subtract(&tunnel_bore).into();

    // ── Front flange bolt holes (6x M6 on 100mm PCD) ──
    let bolt_r = TG_BOLT_PCD / 2.0;
    let bolt_hole_r = TG_BOLT_CLEARANCE_DIA / 2.0;
    for i in 0..TG_BOLT_COUNT {
        let angle = (i as f64) * 360.0 / (TG_BOLT_COUNT as f64);
        let angle_rad = angle.to_radians();
        let bx = bolt_r * angle_rad.cos();
        let by = bolt_r * angle_rad.sin();
        let bolt = cyl_at(bx, by, -0.5, 16.0, bolt_hole_r);
        housing = housing.subtract(&bolt).into();
    }

    // ── Export ──
    housing
        .write_step("output/turbogen_hot_section.stp")
        .expect("Failed to write hot section STEP");

    println!("=== Turbogenerator Hot Section Housing ===");
    println!("Exported: output/turbogen_hot_section.stp");
    println!("Material: 6061-T6 aluminum");
    println!("Billet: {:.1}mm round x {:.0}mm long", TG_HOT_BILLET_DIA, TG_HOT_BILLET_LENGTH);
    println!("Finished OD: {:.0}mm, Length: {:.0}mm", housing_od, length);
    println!("Features: turbine scroll (36 segments), turbine shroud bore,");
    println!("          vaneless nozzle gap, exhaust cone, rear bearing bore (28mm H7),");
    println!("          thermal isolation pockets, shaft tunnel bore,");
    println!("          6x M6 bolt holes on {}mm PCD", TG_BOLT_PCD);
    println!("WARNING: Hot section operates at 200-300C. Monitor for aluminum softening.");
}
