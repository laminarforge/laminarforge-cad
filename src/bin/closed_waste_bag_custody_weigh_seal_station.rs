use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste-bag chain-of-custody weighing and seal-verification station.
//
// Research assumptions used for this concept geometry:
// - Commercial checkweigher and bench-scale stations package a protected
//   gravimetric pad/load-cell envelope, scan lands, reject/hold routing, and
//   service clearance into one mechanically keyed workflow.
// - Tamper-evident custody workflows commonly combine barcode/RFID scans,
//   seal/overbag checks, evidence cards or vial witness pockets, and explicit
//   disposition lanes before release.
// - Waste-bag handling should keep clean inbound staging, used bag custody,
//   leak/drip containment, and reject routing physically segregated.
//
// This is mechanical product-concept CAD for bought-in scales, seal check
// tools, barcode/RFID readers, and waste-routing interfaces. It is not a
// decontamination chemistry design, waste-treatment protocol, or custody SOP.

const OUTPUTS: &[&str] = &[
    "output/closed_waste_bag_custody_weigh_seal_station_base_leak_drip_tray.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_sealed_bag_receiver.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_gravimetric_pad.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_seal_verification_pockets.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_overbag_cap_check_lands.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_barcode_rfid_chain_custody_lands.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_released_hold_reject_lanes.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_evidence_vial_card_pockets.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_clean_used_segregation_barriers.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_waste_routing_chutes.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_robot_service_keepouts.stl",
    "output/closed_waste_bag_custody_weigh_seal_station_assembly.stl",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const MOUNT_HOLE_D: f64 = 6.8;

const DRIP_TRAY_X: f64 = 1160.0;
const DRIP_TRAY_Y: f64 = 700.0;
const DRIP_TRAY_DEPTH: f64 = 10.0;
const LEAK_SENSOR_WELLS: usize = 6;
const DRAIN_PORT_D: f64 = 18.0;

const RECEIVER_X: f64 = 410.0;
const RECEIVER_Y: f64 = 320.0;
const RECEIVER_Z: f64 = 118.0;
const RECEIVER_WALL: f64 = 18.0;
const BAG_SADDLE_X: f64 = 332.0;
const BAG_SADDLE_Y: f64 = 220.0;
const BAG_SADDLE_Z: f64 = 42.0;
const RECEIVER_CLAMPS: usize = 4;

const SCALE_PAD_X: f64 = 360.0;
const SCALE_PAD_Y: f64 = 280.0;
const SCALE_PAD_Z: f64 = 34.0;
const SCALE_LOAD_CELL_PADS: usize = 4;
const SCALE_DISPLAY_LAND_X: f64 = 138.0;
const SCALE_DISPLAY_LAND_Y: f64 = 64.0;
const SCALE_SERVICE_CLEARANCE_Z: f64 = 120.0;

const SEAL_TOOL_POCKETS: usize = 6;
const SEAL_POCKET_COLS: usize = 3;
const SEAL_POCKET_X: f64 = 96.0;
const SEAL_POCKET_Y: f64 = 64.0;
const SEAL_POCKET_Z: f64 = 30.0;
const SEAL_POCKET_PITCH_X: f64 = 118.0;
const SEAL_POCKET_PITCH_Y: f64 = 88.0;
const SEAL_POCKET_BLOCK_X: f64 = 420.0;
const SEAL_POCKET_BLOCK_Y: f64 = 222.0;
const SEAL_POCKET_BLOCK_Z: f64 = 42.0;
const SEAL_WITNESS_STRIPS: usize = 6;

const OVERBAG_LANDS: usize = 4;
const CAP_CHECK_LANDS: usize = 8;
const OVERBAG_LAND_X: f64 = 168.0;
const OVERBAG_LAND_Y: f64 = 78.0;
const CAP_CHECK_D: f64 = 34.0;
const CHECK_LAND_Z: f64 = 14.0;

const BARCODE_LANDS: usize = 8;
const RFID_LANDS: usize = 6;
const LABEL_LAND_X: f64 = 108.0;
const LABEL_LAND_Y: f64 = 34.0;
const LABEL_LAND_Z: f64 = 5.0;
const RFID_LAND_X: f64 = 82.0;
const RFID_LAND_Y: f64 = 52.0;

const DISPOSITION_LANES: usize = 3;
const LANE_X: f64 = 255.0;
const LANE_Y: f64 = 430.0;
const LANE_Z: f64 = 46.0;
const LANE_PITCH_X: f64 = 300.0;
const LANE_RAIL_Z: f64 = 74.0;

const EVIDENCE_VIALS: usize = 12;
const EVIDENCE_CARDS: usize = 6;
const VIAL_POCKET_D: f64 = 18.5;
const VIAL_POCKET_BLOCK_X: f64 = 330.0;
const VIAL_POCKET_BLOCK_Y: f64 = 150.0;
const VIAL_POCKET_BLOCK_Z: f64 = 38.0;
const CARD_SLOT_X: f64 = 86.0;
const CARD_SLOT_Y: f64 = 42.0;
const CARD_SLOT_Z: f64 = 12.0;

const CLEAN_USED_BARRIER_X: f64 = 32.0;
const CLEAN_USED_BARRIER_Y: f64 = 740.0;
const CLEAN_USED_BARRIER_Z: f64 = 112.0;
const CLEAN_STAGING_X: f64 = 300.0;
const USED_CUSTODY_X: f64 = 420.0;
const SEGREGATION_LABEL_LANDS: usize = 4;

const ROUTING_CHUTES: usize = 3;
const CHUTE_X: f64 = 220.0;
const CHUTE_Y: f64 = 240.0;
const CHUTE_Z: f64 = 76.0;
const CHUTE_PITCH_X: f64 = 300.0;
const REJECT_BIN_INTERFACE_X: f64 = 250.0;
const REJECT_BIN_INTERFACE_Y: f64 = 114.0;
const REJECT_BIN_INTERFACE_Z: f64 = 72.0;

const ROBOT_KEEP_OUT_Z: f64 = 260.0;
const FRONT_ROBOT_APPROACH: f64 = 430.0;
const REAR_SCALE_SERVICE: f64 = 260.0;
const LEFT_CLEAN_STAGING_SERVICE: f64 = 190.0;
const RIGHT_REJECT_BIN_SERVICE: f64 = 220.0;
const TOP_BAG_LIFT_CLEARANCE: f64 = 360.0;

const RECEIVER_POS: (f64, f64) = (-370.0, 108.0);
const SCALE_POS: (f64, f64) = (-30.0, 112.0);
const SEAL_POS: (f64, f64) = (360.0, 116.0);
const CHECK_POS: (f64, f64) = (-430.0, -220.0);
const TRACE_POS: (f64, f64) = (-20.0, -246.0);
const LANE_POS: (f64, f64) = (260.0, -210.0);
const EVIDENCE_POS: (f64, f64) = (455.0, -318.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_drip_tray();
    export(&base, OUTPUTS[0]);

    let receiver = sealed_bag_receiver();
    export(&receiver, OUTPUTS[1]);

    let scale = gravimetric_pad();
    export(&scale, OUTPUTS[2]);

    let seal = seal_verification_pockets();
    export(&seal, OUTPUTS[3]);

    let checks = overbag_cap_check_lands();
    export(&checks, OUTPUTS[4]);

    let traceability = barcode_rfid_chain_custody_lands();
    export(&traceability, OUTPUTS[5]);

    let lanes = released_hold_reject_lanes();
    export(&lanes, OUTPUTS[6]);

    let evidence = evidence_vial_card_pockets();
    export(&evidence, OUTPUTS[7]);

    let segregation = clean_used_segregation_barriers();
    export(&segregation, OUTPUTS[8]);

    let routing = waste_routing_chutes();
    export(&routing, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly =
        base + receiver.translate(
            RECEIVER_POS.0,
            RECEIVER_POS.1,
            BASE_Z + RECEIVER_Z / 2.0 + 8.0,
        ) + scale.translate(SCALE_POS.0, SCALE_POS.1, BASE_Z + SCALE_PAD_Z / 2.0 + 10.0)
            + seal.translate(
                SEAL_POS.0,
                SEAL_POS.1,
                BASE_Z + SEAL_POCKET_BLOCK_Z / 2.0 + 10.0,
            )
            + checks.translate(CHECK_POS.0, CHECK_POS.1, BASE_Z + CHECK_LAND_Z / 2.0 + 8.0)
            + traceability.translate(TRACE_POS.0, TRACE_POS.1, BASE_Z + LABEL_LAND_Z / 2.0 + 8.0)
            + lanes.translate(LANE_POS.0, LANE_POS.1, BASE_Z + LANE_Z / 2.0 + 10.0)
            + evidence.translate(
                EVIDENCE_POS.0,
                EVIDENCE_POS.1,
                BASE_Z + VIAL_POCKET_BLOCK_Z / 2.0 + 8.0,
            )
            + segregation
            + routing.translate(260.0, -360.0, BASE_Z + CHUTE_Z / 2.0 + 16.0)
            + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed waste-bag custody weigh/seal station:");
    println!(
        "  Deck and containment:       {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {DRIP_TRAY_X:.0}mm x {DRIP_TRAY_Y:.0}mm drip tray, {LEAK_SENSOR_WELLS} leak sensor wells, {DRAIN_PORT_D:.0}mm drain interface"
    );
    println!(
        "  Bag custody workflow:       sealed receiver, purchased-scale pad envelope with {SCALE_LOAD_CELL_PADS} load-cell lands, {SEAL_TOOL_POCKETS} seal tool pockets, {OVERBAG_LANDS} overbag lands, {CAP_CHECK_LANDS} cap check lands"
    );
    println!(
        "  Traceability:               {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {EVIDENCE_VIALS} evidence vial pockets, {EVIDENCE_CARDS} evidence card slots"
    );
    println!(
        "  Disposition routing:        {DISPOSITION_LANES} released/hold/reject lanes and {ROUTING_CHUTES} closed routing chutes with reject-bin interface"
    );
    println!(
        "  Service envelopes:          {FRONT_ROBOT_APPROACH:.0}mm front robot approach, {REAR_SCALE_SERVICE:.0}mm rear scale service, {LEFT_CLEAN_STAGING_SERVICE:.0}mm clean-side service, {RIGHT_REJECT_BIN_SERVICE:.0}mm reject-bin service, {TOP_BAG_LIFT_CLEARANCE:.0}mm top lift clearance"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(SEAL_TOOL_POCKETS, SEAL_POCKET_COLS * 2);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(ROUTING_CHUTES, DISPOSITION_LANES);
    assert!(receiver_left_edge() > -STATION_X / 2.0 + 58.0);
    assert!(seal_right_edge() < STATION_X / 2.0 - 58.0);
    assert!(lane_span_x() + LANE_X < STATION_X - 180.0);
    assert!(BARCODE_LANDS >= DISPOSITION_LANES + RECEIVER_CLAMPS);
    assert!(RFID_LANDS >= DISPOSITION_LANES + 2);
    assert!(EVIDENCE_VIALS >= EVIDENCE_CARDS * 2);
}

fn base_leak_drip_tray() -> Part {
    let deck = centered_cube(
        "waste_custody_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let tray_recess = centered_cube(
        "waste_custody_station_drip_tray_recess",
        DRIP_TRAY_X,
        DRIP_TRAY_Y,
        DRIP_TRAY_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z - DRIP_TRAY_DEPTH / 2.0);
    let drain = centered_cylinder(
        "waste_custody_station_front_drain_port",
        DRAIN_PORT_D / 2.0,
        48.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 30.0, BASE_Z - 9.0);

    deck - tray_recess - drain - mount_holes()
        + perimeter_rim()
        + drip_slope_witness_ribs()
        + leak_sensor_wells()
        + deck_zone_lands()
}

fn perimeter_rim() -> Part {
    let front = centered_cube("waste_custody_front_rim", STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("waste_custody_rear_rim", STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("waste_custody_left_rim", RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("waste_custody_right_rim", RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("waste_custody_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 52.0),
        (0.0, -STATION_Y / 2.0 + 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("waste_custody_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn drip_slope_witness_ribs() -> Part {
    let mut ribs = Part::empty("waste_custody_drip_slope_witness_ribs");
    for (i, y) in [-270.0, -135.0, 0.0, 135.0, 270.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("waste_custody_drip_flow_rib_{i}"),
                DRIP_TRAY_X - 120.0,
                9.0,
                8.0,
            )
            .translate(0.0, *y, BASE_Z + 4.0);
    }
    ribs
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("waste_custody_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = -430.0 + (i % 3) as f64 * 430.0;
        let y = -318.0 + (i / 3) as f64 * 636.0;
        let pocket = centered_cube(
            format!("waste_custody_leak_sensor_recess_{i}"),
            64.0,
            34.0,
            6.0,
        )
        .translate(x, y, BASE_Z + 3.0);
        let wire_slot = centered_cube(
            format!("waste_custody_leak_sensor_wire_slot_{i}"),
            92.0,
            7.0,
            6.0,
        )
        .translate(x, y + 28.0, BASE_Z + 3.0);
        wells = wells + pocket + wire_slot;
    }
    wells
}

fn deck_zone_lands() -> Part {
    let clean = centered_cube(
        "waste_custody_clean_inbound_stage_land",
        CLEAN_STAGING_X,
        96.0,
        7.0,
    )
    .translate(-505.0, 330.0, BASE_Z + 3.5);
    let used = centered_cube(
        "waste_custody_used_bag_custody_land",
        USED_CUSTODY_X,
        96.0,
        7.0,
    )
    .translate(-35.0, 330.0, BASE_Z + 3.5);
    let reject = centered_cube(
        "waste_custody_reject_bin_land",
        REJECT_BIN_INTERFACE_X,
        96.0,
        7.0,
    )
    .translate(470.0, 330.0, BASE_Z + 3.5);
    clean + used + reject
}

fn sealed_bag_receiver() -> Part {
    let shell = centered_cube(
        "waste_custody_sealed_bag_receiver_outer",
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
    );
    let bag_cavity = centered_cube(
        "waste_custody_sealed_bag_receiver_cavity",
        RECEIVER_X - 2.0 * RECEIVER_WALL,
        RECEIVER_Y - 2.0 * RECEIVER_WALL,
        RECEIVER_Z - 28.0,
    )
    .translate(0.0, 0.0, 14.0);
    let front_window = centered_cube(
        "waste_custody_receiver_front_access_window",
        RECEIVER_X - 74.0,
        RECEIVER_WALL + 4.0,
        RECEIVER_Z - 38.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 + RECEIVER_WALL / 2.0, 8.0);
    let saddle = centered_cube(
        "waste_custody_closed_bag_saddle_radius_proxy",
        BAG_SADDLE_X,
        BAG_SADDLE_Y,
        BAG_SADDLE_Z,
    )
    .translate(0.0, -8.0, -RECEIVER_Z / 2.0 + BAG_SADDLE_Z / 2.0 + 10.0);

    shell - bag_cavity - front_window - saddle + receiver_clamps() + receiver_datum_pins()
}

fn receiver_clamps() -> Part {
    let mut clamps = Part::empty("waste_custody_receiver_clamps");
    for (i, x) in [-150.0, 150.0].iter().enumerate() {
        for (j, y) in [-112.0, 112.0].iter().enumerate() {
            let id = i * 2 + j;
            clamps = clamps
                + centered_cube(
                    format!("waste_custody_receiver_toggle_clamp_{id}"),
                    72.0,
                    24.0,
                    28.0,
                )
                .translate(*x, *y, RECEIVER_Z / 2.0 + 14.0)
                + centered_cylinder(
                    format!("waste_custody_receiver_clamp_pivot_{id}"),
                    8.0,
                    84.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(*x, *y, RECEIVER_Z / 2.0 + 30.0);
        }
    }
    clamps
}

fn receiver_datum_pins() -> Part {
    let mut pins = Part::empty("waste_custody_receiver_datum_pins");
    for (i, x) in [-160.0, 160.0].iter().enumerate() {
        pins = pins
            + centered_cylinder(
                format!("waste_custody_receiver_datum_pin_{i}"),
                6.0,
                28.0,
                24,
            )
            .translate(*x, -RECEIVER_Y / 2.0 + 34.0, -RECEIVER_Z / 2.0 + 14.0);
    }
    pins
}

fn gravimetric_pad() -> Part {
    let pad = centered_cube(
        "waste_custody_purchased_scale_envelope_pad",
        SCALE_PAD_X,
        SCALE_PAD_Y,
        SCALE_PAD_Z,
    );
    let pan_recess = centered_cube(
        "waste_custody_scale_pan_recess",
        SCALE_PAD_X - 52.0,
        SCALE_PAD_Y - 48.0,
        9.0,
    )
    .translate(0.0, 0.0, SCALE_PAD_Z / 2.0 - 4.5);
    let display_land = centered_cube(
        "waste_custody_scale_display_reader_land",
        SCALE_DISPLAY_LAND_X,
        SCALE_DISPLAY_LAND_Y,
        10.0,
    )
    .translate(0.0, -SCALE_PAD_Y / 2.0 - 50.0, 4.0);

    pad - pan_recess + scale_load_cell_lands() + display_land + scale_cable_strain_relief()
}

fn scale_load_cell_lands() -> Part {
    let mut lands = Part::empty("waste_custody_scale_load_cell_lands");
    for (i, (x, y)) in [
        (-126.0, -86.0),
        (126.0, -86.0),
        (-126.0, 86.0),
        (126.0, 86.0),
    ]
    .iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(format!("waste_custody_load_cell_pad_{i}"), 76.0, 48.0, 12.0)
                .translate(*x, *y, SCALE_PAD_Z / 2.0 + 6.0)
            + centered_cylinder(
                format!("waste_custody_load_cell_fastener_{i}"),
                3.2,
                16.0,
                20,
            )
            .translate(*x, *y, SCALE_PAD_Z / 2.0 + 12.0);
    }
    lands
}

fn scale_cable_strain_relief() -> Part {
    let comb = centered_cube(
        "waste_custody_scale_cable_strain_relief_comb",
        SCALE_PAD_X - 90.0,
        24.0,
        24.0,
    )
    .translate(0.0, SCALE_PAD_Y / 2.0 + 32.0, -2.0);
    let mut slots = Part::empty("waste_custody_scale_cable_slots");
    for i in 0..5 {
        slots = slots
            + centered_cube(
                format!("waste_custody_scale_cable_slot_{i}"),
                12.0,
                30.0,
                14.0,
            )
            .translate(-80.0 + i as f64 * 40.0, SCALE_PAD_Y / 2.0 + 32.0, -2.0);
    }
    comb - slots
}

fn seal_verification_pockets() -> Part {
    let block = centered_cube(
        "waste_custody_seal_verification_pocket_block",
        SEAL_POCKET_BLOCK_X,
        SEAL_POCKET_BLOCK_Y,
        SEAL_POCKET_BLOCK_Z,
    );
    let mut pockets = Part::empty("waste_custody_seal_tool_pocket_cuts");
    let rows = SEAL_TOOL_POCKETS / SEAL_POCKET_COLS;
    for i in 0..SEAL_TOOL_POCKETS {
        let col = i % SEAL_POCKET_COLS;
        let row = i / SEAL_POCKET_COLS;
        let x = (col as f64 - (SEAL_POCKET_COLS as f64 - 1.0) / 2.0) * SEAL_POCKET_PITCH_X;
        let y = (row as f64 - (rows as f64 - 1.0) / 2.0) * SEAL_POCKET_PITCH_Y;
        pockets = pockets
            + centered_cube(
                format!("waste_custody_tamper_seal_tool_pocket_{i}"),
                SEAL_POCKET_X,
                SEAL_POCKET_Y,
                SEAL_POCKET_Z,
            )
            .translate(x, y, SEAL_POCKET_BLOCK_Z / 2.0 - SEAL_POCKET_Z / 2.0 + 4.0);
    }
    block - pockets + seal_witness_strip_lands() + seal_camera_bridge()
}

fn seal_witness_strip_lands() -> Part {
    let mut strips = Part::empty("waste_custody_seal_witness_strip_lands");
    for i in 0..SEAL_WITNESS_STRIPS {
        strips = strips
            + centered_cube(
                format!("waste_custody_seal_witness_strip_land_{i}"),
                88.0,
                12.0,
                6.0,
            )
            .translate(
                -165.0 + i as f64 * 66.0,
                -SEAL_POCKET_BLOCK_Y / 2.0 - 22.0,
                18.0,
            );
    }
    strips
}

fn seal_camera_bridge() -> Part {
    let left_post = centered_cube("waste_custody_seal_camera_left_post", 24.0, 24.0, 116.0)
        .translate(-SEAL_POCKET_BLOCK_X / 2.0 + 32.0, 0.0, 79.0);
    let right_post = centered_cube("waste_custody_seal_camera_right_post", 24.0, 24.0, 116.0)
        .translate(SEAL_POCKET_BLOCK_X / 2.0 - 32.0, 0.0, 79.0);
    let beam = centered_cube(
        "waste_custody_seal_camera_reader_bridge",
        SEAL_POCKET_BLOCK_X - 28.0,
        30.0,
        22.0,
    )
    .translate(0.0, 0.0, 148.0);
    left_post + right_post + beam
}

fn overbag_cap_check_lands() -> Part {
    let mut lands = Part::empty("waste_custody_overbag_cap_check_lands");
    for i in 0..OVERBAG_LANDS {
        let x = -210.0 + i as f64 * 140.0;
        lands = lands
            + centered_cube(
                format!("waste_custody_overbag_flatness_check_land_{i}"),
                OVERBAG_LAND_X,
                OVERBAG_LAND_Y,
                CHECK_LAND_Z,
            )
            .translate(x, 44.0, 0.0);
    }
    for i in 0..CAP_CHECK_LANDS {
        let x = -245.0 + (i % 4) as f64 * 70.0;
        let y = -70.0 + (i / 4) as f64 * 58.0;
        lands = lands
            + centered_cylinder(
                format!("waste_custody_cap_plug_check_land_{i}"),
                CAP_CHECK_D / 2.0,
                CHECK_LAND_Z,
                28,
            )
            .translate(x, y, 0.0);
    }
    lands
}

fn barcode_rfid_chain_custody_lands() -> Part {
    let mut lands = Part::empty("waste_custody_barcode_rfid_lands");
    for i in 0..BARCODE_LANDS {
        let x = -385.0 + (i % 4) as f64 * 154.0;
        let y = -40.0 + (i / 4) as f64 * 74.0;
        lands = lands
            + centered_cube(
                format!("waste_custody_barcode_chain_custody_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(x, y, 0.0);
    }
    for i in 0..RFID_LANDS {
        let x = -320.0 + (i % 3) as f64 * 220.0;
        let y = 134.0 + (i / 3) as f64 * 62.0;
        lands = lands
            + centered_cube(
                format!("waste_custody_rfid_chain_custody_land_{i}"),
                RFID_LAND_X,
                RFID_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(x, y, 0.0);
    }
    lands + scan_gate_posts()
}

fn scan_gate_posts() -> Part {
    let left = centered_cube("waste_custody_scan_gate_left_post", 22.0, 34.0, 126.0)
        .translate(-470.0, 24.0, 64.0);
    let right = centered_cube("waste_custody_scan_gate_right_post", 22.0, 34.0, 126.0)
        .translate(470.0, 24.0, 64.0);
    let beam = centered_cube("waste_custody_scan_gate_reader_bar", 980.0, 28.0, 22.0)
        .translate(0.0, 24.0, 138.0);
    left + right + beam
}

fn released_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("waste_custody_released_hold_reject_lanes");
    for i in 0..DISPOSITION_LANES {
        let x = (i as f64 - 1.0) * LANE_PITCH_X;
        let lane = centered_cube(
            format!("waste_custody_disposition_lane_floor_{i}"),
            LANE_X,
            LANE_Y,
            LANE_Z,
        )
        .translate(x, 0.0, 0.0);
        let left_rail = centered_cube(
            format!("waste_custody_disposition_lane_left_rail_{i}"),
            16.0,
            LANE_Y,
            LANE_RAIL_Z,
        )
        .translate(x - LANE_X / 2.0 + 8.0, 0.0, LANE_RAIL_Z / 2.0);
        let right_rail = centered_cube(
            format!("waste_custody_disposition_lane_right_rail_{i}"),
            16.0,
            LANE_Y,
            LANE_RAIL_Z,
        )
        .translate(x + LANE_X / 2.0 - 8.0, 0.0, LANE_RAIL_Z / 2.0);
        let stop = centered_cube(
            format!("waste_custody_disposition_lane_end_stop_{i}"),
            LANE_X,
            18.0,
            LANE_RAIL_Z,
        )
        .translate(x, LANE_Y / 2.0 - 9.0, LANE_RAIL_Z / 2.0);
        lanes = lanes + lane + left_rail + right_rail + stop;
    }
    lanes
}

fn evidence_vial_card_pockets() -> Part {
    let block = centered_cube(
        "waste_custody_evidence_vial_card_block",
        VIAL_POCKET_BLOCK_X,
        VIAL_POCKET_BLOCK_Y,
        VIAL_POCKET_BLOCK_Z,
    );
    let mut cuts = Part::empty("waste_custody_evidence_pocket_cuts");
    for i in 0..EVIDENCE_VIALS {
        let x = -132.0 + (i % 6) as f64 * 52.8;
        let y = -36.0 + (i / 6) as f64 * 72.0;
        cuts = cuts
            + centered_cylinder(
                format!("waste_custody_evidence_vial_pocket_{i}"),
                VIAL_POCKET_D / 2.0,
                VIAL_POCKET_BLOCK_Z + 4.0,
                24,
            )
            .translate(x, y, 4.0);
    }

    let mut card_slots = Part::empty("waste_custody_evidence_card_slots");
    for i in 0..EVIDENCE_CARDS {
        let x = -135.0 + (i % 3) as f64 * 135.0;
        let y = VIAL_POCKET_BLOCK_Y / 2.0 + 34.0 + (i / 3) as f64 * 38.0;
        card_slots = card_slots
            + centered_cube(
                format!("waste_custody_evidence_card_slot_{i}"),
                CARD_SLOT_X,
                CARD_SLOT_Y,
                CARD_SLOT_Z,
            )
            .translate(x, y, VIAL_POCKET_BLOCK_Z / 2.0);
    }
    block - cuts + card_slots
}

fn clean_used_segregation_barriers() -> Part {
    let center = centered_cube(
        "waste_custody_clean_used_center_barrier",
        CLEAN_USED_BARRIER_X,
        CLEAN_USED_BARRIER_Y,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(-230.0, 0.0, BASE_Z + CLEAN_USED_BARRIER_Z / 2.0);
    let downstream = centered_cube(
        "waste_custody_hold_reject_downstream_barrier",
        CLEAN_USED_BARRIER_X,
        CLEAN_USED_BARRIER_Y - 120.0,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(585.0, -40.0, BASE_Z + CLEAN_USED_BARRIER_Z / 2.0);
    let pass_gate = centered_cube("waste_custody_controlled_bag_pass_gate", 110.0, 34.0, 86.0)
        .translate(-230.0, -330.0, BASE_Z + 43.0);

    center + downstream + pass_gate + segregation_label_lands()
}

fn segregation_label_lands() -> Part {
    let mut labels = Part::empty("waste_custody_segregation_label_lands");
    for i in 0..SEGREGATION_LABEL_LANDS {
        labels = labels
            + centered_cube(
                format!("waste_custody_zone_label_land_{i}"),
                132.0,
                28.0,
                6.0,
            )
            .translate(-492.0 + i as f64 * 310.0, 392.0, BASE_Z + 6.0);
    }
    labels
}

fn waste_routing_chutes() -> Part {
    let mut chutes = Part::empty("waste_custody_waste_routing_chutes");
    for i in 0..ROUTING_CHUTES {
        let x = (i as f64 - 1.0) * CHUTE_PITCH_X;
        let chute = centered_cube(
            format!("waste_custody_closed_routing_chute_{i}"),
            CHUTE_X,
            CHUTE_Y,
            CHUTE_Z,
        )
        .translate(x, 0.0, 0.0);
        let mouth = centered_cube(
            format!("waste_custody_routing_chute_mouth_cut_{i}"),
            CHUTE_X - 42.0,
            44.0,
            CHUTE_Z - 18.0,
        )
        .translate(x, -CHUTE_Y / 2.0 + 20.0, 7.0);
        let throat = centered_cube(
            format!("waste_custody_routing_chute_exit_cut_{i}"),
            CHUTE_X - 64.0,
            42.0,
            CHUTE_Z - 20.0,
        )
        .translate(x, CHUTE_Y / 2.0 - 22.0, 4.0);
        chutes = chutes + (chute - mouth - throat);
    }
    let reject_interface = centered_cube(
        "waste_custody_reject_bin_closed_interface",
        REJECT_BIN_INTERFACE_X,
        REJECT_BIN_INTERFACE_Y,
        REJECT_BIN_INTERFACE_Z,
    )
    .translate(
        CHUTE_PITCH_X,
        CHUTE_Y / 2.0 + REJECT_BIN_INTERFACE_Y / 2.0 + 10.0,
        0.0,
    );
    chutes + reject_interface
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "waste_custody_front_robot_approach_keepout",
        STATION_X,
        FRONT_ROBOT_APPROACH,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );
    let rear_scale = centered_cube(
        "waste_custody_rear_scale_service_keepout",
        SCALE_PAD_X + 180.0,
        REAR_SCALE_SERVICE,
        SCALE_SERVICE_CLEARANCE_Z,
    )
    .translate(
        SCALE_POS.0,
        STATION_Y / 2.0 + REAR_SCALE_SERVICE / 2.0,
        SCALE_SERVICE_CLEARANCE_Z / 2.0,
    );
    let clean_side = centered_cube(
        "waste_custody_left_clean_staging_service_keepout",
        LEFT_CLEAN_STAGING_SERVICE,
        STATION_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CLEAN_STAGING_SERVICE / 2.0,
        0.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );
    let reject_side = centered_cube(
        "waste_custody_right_reject_bin_service_keepout",
        RIGHT_REJECT_BIN_SERVICE,
        STATION_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_REJECT_BIN_SERVICE / 2.0,
        0.0,
        ROBOT_KEEP_OUT_Z / 2.0,
    );
    let top_lift = centered_cube(
        "waste_custody_top_bag_lift_keepout",
        RECEIVER_X + 130.0,
        RECEIVER_Y + 170.0,
        TOP_BAG_LIFT_CLEARANCE,
    )
    .translate(
        RECEIVER_POS.0,
        RECEIVER_POS.1,
        BASE_Z + RECEIVER_Z + TOP_BAG_LIFT_CLEARANCE / 2.0,
    );
    front_robot + rear_scale + clean_side + reject_side + top_lift
}

fn receiver_left_edge() -> f64 {
    RECEIVER_POS.0 - RECEIVER_X / 2.0
}

fn seal_right_edge() -> f64 {
    SEAL_POS.0 + SEAL_POCKET_BLOCK_X / 2.0
}

fn lane_span_x() -> f64 {
    (DISPOSITION_LANES - 1) as f64 * LANE_PITCH_X
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct PartSpec {
    path: &'static str,
    min_size: [f64; 3],
}

#[cfg(test)]
fn output_specs() -> [PartSpec; 12] {
    [
        PartSpec {
            path: OUTPUTS[0],
            min_size: [STATION_X, STATION_Y, RIM_Z],
        },
        PartSpec {
            path: OUTPUTS[1],
            min_size: [RECEIVER_X, RECEIVER_Y, RECEIVER_Z],
        },
        PartSpec {
            path: OUTPUTS[2],
            min_size: [SCALE_PAD_X, SCALE_PAD_Y, SCALE_PAD_Z],
        },
        PartSpec {
            path: OUTPUTS[3],
            min_size: [
                SEAL_POCKET_BLOCK_X,
                SEAL_POCKET_BLOCK_Y,
                SEAL_POCKET_BLOCK_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[4],
            min_size: [OVERBAG_LAND_X, OVERBAG_LAND_Y, CHECK_LAND_Z],
        },
        PartSpec {
            path: OUTPUTS[5],
            min_size: [LABEL_LAND_X, LABEL_LAND_Y, LABEL_LAND_Z],
        },
        PartSpec {
            path: OUTPUTS[6],
            min_size: [lane_span_x() + LANE_X, LANE_Y, LANE_RAIL_Z],
        },
        PartSpec {
            path: OUTPUTS[7],
            min_size: [
                VIAL_POCKET_BLOCK_X,
                VIAL_POCKET_BLOCK_Y,
                VIAL_POCKET_BLOCK_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[8],
            min_size: [
                CLEAN_USED_BARRIER_X,
                CLEAN_USED_BARRIER_Y,
                CLEAN_USED_BARRIER_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[9],
            min_size: [
                (ROUTING_CHUTES - 1) as f64 * CHUTE_PITCH_X + CHUTE_X,
                CHUTE_Y,
                CHUTE_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[10],
            min_size: [
                STATION_X + LEFT_CLEAN_STAGING_SERVICE + RIGHT_REJECT_BIN_SERVICE,
                STATION_Y + FRONT_ROBOT_APPROACH,
                ROBOT_KEEP_OUT_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[11],
            min_size: [
                STATION_X + LEFT_CLEAN_STAGING_SERVICE + RIGHT_REJECT_BIN_SERVICE,
                STATION_Y + FRONT_ROBOT_APPROACH,
                BASE_Z + RECEIVER_Z + TOP_BAG_LIFT_CLEARANCE,
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_contract_lists_station_parts_and_assembly() {
        let specs = output_specs();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(specs.len(), OUTPUTS.len());
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/closed_waste_bag_custody_weigh_seal_station_")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
        assert!(specs.iter().all(|spec| spec.path.ends_with(".stl")));
        assert!(specs.iter().all(|spec| spec.min_size[0] > 0.0
            && spec.min_size[1] > 0.0
            && spec.min_size[2] > 0.0));
    }

    #[test]
    fn custody_capacity_covers_weigh_seal_traceability_workflow() {
        assert_eq!(SCALE_LOAD_CELL_PADS, 4);
        assert_eq!(SEAL_TOOL_POCKETS, 6);
        assert_eq!(SEAL_TOOL_POCKETS, SEAL_POCKET_COLS * 2);
        assert_eq!(OVERBAG_LANDS, 4);
        assert_eq!(CAP_CHECK_LANDS, 8);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(ROUTING_CHUTES, DISPOSITION_LANES);
        assert!(BARCODE_LANDS >= DISPOSITION_LANES + RECEIVER_CLAMPS);
        assert!(RFID_LANDS >= DISPOSITION_LANES + 2);
        assert!(EVIDENCE_VIALS >= EVIDENCE_CARDS * 2);
    }

    #[test]
    fn geometry_constants_keep_workflow_inside_station_deck() {
        assert_layout();
        assert!(DRIP_TRAY_X < STATION_X - 2.0 * RIM_W);
        assert!(DRIP_TRAY_Y < STATION_Y - 2.0 * RIM_W);
        assert!(receiver_left_edge() > -STATION_X / 2.0);
        assert!(seal_right_edge() < STATION_X / 2.0);
        assert!(SCALE_PAD_X + SCALE_DISPLAY_LAND_X < STATION_X / 2.0);
        assert!(lane_span_x() + LANE_X < STATION_X);
        assert!(CLEAN_STAGING_X + USED_CUSTODY_X < STATION_X);
    }

    #[test]
    fn output_specs_cover_requested_station_interfaces() {
        let specs = output_specs();
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("sealed_bag_receiver")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("gravimetric_pad")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("seal_verification_pockets")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("barcode_rfid_chain_custody_lands")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("released_hold_reject_lanes")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("evidence_vial_card_pockets")));
        assert!(specs
            .iter()
            .any(|spec| spec.path.contains("robot_service_keepouts")));
    }
}
