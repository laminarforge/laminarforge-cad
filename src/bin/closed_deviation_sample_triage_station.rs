use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed deviation sample triage station for culture automation investigations.
//
// Intent:
// - Receive sealed tubes, fractions, or sample-loop handoffs after environmental
//   excursions, failed fluid-path checks, contamination suspicion, or other
//   out-of-spec signals.
// - Keep custody labels, hold/retest/archive/reject segregation, cold aliquot
//   handling, photo evidence capture, clean/used separation, and robot/service
//   clearances mechanically visible.
// - Model product-concept packaging only. Biological decisions, diagnostics,
//   release criteria, sampling plans, and GMP validation remain outside CAD.
//
// Research assumptions from the Exa pass:
// - Deviation and quarantine workflows need explicit identity/status controls,
//   chain-of-custody records, and physically separated hold/retest/archive/reject
//   material paths.
// - Biospecimen aliquot workflows commonly require traceable sealed containers,
//   cold handling, and retained/archive samples.
// - Contamination-suspect or excursion samples should be isolated from clean
//   consumables while still allowing inspection/evidence capture.

const OUTPUTS: [&str; 12] = [
    "output/closed_deviation_sample_triage_station_leak_tray_base.stl",
    "output/closed_deviation_sample_triage_station_sealed_incoming_sample_receiver.stl",
    "output/closed_deviation_sample_triage_station_chain_of_custody_barcode_lands.stl",
    "output/closed_deviation_sample_triage_station_hold_retest_archive_reject_segregation.stl",
    "output/closed_deviation_sample_triage_station_cold_block_interface.stl",
    "output/closed_deviation_sample_triage_station_sterile_sample_split_aliquot_handoff.stl",
    "output/closed_deviation_sample_triage_station_contamination_suspect_isolation_cover.stl",
    "output/closed_deviation_sample_triage_station_evidence_photo_inspection_bridge.stl",
    "output/closed_deviation_sample_triage_station_robot_pick_datums.stl",
    "output/closed_deviation_sample_triage_station_clean_used_separation.stl",
    "output/closed_deviation_sample_triage_station_service_keepouts.stl",
    "output/closed_deviation_sample_triage_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_incoming_sample_receiver",
    "chain_of_custody_barcode_lands",
    "hold_retest_archive_reject_segregation",
    "small_cold_block_interface",
    "sterile_sample_split_aliquot_handoff",
    "contamination_suspect_isolation_cover",
    "evidence_photo_inspection_bridge",
    "leak_tray",
    "robot_pick_datums",
    "clean_used_separation",
    "service_keepouts",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 740.0;
const BASE_Z: f64 = 20.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 34.0;
const LEAK_SUMP_DEPTH: f64 = 9.0;
const DRAIN_PORT_D: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 5.2;

const RECEIVER_CENTER: (f64, f64) = (-300.0, 170.0);
const RECEIVER_X: f64 = 390.0;
const RECEIVER_Y: f64 = 210.0;
const RECEIVER_Z: f64 = 58.0;
const RECEIVER_TUBE_POSITIONS: usize = 6;
const RECEIVER_FRACTION_POSITIONS: usize = 6;
const RECEIVER_LOOP_POSITIONS: usize = 4;
const RECEIVER_POSITIONS: usize =
    RECEIVER_TUBE_POSITIONS + RECEIVER_FRACTION_POSITIONS + RECEIVER_LOOP_POSITIONS;
const RECEIVER_COLS: usize = 4;
const RECEIVER_PITCH_X: f64 = 78.0;
const RECEIVER_PITCH_Y: f64 = 48.0;
const TUBE_WELL_D: f64 = 13.5;
const FRACTION_WELL_D: f64 = 9.2;
const LOOP_SADDLE_X: f64 = 44.0;
const LOOP_SADDLE_Y: f64 = 10.0;

const SEG_CENTER: (f64, f64) = (315.0, 125.0);
const SEG_PANEL_X: f64 = 370.0;
const SEG_PANEL_Y: f64 = 285.0;
const SEG_PANEL_Z: f64 = 36.0;
const SEG_LANES: usize = 4;
const SEG_SLOTS_PER_LANE: usize = 4;
const SEG_TOTAL_SLOTS: usize = SEG_LANES * SEG_SLOTS_PER_LANE;
const STATUS_SLOT_X: f64 = 62.0;
const STATUS_SLOT_Y: f64 = 42.0;
const STATUS_SLOT_PITCH_X: f64 = 82.0;
const STATUS_SLOT_PITCH_Y: f64 = 54.0;
#[cfg(test)]
const SEG_LANE_GAP_MIN: f64 = 18.0;

const COLD_CENTER: (f64, f64) = (-355.0, -155.0);
const COLD_X: f64 = 220.0;
const COLD_Y: f64 = 150.0;
const COLD_Z: f64 = 42.0;
const COLD_ALIQUOT_ROWS: usize = 4;
const COLD_ALIQUOT_COLS: usize = 6;
const COLD_ALIQUOT_POSITIONS: usize = COLD_ALIQUOT_ROWS * COLD_ALIQUOT_COLS;
const COLD_ALIQUOT_PITCH_X: f64 = 28.0;
const COLD_ALIQUOT_PITCH_Y: f64 = 28.0;
const ALIQUOT_WELL_D: f64 = 8.0;
const COOLANT_BORE_D: f64 = 6.4;
const THERMOWELL_D: f64 = 3.6;

const SPLIT_CENTER: (f64, f64) = (-85.0, -180.0);
const SPLIT_X: f64 = 310.0;
const SPLIT_Y: f64 = 110.0;
const SPLIT_Z: f64 = 54.0;
const SPLIT_LANES: usize = 8;
const SPLIT_LANE_PITCH: f64 = 34.0;
const SPLIT_OUTPUTS_PER_LANE: usize = 2;
const SPLIT_ALIQUOT_PORTS: usize = SPLIT_LANES * SPLIT_OUTPUTS_PER_LANE;
const SAMPLE_TUBE_BORE_D: f64 = 5.8;
const VALVE_PAD_X: f64 = 18.0;
const VALVE_PAD_Y: f64 = 16.0;

const COVER_CENTER: (f64, f64) = (315.0, 120.0);
const COVER_X: f64 = 410.0;
const COVER_Y: f64 = 300.0;
const COVER_Z: f64 = 230.0;
const COVER_POST_T: f64 = 16.0;
const COVER_FLANGE_Z: f64 = 10.0;
const COVER_TOP_RAIL_Z: f64 = 18.0;
const FILTER_VENT_COUNT: usize = 2;

const BRIDGE_CENTER: (f64, f64) = (0.0, 30.0);
const BRIDGE_SPAN_X: f64 = 930.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_POST_X: f64 = 28.0;
const BRIDGE_UNDERSIDE_Z: f64 = 182.0;
const BRIDGE_BEAM_Z: f64 = 26.0;
const CAMERA_COUNT: usize = 3;
const LED_SEGMENTS: usize = 8;
const INSPECTION_TARGET_COUNT: usize = 4;

const SEPARATOR_X: f64 = 86.0;
const SEPARATOR_Y: f64 = 0.0;
const SEPARATOR_W: f64 = 14.0;
const SEPARATOR_LEN: f64 = 660.0;
const SEPARATOR_Z: f64 = 58.0;
const CLEAN_BIN_COUNT: usize = 3;
const USED_BIN_COUNT: usize = 3;

const BARCODE_LANDS: usize = 12;
const CUSTODY_CARD_SLOTS: usize = 6;
const STATUS_TOKEN_LANDS: usize = 8;
const ROBOT_PICK_DATUMS: usize = 10;
const SERVICE_KEEPOUTS: usize = 4;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_COLD_SERVICE_CLEARANCE: f64 = 180.0;
const RIGHT_QUARANTINE_SERVICE_CLEARANCE: f64 = 260.0;
const ROBOT_Z_CLEARANCE: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let receiver = sealed_incoming_sample_receiver();
    export(OUTPUTS[1], &receiver);

    let custody = chain_of_custody_barcode_lands();
    export(OUTPUTS[2], &custody);

    let segregation = hold_retest_archive_reject_segregation();
    export(OUTPUTS[3], &segregation);

    let cold_block = cold_block_interface();
    export(OUTPUTS[4], &cold_block);

    let split = sterile_sample_split_aliquot_handoff();
    export(OUTPUTS[5], &split);

    let cover = contamination_suspect_isolation_cover();
    export(OUTPUTS[6], &cover);

    let bridge = evidence_photo_inspection_bridge();
    export(OUTPUTS[7], &bridge);

    let datums = robot_pick_datums();
    export(OUTPUTS[8], &datums);

    let separation = clean_used_separation();
    export(OUTPUTS[9], &separation);

    let keepouts = service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + receiver.translate(
            RECEIVER_CENTER.0,
            RECEIVER_CENTER.1,
            deck_insert_z(RECEIVER_Z),
        ) + custody.translate(0.0, 0.0, BASE_Z + 4.0)
            + segregation.translate(SEG_CENTER.0, SEG_CENTER.1, deck_insert_z(SEG_PANEL_Z))
            + cold_block.translate(COLD_CENTER.0, COLD_CENTER.1, deck_insert_z(COLD_Z))
            + split.translate(SPLIT_CENTER.0, SPLIT_CENTER.1, deck_insert_z(SPLIT_Z))
            + cover.translate(COVER_CENTER.0, COVER_CENTER.1, BASE_Z / 2.0)
            + bridge.translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z / 2.0)
            + datums.translate(0.0, 0.0, BASE_Z + 3.0)
            + separation.translate(SEPARATOR_X, SEPARATOR_Y, deck_insert_z(SEPARATOR_Z))
            + keepouts.translate(0.0, 0.0, BASE_Z + 4.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed deviation sample triage station:");
    println!("  Deck/leak tray:              {STATION_X:.0}mm x {STATION_Y:.0}mm x {BASE_Z:.0}mm");
    println!(
        "  Incoming receiver:           {RECEIVER_POSITIONS} sealed sample positions ({RECEIVER_TUBE_POSITIONS} tube, {RECEIVER_FRACTION_POSITIONS} fraction, {RECEIVER_LOOP_POSITIONS} loop)"
    );
    println!(
        "  Segregation:                 hold/retest/archive/reject, {SEG_SLOTS_PER_LANE} slots each ({SEG_TOTAL_SLOTS} total)"
    );
    println!(
        "  Cold aliquot block:          {COLD_X:.0}mm x {COLD_Y:.0}mm x {COLD_Z:.0}mm, {COLD_ALIQUOT_POSITIONS} aliquot wells"
    );
    println!(
        "  Split handoff:               {SPLIT_LANES} closed split lanes, {SPLIT_ALIQUOT_PORTS} aliquot ports"
    );
    println!(
        "  Isolation/inspection:        {COVER_X:.0}mm x {COVER_Y:.0}mm x {COVER_Z:.0}mm cover envelope, {CAMERA_COUNT} cameras, {LED_SEGMENTS} LED segments"
    );
    println!(
        "  Traceability/service:        {BARCODE_LANDS} barcode lands, {CUSTODY_CARD_SLOTS} custody cards, {ROBOT_PICK_DATUMS} robot datums, {SERVICE_KEEPOUTS} service keepouts"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    for (name, center, width, depth) in insert_specs() {
        assert!(
            fits_on_station(center, width, depth, 14.0),
            "{name} exceeds station envelope"
        );
    }

    let receiver = rect(RECEIVER_CENTER, RECEIVER_X, RECEIVER_Y);
    let segregation = rect(SEG_CENTER, SEG_PANEL_X, SEG_PANEL_Y);
    let cold = rect(COLD_CENTER, COLD_X, COLD_Y);
    let split = rect(SPLIT_CENTER, SPLIT_X, SPLIT_Y);
    let separator = rect((SEPARATOR_X, SEPARATOR_Y), SEPARATOR_W, SEPARATOR_LEN);

    assert!(!rects_overlap(receiver, segregation));
    assert!(!rects_overlap(receiver, cold));
    assert!(!rects_overlap(cold, split));
    assert!(!rects_overlap(split, separator));
    assert!(!rects_overlap(segregation, separator));
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 5] {
    [
        (
            "sealed_incoming_sample_receiver",
            RECEIVER_CENTER,
            RECEIVER_X,
            RECEIVER_Y,
        ),
        (
            "hold_retest_archive_reject_segregation",
            SEG_CENTER,
            SEG_PANEL_X,
            SEG_PANEL_Y,
        ),
        ("cold_block_interface", COLD_CENTER, COLD_X, COLD_Y),
        (
            "sterile_sample_split_aliquot_handoff",
            SPLIT_CENTER,
            SPLIT_X,
            SPLIT_Y,
        ),
        (
            "contamination_suspect_isolation_cover",
            COVER_CENTER,
            COVER_X,
            COVER_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - margin
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - margin
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "closed_deviation_triage_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let sump = centered_cube(
        "closed_deviation_triage_recessed_leak_sump",
        STATION_X - 112.0,
        STATION_Y - 104.0,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z - LEAK_SUMP_DEPTH / 2.0);

    let drain = centered_cylinder(
        "closed_deviation_triage_front_drain_port",
        DRAIN_PORT_D / 2.0,
        54.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 16.0,
        BASE_Z - 7.0,
    );

    deck - sump - drain + tray_rim() + leak_witness_ribs() + module_mount_bosses()
}

fn tray_rim() -> Part {
    let left = centered_cube(
        "closed_deviation_triage_left_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_deviation_triage_right_raised_lip",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_deviation_triage_rear_raised_lip",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_deviation_triage_front_low_service_lip",
        STATION_X - 160.0,
        10.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 8.0, BASE_Z + 9.0);

    left + right + rear + front
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_deviation_triage_leak_witness_ribs");
    for (i, x) in [-420.0, -280.0, -140.0, 0.0, 140.0, 280.0, 420.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_deviation_triage_sump_witness_rib_{i}"),
                8.0,
                STATION_Y - 150.0,
                5.0,
            )
            .translate(x, 0.0, BASE_Z + 2.5);
    }

    let drain_gutter = centered_cube("closed_deviation_triage_drain_gutter", 180.0, 8.0, 7.0)
        .translate(
            STATION_X / 2.0 - 150.0,
            -STATION_Y / 2.0 + 48.0,
            BASE_Z + 3.5,
        );

    ribs + drain_gutter
}

fn module_mount_bosses() -> Part {
    let mut bosses = Part::empty("closed_deviation_triage_module_mount_bosses");
    for (i, (x, y)) in datum_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_deviation_triage_module_mount_boss_{i}"),
            10.0,
            8.0,
            28,
        )
        .translate(x, y, BASE_Z + 4.0);
        let clearance = centered_cylinder(
            format!("closed_deviation_triage_module_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z + 4.0);
        bosses = bosses + (boss - clearance);
    }
    bosses
}

fn sealed_incoming_sample_receiver() -> Part {
    let body = centered_cube(
        "closed_deviation_triage_receiver_body",
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
    );

    let gasket_groove = centered_cube(
        "closed_deviation_triage_receiver_gasket_groove",
        RECEIVER_X - 38.0,
        RECEIVER_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, RECEIVER_Z / 2.0 - 3.0);

    let mut cuts = Part::empty("closed_deviation_triage_receiver_well_cuts");
    let mut rims = Part::empty("closed_deviation_triage_receiver_position_rims");
    for index in 0..RECEIVER_POSITIONS {
        let (x, y) = receiver_grid_position(index);
        let (well_d, name) = if index < RECEIVER_TUBE_POSITIONS {
            (TUBE_WELL_D, "tube")
        } else if index < RECEIVER_TUBE_POSITIONS + RECEIVER_FRACTION_POSITIONS {
            (FRACTION_WELL_D, "fraction")
        } else {
            (TUBE_WELL_D - 1.5, "loop")
        };
        let well = centered_cylinder(
            format!("closed_deviation_triage_receiver_{name}_well_{index}"),
            well_d / 2.0,
            RECEIVER_Z + 3.0,
            28,
        )
        .translate(x, y, 0.0);
        cuts = cuts + well;

        let rim = centered_cylinder(
            format!("closed_deviation_triage_receiver_{name}_seal_rim_{index}"),
            well_d / 2.0 + 2.4,
            3.0,
            28,
        )
        .translate(x, y, RECEIVER_Z / 2.0 + 1.5);
        let rim_opening = centered_cylinder(
            format!("closed_deviation_triage_receiver_{name}_seal_opening_{index}"),
            well_d / 2.0 + 0.4,
            3.4,
            28,
        )
        .translate(x, y, RECEIVER_Z / 2.0 + 1.5);
        rims = rims + (rim - rim_opening);
    }

    let mut loop_saddles = Part::empty("closed_deviation_triage_receiver_loop_saddles");
    for lane in 0..RECEIVER_LOOP_POSITIONS {
        let x = receiver_grid_x(RECEIVER_TUBE_POSITIONS + RECEIVER_FRACTION_POSITIONS + lane);
        let y = receiver_grid_y(RECEIVER_TUBE_POSITIONS + RECEIVER_FRACTION_POSITIONS + lane);
        loop_saddles = loop_saddles
            + centered_cube(
                format!("closed_deviation_triage_receiver_loop_saddle_{lane}"),
                LOOP_SADDLE_X,
                LOOP_SADDLE_Y,
                4.0,
            )
            .translate(x, y - 18.0, RECEIVER_Z / 2.0 + 2.0);
    }

    let sealed_tote_lip = centered_cube(
        "closed_deviation_triage_receiver_sealed_tote_front_lip",
        RECEIVER_X - 42.0,
        14.0,
        18.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 + 18.0, RECEIVER_Z / 2.0 + 9.0);

    let clamp_rails = centered_cube(
        "closed_deviation_triage_receiver_left_clamp_rail",
        12.0,
        RECEIVER_Y - 48.0,
        18.0,
    )
    .translate(-RECEIVER_X / 2.0 + 22.0, 0.0, RECEIVER_Z / 2.0 + 9.0)
        + centered_cube(
            "closed_deviation_triage_receiver_right_clamp_rail",
            12.0,
            RECEIVER_Y - 48.0,
            18.0,
        )
        .translate(RECEIVER_X / 2.0 - 22.0, 0.0, RECEIVER_Z / 2.0 + 9.0);

    body - gasket_groove - cuts + rims + loop_saddles + sealed_tote_lip + clamp_rails
}

fn chain_of_custody_barcode_lands() -> Part {
    let mut lands = Part::empty("closed_deviation_triage_chain_of_custody_lands");
    for (i, (x, y, w, h)) in barcode_land_specs().into_iter().enumerate() {
        let land = centered_cube(
            format!("closed_deviation_triage_barcode_land_{i}"),
            w,
            h,
            4.0,
        )
        .translate(x, y, 0.0);
        let scan_window = centered_cube(
            format!("closed_deviation_triage_barcode_scan_window_{i}"),
            w - 14.0,
            h - 6.0,
            5.0,
        )
        .translate(x, y, 0.6);
        lands = lands + (land - scan_window);
    }

    for i in 0..CUSTODY_CARD_SLOTS {
        let x = centered_index(i, CUSTODY_CARD_SLOTS, 82.0);
        let slot = centered_cube(
            format!("closed_deviation_triage_custody_card_slot_{i}"),
            64.0,
            28.0,
            8.0,
        )
        .translate(x, -STATION_Y / 2.0 + 74.0, 0.0);
        let card_relief = centered_cube(
            format!("closed_deviation_triage_custody_card_relief_{i}"),
            52.0,
            20.0,
            9.0,
        )
        .translate(x, -STATION_Y / 2.0 + 74.0, 1.0);
        lands = lands + (slot - card_relief);
    }

    for i in 0..STATUS_TOKEN_LANDS {
        let x = centered_index(i % 4, 4, 78.0) + SEG_CENTER.0;
        let y = SEG_CENTER.1 - SEG_PANEL_Y / 2.0 - 28.0 + (i / 4) as f64 * 28.0;
        lands = lands
            + centered_cube(
                format!("closed_deviation_triage_status_token_land_{i}"),
                56.0,
                14.0,
                4.0,
            )
            .translate(x, y, 0.0);
    }

    lands
}

fn hold_retest_archive_reject_segregation() -> Part {
    let panel = centered_cube(
        "closed_deviation_triage_status_segregation_panel",
        SEG_PANEL_X,
        SEG_PANEL_Y,
        SEG_PANEL_Z,
    );

    let mut cuts = Part::empty("closed_deviation_triage_status_slot_cuts");
    let mut dividers = Part::empty("closed_deviation_triage_status_lane_dividers");
    for lane in 0..SEG_LANES {
        let lane_x = status_lane_x(lane);
        for slot in 0..SEG_SLOTS_PER_LANE {
            let y = status_slot_y(slot);
            cuts = cuts
                + centered_cube(
                    format!("closed_deviation_triage_status_lane_{lane}_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    SEG_PANEL_Z + 2.0,
                )
                .translate(lane_x, y, 0.0);
        }

        dividers = dividers
            + centered_cube(
                format!("closed_deviation_triage_status_lane_backstop_{lane}"),
                STATUS_SLOT_X + 16.0,
                8.0,
                18.0,
            )
            .translate(lane_x, SEG_PANEL_Y / 2.0 - 18.0, SEG_PANEL_Z / 2.0 + 9.0);
    }

    for i in 0..SEG_LANES - 1 {
        let x = (status_lane_x(i) + status_lane_x(i + 1)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("closed_deviation_triage_status_lane_separator_{i}"),
                8.0,
                SEG_PANEL_Y - 38.0,
                28.0,
            )
            .translate(x, 0.0, SEG_PANEL_Z / 2.0 + 14.0);
    }

    let reject_high_wall = centered_cube(
        "closed_deviation_triage_reject_lane_high_wall",
        STATUS_SLOT_X + 30.0,
        SEG_PANEL_Y - 26.0,
        56.0,
    )
    .translate(status_lane_x(3), 0.0, SEG_PANEL_Z / 2.0 + 28.0);
    let reject_view_cut = centered_cube(
        "closed_deviation_triage_reject_lane_front_view_cut",
        STATUS_SLOT_X + 12.0,
        SEG_PANEL_Y - 60.0,
        50.0,
    )
    .translate(status_lane_x(3), 0.0, SEG_PANEL_Z / 2.0 + 34.0);

    panel - cuts + dividers + (reject_high_wall - reject_view_cut)
}

fn cold_block_interface() -> Part {
    let block = centered_cube(
        "closed_deviation_triage_cold_block_body",
        COLD_X,
        COLD_Y,
        COLD_Z,
    );

    let pocket = centered_cube(
        "closed_deviation_triage_cold_block_insulated_plate_pocket",
        COLD_X - 34.0,
        COLD_Y - 28.0,
        8.0,
    )
    .translate(0.0, 0.0, COLD_Z / 2.0 - 4.0);

    let mut cuts = Part::empty("closed_deviation_triage_cold_aliquot_well_cuts");
    let mut rims = Part::empty("closed_deviation_triage_cold_aliquot_well_rims");
    for row in 0..COLD_ALIQUOT_ROWS {
        for col in 0..COLD_ALIQUOT_COLS {
            let index = row * COLD_ALIQUOT_COLS + col;
            let x = centered_index(col, COLD_ALIQUOT_COLS, COLD_ALIQUOT_PITCH_X);
            let y = centered_index(row, COLD_ALIQUOT_ROWS, COLD_ALIQUOT_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("closed_deviation_triage_cold_aliquot_well_{index}"),
                    ALIQUOT_WELL_D / 2.0,
                    COLD_Z + 2.0,
                    24,
                )
                .translate(x, y, 0.0);
            rims = rims
                + centered_cylinder(
                    format!("closed_deviation_triage_cold_aliquot_retainer_{index}"),
                    ALIQUOT_WELL_D / 2.0 + 1.7,
                    2.0,
                    24,
                )
                .translate(x, y, COLD_Z / 2.0 + 1.0);
        }
    }

    let coolant_in = centered_cylinder(
        "closed_deviation_triage_cold_block_coolant_inlet",
        COOLANT_BORE_D / 2.0,
        COLD_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, COLD_Y / 2.0 - 22.0, -4.0);
    let coolant_out = centered_cylinder(
        "closed_deviation_triage_cold_block_coolant_return",
        COOLANT_BORE_D / 2.0,
        COLD_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -COLD_Y / 2.0 + 22.0, -4.0);
    let thermowell = centered_cylinder(
        "closed_deviation_triage_cold_block_thermowell",
        THERMOWELL_D / 2.0,
        COLD_Y + 6.0,
        18,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(COLD_X / 2.0 - 28.0, 0.0, 5.0);

    block - pocket - cuts - coolant_in - coolant_out - thermowell + rims + cold_block_gripper_tabs()
}

fn cold_block_gripper_tabs() -> Part {
    centered_cube(
        "closed_deviation_triage_cold_block_left_gripper_tab",
        18.0,
        58.0,
        12.0,
    )
    .translate(-COLD_X / 2.0 - 9.0, 0.0, COLD_Z / 2.0 - 2.0)
        + centered_cube(
            "closed_deviation_triage_cold_block_right_gripper_tab",
            18.0,
            58.0,
            12.0,
        )
        .translate(COLD_X / 2.0 + 9.0, 0.0, COLD_Z / 2.0 - 2.0)
}

fn sterile_sample_split_aliquot_handoff() -> Part {
    let body = centered_cube(
        "closed_deviation_triage_split_handoff_body",
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    );

    let mut ports = Part::empty("closed_deviation_triage_split_handoff_ports");
    let mut features = Part::empty("closed_deviation_triage_split_handoff_features");
    for lane in 0..SPLIT_LANES {
        let x = split_lane_x(lane);
        let inlet = centered_cylinder(
            format!("closed_deviation_triage_split_lane_{lane}_inlet_port"),
            SAMPLE_TUBE_BORE_D / 2.0,
            SPLIT_Z + 3.0,
            24,
        )
        .translate(x, SPLIT_Y / 2.0 - 24.0, 0.0);
        ports = ports + inlet;

        for output in 0..SPLIT_OUTPUTS_PER_LANE {
            let y = -18.0 - output as f64 * 26.0;
            ports = ports
                + centered_cylinder(
                    format!("closed_deviation_triage_split_lane_{lane}_aliquot_port_{output}"),
                    (SAMPLE_TUBE_BORE_D - 0.8) / 2.0,
                    SPLIT_Z + 3.0,
                    24,
                )
                .translate(x, y, 0.0);
        }

        features = features
            + centered_cube(
                format!("closed_deviation_triage_split_lane_{lane}_valve_pad"),
                VALVE_PAD_X,
                VALVE_PAD_Y,
                6.0,
            )
            .translate(x, 17.0, SPLIT_Z / 2.0 + 3.0)
            + centered_cube(
                format!("closed_deviation_triage_split_lane_{lane}_connector_saddle"),
                24.0,
                9.0,
                8.0,
            )
            .translate(x, SPLIT_Y / 2.0 - 8.0, SPLIT_Z / 2.0 + 4.0);
    }

    let waste_bypass_channel = centered_cylinder(
        "closed_deviation_triage_split_waste_bypass_channel",
        4.6,
        SPLIT_X - 42.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -SPLIT_Y / 2.0 + 18.0, -6.0);

    body - ports - waste_bypass_channel + features
}

fn contamination_suspect_isolation_cover() -> Part {
    let flange = centered_cube(
        "closed_deviation_triage_isolation_cover_base_flange",
        COVER_X,
        COVER_Y,
        COVER_FLANGE_Z,
    )
    .translate(0.0, 0.0, BASE_Z + COVER_FLANGE_Z / 2.0);

    let inner_relief = centered_cube(
        "closed_deviation_triage_isolation_cover_inner_relief",
        COVER_X - 44.0,
        COVER_Y - 44.0,
        COVER_FLANGE_Z + 2.0,
    )
    .translate(0.0, 0.0, BASE_Z + COVER_FLANGE_Z / 2.0);

    let mut frame = flange - inner_relief;
    let post_mid_z = BASE_Z + COVER_FLANGE_Z + (COVER_Z - COVER_FLANGE_Z) / 2.0;
    for (i, (x, y)) in [
        (
            -COVER_X / 2.0 + COVER_POST_T / 2.0,
            -COVER_Y / 2.0 + COVER_POST_T / 2.0,
        ),
        (
            COVER_X / 2.0 - COVER_POST_T / 2.0,
            -COVER_Y / 2.0 + COVER_POST_T / 2.0,
        ),
        (
            -COVER_X / 2.0 + COVER_POST_T / 2.0,
            COVER_Y / 2.0 - COVER_POST_T / 2.0,
        ),
        (
            COVER_X / 2.0 - COVER_POST_T / 2.0,
            COVER_Y / 2.0 - COVER_POST_T / 2.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        frame = frame
            + centered_cube(
                format!("closed_deviation_triage_isolation_cover_corner_post_{i}"),
                COVER_POST_T,
                COVER_POST_T,
                COVER_Z - COVER_FLANGE_Z,
            )
            .translate(x, y, post_mid_z);
    }

    let top_z = BASE_Z + COVER_Z - COVER_TOP_RAIL_Z / 2.0;
    frame = frame
        + centered_cube(
            "closed_deviation_triage_isolation_cover_top_front_rail",
            COVER_X,
            COVER_POST_T,
            COVER_TOP_RAIL_Z,
        )
        .translate(0.0, -COVER_Y / 2.0 + COVER_POST_T / 2.0, top_z)
        + centered_cube(
            "closed_deviation_triage_isolation_cover_top_rear_rail",
            COVER_X,
            COVER_POST_T,
            COVER_TOP_RAIL_Z,
        )
        .translate(0.0, COVER_Y / 2.0 - COVER_POST_T / 2.0, top_z)
        + centered_cube(
            "closed_deviation_triage_isolation_cover_top_left_rail",
            COVER_POST_T,
            COVER_Y,
            COVER_TOP_RAIL_Z,
        )
        .translate(-COVER_X / 2.0 + COVER_POST_T / 2.0, 0.0, top_z)
        + centered_cube(
            "closed_deviation_triage_isolation_cover_top_right_rail",
            COVER_POST_T,
            COVER_Y,
            COVER_TOP_RAIL_Z,
        )
        .translate(COVER_X / 2.0 - COVER_POST_T / 2.0, 0.0, top_z);

    for i in 0..FILTER_VENT_COUNT {
        let x = centered_index(i, FILTER_VENT_COUNT, 110.0);
        let vent = centered_cylinder(
            format!("closed_deviation_triage_isolation_cover_filter_vent_{i}"),
            23.0,
            12.0,
            36,
        )
        .translate(x, COVER_Y / 2.0 - 42.0, BASE_Z + COVER_Z + 6.0);
        let bore = centered_cylinder(
            format!("closed_deviation_triage_isolation_cover_filter_bore_{i}"),
            14.0,
            14.0,
            36,
        )
        .translate(x, COVER_Y / 2.0 - 42.0, BASE_Z + COVER_Z + 6.0);
        frame = frame + (vent - bore);
    }

    let photo_window_frame = centered_cube(
        "closed_deviation_triage_isolation_cover_photo_window_frame",
        160.0,
        10.0,
        62.0,
    )
    .translate(0.0, -COVER_Y / 2.0 + 6.0, BASE_Z + 118.0);
    let photo_window_cut = centered_cube(
        "closed_deviation_triage_isolation_cover_photo_window_opening",
        136.0,
        12.0,
        44.0,
    )
    .translate(0.0, -COVER_Y / 2.0 + 6.0, BASE_Z + 118.0);

    frame + (photo_window_frame - photo_window_cut)
}

fn evidence_photo_inspection_bridge() -> Part {
    let post_z = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
    let post_mid_z = post_z / 2.0;
    let left_post = centered_cube(
        "closed_deviation_triage_photo_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, post_mid_z);
    let right_post = centered_cube(
        "closed_deviation_triage_photo_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, post_mid_z);
    let beam = centered_cube(
        "closed_deviation_triage_photo_bridge_crossbeam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut cameras = Part::empty("closed_deviation_triage_photo_bridge_camera_sleds");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 250.0);
        cameras = cameras
            + centered_cube(
                format!("closed_deviation_triage_photo_bridge_camera_sled_{i}"),
                78.0,
                54.0,
                18.0,
            )
            .translate(x, -10.0, BRIDGE_UNDERSIDE_Z - 11.0)
            + centered_cylinder(
                format!("closed_deviation_triage_photo_bridge_lens_bore_{i}"),
                13.0,
                20.0,
                32,
            )
            .translate(x, -10.0, BRIDGE_UNDERSIDE_Z - 22.0);
    }

    let mut leds = Part::empty("closed_deviation_triage_photo_bridge_led_segments");
    for i in 0..LED_SEGMENTS {
        let x = centered_index(i, LED_SEGMENTS, 102.0);
        leds = leds
            + centered_cube(
                format!("closed_deviation_triage_photo_bridge_led_segment_{i}"),
                74.0,
                8.0,
                6.0,
            )
            .translate(x, BRIDGE_POST_Y / 2.0 + 5.0, BRIDGE_UNDERSIDE_Z - 12.0);
    }

    let mut targets = Part::empty("closed_deviation_triage_photo_bridge_calibration_targets");
    for i in 0..INSPECTION_TARGET_COUNT {
        let x = centered_index(i, INSPECTION_TARGET_COUNT, 140.0);
        targets = targets
            + centered_cube(
                format!("closed_deviation_triage_photo_bridge_calibration_target_{i}"),
                46.0,
                2.0,
                20.0,
            )
            .translate(x, -BRIDGE_POST_Y / 2.0 - 4.0, BRIDGE_UNDERSIDE_Z - 18.0);
    }

    left_post + right_post + beam + cameras + leds + targets
}

fn robot_pick_datums() -> Part {
    let mut datums = Part::empty("closed_deviation_triage_robot_pick_datums");
    for (i, (x, y)) in datum_points().into_iter().enumerate() {
        let pad = centered_cylinder(
            format!("closed_deviation_triage_robot_datum_pad_{i}"),
            9.0,
            4.0,
            32,
        )
        .translate(x, y, 0.0);
        let center = centered_cylinder(
            format!("closed_deviation_triage_robot_datum_center_mark_{i}"),
            2.0,
            5.0,
            20,
        )
        .translate(x, y, 0.0);
        let key = centered_cube(
            format!("closed_deviation_triage_robot_datum_key_{i}"),
            20.0,
            3.0,
            5.0,
        )
        .translate(x, y + 13.0, 0.0);
        datums = datums + (pad - center) + key;
    }
    datums
}

fn clean_used_separation() -> Part {
    let barrier = centered_cube(
        "closed_deviation_triage_clean_used_vertical_barrier",
        SEPARATOR_W,
        SEPARATOR_LEN,
        SEPARATOR_Z,
    );
    let clean_label = centered_cube("closed_deviation_triage_clean_side_land", 112.0, 16.0, 6.0)
        .translate(-54.0, -SEPARATOR_LEN / 2.0 + 34.0, SEPARATOR_Z / 2.0 + 3.0);
    let used_label = centered_cube("closed_deviation_triage_used_side_land", 112.0, 16.0, 6.0)
        .translate(54.0, -SEPARATOR_LEN / 2.0 + 34.0, SEPARATOR_Z / 2.0 + 3.0);

    let mut bins = Part::empty("closed_deviation_triage_clean_used_small_bins");
    for i in 0..CLEAN_BIN_COUNT {
        bins = bins
            + bin_pocket(
                &format!("closed_deviation_triage_clean_bin_{i}"),
                -82.0,
                centered_index(i, CLEAN_BIN_COUNT, 84.0),
            );
    }
    for i in 0..USED_BIN_COUNT {
        bins = bins
            + bin_pocket(
                &format!("closed_deviation_triage_used_bin_{i}"),
                82.0,
                centered_index(i, USED_BIN_COUNT, 84.0),
            );
    }

    barrier + clean_label + used_label + bins
}

fn bin_pocket(name: &str, x: f64, y: f64) -> Part {
    let body = centered_cube(format!("{name}_body"), 54.0, 46.0, 24.0).translate(
        x,
        y,
        -SEPARATOR_Z / 2.0 + 12.0,
    );
    let relief = centered_cube(format!("{name}_relief"), 42.0, 34.0, 18.0).translate(
        x,
        y,
        -SEPARATOR_Z / 2.0 + 16.0,
    );
    body - relief
}

fn service_keepouts() -> Part {
    let front = centered_cube(
        "closed_deviation_triage_front_robot_service_keepout",
        STATION_X - 120.0,
        10.0,
        8.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, 0.0);
    let rear = centered_cube(
        "closed_deviation_triage_rear_utility_service_keepout",
        STATION_X - 160.0,
        10.0,
        8.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE, 0.0);
    let left = centered_cube(
        "closed_deviation_triage_left_cold_block_service_keepout",
        10.0,
        STATION_Y - 160.0,
        8.0,
    )
    .translate(-STATION_X / 2.0 + LEFT_COLD_SERVICE_CLEARANCE, 0.0, 0.0);
    let right = centered_cube(
        "closed_deviation_triage_right_quarantine_service_keepout",
        10.0,
        STATION_Y - 180.0,
        8.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_QUARANTINE_SERVICE_CLEARANCE,
        0.0,
        0.0,
    );

    let z_gauge = centered_cube(
        "closed_deviation_triage_robot_z_clearance_gauge",
        120.0,
        120.0,
        8.0,
    )
    .translate(
        COVER_CENTER.0,
        COVER_CENTER.1,
        ROBOT_Z_CLEARANCE - BASE_Z - 4.0,
    );

    front + rear + left + right + z_gauge
}

fn barcode_land_specs() -> [(f64, f64, f64, f64); BARCODE_LANDS] {
    [
        (-466.0, 306.0, 118.0, 18.0),
        (-332.0, 306.0, 118.0, 18.0),
        (-198.0, 306.0, 118.0, 18.0),
        (-466.0, 42.0, 118.0, 18.0),
        (-332.0, 42.0, 118.0, 18.0),
        (-198.0, 42.0, 118.0, 18.0),
        (178.0, 294.0, 104.0, 18.0),
        (298.0, 294.0, 104.0, 18.0),
        (418.0, 294.0, 104.0, 18.0),
        (-452.0, -274.0, 112.0, 18.0),
        (-330.0, -274.0, 112.0, 18.0),
        (-208.0, -274.0, 112.0, 18.0),
    ]
}

fn datum_points() -> [(f64, f64); ROBOT_PICK_DATUMS] {
    [
        (-470.0, 286.0),
        (-136.0, 286.0),
        (-470.0, 54.0),
        (-136.0, 54.0),
        (172.0, 262.0),
        (492.0, 262.0),
        (-450.0, -250.0),
        (-260.0, -250.0),
        (-224.0, -246.0),
        (56.0, -246.0),
    ]
}

fn receiver_grid_position(index: usize) -> (f64, f64) {
    (receiver_grid_x(index), receiver_grid_y(index))
}

fn receiver_grid_x(index: usize) -> f64 {
    centered_index(index % RECEIVER_COLS, RECEIVER_COLS, RECEIVER_PITCH_X)
}

fn receiver_grid_y(index: usize) -> f64 {
    let rows = RECEIVER_POSITIONS.div_ceil(RECEIVER_COLS);
    centered_index(index / RECEIVER_COLS, rows, RECEIVER_PITCH_Y)
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, SEG_LANES, STATUS_SLOT_PITCH_X)
}

fn status_slot_y(slot: usize) -> f64 {
    centered_index(slot, SEG_SLOTS_PER_LANE, STATUS_SLOT_PITCH_Y)
}

fn split_lane_x(lane: usize) -> f64 {
    centered_index(lane, SPLIT_LANES, SPLIT_LANE_PITCH)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn rect(center: (f64, f64), width: f64, height: f64) -> Rect {
    Rect {
        x: center.0,
        y: center.1,
        w: width,
        h: height,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    (a.x - b.x).abs() < (a.w + b.w) / 2.0 && (a.y - b.y).abs() < (a.h + b.h) / 2.0
}

#[cfg(test)]
fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

#[cfg(test)]
fn lane_gap() -> f64 {
    STATUS_SLOT_PITCH_X - STATUS_SLOT_X
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_deviation_sample_triage_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_covers_required_deviation_triage_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"sealed_incoming_sample_receiver"));
        assert!(REQUIRED_FEATURES.contains(&"chain_of_custody_barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"hold_retest_archive_reject_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"small_cold_block_interface"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_sample_split_aliquot_handoff"));
        assert!(REQUIRED_FEATURES.contains(&"contamination_suspect_isolation_cover"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_photo_inspection_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"robot_pick_datums"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_separation"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn main_modules_fit_and_remain_segregated() {
        assert_layout();

        let receiver = rect(RECEIVER_CENTER, RECEIVER_X, RECEIVER_Y);
        let segregation = rect(SEG_CENTER, SEG_PANEL_X, SEG_PANEL_Y);
        let cold = rect(COLD_CENTER, COLD_X, COLD_Y);
        let split = rect(SPLIT_CENTER, SPLIT_X, SPLIT_Y);
        let separator = rect((SEPARATOR_X, SEPARATOR_Y), SEPARATOR_W, SEPARATOR_LEN);

        assert!(horizontal_gap(receiver, segregation) >= 30.0);
        assert!(horizontal_gap(cold, split) >= 4.0);
        assert!(horizontal_gap(split, separator) >= 4.0);
        assert!(horizontal_gap(separator, segregation) >= 30.0);
    }

    #[test]
    fn receiver_and_split_handoff_counts_match_closed_sample_scope() {
        assert_eq!(RECEIVER_POSITIONS, 16);
        assert_eq!(
            RECEIVER_POSITIONS,
            RECEIVER_TUBE_POSITIONS + RECEIVER_FRACTION_POSITIONS + RECEIVER_LOOP_POSITIONS
        );
        assert_eq!(SPLIT_LANES, 8);
        assert_eq!(SPLIT_ALIQUOT_PORTS, 16);
        assert_eq!(SPLIT_ALIQUOT_PORTS, SPLIT_LANES * SPLIT_OUTPUTS_PER_LANE);
        assert!(TUBE_WELL_D > FRACTION_WELL_D);
        assert!(SAMPLE_TUBE_BORE_D < TUBE_WELL_D);
    }

    #[test]
    fn hold_retest_archive_reject_paths_have_physical_separation() {
        assert_eq!(SEG_LANES, 4);
        assert_eq!(SEG_TOTAL_SLOTS, 16);
        assert_eq!(SEG_SLOTS_PER_LANE, 4);
        assert!(lane_gap() >= SEG_LANE_GAP_MIN);
        assert!(SEG_PANEL_X > SEG_LANES as f64 * STATUS_SLOT_X);
        assert!(SEG_PANEL_Y > SEG_SLOTS_PER_LANE as f64 * STATUS_SLOT_Y);
    }

    #[test]
    fn cold_block_and_traceability_counts_are_explicit() {
        assert_eq!(COLD_ALIQUOT_POSITIONS, 24);
        assert_eq!(
            COLD_ALIQUOT_POSITIONS,
            COLD_ALIQUOT_ROWS * COLD_ALIQUOT_COLS
        );
        assert!(COLD_X >= 200.0);
        assert!(COLD_Y >= 140.0);
        assert!(COOLANT_BORE_D > THERMOWELL_D);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(CUSTODY_CARD_SLOTS, 6);
        assert_eq!(STATUS_TOKEN_LANDS, 8);
    }

    #[test]
    fn cover_bridge_and_service_clearances_bound_the_station() {
        assert!(COVER_X >= SEG_PANEL_X);
        assert!(COVER_Y >= SEG_PANEL_Y);
        assert!(COVER_Z >= 220.0);
        assert!(BRIDGE_SPAN_X > RECEIVER_X + SEG_PANEL_X);
        assert!(BRIDGE_UNDERSIDE_Z >= 170.0);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LED_SEGMENTS, 8);
        assert_eq!(FILTER_VENT_COUNT, 2);
        assert!(FRONT_ROBOT_CLEARANCE >= 340.0);
        assert!(ROBOT_Z_CLEARANCE >= COVER_Z + BASE_Z);
        assert_eq!(SERVICE_KEEPOUTS, 4);
    }

    #[test]
    fn clean_and_used_paths_are_identified_and_balanced() {
        assert_eq!(CLEAN_BIN_COUNT, 3);
        assert_eq!(USED_BIN_COUNT, 3);
        assert!(SEPARATOR_LEN >= 640.0);
        assert!(SEPARATOR_Z >= 56.0);
        assert_eq!(ROBOT_PICK_DATUMS, 10);
        assert_eq!(datum_points().len(), ROBOT_PICK_DATUMS);
    }
}
