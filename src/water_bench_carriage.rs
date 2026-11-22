//! Separate dry carriage review: X-axis through passages and removable retention.
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Carriage {
    body_length_mm: f64,
    body_width_mm: f64,
    body_height_mm: f64,
    rod_diameter_mm: f64,
    rod_diametral_clearance_mm: f64,
    rod_center_pitch_mm: f64,
    bearing_outer_diameter_mm: f64,
    bearing_length_mm: f64,
    bearing_diametral_clearance_mm: f64,
    bearing_axial_clearance_mm: f64,
    lead_screw_diameter_mm: f64,
    lead_screw_diametral_clearance_mm: f64,
    nut_width_mm: f64,
    nut_height_mm: f64,
    nut_length_mm: f64,
    nut_side_clearance_mm: f64,
    nut_axial_clearance_mm: f64,
    cap_thickness_mm: f64,
    cap_hole_diameter_mm: f64,
    cap_bolt_pitch_y_mm: f64,
    cap_bolt_pitch_z_mm: f64,
    minimum_wall_mm: f64,
}
impl Carriage {
    fn axis_z(&self) -> f64 {
        self.body_height_mm / 2.
    }
    fn pocket_radius(&self) -> f64 {
        (self.bearing_outer_diameter_mm + self.bearing_diametral_clearance_mm) / 2.
    }
    fn rod_radius(&self) -> f64 {
        (self.rod_diameter_mm + self.rod_diametral_clearance_mm) / 2.
    }
    fn lead_radius(&self) -> f64 {
        (self.lead_screw_diameter_mm + self.lead_screw_diametral_clearance_mm) / 2.
    }
    fn pocket_length(&self) -> f64 {
        self.bearing_length_mm + self.bearing_axial_clearance_mm
    }
    fn nut_depth(&self) -> f64 {
        self.nut_length_mm + self.nut_axial_clearance_mm
    }
    fn nut_half_y(&self) -> f64 {
        (self.nut_width_mm + self.nut_side_clearance_mm) / 2.
    }
    fn nut_half_z(&self) -> f64 {
        (self.nut_height_mm + self.nut_side_clearance_mm) / 2.
    }
    fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require(
            (32. ..=55.).contains(&self.body_length_mm)
                && (60. ..=100.).contains(&self.body_width_mm)
                && (30. ..=50.).contains(&self.body_height_mm)
                && (3. ..=6.).contains(&self.minimum_wall_mm),
            "invalid carriage envelope or minimum web",
        )?;
        require(
            (6. ..=12.).contains(&self.rod_diameter_mm)
                && (0.4..=1.).contains(&self.rod_diametral_clearance_mm)
                && (36. ..=70.).contains(&self.rod_center_pitch_mm),
            "invalid rod clearance/pitch",
        )?;
        require(
            self.bearing_outer_diameter_mm >= self.rod_diameter_mm + 5.
                && self.bearing_outer_diameter_mm <= 24.
                && (16. ..=35.).contains(&self.bearing_length_mm)
                && (0.15..=0.6).contains(&self.bearing_diametral_clearance_mm)
                && (0.3..=1.).contains(&self.bearing_axial_clearance_mm),
            "invalid bearing envelope/clearances",
        )?;
        require(
            self.body_length_mm - self.pocket_length() >= self.minimum_wall_mm
                && self.pocket_radius() - self.rod_radius() >= self.minimum_wall_mm,
            "bearing shoulder has insufficient axial/radial land",
        )?;
        require(
            self.body_width_mm / 2. - self.rod_center_pitch_mm / 2. - self.pocket_radius()
                >= self.minimum_wall_mm
                && self.axis_z() - self.pocket_radius() >= self.minimum_wall_mm,
            "bearing pocket breaks body wall",
        )?;
        require(
            (6. ..=10.).contains(&self.lead_screw_diameter_mm)
                && (0.4..=1.).contains(&self.lead_screw_diametral_clearance_mm)
                && (14. ..=26.).contains(&self.nut_width_mm)
                && (14. ..=26.).contains(&self.nut_height_mm)
                && (10. ..=22.).contains(&self.nut_length_mm)
                && (0.3..=0.8).contains(&self.nut_side_clearance_mm)
                && (0.3..=0.8).contains(&self.nut_axial_clearance_mm),
            "invalid provisional rectangular nut/lead clearance",
        )?;
        require(
            self.rod_center_pitch_mm / 2. - self.pocket_radius() - self.nut_half_y()
                >= self.minimum_wall_mm
                && self.axis_z() - self.nut_half_z() >= self.minimum_wall_mm
                && self.body_length_mm - self.nut_depth() >= self.minimum_wall_mm
                && self.nut_half_y().min(self.nut_half_z()) - self.lead_radius()
                    >= self.minimum_wall_mm,
            "nut pocket conflicts with bearing or has insufficient shoulder",
        )?;
        require(
            (3. ..=6.).contains(&self.cap_thickness_mm)
                && (3.2..=4.).contains(&self.cap_hole_diameter_mm)
                && self.cap_bolt_pitch_y_mm >= 20.
                && self.cap_bolt_pitch_z_mm >= 20.,
            "invalid removable cap/hole pattern",
        )?;
        let y = self.cap_bolt_pitch_y_mm / 2.;
        let z = self.cap_bolt_pitch_z_mm / 2.;
        let r = self.cap_hole_diameter_mm / 2.;
        let bearing_distance = ((self.rod_center_pitch_mm / 2. - y).powi(2) + z.powi(2)).sqrt();
        let nut_distance = ((y - self.nut_half_y()).max(0.).powi(2)
            + (z - self.nut_half_z()).max(0.).powi(2))
        .sqrt();
        require(
            self.body_width_mm / 2. - y - r >= self.minimum_wall_mm
                && self.axis_z() - z - r >= self.minimum_wall_mm
                && bearing_distance - self.pocket_radius() - r >= self.minimum_wall_mm
                && nut_distance - r >= self.minimum_wall_mm,
            "cap bolt conflicts with pockets or edge land",
        )
    }
    fn x_cylinder(&self, name: &str, r: f64, len: f64, start: f64, y: f64, z: f64) -> Part {
        centered_cylinder(name, r, len, 64)
            .rotate(0., 90., 0.)
            .translate(start + len / 2., y, z)
    }
    fn bolt_holes(&self, mut part: Part, start: f64, len: f64) -> Part {
        for y in [-1., 1.] {
            for z in [-1., 1.] {
                part = part
                    - self.x_cylinder(
                        "cap_through_bolt_clearance",
                        self.cap_hole_diameter_mm / 2.,
                        len + 2. * E,
                        start - E,
                        y * self.cap_bolt_pitch_y_mm / 2.,
                        self.axis_z() + z * self.cap_bolt_pitch_z_mm / 2.,
                    );
            }
        }
        part
    }
    fn body(&self) -> Part {
        let mut part = block(
            "carriage_body",
            self.body_length_mm,
            self.body_width_mm,
            self.body_height_mm,
            0.,
        )
        .translate(self.body_length_mm / 2., 0., 0.);
        for y in [
            -self.rod_center_pitch_mm / 2.,
            self.rod_center_pitch_mm / 2.,
        ] {
            part = part
                - self.x_cylinder(
                    "full_length_rod_clearance",
                    self.rod_radius(),
                    self.body_length_mm + 2. * E,
                    -E,
                    y,
                    self.axis_z(),
                );
            part = part
                - self.x_cylinder(
                    "rear_loading_bearing_pocket",
                    self.pocket_radius(),
                    self.pocket_length() + E,
                    -E,
                    y,
                    self.axis_z(),
                );
        }
        part = part
            - block(
                "rectangular_anti_rotation_nut_review_pocket",
                self.nut_depth() + E,
                2. * self.nut_half_y(),
                2. * self.nut_half_z(),
                self.axis_z() - self.nut_half_z(),
            )
            .translate((self.nut_depth() - E) / 2., 0., 0.);
        part = part
            - self.x_cylinder(
                "X_axis_lead_screw_clearance",
                self.lead_radius(),
                self.body_length_mm + 2. * E,
                -E,
                0.,
                self.axis_z(),
            );
        self.bolt_holes(part, 0., self.body_length_mm)
    }
    fn cap(&self) -> Part {
        let mut part = block(
            "removable_bearing_and_nut_end_plate",
            self.cap_thickness_mm,
            self.body_width_mm,
            self.body_height_mm,
            0.,
        )
        .translate(-self.cap_thickness_mm / 2., 0., 0.);
        for y in [
            -self.rod_center_pitch_mm / 2.,
            self.rod_center_pitch_mm / 2.,
        ] {
            part = part
                - self.x_cylinder(
                    "cap_rod_passage",
                    self.rod_radius(),
                    self.cap_thickness_mm + 2. * E,
                    -self.cap_thickness_mm - E,
                    y,
                    self.axis_z(),
                );
        }
        part = part
            - self.x_cylinder(
                "cap_lead_screw_passage",
                self.lead_radius(),
                self.cap_thickness_mm + 2. * E,
                -self.cap_thickness_mm - E,
                0.,
                self.axis_z(),
            );
        self.bolt_holes(part, -self.cap_thickness_mm, self.cap_thickness_mm)
    }
    fn bearings(&self) -> Part {
        let bearing = |y| {
            self.x_cylinder(
                "REF_bearing_OD",
                self.bearing_outer_diameter_mm / 2.,
                self.bearing_length_mm,
                self.bearing_axial_clearance_mm / 2.,
                y,
                self.axis_z(),
            ) - self.x_cylinder(
                "REF_nominal_bearing_ID",
                self.rod_diameter_mm / 2.,
                self.bearing_length_mm + 2. * E,
                self.bearing_axial_clearance_mm / 2. - E,
                y,
                self.axis_z(),
            )
        };
        bearing(-self.rod_center_pitch_mm / 2.) + bearing(self.rod_center_pitch_mm / 2.)
    }
    fn nut(&self) -> Part {
        block(
            "REF_unselected_rectangular_nut_envelope",
            self.nut_length_mm,
            self.nut_width_mm,
            self.nut_height_mm,
            self.axis_z() - self.nut_height_mm / 2.,
        )
        .translate(
            self.nut_length_mm / 2. + self.nut_axial_clearance_mm / 2.,
            0.,
            0.,
        ) - self.x_cylinder(
            "REF_unthreaded_lead_path",
            self.lead_screw_diameter_mm / 2.,
            self.nut_length_mm + 2. * E,
            self.nut_axial_clearance_mm / 2. - E,
            0.,
            self.axis_z(),
        )
    }
    fn exports(&self) -> Vec<Export> {
        let by = self.rod_center_pitch_mm / 2. + self.bearing_outer_diameter_mm / 2.;
        let bz = self.bearing_outer_diameter_mm / 2.;
        vec![
            Export {
                name: "carriage_body_DRY_FIT_REVIEW.stl",
                part: self
                    .body()
                    .rotate(0., 90., 0.)
                    .translate(0., 0., self.body_length_mm),
                bounds: (
                    [0., -self.body_width_mm / 2., 0.],
                    [
                        self.body_height_mm,
                        self.body_width_mm / 2.,
                        self.body_length_mm,
                    ],
                ),
                role: "pocket_up_review_orientation_not_functional_pump_release",
            },
            Export {
                name: "carriage_cap_DRY_FIT_REVIEW.stl",
                part: self.cap().rotate(0., 90., 0.),
                bounds: (
                    [0., -self.body_width_mm / 2., 0.],
                    [
                        self.body_height_mm,
                        self.body_width_mm / 2.,
                        self.cap_thickness_mm,
                    ],
                ),
                role: "separate_removable_end_plate_fasteners_omitted",
            },
            Export {
                name: "carriage_REF_bearing_pair_NOT_FOR_MANUFACTURE.stl",
                part: self.bearings(),
                bounds: (
                    [
                        self.bearing_axial_clearance_mm / 2.,
                        -by,
                        self.axis_z() - bz,
                    ],
                    [
                        self.bearing_length_mm + self.bearing_axial_clearance_mm / 2.,
                        by,
                        self.axis_z() + bz,
                    ],
                ),
                role: "nominal_bearing_envelopes_no_rolling_elements_or_fitted_hardware",
            },
            Export {
                name: "carriage_REF_rectangular_nut_NOT_FOR_MANUFACTURE.stl",
                part: self.nut(),
                bounds: (
                    [
                        self.nut_axial_clearance_mm / 2.,
                        -self.nut_width_mm / 2.,
                        self.axis_z() - self.nut_height_mm / 2.,
                    ],
                    [
                        self.nut_length_mm + self.nut_axial_clearance_mm / 2.,
                        self.nut_width_mm / 2.,
                        self.axis_z() + self.nut_height_mm / 2.,
                    ],
                ),
                role: "provisional_rectangular_envelope_not_a_selected_commercial_nut_or_thread",
            },
            Export {
                name: "carriage_assembly_DRY_with_REF_parts.stl",
                part: self.body() + self.cap() + self.bearings() + self.nut(),
                bounds: (
                    [-self.cap_thickness_mm, -self.body_width_mm / 2., 0.],
                    [
                        self.body_length_mm,
                        self.body_width_mm / 2.,
                        self.body_height_mm,
                    ],
                ),
                role: "configured_mechanical_review_no_rods_screw_motor_or_syringe",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let body_point = |x: f64, y: f64, z: f64| [z, y, self.body_length_mm - x];
        let cap_point = |x: f64, y: f64, z: f64| [z, y, -x];
        let y = self.rod_center_pitch_mm / 2.;
        let radial = (self.pocket_radius() + self.rod_radius()) / 2.;
        vec![
            Probe {
                index: 0,
                name: "rod_clearance_continues_through_front_end",
                point: body_point(self.body_length_mm - 1., y, self.axis_z()),
                solid: false,
            },
            Probe {
                index: 0,
                name: "bearing_can_enter_from_rear",
                point: body_point(1., y + radial, self.axis_z()),
                solid: false,
            },
            Probe {
                index: 0,
                name: "bearing_forward_shoulder",
                point: body_point(self.pocket_length() + 1., y + radial, self.axis_z()),
                solid: true,
            },
            Probe {
                index: 0,
                name: "lead_screw_axis_is_X_full_through",
                point: body_point(self.body_length_mm - 1., 0., self.axis_z()),
                solid: false,
            },
            Probe {
                index: 0,
                name: "rectangular_nut_entry",
                point: body_point(1., self.nut_half_y() - 2., self.axis_z()),
                solid: false,
            },
            Probe {
                index: 0,
                name: "nut_forward_shoulder",
                point: body_point(self.nut_depth() + 1., self.nut_half_y() - 2., self.axis_z()),
                solid: true,
            },
            Probe {
                index: 1,
                name: "cap_rod_passage",
                point: cap_point(-self.cap_thickness_mm / 2., y, self.axis_z()),
                solid: false,
            },
            Probe {
                index: 1,
                name: "cap_retains_bearing_outer_land",
                point: cap_point(-self.cap_thickness_mm / 2., y + radial, self.axis_z()),
                solid: true,
            },
            Probe {
                index: 1,
                name: "cap_lead_passage",
                point: cap_point(-self.cap_thickness_mm / 2., 0., self.axis_z()),
                solid: false,
            },
            Probe {
                index: 1,
                name: "cap_retains_nut_outer_land",
                point: cap_point(
                    -self.cap_thickness_mm / 2.,
                    self.nut_half_y() - 2.,
                    self.axis_z(),
                ),
                solid: true,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: Carriage = toml::from_str(raw)?;
    p.validate()?;
    let design = json!({"motion_and_bore_axis":"X","bearing_loading_face_x_mm":0.,"bearing_pocket_depth_mm":p.pocket_length(),"full_rod_passage_diameter_mm":2.*p.rod_radius(),"rectangular_nut_pocket_mm":[p.nut_depth(),2.*p.nut_half_y(),2.*p.nut_half_z()],"body_print_rotation_degrees_xyz":[0.,90.,0.],"body_print_translation_mm":[0.,0.,p.body_length_mm],"cap_print_rotation_degrees_xyz":[0.,90.,0.],"assembly_axes":"unrotated engineering frame; cap at X -thickness to 0, body X 0 to length","cap_bolt_pitch_yz_mm":[p.cap_bolt_pitch_y_mm,p.cap_bolt_pitch_z_mm],"rules":["Historical frozen pump model is unchanged. This separate review is not a working pump.","Full-length rod bores run along X; rear-open bearing pockets allow insertion before the separate cap is fitted.","Front shoulders and removable cap provide nominal axial end lands. Actual endplay, fit, shimming and fastener stack need review.","Rectangular nut pocket gives nominal flat restraint; its dimensions are provisional and do not match an adopted commercial nut.","Reference nut has an unthreaded visual hole, not a manufactured thread or a printable lead nut.","No rods, lead screw, motor, coupler, guide frame, plunger/flange restraint, stops or electrical controls are included.","No force, bearing alignment, torque, anti-backlash, travel, motion, water delivery or pressure qualification is established.","Dry-fit meshes need slicer/material and hardware review before fabrication; do not energize based on this geometry."]});
    publish(
        args,
        "water_bench_carriage",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
