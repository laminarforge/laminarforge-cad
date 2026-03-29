use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Compressor Wheel — STEP Export for CNC Quoting ───
//
// Reference: Artifact A-839C84B9 (Compressor Wheel Aerodynamic Design)
// Material: 7075-T6 aluminum (BOM B1)
// Billet: 4" round
// 80mm tip diameter, 7+7 splitter blade configuration
// Open (unshrouded) impeller, CNC from billet
//
// CNC operations: 5-axis simultaneous milling
// This model captures the hub profile and blade envelope for quoting.
// The 5-axis shop will program exact blade surfaces from the aero spec.

/// Cylinder from z_bottom, along +Z
fn cyl_z(z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(0.0, 0.0, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

/// Cylinder at (cx, cy) offset
fn cyl_at(cx: f64, cy: f64, z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

/// Rectangular box at (cx, cy, z_bottom)
fn box_at(cx: f64, cy: f64, z_bottom: f64, width: f64, depth: f64, height: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, z_bottom))
        .rect(width, depth);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

fn main() {
    // Orientation: Z-axis = shaft axis, z=0 at inducer eye (front face)
    // +Z toward exit (rear/diffuser side)

    let tip_r = TG_COMP_TIP_RADIUS;      // 40mm
    let inducer_tip_r = TG_COMP_INDUCER_TIP_DIA / 2.0; // 24mm
    let blade_height = TG_COMP_EXIT_BLADE_HEIGHT; // 5.5mm at exit
    let blade_count = TG_COMP_BLADE_COUNT; // 7 main
    let splitter_count = TG_COMP_SPLITTER_COUNT; // 7 splitter
    let total_blades = blade_count + splitter_count; // 14 total
    let blade_thickness = TG_COMP_BLADE_THICKNESS; // 1.5mm

    // ── Hub profile ──
    // Inducer (front): hub radius ~12mm, tip at 24mm
    // Exit (rear): hub radius ~34.5mm (tip_r - blade_height = 40 - 5.5)
    // Hub spans from z=0 to z=28mm (axial depth of wheel)

    let hub_front_r = 12.0;   // hub radius at inducer
    let hub_rear_r = tip_r - blade_height; // 34.5mm at exit
    let wheel_depth = 28.0;   // axial depth from inducer to exit plane

    // Hub modeled as stepped cylinders (approximation of the curved hub contour)
    let hub_steps = 10;
    let mut hub: Shape = cyl_z(0.0, wheel_depth / (hub_steps as f64), hub_front_r);

    for step in 1..hub_steps {
        let frac = step as f64 / hub_steps as f64;
        let z = frac * wheel_depth;
        let r = hub_front_r + (hub_rear_r - hub_front_r) * frac;
        let dz = wheel_depth / hub_steps as f64;
        let section = cyl_z(z, dz, r);
        hub = hub.union(&section).into();
    }

    // ── Back disk (behind exit plane) ──
    // Solid disk that the shaft bore goes through
    let back_disk_thickness = 7.0;
    let back_disk = cyl_z(wheel_depth, back_disk_thickness, tip_r);
    hub = hub.union(&back_disk).into();

    // ── Shaft bore (M8 LH thread + 12mm bore) ──
    let shaft_bore = cyl_z(-0.5, wheel_depth + back_disk_thickness + 1.0, TG_SHAFT_DIA / 2.0);
    hub = hub.subtract(&shaft_bore).into();

    // ── Blades ──
    // 7 main blades extending from hub to tip, full length (z=0 to z=wheel_depth)
    // 7 splitter blades starting at ~50% chord (z=wheel_depth/2 to z=wheel_depth)
    //
    // Each blade modeled as a thin rectangular extrusion at the correct angular
    // position. The blade extends radially from hub to tip. For quoting purposes,
    // this captures blade count, spacing, thickness, and radial extent.
    //
    // NOTE: Real blades have 3D compound curvature with backsweep and twist.
    // The 5-axis shop will program from the aerodynamic specification (A-839C84B9),
    // not from this simplified STEP model.

    for i in 0..blade_count {
        let angle = (i as f64) * 360.0 / (blade_count as f64);
        let angle_rad = angle.to_radians();

        // Main blade: full length, radial from hub to tip
        let blade_r = (hub_front_r + tip_r) / 2.0; // midpoint
        let blade_length = tip_r - hub_front_r; // radial extent
        let cx = blade_r * angle_rad.cos();
        let cy = blade_r * angle_rad.sin();

        // Blade as a thin box, rotated to radial direction
        // Approximate: blade extends in the radial direction at this angle
        let blade = box_at(cx, cy, 0.0, blade_thickness, blade_length, wheel_depth);
        hub = hub.union(&blade).into();
    }

    // Splitter blades (start at 50% chord)
    for i in 0..splitter_count {
        let angle = (i as f64) * 360.0 / (splitter_count as f64) + 360.0 / (total_blades as f64);
        let angle_rad = angle.to_radians();

        let blade_r = (hub_rear_r + tip_r) / 2.0;
        let blade_length = tip_r - hub_rear_r;
        let cx = blade_r * angle_rad.cos();
        let cy = blade_r * angle_rad.sin();

        let splitter = box_at(cx, cy, wheel_depth / 2.0, blade_thickness, blade_length, wheel_depth / 2.0);
        hub = hub.union(&splitter).into();
    }

    // ── Export ──
    hub.write_step("output/turbogen_compressor_wheel.stp")
        .expect("Failed to write compressor wheel STEP");

    println!("=== Turbogenerator Compressor Wheel ===");
    println!("Exported: output/turbogen_compressor_wheel.stp");
    println!("Material: 7075-T6 aluminum");
    println!("Tip diameter: {}mm", TG_COMP_TIP_DIA);
    println!("Inducer eye: {}mm", TG_COMP_INDUCER_TIP_DIA);
    println!("Blade count: {} main + {} splitter = {} total", blade_count, splitter_count, total_blades);
    println!("Exit blade height: {}mm", blade_height);
    println!("Blade thickness: {}mm body, {}mm LE", TG_COMP_BLADE_THICKNESS, TG_COMP_BLADE_LE_THICKNESS);
    println!("NOTE: Blade surfaces are simplified rectangles for quoting envelope.");
    println!("      5-axis shop programs from aerodynamic spec (A-839C84B9).");
}
