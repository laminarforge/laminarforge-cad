use laminarforge_cad::lamp_rev_a_electrical::{default_output_paths, validate_default};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REQUIRED_OUTPUT_TOKENS: &[&str] = &[
    "vbus",
    "p5v",
    "p3v3",
    "p12raw",
    "p12",
    "heater_supply",
    "heater_p",
    "vusb#branch",
    "v3v3#branch",
    "v12#branch",
];

fn main() -> Result<(), Box<dyn Error>> {
    let repo_root = Path::new(".");
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let (netlist_path, log_path) = match args.as_slice() {
        [] => {
            validate_default(repo_root)?;
            let outputs = default_output_paths(repo_root)?;
            let log_path = default_log_path(&outputs.spice_netlist);
            (outputs.spice_netlist, log_path)
        }
        [netlist, log] => (PathBuf::from(netlist), PathBuf::from(log)),
        _ => {
            return Err("usage: lamp_rev_a_spice_check [NETLIST_PATH LOG_PATH]".into());
        }
    };

    ensure_file(&netlist_path)?;
    ensure_parent(&log_path)?;

    let output = Command::new("ngspice")
        .arg("-b")
        .arg("-o")
        .arg(&log_path)
        .arg(&netlist_path)
        .output()
        .map_err(|err| {
            format!(
                "ngspice is required for LAMP Rev A SPICE validation and could not be started: {err}"
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "ngspice failed for {} with status {:?}\nstdout:\n{}\nstderr:\n{}",
            netlist_path.display(),
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let log = fs::read_to_string(&log_path)?;
    validate_ngspice_log(&log)?;

    println!("LAMP Rev A SPICE operating-point check passed.");
    println!("  netlist: {}", netlist_path.display());
    println!("  log: {}", log_path.display());
    Ok(())
}

fn default_log_path(netlist_path: &Path) -> PathBuf {
    let mut log_path = netlist_path.to_path_buf();
    log_path.set_extension("ngspice.log");
    log_path
}

fn ensure_file(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.is_file() {
        return Err(format!("required SPICE input is missing: {}", path.display()).into());
    }
    if fs::metadata(path)?.len() == 0 {
        return Err(format!("required SPICE input is empty: {}", path.display()).into());
    }
    Ok(())
}

fn ensure_parent(path: &Path) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn validate_ngspice_log(log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    for bad_token in ["fatal", "singular matrix", "error on line"] {
        if lower.contains(bad_token) {
            return Err(format!("ngspice log contains failure token `{bad_token}`").into());
        }
    }
    for token in REQUIRED_OUTPUT_TOKENS {
        if !lower.contains(token) {
            return Err(format!("ngspice log is missing operating-point token `{token}`").into());
        }
    }
    if !lower.contains("no. of data rows") {
        return Err("ngspice log does not contain an operating-point data table".into());
    }
    Ok(())
}
