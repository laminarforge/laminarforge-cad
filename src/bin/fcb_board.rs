use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Fluidic Circuit Board (FCB16) for 16-Chamber Cell Culture Platform ───
//
// Two-plate PMMA construction:
//   Bottom plate: 180mm × 140mm × 3mm — channels milled into TOP face
//   Top plate:    180mm × 140mm × 3mm — through-holes + O-ring grooves on TOP face
//   Assembly:     Bonded together (Weld-On 3), channels sealed at mating interface
//
// The 16-chamber CNC chip (127.76mm × 85.48mm) sits ON TOP of the FCB, centered.
// Chip ports align with FCB top-face through-holes via O-ring seals.
//
// Architecture:
//   4 fluid inputs (left edge) → shared bus channel (Y = -47)
//   Bus distributes to 16 per-chamber distribution channels
//   Each channel routes: bus → valve OUT port → (external solenoid) → valve RETURN → inlet
//   16 outlet channels collect to manifold (Y = +47) → 2 output ports (right edge)
//
// Coordinate system:
//   FCB center at origin
//   X: left (-90) to right (+90)    — 180mm
//   Y: bottom (-70) to top (+70)    — 140mm
//   Z: bottom (-3) to top (+3)      — 6mm (two 3mm plates)
//   Bottom plate: Z = -3 to 0
//   Top plate:    Z = 0 to +3
//
// Chip sits centered on top face, 4×4 grid:
//   Column X centers: -37.50, -12.50, +12.50, +37.50
//   Row Y centers:    +27.00, +9.00, -9.00, -27.00
//   Inlet port:  (col_x, row_y - 10.0)
//   Outlet port: (col_x, row_y + 10.0)

fn main() {
    // ── Constants from lib.rs ──
    let length = FCB16_LENGTH; // 180mm
    let width = FCB16_WIDTH; // 140mm
    let plate_t = FCB16_PLATE_THICKNESS; // 3mm each

    let port_dia = FCB16_PORT_DIAMETER; // 1.5mm
    let mount_dia = FCB16_MOUNT_DIAMETER; // 3.2mm

    let bus_w = FCB16_BUS_WIDTH; // 0.5mm
    let bus_d = FCB16_BUS_DEPTH; // 0.3mm
    let dist_w = FCB16_DIST_WIDTH; // 0.5mm
    let dist_d = FCB16_DIST_DEPTH; // 0.2mm
    let out_w = FCB16_OUTLET_WIDTH; // 0.5mm
    let out_d = FCB16_OUTLET_DEPTH; // 0.2mm

    let oring_od = FCB16_ORING_GROOVE_OD; // 3.5mm
    let oring_w = FCB16_ORING_GROOVE_WIDTH; // 0.75mm
    let oring_depth = FCB16_ORING_GROOVE_DEPTH; // 0.5mm
    let oring_id = oring_od - 2.0 * oring_w; // 2.0mm

    let num_inputs = FCB16_NUM_INPUTS; // 4
    let input_spacing = FCB16_INPUT_SPACING; // 15mm
    let input_dia = FCB16_INPUT_PORT_DIAMETER; // 1.5mm

    let num_outputs = FCB16_NUM_OUTPUTS; // 2
    let output_spacing = FCB16_OUTPUT_SPACING; // 15mm

    let valve_dia = FCB16_VALVE_PORT_DIAMETER; // 1.5mm
    let valve_pair_offset = FCB16_VALVE_PAIR_OFFSET; // 2.0mm (between OUT and RETURN)
    let _valve_row_spacing = FCB16_VALVE_ROW_SPACING; // 5.0mm (between pairs in a column)

    let base_inset = FCB16_BASE_MOUNT_INSET; // 6mm from FCB edges
    let chip_clamp_inset = FCB16_CHIP_CLAMP_INSET; // 8mm from chip edge

    // ── Derived positions ──

    // FCB edges
    let left = -length / 2.0; // -90
    let right = length / 2.0; // +90
    let bottom = -width / 2.0; // -70
    let top = width / 2.0; // +70

    // Plate Z ranges
    let bot_plate_top = 0.0; // mating surface
    let top_plate_top = plate_t; // +3.0, top face of FCB

    // Grid positions from helper functions
    let col_xs = fcb16_col_xs(); // [-37.5, -12.5, +12.5, +37.5]
    let row_ys = fcb16_row_ys(); // [+27.0, +9.0, -9.0, -27.0]

    // Input port positions (left edge, centered vertically, 15mm spacing)
    let input_x: f64 = -85.0; // left edge + 5mm inset
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

    // Bus channel Y position — below all chip inlet ports, above valve area
    let bus_y: f64 = -47.0;

    // Collector manifold Y position — above all chip outlet ports
    let collector_y: f64 = 47.0;

    // ── Tool heights for boolean cutting ──
    let bus_tool_h = bus_d + 0.2;
    let bus_z = bot_plate_top - bus_d / 2.0 + 0.1;

    let dist_tool_h = dist_d + 0.2;
    let dist_z = bot_plate_top - dist_d / 2.0 + 0.1;

    let out_tool_h = out_d + 0.2;
    let out_z = bot_plate_top - out_d / 2.0 + 0.1;

    // Through-holes extend well beyond plate thickness
    let through_h = plate_t + 1.0; // 4mm

    // O-ring groove tool
    let oring_tool_h = oring_depth + 0.2;

    // ── Compute valve port positions for all 16 chambers ──
    // Each column has 4 pairs (one per row), organized along X around the column center.
    // Within each column (col_x), 4 pairs spaced 5mm apart:
    //   Row 3 pair at col_x - 7.5
    //   Row 2 pair at col_x - 2.5
    //   Row 1 pair at col_x + 2.5
    //   Row 0 pair at col_x + 7.5
    // Each pair: OUT at pair_center - 1.0, RETURN at pair_center + 1.0

    struct ValvePort {
        out_x: f64,
        ret_x: f64,
        y: f64,
        col: usize,
        row: usize,
    }

    let mut valve_ports: Vec<ValvePort> = Vec::new();

    // Also compute distribution channel X offsets for each chamber
    // Within a column, each row's distribution channel runs at the same X as its valve pair center
    struct DistRoute {
        col: usize,
        row: usize,
        col_x: f64,
        _row_y: f64,
        inlet_y: f64,
        outlet_y: f64,
        valve_out_x: f64,
        valve_ret_x: f64,
        dist_x: f64,         // X offset for distribution channel
        outlet_route_x: f64, // X offset for outlet routing
    }

    let mut routes: Vec<DistRoute> = Vec::new();

    for (ci, &cx) in col_xs.iter().enumerate() {
        for (ri, &ry) in row_ys.iter().enumerate() {
            // Valve pair center: row 3 at -7.5, row 2 at -2.5, row 1 at +2.5, row 0 at +7.5
            // Using index mapping: row 0 → +7.5, row 1 → +2.5, row 2 → -2.5, row 3 → -7.5
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

            // Outlet routing offsets within column:
            // Row 0 at col_x - 1.5 (closest to collector)
            // Row 1 at col_x - 0.5
            // Row 2 at col_x + 0.5
            // Row 3 at col_x + 1.5
            let outlet_route_x = cx
                + match ri {
                    0 => -1.5,
                    1 => -0.5,
                    2 => 0.5,
                    3 => 1.5,
                    _ => unreachable!(),
                };

            let inlet_y_pos = fcb16_inlet_y(ry);
            let outlet_y_pos = fcb16_outlet_y(ry);

            routes.push(DistRoute {
                col: ci,
                row: ri,
                col_x: cx,
                _row_y: ry,
                inlet_y: inlet_y_pos,
                outlet_y: outlet_y_pos,
                valve_out_x: out_x,
                valve_ret_x: ret_x,
                dist_x,
                outlet_route_x,
            });
        }
    }

    // ════════════════════════════════════════════════════════════════
    // BOTTOM PLATE — channels milled into top face
    // ════════════════════════════════════════════════════════════════

    let mut bottom_plate =
        centered_cube("fcb16_bottom", length, width, plate_t).translate(0.0, 0.0, -plate_t / 2.0); // Z: -3 to 0

    // ── Shared bus channel ──
    // Horizontal bus at Y = -47, from input_x (-85) to rightmost column (+37.5)
    let bus_x_start = input_x; // -85.0
    let bus_x_end = *col_xs.last().unwrap(); // +37.5
    let bus_length = bus_x_end - bus_x_start;
    let bus_center_x = (bus_x_start + bus_x_end) / 2.0;

    let bus_channel =
        centered_cube("bus", bus_length, bus_w, bus_tool_h).translate(bus_center_x, bus_y, bus_z);
    bottom_plate = bottom_plate - bus_channel;

    // ── Input feed channels (from input ports vertically down to bus) ──
    for (i, &iy) in input_ys.iter().enumerate() {
        let feed_len = (iy - bus_y).abs();
        let feed_center_y = (iy + bus_y) / 2.0;
        let feed = centered_cube(format!("input_feed_{i}"), bus_w, feed_len, bus_tool_h).translate(
            input_x,
            feed_center_y,
            bus_z,
        );
        bottom_plate = bottom_plate - feed;
    }

    // ── Distribution channels (bus → valve OUT → [external valve] → valve RETURN → inlet) ──
    for r in &routes {
        let ch_label = format!("c{}r{}", r.col, r.row);

        // Bus tap: horizontal connector from bus at dist_x to the bus
        // The bus runs at bus_y. We need a horizontal tap from the bus's X range to dist_x.
        // Since bus runs from input_x to rightmost col, dist_x is within or near this range.
        // The tap connects the bus to the distribution channel's X offset.
        // Short horizontal at bus_y from col_x (which is on the bus) to dist_x
        let tap_x_start = r.col_x.min(r.dist_x);
        let tap_x_end = r.col_x.max(r.dist_x);
        let tap_len = tap_x_end - tap_x_start;
        if tap_len > 0.01 {
            let tap = centered_cube(
                format!("bus_tap_{ch_label}"),
                tap_len + dist_w, // add overlap for connection
                bus_w,
                bus_tool_h,
            )
            .translate((tap_x_start + tap_x_end) / 2.0, bus_y, bus_z);
            bottom_plate = bottom_plate - tap;
        }

        // Segment A: vertical channel from bus_y down to valve OUT port
        // Runs at dist_x (= valve pair center X, close to valve_out_x)
        // First, vertical from bus_y to valve_y at valve_out_x
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
            bottom_plate = bottom_plate - h_conn;
        }

        let seg_a = centered_cube(format!("dist_a_{ch_label}"), dist_w, seg_a_len, dist_tool_h)
            .translate(seg_a_x, seg_a_center_y, dist_z);
        bottom_plate = bottom_plate - seg_a;

        // Segment B: vertical channel from valve RETURN port up to chip inlet
        let seg_b_x = r.valve_ret_x;
        let seg_b_len = (r.inlet_y - valve_y).abs();
        let seg_b_center_y = (valve_y + r.inlet_y) / 2.0;

        let seg_b = centered_cube(format!("dist_b_{ch_label}"), dist_w, seg_b_len, dist_tool_h)
            .translate(seg_b_x, seg_b_center_y, dist_z);
        bottom_plate = bottom_plate - seg_b;

        // Horizontal connector from valve_ret_x to col_x at inlet_y (if they differ)
        let h_ret_to_inlet_len = (r.col_x - seg_b_x).abs();
        if h_ret_to_inlet_len > 0.01 {
            let h_conn_b = centered_cube(
                format!("inlet_conn_{ch_label}"),
                h_ret_to_inlet_len + dist_w,
                dist_w,
                dist_tool_h,
            )
            .translate((r.col_x + seg_b_x) / 2.0, r.inlet_y, dist_z);
            bottom_plate = bottom_plate - h_conn_b;
        }
    }

    // ── Outlet collection channels ──
    // Each outlet port routes vertically to collector manifold at Y = +47
    for r in &routes {
        let ch_label = format!("c{}r{}", r.col, r.row);

        // Vertical from outlet port Y to collector_y at outlet_route_x
        let out_vert_len = (collector_y - r.outlet_y).abs();
        let out_center_y = (r.outlet_y + collector_y) / 2.0;

        let out_ch = centered_cube(
            format!("outlet_{ch_label}"),
            out_w,
            out_vert_len,
            out_tool_h,
        )
        .translate(r.outlet_route_x, out_center_y, out_z);
        bottom_plate = bottom_plate - out_ch;

        // Horizontal connector from col_x to outlet_route_x at outlet_y (if they differ)
        let h_out_len = (r.col_x - r.outlet_route_x).abs();
        if h_out_len > 0.01 {
            let h_out = centered_cube(
                format!("outlet_h_{ch_label}"),
                h_out_len + out_w,
                out_w,
                out_tool_h,
            )
            .translate((r.col_x + r.outlet_route_x) / 2.0, r.outlet_y, out_z);
            bottom_plate = bottom_plate - h_out;
        }
    }

    // Collector manifold — horizontal channel at Y = collector_y
    // from leftmost column to output port region
    let collector_x_start = *col_xs.first().unwrap(); // -37.5
    let collector_x_end = output_x; // +85.0
    let collector_len = collector_x_end - collector_x_start;
    let collector_center_x = (collector_x_start + collector_x_end) / 2.0;

    let collector_manifold = centered_cube("collector", collector_len, out_w, out_tool_h)
        .translate(collector_center_x, collector_y, out_z);
    bottom_plate = bottom_plate - collector_manifold;

    // Output feed channels (collector manifold to output ports on right edge)
    for (i, &oy) in output_ys.iter().enumerate() {
        // Vertical from collector_y to output port Y at output_x
        let out_feed_len = (oy - collector_y).abs();
        if out_feed_len > 0.01 {
            let out_feed_center_y = (collector_y + oy) / 2.0;
            let out_feed =
                centered_cube(format!("output_feed_{i}"), out_w, out_feed_len, out_tool_h)
                    .translate(output_x, out_feed_center_y, out_z);
            bottom_plate = bottom_plate - out_feed;
        }
    }

    // ── Through-holes in BOTTOM PLATE ──

    // Input port through-holes
    for (i, &iy) in input_ys.iter().enumerate() {
        let hole = centered_cylinder(
            format!("input_port_bot_{i}"),
            input_dia / 2.0,
            through_h,
            32,
        )
        .translate(input_x, iy, -plate_t / 2.0);
        bottom_plate = bottom_plate - hole;
    }

    // Output port through-holes
    for (i, &oy) in output_ys.iter().enumerate() {
        let hole = centered_cylinder(
            format!("output_port_bot_{i}"),
            FCB16_OUTPUT_PORT_DIAMETER / 2.0,
            through_h,
            32,
        )
        .translate(output_x, oy, -plate_t / 2.0);
        bottom_plate = bottom_plate - hole;
    }

    // Valve port through-holes (32 total: 16 OUT + 16 RETURN)
    for (idx, vp) in valve_ports.iter().enumerate() {
        let hole_out = centered_cylinder(
            format!("valve_out_bot_{idx}"),
            valve_dia / 2.0,
            through_h,
            32,
        )
        .translate(vp.out_x, vp.y, -plate_t / 2.0);

        let hole_ret = centered_cylinder(
            format!("valve_ret_bot_{idx}"),
            valve_dia / 2.0,
            through_h,
            32,
        )
        .translate(vp.ret_x, vp.y, -plate_t / 2.0);

        bottom_plate = bottom_plate - hole_out - hole_ret;
    }

    // Chip interface port through-holes (32 total: 16 inlet + 16 outlet)
    for (idx, r) in routes.iter().enumerate() {
        let inlet_hole = centered_cylinder(
            format!("chip_inlet_bot_{idx}"),
            port_dia / 2.0,
            through_h,
            32,
        )
        .translate(r.col_x, r.inlet_y, -plate_t / 2.0);

        let outlet_hole = centered_cylinder(
            format!("chip_outlet_bot_{idx}"),
            port_dia / 2.0,
            through_h,
            32,
        )
        .translate(r.col_x, r.outlet_y, -plate_t / 2.0);

        bottom_plate = bottom_plate - inlet_hole - outlet_hole;
    }

    // Chip clamp mounting holes (4) — positioned outside chip footprint by 8mm
    let chip_half_x = FCB16_CHIP_LENGTH / 2.0; // 63.88
    let chip_half_y = FCB16_CHIP_WIDTH / 2.0; // 42.74
    let clamp_x = chip_half_x + chip_clamp_inset; // 71.88
    let clamp_y = chip_half_y + chip_clamp_inset; // 50.74

    let chip_clamp_positions = [
        (-clamp_x, -clamp_y),
        (-clamp_x, clamp_y),
        (clamp_x, -clamp_y),
        (clamp_x, clamp_y),
    ];

    for (i, &(mx, my)) in chip_clamp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("clamp_mount_bot_{i}"),
            mount_dia / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, -plate_t / 2.0);
        bottom_plate = bottom_plate - hole;
    }

    // Baseplate mounting holes (4) — 6mm from FCB edges
    let base_mount_positions = [
        (left + base_inset, bottom + base_inset),  // (-84, -64)
        (left + base_inset, top - base_inset),     // (-84, +64)
        (right - base_inset, bottom + base_inset), // (+84, -64)
        (right - base_inset, top - base_inset),    // (+84, +64)
    ];

    for (i, &(mx, my)) in base_mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("base_mount_bot_{i}"),
            mount_dia / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, -plate_t / 2.0);
        bottom_plate = bottom_plate - hole;
    }

    // ════════════════════════════════════════════════════════════════
    // TOP PLATE — through-holes + O-ring grooves
    // ════════════════════════════════════════════════════════════════

    let mut top_plate =
        centered_cube("fcb16_top", length, width, plate_t).translate(0.0, 0.0, plate_t / 2.0); // Z: 0 to +3

    // ── Chip interface through-holes + O-ring grooves (32 ports) ──
    for (idx, r) in routes.iter().enumerate() {
        for (label, py) in [("inlet", r.inlet_y), ("outlet", r.outlet_y)] {
            // Through-hole
            let hole = centered_cylinder(
                format!("chip_{label}_top_{idx}"),
                port_dia / 2.0,
                through_h,
                32,
            )
            .translate(r.col_x, py, plate_t / 2.0);

            // O-ring groove (annular pocket on top face)
            let oring_outer = centered_cylinder(
                format!("oring_outer_{label}_{idx}"),
                oring_od / 2.0,
                oring_tool_h,
                48,
            )
            .translate(r.col_x, py, top_plate_top - oring_depth / 2.0 + 0.1);

            let oring_inner = centered_cylinder(
                format!("oring_inner_{label}_{idx}"),
                oring_id / 2.0,
                oring_tool_h + 0.2,
                48,
            )
            .translate(r.col_x, py, top_plate_top - oring_depth / 2.0 + 0.1);

            let groove = oring_outer - oring_inner;
            top_plate = top_plate - hole - groove;
        }
    }

    // ── Input port through-holes (top plate) ──
    for (i, &iy) in input_ys.iter().enumerate() {
        let hole = centered_cylinder(
            format!("input_port_top_{i}"),
            input_dia / 2.0,
            through_h,
            32,
        )
        .translate(input_x, iy, plate_t / 2.0);
        top_plate = top_plate - hole;
    }

    // ── Output port through-holes (top plate) ──
    for (i, &oy) in output_ys.iter().enumerate() {
        let hole = centered_cylinder(
            format!("output_port_top_{i}"),
            FCB16_OUTPUT_PORT_DIAMETER / 2.0,
            through_h,
            32,
        )
        .translate(output_x, oy, plate_t / 2.0);
        top_plate = top_plate - hole;
    }

    // ── Valve port through-holes (top plate, 32 total) ──
    for (idx, vp) in valve_ports.iter().enumerate() {
        let hole_out = centered_cylinder(
            format!("valve_out_top_{idx}"),
            valve_dia / 2.0,
            through_h,
            32,
        )
        .translate(vp.out_x, vp.y, plate_t / 2.0);

        let hole_ret = centered_cylinder(
            format!("valve_ret_top_{idx}"),
            valve_dia / 2.0,
            through_h,
            32,
        )
        .translate(vp.ret_x, vp.y, plate_t / 2.0);

        top_plate = top_plate - hole_out - hole_ret;
    }

    // ── Mounting holes (top plate) ──
    for (i, &(mx, my)) in chip_clamp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("clamp_mount_top_{i}"),
            mount_dia / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, plate_t / 2.0);
        top_plate = top_plate - hole;
    }

    for (i, &(mx, my)) in base_mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("base_mount_top_{i}"),
            mount_dia / 2.0,
            through_h,
            32,
        )
        .translate(mx, my, plate_t / 2.0);
        top_plate = top_plate - hole;
    }

    // ════════════════════════════════════════════════════════════════
    // CHIP CLAMP PLATE (optional accessory)
    // ════════════════════════════════════════════════════════════════
    // Flat plate with through-holes for clamp bolts + large optical window.
    // Sits on top of chip, bolted through chip clamp holes to compress O-rings.

    let clamp_length = FCB16_CHIP_LENGTH + 25.0; // ~152.76mm
    let clamp_width_dim = FCB16_CHIP_WIDTH + 25.0; // ~110.48mm
    let clamp_thickness = 3.0;

    let mut clamp_plate = centered_cube("clamp", clamp_length, clamp_width_dim, clamp_thickness);

    // Through-holes for mounting bolts (matching chip clamp positions)
    for (i, &(mx, my)) in chip_clamp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("clamp_bolt_{i}"),
            mount_dia / 2.0,
            clamp_thickness + 1.0,
            32,
        )
        .translate(mx, my, 0.0);
        clamp_plate = clamp_plate - hole;
    }

    // Large window cutout for optical access
    // Window: chip minus 5mm per side
    let window_x = FCB16_CHIP_LENGTH - 10.0; // ~117.76mm
    let window_y = FCB16_CHIP_WIDTH - 10.0; // ~75.48mm
    let window = centered_cube("window", window_x, window_y, clamp_thickness + 1.0);
    clamp_plate = clamp_plate - window;

    // ════════════════════════════════════════════════════════════════
    // EXPORT STLs
    // ════════════════════════════════════════════════════════════════

    bottom_plate
        .write_stl("output/fcb16_board_bottom_plate.stl")
        .unwrap();
    top_plate
        .write_stl("output/fcb16_board_top_plate.stl")
        .unwrap();
    clamp_plate
        .write_stl("output/fcb16_board_clamp_plate.stl")
        .unwrap();

    // ════════════════════════════════════════════════════════════════
    // TECHNICAL DRAWING
    // ════════════════════════════════════════════════════════════════

    let mut drawing = String::new();
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  TECHNICAL DRAWING -- FLUIDIC CIRCUIT BOARD (FCB16)\n");
    drawing.push_str("  16-Chamber AAV Selectivity Screening Platform\n");
    drawing.push_str("  LaminarForge\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str("  Drawing Rev:    A (initial release)\n");
    drawing.push_str("  Date:           2026-02-26\n");
    drawing.push_str("  Units:          millimeters (mm)\n");
    drawing.push_str("  Origin:         Geometric center of FCB\n");
    drawing.push_str("  Coordinate:     X = long axis, Y = short axis, Z = thickness axis\n");
    drawing.push_str("  Tolerances:     See Section 8\n");
    drawing.push('\n');

    // Section 1: Overall Dimensions
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  1. OVERALL DIMENSIONS\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str(&format!(
        "  FCB Overall:      {:.0}mm x {:.0}mm x {:.0}mm (two bonded {:.0}mm plates)\n",
        length, width, FCB16_TOTAL_THICKNESS, plate_t
    ));
    drawing.push('\n');
    drawing.push_str("  PART 1 -- BOTTOM PLATE (channel plate)\n");
    drawing.push_str(&format!("    Length (X):     {:.0} mm\n", length));
    drawing.push_str(&format!("    Width  (Y):     {:.0} mm\n", width));
    drawing.push_str(&format!("    Thickness (Z):  {:.1} mm\n", plate_t));
    drawing.push_str("    Z range:        -3.0 to 0.0 mm\n");
    drawing.push_str("    Features:       Channels milled into TOP face (Z ~ 0)\n");
    drawing.push_str("    Material:       Cast PMMA (acrylic), optically clear\n");
    drawing.push('\n');
    drawing.push_str("  PART 2 -- TOP PLATE (port plate)\n");
    drawing.push_str(&format!("    Length (X):     {:.0} mm\n", length));
    drawing.push_str(&format!("    Width  (Y):     {:.0} mm\n", width));
    drawing.push_str(&format!("    Thickness (Z):  {:.1} mm\n", plate_t));
    drawing.push_str("    Z range:        0.0 to +3.0 mm\n");
    drawing.push_str("    Features:       Through-holes + O-ring grooves on TOP face\n");
    drawing.push_str("    Material:       Cast PMMA (acrylic), optically clear\n");
    drawing.push('\n');
    drawing.push_str("  PART 3 -- CLAMP PLATE (optional accessory)\n");
    drawing.push_str(&format!("    Length (X):     {:.2} mm\n", clamp_length));
    drawing.push_str(&format!("    Width  (Y):     {:.2} mm\n", clamp_width_dim));
    drawing.push_str(&format!("    Thickness (Z):  {:.1} mm\n", clamp_thickness));
    drawing.push_str(&format!(
        "    Window cutout:  {:.2}mm x {:.2}mm (optical access)\n",
        window_x, window_y
    ));
    drawing.push_str("    Material:       Cast PMMA (acrylic)\n");
    drawing.push('\n');
    drawing.push_str("  ASSEMBLY:\n");
    drawing.push_str("    Bond bottom plate to top plate with Weld-On 3 solvent cement.\n");
    drawing.push_str("    Chip (127.76 x 85.48 mm) sits centered on top face of FCB.\n");
    drawing.push_str("    Secured with clamp plate + M3 bolts through clamp holes.\n");
    drawing.push('\n');

    // Section 2: Port Coordinates
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  2. PORT COORDINATES (all 68 through-holes)\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str(&format!(
        "  Port diameter:    {:.2} mm (all ports)\n",
        port_dia
    ));
    drawing.push_str("  Port type:        Through-hole, both plates\n");
    drawing.push('\n');

    // Input ports
    drawing.push_str("  INPUT PORTS (4, left edge):\n");
    drawing.push_str("    Port  |  Label   |    X (mm)   |    Y (mm)\n");
    drawing.push_str("    ------+----------+-------------+-------------\n");
    for (i, &iy) in input_ys.iter().enumerate() {
        drawing.push_str(&format!(
            "      {}   | {:8} |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            input_labels[i],
            input_x,
            iy
        ));
    }
    drawing.push('\n');

    // Output ports
    drawing.push_str("  OUTPUT PORTS (2, right edge):\n");
    drawing.push_str("    Port  |  Label   |    X (mm)   |    Y (mm)\n");
    drawing.push_str("    ------+----------+-------------+-------------\n");
    for (i, &oy) in output_ys.iter().enumerate() {
        drawing.push_str(&format!(
            "      {}   | {:8} |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            output_labels[i],
            output_x,
            oy
        ));
    }
    drawing.push('\n');

    // Chip interface ports
    drawing.push_str("  CHIP INTERFACE PORTS (32: 16 inlet + 16 outlet):\n");
    drawing.push_str("    Ch  | Col | Row |  Inlet X  |  Inlet Y  | Outlet X  | Outlet Y\n");
    drawing.push_str("    ----+-----+-----+-----------+-----------+-----------+-----------\n");
    let mut ch_num = 1;
    for r in routes.iter() {
        drawing.push_str(&format!(
            "    {:2}  |  {}  |  {}  |  {:+7.2}  |  {:+7.2}  |  {:+7.2}  |  {:+7.2}\n",
            ch_num, r.col, r.row, r.col_x, r.inlet_y, r.col_x, r.outlet_y
        ));
        ch_num += 1;
    }
    drawing.push('\n');

    // Valve ports
    drawing.push_str("  VALVE PORTS (32: 16 OUT + 16 RETURN, bottom edge):\n");
    drawing.push_str(&format!("    All valve ports at Y = {:.1} mm\n", valve_y));
    drawing.push_str("    Ch  | Col | Row |   OUT X   | RETURN X\n");
    drawing.push_str("    ----+-----+-----+-----------+-----------\n");
    ch_num = 1;
    for vp in valve_ports.iter() {
        drawing.push_str(&format!(
            "    {:2}  |  {}  |  {}  |  {:+7.2}  |  {:+7.2}\n",
            ch_num, vp.col, vp.row, vp.out_x, vp.ret_x
        ));
        ch_num += 1;
    }
    drawing.push('\n');
    drawing.push_str("  TOTAL THROUGH-HOLES: 68\n");
    drawing.push_str("    Input ports:         4\n");
    drawing.push_str("    Output ports:        2\n");
    drawing.push_str("    Chip interface:      32 (16 inlet + 16 outlet)\n");
    drawing.push_str("    Valve ports:         32 (16 OUT + 16 RETURN)\n");
    drawing.push_str("    (All ports present in BOTH plates)\n");
    drawing.push('\n');

    // Section 3: Channel Dimensions and Routing
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  3. CHANNEL DIMENSIONS AND ROUTING\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str("  All channels milled into TOP face of bottom plate only.\n");
    drawing.push_str("  Sealed by bonding top plate on top.\n");
    drawing.push('\n');
    drawing.push_str(&format!(
        "  Bus channel:          {:.1}mm W x {:.1}mm D (500um x 300um)\n",
        bus_w, bus_d
    ));
    drawing.push_str(&format!(
        "  Distribution ch:      {:.1}mm W x {:.1}mm D (500um x 200um)\n",
        dist_w, dist_d
    ));
    drawing.push_str(&format!(
        "  Outlet collection ch: {:.1}mm W x {:.1}mm D (500um x 200um)\n",
        out_w, out_d
    ));
    drawing.push('\n');
    drawing.push_str("  DISTRIBUTION ROUTING (per chamber):\n");
    drawing.push_str("    1. Bus tap: horizontal at bus_y from bus to distribution X offset\n");
    drawing.push_str("    2. Segment A: vertical from bus_y down to valve OUT port\n");
    drawing.push_str("    3. [External solenoid pinch valve on silicone tubing]\n");
    drawing.push_str("    4. Segment B: vertical from valve RETURN port up to chip inlet\n");
    drawing.push_str("    5. Horizontal connector at inlet_y if valve return X != col X\n");
    drawing.push('\n');
    drawing.push_str("  Distribution X offsets (within each column, to avoid crossing):\n");
    drawing.push_str("    Row 3 channel: col_x - 7.5 mm (shortest path)\n");
    drawing.push_str("    Row 2 channel: col_x - 2.5 mm\n");
    drawing.push_str("    Row 1 channel: col_x + 2.5 mm\n");
    drawing.push_str("    Row 0 channel: col_x + 7.5 mm (longest path)\n");
    drawing.push('\n');
    drawing.push_str("  OUTLET ROUTING (per chamber):\n");
    drawing.push_str("    1. Horizontal connector from col_x to outlet_route_x at outlet_y\n");
    drawing.push_str("    2. Vertical from outlet_y up to collector manifold at Y = +47\n");
    drawing.push('\n');
    drawing.push_str("  Outlet X offsets (within each column, to avoid crossing):\n");
    drawing.push_str("    Row 0: col_x - 1.5 mm (closest to collector)\n");
    drawing.push_str("    Row 1: col_x - 0.5 mm\n");
    drawing.push_str("    Row 2: col_x + 0.5 mm\n");
    drawing.push_str("    Row 3: col_x + 1.5 mm (longest path)\n");
    drawing.push('\n');

    // Section 4: Bus Layout
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  4. BUS LAYOUT\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str(&format!("  Horizontal shared bus at Y = {:.1} mm\n", bus_y));
    drawing.push_str(&format!(
        "  Runs from X = {:.1} (input port) to X = {:.1} (rightmost column)\n",
        bus_x_start, bus_x_end
    ));
    drawing.push_str(&format!("  Bus length:         {:.1} mm\n", bus_length));
    drawing.push_str(&format!(
        "  Bus dimensions:     {:.1}mm W x {:.1}mm D\n",
        bus_w, bus_d
    ));
    drawing.push('\n');
    drawing.push_str("  Input feed channels: 4 vertical channels from input ports down to bus\n");
    for (i, &iy) in input_ys.iter().enumerate() {
        drawing.push_str(&format!(
            "    {} ({}): X = {:.1}, Y = {:.1} to {:.1} (length {:.1}mm)\n",
            i + 1,
            input_labels[i],
            input_x,
            iy,
            bus_y,
            (iy - bus_y).abs()
        ));
    }
    drawing.push('\n');
    drawing.push_str(&format!(
        "  Collector manifold at Y = {:.1} mm\n",
        collector_y
    ));
    drawing.push_str(&format!(
        "  Runs from X = {:.1} to X = {:.1} (output ports)\n",
        *col_xs.first().unwrap(),
        output_x
    ));
    drawing.push_str(&format!("  Collector length:   {:.1} mm\n", collector_len));
    drawing.push('\n');

    // Section 5: O-ring Groove Specs
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  5. O-RING GROOVE SPECIFICATIONS\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing
        .push_str("  Location:         Top face of top plate, around each chip interface port\n");
    drawing.push_str("  Number:           32 grooves (16 inlet + 16 outlet)\n");
    drawing.push_str(&format!("  Groove OD:        {:.1} mm\n", oring_od));
    drawing.push_str(&format!("  Groove ID:        {:.1} mm\n", oring_id));
    drawing.push_str(&format!(
        "  Groove width:     {:.2} mm (annular)\n",
        oring_w
    ));
    drawing.push_str(&format!(
        "  Groove depth:     {:.1} mm (from top face)\n",
        oring_depth
    ));
    drawing.push_str("  O-ring spec:      AS568-003 or custom (1.78mm CS, ~2mm ID)\n");
    drawing.push_str("  Material:         FKM (Viton) or silicone, Shore 70A\n");
    drawing.push('\n');

    // Section 6: Mounting Hole Positions
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  6. MOUNTING HOLE POSITIONS\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str(&format!(
        "  Hole diameter:    {:.1} mm (M3 clearance, ISO 273)\n",
        mount_dia
    ));
    drawing.push_str("  Hole type:        Through-hole, both plates\n");
    drawing.push('\n');
    drawing.push_str("  CHIP CLAMP HOLES (4) -- secure chip + clamp plate:\n");
    drawing.push_str(&format!(
        "    Positioned {:.0}mm outside chip footprint edges\n",
        chip_clamp_inset
    ));
    drawing.push_str("    Mount |    X (mm)   |    Y (mm)\n");
    drawing.push_str("    ------+-------------+-------------\n");
    for (i, &(mx, my)) in chip_clamp_positions.iter().enumerate() {
        drawing.push_str(&format!(
            "      {}   |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            mx,
            my
        ));
    }
    drawing.push('\n');
    drawing.push_str("  BASEPLATE HOLES (4) -- secure FCB to baseplate:\n");
    drawing.push_str(&format!(
        "    Positioned {:.0}mm from FCB edges\n",
        base_inset
    ));
    drawing.push_str("    Mount |    X (mm)   |    Y (mm)\n");
    drawing.push_str("    ------+-------------+-------------\n");
    for (i, &(mx, my)) in base_mount_positions.iter().enumerate() {
        drawing.push_str(&format!(
            "      {}   |   {:+7.2}   |   {:+7.2}\n",
            i + 1,
            mx,
            my
        ));
    }
    drawing.push('\n');

    // Section 7: Material Spec
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  7. MATERIAL SPECIFICATION\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing
        .push_str("  Material:       Cast PMMA (polymethyl methacrylate), also known as acrylic\n");
    drawing.push_str("  Grade:          Cell culture grade, optically clear\n");
    drawing
        .push_str("  Suppliers:      McMaster-Carr 8589K11, Goodfellow AC341300, or equivalent\n");
    drawing.push_str("  Requirements:\n");
    drawing.push_str("    - Cast (NOT extruded) -- required for solvent bonding with Weld-On 3\n");
    drawing.push_str("    - Optically clear -- required for fluorescence microscopy\n");
    drawing.push_str("    - Biocompatible -- no cytotoxic additives\n");
    drawing.push('\n');
    drawing.push_str("  Bottom plate stock: 3.0 mm thick cast PMMA sheet\n");
    drawing.push_str("  Top plate stock:    3.0 mm thick cast PMMA sheet\n");
    drawing.push_str("  Clamp plate stock:  3.0 mm thick cast PMMA sheet\n");
    drawing.push('\n');
    drawing.push_str("  Bonding agent:  Weld-On 3 (IPS Adhesives)\n");
    drawing.push_str("    - Capillary-action solvent cement for acrylic-to-acrylic bonding\n");
    drawing.push_str("    - Apply via syringe along plate edge, wicks into joint\n");
    drawing.push_str("    - Cure time: 24-48 hours before use\n");
    drawing.push_str("    - Achieves hermetic seal at microfluidic pressures (<10 psi)\n");
    drawing.push('\n');

    // Section 8: Tolerances
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  8. TOLERANCES\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str("  Dimension                       Tolerance       Notes\n");
    drawing
        .push_str("  ----------------------------    -----------     -------------------------\n");
    drawing.push_str("  Plate length / width            +/- 0.10 mm     CNC edge profile\n");
    drawing
        .push_str("  Plate thickness                 +/- 0.05 mm     Stock PMMA sheet tolerance\n");
    drawing.push_str("  Bus channel width               +/- 0.05 mm     500 um channel\n");
    drawing.push_str("  Bus channel depth               +/- 0.025 mm    Critical for flow\n");
    drawing.push_str("  Distribution ch width           +/- 0.05 mm     500 um channel\n");
    drawing.push_str("  Distribution ch depth           +/- 0.025 mm    Critical for flow\n");
    drawing.push_str("  Port hole diameter              +/- 0.05 mm     Drilling\n");
    drawing.push_str("  Port hole position              +/- 0.10 mm     CNC positioning\n");
    drawing.push_str("  O-ring groove OD                +/- 0.05 mm     Annular pocket\n");
    drawing.push_str("  O-ring groove depth             +/- 0.025 mm    Critical for seal\n");
    drawing.push_str("  Mounting hole diameter           +/- 0.10 mm     M3 clearance\n");
    drawing.push_str("  Surface roughness (channels)    Ra < 0.4 um     Flow uniformity\n");
    drawing.push_str("  Surface roughness (mating face) Ra < 0.2 um     Bonding surface\n");
    drawing.push('\n');
    drawing.push_str("  CRITICAL DIMENSIONS:\n");
    drawing.push_str("  - Bus depth (300 um +/- 25 um): Primary feed channel, affects all flow.\n");
    drawing.push_str("  - Distribution depth (200 um +/- 25 um): Per-chamber flow uniformity.\n");
    drawing.push_str("  - O-ring groove depth (500 um +/- 25 um): Must seal against chip ports.\n");
    drawing.push_str("  - Mating surface flatness: < 5 um across full 180x140mm plate.\n");
    drawing.push('\n');

    // Section 9: Machinist Notes
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  9. MACHINIST NOTES\n");
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push('\n');
    drawing.push_str("  End mills required:\n");
    drawing.push_str("    - 0.5 mm flat end mill -- bus + distribution + outlet channels\n");
    drawing.push_str("    - 0.75 mm flat end mill -- O-ring grooves (annular pockets)\n");
    drawing.push_str("    - 1.5 mm drill bit -- all port through-holes (68 total)\n");
    drawing.push_str("    - 3.2 mm drill bit -- mounting holes (8 total)\n");
    drawing.push('\n');
    drawing.push_str("  BOTTOM PLATE milling strategy:\n");
    drawing.push_str("    1. Fixture 3 mm PMMA stock on vacuum table\n");
    drawing.push_str("    2. Face mill top surface for flatness\n");
    drawing.push_str("    3. Mill bus channel (300 um deep)\n");
    drawing.push_str("    4. Mill distribution channels (200 um deep)\n");
    drawing.push_str("    5. Mill outlet collection channels (200 um deep)\n");
    drawing.push_str("    6. Drill all 68 through-holes (1.5 mm dia)\n");
    drawing.push_str("    7. Drill 8 mounting holes (3.2 mm dia)\n");
    drawing.push_str("    8. Profile cut plate outline (180 x 140 mm)\n");
    drawing.push('\n');
    drawing.push_str("  TOP PLATE milling strategy:\n");
    drawing.push_str("    1. Fixture 3 mm PMMA stock on vacuum table\n");
    drawing.push_str("    2. Face mill top surface for flatness\n");
    drawing.push_str("    3. Mill 32 O-ring grooves on top face (500 um deep, annular)\n");
    drawing.push_str("    4. Drill all 68 through-holes (1.5 mm dia)\n");
    drawing.push_str("    5. Drill 8 mounting holes (3.2 mm dia)\n");
    drawing.push_str("    6. Profile cut plate outline (180 x 140 mm)\n");
    drawing.push('\n');
    drawing.push_str("  Feeds and speeds (cast PMMA):\n");
    drawing.push_str("    - 0.5 mm end mill: 18,000 RPM, 200 mm/min feed\n");
    drawing.push_str("    - 0.75 mm end mill: 16,000 RPM, 250 mm/min feed\n");
    drawing.push_str("    - Drilling: 8,000 RPM, 100 mm/min plunge rate\n");
    drawing.push_str("    - Use air blast or mist coolant (no flood -- swells PMMA)\n");
    drawing.push('\n');
    drawing.push_str("  CAUTION:\n");
    drawing.push_str("    - PMMA melts easily -- do not let chips reweld to surface\n");
    drawing.push_str("    - Use sharp, single-flute or O-flute end mills for plastics\n");
    drawing.push_str("    - Deburr all through-holes from both sides\n");
    drawing.push_str("    - Clean with IPA after machining (no acetone -- dissolves PMMA)\n");
    drawing.push_str("    - Handle with gloves after cleaning to prevent oil contamination\n");
    drawing.push('\n');
    drawing.push_str(
        "================================================================================\n",
    );
    drawing.push_str("  END OF DRAWING\n");
    drawing.push_str(
        "================================================================================\n",
    );

    std::fs::write("output/fcb16_board_drawing.txt", &drawing).unwrap();

    // ════════════════════════════════════════════════════════════════
    // CONSOLE SUMMARY
    // ════════════════════════════════════════════════════════════════

    println!("Exported:");
    println!("  output/fcb16_board_bottom_plate.stl");
    println!("  output/fcb16_board_top_plate.stl");
    println!("  output/fcb16_board_clamp_plate.stl");
    println!("  output/fcb16_board_drawing.txt");
    println!();
    println!("== Fluidic Circuit Board (FCB16) ==");
    println!();
    println!("  PART 1: Bottom Plate (channel plate)");
    println!("    Overall:          {length:.0}mm x {width:.0}mm x {plate_t:.0}mm");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Bus channel:      {bus_w:.1}mm W x {bus_d:.1}mm D (Y = {bus_y:.0})");
    println!("    Distribution ch:  {dist_w:.1}mm W x {dist_d:.1}mm D");
    println!("    Outlet ch:        {out_w:.1}mm W x {out_d:.1}mm D");
    println!("    Bus length:       {bus_length:.1}mm (X = {bus_x_start:.1} to {bus_x_end:.1})");
    println!("    Collector:        Y = {collector_y:.0}, length = {collector_len:.1}mm");
    println!();
    println!("  PART 2: Top Plate (port plate)");
    println!("    Overall:          {length:.0}mm x {width:.0}mm x {plate_t:.0}mm");
    println!("    Material:         Cast PMMA (acrylic), optically clear");
    println!("    Chip ports:       32x dia{port_dia:.1}mm through-holes + O-ring grooves");
    println!(
        "    O-ring groove:    OD={oring_od:.1}mm, ID={oring_id:.1}mm, depth={oring_depth:.1}mm"
    );
    println!();
    println!("  PART 3: Clamp Plate (optional accessory)");
    println!("    Overall:          {clamp_length:.2}mm x {clamp_width_dim:.2}mm x {clamp_thickness:.0}mm");
    println!("    Window:           {window_x:.2}mm x {window_y:.2}mm optical access");
    println!();
    println!("  PORT SUMMARY:");
    println!("    Input ports:      {num_inputs} (left edge, {input_spacing:.0}mm spacing)");
    for (i, &iy) in input_ys.iter().enumerate() {
        println!(
            "      {}: {} at ({:.1}, {:.1})",
            i + 1,
            input_labels[i],
            input_x,
            iy
        );
    }
    println!("    Output ports:     {num_outputs} (right edge, {output_spacing:.0}mm spacing)");
    for (i, &oy) in output_ys.iter().enumerate() {
        println!(
            "      {}: {} at ({:.1}, {:.1})",
            i + 1,
            output_labels[i],
            output_x,
            oy
        );
    }
    println!("    Valve port pairs: 16 (bottom edge at Y = {valve_y:.0})");
    println!("    Chip interface:   32 (16 inlet + 16 outlet, with O-ring grooves)");
    println!("    Mounting holes:   8 (4 chip clamp + 4 baseplate)");
    println!("    TOTAL HOLES:      76 (68 ports + 8 mounting)");
    println!();

    println!("  CHIP INTERFACE PORT POSITIONS:");
    println!("    Ch  | Col | Row |  Inlet (X, Y)          |  Outlet (X, Y)");
    println!("    ----+-----+-----+------------------------+------------------------");
    let mut ch_n = 1;
    for r in routes.iter() {
        println!(
            "    {:2}  |  {}  |  {}  |  ({:+7.2}, {:+7.2})  |  ({:+7.2}, {:+7.2})",
            ch_n, r.col, r.row, r.col_x, r.inlet_y, r.col_x, r.outlet_y
        );
        ch_n += 1;
    }
    println!();

    println!("  VALVE PORT POSITIONS (Y = {valve_y:.0}):");
    println!("    Ch  | Col | Row |  OUT X     | RETURN X");
    println!("    ----+-----+-----+------------+------------");
    ch_n = 1;
    for vp in valve_ports.iter() {
        println!(
            "    {:2}  |  {}  |  {}  |  {:+7.2}    |  {:+7.2}",
            ch_n, vp.col, vp.row, vp.out_x, vp.ret_x
        );
        ch_n += 1;
    }
    println!();
    println!("  Assembly: Bond bottom plate to top plate with Weld-On 3");
    println!("  Chip sits on top face, secured with clamp plate + M3 bolts");
    println!("  Technical drawing written to output/fcb16_board_drawing.txt");
}
