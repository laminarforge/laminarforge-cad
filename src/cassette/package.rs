//! Readable handbooks and 1:1 cutting templates, generated alongside CAD.
use super::{Config, Layout};
use std::{fs, path::Path};
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
pub fn templates(dir: &Path, p: &Config, d: &Layout) -> Result<(), Box<dyn std::error::Error>> {
    let mut dxf = String::from(
        "0\nSECTION\n2\nHEADER\n9\n$INSUNITS\n70\n4\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n",
    );
    for (w, h, r) in [
        (d.gasket_outer_x, d.gasket_outer_z, p.gasket_outer_radius),
        (
            d.gasket_outer_x - 2.0 * p.gasket_width,
            d.gasket_outer_z - 2.0 * p.gasket_width,
            p.gasket_outer_radius - p.gasket_width,
        ),
    ] {
        let z = d.gasket_center_z;
        let b = (std::f64::consts::PI / 8.0).tan();
        let points = [
            (-w / 2.0 + r, z - h / 2.0, 0.0),
            (w / 2.0 - r, z - h / 2.0, b),
            (w / 2.0, z - h / 2.0 + r, 0.0),
            (w / 2.0, z + h / 2.0 - r, b),
            (w / 2.0 - r, z + h / 2.0, 0.0),
            (-w / 2.0 + r, z + h / 2.0, b),
            (-w / 2.0, z + h / 2.0 - r, 0.0),
            (-w / 2.0, z - h / 2.0 + r, b),
        ];
        dxf.push_str("0\nLWPOLYLINE\n8\nGASKET_CUT\n90\n8\n70\n1\n");
        for (x, y, b) in points {
            dxf.push_str(&format!("10\n{x:.6}\n20\n{y:.6}\n42\n{b:.9}\n"));
        }
    }
    dxf.push_str("0\nENDSEC\n0\nEOF\n");
    fs::write(dir.join("gasket-cut-1to1-mm.dxf"), dxf)?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{font-family:Arial;font-size:20px;fill:#163040}</style><text x=\"45\" y=\"55\" style=\"font-size:32px;font-weight:bold\">Harness routing / top view below drawer</text>");
    for (col, travel) in [(0, 0.0), (1, d.stroke)] {
        let sx = if col == 0 { 350.0 } else { 1100.0 };
        let sy = 190.0;
        let scale = 2.2;
        let a = d.drawer_y - 2.0;
        let tangent = (2.0 * a - travel + 250.0 - std::f64::consts::PI * 35.0) / 2.0;
        let x = |v: f64| sx + v * scale;
        let y = |v: f64| sy + v * scale;
        svg.push_str(&format!("<text x=\"{}\" y=\"110\">{}</text><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#e8edf0\" stroke=\"#365466\"/><path d=\"M {} {} L {} {} A {} {} 0 0 0 {} {} L {} {}\" fill=\"none\" stroke=\"#167a8a\" stroke-width=\"13.2\"/><text x=\"{}\" y=\"870\">R35; 250 mm free cable; travel {:.2} mm</text>",sx-230.0,if col==0{"CLOSED"}else{"FULLY OPEN"},x(-d.outer_x/2.0),y(p.bezel_thickness),d.outer_x*scale,(d.rear_y-p.bezel_thickness)*scale,x(-35.0),y(a-travel),x(-35.0),y(tangent),35.0*scale,35.0*scale,x(35.0),y(tangent),x(35.0),y(a),sx-260.0,travel));
    }
    svg.push_str("<text x=\"45\" y=\"950\">Two separate XY loop planes: power center Z=-18, signal center Z=-9. Cables exit clamps REARWARD.</text><text x=\"45\" y=\"988\">Reserve X=-45..45, Y=49..270, Z=-24..-3. Keep rear bay clear; do not trap loops behind a rack wall.</text><text x=\"45\" y=\"1026\">Nominal bend path shown. Inspect real jacket sag/twist and strain relief through full manual travel.</text></svg>");
    fs::write(dir.join("drawing-pages/harness-routing.svg"), svg)?;
    control_box(dir)?;
    Ok(())
}
fn control_box(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{font-family:Arial;font-size:20px;fill:#163040}</style><text x=\"40\" y=\"55\" style=\"font-size:30px;font-weight:bold\">Control box / Hammond 1554YA2GY + 1554YPL / Rev A</text><text x=\"40\" y=\"92\">Units mm. Modify purchased parts; preserve factory mounting holes. Coordinate tables in handbook control.</text>");
    for (label, oy, rear) in [
        ("FRONT BASE WALL - X / Z", 150.0, false),
        ("REAR BASE WALL - X / Z", 420.0, true),
    ] {
        svg.push_str(&format!("<text x=\"45\" y=\"{oy}\">{label}</text><rect x=\"45\" y=\"{}\" width=\"659\" height=\"154\" fill=\"#eef3f5\" stroke=\"#234\"/>",oy+20.0));
        let x = |x: f64| 45.0 + 2.2 * x;
        let y = |z: f64| oy + 20.0 + 2.2 * (69.84 - z);
        if rear {
            for xx in [45.0, 90.0, 135.0, 180.0] {
                svg.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"13.42\" fill=\"white\" stroke=\"#234\"/>",
                    x(xx),
                    y(35.0)
                ));
            }
            svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"32.56\" height=\"32.56\" fill=\"white\" stroke=\"#234\"/>",x(255.0-7.4),y(35.0+7.4)));
            for z in [25.5, 44.5] {
                svg.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"2.97\" fill=\"white\" stroke=\"#234\"/>",
                    x(255.0),
                    y(z)
                ));
            }
        } else {
            for xx in [50.0, 115.0] {
                svg.push_str(&format!("<rect x=\"{}\" y=\"{}\" width=\"99\" height=\"99\" fill=\"white\" stroke=\"#234\"/>",x(xx-22.5),y(57.5)));
            }
            for xx in [185.0, 225.0, 265.0] {
                svg.push_str(&format!(
                    "<circle cx=\"{}\" cy=\"{}\" r=\"24.75\" fill=\"white\" stroke=\"#234\"/>",
                    x(xx),
                    y(35.0)
                ));
            }
        }
    }
    svg.push_str("<text x=\"820\" y=\"150\">PANEL TOP VIEW - P(X,Y)</text><rect x=\"820\" y=\"175\" width=\"627\" height=\"495\" fill=\"#eef3f5\" stroke=\"#234\"/>");
    for (cx, cy, diam) in [
        (20.0, 170.0, 4.5),
        (120.0, 170.0, 4.5),
        (220.0, 170.0, 4.5),
        (190.0, 66.25, 5.5),
        (190.0, 113.75, 5.5),
        (250.0, 66.25, 5.5),
        (250.0, 113.75, 5.5),
    ] {
        svg.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"white\" stroke=\"#234\"/>",
            820.0 + cx * 2.2,
            175.0 + cy * 2.2,
            diam * 1.1
        ));
    }
    svg.push_str("<rect x=\"842\" y=\"510.5\" width=\"484\" height=\"77\" fill=\"none\" stroke=\"#087b86\" stroke-dasharray=\"8 4\"/><text x=\"45\" y=\"735\">Front: PID centers (50,35), (115,35), square 45; buttons (185,35), (225,35), (265,35), diameter 22.5.</text><text x=\"45\" y=\"775\">Rear: glands at X=45/90/135/180, Z=35, diameter 12.2; inlet centered (255,35), square 14.8.</text><text x=\"45\" y=\"815\">Inlet screw centers (255,25.5), (255,44.5), diameter 2.7. Jack flange BEHIND wall; bosses inward.</text><text x=\"45\" y=\"855\">Panel rail holes: (20,170), (120,170), (220,170), diameter 4.5. Rail length 220, centered Y=170.</text><text x=\"45\" y=\"895\">SSR holes: X=190/250, Y=66.25/113.75, diameter 5.5. Factory panel outline is NOT a new cutting profile.</text><text x=\"45\" y=\"945\">Fit/assembly checks: PID clips, panel elevation, terminal access, inlet mating and retained lid service leads.</text><text x=\"45\" y=\"985\">Do not drill into the enclosure floor through unused panel holes. Do not mount PIDs on the lid.</text><text x=\"45\" y=\"1035\">Read control-box-layout.md for tolerances, screw lengths, exact component positions and source drawings.</text></svg>");
    fs::write(dir.join("drawing-pages/control-box.svg"), svg)?;
    Ok(())
}
fn wrap(s: &str, max: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut line = String::new();
    for word in s.split_whitespace() {
        if word.chars().count() > max {
            if !line.is_empty() {
                out.push(std::mem::take(&mut line));
            }
            let chars: Vec<char> = word.chars().collect();
            for c in chars.chunks(max) {
                out.push(c.iter().collect());
            }
            continue;
        }
        if line.chars().count() + word.chars().count() + 1 > max {
            out.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        out.push(line);
    }
    out
}
pub fn handbook(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    render_docs(
        &dir.join("manual-pages"),
        &[
            "README.md",
            "assembly.md",
            "validation.md",
            "electrical-assembly.md",
            "control-box-layout.md",
        ],
    )?;
    render_docs(
        &dir.join("manufacturing-pages"),
        &["README.md", "manufacturing-notes.md"],
    )?;
    fs::copy(
        "docs/heated-cassette-v0/wiring.svg",
        dir.join("drawing-pages/wiring.svg"),
    )?;
    procurement(dir)?;
    Ok(())
}
fn procurement(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let quote = |s: &str| format!("\"{}\"", s.replace('"', "\"\""));
    let mut csv = String::from("section,quantity,item,notes\n");
    for (section, name) in [
        ("electrical", "electrical-assembly.md"),
        ("mechanical", "assembly.md"),
    ] {
        let text = fs::read_to_string(Path::new("docs/heated-cassette-v0").join(name))?;
        let mut in_table = false;
        for line in text.lines() {
            if line.starts_with('|') {
                if !in_table {
                    in_table = true;
                    continue;
                }
                if line.contains("---") {
                    continue;
                }
                let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
                if section == "mechanical"
                    && cells.get(1).is_some_and(|s| {
                        s.contains("Keenovo") || s.contains("Omega") || s.contains("Sensata")
                    })
                {
                    continue;
                }
                let (q, item, notes) = if section == "electrical" {
                    (
                        cells[0],
                        format!("{} [{}]", cells[2], cells[1]),
                        cells[3].to_string(),
                    )
                } else {
                    (cells[0], cells[1].to_string(), cells[2].to_string())
                };
                csv.push_str(&format!(
                    "{},{},{},{}\n",
                    quote(section),
                    quote(q),
                    quote(&item),
                    quote(&notes)
                ));
            } else if in_table {
                break;
            }
        }
    }
    for (q, item, notes) in [
        (
            "1",
            "01 housing; 03 rear; 04 drawer; 05 flange; 06 nest; left/right 07 rails",
            "One EACH; see individual STEP/drawings",
        ),
        ("2", "10 bench riser", "Identical parts after translation"),
        (
            "8",
            "11 guard spacer",
            "Diameter 6 / bore 3.4 / length 8; identical",
        ),
        ("2", "12 guard sheet", "Identical 1 mm sheet"),
        (
            "1 each",
            "14 moving/fixed cable brackets",
            "Different bases",
        ),
        ("2", "15 cable jaws", "Identical after translation"),
        (
            "4",
            "Hammond 1591MM100 M3x8",
            "Control-box factory panel mounting",
        ),
        (
            "3 sets",
            "M4x10 pan screw, flat washer, locking nut",
            "DIN rail",
        ),
        (
            "4 sets",
            "M5x10 pan screw, flat washer, locking nut",
            "SSRs",
        ),
        (
            "2",
            "M2.5x12 pan screws",
            "Trim to 6.60 mm plus measured wall thickness for inlet",
        ),
        (
            "8",
            "Panduit ABM2S-A-D plus cable ties",
            "Lid fuse-holder retention",
        ),
        (
            "2 m each",
            "16/18/20/24 AWG stranded insulated copper",
            "Prototype stock; use exact scheduled gauge and rating per circuit",
        ),
        (
            "2 m",
            "0.5 mm2 stranded control wire",
            "Controller/relay internal wiring",
        ),
        (
            "as needed",
            "Ferrules, labels, insulating sleeving, heat shrink",
            "Match terminal and wire sizes; >=80 C insulation; joints strain-relieved",
        ),
    ] {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            quote("custom/supplement"),
            quote(q),
            quote(item),
            quote(notes)
        ));
    }
    fs::write(dir.join("procurement-bom.csv"), csv)?;
    fs::write(dir.join("commissioning-log.csv"),"timestamp,assembly_revision,ambient_c,roof_metal_c,drawer_metal_c,water_corner_1_c,water_corner_2_c,water_corner_3_c,water_corner_4_c,water_center_c,supply_v,total_current_a,drawer_state,notes\n")?;
    Ok(())
}
fn render_docs(folder: &Path, names: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(folder)?;
    let mut pages: Vec<Vec<(String, u32)>> = Vec::new();
    let mut lines = Vec::new();
    let mut height = 0u32;
    for name in names {
        if !lines.is_empty() {
            pages.push(std::mem::take(&mut lines));
            height = 0;
        }
        let text = fs::read_to_string(Path::new("docs/heated-cassette-v0").join(name))?;
        let mut headers: Vec<String> = Vec::new();
        for (line_index, raw) in text.lines().enumerate() {
            let raw = raw.trim();
            if raw.is_empty() {
                if height + 22 > 1300 {
                    pages.push(std::mem::take(&mut lines));
                    height = 0;
                } else {
                    lines.push((String::new(), 15));
                    height += 22;
                }
                continue;
            }
            if raw.starts_with("|---") || raw.starts_with("|--") {
                continue;
            }
            let (body, size) = if raw.starts_with('#') {
                (
                    raw.trim_start_matches('#').trim().to_string(),
                    if raw.starts_with("# ") { 32 } else { 26 },
                )
            } else if raw.starts_with('|') {
                let cells: Vec<String> = raw
                    .trim_matches('|')
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .collect();
                if headers.is_empty() {
                    headers = cells;
                    continue;
                }
                (
                    cells
                        .iter()
                        .enumerate()
                        .map(|(j, s)| {
                            format!(
                                "{}: {}",
                                headers.get(j).map(String::as_str).unwrap_or(""),
                                s
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("; "),
                    20,
                )
            } else {
                headers.clear();
                (raw.replace("**", "").replace('`', ""), 20)
            };
            let wrapped = wrap(&body, if size >= 26 { 65 } else { 96 });
            let keep_next = if size >= 26 {
                text.lines()
                    .skip(line_index + 1)
                    .find(|s| !s.trim().is_empty())
                    .map(|s| wrap(s, 96).len() as u32 * 27 + 44)
                    .unwrap_or(0)
            } else {
                0
            };
            if height + (wrapped.len() as u32 * (size + 7)) + keep_next > 1300 {
                pages.push(std::mem::take(&mut lines));
                height = 0;
            }
            for s in wrapped {
                lines.push((s, size));
                height += size + 7;
            }
        }
    }
    if !lines.is_empty() {
        pages.push(lines);
    }
    let total = pages.len();
    for (idx, page) in pages.into_iter().enumerate() {
        let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"210mm\" height=\"297mm\" viewBox=\"0 0 1200 1697\"><rect width=\"1200\" height=\"1697\" fill=\"white\"/><style>text{font-family:Arial,sans-serif;fill:#163040}</style><text x=\"65\" y=\"65\" font-size=\"18\">LAMINARFORGE / CASSETTE V0 / REV A / BUILD AND COMMISSION</text><path d=\"M65 90 H1135\" stroke=\"#8aa3b2\"/>");
        let mut y = 145;
        for (s, size) in page {
            svg.push_str(&format!(
                "<text x=\"65\" y=\"{y}\" font-size=\"{size}\" font-weight=\"{}\">{}</text>",
                if size >= 26 { "bold" } else { "normal" },
                esc(&s)
            ));
            y += size + 7;
        }
        svg.push_str(&format!("<path d=\"M65 1570 H1135\" stroke=\"#8aa3b2\"/><text x=\"65\" y=\"1610\" font-size=\"18\">Water-test prototype. Read paired shop drawings and verification manifest.</text><text x=\"1040\" y=\"1610\" font-size=\"18\">{} / {total}</text></svg>",idx+1));
        fs::write(folder.join(format!("manual-{:03}.svg", idx + 1)), svg)?;
    }
    Ok(())
}
