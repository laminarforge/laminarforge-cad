//! Non-actuated source-barrel support candidates, not a syringe drive.
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SourceSaddle {
    barrel_diameter_mm: f64,
    barrel_diametral_clearance_mm: f64,
    reference_barrel_length_mm: f64,
    saddle_length_mm: f64,
    saddle_top_z_mm: f64,
    saddle_wall_mm: f64,
    base_width_mm: f64,
    base_thickness_mm: f64,
    mount_pitch_x_mm: f64,
    mount_pitch_y_mm: f64,
    mount_hole_diameter_mm: f64,
    washer_envelope_diameter_mm: f64,
    tie_passage_width_mm: f64,
    tie_passage_height_mm: f64,
    tie_passage_bottom_mm: f64,
    support_center_spacing_mm: f64,
}
impl SourceSaddle {
    fn radius(&self) -> f64 {
        (self.barrel_diameter_mm + self.barrel_diametral_clearance_mm) / 2.
    }
    fn body_width(&self) -> f64 {
        2. * (self.radius() + self.saddle_wall_mm)
    }
    fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require(
            (10. ..=32.).contains(&self.barrel_diameter_mm)
                && (0.4..=1.).contains(&self.barrel_diametral_clearance_mm),
            "invalid barrel envelope",
        )?;
        require(
            (20. ..=36.).contains(&self.saddle_length_mm)
                && (18. ..=35.).contains(&self.saddle_top_z_mm)
                && (3. ..=6.).contains(&self.saddle_wall_mm)
                && (4. ..=8.).contains(&self.base_thickness_mm),
            "invalid saddle body/base",
        )?;
        require(
            self.saddle_top_z_mm - self.radius() >= self.base_thickness_mm + 3.,
            "insufficient floor below barrel saddle",
        )?;
        require(
            (48. ..=85.).contains(&self.base_width_mm)
                && self.body_width() + 10. <= self.base_width_mm,
            "insufficient base ears",
        )?;
        require(
            (3.2..=4.).contains(&self.mount_hole_diameter_mm)
                && (6. ..=9.).contains(&self.washer_envelope_diameter_mm)
                && self.washer_envelope_diameter_mm >= self.mount_hole_diameter_mm + 2.,
            "invalid nominal M3 clearance/washer envelope",
        )?;
        require(
            self.mount_pitch_x_mm >= self.tie_passage_width_mm + self.mount_hole_diameter_mm + 4.
                && self.mount_pitch_x_mm + self.mount_hole_diameter_mm + 6.
                    <= self.saddle_length_mm,
            "anchor conflicts with tie passage or base edge",
        )?;
        require(
            self.mount_pitch_y_mm >= self.body_width() + self.washer_envelope_diameter_mm + 4.
                && self.mount_pitch_y_mm + self.washer_envelope_diameter_mm + 6.
                    <= self.base_width_mm,
            "washer conflicts with saddle wall or base edge",
        )?;
        require(
            (3. ..=4.).contains(&self.tie_passage_width_mm)
                && (1.4..=2.2).contains(&self.tie_passage_height_mm)
                && self.tie_passage_bottom_mm >= 2.
                && self.tie_passage_bottom_mm + self.tie_passage_height_mm
                    <= self.base_thickness_mm - 1.6,
            "insufficient material around tie passage",
        )?;
        require(
            self.support_center_spacing_mm >= self.saddle_length_mm + 10.
                && self.support_center_spacing_mm <= 120.
                && self.reference_barrel_length_mm
                    >= self.support_center_spacing_mm + self.saddle_length_mm + 10.
                && self.reference_barrel_length_mm <= 200.,
            "invalid support spacing/reference barrel length",
        )
    }
    fn saddle(&self) -> Part {
        let mut part = block(
            "anchor_base",
            self.saddle_length_mm,
            self.base_width_mm,
            self.base_thickness_mm,
            0.,
        ) + block(
            "barrel_saddle_body",
            self.saddle_length_mm,
            self.body_width(),
            self.saddle_top_z_mm - self.base_thickness_mm + E,
            self.base_thickness_mm - E,
        );
        part = part
            - centered_cylinder(
                "open_half_round_barrel_saddle",
                self.radius(),
                self.saddle_length_mm + 2. * E,
                64,
            )
            .rotate(0., 90., 0.)
            .translate(0., 0., self.saddle_top_z_mm);
        part = part
            - block(
                "transverse_strap_passage",
                self.tie_passage_width_mm,
                self.base_width_mm + 2. * E,
                self.tie_passage_height_mm,
                self.tie_passage_bottom_mm,
            );
        for x in [-1., 1.] {
            for y in [-1., 1.] {
                part = part
                    - cylinder(
                        "mounting_clearance",
                        self.mount_hole_diameter_mm / 2.,
                        self.base_thickness_mm + 2. * E,
                        -E,
                    )
                    .translate(
                        x * self.mount_pitch_x_mm / 2.,
                        y * self.mount_pitch_y_mm / 2.,
                        0.,
                    );
            }
        }
        part
    }
    fn barrel(&self) -> Part {
        centered_cylinder(
            "REF_unselected_barrel",
            self.barrel_diameter_mm / 2.,
            self.reference_barrel_length_mm,
            64,
        )
        .rotate(0., 90., 0.)
    }
    fn exports(&self) -> Vec<Export> {
        let b = (
            [-self.saddle_length_mm / 2., -self.base_width_mm / 2., 0.],
            [
                self.saddle_length_mm / 2.,
                self.base_width_mm / 2.,
                self.saddle_top_z_mm,
            ],
        );
        vec![
            Export {
                name: "source_saddle_A_PRINT.stl",
                part: self.saddle(),
                bounds: b,
                role: "first_saddle_anchor_and_strap_required",
            },
            Export {
                name: "source_saddle_B_PRINT.stl",
                part: self.saddle(),
                bounds: b,
                role: "second_identical_saddle_anchor_and_strap_required",
            },
            Export {
                name: "source_REF_barrel_NOT_FOR_MANUFACTURE.stl",
                part: self.barrel(),
                bounds: (
                    [
                        -self.reference_barrel_length_mm / 2.,
                        -self.barrel_diameter_mm / 2.,
                        -self.barrel_diameter_mm / 2.,
                    ],
                    [
                        self.reference_barrel_length_mm / 2.,
                        self.barrel_diameter_mm / 2.,
                        self.barrel_diameter_mm / 2.,
                    ],
                ),
                role: "unselected_barrel_envelope_no_flange_plunger_or_outlet",
            },
            Export {
                name: "source_assembly_DRY_with_REF_barrel.stl",
                part: self
                    .saddle()
                    .translate(-self.support_center_spacing_mm / 2., 0., 0.)
                    + self
                        .saddle()
                        .translate(self.support_center_spacing_mm / 2., 0., 0.)
                    + self.barrel().translate(
                        0.,
                        0.,
                        self.saddle_top_z_mm - self.barrel_diametral_clearance_mm / 2.,
                    ),
                bounds: (
                    [
                        -self.reference_barrel_length_mm / 2.,
                        -self.base_width_mm / 2.,
                        0.,
                    ],
                    [
                        self.reference_barrel_length_mm / 2.,
                        self.base_width_mm / 2.,
                        self.saddle_top_z_mm - self.barrel_diametral_clearance_mm / 2.
                            + self.barrel_diameter_mm / 2.,
                    ],
                ),
                role: "resting_pose_reference_straps_fasteners_and_source_features_omitted",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        vec![
            Probe {
                index: 0,
                name: "barrel_saddle_open",
                point: [0., 0., self.saddle_top_z_mm - 1.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "barrel_floor_web",
                point: [0., 0., self.base_thickness_mm + 1.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "saddle_side_wall",
                point: [
                    0.,
                    self.radius() + self.saddle_wall_mm / 2.,
                    self.saddle_top_z_mm - 1.,
                ],
                solid: true,
            },
            Probe {
                index: 0,
                name: "transverse_tie_passage",
                point: [
                    0.,
                    0.,
                    self.tie_passage_bottom_mm + self.tie_passage_height_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "anchor_clearance",
                point: [
                    self.mount_pitch_x_mm / 2.,
                    self.mount_pitch_y_mm / 2.,
                    self.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "base_ear_material",
                point: [
                    self.saddle_length_mm / 2. - 1.,
                    self.base_width_mm / 2. - 1.,
                    self.base_thickness_mm / 2.,
                ],
                solid: true,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: SourceSaddle = toml::from_str(raw)?;
    p.validate()?;
    let design = json!({"barrel_axis":"X","saddle_outer_mm":[p.saddle_length_mm,p.base_width_mm,p.saddle_top_z_mm],"support_center_spacing_mm":p.support_center_spacing_mm,"assembly_transforms":[{"file":"source_saddle_A_PRINT.stl","translation_mm":[-p.support_center_spacing_mm/2.,0.,0.]},{"file":"source_saddle_B_PRINT.stl","translation_mm":[p.support_center_spacing_mm/2.,0.,0.]},{"file":"source_REF_barrel_NOT_FOR_MANUFACTURE.stl","translation_mm":[0.,0.,p.saddle_top_z_mm-p.barrel_diametral_clearance_mm/2.]}],"mount_pattern_per_saddle_mm":[p.mount_pitch_x_mm,p.mount_pitch_y_mm],"rules":["Two identical saddle meshes are the two physical supports, not four copies.","Independently anchor both bases to a suitable board/bench and select straps after actual barrel measurement.","Half-round saddle is not axial flange retention or a plunger restraint. No automated drive load is qualified.","Reference barrel has no actual flange, plunger, taper or outlet. Confirm source clearances and access from measured hardware.","Resting reference pose accounts for nominal radial saddle clearance; actual contact, print fit and deformation require inspection.","Do not overtighten a strap or infer a pressure rating, barrel strength or constant flow from this support."],"bom":[{"item":"source saddle","quantity":2},{"item":"base fastening hardware and two straps","selected":false},{"item":"actual source syringe/reservoir","selected":false}],"physical_fit_verified":false});
    publish(
        args,
        "water_bench_source_saddle",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
