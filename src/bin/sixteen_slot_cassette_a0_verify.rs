use std::collections::{HashMap, HashSet};
use std::fs;

use laminarforge_cad::{sixteen_slot_cassette_a0::*, REVC_CHIP_LENGTH};
use sha2::{Digest, Sha256};

const BOUNDS_TOLERANCE_MM: f64 = 0.05;

#[derive(Clone, Copy)]
struct OutputSpec {
    path: &'static str,
    expected_size: [f64; 3],
}

const OUTPUTS: [OutputSpec; 7] = [
    OutputSpec {
        path: "output/sixteen_slot_cassette_lower_carrier.stl",
        expected_size: [CARRIER_X, CARRIER_Y, CARRIER_OVERALL_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_cassette_lid_clamp.stl",
        expected_size: [LID_X, LID_Y, LID_OVERALL_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_cassette_window_placeholder.stl",
        expected_size: [WINDOW_X, WINDOW_Y, WINDOW_OVERALL_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_cassette_gasket_witness_coupon.stl",
        expected_size: [COUPON_X, COUPON_Y, COUPON_OVERALL_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_incubator_dock_plate.stl",
        expected_size: [DOCK_X, DOCK_Y, DOCK_OVERALL_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_service_bulkhead_test_block.stl",
        expected_size: [BULKHEAD_X, BULKHEAD_OVERALL_Y, BULKHEAD_Z],
    },
    OutputSpec {
        path: "output/sixteen_slot_cassette_incubator_first_article_assembly.stl",
        expected_size: [ASSEMBLY_OVERALL_X, ASSEMBLY_OVERALL_Y, ASSEMBLY_OVERALL_Z],
    },
];

#[derive(Debug)]
struct MeshSummary {
    triangles: u32,
    components: usize,
    boundary_edges: usize,
    nonmanifold_edges: usize,
    size: [f64; 3],
    sha256: String,
}

#[derive(Clone, Copy, Debug)]
struct Triangle {
    vertices: [[f64; 3]; 3],
}

#[derive(Debug)]
struct VerifiedMesh {
    summary: MeshSummary,
    triangles: Vec<Triangle>,
}

fn main() {
    validate_contract().expect("invalid 16-slot A0 interface contract");
    validate_output_manifest().expect("invalid 16-slot A0 STL output manifest");

    for spec in OUTPUTS {
        let mesh = verify_binary_stl(spec).unwrap_or_else(|error| panic!("{error}"));
        match spec.path {
            "output/sixteen_slot_cassette_lower_carrier.stl" => {
                verify_carrier_features(&mesh)
                    .unwrap_or_else(|error| panic!("{}: {error}", spec.path));
            }
            "output/sixteen_slot_cassette_lid_clamp.stl" => {
                verify_lid_features(&mesh).unwrap_or_else(|error| panic!("{}: {error}", spec.path));
            }
            "output/sixteen_slot_cassette_window_placeholder.stl" => {
                verify_window_features(&mesh)
                    .unwrap_or_else(|error| panic!("{}: {error}", spec.path));
            }
            "output/sixteen_slot_incubator_dock_plate.stl" => {
                verify_dock_features(&mesh)
                    .unwrap_or_else(|error| panic!("{}: {error}", spec.path));
            }
            "output/sixteen_slot_cassette_incubator_first_article_assembly.stl" => {
                verify_assembly_features(&mesh)
                    .unwrap_or_else(|error| panic!("{}: {error}", spec.path));
            }
            _ => {}
        }
        println!(
            "verified {}: triangles={} components={} boundary_edges={} nonmanifold_edges={} bounds={:.2}x{:.2}x{:.2}mm sha256={}",
            spec.path,
            mesh.summary.triangles,
            mesh.summary.components,
            mesh.summary.boundary_edges,
            mesh.summary.nonmanifold_edges,
            mesh.summary.size[0],
            mesh.summary.size[1],
            mesh.summary.size[2],
            mesh.summary.sha256,
        );
    }
}

fn validate_output_manifest() -> Result<(), String> {
    if OUTPUTS.len() != 7 {
        return Err(format!("expected 7 outputs, found {}", OUTPUTS.len()));
    }
    let mut paths = HashSet::with_capacity(OUTPUTS.len());
    for output in OUTPUTS {
        if !output.path.starts_with("output/sixteen_slot_") {
            return Err(format!("unexpected output path: {}", output.path));
        }
        if !paths.insert(output.path) {
            return Err(format!("duplicate output path: {}", output.path));
        }
    }
    Ok(())
}

fn verify_binary_stl(spec: OutputSpec) -> Result<VerifiedMesh, String> {
    let bytes =
        fs::read(spec.path).map_err(|error| format!("failed to read {}: {error}", spec.path))?;
    if bytes.len() < 84 {
        return Err(format!("{} is too small for binary STL", spec.path));
    }

    let triangles = u32::from_le_bytes(
        bytes[80..84]
            .try_into()
            .map_err(|_| format!("{} has an invalid triangle header", spec.path))?,
    );
    if triangles == 0 {
        return Err(format!("{} has zero triangles", spec.path));
    }

    let expected_bytes = 84usize
        .checked_add(triangles as usize * 50)
        .ok_or_else(|| format!("{} triangle count overflows file-size math", spec.path))?;
    if bytes.len() != expected_bytes {
        return Err(format!(
            "{} size mismatch: {} bytes for {triangles} triangles; expected {expected_bytes}",
            spec.path,
            bytes.len(),
        ));
    }

    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut parsed_triangles = Vec::with_capacity(triangles as usize);
    for triangle in 0..triangles as usize {
        let triangle_start = 84 + triangle * 50;
        let mut vertices = [[0.0; 3]; 3];
        for vertex in 0..3 {
            let vertex_start = triangle_start + 12 + vertex * 12;
            for axis in 0..3 {
                let coordinate_start = vertex_start + axis * 4;
                let coordinate = f32::from_le_bytes(
                    bytes[coordinate_start..coordinate_start + 4]
                        .try_into()
                        .map_err(|_| format!("{} has a truncated vertex", spec.path))?,
                ) as f64;
                if !coordinate.is_finite() {
                    return Err(format!("{} contains a non-finite vertex", spec.path));
                }
                vertices[vertex][axis] = coordinate;
                min[axis] = min[axis].min(coordinate);
                max[axis] = max[axis].max(coordinate);
            }
        }
        parsed_triangles.push(Triangle { vertices });
    }

    let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];
    for axis in 0..3 {
        if (size[axis] - spec.expected_size[axis]).abs() > BOUNDS_TOLERANCE_MM {
            return Err(format!(
                "{} axis {axis} bound {:.4}mm does not match A0 {:.4}mm",
                spec.path, size[axis], spec.expected_size[axis],
            ));
        }
    }

    let components = connected_triangle_components(&parsed_triangles);
    let (boundary_edges, nonmanifold_edges) = mesh_edge_incidence_counts(&parsed_triangles);
    if boundary_edges != 0 || nonmanifold_edges != 0 {
        return Err(format!(
            "{} is not a closed 2-manifold mesh: boundary_edges={boundary_edges}, nonmanifold_edges={nonmanifold_edges}",
            spec.path
        ));
    }
    if spec.path != "output/sixteen_slot_cassette_incubator_first_article_assembly.stl"
        && components != 1
    {
        return Err(format!(
            "{} must be one connected standalone mesh, found {components} components",
            spec.path
        ));
    }

    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    Ok(VerifiedMesh {
        summary: MeshSummary {
            triangles,
            components,
            boundary_edges,
            nonmanifold_edges,
            size,
            sha256,
        },
        triangles: parsed_triangles,
    })
}

fn verify_carrier_features(mesh: &VerifiedMesh) -> Result<(), String> {
    let pocket_floor_z = CARRIER_Z / 2.0 - CHIP_POCKET_DEPTH;
    let closure_z = CARRIER_Z / 2.0 + CLOSURE_PLANE_ABOVE_CARRIER;
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            let pocket_probe_x = x + REVC_CHIP_LENGTH / 2.0 - 5.0;
            require_material_state(
                mesh,
                [pocket_probe_x, y + 0.37, pocket_floor_z + 0.05],
                false,
                &format!("S{slot:02} pocket above exact floor"),
            )?;
            require_material_state(
                mesh,
                [pocket_probe_x, y + 0.37, pocket_floor_z - 0.05],
                true,
                &format!("S{slot:02} pocket below exact floor"),
            )?;
            let land_y = y + PER_SLOT_GASKET_OUTER_Y / 2.0 - GASKET_LAND_W / 2.0;
            require_material_state(
                mesh,
                [x + 3.17, land_y, closure_z - 0.05],
                true,
                &format!("S{slot:02} gasket land immediately below closure plane"),
            )?;
            require_material_state(
                mesh,
                [x + 3.17, land_y, closure_z + 0.05],
                false,
                &format!("S{slot:02} gasket land immediately above closure plane"),
            )?;
        }
    }

    for (index, (x, y)) in inter_slot_stop_points().into_iter().enumerate() {
        require_material_state(
            mesh,
            [x, y, closure_z - 0.05],
            true,
            &format!("inter-slot hard stop {} reaches closure plane", index + 1),
        )?;
        require_material_state(
            mesh,
            [x, y, closure_z + 0.05],
            false,
            &format!("inter-slot hard stop {} ends at closure plane", index + 1),
        )?;
    }

    let gutter_floor_z = CARRIER_Z / 2.0 - LEAK_GUTTER_DEPTH;
    let gutter_y = LEAK_GUTTER_OUTER_Y / 2.0 - LEAK_GUTTER_W / 2.0;
    require_material_state(
        mesh,
        [17.33, gutter_y, gutter_floor_z + 0.05],
        false,
        "rear leak gutter above exact floor",
    )?;
    require_material_state(
        mesh,
        [17.33, gutter_y, gutter_floor_z - 0.05],
        true,
        "rear leak gutter below exact floor",
    )?;

    let datum_probe_z = CARRIER_Z / 2.0 + DATUM_BOSS_Z / 2.0;
    for datum in datum_features() {
        require_material_state(
            mesh,
            [datum.x, datum.y, datum_probe_z],
            false,
            &format!("datum {} bore", datum.id),
        )?;
        require_material_state(
            mesh,
            [datum.x + 7.0, datum.y, datum_probe_z],
            true,
            &format!("datum {} boss wall", datum.id),
        )?;
    }
    Ok(())
}

fn verify_lid_features(mesh: &VerifiedMesh) -> Result<(), String> {
    let groove_floor_z = -LID_Z / 2.0 + GASKET_GROOVE_DEPTH;
    let slot_groove_x = PER_SLOT_GASKET_GROOVE_OUTER_X / 2.0 - GASKET_GROOVE_W / 2.0;
    let slot_groove_y = PER_SLOT_GASKET_GROOVE_OUTER_Y / 2.0 - GASKET_GROOVE_W / 2.0;

    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            verify_groove_section(
                mesh,
                [x + 3.17, y + slot_groove_y],
                [0.0, 1.0],
                groove_floor_z,
                &format!("S{slot:02} rear groove"),
            )?;
            verify_groove_section(
                mesh,
                [x - 4.31, y - slot_groove_y],
                [0.0, 1.0],
                groove_floor_z,
                &format!("S{slot:02} front groove"),
            )?;
            verify_groove_section(
                mesh,
                [x + slot_groove_x, y + 5.29],
                [1.0, 0.0],
                groove_floor_z,
                &format!("S{slot:02} right groove"),
            )?;
            verify_groove_section(
                mesh,
                [x - slot_groove_x, y - 6.43],
                [1.0, 0.0],
                groove_floor_z,
                &format!("S{slot:02} left groove"),
            )?;
            require_material_state(
                mesh,
                [x + 2.71, y - 1.93, 0.0],
                false,
                &format!("S{slot:02} view opening"),
            )?;
            let top_relief_x = x + LID_SLOT_VIEW_OPENING_X / 2.0 + 3.0;
            let relief_roof_z = -LID_Z / 2.0 + LID_CHIP_TOP_RELIEF_DEPTH;
            require_material_state(
                mesh,
                [top_relief_x, y + 0.23, relief_roof_z - 0.05],
                false,
                &format!("S{slot:02} chip-top clearance immediately below roof"),
            )?;
            require_material_state(
                mesh,
                [top_relief_x, y + 0.23, relief_roof_z + 0.05],
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

    let seat_probe_z = -LID_Z / 2.0 + LID_DATUM_PIN_SEAT_DEPTH / 2.0;
    for datum in datum_features() {
        let has_pin_seat = matches!(
            datum.role,
            DatumRole::RoundLocator | DatumRole::RelievedLocator
        );
        require_material_state(
            mesh,
            [datum.x, datum.y, seat_probe_z],
            !has_pin_seat,
            &format!("datum {} lid replaceable-pin seat policy", datum.id),
        )?;
        if has_pin_seat {
            require_material_state(
                mesh,
                [
                    datum.x + LID_DATUM_PIN_SEAT_DIAMETER / 2.0 + 0.5,
                    datum.y,
                    seat_probe_z,
                ],
                true,
                &format!("datum {} lid replaceable-pin seat wall", datum.id),
            )?;
        }
        require_material_state(
            mesh,
            [datum.x, datum.y, -LID_Z / 2.0 - 0.5],
            false,
            &format!("datum {} standalone lid excludes pin surrogate", datum.id),
        )?;
    }

    Ok(())
}

fn verify_groove_section(
    mesh: &VerifiedMesh,
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

fn verify_window_features(mesh: &VerifiedMesh) -> Result<(), String> {
    let component_count = connected_triangle_components(&mesh.triangles);
    if component_count != 1 {
        return Err(format!(
            "window placeholder must be one connected mesh, found {component_count} components"
        ));
    }

    for (index, (x, y)) in window_fiducial_points().into_iter().enumerate() {
        require_material_state(
            mesh,
            [x + 3.0, y, WINDOW_Z / 2.0 + 0.05],
            true,
            &format!("window fiducial {} attachment bridge", index + 1),
        )?;
        require_material_state(
            mesh,
            [
                x + 3.0,
                y,
                WINDOW_Z / 2.0 + WINDOW_FIDUCIAL_EXPOSED_Z - 0.05,
            ],
            true,
            &format!("window fiducial {} upper ring", index + 1),
        )?;
        require_material_state(
            mesh,
            [x, y, WINDOW_FIDUCIAL_CENTER_Z],
            false,
            &format!("window fiducial {} through center", index + 1),
        )?;
    }
    Ok(())
}

fn verify_dock_features(mesh: &VerifiedMesh) -> Result<(), String> {
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

fn verify_assembly_features(mesh: &VerifiedMesh) -> Result<(), String> {
    let carrier_center_z = DOCK_SUPPORT_PLANE_Z + CARRIER_Z / 2.0;
    let carrier_top_z = carrier_center_z + CARRIER_Z / 2.0;
    let pin_only_probe_z = carrier_top_z + DATUM_BOSS_Z + 0.50;
    for datum in datum_features() {
        let expected_pin = matches!(
            datum.role,
            DatumRole::RoundLocator | DatumRole::RelievedLocator
        );
        require_material_state(
            mesh,
            [datum.x, datum.y, pin_only_probe_z],
            expected_pin,
            &format!(
                "datum {} assembly replaceable-pin surrogate policy",
                datum.id
            ),
        )?;
    }
    Ok(())
}

fn require_material_state(
    mesh: &VerifiedMesh,
    point: [f64; 3],
    expected_inside: bool,
    feature: &str,
) -> Result<(), String> {
    let actual_inside = point_inside_mesh(&mesh.triangles, point);
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

fn point_inside_mesh(triangles: &[Triangle], point: [f64; 3]) -> bool {
    const SURFACE_TOLERANCE: f64 = 1e-6;
    let mut intersections = triangles
        .iter()
        .filter_map(|triangle| vertical_intersection_z(*triangle, point[0], point[1]))
        .filter(|z| *z > point[2] + SURFACE_TOLERANCE)
        .collect::<Vec<_>>();
    intersections.sort_by(f64::total_cmp);
    intersections.dedup_by(|left, right| (*left - *right).abs() < SURFACE_TOLERANCE);
    intersections.len() % 2 == 1
}

fn connected_triangle_components(triangles: &[Triangle]) -> usize {
    fn vertex_key(vertex: [f64; 3]) -> [u64; 3] {
        vertex.map(|coordinate| {
            if coordinate == 0.0 {
                0.0f64.to_bits()
            } else {
                coordinate.to_bits()
            }
        })
    }

    fn find(parents: &mut [usize], mut index: usize) -> usize {
        while parents[index] != index {
            parents[index] = parents[parents[index]];
            index = parents[index];
        }
        index
    }

    fn union(parents: &mut [usize], left: usize, right: usize) {
        let left_root = find(parents, left);
        let right_root = find(parents, right);
        if left_root != right_root {
            parents[right_root] = left_root;
        }
    }

    if triangles.is_empty() {
        return 0;
    }

    let mut parents = (0..triangles.len()).collect::<Vec<_>>();
    let mut first_triangle_by_edge = HashMap::<([u64; 3], [u64; 3]), usize>::new();
    for (triangle_index, triangle) in triangles.iter().enumerate() {
        let vertices = triangle.vertices.map(vertex_key);
        for (left, right) in [(0, 1), (1, 2), (2, 0)] {
            let edge = if vertices[left] <= vertices[right] {
                (vertices[left], vertices[right])
            } else {
                (vertices[right], vertices[left])
            };
            if let Some(previous) = first_triangle_by_edge.insert(edge, triangle_index) {
                union(&mut parents, triangle_index, previous);
            }
        }
    }

    (0..triangles.len())
        .map(|index| find(&mut parents, index))
        .collect::<HashSet<_>>()
        .len()
}

fn mesh_edge_incidence_counts(triangles: &[Triangle]) -> (usize, usize) {
    fn vertex_key(vertex: [f64; 3]) -> [u64; 3] {
        vertex.map(|coordinate| {
            if coordinate == 0.0 {
                0.0f64.to_bits()
            } else {
                coordinate.to_bits()
            }
        })
    }

    let mut incidence = HashMap::<([u64; 3], [u64; 3]), usize>::new();
    for triangle in triangles {
        let vertices = triangle.vertices.map(vertex_key);
        for (left, right) in [(0, 1), (1, 2), (2, 0)] {
            let edge = if vertices[left] <= vertices[right] {
                (vertices[left], vertices[right])
            } else {
                (vertices[right], vertices[left])
            };
            *incidence.entry(edge).or_default() += 1;
        }
    }
    (
        incidence.values().filter(|count| **count == 1).count(),
        incidence.values().filter(|count| **count > 2).count(),
    )
}

fn vertical_intersection_z(triangle: Triangle, x: f64, y: f64) -> Option<f64> {
    const PROJECTED_AREA_TOLERANCE: f64 = 1e-12;
    const BARYCENTRIC_TOLERANCE: f64 = 1e-9;
    let [a, b, c] = triangle.vertices;
    let denominator = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
    if denominator.abs() < PROJECTED_AREA_TOLERANCE {
        return None;
    }

    let weight_a = ((b[1] - c[1]) * (x - c[0]) + (c[0] - b[0]) * (y - c[1])) / denominator;
    let weight_b = ((c[1] - a[1]) * (x - c[0]) + (a[0] - c[0]) * (y - c[1])) / denominator;
    let weight_c = 1.0 - weight_a - weight_b;
    if weight_a < -BARYCENTRIC_TOLERANCE
        || weight_b < -BARYCENTRIC_TOLERANCE
        || weight_c < -BARYCENTRIC_TOLERANCE
    {
        return None;
    }

    Some(weight_a * a[2] + weight_b * b[2] + weight_c * c[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn box_triangles(min: [f64; 3], max: [f64; 3]) -> Vec<Triangle> {
        let vertices = [
            [min[0], min[1], min[2]],
            [max[0], min[1], min[2]],
            [max[0], max[1], min[2]],
            [min[0], max[1], min[2]],
            [min[0], min[1], max[2]],
            [max[0], min[1], max[2]],
            [max[0], max[1], max[2]],
            [min[0], max[1], max[2]],
        ];
        [
            [0, 2, 1],
            [0, 3, 2],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [1, 2, 6],
            [1, 6, 5],
            [2, 3, 7],
            [2, 7, 6],
            [3, 0, 4],
            [3, 4, 7],
        ]
        .map(|indices| Triangle {
            vertices: indices.map(|index| vertices[index]),
        })
        .to_vec()
    }

    #[test]
    fn output_manifest_is_complete_and_unique() {
        validate_output_manifest().expect("output manifest must be valid at runtime");
    }

    #[test]
    fn assembly_bounds_enclose_each_part_contract() {
        assert!(ASSEMBLY_OVERALL_X >= CARRIER_X);
        assert!(ASSEMBLY_OVERALL_Y >= DOCK_Y);
        assert!(ASSEMBLY_OVERALL_Z >= BULKHEAD_Z);
    }

    #[test]
    fn vertical_ray_classifies_a_closed_box() {
        let triangles = box_triangles([-2.0, -3.0, -4.0], [2.0, 3.0, 4.0]);

        assert!(point_inside_mesh(&triangles, [0.37, -0.41, 0.29]));
        assert!(!point_inside_mesh(&triangles, [0.37, -0.41, 4.50]));
        assert!(!point_inside_mesh(&triangles, [2.50, -0.41, 0.29]));
        assert_eq!(connected_triangle_components(&triangles), 1);
        assert_eq!(mesh_edge_incidence_counts(&triangles), (0, 0));
    }

    #[test]
    fn component_count_does_not_join_point_touching_solids() {
        let mut triangles = box_triangles([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        triangles.extend(box_triangles([1.0, 1.0, 1.0], [2.0, 2.0, 2.0]));
        assert_eq!(connected_triangle_components(&triangles), 2);
    }
}
