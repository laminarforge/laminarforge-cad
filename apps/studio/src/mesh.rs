use glam::Vec3;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub min: Vec3,
    pub max: Vec3,
    pub triangles: usize,
    pub degenerate: usize,
}

impl Mesh {
    pub fn load(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Cannot open {}: {e}", path.display()))?;
        let bytes = file.metadata().map_err(|e| e.to_string())?.len();
        if bytes > 150_000_000 {
            return Err("This STL exceeds the 150 MB preview limit.".into());
        }
        let mesh =
            stl_io::read_stl(&mut BufReader::new(file)).map_err(|e| format!("Invalid STL: {e}"))?;
        if mesh.faces.is_empty() {
            return Err("The STL has no triangles.".into());
        }
        if mesh.faces.len() > 2_000_000 {
            return Err("This STL exceeds the two million triangle preview limit.".into());
        }
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        let mut vertices = Vec::with_capacity(mesh.faces.len() * 27);
        let mut degenerate = 0;
        for face in &mesh.faces {
            let points = face
                .vertices
                .map(|i| Vec3::from_array(mesh.vertices[i].into()));
            if points
                .iter()
                .any(|v| !v.is_finite() || v.abs().max_element() > 1e12)
            {
                return Err("STL contains non-finite or excessively large coordinates.".into());
            }
            let normal = (points[1] - points[0]).cross(points[2] - points[0]);
            if normal.length_squared() <= 1e-20 {
                degenerate += 1;
                continue;
            }
            let normal = normal.normalize();
            for (i, p) in points.iter().enumerate() {
                min = min.min(*p);
                max = max.max(*p);
                vertices.extend_from_slice(&p.to_array());
                vertices.extend_from_slice(&normal.to_array());
                vertices.extend_from_slice(&[
                    if i == 0 { 1.0 } else { 0.0 },
                    if i == 1 { 1.0 } else { 0.0 },
                    if i == 2 { 1.0 } else { 0.0 },
                ]);
            }
        }
        if vertices.is_empty() {
            return Err("The STL contains only zero-area triangles.".into());
        }
        Ok(Self {
            triangles: vertices.len() / 27,
            vertices,
            min,
            max,
            degenerate,
        })
    }
}

pub fn scan(root: &Path) -> Result<Vec<PathBuf>, String> {
    fn visit(dir: &Path, depth: usize, found: &mut Vec<PathBuf>) -> Result<(), String> {
        if depth > 16 {
            return Err("Library exceeds 16 folder levels. Choose a more specific folder.".into());
        }
        for entry in
            std::fs::read_dir(dir).map_err(|e| format!("Cannot read {}: {e}", dir.display()))?
        {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with('.') || matches!(name.as_ref(), "target" | "node_modules") {
                continue;
            }
            let kind = entry.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                visit(&entry.path(), depth + 1, found)?;
            } else if entry
                .path()
                .extension()
                .is_some_and(|x| x.eq_ignore_ascii_case("stl"))
            {
                found.push(entry.path());
                if found.len() > 10_000 {
                    return Err("Library exceeds 10,000 meshes. Choose a smaller folder.".into());
                }
            }
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(root, 0, &mut found)?;
    found.sort();
    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_and_empty_stl() {
        let root = std::env::temp_dir().join(format!("studio-invalid-{}", std::process::id()));
        std::fs::write(&root, b"not a mesh").unwrap();
        assert!(Mesh::load(&root).is_err());
        std::fs::write(&root, b"solid empty\nendsolid empty\n").unwrap();
        assert!(Mesh::load(&root).is_err());
        std::fs::remove_file(root).unwrap();
    }
    #[test]
    fn reads_ascii_bounds_and_recomputes_normal() {
        let path = std::env::temp_dir().join(format!("studio-ascii-{}", std::process::id()));
        std::fs::write(&path, "solid test\nfacet normal 0 0 0\nouter loop\nvertex 1 2 3\nvertex 5 2 3\nvertex 1 8 3\nendloop\nendfacet\nendsolid test\n").unwrap();
        let m = Mesh::load(&path).unwrap();
        assert_eq!(m.triangles, 1);
        assert_eq!(m.max - m.min, Vec3::new(4.0, 6.0, 0.0));
        assert_eq!(&m.vertices[3..6], &[0.0, 0.0, 1.0]);
        std::fs::remove_file(path).unwrap();
    }
}
