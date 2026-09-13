//! One construction tree drives both review meshes and analytic manufacturing solids.
use std::{
    ops::{Add, Deref, Sub},
    rc::Rc,
};
#[derive(Clone)]
pub enum Primitive {
    Box([f64; 3], [f64; 3]),
    Cylinder([f64; 3], f64, f64, usize),
}
#[derive(Clone)]
enum Node {
    Empty,
    Primitive(Primitive),
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
    pub fn primitive(mesh: vcad::Part, p: Primitive) -> Self {
        Self {
            mesh,
            node: Rc::new(Node::Primitive(p)),
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
        Node::Primitive(Primitive::Box(a, b)) => {
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
        Node::Primitive(Primitive::Cylinder(c, r, h, axis)) => {
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
