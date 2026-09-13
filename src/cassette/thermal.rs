//! Reduced steady sheet-conduction model. This is not a media-temperature or CFD model.
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fs, path::Path};
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub conductivity_w_mk: Vec<f64>,
    pub effective_face_loss_w_m2k: Vec<f64>,
    pub ambient_c: f64,
    pub metal_sensor_setpoint_c: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kgk: f64,
    pub grid_cells: usize,
}
pub fn validate(t: &Parameters) -> Result<(), String> {
    if t.conductivity_w_mk.is_empty()
        || t.effective_face_loss_w_m2k.is_empty()
        || t.grid_cells < 12
        || t.grid_cells > 100
    {
        return Err("invalid thermal grid/property scenarios".into());
    }
    if t.conductivity_w_mk
        .iter()
        .any(|v| !v.is_finite() || *v < 100.0 || *v > 250.0)
        || t.effective_face_loss_w_m2k
            .iter()
            .any(|v| !v.is_finite() || *v < 1.0 || *v > 50.0)
        || !t.ambient_c.is_finite()
        || !t.metal_sensor_setpoint_c.is_finite()
        || t.ambient_c >= t.metal_sensor_setpoint_c
        || !(2500.0..=2900.0).contains(&t.density_kg_m3)
        || !(800.0..=1100.0).contains(&t.specific_heat_j_kgk)
    {
        return Err("invalid thermal material/boundary inputs".into());
    }
    Ok(())
}
pub fn run(
    dir: &Path,
    p: &super::Config,
    d: &super::Layout,
    parts: &[super::Item],
) -> Result<(), Box<dyn std::error::Error>> {
    let t = &p.thermal;
    validate(t)?;
    let n = t.grid_cells;
    let mut scenarios = Vec::new();
    for zone in ["drawer", "roof"] {
        for &k in &t.conductivity_w_mk {
            for &h in &t.effective_face_loss_w_m2k {
                let w = if zone == "drawer" {
                    d.drawer_x
                } else {
                    d.outer_x
                } / 1000.0;
                let length = d.drawer_y / 1000.0;
                let dx = w / n as f64;
                let dy = length / n as f64;
                let thick = if zone == "drawer" {
                    p.drawer_thickness
                } else {
                    p.roof_thickness
                } / 1000.0;
                let gx = k * thick * dy / dx;
                let gy = k * thick * dx / dy;
                let extra_side_height = if zone == "roof" {
                    (d.roof_bottom - p.guide_rail_thickness) / 1000.0
                } else {
                    0.0
                };
                let mut loss = vec![h * dx * dy; n * n];
                let mut heat = vec![0.0; n * n];
                for y in 0..n {
                    for x in 0..n {
                        let xx = ((x as f64 + 0.5) * dx - w / 2.0) * 1000.0;
                        let yy = (y as f64 + 0.5) * dy * 1000.0;
                        if xx.abs() <= p.heater_x / 2.0
                            && (yy - d.plate_center_y).abs() <= p.heater_y / 2.0
                        {
                            heat[y * n + x] = 1.0;
                        }
                        if x == 0 || x == n - 1 {
                            loss[y * n + x] += h * extra_side_height * dy;
                        }
                    }
                }
                let total = heat.iter().sum::<f64>();
                for q in &mut heat {
                    *q /= total;
                }
                let mut response = vec![1.0 / loss.iter().sum::<f64>(); n * n];
                let mut converged = false;
                let mut iterations = 0;
                for iter in 0..40000 {
                    let mut max_delta = 0.0f64;
                    for y in 0..n {
                        for x in 0..n {
                            let j = y * n + x;
                            let mut sum = heat[j];
                            let mut diagonal = loss[j];
                            if x > 0 {
                                sum += gx * response[j - 1];
                                diagonal += gx;
                            }
                            if x + 1 < n {
                                sum += gx * response[j + 1];
                                diagonal += gx;
                            }
                            if y > 0 {
                                sum += gy * response[j - n];
                                diagonal += gy;
                            }
                            if y + 1 < n {
                                sum += gy * response[j + n];
                                diagonal += gy;
                            }
                            let updated = response[j] + 1.75 * (sum / diagonal - response[j]);
                            max_delta = max_delta.max((updated - response[j]).abs());
                            response[j] = updated;
                        }
                    }
                    if max_delta < 1e-9 {
                        converged = true;
                        iterations = iter + 1;
                        break;
                    }
                }
                if !converged {
                    return Err("thermal sheet solve did not converge".into());
                }
                let sensor_x = ((-5.0 / 1000.0 + w / 2.0) / dx).floor() as usize;
                let sensor_y = ((d.drawer_y - 29.5) / 1000.0 / dy).floor() as usize;
                let power =
                    (t.metal_sensor_setpoint_c - t.ambient_c) / response[sensor_y * n + sensor_x];
                let mut under_plate = Vec::new();
                let mut csv = String::from("x_mm,y_mm,metal_temperature_c\n");
                for y in 0..n {
                    for x in 0..n {
                        let xx = ((x as f64 + 0.5) * dx - w / 2.0) * 1000.0;
                        let yy = (y as f64 + 0.5) * dy * 1000.0;
                        let temp = t.ambient_c + power * response[y * n + x];
                        csv.push_str(&format!("{xx:.4},{yy:.4},{temp:.6}\n"));
                        if xx.abs() < p.plate_x / 2.0
                            && (yy - d.plate_center_y).abs() < p.plate_y / 2.0
                        {
                            under_plate.push(temp);
                        }
                    }
                }
                let energy_balance =
                    power * response.iter().zip(&loss).map(|(r, g)| r * g).sum::<f64>();
                let min = under_plate.iter().copied().fold(f64::INFINITY, f64::min);
                let max = under_plate
                    .iter()
                    .copied()
                    .fold(f64::NEG_INFINITY, f64::max);
                if (energy_balance - power).abs() > 1e-4 {
                    return Err("thermal energy balance failed".into());
                }
                let file = format!("thermal-{zone}-k{k:.0}-h{h:.0}.csv");
                fs::write(dir.join(&file), csv)?;
                scenarios.push(json!({"zone":zone,"k_w_mk":k,"effective_h_w_m2k":h,"sensor_c":t.metal_sensor_setpoint_c,"required_power_w":power,"plate_footprint_metal_min_c":min,"plate_footprint_metal_max_c":max,"plate_footprint_metal_span_c":max-min,"iterations":iterations,"energy_balance_error_w":energy_balance-power,"grid_csv":file}));
            }
        }
    }
    let mass = parts
        .iter()
        .filter(|i| !i.reference && !i.name.starts_with("08_") && !i.name.starts_with("13_"))
        .map(|i| i.part.volume())
        .sum::<f64>()
        * 1e-9
        * t.density_kg_m3;
    let energy = mass * t.specific_heat_j_kgk * (t.metal_sensor_setpoint_c - t.ambient_c);
    fs::write(
        dir.join("thermal-sizing.json"),
        serde_json::to_vec_pretty(
            &json!({"model":"2D finite-volume steady aluminum sheet with distributed pad input and ambient loss; roof side losses lumped at two edges","parameters":t,"assembled_aluminum_mass_kg":mass,"all_aluminum_ideal_sensible_energy_j":energy,"ideal_no_loss_120w_seconds_NOT_warmup_prediction":energy/120.0,"scenarios":scenarios,"limits":["No media, plate-plastic, thermal-contact, enclosure-air or evaporation model","No transient/PID simulation; warmup and recovery require experiments","Effective h values are assumed sensitivity cases, not measured boundary conditions","Opposite-zone coupling, rear cover, guide contact and bench conduction not resolved","Metal spread is a sizing estimate, never proof of fluid uniformity"]}),
        )?,
    )?;
    Ok(())
}
