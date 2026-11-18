//! Adjustable dry fixture only: no pump, seal, connector or wetted channel is modeled.
use clap::Parser;
use laminarforge_cad::{
    runtime_cad::{hash, mesh, Args},
    water_transfer_preview::preview,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::Path};
use vcad::{centered_cube, centered_cylinder, Part};

const MODEL: &str = "water_transfer_dock";
const EPS: f64 = 0.02;
const FILES: [&str; 5] = [
    "water_transfer_dock_tray_PRINT.stl",
    "water_transfer_dock_coupon_cradle_PRINT.stl",
    "water_transfer_dock_tube_guide_PRINT.stl",
    "water_transfer_dock_assembly_DRY_with_REF_coupon.stl",
    "water_transfer_dock_REF_coupon_NOT_FOR_MANUFACTURE.stl",
];

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    coupon_length_mm: f64,
    coupon_width_mm: f64,
    coupon_thickness_mm: f64,
    coupon_clearance_per_side_mm: f64,
    coupon_support_overlap_mm: f64,
    cradle_wall_mm: f64,
    cradle_support_thickness_mm: f64,
    cradle_standoff_mm: f64,
    cradle_lip_above_coupon_mm: f64,
    cradle_foot_length_mm: f64,
    cradle_foot_width_mm: f64,
    socket_clearance_per_side_mm: f64,
    socket_depth_mm: f64,
    socket_bottom_above_floor_mm: f64,
    socket_wall_mm: f64,
    tray_length_mm: f64,
    tray_width_mm: f64,
    tray_wall_mm: f64,
    tray_floor_mm: f64,
    tray_spill_depth_mm: f64,
    tube_outer_diameter_mm: f64,
    tube_diametral_clearance_mm: f64,
    guide_length_mm: f64,
    guide_base_width_mm: f64,
    guide_base_thickness_mm: f64,
    guide_body_width_mm: f64,
    guide_under_saddle_web_mm: f64,
    guide_top_wall_mm: f64,
    guide_mount_pitch_mm: f64,
    tie_passage_width_mm: f64,
    tie_passage_height_mm: f64,
    tie_passage_floor_above_base_mm: f64,
    mount_hole_diameter_mm: f64,
    mount_pad_height_mm: f64,
    nut_pocket_across_flats_mm: f64,
    nut_pocket_depth_mm: f64,
    fastener_length_mm: f64,
    nut_thickness_mm: f64,
}

#[derive(Serialize)]
struct Derived {
    cradle_outer_mm: [f64; 2],
    coupon_pocket_mm: [f64; 2],
    viewing_window_mm: [f64; 2],
    cradle_height_mm: f64,
    cradle_foot_centers_mm: [[f64; 2]; 4],
    socket_opening_mm: [f64; 2],
    socket_boss_mm: [f64; 2],
    socket_bottom_z_mm: f64,
    socket_boss_top_z_mm: f64,
    cradle_assembly_z_mm: f64,
    coupon_assembly_z_mm: f64,
    tray_inner_mm: [f64; 2],
    tray_rim_z_mm: f64,
    guide_mount_x_mm: f64,
    guide_saddle_diameter_mm: f64,
    guide_saddle_center_z_mm: f64,
    guide_saddle_bottom_z_mm: f64,
    guide_height_mm: f64,
    tie_passage_bottom_z_mm: f64,
    nut_pocket_circumradius_mm: f64,
    fastener_tip_z_mm: f64,
    nut_bottom_z_mm: f64,
    gross_empty_basin_volume_ml: f64,
}

fn finite_numeric_tree(value: &Value) -> bool {
    match value {
        Value::Number(number) => number.as_f64().is_some_and(f64::is_finite),
        Value::Array(values) => values.iter().all(finite_numeric_tree),
        Value::Object(values) => values.values().all(finite_numeric_tree),
        // serde_json converts a nonfinite floating point number into null.
        _ => false,
    }
}

impl Parameters {
    fn derive(&self) -> Derived {
        let outer = [
            self.coupon_length_mm + 2.0 * (self.coupon_clearance_per_side_mm + self.cradle_wall_mm),
            self.coupon_width_mm + 2.0 * (self.coupon_clearance_per_side_mm + self.cradle_wall_mm),
        ];
        let fx = (outer[0] - self.cradle_foot_length_mm) / 2.0;
        let fy = (outer[1] - self.cradle_foot_width_mm) / 2.0;
        let socket = [
            self.cradle_foot_length_mm + 2.0 * self.socket_clearance_per_side_mm,
            self.cradle_foot_width_mm + 2.0 * self.socket_clearance_per_side_mm,
        ];
        let inner = [
            self.tray_length_mm - 2.0 * self.tray_wall_mm,
            self.tray_width_mm - 2.0 * self.tray_wall_mm,
        ];
        let bottom = self.tray_floor_mm + self.socket_bottom_above_floor_mm;
        let bore = self.tube_outer_diameter_mm + self.tube_diametral_clearance_mm;
        let saddle_bottom = self.guide_base_thickness_mm + self.guide_under_saddle_web_mm;
        Derived {
            cradle_outer_mm: outer,
            coupon_pocket_mm: [
                self.coupon_length_mm + 2.0 * self.coupon_clearance_per_side_mm,
                self.coupon_width_mm + 2.0 * self.coupon_clearance_per_side_mm,
            ],
            viewing_window_mm: [
                self.coupon_length_mm - 2.0 * self.coupon_support_overlap_mm,
                self.coupon_width_mm - 2.0 * self.coupon_support_overlap_mm,
            ],
            cradle_height_mm: self.cradle_standoff_mm
                + self.cradle_support_thickness_mm
                + self.coupon_thickness_mm
                + self.cradle_lip_above_coupon_mm,
            cradle_foot_centers_mm: [[-fx, -fy], [-fx, fy], [fx, -fy], [fx, fy]],
            socket_opening_mm: socket,
            socket_boss_mm: [
                socket[0] + 2.0 * self.socket_wall_mm,
                socket[1] + 2.0 * self.socket_wall_mm,
            ],
            socket_bottom_z_mm: bottom,
            socket_boss_top_z_mm: bottom + self.socket_depth_mm,
            cradle_assembly_z_mm: bottom,
            coupon_assembly_z_mm: bottom
                + self.cradle_standoff_mm
                + self.cradle_support_thickness_mm,
            tray_inner_mm: inner,
            tray_rim_z_mm: self.tray_floor_mm + self.tray_spill_depth_mm,
            guide_mount_x_mm: self.tray_length_mm / 2.0 + self.guide_length_mm / 2.0
                - self.tray_wall_mm,
            guide_saddle_diameter_mm: bore,
            guide_saddle_center_z_mm: saddle_bottom + bore / 2.0,
            guide_saddle_bottom_z_mm: saddle_bottom,
            guide_height_mm: saddle_bottom + bore + self.guide_top_wall_mm,
            tie_passage_bottom_z_mm: self.guide_base_thickness_mm
                + self.tie_passage_floor_above_base_mm,
            nut_pocket_circumradius_mm: self.nut_pocket_across_flats_mm / 3.0_f64.sqrt(),
            fastener_tip_z_mm: self.mount_pad_height_mm + self.guide_base_thickness_mm
                - self.fastener_length_mm,
            nut_bottom_z_mm: self.nut_pocket_depth_mm - self.nut_thickness_mm,
            gross_empty_basin_volume_ml: inner[0] * inner[1] * self.tray_spill_depth_mm / 1000.0,
        }
    }

    fn validate(&self) -> Result<Derived, String> {
        let fields = serde_json::to_value(self).map_err(|e| e.to_string())?;
        for (name, value) in fields.as_object().ok_or("parameters are not an object")? {
            if value.as_f64().is_none_or(|x| !x.is_finite() || x <= 0.0) {
                return Err(format!("{name} must be finite positive millimeters"));
            }
        }
        let d = self.derive();
        let require = |ok: bool, message: &str| if ok { Ok(()) } else { Err(message.to_string()) };
        require(
            finite_numeric_tree(&serde_json::to_value(&d).map_err(|e| e.to_string())?),
            "derived dimensions must remain finite before constructing geometry",
        )?;
        require(
            d.cradle_height_mm <= 30.0
                && d.guide_height_mm <= 40.0
                && (self.mount_pad_height_mm + d.guide_height_mm)
                    .max(d.cradle_assembly_z_mm + d.cradle_height_mm)
                    .max(d.tray_rim_z_mm)
                    <= 60.0,
            "derived cradle, guide or assembled height exceeds the reviewed family envelope",
        )?;
        require(
            (50.0..=100.0).contains(&self.coupon_length_mm)
                && (20.0..=40.0).contains(&self.coupon_width_mm)
                && (0.8..=8.0).contains(&self.coupon_thickness_mm),
            "coupon envelope outside reviewed 50–100 × 20–40 × 0.8–8 mm range",
        )?;
        require(
            (0.2..=0.6).contains(&self.coupon_clearance_per_side_mm)
                && (1.5..=3.0).contains(&self.coupon_support_overlap_mm),
            "coupon clearance/support overlap outside reviewed range",
        )?;
        require(
            self.cradle_wall_mm >= 2.4
                && self.cradle_support_thickness_mm >= 2.4
                && self.cradle_standoff_mm >= 3.0
                && self.cradle_lip_above_coupon_mm >= 0.8,
            "cradle wall, support, standoff or lip is too thin",
        )?;
        require(
            self.cradle_wall_mm <= 6.0
                && self.cradle_support_thickness_mm <= 5.0
                && self.cradle_standoff_mm <= 8.0
                && self.cradle_lip_above_coupon_mm <= 2.0
                && self.socket_depth_mm <= 3.0
                && self.socket_bottom_above_floor_mm <= 1.0
                && self.socket_wall_mm <= 3.0
                && self.tray_floor_mm <= 4.0
                && self.tray_wall_mm <= 4.0,
            "cradle/socket/tray dimensions exceed the reviewed compact-fixture envelope",
        )?;
        require(
            d.viewing_window_mm[0] >= 40.0 && d.viewing_window_mm[1] >= 14.0,
            "viewing window too small",
        )?;
        require(
            self.cradle_foot_length_mm >= 6.0
                && self.cradle_foot_length_mm <= d.cradle_outer_mm[0] / 3.0
                && self.cradle_foot_width_mm >= 3.0
                && self.cradle_foot_width_mm
                    <= self.cradle_wall_mm
                        + self.coupon_clearance_per_side_mm
                        + self.coupon_support_overlap_mm
                        - 0.5,
            "feet do not fit on support lands outside the viewing window",
        )?;
        require(
            (0.2..=0.5).contains(&self.socket_clearance_per_side_mm)
                && self.socket_depth_mm >= 1.2
                && self.socket_depth_mm + 0.6 <= self.cradle_standoff_mm
                && self.socket_bottom_above_floor_mm >= 0.4
                && self.socket_wall_mm >= 1.5,
            "socket clearance/depth/wall would prevent stable removable seating",
        )?;
        require(
            self.tray_floor_mm >= 2.4
                && self.tray_wall_mm >= 2.4
                && (5.0..=15.0).contains(&self.tray_spill_depth_mm),
            "tray floor/wall/spill depth outside reviewed range",
        )?;
        require(
            self.tray_length_mm <= 220.0
                && self.tray_width_mm <= 180.0
                && d.tray_inner_mm[0] >= d.cradle_outer_mm[0] + d.socket_boss_mm[0] + 16.0
                && d.tray_inner_mm[1] >= d.cradle_outer_mm[1] + d.socket_boss_mm[1] + 16.0,
            "tray does not clear cradle, socket bosses and finger access",
        )?;
        require(
            d.socket_boss_top_z_mm + 1.0 < d.tray_rim_z_mm,
            "socket bosses must remain below the catch-tray rim",
        )?;
        require(
            (2.0..=8.0).contains(&self.tube_outer_diameter_mm)
                && (0.4..=1.0).contains(&self.tube_diametral_clearance_mm),
            "tube OD or diametral clearance outside reviewed provisional range",
        )?;
        require(
            self.guide_length_mm >= 18.0
                && self.guide_length_mm <= 30.0
                && self.guide_base_thickness_mm >= 3.0
                && self.guide_top_wall_mm >= 1.6
                && self.guide_body_width_mm >= d.guide_saddle_diameter_mm + 4.0,
            "guide envelope leaves insufficient saddle side walls",
        )?;
        require(
            self.guide_base_width_mm <= 44.0
                && self.guide_base_thickness_mm <= 4.0
                && self.guide_body_width_mm <= 20.0
                && self.guide_under_saddle_web_mm <= 8.0
                && self.guide_top_wall_mm <= 3.0
                && self.tie_passage_width_mm <= 4.0
                && self.tie_passage_height_mm <= 2.0
                && self.tie_passage_floor_above_base_mm <= 2.0,
            "guide/tie passage dimensions exceed the reviewed compact-fixture envelope",
        )?;
        require(
            self.tie_passage_width_mm >= 3.0
                && self.tie_passage_height_mm >= 1.5
                && self.tie_passage_floor_above_base_mm >= 1.5
                && self.tie_passage_width_mm + 4.0 <= self.guide_length_mm
                && d.tie_passage_bottom_z_mm + self.tie_passage_height_mm + 1.5
                    <= d.guide_saddle_bottom_z_mm,
            "cable-tie tunnel floor/roof/end walls are too thin",
        )?;
        require(
            (3.3..=3.6).contains(&self.mount_hole_diameter_mm)
                && (5.9..=6.3).contains(&self.nut_pocket_across_flats_mm)
                && (2.8..=3.2).contains(&self.nut_pocket_depth_mm),
            "M3 hole or nut recess outside reviewed range",
        )?;
        require(self.guide_mount_pitch_mm / 2.0 - self.mount_hole_diameter_mm / 2.0 >= self.guide_body_width_mm / 2.0 + 1.0 && self.guide_base_width_mm / 2.0 - self.guide_mount_pitch_mm / 2.0 - d.nut_pocket_circumradius_mm >= 1.5 && self.guide_length_mm / 2.0 - d.nut_pocket_circumradius_mm >= 1.5, "mount holes/nut recesses collide with guide body or leave insufficient pad edge material")?;
        require(
            d.guide_mount_x_mm - d.nut_pocket_circumradius_mm >= self.tray_length_mm / 2.0 + 1.5
                && self.guide_base_width_mm + 8.0 < self.tray_width_mm
                && self.mount_pad_height_mm >= self.nut_pocket_depth_mm + 3.0
                && self.mount_pad_height_mm <= d.tray_rim_z_mm,
            "external pads or nut recesses invade the basin or fail to join the tray wall",
        )?;
        require(
            (self.fastener_length_mm - 12.0).abs() < 1e-9
                && (self.nut_thickness_mm - 2.4).abs() < 1e-9
                && d.nut_bottom_z_mm >= 0.3
                && d.fastener_tip_z_mm >= 0.1
                && d.fastener_tip_z_mm + 0.2 <= d.nut_bottom_z_mm,
            "reviewed M3×12/2.4 mm nut stack must fully engage while remaining above tabletop Z=0",
        )?;
        Ok(d)
    }
}

fn block(name: &str, x: f64, y: f64, height: f64, bottom: f64) -> Part {
    centered_cube(name, x, y, height).translate(0.0, 0.0, bottom + height / 2.0)
}

fn tray(p: &Parameters, d: &Derived) -> Part {
    let mut part = block(
        "closed_floor_catch_tray",
        p.tray_length_mm,
        p.tray_width_mm,
        d.tray_rim_z_mm,
        0.0,
    ) - block(
        "open_basin",
        d.tray_inner_mm[0],
        d.tray_inner_mm[1],
        p.tray_spill_depth_mm + EPS,
        p.tray_floor_mm,
    );
    for sign in [-1.0, 1.0] {
        let x = sign * d.guide_mount_x_mm;
        part = part
            + block(
                "external_guide_mount_pad",
                p.guide_length_mm,
                p.guide_base_width_mm,
                p.mount_pad_height_mm,
                0.0,
            )
            .translate(x, 0.0, 0.0);
        for y in [-p.guide_mount_pitch_mm / 2.0, p.guide_mount_pitch_mm / 2.0] {
            let bore = centered_cylinder(
                "external_pad_M3_clearance",
                p.mount_hole_diameter_mm / 2.0,
                p.mount_pad_height_mm + 2.0 * EPS,
                48,
            )
            .translate(x, y, p.mount_pad_height_mm / 2.0);
            let nut = centered_cylinder(
                "underside_M3_nut_recess",
                d.nut_pocket_circumradius_mm,
                p.nut_pocket_depth_mm + EPS,
                6,
            )
            .rotate(0.0, 0.0, 30.0)
            .translate(x, y, (p.nut_pocket_depth_mm - EPS) / 2.0);
            part = part - bore - nut;
        }
    }
    for [x, y] in d.cradle_foot_centers_mm {
        part = part
            + block(
                "raised_closed_bottom_socket_boss",
                d.socket_boss_mm[0],
                d.socket_boss_mm[1],
                d.socket_boss_top_z_mm - p.tray_floor_mm,
                p.tray_floor_mm,
            )
            .translate(x, y, 0.0)
            - block(
                "gravity_seat_socket",
                d.socket_opening_mm[0],
                d.socket_opening_mm[1],
                p.socket_depth_mm + EPS,
                d.socket_bottom_z_mm,
            )
            .translate(x, y, 0.0);
    }
    part
}

fn cradle(p: &Parameters, d: &Derived) -> Part {
    let seat_z = p.cradle_standoff_mm + p.cradle_support_thickness_mm;
    let mut part = block(
        "coupon_support_ring",
        d.cradle_outer_mm[0],
        d.cradle_outer_mm[1],
        p.cradle_support_thickness_mm,
        p.cradle_standoff_mm,
    ) - block(
        "unobstructed_viewing_window",
        d.viewing_window_mm[0],
        d.viewing_window_mm[1],
        p.cradle_support_thickness_mm + 2.0 * EPS,
        p.cradle_standoff_mm - EPS,
    );
    part = part
        + (block(
            "lateral_coupon_stops_no_clamping",
            d.cradle_outer_mm[0],
            d.cradle_outer_mm[1],
            p.coupon_thickness_mm + p.cradle_lip_above_coupon_mm,
            seat_z,
        ) - block(
            "open_top_coupon_pocket",
            d.coupon_pocket_mm[0],
            d.coupon_pocket_mm[1],
            p.coupon_thickness_mm + p.cradle_lip_above_coupon_mm + 2.0 * EPS,
            seat_z - EPS,
        ));
    for sign in [-1.0, 1.0] {
        part = part
            - block(
                "coupon_end_finger_lift_relief",
                2.0 * (p.cradle_wall_mm + p.coupon_clearance_per_side_mm),
                p.coupon_width_mm / 2.0,
                p.coupon_thickness_mm + p.cradle_lip_above_coupon_mm + EPS,
                seat_z,
            )
            .translate(sign * d.cradle_outer_mm[0] / 2.0, 0.0, 0.0);
    }
    for [x, y] in d.cradle_foot_centers_mm {
        part = part
            + block(
                "removable_cradle_foot",
                p.cradle_foot_length_mm,
                p.cradle_foot_width_mm,
                p.cradle_standoff_mm + EPS,
                0.0,
            )
            .translate(x, y, 0.0);
    }
    part
}

fn guide(p: &Parameters, d: &Derived) -> Part {
    let mut part = block(
        "bolt_down_guide_base",
        p.guide_length_mm,
        p.guide_base_width_mm,
        p.guide_base_thickness_mm,
        0.0,
    ) + block(
        "open_saddle_body",
        p.guide_length_mm,
        p.guide_body_width_mm,
        d.guide_height_mm - p.guide_base_thickness_mm,
        p.guide_base_thickness_mm,
    );
    let saddle = centered_cylinder(
        "oversize_tube_saddle",
        d.guide_saddle_diameter_mm / 2.0,
        p.guide_length_mm + 2.0 * EPS,
        64,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, d.guide_saddle_center_z_mm);
    let opening = block(
        "full_width_open_top_no_snapfit",
        p.guide_length_mm + 2.0 * EPS,
        d.guide_saddle_diameter_mm,
        d.guide_height_mm - d.guide_saddle_center_z_mm + EPS,
        d.guide_saddle_center_z_mm,
    );
    let tie_tunnel = block(
        "transverse_cable_tie_passage",
        p.tie_passage_width_mm,
        p.guide_body_width_mm + 2.0 * EPS,
        p.tie_passage_height_mm,
        d.tie_passage_bottom_z_mm,
    );
    part = part - saddle - opening - tie_tunnel;
    for y in [-p.guide_mount_pitch_mm / 2.0, p.guide_mount_pitch_mm / 2.0] {
        part = part
            - centered_cylinder(
                "guide_M3_through_hole",
                p.mount_hole_diameter_mm / 2.0,
                p.guide_base_thickness_mm + 2.0 * EPS,
                48,
            )
            .translate(0.0, y, p.guide_base_thickness_mm / 2.0);
    }
    part
}

fn coupon(p: &Parameters) -> Part {
    block(
        "REF_provisional_coupon_envelope_NOT_FOR_MANUFACTURE",
        p.coupon_length_mm,
        p.coupon_width_mm,
        p.coupon_thickness_mm,
        0.0,
    )
}

fn build(p: &Parameters, d: &Derived, index: usize) -> Part {
    match index {
        0 => tray(p, d),
        // Rim-down avoids suspending the whole support ring above four feet.
        // The small inward support ledge still requires slicer/physical review.
        1 => cradle(p, d)
            .rotate(180.0, 0.0, 0.0)
            .translate(0.0, 0.0, d.cradle_height_mm),
        2 => guide(p, d),
        3 => {
            tray(p, d)
                + cradle(p, d).translate(0.0, 0.0, d.cradle_assembly_z_mm)
                + guide(p, d).translate(-d.guide_mount_x_mm, 0.0, p.mount_pad_height_mm)
                + guide(p, d).translate(d.guide_mount_x_mm, 0.0, p.mount_pad_height_mm)
                + coupon(p).translate(0.0, 0.0, d.coupon_assembly_z_mm)
        }
        4 => coupon(p),
        _ => unreachable!(),
    }
}

// Ray parity on actual STL triangles; probes deliberately avoid surface boundaries.
// These checkpoints verify named openings/lands, not full manifold or print qualification.
fn contains(bytes: &[u8], point: [f64; 3]) -> bool {
    let ray = [1.0, 0.314159265, 0.271828183];
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let mut distances = Vec::new();
    for triangle in bytes[84..].chunks_exact(50) {
        let vertex = |v: usize| {
            std::array::from_fn::<_, 3, _>(|a| {
                let offset = 12 + 12 * v + 4 * a;
                f32::from_le_bytes(triangle[offset..offset + 4].try_into().unwrap()) as f64
            })
        };
        let v0 = vertex(0);
        let e1 = sub(vertex(1), v0);
        let e2 = sub(vertex(2), v0);
        let h = cross(ray, e2);
        let determinant = dot(e1, h);
        if determinant.abs() < 1e-10 {
            continue;
        }
        let s = sub(point, v0);
        let u = dot(s, h) / determinant;
        if !(-1e-9..=1.0 + 1e-9).contains(&u) {
            continue;
        }
        let q = cross(s, e1);
        let v = dot(ray, q) / determinant;
        if v < -1e-9 || u + v > 1.0 + 1e-9 {
            continue;
        }
        let t = dot(e2, q) / determinant;
        if t > 1e-7 {
            distances.push(t);
        }
    }
    distances.sort_by(f64::total_cmp);
    distances.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
    distances.len() % 2 == 1
}

fn bounds_check(stats: &Value, low: [f64; 3], high: [f64; 3]) -> Result<(), String> {
    for (name, expected) in [("min_mm", low), ("max_mm", high)] {
        let actual = stats[name].as_array().ok_or("missing mesh bounds")?;
        for axis in 0..3 {
            let value = actual[axis].as_f64().ok_or("invalid mesh bound")?;
            if (value - expected[axis]).abs() > 0.002 {
                return Err(format!(
                    "{name}[{axis}]={value} differs from required {}",
                    expected[axis]
                ));
            }
        }
    }
    Ok(())
}

fn interface_checks(p: &Parameters, d: &Derived, meshes: &[Vec<u8>]) -> Result<Vec<Value>, String> {
    let mut checks = Vec::new();
    let mut probe = |output: usize,
                     name: &str,
                     point: [f64; 3],
                     expected_solid: bool|
     -> Result<(), String> {
        let point = if output == 1 {
            [point[0], -point[1], d.cradle_height_mm - point[2]]
        } else {
            point
        };
        let solid = contains(&meshes[output], point);
        if solid != expected_solid {
            return Err(format!("actual STL checkpoint failed: {name}"));
        }
        checks.push(json!({"check": name, "file": FILES[output], "point_mm": point, "expected_solid": expected_solid, "observed_solid": solid, "pass": true}));
        Ok(())
    };
    for x in [-d.tray_inner_mm[0] / 3.0, 0.0, d.tray_inner_mm[0] / 3.0] {
        for y in [-d.tray_inner_mm[1] / 3.0, 0.0, d.tray_inner_mm[1] / 3.0] {
            probe(
                0,
                "closed_floor_sample",
                [x, y, p.tray_floor_mm / 2.0],
                true,
            )?;
        }
    }
    probe(
        0,
        "open_basin_center",
        [0.0, 0.0, p.tray_floor_mm + p.tray_spill_depth_mm / 2.0],
        false,
    )?;
    for [x, y] in d.cradle_foot_centers_mm {
        probe(
            0,
            "socket_opening",
            [x, y, d.socket_bottom_z_mm + p.socket_depth_mm / 2.0],
            false,
        )?;
        probe(
            0,
            "socket_closed_support_land",
            [x, y, d.socket_bottom_z_mm - 0.2],
            true,
        )?;
        probe(
            1,
            "cradle_foot_seating_solid",
            [x, y, p.cradle_standoff_mm / 2.0],
            true,
        )?;
    }
    probe(
        1,
        "unobstructed_window_center",
        [
            0.0,
            0.0,
            p.cradle_standoff_mm + p.cradle_support_thickness_mm / 2.0,
        ],
        false,
    )?;
    probe(
        1,
        "coupon_edge_support_land",
        [
            0.0,
            p.coupon_width_mm / 2.0 - p.coupon_support_overlap_mm / 2.0,
            p.cradle_standoff_mm + p.cradle_support_thickness_mm / 2.0,
        ],
        true,
    )?;
    probe(
        1,
        "open_coupon_pocket",
        [
            0.0,
            0.0,
            p.cradle_standoff_mm + p.cradle_support_thickness_mm + p.coupon_thickness_mm / 2.0,
        ],
        false,
    )?;
    probe(
        1,
        "coupon_end_lift_relief",
        [
            d.cradle_outer_mm[0] / 2.0 - p.cradle_wall_mm / 2.0,
            0.0,
            d.cradle_height_mm - 0.4,
        ],
        false,
    )?;
    probe(
        2,
        "tube_saddle_clearance",
        [0.0, 0.0, d.guide_saddle_center_z_mm],
        false,
    )?;
    probe(
        2,
        "tie_tunnel_passage",
        [
            0.0,
            0.0,
            d.tie_passage_bottom_z_mm + p.tie_passage_height_mm / 2.0,
        ],
        false,
    )?;
    probe(
        2,
        "web_between_tie_and_saddle",
        [
            0.0,
            0.0,
            (d.tie_passage_bottom_z_mm + p.tie_passage_height_mm + d.guide_saddle_bottom_z_mm)
                / 2.0,
        ],
        true,
    )?;
    for y in [-p.guide_mount_pitch_mm / 2.0, p.guide_mount_pitch_mm / 2.0] {
        probe(
            2,
            "guide_fastener_through_hole",
            [0.0, y, p.guide_base_thickness_mm / 2.0],
            false,
        )?;
        probe(
            0,
            "external_pad_fastener_through_hole",
            [d.guide_mount_x_mm, y, p.mount_pad_height_mm / 2.0],
            false,
        )?;
        probe(
            0,
            "underside_nut_recess",
            [
                d.guide_mount_x_mm + p.mount_hole_diameter_mm / 2.0 + 0.3,
                y,
                p.nut_pocket_depth_mm / 2.0,
            ],
            false,
        )?;
    }
    Ok(checks)
}

fn write_or_verify(
    path: &Path,
    bytes: &[u8],
    verify: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if verify {
        if fs::read(path)? != bytes {
            return Err(format!("publication differs: {}", path.display()).into());
        }
    } else {
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let raw = fs::read_to_string(&args.config)?;
    let p: Parameters = toml::from_str(&raw)?;
    let d = p.validate()?;
    // Construct and check every output before creating a directory or modifying publication.
    let mut meshes = Vec::new();
    let mut previews = Vec::new();
    let mut outputs = Vec::new();
    let overall_half_x = d.guide_mount_x_mm + p.guide_length_mm / 2.0;
    let envelopes = [
        (
            [-overall_half_x, -p.tray_width_mm / 2.0, 0.0],
            [overall_half_x, p.tray_width_mm / 2.0, d.tray_rim_z_mm],
        ),
        (
            [
                -d.cradle_outer_mm[0] / 2.0,
                -d.cradle_outer_mm[1] / 2.0,
                0.0,
            ],
            [
                d.cradle_outer_mm[0] / 2.0,
                d.cradle_outer_mm[1] / 2.0,
                d.cradle_height_mm,
            ],
        ),
        (
            [-p.guide_length_mm / 2.0, -p.guide_base_width_mm / 2.0, 0.0],
            [
                p.guide_length_mm / 2.0,
                p.guide_base_width_mm / 2.0,
                d.guide_height_mm,
            ],
        ),
        (
            [-overall_half_x, -p.tray_width_mm / 2.0, 0.0],
            [
                overall_half_x,
                p.tray_width_mm / 2.0,
                (p.mount_pad_height_mm + d.guide_height_mm)
                    .max(d.cradle_assembly_z_mm + d.cradle_height_mm)
                    .max(d.tray_rim_z_mm),
            ],
        ),
        (
            [-p.coupon_length_mm / 2.0, -p.coupon_width_mm / 2.0, 0.0],
            [
                p.coupon_length_mm / 2.0,
                p.coupon_width_mm / 2.0,
                p.coupon_thickness_mm,
            ],
        ),
    ];
    for (index, name) in FILES.iter().enumerate() {
        let bytes = build(&p, &d, index).to_stl()?;
        let stats = mesh(&bytes)?;
        bounds_check(&stats, envelopes[index].0, envelopes[index].1)?;
        let svg = preview(&bytes)?;
        outputs.push(json!({"file": name, "role": if index < 3 { "individual_print_orientation" } else if index == 3 { "dry_assembly_with_reference_coupon" } else { "reference_only_NOT_FOR_MANUFACTURE" }, "quantity": if index == 2 { 2 } else { 1 }, "bytes": bytes.len(), "sha256": hash(&bytes), "mesh": stats, "envelope_check_pass": true, "preview_sha256": hash(svg.as_bytes())}));
        meshes.push(bytes);
        previews.push(svg);
    }
    let checks = interface_checks(&p, &d, &meshes)?;
    let report = json!({
        "schema_version": 1, "model": MODEL, "units": "mm", "parameters": &p, "derived_dimensions": &d,
        "bom": [
            {"item": "closed-floor tray", "file": FILES[0], "quantity": 1, "manufacture": "print candidate; process/material unqualified"},
            {"item": "removable coupon cradle", "file": FILES[1], "quantity": 1, "manufacture": "print candidate; no snapfit or clamping"},
            {"item": "bolt-down open tube guide", "file": FILES[2], "quantity": 2, "manufacture": "print candidate"},
            {"item": "M3×12 screw", "quantity": 4, "purchase": true, "modeled": false},
            {"item": "M3 full nut, nominal 5.5 mm across flats × 2.4 mm thick", "quantity": 4, "purchase": true, "modeled": false},
            {"item": "cable tie, maximum 2.5 mm wide × 1.2 mm thick", "quantity": 2, "purchase": true, "modeled": false},
            {"item": "provisional coupon envelope; editable dimensions and total stack thickness", "file": FILES[4], "quantity": 1, "manufacture": false, "released_chip": false},
            {"item": "tubing, provisional editable OD; length and connector selection pending", "purchase": true, "modeled": false}
        ],
        "assembly_transforms": [
            {"file": FILES[0], "translation_mm": [0.0, 0.0, 0.0], "rotation_degrees_xyz": [0, 0, 0]},
            {"file": FILES[1], "translation_mm": [0.0, 0.0, d.cradle_assembly_z_mm + d.cradle_height_mm], "rotation_degrees_xyz": [180, 0, 0]},
            {"file": FILES[2], "instance": "inlet", "translation_mm": [-d.guide_mount_x_mm, 0.0, p.mount_pad_height_mm], "rotation_degrees_xyz": [0, 0, 0]},
            {"file": FILES[2], "instance": "outlet", "translation_mm": [d.guide_mount_x_mm, 0.0, p.mount_pad_height_mm], "rotation_degrees_xyz": [0, 0, 0]},
            {"file": FILES[4], "reference_only": true, "translation_mm": [0.0, 0.0, d.coupon_assembly_z_mm], "rotation_degrees_xyz": [0, 0, 0]}
        ],
        "interface_checks": {
            "parameter_constraints_pass": true, "all_individual_print_min_z_mm": 0.0,
            "coupon_clearance_per_side_mm": p.coupon_clearance_per_side_mm,
            "coupon_edge_support_overlap_mm": p.coupon_support_overlap_mm,
            "gravity_socket_clearance_per_side_mm": p.socket_clearance_per_side_mm,
            "gravity_socket_engagement_mm": p.socket_depth_mm,
            "fastener_tip_above_tabletop_mm": d.fastener_tip_z_mm,
            "nut_bottom_above_tabletop_mm": d.nut_bottom_z_mm,
            "through_holes_in_catch_basin": 0,
            "stl_occupancy_checkpoints": checks
        },
        "assembly_steps": [
            "Insert four M3 nuts into the underside external-pad recesses; keep their faces recessed above the tabletop datum.",
            "Seat each guide flat on its external pad and install two M3×12 screws from above; do not drive screws through the basin.",
            "Lower the cradle feet into the four blind sockets; it lifts out vertically and is retained by gravity only.",
            "Place a measured coupon on the narrow support ledges; end reliefs allow lifting without a snapfit.",
            "Route selected tubing through the open saddles and add loose cable ties through the tunnels; verify no tube occlusion before any water run."
        ],
        "limitations": [
            "Dry fixture geometry only. The REF coupon is a blank envelope, not a channel, fluidic connector, seal or manufacturing part.",
            "Coupon thickness and tubing OD are provisional editable dimensions; measure actual purchased parts and total coupon stack before fabrication.",
            "Printed tray watertightness, material compatibility, fit, fastener preload, tube restraint and cleanability are unverified. No pressure or biological-use claim.",
            "Blind locating sockets can retain spilled water; remove the cradle and inspect, empty and dry the sockets after use.",
            "Gross empty-basin volume excludes displacement and freeboard; it is not a tested containment capacity.",
            "STL bounds and sampled occupancy checks do not establish full manifoldness, mechanical strength, printability or manufacturing qualification.",
            "Cradle PRINT export is rim-down. Its small inward support ledge and the guide tie-tunnel roof require slicer/physical support review; no support-free printing claim.",
            "Screws, nuts, ties, tubing, source syringe, collector and actual fluidic coupon are not modeled in the dry assembly."
        ]
    });
    let report_bytes = serde_json::to_vec_pretty(&report)?;
    let manifest_bytes = serde_json::to_vec_pretty(
        &json!({"schema_version": 1, "model": MODEL, "config_sha256": hash(raw.as_bytes()), "generator_sha256": hash(&fs::read(std::env::current_exe()?)?), "parameters": &p, "outputs": outputs, "interface_report": {"file": "water_transfer_dock.interfaces.json", "sha256": hash(&report_bytes)}, "claim": "Checked dry engineering geometry; physical fit, water transfer and manufacturing qualification pending"}),
    )?;
    let manifest = args.output_dir.join("water_transfer_dock.manifest.json");
    if !args.verify {
        fs::create_dir_all(&args.output_dir)?;
        match fs::remove_file(&manifest) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    for (index, name) in FILES.iter().enumerate() {
        let path = args.output_dir.join(name);
        write_or_verify(&path, &meshes[index], args.verify)?;
        write_or_verify(
            &path.with_extension("preview.svg"),
            previews[index].as_bytes(),
            args.verify,
        )?;
    }
    write_or_verify(
        &args.output_dir.join("water_transfer_dock.interfaces.json"),
        &report_bytes,
        args.verify,
    )?;
    // Commit marker is last; --verify compares all outputs and identities without writes.
    write_or_verify(&manifest, &manifest_bytes, args.verify)?;
    println!(
        "{} {MODEL}: {} meshes, dimensional envelopes and actual-STL interface checkpoints",
        if args.verify { "Verified" } else { "Generated" },
        FILES.len()
    );
    Ok(())
}
