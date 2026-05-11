use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── CNC-Milled 16-Chamber Microfluidic Chip (Rev B) ───
//
// AAV selectivity screening chip with 16 independent culture chambers.
// ANSI/SLAS microplate footprint: 127.76mm x 85.48mm
// Compatible with standard plate readers, incubator racks, automated handlers.
//
// Two-part construction:
//   Part 1: Channel plate — 127.76mm x 85.48mm x 3mm cast PMMA
//           All features CNC-milled into TOP face (chambers, channels, ports)
//   Part 2: Cover plate  — 127.76mm x 85.48mm x 1mm cast PMMA
//           Alignment through-holes only. Bonded on top with Weld-On 3.
//
// Chamber layout: 4x4 grid (4 columns x 4 rows)
//   Columns along X (long axis), Rows along Y (short axis)
//   Each chamber: 3mm wide (X) x 10mm long (Y) x 200um deep
//   Each chamber has dedicated inlet (bottom) and outlet (top) channels + ports
//
// Channel routing: straight Y-direction channels from port to chamber
//   Inlet port below chamber, outlet port above chamber
//   All channels 500um wide x 200um deep
//   Channel length: 2mm (port center to chamber edge)
//
// ANSI/SLAS compliance: 3.18mm corner radius on all four corners (§4.1.2.1)
// Port connection: press-fit for 1/16" OD PTFE tubing (1.6mm holes)
//
// Top view (schematic):
//
//   ┌────────────────────────────────────────────────────────────────────────────────┐
//   │ o align                                                             o align   │
//   │     O mount                                                     O mount       │
//   │                                                                               │
//   │         ●out   ●out   ●out   ●out                                             │
//   │         ║       ║       ║       ║    ← outlet channels                        │
//   │        ┌──┐   ┌──┐   ┌──┐   ┌──┐   ← Row 0 (chambers)                       │
//   │        └──┘   └──┘   └──┘   └──┘                                             │
//   │         ║       ║       ║       ║    ← inlet channels                         │
//   │         ●in    ●in    ●in    ●in                                              │
//   │                                                                               │
//   │         ●out   ●out   ●out   ●out                                             │
//   │         ║       ║       ║       ║                                              │
//   │        ┌──┐   ┌──┐   ┌──┐   ┌──┐   ← Row 1                                  │
//   │        └──┘   └──┘   └──┘   └──┘                                             │
//   │         ║       ║       ║       ║                                              │
//   │         ●in    ●in    ●in    ●in                                              │
//   │                                                                               │
//   │         ●out   ●out   ●out   ●out                                             │
//   │         ║       ║       ║       ║                                              │
//   │        ┌──┐   ┌──┐   ┌──┐   ┌──┐   ← Row 2                                  │
//   │        └──┘   └──┘   └──┘   └──┘                                             │
//   │         ║       ║       ║       ║                                              │
//   │         ●in    ●in    ●in    ●in                                              │
//   │                                                                               │
//   │         ●out   ●out   ●out   ●out                                             │
//   │         ║       ║       ║       ║                                              │
//   │        ┌──┐   ┌──┐   ┌──┐   ┌──┐   ← Row 3                                  │
//   │        └──┘   └──┘   └──┘   └──┘                                             │
//   │         ║       ║       ║       ║                                              │
//   │         ●in    ●in    ●in    ●in                                              │
//   │                                                                               │
//   │     O mount                                                     O mount       │
//   │ o align                                                             o align   │
//   └────────────────────────────────────────────────────────────────────────────────┘
//    ←────────────────────── 127.76mm ──────────────────────────────────────────────→
//
// Chamber numbering (left-to-right, top-to-bottom):
//   Row 0: Ch1  (DRG target)  | Ch2  (DRG replicate) | Ch3  (Hepatocytes) | Ch4  (Motor neurons)
//   Row 1: Ch5  (Cortical)    | Ch6  (Cardio)        | Ch7  (Endothelial) | Ch8  (Skeletal)
//   Row 2: Ch9  (PBMCs)       | Ch10 (Kidney)        | Ch11 (Lung)        | Ch12 (Astrocytes)
//   Row 3: Ch13 (Pancreatic)  | Ch14 (Enteric)       | Ch15 (HEK293 +ctrl)| Ch16 (Empty -ctrl)

/// Chamber allocation labels
const CHAMBER_LABELS: [&str; 16] = [
    "DRG sensory neurons (TARGET)",
    "DRG sensory neurons (REPLICATE)",
    "Hepatocytes (liver)",
    "Motor neurons",
    "Cortical neurons",
    "Cardiomyocytes (heart)",
    "Endothelial cells",
    "Skeletal muscle",
    "PBMCs / T cells",
    "Kidney (renal epithelial)",
    "Lung epithelial",
    "Astrocytes",
    "Pancreatic beta cells",
    "Enteric neurons",
    "Positive control (HEK293)",
    "Negative control (empty)",
];

fn main() {
    // ── Dimensions from lib.rs ──

    let length = CNC16_CHIP_LENGTH; // 127.76mm
    let width = CNC16_CHIP_WIDTH; // 85.48mm
    let thickness = CNC16_CHIP_THICKNESS; // 3mm

    let num_cols = CNC16_GRID_COLS; // 4
    let num_rows = CNC16_GRID_ROWS; // 4

    let ch_w = CNC16_CHANNEL_WIDTH; // 0.5mm
    let ch_d = CNC16_CHANNEL_DEPTH; // 0.2mm
    let ch_len = CNC16_CHANNEL_LENGTH; // 2.0mm (Rev B: reduced from 5.0mm)

    let chamber_w = CNC16_CHAMBER_WIDTH; // 3.0mm (X)
    let chamber_l = CNC16_CHAMBER_LENGTH; // 10.0mm (Y)
    let chamber_d = CNC16_CHAMBER_DEPTH; // 0.2mm

    let port_dia = CNC16_PORT_DIAMETER; // 1.6mm (Rev B: increased from 1.5mm)

    let col_spacing = CNC16_COL_SPACING; // 25.0mm
    let row_spacing = CNC16_ROW_SPACING; // 18.0mm

    let align_dia = CNC16_ALIGN_DIAMETER; // 1.0mm
    let align_depth = CNC16_ALIGN_DEPTH; // 0.5mm
    let align_inset = CNC16_ALIGN_INSET; // 4.0mm

    let mount_dia = CNC16_MOUNT_DIAMETER; // 3.2mm
    let mount_inset = CNC16_MOUNT_INSET; // 6.0mm

    let label_l = CNC16_LABEL_LENGTH; // 20mm
    let label_w = CNC16_LABEL_WIDTH; // 5mm
    let label_d = CNC16_LABEL_DEPTH; // 0.1mm
    let corner_r = CNC16_CORNER_RADIUS; // 3.18mm (ANSI/SLAS)

    // ── Derived positions ──
    // vcad centered coordinates: chip center at origin
    // Chip X: [-63.88, +63.88], Y: [-42.74, +42.74], Z: [-1.5, +1.5]

    let chip_top = thickness / 2.0; // +1.5mm

    // Grid center positions
    // 4 columns: centers at -37.5, -12.5, +12.5, +37.5 mm from chip center
    let first_col_x = -((num_cols as f64 - 1.0) * col_spacing) / 2.0; // -37.5
    let first_row_y = ((num_rows as f64 - 1.0) * row_spacing) / 2.0; // +27.0 (top row)

    let col_xs: Vec<f64> = (0..num_cols)
        .map(|c| first_col_x + c as f64 * col_spacing)
        .collect();
    let row_ys: Vec<f64> = (0..num_rows)
        .map(|r| first_row_y - r as f64 * row_spacing) // top to bottom
        .collect();

    // ── Cutting tool heights ──
    let pocket_tool_h = ch_d + 0.2; // 0.4mm
    let pocket_z = chip_top - ch_d / 2.0 + 0.1;

    let align_tool_h = align_depth + 0.2; // 0.7mm
    let align_z = chip_top - align_depth / 2.0 + 0.1;

    let label_tool_h = label_d + 0.2; // 0.3mm
    let label_z = chip_top - label_d / 2.0 + 0.1;

    let through_h = thickness + 1.0; // 4mm, exceeds 3mm plate

    // ════════════════════════════════════════════════════
    // PART 1: Channel Plate
    // ════════════════════════════════════════════════════

    let mut channel_plate = centered_cube("channel_plate", length, width, thickness);

    // ── Chambers + channels + ports for all 16 positions ──
    let mut chamber_idx = 0usize;
    let mut port_positions: Vec<(usize, f64, f64, f64, f64)> = Vec::new(); // (idx, inlet_x, inlet_y, outlet_x, outlet_y)

    for (row, &cy) in row_ys.iter().enumerate() {
        for (col, &cx) in col_xs.iter().enumerate() {
            let idx = row * num_cols + col;

            // Chamber pocket: 3mm(X) x 10mm(Y) x 0.2mm deep
            let chamber = centered_cube(
                format!("chamber_{idx}"),
                chamber_w,
                chamber_l,
                pocket_tool_h,
            )
            .translate(cx, cy, pocket_z);
            channel_plate = channel_plate - chamber;

            // Inlet channel: runs from below chamber to inlet port
            // Inlet port is below chamber (negative Y direction)
            let chamber_bottom_y = cy - chamber_l / 2.0; // bottom edge of chamber
            let inlet_port_y = chamber_bottom_y - ch_len; // port center

            let inlet_ch_center_y = (chamber_bottom_y + inlet_port_y) / 2.0;
            let inlet_channel =
                centered_cube(format!("inlet_ch_{idx}"), ch_w, ch_len, pocket_tool_h).translate(
                    cx,
                    inlet_ch_center_y,
                    pocket_z,
                );
            channel_plate = channel_plate - inlet_channel;

            // Outlet channel: runs from above chamber to outlet port
            let chamber_top_y = cy + chamber_l / 2.0; // top edge of chamber
            let outlet_port_y = chamber_top_y + ch_len; // port center

            let outlet_ch_center_y = (chamber_top_y + outlet_port_y) / 2.0;
            let outlet_channel =
                centered_cube(format!("outlet_ch_{idx}"), ch_w, ch_len, pocket_tool_h).translate(
                    cx,
                    outlet_ch_center_y,
                    pocket_z,
                );
            channel_plate = channel_plate - outlet_channel;

            // Inlet port (through-hole)
            let inlet_port =
                centered_cylinder(format!("inlet_port_{idx}"), port_dia / 2.0, through_h, 32)
                    .translate(cx, inlet_port_y, 0.0);
            channel_plate = channel_plate - inlet_port;

            // Outlet port (through-hole)
            let outlet_port =
                centered_cylinder(format!("outlet_port_{idx}"), port_dia / 2.0, through_h, 32)
                    .translate(cx, outlet_port_y, 0.0);
            channel_plate = channel_plate - outlet_port;

            port_positions.push((idx, cx, inlet_port_y, cx, outlet_port_y));
            chamber_idx += 1;
            let _ = (row, col); // suppress unused warnings
        }
    }

    // ── Alignment pin holes (4 corners, blind from top) ──
    let align_positions = [
        (-(length / 2.0 - align_inset), -(width / 2.0 - align_inset)), // bottom-left
        (-(length / 2.0 - align_inset), width / 2.0 - align_inset),    // top-left
        (length / 2.0 - align_inset, -(width / 2.0 - align_inset)),    // bottom-right
        (length / 2.0 - align_inset, width / 2.0 - align_inset),       // top-right
    ];

    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        let align_hole = centered_cylinder(format!("align_{i}"), align_dia / 2.0, align_tool_h, 24)
            .translate(ax, ay, align_z);
        channel_plate = channel_plate - align_hole;
    }

    // ── Mounting holes (4 near corners, M3 through-holes) ──
    let mount_positions = [
        (-(length / 2.0 - mount_inset), -(width / 2.0 - mount_inset)), // bottom-left
        (-(length / 2.0 - mount_inset), width / 2.0 - mount_inset),    // top-left
        (length / 2.0 - mount_inset, -(width / 2.0 - mount_inset)),    // bottom-right
        (length / 2.0 - mount_inset, width / 2.0 - mount_inset),       // top-right
    ];

    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        let mount_hole = centered_cylinder(format!("mount_{i}"), mount_dia / 2.0, through_h, 32)
            .translate(mx, my, 0.0);
        channel_plate = channel_plate - mount_hole;
    }

    // ── Label area (shallow engraved rectangle, center of chip) ──
    // Position: centered between column 1 and column 2 where no features exist.
    // (Rev A had label on right side, overlapping with Row 1/2 Col 3 ports — fixed in Rev B)
    let label_x = 0.0; // chip center X (between columns 1 and 2)
    let label_y = 0.0; // chip center Y (between rows 1 and 2)
    let label_pocket = centered_cube("label_area", label_l, label_w, label_tool_h)
        .translate(label_x, label_y, label_z);
    channel_plate = channel_plate - label_pocket;

    // ── Corner radii (ANSI/SLAS 1-2004 §4.1.2.1: 3.18 ± 1.6mm) ──
    // CSG approach: at each corner, subtract the square corner, then add back a cylinder
    // to create a rounded edge. Equivalent to: plate -= (corner_square - corner_cylinder)
    let half_l = length / 2.0;
    let half_w = width / 2.0;
    let corner_through = thickness + 1.0; // cut through full plate

    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let corner_sq = centered_cube(
            format!("corner_sq_{}_{}", si.0, si.1),
            corner_r,
            corner_r,
            corner_through,
        )
        .translate(
            sx * (half_l - corner_r / 2.0),
            sy * (half_w - corner_r / 2.0),
            0.0,
        );
        let corner_cyl = centered_cylinder(
            format!("corner_cyl_{}_{}", si.0, si.1),
            corner_r,
            corner_through,
            32,
        )
        .translate(sx * (half_l - corner_r), sy * (half_w - corner_r), 0.0);
        let corner_cut = corner_sq - corner_cyl;
        channel_plate = channel_plate - corner_cut;
    }

    // ════════════════════════════════════════════════════
    // PART 2: Cover Plate (alignment holes + corner radii)
    // ════════════════════════════════════════════════════

    let mut cover_plate = centered_cube(
        "cover_plate",
        CNC16_CHIP_LENGTH,
        CNC16_CHIP_WIDTH,
        CNC16_COVER_THICKNESS,
    );

    // ── Alignment through-holes in cover plate (match channel plate pin holes) ──
    let cover_through = CNC16_COVER_THICKNESS + 1.0;
    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        let align_hole = centered_cylinder(
            format!("cover_align_{i}"),
            align_dia / 2.0,
            cover_through,
            24,
        )
        .translate(ax, ay, 0.0);
        cover_plate = cover_plate - align_hole;
    }

    // ── Corner radii on cover plate (same as channel plate) ──
    let cover_half_l = CNC16_CHIP_LENGTH / 2.0;
    let cover_half_w = CNC16_CHIP_WIDTH / 2.0;
    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let csq = centered_cube(
            format!("cover_cr_sq_{}_{}", si.0, si.1),
            corner_r,
            corner_r,
            cover_through,
        )
        .translate(
            sx * (cover_half_l - corner_r / 2.0),
            sy * (cover_half_w - corner_r / 2.0),
            0.0,
        );
        let ccyl = centered_cylinder(
            format!("cover_cr_cyl_{}_{}", si.0, si.1),
            corner_r,
            cover_through,
            32,
        )
        .translate(
            sx * (cover_half_l - corner_r),
            sy * (cover_half_w - corner_r),
            0.0,
        );
        cover_plate = cover_plate - (csq - ccyl);
    }

    // ════════════════════════════════════════════════════
    // EXPORT STL FILES
    // ════════════════════════════════════════════════════

    channel_plate
        .write_stl("output/microfluidic_chip_16ch_channel_plate.stl")
        .unwrap();

    cover_plate
        .write_stl("output/microfluidic_chip_16ch_cover_plate.stl")
        .unwrap();

    // ════════════════════════════════════════════════════
    // SUMMARY
    // ════════════════════════════════════════════════════

    println!("Exported:");
    println!("  output/microfluidic_chip_16ch_channel_plate.stl");
    println!("  output/microfluidic_chip_16ch_cover_plate.stl");
    println!();
    println!("== CNC-Milled 16-Chamber Microfluidic Chip (Rev B) ==");
    println!();
    println!("  PART 1: Channel Plate");
    println!("    Overall:          {length:.2}mm x {width:.2}mm x {thickness:.0}mm");
    println!("    Footprint:        ANSI/SLAS microplate (127.76 x 85.48mm)");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Chambers:         {chamber_idx} chambers in {num_rows}x{num_cols} grid");
    println!(
        "    Chamber size:     {chamber_w:.0}mm x {chamber_l:.0}mm x {chamber_d:.1}mm (200um)"
    );
    println!("    Channel width:    {ch_w:.1}mm (500um)  +/-50um");
    println!("    Channel depth:    {ch_d:.1}mm (200um)  +/-25um");
    println!("    Channel length:   {ch_len:.1}mm (port to chamber)");
    println!("    Port holes:       32x dia{port_dia:.1}mm through (16 inlet + 16 outlet)");
    println!("    Port connection:  Press-fit for 1/16\" OD PTFE tubing (NOT barb fittings)");
    println!("    Alignment holes:  4x dia{align_dia:.0}mm x {align_depth:.1}mm deep (blind)");
    println!("    Mounting holes:   4x dia{mount_dia:.1}mm through (M3 clearance)");
    println!("    Corner radius:    {corner_r:.2}mm (ANSI/SLAS 1-2004 §4.1.2.1)");
    println!("    Label area:       {label_l:.0}mm x {label_w:.0}mm x {label_d:.1}mm engraved (chip center)");
    println!();
    println!("  PART 2: Cover Plate");
    println!(
        "    Overall:          {:.2}mm x {:.2}mm x {:.0}mm",
        CNC16_CHIP_LENGTH, CNC16_CHIP_WIDTH, CNC16_COVER_THICKNESS
    );
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Alignment holes:  4x dia{align_dia:.0}mm through (matching channel plate)");
    println!("    Corner radius:    {corner_r:.2}mm (ANSI/SLAS 1-2004 §4.1.2.1)");
    println!();
    println!("  Assembly: Bond cover plate to channel plate with Weld-On 3");
    println!("  End mills: 500um flat (channels), 3mm flat (chambers), 1.6mm drill (ports)");
    println!();
    println!("  GRID LAYOUT (center-to-center: {col_spacing:.0}mm col, {row_spacing:.0}mm row):");
    println!(
        "    Column X positions: {:?}",
        col_xs
            .iter()
            .map(|x| format!("{x:+.1}"))
            .collect::<Vec<_>>()
    );
    println!(
        "    Row Y positions:    {:?}",
        row_ys
            .iter()
            .map(|y| format!("{y:+.1}"))
            .collect::<Vec<_>>()
    );
    println!();
    println!("  CHAMBER ALLOCATION:");
    for (i, label) in CHAMBER_LABELS.iter().enumerate() {
        let row = i / num_cols;
        let col = i % num_cols;
        let cx = col_xs[col];
        let cy = row_ys[row];
        println!(
            "    Ch{:2} [{},{}]: X={cx:+7.2}, Y={cy:+7.2}  {label}",
            i + 1,
            row,
            col
        );
    }
    println!();
    println!("  PORT COORDINATES (from chip center):");
    for (idx, in_x, in_y, out_x, out_y) in &port_positions {
        println!(
            "    Ch{:2}: Inlet ({in_x:+7.2}, {in_y:+7.2})  Outlet ({out_x:+7.2}, {out_y:+7.2})",
            idx + 1
        );
    }
    println!();
    println!("  ALIGNMENT HOLES:");
    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        println!("    Align {}: ({ax:+7.2}, {ay:+7.2})", i + 1);
    }
    println!();
    println!("  MOUNTING HOLES:");
    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        println!("    Mount {}: ({mx:+7.2}, {my:+7.2})", i + 1);
    }
}
