//! Independent, bench-anchored adjustable outlet support; dry candidates only.
use super::*;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct OutletSupport {
    base_length_mm: f64,
    base_width_mm: f64,
    base_thickness_mm: f64,
    rail_width_mm: f64,
    rail_thickness_mm: f64,
    rail_height_mm: f64,
    slot_bottom_z_mm: f64,
    slot_top_z_mm: f64,
    mounting_slot_width_mm: f64,
    mounting_pitch_mm: f64,
    arm_width_mm: f64,
    arm_reach_mm: f64,
    arm_thickness_mm: f64,
    bracket_height_mm: f64,
    bracket_thickness_mm: f64,
    mounting_center_z_mm: f64,
    tube_outer_diameter_mm: f64,
    tube_diametral_clearance_mm: f64,
    tube_end_margin_mm: f64,
    tie_slot_width_mm: f64,
    tie_slot_length_mm: f64,
    tie_slot_center_x_mm: f64,
    tube_projection_below_arm_mm: f64,
    anchor_hole_diameter_mm: f64,
    anchor_pitch_x_mm: f64,
    anchor_pitch_y_mm: f64,
}
impl OutletSupport {
    pub(super) fn tube_interface(&self) -> [f64; 3] {
        [
            0.,
            self.arm_y() + self.tube_y(),
            self.arm_z() - self.tube_projection_below_arm_mm,
        ]
    }
    pub(super) fn base_size(&self) -> [f64; 3] {
        [
            self.base_length_mm,
            self.base_width_mm,
            self.base_thickness_mm,
        ]
    }

    fn top(&self) -> f64 {
        self.base_thickness_mm + self.rail_height_mm
    }
    fn hole_radius(&self) -> f64 {
        (self.tube_outer_diameter_mm + self.tube_diametral_clearance_mm) / 2.
    }
    fn tube_y(&self) -> f64 {
        self.arm_reach_mm - self.tube_end_margin_mm
    }
    fn arm_z(&self) -> f64 {
        self.mounting_center_z_mm - self.bracket_height_mm / 2.
    }
    fn arm_y(&self) -> f64 {
        (self.rail_thickness_mm + self.bracket_thickness_mm) / 2.
    }
    pub(super) fn validate(&self) -> Result<(), String> {
        finite(self)?;
        require(
            (90. ..=160.).contains(&self.base_length_mm)
                && (80. ..=150.).contains(&self.base_width_mm)
                && (6. ..=12.).contains(&self.base_thickness_mm),
            "invalid support base",
        )?;
        require(
            (36. ..=60.).contains(&self.rail_width_mm)
                && (5. ..=10.).contains(&self.rail_thickness_mm)
                && (140. ..=240.).contains(&self.rail_height_mm),
            "invalid vertical rail",
        )?;
        require(
            self.rail_width_mm + 20. <= self.base_length_mm && self.top() <= 250.,
            "rail outside base/height envelope",
        )?;
        require(
            (4.2..=5.).contains(&self.mounting_slot_width_mm)
                && (16. ..=32.).contains(&self.mounting_pitch_mm)
                && self.mounting_pitch_mm + self.mounting_slot_width_mm + 8. <= self.rail_width_mm,
            "insufficient slot web/edge material",
        )?;
        require(
            self.slot_bottom_z_mm >= self.base_thickness_mm + 20.
                && self.slot_top_z_mm <= self.top() - 12.
                && self.slot_top_z_mm - self.slot_bottom_z_mm >= 60.,
            "invalid rail travel slot",
        )?;
        require(
            (36. ..=60.).contains(&self.arm_width_mm)
                && self.mounting_pitch_mm + self.mounting_slot_width_mm + 8. <= self.arm_width_mm
                && (80. ..=200.).contains(&self.arm_reach_mm)
                && (6. ..=12.).contains(&self.arm_thickness_mm),
            "invalid arm envelope",
        )?;
        require(
            (24. ..=40.).contains(&self.bracket_height_mm)
                && (5. ..=10.).contains(&self.bracket_thickness_mm)
                && self.bracket_height_mm / 2. - self.mounting_slot_width_mm / 2.
                    >= self.arm_thickness_mm + 2.,
            "bracket bolt access overlaps arm",
        )?;
        require(
            self.mounting_center_z_mm >= self.slot_bottom_z_mm + self.mounting_slot_width_mm / 2.
                && self.mounting_center_z_mm
                    <= self.slot_top_z_mm - self.mounting_slot_width_mm / 2.
                && self.arm_z() >= self.base_thickness_mm + 5.
                && self.arm_z() + self.bracket_height_mm <= self.top(),
            "selected height outside supported travel",
        )?;
        require(
            (2. ..=6.).contains(&self.tube_outer_diameter_mm)
                && (0.4..=1.).contains(&self.tube_diametral_clearance_mm)
                && (10. ..=20.).contains(&self.tube_end_margin_mm)
                && self.tube_end_margin_mm >= self.hole_radius() + 4.,
            "invalid tube hole/end web",
        )?;
        require(
            (3. ..=4.).contains(&self.tie_slot_width_mm)
                && (6. ..=10.).contains(&self.tie_slot_length_mm)
                && self.tie_slot_center_x_mm - self.tie_slot_width_mm / 2.
                    >= self.hole_radius() + 3.
                && self.tie_slot_center_x_mm + self.tie_slot_width_mm / 2. + 4.
                    <= self.arm_width_mm / 2.
                && self.tube_end_margin_mm >= self.tie_slot_length_mm / 2. + 4.,
            "insufficient tie slot web",
        )?;
        require(
            (10. ..=30.).contains(&self.tube_projection_below_arm_mm)
                && self.arm_z() - self.tube_projection_below_arm_mm > self.base_thickness_mm,
            "tube projection below base",
        )?;
        require(
            (4.2..=5.).contains(&self.anchor_hole_diameter_mm)
                && self.anchor_pitch_x_mm >= self.rail_width_mm + 12.
                && self.anchor_pitch_y_mm >= self.rail_thickness_mm + 20.
                && self.anchor_pitch_x_mm + self.anchor_hole_diameter_mm + 10.
                    <= self.base_length_mm
                && self.anchor_pitch_y_mm + self.anchor_hole_diameter_mm + 10.
                    <= self.base_width_mm,
            "invalid independent anchor pattern",
        )
    }
    fn stand(&self) -> Part {
        let mut part = block(
            "bench_anchor_base",
            self.base_length_mm,
            self.base_width_mm,
            self.base_thickness_mm,
            0.,
        ) + block(
            "vertical_slotted_rail",
            self.rail_width_mm,
            self.rail_thickness_mm,
            self.rail_height_mm + E,
            self.base_thickness_mm - E,
        );
        for sign in [-1., 1.] {
            part = part
                - block(
                    "vertical_adjustment_slot",
                    self.mounting_slot_width_mm,
                    self.rail_thickness_mm + 2. * E,
                    self.slot_top_z_mm - self.slot_bottom_z_mm,
                    self.slot_bottom_z_mm,
                )
                .translate(sign * self.mounting_pitch_mm / 2., 0., 0.);
            for side in [-1., 1.] {
                part = part
                    - cylinder(
                        "bench_anchor_clearance",
                        self.anchor_hole_diameter_mm / 2.,
                        self.base_thickness_mm + 2. * E,
                        -E,
                    )
                    .translate(
                        sign * self.anchor_pitch_x_mm / 2.,
                        side * self.anchor_pitch_y_mm / 2.,
                        0.,
                    );
            }
        }
        part
    }
    fn arm(&self) -> Part {
        let mut part = block(
            "clamp_face",
            self.arm_width_mm,
            self.bracket_thickness_mm,
            self.bracket_height_mm,
            0.,
        ) + block(
            "outlet_arm",
            self.arm_width_mm,
            self.arm_reach_mm,
            self.arm_thickness_mm,
            0.,
        )
        .translate(0., self.arm_reach_mm / 2., 0.);
        for sign in [-1., 1.] {
            part = part
                - centered_cylinder(
                    "M4_clamp_clearance",
                    self.mounting_slot_width_mm / 2.,
                    self.bracket_thickness_mm + 2. * E,
                    48,
                )
                .rotate(90., 0., 0.)
                .translate(
                    sign * self.mounting_pitch_mm / 2.,
                    0.,
                    self.bracket_height_mm / 2.,
                );
            part = part
                - block(
                    "tube_tie_slot",
                    self.tie_slot_width_mm,
                    self.tie_slot_length_mm,
                    self.arm_thickness_mm + 2. * E,
                    -E,
                )
                .translate(sign * self.tie_slot_center_x_mm, self.tube_y(), 0.);
        }
        part - cylinder(
            "free_outlet_tube_passage",
            self.hole_radius(),
            self.arm_thickness_mm + 2. * E,
            -E,
        )
        .translate(0., self.tube_y(), 0.)
    }
    pub(super) fn exports(&self) -> Vec<Export> {
        let xmin = -self.base_length_mm / 2.;
        let ymin = -self.base_width_mm / 2.;
        vec![
            Export {
                name: "outlet_support_stand_PRINT.stl",
                part: self.stand(),
                bounds: ([xmin, ymin, 0.], [-xmin, -ymin, self.top()]),
                role: "bench_anchored_dry_candidate_never_use_unrestrained",
            },
            Export {
                name: "outlet_support_arm_PRINT.stl",
                part: self.arm(),
                bounds: (
                    [-self.arm_width_mm / 2., -self.bracket_thickness_mm / 2., 0.],
                    [
                        self.arm_width_mm / 2.,
                        self.arm_reach_mm,
                        self.bracket_height_mm,
                    ],
                ),
                role: "separate_arm_clamp_hardware_required",
            },
            Export {
                name: "outlet_support_assembly_DRY_hardware_omitted.stl",
                part: self.stand() + self.arm().translate(0., self.arm_y(), self.arm_z()),
                bounds: (
                    [xmin, ymin, 0.],
                    [
                        -xmin,
                        (-ymin).max(self.arm_y() + self.arm_reach_mm),
                        self.top(),
                    ],
                ),
                role: "configured_pose_reference_fasteners_and_tubing_omitted",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        vec![
            Probe {
                index: 0,
                name: "anchored_base_floor",
                point: [0., self.base_width_mm / 3., self.base_thickness_mm / 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "anchor_clearance",
                point: [
                    self.anchor_pitch_x_mm / 2.,
                    self.anchor_pitch_y_mm / 2.,
                    self.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "vertical_rail_center_web",
                point: [0., 0., self.mounting_center_z_mm],
                solid: true,
            },
            Probe {
                index: 0,
                name: "vertical_adjustment_slot_open",
                point: [self.mounting_pitch_mm / 2., 0., self.mounting_center_z_mm],
                solid: false,
            },
            Probe {
                index: 1,
                name: "arm_bolt_passage",
                point: [self.mounting_pitch_mm / 2., 0., self.bracket_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "arm_mounting_web",
                point: [0., 0., self.bracket_height_mm - 3.],
                solid: true,
            },
            Probe {
                index: 1,
                name: "outlet_tube_passage",
                point: [0., self.tube_y(), self.arm_thickness_mm / 2.],
                solid: false,
            },
            Probe {
                index: 1,
                name: "tie_slot_open",
                point: [
                    self.tie_slot_center_x_mm,
                    self.tube_y(),
                    self.arm_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 1,
                name: "arm_load_path_web",
                point: [0., self.arm_reach_mm / 2., self.arm_thickness_mm / 2.],
                solid: true,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: OutletSupport = toml::from_str(raw)?;
    p.validate()?;
    let design = json!({"stand_outer_mm":[p.base_length_mm,p.base_width_mm,p.top()],"arm_translation_mm":[0.,p.arm_y(),p.arm_z()],"tube_axis_bench_xy_mm":[0.,p.arm_y()+p.tube_y()],"arm_underside_z_mm":p.arm_z(),"intended_tube_tip_z_mm":p.arm_z()-p.tube_projection_below_arm_mm,"adjustment_slot_z_mm":[p.slot_bottom_z_mm,p.slot_top_z_mm],"mounting_pitch_mm":p.mounting_pitch_mm,"assembly_rules":["Anchor or clamp the base independently to a suitable bench/board. Cantilever is not qualified as a free-standing support.","Two through-fasteners with washers/nuts clamp arm to vertical slots; select hardware stack after actual measurement. Fasteners omitted from STL.","Loosen and adjust only while stationary and supported; inspect slot travel, fastening and arm deflection before use.","Position the balance, pan, draft shield and vial independently beneath the tube; no scale fit is inferred from this model.","Tube end must remain clear of vial, shield and pan. Tube route/tie retention and minimum bend radius require physical inspection.","No connection from stand, tubing or arm into the weighing assembly. Do not clamp base to balance.","A height setting is a nominal pose, not a measured air gap or proof of stability/strength."],"bom":[{"item":"anchored stand","quantity":1},{"item":"separate adjustable outlet arm","quantity":1},{"item":"M4 arm bolts/washers/nuts and bench anchoring or clamps","selected":false},{"item":"tube and two ties","selected":false}],"physical_fit_verified":false});
    publish(
        args,
        "water_bench_outlet_support",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
