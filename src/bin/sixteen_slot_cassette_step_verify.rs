#[cfg(not(feature = "step"))]
fn main() {
    eprintln!(
        "This binary requires the 'step' feature. Run it through the LaminarForge MCP runner with features: ['step']."
    );
    std::process::exit(1);
}

#[cfg(feature = "step")]
mod step_verify {
    use std::fs;

    use glam::DVec3;
    use laminarforge_cad::sixteen_slot_cassette_a0::*;
    use opencascade::mesh::Mesh;
    use opencascade::primitives::Shape;
    use sha2::{Digest, Sha256};

    const BOUNDS_TOLERANCE_MM: f64 = 0.05;

    #[derive(Clone, Copy)]
    struct OutputSpec {
        path: &'static str,
        expected_size: [f64; 3],
        expected_solids: usize,
    }

    const OUTPUTS: [OutputSpec; 7] = [
        OutputSpec {
            path: "output/rfq/sixteen_slot_cassette_lower_carrier.step",
            expected_size: [CARRIER_X, CARRIER_Y, CARRIER_OVERALL_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_cassette_lid_clamp.step",
            expected_size: [LID_X, LID_Y, LID_OVERALL_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_cassette_window_placeholder.step",
            expected_size: [WINDOW_X, WINDOW_Y, WINDOW_OVERALL_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_cassette_gasket_witness_coupon.step",
            expected_size: [COUPON_X, COUPON_Y, COUPON_OVERALL_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_incubator_dock_plate.step",
            expected_size: [DOCK_X, DOCK_Y, DOCK_OVERALL_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_service_bulkhead_test_block.step",
            expected_size: [BULKHEAD_X, BULKHEAD_OVERALL_Y, BULKHEAD_Z],
            expected_solids: 1,
        },
        OutputSpec {
            path: "output/rfq/sixteen_slot_cassette_stackup_reference.step",
            expected_size: [ASSEMBLY_OVERALL_X, ASSEMBLY_OVERALL_Y, ASSEMBLY_OVERALL_Z],
            expected_solids: 3,
        },
    ];

    pub fn main() {
        validate_contract().expect("invalid 16-slot A0 interface contract");
        for spec in OUTPUTS {
            verify_step(spec).unwrap_or_else(|error| panic!("{}: {error}", spec.path));
        }
    }

    fn verify_step(spec: OutputSpec) -> Result<(), String> {
        let bytes =
            fs::read(spec.path).map_err(|error| format!("failed to read STEP output: {error}"))?;
        if bytes.is_empty() {
            return Err("STEP output is empty".to_string());
        }
        let text = std::str::from_utf8(&bytes)
            .map_err(|error| format!("STEP output is not UTF-8 Part 21 text: {error}"))?;
        let solid_count = text.matches("MANIFOLD_SOLID_BREP(").count();
        if solid_count != spec.expected_solids {
            return Err(format!(
                "expected {} closed solids, found {solid_count}",
                spec.expected_solids
            ));
        }

        let shape = Shape::read_step(spec.path)
            .map_err(|error| format!("OpenCascade re-import failed: {error}"))?;
        let face_count = shape.faces().count();
        let edge_count = shape.edges().count();
        if face_count == 0 || edge_count == 0 {
            return Err(format!(
                "re-imported shape has no topology: faces={face_count}, edges={edge_count}"
            ));
        }

        let mesh = shape.mesh();
        if mesh.vertices.is_empty() || mesh.indices.is_empty() {
            return Err("re-imported shape produced an empty verification mesh".to_string());
        }
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for vertex in &mesh.vertices {
            for axis in 0..3 {
                min[axis] = min[axis].min(vertex[axis]);
                max[axis] = max[axis].max(vertex[axis]);
            }
        }
        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
        for axis in 0..3 {
            if (size[axis] - spec.expected_size[axis]).abs() > BOUNDS_TOLERANCE_MM {
                return Err(format!(
                    "axis {axis} bound {:.4}mm does not match A0 {:.4}mm",
                    size[axis], spec.expected_size[axis]
                ));
            }
        }
        if spec.path == "output/rfq/sixteen_slot_cassette_lid_clamp.step" {
            verify_lid_features(&mesh)?;
        }
        if spec.path == "output/rfq/sixteen_slot_incubator_dock_plate.step" {
            verify_dock_features(&mesh)?;
        }

        println!(
            "verified {}: solids={} faces={} edges={} bounds={:.2}x{:.2}x{:.2}mm sha256={:x}",
            spec.path,
            solid_count,
            face_count,
            edge_count,
            size[0],
            size[1],
            size[2],
            Sha256::digest(&bytes),
        );
        Ok(())
    }

    fn verify_lid_features(mesh: &Mesh) -> Result<(), String> {
        let groove_floor_z = -LID_Z / 2.0 + GASKET_GROOVE_DEPTH;
        let slot_groove_x = PER_SLOT_GASKET_GROOVE_OUTER_X / 2.0 - GASKET_GROOVE_W / 2.0;
        let slot_groove_y = PER_SLOT_GASKET_GROOVE_OUTER_Y / 2.0 - GASKET_GROOVE_W / 2.0;

        for row in 0..ROWS {
            for col in 0..COLS {
                let slot = slot_number(row, col);
                let (x, y) = slot_center(row, col);
                for (label, point, normal) in [
                    ("rear", [x + 3.17, y + slot_groove_y], [0.0, 1.0]),
                    ("front", [x - 4.31, y - slot_groove_y], [0.0, 1.0]),
                    ("right", [x + slot_groove_x, y + 5.29], [1.0, 0.0]),
                    ("left", [x - slot_groove_x, y - 6.43], [1.0, 0.0]),
                ] {
                    verify_groove_section(
                        mesh,
                        point,
                        normal,
                        groove_floor_z,
                        &format!("S{slot:02} {label} groove"),
                    )?;
                }

                require_material_state(
                    mesh,
                    [x + 2.71, y - 1.93, 0.0],
                    false,
                    &format!("S{slot:02} view opening"),
                )?;
                let relief_roof_z = -LID_Z / 2.0 + LID_CHIP_TOP_RELIEF_DEPTH;
                let relief_probe_x = x + LID_SLOT_VIEW_OPENING_X / 2.0 + 3.0;
                require_material_state(
                    mesh,
                    [relief_probe_x, y + 0.23, relief_roof_z - 0.05],
                    false,
                    &format!("S{slot:02} chip-top clearance immediately below roof"),
                )?;
                require_material_state(
                    mesh,
                    [relief_probe_x, y + 0.23, relief_roof_z + 0.05],
                    true,
                    &format!("S{slot:02} chip-top clearance immediately above roof"),
                )?;
            }
        }

        let perimeter_groove_x = PERIMETER_GASKET_GROOVE_OUTER_X / 2.0 - GASKET_GROOVE_W / 2.0;
        let perimeter_groove_y = PERIMETER_GASKET_GROOVE_OUTER_Y / 2.0 - GASKET_GROOVE_W / 2.0;
        for (label, point, normal) in [
            ("rear", [41.37, perimeter_groove_y], [0.0, 1.0]),
            ("front", [-43.21, -perimeter_groove_y], [0.0, 1.0]),
            ("right", [perimeter_groove_x, 37.19], [1.0, 0.0]),
            ("left", [-perimeter_groove_x, -31.73], [1.0, 0.0]),
        ] {
            verify_groove_section(
                mesh,
                point,
                normal,
                groove_floor_z,
                &format!("perimeter {label} groove"),
            )?;
        }

        let skin_split_z = -LID_Z / 2.0 + LID_UNDERSIDE_SEAL_SKIN_Z;
        require_material_state(
            mesh,
            [302.0, 20.0, skin_split_z - 0.05],
            true,
            "continuous seal skin immediately below upper lightening relief",
        )?;
        require_material_state(
            mesh,
            [302.0, 20.0, skin_split_z + 0.05],
            false,
            "upper lightening relief immediately above seal skin",
        )?;
        Ok(())
    }

    fn verify_dock_features(mesh: &Mesh) -> Result<(), String> {
        for row in 0..ROWS {
            for col in 0..COLS {
                let slot = slot_number(row, col);
                let (x, y) = slot_center(row, col);
                require_material_state(
                    mesh,
                    [x + 0.41, y + 20.37, DOCK_SLOT_RECESS_FLOOR_Z + 0.05],
                    false,
                    &format!("S{slot:02} dock recess above exact floor"),
                )?;
                require_material_state(
                    mesh,
                    [x + 0.41, y + 20.37, DOCK_SLOT_RECESS_FLOOR_Z - 0.05],
                    true,
                    &format!("S{slot:02} dock recess below exact floor"),
                )?;
            }
        }
        for probe in dock_material_probes() {
            require_material_state(mesh, probe.point, probe.expected_inside, probe.feature)?;
        }
        Ok(())
    }

    fn verify_groove_section(
        mesh: &Mesh,
        point: [f64; 2],
        normal: [f64; 2],
        groove_floor_z: f64,
        feature: &str,
    ) -> Result<(), String> {
        require_material_state(
            mesh,
            [point[0], point[1], groove_floor_z - 0.05],
            false,
            &format!("{feature} cavity immediately below exact floor"),
        )?;
        require_material_state(
            mesh,
            [point[0], point[1], groove_floor_z + 0.05],
            true,
            &format!("{feature} support immediately above exact floor"),
        )?;

        let cavity_z = groove_floor_z - 0.05;
        for (side, direction) in [("inner", -1.0), ("outer", 1.0)] {
            let cavity_edge_offset = GASKET_GROOVE_W / 2.0 - 0.05;
            require_material_state(
                mesh,
                [
                    point[0] + normal[0] * cavity_edge_offset * direction,
                    point[1] + normal[1] * cavity_edge_offset * direction,
                    cavity_z,
                ],
                false,
                &format!("{feature} {side} cavity immediately inside exact wall"),
            )?;
            let shoulder_offset = GASKET_GROOVE_W / 2.0 + 0.05;
            require_material_state(
                mesh,
                [
                    point[0] + normal[0] * shoulder_offset * direction,
                    point[1] + normal[1] * shoulder_offset * direction,
                    cavity_z,
                ],
                true,
                &format!("{feature} {side} shoulder immediately outside exact wall"),
            )?;
        }
        Ok(())
    }

    fn require_material_state(
        mesh: &Mesh,
        point: [f64; 3],
        expected_inside: bool,
        feature: &str,
    ) -> Result<(), String> {
        let actual_inside = point_inside_mesh(mesh, point);
        if actual_inside != expected_inside {
            let expected = if expected_inside { "solid" } else { "void" };
            let actual = if actual_inside { "solid" } else { "void" };
            return Err(format!(
                "{feature} probe at ({:.3}, {:.3}, {:.3}) mm expected {expected}, found {actual}",
                point[0], point[1], point[2]
            ));
        }
        Ok(())
    }

    fn point_inside_mesh(mesh: &Mesh, point: [f64; 3]) -> bool {
        const SURFACE_TOLERANCE: f64 = 1e-6;
        let mut intersections = mesh
            .indices
            .chunks_exact(3)
            .filter_map(|indices| {
                vertical_intersection_z(
                    [
                        mesh.vertices[indices[0]],
                        mesh.vertices[indices[1]],
                        mesh.vertices[indices[2]],
                    ],
                    point[0],
                    point[1],
                )
            })
            .filter(|z| *z > point[2] + SURFACE_TOLERANCE)
            .collect::<Vec<_>>();
        intersections.sort_by(f64::total_cmp);
        intersections.dedup_by(|left, right| (*left - *right).abs() < SURFACE_TOLERANCE);
        intersections.len() % 2 == 1
    }

    fn vertical_intersection_z(triangle: [DVec3; 3], x: f64, y: f64) -> Option<f64> {
        const PROJECTED_AREA_TOLERANCE: f64 = 1e-12;
        const BARYCENTRIC_TOLERANCE: f64 = 1e-9;
        let [a, b, c] = triangle;
        let denominator = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
        if denominator.abs() < PROJECTED_AREA_TOLERANCE {
            return None;
        }
        let weight_a = ((b.y - c.y) * (x - c.x) + (c.x - b.x) * (y - c.y)) / denominator;
        let weight_b = ((c.y - a.y) * (x - c.x) + (a.x - c.x) * (y - c.y)) / denominator;
        let weight_c = 1.0 - weight_a - weight_b;
        if weight_a < -BARYCENTRIC_TOLERANCE
            || weight_b < -BARYCENTRIC_TOLERANCE
            || weight_c < -BARYCENTRIC_TOLERANCE
        {
            return None;
        }
        Some(weight_a * a.z + weight_b * b.z + weight_c * c.z)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn output_manifest_is_complete_unique_and_scoped() {
            assert_eq!(OUTPUTS.len(), 7);
            let mut paths = HashSet::new();
            for output in OUTPUTS {
                assert!(output.path.starts_with("output/rfq/sixteen_slot_"));
                assert!(paths.insert(output.path));
            }
        }

        #[test]
        fn standalone_parts_are_single_solid() {
            assert!(OUTPUTS[..6]
                .iter()
                .all(|output| output.expected_solids == 1));
            assert_eq!(OUTPUTS[6].expected_solids, 3);
        }
    }
}

#[cfg(feature = "step")]
fn main() {
    step_verify::main();
}
