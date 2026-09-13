//! Shared local runtime-config publication and deterministic verification.
use clap::Parser;
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use vcad::Part;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub config: PathBuf,
    #[arg(long)]
    pub output_dir: PathBuf,
    /// Verify the existing publication by regenerating exactly the requested geometry.
    #[arg(long)]
    pub verify: bool,
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn mesh(bytes: &[u8]) -> Result<serde_json::Value, String> {
    if bytes.len() < 84 {
        return Err("truncated STL".into());
    }
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    if count == 0 || count.checked_mul(50).and_then(|n| n.checked_add(84)) != Some(bytes.len()) {
        return Err("invalid STL length/triangle count".into());
    }
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for t in bytes[84..].chunks_exact(50) {
        for v in 0..3 {
            for a in 0..3 {
                let o = 12 + v * 12 + a * 4;
                let x = f32::from_le_bytes(t[o..o + 4].try_into().unwrap()) as f64;
                if !x.is_finite() {
                    return Err("nonfinite STL vertex".into());
                }
                low[a] = low[a].min(x);
                high[a] = high[a].max(x);
            }
        }
    }
    let size = std::array::from_fn::<_, 3, _>(|a| high[a] - low[a]);
    if size.iter().any(|x| *x <= 0.0) {
        return Err("degenerate STL envelope".into());
    }
    Ok(serde_json::json!({"triangles":count,"min_mm":low,"max_mm":high,"size_mm":size}))
}

pub fn run<P: DeserializeOwned + Serialize>(
    args: &Args,
    model: &str,
    names: &[&str],
    build: impl Fn(&P, usize) -> Result<Part, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(&args.config)?;
    let p: P = toml::from_str(&raw)?;
    // Validate and construct before invalidating any existing publication.
    let first = build(&p, 0)?;
    fs::create_dir_all(&args.output_dir)?;
    let manifest = args.output_dir.join(format!("{model}.manifest.json"));
    let old: Option<serde_json::Value> = if args.verify {
        Some(serde_json::from_slice(&fs::read(&manifest)?)?)
    } else {
        None
    };
    if !args.verify {
        match fs::remove_file(&manifest) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    let mut outputs = Vec::new();
    let mut first = Some(first);
    for (i, name) in names.iter().enumerate() {
        let part = if i == 0 {
            first.take().unwrap()
        } else {
            build(&p, i)?
        };
        let path = args.output_dir.join(name);
        let bytes = part.to_stl()?;
        if !args.verify {
            fs::write(&path, &bytes)?;
        }
        let stats = mesh(&bytes)?;
        if args.verify && fs::read(&path)? != bytes {
            return Err(format!("geometry mismatch: {name}").into());
        }
        let svg = preview(&bytes)?;
        let preview_path = path.with_extension("preview.svg");
        if args.verify {
            if fs::read_to_string(&preview_path)? != svg {
                return Err("preview differs".into());
            }
        } else {
            fs::write(&preview_path, &svg)?;
        }
        outputs.push(serde_json::json!({"file":name,"bytes":bytes.len(),"sha256":hash(&bytes),"mesh":stats,"preview_sha256":hash(svg.as_bytes())}));
    }
    let evidence = serde_json::json!({"schema_version":1,"model":model,"parameters":p,"config_sha256":hash(raw.as_bytes()),"generator_sha256":hash(&fs::read(std::env::current_exe()?)?),"outputs":outputs,"claim":"Dry engineering geometry only; not manufacturing qualification"});
    if let Some(old) = old {
        if old != evidence {
            return Err(
                "publication evidence differs from configuration, executable or outputs".into(),
            );
        }
    } else {
        fs::write(manifest, serde_json::to_vec_pretty(&evidence)?)?;
    }
    println!(
        "{} {model}: {} checked meshes",
        if args.verify { "Verified" } else { "Generated" },
        names.len()
    );
    Ok(())
}

pub fn preview(bytes: &[u8]) -> Result<String, String> {
    mesh(bytes)?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"420\" viewBox=\"0 0 1200 420\"><rect width=\"1200\" height=\"420\" fill=\"#f7f8fa\"/><text x=\"24\" y=\"30\" font-family=\"sans-serif\" font-size=\"20\">Mechanical prototype mesh</text>");
    for (panel, (a, b, label)) in [
        (0usize, 1usize, "Top · X/Y"),
        (0, 2, "Front · X/Z"),
        (1, 2, "Side · Y/Z"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut vertices = Vec::new();
        for triangle in bytes[84..].chunks_exact(50) {
            let mut points = [[0.0f64; 3]; 3];
            for (vertex, point) in points.iter_mut().enumerate() {
                for (axis, value) in point.iter_mut().enumerate() {
                    let offset = 12 + vertex * 12 + axis * 4;
                    *value =
                        f32::from_le_bytes(triangle[offset..offset + 4].try_into().unwrap()) as f64;
                }
            }
            vertices.push(points);
        }
        let low_x = vertices
            .iter()
            .flatten()
            .map(|p| p[a])
            .fold(f64::INFINITY, f64::min);
        let high_x = vertices
            .iter()
            .flatten()
            .map(|p| p[a])
            .fold(f64::NEG_INFINITY, f64::max);
        let low_y = vertices
            .iter()
            .flatten()
            .map(|p| p[b])
            .fold(f64::INFINITY, f64::min);
        let high_y = vertices
            .iter()
            .flatten()
            .map(|p| p[b])
            .fold(f64::NEG_INFINITY, f64::max);
        let scale = (350.0 / (high_x - low_x)).min(290.0 / (high_y - low_y));
        let center_x = panel as f64 * 400.0 + 200.0;
        svg.push_str(&format!("<text x=\"{}\" y=\"65\" font-family=\"sans-serif\" font-size=\"16\">{label}</text><g fill=\"none\" stroke=\"#254560\" stroke-width=\"0.65\" opacity=\"0.55\">",panel*400+24));
        let mut edges = std::collections::BTreeSet::new();
        for points in vertices {
            for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                let project = |v: [f64; 3]| {
                    ((center_x + (v[a] - (low_x + high_x) / 2.0) * scale) * 100.0).round() as i64
                };
                let y = |v: [f64; 3]| {
                    ((230.0 - (v[b] - (low_y + high_y) / 2.0) * scale) * 100.0).round() as i64
                };
                let mut edge = [
                    (project(points[i]), y(points[i])),
                    (project(points[j]), y(points[j])),
                ];
                edge.sort();
                if edge[0] != edge[1] {
                    edges.insert(edge);
                }
            }
        }
        for [start, end] in edges {
            svg.push_str(&format!(
                "<path d=\"M {} {} L {} {}\"/>",
                start.0 as f64 / 100.0,
                start.1 as f64 / 100.0,
                end.0 as f64 / 100.0,
                end.1 as f64 / 100.0
            ));
        }
        svg.push_str("</g>");
    }
    svg.push_str("<text x=\"24\" y=\"406\" font-family=\"sans-serif\" font-size=\"12\">Orthographic mesh review only · not manufacturing qualification</text></svg>");
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runtime_contracts_reject_missing_unknown_and_nonfinite_values() {
        macro_rules! contract {
            ($kind:ty, $file:literal, $field:ident) => {{
                let raw = include_str!($file);
                let mut p: $kind = toml::from_str(raw).unwrap();
                p.validate().unwrap();
                assert!(toml::from_str::<$kind>("").is_err());
                // Insert at root even when the configuration ends in a material table.
                assert!(toml::from_str::<$kind>(&format!("misspelled = 1\n{raw}")).is_err());
                for value in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
                    p.$field = value;
                    assert!(p.validate().is_err());
                }
            }};
        }
        contract!(
            crate::p0_cartridge_coupons::Parameters,
            "../models/p0_cartridge_coupons.toml",
            coupon_length_mm
        );
        contract!(
            crate::cassette_print_coupons::Parameters,
            "../models/sixteen_slot_cassette_print_coupons.toml",
            chip_clearance_mm
        );
        contract!(
            crate::diagnostic_cartridge_model::Parameters,
            "../models/diagnostic_cartridge.toml",
            body_height_mm
        );
        let raw = include_str!("../models/swab_integrated_sealed_diagnostic_cartridge.toml");
        use crate::swab_integrated_sealed_diagnostic_cartridge::{verify_design, CartridgeParams};
        let mut p: CartridgeParams = toml::from_str(raw).unwrap();
        verify_design(p).unwrap();
        assert!(toml::from_str::<CartridgeParams>("").is_err());
        assert!(toml::from_str::<CartridgeParams>(&format!("unsupported = 1\n{raw}")).is_err());
        p.body_length_mm = 155.0;
        verify_design(p).unwrap();
        p.waste_internal_height_mm = 3.0;
        assert!(verify_design(p).is_err());
    }
    #[test]
    fn malformed_mesh_is_rejected() {
        assert!(mesh(&[]).is_err());
        assert!(mesh(&[0u8; 84]).is_err());
        let mut raw = vec![0; 134];
        raw[80] = 1;
        assert!(mesh(&raw).is_err());
        raw[96..100].copy_from_slice(&f32::NAN.to_le_bytes());
        assert!(mesh(&raw).is_err());
    }
}
