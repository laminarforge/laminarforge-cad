use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── CNC-Milled 16-Chamber Microfluidic Chip (Rev C — Open-Well, Two-Sided) ───
//
// AAV selectivity screening chip with 16 independent culture chambers.
// ANSI/SLAS microplate footprint: 127.76mm x 85.48mm x 14.35mm total height.
// Compatible with standard plate readers, incubator racks, robotic liquid handlers.
//
// Construction:
//   Part 1: PMMA plate — 127.76mm x 85.48mm x 14.20mm cast PMMA
//           TOP face:    Open wells (4mm Ø × 10mm deep) + vias (1mm Ø × 4mm)
//           BOTTOM face: Channels (500μm × 200μm) + chambers (3mm × 7mm × 200μm)
//   Part 2: Glass bottom — 127.76mm x 85.48mm x 0.15mm borosilicate (Borofloat 33)
//           Bonded to PMMA bottom face with ARcare 90106NB PSA tape gasket.
//
// Two-sided CNC machining:
//   Op 1 (top face down): Machine top to 14.20mm, drill wells, drill vias,
//                          drill alignment + mounting holes
//   Flip via dowel pins (±25μm alignment)
//   Op 2 (bottom face up): Mill channels, mill chambers, engrave label
//
// Well layout: 96-well grid positions — columns 2,5,8,11; row pairs (A,B)-(G,H)
//   Each chamber has inlet well (row A/C/E/G) and outlet well (row B/D/F/H)
//   32 wells total, 16 chambers, gravity-driven rocker flow
//
// Cross-section (Y-Z through one chamber column):
//
//   TOP FACE (wells + vias)
//   ─────────────────────────────────────────────────────
//   │  ┌──┐                              ┌──┐          │
//   │  │  │ inlet well                   │  │ outlet   │
//   │  │  │ 4mm Ø × 10mm                │  │ well     │
//   │  │  │                              │  │          │
//   │  │  │                              │  │          │
//   │  └─○┘ via 1mm Ø                   └─○┘ via      │  14.20mm PMMA
//   │    │  4mm long                       │           │
//   │    │                                 │           │
//   │    ║══════════╗     chamber     ╔════║           │
//   │    ║  channel ║  3×7×0.2mm     ║    ║           │
//   ─────║──────────║────────────────║────║───────────
//   │    ║  0.5×0.2 ║                ║    ║           │  0.15mm glass
//   ─────────────────────────────────────────────────────
//   BOTTOM FACE (channels + chambers, sealed by glass)
//
// Chamber numbering (row-major, 4×4 grid):
//   Row 0: Ch1  DRG neurons    | Ch2  Hepatocytes     | Ch3  Motor neurons   | Ch4  Cortical neurons
//   Row 1: Ch5  Astrocytes     | Ch6  Cardiomyocytes  | Ch7  Skeletal muscle | Ch8  Endothelial
//   Row 2: Ch9  Kidney         | Ch10 Lung            | Ch11 PBMCs           | Ch12 Pancreatic beta
//   Row 3: Ch13 Enteric neurons| Ch14 Schwann cells   | Ch15 HEK293 (+ctrl)  | Ch16 Empty (-ctrl)

fn main() {
    // ── Dimensions from lib.rs ──

    let length = REVC_CHIP_LENGTH;       // 127.76mm
    let width = REVC_CHIP_WIDTH;         // 85.48mm
    let thickness = REVC_PMMA_THICKNESS; // 14.20mm
    let glass_t = REVC_GLASS_THICKNESS;  // 0.15mm
    let corner_r = REVC_CORNER_RADIUS;   // 3.18mm

    let well_dia = REVC_WELL_DIAMETER;   // 4.0mm
    let well_depth = REVC_WELL_DEPTH;    // 10.0mm

    let via_dia = REVC_VIA_DIAMETER;     // 1.0mm
    let via_len = REVC_VIA_LENGTH;       // 4.0mm

    let chamber_w = REVC_CHAMBER_WIDTH;  // 3.0mm
    let chamber_l = REVC_CHAMBER_LENGTH; // 7.0mm
    let chamber_d = REVC_CHAMBER_DEPTH;  // 0.2mm

    let ch_w = REVC_CHANNEL_WIDTH;       // 0.5mm
    let ch_d = REVC_CHANNEL_DEPTH;       // 0.2mm
    let ch_len = REVC_CHANNEL_LENGTH;    // 1.0mm

    let num_cols = REVC_GRID_COLS;       // 4
    let _num_rows = REVC_GRID_ROWS;      // 4

    let align_dia = REVC_ALIGN_DIAMETER; // 1.0mm
    let align_depth = REVC_ALIGN_DEPTH;  // 0.5mm
    let align_inset = REVC_ALIGN_INSET;  // 4.0mm

    let mount_dia = REVC_MOUNT_DIAMETER; // 3.2mm
    let mount_inset = REVC_MOUNT_INSET;  // 6.0mm

    let label_l = REVC_LABEL_LENGTH;     // 20.0mm
    let label_w = REVC_LABEL_WIDTH;      // 5.0mm
    let label_d = REVC_LABEL_DEPTH;      // 0.1mm

    // ── Coordinate system ──
    // Chip centered at origin. Z=0 is chip center.
    // Top face at Z = +thickness/2 = +7.10mm
    // Bottom face at Z = -thickness/2 = -7.10mm
    // Glass sits below bottom face (separate part).

    let chip_top = thickness / 2.0;      // +7.10mm
    let chip_bot = -thickness / 2.0;     // -7.10mm

    // Column X positions (chip-centered coordinates from lib.rs)
    let col_xs = REVC_COL_XS_CENTERED;  // [-40.50, -13.50, 13.50, 40.50]

    // Chamber center Y positions (chip-centered)
    let chamber_ys = REVC_CHAMBER_CENTER_YS; // [-27.00, -9.00, 9.00, 27.00]

    // Inlet/outlet well Y positions (chip-centered)
    let inlet_ys = REVC_INLET_YS;       // [-31.50, -13.50, 4.50, 22.50]
    let outlet_ys = REVC_OUTLET_YS;     // [-22.50, -4.50, 13.50, 31.50]

    // ── Cutting tool oversizes (for clean CSG subtraction) ──
    let well_tool_h = well_depth + 0.2;  // slightly taller than well
    let well_z = chip_top - well_depth / 2.0 + 0.1; // center Z for well cutter

    let via_tool_h = via_len + 0.2;
    let via_z = chip_top - well_depth - via_len / 2.0; // starts at well bottom, goes down

    let chamber_tool_h = chamber_d + 0.2;
    let chamber_z = chip_bot + chamber_d / 2.0 - 0.1; // milled from bottom face

    let channel_tool_h = ch_d + 0.2;
    let channel_z = chip_bot + ch_d / 2.0 - 0.1;

    let align_tool_h = align_depth + 0.2;
    let align_z = chip_top - align_depth / 2.0 + 0.1;

    let label_tool_h = label_d + 0.2;
    let label_z = chip_bot + label_d / 2.0 - 0.1; // label on bottom face

    let through_h = thickness + 1.0; // exceeds plate thickness for through-holes

    // ════════════════════════════════════════════════════
    // PART 1: PMMA Plate
    // ════════════════════════════════════════════════════

    let mut plate = centered_cube("pmma_plate", length, width, thickness);

    // ── Corner radii (ANSI/SLAS 1-2004 §4.1.2.1: 3.18 ± 1.6mm) ──
    let half_l = length / 2.0;
    let half_w = width / 2.0;
    let corner_through = thickness + 1.0;

    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let corner_sq = centered_cube(
            &format!("cr_sq_{}_{}", si.0, si.1),
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
            &format!("cr_cyl_{}_{}", si.0, si.1),
            corner_r,
            corner_through,
            32,
        )
        .translate(
            sx * (half_l - corner_r),
            sy * (half_w - corner_r),
            0.0,
        );
        plate = plate - (corner_sq - corner_cyl);
    }

    // ── TOP FACE: Wells + Vias ──
    // 32 wells (16 inlet + 16 outlet) on 96-well grid positions

    let mut chamber_idx = 0usize;

    for (row, &cy) in chamber_ys.iter().enumerate() {
        let inlet_y = inlet_ys[row];
        let outlet_y = outlet_ys[row];

        for (col, &cx) in col_xs.iter().enumerate() {
            let idx = row * num_cols + col;

            // ── Inlet well (open from top) ──
            let inlet_well = centered_cylinder(
                &format!("inlet_well_{idx}"),
                well_dia / 2.0,
                well_tool_h,
                32,
            )
            .translate(cx, inlet_y, well_z);
            plate = plate - inlet_well;

            // ── Outlet well (open from top) ──
            let outlet_well = centered_cylinder(
                &format!("outlet_well_{idx}"),
                well_dia / 2.0,
                well_tool_h,
                32,
            )
            .translate(cx, outlet_y, well_z);
            plate = plate - outlet_well;

            // ── Inlet via (from well bottom down to bottom face) ──
            let inlet_via = centered_cylinder(
                &format!("inlet_via_{idx}"),
                via_dia / 2.0,
                via_tool_h,
                24,
            )
            .translate(cx, inlet_y, via_z);
            plate = plate - inlet_via;

            // ── Outlet via (from well bottom down to bottom face) ──
            let outlet_via = centered_cylinder(
                &format!("outlet_via_{idx}"),
                via_dia / 2.0,
                via_tool_h,
                24,
            )
            .translate(cx, outlet_y, via_z);
            plate = plate - outlet_via;

            // ── BOTTOM FACE: Chamber ──
            let chamber = centered_cube(
                &format!("chamber_{idx}"),
                chamber_w,
                chamber_l,
                chamber_tool_h,
            )
            .translate(cx, cy, chamber_z);
            plate = plate - chamber;

            // ── BOTTOM FACE: Inlet channel (via to chamber) ──
            // Channel runs from inlet via Y position to chamber bottom edge
            let ch_inlet_center_y = (inlet_y + (cy - chamber_l / 2.0)) / 2.0;
            let ch_inlet_len = (cy - chamber_l / 2.0) - inlet_y;
            let inlet_ch = centered_cube(
                &format!("inlet_ch_{idx}"),
                ch_w,
                ch_inlet_len.abs(),
                channel_tool_h,
            )
            .translate(cx, ch_inlet_center_y, channel_z);
            plate = plate - inlet_ch;

            // ── BOTTOM FACE: Outlet channel (chamber to via) ──
            let ch_outlet_center_y = ((cy + chamber_l / 2.0) + outlet_y) / 2.0;
            let ch_outlet_len = outlet_y - (cy + chamber_l / 2.0);
            let outlet_ch = centered_cube(
                &format!("outlet_ch_{idx}"),
                ch_w,
                ch_outlet_len.abs(),
                channel_tool_h,
            )
            .translate(cx, ch_outlet_center_y, channel_z);
            plate = plate - outlet_ch;

            chamber_idx += 1;
            let _ = (row, col); // suppress unused warnings
        }
    }

    // ── Alignment pin holes (4 corners, blind from top) ──
    let align_positions = [
        (-(half_l - align_inset), -(half_w - align_inset)),
        (-(half_l - align_inset), half_w - align_inset),
        (half_l - align_inset, -(half_w - align_inset)),
        (half_l - align_inset, half_w - align_inset),
    ];

    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        let align_hole = centered_cylinder(
            &format!("align_{i}"),
            align_dia / 2.0,
            align_tool_h,
            24,
        )
        .translate(ax, ay, align_z);
        plate = plate - align_hole;
    }

    // ── Mounting holes (4 near corners, M3 through-holes) ──
    let mount_positions = [
        (-(half_l - mount_inset), -(half_w - mount_inset)),
        (-(half_l - mount_inset), half_w - mount_inset),
        (half_l - mount_inset, -(half_w - mount_inset)),
        (half_l - mount_inset, half_w - mount_inset),
    ];

    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        let mount_hole = centered_cylinder(
            &format!("mount_{i}"),
            mount_dia / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, 0.0);
        plate = plate - mount_hole;
    }

    // ── Label area (shallow engraving on BOTTOM face) ──
    let label_pocket = centered_cube("label_area", label_l, label_w, label_tool_h)
        .translate(0.0, 0.0, label_z);
    plate = plate - label_pocket;

    // ════════════════════════════════════════════════════
    // PART 2: Glass Bottom Plate
    // ════════════════════════════════════════════════════

    let mut glass = centered_cube("glass_bottom", length, width, glass_t);

    // Corner radii on glass (same footprint as PMMA)
    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let gsq = centered_cube(
            &format!("glass_cr_sq_{}_{}", si.0, si.1),
            corner_r,
            corner_r,
            glass_t + 0.1,
        )
        .translate(
            sx * (half_l - corner_r / 2.0),
            sy * (half_w - corner_r / 2.0),
            0.0,
        );
        let gcyl = centered_cylinder(
            &format!("glass_cr_cyl_{}_{}", si.0, si.1),
            corner_r,
            glass_t + 0.1,
            32,
        )
        .translate(
            sx * (half_l - corner_r),
            sy * (half_w - corner_r),
            0.0,
        );
        glass = glass - (gsq - gcyl);
    }

    // ════════════════════════════════════════════════════
    // EXPORT STL FILES
    // ════════════════════════════════════════════════════

    plate
        .write_stl("output/microfluidic_chip_revc_pmma.stl")
        .unwrap();

    glass
        .write_stl("output/microfluidic_chip_revc_glass.stl")
        .unwrap();

    // ════════════════════════════════════════════════════
    // SUMMARY
    // ════════════════════════════════════════════════════

    println!("Exported:");
    println!("  output/microfluidic_chip_revc_pmma.stl");
    println!("  output/microfluidic_chip_revc_glass.stl");
    println!();
    println!("== CNC-Milled 16-Chamber Microfluidic Chip (Rev C) ==");
    println!("== Open-Well, Two-Sided CNC, Gravity-Driven Flow   ==");
    println!();
    println!("  PART 1: PMMA Plate");
    println!("    Overall:          {length:.2}mm x {width:.2}mm x {thickness:.2}mm");
    println!("    Footprint:        ANSI/SLAS microplate (127.76 x 85.48mm)");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Total height:     {:.2}mm (PMMA) + {:.2}mm (glass) = {:.2}mm (SLAS 2-2004)",
        thickness, glass_t, REVC_TOTAL_HEIGHT);
    println!("    Corner radius:    {corner_r:.2}mm (ANSI/SLAS 1-2004 §4.1.2.1)");
    println!();
    println!("  TOP FACE (Op 1):");
    println!("    Wells:            32x dia{well_dia:.0}mm x {well_depth:.0}mm deep (open-top)");
    println!("    Well volume:      ~{:.0}μL each (π×r²×h)", REVC_WELL_VOLUME_UL);
    println!("    Vias:             32x dia{via_dia:.0}mm x {via_len:.0}mm (well bottom → channel)");
    println!("    Via aspect ratio: {:.1}:1", via_len / via_dia);
    println!("    Alignment holes:  4x dia{align_dia:.0}mm x {align_depth:.1}mm deep (blind)");
    println!("    Mounting holes:   4x dia{mount_dia:.1}mm through (M3 clearance)");
    println!();
    println!("  BOTTOM FACE (Op 2):");
    println!("    Chambers:         {chamber_idx}x {chamber_w:.0}mm x {chamber_l:.0}mm x {chamber_d:.1}mm");
    println!("    Channels:         32x {ch_w:.1}mm x {ch_d:.1}mm x ~{ch_len:.0}mm (via→chamber)");
    println!("    Label area:       {label_l:.0}mm x {label_w:.0}mm x {label_d:.1}mm engraved");
    println!();
    println!("  PART 2: Glass Bottom");
    println!("    Material:         150μm borosilicate (Borofloat 33)");
    println!("    Bonding:          ARcare 90106NB PSA tape gasket (140μm)");
    println!();
    println!("  WELL LAYOUT (96-well grid positions):");
    println!("    Column X (centered): {:?}", col_xs);
    println!("    Grid columns:        {:?}", REVC_COL_GRID_INDICES);
    println!();

    for (i, label) in REVC_CHAMBER_LABELS.iter().enumerate() {
        let row = i / num_cols;
        let col = i % num_cols;
        let cx = col_xs[col];
        let cy = chamber_ys[row];
        let (inlet_pos, outlet_pos) = REVC_WELL_POSITIONS[i];
        println!("    Ch{:2} [{inlet_pos},{outlet_pos}]: X={cx:+7.2}, Y={cy:+7.2}  {label}",
            i + 1);
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
    println!();
    println!("  CNC TOOLING:");
    println!("    Wells:    3mm flat carbide, 20kRPM, 500mm/min");
    println!("    Vias:     1mm carbide drill, 20kRPM, peck 0.5mm");
    println!("    Chambers: 1mm flat carbide, 18kRPM, 60mm/min");
    println!("    Channels: 0.3mm flat carbide, 20kRPM, 200mm/min");
    println!("    Profile:  3.175mm flat carbide, 20kRPM, 1000mm/min");
    println!();
    println!("  ASSEMBLY:");
    println!("    1. O₂ plasma treat bottom face (optional)");
    println!("    2. Apply PLL coating to wells + channels");
    println!("    3. Align die-cut PSA tape gasket on bottom face");
    println!("    4. Press 150μm borosilicate glass onto tape");
    println!("    5. Insert 1mm alignment dowel pins");
    println!("    6. UV sterilize packaged chip (60 min)");
}
