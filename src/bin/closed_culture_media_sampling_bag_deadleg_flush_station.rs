use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed culture-media sampling bag deadleg flush validation station.
//
// This generator packages a mechanical validation fixture for checking flush
// accessibility and evidence capture around a sealed sample bag dock and
// removable deadleg surrogate cartridges. It models fixture geometry, custody
// lands, visible witness features, and robot/service keepouts only. It is not a
// sterile-process claim, SOP, pressure-rated device, or biological acceptance
// criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_culture_media_sampling_bag_deadleg_flush_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_containment_deck.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_sealed_sample_bag_dock.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_deadleg_surrogate_loop_cartridges.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_flush_volume_token_rail.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_high_low_carryover_standard_wells.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_sterile_sample_bulkhead.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_waste_flush_capture.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_bubble_wetness_witness_windows.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_pressure_flow_taps.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_barcode_coa_custody_lands.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_release_hold_reject_gates.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_camera_evidence_bridge_robot_keepouts.stl",
    "output/closed_culture_media_sampling_bag_deadleg_flush_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 22] = [
    "sealed_sample_bag_dock",
    "bag_datum_saddles",
    "sample_port_cap_parking",
    "deadleg_surrogate_loop_cartridges",
    "deadleg_loop_length_keys",
    "flush_volume_token_rail",
    "high_carryover_standard_wells",
    "low_carryover_standard_wells",
    "blank_reference_wells",
    "sterile_sample_bulkhead",
    "waste_flush_capture",
    "segregated_waste_and_flush_cells",
    "bubble_witness_windows",
    "wetness_witness_pads",
    "pressure_taps",
    "flow_taps",
    "barcode_lands",
    "coa_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge_robot_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 4] = [
    "sterile_process_claim",
    "standard_operating_procedure",
    "pressure_rated_device",
    "biological_acceptance_criterion",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1110.0;
const SUMP_Y: f64 = 666.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const BAG_DOCK_X: f64 = 330.0;
const BAG_DOCK_Y: f64 = 236.0;
const BAG_DOCK_Z: f64 = 50.0;
const BAG_DOCK_POS: (f64, f64) = (-410.0, 238.0);
const BAG_SADDLES: usize = 4;
const BAG_CLAMP_RIBS: usize = 5;
const SAMPLE_PORTS: usize = 4;
const SAMPLE_PORT_D: f64 = 24.0;
const SAMPLE_PORT_SPACING: f64 = 58.0;

const CARTRIDGE_BANK_X: f64 = 482.0;
const CARTRIDGE_BANK_Y: f64 = 205.0;
const CARTRIDGE_BANK_Z: f64 = 42.0;
const CARTRIDGE_BANK_POS: (f64, f64) = (50.0, 238.0);
const DEADLEG_CARTRIDGES: usize = 4;
const CARTRIDGE_BODY_X: f64 = 92.0;
const CARTRIDGE_BODY_Y: f64 = 155.0;
const CARTRIDGE_BODY_Z: f64 = 54.0;
const CARTRIDGE_PITCH_X: f64 = 108.0;
const LOOP_CHANNEL_D: f64 = 9.0;
const DEADLEG_STUBS_PER_CARTRIDGE: usize = 3;

const TOKEN_RAIL_X: f64 = 508.0;
const TOKEN_RAIL_Y: f64 = 96.0;
const TOKEN_RAIL_Z: f64 = 34.0;
const TOKEN_RAIL_POS: (f64, f64) = (335.0, 70.0);
const FLUSH_VOLUME_TOKENS: usize = 7;
const TOKEN_PITCH: f64 = 62.0;
const TOKEN_D: f64 = 29.0;

const STANDARD_WELLS_X: f64 = 430.0;
const STANDARD_WELLS_Y: f64 = 172.0;
const STANDARD_WELLS_Z: f64 = 42.0;
const STANDARD_WELLS_POS: (f64, f64) = (394.0, -174.0);
const HIGH_STANDARD_WELLS: usize = 4;
const LOW_STANDARD_WELLS: usize = 4;
const BLANK_STANDARD_WELLS: usize = 2;
const STANDARD_WELL_D: f64 = 29.0;
const STANDARD_WELL_PITCH_X: f64 = 48.0;

const BULKHEAD_X: f64 = 260.0;
const BULKHEAD_Y: f64 = 170.0;
const BULKHEAD_Z: f64 = 84.0;
const BULKHEAD_POS: (f64, f64) = (-468.0, 16.0);
const BULKHEAD_PORTS: usize = 6;
const BULKHEAD_PORT_D: f64 = 21.0;
const BULKHEAD_PORT_PITCH: f64 = 34.0;

const WASTE_X: f64 = 360.0;
const WASTE_Y: f64 = 204.0;
const WASTE_Z: f64 = 52.0;
const WASTE_POS: (f64, f64) = (-420.0, -272.0);
const WASTE_CELLS: usize = 4;
const FLUSH_CAPTURE_CELLS: usize = 4;
const CAPTURE_CELL_X: f64 = 70.0;
const CAPTURE_CELL_Y: f64 = 54.0;

const WITNESS_X: f64 = 470.0;
const WITNESS_Y: f64 = 136.0;
const WITNESS_Z: f64 = 30.0;
const WITNESS_POS: (f64, f64) = (25.0, -287.0);
const BUBBLE_WINDOWS: usize = 6;
const WETNESS_PADS: usize = 8;
const WITNESS_WINDOW_D: f64 = 31.0;
const WITNESS_WINDOW_PITCH: f64 = 56.0;

const TAP_BAR_X: f64 = 538.0;
const TAP_BAR_Y: f64 = 112.0;
const TAP_BAR_Z: f64 = 44.0;
const TAP_BAR_POS: (f64, f64) = (26.0, -55.0);
const PRESSURE_TAPS: usize = 4;
const FLOW_TAPS: usize = 3;
const TAP_BOSS_D: f64 = 34.0;
const TAP_BORE_D: f64 = 7.0;
const TAP_PITCH: f64 = 68.0;

const CUSTODY_X: f64 = 398.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (395.0, 315.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_PADS: usize = 5;

const GATE_X: f64 = 424.0;
const GATE_Y: f64 = 110.0;
const GATE_Z: f64 = 38.0;
const GATE_POS: (f64, f64) = (94.0, -350.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 956.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 224.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (5.0, -28.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const ROBOT_KEEPOUT_X: f64 = 1130.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 80.0;
const SERVICE_KEEPOUT_X: f64 = 100.0;
const SERVICE_KEEPOUT_Y: f64 = 682.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 310.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(sealed_sample_bag_dock(), OUTPUTS[1]);
    write_part(deadleg_surrogate_loop_cartridges(), OUTPUTS[2]);
    write_part(flush_volume_token_rail(), OUTPUTS[3]);
    write_part(high_low_carryover_standard_wells(), OUTPUTS[4]);
    write_part(sterile_sample_bulkhead(), OUTPUTS[5]);
    write_part(waste_flush_capture(), OUTPUTS[6]);
    write_part(bubble_wetness_witness_windows(), OUTPUTS[7]);
    write_part(pressure_flow_taps(), OUTPUTS[8]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[9]);
    write_part(release_hold_reject_gates(), OUTPUTS[10]);
    write_part(camera_evidence_bridge_robot_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed culture-media sample bag deadleg flush station: {:.0}mm x {:.0}mm containment deck, {} deadleg loop cartridges, {} flush-volume tokens.",
        DECK_X, DECK_Y, DEADLEG_CARTRIDGES, FLUSH_VOLUME_TOKENS
    );
    println!(
        "Witness/carryover features: {} high standards, {} low standards, {} blanks, {} bubble windows, {} wetness pads.",
        HIGH_STANDARD_WELLS,
        LOW_STANDARD_WELLS,
        BLANK_STANDARD_WELLS,
        BUBBLE_WINDOWS,
        WETNESS_PADS
    );
    println!(
        "Mechanical custody and routing: {} bulkhead ports, {} pressure taps, {} flow taps, {} barcode lands, {} COA lands, release/hold/reject gates.",
        BULKHEAD_PORTS,
        PRESSURE_TAPS,
        FLOW_TAPS,
        BARCODE_LANDS,
        COA_LANDS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_deadleg_flush_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_deadleg_flush_station_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_deadleg_flush_station_captured_drain",
        DRAIN_D / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 76.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_deadleg_flush_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_center_z());
    let right = centered_cube(
        "closed_deadleg_flush_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_deadleg_flush_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_deadleg_flush_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -(DECK_Y / 2.0 - RIM_W / 2.0), rim_center_z());

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = datum_boss(0);
    for i in 1..DATUM_BOSSES {
        datums = datums + datum_boss(i);
    }

    datums
}

fn datum_boss(i: usize) -> Part {
    let x = -DECK_X / 2.0 + 78.0 + i as f64 * ((DECK_X - 156.0) / 9.0);
    let y = if i % 2 == 0 {
        DECK_Y / 2.0 - 70.0
    } else {
        -DECK_Y / 2.0 + 70.0
    };
    let pad = centered_cylinder(
        format!("closed_deadleg_flush_station_datum_pad_{i}"),
        7.0,
        5.0,
        32,
    )
    .translate(x, y, top_z(5.0));
    let bore = centered_cylinder(
        format!("closed_deadleg_flush_station_datum_bore_{i}"),
        1.8,
        9.0,
        20,
    )
    .translate(x, y, top_z(9.0));

    pad - bore
}

fn station_landing_pockets() -> Part {
    landing_pocket("bag_dock_land", BAG_DOCK_X, BAG_DOCK_Y, BAG_DOCK_POS)
        + landing_pocket(
            "deadleg_loop_cartridge_land",
            CARTRIDGE_BANK_X,
            CARTRIDGE_BANK_Y,
            CARTRIDGE_BANK_POS,
        )
        + landing_pocket(
            "flush_token_rail_land",
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
            TOKEN_RAIL_POS,
        )
        + landing_pocket(
            "carryover_standard_well_land",
            STANDARD_WELLS_X,
            STANDARD_WELLS_Y,
            STANDARD_WELLS_POS,
        )
        + landing_pocket("sample_bulkhead_land", BULKHEAD_X, BULKHEAD_Y, BULKHEAD_POS)
        + landing_pocket("waste_flush_capture_land", WASTE_X, WASTE_Y, WASTE_POS)
        + landing_pocket("witness_window_land", WITNESS_X, WITNESS_Y, WITNESS_POS)
        + landing_pocket("pressure_flow_tap_land", TAP_BAR_X, TAP_BAR_Y, TAP_BAR_POS)
        + landing_pocket("custody_panel_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pocket("disposition_gate_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("closed_deadleg_flush_station_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn sealed_sample_bag_dock() -> Part {
    let base = centered_cube(
        "closed_deadleg_flush_station_sealed_sample_bag_dock_base",
        BAG_DOCK_X,
        BAG_DOCK_Y,
        BAG_DOCK_Z,
    );
    let bag_recess = centered_cube(
        "closed_deadleg_flush_station_sample_bag_soft_recess",
        BAG_DOCK_X - 64.0,
        BAG_DOCK_Y - 68.0,
        22.0,
    )
    .translate(0.0, 0.0, BAG_DOCK_Z / 2.0 - 5.0);
    let perimeter_gasket = centered_cube(
        "closed_deadleg_flush_station_bag_gasket_witness_rim",
        BAG_DOCK_X - 34.0,
        BAG_DOCK_Y - 38.0,
        9.0,
    )
    .translate(0.0, 0.0, BAG_DOCK_Z / 2.0 + 4.0);

    let mut dock = base - bag_recess + perimeter_gasket;

    for i in 0..BAG_SADDLES {
        let x = -112.0 + i as f64 * 74.0;
        let saddle = centered_cylinder(
            format!("closed_deadleg_flush_station_bag_datum_saddle_{i}"),
            12.0,
            BAG_DOCK_Y - 86.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, BAG_DOCK_Z / 2.0 + 15.0);
        dock = dock + saddle;
    }

    for i in 0..BAG_CLAMP_RIBS {
        let x = -132.0 + i as f64 * 66.0;
        let rib = centered_cube(
            format!("closed_deadleg_flush_station_bag_clamp_rib_{i}"),
            14.0,
            BAG_DOCK_Y + 10.0,
            18.0,
        )
        .translate(x, 0.0, BAG_DOCK_Z / 2.0 + 11.0);
        dock = dock + rib;
    }

    for i in 0..SAMPLE_PORTS {
        let x = port_x(i, SAMPLE_PORTS, SAMPLE_PORT_SPACING);
        let collar = centered_cylinder(
            format!("closed_deadleg_flush_station_sealed_sample_port_collar_{i}"),
            SAMPLE_PORT_D / 2.0,
            38.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BAG_DOCK_Y / 2.0 - 6.0, 8.0);
        let bore = centered_cylinder(
            format!("closed_deadleg_flush_station_sealed_sample_port_bore_{i}"),
            4.6,
            48.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BAG_DOCK_Y / 2.0 - 6.0, 8.0);
        let cap_land = centered_cube(
            format!("closed_deadleg_flush_station_sample_port_cap_parking_{i}"),
            36.0,
            24.0,
            6.0,
        )
        .translate(x, BAG_DOCK_Y / 2.0 - 24.0, BAG_DOCK_Z / 2.0 + 7.0);
        dock = dock + collar - bore + cap_land;
    }

    dock
}

fn deadleg_surrogate_loop_cartridges() -> Part {
    let base = centered_cube(
        "closed_deadleg_flush_station_deadleg_cartridge_bank_base",
        CARTRIDGE_BANK_X,
        CARTRIDGE_BANK_Y,
        CARTRIDGE_BANK_Z,
    );
    let rail_cut = centered_cube(
        "closed_deadleg_flush_station_cartridge_slide_clearance",
        CARTRIDGE_BANK_X - 42.0,
        34.0,
        14.0,
    )
    .translate(
        0.0,
        -CARTRIDGE_BANK_Y / 2.0 + 28.0,
        CARTRIDGE_BANK_Z / 2.0 - 2.0,
    );

    let mut bank = base - rail_cut;

    for i in 0..DEADLEG_CARTRIDGES {
        let x = cartridge_x(i);
        let body = centered_cube(
            format!("closed_deadleg_flush_station_deadleg_surrogate_loop_cartridge_{i}"),
            CARTRIDGE_BODY_X,
            CARTRIDGE_BODY_Y,
            CARTRIDGE_BODY_Z,
        )
        .translate(x, 8.0, CARTRIDGE_BANK_Z / 2.0 + CARTRIDGE_BODY_Z / 2.0);
        let inlet = centered_cylinder(
            format!("closed_deadleg_flush_station_cartridge_inlet_channel_{i}"),
            LOOP_CHANNEL_D / 2.0,
            CARTRIDGE_BODY_Y - 26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 24.0, 8.0, CARTRIDGE_BANK_Z + 15.0);
        let return_leg = centered_cylinder(
            format!("closed_deadleg_flush_station_cartridge_return_channel_{i}"),
            LOOP_CHANNEL_D / 2.0,
            CARTRIDGE_BODY_Y - 26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 24.0, 8.0, CARTRIDGE_BANK_Z + 15.0);
        let loop_turn = centered_cylinder(
            format!("closed_deadleg_flush_station_cartridge_u_turn_visible_loop_{i}"),
            25.0,
            8.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CARTRIDGE_BODY_Y / 2.0 - 8.0, CARTRIDGE_BANK_Z + 15.0);
        let length_key = centered_cube(
            format!("closed_deadleg_flush_station_deadleg_loop_length_key_{i}"),
            14.0 + i as f64 * 5.0,
            28.0,
            10.0,
        )
        .translate(
            x,
            -CARTRIDGE_BODY_Y / 2.0 + 8.0,
            CARTRIDGE_BANK_Z + CARTRIDGE_BODY_Z + 8.0,
        );

        bank = bank + body + inlet + return_leg + loop_turn + length_key;

        for j in 0..DEADLEG_STUBS_PER_CARTRIDGE {
            let stub = centered_cylinder(
                format!("closed_deadleg_flush_station_deadleg_stub_{i}_{j}"),
                4.0,
                34.0 + j as f64 * 10.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x,
                -38.0 + j as f64 * 28.0,
                CARTRIDGE_BANK_Z + 34.0 + j as f64 * 3.0,
            );
            bank = bank + stub;
        }
    }

    bank
}

fn flush_volume_token_rail() -> Part {
    let rail = centered_cube(
        "closed_deadleg_flush_station_flush_volume_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let sight_slot = centered_cube(
        "closed_deadleg_flush_station_token_rail_sight_slot",
        TOKEN_RAIL_X - 58.0,
        18.0,
        16.0,
    )
    .translate(0.0, -8.0, TOKEN_RAIL_Z / 2.0);

    let mut part = rail - sight_slot;

    for i in 0..FLUSH_VOLUME_TOKENS {
        let x = token_x(i);
        let well = centered_cylinder(
            format!("closed_deadleg_flush_station_flush_volume_token_well_{i}"),
            TOKEN_D / 2.0,
            TOKEN_RAIL_Z + 10.0,
            36,
        )
        .translate(x, 20.0, 6.0);
        let token = centered_cylinder(
            format!("closed_deadleg_flush_station_flush_volume_token_gauge_{i}"),
            TOKEN_D / 2.0 - 3.0,
            6.0,
            36,
        )
        .translate(x, 20.0, TOKEN_RAIL_Z / 2.0 + 5.0);
        let volume_step = centered_cube(
            format!("closed_deadleg_flush_station_flush_volume_step_gauge_{i}"),
            12.0,
            34.0,
            5.0 + i as f64 * 2.0,
        )
        .translate(
            x,
            -TOKEN_RAIL_Y / 2.0 + 20.0,
            TOKEN_RAIL_Z / 2.0 + 2.5 + i as f64,
        );
        part = part - well + token + volume_step;
    }

    part
}

fn high_low_carryover_standard_wells() -> Part {
    let base = centered_cube(
        "closed_deadleg_flush_station_high_low_carryover_standard_well_bank",
        STANDARD_WELLS_X,
        STANDARD_WELLS_Y,
        STANDARD_WELLS_Z,
    );
    let divider = centered_cube(
        "closed_deadleg_flush_station_high_low_standard_divider",
        STANDARD_WELLS_X - 34.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, STANDARD_WELLS_Z / 2.0 + 6.0);

    let mut bank = base + divider;

    for i in 0..HIGH_STANDARD_WELLS {
        let well = standard_well("high_carryover_standard", i, -44.0, HIGH_STANDARD_WELLS);
        bank = bank - well.0 + well.1;
    }
    for i in 0..LOW_STANDARD_WELLS {
        let well = standard_well("low_carryover_standard", i, 44.0, LOW_STANDARD_WELLS);
        bank = bank - well.0 + well.1;
    }
    for i in 0..BLANK_STANDARD_WELLS {
        let x = -STANDARD_WELLS_X / 2.0 + 36.0 + i as f64 * 42.0;
        let blank = centered_cylinder(
            format!("closed_deadleg_flush_station_blank_reference_well_{i}"),
            11.0,
            STANDARD_WELLS_Z + 8.0,
            28,
        )
        .translate(x, 0.0, 8.0);
        let collar = centered_cylinder(
            format!("closed_deadleg_flush_station_blank_reference_collar_{i}"),
            15.0,
            5.0,
            28,
        )
        .translate(x, 0.0, STANDARD_WELLS_Z / 2.0 + 4.0);
        bank = bank - blank + collar;
    }

    bank
}

fn standard_well(prefix: &str, index: usize, y: f64, count: usize) -> (Part, Part) {
    let x = port_x(index, count, STANDARD_WELL_PITCH_X);
    let cut = centered_cylinder(
        format!("closed_deadleg_flush_station_{prefix}_well_cut_{index}"),
        STANDARD_WELL_D / 2.0,
        STANDARD_WELLS_Z + 8.0,
        36,
    )
    .translate(x, y, 8.0);
    let collar = centered_cylinder(
        format!("closed_deadleg_flush_station_{prefix}_well_collar_{index}"),
        STANDARD_WELL_D / 2.0 + 4.0,
        5.0,
        36,
    )
    .translate(x, y, STANDARD_WELLS_Z / 2.0 + 4.0);
    (cut, collar)
}

fn sterile_sample_bulkhead() -> Part {
    let base = centered_cube(
        "closed_deadleg_flush_station_sterile_sample_bulkhead_base",
        BULKHEAD_X,
        BULKHEAD_Y,
        20.0,
    );
    let wall = centered_cube(
        "closed_deadleg_flush_station_sterile_sample_bulkhead_wall",
        BULKHEAD_X - 26.0,
        18.0,
        BULKHEAD_Z,
    )
    .translate(0.0, 10.0, BULKHEAD_Z / 2.0);
    let drip_lip = centered_cube(
        "closed_deadleg_flush_station_bulkhead_drip_lip",
        BULKHEAD_X - 52.0,
        12.0,
        12.0,
    )
    .translate(0.0, -BULKHEAD_Y / 2.0 + 20.0, 20.0);

    let mut bulkhead = base + wall + drip_lip;

    for i in 0..BULKHEAD_PORTS {
        let x = port_x(i, BULKHEAD_PORTS, BULKHEAD_PORT_PITCH);
        let port = centered_cylinder(
            format!("closed_deadleg_flush_station_bulkhead_sample_port_boss_{i}"),
            BULKHEAD_PORT_D / 2.0,
            32.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 10.0, BULKHEAD_Z / 2.0 + 6.0);
        let bore = centered_cylinder(
            format!("closed_deadleg_flush_station_bulkhead_sample_port_bore_{i}"),
            4.2,
            42.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 10.0, BULKHEAD_Z / 2.0 + 6.0);
        let custody_tab = centered_cube(
            format!("closed_deadleg_flush_station_bulkhead_port_custody_tab_{i}"),
            23.0,
            8.0,
            16.0,
        )
        .translate(x, -8.0, BULKHEAD_Z - 8.0);
        bulkhead = bulkhead + port - bore + custody_tab;
    }

    bulkhead
}

fn waste_flush_capture() -> Part {
    let tray = centered_cube(
        "closed_deadleg_flush_station_waste_flush_capture_tray",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let moat = centered_cube(
        "closed_deadleg_flush_station_waste_flush_capture_moat",
        WASTE_X - 34.0,
        WASTE_Y - 34.0,
        28.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 4.0);
    let front_weir = centered_cube(
        "closed_deadleg_flush_station_flush_capture_front_weir",
        WASTE_X - 22.0,
        10.0,
        26.0,
    )
    .translate(0.0, -WASTE_Y / 2.0 + 17.0, WASTE_Z / 2.0 + 10.0);

    let mut capture = tray - moat + front_weir;

    for i in 0..WASTE_CELLS {
        let cell = capture_cell("waste", i, -36.0, WASTE_CELLS);
        capture = capture - cell.0 + cell.1;
    }
    for i in 0..FLUSH_CAPTURE_CELLS {
        let cell = capture_cell("flush", i, 48.0, FLUSH_CAPTURE_CELLS);
        capture = capture - cell.0 + cell.1;
    }

    let drain = centered_cylinder(
        "closed_deadleg_flush_station_waste_flush_capture_drain",
        7.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_X / 2.0 - 42.0, -WASTE_Y / 2.0 - 5.0, 4.0);

    capture - drain
}

fn capture_cell(prefix: &str, index: usize, y: f64, count: usize) -> (Part, Part) {
    let x = port_x(index, count, CAPTURE_CELL_X);
    let cut = centered_cube(
        format!("closed_deadleg_flush_station_{prefix}_capture_cell_cut_{index}"),
        CAPTURE_CELL_X - 16.0,
        CAPTURE_CELL_Y,
        WASTE_Z + 8.0,
    )
    .translate(x, y, 8.0);
    let label_land = centered_cube(
        format!("closed_deadleg_flush_station_{prefix}_capture_cell_label_land_{index}"),
        CAPTURE_CELL_X - 18.0,
        8.0,
        4.0,
    )
    .translate(x, y + CAPTURE_CELL_Y / 2.0 + 10.0, WASTE_Z / 2.0 + 3.0);
    (cut, label_land)
}

fn bubble_wetness_witness_windows() -> Part {
    let frame = centered_cube(
        "closed_deadleg_flush_station_bubble_wetness_witness_window_frame",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let backlight_slot = centered_cube(
        "closed_deadleg_flush_station_witness_backlight_slot",
        WITNESS_X - 66.0,
        20.0,
        18.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);

    let mut witness = frame - backlight_slot;

    for i in 0..BUBBLE_WINDOWS {
        let x = port_x(i, BUBBLE_WINDOWS, WITNESS_WINDOW_PITCH);
        let window = centered_cylinder(
            format!("closed_deadleg_flush_station_bubble_witness_window_{i}"),
            WITNESS_WINDOW_D / 2.0,
            WITNESS_Z + 8.0,
            36,
        )
        .translate(x, 35.0, 6.0);
        let lens_land = centered_cylinder(
            format!("closed_deadleg_flush_station_bubble_witness_lens_land_{i}"),
            WITNESS_WINDOW_D / 2.0 + 4.0,
            4.0,
            36,
        )
        .translate(x, 35.0, WITNESS_Z / 2.0 + 3.0);
        witness = witness - window + lens_land;
    }

    for i in 0..WETNESS_PADS {
        let x = port_x(i, WETNESS_PADS, 48.0);
        let pad = centered_cube(
            format!("closed_deadleg_flush_station_wetness_witness_pad_{i}"),
            30.0,
            24.0,
            5.0,
        )
        .translate(x, -38.0, WITNESS_Z / 2.0 + 4.0);
        witness = witness + pad;
    }

    witness
}

fn pressure_flow_taps() -> Part {
    let bar = centered_cube(
        "closed_deadleg_flush_station_pressure_flow_tap_bar",
        TAP_BAR_X,
        TAP_BAR_Y,
        TAP_BAR_Z,
    );
    let tube_channel = centered_cylinder(
        "closed_deadleg_flush_station_pressure_flow_main_tube_channel",
        6.0,
        TAP_BAR_X - 38.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, TAP_BAR_Z / 2.0 + 4.0);

    let mut taps = bar + tube_channel;

    for i in 0..PRESSURE_TAPS {
        let x = port_x(i, PRESSURE_TAPS, TAP_PITCH);
        let boss = tap_boss("pressure", i, x, 24.0);
        taps = taps + boss.0 - boss.1;
    }

    for i in 0..FLOW_TAPS {
        let x = port_x(i, FLOW_TAPS, TAP_PITCH + 12.0);
        let boss = tap_boss("flow", i, x, -28.0);
        let paddle = centered_cube(
            format!("closed_deadleg_flush_station_flow_tap_paddle_witness_{i}"),
            36.0,
            14.0,
            6.0,
        )
        .translate(x, -48.0, TAP_BAR_Z / 2.0 + 6.0);
        taps = taps + boss.0 - boss.1 + paddle;
    }

    taps
}

fn tap_boss(prefix: &str, index: usize, x: f64, y: f64) -> (Part, Part) {
    let boss = centered_cylinder(
        format!("closed_deadleg_flush_station_{prefix}_tap_boss_{index}"),
        TAP_BOSS_D / 2.0,
        18.0,
        36,
    )
    .translate(x, y, TAP_BAR_Z / 2.0 + 9.0);
    let bore = centered_cylinder(
        format!("closed_deadleg_flush_station_{prefix}_tap_bore_{index}"),
        TAP_BORE_D / 2.0,
        26.0,
        24,
    )
    .translate(x, y, TAP_BAR_Z / 2.0 + 9.0);
    (boss, bore)
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_deadleg_flush_station_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut custody = panel;

    for i in 0..BARCODE_LANDS {
        let land = centered_cube(
            format!("closed_deadleg_flush_station_barcode_land_{i}"),
            48.0,
            22.0,
            4.0,
        )
        .translate(port_x(i, BARCODE_LANDS, 58.0), 28.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..COA_LANDS {
        let land = centered_cube(
            format!("closed_deadleg_flush_station_coa_land_{i}"),
            78.0,
            30.0,
            4.0,
        )
        .translate(port_x(i, COA_LANDS, 96.0), -22.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..TAMPER_SEAL_PADS {
        let pad = centered_cylinder(
            format!("closed_deadleg_flush_station_tamper_seal_pad_{i}"),
            8.0,
            4.0,
            24,
        )
        .translate(
            -CUSTODY_X / 2.0 + 28.0 + i as f64 * 30.0,
            -CUSTODY_Y / 2.0 + 20.0,
            CUSTODY_Z / 2.0 + 4.0,
        );
        custody = custody + pad;
    }

    custody
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_deadleg_flush_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = base;

    for i in 0..DISPOSITION_GATES {
        let name = disposition_gate_name(i);
        let x = port_x(i, DISPOSITION_GATES, 126.0);
        let gate = centered_cube(
            format!("closed_deadleg_flush_station_{name}_gate_slider"),
            82.0,
            54.0,
            12.0,
        )
        .translate(x, 8.0, GATE_Z / 2.0 + 8.0);
        let flag = centered_cube(
            format!("closed_deadleg_flush_station_{name}_gate_flag_land"),
            58.0,
            12.0,
            28.0,
        )
        .translate(x, -GATE_Y / 2.0 + 16.0, GATE_Z / 2.0 + 16.0);
        gates = gates + gate + flag;
    }

    for i in 0..GATE_TOKEN_SLOTS {
        let slot = centered_cube(
            format!("closed_deadleg_flush_station_disposition_token_slot_{i}"),
            34.0,
            16.0,
            GATE_Z + 8.0,
        )
        .translate(port_x(i, GATE_TOKEN_SLOTS, 55.0), GATE_Y / 2.0 - 24.0, 6.0);
        gates = gates - slot;
    }

    gates
}

fn camera_evidence_bridge_robot_keepouts() -> Part {
    let left_post = centered_cube(
        "closed_deadleg_flush_station_camera_bridge_left_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_deadleg_flush_station_camera_bridge_right_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "closed_deadleg_flush_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        34.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 17.0);

    let mut bridge = left_post + right_post + beam;

    for i in 0..CAMERA_MOUNTS {
        let x = port_x(i, CAMERA_MOUNTS, 176.0);
        let mount = centered_cylinder(
            format!("closed_deadleg_flush_station_camera_mount_{i}"),
            15.0,
            9.0,
            32,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 5.0, CAMERA_BRIDGE_Z - 38.0);
        let bore = centered_cylinder(
            format!("closed_deadleg_flush_station_camera_mount_bore_{i}"),
            3.0,
            13.0,
            18,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 5.0, CAMERA_BRIDGE_Z - 38.0);
        bridge = bridge + mount - bore;
    }

    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -CAMERA_BRIDGE_X / 2.0 + 75.0 + i as f64 * ((CAMERA_BRIDGE_X - 150.0) / 9.0);
        let fiducial = centered_cylinder(
            format!("closed_deadleg_flush_station_evidence_fiducial_{i}"),
            5.5,
            4.0,
            20,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 21.0);
        bridge = bridge + fiducial;
    }

    bridge + robot_service_keepouts()
}

fn robot_service_keepouts() -> Part {
    let robot_front = centered_cube(
        "closed_deadleg_flush_station_front_robot_keepout_gauge",
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
        "closed_deadleg_flush_station_left_service_keepout_gauge",
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
        "closed_deadleg_flush_station_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top = centered_cube(
        "closed_deadleg_flush_station_top_service_clearance_gauge",
        680.0,
        420.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    robot_front + service_left + service_right + top
}

fn station_assembly() -> Part {
    containment_deck()
        + sealed_sample_bag_dock().translate(BAG_DOCK_POS.0, BAG_DOCK_POS.1, top_z(BAG_DOCK_Z))
        + deadleg_surrogate_loop_cartridges().translate(
            CARTRIDGE_BANK_POS.0,
            CARTRIDGE_BANK_POS.1,
            top_z(CARTRIDGE_BANK_Z),
        )
        + flush_volume_token_rail().translate(
            TOKEN_RAIL_POS.0,
            TOKEN_RAIL_POS.1,
            top_z(TOKEN_RAIL_Z),
        )
        + high_low_carryover_standard_wells().translate(
            STANDARD_WELLS_POS.0,
            STANDARD_WELLS_POS.1,
            top_z(STANDARD_WELLS_Z),
        )
        + sterile_sample_bulkhead().translate(BULKHEAD_POS.0, BULKHEAD_POS.1, top_z(20.0))
        + waste_flush_capture().translate(WASTE_POS.0, WASTE_POS.1, top_z(WASTE_Z))
        + bubble_wetness_witness_windows().translate(WITNESS_POS.0, WITNESS_POS.1, top_z(WITNESS_Z))
        + pressure_flow_taps().translate(TAP_BAR_POS.0, TAP_BAR_POS.1, top_z(TAP_BAR_Z))
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge_robot_keepouts().translate(
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0,
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

fn cartridge_x(index: usize) -> f64 {
    port_x(index, DEADLEG_CARTRIDGES, CARTRIDGE_PITCH_X)
}

fn token_x(index: usize) -> f64 {
    port_x(index, FLUSH_VOLUME_TOKENS, TOKEN_PITCH)
}

fn token_span() -> f64 {
    (FLUSH_VOLUME_TOKENS as f64 - 1.0) * TOKEN_PITCH + TOKEN_D
}

fn cartridge_bank_span() -> f64 {
    (DEADLEG_CARTRIDGES as f64 - 1.0) * CARTRIDGE_PITCH_X + CARTRIDGE_BODY_X
}

fn witness_window_span() -> f64 {
    (BUBBLE_WINDOWS as f64 - 1.0) * WITNESS_WINDOW_PITCH + WITNESS_WINDOW_D
}

fn tap_span(count: usize) -> f64 {
    (count as f64 - 1.0) * TAP_PITCH + TAP_BOSS_D
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
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert!(inside_deck(BAG_DOCK_POS, BAG_DOCK_X, BAG_DOCK_Y));
    assert!(inside_deck(
        CARTRIDGE_BANK_POS,
        CARTRIDGE_BANK_X,
        CARTRIDGE_BANK_Y
    ));
    assert!(inside_deck(TOKEN_RAIL_POS, TOKEN_RAIL_X, TOKEN_RAIL_Y));
    assert!(inside_deck(
        STANDARD_WELLS_POS,
        STANDARD_WELLS_X,
        STANDARD_WELLS_Y
    ));
    assert!(inside_deck(BULKHEAD_POS, BULKHEAD_X, BULKHEAD_Y));
    assert!(inside_deck(WASTE_POS, WASTE_X, WASTE_Y));
    assert!(inside_deck(WITNESS_POS, WITNESS_X, WITNESS_Y));
    assert!(inside_deck(TAP_BAR_POS, TAP_BAR_X, TAP_BAR_Y));
    assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
    assert!(inside_deck(GATE_POS, GATE_X, GATE_Y));
    assert!(token_span() < TOKEN_RAIL_X - 52.0);
    assert!(cartridge_bank_span() < CARTRIDGE_BANK_X - 28.0);
    assert!(witness_window_span() < WITNESS_X - 80.0);
    assert!(tap_span(PRESSURE_TAPS) < TAP_BAR_X - 220.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_mechanical_validation_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 22);
        assert!(REQUIRED_FEATURES.contains(&"sealed_sample_bag_dock"));
        assert!(REQUIRED_FEATURES.contains(&"deadleg_surrogate_loop_cartridges"));
        assert!(REQUIRED_FEATURES.contains(&"flush_volume_token_rail"));
        assert!(REQUIRED_FEATURES.contains(&"high_carryover_standard_wells"));
        assert!(REQUIRED_FEATURES.contains(&"low_carryover_standard_wells"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_sample_bulkhead"));
        assert!(REQUIRED_FEATURES.contains(&"waste_flush_capture"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"wetness_witness_pads"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_taps"));
        assert!(REQUIRED_FEATURES.contains(&"flow_taps"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge_robot_keepouts"));
    }

    #[test]
    fn scope_excludes_process_and_acceptance_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 4);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"standard_operating_procedure"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"pressure_rated_device"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_acceptance_criterion"));
    }

    #[test]
    fn counts_match_deadleg_flush_fixture_intent() {
        assert_eq!(DEADLEG_CARTRIDGES, 4);
        assert!(DEADLEG_STUBS_PER_CARTRIDGE >= 3);
        assert!(FLUSH_VOLUME_TOKENS >= DEADLEG_CARTRIDGES + 2);
        assert_eq!(HIGH_STANDARD_WELLS, LOW_STANDARD_WELLS);
        assert_eq!(
            HIGH_STANDARD_WELLS + LOW_STANDARD_WELLS + BLANK_STANDARD_WELLS,
            10
        );
        assert!(PRESSURE_TAPS > FLOW_TAPS);
        assert_eq!(DISPOSITION_GATES, 3);
    }

    #[test]
    fn layout_fits_contained_robotic_station() {
        assert_layout();
        assert!(inside_deck(BAG_DOCK_POS, BAG_DOCK_X, BAG_DOCK_Y));
        assert!(inside_deck(
            CARTRIDGE_BANK_POS,
            CARTRIDGE_BANK_X,
            CARTRIDGE_BANK_Y
        ));
        assert!(inside_deck(WASTE_POS, WASTE_X, WASTE_Y));
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
    }

    #[test]
    fn geometry_spans_remain_printable_and_nonoverrunning() {
        assert!(token_span() < TOKEN_RAIL_X - 52.0);
        assert!(cartridge_bank_span() < CARTRIDGE_BANK_X - 28.0);
        assert!(witness_window_span() < WITNESS_X - 80.0);
        assert!(tap_span(PRESSURE_TAPS) < TAP_BAR_X - 220.0);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }
}
