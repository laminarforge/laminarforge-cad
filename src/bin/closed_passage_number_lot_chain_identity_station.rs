use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed passage-number, lot-chain, and cassette-identity validation station.
//
// The generator models a sealed benchtop validation fixture for reproducible
// tissue-on-chip runs: cassette identity capture that does not depend on the
// operator, cell lot and passage-number custody, thaw/seed timing evidence,
// QC sample split custody, and quarantine/release segregation. This is concept
// CAD only; it is not a biological release method, sterility claim, or SOP.

const OUTPUT_PREFIX: &str = "closed_passage_number_lot_chain_identity_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_passage_number_lot_chain_identity_station_base_containment_deck.stl",
    "output/closed_passage_number_lot_chain_identity_station_cassette_identity_gate.stl",
    "output/closed_passage_number_lot_chain_identity_station_lot_passage_ledger_panel.stl",
    "output/closed_passage_number_lot_chain_identity_station_thaw_seed_timing_witness_rail.stl",
    "output/closed_passage_number_lot_chain_identity_station_qc_sample_split_custody_bank.stl",
    "output/closed_passage_number_lot_chain_identity_station_quarantine_release_lane_gate.stl",
    "output/closed_passage_number_lot_chain_identity_station_closed_handoff_port_bar.stl",
    "output/closed_passage_number_lot_chain_identity_station_tamper_seal_and_operator_blind_coupon_bank.stl",
    "output/closed_passage_number_lot_chain_identity_station_environmental_evidence_bridge.stl",
    "output/closed_passage_number_lot_chain_identity_station_robot_service_keepouts.stl",
    "output/closed_passage_number_lot_chain_identity_station_machine_readable_label_tiles.stl",
    "output/closed_passage_number_lot_chain_identity_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "operator_independent_cassette_identity",
    "cell_lot_chain_of_identity",
    "passage_number_token_ladder",
    "thaw_start_witness",
    "seed_start_witness",
    "quarantine_release_lanes",
    "sterility_mycoplasma_authentication_split",
    "tamper_evident_seals",
    "environmental_logger_pocket",
    "camera_evidence_bridge",
    "closed_handoff_ports",
    "named_stl_outputs",
];

const STATION_X: f64 = 1460.0;
const STATION_Y: f64 = 940.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const BASIN_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;
const MOUNT_BOSS_D: f64 = 30.0;

const CASSETTE_COUNT: usize = 4;
const CASSETTE_ROWS: usize = 2;
const CASSETTE_COLS: usize = 2;
const CASS_GATE_CENTER: (f64, f64) = (-420.0, 155.0);
const CASS_GATE_X: f64 = 430.0;
const CASS_GATE_Y: f64 = 310.0;
const CASS_GATE_Z: f64 = 58.0;
const CASS_POCKET_X: f64 = 142.0;
const CASS_POCKET_Y: f64 = 82.0;
const CASS_POCKET_DEPTH: f64 = 30.0;
const CASS_PITCH_X: f64 = 180.0;
const CASS_PITCH_Y: f64 = 132.0;
const CASS_LOCATOR_PIN_D: f64 = 9.0;
const BARCODE_WINDOWS_PER_CASSETTE: usize = 2;
const RFID_LANDS_PER_CASSETTE: usize = 1;
const ID_CAMERA_COUNT: usize = 3;

const LEDGER_CENTER: (f64, f64) = (245.0, 190.0);
const LEDGER_X: f64 = 520.0;
const LEDGER_Y: f64 = 260.0;
const LEDGER_Z: f64 = 42.0;
const LOT_CARD_SLOTS: usize = 6;
const PASSAGE_TOKEN_COUNT: usize = 12;
const PASSAGE_TOKEN_PITCH: f64 = 34.0;
const SOURCE_VIAL_CHAIN_SLOTS: usize = 8;
const LOT_PASS_BARCODE_LANDS: usize = 10;

const TIMING_CENTER: (f64, f64) = (-405.0, -205.0);
const TIMING_X: f64 = 430.0;
const TIMING_Y: f64 = 230.0;
const TIMING_Z: f64 = 42.0;
const TIMING_EVENT_COUNT: usize = 6;
const TIMING_EVENTS: [&str; TIMING_EVENT_COUNT] = [
    "thaw_start",
    "thaw_end",
    "seed_start",
    "seed_end",
    "incubator_load",
    "first_media_exchange",
];
const TIMER_TOKEN_SLOT_X: f64 = 78.0;
const TIMER_TOKEN_SLOT_Y: f64 = 26.0;
const TIMER_TOKEN_SLOT_DEPTH: f64 = 8.0;
const MAX_THAW_TO_SEED_MINUTES: usize = 90;

const QC_CENTER: (f64, f64) = (180.0, -170.0);
const QC_X: f64 = 500.0;
const QC_Y: f64 = 250.0;
const QC_Z: f64 = 48.0;
const QC_TEST_TYPES: usize = 4;
const QC_TEST_REPLICATES: usize = 4;
const RETAIN_ALIQUOT_SLOTS: usize = 8;
const QC_TUBE_D: f64 = 16.0;
const QC_TUBE_PITCH_X: f64 = 42.0;
const QC_TUBE_PITCH_Y: f64 = 42.0;

const RELEASE_CENTER: (f64, f64) = (0.0, -380.0);
const RELEASE_X: f64 = 1240.0;
const RELEASE_Y: f64 = 86.0;
const RELEASE_Z: f64 = 36.0;
const RELEASE_LANE_NAMES: [&str; 3] = ["quarantine", "qa_hold", "released"];
const RELEASE_SLOTS_PER_LANE: usize = CASSETTE_COUNT;
const RELEASE_LANE_PITCH_X: f64 = 378.0;
const RELEASE_SLOT_PITCH_X: f64 = 74.0;
const RELEASE_KEY_WELLS_PER_LANE: usize = 2;

const HANDOFF_CENTER: (f64, f64) = (0.0, 405.0);
const HANDOFF_X: f64 = 980.0;
const HANDOFF_Y: f64 = 70.0;
const HANDOFF_Z: f64 = 70.0;
const HANDOFF_PORTS: usize = 4;
const HANDOFF_PORT_D: f64 = 24.0;
const HANDOFF_PORT_PITCH: f64 = 220.0;

const SEAL_CENTER: (f64, f64) = (612.0, -160.0);
const SEAL_X: f64 = 142.0;
const SEAL_Y: f64 = 250.0;
const SEAL_Z: f64 = 36.0;
const TAMPER_SEAL_SLOTS: usize = 12;
const OPERATOR_BLIND_COUPONS: usize = 8;

const BRIDGE_CENTER: (f64, f64) = (0.0, 80.0);
const BRIDGE_SPAN_X: f64 = 1300.0;
const BRIDGE_POST_X: f64 = 28.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_CLEARANCE_Z: f64 = 246.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const BRIDGE_TOTAL_Z: f64 = BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z;
const EVIDENCE_CAMERA_PODS: usize = 5;
const EVIDENCE_LIGHT_BARS: usize = 4;
const LOGGER_POCKETS: usize = 2;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 330.0;
const REAR_HANDOFF_KEEP_OUT_Y: f64 = 190.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 230.0;
const OVERHEAD_CAMERA_KEEP_OUT_Z: f64 = 318.0;
const KEEP_OUT_GAUGES: usize = 5;
const KEEP_OUT_Z: f64 = 6.0;

const LABEL_TILE_CENTER: (f64, f64) = (610.0, 140.0);
const LABEL_TILE_X: f64 = 150.0;
const LABEL_TILE_Y: f64 = 248.0;
const LABEL_TILE_Z: f64 = 10.0;
const LABEL_TILE_COUNT: usize = 6;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 10.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 10.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let identity = cassette_identity_gate().translate(
        CASS_GATE_CENTER.0,
        CASS_GATE_CENTER.1,
        deck_insert_z(CASS_GATE_Z),
    );
    export(OUTPUTS[1], &identity);

    let ledger = lot_passage_ledger_panel().translate(
        LEDGER_CENTER.0,
        LEDGER_CENTER.1,
        deck_insert_z(LEDGER_Z),
    );
    export(OUTPUTS[2], &ledger);

    let timing = thaw_seed_timing_witness_rail().translate(
        TIMING_CENTER.0,
        TIMING_CENTER.1,
        deck_insert_z(TIMING_Z),
    );
    export(OUTPUTS[3], &timing);

    let qc =
        qc_sample_split_custody_bank().translate(QC_CENTER.0, QC_CENTER.1, deck_insert_z(QC_Z));
    export(OUTPUTS[4], &qc);

    let release = quarantine_release_lane_gate().translate(
        RELEASE_CENTER.0,
        RELEASE_CENTER.1,
        deck_insert_z(RELEASE_Z),
    );
    export(OUTPUTS[5], &release);

    let handoff = closed_handoff_port_bar().translate(
        HANDOFF_CENTER.0,
        HANDOFF_CENTER.1,
        deck_insert_z(HANDOFF_Z),
    );
    export(OUTPUTS[6], &handoff);

    let seals = tamper_seal_and_operator_blind_coupon_bank().translate(
        SEAL_CENTER.0,
        SEAL_CENTER.1,
        deck_insert_z(SEAL_Z),
    );
    export(OUTPUTS[7], &seals);

    let bridge = environmental_evidence_bridge().translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + BRIDGE_TOTAL_Z / 2.0,
    );
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let labels = machine_readable_label_tiles().translate(
        LABEL_TILE_CENTER.0,
        LABEL_TILE_CENTER.1,
        deck_insert_z(LABEL_TILE_Z),
    );
    export(OUTPUTS[10], &labels);

    let assembly = base
        + identity
        + ledger
        + timing
        + qc
        + release
        + handoff
        + seals
        + bridge
        + keepouts
        + labels;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed passage-number/lot-chain/cassette-identity station:");
    println!(
        "  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck with {RIM_Z:.0}mm rim and {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Cassette identity:          {CASSETTE_COUNT} cassette nests, {} barcode windows, {} RFID lands, and {ID_CAMERA_COUNT} identity camera datums",
        CASSETTE_COUNT * BARCODE_WINDOWS_PER_CASSETTE,
        CASSETTE_COUNT * RFID_LANDS_PER_CASSETTE
    );
    println!(
        "  Lot/passage chain:          {LOT_CARD_SLOTS} lot card slots, {PASSAGE_TOKEN_COUNT} passage tokens, {SOURCE_VIAL_CHAIN_SLOTS} source-vial chain slots, {LOT_PASS_BARCODE_LANDS} barcode lands"
    );
    println!(
        "  Thaw/seed timing:           {TIMING_EVENT_COUNT} event token slots for {:?}, with max thaw-to-seed witness window {MAX_THAW_TO_SEED_MINUTES} minutes",
        TIMING_EVENTS
    );
    println!(
        "  QC split custody:           {QC_TEST_TYPES} test types x {QC_TEST_REPLICATES} replicates plus {RETAIN_ALIQUOT_SLOTS} retain aliquot slots"
    );
    println!(
        "  Quarantine/release:         {} lanes ({:?}) with {RELEASE_SLOTS_PER_LANE} cassette slots per lane and {RELEASE_KEY_WELLS_PER_LANE} release key wells per lane",
        RELEASE_LANE_NAMES.len(),
        RELEASE_LANE_NAMES
    );
    println!(
        "  Evidence and handoff:       {HANDOFF_PORTS} closed handoff ports, {TAMPER_SEAL_SLOTS} tamper seal slots, {OPERATOR_BLIND_COUPONS} blind coupon slots, {EVIDENCE_CAMERA_PODS} camera pods, {LOGGER_POCKETS} logger pockets"
    );
    println!(
        "  Keepouts:                   {KEEP_OUT_GAUGES} gauges; front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear handoff {REAR_HANDOFF_KEEP_OUT_Y:.0}mm, side service {SIDE_SERVICE_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_CAMERA_KEEP_OUT_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(CASSETTE_COUNT, CASSETTE_ROWS * CASSETTE_COLS);
    assert_eq!(OUTPUTS.len(), 12);
    assert!(PASSAGE_TOKEN_COUNT >= 10);
    assert!(TIMING_EVENTS.contains(&"thaw_start"));
    assert!(TIMING_EVENTS.contains(&"seed_start"));
    assert!(QC_TEST_TYPES * QC_TEST_REPLICATES >= CASSETTE_COUNT * 4);
    assert_eq!(RELEASE_LANE_NAMES, ["quarantine", "qa_hold", "released"]);
    assert!(HANDOFF_PORTS >= RELEASE_LANE_NAMES.len());
    assert!(BRIDGE_CLEARANCE_Z > DECK_Z + CASS_GATE_Z + 120.0);
    assert!(OVERHEAD_CAMERA_KEEP_OUT_Z >= BRIDGE_CLEARANCE_Z);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} must fit inside the contained station deck",
            rect.name
        );
    }

    for (i, first) in rects.iter().enumerate() {
        for second in rects.iter().skip(i + 1) {
            assert!(
                !first.overlaps(*second),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn layout_rects() -> [Rect; 8] {
    [
        rect(
            "cassette_identity_gate",
            CASS_GATE_CENTER,
            CASS_GATE_X,
            CASS_GATE_Y,
        ),
        rect(
            "lot_passage_ledger_panel",
            LEDGER_CENTER,
            LEDGER_X,
            LEDGER_Y,
        ),
        rect(
            "thaw_seed_timing_witness_rail",
            TIMING_CENTER,
            TIMING_X,
            TIMING_Y,
        ),
        rect("qc_sample_split_custody_bank", QC_CENTER, QC_X, QC_Y),
        rect(
            "quarantine_release_lane_gate",
            RELEASE_CENTER,
            RELEASE_X,
            RELEASE_Y,
        ),
        rect(
            "closed_handoff_port_bar",
            HANDOFF_CENTER,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        rect(
            "tamper_seal_and_operator_blind_coupon_bank",
            SEAL_CENTER,
            SEAL_X,
            SEAL_Y,
        ),
        rect(
            "machine_readable_label_tiles",
            LABEL_TILE_CENTER,
            LABEL_TILE_X,
            LABEL_TILE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_quarantine_spill_basin_cut"),
        STATION_X - 112.0,
        STATION_Y - 118.0,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -18.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);
    let front_drain_channel = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_channel_cut"),
        STATION_X - 190.0,
        34.0,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 64.0, DECK_Z - BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_quarantine_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        RIM_W + 42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 126.0,
        -STATION_Y / 2.0 + 22.0,
        DECK_Z - 4.0,
    );

    deck - basin - front_drain_channel - drain - component_sockets() - mount_holes()
        + perimeter_rim()
        + module_locator_bosses()
        + workflow_route_rails()
}

fn component_sockets() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_component_sockets"));
    for spec in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_deck_socket", spec.name),
                spec.x + 10.0,
                spec.y + 10.0,
                6.0,
            )
            .translate(spec.center.0, spec.center.1, DECK_Z - 3.0);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mount_holes"));
    for (i, (x, y)) in mount_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let z = DECK_Z + RIM_Z / 2.0;
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(format!("{OUTPUT_PREFIX}_rear_rim"), STATION_X, RIM_W, RIM_Z)
        .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(format!("{OUTPUT_PREFIX}_left_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn module_locator_bosses() -> Part {
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_module_locator_bosses"));
    for (i, spec) in layout_rects().iter().enumerate() {
        if spec.name == "quarantine_release_lane_gate" || spec.name == "closed_handoff_port_bar" {
            continue;
        }
        for (j, (x, y)) in rect_corner_points(*spec, 28.0).iter().enumerate() {
            let boss = centered_cylinder(
                format!("{OUTPUT_PREFIX}_locator_boss_{i}_{j}"),
                MOUNT_BOSS_D / 2.0,
                8.0,
                32,
            )
            .translate(*x, *y, DECK_Z + 4.0);
            let socket = centered_cylinder(
                format!("{OUTPUT_PREFIX}_locator_pin_socket_{i}_{j}"),
                3.2,
                10.0,
                18,
            )
            .translate(*x, *y, DECK_Z + 4.0);
            bosses = bosses + (boss - socket);
        }
    }
    bosses
}

fn workflow_route_rails() -> Part {
    let id_to_ledger = centered_cube(
        format!("{OUTPUT_PREFIX}_identity_to_lot_passage_route_rail"),
        330.0,
        8.0,
        12.0,
    )
    .translate(-85.0, 166.0, DECK_Z + 6.0);
    let ledger_to_qc = centered_cube(
        format!("{OUTPUT_PREFIX}_ledger_to_qc_route_rail"),
        350.0,
        8.0,
        12.0,
    )
    .rotate(0.0, 0.0, -62.0)
    .translate(245.0, 25.0, DECK_Z + 6.0);
    let timing_to_release = centered_cube(
        format!("{OUTPUT_PREFIX}_timing_to_quarantine_route_rail"),
        240.0,
        8.0,
        12.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(-260.0, -304.0, DECK_Z + 6.0);
    let qc_to_release = centered_cube(
        format!("{OUTPUT_PREFIX}_qc_to_release_route_rail"),
        230.0,
        8.0,
        12.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(188.0, -300.0, DECK_Z + 6.0);

    id_to_ledger + ledger_to_qc + timing_to_release + qc_to_release
}

fn cassette_identity_gate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_identity_gate_body"),
        CASS_GATE_X,
        CASS_GATE_Y,
        CASS_GATE_Z,
    );

    let mut pocket_cuts = Part::empty(format!("{OUTPUT_PREFIX}_cassette_identity_gate_cuts"));
    for index in 0..CASSETTE_COUNT {
        let (x, y) = cassette_xy(index);
        let cassette_pocket = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_{index}_sealed_cassette_pocket_cut"),
            CASS_POCKET_X,
            CASS_POCKET_Y,
            CASS_POCKET_DEPTH + 2.0,
        )
        .translate(x, y, CASS_GATE_Z / 2.0 - CASS_POCKET_DEPTH / 2.0 + 0.5);
        let keyed_corner = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_{index}_asymmetric_key_cut"),
            24.0,
            42.0,
            CASS_POCKET_DEPTH + 4.0,
        )
        .translate(
            x + CASS_POCKET_X / 2.0 - 14.0,
            y + CASS_POCKET_Y / 2.0 - 22.0,
            CASS_GATE_Z / 2.0 - CASS_POCKET_DEPTH / 2.0,
        );
        pocket_cuts = pocket_cuts + cassette_pocket + keyed_corner;
    }

    body - pocket_cuts
        + cassette_locator_pins()
        + cassette_barcode_windows()
        + cassette_rfid_lands()
        + identity_camera_datums()
        + wrong_orientation_blockers()
}

fn cassette_locator_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_cassette_locator_pins"));
    for index in 0..CASSETTE_COUNT {
        let (x, y) = cassette_xy(index);
        for (j, dx) in [-52.0, 52.0].iter().enumerate() {
            pins = pins
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_cassette_{index}_locator_pin_{j}"),
                    CASS_LOCATOR_PIN_D / 2.0,
                    20.0,
                    28,
                )
                .translate(
                    x + dx,
                    y - CASS_POCKET_Y / 2.0 - 14.0,
                    CASS_GATE_Z / 2.0 + 10.0,
                );
        }
    }
    pins
}

fn cassette_barcode_windows() -> Part {
    let mut windows = Part::empty(format!("{OUTPUT_PREFIX}_cassette_barcode_scan_windows"));
    for index in 0..CASSETTE_COUNT {
        let (x, y) = cassette_xy(index);
        for side in 0..BARCODE_WINDOWS_PER_CASSETTE {
            let y_offset = if side == 0 { -58.0 } else { 58.0 };
            let land = centered_cube(
                format!("{OUTPUT_PREFIX}_cassette_{index}_barcode_window_{side}"),
                92.0,
                18.0,
                7.0,
            )
            .translate(x, y + y_offset, CASS_GATE_Z / 2.0 + 3.5);
            let scan_fence = centered_cube(
                format!("{OUTPUT_PREFIX}_cassette_{index}_barcode_window_{side}_scan_fence"),
                104.0,
                5.0,
                18.0,
            )
            .translate(x, y + y_offset + 14.0, CASS_GATE_Z / 2.0 + 9.0);
            windows = windows + land + scan_fence;
        }
    }
    windows
}

fn cassette_rfid_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_cassette_rfid_lands"));
    for index in 0..CASSETTE_COUNT {
        let (x, y) = cassette_xy(index);
        lands = lands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_cassette_{index}_rfid_coil_land"),
                19.0,
                5.0,
                42,
            )
            .translate(x - 82.0, y, CASS_GATE_Z / 2.0 + 2.5);
    }
    lands
}

fn identity_camera_datums() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_identity_camera_datums"));
    for i in 0..ID_CAMERA_COUNT {
        let x = centered_index(i, ID_CAMERA_COUNT, 122.0);
        let mast = centered_cube(
            format!("{OUTPUT_PREFIX}_identity_camera_datum_mast_{i}"),
            18.0,
            18.0,
            78.0,
        )
        .translate(x, CASS_GATE_Y / 2.0 - 26.0, CASS_GATE_Z / 2.0 + 39.0);
        let target = centered_cylinder(
            format!("{OUTPUT_PREFIX}_identity_camera_fiducial_target_{i}"),
            13.0,
            5.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CASS_GATE_Y / 2.0 - 36.0, CASS_GATE_Z / 2.0 + 74.0);
        datums = datums + mast + target;
    }
    datums
}

fn wrong_orientation_blockers() -> Part {
    let mut blockers = Part::empty(format!("{OUTPUT_PREFIX}_wrong_orientation_blockers"));
    for index in 0..CASSETTE_COUNT {
        let (x, y) = cassette_xy(index);
        let left = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_{index}_left_datum_blocker"),
            12.0,
            CASS_POCKET_Y + 46.0,
            24.0,
        )
        .translate(x - CASS_POCKET_X / 2.0 - 18.0, y, CASS_GATE_Z / 2.0 + 12.0);
        let rear = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_{index}_rear_datum_blocker"),
            CASS_POCKET_X + 42.0,
            10.0,
            20.0,
        )
        .translate(x, y + CASS_POCKET_Y / 2.0 + 18.0, CASS_GATE_Z / 2.0 + 10.0);
        blockers = blockers + left + rear;
    }
    blockers
}

fn lot_passage_ledger_panel() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_lot_passage_ledger_body"),
        LEDGER_X,
        LEDGER_Y,
        LEDGER_Z,
    );
    let card_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_lot_card_recess_cut"),
        252.0,
        90.0,
        10.0,
    )
    .translate(-122.0, 70.0, LEDGER_Z / 2.0 - 4.0);
    let passage_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_passage_token_ladder_recess_cut"),
        442.0,
        38.0,
        8.0,
    )
    .translate(24.0, -36.0, LEDGER_Z / 2.0 - 3.5);
    let source_chain_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_source_vial_chain_recess_cut"),
        350.0,
        40.0,
        8.0,
    )
    .translate(-18.0, -88.0, LEDGER_Z / 2.0 - 3.5);

    body - card_recess - passage_recess - source_chain_recess
        + lot_card_slots()
        + passage_token_ladder()
        + source_vial_chain_slots()
        + lot_pass_barcode_lands()
        + operator_independent_review_fences()
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_lot_card_slots"));
    for i in 0..LOT_CARD_SLOTS {
        let x = -232.0 + i as f64 * 46.0;
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_lot_certificate_card_slot_{i}"),
            34.0,
            66.0,
            14.0,
        )
        .translate(x, 74.0, LEDGER_Z / 2.0 + 7.0);
        let retain_pin = centered_cylinder(
            format!("{OUTPUT_PREFIX}_lot_certificate_retainer_pin_{i}"),
            3.2,
            18.0,
            18,
        )
        .translate(x, 114.0, LEDGER_Z / 2.0 + 9.0);
        slots = slots + slot + retain_pin;
    }
    slots
}

fn passage_token_ladder() -> Part {
    let mut ladder = Part::empty(format!("{OUTPUT_PREFIX}_passage_number_token_ladder"));
    for i in 0..PASSAGE_TOKEN_COUNT {
        let x = centered_index(i, PASSAGE_TOKEN_COUNT, PASSAGE_TOKEN_PITCH);
        let pocket = centered_cube(
            format!("{OUTPUT_PREFIX}_passage_token_p{i}_pocket"),
            24.0,
            28.0,
            10.0,
        )
        .translate(x, -36.0, LEDGER_Z / 2.0 + 5.0);
        let detent = centered_cylinder(
            format!("{OUTPUT_PREFIX}_passage_token_p{i}_detent"),
            3.0,
            13.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -54.0, LEDGER_Z / 2.0 + 9.0);
        ladder = ladder + pocket - detent;
    }
    ladder
}

fn source_vial_chain_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_source_vial_chain_slots"));
    for i in 0..SOURCE_VIAL_CHAIN_SLOTS {
        let x = centered_index(i, SOURCE_VIAL_CHAIN_SLOTS, 42.0);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_source_vial_chain_slot_{i}"),
            30.0,
            28.0,
            10.0,
        )
        .translate(x, -88.0, LEDGER_Z / 2.0 + 5.0);
        let small_barcode = centered_cube(
            format!("{OUTPUT_PREFIX}_source_vial_chain_barcode_land_{i}"),
            28.0,
            8.0,
            5.0,
        )
        .translate(x, -112.0, LEDGER_Z / 2.0 + 2.5);
        slots = slots + slot + small_barcode;
    }
    slots
}

fn lot_pass_barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_lot_pass_barcode_lands"));
    for i in 0..LOT_PASS_BARCODE_LANDS {
        let x = centered_index(i % 5, 5, 88.0);
        let y = if i < 5 { 102.0 } else { 42.0 };
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_lot_pass_barcode_land_{i}"),
                66.0,
                18.0,
                6.0,
            )
            .translate(x + 62.0, y, LEDGER_Z / 2.0 + 3.0);
    }
    lands
}

fn operator_independent_review_fences() -> Part {
    let lot_fence = centered_cube(
        format!("{OUTPUT_PREFIX}_lot_chain_review_fence"),
        LEDGER_X - 46.0,
        8.0,
        24.0,
    )
    .translate(0.0, 8.0, LEDGER_Z / 2.0 + 12.0);
    let pass_fence = centered_cube(
        format!("{OUTPUT_PREFIX}_passage_number_review_fence"),
        LEDGER_X - 46.0,
        8.0,
        24.0,
    )
    .translate(0.0, -68.0, LEDGER_Z / 2.0 + 12.0);

    lot_fence + pass_fence
}

fn thaw_seed_timing_witness_rail() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_thaw_seed_timing_rail_body"),
        TIMING_X,
        TIMING_Y,
        TIMING_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_thaw_seed_timing_slot_cuts"));
    for i in 0..TIMING_EVENT_COUNT {
        let (x, y) = timing_event_xy(i);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_timer_token_slot_cut", TIMING_EVENTS[i]),
                TIMER_TOKEN_SLOT_X,
                TIMER_TOKEN_SLOT_Y,
                TIMER_TOKEN_SLOT_DEPTH + 1.0,
            )
            .translate(x, y, TIMING_Z / 2.0 - TIMER_TOKEN_SLOT_DEPTH / 2.0 + 0.5);
    }

    body - cuts + timing_event_witness_posts() + timing_bridge_comb() + cassette_time_crosslinks()
}

fn timing_event_witness_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_timing_event_witness_posts"));
    for i in 0..TIMING_EVENT_COUNT {
        let (x, y) = timing_event_xy(i);
        let start_stop_well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_{}_witness_well", TIMING_EVENTS[i]),
            11.0,
            16.0,
            28,
        )
        .translate(x - 54.0, y, TIMING_Z / 2.0 + 8.0);
        let flag = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_snap_flag", TIMING_EVENTS[i]),
            18.0,
            34.0,
            28.0,
        )
        .translate(x + 54.0, y, TIMING_Z / 2.0 + 14.0);
        posts = posts + start_stop_well + flag;
    }
    posts
}

fn timing_bridge_comb() -> Part {
    let comb = centered_cube(
        format!("{OUTPUT_PREFIX}_thaw_to_seed_elapsed_time_comb"),
        TIMING_X - 70.0,
        14.0,
        28.0,
    )
    .translate(0.0, TIMING_Y / 2.0 - 28.0, TIMING_Z / 2.0 + 14.0);
    let mut tick_cuts = Part::empty(format!("{OUTPUT_PREFIX}_elapsed_time_comb_tick_cuts"));
    for i in 0..=6 {
        let x = centered_index(i, 7, 54.0);
        tick_cuts = tick_cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_elapsed_time_tick_{i}_cut"),
                5.0,
                22.0,
                30.0,
            )
            .translate(x, TIMING_Y / 2.0 - 28.0, TIMING_Z / 2.0 + 14.0);
    }
    comb - tick_cuts
}

fn cassette_time_crosslinks() -> Part {
    let mut links = Part::empty(format!("{OUTPUT_PREFIX}_cassette_time_crosslink_lands"));
    for i in 0..CASSETTE_COUNT {
        let x = centered_index(i, CASSETTE_COUNT, 86.0);
        let link = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_{i}_time_crosslink_land"),
            66.0,
            16.0,
            6.0,
        )
        .translate(x, -TIMING_Y / 2.0 + 26.0, TIMING_Z / 2.0 + 3.0);
        let rfid = centered_cylinder(
            format!("{OUTPUT_PREFIX}_cassette_{i}_time_crosslink_rfid_disc"),
            7.0,
            5.0,
            22,
        )
        .translate(x - 42.0, -TIMING_Y / 2.0 + 26.0, TIMING_Z / 2.0 + 2.5);
        links = links + link + rfid;
    }
    links
}

fn qc_sample_split_custody_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_qc_split_bank_body"),
        QC_X,
        QC_Y,
        QC_Z,
    );
    let split_basin = centered_cube(
        format!("{OUTPUT_PREFIX}_qc_split_spill_basin_cut"),
        QC_X - 42.0,
        QC_Y - 42.0,
        10.0,
    )
    .translate(0.0, 0.0, QC_Z / 2.0 - 4.0);

    body - split_basin + qc_test_tube_matrix() + retain_aliquot_slots() + qc_chain_divider_walls()
}

fn qc_test_tube_matrix() -> Part {
    let mut tubes = Part::empty(format!("{OUTPUT_PREFIX}_qc_test_tube_matrix"));
    for test in 0..QC_TEST_TYPES {
        for rep in 0..QC_TEST_REPLICATES {
            let x = -174.0 + rep as f64 * QC_TUBE_PITCH_X;
            let y = 74.0 - test as f64 * QC_TUBE_PITCH_Y;
            let socket = centered_cylinder(
                format!("{OUTPUT_PREFIX}_qc_test_{test}_replicate_{rep}_tube_socket"),
                QC_TUBE_D / 2.0,
                QC_Z + 4.0,
                30,
            )
            .translate(x, y, 0.0);
            let collar = centered_cylinder(
                format!("{OUTPUT_PREFIX}_qc_test_{test}_replicate_{rep}_collar"),
                QC_TUBE_D / 2.0 + 5.0,
                7.0,
                30,
            )
            .translate(x, y, QC_Z / 2.0 + 3.5);
            tubes = tubes - socket + collar;
        }
    }
    tubes
}

fn retain_aliquot_slots() -> Part {
    let mut retain = Part::empty(format!("{OUTPUT_PREFIX}_retain_aliquot_slots"));
    for i in 0..RETAIN_ALIQUOT_SLOTS {
        let x = 82.0 + centered_index(i % 4, 4, 44.0);
        let y = if i < 4 { 62.0 } else { 10.0 };
        let socket = centered_cylinder(
            format!("{OUTPUT_PREFIX}_retain_aliquot_socket_{i}"),
            7.0,
            QC_Z + 4.0,
            26,
        )
        .translate(x, y, 0.0);
        let custody_tag = centered_cube(
            format!("{OUTPUT_PREFIX}_retain_aliquot_custody_tag_{i}"),
            28.0,
            9.0,
            5.0,
        )
        .translate(x, y - 25.0, QC_Z / 2.0 + 2.5);
        retain = retain - socket + custody_tag;
    }
    retain
}

fn qc_chain_divider_walls() -> Part {
    let vertical = centered_cube(
        format!("{OUTPUT_PREFIX}_qc_chain_vertical_divider"),
        10.0,
        QC_Y - 54.0,
        32.0,
    )
    .translate(14.0, 0.0, QC_Z / 2.0 + 16.0);
    let horizontal = centered_cube(
        format!("{OUTPUT_PREFIX}_qc_chain_sterility_auth_divider"),
        QC_X - 82.0,
        8.0,
        26.0,
    )
    .translate(-38.0, -22.0, QC_Z / 2.0 + 13.0);
    let myco_flag = centered_cube(
        format!("{OUTPUT_PREFIX}_mycoplasma_authentication_flag_land"),
        126.0,
        18.0,
        8.0,
    )
    .translate(126.0, -88.0, QC_Z / 2.0 + 4.0);

    vertical + horizontal + myco_flag
}

fn quarantine_release_lane_gate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_quarantine_release_lane_body"),
        RELEASE_X,
        RELEASE_Y,
        RELEASE_Z,
    );
    let mut slot_cuts = Part::empty(format!("{OUTPUT_PREFIX}_quarantine_release_slot_cuts"));
    for lane in 0..RELEASE_LANE_NAMES.len() {
        let lane_x = release_lane_x(lane);
        for slot in 0..RELEASE_SLOTS_PER_LANE {
            let x = lane_x + centered_index(slot, RELEASE_SLOTS_PER_LANE, RELEASE_SLOT_PITCH_X);
            slot_cuts = slot_cuts
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_{}_cassette_slot_{slot}_cut",
                        RELEASE_LANE_NAMES[lane]
                    ),
                    54.0,
                    42.0,
                    RELEASE_Z + 4.0,
                )
                .translate(x, -8.0, 0.0);
        }
    }

    body - slot_cuts + release_gate_shutters() + release_key_wells() + lane_status_fences()
}

fn release_gate_shutters() -> Part {
    let mut shutters = Part::empty(format!("{OUTPUT_PREFIX}_release_gate_shutters"));
    for lane in 0..RELEASE_LANE_NAMES.len() {
        let lane_x = release_lane_x(lane);
        let shutter = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_shutter_bar", RELEASE_LANE_NAMES[lane]),
            302.0,
            10.0,
            28.0,
        )
        .translate(lane_x, RELEASE_Y / 2.0 + 11.0, RELEASE_Z / 2.0 + 14.0);
        let lane_flag = centered_cube(
            format!(
                "{OUTPUT_PREFIX}_{}_status_flag_land",
                RELEASE_LANE_NAMES[lane]
            ),
            206.0,
            18.0,
            6.0,
        )
        .translate(lane_x, -RELEASE_Y / 2.0 + 15.0, RELEASE_Z / 2.0 + 3.0);
        shutters = shutters + shutter + lane_flag;
    }
    shutters
}

fn release_key_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_two_key_release_wells"));
    for lane in 0..RELEASE_LANE_NAMES.len() {
        let lane_x = release_lane_x(lane);
        for key in 0..RELEASE_KEY_WELLS_PER_LANE {
            let x = lane_x + centered_index(key, RELEASE_KEY_WELLS_PER_LANE, 46.0);
            wells = wells
                + centered_cylinder(
                    format!(
                        "{OUTPUT_PREFIX}_{}_release_key_well_{key}",
                        RELEASE_LANE_NAMES[lane]
                    ),
                    10.0,
                    12.0,
                    30,
                )
                .translate(x, RELEASE_Y / 2.0 - 20.0, RELEASE_Z / 2.0 + 6.0);
        }
    }
    wells
}

fn lane_status_fences() -> Part {
    let left_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_quarantine_hold_lane_divider"),
        10.0,
        RELEASE_Y + 16.0,
        32.0,
    )
    .translate(-RELEASE_LANE_PITCH_X / 2.0, 0.0, RELEASE_Z / 2.0 + 16.0);
    let right_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_hold_released_lane_divider"),
        10.0,
        RELEASE_Y + 16.0,
        32.0,
    )
    .translate(RELEASE_LANE_PITCH_X / 2.0, 0.0, RELEASE_Z / 2.0 + 16.0);

    left_divider + right_divider
}

fn closed_handoff_port_bar() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_closed_handoff_bar_body"),
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let mut port_cuts = Part::empty(format!("{OUTPUT_PREFIX}_closed_handoff_port_cuts"));
    for i in 0..HANDOFF_PORTS {
        let x = centered_index(i, HANDOFF_PORTS, HANDOFF_PORT_PITCH);
        let port = centered_cylinder(
            format!("{OUTPUT_PREFIX}_handoff_port_{i}_dry_break_cut"),
            HANDOFF_PORT_D / 2.0,
            HANDOFF_Y + 8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 0.0);
        let latch_window = centered_cube(
            format!("{OUTPUT_PREFIX}_handoff_port_{i}_latch_window_cut"),
            52.0,
            12.0,
            28.0,
        )
        .translate(x, -HANDOFF_Y / 2.0 + 7.0, 8.0);
        port_cuts = port_cuts + port + latch_window;
    }

    body - port_cuts + handoff_latch_sensors() + handoff_route_flags()
}

fn handoff_latch_sensors() -> Part {
    let mut sensors = Part::empty(format!("{OUTPUT_PREFIX}_handoff_latch_sensors"));
    for i in 0..HANDOFF_PORTS {
        let x = centered_index(i, HANDOFF_PORTS, HANDOFF_PORT_PITCH);
        for side in [-1.0, 1.0] {
            sensors = sensors
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_handoff_port_{i}_latch_sensor_{side}"),
                    4.0,
                    6.0,
                    18,
                )
                .translate(x + side * 34.0, -HANDOFF_Y / 2.0 - 4.0, 12.0);
        }
    }
    sensors
}

fn handoff_route_flags() -> Part {
    let mut flags = Part::empty(format!("{OUTPUT_PREFIX}_handoff_route_flags"));
    for i in 0..HANDOFF_PORTS {
        let x = centered_index(i, HANDOFF_PORTS, HANDOFF_PORT_PITCH);
        flags = flags
            + centered_cube(
                format!("{OUTPUT_PREFIX}_handoff_port_{i}_route_barcode_land"),
                86.0,
                14.0,
                6.0,
            )
            .translate(x, HANDOFF_Y / 2.0 + 12.0, HANDOFF_Z / 2.0 + 3.0);
    }
    flags
}

fn tamper_seal_and_operator_blind_coupon_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_tamper_seal_bank_body"),
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_tamper_seal_bank_cuts"));
    for i in 0..TAMPER_SEAL_SLOTS {
        let x = if i % 2 == 0 { -24.0 } else { 24.0 };
        let y = 96.0 - (i / 2) as f64 * 31.0;
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tamper_evident_seal_slot_{i}_cut"),
                38.0,
                18.0,
                9.0,
            )
            .translate(x, y, SEAL_Z / 2.0 - 4.0);
    }

    body - cuts + operator_blind_coupon_slots() + seal_break_witness_tabs()
}

fn operator_blind_coupon_slots() -> Part {
    let mut coupons = Part::empty(format!("{OUTPUT_PREFIX}_operator_blind_coupon_slots"));
    for i in 0..OPERATOR_BLIND_COUPONS {
        let x = centered_index(i % 2, 2, 48.0);
        let y = -58.0 - (i / 2) as f64 * 24.0;
        coupons = coupons
            + centered_cube(
                format!("{OUTPUT_PREFIX}_operator_blind_coupon_slot_{i}"),
                38.0,
                13.0,
                7.0,
            )
            .translate(x, y, SEAL_Z / 2.0 + 3.5);
    }
    coupons
}

fn seal_break_witness_tabs() -> Part {
    let upper = centered_cube(
        format!("{OUTPUT_PREFIX}_seal_break_witness_tab_upper"),
        SEAL_X - 24.0,
        10.0,
        20.0,
    )
    .translate(0.0, SEAL_Y / 2.0 - 14.0, SEAL_Z / 2.0 + 10.0);
    let lower = centered_cube(
        format!("{OUTPUT_PREFIX}_seal_break_witness_tab_lower"),
        SEAL_X - 24.0,
        10.0,
        20.0,
    )
    .translate(0.0, -SEAL_Y / 2.0 + 14.0, SEAL_Z / 2.0 + 10.0);

    upper + lower
}

fn environmental_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_CLEARANCE_Z / 2.0);

    left_post + right_post + beam + bridge_camera_pods() + bridge_light_bars() + logger_pockets()
}

fn bridge_camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_evidence_camera_pods"));
    for i in 0..EVIDENCE_CAMERA_PODS {
        let x = centered_index(i, EVIDENCE_CAMERA_PODS, 246.0);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_evidence_camera_pod_{i}"),
            62.0,
            48.0,
            34.0,
        )
        .translate(
            x,
            -BRIDGE_POST_Y / 2.0 - 22.0,
            BRIDGE_CLEARANCE_Z / 2.0 - 12.0,
        );
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_evidence_camera_lens_{i}"),
            11.0,
            9.0,
            30,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            -BRIDGE_POST_Y / 2.0 - 48.0,
            BRIDGE_CLEARANCE_Z / 2.0 - 12.0,
        );
        pods = pods + pod + lens;
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for i in 0..EVIDENCE_LIGHT_BARS {
        let x = centered_index(i, EVIDENCE_LIGHT_BARS, 285.0);
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{i}"),
                180.0,
                10.0,
                9.0,
            )
            .translate(
                x,
                BRIDGE_POST_Y / 2.0 + 8.0,
                BRIDGE_CLEARANCE_Z / 2.0 - 28.0,
            );
    }
    bars
}

fn logger_pockets() -> Part {
    let mut pockets = Part::empty(format!("{OUTPUT_PREFIX}_environmental_logger_pockets"));
    for i in 0..LOGGER_POCKETS {
        let x = centered_index(i, LOGGER_POCKETS, 160.0);
        let pocket = centered_cube(
            format!("{OUTPUT_PREFIX}_environmental_logger_pocket_{i}"),
            104.0,
            28.0,
            34.0,
        )
        .translate(
            x,
            BRIDGE_POST_Y / 2.0 + 34.0,
            BRIDGE_CLEARANCE_Z / 2.0 + 4.0,
        );
        let cable = centered_cylinder(
            format!("{OUTPUT_PREFIX}_environmental_logger_cable_gland_{i}"),
            5.0,
            18.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x + 58.0,
            BRIDGE_POST_Y / 2.0 + 50.0,
            BRIDGE_CLEARANCE_Z / 2.0 + 4.0,
        );
        pockets = pockets + pocket + cable;
    }
    pockets
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_sweep_keepout_gauge"),
        STATION_X - 160.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 48.0,
        DECK_Z + 3.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_handoff_keepout_gauge"),
        STATION_X - 240.0,
        REAR_HANDOFF_KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_HANDOFF_KEEP_OUT_Y / 2.0 - 52.0,
        DECK_Z + 3.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_service_keepout_gauge"),
        SIDE_SERVICE_KEEP_OUT_X,
        STATION_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X / 2.0 + 52.0,
        0.0,
        DECK_Z + 3.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_service_keepout_gauge"),
        SIDE_SERVICE_KEEP_OUT_X,
        STATION_Y - 220.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X / 2.0 - 52.0,
        0.0,
        DECK_Z + 3.0,
    );
    let overhead = centered_cube(
        format!("{OUTPUT_PREFIX}_overhead_camera_clearance_keepout_gauge"),
        STATION_X - 300.0,
        42.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, OVERHEAD_CAMERA_KEEP_OUT_Z);

    front + rear + left + right + overhead
}

fn machine_readable_label_tiles() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_machine_readable_label_tile_backer"),
        LABEL_TILE_X,
        LABEL_TILE_Y,
        LABEL_TILE_Z,
    );
    let mut tiles = Part::empty(format!("{OUTPUT_PREFIX}_machine_readable_label_tiles"));
    for i in 0..LABEL_TILE_COUNT {
        let x = centered_index(i % 2, 2, 64.0);
        let y = 82.0 - (i / 2) as f64 * 72.0;
        let tile = centered_cube(
            format!("{OUTPUT_PREFIX}_identity_label_tile_{i}"),
            48.0,
            34.0,
            7.0,
        )
        .translate(x, y, LABEL_TILE_Z / 2.0 + 3.5);
        let notch_count = i + 1;
        let mut notches = Part::empty(format!("{OUTPUT_PREFIX}_identity_label_tile_{i}_notches"));
        for j in 0..notch_count {
            notches = notches
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_identity_label_tile_{i}_notch_{j}"),
                    5.0,
                    24.0,
                    8.0,
                )
                .translate(x - 18.0 + j as f64 * 7.0, y, LABEL_TILE_Z / 2.0 + 4.0);
        }
        tiles = tiles + (tile - notches);
    }

    body + tiles
}

fn mount_positions() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 66.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 66.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 66.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 66.0),
        (-STATION_X / 2.0 + 70.0, 0.0),
        (STATION_X / 2.0 - 70.0, 0.0),
        (0.0, -STATION_Y / 2.0 + 66.0),
        (0.0, STATION_Y / 2.0 - 66.0),
    ]
}

fn rect_corner_points(rect: Rect, inset: f64) -> [(f64, f64); 4] {
    [
        (
            rect.center.0 - rect.x / 2.0 + inset,
            rect.center.1 - rect.y / 2.0 + inset,
        ),
        (
            rect.center.0 + rect.x / 2.0 - inset,
            rect.center.1 - rect.y / 2.0 + inset,
        ),
        (
            rect.center.0 - rect.x / 2.0 + inset,
            rect.center.1 + rect.y / 2.0 - inset,
        ),
        (
            rect.center.0 + rect.x / 2.0 - inset,
            rect.center.1 + rect.y / 2.0 - inset,
        ),
    ]
}

fn cassette_xy(index: usize) -> (f64, f64) {
    let row = index / CASSETTE_COLS;
    let col = index % CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, CASS_PITCH_X),
        centered_index(row, CASSETTE_ROWS, CASS_PITCH_Y),
    )
}

fn timing_event_xy(index: usize) -> (f64, f64) {
    let row = index / 3;
    let col = index % 3;
    (
        centered_index(col, 3, 128.0),
        if row == 0 { 48.0 } else { -46.0 },
    )
}

fn release_lane_x(lane: usize) -> f64 {
    centered_index(lane, RELEASE_LANE_NAMES.len(), RELEASE_LANE_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
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
            assert!(path.starts_with("output/closed_passage_number_lot_chain_identity_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_lot_passage_identity_and_release_controls() {
        for feature in [
            "operator_independent_cassette_identity",
            "cell_lot_chain_of_identity",
            "passage_number_token_ladder",
            "thaw_start_witness",
            "seed_start_witness",
            "quarantine_release_lanes",
            "sterility_mycoplasma_authentication_split",
            "tamper_evident_seals",
            "environmental_logger_pocket",
            "camera_evidence_bridge",
            "closed_handoff_ports",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn all_primary_modules_fit_without_plan_view_collision() {
        assert_design_constraints();
        assert!(layout_rects().iter().all(|rect| rect.fits_inside_station()));
        assert!(CASS_GATE_CENTER.0 + CASS_GATE_X / 2.0 < LEDGER_CENTER.0 - LEDGER_X / 2.0);
        assert!(TIMING_CENTER.0 + TIMING_X / 2.0 < QC_CENTER.0 - QC_X / 2.0);
        assert!(QC_CENTER.1 - QC_Y / 2.0 > RELEASE_CENTER.1 + RELEASE_Y / 2.0);
    }

    #[test]
    fn cassette_identity_is_redundant_and_operator_independent() {
        assert_eq!(CASSETTE_COUNT, 4);
        assert_eq!(CASSETTE_COUNT, CASSETTE_ROWS * CASSETTE_COLS);
        assert_eq!(BARCODE_WINDOWS_PER_CASSETTE, 2);
        assert_eq!(RFID_LANDS_PER_CASSETTE, 1);
        assert!(ID_CAMERA_COUNT >= 3);
        for index in 0..CASSETTE_COUNT {
            let (x, y) = cassette_xy(index);
            assert!(x.abs() + CASS_POCKET_X / 2.0 < CASS_GATE_X / 2.0 - 34.0);
            assert!(y.abs() + CASS_POCKET_Y / 2.0 < CASS_GATE_Y / 2.0 - 34.0);
        }
    }

    #[test]
    fn lot_passage_chain_has_more_capacity_than_single_run() {
        assert!(LOT_CARD_SLOTS >= CASSETTE_COUNT);
        assert!(PASSAGE_TOKEN_COUNT >= 10);
        assert!(SOURCE_VIAL_CHAIN_SLOTS >= CASSETTE_COUNT * 2);
        assert!(LOT_PASS_BARCODE_LANDS >= CASSETTE_COUNT * 2);
        let first = centered_index(0, PASSAGE_TOKEN_COUNT, PASSAGE_TOKEN_PITCH);
        let last = centered_index(
            PASSAGE_TOKEN_COUNT - 1,
            PASSAGE_TOKEN_COUNT,
            PASSAGE_TOKEN_PITCH,
        );
        assert!(first > -LEDGER_X / 2.0 + 40.0);
        assert!(last < LEDGER_X / 2.0 - 40.0);
    }

    #[test]
    fn thaw_seed_timing_chain_names_critical_events() {
        assert_eq!(TIMING_EVENT_COUNT, TIMING_EVENTS.len());
        assert_eq!(TIMING_EVENTS[0], "thaw_start");
        assert_eq!(TIMING_EVENTS[2], "seed_start");
        assert!(TIMING_EVENTS.contains(&"incubator_load"));
        assert!(MAX_THAW_TO_SEED_MINUTES <= 90);
        for index in 0..TIMING_EVENT_COUNT {
            let (x, y) = timing_event_xy(index);
            assert!(x.abs() + TIMER_TOKEN_SLOT_X / 2.0 < TIMING_X / 2.0 - 16.0);
            assert!(y.abs() + TIMER_TOKEN_SLOT_Y / 2.0 < TIMING_Y / 2.0 - 16.0);
        }
    }

    #[test]
    fn quarantine_release_controls_are_physically_distinct() {
        assert_eq!(RELEASE_LANE_NAMES, ["quarantine", "qa_hold", "released"]);
        assert_eq!(RELEASE_SLOTS_PER_LANE, CASSETTE_COUNT);
        assert_eq!(RELEASE_KEY_WELLS_PER_LANE, 2);
        assert!(RELEASE_LANE_PITCH_X > RELEASE_SLOTS_PER_LANE as f64 * RELEASE_SLOT_PITCH_X);
        assert!(release_lane_x(0) < release_lane_x(1));
        assert!(release_lane_x(1) < release_lane_x(2));
    }

    #[test]
    fn qc_and_evidence_capacity_cover_reproducibility_controls() {
        assert_eq!(QC_TEST_TYPES, 4);
        assert_eq!(QC_TEST_REPLICATES, 4);
        assert!(QC_TEST_TYPES * QC_TEST_REPLICATES >= CASSETTE_COUNT * 4);
        assert!(RETAIN_ALIQUOT_SLOTS >= CASSETTE_COUNT * 2);
        assert!(TAMPER_SEAL_SLOTS >= CASSETTE_COUNT * 3);
        assert!(OPERATOR_BLIND_COUPONS >= CASSETTE_COUNT * 2);
        assert!(EVIDENCE_CAMERA_PODS >= 5);
        assert!(LOGGER_POCKETS >= 2);
        assert_eq!(KEEP_OUT_GAUGES, 5);
    }
}
