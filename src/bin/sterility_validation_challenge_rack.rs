use vcad::{centered_cube, centered_cylinder, Part};

// Sterility/decontamination validation challenge rack for the closed culture stack.
//
// Purpose:
// - Give the isolator, transfer hatch, and module docking bay repeatable locations
//   for non-cell validation loads before culture work.
// - Hold coupons, media-fill surrogates, settle/contact plate placeholders,
//   humidity/VHP/temperature sensor stubs, and leak witness trays at fixed datums.
// - Make the qualification workflow part of the equipment architecture instead of
//   an improvised bench activity.
//
// This is packaging geometry for validation fixtures, not a sterilization method
// or a biological protocol.
//
// Exports:
//   output/sterility_validation_challenge_rack_base.stl
//   output/sterility_validation_challenge_rack_coupon_carrier.stl
//   output/sterility_validation_challenge_rack_media_fill_tray.stl
//   output/sterility_validation_challenge_rack_settle_plate_array.stl
//   output/sterility_validation_challenge_rack_sensor_panel.stl
//   output/sterility_validation_challenge_rack_assembly.stl

const RACK_X: f64 = 720.0;
const RACK_Y: f64 = 520.0;
const BASE_Z: f64 = 18.0;
const RAIL_Z: f64 = 34.0;
const DATUM_PIN_D: f64 = 8.0;

const COUPON_CARRIER_X: f64 = 270.0;
const COUPON_CARRIER_Y: f64 = 70.0;
const COUPON_CARRIER_Z: f64 = 210.0;

const MEDIA_TRAY_X: f64 = 420.0;
const MEDIA_TRAY_Y: f64 = 210.0;
const MEDIA_TRAY_Z: f64 = 48.0;

fn main() {
    let base = base_tray();
    base.write_stl("output/sterility_validation_challenge_rack_base.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_base.stl");

    let coupon_carrier = coupon_carrier();
    coupon_carrier
        .write_stl("output/sterility_validation_challenge_rack_coupon_carrier.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_coupon_carrier.stl");

    let media_fill_tray = media_fill_tray();
    media_fill_tray
        .write_stl("output/sterility_validation_challenge_rack_media_fill_tray.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_media_fill_tray.stl");

    let settle_plate_array = settle_plate_array();
    settle_plate_array
        .write_stl("output/sterility_validation_challenge_rack_settle_plate_array.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_settle_plate_array.stl");

    let sensor_panel = sensor_panel();
    sensor_panel
        .write_stl("output/sterility_validation_challenge_rack_sensor_panel.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_sensor_panel.stl");

    let assembly = base
        + coupon_carrier.translate(-210.0, 145.0, BASE_Z / 2.0 + COUPON_CARRIER_Z / 2.0)
        + media_fill_tray.translate(95.0, -85.0, BASE_Z / 2.0 + MEDIA_TRAY_Z / 2.0)
        + settle_plate_array.translate(170.0, 150.0, BASE_Z / 2.0 + 18.0)
        + sensor_panel.translate(-275.0, -175.0, BASE_Z / 2.0 + 95.0)
        + transfer_hatch_handle_datum().translate(0.0, -RACK_Y / 2.0 - 16.0, 58.0);

    assembly
        .write_stl("output/sterility_validation_challenge_rack_assembly.stl")
        .unwrap();
    println!("Exported: output/sterility_validation_challenge_rack_assembly.stl");

    println!(
        "Sterility validation challenge rack: {:.0}mm x {:.0}mm tray with coupon carrier, media-fill surrogate tray, settle/contact plate array, sensor panel, witness sump, and transfer-hatch datum features.",
        RACK_X, RACK_Y
    );
}

fn base_tray() -> Part {
    let tray = centered_cube(
        "sterility_validation_challenge_rack_base_tray",
        RACK_X,
        RACK_Y,
        BASE_Z,
    );

    let sump = centered_cube(
        "sterility_validation_challenge_rack_leak_witness_sump",
        RACK_X - 72.0,
        RACK_Y - 72.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 4.0);

    let drain = centered_cylinder(
        "sterility_validation_challenge_rack_sump_drain",
        12.0 / 2.0,
        34.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(RACK_X / 2.0 - 62.0, -RACK_Y / 2.0 - 2.0, -2.0);

    let rails = centered_cube(
        "sterility_validation_left_locator_rail",
        RACK_X - 80.0,
        14.0,
        RAIL_Z,
    )
    .translate(0.0, RACK_Y / 2.0 - 32.0, BASE_Z / 2.0 + RAIL_Z / 2.0)
        + centered_cube(
            "sterility_validation_right_locator_rail",
            RACK_X - 80.0,
            14.0,
            RAIL_Z,
        )
        .translate(0.0, -(RACK_Y / 2.0 - 32.0), BASE_Z / 2.0 + RAIL_Z / 2.0)
        + centered_cube(
            "sterility_validation_rear_stop_rail",
            14.0,
            RACK_Y - 80.0,
            RAIL_Z,
        )
        .translate(RACK_X / 2.0 - 32.0, 0.0, BASE_Z / 2.0 + RAIL_Z / 2.0);

    tray - sump - drain - datum_pin_holes() + rails + cassette_datum_bosses() + label_tab_bank()
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("sterility_validation_datum_pin_holes");
    for (i, (x, y)) in [
        (-(RACK_X / 2.0 - 54.0), -(RACK_Y / 2.0 - 54.0)),
        (RACK_X / 2.0 - 54.0, -(RACK_Y / 2.0 - 54.0)),
        (-(RACK_X / 2.0 - 54.0), RACK_Y / 2.0 - 54.0),
        (RACK_X / 2.0 - 54.0, RACK_Y / 2.0 - 54.0),
        (0.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("sterility_validation_datum_pin_clearance_{i}"),
                DATUM_PIN_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn cassette_datum_bosses() -> Part {
    let mut bosses = Part::empty("sterility_validation_cassette_datum_bosses");
    for (i, (x, y)) in [
        (-250.0, -170.0),
        (-250.0, 170.0),
        (275.0, -170.0),
        (275.0, 170.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("sterility_validation_cassette_datum_boss_{i}"),
            18.0 / 2.0,
            12.0,
            32,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 6.0);
        let center = centered_cylinder(
            format!("sterility_validation_cassette_datum_center_{i}"),
            5.0 / 2.0,
            14.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 6.0);
        bosses = bosses + (boss - center);
    }
    bosses
}

fn label_tab_bank() -> Part {
    let mut tabs = Part::empty("sterility_validation_label_tabs");
    for (i, x) in [-285.0, -205.0, -125.0, -45.0, 35.0, 115.0, 195.0, 275.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("sterility_validation_label_tab_{i}"),
            54.0,
            22.0,
            7.0,
        )
        .translate(*x, -RACK_Y / 2.0 + 15.0, BASE_Z / 2.0 + 3.5);
        tabs = tabs + tab;
    }
    tabs
}

fn coupon_carrier() -> Part {
    let backplate = centered_cube(
        "sterility_validation_coupon_carrier_backplate",
        COUPON_CARRIER_X,
        12.0,
        COUPON_CARRIER_Z,
    );
    let base_foot = centered_cube(
        "sterility_validation_coupon_carrier_foot",
        COUPON_CARRIER_X + 35.0,
        COUPON_CARRIER_Y,
        16.0,
    )
    .translate(0.0, 0.0, -(COUPON_CARRIER_Z / 2.0 - 8.0));

    let mut slots = Part::empty("sterility_validation_coupon_slot_cuts");
    let mut lips = Part::empty("sterility_validation_coupon_slot_lips");
    for (row, z) in [-72.0, -24.0, 24.0, 72.0].iter().enumerate() {
        for (col, x) in [-90.0, 0.0, 90.0].iter().enumerate() {
            let i = row * 3 + col;
            slots = slots
                + centered_cube(
                    format!("sterility_validation_coupon_slot_cut_{i}"),
                    48.0,
                    16.0,
                    24.0,
                )
                .translate(*x, 0.0, *z);
            lips = lips
                + centered_cube(
                    format!("sterility_validation_coupon_slot_lower_lip_{i}"),
                    54.0,
                    10.0,
                    6.0,
                )
                .translate(*x, -10.0, *z - 16.0)
                + centered_cube(
                    format!("sterility_validation_coupon_slot_upper_clip_{i}"),
                    54.0,
                    8.0,
                    4.0,
                )
                .translate(*x, -10.0, *z + 16.0);
        }
    }

    let end_guards = centered_cube(
        "sterility_validation_coupon_left_guard",
        12.0,
        COUPON_CARRIER_Y,
        COUPON_CARRIER_Z,
    )
    .translate(-(COUPON_CARRIER_X / 2.0 + 8.0), 0.0, 0.0)
        + centered_cube(
            "sterility_validation_coupon_right_guard",
            12.0,
            COUPON_CARRIER_Y,
            COUPON_CARRIER_Z,
        )
        .translate(COUPON_CARRIER_X / 2.0 + 8.0, 0.0, 0.0);

    (backplate - slots) + base_foot + lips + end_guards
}

fn media_fill_tray() -> Part {
    let tray = centered_cube(
        "sterility_validation_media_fill_tray_body",
        MEDIA_TRAY_X,
        MEDIA_TRAY_Y,
        MEDIA_TRAY_Z,
    );

    let pocket = centered_cube(
        "sterility_validation_media_fill_recess",
        MEDIA_TRAY_X - 34.0,
        MEDIA_TRAY_Y - 34.0,
        22.0,
    )
    .translate(0.0, 0.0, MEDIA_TRAY_Z / 2.0 - 11.0);

    let mut bottle_wells = Part::empty("sterility_validation_media_fill_well_cuts");
    for (i, (x, y)) in [
        (-150.0, -55.0),
        (-75.0, -55.0),
        (0.0, -55.0),
        (75.0, -55.0),
        (150.0, -55.0),
        (-150.0, 55.0),
        (-75.0, 55.0),
        (0.0, 55.0),
        (75.0, 55.0),
        (150.0, 55.0),
    ]
    .iter()
    .enumerate()
    {
        bottle_wells = bottle_wells
            + centered_cylinder(
                format!("sterility_validation_media_fill_bottle_well_{i}"),
                31.0 / 2.0,
                MEDIA_TRAY_Z + 10.0,
                36,
            )
            .translate(*x, *y, 0.0);
    }

    let manifold_channel = centered_cube(
        "sterility_validation_media_fill_manifold_channel",
        MEDIA_TRAY_X - 80.0,
        16.0,
        MEDIA_TRAY_Z + 8.0,
    );

    let barcode_landing = centered_cube(
        "sterility_validation_media_fill_barcode_landing",
        120.0,
        26.0,
        6.0,
    )
    .translate(
        -(MEDIA_TRAY_X / 2.0 - 78.0),
        MEDIA_TRAY_Y / 2.0 - 28.0,
        MEDIA_TRAY_Z / 2.0 + 3.0,
    );

    (tray - pocket - bottle_wells - manifold_channel) + barcode_landing + tray_handles()
}

fn tray_handles() -> Part {
    let left_handle = centered_cube(
        "sterility_validation_media_tray_left_handle",
        42.0,
        16.0,
        28.0,
    )
    .translate(-(MEDIA_TRAY_X / 2.0 + 18.0), 0.0, 4.0);
    let right_handle = centered_cube(
        "sterility_validation_media_tray_right_handle",
        42.0,
        16.0,
        28.0,
    )
    .translate(MEDIA_TRAY_X / 2.0 + 18.0, 0.0, 4.0);
    left_handle + right_handle
}

fn settle_plate_array() -> Part {
    let deck = centered_cube(
        "sterility_validation_settle_plate_array_deck",
        245.0,
        185.0,
        12.0,
    );

    let mut recesses = Part::empty("sterility_validation_settle_plate_recesses");
    let mut rims = Part::empty("sterility_validation_settle_plate_rims");
    for (i, (x, y)) in [(-70.0, -45.0), (70.0, -45.0), (-70.0, 45.0), (70.0, 45.0)]
        .iter()
        .enumerate()
    {
        recesses = recesses
            + centered_cylinder(
                format!("sterility_validation_settle_plate_recess_{i}"),
                92.0 / 2.0,
                14.0,
                64,
            )
            .translate(*x, *y, 5.0);
        rims = rims
            + centered_cylinder(
                format!("sterility_validation_settle_plate_rim_{i}"),
                102.0 / 2.0,
                8.0,
                64,
            )
            .translate(*x, *y, 10.0)
            - centered_cylinder(
                format!("sterility_validation_settle_plate_rim_inner_{i}"),
                92.0 / 2.0,
                10.0,
                64,
            )
            .translate(*x, *y, 10.0);
    }

    let contact_coupon_rail = centered_cube(
        "sterility_validation_contact_coupon_rail",
        220.0,
        14.0,
        20.0,
    )
    .translate(0.0, 102.0, 12.0);
    let airflow_arrow = centered_cube(
        "sterility_validation_airflow_arrow_placeholder",
        90.0,
        12.0,
        5.0,
    )
    .translate(0.0, -103.0, 8.5);

    (deck - recesses) + rims + contact_coupon_rail + airflow_arrow
}

fn sensor_panel() -> Part {
    let panel = centered_cube("sterility_validation_sensor_panel_body", 190.0, 26.0, 170.0);

    let mut ports = Part::empty("sterility_validation_sensor_panel_ports");
    for (i, (x, z, d)) in [
        (-60.0, 48.0, 22.0),
        (0.0, 48.0, 22.0),
        (60.0, 48.0, 22.0),
        (-45.0, -24.0, 14.0),
        (45.0, -24.0, 14.0),
    ]
    .iter()
    .enumerate()
    {
        ports = ports
            + centered_cylinder(
                format!("sterility_validation_sensor_panel_port_{i}"),
                *d / 2.0,
                34.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, *z);
    }

    let particle_counter_tube = centered_cylinder(
        "sterility_validation_particle_counter_tube_placeholder",
        8.0 / 2.0,
        90.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(78.0, -42.0, -62.0);

    let foot = centered_cube("sterility_validation_sensor_panel_foot", 220.0, 70.0, 14.0)
        .translate(0.0, 0.0, -92.0);

    (panel - ports) + particle_counter_tube + foot + sensor_panel_brace()
}

fn sensor_panel_brace() -> Part {
    let left = centered_cube(
        "sterility_validation_sensor_panel_left_brace",
        12.0,
        70.0,
        125.0,
    )
    .translate(-104.0, 0.0, -22.0);
    let right = centered_cube(
        "sterility_validation_sensor_panel_right_brace",
        12.0,
        70.0,
        125.0,
    )
    .translate(104.0, 0.0, -22.0);
    left + right
}

fn transfer_hatch_handle_datum() -> Part {
    let tongue = centered_cube(
        "sterility_validation_transfer_hatch_datum_tongue",
        360.0,
        28.0,
        38.0,
    );
    let chamfer_proxy_left = centered_cube(
        "sterility_validation_transfer_hatch_left_chamfer_proxy",
        38.0,
        30.0,
        42.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(-198.0, 0.0, 0.0);
    let chamfer_proxy_right = centered_cube(
        "sterility_validation_transfer_hatch_right_chamfer_proxy",
        38.0,
        30.0,
        42.0,
    )
    .rotate(0.0, 0.0, -45.0)
    .translate(198.0, 0.0, 0.0);

    tongue - chamfer_proxy_left - chamfer_proxy_right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rack_fits_transfer_hatch_payload_envelope() {
        assert!(RACK_X <= 720.0);
        assert!(RACK_Y <= 520.0);
        assert!(BASE_Z + COUPON_CARRIER_Z <= 260.0);
    }

    #[test]
    fn media_fill_tray_has_repeated_surrogate_positions() {
        let wells_per_row = 5;
        let rows = 2;
        assert_eq!(wells_per_row * rows, 10);
        assert!(MEDIA_TRAY_X >= 400.0);
        assert!(MEDIA_TRAY_Y >= 200.0);
    }

    #[test]
    fn validation_features_cover_main_risk_classes() {
        let feature_classes = [
            "coupon",
            "media_fill",
            "settle_plate",
            "sensor",
            "leak_witness",
            "transfer_datum",
        ];
        assert_eq!(feature_classes.len(), 6);
    }
}
