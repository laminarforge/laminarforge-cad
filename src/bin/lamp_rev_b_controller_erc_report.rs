use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const SCHEMATIC_PATH: &str = "pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_sch";
const ERC_REPORT_PATH: &str = "pcb/lamp_rev_b_controller/reports/erc.json";

#[derive(Debug, Deserialize)]
struct ErcReport {
    #[serde(default)]
    violations: Vec<ErcEntry>,
}

#[derive(Debug, Deserialize)]
struct ErcEntry {
    #[serde(default)]
    description: String,
    #[serde(default)]
    severity: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    refresh_erc_report()?;
    let content = fs::read_to_string(ERC_REPORT_PATH)?;
    let report: ErcReport = serde_json::from_str(&content)?;

    println!("LAMP Rev B controller ERC report");
    println!("  ERC violations: {}", report.violations.len());
    for violation in &report.violations {
        println!(
            "  - [{}] {}",
            violation.severity,
            violation.description.trim()
        );
    }

    if !report.violations.is_empty() {
        return Err("KiCad ERC has violations; fix captured connectivity before release".into());
    }

    Ok(())
}

fn refresh_erc_report() -> Result<(), Box<dyn Error>> {
    let report_path = Path::new(ERC_REPORT_PATH);
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let args = [
        "sch",
        "erc",
        SCHEMATIC_PATH,
        "-o",
        ERC_REPORT_PATH,
        "--format",
        "json",
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
