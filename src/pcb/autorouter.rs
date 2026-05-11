use super::nets::*;
use super::{pad_abs_pos, pad_idx, write_trace, write_via, Component};
use crate::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

// Grid resolution: 0.1mm per cell — 2 cells = 0.2mm clearance
const GRID_RES: f64 = 0.1;
const BOARD_W: f64 = 100.0;
const BOARD_H: f64 = 80.0;
const GW: usize = (BOARD_W / GRID_RES) as usize; // 1000
const GH: usize = (BOARD_H / GRID_RES) as usize; // 800
const NUM_LAYERS: usize = 2;
const TOTAL_CELLS: usize = GW * GH * NUM_LAYERS;

const LAYER_F: usize = 0;
const LAYER_B: usize = 1;
const VIA_COST: u32 = 40;
const EDGE_CLEAR: f64 = 0.5;

fn to_grid(mm: f64) -> i32 {
    (mm / GRID_RES).round() as i32
}
fn to_grid_ceil(mm: f64) -> i32 {
    (mm / GRID_RES).ceil() as i32
}
fn to_mm(g: i32) -> f64 {
    g as f64 * GRID_RES
}
fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && (x as usize) < GW && (y as usize) < GH
}
fn cell_idx(x: i32, y: i32, layer: usize) -> usize {
    layer * GW * GH + y as usize * GW + x as usize
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32,
    layer: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    f: u32,
    cell: Cell,
}
impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering {
        o.f.cmp(&self.f)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

struct Grid {
    blocked: Vec<bool>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            blocked: vec![false; TOTAL_CELLS],
        }
    }
    fn is_blocked(&self, c: Cell) -> bool {
        if !in_bounds(c.x, c.y) {
            return true;
        }
        self.blocked[cell_idx(c.x, c.y, c.layer)]
    }
    fn set(&mut self, x: i32, y: i32, layer: usize, val: bool) {
        if in_bounds(x, y) {
            self.blocked[cell_idx(x, y, layer)] = val;
        }
    }
    fn block_rect(&mut self, layer: usize, cx: f64, cy: f64, w: f64, h: f64, clearance: f64) {
        let hw = w / 2.0 + clearance;
        let hh = h / 2.0 + clearance;
        // Floor for min, ceil for max → conservative blocking
        let x0 = ((cx - hw) / GRID_RES).floor() as i32;
        let y0 = ((cy - hh) / GRID_RES).floor() as i32;
        let x1 = ((cx + hw) / GRID_RES).ceil() as i32;
        let y1 = ((cy + hh) / GRID_RES).ceil() as i32;
        for yy in y0.max(0)..=y1.min(GW as i32 - 1) {
            for xx in x0.max(0)..=x1.min(GW as i32 - 1) {
                self.set(xx, yy, layer, true);
            }
        }
    }
    fn block_pad(&mut self, comp: &Component, pad: &super::Pad, clearance: f64) {
        let theta = (-comp.rotation).to_radians();
        let (sin, cos) = theta.sin_cos();
        let ax = comp.x + pad.x * cos - pad.y * sin;
        let ay = comp.y + pad.x * sin + pad.y * cos;
        let (w, h) = if comp.rotation.abs() > 0.01 {
            let d = (pad.width * pad.width + pad.height * pad.height).sqrt();
            (d, d)
        } else {
            (pad.width, pad.height)
        };
        let on_f = pad.layers.contains("F.Cu");
        let on_b = pad.layers.contains("B.Cu");
        let thru = pad.drill.is_some();
        if on_f || thru {
            self.block_rect(LAYER_F, ax, ay, w, h, clearance);
        }
        if on_b || thru {
            self.block_rect(LAYER_B, ax, ay, w, h, clearance);
        }
    }
    fn unblock_pad(&mut self, comp: &Component, pad: &super::Pad) {
        let theta = (-comp.rotation).to_radians();
        let (sin, cos) = theta.sin_cos();
        let ax = comp.x + pad.x * cos - pad.y * sin;
        let ay = comp.y + pad.x * sin + pad.y * cos;
        let (w, h) = if comp.rotation.abs() > 0.01 {
            let d = (pad.width * pad.width + pad.height * pad.height).sqrt();
            (d, d)
        } else {
            (pad.width, pad.height)
        };
        let hw = w / 2.0;
        let hh = h / 2.0;
        let x0 = to_grid(ax - hw).max(0);
        let y0 = to_grid(ay - hh).max(0);
        let x1 = to_grid(ax + hw).min(GW as i32 - 1);
        let y1 = to_grid(ay + hh).min(GH as i32 - 1);
        let on_f = pad.layers.contains("F.Cu");
        let on_b = pad.layers.contains("B.Cu");
        let thru = pad.drill.is_some();
        for yy in y0..=y1 {
            for xx in x0..=x1 {
                if on_f || thru {
                    self.set(xx, yy, LAYER_F, false);
                }
                if on_b || thru {
                    self.set(xx, yy, LAYER_B, false);
                }
            }
        }
    }
}

struct TraceSegment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: usize,
    width: f64,
}

/// Block a trace path on the grid as an obstacle (used for pre-blocking manual connections).
/// Uses the same hw formula as mark_trace to ensure consistent clearance.
fn block_trace_path(g: &mut Grid, x1: f64, y1: f64, x2: f64, y2: f64, width: f64, layer: usize) {
    let hw = width / 2.0 + MIN_CLEARANCE;
    if (x1 - x2).abs() < 0.001 {
        let x = to_grid(x1);
        let y0 = to_grid(y1.min(y2));
        let y1g = to_grid(y1.max(y2));
        let xw = to_grid_ceil(hw);
        for y in y0..=y1g {
            for dx in -xw..=xw {
                g.set(x + dx, y, layer, true);
            }
        }
    } else if (y1 - y2).abs() < 0.001 {
        let y = to_grid(y1);
        let x0 = to_grid(x1.min(x2));
        let x1g = to_grid(x1.max(x2));
        let yw = to_grid_ceil(hw);
        for x in x0..=x1g {
            for dy in -yw..=yw {
                g.set(x, y + dy, layer, true);
            }
        }
    } else {
        let x0 = to_grid(x1.min(x2) - hw);
        let x1g = to_grid(x1.max(x2) + hw);
        let y0 = to_grid(y1.min(y2) - hw);
        let y1g = to_grid(y1.max(y2) + hw);
        for y in y0..=y1g {
            for x in x0..=x1g {
                g.set(x, y, layer, true);
            }
        }
    }
}

/// Block a via location on both layers
fn block_via_point(g: &mut Grid, x: f64, y: f64) {
    let r = to_grid_ceil(VIA_PAD / 2.0 + MIN_CLEARANCE);
    let cx = to_grid(x);
    let cy = to_grid(y);
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                g.set(cx + dx, cy + dy, LAYER_F, true);
                g.set(cx + dx, cy + dy, LAYER_B, true);
            }
        }
    }
}

fn build_grid(components: &[Component], exclude_nets: &HashSet<u32>) -> Grid {
    let mut g = Grid::new();
    // Board edges
    let ec = to_grid(EDGE_CLEAR);
    for y in 0..GH as i32 {
        for x in 0..GW as i32 {
            if x < ec || x >= GW as i32 - ec || y < ec || y >= GH as i32 - ec {
                g.set(x, y, LAYER_F, true);
                g.set(x, y, LAYER_B, true);
            }
        }
    }
    // Mounting holes
    for (hx, hy) in [(5.0, 5.0), (95.0, 5.0), (5.0, 75.0), (95.0, 75.0)] {
        g.block_rect(LAYER_F, hx, hy, 6.35, 6.35, MIN_CLEARANCE);
        g.block_rect(LAYER_B, hx, hy, 6.35, 6.35, MIN_CLEARANCE);
    }
    // Component pads not in excluded set
    for comp in components {
        for pad in &comp.pads {
            if pad.net_id != NET_UNCONNECTED && !exclude_nets.contains(&pad.net_id) {
                g.block_pad(comp, pad, MIN_CLEARANCE);
            }
        }
    }

    // Pre-block manual connection paths — these are written by
    // write_manual_connections() AFTER autorouting, so the autorouter must
    // avoid them to prevent shorts, crossings, and clearance violations.
    let find = |r: &str| components.iter().find(|c| c.reference == r).unwrap();
    let ap = |c: &Component, p: &str| pad_abs_pos(c, pad_idx(c, p));
    let q1 = find("Q1");
    let d9 = find("D9");
    let j4 = find("J4");
    let (qx, qy) = ap(q1, "3"); // drain
    let (d9x, d9y) = ap(d9, "1"); // anode
    let (jx, jy) = ap(j4, "2"); // HEATER_P pad
    let pw = POWER_TRACE_WIDTH;

    // ── HEATER_P pre-blocking: J4 pin2 → D9 anode + Q1 drain (F.Cu only) ──
    // F.Cu: J4 pin2 east to x=88
    block_trace_path(&mut g, jx, jy, 88.0, jy, pw, LAYER_F);
    // F.Cu: south to D9 anode
    block_trace_path(&mut g, 88.0, jy, 88.0, d9y, pw, LAYER_F);
    block_trace_path(&mut g, 88.0, d9y, d9x, d9y, pw, LAYER_F);
    // F.Cu: branch to Q1 drain
    block_trace_path(&mut g, 88.0, jy, 88.0, 14.0, pw, LAYER_F);
    block_trace_path(&mut g, 88.0, 14.0, qx, 14.0, pw, LAYER_F);
    block_trace_path(&mut g, qx, 14.0, qx, qy, pw, LAYER_F);

    // ── VBUS pre-blocking: VBUS bus → D14 → +5V ──
    // B.Cu: south then east to via
    block_trace_path(&mut g, 15.0, 6.0, 15.0, 1.5, pw, LAYER_B);
    block_trace_path(&mut g, 15.0, 1.5, 73.0, 1.5, pw, LAYER_B);
    // F.Cu: via south to D14 anode
    block_trace_path(&mut g, 73.0, 1.5, 73.0, 2.0, pw, LAYER_F);
    // F.Cu: D14 cathode → +5V bus
    block_trace_path(&mut g, 77.0, 2.0, 77.0, 5.0, pw, LAYER_F);
    block_trace_path(&mut g, 77.0, 5.0, 80.5, 5.0, pw, LAYER_F);
    // Via
    block_via_point(&mut g, 73.0, 1.5);

    // GND stitching vias
    for &(x, y) in &[
        (83.0, 2.0),
        (97.0, 2.0),
        (0.5, 20.0),
        (93.0, 20.0),
        (0.5, 45.0),
        (93.0, 46.0),
        (93.0, 47.0),
        (97.0, 47.0),
        (3.0, 74.0),
        (50.0, 74.0),
        (97.0, 74.0),
        (50.0, 78.0),
    ] {
        block_via_point(&mut g, x, y);
    }

    g
}

/// Route between two pins. `targets` is a set of cell indices that are valid
/// endpoints (the entire pad area of the destination, not just center).
/// `sources` is the set of cell indices for the source pad area.
fn route_two_pins(
    grid: &Grid,
    src: Cell,
    dst: Cell,
    src_zone: &HashSet<usize>,
    dst_zone: &HashSet<usize>,
) -> Option<Vec<Cell>> {
    let h = |c: Cell| -> u32 {
        let dx = (c.x - dst.x).unsigned_abs();
        let dy = (c.y - dst.y).unsigned_abs();
        dx + dy + if c.layer != dst.layer { VIA_COST } else { 0 }
    };

    let mut g_cost = vec![u32::MAX; TOTAL_CELLS];
    let mut prev = vec![u32::MAX; TOTAL_CELLS];
    let mut heap = BinaryHeap::new();

    let src_idx = cell_idx(src.x, src.y, src.layer);
    g_cost[src_idx] = 0;
    heap.push(State {
        f: h(src),
        cell: src,
    });

    while let Some(State { f, cell }) = heap.pop() {
        let ci = cell_idx(cell.x, cell.y, cell.layer);

        // Reached destination zone?
        if dst_zone.contains(&ci) {
            let mut path = vec![cell];
            let mut cur_ci = ci;
            loop {
                if prev[cur_ci] == u32::MAX {
                    break;
                }
                let pi = prev[cur_ci] as usize;
                let pl = pi / (GW * GH);
                let rem = pi % (GW * GH);
                let py = (rem / GW) as i32;
                let px = (rem % GW) as i32;
                path.push(Cell {
                    x: px,
                    y: py,
                    layer: pl,
                });
                cur_ci = pi;
            }
            path.reverse();
            return Some(path);
        }

        let g = g_cost[ci];
        if f > g.saturating_add(h(cell)) {
            continue;
        }

        for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let nx = cell.x + dx;
            let ny = cell.y + dy;
            if !in_bounds(nx, ny) {
                continue;
            }
            let next = Cell {
                x: nx,
                y: ny,
                layer: cell.layer,
            };
            let ni = cell_idx(nx, ny, cell.layer);
            // Allow movement through source and destination pad zones even if blocked
            let passable =
                !grid.is_blocked(next) || dst_zone.contains(&ni) || src_zone.contains(&ni);
            if !passable {
                continue;
            }
            let new_g = g + 1;
            if new_g < g_cost[ni] {
                g_cost[ni] = new_g;
                prev[ni] = ci as u32;
                heap.push(State {
                    f: new_g + h(next),
                    cell: next,
                });
            }
        }
        // Via
        let ol = 1 - cell.layer;
        let vc = Cell {
            x: cell.x,
            y: cell.y,
            layer: ol,
        };
        let vi = cell_idx(cell.x, cell.y, ol);
        let via_passable = !grid.is_blocked(vc) || dst_zone.contains(&vi) || src_zone.contains(&vi);
        if via_passable {
            let new_g = g + VIA_COST;
            if new_g < g_cost[vi] {
                g_cost[vi] = new_g;
                prev[vi] = ci as u32;
                heap.push(State {
                    f: new_g + h(vc),
                    cell: vc,
                });
            }
        }
    }
    None
}

/// Build the set of grid cell indices that form a pad's landing zone
fn pad_zone_inner(comp: &Component, pad: &super::Pad, clearance: f64) -> HashSet<usize> {
    let theta = (-comp.rotation).to_radians();
    let (sin, cos) = theta.sin_cos();
    let ax = comp.x + pad.x * cos - pad.y * sin;
    let ay = comp.y + pad.x * sin + pad.y * cos;
    let (w, h) = if comp.rotation.abs() > 0.01 {
        let d = (pad.width * pad.width + pad.height * pad.height).sqrt();
        (d, d)
    } else {
        (pad.width, pad.height)
    };
    let on_f = pad.layers.contains("F.Cu");
    let on_b = pad.layers.contains("B.Cu");
    let thru = pad.drill.is_some();
    let hw = w / 2.0 + clearance;
    let hh = h / 2.0 + clearance;
    let x0 = to_grid(ax - hw).max(0);
    let y0 = to_grid(ay - hh).max(0);
    let x1 = to_grid(ax + hw).min(GW as i32 - 1);
    let y1 = to_grid(ay + hh).min(GH as i32 - 1);
    let mut zone = HashSet::new();
    for yy in y0..=y1 {
        for xx in x0..=x1 {
            if on_f || thru {
                zone.insert(cell_idx(xx, yy, LAYER_F));
            }
            if on_b || thru {
                zone.insert(cell_idx(xx, yy, LAYER_B));
            }
        }
    }
    zone
}

fn pad_zone(comp: &Component, pad: &super::Pad) -> HashSet<usize> {
    pad_zone_inner(comp, pad, 0.0)
}

fn path_to_segments(path: &[Cell], width: f64) -> (Vec<TraceSegment>, Vec<(f64, f64)>) {
    let mut segs = Vec::new();
    let mut vias = Vec::new();
    if path.len() < 2 {
        return (segs, vias);
    }

    let mut start = 0;
    for i in 1..path.len() {
        let p = path[i - 1];
        let c = path[i];
        if p.layer != c.layer {
            // Emit segment up to previous cell
            if start < i - 1 || (start == i - 1 && (path[start].x != p.x || path[start].y != p.y)) {
                let s = path[start];
                segs.push(TraceSegment {
                    x1: to_mm(s.x),
                    y1: to_mm(s.y),
                    x2: to_mm(p.x),
                    y2: to_mm(p.y),
                    layer: s.layer,
                    width,
                });
            }
            vias.push((to_mm(c.x), to_mm(c.y)));
            start = i;
        } else if i + 1 < path.len() && path[i + 1].layer == c.layer {
            let dx1 = c.x - p.x;
            let dy1 = c.y - p.y;
            let dx2 = path[i + 1].x - c.x;
            let dy2 = path[i + 1].y - c.y;
            if dx1 != dx2 || dy1 != dy2 {
                let s = path[start];
                segs.push(TraceSegment {
                    x1: to_mm(s.x),
                    y1: to_mm(s.y),
                    x2: to_mm(c.x),
                    y2: to_mm(c.y),
                    layer: s.layer,
                    width,
                });
                start = i;
            }
        }
    }
    // Final segment
    let s = path[start];
    let e = path[path.len() - 1];
    if s.x != e.x || s.y != e.y {
        segs.push(TraceSegment {
            x1: to_mm(s.x),
            y1: to_mm(s.y),
            x2: to_mm(e.x),
            y2: to_mm(e.y),
            layer: s.layer,
            width,
        });
    }
    (segs, vias)
}

fn mark_trace(grid: &mut Grid, seg: &TraceSegment) {
    // Add GRID_RES to account for the next trace's half-width eating into
    // the clearance zone. Without this, two 0.5mm power traces end up
    // only 0.1mm apart (0.2mm required). The extra cell guarantees
    // >= MIN_CLEARANCE physical gap between any two trace edges.
    let hw = seg.width / 2.0 + MIN_CLEARANCE + GRID_RES;
    let layer = seg.layer;
    if (seg.x1 - seg.x2).abs() < 0.001 {
        let x = to_grid(seg.x1);
        let y0 = to_grid(seg.y1.min(seg.y2) - hw);
        let y1 = to_grid(seg.y1.max(seg.y2) + hw);
        let xw = to_grid_ceil(hw);
        for y in y0..=y1 {
            for dx in -xw..=xw {
                grid.set(x + dx, y, layer, true);
            }
        }
    } else if (seg.y1 - seg.y2).abs() < 0.001 {
        let y = to_grid(seg.y1);
        let x0 = to_grid(seg.x1.min(seg.x2) - hw);
        let x1 = to_grid(seg.x1.max(seg.x2) + hw);
        let yw = to_grid_ceil(hw);
        for x in x0..=x1 {
            for dy in -yw..=yw {
                grid.set(x, y + dy, layer, true);
            }
        }
    } else {
        let x0 = to_grid(seg.x1.min(seg.x2) - hw);
        let x1 = to_grid(seg.x1.max(seg.x2) + hw);
        let y0 = to_grid(seg.y1.min(seg.y2) - hw);
        let y1 = to_grid(seg.y1.max(seg.y2) + hw);
        for y in y0..=y1 {
            for x in x0..=x1 {
                grid.set(x, y, layer, true);
            }
        }
    }
}

fn mark_via(grid: &mut Grid, x: f64, y: f64) {
    let r = to_grid_ceil(VIA_PAD / 2.0 + MIN_CLEARANCE + GRID_RES);
    let cx = to_grid(x);
    let cy = to_grid(y);
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                grid.set(cx + dx, cy + dy, LAYER_F, true);
                grid.set(cx + dx, cy + dy, LAYER_B, true);
            }
        }
    }
}

struct NetRoute {
    net_id: u32,
    net_name: &'static str,
    pins: Vec<(f64, f64, usize)>,
    is_power: bool,
}

fn collect_nets(components: &[Component]) -> Vec<NetRoute> {
    let mut pin_map: HashMap<u32, Vec<(f64, f64, usize, &'static str)>> = HashMap::new();
    for comp in components {
        for (i, pad) in comp.pads.iter().enumerate() {
            if pad.net_id == NET_UNCONNECTED || pad.net_id == NET_GND {
                continue;
            }
            let (ax, ay) = pad_abs_pos(comp, i);
            let layer = if pad.layers.contains("F.Cu") {
                LAYER_F
            } else {
                LAYER_B
            };
            pin_map
                .entry(pad.net_id)
                .or_default()
                .push((ax, ay, layer, pad.net_name));
        }
    }
    let power: HashSet<u32> = [NET_12V, NET_12V_RAW, NET_5V, NET_3V3, NET_VBUS]
        .iter()
        .copied()
        .collect();
    let mut nets: Vec<NetRoute> = pin_map
        .into_iter()
        .filter(|(_, pins)| pins.len() >= 2)
        .map(|(id, pins)| {
            let name = pins[0].3;
            let coords: Vec<_> = pins.iter().map(|p| (p.0, p.1, p.2)).collect();
            NetRoute {
                net_id: id,
                net_name: name,
                pins: coords,
                is_power: power.contains(&id),
            }
        })
        .collect();
    // Routing order is critical.
    // 1. Very short stubs (< 5mm Manhattan) — these are direct pad-to-pad and
    //    must go first before long routes block their tiny corridors.
    // 2. Power nets — wide traces, need room.
    // 3. Long signal nets — route while board is still open.
    // 4. Medium signal nets last.
    let span = |n: &NetRoute| -> f64 {
        let (mut xmin, mut xmax) = (f64::MAX, f64::MIN);
        let (mut ymin, mut ymax) = (f64::MAX, f64::MIN);
        for p in &n.pins {
            xmin = xmin.min(p.0);
            xmax = xmax.max(p.0);
            ymin = ymin.min(p.1);
            ymax = ymax.max(p.1);
        }
        (xmax - xmin) + (ymax - ymin)
    };
    nets.sort_by(|a, b| {
        let sa = span(a);
        let sb = span(b);
        let a_stub = sa < 5.0;
        let b_stub = sb < 5.0;
        // Stubs first
        b_stub
            .cmp(&a_stub)
            // Then power nets
            .then(b.is_power.cmp(&a.is_power))
            // Then longest first
            .then(sb.partial_cmp(&sa).unwrap_or(Ordering::Equal))
    });
    nets
}

pub fn write_manual_connections(pcb: &mut String, components: &[Component]) {
    let pw = POWER_TRACE_WIDTH;
    let find = |r: &str| -> &Component { components.iter().find(|c| c.reference == r).unwrap() };
    let ap = |c: &Component, p: &str| -> (f64, f64) { pad_abs_pos(c, pad_idx(c, p)) };

    let q1 = find("Q1");
    let d9 = find("D9");
    let j4 = find("J4");
    let d14 = find("D14");
    let (qx, qy) = ap(q1, "3"); // drain
    let (d9x, d9y) = ap(d9, "1"); // anode
    let (jx, jy) = ap(j4, "2"); // HEATER_P pad
    let (d14_ax, d14_ay) = ap(d14, "1"); // anode (VBUS)
    let (d14_cx, d14_cy) = ap(d14, "2"); // cathode (+5V)

    // ── HEATER_P: J4 pin2 → D9 anode + Q1 drain (F.Cu only) ──
    // J4 is the external heater connector. HEATER_P routes on F.Cu
    // east to x=88, then branches south to D9 and north to Q1.
    // F.Cu: J4 pin2 east to x=88
    write_trace(pcb, jx, jy, 88.0, jy, pw, "F.Cu", NET_HEATER_P, "HEATER_P");
    // F.Cu: south to D9 anode
    write_trace(
        pcb,
        88.0,
        jy,
        88.0,
        d9y,
        pw,
        "F.Cu",
        NET_HEATER_P,
        "HEATER_P",
    );
    write_trace(
        pcb,
        88.0,
        d9y,
        d9x,
        d9y,
        pw,
        "F.Cu",
        NET_HEATER_P,
        "HEATER_P",
    );
    // F.Cu: branch north to Q1 drain
    write_trace(
        pcb,
        88.0,
        jy,
        88.0,
        14.0,
        pw,
        "F.Cu",
        NET_HEATER_P,
        "HEATER_P",
    );
    write_trace(
        pcb,
        88.0,
        14.0,
        qx,
        14.0,
        pw,
        "F.Cu",
        NET_HEATER_P,
        "HEATER_P",
    );
    write_trace(pcb, qx, 14.0, qx, qy, pw, "F.Cu", NET_HEATER_P, "HEATER_P");

    // ── VBUS → D14 power path ──
    // B.Cu: VBUS bus south then east to via
    write_trace(pcb, 15.0, 6.0, 15.0, 1.5, pw, "B.Cu", NET_VBUS, "VBUS");
    write_trace(pcb, 15.0, 1.5, 73.0, 1.5, pw, "B.Cu", NET_VBUS, "VBUS");
    write_via(pcb, 73.0, 1.5, VIA_PAD, VIA_DRILL, NET_VBUS);
    // F.Cu: via south to D14 anode
    write_trace(pcb, 73.0, 1.5, d14_ax, d14_ay, pw, "F.Cu", NET_VBUS, "VBUS");
    // D14 cathode → +5V bus
    write_trace(pcb, d14_cx, d14_cy, d14_cx, 5.0, pw, "F.Cu", NET_5V, "+5V");
    write_trace(pcb, d14_cx, 5.0, 80.5, 5.0, pw, "F.Cu", NET_5V, "+5V");
}

pub fn write_gnd_stitching(pcb: &mut String) {
    for &(x, y) in &[
        (83.0, 2.0),
        (97.0, 2.0),
        (0.5, 20.0),
        (93.0, 20.0),
        (0.5, 45.0),
        (93.0, 46.0),
        (93.0, 47.0),
        (97.0, 47.0),
        (3.0, 74.0),
        (50.0, 74.0),
        (97.0, 74.0),
        (50.0, 78.0),
    ] {
        write_via(pcb, x, y, VIA_PAD, VIA_DRILL, NET_GND);
    }
}

pub fn write_autorouted_traces(pcb: &mut String, components: &[Component]) {
    println!("\n── Autorouter ──");
    println!(
        "Grid: {}x{}x2 = {} cells ({:.1} MB)",
        GW,
        GH,
        TOTAL_CELLS,
        TOTAL_CELLS as f64 / 1024.0 / 1024.0
    );

    let skip: HashSet<u32> = [NET_GND, NET_HEATER_P, NET_VBUS].iter().copied().collect();
    let nets = collect_nets(components);
    let routeable: Vec<&NetRoute> = nets.iter().filter(|n| !skip.contains(&n.net_id)).collect();
    println!(
        "Nets to route: {} ({} skipped)",
        routeable.len(),
        nets.len() - routeable.len()
    );

    // Build grid — exclude all routeable nets so their pads aren't obstacles
    let all_ids: HashSet<u32> = routeable.iter().map(|n| n.net_id).collect();
    let mut grid = build_grid(components, &all_ids);

    let mut routed = 0;
    let mut failed = 0;
    let mut total_segs = 0;
    let mut total_vias = 0;

    for net in &routeable {
        let width = if net.is_power {
            POWER_TRACE_WIDTH
        } else {
            SIGNAL_TRACE_WIDTH
        };

        // Unblock this net's pads
        for comp in components {
            for pad in &comp.pads {
                if pad.net_id == net.net_id {
                    grid.unblock_pad(comp, pad);
                }
            }
        }

        // Route using nearest-neighbor MST
        let mut all_segs = Vec::new();
        let mut all_vias = Vec::new();
        // Store (x, y, layer, original_pin_index)
        let mut connected: Vec<(f64, f64, usize, usize)> =
            vec![(net.pins[0].0, net.pins[0].1, net.pins[0].2, 0)];
        let mut remaining: Vec<(f64, f64, usize, usize)> = net.pins[1..]
            .iter()
            .enumerate()
            .map(|(i, p)| (p.0, p.1, p.2, i + 1))
            .collect();
        let mut success = true;

        // Pre-build pad zones for all pins of this net
        let mut pin_zones: Vec<HashSet<usize>> = Vec::new();
        for (px, py, _pl) in &net.pins {
            let mut zone = HashSet::new();
            for comp in components {
                for (pi, pad) in comp.pads.iter().enumerate() {
                    if pad.net_id == net.net_id {
                        let (ax, ay) = pad_abs_pos(comp, pi);
                        if (ax - px).abs() < 0.2 && (ay - py).abs() < 0.2 {
                            zone = pad_zone(comp, pad);
                        }
                    }
                }
            }
            pin_zones.push(zone);
        }

        while !remaining.is_empty() {
            let mut best_ri = 0;
            let mut best_ci = 0;
            let mut best_d = f64::MAX;
            for (ri, rp) in remaining.iter().enumerate() {
                for (ci, cp) in connected.iter().enumerate() {
                    let d = (rp.0 - cp.0).abs() + (rp.1 - cp.1).abs();
                    if d < best_d {
                        best_d = d;
                        best_ri = ri;
                        best_ci = ci;
                    }
                }
            }
            let target = remaining.remove(best_ri);
            let source = connected[best_ci];

            let src = Cell {
                x: to_grid(source.0),
                y: to_grid(source.1),
                layer: source.2,
            };
            let dst = Cell {
                x: to_grid(target.0),
                y: to_grid(target.1),
                layer: target.2,
            };

            // Source zone = source pad + all already-connected pads
            let mut src_zone = HashSet::new();
            src_zone.extend(pin_zones[source.3].iter());
            for cp in &connected {
                src_zone.extend(pin_zones[cp.3].iter());
            }
            // Destination zone = only the target pad
            let dst_zone = &pin_zones[target.3];

            match route_two_pins(&grid, src, dst, &src_zone, dst_zone) {
                Some(path) => {
                    let (mut segs, vias) = path_to_segments(&path, width);
                    // Snap trace endpoints to actual pad positions — the grid
                    // may not perfectly align with pad centers, causing dangling
                    // track DRC violations if the trace ends 0.05mm from the pad.
                    if let Some(first) = segs.first_mut() {
                        first.x1 = source.0;
                        first.y1 = source.1;
                    }
                    if let Some(last) = segs.last_mut() {
                        last.x2 = target.0;
                        last.y2 = target.1;
                    }
                    for s in &segs {
                        mark_trace(&mut grid, s);
                    }
                    for &(vx, vy) in &vias {
                        mark_via(&mut grid, vx, vy);
                    }
                    all_segs.extend(segs);
                    all_vias.extend(vias);
                    connected.push(target);
                }
                None => {
                    eprintln!(
                        "  FAIL: {} ({:.1},{:.1})->({:.1},{:.1})",
                        net.net_name, source.0, source.1, target.0, target.1
                    );
                    success = false;
                    break;
                }
            }
        }

        // Always write accumulated segments — even on partial failure, the
        // successful pin-pair routes are valid. Manual routes in routing.rs
        // cover the gaps for any failed segments.
        let ln = |l: usize| if l == LAYER_F { "F.Cu" } else { "B.Cu" };
        for s in &all_segs {
            write_trace(
                pcb,
                s.x1,
                s.y1,
                s.x2,
                s.y2,
                s.width,
                ln(s.layer),
                net.net_id,
                net.net_name,
            );
        }
        for &(vx, vy) in &all_vias {
            write_via(pcb, vx, vy, VIA_PAD, VIA_DRILL, net.net_id);
        }
        total_segs += all_segs.len();
        total_vias += all_vias.len();
        if success {
            routed += 1;
            println!(
                "  {} [{}p]: {} segs, {} vias",
                net.net_name,
                net.pins.len(),
                all_segs.len(),
                all_vias.len()
            );
        } else {
            failed += 1;
        }

        // Re-block this net's pads for future nets' clearance
        for comp in components {
            for pad in &comp.pads {
                if pad.net_id == net.net_id {
                    grid.block_pad(comp, pad, MIN_CLEARANCE);
                }
            }
        }
    }

    println!(
        "\nResult: {}/{} routed, {} failed, {} segments, {} vias",
        routed,
        routeable.len(),
        failed,
        total_segs,
        total_vias
    );

    write_manual_connections(pcb, components);
    write_gnd_stitching(pcb);
    pcb.push('\n');
}
