//! Dry motor interface and reference drivetrain layout, with no powered release.
use super::frame::Frame;
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Motor {
    motor: Parameters,
    guide: Frame,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    body_square_mm: f64,
    body_length_mm: f64,
    mounting_pitch_mm: f64,
    boss_diameter_mm: f64,
    shaft_diameter_mm: f64,
    shaft_length_mm: f64,
    shaft_length_tolerance_mm: f64,
    axis_height_mm: f64,
    plate_thickness_mm: f64,
    plate_width_mm: f64,
    plate_top_mm: f64,
    boss_diametral_clearance_mm: f64,
    mount_hole_diameter_mm: f64,
    base_back_mm: f64,
    base_front_mm: f64,
    base_width_mm: f64,
    base_thickness_mm: f64,
    anchor_back_x_mm: f64,
    anchor_front_x_mm: f64,
    anchor_pitch_y_mm: f64,
    anchor_hole_diameter_mm: f64,
    side_web_length_mm: f64,
    side_web_width_mm: f64,
    side_web_height_mm: f64,
    side_web_center_y_mm: f64,
    coupler_diameter_mm: f64,
    coupler_length_mm: f64,
    coupler_start_x_mm: f64,
    reference_screw_diameter_mm: f64,
    reference_screw_start_x_mm: f64,
    reference_screw_length_mm: f64,
    motor_face_in_guide_x_mm: f64,
    minimum_wall_mm: f64,
}
impl Motor {
    fn validate(&self) -> Result<(), String> {
        finite(&self.motor)?;
        self.guide.validate()?;
        let p = &self.motor;
        require(
            (30. ..=45.).contains(&p.body_square_mm)
                && (25. ..=50.).contains(&p.body_length_mm)
                && (25. ..=33.).contains(&p.mounting_pitch_mm)
                && (18. ..=24.).contains(&p.boss_diameter_mm)
                && (4. ..=6.).contains(&p.shaft_diameter_mm)
                && (18. ..=30.).contains(&p.shaft_length_mm)
                && (0. ..=2.).contains(&p.shaft_length_tolerance_mm),
            "invalid motor reference envelope",
        )?;
        require(
            (4. ..=8.).contains(&p.plate_thickness_mm)
                && (52. ..=80.).contains(&p.plate_width_mm)
                && (50. ..=80.).contains(&p.plate_top_mm)
                && (4. ..=10.).contains(&p.base_thickness_mm)
                && (42. ..=70.).contains(&p.base_back_mm)
                && (32. ..=60.).contains(&p.base_front_mm)
                && (64. ..=100.).contains(&p.base_width_mm)
                && (3. ..=5.).contains(&p.minimum_wall_mm),
            "invalid bracket envelope",
        )?;
        let r = (p.boss_diameter_mm + p.boss_diametral_clearance_mm) / 2.;
        let h = p.mount_hole_diameter_mm / 2.;
        let a = p.mounting_pitch_mm / 2.;
        require(
            (0.3..=0.8).contains(&p.boss_diametral_clearance_mm)
                && (3.2..=4.).contains(&p.mount_hole_diameter_mm)
                && p.axis_height_mm - p.body_square_mm / 2. - p.base_thickness_mm >= 0.5
                && p.axis_height_mm - r - p.base_thickness_mm >= p.minimum_wall_mm
                && p.plate_top_mm - p.axis_height_mm - r >= p.minimum_wall_mm
                && p.plate_width_mm / 2. - a - h >= p.minimum_wall_mm
                && p.axis_height_mm - a - h - p.base_thickness_mm >= p.minimum_wall_mm
                && p.plate_top_mm - p.axis_height_mm - a - h >= p.minimum_wall_mm
                && (2. * a * a).sqrt() - r - h >= p.minimum_wall_mm,
            "motor holes or reference body conflict with bracket",
        )?;
        require(
            (4.3..=5.).contains(&p.anchor_hole_diameter_mm)
                && p.anchor_back_x_mm < -p.body_length_mm - 5.
                && p.anchor_front_x_mm > p.plate_thickness_mm + p.side_web_length_mm + 5.
                && p.base_back_mm + p.anchor_back_x_mm - p.anchor_hole_diameter_mm / 2.
                    >= p.minimum_wall_mm
                && p.base_front_mm - p.anchor_front_x_mm - p.anchor_hole_diameter_mm / 2.
                    >= p.minimum_wall_mm
                && p.base_width_mm / 2. - p.anchor_pitch_y_mm / 2. - p.anchor_hole_diameter_mm / 2.
                    >= p.minimum_wall_mm
                && p.anchor_pitch_y_mm / 2. - p.body_square_mm / 2. >= 6.,
            "anchor holes conflict with motor envelope or base edges",
        )?;
        require(
            (12. ..=26.).contains(&p.side_web_length_mm)
                && (3. ..=6.).contains(&p.side_web_width_mm)
                && (10. ..=22.).contains(&p.side_web_height_mm)
                && p.side_web_center_y_mm - p.side_web_width_mm / 2. >= p.body_square_mm / 2. + 5.
                && p.side_web_center_y_mm + p.side_web_width_mm / 2. <= p.plate_width_mm / 2.
                && p.side_web_center_y_mm + p.side_web_width_mm / 2. <= p.base_width_mm / 2.,
            "side webs obstruct motor or miss plate/base",
        )?;
        require(
            (16. ..=24.).contains(&p.coupler_diameter_mm)
                && (20. ..=32.).contains(&p.coupler_length_mm)
                && p.coupler_start_x_mm - p.plate_thickness_mm >= 5.
                && p.shaft_length_mm - p.shaft_length_tolerance_mm - p.coupler_start_x_mm >= 6.
                && p.reference_screw_start_x_mm - (p.shaft_length_mm + p.shaft_length_tolerance_mm)
                    >= 3.
                && p.coupler_start_x_mm + p.coupler_length_mm - p.reference_screw_start_x_mm >= 6.
                && p.reference_screw_start_x_mm >= p.coupler_start_x_mm + p.coupler_length_mm / 2.
                && (6. ..=10.).contains(&p.reference_screw_diameter_mm)
                && (180. ..=350.).contains(&p.reference_screw_length_mm),
            "invalid nominal shaft/coupler layout",
        )?;
        require(
            (self.guide.axis_height() - p.axis_height_mm).abs() < 1e-8
                && (self.guide.screw_diameter() - p.reference_screw_diameter_mm).abs() < 1e-8
                && self.guide.base_bounds().0[0] - (p.motor_face_in_guide_x_mm + p.base_front_mm)
                    >= 5.
                && p.motor_face_in_guide_x_mm + p.reference_screw_start_x_mm
                    < self.guide.base_bounds().0[0]
                && p.motor_face_in_guide_x_mm
                    + p.reference_screw_start_x_mm
                    + p.reference_screw_length_mm
                    > self.guide.required_nut_end_x(),
            "motor/guide axes, separated bases or reference screw reach do not match",
        )
    }
    fn x_cylinder(&self, name: &str, r: f64, x: f64, len: f64, y: f64, z: f64) -> Part {
        centered_cylinder(name, r, len, 64)
            .rotate(0., 90., 0.)
            .translate(x + len / 2., y, z)
    }
    fn bracket(&self) -> Part {
        let p = &self.motor;
        let mut part = block(
            "motor_bracket_base",
            p.base_back_mm + p.base_front_mm,
            p.base_width_mm,
            p.base_thickness_mm,
            0.,
        )
        .translate((p.base_front_mm - p.base_back_mm) / 2., 0., 0.)
            + block(
                "motor_face_plate",
                p.plate_thickness_mm,
                p.plate_width_mm,
                p.plate_top_mm - p.base_thickness_mm + E,
                p.base_thickness_mm - E,
            )
            .translate(p.plate_thickness_mm / 2., 0., 0.);
        for y in [-p.side_web_center_y_mm, p.side_web_center_y_mm] {
            part = part
                + block(
                    "side_web",
                    p.side_web_length_mm + E,
                    p.side_web_width_mm,
                    p.side_web_height_mm + E,
                    p.base_thickness_mm - E,
                )
                .translate(
                    p.plate_thickness_mm + (p.side_web_length_mm - E) / 2.,
                    y,
                    0.,
                );
        }
        part = part
            - self.x_cylinder(
                "full_boss_clearance",
                (p.boss_diameter_mm + p.boss_diametral_clearance_mm) / 2.,
                -E,
                p.plate_thickness_mm + 2. * E,
                0.,
                p.axis_height_mm,
            );
        for y in [-1., 1.] {
            for z in [-1., 1.] {
                part = part
                    - self.x_cylinder(
                        "M3_motor_mount_clearance",
                        p.mount_hole_diameter_mm / 2.,
                        -E,
                        p.plate_thickness_mm + 2. * E,
                        y * p.mounting_pitch_mm / 2.,
                        p.axis_height_mm + z * p.mounting_pitch_mm / 2.,
                    );
            }
        }
        for x in [p.anchor_back_x_mm, p.anchor_front_x_mm] {
            for y in [-p.anchor_pitch_y_mm / 2., p.anchor_pitch_y_mm / 2.] {
                part = part
                    - cylinder(
                        "bench_anchor_clearance",
                        p.anchor_hole_diameter_mm / 2.,
                        p.base_thickness_mm + 2. * E,
                        -E,
                    )
                    .translate(x, y, 0.);
            }
        }
        part
    }
    fn motor_reference(&self) -> Part {
        let p = &self.motor;
        block(
            "REF_conservative_motor_body",
            p.body_length_mm,
            p.body_square_mm,
            p.body_square_mm,
            p.axis_height_mm - p.body_square_mm / 2.,
        )
        .translate(-p.body_length_mm / 2., 0., 0.)
            + self.x_cylinder(
                "REF_round_shaft_without_flat",
                p.shaft_diameter_mm / 2.,
                -E,
                p.shaft_length_mm + E,
                0.,
                p.axis_height_mm,
            )
    }
    fn coupler_reference(&self) -> Part {
        let p = &self.motor;
        self.x_cylinder(
            "REF_coupler_outer_envelope_no_bores_or_setscrews",
            p.coupler_diameter_mm / 2.,
            p.coupler_start_x_mm,
            p.coupler_length_mm,
            0.,
            p.axis_height_mm,
        )
    }
    fn screw_reference(&self) -> Part {
        let p = &self.motor;
        self.x_cylinder(
            "REF_smooth_lead_screw_envelope_no_threads",
            p.reference_screw_diameter_mm / 2.,
            p.reference_screw_start_x_mm,
            p.reference_screw_length_mm,
            0.,
            p.axis_height_mm,
        )
    }
    fn exports(&self) -> Vec<Export> {
        let p = &self.motor;
        let bracket_bounds = (
            [-p.base_back_mm, -p.base_width_mm / 2., 0.],
            [p.base_front_mm, p.base_width_mm / 2., p.plate_top_mm],
        );
        let g = self.guide.assembly_bounds();
        let layout_bounds = (
            [
                g.0[0].min(p.motor_face_in_guide_x_mm - p.base_back_mm),
                g.0[1].min(-p.base_width_mm / 2.),
                0.,
            ],
            [
                g.1[0].max(
                    p.motor_face_in_guide_x_mm
                        + p.reference_screw_start_x_mm
                        + p.reference_screw_length_mm,
                ),
                g.1[1].max(p.base_width_mm / 2.),
                g.1[2].max(p.plate_top_mm),
            ],
        );
        vec![
            Export {
                name: "motor_bracket_DRY_FIT_REVIEW.stl",
                part: self.bracket(),
                bounds: bracket_bounds,
                role: "upright_review_orientation_slicer_support_and_strength_unqualified",
            },
            Export {
                name: "motor_REF_body_and_shaft_NOT_FOR_MANUFACTURE.stl",
                part: self.motor_reference(),
                bounds: (
                    [
                        -p.body_length_mm,
                        -p.body_square_mm / 2.,
                        p.axis_height_mm - p.body_square_mm / 2.,
                    ],
                    [
                        p.shaft_length_mm,
                        p.body_square_mm / 2.,
                        p.axis_height_mm + p.body_square_mm / 2.,
                    ],
                ),
                role: "conservative_body_envelope_shaft_flat_boss_wires_mount_threads_omitted",
            },
            Export {
                name: "motor_REF_coupler_outer_NOT_FOR_MANUFACTURE.stl",
                part: self.coupler_reference(),
                bounds: (
                    [
                        p.coupler_start_x_mm,
                        -p.coupler_diameter_mm / 2.,
                        p.axis_height_mm - p.coupler_diameter_mm / 2.,
                    ],
                    [
                        p.coupler_start_x_mm + p.coupler_length_mm,
                        p.coupler_diameter_mm / 2.,
                        p.axis_height_mm + p.coupler_diameter_mm / 2.,
                    ],
                ),
                role: "outer_envelope_only_internal_insertion_depth_not_verified",
            },
            Export {
                name: "motor_REF_smooth_screw_NOT_FOR_MANUFACTURE.stl",
                part: self.screw_reference(),
                bounds: (
                    [
                        p.reference_screw_start_x_mm,
                        -p.reference_screw_diameter_mm / 2.,
                        p.axis_height_mm - p.reference_screw_diameter_mm / 2.,
                    ],
                    [
                        p.reference_screw_start_x_mm + p.reference_screw_length_mm,
                        p.reference_screw_diameter_mm / 2.,
                        p.axis_height_mm + p.reference_screw_diameter_mm / 2.,
                    ],
                ),
                role: "smooth_reference_shaft_not_threaded_hardware_or_thrust_support",
            },
            Export {
                name: "motor_assembly_DRY_VIEW_ONLY.stl",
                part: self.bracket() + self.motor_reference() + self.coupler_reference(),
                bounds: bracket_bounds,
                role: "nominal_alignment_view_no_power_or_physical_fit",
            },
            Export {
                name: "motor_guide_layout_DRY_VIEW_ONLY.stl",
                part: self.guide.start_assembly()
                    + (self.bracket()
                        + self.motor_reference()
                        + self.coupler_reference()
                        + self.screw_reference())
                    .translate(p.motor_face_in_guide_x_mm, 0., 0.),
                bounds: layout_bounds,
                role:
                    "separate_bases_on_common_bench_plane_no_thrust_support_or_safe_powered_drive",
            },
        ]
    }
    fn probes(&self) -> Vec<Probe> {
        let p = &self.motor;
        let mid = p.plate_thickness_mm / 2.;
        vec![
            Probe {
                index: 0,
                name: "boss_clearance_is_full_through",
                point: [mid, 0., p.axis_height_mm],
                solid: false,
            },
            Probe {
                index: 0,
                name: "motor_mount_hole_through",
                point: [
                    mid,
                    p.mounting_pitch_mm / 2.,
                    p.axis_height_mm + p.mounting_pitch_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "face_plate_land",
                point: [mid, p.plate_width_mm / 2. - 2., p.plate_top_mm - 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "anchor_through_base",
                point: [
                    p.anchor_back_x_mm,
                    p.anchor_pitch_y_mm / 2.,
                    p.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "base_plate_continuous",
                point: [-p.base_back_mm + 2., 0., p.base_thickness_mm / 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "side_web_joined_to_base",
                point: [
                    p.plate_thickness_mm + 2.,
                    p.side_web_center_y_mm,
                    p.base_thickness_mm + 2.,
                ],
                solid: true,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: Motor = toml::from_str(raw)?;
    p.validate()?;
    let m = &p.motor;
    let design = json!({"motor_reference":"Adafruit 324 / linked XY42STH34-0354A drawing, unpurchased","coupler_reference":"Adafruit 1176, 5 to 8 mm, outer envelope only","axis":"X","axis_height_mm":m.axis_height_mm,"motor_face_in_guide_x_mm":m.motor_face_in_guide_x_mm,"motor_body_bottom_gap_mm":m.axis_height_mm-m.body_square_mm/2.-m.base_thickness_mm,"mount_hole_pitch_yz_mm":[m.mounting_pitch_mm,m.mounting_pitch_mm],"boss_clearance_diameter_mm":m.boss_diameter_mm+m.boss_diametral_clearance_mm,"nominal_shaft_end_gap_mm":m.reference_screw_start_x_mm-m.shaft_length_mm,"worst_reference_shaft_end_gap_mm":m.reference_screw_start_x_mm-m.shaft_length_mm-m.shaft_length_tolerance_mm,"nominal_motor_overlap_with_coupler_outer_mm":m.shaft_length_mm-m.coupler_start_x_mm,"nominal_screw_overlap_with_coupler_outer_mm":m.coupler_start_x_mm+m.coupler_length_mm-m.reference_screw_start_x_mm,"base_to_guide_base_gap_mm":p.guide.base_bounds().0[0]-m.motor_face_in_guide_x_mm-m.base_front_mm,"rules":["Dry mechanical engineering review only. Do not energize this assembly.","Motor body is a conservative square/length envelope; shaft flat, boss projection, mounting threads, wires and connectors are omitted.","Product page and linked drawing differ on winding resistance/holding torque; no force or drive-current design is inferred here.","Coupler is an outer envelope only. Bore transition, insertion depths, set-screw positions, thread-shaft grip and tightening access require actual hardware.","Smooth reference screw has no threads and is not a printable hardware part.","A flexible coupler is not an axial thrust bearing. Screw radial and bidirectional thrust support are still absent.","Separate bases have no structural connection; a common anchored datum, alignment and rigidity are not established.","No syringe restraint, limits, hard stops, guard, driver, power supply or electrical stop circuit is included.","Bracket is in upright engineering orientation; actual slicer supports, material, fasteners and load capacity remain unresolved."]});
    publish(
        args,
        "water_bench_motor",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
