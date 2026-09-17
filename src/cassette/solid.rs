//! One construction tree drives both review meshes and analytic manufacturing solids.
use std::{
    ops::{Add, Deref, Sub},
    rc::Rc,
};
#[derive(Clone, Debug)]
pub struct Hole {
    pub name: String,
    pub center: [f64; 3],
    pub diameter: f64,
    pub axis: usize,
}
fn collect_holes(n: &Node, cut: bool, s: f64, t: [f64; 3], out: &mut Vec<Hole>) {
    match n {
        Node::Primitive(name, Primitive::Cylinder(c, r, _, axis)) if cut => {
            if name.contains("tap")
                || name.contains("clearance")
                || name.contains("counterbore")
                || name.contains("access")
                || name == "stop_receiver"
                || name == "validation_M5_port"
                || name == "shim_head_relief"
                || name == "spacer_bore"
                || name == "harness_anchor_M3"
            {
                out.push(Hole {
                    name: name.clone(),
                    center: [s * c[0] + t[0], c[1] + t[1], c[2] + t[2]],
                    diameter: 2.0 * r,
                    axis: *axis,
                });
            }
        }
        Node::Union(a, b) => {
            collect_holes(a, cut, s, t, out);
            collect_holes(b, cut, s, t, out);
        }
        Node::Cut(a, b) => {
            collect_holes(a, cut, s, t, out);
            collect_holes(b, !cut, s, t, out);
        }
        Node::Move(a, v) => {
            collect_holes(a, cut, s, [t[0] + s * v[0], t[1] + v[1], t[2] + v[2]], out)
        }
        Node::Mirror(a) => collect_holes(a, cut, -s, t, out),
        _ => {}
    }
}
#[derive(Clone)]
pub enum Primitive {
    Box([f64; 3], [f64; 3]),
    Cylinder([f64; 3], f64, f64, usize),
}
#[derive(Clone)]
enum Node {
    Empty,
    Primitive(String, Primitive),
    Union(Rc<Node>, Rc<Node>),
    Cut(Rc<Node>, Rc<Node>),
    Move(Rc<Node>, [f64; 3]),
    Mirror(Rc<Node>),
}
pub struct Part {
    mesh: vcad::Part,
    node: Rc<Node>,
}
impl Deref for Part {
    type Target = vcad::Part;
    fn deref(&self) -> &vcad::Part {
        &self.mesh
    }
}
impl Part {
    pub fn primitive(name: &str, mesh: vcad::Part, p: Primitive) -> Self {
        Self {
            mesh,
            node: Rc::new(Node::Primitive(name.into(), p)),
        }
    }
    pub fn empty(name: &str) -> Self {
        Self {
            mesh: vcad::Part::empty(name),
            node: Rc::new(Node::Empty),
        }
    }
    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        Self {
            mesh: self.mesh.translate(x, y, z),
            node: Rc::new(Node::Move(self.node.clone(), [x, y, z])),
        }
    }
    pub fn mirror_x(&self) -> Self {
        Self {
            mesh: self.mesh.mirror_x(),
            node: Rc::new(Node::Mirror(self.node.clone())),
        }
    }
    pub fn intersection(&self, other: &Self) -> vcad::Part {
        self.mesh.intersection(&other.mesh)
    }
    /// Named subtractive cylinders only; rounded profiles and slots are not drilled holes.
    pub fn holes(&self) -> Vec<Hole> {
        let mut holes = Vec::new();
        collect_holes(&self.node, false, 1.0, [0.0; 3], &mut holes);
        holes
    }
    #[cfg(feature = "step")]
    pub fn analytic(&self) -> opencascade::primitives::Shape {
        analytic(&self.node, 1.0, [0.0; 3]).expect("cannot export empty analytic shape")
    }
}
impl Add for Part {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {
            mesh: self.mesh + b.mesh,
            node: Rc::new(Node::Union(self.node, b.node)),
        }
    }
}
impl Sub for Part {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self {
            mesh: self.mesh - b.mesh,
            node: Rc::new(Node::Cut(self.node, b.node)),
        }
    }
}
#[cfg(feature = "step")]
fn analytic(n: &Node, s: f64, t: [f64; 3]) -> Option<opencascade::primitives::Shape> {
    use glam::dvec3;
    use opencascade::workplane::Workplane;
    let point = |p: [f64; 3]| [s * p[0] + t[0], p[1] + t[1], p[2] + t[2]];
    match n {
        Node::Empty => None,
        Node::Primitive(_, Primitive::Box(a, b)) => {
            let a = point(*a);
            let b = point(*b);
            Some(
                Workplane::xy()
                    .translated(dvec3((a[0] + b[0]) / 2.0, (a[1] + b[1]) / 2.0, a[2]))
                    .rect((b[0] - a[0]).abs(), b[1] - a[1])
                    .to_face()
                    .extrude(dvec3(0.0, 0.0, b[2] - a[2]))
                    .into(),
            )
        }
        Node::Primitive(_, Primitive::Cylinder(c, r, h, axis)) => {
            let mut c = point(*c);
            c[*axis] -= h / 2.0;
            let (mut wp, v) = match *axis {
                0 => (Workplane::yz(), dvec3(*h, 0.0, 0.0)),
                1 => (Workplane::xz(), dvec3(0.0, *h, 0.0)),
                _ => (Workplane::xy(), dvec3(0.0, 0.0, *h)),
            };
            wp.set_translation(dvec3(c[0], c[1], c[2]));
            Some(wp.circle(0.0, 0.0, *r).to_face().extrude(v).into())
        }
        Node::Union(a, b) => match (analytic(a, s, t), analytic(b, s, t)) {
            (Some(a), Some(b)) => Some(a.union(&b).into()),
            (a, None) => a,
            (None, b) => b,
        },
        Node::Cut(a, b) => Some(analytic(a, s, t)?.subtract(&analytic(b, s, t)?).into()),
        Node::Move(a, v) => analytic(a, s, [t[0] + s * v[0], t[1] + v[1], t[2] + v[2]]),
        Node::Mirror(a) => analytic(a, -s, t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hole_locations_follow_reflection_translation_and_cut_polarity() {
        let bore = Rc::new(Node::Primitive(
            "rail_M3_tap".into(),
            Primitive::Cylinder([2.0, 3.0, 4.0], 1.25, 12.0, 2),
        ));
        let tree = Node::Move(
            Rc::new(Node::Mirror(Rc::new(Node::Cut(
                Rc::new(Node::Empty),
                bore.clone(),
            )))),
            [10.0, 20.0, 30.0],
        );
        let mut result = Vec::new();
        collect_holes(&tree, false, 1.0, [0.0; 3], &mut result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].center, [8.0, 23.0, 34.0]);
        assert_eq!(result[0].diameter, 2.5);
        let double_cut = Node::Cut(
            Rc::new(Node::Empty),
            Rc::new(Node::Cut(Rc::new(Node::Empty), bore)),
        );
        result.clear();
        collect_holes(&double_cut, false, 1.0, [0.0; 3], &mut result);
        assert!(result.is_empty());
    }
}
