#![allow(dead_code)]

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const CONTRACT_PATH: &str = "pcb/lamp_rev_a/contract.toml";
const PARTS_PATH: &str = "pcb/lamp_rev_a/parts.toml";
const OPTICAL_MODE_PATH: &str = "pcb/lamp_rev_a/optical_mode.md";
const SCHEMATIC_PATH: &str = "pcb/lamp_rev_a/lamp_rev_a.kicad_sch";
const BOARD_PATH: &str = "pcb/lamp_rev_a/lamp_rev_a.kicad_pcb";
const README_PATH: &str = "pcb/lamp_rev_a/README.md";
const DRU_PATH: &str = "pcb/lamp_rev_a/lamp_rev_a.kicad_dru";
const KIBOT_PATH: &str = "pcb/lamp_rev_a/kibot.yaml";
const SYM_LIB_TABLE_PATH: &str = "pcb/lamp_rev_a/sym-lib-table";
const FP_LIB_TABLE_PATH: &str = "pcb/lamp_rev_a/fp-lib-table";
const SYMBOL_LIBRARY_PATH: &str = "pcb/lib/lcsc.kicad_sym";

#[derive(Debug, Deserialize)]
struct Contract {
    package: Package,
    board: Board,
    stackup: Stackup,
    zones: Vec<Zone>,
    modules: Vec<Module>,
    rails: Vec<Rail>,
    nets: Vec<Net>,
    #[serde(default)]
    net_groups: Vec<NetGroup>,
    gpio_map: Vec<GpioMap>,
    test_points: Vec<TestPoint>,
    verification: Verification,
    manufacturing: Manufacturing,
}

#[derive(Debug, Deserialize)]
struct PartsManifest {
    package: PartsPackage,
    schematic: SchematicSource,
    blocks: Vec<SchematicBlock>,
    selected_parts: Vec<SelectedPart>,
    #[serde(default)]
    selection_gaps: Vec<SelectionGap>,
}

#[derive(Debug, Deserialize)]
struct PartsPackage {
    name: String,
    ticket: String,
    revision: String,
    source_stage: String,
}

#[derive(Debug, Deserialize)]
struct SchematicSource {
    root_file: String,
    symbol_library: String,
    footprint_library: String,
}

#[derive(Debug, Deserialize)]
struct SchematicBlock {
    name: String,
    title: String,
    modules: Vec<String>,
    required_nets: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SelectedPart {
    id: String,
    module: String,
    quantity: u32,
    reference_prefix: String,
    value: String,
    symbol: String,
    footprint: String,
    lcsc_part: String,
    nets: Vec<String>,
    verification: String,
}

#[derive(Debug, Deserialize)]
struct SelectionGap {
    id: String,
    module: String,
    blocks_fabrication: bool,
    reason: String,
    resolve_with: String,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    ticket: String,
    revision: String,
    purpose: String,
    source_stage: String,
}

#[derive(Debug, Deserialize)]
struct Board {
    physical_board_count: u32,
    width_mm: f64,
    height_mm: f64,
    thickness_mm: f64,
    layer_count: u32,
    slot_count: u32,
    slot_spacing_mm: f64,
    primary_mcu: String,
    manufacturer_class: String,
}

#[derive(Debug, Deserialize)]
struct Stackup {
    copper_layers: Vec<String>,
    ground_plane_layer: String,
    power_plane_layer: String,
    outer_copper_oz: f64,
    inner_copper_oz: f64,
    min_clearance_mm: f64,
    min_signal_track_mm: f64,
    min_via_drill_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Zone {
    name: String,
    purpose: String,
    x_min_mm: f64,
    x_max_mm: f64,
    y_min_mm: f64,
    y_max_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Module {
    name: String,
    status: String,
    owner: String,
    deliverable: String,
}

#[derive(Debug, Deserialize)]
struct Rail {
    name: String,
    nominal_voltage_v: f64,
    max_current_ma: u32,
    source: String,
    must_have_testpoint: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct Net {
    name: String,
    class: String,
    min_track_width_mm: f64,
    must_have_testpoint: bool,
}

#[derive(Debug, Deserialize)]
struct NetGroup {
    prefix: String,
    count: u32,
    class: String,
    min_track_width_mm: f64,
    must_have_testpoint: bool,
}

#[derive(Debug, Deserialize)]
struct GpioMap {
    esp32_module_pin: u32,
    net: String,
    function: String,
    locked: bool,
}

#[derive(Debug, Deserialize)]
struct TestPoint {
    name: String,
    net: String,
    purpose: String,
}

#[derive(Debug, Deserialize)]
struct Verification {
    required_gates: Vec<String>,
    autorouter_policy: String,
    simulation_policy: String,
}

#[derive(Debug, Deserialize)]
struct Manufacturing {
    target_vendor: String,
    assembly_side: String,
    requires_bom_lcsc: bool,
    requires_cpl: bool,
    requires_gerbers: bool,
    requires_drills: bool,
    requires_drc_clean: bool,
    license_marking: String,
}

fn main() {
    let root = std::env::current_dir().expect("current dir");
    let contract = load_contract(&root.join(CONTRACT_PATH));
    let parts = load_parts(&root.join(PARTS_PATH));
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    require_file(&root.join(README_PATH), &mut errors);
    require_file(&root.join(DRU_PATH), &mut errors);
    require_file(&root.join(KIBOT_PATH), &mut errors);
    require_file(&root.join(PARTS_PATH), &mut errors);
    require_file(&root.join(OPTICAL_MODE_PATH), &mut errors);
    require_file(&root.join(SCHEMATIC_PATH), &mut errors);
    require_file(&root.join(SYM_LIB_TABLE_PATH), &mut errors);
    require_file(&root.join(FP_LIB_TABLE_PATH), &mut errors);
    require_file(&root.join(BOARD_PATH), &mut errors);

    validate_board(&contract, &mut errors);
    validate_stackup(&contract, &mut errors);
    validate_zones(&contract, &mut errors);
    let nets = expand_nets(&contract);
    validate_nets(&nets, &contract, &mut errors);
    validate_gpio(&contract, &nets, &mut errors);
    validate_test_points(&contract, &nets, &mut errors);
    validate_verification(&contract, &mut errors);
    validate_manufacturing(&contract, &mut errors);
    validate_schematic_shell(
        &root.join(SCHEMATIC_PATH),
        &parts,
        &contract,
        &nets,
        &mut errors,
    );
    validate_parts_manifest(&root, &parts, &contract, &nets, &mut errors, &mut warnings);
    validate_kicad_seed(
        &root.join(BOARD_PATH),
        &contract,
        &mut errors,
        &mut warnings,
    );

    if !warnings.is_empty() {
        println!("Warnings:");
        for warning in &warnings {
            println!("  - {warning}");
        }
    }

    if errors.is_empty() {
        println!("LAMP Rev A PCBA contract check passed.");
        println!(
            "  board: {} x {} mm, {} layers, {} slots",
            contract.board.width_mm,
            contract.board.height_mm,
            contract.board.layer_count,
            contract.board.slot_count
        );
        println!("  nets: {}", nets.len());
        println!("  gpio assignments: {}", contract.gpio_map.len());
        println!("  test points: {}", contract.test_points.len());
        println!("  selected part groups: {}", parts.selected_parts.len());
        println!(
            "  fab-blocking selection gaps: {}",
            fabrication_gap_count(&parts)
        );
    } else {
        eprintln!("LAMP Rev A PCBA contract check failed:");
        for error in errors {
            eprintln!("  - {error}");
        }
        std::process::exit(1);
    }
}

fn load_contract(path: &Path) -> Contract {
    let content =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    toml::from_str(&content).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn load_parts(path: &Path) -> PartsManifest {
    let content =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    toml::from_str(&content).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn require_file(path: &Path, errors: &mut Vec<String>) {
    if !path.is_file() {
        errors.push(format!("required file is missing: {}", path.display()));
    }
}

fn validate_board(contract: &Contract, errors: &mut Vec<String>) {
    let board = &contract.board;
    if board.physical_board_count != 1 {
        errors.push("Rev A must be one physical PCB".to_string());
    }
    if board.layer_count != 4 {
        errors.push("Rev A must use a 4-layer stackup".to_string());
    }
    if board.width_mm <= 0.0 || board.height_mm <= 0.0 {
        errors.push("board dimensions must be positive".to_string());
    }
    if board.thickness_mm < 1.0 {
        errors.push("board thickness is too low for the intended mechanical stack".to_string());
    }
    if board.slot_count != 8 {
        errors.push("LAMP Rev A must retain eight optical/tube slots".to_string());
    }
    if board.slot_spacing_mm < 12.0 {
        errors.push(
            "slot spacing must be at least 12 mm for the current optical package".to_string(),
        );
    }
    if !board.primary_mcu.contains("ESP32-S3-WROOM") {
        errors.push(
            "primary MCU must be an ESP32-S3-WROOM module, not a bare ESP32 chip".to_string(),
        );
    }
}

fn validate_stackup(contract: &Contract, errors: &mut Vec<String>) {
    let expected = ["F.Cu", "In1.Cu", "In2.Cu", "B.Cu"];
    if contract.stackup.copper_layers.len() != expected.len() {
        errors.push("stackup must declare exactly four copper layers".to_string());
    }
    for layer in expected {
        if !contract
            .stackup
            .copper_layers
            .iter()
            .any(|value| value == layer)
        {
            errors.push(format!("stackup is missing copper layer {layer}"));
        }
    }
    if contract.stackup.ground_plane_layer != "In1.Cu" {
        errors.push("In1.Cu must be the continuous ground plane".to_string());
    }
    if contract.stackup.power_plane_layer != "In2.Cu" {
        errors.push("In2.Cu must be the primary power plane".to_string());
    }
    if contract.stackup.min_clearance_mm < 0.15 {
        errors.push("minimum clearance must be at least 0.15 mm".to_string());
    }
    if contract.stackup.min_signal_track_mm < 0.20 {
        errors.push("minimum signal track width must be at least 0.20 mm".to_string());
    }
}

fn validate_zones(contract: &Contract, errors: &mut Vec<String>) {
    let required = [
        "mcu_usb",
        "analog_optical",
        "heater_power",
        "mechanical_thermal",
    ];
    let zones: BTreeSet<&str> = contract
        .zones
        .iter()
        .map(|zone| zone.name.as_str())
        .collect();
    for name in required {
        if !zones.contains(name) {
            errors.push(format!("missing placement zone: {name}"));
        }
    }

    for zone in &contract.zones {
        if zone.x_min_mm < 0.0
            || zone.y_min_mm < 0.0
            || zone.x_max_mm > contract.board.width_mm
            || zone.y_max_mm > contract.board.height_mm
            || zone.x_min_mm >= zone.x_max_mm
            || zone.y_min_mm >= zone.y_max_mm
        {
            errors.push(format!(
                "placement zone {} is outside board bounds",
                zone.name
            ));
        }
    }
}

fn expand_nets(contract: &Contract) -> Vec<Net> {
    let mut nets = contract.nets.clone();
    for group in &contract.net_groups {
        for idx in 0..group.count {
            nets.push(Net {
                name: format!("{}{}", group.prefix, idx),
                class: group.class.clone(),
                min_track_width_mm: group.min_track_width_mm,
                must_have_testpoint: group.must_have_testpoint,
            });
        }
    }
    nets
}

fn validate_nets(nets: &[Net], contract: &Contract, errors: &mut Vec<String>) {
    let mut by_name = BTreeMap::new();
    for net in nets {
        if by_name.insert(net.name.as_str(), net).is_some() {
            errors.push(format!("duplicate net: {}", net.name));
        }
        if net.min_track_width_mm < contract.stackup.min_signal_track_mm {
            errors.push(format!(
                "net {} has track width below stackup minimum",
                net.name
            ));
        }
        if net.class == "Heater" && net.min_track_width_mm < 1.5 {
            errors.push(format!("heater net {} must be at least 1.50 mm", net.name));
        }
        if net.class == "Power" && net.min_track_width_mm < 0.3 {
            errors.push(format!("power net {} must be at least 0.30 mm", net.name));
        }
    }

    for rail in &contract.rails {
        if !by_name.contains_key(rail.name.as_str()) {
            errors.push(format!("rail {} is missing from nets", rail.name));
        }
        if rail.max_current_ma >= 2500 {
            match by_name.get(rail.name.as_str()) {
                Some(net) if net.min_track_width_mm >= 1.0 => {}
                Some(_) => errors.push(format!(
                    "high-current rail {} needs at least 1.00 mm track width",
                    rail.name
                )),
                None => {}
            }
        }
    }
}

fn validate_gpio(contract: &Contract, nets: &[Net], errors: &mut Vec<String>) {
    let net_names: BTreeSet<&str> = nets.iter().map(|net| net.name.as_str()).collect();
    let mut pins = BTreeSet::new();
    let mut gpio_nets = BTreeSet::new();
    for gpio in &contract.gpio_map {
        if !pins.insert(gpio.esp32_module_pin) {
            errors.push(format!(
                "duplicate ESP32 module pin {}",
                gpio.esp32_module_pin
            ));
        }
        if !gpio_nets.insert(gpio.net.as_str()) {
            errors.push(format!("duplicate GPIO net {}", gpio.net));
        }
        if !net_names.contains(gpio.net.as_str()) {
            errors.push(format!(
                "GPIO net {} is missing from net contract",
                gpio.net
            ));
        }
        if !gpio.locked {
            errors.push(format!("GPIO assignment for {} must be locked", gpio.net));
        }
    }

    for required in [
        "USB_DP",
        "USB_DN",
        "HEATER_PWM",
        "SDA",
        "SCL",
        "UART_TX",
        "UART_RX",
    ] {
        if !gpio_nets.contains(required) {
            errors.push(format!("required GPIO assignment missing: {required}"));
        }
    }
}

fn validate_test_points(contract: &Contract, nets: &[Net], errors: &mut Vec<String>) {
    let net_names: BTreeSet<&str> = nets.iter().map(|net| net.name.as_str()).collect();
    let test_nets: BTreeSet<&str> = contract
        .test_points
        .iter()
        .map(|point| point.net.as_str())
        .collect();
    let mut test_names = BTreeSet::new();

    for point in &contract.test_points {
        if !test_names.insert(point.name.as_str()) {
            errors.push(format!("duplicate test point {}", point.name));
        }
        if !net_names.contains(point.net.as_str()) {
            errors.push(format!(
                "test point {} references unknown net {}",
                point.name, point.net
            ));
        }
    }

    for net in nets {
        if net.must_have_testpoint && !test_nets.contains(net.name.as_str()) {
            errors.push(format!("net {} requires a test point", net.name));
        }
    }

    for rail in &contract.rails {
        if rail.must_have_testpoint && !test_nets.contains(rail.name.as_str()) {
            errors.push(format!("rail {} requires a test point", rail.name));
        }
    }
}

fn validate_verification(contract: &Contract, errors: &mut Vec<String>) {
    let gates: BTreeSet<&str> = contract
        .verification
        .required_gates
        .iter()
        .map(String::as_str)
        .collect();
    for gate in [
        "contract_check",
        "kicad_erc",
        "kicad_drc",
        "schematic_pcb_parity",
        "bom_jlcpcb_fields",
        "cpl_jlcpcb_fields",
        "gerber_drill_export",
        "step_export",
        "pre_fab_human_review",
    ] {
        if !gates.contains(gate) {
            errors.push(format!("verification gate missing: {gate}"));
        }
    }
}

fn validate_manufacturing(contract: &Contract, errors: &mut Vec<String>) {
    if contract.manufacturing.target_vendor != "JLCPCB" {
        errors.push("target vendor must be JLCPCB for the current fab flow".to_string());
    }
    if contract.manufacturing.assembly_side != "top" {
        errors.push("Rev A assumes top-side assembly unless explicitly redesigned".to_string());
    }
    if !contract.manufacturing.requires_bom_lcsc
        || !contract.manufacturing.requires_cpl
        || !contract.manufacturing.requires_gerbers
        || !contract.manufacturing.requires_drills
        || !contract.manufacturing.requires_drc_clean
    {
        errors.push(
            "manufacturing contract must require BOM, CPL, Gerbers, drills, and clean DRC"
                .to_string(),
        );
    }
}

fn validate_schematic_shell(
    path: &Path,
    parts: &PartsManifest,
    contract: &Contract,
    nets: &[Net],
    errors: &mut Vec<String>,
) {
    let schematic = match fs::read_to_string(path) {
        Ok(schematic) => schematic,
        Err(_) => return,
    };

    if !schematic.contains("(kicad_sch") {
        errors.push("schematic root is not a KiCad schematic".to_string());
    }
    if !schematic.contains("LaminarForge LAMP Rev A PCBA") {
        errors.push("schematic title must identify LaminarForge LAMP Rev A PCBA".to_string());
    }
    if !schematic.contains(&contract.board.primary_mcu) {
        errors.push("schematic shell must name the locked ESP32-S3 module".to_string());
    }

    let net_names: BTreeSet<&str> = nets.iter().map(|net| net.name.as_str()).collect();
    for block in &parts.blocks {
        if !schematic.contains(&block.title) {
            errors.push(format!("schematic shell is missing block {}", block.title));
        }
        for net in &block.required_nets {
            if !net_names.contains(net.as_str()) {
                errors.push(format!(
                    "schematic block {} references unknown required net {}",
                    block.name, net
                ));
            }
        }
    }
}

fn validate_parts_manifest(
    root: &Path,
    parts: &PartsManifest,
    contract: &Contract,
    nets: &[Net],
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    validate_parts_package(parts, contract, errors);
    validate_parts_sources(parts, errors);

    let module_names: BTreeSet<&str> = contract
        .modules
        .iter()
        .map(|module| module.name.as_str())
        .collect();
    let net_names: BTreeSet<&str> = nets.iter().map(|net| net.name.as_str()).collect();
    let symbol_library = fs::read_to_string(root.join(SYMBOL_LIBRARY_PATH)).unwrap_or_else(|err| {
        errors.push(format!("read symbol library {SYMBOL_LIBRARY_PATH}: {err}"));
        String::new()
    });

    validate_blocks(parts, &module_names, &net_names, errors);
    validate_selected_parts(
        root,
        parts,
        &module_names,
        &net_names,
        &symbol_library,
        errors,
    );
    validate_selection_gaps(parts, &module_names, contract, errors, warnings);
}

fn validate_parts_package(parts: &PartsManifest, contract: &Contract, errors: &mut Vec<String>) {
    if parts.package.name != "lamp_rev_a_parts" {
        errors.push("parts package name must be lamp_rev_a_parts".to_string());
    }
    if parts.package.ticket != contract.package.ticket {
        errors.push(format!(
            "parts ticket {} does not match contract ticket {}",
            parts.package.ticket, contract.package.ticket
        ));
    }
    if parts.package.revision != contract.package.revision {
        errors.push("parts revision must match contract revision".to_string());
    }
    if parts.package.source_stage != contract.package.source_stage {
        errors.push("parts source stage must match contract source stage".to_string());
    }
}

fn validate_parts_sources(parts: &PartsManifest, errors: &mut Vec<String>) {
    if parts.schematic.root_file != "lamp_rev_a.kicad_sch" {
        errors.push("parts schematic root_file must be lamp_rev_a.kicad_sch".to_string());
    }
    if parts.schematic.symbol_library != "../lib/lcsc.kicad_sym" {
        errors
            .push("parts schematic symbol_library must point at ../lib/lcsc.kicad_sym".to_string());
    }
    if parts.schematic.footprint_library != "../lib/lcsc.pretty" {
        errors
            .push("parts schematic footprint_library must point at ../lib/lcsc.pretty".to_string());
    }
}

fn validate_blocks(
    parts: &PartsManifest,
    module_names: &BTreeSet<&str>,
    net_names: &BTreeSet<&str>,
    errors: &mut Vec<String>,
) {
    let mut block_names = BTreeSet::new();
    for block in &parts.blocks {
        if !block_names.insert(block.name.as_str()) {
            errors.push(format!("duplicate schematic block {}", block.name));
        }
        if block.title.trim().is_empty() {
            errors.push(format!("schematic block {} needs a title", block.name));
        }
        for module in &block.modules {
            if !module_names.contains(module.as_str()) {
                errors.push(format!(
                    "schematic block {} references unknown module {}",
                    block.name, module
                ));
            }
        }
        for net in &block.required_nets {
            if !net_names.contains(net.as_str()) {
                errors.push(format!(
                    "schematic block {} references unknown net {}",
                    block.name, net
                ));
            }
        }
    }
}

fn validate_selected_parts(
    root: &Path,
    parts: &PartsManifest,
    module_names: &BTreeSet<&str>,
    net_names: &BTreeSet<&str>,
    symbol_library: &str,
    errors: &mut Vec<String>,
) {
    let mut ids = BTreeSet::new();
    let mut selected_modules = BTreeSet::new();

    for part in &parts.selected_parts {
        if !ids.insert(part.id.as_str()) {
            errors.push(format!("duplicate selected part id {}", part.id));
        }
        if !module_names.contains(part.module.as_str()) {
            errors.push(format!(
                "selected part {} references unknown module {}",
                part.id, part.module
            ));
        }
        selected_modules.insert(part.module.as_str());
        if part.quantity == 0 {
            errors.push(format!("selected part {} has zero quantity", part.id));
        }
        if part.reference_prefix.trim().is_empty() {
            errors.push(format!(
                "selected part {} needs a reference prefix",
                part.id
            ));
        }
        if part.value.trim().is_empty() {
            errors.push(format!("selected part {} needs a value", part.id));
        }
        if !part.lcsc_part.starts_with('C')
            || !part.lcsc_part[1..].chars().all(|ch| ch.is_ascii_digit())
        {
            errors.push(format!(
                "selected part {} has invalid LCSC part {}",
                part.id, part.lcsc_part
            ));
        }
        if !symbol_library.contains(&format!("(symbol \"{}\"", part.symbol)) {
            errors.push(format!(
                "selected part {} symbol {} is missing from {}",
                part.id, part.symbol, SYMBOL_LIBRARY_PATH
            ));
        }
        match footprint_path(root, &part.footprint) {
            Some(path) if path.is_file() => {}
            Some(path) => errors.push(format!(
                "selected part {} footprint {} is missing at {}",
                part.id,
                part.footprint,
                path.display()
            )),
            None => errors.push(format!(
                "selected part {} footprint {} must use library:name form",
                part.id, part.footprint
            )),
        }
        if part.verification.trim().is_empty() {
            errors.push(format!(
                "selected part {} needs verification notes",
                part.id
            ));
        }
        for net in &part.nets {
            if !net_names.contains(net.as_str()) {
                errors.push(format!(
                    "selected part {} references unknown net {}",
                    part.id, net
                ));
            }
        }
    }

    for required in [
        "esp32_s3",
        "usb_programming",
        "power",
        "heater_drive",
        "optical_detection",
        "firmware_debug",
    ] {
        if !selected_modules.contains(required) {
            errors.push(format!("required module {required} has no selected parts"));
        }
    }
}

fn validate_selection_gaps(
    parts: &PartsManifest,
    module_names: &BTreeSet<&str>,
    contract: &Contract,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let mut ids = BTreeSet::new();
    for gap in &parts.selection_gaps {
        if !ids.insert(gap.id.as_str()) {
            errors.push(format!("duplicate selection gap {}", gap.id));
        }
        if !module_names.contains(gap.module.as_str()) {
            errors.push(format!(
                "selection gap {} references unknown module {}",
                gap.id, gap.module
            ));
        }
        if gap.reason.trim().is_empty() || gap.resolve_with.trim().is_empty() {
            errors.push(format!(
                "selection gap {} needs reason and resolve_with text",
                gap.id
            ));
        }
        if gap.blocks_fabrication {
            warnings.push(format!(
                "fab blocked by {}: {} Resolve with: {}",
                gap.id, gap.reason, gap.resolve_with
            ));
        }
    }

    if contract.package.source_stage == "fabrication_release" && fabrication_gap_count(parts) > 0 {
        errors.push("fabrication_release cannot have fab-blocking selection gaps".to_string());
    }
}

fn footprint_path(root: &Path, footprint: &str) -> Option<std::path::PathBuf> {
    let (library, name) = footprint.split_once(':')?;
    Some(root.join(format!("pcb/lib/{library}.pretty/{name}.kicad_mod")))
}

fn fabrication_gap_count(parts: &PartsManifest) -> usize {
    parts
        .selection_gaps
        .iter()
        .filter(|gap| gap.blocks_fabrication)
        .count()
}

fn validate_kicad_seed(
    path: &Path,
    contract: &Contract,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let board = match fs::read_to_string(path) {
        Ok(board) => board,
        Err(_) => return,
    };

    for layer in &contract.stackup.copper_layers {
        if !board.contains(&format!("\"{layer}\"")) {
            errors.push(format!("KiCad seed is missing copper layer {layer}"));
        }
    }

    let has_width = board.contains(&format!(
        "(end {} {}",
        contract.board.width_mm, contract.board.height_mm
    )) || board.contains(&format!(
        "(end {:.0} {:.0}",
        contract.board.width_mm, contract.board.height_mm
    ));
    if !has_width {
        errors.push("KiCad seed outline does not match contract dimensions".to_string());
    }

    if !board.contains("LaminarForge LAMP Rev A") {
        warnings.push("KiCad seed has no Rev A silkscreen marker".to_string());
    }
}
