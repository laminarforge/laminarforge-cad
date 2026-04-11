use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Print Sample: Single Chamber of Rev C Microfluidic Chip (3× Scale) ───
//
// Scaled-up single-chamber model for FDM 3D printing verification.
// Shows one complete flow path: inlet well → via → channel → chamber → channel → via → outlet well
//
// 3× XY scale, capped block height at 15mm.
// Channels: 0.5mm → 1.5mm wide, 0.2mm → 0.6mm deep (3 layers at 0.2mm LH)
// Designed for fast print (~1-2 hrs) on Bambu A1.

const SCALE: f64 = 3.0;

/// Scale XY dimensions
fn s(v: f64) -> f64 { v * SCALE }

fn main() {
    // ── Original dimensions (from lib.rs) ──
    let well_dia = REVC_WELL_DIAMETER;   // 4.0mm
    let via_dia = REVC_VIA_DIAMETER;     // 1.0mm
    let chamber_w = REVC_CHAMBER_WIDTH;  // 3.0mm
    let chamber_l = REVC_CHAMBER_LENGTH; // 7.0mm
    let chamber_d = REVC_CHAMBER_DEPTH;  // 0.2mm
    let ch_w = REVC_CHANNEL_WIDTH;       // 0.5mm
    let ch_d = REVC_CHANNEL_DEPTH;       // 0.2mm

    // ── Capped block height ──
    // 15mm total. Wells: 8mm deep, vias: 4mm, leaves 3mm floor above channels.
    let block_h = 15.0;
    let well_depth_print = 8.0;
    let via_len_print = 4.0;

    // ── Single chamber (row 0, column 0) ──
    let cx = REVC_COL_XS_CENTERED[0];       // -40.50
    let cy = REVC_CHAMBER_CENTER_YS[0];     // -27.00
    let iy = REVC_INLET_YS[0];              // -31.50
    let oy = REVC_OUTLET_YS[0];             // -22.50

    // Section block size — just enough to frame one chamber with margins
    let margin = 4.0;
    let section_w = s(well_dia + margin * 2.0);  // width around one column
    let section_h = s((oy - iy) + well_dia + margin * 2.0); // height from inlet to outlet + margins

    // ── Build block ──
    let chip_top = block_h / 2.0;
    let chip_bot = -block_h / 2.0;

    // Section centered at origin — all features relative to (cx, midpoint of inlet/outlet)
    let section_cy_origin = (iy + oy) / 2.0;

    let mut block = centered_cube("sample_block", section_w, section_h, block_h);

    // ── Cutting tools ──
    // ── Flipped orientation: channels on TOP face, wells open from BOTTOM ──
    // This puts the interesting channel detail visible on the top surface.
    let well_tool_h = well_depth_print + 0.5;
    let well_z = chip_bot + well_depth_print / 2.0 - 0.25;

    let via_tool_h = via_len_print + 0.5;
    let via_z = chip_bot + well_depth_print + via_len_print / 2.0;

    let chamber_tool_h = s(chamber_d) + 0.5;
    let chamber_z = chip_top - s(chamber_d) / 2.0 + 0.25;

    let channel_tool_h = s(ch_d) + 0.5;
    let channel_z = chip_top - s(ch_d) / 2.0 + 0.25;

    // Block-local scaled coordinates (everything centered at origin)
    let ly = s(cy - section_cy_origin);
    let liy = s(iy - section_cy_origin);
    let loy = s(oy - section_cy_origin);

    // ── Inlet well ──
    let inlet_well = centered_cylinder(
        "iw_0", s(well_dia) / 2.0, well_tool_h, 32,
    ).translate(0.0, liy, well_z);
    block = block - inlet_well;

    // ── Outlet well ──
    let outlet_well = centered_cylinder(
        "ow_0", s(well_dia) / 2.0, well_tool_h, 32,
    ).translate(0.0, loy, well_z);
    block = block - outlet_well;

    // ── Inlet via ──
    let inlet_via = centered_cylinder(
        "iv_0", s(via_dia) / 2.0, via_tool_h, 24,
    ).translate(0.0, liy, via_z);
    block = block - inlet_via;

    // ── Outlet via ──
    let outlet_via = centered_cylinder(
        "ov_0", s(via_dia) / 2.0, via_tool_h, 24,
    ).translate(0.0, loy, via_z);
    block = block - outlet_via;

    // ── Chamber (bottom face) ──
    let chamber = centered_cube(
        "ch_0", s(chamber_w), s(chamber_l), chamber_tool_h,
    ).translate(0.0, ly, chamber_z);
    block = block - chamber;

    // ── Inlet channel (via → chamber) ──
    let ch_in_y1 = iy - section_cy_origin;
    let ch_in_y2 = cy - chamber_l / 2.0 - section_cy_origin;
    let ch_in_center = s((ch_in_y1 + ch_in_y2) / 2.0);
    let ch_in_len = s((ch_in_y2 - ch_in_y1).abs());
    let inlet_ch = centered_cube(
        "ic_0", s(ch_w), ch_in_len, channel_tool_h,
    ).translate(0.0, ch_in_center, channel_z);
    block = block - inlet_ch;

    // ── Outlet channel (chamber → via) ──
    let ch_out_y1 = cy + chamber_l / 2.0 - section_cy_origin;
    let ch_out_y2 = oy - section_cy_origin;
    let ch_out_center = s((ch_out_y1 + ch_out_y2) / 2.0);
    let ch_out_len = s((ch_out_y2 - ch_out_y1).abs());
    let outlet_ch = centered_cube(
        "oc_0", s(ch_w), ch_out_len, channel_tool_h,
    ).translate(0.0, ch_out_center, channel_z);
    block = block - outlet_ch;

    // ── Export ──
    block
        .write_stl("output/microfluidic_chip_revc_print_sample.stl")
        .unwrap();

    println!("Exported: output/microfluidic_chip_revc_print_sample.stl");
    println!();
    println!("== Rev C Print Sample (3× XY, 15mm Height, 1 Chamber) ==");
    println!("  Block:    {:.1}mm x {:.1}mm x {:.1}mm", section_w, section_h, block_h);
    println!("  Chambers: 1 (single flow path)");
    println!();
    println!("  Scaled features:");
    println!("    Wells:    {:.1}mm Ø × {:.1}mm deep", s(well_dia), well_depth_print);
    println!("    Vias:     {:.1}mm Ø × {:.1}mm long", s(via_dia), via_len_print);
    println!("    Channels: {:.1}mm wide × {:.1}mm deep", s(ch_w), s(ch_d));
    println!("    Chambers: {:.1}mm × {:.1}mm × {:.1}mm", s(chamber_w), s(chamber_l), s(chamber_d));
}
