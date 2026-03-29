use glam::dvec3;
use laminarforge_cad::*;
use opencascade::primitives::Shape;
use opencascade::workplane::Workplane;

// ─── Turbogenerator Shaft — STEP Export for CNC Quoting ───
//
// Reference: Artifact A-D45A776A (Shaft Design and Stress Analysis)
// Material: 4140 Q&T steel
// Total length: 198mm, main diameter: 12mm
// Overhung-overhung configuration with S6001 ceramic bearings
// 110mm bearing center-to-center span
//
// CNC operations: Turning + precision grinding (bearing seats h5, wheel seats g6)
// Threads: M8x1.0 LH (compressor end), M8x1.0 RH (turbine end)
// Thermal choke: 8mm diameter section near rear bearing

/// Create a cylinder from z_bottom upward, centered on the Z-axis
fn cyl_z(z_bottom: f64, height: f64, radius: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(0.0, 0.0, z_bottom))
        .circle(0.0, 0.0, radius);
    wire.to_face().extrude(dvec3(0.0, 0.0, height)).into()
}

fn main() {
    // ── Shaft radii ──
    let r_main = TG_SHAFT_DIA / 2.0;        // 6.0mm
    let r_thread = TG_SHAFT_THREAD_DIA / 2.0; // 4.0mm
    let r_choke = TG_SHAFT_CHOKE_DIA / 2.0;   // 4.0mm

    // ── Axial stations (z=0 at front tip, +Z toward rear) ──
    //
    // Layout:
    //   [Thread LH][Comp wheel seat][Brg1][---Main body---][Choke][Brg2][Spacer][Turb seat][Thread RH]
    //   z=0    z=15  z=16      z=36  z=44           z=128  z=143 z=151 z=154  z=180  z=183    z=198
    //

    // Section definitions: (z_start, height, radius)
    let sections: &[(f64, f64, f64)] = &[
        // Front M8 LH thread (simplified as reduced diameter)
        (0.0, 15.0, r_thread),
        // Step: short transition to full diameter
        (15.0, 1.0, r_main),
        // Compressor wheel seat (g6 tolerance) + front overhang
        (16.0, 20.0, r_main),
        // Front bearing seat (h5 tolerance, S6001 = 8mm wide)
        (36.0, 8.0, r_main),
        // Main body between bearings
        (44.0, 84.0, r_main),
        // Thermal choke (8mm dia, 15mm long — isolates rear bearing from turbine heat)
        (128.0, 15.0, r_choke),
        // Post-choke body
        (143.0, 8.0, r_main),
        // Rear bearing seat (h5 tolerance)
        (151.0, 8.0, r_main),
        // Ceramic spacer zone
        (159.0, 3.0, r_main),
        // Turbine wheel seat (g6 tolerance)
        (162.0, 18.0, r_main),
        // Step: transition to thread
        (180.0, 3.0, r_main),
        // Rear M8 RH thread
        (183.0, 15.0, r_thread),
    ];

    // Build shaft by unioning all sections
    let mut shaft: Shape = cyl_z(sections[0].0, sections[0].1, sections[0].2);

    for section in &sections[1..] {
        let cyl = cyl_z(section.0, section.1, section.2);
        shaft = shaft.union(&cyl).into();
    }

    // Export STEP
    shaft
        .write_step("output/turbogen_shaft.stp")
        .expect("Failed to write shaft STEP");

    println!("=== Turbogenerator Shaft ===");
    println!("Exported: output/turbogen_shaft.stp");
    println!("Material: 4140 Q&T steel");
    println!("Total length: {}mm", TG_SHAFT_LENGTH);
    println!("Main diameter: {}mm", TG_SHAFT_DIA);
    println!("Thread diameter: {}mm (M8x1.0)", TG_SHAFT_THREAD_DIA);
    println!("Thermal choke: {}mm dia x 15mm long", TG_SHAFT_CHOKE_DIA);
    println!("Bearing seats: h5 tolerance (precision ground)");
    println!("Wheel seats: g6 tolerance");
    println!("Threads: M8x1.0 LH (front/compressor), M8x1.0 RH (rear/turbine)");
}
