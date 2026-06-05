use std::fs;
use std::path::Path;

use vcad::{centered_cube, centered_cylinder, Part};

// First physical visualization for the post-16-slot LaminarForge AAV condition
// module architecture.
//
// This is a dry, Bambu-printable mockup. It is not a wetted path, sterile part,
// live-cell fixture, AAV-contact part, or vendor drawing.

const OUTPUT_DIR: &str = "output/single_condition_module";
const OUTPUTS: [&str; 4] = [
    "output/single_condition_module/single_aav_condition_module_mockup.stl",
    "output/single_condition_module/single_aav_condition_tray_reference.stl",
    "output/single_condition_module/single_aav_connector_face_coupon.stl",
    "output/single_condition_module/single_aav_16_zone_scale_ghost.stl",
];

const SLAS_X: f64 = 127.76;
const SLAS_Y: f64 = 85.48;
const SLAS_CORNER_R: f64 = 3.18;
const MODULE_BASE_Z: f64 = 8.0;
const MODULE_SERVICE_Z: f64 = 17.0;
const FLANGE_Z: f64 = 2.4;
const FLANGE_W: f64 = 2.0;

const LOCAL_ZONE_X: f64 = 58.0;
const LOCAL_ZONE_Y: f64 = 36.0;
const LOCAL_ZONE_POCKET_Z: f64 = 3.0;
const OPTICAL_WINDOW_X: f64 = 40.0;
const OPTICAL_WINDOW_Y: f64 = 22.0;
const GASKET_FRAME_X: f64 = 70.0;
const GASKET_FRAME_Y: f64 = 48.0;
const GASKET_FRAME_W: f64 = 4.0;
const GASKET_FRAME_Z: f64 = 2.2;
const HARD_STOP_Z: f64 = 4.0;

const CONNECTOR_BLOCK_X: f64 = 52.0;
const CONNECTOR_BLOCK_Y: f64 = 11.0;
const CONNECTOR_BLOCK_Z: f64 = 11.0;
const CONNECTOR_PORT_R: f64 = 3.0;
const CONNECTOR_PORT_SPACING: f64 = 14.0;
const CONNECTOR_COUPON_X: f64 = 160.0;
const CONNECTOR_COUPON_MOUSE_EAR_R: f64 = 8.0;
const CONNECTOR_COUPON_EFFECTIVE_BED_TARGET: f64 = 180.0;

const ID_LAND_X: f64 = 38.0;
const ID_LAND_Y: f64 = 14.0;
const ID_LAND_Z: f64 = 1.2;

const TRAY_CLEARANCE: f64 = 2.0;
const TRAY_WALL: f64 = 8.0;
const TRAY_Z: f64 = 9.0;

const PRINT_BED_TARGET: f64 = 256.0;

fn main() {
    assert_print_envelopes();
    fs::create_dir_all(OUTPUT_DIR).expect("failed to create output/single_condition_module");

    export(OUTPUTS[0], &condition_module_mockup());
    export(OUTPUTS[1], &tray_reference());
    export(OUTPUTS[2], &connector_face_coupon());
    export(OUTPUTS[3], &sixteen_zone_scale_ghost());

    for path in OUTPUTS {
        assert!(
            Path::new(path).exists(),
            "single condition module export did not create required output: {path}"
        );
    }

    println!();
    println!("Single AAV condition module visualization outputs:");
    println!("  Output directory:       {OUTPUT_DIR}");
    println!("  Module footprint:       {SLAS_X:.2} x {SLAS_Y:.2} mm");
    println!("  Module service height:  {MODULE_SERVICE_Z:.1} mm visualization envelope");
    println!("  Local readout zone:     {LOCAL_ZONE_X:.1} x {LOCAL_ZONE_Y:.1} mm placeholder");
    println!("  Scale target:           one zone first; 16 same-condition zones later");
    println!("  Print status:           dry visualization only; not sterile or wetted path");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap_or_else(|error| {
        panic!("failed to write single condition module output {path}: {error:?}");
    });
    println!("Exported: {path}");
}

fn condition_module_mockup() -> Part {
    let base = rounded_slas_plate("single_aav_condition_module_base", MODULE_BASE_Z);
    let flange = bottom_flange().translate(0.0, 0.0, -MODULE_BASE_Z / 2.0 + FLANGE_Z / 2.0);

    let readout_pocket = centered_cube(
        "single_aav_condition_local_readout_pocket_cut",
        LOCAL_ZONE_X,
        LOCAL_ZONE_Y,
        LOCAL_ZONE_POCKET_Z + 0.4,
    )
    .translate(
        -13.0,
        -5.0,
        MODULE_BASE_Z / 2.0 - LOCAL_ZONE_POCKET_Z / 2.0 + 0.2,
    );

    let optical_window = centered_cube(
        "single_aav_condition_optical_window_cut",
        OPTICAL_WINDOW_X,
        OPTICAL_WINDOW_Y,
        MODULE_BASE_Z + 4.0,
    )
    .translate(-13.0, -5.0, 0.0);

    let gasket_frame = rectangular_frame(
        "single_aav_condition_local_gasket_witness_frame",
        GASKET_FRAME_X,
        GASKET_FRAME_Y,
        GASKET_FRAME_Z,
        GASKET_FRAME_W,
    )
    .translate(-13.0, -5.0, MODULE_BASE_Z / 2.0 + GASKET_FRAME_Z / 2.0);

    let hard_stops = hard_stop_set(-13.0, -5.0);
    let connector = connector_face_block().translate(24.0, SLAS_Y / 2.0 - 6.0, MODULE_BASE_Z / 2.0);
    let id_land = id_land().translate(-38.0, -SLAS_Y / 2.0 + 13.0, MODULE_BASE_Z / 2.0);
    let fiducials = fiducial_set(MODULE_BASE_Z / 2.0);
    let orientation_key = orientation_key().translate(
        -SLAS_X / 2.0 + 10.0,
        SLAS_Y / 2.0 - 10.0,
        MODULE_BASE_Z / 2.0,
    );
    let tray_contact_witness = tray_contact_witness().translate(0.0, 0.0, MODULE_BASE_Z / 2.0);

    base + flange - readout_pocket - optical_window
        + gasket_frame
        + hard_stops
        + connector
        + id_land
        + fiducials
        + orientation_key
        + tray_contact_witness
}

fn tray_reference() -> Part {
    let tray_x = SLAS_X + 2.0 * (TRAY_CLEARANCE + TRAY_WALL);
    let tray_y = SLAS_Y + 2.0 * (TRAY_CLEARANCE + TRAY_WALL);
    let tray = rounded_plate(
        "single_aav_condition_tray_outer",
        tray_x,
        tray_y,
        TRAY_Z,
        5.0,
    );

    let module_clearance = rounded_plate(
        "single_aav_condition_tray_module_clearance_cut",
        SLAS_X + 2.0 * TRAY_CLEARANCE,
        SLAS_Y + 2.0 * TRAY_CLEARANCE,
        TRAY_Z + 2.0,
        SLAS_CORNER_R + TRAY_CLEARANCE,
    )
    .translate(0.0, 0.0, 2.0);

    let rear_rail = centered_cube(
        "single_aav_condition_tray_rear_reference_rail",
        SLAS_X + 16.0,
        3.0,
        8.0,
    )
    .translate(0.0, SLAS_Y / 2.0 + TRAY_CLEARANCE + 1.5, TRAY_Z / 2.0 + 4.0);

    let left_rail = centered_cube(
        "single_aav_condition_tray_left_reference_rail",
        3.0,
        SLAS_Y + 12.0,
        8.0,
    )
    .translate(
        -SLAS_X / 2.0 - TRAY_CLEARANCE - 1.5,
        0.0,
        TRAY_Z / 2.0 + 4.0,
    );

    let label_tab = centered_cube(
        "single_aav_condition_tray_noncritical_label_tab",
        52.0,
        13.0,
        2.0,
    )
    .translate(0.0, -tray_y / 2.0 + 9.0, TRAY_Z / 2.0 + 1.0);

    tray - module_clearance + rear_rail + left_rail + label_tab
}

fn connector_face_coupon() -> Part {
    let body = centered_cube("single_aav_connector_coupon_body", CONNECTOR_COUPON_X, 66.0, 10.0);
    let adhesion_ears = connector_coupon_adhesion_ears();

    let nanotight = connector_test_block("nanotight_10_32_placeholder", -50.0, -8.0, 10.0);
    let magnetic = connector_test_block("magnetic_gasket_placeholder", 0.0, -8.0, 13.0);
    let luer = connector_test_block("luer_bench_adapter_placeholder", 50.0, -8.0, 17.0);

    let separator_a = centered_cube("single_aav_connector_coupon_separator_a", 2.0, 52.0, 5.0)
        .translate(-25.0, 0.0, 7.5);
    let separator_b = centered_cube("single_aav_connector_coupon_separator_b", 2.0, 52.0, 5.0)
        .translate(25.0, 0.0, 7.5);

    let tubing_comb = tubing_comb("single_aav_connector_coupon_tubing_comb", 0.0, 22.0, 10.0);

    (body + adhesion_ears + nanotight + magnetic + luer + separator_a + separator_b + tubing_comb)
        .translate(0.0, 0.0, 5.0)
}

fn connector_coupon_adhesion_ears() -> Part {
    let mut ears = Part::empty("single_aav_connector_coupon_sacrificial_mouse_ears");
    for (i, (x, y)) in [(-80.0, -33.0), (80.0, -33.0), (-80.0, 33.0), (80.0, 33.0)]
        .into_iter()
        .enumerate()
    {
        ears = ears
            + centered_cylinder(
                format!("single_aav_connector_coupon_mouse_ear_{i}"),
                CONNECTOR_COUPON_MOUSE_EAR_R,
                0.6,
                40,
            )
            .translate(x, y, -4.7);
    }
    ears
}

fn sixteen_zone_scale_ghost() -> Part {
    let body = rounded_slas_plate("single_aav_16_zone_scale_ghost_base", 4.0);
    let mut zones = Part::empty("single_aav_16_zone_same_condition_ghost_zones");

    let pitch_x = 24.0;
    let pitch_y = 15.5;
    let zone_x = 14.0;
    let zone_y = 8.0;
    for row in 0..4 {
        for col in 0..4 {
            let x = (col as f64 - 1.5) * pitch_x;
            let y = (row as f64 - 1.5) * pitch_y;
            zones = zones
                + centered_cube(
                    format!("single_aav_16_zone_same_condition_marker_{row}_{col}"),
                    zone_x,
                    zone_y,
                    1.8,
                )
                .translate(x, y, 2.9);
        }
    }

    let inlet_bus = centered_cube(
        "single_aav_16_zone_common_condition_inlet_witness",
        104.0,
        2.0,
        1.8,
    )
    .translate(0.0, -34.0, 2.9);
    let outlet_bus = centered_cube(
        "single_aav_16_zone_common_condition_outlet_witness",
        104.0,
        2.0,
        1.8,
    )
    .translate(0.0, 34.0, 2.9);
    let id_land = id_land().translate(-38.0, -SLAS_Y / 2.0 + 12.0, 2.0);

    body + zones + inlet_bus + outlet_bus + id_land
}

fn connector_face_block() -> Part {
    let block = centered_cube(
        "single_aav_condition_connector_face_block",
        CONNECTOR_BLOCK_X,
        CONNECTOR_BLOCK_Y,
        CONNECTOR_BLOCK_Z,
    );

    let mut ports = Part::empty("single_aav_condition_connector_face_port_placeholders");
    for (i, x) in [-CONNECTOR_PORT_SPACING, 0.0, CONNECTOR_PORT_SPACING]
        .into_iter()
        .enumerate()
    {
        ports = ports
            + centered_cylinder(
                format!("single_aav_condition_connector_port_boss_{i}"),
                CONNECTOR_PORT_R,
                3.0,
                32,
            )
            .translate(x, -CONNECTOR_BLOCK_Y / 2.0 - 1.0, 0.0);
    }

    let strain_relief = tubing_comb(
        "single_aav_condition_connector_face_strain_relief_comb",
        0.0,
        CONNECTOR_BLOCK_Y / 2.0 + 3.0,
        0.0,
    );

    block + ports + strain_relief
}

fn connector_test_block(name: &str, x: f64, y: f64, boss_d: f64) -> Part {
    let base = centered_cube(format!("{name}_base"), 46.0, 30.0, 9.0).translate(x, y, 9.5);
    let boss =
        centered_cylinder(format!("{name}_boss"), boss_d / 2.0, 7.0, 40).translate(x, y, 17.5);
    let port =
        centered_cylinder(format!("{name}_port_witness"), 1.6, 8.0, 24).translate(x, y, 18.0);
    base + boss - port
}

fn id_land() -> Part {
    centered_cube(
        "single_aav_condition_module_id_barcode_land",
        ID_LAND_X,
        ID_LAND_Y,
        ID_LAND_Z,
    )
    .translate(0.0, 0.0, ID_LAND_Z / 2.0)
}

fn hard_stop_set(center_x: f64, center_y: f64) -> Part {
    let mut stops = Part::empty("single_aav_condition_local_hard_stop_set");
    let stop_x = GASKET_FRAME_X / 2.0 - 5.0;
    let stop_y = GASKET_FRAME_Y / 2.0 - 5.0;
    for (i, (x, y)) in [
        (-stop_x, -stop_y),
        (stop_x, -stop_y),
        (-stop_x, stop_y),
        (stop_x, stop_y),
    ]
    .into_iter()
    .enumerate()
    {
        stops = stops
            + centered_cylinder(
                format!("single_aav_condition_hard_stop_{i}"),
                2.2,
                HARD_STOP_Z,
                24,
            )
            .translate(
                center_x + x,
                center_y + y,
                MODULE_BASE_Z / 2.0 + HARD_STOP_Z / 2.0,
            );
    }
    stops
}

fn fiducial_set(z: f64) -> Part {
    let mut fiducials = Part::empty("single_aav_condition_imaging_fiducials");
    for (i, (x, y)) in [
        (-SLAS_X / 2.0 + 14.0, -SLAS_Y / 2.0 + 14.0),
        (SLAS_X / 2.0 - 14.0, -SLAS_Y / 2.0 + 14.0),
        (-SLAS_X / 2.0 + 14.0, SLAS_Y / 2.0 - 14.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(format!("single_aav_condition_fiducial_{i}"), 2.0, 1.2, 24)
                .translate(x, y, z + 0.6);
    }
    fiducials
}

fn orientation_key() -> Part {
    let shaft = centered_cube(
        "single_aav_condition_front_left_orientation_key_shaft",
        10.0,
        2.2,
        1.4,
    )
    .translate(1.5, 0.0, 0.7);
    let head = centered_cube(
        "single_aav_condition_front_left_orientation_key_head",
        4.0,
        6.0,
        1.4,
    )
    .translate(7.0, 0.0, 0.7);
    shaft + head
}

fn tray_contact_witness() -> Part {
    rectangular_frame(
        "single_aav_condition_noncritical_tray_contact_witness",
        SLAS_X - 8.0,
        SLAS_Y - 8.0,
        0.8,
        1.4,
    )
    .translate(0.0, 0.0, 0.4)
}

fn tubing_comb(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut comb = Part::empty(name);
    for (i, dx) in [-12.0, 0.0, 12.0].into_iter().enumerate() {
        comb = comb
            + centered_cube(format!("{name}_finger_{i}"), 3.0, 12.0, 5.0).translate(
                x + dx,
                y,
                z + 2.5,
            );
    }
    comb
}

fn bottom_flange() -> Part {
    rectangular_frame(
        "single_aav_condition_slas_bottom_flange_witness",
        SLAS_X - 4.0,
        SLAS_Y - 4.0,
        FLANGE_Z,
        FLANGE_W,
    )
}

fn rounded_slas_plate(name: &str, z: f64) -> Part {
    rounded_plate(name, SLAS_X, SLAS_Y, z, SLAS_CORNER_R)
}

fn rounded_plate(name: &str, x: f64, y: f64, z: f64, corner_r: f64) -> Part {
    let mut plate = centered_cube(name, x, y, z);
    let half_x = x / 2.0;
    let half_y = y / 2.0;
    let through = z + 1.0;

    for &(sx, sy) in &[(1.0_f64, 1.0_f64), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)] {
        let corner_square = centered_cube(
            format!("{name}_corner_square_cut_{sx}_{sy}"),
            corner_r,
            corner_r,
            through,
        )
        .translate(
            sx * (half_x - corner_r / 2.0),
            sy * (half_y - corner_r / 2.0),
            0.0,
        );
        let corner_round = centered_cylinder(
            format!("{name}_corner_radius_keep_{sx}_{sy}"),
            corner_r,
            through,
            32,
        )
        .translate(sx * (half_x - corner_r), sy * (half_y - corner_r), 0.0);
        plate = plate - (corner_square - corner_round);
    }

    plate
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, z: f64, wall: f64) -> Part {
    centered_cube(name, outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner_cut"),
            outer_x - 2.0 * wall,
            outer_y - 2.0 * wall,
            z + 0.4,
        )
}

fn assert_print_envelopes() {
    assert!(SLAS_X < PRINT_BED_TARGET);
    assert!(SLAS_Y < PRINT_BED_TARGET);
    assert!(SLAS_CORNER_R > 1.5);
    assert!(FLANGE_W >= 1.27);
    assert!(LOCAL_ZONE_X < SLAS_X - 36.0);
    assert!(LOCAL_ZONE_Y < SLAS_Y - 28.0);
    assert!(OPTICAL_WINDOW_X < LOCAL_ZONE_X);
    assert!(OPTICAL_WINDOW_Y < LOCAL_ZONE_Y);
    assert!(GASKET_FRAME_X > LOCAL_ZONE_X);
    assert!(GASKET_FRAME_Y > LOCAL_ZONE_Y);
    assert!(TRAY_Z < MODULE_SERVICE_Z);
    assert!(SLAS_X + 2.0 * (TRAY_CLEARANCE + TRAY_WALL) < PRINT_BED_TARGET);
    assert!(SLAS_Y + 2.0 * (TRAY_CLEARANCE + TRAY_WALL) < PRINT_BED_TARGET);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_fits_conservative_bambu_bed() {
        assert_print_envelopes();
    }

    #[test]
    fn local_zone_is_not_a_dead_end_architecture() {
        assert!(LOCAL_ZONE_X < SLAS_X / 2.0);
        assert!(GASKET_FRAME_X < SLAS_X - 40.0);
    }

    #[test]
    fn connector_face_keeps_three_visual_options() {
        assert_eq!(CONNECTOR_PORT_SPACING, 14.0);
        assert!(CONNECTOR_BLOCK_X > 3.0 * CONNECTOR_PORT_SPACING);
    }

    #[test]
    fn connector_coupon_stays_inside_small_effective_bed() {
        assert!(
            CONNECTOR_COUPON_X + 2.0 * CONNECTOR_COUPON_MOUSE_EAR_R
                <= CONNECTOR_COUPON_EFFECTIVE_BED_TARGET
        );
    }
}
