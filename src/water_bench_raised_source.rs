//! Raised generic barrel supports and shared dry drive-board contract.
use super::restraints::Restraints;
use super::screw_support::Support;
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RaisedSource {
    raised: Parameters,
    pump: Support,
    fixture: Restraints,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    saddle_a_center_x_mm: f64,
    saddle_spacing_mm: f64,
    reference_barrel_length_mm: f64,
    base_length_mm: f64,
    base_width_mm: f64,
    base_thickness_mm: f64,
    plate_thickness_mm: f64,
    saddle_wall_mm: f64,
    web_length_mm: f64,
    web_width_mm: f64,
    web_height_mm: f64,
    anchor_pitch_x_mm: f64,
    anchor_pitch_y_mm: f64,
    anchor_hole_mm: f64,
    strap_width_mm: f64,
    strap_height_mm: f64,
    minimum_base_gap_mm: f64,
    minimum_barrel_end_land_mm: f64,
    board_min_x_mm: f64,
    board_max_x_mm: f64,
    board_width_mm: f64,
    reference_board_thickness_mm: f64,
    minimum_board_margin_mm: f64,
}
impl RaisedSource {
    fn p(&self) -> &Parameters {
        &self.raised
    }
    fn c(&self) -> [f64; 5] {
        self.fixture.source_contract()
    }
    fn radius(&self) -> f64 {
        (self.c()[1] + self.c()[2]) / 2.
    }
    fn top(&self) -> f64 {
        self.c()[0] + self.c()[2] / 2.
    }
    fn width(&self) -> f64 {
        2. * (self.radius() + self.p().saddle_wall_mm)
    }
    fn centers(&self) -> [f64; 2] {
        [
            self.p().saddle_a_center_x_mm,
            self.p().saddle_a_center_x_mm + self.p().saddle_spacing_mm,
        ]
    }
    fn strap_roof_height(&self) -> f64 {
        self.p().strap_width_mm / 2. * 3_f64.sqrt()
    }
    fn strap_bottom(&self) -> f64 {
        self.c()[0]
            - self.c()[1] / 2.
            - self.p().saddle_wall_mm
            - self.p().strap_height_mm
            - self.strap_roof_height()
    }
    fn gauge_bottom(&self) -> f64 {
        self.c()[0] - self.c()[1] / 2. - self.p().saddle_wall_mm + 0.2
    }
    fn anchors(&self) -> Vec<[f64; 3]> {
        let p = self.p();
        let mut a = self.pump.board_anchors();
        a.extend(self.fixture.board_anchors());
        for x in self.centers() {
            for sx in [-1., 1.] {
                for sy in [-1., 1.] {
                    a.push([
                        x + sx * p.anchor_pitch_x_mm / 2.,
                        sy * p.anchor_pitch_y_mm / 2.,
                        p.anchor_hole_mm,
                    ]);
                }
            }
        }
        a
    }
    fn validate(&self) -> Result<(), String> {
        let p = self.p();
        let c = self.c();
        finite(p)?;
        self.pump.validate()?;
        self.fixture.validate()?;
        require(
            serde_json::to_value(self.pump.guide_config()).map_err(|e| e.to_string())?
                == serde_json::to_value(self.fixture.guide_config()).map_err(|e| e.to_string())?,
            "pump and fixture guide configurations differ",
        )?;
        require(
            self.pump.has_axial_candidate(),
            "integrated layout requires explicit axial-retention candidate",
        )?;
        require(
            (28. ..=44.).contains(&p.base_length_mm)
                && (80. ..=110.).contains(&p.base_width_mm)
                && (6. ..=10.).contains(&p.base_thickness_mm)
                && (8. ..=14.).contains(&p.plate_thickness_mm)
                && (4. ..=6.).contains(&p.saddle_wall_mm)
                && (6. ..=10.).contains(&p.web_length_mm)
                && (3. ..=6.).contains(&p.web_width_mm)
                && (25. ..=45.).contains(&p.web_height_mm),
            "invalid tall saddle envelope",
        )?;
        require(
            (3. ..=4.).contains(&p.strap_width_mm)
                && (1.4..=2.).contains(&p.strap_height_mm)
                && (p.plate_thickness_mm - p.strap_width_mm) / 2. >= 3.
                && self.strap_bottom() >= p.base_thickness_mm + p.web_height_mm + 4.
                && p.base_length_mm / 2. - p.plate_thickness_mm / 2. - p.web_length_mm >= 3.,
            "strap route conflicts with braces, floor or side lands",
        )?;
        require(
            p.minimum_base_gap_mm >= 5.
                && p.minimum_barrel_end_land_mm >= 5.
                && self.centers()[0] - p.base_length_mm / 2. - c[4] >= p.minimum_base_gap_mm
                && p.saddle_spacing_mm - p.base_length_mm >= p.minimum_base_gap_mm
                && self.centers()[0] - p.plate_thickness_mm / 2. - c[3]
                    >= p.minimum_barrel_end_land_mm
                && c[3] + p.reference_barrel_length_mm
                    - self.centers()[1]
                    - p.plate_thickness_mm / 2.
                    >= p.minimum_barrel_end_land_mm
                && (80. ..=160.).contains(&p.reference_barrel_length_mm),
            "supports overlap bases or exceed generic reference barrel ends",
        )?;
        require(
            (4.3..=5.).contains(&p.anchor_hole_mm)
                && p.base_length_mm / 2. - p.anchor_pitch_x_mm / 2. - p.anchor_hole_mm / 2. >= 3.
                && p.base_width_mm / 2. - p.anchor_pitch_y_mm / 2. - p.anchor_hole_mm / 2. >= 3.
                && p.anchor_pitch_y_mm / 2. - 4. - self.width() / 2. >= 3.,
            "anchor edge lands or tool access conflict",
        )?;
        let b = self.pump.layout_bounds();
        require(
            (8. ..=20.).contains(&p.reference_board_thickness_mm)
                && p.minimum_board_margin_mm >= 10.
                && p.board_max_x_mm > p.board_min_x_mm
                && p.board_max_x_mm - p.board_min_x_mm <= 800.
                && p.board_min_x_mm + p.minimum_board_margin_mm <= b.0[0]
                && p.board_max_x_mm - p.minimum_board_margin_mm
                    >= self.centers()[1] + p.base_length_mm / 2.
                && p.board_width_mm / 2. - p.minimum_board_margin_mm
                    >= b.1[1].abs().max(b.0[1].abs()).max(p.base_width_mm / 2.),
            "board lacks footprint margin; reference thickness is not material qualification",
        )?;
        let anchors = self.anchors();
        for (i, a) in anchors.iter().enumerate() {
            require(
                a[0] - a[2] / 2. >= p.board_min_x_mm + 3.
                    && a[0] + a[2] / 2. <= p.board_max_x_mm - 3.
                    && a[1].abs() + a[2] / 2. <= p.board_width_mm / 2. - 3.,
                "anchor lacks board edge land",
            )?;
            for b in &anchors[..i] {
                require(
                    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
                        >= (a[2] + b[2]) / 2. + 3.,
                    "board anchor paths overlap or lack land",
                )?;
            }
        }
        Ok(())
    }
    fn strap_cut(&self) -> Part {
        let p = self.p();
        let length = self.width() + 2. * E;
        let z = self.strap_bottom() + p.strap_height_mm;
        let edge = p.strap_width_mm / 2_f64.sqrt();
        let roof = block("strap_roof_diamond", edge, length, edge, -edge / 2.)
            .rotate(0., 45., 0.)
            .scale(1., 1., 3_f64.sqrt())
            .translate(0., 0., z)
            & block(
                "upper_half_roof_only",
                p.strap_width_mm + 2. * E,
                length + 2. * E,
                self.strap_roof_height() + E,
                z,
            );
        block(
            "strap_rectangular_clearance",
            p.strap_width_mm,
            length,
            p.strap_height_mm + E,
            self.strap_bottom(),
        ) + roof
    }
    fn saddle(&self) -> Part {
        let p = self.p();
        let mut a = block(
            "raised_source_anchor_base",
            p.base_length_mm,
            p.base_width_mm,
            p.base_thickness_mm,
            0.,
        ) + block(
            "integrated_tall_saddle",
            p.plate_thickness_mm,
            self.width(),
            self.top() - p.base_thickness_mm + E,
            p.base_thickness_mm - E,
        );
        a = a - centered_cylinder(
            "open_half_round_barrel_seat",
            self.radius(),
            p.plate_thickness_mm + 2. * E,
            64,
        )
        .rotate(0., 90., 0.)
        .translate(0., 0., self.top());
        let y = self.width() / 2. - p.web_width_mm / 2. - 1.;
        for sx in [-1., 1.] {
            for sy in [-1., 1.] {
                a = a + block(
                    "tall_saddle_brace",
                    p.web_length_mm + E,
                    p.web_width_mm,
                    p.web_height_mm + E,
                    p.base_thickness_mm - E,
                )
                .translate(
                    sx * (p.plate_thickness_mm / 2. + (p.web_length_mm - E) / 2.),
                    sy * y,
                    0.,
                );
            }
        }
        for x in [-p.anchor_pitch_x_mm / 2., p.anchor_pitch_x_mm / 2.] {
            for y in [-p.anchor_pitch_y_mm / 2., p.anchor_pitch_y_mm / 2.] {
                a = a - cylinder(
                    "saddle_board_anchor",
                    p.anchor_hole_mm / 2.,
                    p.base_thickness_mm + 2. * E,
                    -E,
                )
                .translate(x, y, 0.);
            }
        }
        a - self.strap_cut()
    }
    fn gauge(&self) -> Part {
        let p = self.p();
        let bottom = self.gauge_bottom();
        let clip = block(
            "actual_saddle_upper_section",
            p.plate_thickness_mm,
            self.width(),
            self.top() - bottom,
            bottom,
        );
        (self.saddle() & clip)
            .rotate(0., 90., 0.)
            .translate(-bottom, 0., p.plate_thickness_mm / 2.)
    }
    fn pair(&self) -> Part {
        let c = self.centers();
        self.saddle().translate(c[0], 0., 0.) + self.saddle().translate(c[1], 0., 0.)
    }
    fn barrel(&self) -> Part {
        let c = self.c();
        let p = self.p();
        centered_cylinder(
            "REF_unselected_barrel",
            c[1] / 2.,
            p.reference_barrel_length_mm,
            64,
        )
        .rotate(0., 90., 0.)
        .translate(c[3] + p.reference_barrel_length_mm / 2., 0., c[0])
    }
    fn board(&self) -> Part {
        let p = self.p();
        let mut a = block(
            "REF_drive_board_unselected_material",
            p.board_max_x_mm - p.board_min_x_mm,
            p.board_width_mm,
            p.reference_board_thickness_mm,
            -p.reference_board_thickness_mm,
        )
        .translate((p.board_min_x_mm + p.board_max_x_mm) / 2., 0., 0.);
        for h in self.anchors() {
            a = a - cylinder(
                "REF_shared_board_anchor",
                h[2] / 2.,
                p.reference_board_thickness_mm + 2. * E,
                -p.reference_board_thickness_mm - E,
            )
            .translate(h[0], h[1], 0.);
        }
        a
    }
    fn exports(&self) -> Vec<Export> {
        let p = self.p();
        let c = self.c();
        let centers = self.centers();
        let z = self.top().max(self.fixture.assembly_top());
        vec![
   Export{name:"raised_source_saddle_DRY_FIT_REVIEW.stl",part:self.saddle(),bounds:([-p.base_length_mm/2.,-p.base_width_mm/2.,0.],[p.base_length_mm/2.,p.base_width_mm/2.,self.top()]),role:"quantity_two_identical_integrated_saddles_straps_and_board_bolts_unselected"},
   Export{name:"raised_source_seat_gauge_DRY_FIT_REVIEW.stl",part:self.gauge(),bounds:([0.,-self.width()/2.,0.],[self.top()-self.gauge_bottom(),self.width()/2.,p.plate_thickness_mm]),role:"actual_upper_saddle_section_flat_for_future_measured_fit"},
   Export{name:"raised_source_barrel_REF_NOT_FOR_MANUFACTURE.stl",part:self.barrel(),bounds:([c[3],-c[1]/2.,c[0]-c[1]/2.],[c[3]+p.reference_barrel_length_mm,c[1]/2.,c[0]+c[1]/2.]),role:"generic_unmeasured_barrel_only_no_plunger_outlet_or_flange"},
   Export{name:"raised_source_pair_DRY_VIEW_ONLY.stl",part:self.pair(),bounds:([centers[0]-p.base_length_mm/2.,-p.base_width_mm/2.,0.],[centers[1]+p.base_length_mm/2.,p.base_width_mm/2.,self.top()]),role:"two_posed_saddles_reference_barrel_exported_separately_to_preserve_part_identity"},
   Export{name:"raised_source_board_REF_NOT_FOR_MANUFACTURE.stl",part:self.board(),bounds:([p.board_min_x_mm,-p.board_width_mm/2.,-p.reference_board_thickness_mm],[p.board_max_x_mm,p.board_width_mm/2.,0.]),role:"reference_board_top_z_zero_not_material_thickness_drilling_or_fabrication_release"},
   Export{name:"raised_source_drive_DRY_VIEW_ONLY.stl",part:self.board()+self.pump.start_layout()+self.fixture.attached_start()+self.pair(),bounds:([p.board_min_x_mm,-p.board_width_mm/2.,-p.reference_board_thickness_mm],[p.board_max_x_mm,p.board_width_mm/2.,z]),role:"integrated_dry_start_pose_separate_generic_barrel_reference_no_power_or_motion_qualification"},
  ]
    }
    fn probes(&self) -> Vec<Probe> {
        let p = self.p();
        vec![
            Probe {
                index: 0,
                name: "open_barrel_seat",
                point: [0., 0., self.top() - 0.2],
                solid: false,
            },
            Probe {
                index: 0,
                name: "barrel_seat_floor_land",
                point: [0., 0., self.c()[0] - self.c()[1] / 2. - 1.],
                solid: true,
            },
            Probe {
                index: 0,
                name: "strap_rectangular_clearance",
                point: [0., 0., self.strap_bottom() + p.strap_height_mm / 2.],
                solid: false,
            },
            Probe {
                index: 0,
                name: "strap_roof_clearance",
                point: [
                    0.,
                    0.,
                    self.strap_bottom() + p.strap_height_mm + self.strap_roof_height() / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 0,
                name: "anchor_full_through",
                point: [
                    p.anchor_pitch_x_mm / 2.,
                    p.anchor_pitch_y_mm / 2.,
                    p.base_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 1,
                name: "gauge_barrel_seat",
                point: [
                    self.top() - self.gauge_bottom() - 0.2,
                    0.,
                    p.plate_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 4,
                name: "board_first_anchor_through",
                point: [
                    self.anchors()[0][0],
                    self.anchors()[0][1],
                    -p.reference_board_thickness_mm / 2.,
                ],
                solid: false,
            },
            Probe {
                index: 4,
                name: "board_corner_land",
                point: [
                    p.board_min_x_mm + 2.,
                    p.board_width_mm / 2. - 2.,
                    -p.reference_board_thickness_mm / 2.,
                ],
                solid: true,
            },
        ]
    }
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let r: RaisedSource = toml::from_str(raw)?;
    r.validate()?;
    let p = r.p();
    let c = r.c();
    let design = json!({"source_axis_height_mm":c[0],"saddle_cut_axis_height_mm":r.top(),"source_barrel_diameter_mm":c[1],"saddle_diametral_clearance_mm":c[2],"source_reference_barrel_x_mm":[c[3],c[3]+p.reference_barrel_length_mm],"saddle_quantity":2,"saddle_assembly_translations_mm":[[r.centers()[0],0.,0.],[r.centers()[1],0.,0.]],"seat_gauge_to_saddle_rotation_xyz_deg":[0.,-90.,0.],"seat_gauge_to_saddle_translation_mm":[p.plate_thickness_mm/2.,0.,r.gauge_bottom()],"saddle_base_gaps_mm":[r.centers()[0]-p.base_length_mm/2.-c[4],p.saddle_spacing_mm-p.base_length_mm],"barrel_seat_to_strap_roof_land_mm":p.saddle_wall_mm,"board_bounds_xy_mm":[[p.board_min_x_mm,-p.board_width_mm/2.],[p.board_max_x_mm,p.board_width_mm/2.]],"reference_board_thickness_mm":p.reference_board_thickness_mm,"board_anchors_x_y_diameter_mm":r.anchors(),"nested_guide_configurations_identical":true,"rules":["Unpowered water-only engineering candidate. All syringe dimensions and length remain provisional, not measured hardware.","Raised saddles are single pieces, quantity two; existing low saddles are not stacked on them.","Cut axis is source axis plus half the diametral clearance so the circular reference barrel rests nominally at the shared source axis. Actual strap force/deformation and print fit remain unqualified.","Strap passages include a rectangular throat plus 60-degree roof. No strap is selected or verified, so lift and barrel retention remain unqualified.","Common board top is Z0; its 12mm example thickness is an envelope only. Material, actual thickness, rigidity, flatness, bolts, underside access and drilling template remain unselected.","Twenty-four anchors derive from nested guide, motor, radial stand, yoke and raised-saddle parameters. Configuration equality prevents a mismatched carriage start/guide snapshot.","The 560x150 drive-board example supersedes the old 400x150 proposal for this candidate only; receiver, dock, controller and outlet remain separate bench modules.","Integrated view retains provisional references for motor/screw/thrust stack and thumb/flange. Generic barrel is exported separately to preserve tangent-contact part identity; no complete syringe stem/outlet is modeled.","Earlier high flange-yoke preflight warning remains unresolved. Whole-board/drive views are not print jobs; individual parts require toolpath/material and physical dry-fit review.","No force, pressure, flow, load capacity, alignment or continuous-motion qualification; limits, hard stops, guarding and electrical control remain absent. Do not energize."]});
    publish(
        args,
        "water_bench_raised_source",
        raw,
        &r,
        r.exports(),
        r.probes(),
        design,
    )
}
