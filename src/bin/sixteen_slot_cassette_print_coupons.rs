use std::fs;
use std::path::Path;

use laminarforge_cad::{sixteen_slot_cassette_a0::*, REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Desktop-printable dry-fit coupons for the 16-slot first-article cassette.
//
// These are intentionally not biological parts. They are scaled 1:1 mechanical
// checks for pocket clearance, gasket compression, dock datum behavior, and
// bulkhead connector/strain-relief ergonomics before spending money on vendor
// RFQ parts.

const OUTPUT_DIR: &str = "output/print_coupons";
const OUTPUTS: [&str; 5] = [
    "output/print_coupons/sixteen_slot_chip_pocket_fit_coupon.stl",
    "output/print_coupons/sixteen_slot_gasket_compression_coupon.stl",
    "output/print_coupons/sixteen_slot_dock_datum_rail_coupon.stl",
    "output/print_coupons/sixteen_slot_carrier_corner_surrogate.stl",
    "output/print_coupons/sixteen_slot_bulkhead_connector_mockup.stl",
];

const PRINT_BED_TARGET_XY: f64 = 256.0;
const FEATURE_ANCHOR_OVERLAP: f64 = 0.40;
const COUPON_STOP_CLEARANCE: f64 = 1.0;
const CHIP_COUPON_STOP_OFFSET_X: f64 =
    PER_SLOT_GASKET_OUTER_X / 2.0 + INTERNAL_STOP_DIAMETER / 2.0 + COUPON_STOP_CLEARANCE;
const CHIP_COUPON_STOP_OFFSET_Y: f64 =
    PER_SLOT_GASKET_OUTER_Y / 2.0 + INTERNAL_STOP_DIAMETER / 2.0 + COUPON_STOP_CLEARANCE;

fn main() {
    assert_coupon_envelopes();
    fs::create_dir_all(OUTPUT_DIR).expect("failed to create output/print_coupons");

    let chip_pocket = chip_pocket_fit_coupon();
    export(OUTPUTS[0], &chip_pocket);

    let gasket = gasket_compression_coupon();
    export(OUTPUTS[1], &gasket);

    let dock = dock_datum_rail_coupon();
    export(OUTPUTS[2], &dock);

    let carrier_corner = carrier_corner_surrogate();
    export(OUTPUTS[3], &carrier_corner);

    let bulkhead = bulkhead_connector_mockup();
    export(OUTPUTS[4], &bulkhead);

    for path in OUTPUTS {
        assert!(
            Path::new(path).exists(),
            "print coupon export did not create required output: {path}"
        );
    }
    verify_binary_stl_size(
        OUTPUTS[0],
        [
            CHIP_FIT_COUPON_X,
            CHIP_FIT_COUPON_Y,
            CHIP_FIT_COUPON_OVERALL_Z,
        ],
        0.05,
    )
    .expect("chip-fit coupon STL does not match the A0 envelope");

    println!();
    println!("16-slot cassette printable dry-fit coupons:");
    println!("  Export directory:       {OUTPUT_DIR}");
    println!("  Output count:           {}", OUTPUTS.len());
    println!(
        "  Chip pocket:            {:.2} x {:.2} mm pocket, {:.2} mm/side CAD clearance",
        REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
        REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
        CHIP_CLEARANCE
    );
    println!(
        "  Gasket compression:     {:.2} mm free -> {:.2} mm target; guard coupons {:.2}/{:.2} mm",
        GASKET_FREE_HEIGHT,
        GASKET_COMPRESSED_HEIGHT,
        GASKET_GUARD_MIN_COMPRESSED_HEIGHT,
        GASKET_GUARD_MAX_COMPRESSED_HEIGHT
    );
    println!(
        "  Dock rail coupon:       rear/left datum rails {:.1} mm wide x {:.1} mm high",
        DOCK_RAIL_W, DOCK_RAIL_Z
    );
    println!("  Bulkhead coupon:        M0-M6 media ports plus W0/W1/W3 waste check section");
    println!("  Print status:           dry no-cell fit-check only; not sterile or wetted path");
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap_or_else(|error| {
        panic!("failed to write print coupon {path}: {error:?}");
    });
    println!("Exported: {path}");
}

fn verify_binary_stl_size(path: &str, expected: [f64; 3], tolerance: f64) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| format!("failed to read {path}: {error}"))?;
    if bytes.len() < 84 {
        return Err(format!("{path} is too small for binary STL"));
    }
    let triangle_count = u32::from_le_bytes(
        bytes[80..84]
            .try_into()
            .map_err(|_| format!("{path} has an invalid triangle header"))?,
    ) as usize;
    let expected_length = 84usize
        .checked_add(
            triangle_count
                .checked_mul(50)
                .ok_or_else(|| format!("{path} triangle count overflows"))?,
        )
        .ok_or_else(|| format!("{path} length overflows"))?;
    if triangle_count == 0 || bytes.len() != expected_length {
        return Err(format!(
            "{path} has invalid binary STL length {} for {triangle_count} triangles",
            bytes.len()
        ));
    }

    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for triangle in 0..triangle_count {
        let triangle_start = 84 + triangle * 50;
        for vertex in 0..3 {
            let vertex_start = triangle_start + 12 + vertex * 12;
            for axis in 0..3 {
                let coordinate_start = vertex_start + axis * 4;
                let coordinate = f32::from_le_bytes(
                    bytes[coordinate_start..coordinate_start + 4]
                        .try_into()
                        .map_err(|_| format!("{path} has a truncated vertex"))?,
                ) as f64;
                if !coordinate.is_finite() {
                    return Err(format!("{path} contains a non-finite vertex"));
                }
                min[axis] = min[axis].min(coordinate);
                max[axis] = max[axis].max(coordinate);
            }
        }
    }
    for axis in 0..3 {
        let size = max[axis] - min[axis];
        if (size - expected[axis]).abs() > tolerance {
            return Err(format!(
                "{path} axis {axis} size {size:.4}mm does not match A0 {:.4}mm",
                expected[axis]
            ));
        }
    }
    Ok(())
}

fn chip_pocket_fit_coupon() -> Part {
    let body = centered_cube(
        "chip_pocket_coupon_body",
        CHIP_FIT_COUPON_X,
        CHIP_FIT_COUPON_Y,
        CARRIER_Z,
    );

    let pocket = centered_cube(
        "chip_pocket_coupon_rev_c_clearance_pocket",
        REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
        REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
        top_face_cut_height(CHIP_POCKET_DEPTH),
    )
    .translate(0.0, 0.0, top_face_cut_z(CARRIER_Z, CHIP_POCKET_DEPTH));

    let optical_window = centered_cube(
        "chip_pocket_coupon_optical_window_cut",
        REVC_CHIP_LENGTH - OPTICAL_WINDOW_MARGIN,
        REVC_CHIP_WIDTH - OPTICAL_WINDOW_MARGIN,
        CARRIER_Z + 2.0,
    );

    let gasket_land = rectangular_frame(
        "chip_pocket_coupon_gasket_land",
        PER_SLOT_GASKET_OUTER_X,
        PER_SLOT_GASKET_OUTER_Y,
        GASKET_LAND_Z + FEATURE_ANCHOR_OVERLAP,
        GASKET_LAND_W,
    )
    .translate(
        0.0,
        0.0,
        CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0 - FEATURE_ANCHOR_OVERLAP / 2.0,
    );

    let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
    let mut stops = Part::empty("chip_pocket_coupon_25pct_hard_stops");
    for (i, (x, y)) in [
        (-CHIP_COUPON_STOP_OFFSET_X, -CHIP_COUPON_STOP_OFFSET_Y),
        (CHIP_COUPON_STOP_OFFSET_X, -CHIP_COUPON_STOP_OFFSET_Y),
        (-CHIP_COUPON_STOP_OFFSET_X, CHIP_COUPON_STOP_OFFSET_Y),
        (CHIP_COUPON_STOP_OFFSET_X, CHIP_COUPON_STOP_OFFSET_Y),
    ]
    .into_iter()
    .enumerate()
    {
        stops = stops
            + centered_cylinder(
                format!("chip_pocket_coupon_hard_stop_{i}"),
                INTERNAL_STOP_DIAMETER / 2.0,
                stop_z + FEATURE_ANCHOR_OVERLAP,
                24,
            )
            .translate(
                x,
                y,
                CARRIER_Z / 2.0 + stop_z / 2.0 - FEATURE_ANCHOR_OVERLAP / 2.0,
            );
    }

    body - pocket - optical_window + gasket_land + stops
}

fn gasket_compression_coupon() -> Part {
    let coupon_x = 190.0;
    let coupon_y = 88.0;
    let coupon_z = 12.0;
    let mut coupon = centered_cube(
        "gasket_compression_coupon_body",
        coupon_x,
        coupon_y,
        coupon_z,
    );

    for (i, (name, y, compressed_height)) in [
        (
            "max_squeeze_30pct",
            26.0,
            GASKET_GUARD_MIN_COMPRESSED_HEIGHT,
        ),
        ("target_squeeze_25pct", 0.0, GASKET_COMPRESSED_HEIGHT),
        (
            "min_squeeze_20pct",
            -26.0,
            GASKET_GUARD_MAX_COMPRESSED_HEIGHT,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let groove = centered_cube(
            format!("gasket_compression_coupon_{name}_groove_cut"),
            138.0,
            GASKET_GROOVE_W,
            GASKET_GROOVE_CUT_HEIGHT,
        )
        .translate(0.0, y, top_face_gasket_groove_cut_z(coupon_z));

        let left_stop = centered_cube(
            format!("gasket_compression_coupon_{name}_left_stop"),
            14.0,
            18.0,
            compressed_height,
        )
        .translate(-78.0, y, coupon_z / 2.0 + compressed_height / 2.0);
        let right_stop = centered_cube(
            format!("gasket_compression_coupon_{name}_right_stop"),
            14.0,
            18.0,
            compressed_height,
        )
        .translate(78.0, y, coupon_z / 2.0 + compressed_height / 2.0);
        let witness_pad = centered_cube(
            format!("gasket_compression_coupon_{name}_witness_pad_{i}"),
            34.0,
            18.0,
            compressed_height,
        )
        .translate(0.0, y, coupon_z / 2.0 + compressed_height / 2.0);

        coupon = coupon - groove + left_stop + right_stop + witness_pad;
    }

    let mounting_holes =
        centered_cylinder("gasket_coupon_mount_hole_left", 2.7, coupon_z + 2.0, 24)
            .translate(-86.0, -38.0, 0.0)
            + centered_cylinder("gasket_coupon_mount_hole_right", 2.7, coupon_z + 2.0, 24)
                .translate(86.0, -38.0, 0.0);

    coupon - mounting_holes
}

fn dock_datum_rail_coupon() -> Part {
    let base_x = 198.0;
    let base_y = 178.0;
    let base = centered_cube("dock_datum_rail_coupon_base", base_x, base_y, DOCK_Z);

    let rear_rail = centered_cube(
        "dock_datum_rail_coupon_rear_primary_rail",
        base_x - 26.0,
        DOCK_RAIL_W,
        DOCK_RAIL_Z,
    )
    .translate(0.0, base_y / 2.0 - 24.0, DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0);

    let left_rail = centered_cube(
        "dock_datum_rail_coupon_left_secondary_rail",
        DOCK_RAIL_W,
        base_y - 30.0,
        DOCK_RAIL_Z,
    )
    .translate(-base_x / 2.0 + 24.0, -2.0, DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0);

    let front_lip = centered_cube(
        "dock_datum_rail_coupon_front_low_retention_lip",
        base_x - 26.0,
        10.0,
        10.0,
    )
    .translate(0.0, -base_y / 2.0 + 20.0, DOCK_Z / 2.0 + 5.0);

    let gutter = centered_cube(
        "dock_datum_rail_coupon_visible_leak_gutter",
        base_x - 52.0,
        10.0,
        DOCK_Z + 2.0,
    )
    .translate(10.0, -base_y / 2.0 + 44.0, 0.0);

    let mounting_holes = centered_cylinder("dock_datum_rail_coupon_m5_left", 2.7, DOCK_Z + 2.0, 24)
        .translate(-base_x / 2.0 + 22.0, -base_y / 2.0 + 22.0, 0.0)
        + centered_cylinder("dock_datum_rail_coupon_m5_right", 2.7, DOCK_Z + 2.0, 24).translate(
            base_x / 2.0 - 22.0,
            base_y / 2.0 - 22.0,
            0.0,
        );

    base + rear_rail + left_rail + front_lip - gutter - mounting_holes
}

fn carrier_corner_surrogate() -> Part {
    let plate_x = 156.0;
    let plate_y = 132.0;
    let plate_z = 24.0;
    let plate = centered_cube("carrier_corner_surrogate_plate", plate_x, plate_y, plate_z);

    let rear_contact = centered_cube(
        "carrier_corner_surrogate_rear_datum_face_witness",
        plate_x - 24.0,
        6.0,
        12.0,
    )
    .translate(6.0, plate_y / 2.0 - 3.0, plate_z / 2.0 + 6.0);

    let left_contact = centered_cube(
        "carrier_corner_surrogate_left_datum_face_witness",
        6.0,
        plate_y - 24.0,
        12.0,
    )
    .translate(-plate_x / 2.0 + 3.0, -6.0, plate_z / 2.0 + 6.0);

    let handling_land = centered_cube("carrier_corner_surrogate_handling_land", 58.0, 16.0, 1.5)
        .translate(10.0, -plate_y / 2.0 + 28.0, plate_z / 2.0 + 0.75);

    let orientation_marker = centered_cube(
        "carrier_corner_surrogate_slot_one_orientation_marker_x",
        34.0,
        5.0,
        3.0,
    )
    .translate(
        -plate_x / 2.0 + 34.0,
        -plate_y / 2.0 + 28.0,
        plate_z / 2.0 + 1.5,
    ) + centered_cube(
        "carrier_corner_surrogate_slot_one_orientation_marker_y",
        5.0,
        34.0,
        3.0,
    )
    .translate(
        -plate_x / 2.0 + 20.0,
        -plate_y / 2.0 + 42.0,
        plate_z / 2.0 + 1.5,
    );

    let relief = centered_cube(
        "carrier_corner_surrogate_leak_relief_channel",
        plate_x - 34.0,
        8.0,
        plate_z + 2.0,
    )
    .translate(10.0, -plate_y / 2.0 + 12.0, 0.0);

    plate + rear_contact + left_contact + handling_land + orientation_marker - relief
}

fn bulkhead_connector_mockup() -> Part {
    let block_x = 232.0;
    let block_y = 52.0;
    let block_z = 76.0;
    let block = centered_cube("bulkhead_connector_mockup_body", block_x, block_y, block_z);

    let mut media_ports = Part::empty("bulkhead_connector_mockup_media_port_cuts");
    for (i, x) in [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0]
        .into_iter()
        .enumerate()
    {
        media_ports = media_ports
            + centered_cylinder(
                format!("bulkhead_connector_mockup_m{i}"),
                3.2,
                block_y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
    }

    let mut waste_ports = Part::empty("bulkhead_connector_mockup_waste_port_cuts");
    for (i, x) in [124.0, 150.0, 176.0].into_iter().enumerate() {
        waste_ports = waste_ports
            + centered_cylinder(
                format!("bulkhead_connector_mockup_w{i}"),
                3.2,
                block_y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 52.0, 0.0, 0.0);
    }

    let label_strip = centered_cube(
        "bulkhead_connector_mockup_label_strip",
        block_x - 24.0,
        2.0,
        8.0,
    )
    .translate(0.0, -block_y / 2.0 - 1.0, block_z / 2.0 - 14.0);

    let mut comb = Part::empty("bulkhead_connector_mockup_strain_relief_comb");
    for i in 0..10 {
        comb = comb
            + centered_cube(
                format!("bulkhead_connector_mockup_comb_tooth_{i}"),
                5.0,
                24.0,
                18.0,
            )
            .translate(centered_index(i, 10, 20.0), -block_y / 2.0 - 12.0, -20.0);
    }

    let mounting_holes = centered_cylinder(
        "bulkhead_connector_mockup_mount_left",
        2.7,
        block_y + 4.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-block_x / 2.0 + 18.0, 0.0, -block_z / 2.0 + 16.0)
        + centered_cylinder(
            "bulkhead_connector_mockup_mount_right",
            2.7,
            block_y + 4.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(block_x / 2.0 - 18.0, 0.0, -block_z / 2.0 + 16.0);

    block - media_ports - waste_ports - mounting_holes + label_strip + comb
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - wall * 2.0,
        y - wall * 2.0,
        z + 2.0,
    );
    outer - inner
}

fn assert_coupon_envelopes() {
    assert!(REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0 < PRINT_BED_TARGET_XY);
    assert!(REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0 < PRINT_BED_TARGET_XY);
    assert!((CHIP_CLEARANCE - 1.20).abs() < 1e-9);
    assert!((DRAWING_TARGET_CHIP_CLEARANCE - 0.80).abs() < 1e-9);
    assert!((GASKET_COMPRESSED_HEIGHT - 1.80).abs() < 1e-9);
    assert_eq!(OUTPUTS.len(), 5);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with("output/print_coupons/sixteen_slot_")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coupon_outputs_are_stable() {
        assert_eq!(OUTPUTS.len(), 5);
        assert!(OUTPUTS.contains(&"output/print_coupons/sixteen_slot_chip_pocket_fit_coupon.stl"));
        assert!(
            OUTPUTS.contains(&"output/print_coupons/sixteen_slot_gasket_compression_coupon.stl")
        );
        assert!(
            OUTPUTS.contains(&"output/print_coupons/sixteen_slot_bulkhead_connector_mockup.stl")
        );
    }

    #[test]
    fn coupons_preserve_controlling_interface_values() {
        const TOL: f64 = 1e-9;
        assert!((CHIP_CLEARANCE - 1.20).abs() < TOL);
        assert!((DRAWING_TARGET_CHIP_CLEARANCE - 0.80).abs() < TOL);
        assert!((GASKET_FREE_HEIGHT - 2.40).abs() < TOL);
        assert!((GASKET_COMPRESSED_HEIGHT - 1.80).abs() < TOL);
        assert!((GASKET_GROOVE_DEPTH - 1.80).abs() < TOL);
        assert!((GASKET_GROOVE_W - 3.20).abs() < TOL);
        assert!(PER_SLOT_GASKET_INNER_X > REVC_CHIP_LENGTH);
        assert!(PER_SLOT_GASKET_INNER_Y > REVC_CHIP_WIDTH);
        assert!(
            CHIP_COUPON_STOP_OFFSET_X - INTERNAL_STOP_DIAMETER / 2.0
                > PER_SLOT_GASKET_OUTER_X / 2.0
        );
        assert!(
            CHIP_COUPON_STOP_OFFSET_Y - INTERNAL_STOP_DIAMETER / 2.0
                > PER_SLOT_GASKET_OUTER_Y / 2.0
        );
        assert!((CLOSURE_PLANE_ABOVE_CARRIER - GASKET_LAND_Z).abs() < TOL);
        assert_eq!(DOCK_RAIL_W, 16.0);
        assert_eq!(DOCK_RAIL_Z, 18.0);
        assert_eq!(FEATURE_ANCHOR_OVERLAP, 0.40);
    }

    #[test]
    fn primary_print_envelopes_fit_common_bambu_bed() {
        assert!(CHIP_FIT_COUPON_X <= PRINT_BED_TARGET_XY);
        assert!(CHIP_FIT_COUPON_Y <= PRINT_BED_TARGET_XY);
        assert!(190.0 <= PRINT_BED_TARGET_XY);
        assert!(232.0 <= PRINT_BED_TARGET_XY);
    }

    #[test]
    fn chip_fit_coupon_features_reach_the_shared_closure_plane() {
        const TOL: f64 = 1e-9;
        let land_primitive_height = GASKET_LAND_Z + FEATURE_ANCHOR_OVERLAP;
        let land_center_z = CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0 - FEATURE_ANCHOR_OVERLAP / 2.0;
        let stop_primitive_height = CLOSURE_PLANE_ABOVE_CARRIER + FEATURE_ANCHOR_OVERLAP;
        let stop_center_z =
            CARRIER_Z / 2.0 + CLOSURE_PLANE_ABOVE_CARRIER / 2.0 - FEATURE_ANCHOR_OVERLAP / 2.0;
        let closure_z = CARRIER_Z / 2.0 + CLOSURE_PLANE_ABOVE_CARRIER;

        assert!((land_center_z + land_primitive_height / 2.0 - closure_z).abs() < TOL);
        assert!((stop_center_z + stop_primitive_height / 2.0 - closure_z).abs() < TOL);
        assert!(
            (land_center_z
                - land_primitive_height / 2.0
                - (CARRIER_Z / 2.0 - FEATURE_ANCHOR_OVERLAP))
                .abs()
                < TOL
        );
        assert!(
            (stop_center_z
                - stop_primitive_height / 2.0
                - (CARRIER_Z / 2.0 - FEATURE_ANCHOR_OVERLAP))
                .abs()
                < TOL
        );
        assert!((CHIP_FIT_COUPON_OVERALL_Z - (closure_z + CARRIER_Z / 2.0)).abs() < TOL);
    }
}
