use vcad::{centered_cylinder, Part};

// ─── BCC Microlattice Swab Head ───
//
// 3D-printed (Elegoo Saturn 4 Ultra 16K resin) diagnostic sampling swab
// with a body-centered cubic (BCC) microlattice head for maximum surface
// area collection. The lattice structure traps cells and fluid in its
// interstitial spaces, improving sample yield over smooth swabs.
//
// Design:
// - Lattice head: 3mm dia cylinder, 15mm long, filled with BCC lattice
//   (1.0mm unit cells, 0.3mm strut diameter)
// - Handle shaft: 2mm dia cylinder, 60mm long
// - Breakpoint notch: 0.8mm neck at 75mm from tip (snap-off into tube)
// - Total length: ~80mm
//
// BCC lattice: Each unit cell has 8 corner vertices and 1 body center.
// Struts connect each corner to the body center (8 struts per cell).
// Cells tile in X/Y/Z within the cylindrical head envelope.
//
// Material: biocompatible resin (e.g., Elegoo ABS-like or bio-resin)
// Printer: Elegoo Saturn 4 Ultra 16K (12μm XY resolution)

fn main() {
    // ── Dimensions ──

    let head_diameter = 3.0; // mm
    let head_length = 15.0; // mm
    let head_radius = head_diameter / 2.0;

    let shaft_diameter = 2.0; // mm
    let shaft_length = 60.0; // mm

    let unit_cell = 1.0; // mm BCC unit cell size
    let strut_diameter = 0.3; // mm
    let strut_radius = strut_diameter / 2.0;

    let breakpoint_diameter = 0.8; // mm
    let breakpoint_length = 1.0; // mm (short neck region)

    // Total: head (15) + transition (4) + shaft (60) + break zone (1) = 80mm
    let transition_length = 4.0; // taper zone between head and shaft

    // ── Build lattice head ──
    // Generate BCC struts within the head cylinder envelope.
    // Head is centered at origin, extending along Z axis.

    // Number of cells along each axis
    let cells_x = ((head_diameter / unit_cell) as f64).ceil() as i32 + 1;
    let cells_y = cells_x;
    let cells_z = ((head_length / unit_cell) as f64).ceil() as i32;

    // Offset so lattice is centered on head cylinder
    let offset_x = -(cells_x as f64 * unit_cell) / 2.0;
    let offset_y = -(cells_y as f64 * unit_cell) / 2.0;
    let offset_z = -(cells_z as f64 * unit_cell) / 2.0;

    let mut lattice = Part::empty("lattice");

    for iz in 0..cells_z {
        for iy in 0..cells_y {
            for ix in 0..cells_x {
                // Cell origin (corner)
                let cx = offset_x + (ix as f64) * unit_cell;
                let cy = offset_y + (iy as f64) * unit_cell;
                let cz = offset_z + (iz as f64) * unit_cell;

                // Body center of this cell
                let bx = cx + unit_cell / 2.0;
                let by = cy + unit_cell / 2.0;
                let bz = cz + unit_cell / 2.0;

                // Check if body center is within the head cylinder radius
                let r = (bx * bx + by * by).sqrt();
                if r > head_radius + unit_cell * 0.5 {
                    continue;
                }

                // BCC struts: connect each corner to body center
                // 8 corners of the unit cell
                let corners: [(f64, f64, f64); 8] = [
                    (cx, cy, cz),
                    (cx + unit_cell, cy, cz),
                    (cx, cy + unit_cell, cz),
                    (cx + unit_cell, cy + unit_cell, cz),
                    (cx, cy, cz + unit_cell),
                    (cx + unit_cell, cy, cz + unit_cell),
                    (cx, cy + unit_cell, cz + unit_cell),
                    (cx + unit_cell, cy + unit_cell, cz + unit_cell),
                ];

                for (ci, &(px, py, pz)) in corners.iter().enumerate() {
                    // Strut from corner to body center
                    let dx = bx - px;
                    let dy = by - py;
                    let dz = bz - pz;
                    let length = (dx * dx + dy * dy + dz * dz).sqrt();

                    if length < 0.01 {
                        continue;
                    }

                    // Midpoint of strut
                    let mx = (px + bx) / 2.0;
                    let my = (py + by) / 2.0;
                    let mz = (pz + bz) / 2.0;

                    // Rotation angles to orient the cylinder from Z-axis to strut direction
                    // Elevation angle from Z axis
                    let elevation = (dz / length).acos().to_degrees();
                    // Azimuth angle in XY plane
                    let azimuth = dy.atan2(dx).to_degrees();

                    let strut = centered_cylinder(
                        &format!("strut_{iz}_{iy}_{ix}_{ci}"),
                        strut_radius,
                        length,
                        6, // low poly for performance (many struts)
                    )
                    .rotate(0.0, elevation, azimuth)
                    .translate(mx, my, mz);

                    lattice = lattice + strut;
                }
            }
        }
    }

    // Clip lattice to head cylinder
    let head_envelope = centered_cylinder(
        "head_envelope",
        head_radius,
        head_length,
        32,
    );

    let lattice_head = lattice & head_envelope;

    // ── Handle shaft ──
    // Shaft extends below the head along -Z

    let shaft = centered_cylinder(
        "shaft",
        shaft_diameter / 2.0,
        shaft_length,
        24,
    )
    .translate(0.0, 0.0, -(head_length / 2.0 + transition_length + shaft_length / 2.0));

    // ── Transition taper ──
    // Cone from head diameter to shaft diameter
    let taper = Part::cone(
        "taper",
        head_radius,
        shaft_diameter / 2.0,
        transition_length,
        24,
    )
    .translate(0.0, 0.0, -(head_length / 2.0 + transition_length / 2.0));

    // ── Breakpoint notch ──
    // Located at 75mm from tip = shaft end region
    // Tip is at +Z (top of head), so breakpoint is at:
    // head/2 + transition + shaft - (total - 75) from center of head
    let total_length = head_length + transition_length + shaft_length + breakpoint_length;
    let tip_z = head_length / 2.0;
    let break_z = tip_z - 75.0;

    // Create the notch by subtracting a ring at the breakpoint
    let break_full = centered_cylinder(
        "break_full",
        shaft_diameter / 2.0 + 0.1,
        breakpoint_length,
        24,
    )
    .translate(0.0, 0.0, break_z);

    let break_core = centered_cylinder(
        "break_core",
        breakpoint_diameter / 2.0,
        breakpoint_length + 0.2,
        24,
    )
    .translate(0.0, 0.0, break_z);

    let notch_ring = break_full - break_core;

    // ── Short tail beyond breakpoint ──
    let tail_length = total_length - 75.0 - breakpoint_length;
    let tail = centered_cylinder(
        "tail",
        shaft_diameter / 2.0,
        tail_length,
        24,
    )
    .translate(0.0, 0.0, break_z - breakpoint_length / 2.0 - tail_length / 2.0);

    // ── Assemble ──

    let swab = lattice_head + taper + shaft + tail - notch_ring;

    // ── Export ──

    swab.write_stl("output/microlattice_swab.stl").unwrap();

    println!("Exported: output/microlattice_swab.stl");
    println!();
    println!("── Microlattice Swab Specs ──");
    println!("  Total length:   {total_length:.1}mm");
    println!("  Lattice head:   {head_diameter:.1}mm dia x {head_length:.1}mm");
    println!("  Unit cell:      {unit_cell:.1}mm BCC");
    println!("  Strut dia:      {strut_diameter:.1}mm");
    println!("  Handle shaft:   {shaft_diameter:.1}mm dia x {shaft_length:.1}mm");
    println!("  Breakpoint:     {breakpoint_diameter:.1}mm neck at 75mm from tip");
    println!("  Lattice cells:  {cells_x} x {cells_y} x {cells_z}");
    println!("  Printer:        Elegoo Saturn 4 Ultra 16K");
    println!("  Material:       biocompatible resin");
}
