//! Separate dry radial-bearing stand. No axial thrust restraint is established.
use super::motor::Motor;
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Support {
    support: Parameters,
    drive: Motor,
    retention: Option<Retention>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Retention {
    collar_outer_diameter_mm: f64,
    collar_length_mm: f64,
    collar_rotating_envelope_diameter_mm: f64,
    minimum_collar_plate_axial_gap_mm: f64,
    thrust_outer_diameter_mm: f64,
    thrust_length_mm: f64,
    bore_mm: f64,
    minimum_radial_clearance_mm: f64,
    minimum_end_gap_mm: f64,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    base_center_x_mm: f64,
    base_length_mm: f64,
    base_width_mm: f64,
    base_thickness_mm: f64,
    plate_thickness_mm: f64,
    plate_width_mm: f64,
    plate_top_mm: f64,
    pillow_rear_x_mm: f64,
    pillow_length_mm: f64,
    pillow_width_mm: f64,
    pillow_height_mm: f64,
    pillow_bore_mm: f64,
    pillow_mount_pitch_mm: f64,
    pillow_mount_hole_mm: f64,
    stand_mount_hole_mm: f64,
    pilot_clearance_mm: f64,
    axis_height_mm: f64,
    anchor_pitch_x_mm: f64,
    anchor_pitch_y_mm: f64,
    anchor_hole_mm: f64,
    web_length_mm: f64,
    web_width_mm: f64,
    web_height_mm: f64,
    web_center_y_mm: f64,
    minimum_wall_mm: f64,
    minimum_base_gap_mm: f64,
}
impl Support {
    pub(super) fn guide_config(&self) -> &super::frame::Frame {
        self.drive.guide_config()
    }
    pub(super) fn has_axial_candidate(&self) -> bool {
        self.retention.is_some()
    }
    pub(super) fn board_anchors(&self) -> Vec<[f64; 3]> {
        let p = &self.support;
        let mut v = self.drive.board_anchors();
        for x in [-1., 1.] {
            for y in [-1., 1.] {
                v.push([
                    p.base_center_x_mm + x * p.anchor_pitch_x_mm / 2.,
                    y * p.anchor_pitch_y_mm / 2.,
                    p.anchor_hole_mm,
                ]);
            }
        }
        v
    }
    pub(super) fn start_layout(&self) -> Part {
        let mut a = self.drive.layout() + self.stand() + self.housing_reference();
        if let Some(t) = &self.retention {
            a = a + self.retention_reference(t);
        }
        a
    }
    pub(super) fn layout_bounds(&self) -> Bounds {
        let g = self.drive.layout_bounds();
        let (a, b) = self.base_bounds();
        (
            [g.0[0].min(a[0]), g.0[1].min(a[1]), 0.],
            [
                g.1[0].max(b[0]),
                g.1[1].max(b[1]),
                g.1[2].max(self.support.plate_top_mm),
            ],
        )
    }

    fn base_bounds(&self) -> Bounds {
        let p = &self.support;
        (
            [
                p.base_center_x_mm - p.base_length_mm / 2.,
                -p.base_width_mm / 2.,
                0.,
            ],
            [
                p.base_center_x_mm + p.base_length_mm / 2.,
                p.base_width_mm / 2.,
                p.base_thickness_mm,
            ],
        )
    }
    fn plate_x(&self) -> f64 {
        self.support.pillow_rear_x_mm - self.support.plate_thickness_mm
    }
    pub(super) fn validate(&self) -> Result<(), String> {
        finite(&self.support)?;
        self.drive.validate()?;
        let p = &self.support;
        require(
            (28. ..=48.).contains(&p.base_length_mm)
                && (80. ..=110.).contains(&p.base_width_mm)
                && (4. ..=8.).contains(&p.base_thickness_mm)
                && (4. ..=10.).contains(&p.plate_thickness_mm)
                && (56. ..=76.).contains(&p.plate_width_mm)
                && (42. ..=56.).contains(&p.plate_top_mm)
                && (3. ..=5.).contains(&p.minimum_wall_mm)
                && p.minimum_base_gap_mm >= 2.,
            "invalid radial stand envelope",
        )?;
        require(
            (self.drive.axis_height() - p.axis_height_mm).abs() < 1e-8
                && (self.drive.screw_diameter() - p.pillow_bore_mm).abs() < 1e-8
                && (6. ..=9.).contains(&p.pillow_length_mm)
                && (40. ..=46.).contains(&p.pillow_width_mm)
                && (18. ..=24.).contains(&p.pillow_height_mm)
                && (30. ..=34.).contains(&p.pillow_mount_pitch_mm)
                && (4. ..=4.2).contains(&p.pillow_mount_hole_mm)
                && (4.3..=5.).contains(&p.stand_mount_hole_mm)
                && (18. ..=22.).contains(&p.pilot_clearance_mm),
            "invalid referenced housing or drive axis",
        )?;
        if let Some(t) = &self.retention {
            finite(t)?;
            require(
                (t.bore_mm - p.pillow_bore_mm).abs() < 1e-8
                    && (20. ..=22.).contains(&t.collar_outer_diameter_mm)
                    && (8. ..=10.).contains(&t.collar_length_mm)
                    && (15. ..=16.).contains(&t.thrust_outer_diameter_mm)
                    && (4.8..=5.2).contains(&t.thrust_length_mm)
                    && (24.5..=26.).contains(&t.collar_rotating_envelope_diameter_mm)
                    && t.minimum_collar_plate_axial_gap_mm >= 0.5
                    && t.minimum_radial_clearance_mm >= 0.2
                    && t.minimum_end_gap_mm >= 3.,
                "invalid dry axial hardware references",
            )?;
            let rear = p.pillow_rear_x_mm - t.thrust_length_mm - t.collar_length_mm;
            let front =
                p.pillow_rear_x_mm + p.pillow_length_mm + t.thrust_length_mm + t.collar_length_mm;
            require(
                (p.pilot_clearance_mm - t.thrust_outer_diameter_mm) / 2. + 1e-8
                    >= t.minimum_radial_clearance_mm
                    && p.pillow_rear_x_mm-t.thrust_length_mm<=self.plate_x()-t.minimum_collar_plate_axial_gap_mm
                    && p.axis_height_mm-t.collar_rotating_envelope_diameter_mm/2.>=p.base_thickness_mm+p.minimum_wall_mm
                    && rear - self.drive.coupler_end_x() >= t.minimum_end_gap_mm
                    && self.drive.guide_rear_support_x() - front >= t.minimum_end_gap_mm,
                "rotating collar overlaps upright/base or thrust hits opening, coupler or guide support; dry axial layout rejected",
            )?;
        }
        let r = p.pilot_clearance_mm / 2.;
        let h = p.stand_mount_hole_mm / 2.;
        let w = p.minimum_wall_mm;
        require(
            p.pillow_mount_pitch_mm / 2. - r - h >= w
                && p.axis_height_mm - r - p.base_thickness_mm >= w
                && p.plate_top_mm - p.axis_height_mm - r >= w
                && p.plate_width_mm / 2. - p.pillow_mount_pitch_mm / 2. - h >= w
                && p.axis_height_mm - p.pillow_height_mm / 2. >= p.base_thickness_mm + w,
            "bearing mount conflicts with base, pilot or edge lands",
        )?;
        let (a, b) = self.base_bounds();
        let [motor_front, guide_rear] = self.drive.base_gap_edges();
        require(
            a[0] - motor_front >= p.minimum_base_gap_mm
                && guide_rear - b[0] >= p.minimum_base_gap_mm
                && self.plate_x() - a[0] >= w
                && b[0] - p.pillow_rear_x_mm - p.web_length_mm >= w
                && p.pillow_rear_x_mm + p.pillow_length_mm < guide_rear,
            "stand overlaps another base or housing crosses guide-base plane",
        )?;
        let ar = p.anchor_hole_mm / 2.;
        require(
            (4.3..=5.).contains(&p.anchor_hole_mm)
                && p.base_length_mm / 2. - p.anchor_pitch_x_mm / 2. - ar >= w
                && p.base_width_mm / 2. - p.anchor_pitch_y_mm / 2. - ar >= w
                && p.anchor_pitch_y_mm / 2. - p.plate_width_mm / 2. - 4. >= w,
            "bench anchors lack edge land or top access outside plate",
        )?;
        require(
            (8. ..=16.).contains(&p.web_length_mm)
                && (3. ..=6.).contains(&p.web_width_mm)
                && (10. ..=18.).contains(&p.web_height_mm)
                && p.web_center_y_mm - p.web_width_mm / 2. - p.pillow_width_mm / 2. >= w
                && p.web_center_y_mm + p.web_width_mm / 2. <= p.plate_width_mm / 2.,
            "stand webs conflict with conservative housing or miss plate",
        )
    }
    fn x_cylinder(&self, name: &str, r: f64, x: f64, length: f64, y: f64, z: f64) -> Part {
        centered_cylinder(name, r, length, 64)
            .rotate(0., 90., 0.)
            .translate(x + length / 2., y, z)
    }
    fn face(&self) -> Part {
        let p = &self.support;
        let mut part = block(
            "radial_stand_face",
            p.plate_thickness_mm,
            p.plate_width_mm,
            p.plate_top_mm - p.base_thickness_mm + E,
            p.base_thickness_mm - E,
        )
        .translate(p.plate_thickness_mm / 2., 0., 0.)
            - self.x_cylinder(
                "central_bearing_clearance_not_thrust_shoulder",
                p.pilot_clearance_mm / 2.,
                -E,
                p.plate_thickness_mm + 2. * E,
                0.,
                p.axis_height_mm,
            );
        for y in [-p.pillow_mount_pitch_mm / 2., p.pillow_mount_pitch_mm / 2.] {
            part = part
                - self.x_cylinder(
                    "face_mount_M4_clearance",
                    p.stand_mount_hole_mm / 2.,
                    -E,
                    p.plate_thickness_mm + 2. * E,
                    y,
                    p.axis_height_mm,
                );
        }
        part
    }
    fn stand(&self) -> Part {
        let p = &self.support;
        let mut part = block(
            "separate_radial_stand_base",
            p.base_length_mm,
            p.base_width_mm,
            p.base_thickness_mm,
            0.,
        )
        .translate(p.base_center_x_mm, 0., 0.)
            + self.face().translate(self.plate_x(), 0., 0.);
        for y in [-p.web_center_y_mm, p.web_center_y_mm] {
            part = part
                + block(
                    "outside_housing_side_web",
                    p.web_length_mm + E,
                    p.web_width_mm,
                    p.web_height_mm + E,
                    p.base_thickness_mm - E,
                )
                .translate(p.pillow_rear_x_mm + (p.web_length_mm - E) / 2., y, 0.);
        }
        for x in [-1., 1.] {
            for y in [-1., 1.] {
                part = part
                    - cylinder(
                        "stand_bench_anchor_through",
                        p.anchor_hole_mm / 2.,
                        p.base_thickness_mm + 2. * E,
                        -E,
                    )
                    .translate(
                        p.base_center_x_mm + x * p.anchor_pitch_x_mm / 2.,
                        y * p.anchor_pitch_y_mm / 2.,
                        0.,
                    );
            }
        }
        part
    }
    fn gauge(&self) -> Part {
        let p = &self.support;
        // Trim the intentional face/base union overlap off the independent gauge.
        let trim = block(
            "gauge_without_join_overlap",
            p.plate_thickness_mm + 2. * E,
            p.plate_width_mm + 2. * E,
            p.plate_top_mm - p.base_thickness_mm,
            p.base_thickness_mm,
        )
        .translate(p.plate_thickness_mm / 2., 0., 0.);
        (self.face() & trim).rotate(0., 90., 0.).translate(
            -p.base_thickness_mm,
            0.,
            p.plate_thickness_mm,
        )
    }
    fn housing_reference(&self) -> Part {
        let p = &self.support;
        let mut part = block(
            "REF_conservative_housing_envelope",
            p.pillow_length_mm,
            p.pillow_width_mm,
            p.pillow_height_mm,
            p.axis_height_mm - p.pillow_height_mm / 2.,
        )
        .translate(p.pillow_rear_x_mm + p.pillow_length_mm / 2., 0., 0.)
            - self.x_cylinder(
                "REF_bearing_bore",
                p.pillow_bore_mm / 2.,
                p.pillow_rear_x_mm - E,
                p.pillow_length_mm + 2. * E,
                0.,
                p.axis_height_mm,
            );
        for y in [-p.pillow_mount_pitch_mm / 2., p.pillow_mount_pitch_mm / 2.] {
            part = part
                - self.x_cylinder(
                    "REF_nominal_face_mount_hole",
                    p.pillow_mount_hole_mm / 2.,
                    p.pillow_rear_x_mm - E,
                    p.pillow_length_mm + 2. * E,
                    y,
                    p.axis_height_mm,
                );
        }
        part
    }
    fn retention_bounds(&self, t: &Retention) -> Bounds {
        let p = &self.support;
        (
            [
                p.pillow_rear_x_mm - t.thrust_length_mm - t.collar_length_mm,
                -t.collar_rotating_envelope_diameter_mm / 2.,
                p.axis_height_mm - t.collar_rotating_envelope_diameter_mm / 2.,
            ],
            [
                p.pillow_rear_x_mm + p.pillow_length_mm + t.thrust_length_mm + t.collar_length_mm,
                t.collar_rotating_envelope_diameter_mm / 2.,
                p.axis_height_mm + t.collar_rotating_envelope_diameter_mm / 2.,
            ],
        )
    }
    fn retention_reference(&self, t: &Retention) -> Part {
        let p = &self.support;
        let annulus = |x: f64, length: f64, diameter: f64| {
            self.x_cylinder(
                "REF_smooth_axial_envelope",
                diameter / 2.,
                x,
                length,
                0.,
                p.axis_height_mm,
            ) - self.x_cylinder(
                "REF_unthreaded_bore",
                t.bore_mm / 2.,
                x - E,
                length + 2. * E,
                0.,
                p.axis_height_mm,
            )
        };
        let front = p.pillow_rear_x_mm + p.pillow_length_mm;
        annulus(
            p.pillow_rear_x_mm - t.thrust_length_mm - t.collar_length_mm,
            t.collar_length_mm,
            t.collar_rotating_envelope_diameter_mm,
        ) + annulus(
            p.pillow_rear_x_mm - t.thrust_length_mm,
            t.thrust_length_mm,
            t.thrust_outer_diameter_mm,
        ) + annulus(front, t.thrust_length_mm, t.thrust_outer_diameter_mm)
            + annulus(
                front + t.thrust_length_mm,
                t.collar_length_mm,
                t.collar_rotating_envelope_diameter_mm,
            )
    }
    fn exports(&self) -> Vec<Export> {
        let p = &self.support;
        let (a, b) = self.base_bounds();
        let g = self.drive.layout_bounds();
        let local = (
            [a[0] - p.base_center_x_mm, a[1], 0.],
            [b[0] - p.base_center_x_mm, b[1], p.plate_top_mm],
        );
        let mut exports = vec![
            Export {
                name: "screw_radial_stand_DRY_FIT_REVIEW.stl",
                part: self.stand().translate(-p.base_center_x_mm, 0., 0.),
                bounds: local,
                role: "separate_stand_global_assembly_translation_recorded_no_thrust_restraint",
            },
            Export {
                name: "screw_support_face_gauge_DRY_FIT_REVIEW.stl",
                part: self.gauge(),
                bounds: (
                    [0., -p.plate_width_mm / 2., 0.],
                    [
                        p.plate_top_mm - p.base_thickness_mm,
                        p.plate_width_mm / 2.,
                        p.plate_thickness_mm,
                    ],
                ),
                role: "flat_pattern_review_gauge_not_a_structural_mount",
            },
            Export {
                name: "screw_REF_housing_envelope_NOT_FOR_MANUFACTURE.stl",
                part: self.housing_reference(),
                bounds: (
                    [
                        p.pillow_rear_x_mm,
                        -p.pillow_width_mm / 2.,
                        p.axis_height_mm - p.pillow_height_mm / 2.,
                    ],
                    [
                        p.pillow_rear_x_mm + p.pillow_length_mm,
                        p.pillow_width_mm / 2.,
                        p.axis_height_mm + p.pillow_height_mm / 2.,
                    ],
                ),
                role: "conservative_bbox_with_bore_and_face_holes_not_exact_supplier_part",
            },
            Export {
                name: "screw_stand_assembly_DRY_VIEW_ONLY.stl",
                part: self.stand() + self.housing_reference(),
                bounds: (a, [b[0], b[1], p.plate_top_mm]),
                role: "nominal_bearing_mount_view_fasteners_and_thrust_components_omitted",
            },
            Export {
                name: "screw_supported_drive_DRY_VIEW_ONLY.stl",
                part: self.drive.layout() + self.stand() + self.housing_reference(),
                bounds: (
                    [g.0[0].min(a[0]), g.0[1].min(a[1]), 0.],
                    [
                        g.1[0].max(b[0]),
                        g.1[1].max(b[1]),
                        g.1[2].max(p.plate_top_mm),
                    ],
                ),
                role: "radial_support_layout_no_safe_powered_pump_or_axial_retention",
            },
        ];
        if let Some(t) = &self.retention {
            exports.push(Export {name:"screw_axial_REF_envelopes_NOT_FOR_MANUFACTURE.stl",part:self.retention_reference(t),bounds:self.retention_bounds(t),role:"two_supplier_thread_matching_collars_two_thrust_bearings_smooth_envelopes_only"});
            exports.push(Export {
                name: "screw_axial_drive_DRY_VIEW_ONLY.stl",
                part: self.drive.layout()
                    + self.stand()
                    + self.housing_reference()
                    + self.retention_reference(t),
                bounds: (
                    [g.0[0].min(a[0]), g.0[1].min(a[1]), 0.],
                    [
                        g.1[0].max(b[0]),
                        g.1[1].max(b[1]),
                        g.1[2].max(p.plate_top_mm),
                    ],
                ),
                role: "proposed_bidirectional_axial_stack_no_preload_or_load_qualification",
            });
        }
        exports
    }
    fn probes(&self) -> Vec<Probe> {
        let p = &self.support;
        let x = self.plate_x() + p.plate_thickness_mm / 2. - p.base_center_x_mm;
        vec![
            Probe {
                index: 0,
                name: "bearing_clearance_full_through",
                point: [x, 0., p.axis_height_mm],
                solid: false,
            },
            Probe {
                index: 0,
                name: "face_mount_path_full_through",
                point: [x, p.pillow_mount_pitch_mm / 2., p.axis_height_mm],
                solid: false,
            },
            Probe {
                index: 0,
                name: "plate_land_outside_mount_hole",
                point: [x, p.plate_width_mm / 2. - 2., p.plate_top_mm - 2.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "anchor_path_through_base",
                point: [
                    p.anchor_pitch_x_mm / 2.,
                    p.anchor_pitch_y_mm / 2.,
                    p.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "side_web_joined",
                point: [
                    p.pillow_rear_x_mm + 2. - p.base_center_x_mm,
                    p.web_center_y_mm,
                    p.base_thickness_mm + 2.,
                ],
                solid: true,
            },
            Probe {
                index: 1,
                name: "gauge_bearing_clearance",
                point: [
                    p.axis_height_mm - p.base_thickness_mm,
                    0.,
                    p.plate_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 1,
                name: "gauge_mount_clearance",
                point: [
                    p.axis_height_mm - p.base_thickness_mm,
                    p.pillow_mount_pitch_mm / 2.,
                    p.plate_thickness_mm / 2.,
                ],
                solid: false,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: Support = toml::from_str(raw)?;
    p.validate()?;
    let s = &p.support;
    let (a, b) = p.base_bounds();
    let edges = p.drive.base_gap_edges();
    let mut design = json!({"supplier_reference":"ServoCity 1606-0043-0008, unpurchased; actual supplier STEP inspected separately","axis":"X","axis_height_mm":s.axis_height_mm,"stand_mesh_to_assembly_translation_mm":[s.base_center_x_mm,0.,0.],"pillow_rear_face_x_mm":s.pillow_rear_x_mm,"face_mount_pitch_y_mm":s.pillow_mount_pitch_mm,"central_clearance_diameter_mm":s.pilot_clearance_mm,"neighboring_base_gaps_mm":[a[0]-edges[0],edges[1]-b[0]],"rear_space_to_coupler_end_mm":s.pillow_rear_x_mm-p.drive.coupler_end_x(),"rules":["Dry radial support engineering review only; no axial thrust restraint is established.","Housing reference is a conservative envelope, not exact supplier CAD, bearing internals or printable hardware.","The separate actual supplier STEP must be posed and checked against actual exported stand before claiming CAD mating clearance.","Two face mounting through paths need real bolts, washers and nuts; thread/seat/access and anchoring remain unqualified.","Three separate bases share a nominal bench plane but have no structural connection or established alignment datum.","Screw collars, thrust bearings, axial shoulders, preload and bidirectional thrust load path remain absent.","No syringe restraints, limits, hard stops, guard, driver or electrical stop is included. Do not energize.","Gauge is for later measured dry fit, not a structural mounting component or physical fit proof.","Pilot opening is horizontal in the stand review orientation; actual print support/material and strength are unqualified."]});
    if let Some(t) = &p.retention {
        let rear = s.pillow_rear_x_mm - t.thrust_length_mm - t.collar_length_mm;
        let front =
            s.pillow_rear_x_mm + s.pillow_length_mm + t.thrust_length_mm + t.collar_length_mm;
        design["axial_retention_candidate"] = json!({"collar_sku":"3504-0804-2109","collar_quantity":2,"thrust_bearing_sku":"1613-0516-0008","thrust_bearing_quantity":2,"supplier_arrangement_basis":"3212-0001-0001 manual page 4; this candidate substitutes second threaded collar for kit hyper hub","supplier_manual_url":"https://www.servocity.com/content/user_manuals/3212-0001-0001_assembly-instructions.min.pdf","thrust_catalog_od_mm":15.5,"thrust_step_od_mm":t.thrust_outer_diameter_mm,"thrust_step_length_mm":t.thrust_length_mm,"axial_stack_bounds_mm":p.retention_bounds(t),"collar_to_coupler_gap_mm":rear-p.drive.coupler_end_x(),"collar_to_guide_support_gap_mm":p.drive.guide_rear_support_x()-front,"nominal_collar_body_diameter_mm":t.collar_outer_diameter_mm,"collar_rotating_envelope_diameter_mm":t.collar_rotating_envelope_diameter_mm,"collar_to_plate_axial_gap_mm":p.plate_x()-(s.pillow_rear_x_mm-t.thrust_length_mm),"thrust_radial_stand_clearance_mm":(s.pilot_clearance_mm-t.thrust_outer_diameter_mm)/2.,"nominal_face_stack":"rear collar / thrust bearing / pillow block / thrust bearing / front collar","intended_reaction_path":"screw threads -> clamped collar -> thrust bearing -> stationary pillow-block face -> face bolts -> stand -> common board; direction selects opposite stack","qualification":"Topology proposal only. Actual supplier contacts, radial inner/outer race isolation, washer seats, endplay, preload, bolt access, collar slip, printed stand strength, and common board remain unqualified. Do not energize.","hardware_reference_mesh":"Smooth annular envelopes, no threads, rolling elements, split collar, pinch bolt, or exact supplier faces; use actual supplier STEP for collision/contact review."});
    }
    publish(
        args,
        "water_bench_screw_support",
        raw,
        &p,
        p.exports(),
        p.probes(),
        design,
    )
}
