use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Robotic spill-response and localized decontamination kit station for the
// closed culture support pod.
//
// Intent:
// - Stage contained spill pads, neutralizer/disinfectant cartridges, absorbent
//   cassettes, wipe tools, evidence vials, and used-material quarantine in a
//   physically segregated station that a robot or assisted operator can service.
// - Make response consumables, contact-time evidence, scan/run-record lands,
//   clean/used/reject lanes, leak containment, robot grip datums, and service
//   clearances explicit for automation planning.
// - Keep geometry as architecture/fit CAD. It does not define a validated
//   disinfection cycle, spill protocol, or biological safety claim.

const OUTPUTS: &[&str] = &[
    "output/robotic_spill_response_decon_kit_station_drip_leak_tray.stl",
    "output/robotic_spill_response_decon_kit_station_sealed_spill_pad_cassette.stl",
    "output/robotic_spill_response_decon_kit_station_disinfectant_neutralizer_cartridge_docks.stl",
    "output/robotic_spill_response_decon_kit_station_absorbent_roll_cassette_pockets.stl",
    "output/robotic_spill_response_decon_kit_station_wipe_tool_shuttle.stl",
    "output/robotic_spill_response_decon_kit_station_evidence_vial_custody_holder.stl",
    "output/robotic_spill_response_decon_kit_station_used_material_quarantine_bin.stl",
    "output/robotic_spill_response_decon_kit_station_contact_time_token_lands.stl",
    "output/robotic_spill_response_decon_kit_station_barcode_run_record_scan_lands.stl",
    "output/robotic_spill_response_decon_kit_station_clean_used_reject_lanes.stl",
    "output/robotic_spill_response_decon_kit_station_robot_gripper_datums.stl",
    "output/robotic_spill_response_decon_kit_station_service_keepouts.stl",
    "output/robotic_spill_response_decon_kit_station_assembly.stl",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const DRAIN_D: f64 = 16.0;
const SENSOR_WELL_D: f64 = 22.0;

const SPILL_PAD_CENTER: (f64, f64) = (-405.0, 205.0);
const SPILL_PAD_X: f64 = 380.0;
const SPILL_PAD_Y: f64 = 280.0;
const SPILL_PAD_Z: f64 = 118.0;
const SPILL_PAD_COUNT: usize = 24;
const PAD_STACKS: usize = 4;
const PAD_PER_STACK: usize = SPILL_PAD_COUNT / PAD_STACKS;

const CARTRIDGE_CENTER: (f64, f64) = (0.0, 232.0);
const CARTRIDGE_PANEL_X: f64 = 462.0;
const CARTRIDGE_PANEL_Y: f64 = 230.0;
const CARTRIDGE_PANEL_Z: f64 = 48.0;
const DISINFECTANT_CARTRIDGES: usize = 3;
const NEUTRALIZER_CARTRIDGES: usize = 3;
const CARTRIDGE_COUNT: usize = DISINFECTANT_CARTRIDGES + NEUTRALIZER_CARTRIDGES;
const CARTRIDGE_D: f64 = 54.0;
const CARTRIDGE_CLEARANCE_D: f64 = 62.0;
const CARTRIDGE_Z: f64 = 146.0;
const CARTRIDGE_PITCH_X: f64 = 82.0;
const CARTRIDGE_ROW_Y: f64 = 52.0;

const ABSORBENT_CENTER: (f64, f64) = (392.0, 205.0);
const ABSORBENT_X: f64 = 360.0;
const ABSORBENT_Y: f64 = 270.0;
const ABSORBENT_Z: f64 = 54.0;
const ABSORBENT_ROLLS: usize = 3;
const ABSORBENT_CASSETTES: usize = 4;
const ABSORBENT_ROLL_D: f64 = 72.0;
const ABSORBENT_ROLL_LEN: f64 = 206.0;

const WIPE_TOOL_CENTER: (f64, f64) = (-425.0, -105.0);
const WIPE_TOOL_X: f64 = 330.0;
const WIPE_TOOL_Y: f64 = 270.0;
const WIPE_TOOL_Z: f64 = 44.0;
const WIPE_TOOLS: usize = 6;
const WIPE_TOOL_PITCH_Y: f64 = 36.0;
const WET_WIPE_CASSETTES: usize = 2;

const EVIDENCE_CENTER: (f64, f64) = (-85.0, -184.0);
const EVIDENCE_X: f64 = 370.0;
const EVIDENCE_Y: f64 = 260.0;
const EVIDENCE_Z: f64 = 52.0;
const EVIDENCE_VIALS: usize = 16;
const EVIDENCE_COLS: usize = 4;
const EVIDENCE_ROWS: usize = 4;
const EVIDENCE_VIAL_D: f64 = 16.0;
const EVIDENCE_CAP_D: f64 = 24.0;
const EVIDENCE_PITCH_X: f64 = 52.0;
const EVIDENCE_PITCH_Y: f64 = 48.0;
const SWAB_EVIDENCE_SLOTS: usize = 4;

const QUARANTINE_CENTER: (f64, f64) = (382.0, -122.0);
const QUARANTINE_X: f64 = 380.0;
const QUARANTINE_Y: f64 = 300.0;
const QUARANTINE_Z: f64 = 228.0;
const QUARANTINE_LID_Z: f64 = 26.0;
const QUARANTINE_VENT_FILTERS: usize = 3;
const BAG_COLLAR_X: f64 = 190.0;
const BAG_COLLAR_Y: f64 = 128.0;

const TOKEN_CENTER: (f64, f64) = (12.0, 354.0);
const TOKEN_PANEL_X: f64 = 480.0;
const TOKEN_PANEL_Y: f64 = 72.0;
const TOKEN_PANEL_Z: f64 = 24.0;
const CONTACT_TIME_TOKENS: usize = 12;
const TOKEN_PITCH_X: f64 = 36.0;
const TOKEN_LANE_COUNT: usize = 3;

const SCAN_CENTER: (f64, f64) = (-392.0, -360.0);
const SCAN_PANEL_X: f64 = 396.0;
const SCAN_PANEL_Y: f64 = 72.0;
const SCAN_PANEL_Z: f64 = 22.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 8;

const LANE_CENTER: (f64, f64) = (278.0, -350.0);
const LANE_PANEL_X: f64 = 486.0;
const LANE_PANEL_Y: f64 = 82.0;
const LANE_PANEL_Z: f64 = 26.0;
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 82.0;
const STATUS_SLOT_Y: f64 = 38.0;

const ROBOT_DATUMS: usize = 12;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 430.0;
const REAR_CARTRIDGE_SERVICE_Y: f64 = 260.0;
const RIGHT_BIN_SERVICE_X: f64 = 260.0;
const TOP_BIN_LIFT_Z: f64 = 310.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let tray = drip_leak_tray();
    export(&tray, OUTPUTS[0]);

    let spill_pad = sealed_spill_pad_cassette();
    export(&spill_pad, OUTPUTS[1]);

    let cartridges = disinfectant_neutralizer_cartridge_docks();
    export(&cartridges, OUTPUTS[2]);

    let absorbent = absorbent_roll_cassette_pockets();
    export(&absorbent, OUTPUTS[3]);

    let wipe_tools = wipe_tool_shuttle();
    export(&wipe_tools, OUTPUTS[4]);

    let evidence = evidence_vial_custody_holder();
    export(&evidence, OUTPUTS[5]);

    let quarantine = used_material_quarantine_bin();
    export(&quarantine, OUTPUTS[6]);

    let tokens = contact_time_token_lands();
    export(&tokens, OUTPUTS[7]);

    let scans = barcode_run_record_scan_lands();
    export(&scans, OUTPUTS[8]);

    let lanes = clean_used_reject_lanes();
    export(&lanes, OUTPUTS[9]);

    let datums = robot_gripper_datums();
    export(&datums, OUTPUTS[10]);

    let keepouts = service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = tray
        + spill_pad.translate(SPILL_PAD_CENTER.0, SPILL_PAD_CENTER.1, DECK_Z)
        + cartridges.translate(CARTRIDGE_CENTER.0, CARTRIDGE_CENTER.1, DECK_Z)
        + absorbent.translate(ABSORBENT_CENTER.0, ABSORBENT_CENTER.1, DECK_Z)
        + wipe_tools.translate(WIPE_TOOL_CENTER.0, WIPE_TOOL_CENTER.1, DECK_Z)
        + evidence.translate(EVIDENCE_CENTER.0, EVIDENCE_CENTER.1, DECK_Z)
        + quarantine.translate(QUARANTINE_CENTER.0, QUARANTINE_CENTER.1, DECK_Z)
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z)
        + scans.translate(SCAN_CENTER.0, SCAN_CENTER.1, DECK_Z)
        + lanes.translate(LANE_CENTER.0, LANE_CENTER.1, DECK_Z)
        + datums.translate(0.0, 0.0, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(&assembly, OUTPUTS[12]);

    println!();
    println!("Robotic spill response/decon kit station:");
    println!(
        "  Deck/tray:                {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm with raised rim, sensor wells, and {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Spill pads:               {SPILL_PAD_X:.0}mm x {SPILL_PAD_Y:.0}mm sealed cassette with {SPILL_PAD_COUNT} pads in {PAD_STACKS} indexed stacks"
    );
    println!(
        "  Chemistry cartridges:     {DISINFECTANT_CARTRIDGES} disinfectant + {NEUTRALIZER_CARTRIDGES} neutralizer docks, {CARTRIDGE_D:.0}mm cartridge envelope"
    );
    println!(
        "  Absorbents/wipes:         {ABSORBENT_ROLLS} roll pockets, {ABSORBENT_CASSETTES} absorbent cassettes, {WIPE_TOOLS} wipe-tool slots, {WET_WIPE_CASSETTES} wet-wipe cassette pockets"
    );
    println!(
        "  Evidence/run record:      {EVIDENCE_VIALS} evidence vials, {SWAB_EVIDENCE_SLOTS} swab slots, {CONTACT_TIME_TOKENS} contact-time tokens, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands"
    );
    println!(
        "  Waste quarantine:         {QUARANTINE_X:.0}mm x {QUARANTINE_Y:.0}mm x {QUARANTINE_Z:.0}mm used-material bin with bag collar and {QUARANTINE_VENT_FILTERS} vent/filter placeholders"
    );
    println!(
        "  Automation clearances:    front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear cartridge service {REAR_CARTRIDGE_SERVICE_Y:.0}mm, right bin service {RIGHT_BIN_SERVICE_X:.0}mm, top lift {TOP_BIN_LIFT_Z:.0}mm"
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13, "unexpected STL output count");
    assert_eq!(
        CARTRIDGE_COUNT, 6,
        "station expects six chemical cartridge docks"
    );
    assert_eq!(
        EVIDENCE_VIALS,
        EVIDENCE_COLS * EVIDENCE_ROWS,
        "evidence holder grid count changed"
    );
    assert_eq!(
        SPILL_PAD_COUNT,
        PAD_STACKS * PAD_PER_STACK,
        "spill-pad magazine count must divide evenly into indexed stacks"
    );

    for (center, x, y, margin, label) in [
        (
            SPILL_PAD_CENTER,
            SPILL_PAD_X,
            SPILL_PAD_Y,
            18.0,
            "spill pad cassette",
        ),
        (
            CARTRIDGE_CENTER,
            CARTRIDGE_PANEL_X,
            CARTRIDGE_PANEL_Y,
            18.0,
            "cartridge docks",
        ),
        (
            ABSORBENT_CENTER,
            ABSORBENT_X,
            ABSORBENT_Y,
            18.0,
            "absorbent pockets",
        ),
        (
            WIPE_TOOL_CENTER,
            WIPE_TOOL_X,
            WIPE_TOOL_Y,
            18.0,
            "wipe tool shuttle",
        ),
        (
            EVIDENCE_CENTER,
            EVIDENCE_X,
            EVIDENCE_Y,
            18.0,
            "evidence holder",
        ),
        (
            QUARANTINE_CENTER,
            QUARANTINE_X,
            QUARANTINE_Y,
            18.0,
            "quarantine bin",
        ),
        (
            TOKEN_CENTER,
            TOKEN_PANEL_X,
            TOKEN_PANEL_Y,
            12.0,
            "contact-time tokens",
        ),
        (SCAN_CENTER, SCAN_PANEL_X, SCAN_PANEL_Y, 12.0, "scan lands"),
        (
            LANE_CENTER,
            LANE_PANEL_X,
            LANE_PANEL_Y,
            12.0,
            "status lanes",
        ),
    ] {
        assert!(fits_on_deck(center, x, y, margin), "{label} off deck");
    }

    assert!(
        QUARANTINE_CENTER.0 - SPILL_PAD_CENTER.0 > 720.0,
        "used-material quarantine should be physically separated from clean spill pads"
    );
    assert!(
        (QUARANTINE_CENTER.1 - EVIDENCE_CENTER.1).abs() > 48.0,
        "quarantine bin should not crowd evidence custody area"
    );
}

fn drip_leak_tray() -> Part {
    let deck = centered_cube(
        "spill_response_station_secondary_containment_floor",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let front_rim = centered_cube(
        "spill_response_station_front_retention_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -(DECK_Y / 2.0 - RIM_W / 2.0), DECK_Z + RIM_Z / 2.0);
    let rear_rim = centered_cube(
        "spill_response_station_rear_retention_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left_rim = centered_cube(
        "spill_response_station_left_retention_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, DECK_Z + RIM_Z / 2.0);
    let right_rim = centered_cube(
        "spill_response_station_right_retention_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    let low_sump = centered_cube(
        "spill_response_station_low_point_sump_relief",
        520.0,
        34.0,
        8.0,
    )
    .translate(240.0, -(DECK_Y / 2.0 - 62.0), DECK_Z - 2.0);
    let drain = centered_cylinder(
        "spill_response_station_closed_drain_port_clearance",
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 96.0, -(DECK_Y / 2.0 + 2.0), 12.0);
    let leak_sensor_trace = centered_cube(
        "spill_response_station_leak_sensor_trace_groove",
        DECK_X - 174.0,
        12.0,
        12.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 - 98.0), DECK_Z - 1.0);

    deck + front_rim + rear_rim + left_rim + right_rim - low_sump - drain - leak_sensor_trace
        + sensor_well_set()
        + mount_boss_set()
        + clean_dirty_floor_lip()
}

fn sealed_spill_pad_cassette() -> Part {
    let base = centered_cube(
        "spill_response_sealed_pad_cassette_base",
        SPILL_PAD_X,
        SPILL_PAD_Y,
        44.0,
    )
    .translate(0.0, 0.0, 22.0);
    let magazine = centered_cube(
        "spill_response_sealed_pad_magazine_envelope",
        SPILL_PAD_X - 58.0,
        SPILL_PAD_Y - 70.0,
        SPILL_PAD_Z,
    )
    .translate(0.0, 12.0, 44.0 + SPILL_PAD_Z / 2.0);
    let pick_window = centered_cube(
        "spill_response_sealed_pad_pick_window",
        SPILL_PAD_X - 116.0,
        46.0,
        58.0,
    )
    .translate(0.0, -(SPILL_PAD_Y / 2.0 - 44.0), 98.0);
    let rear_gasket = centered_cube(
        "spill_response_sealed_pad_rear_gasket_land",
        SPILL_PAD_X - 94.0,
        12.0,
        10.0,
    )
    .translate(0.0, SPILL_PAD_Y / 2.0 - 54.0, 164.0);

    let mut pads = Part::empty("spill_response_indexed_pad_stacks");
    for stack in 0..PAD_STACKS {
        let x = index_offset(stack, PAD_STACKS, 74.0);
        pads = pads + pad_stack(stack).translate(x, 14.0, 62.0);
    }

    let mut detents = Part::empty("spill_response_pad_cassette_index_detents");
    for stack in 0..PAD_STACKS {
        let x = index_offset(stack, PAD_STACKS, 74.0);
        detents = detents
            + centered_cube(
                format!("spill_response_pad_stack_index_notch_{stack}"),
                44.0,
                10.0,
                12.0,
            )
            .translate(x, -(SPILL_PAD_Y / 2.0 - 16.0), 52.0)
            + centered_cube(
                format!("spill_response_pad_stack_robot_pull_lip_{stack}"),
                58.0,
                8.0,
                16.0,
            )
            .translate(x, -(SPILL_PAD_Y / 2.0 - 38.0), 82.0);
    }
    let mut seal_witness_buttons = Part::empty("spill_response_pad_cassette_seal_witness_buttons");
    for (i, (x, y)) in [
        (-(SPILL_PAD_X / 2.0 - 36.0), -(SPILL_PAD_Y / 2.0 - 32.0)),
        (SPILL_PAD_X / 2.0 - 36.0, -(SPILL_PAD_Y / 2.0 - 32.0)),
        (-(SPILL_PAD_X / 2.0 - 36.0), SPILL_PAD_Y / 2.0 - 32.0),
        (SPILL_PAD_X / 2.0 - 36.0, SPILL_PAD_Y / 2.0 - 32.0),
    ]
    .iter()
    .enumerate()
    {
        seal_witness_buttons = seal_witness_buttons
            + centered_cylinder(
                format!("spill_response_pad_cassette_seal_witness_button_{i}"),
                8.0,
                10.0,
                32,
            )
            .translate(*x, *y, 170.0);
    }

    base + (magazine - pick_window)
        + rear_gasket
        + pads
        + detents
        + seal_witness_buttons
        + cassette_latch_lugs("spill_pad", SPILL_PAD_X, SPILL_PAD_Y, 76.0)
}

fn disinfectant_neutralizer_cartridge_docks() -> Part {
    let panel = centered_cube(
        "spill_response_chemistry_cartridge_dock_panel",
        CARTRIDGE_PANEL_X,
        CARTRIDGE_PANEL_Y,
        CARTRIDGE_PANEL_Z,
    )
    .translate(0.0, 0.0, CARTRIDGE_PANEL_Z / 2.0);
    let back_splash = centered_cube(
        "spill_response_chemistry_rear_drip_splash_guard",
        CARTRIDGE_PANEL_X - 40.0,
        16.0,
        96.0,
    )
    .translate(0.0, CARTRIDGE_PANEL_Y / 2.0 - 16.0, 98.0);

    let mut clearance_holes = Part::empty("spill_response_cartridge_clearance_holes");
    let mut collars = Part::empty("spill_response_cartridge_locking_collars");
    let mut envelopes = Part::empty("spill_response_cartridge_envelopes");
    for i in 0..CARTRIDGE_COUNT {
        let x = index_offset(i % 3, 3, CARTRIDGE_PITCH_X);
        let y = if i < DISINFECTANT_CARTRIDGES {
            -CARTRIDGE_ROW_Y
        } else {
            CARTRIDGE_ROW_Y
        };
        let label = if i < DISINFECTANT_CARTRIDGES {
            "disinfectant"
        } else {
            "neutralizer"
        };
        clearance_holes = clearance_holes
            + centered_cylinder(
                format!("spill_response_{label}_cartridge_clearance_{i}"),
                CARTRIDGE_CLEARANCE_D / 2.0,
                CARTRIDGE_PANEL_Z + 8.0,
                40,
            )
            .translate(x, y, CARTRIDGE_PANEL_Z / 2.0);
        collars = collars
            + annular_collar(
                &format!("spill_response_{label}_cartridge_bayonet_collar_{i}"),
                78.0,
                CARTRIDGE_CLEARANCE_D,
                12.0,
            )
            .translate(x, y, CARTRIDGE_PANEL_Z + 7.0);
        envelopes = envelopes
            + centered_cylinder(
                format!("spill_response_{label}_cartridge_envelope_{i}"),
                CARTRIDGE_D / 2.0,
                CARTRIDGE_Z,
                40,
            )
            .translate(x, y, CARTRIDGE_PANEL_Z + CARTRIDGE_Z / 2.0 + 8.0)
            + centered_cube(
                format!("spill_response_{label}_lot_label_window_{i}"),
                42.0,
                7.0,
                42.0,
            )
            .translate(x, y - CARTRIDGE_D / 2.0 - 4.0, CARTRIDGE_PANEL_Z + 96.0);
    }

    (panel - clearance_holes) + collars + envelopes + back_splash + chemistry_manifold_ports()
}

fn absorbent_roll_cassette_pockets() -> Part {
    let deck = centered_cube(
        "spill_response_absorbent_module_deck",
        ABSORBENT_X,
        ABSORBENT_Y,
        ABSORBENT_Z,
    )
    .translate(0.0, 0.0, ABSORBENT_Z / 2.0);
    let rear_guard = centered_cube(
        "spill_response_absorbent_rear_retainer",
        ABSORBENT_X - 40.0,
        14.0,
        82.0,
    )
    .translate(0.0, ABSORBENT_Y / 2.0 - 18.0, ABSORBENT_Z + 41.0);

    let mut roll_pockets = Part::empty("spill_response_absorbent_roll_pockets");
    for i in 0..ABSORBENT_ROLLS {
        let y = index_offset(i, ABSORBENT_ROLLS, 76.0);
        roll_pockets = roll_pockets
            + centered_cylinder(
                format!("spill_response_absorbent_roll_cradle_{i}"),
                ABSORBENT_ROLL_D / 2.0,
                ABSORBENT_ROLL_LEN,
                40,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(-54.0, y, ABSORBENT_Z + ABSORBENT_ROLL_D / 2.0)
            + centered_cube(
                format!("spill_response_absorbent_roll_axle_land_{i}_left"),
                12.0,
                54.0,
                34.0,
            )
            .translate(-ABSORBENT_ROLL_LEN / 2.0 - 74.0, y, ABSORBENT_Z + 52.0)
            + centered_cube(
                format!("spill_response_absorbent_roll_axle_land_{i}_right"),
                12.0,
                54.0,
                34.0,
            )
            .translate(ABSORBENT_ROLL_LEN / 2.0 - 34.0, y, ABSORBENT_Z + 52.0);
    }

    let mut cassette_pockets = Part::empty("spill_response_flat_absorbent_cassette_pockets");
    for i in 0..ABSORBENT_CASSETTES {
        let x = -130.0 + (i % 2) as f64 * 260.0;
        let y = -92.0 + (i / 2) as f64 * 64.0;
        cassette_pockets = cassette_pockets
            + centered_cube(
                format!("spill_response_flat_absorbent_cassette_pocket_{i}"),
                96.0,
                42.0,
                12.0,
            )
            .translate(x, y, ABSORBENT_Z + 10.0)
            + centered_cube(
                format!("spill_response_flat_absorbent_cassette_pick_lip_{i}"),
                78.0,
                8.0,
                16.0,
            )
            .translate(x, y - 24.0, ABSORBENT_Z + 28.0);
    }

    deck + rear_guard + roll_pockets + cassette_pockets + absorbent_sensor_strip()
}

fn wipe_tool_shuttle() -> Part {
    let tray = centered_cube(
        "spill_response_wipe_tool_shuttle_tray",
        WIPE_TOOL_X,
        WIPE_TOOL_Y,
        WIPE_TOOL_Z,
    )
    .translate(0.0, 0.0, WIPE_TOOL_Z / 2.0);
    let mut wells = Part::empty("spill_response_wipe_tool_wells");
    let mut tools = Part::empty("spill_response_wipe_tool_grip_handles");
    for i in 0..WIPE_TOOLS {
        let y = index_offset(i, WIPE_TOOLS, WIPE_TOOL_PITCH_Y);
        wells = wells
            + centered_cube(
                format!("spill_response_wipe_tool_well_cutout_{i}"),
                218.0,
                24.0,
                18.0,
            )
            .translate(-22.0, y, WIPE_TOOL_Z - 5.0);
        tools = tools
            + centered_cube(
                format!("spill_response_wipe_tool_handle_datum_{i}"),
                196.0,
                14.0,
                12.0,
            )
            .translate(-22.0, y, WIPE_TOOL_Z + 11.0)
            + centered_cube(
                format!("spill_response_wipe_tool_gripper_tab_{i}"),
                34.0,
                18.0,
                20.0,
            )
            .translate(112.0, y, WIPE_TOOL_Z + 16.0);
    }

    let mut wet_wipes = Part::empty("spill_response_wet_wipe_cassette_pockets");
    for i in 0..WET_WIPE_CASSETTES {
        let x = -102.0 + i as f64 * 122.0;
        wet_wipes = wet_wipes
            + centered_cube(
                format!("spill_response_wet_wipe_cassette_pocket_{i}"),
                94.0,
                64.0,
                14.0,
            )
            .translate(x, -(WIPE_TOOL_Y / 2.0 - 42.0), WIPE_TOOL_Z + 8.0)
            + centered_cube(
                format!("spill_response_wet_wipe_lid_grip_slot_{i}"),
                62.0,
                8.0,
                16.0,
            )
            .translate(x, -(WIPE_TOOL_Y / 2.0 - 76.0), WIPE_TOOL_Z + 26.0);
    }

    (tray - wells) + tools + wet_wipes + wipe_tool_quick_release_bar()
}

fn evidence_vial_custody_holder() -> Part {
    let holder = centered_cube(
        "spill_response_evidence_custody_holder",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_Z / 2.0);
    let mut vial_holes = Part::empty("spill_response_evidence_vial_clearance_holes");
    let mut collars = Part::empty("spill_response_evidence_vial_lip_collars");
    for row in 0..EVIDENCE_ROWS {
        for col in 0..EVIDENCE_COLS {
            let idx = row * EVIDENCE_COLS + col;
            let x = index_offset(col, EVIDENCE_COLS, EVIDENCE_PITCH_X) - 38.0;
            let y = index_offset(row, EVIDENCE_ROWS, EVIDENCE_PITCH_Y) + 26.0;
            vial_holes = vial_holes
                + centered_cylinder(
                    format!("spill_response_evidence_vial_well_{idx}"),
                    EVIDENCE_CAP_D / 2.0,
                    EVIDENCE_Z + 6.0,
                    32,
                )
                .translate(x, y, EVIDENCE_Z / 2.0);
            collars = collars
                + annular_collar(
                    &format!("spill_response_evidence_vial_retention_collar_{idx}"),
                    EVIDENCE_CAP_D + 10.0,
                    EVIDENCE_VIAL_D,
                    8.0,
                )
                .translate(x, y, EVIDENCE_Z + 4.0);
        }
    }

    let mut swab_slots = Part::empty("spill_response_swab_evidence_slots");
    for i in 0..SWAB_EVIDENCE_SLOTS {
        let y = index_offset(i, SWAB_EVIDENCE_SLOTS, 38.0) + 10.0;
        swab_slots = swab_slots
            + centered_cube(
                format!("spill_response_swab_evidence_slot_{i}"),
                118.0,
                16.0,
                16.0,
            )
            .translate(120.0, y, EVIDENCE_Z + 8.0);
    }

    let custody_card_land = centered_cube(
        "spill_response_evidence_chain_of_custody_card_land",
        142.0,
        64.0,
        8.0,
    )
    .translate(106.0, -(EVIDENCE_Y / 2.0 - 46.0), EVIDENCE_Z + 4.0);

    (holder - vial_holes) + collars + swab_slots + custody_card_land + evidence_seal_tab_strip()
}

fn used_material_quarantine_bin() -> Part {
    let shell = centered_cube(
        "spill_response_used_material_quarantine_outer_bin",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0);
    let cavity = centered_cube(
        "spill_response_used_material_quarantine_bin_cavity",
        QUARANTINE_X - 58.0,
        QUARANTINE_Y - 58.0,
        QUARANTINE_Z - 36.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0 + 18.0);
    let lid_flange = centered_cube(
        "spill_response_used_material_quarantine_lid_flange",
        QUARANTINE_X + 34.0,
        QUARANTINE_Y + 34.0,
        QUARANTINE_LID_Z,
    )
    .translate(0.0, 0.0, QUARANTINE_Z + QUARANTINE_LID_Z / 2.0);
    let bag_collar = centered_cube(
        "spill_response_used_material_bag_collar",
        BAG_COLLAR_X,
        BAG_COLLAR_Y,
        20.0,
    )
    .translate(-36.0, -12.0, QUARANTINE_Z + QUARANTINE_LID_Z + 10.0);
    let bag_collar_opening = centered_cube(
        "spill_response_used_material_bag_collar_opening",
        BAG_COLLAR_X - 44.0,
        BAG_COLLAR_Y - 36.0,
        26.0,
    )
    .translate(-36.0, -12.0, QUARANTINE_Z + QUARANTINE_LID_Z + 10.0);
    let lid_pick_slot = centered_cube(
        "spill_response_quarantine_bin_lid_robot_pick_slot",
        118.0,
        18.0,
        18.0,
    )
    .translate(92.0, -(QUARANTINE_Y / 2.0 + 15.0), QUARANTINE_Z + 24.0);

    (shell - cavity)
        + lid_flange
        + (bag_collar - bag_collar_opening)
        + lid_pick_slot
        + quarantine_vent_filter_bank()
        + quarantine_latch_and_status_lands()
}

fn contact_time_token_lands() -> Part {
    let panel = centered_cube(
        "spill_response_contact_time_token_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    )
    .translate(0.0, 0.0, TOKEN_PANEL_Z / 2.0);
    let mut tokens = Part::empty("spill_response_contact_time_tokens");
    for lane in 0..TOKEN_LANE_COUNT {
        for i in 0..(CONTACT_TIME_TOKENS / TOKEN_LANE_COUNT) {
            let idx = lane * (CONTACT_TIME_TOKENS / TOKEN_LANE_COUNT) + i;
            let x = index_offset(i, CONTACT_TIME_TOKENS / TOKEN_LANE_COUNT, TOKEN_PITCH_X)
                + index_offset(lane, TOKEN_LANE_COUNT, 158.0);
            tokens = tokens
                + centered_cube(
                    format!("spill_response_contact_time_token_slot_{idx}"),
                    26.0,
                    34.0,
                    8.0,
                )
                .translate(x, -6.0, TOKEN_PANEL_Z + 4.0)
                + centered_cube(
                    format!("spill_response_contact_time_label_land_{idx}"),
                    28.0,
                    8.0,
                    6.0,
                )
                .translate(x, 26.0, TOKEN_PANEL_Z + 3.0);
        }
    }
    let timer_dock = centered_cube("spill_response_contact_time_timer_dock", 78.0, 46.0, 14.0)
        .translate(-(TOKEN_PANEL_X / 2.0 - 62.0), 0.0, TOKEN_PANEL_Z + 7.0);

    panel + tokens + timer_dock + token_lane_dividers()
}

fn barcode_run_record_scan_lands() -> Part {
    let panel = centered_cube(
        "spill_response_run_record_scan_panel",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    )
    .translate(0.0, 0.0, SCAN_PANEL_Z / 2.0);

    let mut barcodes = Part::empty("spill_response_barcode_run_record_lands");
    for i in 0..BARCODE_LANDS {
        let x = index_offset(i, BARCODE_LANDS, 30.0);
        barcodes =
            barcodes
                + centered_cube(format!("spill_response_barcode_land_{i}"), 24.0, 20.0, 5.0)
                    .translate(x, -17.0, SCAN_PANEL_Z + 3.0);
    }

    let mut rfid = Part::empty("spill_response_rfid_lands");
    for i in 0..RFID_LANDS {
        let x = index_offset(i, RFID_LANDS, 42.0);
        rfid =
            rfid + centered_cube(format!("spill_response_rfid_land_{i}"), 34.0, 18.0, 4.0)
                .translate(x, 18.0, SCAN_PANEL_Z + 2.0);
    }

    let handheld_scan_bridge = centered_cube(
        "spill_response_scan_bridge_camera_bar",
        SCAN_PANEL_X - 70.0,
        10.0,
        86.0,
    )
    .translate(0.0, -(SCAN_PANEL_Y / 2.0 + 12.0), 66.0);

    panel + barcodes + rfid + handheld_scan_bridge + run_record_card_slots()
}

fn clean_used_reject_lanes() -> Part {
    let panel = centered_cube(
        "spill_response_clean_used_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0);
    let mut lanes = Part::empty("spill_response_clean_used_reject_lanes");
    for lane in 0..STATUS_LANES {
        let x = index_offset(lane, STATUS_LANES, 146.0);
        lanes = lanes
            + centered_cube(
                format!("spill_response_status_lane_divider_{lane}_left"),
                8.0,
                LANE_PANEL_Y,
                32.0,
            )
            .translate(x - 56.0, 0.0, LANE_PANEL_Z + 16.0)
            + centered_cube(
                format!("spill_response_status_lane_divider_{lane}_right"),
                8.0,
                LANE_PANEL_Y,
                32.0,
            )
            .translate(x + 56.0, 0.0, LANE_PANEL_Z + 16.0);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = index_offset(slot, STATUS_SLOTS_PER_LANE, 18.0);
            lanes = lanes
                + centered_cube(
                    format!("spill_response_status_lane_{lane}_token_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y / 2.0,
                    8.0,
                )
                .translate(x, y, LANE_PANEL_Z + 4.0);
        }
    }

    panel + lanes + status_lane_header_lands()
}

fn robot_gripper_datums() -> Part {
    let datum_points = [
        (SPILL_PAD_CENTER.0 - 122.0, SPILL_PAD_CENTER.1 - 118.0),
        (SPILL_PAD_CENTER.0 + 122.0, SPILL_PAD_CENTER.1 - 118.0),
        (CARTRIDGE_CENTER.0 - 184.0, CARTRIDGE_CENTER.1 - 96.0),
        (CARTRIDGE_CENTER.0 + 184.0, CARTRIDGE_CENTER.1 - 96.0),
        (ABSORBENT_CENTER.0 - 130.0, ABSORBENT_CENTER.1 - 112.0),
        (ABSORBENT_CENTER.0 + 130.0, ABSORBENT_CENTER.1 - 112.0),
        (WIPE_TOOL_CENTER.0 - 120.0, WIPE_TOOL_CENTER.1 + 116.0),
        (WIPE_TOOL_CENTER.0 + 120.0, WIPE_TOOL_CENTER.1 + 116.0),
        (EVIDENCE_CENTER.0 - 145.0, EVIDENCE_CENTER.1 - 112.0),
        (EVIDENCE_CENTER.0 + 145.0, EVIDENCE_CENTER.1 - 112.0),
        (QUARANTINE_CENTER.0 - 142.0, QUARANTINE_CENTER.1 + 126.0),
        (QUARANTINE_CENTER.0 + 142.0, QUARANTINE_CENTER.1 + 126.0),
    ];
    assert_eq!(datum_points.len(), ROBOT_DATUMS);

    let mut datums = Part::empty("spill_response_robot_gripper_datums");
    for (i, (x, y)) in datum_points.iter().enumerate() {
        datums = datums
            + centered_cube(
                format!("spill_response_robot_gripper_pad_{i}"),
                42.0,
                42.0,
                8.0,
            )
            .translate(*x, *y, 4.0)
            + centered_cylinder(
                format!("spill_response_robot_fiducial_boss_{i}"),
                9.0,
                8.0,
                28,
            )
            .translate(*x, *y, 12.0);
    }
    datums + robot_approach_gauge_arrows()
}

fn service_keepouts() -> Part {
    let front_robot = clearance_frame(
        "spill_response_front_robot_response_keepout",
        DECK_X - 130.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        260.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0), 132.0);
    let rear_cartridge = clearance_frame(
        "spill_response_rear_cartridge_change_keepout",
        620.0,
        REAR_CARTRIDGE_SERVICE_Y,
        230.0,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        DECK_Y / 2.0 + REAR_CARTRIDGE_SERVICE_Y / 2.0,
        120.0,
    );
    let right_bin = clearance_frame(
        "spill_response_right_used_material_bin_service_keepout",
        RIGHT_BIN_SERVICE_X,
        360.0,
        260.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_BIN_SERVICE_X / 2.0,
        QUARANTINE_CENTER.1,
        135.0,
    );
    let top_lift = clearance_frame(
        "spill_response_top_bin_lid_lift_keepout",
        QUARANTINE_X + 80.0,
        QUARANTINE_Y + 80.0,
        TOP_BIN_LIFT_Z,
    )
    .translate(
        QUARANTINE_CENTER.0,
        QUARANTINE_CENTER.1,
        QUARANTINE_Z + TOP_BIN_LIFT_Z / 2.0,
    );

    front_robot + rear_cartridge + right_bin + top_lift
}

fn sensor_well_set() -> Part {
    let mut wells = Part::empty("spill_response_leak_sensor_wells");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 72.0), -(DECK_Y / 2.0 - 76.0)),
        (DECK_X / 2.0 - 72.0, -(DECK_Y / 2.0 - 76.0)),
        (-(DECK_X / 2.0 - 72.0), DECK_Y / 2.0 - 76.0),
        (DECK_X / 2.0 - 72.0, DECK_Y / 2.0 - 76.0),
    ]
    .iter()
    .enumerate()
    {
        wells = wells
            + annular_collar(
                &format!("spill_response_leak_sensor_well_{i}"),
                SENSOR_WELL_D + 12.0,
                SENSOR_WELL_D,
                8.0,
            )
            .translate(*x, *y, DECK_Z + 4.0)
            + centered_cube(
                format!("spill_response_leak_sensor_wire_relief_{i}"),
                44.0,
                8.0,
                6.0,
            )
            .translate(*x, *y + 32.0, DECK_Z + 3.0);
    }
    wells
}

fn mount_boss_set() -> Part {
    let mut bosses = Part::empty("spill_response_station_mount_bosses");
    for (i, (x, y)) in [
        (-560.0, -330.0),
        (-560.0, 330.0),
        (560.0, -330.0),
        (560.0, 330.0),
        (-208.0, 0.0),
        (208.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        bosses = bosses
            + annular_collar(
                &format!("spill_response_station_mount_boss_{i}"),
                38.0,
                7.0,
                16.0,
            )
            .translate(*x, *y, DECK_Z + 8.0);
    }
    bosses
}

fn clean_dirty_floor_lip() -> Part {
    let cross_lip = centered_cube(
        "spill_response_clean_dirty_floor_lip",
        DECK_X - 118.0,
        12.0,
        18.0,
    )
    .translate(0.0, -304.0, DECK_Z + 9.0);
    let chemistry_lip = centered_cube(
        "spill_response_chemistry_to_absorbent_separation_lip",
        14.0,
        296.0,
        18.0,
    )
    .translate(186.0, 170.0, DECK_Z + 9.0);

    cross_lip + chemistry_lip
}

fn pad_stack(stack: usize) -> Part {
    let mut pads = Part::empty(format!("spill_response_pad_stack_{stack}"));
    for i in 0..PAD_PER_STACK {
        pads = pads
            + centered_cube(
                format!("spill_response_pad_stack_{stack}_pad_{i}"),
                58.0,
                178.0,
                3.2,
            )
            .translate(0.0, 0.0, i as f64 * 4.0);
    }
    pads
}

fn cassette_latch_lugs(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    let mut lugs = Part::empty(format!("{prefix}_cassette_latch_lugs"));
    for (i, (lx, ly)) in [
        (-(x / 2.0 - 34.0), -(y / 2.0 - 34.0)),
        (x / 2.0 - 34.0, -(y / 2.0 - 34.0)),
        (-(x / 2.0 - 34.0), y / 2.0 - 34.0),
        (x / 2.0 - 34.0, y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        lugs = lugs
            + centered_cube(format!("{prefix}_latch_lug_{i}"), 36.0, 24.0, 18.0)
                .translate(*lx, *ly, z);
    }
    lugs
}

fn chemistry_manifold_ports() -> Part {
    let manifold = centered_cube(
        "spill_response_chemistry_closed_handoff_manifold",
        CARTRIDGE_PANEL_X - 84.0,
        22.0,
        48.0,
    )
    .translate(0.0, -(CARTRIDGE_PANEL_Y / 2.0 + 12.0), 76.0);
    let mut ports = Part::empty("spill_response_chemistry_handoff_ports");
    for i in 0..CARTRIDGE_COUNT {
        let x = index_offset(i, CARTRIDGE_COUNT, 52.0);
        ports = ports
            + annular_collar(
                &format!("spill_response_chemistry_handoff_port_{i}"),
                24.0,
                8.0,
                10.0,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(CARTRIDGE_PANEL_Y / 2.0 + 24.0), 76.0);
    }
    manifold + ports
}

fn absorbent_sensor_strip() -> Part {
    let strip = centered_cube(
        "spill_response_absorbent_remaining_sensor_strip",
        ABSORBENT_X - 64.0,
        10.0,
        18.0,
    )
    .translate(0.0, ABSORBENT_Y / 2.0 - 44.0, ABSORBENT_Z + 22.0);
    let mut flags = Part::empty("spill_response_absorbent_remaining_flag_lands");
    for i in 0..ABSORBENT_ROLLS {
        let y = index_offset(i, ABSORBENT_ROLLS, 76.0);
        flags = flags
            + centered_cube(
                format!("spill_response_absorbent_roll_remaining_flag_{i}"),
                34.0,
                8.0,
                16.0,
            )
            .translate(136.0, y, ABSORBENT_Z + 38.0);
    }
    strip + flags
}

fn wipe_tool_quick_release_bar() -> Part {
    let bar = centered_cube(
        "spill_response_wipe_tool_quick_release_bar",
        WIPE_TOOL_X - 48.0,
        12.0,
        24.0,
    )
    .translate(0.0, WIPE_TOOL_Y / 2.0 - 34.0, WIPE_TOOL_Z + 12.0);
    let mut latch_flags = Part::empty("spill_response_wipe_tool_latch_flags");
    for i in 0..WIPE_TOOLS {
        latch_flags = latch_flags
            + centered_cube(
                format!("spill_response_wipe_tool_latch_flag_{i}"),
                18.0,
                10.0,
                22.0,
            )
            .translate(
                136.0,
                index_offset(i, WIPE_TOOLS, WIPE_TOOL_PITCH_Y),
                WIPE_TOOL_Z + 20.0,
            );
    }
    bar + latch_flags
}

fn evidence_seal_tab_strip() -> Part {
    let strip = centered_cube(
        "spill_response_evidence_tamper_seal_tab_strip",
        EVIDENCE_X - 58.0,
        10.0,
        20.0,
    )
    .translate(0.0, EVIDENCE_Y / 2.0 - 20.0, EVIDENCE_Z + 10.0);
    let mut tabs = Part::empty("spill_response_evidence_tamper_seal_tabs");
    for i in 0..8 {
        tabs = tabs
            + centered_cube(
                format!("spill_response_evidence_tamper_seal_tab_{i}"),
                28.0,
                8.0,
                10.0,
            )
            .translate(
                index_offset(i, 8, 36.0),
                EVIDENCE_Y / 2.0 - 34.0,
                EVIDENCE_Z + 5.0,
            );
    }
    strip + tabs
}

fn quarantine_vent_filter_bank() -> Part {
    let panel = centered_cube(
        "spill_response_quarantine_vent_filter_panel",
        150.0,
        20.0,
        84.0,
    )
    .translate(
        QUARANTINE_X / 2.0 - 54.0,
        QUARANTINE_Y / 2.0 + 4.0,
        QUARANTINE_Z - 54.0,
    );
    let mut filters = Part::empty("spill_response_quarantine_vent_filters");
    for i in 0..QUARANTINE_VENT_FILTERS {
        filters = filters
            + annular_collar(
                &format!("spill_response_quarantine_vent_filter_{i}"),
                34.0,
                18.0,
                12.0,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                QUARANTINE_X / 2.0 - 104.0 + i as f64 * 50.0,
                QUARANTINE_Y / 2.0 + 14.0,
                QUARANTINE_Z - 54.0,
            );
    }
    panel + filters
}

fn quarantine_latch_and_status_lands() -> Part {
    let mut latches = Part::empty("spill_response_quarantine_latch_lands");
    for i in 0..4 {
        let x = if i % 2 == 0 {
            -(QUARANTINE_X / 2.0 + 8.0)
        } else {
            QUARANTINE_X / 2.0 + 8.0
        };
        let y = if i < 2 { -74.0 } else { 74.0 };
        latches = latches
            + centered_cube(
                format!("spill_response_quarantine_latch_land_{i}"),
                18.0,
                54.0,
                28.0,
            )
            .translate(x, y, QUARANTINE_Z - 42.0);
    }
    let status_land = centered_cube(
        "spill_response_quarantine_bin_status_scan_land",
        118.0,
        28.0,
        8.0,
    )
    .translate(
        -(QUARANTINE_X / 2.0 - 78.0),
        -(QUARANTINE_Y / 2.0 + 8.0),
        74.0,
    );
    latches + status_land
}

fn token_lane_dividers() -> Part {
    let mut dividers = Part::empty("spill_response_contact_time_lane_dividers");
    for i in 0..=TOKEN_LANE_COUNT {
        let x = -TOKEN_PANEL_X / 2.0 + 84.0 + i as f64 * 158.0;
        dividers = dividers
            + centered_cube(
                format!("spill_response_contact_time_lane_divider_{i}"),
                6.0,
                TOKEN_PANEL_Y - 14.0,
                18.0,
            )
            .translate(x, 0.0, TOKEN_PANEL_Z + 9.0);
    }
    dividers
}

fn run_record_card_slots() -> Part {
    let mut slots = Part::empty("spill_response_run_record_card_slots");
    for i in 0..3 {
        slots = slots
            + centered_cube(
                format!("spill_response_run_record_card_slot_{i}"),
                82.0,
                8.0,
                22.0,
            )
            .translate(
                index_offset(i, 3, 100.0),
                SCAN_PANEL_Y / 2.0 + 6.0,
                SCAN_PANEL_Z + 11.0,
            );
    }
    slots
}

fn status_lane_header_lands() -> Part {
    let mut headers = Part::empty("spill_response_status_lane_header_lands");
    for lane in 0..STATUS_LANES {
        let x = index_offset(lane, STATUS_LANES, 146.0);
        headers = headers
            + centered_cube(
                format!("spill_response_status_lane_header_{lane}"),
                92.0,
                12.0,
                8.0,
            )
            .translate(x, LANE_PANEL_Y / 2.0 + 6.0, LANE_PANEL_Z + 4.0);
    }
    headers
}

fn robot_approach_gauge_arrows() -> Part {
    let front_gauge = centered_cube(
        "spill_response_robot_front_approach_gauge",
        DECK_X - 180.0,
        18.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(0.0, -(DECK_Y / 2.0 + 26.0), KEEP_OUT_GAUGE_Z / 2.0);
    let chemistry_gauge = centered_cube(
        "spill_response_cartridge_change_approach_gauge",
        CARTRIDGE_PANEL_X,
        14.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1 + 142.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let waste_gauge = centered_cube(
        "spill_response_quarantine_bin_service_approach_gauge",
        14.0,
        QUARANTINE_Y + 60.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        QUARANTINE_CENTER.0 + QUARANTINE_X / 2.0 + 38.0,
        QUARANTINE_CENTER.1,
        KEEP_OUT_GAUGE_Z / 2.0,
    );

    front_gauge + chemistry_gauge + waste_gauge
}

fn clearance_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let rail = 10.0;
    let mut frame = Part::empty(format!("{name}_frame"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        frame = frame
            + centered_cube(format!("{name}_post_{i}"), rail, rail, z).translate(
                sx * (x / 2.0 - rail / 2.0),
                sy * (y / 2.0 - rail / 2.0),
                0.0,
            );
    }
    for (i, zz) in [-z / 2.0, z / 2.0].iter().enumerate() {
        frame = frame
            + centered_cube(format!("{name}_front_rail_{i}"), x, rail, rail).translate(
                0.0,
                -(y / 2.0 - rail / 2.0),
                *zz,
            )
            + centered_cube(format!("{name}_rear_rail_{i}"), x, rail, rail).translate(
                0.0,
                y / 2.0 - rail / 2.0,
                *zz,
            )
            + centered_cube(format!("{name}_left_rail_{i}"), rail, y, rail).translate(
                -(x / 2.0 - rail / 2.0),
                0.0,
                *zz,
            )
            + centered_cube(format!("{name}_right_rail_{i}"), rail, y, rail).translate(
                x / 2.0 - rail / 2.0,
                0.0,
                *zz,
            );
    }
    frame
}

fn annular_collar(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 40)
        - centered_cylinder(
            format!("{name}_inner_clearance"),
            inner_d / 2.0,
            z + 2.0,
            40,
        )
}

fn index_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0 - x / 2.0 >= -DECK_X / 2.0 + margin
        && center.0 + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1 - y / 2.0 >= -DECK_Y / 2.0 + margin
        && center.1 + y / 2.0 <= DECK_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_outputs_are_station_scoped() {
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/robotic_spill_response_decon_kit_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn layout_fits_on_bench_tray() {
        assert_layout();
    }

    #[test]
    fn consumable_capacity_is_useful_for_local_response() {
        assert!(SPILL_PAD_COUNT >= 20);
        assert!(CARTRIDGE_COUNT >= 6);
        assert!(ABSORBENT_ROLLS >= 3);
        assert!(WIPE_TOOLS >= 6);
        assert!(EVIDENCE_VIALS >= 16);
    }

    #[test]
    fn clean_and_used_materials_are_physically_segregated() {
        let clean_zone_x = SPILL_PAD_CENTER.0 + SPILL_PAD_X / 2.0;
        let used_zone_x = QUARANTINE_CENTER.0 - QUARANTINE_X / 2.0;
        assert!(used_zone_x - clean_zone_x > 390.0);
        assert!(LANE_CENTER.1 < -300.0);
    }

    #[test]
    fn service_clearances_are_robot_accessible() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 400.0);
        assert!(REAR_CARTRIDGE_SERVICE_Y >= 240.0);
        assert!(RIGHT_BIN_SERVICE_X >= 240.0);
        assert!(TOP_BIN_LIFT_Z > QUARANTINE_Z);
    }

    #[test]
    fn traceability_features_are_explicit() {
        assert!(BARCODE_LANDS >= 12);
        assert!(RFID_LANDS >= 8);
        assert!(CONTACT_TIME_TOKENS >= 12);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(ROBOT_DATUMS, 12);
    }
}
