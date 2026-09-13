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
    for i in parts.iter().filter(|i| !i.reference) {
        let mesh = i.part.to_mesh();
        let mut bounds = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
        for v in mesh.vertices().chunks_exact(3) {
            for a in 0..3 {
                bounds[0][a] = bounds[0][a].min(v[a] as f64);
                bounds[1][a] = bounds[1][a].max(v[a] as f64);
            }
        }
        let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{{font-family:Arial,sans-serif;fill:#132936}}.title{{font-size:28px;font-weight:bold}}.note{{font-size:19px}}.small{{font-size:17px}}</style><rect x=\"25\" y=\"25\" width=\"1550\" height=\"1081\" fill=\"none\" stroke=\"#132936\"/><text x=\"50\" y=\"67\" class=\"title\">LF-CAS-V0 / {} / REV A</text><text x=\"50\" y=\"96\" class=\"small\">Units mm | Model-based definition: paired analytic STEP controls unlisted geometry | Do not scale</text>",esc(&i.name));
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
        }
        let mut notes=vec![format!("Assembly coordinates: X [{:.3}, {:.3}]; Y [{:.3}, {:.3}]; Z [{:.3}, {:.3}].",bounds[0][0],bounds[1][0],bounds[0][1],bounds[1][1],bounds[0][2],bounds[1][2]),"Finished dimensions after coating. General +/-0.10; bores +0.10/0; angles +/-0.5 deg; deburr 0.2-0.4.".into(),"Unless identified below: 6061-T6 aluminum; Type III hard anodize, undyed, nominal 50 um. Mask threads.".into(),"STEP bores are tap-drill geometry where called out below. Apply specified threads; do not leave pilot bores.".into()];
        notes.extend(part_notes(&i.name, p, d));
        for (j, n) in notes.iter().enumerate() {
            svg.push_str(&format!(
                "<text x=\"50\" y=\"{}\" class=\"note\">{}</text>",
                610 + j * 29,
                esc(n)
            ));
        }
        svg.push_str("<path d=\"M25 1040 H1575\" stroke=\"#132936\"/><text x=\"50\" y=\"1070\" class=\"note\">WATER-TEST PROTOTYPE | Thermal performance must be measured after assembly | Not a biological release</text><text x=\"50\" y=\"1097\" class=\"small\">Read manufacturing-notes and assembly instructions with this sheet. Coordinates share the assembly datum.</text></svg>");
        fs::write(dir.join(format!("{}.svg", i.name)), svg)?;
    }
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
        format!("Datum A: front stop face Y=0, flatness 0.05. Datum B: guide mounting plane Z={:.3}. C: X=0.",p.guide_rail_thickness),
        format!("Guide roof Z={:.3} +/-0.03; relief width {:.3} +/-0.03; guide mounting face flatness 0.05.",d.guide_ceiling,d.guide_width),
        "6x M3x0.5-6H rail taps from underside: 8 min full thread, 11 drill depth. Coordinates from STEP.".into(),
        "4x M3x0.5-6H rear taps: 8 min full thread, 11 drill depth; 2x M4x0.7-6H closure taps THRU bezel.".into(),
        "4x guard + 1x thermostat M3x0.5-6H THRU roof; 2x shim-retention M2x0.4-6H THRU bezel.".into(),
        format!("Stop receiver X={:.3}, Y={:.3}: M2x0.4-6H, 4.2 min full thread from guide roof; 8 drill depth.",d.stop_x,p.stop_y),
        "Gasket groove depth 2.50 +/-0.05 from A; width 3.50 +0.10/0; flat floor, Ra 3.2 max.".into(),
        "Heater/sensor bond lands Ra 1.6 max; mask roof pad 100x100 and sensor land per mounting schedule.".into(),
        "Machine from bottom/front/rear; internal unfunctional edge radius 0.5 max; seal corners as STEP.".into()]
    } else if name.starts_with("04_") {
        vec![
        format!("Plate contact top Z={:.3}: flatness 0.05, Ra 1.6 max. Thickness 6.00 +/-0.03; width +/-0.03.",d.drawer_top),
        "3x M3x0.5-6H flange taps from front: 8 full thread min, 11 drill depth.".into(),
        "4x M2x0.4-6H nest taps: 3.5 full thread min, 4.5 drill depth from top. Do not break underside.".into(),
        "4x guard + 1x thermostat + 2x cable anchor: M3x0.5-6H THRU; locate by STEP pilot centerlines.".into(),
        format!("Stop slot width 3.40 +0.10/0; end center spacing {:.3} +/-0.10. Slot does not set closed seal position.",d.stroke),
        "Mask underside 100x100 pad and 30x25 sensor bond lands. Preserve flat plate-support surface.".into()]
    } else if name.starts_with("05_") {
        vec!["Rear hard-stop/seal face Y=0: flatness 0.05, Ra 1.6 max; no scratches crossing gasket contact.".into(),"2x closure clearance diameter 6.0; 3x flange attachment clearance diameter 3.4.".into(),"2x shim-head clearances diameter 4.6 x 1.5 deep from rear mating face; no threads.".into(),"X=0, Z=25: M5x0.8-6H THRU, temporary probe feedthrough; deburr both ends.".into()]
    } else if name.starts_with("03_") {
        vec!["4x diameter 3.4 THRU; M3x12 screws into fixed housing; no pressure seal required.".into(),"2x M3x0.5-6H fixed harness mounting taps from underside: 6 full thread min, 9 drill depth.".into()]
    } else if name.starts_with("06_") {
        vec!["4x diameter 2.2 THRU. M2x5 screws; plate/lid footprint clearance controls inner profile.".into(),"Provide inner corner cutter reliefs per STEP. Deburr gently; do not reduce nest web below specified profile.".into()]
    } else if name.starts_with("07_") {
        vec!["Rail top Z=6: flatness 0.05; thickness 6.00 +/-0.03; 3x diameter 3.4 THRU.".into(),"Left rail only: diameter 6.0 stop-installation access. Rail pair and risers share M3x40 screws.".into()]
    } else if name.starts_with("08_") {
        vec!["MATERIAL/FINISH OVERRIDE: igus A160-T-010-0500-G adhesive-backed tribotape; no anodize.".into(),"STEP thickness 1.225 is nominal installed envelope. Accept installed thickness 1.10-1.35; verify slide fit.".into(),"Cut outline to +/-0.2; no adhesive overhang; replace contaminated tape. Left upper starts beyond stop pin.".into()]
    } else if name.starts_with("10_") {
        vec!["3x diameter 3.4 THRU with underside diameter 6.0 x 3.2 deep counterbores. M3x40 socket caps.".into(),"Bottom Z=-30: coplanar pair within 0.10; seat on continuous flat bench. No protruding screw heads.".into()]
    } else if name.starts_with("11_") {
        vec![
            "8 identical spacers total: diameter 6.0, bore 3.4 THRU, length 8.00 +/-0.05.".into(),
            "Use M3x14 socket caps through 1 mm guard and 8 mm spacer into heated block.".into(),
        ]
    } else if name.starts_with("12_") {
        vec!["1.0 mm 6061 sheet, +/-0.10 thickness; laser/waterjet/router outline; 4x diameter 3.4 THRU.".into(),"Deburr all cable edges to R0.5; no sharp burrs. Rear notch clears cable anchor and lead exit.".into(),"FINISH OVERRIDE: clear Type II anodize acceptable for non-sliding guards; do not coat beyond size tolerance.".into()]
    } else if name.starts_with("13_") {
        vec!["MATERIAL/FINISH OVERRIDE: 302 stainless shim stock, no anodize. This STEP shows nominal 0.60 total metal.".into(),"Supply matching frames in 0.05/0.10/0.15/0.20/0.25/0.30/0.40/0.50 mm stock; two 0.50 frames.".into(),"Ring +/-0.10 outline, ears +/-0.15; 2x diameter 2.2. Cut burr-free; stack full frames only.".into(),"3M9731 adhesive on ring only. Select stack from measured foam and adhesive; 20-30% closing compression.".into()]
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
