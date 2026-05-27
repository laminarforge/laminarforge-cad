use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Cell-lot release QC panel for the closed automated tissue-chip workflow.
//
// Intent:
// - Mechanically package release-QC sample custody without reopening the cell
//   process to manual liquid handling.
// - Represent sealed sample receiving, count/viability analyzer docking,
//   sterility/mycoplasma custody, chain-of-identity barcode lands, status
//   segregation, environmental excursion evidence, archive aliquots, clean/used
//   separation, and closed handoff to upstream passaging/seeding modules.
//
// This is product-concept CAD only. It does not define biological release
// criteria, validated assays, sterility methods, or clinical disposition rules.
//
// Research assumptions from an Exa-only pass:
// - Cell-therapy release packages include batch-record review, Certificates of
//   Analysis, quality events, product holds/quarantine, and interim/final
//   sterility information.
// - Automated QC workcells are being commercialized to reduce manual release-QC
//   bottlenecks and need modular analyzer/instrument positions.
// - Count/viability analyzers used in CGT workflows can be cassette-based,
//   no-contact to the instrument, API/data-system integrated, and roughly
//   400mm W x 250mm D x 230mm H at small benchtop scale.

const OUTPUTS: &[&str] = &[
    "output/cell_lot_release_qc_panel_base_tray.stl",
    "output/cell_lot_release_qc_panel_sealed_sample_receiving_nest.stl",
    "output/cell_lot_release_qc_panel_count_viability_analyzer_dock.stl",
    "output/cell_lot_release_qc_panel_sterility_mycoplasma_custody_slots.stl",
    "output/cell_lot_release_qc_panel_identity_passage_lot_barcode_lands.stl",
    "output/cell_lot_release_qc_panel_released_hold_reject_status_lanes.stl",
    "output/cell_lot_release_qc_panel_environmental_excursion_evidence_slot.stl",
    "output/cell_lot_release_qc_panel_archive_aliquot_staging.stl",
    "output/cell_lot_release_qc_panel_clean_used_segregation.stl",
    "output/cell_lot_release_qc_panel_upstream_handoff_interface.stl",
    "output/cell_lot_release_qc_panel_robot_service_keepouts.stl",
    "output/cell_lot_release_qc_panel_assembly.stl",
];

const BASE_X: f64 = 1180.0;
const BASE_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 34.0;

const RECEIVING_POSITIONS: usize = 8;
const RECEIVING_X: f64 = 342.0;
const RECEIVING_Y: f64 = 238.0;
const RECEIVING_Z: f64 = 48.0;
const RECEIVING_CENTER_X: f64 = -386.0;
const RECEIVING_CENTER_Y: f64 = 114.0;
const RECEIVING_COLS: usize = 4;
const RECEIVING_ROWS: usize = 2;
const RECEIVING_PITCH_X: f64 = 72.0;
const RECEIVING_PITCH_Y: f64 = 74.0;
const SEALED_SAMPLE_POCKET_D: f64 = 32.0;
const SAMPLE_CAP_CLEARANCE_D: f64 = 40.0;

const ANALYZER_DOCK_X: f64 = 486.0;
const ANALYZER_DOCK_Y: f64 = 322.0;
const ANALYZER_DOCK_Z: f64 = 52.0;
const ANALYZER_ENV_X: f64 = 430.0;
const ANALYZER_ENV_Y: f64 = 282.0;
const ANALYZER_ENV_Z: f64 = 262.0;
const ANALYZER_CENTER_X: f64 = 252.0;
const ANALYZER_CENTER_Y: f64 = 120.0;
const VIABILITY_CASSETTES: usize = 12;
const CASSETTE_SLOT_PITCH: f64 = 24.0;

const CUSTODY_X: f64 = 334.0;
const CUSTODY_Y: f64 = 210.0;
const CUSTODY_Z: f64 = 44.0;
const CUSTODY_CENTER_X: f64 = -400.0;
const CUSTODY_CENTER_Y: f64 = -198.0;
const STERILITY_SLOTS: usize = 6;
const MYCOPLASMA_SLOTS: usize = 8;
const RETAIN_SLOTS: usize = 4;

const BARCODE_LANDS: usize = 12;
const BARCODE_LAND_X: f64 = 92.0;
const BARCODE_LAND_Y: f64 = 24.0;
const BARCODE_LAND_Z: f64 = 6.0;

const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 5;
const STATUS_X: f64 = 344.0;
const STATUS_Y: f64 = 230.0;
const STATUS_Z: f64 = 46.0;
const STATUS_CENTER_X: f64 = 386.0;
const STATUS_CENTER_Y: f64 = -202.0;
const STATUS_LANE_PITCH: f64 = 72.0;

const EVIDENCE_X: f64 = 310.0;
const EVIDENCE_Y: f64 = 86.0;
const EVIDENCE_Z: f64 = 30.0;
const EVIDENCE_CENTER_X: f64 = -170.0;
const EVIDENCE_CENTER_Y: f64 = 286.0;
const LOGGER_POCKET_X: f64 = 118.0;
const LOGGER_POCKET_Y: f64 = 52.0;
const COA_CARD_X: f64 = 152.0;
const COA_CARD_Y: f64 = 58.0;

const ARCHIVE_X: f64 = 312.0;
const ARCHIVE_Y: f64 = 220.0;
const ARCHIVE_Z: f64 = 50.0;
const ARCHIVE_CENTER_X: f64 = -38.0;
const ARCHIVE_CENTER_Y: f64 = -198.0;
const ARCHIVE_COLS: usize = 6;
const ARCHIVE_ROWS: usize = 4;
const ARCHIVE_TUBE_COUNT: usize = ARCHIVE_COLS * ARCHIVE_ROWS;
const ARCHIVE_PITCH_X: f64 = 39.0;
const ARCHIVE_PITCH_Y: f64 = 38.0;
const ARCHIVE_TUBE_D: f64 = 13.0;

const SEGREGATION_X: f64 = 1116.0;
const SEGREGATION_Y: f64 = 40.0;
const SEGREGATION_Z: f64 = 46.0;
const SEGREGATION_CENTER_X: f64 = 0.0;
const SEGREGATION_CENTER_Y: f64 = -356.0;

const HANDOFF_X: f64 = 910.0;
const HANDOFF_Y: f64 = 28.0;
const HANDOFF_Z: f64 = 148.0;
const HANDOFF_CENTER_X: f64 = 0.0;
const HANDOFF_CENTER_Y: f64 = BASE_Y / 2.0 - 34.0;
const HANDOFF_PORTS: usize = RECEIVING_POSITIONS;
const HANDOFF_PORT_PITCH: f64 = 86.0;

const ROBOT_FRONT_KEEP_OUT_Y: f64 = 280.0;
const SERVICE_KEEP_OUT_Z: f64 = 332.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 172.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let base = base_tray();
    export(&base, OUTPUTS[0]);

    let receiving = sealed_sample_receiving_nest();
    export(&receiving, OUTPUTS[1]);

    let analyzer = count_viability_analyzer_dock();
    export(&analyzer, OUTPUTS[2]);

    let custody = sterility_mycoplasma_custody_slots();
    export(&custody, OUTPUTS[3]);

    let barcode_lands = identity_passage_lot_barcode_lands();
    export(&barcode_lands, OUTPUTS[4]);

    let status_lanes = released_hold_reject_status_lanes();
    export(&status_lanes, OUTPUTS[5]);

    let evidence = environmental_excursion_evidence_slot();
    export(&evidence, OUTPUTS[6]);

    let archive = archive_aliquot_staging();
    export(&archive, OUTPUTS[7]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[8]);

    let handoff = upstream_handoff_interface();
    export(&handoff, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + receiving.translate(RECEIVING_CENTER_X, RECEIVING_CENTER_Y, DECK_Z / 2.0)
        + analyzer.translate(ANALYZER_CENTER_X, ANALYZER_CENTER_Y, DECK_Z / 2.0)
        + custody.translate(CUSTODY_CENTER_X, CUSTODY_CENTER_Y, DECK_Z / 2.0)
        + barcode_lands.translate(0.0, 0.0, DECK_Z / 2.0)
        + status_lanes.translate(STATUS_CENTER_X, STATUS_CENTER_Y, DECK_Z / 2.0)
        + evidence.translate(EVIDENCE_CENTER_X, EVIDENCE_CENTER_Y, DECK_Z / 2.0)
        + archive.translate(ARCHIVE_CENTER_X, ARCHIVE_CENTER_Y, DECK_Z / 2.0)
        + segregation.translate(SEGREGATION_CENTER_X, SEGREGATION_CENTER_Y, DECK_Z / 2.0)
        + handoff.translate(HANDOFF_CENTER_X, HANDOFF_CENTER_Y, DECK_Z / 2.0)
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Cell-lot release QC panel: {:.0} x {:.0} x {:.0}mm deck, {} sealed sample receiving positions, {:.0} x {:.0} x {:.0}mm count/viability analyzer envelope, {} sterility slots, {} mycoplasma slots, {} retain slots, {} barcode lands, {} release-status lanes with {} slots each, {} archive aliquots, {} upstream handoff ports, and {:.0}mm robot pick clearance.",
        BASE_X,
        BASE_Y,
        DECK_Z,
        RECEIVING_POSITIONS,
        ANALYZER_ENV_X,
        ANALYZER_ENV_Y,
        ANALYZER_ENV_Z,
        STERILITY_SLOTS,
        MYCOPLASMA_SLOTS,
        RETAIN_SLOTS,
        BARCODE_LANDS,
        STATUS_LANES,
        STATUS_SLOTS_PER_LANE,
        ARCHIVE_TUBE_COUNT,
        HANDOFF_PORTS,
        ROBOT_PICK_CLEARANCE_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout_constraints() {
    for rect in [
        receiving_rect(),
        analyzer_rect(),
        custody_rect(),
        archive_rect(),
        status_rect(),
        evidence_rect(),
        segregation_rect(),
        handoff_rect(),
    ] {
        assert!(
            rect.fits_inside(BASE_X, BASE_Y),
            "{} exceeds the QC panel deck footprint",
            rect.name
        );
    }

    assert!(!receiving_rect().overlaps(analyzer_rect()));
    assert!(!custody_rect().overlaps(archive_rect()));
    assert!(!archive_rect().overlaps(status_rect()));
    assert!(!evidence_rect().overlaps(analyzer_rect()));
    assert!(HANDOFF_PORTS == RECEIVING_POSITIONS);
    assert!(ROBOT_PICK_CLEARANCE_Z > RECEIVING_Z + 90.0);
}

fn base_tray() -> Part {
    let deck = centered_cube("cell_lot_release_qc_base_deck", BASE_X, BASE_Y, DECK_Z);
    let front_service_relief = centered_cube(
        "cell_lot_release_qc_front_robot_service_relief",
        BASE_X - 108.0,
        72.0,
        8.0,
    )
    .translate(0.0, -(BASE_Y / 2.0 - 58.0), DECK_Z / 2.0 - 3.5);
    let handoff_socket = centered_cube(
        "cell_lot_release_qc_rear_handoff_panel_socket",
        HANDOFF_X + 34.0,
        HANDOFF_Y + 16.0,
        8.0,
    )
    .translate(0.0, HANDOFF_CENTER_Y, DECK_Z / 2.0 - 3.0);
    let sample_sump = centered_cube(
        "cell_lot_release_qc_sample_custody_spill_sump",
        RECEIVING_X + CUSTODY_X - 32.0,
        46.0,
        8.0,
    )
    .translate(RECEIVING_CENTER_X, -40.0, DECK_Z / 2.0 - 3.0);
    let status_sump = centered_cube(
        "cell_lot_release_qc_status_lane_spill_sump",
        STATUS_X + 24.0,
        52.0,
        8.0,
    )
    .translate(STATUS_CENTER_X, STATUS_CENTER_Y - 126.0, DECK_Z / 2.0 - 3.0);
    let drain = centered_cylinder("cell_lot_release_qc_front_sump_drain", 7.0 / 2.0, 44.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(BASE_X / 2.0 - 70.0, -BASE_Y / 2.0 + 24.0, 0.0);

    deck - front_service_relief
        - handoff_socket
        - sample_sump
        - status_sump
        - drain
        - base_mount_slots()
        + base_perimeter_rim()
        + base_module_locator_bosses()
        + deck_zone_lands()
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("cell_lot_release_qc_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("cell_lot_release_qc_base_m6_clearance_{i}"),
            6.6 / 2.0,
            DECK_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("cell_lot_release_qc_base_m6_slot_relief_{i}"),
            28.0,
            7.0,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_perimeter_rim() -> Part {
    let rear = centered_cube(
        "cell_lot_release_qc_rear_handoff_guard_rail",
        BASE_X - 54.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, BASE_Y / 2.0 - 20.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let left = centered_cube(
        "cell_lot_release_qc_left_custody_guard_rail",
        RIM_W,
        BASE_Y - 94.0,
        RIM_Z,
    )
    .translate(-(BASE_X / 2.0 - 26.0), 20.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "cell_lot_release_qc_right_status_guard_rail",
        RIM_W,
        BASE_Y - 94.0,
        RIM_Z,
    )
    .translate(BASE_X / 2.0 - 26.0, 20.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let front_left = centered_cube(
        "cell_lot_release_qc_front_clean_seg_lip_left",
        470.0,
        12.0,
        18.0,
    )
    .translate(-286.0, -BASE_Y / 2.0 + 16.0, DECK_Z / 2.0 + 9.0);
    let front_right = centered_cube(
        "cell_lot_release_qc_front_used_seg_lip_right",
        470.0,
        12.0,
        18.0,
    )
    .translate(286.0, -BASE_Y / 2.0 + 16.0, DECK_Z / 2.0 + 9.0);

    rear + left + right + front_left + front_right
}

fn base_module_locator_bosses() -> Part {
    let mut bosses = Part::empty("cell_lot_release_qc_module_locator_bosses");
    for (i, rect) in [
        receiving_rect(),
        analyzer_rect(),
        custody_rect(),
        archive_rect(),
        status_rect(),
    ]
    .iter()
    .enumerate()
    {
        for (j, (x, y)) in rect.corner_mounts(32.0).iter().enumerate() {
            let boss = centered_cylinder(
                format!("cell_lot_release_qc_locator_boss_{i}_{j}"),
                8.0,
                8.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 4.0);
            let pin_socket = centered_cylinder(
                format!("cell_lot_release_qc_locator_pin_socket_{i}_{j}"),
                3.2 / 2.0,
                10.0,
                18,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 4.0);
            bosses = bosses + (boss - pin_socket);
        }
    }
    bosses
}

fn deck_zone_lands() -> Part {
    let receiving_land = centered_cube(
        "cell_lot_release_qc_deck_receiving_datum_land",
        RECEIVING_X - 52.0,
        6.0,
        5.0,
    )
    .translate(
        RECEIVING_CENTER_X,
        RECEIVING_CENTER_Y - RECEIVING_Y / 2.0 - 14.0,
        DECK_Z / 2.0 + 2.5,
    );
    let analyzer_land = centered_cube(
        "cell_lot_release_qc_deck_analyzer_datum_land",
        ANALYZER_DOCK_X - 70.0,
        6.0,
        5.0,
    )
    .translate(
        ANALYZER_CENTER_X,
        ANALYZER_CENTER_Y - ANALYZER_DOCK_Y / 2.0 - 14.0,
        DECK_Z / 2.0 + 2.5,
    );
    let status_land = centered_cube(
        "cell_lot_release_qc_deck_status_lane_datum_land",
        STATUS_X - 40.0,
        6.0,
        5.0,
    )
    .translate(
        STATUS_CENTER_X,
        STATUS_CENTER_Y + STATUS_Y / 2.0 + 14.0,
        DECK_Z / 2.0 + 2.5,
    );
    let archive_land = centered_cube(
        "cell_lot_release_qc_deck_archive_datum_land",
        ARCHIVE_X - 52.0,
        6.0,
        5.0,
    )
    .translate(
        ARCHIVE_CENTER_X,
        ARCHIVE_CENTER_Y + ARCHIVE_Y / 2.0 + 14.0,
        DECK_Z / 2.0 + 2.5,
    );

    receiving_land + analyzer_land + status_land + archive_land + deck_flow_arrows()
}

fn deck_flow_arrows() -> Part {
    let upstream_to_receive = centered_cube(
        "cell_lot_release_qc_deck_handoff_to_receive_flow_land",
        206.0,
        5.0,
        5.0,
    )
    .rotate(0.0, 0.0, -28.0)
    .translate(-250.0, 260.0, DECK_Z / 2.0 + 2.5);
    let receive_to_analyzer = centered_cube(
        "cell_lot_release_qc_deck_receive_to_analyzer_flow_land",
        318.0,
        5.0,
        5.0,
    )
    .translate(-68.0, 118.0, DECK_Z / 2.0 + 2.5);
    let analyzer_to_status = centered_cube(
        "cell_lot_release_qc_deck_analyzer_to_status_flow_land",
        250.0,
        5.0,
        5.0,
    )
    .rotate(0.0, 0.0, -72.0)
    .translate(314.0, -38.0, DECK_Z / 2.0 + 2.5);

    upstream_to_receive + receive_to_analyzer + analyzer_to_status
}

fn sealed_sample_receiving_nest() -> Part {
    let tray = centered_cube(
        "cell_lot_release_qc_receiving_outer_tray",
        RECEIVING_X,
        RECEIVING_Y,
        RECEIVING_Z,
    )
    .translate(0.0, 0.0, RECEIVING_Z / 2.0);
    let basin = centered_cube(
        "cell_lot_release_qc_receiving_spill_basin",
        RECEIVING_X - 34.0,
        RECEIVING_Y - 32.0,
        12.0,
    )
    .translate(0.0, 0.0, RECEIVING_Z - 5.0);
    let gripper_access = centered_cube(
        "cell_lot_release_qc_receiving_front_gripper_access",
        RECEIVING_X - 74.0,
        34.0,
        RECEIVING_Z + 4.0,
    )
    .translate(0.0, -(RECEIVING_Y / 2.0 - 16.0), RECEIVING_Z / 2.0);

    let mut pockets = Part::empty("cell_lot_release_qc_receiving_sample_pockets");
    for i in 0..RECEIVING_POSITIONS {
        let (x, y) = receiving_position(i);
        let tube_pocket = centered_cylinder(
            format!("cell_lot_release_qc_receiving_tube_socket_{i}"),
            SEALED_SAMPLE_POCKET_D / 2.0,
            RECEIVING_Z + 4.0,
            36,
        )
        .translate(x, y, RECEIVING_Z / 2.0 + 6.0);
        let cap_recess = centered_cylinder(
            format!("cell_lot_release_qc_receiving_cap_recess_{i}"),
            SAMPLE_CAP_CLEARANCE_D / 2.0,
            10.0,
            36,
        )
        .translate(x, y, RECEIVING_Z - 4.0);
        pockets = pockets + tube_pocket + cap_recess;
    }

    tray - basin - gripper_access - pockets
        + receiving_position_cradles()
        + receiving_chain_of_custody_flags()
        + sealed_sample_tube_comb()
}

fn receiving_position_cradles() -> Part {
    let mut cradles = Part::empty("cell_lot_release_qc_receiving_position_cradles");
    for i in 0..RECEIVING_POSITIONS {
        let (x, y) = receiving_position(i);
        let left_v = centered_cube(
            format!("cell_lot_release_qc_receiving_v_cradle_left_{i}"),
            7.0,
            34.0,
            18.0,
        )
        .rotate(0.0, 0.0, 14.0)
        .translate(x - 22.0, y, RECEIVING_Z + 9.0);
        let right_v = centered_cube(
            format!("cell_lot_release_qc_receiving_v_cradle_right_{i}"),
            7.0,
            34.0,
            18.0,
        )
        .rotate(0.0, 0.0, -14.0)
        .translate(x + 22.0, y, RECEIVING_Z + 9.0);
        let seal_stop = centered_cube(
            format!("cell_lot_release_qc_receiving_sealed_cap_stop_{i}"),
            46.0,
            5.0,
            16.0,
        )
        .translate(x, y + 24.0, RECEIVING_Z + 8.0);
        cradles = cradles + left_v + right_v + seal_stop;
    }
    cradles
}

fn receiving_chain_of_custody_flags() -> Part {
    let mut flags = Part::empty("cell_lot_release_qc_receiving_custody_flags");
    for i in 0..RECEIVING_POSITIONS {
        let (x, y) = receiving_position(i);
        let land = centered_cube(
            format!("cell_lot_release_qc_receiving_sample_barcode_land_{i}"),
            48.0,
            14.0,
            5.0,
        )
        .translate(x, y - 35.0, RECEIVING_Z + 2.5);
        let rfid_disc = centered_cylinder(
            format!("cell_lot_release_qc_receiving_rfid_disc_land_{i}"),
            6.0,
            4.0,
            24,
        )
        .translate(x - 29.0, y - 35.0, RECEIVING_Z + 2.0);
        flags = flags + land + rfid_disc;
    }
    flags
}

fn sealed_sample_tube_comb() -> Part {
    let comb = centered_cube(
        "cell_lot_release_qc_receiving_closed_tube_comb",
        RECEIVING_X - 70.0,
        14.0,
        32.0,
    )
    .translate(0.0, RECEIVING_Y / 2.0 - 28.0, RECEIVING_Z + 16.0);

    let mut tube_gates = Part::empty("cell_lot_release_qc_receiving_tube_gate_cutouts");
    for i in 0..RECEIVING_POSITIONS {
        let (x, _) = receiving_position(i);
        tube_gates = tube_gates
            + centered_cylinder(
                format!("cell_lot_release_qc_receiving_tube_gate_{i}"),
                4.2,
                22.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, RECEIVING_Y / 2.0 - 28.0, RECEIVING_Z + 16.0);
    }

    comb - tube_gates
}

fn count_viability_analyzer_dock() -> Part {
    let dock = centered_cube(
        "cell_lot_release_qc_analyzer_dock_base",
        ANALYZER_DOCK_X,
        ANALYZER_DOCK_Y,
        ANALYZER_DOCK_Z,
    )
    .translate(0.0, 0.0, ANALYZER_DOCK_Z / 2.0);
    let foot_recess = centered_cube(
        "cell_lot_release_qc_analyzer_footprint_recess",
        ANALYZER_ENV_X + 18.0,
        ANALYZER_ENV_Y + 18.0,
        10.0,
    )
    .translate(0.0, 0.0, ANALYZER_DOCK_Z - 4.0);
    let sample_slot = centered_cube(
        "cell_lot_release_qc_analyzer_sample_cassette_slot",
        190.0,
        34.0,
        ANALYZER_DOCK_Z + 4.0,
    )
    .translate(
        -116.0,
        -(ANALYZER_DOCK_Y / 2.0 - 36.0),
        ANALYZER_DOCK_Z / 2.0,
    );
    let cable_trough = centered_cube(
        "cell_lot_release_qc_analyzer_data_power_trough",
        72.0,
        40.0,
        ANALYZER_DOCK_Z + 4.0,
    )
    .translate(
        ANALYZER_DOCK_X / 2.0 - 60.0,
        ANALYZER_DOCK_Y / 2.0 - 42.0,
        ANALYZER_DOCK_Z / 2.0,
    );

    dock - foot_recess - sample_slot - cable_trough
        + analyzer_envelope_placeholder()
        + analyzer_alignment_rails()
        + viability_cassette_magazine()
        + analyzer_data_handoff_pedestal()
}

fn analyzer_envelope_placeholder() -> Part {
    let envelope = centered_cube(
        "cell_lot_release_qc_analyzer_clearance_envelope_430x282x262",
        ANALYZER_ENV_X,
        ANALYZER_ENV_Y,
        ANALYZER_ENV_Z,
    )
    .translate(0.0, 0.0, ANALYZER_DOCK_Z + ANALYZER_ENV_Z / 2.0);
    let front_slot_witness = centered_cube(
        "cell_lot_release_qc_analyzer_front_sample_door_witness",
        150.0,
        10.0,
        58.0,
    )
    .translate(0.0, -(ANALYZER_ENV_Y / 2.0 + 1.0), ANALYZER_DOCK_Z + 84.0);
    let service_screen_land = centered_cube(
        "cell_lot_release_qc_analyzer_screen_data_land",
        158.0,
        8.0,
        72.0,
    )
    .translate(
        ANALYZER_ENV_X / 2.0 - 72.0,
        -(ANALYZER_ENV_Y / 2.0 + 1.0),
        ANALYZER_DOCK_Z + 168.0,
    );

    envelope - front_slot_witness - service_screen_land
}

fn analyzer_alignment_rails() -> Part {
    let left = centered_cube(
        "cell_lot_release_qc_analyzer_left_alignment_rail",
        16.0,
        ANALYZER_ENV_Y + 28.0,
        30.0,
    )
    .translate(-(ANALYZER_ENV_X / 2.0 + 22.0), 0.0, ANALYZER_DOCK_Z + 15.0);
    let right = centered_cube(
        "cell_lot_release_qc_analyzer_right_alignment_rail",
        16.0,
        ANALYZER_ENV_Y + 28.0,
        30.0,
    )
    .translate(ANALYZER_ENV_X / 2.0 + 22.0, 0.0, ANALYZER_DOCK_Z + 15.0);
    let back_stop = centered_cube(
        "cell_lot_release_qc_analyzer_rear_hard_stop",
        ANALYZER_ENV_X + 64.0,
        16.0,
        38.0,
    )
    .translate(0.0, ANALYZER_ENV_Y / 2.0 + 24.0, ANALYZER_DOCK_Z + 19.0);
    let front_low_lip = centered_cube(
        "cell_lot_release_qc_analyzer_front_low_loading_lip",
        ANALYZER_ENV_X - 58.0,
        10.0,
        18.0,
    )
    .translate(34.0, -(ANALYZER_ENV_Y / 2.0 + 20.0), ANALYZER_DOCK_Z + 9.0);

    left + right + back_stop + front_low_lip
}

fn viability_cassette_magazine() -> Part {
    let magazine = centered_cube(
        "cell_lot_release_qc_viability_cassette_magazine_block",
        92.0,
        36.0,
        82.0,
    )
    .translate(
        -(ANALYZER_DOCK_X / 2.0 - 78.0),
        -(ANALYZER_DOCK_Y / 2.0 - 58.0),
        ANALYZER_DOCK_Z + 41.0,
    );

    let mut cassette_slots = Part::empty("cell_lot_release_qc_viability_cassette_slots");
    for i in 0..VIABILITY_CASSETTES {
        cassette_slots = cassette_slots
            + centered_cube(
                format!("cell_lot_release_qc_viability_cassette_slot_{i}"),
                64.0,
                4.2,
                30.0,
            )
            .translate(
                -(ANALYZER_DOCK_X / 2.0 - 78.0),
                -(ANALYZER_DOCK_Y / 2.0 - 72.0) + i as f64 * 2.2,
                ANALYZER_DOCK_Z + 31.0 + i as f64 * (CASSETTE_SLOT_PITCH / 12.0),
            );
    }

    magazine - cassette_slots
}

fn analyzer_data_handoff_pedestal() -> Part {
    let pedestal = centered_cube(
        "cell_lot_release_qc_analyzer_api_data_pedestal",
        112.0,
        42.0,
        42.0,
    )
    .translate(
        ANALYZER_DOCK_X / 2.0 - 82.0,
        -(ANALYZER_DOCK_Y / 2.0 - 54.0),
        ANALYZER_DOCK_Z + 21.0,
    );
    let barcode_window = centered_cube(
        "cell_lot_release_qc_analyzer_api_barcode_window",
        68.0,
        6.0,
        18.0,
    )
    .translate(
        ANALYZER_DOCK_X / 2.0 - 82.0,
        -(ANALYZER_DOCK_Y / 2.0 - 32.0),
        ANALYZER_DOCK_Z + 27.0,
    );
    let cable_gland = centered_cylinder(
        "cell_lot_release_qc_analyzer_data_cable_gland",
        9.0,
        30.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        ANALYZER_DOCK_X / 2.0 - 48.0,
        -(ANALYZER_DOCK_Y / 2.0 - 32.0),
        ANALYZER_DOCK_Z + 18.0,
    );

    pedestal - barcode_window - cable_gland
}

fn sterility_mycoplasma_custody_slots() -> Part {
    let tray = centered_cube(
        "cell_lot_release_qc_custody_outer_tray",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);
    let sealed_lid_recess = centered_cube(
        "cell_lot_release_qc_custody_sealed_lid_recess",
        CUSTODY_X - 34.0,
        CUSTODY_Y - 30.0,
        8.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z - 3.0);
    let rear_chain_slot = centered_cube(
        "cell_lot_release_qc_custody_rear_chain_of_custody_slot",
        CUSTODY_X - 66.0,
        18.0,
        CUSTODY_Z + 2.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 - 30.0, CUSTODY_Z / 2.0);

    tray - sealed_lid_recess - rear_chain_slot - custody_pocket_cuts()
        + custody_section_dividers()
        + custody_retention_clips()
        + custody_label_lands()
}

fn custody_pocket_cuts() -> Part {
    let mut pockets = Part::empty("cell_lot_release_qc_custody_pocket_cuts");
    for i in 0..STERILITY_SLOTS {
        let x = -112.0 + i as f64 * 44.8;
        pockets = pockets
            + centered_cylinder(
                format!("cell_lot_release_qc_sterility_sample_socket_{i}"),
                10.0,
                CUSTODY_Z + 4.0,
                28,
            )
            .translate(x, 48.0, CUSTODY_Z / 2.0 + 3.0);
    }
    for i in 0..MYCOPLASMA_SLOTS {
        let x = -132.0 + i as f64 * 37.7;
        pockets = pockets
            + centered_cylinder(
                format!("cell_lot_release_qc_mycoplasma_sample_socket_{i}"),
                7.2,
                CUSTODY_Z + 4.0,
                24,
            )
            .translate(x, -10.0, CUSTODY_Z / 2.0 + 3.0);
    }
    for i in 0..RETAIN_SLOTS {
        let x = -72.0 + i as f64 * 48.0;
        pockets = pockets
            + centered_cylinder(
                format!("cell_lot_release_qc_retain_sample_socket_{i}"),
                12.8,
                CUSTODY_Z + 4.0,
                30,
            )
            .translate(x, -68.0, CUSTODY_Z / 2.0 + 3.0);
    }
    pockets
}

fn custody_section_dividers() -> Part {
    let top = centered_cube(
        "cell_lot_release_qc_custody_sterility_section_rail",
        CUSTODY_X - 56.0,
        8.0,
        20.0,
    )
    .translate(0.0, 20.0, CUSTODY_Z + 10.0);
    let bottom = centered_cube(
        "cell_lot_release_qc_custody_retain_section_rail",
        CUSTODY_X - 56.0,
        8.0,
        20.0,
    )
    .translate(0.0, -42.0, CUSTODY_Z + 10.0);
    let tamper = centered_cube(
        "cell_lot_release_qc_custody_tamper_evidence_bridge",
        CUSTODY_X - 74.0,
        14.0,
        28.0,
    )
    .translate(0.0, CUSTODY_Y / 2.0 - 30.0, CUSTODY_Z + 14.0);
    top + bottom + tamper
}

fn custody_retention_clips() -> Part {
    let mut clips = Part::empty("cell_lot_release_qc_custody_retention_clips");
    for (i, x) in [-128.0, -64.0, 0.0, 64.0, 128.0].iter().enumerate() {
        let clip = centered_cube(
            format!("cell_lot_release_qc_custody_retention_clip_{i}"),
            24.0,
            7.0,
            20.0,
        )
        .translate(*x, CUSTODY_Y / 2.0 - 17.0, CUSTODY_Z + 10.0);
        clips = clips + clip;
    }
    clips
}

fn custody_label_lands() -> Part {
    let sterility = centered_cube(
        "cell_lot_release_qc_custody_sterility_label_land",
        108.0,
        18.0,
        5.0,
    )
    .translate(-100.0, 80.0, CUSTODY_Z + 2.5);
    let myco = centered_cube(
        "cell_lot_release_qc_custody_mycoplasma_label_land",
        112.0,
        18.0,
        5.0,
    )
    .translate(92.0, 80.0, CUSTODY_Z + 2.5);
    let retain = centered_cube(
        "cell_lot_release_qc_custody_retain_label_land",
        96.0,
        18.0,
        5.0,
    )
    .translate(0.0, -92.0, CUSTODY_Z + 2.5);
    sterility + myco + retain
}

fn identity_passage_lot_barcode_lands() -> Part {
    let mut lands = Part::empty("cell_lot_release_qc_identity_passage_lot_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let (x, y) = barcode_land_position(i);
        let land = centered_cube(
            format!("cell_lot_release_qc_barcode_land_{i}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            BARCODE_LAND_Z,
        )
        .translate(x, y, BARCODE_LAND_Z / 2.0);
        let scan_fiducial = centered_cylinder(
            format!("cell_lot_release_qc_barcode_fiducial_disc_{i}"),
            4.0,
            4.0,
            18,
        )
        .translate(x - BARCODE_LAND_X / 2.0 + 11.0, y, BARCODE_LAND_Z + 2.0);
        let qr_corner = centered_cube(
            format!("cell_lot_release_qc_barcode_qr_corner_relief_{i}"),
            10.0,
            10.0,
            3.0,
        )
        .translate(
            x + BARCODE_LAND_X / 2.0 - 12.0,
            y + BARCODE_LAND_Y / 2.0 - 7.0,
            BARCODE_LAND_Z + 1.5,
        );
        lands = lands + land + scan_fiducial + qr_corner;
    }
    lands
}

fn released_hold_reject_status_lanes() -> Part {
    let tray = centered_cube(
        "cell_lot_release_qc_status_lane_outer_tray",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let lane_basin = centered_cube(
        "cell_lot_release_qc_status_lane_basin",
        STATUS_X - 34.0,
        STATUS_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, STATUS_Z - 4.0);
    let front_access = centered_cube(
        "cell_lot_release_qc_status_lane_front_robot_access",
        STATUS_X - 54.0,
        24.0,
        STATUS_Z + 4.0,
    )
    .translate(0.0, -(STATUS_Y / 2.0 - 14.0), STATUS_Z / 2.0);

    tray - lane_basin - front_access - status_slot_cuts()
        + status_lane_dividers()
        + status_lane_label_lands()
        + status_lane_hard_stops()
}

fn status_slot_cuts() -> Part {
    let mut slots = Part::empty("cell_lot_release_qc_status_slot_cuts");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = -70.0 + slot as f64 * 35.0;
            slots = slots
                + centered_cube(
                    format!("cell_lot_release_qc_status_lane_{lane}_carrier_slot_{slot}"),
                    48.0,
                    22.0,
                    STATUS_Z + 4.0,
                )
                .translate(x, y, STATUS_Z / 2.0 + 3.0);
        }
    }
    slots
}

fn status_lane_dividers() -> Part {
    let div_a = centered_cube(
        "cell_lot_release_qc_status_release_hold_divider",
        8.0,
        STATUS_Y - 42.0,
        28.0,
    )
    .translate(
        (status_lane_x(0) + status_lane_x(1)) / 2.0,
        0.0,
        STATUS_Z + 14.0,
    );
    let div_b = centered_cube(
        "cell_lot_release_qc_status_hold_reject_divider",
        8.0,
        STATUS_Y - 42.0,
        36.0,
    )
    .translate(
        (status_lane_x(1) + status_lane_x(2)) / 2.0,
        0.0,
        STATUS_Z + 18.0,
    );
    let reject_tall_wall = centered_cube(
        "cell_lot_release_qc_status_reject_tall_outer_wall",
        10.0,
        STATUS_Y - 34.0,
        52.0,
    )
    .translate(status_lane_x(2) + 38.0, 0.0, STATUS_Z + 26.0);
    div_a + div_b + reject_tall_wall
}

fn status_lane_label_lands() -> Part {
    let release = centered_cube(
        "cell_lot_release_qc_status_released_label_land",
        62.0,
        18.0,
        6.0,
    )
    .translate(status_lane_x(0), STATUS_Y / 2.0 - 25.0, STATUS_Z + 3.0);
    let hold = centered_cube(
        "cell_lot_release_qc_status_hold_label_land",
        62.0,
        18.0,
        6.0,
    )
    .translate(status_lane_x(1), STATUS_Y / 2.0 - 25.0, STATUS_Z + 3.0);
    let reject = centered_cube(
        "cell_lot_release_qc_status_reject_label_land",
        62.0,
        18.0,
        6.0,
    )
    .translate(status_lane_x(2), STATUS_Y / 2.0 - 25.0, STATUS_Z + 3.0);
    release + hold + reject
}

fn status_lane_hard_stops() -> Part {
    let mut stops = Part::empty("cell_lot_release_qc_status_lane_hard_stops");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        let back_stop = centered_cube(
            format!("cell_lot_release_qc_status_lane_{lane}_back_stop"),
            54.0,
            8.0,
            30.0,
        )
        .translate(x, 82.0, STATUS_Z + 15.0);
        let front_low_stop = centered_cube(
            format!("cell_lot_release_qc_status_lane_{lane}_front_low_stop"),
            54.0,
            6.0,
            18.0,
        )
        .translate(x, -92.0, STATUS_Z + 9.0);
        stops = stops + back_stop + front_low_stop;
    }
    stops
}

fn environmental_excursion_evidence_slot() -> Part {
    let holder = centered_cube(
        "cell_lot_release_qc_environmental_evidence_holder",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_Z / 2.0);
    let logger_pocket = centered_cube(
        "cell_lot_release_qc_excursion_logger_pocket",
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        EVIDENCE_Z + 4.0,
    )
    .translate(-(EVIDENCE_X / 2.0 - 82.0), 10.0, EVIDENCE_Z / 2.0 + 2.0);
    let coa_card_pocket = centered_cube(
        "cell_lot_release_qc_excursion_coa_card_pocket",
        COA_CARD_X,
        COA_CARD_Y,
        10.0,
    )
    .translate(EVIDENCE_X / 2.0 - 76.0, 0.0, EVIDENCE_Z - 4.0);
    let thumb_cut = centered_cylinder(
        "cell_lot_release_qc_excursion_card_thumb_cut",
        16.0,
        32.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(EVIDENCE_X / 2.0 - 17.0, 0.0, EVIDENCE_Z - 2.0);

    holder - logger_pocket - coa_card_pocket - thumb_cut
        + evidence_tamper_clips()
        + evidence_status_led_lands()
}

fn evidence_tamper_clips() -> Part {
    let left = centered_cube(
        "cell_lot_release_qc_excursion_left_tamper_clip",
        14.0,
        EVIDENCE_Y - 30.0,
        24.0,
    )
    .translate(-(EVIDENCE_X / 2.0 - 18.0), 0.0, EVIDENCE_Z + 12.0);
    let right = centered_cube(
        "cell_lot_release_qc_excursion_right_tamper_clip",
        14.0,
        EVIDENCE_Y - 30.0,
        24.0,
    )
    .translate(EVIDENCE_X / 2.0 - 18.0, 0.0, EVIDENCE_Z + 12.0);
    let rear = centered_cube(
        "cell_lot_release_qc_excursion_rear_tamper_clip",
        EVIDENCE_X - 54.0,
        10.0,
        22.0,
    )
    .translate(0.0, EVIDENCE_Y / 2.0 - 18.0, EVIDENCE_Z + 11.0);
    left + right + rear
}

fn evidence_status_led_lands() -> Part {
    let mut leds = Part::empty("cell_lot_release_qc_excursion_status_led_lands");
    for (i, x) in [-22.0, 0.0, 22.0].iter().enumerate() {
        leds = leds
            + centered_cylinder(
                format!("cell_lot_release_qc_excursion_status_led_land_{i}"),
                5.0,
                4.0,
                20,
            )
            .translate(*x, -(EVIDENCE_Y / 2.0 - 18.0), EVIDENCE_Z + 2.0);
    }
    leds
}

fn archive_aliquot_staging() -> Part {
    let cold_block = centered_cube(
        "cell_lot_release_qc_archive_cold_block",
        ARCHIVE_X,
        ARCHIVE_Y,
        ARCHIVE_Z,
    )
    .translate(0.0, 0.0, ARCHIVE_Z / 2.0);
    let drip_basin = centered_cube(
        "cell_lot_release_qc_archive_drip_basin",
        ARCHIVE_X - 36.0,
        ARCHIVE_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, ARCHIVE_Z - 3.0);
    let pull_slot = centered_cube(
        "cell_lot_release_qc_archive_front_pull_slot",
        ARCHIVE_X - 82.0,
        20.0,
        ARCHIVE_Z + 2.0,
    )
    .translate(0.0, -(ARCHIVE_Y / 2.0 - 12.0), ARCHIVE_Z / 2.0);

    cold_block - drip_basin - pull_slot - archive_tube_pockets()
        + archive_tube_position_lands()
        + archive_barcode_spine()
        + archive_cold_interface_ribs()
}

fn archive_tube_pockets() -> Part {
    let mut pockets = Part::empty("cell_lot_release_qc_archive_tube_pockets");
    for row in 0..ARCHIVE_ROWS {
        for col in 0..ARCHIVE_COLS {
            let idx = row * ARCHIVE_COLS + col;
            let (x, y) = archive_tube_position(col, row);
            pockets = pockets
                + centered_cylinder(
                    format!("cell_lot_release_qc_archive_aliquot_tube_socket_{idx}"),
                    ARCHIVE_TUBE_D / 2.0,
                    ARCHIVE_Z + 4.0,
                    24,
                )
                .translate(x, y, ARCHIVE_Z / 2.0 + 3.0);
        }
    }
    pockets
}

fn archive_tube_position_lands() -> Part {
    let mut lands = Part::empty("cell_lot_release_qc_archive_tube_position_lands");
    for row in 0..ARCHIVE_ROWS {
        for col in 0..ARCHIVE_COLS {
            let idx = row * ARCHIVE_COLS + col;
            let (x, y) = archive_tube_position(col, row);
            lands = lands
                + centered_cylinder(
                    format!("cell_lot_release_qc_archive_position_land_{idx}"),
                    ARCHIVE_TUBE_D / 2.0 + 5.0,
                    4.0,
                    24,
                )
                .translate(x, y, ARCHIVE_Z + 2.0);
        }
    }
    lands
}

fn archive_barcode_spine() -> Part {
    let spine = centered_cube(
        "cell_lot_release_qc_archive_barcode_spine",
        34.0,
        ARCHIVE_Y - 46.0,
        18.0,
    )
    .translate(ARCHIVE_X / 2.0 - 34.0, 0.0, ARCHIVE_Z + 9.0);
    let mut lands = Part::empty("cell_lot_release_qc_archive_barcode_spine_lands");
    for i in 0..ARCHIVE_ROWS {
        lands = lands
            + centered_cube(
                format!("cell_lot_release_qc_archive_row_barcode_land_{i}"),
                22.0,
                28.0,
                4.0,
            )
            .translate(
                ARCHIVE_X / 2.0 - 34.0,
                -((ARCHIVE_ROWS as f64 - 1.0) * ARCHIVE_PITCH_Y) / 2.0 + i as f64 * ARCHIVE_PITCH_Y,
                ARCHIVE_Z + 20.0,
            );
    }
    spine + lands
}

fn archive_cold_interface_ribs() -> Part {
    let mut ribs = Part::empty("cell_lot_release_qc_archive_cold_interface_ribs");
    for (i, y) in [-86.0, -43.0, 43.0, 86.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("cell_lot_release_qc_archive_cold_plate_rib_{i}"),
                ARCHIVE_X - 54.0,
                6.0,
                12.0,
            )
            .translate(0.0, *y, 6.0);
    }
    ribs
}

fn clean_used_segregation() -> Part {
    let tray = centered_cube(
        "cell_lot_release_qc_clean_used_segregation_tray",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0);
    let clean_basin = centered_cube(
        "cell_lot_release_qc_clean_consumable_basin",
        SEGREGATION_X / 2.0 - 60.0,
        SEGREGATION_Y - 16.0,
        14.0,
    )
    .translate(-(SEGREGATION_X / 4.0), 0.0, SEGREGATION_Z - 6.0);
    let used_basin = centered_cube(
        "cell_lot_release_qc_used_return_basin",
        SEGREGATION_X / 2.0 - 60.0,
        SEGREGATION_Y - 16.0,
        20.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z - 7.0);
    let waste_chute =
        centered_cylinder("cell_lot_release_qc_used_basin_decon_drain", 10.0, 44.0, 28)
            .rotate(90.0, 0.0, 0.0)
            .translate(
                SEGREGATION_X / 2.0 - 64.0,
                -SEGREGATION_Y / 2.0 + 7.0,
                SEGREGATION_Z / 2.0,
            );

    tray - clean_basin - used_basin - waste_chute
        + clean_used_center_wall()
        + clean_consumable_clip_slots()
        + used_item_tall_lip()
}

fn clean_used_center_wall() -> Part {
    centered_cube(
        "cell_lot_release_qc_clean_used_center_segregation_wall",
        12.0,
        SEGREGATION_Y + 8.0,
        SEGREGATION_Z + 16.0,
    )
    .translate(0.0, 0.0, (SEGREGATION_Z + 16.0) / 2.0)
}

fn clean_consumable_clip_slots() -> Part {
    let mut clips = Part::empty("cell_lot_release_qc_clean_consumable_clip_slots");
    for (i, x) in [-470.0, -390.0, -310.0, -230.0, -150.0].iter().enumerate() {
        let clip = centered_cube(
            format!("cell_lot_release_qc_clean_consumable_clip_land_{i}"),
            48.0,
            8.0,
            18.0,
        )
        .translate(*x, 0.0, SEGREGATION_Z + 9.0);
        clips = clips + clip;
    }
    clips
}

fn used_item_tall_lip() -> Part {
    let rear = centered_cube(
        "cell_lot_release_qc_used_return_rear_tall_lip",
        SEGREGATION_X / 2.0 - 46.0,
        8.0,
        34.0,
    )
    .translate(
        SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 + 2.0,
        SEGREGATION_Z + 17.0,
    );
    let right = centered_cube(
        "cell_lot_release_qc_used_return_side_tall_lip",
        8.0,
        SEGREGATION_Y + 8.0,
        34.0,
    )
    .translate(SEGREGATION_X / 2.0 - 20.0, 0.0, SEGREGATION_Z + 17.0);
    rear + right
}

fn upstream_handoff_interface() -> Part {
    let panel = centered_cube(
        "cell_lot_release_qc_upstream_handoff_bulkhead_panel",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    )
    .translate(0.0, 0.0, HANDOFF_Z / 2.0);
    let lower_service_slot = centered_cube(
        "cell_lot_release_qc_upstream_handoff_lower_service_slot",
        HANDOFF_X - 88.0,
        HANDOFF_Y + 4.0,
        28.0,
    )
    .translate(0.0, 0.0, 26.0);
    let upper_cable_slot = centered_cube(
        "cell_lot_release_qc_upstream_handoff_upper_data_slot",
        210.0,
        HANDOFF_Y + 4.0,
        22.0,
    )
    .translate(HANDOFF_X / 2.0 - 150.0, 0.0, HANDOFF_Z - 34.0);

    panel - lower_service_slot - upper_cable_slot - upstream_handoff_port_cuts()
        + upstream_tube_comb()
        + handoff_kinematic_datums()
        + handoff_lot_latch_bar()
}

fn upstream_handoff_port_cuts() -> Part {
    let mut cuts = Part::empty("cell_lot_release_qc_upstream_handoff_port_cuts");
    for i in 0..HANDOFF_PORTS {
        let x = handoff_port_x(i);
        let port = centered_cylinder(
            format!("cell_lot_release_qc_upstream_handoff_closed_sample_port_{i}"),
            12.0,
            HANDOFF_Y + 6.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 82.0);
        let keyed_flat = centered_cube(
            format!("cell_lot_release_qc_upstream_handoff_key_flat_{i}"),
            12.0,
            HANDOFF_Y + 8.0,
            7.0,
        )
        .translate(x, 0.0, 100.0);
        cuts = cuts + port + keyed_flat;
    }
    cuts
}

fn upstream_tube_comb() -> Part {
    let comb = centered_cube(
        "cell_lot_release_qc_upstream_handoff_tube_comb",
        HANDOFF_X - 120.0,
        18.0,
        30.0,
    )
    .translate(0.0, -(HANDOFF_Y / 2.0 + 12.0), 44.0);
    let mut cuts = Part::empty("cell_lot_release_qc_upstream_handoff_tube_comb_cuts");
    for i in 0..HANDOFF_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("cell_lot_release_qc_upstream_handoff_tube_comb_slot_{i}"),
                4.2,
                28.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(handoff_port_x(i), -(HANDOFF_Y / 2.0 + 12.0), 44.0);
    }
    comb - cuts
}

fn handoff_kinematic_datums() -> Part {
    let mut datums = Part::empty("cell_lot_release_qc_upstream_handoff_kinematic_datums");
    for (i, x) in [-(HANDOFF_X / 2.0 - 48.0), 0.0, HANDOFF_X / 2.0 - 48.0]
        .iter()
        .enumerate()
    {
        let boss = centered_cylinder(
            format!("cell_lot_release_qc_handoff_datum_boss_{i}"),
            12.0,
            10.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -(HANDOFF_Y / 2.0 + 6.0), HANDOFF_Z - 22.0);
        let pin_socket = centered_cylinder(
            format!("cell_lot_release_qc_handoff_datum_pin_socket_{i}"),
            4.0 / 2.0,
            12.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -(HANDOFF_Y / 2.0 + 6.0), HANDOFF_Z - 22.0);
        datums = datums + (boss - pin_socket);
    }
    datums
}

fn handoff_lot_latch_bar() -> Part {
    let bar = centered_cube(
        "cell_lot_release_qc_handoff_lot_latch_bar",
        HANDOFF_X - 100.0,
        10.0,
        18.0,
    )
    .translate(0.0, HANDOFF_Y / 2.0 + 7.0, HANDOFF_Z - 16.0);
    let mut latches = Part::empty("cell_lot_release_qc_handoff_lot_latches");
    for (i, x) in [-330.0, -110.0, 110.0, 330.0].iter().enumerate() {
        latches = latches
            + centered_cube(
                format!("cell_lot_release_qc_handoff_quarter_turn_latch_land_{i}"),
                44.0,
                10.0,
                28.0,
            )
            .translate(*x, HANDOFF_Y / 2.0 + 7.0, HANDOFF_Z - 34.0);
    }
    bar + latches
}

fn robot_service_keepouts() -> Part {
    let front_robot_frame = rect_frame_xy(
        "cell_lot_release_qc_front_robot_pick_sweep_keepout",
        BASE_X - 90.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        10.0,
        8.0,
    )
    .translate(0.0, -164.0, ROBOT_PICK_CLEARANCE_Z);
    let analyzer_lift_frame = rect_frame_xy(
        "cell_lot_release_qc_analyzer_service_lift_keepout",
        ANALYZER_DOCK_X + 96.0,
        ANALYZER_DOCK_Y + 110.0,
        12.0,
        10.0,
    )
    .translate(ANALYZER_CENTER_X, ANALYZER_CENTER_Y, SERVICE_KEEP_OUT_Z);
    let rear_handoff_frame = rect_frame_xy(
        "cell_lot_release_qc_upstream_passaging_seeding_handoff_keepout",
        HANDOFF_X + 82.0,
        124.0,
        10.0,
        8.0,
    )
    .translate(HANDOFF_CENTER_X, HANDOFF_CENTER_Y - 58.0, 180.0);
    let vertical_service_gauge = centered_cube(
        "cell_lot_release_qc_robot_z_service_clearance_gauge",
        20.0,
        20.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        BASE_X / 2.0 - 70.0,
        BASE_Y / 2.0 - 82.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );

    front_robot_frame + analyzer_lift_frame + rear_handoff_frame + vertical_service_gauge
}

fn rect_frame_xy(name: &str, outer_x: f64, outer_y: f64, rail: f64, z: f64) -> Part {
    let left = centered_cube(format!("{name}_left"), rail, outer_y, z).translate(
        -(outer_x / 2.0 - rail / 2.0),
        0.0,
        z / 2.0,
    );
    let right = centered_cube(format!("{name}_right"), rail, outer_y, z).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        z / 2.0,
    );
    let rear = centered_cube(format!("{name}_rear"), outer_x, rail, z).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        z / 2.0,
    );
    let front = centered_cube(format!("{name}_front"), outer_x, rail, z).translate(
        0.0,
        -(outer_y / 2.0 - rail / 2.0),
        z / 2.0,
    );
    left + right + rear + front
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 42.0), -(BASE_Y / 2.0 - 40.0)),
        (BASE_X / 2.0 - 42.0, -(BASE_Y / 2.0 - 40.0)),
        (-(BASE_X / 2.0 - 42.0), BASE_Y / 2.0 - 40.0),
        (BASE_X / 2.0 - 42.0, BASE_Y / 2.0 - 40.0),
        (0.0, -(BASE_Y / 2.0 - 40.0)),
        (0.0, BASE_Y / 2.0 - 40.0),
        (-(BASE_X / 2.0 - 42.0), 0.0),
        (BASE_X / 2.0 - 42.0, 0.0),
    ]
}

fn receiving_position(index: usize) -> (f64, f64) {
    let row = index / RECEIVING_COLS;
    let col = index % RECEIVING_COLS;
    let x =
        -((RECEIVING_COLS as f64 - 1.0) * RECEIVING_PITCH_X) / 2.0 + col as f64 * RECEIVING_PITCH_X;
    let y =
        ((RECEIVING_ROWS as f64 - 1.0) * RECEIVING_PITCH_Y) / 2.0 - row as f64 * RECEIVING_PITCH_Y;
    (x, y)
}

fn status_lane_x(lane: usize) -> f64 {
    -((STATUS_LANES as f64 - 1.0) * STATUS_LANE_PITCH) / 2.0 + lane as f64 * STATUS_LANE_PITCH
}

fn archive_tube_position(col: usize, row: usize) -> (f64, f64) {
    let x = -((ARCHIVE_COLS as f64 - 1.0) * ARCHIVE_PITCH_X) / 2.0 + col as f64 * ARCHIVE_PITCH_X
        - 18.0;
    let y = ((ARCHIVE_ROWS as f64 - 1.0) * ARCHIVE_PITCH_Y) / 2.0 - row as f64 * ARCHIVE_PITCH_Y;
    (x, y)
}

fn barcode_land_position(index: usize) -> (f64, f64) {
    let row = index / 6;
    let col = index % 6;
    let x = -((6.0 - 1.0) * 176.0) / 2.0 + col as f64 * 176.0;
    let y = if row == 0 {
        BASE_Y / 2.0 - 98.0
    } else {
        -BASE_Y / 2.0 + 96.0
    };
    (x, y)
}

fn handoff_port_x(index: usize) -> f64 {
    -((HANDOFF_PORTS as f64 - 1.0) * HANDOFF_PORT_PITCH) / 2.0 + index as f64 * HANDOFF_PORT_PITCH
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        self.x - self.w / 2.0 >= -deck_x / 2.0
            && self.x + self.w / 2.0 <= deck_x / 2.0
            && self.y - self.h / 2.0 >= -deck_y / 2.0
            && self.y + self.h / 2.0 <= deck_y / 2.0
    }

    fn overlaps(self, other: Rect) -> bool {
        let ax0 = self.x - self.w / 2.0;
        let ax1 = self.x + self.w / 2.0;
        let ay0 = self.y - self.h / 2.0;
        let ay1 = self.y + self.h / 2.0;
        let bx0 = other.x - other.w / 2.0;
        let bx1 = other.x + other.w / 2.0;
        let by0 = other.y - other.h / 2.0;
        let by1 = other.y + other.h / 2.0;

        ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
    }

    fn corner_mounts(self, inset: f64) -> [(f64, f64); 4] {
        [
            (self.x - self.w / 2.0 + inset, self.y - self.h / 2.0 + inset),
            (self.x + self.w / 2.0 - inset, self.y - self.h / 2.0 + inset),
            (self.x - self.w / 2.0 + inset, self.y + self.h / 2.0 - inset),
            (self.x + self.w / 2.0 - inset, self.y + self.h / 2.0 - inset),
        ]
    }
}

fn receiving_rect() -> Rect {
    Rect {
        name: "receiving",
        x: RECEIVING_CENTER_X,
        y: RECEIVING_CENTER_Y,
        w: RECEIVING_X,
        h: RECEIVING_Y,
    }
}

fn analyzer_rect() -> Rect {
    Rect {
        name: "analyzer",
        x: ANALYZER_CENTER_X,
        y: ANALYZER_CENTER_Y,
        w: ANALYZER_DOCK_X,
        h: ANALYZER_DOCK_Y,
    }
}

fn custody_rect() -> Rect {
    Rect {
        name: "custody",
        x: CUSTODY_CENTER_X,
        y: CUSTODY_CENTER_Y,
        w: CUSTODY_X,
        h: CUSTODY_Y,
    }
}

fn archive_rect() -> Rect {
    Rect {
        name: "archive",
        x: ARCHIVE_CENTER_X,
        y: ARCHIVE_CENTER_Y,
        w: ARCHIVE_X,
        h: ARCHIVE_Y,
    }
}

fn status_rect() -> Rect {
    Rect {
        name: "status",
        x: STATUS_CENTER_X,
        y: STATUS_CENTER_Y,
        w: STATUS_X,
        h: STATUS_Y,
    }
}

fn evidence_rect() -> Rect {
    Rect {
        name: "evidence",
        x: EVIDENCE_CENTER_X,
        y: EVIDENCE_CENTER_Y,
        w: EVIDENCE_X,
        h: EVIDENCE_Y,
    }
}

fn segregation_rect() -> Rect {
    Rect {
        name: "segregation",
        x: SEGREGATION_CENTER_X,
        y: SEGREGATION_CENTER_Y,
        w: SEGREGATION_X,
        h: SEGREGATION_Y,
    }
}

fn handoff_rect() -> Rect {
    Rect {
        name: "handoff",
        x: HANDOFF_CENTER_X,
        y: HANDOFF_CENTER_Y,
        w: HANDOFF_X,
        h: HANDOFF_Y,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/cell_lot_release_qc_panel_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn all_major_modules_fit_on_panel_without_primary_overlap() {
        assert_layout_constraints();
        assert!(BASE_X >= 1100.0);
        assert!(BASE_Y >= 720.0);
        assert!(EVIDENCE_CENTER_Y + EVIDENCE_Y / 2.0 < HANDOFF_CENTER_Y - HANDOFF_Y / 2.0);
        assert!(SEGREGATION_CENTER_Y - SEGREGATION_Y / 2.0 >= -BASE_Y / 2.0);
    }

    #[test]
    fn sample_receiving_and_handoff_counts_match_closed_path() {
        assert_eq!(RECEIVING_POSITIONS, RECEIVING_COLS * RECEIVING_ROWS);
        assert_eq!(HANDOFF_PORTS, RECEIVING_POSITIONS);
        assert!(RECEIVING_PITCH_X > SAMPLE_CAP_CLEARANCE_D + 24.0);
        assert!(RECEIVING_PITCH_Y > SAMPLE_CAP_CLEARANCE_D + 24.0);
        assert!(handoff_port_x(0).abs() < HANDOFF_X / 2.0 - 90.0);
        assert!(handoff_port_x(HANDOFF_PORTS - 1).abs() < HANDOFF_X / 2.0 - 90.0);
    }

    #[test]
    fn analyzer_dock_reserves_small_benchtop_counter_envelope() {
        assert!(ANALYZER_ENV_X >= 400.0);
        assert!(ANALYZER_ENV_Y >= 250.0);
        assert!(ANALYZER_ENV_Z >= 230.0);
        assert!(ANALYZER_DOCK_X > ANALYZER_ENV_X + 40.0);
        assert!(ANALYZER_DOCK_Y > ANALYZER_ENV_Y + 30.0);
        assert!(VIABILITY_CASSETTES >= RECEIVING_POSITIONS);
    }

    #[test]
    fn custody_and_status_capacity_covers_release_workflow() {
        assert_eq!(STATUS_LANES, 3);
        assert!(STATUS_SLOTS_PER_LANE >= 5);
        assert!(STERILITY_SLOTS >= 6);
        assert!(MYCOPLASMA_SLOTS >= RECEIVING_POSITIONS);
        assert!(RETAIN_SLOTS >= 4);
        assert!(BARCODE_LANDS >= 12);
    }

    #[test]
    fn archive_grid_stays_inside_cold_block() {
        assert_eq!(ARCHIVE_TUBE_COUNT, 24);
        let first = archive_tube_position(0, 0);
        let last = archive_tube_position(ARCHIVE_COLS - 1, ARCHIVE_ROWS - 1);
        assert!(first.0 > -ARCHIVE_X / 2.0 + 28.0);
        assert!(last.0 < ARCHIVE_X / 2.0 - 54.0);
        assert!(last.1 > -ARCHIVE_Y / 2.0 + 28.0);
        assert!(first.1 < ARCHIVE_Y / 2.0 - 28.0);
        assert!(ARCHIVE_PITCH_X > ARCHIVE_TUBE_D + 20.0);
        assert!(ARCHIVE_PITCH_Y > ARCHIVE_TUBE_D + 20.0);
    }
}
