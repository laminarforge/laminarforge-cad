use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const SCHEMATIC_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_sch";
const ERC_REPORT_PATH: &str = "pcb/lamp_rev_b_controller/reports/erc.json";
const FAB_RELEASE_PATH: &str = "pcb/lamp_rev_b_controller/fab_release.toml";

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

#[derive(Debug, Deserialize)]
struct FabRelease {
    erc_review: ErcReview,
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

fn main() -> Result<(), Box<dyn Error>> {
    refresh_erc_report()?;
    let report: ErcReport = serde_json::from_str(&fs::read_to_string(ERC_REPORT_PATH)?)?;
    let release: FabRelease = toml::from_str(&fs::read_to_string(FAB_RELEASE_PATH)?)?;
    validate_acceptance_rules(&release.erc_review.accepted_findings)?;

    let violations = report
        .sheets
        .iter()
        .flat_map(|sheet| sheet.violations.iter())
        .collect::<Vec<_>>();
    let raw_count = violations.len();
    let (accepted, blocking): (Vec<&ErcEntry>, Vec<&ErcEntry>) = violations
        .into_iter()
        .partition(|entry| accepted_rule(entry, &release.erc_review.accepted_findings).is_some());

    println!("LAMP Rev B controller ERC report");
    println!("  raw findings: {raw_count}");
    println!("  accepted reviewed findings: {}", accepted.len());
    println!("  blocking electrical findings: {}", blocking.len());

    if !accepted.is_empty() {
        println!("\nAccepted reviewed findings:");
        for rule in &release.erc_review.accepted_findings {
            let count = accepted
                .iter()
                .filter(|entry| accepted_rule(entry, std::slice::from_ref(rule)).is_some())
                .count();
            if count > 0 {
                println!(
                    "  - [{}] {} x{}: {}",
                    rule.severity, rule.violation_type, count, rule.reason
                );
            }
        }
    }

    if !blocking.is_empty() {
        println!("\nBlocking electrical findings:");
        for entry in &blocking {
            println!(
                "  - [{}] {}: {} ({})",
                entry.severity,
                entry.violation_type,
                entry.description.trim(),
                entry
                    .items
                    .iter()
                    .map(|item| item.description.as_str())
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }
        return Err(format!(
            "KiCad ERC has {} blocking electrical findings after reviewed exceptions",
            blocking.len()
        )
        .into());
    }

    Ok(())
}

fn validate_acceptance_rules(rules: &[AcceptedFinding]) -> Result<(), Box<dyn Error>> {
    for rule in rules {
        match (
            rule.item_description_exact.as_ref(),
            rule.item_description_contains.as_ref(),
        ) {
            (Some(_), Some(_)) => {
                return Err(format!(
                    "ERC acceptance rule {} must use exactly one item matcher",
                    rule.violation_type
                )
                .into());
            }
            (None, None) => {
                return Err(format!(
                    "ERC acceptance rule {} must constrain the affected item",
                    rule.violation_type
                )
                .into());
            }
            _ => {}
        }
    }
    Ok(())
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

fn refresh_erc_report() -> Result<(), Box<dyn Error>> {
    let report_path = Path::new(ERC_REPORT_PATH);
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let args = [
        "sch",
        "erc",
        "--output",
        ERC_REPORT_PATH,
        "--format",
        "json",
        "--severity-all",
        SCHEMATIC_PATH,
    ];
    let output = Command::new("kicad-cli").args(args).output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "failed to refresh KiCad ERC report with kicad-cli\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(violation_type: &str, item: &str) -> ErcEntry {
        ErcEntry {
            description: "reviewed description".to_string(),
            severity: "warning".to_string(),
            violation_type: violation_type.to_string(),
            items: vec![ErcItem {
                description: item.to_string(),
            }],
        }
    }

    #[test]
    fn acceptance_is_scoped_to_exact_rule_and_item_match() {
        let rules = vec![AcceptedFinding {
            violation_type: "lib_symbol_issues".to_string(),
            severity: "warning".to_string(),
            description: "reviewed description".to_string(),
            item_description_exact: None,
            item_description_contains: Some("[LF_CAPTURE_".to_string()),
            reason: "generated embedded symbol".to_string(),
        }];

        assert!(accepted_rule(
            &finding("lib_symbol_issues", "Symbol U1 [LF_CAPTURE_U1]"),
            &rules
        )
        .is_some());
        assert!(accepted_rule(&finding("pin_not_connected", "Symbol U1 Pin 1"), &rules).is_none());
        assert!(accepted_rule(
            &finding("lib_symbol_issues", "Symbol U1 [external]"),
            &rules
        )
        .is_none());
    }
}
