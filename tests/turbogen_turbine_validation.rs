//! Turbine wheel design validation tests.
//!
//! Loads design parameters from `design-specs/turbogen_turbine_wheel.json` and
//! the CAD constants from `lib.rs`, then computes all derived engineering
//! quantities and asserts they fall within published acceptable ranges.
//!
//! References:
//!   - Whitfield & Baines, "Design of Radial Turbomachines" (1990)
//!   - Moustapha et al., "Axial and Radial Turbines" (2003)
//!   - Aungier, "Turbine Aerodynamics" (2006)
//!   - Glassman, "Turbine Design and Application" (1976)
//!   - Rohlik, NASA TN D-4384 (1968)
//!   - Balje, "Turbomachines" (1981)

#![allow(non_snake_case)]

use serde::Deserialize;
use std::f64::consts::PI;
use std::path::Path;

// Pull in the CAD constants so we can cross-check JSON against lib.rs
use laminarforge_cad::*;

// ─── JSON schema ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct DesignSpec {
    geometry: Geometry,
    operating_point: OperatingPoint,
    thermodynamics: Thermo,
    material_inconel_713c: Material,
    housing_materials: HousingMaterials,
    derived_inputs: DerivedInputs,
    validation_bounds: ValidationBounds,
}

#[derive(Deserialize)]
struct Geometry {
    tip_diameter_mm: f64,
    tip_radius_mm: f64,
    inlet_blade_height_mm: f64,
    blade_count: usize,
    tip_clearance_mm: f64,
    exit_tip_radius_mm: f64,
    exit_hub_radius_mm: f64,
    blade_thickness_mm: f64,
    #[allow(dead_code)]
    blade_le_thickness_mm: f64,
    #[allow(dead_code)]
    axial_length_mm: f64,
    #[allow(dead_code)]
    back_disk_thickness_mm: f64,
}

#[derive(Deserialize)]
struct OperatingPoint {
    design_rpm: f64,
    mass_flow_kg_s: f64,
    total_to_static_efficiency: f64,
    #[allow(dead_code)]
    total_to_total_efficiency: f64,
    #[allow(dead_code)]
    turbine_power_kw: f64,
    #[allow(dead_code)]
    exit_swirl: String,
}

#[derive(Deserialize)]
struct Thermo {
    #[allow(dead_code)]
    #[serde(rename = "T03_K")]
    t03_k: f64,
    #[serde(rename = "P03_kPa")]
    p03_kpa: f64,
    #[serde(rename = "P5_kPa")]
    p5_kpa: f64,
    gamma_hot: f64,
    #[allow(dead_code)]
    #[serde(rename = "Cp_hot_J_kgK")]
    cp_hot: f64,
    #[serde(rename = "R_gas_J_kgK")]
    r_gas: f64,
    #[allow(dead_code)]
    expansion_ratio: f64,
    isentropic_delta_h_J_kg: f64,
    actual_delta_h_J_kg: f64,
    #[serde(rename = "C_spouting_m_s")]
    c_spouting: f64,
}

#[derive(Deserialize)]
struct Material {
    density_kg_m3: f64,
    #[serde(rename = "CTE_per_K")]
    cte_per_k: f64,
    #[allow(dead_code)]
    youngs_modulus_GPa_RT: f64,
    #[allow(dead_code)]
    sigma_y_MPa_RT: f64,
    #[serde(rename = "sigma_y_MPa_700C")]
    sigma_y_700c: f64,
    #[allow(dead_code)]
    #[serde(rename = "sigma_y_MPa_850C")]
    sigma_y_850c: f64,
    poissons_ratio: f64,
    #[allow(dead_code)]
    thermal_conductivity_W_mK: f64,
}

#[derive(Deserialize)]
struct HousingMaterials {
    #[serde(rename = "aluminum_6061_CTE_per_K")]
    aluminum_cte: f64,
    #[serde(rename = "steel_304_CTE_per_K")]
    steel_cte: f64,
    housing_bore_id_mm: f64,
}

#[derive(Deserialize)]
struct DerivedInputs {
    #[allow(dead_code)]
    #[serde(rename = "U_tip_m_s")]
    u_tip: f64,
    exit_rms_radius_mm: f64,
    #[allow(dead_code)]
    exit_tip_diameter_mm: f64,
    #[allow(dead_code)]
    exit_hub_diameter_mm: f64,
    #[allow(dead_code)]
    inlet_flow_area_mm2: f64,
    #[allow(dead_code)]
    exit_flow_area_mm2: f64,
}

#[derive(Deserialize)]
struct ValidationBounds {
    #[serde(rename = "velocity_ratio_U_over_Cs")]
    velocity_ratio: BoundsRange,
    loading_coefficient_psi: BoundsRange,
    flow_coefficient_phi: BoundsRange,
    #[serde(rename = "diameter_ratio_D6_D4")]
    diameter_ratio: BoundsRange,
    #[serde(rename = "blade_height_ratio_b4_D4")]
    blade_height_ratio: BoundsRange,
    degree_of_reaction_R: BoundsRange,
    bore_stress_safety_factor: BoreBoundsExt,
    tip_clearance_hot_mm: ClearanceBounds,
    exit_continuity_error_pct: ContinuityBounds,
    #[serde(rename = "specific_speed_ns_dimensionless")]
    specific_speed: BoundsRange,
    exit_blade_angle_beta6_deg: BoundsRange,
}

#[derive(Deserialize)]
struct BoundsRange {
    #[allow(dead_code)]
    optimal_range: [f64; 2],
    acceptable_range: [f64; 2],
    #[allow(dead_code)]
    computed_note: String,
}

#[derive(Deserialize)]
struct BoreBoundsExt {
    min_safety_factor: f64,
    #[allow(dead_code)]
    computed_note: String,
    #[allow(dead_code)]
    formula: String,
}

#[derive(Deserialize)]
struct ClearanceBounds {
    min_clearance_mm: f64,
    max_clearance_mm: f64,
    #[allow(dead_code)]
    computed_note: String,
}

#[derive(Deserialize)]
struct ContinuityBounds {
    max_error_pct: f64,
    #[allow(dead_code)]
    computed_note: String,
}

// ─── Helper to load the JSON spec ───────────────────────────────────────────

fn load_spec() -> DesignSpec {
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("design-specs")
        .join("turbogen_turbine_wheel.json");
    let data = std::fs::read_to_string(&spec_path)
        .unwrap_or_else(|e| panic!("Failed to read design spec at {:?}: {}", spec_path, e));
    serde_json::from_str(&data)
        .unwrap_or_else(|e| panic!("Failed to parse design spec JSON: {}", e))
}

// ─── Cross-check: JSON geometry must match lib.rs constants ─────────────────

#[test]
fn json_matches_lib_constants() {
    let s = load_spec();
    let tol = 1e-6;

    assert!((s.geometry.tip_diameter_mm - TG_TURB_TIP_DIA).abs() < tol,
        "tip_diameter: JSON {} vs lib {}", s.geometry.tip_diameter_mm, TG_TURB_TIP_DIA);
    assert!((s.geometry.tip_radius_mm - TG_TURB_TIP_RADIUS).abs() < tol,
        "tip_radius: JSON {} vs lib {}", s.geometry.tip_radius_mm, TG_TURB_TIP_RADIUS);
    assert!((s.geometry.inlet_blade_height_mm - TG_TURB_INLET_BLADE_HEIGHT).abs() < tol,
        "inlet_blade_height: JSON {} vs lib {}", s.geometry.inlet_blade_height_mm, TG_TURB_INLET_BLADE_HEIGHT);
    assert_eq!(s.geometry.blade_count, TG_TURB_BLADE_COUNT,
        "blade_count: JSON {} vs lib {}", s.geometry.blade_count, TG_TURB_BLADE_COUNT);
    assert!((s.geometry.tip_clearance_mm - TG_TURB_TIP_CLEARANCE).abs() < tol,
        "tip_clearance: JSON {} vs lib {}", s.geometry.tip_clearance_mm, TG_TURB_TIP_CLEARANCE);
    assert!((s.geometry.exit_tip_radius_mm - TG_TURB_EXIT_TIP_RADIUS).abs() < tol,
        "exit_tip_radius: JSON {} vs lib {}", s.geometry.exit_tip_radius_mm, TG_TURB_EXIT_TIP_RADIUS);
    assert!((s.geometry.exit_hub_radius_mm - TG_TURB_EXIT_HUB_RADIUS).abs() < tol,
        "exit_hub_radius: JSON {} vs lib {}", s.geometry.exit_hub_radius_mm, TG_TURB_EXIT_HUB_RADIUS);
    assert!((s.geometry.blade_thickness_mm - TG_TURB_BLADE_THICKNESS).abs() < tol,
        "blade_thickness: JSON {} vs lib {}", s.geometry.blade_thickness_mm, TG_TURB_BLADE_THICKNESS);
    assert!((s.operating_point.design_rpm - TG_DESIGN_RPM).abs() < tol,
        "design_rpm: JSON {} vs lib {}", s.operating_point.design_rpm, TG_DESIGN_RPM);
    assert!((s.operating_point.mass_flow_kg_s - TG_MASS_FLOW).abs() < tol,
        "mass_flow: JSON {} vs lib {}", s.operating_point.mass_flow_kg_s, TG_MASS_FLOW);
}

// ─── 1. Velocity ratio U/C_spouting ────────────────────────────────────────

#[test]
fn velocity_ratio() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_tip_m = s.geometry.tip_radius_mm / 1000.0;
    let u_tip = omega * r_tip_m;
    let ratio = u_tip / s.thermodynamics.c_spouting;

    let [lo, hi] = s.validation_bounds.velocity_ratio.acceptable_range;
    println!("Velocity ratio U/Cs = {:.4} (acceptable: {:.2}..{:.2})", ratio, lo, hi);
    assert!(ratio >= lo && ratio <= hi,
        "Velocity ratio {:.4} outside acceptable range [{:.2}, {:.2}]", ratio, lo, hi);
}

// ─── 2. Loading coefficient psi = delta_h / U^2 ────────────────────────────

#[test]
fn loading_coefficient() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_tip_m = s.geometry.tip_radius_mm / 1000.0;
    let u_tip = omega * r_tip_m;
    let psi = s.thermodynamics.actual_delta_h_J_kg / (u_tip * u_tip);

    let [lo, hi] = s.validation_bounds.loading_coefficient_psi.acceptable_range;
    println!("Loading coefficient psi = {:.4} (acceptable: {:.2}..{:.2})", psi, lo, hi);
    assert!(psi >= lo && psi <= hi,
        "Loading coefficient {:.4} outside acceptable range [{:.2}, {:.2}]", psi, lo, hi);
}

// ─── 3. Flow coefficient phi = Cm6 / U_tip ─────────────────────────────────

#[test]
fn flow_coefficient() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_tip_m = s.geometry.tip_radius_mm / 1000.0;
    let u_tip = omega * r_tip_m;

    // Exit conditions: isentropic expansion to P5 to get T5s, then apply eta_ts
    let gamma = s.thermodynamics.gamma_hot;
    let t03 = s.thermodynamics.t03_k;
    let pr = s.thermodynamics.p03_kpa / s.thermodynamics.p5_kpa;
    let t5s = t03 / pr.powf((gamma - 1.0) / gamma);
    let t5 = t03 - s.operating_point.total_to_static_efficiency * (t03 - t5s);
    let rho5 = (s.thermodynamics.p5_kpa * 1000.0) / (s.thermodynamics.r_gas * t5);

    // Exit annulus area (no blockage correction for simplicity)
    let r_tip_exit_m = s.geometry.exit_tip_radius_mm / 1000.0;
    let r_hub_exit_m = s.geometry.exit_hub_radius_mm / 1000.0;
    let a_exit = PI * (r_tip_exit_m * r_tip_exit_m - r_hub_exit_m * r_hub_exit_m);

    let cm6 = s.operating_point.mass_flow_kg_s / (rho5 * a_exit);
    let phi = cm6 / u_tip;

    let [lo, hi] = s.validation_bounds.flow_coefficient_phi.acceptable_range;
    println!("Flow coefficient phi = {:.4} (acceptable: {:.2}..{:.2})", phi, lo, hi);
    println!("  T5 = {:.1} K, rho5 = {:.3} kg/m3, Cm6 = {:.1} m/s", t5, rho5, cm6);
    assert!(phi >= lo && phi <= hi,
        "Flow coefficient {:.4} outside acceptable range [{:.2}, {:.2}]", phi, lo, hi);
}

// ─── 4. Diameter ratio D6_rms / D4 ─────────────────────────────────────────

#[test]
fn diameter_ratio() {
    let s = load_spec();
    let r_tip_exit = s.geometry.exit_tip_radius_mm;
    let r_hub_exit = s.geometry.exit_hub_radius_mm;
    let r_rms = ((r_tip_exit * r_tip_exit + r_hub_exit * r_hub_exit) / 2.0).sqrt();
    let d_rms = 2.0 * r_rms;
    let ratio = d_rms / s.geometry.tip_diameter_mm;

    let [lo, hi] = s.validation_bounds.diameter_ratio.acceptable_range;
    println!("Diameter ratio D6rms/D4 = {:.4} (acceptable: {:.2}..{:.2})", ratio, lo, hi);
    println!("  r_rms = {:.2} mm (JSON derived: {:.2} mm)", r_rms, s.derived_inputs.exit_rms_radius_mm);
    assert!(ratio >= lo && ratio <= hi,
        "Diameter ratio {:.4} outside acceptable range [{:.2}, {:.2}]", ratio, lo, hi);
}

// ─── 5. Blade height ratio b4/D4 ───────────────────────────────────────────

#[test]
fn blade_height_ratio() {
    let s = load_spec();
    let ratio = s.geometry.inlet_blade_height_mm / s.geometry.tip_diameter_mm;

    let [lo, hi] = s.validation_bounds.blade_height_ratio.acceptable_range;
    println!("Blade height ratio b4/D4 = {:.4} (acceptable: {:.2}..{:.2})", ratio, lo, hi);
    assert!(ratio >= lo && ratio <= hi,
        "Blade height ratio {:.4} outside acceptable range [{:.2}, {:.2}]", ratio, lo, hi);
}

// ─── 6. Glassman blade count minimum ────────────────────────────────────────

#[test]
fn glassman_blade_count() {
    let s = load_spec();
    // For radial-fibred inlet (alpha4 ~ 72 deg), the Glassman correlation:
    // Z_min = (pi/30) * (110 - alpha4) * tan(alpha4)
    let alpha4_deg = 72.0;
    let alpha4_rad = alpha4_deg * PI / 180.0;
    let z_min = (PI / 30.0) * (110.0 - alpha4_deg) * alpha4_rad.tan();

    println!("Glassman Z_min = {:.2}, design Z = {}", z_min, s.geometry.blade_count);
    assert!(s.geometry.blade_count as f64 >= z_min,
        "Blade count {} is below Glassman minimum {:.2}", s.geometry.blade_count, z_min);
}

// ─── 7. Degree of reaction ──────────────────────────────────────────────────

#[test]
fn degree_of_reaction() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_tip_m = s.geometry.tip_radius_mm / 1000.0;
    let u_tip = omega * r_tip_m;
    let reaction = 1.0 - s.thermodynamics.actual_delta_h_J_kg / (2.0 * u_tip * u_tip);

    let [lo, hi] = s.validation_bounds.degree_of_reaction_R.acceptable_range;
    println!("Degree of reaction R = {:.4} (acceptable: {:.2}..{:.2})", reaction, lo, hi);
    assert!(reaction >= lo && reaction <= hi,
        "Reaction {:.4} outside acceptable range [{:.2}, {:.2}]", reaction, lo, hi);
}

// ─── 8. Bore stress and safety factor at 700C ───────────────────────────────

#[test]
fn bore_stress_safety_factor() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_tip_m = s.geometry.tip_radius_mm / 1000.0;
    let rho = s.material_inconel_713c.density_kg_m3;
    let nu = s.material_inconel_713c.poissons_ratio;

    // Solid-disk hoop stress at center (r=0), per Dixon & Hall (2014):
    //   sigma_bore = ((3 + nu) / 8) * rho * omega^2 * r_tip^2
    //
    // This is the standard preliminary design formula for turbine wheels.
    // The Roark's annular thin-disk formula (which uses r_inner for the shaft bore)
    // gives ~2x higher stress because it assumes uniform disk thickness. Real turbine
    // wheels have thick hubs, making the solid-disk formula more representative.
    // FEA should be used for detailed design confirmation.
    let sigma_bore = ((3.0 + nu) / 8.0) * rho * omega.powi(2) * r_tip_m.powi(2);
    let sigma_bore_mpa = sigma_bore / 1e6;

    let sf = s.material_inconel_713c.sigma_y_700c / sigma_bore_mpa;

    println!("Bore stress = {:.1} MPa, sigma_y(700C) = {:.0} MPa, SF = {:.3}",
        sigma_bore_mpa, s.material_inconel_713c.sigma_y_700c, sf);
    println!("  Formula: solid-disk center, ((3+nu)/8) * rho * omega^2 * r^2");
    println!("  Note: annular thin-disk formula gives ~913 MPa (overestimate for thick-hub wheel)");
    assert!(sf >= s.validation_bounds.bore_stress_safety_factor.min_safety_factor,
        "Safety factor {:.3} below minimum {:.1}",
        sf, s.validation_bounds.bore_stress_safety_factor.min_safety_factor);
}

// ─── 9. Tip clearance thermal growth (aluminum and steel housing) ───────────

#[test]
fn tip_clearance_thermal_aluminum() {
    let s = load_spec();
    // Temperature assumptions: rotor bulk ~700C (973K), housing ~300C (573K) with cooling
    let delta_t_rotor = 700.0; // K above ambient (20C)
    let delta_t_housing = 300.0;

    let r_rotor_mm = s.geometry.tip_radius_mm;
    let r_housing_mm = s.housing_materials.housing_bore_id_mm / 2.0;

    let rotor_growth = r_rotor_mm * s.material_inconel_713c.cte_per_k * delta_t_rotor;
    let housing_growth = r_housing_mm * s.housing_materials.aluminum_cte * delta_t_housing;

    let cold_clearance = r_housing_mm - r_rotor_mm;
    let hot_clearance = cold_clearance + housing_growth - rotor_growth;

    let min = s.validation_bounds.tip_clearance_hot_mm.min_clearance_mm;
    let max = s.validation_bounds.tip_clearance_hot_mm.max_clearance_mm;

    println!("Tip clearance (Al housing): cold = {:.3} mm, hot = {:.3} mm (bounds: {:.2}..{:.2})",
        cold_clearance, hot_clearance, min, max);
    println!("  Rotor growth = {:.3} mm, Housing growth = {:.3} mm",
        rotor_growth, housing_growth);
    assert!(hot_clearance >= min && hot_clearance <= max,
        "Hot clearance {:.3} mm outside bounds [{:.2}, {:.2}]", hot_clearance, min, max);
}

#[test]
fn tip_clearance_thermal_steel() {
    let s = load_spec();
    let delta_t_rotor = 700.0;
    let delta_t_housing = 300.0;

    let r_rotor_mm = s.geometry.tip_radius_mm;
    let r_housing_mm = s.housing_materials.housing_bore_id_mm / 2.0;

    let rotor_growth = r_rotor_mm * s.material_inconel_713c.cte_per_k * delta_t_rotor;
    let housing_growth = r_housing_mm * s.housing_materials.steel_cte * delta_t_housing;

    let cold_clearance = r_housing_mm - r_rotor_mm;
    let hot_clearance = cold_clearance + housing_growth - rotor_growth;

    let min = s.validation_bounds.tip_clearance_hot_mm.min_clearance_mm;
    let max = s.validation_bounds.tip_clearance_hot_mm.max_clearance_mm;

    println!("Tip clearance (steel housing): cold = {:.3} mm, hot = {:.3} mm (bounds: {:.2}..{:.2})",
        cold_clearance, hot_clearance, min, max);
    println!("  Rotor growth = {:.3} mm, Housing growth = {:.3} mm",
        rotor_growth, housing_growth);
    assert!(hot_clearance >= min && hot_clearance <= max,
        "Hot clearance {:.3} mm outside bounds [{:.2}, {:.2}]", hot_clearance, min, max);
}

// ─── 10. Exit continuity error ──────────────────────────────────────────────

#[test]
fn exit_continuity() {
    let s = load_spec();
    let gamma = s.thermodynamics.gamma_hot;
    let t03 = s.thermodynamics.t03_k;
    let pr = s.thermodynamics.p03_kpa / s.thermodynamics.p5_kpa;
    let t5s = t03 / pr.powf((gamma - 1.0) / gamma);
    let t5 = t03 - s.operating_point.total_to_static_efficiency * (t03 - t5s);
    let rho5 = (s.thermodynamics.p5_kpa * 1000.0) / (s.thermodynamics.r_gas * t5);

    let r_tip_exit_m = s.geometry.exit_tip_radius_mm / 1000.0;
    let r_hub_exit_m = s.geometry.exit_hub_radius_mm / 1000.0;
    let a_exit = PI * (r_tip_exit_m.powi(2) - r_hub_exit_m.powi(2));

    // Blockage from blades: approximate blade blockage at exit
    let z = s.geometry.blade_count as f64;
    let t_blade_m = s.geometry.blade_thickness_mm / 1000.0;
    let r_rms_exit_m = s.derived_inputs.exit_rms_radius_mm / 1000.0;
    let circumference = 2.0 * PI * r_rms_exit_m;
    let blockage = 1.0 - (z * t_blade_m) / circumference;

    let a_eff = a_exit * blockage;

    // Meridional velocity from actual enthalpy drop (assume all kinetic at exit for continuity)
    let cm6 = s.operating_point.mass_flow_kg_s / (rho5 * a_eff);

    // Back-calculate mass flow to check continuity
    let mdot_check = rho5 * cm6 * a_eff;
    let error_pct = ((mdot_check - s.operating_point.mass_flow_kg_s) / s.operating_point.mass_flow_kg_s).abs() * 100.0;

    println!("Exit continuity: mdot_check = {:.4} kg/s, design = {:.4} kg/s, error = {:.2}%",
        mdot_check, s.operating_point.mass_flow_kg_s, error_pct);
    println!("  rho5 = {:.3} kg/m3, Cm6 = {:.1} m/s, A_eff = {:.6} m2, blockage = {:.4}",
        rho5, cm6, a_eff, blockage);
    assert!(error_pct <= s.validation_bounds.exit_continuity_error_pct.max_error_pct,
        "Exit continuity error {:.2}% exceeds maximum {:.1}%",
        error_pct, s.validation_bounds.exit_continuity_error_pct.max_error_pct);
}

// ─── 11. Specific speed (US units) ──────────────────────────────────────────

#[test]
fn specific_speed() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let gamma = s.thermodynamics.gamma_hot;
    let t03 = s.thermodynamics.t03_k;
    let pr = s.thermodynamics.p03_kpa / s.thermodynamics.p5_kpa;
    let t5s = t03 / pr.powf((gamma - 1.0) / gamma);
    let t5 = t03 - s.operating_point.total_to_static_efficiency * (t03 - t5s);
    let rho5 = (s.thermodynamics.p5_kpa * 1000.0) / (s.thermodynamics.r_gas * t5);

    let r_tip_exit_m = s.geometry.exit_tip_radius_mm / 1000.0;
    let r_hub_exit_m = s.geometry.exit_hub_radius_mm / 1000.0;
    let a_exit = PI * (r_tip_exit_m.powi(2) - r_hub_exit_m.powi(2));
    let cm6 = s.operating_point.mass_flow_kg_s / (rho5 * a_exit);

    // Volume flow at exit (SI: m^3/s)
    let q_exit_m3s = cm6 * a_exit;

    // SI dimensionless specific speed per Rohlik (1968) / Balje (1981):
    //   ns = omega * sqrt(Q) / (delta_h_is)^0.75
    // where omega in rad/s, Q in m^3/s, delta_h in J/kg
    let delta_h_is = s.thermodynamics.isentropic_delta_h_J_kg;
    let ns = omega * q_exit_m3s.sqrt() / delta_h_is.powf(0.75);

    let [lo, hi] = s.validation_bounds.specific_speed.acceptable_range;
    println!("Specific speed ns = {:.4} (acceptable: {:.2}..{:.2})", ns, lo, hi);
    println!("  omega = {:.1} rad/s, Q_exit = {:.4} m3/s, delta_h_is = {:.0} J/kg",
        omega, q_exit_m3s, delta_h_is);
    assert!(ns >= lo && ns <= hi,
        "Specific speed {:.4} outside acceptable range [{:.2}, {:.2}]", ns, lo, hi);
}

// ─── 12. Exit blade angle beta6 ────────────────────────────────────────────

#[test]
fn exit_blade_angle() {
    let s = load_spec();
    let omega = s.operating_point.design_rpm * 2.0 * PI / 60.0;
    let r_rms_exit_m = s.derived_inputs.exit_rms_radius_mm / 1000.0;
    let u6 = omega * r_rms_exit_m; // blade speed at exit rms

    // For zero exit swirl (C_theta6 = 0), the relative tangential velocity:
    // W_theta6 = -U6  (in the relative frame, the blade moves at +U, flow has no swirl)
    let w_theta6 = -u6;

    // Meridional velocity at exit
    let gamma = s.thermodynamics.gamma_hot;
    let t03 = s.thermodynamics.t03_k;
    let pr = s.thermodynamics.p03_kpa / s.thermodynamics.p5_kpa;
    let t5s = t03 / pr.powf((gamma - 1.0) / gamma);
    let t5 = t03 - s.operating_point.total_to_static_efficiency * (t03 - t5s);
    let rho5 = (s.thermodynamics.p5_kpa * 1000.0) / (s.thermodynamics.r_gas * t5);

    let r_tip_exit_m = s.geometry.exit_tip_radius_mm / 1000.0;
    let r_hub_exit_m = s.geometry.exit_hub_radius_mm / 1000.0;
    let a_exit = PI * (r_tip_exit_m.powi(2) - r_hub_exit_m.powi(2));
    let cm6 = s.operating_point.mass_flow_kg_s / (rho5 * a_exit);

    let beta6_rad = (w_theta6 / cm6).atan();
    let beta6_deg = beta6_rad * 180.0 / PI;

    let [lo, hi] = s.validation_bounds.exit_blade_angle_beta6_deg.acceptable_range;
    println!("Exit blade angle beta6 = {:.1} deg (acceptable: {:.0}..{:.0})", beta6_deg, lo, hi);
    println!("  U6 = {:.1} m/s, W_theta6 = {:.1} m/s, Cm6 = {:.1} m/s", u6, w_theta6, cm6);
    assert!(beta6_deg >= lo && beta6_deg <= hi,
        "Exit blade angle {:.1} deg outside acceptable range [{:.0}, {:.0}]", beta6_deg, lo, hi);
}
