//! Provisional water-only syringe restraint interfaces, not a powered pump.
use super::frame::Frame;
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Restraints {
    restraint: Parameters,
    guide: Frame,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    axis_height_mm: f64,
    barrel_diameter_mm: f64,
    flange_width_mm: f64,
    flange_height_mm: f64,
    flange_thickness_mm: f64,
    stem_diameter_mm: f64,
    thumb_width_mm: f64,
    thumb_height_mm: f64,
    thumb_thickness_mm: f64,
    diametral_clearance_mm: f64,
    axial_clearance_mm: f64,
    adapter_thickness_mm: f64,
    keeper_thickness_mm: f64,
    keeper_width_mm: f64,
    keeper_bolt_pitch_mm: f64,
    keeper_hole_mm: f64,
    spacer_outer_diameter_mm: f64,
    yoke_center_x_mm: f64,
    yoke_base_length_mm: f64,
    yoke_base_width_mm: f64,
    yoke_base_thickness_mm: f64,
    yoke_plate_thickness_mm: f64,
    yoke_edge_land_mm: f64,
    anchor_pitch_x_mm: f64,
    anchor_pitch_y_mm: f64,
    anchor_hole_mm: f64,
    strap_width_mm: f64,
    strap_height_mm: f64,
    strap_bottom_mm: f64,
    web_length_mm: f64,
    web_width_mm: f64,
    web_height_mm: f64,
    web_center_y_mm: f64,
    minimum_wall_mm: f64,
}
impl Restraints {
    fn p(&self) -> &Parameters {
        &self.restraint
    }
    fn c(&self) -> [f64; 9] {
        self.guide.restraint_interface()
    }
    fn axis_local(&self) -> f64 {
        self.p().axis_height_mm - self.guide.restraint_positions()[2]
    }
    fn top(&self) -> f64 {
        self.axis_local() + self.p().thumb_height_mm / 2. + 4.
    }
    fn keeper_bottom(&self) -> f64 {
        self.axis_local() - self.p().thumb_height_mm / 2. - 4.
    }
    fn rear_x(&self) -> f64 {
        -self.c()[2] - self.p().adapter_thickness_mm
    }
    fn pocket(&self) -> f64 {
        self.p().thumb_thickness_mm + self.p().axial_clearance_mm
    }
    fn keeper_x(&self) -> f64 {
        -self.c()[2] + self.pocket()
    }
    fn yoke_gap(&self) -> f64 {
        self.p().flange_thickness_mm + self.p().axial_clearance_mm
    }
    fn yoke_top(&self) -> f64 {
        self.p().axis_height_mm + self.p().flange_height_mm / 2. + self.p().yoke_edge_land_mm
    }
    fn validate(&self) -> Result<(), String> {
        self.guide.validate()?;
        finite(self.p())?;
        let p = self.p();
        let c = self.c();
        let pos = self.guide.restraint_positions();
        let w = p.minimum_wall_mm;
        require(
            (3. ..=5.).contains(&w)
                && (60. ..=100.).contains(&p.axis_height_mm)
                && (10. ..=32.).contains(&p.barrel_diameter_mm)
                && (6. ..=16.).contains(&p.stem_diameter_mm),
            "invalid provisional syringe axes or diameters",
        )?;
        require(
            (0.4..=1.).contains(&p.diametral_clearance_mm)
                && (0.4..=1.).contains(&p.axial_clearance_mm)
                && (2. ..=5.).contains(&p.flange_thickness_mm)
                && (3. ..=6.).contains(&p.thumb_thickness_mm),
            "invalid dry interface clearances or thicknesses",
        )?;
        require(
            p.flange_width_mm >= p.barrel_diameter_mm + 2. * w
                && p.flange_height_mm >= p.barrel_diameter_mm + p.diametral_clearance_mm + 2. * w
                && p.thumb_width_mm >= p.stem_diameter_mm + 2. * w
                && p.thumb_height_mm >= p.stem_diameter_mm + p.diametral_clearance_mm + 2. * w,
            "insufficient flange or thumb overlap around open slots",
        )?;
        require(
            (4. ..=8.).contains(&p.adapter_thickness_mm)
                && (3. ..=6.).contains(&p.keeper_thickness_mm)
                && (3.2..=3.6).contains(&p.keeper_hole_mm)
                && p.keeper_width_mm <= c[0]
                && p.keeper_width_mm / 2.
                    - p.keeper_bolt_pitch_mm / 2.
                    - p.spacer_outer_diameter_mm / 2.
                    >= w
                && p.keeper_bolt_pitch_mm / 2.
                    - p.spacer_outer_diameter_mm / 2.
                    - p.thumb_width_mm / 2.
                    >= w
                && p.spacer_outer_diameter_mm / 2. - p.keeper_hole_mm / 2. >= 2.,
            "keeper spacers, thumb or bolt edge lands conflict",
        )?;
        require(
            pos[0] + self.rear_x() >= pos[3] && self.keeper_bottom() > c[1] + w,
            "adapter crosses rear guide support or keeper crosses carriage",
        )?;
        let b = self.guide.base_bounds();
        require(
            (40. ..=65.).contains(&p.yoke_base_length_mm)
                && (80. ..=110.).contains(&p.yoke_base_width_mm)
                && (8. ..=10.).contains(&p.yoke_base_thickness_mm)
                && (4. ..=8.).contains(&p.yoke_plate_thickness_mm)
                && p.yoke_edge_land_mm >= w
                && p.yoke_center_x_mm - p.yoke_base_length_mm / 2. >= b.1[0] + 10.
                && p.yoke_base_length_mm / 2. - self.yoke_gap() / 2. - p.yoke_plate_thickness_mm
                    >= w
                && p.yoke_base_width_mm / 2. - p.flange_width_mm / 2. >= w,
            "fixed yoke overlaps guide base or lacks envelope lands",
        )?;
        require(
            p.anchor_hole_mm >= 4.3
                && p.anchor_hole_mm <= 5.
                && p.yoke_base_length_mm / 2. - p.anchor_pitch_x_mm / 2. - p.anchor_hole_mm / 2.
                    >= w
                && p.yoke_base_width_mm / 2. - p.anchor_pitch_y_mm / 2. - p.anchor_hole_mm / 2.
                    >= w
                && p.anchor_pitch_x_mm / 2.
                    - p.anchor_hole_mm / 2.
                    - self.yoke_gap() / 2.
                    - p.yoke_plate_thickness_mm
                    >= w
                && p.anchor_pitch_y_mm / 2. - p.flange_width_mm / 2. - 4. >= w,
            "yoke anchor land or top bolt access conflicts",
        )?;
        require(
            (6. ..=10.).contains(&p.web_length_mm)
                && (3. ..=5.).contains(&p.web_width_mm)
                && (20. ..=40.).contains(&p.web_height_mm)
                && p.web_center_y_mm - p.web_width_mm / 2. >= p.flange_width_mm / 2. + w
                && p.anchor_pitch_y_mm / 2. - 4. - p.web_center_y_mm - p.web_width_mm / 2. >= w
                && p.yoke_base_length_mm / 2.
                    - self.yoke_gap() / 2.
                    - p.yoke_plate_thickness_mm
                    - p.web_length_mm
                    >= w,
            "side braces conflict with flange, anchors or base edge",
        )?;
        require(
            (3. ..=4.).contains(&p.strap_width_mm)
                && (1.4..=2.).contains(&p.strap_height_mm)
                && p.strap_bottom_mm >= 2.
                && p.strap_bottom_mm + p.strap_height_mm + p.strap_width_mm / 2. * 3_f64.sqrt()
                    <= p.yoke_base_thickness_mm - 1.6
                && p.strap_width_mm >= self.yoke_gap() - 0.5,
            "invalid transverse strap passage",
        )
    }
    fn xcyl(&self, name: &str, r: f64, x: f64, length: f64, y: f64, z: f64) -> Part {
        centered_cylinder(name, r, length, 64)
            .rotate(0., 90., 0.)
            .translate(x + length / 2., y, z)
    }
    fn ucut(&self, x: f64, length: f64, width: f64, axis: f64, top: f64) -> Part {
        block(
            "open_top_stem_or_barrel_slot",
            length,
            width,
            top - axis + width / 2. + E,
            axis - width / 2.,
        )
        .translate(x + length / 2., 0., 0.)
    }
    fn adapter(&self) -> Part {
        let p = self.p();
        let c = self.c();
        let mut a = block(
            "rear_cap_bolt_pattern_adapter",
            p.adapter_thickness_mm,
            c[0],
            self.top(),
            0.,
        )
        .translate(self.rear_x() + p.adapter_thickness_mm / 2., 0., 0.);
        for (y, d) in [(-c[6] / 2., c[7]), (c[6] / 2., c[7]), (0., c[8])] {
            a = a - self.xcyl(
                "guide_or_screw_passage",
                d / 2.,
                self.rear_x() - E,
                p.adapter_thickness_mm + 2. * E,
                y,
                c[1] / 2.,
            );
        }
        for y in [-c[3] / 2., c[3] / 2.] {
            for z in [c[1] / 2. - c[4] / 2., c[1] / 2. + c[4] / 2.] {
                a = a - self.xcyl(
                    "existing_cap_bolt_path",
                    c[5] / 2.,
                    self.rear_x() - E,
                    p.adapter_thickness_mm + 2. * E,
                    y,
                    z,
                );
            }
        }
        for y in [-p.keeper_bolt_pitch_mm / 2., p.keeper_bolt_pitch_mm / 2.] {
            a = a - self.xcyl(
                "keeper_bolt_path",
                p.keeper_hole_mm / 2.,
                self.rear_x() - E,
                p.adapter_thickness_mm + 2. * E,
                y,
                self.axis_local(),
            );
        }
        a
    }
    fn keeper(&self) -> Part {
        let p = self.p();
        let mut a = block(
            "removable_thumb_keeper",
            p.keeper_thickness_mm,
            p.keeper_width_mm,
            self.top() - self.keeper_bottom(),
            self.keeper_bottom(),
        )
        .translate(self.keeper_x() + p.keeper_thickness_mm / 2., 0., 0.)
            - self.ucut(
                self.keeper_x() - E,
                p.keeper_thickness_mm + 2. * E,
                p.stem_diameter_mm + p.diametral_clearance_mm,
                self.axis_local(),
                self.top(),
            );
        for y in [-p.keeper_bolt_pitch_mm / 2., p.keeper_bolt_pitch_mm / 2.] {
            a = a - self.xcyl(
                "keeper_bolt_through",
                p.keeper_hole_mm / 2.,
                self.keeper_x() - E,
                p.keeper_thickness_mm + 2. * E,
                y,
                self.axis_local(),
            );
        }
        a
    }
    fn spacer(&self) -> Part {
        let p = self.p();
        cylinder(
            "keeper_hard_spacer",
            p.spacer_outer_diameter_mm / 2.,
            self.pocket(),
            0.,
        ) - cylinder(
            "spacer_bolt_clearance",
            p.keeper_hole_mm / 2.,
            self.pocket() + 2. * E,
            -E,
        )
    }
    fn moving(&self) -> Part {
        let p = self.p();
        let mut a = self.adapter() + self.keeper();
        for y in [-p.keeper_bolt_pitch_mm / 2., p.keeper_bolt_pitch_mm / 2.] {
            a = a + self
                .spacer()
                .rotate(0., 90., 0.)
                .translate(-self.c()[2], y, self.axis_local());
        }
        a
    }
    fn thumb(&self) -> Part {
        let p = self.p();
        block(
            "REF_unmeasured_thumb",
            p.thumb_thickness_mm,
            p.thumb_width_mm,
            p.thumb_height_mm,
            self.axis_local() - p.thumb_height_mm / 2.,
        )
        .translate(
            -self.c()[2] + p.axial_clearance_mm / 2. + p.thumb_thickness_mm / 2.,
            0.,
            0.,
        )
    }
    fn yoke(&self) -> Part {
        let p = self.p();
        let mut a = block(
            "independently_anchored_flange_yoke",
            p.yoke_base_length_mm,
            p.yoke_base_width_mm,
            p.yoke_base_thickness_mm,
            0.,
        );
        for (x, width) in [
            (
                -self.yoke_gap() / 2. - p.yoke_plate_thickness_mm,
                p.stem_diameter_mm + p.diametral_clearance_mm,
            ),
            (
                self.yoke_gap() / 2.,
                p.barrel_diameter_mm + p.diametral_clearance_mm,
            ),
        ] {
            a = a
                + (block(
                    "fixed_flange_capture_face",
                    p.yoke_plate_thickness_mm,
                    p.yoke_base_width_mm,
                    self.yoke_top() - p.yoke_base_thickness_mm + E,
                    p.yoke_base_thickness_mm - E,
                )
                .translate(x + p.yoke_plate_thickness_mm / 2., 0., 0.)
                    - self.ucut(
                        x - E,
                        p.yoke_plate_thickness_mm + 2. * E,
                        width,
                        p.axis_height_mm,
                        self.yoke_top(),
                    ));
        }
        for x in [-p.anchor_pitch_x_mm / 2., p.anchor_pitch_x_mm / 2.] {
            for y in [-p.anchor_pitch_y_mm / 2., p.anchor_pitch_y_mm / 2.] {
                a = a - cylinder(
                    "yoke_anchor",
                    p.anchor_hole_mm / 2.,
                    p.yoke_base_thickness_mm + 2. * E,
                    -E,
                )
                .translate(x, y, 0.);
            }
        }
        for sign in [-1., 1.] {
            for y in [-p.web_center_y_mm, p.web_center_y_mm] {
                a = a + block(
                    "outside_flange_side_brace",
                    p.web_length_mm + E,
                    p.web_width_mm,
                    p.web_height_mm + E,
                    p.yoke_base_thickness_mm - E,
                )
                .translate(
                    sign * (self.yoke_gap() / 2.
                        + p.yoke_plate_thickness_mm
                        + (p.web_length_mm - E) / 2.),
                    y,
                    0.,
                );
            }
        }
        let z = p.strap_bottom_mm + p.strap_height_mm;
        let edge = p.strap_width_mm / 2_f64.sqrt();
        let roof_h = p.strap_width_mm / 2. * 3_f64.sqrt();
        let roof = block(
            "strap_roof_diamond",
            edge,
            p.yoke_base_width_mm + 2. * E,
            edge,
            -edge / 2.,
        )
        .rotate(0., 45., 0.)
        .scale(1., 1., 3_f64.sqrt())
        .translate(0., 0., z)
            & block(
                "upper_half_only",
                p.strap_width_mm + 2. * E,
                p.yoke_base_width_mm + 4. * E,
                roof_h + E,
                z,
            );
        a - (block(
            "transverse_lift_restraint_strap_route",
            p.strap_width_mm,
            p.yoke_base_width_mm + 2. * E,
            p.strap_height_mm + E,
            p.strap_bottom_mm,
        ) + roof)
    }

    fn flange(&self) -> Part {
        let p = self.p();
        block(
            "REF_unmeasured_flange",
            p.flange_thickness_mm,
            p.flange_width_mm,
            p.flange_height_mm,
            p.axis_height_mm - p.flange_height_mm / 2.,
        )
    }
    fn view(&self, end: bool) -> Part {
        let pos = self.guide.restraint_positions();
        let x = pos[0] + if end { pos[1] } else { 0. };
        let p = self.p();
        let guide = if end {
            self.guide.end_assembly()
        } else {
            self.guide.start_assembly()
        };
        guide
            + (self.moving() + self.thumb()).translate(x, 0., pos[2])
            + (self.yoke() + self.flange()).translate(p.yoke_center_x_mm, 0., 0.)
    }
    fn exports(&self) -> Vec<Export> {
        let p = self.p();
        let c = self.c();
        let gb = self.guide.assembly_bounds();
        let bounds = (
            [gb.0[0], gb.0[1].min(-p.yoke_base_width_mm / 2.), 0.],
            [
                p.yoke_center_x_mm + p.yoke_base_length_mm / 2.,
                gb.1[1].max(p.yoke_base_width_mm / 2.),
                self.yoke_top()
                    .max(self.top() + self.guide.restraint_positions()[2]),
            ],
        );
        vec![
            Export {
                name: "restraint_carriage_adapter_DRY_FIT_REVIEW.stl",
                part: self.adapter().rotate(0., 90., 0.).translate(0., 0., -c[2]),
                bounds: (
                    [0., -c[0] / 2., 0.],
                    [self.top(), c[0] / 2., p.adapter_thickness_mm],
                ),
                role: "flat_rear_cap_adapter_existing_bolt_rod_and_screw_paths",
            },
            Export {
                name: "restraint_plunger_keeper_DRY_FIT_REVIEW.stl",
                part: self.keeper().rotate(0., 90., 0.).translate(
                    -self.keeper_bottom(),
                    0.,
                    self.keeper_x() + p.keeper_thickness_mm,
                ),
                bounds: (
                    [0., -p.keeper_width_mm / 2., 0.],
                    [
                        self.top() - self.keeper_bottom(),
                        p.keeper_width_mm / 2.,
                        p.keeper_thickness_mm,
                    ],
                ),
                role: "removable_open_top_stem_keeper_use_two_hard_spacers",
            },
            Export {
                name: "restraint_keeper_spacer_DRY_FIT_REVIEW.stl",
                part: self.spacer(),
                bounds: (
                    [
                        -p.spacer_outer_diameter_mm / 2.,
                        -p.spacer_outer_diameter_mm / 2.,
                        0.,
                    ],
                    [
                        p.spacer_outer_diameter_mm / 2.,
                        p.spacer_outer_diameter_mm / 2.,
                        self.pocket(),
                    ],
                ),
                role: "quantity_two_identical_axial_keeper_spacers_no_preload_qualification",
            },
            Export {
                name: "restraint_fixed_flange_yoke_DRY_FIT_REVIEW.stl",
                part: self.yoke(),
                bounds: (
                    [-p.yoke_base_length_mm / 2., -p.yoke_base_width_mm / 2., 0.],
                    [
                        p.yoke_base_length_mm / 2.,
                        p.yoke_base_width_mm / 2.,
                        self.yoke_top(),
                    ],
                ),
                role: "independent_board_anchors_strap_required_for_vertical_retention",
            },
            Export {
                name: "restraint_thumb_REF_NOT_FOR_MANUFACTURE.stl",
                part: self.thumb(),
                bounds: (
                    [
                        -c[2] + p.axial_clearance_mm / 2.,
                        -p.thumb_width_mm / 2.,
                        self.axis_local() - p.thumb_height_mm / 2.,
                    ],
                    [
                        -c[2] + p.axial_clearance_mm / 2. + p.thumb_thickness_mm,
                        p.thumb_width_mm / 2.,
                        self.axis_local() + p.thumb_height_mm / 2.,
                    ],
                ),
                role: "generic_unmeasured_thumb_envelope",
            },
            Export {
                name: "restraint_flange_REF_NOT_FOR_MANUFACTURE.stl",
                part: self.flange(),
                bounds: (
                    [
                        -p.flange_thickness_mm / 2.,
                        -p.flange_width_mm / 2.,
                        p.axis_height_mm - p.flange_height_mm / 2.,
                    ],
                    [
                        p.flange_thickness_mm / 2.,
                        p.flange_width_mm / 2.,
                        p.axis_height_mm + p.flange_height_mm / 2.,
                    ],
                ),
                role: "generic_unmeasured_flange_envelope",
            },
            Export {
                name: "restraint_start_DRY_VIEW_ONLY.stl",
                part: self.view(false),
                bounds,
                role: "static_start_pose_no_complete_syringe_or_barrel_support",
            },
            Export {
                name: "restraint_end_DRY_VIEW_ONLY.stl",
                part: self.view(true),
                bounds,
                role: "static_end_pose_not_observed_motion_or_selected_syringe_stroke",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let p = self.p();
        let c = self.c();
        vec![
            Probe {
                index: 0,
                name: "adapter_rod_passage",
                point: [c[1] / 2., c[6] / 2., p.adapter_thickness_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "adapter_cap_path",
                point: [
                    c[1] / 2. + c[4] / 2.,
                    c[3] / 2.,
                    p.adapter_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "adapter_upper_force_panel",
                point: [self.axis_local(), 0., p.adapter_thickness_mm / 2.],
                solid: true,
            },
            Probe {
                index: 1,
                name: "keeper_stem_slot",
                point: [
                    self.axis_local() - self.keeper_bottom(),
                    0.,
                    p.keeper_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 1,
                name: "keeper_lower_retention_land",
                point: [1., 0., p.keeper_thickness_mm / 2.],
                solid: true,
            },
            Probe {
                index: 2,
                name: "spacer_bolt_passage",
                point: [0., 0., self.pocket() / 2.],
                solid: false,
            },
            Probe {
                index: 2,
                name: "spacer_wall",
                point: [
                    p.spacer_outer_diameter_mm / 2. - 0.5,
                    0.,
                    self.pocket() / 2.,
                ],
                solid: true,
            },
            Probe {
                index: 3,
                name: "fixed_yoke_flange_gap",
                point: [0., 0., p.axis_height_mm - p.flange_height_mm / 2. + 1.],
                solid: false,
            },
            Probe {
                index: 3,
                name: "barrel_slot",
                point: [
                    self.yoke_gap() / 2. + p.yoke_plate_thickness_mm / 2.,
                    0.,
                    p.axis_height_mm,
                ],
                solid: false,
            },
            Probe {
                index: 3,
                name: "strap_passage",
                point: [0., 0., p.strap_bottom_mm + p.strap_height_mm / 2.],
                solid: false,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let r: Restraints = toml::from_str(raw)?;
    r.validate()?;
    let p = r.p();
    let c = r.c();
    let pos = r.guide.restraint_positions();
    let design = json!({"scope":"unpowered water-only provisional syringe interfaces","measured_hardware_selected":false,"axis_height_mm":p.axis_height_mm,"existing_carriage_contract":c,"carriage_positions_mm":pos,"adapter_rear_at_start_x_mm":pos[0]+r.rear_x(),"thumb_pocket_length_mm":r.pocket(),"flange_gap_mm":r.yoke_gap(),"nominal_axial_play_mm":p.axial_clearance_mm,"strap_roof_slope_deg":60.,"strap_roof_peak_z_mm":p.strap_bottom_mm+p.strap_height_mm+p.strap_width_mm/2.*3_f64.sqrt(),"strap_roof_minimum_top_land_mm":p.yoke_base_thickness_mm-p.strap_bottom_mm-p.strap_height_mm-p.strap_width_mm/2.*3_f64.sqrt(),"spacer_quantity":2,"moving_adapter_to_assembly_rotation_xyz_deg":[0.,-90.,0.],"moving_adapter_to_assembly_translation_mm":[pos[0]-c[2],0.,pos[2]],"keeper_to_assembly_rotation_xyz_deg":[0.,-90.,0.],"keeper_to_assembly_translation_mm":[pos[0]+r.keeper_x()+p.keeper_thickness_mm,0.,r.keeper_bottom()+pos[2]],"fixed_yoke_to_assembly_translation_mm":[p.yoke_center_x_mm,0.,0.],"thumb_to_flange_face_gap_start_mm":p.yoke_center_x_mm-p.flange_thickness_mm/2.-(pos[0]-c[2]+p.axial_clearance_mm/2.+p.thumb_thickness_mm),"rules":["All syringe dimensions are provisional examples, not measured or supplier-selected hardware.","Adapter shares nested carriage cap bolt and rod/screw passages; longer cap bolts and actual fastener stack/access are unselected.","Start position accounts for adapter thickness. Static endpoints do not qualify travel, bearing alignment or syringe stroke.","Keeper uses two identical hard spacers to limit clamping of the generic thumb; actual fit/endplay/strength remain unqualified.","Fixed yoke captures flange axially with open-top stem/barrel slots. A separately selected and verified strap is required to prevent lift; passage alone is not restraint.","Yoke has four independent M4 board anchor paths. Board material, underside access, flatness, stiffness and actual bolts are unqualified.","Existing low source saddles do not reach this axis. Barrel supports/risers at this height remain to be designed; the yoke must not carry an unsupported barrel alone.","No complete syringe, source-length fit, stem articulation, water tubing, motor, lead screw, thrust stack, guard, limits or control is included in these views.","No physical fit, force, pressure or flow qualification. Do not energize or fabricate an operating pump from these review meshes."]});
    publish(
        args,
        "water_bench_restraints",
        raw,
        &r,
        r.exports(),
        r.probes(),
        design,
    )
}
