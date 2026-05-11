#[cfg(not(feature = "step"))]
fn main() {
    eprintln!("This binary requires the 'step' feature. Build with: cargo build --bin microfluidic_chip_cnc_step --features step");
    std::process::exit(1);
}

#[cfg(feature = "step")]
use glam::dvec3;
#[cfg(feature = "step")]
use laminarforge_cad::*;
#[cfg(feature = "step")]
use opencascade::primitives::Shape;
#[cfg(feature = "step")]
use opencascade::workplane::Workplane;

// ─── CNC-Milled 16-Chamber Microfluidic Chip (Rev B) — STEP Export ───
//
// Identical geometry to microfluidic_chip_cnc.rs but uses OpenCascade B-rep
// kernel for proper STEP file output (required by CNC shops).

#[cfg(feature = "step")]
/// Centered box at (cx, cy, cz) with dimensions (w, d, h)
fn centered_box(cx: f64, cy: f64, cz: f64, w: f64, d: f64, h: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, cz - h / 2.0))
        .rect(w, d);
    wire.to_face().extrude(dvec3(0.0, 0.0, h)).into()
}

#[cfg(feature = "step")]
/// Centered cylinder at (cx, cy, cz) with radius r and height h along Z
fn centered_cyl(cx: f64, cy: f64, cz: f64, r: f64, h: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, cz - h / 2.0))
        .circle(0.0, 0.0, r);
    wire.to_face().extrude(dvec3(0.0, 0.0, h)).into()
}

#[cfg(feature = "step")]
/// Cylinder from z_bottom upward with height h
fn cyl_at(cx: f64, cy: f64, z_bottom: f64, r: f64, h: f64) -> Shape {
    let wire = Workplane::xy()
        .translated(dvec3(cx, cy, z_bottom))
        .circle(0.0, 0.0, r);
    wire.to_face().extrude(dvec3(0.0, 0.0, h)).into()
}

#[cfg(feature = "step")]
fn main() {
    // ── Dimensions from lib.rs ──
    let length = CNC16_CHIP_LENGTH;
    let width = CNC16_CHIP_WIDTH;
    let thickness = CNC16_CHIP_THICKNESS;
    let corner_r = CNC16_CORNER_RADIUS;

    let num_cols = CNC16_GRID_COLS;
    let num_rows = CNC16_GRID_ROWS;

    let ch_w = CNC16_CHANNEL_WIDTH;
    let ch_d = CNC16_CHANNEL_DEPTH;
    let ch_len = CNC16_CHANNEL_LENGTH;

    let chamber_w = CNC16_CHAMBER_WIDTH;
    let chamber_l = CNC16_CHAMBER_LENGTH;

    let port_dia = CNC16_PORT_DIAMETER;

    let col_spacing = CNC16_COL_SPACING;
    let row_spacing = CNC16_ROW_SPACING;

    let align_dia = CNC16_ALIGN_DIAMETER;
    let align_depth = CNC16_ALIGN_DEPTH;
    let align_inset = CNC16_ALIGN_INSET;

    let mount_dia = CNC16_MOUNT_DIAMETER;
    let mount_inset = CNC16_MOUNT_INSET;

    let label_l = CNC16_LABEL_LENGTH;
    let label_w = CNC16_LABEL_WIDTH;
    let label_d = CNC16_LABEL_DEPTH;

    let half_l = length / 2.0;
    let half_w = width / 2.0;
    let chip_top = thickness / 2.0;

    // ── Grid positions ──
    let first_col_x = -((num_cols as f64 - 1.0) * col_spacing) / 2.0;
    let first_row_y = ((num_rows as f64 - 1.0) * row_spacing) / 2.0;

    let col_xs: Vec<f64> = (0..num_cols)
        .map(|c| first_col_x + c as f64 * col_spacing)
        .collect();
    let row_ys: Vec<f64> = (0..num_rows)
        .map(|r| first_row_y - r as f64 * row_spacing)
        .collect();

    // ════════════════════════════════════════════════════
    // PART 1: Channel Plate (rounded corners via Wire.fillet)
    // ════════════════════════════════════════════════════

    let mut outline = Workplane::xy()
        .translated(dvec3(0.0, 0.0, -thickness / 2.0))
        .rect(length, width);
    outline.fillet(corner_r);

    let mut plate: Shape = outline.to_face().extrude(dvec3(0.0, 0.0, thickness)).into();

    // ── Chambers + channels + ports ──
    let pocket_z = chip_top - ch_d / 2.0; // center Z of shallow pockets

    for &cy in &row_ys {
        for &cx in &col_xs {
            // Chamber pocket
            let chamber = centered_box(cx, cy, pocket_z, chamber_w, chamber_l, ch_d);
            plate = plate.subtract(&chamber).into();

            // Inlet channel (below chamber)
            let inlet_port_y = cy - chamber_l / 2.0 - ch_len;
            let inlet_ch_y = cy - chamber_l / 2.0 - ch_len / 2.0;
            let inlet_ch = centered_box(cx, inlet_ch_y, pocket_z, ch_w, ch_len, ch_d);
            plate = plate.subtract(&inlet_ch).into();

            // Outlet channel (above chamber)
            let outlet_port_y = cy + chamber_l / 2.0 + ch_len;
            let outlet_ch_y = cy + chamber_l / 2.0 + ch_len / 2.0;
            let outlet_ch = centered_box(cx, outlet_ch_y, pocket_z, ch_w, ch_len, ch_d);
            plate = plate.subtract(&outlet_ch).into();

            // Port through-holes
            let through = thickness + 1.0;
            let inlet_port = centered_cyl(cx, inlet_port_y, 0.0, port_dia / 2.0, through);
            plate = plate.subtract(&inlet_port).into();

            let outlet_port = centered_cyl(cx, outlet_port_y, 0.0, port_dia / 2.0, through);
            plate = plate.subtract(&outlet_port).into();
        }
    }

    // ── Alignment pin holes (blind from top) ──
    let align_positions = [
        (-(half_l - align_inset), -(half_w - align_inset)),
        (-(half_l - align_inset), half_w - align_inset),
        (half_l - align_inset, -(half_w - align_inset)),
        (half_l - align_inset, half_w - align_inset),
    ];

    for &(ax, ay) in &align_positions {
        let hole = cyl_at(
            ax,
            ay,
            chip_top - align_depth,
            align_dia / 2.0,
            align_depth + 0.1,
        );
        plate = plate.subtract(&hole).into();
    }

    // ── Mounting holes (M3 through) ──
    let mount_positions = [
        (-(half_l - mount_inset), -(half_w - mount_inset)),
        (-(half_l - mount_inset), half_w - mount_inset),
        (half_l - mount_inset, -(half_w - mount_inset)),
        (half_l - mount_inset, half_w - mount_inset),
    ];

    for &(mx, my) in &mount_positions {
        let hole = centered_cyl(mx, my, 0.0, mount_dia / 2.0, thickness + 1.0);
        plate = plate.subtract(&hole).into();
    }

    // ── Label pocket (center of chip) ──
    let label = centered_box(
        0.0,
        0.0,
        chip_top - label_d / 2.0,
        label_l,
        label_w,
        label_d,
    );
    plate = plate.subtract(&label).into();

    // ════════════════════════════════════════════════════
    // PART 2: Cover Plate
    // ════════════════════════════════════════════════════

    let cover_t = CNC16_COVER_THICKNESS;
    let mut cover_outline = Workplane::xy()
        .translated(dvec3(0.0, 0.0, -cover_t / 2.0))
        .rect(length, width);
    cover_outline.fillet(corner_r);

    let mut cover: Shape = cover_outline
        .to_face()
        .extrude(dvec3(0.0, 0.0, cover_t))
        .into();

    // Alignment through-holes
    for &(ax, ay) in &align_positions {
        let hole = centered_cyl(ax, ay, 0.0, align_dia / 2.0, cover_t + 1.0);
        cover = cover.subtract(&hole).into();
    }

    // ════════════════════════════════════════════════════
    // EXPORT
    // ════════════════════════════════════════════════════

    plate
        .write_step("output/microfluidic_chip_16ch_channel_plate.stp")
        .expect("Failed to write channel plate STEP");

    cover
        .write_step("output/microfluidic_chip_16ch_cover_plate.stp")
        .expect("Failed to write cover plate STEP");

    println!("Exported STEP files:");
    println!("  output/microfluidic_chip_16ch_channel_plate.stp");
    println!("  output/microfluidic_chip_16ch_cover_plate.stp");
    println!();
    println!("Rev B | {length}mm x {width}mm | {corner_r}mm corner radius");
    println!("16 chambers (4x4) | 500um channels | 1.6mm ports | ANSI/SLAS footprint");
}
