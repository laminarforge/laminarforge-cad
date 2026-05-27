use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator slot reassignment after excursion balance station.
//
// This standalone generator models a no-cell validation fixture for proving
// that cassettes can be reassigned after a local temperature/CO2/humidity
// excursion without introducing slot-position bias. The physical workflow is:
// capture the original slot map, tag the local excursion, move cassettes by a
// balanced permutation, route affected articles through quarantine/recovery
// lanes, pair environmental witness coupons, and require release/hold/reject
// evidence gates before returning material to the run.

const OUTPUT_PREFIX: &str = "closed_incubator_slot_reassignment_after_excursion_balance_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_base_deck.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_slot_map_board.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_excursion_marker_tokens.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_balanced_reassignment_grid.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_quarantine_recovery_lanes.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_environmental_equivalence_witness_coupons.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_temp_co2_rh_logger_evidence_docks.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_release_hold_reject_evidence_gates.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_barcode_custody_transfer_lands.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_bias_balance_dummy_load_equalizers.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_robot_service_keepouts.stl",
    "output/closed_incubator_slot_reassignment_after_excursion_balance_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "slot_map_board",
    "temperature_co2_humidity_excursion_marker_tokens",
    "balanced_reassignment_grid",
    "quarantine_lane",
    "recovery_lane",
    "matched_control_lane",
    "environmental_equivalence_witness_coupons",
    "temp_co2_rh_logger_evidence_docks",
    "release_hold_reject_evidence_gates",
    "barcode_custody_transfer_lands",
    "bias_balance_dummy_load_equalizers",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const SLOT_COLS: usize = 4;
const SLOT_ROWS: usize = 4;
const SLOT_COUNT: usize = SLOT_COLS * SLOT_ROWS;
const CORNER_SLOT_COUNT: usize = 4;
const EDGE_SLOT_COUNT: usize = 8;
const CENTER_SLOT_COUNT: usize = 4;
const SLOT_CLASS_COUNT: usize = 3;

const EXCURSION_SLOTS: [usize; 4] = [0, 1, 4, 5];
const RECOVERY_TARGET_SLOTS: [usize; 4] = [10, 11, 14, 15];
const REASSIGNMENT: [usize; SLOT_COUNT] = [10, 11, 8, 9, 14, 15, 12, 13, 2, 3, 0, 1, 6, 7, 4, 5];

const ENV_CHANNEL_COUNT: usize = 3;
const ENV_WITNESS_PAIR_COUNT: usize = EXCURSION_SLOTS.len();
const ENV_WITNESS_COUPON_COUNT: usize = ENV_CHANNEL_COUNT * ENV_WITNESS_PAIR_COUNT * 2;
const EXCURSION_MARKER_COUNT: usize = 4;
const LOGGER_DOCK_COUNT: usize = 4;
const PROCESS_LANE_COUNT: usize = 3;
const EVIDENCE_GATE_COUNT: usize = 3;

const DECK_X: f64 = 1420.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 38.0;
const MOUNT_HOLE_D: f64 = 6.6;
const MODULE_RECESS_DEPTH: f64 = 4.0;

const SLOT_MAP_CENTER: (f64, f64) = (-515.0, 240.0);
const SLOT_MAP_X: f64 = 330.0;
const SLOT_MAP_Y: f64 = 300.0;
const SLOT_MAP_Z: f64 = 22.0;
const SLOT_MAP_CELL_X: f64 = 56.0;
const SLOT_MAP_CELL_Y: f64 = 44.0;
const SLOT_MAP_PITCH_X: f64 = 65.0;
const SLOT_MAP_PITCH_Y: f64 = 52.0;
const SLOT_MAP_TOKEN_D: f64 = 16.0;

const MARKER_CENTER: (f64, f64) = (120.0, 365.0);
const MARKER_PANEL_X: f64 = 520.0;
const MARKER_PANEL_Y: f64 = 150.0;
const MARKER_PANEL_Z: f64 = 22.0;
const MARKER_TOKEN_D: f64 = 26.0;
const MARKER_TOKEN_Z: f64 = 7.0;
const AFFECTED_SLOT_TOKEN_D: f64 = 17.0;

const GRID_CENTER: (f64, f64) = (0.0, 25.0);
const GRID_MARGIN_X: f64 = 42.0;
const GRID_MARGIN_Y: f64 = 38.0;
const GRID_SLOT_X: f64 = REVC_CHIP_LENGTH + 12.0;
const GRID_SLOT_Y: f64 = REVC_CHIP_WIDTH + 10.0;
const GRID_SLOT_Z: f64 = REVC_TOTAL_HEIGHT + 22.0;
const GRID_GAP_X: f64 = 10.0;
const GRID_GAP_Y: f64 = 8.0;
const GRID_X: f64 =
    SLOT_COLS as f64 * GRID_SLOT_X + (SLOT_COLS as f64 - 1.0) * GRID_GAP_X + 2.0 * GRID_MARGIN_X;
const GRID_Y: f64 =
    SLOT_ROWS as f64 * GRID_SLOT_Y + (SLOT_ROWS as f64 - 1.0) * GRID_GAP_Y + 2.0 * GRID_MARGIN_Y;
const GRID_Z: f64 = 40.0;
const GRID_RECESS_DEPTH: f64 = REVC_TOTAL_HEIGHT + 8.0;
const GRID_DOCK_RAIL_W: f64 = 12.0;
const GRID_DOCK_RAIL_Z: f64 = 26.0;
const ASSIGNMENT_TOKEN_D: f64 = 13.0;
const RECOVERY_HALO_D: f64 = 24.0;

const LANE_CENTER: (f64, f64) = (510.0, -95.0);
const LANE_PANEL_X: f64 = 340.0;
const LANE_PANEL_Y: f64 = 300.0;
const LANE_PANEL_Z: f64 = 30.0;
const PROCESS_LANE_X: f64 = 104.0;
const PROCESS_LANE_Y: f64 = 240.0;
const PROCESS_LANE_PITCH_X: f64 = 116.0;
const PROCESS_LANE_RECESS_DEPTH: f64 = 11.0;
const PROCESS_LANE_WALL_W: f64 = 8.0;
const PROCESS_LANE_CAPACITY: usize = EXCURSION_SLOTS.len();

const WITNESS_CENTER: (f64, f64) = (-525.0, -190.0);
const WITNESS_PANEL_X: f64 = 330.0;
const WITNESS_PANEL_Y: f64 = 310.0;
const WITNESS_PANEL_Z: f64 = 24.0;
const WITNESS_COUPON_D: f64 = 18.0;
const WITNESS_COUPON_Z: f64 = 5.0;
const WITNESS_PAIR_PITCH_X: f64 = 56.0;
const WITNESS_CHANNEL_PITCH_Y: f64 = 78.0;
const WITNESS_PAIR_OFFSET_Y: f64 = 15.0;

const LOGGER_CENTER: (f64, f64) = (520.0, 170.0);
const LOGGER_PANEL_X: f64 = 340.0;
const LOGGER_PANEL_Y: f64 = 210.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_POCKET_X: f64 = 70.0;
const LOGGER_POCKET_Y: f64 = 58.0;
const LOGGER_RECESS_DEPTH: f64 = 12.0;
const LOGGER_PITCH_X: f64 = 82.0;
const LOGGER_CABLE_SLOT_W: f64 = 9.0;

const GATE_CENTER: (f64, f64) = (0.0, -375.0);
const GATE_PANEL_X: f64 = 450.0;
const GATE_PANEL_Y: f64 = 150.0;
const GATE_PANEL_Z: f64 = 28.0;
const GATE_X: f64 = 124.0;
const GATE_Y: f64 = 104.0;
const GATE_PITCH_X: f64 = 142.0;
const GATE_RECESS_DEPTH: f64 = 10.0;
const GATE_CARD_LAND_X: f64 = 74.0;
const GATE_CARD_LAND_Y: f64 = 18.0;

const CUSTODY_CENTER: (f64, f64) = (-505.0, -405.0);
const CUSTODY_PANEL_X: f64 = 340.0;
const CUSTODY_PANEL_Y: f64 = 116.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const CUSTODY_LAND_X: f64 = 34.0;
const CUSTODY_LAND_Y: f64 = 13.0;
const CUSTODY_LAND_Z: f64 = 3.0;
const CUSTODY_SEAL_COUNT: usize = 6;

const DUMMY_CENTER: (f64, f64) = (500.0, -375.0);
const DUMMY_PANEL_X: f64 = 340.0;
const DUMMY_PANEL_Y: f64 = 150.0;
const DUMMY_PANEL_Z: f64 = 24.0;
const DUMMY_LOAD_COUNT: usize = SLOT_COUNT;
const DUMMY_LOAD_D: f64 = 18.0;
const DUMMY_LOAD_RIM_D: f64 = 28.0;
const DUMMY_PITCH_X: f64 = 42.0;
const DUMMY_PITCH_Y: f64 = 34.0;
const DUMMY_RECESS_DEPTH: f64 = 8.0;

const KEEP_OUT_Z: f64 = 6.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 120.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 96.0;
const LEFT_TRANSFER_KEEP_OUT_X: f64 = 92.0;
const RIGHT_LOGGER_SERVICE_KEEP_OUT_X: f64 = 110.0;
const VERTICAL_PICK_CLEARANCE_Z: f64 = 150.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SlotClass {
    Corner,
    Edge,
    Center,
}

impl SlotClass {
    fn label(self) -> &'static str {
        match self {
            SlotClass::Corner => "corner",
            SlotClass::Edge => "edge",
            SlotClass::Center => "center",
        }
    }

    fn index(self) -> usize {
        match self {
            SlotClass::Corner => 0,
            SlotClass::Edge => 1,
            SlotClass::Center => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EnvChannel {
    Temperature,
    Co2,
    Humidity,
}

impl EnvChannel {
    fn all() -> [EnvChannel; ENV_CHANNEL_COUNT] {
        [
            EnvChannel::Temperature,
            EnvChannel::Co2,
            EnvChannel::Humidity,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            EnvChannel::Temperature => "temperature",
            EnvChannel::Co2 => "co2",
            EnvChannel::Humidity => "humidity",
        }
    }

    fn index(self) -> usize {
        match self {
            EnvChannel::Temperature => 0,
            EnvChannel::Co2 => 1,
            EnvChannel::Humidity => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExcursionMarker {
    Temperature,
    Co2,
    Humidity,
    Duration,
}

impl ExcursionMarker {
    fn all() -> [ExcursionMarker; EXCURSION_MARKER_COUNT] {
        [
            ExcursionMarker::Temperature,
            ExcursionMarker::Co2,
            ExcursionMarker::Humidity,
            ExcursionMarker::Duration,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            ExcursionMarker::Temperature => "temperature_excursion",
            ExcursionMarker::Co2 => "co2_excursion",
            ExcursionMarker::Humidity => "humidity_excursion",
            ExcursionMarker::Duration => "duration_window",
        }
    }

    fn index(self) -> usize {
        match self {
            ExcursionMarker::Temperature => 0,
            ExcursionMarker::Co2 => 1,
            ExcursionMarker::Humidity => 2,
            ExcursionMarker::Duration => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LoggerDock {
    Temperature,
    Co2,
    Humidity,
    Reference,
}

impl LoggerDock {
    fn all() -> [LoggerDock; LOGGER_DOCK_COUNT] {
        [
            LoggerDock::Temperature,
            LoggerDock::Co2,
            LoggerDock::Humidity,
            LoggerDock::Reference,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            LoggerDock::Temperature => "temperature",
            LoggerDock::Co2 => "co2",
            LoggerDock::Humidity => "humidity",
            LoggerDock::Reference => "reference",
        }
    }

    fn index(self) -> usize {
        match self {
            LoggerDock::Temperature => 0,
            LoggerDock::Co2 => 1,
            LoggerDock::Humidity => 2,
            LoggerDock::Reference => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProcessLane {
    Quarantine,
    Recovery,
    MatchedControl,
}

impl ProcessLane {
    fn all() -> [ProcessLane; PROCESS_LANE_COUNT] {
        [
            ProcessLane::Quarantine,
            ProcessLane::Recovery,
            ProcessLane::MatchedControl,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            ProcessLane::Quarantine => "quarantine",
            ProcessLane::Recovery => "recovery",
            ProcessLane::MatchedControl => "matched_control",
        }
    }

    fn index(self) -> usize {
        match self {
            ProcessLane::Quarantine => 0,
            ProcessLane::Recovery => 1,
            ProcessLane::MatchedControl => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceGate {
    Release,
    Hold,
    Reject,
}

impl EvidenceGate {
    fn all() -> [EvidenceGate; EVIDENCE_GATE_COUNT] {
        [
            EvidenceGate::Release,
            EvidenceGate::Hold,
            EvidenceGate::Reject,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            EvidenceGate::Release => "release",
            EvidenceGate::Hold => "hold",
            EvidenceGate::Reject => "reject",
        }
    }

    fn index(self) -> usize {
        match self {
            EvidenceGate::Release => 0,
            EvidenceGate::Hold => 1,
            EvidenceGate::Reject => 2,
        }
    }
}

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    export(OUTPUTS[0], &base_deck());
    export(OUTPUTS[1], &slot_map_board());
    export(OUTPUTS[2], &excursion_marker_tokens());
    export(OUTPUTS[3], &balanced_reassignment_grid());
    export(OUTPUTS[4], &quarantine_recovery_lanes());
    export(OUTPUTS[5], &environmental_equivalence_witness_coupons());
    export(OUTPUTS[6], &temp_co2_rh_logger_evidence_docks());
    export(OUTPUTS[7], &release_hold_reject_evidence_gates());
    export(OUTPUTS[8], &barcode_custody_transfer_lands());
    export(OUTPUTS[9], &bias_balance_dummy_load_equalizers());
    export(OUTPUTS[10], &robot_service_keepouts());
    export(OUTPUTS[11], &station_assembly());

    println!();
    println!("Closed incubator slot reassignment after excursion balance station:");
    println!("  Slot map:                 {SLOT_COLS}x{SLOT_ROWS} incubator board with {} local excursion slots", EXCURSION_SLOTS.len());
    println!(
        "  Excursion evidence:       {EXCURSION_MARKER_COUNT} tokens for temperature, CO2, humidity, and duration"
    );
    println!(
        "  Reassignment balance:     {SLOT_COUNT} source slots, {SLOT_COUNT} unique targets, class counts corner/edge/center = {CORNER_SLOT_COUNT}/{EDGE_SLOT_COUNT}/{CENTER_SLOT_COUNT}"
    );
    println!(
        "  Recovery targets:         {:?} with matched source-class distribution {:?}",
        RECOVERY_TARGET_SLOTS,
        excursion_source_class_counts()
    );
    println!(
        "  Witness coupons:          {ENV_WITNESS_COUPON_COUNT} paired environmental coupons across {ENV_CHANNEL_COUNT} channels"
    );
    println!(
        "  Process lanes/gates:      {PROCESS_LANE_COUNT} quarantine/recovery/control lanes and {EVIDENCE_GATE_COUNT} release/hold/reject gates"
    );
    println!("  STL outputs:              {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_deck()
        + slot_map_board()
        + excursion_marker_tokens()
        + balanced_reassignment_grid()
        + quarantine_recovery_lanes()
        + environmental_equivalence_witness_coupons()
        + temp_co2_rh_logger_evidence_docks()
        + release_hold_reject_evidence_gates()
        + barcode_custody_transfer_lands()
        + bias_balance_dummy_load_equalizers()
        + robot_service_keepouts()
}

fn assert_layout() {
    assert_eq!(SLOT_COUNT, 16);
    assert_eq!(EnvChannel::all().len(), ENV_CHANNEL_COUNT);
    assert_eq!(ExcursionMarker::all().len(), EXCURSION_MARKER_COUNT);
    assert_eq!(LoggerDock::all().len(), LOGGER_DOCK_COUNT);
    assert_eq!(ProcessLane::all().len(), PROCESS_LANE_COUNT);
    assert_eq!(EvidenceGate::all().len(), EVIDENCE_GATE_COUNT);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|output| output.contains(OUTPUT_PREFIX)));
    assert_eq!(
        natural_slot_class_counts(),
        [CORNER_SLOT_COUNT, EDGE_SLOT_COUNT, CENTER_SLOT_COUNT]
    );
    assert_eq!(
        assignment_target_class_counts(),
        natural_slot_class_counts()
    );
    assert!(assignment_is_permutation());
    assert!(assignment_moves_every_slot());
    assert_eq!(excursion_target_slots(), RECOVERY_TARGET_SLOTS);
    assert_eq!(
        excursion_source_class_counts(),
        excursion_target_class_counts()
    );
    assert_eq!(ENV_WITNESS_COUPON_COUNT, 24);
    assert!(GRID_SLOT_Z > REVC_TOTAL_HEIGHT);
    assert!(GRID_Z > REVC_TOTAL_HEIGHT + 20.0);
    assert!(vertical_pick_clearance_above_cassette() > 95.0);
    assert!(footprints_fit_on_deck());
    assert!(!critical_footprints_overlap());
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "slot_reassignment_after_excursion_base_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    deck - deck_mount_holes() - module_registration_recesses()
        + deck_perimeter_rims()
        + deck_datum_targets()
        + deck_flow_arrows()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("slot_reassignment_deck_mount_holes");
    for (index, (x, y)) in [
        (-DECK_X / 2.0 + 45.0, -DECK_Y / 2.0 + 45.0),
        (DECK_X / 2.0 - 45.0, -DECK_Y / 2.0 + 45.0),
        (-DECK_X / 2.0 + 45.0, DECK_Y / 2.0 - 45.0),
        (DECK_X / 2.0 - 45.0, DECK_Y / 2.0 - 45.0),
        (0.0, -DECK_Y / 2.0 + 45.0),
        (0.0, DECK_Y / 2.0 - 45.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("slot_reassignment_deck_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 2.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn module_registration_recesses() -> Part {
    let mut recesses = Part::empty("slot_reassignment_module_registration_recesses");
    for footprint in layout_footprints() {
        recesses = recesses
            + centered_cube(
                format!(
                    "slot_reassignment_{}_module_registration_recess",
                    footprint.name
                ),
                footprint.x + 14.0,
                footprint.y + 14.0,
                MODULE_RECESS_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                DECK_Z - MODULE_RECESS_DEPTH / 2.0 + 0.2,
            );
    }
    recesses
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        "slot_reassignment_front_deck_clean_wipe_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "slot_reassignment_rear_deck_clean_wipe_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - DECK_RIM_W / 2.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "slot_reassignment_left_deck_clean_wipe_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "slot_reassignment_right_deck_clean_wipe_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z + DECK_RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn deck_datum_targets() -> Part {
    let mut datums = Part::empty("slot_reassignment_deck_datum_targets");
    for (index, footprint) in layout_footprints().into_iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("slot_reassignment_{}_datum_target_{index}", footprint.name),
                6.0,
                4.0,
                28,
            )
            .translate(
                footprint.center.0 - footprint.x / 2.0 + 18.0,
                footprint.center.1 + footprint.y / 2.0 - 18.0,
                DECK_Z + 2.0,
            );
    }
    datums
}

fn deck_flow_arrows() -> Part {
    let mut arrows = Part::empty("slot_reassignment_process_flow_arrows");
    for (index, (x, y, width)) in [
        (-300.0, 332.0, 88.0),
        (-205.0, -348.0, 98.0),
        (260.0, -342.0, 96.0),
    ]
    .into_iter()
    .enumerate()
    {
        let shaft = centered_cube(
            format!("slot_reassignment_flow_arrow_{index}_shaft"),
            width,
            7.0,
            6.0,
        )
        .translate(x, y, DECK_Z + 3.0);
        let head = centered_cube(
            format!("slot_reassignment_flow_arrow_{index}_head"),
            18.0,
            22.0,
            6.0,
        )
        .translate(x + width / 2.0 + 8.0, y, DECK_Z + 3.0);
        arrows = arrows + shaft + head;
    }
    arrows
}

fn slot_map_board() -> Part {
    let panel = centered_cube(
        "slot_reassignment_slot_map_board_panel",
        SLOT_MAP_X,
        SLOT_MAP_Y,
        SLOT_MAP_Z,
    );
    let mut slot_lands = Part::empty("slot_reassignment_slot_map_board_lands");
    let mut recesses = Part::empty("slot_reassignment_slot_map_board_slot_recesses");

    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_map_xy(slot);
        let class = slot_class(slot);
        let target = reassignment_target(slot);
        let status = if is_excursion_slot(slot) {
            "affected"
        } else {
            "control"
        };

        recesses = recesses
            + centered_cube(
                format!(
                    "slot_reassignment_slot_map_source_{slot:02}_{status}_to_target_{target:02}_recess"
                ),
                SLOT_MAP_CELL_X,
                SLOT_MAP_CELL_Y,
                5.0,
            )
            .translate(x, y, SLOT_MAP_Z / 2.0 - 2.0);

        let class_token = centered_cylinder(
            format!(
                "slot_reassignment_slot_map_source_{slot:02}_{}_class_token",
                class.label()
            ),
            class_marker_radius(class),
            4.0,
            28,
        )
        .translate(x - SLOT_MAP_CELL_X / 2.0 + 13.0, y, SLOT_MAP_Z / 2.0 + 2.0);
        let target_land = centered_cube(
            format!("slot_reassignment_slot_map_source_{slot:02}_target_{target:02}_land"),
            25.0,
            10.0,
            4.0,
        )
        .translate(
            x + 12.0,
            y - SLOT_MAP_CELL_Y / 2.0 + 9.0,
            SLOT_MAP_Z / 2.0 + 2.0,
        );
        let affected_pin = if is_excursion_slot(slot) {
            centered_cylinder(
                format!("slot_reassignment_slot_map_source_{slot:02}_excursion_pin"),
                SLOT_MAP_TOKEN_D / 2.0,
                6.0,
                32,
            )
            .translate(x + SLOT_MAP_CELL_X / 2.0 - 12.0, y, SLOT_MAP_Z / 2.0 + 3.0)
        } else {
            centered_cube(
                format!("slot_reassignment_slot_map_source_{slot:02}_control_tick"),
                5.0,
                22.0,
                3.0,
            )
            .translate(x + SLOT_MAP_CELL_X / 2.0 - 12.0, y, SLOT_MAP_Z / 2.0 + 1.5)
        };

        slot_lands = slot_lands + class_token + target_land + affected_pin;
    }

    let excursion_frame = slot_map_excursion_zone_frame();
    (panel - recesses + slot_lands + excursion_frame).translate(
        SLOT_MAP_CENTER.0,
        SLOT_MAP_CENTER.1,
        DECK_Z + SLOT_MAP_Z / 2.0,
    )
}

fn slot_map_excursion_zone_frame() -> Part {
    let x0 = (slot_map_xy(0).0 + slot_map_xy(5).0) / 2.0;
    let y0 = (slot_map_xy(0).1 + slot_map_xy(5).1) / 2.0;
    let frame_x = SLOT_MAP_PITCH_X + SLOT_MAP_CELL_X + 16.0;
    let frame_y = SLOT_MAP_PITCH_Y + SLOT_MAP_CELL_Y + 16.0;
    let top = centered_cube(
        "slot_reassignment_slot_map_excursion_zone_top_rail",
        frame_x,
        6.0,
        7.0,
    )
    .translate(x0, y0 + frame_y / 2.0, SLOT_MAP_Z / 2.0 + 3.5);
    let bottom = centered_cube(
        "slot_reassignment_slot_map_excursion_zone_bottom_rail",
        frame_x,
        6.0,
        7.0,
    )
    .translate(x0, y0 - frame_y / 2.0, SLOT_MAP_Z / 2.0 + 3.5);
    let left = centered_cube(
        "slot_reassignment_slot_map_excursion_zone_left_rail",
        6.0,
        frame_y,
        7.0,
    )
    .translate(x0 - frame_x / 2.0, y0, SLOT_MAP_Z / 2.0 + 3.5);
    let right = centered_cube(
        "slot_reassignment_slot_map_excursion_zone_right_rail",
        6.0,
        frame_y,
        7.0,
    )
    .translate(x0 + frame_x / 2.0, y0, SLOT_MAP_Z / 2.0 + 3.5);
    top + bottom + left + right
}

fn excursion_marker_tokens() -> Part {
    let panel = centered_cube(
        "slot_reassignment_temperature_co2_humidity_excursion_marker_token_panel",
        MARKER_PANEL_X,
        MARKER_PANEL_Y,
        MARKER_PANEL_Z,
    );
    let mut reliefs = Part::empty("slot_reassignment_excursion_marker_token_reliefs");
    let mut tokens = Part::empty("slot_reassignment_excursion_marker_tokens");

    for marker in ExcursionMarker::all() {
        let (x, y) = marker_token_xy(marker);
        reliefs = reliefs
            + centered_cylinder(
                format!("slot_reassignment_{}_marker_token_recess", marker.label()),
                MARKER_TOKEN_D / 2.0 + 3.0,
                8.0,
                36,
            )
            .translate(x, y, MARKER_PANEL_Z / 2.0 - 3.0);
        let token = centered_cylinder(
            format!("slot_reassignment_{}_marker_token", marker.label()),
            MARKER_TOKEN_D / 2.0,
            MARKER_TOKEN_Z,
            36,
        )
        .translate(x, y, MARKER_PANEL_Z / 2.0 + MARKER_TOKEN_Z / 2.0);
        let notch = centered_cube(
            format!(
                "slot_reassignment_{}_marker_orientation_notch",
                marker.label()
            ),
            5.0,
            MARKER_TOKEN_D,
            3.0,
        )
        .translate(
            x + MARKER_TOKEN_D / 4.0,
            y,
            MARKER_PANEL_Z / 2.0 + MARKER_TOKEN_Z + 1.5,
        );
        tokens = tokens + token + notch;
    }

    for (index, slot) in EXCURSION_SLOTS.into_iter().enumerate() {
        let x = centered_index(index, EXCURSION_SLOTS.len(), 42.0);
        let y = -MARKER_PANEL_Y / 2.0 + 34.0;
        tokens = tokens
            + centered_cylinder(
                format!("slot_reassignment_affected_slot_{slot:02}_local_excursion_flag"),
                AFFECTED_SLOT_TOKEN_D / 2.0,
                5.0,
                28,
            )
            .translate(x, y, MARKER_PANEL_Z / 2.0 + 2.5)
            + centered_cube(
                format!("slot_reassignment_affected_slot_{slot:02}_timebox_bar"),
                26.0,
                6.0,
                4.0,
            )
            .translate(x, y - 20.0, MARKER_PANEL_Z / 2.0 + 2.0);
    }

    (panel - reliefs + tokens).translate(
        MARKER_CENTER.0,
        MARKER_CENTER.1,
        DECK_Z + MARKER_PANEL_Z / 2.0,
    )
}

fn balanced_reassignment_grid() -> Part {
    let panel = centered_cube(
        "slot_reassignment_balanced_reassignment_grid_panel",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let mut reliefs = Part::empty("slot_reassignment_balanced_grid_slot_reliefs");
    let mut features = Part::empty("slot_reassignment_balanced_grid_features");

    for source in 0..SLOT_COUNT {
        let target = reassignment_target(source);
        let (x, y) = reassignment_grid_xy(target);
        let class = slot_class(target);
        reliefs = reliefs
            + centered_cube(
                format!(
                    "slot_reassignment_balanced_grid_target_{target:02}_from_source_{source:02}_relief"
                ),
                GRID_SLOT_X,
                GRID_SLOT_Y,
                GRID_RECESS_DEPTH,
            )
            .translate(x, y, GRID_Z / 2.0 - GRID_RECESS_DEPTH / 2.0 + 0.6);

        let assignment_token = centered_cylinder(
            format!(
                "slot_reassignment_source_{source:02}_to_target_{target:02}_{}_assignment_token",
                class.label()
            ),
            ASSIGNMENT_TOKEN_D / 2.0,
            5.0,
            28,
        )
        .translate(
            x - GRID_SLOT_X / 2.0 + 20.0,
            y + GRID_SLOT_Y / 2.0 - 17.0,
            GRID_Z / 2.0 + 2.5,
        );
        let barcode_land = centered_cube(
            format!("slot_reassignment_source_{source:02}_to_target_{target:02}_barcode_land"),
            44.0,
            12.0,
            4.0,
        )
        .translate(x, y - GRID_SLOT_Y / 2.0 - 10.0, GRID_Z / 2.0 + 2.0);
        features = features + assignment_token + barcode_land;

        if is_recovery_target_slot(target) {
            features = features
                + centered_cylinder(
                    format!("slot_reassignment_target_{target:02}_recovery_halo"),
                    RECOVERY_HALO_D / 2.0,
                    4.0,
                    36,
                )
                .translate(
                    x + GRID_SLOT_X / 2.0 - 21.0,
                    y + GRID_SLOT_Y / 2.0 - 19.0,
                    GRID_Z / 2.0 + 2.0,
                );
        }
    }

    let rails = reassignment_grid_datum_rails();
    let class_key = reassignment_class_key();
    (panel - reliefs + features + rails + class_key).translate(
        GRID_CENTER.0,
        GRID_CENTER.1,
        DECK_Z + GRID_Z / 2.0,
    )
}

fn reassignment_grid_datum_rails() -> Part {
    let left = centered_cube(
        "slot_reassignment_balanced_grid_left_datum_rail",
        GRID_DOCK_RAIL_W,
        GRID_Y,
        GRID_DOCK_RAIL_Z,
    )
    .translate(
        -GRID_X / 2.0 + GRID_DOCK_RAIL_W / 2.0,
        0.0,
        GRID_Z / 2.0 + GRID_DOCK_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "slot_reassignment_balanced_grid_rear_datum_rail",
        GRID_X,
        GRID_DOCK_RAIL_W,
        GRID_DOCK_RAIL_Z,
    )
    .translate(
        0.0,
        GRID_Y / 2.0 - GRID_DOCK_RAIL_W / 2.0,
        GRID_Z / 2.0 + GRID_DOCK_RAIL_Z / 2.0,
    );
    let right_soft = centered_cube(
        "slot_reassignment_balanced_grid_right_soft_capture_rail",
        GRID_DOCK_RAIL_W,
        GRID_Y * 0.68,
        GRID_DOCK_RAIL_Z * 0.64,
    )
    .translate(
        GRID_X / 2.0 - GRID_DOCK_RAIL_W / 2.0,
        -28.0,
        GRID_Z / 2.0 + GRID_DOCK_RAIL_Z * 0.32,
    );
    left + rear + right_soft
}

fn reassignment_class_key() -> Part {
    let mut key = Part::empty("slot_reassignment_balanced_grid_corner_edge_center_key");
    for class in [SlotClass::Corner, SlotClass::Edge, SlotClass::Center] {
        let x = centered_index(class.index(), SLOT_CLASS_COUNT, 38.0);
        key = key
            + centered_cylinder(
                format!(
                    "slot_reassignment_balanced_grid_{}_class_key",
                    class.label()
                ),
                class_marker_radius(class),
                5.0,
                32,
            )
            .translate(x, -GRID_Y / 2.0 + 24.0, GRID_Z / 2.0 + 2.5);
    }
    key
}

fn quarantine_recovery_lanes() -> Part {
    let panel = centered_cube(
        "slot_reassignment_quarantine_recovery_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    );
    let mut reliefs = Part::empty("slot_reassignment_quarantine_recovery_lane_reliefs");
    let mut lane_features = Part::empty("slot_reassignment_quarantine_recovery_lane_features");

    for lane in ProcessLane::all() {
        let x = process_lane_x(lane);
        reliefs = reliefs
            + centered_cube(
                format!("slot_reassignment_{}_lane_token_recess", lane.label()),
                PROCESS_LANE_X,
                PROCESS_LANE_Y,
                PROCESS_LANE_RECESS_DEPTH,
            )
            .translate(
                x,
                0.0,
                LANE_PANEL_Z / 2.0 - PROCESS_LANE_RECESS_DEPTH / 2.0 + 0.5,
            );
        let wall = centered_cube(
            format!("slot_reassignment_{}_lane_front_status_wall", lane.label()),
            PROCESS_LANE_X,
            PROCESS_LANE_WALL_W,
            18.0,
        )
        .translate(x, -PROCESS_LANE_Y / 2.0, LANE_PANEL_Z / 2.0 + 9.0);
        let capacity = lane_capacity_posts(lane);
        lane_features = lane_features + wall + capacity;
    }

    let cross_lane_balance_bar = centered_cube(
        "slot_reassignment_quarantine_recovery_cross_lane_balance_bar",
        LANE_PANEL_X - 42.0,
        7.0,
        8.0,
    )
    .translate(0.0, LANE_PANEL_Y / 2.0 - 34.0, LANE_PANEL_Z / 2.0 + 4.0);

    (panel - reliefs + lane_features + cross_lane_balance_bar).translate(
        LANE_CENTER.0,
        LANE_CENTER.1,
        DECK_Z + LANE_PANEL_Z / 2.0,
    )
}

fn lane_capacity_posts(lane: ProcessLane) -> Part {
    let mut posts = Part::empty(format!(
        "slot_reassignment_{}_lane_capacity_posts",
        lane.label()
    ));
    let x = process_lane_x(lane);
    for index in 0..PROCESS_LANE_CAPACITY {
        let y = centered_index(index, PROCESS_LANE_CAPACITY, 38.0);
        posts = posts
            + centered_cylinder(
                format!(
                    "slot_reassignment_{}_lane_slot_{index}_capacity_post",
                    lane.label()
                ),
                3.0,
                7.0,
                20,
            )
            .translate(x + PROCESS_LANE_X / 2.0 - 13.0, y, LANE_PANEL_Z / 2.0 + 3.5)
            + centered_cube(
                format!(
                    "slot_reassignment_{}_lane_slot_{index}_evidence_land",
                    lane.label()
                ),
                42.0,
                12.0,
                4.0,
            )
            .translate(x - 10.0, y, LANE_PANEL_Z / 2.0 + 2.0);
    }
    posts
}

fn environmental_equivalence_witness_coupons() -> Part {
    let panel = centered_cube(
        "slot_reassignment_environmental_equivalence_witness_coupon_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );
    let mut coupons = Part::empty("slot_reassignment_environmental_equivalence_witness_coupons");

    for channel in EnvChannel::all() {
        for pair in 0..ENV_WITNESS_PAIR_COUNT {
            let source_slot = EXCURSION_SLOTS[pair];
            let target_slot = RECOVERY_TARGET_SLOTS[pair];
            let (x, y) = witness_pair_xy(channel, pair);
            let affected = witness_coupon(
                format!(
                    "slot_reassignment_{}_affected_source_{source_slot:02}_witness_coupon",
                    channel.label()
                ),
                source_slot,
            )
            .translate(
                x,
                y + WITNESS_PAIR_OFFSET_Y,
                WITNESS_PANEL_Z / 2.0 + WITNESS_COUPON_Z / 2.0,
            );
            let matched = witness_coupon(
                format!(
                    "slot_reassignment_{}_matched_target_{target_slot:02}_witness_coupon",
                    channel.label()
                ),
                target_slot,
            )
            .translate(
                x,
                y - WITNESS_PAIR_OFFSET_Y,
                WITNESS_PANEL_Z / 2.0 + WITNESS_COUPON_Z / 2.0,
            );
            let pair_bridge = centered_cube(
                format!(
                    "slot_reassignment_{}_source_{source_slot:02}_target_{target_slot:02}_equivalence_bridge",
                    channel.label()
                ),
                5.0,
                WITNESS_PAIR_OFFSET_Y * 2.0,
                4.0,
            )
            .translate(x, y, WITNESS_PANEL_Z / 2.0 + 2.0);
            coupons = coupons + affected + matched + pair_bridge;
        }
    }

    let channel_dividers = witness_channel_dividers();
    (panel + coupons + channel_dividers).translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        DECK_Z + WITNESS_PANEL_Z / 2.0,
    )
}

fn witness_coupon(name: String, slot: usize) -> Part {
    let class = slot_class(slot);
    let coupon = centered_cylinder(name.clone(), WITNESS_COUPON_D / 2.0, WITNESS_COUPON_Z, 36);
    let class_key = centered_cube(
        format!("{name}_{}_class_key", class.label()),
        class_marker_radius(class) * 1.2,
        4.0,
        2.0,
    )
    .translate(
        0.0,
        WITNESS_COUPON_D / 2.0 - 3.0,
        WITNESS_COUPON_Z / 2.0 + 1.0,
    );
    coupon + class_key
}

fn witness_channel_dividers() -> Part {
    let mut dividers = Part::empty("slot_reassignment_witness_channel_dividers");
    for index in 0..ENV_CHANNEL_COUNT - 1 {
        let y = centered_index(index, ENV_CHANNEL_COUNT - 1, WITNESS_CHANNEL_PITCH_Y)
            + WITNESS_CHANNEL_PITCH_Y / 2.0
            - WITNESS_CHANNEL_PITCH_Y / 4.0;
        dividers = dividers
            + centered_cube(
                format!("slot_reassignment_witness_channel_divider_{index}"),
                WITNESS_PANEL_X - 46.0,
                5.0,
                8.0,
            )
            .translate(0.0, y, WITNESS_PANEL_Z / 2.0 + 4.0);
    }
    dividers
}

fn temp_co2_rh_logger_evidence_docks() -> Part {
    let panel = centered_cube(
        "slot_reassignment_temp_co2_rh_logger_evidence_dock_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let mut docks = Part::empty("slot_reassignment_temp_co2_rh_logger_evidence_docks");

    for dock in LoggerDock::all() {
        docks = docks + logger_dock(dock).translate(logger_dock_x(dock), 0.0, 0.0);
    }

    let cable_comb = logger_cable_comb();
    let evidence_seal_bar = centered_cube(
        "slot_reassignment_logger_evidence_seal_bar",
        LOGGER_PANEL_X - 36.0,
        8.0,
        8.0,
    )
    .translate(0.0, LOGGER_PANEL_Y / 2.0 - 26.0, LOGGER_PANEL_Z / 2.0 + 4.0);

    (panel + docks + cable_comb + evidence_seal_bar).translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1,
        DECK_Z + LOGGER_PANEL_Z / 2.0,
    )
}

fn logger_dock(dock: LoggerDock) -> Part {
    let label = dock.label();
    let body = centered_cube(
        format!("slot_reassignment_{label}_logger_dock_body"),
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
        LOGGER_PANEL_Z,
    );
    let recess = centered_cube(
        format!("slot_reassignment_{label}_logger_recess"),
        LOGGER_POCKET_X - 14.0,
        LOGGER_POCKET_Y - 14.0,
        LOGGER_RECESS_DEPTH,
    )
    .translate(
        0.0,
        0.0,
        LOGGER_PANEL_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0 + 0.5,
    );
    let cable_exit = centered_cube(
        format!("slot_reassignment_{label}_logger_cable_exit"),
        LOGGER_CABLE_SLOT_W,
        LOGGER_POCKET_Y + 3.0,
        LOGGER_RECESS_DEPTH + 2.0,
    )
    .translate(
        LOGGER_POCKET_X / 2.0 - 12.0,
        0.0,
        LOGGER_PANEL_Z / 2.0 - 2.0,
    );
    let environmental_access = match dock {
        LoggerDock::Temperature => logger_probe_cradle(label),
        LoggerDock::Co2 => logger_diffusion_windows(label, 3),
        LoggerDock::Humidity => logger_louver_windows(label, 4),
        LoggerDock::Reference => logger_reference_key(label),
    };

    body - recess - cable_exit - environmental_access
}

fn logger_probe_cradle(label: &str) -> Part {
    centered_cube(
        format!("slot_reassignment_{label}_logger_probe_cradle_window"),
        44.0,
        7.0,
        LOGGER_RECESS_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0)
}

fn logger_diffusion_windows(label: &str, count: usize) -> Part {
    let mut windows = Part::empty(format!(
        "slot_reassignment_{label}_logger_diffusion_windows"
    ));
    for index in 0..count {
        windows = windows
            + centered_cylinder(
                format!("slot_reassignment_{label}_logger_diffusion_window_{index}"),
                4.0,
                LOGGER_RECESS_DEPTH + 2.0,
                24,
            )
            .translate(
                centered_index(index, count, 16.0),
                -10.0,
                LOGGER_PANEL_Z / 2.0,
            );
    }
    windows
}

fn logger_louver_windows(label: &str, count: usize) -> Part {
    let mut louvers = Part::empty(format!("slot_reassignment_{label}_logger_louver_windows"));
    for index in 0..count {
        louvers = louvers
            + centered_cube(
                format!("slot_reassignment_{label}_logger_louver_window_{index}"),
                42.0,
                2.5,
                LOGGER_RECESS_DEPTH + 2.0,
            )
            .translate(0.0, centered_index(index, count, 8.0), LOGGER_PANEL_Z / 2.0);
    }
    louvers
}

fn logger_reference_key(label: &str) -> Part {
    centered_cube(
        format!("slot_reassignment_{label}_logger_reference_key_window"),
        12.0,
        36.0,
        LOGGER_RECESS_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0)
}

fn logger_cable_comb() -> Part {
    let mut comb = Part::empty("slot_reassignment_logger_cable_comb");
    for index in 0..LOGGER_DOCK_COUNT {
        let x = centered_index(index, LOGGER_DOCK_COUNT, LOGGER_PITCH_X);
        let clamp = centered_cube(
            format!("slot_reassignment_logger_cable_clamp_{index}"),
            30.0,
            12.0,
            10.0,
        )
        .translate(x, -LOGGER_PANEL_Y / 2.0 + 24.0, LOGGER_PANEL_Z / 2.0 + 5.0);
        let channel = centered_cube(
            format!("slot_reassignment_logger_cable_channel_{index}"),
            9.0,
            16.0,
            12.0,
        )
        .translate(x, -LOGGER_PANEL_Y / 2.0 + 24.0, LOGGER_PANEL_Z / 2.0 + 5.0);
        comb = comb + (clamp - channel);
    }
    comb
}

fn release_hold_reject_evidence_gates() -> Part {
    let panel = centered_cube(
        "slot_reassignment_release_hold_reject_evidence_gate_panel",
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );
    let mut reliefs = Part::empty("slot_reassignment_release_hold_reject_gate_reliefs");
    let mut gates = Part::empty("slot_reassignment_release_hold_reject_evidence_gates");

    for gate in EvidenceGate::all() {
        let x = evidence_gate_x(gate);
        reliefs = reliefs
            + centered_cube(
                format!("slot_reassignment_{}_evidence_gate_recess", gate.label()),
                GATE_X,
                GATE_Y,
                GATE_RECESS_DEPTH,
            )
            .translate(x, 0.0, GATE_PANEL_Z / 2.0 - GATE_RECESS_DEPTH / 2.0 + 0.5);
        let card_land = centered_cube(
            format!("slot_reassignment_{}_evidence_gate_card_land", gate.label()),
            GATE_CARD_LAND_X,
            GATE_CARD_LAND_Y,
            4.0,
        )
        .translate(x, -GATE_Y / 2.0 + 18.0, GATE_PANEL_Z / 2.0 + 2.0);
        let interlock_pin = centered_cylinder(
            format!(
                "slot_reassignment_{}_evidence_gate_interlock_pin",
                gate.label()
            ),
            6.0,
            8.0,
            28,
        )
        .translate(
            x + GATE_X / 2.0 - 18.0,
            GATE_Y / 2.0 - 18.0,
            GATE_PANEL_Z / 2.0 + 4.0,
        );
        let witness_posts = gate_witness_posts(gate);
        gates = gates + card_land + interlock_pin + witness_posts;
    }

    let gate_sequencing_bar = centered_cube(
        "slot_reassignment_release_hold_reject_gate_sequencing_bar",
        GATE_PANEL_X - 48.0,
        6.0,
        8.0,
    )
    .translate(0.0, GATE_PANEL_Y / 2.0 - 24.0, GATE_PANEL_Z / 2.0 + 4.0);

    (panel - reliefs + gates + gate_sequencing_bar).translate(
        GATE_CENTER.0,
        GATE_CENTER.1,
        DECK_Z + GATE_PANEL_Z / 2.0,
    )
}

fn gate_witness_posts(gate: EvidenceGate) -> Part {
    let mut posts = Part::empty(format!(
        "slot_reassignment_{}_evidence_gate_witness_posts",
        gate.label()
    ));
    let x = evidence_gate_x(gate);
    for index in 0..EXCURSION_SLOTS.len() {
        posts = posts
            + centered_cylinder(
                format!(
                    "slot_reassignment_{}_gate_excursion_pair_{index}_witness_post",
                    gate.label()
                ),
                2.8,
                5.0,
                20,
            )
            .translate(
                x - GATE_X / 2.0 + 18.0 + index as f64 * 14.0,
                GATE_Y / 2.0 - 18.0,
                GATE_PANEL_Z / 2.0 + 2.5,
            );
    }
    posts
}

fn barcode_custody_transfer_lands() -> Part {
    let panel = centered_cube(
        "slot_reassignment_barcode_custody_transfer_land_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    let mut lands = Part::empty("slot_reassignment_barcode_custody_transfer_lands");

    for slot in 0..SLOT_COUNT {
        let (x, y) = custody_land_xy(slot);
        let target = reassignment_target(slot);
        lands = lands
            + centered_cube(
                format!(
                    "slot_reassignment_source_{slot:02}_target_{target:02}_custody_barcode_land"
                ),
                CUSTODY_LAND_X,
                CUSTODY_LAND_Y,
                CUSTODY_LAND_Z,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + CUSTODY_LAND_Z / 2.0);
    }

    for index in 0..CUSTODY_SEAL_COUNT {
        let x = centered_index(index, CUSTODY_SEAL_COUNT, 42.0);
        lands = lands
            + centered_cylinder(
                format!("slot_reassignment_custody_evidence_seal_boss_{index}"),
                6.0,
                5.0,
                28,
            )
            .translate(x, CUSTODY_PANEL_Y / 2.0 - 24.0, CUSTODY_PANEL_Z / 2.0 + 2.5);
    }

    let transfer_datum = centered_cube(
        "slot_reassignment_custody_transfer_datum_bar",
        CUSTODY_PANEL_X - 42.0,
        6.0,
        7.0,
    )
    .translate(
        0.0,
        -CUSTODY_PANEL_Y / 2.0 + 22.0,
        CUSTODY_PANEL_Z / 2.0 + 3.5,
    );

    (panel + lands + transfer_datum).translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        DECK_Z + CUSTODY_PANEL_Z / 2.0,
    )
}

fn bias_balance_dummy_load_equalizers() -> Part {
    let panel = centered_cube(
        "slot_reassignment_bias_balance_dummy_load_equalizer_panel",
        DUMMY_PANEL_X,
        DUMMY_PANEL_Y,
        DUMMY_PANEL_Z,
    );
    let mut reliefs = Part::empty("slot_reassignment_dummy_load_equalizer_reliefs");
    let mut rims = Part::empty("slot_reassignment_dummy_load_equalizer_rims");

    for slot in 0..DUMMY_LOAD_COUNT {
        let (x, y) = dummy_load_xy(slot);
        let class = slot_class(slot);
        reliefs = reliefs
            + centered_cylinder(
                format!(
                    "slot_reassignment_dummy_load_slot_{slot:02}_{}_relief",
                    class.label()
                ),
                DUMMY_LOAD_D / 2.0,
                DUMMY_RECESS_DEPTH,
                28,
            )
            .translate(x, y, DUMMY_PANEL_Z / 2.0 - DUMMY_RECESS_DEPTH / 2.0 + 0.5);
        rims = rims
            + centered_cylinder(
                format!(
                    "slot_reassignment_dummy_load_slot_{slot:02}_{}_bias_balance_rim",
                    class.label()
                ),
                DUMMY_LOAD_RIM_D / 2.0,
                4.0,
                32,
            )
            .translate(x, y, DUMMY_PANEL_Z / 2.0 + 2.0);
    }

    let balance_spine = centered_cube(
        "slot_reassignment_dummy_load_corner_edge_center_balance_spine",
        DUMMY_PANEL_X - 44.0,
        7.0,
        8.0,
    )
    .translate(0.0, 0.0, DUMMY_PANEL_Z / 2.0 + 4.0);

    (panel - reliefs + rims + balance_spine).translate(
        DUMMY_CENTER.0,
        DUMMY_CENTER.1,
        DECK_Z + DUMMY_PANEL_Z / 2.0,
    )
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "slot_reassignment_front_robot_handoff_keepout_gauge",
        DECK_X - 150.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 24.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let rear_service = centered_cube(
        "slot_reassignment_rear_service_keepout_gauge",
        DECK_X - 150.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0 - 24.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let left_transfer = centered_cube(
        "slot_reassignment_left_transfer_keepout_gauge",
        LEFT_TRANSFER_KEEP_OUT_X,
        DECK_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_TRANSFER_KEEP_OUT_X / 2.0 + 24.0,
        0.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let right_service = centered_cube(
        "slot_reassignment_right_logger_service_keepout_gauge",
        RIGHT_LOGGER_SERVICE_KEEP_OUT_X,
        DECK_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_LOGGER_SERVICE_KEEP_OUT_X / 2.0 - 24.0,
        0.0,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let vertical_pick = centered_cube(
        "slot_reassignment_reassigned_cassette_vertical_pick_clearance_gauge",
        38.0,
        GRID_Y + 78.0,
        VERTICAL_PICK_CLEARANCE_Z,
    )
    .translate(
        GRID_CENTER.0 + GRID_X / 2.0 + 34.0,
        GRID_CENTER.1,
        DECK_Z + VERTICAL_PICK_CLEARANCE_Z / 2.0,
    );

    front_robot + rear_service + left_transfer + right_service + vertical_pick
}

fn layout_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "slot_map_board",
            center: SLOT_MAP_CENTER,
            x: SLOT_MAP_X,
            y: SLOT_MAP_Y,
        },
        Footprint {
            name: "marker_tokens",
            center: MARKER_CENTER,
            x: MARKER_PANEL_X,
            y: MARKER_PANEL_Y,
        },
        Footprint {
            name: "reassignment_grid",
            center: GRID_CENTER,
            x: GRID_X,
            y: GRID_Y,
        },
        Footprint {
            name: "quarantine_recovery_lanes",
            center: LANE_CENTER,
            x: LANE_PANEL_X,
            y: LANE_PANEL_Y,
        },
        Footprint {
            name: "witness_coupons",
            center: WITNESS_CENTER,
            x: WITNESS_PANEL_X,
            y: WITNESS_PANEL_Y,
        },
        Footprint {
            name: "logger_docks",
            center: LOGGER_CENTER,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Footprint {
            name: "evidence_gates",
            center: GATE_CENTER,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
        Footprint {
            name: "custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Footprint {
            name: "dummy_load_equalizers",
            center: DUMMY_CENTER,
            x: DUMMY_PANEL_X,
            y: DUMMY_PANEL_Y,
        },
    ]
}

fn slot_map_xy(slot: usize) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (
        centered_index(col, SLOT_COLS, SLOT_MAP_PITCH_X),
        -centered_index(row, SLOT_ROWS, SLOT_MAP_PITCH_Y),
    )
}

fn reassignment_grid_xy(slot: usize) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (
        centered_index(col, SLOT_COLS, GRID_SLOT_X + GRID_GAP_X),
        -centered_index(row, SLOT_ROWS, GRID_SLOT_Y + GRID_GAP_Y),
    )
}

fn marker_token_xy(marker: ExcursionMarker) -> (f64, f64) {
    (
        centered_index(marker.index(), EXCURSION_MARKER_COUNT, 88.0),
        24.0,
    )
}

fn process_lane_x(lane: ProcessLane) -> f64 {
    centered_index(lane.index(), PROCESS_LANE_COUNT, PROCESS_LANE_PITCH_X)
}

fn witness_pair_xy(channel: EnvChannel, pair: usize) -> (f64, f64) {
    (
        centered_index(pair, ENV_WITNESS_PAIR_COUNT, WITNESS_PAIR_PITCH_X),
        -centered_index(channel.index(), ENV_CHANNEL_COUNT, WITNESS_CHANNEL_PITCH_Y),
    )
}

fn logger_dock_x(dock: LoggerDock) -> f64 {
    centered_index(dock.index(), LOGGER_DOCK_COUNT, LOGGER_PITCH_X)
}

fn evidence_gate_x(gate: EvidenceGate) -> f64 {
    centered_index(gate.index(), EVIDENCE_GATE_COUNT, GATE_PITCH_X)
}

fn custody_land_xy(slot: usize) -> (f64, f64) {
    let col = slot % 8;
    let row = slot / 8;
    (
        centered_index(col, 8, 38.0),
        centered_index(row, 2, 34.0) - 10.0,
    )
}

fn dummy_load_xy(slot: usize) -> (f64, f64) {
    let col = slot % 8;
    let row = slot / 8;
    (
        centered_index(col, 8, DUMMY_PITCH_X),
        centered_index(row, 2, DUMMY_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn slot_class(slot: usize) -> SlotClass {
    let row = slot / SLOT_COLS;
    let col = slot % SLOT_COLS;
    let row_edge = row == 0 || row == SLOT_ROWS - 1;
    let col_edge = col == 0 || col == SLOT_COLS - 1;

    match (row_edge, col_edge) {
        (true, true) => SlotClass::Corner,
        (true, false) | (false, true) => SlotClass::Edge,
        (false, false) => SlotClass::Center,
    }
}

fn class_marker_radius(class: SlotClass) -> f64 {
    match class {
        SlotClass::Corner => 4.8,
        SlotClass::Edge => 6.0,
        SlotClass::Center => 7.2,
    }
}

fn reassignment_target(source_slot: usize) -> usize {
    REASSIGNMENT[source_slot]
}

fn is_excursion_slot(slot: usize) -> bool {
    EXCURSION_SLOTS.contains(&slot)
}

fn is_recovery_target_slot(slot: usize) -> bool {
    RECOVERY_TARGET_SLOTS.contains(&slot)
}

fn assignment_is_permutation() -> bool {
    let mut seen = [false; SLOT_COUNT];
    for &target in &REASSIGNMENT {
        if target >= SLOT_COUNT || seen[target] {
            return false;
        }
        seen[target] = true;
    }
    seen.into_iter().all(|hit| hit)
}

fn assignment_moves_every_slot() -> bool {
    REASSIGNMENT
        .iter()
        .enumerate()
        .all(|(source, target)| source != *target)
}

fn excursion_target_slots() -> [usize; EXCURSION_SLOTS.len()] {
    [
        reassignment_target(EXCURSION_SLOTS[0]),
        reassignment_target(EXCURSION_SLOTS[1]),
        reassignment_target(EXCURSION_SLOTS[2]),
        reassignment_target(EXCURSION_SLOTS[3]),
    ]
}

fn natural_slot_class_counts() -> [usize; SLOT_CLASS_COUNT] {
    let mut counts = [0; SLOT_CLASS_COUNT];
    for slot in 0..SLOT_COUNT {
        counts[slot_class(slot).index()] += 1;
    }
    counts
}

fn assignment_target_class_counts() -> [usize; SLOT_CLASS_COUNT] {
    let mut counts = [0; SLOT_CLASS_COUNT];
    for &slot in &REASSIGNMENT {
        counts[slot_class(slot).index()] += 1;
    }
    counts
}

fn excursion_source_class_counts() -> [usize; SLOT_CLASS_COUNT] {
    class_counts(&EXCURSION_SLOTS)
}

fn excursion_target_class_counts() -> [usize; SLOT_CLASS_COUNT] {
    class_counts(&RECOVERY_TARGET_SLOTS)
}

fn class_counts(slots: &[usize]) -> [usize; SLOT_CLASS_COUNT] {
    let mut counts = [0; SLOT_CLASS_COUNT];
    for &slot in slots {
        counts[slot_class(slot).index()] += 1;
    }
    counts
}

fn vertical_pick_clearance_above_cassette() -> f64 {
    VERTICAL_PICK_CLEARANCE_Z - (GRID_Z + REVC_TOTAL_HEIGHT)
}

fn footprints_fit_on_deck() -> bool {
    layout_footprints().into_iter().all(|footprint| {
        footprint.center.0.abs() + footprint.x / 2.0 < DECK_X / 2.0 - DECK_RIM_W
            && footprint.center.1.abs() + footprint.y / 2.0 < DECK_Y / 2.0 - DECK_RIM_W
    })
}

fn critical_footprints_overlap() -> bool {
    let footprints = layout_footprints();
    for (left_index, left) in footprints.iter().enumerate() {
        for right in footprints.iter().skip(left_index + 1) {
            if rects_overlap(footprint_rect(*left), footprint_rect(*right)) {
                return true;
            }
        }
    }
    false
}

fn footprint_rect(footprint: Footprint) -> Rect {
    Rect {
        x: footprint.center.0,
        y: footprint.center.1,
        w: footprint.x,
        h: footprint.y,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_stable_and_source_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output.starts_with("output/"));
            assert!(output.contains(OUTPUT_PREFIX));
            assert!(output.ends_with(".stl"));
        }
        assert!(OUTPUTS[0].ends_with("_base_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn feature_manifest_covers_excursion_balance_workflow() {
        for required in [
            "slot_map_board",
            "temperature_co2_humidity_excursion_marker_tokens",
            "balanced_reassignment_grid",
            "quarantine_lane",
            "recovery_lane",
            "environmental_equivalence_witness_coupons",
            "release_hold_reject_evidence_gates",
        ] {
            assert!(REQUIRED_FEATURES.contains(&required));
        }
        assert_eq!(EnvChannel::all().len(), ENV_CHANNEL_COUNT);
        assert_eq!(LoggerDock::all().len(), LOGGER_DOCK_COUNT);
        assert_eq!(ProcessLane::all().len(), PROCESS_LANE_COUNT);
        assert_eq!(EvidenceGate::all().len(), EVIDENCE_GATE_COUNT);
    }

    #[test]
    fn reassignment_is_a_balanced_unbiased_permutation() {
        assert!(assignment_is_permutation());
        assert!(assignment_moves_every_slot());
        assert_eq!(natural_slot_class_counts(), [4, 8, 4]);
        assert_eq!(
            assignment_target_class_counts(),
            natural_slot_class_counts()
        );
        assert_eq!(excursion_target_slots(), RECOVERY_TARGET_SLOTS);
        assert_eq!(excursion_source_class_counts(), [1, 2, 1]);
        assert_eq!(
            excursion_source_class_counts(),
            excursion_target_class_counts()
        );
        for slot in EXCURSION_SLOTS {
            assert!(!is_excursion_slot(reassignment_target(slot)));
            assert!(is_recovery_target_slot(reassignment_target(slot)));
        }
    }

    #[test]
    fn dimensions_provide_physical_clearance_and_fit() {
        assert!(GRID_SLOT_X > REVC_CHIP_LENGTH);
        assert!(GRID_SLOT_Y > REVC_CHIP_WIDTH);
        assert!(GRID_Z > REVC_TOTAL_HEIGHT + 20.0);
        assert!(vertical_pick_clearance_above_cassette() > 95.0);
        assert!(footprints_fit_on_deck());
        assert!(!critical_footprints_overlap());
    }

    #[test]
    fn witness_and_lane_counts_match_excursion_pairs() {
        assert_eq!(ENV_WITNESS_PAIR_COUNT, EXCURSION_SLOTS.len());
        assert_eq!(ENV_WITNESS_COUPON_COUNT, 24);
        assert_eq!(PROCESS_LANE_CAPACITY, EXCURSION_SLOTS.len());
        assert_eq!(EXCURSION_MARKER_COUNT, 4);
        assert_eq!(DUMMY_LOAD_COUNT, SLOT_COUNT);
    }
}
