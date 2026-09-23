//! Feature leaders on enlarged part views. Values are derived from the same layout as CAD.
use super::{Config, Item, Layout};
use std::{fs, path::Path};
struct Note {
    point: [f64; 2],
    lines: Vec<String>,
}
fn note(point: [f64; 2], lines: Vec<String>) -> Note {
    Note { point, lines }
}
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
pub fn extent(name: &str, axis: usize, value: f64) -> String {
    if name.starts_with("08_") || name.starts_with("13_") {
        return format!("{value:.3} REF");
    }
    let tol = if (axis == 2
        && (name.starts_with("04_") || name.starts_with("07_") || name == "01_roof_plate"))
        || (axis == 1 && name == "01_front_bezel")
        || (axis == 0 && name.ends_with("side_plate"))
    {
        "+/-0.05"
    } else {
        "+/-0.10"
    };
    format!("{value:.3} {tol}")
}
pub fn sheets(
    dir: &Path,
    i: &Item,
    p: &Config,
    d: &Layout,
    source: &str,
    bounds: [[f64; 3]; 2],
) -> Result<(), Box<dyn std::error::Error>> {
    let n = &i.name;
    let a = d.drawer_y - 12.;
    let shim_top = d.gasket_center_z + d.gasket_outer_z / 2.;
    let mut pages: Vec<(usize, &str, Vec<Note>)> = Vec::new();
    if n == "01_front_bezel" {
        pages.push((
            1,
            "FRONT SEAL AND OPENING",
            vec![
                note(
                    [d.bezel_x / 2. - 2., d.bezel_max_z - 2.],
                    vec![
                        "A: front stop face Y=0".into(),
                        "FLATNESS 0.10; Ra 3.2 max".into(),
                        "Bezel thickness 6.00 +/-0.05".into(),
                    ],
                ),
                note(
                    [d.opening_x / 2. - 10., d.opening_max_z],
                    vec![
                        format!(
                            "Opening {:.3} W x {:.3} H +/-0.10",
                            d.opening_x,
                            d.opening_max_z - d.opening_min_z
                        ),
                        format!("Centered X=0; lower Z={:.3}", d.opening_min_z),
                        "4x D2 corner relief at opening corners".into(),
                    ],
                ),
                note(
                    [d.gasket_outer_x / 2. + 0.25, d.gasket_center_z],
                    vec![
                        format!(
                            "Groove outer {:.3} W x {:.3} H +/-0.10",
                            d.gasket_outer_x + 0.5,
                            d.gasket_outer_z + 0.5
                        ),
                        format!("Center X=0 / Z={:.3} +/-0.10", d.gasket_center_z),
                        format!("Outer corner R{:.2} +/-0.10", p.gasket_outer_radius + 0.25),
                    ],
                ),
                note(
                    [
                        d.gasket_outer_x / 2. - 12.,
                        d.gasket_center_z - d.gasket_outer_z / 2. + 1.5,
                    ],
                    vec![
                        "Groove radial width 3.50 +0.10/0".into(),
                        "Depth from A 2.50 +/-0.05".into(),
                        "Flat groove floor; Ra 3.2 max".into(),
                    ],
                ),
                note(
                    [50., shim_top + 3.],
                    vec![
                        "2x ear pockets 6.40 W x 7.40 H".into(),
                        format!("Centers X=+/-50; Z={:.3}", shim_top + 3.),
                        "R1 corners; depth 2.50 +/-0.05".into(),
                    ],
                ),
            ],
        ));
        pages.push((
            1,
            "BEZEL JOINT HOLES",
            vec![
                note(
                    [
                        d.outer_x / 2. - p.joints.front_edge,
                        p.joints.front_screw_z[1],
                    ],
                    vec![
                        "4x D3.4 THRU with D6 +0.10/0 CB".into(),
                        "Counterbore 3.20 +0.05/0 from front".into(),
                        "Coordinates +/-0.025; see hole table".into(),
                    ],
                ),
                note(
                    [d.outer_x / 2. - p.joints.front_edge, p.joints.front_pin_z],
                    vec![
                        "2x D3.15 +0.02/0 slip bore THRU".into(),
                        "Coordinates +/-0.025; see hole table".into(),
                        "Pins and screw heads below seal face".into(),
                    ],
                ),
                note(
                    [
                        d.outer_x / 2. - p.joints.front_edge - 3.,
                        p.joints.front_screw_z[0],
                    ],
                    vec![
                        "Do not break into gasket groove".into(),
                        format!(
                            "Nominal groove land {:.2}",
                            d.outer_x / 2.
                                - p.joints.front_edge
                                - 3.
                                - (d.gasket_outer_x + 0.5) / 2.
                        ),
                        "Bezel finished thickness 6.00 +/-0.05".into(),
                    ],
                ),
            ],
        ));
    } else if n == "01_roof_plate" {
        pages.push((
            0,
            "ROOF JOINTS AND MATING LANDS",
            vec![
                note(
                    [
                        d.outer_x / 2. - p.joints.roof_edge,
                        p.joints.roof_screw_y[0],
                    ],
                    vec![
                        "6x D3.4 THRU; M3x14 heads on top".into(),
                        "Hole schedule gives assembly X/Y".into(),
                        "Joint coordinates +/-0.025".into(),
                    ],
                ),
                note(
                    [d.outer_x / 2. - p.joints.roof_edge, p.joints.roof_pin_y[1]],
                    vec![
                        "4x D3.15 +0.02/0 THRU".into(),
                        "Slip fit to D3 m6 locating pins".into(),
                        "Joint coordinates +/-0.025".into(),
                    ],
                ),
                note(
                    [d.outer_x / 2. - 2., d.rear_y / 2.],
                    vec![
                        "Underside side-contact lands".into(),
                        "FLATNESS / COPLANARITY 0.05".into(),
                        "Plate thickness 6.00 +/-0.05".into(),
                    ],
                ),
            ],
        ));
    } else if n.ends_with("side_plate") {
        pages.push((
            2,
            "SIDE PLATE GUIDE AND JOINT DATUMS",
            vec![
                note(
                    [d.rear_y / 2., p.guide_rail_thickness],
                    vec![
                        "B: rail seat FLATNESS 0.05".into(),
                        format!(
                            "Guide roof above B: {:.3} +/-0.05",
                            d.guide_ceiling - p.guide_rail_thickness
                        ),
                        "Guide roof FLATNESS 0.05".into(),
                    ],
                ),
                note(
                    [d.rear_y / 2., d.roof_bottom],
                    vec![
                        "Roof contact land FLATNESS 0.05".into(),
                        "Front face perpendicular within 0.05".into(),
                        format!("Thickness {:.3} +/-0.05", p.side_wall),
                    ],
                ),
                note(
                    [p.joints.roof_pin_y[0], d.roof_bottom - 3.],
                    vec![
                        "2x roof + 1x front D3 H7 bore".into(),
                        "Depth 6.00 +0.10/0 from joint face".into(),
                        "Ream after coating; coordinate +/-0.025".into(),
                    ],
                ),
                note(
                    [p.bezel_thickness + 5., p.joints.front_screw_z[1]],
                    vec![
                        "2x front + 3x roof M3x0.5-6H".into(),
                        "Full thread: front 8 min, roof 9 min".into(),
                        "Drill 11 deep; coordinates +/-0.025".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("04_") {
        pages.push((
            0,
            "DRAWER SLOT AND CONTACT FACE",
            vec![
                note(
                    [0., d.plate_center_y],
                    vec![
                        format!("Plate-support top Z={:.3}", d.drawer_top),
                        "FLATNESS 0.10; Ra 3.2 max".into(),
                        "Thickness 6.00 +/-0.05".into(),
                    ],
                ),
                note(
                    [d.stop_x, p.stop_y],
                    vec![
                        format!("Slot center X={:.3} +/-0.10", d.stop_x),
                        format!("Front end center Y={:.3} +/-0.10", p.stop_y),
                        "Width 3.40 +0.10/0 THRU".into(),
                    ],
                ),
                note(
                    [d.stop_x, p.stop_y + d.stroke],
                    vec![
                        format!("End center spacing {:.3} +/-0.10", d.stroke),
                        "Semicircular ends R1.70 nominal".into(),
                        "Closed seal position set by flange stop".into(),
                    ],
                ),
                note(
                    [d.drawer_x / 2., d.drawer_y / 2.],
                    vec![
                        format!("Width {:.3} +/-0.10", d.drawer_x),
                        "Edge sliding faces FLATNESS 0.05".into(),
                        "No coating buildup/burrs on edges".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("05_") {
        pages.push((
            1,
            "FLANGE MATING FACE",
            vec![
                note(
                    [d.bezel_x / 2. - 4., d.bezel_min_z + 4.],
                    vec![
                        "Rear face Y=0: FLATNESS 0.10".into(),
                        "Ra 3.2 max; continuous gasket contact".into(),
                        "No scratches crossing seal path".into(),
                    ],
                ),
                note(
                    [-50., shim_top + 3.5],
                    vec![
                        "2x D4.60 +0.10/0 head pockets".into(),
                        "Depth 1.50 +/-0.10 from rear Y=0".into(),
                        "Centers and port: hole schedule".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("06_") {
        pages.push((
            0,
            "NEST OPENING",
            vec![
                note(
                    [0., d.plate_center_y - p.plate_y / 2. - p.plate_clearance],
                    vec![
                        format!(
                            "Opening {:.3} W x {:.3} L +/-0.10",
                            p.plate_x + 2. * p.plate_clearance,
                            p.plate_y + 2. * p.plate_clearance
                        ),
                        format!("Center X=0 / Y={:.3} +/-0.10", d.plate_center_y),
                        "Through 2.00 +/-0.10 thickness".into(),
                    ],
                ),
                note(
                    [
                        p.plate_x / 2. + p.plate_clearance,
                        d.plate_center_y + p.plate_y / 2. + p.plate_clearance,
                    ],
                    vec![
                        "4x D2.00 +/-0.10 corner relief THRU".into(),
                        "Centers at opening rectangle corners".into(),
                        "Preserve remaining web; gently deburr".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("07_") {
        pages.push((
            0,
            "RAIL SEATING FACE",
            vec![note(
                [(bounds[0][0] + bounds[1][0]) / 2., d.rear_y / 2.],
                vec![
                    "Top face Z=6.000: FLATNESS 0.05".into(),
                    "Thickness 6.00 +/-0.05 after finish".into(),
                    "Ra 3.2 max; no raised burrs".into(),
                ],
            )],
        ));
    } else if n.starts_with("10_") {
        pages.push((
            2,
            "BENCH RISER",
            vec![
                note(
                    [d.rear_y / 2., -30.],
                    vec![
                        "Bottom face Z=-30: FLATNESS 0.10".into(),
                        "Assemble pair coplanar within 0.10".into(),
                        "Support both full lengths on bench".into(),
                    ],
                ),
                note(
                    [p.bezel_thickness + 20., -28.4],
                    vec![
                        "3x D6.00 +0.10/0 counterbores".into(),
                        "Depth 3.20 +/-0.10 from underside".into(),
                        "D3.40 +0.10/0 THRU; hole schedule".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("11_") {
        pages.push((
            1,
            "SPACER",
            vec![note(
                [
                    (bounds[0][0] + bounds[1][0]) / 2.,
                    (bounds[0][2] + bounds[1][2]) / 2.,
                ],
                vec![
                    "OD6.00 +/-0.10; D3.40 +0.10/0 THRU".into(),
                    "Length 8.00 +/-0.10; ends square".into(),
                    "Plain 6061 tube cut and faced allowed".into(),
                ],
            )],
        ));
    } else if n.starts_with("12_") {
        pages.push((
            0,
            "GUARD LEAD EXIT",
            vec![
                note(
                    [-35., d.drawer_y - 22.],
                    vec![
                        "Rear notch X=-48 to -22".into(),
                        format!("Y={:.3} to rear edge", d.drawer_y - 22.),
                        "26.00 W x 22.00 deep +/-0.10".into(),
                    ],
                ),
                note(
                    [-48., d.drawer_y - 22.],
                    vec![
                        "Cable-contact edges R0.5 minimum".into(),
                        "1.00 +/-0.10 sheet; 2 identical guards".into(),
                        "No sharp or burred lead-exit edges".into(),
                    ],
                ),
            ],
        ));
    } else if n.starts_with("14_") || n.starts_with("15_") {
        let moving = n.contains("moving");
        let c = if moving { -35. } else { 35. };
        let is_jaw = n.starts_with("15_");
        let mut notes = vec![
            note(
                [c, -18.],
                vec![
                    "Lower cable half-groove D5.80 +/-0.10".into(),
                    format!("Axis Y; center X={c:.3}, Z=-18.000"),
                    "Cable entry/exit edges R0.5 minimum".into(),
                ],
            ),
            note(
                [c, -9.],
                vec![
                    "Upper cable half-groove D5.30 +/-0.10".into(),
                    format!("Axis Y; center X={c:.3}, Z=-9.000"),
                    format!("Both grooves Y={:.3}..{:.3}", a - 10., a + 10.),
                ],
            ),
            note(
                [if is_jaw { c + 8. } else { c - 8. }, -24.],
                vec![
                    format!(
                        "Clamp {} 8.00 W x 20.00 L x 21.00 H",
                        if is_jaw { "jaw" } else { "base" }
                    ),
                    format!(
                        "X={:.3}..{:.3}; Z=-24..-3",
                        if is_jaw { c } else { c - 8. },
                        if is_jaw { c + 8. } else { c }
                    ),
                    "All profile sizes +/-0.10".into(),
                ],
            ),
        ];
        if !is_jaw {
            notes.push(note(
                [
                    if moving { -42. } else { 42. },
                    if moving { d.drawer_bottom } else { 0. },
                ],
                vec![
                    format!(
                        "Mount flange 24.00 W x {:.2} L x 4.00 H",
                        if moving { 10. } else { 6. }
                    ),
                    format!(
                        "X={:.3}..{:.3}; top Z={:.3}",
                        if moving { -47. } else { 23. },
                        if moving { -23. } else { 47. },
                        if moving { d.drawer_bottom } else { 0. }
                    ),
                    format!(
                        "Mount Y={:.3}..{:.3}",
                        if moving { a - 5. } else { d.rear_y },
                        if moving { a + 5. } else { d.rear_y + 6. }
                    ),
                ],
            ));
        }
        pages.push((1, "CABLE CLAMP PROFILE", notes));
        if !is_jaw {
            let notes = if moving {
                vec![
                    note(
                        [a, -10.],
                        vec![
                            "Web X=-47..-39; width 8.00".into(),
                            format!("Y={:.3}..{:.3}; length 10.00", a - 5., a + 5.),
                            format!("Z=-24.000..{:.3}", d.drawer_bottom - 4.),
                        ],
                    ),
                    note(
                        [a, d.drawer_bottom - 2.],
                        vec![
                            "Mount is 4.00 thick; sizes +/-0.10".into(),
                            "Two D3.40 mounting bores THRU".into(),
                            "Split-face taps and bores: hole schedule".into(),
                        ],
                    ),
                ]
            } else {
                vec![
                    note(
                        [d.rear_y + 3., -6.],
                        vec![
                            "Rear web X=23..31; width 8.00".into(),
                            format!("Y={:.3}..{:.3}", d.rear_y, d.rear_y + 6.),
                            "Z=-8.000..0.000; sizes +/-0.10".into(),
                        ],
                    ),
                    note(
                        [(a - 5. + d.rear_y + 6.) / 2., -6.],
                        vec![
                            "Cantilever X=23..29; width 6.00".into(),
                            format!("Y={:.3}..{:.3}", a - 5., d.rear_y + 6.),
                            "Z=-8.000..-4.000; sizes +/-0.10".into(),
                        ],
                    ),
                ]
            };
            pages.push((2, "BRACKET SIDE PROFILE", notes));
        }
    }
    // Each feature sheet reuses the same projected analytic-review mesh as its paired part sheet.
    let images: Vec<&str> = source
        .match_indices("<image ")
        .map(|(idx, _)| {
            let end = source[idx..].find("/>").unwrap() + idx + 2;
            &source[idx..end]
        })
        .collect();
    for (sheet, (view, title, mut notes)) in pages.into_iter().enumerate() {
        let axes = match view {
            0 => [0, 1],
            1 => [0, 2],
            _ => [1, 2],
        };
        let [aa, bb] = axes;
        let scale =
            (430.0 / (bounds[1][aa] - bounds[0][aa])).min(310.0 / (bounds[1][bb] - bounds[0][bb]));
        let ox = 240. - scale * (bounds[0][aa] + bounds[1][aa]) / 2.;
        let oy = 190. + scale * (bounds[0][bb] + bounds[1][bb]) / 2.;
        let original_x = 45. + view as f64 * 510.;
        let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420mm\" height=\"297mm\" viewBox=\"0 0 1600 1131\"><rect width=\"1600\" height=\"1131\" fill=\"white\"/><style>text{{font-family:Arial,sans-serif;fill:#132936;font-size:19px}}</style><rect x=\"25\" y=\"25\" width=\"1550\" height=\"1081\" fill=\"none\" stroke=\"#132936\"/><text x=\"50\" y=\"65\" font-size=\"26\">LF-CAS-V0 / {n} / REV D / DETAIL {}</text><text x=\"50\" y=\"105\">{title} | Units mm | Finished after coating | Do not scale</text><svg x=\"45\" y=\"175\" width=\"864\" height=\"684\" viewBox=\"{original_x} 152 480 380\">{}</svg>",sheet+1,images[view]);
        notes.sort_by(|a, b| b.point[1].total_cmp(&a.point[1]));
        for (j, note) in notes.iter().enumerate() {
            let hs = if n.starts_with("05_") && view == 1 {
                -1.0
            } else {
                1.0
            };
            let x = 45. + 1.8 * (ox + hs * scale * note.point[0]);
            let y = 175. + 1.8 * (oy - scale * note.point[1]);
            let ly = 175. + j as f64 * 150.;
            svg.push_str(&format!("<path d=\"M{x} {y} L925 {} L955 {}\" stroke=\"#087482\" fill=\"none\"/><circle cx=\"{x}\" cy=\"{y}\" r=\"5\" fill=\"#087482\"/>",ly+20.,ly+20.));
            for (k, line) in note.lines.iter().enumerate() {
                svg.push_str(&format!(
                    "<text x=\"970\" y=\"{}\">{}</text>",
                    ly + k as f64 * 29.,
                    esc(line)
                ));
            }
        }
        svg.push_str("<text x=\"50\" y=\"960\">Feature locations use the common assembly coordinates shown on the orthographic and hole-schedule sheets.</text><text x=\"50\" y=\"997\">General linear +/-0.10, clearance bores +0.10/0, angles +/-0.5 deg; explicit feature tolerances override general.</text><path d=\"M25 1040 H1575\" stroke=\"#132936\"/><text x=\"50\" y=\"1075\">Paired part outline, hole schedule and detail sheets jointly define this part. Report any conflict before machining.</text></svg>");
        fs::write(dir.join(format!("{n}-detail-{}.svg", sheet + 1)), svg)?;
    }
    Ok(())
}
