use serde::Deserialize;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

const DRC_REPORT_PATH: &str = "pcb/lamp_rev_a/reports/drc.json";

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

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(DRC_REPORT_PATH);
    let content = fs::read_to_string(path).map_err(|error| {
        format!(
            "read {DRC_REPORT_PATH}: {error}; run `kicad-cli pcb drc --format json --output {DRC_REPORT_PATH} pcb/lamp_rev_a/lamp_rev_a.kicad_pcb` first"
        )
    })?;
    let report: DrcReport = serde_json::from_str(&content)?;
    let active_unconnected = report
        .unconnected_items
        .iter()
        .filter(|item| !is_self_zone_unconnected(item))
        .collect::<Vec<_>>();
    let ignored_self_zone_items = report.unconnected_items.len() - active_unconnected.len();

    let mut by_net: BTreeMap<String, usize> = BTreeMap::new();
    for item in &active_unconnected {
        if let Some(net) = item
            .items
            .iter()
            .find_map(|item| extract_net(&item.description))
        {
            *by_net.entry(net).or_default() += 1;
        }
    }

    println!("LAMP Rev A routing report");
    println!("  DRC violations: {}", report.violations.len());
    println!("  Unconnected items: {}", active_unconnected.len());
    if ignored_self_zone_items > 0 {
        println!("  Ignored KiCad self-zone items: {ignored_self_zone_items}");
    }
    println!();
    println!("Unconnected items by net:");
    let mut rows = by_net.iter().collect::<Vec<_>>();
    rows.sort_by(|(net_a, count_a), (net_b, count_b)| {
        count_b.cmp(count_a).then_with(|| net_a.cmp(net_b))
    });
    for (net, count) in rows {
        println!("  {net}: {count}");
    }

    if !report.violations.is_empty() {
        return Err("KiCad DRC has physical violations; fix placement/rules before routing".into());
    }

    Ok(())
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
