use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed single-use tubing harness disposal and deactivation station.
//
// Intent:
// - Package the post-run used-harness handoff, cap/plug accountability checks,
//   bought disinfectant/neutralizer container docks, contact-time token lands,
//   leak/drip containment, waste routing, quarantine, evidence sampling, and
//   run-record scan surfaces in one physically segregated station.
// - Keep clean service parts, used harnesses, and rejects separated so a robot
//   or operator cannot confuse material state after a culture run.
//
// This is packaging/interface CAD. Disinfectant, neutralizer, contact-time, and
// waste paths are placeholders for selected commercial containers and validated
// site procedures; this file does not model safety-critical decon internals.

const OUTPUTS: &[&str] = &[
    "output/closed_single_use_harness_disposal_deactivation_station_deck_drip_tray.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_sealed_used_harness_receiver.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_connector_cap_plug_retention_checks.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_disinfectant_neutralizer_docks.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_contact_time_token_lands.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_liquid_waste_routing.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_solid_waste_quarantine_bin.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_sample_evidence_vial_wells.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_barcode_rfid_run_record_lands.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_clean_used_reject_segregation.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_scanner_bridge.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_robot_service_keepouts.stl",
    "output/closed_single_use_harness_disposal_deactivation_station_assembly.stl",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const DRIP_BASIN_DEPTH: f64 = 9.0;

const CLEAN_ZONE_X: f64 = -398.0;
const USED_ZONE_X: f64 = -54.0;
const REJECT_ZONE_X: f64 = 318.0;
const REAR_ROW_Y: f64 = 166.0;
const FRONT_ROW_Y: f64 = -244.0;

const RECEIVER_X: f64 = 378.0;
const RECEIVER_Y: f64 = 274.0;
const RECEIVER_Z: f64 = 168.0;
const RECEIVER_MOUTH_X: f64 = 278.0;
const RECEIVER_MOUTH_Y: f64 = 144.0;
const RECEIVER_GASKET_W: f64 = 12.0;
const RECEIVER_LID_Z: f64 = 18.0;
const RECEIVER_LATCH_COUNT: usize = 4;

const CONNECTOR_COUNT: usize = 12;
const CAP_WELL_COUNT: usize = 24;
const PLUG_WELL_COUNT: usize = 24;
const RETENTION_BLOCK_X: f64 = 352.0;
const RETENTION_BLOCK_Y: f64 = 206.0;
const RETENTION_BLOCK_Z: f64 = 38.0;
const CONNECTOR_GO_D: f64 = 11.0;
const CAP_WELL_D: f64 = 8.4;
const PLUG_WELL_D: f64 = 6.2;

const CHEM_DOCK_X: f64 = 390.0;
const CHEM_DOCK_Y: f64 = 250.0;
const CHEM_DOCK_Z: f64 = 54.0;
const CHEM_BOTTLE_D: f64 = 96.0;
const CHEM_BOTTLE_CLEARANCE: f64 = 8.0;
const CHEM_BOTTLE_COUNT: usize = 2;
const CHEM_STRAP_Z: f64 = 104.0;

const CONTACT_TOKEN_COUNT: usize = 8;
const TOKEN_LAND_X: f64 = 74.0;
const TOKEN_LAND_Y: f64 = 38.0;
const TOKEN_LAND_Z: f64 = 8.0;
const TOKEN_RAIL_X: f64 = 690.0;
const TOKEN_RAIL_Y: f64 = 64.0;

const WASTE_ROUTING_X: f64 = 562.0;
const WASTE_ROUTING_Y: f64 = 88.0;
const WASTE_ROUTING_Z: f64 = 42.0;
const WASTE_PORT_COUNT: usize = 5;
const WASTE_PORT_D: f64 = 13.0;
const LIQUID_WASTE_BOTTLE_D: f64 = 122.0;

const SOLID_BIN_X: f64 = 306.0;
const SOLID_BIN_Y: f64 = 242.0;
const SOLID_BIN_Z: f64 = 154.0;
const SOLID_BIN_LID_Z: f64 = 16.0;

const VIAL_WELL_COUNT: usize = 10;
const VIAL_BLOCK_X: f64 = 310.0;
const VIAL_BLOCK_Y: f64 = 132.0;
const VIAL_BLOCK_Z: f64 = 38.0;
const VIAL_WELL_D: f64 = 13.5;

const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 6;
const RUN_RECORD_LANDS: usize = 3;
const LABEL_LAND_X: f64 = 94.0;
const LABEL_LAND_Y: f64 = 30.0;
const RFID_LAND_X: f64 = 52.0;
const RFID_LAND_Y: f64 = 36.0;

const SEGREGATION_WALL_Z: f64 = 112.0;
const SEGREGATION_WALL_W: f64 = 14.0;
const CLEAN_LANE_W: f64 = 250.0;
const USED_LANE_W: f64 = 360.0;
const REJECT_LANE_W: f64 = 248.0;

const SCANNER_BRIDGE_SPAN_X: f64 = 820.0;
const SCANNER_BRIDGE_POST_Y: f64 = 42.0;
const SCANNER_UNDERSIDE_Z: f64 = 236.0;
const SCANNER_BEAM_Z: f64 = 30.0;
const SCANNER_HEAD_COUNT: usize = 3;

const FRONT_ROBOT_KEEP_OUT: f64 = 430.0;
const REAR_SERVICE_KEEP_OUT: f64 = 280.0;
const CHEM_SWAP_KEEP_OUT: f64 = 230.0;
const WASTE_DRAWER_KEEP_OUT: f64 = 300.0;
const SERVICE_KEEP_OUT_Z: f64 = 318.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = deck_drip_tray();
    export(&deck, OUTPUTS[0]);

    let receiver = sealed_used_harness_receiver();
    export(&receiver, OUTPUTS[1]);

    let retention = connector_cap_plug_retention_checks();
    export(&retention, OUTPUTS[2]);

    let chemistry = disinfectant_neutralizer_docks();
    export(&chemistry, OUTPUTS[3]);

    let tokens = contact_time_token_lands();
    export(&tokens, OUTPUTS[4]);

    let routing = liquid_waste_routing();
    export(&routing, OUTPUTS[5]);

    let solid = solid_waste_quarantine_bin();
    export(&solid, OUTPUTS[6]);

    let evidence = sample_evidence_vial_wells();
    export(&evidence, OUTPUTS[7]);

    let records = barcode_rfid_run_record_lands();
    export(&records, OUTPUTS[8]);

    let segregation = clean_used_reject_segregation();
    export(&segregation, OUTPUTS[9]);

    let scanner = scanner_bridge();
    export(&scanner, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = deck
        + receiver.translate(USED_ZONE_X, REAR_ROW_Y, DECK_Z + RECEIVER_Z / 2.0)
        + retention.translate(CLEAN_ZONE_X, REAR_ROW_Y, DECK_Z + RETENTION_BLOCK_Z / 2.0)
        + chemistry.translate(REJECT_ZONE_X, REAR_ROW_Y, DECK_Z + CHEM_DOCK_Z / 2.0)
        + tokens.translate(USED_ZONE_X, 18.0, DECK_Z + TOKEN_LAND_Z / 2.0)
        + routing.translate(
            USED_ZONE_X,
            FRONT_ROW_Y + 88.0,
            DECK_Z + WASTE_ROUTING_Z / 2.0,
        )
        + solid.translate(REJECT_ZONE_X, FRONT_ROW_Y, DECK_Z + SOLID_BIN_Z / 2.0)
        + evidence.translate(CLEAN_ZONE_X, FRONT_ROW_Y, DECK_Z + VIAL_BLOCK_Z / 2.0)
        + records
        + segregation
        + scanner
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed single-use harness disposal/deactivation station: {:.0} x {:.0} mm drip-tray deck, sealed receiver {:.0} x {:.0} x {:.0} mm, {} connector checks, {} cap wells, {} plug wells.",
        DECK_X,
        DECK_Y,
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
        CONNECTOR_COUNT,
        CAP_WELL_COUNT,
        PLUG_WELL_COUNT
    );
    println!(
        "Bought chemistry docks: {} positions for disinfectant/neutralizer containers, {} contact-time token lands, {} waste ports, {} vial wells, {} barcode lands, {} RFID lands.",
        CHEM_BOTTLE_COUNT,
        CONTACT_TOKEN_COUNT,
        WASTE_PORT_COUNT,
        VIAL_WELL_COUNT,
        BARCODE_LANDS,
        RFID_LANDS
    );
    println!(
        "Segregation and access: clean {:.0} mm, used {:.0} mm, reject {:.0} mm lanes; robot/front {:.0} mm, rear service {:.0} mm, chemistry swap {:.0} mm keepouts.",
        CLEAN_LANE_W,
        USED_LANE_W,
        REJECT_LANE_W,
        FRONT_ROBOT_KEEP_OUT,
        REAR_SERVICE_KEEP_OUT,
        CHEM_SWAP_KEEP_OUT
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_drip_tray() -> Part {
    let deck = centered_cube("harness_disposal_deck_pan", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let basin = centered_cube(
        "harness_disposal_recessed_drip_basin",
        DECK_X - 2.0 * (RIM_W + 42.0),
        DECK_Y - 2.0 * (RIM_W + 46.0),
        DRIP_BASIN_DEPTH,
    )
    .translate(0.0, 10.0, DECK_Z - DRIP_BASIN_DEPTH / 2.0);
    let sump = centered_cube("harness_disposal_low_point_sump", 116.0, 58.0, DECK_Z + 4.0)
        .translate(USED_ZONE_X + 170.0, FRONT_ROW_Y + 36.0, DECK_Z / 2.0);
    let leak_sensor_land = centered_cube(
        "harness_disposal_leak_sensor_land",
        DECK_X - 190.0,
        18.0,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 58.0, DECK_Z + 3.5);

    deck - basin - sump + deck_rim() + deck_mount_holes() + leak_sensor_land
}

fn deck_rim() -> Part {
    let front = centered_cube("harness_disposal_front_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube("harness_disposal_rear_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("harness_disposal_left_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("harness_disposal_right_rim", RIM_W, DECK_Y, RIM_Z).translate(
        DECK_X / 2.0 - RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("harness_disposal_deck_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 86.0, -DECK_Y / 2.0 + 86.0),
        (DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 86.0),
        (-DECK_X / 2.0 + 86.0, DECK_Y / 2.0 - 86.0),
        (DECK_X / 2.0 - 86.0, DECK_Y / 2.0 - 86.0),
        (-120.0, -DECK_Y / 2.0 + 78.0),
        (120.0, -DECK_Y / 2.0 + 78.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("harness_disposal_mount_hole_{i}"),
                5.2,
                DECK_Z + 8.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn sealed_used_harness_receiver() -> Part {
    let shell = centered_cube(
        "harness_disposal_sealed_receiver_shell",
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
    );
    let cavity = centered_cube(
        "harness_disposal_receiver_bag_cavity",
        RECEIVER_X - 48.0,
        RECEIVER_Y - 46.0,
        RECEIVER_Z - 38.0,
    )
    .translate(0.0, 0.0, 16.0);
    let mouth = centered_cube(
        "harness_disposal_receiver_gasketed_mouth",
        RECEIVER_MOUTH_X,
        RECEIVER_MOUTH_Y,
        RECEIVER_Z + 6.0,
    )
    .translate(0.0, -38.0, 28.0);
    let lid = centered_cube(
        "harness_disposal_receiver_clear_lid_placeholder",
        RECEIVER_X - 28.0,
        RECEIVER_Y - 28.0,
        RECEIVER_LID_Z,
    )
    .translate(0.0, 0.0, RECEIVER_Z / 2.0 + RECEIVER_LID_Z / 2.0 + 6.0);
    let gasket_front = centered_cube(
        "harness_disposal_receiver_front_gasket_land",
        RECEIVER_MOUTH_X + 2.0 * RECEIVER_GASKET_W,
        RECEIVER_GASKET_W,
        10.0,
    )
    .translate(
        0.0,
        -38.0 - RECEIVER_MOUTH_Y / 2.0 - RECEIVER_GASKET_W / 2.0,
        RECEIVER_Z / 2.0 + 11.0,
    );
    let gasket_rear = centered_cube(
        "harness_disposal_receiver_rear_gasket_land",
        RECEIVER_MOUTH_X + 2.0 * RECEIVER_GASKET_W,
        RECEIVER_GASKET_W,
        10.0,
    )
    .translate(
        0.0,
        -38.0 + RECEIVER_MOUTH_Y / 2.0 + RECEIVER_GASKET_W / 2.0,
        RECEIVER_Z / 2.0 + 11.0,
    );
    let gasket_left = centered_cube(
        "harness_disposal_receiver_left_gasket_land",
        RECEIVER_GASKET_W,
        RECEIVER_MOUTH_Y,
        10.0,
    )
    .translate(
        -RECEIVER_MOUTH_X / 2.0 - RECEIVER_GASKET_W / 2.0,
        -38.0,
        RECEIVER_Z / 2.0 + 11.0,
    );
    let gasket_right = centered_cube(
        "harness_disposal_receiver_right_gasket_land",
        RECEIVER_GASKET_W,
        RECEIVER_MOUTH_Y,
        10.0,
    )
    .translate(
        RECEIVER_MOUTH_X / 2.0 + RECEIVER_GASKET_W / 2.0,
        -38.0,
        RECEIVER_Z / 2.0 + 11.0,
    );
    let latch_flags = receiver_latch_flags();
    let bag_retainer = receiver_bag_retainer_ring();
    let handoff_chute = centered_cube(
        "harness_disposal_receiver_sloped_handoff_chute_envelope",
        RECEIVER_X - 116.0,
        56.0,
        42.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 - 14.0, 22.0)
    .rotate(12.0, 0.0, 0.0);

    shell - cavity - mouth
        + lid
        + gasket_front
        + gasket_rear
        + gasket_left
        + gasket_right
        + latch_flags
        + bag_retainer
        + handoff_chute
}

fn receiver_latch_flags() -> Part {
    let mut latches = Part::empty("harness_disposal_receiver_latch_flags");
    for i in 0..RECEIVER_LATCH_COUNT {
        let x = if i % 2 == 0 {
            -RECEIVER_X / 2.0 - 10.0
        } else {
            RECEIVER_X / 2.0 + 10.0
        };
        let y = if i < 2 { -72.0 } else { 72.0 };
        latches = latches
            + centered_cube(format!("harness_disposal_latch_flag_{i}"), 20.0, 50.0, 28.0)
                .translate(x, y, RECEIVER_Z / 2.0 + 20.0);
    }
    latches
}

fn receiver_bag_retainer_ring() -> Part {
    let front = centered_cube(
        "harness_disposal_bag_retainer_front",
        RECEIVER_X - 76.0,
        12.0,
        18.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 + 38.0, RECEIVER_Z / 2.0 - 18.0);
    let rear = centered_cube(
        "harness_disposal_bag_retainer_rear",
        RECEIVER_X - 76.0,
        12.0,
        18.0,
    )
    .translate(0.0, RECEIVER_Y / 2.0 - 38.0, RECEIVER_Z / 2.0 - 18.0);
    let left = centered_cube(
        "harness_disposal_bag_retainer_left",
        12.0,
        RECEIVER_Y - 86.0,
        18.0,
    )
    .translate(-RECEIVER_X / 2.0 + 38.0, 0.0, RECEIVER_Z / 2.0 - 18.0);
    let right = centered_cube(
        "harness_disposal_bag_retainer_right",
        12.0,
        RECEIVER_Y - 86.0,
        18.0,
    )
    .translate(RECEIVER_X / 2.0 - 38.0, 0.0, RECEIVER_Z / 2.0 - 18.0);

    front + rear + left + right
}

fn connector_cap_plug_retention_checks() -> Part {
    let block = centered_cube(
        "harness_disposal_retention_check_block",
        RETENTION_BLOCK_X,
        RETENTION_BLOCK_Y,
        RETENTION_BLOCK_Z,
    );
    let mut subtractors = Part::empty("harness_disposal_retention_check_subtractors");
    for i in 0..CONNECTOR_COUNT {
        subtractors = subtractors
            + centered_cylinder(
                format!("harness_disposal_connector_go_nogo_bore_{i}"),
                CONNECTOR_GO_D / 2.0,
                RETENTION_BLOCK_Z + 5.0,
                28,
            )
            .translate(connector_check_x(i), 64.0, 0.0);
    }
    for i in 0..CAP_WELL_COUNT {
        subtractors = subtractors
            + centered_cylinder(
                format!("harness_disposal_cap_retention_well_{i}"),
                CAP_WELL_D / 2.0,
                RETENTION_BLOCK_Z + 5.0,
                24,
            )
            .translate(small_well_x(i, CAP_WELL_COUNT), 4.0, 0.0);
    }
    for i in 0..PLUG_WELL_COUNT {
        subtractors = subtractors
            + centered_cylinder(
                format!("harness_disposal_plug_retention_well_{i}"),
                PLUG_WELL_D / 2.0,
                RETENTION_BLOCK_Z + 5.0,
                20,
            )
            .translate(small_well_x(i, PLUG_WELL_COUNT), -52.0, 0.0);
    }
    let go_no_go_rails = centered_cube(
        "harness_disposal_connector_go_nogo_label_rail",
        RETENTION_BLOCK_X - 34.0,
        10.0,
        18.0,
    )
    .translate(0.0, 94.0, RETENTION_BLOCK_Z / 2.0 + 9.0)
        + centered_cube(
            "harness_disposal_cap_plug_missing_item_flag_rail",
            RETENTION_BLOCK_X - 34.0,
            10.0,
            18.0,
        )
        .translate(0.0, -94.0, RETENTION_BLOCK_Z / 2.0 + 9.0);

    block - subtractors + go_no_go_rails
}

fn disinfectant_neutralizer_docks() -> Part {
    let dock = centered_cube(
        "harness_disposal_chemistry_dock_tray",
        CHEM_DOCK_X,
        CHEM_DOCK_Y,
        CHEM_DOCK_Z,
    );
    let mut bottle_pockets = Part::empty("harness_disposal_chemistry_bottle_pockets");
    let mut bottle_envelopes = Part::empty("harness_disposal_chemistry_bottle_envelopes");
    for i in 0..CHEM_BOTTLE_COUNT {
        let x = chemistry_bottle_x(i);
        bottle_pockets = bottle_pockets
            + centered_cylinder(
                format!("harness_disposal_chemistry_bottle_pocket_{i}"),
                (CHEM_BOTTLE_D + CHEM_BOTTLE_CLEARANCE) / 2.0,
                CHEM_DOCK_Z + 4.0,
                48,
            )
            .translate(x, 18.0, 0.0);
        bottle_envelopes = bottle_envelopes
            + centered_cylinder(
                format!("harness_disposal_bought_chemistry_container_envelope_{i}"),
                CHEM_BOTTLE_D / 2.0,
                168.0,
                48,
            )
            .translate(x, 18.0, CHEM_DOCK_Z / 2.0 + 84.0);
    }
    let strap_front = centered_cube(
        "harness_disposal_chemistry_front_strap_land",
        CHEM_DOCK_X - 46.0,
        12.0,
        18.0,
    )
    .translate(0.0, -40.0, CHEM_STRAP_Z);
    let strap_rear = centered_cube(
        "harness_disposal_chemistry_rear_strap_land",
        CHEM_DOCK_X - 46.0,
        12.0,
        18.0,
    )
    .translate(0.0, 86.0, CHEM_STRAP_Z);
    let sds_pocket = centered_cube(
        "harness_disposal_chemistry_sds_card_land",
        CHEM_DOCK_X - 86.0,
        28.0,
        8.0,
    )
    .translate(0.0, -CHEM_DOCK_Y / 2.0 - 20.0, CHEM_DOCK_Z / 2.0 + 4.0);

    dock - bottle_pockets + bottle_envelopes + strap_front + strap_rear + sds_pocket
}

fn contact_time_token_lands() -> Part {
    let rail = centered_cube(
        "harness_disposal_contact_time_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_LAND_Z,
    );
    let mut lands = Part::empty("harness_disposal_contact_time_token_lands");
    for i in 0..CONTACT_TOKEN_COUNT {
        lands = lands
            + centered_cube(
                format!("harness_disposal_contact_time_token_land_{i}"),
                TOKEN_LAND_X,
                TOKEN_LAND_Y,
                TOKEN_LAND_Z + 3.0,
            )
            .translate(token_land_x(i), 0.0, 2.0)
            + centered_cube(
                format!("harness_disposal_contact_time_start_stop_notch_{i}"),
                9.0,
                TOKEN_LAND_Y + 12.0,
                TOKEN_LAND_Z + 4.0,
            )
            .translate(token_land_x(i) + TOKEN_LAND_X / 2.0 - 9.0, 0.0, 3.0);
    }
    let quarantine_timer_pad = centered_cube(
        "harness_disposal_contact_time_timer_scan_pad",
        112.0,
        42.0,
        TOKEN_LAND_Z + 6.0,
    )
    .translate(TOKEN_RAIL_X / 2.0 + 82.0, 0.0, 3.0);

    rail + lands + quarantine_timer_pad
}

fn liquid_waste_routing() -> Part {
    let manifold = centered_cube(
        "harness_disposal_liquid_waste_routing_manifold",
        WASTE_ROUTING_X,
        WASTE_ROUTING_Y,
        WASTE_ROUTING_Z,
    );
    let mut ports = Part::empty("harness_disposal_liquid_waste_port_cuts");
    for i in 0..WASTE_PORT_COUNT {
        ports = ports
            + centered_cylinder(
                format!("harness_disposal_liquid_waste_port_{i}"),
                WASTE_PORT_D / 2.0,
                WASTE_ROUTING_Y + 12.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(waste_port_x(i), 0.0, 0.0);
    }
    let waste_bottle_cradle = centered_cylinder(
        "harness_disposal_liquid_waste_bottle_cradle",
        (LIQUID_WASTE_BOTTLE_D + 12.0) / 2.0,
        150.0,
        54,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_ROUTING_X / 2.0 - 78.0, -118.0, 14.0);
    let pinch_guard = centered_cube(
        "harness_disposal_waste_tube_pinch_guard",
        WASTE_ROUTING_X - 96.0,
        22.0,
        34.0,
    )
    .translate(-32.0, WASTE_ROUTING_Y / 2.0 + 18.0, 16.0);
    let drip_tongue = centered_cube(
        "harness_disposal_waste_drip_return_tongue",
        WASTE_ROUTING_X - 40.0,
        38.0,
        10.0,
    )
    .translate(
        0.0,
        -WASTE_ROUTING_Y / 2.0 - 24.0,
        -WASTE_ROUTING_Z / 2.0 + 5.0,
    );

    manifold - ports + waste_bottle_cradle + pinch_guard + drip_tongue
}

fn solid_waste_quarantine_bin() -> Part {
    let bin = centered_cube(
        "harness_disposal_solid_waste_quarantine_bin_shell",
        SOLID_BIN_X,
        SOLID_BIN_Y,
        SOLID_BIN_Z,
    );
    let cavity = centered_cube(
        "harness_disposal_solid_waste_quarantine_bin_cavity",
        SOLID_BIN_X - 42.0,
        SOLID_BIN_Y - 42.0,
        SOLID_BIN_Z - 28.0,
    )
    .translate(0.0, 0.0, 18.0);
    let lid = centered_cube(
        "harness_disposal_solid_waste_quarantine_bin_lid",
        SOLID_BIN_X + 18.0,
        SOLID_BIN_Y + 18.0,
        SOLID_BIN_LID_Z,
    )
    .translate(0.0, 0.0, SOLID_BIN_Z / 2.0 + SOLID_BIN_LID_Z / 2.0 + 7.0);
    let reject_slot = centered_cube(
        "harness_disposal_reject_drop_slot",
        SOLID_BIN_X - 96.0,
        38.0,
        SOLID_BIN_LID_Z + 8.0,
    )
    .translate(0.0, -44.0, SOLID_BIN_Z / 2.0 + SOLID_BIN_LID_Z / 2.0 + 7.0);
    let tamper_tabs = centered_cube(
        "harness_disposal_quarantine_tamper_evident_front_tabs",
        SOLID_BIN_X - 70.0,
        12.0,
        28.0,
    )
    .translate(0.0, -SOLID_BIN_Y / 2.0 - 12.0, SOLID_BIN_Z / 2.0 + 20.0);

    bin - cavity + lid - reject_slot + tamper_tabs
}

fn sample_evidence_vial_wells() -> Part {
    let block = centered_cube(
        "harness_disposal_sample_evidence_vial_block",
        VIAL_BLOCK_X,
        VIAL_BLOCK_Y,
        VIAL_BLOCK_Z,
    );
    let mut wells = Part::empty("harness_disposal_sample_evidence_vial_wells");
    for i in 0..VIAL_WELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("harness_disposal_evidence_vial_well_{i}"),
                VIAL_WELL_D / 2.0,
                VIAL_BLOCK_Z + 5.0,
                28,
            )
            .translate(vial_well_x(i), 22.0, 0.0)
            + centered_cube(
                format!("harness_disposal_vial_chain_of_custody_label_land_{i}"),
                24.0,
                18.0,
                5.0,
            )
            .translate(vial_well_x(i), -44.0, VIAL_BLOCK_Z / 2.0 + 2.5);
    }
    let evidence_seal_rail = centered_cube(
        "harness_disposal_evidence_seal_rail",
        VIAL_BLOCK_X - 26.0,
        10.0,
        16.0,
    )
    .translate(0.0, VIAL_BLOCK_Y / 2.0 + 11.0, VIAL_BLOCK_Z / 2.0 + 8.0);

    block - wells + evidence_seal_rail
}

fn barcode_rfid_run_record_lands() -> Part {
    let mut lands = Part::empty("harness_disposal_traceability_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("harness_disposal_barcode_land_{i}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                5.0,
            )
            .translate(barcode_land_x(i), -DECK_Y / 2.0 + 112.0, DECK_Z + 2.5);
    }
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("harness_disposal_rfid_land_{i}"),
                RFID_LAND_X,
                RFID_LAND_Y,
                6.0,
            )
            .translate(rfid_land_x(i), DECK_Y / 2.0 - 96.0, DECK_Z + 3.0);
    }
    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("harness_disposal_run_record_land_{i}"),
                154.0,
                42.0,
                6.0,
            )
            .translate(-420.0 + i as f64 * 420.0, -18.0, DECK_Z + 3.0);
    }
    lands
}

fn clean_used_reject_segregation() -> Part {
    let left_divider_x = (CLEAN_ZONE_X + USED_ZONE_X) / 2.0 + 64.0;
    let right_divider_x = (USED_ZONE_X + REJECT_ZONE_X) / 2.0 + 54.0;
    let left = centered_cube(
        "harness_disposal_clean_to_used_divider",
        SEGREGATION_WALL_W,
        DECK_Y - 112.0,
        SEGREGATION_WALL_Z,
    )
    .translate(left_divider_x, 18.0, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let right = centered_cube(
        "harness_disposal_used_to_reject_divider",
        SEGREGATION_WALL_W,
        DECK_Y - 112.0,
        SEGREGATION_WALL_Z,
    )
    .translate(right_divider_x, 18.0, DECK_Z + SEGREGATION_WALL_Z / 2.0);
    let clean_threshold = centered_cube(
        "harness_disposal_clean_lane_threshold",
        CLEAN_LANE_W,
        14.0,
        44.0,
    )
    .translate(CLEAN_ZONE_X, -86.0, DECK_Z + 22.0);
    let used_threshold = centered_cube(
        "harness_disposal_used_lane_threshold",
        USED_LANE_W,
        14.0,
        44.0,
    )
    .translate(USED_ZONE_X, -86.0, DECK_Z + 22.0);
    let reject_threshold = centered_cube(
        "harness_disposal_reject_lane_threshold",
        REJECT_LANE_W,
        14.0,
        44.0,
    )
    .translate(REJECT_ZONE_X, -86.0, DECK_Z + 22.0);
    let interlock_tabs = centered_cube(
        "harness_disposal_material_state_interlock_tabs",
        DECK_X - 150.0,
        18.0,
        26.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 136.0, DECK_Z + 13.0);

    left + right + clean_threshold + used_threshold + reject_threshold + interlock_tabs
}

fn scanner_bridge() -> Part {
    let left_post = centered_cube(
        "harness_disposal_scanner_bridge_left_post",
        34.0,
        SCANNER_BRIDGE_POST_Y,
        SCANNER_UNDERSIDE_Z,
    )
    .translate(
        -SCANNER_BRIDGE_SPAN_X / 2.0,
        -34.0,
        DECK_Z + SCANNER_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "harness_disposal_scanner_bridge_right_post",
        34.0,
        SCANNER_BRIDGE_POST_Y,
        SCANNER_UNDERSIDE_Z,
    )
    .translate(
        SCANNER_BRIDGE_SPAN_X / 2.0,
        -34.0,
        DECK_Z + SCANNER_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "harness_disposal_scanner_bridge_beam",
        SCANNER_BRIDGE_SPAN_X + 86.0,
        SCANNER_BRIDGE_POST_Y,
        SCANNER_BEAM_Z,
    )
    .translate(
        0.0,
        -34.0,
        DECK_Z + SCANNER_UNDERSIDE_Z + SCANNER_BEAM_Z / 2.0,
    );
    let mut heads = Part::empty("harness_disposal_scanner_heads");
    for i in 0..SCANNER_HEAD_COUNT {
        heads = heads
            + centered_cube(
                format!("harness_disposal_barcode_rfid_scanner_head_{i}"),
                78.0,
                36.0,
                24.0,
            )
            .translate(
                scanner_head_x(i),
                -72.0,
                DECK_Z + SCANNER_UNDERSIDE_Z - 16.0,
            );
    }
    let cable_tray = centered_cube(
        "harness_disposal_scanner_bridge_cable_tray",
        SCANNER_BRIDGE_SPAN_X - 80.0,
        22.0,
        18.0,
    )
    .translate(
        0.0,
        2.0,
        DECK_Z + SCANNER_UNDERSIDE_Z + SCANNER_BEAM_Z + 12.0,
    );

    left_post + right_post + beam + heads + cable_tray
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "harness_disposal_front_robot_handoff_keepout",
        DECK_X - 120.0,
        FRONT_ROBOT_KEEP_OUT,
        38.0,
    )
    .translate(0.0, -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT / 2.0, 19.0);
    let rear_service = centered_cube(
        "harness_disposal_rear_service_keepout",
        DECK_X - 160.0,
        REAR_SERVICE_KEEP_OUT,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT / 2.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let chemistry_swap = centered_cube(
        "harness_disposal_chemistry_swap_keepout",
        CHEM_DOCK_X + 80.0,
        CHEM_SWAP_KEEP_OUT,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        REJECT_ZONE_X,
        REAR_ROW_Y + CHEM_DOCK_Y / 2.0 + CHEM_SWAP_KEEP_OUT / 2.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let waste_drawer = centered_cube(
        "harness_disposal_waste_drawer_pull_keepout",
        SOLID_BIN_X + 92.0,
        WASTE_DRAWER_KEEP_OUT,
        SOLID_BIN_Z + 80.0,
    )
    .translate(
        REJECT_ZONE_X,
        FRONT_ROW_Y - SOLID_BIN_Y / 2.0 - WASTE_DRAWER_KEEP_OUT / 2.0,
        (SOLID_BIN_Z + 80.0) / 2.0,
    );

    front_robot + rear_service + chemistry_swap + waste_drawer
}

fn connector_check_x(index: usize) -> f64 {
    let pitch = 27.0;
    -((CONNECTOR_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn small_well_x(index: usize, count: usize) -> f64 {
    let pitch = 13.0;
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn chemistry_bottle_x(index: usize) -> f64 {
    let pitch = 164.0;
    -((CHEM_BOTTLE_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn token_land_x(index: usize) -> f64 {
    let pitch = 82.0;
    -((CONTACT_TOKEN_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn waste_port_x(index: usize) -> f64 {
    let pitch = 92.0;
    -((WASTE_PORT_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn vial_well_x(index: usize) -> f64 {
    let pitch = 28.0;
    -((VIAL_WELL_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn barcode_land_x(index: usize) -> f64 {
    let pitch = 86.0;
    -((BARCODE_LANDS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rfid_land_x(index: usize) -> f64 {
    let pitch = 92.0;
    -((RFID_LANDS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn scanner_head_x(index: usize) -> f64 {
    let pitch = 300.0;
    -((SCANNER_HEAD_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct PartSpec {
    path: &'static str,
    min_size: [f64; 3],
}

#[cfg(test)]
fn output_specs() -> [PartSpec; 13] {
    [
        PartSpec {
            path: OUTPUTS[0],
            min_size: [DECK_X, DECK_Y, RIM_Z],
        },
        PartSpec {
            path: OUTPUTS[1],
            min_size: [RECEIVER_X, RECEIVER_Y, RECEIVER_Z],
        },
        PartSpec {
            path: OUTPUTS[2],
            min_size: [RETENTION_BLOCK_X, RETENTION_BLOCK_Y, RETENTION_BLOCK_Z],
        },
        PartSpec {
            path: OUTPUTS[3],
            min_size: [CHEM_DOCK_X, CHEM_DOCK_Y, CHEM_STRAP_Z],
        },
        PartSpec {
            path: OUTPUTS[4],
            min_size: [TOKEN_RAIL_X, TOKEN_RAIL_Y, TOKEN_LAND_Z],
        },
        PartSpec {
            path: OUTPUTS[5],
            min_size: [WASTE_ROUTING_X, WASTE_ROUTING_Y, WASTE_ROUTING_Z],
        },
        PartSpec {
            path: OUTPUTS[6],
            min_size: [SOLID_BIN_X, SOLID_BIN_Y, SOLID_BIN_Z],
        },
        PartSpec {
            path: OUTPUTS[7],
            min_size: [VIAL_BLOCK_X, VIAL_BLOCK_Y, VIAL_BLOCK_Z],
        },
        PartSpec {
            path: OUTPUTS[8],
            min_size: [DECK_X - 180.0, LABEL_LAND_Y, 6.0],
        },
        PartSpec {
            path: OUTPUTS[9],
            min_size: [
                CLEAN_LANE_W + USED_LANE_W + REJECT_LANE_W,
                DECK_Y - 112.0,
                SEGREGATION_WALL_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[10],
            min_size: [
                SCANNER_BRIDGE_SPAN_X,
                SCANNER_BRIDGE_POST_Y,
                SCANNER_UNDERSIDE_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[11],
            min_size: [
                DECK_X,
                DECK_Y + FRONT_ROBOT_KEEP_OUT + REAR_SERVICE_KEEP_OUT,
                SERVICE_KEEP_OUT_Z,
            ],
        },
        PartSpec {
            path: OUTPUTS[12],
            min_size: [
                DECK_X,
                DECK_Y + FRONT_ROBOT_KEEP_OUT,
                SCANNER_UNDERSIDE_Z + SCANNER_BEAM_Z,
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_contract_is_unique_scoped_and_has_assembly() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        let specs = output_specs();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(specs.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_single_use_harness_disposal_deactivation_station_")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
        assert!(specs.iter().all(|spec| spec.path.ends_with(".stl")));
        assert!(specs.iter().all(|spec| spec.min_size[0] > 0.0
            && spec.min_size[1] > 0.0
            && spec.min_size[2] > 0.0));
    }

    #[test]
    fn station_dimensions_keep_major_modules_on_deck() {
        assert!(RECEIVER_X < USED_LANE_W + 40.0);
        assert!(RETENTION_BLOCK_X < CLEAN_LANE_W + 120.0);
        assert!(SOLID_BIN_X < REJECT_LANE_W + 100.0);
        assert!(REAR_ROW_Y + RECEIVER_Y / 2.0 < DECK_Y / 2.0 - RIM_W);
        assert!(FRONT_ROW_Y - SOLID_BIN_Y / 2.0 > -DECK_Y / 2.0 + RIM_W);
        assert!(SCANNER_UNDERSIDE_Z > DECK_Z + RECEIVER_Z + 38.0);
    }

    #[test]
    fn retention_checks_cover_every_connector_with_caps_and_plugs() {
        assert_eq!(CONNECTOR_COUNT, 12);
        assert_eq!(CAP_WELL_COUNT, CONNECTOR_COUNT * 2);
        assert_eq!(PLUG_WELL_COUNT, CONNECTOR_COUNT * 2);
        assert!(CONNECTOR_GO_D > CAP_WELL_D);
        assert!(CAP_WELL_D > PLUG_WELL_D);
        assert!(connector_check_x(0).abs() + CONNECTOR_GO_D / 2.0 < RETENTION_BLOCK_X / 2.0);
        assert!(
            connector_check_x(CONNECTOR_COUNT - 1).abs() + CONNECTOR_GO_D / 2.0
                < RETENTION_BLOCK_X / 2.0
        );
        assert!(small_well_x(0, CAP_WELL_COUNT).abs() + CAP_WELL_D / 2.0 < RETENTION_BLOCK_X / 2.0);
    }

    #[test]
    fn chemistry_and_contact_time_are_packaging_placeholders() {
        assert_eq!(CHEM_BOTTLE_COUNT, 2);
        assert!(CHEM_BOTTLE_D + CHEM_BOTTLE_CLEARANCE < CHEM_DOCK_Y);
        assert!(
            chemistry_bottle_x(0).abs() + (CHEM_BOTTLE_D + CHEM_BOTTLE_CLEARANCE) / 2.0
                < CHEM_DOCK_X / 2.0
        );
        assert_eq!(CONTACT_TOKEN_COUNT, 8);
        assert!(token_land_x(0).abs() + TOKEN_LAND_X / 2.0 < TOKEN_RAIL_X / 2.0);
        assert!(
            token_land_x(CONTACT_TOKEN_COUNT - 1).abs() + TOKEN_LAND_X / 2.0 < TOKEN_RAIL_X / 2.0
        );
    }

    #[test]
    fn waste_evidence_and_traceability_capacity_is_explicit() {
        assert_eq!(WASTE_PORT_COUNT, 5);
        assert!(waste_port_x(0).abs() + WASTE_PORT_D / 2.0 < WASTE_ROUTING_X / 2.0);
        assert_eq!(VIAL_WELL_COUNT, 10);
        assert!(vial_well_x(VIAL_WELL_COUNT - 1).abs() + VIAL_WELL_D / 2.0 < VIAL_BLOCK_X / 2.0);
        assert!(BARCODE_LANDS >= CONNECTOR_COUNT);
        assert!(RFID_LANDS >= RUN_RECORD_LANDS * 2);
        assert!(barcode_land_x(0).abs() + LABEL_LAND_X / 2.0 < DECK_X / 2.0 - RIM_W);
    }

    #[test]
    fn keepouts_cover_robot_service_chemistry_and_waste_access() {
        assert!(FRONT_ROBOT_KEEP_OUT >= 400.0);
        assert!(REAR_SERVICE_KEEP_OUT >= 240.0);
        assert!(CHEM_SWAP_KEEP_OUT >= CHEM_BOTTLE_D * 2.0);
        assert!(WASTE_DRAWER_KEEP_OUT >= SOLID_BIN_Y);
        assert!(SERVICE_KEEP_OUT_Z > CHEM_STRAP_Z);
    }
}
