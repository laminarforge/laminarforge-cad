use vcad::{centered_cylinder, Part};

// ─── BCC Microlattice Swab — Genital Specimen Collection ───
//
// Literature-validated 3D-printed diagnostic sampling swab with BCC
// microlattice head optimized for genital mucosal specimen collection.
//
// Research basis (see artifact A-E5A1E1FD):
//   Lu et al. 2024, Nature Comm Eng — BCC/auxetic/dodecahedron lattice
//     swabs, 2.3x release volume, ~100% centrifugal recovery
//   Tooker et al. 2021, MRS Bulletin — 6 commercial 3D-printed swabs
//     vs Copan FLOQSwab, mechanical + functional benchmarks
//   van der Elst et al. 2020, Adv Eng Mater — SLA swab fabrication,
//     Formlabs Surgical Guide Resin, autoclave validation
//
// Genital vs nasopharyngeal adaptations:
//   - Tapered tip (2.6mm -> 3.0mm) for flat mucosal surfaces
//     instead of cylindrical head designed for nasal cavity navigation
//   - Same BCC lattice: capillary uptake + centrifugal release proven
//
// Geometry (literature-validated ranges in parentheses):
//   Total length:   150mm  (literature: >=150mm)
//   Lattice head:   18mm   (literature: 15-20mm)
//   Head diameter:  2.6mm tip -> 3.0mm base (tapered for genital anatomy)
//   Neck taper:     3mm transition to shaft
//   Shaft:          2.0mm dia (literature: 2.0-2.5mm)
//   Breakpoint:     0.8mm neck at 80mm from tip (literature: 70-80mm)
//   Tail:           69mm finger grip
//
// Lattice:
//   Topology: BCC (body-centered cubic) — best balance of printability,
//     flexibility, and sample capture (Lu 2024)
//   Unit cell: 0.8mm (literature: 0.5-1.0mm)
//   Strut dia: 0.3mm (literature: 0.2-0.4mm, ~6 layers at 50um)
//
// Printer: Elegoo Saturn 4 Ultra 16K
//   XY resolution: 14x19um
//   Build area: 218.88 x 122.88mm
//   Layer height: 50um (for 0.3mm struts = ~6 layers/strut diameter)
// Resin: Liqcreate Bio-Med Clear (ISO 10993 biocompatible)
// Post-process: IPA wash -> UV cure -> autoclave 121C 15min

fn main() {
    // ── Dimensions (validated against literature A-E5A1E1FD) ──

    let head_length = 18.0;          // mm  (literature: 15-20mm, mid-range)
    let head_tip_diameter = 2.6;     // mm  (tapered for genital anatomy)
    let head_base_diameter = 3.0;    // mm  (max, fits 12mm OD collector tube)
    let shaft_diameter = 2.0;        // mm  (literature: 2.0-2.5mm)
    let total_length = 150.0;        // mm  (literature: >=150mm)
    let breakpoint_from_tip = 80.0;  // mm  (literature: 70-80mm)

    let unit_cell = 0.8;            // mm  (literature: 0.5-1.0mm)
    let strut_diameter = 0.3;       // mm  (literature: 0.2-0.4mm)
    let strut_radius = strut_diameter / 2.0;

    let breakpoint_diameter = 0.8;   // mm  (snap-off neck)
    let breakpoint_length = 1.0;     // mm
    let neck_length = 3.0;           // mm  (taper from head to shaft)

    // Derived dimensions
    let head_tip_radius = head_tip_diameter / 2.0;
    let head_base_radius = head_base_diameter / 2.0;
    let shaft_radius = shaft_diameter / 2.0;

    // The shaft+tail is one continuous piece, with a notch subtracted
    // at the breakpoint. This avoids precision issues at segment boundaries.
    let shaft_total = total_length - head_length - neck_length;
    // = 150 - 18 - 3 = 129mm

    // Verify total length
    let computed_total = head_length + neck_length + shaft_total;
    let length_error: f64 = computed_total - total_length;
    assert!(
        length_error.abs() < 0.01,
        "Length mismatch: {computed_total:.1}mm != {total_length:.1}mm"
    );

    // ── Z-axis coordinate system ──
    // Head centered at origin: tip at +Z, base at -Z
    // Everything else extends downward (-Z)
    //
    //   z = +9.0   tip (top of head)
    //   z = -9.0   head base / neck top
    //   z = -12.0  neck bottom / shaft top
    //   z = -71.0  breakpoint center (80mm from tip)
    //   z = -141.0 bottom of tail
    //
    // Total: 9.0 + 141.0 = 150mm from tip to bottom

    let tip_z = head_length / 2.0; // +9.0

    // ── Build BCC lattice head ──
    // Generate struts within the head envelope, then clip to tapered cone.

    let cells_x = ((head_base_diameter / unit_cell) as f64).ceil() as i32 + 1;
    let cells_y = cells_x;
    let cells_z = ((head_length / unit_cell) as f64).ceil() as i32;

    let offset_x = -(cells_x as f64 * unit_cell) / 2.0;
    let offset_y = -(cells_y as f64 * unit_cell) / 2.0;
    let offset_z = -(cells_z as f64 * unit_cell) / 2.0;

    let mut lattice = Part::empty("lattice");

    for iz in 0..cells_z {
        for iy in 0..cells_y {
            for ix in 0..cells_x {
                let cx = offset_x + (ix as f64) * unit_cell;
                let cy = offset_y + (iy as f64) * unit_cell;
                let cz = offset_z + (iz as f64) * unit_cell;

                // Body center of this unit cell
                let bx = cx + unit_cell / 2.0;
                let by = cy + unit_cell / 2.0;
                let bz = cz + unit_cell / 2.0;

                // Skip cells clearly outside the head envelope
                let r = (bx * bx + by * by).sqrt();
                if r > head_base_radius + unit_cell * 0.5 {
                    continue;
                }

                // BCC: 8 struts from corners to body center
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
                    let dx = bx - px;
                    let dy = by - py;
                    let dz = bz - pz;
                    let length = (dx * dx + dy * dy + dz * dz).sqrt();

                    if length < 0.01 {
                        continue;
                    }

                    let mx = (px + bx) / 2.0;
                    let my = (py + by) / 2.0;
                    let mz = (pz + bz) / 2.0;

                    let elevation = (dz / length).acos().to_degrees();
                    let azimuth = dy.atan2(dx).to_degrees();

                    let strut = centered_cylinder(
                        &format!("s_{iz}_{iy}_{ix}_{ci}"),
                        strut_radius,
                        length,
                        6, // low-poly for performance (many struts)
                    )
                    .rotate(0.0, elevation, azimuth)
                    .translate(mx, my, mz);

                    lattice = lattice + strut;
                }
            }
        }
    }

    // Clip lattice to tapered head envelope (cone, not cylinder).
    // Taper: 3.0mm base (-Z) to 2.6mm tip (+Z) for genital anatomy.
    let head_envelope = Part::cone(
        "head_envelope",
        head_base_radius,  // bottom (-Z) = base, wider
        head_tip_radius,   // top (+Z) = tip, narrower
        head_length,
        32,
    );

    let lattice_head = lattice & head_envelope;

    // ── Neck taper: head base -> shaft ──
    // Transitions from 3.0mm (head base) down to 2.0mm (shaft)
    let neck = Part::cone(
        "neck",
        shaft_radius,       // bottom (-Z) = shaft side, narrower
        head_base_radius,   // top (+Z) = head base side, wider
        neck_length,
        24,
    )
    .translate(0.0, 0.0, -(head_length / 2.0 + neck_length / 2.0));

    // ── Continuous shaft + tail ──
    // One long cylinder from below neck to swab bottom.
    // The breakpoint notch is subtracted later.
    let shaft = centered_cylinder(
        "shaft",
        shaft_radius,
        shaft_total,
        24,
    )
    .translate(0.0, 0.0, -(head_length / 2.0 + neck_length + shaft_total / 2.0));

    // ── Breakpoint notch ──
    // Ring subtracted at 80mm from tip to create snap-off neck.
    // break_z = center of notch = tip position minus breakpoint distance
    let break_z = tip_z - breakpoint_from_tip; // 9.0 - 80.0 = -71.0

    let notch_outer = centered_cylinder(
        "notch_outer",
        shaft_radius + 0.1,
        breakpoint_length,
        24,
    )
    .translate(0.0, 0.0, break_z);

    let notch_inner = centered_cylinder(
        "notch_inner",
        breakpoint_diameter / 2.0,
        breakpoint_length + 0.2,
        24,
    )
    .translate(0.0, 0.0, break_z);

    let notch = notch_outer - notch_inner;

    // ── Assemble single swab ──

    let swab = lattice_head + neck + shaft - notch;

    // ── Export ──

    swab.write_stl("output/microlattice_swab.stl").unwrap();

    // ── Build plate array parameters ──
    // Elegoo Saturn 4 Ultra 16K: 218.88 x 122.88mm build area
    // Swabs printed vertically (Z-axis), XY footprint = head diameter
    // Per Lu 2024: batch printing improves success rate for slender structures
    //
    // Array handled by slicer (Chitubox/Lychee) — CSG boolean of 500+
    // lattice swabs would take hours. Import single STL, use slicer's
    // array/clone tool with the pitch below.

    let build_x = 218.88_f64;
    let build_y = 122.88_f64;
    let pitch = 5.0; // mm center-to-center (3mm head + 2mm gap)

    let cols = (build_x / pitch) as i32;
    let rows = (build_y / pitch) as i32;
    let swabs_per_plate = cols * rows;

    // ── Print report ──

    println!("Exported: output/microlattice_swab.stl");
    println!();
    println!("── Microlattice Swab Design Report ──");
    println!("  Research: Lu 2024, Tooker 2021, van der Elst 2020");
    println!("  Artifact: A-E5A1E1FD");
    println!();
    println!("  Geometry");
    println!("    Total length:    {total_length:.1}mm");
    println!("    Lattice head:    {head_length:.1}mm, {head_tip_diameter:.1}->{head_base_diameter:.1}mm tapered");
    println!("    Neck taper:      {neck_length:.1}mm transition to shaft");
    println!("    Shaft:           {shaft_diameter:.1}mm dia");
    println!("    Breakpoint:      {breakpoint_diameter:.1}mm neck at {breakpoint_from_tip:.0}mm from tip");
    println!("    Tail:            {:.1}mm below breakpoint", total_length - breakpoint_from_tip - breakpoint_length);
    println!();
    println!("  Lattice");
    println!("    Topology:        BCC (body-centered cubic)");
    println!("    Unit cell:       {unit_cell:.1}mm");
    println!("    Strut diameter:  {strut_diameter:.1}mm ({:.0} layers at 50um)", strut_diameter / 0.05);
    println!("    Grid:            {cells_x} x {cells_y} x {cells_z} cells");
    println!();
    println!("  Build Plate (Elegoo Saturn 4 Ultra 16K)");
    println!("    Build area:      {build_x:.2} x {build_y:.2}mm");
    println!("    Array pitch:     {pitch:.1}mm");
    println!("    Swabs per plate: {cols} x {rows} = {swabs_per_plate}");
    println!("    Layer height:    50um");
    println!("    Resin:           Liqcreate Bio-Med Clear (ISO 10993)");
}
