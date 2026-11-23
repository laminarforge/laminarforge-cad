//! Unpowered guide-frame review. No lead-screw thrust support or syringe load path.
use super::carriage::Carriage;
use super::*;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Frame {
    frame: Parameters,
    carriage: Carriage,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    clear_span_mm: f64,
    support_length_mm: f64,
    support_width_mm: f64,
    cap_height_mm: f64,
    base_thickness_mm: f64,
    base_end_margin_mm: f64,
    base_side_margin_mm: f64,
    carriage_floor_gap_mm: f64,
    rod_diametral_clearance_mm: f64,
    cap_bolt_offset_from_rod_mm: f64,
    cap_hole_diameter_mm: f64,
    lead_clearance_diameter_mm: f64,
    bench_hole_diameter_mm: f64,
    bench_edge_offset_mm: f64,
    carriage_start_x_mm: f64,
    travel_mm: f64,
    minimum_end_gap_mm: f64,
    minimum_wall_mm: f64,
}
impl Frame {
    fn interface(&self) -> [f64; 4] {
        self.carriage.guide_interface()
    }
    fn carriage_z(&self) -> f64 {
        self.frame.base_thickness_mm + self.frame.carriage_floor_gap_mm
    }
    fn axis_z(&self) -> f64 {
        self.carriage_z() + self.interface()[2]
    }
    fn radius(&self) -> f64 {
        (self.interface()[1] + self.frame.rod_diametral_clearance_mm) / 2.
    }
    pub(super) fn base_bounds(&self) -> Bounds {
        let p = &self.frame;
        (
            [
                -p.support_length_mm - p.base_end_margin_mm,
                -p.support_width_mm / 2. - p.base_side_margin_mm,
                0.,
            ],
            [
                p.clear_span_mm + p.support_length_mm + p.base_end_margin_mm,
                p.support_width_mm / 2. + p.base_side_margin_mm,
                p.base_thickness_mm,
            ],
        )
    }
    fn bolt_ys(&self) -> [f64; 4] {
        let r = self.interface()[0] / 2.;
        let d = self.frame.cap_bolt_offset_from_rod_mm;
        [-r - d, -r + d, r - d, r + d]
    }
    pub(super) fn validate(&self) -> Result<(), String> {
        finite(&self.frame)?;
        self.carriage.validate()?;
        let p = &self.frame;
        let [pitch, rod, _, width] = self.interface();
        let r = self.radius();
        require(
            (140. ..=260.).contains(&p.clear_span_mm)
                && (16. ..=30.).contains(&p.support_length_mm)
                && (80. ..=120.).contains(&p.support_width_mm)
                && (8. ..=16.).contains(&p.cap_height_mm)
                && (4. ..=10.).contains(&p.base_thickness_mm)
                && (3. ..=10.).contains(&p.carriage_floor_gap_mm)
                && (8. ..=20.).contains(&p.base_end_margin_mm)
                && (8. ..=20.).contains(&p.base_side_margin_mm)
                && (2.5..=5.).contains(&p.minimum_wall_mm),
            "invalid guide frame envelope",
        )?;
        require(
            (0.15..=0.6).contains(&p.rod_diametral_clearance_mm)
                && (3.2..=4.).contains(&p.cap_hole_diameter_mm)
                && (10. ..=16.).contains(&p.lead_clearance_diameter_mm)
                && (4.3..=5.).contains(&p.bench_hole_diameter_mm),
            "invalid frame bores",
        )?;
        require(
            p.support_width_mm >= width
                && p.cap_height_mm - r >= p.minimum_wall_mm
                && self.axis_z() - p.base_thickness_mm - r >= p.minimum_wall_mm
                && pitch / 2. - r - p.lead_clearance_diameter_mm / 2. >= p.minimum_wall_mm
                && p.lead_clearance_diameter_mm > rod,
            "frame bores conflict or lack support lands",
        )?;
        let h = p.cap_hole_diameter_mm / 2.;
        let d = p.cap_bolt_offset_from_rod_mm;
        require(
            d - r - h >= p.minimum_wall_mm
                && p.support_width_mm / 2. - pitch / 2. - d - h >= p.minimum_wall_mm
                && pitch - 2. * d - 2. * h >= p.minimum_wall_mm
                && p.support_length_mm / 2. - h >= p.minimum_wall_mm,
            "split cap holes conflict with rod saddles or edges",
        )?;
        require(
            p.carriage_start_x_mm - self.carriage.cap_thickness() >= p.minimum_end_gap_mm
                && p.carriage_start_x_mm + p.travel_mm + self.carriage.body_length()
                    <= p.clear_span_mm - p.minimum_end_gap_mm
                && (40. ..=180.).contains(&p.travel_mm)
                && p.minimum_end_gap_mm >= 5.,
            "configured travel collides with end supports",
        )?;
        let b = self.base_bounds();
        require(
            (6. ..=10.).contains(&p.bench_edge_offset_mm)
                && p.bench_edge_offset_mm - p.bench_hole_diameter_mm / 2. >= p.minimum_wall_mm
                && p.base_side_margin_mm - p.bench_edge_offset_mm - p.bench_hole_diameter_mm / 2.
                    >= 1.,
            "bench holes conflict with support footprint or base edge",
        )?;
        require(b.1[0] > b.0[0], "invalid base bounds")
    }
    fn x_bore(&self, name: &str, r: f64, x: f64, len: f64, y: f64) -> Part {
        centered_cylinder(name, r, len, 64)
            .rotate(0., 90., 0.)
            .translate(x + len / 2., y, self.axis_z())
    }
    fn bolt_holes(&self, mut part: Part, x: f64, z: f64, h: f64) -> Part {
        for y in self.bolt_ys() {
            part = part
                - cylinder(
                    "cap_vertical_through_bolt",
                    self.frame.cap_hole_diameter_mm / 2.,
                    h + 2. * E,
                    z - E,
                )
                .translate(x, y, 0.);
        }
        part
    }
    fn support(&self, cap: bool) -> Part {
        let p = &self.frame;
        let (z, h) = if cap {
            (self.axis_z(), p.cap_height_mm)
        } else {
            (p.base_thickness_mm, self.axis_z() - p.base_thickness_mm)
        };
        let mut part = block(
            if cap {
                "split_cap"
            } else {
                "lower_rod_support"
            },
            p.support_length_mm,
            p.support_width_mm,
            h,
            z,
        );
        for y in [-self.interface()[0] / 2., self.interface()[0] / 2.] {
            part = part
                - self.x_bore(
                    "split_rod_saddle",
                    self.radius(),
                    -p.support_length_mm / 2. - E,
                    p.support_length_mm + 2. * E,
                    y,
                );
        }
        part = part
            - self.x_bore(
                "unloaded_lead_axis_clearance",
                p.lead_clearance_diameter_mm / 2.,
                -p.support_length_mm / 2. - E,
                p.support_length_mm + 2. * E,
                0.,
            );
        self.bolt_holes(part, 0., z, h)
    }
    fn base(&self) -> Part {
        let p = &self.frame;
        let (a, b) = self.base_bounds();
        let mut part = block(
            "flat_base_review",
            b[0] - a[0],
            b[1] - a[1],
            p.base_thickness_mm,
            0.,
        )
        .translate((a[0] + b[0]) / 2., 0., 0.);
        for x in [
            -p.support_length_mm / 2.,
            p.clear_span_mm + p.support_length_mm / 2.,
        ] {
            part = self.bolt_holes(part, x, 0., p.base_thickness_mm);
        }
        for x in [a[0] + p.bench_edge_offset_mm, b[0] - p.bench_edge_offset_mm] {
            for y in [a[1] + p.bench_edge_offset_mm, b[1] - p.bench_edge_offset_mm] {
                part = part
                    - cylinder(
                        "bench_anchor_through_hole",
                        p.bench_hole_diameter_mm / 2.,
                        p.base_thickness_mm + 2. * E,
                        -E,
                    )
                    .translate(x, y, 0.);
            }
        }
        part
    }
    fn rods(&self) -> Part {
        let p = &self.frame;
        let one = |y| {
            self.x_bore(
                "REF_straight_rod",
                self.interface()[1] / 2.,
                -p.support_length_mm,
                p.clear_span_mm + 2. * p.support_length_mm,
                y,
            )
        };
        one(-self.interface()[0] / 2.) + one(self.interface()[0] / 2.)
    }
    fn assembly(&self, x: f64) -> Part {
        let p = &self.frame;
        let first_end = self.support(false) + self.support(true);
        let second_end = self.support(false) + self.support(true);
        self.base()
            + first_end.translate(-p.support_length_mm / 2., 0., 0.)
            + second_end.translate(p.clear_span_mm + p.support_length_mm / 2., 0., 0.)
            + self.rods()
            + self
                .carriage
                .dry_assembly()
                .translate(x, 0., self.carriage_z())
    }
    pub(super) fn rear_support_x(&self) -> f64 {
        -self.frame.support_length_mm
    }
    pub(super) fn axis_height(&self) -> f64 {
        self.axis_z()
    }
    pub(super) fn screw_diameter(&self) -> f64 {
        self.carriage.screw_diameter()
    }
    pub(super) fn required_nut_end_x(&self) -> f64 {
        self.frame.carriage_start_x_mm + self.frame.travel_mm + self.carriage.body_length()
    }
    pub(super) fn start_assembly(&self) -> Part {
        self.assembly(self.frame.carriage_start_x_mm)
    }
    pub(super) fn assembly_bounds(&self) -> Bounds {
        let (a, b) = self.base_bounds();
        (
            a,
            [
                b[0],
                b[1],
                (self.axis_z() + self.frame.cap_height_mm)
                    .max(self.carriage_z() + 2. * self.interface()[2]),
            ],
        )
    }
    fn exports(&self) -> Vec<Export> {
        let p = &self.frame;
        let (a, b) = self.base_bounds();
        let z = self.axis_z();
        let assembly_bounds = (
            a,
            [
                b[0],
                b[1],
                (z + p.cap_height_mm).max(self.carriage_z() + 2. * self.interface()[2]),
            ],
        );
        vec![
            Export {
                name: "frame_base_DRY_FIT_REVIEW.stl",
                part: self.base(),
                bounds: (a, b),
                role: "unqualified_flat_base_stock_material_and_fasteners_unselected",
            },
            Export {
                name: "frame_lower_support_DRY_FIT_REVIEW_make_two.stl",
                part: self.support(false).translate(0., 0., -p.base_thickness_mm),
                bounds: (
                    [-p.support_length_mm / 2., -p.support_width_mm / 2., 0.],
                    [
                        p.support_length_mm / 2.,
                        p.support_width_mm / 2.,
                        z - p.base_thickness_mm,
                    ],
                ),
                role: "two_lower_supports_required_hardware_fit_and_clamping_unverified",
            },
            Export {
                name: "frame_split_cap_DRY_FIT_REVIEW_make_two.stl",
                part: self.support(true).rotate(180., 0., 0.).translate(
                    0.,
                    0.,
                    z + p.cap_height_mm,
                ),
                bounds: (
                    [-p.support_length_mm / 2., -p.support_width_mm / 2., 0.],
                    [
                        p.support_length_mm / 2.,
                        p.support_width_mm / 2.,
                        p.cap_height_mm,
                    ],
                ),
                role: "two_caps_required_flat_face_down_saddles_up_clamping_unverified",
            },
            Export {
                name: "frame_REF_rod_pair_NOT_FOR_MANUFACTURE.stl",
                part: self.rods(),
                bounds: (
                    [
                        -p.support_length_mm,
                        -self.interface()[0] / 2. - self.interface()[1] / 2.,
                        z - self.interface()[1] / 2.,
                    ],
                    [
                        p.clear_span_mm + p.support_length_mm,
                        self.interface()[0] / 2. + self.interface()[1] / 2.,
                        z + self.interface()[1] / 2.,
                    ],
                ),
                role: "nominal_straight_rods_no_tolerance_straightness_or_surface_finish_claim",
            },
            Export {
                name: "frame_assembly_start_VIEW_ONLY.stl",
                part: self.assembly(p.carriage_start_x_mm),
                bounds: assembly_bounds,
                role: "static_start_position_no_motor_screw_or_syringe",
            },
            Export {
                name: "frame_assembly_end_VIEW_ONLY.stl",
                part: self.assembly(p.carriage_start_x_mm + p.travel_mm),
                bounds: assembly_bounds,
                role: "static_end_position_geometric_travel_only_not_observed_motion",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let p = &self.frame;
        let y = self.interface()[0] / 2.;
        let z = self.axis_z() - p.base_thickness_mm;
        vec![
            Probe {
                index: 0,
                name: "base_cap_fastener_through_path",
                point: [
                    -p.support_length_mm / 2.,
                    self.bolt_ys()[0],
                    p.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 1,
                name: "lower_rod_saddle_open",
                point: [0., y, z - self.radius() / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "lower_support_below_rod",
                point: [0., y, z - self.radius() - 1.],
                solid: true,
            },
            Probe {
                index: 1,
                name: "lower_cap_fastener_through",
                point: [0., self.bolt_ys()[0], z / 2.],
                solid: false,
            },
            Probe {
                index: 2,
                name: "upper_rod_saddle_open",
                point: [0., y, p.cap_height_mm - self.radius() / 2.],
                solid: false,
            },
            Probe {
                index: 2,
                name: "upper_cap_retention_land",
                point: [0., y, p.cap_height_mm - self.radius() - 1.],
                solid: true,
            },
            Probe {
                index: 2,
                name: "upper_cap_fastener_through",
                point: [0., self.bolt_ys()[0], p.cap_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "lead_axis_is_clearance_only",
                point: [0., 0., z - 1.],
                solid: false,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: Frame = toml::from_str(raw)?;
    p.validate()?;
    let f = &p.frame;
    let (a, b) = p.base_bounds();
    let design = json!({"axis":"X","rod_axis_yz_mm":[[-p.interface()[0]/2.,p.axis_z()],[p.interface()[0]/2.,p.axis_z()]],"rod_nominal_diameter_mm":p.interface()[1],"rod_reference_length_mm":f.clear_span_mm+2.*f.support_length_mm,"support_saddle_diameter_mm":2.*p.radius(),"carriage_floor_gap_mm":f.carriage_floor_gap_mm,"carriage_body_start_positions_x_mm":[f.carriage_start_x_mm,f.carriage_start_x_mm+f.travel_mm],"configured_geometric_travel_mm":f.travel_mm,"closest_end_gaps_mm":[f.carriage_start_x_mm-p.carriage.cap_thickness(),f.clear_span_mm-f.carriage_start_x_mm-f.travel_mm-p.carriage.body_length()],"base_envelope_mm":[b[0]-a[0],b[1]-a[1],b[2]],"support_center_x_mm":[-f.support_length_mm/2.,f.clear_span_mm+f.support_length_mm/2.],"cap_bolt_y_mm":p.bolt_ys(),"cap_export_rotation_degrees_xyz":[180.,0.,0.],"cap_export_translation_mm":[0.,0.,p.axis_z()+f.cap_height_mm],"parts_required":{"base":1,"lower_support":2,"split_cap":2,"reference_rod":2,"carriage_body":1,"carriage_cap":1},"rules":["Dry unpowered review only; do not manufacture or energize a working pump from these meshes.","Carriage geometry comes from the nested carriage configuration, not a separately guessed rod pattern.","Two endpoint views are static poses; they do not show observed motion or demonstrate alignment.","Split caps have a fixed seam and diametral clearance; preload, shims, rod retention and tolerance stack remain unresolved.","Vertical bolt paths run through cap, lower support and base. No fasteners, nuts, counterbores or under-base clearance are included.","Base is a review plate; material, flatness, stiffness and manufacturing method are not specified. Its footprint may exceed a printer bed.","Central support opening is clearance only, not a radial bearing, thrust bearing or lead-screw support.","No screw, actual nut, motor, coupler, syringe flange or plunger restraint, hard stops or limit switches are included.","Configured travel is an unloaded geometric envelope, not a syringe stroke or a delivery claim."]});
    publish(
        args,
        "water_bench_frame",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
