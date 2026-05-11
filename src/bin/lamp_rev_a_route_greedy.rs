use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

const BOARD_PATH: &str = "pcb/lamp_rev_a/lamp_rev_a.kicad_pcb";
const DRC_REPORT_PATH: &str = "pcb/lamp_rev_a/reports/drc.json";
const ROUTING_SEED_PATH: &str = "pcb/lamp_rev_a/routing_seed.toml";

#[derive(Debug, Deserialize)]
struct DrcReport {
    #[serde(default)]
    violations: Vec<DrcEntry>,
    #[serde(default)]
    unconnected_items: Vec<DrcEntry>,
}

#[derive(Debug, Deserialize, Clone)]
struct DrcEntry {
    #[serde(default)]
    items: Vec<DrcItem>,
}

#[derive(Debug, Deserialize, Clone)]
struct DrcItem {
    description: String,
    pos: Position,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize, Clone)]
struct RoutingSeed {
    #[serde(default)]
    segments: Vec<RouteSegment>,
}

#[derive(Debug, Deserialize, Clone)]
struct RouteSegment {
    net: String,
    layer: String,
    #[serde(default)]
    via_at_ends: bool,
    width_mm: f64,
    start_x_mm: f64,
    start_y_mm: f64,
    end_x_mm: f64,
    end_y_mm: f64,
}

#[derive(Clone)]
struct Candidate {
    label: String,
    distance_mm: f64,
    segments: Vec<RouteSegment>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let max_accepts = env_usize("LAMP_ROUTE_MAX_ACCEPTS", 12)?;
    let max_candidates = env_usize("LAMP_ROUTE_MAX_CANDIDATES", 240)?;
    let max_rejects_per_round = env_usize("LAMP_ROUTE_MAX_REJECTS_PER_ROUND", 90)?;
    let kicad_cli = env::var("KICAD_CLI").unwrap_or_else(|_| "/opt/homebrew/bin/kicad-cli".into());

    let mut seed = read_seed()?;
    let original_seed = seed.clone();
    let mut accepted = 0usize;

    run_materializer()?;
    run_drc(&kicad_cli)?;
    let mut baseline = read_drc_report()?;
    if !baseline.violations.is_empty() {
        return Err(format!(
            "refusing greedy routing with {} physical DRC violations",
            baseline.violations.len()
        )
        .into());
    }

    println!("LAMP Rev A greedy routing");
    println!(
        "  starting unconnected: {}",
        baseline.unconnected_items.len()
    );
    println!("  existing seed segments: {}", seed.segments.len());

    while accepted < max_accepts && !baseline.unconnected_items.is_empty() {
        let candidates = build_candidates(&baseline, max_candidates);
        let mut accepted_this_round = false;
        let mut rejected_this_round = 0usize;

        for candidate in candidates {
            let trial_seed = with_candidate(&seed, &candidate);
            write_seed(&trial_seed)?;
            run_materializer()?;
            run_drc(&kicad_cli)?;
            let trial = read_drc_report()?;

            if trial.violations.is_empty()
                && trial.unconnected_items.len() < baseline.unconnected_items.len()
            {
                let removed = baseline.unconnected_items.len() - trial.unconnected_items.len();
                println!(
                    "  accepted {}: -{} unconnected ({} -> {})",
                    candidate.label,
                    removed,
                    baseline.unconnected_items.len(),
                    trial.unconnected_items.len()
                );
                seed = trial_seed;
                baseline = trial;
                accepted += 1;
                accepted_this_round = true;
                break;
            }

            write_seed(&seed)?;
            rejected_this_round += 1;
            if rejected_this_round >= max_rejects_per_round {
                println!("  stopping round after {rejected_this_round} rejected candidates");
                break;
            }
        }

        if !accepted_this_round {
            break;
        }
    }

    write_seed(&seed)?;
    run_materializer()?;
    run_drc(&kicad_cli)?;
    let final_report = read_drc_report()?;
    if !final_report.violations.is_empty() {
        write_seed(&original_seed)?;
        run_materializer()?;
        return Err(format!(
            "greedy router ended with {} DRC violations; restored original seed",
            final_report.violations.len()
        )
        .into());
    }

    println!("  accepted routes: {accepted}");
    println!("  final seed segments: {}", seed.segments.len());
    println!(
        "  final unconnected: {}",
        final_report.unconnected_items.len()
    );
    Ok(())
}

fn build_candidates(report: &DrcReport, max_candidates: usize) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();

    for entry in &report.unconnected_items {
        let [start, end] = entry.items.as_slice() else {
            continue;
        };
        let Some(net) = extract_net(&start.description) else {
            continue;
        };
        if extract_net(&end.description).as_deref() != Some(net.as_str()) {
            continue;
        }

        for candidate in candidates_for_pair(&net, start.pos, end.pos) {
            if seen.insert(candidate_key(&candidate)) {
                candidates.push(candidate);
            }
        }
    }

    candidates.sort_by(|a, b| {
        a.distance_mm
            .partial_cmp(&b.distance_mm)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.segments.len().cmp(&b.segments.len()))
            .then_with(|| a.label.cmp(&b.label))
    });
    candidates.truncate(max_candidates);
    candidates
}

fn candidates_for_pair(net: &str, start: Position, end: Position) -> Vec<Candidate> {
    let width = width_for_net(net);
    let direct_distance = distance(start, end);
    let mut candidates = Vec::new();

    candidates.push(Candidate {
        label: format!("{net}:F-direct"),
        distance_mm: direct_distance,
        segments: vec![segment(net, "F.Cu", false, width, start, end)],
    });
    candidates.push(Candidate {
        label: format!("{net}:B-direct"),
        distance_mm: direct_distance + 0.25,
        segments: vec![segment(net, "B.Cu", true, width, start, end)],
    });

    for (label, corner) in [
        (
            "hv",
            Position {
                x: end.x,
                y: start.y,
            },
        ),
        (
            "vh",
            Position {
                x: start.x,
                y: end.y,
            },
        ),
    ] {
        if distance(start, corner) < 0.05 || distance(corner, end) < 0.05 {
            continue;
        }
        candidates.push(Candidate {
            label: format!("{net}:F-{label}"),
            distance_mm: distance(start, corner) + distance(corner, end) + 0.5,
            segments: vec![
                segment(net, "F.Cu", false, width, start, corner),
                segment(net, "F.Cu", false, width, corner, end),
            ],
        });
        candidates.push(Candidate {
            label: format!("{net}:B-{label}"),
            distance_mm: distance(start, corner) + distance(corner, end) + 0.75,
            segments: vec![
                segment(net, "B.Cu", true, width, start, corner),
                segment(net, "B.Cu", true, width, corner, end),
            ],
        });
    }

    for &y in routing_channel_ys(net) {
        if (start.y - y).abs() < 0.05 || (end.y - y).abs() < 0.05 {
            continue;
        }
        let a = Position { x: start.x, y };
        let b = Position { x: end.x, y };
        candidates.push(Candidate {
            label: format!("{net}:B-y{y:.1}"),
            distance_mm: distance(start, a) + distance(a, b) + distance(b, end) + 1.5,
            segments: vec![
                segment(net, "B.Cu", true, width, start, a),
                segment(net, "B.Cu", true, width, a, b),
                segment(net, "B.Cu", true, width, b, end),
            ],
        });
    }

    for &x in routing_channel_xs(net) {
        if (start.x - x).abs() < 0.05 || (end.x - x).abs() < 0.05 {
            continue;
        }
        let a = Position { x, y: start.y };
        let b = Position { x, y: end.y };
        candidates.push(Candidate {
            label: format!("{net}:B-x{x:.1}"),
            distance_mm: distance(start, a) + distance(a, b) + distance(b, end) + 1.5,
            segments: vec![
                segment(net, "B.Cu", true, width, start, a),
                segment(net, "B.Cu", true, width, a, b),
                segment(net, "B.Cu", true, width, b, end),
            ],
        });
    }

    candidates
}

fn with_candidate(seed: &RoutingSeed, candidate: &Candidate) -> RoutingSeed {
    let mut seed = seed.clone();
    seed.segments.extend(candidate.segments.clone());
    seed
}

fn read_seed() -> Result<RoutingSeed, Box<dyn Error>> {
    let content = fs::read_to_string(ROUTING_SEED_PATH)?;
    Ok(toml::from_str(&content)?)
}

fn write_seed(seed: &RoutingSeed) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    writeln!(
        output,
        r#"[package]
name = "lamp_rev_a_routing_seed"
revision = "Rev A"
source_stage = "greedy_drc_clean_seed"
source_report = "pcb/lamp_rev_a/reports/drc.json"
segments = {}

"#,
        seed.segments.len()
    )?;
    for route in &seed.segments {
        writeln!(
            output,
            r#"[[segments]]
net = "{}"
layer = "{}"
via_at_ends = {}
width_mm = {:.3}
start_x_mm = {:.3}
start_y_mm = {:.3}
end_x_mm = {:.3}
end_y_mm = {:.3}
"#,
            escape(&route.net),
            escape(&route.layer),
            route.via_at_ends,
            route.width_mm,
            route.start_x_mm,
            route.start_y_mm,
            route.end_x_mm,
            route.end_y_mm
        )?;
    }
    fs::write(ROUTING_SEED_PATH, output)?;
    Ok(())
}

fn read_drc_report() -> Result<DrcReport, Box<dyn Error>> {
    let content = fs::read_to_string(DRC_REPORT_PATH)?;
    Ok(serde_json::from_str(&content)?)
}

fn run_materializer() -> Result<(), Box<dyn Error>> {
    let exe = env::current_exe()?;
    let Some(bin_dir) = exe.parent() else {
        return Err("could not resolve current executable directory".into());
    };
    let materializer = bin_dir.join("lamp_rev_a_materialize_board");
    if !materializer.exists() {
        return Err(format!(
            "{} is missing; run `cargo build --release --bin lamp_rev_a_materialize_board --bin lamp_rev_a_route_greedy` first",
            materializer.display()
        )
        .into());
    }
    run(&mut Command::new(materializer))
}

fn run_drc(kicad_cli: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(DRC_REPORT_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    let status = Command::new(kicad_cli)
        .args([
            "pcb",
            "drc",
            "--refill-zones",
            "--output",
            DRC_REPORT_PATH,
            "--format",
            "json",
            "--severity-all",
            BOARD_PATH,
        ])
        .status()?;

    if !status.success() && !Path::new(DRC_REPORT_PATH).is_file() {
        return Err(format!("kicad-cli DRC failed without writing {DRC_REPORT_PATH}").into());
    }
    Ok(())
}

fn run(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let status = command.status()?;
    if !status.success() {
        return Err(format!("command failed with status {status}: {:?}", command).into());
    }
    Ok(())
}

fn segment(
    net: &str,
    layer: &str,
    via_at_ends: bool,
    width_mm: f64,
    start: Position,
    end: Position,
) -> RouteSegment {
    RouteSegment {
        net: net.to_string(),
        layer: layer.to_string(),
        via_at_ends,
        width_mm,
        start_x_mm: round3(start.x),
        start_y_mm: round3(start.y),
        end_x_mm: round3(end.x),
        end_y_mm: round3(end.y),
    }
}

fn candidate_key(candidate: &Candidate) -> String {
    let mut key = String::new();
    for segment in &candidate.segments {
        let forward = format!(
            "{}:{}:{:.3},{:.3}->{:.3},{:.3};",
            segment.net,
            segment.layer,
            segment.start_x_mm,
            segment.start_y_mm,
            segment.end_x_mm,
            segment.end_y_mm
        );
        let reverse = format!(
            "{}:{}:{:.3},{:.3}->{:.3},{:.3};",
            segment.net,
            segment.layer,
            segment.end_x_mm,
            segment.end_y_mm,
            segment.start_x_mm,
            segment.start_y_mm
        );
        key.push_str(&forward.min(reverse));
    }
    key
}

fn extract_net(description: &str) -> Option<String> {
    let start = description.find('[')? + 1;
    let end = description[start..].find(']')? + start;
    Some(description[start..end].to_string())
}

fn width_for_net(net: &str) -> f64 {
    match net {
        "HEATER_SUPPLY" | "HEATER_P" => 1.50,
        "+12V_RAW" | "+12V" | "+5V" | "+3V3" | "VBUS" | "GND" => 0.50,
        _ => 0.20,
    }
}

fn routing_channel_ys(net: &str) -> &'static [f64] {
    match net {
        "USB_DP" | "USB_DN" | "USB_CC1" | "USB_CC2" | "VBUS" => &[2.0, 7.5, 12.0, 18.0],
        "HEATER_SUPPLY" | "HEATER_P" | "+12V_RAW" | "+12V" => &[12.0, 18.0, 24.0, 32.0, 40.0],
        "SDA" | "SCL" | "MUX_S0" | "MUX_S1" | "MUX_S2" | "ESP_EN" | "ESP_GPIO0" | "UART_TX"
        | "UART_RX" | "HEATER_PWM" => &[28.0, 34.0, 40.0, 46.0, 54.0, 72.0],
        "ADC_AIN1" | "MUX_COM" => &[52.0, 58.0, 64.0, 72.0, 76.0],
        _ => &[12.0, 24.0, 36.0, 48.0, 60.0, 72.0],
    }
}

fn routing_channel_xs(net: &str) -> &'static [f64] {
    match net {
        "USB_DP" | "USB_DN" | "USB_CC1" | "USB_CC2" | "VBUS" => &[3.0, 12.0, 20.0, 30.0],
        "HEATER_SUPPLY" | "HEATER_P" | "+12V_RAW" | "+12V" => &[72.0, 82.0, 92.0, 97.0],
        "SDA" | "SCL" | "MUX_S0" | "MUX_S1" | "MUX_S2" | "ESP_EN" | "ESP_GPIO0" | "UART_TX"
        | "UART_RX" | "HEATER_PWM" => &[18.0, 24.0, 34.0, 42.0, 62.0],
        "ADC_AIN1" | "MUX_COM" => &[38.0, 46.0, 54.0, 62.0],
        _ => &[12.0, 24.0, 36.0, 48.0, 60.0, 72.0, 84.0],
    }
}

fn distance(start: Position, end: Position) -> f64 {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    (dx * dx + dy * dy).sqrt()
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn env_usize(key: &str, default: usize) -> Result<usize, Box<dyn Error>> {
    let Ok(value) = env::var(key) else {
        return Ok(default);
    };
    Ok(value.parse()?)
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
