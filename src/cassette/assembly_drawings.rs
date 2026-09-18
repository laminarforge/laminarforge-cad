//! Assembly identification and connection drawings, generated from the actual parts.
use super::{drawings::part_key, Config, Item, Layout};
use base64::Engine;
use std::{collections::BTreeMap, fs, path::Path};

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn page(title: &str) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{{font-family:Arial,sans-serif;fill:#132936;font-size:19px}}</style><rect x=\"25\" y=\"25\" width=\"1550\" height=\"1081\" fill=\"none\" stroke=\"#132936\"/><text x=\"50\" y=\"65\" font-size=\"28\">LF-CAS-V0 / REV C / {title}</text><text x=\"50\" y=\"102\">Units mm | X right, Y rearward, Z up | Assembly coordinates | Do not scale</text>")
}
fn end(mut s: String) -> String {
    s.push_str("<path d=\"M25 1040 H1575\" stroke=\"#132936\"/><text x=\"50\" y=\"1075\">Manual water-test prototype | M-items: custom parts; H-items: owner-supplied hardware | No fabrication PO</text></svg>");
    s
}
pub fn groups<'a>(parts: &'a [Item]) -> Vec<Vec<&'a Item>> {
    let mut g = BTreeMap::<String, Vec<&Item>>::new();
    for i in parts.iter().filter(|i| !i.reference) {
        g.entry(part_key(&i.name)).or_default().push(i);
    }
    g.into_values().collect()
}
pub fn item_id(name: &str, parts: &[Item]) -> String {
    let key = part_key(name);
    let idx = groups(parts)
        .iter()
        .position(|g| part_key(&g[0].name) == key)
        .expect("assembly item missing");
    format!("M{:02}", idx + 1)
}
// Orthographic isometric projection; raster Z-buffer, never a box proxy.
fn proj(v: [f64; 3]) -> [f64; 3] {
    [
        0.85 * v[0] + 0.5 * v[1],
        0.25 * v[0] - 0.425 * v[1] - 0.85 * v[2],
        0.425 * v[0] - 0.7225 * v[1] + 0.48625 * v[2],
    ]
}
fn view(
    svg: &mut String,
    items: &[(&Item, [f64; 3])],
    area: [f64; 4],
) -> Result<Vec<Option<[f64; 2]>>, Box<dyn std::error::Error>> {
    let mut tris = Vec::new();
    let mut bounds = [
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
    ];
    let mut centers = Vec::new();
    for (owner, (i, offset)) in items.iter().enumerate() {
        let mesh = i.part.to_mesh();
        let vv = mesh.vertices();
        let (lo, hi) = i.part.bounding_box();
        centers.push(proj(std::array::from_fn(|k| {
            (lo[k] + hi[k]) / 2.0 + offset[k]
        })));
        for ids in mesh.indices().chunks_exact(3) {
            let xyz: [[f64; 3]; 3] = std::array::from_fn(|j| {
                std::array::from_fn(|k| vv[ids[j] as usize * 3 + k] as f64 + offset[k])
            });
            let t = xyz.map(proj);
            for v in t {
                bounds[0] = bounds[0].min(v[0]);
                bounds[1] = bounds[1].min(v[1]);
                bounds[2] = bounds[2].max(v[0]);
                bounds[3] = bounds[3].max(v[1]);
            }
            let a = glam::DVec3::from_array(xyz[1]) - glam::DVec3::from_array(xyz[0]);
            let b = glam::DVec3::from_array(xyz[2]) - glam::DVec3::from_array(xyz[0]);
            let n = a.cross(b).normalize_or_zero();
            let shade = 0.65 + 0.35 * n.z.abs();
            tris.push((t, i.color.map(|c| (c as f64 * shade) as u8), owner));
        }
    }
    let [px, py, width, height] = area;
    let scale =
        ((width - 20.0) / (bounds[2] - bounds[0])).min((height - 20.0) / (bounds[3] - bounds[1]));
    let ox = width / 2.0 - scale * (bounds[0] + bounds[2]) / 2.0;
    let oy = height / 2.0 - scale * (bounds[1] + bounds[3]) / 2.0;
    let w = (width * 2.0) as usize;
    let h = (height * 2.0) as usize;
    let mut raster = image::RgbImage::from_pixel(w as u32, h as u32, image::Rgb([255, 255, 255]));
    let mut depth = vec![f64::NEG_INFINITY; w * h];
    let mut owners = vec![usize::MAX; w * h];
    for (mut t, c, owner) in tris {
        for v in &mut t {
            v[0] = 2.0 * (ox + v[0] * scale);
            v[1] = 2.0 * (oy + v[1] * scale);
        }
        let ar =
            (t[1][0] - t[0][0]) * (t[2][1] - t[0][1]) - (t[1][1] - t[0][1]) * (t[2][0] - t[0][0]);
        if ar.abs() < 1e-9 {
            continue;
        }
        let xmin = t
            .iter()
            .map(|v| v[0])
            .fold(f64::INFINITY, f64::min)
            .floor()
            .clamp(0.0, (w - 1) as f64) as usize;
        let xmax = t
            .iter()
            .map(|v| v[0])
            .fold(f64::NEG_INFINITY, f64::max)
            .ceil()
            .clamp(0.0, (w - 1) as f64) as usize;
        let ymin = t
            .iter()
            .map(|v| v[1])
            .fold(f64::INFINITY, f64::min)
            .floor()
            .clamp(0.0, (h - 1) as f64) as usize;
        let ymax = t
            .iter()
            .map(|v| v[1])
            .fold(f64::NEG_INFINITY, f64::max)
            .ceil()
            .clamp(0.0, (h - 1) as f64) as usize;
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let xx = x as f64 + 0.5;
                let yy = y as f64 + 0.5;
                let b = ((xx - t[0][0]) * (t[2][1] - t[0][1])
                    - (yy - t[0][1]) * (t[2][0] - t[0][0]))
                    / ar;
                let c1 = ((t[1][0] - t[0][0]) * (yy - t[0][1])
                    - (t[1][1] - t[0][1]) * (xx - t[0][0]))
                    / ar;
                let a = 1.0 - b - c1;
                if a >= -1e-8 && b >= -1e-8 && c1 >= -1e-8 {
                    let z = a * t[0][2] + b * t[1][2] + c1 * t[2][2];
                    let k = y * w + x;
                    if z > depth[k] {
                        depth[k] = z;
                        owners[k] = owner;
                        raster.put_pixel(x as u32, y as u32, image::Rgb(c));
                    }
                }
            }
        }
    }
    let mut png = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(raster).write_to(&mut png, image::ImageFormat::Png)?;
    svg.push_str(&format!("<image x=\"{px}\" y=\"{py}\" width=\"{width}\" height=\"{height}\" href=\"data:image/png;base64,{}\"/>",base64::engine::general_purpose::STANDARD.encode(png.into_inner())));
    // Land leaders on a visible pixel of the named solid, never in a hollow bounding-box center.
    let mut anchors = vec![None; items.len()];
    let mut nearest = vec![f64::INFINITY; items.len()];
    for (pixel, &owner) in owners.iter().enumerate() {
        if owner == usize::MAX {
            continue;
        }
        let x = (pixel % w) as f64 / 2.0;
        let y = (pixel / w) as f64 / 2.0;
        let v = centers[owner];
        let distance = (x - ox - scale * v[0]).powi(2) + (y - oy - scale * v[1]).powi(2);
        if distance < nearest[owner] {
            nearest[owner] = distance;
            anchors[owner] = Some([px + x, py + y]);
        }
    }
    Ok(anchors)
}
fn text(svg: &mut String, x: f64, y: f64, s: &str) {
    svg.push_str(&format!("<text x=\"{x}\" y=\"{y}\">{}</text>", esc(s)));
}
pub fn sheets(
    dir: &Path,
    parts: &[Item],
    p: &Config,
    d: &Layout,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dir)?;
    let mut svg = page("A01 GENERAL ASSEMBLY");
    text(&mut svg, 60.0, 150.0, "CLOSED: seal faces meet at Y=0");
    text(
        &mut svg,
        820.0,
        150.0,
        "OPEN: moving assembly displaced toward -Y",
    );
    view(
        &mut svg,
        &parts.iter().map(|i| (i, [0.0; 3])).collect::<Vec<_>>(),
        [50.0, 175.0, 720.0, 485.0],
    )?;
    view(
        &mut svg,
        &parts
            .iter()
            .map(|i| (i, [0.0, if i.moving { -d.stroke } else { 0.0 }, 0.0]))
            .collect::<Vec<_>>(),
        [815.0, 175.0, 720.0, 485.0],
    )?;
    for (j,s) in [format!("Body envelope: W {:.3}, front flange Y=-{:.3}, rear face Y={:.3}, Z={:.3}..{:.3} (REF).",d.bezel_x,p.flange_thickness,d.rear_y+p.rear_thickness,d.bezel_min_z,d.bezel_max_z),format!("Drawer nominal travel {:.3}; rear engagement {:.3}. Shoulder-stop travel allows pin/slot clearance; see A04.",d.stroke,p.rear_retention),"Bench footprint: both risers supported; hardware extends beyond body. Reserve rear harness space to Y=270.".into(),"Two heaters and two Pt100 sensors: roof exterior and drawer underside. Guards stand 8 mm off each surface.".into(),"Assembly datums: front stop Y=0; X=0 symmetry; housing rail-seat Z=6. General part dimensions after finish.".into(),"A02: all custom parts and quantities. A03: exploded groups with BOM balloons. A04: hardware and connections.".into(),"Plate is an owner-supplied water-test surrogate. Assembly geometry does not establish culture performance.".into()].iter().enumerate(){text(&mut svg,50.0,710.0+j as f64*43.0,s);}
    fs::write(dir.join("A01-general-assembly.svg"), end(svg))?;
    let groups = groups(parts);
    let mut svg = page("A02 CUSTOM PARTS / ASSEMBLY BOM");
    for (x, s) in [
        (50., "ITEM"),
        (135., "QTY"),
        (215., "REPRESENTATIVE STEP / PART"),
        (910., "MATERIAL"),
        (1190., "SUPPLY SCOPE"),
    ] {
        text(&mut svg, x, 158., s);
    }
    let mut csv = String::from("item,quantity,representative_step,material,scope\r\n");
    for (j, g) in groups.iter().enumerate() {
        let name = &g[0].name;
        let id = format!("M{:02}", j + 1);
        let optional = name.starts_with("08_") || name.starts_with("13_");
        let material = if name.starts_with("08_") {
            "A160 guide tape"
        } else if name.starts_with("13_") {
            "302 SS shim reference"
        } else {
            "6061-T6 aluminum"
        };
        let y = 202.0 + j as f64 * 36.0;
        for (x, s) in [
            (50., id.clone()),
            (135., g.len().to_string()),
            (215., format!("{name}.step")),
            (910., material.into()),
            (
                1190.,
                if optional {
                    "Owner supplied"
                } else {
                    "Base shop quote"
                }
                .into(),
            ),
        ] {
            text(&mut svg, x, y, &s);
        }
        csv.push_str(&format!(
            "{id},{},{name}.step,{material},{}\r\n",
            g.len(),
            if optional { "owner" } else { "shop" }
        ));
    }
    text(&mut svg,50.,965.,"One representative file per identical group; do not quote each translated instance as a different design.");
    text(&mut svg,50.,1000.,"13 aluminum designs / 23 physical aluminum pieces per set. M-items remain stable across drawings and CSV.");
    fs::write(dir.join("A02-assembly-bom.svg"), end(svg))?;
    fs::write(dir.parent().unwrap().join("assembly-bom.csv"), csv)?;
    // Separate each mechanism so balloons land on actual visible solids rather than hidden assembly centers.
    for (page_index, moving) in [(0, false), (1, true)] {
        let mut svg = page(&format!(
            "A03{} EXPLODED {} GROUP",
            if moving { "B" } else { "A" },
            if moving { "DRAWER" } else { "FIXED" }
        ));
        let mut entries: Vec<(&Item, [f64; 3])> = Vec::new();
        for i in parts.iter().filter(|i| !i.reference && i.moving == moving) {
            let n = &i.name;
            let o = if n.starts_with("01_") {
                [0., 0., 55.]
            } else if n.starts_with("03_") {
                [0., 100., 20.]
            } else if n.starts_with("05_") {
                [0., -65., 0.]
            } else if n.starts_with("06_") {
                [0., 0., 65.]
            } else if n.starts_with("07_") {
                [if n.ends_with("left") { -35. } else { 35. }, 0., -25.]
            } else if n.starts_with("08_") {
                [
                    if n.contains("left") { -70. } else { 70. },
                    0.,
                    if n.contains("upper") { 15. } else { -5. },
                ]
            } else if n.starts_with("10_") {
                [if n.ends_with("left") { -35. } else { 35. }, 0., -85.]
            } else if n.starts_with("11_") {
                [0., 0., if moving { -40. } else { 105. }]
            } else if n.starts_with("12_") {
                [0., 0., if moving { -95. } else { 155. }]
            } else if n.starts_with("13_") {
                [0., -75., 55.]
            } else if n.starts_with("14_") {
                [0., 105., -40.]
            } else if n.starts_with("15_") {
                [45., 105., -40.]
            } else {
                [0.; 3]
            };
            entries.push((i, o));
        }
        let anchors = view(&mut svg, &entries, [260., 150., 1050., 775.])?;
        let mut unique = BTreeMap::<String, usize>::new();
        for (j, (i, _)) in entries.iter().enumerate() {
            unique.entry(item_id(&i.name, parts)).or_insert(j);
        }
        let n = unique.len();
        for (j, (id, k)) in unique.iter().enumerate() {
            let left = j < n.div_ceil(2);
            let row = if left { j } else { j - n.div_ceil(2) };
            let x = if left { 130. } else { 1450. };
            let y = 185. + row as f64 * 98.;
            let a = anchors[*k].ok_or_else(|| {
                format!(
                    "exploded assembly hides ballooned part {}",
                    entries[*k].0.name
                )
            })?;
            svg.push_str(&format!("<path d=\"M{} {y} L{} {y} L{} {}\" stroke=\"#345\" fill=\"none\"/><circle cx=\"{x}\" cy=\"{y}\" r=\"26\" fill=\"white\" stroke=\"#345\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\">{id}</text><circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"#345\"/>",if left{x+26.}else{x-26.},if left{245.}else{1330.},a[0],a[1],y+7.,a[0],a[1]));
        }
        text(&mut svg,50.,970.,"Exploded offsets are illustrative only; all STEP files use the closed assembly coordinates. See A02 for quantities.");
        text(&mut svg,50.,1005.,"Duplicate spacers/guards/risers/jaws share one item number. Hardware connections and thermal parts: A04.");
        fs::write(
            dir.join(format!("A03-{}-exploded.svg", page_index + 1)),
            end(svg),
        )?;
    }
    hardware(dir, parts, d)?;
    Ok(())
}
fn hardware(dir: &Path, parts: &[Item], d: &Layout) -> Result<(), Box<dyn std::error::Error>> {
    let id = |n: &str| item_id(n, parts);
    let mut svg = page("A04 MECHANICAL CONNECTIONS / OWNER HARDWARE");
    let rows = vec![
        (
            "H01",
            "6",
            "M3x40 socket cap",
            format!(
                "Risers + rails into housing {}; heads in riser counterbores",
                id("01_fixed_U_housing")
            ),
        ),
        (
            "H02",
            "4",
            "M3x12 socket cap",
            format!(
                "Rear {} into housing {}",
                id("03_rear_cover"),
                id("01_fixed_U_housing")
            ),
        ),
        (
            "H03",
            "3",
            "M3x14 socket cap",
            format!(
                "Flange {} into drawer {}",
                id("05_drawer_front_flange"),
                id("04_heated_drawer")
            ),
        ),
        (
            "H04",
            "8",
            "M3x14 socket cap",
            "Two guards through eight spacers into roof/drawer".into(),
        ),
        (
            "H05",
            "4",
            "M2x5 socket cap",
            format!(
                "Nest {} into drawer {}",
                id("06_replaceable_nest"),
                id("04_heated_drawer")
            ),
        ),
        (
            "H06",
            "2",
            "M2x4 socket cap",
            "Shim ears into housing; flange has head-relief pockets".into(),
        ),
        (
            "H07",
            "4",
            "M3x8 socket cap",
            "Fixed bracket into rear; moving bracket into drawer".into(),
        ),
        (
            "H08",
            "4",
            "M2x12 socket cap",
            "Two jaws into bracket split faces; grip jackets evenly".into(),
        ),
        (
            "H09",
            "2",
            "M3x6 + flat washer",
            "One Sensata 67L050 thermostat per zone; inspect tip clearance".into(),
        ),
        (
            "H10",
            "1",
            "Accu SKH-M2-10-A2",
            "D3 x 10 shoulder; M2 thread into housing stop receiver".into(),
        ),
        (
            "H11",
            "2",
            "Elesa 6342 / M4x16",
            "Closure knobs through flange into housing; stop faces meet".into(),
        ),
        (
            "H12",
            "1",
            "M5x8 set screw",
            "Close flange probe port when unused; remove for probe sleeve".into(),
        ),
        (
            "H13",
            "1",
            "PTFE tube 4 OD / 2 ID",
            "10 mm long probe-wire sleeve; alternative to H12".into(),
        ),
        (
            "H14",
            "2",
            "Keenovo 12073 / 24 V 60 W",
            "One 100 x 100 pad per zone; bond lands on detail sheet".into(),
        ),
        (
            "H15",
            "2",
            "Omega SA1-RTD / Pt100",
            "One directly bonded RTD per zone; separate from heater".into(),
        ),
        (
            "H16",
            "2",
            "Sensata 67L050 NC",
            "One thermal cutoff per zone; H09 retention".into(),
        ),
        (
            "H17",
            "1",
            "BF-1000 + 9731 PSA",
            "Continuous face ring; measured shim stack; 20-30% compression".into(),
        ),
        (
            "H18",
            "1+1",
            "Greiner 655101 / 656101",
            "Owner water-test plate/lid; reference envelope only".into(),
        ),
    ];
    for (x, s) in [
        (50., "ITEM"),
        (130., "QTY"),
        (220., "HARDWARE / COMPONENT"),
        (745., "CONNECTION / LOCATION"),
    ] {
        text(&mut svg, x, 160., s);
    }
    for (j, (n, q, item, note)) in rows.iter().enumerate() {
        let y = 202. + j as f64 * 39.;
        for (x, s) in [(50., *n), (130., *q), (220., *item), (745., note.as_str())] {
            text(&mut svg, x, y, s);
        }
    }
    text(&mut svg,50.,945.,&format!("Stop: slot centerline X={:.3}; closed end Y=16; center spacing {:.3}. H10 shoulder seats on guide roof.",d.stop_x,d.stroke));
    text(&mut svg,50.,980.,"ISO 4762 A2-70 ordinary screws. Start M3 at <=0.3 N m, M2 <=0.08 N m; check free travel and no bottoming.");
    text(&mut svg,50.,1015.,"See assembly handbook for wiring, cable loops, adhesive preparation, measured gasket stack and commissioning.");
    fs::write(dir.join("A04-hardware-connections.svg"), end(svg))?;
    Ok(())
}
