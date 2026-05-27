use serde::Serialize;

const INNER_X_M: f64 = 0.300;
const INNER_Y_M: f64 = 0.250;
const INNER_Z_M: f64 = 0.250;
const CHAMBER_VOLUME_L: f64 = INNER_X_M * INNER_Y_M * INNER_Z_M * 1000.0;
const CHAMBER_AREA_M2: f64 =
    2.0 * (INNER_X_M * INNER_Y_M + INNER_X_M * INNER_Z_M + INNER_Y_M * INNER_Z_M);

const AMBIENT_C: f64 = 23.0;
const TARGET_TEMP_C: f64 = 37.0;
const TARGET_CO2_PERCENT: f64 = 5.0;

const SIM_SECONDS: usize = 8 * 60 * 60;
const DT_S: f64 = 1.0;

#[derive(Debug, Clone, Copy)]
struct SimConfig {
    heater_max_w: f64,
    insulation_thickness_m: f64,
    insulation_k_w_mk: f64,
    parasitic_ua_w_k: f64,
    effective_thermal_mass_j_k: f64,
    co2_injection_l_min: f64,
    leak_exchange_l_min: f64,
    door_open_start_s: usize,
    door_open_duration_s: usize,
    door_open_exchange_l_min: f64,
}

#[derive(Debug, Default, Clone, Copy)]
struct Controller {
    temp_integral: f64,
}

#[derive(Debug, Clone, Copy)]
struct State {
    temp_c: f64,
    co2_percent: f64,
}

#[derive(Debug, Serialize)]
struct SimReport {
    chamber_volume_l: f64,
    chamber_area_m2: f64,
    ua_w_k: f64,
    warmup_time_min_to_36_5c: Option<f64>,
    max_temp_c: f64,
    min_temp_after_warmup_c: f64,
    final_temp_c: f64,
    max_co2_percent: f64,
    min_co2_after_warmup_percent: f64,
    co2_recovery_min_after_door_close: Option<f64>,
    final_co2_percent: f64,
    heater_energy_wh: f64,
    co2_used_l: f64,
    pass: bool,
    failures: Vec<String>,
}

fn main() {
    let report = simulate(default_config());

    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("report serializes")
    );

    if !report.pass {
        std::process::exit(1);
    }
}

fn default_config() -> SimConfig {
    SimConfig {
        heater_max_w: 80.0,
        insulation_thickness_m: 0.025,
        insulation_k_w_mk: 0.022,
        parasitic_ua_w_k: 0.20,
        // Air is only ~23 J/K; this includes shelves, tray water, liner, and
        // cultureware as a first-order thermal buffer estimate.
        effective_thermal_mass_j_k: 4200.0,
        co2_injection_l_min: 0.25,
        leak_exchange_l_min: 0.025,
        door_open_start_s: 2 * 60 * 60,
        door_open_duration_s: 60,
        door_open_exchange_l_min: 22.0,
    }
}

fn simulate(config: SimConfig) -> SimReport {
    let ua_w_k = chamber_ua_w_k(config);
    let mut state = State {
        temp_c: AMBIENT_C,
        co2_percent: 0.04,
    };
    let mut controller = Controller::default();

    let mut warmup_time_min_to_36_5c = None;
    let mut max_temp_c = state.temp_c;
    let mut min_temp_after_warmup_c = f64::INFINITY;
    let mut max_co2_percent = state.co2_percent;
    let mut min_co2_after_warmup_percent = f64::INFINITY;
    let mut co2_recovery_min_after_door_close = None;
    let mut heater_energy_wh = 0.0;
    let mut co2_used_l = 0.0;
    let door_close_s = config.door_open_start_s + config.door_open_duration_s;

    for second in 0..SIM_SECONDS {
        let door_open = second >= config.door_open_start_s
            && second < config.door_open_start_s + config.door_open_duration_s;
        let heater_w = heater_command_w(&mut controller, state.temp_c, config.heater_max_w);
        let co2_on = state.co2_percent < TARGET_CO2_PERCENT - 0.08;

        let exchange_l_min = if door_open {
            config.door_open_exchange_l_min
        } else {
            config.leak_exchange_l_min
        };

        state = step_state(state, config, ua_w_k, heater_w, co2_on, exchange_l_min);

        if warmup_time_min_to_36_5c.is_none() && state.temp_c >= 36.5 {
            warmup_time_min_to_36_5c = Some(second as f64 / 60.0);
        }
        if warmup_time_min_to_36_5c.is_some() {
            min_temp_after_warmup_c = min_temp_after_warmup_c.min(state.temp_c);
            min_co2_after_warmup_percent = min_co2_after_warmup_percent.min(state.co2_percent);
        }
        if second >= door_close_s
            && co2_recovery_min_after_door_close.is_none()
            && (4.8..=5.2).contains(&state.co2_percent)
        {
            co2_recovery_min_after_door_close = Some((second - door_close_s) as f64 / 60.0);
        }
        max_temp_c = max_temp_c.max(state.temp_c);
        max_co2_percent = max_co2_percent.max(state.co2_percent);
        heater_energy_wh += heater_w * DT_S / 3600.0;
        if co2_on {
            co2_used_l += config.co2_injection_l_min / 60.0 * DT_S;
        }
    }

    let mut failures = Vec::new();
    match warmup_time_min_to_36_5c {
        Some(minutes) if minutes <= 45.0 => {}
        Some(minutes) => failures.push(format!(
            "warmup took {minutes:.1} min to reach 36.5 C; limit is 45 min"
        )),
        None => failures.push("never reached 36.5 C".to_string()),
    }
    if max_temp_c > 38.0 {
        failures.push(format!(
            "temperature overshoot reached {max_temp_c:.2} C; limit is 38.0 C"
        ));
    }
    if !(36.8..=37.2).contains(&state.temp_c) {
        failures.push(format!(
            "final temperature is {:.2} C; expected 36.8-37.2 C",
            state.temp_c
        ));
    }
    if max_co2_percent > TARGET_CO2_PERCENT + 0.6 {
        failures.push(format!(
            "CO2 overshoot reached {max_co2_percent:.2}%; limit is 5.6%"
        ));
    }
    if !(4.8..=5.2).contains(&state.co2_percent) {
        failures.push(format!(
            "final CO2 is {:.2}%; expected 4.8-5.2%",
            state.co2_percent
        ));
    }
    if co2_used_l > 10.0 {
        failures.push(format!(
            "CO2 use is {co2_used_l:.2} L over 8 h; limit is 10 L before leak/gas design review"
        ));
    }
    match co2_recovery_min_after_door_close {
        Some(minutes) if minutes <= 20.0 => {}
        Some(minutes) => failures.push(format!(
            "CO2 took {minutes:.1} min to recover after door close; limit is 20 min"
        )),
        None => failures.push("CO2 never recovered to 4.8-5.2% after door close".to_string()),
    }

    SimReport {
        chamber_volume_l: CHAMBER_VOLUME_L,
        chamber_area_m2: CHAMBER_AREA_M2,
        ua_w_k,
        warmup_time_min_to_36_5c,
        max_temp_c,
        min_temp_after_warmup_c,
        final_temp_c: state.temp_c,
        max_co2_percent,
        min_co2_after_warmup_percent,
        co2_recovery_min_after_door_close,
        final_co2_percent: state.co2_percent,
        heater_energy_wh,
        co2_used_l,
        pass: failures.is_empty(),
        failures,
    }
}

fn chamber_ua_w_k(config: SimConfig) -> f64 {
    CHAMBER_AREA_M2 * config.insulation_k_w_mk / config.insulation_thickness_m
        + config.parasitic_ua_w_k
}

fn heater_command_w(controller: &mut Controller, temp_c: f64, heater_max_w: f64) -> f64 {
    let error = TARGET_TEMP_C - temp_c;
    controller.temp_integral = (controller.temp_integral + error * DT_S).clamp(-400.0, 400.0);
    let command = 14.0 * error + 0.025 * controller.temp_integral;
    command.clamp(0.0, heater_max_w)
}

fn step_state(
    state: State,
    config: SimConfig,
    ua_w_k: f64,
    heater_w: f64,
    co2_on: bool,
    exchange_l_min: f64,
) -> State {
    let heat_loss_w = ua_w_k * (state.temp_c - AMBIENT_C);
    let temp_c = state.temp_c + (heater_w - heat_loss_w) * DT_S / config.effective_thermal_mass_j_k;

    let exchange_fraction = (exchange_l_min / CHAMBER_VOLUME_L) / 60.0 * DT_S;
    let injection_fraction = if co2_on {
        (config.co2_injection_l_min / CHAMBER_VOLUME_L) / 60.0 * DT_S
    } else {
        0.0
    };
    let ambient_co2 = 0.04;
    let mut co2_percent = state.co2_percent;
    co2_percent += exchange_fraction * (ambient_co2 - co2_percent);
    co2_percent += injection_fraction * (100.0 - co2_percent);

    State {
        temp_c,
        co2_percent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_design_passes_lumped_gate() {
        let report = simulate(default_config());
        assert!(report.pass, "{:?}", report.failures);
    }

    #[test]
    fn insufficient_heater_fails_gate() {
        let mut config = default_config();
        config.heater_max_w = 10.0;
        let report = simulate(config);
        assert!(!report.pass);
    }

    #[test]
    fn high_leak_rate_fails_co2_gate() {
        let mut config = default_config();
        config.leak_exchange_l_min = 2.0;
        let report = simulate(config);
        assert!(!report.pass);
    }
}
