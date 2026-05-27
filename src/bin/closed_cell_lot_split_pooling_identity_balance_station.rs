use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-lot split/pooling identity balance validation station.
//
// This generator packages a mechanical validation fixture for checking source
// lot identity, equal split/pool surrogate routing, mass/volume reconciliation,
// retain aliquot custody, environmental logger placement, and evidence capture
// around a closed cell-lot handling cassette. It is geometry for mechanical
// validation packaging only. It is not a biological SOP, sterile-process claim,
// GMP release decision, clinical acceptance criterion, or validated cell
// processing instruction.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_cell_lot_split_pooling_identity_balance_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_lot_split_pooling_identity_balance_station_containment_deck.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_source_lot_bag_vial_nests.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_split_pool_manifold_surrogate.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_equalized_branch_volume_witnesses.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_identity_barcode_rfid_custody_lands.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_mass_volume_balance_pads.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_retain_aliquot_wells.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_temperature_logger_pockets.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_bubble_dead_volume_windows.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_release_hold_reject_gates.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_camera_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_cell_lot_split_pooling_identity_balance_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 25] = [
    "source_lot_bag_nests",
    "source_lot_vial_nests",
    "split_pool_manifold_surrogate",
    "split_branch_ports",
    "pool_header_ports",
    "equalized_branch_volume_witnesses",
    "matched_volume_token_lands",
    "identity_barcode_lands",
    "identity_rfid_custody_lands",
    "chain_of_custody_tamper_lands",
    "mass_balance_pads",
    "volume_balance_pads",
    "retain_aliquot_wells",
    "retain_cap_parking_lands",
    "temperature_logger_pockets",
    "temperature_probe_channel",
    "bubble_witness_windows",
    "dead_volume_windows",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 5] = [
    "biological_standard_operating_procedure",
    "sterile_process_claim",
    "gmp_release_decision",
    "clinical_acceptance_criterion",
    "validated_cell_processing_instruction",
];

const DECK_X: f64 = 1360.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 52.0;
const SUMP_X: f64 = 1180.0;
const SUMP_Y: f64 = 715.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const SOURCE_NEST_X: f64 = 400.0;
const SOURCE_NEST_Y: f64 = 250.0;
const SOURCE_NEST_Z: f64 = 48.0;
const SOURCE_NEST_POS: (f64, f64) = (-435.0, 265.0);
const SOURCE_BAG_NESTS: usize = 3;
const SOURCE_VIAL_NESTS: usize = 6;
const BAG_NEST_X: f64 = 92.0;
const BAG_NEST_Y: f64 = 128.0;
const BAG_NEST_PITCH_X: f64 = 112.0;
const BAG_NECK_D: f64 = 22.0;
const VIAL_NEST_D: f64 = 25.0;
const VIAL_NEST_PITCH_X: f64 = 52.0;

const MANIFOLD_X: f64 = 490.0;
const MANIFOLD_Y: f64 = 190.0;
const MANIFOLD_Z: f64 = 46.0;
const MANIFOLD_POS: (f64, f64) = (95.0, 260.0);
const BRANCH_COUNT: usize = 6;
const BRANCH_PITCH: f64 = 64.0;
const BRANCH_CHANNEL_D: f64 = 8.0;
const SOURCE_PORTS: usize = 3;
const POOL_PORTS: usize = 3;
const PORT_COLLAR_D: f64 = 29.0;

const WITNESS_X: f64 = 560.0;
const WITNESS_Y: f64 = 145.0;
const WITNESS_Z: f64 = 30.0;
const WITNESS_POS: (f64, f64) = (140.0, 50.0);
const WITNESS_LANES: usize = BRANCH_COUNT;
const WITNESS_LANE_PITCH_X: f64 = 78.0;
const WITNESS_WINDOW_D: f64 = 24.0;
const VOLUME_TOKEN_D: f64 = 18.0;

const CUSTODY_X: f64 = 300.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (495.0, 260.0);
const BARCODE_LANDS: usize = BRANCH_COUNT + SOURCE_BAG_NESTS;
const RFID_LANDS: usize = BRANCH_COUNT;
const TAMPER_LANDS: usize = 5;

const BALANCE_X: f64 = 430.0;
const BALANCE_Y: f64 = 160.0;
const BALANCE_Z: f64 = 34.0;
const BALANCE_POS: (f64, f64) = (-385.0, 40.0);
const MASS_BALANCE_PADS: usize = 4;
const VOLUME_BALANCE_PADS: usize = 4;
const BALANCE_PAD_D: f64 = 46.0;
const BALANCE_PAD_PITCH_X: f64 = 72.0;

const RETAIN_X: f64 = 365.0;
const RETAIN_Y: f64 = 170.0;
const RETAIN_Z: f64 = 42.0;
const RETAIN_POS: (f64, f64) = (-430.0, -245.0);
const RETAIN_WELLS: usize = 12;
const RETAIN_WELL_D: f64 = 24.0;
const RETAIN_COLS: usize = 6;
const RETAIN_PITCH_X: f64 = 45.0;
const RETAIN_PITCH_Y: f64 = 52.0;

const LOGGER_X: f64 = 270.0;
const LOGGER_Y: f64 = 135.0;
const LOGGER_Z: f64 = 36.0;
const LOGGER_POS: (f64, f64) = (500.0, 40.0);
const TEMPERATURE_LOGGERS: usize = 4;
const LOGGER_POCKET_X: f64 = 52.0;
const LOGGER_POCKET_Y: f64 = 82.0;
const LOGGER_PITCH_X: f64 = 61.0;

const BUBBLE_X: f64 = 520.0;
const BUBBLE_Y: f64 = 145.0;
const BUBBLE_Z: f64 = 28.0;
const BUBBLE_POS: (f64, f64) = (110.0, -225.0);
const BUBBLE_WINDOWS: usize = 8;
const DEAD_VOLUME_WINDOWS: usize = 6;
const BUBBLE_WINDOW_D: f64 = 25.0;
const DEAD_VOLUME_WINDOW_X: f64 = 42.0;
const WINDOW_PITCH_X: f64 = 58.0;

const GATE_X: f64 = 425.0;
const GATE_Y: f64 = 95.0;
const GATE_Z: f64 = 36.0;
const GATE_POS: (f64, f64) = (400.0, -370.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 980.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 210.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (10.0, -30.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const ROBOT_KEEPOUT_X: f64 = 1180.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 82.0;
const SERVICE_KEEPOUT_X: f64 = 104.0;
const SERVICE_KEEPOUT_Y: f64 = 700.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 300.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(source_lot_bag_vial_nests(), OUTPUTS[1]);
    write_part(split_pool_manifold_surrogate(), OUTPUTS[2]);
    write_part(equalized_branch_volume_witnesses(), OUTPUTS[3]);
    write_part(identity_barcode_rfid_custody_lands(), OUTPUTS[4]);
    write_part(mass_volume_balance_pads(), OUTPUTS[5]);
    write_part(retain_aliquot_wells(), OUTPUTS[6]);
    write_part(temperature_logger_pockets(), OUTPUTS[7]);
    write_part(bubble_dead_volume_windows(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge_robot_service_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed cell-lot split/pooling identity balance station: {:.0}mm x {:.0}mm contained deck, {} source bag nests, {} source vial nests, {} equalized branch witnesses.",
        DECK_X, DECK_Y, SOURCE_BAG_NESTS, SOURCE_VIAL_NESTS, WITNESS_LANES
    );
    println!(
        "Custody and balance packaging: {} barcode lands, {} RFID lands, {} mass pads, {} volume pads, {} retain aliquot wells, {} temperature logger pockets.",
        BARCODE_LANDS,
        RFID_LANDS,
        MASS_BALANCE_PADS,
        VOLUME_BALANCE_PADS,
        RETAIN_WELLS,
        TEMPERATURE_LOGGERS
    );
    println!(
        "Evidence features: {} bubble windows, {} dead-volume windows, release/hold/reject gates, {} camera mounts, top service clearance {:.0}mm.",
        BUBBLE_WINDOWS,
        DEAD_VOLUME_WINDOWS,
        CAMERA_MOUNTS,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_cell_lot_identity_balance_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_cell_lot_identity_balance_station_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_cell_lot_identity_balance_station_captured_drain",
        DRAIN_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 78.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_cell_lot_identity_balance_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_center_z());
    let right = centered_cube(
        "closed_cell_lot_identity_balance_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_cell_lot_identity_balance_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_cell_lot_identity_balance_station_front_low_containment_rim",
        DECK_X - 120.0,
        RIM_W,
        RIM_Z * 0.70,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.35,
    );

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_cell_lot_identity_balance_station_deck_datums");

    for i in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 86.0 + i as f64 * ((DECK_X - 172.0) / 9.0);
        let y = if i % 2 == 0 {
            DECK_Y / 2.0 - 72.0
        } else {
            -DECK_Y / 2.0 + 72.0
        };
        let pad = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_datum_pad_{i}"),
            12.0,
            5.0,
            36,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_datum_bore_{i}"),
            3.0,
            8.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }

    datums
}

fn station_landing_pockets() -> Part {
    landing_pocket(
        "source_lot_nest_land",
        SOURCE_NEST_X,
        SOURCE_NEST_Y,
        SOURCE_NEST_POS,
    ) + landing_pocket(
        "split_pool_manifold_land",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_POS,
    ) + landing_pocket(
        "equalized_branch_witness_land",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_POS,
    ) + landing_pocket("custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pocket("balance_pad_land", BALANCE_X, BALANCE_Y, BALANCE_POS)
        + landing_pocket("retain_aliquot_land", RETAIN_X, RETAIN_Y, RETAIN_POS)
        + landing_pocket("temperature_logger_land", LOGGER_X, LOGGER_Y, LOGGER_POS)
        + landing_pocket("bubble_dead_volume_land", BUBBLE_X, BUBBLE_Y, BUBBLE_POS)
        + landing_pocket("disposition_gate_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("closed_cell_lot_identity_balance_station_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn source_lot_bag_vial_nests() -> Part {
    let base = centered_cube(
        "closed_cell_lot_identity_balance_station_source_lot_nest_base",
        SOURCE_NEST_X,
        SOURCE_NEST_Y,
        SOURCE_NEST_Z,
    );
    let bag_lane_label = centered_cube(
        "closed_cell_lot_identity_balance_station_source_lot_bag_lane_label_land",
        SOURCE_NEST_X - 36.0,
        18.0,
        4.0,
    )
    .translate(0.0, SOURCE_NEST_Y / 2.0 - 20.0, SOURCE_NEST_Z / 2.0 + 4.0);
    let vial_lane_label = centered_cube(
        "closed_cell_lot_identity_balance_station_source_lot_vial_lane_label_land",
        SOURCE_NEST_X - 48.0,
        18.0,
        4.0,
    )
    .translate(0.0, -SOURCE_NEST_Y / 2.0 + 20.0, SOURCE_NEST_Z / 2.0 + 4.0);

    let mut nests = base + bag_lane_label + vial_lane_label;

    for i in 0..SOURCE_BAG_NESTS {
        let x = port_x(i, SOURCE_BAG_NESTS, BAG_NEST_PITCH_X);
        let bag_recess = centered_cube(
            format!("closed_cell_lot_identity_balance_station_source_lot_bag_nest_recess_{i}"),
            BAG_NEST_X,
            BAG_NEST_Y,
            25.0,
        )
        .translate(x, 28.0, SOURCE_NEST_Z / 2.0 - 6.0);
        let saddle_a = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_bag_saddle_a_{i}"),
            10.0,
            BAG_NEST_Y - 18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 24.0, 28.0, SOURCE_NEST_Z / 2.0 + 11.0);
        let saddle_b = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_bag_saddle_b_{i}"),
            10.0,
            BAG_NEST_Y - 18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 24.0, 28.0, SOURCE_NEST_Z / 2.0 + 11.0);
        let neck_land = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_bag_neck_land_{i}"),
            BAG_NECK_D / 2.0 + 5.0,
            8.0,
            32,
        )
        .translate(x, -38.0, SOURCE_NEST_Z / 2.0 + 5.0);
        let neck_bore = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_bag_neck_bore_{i}"),
            BAG_NECK_D / 2.0,
            14.0,
            28,
        )
        .translate(x, -38.0, SOURCE_NEST_Z / 2.0 + 5.0);
        nests = nests - bag_recess + saddle_a + saddle_b + neck_land - neck_bore;
    }

    for i in 0..SOURCE_VIAL_NESTS {
        let x = port_x(i, SOURCE_VIAL_NESTS, VIAL_NEST_PITCH_X);
        let vial_bore = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_vial_nest_bore_{i}"),
            VIAL_NEST_D / 2.0,
            SOURCE_NEST_Z + 8.0,
            36,
        )
        .translate(x, -76.0, 8.0);
        let vial_lip = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_source_lot_vial_nest_lip_{i}"),
            VIAL_NEST_D / 2.0 + 5.0,
            5.0,
            36,
        )
        .translate(x, -76.0, SOURCE_NEST_Z / 2.0 + 3.0);
        let identity_tick = centered_cube(
            format!("closed_cell_lot_identity_balance_station_source_lot_vial_identity_tick_{i}"),
            20.0,
            5.0,
            4.0,
        )
        .translate(x, -45.0, SOURCE_NEST_Z / 2.0 + 4.0);
        nests = nests - vial_bore + vial_lip + identity_tick;
    }

    nests
}

fn split_pool_manifold_surrogate() -> Part {
    let block = centered_cube(
        "closed_cell_lot_identity_balance_station_split_pool_manifold_surrogate_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let source_header = centered_cylinder(
        "closed_cell_lot_identity_balance_station_source_lot_split_header_surrogate",
        BRANCH_CHANNEL_D / 2.0,
        MANIFOLD_X - 58.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 45.0, MANIFOLD_Z / 2.0 + 7.0);
    let pool_header = centered_cylinder(
        "closed_cell_lot_identity_balance_station_pool_header_surrogate",
        BRANCH_CHANNEL_D / 2.0,
        MANIFOLD_X - 58.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -45.0, MANIFOLD_Z / 2.0 + 7.0);
    let equalization_key = centered_cube(
        "closed_cell_lot_identity_balance_station_split_pool_equalization_key_bar",
        MANIFOLD_X - 74.0,
        12.0,
        12.0,
    )
    .translate(0.0, 0.0, MANIFOLD_Z / 2.0 + 10.0);

    let mut manifold = block + source_header + pool_header + equalization_key;

    for i in 0..BRANCH_COUNT {
        let x = branch_x(i);
        let branch = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_equal_split_branch_surrogate_{i}"),
            BRANCH_CHANNEL_D / 2.0,
            96.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, MANIFOLD_Z / 2.0 + 7.0);
        let branch_port = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_split_branch_port_collar_{i}"),
            PORT_COLLAR_D / 2.0,
            8.0,
            32,
        )
        .translate(x, 78.0, MANIFOLD_Z / 2.0 + 5.0);
        let pool_port = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_pool_header_port_collar_{i}"),
            PORT_COLLAR_D / 2.0,
            8.0,
            32,
        )
        .translate(x, -78.0, MANIFOLD_Z / 2.0 + 5.0);
        let route_key = centered_cube(
            format!("closed_cell_lot_identity_balance_station_manifold_branch_identity_key_{i}"),
            18.0,
            18.0,
            10.0,
        )
        .translate(x, 0.0, MANIFOLD_Z / 2.0 + 15.0);
        manifold = manifold + branch + branch_port + pool_port + route_key;
    }

    for i in 0..SOURCE_PORTS {
        let port = end_port("source", i, 1.0);
        manifold = manifold + port.0 - port.1;
    }
    for i in 0..POOL_PORTS {
        let port = end_port("pool", i, -1.0);
        manifold = manifold + port.0 - port.1;
    }

    manifold
}

fn end_port(prefix: &str, index: usize, y_sign: f64) -> (Part, Part) {
    let x = port_x(index, 3, 58.0);
    let y = y_sign * (MANIFOLD_Y / 2.0 + 8.0);
    let boss = centered_cylinder(
        format!("closed_cell_lot_identity_balance_station_{prefix}_external_port_boss_{index}"),
        16.0,
        28.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, y, MANIFOLD_Z / 2.0 + 5.0);
    let bore = centered_cylinder(
        format!("closed_cell_lot_identity_balance_station_{prefix}_external_port_bore_{index}"),
        4.0,
        36.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, y, MANIFOLD_Z / 2.0 + 5.0);
    (boss, bore)
}

fn equalized_branch_volume_witnesses() -> Part {
    let frame = centered_cube(
        "closed_cell_lot_identity_balance_station_equalized_branch_volume_witness_frame",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let backlight_slot = centered_cube(
        "closed_cell_lot_identity_balance_station_equalized_witness_backlight_slot",
        WITNESS_X - 62.0,
        18.0,
        16.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);

    let mut witnesses = frame - backlight_slot;

    for i in 0..WITNESS_LANES {
        let x = witness_x(i);
        let lane_a = centered_cube(
            format!("closed_cell_lot_identity_balance_station_equalized_branch_lane_a_{i}"),
            12.0,
            WITNESS_Y - 34.0,
            8.0,
        )
        .translate(x - 12.0, 0.0, WITNESS_Z / 2.0 + 5.0);
        let lane_b = centered_cube(
            format!("closed_cell_lot_identity_balance_station_equalized_branch_lane_b_{i}"),
            12.0,
            WITNESS_Y - 34.0,
            8.0,
        )
        .translate(x + 12.0, 0.0, WITNESS_Z / 2.0 + 5.0);
        let turn_window = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_branch_volume_witness_window_{i}"),
            WITNESS_WINDOW_D / 2.0,
            WITNESS_Z + 8.0,
            36,
        )
        .translate(x, 34.0, 5.0);
        let volume_token = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_matched_volume_token_land_{i}"),
            VOLUME_TOKEN_D / 2.0,
            5.0,
            28,
        )
        .translate(x, -48.0, WITNESS_Z / 2.0 + 4.0);
        let graduation = centered_cube(
            format!("closed_cell_lot_identity_balance_station_equalized_branch_graduation_{i}"),
            34.0,
            4.0,
            5.0,
        )
        .translate(x, -8.0, WITNESS_Z / 2.0 + 4.0);
        witnesses = witnesses + lane_a + lane_b - turn_window + volume_token + graduation;
    }

    witnesses
}

fn identity_barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_cell_lot_identity_balance_station_identity_barcode_rfid_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut custody = panel;

    for i in 0..BARCODE_LANDS {
        let land = centered_cube(
            format!("closed_cell_lot_identity_balance_station_identity_barcode_land_{i}"),
            36.0,
            19.0,
            4.0,
        )
        .translate(port_x(i, BARCODE_LANDS, 34.0), 32.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..RFID_LANDS {
        let land = centered_cube(
            format!("closed_cell_lot_identity_balance_station_identity_rfid_custody_land_{i}"),
            38.0,
            30.0,
            4.0,
        )
        .translate(port_x(i, RFID_LANDS, 44.0), -16.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..TAMPER_LANDS {
        let seal = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_chain_of_custody_tamper_land_{i}"),
            8.0,
            4.0,
            24,
        )
        .translate(-120.0 + i as f64 * 60.0, -48.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + seal;
    }

    custody
}

fn mass_volume_balance_pads() -> Part {
    let base = centered_cube(
        "closed_cell_lot_identity_balance_station_mass_volume_balance_pad_base",
        BALANCE_X,
        BALANCE_Y,
        BALANCE_Z,
    );
    let divider = centered_cube(
        "closed_cell_lot_identity_balance_station_mass_volume_balance_divider",
        BALANCE_X - 28.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, BALANCE_Z / 2.0 + 7.0);

    let mut pads = base + divider;

    for i in 0..MASS_BALANCE_PADS {
        let x = port_x(i, MASS_BALANCE_PADS, BALANCE_PAD_PITCH_X);
        let pad = balance_pad("mass", i, x, 42.0);
        pads = pads + pad.0 - pad.1 + pad.2;
    }
    for i in 0..VOLUME_BALANCE_PADS {
        let x = port_x(i, VOLUME_BALANCE_PADS, BALANCE_PAD_PITCH_X);
        let pad = balance_pad("volume", i, x, -42.0);
        pads = pads + pad.0 - pad.1 + pad.2;
    }

    let reconciliation_bar = centered_cube(
        "closed_cell_lot_identity_balance_station_mass_volume_reconciliation_bar",
        BALANCE_X - 70.0,
        11.0,
        5.0,
    )
    .translate(0.0, -4.0, BALANCE_Z / 2.0 + 5.0);

    pads + reconciliation_bar
}

fn balance_pad(prefix: &str, index: usize, x: f64, y: f64) -> (Part, Part, Part) {
    let land = centered_cylinder(
        format!("closed_cell_lot_identity_balance_station_{prefix}_balance_pad_land_{index}"),
        BALANCE_PAD_D / 2.0,
        8.0,
        40,
    )
    .translate(x, y, BALANCE_Z / 2.0 + 4.0);
    let recess = centered_cylinder(
        format!("closed_cell_lot_identity_balance_station_{prefix}_balance_sensor_recess_{index}"),
        BALANCE_PAD_D / 2.0 - 6.0,
        12.0,
        36,
    )
    .translate(x, y, BALANCE_Z / 2.0 + 3.0);
    let label = centered_cube(
        format!("closed_cell_lot_identity_balance_station_{prefix}_balance_label_land_{index}"),
        34.0,
        6.0,
        4.0,
    )
    .translate(x, y + 31.0, BALANCE_Z / 2.0 + 4.0);
    (land, recess, label)
}

fn retain_aliquot_wells() -> Part {
    let bank = centered_cube(
        "closed_cell_lot_identity_balance_station_retain_aliquot_well_bank",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    );
    let cap_rail = centered_cube(
        "closed_cell_lot_identity_balance_station_retain_cap_parking_rail",
        RETAIN_X - 42.0,
        26.0,
        8.0,
    )
    .translate(0.0, RETAIN_Y / 2.0 - 28.0, RETAIN_Z / 2.0 + 6.0);

    let mut retain = bank + cap_rail;

    for i in 0..RETAIN_WELLS {
        let (x, y) = retain_well_center(i);
        let well = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_retain_aliquot_well_{i}"),
            RETAIN_WELL_D / 2.0,
            RETAIN_Z + 8.0,
            36,
        )
        .translate(x, y, 8.0);
        let lip = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_retain_aliquot_well_lip_{i}"),
            RETAIN_WELL_D / 2.0 + 4.0,
            5.0,
            36,
        )
        .translate(x, y, RETAIN_Z / 2.0 + 3.0);
        let cap_land = centered_cube(
            format!("closed_cell_lot_identity_balance_station_retain_cap_parking_land_{i}"),
            22.0,
            16.0,
            4.0,
        )
        .translate(x, RETAIN_Y / 2.0 - 28.0, RETAIN_Z / 2.0 + 13.0);
        retain = retain - well + lip + cap_land;
    }

    retain
}

fn temperature_logger_pockets() -> Part {
    let base = centered_cube(
        "closed_cell_lot_identity_balance_station_temperature_logger_pocket_base",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let probe_channel = centered_cylinder(
        "closed_cell_lot_identity_balance_station_temperature_probe_channel",
        4.2,
        LOGGER_X - 34.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -LOGGER_Y / 2.0 + 28.0, LOGGER_Z / 2.0 + 4.0);

    let mut loggers = base + probe_channel;

    for i in 0..TEMPERATURE_LOGGERS {
        let x = port_x(i, TEMPERATURE_LOGGERS, LOGGER_PITCH_X);
        let pocket = centered_cube(
            format!("closed_cell_lot_identity_balance_station_temperature_logger_pocket_{i}"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            18.0,
        )
        .translate(x, 18.0, LOGGER_Z / 2.0 - 5.0);
        let latch = centered_cube(
            format!("closed_cell_lot_identity_balance_station_temperature_logger_latch_land_{i}"),
            LOGGER_POCKET_X - 10.0,
            8.0,
            5.0,
        )
        .translate(x, -34.0, LOGGER_Z / 2.0 + 4.0);
        loggers = loggers - pocket + latch;
    }

    loggers
}

fn bubble_dead_volume_windows() -> Part {
    let frame = centered_cube(
        "closed_cell_lot_identity_balance_station_bubble_dead_volume_window_frame",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );
    let backlight = centered_cube(
        "closed_cell_lot_identity_balance_station_bubble_dead_volume_backlight_slot",
        BUBBLE_X - 58.0,
        18.0,
        16.0,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);

    let mut windows = frame - backlight;

    for i in 0..BUBBLE_WINDOWS {
        let x = port_x(i, BUBBLE_WINDOWS, WINDOW_PITCH_X);
        let window = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_bubble_witness_window_{i}"),
            BUBBLE_WINDOW_D / 2.0,
            BUBBLE_Z + 8.0,
            36,
        )
        .translate(x, 36.0, 5.0);
        let lens = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_bubble_witness_lens_land_{i}"),
            BUBBLE_WINDOW_D / 2.0 + 4.0,
            4.0,
            36,
        )
        .translate(x, 36.0, BUBBLE_Z / 2.0 + 3.0);
        windows = windows - window + lens;
    }

    for i in 0..DEAD_VOLUME_WINDOWS {
        let x = port_x(i, DEAD_VOLUME_WINDOWS, 70.0);
        let slot = centered_cube(
            format!("closed_cell_lot_identity_balance_station_dead_volume_window_{i}"),
            DEAD_VOLUME_WINDOW_X,
            22.0,
            BUBBLE_Z + 8.0,
        )
        .translate(x, -38.0, 5.0);
        let collar = centered_cube(
            format!("closed_cell_lot_identity_balance_station_dead_volume_window_bezel_{i}"),
            DEAD_VOLUME_WINDOW_X + 8.0,
            30.0,
            4.0,
        )
        .translate(x, -38.0, BUBBLE_Z / 2.0 + 3.0);
        windows = windows - slot + collar;
    }

    windows
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_cell_lot_identity_balance_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = base;

    for i in 0..DISPOSITION_GATES {
        let name = disposition_gate_name(i);
        let x = port_x(i, DISPOSITION_GATES, 126.0);
        let lane = centered_cube(
            format!("closed_cell_lot_identity_balance_station_{name}_gate_lane_recess"),
            92.0,
            52.0,
            12.0,
        )
        .translate(x, 4.0, GATE_Z / 2.0 - 4.0);
        let slider = centered_cube(
            format!("closed_cell_lot_identity_balance_station_{name}_gate_slider"),
            74.0,
            42.0,
            10.0,
        )
        .translate(x, 4.0, GATE_Z / 2.0 + 8.0);
        let flag = centered_cube(
            format!("closed_cell_lot_identity_balance_station_{name}_gate_flag_land"),
            46.0,
            12.0,
            30.0,
        )
        .translate(x, -GATE_Y / 2.0 + 15.0, GATE_Z / 2.0 + 16.0);
        gates = gates - lane + slider + flag;
    }

    for i in 0..GATE_TOKEN_SLOTS {
        let slot = centered_cube(
            format!("closed_cell_lot_identity_balance_station_disposition_token_slot_{i}"),
            34.0,
            16.0,
            GATE_Z + 8.0,
        )
        .translate(port_x(i, GATE_TOKEN_SLOTS, 55.0), GATE_Y / 2.0 - 22.0, 6.0);
        gates = gates - slot;
    }

    gates
}

fn camera_evidence_bridge_robot_service_keepouts() -> Part {
    camera_bridge() + evidence_fiducials() + robot_service_keepouts()
}

fn camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_cell_lot_identity_balance_station_camera_bridge_left_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_cell_lot_identity_balance_station_camera_bridge_right_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "closed_cell_lot_identity_balance_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        30.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 15.0);

    let mut bridge = left_post + right_post + beam;

    for i in 0..CAMERA_MOUNTS {
        let x = port_x(i, CAMERA_MOUNTS, 180.0);
        let mount = centered_cube(
            format!("closed_cell_lot_identity_balance_station_camera_mount_plate_{i}"),
            58.0,
            10.0,
            34.0,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 6.0, CAMERA_BRIDGE_Z - 46.0);
        let bore = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_camera_mount_bore_{i}"),
            3.0,
            14.0,
            18,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 6.0, CAMERA_BRIDGE_Z - 46.0);
        bridge = bridge + mount - bore;
    }

    bridge
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_cell_lot_identity_balance_station_evidence_fiducials");

    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -CAMERA_BRIDGE_X / 2.0 + 82.0 + i as f64 * ((CAMERA_BRIDGE_X - 164.0) / 9.0);
        let disk = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_evidence_fiducial_disk_{i}"),
            7.0,
            4.0,
            28,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 24.0);
        let center = centered_cylinder(
            format!("closed_cell_lot_identity_balance_station_evidence_fiducial_bore_{i}"),
            2.0,
            6.0,
            20,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 24.0);
        fiducials = fiducials + (disk - center);
    }

    fiducials
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "closed_cell_lot_identity_balance_station_front_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let service_left = centered_cube(
        "closed_cell_lot_identity_balance_station_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "closed_cell_lot_identity_balance_station_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_service = centered_cube(
        "closed_cell_lot_identity_balance_station_top_service_clearance_gauge",
        700.0,
        420.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    robot_front + service_left + service_right + top_service
}

fn station_assembly() -> Part {
    containment_deck()
        + source_lot_bag_vial_nests().translate(
            SOURCE_NEST_POS.0,
            SOURCE_NEST_POS.1,
            top_z(SOURCE_NEST_Z),
        )
        + split_pool_manifold_surrogate().translate(
            MANIFOLD_POS.0,
            MANIFOLD_POS.1,
            top_z(MANIFOLD_Z),
        )
        + equalized_branch_volume_witnesses().translate(
            WITNESS_POS.0,
            WITNESS_POS.1,
            top_z(WITNESS_Z),
        )
        + identity_barcode_rfid_custody_lands().translate(
            CUSTODY_POS.0,
            CUSTODY_POS.1,
            top_z(CUSTODY_Z),
        )
        + mass_volume_balance_pads().translate(BALANCE_POS.0, BALANCE_POS.1, top_z(BALANCE_Z))
        + retain_aliquot_wells().translate(RETAIN_POS.0, RETAIN_POS.1, top_z(RETAIN_Z))
        + temperature_logger_pockets().translate(LOGGER_POS.0, LOGGER_POS.1, top_z(LOGGER_Z))
        + bubble_dead_volume_windows().translate(BUBBLE_POS.0, BUBBLE_POS.1, top_z(BUBBLE_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge_robot_service_keepouts().translate(
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0,
        )
}

fn retain_well_center(index: usize) -> (f64, f64) {
    let col = index % RETAIN_COLS;
    let row = index / RETAIN_COLS;
    (
        port_x(col, RETAIN_COLS, RETAIN_PITCH_X),
        -RETAIN_PITCH_Y / 2.0 + row as f64 * RETAIN_PITCH_Y,
    )
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn branch_x(index: usize) -> f64 {
    port_x(index, BRANCH_COUNT, BRANCH_PITCH)
}

fn witness_x(index: usize) -> f64 {
    port_x(index, WITNESS_LANES, WITNESS_LANE_PITCH_X)
}

fn source_bag_span() -> f64 {
    (SOURCE_BAG_NESTS as f64 - 1.0) * BAG_NEST_PITCH_X + BAG_NEST_X
}

fn source_vial_span() -> f64 {
    (SOURCE_VIAL_NESTS as f64 - 1.0) * VIAL_NEST_PITCH_X + VIAL_NEST_D
}

fn manifold_branch_span() -> f64 {
    (BRANCH_COUNT as f64 - 1.0) * BRANCH_PITCH + PORT_COLLAR_D
}

fn witness_span() -> f64 {
    (WITNESS_LANES as f64 - 1.0) * WITNESS_LANE_PITCH_X + WITNESS_WINDOW_D
}

fn bubble_window_span() -> f64 {
    (BUBBLE_WINDOWS as f64 - 1.0) * WINDOW_PITCH_X + BUBBLE_WINDOW_D
}

fn dead_volume_window_span() -> f64 {
    (DEAD_VOLUME_WINDOWS as f64 - 1.0) * 70.0 + DEAD_VOLUME_WINDOW_X
}

fn inside_deck(pos: (f64, f64), x: f64, y: f64) -> bool {
    pos.0 - x / 2.0 > -DECK_X / 2.0 + RIM_W
        && pos.0 + x / 2.0 < DECK_X / 2.0 - RIM_W
        && pos.1 - y / 2.0 > -DECK_Y / 2.0 + RIM_W
        && pos.1 + y / 2.0 < DECK_Y / 2.0 - RIM_W
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(BRANCH_COUNT, WITNESS_LANES);
    assert_eq!(SOURCE_PORTS, POOL_PORTS);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert_eq!(RETAIN_WELLS % RETAIN_COLS, 0);
    assert!(inside_deck(SOURCE_NEST_POS, SOURCE_NEST_X, SOURCE_NEST_Y));
    assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
    assert!(inside_deck(WITNESS_POS, WITNESS_X, WITNESS_Y));
    assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
    assert!(inside_deck(BALANCE_POS, BALANCE_X, BALANCE_Y));
    assert!(inside_deck(RETAIN_POS, RETAIN_X, RETAIN_Y));
    assert!(inside_deck(LOGGER_POS, LOGGER_X, LOGGER_Y));
    assert!(inside_deck(BUBBLE_POS, BUBBLE_X, BUBBLE_Y));
    assert!(inside_deck(GATE_POS, GATE_X, GATE_Y));
    assert!(source_bag_span() < SOURCE_NEST_X - 44.0);
    assert!(source_vial_span() < SOURCE_NEST_X - 42.0);
    assert!(manifold_branch_span() < MANIFOLD_X - 92.0);
    assert!(witness_span() < WITNESS_X - 80.0);
    assert!(bubble_window_span() < BUBBLE_X - 64.0);
    assert!(dead_volume_window_span() < BUBBLE_X - 82.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_mechanical_validation_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 25);
        assert!(REQUIRED_FEATURES.contains(&"source_lot_bag_nests"));
        assert!(REQUIRED_FEATURES.contains(&"source_lot_vial_nests"));
        assert!(REQUIRED_FEATURES.contains(&"split_pool_manifold_surrogate"));
        assert!(REQUIRED_FEATURES.contains(&"equalized_branch_volume_witnesses"));
        assert!(REQUIRED_FEATURES.contains(&"identity_barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"identity_rfid_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"mass_balance_pads"));
        assert!(REQUIRED_FEATURES.contains(&"volume_balance_pads"));
        assert!(REQUIRED_FEATURES.contains(&"retain_aliquot_wells"));
        assert!(REQUIRED_FEATURES.contains(&"temperature_logger_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"dead_volume_windows"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn scope_excludes_process_release_and_clinical_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 5);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_standard_operating_procedure"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"gmp_release_decision"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"clinical_acceptance_criterion"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"validated_cell_processing_instruction"));
    }

    #[test]
    fn counts_match_split_pool_identity_balance_station_intent() {
        assert_eq!(SOURCE_BAG_NESTS, 3);
        assert_eq!(SOURCE_VIAL_NESTS, BRANCH_COUNT);
        assert_eq!(BRANCH_COUNT, WITNESS_LANES);
        assert_eq!(MASS_BALANCE_PADS, VOLUME_BALANCE_PADS);
        assert!(RETAIN_WELLS >= BRANCH_COUNT * 2);
        assert!(BARCODE_LANDS >= BRANCH_COUNT);
        assert_eq!(RFID_LANDS, BRANCH_COUNT);
        assert_eq!(DISPOSITION_GATES, 3);
    }

    #[test]
    fn layout_fits_contained_robotic_station() {
        assert_layout();
        assert!(inside_deck(SOURCE_NEST_POS, SOURCE_NEST_X, SOURCE_NEST_Y));
        assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
        assert!(inside_deck(BALANCE_POS, BALANCE_X, BALANCE_Y));
        assert!(inside_deck(RETAIN_POS, RETAIN_X, RETAIN_Y));
        assert!(inside_deck(BUBBLE_POS, BUBBLE_X, BUBBLE_Y));
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
    }

    #[test]
    fn geometry_spans_remain_within_component_envelopes() {
        assert!(source_bag_span() < SOURCE_NEST_X - 44.0);
        assert!(source_vial_span() < SOURCE_NEST_X - 42.0);
        assert!(manifold_branch_span() < MANIFOLD_X - 92.0);
        assert!(witness_span() < WITNESS_X - 80.0);
        assert!(bubble_window_span() < BUBBLE_X - 64.0);
        assert!(dead_volume_window_span() < BUBBLE_X - 82.0);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }
}
