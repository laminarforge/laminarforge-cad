//! Parametric, single-architecture sealed swab diagnostic cartridge.
//!
//! All dimensions are proposed engineering envelopes for CAD integration and
//! design verification. They are not validated manufacturing tolerances,
//! clinical performance claims, assay conditions, or wet-lab instructions.

use serde::Serialize;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use vcad::{centered_cube, centered_cylinder, Part};

pub const PUBLICATION_STEM: &str = "swab_integrated_sealed_diagnostic_cartridge";
pub const VERIFY_BIN: &str = "swab_integrated_sealed_diagnostic_cartridge_verify";
pub const TICKET_ID: &str = "T-588F689A";
pub const REVISION: &str = "P1-INT-R0";
pub const SOURCE_ARTIFACTS: [&str; 7] = [
    "A-A1A77D11",
    "A-345211E9",
    "A-696CE730",
    "A-CE59D39F",
    "A-BC05D7E3",
    "A-EE7B1198",
    "A-7C487534",
];

pub const REQUIRED_FEATURES: [&str; 15] = [
    "permanently_closing_swab_entry",
    "sealed_swab_elution_chamber",
    "wash_pouch_cell",
    "paired_reaction_fill_pouch_lobes",
    "two_dry_reader_plungers",
    "pressure_staged_one_way_planar_path",
    "lysis_heater_region",
    "two_dry_reagent_rehydration_regions",
    "two_clear_amplification_optical_chambers",
    "bubble_management_high_points",
    "reader_thermal_optical_alignment_interfaces",
    "terminal_one_way_waste_chamber",
    "mechanically_confined_low_shedding_absorbent_pad",
    "reserved_waste_headspace_and_splash_baffle",
    "physically_separated_gas_only_hydrophobic_aerosol_barrier_vent",
];

pub const PROHIBITED_FEATURES: [&str; 7] = [
    "evaporation_or_desiccation_path",
    "superabsorbent_gel",
    "waste_heating",
    "capture_wash_alternate",
    "reusable_pump",
    "reusable_wet_plumbing",
    "alternate_flow_route",
];

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CartridgeParams {
    pub body_length_mm: f64,
    pub body_width_mm: f64,
    pub rigid_base_thickness_mm: f64,
    pub represented_spacer_thickness_mm: f64,
    pub optical_cover_film_thickness_mm: f64,
    pub minimum_perimeter_seal_land_mm: f64,
    pub liquid_channel_width_mm: f64,
    pub liquid_channel_depth_mm: f64,
    pub swab_cup_inside_diameter_mm: f64,
    pub swab_cup_wetted_height_mm: f64,
    pub wash_pouch_length_mm: f64,
    pub wash_pouch_width_mm: f64,
    pub wash_pouch_internal_height_mm: f64,
    pub reaction_pouch_lobe_length_mm: f64,
    pub reaction_pouch_lobe_width_mm: f64,
    pub reaction_pouch_lobe_internal_height_mm: f64,
    pub amplification_length_mm: f64,
    pub amplification_width_mm: f64,
    pub amplification_internal_height_mm: f64,
    pub waste_internal_length_mm: f64,
    pub waste_internal_width_mm: f64,
    pub waste_internal_height_mm: f64,
    pub waste_reserved_headspace_height_mm: f64,
    pub proposed_pad_retained_capacity_ul: f64,
    pub plunger_count: usize,
    pub optical_chamber_count: usize,
}

impl Default for CartridgeParams {
    fn default() -> Self {
        Self {
            body_length_mm: 150.0,
            body_width_mm: 86.0,
            rigid_base_thickness_mm: 2.0,
            represented_spacer_thickness_mm: 0.14,
            optical_cover_film_thickness_mm: 0.20,
            minimum_perimeter_seal_land_mm: 3.0,
            liquid_channel_width_mm: 1.2,
            liquid_channel_depth_mm: 0.14,
            swab_cup_inside_diameter_mm: 13.0,
            swab_cup_wetted_height_mm: 20.0,
            wash_pouch_length_mm: 30.0,
            wash_pouch_width_mm: 20.0,
            wash_pouch_internal_height_mm: 1.0,
            reaction_pouch_lobe_length_mm: 20.0,
            reaction_pouch_lobe_width_mm: 8.0,
            reaction_pouch_lobe_internal_height_mm: 0.14,
            amplification_length_mm: 20.0,
            amplification_width_mm: 8.0,
            amplification_internal_height_mm: 0.14,
            waste_internal_length_mm: 36.0,
            waste_internal_width_mm: 30.0,
            waste_internal_height_mm: 2.4,
            waste_reserved_headspace_height_mm: 0.8,
            proposed_pad_retained_capacity_ul: 1_350.0,
            plunger_count: 2,
            optical_chamber_count: 2,
        }
    }
}

impl CartridgeParams {
    pub fn wash_pouch_nominal_ul(self) -> f64 {
        self.wash_pouch_length_mm * self.wash_pouch_width_mm * self.wash_pouch_internal_height_mm
    }

    pub fn reaction_pouch_lobe_nominal_ul(self) -> f64 {
        self.reaction_pouch_lobe_length_mm
            * self.reaction_pouch_lobe_width_mm
            * self.reaction_pouch_lobe_internal_height_mm
    }

    pub fn amplification_chamber_nominal_ul(self) -> f64 {
        self.amplification_length_mm
            * self.amplification_width_mm
            * self.amplification_internal_height_mm
    }

    pub fn maximum_delivered_liquid_ul(self) -> f64 {
        self.wash_pouch_nominal_ul() + 2.0 * self.reaction_pouch_lobe_nominal_ul()
    }

    pub fn waste_internal_volume_ul(self) -> f64 {
        self.waste_internal_length_mm * self.waste_internal_width_mm * self.waste_internal_height_mm
    }

    pub fn reserved_headspace_fraction(self) -> f64 {
        self.waste_reserved_headspace_height_mm / self.waste_internal_height_mm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Liquid,
    GasOnly,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct FlowEdge {
    pub from: &'static str,
    pub to: &'static str,
    pub kind: EdgeKind,
}

pub const FLOW_EDGES: [FlowEdge; 20] = [
    FlowEdge {
        from: "wash_pouch",
        to: "swab_elution",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "swab_elution",
        to: "lysis",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "lysis",
        to: "debris_barrier",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "debris_barrier",
        to: "target_meter",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "debris_barrier",
        to: "control_meter",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "debris_barrier",
        to: "terminal_waste",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "target_meter",
        to: "target_reagent",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "target_reaction_pouch",
        to: "target_reagent",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "target_reagent",
        to: "target_amplification",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "target_amplification",
        to: "target_bubble_outlet",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "target_bubble_outlet",
        to: "terminal_waste",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "control_meter",
        to: "control_reagent",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "control_reaction_pouch",
        to: "control_reagent",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "control_reagent",
        to: "control_amplification",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "control_amplification",
        to: "control_bubble_outlet",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "control_bubble_outlet",
        to: "terminal_waste",
        kind: EdgeKind::Liquid,
    },
    FlowEdge {
        from: "terminal_waste",
        to: "captive_headspace",
        kind: EdgeKind::GasOnly,
    },
    FlowEdge {
        from: "captive_headspace",
        to: "splash_baffle",
        kind: EdgeKind::GasOnly,
    },
    FlowEdge {
        from: "splash_baffle",
        to: "vent_plenum",
        kind: EdgeKind::GasOnly,
    },
    FlowEdge {
        from: "vent_plenum",
        to: "hydrophobic_aerosol_barrier",
        kind: EdgeKind::GasOnly,
    },
];

pub const PRESSURE_STAGE_KPA: [f64; 5] = [12.0, 18.0, 24.0, 31.0, 38.0];
pub const PROPOSED_MINIMUM_DESTRUCTIVE_PRESSURE_KPA: f64 = 80.0;

#[derive(Debug, Serialize)]
pub struct DesignManifest {
    pub schema_version: &'static str,
    pub publication_stem: &'static str,
    pub revision: &'static str,
    pub ticket: &'static str,
    pub source_artifacts: [&'static str; 7],
    pub architecture: &'static str,
    pub design_status: &'static str,
    pub parameters: CartridgeParams,
    pub derived: DerivedParameters,
    pub pressure_stage_kpa: [f64; 5],
    pub minimum_destructive_pressure_kpa: f64,
    pub flow_edges: &'static [FlowEdge],
    pub required_features: [&'static str; 15],
    pub prohibited_features_absent: [&'static str; 7],
    pub reusable_reader_wet_interfaces: usize,
    pub disposable_liquid_outlets: usize,
}

#[derive(Debug, Serialize)]
pub struct DerivedParameters {
    pub wash_pouch_nominal_ul: f64,
    pub reaction_pouch_lobe_nominal_ul: f64,
    pub amplification_chamber_nominal_ul: f64,
    pub maximum_delivered_liquid_ul: f64,
    pub waste_internal_volume_ul: f64,
    pub waste_reserved_headspace_fraction: f64,
    pub pad_capacity_to_maximum_delivery_ratio: f64,
}

pub fn design_manifest(params: CartridgeParams) -> DesignManifest {
    DesignManifest {
        schema_version: "1",
        publication_stem: PUBLICATION_STEM,
        revision: REVISION,
        ticket: TICKET_ID,
        source_artifacts: SOURCE_ARTIFACTS,
        architecture: "one irreversible two-plunger laminated-planar sealed disposable path",
        design_status: "proposed engineering dimensions; no manufacturing, assay, shelf-life, disposal, or clinical validation claimed",
        parameters: params,
        derived: DerivedParameters {
            wash_pouch_nominal_ul: params.wash_pouch_nominal_ul(),
            reaction_pouch_lobe_nominal_ul: params.reaction_pouch_lobe_nominal_ul(),
            amplification_chamber_nominal_ul: params.amplification_chamber_nominal_ul(),
            maximum_delivered_liquid_ul: params.maximum_delivered_liquid_ul(),
            waste_internal_volume_ul: params.waste_internal_volume_ul(),
            waste_reserved_headspace_fraction: params.reserved_headspace_fraction(),
            pad_capacity_to_maximum_delivery_ratio: params.proposed_pad_retained_capacity_ul
                / params.maximum_delivered_liquid_ul(),
        },
        pressure_stage_kpa: PRESSURE_STAGE_KPA,
        minimum_destructive_pressure_kpa: PROPOSED_MINIMUM_DESTRUCTIVE_PRESSURE_KPA,
        flow_edges: &FLOW_EDGES,
        required_features: REQUIRED_FEATURES,
        prohibited_features_absent: PROHIBITED_FEATURES,
        reusable_reader_wet_interfaces: 0,
        disposable_liquid_outlets: 0,
    }
}

pub fn verify_design(params: CartridgeParams) -> Result<(), String> {
    let close = |actual: f64, expected: f64| (actual - expected).abs() < 1.0e-9;
    if !close(params.wash_pouch_nominal_ul(), 600.0) {
        return Err("wash pouch must remain the proposed exact-dose 600 uL envelope".into());
    }
    if !close(params.reaction_pouch_lobe_nominal_ul(), 22.4) {
        return Err("each separately sealed reaction pouch lobe must remain 22.4 uL".into());
    }
    if !close(params.amplification_chamber_nominal_ul(), 22.4) {
        return Err("each optical amplification chamber must remain 22.4 uL".into());
    }
    if params.plunger_count != 2 || params.optical_chamber_count != 2 {
        return Err(
            "architecture requires exactly two dry plungers and two optical chambers".into(),
        );
    }
    if params.minimum_perimeter_seal_land_mm < 3.0 {
        return Err("represented perimeter seal land must be at least 3.0 mm".into());
    }
    if params.reserved_headspace_fraction() < 0.20 {
        return Err("terminal waste must reserve at least 20 percent gas headspace".into());
    }
    if params.proposed_pad_retained_capacity_ul < 2.0 * params.maximum_delivered_liquid_ul() {
        return Err("proposed confined pad capacity must be at least 2x maximum delivery".into());
    }
    if PRESSURE_STAGE_KPA.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("one-way pressure stages must be strictly increasing".into());
    }
    if PRESSURE_STAGE_KPA[PRESSURE_STAGE_KPA.len() - 1]
        > PROPOSED_MINIMUM_DESTRUCTIVE_PRESSURE_KPA / 2.0
    {
        return Err(
            "highest operating stage exceeds 50 percent destructive-pressure screen".into(),
        );
    }
    verify_topology()?;
    Ok(())
}

fn verify_topology() -> Result<(), String> {
    let liquid: Vec<_> = FLOW_EDGES
        .iter()
        .filter(|edge| edge.kind == EdgeKind::Liquid)
        .collect();
    let gas: Vec<_> = FLOW_EDGES
        .iter()
        .filter(|edge| edge.kind == EdgeKind::GasOnly)
        .collect();
    if liquid
        .iter()
        .any(|edge| edge.to == "hydrophobic_aerosol_barrier")
    {
        return Err("the exterior barrier may only receive the separated gas path".into());
    }
    if gas.first().map(|edge| edge.from) != Some("terminal_waste")
        || gas.last().map(|edge| edge.to) != Some("hydrophobic_aerosol_barrier")
    {
        return Err("gas-only vent path must start at waste and terminate at the barrier".into());
    }

    let mut adjacency: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in &liquid {
        adjacency.entry(edge.from).or_default().push(edge.to);
    }
    for source in [
        "wash_pouch",
        "target_reaction_pouch",
        "control_reaction_pouch",
    ] {
        if !reaches_terminal(source, &adjacency, &mut BTreeSet::new()) {
            return Err(format!(
                "liquid source {source} does not reach terminal waste"
            ));
        }
    }
    if has_cycle(&adjacency) {
        return Err("liquid topology must remain acyclic and irreversible".into());
    }
    Ok(())
}

fn reaches_terminal<'a>(
    node: &'a str,
    adjacency: &BTreeMap<&'a str, Vec<&'a str>>,
    visited: &mut BTreeSet<&'a str>,
) -> bool {
    if node == "terminal_waste" {
        return true;
    }
    if !visited.insert(node) {
        return false;
    }
    adjacency.get(node).is_some_and(|next| {
        next.iter()
            .any(|node| reaches_terminal(node, adjacency, visited))
    })
}

fn has_cycle(adjacency: &BTreeMap<&str, Vec<&str>>) -> bool {
    fn visit<'a>(
        node: &'a str,
        adjacency: &BTreeMap<&'a str, Vec<&'a str>>,
        temporary: &mut BTreeSet<&'a str>,
        permanent: &mut BTreeSet<&'a str>,
    ) -> bool {
        if permanent.contains(node) {
            return false;
        }
        if !temporary.insert(node) {
            return true;
        }
        if adjacency.get(node).is_some_and(|next| {
            next.iter()
                .any(|node| visit(node, adjacency, temporary, permanent))
        }) {
            return true;
        }
        temporary.remove(node);
        permanent.insert(node);
        false
    }

    let mut temporary = BTreeSet::new();
    let mut permanent = BTreeSet::new();
    adjacency
        .keys()
        .any(|node| visit(node, adjacency, &mut temporary, &mut permanent))
}

pub fn build_model(params: CartridgeParams) -> Part {
    build_model_components(params).into_iter().fold(
        Part::empty("integrated_sealed_swab_cartridge"),
        |assembly, part| assembly + part,
    )
}

/// Return the integrated assembly as deterministic, named multi-solid groups.
/// Keeping groups separate during STL publication avoids adding artificial
/// triangle splits where opaque review witnesses merely touch or overlap.
pub fn build_model_components(params: CartridgeParams) -> Vec<Part> {
    verify_design(params).expect("integrated cartridge parameters must satisfy architecture gates");
    vec![
        cartridge_base(params),
        perimeter_seal(params),
        swab_entry_and_elution(params),
        pouch_cells_and_dry_plungers(params),
        planar_fluid_path(params),
        lysis_and_reagent_regions(params),
        amplification_and_bubble_regions(params),
        waste_headspace_and_vent(params),
        reader_interfaces(params),
        geometric_identity_marks(params),
    ]
}

/// Bounded-face publication solids for STL→STEP→USDZ conversion.
///
/// This is the same integrated architecture as [`build_model_components`],
/// expressed without Boolean unions between adjacent witness solids. The CAD
/// tab therefore retains every visible feature while avoiding triangle-face
/// multiplication in the required STL-derived STEP conversion.
pub fn build_publication_components(p: CartridgeParams) -> Vec<Part> {
    verify_design(p).expect("integrated cartridge parameters must satisfy architecture gates");
    let parts = RefCell::new(Vec::new());
    let cube = |name: &str, x: f64, y: f64, z: f64, sx: f64, sy: f64, sz: f64| {
        parts
            .borrow_mut()
            .push(centered_cube(name, sx, sy, sz).translate(x, y, z));
    };

    cube(
        "sealed_laminated_cartridge_rigid_base",
        0.0,
        0.0,
        0.0,
        p.body_length_mm,
        p.body_width_mm,
        2.0,
    );
    // Continuous perimeter seal, represented as four independent converted rails.
    cube("perimeter_seal_front", 0.0, -40.0, 1.5, 144.0, 3.0, 0.7);
    cube("perimeter_seal_rear", 0.0, 40.0, 1.5, 144.0, 3.0, 0.7);
    cube("perimeter_seal_left", -72.0, 0.0, 1.5, 3.0, 80.0, 0.7);
    cube("perimeter_seal_right", 72.0, 0.0, 1.5, 3.0, 80.0, 0.7);

    // Permanently closed swab entry and elution body.
    parts.borrow_mut().push(
        centered_cylinder("sealed_swab_elution_chamber_body", 8.7, 22.0, 8)
            .translate(-61.0, -10.0, 12.3),
    );
    parts.borrow_mut().push(
        centered_cylinder("cartridge_owned_compression_grommet", 4.7, 2.0, 8)
            .translate(-61.0, -10.0, 23.2),
    );
    parts
        .borrow_mut()
        .push(centered_cylinder("captured_swab_shaft", 1.5, 22.0, 8).translate(-61.0, -10.0, 14.0));
    parts.borrow_mut().push(
        centered_cylinder("permanently_closing_swab_cap", 9.7, 2.4, 8)
            .translate(-61.0, -10.0, 25.0),
    );
    cube(
        "tamper_evident_irreversible_latch",
        -61.0,
        -10.0,
        24.0,
        25.0,
        3.0,
        2.0,
    );
    cube("swab_cap_hard_stop", -50.0, -10.0, 22.0, 3.0, 23.0, 5.0);

    // Separately sealed positive-displacement pouch cells and two dry plungers.
    cube(
        "exact_dose_wash_pouch_600ul",
        -53.0,
        27.0,
        2.3,
        30.0,
        20.0,
        1.4,
    );
    push_rail_frame(
        &parts,
        "wash_pouch_seal",
        -53.0,
        27.0,
        34.0,
        24.0,
        2.0,
        0.7,
        2.0,
    );
    parts
        .borrow_mut()
        .push(centered_cylinder("dry_reader_plunger_a", 6.0, 10.0, 8).translate(-53.0, 27.0, 11.0));
    parts.borrow_mut().push(
        centered_cylinder("dry_reader_plunger_a_guide", 8.0, 2.0, 8).translate(-53.0, 27.0, 6.0),
    );
    cube(
        "sealed_target_reaction_fill_lobe_22_4ul",
        -18.0,
        31.0,
        2.3,
        20.0,
        8.0,
        1.1,
    );
    cube(
        "sealed_control_reaction_fill_lobe_22_4ul",
        -18.0,
        22.0,
        2.3,
        20.0,
        8.0,
        1.1,
    );
    push_rail_frame(
        &parts,
        "paired_reaction_lobe_seal",
        -18.0,
        26.5,
        26.0,
        23.0,
        2.0,
        0.7,
        2.0,
    );
    cube(
        "reaction_lobe_independence_seal",
        -18.0,
        26.5,
        2.0,
        24.0,
        2.0,
        0.7,
    );
    cube(
        "dry_reader_plunger_b_equalizing_platen",
        -18.0,
        26.5,
        6.3,
        20.0,
        17.0,
        2.0,
    );
    parts
        .borrow_mut()
        .push(centered_cylinder("dry_reader_plunger_b", 6.0, 9.0, 8).translate(-18.0, 26.5, 12.0));
    parts.borrow_mut().push(
        centered_cylinder("dry_reader_plunger_b_guide", 8.0, 2.0, 8).translate(-18.0, 26.5, 7.4),
    );

    // One-way laminated planar liquid path, including the single terminal overflow route.
    let channel_specs = [
        ("wash_to_swab", -61.0, 11.0, 1.2, 20.0),
        ("swab_to_lysis", -48.0, -10.0, 16.0, 1.2),
        ("lysis_to_debris", -23.0, -10.0, 10.0, 1.2),
        ("debris_target_split", -13.0, 4.0, 10.0, 1.2),
        ("target_meter_riser", -8.0, -3.0, 1.2, 14.0),
        ("debris_control_split", -13.0, -10.0, 10.0, 1.2),
        ("target_meter_to_reagent", -1.0, 4.0, 12.0, 1.2),
        ("target_reagent_to_optical", 11.0, 4.0, 12.0, 1.2),
        ("target_optical_to_bubble", 31.0, 4.0, 12.0, 1.2),
        ("target_bubble_to_waste", 42.5, 4.0, 11.0, 1.2),
        ("control_meter_to_reagent", -1.0, -10.0, 12.0, 1.2),
        ("control_reagent_to_optical", 11.0, -10.0, 12.0, 1.2),
        ("control_optical_to_bubble", 31.0, -10.0, 12.0, 1.2),
        ("control_bubble_to_waste", 42.5, -10.0, 11.0, 1.2),
        ("target_lobe_to_reagent", -18.0, 17.5, 1.2, 27.0),
        ("target_lobe_crossfeed", -7.0, 4.0, 22.0, 1.2),
        ("control_lobe_to_reagent", -12.0, 6.0, 1.2, 32.0),
        ("control_lobe_crossfeed", -4.0, -10.0, 16.0, 1.2),
        ("meter_overflow_drop", -3.0, -22.0, 1.2, 24.0),
        ("terminal_overflow_bus", 25.0, -22.0, 56.0, 1.2),
        ("terminal_overflow_entry", 53.0, -16.0, 1.2, 12.0),
    ];
    for (name, x, y, sx, sy) in channel_specs {
        cube(name, x, y, 2.3, sx, sy, 0.5);
    }
    for (index, (x, y)) in [
        (-61.0, 9.0),
        (-19.0, -10.0),
        (-6.5, 4.0),
        (37.0, 4.0),
        (47.5, -3.0),
        (37.0, -3.0),
    ]
    .into_iter()
    .enumerate()
    {
        cube(
            &format!("pressure_stage_{index}_burst_throat"),
            x,
            y,
            3.0,
            1.0,
            4.0,
            1.2,
        );
        cube(
            &format!("pressure_stage_{index}_forward_flap"),
            x + 1.5,
            y,
            3.4,
            3.0,
            0.8,
            1.0,
        );
        cube(
            &format!("pressure_stage_{index}_reverse_stop"),
            x - 1.2,
            y,
            3.5,
            1.2,
            1.2,
            1.6,
        );
    }

    // Lysis, debris, meters, dry-reagent rehydration, optics, and bubble management.
    cube(
        "heater_coupled_lysis_region",
        -36.0,
        -10.0,
        3.0,
        24.0,
        16.0,
        1.0,
    );
    push_rail_frame(
        &parts,
        "lysis_hold_seal",
        -36.0,
        -10.0,
        28.0,
        20.0,
        2.0,
        0.7,
        2.5,
    );
    cube("settling_region", -18.0, -10.0, 3.0, 10.0, 13.0, 0.9);
    cube(
        "supported_debris_barrier",
        -13.0,
        -10.0,
        3.2,
        2.0,
        13.0,
        1.6,
    );
    cube("target_overflow_meter_2_5ul", -8.0, 4.0, 3.0, 7.0, 5.0, 0.9);
    cube(
        "control_overflow_meter_2_5ul",
        -8.0,
        -10.0,
        3.0,
        7.0,
        5.0,
        0.9,
    );
    for (name, y) in [("target", 4.0), ("control", -10.0)] {
        push_rail_frame(
            &parts,
            &format!("{name}_dry_reagent_vestibule"),
            4.0,
            y,
            10.0,
            8.0,
            1.5,
            0.8,
            3.0,
        );
        cube(
            &format!("{name}_dry_reagent_deposit"),
            4.0,
            y,
            3.6,
            2.5,
            2.5,
            1.5,
        );
        cube(
            &format!("clear_{name}_amplification_optical_window"),
            20.0,
            y,
            3.0,
            20.0,
            8.0,
            0.35,
        );
        push_rail_frame(
            &parts,
            &format!("{name}_lane_isolating_seal"),
            20.0,
            y,
            24.0,
            12.0,
            2.0,
            0.65,
            2.6,
        );
        cube(
            &format!("{name}_bottom_up_fill_ramp"),
            9.5,
            y,
            3.0,
            5.0,
            4.0,
            0.8,
        );
        cube(
            &format!("{name}_high_point_bubble_outlet"),
            38.0,
            y,
            4.3,
            4.0,
            4.0,
            4.0,
        );
    }

    // Terminal confined pad, reserved headspace, splash baffles, and separated gas vent.
    cube("terminal_waste_floor", 55.0, -7.0, 1.5, 36.0, 30.0, 0.6);
    push_rail_frame(
        &parts,
        "terminal_waste_wall",
        55.0,
        -7.0,
        40.0,
        34.0,
        2.0,
        5.0,
        3.7,
    );
    for index in 0..6 {
        cube(
            &format!("low_shedding_pad_wick_{index}"),
            55.0,
            -17.0 + index as f64 * 4.0,
            2.4,
            28.0,
            1.5,
            0.8,
        );
    }
    cube("pad_retainer_left", 39.5, -7.0, 3.0, 2.0, 27.0, 2.3);
    cube("pad_retainer_right", 70.5, -7.0, 3.0, 2.0, 27.0, 2.3);
    cube("pad_retainer_front", 55.0, -20.5, 3.0, 33.0, 2.0, 2.3);
    cube("pad_retainer_rear", 55.0, 6.5, 3.0, 33.0, 2.0, 2.3);
    push_rail_frame(
        &parts,
        "reserved_gas_headspace",
        55.0,
        -7.0,
        34.0,
        28.0,
        2.0,
        0.7,
        5.8,
    );
    cube("splash_baffle_a", 63.0, -8.0, 4.0, 2.0, 22.0, 3.3);
    cube("splash_baffle_b", 68.0, -1.0, 4.0, 2.0, 18.0, 3.3);
    cube(
        "physically_separated_gas_only_path",
        68.0,
        17.0,
        5.8,
        1.4,
        20.0,
        0.8,
    );
    cube("gas_only_vent_plenum", 62.0, 27.0, 5.8, 12.0, 1.4, 0.8);
    parts.borrow_mut().push(
        centered_cylinder("hydrophobic_aerosol_barrier_support", 6.0, 1.4, 8)
            .translate(56.0, 27.0, 5.8),
    );
    parts.borrow_mut().push(
        centered_cylinder("gas_only_hydrophobic_aerosol_barrier_vent", 4.5, 0.7, 8)
            .translate(56.0, 27.0, 6.7),
    );

    // Dry reader thermal, optical, and deterministic alignment interfaces.
    cube(
        "reader_lysis_thermal_contact",
        -36.0,
        -10.0,
        -1.4,
        26.0,
        18.0,
        0.8,
    );
    cube(
        "reader_target_thermal_contact",
        20.0,
        4.0,
        -1.4,
        22.0,
        10.0,
        0.8,
    );
    cube(
        "reader_control_thermal_contact",
        20.0,
        -10.0,
        -1.4,
        22.0,
        10.0,
        0.8,
    );
    push_rail_frame(
        &parts,
        "target_camera_led_alignment",
        20.0,
        4.0,
        24.0,
        12.0,
        2.0,
        1.0,
        4.2,
    );
    push_rail_frame(
        &parts,
        "control_camera_led_alignment",
        20.0,
        -10.0,
        24.0,
        12.0,
        2.0,
        1.0,
        4.2,
    );
    parts
        .borrow_mut()
        .push(centered_cylinder("reader_round_datum", 1.6, 1.0, 8).translate(-66.0, -35.0, -1.5));
    cube("reader_relief_slot_datum", 61.0, -35.0, -1.5, 8.0, 3.2, 1.0);
    cube(
        "asymmetric_orientation_key",
        -69.0,
        39.0,
        1.6,
        9.0,
        6.0,
        1.0,
    );
    cube("reader_clamp_datum_left", -30.0, -42.0, 1.0, 10.0, 3.0, 3.0);
    cube("reader_clamp_datum_right", 32.0, -42.0, 1.0, 10.0, 3.0, 3.0);
    parts.into_inner()
}

fn push_rail_frame(
    parts: &RefCell<Vec<Part>>,
    name: &str,
    x: f64,
    y: f64,
    outer_x: f64,
    outer_y: f64,
    rail: f64,
    height: f64,
    z: f64,
) {
    let mut parts = parts.borrow_mut();
    parts.push(
        centered_cube(format!("{name}_front"), outer_x, rail, height).translate(
            x,
            y - (outer_y - rail) / 2.0,
            z,
        ),
    );
    parts.push(
        centered_cube(format!("{name}_rear"), outer_x, rail, height).translate(
            x,
            y + (outer_y - rail) / 2.0,
            z,
        ),
    );
    parts.push(
        centered_cube(format!("{name}_left"), rail, outer_y - 2.0 * rail, height).translate(
            x - (outer_x - rail) / 2.0,
            y,
            z,
        ),
    );
    parts.push(
        centered_cube(format!("{name}_right"), rail, outer_y - 2.0 * rail, height).translate(
            x + (outer_x - rail) / 2.0,
            y,
            z,
        ),
    );
}

/// Deterministically encode the multi-solid review assembly as binary STL.
pub fn composite_stl_bytes(parts: &[Part]) -> Result<Vec<u8>, String> {
    let meshes: Vec<_> = parts.iter().map(Part::to_mesh).collect();
    let triangle_count: usize = meshes.iter().map(|mesh| mesh.indices().len() / 3).sum();
    if triangle_count == 0 {
        return Err("integrated publication geometry is empty".into());
    }

    let mut bytes = Vec::with_capacity(84 + triangle_count * 50);
    let mut header = [0u8; 80];
    let label = b"swab_integrated_sealed_diagnostic_cartridge multi-solid P1-INT-R0";
    header[..label.len()].copy_from_slice(label);
    bytes.extend_from_slice(&header);
    bytes.extend_from_slice(&(triangle_count as u32).to_le_bytes());

    for mesh in &meshes {
        let vertices = mesh.vertices();
        for triangle in mesh.indices().chunks_exact(3) {
            let vertex = |index: u32| {
                let offset = index as usize * 3;
                [vertices[offset], vertices[offset + 1], vertices[offset + 2]]
            };
            let a = vertex(triangle[0]);
            let b = vertex(triangle[1]);
            let c = vertex(triangle[2]);
            let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let magnitude =
                (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            let normal = if magnitude > f32::EPSILON {
                [
                    cross[0] / magnitude,
                    cross[1] / magnitude,
                    cross[2] / magnitude,
                ]
            } else {
                [0.0, 0.0, 0.0]
            };
            for value in normal.into_iter().chain(a).chain(b).chain(c) {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            bytes.extend_from_slice(&0u16.to_le_bytes());
        }
    }
    Ok(bytes)
}

fn cartridge_base(p: CartridgeParams) -> Part {
    let base = centered_cube(
        "sealed_laminated_cartridge_rigid_base",
        p.body_length_mm,
        p.body_width_mm,
        p.rigid_base_thickness_mm,
    );
    let round_hole =
        centered_cylinder("reader_round_datum_hole", 1.6, 4.0, 8).translate(-66.0, -35.0, 0.0);
    let slot = centered_cube("reader_relief_datum_slot", 8.0, 3.2, 4.0).translate(61.0, -35.0, 0.0)
        + centered_cylinder("reader_relief_slot_round_a", 1.6, 4.0, 8).translate(57.0, -35.0, 0.0)
        + centered_cylinder("reader_relief_slot_round_b", 1.6, 4.0, 8).translate(65.0, -35.0, 0.0);
    let orientation_key =
        centered_cube("asymmetric_orientation_key", 10.0, 8.0, 4.0).translate(-70.0, 39.0, 0.0);
    base - round_hole - slot - orientation_key
}

fn perimeter_seal(p: CartridgeParams) -> Part {
    rectangular_ring(
        "continuous_moisture_barrier_perimeter_seal",
        p.body_length_mm - 4.0,
        p.body_width_mm - 4.0,
        p.minimum_perimeter_seal_land_mm,
        0.65,
        1.30,
    )
}

fn swab_entry_and_elution(p: CartridgeParams) -> Part {
    let center = (-61.0, -10.0);
    let inner_r = p.swab_cup_inside_diameter_mm / 2.0;
    let wall = centered_cylinder("sealed_swab_elution_outer_wall", inner_r + 2.2, 23.0, 8)
        .translate(center.0, center.1, 12.5)
        - centered_cylinder("sealed_swab_elution_internal_volume", inner_r, 20.0, 8)
            .translate(center.0, center.1, 14.0);
    let elution_floor = centered_cylinder("sealed_swab_elution_floor", inner_r + 2.2, 2.0, 8)
        .translate(center.0, center.1, 2.0);
    let captured_swab_shaft =
        centered_cylinder("captured_swab_shaft", 1.5, 23.0, 8).translate(center.0, center.1, 14.0);
    let compression_grommet = centered_cylinder("cartridge_owned_compression_grommet", 4.7, 2.0, 8)
        .translate(center.0, center.1, 23.2)
        - centered_cylinder("grommet_shaft_bore", 1.6, 3.0, 8).translate(center.0, center.1, 23.2);
    let permanent_cap =
        centered_cylinder("permanently_closing_keyed_swab_cap", inner_r + 3.2, 2.4, 8)
            .translate(center.0, center.1, 25.0);
    let latch = centered_cube("tamper_evident_irreversible_latch_bridge", 26.0, 3.0, 2.0)
        .translate(center.0, center.1, 24.0)
        + centered_cube("swab_cap_hard_stop", 3.0, 24.0, 5.0).translate(-50.0, center.1, 22.0);
    wall + elution_floor + captured_swab_shaft + compression_grommet + permanent_cap + latch
}

fn pouch_cells_and_dry_plungers(_p: CartridgeParams) -> Part {
    let z = 2.15;
    let wash_pouch = rounded_cell("hermetic_exact_dose_wash_pouch_600ul", 30.0, 20.0, 1.8, 14)
        .translate(-53.0, 27.0, z)
        + rectangular_ring("wash_pouch_capture_seal", 34.0, 24.0, 2.0, 0.7, z);

    let target_lobe = rounded_cell(
        "sealed_target_reaction_fill_lobe_22_4ul",
        20.0,
        8.0,
        1.4,
        12,
    )
    .translate(-18.0, 31.0, z);
    let control_lobe = rounded_cell(
        "sealed_control_reaction_fill_lobe_22_4ul",
        20.0,
        8.0,
        1.4,
        12,
    )
    .translate(-18.0, 22.0, z);
    let reaction_capture =
        rectangular_ring("paired_reaction_lobe_capture_seal", 26.0, 23.0, 2.0, 0.7, z)
            .translate(-18.0, 26.5, 0.0)
            + centered_cube("reaction_lobe_independence_seal_bar", 24.0, 2.0, 0.7)
                .translate(-18.0, 26.5, z);

    let wash_plunger = centered_cylinder("dry_reader_plunger_a", 6.0, 11.0, 8)
        .translate(-53.0, 27.0, 11.0)
        + centered_cylinder("plunger_a_dry_guide_collar", 8.0, 2.0, 8).translate(-53.0, 27.0, 6.0);
    let equalizing_platen =
        centered_cube("dry_reader_plunger_b_equalizing_platen", 20.0, 17.0, 2.0)
            .translate(-18.0, 26.5, 6.3);
    let reaction_plunger = centered_cylinder("dry_reader_plunger_b", 6.0, 10.0, 8)
        .translate(-18.0, 26.5, 12.0)
        + centered_cylinder("plunger_b_dry_guide_collar", 8.0, 2.0, 8).translate(-18.0, 26.5, 7.4);

    wash_pouch
        + target_lobe
        + control_lobe
        + reaction_capture
        + wash_plunger
        + equalizing_platen
        + reaction_plunger
}

fn planar_fluid_path(p: CartridgeParams) -> Part {
    let z = 2.30;
    let h = 0.55;
    let w = p.liquid_channel_width_mm;
    let main = channel_y("wash_pouch_to_swab", -61.0, 11.0, 20.0, w, h, z)
        + channel_x("swab_to_lysis", -48.0, -10.0, 16.0, w, h, z)
        + channel_x("lysis_to_debris", -23.0, -10.0, 10.0, w, h, z);

    let split = channel_x("debris_to_target_meter", -13.0, 4.0, 10.0, w, h, z)
        + channel_y("target_meter_riser", -8.0, -3.0, 14.0, w, h, z)
        + channel_x("debris_to_control_meter", -13.0, -10.0, 10.0, w, h, z)
        + channel_y("control_meter_drop", -8.0, -10.0, 6.0, w, h, z);

    let target_lane = channel_x("target_meter_to_reagent", -1.0, 4.0, 12.0, w, h, z)
        + channel_x("target_reagent_to_optical", 11.0, 4.0, 12.0, w, h, z)
        + channel_x("target_optical_to_bubble_outlet", 31.0, 4.0, 12.0, w, h, z)
        + channel_x("target_bubble_to_terminal_waste", 42.5, 4.0, 11.0, w, h, z);
    let control_lane = channel_x("control_meter_to_reagent", -1.0, -10.0, 12.0, w, h, z)
        + channel_x("control_reagent_to_optical", 11.0, -10.0, 12.0, w, h, z)
        + channel_x(
            "control_optical_to_bubble_outlet",
            31.0,
            -10.0,
            12.0,
            w,
            h,
            z,
        )
        + channel_x(
            "control_bubble_to_terminal_waste",
            42.5,
            -10.0,
            11.0,
            w,
            h,
            z,
        );

    let reaction_fill = channel_y(
        "target_reaction_lobe_to_reagent",
        -18.0,
        17.5,
        27.0,
        w,
        h,
        z,
    ) + channel_x("target_reaction_fill_crossfeed", -7.0, 4.0, 22.0, w, h, z)
        + channel_y(
            "control_reaction_lobe_to_reagent",
            -12.0,
            6.0,
            32.0,
            w,
            h,
            z,
        )
        + channel_x(
            "control_reaction_fill_crossfeed",
            -4.0,
            -10.0,
            16.0,
            w,
            h,
            z,
        );

    let overflow = channel_y(
        "meter_overflow_to_terminal_waste",
        -3.0,
        -22.0,
        24.0,
        w,
        h,
        z,
    ) + channel_x("overflow_terminal_bus", 25.0, -22.0, 56.0, w, h, z)
        + channel_y("overflow_terminal_entry", 53.0, -16.0, 12.0, w, h, z);

    let pressure_gates = [
        (-61.0, 9.0),
        (-19.0, -10.0),
        (-6.5, 4.0),
        (37.0, 4.0),
        (47.5, -3.0),
    ]
    .into_iter()
    .enumerate()
    .fold(
        Part::empty("pressure_staged_burst_and_flap_elements"),
        |part, (index, (x, y))| part + one_way_gate(index, x, y, z + 0.45),
    );

    main + split + target_lane + control_lane + reaction_fill + overflow + pressure_gates
}

fn lysis_and_reagent_regions(_p: CartridgeParams) -> Part {
    let lysis = rounded_cell("heater_coupled_lysis_pocket", 24.0, 16.0, 1.0, 14)
        .translate(-36.0, -10.0, 2.65)
        + rectangular_ring("lysis_pressure_hold_seal", 28.0, 20.0, 2.0, 0.7, 2.35)
            .translate(-36.0, -10.0, 0.0);
    let settling = rounded_cell("settling_and_supported_debris_region", 10.0, 13.0, 0.9, 12)
        .translate(-18.0, -10.0, 2.6)
        + centered_cube("supported_debris_barrier", 2.0, 13.0, 1.6).translate(-13.0, -10.0, 3.0);
    let meters = rounded_cell("target_overflow_meter_2_5ul", 7.0, 5.0, 0.9, 12)
        .translate(-8.0, 4.0, 2.6)
        + rounded_cell("control_overflow_meter_2_5ul", 7.0, 5.0, 0.9, 12)
            .translate(-8.0, -10.0, 2.6);
    let reagents = dry_reagent_region("target_dry_reagent_rehydration", 4.0, 4.0)
        + dry_reagent_region("control_dry_reagent_rehydration", 4.0, -10.0);
    lysis + settling + meters + reagents
}

fn amplification_and_bubble_regions(_p: CartridgeParams) -> Part {
    let target = optical_chamber("clear_target_amplification_optical_chamber", 20.0, 4.0);
    let control = optical_chamber("clear_control_amplification_optical_chamber", 20.0, -10.0);
    let bubble_outlets = centered_cube("target_high_point_bubble_outlet", 4.0, 4.0, 4.0)
        .translate(38.0, 4.0, 4.3)
        + centered_cube("control_high_point_bubble_outlet", 4.0, 4.0, 4.0)
            .translate(38.0, -10.0, 4.3)
        + centered_cube("target_bottom_up_fill_ramp", 5.0, 4.0, 1.0)
            .rotate(0.0, 12.0, 0.0)
            .translate(9.5, 4.0, 2.8)
        + centered_cube("control_bottom_up_fill_ramp", 5.0, 4.0, 1.0)
            .rotate(0.0, 12.0, 0.0)
            .translate(9.5, -10.0, 2.8);
    target + control + bubble_outlets
}

fn waste_headspace_and_vent(_p: CartridgeParams) -> Part {
    let waste_floor =
        centered_cube("terminal_waste_chamber_floor", 36.0, 30.0, 0.6).translate(55.0, -7.0, 1.45);
    let waste_wall = rectangular_ring(
        "sealed_terminal_waste_chamber_wall",
        40.0,
        34.0,
        2.0,
        5.0,
        3.7,
    )
    .translate(55.0, -7.0, 0.0);
    let entry_gate = one_way_gate(5, 37.0, -3.0, 3.0);

    let pad =
        (0..6).fold(
            Part::empty("low_shedding_absorbent_pad_wick_grid"),
            |part, index| {
                part + centered_cube(format!("confined_pad_wick_{index}"), 28.0, 1.5, 0.8)
                    .translate(55.0, -17.0 + index as f64 * 4.0, 2.3)
            },
        );
    let pad_retainers = centered_cube("pad_retainer_left", 2.0, 27.0, 2.3)
        .translate(39.5, -7.0, 2.8)
        + centered_cube("pad_retainer_right", 2.0, 27.0, 2.3).translate(70.5, -7.0, 2.8)
        + centered_cube("pad_retainer_front", 33.0, 2.0, 2.3).translate(55.0, -20.5, 2.8)
        + centered_cube("pad_retainer_rear", 33.0, 2.0, 2.3).translate(55.0, 6.5, 2.8);

    let headspace_frame = rectangular_ring(
        "reserved_noncompressive_gas_headspace",
        34.0,
        28.0,
        2.0,
        0.7,
        5.8,
    )
    .translate(55.0, -7.0, 0.0);
    let baffles = centered_cube("waste_splash_baffle_a", 2.0, 22.0, 3.3).translate(63.0, -8.0, 3.8)
        + centered_cube("waste_splash_baffle_b", 2.0, 18.0, 3.3).translate(68.0, -1.0, 3.8);

    let gas_path = channel_y(
        "physically_separated_gas_only_vent_path",
        68.0,
        17.0,
        20.0,
        1.4,
        0.8,
        5.7,
    ) + channel_x("gas_only_vent_plenum", 62.0, 27.0, 12.0, 1.4, 0.8, 5.7);
    let vent_guard = centered_cylinder("hydrophobic_aerosol_barrier_support", 6.0, 1.4, 8)
        .translate(56.0, 27.0, 5.7);
    let vent_membrane = centered_cylinder("gas_only_hydrophobic_aerosol_barrier_vent", 4.5, 0.7, 8)
        .translate(56.0, 27.0, 6.6);
    let vent_pores = [
        (-2.0, -1.5),
        (0.0, -1.5),
        (2.0, -1.5),
        (-1.0, 1.0),
        (1.0, 1.0),
    ]
    .into_iter()
    .enumerate()
    .fold(
        Part::empty("vent_visual_pore_witnesses"),
        |part, (index, (dx, dy))| {
            part + centered_cube(format!("vent_pore_witness_{index}"), 0.5, 0.5, 0.6).translate(
                56.0 + dx,
                27.0 + dy,
                7.2,
            )
        },
    );

    waste_floor
        + waste_wall
        + entry_gate
        + pad
        + pad_retainers
        + headspace_frame
        + baffles
        + gas_path
        + vent_guard
        + vent_membrane
        + vent_pores
}

fn reader_interfaces(_p: CartridgeParams) -> Part {
    let lysis_thermal = centered_cube("reader_lysis_thermal_contact", 26.0, 18.0, 0.8)
        .translate(-36.0, -10.0, -1.4);
    let target_thermal = centered_cube(
        "reader_target_amplification_thermal_contact",
        22.0,
        10.0,
        0.8,
    )
    .translate(20.0, 4.0, -1.4);
    let control_thermal = centered_cube(
        "reader_control_amplification_thermal_contact",
        22.0,
        10.0,
        0.8,
    )
    .translate(20.0, -10.0, -1.4);
    let optical_frames = rectangular_ring(
        "target_camera_led_alignment_frame",
        24.0,
        12.0,
        2.0,
        1.0,
        4.2,
    )
    .translate(20.0, 4.0, 0.0)
        + rectangular_ring(
            "control_camera_led_alignment_frame",
            24.0,
            12.0,
            2.0,
            1.0,
            4.2,
        )
        .translate(20.0, -10.0, 0.0);
    let clamp_features = centered_cube("reader_clamp_datum_left", 10.0, 3.0, 3.0)
        .translate(-30.0, -43.0, 1.0)
        + centered_cube("reader_clamp_datum_right", 10.0, 3.0, 3.0).translate(32.0, -43.0, 1.0);
    lysis_thermal + target_thermal + control_thermal + optical_frames + clamp_features
}

fn geometric_identity_marks(_p: CartridgeParams) -> Part {
    let revision = (0..5).fold(
        Part::empty("revision_p1_int_r0_geometry_code"),
        |part, index| {
            part + centered_cube(
                format!("revision_bar_{index}"),
                0.8,
                5.0 + index as f64,
                0.7,
            )
            .translate(-2.0 + index as f64 * 2.0, -35.0, 1.35)
        },
    );
    let flow_direction = (0..4).fold(
        Part::empty("irreversible_flow_direction_witness"),
        |part, index| {
            part + centered_cube(format!("forward_flow_tick_{index}"), 3.0, 0.7, 0.7)
                .rotate(0.0, 0.0, 45.0)
                .translate(-29.0 + index as f64 * 18.0, 15.0, 1.35)
        },
    );
    revision + flow_direction
}

fn dry_reagent_region(name: &str, x: f64, y: f64) -> Part {
    rectangular_ring(
        format!("{name}_sealed_vestibule"),
        10.0,
        8.0,
        1.5,
        0.8,
        2.75,
    )
    .translate(x, y, 0.0)
        + centered_cube(format!("{name}_dry_deposit_witness"), 2.5, 2.5, 1.5).translate(x, y, 3.3)
}

fn optical_chamber(name: &str, x: f64, y: f64) -> Part {
    let chamber =
        centered_cube(format!("{name}_clear_window"), 20.0, 8.0, 0.35).translate(x, y, 2.75);
    let seal = rectangular_ring(
        format!("{name}_isolating_seal"),
        24.0,
        12.0,
        2.0,
        0.65,
        2.45,
    )
    .translate(x, y, 0.0);
    chamber + seal
}

fn one_way_gate(index: usize, x: f64, y: f64, z: f64) -> Part {
    centered_cube(
        format!("pressure_stage_{index}_burst_throat"),
        1.0,
        4.0,
        1.2,
    )
    .translate(x, y, z)
        + centered_cube(
            format!("pressure_stage_{index}_forward_flap"),
            3.0,
            0.8,
            1.0,
        )
        .rotate(0.0, 0.0, 22.0)
        .translate(x + 1.6, y, z + 0.4)
        + centered_cube(
            format!("pressure_stage_{index}_reverse_stop"),
            1.2,
            1.2,
            1.6,
        )
        .translate(x - 1.2, y, z + 0.6)
}

fn channel_x(
    name: &str,
    center_x: f64,
    y: f64,
    length: f64,
    width: f64,
    height: f64,
    z: f64,
) -> Part {
    centered_cube(name, length, width, height).translate(center_x, y, z)
}

fn channel_y(
    name: &str,
    x: f64,
    center_y: f64,
    length: f64,
    width: f64,
    height: f64,
    z: f64,
) -> Part {
    centered_cube(name, width, length, height).translate(x, center_y, z)
}

fn rounded_cell(name: &str, length: f64, width: f64, height: f64, _segments: u32) -> Part {
    // A faceted cell envelope keeps the triangulated STEP publication bounded.
    // Dose math is defined by the parametric internal dimensions, not this
    // raised opaque review witness.
    centered_cube(format!("{name}_faceted_envelope"), length, width, height)
}

fn rectangular_ring(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    wall: f64,
    height: f64,
    z: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height).translate(0.0, 0.0, z)
        - centered_cube(
            format!("{name}_inner"),
            outer_x - 2.0 * wall,
            outer_y - 2.0 * wall,
            height + 0.2,
        )
        .translate(0.0, 0.0, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_dose_and_chamber_envelopes_are_deterministic() {
        let p = CartridgeParams::default();
        for (actual, expected) in [
            (p.wash_pouch_nominal_ul(), 600.0),
            (p.reaction_pouch_lobe_nominal_ul(), 22.4),
            (p.amplification_chamber_nominal_ul(), 22.4),
        ] {
            assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
        }
    }

    #[test]
    fn waste_capacity_and_headspace_gates_are_met() {
        let p = CartridgeParams::default();
        assert!(p.proposed_pad_retained_capacity_ul >= 2.0 * p.maximum_delivered_liquid_ul());
        assert!(p.reserved_headspace_fraction() >= 0.20);
        assert!(p.waste_internal_volume_ul() > p.maximum_delivered_liquid_ul());
    }

    #[test]
    fn topology_is_one_way_and_vent_is_gas_only() {
        verify_topology().unwrap();
        assert!(FLOW_EDGES
            .iter()
            .filter(|edge| edge.to == "hydrophobic_aerosol_barrier")
            .all(|edge| edge.kind == EdgeKind::GasOnly));
    }

    #[test]
    fn architecture_has_exactly_two_dry_actuators() {
        let p = CartridgeParams::default();
        assert_eq!(p.plunger_count, 2);
        assert_eq!(p.optical_chamber_count, 2);
    }

    #[test]
    fn proposed_pressure_hierarchy_has_required_margin() {
        assert!(PRESSURE_STAGE_KPA.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(PRESSURE_STAGE_KPA[4] <= PROPOSED_MINIMUM_DESTRUCTIVE_PRESSURE_KPA / 2.0);
    }

    #[test]
    fn complete_default_design_passes_all_invariants() {
        verify_design(CartridgeParams::default()).unwrap();
    }
}
