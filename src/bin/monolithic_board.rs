use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, export::DxfDocument, Material};

// ─── Monolithic 16-Chamber Microfluidic Board ───
//
// Single PMMA plate replaces BOTH the 16-chamber chip AND the FCB.
// All features milled into the top face of one 3mm plate:
//   - 16 culture chambers (4x4 grid)
//   - Inlet/outlet channels per chamber
//   - Shared bus channel + input feed channels
//   - Distribution channels (bus -> valve -> chamber inlet)
//   - Outlet collection channels + collector manifold
//   - Through-holes ONLY for external connections (input, output, valve ports)
//   - Alignment pin holes for cover plate registration
//   - Label area
//
// Cover plate: flat 180mm x 128mm x 1mm, bonded via chloroform vapor-assisted thermal bonding.
// (12mm shorter in Y to expose gold electrode connector strip at north edge)
//
// Integrated sensing: gold TEER electrodes (sputter-coated) + sensor PCB (impedance + fluorescence)
//
// MONOLITHIC DESIGN -- No separate chip, no O-rings, no clamping.
// Through-hole count: 38 (was 76 in the old two-piece design).
// Blind holes: 4 alignment + 4 sensor PCB mount = 8.
//
// Coordinate system:
//   Board center at origin
//   X: left (-90) to right (+90)    -- 180mm
//   Y: bottom (-70) to top (+70)    -- 140mm
//   Z: bottom (-1.5) to top (+1.5)  -- 3mm
//
// Chamber grid (same as old chip, centered on board):
//   Column X centers: -37.50, -12.50, +12.50, +37.50
//   Row Y centers:    +27.00, +9.00, -9.00, -27.00
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
    // ══════════════════════════════════════════════════════════════
    // CONSTANTS
    // ══════════════════════════════════════════════════════════════

    // Board dimensions (from FCB16 constants -- same footprint)
    let length = FCB16_LENGTH; // 180mm
    let width = FCB16_WIDTH; // 140mm
    let thickness = 3.0_f64; // single 3mm plate (not two plates)
    let cover_thickness = 1.0_f64; // cover plate

    // Chamber dimensions (from CNC16 constants)
    let num_cols = CNC16_GRID_COLS; // 4
    let num_rows = CNC16_GRID_ROWS; // 4
    let chamber_w = CNC16_CHAMBER_WIDTH; // 3.0mm (X)
    let chamber_l = CNC16_CHAMBER_LENGTH; // 10.0mm (Y)
    let chamber_d = CNC16_CHAMBER_DEPTH; // 0.2mm

    // Chamber channel dimensions
    let ch_w = CNC16_CHANNEL_WIDTH; // 0.5mm
    let ch_d = CNC16_CHANNEL_DEPTH; // 0.2mm
                                    // OVERRIDE: 3mm instead of CNC16_CHANNEL_LENGTH (5mm) to prevent overlap
                                    // between adjacent rows. Row spacing = 18mm, feature span = 3+10+3 = 16mm,
                                    // leaving 2mm solid wall between rows. Literature confirms 2mm inter-channel
                                    // spacing is standard for CNC-milled PMMA microfluidics.
    let ch_len = 3.0_f64;

    // Grid spacing (from CNC16 constants)
    let col_spacing = CNC16_COL_SPACING; // 25.0mm
    let row_spacing = CNC16_ROW_SPACING; // 18.0mm

    // Bus channel (from FCB16 constants)
    let bus_w = FCB16_BUS_WIDTH; // 0.5mm
    let bus_d = FCB16_BUS_DEPTH; // 0.3mm

    // Distribution channels (from FCB16 constants)
    let dist_w = FCB16_DIST_WIDTH; // 0.5mm
    let dist_d = FCB16_DIST_DEPTH; // 0.2mm

    // Outlet collection channels (from FCB16 constants)
    let out_w = FCB16_OUTLET_WIDTH; // 0.5mm
    let out_d = FCB16_OUTLET_DEPTH; // 0.2mm

    // Port diameters
    let port_dia = FCB16_PORT_DIAMETER; // 1.5mm
    let valve_dia = FCB16_VALVE_PORT_DIAMETER; // 1.5mm
    let input_dia = FCB16_INPUT_PORT_DIAMETER; // 1.5mm

    // Valve pair geometry
    let valve_pair_offset = FCB16_VALVE_PAIR_OFFSET; // 2.0mm

    // Alignment pin holes
    let align_dia = CNC16_ALIGN_DIAMETER; // 1.0mm
    let align_depth = CNC16_ALIGN_DEPTH; // 0.5mm
    let align_inset = 6.0_f64; // 6mm inset for the larger 180x140 board

    // Label area
    let label_l = CNC16_LABEL_LENGTH; // 20mm
    let label_w = CNC16_LABEL_WIDTH; // 5mm
    let label_d = CNC16_LABEL_DEPTH; // 0.1mm

    // Electrode connector strip (exposed north edge, not covered by cover plate)
    let connector_strip = 12.0_f64; // mm, exposed for gold electrode FFC pads
    let cover_width = width - connector_strip; // 128mm
    let cover_y_offset = -connector_strip / 2.0; // -6.0mm (cover plate centered lower)

    // Sensor PCB mounting holes (blind holes on main board, through-holes on cover plate)
    let sensor_mount_dia = 2.2_f64; // M2 clearance
    let sensor_mount_depth = 1.0_f64; // blind holes
    let sensor_mount_positions: [(f64, f64); 4] =
        [(-50.0, -35.0), (-50.0, 35.0), (50.0, -35.0), (50.0, 35.0)];

    // ══════════════════════════════════════════════════════════════
    // DERIVED POSITIONS
    // ══════════════════════════════════════════════════════════════

    let board_top = thickness / 2.0; // +1.5mm

    // Grid center positions (same as old chip + FCB)
    let col_xs = fcb16_col_xs(); // [-37.5, -12.5, +12.5, +37.5]
    let row_ys = fcb16_row_ys(); // [+27.0, +9.0, -9.0, -27.0]

    // Input port positions (left edge, centered vertically, 15mm spacing)
    let input_x: f64 = -85.0; // left edge + 5mm inset
    let num_inputs = FCB16_NUM_INPUTS; // 4
    let input_spacing = FCB16_INPUT_SPACING; // 15mm
    let input_ys: Vec<f64> = {
        let total_span = (num_inputs as f64 - 1.0) * input_spacing; // 45mm
        let first_y = -total_span / 2.0; // -22.5
        (0..num_inputs)
            .map(|i| first_y + i as f64 * input_spacing)
            .collect()
    };
    let input_labels = ["Media", "AAV", "PBS", "Trigger"];

    // Output port positions (right edge, centered, 15mm spacing)
    let output_x: f64 = 85.0; // right edge - 5mm inset
    let num_outputs = FCB16_NUM_OUTPUTS; // 2
    let output_spacing = FCB16_OUTPUT_SPACING; // 15mm
    let output_ys: Vec<f64> = {
        let total_span = (num_outputs as f64 - 1.0) * output_spacing; // 15mm
        let first_y = -total_span / 2.0; // -7.5
        (0..num_outputs)
            .map(|i| first_y + i as f64 * output_spacing)
            .collect()
    };
    let output_labels = ["Waste", "Sample"];

    // Valve port Y position (bottom edge + 5mm inset)
    let valve_y: f64 = -65.0;

    // Bus channel Y position -- below all chamber inlet channel endpoints
    let bus_y: f64 = -47.0;

    // Collector manifold Y position -- above all chamber outlet channel endpoints
    let collector_y: f64 = 47.0;

    // ── Tool heights for boolean cutting ──
    // Bus channel: 0.3mm deep
    let bus_tool_h = bus_d + 0.2;
    let bus_z = board_top - bus_d / 2.0 + 0.1;

    // Distribution channels: 0.2mm deep
    let dist_tool_h = dist_d + 0.2;
    let dist_z = board_top - dist_d / 2.0 + 0.1;

    // Chamber + inlet/outlet channels: 0.2mm deep
    let pocket_tool_h = ch_d + 0.2; // 0.4mm
    let pocket_z = board_top - ch_d / 2.0 + 0.1;

    // Outlet collection channels: 0.2mm deep
    let out_tool_h = out_d + 0.2;
    let out_z = board_top - out_d / 2.0 + 0.1;

    // Alignment pin holes: 0.5mm deep blind
    let align_tool_h = align_depth + 0.2;
    let align_z = board_top - align_depth / 2.0 + 0.1;

    // Label area: 0.1mm deep
    let label_tool_h = label_d + 0.2;
    let label_z = board_top - label_d / 2.0 + 0.1;

    // Through-holes exceed plate thickness
    let through_h = thickness + 1.0; // 4mm

    // ══════════════════════════════════════════════════════════════
    // COMPUTE VALVE PORT AND ROUTING POSITIONS
    // ══════════════════════════════════════════════════════════════

    struct ValvePort {
        out_x: f64,
        ret_x: f64,
        y: f64,
        col: usize,
        row: usize,
    }

    struct Route {
        col: usize,
        row: usize,
        col_x: f64,
        _row_y: f64,
        inlet_endpoint_y: f64, // bottom of inlet channel = chamber_bottom - ch_len
        outlet_endpoint_y: f64, // top of outlet channel = chamber_top + ch_len
        valve_out_x: f64,
        valve_ret_x: f64,
        dist_x: f64,         // distribution channel X offset
        outlet_route_x: f64, // outlet routing X offset
    }

    let mut valve_ports: Vec<ValvePort> = Vec::new();
    let mut routes: Vec<Route> = Vec::new();

    for (ci, &cx) in col_xs.iter().enumerate() {
        for (ri, &ry) in row_ys.iter().enumerate() {
            // Valve pair center: same layout as FCB16
            let pair_center_x = cx
                + match ri {
                    0 => 7.5,
                    1 => 2.5,
                    2 => -2.5,
                    3 => -7.5,
                    _ => unreachable!(),
                };

            let out_x = pair_center_x - valve_pair_offset / 2.0; // pair_center - 1.0
            let ret_x = pair_center_x + valve_pair_offset / 2.0; // pair_center + 1.0

            valve_ports.push(ValvePort {
                out_x,
                ret_x,
                y: valve_y,
                col: ci,
                row: ri,
            });

            // Distribution channel X = valve pair center X
            let dist_x = pair_center_x;

            // Outlet routing offsets within column
            let outlet_route_x = cx
                + match ri {
                    0 => -1.5,
                    1 => -0.5,
                    2 => 0.5,
                    3 => 1.5,
                    _ => unreachable!(),
                };

            // Inlet endpoint: bottom of inlet channel (ch_len below chamber bottom edge)
            let inlet_endpoint_y = ry - chamber_l / 2.0 - ch_len; // ry - 5.0 - 3.0 = ry - 8.0
                                                                  // Outlet endpoint: top of outlet channel (ch_len above chamber top edge)
            let outlet_endpoint_y = ry + chamber_l / 2.0 + ch_len; // ry + 5.0 + 3.0 = ry + 8.0

            routes.push(Route {
                col: ci,
                row: ri,
                col_x: cx,
                _row_y: ry,
                inlet_endpoint_y,
                outlet_endpoint_y,
                valve_out_x: out_x,
                valve_ret_x: ret_x,
                dist_x,
                outlet_route_x,
            });
        }
    }

    // ══════════════════════════════════════════════════════════════
    // MAIN BOARD -- 180mm x 140mm x 3mm, all features on top face
    // ══════════════════════════════════════════════════════════════

    let mut board = centered_cube("monolithic_board", length, width, thickness);

    // ── 1. CULTURE CHAMBERS (16, 4x4 grid) ──
    for (ri, &ry) in row_ys.iter().enumerate() {
        for (ci, &cx) in col_xs.iter().enumerate() {
            let idx = ri * num_cols + ci;
            let chamber = centered_cube(
                format!("chamber_{idx}"),
                chamber_w,
                chamber_l,
                pocket_tool_h,
            )
            .translate(cx, ry, pocket_z);
            board = board - chamber;
        }
    }

    // ── 2. CHAMBER INLET CHANNELS (16) ──
    // Run in Y-direction from chamber bottom edge downward for ch_len (3mm)
    for (ri, &ry) in row_ys.iter().enumerate() {
        for (ci, &cx) in col_xs.iter().enumerate() {
            let idx = ri * num_cols + ci;
            let chamber_bottom_y = ry - chamber_l / 2.0;
            let inlet_end_y = chamber_bottom_y - ch_len;
            let inlet_center_y = (chamber_bottom_y + inlet_end_y) / 2.0;

            let inlet_ch = centered_cube(format!("inlet_ch_{idx}"), ch_w, ch_len, pocket_tool_h)
                .translate(cx, inlet_center_y, pocket_z);
            board = board - inlet_ch;
        }
    }

    // ── 3. CHAMBER OUTLET CHANNELS (16) ──
    // Run in Y-direction from chamber top edge upward for ch_len (3mm)
    for (ri, &ry) in row_ys.iter().enumerate() {
        for (ci, &cx) in col_xs.iter().enumerate() {
            let idx = ri * num_cols + ci;
            let chamber_top_y = ry + chamber_l / 2.0;
            let outlet_end_y = chamber_top_y + ch_len;
            let outlet_center_y = (chamber_top_y + outlet_end_y) / 2.0;

            let outlet_ch = centered_cube(format!("outlet_ch_{idx}"), ch_w, ch_len, pocket_tool_h)
                .translate(cx, outlet_center_y, pocket_z);
            board = board - outlet_ch;
        }
    }

    // ── 4. SHARED BUS CHANNEL ──
    // Horizontal at Y = -47, from input_x (-85) to rightmost column (+37.5)
    let bus_x_start = input_x; // -85.0
    let bus_x_end = *col_xs.last().unwrap(); // +37.5
    let bus_length = bus_x_end - bus_x_start;
    let bus_center_x = (bus_x_start + bus_x_end) / 2.0;

    let bus_channel =
        centered_cube("bus", bus_length, bus_w, bus_tool_h).translate(bus_center_x, bus_y, bus_z);
    board = board - bus_channel;

    // ── 5. INPUT FEED CHANNELS (4) ──
    // Vertical channels from each input port down to the bus
    for (i, &iy) in input_ys.iter().enumerate() {
        let feed_len = (iy - bus_y).abs();
        let feed_center_y = (iy + bus_y) / 2.0;
        let feed = centered_cube(format!("input_feed_{i}"), bus_w, feed_len, bus_tool_h).translate(
            input_x,
            feed_center_y,
            bus_z,
        );
        board = board - feed;
    }

    // ── 6. DISTRIBUTION CHANNELS (16, bus -> valve OUT -> [ext valve] -> valve RETURN -> inlet) ──
    for (idx, r) in routes.iter().enumerate() {
        let ch_label = format!("c{}r{}", r.col, r.row);

        // Bus tap: horizontal connector from col_x on bus to dist_x
        let tap_x_start = r.col_x.min(r.dist_x);
        let tap_x_end = r.col_x.max(r.dist_x);
        let tap_len = tap_x_end - tap_x_start;
        if tap_len > 0.01 {
            let tap = centered_cube(
                format!("bus_tap_{ch_label}"),
                tap_len + dist_w,
                bus_w,
                bus_tool_h,
            )
            .translate((tap_x_start + tap_x_end) / 2.0, bus_y, bus_z);
            board = board - tap;
        }

        // Segment A: vertical from bus_y down to valve OUT port at valve_y
        let seg_a_x = r.valve_out_x;
        let seg_a_len = (bus_y - valve_y).abs();
        let seg_a_center_y = (bus_y + valve_y) / 2.0;

        // Horizontal connector from dist_x to valve_out_x at bus_y (if they differ)
        let h_bus_to_valve_len = (r.dist_x - seg_a_x).abs();
        if h_bus_to_valve_len > 0.01 {
            let h_conn = centered_cube(
                format!("dist_h_bus_{ch_label}"),
                h_bus_to_valve_len + dist_w,
                dist_w,
                dist_tool_h,
            )
            .translate((r.dist_x + seg_a_x) / 2.0, bus_y, dist_z);
            board = board - h_conn;
        }

        let seg_a = centered_cube(format!("dist_a_{ch_label}"), dist_w, seg_a_len, dist_tool_h)
            .translate(seg_a_x, seg_a_center_y, dist_z);
        board = board - seg_a;

        // Segment B: vertical from valve RETURN port up to inlet channel endpoint
        // In monolithic design, the distribution channel connects DIRECTLY to the
        // inlet channel endpoint (no through-hole interface).
        let seg_b_x = r.valve_ret_x;
        let seg_b_len = (r.inlet_endpoint_y - valve_y).abs();
        let seg_b_center_y = (valve_y + r.inlet_endpoint_y) / 2.0;

        let seg_b = centered_cube(format!("dist_b_{ch_label}"), dist_w, seg_b_len, dist_tool_h)
            .translate(seg_b_x, seg_b_center_y, dist_z);
        board = board - seg_b;

        // Horizontal connector from valve_ret_x to col_x at inlet_endpoint_y
        // This is the CHANNEL JUNCTION -- distribution channel meets inlet channel
        let h_ret_to_inlet_len = (r.col_x - seg_b_x).abs();
        if h_ret_to_inlet_len > 0.01 {
            let h_conn_b = centered_cube(
                format!("inlet_junc_{ch_label}"),
                h_ret_to_inlet_len + dist_w,
                dist_w,
                dist_tool_h,
            )
            .translate((r.col_x + seg_b_x) / 2.0, r.inlet_endpoint_y, dist_z);
            board = board - h_conn_b;
        }

        // ── 7. OUTLET COLLECTION CHANNELS ──
        // Horizontal connector from col_x to outlet_route_x at outlet_endpoint_y
        let h_out_len = (r.col_x - r.outlet_route_x).abs();
        if h_out_len > 0.01 {
            let h_out = centered_cube(
                format!("outlet_junc_{ch_label}"),
                h_out_len + out_w,
                out_w,
                out_tool_h,
            )
            .translate(
                (r.col_x + r.outlet_route_x) / 2.0,
                r.outlet_endpoint_y,
                out_z,
            );
            board = board - h_out;
        }

        // Vertical from outlet_endpoint_y up to collector_y at outlet_route_x
        let out_vert_len = (collector_y - r.outlet_endpoint_y).abs();
        let out_center_y = (r.outlet_endpoint_y + collector_y) / 2.0;

        let out_ch = centered_cube(
            format!("outlet_{ch_label}"),
            out_w,
            out_vert_len,
            out_tool_h,
        )
        .translate(r.outlet_route_x, out_center_y, out_z);
        board = board - out_ch;

        let _ = idx; // suppress warning
    }

    // ── 8. COLLECTOR MANIFOLD ──
    // Horizontal at Y = +47, from leftmost column (-37.5) to output_x (+85)
    let collector_x_start = *col_xs.first().unwrap(); // -37.5
    let collector_x_end = output_x; // +85.0
    let collector_len = collector_x_end - collector_x_start;
    let collector_center_x = (collector_x_start + collector_x_end) / 2.0;

    let collector_manifold = centered_cube("collector", collector_len, out_w, out_tool_h)
        .translate(collector_center_x, collector_y, out_z);
    board = board - collector_manifold;

    // Output feed channels (collector manifold to output ports on right edge)
    for (i, &oy) in output_ys.iter().enumerate() {
        let out_feed_len = (oy - collector_y).abs();
        if out_feed_len > 0.01 {
            let out_feed_center_y = (collector_y + oy) / 2.0;
            let out_feed =
                centered_cube(format!("output_feed_{i}"), out_w, out_feed_len, out_tool_h)
                    .translate(output_x, out_feed_center_y, out_z);
            board = board - out_feed;
        }
    }

    // ══════════════════════════════════════════════════════════════
    // THROUGH-HOLES (only for external connections -- 38 total)
    // ══════════════════════════════════════════════════════════════

    // ── 9a. Input port through-holes (4) ──
    for (i, &iy) in input_ys.iter().enumerate() {
        let hole = centered_cylinder(format!("input_port_{i}"), input_dia / 2.0, through_h, 32)
            .translate(input_x, iy, 0.0);
        board = board - hole;
    }

    // ── 9b. Output port through-holes (2) ──
    for (i, &oy) in output_ys.iter().enumerate() {
        let hole = centered_cylinder(format!("output_port_{i}"), port_dia / 2.0, through_h, 32)
            .translate(output_x, oy, 0.0);
        board = board - hole;
    }

    // ── 9c. Valve port through-holes (32: 16 OUT + 16 RETURN) ──
    for (idx, vp) in valve_ports.iter().enumerate() {
        let hole_out =
            centered_cylinder(format!("valve_out_{idx}"), valve_dia / 2.0, through_h, 32)
                .translate(vp.out_x, vp.y, 0.0);

        let hole_ret =
            centered_cylinder(format!("valve_ret_{idx}"), valve_dia / 2.0, through_h, 32)
                .translate(vp.ret_x, vp.y, 0.0);

        board = board - hole_out - hole_ret;
    }

    // ── 10. ALIGNMENT PIN HOLES (4 corners, blind 0.5mm) ──
    // Top alignment holes at Y = +52 (inside cover plate area, 6mm from cover top edge at +58)
    // Bottom alignment holes at Y = -64 (6mm from board bottom edge)
    // Y separation = 116mm for accurate cover plate registration
    let align_positions = [
        (-(length / 2.0 - align_inset), -(width / 2.0 - align_inset)), // BL: (-84, -64)
        (
            -(length / 2.0 - align_inset),
            width / 2.0 - connector_strip - align_inset,
        ), // TL: (-84, +52)
        (length / 2.0 - align_inset, -(width / 2.0 - align_inset)),    // BR: (+84, -64)
        (
            length / 2.0 - align_inset,
            width / 2.0 - connector_strip - align_inset,
        ), // TR: (+84, +52)
    ];

    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        let align_hole = centered_cylinder(format!("align_{i}"), align_dia / 2.0, align_tool_h, 24)
            .translate(ax, ay, align_z);
        board = board - align_hole;
    }

    // ── 10b. SENSOR PCB MOUNTING HOLES (4, blind, M2) ──
    let sensor_mount_tool_h = sensor_mount_depth + 0.2;
    let sensor_mount_z = board_top - sensor_mount_depth / 2.0 + 0.1;
    for (i, &(sx, sy)) in sensor_mount_positions.iter().enumerate() {
        let mount_hole = centered_cylinder(
            format!("sensor_mount_{i}"),
            sensor_mount_dia / 2.0,
            sensor_mount_tool_h,
            24,
        )
        .translate(sx, sy, sensor_mount_z);
        board = board - mount_hole;
    }

    // ── 11. LABEL AREA ──
    // Right side of board, Y = 0 (centered), 20mm x 5mm x 0.1mm
    let label_x = length / 2.0 - align_inset - label_l / 2.0 - 2.0; // offset from right edge
    let label_y = 0.0;
    let label_pocket = centered_cube("label_area", label_l, label_w, label_tool_h)
        .translate(label_x, label_y, label_z);
    board = board - label_pocket;

    // ══════════════════════════════════════════════════════════════
    // COVER PLATE -- 180mm x 128mm x 1mm (12mm shorter for connector strip)
    // ══════════════════════════════════════════════════════════════

    // Cover plate: 180mm x 128mm x 1mm, offset south to expose 12mm connector strip at top
    let mut cover_plate = centered_cube("cover_plate", length, cover_width, cover_thickness)
        .translate(0.0, cover_y_offset, 0.0);

    // Cover plate through-holes: alignment pins + sensor PCB mounting
    let cover_through_h = cover_thickness + 0.5;
    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("cover_align_{i}"),
            align_dia / 2.0,
            cover_through_h,
            24,
        )
        .translate(ax, ay, 0.0);
        cover_plate = cover_plate - hole;
    }
    for (i, &(sx, sy)) in sensor_mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("cover_sensor_mount_{i}"),
            sensor_mount_dia / 2.0,
            cover_through_h,
            24,
        )
        .translate(sx, sy, 0.0);
        cover_plate = cover_plate - hole;
    }

    // ══════════════════════════════════════════════════════════════
    // EXPORT STL FILES
    // ══════════════════════════════════════════════════════════════

    board.write_stl("output/monolithic_board_16ch.stl").unwrap();
    cover_plate
        .write_stl("output/monolithic_board_16ch_cover.stl")
        .unwrap();

    // ══════════════════════════════════════════════════════════════
    // EXPORT DXF (2D TOP-FACE PROFILE FOR CNC)
    // ══════════════════════════════════════════════════════════════
    // DXF shows all features on the milled surface as 2D outlines.
    // CNC shops use this for toolpath verification alongside the 3D model.

    let mut dxf = DxfDocument::new();

    // Board outline
    dxf.add_rectangle(length, width, 0.0, 0.0);

    // 16 chambers
    for &cx in &col_xs {
        for &ry in &row_ys {
            dxf.add_rectangle(chamber_w, chamber_l, cx, ry);
        }
    }

    // Inlet + outlet channels per chamber
    for r in &routes {
        let cx = r.col_x;
        let ry = r._row_y;
        // Inlet channel (below chamber)
        dxf.add_rectangle(ch_w, ch_len, cx, ry - chamber_l / 2.0 - ch_len / 2.0);
        // Outlet channel (above chamber)
        dxf.add_rectangle(ch_w, ch_len, cx, ry + chamber_l / 2.0 + ch_len / 2.0);
    }

    // Bus channel (horizontal line across the board)
    let bus_half_len = (col_xs[num_cols - 1] - col_xs[0]) / 2.0 + col_spacing / 2.0;
    dxf.add_rectangle(bus_half_len * 2.0, bus_w, 0.0, bus_y);

    // Input feed channels (input ports to bus)
    for &iy in &input_ys {
        dxf.add_rectangle(
            bus_w,
            (iy - bus_y).abs(),
            input_x + (0.0 - input_x) / 2.0,
            (iy + bus_y) / 2.0,
        );
    }

    // Distribution channels (bus to chamber inlets, vertical segments)
    for r in &routes {
        let dist_top = r.inlet_endpoint_y;
        let dist_bot = bus_y;
        dxf.add_rectangle(
            dist_w,
            (dist_top - dist_bot).abs(),
            r.dist_x,
            (dist_top + dist_bot) / 2.0,
        );
    }

    // Outlet collection channels (chamber outlets to collector)
    for r in &routes {
        let out_bot = r.outlet_endpoint_y;
        let out_top = collector_y;
        dxf.add_rectangle(
            out_w,
            (out_top - out_bot).abs(),
            r.outlet_route_x,
            (out_top + out_bot) / 2.0,
        );
    }

    // Collector manifold (horizontal)
    let collector_len = (col_xs[num_cols - 1] - col_xs[0]) + col_spacing;
    dxf.add_rectangle(collector_len, out_w, 0.0, collector_y);

    // Input ports (through-holes, circles)
    for &iy in &input_ys {
        dxf.add_circle(input_x, iy, input_dia / 2.0);
    }

    // Output ports (through-holes, circles)
    for &oy in &output_ys {
        dxf.add_circle(output_x, oy, port_dia / 2.0);
    }

    // Valve ports (through-holes, circles)
    for vp in &valve_ports {
        dxf.add_circle(vp.out_x, vp.y, valve_dia / 2.0);
        dxf.add_circle(vp.ret_x, vp.y, valve_dia / 2.0);
    }

    // Alignment pin holes (circles)
    for &(ax, ay) in &align_positions {
        dxf.add_circle(ax, ay, align_dia / 2.0);
    }

    // Sensor PCB mounting holes (circles)
    for &(sx, sy) in &sensor_mount_positions {
        dxf.add_circle(sx, sy, sensor_mount_dia / 2.0);
    }

    // Label area
    dxf.add_rectangle(label_l, label_w, label_x, label_y);

    dxf.export("output/monolithic_board_16ch_top.dxf").unwrap();

    // ══════════════════════════════════════════════════════════════
    // EXPORT GLB (3D VISUALIZATION WITH MATERIALS)
    // ══════════════════════════════════════════════════════════════

    let pmma_clear = Material {
        name: "pmma_clear".to_string(),
        description: Some("Cast PMMA (acrylic), optically clear".to_string()),
        color: [0.92, 0.95, 0.98],
        metallic: 0.0,
        roughness: 0.1,
        density: 1180.0,
        friction: 0.4,
    };

    vcad::export::export_glb(&board, &pmma_clear, "output/monolithic_board_16ch.glb").unwrap();
    vcad::export::export_glb(
        &cover_plate,
        &pmma_clear,
        "output/monolithic_board_16ch_cover.glb",
    )
    .unwrap();

    // ══════════════════════════════════════════════════════════════
    // EXPORT STEP (VIA stltostp -- STL-TO-STEP BREP CONVERSION)
    // ══════════════════════════════════════════════════════════════
    // stltostp is a standalone C++ tool (no OpenCASCADE/FreeCAD dependency)
    // that converts STL triangles directly to STEP BRep faces.

    let stltostp = std::path::Path::new(env!("HOME")).join(".local/bin/stltostp");
    if stltostp.exists() {
        for (stl, step) in [
            (
                "output/monolithic_board_16ch.stl",
                "output/monolithic_board_16ch.step",
            ),
            (
                "output/monolithic_board_16ch_cover.stl",
                "output/monolithic_board_16ch_cover.step",
            ),
        ] {
            let status = std::process::Command::new(&stltostp)
                .arg(stl)
                .arg(step)
                .status()
                .expect("failed to run stltostp");
            if !status.success() {
                eprintln!("WARNING: stltostp failed for {stl}");
            }
        }
    } else {
        eprintln!("NOTE: stltostp not found at ~/.local/bin/stltostp — skipping STEP export");
        eprintln!("      Install: git clone https://github.com/slugdev/stltostp && cd stltostp && mkdir build && cd build && cmake .. && make");
    }

    // ══════════════════════════════════════════════════════════════
    // TECHNICAL DRAWING
    // ══════════════════════════════════════════════════════════════

    let mut d = String::new();

    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  TECHNICAL DRAWING -- MONOLITHIC 16-CHAMBER MICROFLUIDIC BOARD\n");
    d.push_str("  AAV Selectivity Screening Platform\n");
    d.push_str("  LaminarForge\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  ***  MONOLITHIC DESIGN -- No separate chip, no O-rings, no clamping  ***\n");
    d.push('\n');
    d.push_str("  Drawing Rev:    A (initial release)\n");
    d.push_str("  Date:           2026-02-28\n");
    d.push_str("  Units:          millimeters (mm)\n");
    d.push_str("  Origin:         Geometric center of board\n");
    d.push_str("  Coordinate:     X = long axis, Y = short axis, Z = thickness axis\n");
    d.push_str("  Tolerances:     See Section 9\n");
    d.push('\n');

    // Section 1: Overall Dimensions
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  1. OVERALL DIMENSIONS\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  PART 1 -- MAIN BOARD (all features milled into top face)\n");
    d.push_str(&format!("    Length (X):     {:.0} mm\n", length));
    d.push_str(&format!("    Width  (Y):     {:.0} mm\n", width));
    d.push_str(&format!("    Thickness (Z):  {:.1} mm\n", thickness));
    d.push_str("    Material:       Cast PMMA (acrylic), optically clear\n");
    d.push_str("    Z range:        -1.5 to +1.5 mm\n");
    d.push_str(
        "    Features:       Chambers, channels, bus, manifold, through-holes -- all on top face\n",
    );
    d.push('\n');
    d.push_str("  PART 2 -- COVER PLATE\n");
    d.push_str(&format!("    Length (X):     {:.0} mm\n", length));
    d.push_str(&format!(
        "    Width  (Y):     {:.0} mm (board is {:.0}mm; {:.0}mm connector strip exposed at top)\n",
        cover_width, width, connector_strip
    ));
    d.push_str(&format!("    Thickness (Z):  {:.1} mm\n", cover_thickness));
    d.push_str("    Material:       Cast PMMA (acrylic), optically clear\n");
    d.push_str("    Y offset:       Bottom-aligned with board (Y=-70), top edge at Y=+58\n");
    d.push_str(
        "    Features:       4 alignment pin through-holes + 4 sensor PCB mounting through-holes\n",
    );
    d.push('\n');
    d.push_str("  ASSEMBLY:\n");
    d.push_str("    1. CNC mill main board (all features on top face)\n");
    d.push_str(
        "    2. Gold electrode deposition (sputter coat with shadow mask: 10nm Ti + 100nm Au)\n",
    );
    d.push_str(
        "    3. Bond cover plate via chloroform vapor-assisted thermal bonding (30s exposure)\n",
    );
    d.push_str("       Simultaneously smooths milling roughness (~153nm -> ~39nm Ra) and bonds\n");
    d.push_str(
        "       without collapsing the wide shallow chambers (3mm W x 0.2mm D, aspect 15:1).\n",
    );
    d.push_str("       DO NOT use liquid Weld-On 3 -- risk of capillary wicking into chambers.\n");
    d.push_str(
        "       Cover plate is 12mm shorter (Y) to expose electrode connector pads at top edge.\n",
    );
    d.push_str(
        "    4. Mount sensor PCB on top of cover plate (M2 nylon screws, 4 mounting points)\n",
    );
    d.push_str("    5. Connect sensor PCB to electrode pads via FFC (flat flex cable)\n");
    d.push_str("    Alignment via 4 pin holes (2 bottom corners + 2 near cover plate top edge).\n");
    d.push_str(&format!(
        "    Total assembled thickness: {:.1} mm (board + cover, excludes sensor PCB)\n",
        thickness + cover_thickness
    ));
    d.push('\n');
    d.push_str("  KEY DIFFERENCE FROM OLD DESIGN:\n");
    d.push_str("    This monolithic board replaces both the 16-chamber chip (127.76x85.48x3mm)\n");
    d.push_str("    and the FCB (180x140x6mm, two plates). Chambers, inlet/outlet channels,\n");
    d.push_str("    bus, distribution, valves, and collection are all on ONE plate.\n");
    d.push_str("    Eliminated: O-ring grooves, chip interface through-holes, mounting holes,\n");
    d.push_str("    clamp plate, separate chip. Through-holes reduced from 76 to 38.\n");
    d.push('\n');

    // Section 2: Chamber Layout
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  2. CHAMBER LAYOUT -- 4x4 GRID\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  Grid:           4 columns (X) x 4 rows (Y)\n");
    d.push_str(&format!(
        "  Column spacing: {:.2} mm center-to-center\n",
        col_spacing
    ));
    d.push_str(&format!(
        "  Row spacing:    {:.2} mm center-to-center\n",
        row_spacing
    ));
    d.push('\n');
    d.push_str("  Column X centers:  -37.50, -12.50, +12.50, +37.50 mm\n");
    d.push_str("  Row Y centers:     +27.00, +9.00, -9.00, -27.00 mm\n");
    d.push('\n');
    d.push_str("  Chamber dimensions (each identical):\n");
    d.push_str(&format!("    Width  (X):     {:.2} mm\n", chamber_w));
    d.push_str(&format!("    Length (Y):    {:.2} mm\n", chamber_l));
    d.push_str(&format!(
        "    Depth  (Z):     {:.2} mm (200 um) -- milled pocket from top face\n",
        chamber_d
    ));
    d.push_str("    Corner radius:  0.50 mm minimum (natural end mill radius)\n");
    d.push('\n');
    d.push_str("  Chamber center coordinates:\n");
    d.push('\n');
    d.push_str("    Ch  | Row | Col |   X (mm)  |   Y (mm)\n");
    d.push_str("    ----+-----+-----+-----------+-----------\n");
    for (ri, &row_y) in row_ys.iter().enumerate().take(num_rows) {
        for (ci, &col_x) in col_xs.iter().enumerate().take(num_cols) {
            let idx = ri * num_cols + ci;
            d.push_str(&format!(
                "    {:2}  |  {}  |  {}  |  {:+7.2}  |  {:+7.2}\n",
                idx + 1,
                ri,
                ci,
                col_x,
                row_y
            ));
        }
    }
    d.push('\n');

    // Section 3: Chamber Allocation Table
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  3. CHAMBER ALLOCATION TABLE\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("    Ch  | Row | Col | Cell Type / Purpose\n");
    d.push_str("    ----+-----+-----+----------------------------------------------\n");
    for (i, label) in CHAMBER_LABELS.iter().enumerate() {
        let ri = i / num_cols;
        let ci = i % num_cols;
        d.push_str(&format!(
            "    {:2}  |  {}  |  {}  | {}\n",
            i + 1,
            ri,
            ci,
            label
        ));
    }
    d.push('\n');
    d.push_str("  Purpose: Expose all 16 chambers simultaneously to the same AAV library\n");
    d.push_str("  to assess which serotype variants preferentially transduce each cell type.\n");
    d.push_str(
        "  DRG sensory neurons are the primary target; all others are off-target tissues.\n",
    );
    d.push('\n');

    // Section 4: Channel Dimensions
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  4. CHANNEL DIMENSIONS\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  All channels milled into top face of main board only.\n");
    d.push_str("  Sealed by bonding cover plate on top.\n");
    d.push('\n');
    d.push_str("  Channel Type               Width (mm)   Depth (mm)   Notes\n");
    d.push_str(
        "  -------------------------  ----------   ----------   -------------------------\n",
    );
    d.push_str(&format!(
        "  Bus channel                  {:.1}          {:.1}         Shared feed, 500x300 um\n",
        bus_w, bus_d
    ));
    d.push_str(&format!(
        "  Input feed channels          {:.1}          {:.1}         Vertical, input port to bus\n",
        bus_w, bus_d
    ));
    d.push_str(&format!(
        "  Distribution channels        {:.1}          {:.1}         Bus to valve to inlet\n",
        dist_w, dist_d
    ));
    d.push_str(&format!(
        "  Chamber inlet channels       {:.1}          {:.1}         {:.0}mm, Y-direction\n",
        ch_w, ch_d, ch_len
    ));
    d.push_str(&format!(
        "  Culture chambers             {:.1}(W)       {:.1}         3mm x 10mm pockets\n",
        chamber_w, chamber_d
    ));
    d.push_str(&format!(
        "  Chamber outlet channels      {:.1}          {:.1}         {:.0}mm, Y-direction\n",
        ch_w, ch_d, ch_len
    ));
    d.push_str(&format!("  Outlet collection channels   {:.1}          {:.1}         Vertical, outlet to collector\n", out_w, out_d));
    d.push_str(&format!(
        "  Collector manifold           {:.1}          {:.1}         Horizontal, to output ports\n",
        out_w, out_d
    ));
    d.push_str(&format!(
        "  Output feed channels         {:.1}          {:.1}         Collector to output port\n",
        out_w, out_d
    ));
    d.push('\n');
    d.push_str("  Inlet channels (16 total):\n");
    d.push_str(&format!(
        "    Width: {:.2} mm, Depth: {:.2} mm, Length: {:.2} mm\n",
        ch_w, ch_d, ch_len
    ));
    d.push_str("    Route: Straight Y-direction from chamber bottom edge downward\n");
    d.push_str(
        "    Connects DIRECTLY to distribution channel (channel junction, no through-hole)\n",
    );
    d.push('\n');
    d.push_str("  Outlet channels (16 total):\n");
    d.push_str(&format!(
        "    Width: {:.2} mm, Depth: {:.2} mm, Length: {:.2} mm\n",
        ch_w, ch_d, ch_len
    ));
    d.push_str("    Route: Straight Y-direction from chamber top edge upward\n");
    d.push_str(
        "    Connects DIRECTLY to outlet collection channel (channel junction, no through-hole)\n",
    );
    d.push('\n');

    // Section 5: Through-Hole Coordinates
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  5. THROUGH-HOLE COORDINATES (38 total)\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str(&format!(
        "  All through-holes: {:.2} mm diameter, full thickness ({:.1} mm)\n",
        port_dia, thickness
    ));
    d.push_str("  Compatible with 1/16\" OD barb fittings\n");
    d.push('\n');
    d.push_str("  INPUT PORTS (4, left edge):\n");
    d.push_str("    Port  |  Label   |    X (mm)   |    Y (mm)\n");
    d.push_str("    ------+----------+-------------+-------------\n");
    for (i, &iy) in input_ys.iter().enumerate() {
        d.push_str(&format!(
            "      {}   | {:8} |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            input_labels[i],
            input_x,
            iy
        ));
    }
    d.push('\n');

    d.push_str("  OUTPUT PORTS (2, right edge):\n");
    d.push_str("    Port  |  Label   |    X (mm)   |    Y (mm)\n");
    d.push_str("    ------+----------+-------------+-------------\n");
    for (i, &oy) in output_ys.iter().enumerate() {
        d.push_str(&format!(
            "      {}   | {:8} |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            output_labels[i],
            output_x,
            oy
        ));
    }
    d.push('\n');

    d.push_str("  VALVE PORTS (32: 16 OUT + 16 RETURN, bottom edge):\n");
    d.push_str(&format!("    All valve ports at Y = {:.1} mm\n", valve_y));
    d.push_str("    Ch  | Col | Row |   OUT X   | RETURN X\n");
    d.push_str("    ----+-----+-----+-----------+-----------\n");
    for vp in &valve_ports {
        let ch_num = vp.row * num_cols + vp.col + 1; // actual chamber number (row-first)
        d.push_str(&format!(
            "    {:2}  |  {}  |  {}  |  {:+7.2}  |  {:+7.2}\n",
            ch_num, vp.col, vp.row, vp.out_x, vp.ret_x
        ));
    }
    d.push('\n');
    d.push_str("  TOTAL THROUGH-HOLES: 38\n");
    d.push_str("    Input ports:         4\n");
    d.push_str("    Output ports:        2\n");
    d.push_str("    Valve ports:         32 (16 OUT + 16 RETURN)\n");
    d.push_str("    Chip interface:      0 (ELIMINATED -- chambers are on-board)\n");
    d.push_str("    Mounting holes:      0 (ELIMINATED -- no clamping needed)\n");
    d.push('\n');

    // Section 6: Bus and Manifold Layout
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  6. BUS AND MANIFOLD LAYOUT\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  SHARED BUS CHANNEL:\n");
    d.push_str(&format!("    Horizontal at Y = {:.1} mm\n", bus_y));
    d.push_str(&format!(
        "    X range: {:.1} to {:.1} mm (length {:.1} mm)\n",
        bus_x_start, bus_x_end, bus_length
    ));
    d.push_str(&format!(
        "    Dimensions: {:.1}mm W x {:.1}mm D\n",
        bus_w, bus_d
    ));
    d.push('\n');
    d.push_str("  Input feed channels (4 vertical, input port to bus):\n");
    for (i, &iy) in input_ys.iter().enumerate() {
        d.push_str(&format!(
            "    {} ({}): X = {:.1}, Y = {:.1} to {:.1} (length {:.1}mm)\n",
            i + 1,
            input_labels[i],
            input_x,
            iy,
            bus_y,
            (iy - bus_y).abs()
        ));
    }
    d.push('\n');
    d.push_str("  COLLECTOR MANIFOLD:\n");
    d.push_str(&format!("    Horizontal at Y = {:.1} mm\n", collector_y));
    d.push_str(&format!(
        "    X range: {:.1} to {:.1} mm (length {:.1} mm)\n",
        collector_x_start, collector_x_end, collector_len
    ));
    d.push_str(&format!(
        "    Dimensions: {:.1}mm W x {:.1}mm D\n",
        out_w, out_d
    ));
    d.push('\n');
    d.push_str("  Output feed channels (2 vertical, collector to output ports):\n");
    for (i, &oy) in output_ys.iter().enumerate() {
        d.push_str(&format!(
            "    {} ({}): X = {:.1}, Y = {:.1} to {:.1} (length {:.1}mm)\n",
            i + 1,
            output_labels[i],
            output_x,
            collector_y,
            oy,
            (collector_y - oy).abs()
        ));
    }
    d.push('\n');

    // Section 7: Distribution Routing
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  7. DISTRIBUTION ROUTING\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  DISTRIBUTION PATH (per chamber):\n");
    d.push_str("    1. Bus tap: horizontal at bus_y from bus to distribution X offset\n");
    d.push_str("    2. Segment A: vertical from bus_y down to valve OUT port\n");
    d.push_str("    3. [External solenoid pinch valve on silicone tubing]\n");
    d.push_str("    4. Segment B: vertical from valve RETURN port up to inlet channel endpoint\n");
    d.push_str("    5. Horizontal connector at inlet_endpoint_y (CHANNEL JUNCTION)\n");
    d.push_str("       --> connects distribution channel directly to chamber inlet channel\n");
    d.push_str("       --> NO through-hole interface (monolithic design)\n");
    d.push('\n');
    d.push_str("  Distribution X offsets (within each column, to avoid crossing):\n");
    d.push_str("    Row 3 channel: col_x - 7.5 mm\n");
    d.push_str("    Row 2 channel: col_x - 2.5 mm\n");
    d.push_str("    Row 1 channel: col_x + 2.5 mm\n");
    d.push_str("    Row 0 channel: col_x + 7.5 mm\n");
    d.push('\n');
    d.push_str("  OUTLET ROUTING (per chamber):\n");
    d.push_str("    1. Horizontal connector at outlet_endpoint_y (CHANNEL JUNCTION)\n");
    d.push_str("       --> connects chamber outlet channel directly to outlet routing\n");
    d.push_str("    2. Vertical from outlet_endpoint_y up to collector manifold at Y = +47\n");
    d.push('\n');
    d.push_str("  Outlet X offsets (within each column, to avoid crossing):\n");
    d.push_str("    Row 0: col_x - 1.5 mm\n");
    d.push_str("    Row 1: col_x - 0.5 mm\n");
    d.push_str("    Row 2: col_x + 0.5 mm\n");
    d.push_str("    Row 3: col_x + 1.5 mm\n");
    d.push('\n');

    // Section 8: Alignment Pin Positions
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  8. ALIGNMENT PIN HOLES\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  4 alignment pin holes for cover plate registration.\n");
    d.push_str("  Bottom 2 at Y = -64 (6mm from board bottom edge).\n");
    d.push_str(
        "  Top 2 at Y = +52 (6mm from cover plate top edge at +58, inside cover plate area).\n",
    );
    d.push_str(
        "  NOTE: Top holes moved inward from board corners to accommodate 12mm connector strip.\n",
    );
    d.push('\n');
    d.push_str(&format!("  Hole diameter:  {:.2} mm\n", align_dia));
    d.push_str(&format!(
        "  Hole depth:     {:.2} mm (blind hole, from top face only)\n",
        align_depth
    ));
    d.push_str("  Inset:          6.00 mm from nearest edge (board edge for bottom, cover plate edge for top)\n");
    d.push_str("  Pin material:   1.00 mm dia stainless steel dowel pins\n");
    d.push('\n');
    d.push_str("  Alignment hole coordinates (X, Y from board center):\n");
    d.push('\n');
    d.push_str("    Align |    X (mm)   |    Y (mm)\n");
    d.push_str("    ------+-------------+-------------\n");
    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        d.push_str(&format!(
            "      {}   |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            ax,
            ay
        ));
    }
    d.push('\n');

    // Section 9: Tolerances
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  9. TOLERANCES\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  Dimension                       Tolerance       Notes\n");
    d.push_str("  ----------------------------    -----------     -------------------------\n");
    d.push_str("  Board length / width            +/- 0.10 mm     CNC edge profile\n");
    d.push_str("  Board thickness                 +/- 0.05 mm     Stock PMMA sheet tolerance\n");
    d.push_str("  Chamber width / length          +/- 0.05 mm     Pocket milling\n");
    d.push_str("  Chamber depth (200 um)          +/- 0.025 mm    Critical for cell culture\n");
    d.push_str("  Bus channel depth (300 um)      +/- 0.025 mm    Critical for flow\n");
    d.push_str("  Distribution ch width           +/- 0.05 mm     500 um channel\n");
    d.push_str("  Distribution ch depth           +/- 0.025 mm    Critical for flow\n");
    d.push_str("  Outlet ch depth                 +/- 0.025 mm    Critical for flow\n");
    d.push_str("  Port hole diameter              +/- 0.05 mm     Drilling\n");
    d.push_str("  Port hole position              +/- 0.10 mm     CNC positioning\n");
    d.push_str("  Alignment hole diameter         +/- 0.02 mm     Press-fit for pins\n");
    d.push_str("  Alignment hole depth            +/- 0.05 mm     Blind hole\n");
    d.push_str("  Chamber center-to-center        +/- 0.10 mm     Grid accuracy\n");
    d.push_str("  Surface roughness (pockets)     Ra < 0.4 um     Optical clarity req'd\n");
    d.push_str("  Surface roughness (top face)    Ra < 0.2 um     Bonding surface\n");
    d.push('\n');
    d.push_str("  CRITICAL DIMENSIONS:\n");
    d.push_str("  - Chamber depth (200 um +/- 25 um): Directly affects cell culture volume\n");
    d.push_str("    and shear stress. Must be uniform across all 16 chambers.\n");
    d.push_str("  - Bus depth (300 um +/- 25 um): Primary feed channel, affects all flow.\n");
    d.push_str("  - Distribution depth (200 um +/- 25 um): Per-chamber flow uniformity.\n");
    d.push_str("  - Mating surface flatness: < 5 um across full 180x140mm board.\n");
    d.push_str("  - Channel junctions must be continuous with no step or ridge between\n");
    d.push_str("    connecting channels of the same depth.\n");
    d.push('\n');

    // Section 10: Material Specification
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  10. MATERIAL SPECIFICATION\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  Material:       Cast PMMA (polymethyl methacrylate), also known as acrylic\n");
    d.push_str("  Grade:          Cell culture grade, optically clear\n");
    d.push_str("  Suppliers:      McMaster-Carr 8589K11, Goodfellow AC341300, or equivalent\n");
    d.push_str("  Requirements:\n");
    d.push_str("    - Cast (NOT extruded) -- required for chloroform vapor bonding\n");
    d.push_str("    - Optically clear -- required for fluorescence microscopy\n");
    d.push_str("    - Biocompatible -- no cytotoxic additives\n");
    d.push_str("    - UV transmissive if UV sterilization is planned\n");
    d.push('\n');
    d.push_str("  Main board stock:  3.0 mm thick cast PMMA sheet\n");
    d.push_str("  Cover plate stock: 1.0 mm thick cast PMMA sheet\n");
    d.push('\n');
    d.push_str("  Bonding method:  Chloroform vapor-assisted thermal bonding\n");
    d.push_str("    - Expose mating surfaces to chloroform vapor for exactly 30 seconds\n");
    d.push_str("    - Immediately press cover plate onto board, apply uniform pressure\n");
    d.push_str("    - Bond at 50-60C under 0.5 MPa for 10-15 minutes\n");
    d.push_str("    - Simultaneously smooths milling roughness (~153nm -> ~39nm Ra)\n");
    d.push_str(
        "    - Preserves channel integrity (no collapse of 3mm wide x 0.2mm deep chambers)\n",
    );
    d.push_str("    - DO NOT exceed 30s exposure -- causes channel swelling/distortion\n");
    d.push_str("    - DO NOT use liquid Weld-On 3 -- capillary wicking fills shallow channels\n");
    d.push_str("    - Ref: Ahmed et al., Sci Rep 14:2831 (2024); Ogilvie et al. (2010)\n");
    d.push('\n');

    // Section 11: Machinist Notes
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  11. MACHINIST NOTES\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  End mills required:\n");
    d.push_str(
        "    - 0.5 mm flat end mill -- bus, distribution, inlet/outlet, collection channels\n",
    );
    d.push_str("    - 3.0 mm flat end mill -- chamber pockets (or use 0.5mm in raster)\n");
    d.push_str("    - 1.5 mm drill bit -- all 38 port through-holes\n");
    d.push_str("    - 1.0 mm drill bit -- alignment pin holes (blind, 0.5mm deep)\n");
    d.push_str("    - 2.2 mm drill bit -- sensor PCB mounting holes (blind, 1.0mm deep)\n");
    d.push('\n');
    d.push_str("  Milling strategy:\n");
    d.push_str("    1. Fixture 3 mm PMMA stock on vacuum table\n");
    d.push_str("    2. Face mill top surface for flatness\n");
    d.push_str("    3. Mill bus channel (300 um deep) -- deepest feature first\n");
    d.push_str("    4. Mill all 200 um deep features in single depth pass:\n");
    d.push_str("       - 16 chamber pockets (3mm x 10mm)\n");
    d.push_str("       - 16 inlet channels + 16 outlet channels\n");
    d.push_str("       - 16 distribution channels (bus to valve to inlet junction)\n");
    d.push_str("       - 16 outlet collection channels (outlet junction to collector)\n");
    d.push_str("       - Collector manifold\n");
    d.push_str("       - Output feed channels\n");
    d.push_str("    5. Drill 38 through-holes (1.5 mm dia)\n");
    d.push_str("    6. Drill 4 blind alignment holes (1.0 mm dia, 0.5 mm deep)\n");
    d.push_str("    6b. Drill 4 blind sensor PCB mount holes (2.2 mm dia, 1.0 mm deep)\n");
    d.push_str("    7. Engrave label area (0.1 mm deep)\n");
    d.push_str("    8. Profile cut board outline (180 x 140 mm)\n");
    d.push('\n');
    d.push_str("  Feeds and speeds (cast PMMA):\n");
    d.push_str("    - 0.5 mm end mill: 18,000 RPM, 200 mm/min feed, single pass\n");
    d.push_str("    - 3.0 mm end mill: 12,000 RPM, 500 mm/min feed, single pass\n");
    d.push_str("    - Drilling: 8,000 RPM, 100 mm/min plunge rate\n");
    d.push_str("    - Use air blast or mist coolant (no flood -- swells PMMA)\n");
    d.push('\n');
    d.push_str("  CRITICAL GEOMETRY NOTES:\n");
    d.push_str(
        "    - Inter-row wall: 2.0mm solid PMMA between adjacent rows' channel endpoints.\n",
    );
    d.push_str("      Row feature span = 3mm inlet + 10mm chamber + 3mm outlet = 16mm.\n");
    d.push_str("      Row spacing = 18mm. Wall = 18 - 16 = 2mm. DO NOT increase channel length.\n");
    d.push_str("    - Outlet collection channels within each column have 0.5mm walls between\n");
    d.push_str("      adjacent 0.5mm-wide channels. Use single-pass 500um end mill. Verify\n");
    d.push_str("      wall integrity under magnification after milling.\n");
    d.push('\n');
    d.push_str("  CAUTION:\n");
    d.push_str("    - PMMA melts easily -- do not let chips reweld to surface\n");
    d.push_str("    - Use sharp, single-flute or O-flute end mills designed for plastics\n");
    d.push_str("    - Deburr all through-holes from both sides\n");
    d.push_str("    - Clean with IPA after machining (no acetone -- dissolves PMMA)\n");
    d.push_str("    - Handle with gloves after cleaning to prevent oil contamination\n");
    d.push('\n');

    // Section 12: Key Difference Callout
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  12. MONOLITHIC DESIGN -- KEY DIFFERENCES FROM OLD TWO-PIECE DESIGN\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  OLD DESIGN (separate chip + FCB):\n");
    d.push_str("    - 16-chamber chip: 127.76mm x 85.48mm x 3mm + 1mm cover = 4mm\n");
    d.push_str("    - FCB: 180mm x 140mm x 6mm (two 3mm plates)\n");
    d.push_str("    - Clamp plate: ~153mm x ~110mm x 3mm\n");
    d.push_str("    - 32 O-ring grooves on FCB top face\n");
    d.push_str("    - 76 through-holes total (68 ports + 8 mounting)\n");
    d.push_str("    - 3 separate parts to machine and align\n");
    d.push_str("    - Total stack height: ~13mm\n");
    d.push('\n');
    d.push_str("  NEW DESIGN (monolithic board + integrated sensors):\n");
    d.push_str(&format!(
        "    - Main board: {:.0}mm x {:.0}mm x {:.0}mm\n",
        length, width, thickness
    ));
    d.push_str(&format!("    - Cover plate: {:.0}mm x {:.0}mm x {:.0}mm (12mm shorter for electrode connector strip)\n", length, cover_width, cover_thickness));
    d.push_str("    - Gold electrodes: 32 TEER pads + traces (Ti/Au sputter-coated)\n");
    d.push_str("    - Sensor PCB: impedance + fluorescence, mounts on cover plate\n");
    d.push_str("    - 0 O-ring grooves\n");
    d.push_str("    - 38 through-holes + 8 blind holes (4 align + 4 sensor mount)\n");
    d.push_str("    - 2 PMMA parts + 1 sensor PCB\n");
    d.push_str(&format!(
        "    - Total stack height: {:.0}mm (board + cover, excl. sensor PCB)\n",
        thickness + cover_thickness
    ));
    d.push('\n');
    d.push_str("  BENEFITS:\n");
    d.push_str("    - 50% fewer through-holes (38 vs 76)\n");
    d.push_str("    - No O-rings (32 eliminated) -- no seal failures\n");
    d.push_str("    - No clamping mechanism -- no alignment drift\n");
    d.push_str("    - 70% thinner stack (4mm vs 13mm)\n");
    d.push_str("    - Fewer parts: 2 vs 4 (board + cover vs chip + cover + FCB top + FCB bottom + clamp)\n");
    d.push_str("    - Channel junctions are continuous (no dead volume at port interfaces)\n");
    d.push_str("    - Lower manufacturing cost and faster assembly\n");
    d.push('\n');
    // Section 13: Electrode Layout
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  13. GOLD ELECTRODE LAYOUT (TEER / Impedance Sensing)\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  PURPOSE: Measure transepithelial electrical resistance (TEER) and impedance\n");
    d.push_str("  across cell layers in each chamber. Resistance increases as cells grow and\n");
    d.push_str("  form tight junctions; drops when cells die or detach.\n");
    d.push('\n');
    d.push_str("  ELECTRODE MATERIAL: Ti/Au thin film (10nm Ti adhesion layer + 100nm Au)\n");
    d.push_str("  DEPOSITION METHOD: Sputter coating with shadow mask (3D-printed or laser-cut)\n");
    d.push_str("  DEPOSITION TIMING: After CNC milling, BEFORE cover plate bonding\n");
    d.push_str("  NOTE: Gold is unaffected by chloroform vapor bonding step.\n");
    d.push('\n');
    d.push_str("  ELECTRODES PER CHAMBER: 2 (one near inlet end, one near outlet end)\n");
    d.push_str(
        "    Electrode A (inlet):  1.0mm x 2.0mm pad, centered in chamber, 1.5mm from inlet edge\n",
    );
    d.push_str("    Electrode B (outlet): 1.0mm x 2.0mm pad, centered in chamber, 1.5mm from outlet edge\n");
    d.push_str("    Total: 32 electrodes (16 chambers x 2) + 1 common ground = 33 connections\n");
    d.push('\n');
    d.push_str("  ELECTRODE POSITIONS (within each chamber):\n");
    d.push_str("    Chamber dimensions: 3.0mm W (X) x 10.0mm L (Y) x 0.2mm D\n");
    d.push_str("    Electrode A center: chamber_center_X, chamber_center_Y - 3.5mm (inlet side)\n");
    d.push_str(
        "    Electrode B center: chamber_center_X, chamber_center_Y + 3.5mm (outlet side)\n",
    );
    d.push('\n');
    for (ri, &ry) in row_ys.iter().enumerate().take(num_rows) {
        for (ci, &cx) in col_xs.iter().enumerate().take(num_cols) {
            let idx = ri * num_cols + ci;
            d.push_str(&format!(
                "    Ch{:2} [{},{}]: Elec A ({:+7.2}, {:+7.2})  Elec B ({:+7.2}, {:+7.2})\n",
                idx + 1,
                ri,
                ci,
                cx,
                ry - 3.5,
                cx,
                ry + 3.5
            ));
        }
    }
    d.push('\n');
    d.push_str("  TRACE ROUTING:\n");
    d.push_str(
        "    Traces run on the top (milled) face of the main board, between milled features.\n",
    );
    d.push_str(
        "    Each trace routes NORTH from its chamber to the connector strip (Y = +58 to +70).\n",
    );
    d.push_str(
        "    Trace width: 200um (0.2mm). Trace spacing: >= 200um between adjacent traces.\n",
    );
    d.push_str("    Traces are ~100nm thick (flat on surface, do not affect channel geometry).\n");
    d.push('\n');
    d.push_str("  CONNECTOR STRIP (EXPOSED NORTH EDGE):\n");
    d.push_str(&format!(
        "    Location: Y = +{:.0} to +{:.0} (top {:.0}mm of board, not covered by cover plate)\n",
        width / 2.0 - connector_strip,
        width / 2.0,
        connector_strip
    ));
    d.push_str("    33 edge pads at 1.0mm pitch, centered on board X axis\n");
    d.push_str("    Pad dimensions: 1.0mm x 3.0mm each\n");
    d.push_str("    Total connector width: 33mm (centered at X = 0)\n");
    d.push_str("    Compatible with 33-pin, 1.0mm pitch FFC/FPC connector\n");
    d.push('\n');
    d.push_str("  EDGE PAD PINOUT (left to right, X ascending):\n");
    d.push_str("    Pad 1:  GND (common ground)\n");
    for i in 0..16 {
        let ri = i / num_cols;
        let ci = i % num_cols;
        d.push_str(&format!(
            "    Pad {:2}: Ch{:2} Electrode A (inlet)   [{},{}]\n",
            i * 2 + 2,
            i + 1,
            ri,
            ci
        ));
        d.push_str(&format!(
            "    Pad {:2}: Ch{:2} Electrode B (outlet)  [{},{}]\n",
            i * 2 + 3,
            i + 1,
            ri,
            ci
        ));
    }
    d.push('\n');
    d.push_str("  SHADOW MASK:\n");
    d.push_str("    Material: Kapton film (0.05mm thick) or laser-cut stainless steel\n");
    d.push_str(
        "    Must have cutouts for: 32 electrode pads, 32 traces, 33 edge pads, 1 ground plane\n",
    );
    d.push_str("    Registration: Use same 4 alignment pin holes as cover plate\n");
    d.push('\n');

    // Section 14: Sensor Integration
    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  14. SENSOR PCB INTEGRATION\n");
    d.push_str(
        "================================================================================\n",
    );
    d.push('\n');
    d.push_str("  SENSOR PCB: Sits on top of cover plate, aligned with chamber grid.\n");
    d.push_str("  Provides impedance sensing (via gold electrodes) and fluorescence detection.\n");
    d.push('\n');
    d.push_str("  MOUNTING:\n");
    d.push_str("    4 blind holes on main board (M2, 1.0mm deep) with matching cover plate through-holes\n");
    d.push_str("    Positions (X, Y from board center):\n");
    for (i, &(sx, sy)) in sensor_mount_positions.iter().enumerate() {
        d.push_str(&format!(
            "      Mount {}: ({:+.1}, {:+.1})\n",
            i + 1,
            sx,
            sy
        ));
    }
    d.push_str("    Fasteners: M2 nylon screws (non-conductive, biocompatible)\n");
    d.push('\n');
    d.push_str("  IMPEDANCE SENSING:\n");
    d.push_str("    IC: AD5933 impedance network analyzer (~$15, 1kHz-100kHz sweep)\n");
    d.push_str("    Multiplexer: CD74HC4067 16-channel analog MUX (x2 for 32 electrodes)\n");
    d.push_str(
        "    Connection: FFC cable from sensor PCB to board edge pads (33-pin, 1.0mm pitch)\n",
    );
    d.push_str("    Measurement: AC impedance at multiple frequencies per chamber\n");
    d.push_str("    Read rate: All 16 chambers in < 30 seconds\n");
    d.push('\n');
    d.push_str("  FLUORESCENCE DETECTION:\n");
    d.push_str(
        "    Excitation: 16x blue LEDs (470nm) on sensor PCB bottom face, one per chamber\n",
    );
    d.push_str(
        "    Detection: 16x OPT101P photodiodes on sensor PCB bottom face, one per chamber\n",
    );
    d.push_str(
        "    Filter: Emission filter film (510nm longpass) between cover plate and sensor PCB\n",
    );
    d.push_str("    Optical path: LED -> cover plate -> chamber (cells + GFP) -> cover plate -> filter -> photodiode\n");
    d.push_str("    ADC: ADS1115 16-bit ADC (x4 for 16 photodiodes, or MUX + single ADC)\n");
    d.push_str("    NOTE: Both LED and photodiode on SAME side (top), epifluorescence geometry\n");
    d.push('\n');
    d.push_str("  CONTROLLER:\n");
    d.push_str("    MCU: ESP32-S3 (WiFi + BT, I2C for AD5933/ADS1115)\n");
    d.push_str("    Data logging: SD card + WiFi upload\n");
    d.push_str("    Measurement interval: Configurable (default: every 5 minutes)\n");
    d.push_str("    Power: USB-C, 5V\n");
    d.push('\n');
    d.push_str("  SENSOR PCB DIMENSIONS (approximate):\n");
    d.push_str("    Length (X): ~120mm (covers chamber grid + margins)\n");
    d.push_str("    Width (Y): ~90mm (covers chamber grid + FFC connector)\n");
    d.push_str("    Top face: ESP32, AD5933, MUX ICs, ADS1115, FFC connector, USB-C, SD slot\n");
    d.push_str("    Bottom face: 16 blue LEDs + 16 OPT101P photodiodes (aligned with chambers)\n");
    d.push('\n');

    d.push_str(
        "================================================================================\n",
    );
    d.push_str("  END OF DRAWING\n");
    d.push_str(
        "================================================================================\n",
    );

    std::fs::write("output/monolithic_board_16ch_drawing.txt", &d).unwrap();

    // ══════════════════════════════════════════════════════════════
    // CNC SHOP SPEC SHEET (submit with STEP file)
    // ══════════════════════════════════════════════════════════════

    let mut s = String::new();
    s.push_str("╔══════════════════════════════════════════════════════════════════════════╗\n");
    s.push_str("║  CNC MANUFACTURING SPEC SHEET                                          ║\n");
    s.push_str("║  Monolithic 16-Chamber Microfluidic Board                               ║\n");
    s.push_str("║  LaminarForge — laminarforge.org                                        ║\n");
    s.push_str("╚══════════════════════════════════════════════════════════════════════════╝\n");
    s.push('\n');
    s.push_str("FILES INCLUDED:\n");
    s.push_str("  monolithic_board_16ch.step        ← PRIMARY (3D main board)\n");
    s.push_str("  monolithic_board_16ch_cover.step  ← PRIMARY (3D cover plate)\n");
    s.push_str("  monolithic_board_16ch_top.dxf     ← 2D top-face reference\n");
    s.push_str("  monolithic_board_16ch_drawing.txt ← Full technical drawing with coordinates\n");
    s.push('\n');

    // ── PART 1 ──
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("PART 1: MAIN BOARD\n");
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str(&format!(
        "  Stock:       {} x {} x {}mm\n",
        length, width, thickness
    ));
    s.push_str("  Material:    Cast PMMA (acrylic), optically clear\n");
    s.push_str("               McMaster 8560K265 or equiv. (extruded OK if Ra < 0.4um)\n");
    s.push_str("  Qty:         1\n");
    s.push('\n');
    s.push_str("  CRITICAL: All features milled into TOP FACE ONLY.\n");
    s.push_str("  Board is flat-bottomed. No features on bottom or edges.\n");
    s.push('\n');

    s.push_str("FEATURE TABLE:\n");
    s.push_str(
        "  ┌──────────────────────────┬────────┬────────┬────────┬──────────┬──────────────┐\n",
    );
    s.push_str(
        "  │ Feature                   │ Width  │ Length │ Depth  │ Qty      │ Type         │\n",
    );
    s.push_str(
        "  ├──────────────────────────┼────────┼────────┼────────┼──────────┼──────────────┤\n",
    );
    s.push_str(&format!(
        "  │ Culture chambers           │ {:5.1}mm│ {:5.1}mm│ {:5.2}mm│ 16       │ Blind pocket │\n",
        chamber_w, chamber_l, chamber_d));
    s.push_str(&format!(
        "  │ Inlet/outlet channels      │ {:5.1}mm│ {:5.1}mm│ {:5.2}mm│ 32       │ Blind pocket │\n",
        ch_w, ch_len, ch_d));
    s.push_str(&format!(
        "  │ Bus channel                │ {:5.1}mm│{:5.0}mm │ {:5.2}mm│ 1        │ Blind pocket │\n",
        bus_w, (col_xs[num_cols-1] - col_xs[0]) + col_spacing, bus_d));
    s.push_str(&format!(
        "  │ Distribution channels      │ {:5.1}mm│  var   │ {:5.2}mm│ 16       │ Blind pocket │\n",
        dist_w, dist_d
    ));
    s.push_str(&format!(
        "  │ Outlet collection channels │ {:5.1}mm│  var   │ {:5.2}mm│ 16       │ Blind pocket │\n",
        out_w, out_d
    ));
    s.push_str(&format!(
        "  │ Collector manifold         │ {:5.1}mm│{:5.0}mm │ {:5.2}mm│ 1        │ Blind pocket │\n",
        out_w, collector_len, out_d));
    s.push_str(&format!(
        "  │ Input ports                │ ø{:.1}mm│        │ THRU   │ 4        │ Through-hole │\n",
        input_dia
    ));
    s.push_str(&format!(
        "  │ Output ports               │ ø{:.1}mm│        │ THRU   │ 2        │ Through-hole │\n",
        port_dia
    ));
    s.push_str(&format!(
        "  │ Valve ports                │ ø{:.1}mm│        │ THRU   │ 32       │ Through-hole │\n",
        valve_dia
    ));
    s.push_str(&format!(
        "  │ Alignment pin holes        │ ø{:.1}mm│        │ {:5.2}mm│ 4        │ Blind hole   │\n",
        align_dia, align_depth
    ));
    s.push_str(&format!(
        "  │ Sensor mount holes         │ ø{:.1}mm│        │ {:5.2}mm│ 4        │ Blind hole   │\n",
        sensor_mount_dia, sensor_mount_depth
    ));
    s.push_str(&format!(
        "  │ Label engraving            │ {:5.1}mm│ {:5.1}mm│ {:5.2}mm│ 1        │ Blind pocket │\n",
        label_w, label_l, label_d));
    s.push_str(
        "  └──────────────────────────┴────────┴────────┴────────┴──────────┴──────────────┘\n",
    );
    s.push('\n');

    s.push_str("TOLERANCES:\n");
    s.push_str("  Channel width:     ±0.05mm  (critical for flow control)\n");
    s.push_str("  Channel depth:     ±0.02mm  (critical — 200um channels, 10% tolerance)\n");
    s.push_str("  Through-hole dia:  ±0.05mm\n");
    s.push_str("  Through-hole pos:  ±0.10mm\n");
    s.push_str("  Chamber flatness:  ≤0.01mm  (bonding surface must be flat)\n");
    s.push_str("  Overall dims:      ±0.10mm\n");
    s.push('\n');

    s.push_str("SURFACE FINISH:\n");
    s.push_str("  Top face (milled): Ra ≤ 0.4um (optical polish preferred)\n");
    s.push_str("  Channel floors:    Ra ≤ 0.8um (smooth enough for solvent bonding)\n");
    s.push_str("  Bottom face:       As-received (not machined)\n");
    s.push_str("  NOTE: Part will be solvent-bonded to a cover plate.\n");
    s.push_str("         Top surface quality directly affects bond integrity.\n");
    s.push('\n');

    s.push_str("MACHINING NOTES:\n");
    s.push_str("  • Use single-flute O-flute end mills for PMMA (no chipping)\n");
    s.push_str("  • Smallest feature: 0.5mm wide channels — use 0.4mm or 0.5mm end mill\n");
    s.push_str("  • Spindle speed: 10,000-24,000 RPM (PMMA likes high speed, low feed)\n");
    s.push_str("  • Feed rate: 200-500mm/min for micro features\n");
    s.push_str("  • Coolant: Compressed air blast (NO liquid coolant — may crack PMMA)\n");
    s.push_str("  • Fixturing: Vacuum table or double-sided tape (NO clamps on thin stock)\n");
    s.push_str("  • Deburr all through-holes from bottom side\n");
    s.push_str("  • DO NOT use climb milling on thin walls between channels\n");
    s.push_str("  • Clean with IPA after machining (remove chips from channels)\n");
    s.push('\n');

    // ── PART 2 ──
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("PART 2: COVER PLATE\n");
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str(&format!(
        "  Stock:       {} x {} x {}mm\n",
        length, cover_width, cover_thickness
    ));
    s.push_str("  Material:    Cast PMMA (acrylic), optically clear (SAME material as Part 1)\n");
    s.push_str("  Qty:         1\n");
    s.push('\n');
    s.push_str("FEATURE TABLE:\n");
    s.push_str(&format!(
        "  Alignment through-holes:   4x ø{:.1}mm, THRU\n",
        align_dia
    ));
    s.push_str(&format!(
        "  Sensor mount through-holes: 4x ø{:.1}mm, THRU\n",
        sensor_mount_dia
    ));
    s.push_str("  All other surfaces: flat, no features\n");
    s.push('\n');
    s.push_str("  NOTE: Cover plate holes MUST align with Part 1 blind holes.\n");
    s.push_str("        See STEP file for exact positions.\n");
    s.push_str("        Both faces must be optically smooth (Ra ≤ 0.2um).\n");
    s.push('\n');

    // ── INSPECTION ──
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("INSPECTION REQUIREMENTS\n");
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("  1. Verify channel depth with profilometer (sample 3 channels minimum)\n");
    s.push_str("  2. Verify through-hole positions with CMM or optical comparator\n");
    s.push_str("  3. Verify top-face flatness (bonding surface)\n");
    s.push_str("  4. Visual inspection: no cracks, chips, or crazing on milled surfaces\n");
    s.push_str("  5. Cover plate alignment: dry-fit with alignment pins before shipping\n");
    s.push('\n');

    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("CONTACT\n");
    s.push_str("═══════════════════════════════════════════════════\n");
    s.push_str("  LaminarForge\n");
    s.push_str("  Alex Lewis — Founder\n");
    s.push_str("  laminarforge.org\n");

    std::fs::write("output/monolithic_board_16ch_shopspec.txt", &s).unwrap();

    // ══════════════════════════════════════════════════════════════
    // CONSOLE SUMMARY
    // ══════════════════════════════════════════════════════════════

    println!("Exported:");
    println!("  output/monolithic_board_16ch.step         ← SEND THIS (3D main board)");
    println!("  output/monolithic_board_16ch_cover.step   ← SEND THIS (3D cover plate)");
    println!("  output/monolithic_board_16ch_shopspec.txt ← SEND THIS (manufacturing spec sheet)");
    println!("  output/monolithic_board_16ch_top.dxf      ← SEND THIS (2D top-face reference)");
    println!("  output/monolithic_board_16ch.stl          (3D main board, mesh backup)");
    println!("  output/monolithic_board_16ch_cover.stl    (3D cover plate, mesh backup)");
    println!("  output/monolithic_board_16ch.glb          (3D visualization)");
    println!("  output/monolithic_board_16ch_cover.glb    (3D visualization)");
    println!("  output/monolithic_board_16ch_drawing.txt  (full technical drawing, internal ref)");
    println!();
    println!("== MONOLITHIC 16-CHAMBER MICROFLUIDIC BOARD ==");
    println!("== No separate chip, no O-rings, no clamping ==");
    println!();
    println!("  PART 1: Main Board");
    println!("    Overall:          {length:.0}mm x {width:.0}mm x {thickness:.0}mm");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Chambers:         16 in 4x4 grid");
    println!(
        "    Chamber size:     {chamber_w:.0}mm x {chamber_l:.0}mm x {chamber_d:.1}mm (200um)"
    );
    println!("    Bus channel:      {bus_w:.1}mm W x {bus_d:.1}mm D (Y = {bus_y:.0})");
    println!("    Distribution ch:  {dist_w:.1}mm W x {dist_d:.1}mm D");
    println!("    Outlet ch:        {out_w:.1}mm W x {out_d:.1}mm D");
    println!("    Collector:        Y = {collector_y:.0}, length = {collector_len:.1}mm");
    println!("    Through-holes:    38 total (4 input + 2 output + 32 valve)");
    println!("    Alignment pins:   4 (blind 0.5mm, {align_inset:.0}mm from edges)");
    println!();
    println!("  PART 2: Cover Plate");
    println!("    Overall:          {length:.0}mm x {cover_width:.0}mm x {cover_thickness:.0}mm");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Y offset:         Bottom-aligned, top edge at Y=+58 ({connector_strip:.0}mm connector strip exposed)");
    println!("    Features:         4 alignment through-holes + 4 sensor mount through-holes");
    println!();
    println!("  ASSEMBLY:");
    println!(
        "    1. CNC mill  2. Sputter coat electrodes  3. Bond cover plate  4. Mount sensor PCB"
    );
    println!(
        "    Total thickness:  {:.1}mm (board + cover, excl. sensor PCB)",
        thickness + cover_thickness
    );
    println!();
    println!("  GRID LAYOUT:");
    println!(
        "    Column X:  {:?}",
        col_xs
            .iter()
            .map(|x| format!("{x:+.1}"))
            .collect::<Vec<_>>()
    );
    println!(
        "    Row Y:     {:?}",
        row_ys
            .iter()
            .map(|y| format!("{y:+.1}"))
            .collect::<Vec<_>>()
    );
    println!();

    println!("  CHAMBER ALLOCATION:");
    for (i, label) in CHAMBER_LABELS.iter().enumerate() {
        let ri = i / num_cols;
        let ci = i % num_cols;
        println!(
            "    Ch{:2} [{},{}]: X={:+7.2}, Y={:+7.2}  {label}",
            i + 1,
            ri,
            ci,
            col_xs[ci],
            row_ys[ri]
        );
    }
    println!();

    println!("  INPUT PORTS (left edge, X = {input_x:.1}):");
    for (i, &iy) in input_ys.iter().enumerate() {
        println!(
            "    {}: {} at ({:.1}, {:.1})",
            i + 1,
            input_labels[i],
            input_x,
            iy
        );
    }
    println!();

    println!("  OUTPUT PORTS (right edge, X = {output_x:.1}):");
    for (i, &oy) in output_ys.iter().enumerate() {
        println!(
            "    {}: {} at ({:.1}, {:.1})",
            i + 1,
            output_labels[i],
            output_x,
            oy
        );
    }
    println!();

    println!("  VALVE PORTS (Y = {valve_y:.0}):");
    println!("    Ch  | Col | Row |   OUT X   | RETURN X");
    println!("    ----+-----+-----+-----------+-----------");
    for vp in &valve_ports {
        let ch_num = vp.row * num_cols + vp.col + 1; // actual chamber number (row-first)
        println!(
            "    {:2}  |  {}  |  {}  |  {:+7.2}  |  {:+7.2}",
            ch_num, vp.col, vp.row, vp.out_x, vp.ret_x
        );
    }
    println!();

    println!("  CHANNEL JUNCTIONS (monolithic -- replaces old through-hole interfaces):");
    for r in routes.iter() {
        let idx = r.row * num_cols + r.col + 1;
        println!("    Ch{:2}: Inlet junction at ({:+7.2}, {:+7.2})  Outlet junction at ({:+7.2}, {:+7.2})",
            idx, r.col_x, r.inlet_endpoint_y, r.col_x, r.outlet_endpoint_y);
    }
    println!();

    println!("  ALIGNMENT HOLES (cover plate registration):");
    for (i, &(ax, ay)) in align_positions.iter().enumerate() {
        println!("    Align {}: ({ax:+7.2}, {ay:+7.2})", i + 1);
    }
    println!();

    println!("  SENSOR PCB MOUNTING HOLES:");
    for (i, &(sx, sy)) in sensor_mount_positions.iter().enumerate() {
        println!(
            "    Mount {}: ({sx:+7.2}, {sy:+7.2})  M2, {sensor_mount_depth:.1}mm blind",
            i + 1
        );
    }
    println!();

    println!("  ELECTRODE CONNECTOR STRIP:");
    println!("    Location:         Y = +58 to +70 (top 12mm, exposed)");
    println!("    Pads:             33 (32 electrodes + 1 GND) at 1.0mm pitch");
    println!("    Connector:        33-pin FFC/FPC, 1.0mm pitch");
    println!();

    println!("  LABEL AREA:");
    println!("    Center: ({label_x:+.2}, {label_y:+.2})");
    println!("    Size:   {label_l:.0}mm x {label_w:.0}mm x {label_d:.1}mm");
    println!();

    println!("  INTEGRATED SENSORS:");
    println!("    Gold electrodes:  32 TEER pads (2 per chamber) + traces to edge connector");
    println!("    Sensor PCB:       Impedance (AD5933) + Fluorescence (LED/OPT101P) + ESP32");
    println!("    Connection:       33-pin FFC from sensor PCB to board edge pads");
    println!();
    println!("  vs OLD DESIGN:");
    println!("    Through-holes:  38 (was 76) -- 50% reduction");
    println!("    O-ring grooves: 0 (was 32) -- eliminated");
    println!("    Parts:          2 PMMA + sensor PCB (was 4+ PMMA)");
    println!("    Stack height:   4mm + sensor (was ~13mm)");
    println!("    Sensing:        Integrated (was: none, required microscope)");
}
