//! Deterministic dry P0 laminated-cartridge coupon geometry.
//!
//! The dimensions in this module are proposed CAD envelopes for engineering
//! coupons. They are not validated tolerances, manufacturing release values,
//! assay conditions, or vendor-specific material selections.

use serde::Serialize;
use vcad::{centered_cube, centered_cylinder, Part};

pub const SUITE_ID: &str = "LF-P0-CARTRIDGE-COUPONS";
pub const REVISION: &str = "P0-R0";
pub const TICKET_ID: &str = "T-A2021311";
pub const SOURCE_ARTIFACTS: [&str; 3] = ["A-A1A77D11", "A-696CE730", "A-CE59D39F"];

pub const COUPON_LENGTH_MM: f64 = 86.0;
pub const COUPON_WIDTH_MM: f64 = 54.0;
pub const KEY_NOTCH_LENGTH_MM: f64 = 8.0;
pub const KEY_NOTCH_WIDTH_MM: f64 = 7.0;
pub const REGISTRATION_HOLE_DIAMETER_MM: f64 = 3.2;
pub const REGISTRATION_SLOT_WIDTH_MM: f64 = 3.2;
pub const REGISTRATION_SLOT_LENGTH_MM: f64 = 8.0;
pub const REGISTRATION_Y_MM: f64 = -20.0;
pub const REGISTRATION_LEFT_X_MM: f64 = -31.0;
pub const REGISTRATION_RIGHT_X_MM: f64 = 28.0;
pub const MIN_SEAL_LAND_MM: f64 = 3.0;
pub const CHANNEL_WIDTH_MM: f64 = 1.2;
pub const CHAMBER_LENGTH_MM: f64 = 20.0;
pub const CHAMBER_WIDTH_MM: f64 = 8.0;
pub const METER_NOMINAL_UL: f64 = 2.5;
pub const FEATURE_OVERLAP_MM: f64 = 0.04;

const _: () = {
    assert!(METER_NOMINAL_UL >= 0.5 && METER_NOMINAL_UL <= 5.0);
    assert!(MIN_SEAL_LAND_MM > CHANNEL_WIDTH_MM);
    assert!(REGISTRATION_SLOT_LENGTH_MM > REGISTRATION_SLOT_WIDTH_MM);
};

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

    pub fn chamber_volume_ul(self) -> f64 {
        CHAMBER_LENGTH_MM * CHAMBER_WIDTH_MM * self.spacer_thickness_mm
    }
}

pub const MATERIAL_STACKS: [MaterialStack; 3] = [
    MaterialStack {
        id: MaterialStackId::CocCopTarget,
        slug: "coc_cop_target",
        role: "target-faithful COC/COP-centered engineering comparator",
        base_thickness_mm: 1.00,
        spacer_thickness_mm: 0.14,
        cover_thickness_mm: 0.20,
        conditional: false,
    },
    MaterialStack {
        id: MaterialStackId::PmmaControl,
        slug: "pmma_control",
        role: "rapid-fabrication PMMA engineering control",
        base_thickness_mm: 1.50,
        spacer_thickness_mm: 0.14,
        cover_thickness_mm: 0.20,
        conditional: false,
    },
    MaterialStack {
        id: MaterialStackId::PetComparator,
        slug: "pet_comparator",
        role: "conditional all-film PET process comparator",
        base_thickness_mm: 0.25,
        spacer_thickness_mm: 0.14,
        cover_thickness_mm: 0.125,
        conditional: true,
    },
];

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

pub fn build_coupon(family: CouponFamily, stack: MaterialStack) -> Part {
    assert_valid_stack(stack);

    let base = keyed_layer(
        format!("{}_{}_base", family.slug(), stack.slug),
        stack.base_thickness_mm,
        stack.base_thickness_mm / 2.0,
    ) - registration_cuts(stack.total_thickness_mm() + 2.0);

    let spacer_z = stack.base_thickness_mm + stack.spacer_thickness_mm / 2.0;
    let spacer = keyed_layer(
        format!("{}_{}_spacer", family.slug(), stack.slug),
        stack.spacer_thickness_mm,
        spacer_z,
    ) - registration_cuts(stack.total_thickness_mm() + 2.0)
        - family_fluidic_cuts(
            family,
            spacer_z,
            stack.spacer_thickness_mm + 0.08,
            stack.spacer_thickness_mm,
        );

    let cover_z =
        stack.base_thickness_mm + stack.spacer_thickness_mm + stack.cover_thickness_mm / 2.0;
    let cover = keyed_layer(
        format!("{}_{}_cover", family.slug(), stack.slug),
        stack.cover_thickness_mm,
        cover_z,
    ) - registration_cuts(stack.total_thickness_mm() + 2.0)
        - fiducial_cuts(stack.total_thickness_mm() + 2.0);

    base + spacer + cover + family_top_features(family, stack) + revision_id_marking(family, stack)
}

pub fn build_alignment_nest() -> Part {
    let base_x = COUPON_LENGTH_MM + 20.0;
    let base_y = COUPON_WIDTH_MM + 20.0;
    let base_z = 6.0;
    let base = centered_cube("p0_321_alignment_nest_base", base_x, base_y, base_z);

    // Primary datum: three non-collinear planar support pads.
    let mut primary_pads = Part::empty("p0_321_primary_three_point_plane");
    for (index, (x, y)) in [(-34.0, -21.0), (34.0, -21.0), (0.0, 21.0)]
        .into_iter()
        .enumerate()
    {
        primary_pads = primary_pads
            + centered_cylinder(format!("primary_datum_pad_{index}"), 4.0, 1.5, 32).translate(
                x,
                y,
                base_z / 2.0 + 0.75 - FEATURE_OVERLAP_MM,
            );
    }

    // Secondary datum: two separated contact blocks on the long edge.
    let secondary_a = centered_cube("secondary_datum_contact_a", 16.0, 3.0, 5.0).translate(
        -24.0,
        COUPON_WIDTH_MM / 2.0 + 1.5,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );
    let secondary_b = centered_cube("secondary_datum_contact_b", 16.0, 3.0, 5.0).translate(
        16.0,
        COUPON_WIDTH_MM / 2.0 + 1.5,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );

    // Tertiary datum: one end stop. The opposite end remains unconstrained.
    let tertiary = centered_cube("tertiary_datum_single_end_stop", 3.0, 18.0, 5.0).translate(
        -COUPON_LENGTH_MM / 2.0 - 1.5,
        0.0,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );

    let round_pin = centered_cylinder(
        "round_registration_pin",
        (REGISTRATION_HOLE_DIAMETER_MM - 0.2) / 2.0,
        5.0,
        32,
    )
    .translate(
        REGISTRATION_LEFT_X_MM,
        REGISTRATION_Y_MM,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );
    let slot_pin = obround(
        "slot_registration_pin",
        REGISTRATION_SLOT_LENGTH_MM - 0.4,
        REGISTRATION_SLOT_WIDTH_MM - 0.2,
        5.0,
    )
    .translate(
        REGISTRATION_RIGHT_X_MM,
        REGISTRATION_Y_MM,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );

    let keyed_guard = centered_cube("keyed_orientation_guard", 11.0, 4.0, 5.0).translate(
        COUPON_LENGTH_MM / 2.0 - KEY_NOTCH_LENGTH_MM / 2.0,
        COUPON_WIDTH_MM / 2.0 + 2.0,
        base_z / 2.0 + 2.5 - FEATURE_OVERLAP_MM,
    );

    base + primary_pads + secondary_a + secondary_b + tertiary + round_pin + slot_pin + keyed_guard
}

fn keyed_layer(name: String, thickness: f64, center_z: f64) -> Part {
    let plate = centered_cube(name, COUPON_LENGTH_MM, COUPON_WIDTH_MM, thickness)
        .translate(0.0, 0.0, center_z);
    let notch = centered_cube(
        "asymmetric_key_notch",
        KEY_NOTCH_LENGTH_MM,
        KEY_NOTCH_WIDTH_MM,
        thickness + 0.2,
    )
    .translate(
        COUPON_LENGTH_MM / 2.0 - KEY_NOTCH_LENGTH_MM / 2.0,
        COUPON_WIDTH_MM / 2.0 - KEY_NOTCH_WIDTH_MM / 2.0,
        center_z,
    );
    plate - notch
}

fn registration_cuts(height: f64) -> Part {
    let round = centered_cylinder(
        "round_registration_hole",
        REGISTRATION_HOLE_DIAMETER_MM / 2.0,
        height,
        32,
    )
    .translate(REGISTRATION_LEFT_X_MM, REGISTRATION_Y_MM, height / 2.0);
    let slot = obround(
        "relief_registration_slot",
        REGISTRATION_SLOT_LENGTH_MM,
        REGISTRATION_SLOT_WIDTH_MM,
        height,
    )
    .translate(REGISTRATION_RIGHT_X_MM, REGISTRATION_Y_MM, height / 2.0);
    round + slot
}

fn fiducial_cuts(height: f64) -> Part {
    let mut cuts = Part::empty("fiducial_cross_cuts");
    for (index, (x, y)) in [(-27.0, 19.0), (27.0, 14.0), (0.0, -14.0)]
        .into_iter()
        .enumerate()
    {
        let horizontal = centered_cube(format!("fiducial_{index}_h"), 4.0, 0.7, height).translate(
            x,
            y,
            height / 2.0,
        );
        let vertical = centered_cube(format!("fiducial_{index}_v"), 0.7, 4.0, height).translate(
            x,
            y,
            height / 2.0,
        );
        cuts = cuts + horizontal + vertical;
    }
    cuts
}

fn family_fluidic_cuts(
    family: CouponFamily,
    z: f64,
    height: f64,
    spacer_thickness_mm: f64,
) -> Part {
    match family {
        CouponFamily::MaterialContact => material_contact_cuts(z, height),
        CouponFamily::OpticalWindow => optical_window_cuts(z, height),
        CouponFamily::ThermalEvaporation => thermal_evaporation_cuts(z, height),
        CouponFamily::BondRegistration => bond_registration_cuts(z, height),
        CouponFamily::MeteringDebris => metering_debris_cuts(z, height, spacer_thickness_mm),
        CouponFamily::VentWaste => vent_waste_cuts(z, height),
        CouponFamily::SealBackflow => seal_backflow_cuts(z, height),
        CouponFamily::DualLaneIsolation => dual_lane_cuts(z, height),
        CouponFamily::SwabDockRetention => swab_dock_drain_cuts(z, height),
        CouponFamily::ConditionalBlister => blister_cuts(z, height),
        CouponFamily::SealedContainment => containment_cuts(z, height),
    }
}

fn material_contact_cuts(z: f64, height: f64) -> Part {
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

fn optical_window_cuts(z: f64, height: f64) -> Part {
    chamber("optical_roi_a", -13.0, 3.0, z, height)
        + chamber("optical_roi_b", 13.0, 3.0, z, height)
        + centered_cube("adhesive_edge_witness", 54.0, 1.0, height).translate(0.0, 14.0, z)
        + centered_cylinder("bubble_failure_control", 1.5, height, 24).translate(-26.0, -8.0, z)
        + centered_cube("wrinkle_failure_control", 12.0, 0.8, height).translate(18.0, -9.0, z)
}

fn thermal_evaporation_cuts(z: f64, height: f64) -> Part {
    chamber("thermal_chamber_a", -13.0, 4.0, z, height)
        + chamber("thermal_chamber_b", 13.0, 4.0, z, height)
        + channel("thermal_fill_bus", 52.0, 0.0, -9.0, z, height)
        + channel("thermal_fill_a", 12.0, -13.0, -3.0, z, height).rotate(0.0, 0.0, 90.0)
        + channel("thermal_fill_b", 12.0, 13.0, -3.0, z, height).rotate(0.0, 0.0, 90.0)
        + centered_cylinder("bubble_witness_a", 2.0, height, 24).translate(-20.0, 4.0, z)
        + centered_cylinder("bubble_witness_b", 2.0, height, 24).translate(20.0, 4.0, z)
}

fn bond_registration_cuts(z: f64, height: f64) -> Part {
    let straight = channel("bond_straight", 58.0, 0.0, -10.0, z, height);
    let vertical =
        channel("bond_corner_vertical", 24.0, -25.0, 1.5, z, height).rotate(0.0, 0.0, 90.0);
    let top = channel("bond_corner_top", 34.0, -9.0, 13.0, z, height);
    let t_branch = channel("bond_t_branch", 19.0, 8.0, 4.0, z, height).rotate(0.0, 0.0, 90.0);
    let narrow =
        centered_cube("narrow_seal_land_challenge", 22.0, 0.6, height).translate(24.0, 7.0, z);
    straight + vertical + top + t_branch + narrow
}

fn metering_debris_cuts(z: f64, height: f64, spacer_thickness_mm: f64) -> Part {
    let inlet = centered_cube("debris_inlet", 12.0, 9.0, height).translate(-29.0, 5.0, z);
    let settling = centered_cube("settling_pocket", 18.0, 14.0, height).translate(-15.0, 5.0, z);
    let weir = channel("debris_weir_gap", 5.0, -3.0, 5.0, z, height).rotate(0.0, 0.0, 90.0);
    let barrier = centered_cube("barrier_insert_window", 2.0, 17.0, height).translate(0.0, 5.0, z);
    let meter_radius = (METER_NOMINAL_UL / (std::f64::consts::PI * spacer_thickness_mm)).sqrt();
    let meter =
        centered_cylinder("nominal_2p5ul_meter", meter_radius, height, 40).translate(10.0, 5.0, z);
    let overflow = channel("meter_overflow_to_waste", 26.0, 24.0, 5.0, z, height);
    let outlet =
        channel("meter_delivery_outlet", 17.0, 10.0, -5.0, z, height).rotate(0.0, 0.0, 90.0);
    inlet + settling + weir + barrier + meter + overflow + outlet
}

fn vent_waste_cuts(z: f64, height: f64) -> Part {
    let inlet = channel("vent_waste_inlet", 32.0, -22.0, -7.0, z, height);
    let headspace =
        centered_cube("captive_terminal_headspace", 22.0, 18.0, height).translate(3.0, -7.0, z);
    let absorbent_bay = centered_cube("optional_absorbent_comparator_bay", 18.0, 18.0, height)
        .translate(27.0, -7.0, z);
    let vent_neck =
        channel("protected_vent_neck", 11.0, 3.0, 8.0, z, height).rotate(0.0, 0.0, 90.0);
    inlet + headspace + absorbent_bay + vent_neck
}

fn seal_backflow_cuts(z: f64, height: f64) -> Part {
    let inlet = channel("pressure_ramp_inlet", 30.0, -24.0, 0.0, z, height);
    let burst = centered_cube("controlled_burst_throat", 1.0, 5.0, height).translate(-8.5, 0.0, z);
    let chamber =
        centered_cube("burst_collection_chamber", 15.0, 14.0, height).translate(0.0, 0.0, z);
    let labyrinth_a = channel("backflow_labyrinth_a", 18.0, 16.0, 5.0, z, height);
    let labyrinth_b =
        channel("backflow_labyrinth_b", 10.0, 24.0, 0.5, z, height).rotate(0.0, 0.0, 90.0);
    let outlet = channel("reverse_pressure_port", 15.0, 31.0, -4.0, z, height);
    inlet + burst + chamber + labyrinth_a + labyrinth_b + outlet
}

fn dual_lane_cuts(z: f64, height: f64) -> Part {
    let mut lanes = Part::empty("physically_isolated_dual_lanes");
    for (index, y) in [-8.0, 8.0].into_iter().enumerate() {
        let inlet = channel(format!("lane_{index}_inlet"), 17.0, -31.0, y, z, height);
        let meter = centered_cylinder(format!("lane_{index}_meter"), 2.4, height, 32)
            .translate(-20.0, y, z);
        let chamber = centered_cube(
            format!("lane_{index}_20_25ul_chamber"),
            CHAMBER_LENGTH_MM,
            CHAMBER_WIDTH_MM,
            height,
        )
        .translate(0.0, y, z);
        let waste = centered_cube(format!("lane_{index}_terminal_waste"), 13.0, 7.0, height)
            .translate(27.0, y, z);
        lanes = lanes + inlet + meter + chamber + waste;
    }
    lanes
}

fn swab_dock_drain_cuts(z: f64, height: f64) -> Part {
    centered_cube("swab_dock_drainage_proxy", 40.0, 2.0, height).translate(5.0, 0.0, z)
        + centered_cube("swab_tip_contact_pocket", 14.0, 12.0, height).translate(27.0, 0.0, z)
}

fn blister_cuts(z: f64, height: f64) -> Part {
    centered_cylinder("blister_outlet_capture", 3.0, height, 32).translate(-12.0, 0.0, z)
        + channel("blister_captive_outlet", 32.0, 7.0, 0.0, z, height)
        + centered_cube("anti_rebound_terminal", 10.0, 12.0, height).translate(28.0, 0.0, z)
}

fn containment_cuts(z: f64, height: f64) -> Part {
    let internal = centered_cube("representative_contained_path", 55.0, 24.0, height);
    let fill = centered_cylinder("sealed_fill_port", 2.5, height, 32).translate(-31.0, 0.0, z);
    let vent = centered_cylinder("terminal_vent_capture", 3.5, height, 32).translate(31.0, 0.0, z);
    internal.translate(0.0, 0.0, z) + fill + vent
}

fn family_top_features(family: CouponFamily, stack: MaterialStack) -> Part {
    let top_z = stack.total_thickness_mm();
    match family {
        CouponFamily::VentWaste => vent_top_features(top_z),
        CouponFamily::SwabDockRetention => swab_dock_top_features(top_z),
        CouponFamily::ConditionalBlister => blister_top_features(top_z),
        CouponFamily::SealedContainment => containment_top_features(top_z),
        CouponFamily::BondRegistration => bond_witness_features(top_z),
        _ => Part::empty(format!("{}_no_top_features", family.slug())),
    }
}

fn vent_top_features(top_z: f64) -> Part {
    let ring = centered_cylinder("vent_membrane_bond_land", 7.0, 0.8, 40)
        - centered_cylinder("vent_membrane_aperture", 4.0, 1.2, 40);
    let ring = ring.translate(3.0, 16.0, top_z + 0.4 - FEATURE_OVERLAP_MM);
    let baffle_a = centered_cube("splash_baffle_a", 2.0, 14.0, 1.2).translate(
        -1.0,
        -7.0,
        top_z + 0.6 - FEATURE_OVERLAP_MM,
    );
    let baffle_b = centered_cube("splash_baffle_b", 2.0, 14.0, 1.2).translate(
        9.0,
        -7.0,
        top_z + 0.6 - FEATURE_OVERLAP_MM,
    );
    ring + baffle_a + baffle_b
}

fn swab_dock_top_features(top_z: f64) -> Part {
    let outer = centered_cylinder("dry_swab_dock_shell", 7.5, 24.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(-22.0, 0.0, top_z + 6.0);
    let bore = centered_cylinder("dry_swab_shaft_bore", 5.2, 26.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(-22.0, 0.0, top_z + 6.0);
    let stop =
        centered_cube("swab_positive_hard_stop", 3.0, 17.0, 15.0).translate(-9.0, 0.0, top_z + 6.0);
    let latch_a = centered_cube("swab_latch_a", 8.0, 3.0, 4.0).translate(
        -27.0,
        -8.0,
        top_z + 2.0 - FEATURE_OVERLAP_MM,
    );
    let latch_b = centered_cube("swab_latch_b", 8.0, 3.0, 4.0).translate(
        -27.0,
        8.0,
        top_z + 2.0 - FEATURE_OVERLAP_MM,
    );
    (outer - bore) + stop + latch_a + latch_b
}

fn blister_top_features(top_z: f64) -> Part {
    let seal_land = centered_cylinder("conditional_blister_seal_land", 12.0, 0.8, 48)
        - centered_cylinder("conditional_blister_inner_land", 9.5, 1.2, 48);
    let captive_ring = centered_cylinder("captive_puncture_guard", 5.0, 2.0, 40)
        - centered_cylinder("puncture_target", 2.0, 2.4, 32);
    let actuator = centered_cylinder("external_actuator_target", 7.0, 1.0, 40);
    seal_land.translate(-18.0, 0.0, top_z + 0.4 - FEATURE_OVERLAP_MM)
        + captive_ring.translate(-12.0, 0.0, top_z + 1.0 - FEATURE_OVERLAP_MM)
        + actuator.translate(-18.0, 0.0, top_z + 1.0 - FEATURE_OVERLAP_MM)
}

fn containment_top_features(top_z: f64) -> Part {
    rectangular_frame("external_witness_moat", 72.0, 40.0, 1.4, 0.8).translate(
        0.0,
        0.0,
        top_z + 0.4 - FEATURE_OVERLAP_MM,
    ) + centered_cube("tamper_witness_bridge", 12.0, 3.0, 1.2).translate(
        -36.0,
        0.0,
        top_z + 0.6 - FEATURE_OVERLAP_MM,
    )
}

fn bond_witness_features(top_z: f64) -> Part {
    let mut witnesses = Part::empty("registration_offset_witness_ladder");
    for (index, x) in [-18.0, -9.0, 0.0, 9.0, 18.0].into_iter().enumerate() {
        witnesses = witnesses
            + centered_cube(
                format!("offset_witness_{index}"),
                0.8,
                6.0 + index as f64,
                0.7,
            )
            .translate(x, 19.0, top_z + 0.35 - FEATURE_OVERLAP_MM);
    }
    witnesses
}

fn revision_id_marking(family: CouponFamily, stack: MaterialStack) -> Part {
    // Geometric marking is intentionally font-independent: one long revision
    // bar, a family-count ladder, and a stack-count ladder. It remains visible
    // in STL and does not depend on installed fonts or tessellation libraries.
    let top_z = stack.total_thickness_mm();
    let mut marks = centered_cube("revision_p0_r0_bar", 18.0, 1.0, 0.5).translate(
        -20.0,
        -15.5,
        top_z + 0.25 - FEATURE_OVERLAP_MM,
    );
    for index in 0..family.index() {
        marks = marks
            + centered_cube(format!("family_id_bar_{index}"), 0.65, 3.0, 0.5).translate(
                -28.0 + index as f64 * 1.25,
                -12.5,
                top_z + 0.25 - FEATURE_OVERLAP_MM,
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
                top_z + 0.25 - FEATURE_OVERLAP_MM,
            );
    }
    marks
}

fn chamber(name: impl Into<String>, x: f64, y: f64, z: f64, height: f64) -> Part {
    centered_cube(name, CHAMBER_LENGTH_MM, CHAMBER_WIDTH_MM, height).translate(x, y, z)
}

fn channel(name: impl Into<String>, length: f64, x: f64, y: f64, z: f64, height: f64) -> Part {
    centered_cube(name, length, CHANNEL_WIDTH_MM, height).translate(x, y, z)
}

fn obround(name: impl Into<String>, length: f64, width: f64, height: f64) -> Part {
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

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, wall: f64, height: f64) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
        - centered_cube(
            format!("{name}_inner"),
            outer_x - wall * 2.0,
            outer_y - wall * 2.0,
            height + 0.2,
        )
}

pub fn assert_valid_stack(stack: MaterialStack) {
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
        (20.0..=25.0).contains(&stack.chamber_volume_ul()),
        "paired chamber geometry must remain inside the proposed 20-25 uL envelope"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn all_material_stacks_are_valid_and_configurable() {
        assert_eq!(MATERIAL_STACKS.len(), 3);
        for stack in MATERIAL_STACKS {
            assert_valid_stack(stack);
            assert!((20.0..=25.0).contains(&stack.chamber_volume_ul()));
        }
        assert!(MATERIAL_STACKS[2].conditional);
        assert!(!MATERIAL_STACKS[0].conditional);
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
        assert_eq!(REGISTRATION_HOLE_DIAMETER_MM, REGISTRATION_SLOT_WIDTH_MM);
        assert!(REGISTRATION_SLOT_LENGTH_MM > REGISTRATION_SLOT_WIDTH_MM);
        assert_ne!(REGISTRATION_LEFT_X_MM, REGISTRATION_RIGHT_X_MM);
        assert_eq!(REGISTRATION_Y_MM, REGISTRATION_Y_MM);
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
        assert_eq!(stacks.len(), MATERIAL_STACKS.len());
        assert_eq!(manifest["suite"]["ticket"].as_str(), Some(TICKET_ID));
        assert_eq!(manifest["suite"]["revision"].as_str(), Some(REVISION));
    }
}
