#![allow(dead_code)]

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const ROOT_DIR: &str = "pcb/lamp_rev_b_controller";
const CONTRACT_PATH: &str = "pcb/lamp_rev_b_controller/contract.toml";
const PARTS_PATH: &str = "pcb/lamp_rev_b_controller/parts.toml";
const POWER_PATH: &str = "pcb/lamp_rev_b_controller/power_architecture.toml";
const OPTICAL_PATH: &str = "pcb/lamp_rev_b_controller/optical_interface.toml";
const PLACEMENT_PATH: &str = "pcb/lamp_rev_b_controller/placement.toml";
const PIN_NETS_PATH: &str = "pcb/lamp_rev_b_controller/pin_nets.toml";
const FIRMWARE_PATH: &str = "pcb/lamp_rev_b_controller/firmware_handoff.toml";
const ELECTRICAL_PATH: &str = "pcb/lamp_rev_b_controller/electrical_validation.toml";
const ROUTING_PLAN_PATH: &str = "pcb/lamp_rev_b_controller/routing_plan.toml";
const ROUTING_SEED_PATH: &str = "pcb/lamp_rev_b_controller/routing_seed.toml";
const COPPER_ZONES_PATH: &str = "pcb/lamp_rev_b_controller/copper_zones.toml";
const FAB_CONFIG_PATH: &str = "pcb/lamp_rev_b_controller/fab_release.toml";
const README_PATH: &str = "pcb/lamp_rev_b_controller/README.md";
const BRINGUP_PATH: &str = "pcb/lamp_rev_b_controller/manufacturing_bringup.md";
const FOLLOWUP_PATH: &str = "pcb/lamp_rev_b_controller/fab_release_followup.md";
const SCH_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_sch";
const PCB_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_pcb";
const PRO_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_pro";
const DRU_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_dru";
const KIBOT_PATH: &str = "pcb/lamp_rev_b_controller/kibot.yaml";
const SYM_LIB_TABLE_PATH: &str = "pcb/lamp_rev_b_controller/sym-lib-table";
const FP_LIB_TABLE_PATH: &str = "pcb/lamp_rev_b_controller/fp-lib-table";
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
    manufacturing: Manufacturing,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    ticket: String,
    revision: String,
    purpose: String,
    source_artifacts: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Board {
    physical_board_count: u32,
    width_mm: f64,
    height_mm: f64,
    thickness_mm: f64,
    layer_count: u32,
    spatial_lane_count: u32,
    primary_mcu: String,
    prototype_scope: String,
}

#[derive(Debug, Deserialize)]
struct Stackup {
    copper_layers: Vec<String>,
    ground_plane_layer: String,
    power_plane_layer: String,
    min_clearance_mm: f64,
    min_signal_track_mm: f64,
    min_via_drill_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Zone {
    name: String,
    x_min_mm: f64,
    x_max_mm: f64,
    y_min_mm: f64,
    y_max_mm: f64,
}

#[derive(Debug, Deserialize)]
struct Module {
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct Rail {
    name: String,
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
}

#[derive(Debug, Deserialize)]
struct Manufacturing {
    assembly_side: String,
    first_article_quantity: u32,
    requires_drc_clean: bool,
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
}

#[derive(Debug, Deserialize)]
struct SchematicSource {
    symbol_library: String,
    footprint_library: String,
}

#[derive(Debug, Deserialize)]
struct SchematicBlock {
    name: String,
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
}

#[derive(Debug, Deserialize)]
struct SelectionGap {
    id: String,
    module: String,
    blocks_fabrication: bool,
}

#[derive(Debug, Deserialize)]
struct PlacementPlan {
    placements: Vec<Placement>,
    test_points: Vec<TestPointPlacement>,
}

#[derive(Debug, Deserialize)]
struct Placement {
    reference: String,
    part_id: String,
    zone: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct TestPointPlacement {
    name: String,
    net: String,
    x_mm: f64,
    y_mm: f64,
    side: String,
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
struct CopperZonePlan {
    zones: Vec<CopperZone>,
}

#[derive(Debug, Deserialize)]
struct CopperZone {
    name: String,
    net: String,
    layer: String,
    points: Vec<CopperZonePoint>,
}

#[derive(Debug, Deserialize)]
struct CopperZonePoint {
    x_mm: f64,
    y_mm: f64,
}

fn main() {
    let root = std::env::current_dir().expect("current dir");
    let contract = load_toml::<Contract>(&root.join(CONTRACT_PATH));
    let parts = load_toml::<PartsManifest>(&root.join(PARTS_PATH));
    let placement = load_toml::<PlacementPlan>(&root.join(PLACEMENT_PATH));
    let pin_nets = load_toml::<PinNetManifest>(&root.join(PIN_NETS_PATH));
    let copper_zones = load_toml::<CopperZonePlan>(&root.join(COPPER_ZONES_PATH));

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for path in [
        ROOT_DIR,
        CONTRACT_PATH,
        PARTS_PATH,
        POWER_PATH,
        OPTICAL_PATH,
        PLACEMENT_PATH,
        PIN_NETS_PATH,
        FIRMWARE_PATH,
        ELECTRICAL_PATH,
        ROUTING_PLAN_PATH,
        ROUTING_SEED_PATH,
        COPPER_ZONES_PATH,
        FAB_CONFIG_PATH,
        README_PATH,
        BRINGUP_PATH,
        FOLLOWUP_PATH,
        SCH_PATH,
        PCB_PATH,
        PRO_PATH,
        DRU_PATH,
        KIBOT_PATH,
        SYM_LIB_TABLE_PATH,
        FP_LIB_TABLE_PATH,
        SYMBOL_LIBRARY_PATH,
    ] {
        require_path(&root.join(path), path, &mut errors);
    }

    let nets = expand_nets(&contract);
    validate_contract(&contract, &nets, &mut errors);
    validate_parts(&root, &parts, &contract, &nets, &mut errors, &mut warnings);
    validate_placement(&placement, &parts, &contract, &nets, &mut errors);
    validate_pin_nets(&pin_nets, &placement, &nets, &mut errors);
    validate_copper_zones(&copper_zones, &contract, &nets, &mut errors);
    validate_text_content(&root, &mut errors, &mut warnings);

    if !warnings.is_empty() {
        println!("Warnings:");
        for warning in &warnings {
            println!("  - {warning}");
        }
    }

    if errors.is_empty() {
        println!("LAMP Rev B controller PCBA contract check passed.");
        println!(
            "  board: {} x {} mm, {} layers",
            contract.board.width_mm, contract.board.height_mm, contract.board.layer_count
        );
        println!("  selected part groups: {}", parts.selected_parts.len());
        println!("  placed footprints: {}", placement.placements.len());
        println!("  test points: {}", placement.test_points.len());
        println!("  nets: {}", nets.len());
        println!("  gpio assignments: {}", contract.gpio_map.len());
        println!(
            "  pin-net assignment groups: {}",
            pin_nets.assignments.len()
        );
        println!(
            "  fab-release blocking gaps: {}",
            parts
                .selection_gaps
                .iter()
                .filter(|gap| gap.blocks_fabrication)
                .count()
        );
    } else {
        eprintln!("LAMP Rev B controller PCBA contract check failed:");
        for error in errors {
            eprintln!("  - {error}");
        }
        std::process::exit(1);
    }
}

fn load_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let content =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    toml::from_str(&content).unwrap_or_else(|err| panic!("parse {}: {err}", path.display()))
}

fn require_path(path: &Path, label: &str, errors: &mut Vec<String>) {
    if !path.exists() {
        errors.push(format!("required path is missing: {label}"));
    }
}

fn expand_nets(contract: &Contract) -> BTreeMap<String, Net> {
    let mut nets = contract
        .nets
        .iter()
        .cloned()
        .map(|net| (net.name.clone(), net))
        .collect::<BTreeMap<_, _>>();
    for group in &contract.net_groups {
        for index in 0..group.count {
            let name = format!("{}{}", group.prefix, index);
            nets.insert(
                name.clone(),
                Net {
                    name,
                    class: group.class.clone(),
                    min_track_width_mm: group.min_track_width_mm,
                    must_have_testpoint: group.must_have_testpoint,
                },
            );
        }
    }
    nets
}

fn validate_contract(contract: &Contract, nets: &BTreeMap<String, Net>, errors: &mut Vec<String>) {
    if contract.package.name != "lamp_rev_b_controller" {
        errors.push("contract package name must be lamp_rev_b_controller".to_string());
    }
    if contract.package.ticket != "T-4C206871" {
        errors.push("contract ticket must be T-4C206871".to_string());
    }
    for required in [
        "A-45684C9D",
        "A-1BDCE3AC",
        "A-A17708F1",
        "A-CEDCB22B",
        "A-8633FB3D",
        "A-5F5799DF",
        "A-6804F5CC",
    ] {
        if !contract
            .package
            .source_artifacts
            .iter()
            .any(|artifact| artifact == required)
        {
            errors.push(format!("contract missing source artifact {required}"));
        }
    }
    if contract.board.physical_board_count != 1 {
        errors.push("Rev B controller must be one main carrier PCB".to_string());
    }
    if contract.board.width_mm != 118.0 || contract.board.height_mm != 88.0 {
        errors.push("Rev B controller envelope must be 118 x 88 mm".to_string());
    }
    if contract.board.layer_count != 4 {
        errors.push("Rev B controller must use a 4-layer stackup".to_string());
    }
    if contract.board.prototype_scope != "controller_carrier_only" {
        errors.push("prototype scope must remain controller_carrier_only".to_string());
    }
    if !contract.board.primary_mcu.contains("ESP32-S3-WROOM-1-N16") {
        errors.push("primary MCU must name ESP32-S3-WROOM-1-N16".to_string());
    }
    if contract.stackup.copper_layers != ["F.Cu", "In1.Cu", "In2.Cu", "B.Cu"] {
        errors.push("stackup copper layer order must match the 4-layer Rev B plan".to_string());
    }
    if contract.stackup.ground_plane_layer != "In1.Cu" {
        errors.push("In1.Cu must be the ground plane".to_string());
    }
    if contract.stackup.power_plane_layer != "In2.Cu" {
        errors.push("In2.Cu must be the power/control plane".to_string());
    }
    for zone in &contract.zones {
        if zone.x_min_mm < 0.0
            || zone.y_min_mm < 0.0
            || zone.x_max_mm > contract.board.width_mm
            || zone.y_max_mm > contract.board.height_mm
            || zone.x_min_mm >= zone.x_max_mm
            || zone.y_min_mm >= zone.y_max_mm
        {
            errors.push(format!("zone {} is outside board bounds", zone.name));
        }
    }
    let modules = contract
        .modules
        .iter()
        .map(|module| module.name.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "esp32_s3",
        "usb_programming_power",
        "power_entry",
        "heater_drive",
        "temperature_sense",
        "optics_control",
        "interlocks",
        "firmware_debug_logging",
        "manufacturing_test",
    ] {
        if !modules.contains(required) {
            errors.push(format!("missing required module {required}"));
        }
    }
    for net in [
        "USB_DP",
        "USB_DN",
        "VIN_12_24",
        "VIN_HEATER",
        "+5V",
        "+3V3",
        "+3V3_ANA",
        "HEATER0_PWM",
        "HEATER1_PWM",
        "HEATER_ARM",
        "THERM_MUX_OUT",
        "LED_EXC_PWM",
        "LED_EXC_EN",
        "FRAME_TRIG_OUT",
        "LID_CLOSED_N",
        "CART_PRESENT_N",
    ] {
        if !nets.contains_key(net) {
            errors.push(format!("missing required net {net}"));
        }
    }
    for rail in &contract.rails {
        if rail.must_have_testpoint && !contract.test_points.iter().any(|tp| tp.net == rail.name) {
            errors.push(format!("rail {} requires a test point", rail.name));
        }
    }
    for net in nets.values().filter(|net| net.must_have_testpoint) {
        if !contract.test_points.iter().any(|tp| tp.net == net.name) {
            errors.push(format!("net {} requires a contract test point", net.name));
        }
    }
    for gpio in &contract.gpio_map {
        if !nets.contains_key(&gpio.net) {
            errors.push(format!(
                "GPIO pad {} maps to unknown net {}",
                gpio.esp32_module_pin, gpio.net
            ));
        }
    }
    let forbidden_gpio_nets = ["GPIO3", "GPIO45", "GPIO46"];
    for gpio in &contract.gpio_map {
        for forbidden in forbidden_gpio_nets {
            if contains_gpio_token(&gpio.function, forbidden) {
                errors.push(format!(
                    "strap-sensitive {forbidden} must not have an application GPIO assignment"
                ));
            }
        }
    }
    if contract.manufacturing.assembly_side != "top" {
        errors.push("first article assembly side must be top".to_string());
    }
    if contract.manufacturing.first_article_quantity != 5 {
        errors.push("first article quantity must be 5 assembled PCBAs".to_string());
    }
    if !contract.manufacturing.requires_drc_clean {
        errors.push("manufacturing policy must require DRC-clean release".to_string());
    }
}

fn contains_gpio_token(function: &str, token: &str) -> bool {
    function
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .any(|part| part == token)
}

fn validate_parts(
    root: &Path,
    parts: &PartsManifest,
    contract: &Contract,
    nets: &BTreeMap<String, Net>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    if parts.package.name != "lamp_rev_b_controller_parts" {
        errors.push("parts package name must be lamp_rev_b_controller_parts".to_string());
    }
    if parts.package.ticket != "T-4C206871" {
        errors.push("parts ticket must be T-4C206871".to_string());
    }
    let module_names = contract
        .modules
        .iter()
        .map(|module| module.name.as_str())
        .collect::<BTreeSet<_>>();
    let symbol_library =
        fs::read_to_string(root.join(SYMBOL_LIBRARY_PATH)).unwrap_or_else(|_| String::new());
    let footprint_dir = root.join(ROOT_DIR).join(&parts.schematic.footprint_library);
    if !footprint_dir.is_dir() {
        errors.push(format!(
            "footprint library path does not exist: {}",
            footprint_dir.display()
        ));
    }
    for block in &parts.blocks {
        for net in &block.required_nets {
            if !nets.contains_key(net) {
                errors.push(format!("block {} requires unknown net {}", block.name, net));
            }
        }
    }
    let mut part_ids = BTreeSet::new();
    let parts_by_id = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    for part in &parts.selected_parts {
        if !part_ids.insert(part.id.as_str()) {
            errors.push(format!("duplicate selected part id {}", part.id));
        }
        if !module_names.contains(part.module.as_str()) {
            errors.push(format!(
                "selected part {} references unknown module {}",
                part.id, part.module
            ));
        }
        if part.quantity == 0 {
            errors.push(format!("selected part {} has zero quantity", part.id));
        }
        if part.reference_prefix.is_empty() {
            errors.push(format!(
                "selected part {} has empty reference prefix",
                part.id
            ));
        }
        if part.value.trim().is_empty() || part.lcsc_part.trim().is_empty() {
            errors.push(format!("selected part {} is missing value/source", part.id));
        }
        if !symbol_library.contains(&format!("(symbol \"{}\"", part.symbol)) {
            errors.push(format!(
                "selected part {} references missing symbol {}",
                part.id, part.symbol
            ));
        }
        match part.footprint.split_once(':') {
            Some((_, footprint)) => {
                let path = footprint_dir.join(format!("{footprint}.kicad_mod"));
                if !path.is_file() {
                    errors.push(format!(
                        "selected part {} references missing footprint {}",
                        part.id,
                        path.display()
                    ));
                }
            }
            None => errors.push(format!(
                "selected part {} footprint {} lacks library prefix",
                part.id, part.footprint
            )),
        }
        for net in &part.nets {
            if !nets.contains_key(net) {
                errors.push(format!(
                    "selected part {} references unknown net {}",
                    part.id, net
                ));
            }
        }
    }
    require_selected_part(
        &parts_by_id,
        "five_v_buck_module",
        "P7805-2000-S",
        "lcsc:P78-2000-S_SIP3",
        "C2848816",
        &["VIN_PROTECTED", "+5V", "GND"],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "three_v_three_regulator",
        "AP63203WU-7",
        "lcsc:TSOT26_AP63203WU",
        "C780769",
        &["+5V", "+3V3", "3V3_SW", "3V3_BST", "GND"],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "three_v_three_inductor",
        "74439346068",
        "lcsc:IND_74439346068",
        "C2041388",
        &["3V3_SW", "+3V3"],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "led_driver_module",
        "LDD-700H",
        "lcsc:MEANWELL_LDD-300-1000H_THT",
        "C17537709",
        &[
            "LED_SUPPLY",
            "GND",
            "LED_DIM_DRIVE",
            "LED_DRV_PLUS",
            "LED_MINUS",
        ],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "led_dim_and_gate",
        "SN74LVC1G08DBVR",
        "lcsc:SOT-23-5",
        "C7666",
        &["+3V3", "GND", "LED_EXC_PWM", "LED_EXC_EN", "LED_DIM_GATE"],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "led_current_sense_amp",
        "INA180A1IDBVR",
        "lcsc:SOT-23-5",
        "C122228",
        &[
            "+3V3",
            "GND",
            "LED_DRV_PLUS",
            "LED_PLUS",
            "LED_CURRENT_SENSE",
        ],
        errors,
    );
    require_selected_part(
        &parts_by_id,
        "led_current_shunt",
        "0.1R",
        "lcsc:R2512",
        "",
        &["LED_DRV_PLUS", "LED_PLUS"],
        errors,
    );
    for gap in &parts.selection_gaps {
        if !module_names.contains(gap.module.as_str()) {
            errors.push(format!(
                "selection gap {} references unknown module {}",
                gap.id, gap.module
            ));
        }
        if gap.blocks_fabrication {
            warnings.push(format!(
                "fab release blocked by selection gap {} in module {}",
                gap.id, gap.module
            ));
        }
    }
    for resolved_blocker in [
        "integrated_buck_regulator",
        "constant_current_led_driver_ic",
    ] {
        if parts
            .selection_gaps
            .iter()
            .any(|gap| gap.id == resolved_blocker && gap.blocks_fabrication)
        {
            errors.push(format!(
                "resolved release blocker {resolved_blocker} must not remain fabrication-blocking"
            ));
        }
    }
    for forbidden in ["turbidity_emitters", "photodiodes", "optical_tia_opamp"] {
        if part_ids.contains(forbidden) {
            errors.push(format!(
                "Rev A optical part group {forbidden} must not appear"
            ));
        }
    }
}

fn require_selected_part(
    parts_by_id: &BTreeMap<&str, &SelectedPart>,
    id: &str,
    value_token: &str,
    footprint: &str,
    lcsc_token: &str,
    required_nets: &[&str],
    errors: &mut Vec<String>,
) {
    let Some(part) = parts_by_id.get(id) else {
        errors.push(format!("missing selected part {id}"));
        return;
    };
    if !value_token.is_empty() && !part.value.contains(value_token) {
        errors.push(format!(
            "selected part {id} value must mention {value_token}"
        ));
    }
    if part.footprint != footprint {
        errors.push(format!(
            "selected part {id} footprint must be {footprint}, got {}",
            part.footprint
        ));
    }
    if !lcsc_token.is_empty() && !part.lcsc_part.contains(lcsc_token) {
        errors.push(format!(
            "selected part {id} source must mention {lcsc_token}"
        ));
    }
    for net in required_nets {
        if !part.nets.iter().any(|candidate| candidate == net) {
            errors.push(format!("selected part {id} missing required net {net}"));
        }
    }
}

fn validate_placement(
    placement: &PlacementPlan,
    parts: &PartsManifest,
    contract: &Contract,
    nets: &BTreeMap<String, Net>,
    errors: &mut Vec<String>,
) {
    let zones = contract
        .zones
        .iter()
        .map(|zone| zone.name.as_str())
        .collect::<BTreeSet<_>>();
    let part_ids = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part.quantity))
        .collect::<BTreeMap<_, _>>();
    let mut by_part: BTreeMap<&str, u32> = BTreeMap::new();
    let mut refs = BTreeSet::new();
    for item in &placement.placements {
        if !refs.insert(item.reference.as_str()) {
            errors.push(format!("duplicate placement reference {}", item.reference));
        }
        if !part_ids.contains_key(item.part_id.as_str()) {
            errors.push(format!(
                "placement {} references unknown part {}",
                item.reference, item.part_id
            ));
        }
        if !zones.contains(item.zone.as_str()) {
            errors.push(format!(
                "placement {} references unknown zone {}",
                item.reference, item.zone
            ));
        }
        if item.side != "top" {
            errors.push(format!("placement {} must be top side", item.reference));
        }
        if item.x_mm < 0.0
            || item.y_mm < 0.0
            || item.x_mm > contract.board.width_mm
            || item.y_mm > contract.board.height_mm
            || !item.rotation_deg.is_finite()
        {
            errors.push(format!(
                "placement {} is outside board bounds",
                item.reference
            ));
        }
        *by_part.entry(item.part_id.as_str()).or_default() += 1;
    }
    for part in &parts.selected_parts {
        let found = by_part.get(part.id.as_str()).copied().unwrap_or_default();
        if found != part.quantity {
            errors.push(format!(
                "part {} expects {} placements but found {}",
                part.id, part.quantity, found
            ));
        }
    }
    let contract_tps = contract
        .test_points
        .iter()
        .map(|tp| tp.name.as_str())
        .collect::<BTreeSet<_>>();
    for tp in &placement.test_points {
        if !contract_tps.contains(tp.name.as_str()) {
            errors.push(format!("placed test point {} is not in contract", tp.name));
        }
        if !nets.contains_key(&tp.net) {
            errors.push(format!(
                "placed test point {} uses unknown net {}",
                tp.name, tp.net
            ));
        }
        if tp.side != "top" {
            errors.push(format!("test point {} must be top side", tp.name));
        }
        if tp.x_mm < 0.0
            || tp.y_mm < 0.0
            || tp.x_mm > contract.board.width_mm
            || tp.y_mm > contract.board.height_mm
        {
            errors.push(format!("test point {} is outside board bounds", tp.name));
        }
    }
}

fn validate_pin_nets(
    pin_nets: &PinNetManifest,
    placement: &PlacementPlan,
    nets: &BTreeMap<String, Net>,
    errors: &mut Vec<String>,
) {
    let placed_refs = placement
        .placements
        .iter()
        .map(|item| item.reference.as_str())
        .collect::<BTreeSet<_>>();
    let mut refs = BTreeSet::new();
    for assignment in &pin_nets.assignments {
        if !refs.insert(assignment.reference.as_str()) {
            errors.push(format!(
                "duplicate pin-net assignment {}",
                assignment.reference
            ));
        }
        if !placed_refs.contains(assignment.reference.as_str()) {
            errors.push(format!(
                "pin-net assignment {} has no matching placement",
                assignment.reference
            ));
        }
        for net in assignment.pins.values() {
            if !nets.contains_key(net) {
                errors.push(format!(
                    "pin-net assignment {} references unknown net {}",
                    assignment.reference, net
                ));
            }
        }
    }
}

fn validate_copper_zones(
    copper_zones: &CopperZonePlan,
    contract: &Contract,
    nets: &BTreeMap<String, Net>,
    errors: &mut Vec<String>,
) {
    for zone in &copper_zones.zones {
        if !nets.contains_key(&zone.net) {
            errors.push(format!(
                "copper zone {} uses unknown net {}",
                zone.name, zone.net
            ));
        }
        if !contract
            .stackup
            .copper_layers
            .iter()
            .any(|layer| layer == &zone.layer)
        {
            errors.push(format!(
                "copper zone {} uses unsupported layer {}",
                zone.name, zone.layer
            ));
        }
        if zone.points.len() < 3 {
            errors.push(format!(
                "copper zone {} needs at least three points",
                zone.name
            ));
        }
        for point in &zone.points {
            if point.x_mm < 0.0
                || point.y_mm < 0.0
                || point.x_mm > contract.board.width_mm
                || point.y_mm > contract.board.height_mm
            {
                errors.push(format!("copper zone {} has off-board point", zone.name));
            }
        }
    }
}

fn validate_text_content(root: &Path, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
    let package_text = fs::read_to_string(root.join(CONTRACT_PATH)).unwrap_or_default()
        + &fs::read_to_string(root.join(PARTS_PATH)).unwrap_or_default()
        + &fs::read_to_string(root.join(OPTICAL_PATH)).unwrap_or_default()
        + &fs::read_to_string(root.join(README_PATH)).unwrap_or_default();
    for forbidden in ["650 nm turbidimetry", "eight LED/photodiode channels"] {
        if package_text.contains(forbidden) {
            errors.push(format!(
                "Rev A retired optical path text remains: {forbidden}"
            ));
        }
    }
    let followup = fs::read_to_string(root.join(FOLLOWUP_PATH)).unwrap_or_default();
    if !followup.contains("lamp_rev_b_controller_fab_release") {
        warnings.push("fab_release follow-up does not name the future release binary".to_string());
    }
}
