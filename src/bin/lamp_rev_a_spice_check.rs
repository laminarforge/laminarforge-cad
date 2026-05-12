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

#[derive(Clone, Copy)]
enum SpiceMode {
    OperatingPoint,
    HeaterPwmTransient,
    HeaterThermalTransient,
    UsbInrushStartup,
    RailLoadStep,
    PowerDomainFault,
    AnalogFrontEnd,
}

fn main() -> Result<(), Box<dyn Error>> {
    let repo_root = Path::new(".");
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let (netlist_path, log_path, mode) = match args.as_slice() {
        [] => {
            validate_default(repo_root)?;
            let outputs = default_output_paths(repo_root)?;
            run_spice_check(
                &outputs.spice_netlist,
                &default_log_path(&outputs.spice_netlist),
                SpiceMode::OperatingPoint,
            )?;
            run_spice_check(
                &outputs.heater_pwm_transient_netlist,
                &default_log_path(&outputs.heater_pwm_transient_netlist),
                SpiceMode::HeaterPwmTransient,
            )?;
            run_spice_check(
                &outputs.heater_thermal_transient_netlist,
                &default_log_path(&outputs.heater_thermal_transient_netlist),
                SpiceMode::HeaterThermalTransient,
            )?;
            run_spice_check(
                &outputs.usb_inrush_startup_netlist,
                &default_log_path(&outputs.usb_inrush_startup_netlist),
                SpiceMode::UsbInrushStartup,
            )?;
            run_spice_check(
                &outputs.rail_load_step_netlist,
                &default_log_path(&outputs.rail_load_step_netlist),
                SpiceMode::RailLoadStep,
            )?;
            run_spice_check(
                &outputs.power_domain_fault_netlist,
                &default_log_path(&outputs.power_domain_fault_netlist),
                SpiceMode::PowerDomainFault,
            )?;
            run_spice_check(
                &outputs.analog_front_end_netlist,
                &default_log_path(&outputs.analog_front_end_netlist),
                SpiceMode::AnalogFrontEnd,
            )?;
            println!("LAMP Rev A SPICE checks passed.");
            return Ok(());
        }
        [netlist, log] => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::OperatingPoint,
        ),
        [mode, netlist, log] if mode == "heater-pwm-transient" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::HeaterPwmTransient,
        ),
        [mode, netlist, log] if mode == "heater-thermal-transient" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::HeaterThermalTransient,
        ),
        [mode, netlist, log] if mode == "usb-inrush-startup" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::UsbInrushStartup,
        ),
        [mode, netlist, log] if mode == "rail-load-step" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::RailLoadStep,
        ),
        [mode, netlist, log] if mode == "power-domain-fault" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::PowerDomainFault,
        ),
        [mode, netlist, log] if mode == "analog-front-end" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::AnalogFrontEnd,
        ),
        [mode, netlist, log] if mode == "operating-point" => (
            PathBuf::from(netlist),
            PathBuf::from(log),
            SpiceMode::OperatingPoint,
        ),
        _ => {
            return Err(
                "usage: lamp_rev_a_spice_check [NETLIST_PATH LOG_PATH] | [operating-point|heater-pwm-transient|heater-thermal-transient|usb-inrush-startup|rail-load-step|power-domain-fault|analog-front-end NETLIST_PATH LOG_PATH]"
                    .into(),
            );
        }
    };

    run_spice_check(&netlist_path, &log_path, mode)
}

fn run_spice_check(
    netlist_path: &Path,
    log_path: &Path,
    mode: SpiceMode,
) -> Result<(), Box<dyn Error>> {
    ensure_file(netlist_path)?;
    ensure_parent(log_path)?;

    let output = Command::new("ngspice")
        .arg("-b")
        .arg("-o")
        .arg(log_path)
        .arg(netlist_path)
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

    let log = fs::read_to_string(log_path)?;
    let netlist = fs::read_to_string(netlist_path)?;
    match mode {
        SpiceMode::OperatingPoint => validate_operating_point_log(&log)?,
        SpiceMode::HeaterPwmTransient => validate_heater_pwm_transient_log(&netlist, &log)?,
        SpiceMode::HeaterThermalTransient => validate_heater_thermal_transient_log(&netlist, &log)?,
        SpiceMode::UsbInrushStartup => validate_usb_inrush_startup_log(&netlist, &log)?,
        SpiceMode::RailLoadStep => validate_rail_load_step_log(&netlist, &log)?,
        SpiceMode::PowerDomainFault => validate_power_domain_fault_log(&netlist, &log)?,
        SpiceMode::AnalogFrontEnd => validate_analog_front_end_log(&netlist, &log)?,
    }

    println!(
        "LAMP Rev A {} SPICE check passed.",
        match mode {
            SpiceMode::OperatingPoint => "operating-point",
            SpiceMode::HeaterPwmTransient => "heater PWM transient",
            SpiceMode::HeaterThermalTransient => "heater thermal transient",
            SpiceMode::UsbInrushStartup => "USB/VBUS startup transient",
            SpiceMode::RailLoadStep => "rail load-step transient",
            SpiceMode::PowerDomainFault => "power-domain fault transient",
            SpiceMode::AnalogFrontEnd => "analog front-end transient",
        }
    );
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

fn validate_operating_point_log(log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;
    for bad_token in ["fatal", "singular matrix", "error on line"] {
        debug_assert!(!lower.contains(bad_token));
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

fn validate_heater_pwm_transient_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let min_on_current = netlist_limit(netlist, "check_min_on_current_a")?;
    let max_on_current = netlist_limit(netlist, "check_max_on_current_a")?;
    let max_off_current_a = netlist_limit(netlist, "check_max_off_current_ma")? / 1000.0;
    let min_gate_high = netlist_limit(netlist, "check_min_gate_high_v")?;
    let max_gate_low = netlist_limit(netlist, "check_max_gate_low_v")?;

    let heater_current_on = measurement(&lower, "heater_current_on")?.abs();
    let heater_current_off = measurement(&lower, "heater_current_off")?.abs();
    let gate_high_max = measurement(&lower, "gate_high_max")?;
    let gate_low_min = measurement(&lower, "gate_low_min")?;

    if heater_current_on < min_on_current || heater_current_on > max_on_current {
        return Err(format!(
            "heater PWM transient on-current {heater_current_on:.6} A is outside {min_on_current:.6}..{max_on_current:.6} A"
        )
        .into());
    }
    if heater_current_off > max_off_current_a {
        return Err(format!(
            "heater PWM transient off-current {heater_current_off:.9} A exceeds {max_off_current_a:.9} A"
        )
        .into());
    }
    if gate_high_max < min_gate_high {
        return Err(format!(
            "heater PWM transient gate high {gate_high_max:.6} V is below {min_gate_high:.6} V"
        )
        .into());
    }
    if gate_low_min > max_gate_low {
        return Err(format!(
            "heater PWM transient gate low {gate_low_min:.6} V exceeds {max_gate_low:.6} V"
        )
        .into());
    }
    Ok(())
}

fn validate_heater_thermal_transient_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let target_c = netlist_limit(netlist, "check_target_c")?;
    let max_temperature_c = netlist_limit(netlist, "check_max_temperature_c")?;
    let max_overshoot_c = netlist_limit(netlist, "check_max_overshoot_c")?;
    let max_warmup_s = netlist_limit(netlist, "check_max_warmup_s")?;
    let max_hold_error_c = netlist_limit(netlist, "check_max_hold_error_c")?;
    let min_final_c = netlist_limit(netlist, "check_min_final_c")?;
    let max_final_c = netlist_limit(netlist, "check_max_final_c")?;

    let max_temp = measurement(&lower, "max_temp")?;
    let final_temp = measurement(&lower, "final_temp")?;
    let hold_max = measurement(&lower, "hold_max")?;
    let hold_min = measurement(&lower, "hold_min")?;
    let reached_time = measurement(&lower, "reached_time")?;

    let overshoot_c = (max_temp - target_c).max(0.0);
    let hold_error_c = (hold_max - target_c).abs().max((hold_min - target_c).abs());

    if reached_time > max_warmup_s {
        return Err(format!(
            "heater thermal transient warm-up time {reached_time:.6} s exceeds {max_warmup_s:.6} s"
        )
        .into());
    }
    if max_temp > max_temperature_c {
        return Err(format!(
            "heater thermal transient maximum temperature {max_temp:.6} C exceeds {max_temperature_c:.6} C"
        )
        .into());
    }
    if overshoot_c > max_overshoot_c {
        return Err(format!(
            "heater thermal transient overshoot {overshoot_c:.6} C exceeds {max_overshoot_c:.6} C"
        )
        .into());
    }
    if hold_error_c > max_hold_error_c {
        return Err(format!(
            "heater thermal transient hold error {hold_error_c:.6} C exceeds {max_hold_error_c:.6} C"
        )
        .into());
    }
    if final_temp < min_final_c || final_temp > max_final_c {
        return Err(format!(
            "heater thermal transient final temperature {final_temp:.6} C is outside {min_final_c:.6}..{max_final_c:.6} C"
        )
        .into());
    }
    Ok(())
}

fn validate_usb_inrush_startup_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let max_vbus_inrush_ma = netlist_limit(netlist, "check_max_vbus_inrush_ma")?;
    let max_five_v_inrush_ma = netlist_limit(netlist, "check_max_5v_inrush_ma")?;
    let min_final_three_v_three = netlist_limit(netlist, "check_min_final_3v3_v")?;
    let max_final_three_v_three = netlist_limit(netlist, "check_max_final_3v3_v")?;
    let regulated_three_v_three = netlist_limit(netlist, "check_regulated_3v3_v")?;
    let max_three_v_three_overshoot = netlist_limit(netlist, "check_max_3v3_overshoot_v")?;
    let max_startup_ms = netlist_limit(netlist, "check_max_startup_ms")?;

    let vbus_inrush_ma = measurement(&lower, "vbus_inrush_pos_a")?
        .abs()
        .max(measurement(&lower, "vbus_inrush_neg_a")?.abs())
        * 1000.0;
    let five_v_inrush_ma = measurement(&lower, "five_v_inrush_pos_a")?
        .abs()
        .max(measurement(&lower, "five_v_inrush_neg_a")?.abs())
        * 1000.0;
    let startup_ms = measurement(&lower, "three_v_three_startup_time_s")? * 1000.0;
    let final_three_v_three = measurement(&lower, "three_v_three_final_v")?;
    let max_three_v_three = measurement(&lower, "three_v_three_max_v")?;
    let overshoot_v = (max_three_v_three - regulated_three_v_three).max(0.0);

    if vbus_inrush_ma > max_vbus_inrush_ma {
        return Err(format!(
            "USB startup VBUS inrush {vbus_inrush_ma:.6} mA exceeds {max_vbus_inrush_ma:.6} mA"
        )
        .into());
    }
    if five_v_inrush_ma > max_five_v_inrush_ma {
        return Err(format!(
            "USB startup +5 V inrush {five_v_inrush_ma:.6} mA exceeds {max_five_v_inrush_ma:.6} mA"
        )
        .into());
    }
    if startup_ms > max_startup_ms {
        return Err(format!(
            "USB startup +3V3 valid time {startup_ms:.6} ms exceeds {max_startup_ms:.6} ms"
        )
        .into());
    }
    if final_three_v_three < min_final_three_v_three
        || final_three_v_three > max_final_three_v_three
    {
        return Err(format!(
            "USB startup final +3V3 {final_three_v_three:.6} V is outside {min_final_three_v_three:.6}..{max_final_three_v_three:.6} V"
        )
        .into());
    }
    if overshoot_v > max_three_v_three_overshoot {
        return Err(format!(
            "USB startup +3V3 overshoot {overshoot_v:.6} V exceeds {max_three_v_three_overshoot:.6} V"
        )
        .into());
    }
    Ok(())
}

fn validate_rail_load_step_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let min_rail_voltage = netlist_limit(netlist, "check_min_rail_voltage_v")?;
    let max_source_current_ma = netlist_limit(netlist, "check_max_source_current_ma")?;

    let rail_min_v = measurement(&lower, "rail_min_v")?;
    let source_current_max_ma = measurement(&lower, "source_current_max")?.abs() * 1000.0;
    let rail_recovery_v = measurement(&lower, "rail_recovery_v")?;

    if rail_min_v < min_rail_voltage {
        return Err(format!(
            "rail load-step minimum voltage {rail_min_v:.6} V is below {min_rail_voltage:.6} V"
        )
        .into());
    }
    if rail_recovery_v < min_rail_voltage {
        return Err(format!(
            "rail load-step recovery voltage {rail_recovery_v:.6} V is below {min_rail_voltage:.6} V"
        )
        .into());
    }
    if source_current_max_ma > max_source_current_ma {
        return Err(format!(
            "rail load-step source current {source_current_max_ma:.6} mA exceeds {max_source_current_ma:.6} mA"
        )
        .into());
    }
    Ok(())
}

fn validate_power_domain_fault_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let max_vbus_fault = netlist_limit(netlist, "check_max_vbus_fault_v")?;
    let max_five_v_fault = netlist_limit(netlist, "check_max_5v_fault_v")?;
    let max_three_v_three_fault = netlist_limit(netlist, "check_max_3v3_fault_v")?;
    let max_usb_backfeed_ma = netlist_limit(netlist, "check_max_usb_backfeed_ma")?;
    let max_regulator_reverse_ma = netlist_limit(netlist, "check_max_regulator_reverse_ma")?;

    let vbus_fault_v = measurement(&lower, "vbus_fault_v")?;
    let five_v_fault_v = measurement(&lower, "five_v_fault_v")?;
    let three_v_three_fault_v = measurement(&lower, "three_v_three_fault_v")?;
    let usb_backfeed_ma = measurement(&lower, "usb_backfeed_a")?.abs() * 1000.0;
    let regulator_reverse_ma = measurement(&lower, "regulator_reverse_a")?.abs() * 1000.0;

    if vbus_fault_v > max_vbus_fault {
        return Err(format!(
            "power-domain fault VBUS voltage {vbus_fault_v:.6} V exceeds {max_vbus_fault:.6} V"
        )
        .into());
    }
    if five_v_fault_v > max_five_v_fault {
        return Err(format!(
            "power-domain fault +5V voltage {five_v_fault_v:.6} V exceeds {max_five_v_fault:.6} V"
        )
        .into());
    }
    if three_v_three_fault_v > max_three_v_three_fault {
        return Err(format!(
            "power-domain fault +3V3 voltage {three_v_three_fault_v:.6} V exceeds {max_three_v_three_fault:.6} V"
        )
        .into());
    }
    if usb_backfeed_ma > max_usb_backfeed_ma {
        return Err(format!(
            "power-domain fault USB backfeed {usb_backfeed_ma:.6} mA exceeds {max_usb_backfeed_ma:.6} mA"
        )
        .into());
    }
    if regulator_reverse_ma > max_regulator_reverse_ma {
        return Err(format!(
            "power-domain fault regulator reverse current {regulator_reverse_ma:.6} mA exceeds {max_regulator_reverse_ma:.6} mA"
        )
        .into());
    }
    Ok(())
}

fn validate_analog_front_end_log(netlist: &str, log: &str) -> Result<(), Box<dyn Error>> {
    let lower = log.to_ascii_lowercase();
    validate_no_ngspice_failure_tokens(&lower)?;

    let min_signal_delta = netlist_limit(netlist, "check_min_signal_delta_v")?;
    let min_adc_voltage = netlist_limit(netlist, "check_min_adc_voltage_v")?;
    let max_adc_voltage = netlist_limit(netlist, "check_max_adc_voltage_v")?;

    let dark_v = measurement(&lower, "dark_v")?;
    let light_v = measurement(&lower, "light_v")?;
    let adc_max_v = measurement(&lower, "adc_max_v")?;
    let adc_min_v = measurement(&lower, "adc_min_v")?;
    let signal_delta = light_v - dark_v;

    if signal_delta < min_signal_delta {
        return Err(format!(
            "analog front-end signal delta {signal_delta:.6} V is below {min_signal_delta:.6} V"
        )
        .into());
    }
    if adc_min_v < min_adc_voltage {
        return Err(format!(
            "analog front-end ADC minimum {adc_min_v:.6} V is below {min_adc_voltage:.6} V"
        )
        .into());
    }
    if adc_max_v > max_adc_voltage {
        return Err(format!(
            "analog front-end ADC maximum {adc_max_v:.6} V exceeds {max_adc_voltage:.6} V"
        )
        .into());
    }
    Ok(())
}

fn validate_no_ngspice_failure_tokens(lower_log: &str) -> Result<(), Box<dyn Error>> {
    for bad_token in ["fatal", "singular matrix", "error on line", "failed"] {
        if lower_log.contains(bad_token) {
            return Err(format!("ngspice log contains failure token `{bad_token}`").into());
        }
    }
    Ok(())
}

fn netlist_limit(netlist: &str, key: &str) -> Result<f64, Box<dyn Error>> {
    let prefix = format!("* {key}=");
    for line in netlist.lines() {
        if let Some(value) = line.trim().strip_prefix(&prefix) {
            return value.trim().parse::<f64>().map_err(|err| {
                format!("invalid numeric SPICE check limit `{key}` value `{value}`: {err}").into()
            });
        }
    }
    Err(format!("SPICE netlist is missing check limit `{key}`").into())
}

fn measurement(lower_log: &str, name: &str) -> Result<f64, Box<dyn Error>> {
    for line in lower_log.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(name) {
            if let Some(value) = rest
                .split('=')
                .nth(1)
                .and_then(|rhs| rhs.split_whitespace().next())
            {
                return value.parse::<f64>().map_err(|err| {
                    format!("invalid ngspice measurement `{name}` value `{value}`: {err}").into()
                });
            }
        }
    }
    Err(format!("ngspice log is missing transient measurement `{name}`").into())
}
