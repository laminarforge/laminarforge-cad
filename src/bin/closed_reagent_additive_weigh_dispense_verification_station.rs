use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent additive weigh/dispense verification station for media formulation.
//
// Research assumptions used for the interface CAD:
// - GMP weigh/dispense workflows rely on calibrated balances, scan-enforced
//   identity, tolerance checks, material status, and batch-record traceability.
// - Single-use aseptic connectors and bag/vial assemblies are purchased sterile
//   components; this station only provides docks, nests, and handoff geometry.
// - Additive disposition needs physical separation between released, hold, and
//   reject states, plus spill capture and calibration-weight custody.
//
// This is mechanical packaging/interface CAD. It does not model compounding
// chemistry, open powder handling, sterility validation, or a dispensing recipe.

const OUTPUTS: [&str; 13] = [
    "output/closed_reagent_additive_weigh_dispense_verification_station_base_spill_tray.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_balance_load_cell_bays.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_additive_vial_bag_nests.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_gravimetric_verification_pads.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_sterile_connector_docks.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_released_hold_reject_lanes.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_barcode_rfid_coa_lands.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_calibration_weight_custody_pockets.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_clean_used_segregation.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_closed_handoff_bulkhead.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_robot_service_keepouts.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_guard_enclosure.stl",
    "output/closed_reagent_additive_weigh_dispense_verification_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "additive_vial_nests",
    "additive_bag_nests",
    "gravimetric_verification_pads",
    "purchased_balance_load_cell_bays",
    "sterile_connector_docks",
    "released_hold_reject_lanes",
    "barcode_rfid_coa_lands",
    "calibration_weight_custody_pockets",
    "spill_leak_tray",
    "clean_used_segregation",
    "robot_service_keepouts",
    "assembly_export",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 24.0;
const TRAY_RIM_W: f64 = 24.0;
const TRAY_RIM_Z: f64 = 46.0;
const SPILL_SUMP_X: f64 = 1130.0;
const SPILL_SUMP_Y: f64 = 660.0;
const SPILL_SUMP_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 18.0;

const BALANCE_COUNT: usize = 2;
const BALANCE_BAY_X: f64 = 310.0;
const BALANCE_BAY_Y: f64 = 250.0;
const BALANCE_BAY_Z: f64 = 38.0;
const BALANCE_PITCH_X: f64 = 390.0;
const LOAD_CELL_PAD_X: f64 = 172.0;
const LOAD_CELL_PAD_Y: f64 = 142.0;
const LOAD_CELL_PAD_Z: f64 = 12.0;
const DRAFT_SHIELD_Z: f64 = 250.0;

const VIAL_NEST_COUNT: usize = 8;
const VIAL_COLS: usize = 4;
const VIAL_NEST_D: f64 = 34.0;
const VIAL_COLLAR_D: f64 = 58.0;
const VIAL_PITCH_X: f64 = 92.0;
const VIAL_PITCH_Y: f64 = 82.0;
const VIAL_CENTER_X: f64 = -390.0;
const VIAL_CENTER_Y: f64 = 108.0;

const BAG_NEST_COUNT: usize = 4;
const BAG_NEST_X: f64 = 155.0;
const BAG_NEST_Y: f64 = 220.0;
const BAG_NEST_Z: f64 = 20.0;
const BAG_PITCH_X: f64 = 190.0;
const BAG_CENTER_X: f64 = 335.0;
const BAG_CENTER_Y: f64 = 108.0;

const VERIFICATION_PAD_COUNT: usize = VIAL_NEST_COUNT + BAG_NEST_COUNT;
const VERIFY_PAD_X: f64 = 64.0;
const VERIFY_PAD_Y: f64 = 48.0;
const VERIFY_PAD_Z: f64 = 8.0;

const CONNECTOR_DOCKS: usize = 6;
const CONNECTOR_DOCK_X: f64 = 132.0;
const CONNECTOR_DOCK_Y: f64 = 76.0;
const CONNECTOR_DOCK_Z: f64 = 44.0;
const CONNECTOR_PITCH_X: f64 = 168.0;
const CONNECTOR_PORT_D: f64 = 24.0;
const CAP_PARKS: usize = CONNECTOR_DOCKS;

const DISPOSITION_LANES: usize = 3;
const LANE_X: f64 = 292.0;
const LANE_Y: f64 = 156.0;
const LANE_Z: f64 = 24.0;
const LANE_PITCH_X: f64 = 330.0;

const BARCODE_LANDS: usize = VIAL_NEST_COUNT + BAG_NEST_COUNT + DISPOSITION_LANES;
const LABEL_LAND_X: f64 = 76.0;
const LABEL_LAND_Y: f64 = 34.0;
const RFID_LAND_X: f64 = 46.0;
const RFID_LAND_Y: f64 = 38.0;
const COA_CARD_X: f64 = 210.0;
const COA_CARD_Y: f64 = 74.0;

const CAL_WEIGHT_POCKETS: usize = 5;
const CAL_WEIGHT_POCKET_D: f64 = 42.0;
const CAL_WEIGHT_PITCH_X: f64 = 64.0;
const CAL_DRAWER_X: f64 = 420.0;
const CAL_DRAWER_Y: f64 = 112.0;
const CAL_DRAWER_Z: f64 = 42.0;

const SEGREGATION_WALL_X: f64 = 28.0;
const SEGREGATION_WALL_Y: f64 = 700.0;
const SEGREGATION_WALL_Z: f64 = 150.0;
const USED_BIN_X: f64 = 230.0;
const USED_BIN_Y: f64 = 160.0;
const USED_BIN_Z: f64 = 70.0;
const CLEAN_BIN_X: f64 = 230.0;
const CLEAN_BIN_Y: f64 = 160.0;
const CLEAN_BIN_Z: f64 = 70.0;

const HANDOFF_BULKHEAD_X: f64 = 1110.0;
const HANDOFF_BULKHEAD_Y: f64 = 34.0;
const HANDOFF_BULKHEAD_Z: f64 = 230.0;
const HANDOFF_PORTS: usize = 12;
const HANDOFF_PORT_PITCH_X: f64 = 82.0;

const FRONT_ROBOT_KEEP_OUT: f64 = 520.0;
const REAR_SERVICE_KEEP_OUT: f64 = 360.0;
const LEFT_SERVICE_KEEP_OUT: f64 = 260.0;
const RIGHT_SERVICE_KEEP_OUT: f64 = 320.0;
const KEEP_OUT_Z: f64 = 170.0;

const GUARD_X: f64 = 1240.0;
const GUARD_Y: f64 = 760.0;
const GUARD_Z: f64 = 420.0;
const GUARD_WALL_T: f64 = 18.0;
const PASS_THROUGH_SLOT_X: f64 = 460.0;
const PASS_THROUGH_SLOT_Z: f64 = 112.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_spill_tray();
    export(OUTPUTS[0], &base);

    let balances = balance_load_cell_bays();
    export(OUTPUTS[1], &balances);

    let nests = additive_vial_bag_nests();
    export(OUTPUTS[2], &nests);

    let verification = gravimetric_verification_pads();
    export(OUTPUTS[3], &verification);

    let connector_docks = sterile_connector_docks();
    export(OUTPUTS[4], &connector_docks);

    let lanes = released_hold_reject_lanes();
    export(OUTPUTS[5], &lanes);

    let traceability = barcode_rfid_coa_lands();
    export(OUTPUTS[6], &traceability);

    let calibration = calibration_weight_custody_pockets();
    export(OUTPUTS[7], &calibration);

    let segregation = clean_used_segregation();
    export(OUTPUTS[8], &segregation);

    let handoff = closed_handoff_bulkhead();
    export(OUTPUTS[9], &handoff);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let guard = guard_enclosure();
    export(OUTPUTS[11], &guard);

    let assembly = base
        + balances
        + nests
        + verification
        + connector_docks
        + lanes
        + traceability
        + calibration
        + segregation
        + handoff
        + keepouts
        + guard;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed reagent additive weigh/dispense verification station:");
    println!(
        "  Deck and containment:       {DECK_X:.0}mm x {DECK_Y:.0}mm tray with recessed {SPILL_SUMP_X:.0}mm x {SPILL_SUMP_Y:.0}mm sump and {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Purchased weighing bays:    {BALANCE_COUNT} balance/load-cell bays with {LOAD_CELL_PAD_X:.0}mm x {LOAD_CELL_PAD_Y:.0}mm verification pads and draft-shield envelopes"
    );
    println!(
        "  Additive container nests:   {VIAL_NEST_COUNT} vial nests, {BAG_NEST_COUNT} bag nests, {VERIFICATION_PAD_COUNT} gravimetric check lands"
    );
    println!(
        "  Closed connector handoff:   {CONNECTOR_DOCKS} connector docks, {HANDOFF_PORTS} bulkhead ports, {CAP_PARKS} cap parks"
    );
    println!(
        "  Disposition controls:       {DISPOSITION_LANES} released/hold/reject lanes, {BARCODE_LANDS} barcode/RFID lands, {CAL_WEIGHT_POCKETS} calibration-weight custody pockets"
    );
    println!(
        "  Keepouts:                   front robot {FRONT_ROBOT_KEEP_OUT:.0}mm, rear service {REAR_SERVICE_KEEP_OUT:.0}mm, left {LEFT_SERVICE_KEEP_OUT:.0}mm, right {RIGHT_SERVICE_KEEP_OUT:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(VERIFICATION_PAD_COUNT, VIAL_NEST_COUNT + BAG_NEST_COUNT);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(vial_span_x() < 420.0);
    assert!(bag_span_x() < 760.0);
    assert!(connector_span_x() + CONNECTOR_DOCK_X < HANDOFF_BULKHEAD_X);
    assert!(handoff_span_x() + CONNECTOR_PORT_D < HANDOFF_BULKHEAD_X - 80.0);
    assert!(BALANCE_PITCH_X + BALANCE_BAY_X < DECK_X - 220.0);
    assert!(LANE_PITCH_X * 2.0 + LANE_X < DECK_X - 160.0);
    assert!(SPILL_SUMP_X < DECK_X - 2.0 * TRAY_RIM_W);
    assert!(SPILL_SUMP_Y < DECK_Y - 2.0 * TRAY_RIM_W);
}

fn base_spill_tray() -> Part {
    let deck = centered_cube(
        "closed_reagent_additive_station_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        "closed_reagent_additive_station_recessed_spill_sump_cut",
        SPILL_SUMP_X,
        SPILL_SUMP_Y,
        SPILL_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - SPILL_SUMP_DEPTH / 2.0);
    let drain = centered_cylinder(
        "closed_reagent_additive_station_spill_drain_port_cut",
        DRAIN_PORT_D / 2.0,
        DECK_Z + 8.0,
        32,
    )
    .translate(DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 86.0, DECK_Z / 2.0);

    deck - sump - drain
        + rim("front", DECK_X, TRAY_RIM_W, TRAY_RIM_Z).translate(
            0.0,
            -DECK_Y / 2.0,
            TRAY_RIM_Z / 2.0,
        )
        + rim("rear", DECK_X, TRAY_RIM_W, TRAY_RIM_Z).translate(0.0, DECK_Y / 2.0, TRAY_RIM_Z / 2.0)
        + rim("left", TRAY_RIM_W, DECK_Y, TRAY_RIM_Z).translate(
            -DECK_X / 2.0,
            0.0,
            TRAY_RIM_Z / 2.0,
        )
        + rim("right", TRAY_RIM_W, DECK_Y, TRAY_RIM_Z).translate(
            DECK_X / 2.0,
            0.0,
            TRAY_RIM_Z / 2.0,
        )
}

fn balance_load_cell_bays() -> Part {
    let mut bays = Part::empty("closed_reagent_additive_station_balance_load_cell_bays");
    for i in 0..BALANCE_COUNT {
        let x = balance_x(i);
        let bay = centered_cube(
            format!("closed_reagent_additive_station_balance_{i}_purchased_scale_bay"),
            BALANCE_BAY_X,
            BALANCE_BAY_Y,
            BALANCE_BAY_Z,
        )
        .translate(x, -156.0, DECK_Z + BALANCE_BAY_Z / 2.0);
        let pocket = centered_cube(
            format!("closed_reagent_additive_station_balance_{i}_drop_in_scale_pocket_cut"),
            BALANCE_BAY_X - 56.0,
            BALANCE_BAY_Y - 58.0,
            BALANCE_BAY_Z + 4.0,
        )
        .translate(x, -156.0, DECK_Z + BALANCE_BAY_Z / 2.0 + 6.0);
        let load_cell = centered_cube(
            format!("closed_reagent_additive_station_balance_{i}_load_cell_interface_pad"),
            LOAD_CELL_PAD_X,
            LOAD_CELL_PAD_Y,
            LOAD_CELL_PAD_Z,
        )
        .translate(
            x,
            -156.0,
            DECK_Z + BALANCE_BAY_Z + LOAD_CELL_PAD_Z / 2.0 + 2.0,
        );
        let shield = guard_frame(
            format!("closed_reagent_additive_station_balance_{i}_draft_shield_envelope"),
            BALANCE_BAY_X + 48.0,
            BALANCE_BAY_Y + 42.0,
            DRAFT_SHIELD_Z,
        )
        .translate(x, -156.0, DECK_Z + BALANCE_BAY_Z + DRAFT_SHIELD_Z / 2.0);
        bays = bays + (bay - pocket) + load_cell + shield;
    }
    bays
}

fn additive_vial_bag_nests() -> Part {
    let mut nests = Part::empty("closed_reagent_additive_station_additive_vial_bag_nests");
    for i in 0..VIAL_NEST_COUNT {
        let (x, y) = vial_xy(i);
        let collar = centered_cylinder(
            format!("closed_reagent_additive_station_vial_{i}_datum_collar"),
            VIAL_COLLAR_D / 2.0,
            20.0,
            48,
        )
        .translate(x, y, DECK_Z + 10.0);
        let bore = centered_cylinder(
            format!("closed_reagent_additive_station_vial_{i}_body_clearance_cut"),
            VIAL_NEST_D / 2.0,
            26.0,
            48,
        )
        .translate(x, y, DECK_Z + 12.0);
        let anti_rotate = centered_cube(
            format!("closed_reagent_additive_station_vial_{i}_cap_tab_flat"),
            18.0,
            8.0,
            14.0,
        )
        .translate(x, y - VIAL_COLLAR_D / 2.0 + 4.0, DECK_Z + 18.0);
        nests = nests + (collar - bore) + anti_rotate;
    }

    for i in 0..BAG_NEST_COUNT {
        let x = bag_x(i);
        let tray = centered_cube(
            format!("closed_reagent_additive_station_bag_{i}_flex_bag_nest_tray"),
            BAG_NEST_X,
            BAG_NEST_Y,
            BAG_NEST_Z,
        )
        .translate(x, BAG_CENTER_Y, DECK_Z + BAG_NEST_Z / 2.0);
        let recess = centered_cube(
            format!("closed_reagent_additive_station_bag_{i}_bag_recess_cut"),
            BAG_NEST_X - 28.0,
            BAG_NEST_Y - 36.0,
            BAG_NEST_Z + 4.0,
        )
        .translate(x, BAG_CENTER_Y, DECK_Z + BAG_NEST_Z / 2.0 + 5.0);
        let neck_clip = centered_cube(
            format!("closed_reagent_additive_station_bag_{i}_neck_clip"),
            50.0,
            22.0,
            32.0,
        )
        .translate(x, BAG_CENTER_Y + BAG_NEST_Y / 2.0 - 18.0, DECK_Z + 32.0);
        nests = nests + (tray - recess) + neck_clip;
    }
    nests
}

fn gravimetric_verification_pads() -> Part {
    let mut pads = Part::empty("closed_reagent_additive_station_gravimetric_verification_pads");
    for i in 0..VIAL_NEST_COUNT {
        let (x, y) = vial_xy(i);
        pads = pads
            + centered_cube(
                format!("closed_reagent_additive_station_vial_{i}_verification_pad"),
                VERIFY_PAD_X,
                VERIFY_PAD_Y,
                VERIFY_PAD_Z,
            )
            .translate(x, y - 48.0, DECK_Z + VERIFY_PAD_Z / 2.0 + 2.0);
    }
    for i in 0..BAG_NEST_COUNT {
        pads = pads
            + centered_cube(
                format!("closed_reagent_additive_station_bag_{i}_verification_pad"),
                VERIFY_PAD_X + 36.0,
                VERIFY_PAD_Y,
                VERIFY_PAD_Z,
            )
            .translate(
                bag_x(i),
                BAG_CENTER_Y - 136.0,
                DECK_Z + VERIFY_PAD_Z / 2.0 + 2.0,
            );
    }
    pads
}

fn sterile_connector_docks() -> Part {
    let mut docks = Part::empty("closed_reagent_additive_station_sterile_connector_docks");
    for i in 0..CONNECTOR_DOCKS {
        let x = connector_x(i);
        let block = centered_cube(
            format!("closed_reagent_additive_station_connector_dock_{i}_body"),
            CONNECTOR_DOCK_X,
            CONNECTOR_DOCK_Y,
            CONNECTOR_DOCK_Z,
        )
        .translate(x, DECK_Y / 2.0 - 132.0, DECK_Z + CONNECTOR_DOCK_Z / 2.0);
        let port = centered_cylinder(
            format!("closed_reagent_additive_station_connector_dock_{i}_sterile_port_cut"),
            CONNECTOR_PORT_D / 2.0,
            CONNECTOR_DOCK_Y + 8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, DECK_Y / 2.0 - 132.0, DECK_Z + CONNECTOR_DOCK_Z / 2.0);
        let latch_land = centered_cube(
            format!("closed_reagent_additive_station_connector_dock_{i}_latch_land"),
            38.0,
            16.0,
            18.0,
        )
        .translate(
            x + 34.0,
            DECK_Y / 2.0 - 174.0,
            DECK_Z + CONNECTOR_DOCK_Z + 10.0,
        );
        let cap_park = centered_cylinder(
            format!("closed_reagent_additive_station_connector_dock_{i}_cap_park"),
            15.0,
            18.0,
            30,
        )
        .translate(x - 38.0, DECK_Y / 2.0 - 180.0, DECK_Z + 9.0);
        docks = docks + (block - port) + latch_land + cap_park;
    }
    docks
}

fn released_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("closed_reagent_additive_station_released_hold_reject_lanes");
    for (i, name) in ["released", "hold", "reject"].iter().enumerate() {
        let x = lane_x(i);
        let tray = centered_cube(
            format!("closed_reagent_additive_station_{name}_lane_tray"),
            LANE_X,
            LANE_Y,
            LANE_Z,
        )
        .translate(x, -DECK_Y / 2.0 + 116.0, DECK_Z + LANE_Z / 2.0);
        let recess = centered_cube(
            format!("closed_reagent_additive_station_{name}_lane_recess_cut"),
            LANE_X - 44.0,
            LANE_Y - 42.0,
            LANE_Z + 4.0,
        )
        .translate(x, -DECK_Y / 2.0 + 116.0, DECK_Z + LANE_Z / 2.0 + 5.0);
        let gate = centered_cube(
            format!("closed_reagent_additive_station_{name}_lane_interlock_gate"),
            LANE_X - 58.0,
            18.0,
            52.0,
        )
        .translate(x, -DECK_Y / 2.0 + 194.0, DECK_Z + 46.0);
        lanes = lanes + (tray - recess) + gate;
    }
    lanes
}

fn barcode_rfid_coa_lands() -> Part {
    let mut lands = Part::empty("closed_reagent_additive_station_barcode_rfid_coa_lands");
    for i in 0..VIAL_NEST_COUNT {
        let (x, y) = vial_xy(i);
        lands = lands + label_pair(format!("vial_{i}"), x + 48.0, y + 22.0);
    }
    for i in 0..BAG_NEST_COUNT {
        lands = lands + label_pair(format!("bag_{i}"), bag_x(i) + 44.0, BAG_CENTER_Y + 68.0);
    }
    for i in 0..DISPOSITION_LANES {
        lands = lands
            + label_pair(
                format!("disposition_lane_{i}"),
                lane_x(i),
                -DECK_Y / 2.0 + 42.0,
            );
    }
    let coa_card = centered_cube(
        "closed_reagent_additive_station_coa_packet_land",
        COA_CARD_X,
        COA_CARD_Y,
        6.0,
    )
    .translate(-DECK_X / 2.0 + 156.0, -DECK_Y / 2.0 + 244.0, DECK_Z + 7.0);
    let scanner_tower = centered_cube(
        "closed_reagent_additive_station_barcode_rfid_scanner_tower_envelope",
        96.0,
        62.0,
        190.0,
    )
    .translate(-DECK_X / 2.0 + 110.0, -42.0, DECK_Z + 95.0);
    lands + coa_card + scanner_tower
}

fn calibration_weight_custody_pockets() -> Part {
    let drawer = centered_cube(
        "closed_reagent_additive_station_calibration_weight_locking_drawer",
        CAL_DRAWER_X,
        CAL_DRAWER_Y,
        CAL_DRAWER_Z,
    )
    .translate(
        DECK_X / 2.0 - 258.0,
        -DECK_Y / 2.0 + 258.0,
        DECK_Z + CAL_DRAWER_Z / 2.0,
    );
    let mut pockets = Part::empty("closed_reagent_additive_station_calibration_weight_pocket_cuts");
    let mut seals = Part::empty("closed_reagent_additive_station_calibration_weight_tamper_seals");
    for i in 0..CAL_WEIGHT_POCKETS {
        let x = DECK_X / 2.0 - 258.0 + (i as f64 - 2.0) * CAL_WEIGHT_PITCH_X;
        pockets = pockets
            + centered_cylinder(
                format!("closed_reagent_additive_station_calibration_weight_{i}_pocket_cut"),
                CAL_WEIGHT_POCKET_D / 2.0,
                CAL_DRAWER_Z + 6.0,
                40,
            )
            .translate(x, -DECK_Y / 2.0 + 258.0, DECK_Z + CAL_DRAWER_Z / 2.0 + 6.0);
        seals = seals
            + centered_cube(
                format!("closed_reagent_additive_station_calibration_weight_{i}_custody_seal_land"),
                30.0,
                8.0,
                12.0,
            )
            .translate(x, -DECK_Y / 2.0 + 200.0, DECK_Z + CAL_DRAWER_Z + 8.0);
    }
    drawer - pockets + seals
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "closed_reagent_additive_station_clean_used_center_segregation_wall",
        SEGREGATION_WALL_X,
        SEGREGATION_WALL_Y,
        SEGREGATION_WALL_Z,
    )
    .translate(0.0, 18.0, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let clean_bin = centered_cube(
        "closed_reagent_additive_station_clean_connector_cap_bin",
        CLEAN_BIN_X,
        CLEAN_BIN_Y,
        CLEAN_BIN_Z,
    )
    .translate(
        -DECK_X / 2.0 + 162.0,
        DECK_Y / 2.0 - 156.0,
        DECK_Z + CLEAN_BIN_Z / 2.0,
    );
    let used_bin = centered_cube(
        "closed_reagent_additive_station_used_connector_cap_bin",
        USED_BIN_X,
        USED_BIN_Y,
        USED_BIN_Z,
    )
    .translate(
        DECK_X / 2.0 - 162.0,
        DECK_Y / 2.0 - 156.0,
        DECK_Z + USED_BIN_Z / 2.0,
    );
    let clean_cut = centered_cube(
        "closed_reagent_additive_station_clean_bin_recess_cut",
        CLEAN_BIN_X - 38.0,
        CLEAN_BIN_Y - 34.0,
        CLEAN_BIN_Z + 4.0,
    )
    .translate(
        -DECK_X / 2.0 + 162.0,
        DECK_Y / 2.0 - 156.0,
        DECK_Z + CLEAN_BIN_Z / 2.0 + 8.0,
    );
    let used_cut = centered_cube(
        "closed_reagent_additive_station_used_bin_recess_cut",
        USED_BIN_X - 38.0,
        USED_BIN_Y - 34.0,
        USED_BIN_Z + 4.0,
    )
    .translate(
        DECK_X / 2.0 - 162.0,
        DECK_Y / 2.0 - 156.0,
        DECK_Z + USED_BIN_Z / 2.0 + 8.0,
    );
    wall + (clean_bin - clean_cut) + (used_bin - used_cut)
}

fn closed_handoff_bulkhead() -> Part {
    let plate = centered_cube(
        "closed_reagent_additive_station_closed_connector_handoff_bulkhead",
        HANDOFF_BULKHEAD_X,
        HANDOFF_BULKHEAD_Y,
        HANDOFF_BULKHEAD_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + HANDOFF_BULKHEAD_Y / 2.0,
        DECK_Z + HANDOFF_BULKHEAD_Z / 2.0,
    );
    let mut ports = Part::empty("closed_reagent_additive_station_handoff_bulkhead_port_cuts");
    let mut collars = Part::empty("closed_reagent_additive_station_handoff_bulkhead_port_collars");
    for i in 0..HANDOFF_PORTS {
        let x = handoff_x(i);
        ports = ports
            + centered_cylinder(
                format!("closed_reagent_additive_station_handoff_port_{i}_cut"),
                CONNECTOR_PORT_D / 2.0,
                HANDOFF_BULKHEAD_Y + 10.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, DECK_Y / 2.0 + HANDOFF_BULKHEAD_Y / 2.0, DECK_Z + 122.0);
        collars = collars
            + centered_cylinder(
                format!("closed_reagent_additive_station_handoff_port_{i}_collar"),
                20.0,
                10.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, DECK_Y / 2.0 + HANDOFF_BULKHEAD_Y + 6.0, DECK_Z + 122.0);
    }
    plate - ports + collars
}

fn robot_service_keepouts() -> Part {
    let front = keepout_box(
        "front_robot_approach",
        DECK_X,
        FRONT_ROBOT_KEEP_OUT,
        KEEP_OUT_Z,
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT / 2.0,
    );
    let rear = keepout_box(
        "rear_service_access",
        DECK_X,
        REAR_SERVICE_KEEP_OUT,
        KEEP_OUT_Z,
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT / 2.0 + HANDOFF_BULKHEAD_Y,
    );
    let left = keepout_box(
        "left_clean_material_service",
        LEFT_SERVICE_KEEP_OUT,
        DECK_Y,
        KEEP_OUT_Z,
        -DECK_X / 2.0 - LEFT_SERVICE_KEEP_OUT / 2.0,
        0.0,
    );
    let right = keepout_box(
        "right_used_material_service",
        RIGHT_SERVICE_KEEP_OUT,
        DECK_Y,
        KEEP_OUT_Z,
        DECK_X / 2.0 + RIGHT_SERVICE_KEEP_OUT / 2.0,
        0.0,
    );
    front + rear + left + right
}

fn guard_enclosure() -> Part {
    let outer = centered_cube(
        "closed_reagent_additive_station_clear_guard_outer_envelope",
        GUARD_X,
        GUARD_Y,
        GUARD_Z,
    )
    .translate(0.0, 0.0, DECK_Z + GUARD_Z / 2.0);
    let inner = centered_cube(
        "closed_reagent_additive_station_clear_guard_inner_volume_cut",
        GUARD_X - 2.0 * GUARD_WALL_T,
        GUARD_Y - 2.0 * GUARD_WALL_T,
        GUARD_Z - GUARD_WALL_T,
    )
    .translate(
        0.0,
        0.0,
        DECK_Z + GUARD_WALL_T + (GUARD_Z - GUARD_WALL_T) / 2.0,
    );
    let front_slot = centered_cube(
        "closed_reagent_additive_station_guard_front_material_pass_slot_cut",
        PASS_THROUGH_SLOT_X,
        GUARD_WALL_T + 6.0,
        PASS_THROUGH_SLOT_Z,
    )
    .translate(0.0, -GUARD_Y / 2.0, DECK_Z + 152.0);
    let service_slot = centered_cube(
        "closed_reagent_additive_station_guard_rear_connector_service_slot_cut",
        PASS_THROUGH_SLOT_X + 260.0,
        GUARD_WALL_T + 6.0,
        PASS_THROUGH_SLOT_Z,
    )
    .translate(0.0, GUARD_Y / 2.0, DECK_Z + 186.0);

    outer - inner - front_slot - service_slot
        + frame_xz(
            "closed_reagent_additive_station_guard_front_pass_slot_gasket",
            PASS_THROUGH_SLOT_X + 42.0,
            12.0,
            PASS_THROUGH_SLOT_Z + 36.0,
            14.0,
        )
        .translate(0.0, -GUARD_Y / 2.0 - 8.0, DECK_Z + 152.0)
        + frame_xz(
            "closed_reagent_additive_station_guard_rear_connector_slot_gasket",
            PASS_THROUGH_SLOT_X + 302.0,
            12.0,
            PASS_THROUGH_SLOT_Z + 36.0,
            14.0,
        )
        .translate(0.0, GUARD_Y / 2.0 + 8.0, DECK_Z + 186.0)
}

fn rim(name: &str, x: f64, y: f64, z: f64) -> Part {
    centered_cube(
        format!("closed_reagent_additive_station_spill_tray_{name}_rim"),
        x,
        y,
        z,
    )
}

fn guard_frame(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let rail = 12.0;
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail, z).translate(0.0, y / 2.0, 0.0);
    let front =
        centered_cube(format!("{name}_front_rail"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right =
        centered_cube(format!("{name}_right_rail"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    rear + front + left + right
}

fn label_pair(name: String, x: f64, y: f64) -> Part {
    centered_cube(
        format!("closed_reagent_additive_station_{name}_barcode_land"),
        LABEL_LAND_X,
        LABEL_LAND_Y,
        5.0,
    )
    .translate(x, y, DECK_Z + 6.0)
        + centered_cube(
            format!("closed_reagent_additive_station_{name}_rfid_land"),
            RFID_LAND_X,
            RFID_LAND_Y,
            5.0,
        )
        .translate(x - 62.0, y, DECK_Z + 6.0)
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64, cx: f64, cy: f64) -> Part {
    guard_frame(
        format!("closed_reagent_additive_station_{name}_keepout_frame"),
        x,
        y,
        z,
    )
    .translate(cx, cy, DECK_Z + z / 2.0)
}

fn frame_xz(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), x, y, rail).translate(0.0, 0.0, z / 2.0);
    let bottom = centered_cube(format!("{name}_bottom"), x, y, rail).translate(0.0, 0.0, -z / 2.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    top + bottom + left + right
}

fn balance_x(index: usize) -> f64 {
    (index as f64 - (BALANCE_COUNT as f64 - 1.0) / 2.0) * BALANCE_PITCH_X
}

fn vial_xy(index: usize) -> (f64, f64) {
    let col = index % VIAL_COLS;
    let row = index / VIAL_COLS;
    let x = VIAL_CENTER_X + (col as f64 - (VIAL_COLS as f64 - 1.0) / 2.0) * VIAL_PITCH_X;
    let y = VIAL_CENTER_Y + (row as f64 - 0.5) * VIAL_PITCH_Y;
    (x, y)
}

fn bag_x(index: usize) -> f64 {
    BAG_CENTER_X + (index as f64 - (BAG_NEST_COUNT as f64 - 1.0) / 2.0) * BAG_PITCH_X
}

fn connector_x(index: usize) -> f64 {
    (index as f64 - (CONNECTOR_DOCKS as f64 - 1.0) / 2.0) * CONNECTOR_PITCH_X
}

fn lane_x(index: usize) -> f64 {
    (index as f64 - 1.0) * LANE_PITCH_X
}

fn handoff_x(index: usize) -> f64 {
    (index as f64 - (HANDOFF_PORTS as f64 - 1.0) / 2.0) * HANDOFF_PORT_PITCH_X
}

fn vial_span_x() -> f64 {
    (VIAL_COLS as f64 - 1.0) * VIAL_PITCH_X + VIAL_COLLAR_D
}

fn bag_span_x() -> f64 {
    (BAG_NEST_COUNT as f64 - 1.0) * BAG_PITCH_X + BAG_NEST_X
}

fn connector_span_x() -> f64 {
    (CONNECTOR_DOCKS as f64 - 1.0) * CONNECTOR_PITCH_X
}

fn handoff_span_x() -> f64 {
    (HANDOFF_PORTS as f64 - 1.0) * HANDOFF_PORT_PITCH_X
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_reagent_additive_weigh_dispense_verification_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_set_covers_task_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"additive_vial_nests"));
        assert!(REQUIRED_FEATURES.contains(&"additive_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"gravimetric_verification_pads"));
        assert!(REQUIRED_FEATURES.contains(&"purchased_balance_load_cell_bays"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_connector_docks"));
        assert!(REQUIRED_FEATURES.contains(&"released_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"calibration_weight_custody_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"spill_leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"assembly_export"));
    }

    #[test]
    fn container_and_verification_counts_match() {
        assert_eq!(VIAL_NEST_COUNT, 8);
        assert_eq!(BAG_NEST_COUNT, 4);
        assert_eq!(VERIFICATION_PAD_COUNT, 12);
        assert_eq!(BARCODE_LANDS, VERIFICATION_PAD_COUNT + DISPOSITION_LANES);
        assert!(VERIFY_PAD_X > CAL_WEIGHT_POCKET_D);
    }

    #[test]
    fn balances_connectors_and_lanes_fit_on_deck() {
        assert!(BALANCE_BAY_X < BALANCE_PITCH_X);
        assert!(BALANCE_PITCH_X + BALANCE_BAY_X < DECK_X - 200.0);
        assert!(bag_span_x() < 760.0);
        assert!(connector_span_x() + CONNECTOR_DOCK_X < HANDOFF_BULKHEAD_X);
        assert!(handoff_span_x() + CONNECTOR_PORT_D < HANDOFF_BULKHEAD_X - 80.0);
        assert!(LANE_PITCH_X * 2.0 + LANE_X < DECK_X - 160.0);
    }

    #[test]
    fn segregation_and_spill_containment_are_physical() {
        assert!(SEGREGATION_WALL_Z > USED_BIN_Z);
        assert!(SEGREGATION_WALL_Y < DECK_Y - 80.0);
        assert!(SPILL_SUMP_X < DECK_X - 2.0 * TRAY_RIM_W);
        assert!(SPILL_SUMP_Y < DECK_Y - 2.0 * TRAY_RIM_W);
        assert!(DRAIN_PORT_D >= 12.0);
    }

    #[test]
    fn keepouts_and_guard_preserve_service_access() {
        assert!(FRONT_ROBOT_KEEP_OUT >= 500.0);
        assert!(REAR_SERVICE_KEEP_OUT >= 320.0);
        assert!(LEFT_SERVICE_KEEP_OUT >= 240.0);
        assert!(RIGHT_SERVICE_KEEP_OUT >= 300.0);
        assert!(GUARD_X < DECK_X);
        assert!(GUARD_Y < DECK_Y);
        assert!(PASS_THROUGH_SLOT_X < GUARD_X - 400.0);
    }
}
