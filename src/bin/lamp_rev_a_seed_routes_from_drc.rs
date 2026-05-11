use serde::Deserialize;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

const DRC_REPORT_PATH: &str = "pcb/lamp_rev_a/reports/drc.json";
const ROUTING_SEED_PATH: &str = "pcb/lamp_rev_a/routing_seed.toml";
const MAX_DIRECT_SEED_LENGTH_MM: f64 = 4.0;

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
    pos: Position,
}

#[derive(Debug, Deserialize)]
struct Position {
    x: f64,
    y: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(DRC_REPORT_PATH).map_err(|error| {
        format!("read {DRC_REPORT_PATH}: {error}; run KiCad DRC before seeding routes")
    })?;
    let report: DrcReport = serde_json::from_str(&content)?;
    if !report.violations.is_empty() {
        return Err(format!(
            "refusing to seed routes while KiCad reports {} physical DRC violations",
            report.violations.len()
        )
        .into());
    }

    let mut seen = BTreeSet::new();
    let mut routes = Vec::new();
    for entry in &report.unconnected_items {
        let [start, end] = entry.items.as_slice() else {
            continue;
        };
        let Some(net) = extract_net(&start.description) else {
            continue;
        };
        if extract_net(&end.description).as_deref() != Some(net.as_str()) {
            return Err(format!(
                "unconnected item has mismatched nets: {} <-> {}",
                start.description, end.description
            )
            .into());
        }
        if !is_seedable_direct_route(&net, &start.description, &end.description) {
            continue;
        }

        let route = Route {
            net,
            layer: "F.Cu",
            width_mm: width_for_net(&start.description),
            start_x_mm: round3(start.pos.x),
            start_y_mm: round3(start.pos.y),
            end_x_mm: round3(end.pos.x),
            end_y_mm: round3(end.pos.y),
        };
        if route.length_mm() > MAX_DIRECT_SEED_LENGTH_MM {
            continue;
        }
        let key = route.key();
        if seen.insert(key) {
            routes.push(route);
        }
    }

    let mut output = String::new();
    output.push_str(
        r#"[package]
name = "lamp_rev_a_routing_seed"
revision = "Rev A"
source_stage = "drc_unconnected_seed"
source_report = "pcb/lamp_rev_a/reports/drc.json"
max_direct_seed_length_mm = 4.0

"#,
    );
    for route in &routes {
        writeln!(
            output,
            r#"[[segments]]
net = "{}"
layer = "{}"
width_mm = {:.3}
start_x_mm = {:.3}
start_y_mm = {:.3}
end_x_mm = {:.3}
end_y_mm = {:.3}
"#,
            escape(&route.net),
            route.layer,
            route.width_mm,
            route.start_x_mm,
            route.start_y_mm,
            route.end_x_mm,
            route.end_y_mm
        )?;
    }

    fs::write(Path::new(ROUTING_SEED_PATH), output)?;
    println!("Wrote {ROUTING_SEED_PATH}");
    println!("  route segments: {}", routes.len());
    Ok(())
}

struct Route {
    net: String,
    layer: &'static str,
    width_mm: f64,
    start_x_mm: f64,
    start_y_mm: f64,
    end_x_mm: f64,
    end_y_mm: f64,
}

impl Route {
    fn length_mm(&self) -> f64 {
        let dx = self.end_x_mm - self.start_x_mm;
        let dy = self.end_y_mm - self.start_y_mm;
        (dx * dx + dy * dy).sqrt()
    }

    fn key(&self) -> String {
        let forward = format!(
            "{}:{:.3},{:.3}->{:.3},{:.3}",
            self.net, self.start_x_mm, self.start_y_mm, self.end_x_mm, self.end_y_mm
        );
        let reverse = format!(
            "{}:{:.3},{:.3}->{:.3},{:.3}",
            self.net, self.end_x_mm, self.end_y_mm, self.start_x_mm, self.start_y_mm
        );
        forward.min(reverse)
    }
}

fn extract_net(description: &str) -> Option<String> {
    let start = description.find('[')? + 1;
    let end = description[start..].find(']')? + start;
    Some(description[start..end].to_string())
}

fn width_for_net(description: &str) -> f64 {
    let Some(net) = extract_net(description) else {
        return 0.20;
    };
    match net.as_str() {
        "HEATER_SUPPLY" | "HEATER_P" => 1.50,
        "+12V_RAW" | "+12V" | "+5V" | "+3V3" | "VBUS" | "GND" => 0.50,
        _ => 0.20,
    }
}

fn is_seedable_direct_route(net: &str, start: &str, end: &str) -> bool {
    if matches!(net, "USB_DP" | "USB_DN") && start.contains("of J1") && end.contains("of J1") {
        return false;
    }
    if net == "GND" && start.contains("of U4") && end.contains("of U4") {
        return false;
    }
    true
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
