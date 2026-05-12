use laminarforge_cad::lamp_rev_a_electrical::{validate_to_outputs, ElectricalOutputPaths};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

const CONFIG_PATH: &str = "pcb/lamp_rev_a/fab_release.toml";

#[derive(Debug, Deserialize)]
struct ReleaseConfig {
    package: Package,
    toolchain: ToolchainConfig,
    inputs: Inputs,
    outputs: Outputs,
    assembly: AssemblyConfig,
    review: ReviewConfig,
    bringup: BringupConfig,
    source_snapshot: SourceSnapshotConfig,
    gerbers: GerberConfig,
    drills: DrillConfig,
    position: PositionConfig,
    step: StepConfig,
    gates: Gates,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    revision: String,
}

#[derive(Debug, Deserialize)]
struct ToolchainConfig {
    min_kicad_major: u32,
}

#[derive(Debug, Deserialize)]
struct Inputs {
    board: String,
    schematic: String,
    contract: String,
    parts: String,
    placement: String,
    pin_nets: String,
    firmware_handoff: String,
    electrical_validation: String,
}

#[derive(Debug, Deserialize)]
struct Outputs {
    root_dir: String,
    reports_dir: String,
    gerbers_dir: String,
    drills_dir: String,
    assembly_dir: String,
    review_dir: String,
    bundles_dir: String,
    drc_report: String,
    erc_report: String,
    drill_report: String,
    bom_file: String,
    cpl_file: String,
    manual_file: String,
    position_file: String,
    step_file: String,
    order_audit_file: String,
    bringup_file: String,
    firmware_handoff_file: String,
    electrical_validation_file: String,
    electrical_validation_gates_file: String,
    spice_netlist_file: String,
    simulation_handoff_file: String,
    simulation_inputs_file: String,
    pdn_current_paths_file: String,
    thermal_power_file: String,
    first_article_measurements_file: String,
    component_derating_file: String,
    fault_fmea_file: String,
    emc_esd_file: String,
    startup_safety_file: String,
    manufacturing_test_file: String,
    calibration_readiness_file: String,
    validation_traceability_file: String,
    pdn_dc_simulation_file: String,
    bundle_checksums_file: String,
    manifest_file: String,
    fabrication_bundle: String,
    assembly_bundle: String,
    review_bundle: String,
    source_bundle: String,
}

#[derive(Debug, Deserialize)]
struct AssemblyConfig {
    machine_side: String,
    manual_part_ids: Vec<String>,
    require_cpl_matches_kicad_position: bool,
}

#[derive(Debug, Deserialize)]
struct ReviewConfig {
    remaining_gates: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BringupConfig {
    usb_power_limit_ma: u32,
    heater_power_limit_ma: u32,
    required_test_points: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SourceSnapshotConfig {
    files: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GerberConfig {
    layers: Vec<String>,
    check_zones: bool,
    precision: u8,
    use_board_plot_params: bool,
}

#[derive(Debug, Deserialize)]
struct DrillConfig {
    format: String,
    units: String,
    origin: String,
    zeros_format: String,
    oval_format: String,
    separate_th: bool,
    generate_map: bool,
    generate_report: bool,
}

#[derive(Debug, Deserialize)]
struct PositionConfig {
    side: String,
    format: String,
    units: String,
    smd_only: bool,
    exclude_dnp: bool,
}

#[derive(Debug, Deserialize)]
struct StepConfig {
    enabled: bool,
    board_only: bool,
    force: bool,
}

#[derive(Debug, Deserialize)]
struct Gates {
    allow_kicad_self_zone_unconnected: bool,
    max_physical_drc_violations: usize,
    max_real_unconnected_items: usize,
    max_erc_violations: usize,
}

#[derive(Debug, Clone, Copy)]
struct ReleaseGateResults {
    drc_violations: usize,
    real_unconnected: usize,
    ignored_self_zone: usize,
    erc_violations: usize,
}

#[derive(Debug, Deserialize)]
struct PartsManifest {
    selected_parts: Vec<SelectedPart>,
}

#[derive(Debug, Deserialize)]
struct SelectedPart {
    id: String,
    quantity: u32,
    value: String,
    footprint: String,
    lcsc_part: String,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    placements: Vec<Placement>,
    test_points: Vec<TestPoint>,
}

#[derive(Debug, Deserialize)]
struct Placement {
    reference: String,
    part_id: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct TestPoint {
    name: String,
    net: String,
    x_mm: f64,
    y_mm: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct Contract {
    board: BoardContract,
    nets: Vec<ContractNet>,
    #[serde(default)]
    net_groups: Vec<ContractNetGroup>,
    gpio_map: Vec<GpioMap>,
}

#[derive(Debug, Deserialize)]
struct BoardContract {
    slot_count: usize,
    primary_mcu: String,
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
struct GpioMap {
    esp32_module_pin: u32,
    net: String,
    function: String,
    locked: bool,
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
    package: FirmwareHandoffPackage,
    datasheet: FirmwareDatasheet,
    mcu: FirmwareMcu,
    peripherals: FirmwarePeripherals,
    module_pins: Vec<FirmwareModulePin>,
    slots: Vec<FirmwareSlot>,
}

#[derive(Debug, Deserialize)]
struct FirmwareHandoffPackage {
    revision: String,
}

#[derive(Debug, Deserialize)]
struct FirmwareDatasheet {
    source: String,
    url: String,
    pin_table: String,
}

#[derive(Debug, Deserialize)]
struct FirmwareMcu {
    reference: String,
    module: String,
    module_pin_unit: String,
    firmware_pin_unit: String,
}

#[derive(Debug, Deserialize)]
struct FirmwarePeripherals {
    i2c_sda_net: String,
    i2c_scl_net: String,
    adc_device: String,
    adc_i2c_address: String,
    adc_input_net: String,
    mux_common_net: String,
    mux_select_nets: Vec<String>,
    heater_pwm_net: String,
    usb_dn_net: String,
    usb_dp_net: String,
    uart_rx_net: String,
    uart_tx_net: String,
    boot_net: String,
    enable_net: String,
    activity_net: String,
    notes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FirmwareModulePin {
    module_pin: u32,
    soc_gpio: i32,
    net: String,
    role: String,
    firmware_direction: String,
    boot_sensitive: bool,
}

#[derive(Debug, Deserialize)]
struct FirmwareSlot {
    slot: usize,
    led_net: String,
    mux_channel_net: String,
    select_bits: Vec<u8>,
}

#[derive(Debug, Deserialize)]
struct KiCadReport {
    #[serde(default)]
    violations: Vec<ReportEntry>,
    #[serde(default)]
    unconnected_items: Vec<ReportEntry>,
}

#[derive(Debug, Deserialize)]
struct ReportEntry {
    #[serde(default)]
    items: Vec<ReportItem>,
}

#[derive(Debug, Deserialize)]
struct ReportItem {
    description: String,
    #[serde(default)]
    uuid: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let config = read_toml::<ReleaseConfig>(&root.join(CONFIG_PATH))?;
    validate_config(&config)?;

    let output_root = root.join(&config.outputs.root_dir);
    if output_root.exists() {
        fs::remove_dir_all(&output_root)?;
    }
    create_output_dirs(&output_root, &config.outputs)?;

    let board = root.join(&config.inputs.board);
    let schematic = root.join(&config.inputs.schematic);
    let contract_path = root.join(&config.inputs.contract);
    let parts_path = root.join(&config.inputs.parts);
    let placement_path = root.join(&config.inputs.placement);
    let pin_nets_path = root.join(&config.inputs.pin_nets);
    let firmware_handoff_path = root.join(&config.inputs.firmware_handoff);
    let electrical_validation_path = root.join(&config.inputs.electrical_validation);
    ensure_file(&board)?;
    ensure_file(&schematic)?;
    ensure_file(&contract_path)?;
    ensure_file(&parts_path)?;
    ensure_file(&placement_path)?;
    ensure_file(&pin_nets_path)?;
    ensure_file(&firmware_handoff_path)?;
    ensure_file(&electrical_validation_path)?;

    validate_kicad_version(config.toolchain.min_kicad_major)?;

    let drc_report = output_root.join(&config.outputs.drc_report);
    let erc_report = output_root.join(&config.outputs.erc_report);

    run_kicad_drc(&board, &drc_report)?;
    let (drc_violations, real_unconnected, ignored_self_zone) =
        validate_drc_report(&drc_report, &config.gates)?;

    run_kicad_erc(&schematic, &erc_report)?;
    let erc_violations = validate_erc_report(&erc_report, &config.gates)?;
    let gate_results = ReleaseGateResults {
        drc_violations,
        real_unconnected,
        ignored_self_zone,
        erc_violations,
    };

    let parts = read_toml::<PartsManifest>(&parts_path)?;
    let placement = read_toml::<PlacementPlan>(&placement_path)?;
    let contract = read_toml::<Contract>(&contract_path)?;
    let pin_nets = read_toml::<PinNetManifest>(&pin_nets_path)?;
    let firmware_handoff = read_toml::<FirmwareHandoff>(&firmware_handoff_path)?;
    validate_assembly_sources(&parts, &placement)?;
    validate_bringup_sources(&config.bringup, &placement)?;
    validate_firmware_handoff_sources(&contract, &pin_nets, &firmware_handoff)?;
    write_bom(
        &parts,
        &placement,
        &config.assembly,
        &output_root.join(&config.outputs.bom_file),
    )?;
    write_cpl(
        &parts,
        &placement,
        &config.assembly,
        &output_root.join(&config.outputs.cpl_file),
    )?;
    write_manual_parts(
        &parts,
        &placement,
        &config.assembly,
        &output_root.join(&config.outputs.manual_file),
    )?;

    run_gerber_export(&config, &board, &output_root)?;
    run_drill_export(&config, &board, &output_root)?;
    run_position_export(&config, &board, &output_root)?;
    if config.step.enabled {
        run_step_export(&config, &board, &output_root)?;
    }
    write_manifest(&config, &output_root, gate_results)?;
    write_order_audit_report(&config, &parts, &placement, &output_root, gate_results)?;
    write_bringup_checklist(&config, &placement, &output_root)?;
    write_firmware_handoff(
        &config,
        &contract,
        &placement,
        &firmware_handoff,
        &output_root,
    )?;
    let electrical_outputs = ElectricalOutputPaths {
        report_md: output_root.join(&config.outputs.electrical_validation_file),
        gates_csv: output_root.join(&config.outputs.electrical_validation_gates_file),
        spice_netlist: output_root.join(&config.outputs.spice_netlist_file),
        simulation_handoff_md: output_root.join(&config.outputs.simulation_handoff_file),
        simulation_inputs_csv: output_root.join(&config.outputs.simulation_inputs_file),
        pdn_current_paths_csv: output_root.join(&config.outputs.pdn_current_paths_file),
        thermal_power_csv: output_root.join(&config.outputs.thermal_power_file),
        first_article_measurements_csv: output_root
            .join(&config.outputs.first_article_measurements_file),
        component_derating_csv: output_root.join(&config.outputs.component_derating_file),
        fault_fmea_csv: output_root.join(&config.outputs.fault_fmea_file),
        emc_esd_csv: output_root.join(&config.outputs.emc_esd_file),
        startup_safety_csv: output_root.join(&config.outputs.startup_safety_file),
        manufacturing_test_csv: output_root.join(&config.outputs.manufacturing_test_file),
        calibration_readiness_csv: output_root.join(&config.outputs.calibration_readiness_file),
        validation_traceability_csv: output_root.join(&config.outputs.validation_traceability_file),
        pdn_dc_simulation_csv: output_root.join(&config.outputs.pdn_dc_simulation_file),
    };
    validate_to_outputs(&root, &electrical_outputs)?;
    write_release_bundles(&config, &root, &output_root)?;
    validate_release_outputs(&config, &parts, &placement, &output_root)?;

    println!("Wrote LAMP Rev A fab release:");
    println!("  {}", output_root.display());
    println!("  DRC violations: {drc_violations}");
    println!("  real unconnected items: {real_unconnected}");
    println!("  ignored KiCad self-zone items: {ignored_self_zone}");
    println!("  ERC violations: {erc_violations}");
    Ok(())
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

fn validate_config(config: &ReleaseConfig) -> Result<(), Box<dyn Error>> {
    if config.package.name != "lamp_rev_a_fab_release" {
        return Err(format!("unexpected release package {}", config.package.name).into());
    }
    if config.package.revision != "Rev A" {
        return Err(format!("unexpected release revision {}", config.package.revision).into());
    }
    if config.gerbers.layers.is_empty() {
        return Err("gerber layer list cannot be empty".into());
    }
    if config.assembly.machine_side != "top" {
        return Err("Rev A release supports top-side machine assembly only".into());
    }
    if config.review.remaining_gates.is_empty() {
        return Err("fab release review remaining_gates cannot be empty".into());
    }
    if config.bringup.required_test_points.is_empty() {
        return Err("fab release bringup required_test_points cannot be empty".into());
    }
    if config.bringup.usb_power_limit_ma == 0 || config.bringup.heater_power_limit_ma == 0 {
        return Err("fab release bringup current limits must be greater than zero".into());
    }
    if config.source_snapshot.files.is_empty() {
        return Err("fab release source_snapshot files cannot be empty".into());
    }
    if config.gerbers.precision != 5 && config.gerbers.precision != 6 {
        return Err("gerber precision must be 5 or 6".into());
    }
    for layer in &config.gerbers.layers {
        if layer.trim().is_empty() {
            return Err("gerber layer names cannot be empty".into());
        }
    }
    Ok(())
}

fn create_output_dirs(output_root: &Path, outputs: &Outputs) -> Result<(), Box<dyn Error>> {
    for dir in [
        &outputs.reports_dir,
        &outputs.gerbers_dir,
        &outputs.drills_dir,
        &outputs.assembly_dir,
        &outputs.review_dir,
        &outputs.bundles_dir,
    ] {
        fs::create_dir_all(output_root.join(dir))?;
    }
    Ok(())
}

fn ensure_file(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.is_file() {
        return Err(format!("required file is missing: {}", path.display()).into());
    }
    Ok(())
}

fn validate_kicad_version(min_major: u32) -> Result<(), Box<dyn Error>> {
    let output = Command::new("kicad-cli").arg("version").output()?;
    if !output.status.success() {
        return Err(format!(
            "kicad-cli version failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let version = String::from_utf8_lossy(&output.stdout);
    let major = version
        .trim()
        .split('.')
        .next()
        .ok_or("kicad-cli version output is empty")?
        .parse::<u32>()?;
    if major < min_major {
        return Err(format!(
            "KiCad {version} is too old for LAMP Rev A fab release; require major version {min_major} or newer"
        )
        .into());
    }
    Ok(())
}

fn run_kicad_drc(board: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    run_command(
        "kicad-cli",
        &[
            "pcb".to_string(),
            "drc".to_string(),
            "--refill-zones".to_string(),
            "--output".to_string(),
            output.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--severity-all".to_string(),
            board.display().to_string(),
        ],
    )
}

fn run_kicad_erc(schematic: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    run_command(
        "kicad-cli",
        &[
            "sch".to_string(),
            "erc".to_string(),
            schematic.display().to_string(),
            "-o".to_string(),
            output.display().to_string(),
            "--format".to_string(),
            "json".to_string(),
        ],
    )
}

fn validate_drc_report(
    path: &Path,
    gates: &Gates,
) -> Result<(usize, usize, usize), Box<dyn Error>> {
    let report = read_json_report(path)?;
    let real_unconnected = report
        .unconnected_items
        .iter()
        .filter(|entry| !is_ignored_self_zone_unconnected(entry, gates))
        .count();
    let ignored_self_zone = report.unconnected_items.len() - real_unconnected;
    let violations = report.violations.len();

    if violations > gates.max_physical_drc_violations {
        return Err(format!(
            "DRC has {violations} physical violations; allowed {}",
            gates.max_physical_drc_violations
        )
        .into());
    }
    if real_unconnected > gates.max_real_unconnected_items {
        return Err(format!(
            "DRC has {real_unconnected} real unconnected items; allowed {}",
            gates.max_real_unconnected_items
        )
        .into());
    }
    Ok((violations, real_unconnected, ignored_self_zone))
}

fn validate_erc_report(path: &Path, gates: &Gates) -> Result<usize, Box<dyn Error>> {
    let report = read_json_report(path)?;
    let violations = report.violations.len();
    if violations > gates.max_erc_violations {
        return Err(format!(
            "ERC has {violations} violations; allowed {}",
            gates.max_erc_violations
        )
        .into());
    }
    Ok(violations)
}

fn read_json_report(path: &Path) -> Result<KiCadReport, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

fn is_ignored_self_zone_unconnected(entry: &ReportEntry, gates: &Gates) -> bool {
    if !gates.allow_kicad_self_zone_unconnected || entry.items.len() != 2 {
        return false;
    }
    let first = &entry.items[0];
    let second = &entry.items[1];
    first.description.starts_with("Zone [")
        && second.description.starts_with("Zone [")
        && first.description == second.description
        && first.uuid.is_some()
        && first.uuid == second.uuid
}

fn validate_assembly_sources(
    parts: &PartsManifest,
    placement: &PlacementPlan,
) -> Result<(), Box<dyn Error>> {
    let part_ids = parts
        .selected_parts
        .iter()
        .map(|part| part.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut by_part: BTreeMap<&str, u32> = BTreeMap::new();
    let mut refs = BTreeSet::new();
    let mut errors = Vec::new();

    for item in &placement.placements {
        if !refs.insert(item.reference.as_str()) {
            errors.push(format!("duplicate placement reference {}", item.reference));
        }
        if !part_ids.contains(item.part_id.as_str()) {
            errors.push(format!(
                "placement {} references unknown part group {}",
                item.reference, item.part_id
            ));
        }
        if item.side != "top" {
            errors.push(format!(
                "placement {} is {}, but Rev A release supports top-side PCBA only",
                item.reference, item.side
            ));
        }
        *by_part.entry(item.part_id.as_str()).or_default() += 1;
    }

    for part in &parts.selected_parts {
        let count = by_part.get(part.id.as_str()).copied().unwrap_or_default();
        if count != part.quantity {
            errors.push(format!(
                "part group {} expects {} placements but found {}",
                part.id, part.quantity, count
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn validate_bringup_sources(
    bringup: &BringupConfig,
    placement: &PlacementPlan,
) -> Result<(), Box<dyn Error>> {
    let mut errors = Vec::new();
    let mut names = BTreeSet::new();
    for point in &placement.test_points {
        if !names.insert(point.name.as_str()) {
            errors.push(format!("duplicate bring-up test point {}", point.name));
        }
        if point.net.trim().is_empty() {
            errors.push(format!(
                "bring-up test point {} has an empty net",
                point.name
            ));
        }
        if point.side != "top" {
            errors.push(format!(
                "bring-up test point {} is {}, but Rev A inspection expects top-side access",
                point.name, point.side
            ));
        }
    }

    for required in &bringup.required_test_points {
        if !names.contains(required.as_str()) {
            errors.push(format!(
                "bring-up checklist requires missing test point {required}"
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn validate_firmware_handoff_sources(
    contract: &Contract,
    pin_nets: &PinNetManifest,
    firmware: &FirmwareHandoff,
) -> Result<(), Box<dyn Error>> {
    let mut errors = Vec::new();

    if firmware.package.revision != "Rev A" {
        errors.push(format!(
            "firmware handoff revision must be Rev A, found {}",
            firmware.package.revision
        ));
    }
    if firmware.mcu.module != contract.board.primary_mcu {
        errors.push(format!(
            "firmware handoff MCU {} does not match contract MCU {}",
            firmware.mcu.module, contract.board.primary_mcu
        ));
    }
    if firmware.slots.len() != contract.board.slot_count {
        errors.push(format!(
            "firmware handoff has {} slots, contract requires {}",
            firmware.slots.len(),
            contract.board.slot_count
        ));
    }

    let expanded_nets = expand_contract_nets(contract);
    let Some(mcu_pins) = pin_nets
        .assignments
        .iter()
        .find(|assignment| assignment.reference == firmware.mcu.reference)
    else {
        errors.push(format!(
            "pin_nets is missing MCU reference {}",
            firmware.mcu.reference
        ));
        return Err(errors.join("\n").into());
    };

    let mut module_pins = BTreeSet::new();
    let mut soc_gpios = BTreeSet::new();
    let mut nets_by_module_pin = BTreeMap::new();
    let mut module_pin_by_net = BTreeMap::new();
    for pin in &firmware.module_pins {
        if !module_pins.insert(pin.module_pin) {
            errors.push(format!(
                "firmware handoff duplicates module pin {}",
                pin.module_pin
            ));
        }
        if pin.soc_gpio >= 0 && !soc_gpios.insert(pin.soc_gpio) {
            errors.push(format!(
                "firmware handoff duplicates SoC GPIO {}",
                pin.soc_gpio
            ));
        }
        if !expanded_nets.contains(pin.net.as_str()) {
            errors.push(format!(
                "firmware handoff net {} is missing from contract",
                pin.net
            ));
        }
        if pin.role.trim().is_empty() || pin.firmware_direction.trim().is_empty() {
            errors.push(format!(
                "firmware handoff module pin {} needs role and firmware_direction",
                pin.module_pin
            ));
        }

        let module_pin = pin.module_pin.to_string();
        match mcu_pins.pins.get(&module_pin) {
            Some(net) if net == &pin.net => {}
            Some(net) => errors.push(format!(
                "firmware handoff module pin {} net {} does not match pin_nets {}",
                pin.module_pin, pin.net, net
            )),
            None => errors.push(format!(
                "firmware handoff module pin {} is missing from pin_nets for {}",
                pin.module_pin, firmware.mcu.reference
            )),
        }
        nets_by_module_pin.insert(pin.module_pin, pin.net.as_str());
        module_pin_by_net.insert(pin.net.as_str(), pin.module_pin);
    }

    for gpio in &contract.gpio_map {
        match nets_by_module_pin.get(&gpio.esp32_module_pin) {
            Some(net) if *net == gpio.net => {}
            Some(net) => errors.push(format!(
                "contract GPIO pin {} maps to {}, but firmware handoff maps it to {}",
                gpio.esp32_module_pin, gpio.net, net
            )),
            None => errors.push(format!(
                "firmware handoff missing contract GPIO pin {} for {}",
                gpio.esp32_module_pin, gpio.net
            )),
        }
        if !gpio.locked {
            errors.push(format!(
                "contract GPIO pin {} for {} is not locked",
                gpio.esp32_module_pin, gpio.net
            ));
        }
    }

    for net in firmware_required_nets(firmware) {
        if !expanded_nets.contains(net.as_str()) {
            errors.push(format!(
                "firmware handoff required net {} is missing from contract",
                net
            ));
        }
    }

    let mut slot_numbers = BTreeSet::new();
    for slot in &firmware.slots {
        if !slot_numbers.insert(slot.slot) {
            errors.push(format!("firmware handoff duplicates slot {}", slot.slot));
        }
        if slot.slot >= contract.board.slot_count {
            errors.push(format!(
                "firmware handoff slot {} is outside 0..{}",
                slot.slot,
                contract.board.slot_count - 1
            ));
        }
        if slot.select_bits.len() != firmware.peripherals.mux_select_nets.len() {
            errors.push(format!(
                "firmware handoff slot {} has {} select bits, expected {}",
                slot.slot,
                slot.select_bits.len(),
                firmware.peripherals.mux_select_nets.len()
            ));
        }
        if slot.select_bits.iter().any(|bit| *bit > 1) {
            errors.push(format!(
                "firmware handoff slot {} select bits must be binary",
                slot.slot
            ));
        }
        for net in [&slot.led_net, &slot.mux_channel_net] {
            if !expanded_nets.contains(net.as_str()) {
                errors.push(format!(
                    "firmware handoff slot {} net {} is missing from contract",
                    slot.slot, net
                ));
            }
        }
        if !module_pin_by_net.contains_key(slot.led_net.as_str()) {
            errors.push(format!(
                "firmware handoff slot {} LED net {} has no MCU module pin mapping",
                slot.slot, slot.led_net
            ));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn expand_contract_nets(contract: &Contract) -> BTreeSet<String> {
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

fn firmware_required_nets(firmware: &FirmwareHandoff) -> Vec<String> {
    let mut nets = vec![
        firmware.peripherals.i2c_sda_net.clone(),
        firmware.peripherals.i2c_scl_net.clone(),
        firmware.peripherals.adc_input_net.clone(),
        firmware.peripherals.mux_common_net.clone(),
        firmware.peripherals.heater_pwm_net.clone(),
        firmware.peripherals.usb_dn_net.clone(),
        firmware.peripherals.usb_dp_net.clone(),
        firmware.peripherals.uart_rx_net.clone(),
        firmware.peripherals.uart_tx_net.clone(),
        firmware.peripherals.boot_net.clone(),
        firmware.peripherals.enable_net.clone(),
        firmware.peripherals.activity_net.clone(),
    ];
    nets.extend(firmware.peripherals.mux_select_nets.iter().cloned());
    nets
}

fn write_bom(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    assembly: &AssemblyConfig,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["Comment", "Designator", "Footprint", "LCSC Part #"])?;
    for part in &parts.selected_parts {
        if is_manual_part(assembly, part) {
            continue;
        }
        let mut placements = by_part
            .remove(part.id.as_str())
            .ok_or_else(|| format!("missing placement group {}", part.id))?;
        placements.sort_by_key(|item| reference_order(&item.reference));
        let designators = placements
            .iter()
            .map(|item| item.reference.as_str())
            .collect::<Vec<_>>()
            .join(",");
        writer.write_record([
            part.value.as_str(),
            designators.as_str(),
            part.footprint.as_str(),
            part.lcsc_part.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_cpl(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    assembly: &AssemblyConfig,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let machine_part_ids = machine_part_ids(parts, assembly);
    let mut placements = placement
        .placements
        .iter()
        .filter(|item| machine_part_ids.contains(item.part_id.as_str()))
        .collect::<Vec<_>>();
    placements.sort_by_key(|item| reference_order(&item.reference));

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["Designator", "Mid X", "Mid Y", "Layer", "Rotation"])?;
    for item in placements {
        writer.write_record([
            item.reference.as_str(),
            format!("{:.3}", item.x_mm).as_str(),
            format!("{:.3}", item.y_mm).as_str(),
            "TopLayer",
            format!("{:.3}", item.rotation_deg).as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

fn write_manual_parts(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    assembly: &AssemblyConfig,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let manual_part_ids = manual_part_ids(assembly);
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record([
        "Designator",
        "Value",
        "Footprint",
        "LCSC Part #",
        "Install Note",
    ])?;
    for part in &parts.selected_parts {
        if !manual_part_ids.contains(part.id.as_str()) {
            continue;
        }
        let mut placements = by_part
            .remove(part.id.as_str())
            .ok_or_else(|| format!("missing manual placement group {}", part.id))?;
        placements.sort_by_key(|item| reference_order(&item.reference));
        for item in placements {
            writer.write_record([
                item.reference.as_str(),
                part.value.as_str(),
                part.footprint.as_str(),
                part.lcsc_part.as_str(),
                "Manual install after SMT assembly",
            ])?;
        }
    }
    writer.flush()?;
    Ok(())
}

fn is_manual_part(assembly: &AssemblyConfig, part: &SelectedPart) -> bool {
    assembly.manual_part_ids.iter().any(|id| id == &part.id)
}

fn manual_part_ids(assembly: &AssemblyConfig) -> BTreeSet<&str> {
    assembly
        .manual_part_ids
        .iter()
        .map(String::as_str)
        .collect()
}

fn machine_part_ids<'a>(parts: &'a PartsManifest, assembly: &AssemblyConfig) -> BTreeSet<&'a str> {
    let manual_part_ids = manual_part_ids(assembly);
    parts
        .selected_parts
        .iter()
        .filter(|part| !manual_part_ids.contains(part.id.as_str()))
        .map(|part| part.id.as_str())
        .collect()
}

fn run_gerber_export(
    config: &ReleaseConfig,
    board: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut args = vec![
        "pcb".to_string(),
        "export".to_string(),
        "gerbers".to_string(),
        "--output".to_string(),
        output_root
            .join(&config.outputs.gerbers_dir)
            .display()
            .to_string(),
        "--layers".to_string(),
        config.gerbers.layers.join(","),
        "--precision".to_string(),
        config.gerbers.precision.to_string(),
    ];
    if config.gerbers.check_zones {
        args.push("--check-zones".to_string());
    }
    if config.gerbers.use_board_plot_params {
        args.push("--board-plot-params".to_string());
    }
    args.push(board.display().to_string());
    run_command("kicad-cli", &args)
}

fn run_drill_export(
    config: &ReleaseConfig,
    board: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut args = vec![
        "pcb".to_string(),
        "export".to_string(),
        "drill".to_string(),
        "--output".to_string(),
        output_root
            .join(&config.outputs.drills_dir)
            .display()
            .to_string(),
        "--format".to_string(),
        config.drills.format.clone(),
        "--drill-origin".to_string(),
        config.drills.origin.clone(),
        "--excellon-zeros-format".to_string(),
        config.drills.zeros_format.clone(),
        "--excellon-oval-format".to_string(),
        config.drills.oval_format.clone(),
        "--excellon-units".to_string(),
        config.drills.units.clone(),
    ];
    if config.drills.separate_th {
        args.push("--excellon-separate-th".to_string());
    }
    if config.drills.generate_map {
        args.push("--generate-map".to_string());
    }
    if config.drills.generate_report {
        args.push("--generate-report".to_string());
        args.push("--report-path".to_string());
        args.push(
            output_root
                .join(&config.outputs.drill_report)
                .display()
                .to_string(),
        );
    }
    args.push(board.display().to_string());
    run_command("kicad-cli", &args)
}

fn run_position_export(
    config: &ReleaseConfig,
    board: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut args = vec![
        "pcb".to_string(),
        "export".to_string(),
        "pos".to_string(),
        "--output".to_string(),
        output_root
            .join(&config.outputs.position_file)
            .display()
            .to_string(),
        "--side".to_string(),
        config.position.side.clone(),
        "--format".to_string(),
        config.position.format.clone(),
        "--units".to_string(),
        config.position.units.clone(),
    ];
    if config.position.smd_only {
        args.push("--smd-only".to_string());
    }
    if config.position.exclude_dnp {
        args.push("--exclude-dnp".to_string());
    }
    args.push(board.display().to_string());
    run_command("kicad-cli", &args)
}

fn run_step_export(
    config: &ReleaseConfig,
    board: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut args = vec![
        "pcb".to_string(),
        "export".to_string(),
        "step".to_string(),
        "--output".to_string(),
        output_root
            .join(&config.outputs.step_file)
            .display()
            .to_string(),
    ];
    if config.step.force {
        args.push("--force".to_string());
    }
    if config.step.board_only {
        args.push("--board-only".to_string());
    }
    args.push(board.display().to_string());
    run_command("kicad-cli", &args)
}

fn run_command(program: &str, args: &[String]) -> Result<(), Box<dyn Error>> {
    let output = Command::new(program).args(args).output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "command failed: {}\nstdout:\n{}\nstderr:\n{}",
            display_command(program, args),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

fn display_command(program: &str, args: &[String]) -> String {
    std::iter::once(program.to_string())
        .chain(args.iter().cloned())
        .collect::<Vec<_>>()
        .join(" ")
}

fn write_manifest(
    config: &ReleaseConfig,
    output_root: &Path,
    gate_results: ReleaseGateResults,
) -> Result<(), Box<dyn Error>> {
    let drc_violations = gate_results.drc_violations;
    let real_unconnected = gate_results.real_unconnected;
    let ignored_self_zone = gate_results.ignored_self_zone;
    let erc_violations = gate_results.erc_violations;
    let mut file = fs::File::create(output_root.join(&config.outputs.manifest_file))?;
    writeln!(file, "LaminarForge LAMP Rev A PCBA Fab Release")?;
    writeln!(file, "revision: {}", config.package.revision)?;
    writeln!(file, "drc_violations: {drc_violations}")?;
    writeln!(file, "real_unconnected_items: {real_unconnected}")?;
    writeln!(file, "ignored_kicad_self_zone_items: {ignored_self_zone}")?;
    writeln!(file, "erc_violations: {erc_violations}")?;
    writeln!(
        file,
        "gerber_files: {}",
        count_files(&output_root.join(&config.outputs.gerbers_dir))?
    )?;
    writeln!(
        file,
        "drill_files: {}",
        count_files(&output_root.join(&config.outputs.drills_dir))?
    )?;
    writeln!(file, "bom: {}", config.outputs.bom_file)?;
    writeln!(file, "cpl: {}", config.outputs.cpl_file)?;
    writeln!(file, "manual_install_parts: {}", config.outputs.manual_file)?;
    writeln!(file, "position: {}", config.outputs.position_file)?;
    if config.step.enabled {
        writeln!(file, "step: {}", config.outputs.step_file)?;
    }
    writeln!(file, "order_audit: {}", config.outputs.order_audit_file)?;
    writeln!(file, "bringup_checklist: {}", config.outputs.bringup_file)?;
    writeln!(
        file,
        "firmware_handoff: {}",
        config.outputs.firmware_handoff_file
    )?;
    writeln!(
        file,
        "electrical_validation: {}",
        config.outputs.electrical_validation_file
    )?;
    writeln!(
        file,
        "electrical_validation_gates: {}",
        config.outputs.electrical_validation_gates_file
    )?;
    writeln!(file, "spice_netlist: {}", config.outputs.spice_netlist_file)?;
    writeln!(
        file,
        "simulation_handoff: {}",
        config.outputs.simulation_handoff_file
    )?;
    writeln!(
        file,
        "simulation_inputs: {}",
        config.outputs.simulation_inputs_file
    )?;
    writeln!(
        file,
        "pdn_current_paths: {}",
        config.outputs.pdn_current_paths_file
    )?;
    writeln!(
        file,
        "pdn_dc_simulation: {}",
        config.outputs.pdn_dc_simulation_file
    )?;
    writeln!(file, "thermal_power: {}", config.outputs.thermal_power_file)?;
    writeln!(
        file,
        "first_article_measurements: {}",
        config.outputs.first_article_measurements_file
    )?;
    writeln!(
        file,
        "component_derating: {}",
        config.outputs.component_derating_file
    )?;
    writeln!(file, "fault_fmea: {}", config.outputs.fault_fmea_file)?;
    writeln!(file, "emc_esd: {}", config.outputs.emc_esd_file)?;
    writeln!(
        file,
        "startup_safety: {}",
        config.outputs.startup_safety_file
    )?;
    writeln!(
        file,
        "manufacturing_test: {}",
        config.outputs.manufacturing_test_file
    )?;
    writeln!(
        file,
        "calibration_readiness: {}",
        config.outputs.calibration_readiness_file
    )?;
    writeln!(
        file,
        "validation_traceability: {}",
        config.outputs.validation_traceability_file
    )?;
    writeln!(
        file,
        "bundle_checksums: {}",
        config.outputs.bundle_checksums_file
    )?;
    writeln!(
        file,
        "fabrication_bundle: {}",
        config.outputs.fabrication_bundle
    )?;
    writeln!(file, "assembly_bundle: {}", config.outputs.assembly_bundle)?;
    writeln!(file, "review_bundle: {}", config.outputs.review_bundle)?;
    writeln!(file, "source_bundle: {}", config.outputs.source_bundle)?;
    Ok(())
}

fn write_order_audit_report(
    config: &ReleaseConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    output_root: &Path,
    gate_results: ReleaseGateResults,
) -> Result<(), Box<dyn Error>> {
    let drc_violations = gate_results.drc_violations;
    let real_unconnected = gate_results.real_unconnected;
    let ignored_self_zone = gate_results.ignored_self_zone;
    let erc_violations = gate_results.erc_violations;
    let machine_count = machine_placement_count(parts, placement, &config.assembly);
    let manual_count = manual_placement_count(placement, &config.assembly);
    let mut file = fs::File::create(output_root.join(&config.outputs.order_audit_file))?;

    writeln!(file, "# LaminarForge LAMP Rev A Fab Order Audit")?;
    writeln!(file)?;
    writeln!(file, "## Gate Results")?;
    writeln!(file)?;
    writeln!(file, "- DRC violations: `{drc_violations}`")?;
    writeln!(file, "- Real unconnected items: `{real_unconnected}`")?;
    writeln!(
        file,
        "- Ignored KiCad self-zone report items: `{ignored_self_zone}`"
    )?;
    writeln!(file, "- ERC violations: `{erc_violations}`")?;
    writeln!(file, "- Machine-assembly placements: `{machine_count}`")?;
    writeln!(file, "- Manual-install placements: `{manual_count}`")?;
    writeln!(file)?;

    writeln!(file, "## Upload Files")?;
    writeln!(file)?;
    writeln!(file, "### Gerbers")?;
    for path in list_relative_files(output_root, &config.outputs.gerbers_dir)? {
        writeln!(file, "- `{path}`")?;
    }
    writeln!(file)?;
    writeln!(file, "### Drill Files")?;
    for path in list_relative_files(output_root, &config.outputs.drills_dir)? {
        writeln!(file, "- `{path}`")?;
    }
    writeln!(file)?;
    writeln!(file, "### Assembly")?;
    for path in list_relative_files(output_root, &config.outputs.assembly_dir)? {
        writeln!(file, "- `{path}`")?;
    }
    writeln!(file)?;
    writeln!(file, "### Review")?;
    for path in list_relative_files(output_root, &config.outputs.review_dir)? {
        if path != config.outputs.order_audit_file {
            writeln!(file, "- `{path}`")?;
        }
    }
    writeln!(file)?;
    writeln!(file, "### Upload Bundles")?;
    writeln!(file, "- `{}`", config.outputs.fabrication_bundle)?;
    writeln!(file, "- `{}`", config.outputs.assembly_bundle)?;
    writeln!(file, "- `{}`", config.outputs.review_bundle)?;
    writeln!(file, "- `{}`", config.outputs.source_bundle)?;
    writeln!(file)?;

    writeln!(file, "## Manual-Install Parts")?;
    writeln!(file)?;
    for row in manual_install_rows(parts, placement, &config.assembly)? {
        writeln!(
            file,
            "- `{}`: {} / {} / {}",
            row.reference, row.value, row.footprint, row.lcsc_part
        )?;
    }
    writeln!(file)?;

    writeln!(file, "## Remaining Review Gates")?;
    writeln!(file)?;
    for gate in &config.review.remaining_gates {
        writeln!(file, "- {gate}")?;
    }

    Ok(())
}

fn write_bringup_checklist(
    config: &ReleaseConfig,
    placement: &PlacementPlan,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut points = placement.test_points.iter().collect::<Vec<_>>();
    points.sort_by_key(|point| reference_order(&point.name));

    let mut file = fs::File::create(output_root.join(&config.outputs.bringup_file))?;
    writeln!(file, "# LaminarForge LAMP Rev A Bring-Up Checklist")?;
    writeln!(file)?;
    writeln!(
        file,
        "Use this checklist after receiving the Rev A PCBA and before applying heater power."
    )?;
    writeln!(file)?;

    writeln!(file, "## Power-Off Inspection")?;
    writeln!(file)?;
    writeln!(
        file,
        "- Confirm J2 and J3 are installed manually after SMT assembly and match the enclosure/mechanical stack."
    )?;
    writeln!(
        file,
        "- Inspect USB-C, ESP32-S3 module, AMS1117 regulator, optical mux, ADC, and TIA footprints under magnification."
    )?;
    writeln!(
        file,
        "- Verify the external heater assembly includes the inline thermal cutoff before connecting J3."
    )?;
    writeln!(
        file,
        "- Check resistance from each power rail test point to `TP_GND` before applying power."
    )?;
    writeln!(file)?;

    writeln!(file, "## Staged Power-Up")?;
    writeln!(file)?;
    writeln!(
        file,
        "1. Apply USB power only with a current limit of `{}` mA; verify `TP_VBUS`, `TP_5V`, and `TP_3V3` before connecting 12 V.",
        config.bringup.usb_power_limit_ma
    )?;
    writeln!(
        file,
        "2. Connect 12 V heater supply with a current limit of `{}` mA; verify `TP_12V` and `TP_HEATER_SUPPLY` while firmware keeps `TP_PWM` inactive.",
        config.bringup.heater_power_limit_ma
    )?;
    writeln!(
        file,
        "3. Confirm boot/programming access through `TP_EN`, `TP_BOOT`, `TP_TX`, and `TP_RX` before loading heater-control firmware."
    )?;
    writeln!(
        file,
        "4. Run one dark optical read and one 650 nm emitter read per slot; watch `TP_ADC` and `TP_MUX` for saturation or unstable baseline."
    )?;
    writeln!(file)?;

    writeln!(file, "## Required Test Points")?;
    writeln!(file)?;
    writeln!(file, "| Test Point | Net | X mm | Y mm | Side |")?;
    writeln!(file, "| --- | --- | ---: | ---: | --- |")?;
    for point in points {
        writeln!(
            file,
            "| `{}` | `{}` | {:.3} | {:.3} | {} |",
            point.name, point.net, point.x_mm, point.y_mm, point.side
        )?;
    }
    writeln!(file)?;

    writeln!(file, "## Release Gate")?;
    writeln!(file)?;
    writeln!(
        file,
        "All `{}` configured required test points must be accessible before the board is accepted for firmware bring-up.",
        config.bringup.required_test_points.len()
    )?;
    Ok(())
}

fn write_firmware_handoff(
    config: &ReleaseConfig,
    contract: &Contract,
    placement: &PlacementPlan,
    firmware: &FirmwareHandoff,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut module_pins = firmware.module_pins.iter().collect::<Vec<_>>();
    module_pins.sort_by_key(|pin| pin.module_pin);

    let pin_by_net = module_pins
        .iter()
        .map(|pin| (pin.net.as_str(), *pin))
        .collect::<BTreeMap<_, _>>();
    let function_by_net = contract
        .gpio_map
        .iter()
        .map(|gpio| (gpio.net.as_str(), gpio.function.as_str()))
        .collect::<BTreeMap<_, _>>();

    let mut slots = firmware.slots.iter().collect::<Vec<_>>();
    slots.sort_by_key(|slot| slot.slot);

    let mut test_points = placement.test_points.iter().collect::<Vec<_>>();
    test_points.sort_by_key(|point| reference_order(&point.name));

    let mut file = fs::File::create(output_root.join(&config.outputs.firmware_handoff_file))?;
    writeln!(file, "# LaminarForge LAMP Rev A Firmware Handoff")?;
    writeln!(file)?;
    writeln!(
        file,
        "Generated from `contract.toml`, `pin_nets.toml`, and `firmware_handoff.toml` during the fab release gate."
    )?;
    writeln!(file)?;
    writeln!(file, "## Source")?;
    writeln!(file)?;
    writeln!(file, "- Board revision: `{}`", firmware.package.revision)?;
    writeln!(
        file,
        "- MCU: `{}` / `{}`",
        firmware.mcu.reference, firmware.mcu.module
    )?;
    writeln!(
        file,
        "- Module pin unit: `{}`",
        firmware.mcu.module_pin_unit
    )?;
    writeln!(
        file,
        "- Firmware pin unit: `{}`",
        firmware.mcu.firmware_pin_unit
    )?;
    writeln!(
        file,
        "- Datasheet: {} ({}, {})",
        firmware.datasheet.source, firmware.datasheet.pin_table, firmware.datasheet.url
    )?;
    writeln!(file)?;

    writeln!(file, "## Gate Notes")?;
    writeln!(file)?;
    for note in &firmware.peripherals.notes {
        writeln!(file, "- {note}")?;
    }
    writeln!(file)?;

    writeln!(file, "## MCU Pin Map")?;
    writeln!(file)?;
    writeln!(
        file,
        "| Module Pin | SoC GPIO | Net | Role | Contract Function | Direction | Boot Sensitive |"
    )?;
    writeln!(file, "| ---: | ---: | --- | --- | --- | --- | --- |")?;
    for pin in module_pins {
        writeln!(
            file,
            "| {} | {} | `{}` | {} | {} | `{}` | {} |",
            pin.module_pin,
            firmware_gpio_label(pin),
            pin.net,
            pin.role,
            function_by_net
                .get(pin.net.as_str())
                .copied()
                .unwrap_or("n/a"),
            pin.firmware_direction,
            if pin.boot_sensitive { "yes" } else { "no" }
        )?;
    }
    writeln!(file)?;

    writeln!(file, "## Firmware Constants")?;
    writeln!(file)?;
    writeln!(file, "```rust")?;
    writeln!(file, "pub const LAMP_REVISION: &str = \"Rev A\";")?;
    writeln!(
        file,
        "pub const LAMP_SLOT_COUNT: usize = {};",
        contract.board.slot_count
    )?;
    writeln!(
        file,
        "pub const ADS1115_I2C_ADDRESS: u8 = {};",
        firmware.peripherals.adc_i2c_address
    )?;
    for (constant, net) in [
        ("I2C_SDA_GPIO", firmware.peripherals.i2c_sda_net.as_str()),
        ("I2C_SCL_GPIO", firmware.peripherals.i2c_scl_net.as_str()),
        (
            "HEATER_PWM_GPIO",
            firmware.peripherals.heater_pwm_net.as_str(),
        ),
        (
            "MUX_S0_GPIO",
            firmware.peripherals.mux_select_nets[0].as_str(),
        ),
        (
            "MUX_S1_GPIO",
            firmware.peripherals.mux_select_nets[1].as_str(),
        ),
        (
            "MUX_S2_GPIO",
            firmware.peripherals.mux_select_nets[2].as_str(),
        ),
        ("UART_RX_GPIO", firmware.peripherals.uart_rx_net.as_str()),
        ("UART_TX_GPIO", firmware.peripherals.uart_tx_net.as_str()),
        (
            "ACTIVITY_LED_GPIO",
            firmware.peripherals.activity_net.as_str(),
        ),
    ] {
        let pin = pin_by_net
            .get(net)
            .ok_or_else(|| format!("firmware constant net {net} has no pin mapping"))?;
        writeln!(file, "pub const {constant}: i32 = {};", pin.soc_gpio)?;
    }
    writeln!(file, "```")?;
    writeln!(file)?;

    writeln!(file, "## Slot Map")?;
    writeln!(file)?;
    writeln!(
        file,
        "| Slot | LED Net | LED GPIO | Mux Channel | Select Bits S0/S1/S2 |"
    )?;
    writeln!(file, "| ---: | --- | ---: | --- | --- |")?;
    for slot in slots {
        let led_pin = pin_by_net.get(slot.led_net.as_str()).ok_or_else(|| {
            format!(
                "firmware slot {} LED net {} has no pin mapping",
                slot.slot, slot.led_net
            )
        })?;
        writeln!(
            file,
            "| {} | `{}` | {} | `{}` | `{}/{}/{}` |",
            slot.slot,
            slot.led_net,
            firmware_gpio_label(led_pin),
            slot.mux_channel_net,
            slot.select_bits[0],
            slot.select_bits[1],
            slot.select_bits[2]
        )?;
    }
    writeln!(file)?;

    writeln!(file, "## Peripheral Nets")?;
    writeln!(file)?;
    writeln!(
        file,
        "- I2C: `{}`/`{}` to `{}` at `{}`.",
        firmware.peripherals.i2c_sda_net,
        firmware.peripherals.i2c_scl_net,
        firmware.peripherals.adc_device,
        firmware.peripherals.adc_i2c_address
    )?;
    writeln!(
        file,
        "- ADC input: `{}`; mux common: `{}`.",
        firmware.peripherals.adc_input_net, firmware.peripherals.mux_common_net
    )?;
    writeln!(
        file,
        "- Native USB: `{}` and `{}`.",
        firmware.peripherals.usb_dn_net, firmware.peripherals.usb_dp_net
    )?;
    writeln!(
        file,
        "- Boot/debug: `{}`, `{}`, `{}`, `{}`.",
        firmware.peripherals.enable_net,
        firmware.peripherals.boot_net,
        firmware.peripherals.uart_rx_net,
        firmware.peripherals.uart_tx_net
    )?;
    writeln!(file)?;

    writeln!(file, "## Test Points")?;
    writeln!(file)?;
    writeln!(file, "| Test Point | Net | X mm | Y mm | Side |")?;
    writeln!(file, "| --- | --- | ---: | ---: | --- |")?;
    for point in test_points {
        writeln!(
            file,
            "| `{}` | `{}` | {:.3} | {:.3} | {} |",
            point.name, point.net, point.x_mm, point.y_mm, point.side
        )?;
    }
    Ok(())
}

fn firmware_gpio_label(pin: &FirmwareModulePin) -> String {
    if pin.soc_gpio >= 0 {
        pin.soc_gpio.to_string()
    } else {
        "n/a".to_string()
    }
}

fn write_release_bundles(
    config: &ReleaseConfig,
    repo_root: &Path,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut fabrication_entries = zip_entries_from_dirs(
        output_root,
        &[&config.outputs.gerbers_dir, &config.outputs.drills_dir],
    )?;
    fabrication_entries.sort_by(|left, right| left.archive_name.cmp(&right.archive_name));
    write_zip_bundle(
        output_root,
        &output_root.join(&config.outputs.fabrication_bundle),
        &fabrication_entries,
    )?;

    let assembly_entries = zip_entries_from_files(&[
        &config.outputs.bom_file,
        &config.outputs.cpl_file,
        &config.outputs.manual_file,
        &config.outputs.position_file,
    ])?;
    write_zip_bundle(
        output_root,
        &output_root.join(&config.outputs.assembly_bundle),
        &assembly_entries,
    )?;

    let source_entries = zip_entries_from_source_files(&config.source_snapshot.files)?;
    write_zip_bundle(
        repo_root,
        &output_root.join(&config.outputs.source_bundle),
        &source_entries,
    )?;

    write_bundle_checksums(config, output_root)?;

    let mut review_files = vec![
        config.outputs.manifest_file.as_str(),
        config.outputs.order_audit_file.as_str(),
        config.outputs.bringup_file.as_str(),
        config.outputs.firmware_handoff_file.as_str(),
        config.outputs.electrical_validation_file.as_str(),
        config.outputs.electrical_validation_gates_file.as_str(),
        config.outputs.spice_netlist_file.as_str(),
        config.outputs.simulation_handoff_file.as_str(),
        config.outputs.simulation_inputs_file.as_str(),
        config.outputs.pdn_current_paths_file.as_str(),
        config.outputs.thermal_power_file.as_str(),
        config.outputs.first_article_measurements_file.as_str(),
        config.outputs.component_derating_file.as_str(),
        config.outputs.fault_fmea_file.as_str(),
        config.outputs.emc_esd_file.as_str(),
        config.outputs.startup_safety_file.as_str(),
        config.outputs.manufacturing_test_file.as_str(),
        config.outputs.calibration_readiness_file.as_str(),
        config.outputs.validation_traceability_file.as_str(),
        config.outputs.pdn_dc_simulation_file.as_str(),
        config.outputs.bundle_checksums_file.as_str(),
        config.outputs.drc_report.as_str(),
        config.outputs.erc_report.as_str(),
        config.outputs.drill_report.as_str(),
    ];
    if config.step.enabled {
        review_files.push(config.outputs.step_file.as_str());
    }
    let review_entries = zip_entries_from_files(&review_files)?;
    write_zip_bundle(
        output_root,
        &output_root.join(&config.outputs.review_bundle),
        &review_entries,
    )?;
    Ok(())
}

fn write_zip_bundle(
    source_root: &Path,
    bundle_path: &Path,
    entries: &[ZipEntry],
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = bundle_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = fs::File::create(bundle_path)?;
    let mut zip = ZipWriter::new(file);
    let modified = zip::DateTime::from_date_and_time(2026, 1, 1, 0, 0, 0)?;
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(modified)
        .unix_permissions(0o644);

    let mut archive_names = BTreeSet::new();
    for entry in entries {
        if !archive_names.insert(entry.archive_name.as_str()) {
            return Err(format!(
                "duplicate ZIP entry {} in {}",
                entry.archive_name,
                bundle_path.display()
            )
            .into());
        }
        let content = fs::read(source_root.join(&entry.source_relative))?;
        if content.is_empty() {
            return Err(format!("refusing to bundle empty file {}", entry.source_relative).into());
        }
        zip.start_file(entry.archive_name.as_str(), options)?;
        zip.write_all(&content)?;
    }
    zip.finish()?;
    Ok(())
}

fn write_bundle_checksums(
    config: &ReleaseConfig,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let checksum_paths = bundle_checksum_paths(config);
    let mut file = fs::File::create(output_root.join(&config.outputs.bundle_checksums_file))?;
    for relative in checksum_paths {
        let digest = sha256_file(&output_root.join(&relative))?;
        writeln!(file, "{digest}  {relative}")?;
    }
    Ok(())
}

fn bundle_checksum_paths(config: &ReleaseConfig) -> Vec<String> {
    vec![
        config.outputs.fabrication_bundle.clone(),
        config.outputs.assembly_bundle.clone(),
        config.outputs.source_bundle.clone(),
    ]
}

fn sha256_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let content = fs::read(path)?;
    if content.is_empty() {
        return Err(format!("cannot checksum empty file {}", path.display()).into());
    }
    let digest = Sha256::digest(&content);
    Ok(digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>())
}

fn zip_entries_from_dirs(
    output_root: &Path,
    relative_dirs: &[&str],
) -> Result<Vec<ZipEntry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for relative_dir in relative_dirs {
        for source_relative in list_relative_files(output_root, relative_dir)? {
            entries.push(ZipEntry {
                archive_name: archive_name_for(&source_relative)?,
                source_relative,
            });
        }
    }
    Ok(entries)
}

fn zip_entries_from_files(relative_files: &[&str]) -> Result<Vec<ZipEntry>, Box<dyn Error>> {
    relative_files
        .iter()
        .map(|relative| {
            Ok(ZipEntry {
                source_relative: (*relative).to_string(),
                archive_name: archive_name_for(relative)?,
            })
        })
        .collect()
}

fn zip_entries_from_source_files(
    relative_files: &[String],
) -> Result<Vec<ZipEntry>, Box<dyn Error>> {
    relative_files
        .iter()
        .map(|relative| {
            Ok(ZipEntry {
                source_relative: relative.clone(),
                archive_name: normalize_archive_path(relative)?,
            })
        })
        .collect()
}

fn archive_name_for(relative: &str) -> Result<String, Box<dyn Error>> {
    let filename = Path::new(relative)
        .file_name()
        .ok_or_else(|| format!("release path has no filename: {relative}"))?
        .to_string_lossy()
        .to_string();
    Ok(filename)
}

fn normalize_archive_path(relative: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(relative);
    if path.is_absolute() || relative.contains("..") || relative.trim().is_empty() {
        return Err(format!("invalid source snapshot path: {relative}").into());
    }
    Ok(relative.replace('\\', "/"))
}

fn validate_release_outputs(
    config: &ReleaseConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    output_root: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut errors = Vec::new();

    for layer in &config.gerbers.layers {
        let filename = expected_gerber_filename(layer).ok_or_else(|| {
            format!("fab release does not know expected Gerber filename for layer {layer}")
        })?;
        validate_nonempty_file(
            &output_root.join(&config.outputs.gerbers_dir).join(filename),
            &mut errors,
        );
    }
    validate_nonempty_file(
        &output_root
            .join(&config.outputs.gerbers_dir)
            .join("lamp_rev_a-job.gbrjob"),
        &mut errors,
    );

    for filename in [
        "lamp_rev_a-PTH.drl",
        "lamp_rev_a-NPTH.drl",
        "lamp_rev_a-PTH-drl_map.pdf",
        "lamp_rev_a-NPTH-drl_map.pdf",
    ] {
        validate_nonempty_file(
            &output_root.join(&config.outputs.drills_dir).join(filename),
            &mut errors,
        );
    }

    for relative in [
        &config.outputs.bom_file,
        &config.outputs.cpl_file,
        &config.outputs.manual_file,
        &config.outputs.position_file,
        &config.outputs.order_audit_file,
        &config.outputs.bringup_file,
        &config.outputs.firmware_handoff_file,
        &config.outputs.electrical_validation_file,
        &config.outputs.electrical_validation_gates_file,
        &config.outputs.spice_netlist_file,
        &config.outputs.simulation_handoff_file,
        &config.outputs.simulation_inputs_file,
        &config.outputs.pdn_current_paths_file,
        &config.outputs.thermal_power_file,
        &config.outputs.first_article_measurements_file,
        &config.outputs.component_derating_file,
        &config.outputs.fault_fmea_file,
        &config.outputs.emc_esd_file,
        &config.outputs.startup_safety_file,
        &config.outputs.manufacturing_test_file,
        &config.outputs.calibration_readiness_file,
        &config.outputs.validation_traceability_file,
        &config.outputs.pdn_dc_simulation_file,
        &config.outputs.bundle_checksums_file,
        &config.outputs.manifest_file,
        &config.outputs.drill_report,
        &config.outputs.erc_report,
        &config.outputs.drc_report,
    ] {
        validate_nonempty_file(&output_root.join(relative), &mut errors);
    }
    if config.step.enabled {
        validate_nonempty_file(&output_root.join(&config.outputs.step_file), &mut errors);
    }

    let machine_count = machine_placement_count(parts, placement, &config.assembly);
    let manual_count = manual_placement_count(placement, &config.assembly);
    validate_csv_data_rows(
        &output_root.join(&config.outputs.cpl_file),
        machine_count,
        "JLCPCB CPL",
        &mut errors,
    );
    validate_csv_data_rows(
        &output_root.join(&config.outputs.manual_file),
        manual_count,
        "manual install parts",
        &mut errors,
    );
    if config.assembly.require_cpl_matches_kicad_position {
        validate_csv_data_rows(
            &output_root.join(&config.outputs.position_file),
            machine_count,
            "KiCad position export",
            &mut errors,
        );
    }
    validate_release_bundles(config, output_root, &mut errors);
    validate_bundle_checksums(config, output_root, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn validate_release_bundles(config: &ReleaseConfig, output_root: &Path, errors: &mut Vec<String>) {
    let expected_fabrication = match zip_entries_from_dirs(
        output_root,
        &[&config.outputs.gerbers_dir, &config.outputs.drills_dir],
    ) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(format!("failed to list fabrication bundle inputs: {err}"));
            Vec::new()
        }
    };
    validate_zip_entries(
        &output_root.join(&config.outputs.fabrication_bundle),
        &expected_fabrication,
        "fabrication bundle",
        errors,
    );

    let expected_assembly = match zip_entries_from_files(&[
        &config.outputs.bom_file,
        &config.outputs.cpl_file,
        &config.outputs.manual_file,
        &config.outputs.position_file,
    ]) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(format!("failed to list assembly bundle inputs: {err}"));
            Vec::new()
        }
    };
    validate_zip_entries(
        &output_root.join(&config.outputs.assembly_bundle),
        &expected_assembly,
        "assembly bundle",
        errors,
    );

    let mut review_files = vec![
        config.outputs.manifest_file.as_str(),
        config.outputs.order_audit_file.as_str(),
        config.outputs.bringup_file.as_str(),
        config.outputs.firmware_handoff_file.as_str(),
        config.outputs.electrical_validation_file.as_str(),
        config.outputs.electrical_validation_gates_file.as_str(),
        config.outputs.spice_netlist_file.as_str(),
        config.outputs.simulation_handoff_file.as_str(),
        config.outputs.simulation_inputs_file.as_str(),
        config.outputs.pdn_current_paths_file.as_str(),
        config.outputs.thermal_power_file.as_str(),
        config.outputs.first_article_measurements_file.as_str(),
        config.outputs.component_derating_file.as_str(),
        config.outputs.fault_fmea_file.as_str(),
        config.outputs.emc_esd_file.as_str(),
        config.outputs.startup_safety_file.as_str(),
        config.outputs.manufacturing_test_file.as_str(),
        config.outputs.calibration_readiness_file.as_str(),
        config.outputs.validation_traceability_file.as_str(),
        config.outputs.pdn_dc_simulation_file.as_str(),
        config.outputs.bundle_checksums_file.as_str(),
        config.outputs.drc_report.as_str(),
        config.outputs.erc_report.as_str(),
        config.outputs.drill_report.as_str(),
    ];
    if config.step.enabled {
        review_files.push(config.outputs.step_file.as_str());
    }
    let expected_review = match zip_entries_from_files(&review_files) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(format!("failed to list review bundle inputs: {err}"));
            Vec::new()
        }
    };
    validate_zip_entries(
        &output_root.join(&config.outputs.review_bundle),
        &expected_review,
        "review bundle",
        errors,
    );

    let expected_source = match zip_entries_from_source_files(&config.source_snapshot.files) {
        Ok(entries) => entries,
        Err(err) => {
            errors.push(format!(
                "failed to list source snapshot bundle inputs: {err}"
            ));
            Vec::new()
        }
    };
    validate_zip_entries(
        &output_root.join(&config.outputs.source_bundle),
        &expected_source,
        "source snapshot bundle",
        errors,
    );
}

fn validate_bundle_checksums(config: &ReleaseConfig, output_root: &Path, errors: &mut Vec<String>) {
    let checksum_path = output_root.join(&config.outputs.bundle_checksums_file);
    validate_nonempty_file(&checksum_path, errors);

    let content = match fs::read_to_string(&checksum_path) {
        Ok(content) => content,
        Err(err) => {
            errors.push(format!(
                "failed to read bundle checksum manifest {}: {err}",
                checksum_path.display()
            ));
            return;
        }
    };

    let mut actual = BTreeMap::new();
    for (line_number, line) in content.lines().enumerate() {
        let Some((digest, relative)) = line.split_once("  ") else {
            errors.push(format!(
                "bundle checksum manifest line {} is not '<sha256>  <path>': {}",
                line_number + 1,
                line
            ));
            continue;
        };
        actual.insert(relative.to_string(), digest.to_string());
    }

    for relative in bundle_checksum_paths(config) {
        let expected_digest = match sha256_file(&output_root.join(&relative)) {
            Ok(digest) => digest,
            Err(err) => {
                errors.push(format!("failed to compute checksum for {relative}: {err}"));
                continue;
            }
        };
        match actual.get(&relative) {
            Some(actual_digest) if actual_digest == &expected_digest => {}
            Some(actual_digest) => errors.push(format!(
                "checksum mismatch for {relative}: manifest {actual_digest}, actual {expected_digest}"
            )),
            None => errors.push(format!("checksum manifest is missing {relative}")),
        }
    }

    let expected_paths = bundle_checksum_paths(config)
        .into_iter()
        .collect::<BTreeSet<_>>();
    for relative in actual.keys() {
        if !expected_paths.contains(relative) {
            errors.push(format!(
                "checksum manifest contains unexpected bundle {relative}"
            ));
        }
    }
}

fn validate_zip_entries(
    path: &Path,
    expected_entries: &[ZipEntry],
    label: &str,
    errors: &mut Vec<String>,
) {
    validate_nonempty_file(path, errors);

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            errors.push(format!("failed to open {label} {}: {err}", path.display()));
            return;
        }
    };
    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(err) => {
            errors.push(format!("failed to read {label} {}: {err}", path.display()));
            return;
        }
    };

    let mut expected_names = expected_entries
        .iter()
        .map(|entry| entry.archive_name.clone())
        .collect::<Vec<_>>();
    expected_names.sort();

    let mut actual_names = Vec::new();
    for index in 0..archive.len() {
        let file = match archive.by_index(index) {
            Ok(file) => file,
            Err(err) => {
                errors.push(format!("failed to read {label} entry {index}: {err}"));
                return;
            }
        };
        if file.size() == 0 {
            errors.push(format!("{label} contains empty entry {}", file.name()));
        }
        actual_names.push(file.name().to_string());
    }
    actual_names.sort();

    if actual_names != expected_names {
        errors.push(format!(
            "{label} entries differ for {}\nexpected: {:?}\nactual: {:?}",
            path.display(),
            expected_names,
            actual_names
        ));
    }
}

fn list_relative_files(
    output_root: &Path,
    relative_dir: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let dir = output_root.join(relative_dir);
    let mut paths = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.is_file() {
            let filename = path
                .file_name()
                .ok_or_else(|| format!("path has no filename: {}", path.display()))?
                .to_string_lossy();
            paths.push(format!("{relative_dir}/{filename}"));
        }
    }
    paths.sort();
    Ok(paths)
}

struct ManualInstallRow {
    reference: String,
    value: String,
    footprint: String,
    lcsc_part: String,
}

struct ZipEntry {
    source_relative: String,
    archive_name: String,
}

fn manual_install_rows(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    assembly: &AssemblyConfig,
) -> Result<Vec<ManualInstallRow>, Box<dyn Error>> {
    let manual_part_ids = manual_part_ids(assembly);
    let parts_by_id = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();

    for item in &placement.placements {
        if !manual_part_ids.contains(item.part_id.as_str()) {
            continue;
        }
        let part = parts_by_id.get(item.part_id.as_str()).ok_or_else(|| {
            format!(
                "manual placement {} references unknown part {}",
                item.reference, item.part_id
            )
        })?;
        rows.push(ManualInstallRow {
            reference: item.reference.clone(),
            value: part.value.clone(),
            footprint: part.footprint.clone(),
            lcsc_part: part.lcsc_part.clone(),
        });
    }

    rows.sort_by_key(|row| reference_order(&row.reference));
    Ok(rows)
}

fn expected_gerber_filename(layer: &str) -> Option<&'static str> {
    match layer {
        "F.Cu" => Some("lamp_rev_a-F_Cu.gtl"),
        "In1.Cu" => Some("lamp_rev_a-In1_Cu.g1"),
        "In2.Cu" => Some("lamp_rev_a-In2_Cu.g2"),
        "B.Cu" => Some("lamp_rev_a-B_Cu.gbl"),
        "F.SilkS" => Some("lamp_rev_a-F_Silkscreen.gto"),
        "B.SilkS" => Some("lamp_rev_a-B_Silkscreen.gbo"),
        "F.Mask" => Some("lamp_rev_a-F_Mask.gts"),
        "B.Mask" => Some("lamp_rev_a-B_Mask.gbs"),
        "F.Paste" => Some("lamp_rev_a-F_Paste.gtp"),
        "Edge.Cuts" => Some("lamp_rev_a-Edge_Cuts.gm1"),
        _ => None,
    }
}

fn validate_nonempty_file(path: &Path, errors: &mut Vec<String>) {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_file() && metadata.len() > 0 => {}
        Ok(metadata) if metadata.is_file() => {
            errors.push(format!("release artifact is empty: {}", path.display()));
        }
        Ok(_) => errors.push(format!(
            "release artifact is not a file: {}",
            path.display()
        )),
        Err(err) => errors.push(format!(
            "release artifact is missing: {} ({err})",
            path.display()
        )),
    }
}

fn validate_csv_data_rows(path: &Path, expected: usize, label: &str, errors: &mut Vec<String>) {
    match csv_data_rows(path) {
        Ok(actual) if actual == expected => {}
        Ok(actual) => errors.push(format!(
            "{label} has {actual} data rows, expected {expected}: {}",
            path.display()
        )),
        Err(err) => errors.push(format!("failed to read {label} {}: {err}", path.display())),
    }
}

fn csv_data_rows(path: &Path) -> Result<usize, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut count = 0usize;
    for record in reader.records() {
        record?;
        count += 1;
    }
    Ok(count)
}

fn machine_placement_count(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    assembly: &AssemblyConfig,
) -> usize {
    let machine_part_ids = machine_part_ids(parts, assembly);
    placement
        .placements
        .iter()
        .filter(|item| machine_part_ids.contains(item.part_id.as_str()))
        .count()
}

fn manual_placement_count(placement: &PlacementPlan, assembly: &AssemblyConfig) -> usize {
    let manual_part_ids = manual_part_ids(assembly);
    placement
        .placements
        .iter()
        .filter(|item| manual_part_ids.contains(item.part_id.as_str()))
        .count()
}

fn count_files(path: &Path) -> Result<usize, Box<dyn Error>> {
    let mut count = 0usize;
    for entry in fs::read_dir(path)? {
        if entry?.path().is_file() {
            count += 1;
        }
    }
    Ok(count)
}

fn reference_order(reference: &str) -> (String, u32, String) {
    let prefix = reference
        .chars()
        .take_while(|ch| ch.is_ascii_alphabetic())
        .collect::<String>();
    let digits = reference
        .chars()
        .skip_while(|ch| ch.is_ascii_alphabetic())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    let number = digits.parse::<u32>().unwrap_or_default();
    (prefix, number, reference.to_string())
}
