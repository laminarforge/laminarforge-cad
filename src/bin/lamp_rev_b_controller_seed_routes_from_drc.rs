use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

const DRC_REPORT_PATH: &str = "pcb/lamp_rev_b_controller/reports/drc.json";
const ROUTING_SEED_PATH: &str = "pcb/lamp_rev_b_controller/routing_seed.toml";
const MAX_LOCAL_TOP_ROUTE_MM: f64 = 4.0;

#[derive(Debug, Deserialize, Clone)]
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
    #[serde(default)]
    uuid: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Route {
    net: String,
    layer: &'static str,
    via_at_ends: bool,
    width_mm: f64,
    start: Position,
    end: Position,
}

fn main() -> Result<(), Box<dyn Error>> {
    let kicad_cli = env::var("KICAD_CLI").unwrap_or_else(|_| "/opt/homebrew/bin/kicad-cli".into());
    let mut accepted: Vec<Route> = Vec::new();

    write_seed(&accepted, "drc_clean_local_top_seed")?;
    run_materializer()?;
    run_drc(&kicad_cli)?;
    let mut baseline = read_drc_report()?;
    if !baseline.violations.is_empty() {
        return Err(format!(
            "refusing to seed routes while clean baseline has {} physical DRC violations",
            baseline.violations.len()
        )
        .into());
    }
    let starting_unconnected = active_unconnected_count(&baseline);
    let candidates = build_candidates(&baseline);

    println!("LAMP Rev B deterministic local route seeding");
    println!("  starting unconnected: {starting_unconnected}");
    println!("  candidate local F.Cu segments: {}", candidates.len());

    let mut trials = 0usize;
    for candidate in candidates {
        trials += 1;
        let mut trial_routes = accepted.clone();
        trial_routes.push(candidate.clone());
        write_seed(&trial_routes, "drc_clean_local_top_seed_trial")?;
        run_materializer()?;
        run_drc(&kicad_cli)?;
        let trial_report = read_drc_report()?;
        let trial_unconnected = active_unconnected_count(&trial_report);
        let baseline_unconnected = active_unconnected_count(&baseline);
        if trial_report.violations.is_empty() && trial_unconnected < baseline_unconnected {
            let removed = baseline_unconnected - trial_unconnected;
            println!(
                "  accepted {}: -{} unconnected ({} -> {})",
                candidate.label(),
                removed,
                baseline_unconnected,
                trial_unconnected
            );
            accepted = trial_routes;
            baseline = trial_report;
        } else {
            write_seed(&accepted, "drc_clean_local_top_seed")?;
        }
    }

    write_seed(&accepted, "drc_clean_local_top_seed")?;
    run_materializer()?;
    run_drc(&kicad_cli)?;
    let final_report = read_drc_report()?;
    if !final_report.violations.is_empty() {
        return Err(format!(
            "accepted route set ended with {} DRC violations",
            final_report.violations.len()
        )
        .into());
    }

    println!("  accepted routes: {}", accepted.len());
    println!("  candidate trials: {trials}");
    println!(
        "  final unconnected: {}",
        active_unconnected_count(&final_report)
    );
    Ok(())
}

fn build_candidates(report: &DrcReport) -> Vec<Route> {
    let mut seen = BTreeSet::new();
    let mut routes = Vec::new();
    for entry in &report.unconnected_items {
        if is_self_zone_unconnected(entry) {
            continue;
        }
        let [start, end] = entry.items.as_slice() else {
            continue;
        };
        let Some(net) = extract_net(&start.description) else {
            continue;
        };
        if extract_net(&end.description).as_deref() != Some(net.as_str()) {
            continue;
        }
        let route = Route {
            width_mm: width_for_net(&net),
            via_at_ends: false,
            layer: "F.Cu",
            net,
            start: round_position(start.pos),
            end: round_position(end.pos),
        };
        if route.length_mm() < 0.05 || route.length_mm() > MAX_LOCAL_TOP_ROUTE_MM {
            continue;
        }
        if seen.insert(route.key()) {
            routes.push(route);
        }
    }
    routes.sort_by(|a, b| {
        a.length_mm()
            .total_cmp(&b.length_mm())
            .then_with(|| a.net.cmp(&b.net))
            .then_with(|| a.key().cmp(&b.key()))
    });
    routes
}

fn write_seed(routes: &[Route], source_stage: &str) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    output.push_str(
        r#"[package]
name = "lamp_rev_b_controller_routing_seed"
ticket = "T-49FD0ECC"
revision = "Rev B prototype"
"#,
    );
    writeln!(output, "source_stage = \"{source_stage}\"")?;
    output.push_str(
        r#"source_report = "pcb/lamp_rev_b_controller/reports/drc.json"
max_local_top_route_mm = 4.0
"#,
    );
    writeln!(output, "segments = {}", routes.len())?;
    output.push('\n');

    for route in routes {
        writeln!(
            output,
            r#"
[[segments]]
net = "{}"
layer = "{}"
via_at_ends = {}
width_mm = {:.3}
start_x_mm = {:.3}
start_y_mm = {:.3}
end_x_mm = {:.3}
end_y_mm = {:.3}"#,
            escape(&route.net),
            route.layer,
            route.via_at_ends,
            route.width_mm,
            route.start.x,
            route.start.y,
            route.end.x,
            route.end.y
        )?;
    }

    fs::write(Path::new(ROUTING_SEED_PATH), output)?;
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
    let materializer = bin_dir.join("lamp_rev_b_controller_materialize_board");
    if !materializer.exists() {
        return Err(format!(
            "{} is missing; build lamp_rev_b_controller_materialize_board first",
            materializer.display()
        )
        .into());
    }
    let status = Command::new(materializer).status()?;
    if !status.success() {
        return Err(format!("lamp_rev_b_controller_materialize_board failed with {status}").into());
    }
    Ok(())
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
            "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_pcb",
        ])
        .status()?;
    if !status.success() && !Path::new(DRC_REPORT_PATH).is_file() {
        return Err(format!("kicad-cli DRC failed without writing {DRC_REPORT_PATH}").into());
    }
    Ok(())
}

impl Route {
    fn length_mm(&self) -> f64 {
        distance(self.start, self.end)
    }

    fn key(&self) -> String {
        let forward = format!(
            "{}:{}:{:.3},{:.3}->{:.3},{:.3}",
            self.net, self.layer, self.start.x, self.start.y, self.end.x, self.end.y
        );
        let reverse = format!(
            "{}:{}:{:.3},{:.3}->{:.3},{:.3}",
            self.net, self.layer, self.end.x, self.end.y, self.start.x, self.start.y
        );
        forward.min(reverse)
    }

    fn label(&self) -> String {
        format!(
            "{} {} {:.3},{:.3}->{:.3},{:.3}",
            self.net, self.layer, self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}

fn active_unconnected_count(report: &DrcReport) -> usize {
    report
        .unconnected_items
        .iter()
        .filter(|entry| !is_self_zone_unconnected(entry))
        .count()
}

fn width_for_net(net: &str) -> f64 {
    match net {
        "VIN_12_24" | "VIN_PROTECTED" | "VIN_HEATER" | "VDRV" | "LED_SUPPLY" | "LED_DRV_PLUS"
        | "LED_PLUS" | "LED_MINUS" | "HEATER0_LOW" | "HEATER1_LOW" | "GND" | "GND_EP" => 0.50,
        "+5V" | "+3V3" | "+3V3_ANA" | "VBUS" | "3V3_SW" => 0.35,
        "USB_DP" | "USB_DN" => 0.20,
        _ => 0.20,
    }
}

fn distance(start: Position, end: Position) -> f64 {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    (dx * dx + dy * dy).sqrt()
}

fn round_position(pos: Position) -> Position {
    Position {
        x: round3(pos.x),
        y: round3(pos.y),
    }
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn extract_net(description: &str) -> Option<String> {
    let start = description.find('[')? + 1;
    let end = description[start..].find(']')? + start;
    Some(description[start..end].to_string())
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

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
