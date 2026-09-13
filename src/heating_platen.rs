use crate::*;
use serde::{Deserialize, Serialize};
use vcad::{centered_cube, centered_cylinder, Part};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub block_height_mm: f64,
    pub pocket_depth_mm: f64,
    pub clearance_x_mm: f64,
    pub clearance_y_mm: f64,
    pub heater_bore_diameter_mm: f64,
}

impl Parameters {
    pub fn validate(&self) -> Result<(), String> {
        for (name, value, low, high) in [
            ("block_height_mm", self.block_height_mm, 16.0, 24.0),
            ("pocket_depth_mm", self.pocket_depth_mm, 0.8, 2.0),
            ("clearance_x_mm", self.clearance_x_mm, 0.4, 1.2),
            ("clearance_y_mm", self.clearance_y_mm, 0.4, 1.2),
            (
                "heater_bore_diameter_mm",
                self.heater_bore_diameter_mm,
                6.0,
                6.4,
            ),
        ] {
            if !value.is_finite() || !(low..=high).contains(&value) {
                return Err(format!("{name} must be finite and within {low}..={high} mm for this fixed cartridge interface"));
            }
        }
        Ok(())
    }
}

pub fn build(p: &Parameters) -> Result<Part, String> {
    p.validate()?;
    let body = centered_cube(
        "cartridge_heat_platen_body",
        BLOCK_LENGTH,
        BLOCK_WIDTH,
        p.block_height_mm,
    );

    let top_z = p.block_height_mm / 2.0;

    let cartridge_pocket = centered_cube(
        "cartridge_shallow_registration_pocket",
        CARTRIDGE_LENGTH + p.clearance_x_mm,
        CARTRIDGE_WIDTH + p.clearance_y_mm,
        p.pocket_depth_mm + 0.2,
    )
    .translate(0.0, 0.0, top_z - p.pocket_depth_mm / 2.0 + 0.1);

    let thermal_pad_recess = centered_cube(
        "bottom_film_thermal_pad_recess",
        HEATER_ZONE_LENGTH,
        HEATER_ZONE_WIDTH,
        BLOCK_THERMAL_PAD_RECESS_DEPTH + 0.2,
    )
    .translate(
        0.0,
        REACTION_CHAMBER_CENTER_Y,
        top_z - BLOCK_THERMAL_PAD_RECESS_DEPTH / 2.0 + 0.12,
    );

    let heater_z = -(p.block_height_mm / 2.0) + HEATER_BORE_Z_OFFSET;
    let heater_center_x = -(BLOCK_LENGTH / 2.0) + HEATER_BORE_DEPTH / 2.0;
    let heater_bore = centered_cylinder(
        "cartridge_heater_bore",
        p.heater_bore_diameter_mm / 2.0,
        HEATER_BORE_DEPTH + 1.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(heater_center_x, REACTION_CHAMBER_CENTER_Y, heater_z);

    let therm_center_y = -(BLOCK_WIDTH / 2.0) + THERMISTOR_BORE_DEPTH / 2.0;
    let therm_pocket = centered_cylinder(
        "cartridge_thermistor_pocket",
        THERMISTOR_BORE_DIAMETER / 2.0,
        THERMISTOR_BORE_DEPTH + 1.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, heater_z);

    let mut cutouts = cartridge_pocket + thermal_pad_recess + heater_bore + therm_pocket;

    for (i, &(mx, my)) in mount_positions().iter().enumerate() {
        let hole = centered_cylinder(
            format!("platen_m3_mount_{i}"),
            BLOCK_MOUNT_HOLE_DIAMETER / 2.0,
            p.block_height_mm + 2.0,
            24,
        )
        .translate(mx, my, 0.0);
        cutouts = cutouts + hole;
    }

    for (i, &(px, py)) in alignment_pin_positions().iter().enumerate() {
        let pin_hole = centered_cylinder(
            format!("cartridge_alignment_pin_socket_{i}"),
            CARTRIDGE_ALIGNMENT_HOLE_DIAMETER / 2.0,
            5.0,
            24,
        )
        .translate(px, py, top_z - 2.2);
        cutouts = cutouts + pin_hole;
    }

    let platen = body - cutouts + side_rails(p) + front_stop(p) + clamp_lands(p);

    Ok(platen)
}

fn mount_positions() -> [(f64, f64); 4] {
    [
        (-BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
    ]
}

fn alignment_pin_positions() -> [(f64, f64); 4] {
    [
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
    ]
}

fn side_rails(p: &Parameters) -> Part {
    let rail_z = p.block_height_mm / 2.0 + CARTRIDGE_RAIL_HEIGHT / 2.0;
    let left = centered_cube(
        "left_cartridge_side_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(
        0.0,
        -(CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH / 2.0 + 1.0),
        rail_z,
    );
    let right = centered_cube(
        "right_cartridge_side_rail",
        CARTRIDGE_LENGTH + 8.0,
        CARTRIDGE_RAIL_WIDTH,
        CARTRIDGE_RAIL_HEIGHT,
    )
    .translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH / 2.0 + 1.0,
        rail_z,
    );
    left + right
}

fn front_stop(p: &Parameters) -> Part {
    centered_cube(
        "front_cartridge_insertion_stop",
        CARTRIDGE_LENGTH + 8.0,
        3.0,
        CARTRIDGE_INSERTION_STOP_HEIGHT,
    )
    .translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH + 2.0,
        p.block_height_mm / 2.0 + CARTRIDGE_INSERTION_STOP_HEIGHT / 2.0,
    )
}

fn clamp_lands(p: &Parameters) -> Part {
    let land_z = p.block_height_mm / 2.0 + 1.0;
    let rear = centered_cube("rear_lid_clamp_land", CARTRIDGE_LENGTH + 6.0, 3.0, 2.0).translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 - 2.0,
        land_z,
    );
    let front = centered_cube("front_lid_clamp_land", CARTRIDGE_LENGTH + 6.0, 3.0, 2.0).translate(
        0.0,
        -CARTRIDGE_WIDTH / 2.0 + 2.0,
        land_z,
    );
    rear + front
}

#[derive(clap::Parser)]
pub struct Args {
    #[arg(long, default_value = "models/heating_block.toml")]
    pub config: std::path::PathBuf,
    #[arg(long, default_value = "output")]
    pub output_dir: std::path::PathBuf,
}

impl Args {
    pub fn load(&self) -> Result<(Parameters, String), Box<dyn std::error::Error>> {
        use sha2::{Digest, Sha256};
        let bytes = std::fs::read(&self.config)?;
        let parameters: Parameters = toml::from_str(std::str::from_utf8(&bytes)?)?;
        parameters.validate()?;
        Ok((parameters, format!("{:x}", Sha256::digest(bytes))))
    }
}

pub fn verify_mesh(bytes: &[u8], p: &Parameters) -> Result<serde_json::Value, String> {
    p.validate()?;
    if bytes.len() < 84 {
        return Err("truncated binary STL".into());
    }
    let triangles = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    if triangles == 0 || bytes.len() != 84 + triangles * 50 {
        return Err("empty or malformed binary STL triangle payload".into());
    }
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for triangle in bytes[84..].chunks_exact(50) {
        for (i, coordinate) in triangle[..48].chunks_exact(4).enumerate() {
            let value = f32::from_le_bytes(coordinate.try_into().unwrap()) as f64;
            if !value.is_finite() {
                return Err("non-finite STL coordinate or normal".into());
            }
            if i >= 3 {
                let axis = i % 3;
                min[axis] = min[axis].min(value);
                max[axis] = max[axis].max(value);
            }
        }
    }
    let expected_min = [
        -BLOCK_LENGTH / 2.0,
        (-BLOCK_WIDTH / 2.0).min(-CARTRIDGE_WIDTH / 2.0 - CARTRIDGE_RAIL_WIDTH - 1.0),
        -p.block_height_mm / 2.0,
    ];
    let expected_max = [
        BLOCK_LENGTH / 2.0,
        (BLOCK_WIDTH / 2.0).max(CARTRIDGE_WIDTH / 2.0 + CARTRIDGE_RAIL_WIDTH + 3.5),
        p.block_height_mm / 2.0
            + CARTRIDGE_RAIL_HEIGHT
                .max(CARTRIDGE_INSERTION_STOP_HEIGHT)
                .max(2.0),
    ];
    for axis in 0..3 {
        if (min[axis] - expected_min[axis]).abs() > 0.01
            || (max[axis] - expected_max[axis]).abs() > 0.01
        {
            return Err(format!(
                "STL envelope mismatch on axis {axis}: {}..{}, expected {}..{}",
                min[axis], max[axis], expected_min[axis], expected_max[axis]
            ));
        }
    }
    Ok(serde_json::json!({"triangles":triangles,"min_mm":min,"max_mm":max}))
}

pub fn publication(
    p: &Parameters,
    config_hash: &str,
    bytes: &[u8],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    use sha2::{Digest, Sha256};
    Ok(serde_json::json!({
        "schema_version":1,"model":"heating_block","parameters":p,
        "config_sha256":config_hash,
        "generator_sha256":format!("{:x}",Sha256::digest(std::fs::read(std::env::current_exe()?)?)),
        "stl_sha256":format!("{:x}",Sha256::digest(bytes)),
        "mesh":verify_mesh(bytes,p)?,
        "claim":"Mechanical prototype geometry; no thermal or manufacturing qualification"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heating_runtime_contract_preserves_nominal_and_rejects_invalid_parameters() {
        let raw = include_str!("../models/heating_block.toml");
        let p: Parameters = toml::from_str(raw).unwrap();
        p.validate().unwrap();
        assert_eq!(p.block_height_mm, BLOCK_HEIGHT);
        assert_eq!(p.pocket_depth_mm, BLOCK_CARTRIDGE_POCKET_DEPTH);
        assert_eq!(p.clearance_x_mm, CARTRIDGE_CLEARANCE_X);
        assert_eq!(p.clearance_y_mm, CARTRIDGE_CLEARANCE_Y);
        assert_eq!(p.heater_bore_diameter_mm, HEATER_BORE_DIAMETER);
        assert!(toml::from_str::<Parameters>("block_height_mm = 16").is_err());
        assert!(toml::from_str::<Parameters>(&format!("{raw}\nmisspelled = 1")).is_err());
        for value in [f64::NAN, f64::INFINITY, -1.0, 0.0, 25.0] {
            let mut bad = p.clone();
            bad.block_height_mm = value;
            assert!(bad.validate().is_err());
        }
    }
}

/// Orthographic wireframe for human review. Numerical checks remain authoritative.
pub fn preview_svg(bytes: &[u8], p: &Parameters) -> Result<String, String> {
    verify_mesh(bytes, p)?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"420\" viewBox=\"0 0 1200 420\"><rect width=\"1200\" height=\"420\" fill=\"#f7f8fa\"/><text x=\"24\" y=\"30\" font-family=\"sans-serif\" font-size=\"20\">Heating platen — mechanical prototype mesh</text>");
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
    svg.push_str("<text x=\"24\" y=\"406\" font-family=\"sans-serif\" font-size=\"12\">Orthographic mesh review only · not thermal or manufacturing qualification</text></svg>");
    Ok(svg)
}
