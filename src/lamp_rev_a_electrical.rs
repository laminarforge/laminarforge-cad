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
    pub simulation_inputs_csv: PathBuf,
    pub pdn_current_paths_csv: PathBuf,
    pub thermal_power_csv: PathBuf,
    pub first_article_measurements_csv: PathBuf,
    pub component_derating_csv: PathBuf,
    pub fault_fmea_csv: PathBuf,
    pub emc_esd_csv: PathBuf,
    pub i2c_bus_csv: PathBuf,
    pub heater_protection_csv: PathBuf,
    pub startup_safety_csv: PathBuf,
    pub manufacturing_test_csv: PathBuf,
    pub calibration_readiness_csv: PathBuf,
    pub validation_traceability_csv: PathBuf,
    pub pdn_dc_simulation_csv: PathBuf,
    pub thermal_margin_simulation_csv: PathBuf,
    pub heater_pwm_transient_netlist: PathBuf,
    pub heater_pwm_transient_csv: PathBuf,
    pub heater_thermal_transient_csv: PathBuf,
    pub usb_inrush_startup_csv: PathBuf,
    pub power_domain_fault_netlist: PathBuf,
    pub power_domain_fault_csv: PathBuf,
    pub rail_load_step_netlist: PathBuf,
    pub rail_load_step_csv: PathBuf,
    pub analog_front_end_netlist: PathBuf,
    pub analog_front_end_csv: PathBuf,
    pub optical_crosstalk_csv: PathBuf,
    pub optical_noise_margin_csv: PathBuf,
    pub thermistor_adc_transfer_csv: PathBuf,
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
    simulation_input_policy: SimulationInputPolicy,
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
    i2c_bus_policy: I2cBusPolicy,
    heater_protection_policy: HeaterProtectionPolicy,
    #[serde(default)]
    heater_protection_checks: Vec<HeaterProtectionCheck>,
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
    pdn_dc_simulation_policy: PdnDcSimulationPolicy,
    thermal_margin_simulation_policy: ThermalMarginSimulationPolicy,
    heater_pwm_transient_policy: HeaterPwmTransientPolicy,
    heater_thermal_transient_policy: HeaterThermalTransientPolicy,
    usb_inrush_startup_policy: UsbInrushStartupPolicy,
    power_domain_fault_policy: PowerDomainFaultPolicy,
    rail_load_step_policy: RailLoadStepPolicy,
    analog_front_end_policy: AnalogFrontEndPolicy,
    optical_crosstalk_policy: OpticalCrosstalkPolicy,
    optical_noise_margin_policy: OpticalNoiseMarginPolicy,
    thermistor_adc_policy: ThermistorAdcPolicy,
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
    simulation_inputs_csv: String,
    pdn_current_paths_csv: String,
    thermal_power_csv: String,
    first_article_measurements_csv: String,
    component_derating_csv: String,
    fault_fmea_csv: String,
    emc_esd_csv: String,
    i2c_bus_csv: String,
    heater_protection_csv: String,
    startup_safety_csv: String,
    manufacturing_test_csv: String,
    calibration_readiness_csv: String,
    validation_traceability_csv: String,
    pdn_dc_simulation_csv: String,
    thermal_margin_simulation_csv: String,
    heater_pwm_transient_netlist: String,
    heater_pwm_transient_csv: String,
    heater_thermal_transient_csv: String,
    usb_inrush_startup_csv: String,
    power_domain_fault_netlist: String,
    power_domain_fault_csv: String,
    rail_load_step_netlist: String,
    rail_load_step_csv: String,
    analog_front_end_netlist: String,
    analog_front_end_csv: String,
    optical_crosstalk_csv: String,
    optical_noise_margin_csv: String,
    thermistor_adc_transfer_csv: String,
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
struct SimulationInputPolicy {
    min_handoffs: usize,
    required_handoff_ids: Vec<String>,
    require_recommended_tools: bool,
    require_inputs: bool,
    require_exit_criterion: bool,
}

#[derive(Debug, Deserialize)]
struct PdnDcSimulationPolicy {
    max_voltage_drop_mv: f64,
    max_voltage_drop_pct: f64,
    max_path_power_mw: f64,
    via_resistance_mohm: f64,
    copper_resistivity_ohm_m: f64,
    copper_thickness_um_per_oz: f64,
    require_all_trace_paths: bool,
}

#[derive(Debug, Deserialize)]
struct ThermalMarginSimulationPolicy {
    max_component_temp_c: f64,
    max_trace_temp_c: f64,
    max_total_board_power_w: f64,
    protection_theta_c_per_w: f64,
    trace_theta_c_per_w: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct HeaterPwmTransientPolicy {
    supply_voltage_v: f64,
    heater_resistance_ohm: f64,
    heater_series_resistance_ohm: f64,
    mosfet_rds_on_ohm: f64,
    mosfet_off_resistance_ohm: f64,
    gate_drive_high_v: f64,
    gate_drive_low_v: f64,
    gate_threshold_v: f64,
    gate_hysteresis_v: f64,
    pwm_period_ms: f64,
    pwm_on_ms: f64,
    simulation_stop_ms: f64,
    max_on_current_a: f64,
    min_on_current_a: f64,
    max_off_current_ma: f64,
    min_gate_high_v: f64,
    max_gate_low_v: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct HeaterThermalTransientPolicy {
    ambient_c: f64,
    target_c: f64,
    target_reached_c: f64,
    max_temperature_c: f64,
    max_overshoot_c: f64,
    max_warmup_s: f64,
    max_hold_error_c: f64,
    hold_window_s: f64,
    simulation_stop_s: f64,
    timestep_s: f64,
    heater_power_w: f64,
    thermal_mass_j_per_c: f64,
    thermal_resistance_c_per_w: f64,
    controller_band_c: f64,
    proportional_gain_per_c: f64,
    max_duty: f64,
    min_hold_duty: f64,
    max_hold_duty: f64,
    max_energy_wh: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct UsbInrushStartupPolicy {
    vbus_nominal_v: f64,
    usb_source_resistance_ohm: f64,
    vbus_bulk_cap_u: f64,
    five_v_bulk_cap_u: f64,
    three_v_three_bulk_cap_u: f64,
    five_v_load_ma: f64,
    three_v_three_load_ma: f64,
    schottky_drop_v: f64,
    schottky_series_resistance_ohm: f64,
    regulator_dropout_v: f64,
    regulator_series_resistance_ohm: f64,
    regulated_3v3_v: f64,
    simulation_stop_ms: f64,
    timestep_ms: f64,
    max_vbus_inrush_ma: f64,
    max_5v_inrush_ma: f64,
    min_final_3v3_v: f64,
    max_final_3v3_v: f64,
    max_3v3_overshoot_v: f64,
    max_startup_ms: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct PowerDomainFaultPolicy {
    heater_fault_voltage_v: f64,
    vbus_load_ohm: f64,
    five_v_load_ohm: f64,
    three_v_three_load_ohm: f64,
    heater_to_vbus_isolation_ohm: f64,
    heater_to_five_v_isolation_ohm: f64,
    heater_to_three_v_three_isolation_ohm: f64,
    regulator_reverse_resistance_ohm: f64,
    simulation_stop_ms: f64,
    max_vbus_fault_v: f64,
    max_five_v_fault_v: f64,
    max_three_v_three_fault_v: f64,
    max_usb_backfeed_ma: f64,
    max_regulator_reverse_ma: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct RailLoadStepPolicy {
    nominal_voltage_v: f64,
    source_resistance_ohm: f64,
    bulk_capacitance_u: f64,
    baseline_load_ma: u32,
    burst_load_ma: u32,
    burst_start_ms: f64,
    burst_width_ms: f64,
    period_ms: f64,
    simulation_stop_ms: f64,
    min_rail_voltage_v: f64,
    max_source_current_ma: u32,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct AnalogFrontEndPolicy {
    adc_rail_v: f64,
    feedback_resistor_ohm: f64,
    feedback_cap_pf: f64,
    mux_on_resistance_ohm: f64,
    input_cap_pf: f64,
    dark_current_na: f64,
    light_current_na: f64,
    max_photocurrent_na: f64,
    light_start_ms: f64,
    light_width_ms: f64,
    period_ms: f64,
    simulation_stop_ms: f64,
    min_signal_delta_v: f64,
    min_adc_voltage_v: f64,
    max_adc_voltage_v: f64,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct OpticalCrosstalkPolicy {
    slot_count: usize,
    led_net_prefix: String,
    mux_select_nets: Vec<String>,
    mux_common_net: String,
    adc_net: String,
    max_adjacent_crosstalk_pct: f64,
    max_non_adjacent_crosstalk_pct: f64,
    max_crosstalk_delta_v: f64,
    mux_off_leakage_na: f64,
    max_dark_shift_v: f64,
    tia_output_impedance_ohm: f64,
    adc_sample_cap_pf: f64,
    settling_error_pct: f64,
    sample_settle_us: f64,
    required_measurements: Vec<String>,
    required_outputs: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct OpticalNoiseMarginPolicy {
    adc_full_scale_v: f64,
    adc_counts: u32,
    analog_noise_rms_mv: f64,
    adc_noise_rms_counts: f64,
    quantization_guard_lsb: f64,
    negative_control_drift_mv: f64,
    threshold_fraction: f64,
    min_signal_to_noise_ratio: f64,
    min_threshold_margin_v: f64,
    required_measurements: Vec<String>,
    required_outputs: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct ThermistorAdcPolicy {
    adc_net: String,
    rail: String,
    test_point: String,
    pullup_part_id: String,
    adc_part_id: String,
    rail_voltage_v: f64,
    pullup_ohm: f64,
    thermistor_nominal_ohm: f64,
    beta_k: f64,
    reference_temp_c: f64,
    adc_full_scale_v: f64,
    adc_counts: u32,
    min_temp_c: f64,
    max_temp_c: f64,
    target_temp_c: f64,
    sample_temps_c: Vec<f64>,
    min_operating_counts: f64,
    max_operating_counts: f64,
    min_counts_per_c_at_target: f64,
    open_fault_min_v: f64,
    short_fault_max_v: f64,
    notes: String,
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
struct I2cBusPolicy {
    sda_net: String,
    scl_net: String,
    pullup_part_id: String,
    adc_part_id: String,
    required_device: String,
    required_address: String,
    pullup_ohm: f64,
    bus_capacitance_pf: f64,
    bus_speed_hz: u32,
    max_bus_speed_hz: u32,
    max_rise_time_ns: f64,
    rail_v: f64,
    min_idle_high_v: f64,
    required_test_points: Vec<String>,
    required_measurements: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct HeaterProtectionPolicy {
    min_checks: usize,
    continuous_current_ma: u32,
    protected_current_ma: u32,
    max_fault_current_ma: u32,
    require_external_cutoff: bool,
    required_evidence_ids: Vec<String>,
    required_measurements: Vec<String>,
    notes: String,
}

#[derive(Debug, Deserialize)]
struct HeaterProtectionCheck {
    id: String,
    stage: String,
    current_ma: u32,
    required_parts: Vec<String>,
    required_paths: Vec<String>,
    required_measurements: Vec<String>,
    trip_or_limit: String,
    pass_criterion: String,
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
    peripherals: FirmwarePeripherals,
    module_pins: Vec<FirmwareModulePin>,
}

#[derive(Debug, Deserialize)]
struct FirmwareMcu {
    reference: String,
}

#[derive(Debug, Deserialize)]
struct FirmwarePeripherals {
    i2c_sda_net: String,
    i2c_scl_net: String,
    adc_device: String,
    adc_i2c_address: String,
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
        simulation_inputs_csv: repo_root.join(config.outputs.simulation_inputs_csv),
        pdn_current_paths_csv: repo_root.join(config.outputs.pdn_current_paths_csv),
        thermal_power_csv: repo_root.join(config.outputs.thermal_power_csv),
        first_article_measurements_csv: repo_root
            .join(config.outputs.first_article_measurements_csv),
        component_derating_csv: repo_root.join(config.outputs.component_derating_csv),
        fault_fmea_csv: repo_root.join(config.outputs.fault_fmea_csv),
        emc_esd_csv: repo_root.join(config.outputs.emc_esd_csv),
        i2c_bus_csv: repo_root.join(config.outputs.i2c_bus_csv),
        heater_protection_csv: repo_root.join(config.outputs.heater_protection_csv),
        startup_safety_csv: repo_root.join(config.outputs.startup_safety_csv),
        manufacturing_test_csv: repo_root.join(config.outputs.manufacturing_test_csv),
        calibration_readiness_csv: repo_root.join(config.outputs.calibration_readiness_csv),
        validation_traceability_csv: repo_root.join(config.outputs.validation_traceability_csv),
        pdn_dc_simulation_csv: repo_root.join(config.outputs.pdn_dc_simulation_csv),
        thermal_margin_simulation_csv: repo_root.join(config.outputs.thermal_margin_simulation_csv),
        heater_pwm_transient_netlist: repo_root.join(config.outputs.heater_pwm_transient_netlist),
        heater_pwm_transient_csv: repo_root.join(config.outputs.heater_pwm_transient_csv),
        heater_thermal_transient_csv: repo_root.join(config.outputs.heater_thermal_transient_csv),
        usb_inrush_startup_csv: repo_root.join(config.outputs.usb_inrush_startup_csv),
        power_domain_fault_netlist: repo_root.join(config.outputs.power_domain_fault_netlist),
        power_domain_fault_csv: repo_root.join(config.outputs.power_domain_fault_csv),
        rail_load_step_netlist: repo_root.join(config.outputs.rail_load_step_netlist),
        rail_load_step_csv: repo_root.join(config.outputs.rail_load_step_csv),
        analog_front_end_netlist: repo_root.join(config.outputs.analog_front_end_netlist),
        analog_front_end_csv: repo_root.join(config.outputs.analog_front_end_csv),
        optical_crosstalk_csv: repo_root.join(config.outputs.optical_crosstalk_csv),
        optical_noise_margin_csv: repo_root.join(config.outputs.optical_noise_margin_csv),
        thermistor_adc_transfer_csv: repo_root.join(config.outputs.thermistor_adc_transfer_csv),
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
    validate_i2c_bus(
        &config,
        &parts,
        &contract_nets,
        &firmware,
        &placement,
        &mut rows,
        &mut errors,
    );
    validate_heater_protection(&config, &parts, &mut rows, &mut errors);
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
    validate_simulation_inputs(&config, &mut rows, &mut errors);
    validate_pdn_dc_simulation(&config, &routing, &contract_rails, &mut rows, &mut errors);
    validate_thermal_margin_simulation(&config, &routing, &mut rows, &mut errors);
    validate_heater_pwm_transient(&config, &contract_nets, &mut rows, &mut errors);
    validate_heater_thermal_transient(&config, &contract_nets, &mut rows, &mut errors);
    validate_usb_inrush_startup(&config, &contract_nets, &mut rows, &mut errors);
    validate_power_domain_fault(&config, &contract_nets, &mut rows, &mut errors);
    validate_rail_load_step(&config, &contract_nets, &mut rows, &mut errors);
    validate_analog_front_end_transient(&config, &contract_nets, &mut rows, &mut errors);
    validate_optical_crosstalk(&config, &contract_nets, &mut rows, &mut errors);
    validate_optical_noise_margin(&config, &mut rows, &mut errors);
    validate_thermistor_adc_transfer(
        &config,
        &parts,
        &placement,
        &contract_nets,
        &mut rows,
        &mut errors,
    );
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
    write_i2c_bus_handoff(&config, &parts, &firmware, &placement, outputs)?;
    write_heater_protection_handoff(&config, outputs)?;
    write_startup_safety_handoff(&config, outputs)?;
    write_manufacturing_test_handoff(&config, outputs)?;
    write_calibration_readiness_handoff(&config, outputs)?;
    write_validation_traceability_handoff(&config, outputs)?;
    write_simulation_inputs_handoff(&config, outputs)?;
    write_pdn_dc_simulation_handoff(&config, &routing, &contract_rails, outputs)?;
    write_thermal_margin_simulation_handoff(&config, &routing, outputs)?;
    write_heater_pwm_transient_handoff(&config, outputs)?;
    write_heater_thermal_transient_handoff(&config, outputs)?;
    write_usb_inrush_startup_handoff(&config, outputs)?;
    write_power_domain_fault_handoff(&config, outputs)?;
    write_rail_load_step_handoff(&config, outputs)?;
    write_analog_front_end_handoff(&config, outputs)?;
    write_optical_crosstalk_handoff(&config, outputs)?;
    write_optical_noise_margin_handoff(&config, outputs)?;
    write_thermistor_adc_transfer_handoff(&config, outputs)?;
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

fn validate_pdn_dc_simulation(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    _contract_rails: &BTreeMap<&str, &ContractRail>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let rail_budgets = config
        .rail_budgets
        .iter()
        .map(|rail| (rail.rail.as_str(), rail))
        .collect::<BTreeMap<_, _>>();
    let simulations = pdn_dc_simulation_rows(config, routing);
    for simulation in &simulations {
        push_gate!(
            rows,
            errors,
            "pdn dc simulation",
            format!("{} rail budget", simulation.path_id),
            simulation.net.clone(),
            "present in electrical rail budgets",
            rail_budgets.contains_key(simulation.net.as_str()),
            &simulation.notes,
        );
        push_gate!(
            rows,
            errors,
            "pdn dc simulation",
            format!("{} routed geometry", simulation.path_id),
            format!("{} segments", simulation.segment_count),
            "> 0 routed segments",
            !config.pdn_dc_simulation_policy.require_all_trace_paths
                || simulation.segment_count > 0,
            &simulation.notes,
        );
        push_gate!(
            rows,
            errors,
            "pdn dc simulation",
            format!("{} voltage drop", simulation.path_id),
            format!("{:.2} mV", simulation.voltage_drop_mv),
            format!(
                "<= {:.2} mV",
                config.pdn_dc_simulation_policy.max_voltage_drop_mv
            ),
            simulation.segment_count > 0
                && simulation.voltage_drop_mv
                    <= config.pdn_dc_simulation_policy.max_voltage_drop_mv,
            &simulation.notes,
        );
        push_gate!(
            rows,
            errors,
            "pdn dc simulation",
            format!("{} voltage drop percent", simulation.path_id),
            format!("{:.3}%", simulation.voltage_drop_pct),
            format!(
                "<= {:.3}%",
                config.pdn_dc_simulation_policy.max_voltage_drop_pct
            ),
            simulation.segment_count > 0
                && simulation.voltage_drop_pct
                    <= config.pdn_dc_simulation_policy.max_voltage_drop_pct,
            &simulation.notes,
        );
        push_gate!(
            rows,
            errors,
            "pdn dc simulation",
            format!("{} I2R power", simulation.path_id),
            format!("{:.3} mW", simulation.path_power_mw),
            format!(
                "<= {:.3} mW",
                config.pdn_dc_simulation_policy.max_path_power_mw
            ),
            simulation.segment_count > 0
                && simulation.path_power_mw <= config.pdn_dc_simulation_policy.max_path_power_mw,
            &simulation.notes,
        );
    }
}

#[derive(Debug)]
struct PdnDcSimulationRow {
    path_id: String,
    net: String,
    current_ma: u32,
    nominal_voltage_v: f64,
    segment_count: usize,
    routed_length_mm: f64,
    minimum_width_mm: Option<f64>,
    via_count: usize,
    trace_resistance_mohm: f64,
    via_resistance_mohm: f64,
    total_resistance_mohm: f64,
    voltage_drop_mv: f64,
    voltage_drop_pct: f64,
    path_power_mw: f64,
    notes: String,
}

fn pdn_dc_simulation_rows(
    config: &ValidationConfig,
    routing: &RoutingSeed,
) -> Vec<PdnDcSimulationRow> {
    let rail_budgets = config
        .rail_budgets
        .iter()
        .map(|rail| (rail.rail.as_str(), rail))
        .collect::<BTreeMap<_, _>>();
    config
        .trace_current_paths
        .iter()
        .map(|path| pdn_dc_simulation_row(config, routing, &rail_budgets, path))
        .collect()
}

fn pdn_dc_simulation_row(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    rail_budgets: &BTreeMap<&str, &RailBudget>,
    path: &TraceCurrentPath,
) -> PdnDcSimulationRow {
    let segments = routing
        .segments
        .iter()
        .filter(|segment| segment.net == path.net)
        .collect::<Vec<_>>();
    let mut vias = BTreeSet::new();
    let mut trace_resistance_mohm = 0.0;
    let mut routed_length_mm = 0.0;
    let mut minimum_width_mm: Option<f64> = None;

    for segment in &segments {
        routed_length_mm += segment_length_mm(segment);
        minimum_width_mm =
            Some(minimum_width_mm.map_or(segment.width_mm, |width| width.min(segment.width_mm)));
        trace_resistance_mohm += segment_resistance_mohm(segment, config);
        if segment.via_at_ends || segment.via_at_start {
            vias.insert(route_point_key(segment.start_x_mm, segment.start_y_mm));
        }
        if segment.via_at_ends || segment.via_at_end {
            vias.insert(route_point_key(segment.end_x_mm, segment.end_y_mm));
        }
    }

    let via_resistance_mohm =
        vias.len() as f64 * config.pdn_dc_simulation_policy.via_resistance_mohm;
    let total_resistance_mohm = trace_resistance_mohm + via_resistance_mohm;
    let current_a = path.current_ma as f64 / 1000.0;
    let voltage_drop_mv = current_a * total_resistance_mohm;
    let nominal_voltage_v = rail_budgets
        .get(path.net.as_str())
        .map(|rail| rail.nominal_v)
        .unwrap_or(0.0);
    let voltage_drop_pct = if nominal_voltage_v > 0.0 {
        voltage_drop_mv / (nominal_voltage_v * 1000.0) * 100.0
    } else {
        f64::INFINITY
    };
    let path_power_mw = current_a * current_a * total_resistance_mohm;

    PdnDcSimulationRow {
        path_id: path.id.clone(),
        net: path.net.clone(),
        current_ma: path.current_ma,
        nominal_voltage_v,
        segment_count: segments.len(),
        routed_length_mm,
        minimum_width_mm,
        via_count: vias.len(),
        trace_resistance_mohm,
        via_resistance_mohm,
        total_resistance_mohm,
        voltage_drop_mv,
        voltage_drop_pct,
        path_power_mw,
        notes: path.notes.clone(),
    }
}

fn segment_resistance_mohm(segment: &RouteSegment, config: &ValidationConfig) -> f64 {
    let length_m = segment_length_mm(segment) / 1000.0;
    let width_m = segment.width_mm / 1000.0;
    let copper_oz = if segment.layer.starts_with("In") {
        config.assumptions.inner_copper_oz
    } else {
        config.assumptions.outer_copper_oz
    };
    let thickness_m = copper_oz * config.pdn_dc_simulation_policy.copper_thickness_um_per_oz * 1e-6;
    if width_m <= 0.0 || thickness_m <= 0.0 {
        return f64::INFINITY;
    }
    let resistance_ohm = config.pdn_dc_simulation_policy.copper_resistivity_ohm_m * length_m
        / (width_m * thickness_m);
    resistance_ohm * 1000.0
}

fn validate_thermal_margin_simulation(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let thermal_rows = thermal_margin_simulation_rows(config, routing);
    let total_board_power_w = thermal_rows.iter().map(|row| row.power_w).sum::<f64>();

    for row in &thermal_rows {
        push_gate!(
            rows,
            errors,
            "thermal margin simulation",
            format!("{} steady-state temperature", row.id),
            format!("{:.2} C", row.estimated_temp_c),
            format!("<= {:.2} C", row.max_temp_c),
            row.estimated_temp_c <= row.max_temp_c,
            &row.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "thermal margin simulation",
        "total modeled board heat",
        format!("{total_board_power_w:.4} W"),
        format!(
            "<= {:.4} W",
            config
                .thermal_margin_simulation_policy
                .max_total_board_power_w
        ),
        total_board_power_w
            <= config
                .thermal_margin_simulation_policy
                .max_total_board_power_w,
        &config.thermal_margin_simulation_policy.notes,
    );
}

#[derive(Debug)]
struct ThermalMarginRow {
    id: String,
    source_class: String,
    source_id: String,
    power_w: f64,
    theta_c_per_w: f64,
    temp_rise_c: f64,
    estimated_temp_c: f64,
    max_temp_c: f64,
    margin_c: f64,
    notes: String,
}

fn thermal_margin_simulation_rows(
    config: &ValidationConfig,
    routing: &RoutingSeed,
) -> Vec<ThermalMarginRow> {
    let mut rows = Vec::new();
    for regulator in &config.linear_regulators {
        let current_a = regulator.load_ma as f64 / 1000.0;
        let power_w = (regulator.vin_nominal_v - regulator.vout_nominal_v) * current_a;
        rows.push(thermal_margin_row(
            config,
            ThermalMarginSpec {
                id: regulator.id.clone(),
                source_class: "linear_regulator",
                source_id: regulator.part_id.clone(),
                power_w,
                theta_c_per_w: regulator.theta_ja_c_per_w,
                max_temp_c: config.thermal_margin_simulation_policy.max_component_temp_c,
                notes: regulator.notes.clone(),
            },
        ));
    }

    for protection in &config.protection_checks {
        let current_a = protection.current_ma as f64 / 1000.0;
        let power_w = current_a * protection.voltage_drop_v;
        rows.push(thermal_margin_row(
            config,
            ThermalMarginSpec {
                id: protection.id.clone(),
                source_class: "protection",
                source_id: protection.part_id.clone(),
                power_w,
                theta_c_per_w: config
                    .thermal_margin_simulation_policy
                    .protection_theta_c_per_w,
                max_temp_c: config.thermal_margin_simulation_policy.max_component_temp_c,
                notes: protection.notes.clone(),
            },
        ));
    }

    for mosfet in &config.mosfet_checks {
        let current_a = mosfet.continuous_current_ma as f64 / 1000.0;
        let power_w = current_a * current_a * mosfet.rds_on_mohm / 1000.0;
        rows.push(thermal_margin_row(
            config,
            ThermalMarginSpec {
                id: mosfet.id.clone(),
                source_class: "mosfet",
                source_id: mosfet.part_id.clone(),
                power_w,
                theta_c_per_w: mosfet.package_theta_ja_c_per_w,
                max_temp_c: config.thermal_margin_simulation_policy.max_component_temp_c,
                notes: mosfet.notes.clone(),
            },
        ));
    }

    for pdn in pdn_dc_simulation_rows(config, routing) {
        rows.push(thermal_margin_row(
            config,
            ThermalMarginSpec {
                id: format!("{}_copper", pdn.path_id),
                source_class: "routed_copper",
                source_id: pdn.net,
                power_w: pdn.path_power_mw / 1000.0,
                theta_c_per_w: config.thermal_margin_simulation_policy.trace_theta_c_per_w,
                max_temp_c: config.thermal_margin_simulation_policy.max_trace_temp_c,
                notes: pdn.notes,
            },
        ));
    }

    rows
}

struct ThermalMarginSpec {
    id: String,
    source_class: &'static str,
    source_id: String,
    power_w: f64,
    theta_c_per_w: f64,
    max_temp_c: f64,
    notes: String,
}

fn thermal_margin_row(config: &ValidationConfig, spec: ThermalMarginSpec) -> ThermalMarginRow {
    let temp_rise_c = spec.power_w * spec.theta_c_per_w;
    let estimated_temp_c = config.assumptions.ambient_c + temp_rise_c;
    ThermalMarginRow {
        id: spec.id,
        source_class: spec.source_class.to_string(),
        source_id: spec.source_id,
        power_w: spec.power_w,
        theta_c_per_w: spec.theta_c_per_w,
        temp_rise_c,
        estimated_temp_c,
        max_temp_c: spec.max_temp_c,
        margin_c: spec.max_temp_c - estimated_temp_c,
        notes: spec.notes,
    }
}

fn validate_heater_pwm_transient(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.heater_pwm_transient_policy;
    for net in ["HEATER_SUPPLY", "HEATER_P", "GATE_DRV", "HEATER_PWM"] {
        push_gate!(
            rows,
            errors,
            "heater pwm transient",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    let on_current_a = heater_pwm_on_current_a(policy);
    let off_current_ma = heater_pwm_off_current_ma(policy);
    push_gate!(
        rows,
        errors,
        "heater pwm transient",
        "configured on current envelope",
        format!("{on_current_a:.3} A"),
        format!(
            "{:.3}..{:.3} A",
            policy.min_on_current_a, policy.max_on_current_a
        ),
        on_current_a >= policy.min_on_current_a && on_current_a <= policy.max_on_current_a,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater pwm transient",
        "configured off current leakage",
        format!("{off_current_ma:.6} mA"),
        format!("<= {:.6} mA", policy.max_off_current_ma),
        off_current_ma <= policy.max_off_current_ma,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater pwm transient",
        "gate drive high margin",
        format!("{:.3} V", policy.gate_drive_high_v),
        format!(
            ">= {:.3} V",
            policy.gate_threshold_v + policy.gate_hysteresis_v
        ),
        policy.gate_drive_high_v >= policy.gate_threshold_v + policy.gate_hysteresis_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater pwm transient",
        "gate drive low margin",
        format!("{:.3} V", policy.gate_drive_low_v),
        format!(
            "<= {:.3} V",
            policy.gate_threshold_v - policy.gate_hysteresis_v
        ),
        policy.gate_drive_low_v <= policy.gate_threshold_v - policy.gate_hysteresis_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater pwm transient",
        "PWM timing envelope",
        format!(
            "{:.3} ms on / {:.3} ms period / {:.3} ms stop",
            policy.pwm_on_ms, policy.pwm_period_ms, policy.simulation_stop_ms
        ),
        "0 < on < period and stop >= 2 periods",
        policy.pwm_on_ms > 0.0
            && policy.pwm_on_ms < policy.pwm_period_ms
            && policy.simulation_stop_ms >= 2.0 * policy.pwm_period_ms,
        &policy.notes,
    );
}

#[derive(Debug)]
struct HeaterPwmTransientRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn heater_pwm_transient_rows(config: &ValidationConfig) -> Vec<HeaterPwmTransientRow> {
    let policy = &config.heater_pwm_transient_policy;
    vec![
        HeaterPwmTransientRow {
            id: "heater_on_current",
            measurement: "expected on-state heater current",
            value: heater_pwm_on_current_a(policy),
            units: "A",
            limit: format!(
                "{:.3}..{:.3} A",
                policy.min_on_current_a, policy.max_on_current_a
            ),
            pass: heater_pwm_on_current_a(policy) >= policy.min_on_current_a
                && heater_pwm_on_current_a(policy) <= policy.max_on_current_a,
            notes: policy.notes.clone(),
        },
        HeaterPwmTransientRow {
            id: "heater_off_current",
            measurement: "expected off-state leakage",
            value: heater_pwm_off_current_ma(policy),
            units: "mA",
            limit: format!("<= {:.6} mA", policy.max_off_current_ma),
            pass: heater_pwm_off_current_ma(policy) <= policy.max_off_current_ma,
            notes: policy.notes.clone(),
        },
        HeaterPwmTransientRow {
            id: "gate_high",
            measurement: "gate-drive high level",
            value: policy.gate_drive_high_v,
            units: "V",
            limit: format!(">= {:.3} V", policy.min_gate_high_v),
            pass: policy.gate_drive_high_v >= policy.min_gate_high_v,
            notes: policy.notes.clone(),
        },
        HeaterPwmTransientRow {
            id: "gate_low",
            measurement: "gate-drive low level",
            value: policy.gate_drive_low_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_gate_low_v),
            pass: policy.gate_drive_low_v <= policy.max_gate_low_v,
            notes: policy.notes.clone(),
        },
    ]
}

fn heater_pwm_on_current_a(policy: &HeaterPwmTransientPolicy) -> f64 {
    policy.supply_voltage_v
        / (policy.heater_resistance_ohm
            + policy.heater_series_resistance_ohm
            + policy.mosfet_rds_on_ohm)
}

fn heater_pwm_off_current_ma(policy: &HeaterPwmTransientPolicy) -> f64 {
    policy.supply_voltage_v
        / (policy.heater_resistance_ohm
            + policy.heater_series_resistance_ohm
            + policy.mosfet_off_resistance_ohm)
        * 1000.0
}

fn validate_heater_thermal_transient(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.heater_thermal_transient_policy;
    for net in ["HEATER_SUPPLY", "HEATER_P", "ADC_AIN1"] {
        push_gate!(
            rows,
            errors,
            "heater thermal transient",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "heater thermal transient",
        "thermal model constants",
        format!(
            "{:.3} W, {:.3} J/C, {:.3} C/W",
            policy.heater_power_w, policy.thermal_mass_j_per_c, policy.thermal_resistance_c_per_w
        ),
        "positive power, thermal mass, and thermal resistance",
        policy.heater_power_w > 0.0
            && policy.thermal_mass_j_per_c > 0.0
            && policy.thermal_resistance_c_per_w > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater thermal transient",
        "temperature envelope",
        format!(
            "{:.2} C target, {:.2} C reached, {:.2} C max",
            policy.target_c, policy.target_reached_c, policy.max_temperature_c
        ),
        "ambient < reached <= target < max",
        policy.ambient_c < policy.target_reached_c
            && policy.target_reached_c <= policy.target_c
            && policy.target_c < policy.max_temperature_c,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater thermal transient",
        "control envelope",
        format!(
            "{:.3}..{:.3} hold duty, {:.3} max duty",
            policy.min_hold_duty, policy.max_hold_duty, policy.max_duty
        ),
        "0 <= min <= max hold <= max duty <= 1",
        policy.min_hold_duty >= 0.0
            && policy.min_hold_duty <= policy.max_hold_duty
            && policy.max_hold_duty <= policy.max_duty
            && policy.max_duty <= 1.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater thermal transient",
        "simulation timing",
        format!(
            "{:.3} s step / {:.1} s stop / {:.1} s hold window",
            policy.timestep_s, policy.simulation_stop_s, policy.hold_window_s
        ),
        "0 < step, hold window < stop",
        policy.timestep_s > 0.0
            && policy.hold_window_s > 0.0
            && policy.hold_window_s < policy.simulation_stop_s,
        &policy.notes,
    );

    for row in heater_thermal_transient_rows(config) {
        push_gate!(
            rows,
            errors,
            "heater thermal transient",
            row.measurement,
            format!("{:.6} {}", row.value, row.units),
            row.limit,
            row.pass,
            row.notes,
        );
    }
}

#[derive(Debug)]
struct HeaterThermalTransientRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

#[derive(Debug)]
struct HeaterThermalTransientMetrics {
    reached_target_s: Option<f64>,
    max_temp_c: f64,
    final_temp_c: f64,
    hold_error_c: f64,
    average_hold_duty: f64,
    energy_wh: f64,
    full_power_steady_temp_c: f64,
    thermal_time_constant_s: f64,
}

fn heater_thermal_transient_rows(config: &ValidationConfig) -> Vec<HeaterThermalTransientRow> {
    let policy = &config.heater_thermal_transient_policy;
    let metrics = heater_thermal_transient_metrics(policy);
    let reached_s = metrics.reached_target_s.unwrap_or(f64::INFINITY);
    vec![
        HeaterThermalTransientRow {
            id: "full_power_steady_state",
            measurement: "full-power steady-state temperature",
            value: metrics.full_power_steady_temp_c,
            units: "C",
            limit: format!(">= {:.3} C target", policy.target_c),
            pass: metrics.full_power_steady_temp_c >= policy.target_c,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "thermal_time_constant",
            measurement: "thermal time constant",
            value: metrics.thermal_time_constant_s,
            units: "s",
            limit: "> 0 s".to_string(),
            pass: metrics.thermal_time_constant_s > 0.0,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "target_reach_time",
            measurement: "time to reach target threshold",
            value: reached_s,
            units: "s",
            limit: format!("<= {:.3} s", policy.max_warmup_s),
            pass: reached_s <= policy.max_warmup_s,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "max_temperature",
            measurement: "maximum simulated reaction-block temperature",
            value: metrics.max_temp_c,
            units: "C",
            limit: format!("<= {:.3} C", policy.max_temperature_c),
            pass: metrics.max_temp_c <= policy.max_temperature_c,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "overshoot",
            measurement: "maximum target overshoot",
            value: (metrics.max_temp_c - policy.target_c).max(0.0),
            units: "C",
            limit: format!("<= {:.3} C", policy.max_overshoot_c),
            pass: (metrics.max_temp_c - policy.target_c).max(0.0) <= policy.max_overshoot_c,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "hold_error",
            measurement: "maximum hold-window error",
            value: metrics.hold_error_c,
            units: "C",
            limit: format!("<= {:.3} C", policy.max_hold_error_c),
            pass: metrics.hold_error_c <= policy.max_hold_error_c,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "final_temperature",
            measurement: "final simulated reaction-block temperature",
            value: metrics.final_temp_c,
            units: "C",
            limit: format!(
                "{:.3}..{:.3} C",
                policy.target_c - policy.max_hold_error_c,
                policy.target_c + policy.max_hold_error_c
            ),
            pass: (metrics.final_temp_c - policy.target_c).abs() <= policy.max_hold_error_c,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "average_hold_duty",
            measurement: "average hold-window heater duty",
            value: metrics.average_hold_duty,
            units: "ratio",
            limit: format!("{:.3}..{:.3}", policy.min_hold_duty, policy.max_hold_duty),
            pass: metrics.average_hold_duty >= policy.min_hold_duty
                && metrics.average_hold_duty <= policy.max_hold_duty,
            notes: policy.notes.clone(),
        },
        HeaterThermalTransientRow {
            id: "thermal_energy",
            measurement: "heater energy over simulated profile",
            value: metrics.energy_wh,
            units: "Wh",
            limit: format!("<= {:.3} Wh", policy.max_energy_wh),
            pass: metrics.energy_wh <= policy.max_energy_wh,
            notes: policy.notes.clone(),
        },
    ]
}

fn heater_thermal_transient_metrics(
    policy: &HeaterThermalTransientPolicy,
) -> HeaterThermalTransientMetrics {
    let target_duty = ((policy.target_c - policy.ambient_c) / policy.thermal_resistance_c_per_w)
        / policy.heater_power_w;
    let mut temp_c = policy.ambient_c;
    let mut max_temp_c = temp_c;
    let mut reached_target_s = None;
    let mut energy_j = 0.0;
    let mut hold_error_c = 0.0;
    let mut hold_duty_sum = 0.0;
    let mut hold_duty_count = 0.0;
    let mut time_s = 0.0;

    while time_s <= policy.simulation_stop_s + f64::EPSILON {
        let duty = heater_thermal_control_duty(policy, target_duty, temp_c);
        let heater_power_w = policy.heater_power_w * duty;
        let loss_w = (temp_c - policy.ambient_c) / policy.thermal_resistance_c_per_w;
        temp_c += (heater_power_w - loss_w) * policy.timestep_s / policy.thermal_mass_j_per_c;
        energy_j += heater_power_w * policy.timestep_s;
        time_s += policy.timestep_s;

        if temp_c > max_temp_c {
            max_temp_c = temp_c;
        }
        if reached_target_s.is_none() && temp_c >= policy.target_reached_c {
            reached_target_s = Some(time_s);
        }
        if time_s >= policy.simulation_stop_s - policy.hold_window_s {
            hold_error_c = f64::max(hold_error_c, (temp_c - policy.target_c).abs());
            hold_duty_sum += duty;
            hold_duty_count += 1.0;
        }
    }

    HeaterThermalTransientMetrics {
        reached_target_s,
        max_temp_c,
        final_temp_c: temp_c,
        hold_error_c,
        average_hold_duty: if hold_duty_count > 0.0 {
            hold_duty_sum / hold_duty_count
        } else {
            f64::INFINITY
        },
        energy_wh: energy_j / 3600.0,
        full_power_steady_temp_c: policy.ambient_c
            + policy.heater_power_w * policy.thermal_resistance_c_per_w,
        thermal_time_constant_s: policy.thermal_mass_j_per_c * policy.thermal_resistance_c_per_w,
    }
}

fn heater_thermal_control_duty(
    policy: &HeaterThermalTransientPolicy,
    target_duty: f64,
    temp_c: f64,
) -> f64 {
    if temp_c < policy.target_c - policy.controller_band_c {
        return policy.max_duty;
    }
    let commanded = target_duty + policy.proportional_gain_per_c * (policy.target_c - temp_c);
    commanded
        .clamp(policy.min_hold_duty, policy.max_hold_duty)
        .min(policy.max_duty)
}

fn validate_usb_inrush_startup(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.usb_inrush_startup_policy;
    for net in ["VBUS", "+5V", "+3V3", "GND"] {
        push_gate!(
            rows,
            errors,
            "usb inrush startup",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "usb inrush startup",
        "startup model constants",
        format!(
            "{:.3} V, {:.3} ohm source, {:.3}/{:.3}/{:.3} uF",
            policy.vbus_nominal_v,
            policy.usb_source_resistance_ohm,
            policy.vbus_bulk_cap_u,
            policy.five_v_bulk_cap_u,
            policy.three_v_three_bulk_cap_u
        ),
        "positive source, resistance, capacitance",
        policy.vbus_nominal_v > 0.0
            && policy.usb_source_resistance_ohm > 0.0
            && policy.vbus_bulk_cap_u > 0.0
            && policy.five_v_bulk_cap_u > 0.0
            && policy.three_v_three_bulk_cap_u > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb inrush startup",
        "startup timing",
        format!(
            "{:.6} ms step / {:.3} ms stop",
            policy.timestep_ms, policy.simulation_stop_ms
        ),
        "0 < step < stop",
        policy.timestep_ms > 0.0 && policy.timestep_ms < policy.simulation_stop_ms,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "usb inrush startup",
        "3.3 V acceptance window",
        format!(
            "{:.3}..{:.3} V final, {:.3} V overshoot",
            policy.min_final_3v3_v, policy.max_final_3v3_v, policy.max_3v3_overshoot_v
        ),
        "0 < min <= regulated <= max",
        policy.min_final_3v3_v > 0.0
            && policy.min_final_3v3_v <= policy.regulated_3v3_v
            && policy.regulated_3v3_v <= policy.max_final_3v3_v,
        &policy.notes,
    );

    for row in usb_inrush_startup_rows(config) {
        push_gate!(
            rows,
            errors,
            "usb inrush startup",
            row.measurement,
            format!("{:.6} {}", row.value, row.units),
            row.limit,
            row.pass,
            row.notes,
        );
    }
}

#[derive(Debug)]
struct UsbInrushStartupRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

#[derive(Debug)]
struct UsbInrushStartupMetrics {
    max_vbus_inrush_ma: f64,
    max_five_v_inrush_ma: f64,
    startup_ms: Option<f64>,
    final_vbus_v: f64,
    final_five_v_v: f64,
    final_three_v_three_v: f64,
    max_three_v_three_v: f64,
}

fn usb_inrush_startup_rows(config: &ValidationConfig) -> Vec<UsbInrushStartupRow> {
    let policy = &config.usb_inrush_startup_policy;
    let metrics = usb_inrush_startup_metrics(policy);
    let startup_ms = metrics.startup_ms.unwrap_or(f64::INFINITY);
    let overshoot_v = (metrics.max_three_v_three_v - policy.regulated_3v3_v).max(0.0);
    vec![
        UsbInrushStartupRow {
            id: "vbus_inrush_current",
            measurement: "maximum USB VBUS source inrush",
            value: metrics.max_vbus_inrush_ma,
            units: "mA",
            limit: format!("<= {:.3} mA", policy.max_vbus_inrush_ma),
            pass: metrics.max_vbus_inrush_ma <= policy.max_vbus_inrush_ma,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "five_v_path_inrush_current",
            measurement: "maximum post-Schottky +5 V charging current",
            value: metrics.max_five_v_inrush_ma,
            units: "mA",
            limit: format!("<= {:.3} mA", policy.max_5v_inrush_ma),
            pass: metrics.max_five_v_inrush_ma <= policy.max_5v_inrush_ma,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "three_v_three_startup_time",
            measurement: "time for +3V3 to enter valid window",
            value: startup_ms,
            units: "ms",
            limit: format!("<= {:.3} ms", policy.max_startup_ms),
            pass: startup_ms <= policy.max_startup_ms,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "three_v_three_final_voltage",
            measurement: "final +3V3 voltage",
            value: metrics.final_three_v_three_v,
            units: "V",
            limit: format!(
                "{:.3}..{:.3} V",
                policy.min_final_3v3_v, policy.max_final_3v3_v
            ),
            pass: metrics.final_three_v_three_v >= policy.min_final_3v3_v
                && metrics.final_three_v_three_v <= policy.max_final_3v3_v,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "three_v_three_overshoot",
            measurement: "maximum +3V3 startup overshoot",
            value: overshoot_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_3v3_overshoot_v),
            pass: overshoot_v <= policy.max_3v3_overshoot_v,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "vbus_final_voltage",
            measurement: "final VBUS voltage",
            value: metrics.final_vbus_v,
            units: "V",
            limit: format!(">= {:.3} V", policy.vbus_nominal_v * 0.90),
            pass: metrics.final_vbus_v >= policy.vbus_nominal_v * 0.90,
            notes: policy.notes.clone(),
        },
        UsbInrushStartupRow {
            id: "five_v_final_voltage",
            measurement: "final post-Schottky +5 V voltage",
            value: metrics.final_five_v_v,
            units: "V",
            limit: format!(
                ">= {:.3} V",
                policy.regulated_3v3_v + policy.regulator_dropout_v
            ),
            pass: metrics.final_five_v_v >= policy.regulated_3v3_v + policy.regulator_dropout_v,
            notes: policy.notes.clone(),
        },
    ]
}

fn usb_inrush_startup_metrics(policy: &UsbInrushStartupPolicy) -> UsbInrushStartupMetrics {
    let dt_s = (policy.timestep_ms / 1000.0).max(1.0e-9);
    let stop_s = (policy.simulation_stop_ms / 1000.0).max(dt_s);
    let vbus_cap_f = (policy.vbus_bulk_cap_u * 1.0e-6).max(1.0e-12);
    let five_v_cap_f = (policy.five_v_bulk_cap_u * 1.0e-6).max(1.0e-12);
    let three_v_three_cap_f = (policy.three_v_three_bulk_cap_u * 1.0e-6).max(1.0e-12);
    let five_v_load_a = (policy.five_v_load_ma / 1000.0).max(0.0);
    let three_v_three_load_a = (policy.three_v_three_load_ma / 1000.0).max(0.0);

    let mut vbus_v = 0.0;
    let mut five_v = 0.0;
    let mut three_v_three_v = 0.0;
    let mut max_vbus_inrush_ma = 0.0;
    let mut max_five_v_inrush_ma = 0.0;
    let mut max_three_v_three_v = 0.0;
    let mut startup_ms = None;
    let mut time_s = 0.0;

    while time_s <= stop_s + f64::EPSILON {
        let source_current_a =
            ((policy.vbus_nominal_v - vbus_v) / policy.usb_source_resistance_ohm).max(0.0);
        let diode_current_a = ((vbus_v - policy.schottky_drop_v - five_v)
            / policy.schottky_series_resistance_ohm)
            .max(0.0);
        let regulator_input_available_v = five_v - policy.regulator_dropout_v;
        let regulator_current_a = if three_v_three_v < policy.regulated_3v3_v
            && regulator_input_available_v > three_v_three_v
        {
            ((regulator_input_available_v - three_v_three_v)
                / policy.regulator_series_resistance_ohm)
                .max(0.0)
        } else {
            0.0
        };

        max_vbus_inrush_ma = f64::max(max_vbus_inrush_ma, source_current_a * 1000.0);
        max_five_v_inrush_ma = f64::max(max_five_v_inrush_ma, diode_current_a * 1000.0);

        vbus_v += (source_current_a - diode_current_a) * dt_s / vbus_cap_f;
        five_v += (diode_current_a - five_v_load_a - regulator_current_a) * dt_s / five_v_cap_f;
        three_v_three_v +=
            (regulator_current_a - three_v_three_load_a) * dt_s / three_v_three_cap_f;

        vbus_v = vbus_v.clamp(0.0, policy.vbus_nominal_v);
        five_v = five_v.clamp(0.0, policy.vbus_nominal_v - policy.schottky_drop_v);
        three_v_three_v = three_v_three_v.clamp(0.0, policy.regulated_3v3_v);
        max_three_v_three_v = f64::max(max_three_v_three_v, three_v_three_v);
        time_s += dt_s;

        if startup_ms.is_none() && three_v_three_v >= policy.min_final_3v3_v {
            startup_ms = Some(time_s * 1000.0);
        }
    }

    UsbInrushStartupMetrics {
        max_vbus_inrush_ma,
        max_five_v_inrush_ma,
        startup_ms,
        final_vbus_v: vbus_v,
        final_five_v_v: five_v,
        final_three_v_three_v: three_v_three_v,
        max_three_v_three_v,
    }
}

fn validate_power_domain_fault(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.power_domain_fault_policy;
    for net in ["VBUS", "+5V", "+3V3", "+12V_RAW", "HEATER_SUPPLY"] {
        push_gate!(
            rows,
            errors,
            "power domain fault simulation",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "power domain fault simulation",
        "fault/isolation model constants",
        format!(
            "{:.3} V fault, {:.3} ohm VBUS isolation, {:.3} ohm regulator reverse path",
            policy.heater_fault_voltage_v,
            policy.heater_to_vbus_isolation_ohm,
            policy.regulator_reverse_resistance_ohm
        ),
        "positive fault voltage, load, isolation, and reverse resistance",
        policy.heater_fault_voltage_v > 0.0
            && policy.vbus_load_ohm > 0.0
            && policy.five_v_load_ohm > 0.0
            && policy.three_v_three_load_ohm > 0.0
            && policy.heater_to_vbus_isolation_ohm > 0.0
            && policy.heater_to_five_v_isolation_ohm > 0.0
            && policy.heater_to_three_v_three_isolation_ohm > 0.0
            && policy.regulator_reverse_resistance_ohm > 0.0
            && policy.simulation_stop_ms > 0.0,
        &policy.notes,
    );

    for row in power_domain_fault_rows(config) {
        push_gate!(
            rows,
            errors,
            "power domain fault simulation",
            row.measurement,
            format!("{:.6} {}", row.value, row.units),
            row.limit,
            row.pass,
            row.notes,
        );
    }
}

#[derive(Debug)]
struct PowerDomainFaultMetrics {
    vbus_fault_v: f64,
    five_v_fault_v: f64,
    three_v_three_fault_v: f64,
    usb_backfeed_ma: f64,
    regulator_reverse_ma: f64,
}

#[derive(Debug)]
struct PowerDomainFaultRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn power_domain_fault_rows(config: &ValidationConfig) -> Vec<PowerDomainFaultRow> {
    let policy = &config.power_domain_fault_policy;
    let metrics = power_domain_fault_metrics(policy);
    vec![
        PowerDomainFaultRow {
            id: "vbus_fault_voltage",
            measurement: "VBUS voltage during heater-domain fault",
            value: metrics.vbus_fault_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_vbus_fault_v),
            pass: metrics.vbus_fault_v <= policy.max_vbus_fault_v,
            notes: policy.notes.clone(),
        },
        PowerDomainFaultRow {
            id: "five_v_fault_voltage",
            measurement: "+5V voltage during heater-domain fault",
            value: metrics.five_v_fault_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_five_v_fault_v),
            pass: metrics.five_v_fault_v <= policy.max_five_v_fault_v,
            notes: policy.notes.clone(),
        },
        PowerDomainFaultRow {
            id: "three_v_three_fault_voltage",
            measurement: "+3V3 voltage during heater-domain fault",
            value: metrics.three_v_three_fault_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_three_v_three_fault_v),
            pass: metrics.three_v_three_fault_v <= policy.max_three_v_three_fault_v,
            notes: policy.notes.clone(),
        },
        PowerDomainFaultRow {
            id: "usb_backfeed_current",
            measurement: "heater-domain backfeed current into VBUS",
            value: metrics.usb_backfeed_ma,
            units: "mA",
            limit: format!("<= {:.3} mA", policy.max_usb_backfeed_ma),
            pass: metrics.usb_backfeed_ma <= policy.max_usb_backfeed_ma,
            notes: policy.notes.clone(),
        },
        PowerDomainFaultRow {
            id: "regulator_reverse_current",
            measurement: "reverse current across +5V/+3V3 regulator path",
            value: metrics.regulator_reverse_ma,
            units: "mA",
            limit: format!("<= {:.3} mA", policy.max_regulator_reverse_ma),
            pass: metrics.regulator_reverse_ma <= policy.max_regulator_reverse_ma,
            notes: policy.notes.clone(),
        },
    ]
}

fn power_domain_fault_metrics(policy: &PowerDomainFaultPolicy) -> PowerDomainFaultMetrics {
    let vbus_fault_v = isolated_fault_voltage(
        policy.heater_fault_voltage_v,
        policy.heater_to_vbus_isolation_ohm,
        policy.vbus_load_ohm,
    );
    let five_v_fault_v = isolated_fault_voltage(
        policy.heater_fault_voltage_v,
        policy.heater_to_five_v_isolation_ohm,
        policy.five_v_load_ohm,
    );
    let three_v_three_fault_v = isolated_fault_voltage(
        policy.heater_fault_voltage_v,
        policy.heater_to_three_v_three_isolation_ohm,
        policy.three_v_three_load_ohm,
    );
    let usb_backfeed_ma = ((policy.heater_fault_voltage_v - vbus_fault_v)
        / policy.heater_to_vbus_isolation_ohm)
        .abs()
        * 1000.0;
    let regulator_reverse_ma =
        ((five_v_fault_v - three_v_three_fault_v) / policy.regulator_reverse_resistance_ohm).abs()
            * 1000.0;

    PowerDomainFaultMetrics {
        vbus_fault_v,
        five_v_fault_v,
        three_v_three_fault_v,
        usb_backfeed_ma,
        regulator_reverse_ma,
    }
}

fn isolated_fault_voltage(fault_voltage_v: f64, isolation_ohm: f64, load_ohm: f64) -> f64 {
    fault_voltage_v * load_ohm / (isolation_ohm + load_ohm)
}

fn validate_rail_load_step(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.rail_load_step_policy;
    push_gate!(
        rows,
        errors,
        "rail load step transient",
        "+3V3 contract net",
        "+3V3",
        "present in contract",
        contract_nets.contains("+3V3"),
        &policy.notes,
    );

    push_gate!(
        rows,
        errors,
        "rail load step transient",
        "load-step current envelope",
        format!(
            "{} mA baseline -> {} mA burst",
            policy.baseline_load_ma, policy.burst_load_ma
        ),
        "0 < baseline < burst <= source limit",
        policy.baseline_load_ma > 0
            && policy.baseline_load_ma < policy.burst_load_ma
            && policy.burst_load_ma <= policy.max_source_current_ma,
        &policy.notes,
    );

    push_gate!(
        rows,
        errors,
        "rail load step transient",
        "load-step timing envelope",
        format!(
            "{:.3} ms start / {:.3} ms burst / {:.3} ms period / {:.3} ms stop",
            policy.burst_start_ms,
            policy.burst_width_ms,
            policy.period_ms,
            policy.simulation_stop_ms
        ),
        "0 < start, 0 < width < period, stop >= 2 periods",
        policy.burst_start_ms > 0.0
            && policy.burst_width_ms > 0.0
            && policy.burst_width_ms < policy.period_ms
            && policy.simulation_stop_ms >= 2.0 * policy.period_ms,
        &policy.notes,
    );

    let estimated_min_v = rail_load_step_estimated_min_v(policy);
    push_gate!(
        rows,
        errors,
        "rail load step transient",
        "estimated burst rail sag",
        format!("{estimated_min_v:.3} V"),
        format!(">= {:.3} V", policy.min_rail_voltage_v),
        estimated_min_v >= policy.min_rail_voltage_v,
        &policy.notes,
    );

    push_gate!(
        rows,
        errors,
        "rail load step transient",
        "3.3 V bulk capacitance",
        format!("{:.3} uF", policy.bulk_capacitance_u),
        "> 0 uF",
        policy.bulk_capacitance_u > 0.0,
        &policy.notes,
    );
}

#[derive(Debug)]
struct RailLoadStepRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn rail_load_step_rows(config: &ValidationConfig) -> Vec<RailLoadStepRow> {
    let policy = &config.rail_load_step_policy;
    vec![
        RailLoadStepRow {
            id: "rail_min_voltage",
            measurement: "estimated minimum 3.3 V rail during burst",
            value: rail_load_step_estimated_min_v(policy),
            units: "V",
            limit: format!(">= {:.3} V", policy.min_rail_voltage_v),
            pass: rail_load_step_estimated_min_v(policy) >= policy.min_rail_voltage_v,
            notes: policy.notes.clone(),
        },
        RailLoadStepRow {
            id: "burst_source_current",
            measurement: "burst source current",
            value: policy.burst_load_ma as f64,
            units: "mA",
            limit: format!("<= {} mA", policy.max_source_current_ma),
            pass: policy.burst_load_ma <= policy.max_source_current_ma,
            notes: policy.notes.clone(),
        },
        RailLoadStepRow {
            id: "baseline_source_current",
            measurement: "baseline source current",
            value: policy.baseline_load_ma as f64,
            units: "mA",
            limit: format!("< {} mA burst", policy.burst_load_ma),
            pass: policy.baseline_load_ma < policy.burst_load_ma,
            notes: policy.notes.clone(),
        },
        RailLoadStepRow {
            id: "rail_recovery_voltage",
            measurement: "expected recovered rail voltage",
            value: policy.nominal_voltage_v - rail_load_step_baseline_drop_v(policy),
            units: "V",
            limit: format!(">= {:.3} V", policy.min_rail_voltage_v),
            pass: policy.nominal_voltage_v - rail_load_step_baseline_drop_v(policy)
                >= policy.min_rail_voltage_v,
            notes: policy.notes.clone(),
        },
    ]
}

fn rail_load_step_estimated_min_v(policy: &RailLoadStepPolicy) -> f64 {
    policy.nominal_voltage_v - (policy.burst_load_ma as f64 / 1000.0) * policy.source_resistance_ohm
}

fn rail_load_step_baseline_drop_v(policy: &RailLoadStepPolicy) -> f64 {
    (policy.baseline_load_ma as f64 / 1000.0) * policy.source_resistance_ohm
}

fn validate_analog_front_end_transient(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.analog_front_end_policy;
    for net in ["MUX_COM", "ADC_AIN1", "+3V3", "GND"] {
        push_gate!(
            rows,
            errors,
            "analog front end transient",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    let dark_v = analog_front_end_output_v(policy.dark_current_na, policy);
    let light_v = analog_front_end_output_v(policy.light_current_na, policy);
    let max_signal_v = analog_front_end_output_v(policy.max_photocurrent_na, policy);
    push_gate!(
        rows,
        errors,
        "analog front end transient",
        "dark-to-light signal delta",
        format!("{:.3} V", light_v - dark_v),
        format!(">= {:.3} V", policy.min_signal_delta_v),
        light_v - dark_v >= policy.min_signal_delta_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "analog front end transient",
        "light-state ADC range",
        format!("{light_v:.3} V"),
        format!(
            "{:.3}..{:.3} V",
            policy.min_adc_voltage_v, policy.max_adc_voltage_v
        ),
        light_v >= policy.min_adc_voltage_v && light_v <= policy.max_adc_voltage_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "analog front end transient",
        "maximum photocurrent saturation margin",
        format!("{max_signal_v:.3} V"),
        format!("<= {:.3} V", policy.max_adc_voltage_v),
        max_signal_v <= policy.max_adc_voltage_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "analog front end transient",
        "TIA compensation RC",
        format!(
            "{:.0} ohm / {:.3} pF",
            policy.feedback_resistor_ohm, policy.feedback_cap_pf
        ),
        "positive feedback resistor and capacitor",
        policy.feedback_resistor_ohm > 0.0 && policy.feedback_cap_pf > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "analog front end transient",
        "optical timing envelope",
        format!(
            "{:.3} ms start / {:.3} ms light / {:.3} ms period / {:.3} ms stop",
            policy.light_start_ms,
            policy.light_width_ms,
            policy.period_ms,
            policy.simulation_stop_ms
        ),
        "0 < start, 0 < light < period, stop >= period",
        policy.light_start_ms > 0.0
            && policy.light_width_ms > 0.0
            && policy.light_width_ms < policy.period_ms
            && policy.simulation_stop_ms >= policy.period_ms,
        &policy.notes,
    );
}

#[derive(Debug)]
struct AnalogFrontEndRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn analog_front_end_rows(config: &ValidationConfig) -> Vec<AnalogFrontEndRow> {
    let policy = &config.analog_front_end_policy;
    let dark_v = analog_front_end_output_v(policy.dark_current_na, policy);
    let light_v = analog_front_end_output_v(policy.light_current_na, policy);
    let max_signal_v = analog_front_end_output_v(policy.max_photocurrent_na, policy);
    vec![
        AnalogFrontEndRow {
            id: "dark_output_voltage",
            measurement: "expected dark ADC_AIN1 voltage",
            value: dark_v,
            units: "V",
            limit: format!(
                "{:.3}..{:.3} V",
                policy.min_adc_voltage_v, policy.max_adc_voltage_v
            ),
            pass: dark_v >= policy.min_adc_voltage_v && dark_v <= policy.max_adc_voltage_v,
            notes: policy.notes.clone(),
        },
        AnalogFrontEndRow {
            id: "light_output_voltage",
            measurement: "expected light ADC_AIN1 voltage",
            value: light_v,
            units: "V",
            limit: format!(
                "{:.3}..{:.3} V",
                policy.min_adc_voltage_v, policy.max_adc_voltage_v
            ),
            pass: light_v >= policy.min_adc_voltage_v && light_v <= policy.max_adc_voltage_v,
            notes: policy.notes.clone(),
        },
        AnalogFrontEndRow {
            id: "signal_delta_voltage",
            measurement: "expected dark-to-light signal delta",
            value: light_v - dark_v,
            units: "V",
            limit: format!(">= {:.3} V", policy.min_signal_delta_v),
            pass: light_v - dark_v >= policy.min_signal_delta_v,
            notes: policy.notes.clone(),
        },
        AnalogFrontEndRow {
            id: "max_photocurrent_voltage",
            measurement: "maximum modeled photocurrent ADC_AIN1 voltage",
            value: max_signal_v,
            units: "V",
            limit: format!("<= {:.3} V", policy.max_adc_voltage_v),
            pass: max_signal_v <= policy.max_adc_voltage_v,
            notes: policy.notes.clone(),
        },
    ]
}

fn analog_front_end_output_v(current_na: f64, policy: &AnalogFrontEndPolicy) -> f64 {
    current_na * 1.0e-9 * policy.feedback_resistor_ohm
}

fn validate_optical_crosstalk(
    config: &ValidationConfig,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.optical_crosstalk_policy;
    let analog = &config.analog_front_end_policy;
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();
    let calibration_outputs = config
        .calibration_checks
        .iter()
        .flat_map(|check| check.required_outputs.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "slot count",
        format!("{} slots", policy.slot_count),
        "8 slots",
        policy.slot_count == 8,
        &policy.notes,
    );

    for slot in 0..policy.slot_count {
        let net = format!("{}{}", policy.led_net_prefix, slot);
        push_gate!(
            rows,
            errors,
            "optical crosstalk",
            format!("{net} contract net"),
            net.clone(),
            "present in contract",
            contract_nets.contains(&net),
            &policy.notes,
        );
    }

    for net in policy
        .mux_select_nets
        .iter()
        .chain([&policy.mux_common_net, &policy.adc_net])
    {
        push_gate!(
            rows,
            errors,
            "optical crosstalk",
            format!("{net} contract net"),
            net.clone(),
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    let signal_delta_v = analog_front_end_output_v(analog.light_current_na, analog)
        - analog_front_end_output_v(analog.dark_current_na, analog);
    let adjacent_delta_v = signal_delta_v * policy.max_adjacent_crosstalk_pct / 100.0;
    let non_adjacent_delta_v = signal_delta_v * policy.max_non_adjacent_crosstalk_pct / 100.0;
    let inactive_leakage_v = policy.mux_off_leakage_na
        * policy.slot_count as f64
        * 1.0e-9
        * analog.feedback_resistor_ohm;
    let settling_us = optical_settling_time_us(policy, analog);

    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "adjacent-channel crosstalk envelope",
        format!("{adjacent_delta_v:.4} V"),
        format!("<= {:.4} V", policy.max_crosstalk_delta_v),
        adjacent_delta_v <= policy.max_crosstalk_delta_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "non-adjacent-channel crosstalk envelope",
        format!("{non_adjacent_delta_v:.4} V"),
        format!("<= {:.4} V", policy.max_crosstalk_delta_v),
        non_adjacent_delta_v <= policy.max_crosstalk_delta_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "mux off-leakage dark shift",
        format!("{inactive_leakage_v:.4} V"),
        format!("<= {:.4} V", policy.max_dark_shift_v),
        inactive_leakage_v <= policy.max_dark_shift_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "mux and ADC sample settling",
        format!("{settling_us:.3} us"),
        format!("<= {:.3} us", policy.sample_settle_us),
        settling_us <= policy.sample_settle_us,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical crosstalk",
        "settling error target",
        format!("{:.3} %", policy.settling_error_pct),
        "0 < error < 10%",
        policy.settling_error_pct > 0.0 && policy.settling_error_pct < 10.0,
        &policy.notes,
    );

    for measurement in &policy.required_measurements {
        push_gate!(
            rows,
            errors,
            "optical crosstalk",
            format!("required measurement {measurement}"),
            measurement.clone(),
            "present in first-article measurement plan",
            measurement_ids.contains(measurement.as_str()),
            &policy.notes,
        );
    }
    for output in &policy.required_outputs {
        push_gate!(
            rows,
            errors,
            "optical crosstalk",
            format!("required calibration output {output}"),
            output.clone(),
            "present in calibration output plan",
            calibration_outputs.contains(output.as_str()),
            &policy.notes,
        );
    }
}

#[derive(Debug)]
struct OpticalCrosstalkRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn optical_crosstalk_rows(config: &ValidationConfig) -> Vec<OpticalCrosstalkRow> {
    let policy = &config.optical_crosstalk_policy;
    let analog = &config.analog_front_end_policy;
    let signal_delta_v = analog_front_end_output_v(analog.light_current_na, analog)
        - analog_front_end_output_v(analog.dark_current_na, analog);
    let adjacent_delta_v = signal_delta_v * policy.max_adjacent_crosstalk_pct / 100.0;
    let non_adjacent_delta_v = signal_delta_v * policy.max_non_adjacent_crosstalk_pct / 100.0;
    let inactive_leakage_v = policy.mux_off_leakage_na
        * policy.slot_count as f64
        * 1.0e-9
        * analog.feedback_resistor_ohm;
    let settling_us = optical_settling_time_us(policy, analog);

    vec![
        OpticalCrosstalkRow {
            id: "adjacent_crosstalk_delta",
            measurement: "worst-case adjacent LED crosstalk ADC delta",
            value: adjacent_delta_v,
            units: "V",
            limit: format!("<= {:.4} V", policy.max_crosstalk_delta_v),
            pass: adjacent_delta_v <= policy.max_crosstalk_delta_v,
            notes: policy.notes.clone(),
        },
        OpticalCrosstalkRow {
            id: "non_adjacent_crosstalk_delta",
            measurement: "worst-case non-adjacent LED crosstalk ADC delta",
            value: non_adjacent_delta_v,
            units: "V",
            limit: format!("<= {:.4} V", policy.max_crosstalk_delta_v),
            pass: non_adjacent_delta_v <= policy.max_crosstalk_delta_v,
            notes: policy.notes.clone(),
        },
        OpticalCrosstalkRow {
            id: "mux_off_leakage_dark_shift",
            measurement: "estimated mux off-leakage dark shift",
            value: inactive_leakage_v,
            units: "V",
            limit: format!("<= {:.4} V", policy.max_dark_shift_v),
            pass: inactive_leakage_v <= policy.max_dark_shift_v,
            notes: policy.notes.clone(),
        },
        OpticalCrosstalkRow {
            id: "mux_adc_settling_time",
            measurement: "estimated mux/ADC settling time",
            value: settling_us,
            units: "us",
            limit: format!("<= {:.3} us", policy.sample_settle_us),
            pass: settling_us <= policy.sample_settle_us,
            notes: policy.notes.clone(),
        },
    ]
}

fn optical_settling_time_us(policy: &OpticalCrosstalkPolicy, analog: &AnalogFrontEndPolicy) -> f64 {
    let total_resistance_ohm = analog.mux_on_resistance_ohm + policy.tia_output_impedance_ohm;
    let total_capacitance_f =
        (analog.feedback_cap_pf + analog.input_cap_pf + policy.adc_sample_cap_pf) * 1.0e-12;
    let target_error = policy.settling_error_pct / 100.0;
    -total_resistance_ohm * total_capacitance_f * target_error.ln() * 1.0e6
}

fn validate_optical_noise_margin(
    config: &ValidationConfig,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.optical_noise_margin_policy;
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();
    let calibration_outputs = config
        .calibration_checks
        .iter()
        .flat_map(|check| check.required_outputs.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    let metrics = optical_noise_margin_metrics(config);

    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "ADC full-scale model",
        format!(
            "{:.3} V full-scale, {} counts",
            policy.adc_full_scale_v, policy.adc_counts
        ),
        format!(
            "> 0 counts and <= {:.3} V analog rail",
            config.analog_front_end_policy.adc_rail_v
        ),
        policy.adc_counts > 0
            && policy.adc_full_scale_v > 0.0
            && policy.adc_full_scale_v <= config.analog_front_end_policy.adc_rail_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "threshold fraction",
        format!("{:.3}", policy.threshold_fraction),
        "0.05 < threshold < 0.95",
        policy.threshold_fraction > 0.05 && policy.threshold_fraction < 0.95,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "combined noise floor",
        format!("{:.6} V", metrics.total_noise_v),
        "> 0 V",
        metrics.total_noise_v > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "dark-to-light signal-to-noise ratio",
        format!("{:.2}:1", metrics.signal_to_noise_ratio),
        format!(">= {:.2}:1", policy.min_signal_to_noise_ratio),
        metrics.signal_to_noise_ratio >= policy.min_signal_to_noise_ratio,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "false-positive threshold margin",
        format!("{:.4} V", metrics.false_positive_margin_v),
        format!(">= {:.4} V", policy.min_threshold_margin_v),
        metrics.false_positive_margin_v >= policy.min_threshold_margin_v,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "optical noise margin",
        "detection threshold margin",
        format!("{:.4} V", metrics.detection_margin_v),
        format!(">= {:.4} V", policy.min_threshold_margin_v),
        metrics.detection_margin_v >= policy.min_threshold_margin_v,
        &policy.notes,
    );

    for measurement in &policy.required_measurements {
        push_gate!(
            rows,
            errors,
            "optical noise margin",
            format!("required measurement {measurement}"),
            measurement.clone(),
            "present in first-article measurement plan",
            measurement_ids.contains(measurement.as_str()),
            &policy.notes,
        );
    }
    for output in &policy.required_outputs {
        push_gate!(
            rows,
            errors,
            "optical noise margin",
            format!("required calibration output {output}"),
            output.clone(),
            "present in calibration output plan",
            calibration_outputs.contains(output.as_str()),
            &policy.notes,
        );
    }
}

#[derive(Debug)]
struct OpticalNoiseMarginMetrics {
    signal_delta_v: f64,
    lsb_v: f64,
    analog_noise_v: f64,
    adc_noise_v: f64,
    quantization_guard_v: f64,
    total_noise_v: f64,
    threshold_v: f64,
    negative_control_drift_v: f64,
    signal_to_noise_ratio: f64,
    false_positive_margin_v: f64,
    detection_margin_v: f64,
}

#[derive(Debug)]
struct OpticalNoiseMarginRow {
    id: &'static str,
    measurement: &'static str,
    value: f64,
    units: &'static str,
    limit: String,
    pass: bool,
    notes: String,
}

fn optical_noise_margin_metrics(config: &ValidationConfig) -> OpticalNoiseMarginMetrics {
    let analog = &config.analog_front_end_policy;
    let policy = &config.optical_noise_margin_policy;
    let signal_delta_v = analog_front_end_output_v(analog.light_current_na, analog)
        - analog_front_end_output_v(analog.dark_current_na, analog);
    let lsb_v = policy.adc_full_scale_v / policy.adc_counts as f64;
    let analog_noise_v = policy.analog_noise_rms_mv / 1000.0;
    let adc_noise_v = lsb_v * policy.adc_noise_rms_counts;
    let quantization_guard_v = lsb_v * policy.quantization_guard_lsb;
    let total_noise_v =
        (analog_noise_v.powi(2) + adc_noise_v.powi(2) + quantization_guard_v.powi(2)).sqrt();
    let threshold_v = signal_delta_v * policy.threshold_fraction;
    let negative_control_drift_v = policy.negative_control_drift_mv / 1000.0;
    let signal_to_noise_ratio = signal_delta_v / total_noise_v;
    let false_positive_margin_v = threshold_v - negative_control_drift_v - 3.0 * total_noise_v;
    let detection_margin_v = signal_delta_v - threshold_v - 3.0 * total_noise_v;

    OpticalNoiseMarginMetrics {
        signal_delta_v,
        lsb_v,
        analog_noise_v,
        adc_noise_v,
        quantization_guard_v,
        total_noise_v,
        threshold_v,
        negative_control_drift_v,
        signal_to_noise_ratio,
        false_positive_margin_v,
        detection_margin_v,
    }
}

fn optical_noise_margin_rows(config: &ValidationConfig) -> Vec<OpticalNoiseMarginRow> {
    let policy = &config.optical_noise_margin_policy;
    let metrics = optical_noise_margin_metrics(config);

    vec![
        OpticalNoiseMarginRow {
            id: "signal_delta",
            measurement: "modeled dark-to-light signal delta",
            value: metrics.signal_delta_v,
            units: "V",
            limit: format!(
                ">= {:.4} V",
                config.analog_front_end_policy.min_signal_delta_v
            ),
            pass: metrics.signal_delta_v >= config.analog_front_end_policy.min_signal_delta_v,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "adc_lsb",
            measurement: "ADS1115 voltage per code",
            value: metrics.lsb_v,
            units: "V/count",
            limit: "> 0".to_string(),
            pass: metrics.lsb_v > 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "analog_noise_rms",
            measurement: "assumed analog RMS noise",
            value: metrics.analog_noise_v,
            units: "V",
            limit: "included in total noise".to_string(),
            pass: metrics.analog_noise_v >= 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "adc_noise_rms",
            measurement: "assumed ADC RMS noise",
            value: metrics.adc_noise_v,
            units: "V",
            limit: "included in total noise".to_string(),
            pass: metrics.adc_noise_v >= 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "quantization_guard",
            measurement: "ADC quantization guard band",
            value: metrics.quantization_guard_v,
            units: "V",
            limit: "included in total noise".to_string(),
            pass: metrics.quantization_guard_v >= 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "combined_noise",
            measurement: "combined RMS noise model",
            value: metrics.total_noise_v,
            units: "V",
            limit: "> 0".to_string(),
            pass: metrics.total_noise_v > 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "threshold_voltage",
            measurement: "proposed threshold above dark",
            value: metrics.threshold_v,
            units: "V",
            limit: "inside dark/light signal window".to_string(),
            pass: metrics.threshold_v > 0.0 && metrics.threshold_v < metrics.signal_delta_v,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "negative_control_drift",
            measurement: "negative-control drift allowance",
            value: metrics.negative_control_drift_v,
            units: "V",
            limit: "subtracted from false-positive margin".to_string(),
            pass: metrics.negative_control_drift_v >= 0.0,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "signal_to_noise_ratio",
            measurement: "dark-to-light signal-to-noise ratio",
            value: metrics.signal_to_noise_ratio,
            units: "ratio",
            limit: format!(">= {:.2}", policy.min_signal_to_noise_ratio),
            pass: metrics.signal_to_noise_ratio >= policy.min_signal_to_noise_ratio,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "false_positive_margin",
            measurement: "threshold margin after drift and 3-sigma noise",
            value: metrics.false_positive_margin_v,
            units: "V",
            limit: format!(">= {:.4} V", policy.min_threshold_margin_v),
            pass: metrics.false_positive_margin_v >= policy.min_threshold_margin_v,
            notes: policy.notes.clone(),
        },
        OpticalNoiseMarginRow {
            id: "detection_margin",
            measurement: "remaining light-signal margin after threshold and 3-sigma noise",
            value: metrics.detection_margin_v,
            units: "V",
            limit: format!(">= {:.4} V", policy.min_threshold_margin_v),
            pass: metrics.detection_margin_v >= policy.min_threshold_margin_v,
            notes: policy.notes.clone(),
        },
    ]
}

fn validate_thermistor_adc_transfer(
    config: &ValidationConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    contract_nets: &BTreeSet<String>,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.thermistor_adc_policy;
    let selected = selected_part_ids(parts);
    let test_points = placement
        .test_points
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<BTreeMap<_, _>>();

    for net in [policy.adc_net.as_str(), policy.rail.as_str(), "GND"] {
        push_gate!(
            rows,
            errors,
            "thermistor adc transfer",
            format!("{net} contract net"),
            net,
            "present in contract",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "thermistor adc transfer",
        "thermistor pull-up part",
        policy.pullup_part_id.as_str(),
        "selected in parts.toml",
        selected.contains(policy.pullup_part_id.as_str()),
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "thermistor adc transfer",
        "ADC part",
        policy.adc_part_id.as_str(),
        "selected in parts.toml",
        selected.contains(policy.adc_part_id.as_str()),
        &policy.notes,
    );

    let test_point = test_points.get(policy.test_point.as_str());
    push_gate!(
        rows,
        errors,
        "thermistor adc transfer",
        "ADC test point",
        policy.test_point.as_str(),
        format!("exists on {}", policy.adc_net).as_str(),
        test_point.is_some_and(|point| point.net.as_str() == policy.adc_net.as_str()),
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "thermistor adc transfer",
        "temperature window",
        format!(
            "{:.1}..{:.1} C target {:.1} C",
            policy.min_temp_c, policy.max_temp_c, policy.target_temp_c
        ),
        "min < target < max",
        policy.min_temp_c < policy.target_temp_c && policy.target_temp_c < policy.max_temp_c,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "thermistor adc transfer",
        "divider constants",
        format!(
            "{:.0} ohm pull-up, {:.0} ohm NTC, beta {:.0} K",
            policy.pullup_ohm, policy.thermistor_nominal_ohm, policy.beta_k
        ),
        "positive resistance and beta values",
        policy.pullup_ohm > 0.0 && policy.thermistor_nominal_ohm > 0.0 && policy.beta_k > 0.0,
        &policy.notes,
    );

    for row in thermistor_adc_transfer_rows(config) {
        push_gate!(
            rows,
            errors,
            "thermistor adc transfer",
            row.id.as_str(),
            row.measured.as_str(),
            row.limit.as_str(),
            row.pass,
            row.notes.as_str(),
        );
    }
}

#[derive(Debug)]
struct ThermistorAdcTransferRow {
    id: String,
    temperature_c: Option<f64>,
    thermistor_ohm: Option<f64>,
    voltage_v: f64,
    adc_counts: f64,
    counts_per_c: Option<f64>,
    measured: String,
    limit: String,
    pass: bool,
    notes: String,
}

fn thermistor_adc_transfer_rows(config: &ValidationConfig) -> Vec<ThermistorAdcTransferRow> {
    let policy = &config.thermistor_adc_policy;
    let mut rows = Vec::new();

    for &temp_c in &policy.sample_temps_c {
        let thermistor_ohm = thermistor_resistance_ohm(temp_c, policy);
        let voltage_v = thermistor_adc_voltage_v(thermistor_ohm, policy);
        let adc_counts = thermistor_adc_counts(voltage_v, policy);
        let in_temp_window = temp_c >= policy.min_temp_c && temp_c <= policy.max_temp_c;
        let in_count_window =
            adc_counts >= policy.min_operating_counts && adc_counts <= policy.max_operating_counts;
        rows.push(ThermistorAdcTransferRow {
            id: format!("temp_{temp_c:.0}_c"),
            temperature_c: Some(temp_c),
            thermistor_ohm: Some(thermistor_ohm),
            voltage_v,
            adc_counts,
            counts_per_c: Some(thermistor_counts_per_c(temp_c, policy)),
            measured: format!("{adc_counts:.1} counts at {voltage_v:.3} V"),
            limit: format!(
                "{:.0}..{:.0} counts inside {:.1}..{:.1} C",
                policy.min_operating_counts,
                policy.max_operating_counts,
                policy.min_temp_c,
                policy.max_temp_c
            ),
            pass: in_temp_window && in_count_window,
            notes: policy.notes.clone(),
        });
    }

    let target_counts_per_c = thermistor_counts_per_c(policy.target_temp_c, policy);
    let target_ohm = thermistor_resistance_ohm(policy.target_temp_c, policy);
    let target_v = thermistor_adc_voltage_v(target_ohm, policy);
    rows.push(ThermistorAdcTransferRow {
        id: "target_sensitivity".to_string(),
        temperature_c: Some(policy.target_temp_c),
        thermistor_ohm: Some(target_ohm),
        voltage_v: target_v,
        adc_counts: thermistor_adc_counts(target_v, policy),
        counts_per_c: Some(target_counts_per_c),
        measured: format!("{target_counts_per_c:.1} counts/C"),
        limit: format!(">= {:.1} counts/C", policy.min_counts_per_c_at_target),
        pass: target_counts_per_c >= policy.min_counts_per_c_at_target,
        notes: policy.notes.clone(),
    });

    let open_fault_v = policy.rail_voltage_v;
    rows.push(ThermistorAdcTransferRow {
        id: "open_thermistor_fault".to_string(),
        temperature_c: None,
        thermistor_ohm: None,
        voltage_v: open_fault_v,
        adc_counts: thermistor_adc_counts(open_fault_v, policy),
        counts_per_c: None,
        measured: format!("{open_fault_v:.3} V"),
        limit: format!(">= {:.3} V", policy.open_fault_min_v),
        pass: open_fault_v >= policy.open_fault_min_v,
        notes: "NTC open fault should rail high so firmware can reject the reading.".to_string(),
    });
    let short_fault_v = 0.0;
    rows.push(ThermistorAdcTransferRow {
        id: "short_thermistor_fault".to_string(),
        temperature_c: None,
        thermistor_ohm: None,
        voltage_v: short_fault_v,
        adc_counts: thermistor_adc_counts(short_fault_v, policy),
        counts_per_c: None,
        measured: format!("{short_fault_v:.3} V"),
        limit: format!("<= {:.3} V", policy.short_fault_max_v),
        pass: short_fault_v <= policy.short_fault_max_v,
        notes: "NTC short fault should rail low so firmware can reject the reading.".to_string(),
    });

    rows
}

fn thermistor_resistance_ohm(temp_c: f64, policy: &ThermistorAdcPolicy) -> f64 {
    let temp_k = temp_c + 273.15;
    let reference_k = policy.reference_temp_c + 273.15;
    policy.thermistor_nominal_ohm * (policy.beta_k * (1.0 / temp_k - 1.0 / reference_k)).exp()
}

fn thermistor_adc_voltage_v(thermistor_ohm: f64, policy: &ThermistorAdcPolicy) -> f64 {
    policy.rail_voltage_v * thermistor_ohm / (policy.pullup_ohm + thermistor_ohm)
}

fn thermistor_adc_counts(voltage_v: f64, policy: &ThermistorAdcPolicy) -> f64 {
    voltage_v / policy.adc_full_scale_v * policy.adc_counts as f64
}

fn thermistor_counts_per_c(temp_c: f64, policy: &ThermistorAdcPolicy) -> f64 {
    let delta_c = 0.25;
    let low_ohm = thermistor_resistance_ohm(temp_c - delta_c, policy);
    let high_ohm = thermistor_resistance_ohm(temp_c + delta_c, policy);
    let low_counts = thermistor_adc_counts(thermistor_adc_voltage_v(low_ohm, policy), policy);
    let high_counts = thermistor_adc_counts(thermistor_adc_voltage_v(high_ohm, policy), policy);
    ((high_counts - low_counts) / (2.0 * delta_c)).abs()
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

fn validate_i2c_bus(
    config: &ValidationConfig,
    parts: &PartsManifest,
    contract_nets: &BTreeSet<String>,
    firmware: &FirmwareHandoff,
    placement: &PlacementPlan,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.i2c_bus_policy;
    let selected_part_ids = selected_part_ids(parts);
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
    let rise_time_ns = i2c_rise_time_ns(policy.pullup_ohm, policy.bus_capacitance_pf);
    let address = parse_hex_u8(&policy.required_address);

    for net in [&policy.sda_net, &policy.scl_net] {
        push_gate!(
            rows,
            errors,
            "i2c bus",
            format!("{net} contract net"),
            net.clone(),
            "known contract net",
            contract_nets.contains(net),
            &policy.notes,
        );
    }

    push_gate!(
        rows,
        errors,
        "i2c bus",
        "firmware SDA net",
        firmware.peripherals.i2c_sda_net.clone(),
        policy.sda_net.clone(),
        firmware.peripherals.i2c_sda_net == policy.sda_net,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "firmware SCL net",
        firmware.peripherals.i2c_scl_net.clone(),
        policy.scl_net.clone(),
        firmware.peripherals.i2c_scl_net == policy.scl_net,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "required ADC device",
        firmware.peripherals.adc_device.clone(),
        policy.required_device.clone(),
        firmware.peripherals.adc_device == policy.required_device,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "required ADC address",
        firmware.peripherals.adc_i2c_address.clone(),
        policy.required_address.clone(),
        firmware.peripherals.adc_i2c_address == policy.required_address && address.is_some(),
        &policy.notes,
    );

    push_gate!(
        rows,
        errors,
        "i2c bus",
        "ADC part selected",
        policy.adc_part_id.clone(),
        "selected in parts.toml",
        selected_part_ids.contains(policy.adc_part_id.as_str()),
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "pull-up part selected",
        policy.pullup_part_id.clone(),
        "selected in parts.toml",
        selected_part_ids.contains(policy.pullup_part_id.as_str()),
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "pull-up resistance",
        format!("{:.0} ohm", policy.pullup_ohm),
        "> 0 ohm",
        policy.pullup_ohm > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "bus capacitance estimate",
        format!("{:.1} pF", policy.bus_capacitance_pf),
        "> 0 pF",
        policy.bus_capacitance_pf > 0.0,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "standard-mode bus speed",
        format!("{} Hz", policy.bus_speed_hz),
        format!("<= {} Hz", policy.max_bus_speed_hz),
        policy.bus_speed_hz <= policy.max_bus_speed_hz,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "RC rise time",
        format!("{rise_time_ns:.1} ns"),
        format!("<= {:.1} ns", policy.max_rise_time_ns),
        rise_time_ns <= policy.max_rise_time_ns,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "i2c bus",
        "idle-high threshold",
        format!("{:.3} V", policy.min_idle_high_v),
        format!("<= {:.3} V rail", policy.rail_v),
        policy.min_idle_high_v <= policy.rail_v,
        &policy.notes,
    );

    for test_point in &policy.required_test_points {
        let Some(point) = test_points.get(test_point.as_str()) else {
            push_gate!(
                rows,
                errors,
                "i2c bus",
                format!("{test_point} test point"),
                test_point.clone(),
                "exists in placement.toml",
                false,
                &policy.notes,
            );
            continue;
        };
        let expected_net = if test_point == "TP_SDA" {
            Some(policy.sda_net.as_str())
        } else if test_point == "TP_SCL" {
            Some(policy.scl_net.as_str())
        } else {
            None
        };
        push_gate!(
            rows,
            errors,
            "i2c bus",
            format!("{test_point} test point"),
            point.net.clone(),
            expected_net.unwrap_or("accessible support test point"),
            expected_net.map_or(!point.net.trim().is_empty(), |net| point.net == net),
            &policy.notes,
        );
    }

    for measurement in &policy.required_measurements {
        push_gate!(
            rows,
            errors,
            "i2c bus",
            format!("{measurement} first-article coverage"),
            measurement.clone(),
            "known first-article measurement id",
            measurement_ids.contains(measurement.as_str()),
            &policy.notes,
        );
    }
}

fn validate_heater_protection(
    config: &ValidationConfig,
    parts: &PartsManifest,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let policy = &config.heater_protection_policy;
    let evidence_ids = fault_evidence_ids(config, parts);
    let measurement_ids = config
        .first_article_measurements
        .iter()
        .map(|measurement| measurement.id.as_str())
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "heater protection",
        "minimum heater protection coverage",
        format!("{} checks", config.heater_protection_checks.len()),
        format!(">= {} checks", policy.min_checks),
        config.heater_protection_checks.len() >= policy.min_checks,
        &policy.notes,
    );
    push_gate!(
        rows,
        errors,
        "heater protection",
        "protected current budget",
        format!("{} mA", policy.protected_current_ma),
        format!("<= {} mA fault budget", policy.max_fault_current_ma),
        policy.protected_current_ma <= policy.max_fault_current_ma,
        &policy.notes,
    );

    for evidence in &policy.required_evidence_ids {
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("required evidence {evidence}"),
            evidence.clone(),
            "known selected/external part, path, derating, or analysis id",
            evidence_ids.contains(evidence.as_str()),
            &policy.notes,
        );
    }
    if policy.require_external_cutoff {
        push_gate!(
            rows,
            errors,
            "heater protection",
            "external thermal cutoff evidence",
            "inline_thermal_cutoff",
            "required",
            evidence_ids.contains("inline_thermal_cutoff"),
            &policy.notes,
        );
    }
    for measurement in &policy.required_measurements {
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("required measurement {measurement}"),
            measurement.clone(),
            "known first-article measurement id",
            measurement_ids.contains(measurement.as_str()),
            &policy.notes,
        );
    }

    let mut ids = BTreeSet::new();
    for check in &config.heater_protection_checks {
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("{} unique id", check.id),
            check.id.clone(),
            "unique",
            ids.insert(check.id.as_str()),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("{} current budget", check.id),
            format!("{} mA", check.current_ma),
            format!("<= {} mA protected current", policy.protected_current_ma),
            check.current_ma <= policy.protected_current_ma,
            &check.notes,
        );
        if check.stage.contains("normal") || check.stage.contains("pwm") {
            push_gate!(
                rows,
                errors,
                "heater protection",
                format!("{} normal current budget", check.id),
                format!("{} mA", check.current_ma),
                format!("<= {} mA continuous current", policy.continuous_current_ma),
                check.current_ma <= policy.continuous_current_ma,
                &check.notes,
            );
        }
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("{} trip/limit", check.id),
            check.trip_or_limit.clone(),
            "non-empty",
            !check.trip_or_limit.trim().is_empty(),
            &check.notes,
        );
        push_gate!(
            rows,
            errors,
            "heater protection",
            format!("{} pass criterion", check.id),
            check.pass_criterion.clone(),
            "non-empty",
            !check.pass_criterion.trim().is_empty(),
            &check.notes,
        );

        for part_id in &check.required_parts {
            push_gate!(
                rows,
                errors,
                "heater protection",
                format!("{} part {}", check.id, part_id),
                part_id.clone(),
                "known selected/external part id",
                evidence_ids.contains(part_id.as_str()),
                &check.notes,
            );
        }
        for path_id in &check.required_paths {
            push_gate!(
                rows,
                errors,
                "heater protection",
                format!("{} path {}", check.id, path_id),
                path_id.clone(),
                "known current path id",
                evidence_ids.contains(path_id.as_str()),
                &check.notes,
            );
        }
        for measurement in &check.required_measurements {
            push_gate!(
                rows,
                errors,
                "heater protection",
                format!("{} measurement {}", check.id, measurement),
                measurement.clone(),
                "known first-article measurement id",
                measurement_ids.contains(measurement.as_str()),
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

fn validate_simulation_inputs(
    config: &ValidationConfig,
    rows: &mut Vec<GateRow>,
    errors: &mut Vec<String>,
) {
    let handoff_ids = config
        .external_analysis_handoffs
        .iter()
        .map(|handoff| handoff.id.as_str())
        .collect::<BTreeSet<_>>();

    push_gate!(
        rows,
        errors,
        "simulation inputs",
        "minimum external analysis handoffs",
        format!("{} handoffs", config.external_analysis_handoffs.len()),
        format!(">= {} handoffs", config.simulation_input_policy.min_handoffs),
        config.external_analysis_handoffs.len() >= config.simulation_input_policy.min_handoffs,
        "Simulation-ready release packages must declare the solver classes and inputs before external analysis starts.",
    );

    for required_id in &config.simulation_input_policy.required_handoff_ids {
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("required handoff {}", required_id),
            required_id.clone(),
            "present",
            handoff_ids.contains(required_id.as_str()),
            "Required external analysis handoffs must be present before treating the gate stack as simulation-ready.",
        );
    }

    let mut ids = BTreeSet::new();
    for handoff in &config.external_analysis_handoffs {
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("{} unique id", handoff.id),
            handoff.id.clone(),
            "unique",
            ids.insert(handoff.id.as_str()),
            &handoff.exit_criterion,
        );
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("{} tool class", handoff.id),
            handoff.tool_class.clone(),
            "non-empty",
            !handoff.tool_class.trim().is_empty(),
            &handoff.exit_criterion,
        );
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("{} recommended tools", handoff.id),
            mitigation_list(&handoff.recommended_tools),
            if config.simulation_input_policy.require_recommended_tools {
                "non-empty"
            } else {
                "documented"
            },
            !config.simulation_input_policy.require_recommended_tools
                || !handoff.recommended_tools.is_empty(),
            &handoff.exit_criterion,
        );
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("{} inputs", handoff.id),
            mitigation_list(&handoff.inputs),
            if config.simulation_input_policy.require_inputs {
                "non-empty"
            } else {
                "documented"
            },
            !config.simulation_input_policy.require_inputs || !handoff.inputs.is_empty(),
            &handoff.exit_criterion,
        );
        push_gate!(
            rows,
            errors,
            "simulation inputs",
            format!("{} exit criterion", handoff.id),
            handoff.exit_criterion.clone(),
            if config.simulation_input_policy.require_exit_criterion {
                "non-empty"
            } else {
                "documented"
            },
            !config.simulation_input_policy.require_exit_criterion
                || !handoff.exit_criterion.trim().is_empty(),
            &handoff.exit_criterion,
        );
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

fn write_pdn_dc_simulation_handoff(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    _contract_rails: &BTreeMap<&str, &ContractRail>,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.pdn_dc_simulation_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.pdn_dc_simulation_csv)?;
    writer.write_record([
        "path_id",
        "net",
        "current_ma",
        "nominal_voltage_v",
        "segment_count",
        "routed_length_mm",
        "minimum_width_mm",
        "via_count",
        "trace_resistance_mohm",
        "via_resistance_mohm",
        "total_resistance_mohm",
        "voltage_drop_mv",
        "voltage_drop_pct",
        "path_power_mw",
        "max_voltage_drop_mv",
        "max_voltage_drop_pct",
        "max_path_power_mw",
        "status",
        "notes",
    ])?;

    for row in pdn_dc_simulation_rows(config, routing) {
        let pass = row.segment_count > 0
            && row.voltage_drop_mv <= config.pdn_dc_simulation_policy.max_voltage_drop_mv
            && row.voltage_drop_pct <= config.pdn_dc_simulation_policy.max_voltage_drop_pct
            && row.path_power_mw <= config.pdn_dc_simulation_policy.max_path_power_mw;
        writer.write_record([
            row.path_id.as_str(),
            row.net.as_str(),
            format!("{}", row.current_ma).as_str(),
            format!("{:.3}", row.nominal_voltage_v).as_str(),
            format!("{}", row.segment_count).as_str(),
            format!("{:.3}", row.routed_length_mm).as_str(),
            format_optional_mm(row.minimum_width_mm).as_str(),
            format!("{}", row.via_count).as_str(),
            format!("{:.4}", row.trace_resistance_mohm).as_str(),
            format!("{:.4}", row.via_resistance_mohm).as_str(),
            format!("{:.4}", row.total_resistance_mohm).as_str(),
            format!("{:.4}", row.voltage_drop_mv).as_str(),
            format!("{:.5}", row.voltage_drop_pct).as_str(),
            format!("{:.4}", row.path_power_mw).as_str(),
            format!("{:.3}", config.pdn_dc_simulation_policy.max_voltage_drop_mv).as_str(),
            format!(
                "{:.3}",
                config.pdn_dc_simulation_policy.max_voltage_drop_pct
            )
            .as_str(),
            format!("{:.3}", config.pdn_dc_simulation_policy.max_path_power_mw).as_str(),
            if pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_thermal_margin_simulation_handoff(
    config: &ValidationConfig,
    routing: &RoutingSeed,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.thermal_margin_simulation_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.thermal_margin_simulation_csv)?;
    writer.write_record([
        "id",
        "source_class",
        "source_id",
        "power_w",
        "theta_c_per_w",
        "temp_rise_c",
        "estimated_temp_c",
        "max_temp_c",
        "margin_c",
        "status",
        "notes",
    ])?;

    for row in thermal_margin_simulation_rows(config, routing) {
        writer.write_record([
            row.id.as_str(),
            row.source_class.as_str(),
            row.source_id.as_str(),
            format!("{:.5}", row.power_w).as_str(),
            format!("{:.3}", row.theta_c_per_w).as_str(),
            format!("{:.3}", row.temp_rise_c).as_str(),
            format!("{:.3}", row.estimated_temp_c).as_str(),
            format!("{:.3}", row.max_temp_c).as_str(),
            format!("{:.3}", row.margin_c).as_str(),
            if row.estimated_temp_c <= row.max_temp_c {
                "pass"
            } else {
                "fail"
            },
            row.notes.as_str(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

fn write_heater_pwm_transient_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    write_heater_pwm_transient_csv(config, outputs)?;
    write_heater_pwm_transient_spice(config, outputs)?;
    Ok(())
}

fn write_heater_pwm_transient_csv(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.heater_pwm_transient_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.heater_pwm_transient_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in heater_pwm_transient_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_heater_pwm_transient_spice(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.heater_pwm_transient_netlist)?;
    let policy = &config.heater_pwm_transient_policy;
    let rise_ms = 0.001_f64;
    let fall_ms = 0.001_f64;
    let mut spice = String::new();
    writeln!(
        spice,
        "* LaminarForge LAMP Rev A heater PWM transient check"
    )?;
    writeln!(spice, "* Generated by lamp_rev_a_electrical_validate")?;
    writeln!(
        spice,
        "* This is a first-order switching envelope model, not signoff."
    )?;
    writeln!(
        spice,
        "* check_min_on_current_a={:.6}",
        policy.min_on_current_a
    )?;
    writeln!(
        spice,
        "* check_max_on_current_a={:.6}",
        policy.max_on_current_a
    )?;
    writeln!(
        spice,
        "* check_max_off_current_ma={:.6}",
        policy.max_off_current_ma
    )?;
    writeln!(
        spice,
        "* check_min_gate_high_v={:.6}",
        policy.min_gate_high_v
    )?;
    writeln!(spice, "* check_max_gate_low_v={:.6}", policy.max_gate_low_v)?;
    writeln!(spice, "V12 P12RAW 0 DC {:.6}", policy.supply_voltage_v)?;
    writeln!(
        spice,
        "RFEED P12RAW HEATER_SUPPLY {:.6}",
        policy.heater_series_resistance_ohm
    )?;
    writeln!(spice, "VHEATER HEATER_SUPPLY HEATER_TOP 0")?;
    writeln!(
        spice,
        "RHEATER HEATER_TOP HEATER_P {:.6}",
        policy.heater_resistance_ohm
    )?;
    writeln!(spice, "SLOW HEATER_P 0 GATE 0 HEATER_SWITCH")?;
    writeln!(
        spice,
        ".model HEATER_SWITCH SW(Ron={:.6} Roff={:.6} Vt={:.6} Vh={:.6})",
        policy.mosfet_rds_on_ohm,
        policy.mosfet_off_resistance_ohm,
        policy.gate_threshold_v,
        policy.gate_hysteresis_v
    )?;
    writeln!(
        spice,
        "VGATE GATE 0 PULSE({:.6} {:.6} 0 {:.6}m {:.6}m {:.6}m {:.6}m)",
        policy.gate_drive_low_v,
        policy.gate_drive_high_v,
        rise_ms,
        fall_ms,
        policy.pwm_on_ms,
        policy.pwm_period_ms
    )?;
    writeln!(spice, ".tran 10u {:.6}m", policy.simulation_stop_ms)?;
    writeln!(spice, ".control")?;
    writeln!(spice, "run")?;
    writeln!(
        spice,
        "meas tran heater_current_on AVG i(vheater) FROM={:.6}m TO={:.6}m",
        policy.pwm_on_ms * 0.40,
        policy.pwm_on_ms * 0.80
    )?;
    writeln!(
        spice,
        "meas tran heater_current_off AVG i(vheater) FROM={:.6}m TO={:.6}m",
        policy.pwm_on_ms + (policy.pwm_period_ms - policy.pwm_on_ms) * 0.40,
        policy.pwm_on_ms + (policy.pwm_period_ms - policy.pwm_on_ms) * 0.80
    )?;
    writeln!(
        spice,
        "meas tran gate_high_max MAX v(gate) FROM={:.6}m TO={:.6}m",
        policy.pwm_on_ms * 0.40,
        policy.pwm_on_ms * 0.80
    )?;
    writeln!(
        spice,
        "meas tran gate_low_min MIN v(gate) FROM={:.6}m TO={:.6}m",
        policy.pwm_on_ms + (policy.pwm_period_ms - policy.pwm_on_ms) * 0.40,
        policy.pwm_on_ms + (policy.pwm_period_ms - policy.pwm_on_ms) * 0.80
    )?;
    writeln!(spice, ".endc")?;
    writeln!(spice, ".end")?;
    fs::write(&outputs.heater_pwm_transient_netlist, spice)?;
    Ok(())
}

fn write_heater_thermal_transient_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.heater_thermal_transient_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.heater_thermal_transient_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in heater_thermal_transient_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_usb_inrush_startup_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.usb_inrush_startup_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.usb_inrush_startup_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in usb_inrush_startup_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_power_domain_fault_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    write_power_domain_fault_csv(config, outputs)?;
    write_power_domain_fault_spice(config, outputs)?;
    Ok(())
}

fn write_power_domain_fault_csv(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.power_domain_fault_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.power_domain_fault_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in power_domain_fault_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_power_domain_fault_spice(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.power_domain_fault_netlist)?;
    let policy = &config.power_domain_fault_policy;
    let start_ms = policy.simulation_stop_ms * 0.50;
    let mut spice = String::new();
    writeln!(
        spice,
        "* LaminarForge LAMP Rev A power-domain fault/backfeed transient check"
    )?;
    writeln!(spice, "* Generated by lamp_rev_a_electrical_validate")?;
    writeln!(
        spice,
        "* This is a first-order 12 V heater-domain isolation/backfeed model, not safety certification."
    )?;
    writeln!(
        spice,
        "* check_max_vbus_fault_v={:.6}",
        policy.max_vbus_fault_v
    )?;
    writeln!(
        spice,
        "* check_max_5v_fault_v={:.6}",
        policy.max_five_v_fault_v
    )?;
    writeln!(
        spice,
        "* check_max_3v3_fault_v={:.6}",
        policy.max_three_v_three_fault_v
    )?;
    writeln!(
        spice,
        "* check_max_usb_backfeed_ma={:.6}",
        policy.max_usb_backfeed_ma
    )?;
    writeln!(
        spice,
        "* check_max_regulator_reverse_ma={:.6}",
        policy.max_regulator_reverse_ma
    )?;
    writeln!(
        spice,
        "VFAULT P12FAULT 0 DC {:.6}",
        policy.heater_fault_voltage_v
    )?;
    writeln!(spice, "VISO_VBUS P12FAULT ISO_VBUS 0")?;
    writeln!(
        spice,
        "RISO_VBUS ISO_VBUS VBUS {:.6}",
        policy.heater_to_vbus_isolation_ohm
    )?;
    writeln!(spice, "VISO_5V P12FAULT ISO_5V 0")?;
    writeln!(
        spice,
        "RISO_5V ISO_5V P5V {:.6}",
        policy.heater_to_five_v_isolation_ohm
    )?;
    writeln!(spice, "VISO_3V3 P12FAULT ISO_3V3 0")?;
    writeln!(
        spice,
        "RISO_3V3 ISO_3V3 P3V3 {:.6}",
        policy.heater_to_three_v_three_isolation_ohm
    )?;
    writeln!(spice, "VREGREV P5V REGREV_SENSE 0")?;
    writeln!(
        spice,
        "RREGREV REGREV_SENSE P3V3 {:.6}",
        policy.regulator_reverse_resistance_ohm
    )?;
    writeln!(spice, "RVBUS_LOAD VBUS 0 {:.6}", policy.vbus_load_ohm)?;
    writeln!(spice, "R5V_LOAD P5V 0 {:.6}", policy.five_v_load_ohm)?;
    writeln!(
        spice,
        "R3V3_LOAD P3V3 0 {:.6}",
        policy.three_v_three_load_ohm
    )?;
    writeln!(spice, "CVBUS VBUS 0 1n IC=0")?;
    writeln!(spice, "C5V P5V 0 1n IC=0")?;
    writeln!(spice, "C3V3 P3V3 0 1n IC=0")?;
    writeln!(spice, ".tran 10u {:.6}m uic", policy.simulation_stop_ms)?;
    writeln!(spice, ".control")?;
    writeln!(spice, "run")?;
    writeln!(
        spice,
        "meas tran vbus_fault_v MAX v(vbus) FROM={start_ms:.6}m TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran five_v_fault_v MAX v(p5v) FROM={start_ms:.6}m TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran three_v_three_fault_v MAX v(p3v3) FROM={start_ms:.6}m TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran usb_backfeed_a MIN i(viso_vbus) FROM={start_ms:.6}m TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran regulator_reverse_a MIN i(vregrev) FROM={start_ms:.6}m TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(spice, ".endc")?;
    writeln!(spice, ".end")?;
    fs::write(&outputs.power_domain_fault_netlist, spice)?;
    Ok(())
}

fn write_rail_load_step_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    write_rail_load_step_csv(config, outputs)?;
    write_rail_load_step_spice(config, outputs)?;
    Ok(())
}

fn write_rail_load_step_csv(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.rail_load_step_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.rail_load_step_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in rail_load_step_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_rail_load_step_spice(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.rail_load_step_netlist)?;
    let policy = &config.rail_load_step_policy;
    let rise_ms = 0.010_f64;
    let fall_ms = 0.010_f64;
    let mut spice = String::new();
    writeln!(
        spice,
        "* LaminarForge LAMP Rev A 3.3 V rail load-step transient check"
    )?;
    writeln!(spice, "* Generated by lamp_rev_a_electrical_validate")?;
    writeln!(
        spice,
        "* This is a first-order rail sag/load burst model, not signoff."
    )?;
    writeln!(
        spice,
        "* check_min_rail_voltage_v={:.6}",
        policy.min_rail_voltage_v
    )?;
    writeln!(
        spice,
        "* check_max_source_current_ma={:.6}",
        policy.max_source_current_ma as f64
    )?;
    writeln!(spice, "V3SRC P3SRC 0 DC {:.6}", policy.nominal_voltage_v)?;
    writeln!(spice, "RSRC P3SRC P3V3 {:.6}", policy.source_resistance_ohm)?;
    writeln!(
        spice,
        "C3V3 P3V3 0 {:.6}u IC={:.6}",
        policy.bulk_capacitance_u, policy.nominal_voltage_v
    )?;
    writeln!(
        spice,
        "ILOAD P3V3 0 PULSE({:.6} {:.6} {:.6}m {:.6}m {:.6}m {:.6}m {:.6}m)",
        policy.baseline_load_ma as f64 / 1000.0,
        policy.burst_load_ma as f64 / 1000.0,
        policy.burst_start_ms,
        rise_ms,
        fall_ms,
        policy.burst_width_ms,
        policy.period_ms
    )?;
    writeln!(spice, ".tran 10u {:.6}m uic", policy.simulation_stop_ms)?;
    writeln!(spice, ".control")?;
    writeln!(spice, "run")?;
    writeln!(
        spice,
        "meas tran rail_min_v MIN v(p3v3) FROM={:.6}m TO={:.6}m",
        policy.burst_start_ms, policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran source_current_max MIN i(v3src) FROM={:.6}m TO={:.6}m",
        policy.burst_start_ms, policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran rail_recovery_v AVG v(p3v3) FROM={:.6}m TO={:.6}m",
        policy.simulation_stop_ms - policy.period_ms * 0.50,
        policy.simulation_stop_ms - policy.period_ms * 0.10
    )?;
    writeln!(spice, ".endc")?;
    writeln!(spice, ".end")?;
    fs::write(&outputs.rail_load_step_netlist, spice)?;
    Ok(())
}

fn write_analog_front_end_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    write_analog_front_end_csv(config, outputs)?;
    write_analog_front_end_spice(config, outputs)?;
    Ok(())
}

fn write_analog_front_end_csv(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.analog_front_end_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.analog_front_end_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in analog_front_end_rows(config) {
        writer.write_record([
            row.id,
            row.measurement,
            format!("{:.6}", row.value).as_str(),
            row.units,
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_analog_front_end_spice(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.analog_front_end_netlist)?;
    let policy = &config.analog_front_end_policy;
    let rise_ms = 0.010_f64;
    let fall_ms = 0.010_f64;
    let mut spice = String::new();
    writeln!(
        spice,
        "* LaminarForge LAMP Rev A optical analog front-end transient check"
    )?;
    writeln!(spice, "* Generated by lamp_rev_a_electrical_validate")?;
    writeln!(
        spice,
        "* This is a first-order photodiode/mux/TIA envelope model, not optical signoff."
    )?;
    writeln!(
        spice,
        "* check_min_signal_delta_v={:.6}",
        policy.min_signal_delta_v
    )?;
    writeln!(
        spice,
        "* check_min_adc_voltage_v={:.6}",
        policy.min_adc_voltage_v
    )?;
    writeln!(
        spice,
        "* check_max_adc_voltage_v={:.6}",
        policy.max_adc_voltage_v
    )?;
    writeln!(spice, "VREF VREF 0 DC 0")?;
    writeln!(
        spice,
        "IPD VREF MUX_COM PULSE({:.12} {:.12} {:.6}m {:.6}m {:.6}m {:.6}m {:.6}m)",
        policy.dark_current_na * 1.0e-9,
        policy.light_current_na * 1.0e-9,
        policy.light_start_ms,
        rise_ms,
        fall_ms,
        policy.light_width_ms,
        policy.period_ms
    )?;
    writeln!(
        spice,
        "RMUX MUX_COM ADC_AIN1 {:.6}",
        policy.mux_on_resistance_ohm
    )?;
    writeln!(
        spice,
        "RF ADC_AIN1 VREF {:.6}",
        policy.feedback_resistor_ohm
    )?;
    writeln!(
        spice,
        "CF ADC_AIN1 VREF {:.6}p IC=0",
        policy.feedback_cap_pf
    )?;
    writeln!(spice, "CIN MUX_COM VREF {:.6}p IC=0", policy.input_cap_pf)?;
    writeln!(spice, ".tran 1u {:.6}m uic", policy.simulation_stop_ms)?;
    writeln!(spice, ".control")?;
    writeln!(spice, "run")?;
    writeln!(
        spice,
        "meas tran dark_v AVG v(adc_ain1) FROM={:.6}m TO={:.6}m",
        policy.light_start_ms * 0.20,
        policy.light_start_ms * 0.80
    )?;
    writeln!(
        spice,
        "meas tran light_v AVG v(adc_ain1) FROM={:.6}m TO={:.6}m",
        policy.light_start_ms + policy.light_width_ms * 0.40,
        policy.light_start_ms + policy.light_width_ms * 0.80
    )?;
    writeln!(
        spice,
        "meas tran adc_max_v MAX v(adc_ain1) FROM=0 TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(
        spice,
        "meas tran adc_min_v MIN v(adc_ain1) FROM=0 TO={:.6}m",
        policy.simulation_stop_ms
    )?;
    writeln!(spice, ".endc")?;
    writeln!(spice, ".end")?;
    fs::write(&outputs.analog_front_end_netlist, spice)?;
    Ok(())
}

fn write_optical_crosstalk_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.optical_crosstalk_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.optical_crosstalk_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in optical_crosstalk_rows(config) {
        writer.write_record([
            row.id.to_string(),
            row.measurement.to_string(),
            format!("{:.6}", row.value),
            row.units.to_string(),
            row.limit,
            if row.pass {
                "pass".to_string()
            } else {
                "fail".to_string()
            },
            row.notes,
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_optical_noise_margin_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.optical_noise_margin_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.optical_noise_margin_csv)?;
    writer.write_record([
        "id",
        "measurement",
        "value",
        "units",
        "limit",
        "status",
        "notes",
    ])?;
    for row in optical_noise_margin_rows(config) {
        writer.write_record([
            row.id.to_string(),
            row.measurement.to_string(),
            format!("{:.6}", row.value),
            row.units.to_string(),
            row.limit,
            if row.pass {
                "pass".to_string()
            } else {
                "fail".to_string()
            },
            row.notes,
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_thermistor_adc_transfer_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.thermistor_adc_transfer_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.thermistor_adc_transfer_csv)?;
    writer.write_record([
        "id",
        "temperature_c",
        "thermistor_ohm",
        "voltage_v",
        "adc_counts",
        "counts_per_c",
        "limit",
        "status",
        "notes",
    ])?;
    for row in thermistor_adc_transfer_rows(config) {
        writer.write_record([
            row.id.as_str(),
            format_optional_f64(row.temperature_c).as_str(),
            format_optional_f64(row.thermistor_ohm).as_str(),
            format!("{:.6}", row.voltage_v).as_str(),
            format!("{:.1}", row.adc_counts).as_str(),
            format_optional_f64(row.counts_per_c).as_str(),
            row.limit.as_str(),
            if row.pass { "pass" } else { "fail" },
            row.notes.as_str(),
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

fn write_i2c_bus_handoff(
    config: &ValidationConfig,
    parts: &PartsManifest,
    firmware: &FirmwareHandoff,
    placement: &PlacementPlan,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.i2c_bus_csv)?;
    let policy = &config.i2c_bus_policy;
    let selected_part_ids = selected_part_ids(parts);
    let test_points = placement
        .test_points
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<BTreeMap<_, _>>();
    let rise_time_ns = i2c_rise_time_ns(policy.pullup_ohm, policy.bus_capacitance_pf);
    let address = parse_hex_u8(&policy.required_address)
        .map(|address| format!("0x{address:02X}"))
        .unwrap_or_else(|| "invalid".to_string());

    let mut writer = csv::Writer::from_path(&outputs.i2c_bus_csv)?;
    writer.write_record([
        "sda_net",
        "scl_net",
        "pullup_part_id",
        "pullup_selected",
        "adc_part_id",
        "adc_selected",
        "required_device",
        "required_address",
        "firmware_address",
        "bus_speed_hz",
        "pullup_ohm",
        "bus_capacitance_pf",
        "rise_time_ns",
        "max_rise_time_ns",
        "min_idle_high_v",
        "required_test_points",
        "required_measurements",
        "notes",
    ])?;
    writer.write_record([
        policy.sda_net.as_str(),
        policy.scl_net.as_str(),
        policy.pullup_part_id.as_str(),
        selected_part_ids
            .contains(policy.pullup_part_id.as_str())
            .to_string()
            .as_str(),
        policy.adc_part_id.as_str(),
        selected_part_ids
            .contains(policy.adc_part_id.as_str())
            .to_string()
            .as_str(),
        policy.required_device.as_str(),
        address.as_str(),
        firmware.peripherals.adc_i2c_address.as_str(),
        policy.bus_speed_hz.to_string().as_str(),
        format!("{:.0}", policy.pullup_ohm).as_str(),
        format!("{:.1}", policy.bus_capacitance_pf).as_str(),
        format!("{rise_time_ns:.1}").as_str(),
        format!("{:.1}", policy.max_rise_time_ns).as_str(),
        format!("{:.3}", policy.min_idle_high_v).as_str(),
        policy
            .required_test_points
            .iter()
            .map(|test_point| {
                let status = if test_points.contains_key(test_point.as_str()) {
                    "present"
                } else {
                    "missing"
                };
                format!("{test_point}:{status}")
            })
            .collect::<Vec<_>>()
            .join(";")
            .as_str(),
        mitigation_list(&policy.required_measurements).as_str(),
        policy.notes.as_str(),
    ])?;

    writer.flush()?;
    Ok(())
}

fn write_heater_protection_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.heater_protection_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.heater_protection_csv)?;
    writer.write_record([
        "id",
        "stage",
        "current_ma",
        "continuous_current_ma",
        "protected_current_ma",
        "required_parts",
        "required_paths",
        "required_measurements",
        "trip_or_limit",
        "pass_criterion",
        "notes",
    ])?;

    for check in &config.heater_protection_checks {
        writer.write_record([
            check.id.as_str(),
            check.stage.as_str(),
            check.current_ma.to_string().as_str(),
            config
                .heater_protection_policy
                .continuous_current_ma
                .to_string()
                .as_str(),
            config
                .heater_protection_policy
                .protected_current_ma
                .to_string()
                .as_str(),
            mitigation_list(&check.required_parts).as_str(),
            mitigation_list(&check.required_paths).as_str(),
            mitigation_list(&check.required_measurements).as_str(),
            check.trip_or_limit.as_str(),
            check.pass_criterion.as_str(),
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

fn write_simulation_inputs_handoff(
    config: &ValidationConfig,
    outputs: &ElectricalOutputPaths,
) -> Result<(), Box<dyn Error>> {
    ensure_parent(&outputs.simulation_inputs_csv)?;
    let mut writer = csv::Writer::from_path(&outputs.simulation_inputs_csv)?;
    writer.write_record([
        "id",
        "tool_class",
        "recommended_tools",
        "inputs",
        "exit_criterion",
    ])?;

    for handoff in &config.external_analysis_handoffs {
        writer.write_record([
            handoff.id.as_str(),
            handoff.tool_class.as_str(),
            mitigation_list(&handoff.recommended_tools).as_str(),
            mitigation_list(&handoff.inputs).as_str(),
            handoff.exit_criterion.as_str(),
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

fn format_optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}"))
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

fn i2c_rise_time_ns(pullup_ohm: f64, bus_capacitance_pf: f64) -> f64 {
    0.8473 * pullup_ohm * bus_capacitance_pf * 1e-3
}

fn parse_hex_u8(value: &str) -> Option<u8> {
    u8::from_str_radix(value.trim().trim_start_matches("0x"), 16).ok()
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
    writeln!(
        report,
        "- Heater PWM transient netlist: `{}`",
        outputs
            .heater_pwm_transient_netlist
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lamp_rev_a_heater_pwm_transient.spice")
    )?;
    writeln!(
        report,
        "- 3.3 V rail load-step transient netlist: `{}`",
        outputs
            .rail_load_step_netlist
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lamp_rev_a_rail_load_step.spice")
    )?;
    writeln!(
        report,
        "- Power-domain fault/backfeed transient netlist: `{}`",
        outputs
            .power_domain_fault_netlist
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lamp_rev_a_power_domain_fault.spice")
    )?;
    writeln!(
        report,
        "- Optical analog front-end transient netlist: `{}`",
        outputs
            .analog_front_end_netlist
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("lamp_rev_a_analog_front_end.spice")
    )?;
    writeln!(report)?;
    writeln!(report, "## Generated PDN / Thermal Handoffs")?;
    writeln!(report)?;
    writeln!(
        report,
        "- Simulation input package: `{}`",
        outputs
            .simulation_inputs_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("simulation_inputs.csv")
    )?;
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
        "- DC PDN simulation table: `{}`",
        outputs
            .pdn_dc_simulation_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("pdn_dc_simulation.csv")
    )?;
    writeln!(
        report,
        "- Thermal margin simulation table: `{}`",
        outputs
            .thermal_margin_simulation_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("thermal_margin_simulation.csv")
    )?;
    writeln!(
        report,
        "- Heater PWM transient result table: `{}`",
        outputs
            .heater_pwm_transient_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("heater_pwm_transient_simulation.csv")
    )?;
    writeln!(
        report,
        "- Heater/reaction-block thermal transient table: `{}`",
        outputs
            .heater_thermal_transient_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("heater_thermal_transient_simulation.csv")
    )?;
    writeln!(
        report,
        "- USB/VBUS hot-plug and 3.3 V startup table: `{}`",
        outputs
            .usb_inrush_startup_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("usb_inrush_startup_simulation.csv")
    )?;
    writeln!(
        report,
        "- 3.3 V rail load-step result table: `{}`",
        outputs
            .rail_load_step_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("rail_load_step_simulation.csv")
    )?;
    writeln!(
        report,
        "- Power-domain fault/backfeed result table: `{}`",
        outputs
            .power_domain_fault_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("power_domain_fault_simulation.csv")
    )?;
    writeln!(
        report,
        "- Optical analog front-end result table: `{}`",
        outputs
            .analog_front_end_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("analog_front_end_simulation.csv")
    )?;
    writeln!(
        report,
        "- Optical crosstalk and mux/ADC settling table: `{}`",
        outputs
            .optical_crosstalk_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("optical_crosstalk_validation.csv")
    )?;
    writeln!(
        report,
        "- Optical signal/noise threshold-margin table: `{}`",
        outputs
            .optical_noise_margin_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("optical_noise_margin.csv")
    )?;
    writeln!(
        report,
        "- Thermistor/ADC transfer table: `{}`",
        outputs
            .thermistor_adc_transfer_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("thermistor_adc_transfer.csv")
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
        "- I2C bus validation table: `{}`",
        outputs
            .i2c_bus_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("i2c_bus_validation.csv")
    )?;
    writeln!(
        report,
        "- Heater protection coordination table: `{}`",
        outputs
            .heater_protection_csv
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("heater_protection_coordination.csv")
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
