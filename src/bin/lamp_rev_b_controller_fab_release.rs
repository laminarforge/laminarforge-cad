use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipWriter};

const RELEASE_CONFIG_PATH: &str = "pcb/lamp_rev_b_controller/fab_release.toml";
const RELEASE_MANIFEST_PATH: &str = "pcb/lamp_rev_b_controller/release_manifest.toml";
const DEFAULT_OUTPUT_ROOT: &str = "pcb/lamp_rev_b_controller/fab_release";
const FIXED_ISO_TIMESTAMP: &str = "1980-01-01T00:00:00";
const FIXED_PDF_TIMESTAMP: &str = "D:1980:01:01:00:00:00";
const R25_R26_HAZARD: &str = "MANDATORY DNP: populating either footprint bypasses its heater MOSFET drain-to-source path and can energize the heater whenever VIN_HEATER is present; neither footprint measures current.";

#[derive(Debug, Deserialize)]
struct ReleaseConfig {
    package: PackageConfig,
    toolchain: ToolchainConfig,
    inputs: InputConfig,
    source_policy: SourcePolicy,
    variant: VariantConfig,
    fabrication_order_profile: FabricationOrderProfile,
    portal_gate: PortalGate,
    erc_review: ErcReview,
    assembly: AssemblyConfig,
    procurement: ProcurementConfig,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    name: String,
    ticket: String,
    revision: String,
    variant: String,
    minimum_source_commit: String,
    electrical_evidence_artifact: String,
    electrical_evidence_sha256: String,
}

#[derive(Debug, Deserialize)]
struct ToolchainConfig {
    required_kicad_version: String,
    archive_timestamp: String,
    archive_unix_mode: String,
}

#[derive(Debug, Deserialize)]
struct InputConfig {
    board: String,
    schematic: String,
    project: String,
    design_rules: String,
    contract: String,
    parts: String,
    placement: String,
    pin_nets: String,
    firmware_handoff: String,
    electrical_validation: String,
    routing_seed: String,
    assembly_notes: String,
    fabrication_notes: String,
    manufacturing_bringup: String,
    release_manifest: String,
    electrical_evidence: String,
}

#[derive(Debug, Deserialize)]
struct SourcePolicy {
    canonical_remote_ref: String,
    require_clean_tracked_inputs: bool,
    require_head_equals_remote: bool,
}

#[derive(Debug, Deserialize)]
struct VariantConfig {
    name: String,
    input_voltage_v: f64,
    machine_assembly_side: String,
    first_article_quantity: u32,
    bare_board_overbuild_quantity: String,
    r55_population: String,
    r25_r26_population: String,
    j24_requirement: String,
    panelization_owner: String,
    panelization_input: String,
    panelization_approval: String,
}

#[derive(Debug, Deserialize)]
struct FabricationOrderProfile {
    manufacturer: String,
    base_material: String,
    physical_stackup: String,
    finished_thickness_mm: f64,
    finished_thickness_tolerance: String,
    outer_copper_oz: f64,
    inner_copper_oz: f64,
    surface_finish: String,
    surface_finish_thickness: String,
    soldermask_color: String,
    silkscreen_color: String,
    minimum_annular_ring: String,
    finished_hole_tolerance: String,
    acceptance_class: String,
    impedance_policy: String,
}

#[derive(Debug, Deserialize)]
struct PortalGate {
    preview_approved: bool,
    approval_record: String,
    required_checks: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ErcReview {
    accepted_findings: Vec<AcceptedFinding>,
}

#[derive(Debug, Deserialize)]
struct AcceptedFinding {
    violation_type: String,
    severity: String,
    description: String,
    #[serde(default)]
    item_description_exact: Option<String>,
    #[serde(default)]
    item_description_contains: Option<String>,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct AssemblyConfig {
    manual_part_ids: BTreeSet<String>,
    dnp_part_ids: BTreeSet<String>,
    no_substitution_part_ids: BTreeSet<String>,
}

#[derive(Debug, Deserialize)]
struct ProcurementConfig {
    unresolved_supplier_part_ids: BTreeSet<String>,
    require_explicit_manufacturer: bool,
    require_explicit_manufacturer_part_number: bool,
    require_explicit_supplier: bool,
    require_explicit_supplier_part_number: bool,
}

#[derive(Debug, Deserialize)]
struct ReleaseManifestConfig {
    manifest: ManifestIdentity,
    counts: ExpectedCounts,
    archive: ArchiveConfig,
    gerbers: FileGroup,
    paste: FileGroup,
    review_package: FileGroup,
    release_only: FileGroup,
}

#[derive(Debug, Deserialize)]
struct ManifestIdentity {
    schema_version: u32,
    ticket: String,
    release_prefix: String,
    archive_suffix: String,
    source_commit_file: String,
    checksum_file: String,
    manifest_file: String,
    blocker_file: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectedCounts {
    board_width_mm: f64,
    board_height_mm: f64,
    copper_layers: usize,
    board_footprints: usize,
    bom_designators: usize,
    cpl_designators: usize,
    manual_tht_designators: usize,
    dnp_designators: usize,
    virtual_designators: usize,
    raw_unconnected: usize,
    reviewed_self_zone_unconnected: usize,
    active_unconnected: usize,
    erc_findings: usize,
    erc_blocking: usize,
}

#[derive(Debug, Deserialize)]
struct ArchiveConfig {
    member_order: String,
    member_paths: String,
    timestamp: String,
    unix_mode: String,
    compression: String,
    extended_attributes: bool,
}

#[derive(Debug, Deserialize)]
struct FileGroup {
    #[serde(default)]
    layers: Vec<String>,
    files: Vec<String>,
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
    symbol: String,
    footprint: String,
    lcsc_part: String,
    verification: String,
    #[serde(default)]
    manufacturer: String,
    #[serde(default)]
    manufacturer_part_number: String,
    #[serde(default)]
    supplier: String,
    #[serde(default)]
    supplier_part_number: String,
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
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    side: String,
}

#[derive(Debug, Deserialize)]
struct TestPointPlacement {
    name: String,
}

#[derive(Debug, Deserialize)]
struct DrcReport {
    #[serde(default)]
    violations: Vec<DrcEntry>,
    #[serde(default)]
    unconnected_items: Vec<DrcEntry>,
}

#[derive(Debug, Deserialize)]
struct DrcEntry {
    #[serde(default)]
    items: Vec<DrcItem>,
}

#[derive(Debug, Deserialize)]
struct DrcItem {
    description: String,
    #[serde(default)]
    uuid: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ErcReport {
    sheets: Vec<ErcSheet>,
}

#[derive(Debug, Deserialize)]
struct ErcSheet {
    violations: Vec<ErcEntry>,
}

#[derive(Debug, Deserialize)]
struct ErcEntry {
    description: String,
    severity: String,
    #[serde(rename = "type")]
    violation_type: String,
    items: Vec<ErcItem>,
}

#[derive(Debug, Deserialize)]
struct ErcItem {
    description: String,
}

#[derive(Debug, Clone, Serialize)]
struct ValidationSummary {
    source_commit: String,
    kicad_version: String,
    physical_drc_violations: usize,
    raw_unconnected: usize,
    reviewed_self_zone_unconnected: usize,
    active_unconnected: usize,
    erc_raw_findings: usize,
    erc_accepted_findings: usize,
    erc_blocking_findings: usize,
    bom_designators: usize,
    cpl_designators: usize,
    manual_tht_designators: usize,
    dnp_designators: usize,
    virtual_designators: usize,
    footprint_count: usize,
    bom_cpl_issues: usize,
}

#[derive(Debug, Serialize)]
struct ReleaseManifest {
    schema_version: u32,
    ticket: String,
    board: String,
    revision: String,
    variant: String,
    source_commit: String,
    minimum_source_commit: String,
    kicad_version: String,
    electrical_evidence_artifact: String,
    electrical_evidence_sha256: String,
    release_ready: bool,
    release_blockers: Vec<String>,
    validation: ValidationSummary,
    deterministic_archive: DeterministicArchiveRecord,
    files: Vec<ManifestFile>,
}

#[derive(Debug, Serialize)]
struct DeterministicArchiveRecord {
    order: String,
    paths: String,
    timestamp: String,
    unix_mode: String,
    compression: String,
    extended_attributes: bool,
    two_builds_identical: bool,
}

#[derive(Debug, Serialize)]
struct ManifestFile {
    path: String,
    size_bytes: u64,
    sha256: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("LAMP Rev B manufacturer handoff failed closed:\n{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let release_config: ReleaseConfig = read_toml(&root.join(RELEASE_CONFIG_PATH))?;
    let manifest_config: ReleaseManifestConfig = read_toml(&root.join(RELEASE_MANIFEST_PATH))?;
    let parts: PartsManifest = read_toml(&root.join(&release_config.inputs.parts))?;
    let placement: PlacementPlan = read_toml(&root.join(&release_config.inputs.placement))?;

    validate_static_contract(&root, &release_config, &manifest_config, &parts, &placement)?;
    let source_commit = validate_git_source(&root, &release_config)?;
    let kicad_version = validate_kicad_version(&root, &release_config)?;
    let blockers = collect_release_blockers(&release_config, &parts);

    let short_commit = source_commit
        .get(..7)
        .ok_or("source commit is shorter than seven characters")?;
    let release_name = format!(
        "{}_{}",
        manifest_config.manifest.release_prefix, short_commit
    );
    let output_root = root.join(DEFAULT_OUTPUT_ROOT);
    fs::create_dir_all(&output_root)?;
    let pid = std::process::id();
    let build_a = output_root.join(format!(".{release_name}.build-a-{pid}"));
    let build_b = output_root.join(format!(".{release_name}.build-b-{pid}"));
    let final_dir = output_root.join(&release_name);
    for path in [&build_a, &build_b] {
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
    }

    let build_result = (|| -> Result<ValidationSummary, Box<dyn Error>> {
        let first = build_review_tree(
            &root,
            &build_a,
            &source_commit,
            &kicad_version,
            &release_config,
            &manifest_config,
            &parts,
            &placement,
            &blockers,
        )?;
        let second = build_review_tree(
            &root,
            &build_b,
            &source_commit,
            &kicad_version,
            &release_config,
            &manifest_config,
            &parts,
            &placement,
            &blockers,
        )?;
        if serde_json::to_vec(&first)? != serde_json::to_vec(&second)? {
            return Err("independent builds produced different validation summaries".into());
        }
        compare_trees(&build_a, &build_b)?;
        Ok(first)
    })();

    let validation = match build_result {
        Ok(result) => result,
        Err(error) => {
            let _ = fs::remove_dir_all(&build_a);
            let _ = fs::remove_dir_all(&build_b);
            return Err(error);
        }
    };

    if final_dir.exists() {
        fs::remove_dir_all(&final_dir)?;
    }
    fs::rename(&build_a, &final_dir)?;
    fs::remove_dir_all(&build_b)?;

    println!("Generated byte-reproducible Rev B review tree:");
    println!("  {}", final_dir.display());
    println!("  source commit: {source_commit}");
    println!("  KiCad: {kicad_version}");
    println!(
        "  DRC/connectivity: {}/{} physical/active-open",
        validation.physical_drc_violations, validation.active_unconnected
    );
    println!(
        "  ERC accepted/blocking: {}/{}",
        validation.erc_accepted_findings, validation.erc_blocking_findings
    );
    println!("  BOM/CPL issues: {}", validation.bom_cpl_issues);

    if blockers.is_empty() {
        let archive_name = format!("{release_name}_{}", manifest_config.manifest.archive_suffix);
        let archive_path = output_root.join(archive_name);
        create_tree_zip(&final_dir, &archive_path, Some(&release_name))?;
        println!("  handoff archive: {}", archive_path.display());
        println!("  handoff SHA-256: {}", sha256_file(&archive_path)?);
        Ok(())
    } else {
        println!("  release blockers: {}", blockers.len());
        for blocker in &blockers {
            println!("    - {blocker}");
        }
        Err(format!(
            "{} unresolved manufacturer/order blockers; safe exports and checksums were preserved at {} but no upload directory or outer handoff ZIP was emitted",
            blockers.len(),
            final_dir.display()
        )
        .into())
    }
}

#[allow(clippy::too_many_arguments)]
fn build_review_tree(
    root: &Path,
    stage: &Path,
    source_commit: &str,
    kicad_version: &str,
    release_config: &ReleaseConfig,
    manifest_config: &ReleaseManifestConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    blockers: &[String],
) -> Result<ValidationSummary, Box<dyn Error>> {
    for relative in [
        "fabrication/gerbers",
        "fabrication/drills",
        "assembly/paste",
        "documentation",
        "source",
        "evidence",
    ] {
        fs::create_dir_all(stage.join(relative))?;
    }

    let board = absolute_input(root, &release_config.inputs.board)?;
    let schematic = absolute_input(root, &release_config.inputs.schematic)?;
    let plot_board_dir = stage.join(".plot-source");
    fs::create_dir_all(&plot_board_dir)?;
    let plot_board = plot_board_dir.join("lamp_rev_b_controller.kicad_pcb");
    let plot_project = plot_board_dir.join("lamp_rev_b_controller.kicad_pro");
    write_dnp_plot_board(&board, &plot_board, &["R25", "R26"])?;
    fs::copy(root.join(&release_config.inputs.project), plot_project)?;
    let drc_path = stage.join("evidence/drc.json");
    let erc_path = stage.join("evidence/erc.json");

    run_kicad(
        root,
        &[
            "pcb",
            "drc",
            "--refill-zones",
            "--output",
            path_arg(&drc_path),
            "--format",
            "json",
            "--severity-all",
            path_arg(&board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "sch",
            "erc",
            "--output",
            path_arg(&erc_path),
            "--format",
            "json",
            "--severity-all",
            path_arg(&schematic),
        ],
    )?;
    normalize_json_dates(&drc_path)?;
    normalize_json_dates(&erc_path)?;

    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "gerbers",
            "--output",
            path_arg(&stage.join("fabrication/gerbers")),
            "--layers",
            "F.Cu,In1.Cu,In2.Cu,B.Cu,F.Mask,B.Mask,F.Silkscreen,B.Silkscreen,Edge.Cuts",
            "--precision",
            "6",
            path_arg(&board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "gerbers",
            "--output",
            path_arg(&stage.join("assembly/paste")),
            "--layers",
            "F.Paste,B.Paste",
            "--precision",
            "6",
            path_arg(&board),
        ],
    )?;
    let paste_job = stage.join("assembly/paste/lamp_rev_b_controller-job.gbrjob");
    if paste_job.exists() {
        fs::remove_file(paste_job)?;
    }
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "drill",
            "--output",
            path_arg(&stage.join("fabrication/drills")),
            "--format",
            "excellon",
            "--drill-origin",
            "absolute",
            "--excellon-zeros-format",
            "decimal",
            "--excellon-oval-format",
            "route",
            "--excellon-units",
            "mm",
            "--excellon-separate-th",
            "--generate-map",
            "--map-format",
            "pdf",
            "--generate-report",
            "--report-path",
            path_arg(&stage.join("fabrication/drills/lamp_rev_b_controller-drill-report.rpt")),
            path_arg(&board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "ipcd356",
            "--output",
            path_arg(&stage.join("fabrication/lamp_rev_b_controller.ipc-d-356")),
            path_arg(&board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "stats",
            "--output",
            path_arg(&stage.join("fabrication/lamp_rev_b_controller-board-stats.json")),
            "--format",
            "json",
            "--units",
            "mm",
            path_arg(&board),
        ],
    )?;
    normalize_json_dates(&stage.join("fabrication/lamp_rev_b_controller-board-stats.json"))?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "pdf",
            "--output",
            path_arg(&stage.join("fabrication/lamp_rev_b_controller-fabrication-drawing.pdf")),
            "--layers",
            "F.Fab,B.Fab,Edge.Cuts",
            "--mode-multipage",
            "--black-and-white",
            "--exclude-value",
            "--sketch-pads-on-fab-layers",
            "--scale",
            "0",
            path_arg(&board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "pdf",
            "--output",
            path_arg(&stage.join("assembly/lamp_rev_b_controller-assembly-top.pdf")),
            "--layers",
            "F.Fab,Edge.Cuts",
            "--mode-single",
            "--black-and-white",
            "--exclude-value",
            "--sketch-pads-on-fab-layers",
            "--crossout-DNP-footprints-on-fab-layers",
            "--scale",
            "0",
            path_arg(&plot_board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "pdf",
            "--output",
            path_arg(&stage.join("assembly/lamp_rev_b_controller-assembly-bottom.pdf")),
            "--layers",
            "B.Fab,Edge.Cuts",
            "--mode-single",
            "--mirror",
            "--black-and-white",
            "--exclude-value",
            "--sketch-pads-on-fab-layers",
            "--crossout-DNP-footprints-on-fab-layers",
            "--scale",
            "0",
            path_arg(&plot_board),
        ],
    )?;
    run_kicad(
        root,
        &[
            "sch",
            "export",
            "pdf",
            "--output",
            path_arg(&stage.join("documentation/lamp_rev_b_controller-schematic.pdf")),
            "--black-and-white",
            "--exclude-pdf-metadata",
            path_arg(&schematic),
        ],
    )?;
    run_kicad(
        root,
        &[
            "pcb",
            "export",
            "step",
            "--output",
            path_arg(&stage.join("documentation/lamp_rev_b_controller-board.step")),
            "--force",
            "--board-only",
            path_arg(&board),
        ],
    )?;
    for (side, file) in [
        ("top", "lamp_rev_b_controller-board-top.png"),
        ("bottom", "lamp_rev_b_controller-board-bottom.png"),
    ] {
        run_kicad(
            root,
            &[
                "pcb",
                "render",
                "--output",
                path_arg(&stage.join("documentation").join(file)),
                "--side",
                side,
                "--width",
                "1600",
                "--height",
                "1200",
                "--background",
                "opaque",
                "--quality",
                "basic",
                path_arg(&board),
            ],
        )?;
    }

    fs::remove_dir_all(&plot_board_dir)?;

    normalize_generated_files(stage)?;
    copy_release_sources(root, stage, release_config)?;
    write_bom_cpl(stage, parts, placement, release_config)?;
    write_seeed_quote_bom(stage, parts, placement, release_config)?;
    let validation = validate_electrical_and_assembly(
        stage,
        source_commit,
        kicad_version,
        release_config,
        manifest_config,
        parts,
        placement,
    )?;
    write_json(&stage.join("evidence/validation-summary.json"), &validation)?;
    fs::write(
        stage.join(&manifest_config.manifest.source_commit_file),
        format!("{source_commit}\n"),
    )?;
    write_release_readme(
        &stage.join("README.md"),
        source_commit,
        kicad_version,
        release_config,
        blockers,
    )?;
    write_blockers(
        &stage.join(&manifest_config.manifest.blocker_file),
        blockers,
        release_config,
    )?;

    if blockers.is_empty() {
        create_upload_units(stage, manifest_config)?;
    }

    let files_before_manifest = collect_files(stage)?;
    let manifest_files = files_before_manifest
        .iter()
        .map(|relative| manifest_file(stage, relative))
        .collect::<Result<Vec<_>, _>>()?;
    let release_manifest = ReleaseManifest {
        schema_version: manifest_config.manifest.schema_version,
        ticket: release_config.package.ticket.clone(),
        board: "lamp_rev_b_controller".to_string(),
        revision: release_config.package.revision.clone(),
        variant: release_config.variant.name.clone(),
        source_commit: source_commit.to_string(),
        minimum_source_commit: release_config.package.minimum_source_commit.clone(),
        kicad_version: kicad_version.to_string(),
        electrical_evidence_artifact: release_config.package.electrical_evidence_artifact.clone(),
        electrical_evidence_sha256: release_config.package.electrical_evidence_sha256.clone(),
        release_ready: blockers.is_empty(),
        release_blockers: blockers.to_vec(),
        validation: validation.clone(),
        deterministic_archive: DeterministicArchiveRecord {
            order: manifest_config.archive.member_order.clone(),
            paths: manifest_config.archive.member_paths.clone(),
            timestamp: manifest_config.archive.timestamp.clone(),
            unix_mode: manifest_config.archive.unix_mode.clone(),
            compression: manifest_config.archive.compression.clone(),
            extended_attributes: manifest_config.archive.extended_attributes,
            two_builds_identical: true,
        },
        files: manifest_files,
    };
    write_json(
        &stage.join(&manifest_config.manifest.manifest_file),
        &release_manifest,
    )?;
    write_checksums(stage, &stage.join(&manifest_config.manifest.checksum_file))?;
    validate_exact_allowlist(stage, manifest_config, blockers.is_empty())?;
    validate_checksums(stage, &manifest_config.manifest.checksum_file)?;
    Ok(validation)
}

fn validate_static_contract(
    root: &Path,
    release: &ReleaseConfig,
    manifest: &ReleaseManifestConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
) -> Result<(), Box<dyn Error>> {
    let mut errors = Vec::new();
    if release.package.name != "lamp_rev_b_controller_fab_release" {
        errors.push("release package name must be lamp_rev_b_controller_fab_release".to_string());
    }
    if release.package.ticket != "T-E36FA2C2" || manifest.manifest.ticket != "T-E36FA2C2" {
        errors.push("release package and manifest must name ticket T-E36FA2C2".to_string());
    }
    if release.package.variant != release.variant.name
        || release.variant.name != "rev_b_12v_first_article"
    {
        errors.push("release variant must be rev_b_12v_first_article everywhere".to_string());
    }
    if release.variant.input_voltage_v != 12.0 {
        errors.push("first-article variant must be locked to 12 V".to_string());
    }
    if release.variant.machine_assembly_side != "top" {
        errors.push("first-article machine assembly side must be top".to_string());
    }
    if release.variant.first_article_quantity != 5 {
        errors.push("first-article assembled quantity must remain 5".to_string());
    }
    if !release.variant.r55_population.contains("POPULATE")
        || !release.variant.r55_population.contains("12 V")
    {
        errors.push("R55 must be explicitly populated for the locked 12 V variant".to_string());
    }
    if !release.variant.r25_r26_population.contains("MANDATORY DNP") {
        errors.push("R25/R26 must be an explicit mandatory-DNP rule".to_string());
    }
    if release.variant.panelization_owner != "fabricator"
        || !release.variant.panelization_input.contains("single-up")
        || !release
            .variant
            .panelization_approval
            .contains("portal-preview")
    {
        errors.push("fabricator must own panelization of the released single-up board".to_string());
    }
    if release
        .variant
        .bare_board_overbuild_quantity
        .trim()
        .is_empty()
    {
        errors.push("bare-board overbuild quote instruction must be explicit".to_string());
    }
    if release.fabrication_order_profile.finished_thickness_mm != 1.6
        || release.fabrication_order_profile.outer_copper_oz != 1.0
        || release.fabrication_order_profile.inner_copper_oz != 0.5
        || release.fabrication_order_profile.base_material != "FR-4"
    {
        errors.push("known fabrication baseline must remain FR-4, 1.6 mm, 1/0.5 oz".to_string());
    }
    if manifest.counts.board_width_mm != 118.0
        || manifest.counts.board_height_mm != 94.0
        || manifest.counts.copper_layers != 4
    {
        errors.push("manifest board geometry must remain 118 x 94 mm and four layers".to_string());
    }
    if manifest.gerbers.layers
        != [
            "F.Cu",
            "In1.Cu",
            "In2.Cu",
            "B.Cu",
            "F.Mask",
            "B.Mask",
            "F.Silkscreen",
            "B.Silkscreen",
            "Edge.Cuts",
        ]
    {
        errors.push("Gerber layer allowlist does not match the released board".to_string());
    }
    if release.toolchain.archive_timestamp != "1980-01-01T00:00:00Z"
        || release.toolchain.archive_unix_mode != "0644"
        || manifest.archive.timestamp != release.toolchain.archive_timestamp
        || manifest.archive.unix_mode != release.toolchain.archive_unix_mode
    {
        errors.push("archive normalization contract must use 1980-01-01 and mode 0644".to_string());
    }
    if !manifest.archive.extended_attributes
        && manifest.archive.member_order == "lexicographic"
        && manifest.archive.member_paths.contains("forward-slash")
        && manifest.archive.compression == "deflate"
    {
        // Expected deterministic archive policy.
    } else {
        errors.push("archive policy is not the deterministic release policy".to_string());
    }

    let part_ids = parts
        .selected_parts
        .iter()
        .map(|part| part.id.as_str())
        .collect::<BTreeSet<_>>();
    for (label, ids) in [
        ("manual", &release.assembly.manual_part_ids),
        ("DNP", &release.assembly.dnp_part_ids),
        (
            "no-substitution",
            &release.assembly.no_substitution_part_ids,
        ),
        (
            "unresolved-procurement",
            &release.procurement.unresolved_supplier_part_ids,
        ),
    ] {
        for id in ids {
            if !part_ids.contains(id.as_str()) {
                errors.push(format!("{label} part id {id} does not exist"));
            }
        }
    }
    for required in [
        "mcu_module",
        "usb_c_connector",
        "adc",
        "thermistor_mux",
        "heater_gate_driver",
        "heater_mosfets",
        "heater_output_terminals",
        "heater_cutoff_loop_terminal",
        "led_driver_module",
        "led_current_sense_amp",
        "camera_spi_header",
    ] {
        if !release.assembly.no_substitution_part_ids.contains(required) {
            errors.push(format!("no-substitution coverage is missing {required}"));
        }
    }
    let references = placement
        .placements
        .iter()
        .map(|item| item.reference.as_str())
        .collect::<BTreeSet<_>>();
    for required in ["R25", "R26", "R55", "J24"] {
        if !references.contains(required) {
            errors.push(format!("locked population reference {required} is missing"));
        }
    }
    if placement.test_points.len() != manifest.counts.virtual_designators {
        errors.push(format!(
            "expected {} virtual test points but found {}",
            manifest.counts.virtual_designators,
            placement.test_points.len()
        ));
    }
    if placement
        .test_points
        .iter()
        .any(|test_point| test_point.name.trim().is_empty())
    {
        errors.push("test-point references must be non-empty".to_string());
    }

    for input in required_input_paths(release) {
        if !root.join(&input).is_file() {
            errors.push(format!("required release input is missing: {input}"));
        }
    }
    validate_erc_rules(&release.erc_review.accepted_findings, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n").into())
    }
}

fn validate_git_source(root: &Path, release: &ReleaseConfig) -> Result<String, Box<dyn Error>> {
    let head = git_stdout(root, &["rev-parse", "HEAD"])?;
    if head.len() != 40 || !head.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!("git HEAD is not a full commit SHA: {head}").into());
    }
    let ancestor = Command::new("git")
        .current_dir(root)
        .args([
            "merge-base",
            "--is-ancestor",
            &release.package.minimum_source_commit,
            &head,
        ])
        .status()?;
    if !ancestor.success() {
        return Err(format!(
            "HEAD {head} is not descended from audited minimum commit {}",
            release.package.minimum_source_commit
        )
        .into());
    }
    if release.source_policy.require_clean_tracked_inputs {
        let status = git_stdout(root, &["status", "--porcelain", "--untracked-files=no"])?;
        if !status.is_empty() {
            return Err(format!("tracked release inputs are dirty:\n{status}").into());
        }
    }
    if release.source_policy.require_head_equals_remote {
        let remote = git_stdout(
            root,
            &["rev-parse", &release.source_policy.canonical_remote_ref],
        )?;
        if remote != head {
            return Err(format!(
                "HEAD {head} does not equal canonical {} {remote}",
                release.source_policy.canonical_remote_ref
            )
            .into());
        }
    }
    Ok(head)
}

fn validate_kicad_version(root: &Path, release: &ReleaseConfig) -> Result<String, Box<dyn Error>> {
    let output = run_command(root, "kicad-cli", &["--version"])?;
    let version = String::from_utf8(output.stdout)?.trim().to_string();
    if version != release.toolchain.required_kicad_version {
        return Err(format!(
            "kicad-cli version {version} does not match required {}",
            release.toolchain.required_kicad_version
        )
        .into());
    }
    Ok(version)
}

fn collect_release_blockers(release: &ReleaseConfig, parts: &PartsManifest) -> Vec<String> {
    let mut blockers = Vec::new();
    let profile = &release.fabrication_order_profile;
    for (code, value, requirement) in [
        (
            "MANUFACTURER",
            &profile.manufacturer,
            "selected manufacturer/order profile",
        ),
        (
            "PHYSICAL_STACKUP",
            &profile.physical_stackup,
            "approved dielectric/core/prepreg stackup",
        ),
        (
            "THICKNESS_TOLERANCE",
            &profile.finished_thickness_tolerance,
            "finished-thickness tolerance",
        ),
        (
            "SURFACE_FINISH",
            &profile.surface_finish,
            "final surface-finish type",
        ),
        (
            "SURFACE_FINISH_THICKNESS",
            &profile.surface_finish_thickness,
            "surface-finish thickness specification",
        ),
        (
            "SOLDERMASK_COLOR",
            &profile.soldermask_color,
            "soldermask color",
        ),
        (
            "SILKSCREEN_COLOR",
            &profile.silkscreen_color,
            "silkscreen color",
        ),
        (
            "MINIMUM_ANNULAR_RING",
            &profile.minimum_annular_ring,
            "minimum annular ring",
        ),
        (
            "FINISHED_HOLE_TOLERANCE",
            &profile.finished_hole_tolerance,
            "finished-hole tolerance",
        ),
        (
            "ACCEPTANCE_CLASS",
            &profile.acceptance_class,
            "workmanship/acceptance class and revision",
        ),
        (
            "IMPEDANCE_POLICY",
            &profile.impedance_policy,
            "controlled-impedance requirement or explicit no-controlled-impedance decision",
        ),
    ] {
        if value.trim().is_empty() {
            blockers.push(format!(
                "{code}: record the {requirement}; repository evidence does not define it"
            ));
        }
    }
    if !release.portal_gate.preview_approved
        || release.portal_gate.approval_record.trim().is_empty()
    {
        blockers.push(format!(
            "PORTAL_PREVIEW_APPROVAL: inspect and approve {} and record the approval; no manufacturer was contacted or portal used",
            release.portal_gate.required_checks.join(", ")
        ));
    }

    if release.procurement.require_explicit_manufacturer {
        let missing = parts
            .selected_parts
            .iter()
            .filter(|part| part.manufacturer.trim().is_empty())
            .map(|part| part.id.as_str())
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            blockers.push(format!(
                "PROCUREMENT_MANUFACTURER: explicit manufacturer is missing for part groups: {}",
                missing.join(", ")
            ));
        }
    }
    if release
        .procurement
        .require_explicit_manufacturer_part_number
    {
        let missing = parts
            .selected_parts
            .iter()
            .filter(|part| part.manufacturer_part_number.trim().is_empty())
            .map(|part| part.id.as_str())
            .collect::<Vec<_>>();
        if !missing.is_empty() {
            blockers.push(format!(
                "PROCUREMENT_MPN: explicit manufacturer part number is missing for part groups: {}",
                missing.join(", ")
            ));
        }
    }
    let unresolved = release
        .procurement
        .unresolved_supplier_part_ids
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    if !unresolved.is_empty()
        && (release.procurement.require_explicit_supplier
            || release.procurement.require_explicit_supplier_part_number)
    {
        blockers.push(format!(
            "PROCUREMENT_SUPPLIER: replace mixed/prose/MANUAL supplier fields with explicit supplier and supplier part number for part groups: {}",
            unresolved.join(", ")
        ));
    }
    blockers
}

fn write_bom_cpl(
    stage: &Path,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    release: &ReleaseConfig,
) -> Result<(), Box<dyn Error>> {
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }
    for placements in by_part.values_mut() {
        placements.sort_by_key(|item| reference_order(&item.reference));
    }

    let bom_path = stage.join("assembly/lamp_rev_b_controller-bom.csv");
    let mut bom = csv::Writer::from_path(&bom_path)?;
    bom.write_record([
        "Line",
        "Quantity",
        "Designators",
        "Manufacturer",
        "Manufacturer Part Number",
        "Description",
        "Footprint",
        "Supplier",
        "Supplier Part Number",
        "Side",
        "Population",
        "Variant",
        "No Substitution",
        "Procurement Status",
        "Notes",
    ])?;
    for (index, part) in parts.selected_parts.iter().enumerate() {
        let placements = by_part
            .get(part.id.as_str())
            .ok_or_else(|| format!("part group {} has no placements", part.id))?;
        let references = placements
            .iter()
            .map(|item| item.reference.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let population = population(part, release);
        let (derived_supplier, derived_supplier_part) = supplier_fields(&part.lcsc_part);
        let supplier = if part.supplier.trim().is_empty() {
            derived_supplier
        } else {
            part.supplier.clone()
        };
        let supplier_part = if part.supplier_part_number.trim().is_empty() {
            derived_supplier_part
        } else {
            part.supplier_part_number.clone()
        };
        let procurement_status = if part.manufacturer.trim().is_empty()
            || part.manufacturer_part_number.trim().is_empty()
            || release
                .procurement
                .unresolved_supplier_part_ids
                .contains(&part.id)
        {
            "BLOCKED"
        } else {
            "LOCKED"
        };
        let notes = if part.id == "heater_low_bypass_dnp" {
            R25_R26_HAZARD.to_string()
        } else if part.id == "vdrv_12v_feed_link" {
            release.variant.r55_population.clone()
        } else if part.id == "heater_cutoff_loop_terminal" {
            release.variant.j24_requirement.clone()
        } else {
            part.verification.clone()
        };
        bom.write_record([
            (index + 1).to_string(),
            placements.len().to_string(),
            references,
            part.manufacturer.clone(),
            part.manufacturer_part_number.clone(),
            part.value.clone(),
            part.footprint.clone(),
            supplier,
            supplier_part,
            "Top".to_string(),
            population.to_string(),
            release.variant.name.clone(),
            if release.assembly.no_substitution_part_ids.contains(&part.id) {
                "YES".to_string()
            } else {
                "NO".to_string()
            },
            procurement_status.to_string(),
            notes,
        ])?;
    }
    bom.flush()?;

    let cpl_path = stage.join("assembly/lamp_rev_b_controller-cpl.csv");
    let mut cpl = csv::Writer::from_path(&cpl_path)?;
    cpl.write_record([
        "Designator",
        "Center X (mm)",
        "Center Y (mm)",
        "Rotation (deg CCW, top view)",
        "Side",
        "Footprint",
        "Variant",
    ])?;
    let part_by_id = parts
        .selected_parts
        .iter()
        .map(|part| (part.id.as_str(), part))
        .collect::<BTreeMap<_, _>>();
    let mut machine = placement
        .placements
        .iter()
        .filter(|item| {
            !release.assembly.manual_part_ids.contains(&item.part_id)
                && !release.assembly.dnp_part_ids.contains(&item.part_id)
        })
        .collect::<Vec<_>>();
    machine.sort_by_key(|item| reference_order(&item.reference));
    for item in machine {
        let part = part_by_id
            .get(item.part_id.as_str())
            .ok_or_else(|| format!("placement {} has unknown part id", item.reference))?;
        cpl.write_record([
            item.reference.clone(),
            format!("{:.3}", item.x_mm),
            format!("{:.3}", item.y_mm),
            format!("{:.3}", normalize_rotation(item.rotation_deg)),
            "Top".to_string(),
            part.footprint.clone(),
            release.variant.name.clone(),
        ])?;
    }
    cpl.flush()?;

    let mut dnp = csv::Writer::from_path(stage.join("assembly/lamp_rev_b_controller-dnp.csv"))?;
    dnp.write_record([
        "Designator",
        "Part Group",
        "Variant",
        "Mandatory Rule",
        "Hazard",
    ])?;
    let mut dnp_items = placement
        .placements
        .iter()
        .filter(|item| release.assembly.dnp_part_ids.contains(&item.part_id))
        .collect::<Vec<_>>();
    dnp_items.sort_by_key(|item| reference_order(&item.reference));
    for item in dnp_items {
        dnp.write_record([
            item.reference.as_str(),
            item.part_id.as_str(),
            release.variant.name.as_str(),
            "DO NOT POPULATE",
            R25_R26_HAZARD,
        ])?;
    }
    dnp.flush()?;

    let mut no_sub =
        csv::Writer::from_path(stage.join("assembly/lamp_rev_b_controller-no-substitution.csv"))?;
    no_sub.write_record([
        "Part Group",
        "Designators",
        "Variant",
        "Policy",
        "Change Control",
    ])?;
    for part_id in &release.assembly.no_substitution_part_ids {
        let placements = by_part
            .get(part_id.as_str())
            .ok_or_else(|| format!("no-substitution part group {part_id} has no placements"))?;
        no_sub.write_record([
            part_id.as_str(),
            placements
                .iter()
                .map(|item| item.reference.as_str())
                .collect::<Vec<_>>()
                .join(",")
                .as_str(),
            release.variant.name.as_str(),
            "NO SUBSTITUTION WITHOUT WRITTEN ENGINEERING APPROVAL",
            "Update controlled source/BOM/manifest, rerun all gates, and issue a new archive checksum",
        ])?;
    }
    no_sub.flush()?;
    Ok(())
}

fn write_seeed_quote_bom(
    stage: &Path,
    parts: &PartsManifest,
    placement: &PlacementPlan,
    release: &ReleaseConfig,
) -> Result<(), Box<dyn Error>> {
    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    for item in &placement.placements {
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }
    for placements in by_part.values_mut() {
        placements.sort_by_key(|item| reference_order(&item.reference));
    }

    let path = stage.join("assembly/lamp_rev_b_controller-seeed-bom.csv");
    let mut bom = csv::Writer::from_path(path)?;
    bom.write_record(["Designator", "MPN", "Qty", "Link"])?;
    for part in &parts.selected_parts {
        if release.assembly.dnp_part_ids.contains(&part.id) {
            continue;
        }
        let placements = by_part
            .get(part.id.as_str())
            .ok_or_else(|| format!("part group {} has no placements", part.id))?;
        let references = placements
            .iter()
            .map(|item| item.reference.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let mpn = if part.manufacturer_part_number.trim().is_empty() {
            part.symbol.trim()
        } else {
            part.manufacturer_part_number.trim()
        };
        if mpn.is_empty()
            || mpn.starts_with("HEADER_")
            || matches!(mpn, "FB0805" | "JUMPER_0805_0R")
        {
            return Err(format!(
                "Seeed quote BOM requires an exact MPN for populated part group {}",
                part.id
            )
            .into());
        }
        let link = match supplier_fields(&part.lcsc_part) {
            (supplier, supplier_part) if supplier == "LCSC" => {
                format!("https://www.lcsc.com/product-detail/{supplier_part}.html")
            }
            _ => String::new(),
        };
        bom.write_record([
            references,
            mpn.to_string(),
            placements.len().to_string(),
            link,
        ])?;
    }
    bom.flush()?;
    Ok(())
}

fn validate_electrical_and_assembly(
    stage: &Path,
    source_commit: &str,
    kicad_version: &str,
    release: &ReleaseConfig,
    manifest: &ReleaseManifestConfig,
    parts: &PartsManifest,
    placement: &PlacementPlan,
) -> Result<ValidationSummary, Box<dyn Error>> {
    let drc: DrcReport =
        serde_json::from_str(&fs::read_to_string(stage.join("evidence/drc.json"))?)?;
    let active_unconnected = drc
        .unconnected_items
        .iter()
        .filter(|entry| !is_self_zone_unconnected(entry))
        .count();
    let ignored_self_zone = drc.unconnected_items.len() - active_unconnected;
    let erc: ErcReport =
        serde_json::from_str(&fs::read_to_string(stage.join("evidence/erc.json"))?)?;
    let erc_entries = erc
        .sheets
        .iter()
        .flat_map(|sheet| sheet.violations.iter())
        .collect::<Vec<_>>();
    let accepted = erc_entries
        .iter()
        .filter(|entry| accepted_rule(entry, &release.erc_review.accepted_findings).is_some())
        .count();
    let blocking = erc_entries.len() - accepted;

    let mut by_part: BTreeMap<&str, Vec<&Placement>> = BTreeMap::new();
    let mut unique_refs = BTreeSet::new();
    let mut issues = Vec::new();
    for item in &placement.placements {
        if !unique_refs.insert(item.reference.as_str()) {
            issues.push(format!("duplicate placement reference {}", item.reference));
        }
        if item.side != "top" {
            issues.push(format!("placement {} is not top-side", item.reference));
        }
        if !item.x_mm.is_finite() || !item.y_mm.is_finite() || !item.rotation_deg.is_finite() {
            issues.push(format!(
                "placement {} has non-finite CPL data",
                item.reference
            ));
        }
        if item.x_mm < 0.0
            || item.x_mm > manifest.counts.board_width_mm
            || item.y_mm < -6.0
            || item.y_mm > 88.0
        {
            issues.push(format!(
                "placement {} is outside the board outline",
                item.reference
            ));
        }
        by_part.entry(item.part_id.as_str()).or_default().push(item);
    }
    for part in &parts.selected_parts {
        let count = by_part.get(part.id.as_str()).map(Vec::len).unwrap_or(0);
        if count != part.quantity as usize {
            issues.push(format!(
                "part group {} expects {} placements but has {}",
                part.id, part.quantity, count
            ));
        }
    }
    let manual = placement
        .placements
        .iter()
        .filter(|item| release.assembly.manual_part_ids.contains(&item.part_id))
        .count();
    let dnp = placement
        .placements
        .iter()
        .filter(|item| release.assembly.dnp_part_ids.contains(&item.part_id))
        .count();
    let cpl = placement.placements.len() - manual - dnp;
    for forbidden in ["R25", "R26"] {
        let item = placement
            .placements
            .iter()
            .find(|item| item.reference == forbidden)
            .ok_or_else(|| format!("{forbidden} is missing"))?;
        if !release.assembly.dnp_part_ids.contains(&item.part_id) {
            issues.push(format!("{forbidden} is not classified mandatory DNP"));
        }
    }
    let r55 = placement
        .placements
        .iter()
        .find(|item| item.reference == "R55")
        .ok_or("R55 is missing")?;
    if release.assembly.dnp_part_ids.contains(&r55.part_id) {
        issues.push("R55 is DNP in the locked 12 V variant".to_string());
    }
    let j24 = placement
        .placements
        .iter()
        .find(|item| item.reference == "J24")
        .ok_or("J24 is missing")?;
    if !release.assembly.manual_part_ids.contains(&j24.part_id) {
        issues.push("J24 must be manual/THT".to_string());
    }

    let footprint_count = count_token(
        &fs::read_to_string(stage.join("source/lamp_rev_b_controller.kicad_pcb"))?,
        "(footprint ",
    );
    for (label, actual, expected) in [
        ("physical DRC violations", drc.violations.len(), 0),
        (
            "raw unconnected",
            drc.unconnected_items.len(),
            manifest.counts.raw_unconnected,
        ),
        (
            "reviewed self-zone unconnected",
            ignored_self_zone,
            manifest.counts.reviewed_self_zone_unconnected,
        ),
        (
            "active unconnected",
            active_unconnected,
            manifest.counts.active_unconnected,
        ),
        (
            "ERC findings",
            erc_entries.len(),
            manifest.counts.erc_findings,
        ),
        ("ERC blocking", blocking, manifest.counts.erc_blocking),
        (
            "BOM designators",
            placement.placements.len(),
            manifest.counts.bom_designators,
        ),
        ("CPL designators", cpl, manifest.counts.cpl_designators),
        (
            "manual/THT designators",
            manual,
            manifest.counts.manual_tht_designators,
        ),
        ("DNP designators", dnp, manifest.counts.dnp_designators),
        (
            "virtual designators",
            placement.test_points.len(),
            manifest.counts.virtual_designators,
        ),
        (
            "board footprints",
            footprint_count,
            manifest.counts.board_footprints,
        ),
    ] {
        if actual != expected {
            issues.push(format!("{label}: expected {expected}, found {actual}"));
        }
    }
    if accepted + blocking != erc_entries.len() {
        issues.push("ERC accepted/blocking partition is inconsistent".to_string());
    }
    validate_generated_binary_signatures(stage, &mut issues)?;
    validate_expected_nonempty_outputs(stage, manifest, &mut issues)?;

    if !issues.is_empty() {
        return Err(format!("release validation failed:\n{}", issues.join("\n")).into());
    }
    Ok(ValidationSummary {
        source_commit: source_commit.to_string(),
        kicad_version: kicad_version.to_string(),
        physical_drc_violations: drc.violations.len(),
        raw_unconnected: drc.unconnected_items.len(),
        reviewed_self_zone_unconnected: ignored_self_zone,
        active_unconnected,
        erc_raw_findings: erc_entries.len(),
        erc_accepted_findings: accepted,
        erc_blocking_findings: blocking,
        bom_designators: placement.placements.len(),
        cpl_designators: cpl,
        manual_tht_designators: manual,
        dnp_designators: dnp,
        virtual_designators: placement.test_points.len(),
        footprint_count,
        bom_cpl_issues: 0,
    })
}

fn copy_release_sources(
    root: &Path,
    stage: &Path,
    release: &ReleaseConfig,
) -> Result<(), Box<dyn Error>> {
    let copies = [
        (
            &release.inputs.board,
            "source/lamp_rev_b_controller.kicad_pcb",
        ),
        (
            &release.inputs.schematic,
            "source/lamp_rev_b_controller.kicad_sch",
        ),
        (
            &release.inputs.project,
            "source/lamp_rev_b_controller.kicad_pro",
        ),
        (
            &release.inputs.design_rules,
            "source/lamp_rev_b_controller.kicad_dru",
        ),
        (&release.inputs.contract, "source/contract.toml"),
        (&release.inputs.parts, "source/parts.toml"),
        (&release.inputs.placement, "source/placement.toml"),
        (&release.inputs.pin_nets, "source/pin_nets.toml"),
        (&RELEASE_CONFIG_PATH.to_string(), "source/fab_release.toml"),
        (
            &release.inputs.release_manifest,
            "source/release_manifest.toml",
        ),
        (
            &"pcb/lamp_rev_b_controller/fp-lib-table".to_string(),
            "source/fp-lib-table",
        ),
        (
            &"pcb/lamp_rev_b_controller/sym-lib-table".to_string(),
            "source/sym-lib-table",
        ),
        (
            &release.inputs.assembly_notes,
            "assembly/lamp_rev_b_controller-assembly-notes.md",
        ),
        (
            &release.inputs.fabrication_notes,
            "fabrication/lamp_rev_b_controller-fabrication-notes.md",
        ),
        (
            &release.inputs.manufacturing_bringup,
            "documentation/manufacturing_bringup.md",
        ),
        (
            &release.inputs.firmware_handoff,
            "documentation/firmware_handoff.toml",
        ),
        (
            &release.inputs.electrical_evidence,
            "evidence/A-9F7C4851.md",
        ),
    ];
    for (source, destination) in copies {
        fs::copy(root.join(source), stage.join(destination))?;
    }
    Ok(())
}

fn create_upload_units(
    stage: &Path,
    manifest: &ReleaseManifestConfig,
) -> Result<(), Box<dyn Error>> {
    let upload = stage.join("upload");
    fs::create_dir_all(&upload)?;
    let gerber_files = manifest
        .gerbers
        .files
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    create_zip_from_paths(
        stage,
        &upload.join("lamp_rev_b_controller-gerbers.zip"),
        &gerber_files,
        None,
    )?;
    for (source, destination) in [
        (
            "assembly/lamp_rev_b_controller-bom.csv",
            "upload/lamp_rev_b_controller-bom.csv",
        ),
        (
            "assembly/lamp_rev_b_controller-cpl.csv",
            "upload/lamp_rev_b_controller-cpl.csv",
        ),
        (
            "assembly/lamp_rev_b_controller-assembly-notes.md",
            "upload/lamp_rev_b_controller-assembly-notes.md",
        ),
    ] {
        fs::copy(stage.join(source), stage.join(destination))?;
    }
    Ok(())
}

fn write_release_readme(
    path: &Path,
    source_commit: &str,
    kicad_version: &str,
    release: &ReleaseConfig,
    blockers: &[String],
) -> Result<(), Box<dyn Error>> {
    let state = if blockers.is_empty() {
        "RELEASE READY"
    } else {
        "ENGINEERING REVIEW ONLY — RELEASE BLOCKED"
    };
    let text = format!(
        "# LAMP Rev B Controller Manufacturer Handoff\n\nState: **{state}**  \nTicket: `{}`  \nSource commit: `{source_commit}`  \nKiCad: `{kicad_version}`  \nVariant: `{}` (12 V, R55 populated, R25/R26 mandatory DNP)\n\nThis tree is generated twice and promoted only when every staged byte matches. `SHA256SUMS` covers every file except itself. `MANIFEST.json` binds the checks and source commit.\n\n{}\n\nPanelization is owned by the fabricator from the released single-up board. No panel or portal import is approved by this package alone. Never order from a blocked review tree.\n",
        release.package.ticket,
        release.variant.name,
        if blockers.is_empty() {
            "The `upload/` directory contains the released upload units. The outer normalized ZIP is created beside this directory."
        } else {
            "No `upload/` directory and no outer handoff ZIP were emitted. Resolve every item in `RELEASE_BLOCKERS.md`, update controlled sources, and rerun."
        }
    );
    fs::write(path, text)?;
    Ok(())
}

fn write_blockers(
    path: &Path,
    blockers: &[String],
    release: &ReleaseConfig,
) -> Result<(), Box<dyn Error>> {
    let mut text = format!(
        "# Release Blockers\n\nTicket: `{}`  \nVariant: `{}`\n\n",
        release.package.ticket, release.variant.name
    );
    if blockers.is_empty() {
        text.push_str("No release blockers remain.\n");
    } else {
        text.push_str("The generator completed safe exports and validation but intentionally withheld all upload units and the outer handoff ZIP. Resolve these controlled inputs:\n\n");
        for blocker in blockers {
            text.push_str(&format!("- {blocker}\n"));
        }
        text.push_str("\nNo manufacturer was contacted, no portal was opened, and no requirement above is presented as vendor-cited research.\n");
    }
    fs::write(path, text)?;
    Ok(())
}

fn write_checksums(stage: &Path, path: &Path) -> Result<(), Box<dyn Error>> {
    let checksum_relative = relative_slash(stage, path)?;
    let mut lines = Vec::new();
    for relative in collect_files(stage)? {
        let relative_string = slash_path(&relative);
        if relative_string == checksum_relative {
            continue;
        }
        lines.push(format!(
            "{}  {}",
            sha256_file(&stage.join(&relative))?,
            relative_string
        ));
    }
    fs::write(path, format!("{}\n", lines.join("\n")))?;
    Ok(())
}

fn validate_checksums(stage: &Path, checksum_file: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(stage.join(checksum_file))?;
    for (line_number, line) in content.lines().enumerate() {
        let (expected, relative) = line
            .split_once("  ")
            .ok_or_else(|| format!("invalid checksum line {}", line_number + 1))?;
        let path = stage.join(relative);
        if !path.is_file() {
            return Err(format!("checksum references missing file {relative}").into());
        }
        let actual = sha256_file(&path)?;
        if actual != expected {
            return Err(format!("checksum mismatch for {relative}").into());
        }
    }
    Ok(())
}

fn validate_exact_allowlist(
    stage: &Path,
    manifest: &ReleaseManifestConfig,
    release_ready: bool,
) -> Result<(), Box<dyn Error>> {
    let mut expected = manifest
        .review_package
        .files
        .iter()
        .chain(manifest.gerbers.files.iter())
        .chain(manifest.paste.files.iter())
        .cloned()
        .collect::<BTreeSet<_>>();
    if release_ready {
        expected.extend(manifest.release_only.files.iter().cloned());
    }
    let actual = collect_files(stage)?
        .into_iter()
        .map(|path| slash_path(&path))
        .collect::<BTreeSet<_>>();
    if actual != expected {
        let missing = expected.difference(&actual).cloned().collect::<Vec<_>>();
        let unexpected = actual.difference(&expected).cloned().collect::<Vec<_>>();
        return Err(format!(
            "release allowlist mismatch\nmissing: {}\nunexpected: {}",
            missing.join(", "),
            unexpected.join(", ")
        )
        .into());
    }
    Ok(())
}

fn validate_expected_nonempty_outputs(
    stage: &Path,
    manifest: &ReleaseManifestConfig,
    issues: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for relative in manifest
        .gerbers
        .files
        .iter()
        .chain(manifest.paste.files.iter())
        .chain(manifest.review_package.files.iter())
        .filter(|path| {
            !matches!(
                path.as_str(),
                "README.md"
                    | "SOURCE_COMMIT.txt"
                    | "RELEASE_BLOCKERS.md"
                    | "MANIFEST.json"
                    | "SHA256SUMS"
                    | "evidence/validation-summary.json"
            )
        })
    {
        let path = stage.join(relative);
        match fs::metadata(&path) {
            Ok(metadata) if metadata.is_file() && metadata.len() > 0 => {}
            _ => issues.push(format!("expected non-empty output is missing: {relative}")),
        }
    }
    Ok(())
}

fn validate_generated_binary_signatures(
    stage: &Path,
    issues: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for relative in [
        "documentation/lamp_rev_b_controller-board-top.png",
        "documentation/lamp_rev_b_controller-board-bottom.png",
    ] {
        let bytes = fs::read(stage.join(relative))?;
        if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            issues.push(format!("{relative} is not a PNG"));
        }
    }
    for relative in [
        "fabrication/lamp_rev_b_controller-fabrication-drawing.pdf",
        "assembly/lamp_rev_b_controller-assembly-top.pdf",
        "assembly/lamp_rev_b_controller-assembly-bottom.pdf",
        "documentation/lamp_rev_b_controller-schematic.pdf",
    ] {
        let bytes = fs::read(stage.join(relative))?;
        if !bytes.starts_with(b"%PDF-") {
            issues.push(format!("{relative} is not a PDF"));
        }
    }
    let top_hash = sha256_file(&stage.join("documentation/lamp_rev_b_controller-board-top.png"))?;
    let bottom_hash =
        sha256_file(&stage.join("documentation/lamp_rev_b_controller-board-bottom.png"))?;
    if top_hash == bottom_hash {
        issues.push("top and bottom board renders are identical".to_string());
    }
    Ok(())
}

fn normalize_generated_files(stage: &Path) -> Result<(), Box<dyn Error>> {
    for relative in collect_files(stage)? {
        let path = stage.join(&relative);
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("json") => normalize_json_dates(&path)?,
            Some("gbrjob") => normalize_json_dates(&path)?,
            Some("pdf") => normalize_pdf_creation_date(&path)?,
            Some("step") | Some("gtl") | Some("gbl") | Some("g1") | Some("g2") | Some("gts")
            | Some("gbs") | Some("gto") | Some("gbo") | Some("gm1") | Some("gtp") | Some("gbp")
            | Some("drl") | Some("rpt") => normalize_iso_timestamps(&path)?,
            _ => {}
        }
    }
    Ok(())
}

fn write_dnp_plot_board(
    source: &Path,
    destination: &Path,
    references: &[&str],
) -> Result<(), Box<dyn Error>> {
    let mut board = fs::read_to_string(source)?;
    for reference in references {
        let marker = format!("(property \"Reference\" \"{reference}\"");
        let marker_index = board
            .find(&marker)
            .ok_or_else(|| format!("plot board is missing {reference}"))?;
        let block_start = board[..marker_index]
            .rfind("\n  (footprint ")
            .ok_or_else(|| format!("cannot find footprint start for {reference}"))?;
        let block_end = board[marker_index..]
            .find("\n  (footprint ")
            .map(|offset| marker_index + offset)
            .or_else(|| board.rfind("\n)"))
            .ok_or_else(|| format!("cannot find footprint end for {reference}"))?;
        let block = &board[block_start..block_end];
        if block.contains(" dnp") {
            continue;
        }
        let attr = "(attr smd exclude_from_pos_files)";
        let attr_offset = block
            .find(attr)
            .ok_or_else(|| format!("{reference} has no expected SMT attribute line"))?;
        let absolute = block_start + attr_offset;
        board.replace_range(
            absolute..absolute + attr.len(),
            "(attr smd exclude_from_pos_files dnp)",
        );
    }
    fs::write(destination, board)?;
    Ok(())
}

fn normalize_json_dates(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut value: serde_json::Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    normalize_json_value(&mut value);
    write_json(path, &value)?;
    Ok(())
}

fn normalize_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if (key.eq_ignore_ascii_case("date")
                    || key.eq_ignore_ascii_case("creationdate")
                    || key.eq_ignore_ascii_case("generationdate"))
                    && value.is_string()
                {
                    *value = serde_json::Value::String(FIXED_ISO_TIMESTAMP.to_string());
                } else {
                    normalize_json_value(value);
                }
            }
        }
        serde_json::Value::Array(values) => {
            for value in values {
                normalize_json_value(value);
            }
        }
        _ => {}
    }
}

fn normalize_pdf_creation_date(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut bytes = fs::read(path)?;
    let needle = b"/CreationDate (";
    let mut start = 0;
    while let Some(offset) = find_bytes(&bytes[start..], needle) {
        let value_start = start + offset + needle.len();
        let value_end = bytes[value_start..]
            .iter()
            .position(|byte| *byte == b')')
            .map(|offset| value_start + offset)
            .ok_or("PDF CreationDate is unterminated")?;
        if value_end - value_start != FIXED_PDF_TIMESTAMP.len() {
            return Err(format!("unexpected PDF CreationDate length in {}", path.display()).into());
        }
        bytes[value_start..value_end].copy_from_slice(FIXED_PDF_TIMESTAMP.as_bytes());
        start = value_end;
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn normalize_iso_timestamps(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut text = fs::read_to_string(path)?;
    let bytes = text.as_bytes();
    let mut starts = Vec::new();
    if bytes.len() >= FIXED_ISO_TIMESTAMP.len() {
        for index in 0..=bytes.len() - FIXED_ISO_TIMESTAMP.len() {
            let candidate = &bytes[index..index + FIXED_ISO_TIMESTAMP.len()];
            if looks_like_iso_timestamp(candidate) || looks_like_space_timestamp(candidate) {
                starts.push(index);
            }
        }
    }
    for start in starts.into_iter().rev() {
        let replacement = if text.as_bytes()[start + 10] == b'T' {
            FIXED_ISO_TIMESTAMP
        } else {
            "1980-01-01 00:00:00"
        };
        text.replace_range(start..start + FIXED_ISO_TIMESTAMP.len(), replacement);
    }
    fs::write(path, text)?;
    Ok(())
}

fn looks_like_iso_timestamp(value: &[u8]) -> bool {
    value.len() == 19
        && value[0..4].iter().all(u8::is_ascii_digit)
        && value[4] == b'-'
        && value[5..7].iter().all(u8::is_ascii_digit)
        && value[7] == b'-'
        && value[8..10].iter().all(u8::is_ascii_digit)
        && value[10] == b'T'
        && value[11..13].iter().all(u8::is_ascii_digit)
        && value[13] == b':'
        && value[14..16].iter().all(u8::is_ascii_digit)
        && value[16] == b':'
        && value[17..19].iter().all(u8::is_ascii_digit)
}

fn looks_like_space_timestamp(value: &[u8]) -> bool {
    value.len() == 19
        && value[0..4].iter().all(u8::is_ascii_digit)
        && value[4] == b'-'
        && value[5..7].iter().all(u8::is_ascii_digit)
        && value[7] == b'-'
        && value[8..10].iter().all(u8::is_ascii_digit)
        && value[10] == b' '
        && value[11..13].iter().all(u8::is_ascii_digit)
        && value[13] == b':'
        && value[14..16].iter().all(u8::is_ascii_digit)
        && value[16] == b':'
        && value[17..19].iter().all(u8::is_ascii_digit)
}

fn compare_trees(first: &Path, second: &Path) -> Result<(), Box<dyn Error>> {
    let first_files = tree_hashes(first)?;
    let second_files = tree_hashes(second)?;
    if first_files != second_files {
        let paths = first_files
            .keys()
            .chain(second_files.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        let differences = paths
            .into_iter()
            .filter(|path| first_files.get(path) != second_files.get(path))
            .collect::<Vec<_>>();
        return Err(format!(
            "two independent release builds are not byte-identical: {}",
            differences.join(", ")
        )
        .into());
    }
    Ok(())
}

fn tree_hashes(root: &Path) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    collect_files(root)?
        .into_iter()
        .map(|relative| Ok((slash_path(&relative), sha256_file(&root.join(&relative))?)))
        .collect()
}

fn create_tree_zip(
    source_root: &Path,
    archive: &Path,
    member_root: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let files = collect_files(source_root)?;
    create_zip_from_paths(source_root, archive, &files, member_root)
}

fn create_zip_from_paths(
    source_root: &Path,
    archive: &Path,
    files: &[PathBuf],
    member_root: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let mut files = files.to_vec();
    files.sort_by_key(|path| slash_path(path));
    let output = File::create(archive)?;
    let mut zip = ZipWriter::new(output);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .last_modified_time(DateTime::default())
        .unix_permissions(0o644);
    for relative in files {
        let relative_name = slash_path(&relative);
        let member_name = match member_root {
            Some(root) => format!("{root}/{relative_name}"),
            None => relative_name,
        };
        zip.start_file(member_name, options)?;
        let mut input = File::open(source_root.join(&relative))?;
        std::io::copy(&mut input, &mut zip)?;
    }
    zip.finish()?;
    Ok(())
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    fn visit(root: &Path, current: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
        let mut entries = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let metadata = entry.metadata()?;
            if metadata.is_dir() {
                visit(root, &path, files)?;
            } else if metadata.is_file() {
                files.push(path.strip_prefix(root)?.to_path_buf());
            } else {
                return Err(
                    format!("release tree contains non-file entry {}", path.display()).into(),
                );
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    visit(root, root, &mut files)?;
    files.sort_by_key(|path| slash_path(path));
    Ok(files)
}

fn manifest_file(root: &Path, relative: &Path) -> Result<ManifestFile, Box<dyn Error>> {
    let path = root.join(relative);
    Ok(ManifestFile {
        path: slash_path(relative),
        size_bytes: fs::metadata(&path)?.len(),
        sha256: sha256_file(&path)?,
    })
}

fn sha256_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn Error>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn supplier_fields(source: &str) -> (String, String) {
    let trimmed = source.trim();
    if trimmed.starts_with('C')
        && trimmed.len() > 1
        && trimmed[1..].chars().all(|ch| ch.is_ascii_digit())
    {
        return ("LCSC".to_string(), trimmed.to_string());
    }
    if let Some(part) = trimmed.strip_prefix("DigiKey ") {
        if !part.contains(" or ") && !part.contains(';') {
            return ("DigiKey".to_string(), part.to_string());
        }
    }
    (String::new(), String::new())
}

fn population<'a>(part: &SelectedPart, release: &'a ReleaseConfig) -> &'a str {
    if release.assembly.dnp_part_ids.contains(&part.id) {
        "DNP"
    } else if release.assembly.manual_part_ids.contains(&part.id) {
        "Manual/THT"
    } else {
        "SMT"
    }
}

fn normalize_rotation(rotation: f64) -> f64 {
    rotation.rem_euclid(360.0)
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
    (
        prefix,
        digits.parse::<u32>().unwrap_or_default(),
        reference.to_string(),
    )
}

fn accepted_rule<'a>(
    entry: &ErcEntry,
    rules: &'a [AcceptedFinding],
) -> Option<&'a AcceptedFinding> {
    rules.iter().find(|rule| {
        rule.violation_type == entry.violation_type
            && rule.severity == entry.severity
            && rule.description == entry.description
            && !entry.items.is_empty()
            && entry.items.iter().all(|item| {
                rule.item_description_exact
                    .as_ref()
                    .is_some_and(|expected| item.description == *expected)
                    || rule
                        .item_description_contains
                        .as_ref()
                        .is_some_and(|needle| item.description.contains(needle))
            })
    })
}

fn validate_erc_rules(rules: &[AcceptedFinding], errors: &mut Vec<String>) {
    for rule in rules {
        match (
            rule.item_description_exact.as_ref(),
            rule.item_description_contains.as_ref(),
        ) {
            (Some(_), Some(_)) | (None, None) => errors.push(format!(
                "ERC acceptance rule {} must use exactly one item matcher",
                rule.violation_type
            )),
            _ => {}
        }
        if rule.reason.trim().is_empty() {
            errors.push(format!(
                "ERC acceptance rule {} is missing its reviewed reason",
                rule.violation_type
            ));
        }
    }
}

fn is_self_zone_unconnected(entry: &DrcEntry) -> bool {
    if entry.items.len() != 2 {
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

fn count_token(text: &str, token: &str) -> usize {
    text.matches(token).count()
}

fn required_input_paths(release: &ReleaseConfig) -> Vec<String> {
    vec![
        release.inputs.board.clone(),
        release.inputs.schematic.clone(),
        release.inputs.project.clone(),
        release.inputs.design_rules.clone(),
        release.inputs.contract.clone(),
        release.inputs.parts.clone(),
        release.inputs.placement.clone(),
        release.inputs.pin_nets.clone(),
        release.inputs.firmware_handoff.clone(),
        release.inputs.electrical_validation.clone(),
        release.inputs.routing_seed.clone(),
        release.inputs.assembly_notes.clone(),
        release.inputs.fabrication_notes.clone(),
        release.inputs.manufacturing_bringup.clone(),
        release.inputs.release_manifest.clone(),
        release.inputs.electrical_evidence.clone(),
        RELEASE_CONFIG_PATH.to_string(),
        "pcb/lamp_rev_b_controller/fp-lib-table".to_string(),
        "pcb/lamp_rev_b_controller/sym-lib-table".to_string(),
    ]
}

fn read_toml<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn Error>> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

fn absolute_input(root: &Path, relative: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = root.join(relative);
    if !path.is_file() {
        return Err(format!("required input is missing: {}", path.display()).into());
    }
    Ok(path)
}

fn path_arg(path: &Path) -> &str {
    path.to_str().expect("release paths must be UTF-8")
}

fn run_kicad(root: &Path, args: &[&str]) -> Result<Output, Box<dyn Error>> {
    run_command(root, "kicad-cli", args)
}

fn run_command(root: &Path, program: &str, args: &[&str]) -> Result<Output, Box<dyn Error>> {
    let output = Command::new(program)
        .current_dir(root)
        .args(args)
        .output()?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(format!(
            "command failed: {program} {}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

fn git_stdout(root: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = run_command(root, "git", args)?;
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn relative_slash(root: &Path, path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(slash_path(path.strip_prefix(root)?))
}

fn slash_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplier_fields_only_promotes_unambiguous_repository_evidence() {
        assert_eq!(
            supplier_fields("C37593"),
            ("LCSC".to_string(), "C37593".to_string())
        );
        assert_eq!(
            supplier_fields("DigiKey IRLB3813PBF-ND"),
            ("DigiKey".to_string(), "IRLB3813PBF-ND".to_string())
        );
        assert_eq!(
            supplier_fields("DigiKey TC4427EPA-ND or C636886 SMD alternate"),
            (String::new(), String::new())
        );
        assert_eq!(supplier_fields("MANUAL"), (String::new(), String::new()));
    }

    #[test]
    fn timestamp_normalizer_only_matches_iso_seconds() {
        assert!(looks_like_iso_timestamp(b"2026-07-10T09:31:46"));
        assert!(looks_like_space_timestamp(b"2026-07-10 09:31:46"));
        assert!(!looks_like_iso_timestamp(b"Rev B 2026-07-10 xyz"));
    }

    #[test]
    fn references_sort_naturally() {
        let mut refs = ["R10", "R2", "C1", "R1"];
        refs.sort_by_key(|reference| reference_order(reference));
        assert_eq!(refs, ["C1", "R1", "R2", "R10"]);
    }

    #[test]
    fn rotation_is_normalized() {
        assert_eq!(normalize_rotation(-90.0), 270.0);
        assert_eq!(normalize_rotation(450.0), 90.0);
    }

    #[test]
    fn normalized_zip_is_byte_reproducible() {
        let base = std::env::temp_dir().join(format!(
            "lamp-rev-b-release-zip-test-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("tree/a")).unwrap();
        fs::write(base.join("tree/a/one.txt"), b"one\n").unwrap();
        fs::write(base.join("tree/two.txt"), b"two\n").unwrap();
        create_tree_zip(&base.join("tree"), &base.join("first.zip"), Some("root")).unwrap();
        create_tree_zip(&base.join("tree"), &base.join("second.zip"), Some("root")).unwrap();
        assert_eq!(
            fs::read(base.join("first.zip")).unwrap(),
            fs::read(base.join("second.zip")).unwrap()
        );
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn plot_copy_marks_only_requested_reference_dnp() {
        let base =
            std::env::temp_dir().join(format!("lamp-rev-b-dnp-plot-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let source = base.join("source.kicad_pcb");
        let destination = base.join("plot.kicad_pcb");
        fs::write(
            &source,
            "(kicad_pcb\n  (footprint \"R\"\n    (property \"Reference\" \"R25\")\n    (attr smd exclude_from_pos_files)\n  )\n  (footprint \"R\"\n    (property \"Reference\" \"R26\")\n    (attr smd exclude_from_pos_files)\n  )\n)\n",
        )
        .unwrap();
        write_dnp_plot_board(&source, &destination, &["R25", "R26"]).unwrap();
        let plotted = fs::read_to_string(destination).unwrap();
        assert_eq!(
            plotted
                .matches("(attr smd exclude_from_pos_files dnp)")
                .count(),
            2
        );
        fs::remove_dir_all(base).unwrap();
    }
}
