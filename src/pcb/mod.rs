pub mod autorouter;
pub mod components;
pub mod dsn;
pub mod export;
pub mod footprints;
pub mod header;
pub mod heater;
pub mod libraries;
pub mod nets;
pub mod outline;
pub mod routing;
pub mod ses;
pub mod silkscreen;
pub mod zones;

use std::sync::atomic::{AtomicU64, Ordering};

// Deterministic UUID generator — produces unique UUIDs for each KiCad element.
static UUID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn next_uuid() -> String {
    let n = UUID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!(
        "{:08x}-{:04x}-4{:03x}-8{:03x}-{:012x}",
        (n >> 32) as u32,
        ((n >> 16) & 0xFFFF) as u16,
        (n & 0xFFF) as u16,
        ((n >> 12) & 0xFFF) as u16,
        n & 0xFFFF_FFFF_FFFF
    )
}

pub struct Component {
    pub reference: &'static str,
    pub value: &'static str,
    pub footprint_lib: &'static str,
    pub footprint_name: &'static str,
    pub lcsc: &'static str,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub layer: &'static str,
    pub description: &'static str,
    pub pads: Vec<Pad>,
}

pub struct Pad {
    pub number: &'static str,
    pub pad_type: &'static str,
    pub shape: &'static str,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub layers: &'static str,
    pub net_id: u32,
    pub net_name: &'static str,
    pub drill: Option<f64>,
}

/// Compute absolute board position of a pad given component placement and pad local offset.
/// KiCad rotation is degrees CCW visually (CW mathematically since Y points down).
pub fn pad_abs_pos(comp: &Component, pad_idx: usize) -> (f64, f64) {
    let pad = &comp.pads[pad_idx];
    let theta = (-comp.rotation).to_radians();
    let (sin, cos) = theta.sin_cos();
    let ax = comp.x + pad.x * cos - pad.y * sin;
    let ay = comp.y + pad.x * sin + pad.y * cos;
    (
        (ax * 10000.0).round() / 10000.0,
        (ay * 10000.0).round() / 10000.0,
    )
}

/// Find pad index by its number string; panics if not found.
pub fn pad_idx(comp: &Component, number: &str) -> usize {
    comp.pads
        .iter()
        .position(|p| p.number == number)
        .unwrap_or_else(|| panic!("{} has no pad \"{}\"", comp.reference, number))
}

pub struct HeaterStats {
    pub passes: usize,
    pub length_mm: f64,
    pub resistance: f64,
}

/// Write a single trace segment
#[allow(clippy::too_many_arguments)]
pub fn write_trace(
    pcb: &mut String,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    width: f64,
    layer: &str,
    net: u32,
    _net_name: &str,
) {
    use std::fmt::Write;
    writeln!(
        pcb,
        "  (segment (start {} {}) (end {} {}) (width {}) (layer \"{}\") (net {}) (tstamp \"{}\"))",
        x1,
        y1,
        x2,
        y2,
        width,
        layer,
        net,
        next_uuid()
    )
    .unwrap();
}

/// Write a via
pub fn write_via(pcb: &mut String, x: f64, y: f64, size: f64, drill: f64, net: u32) {
    use std::fmt::Write;
    writeln!(
        pcb,
        "  (via (at {} {}) (size {}) (drill {}) (layers \"F.Cu\" \"B.Cu\") (net {}) (tstamp \"{}\"))",
        x, y, size, drill, net, next_uuid()
    ).unwrap();
}
