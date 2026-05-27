use std::fs;
use std::path::Path;

#[derive(Debug)]
struct ExpectedOutput {
    generator: &'static str,
    path: &'static str,
    min_triangles: u32,
    min_size_mm: [f32; 3],
}

#[derive(Debug)]
struct StlStats {
    triangles: u32,
    bbox_min: [f32; 3],
    bbox_max: [f32; 3],
}

const OUTPUTS: &[ExpectedOutput] = &[
    ExpectedOutput {
        generator: "co2_incubator",
        path: "output/co2_incubator_chamber.stl",
        min_triangles: 100,
        min_size_mm: [250.0, 200.0, 200.0],
    },
    ExpectedOutput {
        generator: "co2_incubator",
        path: "output/co2_incubator_service_manifold.stl",
        min_triangles: 50,
        min_size_mm: [120.0, 5.0, 50.0],
    },
    ExpectedOutput {
        generator: "cell_culture_logger_enclosure",
        path: "output/cell_culture_logger_enclosure_body.stl",
        min_triangles: 80,
        min_size_mm: [120.0, 70.0, 35.0],
    },
    ExpectedOutput {
        generator: "cell_culture_logger_enclosure",
        path: "output/cell_culture_logger_enclosure_lid.stl",
        min_triangles: 40,
        min_size_mm: [120.0, 70.0, 3.0],
    },
    ExpectedOutput {
        generator: "co2_sensor_service_module",
        path: "output/co2_sensor_service_module_base.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "co2_sensor_service_module",
        path: "output/co2_sensor_service_module_cover.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 80.0, 35.0],
    },
    ExpectedOutput {
        generator: "water_bath",
        path: "output/water_bath_basin.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 100.0, 40.0],
    },
    ExpectedOutput {
        generator: "water_bath_safety_kit",
        path: "output/water_bath_safety_bottle_rack.stl",
        min_triangles: 80,
        min_size_mm: [100.0, 60.0, 45.0],
    },
    ExpectedOutput {
        generator: "water_bath_safety_kit",
        path: "output/water_bath_safety_spill_tray.stl",
        min_triangles: 20,
        min_size_mm: [200.0, 150.0, 10.0],
    },
    ExpectedOutput {
        generator: "heating_block",
        path: "output/heating_block.stl",
        min_triangles: 50,
        min_size_mm: [50.0, 20.0, 10.0],
    },
    ExpectedOutput {
        generator: "orbital_shaker",
        path: "output/orbital_shaker_base.stl",
        min_triangles: 50,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "rack_rocker",
        path: "output/rack_rocker_base.stl",
        min_triangles: 50,
        min_size_mm: [100.0, 80.0, 5.0],
    },
    ExpectedOutput {
        generator: "media_reservoir",
        path: "output/media_reservoir_body.stl",
        min_triangles: 100,
        min_size_mm: [150.0, 150.0, 200.0],
    },
    ExpectedOutput {
        generator: "chip_priming_tubing_fixture",
        path: "output/chip_priming_fixture_base.stl",
        min_triangles: 80,
        min_size_mm: [160.0, 130.0, 10.0],
    },
    ExpectedOutput {
        generator: "chip_priming_tubing_fixture",
        path: "output/chip_priming_fixture_tubing_comb.stl",
        min_triangles: 40,
        min_size_mm: [110.0, 100.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_media_exchange_cassette",
        path: "output/automated_media_exchange_cassette_base.stl",
        min_triangles: 80,
        min_size_mm: [550.0, 470.0, 10.0],
    },
    ExpectedOutput {
        generator: "automated_media_exchange_cassette",
        path: "output/automated_media_exchange_cassette_assembly.stl",
        min_triangles: 120,
        min_size_mm: [610.0, 470.0, 30.0],
    },
    ExpectedOutput {
        generator: "pipette_tip_organizer",
        path: "output/pipette_tip_organizer_pipette_stand.stl",
        min_triangles: 80,
        min_size_mm: [240.0, 80.0, 100.0],
    },
    ExpectedOutput {
        generator: "pipette_tip_organizer",
        path: "output/pipette_tip_organizer_tip_box_tray.stl",
        min_triangles: 40,
        min_size_mm: [280.0, 115.0, 15.0],
    },
    ExpectedOutput {
        generator: "aspirator_waste_trap_holder",
        path: "output/aspirator_waste_trap_base_tray.stl",
        min_triangles: 20,
        min_size_mm: [300.0, 140.0, 10.0],
    },
    ExpectedOutput {
        generator: "aspirator_waste_trap_holder",
        path: "output/aspirator_waste_trap_bottle_cradle.stl",
        min_triangles: 80,
        min_size_mm: [200.0, 80.0, 50.0],
    },
    ExpectedOutput {
        generator: "sample_cold_block",
        path: "output/sample_cold_block.stl",
        min_triangles: 50,
        min_size_mm: [50.0, 30.0, 10.0],
    },
    ExpectedOutput {
        generator: "peltier_reservoir_block",
        path: "output/peltier_reservoir_block.stl",
        min_triangles: 50,
        min_size_mm: [40.0, 40.0, 10.0],
    },
    ExpectedOutput {
        generator: "still_air_box_corner",
        path: "output/still_air_box_corner.stl",
        min_triangles: 20,
        min_size_mm: [20.0, 20.0, 20.0],
    },
    ExpectedOutput {
        generator: "still_air_box_rail",
        path: "output/still_air_box_rail_400.stl",
        min_triangles: 20,
        min_size_mm: [350.0, 5.0, 5.0],
    },
    ExpectedOutput {
        generator: "still_air_box_accessories",
        path: "output/still_air_box_tube_rack.stl",
        min_triangles: 20,
        min_size_mm: [50.0, 20.0, 5.0],
    },
    ExpectedOutput {
        generator: "workstation_enclosure",
        path: "output/workstation_rail_segment.stl",
        min_triangles: 20,
        min_size_mm: [200.0, 5.0, 5.0],
    },
    ExpectedOutput {
        generator: "centrifuge_adapter",
        path: "output/centrifuge_adapter_15ml.stl",
        min_triangles: 30,
        min_size_mm: [13.0, 13.0, 15.0],
    },
];

fn main() {
    let mut failures = Vec::new();

    println!("Cell culture equipment STL verification");
    println!("---------------------------------------");

    for output in OUTPUTS {
        match read_binary_stl(output.path) {
            Ok(stats) => {
                let size = [
                    stats.bbox_max[0] - stats.bbox_min[0],
                    stats.bbox_max[1] - stats.bbox_min[1],
                    stats.bbox_max[2] - stats.bbox_min[2],
                ];

                println!(
                    "{} ({}) triangles={} size=[{:.2}, {:.2}, {:.2}]",
                    output.path, output.generator, stats.triangles, size[0], size[1], size[2],
                );

                if stats.triangles < output.min_triangles {
                    failures.push(format!(
                        "{} has {} triangles, expected at least {}",
                        output.path, stats.triangles, output.min_triangles
                    ));
                }

                for (axis, (&actual, &minimum)) in
                    size.iter().zip(output.min_size_mm.iter()).enumerate()
                {
                    if actual < minimum {
                        failures.push(format!(
                            "{} axis {} is {:.2}mm, expected at least {:.2}mm",
                            output.path, axis, actual, minimum
                        ));
                    }
                }
            }
            Err(error) => failures.push(format!(
                "{} missing or invalid; run `{}` first: {error}",
                output.path, output.generator
            )),
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

    println!("All starter cell-culture equipment STL outputs are present and sane.");
}

fn read_binary_stl(path: &str) -> Result<StlStats, String> {
    let bytes = fs::read(Path::new(path)).map_err(|error| format!("failed to read: {error}"))?;
    parse_binary_stl(&bytes)
}

fn parse_binary_stl(bytes: &[u8]) -> Result<StlStats, String> {
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
                read_f32(bytes, vertex_offset)?,
                read_f32(bytes, vertex_offset + 4)?,
                read_f32(bytes, vertex_offset + 8)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_too_small_stl() {
        let result = parse_binary_stl(&[0u8; 12]);
        assert!(result.is_err());
    }

    #[test]
    fn required_outputs_include_new_validation_modules() {
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "cell_culture_logger_enclosure"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "co2_sensor_service_module"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "co2_incubator"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "aspirator_waste_trap_holder"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "chip_priming_tubing_fixture"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "water_bath_safety_kit"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "pipette_tip_organizer"));
        assert!(OUTPUTS
            .iter()
            .any(|output| output.generator == "automated_media_exchange_cassette"));
    }
}
