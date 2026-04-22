use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Rev D Microfluidic Chip — Integrated Sensors + Unidirectional Flow + Fluidic Vias ───
//
// Evolution from Rev C (microfluidic_chip_cnc_revc.rs). Maintains ANSI/SLAS
// microplate footprint: 127.76 × 85.48 × 14.35mm. Backward compatible with
// Rev C rack, rocker, LH interface, imaging gantry.
//
// Key upgrades vs Rev C:
//   1. Integrated TEER electrodes: 4 gold pads per chamber (2 bottom, 2 top)
//      for 4-electrode Kelvin TEER measurement. Eliminates manual chopstick.
//   2. Expanded reservoirs: 5 inlet + 5 outlet wells per chamber (Rev C was
//      1+1). Wells 6mm Ø × 8mm deep → 226µL each (vs 125µL on Rev C).
//   3. Integrated Pt-film thermistor trace for local chip temperature sensing.
//   4. Asymmetric-port channel geometry for 2-axis tilt unidirectional flow
//      (Busek et al. Lab on a Chip 2023, d2lc00919f).
//   5. Fluidic vias with O-ring seats on the SHORT edges — enable chip stacking
//      for multi-organ-on-chip without external tubing.
//   6. Fewer, larger chambers: 4 chambers (was 16 on Rev C) — trade throughput
//      for richer per-chamber instrumentation.
//
// Coordinate convention (same as Rev C):
//   Chip centered at origin. X = long edge (127.76mm), Y = short edge (85.48mm).
//   Top face at Z = +7.10mm, bottom face at Z = -7.10mm. Glass below bottom.
//
// Manufacturing sequence (adds sputter step vs Rev C):
//   Op 1: Machine PMMA top face (wells, vias, mount/align holes, thermistor pocket)
//   Op 2: Flip and machine PMMA bottom face (channels, chambers, label, via wells)
//   Op 3: Sputter-coat glass substrate — Ti 10nm + Au 100nm electrodes + traces
//         through shadow mask; lift-off + Pt 100nm for thermistor RTD
//   Op 4: Align + PSA-bond glass to PMMA (ARcare 90106NB, 140µm)
//   Op 5: Press-fit O-rings into via seats; plug unused vias.

// ════════════════════════════════════════════════════════════════════════════
// Rev D design constants (local — promote to lib.rs once frozen)
// ════════════════════════════════════════════════════════════════════════════

const CHIP_LENGTH: f64 = REVC_CHIP_LENGTH;       // 127.76mm (X, long edge)
const CHIP_WIDTH: f64 = REVC_CHIP_WIDTH;         // 85.48mm  (Y, short edge)
const PMMA_THICKNESS: f64 = REVC_PMMA_THICKNESS; // 14.20mm
const GLASS_THICKNESS: f64 = REVC_GLASS_THICKNESS; // 0.15mm
const TOTAL_HEIGHT: f64 = REVC_TOTAL_HEIGHT;     // 14.35mm (ANSI/SLAS 2-2004)
const CORNER_R: f64 = REVC_CORNER_RADIUS;        // 3.18mm

// ── Reservoir wells (Rev D: 5 inlet + 5 outlet per chamber) ──
const WELL_DIAMETER: f64 = 6.0;   // mm (Rev C: 4.0mm)
const WELL_DEPTH: f64 = 8.0;      // mm (Rev C: 10.0mm) — leaves 6.2mm floor
const WELL_VOLUME_UL: f64 = std::f64::consts::PI * 3.0 * 3.0 * 8.0; // ≈ 226.2µL
const WELL_PITCH: f64 = 9.0;      // mm, 96-well grid pitch (LH compatibility)
const WELLS_PER_SIDE: usize = 5;  // 5 inlet + 5 outlet per chamber

// ── Chamber + channel geometry ──
// Fewer, larger chambers to make room for per-chamber TEER + thermistor and
// to handle the larger reservoir volumes.
const NUM_CHAMBERS: usize = 4;        // 2×2 grid (Rev C was 4×4 = 16)
const GRID_COLS: usize = 2;
const GRID_ROWS: usize = 2;
const CHAMBER_WIDTH: f64 = 6.0;       // mm (X)
const CHAMBER_LENGTH: f64 = 14.0;     // mm (Y) — longer to accept shear flow
const CHAMBER_DEPTH: f64 = 0.2;       // 200µm (same as Rev C)

const CHANNEL_WIDTH: f64 = 0.5;       // 500µm
const CHANNEL_DEPTH_SHALLOW: f64 = 0.15; // 150µm — low port
const CHANNEL_DEPTH_DEEP: f64 = 0.35;    // 350µm — high port
// Asymmetric port depths create a "hydraulic diode": paired with 2-axis tilt,
// the deep port always feeds, the shallow port always drains, regardless of
// which axis is tilted down. See derivation in the Rev D spec artifact.

const VIA_DIAMETER: f64 = 1.0;
const VIA_LENGTH: f64 = PMMA_THICKNESS - WELL_DEPTH - 0.2; // ≈ 6.0mm

// ── TEER electrodes ──
const TEER_PAD_W: f64 = 1.0;       // mm (X) — 1mm × 3mm gold pad
const TEER_PAD_L: f64 = 3.0;       // mm (Y)
// TEER_PAD_THICKNESS: 100nm Au over 10nm Ti — deposited by sputter, not machined.
// We mill a 50µm pocket (TEER_POCKET_DEPTH) so the film sits below surface.
const TEER_POCKET_DEPTH: f64 = 0.05;    // 50µm recess so Au pad doesn't protrude
const TEER_TRACE_WIDTH: f64 = 0.2; // 200µm gold trace to edge pad

// ── Thermistor (Pt-film RTD) ──
const RTD_SERPENTINE_W: f64 = 2.0; // mm (X)
const RTD_SERPENTINE_L: f64 = 10.0; // mm (Y)
const RTD_POCKET_DEPTH: f64 = 0.03; // 30µm pocket for Ti/Pt film

// ── Edge contact pads (pogo-pin landing) ──
const EDGE_PAD_W: f64 = 2.0;       // mm
const EDGE_PAD_L: f64 = 3.0;       // mm
const EDGE_PAD_POCKET_DEPTH: f64 = 0.05; // 50µm
const EDGE_PAD_INSET_X: f64 = 10.0;      // from +X long edge
const EDGE_PAD_PITCH_Y: f64 = 4.0;       // pad-to-pad Y pitch

// ── Fluidic vias on the SHORT (±X) edges for chip stacking ──
const FLUIDIC_VIA_ID: f64 = 1.5;         // mm inner diameter
const FLUIDIC_VIA_OD_SEAT: f64 = 3.0;    // mm O-ring seat OD
const FLUIDIC_VIA_SEAT_DEPTH: f64 = 1.0; // mm O-ring groove depth
const FLUIDIC_VIA_INSET_FROM_EDGE: f64 = 8.0; // from short edge

// ── Alignment + mounting (same as Rev C, keep drop-in rack compatibility) ──
const ALIGN_DIAMETER: f64 = REVC_ALIGN_DIAMETER; // 1.0mm
const ALIGN_DEPTH: f64 = REVC_ALIGN_DEPTH;       // 0.5mm
const ALIGN_INSET: f64 = REVC_ALIGN_INSET;       // 4.0mm
const MOUNT_DIAMETER: f64 = REVC_MOUNT_DIAMETER; // 3.2mm
const MOUNT_INSET: f64 = REVC_MOUNT_INSET;       // 6.0mm

// Chip-centered chamber positions (2×2 grid)
// X spacing: 54mm (spans wide enough for 5+5 wells per chamber in Y)
// Y spacing: ~32mm between chamber centers
const CHAMBER_XS: [f64; 2] = [-30.0, 30.0];
const CHAMBER_YS: [f64; 2] = [-16.0, 16.0];

// Chamber labels — 4 endothelial-centric tissues for unidirectional-flow study
const CHAMBER_LABELS: [&str; 4] = [
    "Endothelial (arterial shear)",
    "Hepatocyte (portal flow)",
    "Renal proximal tubule",
    "Gut enterocyte (Caco-2)",
];

// Well stripe orientation: 5 inlet wells arrayed along X on -Y side of chamber,
// 5 outlet wells arrayed along X on +Y side of chamber. Inlet/outlet stripes
// offset from chamber edge by 6mm (for via length + channel routing).
const WELL_STRIPE_Y_OFFSET: f64 = 10.0; // well center to chamber center, |Y|

// ════════════════════════════════════════════════════════════════════════════
// Modular builders
// ════════════════════════════════════════════════════════════════════════════

/// Rounded-corner PMMA outer body (same profile as Rev C for rack compatibility).
fn outer_body() -> Part {
    let mut plate = centered_cube("pmma_plate", CHIP_LENGTH, CHIP_WIDTH, PMMA_THICKNESS);
    let half_l = CHIP_LENGTH / 2.0;
    let half_w = CHIP_WIDTH / 2.0;
    let through = PMMA_THICKNESS + 1.0;

    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let corner_sq = centered_cube(
            &format!("cr_sq_{}_{}", si.0, si.1),
            CORNER_R,
            CORNER_R,
            through,
        )
        .translate(
            sx * (half_l - CORNER_R / 2.0),
            sy * (half_w - CORNER_R / 2.0),
            0.0,
        );
        let corner_cyl = centered_cylinder(
            &format!("cr_cyl_{}_{}", si.0, si.1),
            CORNER_R,
            through,
            32,
        )
        .translate(sx * (half_l - CORNER_R), sy * (half_w - CORNER_R), 0.0);
        plate = plate - (corner_sq - corner_cyl);
    }

    // Alignment pin holes (blind, from top)
    let half_l = CHIP_LENGTH / 2.0;
    let half_w = CHIP_WIDTH / 2.0;
    let chip_top = PMMA_THICKNESS / 2.0;
    let align_z = chip_top - ALIGN_DEPTH / 2.0 + 0.1;
    let align_tool_h = ALIGN_DEPTH + 0.2;
    for (i, &(sx, sy)) in [(-1.0_f64, -1.0_f64), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let ax = sx * (half_l - ALIGN_INSET);
        let ay = sy * (half_w - ALIGN_INSET);
        let hole = centered_cylinder(
            &format!("align_{i}"),
            ALIGN_DIAMETER / 2.0,
            align_tool_h,
            24,
        )
        .translate(ax, ay, align_z);
        plate = plate - hole;
    }

    // M3 mounting holes (through)
    let through_h = PMMA_THICKNESS + 1.0;
    for (i, &(sx, sy)) in [(-1.0_f64, -1.0_f64), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let mx = sx * (half_l - MOUNT_INSET);
        let my = sy * (half_w - MOUNT_INSET);
        let hole = centered_cylinder(
            &format!("mount_{i}"),
            MOUNT_DIAMETER / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, 0.0);
        plate = plate - hole;
    }

    plate
}

/// Expanded reservoir wells: 5 inlet + 5 outlet per chamber = 40 wells total.
fn subtract_wells(mut plate: Part) -> Part {
    let chip_top = PMMA_THICKNESS / 2.0;
    let well_tool_h = WELL_DEPTH + 0.2;
    let well_z = chip_top - WELL_DEPTH / 2.0 + 0.1;

    // For each chamber: 5 inlet wells along X on -Y side, 5 outlet wells on +Y side.
    for (ci, &cx) in CHAMBER_XS.iter().enumerate() {
        for (ri, &cy) in CHAMBER_YS.iter().enumerate() {
            let chamber_id = ri * GRID_COLS + ci;
            let inlet_y = cy - WELL_STRIPE_Y_OFFSET;
            let outlet_y = cy + WELL_STRIPE_Y_OFFSET;

            // Inlet stripe: 5 wells centered on cx, spaced WELL_PITCH along X
            for k in 0..WELLS_PER_SIDE {
                let dx = (k as f64 - (WELLS_PER_SIDE as f64 - 1.0) / 2.0) * WELL_PITCH;
                let well_x = cx + dx;
                let well = centered_cylinder(
                    &format!("inlet_well_c{chamber_id}_{k}"),
                    WELL_DIAMETER / 2.0,
                    well_tool_h,
                    32,
                )
                .translate(well_x, inlet_y, well_z);
                plate = plate - well;
            }
            // Outlet stripe
            for k in 0..WELLS_PER_SIDE {
                let dx = (k as f64 - (WELLS_PER_SIDE as f64 - 1.0) / 2.0) * WELL_PITCH;
                let well_x = cx + dx;
                let well = centered_cylinder(
                    &format!("outlet_well_c{chamber_id}_{k}"),
                    WELL_DIAMETER / 2.0,
                    well_tool_h,
                    32,
                )
                .translate(well_x, outlet_y, well_z);
                plate = plate - well;
            }
        }
    }
    plate
}

/// Vertical vias — 1mm Ø drills from well bottom to bottom face.
fn subtract_vias(mut plate: Part) -> Part {
    let chip_top = PMMA_THICKNESS / 2.0;
    let via_tool_h = VIA_LENGTH + 0.2;
    let via_z = chip_top - WELL_DEPTH - VIA_LENGTH / 2.0;

    for (ci, &cx) in CHAMBER_XS.iter().enumerate() {
        for (ri, &cy) in CHAMBER_YS.iter().enumerate() {
            let chamber_id = ri * GRID_COLS + ci;
            let inlet_y = cy - WELL_STRIPE_Y_OFFSET;
            let outlet_y = cy + WELL_STRIPE_Y_OFFSET;

            for k in 0..WELLS_PER_SIDE {
                let dx = (k as f64 - (WELLS_PER_SIDE as f64 - 1.0) / 2.0) * WELL_PITCH;
                let x = cx + dx;
                let via_in = centered_cylinder(
                    &format!("via_in_c{chamber_id}_{k}"),
                    VIA_DIAMETER / 2.0,
                    via_tool_h,
                    24,
                )
                .translate(x, inlet_y, via_z);
                plate = plate - via_in;
                let via_out = centered_cylinder(
                    &format!("via_out_c{chamber_id}_{k}"),
                    VIA_DIAMETER / 2.0,
                    via_tool_h,
                    24,
                )
                .translate(x, outlet_y, via_z);
                plate = plate - via_out;
            }
        }
    }
    plate
}

/// Chamber pockets + asymmetric-port channels milled into BOTTOM face.
/// Asymmetric port depths (150µm / 350µm) are what enables unidirectional flow
/// under 2-axis tilt — see spec artifact derivation.
fn subtract_channels_and_chambers(mut plate: Part) -> Part {
    let chip_bot = -PMMA_THICKNESS / 2.0;
    let chamber_tool_h = CHAMBER_DEPTH + 0.2;
    let chamber_z = chip_bot + CHAMBER_DEPTH / 2.0 - 0.1;

    for (ci, &cx) in CHAMBER_XS.iter().enumerate() {
        for (ri, &cy) in CHAMBER_YS.iter().enumerate() {
            let chamber_id = ri * GRID_COLS + ci;

            // Chamber pocket
            let chamber = centered_cube(
                &format!("chamber_{chamber_id}"),
                CHAMBER_WIDTH,
                CHAMBER_LENGTH,
                chamber_tool_h,
            )
            .translate(cx, cy, chamber_z);
            plate = plate - chamber;

            let inlet_y = cy - WELL_STRIPE_Y_OFFSET;
            let outlet_y = cy + WELL_STRIPE_Y_OFFSET;

            // 5 inlet channels from via → chamber (entering -Y edge)
            // Asymmetric depth: odd-indexed wells are "high" (350µm deep),
            // even-indexed are "low" (150µm). This creates the port-height
            // asymmetry that drives unidirectional flow under 2-axis tilt.
            let ch_in_y = (inlet_y + (cy - CHAMBER_LENGTH / 2.0)) / 2.0;
            let ch_in_len = ((cy - CHAMBER_LENGTH / 2.0) - inlet_y).abs();
            for k in 0..WELLS_PER_SIDE {
                let dx = (k as f64 - (WELLS_PER_SIDE as f64 - 1.0) / 2.0) * WELL_PITCH;
                let x = cx + dx;
                let depth = if k % 2 == 0 {
                    CHANNEL_DEPTH_SHALLOW
                } else {
                    CHANNEL_DEPTH_DEEP
                };
                let tool_h = depth + 0.2;
                let z = chip_bot + depth / 2.0 - 0.1;
                let ch = centered_cube(
                    &format!("inlet_ch_c{chamber_id}_{k}"),
                    CHANNEL_WIDTH,
                    ch_in_len,
                    tool_h,
                )
                .translate(x, ch_in_y, z);
                plate = plate - ch;
            }

            // 5 outlet channels from chamber (+Y edge) → via. INVERTED depth
            // pattern vs inlet side — this asymmetry is what turns bidirectional
            // rocker motion into unidirectional net flow under 2-axis tilt.
            let ch_out_y = ((cy + CHAMBER_LENGTH / 2.0) + outlet_y) / 2.0;
            let ch_out_len = (outlet_y - (cy + CHAMBER_LENGTH / 2.0)).abs();
            for k in 0..WELLS_PER_SIDE {
                let dx = (k as f64 - (WELLS_PER_SIDE as f64 - 1.0) / 2.0) * WELL_PITCH;
                let x = cx + dx;
                let depth = if k % 2 == 0 {
                    CHANNEL_DEPTH_DEEP
                } else {
                    CHANNEL_DEPTH_SHALLOW
                };
                let tool_h = depth + 0.2;
                let z = chip_bot + depth / 2.0 - 0.1;
                let ch = centered_cube(
                    &format!("outlet_ch_c{chamber_id}_{k}"),
                    CHANNEL_WIDTH,
                    ch_out_len,
                    tool_h,
                )
                .translate(x, ch_out_y, z);
                plate = plate - ch;
            }
        }
    }

    // Label area on bottom face
    let label_tool_h = REVC_LABEL_DEPTH + 0.2;
    let label_z = chip_bot + REVC_LABEL_DEPTH / 2.0 - 0.1;
    let label = centered_cube(
        "label_area",
        REVC_LABEL_LENGTH,
        REVC_LABEL_WIDTH,
        label_tool_h,
    )
    .translate(0.0, 0.0, label_z);
    plate = plate - label;

    plate
}

/// TEER electrode pockets on TOP face (where top lid electrodes sit in final
/// assembly) and BOTTOM face (for glass-deposited electrodes). Sputter mask
/// lands the actual Ti/Au film into these pockets so the metal is flush with
/// the PMMA surface and doesn't interfere with PSA bonding.
///
/// Four pads per chamber: 2 on the top lid (drive+, sense+), 2 on the glass
/// bottom (drive-, sense-). This is a 4-electrode Kelvin config which removes
/// lead/contact resistance from the TEER reading.
///
/// Plus 4 edge contact pads per chamber on the +X edge (pogo-pin landing for
/// readout board).
fn subtract_teer_electrodes(mut plate: Part) -> Part {
    let chip_top = PMMA_THICKNESS / 2.0;
    let chip_bot = -PMMA_THICKNESS / 2.0;

    let top_pocket_z = chip_top - TEER_POCKET_DEPTH / 2.0 + 0.05;
    let bot_pocket_z = chip_bot + TEER_POCKET_DEPTH / 2.0 - 0.05;
    let pocket_tool_h = TEER_POCKET_DEPTH + 0.1;

    let trace_tool_h = TEER_POCKET_DEPTH + 0.1;

    for (ci, &cx) in CHAMBER_XS.iter().enumerate() {
        for (ri, &cy) in CHAMBER_YS.iter().enumerate() {
            let chamber_id = ri * GRID_COLS + ci;

            // Two TOP lid electrode pockets — placed just inside chamber edges
            // so they face the chamber volume from above (via top lid).
            let top_pad_y0 = cy - CHAMBER_LENGTH / 4.0;
            let top_pad_y1 = cy + CHAMBER_LENGTH / 4.0;
            for (j, py) in [top_pad_y0, top_pad_y1].iter().enumerate() {
                let pocket = centered_cube(
                    &format!("teer_top_c{chamber_id}_{j}"),
                    TEER_PAD_W,
                    TEER_PAD_L,
                    pocket_tool_h,
                )
                .translate(cx, *py, top_pocket_z);
                plate = plate - pocket;
            }

            // Two BOTTOM (glass-side) electrode pockets
            for (j, py) in [top_pad_y0, top_pad_y1].iter().enumerate() {
                let pocket = centered_cube(
                    &format!("teer_bot_c{chamber_id}_{j}"),
                    TEER_PAD_W,
                    TEER_PAD_L,
                    pocket_tool_h,
                )
                .translate(cx, *py, bot_pocket_z);
                plate = plate - pocket;
            }

            // Gold trace routing — bottom face, from electrode pads toward +X
            // edge where the contact pads live. Straight trace for simplicity.
            let chip_half_l = CHIP_LENGTH / 2.0;
            let trace_end_x = chip_half_l - EDGE_PAD_INSET_X;
            let trace_start_x = cx;
            let trace_center_x = (trace_start_x + trace_end_x) / 2.0;
            let trace_len_x = (trace_end_x - trace_start_x).abs();

            for (j, py) in [top_pad_y0, top_pad_y1].iter().enumerate() {
                // Bottom-face trace (connects glass electrode to edge pad)
                let trace = centered_cube(
                    &format!("teer_trace_c{chamber_id}_{j}"),
                    trace_len_x,
                    TEER_TRACE_WIDTH,
                    trace_tool_h,
                )
                .translate(trace_center_x, *py, bot_pocket_z);
                plate = plate - trace;
            }

            // Edge contact pads on bottom face, +X edge (4 pads per chamber:
            // TEER drive+, sense+, drive-, sense-)
            let edge_x = chip_half_l - EDGE_PAD_INSET_X;
            for (j, py_base) in [top_pad_y0, top_pad_y1].iter().enumerate() {
                let py_hi = py_base + EDGE_PAD_PITCH_Y / 2.0;
                let py_lo = py_base - EDGE_PAD_PITCH_Y / 2.0;
                for (k, py) in [py_hi, py_lo].iter().enumerate() {
                    let pad = centered_cube(
                        &format!("edge_pad_c{chamber_id}_{j}_{k}"),
                        EDGE_PAD_W,
                        EDGE_PAD_L,
                        EDGE_PAD_POCKET_DEPTH + 0.1,
                    )
                    .translate(
                        edge_x,
                        *py,
                        bot_pocket_z,
                    );
                    plate = plate - pad;
                }
            }
        }
    }

    plate
}

/// Pt-film thermistor trace pocket — central serpentine on bottom face.
/// 2mm × 10mm footprint, 30µm pocket depth for Ti/Pt sputter deposition.
/// Two edge contact pads provide 4-wire sense access.
fn subtract_thermistor_trace(mut plate: Part) -> Part {
    let chip_bot = -PMMA_THICKNESS / 2.0;
    let pocket_z = chip_bot + RTD_POCKET_DEPTH / 2.0 - 0.05;
    let tool_h = RTD_POCKET_DEPTH + 0.1;

    // Main serpentine footprint (simplified as a rectangular pocket — actual
    // serpentine is defined by the sputter shadow mask).
    let rtd_pocket = centered_cube(
        "rtd_pocket",
        RTD_SERPENTINE_W,
        RTD_SERPENTINE_L,
        tool_h,
    )
    .translate(0.0, 0.0, pocket_z);
    plate = plate - rtd_pocket;

    // 4-wire pads near +X edge (drive+, drive-, sense+, sense-)
    let chip_half_l = CHIP_LENGTH / 2.0;
    let pad_x = chip_half_l - EDGE_PAD_INSET_X;
    for (i, py_off) in [-6.0_f64, -2.0, 2.0, 6.0].iter().enumerate() {
        let pad = centered_cube(
            &format!("rtd_pad_{i}"),
            EDGE_PAD_W,
            EDGE_PAD_L,
            EDGE_PAD_POCKET_DEPTH + 0.1,
        )
        .translate(pad_x, *py_off, pocket_z);
        plate = plate - pad;
    }

    // Trace from serpentine to pad cluster
    let trace_center_x = (0.0 + pad_x) / 2.0;
    let trace_len = (pad_x - 0.0).abs();
    let trace = centered_cube(
        "rtd_trace",
        trace_len,
        0.3,
        tool_h,
    )
    .translate(trace_center_x, 0.0, pocket_z);
    plate = plate - trace;

    plate
}

/// Fluidic vias on the short (±X) edges for chip stacking.
/// Each via: 1.5mm ID through-hole + 3mm OD × 1mm deep O-ring seat on top.
/// Two vias per short edge (media inlet + media outlet) → four per chip.
fn subtract_fluidic_vias(mut plate: Part) -> Part {
    let chip_top = PMMA_THICKNESS / 2.0;
    let through_h = PMMA_THICKNESS + 1.0;
    let seat_tool_h = FLUIDIC_VIA_SEAT_DEPTH + 0.1;
    let seat_z = chip_top - FLUIDIC_VIA_SEAT_DEPTH / 2.0 + 0.05;

    let half_l = CHIP_LENGTH / 2.0;
    // One via on each short edge, paired near both long edges
    // Four total: (-X,-Y), (-X,+Y), (+X,-Y), (+X,+Y)
    for &sx in &[-1.0_f64, 1.0] {
        for &sy in &[-1.0_f64, 1.0] {
            let via_x = sx * (half_l - FLUIDIC_VIA_INSET_FROM_EDGE);
            let via_y = sy * 25.0; // near but not on the long edge
            let tag = format!("fv_{}{}", if sx < 0.0 { "n" } else { "p" }, if sy < 0.0 { "n" } else { "p" });

            // Through-hole
            let hole = centered_cylinder(
                &format!("{tag}_hole"),
                FLUIDIC_VIA_ID / 2.0,
                through_h,
                32,
            )
            .translate(via_x, via_y, 0.0);
            plate = plate - hole;

            // O-ring seat (counterbore on top face)
            let seat = centered_cylinder(
                &format!("{tag}_seat"),
                FLUIDIC_VIA_OD_SEAT / 2.0,
                seat_tool_h,
                32,
            )
            .translate(via_x, via_y, seat_z);
            plate = plate - seat;
        }
    }

    plate
}

/// Build the glass substrate bottom plate with matching corner radii.
fn glass_bottom() -> Part {
    let mut glass = centered_cube("glass_bottom", CHIP_LENGTH, CHIP_WIDTH, GLASS_THICKNESS);
    let half_l = CHIP_LENGTH / 2.0;
    let half_w = CHIP_WIDTH / 2.0;
    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let si = (sx as i32, sy as i32);
        let gsq = centered_cube(
            &format!("glass_cr_sq_{}_{}", si.0, si.1),
            CORNER_R,
            CORNER_R,
            GLASS_THICKNESS + 0.1,
        )
        .translate(
            sx * (half_l - CORNER_R / 2.0),
            sy * (half_w - CORNER_R / 2.0),
            0.0,
        );
        let gcyl = centered_cylinder(
            &format!("glass_cr_cyl_{}_{}", si.0, si.1),
            CORNER_R,
            GLASS_THICKNESS + 0.1,
            32,
        )
        .translate(sx * (half_l - CORNER_R), sy * (half_w - CORNER_R), 0.0);
        glass = glass - (gsq - gcyl);
    }
    glass
}

/// Top assembly: PMMA plate with all features subtracted.
fn assembly() -> Part {
    let plate = outer_body();
    let plate = subtract_wells(plate);
    let plate = subtract_vias(plate);
    let plate = subtract_channels_and_chambers(plate);
    let plate = subtract_teer_electrodes(plate);
    let plate = subtract_thermistor_trace(plate);
    subtract_fluidic_vias(plate)
}

fn main() {
    println!("== Rev D Microfluidic Chip — Integrated Sensors + Unidirectional Flow ==");
    println!();

    let pmma = assembly();
    let glass = glass_bottom();

    let pmma_stl = "output/microfluidic_chip_revd_pmma.stl";
    let glass_stl = "output/microfluidic_chip_revd_glass.stl";

    pmma.write_stl(pmma_stl).unwrap();
    glass.write_stl(glass_stl).unwrap();

    println!("Exported:");
    println!("  {pmma_stl}");
    println!("  {glass_stl}");

    // STEP exports (no-op if stltostp isn't installed)
    laminarforge_cad::stl_to_step(pmma_stl);
    laminarforge_cad::stl_to_step(glass_stl);

    println!();
    println!("  PART 1: PMMA Plate");
    println!("    Overall:          {CHIP_LENGTH:.2} × {CHIP_WIDTH:.2} × {PMMA_THICKNESS:.2}mm");
    println!("    Total stack:      {TOTAL_HEIGHT:.2}mm (ANSI/SLAS 2-2004 compatible)");
    println!("    Corner radius:    {CORNER_R:.2}mm (ANSI/SLAS 1-2004)");
    println!();
    println!("  CHAMBERS ({NUM_CHAMBERS}, {GRID_ROWS}×{GRID_COLS} grid):");
    for (i, label) in CHAMBER_LABELS.iter().enumerate() {
        let col = i % GRID_COLS;
        let row = i / GRID_COLS;
        let cx = CHAMBER_XS[col];
        let cy = CHAMBER_YS[row];
        println!("    Ch{:>2}: ({cx:+7.2}, {cy:+7.2})  {label}", i + 1);
    }
    println!("    Chamber dims:     {CHAMBER_WIDTH:.1} × {CHAMBER_LENGTH:.1} × {CHAMBER_DEPTH:.2}mm");
    println!();
    println!("  RESERVOIRS (5 inlet + 5 outlet per chamber = {} total):", NUM_CHAMBERS * WELLS_PER_SIDE * 2);
    println!("    Well diameter:    {WELL_DIAMETER:.1}mm");
    println!("    Well depth:       {WELL_DEPTH:.1}mm");
    println!("    Well volume:      {WELL_VOLUME_UL:.1}µL each");
    println!("    Well pitch:       {WELL_PITCH:.1}mm (96-well grid)");
    println!();
    println!("  CHANNELS (asymmetric-port for unidirectional flow):");
    println!("    Width:            {CHANNEL_WIDTH:.2}mm");
    println!("    Shallow depth:    {CHANNEL_DEPTH_SHALLOW:.2}mm (low port)");
    println!("    Deep depth:       {CHANNEL_DEPTH_DEEP:.2}mm (high port)");
    println!();
    println!("  INTEGRATED SENSORS:");
    println!("    TEER electrodes:  4 per chamber (2 top lid + 2 glass), 4-electrode Kelvin");
    println!("    TEER pad:         {TEER_PAD_W:.1} × {TEER_PAD_L:.1}mm, 100nm Au over 10nm Ti");
    println!("    Pt RTD:           {RTD_SERPENTINE_W:.1} × {RTD_SERPENTINE_L:.1}mm serpentine (Pt1000 target)");
    println!("    Edge contact pads: {EDGE_PAD_W:.1} × {EDGE_PAD_L:.1}mm, pogo-pin access");
    println!();
    println!("  FLUIDIC VIAS (chip-to-chip stacking):");
    println!("    Count:            4 (2 per short edge)");
    println!("    ID / O-ring seat: {FLUIDIC_VIA_ID:.1}mm / {FLUIDIC_VIA_OD_SEAT:.1}mm × {FLUIDIC_VIA_SEAT_DEPTH:.1}mm deep");
    println!();
    println!("  PART 2: Glass Bottom");
    println!("    Material:         150µm borosilicate (Borofloat 33)");
    println!("    Surface treatment: sputter Ti/Au + Pt film before bonding");
    println!("    Bonding:          ARcare 90106NB PSA tape (140µm)");
    println!();
    println!("  BUILD ORDER:");
    println!("    1. Sputter Ti/Au electrodes + traces + Pt RTD on glass substrate");
    println!("    2. CNC mill PMMA top face (wells, vias, mount holes, align pins)");
    println!("    3. Flip + mill bottom face (channels, chambers, sensor pockets, label)");
    println!("    4. O2 plasma activate, align on dowel pins, PSA bond glass");
    println!("    5. Install O-rings in fluidic via seats; plug unused vias");
}
