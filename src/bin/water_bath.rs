use vcad::{centered_cube, centered_cylinder, Part};

// ─── Water Bath ───
//
// 3D-printed (PETG) heated water bath for laboratory use. Features a
// waterproof inner basin with ports for immersion heater and thermistor,
// an insulated outer shell with 10mm foam gap, a lid with vent hole
// and condensation drip edge, and a test tube rack insert.
//
// Exports:
//   - water_bath_basin.stl  (PETG waterproof inner basin)
//   - water_bath_shell.stl  (insulated outer housing)
//   - water_bath_lid.stl    (lid with vent and drip edge)
//   - water_bath_rack.stl   (test tube rack insert)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 1. BASIN
    // ══════════════════════════════════════════════════════════════

    let basin_inner_x = 150.0;
    let basin_inner_y = 100.0;
    let basin_inner_z = 80.0;
    let basin_wall = 3.0;

    let basin_outer_x = basin_inner_x + basin_wall * 2.0;
    let basin_outer_y = basin_inner_y + basin_wall * 2.0;
    let basin_outer_z = basin_inner_z + basin_wall; // floor only at bottom

    let basin_outer = centered_cube("basin_outer", basin_outer_x, basin_outer_y, basin_outer_z);
    let basin_inner = centered_cube("basin_inner", basin_inner_x, basin_inner_y, basin_inner_z)
        .translate(0.0, 0.0, basin_wall / 2.0);

    // Heater port (8mm dia hole at bottom back wall)
    let heater_port = centered_cylinder("heater_port", 8.0 / 2.0, basin_wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, basin_outer_y / 2.0, -(basin_outer_z / 2.0) + basin_wall + 15.0);

    // Thermistor port (5mm dia hole at mid-height back wall)
    let thermistor_port = centered_cylinder("thermistor_port", 5.0 / 2.0, basin_wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(30.0, basin_outer_y / 2.0, 0.0);

    let basin = (basin_outer - basin_inner) - heater_port - thermistor_port;

    basin.write_stl("output/water_bath_basin.stl").unwrap();

    println!("Exported: output/water_bath_basin.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. SHELL
    // ══════════════════════════════════════════════════════════════

    let foam_gap = 10.0;
    let shell_wall = 3.0;

    let shell_outer_x = basin_outer_x + (foam_gap + shell_wall) * 2.0;
    let shell_outer_y = basin_outer_y + (foam_gap + shell_wall) * 2.0;
    let shell_outer_z = basin_outer_z + foam_gap + shell_wall; // insulation on bottom + sides

    let shell_inner_x = basin_outer_x + foam_gap * 2.0;
    let shell_inner_y = basin_outer_y + foam_gap * 2.0;
    let shell_inner_z = basin_outer_z + foam_gap;

    let shell_outer = centered_cube("shell_outer", shell_outer_x, shell_outer_y, shell_outer_z);
    let shell_inner = centered_cube("shell_inner", shell_inner_x, shell_inner_y, shell_inner_z)
        .translate(0.0, 0.0, shell_wall / 2.0);

    let shell = shell_outer - shell_inner;

    shell.write_stl("output/water_bath_shell.stl").unwrap();

    println!("Exported: output/water_bath_shell.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. LID
    // ══════════════════════════════════════════════════════════════

    let lid_x = basin_outer_x + 4.0; // slight overhang
    let lid_y = basin_outer_y + 4.0;
    let lid_thickness = 5.0;

    let lid_body = centered_cube("lid_body", lid_x, lid_y, lid_thickness);

    // Lip (recessed edge that sits inside basin opening)
    let lip_depth = 8.0;
    let lip_clearance = 0.5;
    let lip_x = basin_inner_x - lip_clearance * 2.0;
    let lip_y = basin_inner_y - lip_clearance * 2.0;
    let lip_wall = 2.0;

    let lip_outer = centered_cube("lip_outer", lip_x, lip_y, lip_depth)
        .translate(0.0, 0.0, -(lid_thickness / 2.0) - lip_depth / 2.0);
    let lip_inner = centered_cube(
        "lip_inner",
        lip_x - lip_wall * 2.0,
        lip_y - lip_wall * 2.0,
        lip_depth + 0.1,
    )
    .translate(0.0, 0.0, -(lid_thickness / 2.0) - lip_depth / 2.0);

    // Vent hole (10mm dia, off-center)
    let vent_hole = centered_cylinder("vent_hole", 10.0 / 2.0, lid_thickness + 2.0, 24)
        .translate(30.0, 20.0, 0.0);

    // Condensation drip edge: a shallow groove around the underside perimeter
    // that catches condensation and channels it back into the basin
    let drip_groove_width = 3.0;
    let drip_groove_depth = 1.5;
    let drip_outer_cut = centered_cube(
        "drip_outer",
        lid_x - 4.0,
        lid_y - 4.0,
        drip_groove_depth,
    )
    .translate(0.0, 0.0, -(lid_thickness / 2.0) + drip_groove_depth / 2.0);

    let drip_inner_fill = centered_cube(
        "drip_inner",
        lid_x - 4.0 - drip_groove_width * 2.0,
        lid_y - 4.0 - drip_groove_width * 2.0,
        drip_groove_depth + 0.1,
    )
    .translate(0.0, 0.0, -(lid_thickness / 2.0) + drip_groove_depth / 2.0);

    let lid = (lid_body + lip_outer - lip_inner - drip_outer_cut + drip_inner_fill) - vent_hole;

    lid.write_stl("output/water_bath_lid.stl").unwrap();

    println!("Exported: output/water_bath_lid.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. TEST TUBE RACK
    // ══════════════════════════════════════════════════════════════

    let tube_d = 15.0; // 15mm dia tubes
    let tube_spacing = 20.0;
    let num_tubes = 6; // 2 rows of 3
    let num_rows = 2;
    let num_cols = 3;

    let rack_x = (num_cols as f64) * tube_spacing + 10.0;
    let rack_y = (num_rows as f64) * tube_spacing + 10.0;
    let rack_z = 60.0; // tall enough to hold tubes upright in basin
    let rack_base_z = 5.0;

    // Rack frame: a base plate with vertical walls forming a grid
    let rack_base = centered_cube("rack_base", rack_x, rack_y, rack_base_z)
        .translate(0.0, 0.0, -(rack_z / 2.0) + rack_base_z / 2.0);

    // Top plate with holes
    let rack_top = centered_cube("rack_top", rack_x, rack_y, rack_base_z)
        .translate(0.0, 0.0, rack_z / 2.0 - rack_base_z / 2.0);

    // Side walls connecting top and bottom
    let side_wall_thick = 2.0;
    let left_wall = centered_cube("left_wall", side_wall_thick, rack_y, rack_z)
        .translate(-(rack_x / 2.0) + side_wall_thick / 2.0, 0.0, 0.0);
    let right_wall = centered_cube("right_wall", side_wall_thick, rack_y, rack_z)
        .translate(rack_x / 2.0 - side_wall_thick / 2.0, 0.0, 0.0);
    let front_wall = centered_cube("front_wall", rack_x, side_wall_thick, rack_z)
        .translate(0.0, -(rack_y / 2.0) + side_wall_thick / 2.0, 0.0);
    let back_wall = centered_cube("back_wall", rack_x, side_wall_thick, rack_z)
        .translate(0.0, rack_y / 2.0 - side_wall_thick / 2.0, 0.0);

    // Tube holes through top plate
    let mut tube_holes = Part::empty("tube_holes");
    for row in 0..num_rows {
        for col in 0..num_cols {
            let hx = -(num_cols as f64 - 1.0) * tube_spacing / 2.0 + (col as f64) * tube_spacing;
            let hy = -(num_rows as f64 - 1.0) * tube_spacing / 2.0 + (row as f64) * tube_spacing;
            let hole = centered_cylinder(
                &format!("tube_{row}_{col}"),
                tube_d / 2.0,
                rack_base_z + 2.0,
                24,
            )
            .translate(hx, hy, rack_z / 2.0 - rack_base_z / 2.0);
            tube_holes = tube_holes + hole;
        }
    }

    let rack = (rack_base + rack_top + left_wall + right_wall + front_wall + back_wall)
        - tube_holes;

    rack.write_stl("output/water_bath_rack.stl").unwrap();

    println!("Exported: output/water_bath_rack.stl");

    // ── Print specs ──

    println!();
    println!("── Water Bath Specs ──");
    println!("  Basin interior:  {basin_inner_x:.0}mm x {basin_inner_y:.0}mm x {basin_inner_z:.0}mm");
    println!("  Basin walls:     {basin_wall:.0}mm PETG (waterproof)");
    println!("  Heater port:     8mm dia (bottom back)");
    println!("  Thermistor port: 5mm dia (mid-height back)");
    println!("  Foam gap:        {foam_gap:.0}mm insulation");
    println!("  Shell outer:     {shell_outer_x:.0}mm x {shell_outer_y:.0}mm x {shell_outer_z:.0}mm");
    println!("  Lid:             {lid_x:.0}mm x {lid_y:.0}mm x {lid_thickness:.0}mm");
    println!("  Vent hole:       10mm dia");
    println!("  Drip edge:       {drip_groove_width:.0}mm wide x {drip_groove_depth:.1}mm deep perimeter groove");
    println!("  Rack:            {num_tubes} slots ({num_cols}x{num_rows}), {tube_d:.0}mm dia, {tube_spacing:.0}mm spacing");
    println!("  Material:        PETG");
}
