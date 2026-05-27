use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline metabolite sampling timing-aliasing validation station.
//
// This generator packages a mechanical validation fixture for inline sample
// loop handling, timed fraction capture, metabolite standard custody, analyzer
// handoff, and evidence capture around timing-aliasing checks. It models deck
// geometry, nests, gauges, custody lands, witness windows, keepout gauges, and
// disposition gates only. It is not an assay SOP, sterile-process claim,
// clinical release method, or biological acceptance criterion.

#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_inline_metabolite_sampling_timing_aliasing_station_";

const OUTPUTS: [&str; 14] = [
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_containment_deck.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_inline_sample_loop_cartridge_nests.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_timed_fraction_collection_wells.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_high_low_metabolite_standard_pockets.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_flow_step_token_rail.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_analyzer_handoff_dock.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_flush_waste_route.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_bubble_dead_volume_windows.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_timestamp_beacon_lands.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_barcode_coa_custody_lands.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_release_hold_reject_gates.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_camera_evidence_bridge.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_robot_service_keepouts.stl",
    "output/closed_inline_metabolite_sampling_timing_aliasing_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 23] = [
    "mechanical_validation_packaging_only",
    "inline_sample_loop_cartridge_nests",
    "sample_loop_length_keys",
    "timed_fraction_collection_wells",
    "fraction_time_index_ridges",
    "high_metabolite_standard_pockets",
    "low_metabolite_standard_pockets",
    "flow_step_token_rail",
    "analyzer_handoff_dock",
    "flush_waste_route",
    "segregated_flush_and_waste_paths",
    "bubble_windows",
    "dead_volume_windows",
    "timestamp_beacon_lands",
    "barcode_custody_lands",
    "coa_custody_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 4] = [
    "assay_standard_operating_procedure",
    "sterile_process_claim",
    "clinical_release_method",
    "biological_acceptance_criterion",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1160.0;
const SUMP_Y: f64 = 700.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const LOOP_BANK_X: f64 = 455.0;
const LOOP_BANK_Y: f64 = 205.0;
const LOOP_BANK_Z: f64 = 42.0;
const LOOP_BANK_POS: (f64, f64) = (-400.0, 245.0);
const SAMPLE_LOOP_CARTRIDGES: usize = 6;
const LOOP_CARTRIDGE_X: f64 = 58.0;
const LOOP_CARTRIDGE_Y: f64 = 148.0;
const LOOP_CARTRIDGE_Z: f64 = 36.0;
const LOOP_CARTRIDGE_PITCH: f64 = 66.0;
const LOOP_CHANNEL_D: f64 = 7.0;
const LOOP_DATUM_PINS_PER_CARTRIDGE: usize = 2;

const FRACTION_BANK_X: f64 = 560.0;
const FRACTION_BANK_Y: f64 = 220.0;
const FRACTION_BANK_Z: f64 = 44.0;
const FRACTION_BANK_POS: (f64, f64) = (135.0, 250.0);
const FRACTION_COLS: usize = 8;
const FRACTION_ROWS: usize = 3;
const FRACTION_WELLS: usize = FRACTION_COLS * FRACTION_ROWS;
const FRACTION_WELL_D: f64 = 24.0;
const FRACTION_WELL_PITCH_X: f64 = 48.0;
const FRACTION_WELL_PITCH_Y: f64 = 52.0;
const FRACTION_TIME_RIDGES: usize = FRACTION_COLS;

const STANDARD_X: f64 = 310.0;
const STANDARD_Y: f64 = 190.0;
const STANDARD_Z: f64 = 42.0;
const STANDARD_POS: (f64, f64) = (-480.0, -35.0);
const HIGH_STANDARD_POCKETS: usize = 4;
const LOW_STANDARD_POCKETS: usize = 4;
const BLANK_STANDARD_POCKETS: usize = 2;
const STANDARD_POCKET_D: f64 = 30.0;
const STANDARD_PITCH_X: f64 = 52.0;

const TOKEN_RAIL_X: f64 = 500.0;
const TOKEN_RAIL_Y: f64 = 100.0;
const TOKEN_RAIL_Z: f64 = 34.0;
const TOKEN_RAIL_POS: (f64, f64) = (-45.0, 45.0);
const FLOW_STEP_TOKENS: usize = 8;
const FLOW_STEP_PITCH: f64 = 54.0;
const FLOW_TOKEN_D: f64 = 28.0;

const ANALYZER_DOCK_X: f64 = 300.0;
const ANALYZER_DOCK_Y: f64 = 235.0;
const ANALYZER_DOCK_Z: f64 = 128.0;
const ANALYZER_DOCK_POS: (f64, f64) = (445.0, 10.0);
const ANALYZER_ENVELOPE_X: f64 = 230.0;
const ANALYZER_ENVELOPE_Y: f64 = 164.0;
const ANALYZER_ENVELOPE_Z: f64 = 104.0;
const HANDOFF_PORTS: usize = 4;
const HANDOFF_PORT_D: f64 = 18.0;
const HANDOFF_PORT_PITCH: f64 = 38.0;

const ROUTE_PLATE_X: f64 = 520.0;
const ROUTE_PLATE_Y: f64 = 150.0;
const ROUTE_PLATE_Z: f64 = 38.0;
const ROUTE_PLATE_POS: (f64, f64) = (-365.0, -250.0);
const FLUSH_CHANNELS: usize = 2;
const WASTE_CHANNELS: usize = 2;
const ROUTE_VALVE_SEATS: usize = 6;
const FLUSH_BORE_D: f64 = 5.0;
const WASTE_BORE_D: f64 = 8.0;
const ROUTE_PORT_D: f64 = 14.0;

const WITNESS_X: f64 = 455.0;
const WITNESS_Y: f64 = 128.0;
const WITNESS_Z: f64 = 32.0;
const WITNESS_POS: (f64, f64) = (135.0, -165.0);
const BUBBLE_WINDOWS: usize = 6;
const DEAD_VOLUME_WINDOWS: usize = 6;
const WINDOW_D: f64 = 27.0;
const WINDOW_PITCH: f64 = 48.0;

const TIMESTAMP_X: f64 = 175.0;
const TIMESTAMP_Y: f64 = 220.0;
const TIMESTAMP_Z: f64 = 18.0;
const TIMESTAMP_POS: (f64, f64) = (520.0, 285.0);
const TIMESTAMP_BEACONS: usize = 6;
const TIMESTAMP_LANE_PITCH: f64 = 32.0;
const TIMESTAMP_LAND_X: f64 = 42.0;
const TIMESTAMP_LAND_Y: f64 = 24.0;

const CUSTODY_X: f64 = 300.0;
const CUSTODY_Y: f64 = 80.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (480.0, -350.0);
const BARCODE_LANDS: usize = 5;
const COA_LANDS: usize = 3;
const TAMPER_SEAL_LANDS: usize = 4;

const GATE_X: f64 = 420.0;
const GATE_Y: f64 = 90.0;
const GATE_Z: f64 = 38.0;
const GATE_POS: (f64, f64) = (85.0, -360.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 1010.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 225.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -24.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 12;

const ROBOT_KEEPOUT_X: f64 = 1185.0;
const ROBOT_KEEPOUT_Y: f64 = 86.0;
const ROBOT_KEEPOUT_Z: f64 = 78.0;
const SERVICE_KEEPOUT_X: f64 = 100.0;
const SERVICE_KEEPOUT_Y: f64 = 690.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const ANALYZER_SERVICE_SWEEP_X: f64 = 360.0;
const ANALYZER_SERVICE_SWEEP_Y: f64 = 250.0;
const ANALYZER_SERVICE_SWEEP_Z: f64 = 72.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 315.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(inline_sample_loop_cartridge_nests(), OUTPUTS[1]);
    write_part(timed_fraction_collection_wells(), OUTPUTS[2]);
    write_part(high_low_metabolite_standard_pockets(), OUTPUTS[3]);
    write_part(flow_step_token_rail(), OUTPUTS[4]);
    write_part(analyzer_handoff_dock(), OUTPUTS[5]);
    write_part(flush_waste_route(), OUTPUTS[6]);
    write_part(bubble_dead_volume_windows(), OUTPUTS[7]);
    write_part(timestamp_beacon_lands(), OUTPUTS[8]);
    write_part(barcode_coa_custody_lands(), OUTPUTS[9]);
    write_part(release_hold_reject_gates(), OUTPUTS[10]);
    write_part(camera_evidence_bridge(), OUTPUTS[11]);
    write_part(robot_service_keepouts(), OUTPUTS[12]);
    write_part(station_assembly(), OUTPUTS[13]);

    println!(
        "Closed inline metabolite sampling timing-aliasing station: {:.0}mm x {:.0}mm contained deck, {} inline sample-loop cartridge nests, {} timed fraction wells.",
        DECK_X, DECK_Y, SAMPLE_LOOP_CARTRIDGES, FRACTION_WELLS
    );
    println!(
        "Mechanical validation packaging only: {} high metabolite standards, {} low metabolite standards, {} flow-step tokens, {} timestamp beacon lands.",
        HIGH_STANDARD_POCKETS, LOW_STANDARD_POCKETS, FLOW_STEP_TOKENS, TIMESTAMP_BEACONS
    );
    println!(
        "Evidence and custody: analyzer handoff dock, {} flush/waste channels, {} bubble windows, {} dead-volume windows, barcode/COA lands, release/hold/reject gates, robot/service keepouts.",
        FLUSH_CHANNELS + WASTE_CHANNELS,
        BUBBLE_WINDOWS,
        DEAD_VOLUME_WINDOWS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_inline_metabolite_timing_aliasing_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let shallow_sump = centered_cube(
        "closed_inline_metabolite_timing_aliasing_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_inline_metabolite_timing_aliasing_captured_sump_drain",
        DRAIN_D / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 72.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - shallow_sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_inline_metabolite_timing_aliasing_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_center_z());
    let right = centered_cube(
        "closed_inline_metabolite_timing_aliasing_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_inline_metabolite_timing_aliasing_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_inline_metabolite_timing_aliasing_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -(DECK_Y / 2.0 - RIM_W / 2.0), rim_center_z());

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_inline_metabolite_timing_aliasing_deck_datums");

    for i in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 82.0 + i as f64 * ((DECK_X - 164.0) / 9.0);
        let y = if i % 2 == 0 {
            DECK_Y / 2.0 - 72.0
        } else {
            -DECK_Y / 2.0 + 72.0
        };
        let boss = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_datum_boss_{i}"),
            7.0,
            5.0,
            32,
        )
        .translate(x, y, top_z(5.0));
        let bore = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_datum_bore_{i}"),
            1.8,
            9.0,
            20,
        )
        .translate(x, y, top_z(9.0));
        datums = datums + boss - bore;
    }

    datums
}

fn station_landing_pockets() -> Part {
    landing_pocket(
        "inline_sample_loop_cartridge_nest_land",
        LOOP_BANK_X,
        LOOP_BANK_Y,
        LOOP_BANK_POS,
    ) + landing_pocket(
        "timed_fraction_collection_land",
        FRACTION_BANK_X,
        FRACTION_BANK_Y,
        FRACTION_BANK_POS,
    ) + landing_pocket(
        "metabolite_standard_pocket_land",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_POS,
    ) + landing_pocket(
        "flow_step_token_rail_land",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_POS,
    ) + landing_pocket(
        "analyzer_handoff_dock_land",
        ANALYZER_DOCK_X,
        ANALYZER_DOCK_Y,
        ANALYZER_DOCK_POS,
    ) + landing_pocket(
        "flush_waste_route_land",
        ROUTE_PLATE_X,
        ROUTE_PLATE_Y,
        ROUTE_PLATE_POS,
    ) + landing_pocket(
        "bubble_dead_volume_window_land",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_POS,
    ) + landing_pocket(
        "timestamp_beacon_land",
        TIMESTAMP_X,
        TIMESTAMP_Y,
        TIMESTAMP_POS,
    ) + landing_pocket(
        "barcode_coa_custody_land",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_POS,
    ) + landing_pocket("disposition_gate_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("closed_inline_metabolite_timing_aliasing_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn inline_sample_loop_cartridge_nests() -> Part {
    let base = centered_cube(
        "closed_inline_metabolite_timing_aliasing_sample_loop_nest_bank",
        LOOP_BANK_X,
        LOOP_BANK_Y,
        LOOP_BANK_Z,
    );
    let slide_relief = centered_cube(
        "closed_inline_metabolite_timing_aliasing_loop_cartridge_slide_relief",
        LOOP_BANK_X - 40.0,
        28.0,
        16.0,
    )
    .translate(0.0, -LOOP_BANK_Y / 2.0 + 23.0, LOOP_BANK_Z / 2.0);
    let rear_datum_rail = centered_cube(
        "closed_inline_metabolite_timing_aliasing_loop_cartridge_rear_datum_rail",
        LOOP_BANK_X - 34.0,
        12.0,
        14.0,
    )
    .translate(0.0, LOOP_BANK_Y / 2.0 - 18.0, LOOP_BANK_Z / 2.0 + 6.0);

    let mut bank = base - slide_relief + rear_datum_rail;

    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = loop_cartridge_x(i);
        let pocket = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_cartridge_pocket_{i}"),
            LOOP_CARTRIDGE_X,
            LOOP_CARTRIDGE_Y,
            LOOP_CARTRIDGE_Z + 12.0,
        )
        .translate(x, 2.0, 8.0);
        let rim = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_cartridge_nest_rim_{i}"),
            LOOP_CARTRIDGE_X + 12.0,
            LOOP_CARTRIDGE_Y + 10.0,
            5.0,
        )
        .translate(x, 2.0, LOOP_BANK_Z / 2.0 + 4.0);
        let inlet_guide = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_inlet_guide_{i}"),
            LOOP_CHANNEL_D / 2.0,
            LOOP_CARTRIDGE_Y - 26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 13.0, 2.0, LOOP_BANK_Z / 2.0 + 15.0);
        let outlet_guide = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_outlet_guide_{i}"),
            LOOP_CHANNEL_D / 2.0,
            LOOP_CARTRIDGE_Y - 26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 13.0, 2.0, LOOP_BANK_Z / 2.0 + 15.0);
        let u_turn_witness = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_u_turn_witness_{i}"),
            18.0,
            7.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LOOP_CARTRIDGE_Y / 2.0 - 8.0, LOOP_BANK_Z / 2.0 + 15.0);
        let length_key = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_length_key_{i}"),
            14.0 + i as f64 * 3.0,
            22.0,
            8.0,
        )
        .translate(x, -LOOP_CARTRIDGE_Y / 2.0 + 20.0, LOOP_BANK_Z / 2.0 + 11.0);
        let latch = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_sample_loop_retaining_latch_{i}"),
            LOOP_CARTRIDGE_X - 16.0,
            8.0,
            12.0,
        )
        .translate(x, -LOOP_BANK_Y / 2.0 + 15.0, LOOP_BANK_Z / 2.0 + 9.0);

        bank =
            bank - pocket + rim + inlet_guide + outlet_guide + u_turn_witness + length_key + latch;

        for j in 0..LOOP_DATUM_PINS_PER_CARTRIDGE {
            let y = if j == 0 { -43.0 } else { 47.0 };
            let pin = centered_cylinder(
                format!(
                    "closed_inline_metabolite_timing_aliasing_loop_cartridge_datum_pin_{i}_{j}"
                ),
                3.2,
                8.0,
                20,
            )
            .translate(x, y, LOOP_BANK_Z / 2.0 + 7.0);
            bank = bank + pin;
        }
    }

    bank
}

fn timed_fraction_collection_wells() -> Part {
    let base = centered_cube(
        "closed_inline_metabolite_timing_aliasing_timed_fraction_collection_bank",
        FRACTION_BANK_X,
        FRACTION_BANK_Y,
        FRACTION_BANK_Z,
    );
    let pull_relief = centered_cube(
        "closed_inline_metabolite_timing_aliasing_fraction_plate_pull_relief",
        112.0,
        26.0,
        FRACTION_BANK_Z + 4.0,
    )
    .translate(0.0, -FRACTION_BANK_Y / 2.0 + 16.0, 6.0);
    let time_zero_datum = centered_cube(
        "closed_inline_metabolite_timing_aliasing_fraction_time_zero_datum",
        34.0,
        FRACTION_BANK_Y - 34.0,
        8.0,
    )
    .translate(
        -FRACTION_BANK_X / 2.0 + 31.0,
        0.0,
        FRACTION_BANK_Z / 2.0 + 5.0,
    );

    let mut bank = base - pull_relief + time_zero_datum;

    for i in 0..FRACTION_WELLS {
        let (x, y) = fraction_well_center(i);
        let cut = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_timed_fraction_well_cut_{i:02}"),
            FRACTION_WELL_D / 2.0,
            FRACTION_BANK_Z + 8.0,
            36,
        )
        .translate(x, y, 8.0);
        let collar = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_timed_fraction_well_collar_{i:02}"),
            FRACTION_WELL_D / 2.0 + 4.0,
            5.0,
            36,
        )
        .translate(x, y, FRACTION_BANK_Z / 2.0 + 4.0);
        let tick = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_timed_fraction_tick_{i:02}"),
            5.0,
            12.0,
            5.0,
        )
        .translate(x, y - 23.0, FRACTION_BANK_Z / 2.0 + 5.0);
        bank = bank - cut + collar + tick;
    }

    for i in 0..FRACTION_TIME_RIDGES {
        let x = grid_x(i, FRACTION_COLS, FRACTION_WELL_PITCH_X);
        let ridge = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_fraction_time_index_ridge_{i}"),
            18.0,
            FRACTION_BANK_Y - 44.0,
            6.0 + i as f64,
        )
        .translate(x, 0.0, FRACTION_BANK_Z / 2.0 + 5.0 + i as f64 / 2.0);
        bank = bank + ridge;
    }

    bank
}

fn high_low_metabolite_standard_pockets() -> Part {
    let base = centered_cube(
        "closed_inline_metabolite_timing_aliasing_high_low_metabolite_standard_pockets",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    );
    let divider = centered_cube(
        "closed_inline_metabolite_timing_aliasing_metabolite_standard_high_low_divider",
        STANDARD_X - 34.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, STANDARD_Z / 2.0 + 6.0);
    let blank_lane = centered_cube(
        "closed_inline_metabolite_timing_aliasing_metabolite_blank_reference_lane",
        42.0,
        STANDARD_Y - 36.0,
        8.0,
    )
    .translate(-STANDARD_X / 2.0 + 32.0, 0.0, STANDARD_Z / 2.0 + 5.0);

    let mut standards = base + divider + blank_lane;

    for i in 0..HIGH_STANDARD_POCKETS {
        let pocket = standard_pocket("high_metabolite_standard", i, 43.0, HIGH_STANDARD_POCKETS);
        standards = standards - pocket.0 + pocket.1;
    }
    for i in 0..LOW_STANDARD_POCKETS {
        let pocket = standard_pocket("low_metabolite_standard", i, -43.0, LOW_STANDARD_POCKETS);
        standards = standards - pocket.0 + pocket.1;
    }
    for i in 0..BLANK_STANDARD_POCKETS {
        let y = if i == 0 { 43.0 } else { -43.0 };
        let cut = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_blank_metabolite_standard_cut_{i}"),
            10.5,
            STANDARD_Z + 8.0,
            28,
        )
        .translate(-STANDARD_X / 2.0 + 32.0, y, 8.0);
        let land = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_blank_metabolite_standard_land_{i}"),
            14.0,
            4.0,
            28,
        )
        .translate(-STANDARD_X / 2.0 + 32.0, y, STANDARD_Z / 2.0 + 4.0);
        standards = standards - cut + land;
    }

    standards
}

fn standard_pocket(prefix: &str, index: usize, y: f64, count: usize) -> (Part, Part) {
    let x = grid_x(index, count, STANDARD_PITCH_X) + 28.0;
    let cut = centered_cylinder(
        format!("closed_inline_metabolite_timing_aliasing_{prefix}_pocket_cut_{index}"),
        STANDARD_POCKET_D / 2.0,
        STANDARD_Z + 8.0,
        36,
    )
    .translate(x, y, 8.0);
    let collar = centered_cylinder(
        format!("closed_inline_metabolite_timing_aliasing_{prefix}_pocket_collar_{index}"),
        STANDARD_POCKET_D / 2.0 + 4.0,
        5.0,
        36,
    )
    .translate(x, y, STANDARD_Z / 2.0 + 4.0);
    (cut, collar)
}

fn flow_step_token_rail() -> Part {
    let rail = centered_cube(
        "closed_inline_metabolite_timing_aliasing_flow_step_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let sight_slot = centered_cube(
        "closed_inline_metabolite_timing_aliasing_flow_step_sight_slot",
        TOKEN_RAIL_X - 56.0,
        18.0,
        16.0,
    )
    .translate(0.0, -9.0, TOKEN_RAIL_Z / 2.0);
    let home_stop = centered_cube(
        "closed_inline_metabolite_timing_aliasing_flow_step_home_stop",
        16.0,
        TOKEN_RAIL_Y,
        24.0,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 22.0, 0.0, TOKEN_RAIL_Z / 2.0 + 8.0);

    let mut rail_part = rail - sight_slot + home_stop;

    for i in 0..FLOW_STEP_TOKENS {
        let x = flow_token_x(i);
        let token_well = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_flow_step_token_well_{i}"),
            FLOW_TOKEN_D / 2.0,
            TOKEN_RAIL_Z + 10.0,
            36,
        )
        .translate(x, 22.0, 6.0);
        let token = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_flow_step_token_{i}"),
            FLOW_TOKEN_D / 2.0 - 3.0,
            6.0,
            36,
        )
        .translate(x, 22.0, TOKEN_RAIL_Z / 2.0 + 5.0);
        let step_gauge = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_flow_step_gauge_{i}"),
            12.0,
            32.0,
            5.0 + i as f64 * 1.6,
        )
        .translate(
            x,
            -TOKEN_RAIL_Y / 2.0 + 22.0,
            TOKEN_RAIL_Z / 2.0 + 2.5 + i as f64 * 0.8,
        );
        rail_part = rail_part - token_well + token + step_gauge;
    }

    rail_part
}

fn analyzer_handoff_dock() -> Part {
    let base = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_handoff_dock_base",
        ANALYZER_DOCK_X,
        ANALYZER_DOCK_Y,
        28.0,
    );
    let left_rail = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_handoff_left_rail",
        18.0,
        ANALYZER_DOCK_Y - 40.0,
        ANALYZER_DOCK_Z,
    )
    .translate(
        -ANALYZER_ENVELOPE_X / 2.0 - 16.0,
        0.0,
        ANALYZER_DOCK_Z / 2.0,
    );
    let right_rail = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_handoff_right_rail",
        18.0,
        ANALYZER_DOCK_Y - 40.0,
        ANALYZER_DOCK_Z,
    )
    .translate(ANALYZER_ENVELOPE_X / 2.0 + 16.0, 0.0, ANALYZER_DOCK_Z / 2.0);
    let rear_stop = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_handoff_rear_stop",
        ANALYZER_DOCK_X - 44.0,
        20.0,
        76.0,
    )
    .translate(0.0, ANALYZER_DOCK_Y / 2.0 - 28.0, 56.0);
    let analyzer_envelope_gauge = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_handoff_envelope_gauge",
        ANALYZER_ENVELOPE_X,
        ANALYZER_ENVELOPE_Y,
        ANALYZER_ENVELOPE_Z,
    )
    .translate(0.0, -8.0, ANALYZER_ENVELOPE_Z / 2.0 + 18.0);
    let front_handoff_chute = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_sample_handoff_chute",
        118.0,
        32.0,
        26.0,
    )
    .translate(0.0, -ANALYZER_DOCK_Y / 2.0 + 24.0, 34.0);

    let mut dock =
        base + left_rail + right_rail + rear_stop + analyzer_envelope_gauge - front_handoff_chute;

    for i in 0..HANDOFF_PORTS {
        let x = grid_x(i, HANDOFF_PORTS, HANDOFF_PORT_PITCH);
        let port = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_analyzer_handoff_port_boss_{i}"),
            HANDOFF_PORT_D / 2.0,
            24.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -ANALYZER_DOCK_Y / 2.0 + 10.0, 46.0);
        let bore = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_analyzer_handoff_port_bore_{i}"),
            3.6,
            34.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -ANALYZER_DOCK_Y / 2.0 + 10.0, 46.0);
        let custody_tab = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_analyzer_handoff_port_tab_{i}"),
            24.0,
            8.0,
            12.0,
        )
        .translate(x, -ANALYZER_DOCK_Y / 2.0 + 32.0, 62.0);
        dock = dock + port - bore + custody_tab;
    }

    dock
}

fn flush_waste_route() -> Part {
    let plate = centered_cube(
        "closed_inline_metabolite_timing_aliasing_flush_waste_route_plate",
        ROUTE_PLATE_X,
        ROUTE_PLATE_Y,
        ROUTE_PLATE_Z,
    );
    let flush_lane = centered_cylinder(
        "closed_inline_metabolite_timing_aliasing_flush_route_channel",
        FLUSH_BORE_D / 2.0,
        ROUTE_PLATE_X - 50.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 28.0, ROUTE_PLATE_Z / 2.0 + 5.0);
    let waste_lane = centered_cylinder(
        "closed_inline_metabolite_timing_aliasing_waste_route_channel",
        WASTE_BORE_D / 2.0,
        ROUTE_PLATE_X - 50.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -28.0, ROUTE_PLATE_Z / 2.0 + 3.0);
    let segregation_wall = centered_cube(
        "closed_inline_metabolite_timing_aliasing_flush_waste_segregation_wall",
        ROUTE_PLATE_X - 44.0,
        8.0,
        22.0,
    )
    .translate(0.0, 0.0, ROUTE_PLATE_Z / 2.0 + 7.0);

    let mut route = plate + flush_lane + waste_lane + segregation_wall;

    for i in 0..ROUTE_VALVE_SEATS {
        let x = grid_x(i, ROUTE_VALVE_SEATS, 72.0);
        let seat = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_route_valve_seat_{i}"),
            ROUTE_PORT_D / 2.0,
            12.0,
            30,
        )
        .translate(x, 0.0, ROUTE_PLATE_Z / 2.0 + 7.0);
        let bore = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_route_valve_bore_{i}"),
            3.2,
            18.0,
            20,
        )
        .translate(x, 0.0, ROUTE_PLATE_Z / 2.0 + 7.0);
        route = route + seat - bore;
    }

    for i in 0..WASTE_CHANNELS {
        let capture = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_waste_capture_cell_{i}"),
            64.0,
            38.0,
            12.0,
        )
        .translate(
            ROUTE_PLATE_X / 2.0 - 52.0,
            -46.0 + i as f64 * 28.0,
            ROUTE_PLATE_Z / 2.0 + 6.0,
        );
        route = route + capture;
    }

    route
}

fn bubble_dead_volume_windows() -> Part {
    let frame = centered_cube(
        "closed_inline_metabolite_timing_aliasing_bubble_dead_volume_window_frame",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let backlight_slot = centered_cube(
        "closed_inline_metabolite_timing_aliasing_bubble_dead_volume_backlight_slot",
        WITNESS_X - 58.0,
        18.0,
        18.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);

    let mut witness = frame - backlight_slot;

    for i in 0..BUBBLE_WINDOWS {
        let x = grid_x(i, BUBBLE_WINDOWS, WINDOW_PITCH);
        let window = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_bubble_window_cut_{i}"),
            WINDOW_D / 2.0,
            WITNESS_Z + 8.0,
            36,
        )
        .translate(x, 34.0, 6.0);
        let lens_land = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_bubble_window_lens_land_{i}"),
            WINDOW_D / 2.0 + 4.0,
            4.0,
            36,
        )
        .translate(x, 34.0, WITNESS_Z / 2.0 + 3.0);
        witness = witness - window + lens_land;
    }

    for i in 0..DEAD_VOLUME_WINDOWS {
        let x = grid_x(i, DEAD_VOLUME_WINDOWS, WINDOW_PITCH);
        let window = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_dead_volume_window_cut_{i}"),
            30.0,
            22.0,
            WITNESS_Z + 8.0,
        )
        .translate(x, -36.0, 6.0);
        let witness_pad = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_dead_volume_witness_pad_{i}"),
            34.0,
            26.0,
            4.0,
        )
        .translate(x, -36.0, WITNESS_Z / 2.0 + 4.0);
        witness = witness - window + witness_pad;
    }

    witness
}

fn timestamp_beacon_lands() -> Part {
    let rail = centered_cube(
        "closed_inline_metabolite_timing_aliasing_timestamp_beacon_land_rail",
        TIMESTAMP_X,
        TIMESTAMP_Y,
        TIMESTAMP_Z,
    );
    let fiducial_spine = centered_cube(
        "closed_inline_metabolite_timing_aliasing_timestamp_fiducial_spine",
        16.0,
        TIMESTAMP_Y - 28.0,
        14.0,
    )
    .translate(-TIMESTAMP_X / 2.0 + 20.0, 0.0, TIMESTAMP_Z / 2.0 + 6.0);

    let mut lands = rail + fiducial_spine;

    for i in 0..TIMESTAMP_BEACONS {
        let y = grid_x(i, TIMESTAMP_BEACONS, TIMESTAMP_LANE_PITCH);
        let pad = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_timestamp_beacon_land_{i}"),
            TIMESTAMP_LAND_X,
            TIMESTAMP_LAND_Y,
            5.0,
        )
        .translate(26.0, y, TIMESTAMP_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_timestamp_beacon_bore_{i}"),
            2.2,
            9.0,
            18,
        )
        .translate(26.0, y, TIMESTAMP_Z / 2.0 + 5.0);
        let tick = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_timestamp_beacon_tick_{i}"),
            22.0 + i as f64 * 3.0,
            4.0,
            4.0,
        )
        .translate(-38.0, y, TIMESTAMP_Z / 2.0 + 4.0);
        lands = lands + pad - bore + tick;
    }

    lands
}

fn barcode_coa_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_inline_metabolite_timing_aliasing_barcode_coa_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    let mut custody = panel;

    for i in 0..BARCODE_LANDS {
        let land = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_barcode_custody_land_{i}"),
            42.0,
            18.0,
            4.0,
        )
        .translate(grid_x(i, BARCODE_LANDS, 48.0), 20.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..COA_LANDS {
        let land = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_coa_custody_land_{i}"),
            70.0,
            24.0,
            4.0,
        )
        .translate(grid_x(i, COA_LANDS, 86.0), -18.0, CUSTODY_Z / 2.0 + 4.0);
        custody = custody + land;
    }

    for i in 0..TAMPER_SEAL_LANDS {
        let pad = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_tamper_seal_land_{i}"),
            7.0,
            4.0,
            24,
        )
        .translate(
            -CUSTODY_X / 2.0 + 22.0 + i as f64 * 22.0,
            -CUSTODY_Y / 2.0 + 12.0,
            CUSTODY_Z / 2.0 + 4.0,
        );
        custody = custody + pad;
    }

    custody
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_inline_metabolite_timing_aliasing_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut gates = base;

    for i in 0..DISPOSITION_GATES {
        let name = disposition_gate_name(i);
        let x = grid_x(i, DISPOSITION_GATES, 120.0);
        let slider = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_{name}_gate_slider"),
            80.0,
            48.0,
            12.0,
        )
        .translate(x, 6.0, GATE_Z / 2.0 + 8.0);
        let flag = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_{name}_gate_flag_land"),
            58.0,
            12.0,
            28.0,
        )
        .translate(x, -GATE_Y / 2.0 + 14.0, GATE_Z / 2.0 + 16.0);
        gates = gates + slider + flag;
    }

    for i in 0..GATE_TOKEN_SLOTS {
        let slot = centered_cube(
            format!("closed_inline_metabolite_timing_aliasing_disposition_token_slot_{i}"),
            32.0,
            14.0,
            GATE_Z + 8.0,
        )
        .translate(grid_x(i, GATE_TOKEN_SLOTS, 52.0), GATE_Y / 2.0 - 21.0, 6.0);
        gates = gates - slot;
    }

    gates
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_inline_metabolite_timing_aliasing_camera_bridge_left_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_inline_metabolite_timing_aliasing_camera_bridge_right_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "closed_inline_metabolite_timing_aliasing_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        34.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 17.0);

    let mut bridge = left_post + right_post + beam;

    for i in 0..CAMERA_MOUNTS {
        let x = grid_x(i, CAMERA_MOUNTS, 184.0);
        let mount = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_camera_mount_{i}"),
            15.0,
            9.0,
            32,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 5.0, CAMERA_BRIDGE_Z - 38.0);
        let bore = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_camera_mount_bore_{i}"),
            3.0,
            13.0,
            18,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 5.0, CAMERA_BRIDGE_Z - 38.0);
        bridge = bridge + mount - bore;
    }

    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -CAMERA_BRIDGE_X / 2.0 + 70.0 + i as f64 * ((CAMERA_BRIDGE_X - 140.0) / 11.0);
        let fiducial = centered_cylinder(
            format!("closed_inline_metabolite_timing_aliasing_evidence_fiducial_{i}"),
            5.0,
            4.0,
            20,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 8.0, CAMERA_BRIDGE_Z - 21.0);
        bridge = bridge + fiducial;
    }

    bridge
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_inline_metabolite_timing_aliasing_front_robot_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_inline_metabolite_timing_aliasing_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "closed_inline_metabolite_timing_aliasing_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let analyzer_sweep = centered_cube(
        "closed_inline_metabolite_timing_aliasing_analyzer_service_sweep_keepout",
        ANALYZER_SERVICE_SWEEP_X,
        ANALYZER_SERVICE_SWEEP_Y,
        ANALYZER_SERVICE_SWEEP_Z,
    )
    .translate(
        ANALYZER_DOCK_POS.0,
        ANALYZER_DOCK_POS.1,
        ANALYZER_SERVICE_SWEEP_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "closed_inline_metabolite_timing_aliasing_top_service_clearance_gauge",
        720.0,
        430.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + left_service + right_service + analyzer_sweep + top_clearance
}

fn station_assembly() -> Part {
    containment_deck()
        + inline_sample_loop_cartridge_nests().translate(
            LOOP_BANK_POS.0,
            LOOP_BANK_POS.1,
            top_z(LOOP_BANK_Z),
        )
        + timed_fraction_collection_wells().translate(
            FRACTION_BANK_POS.0,
            FRACTION_BANK_POS.1,
            top_z(FRACTION_BANK_Z),
        )
        + high_low_metabolite_standard_pockets().translate(
            STANDARD_POS.0,
            STANDARD_POS.1,
            top_z(STANDARD_Z),
        )
        + flow_step_token_rail().translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, top_z(TOKEN_RAIL_Z))
        + analyzer_handoff_dock().translate(ANALYZER_DOCK_POS.0, ANALYZER_DOCK_POS.1, top_z(28.0))
        + flush_waste_route().translate(ROUTE_PLATE_POS.0, ROUTE_PLATE_POS.1, top_z(ROUTE_PLATE_Z))
        + bubble_dead_volume_windows().translate(WITNESS_POS.0, WITNESS_POS.1, top_z(WITNESS_Z))
        + timestamp_beacon_lands().translate(TIMESTAMP_POS.0, TIMESTAMP_POS.1, top_z(TIMESTAMP_Z))
        + barcode_coa_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, top_z(CUSTODY_Z))
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge().translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, DECK_Z / 2.0)
        + robot_service_keepouts()
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn grid_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn grid_center(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (grid_x(col, cols, pitch_x), grid_x(row, rows, pitch_y))
}

fn loop_cartridge_x(index: usize) -> f64 {
    grid_x(index, SAMPLE_LOOP_CARTRIDGES, LOOP_CARTRIDGE_PITCH)
}

fn flow_token_x(index: usize) -> f64 {
    grid_x(index, FLOW_STEP_TOKENS, FLOW_STEP_PITCH)
}

fn fraction_well_center(index: usize) -> (f64, f64) {
    grid_center(
        index,
        FRACTION_COLS,
        FRACTION_ROWS,
        FRACTION_WELL_PITCH_X,
        FRACTION_WELL_PITCH_Y,
    )
}

fn loop_cartridge_span() -> f64 {
    (SAMPLE_LOOP_CARTRIDGES as f64 - 1.0) * LOOP_CARTRIDGE_PITCH + LOOP_CARTRIDGE_X
}

fn fraction_well_span_x() -> f64 {
    (FRACTION_COLS as f64 - 1.0) * FRACTION_WELL_PITCH_X + FRACTION_WELL_D
}

fn fraction_well_span_y() -> f64 {
    (FRACTION_ROWS as f64 - 1.0) * FRACTION_WELL_PITCH_Y + FRACTION_WELL_D
}

fn token_span() -> f64 {
    (FLOW_STEP_TOKENS as f64 - 1.0) * FLOW_STEP_PITCH + FLOW_TOKEN_D
}

fn witness_window_span() -> f64 {
    (BUBBLE_WINDOWS as f64 - 1.0) * WINDOW_PITCH + WINDOW_D
}

fn timestamp_span() -> f64 {
    (TIMESTAMP_BEACONS as f64 - 1.0) * TIMESTAMP_LANE_PITCH + TIMESTAMP_LAND_Y
}

#[cfg(test)]
fn rect_fits_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
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
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(FRACTION_WELLS, FRACTION_COLS * FRACTION_ROWS);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX, 0);
    assert_eq!(HOLD_GATE_INDEX, 1);
    assert_eq!(REJECT_GATE_INDEX, 2);
    assert_eq!(FLUSH_CHANNELS, WASTE_CHANNELS);
    assert_eq!(BUBBLE_WINDOWS, DEAD_VOLUME_WINDOWS);
    assert!(loop_cartridge_span() < LOOP_BANK_X - 36.0);
    assert!(fraction_well_span_x() < FRACTION_BANK_X - 110.0);
    assert!(fraction_well_span_y() < FRACTION_BANK_Y - 64.0);
    assert!(token_span() < TOKEN_RAIL_X - 52.0);
    assert!(witness_window_span() < WITNESS_X - 90.0);
    assert!(timestamp_span() < TIMESTAMP_Y - 28.0);
    assert!(ANALYZER_ENVELOPE_X < ANALYZER_DOCK_X - 40.0);
    assert!(ANALYZER_ENVELOPE_Y < ANALYZER_DOCK_Y - 40.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn feature_metadata_covers_requested_mechanical_station_scope() {
        assert_eq!(REQUIRED_FEATURES.len(), 23);
        assert!(REQUIRED_FEATURES.contains(&"mechanical_validation_packaging_only"));
        assert!(REQUIRED_FEATURES.contains(&"inline_sample_loop_cartridge_nests"));
        assert!(REQUIRED_FEATURES.contains(&"sample_loop_length_keys"));
        assert!(REQUIRED_FEATURES.contains(&"timed_fraction_collection_wells"));
        assert!(REQUIRED_FEATURES.contains(&"fraction_time_index_ridges"));
        assert!(REQUIRED_FEATURES.contains(&"high_metabolite_standard_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"low_metabolite_standard_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"flow_step_token_rail"));
        assert!(REQUIRED_FEATURES.contains(&"analyzer_handoff_dock"));
        assert!(REQUIRED_FEATURES.contains(&"flush_waste_route"));
        assert!(REQUIRED_FEATURES.contains(&"segregated_flush_and_waste_paths"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_windows"));
        assert!(REQUIRED_FEATURES.contains(&"dead_volume_windows"));
        assert!(REQUIRED_FEATURES.contains(&"timestamp_beacon_lands"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"coa_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_fiducials"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn scope_excludes_assay_sterile_clinical_and_biological_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 4);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"assay_standard_operating_procedure"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"clinical_release_method"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_acceptance_criterion"));
    }

    #[test]
    fn repeated_feature_counts_match_timing_aliasing_validation_packaging() {
        assert_eq!(SAMPLE_LOOP_CARTRIDGES, 6);
        assert_eq!(FRACTION_WELLS, 24);
        assert_eq!(HIGH_STANDARD_POCKETS, LOW_STANDARD_POCKETS);
        assert_eq!(
            HIGH_STANDARD_POCKETS + LOW_STANDARD_POCKETS + BLANK_STANDARD_POCKETS,
            10
        );
        assert_eq!(FLOW_STEP_TOKENS, 8);
        assert_eq!(TIMESTAMP_BEACONS, 6);
        assert_eq!(HANDOFF_PORTS, 4);
        assert_eq!(FLUSH_CHANNELS + WASTE_CHANNELS, 4);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(CAMERA_MOUNTS, 5);
        assert_eq!(EVIDENCE_FIDUCIALS, 12);
    }

    #[test]
    fn major_modules_fit_inside_contained_deck() {
        assert!(rect_fits_deck(
            LOOP_BANK_POS,
            LOOP_BANK_X,
            LOOP_BANK_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(
            FRACTION_BANK_POS,
            FRACTION_BANK_X,
            FRACTION_BANK_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(STANDARD_POS, STANDARD_X, STANDARD_Y, RIM_W));
        assert!(rect_fits_deck(
            TOKEN_RAIL_POS,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(
            ANALYZER_DOCK_POS,
            ANALYZER_DOCK_X,
            ANALYZER_DOCK_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(
            ROUTE_PLATE_POS,
            ROUTE_PLATE_X,
            ROUTE_PLATE_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(WITNESS_POS, WITNESS_X, WITNESS_Y, RIM_W));
        assert!(rect_fits_deck(
            TIMESTAMP_POS,
            TIMESTAMP_X,
            TIMESTAMP_Y,
            RIM_W
        ));
        assert!(rect_fits_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y, RIM_W));
        assert!(rect_fits_deck(GATE_POS, GATE_X, GATE_Y, RIM_W));
    }

    #[test]
    fn array_spans_remain_inside_their_banks() {
        assert!(loop_cartridge_span() < LOOP_BANK_X - 36.0);
        assert!(fraction_well_span_x() < FRACTION_BANK_X - 110.0);
        assert!(fraction_well_span_y() < FRACTION_BANK_Y - 64.0);
        assert!(token_span() < TOKEN_RAIL_X - 52.0);
        assert!(witness_window_span() < WITNESS_X - 90.0);
        assert!(timestamp_span() < TIMESTAMP_Y - 28.0);

        let first_fraction = fraction_well_center(0);
        let last_fraction = fraction_well_center(FRACTION_WELLS - 1);
        assert!(first_fraction.0.abs() < FRACTION_BANK_X / 2.0 - FRACTION_WELL_D);
        assert!(last_fraction.0.abs() < FRACTION_BANK_X / 2.0 - FRACTION_WELL_D);
        assert!(first_fraction.1.abs() < FRACTION_BANK_Y / 2.0 - FRACTION_WELL_D);
        assert!(last_fraction.1.abs() < FRACTION_BANK_Y / 2.0 - FRACTION_WELL_D);
    }

    #[test]
    fn analyzer_evidence_and_keepout_geometry_have_expected_clearance() {
        assert!(ANALYZER_ENVELOPE_X < ANALYZER_DOCK_X - 40.0);
        assert!(ANALYZER_ENVELOPE_Y < ANALYZER_DOCK_Y - 40.0);
        assert!(ANALYZER_ENVELOPE_Z < ANALYZER_DOCK_Z);
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(ANALYZER_SERVICE_SWEEP_X > ANALYZER_DOCK_X);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }

    #[test]
    fn layout_invariants_pass_for_runtime_export() {
        assert_layout();
    }
}
