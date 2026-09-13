//! Deterministic dry P0 laminated-cartridge coupon geometry.
//!
//! The dimensions in this module are proposed CAD envelopes for engineering
//! coupons. They are not validated tolerances, manufacturing release values,
//! assay conditions, or vendor-specific material selections.

use serde::{Deserialize, Serialize};
use vcad::{centered_cube, centered_cylinder, Part};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub coupon_length_mm: f64,
    pub coupon_width_mm: f64,
    pub key_notch_length_mm: f64,
    pub key_notch_width_mm: f64,
    pub registration_hole_diameter_mm: f64,
    pub registration_slot_width_mm: f64,
    pub registration_slot_length_mm: f64,
    pub registration_y_mm: f64,
    pub registration_left_x_mm: f64,
    pub registration_right_x_mm: f64,
    pub min_seal_land_mm: f64,
    pub channel_width_mm: f64,
    pub chamber_length_mm: f64,
    pub chamber_width_mm: f64,
    pub meter_nominal_ul: f64,
    pub feature_overlap_mm: f64,
    pub coc_cop_target: StackDimensions,
    pub pmma_control: StackDimensions,
    pub pet_comparator: StackDimensions,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StackDimensions {
    pub base_thickness_mm: f64,
    pub spacer_thickness_mm: f64,
    pub cover_thickness_mm: f64,
}

pub const SUITE_ID: &str = "LF-P0-CARTRIDGE-COUPONS";
pub const REVISION: &str = "P0-R0";
pub const TICKET_ID: &str = "T-A2021311";
pub const SOURCE_ARTIFACTS: [&str; 3] = ["A-A1A77D11", "A-696CE730", "A-CE59D39F"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialStackId {
    CocCopTarget,
    PmmaControl,
    PetComparator,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MaterialStack {
    pub id: MaterialStackId,
    pub slug: &'static str,
    pub role: &'static str,
    pub base_thickness_mm: f64,
    pub spacer_thickness_mm: f64,
    pub cover_thickness_mm: f64,
    pub conditional: bool,
}

impl MaterialStack {
    pub fn total_thickness_mm(self) -> f64 {
        self.base_thickness_mm + self.spacer_thickness_mm + self.cover_thickness_mm
    }

    pub fn chamber_volume_ul(self, p: &Parameters) -> f64 {
        p.chamber_length_mm * p.chamber_width_mm * self.spacer_thickness_mm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CouponFamily {
    MaterialContact,
    OpticalWindow,
    ThermalEvaporation,
    BondRegistration,
    MeteringDebris,
    VentWaste,
    SealBackflow,
    DualLaneIsolation,
    SwabDockRetention,
    ConditionalBlister,
    SealedContainment,
}

impl CouponFamily {
    pub const ALL: [CouponFamily; 11] = [
        CouponFamily::MaterialContact,
        CouponFamily::OpticalWindow,
        CouponFamily::ThermalEvaporation,
        CouponFamily::BondRegistration,
        CouponFamily::MeteringDebris,
        CouponFamily::VentWaste,
        CouponFamily::SealBackflow,
        CouponFamily::DualLaneIsolation,
        CouponFamily::SwabDockRetention,
        CouponFamily::ConditionalBlister,
        CouponFamily::SealedContainment,
    ];

    pub fn slug(self) -> &'static str {
        match self {
            CouponFamily::MaterialContact => "material_contact",
            CouponFamily::OpticalWindow => "optical_window",
            CouponFamily::ThermalEvaporation => "thermal_evaporation",
            CouponFamily::BondRegistration => "bond_registration",
            CouponFamily::MeteringDebris => "metering_debris",
            CouponFamily::VentWaste => "vent_waste",
            CouponFamily::SealBackflow => "seal_backflow",
            CouponFamily::DualLaneIsolation => "dual_lane_isolation",
            CouponFamily::SwabDockRetention => "swab_dock_retention",
            CouponFamily::ConditionalBlister => "conditional_blister",
            CouponFamily::SealedContainment => "sealed_containment",
        }
    }

    pub fn coupon_id(self) -> &'static str {
        match self {
            CouponFamily::MaterialContact => "M-02",
            CouponFamily::OpticalWindow => "M-01/O-01",
            CouponFamily::ThermalEvaporation => "T-01",
            CouponFamily::BondRegistration => "B-01/R-01",
            CouponFamily::MeteringDebris => "F-01/F-02",
            CouponFamily::VentWaste => "F-03",
            CouponFamily::SealBackflow => "F-04",
            CouponFamily::DualLaneIsolation => "F-05",
            CouponFamily::SwabDockRetention => "H-01",
            CouponFamily::ConditionalBlister => "H-02",
            CouponFamily::SealedContainment => "C-01",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|family| *family == self).unwrap() + 1
    }

    pub fn conditional(self) -> bool {
        matches!(self, CouponFamily::ConditionalBlister)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CouponDescriptor {
    pub family: CouponFamily,
    pub coupon_id: &'static str,
    pub purpose: &'static str,
    pub conditional: bool,
}

pub fn descriptors() -> Vec<CouponDescriptor> {
    CouponFamily::ALL
        .into_iter()
        .map(|family| CouponDescriptor {
            family,
            coupon_id: family.coupon_id(),
            purpose: family_purpose(family),
            conditional: family.conditional(),
        })
        .collect()
}

pub fn family_purpose(family: CouponFamily) -> &'static str {
    match family {
        CouponFamily::MaterialContact => "separate material/contact wells and bonded-stack exposure areas",
        CouponFamily::OpticalWindow => "paired optical ROIs, edge contribution, fiducials, and registration",
        CouponFamily::ThermalEvaporation => "paired 20-25 uL-class chambers, window span, fill and bubble witnesses",
        CouponFamily::BondRegistration => "straight, corner, junction, narrow-land, peel, and registration witnesses",
        CouponFamily::MeteringDebris => "0.5-5 uL-class meter envelope, settling pocket, weir, and barrier support",
        CouponFamily::VentWaste => "captive headspace, splash baffles, optional absorbent bay, and membrane support",
        CouponFamily::SealBackflow => "burst throat, seal-land variants, reverse-flow labyrinth, and pressure ports",
        CouponFamily::DualLaneIsolation => "physically isolated paired lanes, chambers, terminal waste, and vents",
        CouponFamily::SwabDockRetention => "dry dock guide, hard stop, shaft capture, latch, and drainage proxy",
        CouponFamily::ConditionalBlister => "conditional single-blister envelope, captive puncture, outlet, and actuator target",
        CouponFamily::SealedContainment => "representative closed perimeter, weak corners, closures, witness moat, and handling interfaces",
    }
}

impl Parameters {
    pub fn build_coupon(&self, family: CouponFamily, stack: MaterialStack) -> Part {
        self.validate().expect("invalid coupon parameters");
        self.assert_valid_stack(stack);

        let base = self.keyed_layer(
            format!("{}_{}_base", family.slug(), stack.slug),
            stack.base_thickness_mm,
            stack.base_thickness_mm / 2.0,
        ) - self.registration_cuts(stack.total_thickness_mm() + 2.0);

        let spacer_z = stack.base_thickness_mm + stack.spacer_thickness_mm / 2.0;
        let spacer = self.keyed_layer(
            format!("{}_{}_spacer", family.slug(), stack.slug),
            stack.spacer_thickness_mm,
            spacer_z,
        ) - self.registration_cuts(stack.total_thickness_mm() + 2.0)
            - self.family_fluidic_cuts(
                family,
                spacer_z,
                stack.spacer_thickness_mm + 0.08,
                stack.spacer_thickness_mm,
            );

        let cover_z =
            stack.base_thickness_mm + stack.spacer_thickness_mm + stack.cover_thickness_mm / 2.0;
        let cover = self.keyed_layer(
            format!("{}_{}_cover", family.slug(), stack.slug),
            stack.cover_thickness_mm,
            cover_z,
        ) - self.registration_cuts(stack.total_thickness_mm() + 2.0)
            - self.fiducial_cuts(stack.total_thickness_mm() + 2.0);

        base + spacer
            + cover
            + self.family_top_features(family, stack)
            + self.revision_id_marking(family, stack)
    }

    pub fn build_alignment_nest(&self) -> Part {
        self.validate().expect("invalid coupon parameters");
        let base_x = self.coupon_length_mm + 20.0;
        let base_y = self.coupon_width_mm + 20.0;
        let base_z = 6.0;
        let base = centered_cube("p0_321_alignment_nest_base", base_x, base_y, base_z);

        // Primary datum: three non-collinear planar support pads.
        let mut primary_pads = Part::empty("p0_321_primary_three_point_plane");
        for (index, (x, y)) in [(-34.0, -21.0), (34.0, -21.0), (0.0, 21.0)]
            .into_iter()
            .enumerate()
        {
            primary_pads =
                primary_pads
                    + centered_cylinder(format!("primary_datum_pad_{index}"), 4.0, 1.5, 32)
                        .translate(x, y, base_z / 2.0 + 0.75 - self.feature_overlap_mm);
        }

        // Secondary datum: two separated contact blocks on the long edge.
        let secondary_a = centered_cube("secondary_datum_contact_a", 16.0, 3.0, 5.0).translate(
            -24.0,
            self.coupon_width_mm / 2.0 + 1.5,
            base_z / 2.0 + 2.5 - self.feature_overlap_mm,
        );
        let secondary_b = centered_cube("secondary_datum_contact_b", 16.0, 3.0, 5.0).translate(
            16.0,
            self.coupon_width_mm / 2.0 + 1.5,
            base_z / 2.0 + 2.5 - self.feature_overlap_mm,
        );

        // Tertiary datum: one end stop. The opposite end remains unconstrained.
        let tertiary = centered_cube("tertiary_datum_single_end_stop", 3.0, 18.0, 5.0).translate(
            -self.coupon_length_mm / 2.0 - 1.5,
            0.0,
            base_z / 2.0 + 2.5 - self.feature_overlap_mm,
        );

        let round_pin = centered_cylinder(
            "round_registration_pin",
            (self.registration_hole_diameter_mm - 0.2) / 2.0,
            5.0,
            32,
        )
        .translate(
            self.registration_left_x_mm,
            self.registration_y_mm,
            base_z / 2.0 + 2.5 - self.feature_overlap_mm,
        );
        let slot_pin = self
            .obround(
                "slot_registration_pin",
                self.registration_slot_length_mm - 0.4,
                self.registration_slot_width_mm - 0.2,
                5.0,
            )
            .translate(
                self.registration_right_x_mm,
                self.registration_y_mm,
                base_z / 2.0 + 2.5 - self.feature_overlap_mm,
            );

        let keyed_guard = centered_cube("keyed_orientation_guard", 11.0, 4.0, 5.0).translate(
            self.coupon_length_mm / 2.0 - self.key_notch_length_mm / 2.0,
            self.coupon_width_mm / 2.0 + 2.0,
            base_z / 2.0 + 2.5 - self.feature_overlap_mm,
        );

        base + primary_pads
            + secondary_a
            + secondary_b
            + tertiary
            + round_pin
            + slot_pin
            + keyed_guard
    }

    fn keyed_layer(&self, name: String, thickness: f64, center_z: f64) -> Part {
        let plate = centered_cube(name, self.coupon_length_mm, self.coupon_width_mm, thickness)
            .translate(0.0, 0.0, center_z);
        let notch = centered_cube(
            "asymmetric_key_notch",
            self.key_notch_length_mm,
            self.key_notch_width_mm,
            thickness + 0.2,
        )
        .translate(
            self.coupon_length_mm / 2.0 - self.key_notch_length_mm / 2.0,
            self.coupon_width_mm / 2.0 - self.key_notch_width_mm / 2.0,
            center_z,
        );
        plate - notch
    }

    fn registration_cuts(&self, height: f64) -> Part {
        let round = centered_cylinder(
            "round_registration_hole",
            self.registration_hole_diameter_mm / 2.0,
            height,
            32,
        )
        .translate(
            self.registration_left_x_mm,
            self.registration_y_mm,
            height / 2.0,
        );
        let slot = self
            .obround(
                "relief_registration_slot",
                self.registration_slot_length_mm,
                self.registration_slot_width_mm,
                height,
            )
            .translate(
                self.registration_right_x_mm,
                self.registration_y_mm,
                height / 2.0,
            );
        round + slot
    }

    fn fiducial_cuts(&self, height: f64) -> Part {
        let mut cuts = Part::empty("fiducial_cross_cuts");
        for (index, (x, y)) in [(-27.0, 19.0), (27.0, 14.0), (0.0, -14.0)]
            .into_iter()
            .enumerate()
        {
            let horizontal = centered_cube(format!("fiducial_{index}_h"), 4.0, 0.7, height)
                .translate(x, y, height / 2.0);
            let vertical = centered_cube(format!("fiducial_{index}_v"), 0.7, 4.0, height)
                .translate(x, y, height / 2.0);
            cuts = cuts + horizontal + vertical;
        }
        cuts
    }

    fn family_fluidic_cuts(
        &self,
        family: CouponFamily,
        z: f64,
        height: f64,
        spacer_thickness_mm: f64,
    ) -> Part {
        match family {
            CouponFamily::MaterialContact => self.material_contact_cuts(z, height),
            CouponFamily::OpticalWindow => self.optical_window_cuts(z, height),
            CouponFamily::ThermalEvaporation => self.thermal_evaporation_cuts(z, height),
            CouponFamily::BondRegistration => self.bond_registration_cuts(z, height),
            CouponFamily::MeteringDebris => {
                self.metering_debris_cuts(z, height, spacer_thickness_mm)
            }
            CouponFamily::VentWaste => self.vent_waste_cuts(z, height),
            CouponFamily::SealBackflow => self.seal_backflow_cuts(z, height),
            CouponFamily::DualLaneIsolation => self.dual_lane_cuts(z, height),
            CouponFamily::SwabDockRetention => self.swab_dock_drain_cuts(z, height),
            CouponFamily::ConditionalBlister => self.blister_cuts(z, height),
            CouponFamily::SealedContainment => self.containment_cuts(z, height),
        }
    }

    fn material_contact_cuts(&self, z: f64, height: f64) -> Part {
        let mut wells = Part::empty("material_contact_wells");
        for (index, (x, y, radius)) in [
            (-24.0, 8.0, 5.0),
            (-8.0, 8.0, 5.0),
            (8.0, 8.0, 5.0),
            (24.0, 8.0, 5.0),
            (-16.0, -8.0, 6.0),
            (0.0, -8.0, 6.0),
            (16.0, -8.0, 6.0),
        ]
        .into_iter()
        .enumerate()
        {
            wells = wells
                + centered_cylinder(format!("contact_well_{index}"), radius, height, 40)
                    .translate(x, y, z);
        }
        wells
    }

    fn optical_window_cuts(&self, z: f64, height: f64) -> Part {
        self.chamber("optical_roi_a", -13.0, 3.0, z, height)
            + self.chamber("optical_roi_b", 13.0, 3.0, z, height)
            + centered_cube("adhesive_edge_witness", 54.0, 1.0, height).translate(0.0, 14.0, z)
            + centered_cylinder("bubble_failure_control", 1.5, height, 24).translate(-26.0, -8.0, z)
            + centered_cube("wrinkle_failure_control", 12.0, 0.8, height).translate(18.0, -9.0, z)
    }

    fn thermal_evaporation_cuts(&self, z: f64, height: f64) -> Part {
        self.chamber("thermal_chamber_a", -13.0, 4.0, z, height)
            + self.chamber("thermal_chamber_b", 13.0, 4.0, z, height)
            + self.channel("thermal_fill_bus", 52.0, 0.0, -9.0, z, height)
            + self
                .channel("thermal_fill_a", 12.0, -13.0, -3.0, z, height)
                .rotate(0.0, 0.0, 90.0)
            + self
                .channel("thermal_fill_b", 12.0, 13.0, -3.0, z, height)
                .rotate(0.0, 0.0, 90.0)
            + centered_cylinder("bubble_witness_a", 2.0, height, 24).translate(-20.0, 4.0, z)
            + centered_cylinder("bubble_witness_b", 2.0, height, 24).translate(20.0, 4.0, z)
    }

    fn bond_registration_cuts(&self, z: f64, height: f64) -> Part {
        let straight = self.channel("bond_straight", 58.0, 0.0, -10.0, z, height);
        let vertical = self
            .channel("bond_corner_vertical", 24.0, -25.0, 1.5, z, height)
            .rotate(0.0, 0.0, 90.0);
        let top = self.channel("bond_corner_top", 34.0, -9.0, 13.0, z, height);
        let t_branch = self
            .channel("bond_t_branch", 19.0, 8.0, 4.0, z, height)
            .rotate(0.0, 0.0, 90.0);
        let narrow =
            centered_cube("narrow_seal_land_challenge", 22.0, 0.6, height).translate(24.0, 7.0, z);
        straight + vertical + top + t_branch + narrow
    }

    fn metering_debris_cuts(&self, z: f64, height: f64, spacer_thickness_mm: f64) -> Part {
        let inlet = centered_cube("debris_inlet", 12.0, 9.0, height).translate(-29.0, 5.0, z);
        let settling =
            centered_cube("settling_pocket", 18.0, 14.0, height).translate(-15.0, 5.0, z);
        let weir = self
            .channel("debris_weir_gap", 5.0, -3.0, 5.0, z, height)
            .rotate(0.0, 0.0, 90.0);
        let barrier =
            centered_cube("barrier_insert_window", 2.0, 17.0, height).translate(0.0, 5.0, z);
        let meter_radius =
            (self.meter_nominal_ul / (std::f64::consts::PI * spacer_thickness_mm)).sqrt();
        let meter = centered_cylinder("nominal_2p5ul_meter", meter_radius, height, 40)
            .translate(10.0, 5.0, z);
        let overflow = self.channel("meter_overflow_to_waste", 26.0, 24.0, 5.0, z, height);
        let outlet = self
            .channel("meter_delivery_outlet", 17.0, 10.0, -5.0, z, height)
            .rotate(0.0, 0.0, 90.0);
        inlet + settling + weir + barrier + meter + overflow + outlet
    }

    fn vent_waste_cuts(&self, z: f64, height: f64) -> Part {
        let inlet = self.channel("vent_waste_inlet", 32.0, -22.0, -7.0, z, height);
        let headspace =
            centered_cube("captive_terminal_headspace", 22.0, 18.0, height).translate(3.0, -7.0, z);
        let absorbent_bay = centered_cube("optional_absorbent_comparator_bay", 18.0, 18.0, height)
            .translate(27.0, -7.0, z);
        let vent_neck = self
            .channel("protected_vent_neck", 11.0, 3.0, 8.0, z, height)
            .rotate(0.0, 0.0, 90.0);
        inlet + headspace + absorbent_bay + vent_neck
    }

    fn seal_backflow_cuts(&self, z: f64, height: f64) -> Part {
        let inlet = self.channel("pressure_ramp_inlet", 30.0, -24.0, 0.0, z, height);
        let burst =
            centered_cube("controlled_burst_throat", 1.0, 5.0, height).translate(-8.5, 0.0, z);
        let chamber =
            centered_cube("burst_collection_chamber", 15.0, 14.0, height).translate(0.0, 0.0, z);
        let labyrinth_a = self.channel("backflow_labyrinth_a", 18.0, 16.0, 5.0, z, height);
        let labyrinth_b = self
            .channel("backflow_labyrinth_b", 10.0, 24.0, 0.5, z, height)
            .rotate(0.0, 0.0, 90.0);
        let outlet = self.channel("reverse_pressure_port", 15.0, 31.0, -4.0, z, height);
        inlet + burst + chamber + labyrinth_a + labyrinth_b + outlet
    }

    fn dual_lane_cuts(&self, z: f64, height: f64) -> Part {
        let mut lanes = Part::empty("physically_isolated_dual_lanes");
        for (index, y) in [-8.0, 8.0].into_iter().enumerate() {
            let inlet = self.channel(format!("lane_{index}_inlet"), 17.0, -31.0, y, z, height);
            let meter = centered_cylinder(format!("lane_{index}_meter"), 2.4, height, 32)
                .translate(-20.0, y, z);
            let chamber = centered_cube(
                format!("lane_{index}_20_25ul_chamber"),
                self.chamber_length_mm,
                self.chamber_width_mm,
                height,
            )
            .translate(0.0, y, z);
            let waste = centered_cube(format!("lane_{index}_terminal_waste"), 13.0, 7.0, height)
                .translate(27.0, y, z);
            lanes = lanes + inlet + meter + chamber + waste;
        }
        lanes
    }

    fn swab_dock_drain_cuts(&self, z: f64, height: f64) -> Part {
        centered_cube("swab_dock_drainage_proxy", 40.0, 2.0, height).translate(5.0, 0.0, z)
            + centered_cube("swab_tip_contact_pocket", 14.0, 12.0, height).translate(27.0, 0.0, z)
    }

    fn blister_cuts(&self, z: f64, height: f64) -> Part {
        centered_cylinder("blister_outlet_capture", 3.0, height, 32).translate(-12.0, 0.0, z)
            + self.channel("blister_captive_outlet", 32.0, 7.0, 0.0, z, height)
            + centered_cube("anti_rebound_terminal", 10.0, 12.0, height).translate(28.0, 0.0, z)
    }

    fn containment_cuts(&self, z: f64, height: f64) -> Part {
        let internal = centered_cube("representative_contained_path", 55.0, 24.0, height);
        let fill = centered_cylinder("sealed_fill_port", 2.5, height, 32).translate(-31.0, 0.0, z);
        let vent =
            centered_cylinder("terminal_vent_capture", 3.5, height, 32).translate(31.0, 0.0, z);
        internal.translate(0.0, 0.0, z) + fill + vent
    }

    fn family_top_features(&self, family: CouponFamily, stack: MaterialStack) -> Part {
        let top_z = stack.total_thickness_mm();
        match family {
            CouponFamily::VentWaste => self.vent_top_features(top_z),
            CouponFamily::SwabDockRetention => self.swab_dock_top_features(top_z),
            CouponFamily::ConditionalBlister => self.blister_top_features(top_z),
            CouponFamily::SealedContainment => self.containment_top_features(top_z),
            CouponFamily::BondRegistration => self.bond_witness_features(top_z),
            _ => Part::empty(format!("{}_no_top_features", family.slug())),
        }
    }

    fn vent_top_features(&self, top_z: f64) -> Part {
        let ring = centered_cylinder("vent_membrane_bond_land", 7.0, 0.8, 40)
            - centered_cylinder("vent_membrane_aperture", 4.0, 1.2, 40);
        let ring = ring.translate(3.0, 16.0, top_z + 0.4 - self.feature_overlap_mm);
        let baffle_a = centered_cube("splash_baffle_a", 2.0, 14.0, 1.2).translate(
            -1.0,
            -7.0,
            top_z + 0.6 - self.feature_overlap_mm,
        );
        let baffle_b = centered_cube("splash_baffle_b", 2.0, 14.0, 1.2).translate(
            9.0,
            -7.0,
            top_z + 0.6 - self.feature_overlap_mm,
        );
        ring + baffle_a + baffle_b
    }

    fn swab_dock_top_features(&self, top_z: f64) -> Part {
        let outer = centered_cylinder("dry_swab_dock_shell", 7.5, 24.0, 48)
            .rotate(0.0, 90.0, 0.0)
            .translate(-22.0, 0.0, top_z + 6.0);
        let bore = centered_cylinder("dry_swab_shaft_bore", 5.2, 26.0, 48)
            .rotate(0.0, 90.0, 0.0)
            .translate(-22.0, 0.0, top_z + 6.0);
        let stop = centered_cube("swab_positive_hard_stop", 3.0, 17.0, 15.0).translate(
            -9.0,
            0.0,
            top_z + 6.0,
        );
        let latch_a = centered_cube("swab_latch_a", 8.0, 3.0, 4.0).translate(
            -27.0,
            -8.0,
            top_z + 2.0 - self.feature_overlap_mm,
        );
        let latch_b = centered_cube("swab_latch_b", 8.0, 3.0, 4.0).translate(
            -27.0,
            8.0,
            top_z + 2.0 - self.feature_overlap_mm,
        );
        (outer - bore) + stop + latch_a + latch_b
    }

    fn blister_top_features(&self, top_z: f64) -> Part {
        let seal_land = centered_cylinder("conditional_blister_seal_land", 12.0, 0.8, 48)
            - centered_cylinder("conditional_blister_inner_land", 9.5, 1.2, 48);
        let captive_ring = centered_cylinder("captive_puncture_guard", 5.0, 2.0, 40)
            - centered_cylinder("puncture_target", 2.0, 2.4, 32);
        let actuator = centered_cylinder("external_actuator_target", 7.0, 1.0, 40);
        seal_land.translate(-18.0, 0.0, top_z + 0.4 - self.feature_overlap_mm)
            + captive_ring.translate(-12.0, 0.0, top_z + 1.0 - self.feature_overlap_mm)
            + actuator.translate(-18.0, 0.0, top_z + 1.0 - self.feature_overlap_mm)
    }

    fn containment_top_features(&self, top_z: f64) -> Part {
        self.rectangular_frame("external_witness_moat", 72.0, 40.0, 1.4, 0.8)
            .translate(0.0, 0.0, top_z + 0.4 - self.feature_overlap_mm)
            + centered_cube("tamper_witness_bridge", 12.0, 3.0, 1.2).translate(
                -36.0,
                0.0,
                top_z + 0.6 - self.feature_overlap_mm,
            )
    }

    fn bond_witness_features(&self, top_z: f64) -> Part {
        let mut witnesses = Part::empty("registration_offset_witness_ladder");
        for (index, x) in [-18.0, -9.0, 0.0, 9.0, 18.0].into_iter().enumerate() {
            witnesses = witnesses
                + centered_cube(
                    format!("offset_witness_{index}"),
                    0.8,
                    6.0 + index as f64,
                    0.7,
                )
                .translate(x, 19.0, top_z + 0.35 - self.feature_overlap_mm);
        }
        witnesses
    }

    fn revision_id_marking(&self, family: CouponFamily, stack: MaterialStack) -> Part {
        // Geometric marking is intentionally font-independent: one long revision
        // bar, a family-count ladder, and a stack-count ladder. It remains visible
        // in STL and does not depend on installed fonts or tessellation libraries.
        let top_z = stack.total_thickness_mm();
        let mut marks = centered_cube("revision_p0_r0_bar", 18.0, 1.0, 0.5).translate(
            -20.0,
            -15.5,
            top_z + 0.25 - self.feature_overlap_mm,
        );
        for index in 0..family.index() {
            marks = marks
                + centered_cube(format!("family_id_bar_{index}"), 0.65, 3.0, 0.5).translate(
                    -28.0 + index as f64 * 1.25,
                    -12.5,
                    top_z + 0.25 - self.feature_overlap_mm,
                );
        }
        let stack_count = match stack.id {
            MaterialStackId::CocCopTarget => 1,
            MaterialStackId::PmmaControl => 2,
            MaterialStackId::PetComparator => 3,
        };
        for index in 0..stack_count {
            marks = marks
                + centered_cube(format!("stack_id_bar_{index}"), 0.8, 4.0, 0.5).translate(
                    20.0 + index as f64 * 1.6,
                    -14.0,
                    top_z + 0.25 - self.feature_overlap_mm,
                );
        }
        marks
    }

    fn chamber(&self, name: impl Into<String>, x: f64, y: f64, z: f64, height: f64) -> Part {
        centered_cube(name, self.chamber_length_mm, self.chamber_width_mm, height)
            .translate(x, y, z)
    }

    fn channel(
        &self,
        name: impl Into<String>,
        length: f64,
        x: f64,
        y: f64,
        z: f64,
        height: f64,
    ) -> Part {
        centered_cube(name, length, self.channel_width_mm, height).translate(x, y, z)
    }

    fn obround(&self, name: impl Into<String>, length: f64, width: f64, height: f64) -> Part {
        assert!(length >= width, "obround length must be at least its width");
        let name = name.into();
        let straight = length - width;
        let center = centered_cube(format!("{name}_center"), straight, width, height);
        let left = centered_cylinder(format!("{name}_left"), width / 2.0, height, 32).translate(
            -straight / 2.0,
            0.0,
            0.0,
        );
        let right = centered_cylinder(format!("{name}_right"), width / 2.0, height, 32).translate(
            straight / 2.0,
            0.0,
            0.0,
        );
        center + left + right
    }

    fn rectangular_frame(
        &self,
        name: &str,
        outer_x: f64,
        outer_y: f64,
        wall: f64,
        height: f64,
    ) -> Part {
        centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
            - centered_cube(
                format!("{name}_inner"),
                outer_x - wall * 2.0,
                outer_y - wall * 2.0,
                height + 0.2,
            )
    }

    pub fn assert_valid_stack(&self, stack: MaterialStack) {
        assert!(
            stack.base_thickness_mm > 0.0,
            "base thickness must be positive"
        );
        assert!(
            stack.spacer_thickness_mm > 0.0,
            "spacer thickness must be positive"
        );
        assert!(
            stack.cover_thickness_mm > 0.0,
            "cover thickness must be positive"
        );
        assert!(
            (20.0..=25.0).contains(&stack.chamber_volume_ul(self)),
            "paired chamber geometry must remain inside the proposed 20-25 uL envelope"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    fn nominal() -> Parameters {
        toml::from_str(include_str!("../models/p0_cartridge_coupons.toml")).unwrap()
    }

    #[test]
    fn all_material_stacks_are_valid_and_configurable() {
        assert_eq!(nominal().material_stacks().len(), 3);
        for stack in nominal().material_stacks() {
            nominal().assert_valid_stack(stack);
            assert!((20.0..=25.0).contains(&stack.chamber_volume_ul(&nominal())));
        }
        assert!(nominal().material_stacks()[2].conditional);
        assert!(!nominal().material_stacks()[0].conditional);
    }

    #[test]
    fn family_matrix_is_complete_and_unique() {
        assert_eq!(CouponFamily::ALL.len(), 11);
        let slugs: BTreeSet<_> = CouponFamily::ALL
            .into_iter()
            .map(CouponFamily::slug)
            .collect();
        assert_eq!(slugs.len(), CouponFamily::ALL.len());
        assert!(CouponFamily::ALL.contains(&CouponFamily::ConditionalBlister));
        assert!(CouponFamily::ALL.contains(&CouponFamily::SealedContainment));
    }

    #[test]
    fn registration_is_round_hole_plus_relief_slot() {
        assert_eq!(
            nominal().registration_hole_diameter_mm,
            nominal().registration_slot_width_mm
        );
        assert!(nominal().registration_slot_length_mm > nominal().registration_slot_width_mm);
        assert_ne!(
            nominal().registration_left_x_mm,
            nominal().registration_right_x_mm
        );
    }

    #[test]
    fn artifact_and_revision_traceability_is_fixed() {
        assert_eq!(TICKET_ID, "T-A2021311");
        assert_eq!(REVISION, "P0-R0");
        assert_eq!(SOURCE_ARTIFACTS.len(), 3);
        assert_eq!(SOURCE_ARTIFACTS[0], "A-A1A77D11");
        assert_eq!(SOURCE_ARTIFACTS[1], "A-696CE730");
        assert_eq!(SOURCE_ARTIFACTS[2], "A-CE59D39F");
    }

    #[test]
    fn static_manifest_tracks_every_family_and_stack() {
        let manifest: toml::Value =
            toml::from_str(include_str!("../manifests/p0_cartridge_coupon_suite.toml"))
                .expect("static P0 coupon manifest must parse");
        let families = manifest["families"].as_array().expect("families array");
        let stacks = manifest["material_stacks"]
            .as_array()
            .expect("material stacks array");
        assert_eq!(families.len(), CouponFamily::ALL.len());
        assert_eq!(stacks.len(), nominal().material_stacks().len());
        assert_eq!(manifest["suite"]["ticket"].as_str(), Some(TICKET_ID));
        assert_eq!(manifest["suite"]["revision"].as_str(), Some(REVISION));
    }
}

impl Parameters {
    pub fn load(path: &std::path::Path) -> Result<(Self, String), Box<dyn std::error::Error>> {
        use sha2::{Digest, Sha256};
        let raw = std::fs::read_to_string(path)?;
        let parameters: Self = toml::from_str(&raw)?;
        parameters.validate()?;
        Ok((parameters, format!("{:x}", Sha256::digest(raw.as_bytes()))))
    }

    pub fn material_stacks(&self) -> [MaterialStack; 3] {
        [
            (
                MaterialStackId::CocCopTarget,
                "coc_cop_target",
                "target-faithful COC/COP-centered engineering comparator",
                self.coc_cop_target,
                false,
            ),
            (
                MaterialStackId::PmmaControl,
                "pmma_control",
                "rapid-fabrication PMMA engineering control",
                self.pmma_control,
                false,
            ),
            (
                MaterialStackId::PetComparator,
                "pet_comparator",
                "conditional all-film PET process comparator",
                self.pet_comparator,
                true,
            ),
        ]
        .map(|(id, slug, role, p, conditional)| MaterialStack {
            id,
            slug,
            role,
            conditional,
            base_thickness_mm: p.base_thickness_mm,
            spacer_thickness_mm: p.spacer_thickness_mm,
            cover_thickness_mm: p.cover_thickness_mm,
        })
    }

    pub fn select_stacks(&self, requested: &str) -> Result<Vec<MaterialStack>, String> {
        self.validate()?;
        let selected: Vec<_> = self
            .material_stacks()
            .into_iter()
            .filter(|s| requested == "all" || s.slug == requested)
            .collect();
        if selected.is_empty() {
            return Err(format!("unknown stack {requested}; expected all, coc_cop_target, pmma_control or pet_comparator"));
        }
        Ok(selected)
    }

    pub fn validate(&self) -> Result<(), String> {
        for (name, value, low, high) in [
            ("coupon_length_mm", self.coupon_length_mm, 86.0, 110.0),
            ("coupon_width_mm", self.coupon_width_mm, 54.0, 70.0),
            ("key_notch_length_mm", self.key_notch_length_mm, 7.5, 8.5),
            ("key_notch_width_mm", self.key_notch_width_mm, 6.5, 7.5),
            (
                "registration_hole_diameter_mm",
                self.registration_hole_diameter_mm,
                2.8,
                3.6,
            ),
            (
                "registration_slot_width_mm",
                self.registration_slot_width_mm,
                2.8,
                3.6,
            ),
            (
                "registration_slot_length_mm",
                self.registration_slot_length_mm,
                7.0,
                9.0,
            ),
            ("registration_y_mm", self.registration_y_mm, -21.0, -19.0),
            (
                "registration_left_x_mm",
                self.registration_left_x_mm,
                -32.0,
                -30.0,
            ),
            (
                "registration_right_x_mm",
                self.registration_right_x_mm,
                27.0,
                29.0,
            ),
            ("min_seal_land_mm", self.min_seal_land_mm, 3.0, 5.0),
            ("channel_width_mm", self.channel_width_mm, 0.8, 1.6),
            ("chamber_length_mm", self.chamber_length_mm, 19.0, 21.0),
            ("chamber_width_mm", self.chamber_width_mm, 7.5, 8.5),
            ("meter_nominal_ul", self.meter_nominal_ul, 0.5, 5.0),
            ("feature_overlap_mm", self.feature_overlap_mm, 0.02, 0.08),
        ] {
            if !value.is_finite() || !(low..=high).contains(&value) {
                return Err(format!(
                    "{name} must be finite and within {low}..={high} for this coupon family"
                ));
            }
        }
        if self.registration_hole_diameter_mm != self.registration_slot_width_mm {
            return Err("round hole and relief slot must share locating width".into());
        }
        if self.coupon_width_mm / 2.0
            - self.registration_y_mm.abs()
            - self.registration_hole_diameter_mm / 2.0
            < self.min_seal_land_mm
        {
            return Err("registration cuts violate minimum edge land".into());
        }
        for s in self.material_stacks() {
            for (name, value, low, high) in [
                ("base", s.base_thickness_mm, 0.2, 2.0),
                ("spacer", s.spacer_thickness_mm, 0.05, 0.4),
                ("cover", s.cover_thickness_mm, 0.1, 0.5),
            ] {
                if !value.is_finite() || !(low..=high).contains(&value) {
                    return Err(format!(
                        "{} {name} thickness must be finite and within {low}..={high} mm",
                        s.slug
                    ));
                }
            }
            if !(20.0..=25.0).contains(&s.chamber_volume_ul(self)) {
                return Err(format!(
                    "{} paired chamber must remain in the proposed 20–25 uL geometry envelope",
                    s.slug
                ));
            }
        }
        Ok(())
    }
}
