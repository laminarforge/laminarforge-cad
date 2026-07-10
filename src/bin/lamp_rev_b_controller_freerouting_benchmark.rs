use clap::Parser;
use laminarforge_cad::pcb::ses::{parse_ses, SesData};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use zip::ZipArchive;

const DEFAULT_CONFIG_PATH: &str = "pcb/lamp_rev_b_controller/freerouting_benchmark.toml";
// Specctra coordinates remain expressed directly in the declared base unit.
// `(resolution um 10)` declares precision; Freerouting emits SES coordinates
// at 10 units/um, which the existing SES parser scales back down.
const DSN_INPUT_UNITS_PER_UM: f64 = 1.0;

#[derive(Debug, Parser)]
#[command(about = "Run an isolated, bounded Rev B Freerouting benchmark")]
struct Args {
    #[arg(long, default_value = DEFAULT_CONFIG_PATH)]
    config: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    #[arg(long)]
    pass_budget: u32,
    #[arg(long, default_value = "java")]
    java: OsString,
    #[arg(long, default_value = "kicad-cli")]
    kicad_cli: OsString,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    benchmark: BenchmarkConfig,
    fixture_publication: FixturePublication,
    upstream_evidence: UpstreamEvidence,
}

#[derive(Debug, Deserialize)]
struct BenchmarkConfig {
    schema_version: u32,
    fixture_name: String,
    board_path: PathBuf,
    project_path: PathBuf,
    design_rules_path: PathBuf,
    freerouting_jar_path: PathBuf,
    expected_freerouting_version: String,
    expected_freerouting_revision: String,
    expected_freerouting_jar_sha256: String,
    max_runtime_seconds: u64,
    autorouter_threads: u32,
    optimizer_enabled: bool,
    optimizer_threads: u32,
    optimizer_improvement_threshold_percent: f64,
    board_update_strategy: String,
    item_selection_strategy: String,
    fanout_enabled: bool,
    analytics_disabled: bool,
    default_track_width_mm: f64,
    default_clearance_mm: f64,
    default_via_size_mm: f64,
    default_via_drill_mm: f64,
    random_seed_supported: bool,
    #[serde(default)]
    random_seed: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FixturePublication {
    safe_for_public_upstream: bool,
    rationale: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpstreamEvidence {
    repository: String,
    source_path: String,
    finding: String,
}

#[derive(Debug, Serialize)]
struct BenchmarkResult {
    schema_version: u32,
    run_id: String,
    fixture_name: String,
    source_git_commit: String,
    input: InputIdentity,
    tools: ToolIdentity,
    effective_settings: EffectiveSettings,
    execution: ExecutionResult,
    freerouting_statistics: Option<Value>,
    metrics: BenchmarkMetrics,
    validation: ValidationResult,
    fixture_publication: FixturePublication,
    upstream_evidence: UpstreamEvidence,
}

#[derive(Debug, Serialize)]
struct InputIdentity {
    board_path: String,
    board_sha256: String,
    dsn_sha256: String,
    dsn_stats: DsnStats,
}

#[derive(Debug, Serialize)]
struct ToolIdentity {
    freerouting_version: String,
    freerouting_build_revision: String,
    freerouting_jar_path: String,
    freerouting_jar_sha256: String,
    java_command: String,
    java_version: String,
    kicad_cli_command: String,
    kicad_version: String,
}

#[derive(Debug, Serialize)]
struct EffectiveSettings {
    max_runtime_seconds: u64,
    pass_budget: u32,
    autorouter_threads: u32,
    optimizer_enabled: bool,
    optimizer_threads: u32,
    optimizer_improvement_threshold_percent: f64,
    board_update_strategy: String,
    item_selection_strategy: String,
    fanout_enabled: bool,
    analytics_disabled: bool,
    random_seed_supported: bool,
    random_seed: Option<u64>,
    command_arguments: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ExecutionResult {
    runtime_ms: u128,
    exit_code: i32,
    timed_out: bool,
    observed_autorouter_passes: u32,
    pass_count_verification: PassCountVerification,
    stdout_log: String,
    stderr_log: String,
    ses_output_sha256: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PassCountVerification {
    ExactBudget,
    CompletedBeforeBudget,
}

impl PassCountVerification {
    fn label(&self) -> &'static str {
        match self {
            Self::ExactBudget => "exact_budget",
            Self::CompletedBeforeBudget => "completed_before_budget",
        }
    }
}

#[derive(Debug, Serialize)]
struct BenchmarkMetrics {
    freerouting_routed_connection_count: Option<u64>,
    freerouting_total_completed_connection_count: Option<u64>,
    freerouting_unrouted_connection_count: Option<u64>,
    ses_net_count: usize,
    ses_wire_count: usize,
    ses_segment_count: usize,
    ses_via_count: usize,
    ses_total_trace_length_mm: f64,
}

#[derive(Debug, Serialize)]
struct ValidationResult {
    before_import: DrcCounts,
    after_import: DrcCounts,
    imported_board_path: String,
    post_import_drc_report_path: String,
    canonical_board_modified: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DrcCounts {
    physical_violation_count: usize,
    error_violation_count: usize,
    warning_violation_count: usize,
    raw_unconnected_count: usize,
    real_unconnected_count: usize,
    ignored_self_zone_unconnected_count: usize,
    physical_violations: Vec<DrcViolationSummary>,
}

#[derive(Debug, Clone, Serialize)]
struct DrcViolationSummary {
    severity: String,
    violation_type: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct DrcReport {
    #[serde(default)]
    violations: Vec<Value>,
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

#[derive(Debug, Clone, Serialize)]
struct DsnStats {
    copper_layer_count: usize,
    footprint_count: usize,
    pad_count: usize,
    net_count: usize,
    protected_wire_count: usize,
    protected_via_count: usize,
    plane_count: usize,
}

#[derive(Debug, Clone)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

impl SExpr {
    fn atom(&self) -> Option<&str> {
        match self {
            Self::Atom(value) => Some(value),
            Self::List(_) => None,
        }
    }

    fn list(&self) -> Option<&[SExpr]> {
        match self {
            Self::Atom(_) => None,
            Self::List(values) => Some(values),
        }
    }

    fn tag(&self) -> Option<&str> {
        self.list()?.first()?.atom()
    }

    fn item(&self, index: usize) -> Option<&SExpr> {
        self.list()?.get(index)
    }

    fn atom_at(&self, index: usize) -> Option<&str> {
        self.item(index)?.atom()
    }

    fn direct(&self, tag: &str) -> Option<&SExpr> {
        self.list()?.iter().find(|child| child.tag() == Some(tag))
    }

    fn direct_all<'a>(&'a self, tag: &'a str) -> impl Iterator<Item = &'a SExpr> {
        self.list()
            .into_iter()
            .flatten()
            .filter(move |child| child.tag() == Some(tag))
    }
}

struct SExprParser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> SExprParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            pos: 0,
        }
    }

    fn parse(mut self) -> Result<SExpr, String> {
        self.skip_space();
        let value = self.parse_value()?;
        self.skip_space();
        if self.pos != self.input.len() {
            return Err(format!("unexpected content at byte {}", self.pos));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<SExpr, String> {
        self.skip_space();
        match self.input.get(self.pos) {
            Some(b'(') => self.parse_list(),
            Some(b'\"') => self.parse_quoted(),
            Some(_) => self.parse_atom(),
            None => Err("unexpected end of s-expression".into()),
        }
    }

    fn parse_list(&mut self) -> Result<SExpr, String> {
        self.pos += 1;
        let mut values = Vec::new();
        loop {
            self.skip_space();
            match self.input.get(self.pos) {
                Some(b')') => {
                    self.pos += 1;
                    return Ok(SExpr::List(values));
                }
                Some(_) => values.push(self.parse_value()?),
                None => return Err("unterminated s-expression list".into()),
            }
        }
    }

    fn parse_quoted(&mut self) -> Result<SExpr, String> {
        self.pos += 1;
        let mut value = String::new();
        while let Some(byte) = self.input.get(self.pos).copied() {
            self.pos += 1;
            match byte {
                b'\"' => return Ok(SExpr::Atom(value)),
                b'\\' => {
                    let escaped = self
                        .input
                        .get(self.pos)
                        .copied()
                        .ok_or("unterminated quoted escape")?;
                    self.pos += 1;
                    value.push(escaped as char);
                }
                other => value.push(other as char),
            }
        }
        Err("unterminated quoted string".into())
    }

    fn parse_atom(&mut self) -> Result<SExpr, String> {
        let start = self.pos;
        while let Some(byte) = self.input.get(self.pos) {
            if byte.is_ascii_whitespace() || *byte == b'(' || *byte == b')' {
                break;
            }
            self.pos += 1;
        }
        if start == self.pos {
            return Err(format!("expected atom at byte {}", self.pos));
        }
        let value =
            std::str::from_utf8(&self.input[start..self.pos]).map_err(|error| error.to_string())?;
        Ok(SExpr::Atom(value.to_string()))
    }

    fn skip_space(&mut self) {
        while self
            .input
            .get(self.pos)
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.pos += 1;
        }
    }
}

#[derive(Debug)]
struct PadInfo {
    dsn_number: String,
    shape: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    width_mm: f64,
    height_mm: f64,
    copper_layers: Vec<String>,
    net_id: u32,
}

#[derive(Debug)]
struct FootprintInfo {
    image_name: String,
    reference: String,
    x_mm: f64,
    y_mm: f64,
    rotation_deg: f64,
    front: bool,
    pads: Vec<PadInfo>,
}

#[derive(Debug)]
struct SegmentInfo {
    start: (f64, f64),
    end: (f64, f64),
    width_mm: f64,
    layer: String,
    net_id: u32,
}

#[derive(Debug)]
struct ViaInfo {
    at: (f64, f64),
    size_mm: f64,
    drill_mm: f64,
    net_id: u32,
}

#[derive(Debug)]
struct PlaneInfo {
    net_id: u32,
    layer: String,
    points: Vec<(f64, f64)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let root = std::env::current_dir()?;
    let config_path = resolve(&root, &args.config);
    let config: ConfigFile = toml::from_str(&fs::read_to_string(&config_path)?)?;
    validate_config(&config.benchmark)?;
    validate_pass_budget(args.pass_budget)?;

    let output_dir = resolve(&root, &args.output_dir);
    prepare_output_dir(&output_dir)?;

    let board_path = resolve(&root, &config.benchmark.board_path);
    let project_path = resolve(&root, &config.benchmark.project_path);
    let design_rules_path = resolve(&root, &config.benchmark.design_rules_path);
    let jar_path = resolve(&root, &config.benchmark.freerouting_jar_path);

    let board_before = fs::read(&board_path)?;
    let board_sha256 = sha256_bytes(&board_before);
    let board_text = String::from_utf8(board_before.clone())?;
    let parsed_board = SExprParser::new(&board_text).parse()?;
    let (dsn, dsn_stats) = export_dsn(&parsed_board, &config.benchmark)?;
    let dsn_path = output_dir.join("input.dsn");
    fs::write(&dsn_path, dsn.as_bytes())?;
    let dsn_sha256 = sha256_bytes(dsn.as_bytes());

    let jar_sha256 = sha256_file(&jar_path)?;
    if jar_sha256 != config.benchmark.expected_freerouting_jar_sha256 {
        return Err(format!(
            "Freerouting JAR SHA-256 mismatch: expected {}, got {}",
            config.benchmark.expected_freerouting_jar_sha256, jar_sha256
        )
        .into());
    }
    let manifest = read_jar_manifest(&jar_path)?;
    let build_revision = manifest
        .get("Build-Revision")
        .ok_or("Freerouting JAR manifest has no Build-Revision")?;
    if build_revision != &config.benchmark.expected_freerouting_revision {
        return Err(format!(
            "Freerouting build revision mismatch: expected {}, got {}",
            config.benchmark.expected_freerouting_revision, build_revision
        )
        .into());
    }

    let before_bundle = output_dir.join("validation_before");
    let before_board = copy_validation_bundle(
        &board_path,
        &project_path,
        &design_rules_path,
        &before_bundle,
        &board_text,
    )?;
    let before_report_path = before_bundle.join("drc.json");
    let before_counts = run_kicad_drc(&args.kicad_cli, &before_board, &before_report_path)?;

    let java_version = command_version(&args.java, &[OsString::from("-version")])?;
    let kicad_version = command_version(&args.kicad_cli, &[OsString::from("version")])?;
    let ses_path = output_dir.join("output.ses");
    let user_data_path = output_dir.join("freerouting_user_data");
    fs::create_dir_all(&user_data_path)?;
    let freerouting_args = freerouting_arguments(
        &jar_path,
        &user_data_path,
        &dsn_path,
        &ses_path,
        &config.benchmark,
        args.pass_budget,
    );
    let process = run_bounded(
        &args.java,
        &freerouting_args,
        &output_dir,
        config.benchmark.max_runtime_seconds,
    )?;
    let stdout = fs::read_to_string(output_dir.join("freerouting.stdout.log"))?;
    let stderr = fs::read_to_string(output_dir.join("freerouting.stderr.log"))?;
    let version_marker = format!(
        "Freerouting v{}",
        config.benchmark.expected_freerouting_version
    );
    if !stdout.contains(&version_marker) && !stderr.contains(&version_marker) {
        return Err(format!(
            "Freerouting runtime did not report pinned version {}",
            config.benchmark.expected_freerouting_version
        )
        .into());
    }
    if process.timed_out {
        return Err(format!(
            "Freerouting exceeded the {} second benchmark bound; see {}",
            config.benchmark.max_runtime_seconds,
            output_dir.join("freerouting.stderr.log").display()
        )
        .into());
    }
    if process.exit_code != 0 {
        return Err(format!(
            "Freerouting failed with exit code {}; see logs in {}",
            process.exit_code,
            output_dir.display()
        )
        .into());
    }
    if !ses_path.is_file() || fs::metadata(&ses_path)?.len() == 0 {
        return Err("Freerouting exited successfully without producing an SES file".into());
    }

    let observed_autorouter_passes = observed_autorouter_passes(&stdout);
    let freerouting_statistics = last_json_object(&stdout);
    let ses_sha256 = sha256_file(&ses_path)?;
    let ses = parse_ses(&ses_path);
    let metrics = ses_metrics(
        &ses,
        freerouting_statistics.as_ref(),
        before_counts.real_unconnected_count,
    );
    let pass_count_verification = verify_observed_autorouter_passes(
        args.pass_budget,
        observed_autorouter_passes,
        metrics.freerouting_unrouted_connection_count,
    )?;
    let net_ids = board_net_ids(&parsed_board)?;
    let imported = import_ses(&board_text, &ses, &net_ids)?;

    let after_bundle = output_dir.join("validation_after");
    let after_board = copy_validation_bundle(
        &board_path,
        &project_path,
        &design_rules_path,
        &after_bundle,
        &imported,
    )?;
    let after_report_path = after_bundle.join("drc.json");
    let after_counts = run_kicad_drc(&args.kicad_cli, &after_board, &after_report_path)?;
    let canonical_board_modified = sha256_file(&board_path)? != board_sha256;
    if canonical_board_modified {
        return Err("canonical Rev B board changed during isolated benchmark".into());
    }

    let run_id = format!(
        "freerouting-{}-{}-{}",
        &board_sha256[..12],
        &dsn_sha256[..12],
        &ses_sha256[..12]
    );
    let source_git_commit = git_commit(&root)?;
    let command_arguments = freerouting_args
        .iter()
        .map(|arg| {
            arg.to_string_lossy()
                .replace(&root.display().to_string(), "$REPO")
        })
        .collect::<Vec<_>>();
    let result = BenchmarkResult {
        schema_version: config.benchmark.schema_version,
        run_id,
        fixture_name: config.benchmark.fixture_name.clone(),
        source_git_commit,
        input: InputIdentity {
            board_path: config.benchmark.board_path.display().to_string(),
            board_sha256,
            dsn_sha256,
            dsn_stats,
        },
        tools: ToolIdentity {
            freerouting_version: config.benchmark.expected_freerouting_version.clone(),
            freerouting_build_revision: build_revision.clone(),
            freerouting_jar_path: config.benchmark.freerouting_jar_path.display().to_string(),
            freerouting_jar_sha256: jar_sha256,
            java_command: args.java.to_string_lossy().to_string(),
            java_version,
            kicad_cli_command: args.kicad_cli.to_string_lossy().to_string(),
            kicad_version,
        },
        effective_settings: EffectiveSettings {
            max_runtime_seconds: config.benchmark.max_runtime_seconds,
            pass_budget: args.pass_budget,
            autorouter_threads: config.benchmark.autorouter_threads,
            optimizer_enabled: config.benchmark.optimizer_enabled,
            optimizer_threads: config.benchmark.optimizer_threads,
            optimizer_improvement_threshold_percent: config
                .benchmark
                .optimizer_improvement_threshold_percent,
            board_update_strategy: config.benchmark.board_update_strategy.clone(),
            item_selection_strategy: config.benchmark.item_selection_strategy.clone(),
            fanout_enabled: config.benchmark.fanout_enabled,
            analytics_disabled: config.benchmark.analytics_disabled,
            random_seed_supported: config.benchmark.random_seed_supported,
            random_seed: config.benchmark.random_seed,
            command_arguments,
        },
        execution: ExecutionResult {
            runtime_ms: process.runtime_ms,
            exit_code: process.exit_code,
            timed_out: process.timed_out,
            observed_autorouter_passes,
            pass_count_verification,
            stdout_log: "freerouting.stdout.log".into(),
            stderr_log: "freerouting.stderr.log".into(),
            ses_output_sha256: ses_sha256,
        },
        freerouting_statistics,
        metrics,
        validation: ValidationResult {
            before_import: before_counts,
            after_import: after_counts,
            imported_board_path: relative_to(&output_dir, &after_board),
            post_import_drc_report_path: relative_to(&output_dir, &after_report_path),
            canonical_board_modified,
        },
        fixture_publication: config.fixture_publication,
        upstream_evidence: config.upstream_evidence,
    };

    let result_json = serde_json::to_string_pretty(&result)? + "\n";
    fs::write(output_dir.join("result.json"), result_json)?;
    let summary = render_summary(&result);
    fs::write(output_dir.join("summary.md"), &summary)?;
    print!("{summary}");
    Ok(())
}

fn validate_config(config: &BenchmarkConfig) -> Result<(), Box<dyn Error>> {
    if config.schema_version != 2 {
        return Err(format!("unsupported benchmark schema {}", config.schema_version).into());
    }
    if config.max_runtime_seconds == 0
        || config.autorouter_threads == 0
        || config.optimizer_threads == 0
    {
        return Err("benchmark limits and thread counts must be positive".into());
    }
    if config.random_seed_supported || config.random_seed.is_some() {
        return Err("Freerouting 2.1.0 does not support an injectable random seed".into());
    }
    if config.board_update_strategy != "greedy" || config.item_selection_strategy != "sequential" {
        return Err("deterministic benchmark requires greedy/sequential exposed settings".into());
    }
    Ok(())
}

fn validate_pass_budget(pass_budget: u32) -> Result<(), Box<dyn Error>> {
    if pass_budget == 0 {
        return Err("pass budget must be positive".into());
    }
    Ok(())
}

fn prepare_output_dir(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() && fs::read_dir(path)?.next().is_some() {
        return Err(format!(
            "output directory must be absent or empty: {}",
            path.display()
        )
        .into());
    }
    fs::create_dir_all(path)?;
    Ok(())
}

fn resolve(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn export_dsn(
    root: &SExpr,
    config: &BenchmarkConfig,
) -> Result<(String, DsnStats), Box<dyn Error>> {
    if root.tag() != Some("kicad_pcb") {
        return Err("input is not a KiCad PCB s-expression".into());
    }
    let nets = board_nets(root)?;
    let copper_layers = board_copper_layers(root)?;
    let boundary = board_boundary(root)?;
    let footprints = board_footprints(root, &copper_layers)?;
    let segments = board_segments(root)?;
    let vias = board_vias(root)?;
    let planes = board_planes(root)?;
    let default_via = via_padstack_name(
        config.default_via_size_mm,
        config.default_via_drill_mm,
        copper_layers.len(),
    );

    let mut out = String::with_capacity(512 * 1024);
    writeln!(
        out,
        "(pcb {}",
        quote(&format!("{}.dsn", config.fixture_name))
    )?;
    writeln!(out, "  (parser")?;
    writeln!(out, r#"    (string_quote ")"#)?;
    writeln!(out, "    (space_in_quoted_tokens on)")?;
    writeln!(out, "    (host_cad \"LaminarForge Rust benchmark\")")?;
    writeln!(out, "    (host_version \"1\")")?;
    writeln!(out, "  )")?;
    writeln!(out, "  (resolution um 10)")?;
    writeln!(out, "  (unit um)")?;
    writeln!(out, "  (structure")?;
    for (index, layer) in copper_layers.iter().enumerate() {
        writeln!(
            out,
            "    (layer {} (type signal) (property (index {})))",
            quote(layer),
            index
        )?;
    }
    let (x1, y1, x2, y2) = boundary;
    writeln!(
        out,
        "    (boundary (path pcb 0  {} {}  {} {}  {} {}  {} {}  {} {}))",
        dsn(x1),
        dsn_y(y1),
        dsn(x2),
        dsn_y(y1),
        dsn(x2),
        dsn_y(y2),
        dsn(x1),
        dsn_y(y2),
        dsn(x1),
        dsn_y(y1)
    )?;
    for plane in &planes {
        let net_name = nets
            .get(&plane.net_id)
            .ok_or_else(|| format!("zone references unknown net {}", plane.net_id))?;
        write!(
            out,
            "    (plane {} (polygon {} 0",
            quote(net_name),
            quote(&plane.layer)
        )?;
        for (x, y) in &plane.points {
            write!(out, " {} {}", dsn(*x), dsn_y(*y))?;
        }
        writeln!(out, "))")?;
    }
    writeln!(out, "    (via {})", quote(&default_via))?;
    writeln!(out, "    (rule")?;
    writeln!(out, "      (width {})", dsn(config.default_track_width_mm))?;
    writeln!(
        out,
        "      (clearance {})",
        dsn(config.default_clearance_mm)
    )?;
    writeln!(out, "    )")?;
    writeln!(out, "  )")?;

    writeln!(out, "  (placement")?;
    for footprint in &footprints {
        writeln!(out, "    (component {}", quote(&footprint.image_name))?;
        writeln!(
            out,
            "      (place {} {} {} {} {:.3})",
            token(&footprint.reference)?,
            dsn(footprint.x_mm),
            dsn_y(footprint.y_mm),
            if footprint.front { "front" } else { "back" },
            normalize_rotation(footprint.rotation_deg)
        )?;
        writeln!(out, "    )")?;
    }
    writeln!(out, "  )")?;

    writeln!(out, "  (library")?;
    for (footprint_index, footprint) in footprints.iter().enumerate() {
        for (pad_index, pad) in footprint.pads.iter().enumerate() {
            let name = padstack_name(footprint_index, pad_index);
            let (width, height) = rotated_size(pad.width_mm, pad.height_mm, pad.rotation_deg);
            writeln!(out, "    (padstack {}", quote(&name))?;
            for layer in &pad.copper_layers {
                if pad.shape == "circle" {
                    writeln!(
                        out,
                        "      (shape (circle {} {}))",
                        quote(layer),
                        dsn(width.max(height))
                    )?;
                } else {
                    writeln!(
                        out,
                        "      (shape (rect {} {} {} {} {}))",
                        quote(layer),
                        dsn(-width / 2.0),
                        dsn_y(height / 2.0),
                        dsn(width / 2.0),
                        dsn_y(-height / 2.0)
                    )?;
                }
            }
            writeln!(out, "      (attach off)")?;
            writeln!(out, "    )")?;
        }
    }
    let mut via_shapes = BTreeSet::new();
    via_shapes.insert((
        quantize(config.default_via_size_mm),
        quantize(config.default_via_drill_mm),
    ));
    for via in &vias {
        via_shapes.insert((quantize(via.size_mm), quantize(via.drill_mm)));
    }
    for (size_q, drill_q) in via_shapes {
        let size = size_q as f64 / 1000.0;
        let drill = drill_q as f64 / 1000.0;
        let name = via_padstack_name(size, drill, copper_layers.len());
        writeln!(out, "    (padstack {}", quote(&name))?;
        for layer in &copper_layers {
            writeln!(out, "      (shape (circle {} {}))", quote(layer), dsn(size))?;
        }
        writeln!(out, "      (attach off)")?;
        writeln!(out, "    )")?;
    }
    for (footprint_index, footprint) in footprints.iter().enumerate() {
        writeln!(out, "    (image {}", quote(&footprint.image_name))?;
        for (pad_index, pad) in footprint.pads.iter().enumerate() {
            writeln!(
                out,
                "      (pin {} {} {} {})",
                quote(&padstack_name(footprint_index, pad_index)),
                token(&pad.dsn_number)?,
                dsn(pad.x_mm),
                dsn_y(pad.y_mm)
            )?;
        }
        writeln!(out, "    )")?;
    }
    writeln!(out, "  )")?;

    let mut pins_by_net: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for footprint in &footprints {
        for pad in &footprint.pads {
            if pad.net_id != 0 {
                pins_by_net
                    .entry(pad.net_id)
                    .or_default()
                    .push(format!("{}-{}", footprint.reference, pad.dsn_number));
            }
        }
    }
    writeln!(out, "  (network")?;
    for (net_id, pins) in &pins_by_net {
        let net_name = nets
            .get(net_id)
            .ok_or_else(|| format!("pad references unknown net {net_id}"))?;
        write!(out, "    (net {} (pins", token(net_name)?)?;
        for pin in pins {
            write!(out, " {}", token(pin)?)?;
        }
        writeln!(out, "))")?;
    }
    write!(out, "    (class default")?;
    for net_id in pins_by_net.keys() {
        write!(
            out,
            " {}",
            token(
                nets.get(net_id)
                    .ok_or("network class references unknown net")?
            )?
        )?;
    }
    writeln!(out)?;
    writeln!(out, "      (circuit (use_via {}))", quote(&default_via))?;
    writeln!(out, "      (rule")?;
    writeln!(
        out,
        "        (width {})",
        dsn(config.default_track_width_mm)
    )?;
    writeln!(
        out,
        "        (clearance {})",
        dsn(config.default_clearance_mm)
    )?;
    writeln!(out, "      )")?;
    writeln!(out, "    )")?;
    writeln!(out, "  )")?;

    writeln!(out, "  (wiring")?;
    for segment in &segments {
        let net_name = nets
            .get(&segment.net_id)
            .ok_or_else(|| format!("segment references unknown net {}", segment.net_id))?;
        writeln!(out, "    (wire")?;
        writeln!(
            out,
            "      (path {} {} {} {} {} {})",
            quote(&segment.layer),
            dsn(segment.width_mm),
            dsn(segment.start.0),
            dsn_y(segment.start.1),
            dsn(segment.end.0),
            dsn_y(segment.end.1)
        )?;
        writeln!(out, "      (net {})", token(net_name)?)?;
        writeln!(out, "      (type protect)")?;
        writeln!(out, "    )")?;
    }
    for via in &vias {
        let net_name = nets
            .get(&via.net_id)
            .ok_or_else(|| format!("via references unknown net {}", via.net_id))?;
        writeln!(
            out,
            "    (via {} {} {} (net {}) (type protect))",
            quote(&via_padstack_name(
                via.size_mm,
                via.drill_mm,
                copper_layers.len()
            )),
            dsn(via.at.0),
            dsn_y(via.at.1),
            token(net_name)?
        )?;
    }
    writeln!(out, "  )")?;
    writeln!(out, ")")?;

    let stats = DsnStats {
        copper_layer_count: copper_layers.len(),
        footprint_count: footprints.len(),
        pad_count: footprints
            .iter()
            .map(|footprint| footprint.pads.len())
            .sum(),
        net_count: pins_by_net.len(),
        protected_wire_count: segments.len(),
        protected_via_count: vias.len(),
        plane_count: planes.len(),
    };
    Ok((out, stats))
}

fn board_nets(root: &SExpr) -> Result<BTreeMap<u32, String>, Box<dyn Error>> {
    let mut nets = BTreeMap::new();
    for node in root.direct_all("net") {
        let id = parse_u32(node.atom_at(1), "net id")?;
        let name = node.atom_at(2).ok_or("net has no name")?.to_string();
        nets.insert(id, name);
    }
    if nets.is_empty() {
        return Err("board has no nets".into());
    }
    Ok(nets)
}

fn board_net_ids(root: &SExpr) -> Result<BTreeMap<String, u32>, Box<dyn Error>> {
    Ok(board_nets(root)?
        .into_iter()
        .map(|(id, name)| (name, id))
        .collect())
}

fn board_copper_layers(root: &SExpr) -> Result<Vec<String>, Box<dyn Error>> {
    let layers = root.direct("layers").ok_or("board has no layers section")?;
    let result = layers
        .list()
        .into_iter()
        .flatten()
        .skip(1)
        .filter_map(|node| node.atom_at(1))
        .filter(|name| name.ends_with(".Cu"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    if result.len() < 2 {
        return Err("board must have at least two copper layers".into());
    }
    Ok(result)
}

fn board_boundary(root: &SExpr) -> Result<(f64, f64, f64, f64), Box<dyn Error>> {
    for rect in root.direct_all("gr_rect") {
        if rect.direct("layer").and_then(|node| node.atom_at(1)) != Some("Edge.Cuts") {
            continue;
        }
        let start = point(rect.direct("start"), "gr_rect start")?;
        let end = point(rect.direct("end"), "gr_rect end")?;
        return Ok((
            start.0.min(end.0),
            start.1.min(end.1),
            start.0.max(end.0),
            start.1.max(end.1),
        ));
    }
    Err("board has no rectangular Edge.Cuts boundary".into())
}

fn board_footprints(
    root: &SExpr,
    copper_layers: &[String],
) -> Result<Vec<FootprintInfo>, Box<dyn Error>> {
    let mut result = Vec::new();
    for (index, node) in root.direct_all("footprint").enumerate() {
        let reference = node
            .direct_all("property")
            .find(|property| property.atom_at(1) == Some("Reference"))
            .and_then(|property| property.atom_at(2))
            .ok_or("footprint has no Reference property")?
            .to_string();
        let at = node.direct("at").ok_or("footprint has no at")?;
        let x_mm = parse_f64(at.atom_at(1), "footprint x")?;
        let y_mm = parse_f64(at.atom_at(2), "footprint y")?;
        let rotation_deg = optional_f64(at.atom_at(3))?;
        let layer = node
            .direct("layer")
            .and_then(|value| value.atom_at(1))
            .ok_or("footprint has no layer")?;
        let front = layer.starts_with('F');
        let mut pads = Vec::new();
        let mut pad_number_counts: BTreeMap<String, usize> = BTreeMap::new();
        for pad in node.direct_all("pad") {
            let number = pad.atom_at(1).ok_or("pad has no number")?.to_string();
            let number_count = pad_number_counts.entry(number.clone()).or_default();
            *number_count += 1;
            let dsn_number = if *number_count == 1 {
                number.clone()
            } else {
                format!("{}__{}", number, number_count)
            };
            let _kind = pad.atom_at(2).ok_or("pad has no kind")?;
            let shape = pad.atom_at(3).ok_or("pad has no shape")?.to_string();
            let pad_at = pad.direct("at").ok_or("pad has no at")?;
            let size = pad.direct("size").ok_or("pad has no size")?;
            let layer_node = pad.direct("layers").ok_or("pad has no layers")?;
            let layer_names = layer_node
                .list()
                .into_iter()
                .flatten()
                .skip(1)
                .filter_map(SExpr::atom)
                .collect::<Vec<_>>();
            let pad_copper_layers = if layer_names.contains(&"*.Cu") {
                copper_layers.to_vec()
            } else {
                layer_names
                    .into_iter()
                    .filter(|name| name.ends_with(".Cu"))
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            };
            if pad_copper_layers.is_empty() {
                continue;
            }
            let net_id = pad
                .direct("net")
                .map(|net| parse_u32(net.atom_at(1), "pad net"))
                .transpose()?
                .unwrap_or(0);
            pads.push(PadInfo {
                dsn_number,
                shape,
                x_mm: parse_f64(pad_at.atom_at(1), "pad x")?,
                y_mm: parse_f64(pad_at.atom_at(2), "pad y")?,
                rotation_deg: optional_f64(pad_at.atom_at(3))?,
                width_mm: parse_f64(size.atom_at(1), "pad width")?,
                height_mm: parse_f64(size.atom_at(2), "pad height")?,
                copper_layers: pad_copper_layers,
                net_id,
            });
        }
        if pads.is_empty() {
            continue;
        }
        result.push(FootprintInfo {
            image_name: format!("fixture_image_{index:03}"),
            reference,
            x_mm,
            y_mm,
            rotation_deg,
            front,
            pads,
        });
    }
    Ok(result)
}

fn board_segments(root: &SExpr) -> Result<Vec<SegmentInfo>, Box<dyn Error>> {
    root.direct_all("segment")
        .map(|node| {
            Ok(SegmentInfo {
                start: point(node.direct("start"), "segment start")?,
                end: point(node.direct("end"), "segment end")?,
                width_mm: parse_f64(
                    node.direct("width").and_then(|value| value.atom_at(1)),
                    "segment width",
                )?,
                layer: node
                    .direct("layer")
                    .and_then(|value| value.atom_at(1))
                    .ok_or("segment has no layer")?
                    .to_string(),
                net_id: parse_u32(
                    node.direct("net").and_then(|value| value.atom_at(1)),
                    "segment net",
                )?,
            })
        })
        .collect()
}

fn board_vias(root: &SExpr) -> Result<Vec<ViaInfo>, Box<dyn Error>> {
    root.direct_all("via")
        .map(|node| {
            Ok(ViaInfo {
                at: point(node.direct("at"), "via at")?,
                size_mm: parse_f64(
                    node.direct("size").and_then(|value| value.atom_at(1)),
                    "via size",
                )?,
                drill_mm: parse_f64(
                    node.direct("drill").and_then(|value| value.atom_at(1)),
                    "via drill",
                )?,
                net_id: parse_u32(
                    node.direct("net").and_then(|value| value.atom_at(1)),
                    "via net",
                )?,
            })
        })
        .collect()
}

fn board_planes(root: &SExpr) -> Result<Vec<PlaneInfo>, Box<dyn Error>> {
    let mut result = Vec::new();
    for node in root.direct_all("zone") {
        let net_id = parse_u32(
            node.direct("net").and_then(|value| value.atom_at(1)),
            "zone net",
        )?;
        if net_id == 0 {
            continue;
        }
        let layer = node
            .direct("layer")
            .and_then(|value| value.atom_at(1))
            .ok_or("zone has no layer")?
            .to_string();
        let polygon = node.direct("polygon").ok_or("zone has no polygon")?;
        let points_node = polygon.direct("pts").ok_or("zone polygon has no pts")?;
        let points = points_node
            .direct_all("xy")
            .map(|point_node| {
                Ok((
                    parse_f64(point_node.atom_at(1), "zone point x")?,
                    parse_f64(point_node.atom_at(2), "zone point y")?,
                ))
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
        if points.len() >= 3 {
            result.push(PlaneInfo {
                net_id,
                layer,
                points,
            });
        }
    }
    Ok(result)
}

fn point(node: Option<&SExpr>, label: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let node = node.ok_or_else(|| format!("missing {label}"))?;
    Ok((
        parse_f64(node.atom_at(1), &format!("{label} x"))?,
        parse_f64(node.atom_at(2), &format!("{label} y"))?,
    ))
}

fn parse_f64(value: Option<&str>, label: &str) -> Result<f64, Box<dyn Error>> {
    value
        .ok_or_else(|| format!("missing {label}"))?
        .parse::<f64>()
        .map_err(|error| format!("invalid {label}: {error}").into())
}

fn optional_f64(value: Option<&str>) -> Result<f64, Box<dyn Error>> {
    value
        .map(str::parse::<f64>)
        .transpose()
        .map(|value| value.unwrap_or(0.0))
        .map_err(Into::into)
}

fn parse_u32(value: Option<&str>, label: &str) -> Result<u32, Box<dyn Error>> {
    value
        .ok_or_else(|| format!("missing {label}"))?
        .parse::<u32>()
        .map_err(|error| format!("invalid {label}: {error}").into())
}

fn dsn(mm: f64) -> i64 {
    (mm * 1000.0 * DSN_INPUT_UNITS_PER_UM).round() as i64
}

fn dsn_y(mm: f64) -> i64 {
    dsn(-mm)
}

fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('\"', "\\\""))
}

fn token(value: &str) -> Result<&str, Box<dyn Error>> {
    if value.is_empty()
        || value.chars().any(|character| {
            character.is_whitespace() || character == '(' || character == ')' || character == '\"'
        })
    {
        return Err(format!("value is not a safe unquoted Specctra token: {value:?}").into());
    }
    Ok(value)
}

fn normalize_rotation(value: f64) -> f64 {
    let mut result = value % 360.0;
    if result >= 180.0 {
        result -= 360.0;
    }
    if result < -180.0 {
        result += 360.0;
    }
    result
}

fn rotated_size(width: f64, height: f64, rotation: f64) -> (f64, f64) {
    let normalized = normalize_rotation(rotation).abs();
    if (normalized - 90.0).abs() < 0.001 {
        (height, width)
    } else {
        (width, height)
    }
}

fn padstack_name(footprint_index: usize, pad_index: usize) -> String {
    format!("Pad_{footprint_index:03}_{pad_index:03}")
}

fn quantize(mm: f64) -> i64 {
    (mm * 1000.0).round() as i64
}

fn via_padstack_name(size_mm: f64, drill_mm: f64, layer_count: usize) -> String {
    format!(
        "Via[0-{}]_{}:{}_um",
        layer_count - 1,
        quantize(size_mm),
        quantize(drill_mm)
    )
}

fn read_jar_manifest(path: &Path) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut manifest = String::new();
    archive
        .by_name("META-INF/MANIFEST.MF")?
        .read_to_string(&mut manifest)?;
    let mut values = BTreeMap::new();
    for line in manifest.lines() {
        if let Some((key, value)) = line.split_once(": ") {
            values.insert(key.to_string(), value.trim().to_string());
        }
    }
    Ok(values)
}

fn copy_validation_bundle(
    board_path: &Path,
    project_path: &Path,
    design_rules_path: &Path,
    output_dir: &Path,
    board_content: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(output_dir)?;
    let board_name = board_path.file_name().ok_or("board path has no filename")?;
    let board_copy = output_dir.join(board_name);
    fs::write(&board_copy, board_content)?;
    fs::copy(
        project_path,
        output_dir.join(
            project_path
                .file_name()
                .ok_or("project path has no filename")?,
        ),
    )?;
    fs::copy(
        design_rules_path,
        output_dir.join(
            design_rules_path
                .file_name()
                .ok_or("design rules path has no filename")?,
        ),
    )?;
    Ok(board_copy)
}

fn run_kicad_drc(
    kicad_cli: &OsString,
    board_path: &Path,
    report_path: &Path,
) -> Result<DrcCounts, Box<dyn Error>> {
    let output = Command::new(kicad_cli)
        .arg("pcb")
        .arg("drc")
        .arg("--refill-zones")
        .arg("--output")
        .arg(report_path)
        .arg("--format")
        .arg("json")
        .arg("--severity-all")
        .arg(board_path)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "kicad-cli DRC failed for {}\nstdout:\n{}\nstderr:\n{}",
            board_path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let report: DrcReport = serde_json::from_slice(&fs::read(report_path)?)?;
    let physical_violations = report
        .violations
        .iter()
        .map(|violation| DrcViolationSummary {
            severity: violation
                .get("severity")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            violation_type: violation
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            description: violation
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or("missing description")
                .to_string(),
        })
        .collect::<Vec<_>>();
    let error_violation_count = physical_violations
        .iter()
        .filter(|violation| violation.severity == "error")
        .count();
    let warning_violation_count = physical_violations
        .iter()
        .filter(|violation| violation.severity == "warning")
        .count();
    let ignored = report
        .unconnected_items
        .iter()
        .filter(|entry| is_self_zone_unconnected(entry))
        .count();
    Ok(DrcCounts {
        physical_violation_count: report.violations.len(),
        error_violation_count,
        warning_violation_count,
        raw_unconnected_count: report.unconnected_items.len(),
        real_unconnected_count: report.unconnected_items.len() - ignored,
        ignored_self_zone_unconnected_count: ignored,
        physical_violations,
    })
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

fn freerouting_arguments(
    jar_path: &Path,
    user_data_path: &Path,
    dsn_path: &Path,
    ses_path: &Path,
    config: &BenchmarkConfig,
    pass_budget: u32,
) -> Vec<OsString> {
    let mut args = vec![
        OsString::from("-Djava.awt.headless=true"),
        OsString::from("-jar"),
        jar_path.as_os_str().to_os_string(),
        OsString::from(format!("--user_data_path={}", user_data_path.display())),
        OsString::from("-de"),
        dsn_path.as_os_str().to_os_string(),
        OsString::from("-do"),
        ses_path.as_os_str().to_os_string(),
        OsString::from("-mp"),
        OsString::from(pass_budget.to_string()),
        OsString::from(format!("--router.stop_pass_no={pass_budget}")),
        OsString::from(format!(
            "--router.max_threads={}",
            config.autorouter_threads
        )),
        OsString::from(format!(
            "--router.optimizer.enabled={}",
            config.optimizer_enabled
        )),
        OsString::from("-mt"),
        OsString::from(config.optimizer_threads.to_string()),
        OsString::from("-oit"),
        OsString::from(config.optimizer_improvement_threshold_percent.to_string()),
        OsString::from("-us"),
        OsString::from(&config.board_update_strategy),
        OsString::from("-is"),
        OsString::from(&config.item_selection_strategy),
        OsString::from(format!("--router.fanout.enabled={}", config.fanout_enabled)),
        OsString::from(format!(
            "--router.job_timeout=00:{:02}:00",
            (config.max_runtime_seconds / 60).clamp(1, 59)
        )),
        OsString::from("--gui.enabled=false"),
    ];
    if config.analytics_disabled {
        args.push(OsString::from("-da"));
    }
    args
}

struct ProcessResult {
    runtime_ms: u128,
    exit_code: i32,
    timed_out: bool,
}

fn run_bounded(
    executable: &OsString,
    args: &[OsString],
    output_dir: &Path,
    max_runtime_seconds: u64,
) -> Result<ProcessResult, Box<dyn Error>> {
    let stdout_path = output_dir.join("freerouting.stdout.log");
    let stderr_path = output_dir.join("freerouting.stderr.log");
    let stdout = File::create(&stdout_path)?;
    let stderr = File::create(&stderr_path)?;
    let start = Instant::now();
    let mut child = Command::new(executable)
        .args(args)
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .spawn()?;
    let deadline = Duration::from_secs(max_runtime_seconds);
    let (status, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (status, false);
        }
        if start.elapsed() >= deadline {
            child.kill()?;
            break (child.wait()?, true);
        }
        thread::sleep(Duration::from_millis(100));
    };
    Ok(ProcessResult {
        runtime_ms: start.elapsed().as_millis(),
        exit_code: status.code().unwrap_or(-1),
        timed_out,
    })
}

fn command_version(command: &OsString, args: &[OsString]) -> Result<String, Box<dyn Error>> {
    let output = Command::new(command).args(args).output()?;
    if !output.status.success() {
        return Err(format!(
            "version command failed: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(combined
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_string())
}

fn last_json_object(log: &str) -> Option<Value> {
    let trimmed = log.trim_end();
    std::iter::once(0)
        .chain(trimmed.match_indices("\n{").map(|(index, _)| index + 1))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .find_map(|start| serde_json::from_str(&trimmed[start..]).ok())
}

fn observed_autorouter_passes(log: &str) -> u32 {
    log.lines()
        .filter_map(|line| {
            let (_, tail) = line.split_once("Auto-router pass #")?;
            let digits = tail
                .chars()
                .take_while(|character| character.is_ascii_digit())
                .collect::<String>();
            digits.parse::<u32>().ok()
        })
        .max()
        .unwrap_or(0)
}

fn verify_observed_autorouter_passes(
    pass_budget: u32,
    observed_passes: u32,
    freerouting_unrouted_connections: Option<u64>,
) -> Result<PassCountVerification, String> {
    if observed_passes == 0 {
        return Err("Freerouting log did not report an autorouter pass".into());
    }
    if observed_passes > pass_budget {
        return Err(format!(
            "Freerouting exceeded the explicit pass budget: requested {pass_budget}, observed {observed_passes}"
        ));
    }
    if observed_passes == pass_budget {
        return Ok(PassCountVerification::ExactBudget);
    }
    if freerouting_unrouted_connections == Some(0) {
        return Ok(PassCountVerification::CompletedBeforeBudget);
    }
    Err(format!(
        "Freerouting stopped before the explicit pass budget without completing routing: requested {pass_budget}, observed {observed_passes}, remaining {}",
        display_optional(freerouting_unrouted_connections)
    ))
}

fn ses_metrics(
    ses: &SesData,
    statistics: Option<&Value>,
    before_real_unconnected: usize,
) -> BenchmarkMetrics {
    let mut wire_count = 0;
    let mut segment_count = 0;
    let mut via_count = 0;
    let mut total_length = 0.0;
    for route in &ses.routes {
        wire_count += route.wires.len();
        via_count += route.vias.len();
        for wire in &route.wires {
            segment_count += wire.points.len().saturating_sub(1);
            total_length += wire
                .points
                .windows(2)
                .map(|points| {
                    let dx = points[1].0 - points[0].0;
                    let dy = points[1].1 - points[0].1;
                    dx.hypot(dy)
                })
                .sum::<f64>();
        }
    }
    let incomplete = statistics
        .and_then(|value| {
            value
                .pointer("/connections/incomplete_count")
                .or_else(|| value.pointer("/connections/incompleteCount"))
        })
        .and_then(Value::as_u64);
    let total = statistics.and_then(|value| {
        value
            .pointer("/connections/maximum_count")
            .or_else(|| value.pointer("/connections/total_count"))
            .or_else(|| value.pointer("/connections/totalCount"))
            .and_then(Value::as_u64)
    });
    BenchmarkMetrics {
        freerouting_routed_connection_count: incomplete
            .map(|unrouted| (before_real_unconnected as u64).saturating_sub(unrouted)),
        freerouting_total_completed_connection_count: total
            .zip(incomplete)
            .map(|(a, b)| a.saturating_sub(b)),
        freerouting_unrouted_connection_count: incomplete,
        ses_net_count: ses.routes.len(),
        ses_wire_count: wire_count,
        ses_segment_count: segment_count,
        ses_via_count: via_count,
        ses_total_trace_length_mm: round_six(total_length),
    }
}

fn import_ses(
    board: &str,
    ses: &SesData,
    net_ids: &BTreeMap<String, u32>,
) -> Result<String, Box<dyn Error>> {
    let mut stripped = remove_top_level_forms(board, &["segment", "via"])?;
    let final_close = stripped.rfind(')').ok_or("board has no final close")?;
    let mut routes = String::new();
    let mut uuid_counter = 1u64;
    for route in &ses.routes {
        let net_id = *net_ids
            .get(&route.net_name)
            .ok_or_else(|| format!("SES references unknown net {}", route.net_name))?;
        for wire in &route.wires {
            for points in wire.points.windows(2) {
                writeln!(
                    routes,
                    "  (segment (start {:.6} {:.6}) (end {:.6} {:.6}) (width {:.6}) (layer {}) (net {}) (uuid {}))",
                    points[0].0,
                    points[0].1,
                    points[1].0,
                    points[1].1,
                    wire.width_mm,
                    quote(&wire.layer),
                    net_id,
                    deterministic_uuid(uuid_counter)
                )?;
                uuid_counter += 1;
            }
        }
        for via in &route.vias {
            let (size, drill) = parse_via_dimensions(&via.padstack)?;
            writeln!(
                routes,
                "  (via (at {:.6} {:.6}) (size {:.6}) (drill {:.6}) (layers \"F.Cu\" \"B.Cu\") (net {}) (uuid {}))",
                via.x_mm,
                via.y_mm,
                size,
                drill,
                net_id,
                deterministic_uuid(uuid_counter)
            )?;
            uuid_counter += 1;
        }
    }
    stripped.insert_str(final_close, &routes);
    Ok(stripped)
}

fn parse_via_dimensions(name: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let tail = name
        .split_once("]_")
        .map(|(_, tail)| tail)
        .ok_or_else(|| format!("SES via padstack does not encode dimensions: {name}"))?;
    let encoded = tail.strip_suffix("_um").unwrap_or(tail);
    let (size, drill) = encoded
        .split_once(':')
        .ok_or_else(|| format!("SES via padstack does not encode size:drill: {name}"))?;
    Ok((
        size.parse::<f64>()? / 1000.0,
        drill.parse::<f64>()? / 1000.0,
    ))
}

fn deterministic_uuid(counter: u64) -> String {
    format!("f0000000-0000-4000-8000-{counter:012x}")
}

fn remove_top_level_forms(input: &str, tags: &[&str]) -> Result<String, Box<dyn Error>> {
    let bytes = input.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    let mut index = 0;
    let mut depth = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if quoted {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'\"' {
                quoted = false;
            }
            index += 1;
            continue;
        }
        if byte == b'\"' {
            quoted = true;
            index += 1;
            continue;
        }
        if byte == b'(' {
            if depth == 1 {
                let tag = form_tag(&input[index + 1..]);
                if tags.contains(&tag) {
                    output.push_str(&input[cursor..index]);
                    let end = matching_close(input, index)?;
                    cursor = end;
                    index = end;
                    continue;
                }
            }
            depth += 1;
        } else if byte == b')' {
            depth = depth.checked_sub(1).ok_or("unbalanced board close")?;
        }
        index += 1;
    }
    if depth != 0 || quoted {
        return Err("unbalanced board while removing routed forms".into());
    }
    output.push_str(&input[cursor..]);
    Ok(output)
}

fn form_tag(input: &str) -> &str {
    let trimmed = input.trim_start();
    let end = trimmed
        .find(|character: char| character.is_whitespace() || character == '(' || character == ')')
        .unwrap_or(trimmed.len());
    &trimmed[..end]
}

fn matching_close(input: &str, start: usize) -> Result<usize, Box<dyn Error>> {
    let bytes = input.as_bytes();
    let mut depth = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    for (offset, byte) in bytes[start..].iter().copied().enumerate() {
        if quoted {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'\"' {
                quoted = false;
            }
            continue;
        }
        if byte == b'\"' {
            quoted = true;
        } else if byte == b'(' {
            depth += 1;
        } else if byte == b')' {
            depth -= 1;
            if depth == 0 {
                return Ok(start + offset + 1);
            }
        }
    }
    Err("unterminated top-level form".into())
}

fn sha256_file(path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(sha256_bytes(&fs::read(path)?))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn git_commit(root: &Path) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err("git rev-parse HEAD failed".into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn round_six(value: f64) -> f64 {
    (value * 1_000_000.0).round() / 1_000_000.0
}

fn relative_to(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn render_summary(result: &BenchmarkResult) -> String {
    format!(
        "# Freerouting baseline: {}\n\n\
- Freerouting: `v{}` at `{}`\n\
- Input board SHA-256: `{}`\n\
- Generated DSN SHA-256: `{}`\n\
- Raw SES SHA-256: `{}`\n\
- Runtime: `{}` ms (bound `{}` s)\n\
- Autorouter passes: `{}` observed (explicit budget `{}`, verification `{}`)\n\
- Freerouting routed/unrouted connections: `{}` / `{}`\n\
- SES: `{}` nets, `{}` wires, `{}` segments, `{}` vias, `{:.6}` mm trace length\n\
- KiCad before import: `{}` physical DRC (`{}` errors, `{}` warnings), `{}` real unconnected\n\
- KiCad after import: `{}` physical DRC (`{}` errors, `{}` warnings), `{}` real unconnected\n\
- Random seed: unsupported by Freerouting 2.1.0 (`null`)\n\
- Canonical board modified: `{}`\n\
- Fixture safe for public upstream use: `{}`\n",
        result.fixture_name,
        result.tools.freerouting_version,
        result.tools.freerouting_build_revision,
        result.input.board_sha256,
        result.input.dsn_sha256,
        result.execution.ses_output_sha256,
        result.execution.runtime_ms,
        result.effective_settings.max_runtime_seconds,
        result.execution.observed_autorouter_passes,
        result.effective_settings.pass_budget,
        result.execution.pass_count_verification.label(),
        display_optional(result.metrics.freerouting_routed_connection_count),
        display_optional(result.metrics.freerouting_unrouted_connection_count),
        result.metrics.ses_net_count,
        result.metrics.ses_wire_count,
        result.metrics.ses_segment_count,
        result.metrics.ses_via_count,
        result.metrics.ses_total_trace_length_mm,
        result.validation.before_import.physical_violation_count,
        result.validation.before_import.error_violation_count,
        result.validation.before_import.warning_violation_count,
        result.validation.before_import.real_unconnected_count,
        result.validation.after_import.physical_violation_count,
        result.validation.after_import.error_violation_count,
        result.validation.after_import.warning_violation_count,
        result.validation.after_import.real_unconnected_count,
        result.validation.canonical_board_modified,
        result.fixture_publication.safe_for_public_upstream,
    )
}

fn display_optional(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not reported".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINI_BOARD: &str = r#"(kicad_pcb
  (layers (0 "F.Cu" signal) (31 "B.Cu" signal))
  (net 0 "")
  (net 1 "GND")
  (gr_rect (start 0 0) (end 10 5) (layer "Edge.Cuts"))
  (footprint "R" (layer "F.Cu") (at 2 3 90)
    (property "Reference" "R1")
    (pad "1" smd rect (at -1 0) (size 1 2) (layers "F.Cu") (net 1 "GND"))
    (pad "2" smd circle (at 1 0) (size 1 1) (layers "F.Cu")))
  (segment (start 1 1) (end 2 1) (width 0.2) (layer "F.Cu") (net 1))
  (via (at 2 1) (size 0.7) (drill 0.3) (layers "F.Cu" "B.Cu") (net 1))
)"#;

    fn test_config() -> BenchmarkConfig {
        BenchmarkConfig {
            schema_version: 2,
            fixture_name: "mini".into(),
            board_path: "mini.kicad_pcb".into(),
            project_path: "mini.kicad_pro".into(),
            design_rules_path: "mini.kicad_dru".into(),
            freerouting_jar_path: "freerouting.jar".into(),
            expected_freerouting_version: "2.1.0".into(),
            expected_freerouting_revision: "revision".into(),
            expected_freerouting_jar_sha256: "hash".into(),
            max_runtime_seconds: 60,
            autorouter_threads: 1,
            optimizer_enabled: false,
            optimizer_threads: 1,
            optimizer_improvement_threshold_percent: 1.0,
            board_update_strategy: "greedy".into(),
            item_selection_strategy: "sequential".into(),
            fanout_enabled: false,
            analytics_disabled: true,
            default_track_width_mm: 0.2,
            default_clearance_mm: 0.15,
            default_via_size_mm: 0.7,
            default_via_drill_mm: 0.3,
            random_seed_supported: false,
            random_seed: None,
        }
    }

    #[test]
    fn parses_board_and_exports_scaled_dsn() {
        let root = SExprParser::new(MINI_BOARD).parse().unwrap();
        let (dsn, stats) = export_dsn(&root, &test_config()).unwrap();
        assert!(dsn.contains("(boundary (path pcb 0  0 0  10000 0"));
        assert!(dsn.contains("(place R1 2000 -3000 front 90.000)"));
        assert!(dsn.contains("(type protect)"));
        assert_eq!(stats.footprint_count, 1);
        assert_eq!(stats.pad_count, 2);
        assert_eq!(stats.protected_wire_count, 1);
        assert_eq!(stats.protected_via_count, 1);
    }

    #[test]
    fn removes_only_top_level_routing_forms() {
        let stripped = remove_top_level_forms(MINI_BOARD, &["segment", "via"]).unwrap();
        assert!(!stripped.contains("  (segment"));
        assert!(!stripped.contains("  (via"));
        assert!(stripped.contains("(pad \"1\" smd rect"));
        SExprParser::new(&stripped).parse().unwrap();
    }

    #[test]
    fn parses_encoded_via_dimensions_without_fallback() {
        assert_eq!(
            parse_via_dimensions("Via[0-3]_1200:500_um").unwrap(),
            (1.2, 0.5)
        );
        assert!(parse_via_dimensions("anonymous-via").is_err());
    }

    #[test]
    fn filters_only_identical_self_zone_connectivity_entries() {
        let entry = DrcEntry {
            items: vec![
                DrcItem {
                    description: "Zone [GND]".into(),
                    uuid: Some("same".into()),
                },
                DrcItem {
                    description: "Zone [GND]".into(),
                    uuid: Some("same".into()),
                },
            ],
        };
        assert!(is_self_zone_unconnected(&entry));
    }

    #[test]
    fn extracts_highest_observed_autorouter_pass() {
        let log = "Auto-router pass #1 completed\nAuto-router pass #12 completed\n";
        assert_eq!(observed_autorouter_passes(log), 12);
    }

    #[test]
    fn verifies_exact_or_clean_early_pass_completion() {
        assert_eq!(
            verify_observed_autorouter_passes(10, 10, Some(3)).unwrap(),
            PassCountVerification::ExactBudget
        );
        assert_eq!(
            verify_observed_autorouter_passes(50, 7, Some(0)).unwrap(),
            PassCountVerification::CompletedBeforeBudget
        );
        assert!(verify_observed_autorouter_passes(10, 11, Some(2)).is_err());
        assert!(verify_observed_autorouter_passes(10, 7, Some(2)).is_err());
        assert!(verify_observed_autorouter_passes(10, 0, None).is_err());
    }
}
