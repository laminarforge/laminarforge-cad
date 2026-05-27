use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Sterile consumable cartridge hotel for the closed isolator/culture workcell.
//
// Intent:
// - Stage sterile tubing harnesses, disposable manifold inserts, sensor
//   cartridges, chip/cassette lids, sterile connector caps, and validation
//   coupons inside a repeatable isolator-side bin.
// - Keep clean items and used return items on separate keyed lanes so routine
//   replenishment does not fall back to open bench handling.
// - Provide a transfer-tray/RTP datum tongue, barcode/lot label lands, VHP/UV
//   exposure clearance placeholders, and front/rear/side service keepouts.
//
// This is mechanical packaging geometry for workflow planning. It is not a
// validated sterilization process or a sterility claim.

const OUTPUTS: &[&str] = &[
    "output/sterile_consumable_cartridge_hotel_base_tray.stl",
    "output/sterile_consumable_cartridge_hotel_clean_shelf_stack.stl",
    "output/sterile_consumable_cartridge_hotel_tubing_harness_nests.stl",
    "output/sterile_consumable_cartridge_hotel_manifold_insert_caddy.stl",
    "output/sterile_consumable_cartridge_hotel_sensor_cartridge_caddy.stl",
    "output/sterile_consumable_cartridge_hotel_lid_cap_coupon_caddy.stl",
    "output/sterile_consumable_cartridge_hotel_used_return_shelf.stl",
    "output/sterile_consumable_cartridge_hotel_exposure_clearance_placeholders.stl",
    "output/sterile_consumable_cartridge_hotel_service_clearances.stl",
    "output/sterile_consumable_cartridge_hotel_assembly.stl",
];

const HOTEL_X: f64 = 720.0;
const HOTEL_Y: f64 = 500.0;
const HOTEL_Z: f64 = 560.0;
const BASE_Z: f64 = 28.0;
const FRAME_W: f64 = 24.0;
const DIVIDER_W: f64 = 18.0;
const SHELF_Z: f64 = 14.0;
const SHELF_COUNT: usize = 4;
const SHELF_LEVELS: [f64; SHELF_COUNT] = [96.0, 210.0, 324.0, 438.0];

const LANE_X: f64 = (HOTEL_X - 2.0 * FRAME_W - DIVIDER_W) / 2.0;
const LANE_Y: f64 = HOTEL_Y - 118.0;
const CLEAN_LANE_CENTER_X: f64 = -(LANE_X / 2.0 + DIVIDER_W / 2.0);
const USED_LANE_CENTER_X: f64 = LANE_X / 2.0 + DIVIDER_W / 2.0;

const TRANSFER_TONGUE_X: f64 = 430.0;
const TRANSFER_TONGUE_Y: f64 = 34.0;
const TRANSFER_TONGUE_Z: f64 = 34.0;
const FRONT_SERVICE_CLEARANCE: f64 = 320.0;
const REAR_SERVICE_CLEARANCE: f64 = 180.0;
const SIDE_SERVICE_CLEARANCE: f64 = 160.0;

const TUBING_NEST_X: f64 = 298.0;
const TUBING_NEST_Y: f64 = 150.0;
const TUBING_NEST_Z: f64 = 26.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const HARNESS_LANES: usize = 4;

const MANIFOLD_CADDY_X: f64 = 300.0;
const MANIFOLD_CADDY_Y: f64 = 100.0;
const MANIFOLD_CADDY_Z: f64 = 38.0;
const MANIFOLD_SLOTS: usize = 5;

const SENSOR_CADDY_X: f64 = 306.0;
const SENSOR_CADDY_Y: f64 = 132.0;
const SENSOR_CADDY_Z: f64 = 36.0;
const SENSOR_SLOTS: usize = 8;

const LID_CAP_CADDY_X: f64 = 306.0;
const LID_CAP_CADDY_Y: f64 = 158.0;
const LID_CAP_CADDY_Z: f64 = 58.0;
const LID_SLOTS: usize = 4;
const CAP_WELLS: usize = 12;
const COUPON_SLOTS: usize = 8;

const USED_RETURN_X: f64 = 300.0;
const USED_RETURN_Y: f64 = 338.0;
const USED_RETURN_Z: f64 = 74.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_tray();
    export(&base, OUTPUTS[0]);

    let clean_stack = clean_shelf_stack();
    export(&clean_stack, OUTPUTS[1]);

    let tubing = tubing_harness_nests();
    export(&tubing, OUTPUTS[2]);

    let manifold = manifold_insert_caddy();
    export(&manifold, OUTPUTS[3]);

    let sensor = sensor_cartridge_caddy();
    export(&sensor, OUTPUTS[4]);

    let lid_cap_coupon = lid_cap_coupon_caddy();
    export(&lid_cap_coupon, OUTPUTS[5]);

    let used = used_return_shelf();
    export(&used, OUTPUTS[6]);

    let exposure = exposure_clearance_placeholders();
    export(&exposure, OUTPUTS[7]);

    let service = service_clearances();
    export(&service, OUTPUTS[8]);

    let assembly = base
        + clean_stack
        + tubing.translate(
            CLEAN_LANE_CENTER_X,
            -116.0,
            SHELF_LEVELS[0] + TUBING_NEST_Z / 2.0,
        )
        + manifold.translate(
            CLEAN_LANE_CENTER_X,
            74.0,
            SHELF_LEVELS[1] + MANIFOLD_CADDY_Z / 2.0,
        )
        + sensor.translate(
            CLEAN_LANE_CENTER_X,
            -104.0,
            SHELF_LEVELS[2] + SENSOR_CADDY_Z / 2.0,
        )
        + lid_cap_coupon.translate(
            CLEAN_LANE_CENTER_X,
            92.0,
            SHELF_LEVELS[3] + LID_CAP_CADDY_Z / 2.0,
        )
        + used.translate(
            USED_LANE_CENTER_X,
            -12.0,
            SHELF_LEVELS[0] + USED_RETURN_Z / 2.0,
        )
        + exposure
        + service;

    export(&assembly, OUTPUTS[9]);

    println!(
        "Sterile consumable cartridge hotel: {:.0}mm W x {:.0}mm D x {:.0}mm H, clean lane {:.0}mm wide, used-return lane {:.0}mm wide, {} shelf levels, {:.0}mm transfer-tray datum tongue, label lands, VHP/UV exposure placeholders, and service keepouts.",
        HOTEL_X,
        HOTEL_Y,
        HOTEL_Z,
        LANE_X,
        LANE_X,
        SHELF_COUNT,
        TRANSFER_TONGUE_X
    );
    println!(
        "Clean caddies: {} tubing harness nests, {} manifold slots, {} sensor cartridge slots, {} lid slots, {} cap wells, and {} coupon slots.",
        HARNESS_LANES,
        MANIFOLD_SLOTS,
        SENSOR_SLOTS,
        LID_SLOTS,
        CAP_WELLS,
        COUPON_SLOTS
    );
    println!(
        "Service clearances: front pull {:.0}mm, rear VHP/lot service {:.0}mm, side hand clearance {:.0}mm.",
        FRONT_SERVICE_CLEARANCE, REAR_SERVICE_CLEARANCE, SIDE_SERVICE_CLEARANCE
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_tray() -> Part {
    let pan = centered_cube("sterile_hotel_base_tray_pan", HOTEL_X, HOTEL_Y, BASE_Z).translate(
        0.0,
        0.0,
        BASE_Z / 2.0,
    );

    let sump = centered_cube(
        "sterile_hotel_base_recessed_spill_sump",
        HOTEL_X - 96.0,
        HOTEL_Y - 112.0,
        12.0,
    )
    .translate(0.0, 8.0, BASE_Z - 6.0);
    let drain = centered_cylinder("sterile_hotel_base_drain_port", 6.0 / 2.0, 38.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(HOTEL_X / 2.0 - 66.0, -HOTEL_Y / 2.0 + 18.0, BASE_Z - 8.0);

    let clean_floor_marker = centered_cube(
        "sterile_hotel_clean_lane_floor_marker",
        LANE_X - 36.0,
        LANE_Y - 52.0,
        4.0,
    )
    .translate(CLEAN_LANE_CENTER_X, 16.0, BASE_Z + 2.0);
    let used_floor_marker = centered_cube(
        "sterile_hotel_used_lane_floor_marker",
        LANE_X - 36.0,
        LANE_Y - 52.0,
        4.0,
    )
    .translate(USED_LANE_CENTER_X, 16.0, BASE_Z + 2.0);

    pan - sump - drain - base_datum_pin_holes()
        + transfer_tray_datum()
        + clean_floor_marker
        + used_floor_marker
        + base_label_lands()
        + base_lane_rails()
}

fn base_datum_pin_holes() -> Part {
    let mut holes = Part::empty("sterile_hotel_base_datum_pin_holes");
    for (i, (x, y)) in [
        (-(HOTEL_X / 2.0 - 52.0), -(HOTEL_Y / 2.0 - 52.0)),
        (HOTEL_X / 2.0 - 52.0, -(HOTEL_Y / 2.0 - 52.0)),
        (-(HOTEL_X / 2.0 - 52.0), HOTEL_Y / 2.0 - 52.0),
        (HOTEL_X / 2.0 - 52.0, HOTEL_Y / 2.0 - 52.0),
        (0.0, -(HOTEL_Y / 2.0 - 52.0)),
        (0.0, HOTEL_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("sterile_hotel_base_datum_pin_clearance_{i}"),
                8.0 / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn transfer_tray_datum() -> Part {
    let y = -(HOTEL_Y / 2.0 + TRANSFER_TONGUE_Y / 2.0 - 2.0);
    let tongue = centered_cube(
        "sterile_hotel_rtp_transfer_tray_datum_tongue",
        TRANSFER_TONGUE_X,
        TRANSFER_TONGUE_Y,
        TRANSFER_TONGUE_Z,
    )
    .translate(0.0, y, BASE_Z / 2.0 + TRANSFER_TONGUE_Z / 2.0);

    let left_chamfer = centered_cube(
        "sterile_hotel_transfer_tongue_left_chamfer_proxy",
        38.0,
        38.0,
        TRANSFER_TONGUE_Z + 4.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        -(TRANSFER_TONGUE_X / 2.0 + 4.0),
        y - TRANSFER_TONGUE_Y / 2.0,
        BASE_Z / 2.0 + TRANSFER_TONGUE_Z / 2.0,
    );
    let right_chamfer = centered_cube(
        "sterile_hotel_transfer_tongue_right_chamfer_proxy",
        38.0,
        38.0,
        TRANSFER_TONGUE_Z + 4.0,
    )
    .rotate(0.0, 0.0, -45.0)
    .translate(
        TRANSFER_TONGUE_X / 2.0 + 4.0,
        y - TRANSFER_TONGUE_Y / 2.0,
        BASE_Z / 2.0 + TRANSFER_TONGUE_Z / 2.0,
    );

    let datum_slot = centered_cube(
        "sterile_hotel_transfer_tray_center_key_slot",
        82.0,
        TRANSFER_TONGUE_Y + 4.0,
        10.0,
    )
    .translate(0.0, y, BASE_Z / 2.0 + TRANSFER_TONGUE_Z - 5.0);

    tongue - left_chamfer - right_chamfer - datum_slot
}

fn base_label_lands() -> Part {
    let mut lands = Part::empty("sterile_hotel_base_label_lands");
    for (i, (x, y, land_x)) in [
        (CLEAN_LANE_CENTER_X, -HOTEL_Y / 2.0 + 34.0, 188.0),
        (USED_LANE_CENTER_X, -HOTEL_Y / 2.0 + 34.0, 188.0),
        (CLEAN_LANE_CENTER_X, HOTEL_Y / 2.0 - 34.0, 188.0),
        (USED_LANE_CENTER_X, HOTEL_Y / 2.0 - 34.0, 188.0),
        (0.0, -HOTEL_Y / 2.0 + 74.0, 140.0),
    ]
    .iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("sterile_hotel_barcode_lot_label_land_{i}"),
                *land_x,
                28.0,
                5.0,
            )
            .translate(*x, *y, BASE_Z + 2.5);
    }
    lands
}

fn base_lane_rails() -> Part {
    let clean_left = centered_cube("sterile_hotel_clean_lane_left_low_rail", 12.0, LANE_Y, 28.0)
        .translate(
            CLEAN_LANE_CENTER_X - LANE_X / 2.0 + 12.0,
            12.0,
            BASE_Z + 14.0,
        );
    let clean_right = centered_cube(
        "sterile_hotel_clean_lane_divider_low_rail",
        12.0,
        LANE_Y,
        28.0,
    )
    .translate(-DIVIDER_W / 2.0 - 6.0, 12.0, BASE_Z + 14.0);
    let used_left = centered_cube(
        "sterile_hotel_used_lane_divider_low_rail",
        12.0,
        LANE_Y,
        28.0,
    )
    .translate(DIVIDER_W / 2.0 + 6.0, 12.0, BASE_Z + 14.0);
    let used_right = centered_cube("sterile_hotel_used_lane_right_low_rail", 12.0, LANE_Y, 28.0)
        .translate(
            USED_LANE_CENTER_X + LANE_X / 2.0 - 12.0,
            12.0,
            BASE_Z + 14.0,
        );

    clean_left + clean_right + used_left + used_right
}

fn clean_shelf_stack() -> Part {
    let mut stack = Part::empty("sterile_hotel_clean_shelf_stack");

    for (i, z) in SHELF_LEVELS.iter().enumerate() {
        stack = stack
            + centered_cube(
                format!("sterile_hotel_clean_shelf_deck_{i}"),
                LANE_X - 42.0,
                LANE_Y - 38.0,
                SHELF_Z,
            )
            .translate(CLEAN_LANE_CENTER_X, 18.0, *z)
            + centered_cube(
                format!("sterile_hotel_clean_shelf_front_lip_{i}"),
                LANE_X - 60.0,
                10.0,
                26.0,
            )
            .translate(CLEAN_LANE_CENTER_X, -LANE_Y / 2.0 + 44.0, *z + 15.0)
            + centered_cube(
                format!("sterile_hotel_clean_shelf_rear_stop_{i}"),
                LANE_X - 60.0,
                12.0,
                30.0,
            )
            .translate(CLEAN_LANE_CENTER_X, LANE_Y / 2.0 - 18.0, *z + 16.0);
    }

    stack + upright_frame() + center_segregation_spine() + clean_pull_handle_slots()
}

fn upright_frame() -> Part {
    let mut frame = Part::empty("sterile_hotel_upright_frame");
    for (i, (x, y)) in [
        (
            -(HOTEL_X / 2.0 - FRAME_W / 2.0),
            -(HOTEL_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            HOTEL_X / 2.0 - FRAME_W / 2.0,
            -(HOTEL_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            -(HOTEL_X / 2.0 - FRAME_W / 2.0),
            HOTEL_Y / 2.0 - FRAME_W / 2.0,
        ),
        (HOTEL_X / 2.0 - FRAME_W / 2.0, HOTEL_Y / 2.0 - FRAME_W / 2.0),
        (-(DIVIDER_W / 2.0), -(HOTEL_Y / 2.0 - FRAME_W / 2.0)),
        (DIVIDER_W / 2.0, HOTEL_Y / 2.0 - FRAME_W / 2.0),
    ]
    .iter()
    .enumerate()
    {
        frame = frame
            + centered_cube(
                format!("sterile_hotel_upright_post_{i}"),
                FRAME_W,
                FRAME_W,
                HOTEL_Z - BASE_Z,
            )
            .translate(*x, *y, BASE_Z + (HOTEL_Z - BASE_Z) / 2.0);
    }

    for (i, z) in [BASE_Z + 58.0, 274.0, HOTEL_Z - 28.0].iter().enumerate() {
        frame = frame
            + centered_cube(
                format!("sterile_hotel_rear_cross_rail_{i}"),
                HOTEL_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, HOTEL_Y / 2.0 - FRAME_W / 2.0, *z)
            + centered_cube(
                format!("sterile_hotel_clean_side_cross_rail_{i}"),
                FRAME_W,
                HOTEL_Y,
                FRAME_W,
            )
            .translate(-(HOTEL_X / 2.0 - FRAME_W / 2.0), 0.0, *z)
            + centered_cube(
                format!("sterile_hotel_used_side_cross_rail_{i}"),
                FRAME_W,
                HOTEL_Y,
                FRAME_W,
            )
            .translate(HOTEL_X / 2.0 - FRAME_W / 2.0, 0.0, *z);
    }

    frame
}

fn center_segregation_spine() -> Part {
    let divider = centered_cube(
        "sterile_hotel_clean_used_segregation_spine",
        DIVIDER_W,
        HOTEL_Y - 62.0,
        HOTEL_Z - BASE_Z - 18.0,
    )
    .translate(0.0, 18.0, BASE_Z + (HOTEL_Z - BASE_Z - 18.0) / 2.0);

    let front_interlock_land = centered_cube(
        "sterile_hotel_segregation_front_interlock_land",
        62.0,
        18.0,
        84.0,
    )
    .translate(0.0, -HOTEL_Y / 2.0 + 44.0, BASE_Z + 94.0);
    let rear_lot_land = centered_cube(
        "sterile_hotel_segregation_rear_lot_label_land",
        58.0,
        18.0,
        126.0,
    )
    .translate(0.0, HOTEL_Y / 2.0 - 48.0, BASE_Z + 160.0);

    divider + front_interlock_land + rear_lot_land
}

fn clean_pull_handle_slots() -> Part {
    let mut handles = Part::empty("sterile_hotel_clean_shelf_pull_handle_slots");
    for (i, z) in SHELF_LEVELS.iter().enumerate() {
        let left = centered_cube(
            format!("sterile_hotel_clean_shelf_pull_handle_left_{i}"),
            54.0,
            16.0,
            24.0,
        )
        .translate(CLEAN_LANE_CENTER_X - 84.0, -LANE_Y / 2.0 + 18.0, *z + 10.0);
        let right = centered_cube(
            format!("sterile_hotel_clean_shelf_pull_handle_right_{i}"),
            54.0,
            16.0,
            24.0,
        )
        .translate(CLEAN_LANE_CENTER_X + 84.0, -LANE_Y / 2.0 + 18.0, *z + 10.0);
        handles = handles + left + right;
    }
    handles
}

fn tubing_harness_nests() -> Part {
    let body = centered_cube(
        "sterile_hotel_tubing_harness_nest_body",
        TUBING_NEST_X,
        TUBING_NEST_Y,
        TUBING_NEST_Z,
    );

    let mut cuts = Part::empty("sterile_hotel_tubing_harness_channel_cuts");
    let tube_channel_d = TUBE_OD + TUBE_CLEARANCE;
    for i in 0..HARNESS_LANES {
        let y = lane_position(i, HARNESS_LANES, 28.0);
        cuts = cuts
            + centered_cylinder(
                format!("sterile_hotel_tubing_harness_main_channel_{i}"),
                tube_channel_d / 2.0,
                TUBING_NEST_X - 38.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -3.0)
            + centered_cube(
                format!("sterile_hotel_tubing_harness_top_loading_slot_{i}"),
                TUBING_NEST_X - 32.0,
                tube_channel_d + 1.6,
                TUBING_NEST_Z,
            )
            .translate(0.0, y, TUBING_NEST_Z / 2.0 - 6.0)
            + centered_cube(
                format!("sterile_hotel_tubing_harness_connector_window_{i}"),
                28.0,
                18.0,
                TUBING_NEST_Z + 2.0,
            )
            .translate(TUBING_NEST_X / 2.0 - 44.0, y, 0.0);
    }

    let keyed_notch = centered_cube(
        "sterile_hotel_tubing_harness_left_keyed_notch",
        42.0,
        20.0,
        TUBING_NEST_Z + 2.0,
    )
    .translate(
        -(TUBING_NEST_X / 2.0 - 38.0),
        TUBING_NEST_Y / 2.0 - 10.0,
        0.0,
    );

    body - cuts - keyed_notch
        + caddy_label_land(
            "sterile_hotel_tubing_harness",
            TUBING_NEST_X - 72.0,
            -TUBING_NEST_Y / 2.0 + 18.0,
            TUBING_NEST_Z,
        )
        + caddy_gripper_ears("sterile_hotel_tubing_harness", TUBING_NEST_X, TUBING_NEST_Z)
}

fn manifold_insert_caddy() -> Part {
    let body = centered_cube(
        "sterile_hotel_manifold_insert_caddy_body",
        MANIFOLD_CADDY_X,
        MANIFOLD_CADDY_Y,
        MANIFOLD_CADDY_Z,
    );

    let mut slots = Part::empty("sterile_hotel_manifold_insert_slot_cuts");
    let mut lips = Part::empty("sterile_hotel_manifold_insert_slot_lips");
    for i in 0..MANIFOLD_SLOTS {
        let x = lane_position(i, MANIFOLD_SLOTS, 54.0);
        slots = slots
            + centered_cube(
                format!("sterile_hotel_manifold_insert_slot_cut_{i}"),
                34.0,
                MANIFOLD_CADDY_Y + 6.0,
                24.0,
            )
            .translate(x, 0.0, MANIFOLD_CADDY_Z / 2.0 - 12.0)
            + centered_cube(
                format!("sterile_hotel_manifold_insert_key_relief_{i}"),
                18.0,
                28.0,
                MANIFOLD_CADDY_Z + 2.0,
            )
            .translate(x + 12.0, MANIFOLD_CADDY_Y / 2.0 - 14.0, 0.0);

        lips =
            lips + centered_cube(
                format!("sterile_hotel_manifold_insert_front_lip_{i}"),
                42.0,
                8.0,
                12.0,
            )
            .translate(
                x,
                -(MANIFOLD_CADDY_Y / 2.0 - 8.0),
                MANIFOLD_CADDY_Z / 2.0 + 6.0,
            ) + centered_cube(
                format!("sterile_hotel_manifold_insert_rear_lip_{i}"),
                42.0,
                8.0,
                12.0,
            )
            .translate(
                x,
                MANIFOLD_CADDY_Y / 2.0 - 8.0,
                MANIFOLD_CADDY_Z / 2.0 + 6.0,
            );
    }

    body - slots
        + lips
        + caddy_label_land(
            "sterile_hotel_manifold_insert",
            MANIFOLD_CADDY_X - 88.0,
            -MANIFOLD_CADDY_Y / 2.0 + 16.0,
            MANIFOLD_CADDY_Z,
        )
        + caddy_gripper_ears(
            "sterile_hotel_manifold_insert",
            MANIFOLD_CADDY_X,
            MANIFOLD_CADDY_Z,
        )
}

fn sensor_cartridge_caddy() -> Part {
    let deck = centered_cube(
        "sterile_hotel_sensor_cartridge_caddy_deck",
        SENSOR_CADDY_X,
        SENSOR_CADDY_Y,
        SENSOR_CADDY_Z,
    );

    let mut pockets = Part::empty("sterile_hotel_sensor_cartridge_pockets");
    let mut datum_pips = Part::empty("sterile_hotel_sensor_cartridge_datum_pips");
    for row in 0..2 {
        for col in 0..4 {
            let i = row * 4 + col;
            let x = lane_position(col, 4, 66.0);
            let y = lane_position(row, 2, 46.0);
            pockets = pockets
                + centered_cube(
                    format!("sterile_hotel_sensor_cartridge_pocket_{i}"),
                    48.0,
                    34.0,
                    18.0,
                )
                .translate(x, y, SENSOR_CADDY_Z / 2.0 - 9.0)
                + centered_cube(
                    format!("sterile_hotel_sensor_cartridge_asym_key_{i}"),
                    14.0,
                    12.0,
                    SENSOR_CADDY_Z + 2.0,
                )
                .translate(x - 17.0, y + 11.0, 0.0);

            datum_pips = datum_pips
                + centered_cylinder(
                    format!("sterile_hotel_sensor_cartridge_datum_pip_{i}"),
                    3.0,
                    4.0,
                    20,
                )
                .translate(x + 18.0, y - 12.0, SENSOR_CADDY_Z / 2.0 + 2.0);
        }
    }

    let cable_gap = centered_cube(
        "sterile_hotel_sensor_cartridge_cable_tail_gap",
        SENSOR_CADDY_X - 52.0,
        12.0,
        SENSOR_CADDY_Z + 2.0,
    )
    .translate(0.0, SENSOR_CADDY_Y / 2.0 - 14.0, 0.0);

    deck - pockets - cable_gap
        + datum_pips
        + caddy_label_land(
            "sterile_hotel_sensor_cartridge",
            SENSOR_CADDY_X - 96.0,
            -SENSOR_CADDY_Y / 2.0 + 18.0,
            SENSOR_CADDY_Z,
        )
        + caddy_gripper_ears(
            "sterile_hotel_sensor_cartridge",
            SENSOR_CADDY_X,
            SENSOR_CADDY_Z,
        )
}

fn lid_cap_coupon_caddy() -> Part {
    let body = centered_cube(
        "sterile_hotel_lid_cap_coupon_caddy_body",
        LID_CAP_CADDY_X,
        LID_CAP_CADDY_Y,
        LID_CAP_CADDY_Z,
    );

    let mut cuts = Part::empty("sterile_hotel_lid_cap_coupon_cuts");
    let mut retainers = Part::empty("sterile_hotel_lid_cap_coupon_retainers");

    for i in 0..LID_SLOTS {
        let x = lane_position(i, LID_SLOTS, 58.0);
        cuts = cuts
            + centered_cube(
                format!("sterile_hotel_lid_vertical_slot_cut_{i}"),
                36.0,
                58.0,
                LID_CAP_CADDY_Z + 4.0,
            )
            .translate(x, -44.0, 0.0);
        retainers = retainers
            + centered_cube(
                format!("sterile_hotel_lid_slot_front_retainer_{i}"),
                42.0,
                8.0,
                34.0,
            )
            .translate(x, -78.0, LID_CAP_CADDY_Z / 2.0 + 10.0);
    }

    for i in 0..CAP_WELLS {
        let row = i / 6;
        let col = i % 6;
        let x = lane_position(col, 6, 40.0);
        let y = 18.0 + row as f64 * 36.0;
        cuts = cuts
            + centered_cylinder(
                format!("sterile_hotel_connector_cap_well_{i}"),
                14.0 / 2.0,
                LID_CAP_CADDY_Z + 4.0,
                28,
            )
            .translate(x, y, 0.0);
    }

    for i in 0..COUPON_SLOTS {
        let x = lane_position(i, COUPON_SLOTS, 32.0);
        cuts = cuts
            + centered_cube(
                format!("sterile_hotel_validation_coupon_slot_cut_{i}"),
                18.0,
                18.0,
                LID_CAP_CADDY_Z + 2.0,
            )
            .translate(x, LID_CAP_CADDY_Y / 2.0 - 18.0, 0.0);
        retainers = retainers
            + centered_cube(
                format!("sterile_hotel_validation_coupon_clip_{i}"),
                22.0,
                8.0,
                20.0,
            )
            .translate(
                x,
                LID_CAP_CADDY_Y / 2.0 - 38.0,
                LID_CAP_CADDY_Z / 2.0 + 10.0,
            );
    }

    body - cuts
        + retainers
        + caddy_label_land(
            "sterile_hotel_lid_cap_coupon",
            LID_CAP_CADDY_X - 98.0,
            -LID_CAP_CADDY_Y / 2.0 + 18.0,
            LID_CAP_CADDY_Z,
        )
        + caddy_gripper_ears(
            "sterile_hotel_lid_cap_coupon",
            LID_CAP_CADDY_X,
            LID_CAP_CADDY_Z,
        )
}

fn used_return_shelf() -> Part {
    let tray = centered_cube(
        "sterile_hotel_used_return_tray_body",
        USED_RETURN_X,
        USED_RETURN_Y,
        USED_RETURN_Z,
    );
    let basin = centered_cube(
        "sterile_hotel_used_return_recessed_basin",
        USED_RETURN_X - 38.0,
        USED_RETURN_Y - 46.0,
        USED_RETURN_Z - 18.0,
    )
    .translate(0.0, 0.0, 14.0);
    let drain = centered_cylinder("sterile_hotel_used_return_drain", 7.0 / 2.0, 32.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            USED_RETURN_X / 2.0 - 34.0,
            -(USED_RETURN_Y / 2.0 - 10.0),
            -USED_RETURN_Z / 2.0 + 16.0,
        );

    let used_harness_slot = centered_cube(
        "sterile_hotel_used_return_harness_slot",
        USED_RETURN_X - 70.0,
        42.0,
        USED_RETURN_Z + 2.0,
    )
    .translate(0.0, -102.0, 0.0);
    let used_lid_slot = centered_cube(
        "sterile_hotel_used_return_lid_slot",
        USED_RETURN_X - 88.0,
        34.0,
        USED_RETURN_Z + 2.0,
    )
    .translate(0.0, -34.0, 0.0);

    let return_guard = centered_cube(
        "sterile_hotel_used_return_high_front_guard",
        USED_RETURN_X + 10.0,
        18.0,
        92.0,
    )
    .translate(0.0, -(USED_RETURN_Y / 2.0 + 5.0), USED_RETURN_Z / 2.0 + 9.0);
    let rear_guard = centered_cube(
        "sterile_hotel_used_return_rear_guard",
        USED_RETURN_X + 10.0,
        16.0,
        72.0,
    )
    .translate(0.0, USED_RETURN_Y / 2.0 + 4.0, USED_RETURN_Z / 2.0 - 1.0);

    tray - basin - drain - used_harness_slot - used_lid_slot
        + return_guard
        + rear_guard
        + used_connector_cap_wells()
        + caddy_label_land(
            "sterile_hotel_used_return",
            USED_RETURN_X - 96.0,
            -USED_RETURN_Y / 2.0 + 24.0,
            USED_RETURN_Z,
        )
}

fn used_connector_cap_wells() -> Part {
    let mut wells = Part::empty("sterile_hotel_used_return_cap_wells");
    for i in 0..8 {
        let col = i % 4;
        let row = i / 4;
        let x = lane_position(col, 4, 52.0);
        let y = 50.0 + row as f64 * 46.0;
        let rim = centered_cylinder(
            format!("sterile_hotel_used_return_cap_well_rim_{i}"),
            18.0 / 2.0,
            10.0,
            28,
        )
        .translate(x, y, USED_RETURN_Z / 2.0 + 5.0);
        let bore = centered_cylinder(
            format!("sterile_hotel_used_return_cap_well_bore_{i}"),
            12.0 / 2.0,
            12.0,
            28,
        )
        .translate(x, y, USED_RETURN_Z / 2.0 + 5.0);
        wells = wells + (rim - bore);
    }
    wells
}

fn exposure_clearance_placeholders() -> Part {
    let uv_sweep = clearance_box(
        "sterile_hotel_uv_sweep_clearance_placeholder",
        HOTEL_X - 96.0,
        HOTEL_Y - 130.0,
        76.0,
        8.0,
    )
    .translate(0.0, 18.0, HOTEL_Z - 66.0);

    let vhp_front_gap = clearance_box(
        "sterile_hotel_vhp_front_gap_placeholder",
        HOTEL_X - 120.0,
        54.0,
        HOTEL_Z - 138.0,
        7.0,
    )
    .translate(0.0, -HOTEL_Y / 2.0 + 86.0, BASE_Z + 238.0);
    let vhp_rear_gap = clearance_box(
        "sterile_hotel_vhp_rear_gap_placeholder",
        HOTEL_X - 120.0,
        54.0,
        HOTEL_Z - 138.0,
        7.0,
    )
    .translate(0.0, HOTEL_Y / 2.0 - 84.0, BASE_Z + 238.0);

    let mut shelf_gap_ribs = Part::empty("sterile_hotel_vhp_shelf_gap_ribs");
    for (i, z) in [154.0, 268.0, 382.0, 496.0].iter().enumerate() {
        shelf_gap_ribs = shelf_gap_ribs
            + centered_cube(
                format!("sterile_hotel_vhp_crossflow_gap_clean_{i}"),
                LANE_X - 84.0,
                10.0,
                8.0,
            )
            .translate(CLEAN_LANE_CENTER_X, 0.0, *z)
            + centered_cube(
                format!("sterile_hotel_vhp_crossflow_gap_used_{i}"),
                LANE_X - 84.0,
                10.0,
                8.0,
            )
            .translate(USED_LANE_CENTER_X, 0.0, *z);
    }

    uv_sweep + vhp_front_gap + vhp_rear_gap + shelf_gap_ribs
}

fn service_clearances() -> Part {
    let front_pull = clearance_box(
        "sterile_hotel_front_pull_service_clearance",
        HOTEL_X - 90.0,
        FRONT_SERVICE_CLEARANCE,
        180.0,
        9.0,
    )
    .translate(
        0.0,
        -(HOTEL_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0 + 18.0),
        BASE_Z + 130.0,
    );

    let rear_lot_vhp = clearance_box(
        "sterile_hotel_rear_lot_vhp_service_clearance",
        HOTEL_X - 116.0,
        REAR_SERVICE_CLEARANCE,
        240.0,
        8.0,
    )
    .translate(
        0.0,
        HOTEL_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0 + 18.0,
        BASE_Z + 184.0,
    );

    let left_hand = clearance_box(
        "sterile_hotel_left_side_hand_clearance",
        SIDE_SERVICE_CLEARANCE,
        HOTEL_Y - 96.0,
        210.0,
        8.0,
    )
    .translate(
        -(HOTEL_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0 + 18.0),
        0.0,
        BASE_Z + 168.0,
    );
    let right_hand = clearance_box(
        "sterile_hotel_right_side_hand_clearance",
        SIDE_SERVICE_CLEARANCE,
        HOTEL_Y - 96.0,
        210.0,
        8.0,
    )
    .translate(
        HOTEL_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0 + 18.0,
        0.0,
        BASE_Z + 168.0,
    );

    front_pull + rear_lot_vhp + left_hand + right_hand
}

fn caddy_label_land(prefix: &str, land_x: f64, y: f64, z: f64) -> Part {
    centered_cube(
        format!("{prefix}_barcode_lot_label_land"),
        land_x,
        24.0,
        5.0,
    )
    .translate(0.0, y, z / 2.0 + 2.5)
}

fn caddy_gripper_ears(prefix: &str, caddy_x: f64, z: f64) -> Part {
    let left = centered_cube(format!("{prefix}_left_gripper_ear"), 22.0, 76.0, 24.0).translate(
        -(caddy_x / 2.0 + 11.0),
        0.0,
        z / 2.0 + 4.0,
    );
    let right = centered_cube(format!("{prefix}_right_gripper_ear"), 22.0, 76.0, 24.0).translate(
        caddy_x / 2.0 + 11.0,
        0.0,
        z / 2.0 + 4.0,
    );
    let left_slot = centered_cube(format!("{prefix}_left_gripper_slot"), 10.0, 56.0, 12.0)
        .translate(-(caddy_x / 2.0 + 11.0), 0.0, z / 2.0 + 4.0);
    let right_slot = centered_cube(format!("{prefix}_right_gripper_slot"), 10.0, 56.0, 12.0)
        .translate(caddy_x / 2.0 + 11.0, 0.0, z / 2.0 + 4.0);
    (left - left_slot) + (right - right_slot)
}

fn clearance_box(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut part = Part::empty(format!("{name}_rails"));

    for (i, dx) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_vertical_{i}_{j}"), rail, rail, z).translate(
                    dx * x / 2.0,
                    dy * y / 2.0,
                    0.0,
                );
        }
    }

    for (i, dz) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_x_rail_{i}_{j}"), x, rail, rail).translate(
                    0.0,
                    dy * y / 2.0,
                    dz * z / 2.0,
                );
        }
        for (j, dx) in [-1.0, 1.0].iter().enumerate() {
            part = part
                + centered_cube(format!("{name}_y_rail_{i}_{j}"), rail, y, rail).translate(
                    dx * x / 2.0,
                    0.0,
                    dz * z / 2.0,
                );
        }
    }

    part
}

fn lane_position(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hotel_fits_isolator_deck_module_envelope() {
        assert!(HOTEL_X <= 760.0);
        assert!(HOTEL_Y <= 540.0);
        assert!(HOTEL_Z <= 620.0);
    }

    #[test]
    fn clean_and_used_lanes_are_physically_segregated() {
        assert!(DIVIDER_W >= 18.0);
        assert!(CLEAN_LANE_CENTER_X < -DIVIDER_W);
        assert!(USED_LANE_CENTER_X > DIVIDER_W);
        assert_eq!(SHELF_COUNT, SHELF_LEVELS.len());
    }

    #[test]
    fn staged_consumables_cover_requested_classes() {
        assert!(HARNESS_LANES >= 4);
        assert!(MANIFOLD_SLOTS >= 5);
        assert!(SENSOR_SLOTS >= 8);
        assert!(LID_SLOTS >= 4);
        assert!(CAP_WELLS >= 12);
        assert!(COUPON_SLOTS >= 8);
    }

    #[test]
    fn service_and_exposure_clearances_are_explicit() {
        assert!(TRANSFER_TONGUE_X >= 420.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_SERVICE_CLEARANCE >= 160.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 150.0);
        assert!(SHELF_LEVELS[SHELF_COUNT - 1] + LID_CAP_CADDY_Z < HOTEL_Z);
    }
}
