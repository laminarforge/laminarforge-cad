#![allow(dead_code)]

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = "pcb/lamp_rev_a/electrical_validation.toml";

#[derive(Debug, Clone)]
pub struct ElectricalOutputPaths {
    pub report_md: PathBuf,
    pub gates_csv: PathBuf,
    pub spice_netlist: PathBuf,
    pub simulation_handoff_md: PathBuf,
    pub pdn_current_paths_csv: PathBuf,
    pub thermal_power_csv: PathBuf,
    pub first_article_measurements_csv: PathBuf,
    pub component_derating_csv: PathBuf,
    pub fault_fmea_csv: PathBuf,
    pub emc_esd_csv: PathBuf,
    pub startup_safety_csv: PathBuf,
    pub manufacturing_test_csv: PathBuf,
    pub calibration_readiness_csv: PathBuf,
    pub validation_traceability_csv: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub struct ElectricalValidationSummary {
    pub pass_count: usize,
    pub fail_count: usize,
    pub manual_count: usize,
}

#[derive(Debug, Deserialize)]
struct ValidationConfig {
    package: Package,
    inputs: Inputs,
    outputs: Outputs,
    assumptions: Assumptions,
    rail_budgets: Vec<RailBudget>,
    linear_regulators: Vec<LinearRegulator>,
    protection_checks: Vec<ProtectionCheck>,
    mosfet_checks: Vec<MosfetCheck>,
    trace_current_paths: Vec<TraceCurrentPath>,
    gpio_domain: GpioDomain,
    analog_checks: Vec<AnalogCheck>,
    #[serde(default)]
    spice_exports: Vec<SpiceExport>,
    usb_signal_integrity: UsbSignalIntegrity,
    #[serde(default)]
    external_analysis_handoffs: Vec<ExternalAnalysisHandoff>,
    #[serde(default)]
    manual_first_article_gates: Vec<ManualFirstArticleGate>,
    #[serde(default)]
    first_article_measurements: Vec<FirstArticleMeasurement>,
    #[serde(default)]
    component_deratings: Vec<ComponentDerating>,
    single_fault_policy: SingleFaultPolicy,
    #[serde(default)]
    single_fault_checks: Vec<SingleFaultCheck>,
    emc_esd_policy: EmcEsdPolicy,
    #[serde(default)]
    emc_esd_checks: Vec<EmcEsdCheck>,
    boot_startup_policy: BootStartupPolicy,
    #[serde(default)]
    boot_startup_checks: Vec<BootStartupCheck>,
    manufacturing_test_policy: ManufacturingTestPolicy,
    #[serde(default)]
    manufacturing_test_checks: Vec<ManufacturingTestCheck>,
    calibration_policy: CalibrationPolicy,
    #[serde(default)]
    calibration_checks: Vec<CalibrationCheck>,
    traceability_policy: TraceabilityPolicy,
    #[serde(default)]
    validation_traceability: Vec<ValidationTraceability>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    ticket: String,
    revision: String,
    source_stage: String,
}

#[derive(Debug, Deserialize)]
struct Inputs {
    contract: String,
    parts: String,
    routing_seed: String,
    pin_nets: String,
    firmware_handoff: String,
    placement: String,
    power_architecture: String,
    optical_architecture: String,
}

#[derive(Debug, Deserialize)]
struct Outputs {
    report_md: String,
    gates_csv: String,
    spice_netlist: String,
    simulation_handoff_md: String,
    pdn_current_paths_csv: String,
    thermal_power_csv: String,
    first_article_measurements_csv: String,
    component_derating_csv: String,
    fault_fmea_csv: String,
    emc_esd_csv: String,
    startup_safety_csv: String,
    manufacturing_test_csv: String,
    calibration_readiness_csv: String,
    validation_traceability_csv: String,
}

#[derive(Debug, Deserialize)]
struct Assumptions {
    ambient_c: f64,
    max_board_surface_c: f64,
    allowed_trace_temp_rise_c: f64,
    outer_copper_oz: f64,
    inner_copper_oz: f64,
    trace_current_derating: f64,
    component_current_derating: f64,
    component_voltage_derating: f64,
    component_power_derating: f64,
    continuous_heater_current_ma: u32,
    protected_heater_current_ma: u32,
    first_article_current_limit_ma: u32,
}

#[derive(Debug, Deserialize)]
struct RailBudget {
    rail: String,
    nominal_v: f64,
    min_v: f64,
    max_v: f64,
    expected_continuous_ma: u32,
    source_limit_ma: u32,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct LinearRegulator {
    id: String,
    part_id: String,
    input_rail: String,
    output_rail: String,
    vin_nominal_v: f64,
    vout_nominal_v: f64,
    load_ma: u32,
    current_rating_ma: u32,
    theta_ja_c_per_w: f64,
    max_temp_rise_c: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct ProtectionCheck {
    id: String,
    part_id: String,
    path: String,
    current_ma: u32,
    voltage_v: f64,
    current_rating_ma: u32,
    voltage_rating_v: f64,
    voltage_drop_v: f64,
    max_power_w: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct MosfetCheck {
    id: String,
    part_id: String,
    drain_source_voltage_v: f64,
    continuous_current_ma: u32,
    pulsed_current_ma: u32,
    rds_on_mohm: f64,
    current_rating_ma: u32,
    voltage_rating_v: f64,
    package_theta_ja_c_per_w: f64,
    max_temp_rise_c: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct TraceCurrentPath {
    id: String,
    net: String,
    current_ma: u32,
    min_width_mm: f64,
    #[serde(default)]
    neckdown_min_width_mm: Option<f64>,
    #[serde(default)]
    max_neckdown_length_mm: Option<f64>,
    allowed_layers: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct GpioDomain {
    mcu_reference: String,
    logic_max_v: f64,
    forbidden_high_voltage_nets: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct AnalogCheck {
    id: String,
    net: String,
    max_expected_v: f64,
    absolute_max_v: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct SpiceExport {
    id: String,
    output: String,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct UsbSignalIntegrity {
    dp_net: String,
    dn_net: String,
    min_width_mm: f64,
    max_route_length_mm: f64,
    max_length_skew_mm: f64,
    max_width_mismatch_mm: f64,
    max_vias_per_net: usize,
    allowed_layers: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct ExternalAnalysisHandoff {
    id: String,
    tool_class: String,
    recommended_tools: Vec<String>,
    inputs: Vec<String>,
    exit_criterion: String,
}

#[derive(Debug, Deserialize)]
struct ManualFirstArticleGate {
    id: String,
    description: String,
    pass_criterion: String,
}

#[derive(Debug, Deserialize)]
struct FirstArticleMeasurement {
    id: String,
    stage: String,
    order: u32,
    test_point: String,
    net: String,
    measurement: String,
    #[serde(default)]
    min_v: Option<f64>,
    #[serde(default)]
    max_v: Option<f64>,
    #[serde(default)]
    current_limit_ma: Option<u32>,
    pass_criterion: String,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct ComponentDerating {
    id: String,
    part_id: String,
    stress_class: String,
    #[serde(default)]
    operating_voltage_v: Option<f64>,
    #[serde(default)]
    rated_voltage_v: Option<f64>,
    #[serde(default)]
    max_voltage_utilization: Option<f64>,
    #[serde(default)]
    operating_current_ma: Option<u32>,
    #[serde(default)]
    rated_current_ma: Option<u32>,
    #[serde(default)]
    max_current_utilization: Option<f64>,
    #[serde(default)]
    operating_power_w: Option<f64>,
    #[serde(default)]
    rated_power_w: Option<f64>,
    #[serde(default)]
    max_power_utilization: Option<f64>,
    #[serde(default)]
    estimated_temp_rise_c: Option<f64>,
    #[serde(default)]
    max_temp_rise_c: Option<f64>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct SingleFaultPolicy {
    max_rpn: u32,
    require_detection_for_severity_at_least: u8,
    require_hardware_mitigation_for_severity_at_least: u8,
    require_first_article_for_severity_at_least: u8,
}

#[derive(Debug, Deserialize)]
struct SingleFaultCheck {
    id: String,
    subsystem: String,
    failure_mode: String,
    cause: String,
    local_effect: String,
    system_effect: String,
    severity: u8,
    occurrence: u8,
    detection: u8,
    hardware_mitigations: Vec<String>,
    firmware_mitigations: Vec<String>,
    detection_methods: Vec<String>,
    verification_measurements: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct EmcEsdPolicy {
    min_checks: usize,
    max_risk_score: u32,
    require_protection_for_risk_at_least: u32,
    require_measurement_for_risk_at_least: u32,
    require_external_analysis_for_risk_at_least: u32,
    require_return_path_strategy: bool,
}

#[derive(Debug, Deserialize)]
struct EmcEsdCheck {
    id: String,
    interface: String,
    exposure_class: String,
    nets: Vec<String>,
    risk_score: u32,
    protection_part_ids: Vec<String>,
    return_path_strategy: String,
    grounding_strategy: String,
    verification_methods: Vec<String>,
    verification_measurements: Vec<String>,
    external_analysis: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct BootStartupPolicy {
    min_checks: usize,
    require_all_boot_sensitive_firmware_pins: bool,
    require_evidence_for_boot_sensitive: bool,
    require_measurement_for_safety_critical: bool,
    forbid_active_safety_outputs_at_reset: bool,
}

#[derive(Debug, Deserialize)]
struct BootStartupCheck {
    id: String,
    net: String,
    module_pin: u32,
    function: String,
    expected_power_on_state: String,
    safe_state: String,
    boot_sensitive: bool,
    safety_critical: bool,
    allowed_active_at_reset: bool,
    evidence_ids: Vec<String>,
    verification_methods: Vec<String>,
    verification_measurements: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct ManufacturingTestPolicy {
    min_checks: usize,
    require_test_points: bool,
    require_pass_criterion: bool,
    require_measurement_or_firmware_test: bool,
}

#[derive(Debug, Deserialize)]
struct ManufacturingTestCheck {
    id: String,
    subsystem: String,
    nets: Vec<String>,
    required_test_points: Vec<String>,
    test_stage: String,
    test_method: String,
    firmware_test: String,
    pass_criterion: String,
    verification_measurements: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct CalibrationPolicy {
    min_checks: usize,
    require_control_conditions: bool,
    require_acceptance_criterion: bool,
    require_firmware_dependency: bool,
    require_output_artifacts: bool,
}

#[derive(Debug, Deserialize)]
struct CalibrationCheck {
    id: String,
    subsystem: String,
    calibrated_item: String,
    dependent_nets: Vec<String>,
    preconditions: Vec<String>,
    procedure: String,
    acceptance_criterion: String,
    control_data: Vec<String>,
    required_outputs: Vec<String>,
    verification_measurements: Vec<String>,
    firmware_dependency: String,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct TraceabilityPolicy {
    min_items: usize,
    require_output_artifact: bool,
    require_release_manifest_entry: bool,
    require_ci_step: bool,
    require_acceptance_criterion: bool,
    require_release_blocking: bool,
}

#[derive(Debug, Deserialize)]
struct ValidationTraceability {
    id: String,
    validation_layer: String,
    #[serde(default)]
    electrical_gate_category: Option<String>,
    output_artifact: String,
    release_manifest_entry: String,
    ci_step: String,
    acceptance_criterion: String,
    blocks_release: bool,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct Contract {
    stackup: Stackup,
    rails: Vec<ContractRail>,
    nets: Vec<ContractNet>,
    #[serde(default)]
    net_groups: Vec<ContractNetGroup>,
}

#[derive(Debug, Deserialize)]
struct Stackup {
    outer_copper_oz: f64,
    inner_copper_oz: f64,
}

#[derive(Debug, Deserialize)]
struct ContractRail {
    name: String,
    nominal_voltage_v: f64,
    max_current_ma: u32,
}

#[derive(Debug, Deserialize)]
struct ContractNet {
    name: String,
}

#[derive(Debug, Deserialize)]
struct ContractNetGroup {
    prefix: String,
    count: usize,
}

#[derive(Debug, Deserialize)]
struct PartsManifest {
    selected_parts: Vec<SelectedPart>,
    #[serde(default)]
    external_safety_parts: Vec<ExternalSafetyPart>,
}

#[derive(Debug, Deserialize)]
struct SelectedPart {
    id: String,
    value: String,
    lcsc_part: String,
}

#[derive(Debug, Deserialize)]
struct ExternalSafetyPart {
    id: String,
    value: String,
    role: String,
    installation: String,
}

#[derive(Debug, Deserialize)]
struct RoutingSeed {
    #[serde(default)]
    segments: Vec<RouteSegment>,
}

#[derive(Debug, Deserialize)]
struct RouteSegment {
    net: String,
    layer: String,
    #[serde(default)]
    via_at_ends: bool,
    #[serde(default)]
    via_at_start: bool,
    #[serde(default)]
    via_at_end: bool,
    width_mm: f64,
    start_x_mm: f64,
    start_y_mm: f64,
    end_x_mm: f64,
    end_y_mm: f64,
}

#[derive(Debug, Deserialize)]
struct PinNetManifest {
    assignments: Vec<PinNetAssignment>,
}

#[derive(Debug, Deserialize)]
struct PinNetAssignment {
    reference: String,
    pins: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct FirmwareHandoff {
    mcu: FirmwareMcu,
    module_pins: Vec<FirmwareModulePin>,
}

#[derive(Debug, Deserialize)]
struct FirmwareMcu {
    reference: String,
}

#[derive(Debug, Deserialize)]
struct FirmwareModulePin {
    module_pin: u32,
    net: String,
    firmware_direction: String,
    boot_sensitive: bool,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    #[serde(default)]
    test_points: Vec<TestPoint>,
}

#[derive(Debug, Deserialize)]
struct TestPoint {
    name: String,
    net: String,
    x_mm: f64,
    y_mm: f64,
    side: String,
}

#[derive(Debug)]
struct GateRow {
    category: String,
    item: String,
    measured: String,
    limit: String,
    status: String,
    notes: String,
}

macro_rules! push_gate {
    (
        $rows:expr,
        $errors:expr,
        $category:expr,
        $item:expr,
        $measured:expr,
        $limit:expr,
        $pass:expr,
        $notes:expr $(,)?
    ) => {
        push_gate_row(
            $rows,
            $errors,
            GateSpec::new($category, $item, $measured, $limit, $pass, $notes),
        )
    };
}

struct GateSpec {
    category: String,
    item: String,
    measured: String,
    limit: String,
    pass: bool,
    notes: String,
}

impl GateSpec {
    fn new(
        category: impl Into<String>,
        item: impl Into<String>,
        measured: impl Into<String>,
        limit: impl Into<String>,
        pass: bool,
        notes: impl Into<String>,
    ) -> Self {
        Self {
            category: category.into(),
            item: item.into(),
            measured: measured.into(),
            limit: limit.into(),
            pass,
            notes: notes.into(),
        }
    }
}

pub fn default_output_paths(repo_root: &Path) -> Result<ElectricalOutputPaths, Box<dyn Error>> {
    let config = read_validation_config(repo_root)?;
    Ok(ElectricalOutputPaths {
        report_md: repo_root.join(config.outputs.report_md),
        gates_csv: repo_root.join(config.outputs.gates_csv),
        spice_netlist: repo_root.join(config.outputs.spice_netlist),
        simulation_handoff_md: repo_root.join(config.outputs.simulation_handoff_md),
        pdn_current_paths_csv: repo_root.join(config.outputs.pdn_current_paths_csv),
        thermal_power_csv: repo_root.join(config.outputs.thermal_power_csv),
        first_article_measurements_csv: repo_root
            .join(config.outputs.first_article_measurements_csv),
        component_derating_csv: repo_root.join(config.outputs.component_derating_csv),
        fault_fmea_csv: repo_root.join(config.outputs.fault_fmea_csv),
        emc_esd_csv: repo_root.join(config.outputs.emc_esd_csv),
        startup_safety_csv: repo_root.join(config.outputs.startup_safety_csv),
        manufacturing_test_csv: repo_root.join(config.outputs.manufacturing_test_csv),
        calibration_readiness_csv: repo_root.join(config.outputs.calibration_readiness_csv),
        validation_traceability_csv: repo_root.join(config.outputs.validation_traceability_csv),
    })
}

pub fn validate_default(repo_root: &Path) -> Result<ElectricalValidationSummary, Box<dyn Error>> {
    let outputs = default_output_paths(repo_root)?;
    validate_to_outputs(repo_root, &outputs)
}

pub fn validate_to_outputs(
    repo_root: &Path,
    outputs: &ElectricalOutputPaths,
) -> Result<ElectricalValidationSummary, Box<dyn Error>> {
    let config = read_validation_config(repo_root)?;
    validate_config_header(&config)?;
    ensure_inputs(repo_root, &config.inputs)?;

    let contract = read_toml::<Contract>(&repo_root.join(&config.inputs.contract))?;
    let parts = read_toml::<PartsManifest>(&repo_root.join(&config.inputs.parts))?;
    let routing = read_toml::<RoutingSeed>(&repo_root.join(&config.inputs.routing_seed))?;
    let pin_nets = read_toml::<PinNetManifest>(&repo_root.join(&config.inputs.pin_nets))?;
    let firmware = read_toml::<FirmwareHandoff>(&repo_root.join(&config.inputs.firmware_handoff))?;
    let placement = read_toml::<PlacementPlan>(&repo_root.join(&config.inputs.placement))?;

    let selected_part_ids = selected_part_ids(&parts);
    let contract_rails = contract_rails(&contract);
    let contract_nets = expanded_contract_nets(&contract);

    let mut rows = Vec::new();
    let mut errors = Vec::new();

    validate_assumptions(&config, &contract, &mut rows, &mut errors);
    validate_rail_budgets(
        &config,
        &contract_rails,
        &contract_nets,
        &mut rows,
        &mut errors,
    );
    validate_linear_regulators(&config, &selected_part_ids, &mut rows, &mut errors);
    validate_protection(&config, &selected_part_ids, &mut rows, &mut errors);
    validate_mosfets(&config, &selected_part_ids, &mut rows, &mut errors);
    validate_trace_current_paths(&config, &routing, &mut rows, &mut errors);
    validate_usb_signal_integrity(&config, &routing, &contract_nets, &mut rows, &mut errors);
    validate_gpio_domains(&config, &pin_nets, &firmware, &mut rows, &mut errors);
    validate_analog_ranges(&config, &contract_nets, &mut rows, &mut errors);
    validate_first_article_measurements(&config, &placement, &mut rows, &mut errors);
    validate_component_deratings(&config, &selected_part_ids, &mut rows, &mut errors);
    validate_single_faults(&config, &parts, &mut rows, &mut errors);
    validate_emc_esd_precompliance(&config, &parts, &contract_nets, &mut rows, &mut errors);
    validate_boot_startup_safety(
        &config,
        &parts,
        &contract_nets,
        &firmware,
        &mut rows,
        &mut errors,
    );
    validate_manufacturing_test_coverage(
        &config,
        &contract_nets,
        &placement,
        &mut rows,
        &mut errors,
    );
    validate_calibration_readiness(&config, &contract_nets, &mut rows, &mut errors);
    let gate_categories = rows
        .iter()
        .map(|row| row.category.clone())
        .collect::<BTreeSet<_>>();
    validate_validation_traceability(&config, &gate_categories, &mut rows, &mut errors);
    add_external_analysis_rows(&config, &mut rows);
    add_manual_gate_rows(&config, &parts, &mut rows, &mut errors);

    write_report(&config, outputs, &rows)?;
    write_gates_csv(&outputs.gates_csv, &rows)?;
    write_spice_handoff(&config, outputs)?;
    write_pdn_current_paths_handoff(&config, &routing, outputs)?;
    write_thermal_power_handoff(&config, outputs)?;
    write_first_article_measurements_handoff(&config, &placement, outputs)?;
    write_component_derating_handoff(&config, outputs)?;
    write_fault_fmea_handoff(&config, outputs)?;
    write_emc_esd_handoff(&config, outputs)?;
    write_startup_safety_handoff(&config, outputs)?;
    write_manufacturing_test_handoff(&config, outputs)?;
    write_calibration_readiness_handoff(&config, outputs)?;
    write_validation_traceability_handoff(&config, outputs)?;
    write_simulation_handoff(&config, outputs)?;

    if errors.is_empty() {
        Ok(summarize_rows(&rows))
    } else {
        Err(format!(
            "LAMP Rev A electrical validation failed:\n{}",
            errors.join("\n")
        )
        .into())
    }
}

fn read_validation_config(repo_root: &Path) -> Result<ValidationConfig, Box<dyn Error>> {
    read_toml(&repo_root.join(CONFIG_PATH))
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn validate_config_header(config: &ValidationConfig) -> Result<(), Box<dyn Error>> {
    if config.package.name != "lamp_rev_a_electrical_validation" {
        return Err(format!(
            "unexpected electrical validation package {}",
            config.package.name
        )
        .into());
    }
    if config.package.revision != "Rev A" {
        return Err(format!(
            "unexpected electrical validation revision {}",
            config.package.revision
        )
        .into());
    }
    if config.rail_budgets.is_empty()
        || config.trace_current_paths.is_empty()
        || config.external_analysis_handoffs.is_empty()
        || config.first_article_measurements.is_empty()
        || config.component_deratings.is_empty()
        || config.single_fault_checks.is_empty()
        || config.emc_esd_checks.is_empty()
        || config.boot_startup_checks.is_empty()
        || config.manufacturing_test_checks.is_empty()
        || config.calibration_checks.is_empty()
        || config.validation_traceability.is_empty()
    {
        return Err("electrical validation config is missing required gate groups".into());
    }
    Ok(())
}

fn ensure_inputs(repo_root: &Path, inputs: &Inputs) -> Result<(), Box<dyn Error>> {
    for relative in [
        &inputs.contract,
        &inputs.parts,
        &inputs.routing_seed,
        &inputs.pin_nets,
        &inputs.firmware_handoff,
        &inputs.placement,
        &inputs.power_architecture,
        &inputs.optical_architecture,
    ] {
        let path = repo_root.join(relative);
        if !path.is_file() {
            return Err(format!("electrical validation input missing: {}", path.display()).into());
        }
    }
    Ok(())
}

fn selected_part_ids(parts: &PartsManifest) -> BTreeSet<&str> {
    parts
        .selected_parts
        .iter()
        .map(|part| part.id.as_str())
        .collect()
}

fn contract_rails(contract: &Contract) -> BTreeMap<&str, &ContractRail> {
    contract
        .rails
        .iter()
        .map(|rail| (rail.name.as_str(), rail))
        .collect()
}

fn expanded_contract_nets(contract: &Contract) -> BTreeSet<String> {
    let mut nets = contract
        .nets
        .iter()
        .map(|net| net.name.clone())
        .collect::<BTreeSet<_>>();
    for group in &contract.net_groups {
        for index in 0..group.count {
            nets.insert(format!("{}{}", group.prefix, index));
        }
    }
    nets
}

fn validate_assumptions(
    config: &ValidationConfig,
    contract: &Contract,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    push_gate!(
        rows,
        errors,
        "assumptions",
        "outer copper weight",
        format!("{:.2} oz", config.assumptions.outer_copper_oz),
        format!("{:.2} oz contract", contract.stackup.outer_copper_oz),
        nearly_equal(
            config.assumptions.outer_copper_oz,
            contract.stackup.outer_copper_oz,
        ),
        "Validation assumptions match the Rev A stackup.",
    );
    push_gate!(
        rows,
        errors,
        "assumptions",
        "inner copper weight",
        format!("{:.2} oz", config.assumptions.inner_copper_oz),
        format!("{:.2} oz contract", contract.stackup.inner_copper_oz),
        nearly_equal(
            config.assumptions.inner_copper_oz,
            contract.stackup.inner_copper_oz,
        ),
        "Validation assumptions match the Rev A stackup.",
    );
    push_gate!(
        rows,
        errors,
        "assumptions",
        "first-article USB current limit",
        format!("{} mA", config.assumptions.first_article_current_limit_ma),
        "500 mA or lower",
        config.assumptions.first_article_current_limit_ma <= 500,
        "Bring-up starts from a current-limited USB/electronics rail.",
    );
}

fn validate_rail_budgets(
    config: &ValidationConfig,
    contract_rails: &BTreeMap<&str, &ContractRail>,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for budget in &config.rail_budgets {
        let known_net = contract_nets.contains(&budget.rail);
        push_gate!(
            rows,
            errors,
            "rail budget",
            format!("{} exists in contract", budget.rail),
            if known_net { "present" } else { "missing" },
            "present",
            known_net,
            &budget.notes,
        );
        if let Some(rail) = contract_rails.get(budget.rail.as_str()) {
            push_gate!(
                rows,
                errors,
                "rail budget",
                format!("{} current budget", budget.rail),
                format!("{} mA expected", budget.expected_continuous_ma),
                format!("<= {} mA contract max", rail.max_current_ma),
                budget.expected_continuous_ma <= rail.max_current_ma,
                &budget.notes,
            );
            push_gate!(
                rows,
                errors,
                "rail budget",
                format!("{} source limit", budget.rail),
                format!("{} mA source", budget.source_limit_ma),
                format!("<= {} mA contract max", rail.max_current_ma),
                budget.source_limit_ma <= rail.max_current_ma,
                &budget.notes,
            );
            push_gate!(
                rows,
                errors,
                "rail budget",
                format!("{} nominal voltage", budget.rail),
                format!("{:.2} V", budget.nominal_v),
                format!("{:.2} V contract", rail.nominal_voltage_v),
                (budget.nominal_v - rail.nominal_voltage_v).abs() <= 0.6
                    || budget.rail == "+5V"
                    || budget.rail == "+12V"
                    || budget.rail == "HEATER_SUPPLY"
                    || budget.rail == "HEATER_P",
                &budget.notes,
            );
        }
        push_gate!(
            rows,
            errors,
            "rail budget",
            format!("{} voltage window", budget.rail),
            format!("{:.2}..{:.2} V", budget.min_v, budget.max_v),
            "min < nominal < max",
            budget.min_v < budget.nominal_v && budget.nominal_v <= budget.max_v,
            &budget.notes,
        );
    }
}

fn validate_linear_regulators(
    config: &ValidationConfig,
    selected_part_ids: &BTreeSet<&str>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for regulator in &config.linear_regulators {
        push_gate!(
            rows,
            errors,
            "linear regulator",
            format!("{} selected part", regulator.id),
            regulator.part_id.clone(),
            "selected in parts.toml",
            selected_part_ids.contains(regulator.part_id.as_str()),
            &regulator.notes,
        );
        let load_a = regulator.load_ma as f64 / 1000.0;
        let dissipation_w = (regulator.vin_nominal_v - regulator.vout_nominal_v).max(0.0) * load_a;
        let temp_rise_c = dissipation_w * regulator.theta_ja_c_per_w;
        push_gate!(
            rows,
            errors,
            "linear regulator",
            format!("{} current", regulator.id),
            format!("{} mA", regulator.load_ma),
            format!("<= {} mA rated", regulator.current_rating_ma),
            regulator.load_ma <= regulator.current_rating_ma,
            &regulator.notes,
        );
        push_gate!(
            rows,
            errors,
            "linear regulator",
            format!("{} thermal rise", regulator.id),
            format!("{temp_rise_c:.1} C from {dissipation_w:.3} W"),
            format!("<= {:.1} C", regulator.max_temp_rise_c),
            temp_rise_c <= regulator.max_temp_rise_c,
            &regulator.notes,
        );
    }
}

fn validate_protection(
    config: &ValidationConfig,
    selected_part_ids: &BTreeSet<&str>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for check in &config.protection_checks {
        push_gate!(
            rows,
            errors,
            "protection",
            format!("{} selected part", check.id),
            check.part_id.clone(),
            "selected in parts.toml",
            selected_part_ids.contains(check.part_id.as_str()),
            &check.notes,
        );
        let derated_current =
            check.current_rating_ma as f64 * config.assumptions.component_current_derating;
        push_gate!(
            rows,
            errors,
            "protection",
            format!("{} current derating", check.id),
            format!("{} mA", check.current_ma),
            format!("<= {derated_current:.0} mA derated"),
            check.current_ma as f64 <= derated_current,
            &check.notes,
        );
        let derated_voltage =
            check.voltage_rating_v * config.assumptions.component_voltage_derating;
        push_gate!(
            rows,
            errors,
            "protection",
            format!("{} voltage derating", check.id),
            format!("{:.1} V", check.voltage_v),
            format!("<= {derated_voltage:.1} V derated"),
            check.voltage_v <= derated_voltage,
            &check.notes,
        );
        let power_w = check.voltage_drop_v * check.current_ma as f64 / 1000.0;
        push_gate!(
            rows,
            errors,
            "protection",
            format!("{} dissipation", check.id),
            format!("{power_w:.3} W"),
            format!("<= {:.3} W", check.max_power_w),
            power_w <= check.max_power_w,
            &check.notes,
        );
    }
}

fn validate_mosfets(
    config: &ValidationConfig,
    selected_part_ids: &BTreeSet<&str>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for check in &config.mosfet_checks {
        push_gate!(
            rows,
            errors,
            "mosfet",
            format!("{} selected part", check.id),
            check.part_id.clone(),
            "selected in parts.toml",
            selected_part_ids.contains(check.part_id.as_str()),
            &check.notes,
        );
        let derated_current =
            check.current_rating_ma as f64 * config.assumptions.component_current_derating;
        push_gate!(
            rows,
            errors,
            "mosfet",
            format!("{} continuous current", check.id),
            format!("{} mA", check.continuous_current_ma),
            format!("<= {derated_current:.0} mA derated"),
            check.continuous_current_ma as f64 <= derated_current,
            &check.notes,
        );
        let derated_voltage =
            check.voltage_rating_v * config.assumptions.component_voltage_derating;
        push_gate!(
            rows,
            errors,
            "mosfet",
            format!("{} Vds derating", check.id),
            format!("{:.1} V", check.drain_source_voltage_v),
            format!("<= {derated_voltage:.1} V derated"),
            check.drain_source_voltage_v <= derated_voltage,
            &check.notes,
        );
        let current_a = check.continuous_current_ma as f64 / 1000.0;
        let rds_on_ohm = check.rds_on_mohm / 1000.0;
        let power_w = current_a * current_a * rds_on_ohm;
        let temp_rise_c = power_w * check.package_theta_ja_c_per_w;
        push_gate!(
            rows,
            errors,
            "mosfet",
            format!("{} conduction heat", check.id),
            format!("{temp_rise_c:.1} C from {power_w:.3} W"),
            format!("<= {:.1} C", check.max_temp_rise_c),
            temp_rise_c <= check.max_temp_rise_c,
            &check.notes,
        );
    }
}

fn validate_trace_current_paths(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for path in &config.trace_current_paths {
        let segments = routing
            .segments
            .iter()
            .filter(|segment| segment.net == path.net)
            .collect::<Vec<_>>();
        push_gate!(
            rows,
            errors,
            "trace current",
            format!("{} routed segments", path.id),
            format!("{}", segments.len()),
            "> 0",
            !segments.is_empty(),
            &path.notes,
        );
        if segments.is_empty() {
            continue;
        }

        let mut min_capacity_ma = f64::INFINITY;
        let mut full_width_segments = 0usize;
        for segment in &segments {
            let allowed_layer = path
                .allowed_layers
                .iter()
                .any(|layer| layer == &segment.layer);
            push_gate!(
                rows,
                errors,
                "trace current",
                format!("{} layer {}", path.id, segment.layer),
                segment.layer.clone(),
                format!("one of {}", path.allowed_layers.join(",")),
                allowed_layer,
                &path.notes,
            );
            let neckdown = allowed_neckdown(segment, path);
            push_gate!(
                rows,
                errors,
                "trace current",
                format!("{} width on {}", path.id, segment.layer),
                format!("{:.3} mm", segment.width_mm),
                trace_width_limit(path),
                segment.width_mm + f64::EPSILON >= path.min_width_mm || neckdown,
                &path.notes,
            );
            if !neckdown {
                let capacity_ma = derated_trace_capacity_ma(
                    segment.width_mm,
                    &segment.layer,
                    &config.assumptions,
                );
                min_capacity_ma = min_capacity_ma.min(capacity_ma);
                full_width_segments += 1;
            }
        }

        push_gate!(
            rows,
            errors,
            "trace current",
            format!("{} IPC-2221 derated capacity", path.id),
            format!("{} mA load", path.current_ma),
            format!("<= {min_capacity_ma:.0} mA estimated"),
            full_width_segments > 0 && path.current_ma as f64 <= min_capacity_ma,
            &path.notes,
        );
    }
}

fn allowed_neckdown(segment: &RouteSegment, path: &TraceCurrentPath) -> bool {
    let Some(min_width_mm) = path.neckdown_min_width_mm else {
        return false;
    };
    let Some(max_length_mm) = path.max_neckdown_length_mm else {
        return false;
    };
    segment.width_mm + f64::EPSILON >= min_width_mm
        && segment_length_mm(segment) <= max_length_mm + f64::EPSILON
}

fn trace_width_limit(path: &TraceCurrentPath) -> String {
    match (path.neckdown_min_width_mm, path.max_neckdown_length_mm) {
        (Some(neckdown_width), Some(neckdown_length)) => format!(
            ">= {:.3} mm, or >= {:.3} mm for <= {:.3} mm neckdown",
            path.min_width_mm, neckdown_width, neckdown_length
        ),
        _ => format!(">= {:.3} mm", path.min_width_mm),
    }
}

fn segment_length_mm(segment: &RouteSegment) -> f64 {
    let dx = segment.end_x_mm - segment.start_x_mm;
    let dy = segment.end_y_mm - segment.start_y_mm;
    (dx * dx + dy * dy).sqrt()
}

fn validate_usb_signal_integrity(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let usb = &config.usb_signal_integrity;
    let allowed_layers = usb
        .allowed_layers
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();

    let dp_stats = collect_usb_route_stats(&usb.dp_net, routing, &allowed_layers);
    let dn_stats = collect_usb_route_stats(&usb.dn_net, routing, &allowed_layers);
    validate_usb_net(&usb.dp_net, &dp_stats, usb, contract_nets, rows, errors);
    validate_usb_net(&usb.dn_net, &dn_stats, usb, contract_nets, rows, errors);

    let length_skew_mm = (dp_stats.length_mm - dn_stats.length_mm).abs();
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        "D+/D- route length skew",
        format!("{length_skew_mm:.2} mm"),
        format!("<= {:.2} mm", usb.max_length_skew_mm),
        dp_stats.segment_count > 0
            && dn_stats.segment_count > 0
            && length_skew_mm <= usb.max_length_skew_mm,
        &usb.notes,
    );

    let width_mismatch_mm = match (dp_stats.min_width_mm, dn_stats.min_width_mm) {
        (Some(dp_width), Some(dn_width)) => (dp_width - dn_width).abs(),
        _ => f64::INFINITY,
    };
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        "D+/D- minimum width mismatch",
        if width_mismatch_mm.is_finite() {
            format!("{width_mismatch_mm:.3} mm")
        } else {
            "missing routed width".to_string()
        },
        format!("<= {:.3} mm", usb.max_width_mismatch_mm),
        width_mismatch_mm.is_finite() && width_mismatch_mm <= usb.max_width_mismatch_mm,
        &usb.notes,
    );
}

#[derive(Debug, Default)]
struct UsbRouteStats {
    segment_count: usize,
    length_mm: f64,
    min_width_mm: Option<f64>,
    layers: BTreeSet<String>,
    disallowed_layers: BTreeSet<String>,
    vias: BTreeSet<(i64, i64)>,
}

fn collect_usb_route_stats(
    net: &str,
    routing: &RoutingSeed,
    allowed_layers: &BTreeSet<&str>,
) -> UsbRouteStats {
    let mut stats = UsbRouteStats::default();
    for segment in routing.segments.iter().filter(|segment| segment.net == net) {
        stats.segment_count += 1;
        stats.length_mm += segment_length_mm(segment);
        stats.min_width_mm = Some(
            stats
                .min_width_mm
                .map_or(segment.width_mm, |width| width.min(segment.width_mm)),
        );
        stats.layers.insert(segment.layer.clone());
        if !allowed_layers.contains(segment.layer.as_str()) {
            stats.disallowed_layers.insert(segment.layer.clone());
        }
        if segment.via_at_ends || segment.via_at_start {
            stats
                .vias
                .insert(route_point_key(segment.start_x_mm, segment.start_y_mm));
        }
        if segment.via_at_ends || segment.via_at_end {
            stats
                .vias
                .insert(route_point_key(segment.end_x_mm, segment.end_y_mm));
        }
    }
    stats
}

fn route_point_key(x_mm: f64, y_mm: f64) -> (i64, i64) {
    (
        (x_mm * 1000.0).round() as i64,
        (y_mm * 1000.0).round() as i64,
    )
}

fn validate_usb_net(
    net: &str,
    stats: &UsbRouteStats,
    usb: &UsbSignalIntegrity,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} contract net"),
        net.to_string(),
        "present in contract",
        contract_nets.contains(net),
        &usb.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} routed segments"),
        format!("{}", stats.segment_count),
        "> 0",
        stats.segment_count > 0,
        &usb.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} route length"),
        format!("{:.2} mm", stats.length_mm),
        format!("<= {:.2} mm", usb.max_route_length_mm),
        stats.segment_count > 0 && stats.length_mm <= usb.max_route_length_mm,
        &usb.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} minimum width"),
        stats
            .min_width_mm
            .map(|width| format!("{width:.3} mm"))
            .unwrap_or_else(|| "missing".to_string()),
        format!(">= {:.3} mm", usb.min_width_mm),
        stats
            .min_width_mm
            .is_some_and(|width| width + f64::EPSILON >= usb.min_width_mm),
        &usb.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} layers"),
        format_string_set(&stats.layers),
        format!("one of {}", usb.allowed_layers.join(",")),
        stats.segment_count > 0 && stats.disallowed_layers.is_empty(),
        &usb.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb signal integrity",
        format!("{net} route vias"),
        format!("{}", stats.vias.len()),
        format!("<= {}", usb.max_vias_per_net),
        stats.vias.len() <= usb.max_vias_per_net,
        &usb.notes,
    );
}

fn format_string_set(values: &BTreeSet<String>) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().cloned().collect::<Vec<_>>().join(",")
    }
}

fn derated_trace_capacity_ma(width_mm: f64, layer: &str, assumptions: &Assumptions) -> f64 {
    let is_inner = layer.starts_with("In");
    let copper_oz = if is_inner {
        assumptions.inner_copper_oz
    } else {
        assumptions.outer_copper_oz
    };
    let k = if is_inner { 0.024 } else { 0.048 };
    let width_mil = width_mm / 0.0254;
    let thickness_mil = copper_oz * 1.378;
    let area_mil2 = width_mil * thickness_mil;
    let current_a = k * assumptions.allowed_trace_temp_rise_c.powf(0.44) * area_mil2.powf(0.725);
    current_a * 1000.0 * assumptions.trace_current_derating
}

fn validate_gpio_domains(
    config: &ValidationConfig,
    pin_nets: &PinNetManifest,
    firmware: &FirmwareHandoff,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let forbidden = config
        .gpio_domain
        .forbidden_high_voltage_nets
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let Some(mcu_assignment) = pin_nets
        .assignments
        .iter()
        .find(|assignment| assignment.reference == config.gpio_domain.mcu_reference)
    else {
        push_gate!(
            rows,
            errors,
            "gpio domain",
            "MCU pin-net assignment",
            "missing",
            "present",
            false,
            &config.gpio_domain.notes,
        );
        return;
    };

    for pin in &firmware.module_pins {
        let in_pin_nets = mcu_assignment
            .pins
            .get(&pin.module_pin.to_string())
            .is_some_and(|net| net == &pin.net);
        push_gate!(
            rows,
            errors,
            "gpio domain",
            format!("module pin {} handoff", pin.module_pin),
            pin.net.clone(),
            "matches pin_nets.toml",
            in_pin_nets,
            &config.gpio_domain.notes,
        );
        push_gate!(
            rows,
            errors,
            "gpio domain",
            format!("module pin {} voltage domain", pin.module_pin),
            pin.net.clone(),
            "not high-voltage rail",
            !forbidden.contains(pin.net.as_str()),
            &config.gpio_domain.notes,
        );
    }
}

fn validate_analog_ranges(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    for check in &config.analog_checks {
        push_gate!(
            rows,
            errors,
            "analog range",
            format!("{} net exists", check.id),
            check.net.clone(),
            "present in contract",
            contract_nets.contains(&check.net),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "analog range",
            format!("{} expected input", check.id),
            format!("{:.2} V", check.max_expected_v),
            format!("<= {:.2} V abs max", check.absolute_max_v),
            check.max_expected_v <= check.absolute_max_v,
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "analog range",
            format!("{} logic rail clamp", check.id),
            format!("{:.2} V", check.max_expected_v),
            "<= 3.30 V nominal rail",
            check.max_expected_v <= 3.3,
            &check.notes,
        );
    }
}

fn validate_first_article_measurements(
    config: &ValidationConfig,
    placement: &PlacementPlan,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let test_points = placement
        .test_points
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<BTreeMap<_, _>>();
    let rail_budgets = config
        .rail_budgets
        .iter()
        .map(|budget| (budget.rail.as_str(), budget))
        .collect::<BTreeMap<_, _>>();
    let mut ids = BTreeSet::new();
    let mut orders = BTreeSet::new();

    for measurement in &config.first_article_measurements {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} unique id", measurement.id),
            measurement.id.clone(),
            "unique",
            ids.insert(measurement.id.as_str()),
            &measurement.notes,
        );
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} unique order", measurement.id),
            format!("{}", measurement.order),
            "unique",
            orders.insert(measurement.order),
            &measurement.notes,
        );
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} stage", measurement.id),
            measurement.stage.clone(),
            "non-empty",
            !measurement.stage.trim().is_empty(),
            &measurement.notes,
        );
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} pass criterion", measurement.id),
            measurement.pass_criterion.clone(),
            "non-empty",
            !measurement.pass_criterion.trim().is_empty(),
            &measurement.notes,
        );

        let Some(point) = test_points.get(measurement.test_point.as_str()) else {
            push_gate!(
                rows,
                errors,
                "first article",
                format!("{} test point", measurement.id),
                measurement.test_point.clone(),
                "exists in placement.toml",
                false,
                &measurement.notes,
            );
            continue;
        };

        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} test point net", measurement.id),
            point.net.clone(),
            measurement.net.clone(),
            point.net == measurement.net,
            &measurement.notes,
        );
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} test point side", measurement.id),
            point.side.clone(),
            "top",
            point.side == "top",
            &measurement.notes,
        );

        validate_measurement_voltage_window(config, measurement, &rail_budgets, rows, errors);
        validate_measurement_current_limit(config, measurement, &rail_budgets, rows, errors);
    }
}

fn validate_measurement_voltage_window(
    config: &ValidationConfig,
    measurement: &FirstArticleMeasurement,
    rail_budgets: &BTreeMap<&str, &RailBudget>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let has_voltage_window = measurement.min_v.is_some() || measurement.max_v.is_some();
    if !has_voltage_window {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} voltage window", measurement.id),
            "not voltage-gated",
            "allowed for resistance/non-voltage checks",
            true,
            &measurement.notes,
        );
        return;
    }

    let (Some(min_v), Some(max_v)) = (measurement.min_v, measurement.max_v) else {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} voltage window", measurement.id),
            "partial window",
            "both min_v and max_v",
            false,
            &measurement.notes,
        );
        return;
    };

    push_gate!(
        rows,
        errors,
        "first article",
        format!("{} voltage window", measurement.id),
        format!("{min_v:.3}..{max_v:.3} V"),
        "min_v <= max_v",
        min_v <= max_v,
        &measurement.notes,
    );

    if let Some(budget) = rail_budgets.get(measurement.net.as_str()) {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} voltage budget", measurement.id),
            format!("{min_v:.3}..{max_v:.3} V"),
            format!("{:.3}..{:.3} V rail budget", budget.min_v, budget.max_v),
            min_v + f64::EPSILON >= budget.min_v && max_v <= budget.max_v + f64::EPSILON,
            &measurement.notes,
        );
    } else {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} logic/analog voltage domain", measurement.id),
            format!("{max_v:.3} V"),
            format!("<= {:.3} V logic abs max", config.gpio_domain.logic_max_v),
            max_v <= config.gpio_domain.logic_max_v,
            &measurement.notes,
        );
    }
}

fn validate_measurement_current_limit(
    config: &ValidationConfig,
    measurement: &FirstArticleMeasurement,
    rail_budgets: &BTreeMap<&str, &RailBudget>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let Some(current_limit_ma) = measurement.current_limit_ma else {
        push_gate!(
            rows,
            errors,
            "first article",
            format!("{} current limit", measurement.id),
            "not powered",
            "allowed for unpowered checks",
            measurement.measurement == "resistance_to_gnd",
            &measurement.notes,
        );
        return;
    };

    let limit_ma = rail_budgets
        .get(measurement.net.as_str())
        .map(|rail| rail.source_limit_ma)
        .unwrap_or(config.assumptions.first_article_current_limit_ma);
    push_gate!(
        rows,
        errors,
        "first article",
        format!("{} current limit", measurement.id),
        format!("{current_limit_ma} mA"),
        format!("<= {limit_ma} mA validation budget"),
        current_limit_ma <= limit_ma,
        &measurement.notes,
    );
}

fn validate_component_deratings(
    config: &ValidationConfig,
    selected_part_ids: &BTreeSet<&str>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let mut ids = BTreeSet::new();
    for derating in &config.component_deratings {
        push_gate!(
            rows,
            errors,
            "component derating",
            format!("{} unique id", derating.id),
            derating.id.clone(),
            "unique",
            ids.insert(derating.id.as_str()),
            &derating.notes,
        );
        push_gate!(
            rows,
            errors,
            "component derating",
            format!("{} part exists", derating.id),
            derating.part_id.clone(),
            "selected part id",
            selected_part_ids.contains(derating.part_id.as_str()),
            &derating.notes,
        );
        push_gate!(
            rows,
            errors,
            "component derating",
            format!("{} stress class", derating.id),
            derating.stress_class.clone(),
            "non-empty",
            !derating.stress_class.trim().is_empty(),
            &derating.notes,
        );

        validate_derating_pair(
            rows,
            errors,
            DeratingPair {
                id: &derating.id,
                label: "voltage",
                operating: derating.operating_voltage_v,
                rated: derating.rated_voltage_v,
                limit: derating
                    .max_voltage_utilization
                    .unwrap_or(config.assumptions.component_voltage_derating),
                unit: "V",
                notes: &derating.notes,
            },
        );
        validate_derating_pair(
            rows,
            errors,
            DeratingPair {
                id: &derating.id,
                label: "current",
                operating: derating.operating_current_ma.map(|value| value as f64),
                rated: derating.rated_current_ma.map(|value| value as f64),
                limit: derating
                    .max_current_utilization
                    .unwrap_or(config.assumptions.component_current_derating),
                unit: "mA",
                notes: &derating.notes,
            },
        );
        validate_derating_pair(
            rows,
            errors,
            DeratingPair {
                id: &derating.id,
                label: "power",
                operating: derating.operating_power_w,
                rated: derating.rated_power_w,
                limit: derating
                    .max_power_utilization
                    .unwrap_or(config.assumptions.component_power_derating),
                unit: "W",
                notes: &derating.notes,
            },
        );

        if let (Some(estimated), Some(limit)) =
            (derating.estimated_temp_rise_c, derating.max_temp_rise_c)
        {
            push_gate!(
                rows,
                errors,
                "component derating",
                format!("{} temperature rise", derating.id),
                format!("{estimated:.2} C"),
                format!("<= {limit:.2} C"),
                estimated <= limit,
                &derating.notes,
            );
        } else {
            push_gate!(
                rows,
                errors,
                "component derating",
                format!("{} temperature rise", derating.id),
                "not modeled",
                "allowed when power/thermal stress is not primary",
                true,
                &derating.notes,
            );
        }
    }
}

struct DeratingPair<'a> {
    id: &'a str,
    label: &'a str,
    operating: Option<f64>,
    rated: Option<f64>,
    limit: f64,
    unit: &'a str,
    notes: &'a str,
}

fn validate_derating_pair(rows: &mut Vec<GateRow>, errors: &mut Vec<String>, pair: DeratingPair) {
    match (pair.operating, pair.rated) {
        (Some(operating), Some(rated)) if rated > 0.0 => {
            let utilization = operating / rated;
            push_gate!(
                rows,
                errors,
                "component derating",
                format!("{} {} utilization", pair.id, pair.label),
                format!(
                    "{operating:.3} {} / {rated:.3} {} = {:.1}%",
                    pair.unit,
                    pair.unit,
                    utilization * 100.0
                ),
                format!("<= {:.1}%", pair.limit * 100.0),
                utilization <= pair.limit,
                pair.notes,
            );
        }
        (None, None) => push_gate!(
            rows,
            errors,
            "component derating",
            format!("{} {} utilization", pair.id, pair.label),
            "not applicable",
            "allowed when not a stress axis",
            true,
            pair.notes,
        ),
        _ => push_gate!(
            rows,
            errors,
            "component derating",
            format!("{} {} utilization", pair.id, pair.label),
            "partial rating data",
            "operating and rated values",
            false,
            pair.notes,
        ),
    }
}

fn validate_single_faults(
    config: &ValidationConfig,
    parts: &PartsManifest,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let evidence_ids = fault_evidence_ids(config, parts);
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();

    for check in &config.single_fault_checks {
        let rpn = fault_rpn(check);
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} score range", check.id),
            format!(
                "S{} O{} D{}",
                check.severity, check.occurrence, check.detection
            ),
            "each 1..10",
            valid_fmea_score(check.severity)
                && valid_fmea_score(check.occurrence)
                && valid_fmea_score(check.detection),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} RPN", check.id),
            format!("{rpn}"),
            format!("<= {}", config.single_fault_policy.max_rpn),
            rpn <= config.single_fault_policy.max_rpn,
            &check.notes,
        );

        let needs_hardware = check.severity
            >= config
                .single_fault_policy
                .require_hardware_mitigation_for_severity_at_least;
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} hardware mitigation", check.id),
            mitigation_list(&check.hardware_mitigations),
            if needs_hardware {
                "required and evidence-backed"
            } else {
                "optional"
            },
            !needs_hardware || !check.hardware_mitigations.is_empty(),
            &check.notes,
        );
        for mitigation in &check.hardware_mitigations {
            push_gate!(
                rows,
                errors,
                "single fault",
                format!("{} hardware evidence {}", check.id, mitigation),
                mitigation.clone(),
                "known validation/part/path id",
                evidence_ids.contains(mitigation.as_str()),
                &check.notes,
            );
        }

        let needs_detection = check.severity
            >= config
                .single_fault_policy
                .require_detection_for_severity_at_least;
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} detection method", check.id),
            mitigation_list(&check.detection_methods),
            if needs_detection {
                "required"
            } else {
                "optional"
            },
            !needs_detection || !check.detection_methods.is_empty(),
            &check.notes,
        );

        let needs_first_article = check.severity
            >= config
                .single_fault_policy
                .require_first_article_for_severity_at_least;
        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} first-article coverage", check.id),
            mitigation_list(&check.verification_measurements),
            if needs_first_article {
                "required and in first_article_measurements"
            } else {
                "optional"
            },
            !needs_first_article || !check.verification_measurements.is_empty(),
            &check.notes,
        );
        for measurement in &check.verification_measurements {
            push_gate!(
                rows,
                errors,
                "single fault",
                format!("{} verifies {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
                &check.notes,
            );
        }

        push_gate!(
            rows,
            errors,
            "single fault",
            format!("{} firmware mitigation", check.id),
            mitigation_list(&check.firmware_mitigations),
            "non-empty for traceable firmware behavior",
            !check.firmware_mitigations.is_empty(),
            &check.notes,
        );
    }
}

fn validate_emc_esd_precompliance(
    config: &ValidationConfig,
    parts: &PartsManifest,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let evidence_ids = fault_evidence_ids(config, parts);
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();
    let external_analysis_ids = config
        .external_analysis_handoffs
        .iter()
        .map(|handoff| handoff.id.as_str())
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "emc esd",
        "minimum interface coverage",
        format!("{} checks", config.emc_esd_checks.len()),
        format!(">= {} checks", config.emc_esd_policy.min_checks),
        config.emc_esd_checks.len() >= config.emc_esd_policy.min_checks,
        "EMC/ESD pre-compliance must cover each exposed/noisy interface, not just one rail.",
    );

    let mut ids = BTreeSet::new();
    for check in &config.emc_esd_checks {
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} risk score", check.id),
            format!("{}", check.risk_score),
            format!("1..={}", config.emc_esd_policy.max_risk_score),
            (1..=config.emc_esd_policy.max_risk_score).contains(&check.risk_score),
            &check.notes,
        );

        for net in &check.nets {
            push_gate!(
                rows,
                errors,
                "emc esd",
                format!("{} net {}", check.id, net),
                net.clone(),
                "known contract net",
                contract_nets.contains(net),
                &check.notes,
            );
        }

        let needs_protection =
            check.risk_score >= config.emc_esd_policy.require_protection_for_risk_at_least;
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} protection coverage", check.id),
            mitigation_list(&check.protection_part_ids),
            if needs_protection {
                "required and evidence-backed"
            } else {
                "optional"
            },
            !needs_protection || !check.protection_part_ids.is_empty(),
            &check.notes,
        );
        for part_id in &check.protection_part_ids {
            push_gate!(
                rows,
                errors,
                "emc esd",
                format!("{} protection evidence {}", check.id, part_id),
                part_id.clone(),
                "known selected/external part or validation id",
                evidence_ids.contains(part_id.as_str()),
                &check.notes,
            );
        }

        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} return path", check.id),
            check.return_path_strategy.clone(),
            if config.emc_esd_policy.require_return_path_strategy {
                "non-empty strategy"
            } else {
                "documented"
            },
            !config.emc_esd_policy.require_return_path_strategy
                || !check.return_path_strategy.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} grounding/shielding", check.id),
            check.grounding_strategy.clone(),
            "non-empty strategy",
            !check.grounding_strategy.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} verification methods", check.id),
            mitigation_list(&check.verification_methods),
            "non-empty",
            !check.verification_methods.is_empty(),
            &check.notes,
        );

        let needs_measurement =
            check.risk_score >= config.emc_esd_policy.require_measurement_for_risk_at_least;
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} first-article coverage", check.id),
            mitigation_list(&check.verification_measurements),
            if needs_measurement {
                "required and in first_article_measurements"
            } else {
                "optional"
            },
            !needs_measurement || !check.verification_measurements.is_empty(),
            &check.notes,
        );
        for measurement in &check.verification_measurements {
            push_gate!(
                rows,
                errors,
                "emc esd",
                format!("{} verifies {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
                &check.notes,
            );
        }

        let needs_external_analysis = check.risk_score
            >= config
                .emc_esd_policy
                .require_external_analysis_for_risk_at_least;
        push_gate!(
            rows,
            errors,
            "emc esd",
            format!("{} external analysis", check.id),
            mitigation_list(&check.external_analysis),
            if needs_external_analysis {
                "required and in external_analysis_handoffs"
            } else {
                "optional"
            },
            !needs_external_analysis || !check.external_analysis.is_empty(),
            &check.notes,
        );
        for analysis_id in &check.external_analysis {
            push_gate!(
                rows,
                errors,
                "emc esd",
                format!("{} analysis evidence {}", check.id, analysis_id),
                analysis_id.clone(),
                "known external analysis handoff id",
                external_analysis_ids.contains(analysis_id.as_str()),
                &check.notes,
            );
        }
    }
}

fn validate_boot_startup_safety(
    config: &ValidationConfig,
    parts: &PartsManifest,
    contract_nets: &BTreeSet<String>,
    firmware: &FirmwareHandoff,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let evidence_ids = startup_evidence_ids(config, parts);
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();
    let firmware_by_net = firmware
        .module_pins
        .iter()
        .map(|pin| (pin.net.as_str(), pin))
        .collect::<BTreeMap<_, _>>();

    push_gate!(
        rows,
        errors,
        "startup safety",
        "minimum startup coverage",
        format!("{} checks", config.boot_startup_checks.len()),
        format!(">= {} checks", config.boot_startup_policy.min_checks),
        config.boot_startup_checks.len() >= config.boot_startup_policy.min_checks,
        "Startup validation must cover ESP32 boot straps plus fail-closed heater control.",
    );

    let mut ids = BTreeSet::new();
    let mut covered_boot_pins = BTreeSet::new();
    for check in &config.boot_startup_checks {
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} net", check.id),
            check.net.clone(),
            "known contract net",
            contract_nets.contains(&check.net),
            &check.notes,
        );

        let firmware_pin = firmware_by_net.get(check.net.as_str()).copied();
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} firmware net", check.id),
            check.net.clone(),
            "present in firmware handoff",
            firmware_pin.is_some(),
            &check.notes,
        );
        if let Some(pin) = firmware_pin {
            push_gate!(
                rows,
                errors,
                "startup safety",
                format!("{} module pin", check.id),
                format!("{}", check.module_pin),
                format!("{}", pin.module_pin),
                check.module_pin == pin.module_pin,
                &check.notes,
            );
            push_gate!(
                rows,
                errors,
                "startup safety",
                format!("{} boot-sensitive flag", check.id),
                format!("{}", check.boot_sensitive),
                format!("{}", pin.boot_sensitive),
                check.boot_sensitive == pin.boot_sensitive,
                &check.notes,
            );
            if pin.boot_sensitive {
                covered_boot_pins.insert((pin.module_pin, pin.net.as_str()));
            }
        }

        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} expected power-on state", check.id),
            check.expected_power_on_state.clone(),
            "non-empty",
            !check.expected_power_on_state.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} safe state", check.id),
            check.safe_state.clone(),
            "non-empty",
            !check.safe_state.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} verification methods", check.id),
            mitigation_list(&check.verification_methods),
            "non-empty",
            !check.verification_methods.is_empty(),
            &check.notes,
        );

        let needs_evidence = check.boot_sensitive
            && config
                .boot_startup_policy
                .require_evidence_for_boot_sensitive;
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} evidence coverage", check.id),
            mitigation_list(&check.evidence_ids),
            if needs_evidence {
                "required and evidence-backed"
            } else {
                "documented"
            },
            !needs_evidence || !check.evidence_ids.is_empty(),
            &check.notes,
        );
        for evidence_id in &check.evidence_ids {
            push_gate!(
                rows,
                errors,
                "startup safety",
                format!("{} evidence {}", check.id, evidence_id),
                evidence_id.clone(),
                "known selected/external part, validation id, or firmware handoff",
                evidence_ids.contains(evidence_id.as_str()),
                &check.notes,
            );
        }

        let needs_measurement = check.safety_critical
            && config
                .boot_startup_policy
                .require_measurement_for_safety_critical;
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} first-article coverage", check.id),
            mitigation_list(&check.verification_measurements),
            if needs_measurement {
                "required and in first_article_measurements"
            } else {
                "optional"
            },
            !needs_measurement || !check.verification_measurements.is_empty(),
            &check.notes,
        );
        for measurement in &check.verification_measurements {
            push_gate!(
                rows,
                errors,
                "startup safety",
                format!("{} verifies {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
                &check.notes,
            );
        }

        let must_be_inactive = check.safety_critical
            && config
                .boot_startup_policy
                .forbid_active_safety_outputs_at_reset;
        push_gate!(
            rows,
            errors,
            "startup safety",
            format!("{} active at reset", check.id),
            format!("{}", check.allowed_active_at_reset),
            if must_be_inactive {
                "false for safety-critical outputs"
            } else {
                "documented"
            },
            !must_be_inactive || !check.allowed_active_at_reset,
            &check.notes,
        );
    }

    if config
        .boot_startup_policy
        .require_all_boot_sensitive_firmware_pins
    {
        for pin in firmware.module_pins.iter().filter(|pin| pin.boot_sensitive) {
            push_gate!(
                rows,
                errors,
                "startup safety",
                format!("boot-sensitive pin {} {}", pin.module_pin, pin.net),
                pin.net.clone(),
                "covered by boot_startup_checks",
                covered_boot_pins.contains(&(pin.module_pin, pin.net.as_str())),
                "Every ESP32-S3 boot-sensitive assignment must have an explicit reset/startup safety check.",
            );
        }
    }
}

fn validate_manufacturing_test_coverage(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    placement: &PlacementPlan,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let test_points = placement
        .test_points
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<BTreeMap<_, _>>();
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "manufacturing test",
        "minimum manufacturing coverage",
        format!("{} checks", config.manufacturing_test_checks.len()),
        format!(">= {} checks", config.manufacturing_test_policy.min_checks),
        config.manufacturing_test_checks.len() >= config.manufacturing_test_policy.min_checks,
        "Manufacturing/DFT validation must cover rails, programming, heater, sensor, debug, manual install, and traceability.",
    );

    let mut ids = BTreeSet::new();
    for check in &config.manufacturing_test_checks {
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} subsystem", check.id),
            check.subsystem.clone(),
            "non-empty",
            !check.subsystem.trim().is_empty(),
            &check.notes,
        );
        for net in &check.nets {
            push_gate!(
                rows,
                errors,
                "manufacturing test",
                format!("{} net {}", check.id, net),
                net.clone(),
                "known contract net",
                contract_nets.contains(net),
                &check.notes,
            );
        }

        let requires_test_points = config.manufacturing_test_policy.require_test_points;
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} test point coverage", check.id),
            mitigation_list(&check.required_test_points),
            if requires_test_points {
                "non-empty and net-matched"
            } else {
                "documented"
            },
            !requires_test_points || !check.required_test_points.is_empty(),
            &check.notes,
        );
        for test_point in &check.required_test_points {
            let point = test_points.get(test_point.as_str());
            push_gate!(
                rows,
                errors,
                "manufacturing test",
                format!("{} test point {}", check.id, test_point),
                test_point.clone(),
                "known placement test point",
                point.is_some(),
                &check.notes,
            );
            if let Some(point) = point {
                push_gate!(
                    rows,
                    errors,
                    "manufacturing test",
                    format!("{} test point {} net", check.id, test_point),
                    point.net.clone(),
                    "one of checked nets",
                    check.nets.iter().any(|net| net == &point.net),
                    &check.notes,
                );
            }
        }

        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} stage", check.id),
            check.test_stage.clone(),
            "non-empty",
            !check.test_stage.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} method", check.id),
            check.test_method.clone(),
            "non-empty",
            !check.test_method.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} pass criterion", check.id),
            check.pass_criterion.clone(),
            if config.manufacturing_test_policy.require_pass_criterion {
                "non-empty"
            } else {
                "documented"
            },
            !config.manufacturing_test_policy.require_pass_criterion
                || !check.pass_criterion.trim().is_empty(),
            &check.notes,
        );

        let has_measurement = !check.verification_measurements.is_empty();
        let has_firmware_test = !check.firmware_test.trim().is_empty();
        push_gate!(
            rows,
            errors,
            "manufacturing test",
            format!("{} automated/bench evidence", check.id),
            format!(
                "measurements: {}; firmware: {}",
                mitigation_list(&check.verification_measurements),
                if has_firmware_test {
                    check.firmware_test.as_str()
                } else {
                    "none"
                }
            ),
            if config
                .manufacturing_test_policy
                .require_measurement_or_firmware_test
            {
                "measurement or firmware test"
            } else {
                "documented"
            },
            !config
                .manufacturing_test_policy
                .require_measurement_or_firmware_test
                || has_measurement
                || has_firmware_test,
            &check.notes,
        );
        for measurement in &check.verification_measurements {
            push_gate!(
                rows,
                errors,
                "manufacturing test",
                format!("{} verifies {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
                &check.notes,
            );
        }
    }
}

fn validate_calibration_readiness(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "calibration readiness",
        "minimum calibration coverage",
        format!("{} checks", config.calibration_checks.len()),
        format!(">= {} checks", config.calibration_policy.min_checks),
        config.calibration_checks.len() >= config.calibration_policy.min_checks,
        "Calibration readiness must cover heater behavior, optical baseline, sensor bus, assay baseline, and traceability.",
    );

    let mut ids = BTreeSet::new();
    for check in &config.calibration_checks {
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} subsystem", check.id),
            check.subsystem.clone(),
            "non-empty",
            !check.subsystem.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} calibrated item", check.id),
            check.calibrated_item.clone(),
            "non-empty",
            !check.calibrated_item.trim().is_empty(),
            &check.notes,
        );
        for net in &check.dependent_nets {
            push_gate!(
                rows,
                errors,
                "calibration readiness",
                format!("{} net {}", check.id, net),
                net.clone(),
                "known contract net",
                contract_nets.contains(net),
                &check.notes,
            );
        }

        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} preconditions", check.id),
            mitigation_list(&check.preconditions),
            if config.calibration_policy.require_control_conditions {
                "non-empty"
            } else {
                "documented"
            },
            !config.calibration_policy.require_control_conditions
                || !check.preconditions.is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} procedure", check.id),
            check.procedure.clone(),
            "non-empty",
            !check.procedure.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} acceptance criterion", check.id),
            check.acceptance_criterion.clone(),
            if config.calibration_policy.require_acceptance_criterion {
                "non-empty"
            } else {
                "documented"
            },
            !config.calibration_policy.require_acceptance_criterion
                || !check.acceptance_criterion.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} control data", check.id),
            mitigation_list(&check.control_data),
            if config.calibration_policy.require_control_conditions {
                "non-empty"
            } else {
                "documented"
            },
            !config.calibration_policy.require_control_conditions || !check.control_data.is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} required outputs", check.id),
            mitigation_list(&check.required_outputs),
            if config.calibration_policy.require_output_artifacts {
                "non-empty"
            } else {
                "documented"
            },
            !config.calibration_policy.require_output_artifacts
                || !check.required_outputs.is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "calibration readiness",
            format!("{} firmware dependency", check.id),
            check.firmware_dependency.clone(),
            if config.calibration_policy.require_firmware_dependency {
                "non-empty"
            } else {
                "documented"
            },
            !config.calibration_policy.require_firmware_dependency
                || !check.firmware_dependency.trim().is_empty(),
            &check.notes,
        );
        for measurement in &check.verification_measurements {
            push_gate!(
                rows,
                errors,
                "calibration readiness",
                format!("{} verifies {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
                &check.notes,
            );
        }
    }
}

fn validate_validation_traceability(
    config: &ValidationConfig,
    gate_categories: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    push_gate!(
        rows,
        errors,
        "validation traceability",
        "minimum validation traceability coverage",
        format!("{} items", config.validation_traceability.len()),
        format!(">= {} items", config.traceability_policy.min_items),
        config.validation_traceability.len() >= config.traceability_policy.min_items,
        "Every major release gate needs a traceable artifact, CI evidence path, and acceptance criterion.",
    );

    let mut ids = BTreeSet::new();
    for item in &config.validation_traceability {
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} unique id", item.id),
            item.id.clone(),
            "unique",
            ids.insert(item.id.as_str()),
            &item.notes,
        );
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} validation layer", item.id),
            item.validation_layer.clone(),
            "non-empty",
            !item.validation_layer.trim().is_empty(),
            &item.notes,
        );
        if let Some(category) = &item.electrical_gate_category {
            push_gate!(
                rows,
                errors,
                "validation traceability",
                format!("{} electrical gate category", item.id),
                category.clone(),
                "present in electrical validation rows",
                gate_categories.contains(category),
                &item.notes,
            );
        }
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} output artifact", item.id),
            item.output_artifact.clone(),
            if config.traceability_policy.require_output_artifact {
                "non-empty"
            } else {
                "documented"
            },
            !config.traceability_policy.require_output_artifact
                || !item.output_artifact.trim().is_empty(),
            &item.notes,
        );
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} release manifest entry", item.id),
            item.release_manifest_entry.clone(),
            if config.traceability_policy.require_release_manifest_entry {
                "non-empty"
            } else {
                "documented"
            },
            !config.traceability_policy.require_release_manifest_entry
                || !item.release_manifest_entry.trim().is_empty(),
            &item.notes,
        );
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} CI step", item.id),
            item.ci_step.clone(),
            if config.traceability_policy.require_ci_step {
                "non-empty"
            } else {
                "documented"
            },
            !config.traceability_policy.require_ci_step || !item.ci_step.trim().is_empty(),
            &item.notes,
        );
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} acceptance criterion", item.id),
            item.acceptance_criterion.clone(),
            if config.traceability_policy.require_acceptance_criterion {
                "non-empty"
            } else {
                "documented"
            },
            !config.traceability_policy.require_acceptance_criterion
                || !item.acceptance_criterion.trim().is_empty(),
            &item.notes,
        );
        push_gate!(
            rows,
            errors,
            "validation traceability",
            format!("{} release blocking", item.id),
            item.blocks_release.to_string(),
            if config.traceability_policy.require_release_blocking {
                "true"
            } else {
                "documented"
            },
            !config.traceability_policy.require_release_blocking || item.blocks_release,
            &item.notes,
        );
    }
}

fn valid_fmea_score(value: u8) -> bool {
    (1..=10).contains(&value)
}

fn fault_rpn(check: &SingleFaultCheck) -> u32 {
    u32::from(check.severity) * u32::from(check.occurrence) * u32::from(check.detection)
}

fn mitigation_list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(";")
    }
}

fn fault_evidence_ids<'a>(
    config: &'a ValidationConfig,
    parts: &'a PartsManifest,
) -> BTreeSet<&'a str> {
    let mut ids = BTreeSet::new();
    ids.extend(parts.selected_parts.iter().map(|part| part.id.as_str()));
    ids.extend(
        parts
            .external_safety_parts
            .iter()
            .map(|part| part.id.as_str()),
    );
    ids.extend(config.rail_budgets.iter().map(|rail| rail.rail.as_str()));
    ids.extend(
        config
            .trace_current_paths
            .iter()
            .map(|path| path.id.as_str()),
    );
    ids.extend(
        config
            .protection_checks
            .iter()
            .map(|check| check.id.as_str()),
    );
    ids.extend(config.mosfet_checks.iter().map(|check| check.id.as_str()));
    ids.extend(config.analog_checks.iter().map(|check| check.id.as_str()));
    ids.extend(
        config
            .component_deratings
            .iter()
            .map(|derating| derating.id.as_str()),
    );
    ids.extend(
        config
            .external_analysis_handoffs
            .iter()
            .map(|handoff| handoff.id.as_str()),
    );
    ids.insert("gpio_domain");
    ids.insert("usb_signal_integrity");
    ids
}

fn startup_evidence_ids<'a>(
    config: &'a ValidationConfig,
    parts: &'a PartsManifest,
) -> BTreeSet<&'a str> {
    let mut ids = fault_evidence_ids(config, parts);
    ids.extend(
        config
            .first_article_measurements
            .iter()
            .map(|measurement| measurement.id.as_str()),
    );
    ids.insert("firmware_handoff");
    ids.insert("boot_startup_policy");
    ids
}

fn add_external_analysis_rows(config: &ValidationConfig, rows: &mut Vec<GateRow>) {
    for handoff in &config.external_analysis_handoffs {
        rows.push(GateRow {
            category: "external analysis".to_string(),
            item: handoff.id.clone(),
            measured: handoff.tool_class.clone(),
            limit: handoff.exit_criterion.clone(),
            status: "manual".to_string(),
            notes: format!(
                "Tools: {}; inputs: {}",
                handoff.recommended_tools.join(", "),
                handoff.inputs.join(", ")
            ),
        });
    }
}

fn add_manual_gate_rows(
    config: &ValidationConfig,
    parts: &PartsManifest,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let has_external_cutoff = parts
        .external_safety_parts
        .iter()
        .any(|part| part.id == "inline_thermal_cutoff");
    push_gate!(
        rows,
        errors,
        "manual gate",
        "external heater thermal cutoff specified",
        if has_external_cutoff {
            "present"
        } else {
            "missing"
        },
        "present",
        has_external_cutoff,
        "External thermal cutoff is required because the heater is outside the PCB.",
    );

    for gate in &config.manual_first_article_gates {
        rows.push(GateRow {
            category: "manual first article".to_string(),
            item: gate.id.clone(),
            measured: gate.description.clone(),
            limit: gate.pass_criterion.clone(),
            status: "manual".to_string(),
            notes: "Required after assembly before full-power operation.".to_string(),
        });
    }
}

fn push_gate_row(rows: &mut Vec<GateRow>, errors: &mut Vec<String>, gate: GateSpec) {
    let status = if gate.pass { "pass" } else { "fail" }.to_string();
    if !gate.pass {
        errors.push(format!(
            "{}: {} measured {}; limit {}",
            gate.category, gate.item, gate.measured, gate.limit
        ));
    }
    rows.push(GateRow {
        category: gate.category,
        item: gate.item,
        measured: gate.measured,
        limit: gate.limit,
        status,
        notes: gate.notes,
    });
}

fn write_report(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
    rows: &[GateRow],
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.report_md)?;
    let summary = summarize_rows(rows);
    let mut report = String::new();
    writeln!(report, "# LAMP Rev A Electrical Validation")?;
    writeln!(report)?;
    writeln!(
        report,
        "Generated from `{CONFIG_PATH}`. This is a pre-fab electrical sanity gate, not a substitute for first-article bench testing."
    )?;
    writeln!(report)?;
    writeln!(report, "## Summary")?;
    writeln!(report)?;
    writeln!(report, "- Passing machine gates: `{}`", summary.pass_count)?;
    writeln!(report, "- Failing machine gates: `{}`", summary.fail_count)?;
    writeln!(
        report,
        "- Manual analysis / first-article gates: `{}`",
        summary.manual_count
    )?;
    writeln!(
        report,
        "- Ambient assumption: `{:.1} C`",
        config.assumptions.ambient_c
    )?;
    writeln!(
        report,
        "- Heater normal current assumption: `{}` mA; protected current: `{}` mA",
        config.assumptions.continuous_heater_current_ma,
        config.assumptions.protected_heater_current_ma
    )?;
    writeln!(report)?;
    writeln!(report, "## Gate Table")?;
    writeln!(report)?;
    writeln!(
        report,
        "| Category | Item | Measured | Limit | Status | Notes |"
    )?;
    writeln!(report, "| --- | --- | --- | --- | --- | --- |")?;
    for row in rows {
        writeln!(
            report,
            "| {} | {} | {} | {} | {} | {} |",
            md(&row.category),
            md(&row.item),
            md(&row.measured),
            md(&row.limit),
            md(&row.status),
            md(&row.notes)
        )?;
    }
    fs::write(&outputs.report_md, report)?;
    Ok(())
}

fn write_gates_csv(path: &Path, rows: &[GateRow]) -> Result<(), Box<dyn Error>> {
    ensure_parent(path)?;
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["category", "item", "measured", "limit", "status", "notes"])?;
    for row in rows {
        writer.write_record([
            row.category.as_str(),
            row.item.as_str(),
            row.measured.as_str(),
            row.limit.as_str(),
            row.status.as_str(),
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_pdn_current_paths_handoff(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.pdn_current_paths_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.pdn_current_paths_csv)?;
    writer.write_record([
        "path_id",
        "net",
        "current_ma",
        "min_required_width_mm",
        "allowed_layers",
        "routed_segment_count",
        "routed_length_mm",
        "minimum_routed_width_mm",
        "estimated_derated_capacity_ma",
        "notes",
    ])?;

    for path in &config.trace_current_paths {
        let segments = routing
            .segments
            .iter()
            .filter(|segment| segment.net == path.net)
            .collect::<Vec<_>>();
        let routed_length_mm = segments
            .iter()
            .map(|segment| segment_length_mm(segment))
            .sum::<f64>();
        let minimum_routed_width_mm = segments
            .iter()
            .map(|segment| segment.width_mm)
            .reduce(f64::min);
        let estimated_derated_capacity_ma = segments
            .iter()
            .filter(|segment| !allowed_neckdown(segment, path))
            .map(|segment| {
                derated_trace_capacity_ma(segment.width_mm, &segment.layer, &config.assumptions)
            })
            .reduce(f64::min);

        writer.write_record([
            path.id.as_str(),
            path.net.as_str(),
            format!("{}", path.current_ma).as_str(),
            format!("{:.3}", path.min_width_mm).as_str(),
            path.allowed_layers.join(";").as_str(),
            format!("{}", segments.len()).as_str(),
            format!("{routed_length_mm:.3}").as_str(),
            format_optional_mm(minimum_routed_width_mm).as_str(),
            format_optional_ma(estimated_derated_capacity_ma).as_str(),
            path.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_thermal_power_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.thermal_power_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.thermal_power_csv)?;
    writer.write_record([
        "id",
        "component_class",
        "part_id",
        "path_or_rails",
        "current_ma",
        "power_w",
        "estimated_temp_rise_c",
        "limit",
        "notes",
    ])?;

    for regulator in &config.linear_regulators {
        let current_a = regulator.load_ma as f64 / 1000.0;
        let power_w = (regulator.vin_nominal_v - regulator.vout_nominal_v) * current_a;
        let temp_rise_c = power_w * regulator.theta_ja_c_per_w;
        writer.write_record([
            regulator.id.as_str(),
            "linear_regulator",
            regulator.part_id.as_str(),
            format!("{} -> {}", regulator.input_rail, regulator.output_rail).as_str(),
            format!("{}", regulator.load_ma).as_str(),
            format!("{power_w:.4}").as_str(),
            format!("{temp_rise_c:.2}").as_str(),
            format!("<= {:.2} C rise", regulator.max_temp_rise_c).as_str(),
            regulator.notes.as_str(),
        ])?;
    }

    for protection in &config.protection_checks {
        let current_a = protection.current_ma as f64 / 1000.0;
        let power_w = current_a * protection.voltage_drop_v;
        writer.write_record([
            protection.id.as_str(),
            "protection",
            protection.part_id.as_str(),
            protection.path.as_str(),
            format!("{}", protection.current_ma).as_str(),
            format!("{power_w:.4}").as_str(),
            "external thermal model required",
            format!("<= {:.4} W", protection.max_power_w).as_str(),
            protection.notes.as_str(),
        ])?;
    }

    for mosfet in &config.mosfet_checks {
        let current_a = mosfet.continuous_current_ma as f64 / 1000.0;
        let rds_on_ohm = mosfet.rds_on_mohm / 1000.0;
        let power_w = current_a * current_a * rds_on_ohm;
        let temp_rise_c = power_w * mosfet.package_theta_ja_c_per_w;
        writer.write_record([
            mosfet.id.as_str(),
            "mosfet",
            mosfet.part_id.as_str(),
            "HEATER_P low-side switch",
            format!("{}", mosfet.continuous_current_ma).as_str(),
            format!("{power_w:.4}").as_str(),
            format!("{temp_rise_c:.2}").as_str(),
            format!("<= {:.2} C rise", mosfet.max_temp_rise_c).as_str(),
            mosfet.notes.as_str(),
        ])?;
    }

    let heater_current_a = config.assumptions.continuous_heater_current_ma as f64 / 1000.0;
    let heater_power_w = heater_current_a * 12.0;
    writer.write_record([
        "external_heater_load",
        "external_load",
        "external_heater",
        "HEATER_SUPPLY -> HEATER_P",
        format!("{}", config.assumptions.continuous_heater_current_ma).as_str(),
        format!("{heater_power_w:.4}").as_str(),
        "outside PCB; validate with bench thermography",
        "external cutoff required",
        "Heater load is external to the PCB; board thermal review focuses on connector, fuse, diode, MOSFET, and copper.",
    ])?;

    writer.flush()?;
    Ok(())
}

fn write_first_article_measurements_handoff(
    config: &ValidationConfig,
    placement: &PlacementPlan,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.first_article_measurements_csv)?;
    let test_points = placement
        .test_points
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<BTreeMap<_, _>>();
    let mut measurements = config.first_article_measurements.iter().collect::<Vec<_>>();
    measurements.sort_by_key(|measurement| measurement.order);

    let mut writer = csv::Writer::from_path(&outputs.first_article_measurements_csv)?;
    writer.write_record([
        "order",
        "stage",
        "id",
        "test_point",
        "net",
        "x_mm",
        "y_mm",
        "side",
        "measurement",
        "min_v",
        "max_v",
        "current_limit_ma",
        "pass_criterion",
        "notes",
    ])?;

    for measurement in measurements {
        let point = test_points.get(measurement.test_point.as_str());
        writer.write_record([
            format!("{}", measurement.order).as_str(),
            measurement.stage.as_str(),
            measurement.id.as_str(),
            measurement.test_point.as_str(),
            measurement.net.as_str(),
            point
                .map(|point| format!("{:.3}", point.x_mm))
                .unwrap_or_else(|| "missing".to_string())
                .as_str(),
            point
                .map(|point| format!("{:.3}", point.y_mm))
                .unwrap_or_else(|| "missing".to_string())
                .as_str(),
            point.map(|point| point.side.as_str()).unwrap_or("missing"),
            measurement.measurement.as_str(),
            format_optional_v(measurement.min_v).as_str(),
            format_optional_v(measurement.max_v).as_str(),
            format_optional_u32(measurement.current_limit_ma).as_str(),
            measurement.pass_criterion.as_str(),
            measurement.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_component_derating_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.component_derating_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.component_derating_csv)?;
    writer.write_record([
        "id",
        "part_id",
        "stress_class",
        "operating_voltage_v",
        "rated_voltage_v",
        "voltage_utilization_pct",
        "operating_current_ma",
        "rated_current_ma",
        "current_utilization_pct",
        "operating_power_w",
        "rated_power_w",
        "power_utilization_pct",
        "estimated_temp_rise_c",
        "max_temp_rise_c",
        "notes",
    ])?;

    for derating in &config.component_deratings {
        writer.write_record([
            derating.id.as_str(),
            derating.part_id.as_str(),
            derating.stress_class.as_str(),
            format_optional_v(derating.operating_voltage_v).as_str(),
            format_optional_v(derating.rated_voltage_v).as_str(),
            format_optional_pct(ratio(
                derating.operating_voltage_v,
                derating.rated_voltage_v,
            ))
            .as_str(),
            format_optional_u32(derating.operating_current_ma).as_str(),
            format_optional_u32(derating.rated_current_ma).as_str(),
            format_optional_pct(ratio_u32(
                derating.operating_current_ma,
                derating.rated_current_ma,
            ))
            .as_str(),
            format_optional_w(derating.operating_power_w).as_str(),
            format_optional_w(derating.rated_power_w).as_str(),
            format_optional_pct(ratio(derating.operating_power_w, derating.rated_power_w)).as_str(),
            format_optional_v(derating.estimated_temp_rise_c).as_str(),
            format_optional_v(derating.max_temp_rise_c).as_str(),
            derating.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_fault_fmea_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.fault_fmea_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.fault_fmea_csv)?;
    writer.write_record([
        "id",
        "subsystem",
        "failure_mode",
        "cause",
        "local_effect",
        "system_effect",
        "severity",
        "occurrence",
        "detection",
        "rpn",
        "hardware_mitigations",
        "firmware_mitigations",
        "detection_methods",
        "verification_measurements",
        "notes",
    ])?;

    for check in &config.single_fault_checks {
        writer.write_record([
            check.id.as_str(),
            check.subsystem.as_str(),
            check.failure_mode.as_str(),
            check.cause.as_str(),
            check.local_effect.as_str(),
            check.system_effect.as_str(),
            format!("{}", check.severity).as_str(),
            format!("{}", check.occurrence).as_str(),
            format!("{}", check.detection).as_str(),
            format!("{}", fault_rpn(check)).as_str(),
            mitigation_list(&check.hardware_mitigations).as_str(),
            mitigation_list(&check.firmware_mitigations).as_str(),
            mitigation_list(&check.detection_methods).as_str(),
            mitigation_list(&check.verification_measurements).as_str(),
            check.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_emc_esd_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.emc_esd_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.emc_esd_csv)?;
    writer.write_record([
        "id",
        "interface",
        "exposure_class",
        "nets",
        "risk_score",
        "protection_part_ids",
        "return_path_strategy",
        "grounding_strategy",
        "verification_methods",
        "verification_measurements",
        "external_analysis",
        "notes",
    ])?;

    for check in &config.emc_esd_checks {
        writer.write_record([
            check.id.as_str(),
            check.interface.as_str(),
            check.exposure_class.as_str(),
            check.nets.join(";").as_str(),
            format!("{}", check.risk_score).as_str(),
            mitigation_list(&check.protection_part_ids).as_str(),
            check.return_path_strategy.as_str(),
            check.grounding_strategy.as_str(),
            mitigation_list(&check.verification_methods).as_str(),
            mitigation_list(&check.verification_measurements).as_str(),
            mitigation_list(&check.external_analysis).as_str(),
            check.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_startup_safety_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.startup_safety_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.startup_safety_csv)?;
    writer.write_record([
        "id",
        "net",
        "module_pin",
        "function",
        "expected_power_on_state",
        "safe_state",
        "boot_sensitive",
        "safety_critical",
        "allowed_active_at_reset",
        "evidence_ids",
        "verification_methods",
        "verification_measurements",
        "notes",
    ])?;

    for check in &config.boot_startup_checks {
        writer.write_record([
            check.id.as_str(),
            check.net.as_str(),
            format!("{}", check.module_pin).as_str(),
            check.function.as_str(),
            check.expected_power_on_state.as_str(),
            check.safe_state.as_str(),
            format!("{}", check.boot_sensitive).as_str(),
            format!("{}", check.safety_critical).as_str(),
            format!("{}", check.allowed_active_at_reset).as_str(),
            mitigation_list(&check.evidence_ids).as_str(),
            mitigation_list(&check.verification_methods).as_str(),
            mitigation_list(&check.verification_measurements).as_str(),
            check.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_manufacturing_test_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.manufacturing_test_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.manufacturing_test_csv)?;
    writer.write_record([
        "id",
        "subsystem",
        "nets",
        "required_test_points",
        "test_stage",
        "test_method",
        "firmware_test",
        "pass_criterion",
        "verification_measurements",
        "notes",
    ])?;

    for check in &config.manufacturing_test_checks {
        writer.write_record([
            check.id.as_str(),
            check.subsystem.as_str(),
            mitigation_list(&check.nets).as_str(),
            mitigation_list(&check.required_test_points).as_str(),
            check.test_stage.as_str(),
            check.test_method.as_str(),
            check.firmware_test.as_str(),
            check.pass_criterion.as_str(),
            mitigation_list(&check.verification_measurements).as_str(),
            check.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_calibration_readiness_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.calibration_readiness_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.calibration_readiness_csv)?;
    writer.write_record([
        "id",
        "subsystem",
        "calibrated_item",
        "dependent_nets",
        "preconditions",
        "procedure",
        "acceptance_criterion",
        "control_data",
        "required_outputs",
        "verification_measurements",
        "firmware_dependency",
        "notes",
    ])?;

    for check in &config.calibration_checks {
        writer.write_record([
            check.id.as_str(),
            check.subsystem.as_str(),
            check.calibrated_item.as_str(),
            mitigation_list(&check.dependent_nets).as_str(),
            mitigation_list(&check.preconditions).as_str(),
            check.procedure.as_str(),
            check.acceptance_criterion.as_str(),
            mitigation_list(&check.control_data).as_str(),
            mitigation_list(&check.required_outputs).as_str(),
            mitigation_list(&check.verification_measurements).as_str(),
            check.firmware_dependency.as_str(),
            check.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_validation_traceability_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.validation_traceability_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.validation_traceability_csv)?;
    writer.write_record([
        "id",
        "validation_layer",
        "electrical_gate_category",
        "output_artifact",
        "release_manifest_entry",
        "ci_step",
        "acceptance_criterion",
        "blocks_release",
        "notes",
    ])?;

    for item in &config.validation_traceability {
        let blocks_release = item.blocks_release.to_string();
        writer.write_record([
            item.id.as_str(),
            item.validation_layer.as_str(),
            item.electrical_gate_category.as_deref().unwrap_or("n/a"),
            item.output_artifact.as_str(),
            item.release_manifest_entry.as_str(),
            item.ci_step.as_str(),
            item.acceptance_criterion.as_str(),
            blocks_release.as_str(),
            item.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn format_optional_v(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_optional_w(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.4}"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_optional_pct(value: Option<f64>) -> String {
    value
        .map(|value| format!("{:.1}", value * 100.0))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_optional_u32(value: Option<u32>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}

fn ratio(operating: Option<f64>, rated: Option<f64>) -> Option<f64> {
    match (operating, rated) {
        (Some(operating), Some(rated)) if rated > 0.0 => Some(operating / rated),
        _ => None,
    }
}

fn ratio_u32(operating: Option<u32>, rated: Option<u32>) -> Option<f64> {
    match (operating, rated) {
        (Some(operating), Some(rated)) if rated > 0 => Some(operating as f64 / rated as f64),
        _ => None,
    }
}

fn format_optional_mm(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
        .unwrap_or_else(|| "missing".to_string())
}

fn format_optional_ma(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.0}"))
        .unwrap_or_else(|| "missing".to_string())
}

fn write_spice_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.spice_netlist)?;
    let rails = config
        .rail_budgets
        .iter()
        .map(|budget| (budget.rail.as_str(), budget))
        .collect::<BTreeMap<_, _>>();
    let heater_current_a = config.assumptions.continuous_heater_current_ma as f64 / 1000.0;
    let heater_supply_v = rails
        .get("HEATER_SUPPLY")
        .map(|rail| rail.nominal_v)
        .unwrap_or(11.6);
    let heater_resistance = heater_supply_v / heater_current_a;
    let three_v_three_load = rails
        .get("+3V3")
        .map(|rail| rail.expected_continuous_ma as f64 / 1000.0)
        .unwrap_or(0.3);
    let three_v_three_resistance = 3.3 / three_v_three_load;
    let five_v_load = rails
        .get("+5V")
        .map(|rail| rail.expected_continuous_ma as f64 / 1000.0)
        .unwrap_or(0.45);
    let five_v_nominal = rails.get("+5V").map(|rail| rail.nominal_v).unwrap_or(4.6);
    let five_v_resistance = five_v_nominal / five_v_load;

    let mut spice = String::new();
    writeln!(
        spice,
        "* LaminarForge LAMP Rev A first-order power path handoff"
    )?;
    writeln!(spice, "* Generated by lamp_rev_a_electrical_validate")?;
    writeln!(
        spice,
        "* This is an operating-point starting point for ngspice, not signoff."
    )?;
    writeln!(spice, "VUSB VBUS 0 DC 5.0")?;
    writeln!(spice, "DVBUS VBUS P5V SS34")?;
    writeln!(spice, "R5V P5V 0 {five_v_resistance:.3}")?;
    writeln!(spice, "V3V3 P3V3 0 DC 3.3")?;
    writeln!(spice, "R3V3 P3V3 0 {three_v_three_resistance:.3}")?;
    writeln!(spice, "V12 P12RAW 0 DC 12.0")?;
    writeln!(spice, "D12 P12RAW P12 SS34")?;
    writeln!(spice, "RPTC P12 HEATER_SUPPLY 0.100")?;
    writeln!(
        spice,
        "RHEATER HEATER_SUPPLY HEATER_P {heater_resistance:.3}"
    )?;
    writeln!(spice, "RMOSFET HEATER_P 0 0.052")?;
    writeln!(spice, ".model SS34 D(Is=1e-8 Rs=0.050 N=1.05 Cjo=200p)")?;
    writeln!(spice, ".op")?;
    writeln!(spice, ".end")?;
    fs::write(&outputs.spice_netlist, spice)?;
    Ok(())
}

fn write_simulation_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.simulation_handoff_md)?;
    let mut report = String::new();
    writeln!(report, "# LAMP Rev A Simulation / Analysis Handoff")?;
    writeln!(report)?;
    writeln!(
        report,
        "This file lists the pre-fab analysis methods that sit above KiCad ERC/DRC."
    )?;
    writeln!(report)?;
    for handoff in &config.external_analysis_handoffs {
        writeln!(report, "## {}", handoff.id)?;
        writeln!(report)?;
        writeln!(report, "- Tool class: `{}`", handoff.tool_class)?;
        writeln!(
            report,
            "- Recommended tools: {}",
            handoff.recommended_tools.join(", ")
        )?;
        writeln!(report, "- Inputs: {}", handoff.inputs.join(", "))?;
        writeln!(report, "- Exit criterion: {}", handoff.exit_criterion)?;
        writeln!(report)?;
    }
    writeln!(report, "## Generated SPICE Handoff")?;
    writeln!(report)?;
    writeln!(
        report,
        "- Netlist: `{}`",
        outputs
            .spice_netlist
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lamp_rev_a_power_path.spice")
    )?;
    writeln!(
        report,
        "- Suggested command: `cargo run --release --bin lamp_rev_a_spice_check`"
    )?;
    writeln!(report)?;
    writeln!(report, "## Generated PDN / Thermal Handoffs")?;
    writeln!(report)?;
    writeln!(
        report,
        "- PDN current-path table: `{}`",
        outputs
            .pdn_current_paths_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("pdn_current_paths.csv")
    )?;
    writeln!(
        report,
        "- Thermal power table: `{}`",
        outputs
            .thermal_power_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("thermal_power_budget.csv")
    )?;
    writeln!(
        report,
        "- First-article measurement table: `{}`",
        outputs
            .first_article_measurements_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("first_article_measurements.csv")
    )?;
    writeln!(
        report,
        "- Component derating table: `{}`",
        outputs
            .component_derating_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("component_derating.csv")
    )?;
    writeln!(
        report,
        "- Single-fault FMEA table: `{}`",
        outputs
            .fault_fmea_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("fault_fmea.csv")
    )?;
    writeln!(
        report,
        "- EMC/ESD pre-compliance table: `{}`",
        outputs
            .emc_esd_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("emc_esd_precompliance.csv")
    )?;
    writeln!(
        report,
        "- Startup safety table: `{}`",
        outputs
            .startup_safety_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("startup_safety.csv")
    )?;
    writeln!(
        report,
        "- Manufacturing test coverage table: `{}`",
        outputs
            .manufacturing_test_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("manufacturing_test_coverage.csv")
    )?;
    writeln!(
        report,
        "- Calibration readiness table: `{}`",
        outputs
            .calibration_readiness_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("calibration_readiness.csv")
    )?;
    writeln!(
        report,
        "- Validation traceability matrix: `{}`",
        outputs
            .validation_traceability_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("validation_traceability.csv")
    )?;
    fs::write(&outputs.simulation_handoff_md, report)?;
    Ok(())
}

fn summarize_rows(rows: &[GateRow]) -> ElectricalValidationSummary {
    ElectricalValidationSummary {
        pass_count: rows.iter().filter(|row| row.status == "pass").count(),
        fail_count: rows.iter().filter(|row| row.status == "fail").count(),
        manual_count: rows.iter().filter(|row| row.status == "manual").count(),
    }
}

fn nearly_equal(left: f64, right: f64) -> bool {
    (left - right).abs() < 0.001
}

fn ensure_parent(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn md(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
