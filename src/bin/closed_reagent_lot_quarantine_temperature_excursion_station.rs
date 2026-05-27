use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent lot quarantine, temperature excursion, and release decision station.
//
// Intent:
// - Receive incoming media/additive reagent lots into physically segregated chilled
//   quarantine nests while maintaining closed custody interfaces.
// - Preserve temperature logger, barcode/RFID, excursion evidence, sealed sampling,
//   and release/hold/reject disposition geometry as inspectable datum features.
// - Represent leak containment, thaw/equilibration timing features, and robot gripper
//   clearance gauges for validation CAD review.
//
// Inspection/validation CAD only. This is not a biological protocol, acceptance
// threshold, release rule, sterility claim, or operating procedure.

const OUTPUTS: [&str; 12] = [
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_leak_tray_base.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_chilled_quarantine_nest_banks.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_temperature_logger_docks.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_excursion_evidence_lane.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_release_hold_reject_lanes.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_sealed_sampling_split_point.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_barcode_rfid_custody_plate.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_thaw_equilibration_timer_features.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_closed_transfer_gate.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_robotic_gripper_clearance_gauges.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_service_keepout_frame.stl",
    "output/closed_reagent_lot_quarantine_temperature_excursion_station_assembly.stl",
];

const DECK_X: f64 = 1680.0;
const DECK_Y: f64 = 1080.0;
const DECK_Z: f64 = 22.0;
const TRAY_CURB_W: f64 = 20.0;
const TRAY_CURB_Z: f64 = 44.0;
const LEAK_SUMP_X: f64 = 1500.0;
const LEAK_SUMP_Y: f64 = 900.0;
const LEAK_SUMP_DEPTH: f64 = 8.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_BOSSES: usize = 8;
const MOUNT_BOSS_D: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.4;

const BANK_COUNT: usize = 3;
const SLOTS_PER_BANK: usize = 4;
const QUARANTINE_SLOT_COUNT: usize = BANK_COUNT * SLOTS_PER_BANK;
const BANK_PLATE_X: f64 = 980.0;
const BANK_PLATE_Y: f64 = 336.0;
const BANK_PLATE_Z: f64 = 46.0;
const BANK_CENTER_X: f64 = -250.0;
const BANK_CENTER_Y: f64 = 245.0;
const BANK_PITCH_X: f64 = 292.0;
const REAGENT_NEST_X: f64 = 160.0;
const REAGENT_NEST_Y: f64 = 92.0;
const REAGENT_NEST_DEPTH: f64 = 18.0;
const BANK_WALL_W: f64 = 12.0;
const COLD_REFERENCE_PUCKS: usize = 6;

const LOGGER_DOCKS: usize = 6;
const LOGGER_PANEL_X: f64 = 520.0;
const LOGGER_PANEL_Y: f64 = 176.0;
const LOGGER_PANEL_Z: f64 = 36.0;
const LOGGER_CENTER_X: f64 = -500.0;
const LOGGER_CENTER_Y: f64 = -285.0;
const LOGGER_DOCK_X: f64 = 62.0;
const LOGGER_DOCK_Y: f64 = 92.0;
const LOGGER_DOCK_DEPTH: f64 = 10.0;
const LOGGER_PITCH_X: f64 = 76.0;
const LOGGER_POGO_PINS: usize = 18;

const EVIDENCE_LANE_X: f64 = 1030.0;
const EVIDENCE_LANE_Y: f64 = 166.0;
const EVIDENCE_LANE_Z: f64 = 28.0;
const EVIDENCE_CENTER_X: f64 = 80.0;
const EVIDENCE_CENTER_Y: f64 = -356.0;
const EVENT_CARD_POCKETS: usize = 8;
const TAMPER_SEAL_WELLS: usize = 12;
const EVIDENCE_CAMERA_PODS: usize = 4;

const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;
const DISPOSITION_SLOT_COUNT: usize = DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE;
const DISPOSITION_PANEL_X: f64 = 520.0;
const DISPOSITION_PANEL_Y: f64 = 372.0;
const DISPOSITION_PANEL_Z: f64 = 38.0;
const DISPOSITION_CENTER_X: f64 = 462.0;
const DISPOSITION_CENTER_Y: f64 = 176.0;
const DISPOSITION_LANE_PITCH_X: f64 = 154.0;
const DISPOSITION_SLOT_X: f64 = 96.0;
const DISPOSITION_SLOT_Y: f64 = 56.0;
const DISPOSITION_SLOT_DEPTH: f64 = 8.0;
const DISPOSITION_MIN_GAP: f64 = 42.0;

const SAMPLING_PANEL_X: f64 = 300.0;
const SAMPLING_PANEL_Y: f64 = 252.0;
const SAMPLING_PANEL_Z: f64 = 48.0;
const SAMPLING_CENTER_X: f64 = -650.0;
const SAMPLING_CENTER_Y: f64 = 90.0;
const SPLIT_VALVE_D: f64 = 72.0;
const SEALED_SAMPLE_VIAL_WELLS: usize = 8;
const CLOSED_CONNECTOR_PORTS: usize = 6;
const CLOSED_CONNECTOR_PITCH_X: f64 = 38.0;

const CUSTODY_PLATE_X: f64 = 430.0;
const CUSTODY_PLATE_Y: f64 = 168.0;
const CUSTODY_PLATE_Z: f64 = 18.0;
const CUSTODY_CENTER_X: f64 = 520.0;
const CUSTODY_CENTER_Y: f64 = 398.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 6;
const CUSTODY_TOKEN_WELLS: usize = 9;

const TIMER_PANEL_X: f64 = 440.0;
const TIMER_PANEL_Y: f64 = 168.0;
const TIMER_PANEL_Z: f64 = 32.0;
const TIMER_CENTER_X: f64 = 25.0;
const TIMER_CENTER_Y: f64 = 398.0;
const TIMER_DIALS: usize = 4;
const TIMER_TOKEN_SLOTS: usize = 8;
const EQUILIBRATION_LANES: usize = 4;

const TRANSFER_GATE_X: f64 = 900.0;
const TRANSFER_GATE_Y: f64 = 34.0;
const TRANSFER_GATE_Z: f64 = 188.0;
const TRANSFER_GATE_CENTER_Y: f64 = -20.0;
const PASS_THROUGH_WINDOW_X: f64 = 310.0;
const PASS_THROUGH_WINDOW_Z: f64 = 84.0;
const GATE_LATCHES: usize = 8;

const GRIPPER_GAUGE_COUNT: usize = 10;
const GRIPPER_PAD_X: f64 = 72.0;
const GRIPPER_PAD_Y: f64 = 32.0;
const GRIPPER_PAD_Z: f64 = 18.0;
const GRIPPER_CLEARANCE_Z: f64 = 148.0;

const FRONT_ROBOT_CLEARANCE_Y: f64 = 440.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 300.0;
const LEFT_SERVICE_CLEARANCE_X: f64 = 260.0;
const RIGHT_DECISION_CLEARANCE_X: f64 = 280.0;
const TOP_CAMERA_CLEARANCE_Z: f64 = 360.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let nests = chilled_quarantine_nest_banks();
    export(OUTPUTS[1], &nests);

    let loggers = temperature_logger_docks();
    export(OUTPUTS[2], &loggers);

    let evidence = excursion_evidence_lane();
    export(OUTPUTS[3], &evidence);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[4], &disposition);

    let sampling = sealed_sampling_split_point();
    export(OUTPUTS[5], &sampling);

    let custody = barcode_rfid_custody_plate();
    export(OUTPUTS[6], &custody);

    let timers = thaw_equilibration_timer_features();
    export(OUTPUTS[7], &timers);

    let gate = closed_transfer_gate();
    export(OUTPUTS[8], &gate);

    let gripper = robotic_gripper_clearance_gauges();
    export(OUTPUTS[9], &gripper);

    let keepouts = service_keepout_frame();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + nests
        + loggers
        + evidence
        + disposition
        + sampling
        + custody
        + timers
        + gate
        + gripper
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed reagent lot quarantine, temperature excursion, and release decision station:");
    println!(
        "  Leak tray:                  {DECK_X:.0}mm x {DECK_Y:.0}mm deck, {LEAK_SUMP_X:.0}mm x {LEAK_SUMP_Y:.0}mm recessed sump, {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Chilled quarantine nests:   {BANK_COUNT} banks x {SLOTS_PER_BANK} closed-lot nests, {COLD_REFERENCE_PUCKS} cold reference puck pockets"
    );
    println!(
        "  Temperature logger docks:   {LOGGER_DOCKS} logger cradles with {LOGGER_POGO_PINS} pogo/read pins and cable custody channels"
    );
    println!(
        "  Excursion evidence lane:    {EVENT_CARD_POCKETS} event-card pockets, {TAMPER_SEAL_WELLS} tamper seal wells, {EVIDENCE_CAMERA_PODS} camera pods"
    );
    println!(
        "  Disposition lanes:          release/hold/reject, {DISPOSITION_SLOTS_PER_LANE} closed reagent positions per lane"
    );
    println!(
        "  Sealed sampling split:      {CLOSED_CONNECTOR_PORTS} closed connector ports, {SEALED_SAMPLE_VIAL_WELLS} sealed sample vial wells"
    );
    println!(
        "  Custody plate:              {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {CUSTODY_TOKEN_WELLS} custody token wells"
    );
    println!(
        "  Thaw/equilibration timing:  {TIMER_DIALS} timer dials, {TIMER_TOKEN_SLOTS} time-token slots, {EQUILIBRATION_LANES} equilibration lanes"
    );
    println!(
        "  Robot/service clearances:   front robot {FRONT_ROBOT_CLEARANCE_Y:.0}mm, rear service {REAR_SERVICE_CLEARANCE_Y:.0}mm, top camera {TOP_CAMERA_CLEARANCE_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(QUARANTINE_SLOT_COUNT, 12);
    assert_eq!(DISPOSITION_SLOT_COUNT, 12);
    assert_eq!(required_feature_count(), 11);
    assert!(component_specs().iter().all(|spec| fits_on_deck(
        spec.center_x,
        spec.center_y,
        spec.x,
        spec.y,
        28.0
    )));
    assert!(bank_span_x() < BANK_PLATE_X - 90.0);
    assert!(logger_span_x() < LOGGER_PANEL_X - 70.0);
    assert!(disposition_lane_gap() >= DISPOSITION_MIN_GAP);
    assert!(closed_connector_span_x() < SAMPLING_PANEL_X - 70.0);
    assert!(timer_span_x() < TIMER_PANEL_X - 80.0);
    assert!(TRANSFER_GATE_Z > GRIPPER_CLEARANCE_Z);
    assert!(TOP_CAMERA_CLEARANCE_Z > TRANSFER_GATE_Z + 120.0);
}

fn leak_tray_base() -> Part {
    let deck = centered_cube("reagent_lot_station_leak_tray_deck", DECK_X, DECK_Y, DECK_Z)
        .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        "reagent_lot_station_recessed_leak_sump",
        LEAK_SUMP_X,
        LEAK_SUMP_Y,
        LEAK_SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -18.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0 + 0.5);
    let drain_channel = centered_cube(
        "reagent_lot_station_front_drain_channel",
        LEAK_SUMP_X - 92.0,
        DRAIN_CHANNEL_W,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 72.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0);
    let drain_port = centered_cylinder(
        "reagent_lot_station_bulkhead_drain_port",
        DRAIN_PORT_D / 2.0,
        TRAY_CURB_W + 24.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 94.0, -DECK_Y / 2.0 + 28.0, DECK_Z - 9.0);

    deck - sump - drain_channel - drain_port + tray_curbs() + mount_bosses() + deck_zone_rails()
}

fn tray_curbs() -> Part {
    let front = centered_cube(
        "reagent_lot_station_front_spill_curb",
        DECK_X,
        TRAY_CURB_W,
        TRAY_CURB_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let rear = centered_cube(
        "reagent_lot_station_rear_spill_curb",
        DECK_X,
        TRAY_CURB_W,
        TRAY_CURB_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let left = centered_cube(
        "reagent_lot_station_left_spill_curb",
        TRAY_CURB_W,
        DECK_Y,
        TRAY_CURB_Z,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_CURB_W / 2.0,
        0.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "reagent_lot_station_right_spill_curb",
        TRAY_CURB_W,
        DECK_Y,
        TRAY_CURB_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_CURB_W / 2.0,
        0.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty("reagent_lot_station_mount_bosses");
    for i in 0..MOUNT_BOSSES {
        let x = if i % 2 == 0 {
            -DECK_X / 2.0 + 86.0
        } else {
            DECK_X / 2.0 - 86.0
        };
        let y_index = i / 2;
        let y = -DECK_Y / 2.0 + 140.0 + y_index as f64 * ((DECK_Y - 280.0) / 3.0);
        let boss = centered_cylinder(
            format!("reagent_lot_station_mount_boss_{i}"),
            MOUNT_BOSS_D / 2.0,
            10.0,
            36,
        )
        .translate(x, y, DECK_Z + 5.0);
        let hole = centered_cylinder(
            format!("reagent_lot_station_mount_hole_{i}"),
            MOUNT_HOLE_D / 2.0,
            14.0,
            28,
        )
        .translate(x, y, DECK_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn deck_zone_rails() -> Part {
    let mut rails = Part::empty("reagent_lot_station_zone_boundary_rails");
    for (i, y) in [-148.0, 34.0, 332.0].iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("reagent_lot_station_zone_boundary_rail_{i}"),
                DECK_X - 180.0,
                8.0,
                10.0,
            )
            .translate(0.0, *y, DECK_Z + 5.0);
    }
    rails
}

fn chilled_quarantine_nest_banks() -> Part {
    let plate = centered_cube(
        "reagent_lot_station_chilled_quarantine_bank_plate",
        BANK_PLATE_X,
        BANK_PLATE_Y,
        BANK_PLATE_Z,
    )
    .translate(BANK_CENTER_X, BANK_CENTER_Y, DECK_Z + BANK_PLATE_Z / 2.0);
    let mut slot_cuts = Part::empty("reagent_lot_station_quarantine_nest_slot_cuts");
    let mut rims = Part::empty("reagent_lot_station_quarantine_nest_rims");
    for bank in 0..BANK_COUNT {
        for slot in 0..SLOTS_PER_BANK {
            let (x, y) = quarantine_slot_xy(bank, slot);
            slot_cuts = slot_cuts
                + centered_cube(
                    format!("reagent_lot_station_quarantine_slot_cut_b{bank}_s{slot}"),
                    REAGENT_NEST_X,
                    REAGENT_NEST_Y,
                    REAGENT_NEST_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    DECK_Z + BANK_PLATE_Z - REAGENT_NEST_DEPTH / 2.0 + 0.5,
                );
            rims = rims
                + nest_rim(
                    format!("reagent_lot_station_quarantine_slot_rim_b{bank}_s{slot}"),
                    x,
                    y,
                );
        }
    }

    (plate - slot_cuts) + rims + bank_separation_walls() + cold_reference_puck_pockets()
}

fn nest_rim(name: impl Into<String>, x: f64, y: f64) -> Part {
    let name = name.into();
    let outer = centered_cube(
        format!("{name}_outer"),
        REAGENT_NEST_X + 18.0,
        REAGENT_NEST_Y + 18.0,
        12.0,
    )
    .translate(x, y, DECK_Z + BANK_PLATE_Z + 6.0);
    let inner = centered_cube(
        format!("{name}_inner"),
        REAGENT_NEST_X + 2.0,
        REAGENT_NEST_Y + 2.0,
        14.0,
    )
    .translate(x, y, DECK_Z + BANK_PLATE_Z + 6.0);
    outer - inner
}

fn bank_separation_walls() -> Part {
    let mut walls = Part::empty("reagent_lot_station_bank_separation_walls");
    for i in 0..=BANK_COUNT {
        let x = BANK_CENTER_X - bank_span_x() / 2.0 - BANK_WALL_W / 2.0 + i as f64 * BANK_PITCH_X;
        walls = walls
            + centered_cube(
                format!("reagent_lot_station_bank_separation_wall_{i}"),
                BANK_WALL_W,
                BANK_PLATE_Y - 36.0,
                64.0,
            )
            .translate(x, BANK_CENTER_Y, DECK_Z + BANK_PLATE_Z + 32.0);
    }
    walls
}

fn cold_reference_puck_pockets() -> Part {
    let mut pucks = Part::empty("reagent_lot_station_cold_reference_puck_pockets");
    for i in 0..COLD_REFERENCE_PUCKS {
        let x = BANK_CENTER_X - 390.0 + i as f64 * 156.0;
        let pocket = centered_cylinder(
            format!("reagent_lot_station_cold_reference_puck_cut_{i}"),
            18.0,
            12.0,
            36,
        )
        .translate(x, BANK_CENTER_Y - 142.0, DECK_Z + BANK_PLATE_Z + 2.0);
        let collar = centered_cylinder(
            format!("reagent_lot_station_cold_reference_puck_collar_{i}"),
            24.0,
            8.0,
            36,
        )
        .translate(x, BANK_CENTER_Y - 142.0, DECK_Z + BANK_PLATE_Z + 4.0);
        pucks = pucks + (collar - pocket);
    }
    pucks
}

fn temperature_logger_docks() -> Part {
    let panel = centered_cube(
        "reagent_lot_station_temperature_logger_dock_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(
        LOGGER_CENTER_X,
        LOGGER_CENTER_Y,
        DECK_Z + LOGGER_PANEL_Z / 2.0,
    );
    let mut cuts = Part::empty("reagent_lot_station_temperature_logger_dock_cuts");
    let mut retainers = Part::empty("reagent_lot_station_temperature_logger_retainers");
    for i in 0..LOGGER_DOCKS {
        let x = logger_dock_x(i);
        cuts = cuts
            + centered_cube(
                format!("reagent_lot_station_temperature_logger_dock_cut_{i}"),
                LOGGER_DOCK_X,
                LOGGER_DOCK_Y,
                LOGGER_DOCK_DEPTH + 1.0,
            )
            .translate(
                x,
                LOGGER_CENTER_Y,
                DECK_Z + LOGGER_PANEL_Z - LOGGER_DOCK_DEPTH / 2.0,
            );
        retainers = retainers
            + centered_cube(
                format!("reagent_lot_station_temperature_logger_retainer_bridge_{i}"),
                LOGGER_DOCK_X + 16.0,
                9.0,
                18.0,
            )
            .translate(
                x,
                LOGGER_CENTER_Y + LOGGER_DOCK_Y / 2.0 + 10.0,
                DECK_Z + LOGGER_PANEL_Z + 9.0,
            );
    }

    (panel - cuts) + retainers + logger_pogo_pin_array() + logger_cable_channels()
}

fn logger_pogo_pin_array() -> Part {
    let mut pins = Part::empty("reagent_lot_station_logger_pogo_pin_array");
    for i in 0..LOGGER_POGO_PINS {
        let dock = i / 3;
        let pin = i % 3;
        pins = pins
            + centered_cylinder(
                format!("reagent_lot_station_logger_pogo_pin_{i}"),
                2.2,
                8.0,
                18,
            )
            .translate(
                logger_dock_x(dock) - 14.0 + pin as f64 * 14.0,
                LOGGER_CENTER_Y - LOGGER_DOCK_Y / 2.0 + 14.0,
                DECK_Z + LOGGER_PANEL_Z + 4.0,
            );
    }
    pins
}

fn logger_cable_channels() -> Part {
    let mut channels = Part::empty("reagent_lot_station_logger_cable_custody_channels");
    for i in 0..LOGGER_DOCKS {
        channels = channels
            + centered_cube(
                format!("reagent_lot_station_logger_cable_channel_{i}"),
                10.0,
                62.0,
                8.0,
            )
            .translate(
                logger_dock_x(i),
                LOGGER_CENTER_Y - LOGGER_DOCK_Y / 2.0 - 34.0,
                DECK_Z + LOGGER_PANEL_Z + 4.0,
            );
    }
    channels
}

fn excursion_evidence_lane() -> Part {
    let lane = centered_cube(
        "reagent_lot_station_excursion_evidence_lane_base",
        EVIDENCE_LANE_X,
        EVIDENCE_LANE_Y,
        EVIDENCE_LANE_Z,
    )
    .translate(
        EVIDENCE_CENTER_X,
        EVIDENCE_CENTER_Y,
        DECK_Z + EVIDENCE_LANE_Z / 2.0,
    );
    let mut card_cuts = Part::empty("reagent_lot_station_excursion_event_card_cuts");
    for i in 0..EVENT_CARD_POCKETS {
        card_cuts = card_cuts
            + centered_cube(
                format!("reagent_lot_station_excursion_event_card_pocket_{i}"),
                92.0,
                48.0,
                7.0,
            )
            .translate(
                evidence_card_x(i),
                EVIDENCE_CENTER_Y - 22.0,
                DECK_Z + EVIDENCE_LANE_Z - 3.0,
            );
    }
    (lane - card_cuts) + tamper_seal_well_collars() + evidence_camera_bridge()
}

fn tamper_seal_well_collars() -> Part {
    let mut wells = Part::empty("reagent_lot_station_tamper_seal_well_collars");
    for i in 0..TAMPER_SEAL_WELLS {
        let x = EVIDENCE_CENTER_X - 450.0 + i as f64 * (900.0 / (TAMPER_SEAL_WELLS as f64 - 1.0));
        let collar = centered_cylinder(
            format!("reagent_lot_station_tamper_seal_well_collar_{i}"),
            14.0,
            7.0,
            28,
        )
        .translate(x, EVIDENCE_CENTER_Y + 54.0, DECK_Z + EVIDENCE_LANE_Z + 3.5);
        let bore = centered_cylinder(
            format!("reagent_lot_station_tamper_seal_well_bore_{i}"),
            8.0,
            9.0,
            24,
        )
        .translate(x, EVIDENCE_CENTER_Y + 54.0, DECK_Z + EVIDENCE_LANE_Z + 3.5);
        wells = wells + (collar - bore);
    }
    wells
}

fn evidence_camera_bridge() -> Part {
    let beam = centered_cube(
        "reagent_lot_station_excursion_evidence_camera_bridge",
        EVIDENCE_LANE_X - 90.0,
        26.0,
        28.0,
    )
    .translate(EVIDENCE_CENTER_X, EVIDENCE_CENTER_Y, DECK_Z + 176.0);
    let mut pods = Part::empty("reagent_lot_station_excursion_evidence_camera_pods");
    for i in 0..EVIDENCE_CAMERA_PODS {
        pods = pods
            + centered_cube(
                format!("reagent_lot_station_excursion_evidence_camera_pod_{i}"),
                64.0,
                58.0,
                34.0,
            )
            .translate(
                EVIDENCE_CENTER_X - 360.0 + i as f64 * 240.0,
                EVIDENCE_CENTER_Y,
                DECK_Z + 142.0,
            );
    }
    let left_post = centered_cube(
        "reagent_lot_station_excursion_evidence_bridge_left_post",
        28.0,
        34.0,
        160.0,
    )
    .translate(
        EVIDENCE_CENTER_X - EVIDENCE_LANE_X / 2.0 + 58.0,
        EVIDENCE_CENTER_Y,
        DECK_Z + 96.0,
    );
    let right_post = centered_cube(
        "reagent_lot_station_excursion_evidence_bridge_right_post",
        28.0,
        34.0,
        160.0,
    )
    .translate(
        EVIDENCE_CENTER_X + EVIDENCE_LANE_X / 2.0 - 58.0,
        EVIDENCE_CENTER_Y,
        DECK_Z + 96.0,
    );
    beam + pods + left_post + right_post
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "reagent_lot_station_release_hold_reject_lane_panel",
        DISPOSITION_PANEL_X,
        DISPOSITION_PANEL_Y,
        DISPOSITION_PANEL_Z,
    )
    .translate(
        DISPOSITION_CENTER_X,
        DISPOSITION_CENTER_Y,
        DECK_Z + DISPOSITION_PANEL_Z / 2.0,
    );
    let mut pockets = Part::empty("reagent_lot_station_disposition_slot_pockets");
    let mut flags = Part::empty("reagent_lot_station_disposition_lane_status_flags");
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            let x = disposition_lane_x(lane);
            let y = disposition_slot_y(slot);
            pockets = pockets
                + centered_cube(
                    format!("reagent_lot_station_disposition_slot_l{lane}_s{slot}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    DISPOSITION_SLOT_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    DECK_Z + DISPOSITION_PANEL_Z - DISPOSITION_SLOT_DEPTH / 2.0,
                );
        }
        flags = flags
            + centered_cube(
                format!("reagent_lot_station_disposition_status_flag_lane_{lane}"),
                DISPOSITION_SLOT_X + 12.0,
                8.0,
                42.0,
            )
            .translate(
                disposition_lane_x(lane),
                DISPOSITION_CENTER_Y + DISPOSITION_PANEL_Y / 2.0 - 24.0,
                DECK_Z + DISPOSITION_PANEL_Z + 21.0,
            );
    }

    (panel - pockets) + flags + disposition_dividers() + disposition_end_stops()
}

fn disposition_dividers() -> Part {
    let mut dividers = Part::empty("reagent_lot_station_disposition_hard_dividers");
    for i in 0..=DISPOSITION_LANES {
        let x = DISPOSITION_CENTER_X - disposition_total_span_x() / 2.0 - 8.0
            + i as f64 * DISPOSITION_LANE_PITCH_X;
        dividers = dividers
            + centered_cube(
                format!("reagent_lot_station_disposition_divider_{i}"),
                14.0,
                DISPOSITION_PANEL_Y - 58.0,
                62.0,
            )
            .translate(
                x,
                DISPOSITION_CENTER_Y - 10.0,
                DECK_Z + DISPOSITION_PANEL_Z + 31.0,
            );
    }
    dividers
}

fn disposition_end_stops() -> Part {
    let mut stops = Part::empty("reagent_lot_station_disposition_end_stops");
    for lane in 0..DISPOSITION_LANES {
        stops = stops
            + centered_cube(
                format!("reagent_lot_station_disposition_end_stop_{lane}"),
                DISPOSITION_SLOT_X + 18.0,
                12.0,
                36.0,
            )
            .translate(
                disposition_lane_x(lane),
                DISPOSITION_CENTER_Y - DISPOSITION_PANEL_Y / 2.0 + 22.0,
                DECK_Z + DISPOSITION_PANEL_Z + 18.0,
            );
    }
    stops
}

fn sealed_sampling_split_point() -> Part {
    let panel = centered_cube(
        "reagent_lot_station_sealed_sampling_split_panel",
        SAMPLING_PANEL_X,
        SAMPLING_PANEL_Y,
        SAMPLING_PANEL_Z,
    )
    .translate(
        SAMPLING_CENTER_X,
        SAMPLING_CENTER_Y,
        DECK_Z + SAMPLING_PANEL_Z / 2.0,
    );
    let valve_body = centered_cylinder(
        "reagent_lot_station_closed_sampling_split_valve_body",
        SPLIT_VALVE_D / 2.0,
        36.0,
        48,
    )
    .translate(
        SAMPLING_CENTER_X,
        SAMPLING_CENTER_Y + 30.0,
        DECK_Z + SAMPLING_PANEL_Z + 18.0,
    );

    panel + valve_body + closed_connector_ports() + sealed_sample_vial_wells() + sample_line_comb()
}

fn closed_connector_ports() -> Part {
    let mut ports = Part::empty("reagent_lot_station_closed_sampling_connector_ports");
    for i in 0..CLOSED_CONNECTOR_PORTS {
        let collar = centered_cylinder(
            format!("reagent_lot_station_closed_sampling_connector_collar_{i}"),
            18.0,
            14.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            sampling_connector_x(i),
            SAMPLING_CENTER_Y - SAMPLING_PANEL_Y / 2.0 - 7.0,
            DECK_Z + SAMPLING_PANEL_Z + 26.0,
        );
        let bore = centered_cylinder(
            format!("reagent_lot_station_closed_sampling_connector_bore_{i}"),
            9.0,
            16.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            sampling_connector_x(i),
            SAMPLING_CENTER_Y - SAMPLING_PANEL_Y / 2.0 - 7.0,
            DECK_Z + SAMPLING_PANEL_Z + 26.0,
        );
        ports = ports + (collar - bore);
    }
    ports
}

fn sealed_sample_vial_wells() -> Part {
    let mut wells = Part::empty("reagent_lot_station_sealed_sample_vial_wells");
    for i in 0..SEALED_SAMPLE_VIAL_WELLS {
        let x = SAMPLING_CENTER_X - 105.0 + (i % 4) as f64 * 70.0;
        let y = SAMPLING_CENTER_Y + 78.0 + (i / 4) as f64 * 42.0;
        let collar = centered_cylinder(
            format!("reagent_lot_station_sealed_sample_vial_well_collar_{i}"),
            15.0,
            8.0,
            30,
        )
        .translate(x, y, DECK_Z + SAMPLING_PANEL_Z + 4.0);
        let bore = centered_cylinder(
            format!("reagent_lot_station_sealed_sample_vial_well_bore_{i}"),
            10.0,
            10.0,
            28,
        )
        .translate(x, y, DECK_Z + SAMPLING_PANEL_Z + 4.0);
        wells = wells + (collar - bore);
    }
    wells
}

fn sample_line_comb() -> Part {
    let mut comb = Part::empty("reagent_lot_station_closed_sampling_line_comb");
    for i in 0..CLOSED_CONNECTOR_PORTS {
        comb = comb
            + centered_cube(
                format!("reagent_lot_station_closed_sampling_line_channel_{i}"),
                8.0,
                96.0,
                10.0,
            )
            .translate(
                sampling_connector_x(i),
                SAMPLING_CENTER_Y - 44.0,
                DECK_Z + SAMPLING_PANEL_Z + 5.0,
            );
    }
    comb
}

fn barcode_rfid_custody_plate() -> Part {
    let plate = centered_cube(
        "reagent_lot_station_barcode_rfid_custody_plate",
        CUSTODY_PLATE_X,
        CUSTODY_PLATE_Y,
        CUSTODY_PLATE_Z,
    )
    .translate(
        CUSTODY_CENTER_X,
        CUSTODY_CENTER_Y,
        DECK_Z + CUSTODY_PLATE_Z / 2.0,
    );
    plate + barcode_lands() + rfid_lands() + custody_token_wells()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("reagent_lot_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("reagent_lot_station_barcode_land_{i}"),
                78.0,
                28.0,
                5.0,
            )
            .translate(
                CUSTODY_CENTER_X - 156.0 + (i % 4) as f64 * 104.0,
                CUSTODY_CENTER_Y - 54.0 + (i / 4) as f64 * 38.0,
                DECK_Z + CUSTODY_PLATE_Z + 2.5,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("reagent_lot_station_rfid_lands");
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("reagent_lot_station_rfid_land_{i}"),
                60.0,
                48.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER_X - 156.0 + (i % 3) as f64 * 156.0,
                CUSTODY_CENTER_Y + 52.0 + (i / 3) as f64 * 48.0,
                DECK_Z + CUSTODY_PLATE_Z + 2.0,
            );
    }
    lands
}

fn custody_token_wells() -> Part {
    let mut wells = Part::empty("reagent_lot_station_custody_token_wells");
    for i in 0..CUSTODY_TOKEN_WELLS {
        let x = CUSTODY_CENTER_X - 144.0 + (i % 3) as f64 * 144.0;
        let y = CUSTODY_CENTER_Y - 8.0 + (i / 3) as f64 * 38.0;
        let collar = centered_cylinder(
            format!("reagent_lot_station_custody_token_well_collar_{i}"),
            13.0,
            5.0,
            26,
        )
        .translate(x, y, DECK_Z + CUSTODY_PLATE_Z + 2.5);
        let bore = centered_cylinder(
            format!("reagent_lot_station_custody_token_well_bore_{i}"),
            8.0,
            7.0,
            24,
        )
        .translate(x, y, DECK_Z + CUSTODY_PLATE_Z + 2.5);
        wells = wells + (collar - bore);
    }
    wells
}

fn thaw_equilibration_timer_features() -> Part {
    let panel = centered_cube(
        "reagent_lot_station_thaw_equilibration_timer_panel",
        TIMER_PANEL_X,
        TIMER_PANEL_Y,
        TIMER_PANEL_Z,
    )
    .translate(TIMER_CENTER_X, TIMER_CENTER_Y, DECK_Z + TIMER_PANEL_Z / 2.0);
    panel + timer_dials() + timer_token_slots() + equilibration_lane_ticks()
}

fn timer_dials() -> Part {
    let mut dials = Part::empty("reagent_lot_station_timer_dials");
    for i in 0..TIMER_DIALS {
        let x = TIMER_CENTER_X - 150.0 + i as f64 * 100.0;
        let base = centered_cylinder(
            format!("reagent_lot_station_timer_dial_base_{i}"),
            24.0,
            8.0,
            48,
        )
        .translate(x, TIMER_CENTER_Y - 38.0, DECK_Z + TIMER_PANEL_Z + 4.0);
        let pointer = centered_cube(
            format!("reagent_lot_station_timer_dial_pointer_{i}"),
            34.0,
            5.0,
            6.0,
        )
        .rotate(0.0, 0.0, i as f64 * 35.0)
        .translate(x, TIMER_CENTER_Y - 38.0, DECK_Z + TIMER_PANEL_Z + 10.0);
        dials = dials + base + pointer;
    }
    dials
}

fn timer_token_slots() -> Part {
    let mut slots = Part::empty("reagent_lot_station_timer_token_slots");
    for i in 0..TIMER_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("reagent_lot_station_timer_token_slot_{i}"),
                40.0,
                20.0,
                6.0,
            )
            .translate(
                TIMER_CENTER_X - 170.0 + (i % 4) as f64 * 114.0,
                TIMER_CENTER_Y + 34.0 + (i / 4) as f64 * 42.0,
                DECK_Z + TIMER_PANEL_Z + 3.0,
            );
    }
    slots
}

fn equilibration_lane_ticks() -> Part {
    let mut ticks = Part::empty("reagent_lot_station_equilibration_lane_ticks");
    for lane in 0..EQUILIBRATION_LANES {
        for tick in 0..5 {
            ticks = ticks
                + centered_cube(
                    format!("reagent_lot_station_equilibration_lane_{lane}_tick_{tick}"),
                    5.0,
                    18.0,
                    5.0,
                )
                .translate(
                    TIMER_CENTER_X - 180.0 + lane as f64 * 120.0 + tick as f64 * 18.0,
                    TIMER_CENTER_Y + 72.0,
                    DECK_Z + TIMER_PANEL_Z + 2.5,
                );
        }
    }
    ticks
}

fn closed_transfer_gate() -> Part {
    let wall = centered_cube(
        "reagent_lot_station_closed_transfer_gate_wall",
        TRANSFER_GATE_X,
        TRANSFER_GATE_Y,
        TRANSFER_GATE_Z,
    )
    .translate(0.0, TRANSFER_GATE_CENTER_Y, DECK_Z + TRANSFER_GATE_Z / 2.0);
    let window = centered_cube(
        "reagent_lot_station_closed_transfer_gate_pass_window_cut",
        PASS_THROUGH_WINDOW_X,
        TRANSFER_GATE_Y + 2.0,
        PASS_THROUGH_WINDOW_Z,
    )
    .translate(0.0, TRANSFER_GATE_CENTER_Y, DECK_Z + 78.0);

    (wall - window) + gate_latch_lands() + gate_gasket_lips()
}

fn gate_latch_lands() -> Part {
    let mut latches = Part::empty("reagent_lot_station_closed_transfer_gate_latches");
    for i in 0..GATE_LATCHES {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let tier = i / 2;
        latches = latches
            + centered_cube(
                format!("reagent_lot_station_closed_transfer_gate_latch_land_{i}"),
                44.0,
                12.0,
                24.0,
            )
            .translate(
                side * (PASS_THROUGH_WINDOW_X / 2.0 + 48.0),
                TRANSFER_GATE_CENTER_Y - TRANSFER_GATE_Y / 2.0 - 6.0,
                DECK_Z + 34.0 + tier as f64 * 38.0,
            );
    }
    latches
}

fn gate_gasket_lips() -> Part {
    let top = centered_cube(
        "reagent_lot_station_closed_transfer_gate_top_gasket_lip",
        PASS_THROUGH_WINDOW_X + 54.0,
        12.0,
        10.0,
    )
    .translate(
        0.0,
        TRANSFER_GATE_CENTER_Y - TRANSFER_GATE_Y / 2.0 - 6.0,
        DECK_Z + 78.0 + PASS_THROUGH_WINDOW_Z / 2.0 + 5.0,
    );
    let bottom = centered_cube(
        "reagent_lot_station_closed_transfer_gate_bottom_gasket_lip",
        PASS_THROUGH_WINDOW_X + 54.0,
        12.0,
        10.0,
    )
    .translate(
        0.0,
        TRANSFER_GATE_CENTER_Y - TRANSFER_GATE_Y / 2.0 - 6.0,
        DECK_Z + 78.0 - PASS_THROUGH_WINDOW_Z / 2.0 - 5.0,
    );
    let left = centered_cube(
        "reagent_lot_station_closed_transfer_gate_left_gasket_lip",
        10.0,
        12.0,
        PASS_THROUGH_WINDOW_Z + 54.0,
    )
    .translate(
        -PASS_THROUGH_WINDOW_X / 2.0 - 5.0,
        TRANSFER_GATE_CENTER_Y - TRANSFER_GATE_Y / 2.0 - 6.0,
        DECK_Z + 78.0,
    );
    let right = centered_cube(
        "reagent_lot_station_closed_transfer_gate_right_gasket_lip",
        10.0,
        12.0,
        PASS_THROUGH_WINDOW_Z + 54.0,
    )
    .translate(
        PASS_THROUGH_WINDOW_X / 2.0 + 5.0,
        TRANSFER_GATE_CENTER_Y - TRANSFER_GATE_Y / 2.0 - 6.0,
        DECK_Z + 78.0,
    );
    top + bottom + left + right
}

fn robotic_gripper_clearance_gauges() -> Part {
    let mut gauges = Part::empty("reagent_lot_station_robotic_gripper_clearance_gauges");
    for i in 0..GRIPPER_GAUGE_COUNT {
        let (x, y) = gripper_gauge_xy(i);
        let base = centered_cube(
            format!("reagent_lot_station_robotic_gripper_pad_gauge_{i}"),
            GRIPPER_PAD_X,
            GRIPPER_PAD_Y,
            GRIPPER_PAD_Z,
        )
        .translate(x, y, DECK_Z + GRIPPER_PAD_Z / 2.0);
        let mast = centered_cube(
            format!("reagent_lot_station_robotic_gripper_clearance_mast_{i}"),
            8.0,
            8.0,
            GRIPPER_CLEARANCE_Z,
        )
        .translate(x, y, DECK_Z + GRIPPER_CLEARANCE_Z / 2.0);
        gauges = gauges + base + mast;
    }
    gauges
}

fn service_keepout_frame() -> Part {
    let front = centered_cube(
        "reagent_lot_station_front_robot_approach_keepout_gauge",
        DECK_X - 160.0,
        FRONT_ROBOT_CLEARANCE_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_CLEARANCE_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        "reagent_lot_station_rear_service_keepout_gauge",
        DECK_X - 200.0,
        REAR_SERVICE_CLEARANCE_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE_Y / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left = centered_cube(
        "reagent_lot_station_left_sampling_service_keepout_gauge",
        LEFT_SERVICE_CLEARANCE_X,
        DECK_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -DECK_X / 2.0 - LEFT_SERVICE_CLEARANCE_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "reagent_lot_station_right_decision_service_keepout_gauge",
        RIGHT_DECISION_CLEARANCE_X,
        DECK_Y - 180.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_DECISION_CLEARANCE_X / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top = centered_cube(
        "reagent_lot_station_top_camera_service_keepout_gauge",
        EVIDENCE_LANE_X,
        112.0,
        16.0,
    )
    .translate(EVIDENCE_CENTER_X, EVIDENCE_CENTER_Y, TOP_CAMERA_CLEARANCE_Z);

    front + rear + left + right + top
}

fn quarantine_slot_xy(bank: usize, slot: usize) -> (f64, f64) {
    assert!(bank < BANK_COUNT);
    assert!(slot < SLOTS_PER_BANK);
    let x = BANK_CENTER_X - bank_span_x() / 2.0 + bank as f64 * BANK_PITCH_X + BANK_PITCH_X / 2.0;
    let y = BANK_CENTER_Y - 96.0 + slot as f64 * 64.0;
    (x, y)
}

fn logger_dock_x(index: usize) -> f64 {
    assert!(index < LOGGER_DOCKS);
    LOGGER_CENTER_X - logger_span_x() / 2.0 + index as f64 * LOGGER_PITCH_X
}

fn evidence_card_x(index: usize) -> f64 {
    assert!(index < EVENT_CARD_POCKETS);
    EVIDENCE_CENTER_X - 420.0 + index as f64 * 120.0
}

fn disposition_lane_x(lane: usize) -> f64 {
    assert!(lane < DISPOSITION_LANES);
    DISPOSITION_CENTER_X - disposition_total_span_x() / 2.0 + lane as f64 * DISPOSITION_LANE_PITCH_X
}

fn disposition_slot_y(slot: usize) -> f64 {
    assert!(slot < DISPOSITION_SLOTS_PER_LANE);
    DISPOSITION_CENTER_Y - 122.0 + slot as f64 * 78.0
}

fn sampling_connector_x(index: usize) -> f64 {
    assert!(index < CLOSED_CONNECTOR_PORTS);
    SAMPLING_CENTER_X - closed_connector_span_x() / 2.0 + index as f64 * CLOSED_CONNECTOR_PITCH_X
}

fn gripper_gauge_xy(index: usize) -> (f64, f64) {
    assert!(index < GRIPPER_GAUGE_COUNT);
    match index {
        0..=3 => {
            let bank = index % BANK_COUNT;
            let slot = index / BANK_COUNT + 1;
            let (x, y) = quarantine_slot_xy(bank, slot);
            (x, y + 18.0)
        }
        4..=6 => (disposition_lane_x(index - 4), DISPOSITION_CENTER_Y - 170.0),
        7 => (SAMPLING_CENTER_X + 102.0, SAMPLING_CENTER_Y - 84.0),
        8 => (LOGGER_CENTER_X + 220.0, LOGGER_CENTER_Y + 82.0),
        _ => (CUSTODY_CENTER_X + 188.0, CUSTODY_CENTER_Y - 66.0),
    }
}

fn bank_span_x() -> f64 {
    BANK_COUNT as f64 * BANK_PITCH_X
}

fn logger_span_x() -> f64 {
    (LOGGER_DOCKS as f64 - 1.0) * LOGGER_PITCH_X + LOGGER_DOCK_X
}

fn disposition_total_span_x() -> f64 {
    (DISPOSITION_LANES as f64 - 1.0) * DISPOSITION_LANE_PITCH_X + DISPOSITION_SLOT_X
}

fn disposition_lane_gap() -> f64 {
    DISPOSITION_LANE_PITCH_X - DISPOSITION_SLOT_X
}

fn closed_connector_span_x() -> f64 {
    (CLOSED_CONNECTOR_PORTS as f64 - 1.0) * CLOSED_CONNECTOR_PITCH_X + 36.0
}

fn timer_span_x() -> f64 {
    (TIMER_DIALS as f64 - 1.0) * 100.0 + 48.0
}

fn required_feature_count() -> usize {
    [
        QUARANTINE_SLOT_COUNT > 0,
        LOGGER_DOCKS > 0,
        EVENT_CARD_POCKETS > 0,
        DISPOSITION_LANES == 3,
        SEALED_SAMPLE_VIAL_WELLS > 0,
        CLOSED_CONNECTOR_PORTS > 0,
        BARCODE_LANDS > 0,
        RFID_LANDS > 0,
        TIMER_DIALS > 0,
        GRIPPER_GAUGE_COUNT > 0,
        DRAIN_PORT_D > 0.0,
    ]
    .iter()
    .filter(|covered| **covered)
    .count()
}

#[cfg_attr(not(test), allow(dead_code))]
struct ComponentSpec {
    name: &'static str,
    center_x: f64,
    center_y: f64,
    x: f64,
    y: f64,
}

fn component_specs() -> [ComponentSpec; 8] {
    [
        ComponentSpec {
            name: "quarantine nest banks",
            center_x: BANK_CENTER_X,
            center_y: BANK_CENTER_Y,
            x: BANK_PLATE_X,
            y: BANK_PLATE_Y,
        },
        ComponentSpec {
            name: "temperature logger docks",
            center_x: LOGGER_CENTER_X,
            center_y: LOGGER_CENTER_Y,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        ComponentSpec {
            name: "excursion evidence lane",
            center_x: EVIDENCE_CENTER_X,
            center_y: EVIDENCE_CENTER_Y,
            x: EVIDENCE_LANE_X,
            y: EVIDENCE_LANE_Y,
        },
        ComponentSpec {
            name: "release hold reject lanes",
            center_x: DISPOSITION_CENTER_X,
            center_y: DISPOSITION_CENTER_Y,
            x: DISPOSITION_PANEL_X,
            y: DISPOSITION_PANEL_Y,
        },
        ComponentSpec {
            name: "sealed sampling split point",
            center_x: SAMPLING_CENTER_X,
            center_y: SAMPLING_CENTER_Y,
            x: SAMPLING_PANEL_X,
            y: SAMPLING_PANEL_Y,
        },
        ComponentSpec {
            name: "barcode rfid custody plate",
            center_x: CUSTODY_CENTER_X,
            center_y: CUSTODY_CENTER_Y,
            x: CUSTODY_PLATE_X,
            y: CUSTODY_PLATE_Y,
        },
        ComponentSpec {
            name: "thaw equilibration timer",
            center_x: TIMER_CENTER_X,
            center_y: TIMER_CENTER_Y,
            x: TIMER_PANEL_X,
            y: TIMER_PANEL_Y,
        },
        ComponentSpec {
            name: "closed transfer gate",
            center_x: 0.0,
            center_y: TRANSFER_GATE_CENTER_Y,
            x: TRANSFER_GATE_X,
            y: TRANSFER_GATE_Y,
        },
    ]
}

fn fits_on_deck(center_x: f64, center_y: f64, x: f64, y: f64, edge_margin: f64) -> bool {
    center_x.abs() + x / 2.0 <= DECK_X / 2.0 - edge_margin
        && center_y.abs() + y / 2.0 <= DECK_Y / 2.0 - edge_margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_has_assembly() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .last()
            .unwrap()
            .ends_with("closed_reagent_lot_quarantine_temperature_excursion_station_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_reagent_lot_quarantine_temperature_excursion_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn geometry_bounds_keep_all_primary_components_on_deck() {
        for spec in component_specs() {
            assert!(
                fits_on_deck(spec.center_x, spec.center_y, spec.x, spec.y, 28.0),
                "{} exceeds deck envelope",
                spec.name
            );
        }
        assert_eq!(DECK_X, 1680.0);
        assert_eq!(DECK_Y, 1080.0);
        assert!(LEAK_SUMP_X < DECK_X - 120.0);
        assert!(LEAK_SUMP_Y < DECK_Y - 120.0);
    }

    #[test]
    fn quarantine_logger_and_evidence_counts_match_design_intent() {
        assert_eq!(BANK_COUNT, 3);
        assert_eq!(SLOTS_PER_BANK, 4);
        assert_eq!(QUARANTINE_SLOT_COUNT, 12);
        assert_eq!(LOGGER_DOCKS, 6);
        assert_eq!(LOGGER_POGO_PINS, LOGGER_DOCKS * 3);
        assert_eq!(EVENT_CARD_POCKETS, 8);
        assert_eq!(TAMPER_SEAL_WELLS, 12);
        assert_eq!(COLD_REFERENCE_PUCKS, 6);
    }

    #[test]
    fn disposition_sampling_and_custody_features_are_explicit() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE, 4);
        assert_eq!(DISPOSITION_SLOT_COUNT, 12);
        assert!(disposition_lane_gap() >= DISPOSITION_MIN_GAP);
        assert_eq!(CLOSED_CONNECTOR_PORTS, 6);
        assert_eq!(SEALED_SAMPLE_VIAL_WELLS, 8);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 6);
        assert_eq!(CUSTODY_TOKEN_WELLS, 9);
    }

    #[test]
    fn timer_transfer_gate_and_robot_clearance_are_represented() {
        assert_eq!(TIMER_DIALS, 4);
        assert_eq!(TIMER_TOKEN_SLOTS, 8);
        assert_eq!(EQUILIBRATION_LANES, 4);
        assert_eq!(GATE_LATCHES, 8);
        assert_eq!(GRIPPER_GAUGE_COUNT, 10);
        assert!(GRIPPER_CLEARANCE_Z > BANK_PLATE_Z + 70.0);
        assert!(TRANSFER_GATE_Z > GRIPPER_CLEARANCE_Z);
        assert!(FRONT_ROBOT_CLEARANCE_Y >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE_Y >= 280.0);
        assert!(TOP_CAMERA_CLEARANCE_Z >= 340.0);
    }

    #[test]
    fn required_feature_coverage_remains_complete() {
        assert_eq!(required_feature_count(), 11);
        assert!(DRAIN_PORT_D > 0.0);
        assert!(MOUNT_BOSSES >= 8);
        assert!(PASS_THROUGH_WINDOW_X < TRANSFER_GATE_X);
        assert!(PASS_THROUGH_WINDOW_Z < TRANSFER_GATE_Z);
    }
}
