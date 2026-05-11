use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const CONFIG_PATH: &str = "pcb/lamp_rev_a/fab_release.toml";

#[derive(Debug, Deserialize)]
struct ReleaseConfig {
    package: Package,
    inputs: Inputs,
    outputs: Outputs,
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
struct Inputs {
    board: String,
    schematic: String,
    parts: String,
    placement: String,
}

#[derive(Debug, Deserialize)]
struct Outputs {
    root_dir: String,
    reports_dir: String,
    gerbers_dir: String,
    drills_dir: String,
    assembly_dir: String,
    review_dir: String,
    drc_report: String,
    erc_report: String,
    drill_report: String,
    bom_file: String,
    cpl_file: String,
    position_file: String,
    step_file: String,
    manifest_file: String,
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
    let parts_path = root.join(&config.inputs.parts);
    let placement_path = root.join(&config.inputs.placement);
    ensure_file(&board)?;
    ensure_file(&schematic)?;
    ensure_file(&parts_path)?;
    ensure_file(&placement_path)?;

    run_command("kicad-cli", &["version".to_string()])?;

    let drc_report = output_root.join(&config.outputs.drc_report);
    let erc_report = output_root.join(&config.outputs.erc_report);

    run_kicad_drc(&board, &drc_report)?;
    let (drc_violations, real_unconnected, ignored_self_zone) =
        validate_drc_report(&drc_report, &config.gates)?;

    run_kicad_erc(&schematic, &erc_report)?;
    let erc_violations = validate_erc_report(&erc_report, &config.gates)?;

    let parts = read_toml::<PartsManifest>(&parts_path)?;
    let placement = read_toml::<PlacementPlan>(&placement_path)?;
    validate_assembly_sources(&parts, &placement)?;
    write_bom(
        &parts,
        &placement,
        &output_root.join(&config.outputs.bom_file),
    )?;
    write_cpl(&placement, &output_root.join(&config.outputs.cpl_file))?;

    run_gerber_export(&config, &board, &output_root)?;
    run_drill_export(&config, &board, &output_root)?;
    run_position_export(&config, &board, &output_root)?;
    if config.step.enabled {
        run_step_export(&config, &board, &output_root)?;
    }

    write_manifest(
        &config,
        &output_root,
        drc_violations,
        real_unconnected,
        ignored_self_zone,
        erc_violations,
    )?;

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

fn write_bom(
    parts: &PartsManifest,
    placement: &PlacementPlan,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }

    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(["Comment", "Designator", "Footprint", "LCSC Part #"])?;
    for part in &parts.selected_parts {
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

fn write_cpl(placement: &PlacementPlan, path: &Path) -> Result<(), Box<dyn Error>> {
    let mut placements = placement.placements.iter().collect::<Vec<_>>();
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
    drc_violations: usize,
    real_unconnected: usize,
    ignored_self_zone: usize,
    erc_violations: usize,
) -> Result<(), Box<dyn Error>> {
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
    writeln!(file, "position: {}", config.outputs.position_file)?;
    if config.step.enabled {
        writeln!(file, "step: {}", config.outputs.step_file)?;
    }
    Ok(())
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
