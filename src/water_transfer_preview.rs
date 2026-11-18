//! Deterministic isometric preview of the published water-transfer STL mesh.
//!
//! This is a visual aid, not a clearance, mesh-closure or fabrication check.

use std::fmt::Write;

type Point = [f64; 3];

struct Face {
    points: [[f64; 2]; 3],
    depth: f64,
    normal: Point,
    index: usize,
}

fn dot(a: Point, b: Point) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn subtract(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] - b[i])
}

fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Render the actual binary STL with a fixed isometric camera and fitted bounds.
///
/// Triangle winding supplies outward normals; the STL's stored normals are not
/// trusted. Visible triangles use centroid depth ordering, which is suitable
/// for this dock's separated, nonintersecting mechanical surfaces. This is not
/// a general hidden-surface solver for arbitrary intersecting meshes.
pub fn preview(bytes: &[u8]) -> Result<String, String> {
    // Validate byte count, finite vertices and nondegenerate 3D bounds before
    // accessing records or using their coordinates in projection arithmetic.
    crate::runtime_cad::mesh(bytes)?;

    let root_two = 2.0_f64.sqrt();
    let root_three = 3.0_f64.sqrt();
    let root_six = 6.0_f64.sqrt();
    let right = [1.0 / root_two, 1.0 / root_two, 0.0];
    let down = [1.0 / root_six, -1.0 / root_six, -(2.0 / 3.0_f64).sqrt()];
    let camera = [1.0 / root_three, -1.0 / root_three, 1.0 / root_three];
    let light = [0.25, -0.45, 0.8573214099741123];

    let mut low = [f64::INFINITY; 2];
    let mut high = [f64::NEG_INFINITY; 2];
    let mut faces = Vec::new();
    for (index, triangle) in bytes[84..].chunks_exact(50).enumerate() {
        let mut vertices = [[0.0; 3]; 3];
        for (vertex, point) in vertices.iter_mut().enumerate() {
            for (axis, value) in point.iter_mut().enumerate() {
                let offset = 12 + vertex * 12 + axis * 4;
                *value =
                    f32::from_le_bytes(triangle[offset..offset + 4].try_into().unwrap()) as f64;
            }
        }
        let points = vertices.map(|point| [dot(point, right), dot(point, down)]);
        for point in points {
            for axis in 0..2 {
                low[axis] = low[axis].min(point[axis]);
                high[axis] = high[axis].max(point[axis]);
            }
        }
        let normal = cross(
            subtract(vertices[1], vertices[0]),
            subtract(vertices[2], vertices[0]),
        );
        let magnitude = dot(normal, normal).sqrt();
        if magnitude == 0.0 {
            continue;
        }
        let normal = normal.map(|component| component / magnitude);
        if dot(normal, camera) <= 1.0e-9 {
            continue;
        }
        let depth = vertices
            .iter()
            .map(|point| dot(*point, camera))
            .sum::<f64>()
            / 3.0;
        faces.push(Face {
            points,
            depth,
            normal,
            index,
        });
    }
    if faces.is_empty() {
        return Err("STL has no outward-facing triangles for the isometric preview".into());
    }

    // Reserve space for the title and generous margins on every side. Bounds
    // include every vertex, including back faces, so no geometry is clipped.
    let width = high[0] - low[0];
    let height = high[1] - low[1];
    if width <= 0.0 || height <= 0.0 || !width.is_finite() || !height.is_finite() {
        return Err("invalid projected STL envelope".into());
    }
    let scale = (1056.0 / width).min(636.0 / height);
    let center = [(low[0] + high[0]) / 2.0, (low[1] + high[1]) / 2.0];
    let project = |point: [f64; 2]| {
        [
            600.0 + (point[0] - center[0]) * scale,
            462.0 + (point[1] - center[1]) * scale,
        ]
    };
    // The index explicitly resolves coplanar ties without relying on a sort's
    // implementation details. Closer triangles are painted after farther ones.
    faces.sort_by(|a, b| a.depth.total_cmp(&b.depth).then(a.index.cmp(&b.index)));

    let mut svg = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"840\" viewBox=\"0 0 1200 840\" role=\"img\" aria-labelledby=\"title description\">\n\
         <title id=\"title\">Water-transfer dock</title>\n\
         <desc id=\"description\">Isometric view of the generated STL mesh. Millimeter units. Mechanical prototype.</desc>\n\
         <rect width=\"1200\" height=\"840\" fill=\"#101a22\"/>\n\
         <text x=\"48\" y=\"56\" fill=\"#eff9f8\" font-family=\"sans-serif\" font-size=\"30\" font-weight=\"600\">Water-transfer dock</text>\n\
         <text x=\"48\" y=\"87\" fill=\"#9bb2bb\" font-family=\"sans-serif\" font-size=\"16\">CAD mesh preview · mm · prototype</text>\n\
         <g stroke-width=\"0.3\" stroke-linejoin=\"round\">\n",
    );
    for face in faces {
        let illumination = 0.52 + 0.48 * dot(face.normal, light).max(0.0);
        let red = (37.0 * illumination).round() as u8;
        let green = (210.0 * illumination).round() as u8;
        let blue = (191.0 * illumination).round() as u8;
        let points = face.points.map(project);
        // Matching fill/stroke closes subpixel seams between coplanar facets
        // without displaying the mesh's triangulation as product features.
        writeln!(
            svg,
            "<polygon points=\"{:.3},{:.3} {:.3},{:.3} {:.3},{:.3}\" fill=\"#{:02x}{:02x}{:02x}\" stroke=\"#{:02x}{:02x}{:02x}\"/>",
            points[0][0], points[0][1],
            points[1][0], points[1][1],
            points[2][0], points[2][1],
            red, green, blue, red, green, blue,
        )
        .map_err(|error| error.to_string())?;
    }
    svg.push_str("</g>\n</svg>\n");
    Ok(svg)
}
