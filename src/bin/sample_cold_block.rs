use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Sample Collection Cold Block (96-Well PCR Plate) ───
//
// CNC-machined 6061-T6 aluminum block designed to hold a standard
// ANSI/SLAS 96-well PCR plate at 4C for sample collection during
// automated liquid handling experiments.
//
// The plate sits in a shallow registration pocket on the top face.
// 96 cylindrical well pockets provide intimate thermal contact with
// the conical bottoms of each PCR well. The block's thermal mass
// keeps samples cold for 30-60 minutes after pre-chilling to 4C.
//
// Primary use: collect media aspirates from microfluidic chip chambers
// during multi-day cell culture experiments. 16 chambers x 7 days =
// 112 samples, fitting in two 96-well plates.
//
// Cooling modes:
//   1. Passive: pre-chill block to 4C in refrigerator, use on bench
//   2. Active: mount TEC1-12706 Peltier module(s) on bottom face
//
// Cross-section (X-Z plane through wells, 12 columns shown):
//
//      x=-64          x=0           x=+64
//       |              |              |
// z=+14  ═══╤════╤════╤════╤════╤═══  top face
//       |   │    │    │    │    │   |  ← plate sits in registration pocket
// z=+12  ───┤well│well│well│well├───  ← pocket floor (2mm deep)
//       |   │5.8 │5.8 │5.8 │5.8 │   |
//       |   │dia │dia │dia │dia │   |  ← well pockets (12mm deep)
//       |   │    │    │    │    │   |
// z=0    ───┴────┴────┴────┴────┴───  ← well floors
//       |                            |
//       |     14mm thermal mass      |  ← aluminum below wells
//       |     (holds cold temp)      |
//       |                            |
// z=-13.5  ─ ─ TEC recess (0.5mm) ─ ─  ← optional Peltier mounting
// z=-14  ════════════════════════════  bottom face
//
// Material: 6061-T6 Aluminum
// Dimensions: 138mm x 96mm x 28mm (~1.0 kg)
// Export: output/sample_cold_block.stl

fn main() {
    // ── ANSI/SLAS 96-well plate dimensions ──
    let plate_length = 127.76; // mm (X, 12 columns)
    let plate_width = 85.48; // mm (Y, 8 rows)

    // ── Block outer dimensions ──
    // Plate footprint + wall thickness on each side for structural integrity
    // and thermal mass around the plate perimeter.
    let wall_thickness = 5.0; // mm per side
    let block_x = plate_length + 2.0 * wall_thickness; // ~138mm
    let block_y = plate_width + 2.0 * wall_thickness; // ~96mm
    let block_z = 28.0; // mm total height

    // ── Plate registration pocket ──
    // Shallow recess on top face that accepts the plate edge/skirt.
    // Keeps the plate precisely aligned and prevents lateral movement.
    let pocket_clearance = 0.25; // mm per side (snug but insertable)
    let pocket_x = plate_length + 2.0 * pocket_clearance; // ~128.26mm
    let pocket_y = plate_width + 2.0 * pocket_clearance; // ~85.98mm
    let pocket_depth = 2.0; // mm (holds plate rim/skirt)

    // ── 96 well pockets ──
    // Cylindrical bores that receive the conical bottoms of PCR plate wells.
    // Tight enough for thermal contact via air/paste interface, loose enough
    // for easy plate insertion and removal.
    let well_diameter = 6.0; // mm (PCR well OD ~5.46mm + 0.5mm clearance)
    let well_depth_from_top = 14.0; // mm from top face (2mm pocket + 12mm bore)
    let well_facets = 32; // facets per cylinder (smooth bore)

    // ── 96-well grid geometry (ANSI/SLAS 4-2004) ──
    let num_cols: usize = 12;
    let num_rows: usize = 8;
    let pitch = WELL96_PITCH; // 9.0mm center-to-center
    let a1_x_from_corner = WELL96_A1_X; // 14.38mm
    let a1_y_from_corner = WELL96_A1_Y; // 11.24mm

    // Convert A1 position to centered coordinates.
    // Grid center coincides with plate center (symmetric layout).
    let plate_center_x = plate_length / 2.0; // 63.88mm
    let plate_center_y = plate_width / 2.0; // 42.74mm
    let first_col_x = a1_x_from_corner - plate_center_x; // -49.50mm
    let first_row_y = plate_center_y - a1_y_from_corner; // +31.50mm (row A at +Y)

    // ── M3 mounting holes ──
    // 6 through-holes: 4 at corners + 2 at mid-length for stable deck attachment.
    // The long block (138mm) benefits from mid-span fasteners.
    let mount_hole_d = 3.2; // mm (M3 clearance)
    let mount_inset_x = 5.0; // mm from block edge (X direction)
    let mount_inset_y = 5.0; // mm from block edge (Y direction)
    let mount_x = block_x / 2.0 - mount_inset_x; // ±64mm
    let mount_y = block_y / 2.0 - mount_inset_y; // ±43mm

    // ── Thermistor pocket ──
    // DS18B20 waterproof probe (3mm dia stainless) inserts from the front
    // face (-Y). Positioned at mid-well-zone height for accurate bulk
    // temperature reading of the cold block.
    let therm_diameter = 3.5; // mm (0.5mm clearance for 3mm probe)
    let therm_depth = 30.0; // mm from front face (reaches block center)
    let therm_z = (block_z / 2.0) - well_depth_from_top / 2.0; // mid-well zone

    // ── Optional TEC recesses (bottom face) ──
    // Two TEC1-12706 (40x40mm) recesses for active Peltier cooling.
    // Spaced along the X axis for even coverage across the plate.
    // Leave these in the design even for passive use — costs nothing to machine.
    let tec_size = 40.2; // mm (0.2mm clearance for 40mm TEC)
    let tec_recess_depth = 0.5; // mm (registration only)
    let tec_spacing = 50.0; // mm between TEC centers (X direction)

    // ═══════════════════════════════════════════════════════
    // BUILD GEOMETRY
    // ═══════════════════════════════════════════════════════

    // ── 1. Solid body ──

    let body = centered_cube("body", block_x, block_y, block_z);

    // ── 2. Plate registration pocket (subtractive, from top face) ──
    // Positioned so the pocket floor is 2mm below the top surface.

    let pocket_z = (block_z - pocket_depth) / 2.0; // shift up: top of pocket = top of block
    let plate_pocket = centered_cube(
        "plate_pocket",
        pocket_x,
        pocket_y,
        pocket_depth + 1.0, // +1mm to cleanly cut through top face
    )
    .translate(0.0, 0.0, pocket_z);

    // ── 3. 96 well pockets (subtractive, from top face) ──
    // Each pocket is a cylinder extending from the top face downward.
    // well_depth_from_top = 14mm leaves 14mm thermal mass below.

    let well_z = (block_z - well_depth_from_top) / 2.0; // position so bottom = well floor

    let mut wells = Part::empty("wells");
    for row in 0..num_rows {
        for col in 0..num_cols {
            let x = first_col_x + (col as f64) * pitch;
            let y = first_row_y - (row as f64) * pitch;

            let well = centered_cylinder(
                &format!("well_r{row}_c{col}"),
                well_diameter / 2.0,
                well_depth_from_top + 1.0, // +1mm to cut through top face
                well_facets,
            )
            .translate(x, y, well_z);

            wells = wells + well;
        }
    }

    // ── 4. M3 mounting holes (through-holes, full block height) ──
    // 6 holes: 4 corners + 2 mid-length on the long edges.

    let mount_positions: [(f64, f64); 6] = [
        // 4 corners
        (-mount_x, -mount_y),
        (mount_x, -mount_y),
        (-mount_x, mount_y),
        (mount_x, mount_y),
        // 2 mid-length (X=0, at both Y edges)
        (0.0, -mount_y),
        (0.0, mount_y),
    ];

    let mut mounts = Part::empty("mounts");
    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("mount_{i}"),
            mount_hole_d / 2.0,
            block_z + 2.0, // through entire block
            24,
        )
        .translate(mx, my, 0.0);
        mounts = mounts + hole;
    }

    // ── 5. Thermistor pocket (horizontal bore from front face -Y) ──

    let therm_center_y = -(block_y / 2.0) + therm_depth / 2.0;
    let therm_pocket = centered_cylinder(
        "therm_pocket",
        therm_diameter / 2.0,
        therm_depth + 1.0, // +1mm to cut through front face
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, therm_z);

    // ── 6. Dual TEC recesses (bottom face, optional) ──
    // Two shallow rectangular pockets for TEC1-12706 registration.

    let tec_z = -(block_z / 2.0) + tec_recess_depth / 2.0 - 0.5;

    let tec_left = centered_cube("tec_left", tec_size, tec_size, tec_recess_depth + 1.0).translate(
        -tec_spacing / 2.0,
        0.0,
        tec_z,
    );

    let tec_right = centered_cube("tec_right", tec_size, tec_size, tec_recess_depth + 1.0)
        .translate(tec_spacing / 2.0, 0.0, tec_z);

    // ═══════════════════════════════════════════════════════
    // ASSEMBLE (subtract all features from body)
    // ═══════════════════════════════════════════════════════

    let part = body - plate_pocket - wells - mounts - therm_pocket - tec_left - tec_right;

    // ═══════════════════════════════════════════════════════
    // EXPORT
    // ═══════════════════════════════════════════════════════

    part.write_stl("output/sample_cold_block.stl").unwrap();

    // ═══════════════════════════════════════════════════════
    // PRINT SPECS
    // ═══════════════════════════════════════════════════════

    let thermal_mass = block_z - well_depth_from_top;
    let block_volume_cm3 = (block_x * block_y * block_z) / 1000.0; // approximate, before subtractions
    let block_mass_kg = block_volume_cm3 * 2.7 / 1000.0; // aluminum density 2.7 g/cm3

    println!("Exported: output/sample_cold_block.stl");
    println!();
    println!("── Sample Collection Cold Block (96-Well PCR Plate) ──");
    println!("  Body:              {block_x:.1}mm x {block_y:.1}mm x {block_z:.0}mm");
    println!("  Approx mass:       {block_mass_kg:.2} kg (6061-T6 aluminum)");
    println!("  Material:          6061-T6 Aluminum");
    println!();
    println!("── Plate Registration ──");
    println!("  Pocket:            {pocket_x:.2}mm x {pocket_y:.2}mm x {pocket_depth:.1}mm deep");
    println!(
        "  Plate clearance:   {pocket_clearance:.2}mm per side (snug, prevents lateral shift)"
    );
    println!("  Plate standard:    ANSI/SLAS 4-2004 ({plate_length:.2}mm x {plate_width:.2}mm)");
    println!();
    println!("── Well Pockets (96x) ──");
    println!("  Diameter:          {well_diameter:.1}mm (PCR well OD ~5.5mm + clearance)");
    println!("  Depth from top:    {well_depth_from_top:.0}mm (embraces full conical portion)");
    println!("  Pitch:             {pitch:.1}mm (ANSI/SLAS standard)");
    println!(
        "  Grid:              {num_cols} cols x {num_rows} rows = {} wells",
        num_cols * num_rows
    );
    println!("  A1 position:       X={first_col_x:.2}mm, Y={first_row_y:.2}mm (centered coords)");
    println!();
    println!("── Thermal Performance ──");
    println!("  Thermal mass:      {thermal_mass:.0}mm aluminum below wells");
    println!("  Pre-chill:         Refrigerate block to 4C, maintains <8C for ~30-45 min on bench");
    println!("  Active cooling:    Optional dual TEC1-12706 on bottom face");
    println!();
    println!("── Mounting ──");
    println!(
        "  Holes:             6x M3 ({mount_hole_d:.1}mm clearance), {mount_inset_x:.0}mm inset from edges"
    );
    println!("  Layout:            4 corners + 2 mid-length (stable over 138mm span)");
    println!();
    println!("── Temperature Sensor ──");
    println!(
        "  Thermistor pocket: {therm_diameter:.1}mm dia x {therm_depth:.0}mm deep (from front face)"
    );
    println!("  Sensor:            DS18B20 waterproof probe (3mm OD stainless)");
    println!("  Z position:        {therm_z:+.1}mm (mid-well-zone height)");
    println!();
    println!("── Optional TEC Recesses (Bottom Face) ──");
    println!("  Count:             2x TEC1-12706 (40x40mm)");
    println!(
        "  Recess:            {tec_size:.1}mm x {tec_size:.1}mm x {tec_recess_depth:.1}mm deep"
    );
    println!("  Spacing:           {tec_spacing:.0}mm center-to-center (X axis)");
    println!("  Note:              Machine these even for passive use — zero added cost");
    println!();
    println!("── Fabrication Notes ──");
    println!("  Stock: 140mm x 100mm x 30mm 6061-T6 aluminum plate/bar");
    println!("  Operations:");
    println!(
        "    1. Face top and bottom, square to {block_x:.1}mm x {block_y:.1}mm x {block_z:.0}mm"
    );
    println!("    2. Mill plate pocket: {pocket_x:.2}mm x {pocket_y:.2}mm, {pocket_depth:.1}mm deep (flat endmill)");
    println!("    3. Drill 96 well pockets: {well_diameter:.1}mm dia, {well_depth_from_top:.0}mm deep (from top face)");
    println!("       Use a {well_diameter:.1}mm endmill or drill bit, 9mm pitch grid");
    println!("       CNC recommended — 96 holes at precise 9mm spacing");
    println!(
        "    4. Mill 2x TEC recesses on bottom: {tec_size:.1}mm sq x {tec_recess_depth:.1}mm deep"
    );
    println!("    5. Drill thermistor pocket: {therm_diameter:.1}mm dia, {therm_depth:.0}mm deep from front");
    println!("    6. Drill 6x mounting holes: {mount_hole_d:.1}mm through");
    println!("    7. Deburr all edges, especially pocket rim and well bores");
    println!();
    println!("── Critical Tolerances ──");
    println!("  Well pitch:        ±0.1mm (cumulative error over 11 pitches must be <0.5mm)");
    println!("  Pocket flatness:   ±0.1mm (plate must sit flat for uniform thermal contact)");
    println!("  TEC recess flat:   <0.05mm (thermal interface quality)");
    println!("  Well diameter:     {well_diameter:.1}mm +0.2/-0 (wells must not be too tight)");
    println!();
    println!("── Usage ──");
    println!("  1. Pre-chill block in refrigerator (4C) for 2+ hours");
    println!("  2. Mount on robot deck via 6x M3 bolts");
    println!("  3. Insert empty 96-well PCR plate into registration pocket");
    println!("  4. Robot collects samples into wells (rows = days, columns = chambers)");
    println!("  5. After collection, seal plate with adhesive PCR film");
    println!("  6. Store at -20C or -80C for downstream analysis");
    println!();
    println!("── Sample Mapping (16-chamber x 7-day experiment) ──");
    println!("  Columns 1-16:  chambers 1-16 (uses 2 plates for >96 samples)");
    println!("  Rows A-H:      days 1-7 (+1 spare row)");
    println!("  Total:          112 samples in 2 plates");
}
