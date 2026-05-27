use std::fs;
use std::path::Path;

#[derive(Debug)]
struct ExpectedPart {
    path: &'static str,
    expected_size_mm: [f32; 3],
    tolerance_mm: f32,
}

#[derive(Debug)]
struct StlStats {
    triangles: u32,
    bbox_min: [f32; 3],
    bbox_max: [f32; 3],
}

const PARTS: &[ExpectedPart] = &[
    ExpectedPart {
        path: "output/co2_incubator_chamber.stl",
        expected_size_mm: [306.0, 256.0, 256.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_shell.stl",
        expected_size_mm: [362.0, 312.0, 312.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_door.stl",
        expected_size_mm: [320.0, 12.0, 270.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_shelf.stl",
        expected_size_mm: [303.4, 240.0, 4.7],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_water_tray.stl",
        expected_size_mm: [200.0, 150.0, 20.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_service_bay.stl",
        expected_size_mm: [160.0, 55.0, 180.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_service_manifold.stl",
        expected_size_mm: [190.0, 12.0, 90.0],
        tolerance_mm: 0.5,
    },
    ExpectedPart {
        path: "output/co2_incubator_heater_diffuser.stl",
        expected_size_mm: [220.0, 35.0, 18.0],
        tolerance_mm: 0.5,
    },
];

fn main() {
    let mut failures = Vec::new();

    println!("CO2 incubator STL verification");
    println!("--------------------------------");

    for part in PARTS {
        match read_binary_stl(part.path) {
            Ok(stats) => {
                let actual_size = [
                    stats.bbox_max[0] - stats.bbox_min[0],
                    stats.bbox_max[1] - stats.bbox_min[1],
                    stats.bbox_max[2] - stats.bbox_min[2],
                ];

                println!(
                    "{} triangles={} bbox_min=[{:.2}, {:.2}, {:.2}] size=[{:.2}, {:.2}, {:.2}]",
                    part.path,
                    stats.triangles,
                    stats.bbox_min[0],
                    stats.bbox_min[1],
                    stats.bbox_min[2],
                    actual_size[0],
                    actual_size[1],
                    actual_size[2],
                );

                for (axis, (&actual, &expected)) in actual_size
                    .iter()
                    .zip(part.expected_size_mm.iter())
                    .enumerate()
                {
                    let delta = (actual - expected).abs();
                    if delta > part.tolerance_mm {
                        failures.push(format!(
                            "{} axis {} size {:.2}mm, expected {:.2}mm (+/- {:.2}mm)",
                            part.path, axis, actual, expected, part.tolerance_mm
                        ));
                    }
                }
            }
            Err(error) => failures.push(format!("{}: {error}", part.path)),
        }
    }

    if !failures.is_empty() {
        eprintln!();
        eprintln!("Verification failed:");
        for failure in failures {
            eprintln!("- {failure}");
        }
        std::process::exit(1);
    }

    println!("All CO2 incubator STL outputs match expected bounding boxes.");
}

fn read_binary_stl(path: &str) -> Result<StlStats, String> {
    let bytes = fs::read(Path::new(path)).map_err(|error| format!("failed to read: {error}"))?;
    if bytes.len() < 84 {
        return Err(format!(
            "file is too small for binary STL: {} bytes",
            bytes.len()
        ));
    }

    let triangles = u32::from_le_bytes(bytes[80..84].try_into().unwrap());
    if triangles == 0 {
        return Err("binary STL has zero triangles".to_string());
    }

    let expected_len = 84usize + triangles as usize * 50usize;
    if bytes.len() != expected_len {
        return Err(format!(
            "binary STL size mismatch: got {} bytes, expected {} bytes for {} triangles",
            bytes.len(),
            expected_len,
            triangles
        ));
    }

    let mut bbox_min = [f32::INFINITY; 3];
    let mut bbox_max = [f32::NEG_INFINITY; 3];

    for tri in 0..triangles as usize {
        let tri_offset = 84 + tri * 50;
        for vertex_idx in 0..3 {
            let vertex_offset = tri_offset + 12 + vertex_idx * 12;
            let vertex = [
                read_f32(&bytes, vertex_offset)?,
                read_f32(&bytes, vertex_offset + 4)?,
                read_f32(&bytes, vertex_offset + 8)?,
            ];
            for axis in 0..3 {
                if !vertex[axis].is_finite() {
                    return Err(format!("non-finite vertex coordinate in triangle {tri}"));
                }
                bbox_min[axis] = bbox_min[axis].min(vertex[axis]);
                bbox_max[axis] = bbox_max[axis].max(vertex[axis]);
            }
        }
    }

    Ok(StlStats {
        triangles,
        bbox_min,
        bbox_max,
    })
}

fn read_f32(bytes: &[u8], offset: usize) -> Result<f32, String> {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("unexpected EOF reading f32 at offset {offset}"))?
        .try_into()
        .unwrap();
    Ok(f32::from_le_bytes(raw))
}
