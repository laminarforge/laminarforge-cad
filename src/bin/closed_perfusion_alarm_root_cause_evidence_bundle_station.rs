use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion alarm root-cause evidence bundle station.
//
// Design intent:
// - Mechanically package the evidence gathered after a closed perfusion alarm:
//   event logs, sensor snapshots, witness coupons, sample retains, cassette
//   quarantine state, custody identifiers, reference faults, and disposition
//   gates on one contained deck.
// - Provide physical lands, nests, coupons, and camera references so automation
//   can preserve and inspect the evidence bundle without defining how alarms
//   are triaged or what biological result is acceptable.
//
// This is mechanical validation packaging only. It is not an alarm SOP,
// sterile-process claim, release criterion, or biological acceptance criterion.

const DESIGN_SCOPE: &str = "mechanical validation packaging only; not an alarm SOP, sterile-process claim, release criterion, or biological acceptance criterion";

const OUTPUTS: [&str; 13] = [
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_containment_deck.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_event_logger_dock.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_sensor_snapshot_cartridge_nests.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_flow_pressure_bubble_witness_coupons.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_sample_retain_split.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_cassette_quarantine_evidence_dock.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_timeline_token_rail.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_barcode_rfid_custody_lands.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_high_low_fault_reference_pockets.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_release_hold_reject_gates.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_camera_evidence_bridge.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_robot_service_keepouts.stl",
    "output/closed_perfusion_alarm_root_cause_evidence_bundle_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 23] = [
    "containment_deck",
    "event_logger_dock",
    "event_export_key",
    "sensor_snapshot_cartridge_nests",
    "flow_witness_coupon",
    "pressure_witness_coupon",
    "bubble_witness_coupon",
    "sample_retain_split",
    "cassette_evidence_dock",
    "quarantine_evidence_dock",
    "timeline_token_rail",
    "barcode_custody_lands",
    "rfid_custody_lands",
    "high_fault_reference_pockets",
    "low_fault_reference_pockets",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepout",
    "service_keepout",
    "top_service_clearance",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 880.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 54.0;
const SUMP_X: f64 = 1160.0;
const SUMP_Y: f64 = 700.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const LOGGER_X: f64 = 390.0;
const LOGGER_Y: f64 = 132.0;
const LOGGER_Z: f64 = 38.0;
const LOGGER_POS: (f64, f64) = (-430.0, 280.0);
const EVENT_LOGGER_BAYS: usize = 4;
const LOGGER_BAY_X: f64 = 74.0;
const LOGGER_BAY_Y: f64 = 78.0;
const LOGGER_EXPORT_KEYS: usize = 4;
const LOGGER_CABLE_COMBS: usize = 5;
const LOGGER_TAMPER_SEAL_PADS: usize = 4;

const SNAPSHOT_X: f64 = 560.0;
const SNAPSHOT_Y: f64 = 190.0;
const SNAPSHOT_Z: f64 = 44.0;
const SNAPSHOT_POS: (f64, f64) = (45.0, 280.0);
const SNAPSHOT_CARTRIDGES: usize = 8;
const SNAPSHOT_COLS: usize = 4;
const SNAPSHOT_ROWS: usize = 2;
const SNAPSHOT_SLOT_X: f64 = 92.0;
const SNAPSHOT_SLOT_Y: f64 = 58.0;
const SNAPSHOT_PITCH_X: f64 = 114.0;
const SNAPSHOT_PITCH_Y: f64 = 78.0;
const SNAPSHOT_LATCHES: usize = 8;

const WITNESS_X: f64 = 570.0;
const WITNESS_Y: f64 = 185.0;
const WITNESS_Z: f64 = 42.0;
const WITNESS_POS: (f64, f64) = (-325.0, 55.0);
const FLOW_COUPONS: usize = 6;
const PRESSURE_COUPONS: usize = 4;
const BUBBLE_COUPONS: usize = 6;
const COUPON_X: f64 = 46.0;
const COUPON_Y: f64 = 28.0;
const COUPON_PITCH_X: f64 = 58.0;
const PRESSURE_MEMBRANE_D: f64 = 31.0;
const BUBBLE_WINDOW_D: f64 = 26.0;

const RETAIN_X: f64 = 370.0;
const RETAIN_Y: f64 = 230.0;
const RETAIN_Z: f64 = 44.0;
const RETAIN_POS: (f64, f64) = (-430.0, -210.0);
const RETAIN_WELLS_PER_SPLIT: usize = 5;
const RETAIN_SPLITS: usize = 2;
const RETAIN_WELL_D: f64 = 28.0;
const RETAIN_WELL_PITCH: f64 = 43.0;
const SPLIT_CHANNEL_D: f64 = 8.0;
const RETAIN_SEAL_PADS: usize = 6;

const CASSETTE_X: f64 = 420.0;
const CASSETTE_Y: f64 = 230.0;
const CASSETTE_Z: f64 = 48.0;
const CASSETTE_POS: (f64, f64) = (15.0, -215.0);
const CASSETTE_POCKET_X: f64 = 238.0;
const CASSETTE_POCKET_Y: f64 = 128.0;
const QUARANTINE_POCKET_X: f64 = 122.0;
const QUARANTINE_POCKET_Y: f64 = 128.0;
const CASSETTE_CLAMPS: usize = 4;
const QUARANTINE_FLAG_WELLS: usize = 5;

const TOKEN_X: f64 = 380.0;
const TOKEN_Y: f64 = 170.0;
const TOKEN_Z: f64 = 32.0;
const TOKEN_POS: (f64, f64) = (420.0, -220.0);
const TIMELINE_EVENTS: usize = 9;
const TOKEN_SLOT_X: f64 = 34.0;
const TOKEN_SLOT_Y: f64 = 26.0;
const TOKEN_PITCH_X: f64 = 36.0;
const TOKEN_LANES: usize = 3;
const TOKEN_LANE_PITCH: f64 = 43.0;

const CUSTODY_X: f64 = 280.0;
const CUSTODY_Y: f64 = 140.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (485.0, 280.0);
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;
const CUSTODY_SEAL_PADS: usize = 4;

const FAULT_REF_X: f64 = 500.0;
const FAULT_REF_Y: f64 = 190.0;
const FAULT_REF_Z: f64 = 40.0;
const FAULT_REF_POS: (f64, f64) = (320.0, 55.0);
const FAULT_REFERENCE_PAIRS: usize = 5;
const FAULT_POCKET_D: f64 = 25.0;
const FAULT_PAIR_PITCH_X: f64 = 76.0;
const HIGH_LOW_LABEL_LANDS: usize = 10;

const GATE_X: f64 = 480.0;
const GATE_Y: f64 = 86.0;
const GATE_Z: f64 = 40.0;
const GATE_POS: (f64, f64) = (0.0, -365.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 980.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 235.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -30.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const EVIDENCE_CARD_CLAMPS: usize = 6;

const ROBOT_KEEPOUT_X: f64 = 1180.0;
const ROBOT_KEEPOUT_Y: f64 = 96.0;
const ROBOT_KEEPOUT_Z: f64 = 76.0;
const SERVICE_KEEPOUT_X: f64 = 102.0;
const SERVICE_KEEPOUT_Y: f64 = 690.0;
const SERVICE_KEEPOUT_Z: f64 = 94.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 305.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(event_logger_dock(), OUTPUTS[1]);
    write_part(sensor_snapshot_cartridge_nests(), OUTPUTS[2]);
    write_part(flow_pressure_bubble_witness_coupons(), OUTPUTS[3]);
    write_part(sample_retain_split(), OUTPUTS[4]);
    write_part(cassette_quarantine_evidence_dock(), OUTPUTS[5]);
    write_part(timeline_token_rail(), OUTPUTS[6]);
    write_part(barcode_rfid_custody_lands(), OUTPUTS[7]);
    write_part(high_low_fault_reference_pockets(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed perfusion alarm root-cause evidence bundle station: {:.0}mm x {:.0}mm contained deck, {} logger bays, {} sensor snapshot cartridges, {} timeline events.",
        DECK_X, DECK_Y, EVENT_LOGGER_BAYS, SNAPSHOT_CARTRIDGES, TIMELINE_EVENTS
    );
    println!(
        "Witness and retain packaging: {} flow coupons, {} pressure coupons, {} bubble coupons, {} split retain wells, {} cassette/quarantine flag wells.",
        FLOW_COUPONS,
        PRESSURE_COUPONS,
        BUBBLE_COUPONS,
        RETAIN_SPLITS * RETAIN_WELLS_PER_SPLIT,
        QUARANTINE_FLAG_WELLS
    );
    println!(
        "Custody and disposition: {} barcode lands, {} RFID lands, {} high/low fault reference pairs, release/hold/reject gates, {} camera mounts. Scope: {DESIGN_SCOPE}.",
        BARCODE_LANDS, RFID_LANDS, FAULT_REFERENCE_PAIRS, CAMERA_MOUNTS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_alarm_evidence_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_alarm_evidence_station_shallow_evidence_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_alarm_evidence_station_capped_evidence_drain",
        DRAIN_D / 2.0,
        46.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 78.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() - module_landing_recesses()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_alarm_evidence_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-(DECK_X / 2.0 - RIM_W / 2.0), 0.0, rim_center_z());
    let right = centered_cube(
        "closed_alarm_evidence_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_alarm_evidence_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_alarm_evidence_station_front_low_evidence_lip",
        DECK_X - 150.0,
        RIM_W,
        RIM_Z * 0.60,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 - RIM_W / 2.0),
        DECK_Z / 2.0 + RIM_Z * 0.30,
    );

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_alarm_evidence_station_deck_datums");
    for (i, (x, y)) in [
        (-595.0, -380.0),
        (595.0, -380.0),
        (-595.0, 380.0),
        (595.0, 380.0),
        (-300.0, -380.0),
        (300.0, -380.0),
        (-300.0, 380.0),
        (300.0, 380.0),
        (0.0, -380.0),
        (0.0, 380.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("closed_alarm_evidence_station_datum_boss_{i}"),
            13.0,
            5.0,
            40,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        let bore = centered_cylinder(
            format!("closed_alarm_evidence_station_datum_bore_{i}"),
            3.2,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.5);
        datums = datums + (pad - bore);
    }
    datums
}

fn module_landing_recesses() -> Part {
    let logger = landing_recess("event_logger", LOGGER_POS, LOGGER_X + 24.0, LOGGER_Y + 22.0);
    let snapshots = landing_recess(
        "sensor_snapshot",
        SNAPSHOT_POS,
        SNAPSHOT_X + 24.0,
        SNAPSHOT_Y + 24.0,
    );
    let witness = landing_recess(
        "witness_coupon",
        WITNESS_POS,
        WITNESS_X + 24.0,
        WITNESS_Y + 24.0,
    );
    let retain = landing_recess("retain_split", RETAIN_POS, RETAIN_X + 24.0, RETAIN_Y + 24.0);
    let cassette = landing_recess(
        "cassette_quarantine",
        CASSETTE_POS,
        CASSETTE_X + 24.0,
        CASSETTE_Y + 24.0,
    );
    let token = landing_recess("timeline_token", TOKEN_POS, TOKEN_X + 22.0, TOKEN_Y + 22.0);
    let custody = landing_recess("custody", CUSTODY_POS, CUSTODY_X + 22.0, CUSTODY_Y + 22.0);
    let fault_ref = landing_recess(
        "fault_reference",
        FAULT_REF_POS,
        FAULT_REF_X + 24.0,
        FAULT_REF_Y + 24.0,
    );

    logger + snapshots + witness + retain + cassette + token + custody + fault_ref
}

fn landing_recess(name: &str, pos: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(
        format!("closed_alarm_evidence_station_{name}_landing_recess"),
        x,
        y,
        5.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 - 1.8)
}

fn event_logger_dock() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_event_logger_dock_base",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let pockets = event_logger_pockets();
    let keys = event_export_keys();
    let combs = logger_cable_combs();
    let seals = logger_tamper_seal_pads();

    (base - pockets + keys + combs + seals).translate(
        LOGGER_POS.0,
        LOGGER_POS.1,
        DECK_Z / 2.0 + LOGGER_Z / 2.0,
    )
}

fn event_logger_pockets() -> Part {
    let mut pockets = Part::empty("closed_alarm_evidence_station_event_logger_pockets");
    for i in 0..EVENT_LOGGER_BAYS {
        let x = -136.5 + i as f64 * 91.0;
        let pocket = centered_cube(
            format!("closed_alarm_evidence_station_event_logger_bay_{i}"),
            LOGGER_BAY_X,
            LOGGER_BAY_Y,
            14.0,
        )
        .translate(x, -8.0, LOGGER_Z / 2.0 - 4.5);
        pockets = pockets + pocket;
    }
    pockets
}

fn event_export_keys() -> Part {
    let mut keys = Part::empty("closed_alarm_evidence_station_event_export_keys");
    for i in 0..LOGGER_EXPORT_KEYS {
        let x = -136.5 + i as f64 * 91.0;
        let key = centered_cube(
            format!("closed_alarm_evidence_station_event_export_key_{i}"),
            30.0,
            12.0,
            11.0,
        )
        .translate(x, -LOGGER_Y / 2.0 + 18.0, LOGGER_Z / 2.0 + 5.5);
        keys = keys + key;
    }
    keys
}

fn logger_cable_combs() -> Part {
    let mut combs = Part::empty("closed_alarm_evidence_station_logger_cable_combs");
    for i in 0..LOGGER_CABLE_COMBS {
        let x = -168.0 + i as f64 * 84.0;
        let comb = centered_cube(
            format!("closed_alarm_evidence_station_logger_cable_comb_{i}"),
            10.0,
            42.0,
            22.0,
        )
        .translate(x, LOGGER_Y / 2.0 - 24.0, LOGGER_Z / 2.0 + 8.0);
        combs = combs + comb;
    }
    combs
}

fn logger_tamper_seal_pads() -> Part {
    let mut pads = Part::empty("closed_alarm_evidence_station_logger_tamper_seal_pads");
    for i in 0..LOGGER_TAMPER_SEAL_PADS {
        let x = if i % 2 == 0 { -176.0 } else { 176.0 };
        let y = if i < 2 { -52.0 } else { 52.0 };
        let pad = centered_cylinder(
            format!("closed_alarm_evidence_station_logger_tamper_seal_pad_{i}"),
            10.0,
            3.0,
            28,
        )
        .translate(x, y, LOGGER_Z / 2.0 + 2.0);
        pads = pads + pad;
    }
    pads
}

fn sensor_snapshot_cartridge_nests() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_sensor_snapshot_cartridge_nest_base",
        SNAPSHOT_X,
        SNAPSHOT_Y,
        SNAPSHOT_Z,
    );
    let cartridge_slots = snapshot_cartridge_slots();
    let latches = snapshot_cartridge_latches();
    let orientation_keys = snapshot_orientation_keys();

    (base - cartridge_slots + latches + orientation_keys).translate(
        SNAPSHOT_POS.0,
        SNAPSHOT_POS.1,
        DECK_Z / 2.0 + SNAPSHOT_Z / 2.0,
    )
}

fn snapshot_cartridge_slots() -> Part {
    let mut slots = Part::empty("closed_alarm_evidence_station_sensor_snapshot_slots");
    for row in 0..SNAPSHOT_ROWS {
        for col in 0..SNAPSHOT_COLS {
            let index = row * SNAPSHOT_COLS + col;
            let x = -snapshot_span_x() / 2.0 + col as f64 * SNAPSHOT_PITCH_X;
            let y = -snapshot_span_y() / 2.0 + row as f64 * SNAPSHOT_PITCH_Y;
            let slot = centered_cube(
                format!("closed_alarm_evidence_station_sensor_snapshot_cartridge_nest_{index}"),
                SNAPSHOT_SLOT_X,
                SNAPSHOT_SLOT_Y,
                16.0,
            )
            .translate(x, y, SNAPSHOT_Z / 2.0 - 5.0);
            slots = slots + slot;
        }
    }
    slots
}

fn snapshot_cartridge_latches() -> Part {
    let mut latches = Part::empty("closed_alarm_evidence_station_sensor_snapshot_latches");
    for i in 0..SNAPSHOT_LATCHES {
        let col = i % SNAPSHOT_COLS;
        let row = i / SNAPSHOT_COLS;
        let x = -snapshot_span_x() / 2.0 + col as f64 * SNAPSHOT_PITCH_X;
        let y = -snapshot_span_y() / 2.0 + row as f64 * SNAPSHOT_PITCH_Y + 34.0;
        let latch = centered_cube(
            format!("closed_alarm_evidence_station_snapshot_latch_{i}"),
            42.0,
            9.0,
            15.0,
        )
        .translate(x, y, SNAPSHOT_Z / 2.0 + 7.5);
        latches = latches + latch;
    }
    latches
}

fn snapshot_orientation_keys() -> Part {
    let mut keys = Part::empty("closed_alarm_evidence_station_sensor_snapshot_orientation_keys");
    for i in 0..SNAPSHOT_CARTRIDGES {
        let col = i % SNAPSHOT_COLS;
        let row = i / SNAPSHOT_COLS;
        let x = -snapshot_span_x() / 2.0 + col as f64 * SNAPSHOT_PITCH_X - 35.0;
        let y = -snapshot_span_y() / 2.0 + row as f64 * SNAPSHOT_PITCH_Y - 23.0;
        let key = centered_cube(
            format!("closed_alarm_evidence_station_snapshot_asymmetric_key_{i}"),
            18.0,
            10.0,
            10.0,
        )
        .translate(x, y, SNAPSHOT_Z / 2.0 + 5.0);
        keys = keys + key;
    }
    keys
}

fn flow_pressure_bubble_witness_coupons() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_flow_pressure_bubble_witness_coupon_base",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let flow = flow_witness_coupons();
    let pressure = pressure_witness_coupons();
    let bubble = bubble_witness_coupons();
    let lane_ribs = witness_lane_ribs();

    (base - flow - pressure - bubble + lane_ribs).translate(
        WITNESS_POS.0,
        WITNESS_POS.1,
        DECK_Z / 2.0 + WITNESS_Z / 2.0,
    )
}

fn flow_witness_coupons() -> Part {
    let mut coupons = Part::empty("closed_alarm_evidence_station_flow_witness_coupons");
    for i in 0..FLOW_COUPONS {
        let x = -145.0 + i as f64 * COUPON_PITCH_X;
        let coupon = centered_cube(
            format!("closed_alarm_evidence_station_flow_witness_coupon_{i}"),
            COUPON_X,
            COUPON_Y,
            13.0,
        )
        .translate(x, -56.0, WITNESS_Z / 2.0 - 4.0);
        let channel = centered_cylinder(
            format!("closed_alarm_evidence_station_flow_coupon_route_bore_{i}"),
            4.0,
            COUPON_X + 16.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -56.0, WITNESS_Z / 2.0 - 2.0);
        coupons = coupons + coupon + channel;
    }
    coupons
}

fn pressure_witness_coupons() -> Part {
    let mut coupons = Part::empty("closed_alarm_evidence_station_pressure_witness_coupons");
    for i in 0..PRESSURE_COUPONS {
        let x = -114.0 + i as f64 * 76.0;
        let pocket = centered_cylinder(
            format!("closed_alarm_evidence_station_pressure_witness_coupon_{i}"),
            PRESSURE_MEMBRANE_D / 2.0,
            16.0,
            36,
        )
        .translate(x, 0.0, WITNESS_Z / 2.0 - 3.0);
        let relief = centered_cube(
            format!("closed_alarm_evidence_station_pressure_coupon_relief_slot_{i}"),
            44.0,
            10.0,
            12.0,
        )
        .translate(x, 25.0, WITNESS_Z / 2.0 - 3.0);
        coupons = coupons + pocket + relief;
    }
    coupons
}

fn bubble_witness_coupons() -> Part {
    let mut coupons = Part::empty("closed_alarm_evidence_station_bubble_witness_coupons");
    for i in 0..BUBBLE_COUPONS {
        let x = -145.0 + i as f64 * COUPON_PITCH_X;
        let window = centered_cylinder(
            format!("closed_alarm_evidence_station_bubble_witness_coupon_{i}"),
            BUBBLE_WINDOW_D / 2.0,
            16.0,
            36,
        )
        .translate(x, 58.0, WITNESS_Z / 2.0 - 3.0);
        let backlight = centered_cube(
            format!("closed_alarm_evidence_station_bubble_witness_backlight_slot_{i}"),
            34.0,
            9.0,
            12.0,
        )
        .translate(x, 78.0, WITNESS_Z / 2.0 - 3.0);
        coupons = coupons + window + backlight;
    }
    coupons
}

fn witness_lane_ribs() -> Part {
    let mut ribs = Part::empty("closed_alarm_evidence_station_witness_lane_ribs");
    for (i, y) in [-28.0, 30.0].iter().enumerate() {
        let rib = centered_cube(
            format!("closed_alarm_evidence_station_witness_lane_separator_rib_{i}"),
            WITNESS_X - 48.0,
            7.0,
            13.0,
        )
        .translate(0.0, *y, WITNESS_Z / 2.0 + 6.5);
        ribs = ribs + rib;
    }
    ribs
}

fn sample_retain_split() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_sample_retain_split_base",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    );
    let wells = split_retain_wells();
    let channels = retain_split_channels();
    let seals = retain_split_seal_pads();

    (base - wells - channels + seals).translate(
        RETAIN_POS.0,
        RETAIN_POS.1,
        DECK_Z / 2.0 + RETAIN_Z / 2.0,
    )
}

fn split_retain_wells() -> Part {
    let mut wells = Part::empty("closed_alarm_evidence_station_split_retain_wells");
    for split in 0..RETAIN_SPLITS {
        let y = if split == 0 { -48.0 } else { 48.0 };
        for i in 0..RETAIN_WELLS_PER_SPLIT {
            let x = -retain_well_span() / 2.0 + i as f64 * RETAIN_WELL_PITCH;
            let well = centered_cylinder(
                format!("closed_alarm_evidence_station_sample_retain_split_{split}_well_{i}"),
                RETAIN_WELL_D / 2.0,
                20.0,
                36,
            )
            .translate(x, y, RETAIN_Z / 2.0 - 3.0);
            wells = wells + well;
        }
    }
    wells
}

fn retain_split_channels() -> Part {
    let inlet = centered_cylinder(
        "closed_alarm_evidence_station_retain_split_inlet_channel",
        SPLIT_CHANNEL_D / 2.0,
        RETAIN_X - 52.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, RETAIN_Z / 2.0 - 4.0);
    let branch_a = centered_cylinder(
        "closed_alarm_evidence_station_retain_split_branch_a_channel",
        SPLIT_CHANNEL_D / 2.0,
        112.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-122.0, -27.0, RETAIN_Z / 2.0 - 4.0);
    let branch_b = centered_cylinder(
        "closed_alarm_evidence_station_retain_split_branch_b_channel",
        SPLIT_CHANNEL_D / 2.0,
        112.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(122.0, 27.0, RETAIN_Z / 2.0 - 4.0);

    inlet + branch_a + branch_b
}

fn retain_split_seal_pads() -> Part {
    let mut pads = Part::empty("closed_alarm_evidence_station_retain_split_seal_pads");
    for i in 0..RETAIN_SEAL_PADS {
        let x = -142.5 + i as f64 * 57.0;
        let pad = centered_cube(
            format!("closed_alarm_evidence_station_retain_split_seal_pad_{i}"),
            38.0,
            14.0,
            4.0,
        )
        .translate(x, RETAIN_Y / 2.0 - 22.0, RETAIN_Z / 2.0 + 2.5);
        pads = pads + pad;
    }
    pads
}

fn cassette_quarantine_evidence_dock() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_cassette_quarantine_evidence_dock_base",
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    let cassette_pocket = centered_cube(
        "closed_alarm_evidence_station_cassette_evidence_dock_pocket",
        CASSETTE_POCKET_X,
        CASSETTE_POCKET_Y,
        18.0,
    )
    .translate(-62.0, 16.0, CASSETTE_Z / 2.0 - 5.0);
    let quarantine_pocket = centered_cube(
        "closed_alarm_evidence_station_quarantine_evidence_dock_pocket",
        QUARANTINE_POCKET_X,
        QUARANTINE_POCKET_Y,
        18.0,
    )
    .translate(128.0, 16.0, CASSETTE_Z / 2.0 - 5.0);
    let clamps = cassette_clamps();
    let flags = quarantine_flag_wells();
    let custody_fence = centered_cube(
        "closed_alarm_evidence_station_cassette_quarantine_custody_fence",
        CASSETTE_X - 42.0,
        10.0,
        24.0,
    )
    .translate(0.0, -CASSETTE_Y / 2.0 + 28.0, CASSETTE_Z / 2.0 + 12.0);

    (base - cassette_pocket - quarantine_pocket - flags + clamps + custody_fence).translate(
        CASSETTE_POS.0,
        CASSETTE_POS.1,
        DECK_Z / 2.0 + CASSETTE_Z / 2.0,
    )
}

fn cassette_clamps() -> Part {
    let mut clamps = Part::empty("closed_alarm_evidence_station_cassette_quarantine_clamps");
    for i in 0..CASSETTE_CLAMPS {
        let x = if i % 2 == 0 { -170.0 } else { 170.0 };
        let y = if i < 2 { -65.0 } else { 97.0 };
        let clamp = centered_cube(
            format!("closed_alarm_evidence_station_cassette_evidence_clamp_{i}"),
            36.0,
            18.0,
            18.0,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 9.0);
        clamps = clamps + clamp;
    }
    clamps
}

fn quarantine_flag_wells() -> Part {
    let mut wells = Part::empty("closed_alarm_evidence_station_quarantine_flag_wells");
    for i in 0..QUARANTINE_FLAG_WELLS {
        let x = -146.0 + i as f64 * 73.0;
        let well = centered_cylinder(
            format!("closed_alarm_evidence_station_quarantine_flag_well_{i}"),
            12.0,
            16.0,
            30,
        )
        .translate(x, -CASSETTE_Y / 2.0 + 58.0, CASSETTE_Z / 2.0 - 3.0);
        wells = wells + well;
    }
    wells
}

fn timeline_token_rail() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_timeline_token_rail_base",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let slots = timeline_token_slots();
    let lane_keys = timeline_lane_keys();
    let end_stops = timeline_end_stops();

    (base - slots + lane_keys + end_stops).translate(
        TOKEN_POS.0,
        TOKEN_POS.1,
        DECK_Z / 2.0 + TOKEN_Z / 2.0,
    )
}

fn timeline_token_slots() -> Part {
    let mut slots = Part::empty("closed_alarm_evidence_station_timeline_token_slots");
    for lane in 0..TOKEN_LANES {
        for event in 0..TIMELINE_EVENTS {
            let x = -timeline_span_x() / 2.0 + event as f64 * TOKEN_PITCH_X;
            let y = -TOKEN_LANE_PITCH + lane as f64 * TOKEN_LANE_PITCH;
            let slot = centered_cube(
                format!("closed_alarm_evidence_station_timeline_lane_{lane}_token_slot_{event}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                12.0,
            )
            .translate(x, y, TOKEN_Z / 2.0 - 4.0);
            slots = slots + slot;
        }
    }
    slots
}

fn timeline_lane_keys() -> Part {
    let mut keys = Part::empty("closed_alarm_evidence_station_timeline_lane_keys");
    for lane in 0..TOKEN_LANES {
        let y = -TOKEN_LANE_PITCH + lane as f64 * TOKEN_LANE_PITCH;
        let key = centered_cube(
            format!("closed_alarm_evidence_station_timeline_lane_{lane}_key"),
            TOKEN_X - 42.0,
            5.0,
            9.0,
        )
        .translate(0.0, y + 18.0, TOKEN_Z / 2.0 + 5.0);
        keys = keys + key;
    }
    keys
}

fn timeline_end_stops() -> Part {
    let left = centered_cube(
        "closed_alarm_evidence_station_timeline_left_end_stop",
        14.0,
        TOKEN_Y - 28.0,
        22.0,
    )
    .translate(-TOKEN_X / 2.0 + 16.0, 0.0, TOKEN_Z / 2.0 + 11.0);
    let right = centered_cube(
        "closed_alarm_evidence_station_timeline_right_end_stop",
        14.0,
        TOKEN_Y - 28.0,
        22.0,
    )
    .translate(TOKEN_X / 2.0 - 16.0, 0.0, TOKEN_Z / 2.0 + 11.0);

    left + right
}

fn barcode_rfid_custody_lands() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_barcode_rfid_custody_land_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let barcode = barcode_custody_lands();
    let rfid = rfid_custody_lands();
    let seals = custody_seal_pads();

    (base + barcode + rfid + seals).translate(
        CUSTODY_POS.0,
        CUSTODY_POS.1,
        DECK_Z / 2.0 + CUSTODY_Z / 2.0,
    )
}

fn barcode_custody_lands() -> Part {
    let mut lands = Part::empty("closed_alarm_evidence_station_barcode_custody_lands");
    for i in 0..BARCODE_LANDS {
        let x = -88.0 + (i % 3) as f64 * 88.0;
        let y = -44.0 + (i / 3) as f64 * 36.0;
        let land = centered_cube(
            format!("closed_alarm_evidence_station_barcode_custody_land_{i}"),
            68.0,
            22.0,
            2.5,
        )
        .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn rfid_custody_lands() -> Part {
    let mut lands = Part::empty("closed_alarm_evidence_station_rfid_custody_lands");
    for i in 0..RFID_LANDS {
        let x = -96.0 + i as f64 * 64.0;
        let land = centered_cylinder(
            format!("closed_alarm_evidence_station_rfid_custody_land_{i}"),
            15.0,
            2.5,
            32,
        )
        .translate(x, CUSTODY_Y / 2.0 - 26.0, CUSTODY_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn custody_seal_pads() -> Part {
    let mut pads = Part::empty("closed_alarm_evidence_station_custody_seal_pads");
    for i in 0..CUSTODY_SEAL_PADS {
        let x = if i % 2 == 0 { -126.0 } else { 126.0 };
        let y = if i < 2 { -58.0 } else { 58.0 };
        let pad = centered_cylinder(
            format!("closed_alarm_evidence_station_custody_seal_pad_{i}"),
            9.0,
            3.0,
            28,
        )
        .translate(x, y, CUSTODY_Z / 2.0 + 2.5);
        pads = pads + pad;
    }
    pads
}

fn high_low_fault_reference_pockets() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_high_low_fault_reference_pocket_base",
        FAULT_REF_X,
        FAULT_REF_Y,
        FAULT_REF_Z,
    );
    let pockets = high_low_fault_pockets();
    let label_lands = high_low_fault_label_lands();
    let gauge_stop = centered_cube(
        "closed_alarm_evidence_station_fault_reference_gauge_stop",
        FAULT_REF_X - 54.0,
        10.0,
        18.0,
    )
    .translate(0.0, 0.0, FAULT_REF_Z / 2.0 + 9.0);

    (base - pockets + label_lands + gauge_stop).translate(
        FAULT_REF_POS.0,
        FAULT_REF_POS.1,
        DECK_Z / 2.0 + FAULT_REF_Z / 2.0,
    )
}

fn high_low_fault_pockets() -> Part {
    let mut pockets = Part::empty("closed_alarm_evidence_station_high_low_fault_reference_pockets");
    for pair in 0..FAULT_REFERENCE_PAIRS {
        let x = -fault_reference_span() / 2.0 + pair as f64 * FAULT_PAIR_PITCH_X;
        let high = fault_reference_pocket("high", pair, x, 46.0);
        let low = fault_reference_pocket("low", pair, x, -46.0);
        pockets = pockets + high + low;
    }
    pockets
}

fn fault_reference_pocket(kind: &str, pair: usize, x: f64, y: f64) -> Part {
    centered_cylinder(
        format!("closed_alarm_evidence_station_{kind}_fault_reference_pocket_{pair}"),
        FAULT_POCKET_D / 2.0,
        18.0,
        34,
    )
    .translate(x, y, FAULT_REF_Z / 2.0 - 3.0)
}

fn high_low_fault_label_lands() -> Part {
    let mut lands = Part::empty("closed_alarm_evidence_station_high_low_fault_label_lands");
    for i in 0..HIGH_LOW_LABEL_LANDS {
        let pair = i / 2;
        let high = i % 2 == 0;
        let x = -fault_reference_span() / 2.0 + pair as f64 * FAULT_PAIR_PITCH_X;
        let y = if high { 74.0 } else { -74.0 };
        let land = centered_cube(
            format!(
                "closed_alarm_evidence_station_{}_fault_reference_label_land_{pair}",
                if high { "high" } else { "low" }
            ),
            42.0,
            12.0,
            3.0,
        )
        .translate(x, y, FAULT_REF_Z / 2.0 + 2.0);
        lands = lands + land;
    }
    lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_alarm_evidence_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let gates = disposition_gate_blades();
    let slots = disposition_token_slots();
    let latch_wells = disposition_latch_wells();

    (base - slots - latch_wells + gates).translate(
        GATE_POS.0,
        GATE_POS.1,
        DECK_Z / 2.0 + GATE_Z / 2.0,
    )
}

fn disposition_gate_blades() -> Part {
    let mut blades = Part::empty("closed_alarm_evidence_station_disposition_gate_blades");
    for i in 0..DISPOSITION_GATES {
        let x = -140.0 + i as f64 * 140.0;
        let name = disposition_gate_name(i);
        let blade = centered_cube(
            format!("closed_alarm_evidence_station_{name}_gate_blade"),
            70.0,
            GATE_Y + 24.0,
            16.0,
        )
        .translate(x, 0.0, GATE_Z / 2.0 + 8.0);
        blades = blades + blade;
    }
    blades
}

fn disposition_token_slots() -> Part {
    let mut slots = Part::empty("closed_alarm_evidence_station_disposition_token_slots");
    for gate in 0..DISPOSITION_GATES {
        for token in 0..GATE_TOKEN_SLOTS {
            let x = -170.0 + gate as f64 * 170.0 + (token % 2) as f64 * 34.0;
            let y = -22.0 + (token / 2) as f64 * 22.0;
            let slot = centered_cube(
                format!("closed_alarm_evidence_station_disposition_gate_{gate}_token_slot_{token}"),
                25.0,
                15.0,
                12.0,
            )
            .translate(x, y, GATE_Z / 2.0 - 4.0);
            slots = slots + slot;
        }
    }
    slots
}

fn disposition_latch_wells() -> Part {
    let mut wells = Part::empty("closed_alarm_evidence_station_disposition_latch_wells");
    for i in 0..DISPOSITION_GATES {
        let x = -140.0 + i as f64 * 140.0;
        let well = centered_cylinder(
            format!("closed_alarm_evidence_station_disposition_latch_well_{i}"),
            10.0,
            14.0,
            28,
        )
        .translate(x, -GATE_Y / 2.0 + 20.0, GATE_Z / 2.0 - 3.0);
        wells = wells + well;
    }
    wells
}

fn camera_evidence_bridge() -> Part {
    let left_post = camera_bridge_post(-CAMERA_BRIDGE_X / 2.0 + 34.0);
    let right_post = camera_bridge_post(CAMERA_BRIDGE_X / 2.0 - 34.0);
    let beam = centered_cube(
        "closed_alarm_evidence_station_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        38.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 19.0,
    );
    let mounts = camera_mounts();
    let fiducials = evidence_fiducials();
    let card_clamps = evidence_card_clamps();

    left_post + right_post + beam + mounts + fiducials + card_clamps
}

fn camera_bridge_post(x: f64) -> Part {
    centered_cube(
        format!("closed_alarm_evidence_station_camera_bridge_post_{x:.0}"),
        34.0,
        40.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_BRIDGE_POS.0 + x,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    )
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("closed_alarm_evidence_station_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        let x = -380.0 + i as f64 * 190.0;
        let mount = centered_cylinder(
            format!("closed_alarm_evidence_station_evidence_camera_mount_{i}"),
            18.0,
            10.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            CAMERA_BRIDGE_POS.0 + x,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 28.0,
        );
        mounts = mounts + mount;
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_alarm_evidence_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -500.0 + i as f64 * 111.0;
        let fid = centered_cylinder(
            format!("closed_alarm_evidence_station_evidence_fiducial_{i}"),
            7.0,
            2.5,
            28,
        )
        .translate(x, -DECK_Y / 2.0 + 58.0, DECK_Z / 2.0 + 2.0);
        fiducials = fiducials + fid;
    }
    fiducials
}

fn evidence_card_clamps() -> Part {
    let mut clamps = Part::empty("closed_alarm_evidence_station_evidence_card_clamps");
    for i in 0..EVIDENCE_CARD_CLAMPS {
        let x = -225.0 + i as f64 * 90.0;
        let clamp = centered_cube(
            format!("closed_alarm_evidence_station_evidence_card_clamp_{i}"),
            40.0,
            12.0,
            15.0,
        )
        .translate(x, -DECK_Y / 2.0 + 86.0, DECK_Z / 2.0 + 7.5);
        clamps = clamps + clamp;
    }
    clamps
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_alarm_evidence_station_front_robot_keepout",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let rear_robot = centered_cube(
        "closed_alarm_evidence_station_rear_robot_keepout",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + ROBOT_KEEPOUT_Y / 2.0,
        ROBOT_KEEPOUT_Z / 2.0,
    );
    let left_service = centered_cube(
        "closed_alarm_evidence_station_left_service_keepout",
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
        "closed_alarm_evidence_station_right_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top_clearance = centered_cube(
        "closed_alarm_evidence_station_top_service_clearance_keepout",
        DECK_X - 150.0,
        DECK_Y - 160.0,
        28.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    front_robot + rear_robot + left_service + right_service + top_clearance
}

fn station_assembly() -> Part {
    containment_deck()
        + event_logger_dock()
        + sensor_snapshot_cartridge_nests()
        + flow_pressure_bubble_witness_coupons()
        + sample_retain_split()
        + cassette_quarantine_evidence_dock()
        + timeline_token_rail()
        + barcode_rfid_custody_lands()
        + high_low_fault_reference_pockets()
        + release_hold_reject_gates()
        + camera_evidence_bridge()
        + robot_service_keepouts()
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13, "unexpected STL output count");
    assert_eq!(DATUM_BOSSES, 10, "datum boss table changed");
    assert!(
        snapshot_span_x() < SNAPSHOT_X - 120.0,
        "snapshot cartridges exceed nest width"
    );
    assert!(
        timeline_span_x() < TOKEN_X - 54.0,
        "timeline token slots exceed rail width"
    );
    assert!(
        retain_well_span() < RETAIN_X - 120.0,
        "retain wells exceed split bank width"
    );
    assert!(
        fault_reference_span() < FAULT_REF_X - 130.0,
        "fault references exceed pocket bank width"
    );
    assert!(
        top_bridge_clearance() >= 165.0,
        "camera/evidence bridge does not clear module stack"
    );
    assert!(
        module_extents_fit_inner_deck(),
        "module envelopes exceed containment rim clearances"
    );
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn snapshot_span_x() -> f64 {
    (SNAPSHOT_COLS as f64 - 1.0) * SNAPSHOT_PITCH_X + SNAPSHOT_SLOT_X
}

fn snapshot_span_y() -> f64 {
    (SNAPSHOT_ROWS as f64 - 1.0) * SNAPSHOT_PITCH_Y + SNAPSHOT_SLOT_Y
}

fn timeline_span_x() -> f64 {
    (TIMELINE_EVENTS as f64 - 1.0) * TOKEN_PITCH_X + TOKEN_SLOT_X
}

fn retain_well_span() -> f64 {
    (RETAIN_WELLS_PER_SPLIT as f64 - 1.0) * RETAIN_WELL_PITCH + RETAIN_WELL_D
}

fn fault_reference_span() -> f64 {
    (FAULT_REFERENCE_PAIRS as f64 - 1.0) * FAULT_PAIR_PITCH_X + FAULT_POCKET_D
}

fn top_bridge_clearance() -> f64 {
    CAMERA_BRIDGE_Z
        - CASSETTE_Z
            .max(SNAPSHOT_Z)
            .max(RETAIN_Z)
            .max(WITNESS_Z)
            .max(FAULT_REF_Z)
}

fn module_extents_fit_inner_deck() -> bool {
    module_fits(LOGGER_POS, LOGGER_X, LOGGER_Y)
        && module_fits(SNAPSHOT_POS, SNAPSHOT_X, SNAPSHOT_Y)
        && module_fits(WITNESS_POS, WITNESS_X, WITNESS_Y)
        && module_fits(RETAIN_POS, RETAIN_X, RETAIN_Y)
        && module_fits(CASSETTE_POS, CASSETTE_X, CASSETTE_Y)
        && module_fits(TOKEN_POS, TOKEN_X, TOKEN_Y)
        && module_fits(CUSTODY_POS, CUSTODY_X, CUSTODY_Y)
        && module_fits(FAULT_REF_POS, FAULT_REF_X, FAULT_REF_Y)
        && module_fits(GATE_POS, GATE_X, GATE_Y)
}

fn module_fits(pos: (f64, f64), x: f64, y: f64) -> bool {
    let inner_x = DECK_X / 2.0 - RIM_W - 8.0;
    let inner_y = DECK_Y / 2.0 - RIM_W - 8.0;
    pos.0.abs() + x / 2.0 <= inner_x && pos.1.abs() + y / 2.0 <= inner_y
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_expected_prefixed_stls() {
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_perfusion_alarm_root_cause_evidence_bundle_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_design_intent_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 23);
        for feature in [
            "event_logger_dock",
            "sensor_snapshot_cartridge_nests",
            "flow_witness_coupon",
            "pressure_witness_coupon",
            "bubble_witness_coupon",
            "sample_retain_split",
            "cassette_evidence_dock",
            "quarantine_evidence_dock",
            "timeline_token_rail",
            "barcode_custody_lands",
            "rfid_custody_lands",
            "high_fault_reference_pockets",
            "low_fault_reference_pockets",
            "release_gate",
            "hold_gate",
            "reject_gate",
            "camera_evidence_bridge",
            "robot_keepout",
            "service_keepout",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn helper_dimensions_preserve_layout_clearances() {
        assert_layout();
        assert!(snapshot_span_x() < SNAPSHOT_X - 120.0);
        assert!(timeline_span_x() < TOKEN_X - 54.0);
        assert!(retain_well_span() < RETAIN_X - 120.0);
        assert!(fault_reference_span() < FAULT_REF_X - 130.0);
        assert!(top_bridge_clearance() >= 165.0);
        assert!(module_extents_fit_inner_deck());
    }

    #[test]
    fn feature_counts_match_evidence_bundle_packaging() {
        assert_eq!(EVENT_LOGGER_BAYS, 4);
        assert_eq!(SNAPSHOT_CARTRIDGES, 8);
        assert_eq!(FLOW_COUPONS + PRESSURE_COUPONS + BUBBLE_COUPONS, 16);
        assert_eq!(RETAIN_SPLITS * RETAIN_WELLS_PER_SPLIT, 10);
        assert_eq!(TIMELINE_EVENTS * TOKEN_LANES, 27);
        assert_eq!(BARCODE_LANDS + RFID_LANDS, 10);
        assert_eq!(FAULT_REFERENCE_PAIRS * 2, HIGH_LOW_LABEL_LANDS);
        assert_eq!(DISPOSITION_GATES, 3);
    }

    #[test]
    fn disposition_gate_names_are_stable() {
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }

    #[test]
    fn scope_excludes_process_and_biological_claims() {
        assert!(DESIGN_SCOPE.contains("mechanical validation packaging only"));
        assert!(DESIGN_SCOPE.contains("not an alarm SOP"));
        assert!(DESIGN_SCOPE.contains("sterile-process claim"));
        assert!(DESIGN_SCOPE.contains("biological acceptance criterion"));
    }
}
