use clap::Parser;
use serde::{Deserialize, Serialize};
use vcad::{centered_cube, centered_cylinder, Part};

use laminarforge_cad::runtime_cad::{run, Args};
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Parameters {
    chip_length_mm: f64,
    chip_width_mm: f64,
    margin_x_mm: f64,
    margin_y_mm: f64,
    base_height_mm: f64,
    pocket_clearance_mm: f64,
    pocket_depth_mm: f64,
}

impl Parameters {
    fn validate(&self) -> Result<(), &'static str> {
        let dimensions = [
            self.chip_length_mm,
            self.chip_width_mm,
            self.margin_x_mm,
            self.margin_y_mm,
            self.base_height_mm,
            self.pocket_clearance_mm,
            self.pocket_depth_mm,
        ];
        if dimensions.iter().any(|v| !v.is_finite() || *v <= 0.0) {
            return Err("all dimensions must be finite and positive millimeters");
        }
        if self.chip_length_mm <= 24.0
            || self.chip_width_mm <= 20.0
            || self.margin_x_mm < 24.0
            || self.margin_y_mm < 34.0
            || self.pocket_clearance_mm >= 2.0
            || self.base_height_mm < 12.0
            || self.pocket_depth_mm >= self.base_height_mm - 2.0
        {
            return Err(
                "dimensions violate fixture window, margin, clearance, or floor constraints",
            );
        }
        Ok(())
    }
}

// Chip priming and tubing fixture for early microfluidic workflow validation.
//
// Holds one Rev C LaminarForge chip in an ANSI/SLAS footprint pocket, routes
// inlet/outlet tubing through strain-relief combs, provides a bubble-view slot,
// and catches priming overflow in a shallow trough.
//
// Exports:
//   output/chip_priming_fixture_base.stl
//   output/chip_priming_fixture_tubing_comb.stl
//   output/chip_priming_fixture_luer_clip.stl
//   output/chip_priming_fixture_assembly.stl

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run::<Parameters>(
        &args,
        "chip_priming_tubing_fixture",
        &[
            "chip_priming_fixture_base.stl",
            "chip_priming_fixture_tubing_comb.stl",
            "chip_priming_fixture_luer_clip.stl",
            "chip_priming_fixture_assembly.stl",
        ],
        |p, i| build(p, i),
    )
}
fn build(parameters: &Parameters, index: usize) -> Result<Part, String> {
    parameters.validate()?;
    let chip_length = parameters.chip_length_mm;
    let chip_width = parameters.chip_width_mm;
    let base_x = chip_length + parameters.margin_x_mm * 2.0;
    let base_y = chip_width + parameters.margin_y_mm * 2.0;
    let base_z = parameters.base_height_mm;
    let pocket_clearance = parameters.pocket_clearance_mm;
    let pocket_depth = parameters.pocket_depth_mm;

    let base_body = centered_cube("priming_base", base_x, base_y, base_z);
    let chip_pocket = centered_cube(
        "chip_pocket",
        chip_length + pocket_clearance * 2.0,
        chip_width + pocket_clearance * 2.0,
        pocket_depth + 0.2,
    )
    .translate(0.0, 0.0, base_z / 2.0 - pocket_depth / 2.0 + 0.1);

    let bubble_window = centered_cube("bubble_window", chip_length - 24.0, 16.0, base_z + 2.0)
        .translate(0.0, 0.0, 0.0);
    let prime_trough = centered_cube("prime_trough", base_x - 28.0, 12.0, 5.0).translate(
        0.0,
        -(base_y / 2.0) + 18.0,
        base_z / 2.0 - 2.5,
    );

    let mut mount_holes = Part::empty("priming_mount_holes");
    for x in [-(base_x / 2.0 - 10.0), base_x / 2.0 - 10.0] {
        for y in [-(base_y / 2.0 - 10.0), base_y / 2.0 - 10.0] {
            mount_holes = mount_holes
                + centered_cylinder("priming_mount", 3.2 / 2.0, base_z + 2.0, 24)
                    .translate(x, y, 0.0);
        }
    }

    let mut dowel_holes = Part::empty("priming_dowel_holes");
    for (i, (x, y)) in [
        (-(chip_length / 2.0 + 2.0), -(chip_width / 2.0 + 2.0)),
        (-(chip_length / 2.0 + 2.0), chip_width / 2.0 + 2.0),
        (chip_length / 2.0 + 2.0, -(chip_width / 2.0 + 2.0)),
    ]
    .iter()
    .enumerate()
    {
        dowel_holes = dowel_holes
            + centered_cylinder(format!("priming_dowel_{i}"), 2.9 / 2.0, 8.0, 24)
                .translate(*x, *y, 1.0);
    }

    let base = base_body - chip_pocket - bubble_window - prime_trough - mount_holes - dowel_holes;
    if index == 0 {
        return Ok(base);
    }

    let inlet_comb = tubing_comb("inlet").translate(0.0, base_y / 2.0 + 12.0, base_z / 2.0 + 6.0);
    let outlet_comb =
        tubing_comb("outlet").translate(0.0, -(base_y / 2.0 + 12.0), base_z / 2.0 + 6.0);
    let combs = inlet_comb + outlet_comb;
    if index == 1 {
        return Ok(combs);
    }

    let luer_clip = luer_clip();
    if index == 2 {
        return Ok(luer_clip);
    }

    // Match component export evaluation before assembling the same solids.
    base.to_mesh();
    combs.to_mesh();
    luer_clip.to_mesh();
    let assembly = base
        + combs
        + luer_clip.translate(
            -(base_x / 2.0 + 24.0),
            base_y / 2.0 + 12.0,
            base_z / 2.0 + 8.0,
        )
        + luer_clip.translate(
            base_x / 2.0 + 24.0,
            -(base_y / 2.0 + 12.0),
            base_z / 2.0 + 8.0,
        );

    if index != 3 {
        return Err("invalid component".into());
    }
    Ok(assembly)
}

fn tubing_comb(name: &str) -> Part {
    let body = centered_cube(format!("{name}_comb_body"), 120.0, 18.0, 14.0);
    let mut channels = Part::empty(format!("{name}_comb_channels"));
    for (i, x) in [-45.0, -15.0, 15.0, 45.0].iter().enumerate() {
        let channel = centered_cylinder(format!("{name}_tube_channel_{i}"), 4.8 / 2.0, 20.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0);
        let top_slot =
            centered_cube(format!("{name}_tube_slot_{i}"), 6.0, 20.0, 12.0).translate(*x, 0.0, 5.0);
        channels = channels + channel + top_slot;
    }
    body - channels
}

fn luer_clip() -> Part {
    let body = centered_cube("luer_clip_body", 42.0, 28.0, 18.0);
    let luer_channel =
        centered_cylinder("luer_channel", 8.0 / 2.0, 44.0, 32).rotate(0.0, 90.0, 0.0);
    let snap_slot = centered_cube("luer_snap_slot", 44.0, 8.0, 14.0).translate(0.0, 7.0, 0.0);
    let mount_hole = centered_cylinder("luer_mount_hole", 3.2 / 2.0, 30.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -7.0, 0.0);
    body - luer_channel - snap_slot - mount_hole
}

#[cfg(test)]
mod tests {
    use super::*;

    fn committed() -> Parameters {
        toml::from_str(&std::fs::read_to_string("models/chip_priming_tubing_fixture.toml").unwrap())
            .unwrap()
    }

    #[test]
    fn committed_dimensions_preserve_original_fixture() {
        let p = committed();
        assert!(p.validate().is_ok());
        assert_eq!(p.chip_length_mm, 127.76);
        assert_eq!(p.chip_width_mm, 85.48);
        assert_eq!(p.pocket_clearance_mm, 0.25);
        assert_eq!(p.base_height_mm, 12.0);
    }

    #[test]
    fn rejects_invalid_geometry_and_unknown_fields() {
        let mut p = committed();
        p.pocket_depth_mm = p.base_height_mm;
        assert!(p.validate().is_err());
        p = committed();
        p.chip_width_mm = f64::NAN;
        assert!(p.validate().is_err());
        let text = std::fs::read_to_string("models/chip_priming_tubing_fixture.toml").unwrap();
        assert!(toml::from_str::<Parameters>(&format!("{text}\nwall_thikness_mm = 2\n")).is_err());
        assert!(toml::from_str::<Parameters>("chip_length_mm = 100").is_err());
    }
}
