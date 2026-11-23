//! Dry receiver/controller candidates sharing the enrolled dock executable.
use laminarforge_cad::{
    runtime_cad::{hash, mesh, Args},
    water_transfer_preview::preview,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use vcad::{centered_cube, centered_cylinder, Part};

#[path = "water_bench_carriage.rs"]
mod carriage;
#[path = "water_bench_frame.rs"]
mod frame;
#[path = "water_bench_layout.rs"]
mod layout;
#[path = "water_bench_motor.rs"]
mod motor;
#[path = "water_bench_outlet.rs"]
mod outlet;
#[path = "water_bench_screw_support.rs"]
mod screw_support;
#[path = "water_bench_source.rs"]
mod source;

const E: f64 = 0.02;
type Bounds = ([f64; 3], [f64; 3]);
struct Export {
    name: &'static str,
    part: Part,
    bounds: Bounds,
    role: &'static str,
}
struct Probe {
    index: usize,
    name: &'static str,
    point: [f64; 3],
    solid: bool,
}
fn block(name: &str, x: f64, y: f64, h: f64, z: f64) -> Part {
    centered_cube(name, x, y, h).translate(0., 0., z + h / 2.)
}
fn cylinder(name: &str, radius: f64, h: f64, z: f64) -> Part {
    centered_cylinder(name, radius, h, 64).translate(0., 0., z + h / 2.)
}
fn require(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}
fn finite<P: Serialize>(p: &P) -> Result<(), String> {
    require(
        super::finite_numeric_tree(&serde_json::to_value(p).map_err(|e| e.to_string())?),
        "nonfinite parameter",
    )
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Receiver {
    scale_length_mm: f64,
    scale_width_mm: f64,
    scale_pan_height_mm: f64,
    pan_length_mm: f64,
    pan_width_mm: f64,
    vial_diameter_mm: f64,
    vial_height_mm: f64,
    vial_clearance_per_side_mm: f64,
    nest_wall_mm: f64,
    nest_base_mm: f64,
    nest_capture_height_mm: f64,
    scale_to_frame_gap_mm: f64,
    frame_base_mm: f64,
    post_width_mm: f64,
    crossbar_depth_mm: f64,
    crossbar_thickness_mm: f64,
    tube_outer_diameter_mm: f64,
    tube_diametral_clearance_mm: f64,
    outlet_above_vial_mm: f64,
    tube_projection_below_crossbar_mm: f64,
}
impl Receiver {
    fn nest_radius(&self) -> f64 {
        self.vial_diameter_mm / 2. + self.vial_clearance_per_side_mm + self.nest_wall_mm
    }
    fn span(&self) -> [f64; 2] {
        [
            self.scale_length_mm + 2. * (self.scale_to_frame_gap_mm + self.post_width_mm),
            self.scale_width_mm + 2. * (self.scale_to_frame_gap_mm + self.post_width_mm),
        ]
    }
    fn vial_z(&self) -> f64 {
        self.scale_pan_height_mm + self.nest_base_mm
    }
    fn bar_z(&self) -> f64 {
        self.vial_z()
            + self.vial_height_mm
            + self.outlet_above_vial_mm
            + self.tube_projection_below_crossbar_mm
    }
    fn height(&self) -> f64 {
        self.bar_z() + self.crossbar_thickness_mm
    }
    fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require(
            (60. ..=200.).contains(&self.scale_length_mm)
                && (60. ..=200.).contains(&self.scale_width_mm)
                && (15. ..=80.).contains(&self.scale_pan_height_mm),
            "scale envelope outside candidate range",
        )?;
        require(
            self.pan_length_mm >= 40.
                && self.pan_length_mm <= self.scale_length_mm
                && self.pan_width_mm >= 40.
                && self.pan_width_mm <= self.scale_width_mm,
            "invalid weighing pan envelope",
        )?;
        require(
            (10. ..=40.).contains(&self.vial_diameter_mm)
                && (20. ..=90.).contains(&self.vial_height_mm),
            "invalid vial envelope",
        )?;
        require(
            (0.2..=0.8).contains(&self.vial_clearance_per_side_mm)
                && (1.6..=4.).contains(&self.nest_wall_mm)
                && (2. ..=5.).contains(&self.nest_base_mm),
            "invalid nest wall, floor or clearance",
        )?;
        require(
            (5. ..=15.).contains(&self.nest_capture_height_mm)
                && self.nest_capture_height_mm < self.vial_height_mm / 2.,
            "nest obstructs vial access",
        )?;
        require(
            2. * (self.nest_radius() + 2.) + 4. <= self.pan_length_mm.min(self.pan_width_mm),
            "nest footprint does not fit pan with edge clearance",
        )?;
        require(
            (10. ..=25.).contains(&self.scale_to_frame_gap_mm)
                && (5. ..=10.).contains(&self.frame_base_mm)
                && (10. ..=20.).contains(&self.post_width_mm),
            "invalid frame scale clearance/base/post",
        )?;
        require(
            (10. ..=20.).contains(&self.crossbar_depth_mm)
                && (6. ..=12.).contains(&self.crossbar_thickness_mm),
            "invalid crossbar",
        )?;
        require(
            (2. ..=6.).contains(&self.tube_outer_diameter_mm)
                && (0.4..=1.).contains(&self.tube_diametral_clearance_mm)
                && self.tube_outer_diameter_mm + self.tube_diametral_clearance_mm + 4.
                    <= self.crossbar_depth_mm,
            "insufficient tube guide wall",
        )?;
        require(
            (5. ..=20.).contains(&self.outlet_above_vial_mm)
                && (8. ..=25.).contains(&self.tube_projection_below_crossbar_mm),
            "invalid outlet/tube clearance",
        )?;
        require(
            self.height() <= 200.
                && self.span().iter().all(|v| *v <= 230.)
                && self.frame_base_mm < self.bar_z(),
            "frame exceeds candidate envelope",
        )
    }
    fn nest(&self) -> Part {
        cylinder("nest_base", self.nest_radius() + 2., self.nest_base_mm, 0.)
            + (cylinder(
                "nest_ring",
                self.nest_radius(),
                self.nest_capture_height_mm,
                self.nest_base_mm,
            ) - cylinder(
                "vial_pocket",
                self.vial_diameter_mm / 2. + self.vial_clearance_per_side_mm,
                self.nest_capture_height_mm + 2. * E,
                self.nest_base_mm - E,
            ))
    }
    fn frame(&self) -> Part {
        let [x, y] = self.span();
        let mut part = block("frame_base", x, y, self.frame_base_mm, 0.)
            - block(
                "scale_keepout",
                x - 2. * self.post_width_mm,
                y - 2. * self.post_width_mm,
                self.frame_base_mm + 2. * E,
                -E,
            );
        for sign in [-1., 1.] {
            part = part
                + block(
                    "upright",
                    self.post_width_mm,
                    self.crossbar_depth_mm,
                    self.height() - self.frame_base_mm,
                    self.frame_base_mm,
                )
                .translate(sign * (x - self.post_width_mm) / 2., 0., 0.);
        }
        part + (block(
            "crossbar",
            x,
            self.crossbar_depth_mm,
            self.crossbar_thickness_mm,
            self.bar_z(),
        ) - cylinder(
            "outlet_guide",
            (self.tube_outer_diameter_mm + self.tube_diametral_clearance_mm) / 2.,
            self.crossbar_thickness_mm + 2. * E,
            self.bar_z() - E,
        ))
    }
    fn scale(&self) -> Part {
        block(
            "REF_scale_body",
            self.scale_length_mm,
            self.scale_width_mm,
            self.scale_pan_height_mm - 3.,
            0.,
        ) + block(
            "REF_weighing_pan",
            self.pan_length_mm,
            self.pan_width_mm,
            3.,
            self.scale_pan_height_mm - 3.,
        )
    }
    fn vial(&self) -> Part {
        cylinder(
            "REF_vial",
            self.vial_diameter_mm / 2.,
            self.vial_height_mm,
            0.,
        )
    }
    fn exports(&self) -> Vec<Export> {
        let [x, y] = self.span();
        let r = self.nest_radius() + 2.;
        vec![
            Export {
                name: "receiver_vial_nest_PRINT.stl",
                part: self.nest(),
                bounds: (
                    [-r, -r, 0.],
                    [r, r, self.nest_base_mm + self.nest_capture_height_mm],
                ),
                role: "print_candidate_on_weighing_pan_only",
            },
            Export {
                name: "receiver_tube_frame_PRINT.stl",
                part: self.frame(),
                bounds: ([-x / 2., -y / 2., 0.], [x / 2., y / 2., self.height()]),
                role: "print_candidate_off_scale_frame_support_review_required",
            },
            Export {
                name: "receiver_assembly_DRY_with_REF_parts.stl",
                part: self.frame()
                    + self.nest().translate(0., 0., self.scale_pan_height_mm)
                    + self.scale()
                    + self.vial().translate(0., 0., self.vial_z()),
                bounds: ([-x / 2., -y / 2., 0.], [x / 2., y / 2., self.height()]),
                role: "assembly_reference_only",
            },
            Export {
                name: "receiver_REF_scale_NOT_FOR_MANUFACTURE.stl",
                part: self.scale(),
                bounds: (
                    [-self.scale_length_mm / 2., -self.scale_width_mm / 2., 0.],
                    [
                        self.scale_length_mm / 2.,
                        self.scale_width_mm / 2.,
                        self.scale_pan_height_mm,
                    ],
                ),
                role: "unselected_commercial_envelope_only",
            },
            Export {
                name: "receiver_REF_vial_NOT_FOR_MANUFACTURE.stl",
                part: self.vial(),
                bounds: (
                    [-self.vial_diameter_mm / 2., -self.vial_diameter_mm / 2., 0.],
                    [
                        self.vial_diameter_mm / 2.,
                        self.vial_diameter_mm / 2.,
                        self.vial_height_mm,
                    ],
                ),
                role: "unselected_commercial_envelope_only",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let x = (self.span()[0] - self.post_width_mm) / 2.;
        vec![
            Probe {
                index: 0,
                name: "nest_closed_floor",
                point: [0., 0., self.nest_base_mm / 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "vial_pocket_open",
                point: [0., 0., self.nest_base_mm + self.nest_capture_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "nest_wall",
                point: [
                    self.nest_radius() - self.nest_wall_mm / 2.,
                    0.,
                    self.nest_base_mm + self.nest_capture_height_mm / 2.,
                ],
                solid: true,
            },
            Probe {
                index: 1,
                name: "scale_base_keepout",
                point: [0., 0., self.frame_base_mm / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "frame_base_support",
                point: [x, 0., self.frame_base_mm / 2.],
                solid: true,
            },
            Probe {
                index: 1,
                name: "upright_support",
                point: [x, 0., (self.frame_base_mm + self.bar_z()) / 2.],
                solid: true,
            },
            Probe {
                index: 1,
                name: "tube_passage",
                point: [0., 0., self.bar_z() + self.crossbar_thickness_mm / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "crossbar_land",
                point: [10., 0., self.bar_z() + self.crossbar_thickness_mm / 2.],
                solid: true,
            },
        ]
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Controller {
    board_length_mm: f64,
    board_width_mm: f64,
    board_thickness_mm: f64,
    mount_pitch_x_mm: f64,
    mount_pitch_y_mm: f64,
    mount_hole_diameter_mm: f64,
    board_edge_clearance_mm: f64,
    wall_mm: f64,
    base_mm: f64,
    foot_height_mm: f64,
    foot_width_mm: f64,
    standoff_height_mm: f64,
    standoff_diameter_mm: f64,
    rim_height_mm: f64,
    cable_window_width_mm: f64,
    cable_window_bottom_above_base_mm: f64,
}
impl Controller {
    fn outer(&self) -> [f64; 2] {
        [
            self.board_length_mm + 2. * (self.board_edge_clearance_mm + self.wall_mm),
            self.board_width_mm + 2. * (self.board_edge_clearance_mm + self.wall_mm),
        ]
    }
    fn floor_z(&self) -> f64 {
        self.foot_height_mm + self.base_mm
    }
    fn board_z(&self) -> f64 {
        self.floor_z() + self.standoff_height_mm
    }
    fn top(&self) -> f64 {
        self.floor_z() + self.rim_height_mm
    }
    fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require(
            (40. ..=160.).contains(&self.board_length_mm)
                && (30. ..=100.).contains(&self.board_width_mm)
                && (1. ..=3.).contains(&self.board_thickness_mm),
            "invalid generic board envelope",
        )?;
        require(
            (3.2..=3.8).contains(&self.mount_hole_diameter_mm)
                && (8. ..=12.).contains(&self.standoff_diameter_mm),
            "invalid mounting bore/boss",
        )?;
        require(
            self.mount_pitch_x_mm >= 20.
                && self.mount_pitch_y_mm >= 15.
                && self.mount_pitch_x_mm + self.standoff_diameter_mm + 4. <= self.board_length_mm
                && self.mount_pitch_y_mm + self.standoff_diameter_mm + 4. <= self.board_width_mm,
            "mount pattern/bosses do not fit board",
        )?;
        require(
            (1. ..=4.).contains(&self.board_edge_clearance_mm)
                && (2. ..=4.).contains(&self.wall_mm)
                && (2.4..=5.).contains(&self.base_mm),
            "invalid floor or wall",
        )?;
        require(
            (4. ..=8.).contains(&self.foot_height_mm)
                && (8. ..=14.).contains(&self.foot_width_mm)
                && (5. ..=10.).contains(&self.standoff_height_mm),
            "invalid feet or standoffs",
        )?;
        require(
            (6. ..=14.).contains(&self.rim_height_mm)
                && self.rim_height_mm >= self.standoff_height_mm + self.board_thickness_mm,
            "rim below board edge",
        )?;
        require(
            (10. ..=24.).contains(&self.cable_window_width_mm)
                && self.cable_window_width_mm + self.standoff_diameter_mm + 4.
                    < self.mount_pitch_y_mm,
            "window interferes with mounting boss region",
        )?;
        require(
            self.cable_window_bottom_above_base_mm >= 1.
                && self.cable_window_bottom_above_base_mm < self.standoff_height_mm,
            "invalid cable opening floor",
        )?;
        // Keep nuts and through holes clear of the four tabletop feet.
        require(
            self.mount_pitch_x_mm + self.mount_hole_diameter_mm
                < self.outer()[0] - 2. * self.foot_width_mm
                || self.mount_pitch_y_mm + self.mount_hole_diameter_mm
                    < self.outer()[1] - 2. * self.foot_width_mm,
            "feet intersect fastener access",
        )
    }
    fn holes(&self, mut part: Part, height: f64, z: f64) -> Part {
        for x in [-self.mount_pitch_x_mm / 2., self.mount_pitch_x_mm / 2.] {
            for y in [-self.mount_pitch_y_mm / 2., self.mount_pitch_y_mm / 2.] {
                part = part
                    - cylinder(
                        "mount_clearance",
                        self.mount_hole_diameter_mm / 2.,
                        height,
                        z,
                    )
                    .translate(x, y, 0.);
            }
        }
        part
    }
    fn tray(&self) -> Part {
        let [x, y] = self.outer();
        let mut p = block("base", x, y, self.base_mm, self.foot_height_mm)
            + (block("rim", x, y, self.rim_height_mm, self.floor_z())
                - block(
                    "interior",
                    x - 2. * self.wall_mm,
                    y - 2. * self.wall_mm,
                    self.rim_height_mm + 2. * E,
                    self.floor_z() - E,
                ));
        for sx in [-1., 1.] {
            for sy in [-1., 1.] {
                p = p + block(
                    "tabletop_foot",
                    self.foot_width_mm,
                    self.foot_width_mm,
                    self.foot_height_mm,
                    0.,
                )
                .translate(
                    sx * (x - self.foot_width_mm) / 2.,
                    sy * (y - self.foot_width_mm) / 2.,
                    0.,
                );
                p = p + cylinder(
                    "board_standoff",
                    self.standoff_diameter_mm / 2.,
                    self.standoff_height_mm,
                    self.floor_z(),
                )
                .translate(
                    sx * self.mount_pitch_x_mm / 2.,
                    sy * self.mount_pitch_y_mm / 2.,
                    0.,
                );
            }
        }
        for sign in [-1., 1.] {
            p = p - block(
                "connector_window",
                self.wall_mm + 2. * E,
                self.cable_window_width_mm,
                self.rim_height_mm - self.cable_window_bottom_above_base_mm + E,
                self.floor_z() + self.cable_window_bottom_above_base_mm,
            )
            .translate(sign * (x - self.wall_mm) / 2., 0., 0.);
        }
        self.holes(p, self.top() + 2. * E, -E)
    }
    fn board(&self) -> Part {
        self.holes(
            block(
                "REF_board",
                self.board_length_mm,
                self.board_width_mm,
                self.board_thickness_mm,
                0.,
            ),
            self.board_thickness_mm + 2. * E,
            -E,
        )
    }
    fn exports(&self) -> Vec<Export> {
        let [x, y] = self.outer();
        vec![
            Export {
                name: "controller_carrier_PRINT.stl",
                part: self.tray(),
                bounds: ([-x / 2., -y / 2., 0.], [x / 2., y / 2., self.top()]),
                role: "dry_open_carrier_print_candidate",
            },
            Export {
                name: "controller_REF_board_NOT_FOR_MANUFACTURE.stl",
                part: self.board(),
                bounds: (
                    [-self.board_length_mm / 2., -self.board_width_mm / 2., 0.],
                    [
                        self.board_length_mm / 2.,
                        self.board_width_mm / 2.,
                        self.board_thickness_mm,
                    ],
                ),
                role: "unselected_board_envelope_only",
            },
            Export {
                name: "controller_assembly_DRY_with_REF_board.stl",
                part: self.tray() + self.board().translate(0., 0., self.board_z()),
                bounds: ([-x / 2., -y / 2., 0.], [x / 2., y / 2., self.top()]),
                role: "assembly_reference_only",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let [x, y] = self.outer();
        let bx = self.mount_pitch_x_mm / 2.;
        let by = self.mount_pitch_y_mm / 2.;
        vec![
            Probe {
                index: 0,
                name: "closed_base",
                point: [0., 0., self.foot_height_mm + self.base_mm / 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "open_board_space",
                point: [0., 0., self.floor_z() + self.standoff_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "mounting_through_hole",
                point: [bx, by, self.floor_z() + self.standoff_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "standoff_land",
                point: [
                    bx + (self.standoff_diameter_mm + self.mount_hole_diameter_mm) / 4.,
                    by,
                    self.floor_z() + self.standoff_height_mm / 2.,
                ],
                solid: true,
            },
            Probe {
                index: 0,
                name: "tabletop_foot",
                point: [
                    (x - self.foot_width_mm) / 2.,
                    (y - self.foot_width_mm) / 2.,
                    self.foot_height_mm / 2.,
                ],
                solid: true,
            },
            Probe {
                index: 0,
                name: "connector_window",
                point: [
                    (x - self.wall_mm) / 2.,
                    0.,
                    self.floor_z()
                        + (self.rim_height_mm + self.cable_window_bottom_above_base_mm) / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "sidewall",
                point: [
                    0.,
                    (y - self.wall_mm) / 2.,
                    self.floor_z() + self.rim_height_mm / 2.,
                ],
                solid: true,
            },
        ]
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Coupon {
    length_mm: f64,
    width_mm: f64,
    body_thickness_mm: f64,
    bottom_thickness_mm: f64,
    channel_width_mm: f64,
    channel_depth_mm: f64,
    chamber_length_mm: f64,
    chamber_width_mm: f64,
    port_pitch_mm: f64,
    port_diameter_mm: f64,
    minimum_bond_land_mm: f64,
}
impl Coupon {
    fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require((self.length_mm-75.).abs()<1e-6 && (self.width_mm-25.).abs()<1e-6 && (self.body_thickness_mm+self.bottom_thickness_mm-4.).abs()<1e-6,"current dock interface requires 75x25x4 mm; revise mating contract explicitly for other envelopes")?;
        require(
            (0.7..=1.5).contains(&self.bottom_thickness_mm)
                && (2.5..=3.3).contains(&self.body_thickness_mm),
            "unsupported provisional body/bottom stack",
        )?;
        require(
            (0.3..=1.).contains(&self.channel_width_mm)
                && (0.1..=0.5).contains(&self.channel_depth_mm)
                && self.body_thickness_mm - self.channel_depth_mm >= 1.5,
            "invalid channel or insufficient roof",
        )?;
        require(
            (8. ..=20.).contains(&self.chamber_length_mm)
                && (3. ..=10.).contains(&self.chamber_width_mm)
                && self.channel_width_mm < self.chamber_width_mm,
            "invalid chamber/channel transition",
        )?;
        require(
            (1. ..=4.).contains(&self.port_diameter_mm)
                && self.port_diameter_mm >= self.channel_width_mm
                && (2. ..=4.).contains(&self.minimum_bond_land_mm),
            "invalid unselected port/bond-land envelope",
        )?;
        require(
            self.port_pitch_mm / 2.
                > self.chamber_length_mm / 2.
                    + self.port_diameter_mm / 2.
                    + self.minimum_bond_land_mm,
            "port overlaps chamber transition",
        )?;
        require(
            self.length_mm / 2. - self.port_pitch_mm / 2. - self.port_diameter_mm / 2.
                >= self.minimum_bond_land_mm
                && self.width_mm / 2.
                    - (self.chamber_width_mm / 2.).max(self.port_diameter_mm / 2.)
                    >= self.minimum_bond_land_mm,
            "insufficient nominal edge/bond land",
        )
    }
    fn void(&self, epsilon: f64) -> Part {
        let mut p = block(
            "channel",
            self.port_pitch_mm,
            self.channel_width_mm,
            self.channel_depth_mm + epsilon,
            -epsilon,
        ) + block(
            "chamber",
            self.chamber_length_mm,
            self.chamber_width_mm,
            self.channel_depth_mm + epsilon,
            -epsilon,
        );
        for sign in [-1., 1.] {
            p = p + cylinder(
                "port",
                self.port_diameter_mm / 2.,
                self.body_thickness_mm + 2. * epsilon,
                -epsilon,
            )
            .translate(sign * self.port_pitch_mm / 2., 0., 0.);
        }
        p
    }
    fn body(&self) -> Part {
        block(
            "coupon_body",
            self.length_mm,
            self.width_mm,
            self.body_thickness_mm,
            0.,
        ) - self.void(E)
    }
    fn bottom(&self) -> Part {
        block(
            "coupon_bottom",
            self.length_mm,
            self.width_mm,
            self.bottom_thickness_mm,
            0.,
        )
    }
    fn exports(&self) -> Vec<Export> {
        let low = [-self.length_mm / 2., -self.width_mm / 2., 0.];
        let high = [
            self.length_mm / 2.,
            self.width_mm / 2.,
            self.body_thickness_mm,
        ];
        let vx = (self.port_pitch_mm + self.port_diameter_mm) / 2.;
        let vy = self.chamber_width_mm.max(self.port_diameter_mm) / 2.;
        vec![
            Export {
                name: "coupon_body_REVIEW_NOT_FOR_MANUFACTURE.stl",
                part: self.body(),
                bounds: (low, high),
                role: "water_only_review_body_not_fabrication_release",
            },
            Export {
                name: "coupon_bottom_REVIEW_NOT_FOR_MANUFACTURE.stl",
                part: self.bottom(),
                bounds: (low, [high[0], high[1], self.bottom_thickness_mm]),
                role: "unselected_bottom_and_bond_stack_review_only",
            },
            Export {
                name: "coupon_assembly_REVIEW_NOT_FOR_MANUFACTURE.stl",
                part: self.bottom() + self.body().translate(0., 0., self.bottom_thickness_mm),
                bounds: (low, [high[0], high[1], 4.]),
                role: "nominal_zero_bondline_assembly_review_only",
            },
            Export {
                name: "coupon_channel_face_VIEW_ONLY.stl",
                part: self
                    .body()
                    .rotate(180., 0., 0.)
                    .translate(0., 0., self.body_thickness_mm),
                bounds: (low, high),
                role: "alternate_underside_view_same_body_not_additional_part",
            },
            Export {
                name: "coupon_fluid_space_VIEW_ONLY.stl",
                part: self.void(0.),
                bounds: ([-vx, -vy, 0.], [vx, vy, self.body_thickness_mm]),
                role: "negative_space_visualization_not_a_physical_part",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let mut probes = vec![
            Probe {
                index: 0,
                name: "chamber_open_at_bond_face",
                point: [0., 0., self.channel_depth_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "chamber_roof",
                point: [0., 0., self.channel_depth_mm + 0.3],
                solid: true,
            },
            Probe {
                index: 0,
                name: "bond_land",
                point: [
                    0.,
                    self.chamber_width_mm / 2. + self.minimum_bond_land_mm / 2.,
                    self.channel_depth_mm / 2.,
                ],
                solid: true,
            },
            Probe {
                index: 1,
                name: "bottom_closed",
                point: [0., 0., self.bottom_thickness_mm / 2.],
                solid: true,
            },
            Probe {
                index: 2,
                name: "assembled_chamber_void",
                point: [
                    0.,
                    0.,
                    self.bottom_thickness_mm + self.channel_depth_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 4,
                name: "fluid_chamber",
                point: [0., 0., self.channel_depth_mm / 2.],
                solid: true,
            },
        ];
        for sign in [-1., 1.] {
            let x = sign * (self.port_pitch_mm + self.chamber_length_mm) / 4.;
            probes.push(Probe {
                index: 0,
                name: "channel_connects_chamber_and_port",
                point: [x, 0., self.channel_depth_mm / 2.],
                solid: false,
            });
            probes.push(Probe {
                index: 0,
                name: "through_port",
                point: [
                    sign * self.port_pitch_mm / 2.,
                    0.,
                    self.body_thickness_mm / 2.,
                ],
                solid: false,
            });
            probes.push(Probe {
                index: 4,
                name: "fluid_channel",
                point: [x, 0., self.channel_depth_mm / 2.],
                solid: true,
            });
            probes.push(Probe {
                index: 4,
                name: "fluid_port",
                point: [
                    sign * self.port_pitch_mm / 2.,
                    0.,
                    self.body_thickness_mm / 2.,
                ],
                solid: true,
            });
        }
        probes
    }
}

fn publish<P: Serialize>(
    args: &Args,
    model: &str,
    raw: &str,
    p: &P,
    exports: Vec<Export>,
    probes: Vec<Probe>,
    design: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut generated = Vec::new();
    let mut records = Vec::new();
    let mut checks = Vec::new();
    for item in exports {
        let bytes = item.part.to_stl()?;
        let stats = mesh(&bytes)?;
        super::bounds_check(&stats, item.bounds.0, item.bounds.1)?;
        let svg = preview(&bytes)?.replace("Water-transfer dock", model);
        records.push(json!({"file":item.name,"role":item.role,"quantity":1,"sha256":hash(&bytes),"bytes":bytes.len(),"mesh":stats,"preview_sha256":hash(svg.as_bytes())}));
        generated.push((item.name, bytes, svg));
    }
    for probe in probes {
        let observed = super::contains(&generated[probe.index].1, probe.point);
        require(
            observed == probe.solid,
            &format!("actual STL checkpoint failed: {}", probe.name),
        )?;
        checks.push(json!({"check":probe.name,"file":generated[probe.index].0,"point_mm":probe.point,"expected_solid":probe.solid,"observed_solid":observed,"pass":true}));
    }
    let report = json!({"model":model,"units":"mm","parameters":p,"design":design,"actual_stl_checkpoints":checks,"limits":["Dry engineering candidates. All bought-component dimensions are provisional and unmeasured.","Reference meshes are envelopes only and must not be manufactured as purchased components.","No print support, stability, fastener, electrical, fluid-pressure, scale accuracy or biological qualification.","Sampled occupancy and envelope checks do not establish full collision freedom or fabrication readiness."]});
    let report_bytes = serde_json::to_vec_pretty(&report)?;
    let report_name = format!("{model}.interfaces.json");
    let evidence = json!({"schema_version":1,"model":model,"config_sha256":hash(raw.as_bytes()),"generator_sha256":hash(&fs::read(std::env::current_exe()?)?),"parameters":p,"outputs":records,"interface_report":{"file":report_name,"sha256":hash(&report_bytes)},"claim":"Dry geometry candidate; purchased interfaces and physical behavior unverified"});
    let manifest = args.output_dir.join(format!("{model}.manifest.json"));
    if !args.verify {
        fs::create_dir_all(&args.output_dir)?;
        match fs::remove_file(&manifest) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    for (name, bytes, svg) in generated {
        let path = args.output_dir.join(name);
        super::write_or_verify(&path, &bytes, args.verify)?;
        super::write_or_verify(
            &path.with_extension("preview.svg"),
            svg.as_bytes(),
            args.verify,
        )?;
    }
    super::write_or_verify(
        &args.output_dir.join(report_name),
        &report_bytes,
        args.verify,
    )?;
    super::write_or_verify(
        &manifest,
        &serde_json::to_vec_pretty(&evidence)?,
        args.verify,
    )?;
    println!(
        "{} {model}: checked meshes, bounds and actual-STL interface checkpoints",
        if args.verify { "Verified" } else { "Generated" }
    );
    Ok(())
}

pub fn run(args: &Args, module: &str) -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(&args.config)?;
    match module {
        "outlet" => outlet::run(args, &raw),
        "source" => source::run(args, &raw),
        "bench" => layout::run(args, &raw),
        "carriage" => carriage::run(args, &raw),
        "frame" => frame::run(args, &raw),
        "motor" => motor::run(args, &raw),
        "screw_support" => screw_support::run(args, &raw),
        "coupon" => {
            let p: Coupon = toml::from_str(&raw)?;
            p.validate()?;
            let design = json!({"use":"water/dye engineering review only","fabrication_released":false,"outer_envelope_mm":[p.length_mm,p.width_mm,4.],"port_centers_body_mm":[[-p.port_pitch_mm/2.,0.],[p.port_pitch_mm/2.,0.]],"chamber_size_mm":[p.chamber_length_mm,p.chamber_width_mm,p.channel_depth_mm],"channel_size_mm":[p.channel_width_mm,p.channel_depth_mm],"assembly_transforms":[{"file":"coupon_bottom_REVIEW_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,0.]},{"file":"coupon_body_REVIEW_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,p.bottom_thickness_mm]},{"file":"coupon_fluid_space_VIEW_ONLY.stl","translation_mm":[0.,0.,p.bottom_thickness_mm],"negative_space_only":true}],"review_decisions_open":["actual bought fitting and port interface","body and bottom materials","bond process, bond-line thickness and intrusion allowance","dimensional tolerances and inspection","fabricator-accepted analytic solid and controlled drawings","leak/pressure criteria and physical flow observations"],"assembly_rules":["No STL here is a manufacturing release. Review with the selected fabricator first.","Zero-thickness bond interface is a geometric assumption, not a specified adhesive or sealing process.","The underside-view STL is the same body turned over, not an extra component.","The fluid-space STL shows intended empty volume; never manufacture it as a part.","No fittings, seals, tubing or biological features are modeled." ]});
            publish(
                args,
                "water_bench_coupon",
                &raw,
                &p,
                p.exports(),
                p.probes(),
                design,
            )
        }
        "receiver" => {
            let p: Receiver = toml::from_str(&raw)?;
            p.validate()?;
            let design = json!({"frame_outer_mm":p.span(),"frame_height_mm":p.height(),"scale_to_frame_gap_mm":p.scale_to_frame_gap_mm,"vial_bottom_z_mm":p.vial_z(),"vial_top_z_mm":p.vial_z()+p.vial_height_mm,"outlet_tip_z_mm":p.vial_z()+p.vial_height_mm+p.outlet_above_vial_mm,"crossbar_bottom_z_mm":p.bar_z(),"assembly_transforms":[{"file":"receiver_vial_nest_PRINT.stl","translation_mm":[0.,0.,p.scale_pan_height_mm]},{"file":"receiver_tube_frame_PRINT.stl","translation_mm":[0.,0.,0.]},{"file":"receiver_REF_scale_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,0.]},{"file":"receiver_REF_vial_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,p.vial_z()]}],"assembly_rules":["Nest and vial sit only on the weighing pan; tare their combined mass.","Frame sits only on the bench, with clearance around the entire scale body; no attachment between frame and scale.","Position outlet above vial without touching vial, nest or weighing pan. Verify clearances under actual tubing loads.","Tube passage is a guide, not validated strain relief. Use an independent off-scale anchor and verify slip, kinks and stability.","Integral frame bridge requires printer/orientation/support review; no support-free claim."],"bom":[{"item":"vial nest","quantity":1},{"item":"off-scale tube frame","quantity":1},{"item":"commercial scale and vial","selected":false},{"item":"tubing/anchor hardware","selected":false}]});
            publish(
                args,
                "water_bench_receiver",
                &raw,
                &p,
                p.exports(),
                p.probes(),
                design,
            )
        }
        "controller" => {
            let p: Controller = toml::from_str(&raw)?;
            p.validate()?;
            let design = json!({"carrier_outer_mm":p.outer(),"carrier_height_mm":p.top(),"board_bottom_z_mm":p.board_z(),"board_top_z_mm":p.board_z()+p.board_thickness_mm,"mount_pitch_mm":[p.mount_pitch_x_mm,p.mount_pitch_y_mm],"assembly_transforms":[{"file":"controller_carrier_PRINT.stl","translation_mm":[0.,0.,0.]},{"file":"controller_REF_board_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,p.board_z()]}],"assembly_rules":["Measure actual board mounting holes and connector envelopes before revising TOML.","Use nonconductive supports and verify board underside component/copper clearances.","Select screw/nut/washer stack after measurement; keep fastener ends above tabletop and prevent contact with traces.","Open carrier has no ingress, electrical, fire or touch-safety rating; keep it outside wet area.","Cable windows are provisional and do not establish connector fit or strain relief."],"bom":[{"item":"open dry board carrier","quantity":1},{"item":"controller board/driver/power supply","selected":false},{"item":"four board fasteners and insulating hardware","selected":false}]});
            publish(
                args,
                "water_bench_controller",
                &raw,
                &p,
                p.exports(),
                p.probes(),
                design,
            )
        }
        _ => Err("unsupported module".into()),
    }
}
