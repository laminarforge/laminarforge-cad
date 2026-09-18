//! Dimensioned model-based definition companion sheets; STEP supplies unlisted geometry.
use super::{Config, Item, Layout};
use base64::Engine;
use std::{fs, path::Path};
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
pub fn sheets(
    dir: &Path,
    parts: &[Item],
    p: &Config,
    d: &Layout,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir)?;
    let mut seen = std::collections::BTreeSet::new();
    for i in parts.iter().filter(|i| !i.reference) {
        if !seen.insert(part_key(&i.name)) {
            continue;
        }
        let holes = i.part.holes();
        let mesh = i.part.to_mesh();
        let mut bounds = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
        for v in mesh.vertices().chunks_exact(3) {
            for a in 0..3 {
                bounds[0][a] = bounds[0][a].min(v[a] as f64);
                bounds[1][a] = bounds[1][a].max(v[a] as f64);
            }
        }
        let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{{font-family:Arial,sans-serif;fill:#132936}}.title{{font-size:28px;font-weight:bold}}.note{{font-size:19px}}.small{{font-size:17px}}</style><rect x=\"25\" y=\"25\" width=\"1550\" height=\"1081\" fill=\"none\" stroke=\"#132936\"/><text x=\"50\" y=\"67\" class=\"title\">LF-CAS-V0 / {} / REV C</text><text x=\"50\" y=\"96\" class=\"small\">Units mm | Model-based definition: paired analytic STEP controls unlisted geometry | Do not scale</text>",esc(&i.name));
        for (index, axes, title) in [
            (0, [0, 1, 2], "TOP: X / Y, looking -Z"),
            (1, [0, 2, 1], "FRONT: X / Z, looking +Y"),
            (2, [1, 2, 0], "SIDE: Y / Z, looking -X"),
        ] {
            let px = 45 + index * 510;
            let py = 140;
            let a = axes[0];
            let b = axes[1];
            let dep = axes[2];
            let width = bounds[1][a] - bounds[0][a];
            let height = bounds[1][b] - bounds[0][b];
            let scale = (430.0 / width).min(310.0 / height);
            let ox = 240.0 - scale * (bounds[0][a] + bounds[1][a]) / 2.0;
            let oy = 190.0 + scale * (bounds[0][b] + bounds[1][b]) / 2.0;
            let (w, h) = (960usize, 760usize);
            let mut depth = vec![f64::NEG_INFINITY; w * h];
            let mut normal = vec![[0.0; 3]; w * h];
            let vertices = mesh.vertices();
            for ids in mesh.indices().chunks_exact(3) {
                let pts = ids.map_indexed(&vertices);
                let u = [
                    pts[1][0] - pts[0][0],
                    pts[1][1] - pts[0][1],
                    pts[1][2] - pts[0][2],
                ];
                let v = [
                    pts[2][0] - pts[0][0],
                    pts[2][1] - pts[0][1],
                    pts[2][2] - pts[0][2],
                ];
                let n = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                let len = n.iter().map(|a| a * a).sum::<f64>().sqrt().max(1e-12);
                let n = n.map(|a| a / len);
                let t = pts.map(|v| {
                    [
                        2.0 * (ox + scale * v[a]),
                        2.0 * (oy - scale * v[b]),
                        v[dep] * if index == 1 { -1.0 } else { 1.0 },
                    ]
                });
                let ar = (t[1][0] - t[0][0]) * (t[2][1] - t[0][1])
                    - (t[1][1] - t[0][1]) * (t[2][0] - t[0][0]);
                if ar.abs() < 1e-9 {
                    continue;
                }
                let lo_x = t
                    .iter()
                    .map(|v| v[0])
                    .fold(f64::INFINITY, f64::min)
                    .floor()
                    .max(0.0) as usize;
                let hi_x = t
                    .iter()
                    .map(|v| v[0])
                    .fold(f64::NEG_INFINITY, f64::max)
                    .ceil()
                    .clamp(0.0, (w - 1) as f64) as usize;
                let lo_y = t
                    .iter()
                    .map(|v| v[1])
                    .fold(f64::INFINITY, f64::min)
                    .floor()
                    .max(0.0) as usize;
                let hi_y = t
                    .iter()
                    .map(|v| v[1])
                    .fold(f64::NEG_INFINITY, f64::max)
                    .ceil()
                    .clamp(0.0, (h - 1) as f64) as usize;
                for y in lo_y..=hi_y {
                    for x in lo_x..=hi_x {
                        let xx = x as f64 + 0.5;
                        let yy = y as f64 + 0.5;
                        let b = ((xx - t[0][0]) * (t[2][1] - t[0][1])
                            - (yy - t[0][1]) * (t[2][0] - t[0][0]))
                            / ar;
                        let c = ((t[1][0] - t[0][0]) * (yy - t[0][1])
                            - (t[1][1] - t[0][1]) * (xx - t[0][0]))
                            / ar;
                        let a = 1.0 - b - c;
                        if a >= -1e-8 && b >= -1e-8 && c >= -1e-8 {
                            let z = a * t[0][2] + b * t[1][2] + c * t[2][2];
                            let k = y * w + x;
                            if z > depth[k] {
                                depth[k] = z;
                                normal[k] = n;
                            }
                        }
                    }
                }
            }
            let mut raster =
                image::RgbImage::from_pixel(w as u32, h as u32, image::Rgb([255, 255, 255]));
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let k = y * w + x;
                    if depth[k].is_finite() {
                        let edge = [k - 1, k + 1, k - w, k + w].iter().any(|&j| {
                            !depth[j].is_finite()
                                || (depth[k] - depth[j]).abs() > 0.3
                                || normal[k]
                                    .iter()
                                    .zip(normal[j])
                                    .map(|(a, b)| a * b)
                                    .sum::<f64>()
                                    < 0.95
                        });
                        raster.put_pixel(
                            x as u32,
                            y as u32,
                            image::Rgb(if edge { [30, 40, 45] } else { [231, 235, 237] }),
                        );
                    }
                }
            }
            let mut png = std::io::Cursor::new(Vec::new());
            image::DynamicImage::ImageRgb8(raster).write_to(&mut png, image::ImageFormat::Png)?;
            svg.push_str(&format!("<text x=\"{px}\" y=\"{py}\" class=\"note\">{title}</text><image x=\"{px}\" y=\"{}\" width=\"480\" height=\"380\" href=\"data:image/png;base64,{}\"/><text x=\"{px}\" y=\"560\" class=\"note\">Extents {:.3} x {:.3}</text>",py+12,base64::engine::general_purpose::STANDARD.encode(png.into_inner()),width,height));
            let x0 = px as f64 + ox + scale * bounds[0][a];
            let x1 = px as f64 + ox + scale * bounds[1][a];
            let y0 = py as f64 + 12.0 + oy - scale * bounds[1][b];
            let y1 = py as f64 + 12.0 + oy - scale * bounds[0][b];
            dimension(
                &mut svg,
                x0,
                y1 + 20.0,
                x1,
                y1 + 20.0,
                &super::feature_drawings::extent(&i.name, a, width),
            );
            dimension(
                &mut svg,
                x0 - 16.0,
                y0,
                x0 - 16.0,
                y1,
                &super::feature_drawings::extent(&i.name, b, height),
            );
            svg.push_str(&format!("<path d=\"M{x0} {y1} V{} M{x1} {y1} V{} M{x0} {y0} H{} M{x0} {y1} H{}\" stroke=\"#8aa0aa\" stroke-width=\"0.7\"/>",y1+25.0,y1+25.0,x0-21.0,x0-21.0));
            let mut marks = std::collections::BTreeMap::<String, (f64, f64, Vec<usize>)>::new();
            for (h, hole) in holes.iter().enumerate().filter(|(_, h)| h.axis == dep) {
                let x = px as f64 + ox + scale * hole.center[a];
                let y = py as f64 + 12.0 + oy - scale * hole.center[b];
                marks
                    .entry(format!("{x:.3}:{y:.3}"))
                    .or_insert((x, y, Vec::new()))
                    .2
                    .push(h + 1);
            }
            for (_, (x, y, ids)) in marks {
                let label = ids
                    .iter()
                    .map(|h| format!("H{h:02}"))
                    .collect::<Vec<_>>()
                    .join("/");
                svg.push_str(&format!("<path d=\"M {} {y} h14 M {x} {} v14\" stroke=\"#007482\"/><text x=\"{}\" y=\"{}\" font-size=\"13\">{label}</text>",x-7.0,y-7.0,x+7.0,y-7.0));
            }
        }
        hole_schedule(dir, &i.name, &holes)?;
        let mut notes=vec![format!("Assembly coordinates: X [{:.3}, {:.3}]; Y [{:.3}, {:.3}]; Z [{:.3}, {:.3}].",bounds[0][0],bounds[1][0],bounds[0][1],bounds[1][1],bounds[0][2],bounds[1][2]),"Finished dimensions after coating. General +/-0.10; bores +0.10/0; angles +/-0.5 deg; deburr 0.2-0.4.".into(),"Unless identified below: 6061-T6 aluminum; Type II clear sealed anodize, 5-15 um. Mask threads and labeled bond lands.".into(),"STEP bores are tap-drill geometry where called out below. Apply specified threads; do not leave pilot bores.".into()];
        notes.extend(part_notes(&i.name, p, d));
        for (j, n) in notes.iter().enumerate() {
            svg.push_str(&format!(
                "<text x=\"50\" y=\"{}\" class=\"note\">{}</text>",
                610 + j * 29,
                esc(n)
            ));
        }
        svg.push_str("<path d=\"M25 1040 H1575\" stroke=\"#132936\"/><text x=\"50\" y=\"1070\" class=\"note\">WATER-TEST PROTOTYPE | Thermal performance must be measured after assembly | Not a biological release</text><text x=\"50\" y=\"1097\" class=\"small\">Read manufacturing-notes and assembly instructions with this sheet. Coordinates share the assembly datum.</text></svg>");
        super::feature_drawings::sheets(dir, i, p, d, &svg, bounds)?;
        fs::write(dir.join(format!("{}.svg", i.name)), svg)?;
    }
    land_sheet(dir, p, d)?;
    guide_section(dir, p, d)?;
    Ok(())
}
trait Points {
    fn map_indexed(&self, v: &[f32]) -> [[f64; 3]; 3];
}
impl Points for [u32] {
    fn map_indexed(&self, v: &[f32]) -> [[f64; 3]; 3] {
        let mut p = [[0.0; 3]; 3];
        for j in 0..3 {
            for k in 0..3 {
                p[j][k] = v[self[j] as usize * 3 + k] as f64;
            }
        }
        p
    }
}
fn part_notes(name: &str, p: &Config, d: &Layout) -> Vec<String> {
    if name.starts_with("01_") {
        vec![
        format!("Datum A: front stop face Y=0, flatness 0.10. Datum B: guide mounting plane Z={:.3}. C: X=0.",p.guide_rail_thickness),
        format!("Guide roof nominal Z={:.3}; relief width {:.3} +/-0.10; guide seat flatness 0.05.",d.guide_ceiling,d.guide_width),
        "6x M3x0.5-6H rail taps from underside: 8 min full thread, 11 drill depth. See labeled hole schedule.".into(),
        "4x M3x0.5-6H rear taps: 8 min full thread, 11 drill depth; 2x M4x0.7-6H closure taps THRU bezel.".into(),
        "4x guard + 1x thermostat M3x0.5-6H THRU roof; 2x shim-retention M2x0.4-6H THRU bezel.".into(),
        format!("Stop receiver X={:.3}, Y={:.3}: M2x0.4-6H, 4.2 min full thread from guide roof; 8 drill depth.",d.stop_x,p.stop_y),
        "Gasket groove depth 2.50 +/-0.05 from A; width 3.50 +0.10/0; flat floor, Ra 3.2 max.".into(),
        "Heater/sensor bond lands Ra 1.6 max; mask labeled roof pad and RTD land on companion land sheet.".into(),
        "Machine from bottom/front/rear; nonfunctional corner changes require a marked-up proposal; seal/guide geometry per STEP.".into()]
    } else if name.starts_with("04_") {
        vec![
        format!("Plate contact top Z={:.3}: flatness 0.10, Ra 3.2 max. Thickness 6.00 +/-0.05; width +/-0.10.",d.drawer_top),
        "3x M3x0.5-6H flange taps from front: 8 full thread min, 11 drill depth.".into(),
        "4x M2x0.4-6H nest taps: 3.5 full thread min, 4.5 drill depth from top. Do not break underside.".into(),
        "4x guard + 1x thermostat + 2x cable anchor: M3x0.5-6H THRU; see labeled hole schedule.".into(),
        format!("Stop slot width 3.40 +0.10/0; end center spacing {:.3} +/-0.10. Slot does not set closed seal position.",d.stroke),
        "Mask underside 100x100 pad and 30x25 sensor bond lands. Preserve flat plate-support surface.".into()]
    } else if name.starts_with("05_") {
        vec!["Rear hard-stop/seal face Y=0: flatness 0.10, Ra 3.2 max; no scratches crossing gasket contact.".into(),"2x closure clearance diameter 6.0; 3x flange attachment clearance diameter 3.4.".into(),"2x shim-head clearances diameter 4.6 x 1.5 deep from rear mating face; no threads.".into(),"X=0, Z=25: M5x0.8-6H THRU, temporary probe feedthrough; deburr both ends.".into()]
    } else if name.starts_with("03_") {
        vec!["4x diameter 3.4 THRU; M3x12 screws into fixed housing; no pressure seal required.".into(),"2x M3x0.5-6H fixed harness mounting taps from underside: 6 full thread min, 9 drill depth.".into()]
    } else if name.starts_with("06_") {
        vec!["4x diameter 2.2 THRU. M2x5 screws; plate/lid footprint clearance controls inner profile.".into(),"Provide inner corner cutter reliefs per STEP. Deburr gently; do not reduce nest web below specified profile.".into()]
    } else if name.starts_with("07_") {
        vec!["Rail top Z=6: flatness 0.05; thickness 6.00 +/-0.05; 3x diameter 3.4 THRU.".into(),"Left rail only: diameter 6.0 stop-installation access. Rail pair and risers share M3x40 screws.".into()]
    } else if name.starts_with("08_") {
        vec!["MATERIAL/FINISH OVERRIDE: igus A160-T-010-0500-G adhesive-backed tribotape; no anodize.".into(),"STEP thickness 1.225 is nominal installed envelope. Accept installed thickness 1.10-1.35; verify slide fit.".into(),"Cut outline to +/-0.2; no adhesive overhang; replace contaminated tape. Left upper starts beyond stop pin.".into()]
    } else if name.starts_with("10_") {
        vec!["3x diameter 3.4 THRU with underside diameter 6.0 x 3.2 deep counterbores. M3x40 socket caps.".into(),"Bottom Z=-30: coplanar pair within 0.10; seat on continuous flat bench. No protruding screw heads.".into()]
    } else if name.starts_with("11_") {
        vec![
            "8 identical spacers total: diameter 6.0, bore 3.4 THRU, length 8.00 +/-0.10; plain uncoated aluminum allowed.".into(),
            "Use M3x14 socket caps through 1 mm guard and 8 mm spacer into heated block.".into(),
        ]
    } else if name.starts_with("12_") {
        vec!["1.0 mm 6061 sheet, +/-0.10 thickness; laser/waterjet/router outline; 4x diameter 3.4 THRU.".into(),"Deburr all cable edges to R0.5; no sharp burrs. Rear notch clears cable anchor and lead exit.".into(),"FINISH OVERRIDE: clear Type II anodize acceptable for non-sliding guards; do not coat beyond size tolerance.".into()]
    } else if name.starts_with("13_") {
        vec!["MATERIAL/FINISH OVERRIDE: 302 stainless shim stock, no anodize. This STEP shows nominal 0.60 total metal.".into(),"OWNER-SOURCED: select backing from measured foam; do not include nine-frame kit in CNC quote.".into(),"Ring +/-0.10 outline, ears +/-0.15; 2x diameter 2.2. Cut burr-free; stack full frames only.".into(),"3M9731 adhesive on ring only. Select stack from measured foam and adhesive; 20-30% closing compression.".into()]
    } else if name.starts_with("14_") {
        vec!["2x diameter 3.4 mounting clearance; use M3x8 socket screws through 4 mm flange.".into(),"2x jaw M2x0.4-6H taps from split face: 4.5 full thread min, 7 drill depth; M2x12 jaw screws.".into(),"Cable half-grooves diameter 5.8 and 5.3, axes Y. Round cable-entry edges R0.5; no cutting burrs.".into(),"Clamp nominal 6.0 / 5.5 mm jackets; tighten evenly and verify grip without conductor damage.".into()]
    } else if name.starts_with("15_") {
        vec![
            "2x diameter 2.2 jaw screw clearance THRU. Two jaws identical after translation."
                .into(),
            "Cable half-grooves diameter 5.8 and 5.3, axes Y. Round entry/exit contact edges R0.5."
                .into(),
        ]
    } else {
        vec!["See matching STEP and assembly mounting schedule for feature callouts.".into()]
    }
}

pub fn part_key(name: &str) -> String {
    for prefix in ["10_", "11_", "12_", "15_"] {
        if name.starts_with(prefix) {
            return prefix.into();
        }
    }
    name.into()
}
fn dimension(svg: &mut String, x0: f64, y0: f64, x1: f64, y1: f64, label: &str) {
    svg.push_str(&format!("<path d=\"M{x0} {y0} L{x1} {y1} M{} {} l8 8 M{} {} l8 8\" fill=\"none\" stroke=\"#163040\" stroke-width=\"1\"/>",x0-4.0,y0-4.0,x1-4.0,y1-4.0));
    let vertical = (x1 - x0).abs() < 0.01;
    let x = (x0 + x1) / 2.0 - if vertical { 6.0 } else { 0.0 };
    let y = (y0 + y1) / 2.0 - if vertical { 0.0 } else { 5.0 };
    let transform = if vertical {
        format!(" transform=\"rotate(-90 {x} {y})\"")
    } else {
        String::new()
    };
    svg.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" font-size=\"13\"{transform}>{}</text>",
        esc(label)
    ));
}

fn callout(name: &str, diameter: f64) -> String {
    match name {
        "rail_M3_tap" => "M3x0.5-6H; 8 full thread / 11 drill from underside".into(),
        "rear_M3_tap" => "M3x0.5-6H; 8 full thread / 11 drill from rear".into(),
        "flange_M3_tap" => "M3x0.5-6H; 8 full thread / 11 drill from front".into(),
        "fixed_harness_M3_tap" => "M3x0.5-6H; 6 full thread / 9 drill from underside".into(),
        "guard_M3_tap" | "cutoff_M3_tap" | "harness_anchor_M3" => "M3x0.5-6H THRU".into(),
        "stop_receiver" => "M2x0.4-6H; 4.2 full thread / 8 drill from guide roof".into(),
        "nest_M2_tap" => "M2x0.4-6H; 3.5 full thread / 4.5 drill from top".into(),
        "jaw_M2_tap" => "M2x0.4-6H; 4.5 full thread / 7 drill from split face".into(),
        "shim_M2_tap" => "M2x0.4-6H THRU".into(),
        "closure_M4_tap" => "M4x0.7-6H THRU".into(),
        "validation_M5_port" => "M5x0.8-6H THRU".into(),
        "shim_head_relief" => "D4.6 x 1.5 deep from rear mating face".into(),
        "riser_head_counterbore" => "D6.0 x 3.2 deep from underside; paired with D3.4".into(),
        _ => format!("D{diameter:.2} THRU"),
    }
}
fn hole_schedule(
    dir: &Path,
    name: &str,
    holes: &[super::solid::Hole],
) -> Result<(), Box<dyn std::error::Error>> {
    if holes.is_empty() {
        return Ok(());
    }
    let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{{font-family:Arial;fill:#132936;font-size:19px}}</style><text x=\"50\" y=\"60\" font-weight=\"bold\">LF-CAS-V0 / {name} / REV C / HOLE SCHEDULE</text><text x=\"50\" y=\"102\">Coordinates in assembly datum, mm; axis is hole axis; dash means coordinate along hole axis.</text><text x=\"50\" y=\"136\">Center location +/-0.10 unless overridden; clearance diameters +0.10/0. Blind drill depths exclude drill point.</text><text x=\"50\" y=\"170\">Labels match orthographic sheet. Tap-drill diameters in STEP are not finished threaded diameters.</text>");
    for (x, t) in [
        (50, "ID"),
        (130, "Axis"),
        (220, "X"),
        (350, "Y"),
        (480, "Z"),
        (620, "Finished feature"),
    ] {
        svg.push_str(&format!(
            "<text x=\"{x}\" y=\"220\" font-weight=\"bold\">{t}</text>"
        ));
    }
    for (j, h) in holes.iter().enumerate() {
        let y = 260 + j * 34;
        svg.push_str(&format!("<path d=\"M50 {} H1540\" stroke=\"#ddd\"/><text x=\"50\" y=\"{y}\">H{:02}</text><text x=\"130\" y=\"{y}\">{}</text>",y+10,j+1,["X","Y","Z"][h.axis]));
        for k in 0..3 {
            let v = if k == h.axis {
                "-".into()
            } else {
                format!("{:.3}", h.center[k])
            };
            svg.push_str(&format!(
                "<text x=\"{}\" y=\"{y}\">{v}</text>",
                220 + k * 130
            ));
        }
        svg.push_str(&format!(
            "<text x=\"620\" y=\"{y}\">{}</text>",
            esc(&callout(&h.name, h.diameter))
        ));
    }
    svg.push_str("<text x=\"50\" y=\"1080\">No countersinking unless specified. Protect remaining wall beneath blind holes; remove chips and verify thread fit.</text></svg>");
    fs::write(dir.join(format!("{name}-holes.svg")), svg)?;
    Ok(())
}
fn land_sheet(dir: &Path, p: &Config, d: &Layout) -> Result<(), Box<dyn std::error::Error>> {
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{font-family:Arial;fill:#132936;font-size:20px}</style><text x=\"50\" y=\"60\" font-size=\"28\">LF-CAS-V0 / REV C / HEATER LANDS AND GUIDE FIT</text><text x=\"50\" y=\"102\">Coordinate diagram: X horizontal, Y up. Same X/Y lands on roof exterior and drawer underside.</text>");
    let sx = 440.0;
    let sy = 630.0;
    let k = 3.0;
    let rect = |x: f64, y: f64, w: f64, h: f64, label: &str| {
        format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#e5f3f2\" stroke=\"#007482\"/><text x=\"{}\" y=\"{}\">{label}</text>",sx+x*k,sy-(y+h)*k,w*k,h*k,sx+x*k+5.0,sy-(y+h)*k+25.0)
    };
    svg.push_str(&rect(
        -p.heater_x / 2.0,
        d.plate_center_y - p.heater_y / 2.0,
        p.heater_x,
        p.heater_y,
        "H: heater bond land",
    ));
    svg.push_str(&rect(-20.0, d.drawer_y - 42.0, 30.0, 25.0, "S: RTD"));
    svg.push_str(&format!("<text x=\"830\" y=\"210\">H: X={:.3}..{:.3}; Y={:.3}..{:.3}</text><text x=\"830\" y=\"255\">S: X=-20..10; Y={:.3}..{:.3}</text><text x=\"830\" y=\"300\">Roof exterior Z={:.3}</text><text x=\"830\" y=\"345\">Drawer underside Z={:.3}</text>",-p.heater_x/2.0,p.heater_x/2.0,d.plate_center_y-p.heater_y/2.0,d.plate_center_y+p.heater_y/2.0,d.drawer_y-42.0,d.drawer_y-17.0,d.roof_top,d.drawer_bottom));
    svg.push_str("<text x=\"830\" y=\"400\">Mask H and S, boundary +/-0.5 mm.</text><text x=\"830\" y=\"445\">Ra 1.6 max only on these bond lands.</text><text x=\"830\" y=\"490\">Protect bare lands during transport.</text><text x=\"830\" y=\"535\">All other general surfaces Ra 3.2 max.</text>");
    for (j,line) in [
        format!("Guide vertical separation: roof Z={:.3} to rail-seat plane Z={:.3}: {:.3} +/-0.05.",d.guide_ceiling,p.guide_rail_thickness,d.guide_ceiling-p.guide_rail_thickness),
        "Measure separation directly from the actual rail seating plane; do not stack global Z tolerances.".into(),
        "Drawer thickness 6.00 +/-0.05; installed upper/lower tape 1.10..1.35 each.".into(),
        "Calculated vertical clearance 0.15..0.85 before form error; verify assembled 0.10..0.90 cold and warm.".into(),
        "Guide width and drawer width +/-0.10 each; centered side clearance 0.15..0.60 with specified tape.".into(),
        "Retain 0.05 flatness at guide seating/sliding faces. Plate-support and front stop faces: 0.10 flatness.".into(),
        "Uniform water temperature and easy manual travel are acceptance goals; no pressure/leak-rate certification.".into(),
    ].iter().enumerate() {svg.push_str(&format!("<text x=\"50\" y=\"{}\">{}</text>",710+j*48,esc(line)));}
    svg.push_str("</svg>");
    fs::write(dir.join("02_heater-lands-guide-fit.svg"), svg)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opposite_rails_and_bracket_bases_are_not_consolidated() {
        assert_ne!(
            part_key("07_guide_rail_left"),
            part_key("07_guide_rail_right")
        );
        assert_ne!(
            part_key("14_fixed_harness_bracket"),
            part_key("14_moving_harness_bracket")
        );
        assert_eq!(
            part_key("11_roof_spacer_-61_12.00"),
            part_key("11_drawer_spacer_61_152.73")
        );
    }
    #[test]
    fn tapped_pilots_have_finished_thread_callouts() {
        for (name, expected) in [
            ("rail_M3_tap", "M3x0.5"),
            ("harness_anchor_M3", "M3x0.5"),
            ("stop_receiver", "M2x0.4"),
            ("validation_M5_port", "M5x0.8"),
            ("shim_M2_tap", "M2x0.4"),
        ] {
            assert!(callout(name, 1.6).starts_with(expected));
        }
    }
}

fn guide_section(dir: &Path, p: &Config, d: &Layout) -> Result<(), Box<dyn std::error::Error>> {
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><defs><pattern id=\"hatch\" width=\"8\" height=\"8\" patternUnits=\"userSpaceOnUse\"><path d=\"M0 8 L8 0\" stroke=\"#6a8b99\" stroke-width=\"0.8\"/></pattern></defs><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{font-family:Arial;fill:#132936;font-size:20px}</style><text x=\"50\" y=\"60\" font-size=\"28\">LF-CAS-V0 / REV C / GUIDE CROSS-SECTION A-A</text><text x=\"50\" y=\"102\">X/Z section at Y=40 mm, looking rearward (+Y). Plate, nest, guards and cables omitted for clarity.</text>");
    let k = 6.0;
    let x = |v: f64| 800.0 + k * v;
    let y = |v: f64| 620.0 - k * v;
    let mut rect = |a: f64, b: f64, z: f64, t: f64, fill: &str| {
        svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{fill}\" stroke=\"#132936\"/>",x(a),y(t),k*(b-a),k*(t-z)))
    };
    rect(
        -d.outer_x / 2.0,
        d.outer_x / 2.0,
        d.roof_bottom,
        d.roof_top,
        "url(#hatch)",
    );
    for sign in [-1.0, 1.0] {
        let mut r = |lo: f64, hi: f64, z: f64, t: f64, fill: &str| {
            rect(
                (sign * lo).min(sign * hi),
                (sign * lo).max(sign * hi),
                z,
                t,
                fill,
            )
        };
        r(
            d.cavity_x / 2.0,
            d.outer_x / 2.0,
            d.guide_ceiling,
            d.roof_bottom,
            "url(#hatch)",
        );
        r(
            d.guide_width / 2.0,
            d.outer_x / 2.0,
            p.guide_rail_thickness,
            d.guide_ceiling,
            "url(#hatch)",
        );
        r(
            d.cavity_x / 2.0 - p.guide_inreach,
            d.outer_x / 2.0,
            0.0,
            p.guide_rail_thickness,
            "#dce4e8",
        );
        r(
            d.cavity_x / 2.0,
            d.drawer_x / 2.0,
            d.upper_shoe_bottom,
            d.guide_ceiling,
            "#70b6b1",
        );
        r(
            d.cavity_x / 2.0 - p.guide_inreach + 2.0,
            d.drawer_x / 2.0,
            p.guide_rail_thickness,
            d.drawer_bottom,
            "#70b6b1",
        );
        r(
            d.drawer_x / 2.0 + p.lateral_clearance,
            d.guide_width / 2.0,
            p.guide_rail_thickness,
            d.guide_ceiling,
            "#70b6b1",
        );
    }
    rect(
        -d.drawer_x / 2.0,
        d.drawer_x / 2.0,
        d.drawer_bottom,
        d.drawer_top,
        "#e7ad66",
    );
    dimension(
        &mut svg,
        140.0,
        y(d.guide_ceiling),
        140.0,
        y(p.guide_rail_thickness),
        &format!("{:.3} +/-0.05", d.guide_ceiling - p.guide_rail_thickness),
    );
    svg.push_str(&format!(
        "<path d=\"M140 {} H{} M140 {} H{}\" stroke=\"#132936\" stroke-dasharray=\"6 4\"/>",
        y(d.guide_ceiling),
        x(-d.cavity_x / 2.0),
        y(p.guide_rail_thickness),
        x(-d.outer_x / 2.0)
    ));
    dimension(
        &mut svg,
        x(-d.guide_width / 2.0),
        685.0,
        x(d.guide_width / 2.0),
        685.0,
        &format!("guide width {:.3} +/-0.10", d.guide_width),
    );
    svg.push_str(&format!("<text x=\"50\" y=\"770\">B = actual rail-seat plane, nominal Z={:.3}; C = X=0 symmetry plane; A = front stop face Y=0 (out of section).</text>",p.guide_rail_thickness));
    for (j,t) in ["Orange: heated drawer, thickness 6.00 +/-0.05; blue-green: guide tape; hatched: fixed housing.","Guide gap is a direct size between the two seating surfaces, not two independently toleranced elevations.","Measure the assembled fit after finish and tape installation; reject rubbing or excessive play.","Roof and drawer heater/RTD locations are on the separate land sheet. No insulating strip under the plate."].iter().enumerate(){svg.push_str(&format!("<text x=\"50\" y=\"{}\">{t}</text>",820+j*50));}
    svg.push_str("</svg>");
    fs::write(dir.join("02_guide-section.svg"), svg)?;
    Ok(())
}
