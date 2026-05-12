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
    #[serde(default)]
    external_analysis_handoffs: Vec<ExternalAnalysisHandoff>,
    #[serde(default)]
    manual_first_article_gates: Vec<ManualFirstArticleGate>,
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
    power_architecture: String,
    optical_architecture: String,
}

#[derive(Debug, Deserialize)]
struct Outputs {
    report_md: String,
    gates_csv: String,
    spice_netlist: String,
    simulation_handoff_md: String,
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

#[derive(Debug)]
struct GateRow {
    category: String,
    item: String,
    measured: String,
    limit: String,
    status: String,
    notes: String,
}

pub fn default_output_paths(repo_root: &Path) -> Result<ElectricalOutputPaths, Box<dyn Error>> {
    let config = read_validation_config(repo_root)?;
    Ok(ElectricalOutputPaths {
        report_md: repo_root.join(config.outputs.report_md),
        gates_csv: repo_root.join(config.outputs.gates_csv),
        spice_netlist: repo_root.join(config.outputs.spice_netlist),
        simulation_handoff_md: repo_root.join(config.outputs.simulation_handoff_md),
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
    validate_gpio_domains(&config, &pin_nets, &firmware, &mut rows, &mut errors);
    validate_analog_ranges(&config, &contract_nets, &mut rows, &mut errors);
    add_external_analysis_rows(&config, &mut rows);
    add_manual_gate_rows(&config, &parts, &mut rows, &mut errors);

    write_report(&config, outputs, &rows)?;
    write_gates_csv(&outputs.gates_csv, &rows)?;
    write_spice_handoff(&config, outputs)?;
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
    push_gate(
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
    push_gate(
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
    push_gate(
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
        push_gate(
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
            push_gate(
                rows,
                errors,
                "rail budget",
                format!("{} current budget", budget.rail),
                format!("{} mA expected", budget.expected_continuous_ma),
                format!("<= {} mA contract max", rail.max_current_ma),
                budget.expected_continuous_ma <= rail.max_current_ma,
                &budget.notes,
            );
            push_gate(
                rows,
                errors,
                "rail budget",
                format!("{} source limit", budget.rail),
                format!("{} mA source", budget.source_limit_ma),
                format!("<= {} mA contract max", rail.max_current_ma),
                budget.source_limit_ma <= rail.max_current_ma,
                &budget.notes,
            );
            push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
            rows,
            errors,
            "linear regulator",
            format!("{} current", regulator.id),
            format!("{} mA", regulator.load_ma),
            format!("<= {} mA rated", regulator.current_rating_ma),
            regulator.load_ma <= regulator.current_rating_ma,
            &regulator.notes,
        );
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
        push_gate(
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
            push_gate(
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
            push_gate(
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

        push_gate(
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
        push_gate(
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
        push_gate(
            rows,
            errors,
            "gpio domain",
            format!("module pin {} handoff", pin.module_pin),
            pin.net.clone(),
            "matches pin_nets.toml",
            in_pin_nets,
            &config.gpio_domain.notes,
        );
        push_gate(
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
        push_gate(
            rows,
            errors,
            "analog range",
            format!("{} net exists", check.id),
            check.net.clone(),
            "present in contract",
            contract_nets.contains(&check.net),
            &check.notes,
        );
        push_gate(
            rows,
            errors,
            "analog range",
            format!("{} expected input", check.id),
            format!("{:.2} V", check.max_expected_v),
            format!("<= {:.2} V abs max", check.absolute_max_v),
            check.max_expected_v <= check.absolute_max_v,
            &check.notes,
        );
        push_gate(
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
    push_gate(
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

fn push_gate(
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
    category: impl Into<String>,
    item: impl Into<String>,
    measured: impl Into<String>,
    limit: impl Into<String>,
    pass: bool,
    notes: &str,
) {
    let category = category.into();
    let item = item.into();
    let measured = measured.into();
    let limit = limit.into();
    let status = if pass { "pass" } else { "fail" }.to_string();
    if !pass {
        errors.push(format!(
            "{category}: {item} measured {measured}; limit {limit}"
        ));
    }
    rows.push(GateRow {
        category,
        item,
        measured,
        limit,
        status,
        notes: notes.to_string(),
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
        "- Suggested command: `ngspice lamp_rev_a_power_path.spice`"
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
