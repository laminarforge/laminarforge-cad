use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media/reagent cold-chain excursion unpack and release validation station.
//
// Design intent:
// - Receive a sealed cold-chain tote without open-bench unpacking.
// - Preserve temperature logger, cold-pack witness, barcode, certificate, and
//   sampling evidence as physical datum features.
// - Separate thaw/hold assessment from final release/hold/reject disposition.
// - Keep clean released material, used packaging, robot approach, and service
//   envelopes physically explicit for downstream automation planning.
//
// Product-concept CAD only: this model is mechanical layout for validation and
// traceability planning. It is not a cold-chain qualification protocol, release
// rule, sterility claim, or GMP procedure.

const OUTPUTS: [&str; 12] = [
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_base_drain_tray.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_sealed_tote_receiver.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_temperature_logger_nest.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_cold_pack_witness_pockets.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_thaw_hold_status_lanes.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_sampling_coupon_pockets.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_barcode_certificate_lands.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_release_hold_reject_lanes.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_evidence_bridge.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_clean_used_segregation.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_robot_service_keepouts.stl",
    "output/closed_media_reagent_cold_chain_excursion_unpack_station_assembly.stl",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const DRAIN_RECESS_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 12.0;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.4;

const TOTE_CENTER: (f64, f64) = (-420.0, 145.0);
const TOTE_RECEIVER_X: f64 = 560.0;
const TOTE_RECEIVER_Y: f64 = 350.0;
const TOTE_RECEIVER_Z: f64 = 64.0;
const TOTE_CLEAR_X: f64 = 438.0;
const TOTE_CLEAR_Y: f64 = 268.0;
const TOTE_RECESS_Z: f64 = 20.0;
const TOTE_GUIDE_RAIL_W: f64 = 16.0;
const TOTE_GUIDE_RAIL_Z: f64 = 54.0;
const TOTE_LATCH_LANDS: usize = 8;
const TOTE_DATUM_PINS: usize = 4;

const LOGGER_CENTER: (f64, f64) = (-530.0, -275.0);
const LOGGER_PANEL_X: f64 = 340.0;
const LOGGER_PANEL_Y: f64 = 174.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_SLOTS: usize = 4;
const LOGGER_SLOT_X: f64 = 58.0;
const LOGGER_SLOT_Y: f64 = 92.0;
const LOGGER_SLOT_DEPTH: f64 = 10.0;
const LOGGER_SEAL_WELLS: usize = 6;

const COLD_PACK_CENTER: (f64, f64) = (-150.0, -286.0);
const COLD_PACK_PANEL_X: f64 = 390.0;
const COLD_PACK_PANEL_Y: f64 = 188.0;
const COLD_PACK_PANEL_Z: f64 = 38.0;
const COLD_PACK_ROWS: usize = 2;
const COLD_PACK_COLS: usize = 3;
const COLD_PACK_POSITIONS: usize = COLD_PACK_ROWS * COLD_PACK_COLS;
const COLD_PACK_POCKET_X: f64 = 104.0;
const COLD_PACK_POCKET_Y: f64 = 54.0;
const COLD_PACK_POCKET_DEPTH: f64 = 12.0;
const MELT_INDICATOR_WELLS: usize = 6;

const THAW_CENTER: (f64, f64) = (250.0, -286.0);
const THAW_PANEL_X: f64 = 398.0;
const THAW_PANEL_Y: f64 = 188.0;
const THAW_PANEL_Z: f64 = 36.0;
const THAW_STATUS_LANES: usize = 3;
const THAW_SLOTS_PER_LANE: usize = 3;
const THAW_SLOT_X: f64 = 84.0;
const THAW_SLOT_Y: f64 = 42.0;
const THAW_SLOT_DEPTH: f64 = 7.0;
const THAW_LANE_PITCH_X: f64 = 116.0;

const SAMPLING_CENTER: (f64, f64) = (565.0, -64.0);
const SAMPLING_PANEL_X: f64 = 252.0;
const SAMPLING_PANEL_Y: f64 = 332.0;
const SAMPLING_PANEL_Z: f64 = 42.0;
const SAMPLING_COUPONS: usize = 10;
const SAMPLING_COUPON_X: f64 = 52.0;
const SAMPLING_COUPON_Y: f64 = 34.0;
const SAMPLING_COUPON_DEPTH: f64 = 8.0;
const SAMPLE_VIAL_WELLS: usize = 8;
const SAMPLE_VIAL_D: f64 = 20.0;

const IDENTITY_CENTER: (f64, f64) = (505.0, 322.0);
const IDENTITY_PANEL_X: f64 = 390.0;
const IDENTITY_PANEL_Y: f64 = 154.0;
const IDENTITY_PANEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 4;
const STATUS_TOKEN_LANDS: usize = 6;
const LABEL_LAND_X: f64 = 86.0;
const LABEL_LAND_Y: f64 = 30.0;
const LABEL_LAND_Z: f64 = 5.0;

const DISPOSITION_CENTER: (f64, f64) = (86.0, 176.0);
const DISPOSITION_PANEL_X: f64 = 488.0;
const DISPOSITION_PANEL_Y: f64 = 348.0;
const DISPOSITION_PANEL_Z: f64 = 34.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;
const DISPOSITION_SLOT_X: f64 = 92.0;
const DISPOSITION_SLOT_Y: f64 = 54.0;
const DISPOSITION_SLOT_DEPTH: f64 = 8.0;
const DISPOSITION_LANE_PITCH_X: f64 = 142.0;
const DISPOSITION_MIN_GAP: f64 = 34.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 22.0);
const BRIDGE_SPAN_X: f64 = 1240.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 44.0;
const BRIDGE_UNDERSIDE_Z: f64 = 250.0;
const BRIDGE_BEAM_Z: f64 = 36.0;
const CAMERA_PODS: usize = 4;
const LED_BARS: usize = 6;

const SEGREGATION_CENTER_Y: f64 = 22.0;
const SEGREGATION_WALL_X: f64 = 1340.0;
const SEGREGATION_WALL_Y: f64 = 24.0;
const SEGREGATION_WALL_Z: f64 = 132.0;
const SEGREGATION_PASS_SLOT_X: f64 = 290.0;
const SEGREGATION_PASS_SLOT_Z: f64 = 72.0;
const USED_PACKAGING_BIN_X: f64 = 242.0;
const USED_PACKAGING_BIN_Y: f64 = 158.0;
const USED_PACKAGING_BIN_Z: f64 = 88.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 420.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 300.0;
const LEFT_TOTE_SERVICE_KEEP_OUT_X: f64 = 280.0;
const RIGHT_CERT_SERVICE_KEEP_OUT_X: f64 = 240.0;
const TOP_BRIDGE_SERVICE_CLEARANCE_Z: f64 = 350.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_drain_tray();
    export(OUTPUTS[0], &base);

    let receiver = sealed_tote_receiver().translate(TOTE_CENTER.0, TOTE_CENTER.1, BASE_Z);
    export(OUTPUTS[1], &receiver);

    let logger = temperature_logger_nest().translate(LOGGER_CENTER.0, LOGGER_CENTER.1, BASE_Z);
    export(OUTPUTS[2], &logger);

    let cold_pack =
        cold_pack_witness_pockets().translate(COLD_PACK_CENTER.0, COLD_PACK_CENTER.1, BASE_Z);
    export(OUTPUTS[3], &cold_pack);

    let thaw = thaw_hold_status_lanes().translate(THAW_CENTER.0, THAW_CENTER.1, BASE_Z);
    export(OUTPUTS[4], &thaw);

    let sampling =
        sampling_coupon_pockets().translate(SAMPLING_CENTER.0, SAMPLING_CENTER.1, BASE_Z);
    export(OUTPUTS[5], &sampling);

    let identity =
        barcode_certificate_lands().translate(IDENTITY_CENTER.0, IDENTITY_CENTER.1, BASE_Z);
    export(OUTPUTS[6], &identity);

    let disposition =
        release_hold_reject_lanes().translate(DISPOSITION_CENTER.0, DISPOSITION_CENTER.1, BASE_Z);
    export(OUTPUTS[7], &disposition);

    let bridge = evidence_bridge().translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z);
    export(OUTPUTS[8], &bridge);

    let segregation = clean_used_segregation().translate(0.0, 0.0, BASE_Z);
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts().translate(0.0, 0.0, BASE_Z);
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + receiver
        + logger
        + cold_pack
        + thaw
        + sampling
        + identity
        + disposition
        + bridge
        + segregation
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media/reagent cold-chain excursion unpack station:");
    println!(
        "  Deck and containment:        {STATION_X:.0}mm x {STATION_Y:.0}mm drain tray, {DRAIN_PORT_D:.0}mm drain, {RIM_Z:.0}mm raised rim"
    );
    println!(
        "  Sealed tote receiver:        {TOTE_RECEIVER_X:.0}mm x {TOTE_RECEIVER_Y:.0}mm receiver, {TOTE_LATCH_LANDS} latch lands, {TOTE_DATUM_PINS} datum pins"
    );
    println!(
        "  Logger/cold-pack evidence:   {LOGGER_SLOTS} logger pockets, {LOGGER_SEAL_WELLS} seal wells, {COLD_PACK_POSITIONS} cold-pack witness pockets, {MELT_INDICATOR_WELLS} melt indicator wells"
    );
    println!(
        "  Thaw/hold assessment:        {THAW_STATUS_LANES} lanes x {THAW_SLOTS_PER_LANE} slots for frozen, thawing, and hold staging"
    );
    println!(
        "  Sampling identity capture:   {SAMPLING_COUPONS} coupon pockets, {SAMPLE_VIAL_WELLS} vial wells, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Disposition segregation:     release/hold/reject with {DISPOSITION_SLOTS_PER_LANE} closed-material slots per lane and {DISPOSITION_MIN_GAP:.0}mm minimum divider gap"
    );
    println!(
        "  Evidence bridge:             {CAMERA_PODS} camera pods, {LED_BARS} light bars, {BRIDGE_UNDERSIDE_Z:.0}mm underside clearance"
    );
    println!(
        "  Clean/used controls:         {SEGREGATION_WALL_Z:.0}mm divider wall, used-packaging bin, one-way pass slot, clean release side lands"
    );
    println!(
        "  Robot/service keepouts:      front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, left tote service {LEFT_TOTE_SERVICE_KEEP_OUT_X:.0}mm, top bridge {TOP_BRIDGE_SERVICE_CLEARANCE_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12, "unexpected output part count");
    assert_eq!(
        COLD_PACK_POSITIONS, 6,
        "cold-pack witness layout changed unexpectedly"
    );
    for (name, center, width, depth) in component_specs() {
        assert!(
            fits_on_station(center, width, depth, 22.0),
            "{name} exceeds station footprint"
        );
    }
    assert!(
        disposition_lane_gap() >= DISPOSITION_MIN_GAP,
        "release/hold/reject lane gap below segregation target"
    );
    assert!(
        BRIDGE_SPAN_X > TOTE_RECEIVER_X + DISPOSITION_PANEL_X + 80.0,
        "evidence bridge does not span primary evidence zones"
    );
    assert!(
        SEGREGATION_PASS_SLOT_X < TOTE_CLEAR_X,
        "pass slot should be smaller than tote clear opening"
    );
}

fn component_specs() -> [(&'static str, (f64, f64), f64, f64); 7] {
    [
        (
            "sealed tote receiver",
            TOTE_CENTER,
            TOTE_RECEIVER_X,
            TOTE_RECEIVER_Y,
        ),
        (
            "temperature logger nest",
            LOGGER_CENTER,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        ),
        (
            "cold-pack witness pockets",
            COLD_PACK_CENTER,
            COLD_PACK_PANEL_X,
            COLD_PACK_PANEL_Y,
        ),
        ("thaw/hold lanes", THAW_CENTER, THAW_PANEL_X, THAW_PANEL_Y),
        (
            "sampling coupon pockets",
            SAMPLING_CENTER,
            SAMPLING_PANEL_X,
            SAMPLING_PANEL_Y,
        ),
        (
            "barcode/certificate lands",
            IDENTITY_CENTER,
            IDENTITY_PANEL_X,
            IDENTITY_PANEL_Y,
        ),
        (
            "release/hold/reject lanes",
            DISPOSITION_CENTER,
            DISPOSITION_PANEL_X,
            DISPOSITION_PANEL_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn base_drain_tray() -> Part {
    let deck = centered_cube(
        "cold_chain_unpack_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_pan = centered_cube(
        "cold_chain_unpack_station_shallow_drain_recess",
        STATION_X - 128.0,
        STATION_Y - 132.0,
        DRAIN_RECESS_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, BASE_Z - DRAIN_RECESS_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        "cold_chain_unpack_station_front_drain_port",
        DRAIN_PORT_D / 2.0,
        58.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 94.0,
        -STATION_Y / 2.0 + 18.0,
        BASE_Z - 6.0,
    );

    deck - recessed_pan - drain + perimeter_rim() + mount_bosses() + floor_zone_lands()
}

fn perimeter_rim() -> Part {
    let front = centered_cube("cold_chain_unpack_front_rim", STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("cold_chain_unpack_rear_rim", STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("cold_chain_unpack_left_rim", RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("cold_chain_unpack_right_rim", RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty("cold_chain_unpack_mount_bosses");
    for (i, (x, y)) in [
        (-680.0, -400.0),
        (-680.0, 400.0),
        (680.0, -400.0),
        (680.0, 400.0),
        (-120.0, -400.0),
        (120.0, 400.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("cold_chain_unpack_mount_boss_{i}"),
            MOUNT_BOSS_D / 2.0,
            9.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        let hole = centered_cylinder(
            format!("cold_chain_unpack_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            13.0,
            28,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn floor_zone_lands() -> Part {
    let incoming = centered_cube(
        "cold_chain_unpack_incoming_suspect_zone_land",
        TOTE_RECEIVER_X + 54.0,
        TOTE_RECEIVER_Y + 42.0,
        3.0,
    )
    .translate(TOTE_CENTER.0, TOTE_CENTER.1, BASE_Z + 1.5);
    let evidence = centered_cube(
        "cold_chain_unpack_evidence_chain_zone_land",
        910.0,
        244.0,
        3.0,
    )
    .translate(-126.0, -286.0, BASE_Z + 1.5);
    let release = centered_cube(
        "cold_chain_unpack_release_decision_zone_land",
        684.0,
        448.0,
        3.0,
    )
    .translate(238.0, 190.0, BASE_Z + 1.5);

    incoming + evidence + release
}

fn sealed_tote_receiver() -> Part {
    let body = centered_cube(
        "cold_chain_sealed_tote_receiver_base",
        TOTE_RECEIVER_X,
        TOTE_RECEIVER_Y,
        TOTE_RECEIVER_Z,
    )
    .translate(0.0, 0.0, TOTE_RECEIVER_Z / 2.0);
    let tote_recess = centered_cube(
        "cold_chain_sealed_tote_recess_cut",
        TOTE_CLEAR_X,
        TOTE_CLEAR_Y,
        TOTE_RECESS_Z + 1.0,
    )
    .translate(0.0, 0.0, TOTE_RECEIVER_Z - TOTE_RECESS_Z / 2.0 + 0.5);
    let latch_sensor_trough = centered_cube(
        "cold_chain_sealed_tote_front_latch_sensor_trough",
        TOTE_CLEAR_X - 52.0,
        18.0,
        10.0,
    )
    .translate(0.0, -TOTE_CLEAR_Y / 2.0 - 18.0, TOTE_RECEIVER_Z - 5.0);

    body - tote_recess - latch_sensor_trough
        + tote_guide_rails()
        + tote_latch_lands()
        + tote_datum_pins()
        + tote_seal_chain_tabs()
}

fn tote_guide_rails() -> Part {
    let left = centered_cube(
        "cold_chain_tote_left_datum_rail",
        TOTE_GUIDE_RAIL_W,
        TOTE_CLEAR_Y + 72.0,
        TOTE_GUIDE_RAIL_Z,
    )
    .translate(
        -TOTE_CLEAR_X / 2.0 - TOTE_GUIDE_RAIL_W / 2.0 - 12.0,
        0.0,
        TOTE_RECEIVER_Z + TOTE_GUIDE_RAIL_Z / 2.0,
    );
    let right = centered_cube(
        "cold_chain_tote_right_datum_rail",
        TOTE_GUIDE_RAIL_W,
        TOTE_CLEAR_Y + 72.0,
        TOTE_GUIDE_RAIL_Z,
    )
    .translate(
        TOTE_CLEAR_X / 2.0 + TOTE_GUIDE_RAIL_W / 2.0 + 12.0,
        0.0,
        TOTE_RECEIVER_Z + TOTE_GUIDE_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "cold_chain_tote_rear_hard_stop",
        TOTE_CLEAR_X + 72.0,
        22.0,
        TOTE_GUIDE_RAIL_Z,
    )
    .translate(
        0.0,
        TOTE_CLEAR_Y / 2.0 + 26.0,
        TOTE_RECEIVER_Z + TOTE_GUIDE_RAIL_Z / 2.0,
    );
    let front_gasket = centered_cube(
        "cold_chain_tote_front_closed_transfer_gasket_land",
        TOTE_CLEAR_X + 96.0,
        18.0,
        18.0,
    )
    .translate(0.0, -TOTE_CLEAR_Y / 2.0 - 32.0, TOTE_RECEIVER_Z + 9.0);

    left + right + rear + front_gasket
}

fn tote_latch_lands() -> Part {
    let mut lands = Part::empty("cold_chain_tote_latch_and_tamper_lands");
    for i in 0..TOTE_LATCH_LANDS {
        let side = if i < TOTE_LATCH_LANDS / 2 { -1.0 } else { 1.0 };
        let side_index = i % (TOTE_LATCH_LANDS / 2);
        let x = lane_x(side_index, TOTE_LATCH_LANDS / 2, 92.0);
        let y = side * (TOTE_RECEIVER_Y / 2.0 - 32.0);
        lands =
            lands
                + centered_cube(format!("cold_chain_tote_latch_land_{i}"), 58.0, 24.0, 9.0)
                    .translate(x, y, TOTE_RECEIVER_Z + 4.5);
    }
    lands
}

fn tote_datum_pins() -> Part {
    let mut pins = Part::empty("cold_chain_tote_datum_pin_sockets");
    for (i, (x, y)) in [
        (-TOTE_CLEAR_X / 2.0, -TOTE_CLEAR_Y / 2.0),
        (TOTE_CLEAR_X / 2.0, -TOTE_CLEAR_Y / 2.0),
        (-TOTE_CLEAR_X / 2.0, TOTE_CLEAR_Y / 2.0),
        (TOTE_CLEAR_X / 2.0, TOTE_CLEAR_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let socket = centered_cylinder(format!("cold_chain_tote_datum_socket_{i}"), 13.0, 16.0, 36)
            .translate(*x, *y, TOTE_RECEIVER_Z + 8.0);
        let pin_cut = centered_cylinder(
            format!("cold_chain_tote_datum_pin_clearance_{i}"),
            4.0,
            20.0,
            28,
        )
        .translate(*x, *y, TOTE_RECEIVER_Z + 8.0);
        pins = pins + (socket - pin_cut);
    }
    pins
}

fn tote_seal_chain_tabs() -> Part {
    let mut tabs = Part::empty("cold_chain_tote_broken_seal_chain_tabs");
    for i in 0..6 {
        tabs = tabs
            + centered_cube(
                format!("cold_chain_tote_seal_chain_tab_{i}"),
                34.0,
                12.0,
                16.0,
            )
            .translate(
                -150.0 + i as f64 * 60.0,
                -TOTE_RECEIVER_Y / 2.0 - 22.0,
                TOTE_RECEIVER_Z + 8.0,
            );
    }
    tabs
}

fn temperature_logger_nest() -> Part {
    let panel = centered_cube(
        "cold_chain_temperature_logger_nest_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);
    let mut cuts = Part::empty("cold_chain_temperature_logger_recess_cuts");
    for i in 0..LOGGER_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("cold_chain_temperature_logger_recess_{i}"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH,
            )
            .translate(
                lane_x(i, LOGGER_SLOTS, 72.0),
                -10.0,
                LOGGER_PANEL_Z - LOGGER_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    let panel = panel - cuts;

    panel + logger_cable_comb() + logger_seal_wells() + logger_witness_card_lands()
}

fn logger_cable_comb() -> Part {
    let comb = centered_cube(
        "cold_chain_logger_cable_comb_body",
        LOGGER_PANEL_X - 54.0,
        24.0,
        20.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 + 16.0, LOGGER_PANEL_Z - 8.0);
    let mut cuts = Part::empty("cold_chain_logger_cable_comb_channel_cuts");
    for i in 0..LOGGER_SLOTS {
        cuts = cuts
            + centered_cylinder(
                format!("cold_chain_logger_cable_channel_{i}"),
                3.6,
                72.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                lane_x(i, LOGGER_SLOTS, 72.0),
                LOGGER_PANEL_Y / 2.0 + 16.0,
                LOGGER_PANEL_Z - 8.0,
            );
    }
    comb - cuts
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty("cold_chain_logger_tamper_seal_wells");
    for i in 0..LOGGER_SEAL_WELLS {
        wells = wells
            + centered_cylinder(
                format!("cold_chain_logger_tamper_seal_well_{i}"),
                11.0,
                9.0,
                32,
            )
            .translate(
                lane_x(i, LOGGER_SEAL_WELLS, 48.0),
                -LOGGER_PANEL_Y / 2.0 + 24.0,
                LOGGER_PANEL_Z + 4.5,
            );
    }
    wells
}

fn logger_witness_card_lands() -> Part {
    let card = centered_cube(
        "cold_chain_logger_download_certificate_card_land",
        LOGGER_PANEL_X - 82.0,
        28.0,
        5.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 - 22.0, LOGGER_PANEL_Z + 2.5);
    let orientation_key = centered_cube(
        "cold_chain_logger_download_orientation_key",
        42.0,
        8.0,
        12.0,
    )
    .translate(
        -LOGGER_PANEL_X / 2.0 + 44.0,
        LOGGER_PANEL_Y / 2.0 - 22.0,
        LOGGER_PANEL_Z + 6.0,
    );
    card + orientation_key
}

fn cold_pack_witness_pockets() -> Part {
    let panel = centered_cube(
        "cold_chain_cold_pack_witness_panel",
        COLD_PACK_PANEL_X,
        COLD_PACK_PANEL_Y,
        COLD_PACK_PANEL_Z,
    )
    .translate(0.0, 0.0, COLD_PACK_PANEL_Z / 2.0);
    let mut cuts = Part::empty("cold_chain_cold_pack_witness_pocket_cuts");
    for row in 0..COLD_PACK_ROWS {
        for col in 0..COLD_PACK_COLS {
            cuts = cuts
                + centered_cube(
                    format!("cold_chain_cold_pack_witness_pocket_r{row}_c{col}"),
                    COLD_PACK_POCKET_X,
                    COLD_PACK_POCKET_Y,
                    COLD_PACK_POCKET_DEPTH,
                )
                .translate(
                    grid_x(col, COLD_PACK_COLS, 124.0),
                    grid_y(row, COLD_PACK_ROWS, 78.0),
                    COLD_PACK_PANEL_Z - COLD_PACK_POCKET_DEPTH / 2.0 + 0.5,
                );
        }
    }

    (panel - cuts) + melt_indicator_wells() + cold_pack_orientation_ribs()
}

fn melt_indicator_wells() -> Part {
    let mut wells = Part::empty("cold_chain_melt_indicator_witness_wells");
    for i in 0..MELT_INDICATOR_WELLS {
        wells = wells
            + centered_cylinder(format!("cold_chain_melt_indicator_well_{i}"), 9.0, 7.0, 32)
                .translate(
                    lane_x(i, MELT_INDICATOR_WELLS, 54.0),
                    -COLD_PACK_PANEL_Y / 2.0 + 20.0,
                    COLD_PACK_PANEL_Z + 3.5,
                );
    }
    wells
}

fn cold_pack_orientation_ribs() -> Part {
    let mut ribs = Part::empty("cold_chain_pack_orientation_witness_ribs");
    for i in 0..COLD_PACK_POSITIONS {
        ribs = ribs
            + centered_cube(
                format!("cold_chain_pack_orientation_rib_{i}"),
                42.0,
                5.0,
                5.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 18.0 } else { -18.0 })
            .translate(
                -150.0 + i as f64 * 60.0,
                COLD_PACK_PANEL_Y / 2.0 - 18.0,
                COLD_PACK_PANEL_Z + 2.5,
            );
    }
    ribs
}

fn thaw_hold_status_lanes() -> Part {
    let panel = centered_cube(
        "cold_chain_thaw_hold_status_lane_panel",
        THAW_PANEL_X,
        THAW_PANEL_Y,
        THAW_PANEL_Z,
    )
    .translate(0.0, 0.0, THAW_PANEL_Z / 2.0);
    let mut cuts = Part::empty("cold_chain_thaw_hold_status_slot_cuts");
    for lane in 0..THAW_STATUS_LANES {
        for slot in 0..THAW_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("cold_chain_thaw_lane_{lane}_status_slot_{slot}"),
                    THAW_SLOT_X,
                    THAW_SLOT_Y,
                    THAW_SLOT_DEPTH,
                )
                .translate(
                    lane_x(lane, THAW_STATUS_LANES, THAW_LANE_PITCH_X),
                    lane_x(slot, THAW_SLOTS_PER_LANE, 54.0),
                    THAW_PANEL_Z - THAW_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    (panel - cuts) + thaw_lane_dividers() + thaw_time_token_rails()
}

fn thaw_lane_dividers() -> Part {
    let left = centered_cube(
        "cold_chain_thaw_hold_left_lane_divider",
        12.0,
        THAW_PANEL_Y - 34.0,
        30.0,
    )
    .translate(-THAW_LANE_PITCH_X / 2.0, 0.0, THAW_PANEL_Z + 15.0);
    let right = centered_cube(
        "cold_chain_thaw_hold_right_lane_divider",
        12.0,
        THAW_PANEL_Y - 34.0,
        30.0,
    )
    .translate(THAW_LANE_PITCH_X / 2.0, 0.0, THAW_PANEL_Z + 15.0);
    let hold_gate = centered_cube(
        "cold_chain_thaw_hold_quarantine_gate_land",
        THAW_PANEL_X - 44.0,
        12.0,
        24.0,
    )
    .translate(0.0, THAW_PANEL_Y / 2.0 - 26.0, THAW_PANEL_Z + 12.0);

    left + right + hold_gate
}

fn thaw_time_token_rails() -> Part {
    let mut rails = Part::empty("cold_chain_thaw_hold_elapsed_time_token_rails");
    for lane in 0..THAW_STATUS_LANES {
        rails = rails
            + centered_cube(
                format!("cold_chain_thaw_hold_time_token_rail_{lane}"),
                78.0,
                10.0,
                10.0,
            )
            .translate(
                lane_x(lane, THAW_STATUS_LANES, THAW_LANE_PITCH_X),
                -THAW_PANEL_Y / 2.0 + 18.0,
                THAW_PANEL_Z + 5.0,
            );
    }
    rails
}

fn sampling_coupon_pockets() -> Part {
    let panel = centered_cube(
        "cold_chain_sampling_coupon_panel",
        SAMPLING_PANEL_X,
        SAMPLING_PANEL_Y,
        SAMPLING_PANEL_Z,
    )
    .translate(0.0, 0.0, SAMPLING_PANEL_Z / 2.0);
    let mut cuts = Part::empty("cold_chain_sampling_coupon_and_vial_cuts");
    for i in 0..SAMPLING_COUPONS {
        cuts = cuts
            + centered_cube(
                format!("cold_chain_sampling_coupon_pocket_{i}"),
                SAMPLING_COUPON_X,
                SAMPLING_COUPON_Y,
                SAMPLING_COUPON_DEPTH,
            )
            .translate(
                grid_x(i % 2, 2, 72.0),
                -126.0 + (i / 2) as f64 * 54.0,
                SAMPLING_PANEL_Z - SAMPLING_COUPON_DEPTH / 2.0 + 0.5,
            );
    }
    for i in 0..SAMPLE_VIAL_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("cold_chain_sampling_vial_well_{i}"),
                SAMPLE_VIAL_D / 2.0,
                24.0,
                36,
            )
            .translate(
                grid_x(i % 4, 4, 42.0),
                SAMPLING_PANEL_Y / 2.0 - 48.0 - (i / 4) as f64 * 42.0,
                SAMPLING_PANEL_Z - 10.0,
            );
    }

    (panel - cuts) + coupon_retainer_lips() + sterile_sample_cap_parking()
}

fn coupon_retainer_lips() -> Part {
    let mut lips = Part::empty("cold_chain_coupon_retainer_lips");
    for i in 0..SAMPLING_COUPONS {
        let x = grid_x(i % 2, 2, 72.0);
        let y = -126.0 + (i / 2) as f64 * 54.0;
        lips = lips
            + centered_cube(
                format!("cold_chain_coupon_front_retainer_lip_{i}"),
                SAMPLING_COUPON_X + 12.0,
                5.0,
                9.0,
            )
            .translate(x, y - SAMPLING_COUPON_Y / 2.0 - 2.5, SAMPLING_PANEL_Z + 4.5);
    }
    lips
}

fn sterile_sample_cap_parking() -> Part {
    let mut caps = Part::empty("cold_chain_sterile_sample_cap_parking_posts");
    for i in 0..6 {
        caps = caps
            + centered_cylinder(
                format!("cold_chain_sterile_sample_cap_post_{i}"),
                8.0,
                14.0,
                32,
            )
            .translate(
                lane_x(i, 6, 32.0),
                SAMPLING_PANEL_Y / 2.0 - 18.0,
                SAMPLING_PANEL_Z + 7.0,
            );
    }
    caps
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "cold_chain_barcode_certificate_panel",
        IDENTITY_PANEL_X,
        IDENTITY_PANEL_Y,
        IDENTITY_PANEL_Z,
    )
    .translate(0.0, 0.0, IDENTITY_PANEL_Z / 2.0);

    panel + barcode_lands() + certificate_lands() + status_token_lands() + scanner_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("cold_chain_barcode_scan_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("cold_chain_barcode_scan_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(
                grid_x(i % 5, 5, 70.0),
                -IDENTITY_PANEL_Y / 2.0 + 30.0 + (i / 5) as f64 * 40.0,
                IDENTITY_PANEL_Z + LABEL_LAND_Z / 2.0,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("cold_chain_certificate_document_lands");
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(format!("cold_chain_certificate_land_{i}"), 112.0, 34.0, 5.0)
                .translate(
                    lane_x(i, CERTIFICATE_LANDS, 86.0),
                    IDENTITY_PANEL_Y / 2.0 - 28.0,
                    IDENTITY_PANEL_Z + 2.5,
                );
    }
    lands
}

fn status_token_lands() -> Part {
    let mut lands = Part::empty("cold_chain_release_status_token_lands");
    for i in 0..STATUS_TOKEN_LANDS {
        lands = lands
            + centered_cylinder(format!("cold_chain_status_token_land_{i}"), 12.0, 6.0, 32)
                .translate(
                    lane_x(i, STATUS_TOKEN_LANDS, 42.0),
                    6.0,
                    IDENTITY_PANEL_Z + 3.0,
                );
    }
    lands
}

fn scanner_fiducials() -> Part {
    let left = centered_cylinder("cold_chain_identity_left_scanner_fiducial", 9.0, 5.0, 32)
        .translate(-IDENTITY_PANEL_X / 2.0 + 24.0, 0.0, IDENTITY_PANEL_Z + 2.5);
    let right = centered_cylinder("cold_chain_identity_right_scanner_fiducial", 9.0, 5.0, 32)
        .translate(IDENTITY_PANEL_X / 2.0 - 24.0, 0.0, IDENTITY_PANEL_Z + 2.5);
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "cold_chain_release_hold_reject_panel",
        DISPOSITION_PANEL_X,
        DISPOSITION_PANEL_Y,
        DISPOSITION_PANEL_Z,
    )
    .translate(0.0, 0.0, DISPOSITION_PANEL_Z / 2.0);
    let mut cuts = Part::empty("cold_chain_disposition_slot_cuts");
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("cold_chain_disposition_lane_{lane}_slot_{slot}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    DISPOSITION_SLOT_DEPTH,
                )
                .translate(
                    lane_x(lane, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X),
                    lane_x(slot, DISPOSITION_SLOTS_PER_LANE, 74.0),
                    DISPOSITION_PANEL_Z - DISPOSITION_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    (panel - cuts) + disposition_lane_dividers() + disposition_stop_blocks()
}

fn disposition_lane_dividers() -> Part {
    let divider_a = centered_cube(
        "cold_chain_release_hold_first_full_height_divider",
        14.0,
        DISPOSITION_PANEL_Y - 38.0,
        42.0,
    )
    .translate(
        -DISPOSITION_LANE_PITCH_X / 2.0,
        0.0,
        DISPOSITION_PANEL_Z + 21.0,
    );
    let divider_b = centered_cube(
        "cold_chain_release_hold_second_full_height_divider",
        14.0,
        DISPOSITION_PANEL_Y - 38.0,
        42.0,
    )
    .translate(
        DISPOSITION_LANE_PITCH_X / 2.0,
        0.0,
        DISPOSITION_PANEL_Z + 21.0,
    );
    let rear_status_rail = centered_cube(
        "cold_chain_release_hold_reject_rear_status_rail",
        DISPOSITION_PANEL_X - 50.0,
        14.0,
        24.0,
    )
    .translate(
        0.0,
        DISPOSITION_PANEL_Y / 2.0 - 24.0,
        DISPOSITION_PANEL_Z + 12.0,
    );

    divider_a + divider_b + rear_status_rail
}

fn disposition_stop_blocks() -> Part {
    let mut blocks = Part::empty("cold_chain_disposition_lane_stop_blocks");
    for lane in 0..DISPOSITION_LANES {
        blocks = blocks
            + centered_cube(
                format!("cold_chain_disposition_lane_{lane}_front_stop"),
                96.0,
                12.0,
                18.0,
            )
            .translate(
                lane_x(lane, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X),
                -DISPOSITION_PANEL_Y / 2.0 + 22.0,
                DISPOSITION_PANEL_Z + 9.0,
            );
    }
    blocks
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "cold_chain_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        "cold_chain_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        "cold_chain_evidence_bridge_overhead_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        54.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    left_post + right_post + beam + evidence_camera_pods() + evidence_light_bars()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("cold_chain_evidence_bridge_camera_pods");
    for i in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("cold_chain_evidence_camera_pod_{i}"),
                58.0,
                42.0,
                26.0,
            )
            .translate(
                lane_x(i, CAMERA_PODS, 260.0),
                -30.0,
                BRIDGE_UNDERSIDE_Z - 13.0,
            ) + centered_cylinder(
                format!("cold_chain_evidence_camera_lens_envelope_{i}"),
                12.0,
                12.0,
                32,
            )
            .translate(
                lane_x(i, CAMERA_PODS, 260.0),
                -52.0,
                BRIDGE_UNDERSIDE_Z - 26.0,
            );
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("cold_chain_evidence_bridge_light_bars");
    for i in 0..LED_BARS {
        bars = bars
            + centered_cube(format!("cold_chain_evidence_led_bar_{i}"), 118.0, 10.0, 8.0)
                .translate(lane_x(i, LED_BARS, 190.0), 30.0, BRIDGE_UNDERSIDE_Z - 8.0);
    }
    bars
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "cold_chain_clean_used_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(0.0, SEGREGATION_CENTER_Y, SEGREGATION_WALL_Z / 2.0);
    let pass_slot = centered_cube(
        "cold_chain_one_way_clean_pass_slot_cut",
        SEGREGATION_PASS_SLOT_X,
        SEGREGATION_WALL_Y + 4.0,
        SEGREGATION_PASS_SLOT_Z,
    )
    .translate(0.0, SEGREGATION_CENTER_Y, 54.0);
    let wall = wall - pass_slot;

    wall + clean_side_release_shelf() + used_packaging_bin() + segregation_arrow_witnesses()
}

fn clean_side_release_shelf() -> Part {
    let shelf = centered_cube("cold_chain_clean_release_side_shelf", 372.0, 84.0, 28.0).translate(
        150.0,
        SEGREGATION_CENTER_Y + 78.0,
        14.0,
    );
    let lip = centered_cube(
        "cold_chain_clean_release_shelf_front_lip",
        390.0,
        10.0,
        22.0,
    )
    .translate(150.0, SEGREGATION_CENTER_Y + 36.0, 39.0);
    shelf + lip
}

fn used_packaging_bin() -> Part {
    let bin = centered_cube(
        "cold_chain_used_packaging_bin_outer",
        USED_PACKAGING_BIN_X,
        USED_PACKAGING_BIN_Y,
        USED_PACKAGING_BIN_Z,
    )
    .translate(
        -560.0,
        SEGREGATION_CENTER_Y - 108.0,
        USED_PACKAGING_BIN_Z / 2.0,
    );
    let cavity = centered_cube(
        "cold_chain_used_packaging_bin_cavity_cut",
        USED_PACKAGING_BIN_X - 34.0,
        USED_PACKAGING_BIN_Y - 34.0,
        USED_PACKAGING_BIN_Z - 18.0,
    )
    .translate(
        -560.0,
        SEGREGATION_CENTER_Y - 108.0,
        USED_PACKAGING_BIN_Z / 2.0 + 13.0,
    );
    let lid_stop = centered_cube(
        "cold_chain_used_packaging_bin_lid_stop",
        USED_PACKAGING_BIN_X + 26.0,
        12.0,
        22.0,
    )
    .translate(
        -560.0,
        SEGREGATION_CENTER_Y - 20.0,
        USED_PACKAGING_BIN_Z + 11.0,
    );

    (bin - cavity) + lid_stop
}

fn segregation_arrow_witnesses() -> Part {
    let mut arrows = Part::empty("cold_chain_clean_used_direction_witnesses");
    for i in 0..7 {
        arrows = arrows
            + centered_cube(
                format!("cold_chain_clean_used_direction_witness_{i}"),
                46.0,
                5.0,
                5.0,
            )
            .rotate(0.0, 0.0, 28.0)
            .translate(
                -330.0 + i as f64 * 110.0,
                SEGREGATION_CENTER_Y + 18.0,
                SEGREGATION_WALL_Z + 2.5,
            );
    }
    arrows
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "cold_chain_front_robot_approach_keepout_gauge",
        STATION_X - 150.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear_service = centered_cube(
        "cold_chain_rear_service_pull_keepout_gauge",
        STATION_X - 180.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left_tote_service = centered_cube(
        "cold_chain_left_tote_lid_swing_keepout_gauge",
        LEFT_TOTE_SERVICE_KEEP_OUT_X,
        STATION_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_TOTE_SERVICE_KEEP_OUT_X / 2.0,
        28.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right_certificate_service = centered_cube(
        "cold_chain_right_certificate_service_keepout_gauge",
        RIGHT_CERT_SERVICE_KEEP_OUT_X,
        STATION_Y - 220.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_CERT_SERVICE_KEEP_OUT_X / 2.0,
        64.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let overhead = centered_cube(
        "cold_chain_evidence_bridge_top_service_keepout_gauge",
        BRIDGE_SPAN_X,
        108.0,
        18.0,
    )
    .translate(0.0, BRIDGE_CENTER.1, TOP_BRIDGE_SERVICE_CLEARANCE_Z);

    front_robot + rear_service + left_tote_service + right_certificate_service + overhead
}

fn disposition_lane_gap() -> f64 {
    DISPOSITION_LANE_PITCH_X - DISPOSITION_SLOT_X
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count - 1) as f64 * pitch) / 2.0 + index as f64 * pitch
}

fn grid_x(index: usize, count: usize, pitch: f64) -> f64 {
    lane_x(index, count, pitch)
}

fn grid_y(index: usize, count: usize, pitch: f64) -> f64 {
    lane_x(index, count, pitch)
}
