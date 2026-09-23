//! Plate-built frame. Joint dimensions are part of the runtime contract.
use super::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub roof_edge: f64,
    pub front_edge: f64,
    pub roof_screw_y: [f64; 3],
    pub roof_pin_y: [f64; 2],
    pub front_screw_z: [f64; 2],
    pub front_pin_z: f64,
}

pub fn validate(p: &Config, d: &Layout) -> Result<(), String> {
    let j = &p.joints;
    let values = [j.roof_edge, j.front_edge, j.front_pin_z];
    if values
        .iter()
        .chain(j.roof_screw_y.iter())
        .chain(j.roof_pin_y.iter())
        .chain(j.front_screw_z.iter())
        .any(|v| !v.is_finite() || *v <= 0.)
    {
        return Err("joint coordinates must be positive and finite".into());
    }
    // This hardware contract uses M3x14 roof / M3x10 front screws and D3 pins.
    if (p.roof_thickness - 6.).abs() > 0.001
        || (p.bezel_thickness - 6.).abs() > 0.001
        || j.front_edge < 4.
        || j.roof_edge < 5.
        || j.roof_edge > p.side_wall - 5.
        || d.outer_x / 2. - j.front_edge - 3. - (d.gasket_outer_x + 0.5) / 2. < 0.7
    {
        return Err("plate joints violate hardware thickness, edge or gasket land contract".into());
    }
    let mut ys = Vec::from(j.roof_screw_y);
    ys.extend(j.roof_pin_y);
    for (i, y) in ys.iter().enumerate() {
        if *y < p.bezel_thickness + 16.
            || *y > d.rear_y - 16.
            || ys[..i].iter().any(|other| (y - other).abs() < 8.)
        {
            return Err("roof joint edge margin or hole separation invalid".into());
        }
    }
    let zs = [j.front_screw_z[0], j.front_pin_z, j.front_screw_z[1]];
    for (i, z) in zs.iter().enumerate() {
        if *z < d.guide_ceiling + 6.
            || *z > d.roof_bottom - 5.
            || zs[..i].iter().any(|other| (z - other).abs() < 6.)
        {
            return Err("front joint edge margin or hole separation invalid".into());
        }
    }
    Ok(())
}

pub fn side(p: &Config, d: &Layout, sign: f64, rail_positions: [f64; 3]) -> Part {
    let n = p.mesh_segments;
    let j = &p.joints;
    let mut side = block(
        "side_plate",
        [d.cavity_x / 2., p.bezel_thickness, p.guide_rail_thickness],
        [d.outer_x / 2., d.rear_y, d.roof_bottom],
    ) - block(
        "guide_relief",
        [d.cavity_x / 2. - 1., p.bezel_thickness - 1., -1.],
        [d.guide_width / 2., d.rear_y + 1., d.guide_ceiling],
    );
    side = side
        - cy(
            "rear_M3_tap",
            d.cavity_x / 2. + p.side_wall / 2.,
            d.rear_y - 5.,
            (d.guide_ceiling + d.roof_bottom) / 2.,
            2.5,
            12.,
            n,
        );
    for y in rail_positions {
        side = side
            - cz(
                "rail_M3_tap",
                d.rail_screw_x,
                y,
                p.guide_rail_thickness + 5.,
                2.5,
                12.,
                n,
            );
    }
    for y in j.roof_screw_y {
        side = side
            - cz(
                "joint_roof_M3_tap",
                d.outer_x / 2. - j.roof_edge,
                y,
                d.roof_bottom - 5.,
                2.5,
                12.,
                n,
            );
    }
    for y in j.roof_pin_y {
        side = side
            - cz(
                "joint_roof_pin_clearance",
                d.outer_x / 2. - j.roof_edge,
                y,
                d.roof_bottom - 2.75,
                3.,
                6.5,
                n,
            );
    }
    for z in j.front_screw_z {
        side = side
            - cy(
                "joint_front_M3_tap",
                d.outer_x / 2. - j.front_edge,
                p.bezel_thickness + 5.,
                z,
                2.5,
                12.,
                n,
            );
    }
    side = side
        - cy(
            "joint_front_pin_clearance",
            d.outer_x / 2. - j.front_edge,
            p.bezel_thickness + 2.75,
            j.front_pin_z,
            3.,
            6.5,
            n,
        );
    if sign < 0. {
        side = side.mirror_x()
            - cz(
                "stop_receiver",
                d.stop_x,
                p.stop_y,
                d.guide_ceiling + 3.5,
                1.6,
                9.,
                n,
            );
    }
    side
}

pub fn roof_holes(mut roof: Part, p: &Config, d: &Layout) -> Part {
    for sign in [-1., 1.] {
        let x = sign * (d.outer_x / 2. - p.joints.roof_edge);
        for y in p.joints.roof_screw_y {
            roof = roof
                - cz(
                    "joint_roof_M3_clearance",
                    x,
                    y,
                    (d.roof_bottom + d.roof_top) / 2.,
                    3.4,
                    p.roof_thickness + 2.,
                    p.mesh_segments,
                );
        }
        for y in p.joints.roof_pin_y {
            roof = roof
                - cz(
                    "joint_roof_slip_clearance",
                    x,
                    y,
                    (d.roof_bottom + d.roof_top) / 2.,
                    3.15,
                    p.roof_thickness + 2.,
                    p.mesh_segments,
                );
        }
    }
    roof
}

pub fn bezel_holes(mut bezel: Part, p: &Config, d: &Layout) -> Part {
    for sign in [-1., 1.] {
        let x = sign * (d.outer_x / 2. - p.joints.front_edge);
        for z in p.joints.front_screw_z {
            bezel = bezel
                - cy(
                    "joint_front_M3_clearance",
                    x,
                    p.bezel_thickness / 2.,
                    z,
                    3.4,
                    p.bezel_thickness + 2.,
                    p.mesh_segments,
                )
                - cy(
                    "joint_front_counterbore",
                    x,
                    1.55,
                    z,
                    6.,
                    3.3,
                    p.mesh_segments,
                );
        }
        bezel = bezel
            - cy(
                "joint_front_slip_clearance",
                x,
                p.bezel_thickness / 2.,
                p.joints.front_pin_z,
                3.15,
                p.bezel_thickness + 2.,
                p.mesh_segments,
            );
    }
    bezel
}

pub fn hardware(p: &Config, d: &Layout) -> Vec<Item> {
    let mut result = Vec::new();
    for (label, sign) in [("left", -1.), ("right", 1.)] {
        let x = sign * (d.outer_x / 2. - p.joints.roof_edge);
        for (index, y) in p.joints.roof_screw_y.iter().enumerate() {
            let screw = cz("shank", x, *y, d.roof_top - 7., 3., 14., p.mesh_segments)
                + cz("head", x, *y, d.roof_top + 1.5, 5.5, 3., p.mesh_segments);
            result.push(item(
                &format!("REF_joint_roof_{label}_screw_{index}"),
                screw,
                false,
                true,
                [90, 100, 110],
            ));
        }
        for (index, y) in p.joints.roof_pin_y.iter().enumerate() {
            result.push(item(
                &format!("REF_joint_roof_{label}_pin_{index}"),
                cz("pin", x, *y, d.roof_bottom - 1., 3., 10., p.mesh_segments),
                false,
                true,
                [180, 180, 180],
            ));
        }
        let x = sign * (d.outer_x / 2. - p.joints.front_edge);
        for (index, z) in p.joints.front_screw_z.iter().enumerate() {
            let screw = cy("shank", x, 8.2, *z, 3., 10., p.mesh_segments)
                + cy("head", x, 1.7, *z, 5.5, 3., p.mesh_segments);
            result.push(item(
                &format!("REF_joint_front_{label}_screw_{index}"),
                screw,
                false,
                true,
                [90, 100, 110],
            ));
        }
        result.push(item(
            &format!("REF_joint_front_{label}_pin"),
            cy(
                "pin",
                x,
                p.bezel_thickness + 1.,
                p.joints.front_pin_z,
                3.,
                10.,
                p.mesh_segments,
            ),
            false,
            true,
            [180, 180, 180],
        ));
    }
    result
}

/// Threaded engagement intersects the pilot bore intentionally; other parts must clear.
pub fn verify_hardware(parts: &[Item], d: &Layout) -> Result<(), String> {
    for hardware in parts.iter().filter(|i| i.name.starts_with("REF_joint_")) {
        let side = if hardware.name.contains("left") {
            "01_left_side_plate"
        } else {
            "01_right_side_plate"
        };
        let face = if hardware.name.contains("roof") {
            "01_roof_plate"
        } else {
            "01_front_bezel"
        };
        for other in parts
            .iter()
            .filter(|i| !i.reference && i.name != side && i.name != face)
        {
            for fraction in [0., 0.25, 0.5, 0.75, 1.] {
                let shifted = other.part.translate(
                    0.,
                    if other.moving {
                        -d.stroke * fraction
                    } else {
                        0.
                    },
                    0.,
                );
                if hardware.part.intersection(&shifted).volume() > 0.03 {
                    return Err(format!(
                        "joint hardware collision: {} / {} at {fraction}",
                        hardware.name, other.name
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> Config {
        toml::from_str(include_str!(
            "../../models/heated_microplate_cassette_v0.toml"
        ))
        .unwrap()
    }
    #[test]
    fn old_narrow_frame_cannot_accept_front_joints_outside_seal() {
        let mut p = config();
        assert!(p.validate().is_ok());
        p.side_wall = 12.25;
        assert!(p.validate().unwrap_err().contains("gasket land"));
    }
    #[test]
    fn joint_holes_cannot_merge_or_enter_rear_tapped_region() {
        let mut p = config();
        p.joints.roof_pin_y[0] = p.joints.roof_screw_y[0];
        assert!(p.validate().unwrap_err().contains("hole separation"));
        let mut p = config();
        p.joints.roof_screw_y[2] = p.layout().rear_y - 5.;
        assert!(p.validate().unwrap_err().contains("edge margin"));
    }
    #[test]
    fn hardware_contract_rejects_unreviewed_plate_thickness() {
        let mut p = config();
        p.roof_thickness = 6.35;
        assert!(p.validate().unwrap_err().contains("hardware thickness"));
    }
    #[test]
    fn missing_or_unknown_joint_fields_fail_deserialization() {
        let raw = include_str!("../../models/heated_microplate_cassette_v0.toml");
        assert!(toml::from_str::<Config>(&raw.replace("front_edge = 4.0\n", "")).is_err());
        assert!(toml::from_str::<Config>(&format!("{raw}\nunreviewed = 1.0\n")).is_err());
    }
}
