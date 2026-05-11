#![allow(dead_code, unused_variables)]

use std::fs;
use std::path::Path;

use laminarforge_cad::pcb;

fn main() {
    let output_dir = Path::new("pcb/output");
    fs::create_dir_all(output_dir).expect("create output dir");
    fs::create_dir_all(output_dir.join("gerbers")).expect("create gerbers dir");

    let args: Vec<String> = std::env::args().collect();
    let freeroute = args.iter().any(|a| a == "--freeroute");
    let from_ses = args.iter().position(|a| a == "--from-ses").map(|i| {
        args.get(i + 1)
            .unwrap_or_else(|| panic!("--from-ses requires a path argument"))
            .clone()
    });

    let mut s = String::with_capacity(512 * 1024);

    pcb::header::write_header(&mut s);
    pcb::header::write_nets(&mut s);
    pcb::header::write_setup(&mut s);
    pcb::outline::write_board_outline(&mut s);

    // Serpentine heater removed — using external heater element in aluminum block.
    // MOSFET drive circuit (Q1, U4, R34, R35) still on board; heater connects via J4.
    println!("Heater: external (connects via J4)");

    let components = pcb::components::define_components();
    for comp in &components {
        pcb::footprints::write_footprint(&mut s, comp);
    }

    pcb::zones::write_copper_zones(&mut s);

    // Routing mode selection
    if let Some(ses_path) = &from_ses {
        // Import routed traces from Freerouting SES file (pure Rust)
        let ses_data = pcb::ses::parse_ses(Path::new(ses_path));
        let total_wires: usize = ses_data.routes.iter().map(|r| r.wires.len()).sum();
        let total_vias: usize = ses_data.routes.iter().map(|r| r.vias.len()).sum();
        println!(
            "\nImporting SES: {} nets, {} wires, {} vias",
            ses_data.routes.len(),
            total_wires,
            total_vias
        );
        pcb::ses::write_ses_traces(&mut s, &ses_data);

        // ── Post-SES fixups ──
        // Freerouting occasionally leaves small gaps that KiCad DRC catches.
        // Also add GND stitching vias to merge F.Cu zone islands.
        write_ses_fixups(&mut s, &ses_data, &components);
    } else if freeroute {
        // No signal traces — Freerouting handles all routing
        // Heater serpentine is already written above as part of the board structure
    } else if args.iter().any(|a| a == "--manual-route") {
        pcb::routing::write_signal_traces(&mut s, &components);
    } else {
        pcb::autorouter::write_autorouted_traces(&mut s, &components);
    }

    pcb::silkscreen::write_silkscreen(&mut s);
    s.push_str(")\n");

    pcb::libraries::write_footprint_libraries(output_dir, &components);
    println!("Generated footprint libraries in {}", output_dir.display());

    let pcb_path = output_dir.join("lamp_v1.kicad_pcb");
    fs::write(&pcb_path, &s).expect("write .kicad_pcb");
    println!("\nWritten: {}", pcb_path.display());

    let bom_path = output_dir.join("bom.csv");
    pcb::export::write_bom(&bom_path, &components);
    println!("Written: {}", bom_path.display());

    let cpl_path = output_dir.join("cpl.csv");
    pcb::export::write_cpl(&cpl_path, &components);
    println!("Written: {}", cpl_path.display());

    if freeroute {
        // Export DSN using pure Rust (no KiCad Python needed)
        let dsn_path = output_dir.join("lamp_v1.dsn");
        pcb::dsn::write_dsn(&dsn_path, &components);
        println!("\nExported: {}", dsn_path.display());
        println!("\n── Freerouting Workflow ──");
        println!("  1. Open Freerouting GUI:");
        println!("     /opt/homebrew/opt/openjdk/bin/java -jar tools/freerouting.jar");
        println!("  2. File → Open Design → select {}", dsn_path.display());
        println!("  3. Route → Autorouter (or route manually)");
        println!(
            "  4. File → Export Specctra Session File → save as {}",
            output_dir.join("lamp_v1.ses").display()
        );
        println!("  5. Import results:");
        println!(
            "     cargo run --release --bin pcb_layout -- --from-ses {} --validate",
            output_dir.join("lamp_v1.ses").display()
        );
    }

    if args.iter().any(|a| a == "--diagnose") {
        pcb::export::diagnose_format(&pcb_path);
    } else if args.iter().any(|a| a == "--validate" || a == "--export") {
        pcb::export::validate_and_export(&pcb_path, output_dir);
    }
}

/// Minimum distance from point (px,py) to line segment (x1,y1)-(x2,y2).
fn point_to_segment_dist(px: f64, py: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 1e-12 {
        let dpx = px - x1;
        let dpy = py - y1;
        return (dpx * dpx + dpy * dpy).sqrt();
    }
    let t = ((px - x1) * dx + (py - y1) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let proj_x = x1 + t * dx;
    let proj_y = y1 + t * dy;
    let dpx = px - proj_x;
    let dpy = py - proj_y;
    (dpx * dpx + dpy * dpy).sqrt()
}

/// Post-SES fixups: place GND stitching vias on a grid, avoiding all traces,
/// existing vias, component pads, mounting holes, and board edges.
fn write_ses_fixups(pcb: &mut String, ses_data: &pcb::ses::SesData, components: &[pcb::Component]) {
    use laminarforge_cad::pcb::nets::*;
    use laminarforge_cad::pcb::write_via;
    use laminarforge_cad::PCB_LENGTH;
    use laminarforge_cad::PCB_WIDTH;

    let vp = 0.6; // via pad diameter
    let vd = 0.3; // via drill diameter
    let vr = vp / 2.0; // via pad radius

    let clearance = 0.15; // mm clearance between copper edges (above KiCad 0.127mm rule)

    // Trace segments: (x1, y1, x2, y2, half_width)
    let mut segments: Vec<(f64, f64, f64, f64, f64)> = Vec::new();
    for route in &ses_data.routes {
        for wire in &route.wires {
            let hw = wire.width_mm / 2.0;
            for w in wire.points.windows(2) {
                segments.push((w[0].0, w[0].1, w[1].0, w[1].1, hw));
            }
        }
    }

    // Existing via positions (from Freerouting)
    let mut existing_vias: Vec<(f64, f64)> = Vec::new();
    for route in &ses_data.routes {
        for via in &route.vias {
            existing_vias.push((via.x_mm, via.y_mm));
        }
    }

    // Component pad positions (absolute, with rotation)
    let mut pad_obstacles: Vec<(f64, f64, f64)> = Vec::new();
    for comp in components {
        for i in 0..comp.pads.len() {
            let (px, py) = pcb::pad_abs_pos(comp, i);
            let pad = &comp.pads[i];
            let r = pad.width.max(pad.height) / 2.0;
            pad_obstacles.push((px, py, r));
        }
    }

    // Mounting holes
    let mount_holes: [(f64, f64); 4] = [(5.0, 5.0), (95.0, 5.0), (5.0, 75.0), (95.0, 75.0)];
    let mount_keepout = 4.0;
    let edge_keepout = 1.5;

    let grid_step = 2.0;
    let mut placed = 0;
    let mut stitch_positions: Vec<(f64, f64)> = Vec::new();

    let mut x = edge_keepout;
    while x <= PCB_LENGTH - edge_keepout {
        let mut y = edge_keepout;
        while y <= PCB_WIDTH - edge_keepout {
            let near_mount = mount_holes.iter().any(|&(mx, my)| {
                let dx = x - mx;
                let dy = y - my;
                (dx * dx + dy * dy).sqrt() < mount_keepout
            });
            if near_mount {
                y += grid_step;
                continue;
            }

            let too_close_to_trace = segments.iter().any(|&(x1, y1, x2, y2, hw)| {
                point_to_segment_dist(x, y, x1, y1, x2, y2) < hw + vr + clearance
            });
            if too_close_to_trace {
                y += grid_step;
                continue;
            }

            let too_close_to_via = existing_vias.iter().any(|&(vx, vy)| {
                let dx = x - vx;
                let dy = y - vy;
                (dx * dx + dy * dy).sqrt() < vp + clearance
            });
            if too_close_to_via {
                y += grid_step;
                continue;
            }

            let too_close_to_pad = pad_obstacles.iter().any(|&(px, py, pr)| {
                let dx = x - px;
                let dy = y - py;
                (dx * dx + dy * dy).sqrt() < pr + vr + clearance
            });
            if too_close_to_pad {
                y += grid_step;
                continue;
            }

            let too_close_to_stitch = stitch_positions.iter().any(|&(sx, sy)| {
                let dx = x - sx;
                let dy = y - sy;
                (dx * dx + dy * dy).sqrt() < vp + clearance
            });
            if too_close_to_stitch {
                y += grid_step;
                continue;
            }

            write_via(pcb, x, y, vp, vd, NET_GND);
            stitch_positions.push((x, y));
            placed += 1;

            y += grid_step;
        }
        x += grid_step;
    }

    println!("Added post-SES fixups: {} GND stitching vias", placed);
}
