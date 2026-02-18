use vcad::{centered_cube, centered_cylinder, Part};

// ─── Column / Test Tube Rack ───
//
// 3D-printed (PETG) storage rack for chromatography columns and test
// tubes. Features a base with 8 holes (2 rows of 4) for 12mm tubes,
// a tall back wall for labeling, and triangular side braces for
// structural rigidity.
//
// Exports: column_rack.stl

fn main() {
    // ── Dimensions ──

    // Base plate
    let base_x = 120.0;
    let base_y = 60.0;
    let base_z = 15.0;

    // Tube holes
    let hole_d = 12.0;
    let hole_depth = 12.0;
    let hole_spacing = 25.0;
    let num_cols = 4;
    let num_rows = 2;

    // Back wall (for labeling)
    let back_wall_x = 120.0;
    let back_wall_y = 5.0;
    let back_wall_z = 80.0;

    // Side supports (triangular braces)
    let side_x = 5.0;
    let side_y = 60.0;
    let side_z = 80.0;

    // ── Build base ──

    let base = centered_cube("base", base_x, base_y, base_z)
        .translate(0.0, 0.0, base_z / 2.0);

    // ── Tube holes (2 rows of 4, from top of base) ──

    let mut holes = Part::empty("holes");
    for row in 0..num_rows {
        for col in 0..num_cols {
            let hx = -(num_cols as f64 - 1.0) * hole_spacing / 2.0
                + (col as f64) * hole_spacing;
            let hy = -(num_rows as f64 - 1.0) * hole_spacing / 2.0
                + (row as f64) * hole_spacing;
            let hole = centered_cylinder(
                &format!("hole_{row}_{col}"),
                hole_d / 2.0,
                hole_depth + 1.0,
                24,
            )
            .translate(hx, hy, base_z - hole_depth / 2.0);
            holes = holes + hole;
        }
    }

    // ── Back wall ──

    let back_wall = centered_cube("back_wall", back_wall_x, back_wall_y, back_wall_z)
        .translate(0.0, base_y / 2.0 - back_wall_y / 2.0, base_z + back_wall_z / 2.0);

    // ── Side supports (triangular braces) ──
    // Triangular braces: full rectangle that we cut diagonally.
    // The brace is a right triangle: full height at the back wall,
    // tapering to zero height at the front.
    //
    // We create a rectangular brace and subtract a diagonal wedge to
    // make a triangle shape.

    // Left brace
    let left_brace_block = centered_cube("left_brace", side_x, side_y, side_z)
        .translate(-(base_x / 2.0) + side_x / 2.0, 0.0, base_z + side_z / 2.0);

    // Diagonal cut: a large cube rotated to slice from front-top to back-bottom
    // The triangle should be: full height at back (Y = base_y/2), zero at front (Y = -base_y/2)
    // We use a cube positioned and rotated to cut the diagonal
    let cut_angle = (side_z as f64).atan2(side_y as f64).to_degrees();
    let cut_diag = (side_y * side_y + side_z * side_z).sqrt();
    let left_cut = centered_cube("left_cut", side_x + 2.0, cut_diag * 2.0, cut_diag * 2.0)
        .rotate(cut_angle, 0.0, 0.0)
        .translate(-(base_x / 2.0) + side_x / 2.0, -side_y / 2.0, base_z + side_z);

    // Right brace
    let right_brace_block = centered_cube("right_brace", side_x, side_y, side_z)
        .translate(base_x / 2.0 - side_x / 2.0, 0.0, base_z + side_z / 2.0);

    let right_cut = centered_cube("right_cut", side_x + 2.0, cut_diag * 2.0, cut_diag * 2.0)
        .rotate(cut_angle, 0.0, 0.0)
        .translate(base_x / 2.0 - side_x / 2.0, -side_y / 2.0, base_z + side_z);

    // ── Assemble ──

    let rack = (base + back_wall + (left_brace_block - left_cut) + (right_brace_block - right_cut))
        - holes;

    // ── Export ──

    rack.write_stl("output/column_rack.stl").unwrap();

    println!("Exported: output/column_rack.stl");
    println!();
    println!("── Column / Test Tube Rack Specs ──");
    println!("  Base:          {base_x:.0}mm x {base_y:.0}mm x {base_z:.0}mm");
    println!("  Tube holes:    {num_rows}x{num_cols} = {} holes, {hole_d:.0}mm dia x {hole_depth:.0}mm deep, {hole_spacing:.0}mm spacing", num_rows * num_cols);
    println!("  Back wall:     {back_wall_x:.0}mm x {back_wall_y:.0}mm x {back_wall_z:.0}mm (labeling surface)");
    println!("  Side braces:   {side_x:.0}mm x {side_y:.0}mm x {side_z:.0}mm (triangular)");
    println!("  Total height:  {:.0}mm", base_z + back_wall_z);
    println!("  Material:      PETG");
}
