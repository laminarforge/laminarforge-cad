use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Turbine Wheel — STEP Export for CNC Quoting ───
//
// Reference: Artifact A-EE749EE3 (GG Turbine Wheel Aerodynamic Design)
// Material: Inconel 718
// Billet: 3.5" round
// 75mm tip diameter, 13 radial inflow blades
// Gas enters radially, exits axially
//
// CNC operations: 5-axis simultaneous milling (Inconel — flood coolant required)
// This model captures hub + blade envelope for quoting.

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
    // Orientation: Z-axis = shaft axis
    // z=0 at radial inlet plane (front), +Z toward axial exit (rear)
    // Gas enters radially at z=0..12mm (blade height), exits axially at rear

    let tip_r = TG_TURB_TIP_RADIUS;       // 37.5mm
    let inlet_height = TG_TURB_INLET_BLADE_HEIGHT; // 12mm
    let blade_count = TG_TURB_BLADE_COUNT; // 13
    let exit_tip_r = TG_TURB_EXIT_TIP_RADIUS; // 27.5mm
    let exit_hub_r = TG_TURB_EXIT_HUB_RADIUS; // 10mm

    // ── Hub profile ──
    // Radial inflow turbine: gas enters radially at the periphery
    // Hub is a complex 3D shape — disk at inlet, transitioning to annular at exit
    //
    // Simplified as: back disk + conical hub

    let wheel_depth = 20.0; // axial depth from inlet plane to exit plane

    // Back disk (structural, connects to shaft)
    let back_disk_r = tip_r;
    let back_disk_thickness = 5.0;
    let mut wheel: Shape = cyl_z(wheel_depth, back_disk_thickness, back_disk_r);

    // Hub body: stepped cone from inlet to exit
    let hub_steps = 8;
    for step in 0..hub_steps {
        let frac = step as f64 / hub_steps as f64;
        let z = frac * wheel_depth;
        let dz = wheel_depth / hub_steps as f64;

        // Hub radius varies from ~30mm at inlet to exit_hub_r at exit
        let hub_r_at_inlet = tip_r - inlet_height; // 37.5 - 12 = 25.5mm
        let r = hub_r_at_inlet + (exit_hub_r - hub_r_at_inlet) * frac;

        // Outer radius: tip at inlet, exit_tip at exit
        let outer_r = tip_r + (exit_tip_r - tip_r) * frac;

        let section = cyl_z(z, dz, outer_r);
        wheel = wheel.union(&section).into();
    }

    // ── Shaft bore ──
    let shaft_bore = cyl_z(-0.5, wheel_depth + back_disk_thickness + 1.0, TG_SHAFT_DIA / 2.0);
    wheel = wheel.subtract(&shaft_bore).into();

    // ── Flow passage (the space between hub and shroud where gas flows) ──
    // At inlet: annular gap from r=25.5 to r=37.5, height=12mm
    // At exit: annular gap from r=10 to r=27.5
    // This is where the blade channels are carved

    // ── Blades ──
    // 13 radial inflow blades
    // Each blade extends from hub to shroud, curving from radial to axial
    //
    // Simplified: thin rectangular blades at equal angular spacing
    // positioned at the inlet radius, extending the blade height

    let blade_thickness = 1.5; // mm

    for i in 0..blade_count {
        let angle = (i as f64) * 360.0 / (blade_count as f64);
        let angle_rad = angle.to_radians();

        // Blade center at mid-radius of the passage
        let blade_r = (exit_hub_r + tip_r) / 2.0; // ~24mm
        let blade_radial_length = tip_r - exit_hub_r; // 27.5mm

        let cx = blade_r * angle_rad.cos();
        let cy = blade_r * angle_rad.sin();

        // Blade as thin box extending in the radial direction
        let blade = box_at(cx, cy, 0.0, blade_thickness, blade_radial_length, wheel_depth);
        wheel = wheel.union(&blade).into();
    }

    // ── Export ──
    wheel
        .write_step("output/turbogen_turbine_wheel.stp")
        .expect("Failed to write turbine wheel STEP");

    println!("=== Turbogenerator Turbine Wheel ===");
    println!("Exported: output/turbogen_turbine_wheel.stp");
    println!("Material: Inconel 718");
    println!("Tip diameter: {}mm", TG_TURB_TIP_DIA);
    println!("Blade count: {}", blade_count);
    println!("Inlet blade height: {}mm", inlet_height);
    println!("Exit tip radius: {}mm, hub radius: {}mm", exit_tip_r, exit_hub_r);
    println!("NOTE: Blade surfaces are simplified for quoting envelope.");
    println!("      5-axis shop programs from aerodynamic spec (A-EE749EE3).");
    println!("WARNING: Inconel 718 requires flood coolant, rigid setup, specialty tooling.");
}
