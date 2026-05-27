use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic cassette quarantine lid-lockout validation station.
//
// Design intent:
// - Package a suspect culture cassette and its removed/failed lid as a
//   mechanically segregated validation article after an automation exception.
// - Give the robot and evidence cameras hard datums for cassette dock state,
//   lid quarantine state, lockout pin/gate witness state, custody labels,
//   pressure-decay handoff ports, wipe/contact coupon locations, and
//   release/hold/reject disposition routing.
// - Model mechanical validation packaging only. This is not a sterile-process
//   claim, SOP, pressure-rated device, biological acceptance criterion, or
//   acceptance test definition.

const OUTPUTS: [&str; 13] = [
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_containment_deck.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_suspect_cassette_dock.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_quarantine_lid_receiver.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_lockout_pin_gate_witness.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_tamper_evident_seal_lands.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_barcode_rfid_custody_lands.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_pressure_decay_leak_witness_ports.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_wipe_contact_coupon_pockets.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_release_hold_reject_lanes.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_camera_evidence_bridge.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_robot_gripper_approach_gauges.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_service_keepouts.stl",
    "output/closed_robotic_cassette_quarantine_lid_lockout_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 23] = [
    "mechanical_validation_packaging_only",
    "suspect_cassette_dock",
    "quarantine_lid_receiver",
    "lockout_pin_witness_bores",
    "gate_position_witness_flags",
    "tamper_evident_seal_lands",
    "seal_tie_slots",
    "barcode_lands",
    "rfid_lands",
    "custody_card_slots",
    "pressure_decay_ports",
    "leak_witness_cups",
    "wipe_coupon_pockets",
    "contact_coupon_pockets",
    "release_lane",
    "hold_lane",
    "reject_lane",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_gripper_approach_gauges",
    "service_keepouts",
    "not_pressure_rated",
    "not_biological_acceptance_criterion",
];

#[cfg(test)]
const SCOPE_LIMITS: [&str; 4] = [
    "mechanical_validation_packaging_only",
    "not_a_sterile_process_claim",
    "not_a_pressure_rated_device",
    "not_a_biological_acceptance_criterion",
];

const DECK_X: f64 = 1260.0;
const DECK_Y: f64 = 840.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 50.0;
const SUMP_X: f64 = 1085.0;
const SUMP_Y: f64 = 650.0;
const SUMP_Z: f64 = 6.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 8;

const CASSETTE_DOCK_X: f64 = 390.0;
const CASSETTE_DOCK_Y: f64 = 250.0;
const CASSETTE_DOCK_Z: f64 = 44.0;
const CASSETTE_DOCK_POS: (f64, f64) = (-365.0, 180.0);
const CASSETTE_BASIN_X: f64 = 312.0;
const CASSETTE_BASIN_Y: f64 = 168.0;
const CASSETTE_BASIN_DEPTH: f64 = 28.0;
const CASSETTE_DATUM_PINS: usize = 4;
const CASSETTE_GRIPPER_RELIEFS: usize = 4;
const CASSETTE_STATUS_FLAGS: usize = 3;

const LID_RECEIVER_X: f64 = 350.0;
const LID_RECEIVER_Y: f64 = 230.0;
const LID_RECEIVER_Z: f64 = 38.0;
const LID_RECEIVER_POS: (f64, f64) = (155.0, 215.0);
const LID_RECESS_X: f64 = 268.0;
const LID_RECESS_Y: f64 = 142.0;
const LID_RECESS_DEPTH: f64 = 22.0;
const LID_CLAMPS: usize = 4;
const LID_CLAMP_OFFSETS: [(f64, f64); LID_CLAMPS] = [
    (-135.0, -82.0),
    (135.0, -82.0),
    (-135.0, 82.0),
    (135.0, 82.0),
];
const LID_HINGE_WITNESS_SLOTS: usize = 3;
const LID_STATUS_FLAGS: usize = 3;

const LOCKOUT_GATE_X: f64 = 520.0;
const LOCKOUT_GATE_Y: f64 = 130.0;
const LOCKOUT_GATE_Z: f64 = 42.0;
const LOCKOUT_GATE_POS: (f64, f64) = (-160.0, -60.0);
const LOCKOUT_PIN_COUNT: usize = 6;
const GATE_WITNESS_FLAGS: usize = 3;
const LOCKOUT_PIN_D: f64 = 10.0;
const GATE_TOKEN_SLOTS: usize = 6;

const PRESSURE_BAR_X: f64 = 380.0;
const PRESSURE_BAR_Y: f64 = 126.0;
const PRESSURE_BAR_Z: f64 = 34.0;
const PRESSURE_BAR_POS: (f64, f64) = (400.0, -115.0);
const PRESSURE_DECAY_PORTS: usize = 4;
const PRESSURE_PORT_PITCH: f64 = 68.0;
const PRESSURE_PORT_D: f64 = 9.5;
const PRESSURE_PORT_COLLAR_D: f64 = 30.0;
const LEAK_WITNESS_CUPS: usize = 6;
const LEAK_WITNESS_CUP_D: f64 = 24.0;

const COUPON_BANK_X: f64 = 300.0;
const COUPON_BANK_Y: f64 = 160.0;
const COUPON_BANK_Z: f64 = 32.0;
const COUPON_BANK_POS: (f64, f64) = (-430.0, -185.0);
const WIPE_COUPON_POCKETS: usize = 6;
const CONTACT_COUPON_POCKETS: usize = 6;
const WIPE_COUPON_X: f64 = 58.0;
const WIPE_COUPON_Y: f64 = 24.0;
const CONTACT_COUPON_D: f64 = 28.0;

const CUSTODY_PANEL_X: f64 = 360.0;
const CUSTODY_PANEL_Y: f64 = 110.0;
const CUSTODY_PANEL_Z: f64 = 16.0;
const CUSTODY_PANEL_POS: (f64, f64) = (-405.0, -325.0);
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;
const CUSTODY_CARD_SLOTS: usize = 3;

const TAMPER_PANEL_X: f64 = 320.0;
const TAMPER_PANEL_Y: f64 = 106.0;
const TAMPER_PANEL_Z: f64 = 16.0;
const TAMPER_PANEL_POS: (f64, f64) = (-15.0, -326.0);
const TAMPER_SEAL_LANDS: usize = 8;
const SEAL_TIE_SLOTS: usize = 4;
const FRANGIBLE_BRIDGES: usize = 5;

const LANE_BANK_X: f64 = 370.0;
const LANE_BANK_Y: f64 = 120.0;
const LANE_BANK_Z: f64 = 38.0;
const LANE_BANK_POS: (f64, f64) = (390.0, -315.0);
const DISPOSITION_LANES: usize = 3;
const TOKENS_PER_LANE: usize = 4;
const RELEASE_LANE_INDEX: usize = 0;
const HOLD_LANE_INDEX: usize = 1;
const REJECT_LANE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 950.0;
const CAMERA_BRIDGE_Y: f64 = 46.0;
const CAMERA_BRIDGE_Z: f64 = 190.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (-20.0, 18.0);
const CAMERA_MOUNTS: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 8;
const CAMERA_MIN_CLEARANCE: f64 = 132.0;

const ROBOT_GAUGE_X: f64 = 1040.0;
const ROBOT_GAUGE_Y: f64 = 92.0;
const ROBOT_GAUGE_Z: f64 = 66.0;
const GRIPPER_APPROACH_GAUGE_PAIRS: usize = 3;
const GRIPPER_LANE_GAP: f64 = 118.0;

const SERVICE_KEEPOUT_X: f64 = 105.0;
const SERVICE_KEEPOUT_Y: f64 = 650.0;
const SERVICE_KEEPOUT_Z: f64 = 88.0;
const REAR_SERVICE_KEEPOUT_X: f64 = 950.0;
const REAR_SERVICE_KEEPOUT_Y: f64 = 95.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 305.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(suspect_cassette_dock(), OUTPUTS[1]);
    write_part(quarantine_lid_receiver(), OUTPUTS[2]);
    write_part(lockout_pin_gate_witness(), OUTPUTS[3]);
    write_part(tamper_evident_seal_lands(), OUTPUTS[4]);
    write_part(barcode_rfid_custody_lands(), OUTPUTS[5]);
    write_part(pressure_decay_leak_witness_ports(), OUTPUTS[6]);
    write_part(wipe_contact_coupon_pockets(), OUTPUTS[7]);
    write_part(release_hold_reject_lanes(), OUTPUTS[8]);
    write_part(camera_evidence_bridge(), OUTPUTS[9]);
    write_part(robot_gripper_approach_gauges(), OUTPUTS[10]);
    write_part(service_keepouts(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!(
        "Closed robotic cassette quarantine lid-lockout station: {:.0}mm x {:.0}mm contained deck, suspect cassette dock, quarantine lid receiver, {} lockout pin bores, and {} gate witness flags.",
        DECK_X, DECK_Y, LOCKOUT_PIN_COUNT, GATE_WITNESS_FLAGS
    );
    println!(
        "Evidence packaging: {} tamper seal lands, {} barcode lands, {} RFID lands, {} pressure-decay ports, {} leak witness cups, {} wipe/contact coupon pockets.",
        TAMPER_SEAL_LANDS,
        BARCODE_LANDS,
        RFID_LANDS,
        PRESSURE_DECAY_PORTS,
        LEAK_WITNESS_CUPS,
        WIPE_COUPON_POCKETS + CONTACT_COUPON_POCKETS
    );
    println!(
        "Disposition and automation: release/hold/reject lanes, {} camera mounts, {} evidence fiducials, {} gripper approach gauge pairs, top service clearance {:.0}mm. Mechanical validation packaging only; not pressure-rated or a biological acceptance criterion.",
        CAMERA_MOUNTS,
        EVIDENCE_FIDUCIALS,
        GRIPPER_APPROACH_GAUGE_PAIRS,
        TOP_SERVICE_CLEARANCE_Z
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "cassette_quarantine_lockout_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "cassette_quarantine_lockout_station_shallow_evidence_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.3);
    let drain = centered_cylinder(
        "cassette_quarantine_lockout_station_captured_witness_drain",
        DRAIN_D / 2.0,
        RIM_W + 34.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 84.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_zone_markers()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        "cassette_quarantine_lockout_front_low_containment_lip",
        DECK_X - 160.0,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z * 0.31,
    );
    let rear = centered_cube(
        "cassette_quarantine_lockout_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_z());
    let left = centered_cube(
        "cassette_quarantine_lockout_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_z());
    let right = centered_cube(
        "cassette_quarantine_lockout_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_z());

    front + rear + left + right
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("cassette_quarantine_lockout_station_deck_datums");
    for (i, (x, y)) in datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("cassette_quarantine_lockout_station_datum_boss_{i}"),
            13.0,
            6.0,
            36,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("cassette_quarantine_lockout_station_datum_bore_{i}"),
            3.2,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.0);
        datums = datums + (boss - bore);
    }
    datums
}

fn station_zone_markers() -> Part {
    let mut markers = Part::empty("cassette_quarantine_lockout_station_zone_markers");
    for (i, rect) in module_rects().iter().enumerate() {
        markers = markers
            + centered_cube(
                format!("cassette_quarantine_lockout_station_zone_marker_{i}"),
                rect.w + 18.0,
                rect.h + 18.0,
                2.2,
            )
            .translate(rect.x, rect.y, DECK_Z / 2.0 + 1.1);
    }
    markers
}

fn suspect_cassette_dock() -> Part {
    let base = centered_cube(
        "cassette_quarantine_lockout_suspect_cassette_dock_body",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    )
    .translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1,
        top_z(CASSETTE_DOCK_Z),
    );
    let basin = centered_cube(
        "cassette_quarantine_lockout_suspect_cassette_basin_recess",
        CASSETTE_BASIN_X,
        CASSETTE_BASIN_Y,
        CASSETTE_BASIN_DEPTH,
    )
    .translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1,
        recess_z(CASSETTE_DOCK_Z, CASSETTE_BASIN_DEPTH),
    );

    base - basin - cassette_gripper_relief_slots()
        + cassette_dock_side_rails()
        + cassette_datum_pins()
        + cassette_status_flags()
}

fn cassette_dock_side_rails() -> Part {
    let left = centered_cube(
        "cassette_quarantine_lockout_cassette_left_datum_rail",
        12.0,
        CASSETTE_DOCK_Y - 44.0,
        24.0,
    )
    .translate(
        CASSETTE_DOCK_POS.0 - CASSETTE_DOCK_X / 2.0 + 26.0,
        CASSETTE_DOCK_POS.1,
        DECK_Z / 2.0 + CASSETTE_DOCK_Z + 12.0,
    );
    let rear = centered_cube(
        "cassette_quarantine_lockout_cassette_rear_datum_rail",
        CASSETTE_DOCK_X - 58.0,
        12.0,
        24.0,
    )
    .translate(
        CASSETTE_DOCK_POS.0,
        CASSETTE_DOCK_POS.1 + CASSETTE_DOCK_Y / 2.0 - 26.0,
        DECK_Z / 2.0 + CASSETTE_DOCK_Z + 12.0,
    );
    let suspect_front_sweep = centered_cube(
        "cassette_quarantine_lockout_suspect_front_sweep_stop",
        CASSETTE_DOCK_X - 115.0,
        10.0,
        18.0,
    )
    .translate(
        CASSETTE_DOCK_POS.0 + 20.0,
        CASSETTE_DOCK_POS.1 - CASSETTE_DOCK_Y / 2.0 + 24.0,
        DECK_Z / 2.0 + CASSETTE_DOCK_Z + 9.0,
    );

    left + rear + suspect_front_sweep
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("cassette_quarantine_lockout_cassette_datum_pins");
    for (i, (x, y)) in cassette_pin_offsets().iter().enumerate() {
        let pin = centered_cylinder(
            format!("cassette_quarantine_lockout_cassette_datum_pin_{i}"),
            6.0,
            12.0,
            32,
        )
        .translate(
            CASSETTE_DOCK_POS.0 + x,
            CASSETTE_DOCK_POS.1 + y,
            DECK_Z / 2.0 + CASSETTE_DOCK_Z + 6.0,
        );
        let witness_ring = centered_cylinder(
            format!("cassette_quarantine_lockout_cassette_datum_witness_ring_{i}"),
            10.5,
            3.0,
            36,
        )
        .translate(
            CASSETTE_DOCK_POS.0 + x,
            CASSETTE_DOCK_POS.1 + y,
            DECK_Z / 2.0 + CASSETTE_DOCK_Z + 1.5,
        );
        pins = pins + witness_ring + pin;
    }
    pins
}

fn cassette_gripper_relief_slots() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_gripper_relief_slots");
    for i in 0..CASSETTE_GRIPPER_RELIEFS {
        let x = if i % 2 == 0 { -1.0 } else { 1.0 } * (CASSETTE_DOCK_X / 2.0 - 54.0);
        let y = if i < 2 { -54.0 } else { 54.0 };
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_lockout_gripper_relief_slot_{i}"),
                54.0,
                26.0,
                CASSETTE_DOCK_Z + 2.0,
            )
            .translate(
                CASSETTE_DOCK_POS.0 + x,
                CASSETTE_DOCK_POS.1 + y,
                top_z(CASSETTE_DOCK_Z),
            );
    }
    slots
}

fn cassette_status_flags() -> Part {
    let mut flags = Part::empty("cassette_quarantine_lockout_suspect_status_flags");
    for i in 0..CASSETTE_STATUS_FLAGS {
        flags = flags
            + centered_cube(
                format!("cassette_quarantine_lockout_suspect_status_flag_{i}"),
                46.0,
                8.0,
                20.0,
            )
            .translate(
                CASSETTE_DOCK_POS.0 - 60.0 + i as f64 * 60.0,
                CASSETTE_DOCK_POS.1 - CASSETTE_DOCK_Y / 2.0 + 46.0,
                DECK_Z / 2.0 + CASSETTE_DOCK_Z + 10.0,
            );
    }
    flags
}

fn quarantine_lid_receiver() -> Part {
    let base = centered_cube(
        "cassette_quarantine_lockout_lid_receiver_body",
        LID_RECEIVER_X,
        LID_RECEIVER_Y,
        LID_RECEIVER_Z,
    )
    .translate(
        LID_RECEIVER_POS.0,
        LID_RECEIVER_POS.1,
        top_z(LID_RECEIVER_Z),
    );
    let recess = centered_cube(
        "cassette_quarantine_lockout_lid_quarantine_recess",
        LID_RECESS_X,
        LID_RECESS_Y,
        LID_RECESS_DEPTH,
    )
    .translate(
        LID_RECEIVER_POS.0,
        LID_RECEIVER_POS.1,
        recess_z(LID_RECEIVER_Z, LID_RECESS_DEPTH),
    );

    base - recess - lid_hinge_witness_slots() + lid_receiver_clamps() + lid_status_flags()
}

fn lid_receiver_clamps() -> Part {
    let mut clamps = Part::empty("cassette_quarantine_lockout_lid_receiver_clamps");
    for (i, (x, y)) in LID_CLAMP_OFFSETS.iter().enumerate() {
        clamps = clamps
            + centered_cube(
                format!("cassette_quarantine_lockout_lid_receiver_clamp_{i}"),
                42.0,
                16.0,
                22.0,
            )
            .translate(
                LID_RECEIVER_POS.0 + x,
                LID_RECEIVER_POS.1 + y,
                DECK_Z / 2.0 + LID_RECEIVER_Z + 11.0,
            );
    }
    clamps
}

fn lid_hinge_witness_slots() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_lid_hinge_witness_slots");
    for i in 0..LID_HINGE_WITNESS_SLOTS {
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_lockout_lid_hinge_witness_slot_{i}"),
                54.0,
                20.0,
                LID_RECEIVER_Z + 2.0,
            )
            .translate(
                LID_RECEIVER_POS.0 - 70.0 + i as f64 * 70.0,
                LID_RECEIVER_POS.1 + LID_RECEIVER_Y / 2.0 - 36.0,
                top_z(LID_RECEIVER_Z),
            );
    }
    slots
}

fn lid_status_flags() -> Part {
    let mut flags = Part::empty("cassette_quarantine_lockout_lid_status_flags");
    for i in 0..LID_STATUS_FLAGS {
        flags = flags
            + centered_cube(
                format!("cassette_quarantine_lockout_lid_status_flag_{i}"),
                42.0,
                8.0,
                18.0,
            )
            .translate(
                LID_RECEIVER_POS.0 - 54.0 + i as f64 * 54.0,
                LID_RECEIVER_POS.1 - LID_RECEIVER_Y / 2.0 + 32.0,
                DECK_Z / 2.0 + LID_RECEIVER_Z + 9.0,
            );
    }
    flags
}

fn lockout_pin_gate_witness() -> Part {
    let body = centered_cube(
        "cassette_quarantine_lockout_pin_gate_witness_body",
        LOCKOUT_GATE_X,
        LOCKOUT_GATE_Y,
        LOCKOUT_GATE_Z,
    )
    .translate(
        LOCKOUT_GATE_POS.0,
        LOCKOUT_GATE_POS.1,
        top_z(LOCKOUT_GATE_Z),
    );

    body - lockout_pin_bores() - gate_token_slot_cuts() + gate_witness_flags() + pin_lanyard_posts()
}

fn lockout_pin_bores() -> Part {
    let mut bores = Part::empty("cassette_quarantine_lockout_pin_witness_bores");
    let start_x = -((LOCKOUT_PIN_COUNT as f64 - 1.0) * 58.0) / 2.0;
    for i in 0..LOCKOUT_PIN_COUNT {
        bores = bores
            + centered_cylinder(
                format!("cassette_quarantine_lockout_pin_bore_{i}"),
                LOCKOUT_PIN_D / 2.0,
                LOCKOUT_GATE_Z + 3.0,
                32,
            )
            .translate(
                LOCKOUT_GATE_POS.0 + start_x + i as f64 * 58.0,
                LOCKOUT_GATE_POS.1 + 28.0,
                top_z(LOCKOUT_GATE_Z),
            );
    }
    bores
}

fn gate_token_slot_cuts() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_gate_token_slot_cuts");
    let start_x = -((GATE_TOKEN_SLOTS as f64 - 1.0) * 64.0) / 2.0;
    for i in 0..GATE_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_lockout_gate_token_slot_{i}"),
                38.0,
                22.0,
                11.0,
            )
            .translate(
                LOCKOUT_GATE_POS.0 + start_x + i as f64 * 64.0,
                LOCKOUT_GATE_POS.1 - 30.0,
                recess_z(LOCKOUT_GATE_Z, 11.0),
            );
    }
    slots
}

fn gate_witness_flags() -> Part {
    let mut flags = Part::empty("cassette_quarantine_lockout_gate_position_witness_flags");
    for i in 0..GATE_WITNESS_FLAGS {
        let label = gate_witness_name(i);
        flags = flags
            + centered_cube(
                format!("cassette_quarantine_lockout_{label}_gate_witness_flag"),
                34.0,
                78.0,
                36.0,
            )
            .translate(
                LOCKOUT_GATE_POS.0 - 150.0 + i as f64 * 150.0,
                LOCKOUT_GATE_POS.1,
                DECK_Z / 2.0 + LOCKOUT_GATE_Z + 18.0,
            );
    }
    flags
}

fn pin_lanyard_posts() -> Part {
    let mut posts = Part::empty("cassette_quarantine_lockout_pin_lanyard_posts");
    for i in 0..LOCKOUT_PIN_COUNT {
        posts = posts
            + centered_cylinder(
                format!("cassette_quarantine_lockout_pin_lanyard_post_{i}"),
                5.0,
                16.0,
                24,
            )
            .translate(
                LOCKOUT_GATE_POS.0 - 145.0 + i as f64 * 58.0,
                LOCKOUT_GATE_POS.1 + LOCKOUT_GATE_Y / 2.0 - 18.0,
                DECK_Z / 2.0 + LOCKOUT_GATE_Z + 8.0,
            );
    }
    posts
}

fn tamper_evident_seal_lands() -> Part {
    let panel = centered_cube(
        "cassette_quarantine_lockout_tamper_evident_seal_panel",
        TAMPER_PANEL_X,
        TAMPER_PANEL_Y,
        TAMPER_PANEL_Z,
    )
    .translate(
        TAMPER_PANEL_POS.0,
        TAMPER_PANEL_POS.1,
        top_z(TAMPER_PANEL_Z),
    );

    panel + seal_lands() + seal_tie_slots() + frangible_bridge_witnesses()
}

fn seal_lands() -> Part {
    let mut lands = Part::empty("cassette_quarantine_lockout_tamper_evident_seal_lands");
    for i in 0..TAMPER_SEAL_LANDS {
        let col = i % 4;
        let row = i / 4;
        lands = lands
            + centered_cube(
                format!("cassette_quarantine_lockout_tamper_seal_land_{i}"),
                54.0,
                22.0,
                4.0,
            )
            .translate(
                TAMPER_PANEL_POS.0 - 120.0 + col as f64 * 80.0,
                TAMPER_PANEL_POS.1 + 24.0 - row as f64 * 48.0,
                DECK_Z / 2.0 + TAMPER_PANEL_Z + 2.0,
            );
    }
    lands
}

fn seal_tie_slots() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_seal_tie_slots");
    for i in 0..SEAL_TIE_SLOTS {
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_lockout_seal_tie_slot_bridge_{i}"),
                42.0,
                6.0,
                10.0,
            )
            .translate(
                TAMPER_PANEL_POS.0 - 120.0 + i as f64 * 80.0,
                TAMPER_PANEL_POS.1,
                DECK_Z / 2.0 + TAMPER_PANEL_Z + 5.0,
            );
    }
    slots
}

fn frangible_bridge_witnesses() -> Part {
    let mut bridges = Part::empty("cassette_quarantine_lockout_frangible_bridge_witnesses");
    for i in 0..FRANGIBLE_BRIDGES {
        bridges = bridges
            + centered_cube(
                format!("cassette_quarantine_lockout_frangible_bridge_witness_{i}"),
                8.0,
                TAMPER_PANEL_Y - 22.0,
                8.0,
            )
            .translate(
                TAMPER_PANEL_POS.0 - 130.0 + i as f64 * 65.0,
                TAMPER_PANEL_POS.1,
                DECK_Z / 2.0 + TAMPER_PANEL_Z + 4.0,
            );
    }
    bridges
}

fn barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "cassette_quarantine_lockout_barcode_rfid_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_PANEL_POS.0,
        CUSTODY_PANEL_POS.1,
        top_z(CUSTODY_PANEL_Z),
    );

    panel + barcode_lands() + rfid_lands() + custody_card_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("cassette_quarantine_lockout_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("cassette_quarantine_lockout_barcode_land_{i}"),
                70.0,
                22.0,
                3.0,
            )
            .translate(
                CUSTODY_PANEL_POS.0 - 132.0 + (i % 3) as f64 * 132.0,
                CUSTODY_PANEL_POS.1 + 30.0 - (i / 3) as f64 * 36.0,
                DECK_Z / 2.0 + CUSTODY_PANEL_Z + 1.5,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("cassette_quarantine_lockout_rfid_lands");
    for i in 0..RFID_LANDS {
        let x = CUSTODY_PANEL_POS.0 - 132.0 + i as f64 * 88.0;
        let antenna_land = centered_cube(
            format!("cassette_quarantine_lockout_rfid_antenna_land_{i}"),
            54.0,
            26.0,
            3.0,
        )
        .translate(
            x,
            CUSTODY_PANEL_POS.1 - 38.0,
            DECK_Z / 2.0 + CUSTODY_PANEL_Z + 1.5,
        );
        let center_bore = centered_cylinder(
            format!("cassette_quarantine_lockout_rfid_center_witness_{i}"),
            5.0,
            5.0,
            24,
        )
        .translate(
            x,
            CUSTODY_PANEL_POS.1 - 38.0,
            DECK_Z / 2.0 + CUSTODY_PANEL_Z + 1.5,
        );
        lands = lands + (antenna_land - center_bore);
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_custody_card_slots");
    for i in 0..CUSTODY_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("cassette_quarantine_lockout_custody_card_slot_{i}"),
                84.0,
                5.0,
                12.0,
            )
            .translate(
                CUSTODY_PANEL_POS.0 - 96.0 + i as f64 * 96.0,
                CUSTODY_PANEL_POS.1 + CUSTODY_PANEL_Y / 2.0 - 14.0,
                DECK_Z / 2.0 + CUSTODY_PANEL_Z + 6.0,
            );
    }
    slots
}

fn pressure_decay_leak_witness_ports() -> Part {
    let bar = centered_cube(
        "cassette_quarantine_lockout_pressure_decay_leak_witness_port_bar",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    )
    .translate(
        PRESSURE_BAR_POS.0,
        PRESSURE_BAR_POS.1,
        top_z(PRESSURE_BAR_Z),
    );

    bar - pressure_decay_port_bores() - leak_witness_cup_recesses()
        + pressure_port_collars()
        + leak_witness_cup_lips()
        + leak_route_tick_marks()
}

fn pressure_decay_port_bores() -> Part {
    let mut bores = Part::empty("cassette_quarantine_lockout_pressure_decay_port_bores");
    let start_x = -((PRESSURE_DECAY_PORTS as f64 - 1.0) * PRESSURE_PORT_PITCH) / 2.0;
    for i in 0..PRESSURE_DECAY_PORTS {
        bores = bores
            + centered_cylinder(
                format!("cassette_quarantine_lockout_pressure_decay_port_bore_{i}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_BAR_Z + 4.0,
                32,
            )
            .translate(
                PRESSURE_BAR_POS.0 + start_x + i as f64 * PRESSURE_PORT_PITCH,
                PRESSURE_BAR_POS.1 + 30.0,
                top_z(PRESSURE_BAR_Z),
            );
    }
    bores
}

fn pressure_port_collars() -> Part {
    let mut collars = Part::empty("cassette_quarantine_lockout_pressure_port_collars");
    let start_x = -((PRESSURE_DECAY_PORTS as f64 - 1.0) * PRESSURE_PORT_PITCH) / 2.0;
    for i in 0..PRESSURE_DECAY_PORTS {
        collars = collars
            + centered_cylinder(
                format!("cassette_quarantine_lockout_pressure_port_collar_{i}"),
                PRESSURE_PORT_COLLAR_D / 2.0,
                7.0,
                36,
            )
            .translate(
                PRESSURE_BAR_POS.0 + start_x + i as f64 * PRESSURE_PORT_PITCH,
                PRESSURE_BAR_POS.1 + 30.0,
                DECK_Z / 2.0 + PRESSURE_BAR_Z + 3.5,
            );
    }
    collars
}

fn leak_witness_cup_recesses() -> Part {
    let mut cups = Part::empty("cassette_quarantine_lockout_leak_witness_cup_recesses");
    for i in 0..LEAK_WITNESS_CUPS {
        cups = cups
            + centered_cylinder(
                format!("cassette_quarantine_lockout_leak_witness_cup_recess_{i}"),
                LEAK_WITNESS_CUP_D / 2.0,
                PRESSURE_BAR_Z + 4.0,
                36,
            )
            .translate(
                PRESSURE_BAR_POS.0 - 135.0 + i as f64 * 54.0,
                PRESSURE_BAR_POS.1 - 31.0,
                recess_z(PRESSURE_BAR_Z, 18.0),
            );
    }
    cups
}

fn leak_witness_cup_lips() -> Part {
    let mut lips = Part::empty("cassette_quarantine_lockout_leak_witness_cup_lips");
    for i in 0..LEAK_WITNESS_CUPS {
        lips = lips
            + centered_cylinder(
                format!("cassette_quarantine_lockout_leak_witness_cup_lip_{i}"),
                LEAK_WITNESS_CUP_D / 2.0 + 4.0,
                4.0,
                36,
            )
            .translate(
                PRESSURE_BAR_POS.0 - 135.0 + i as f64 * 54.0,
                PRESSURE_BAR_POS.1 - 31.0,
                DECK_Z / 2.0 + PRESSURE_BAR_Z + 2.0,
            );
    }
    lips
}

fn leak_route_tick_marks() -> Part {
    let mut ticks = Part::empty("cassette_quarantine_lockout_leak_route_tick_marks");
    for i in 0..PRESSURE_DECAY_PORTS {
        ticks = ticks
            + centered_cube(
                format!("cassette_quarantine_lockout_pressure_route_tick_{i}"),
                32.0,
                4.0,
                5.0,
            )
            .translate(
                PRESSURE_BAR_POS.0 - 102.0 + i as f64 * PRESSURE_PORT_PITCH,
                PRESSURE_BAR_POS.1,
                DECK_Z / 2.0 + PRESSURE_BAR_Z + 2.5,
            );
    }
    ticks
}

fn wipe_contact_coupon_pockets() -> Part {
    let bank = centered_cube(
        "cassette_quarantine_lockout_wipe_contact_coupon_bank",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(COUPON_BANK_POS.0, COUPON_BANK_POS.1, top_z(COUPON_BANK_Z));

    bank - wipe_coupon_recesses() - contact_coupon_recesses()
        + coupon_retainer_lips()
        + coupon_custody_ticks()
}

fn wipe_coupon_recesses() -> Part {
    let mut pockets = Part::empty("cassette_quarantine_lockout_wipe_coupon_pockets");
    for i in 0..WIPE_COUPON_POCKETS {
        pockets = pockets
            + centered_cube(
                format!("cassette_quarantine_lockout_wipe_coupon_pocket_{i}"),
                WIPE_COUPON_X,
                WIPE_COUPON_Y,
                16.0,
            )
            .translate(
                COUPON_BANK_POS.0 - 90.0 + (i % 3) as f64 * 90.0,
                COUPON_BANK_POS.1 + 38.0 - (i / 3) as f64 * 42.0,
                recess_z(COUPON_BANK_Z, 16.0),
            );
    }
    pockets
}

fn contact_coupon_recesses() -> Part {
    let mut pockets = Part::empty("cassette_quarantine_lockout_contact_coupon_pockets");
    for i in 0..CONTACT_COUPON_POCKETS {
        pockets = pockets
            + centered_cylinder(
                format!("cassette_quarantine_lockout_contact_coupon_pocket_{i}"),
                CONTACT_COUPON_D / 2.0,
                16.0,
                36,
            )
            .translate(
                COUPON_BANK_POS.0 - 112.0 + i as f64 * 45.0,
                COUPON_BANK_POS.1 - 50.0,
                recess_z(COUPON_BANK_Z, 16.0),
            );
    }
    pockets
}

fn coupon_retainer_lips() -> Part {
    let mut lips = Part::empty("cassette_quarantine_lockout_coupon_retainer_lips");
    for i in 0..WIPE_COUPON_POCKETS {
        lips = lips
            + centered_cube(
                format!("cassette_quarantine_lockout_wipe_coupon_lip_{i}"),
                WIPE_COUPON_X + 8.0,
                4.0,
                5.0,
            )
            .translate(
                COUPON_BANK_POS.0 - 90.0 + (i % 3) as f64 * 90.0,
                COUPON_BANK_POS.1 + 22.0 - (i / 3) as f64 * 42.0,
                DECK_Z / 2.0 + COUPON_BANK_Z + 2.5,
            );
    }
    lips
}

fn coupon_custody_ticks() -> Part {
    let mut ticks = Part::empty("cassette_quarantine_lockout_coupon_custody_ticks");
    for i in 0..CONTACT_COUPON_POCKETS {
        ticks = ticks
            + centered_cube(
                format!("cassette_quarantine_lockout_contact_coupon_custody_tick_{i}"),
                14.0,
                4.0,
                4.0,
            )
            .translate(
                COUPON_BANK_POS.0 - 112.0 + i as f64 * 45.0,
                COUPON_BANK_POS.1 - 25.0,
                DECK_Z / 2.0 + COUPON_BANK_Z + 2.0,
            );
    }
    ticks
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "cassette_quarantine_lockout_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(LANE_BANK_POS.0, LANE_BANK_POS.1, top_z(LANE_BANK_Z));

    bank - disposition_lane_recesses() - disposition_token_cuts()
        + disposition_lane_flags()
        + lane_barrier_ribs()
}

fn disposition_lane_recesses() -> Part {
    let mut recesses = Part::empty("cassette_quarantine_lockout_disposition_lane_recesses");
    let lane_pitch = LANE_BANK_Y / DISPOSITION_LANES as f64;
    for lane in 0..DISPOSITION_LANES {
        recesses = recesses
            + centered_cube(
                format!(
                    "cassette_quarantine_lockout_{}_lane_recess",
                    disposition_lane_name(lane)
                ),
                LANE_BANK_X - 46.0,
                lane_pitch - 10.0,
                12.0,
            )
            .translate(
                LANE_BANK_POS.0 - 10.0,
                LANE_BANK_POS.1 - lane_pitch + lane as f64 * lane_pitch,
                recess_z(LANE_BANK_Z, 12.0),
            );
    }
    recesses
}

fn disposition_token_cuts() -> Part {
    let mut slots = Part::empty("cassette_quarantine_lockout_disposition_token_cuts");
    let lane_pitch = LANE_BANK_Y / DISPOSITION_LANES as f64;
    for lane in 0..DISPOSITION_LANES {
        for token in 0..TOKENS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!(
                        "cassette_quarantine_lockout_{}_lane_token_slot_{token}",
                        disposition_lane_name(lane)
                    ),
                    38.0,
                    16.0,
                    8.0,
                )
                .translate(
                    LANE_BANK_POS.0 - 122.0 + token as f64 * 72.0,
                    LANE_BANK_POS.1 - lane_pitch + lane as f64 * lane_pitch,
                    recess_z(LANE_BANK_Z, 8.0),
                );
        }
    }
    slots
}

fn disposition_lane_flags() -> Part {
    let mut flags = Part::empty("cassette_quarantine_lockout_disposition_lane_flags");
    let lane_pitch = LANE_BANK_Y / DISPOSITION_LANES as f64;
    for lane in 0..DISPOSITION_LANES {
        flags = flags
            + centered_cube(
                format!(
                    "cassette_quarantine_lockout_{}_lane_positive_stop",
                    disposition_lane_name(lane)
                ),
                24.0,
                lane_pitch - 8.0,
                32.0,
            )
            .translate(
                LANE_BANK_POS.0 + LANE_BANK_X / 2.0 - 28.0,
                LANE_BANK_POS.1 - lane_pitch + lane as f64 * lane_pitch,
                DECK_Z / 2.0 + LANE_BANK_Z + 16.0,
            );
    }
    flags
}

fn lane_barrier_ribs() -> Part {
    let mut ribs = Part::empty("cassette_quarantine_lockout_lane_barrier_ribs");
    let lane_pitch = LANE_BANK_Y / DISPOSITION_LANES as f64;
    for i in 1..DISPOSITION_LANES {
        ribs = ribs
            + centered_cube(
                format!("cassette_quarantine_lockout_disposition_lane_barrier_{i}"),
                LANE_BANK_X - 32.0,
                4.0,
                18.0,
            )
            .translate(
                LANE_BANK_POS.0,
                LANE_BANK_POS.1 - LANE_BANK_Y / 2.0 + i as f64 * lane_pitch,
                DECK_Z / 2.0 + LANE_BANK_Z + 9.0,
            );
    }
    ribs
}

fn camera_evidence_bridge() -> Part {
    let left_post = bridge_post("left").translate(
        CAMERA_BRIDGE_POS.0 - CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = bridge_post("right").translate(
        CAMERA_BRIDGE_POS.0 + CAMERA_BRIDGE_X / 2.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "cassette_quarantine_lockout_camera_evidence_bridge_beam",
        CAMERA_BRIDGE_X + 34.0,
        CAMERA_BRIDGE_Y,
        24.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z + 12.0,
    );

    left_post + right_post + beam + camera_mounts() + evidence_fiducials() + evidence_light_bars()
}

fn bridge_post(name: &str) -> Part {
    let post = centered_cube(
        format!("cassette_quarantine_lockout_camera_bridge_{name}_post"),
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    );
    let service_slot = centered_cube(
        format!("cassette_quarantine_lockout_camera_bridge_{name}_service_slot"),
        12.0,
        CAMERA_BRIDGE_Y + 2.0,
        CAMERA_BRIDGE_Z - 44.0,
    )
    .translate(0.0, 0.0, 4.0);
    let mount_bore = centered_cylinder(
        format!("cassette_quarantine_lockout_camera_bridge_{name}_mount_bore"),
        3.2,
        CAMERA_BRIDGE_Y + 4.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, -CAMERA_BRIDGE_Z / 2.0 + 24.0);

    post - service_slot - mount_bore
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("cassette_quarantine_lockout_camera_mounts");
    for i in 0..CAMERA_MOUNTS {
        mounts = mounts
            + centered_cube(
                format!("cassette_quarantine_lockout_camera_mount_plate_{i}"),
                66.0,
                12.0,
                36.0,
            )
            .translate(
                CAMERA_BRIDGE_POS.0 - 315.0 + i as f64 * 210.0,
                CAMERA_BRIDGE_POS.1 - CAMERA_BRIDGE_Y / 2.0 - 7.0,
                DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 28.0,
            );
    }
    mounts
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("cassette_quarantine_lockout_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = if i < 4 {
            -510.0 + i as f64 * 340.0
        } else {
            -510.0 + (i - 4) as f64 * 340.0
        };
        let y = if i < 4 { -365.0 } else { 365.0 };
        let disk = centered_cylinder(
            format!("cassette_quarantine_lockout_evidence_fiducial_disk_{i}"),
            9.0,
            4.0,
            36,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.0);
        let bore = centered_cylinder(
            format!("cassette_quarantine_lockout_evidence_fiducial_center_{i}"),
            2.4,
            6.0,
            20,
        )
        .translate(x, y, DECK_Z / 2.0 + 2.0);
        fiducials = fiducials + (disk - bore);
    }
    fiducials
}

fn evidence_light_bars() -> Part {
    let front = centered_cube(
        "cassette_quarantine_lockout_evidence_front_light_bar_placeholder",
        CAMERA_BRIDGE_X - 160.0,
        10.0,
        12.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1 - CAMERA_BRIDGE_Y / 2.0 - 10.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 54.0,
    );
    let rear = centered_cube(
        "cassette_quarantine_lockout_evidence_rear_light_bar_placeholder",
        CAMERA_BRIDGE_X - 160.0,
        10.0,
        12.0,
    )
    .translate(
        CAMERA_BRIDGE_POS.0,
        CAMERA_BRIDGE_POS.1 + CAMERA_BRIDGE_Y / 2.0 + 10.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_Z - 54.0,
    );
    front + rear
}

fn robot_gripper_approach_gauges() -> Part {
    let front_sweep = centered_cube(
        "cassette_quarantine_lockout_front_robot_gripper_approach_gauge",
        ROBOT_GAUGE_X,
        ROBOT_GAUGE_Y,
        ROBOT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_GAUGE_Y / 2.0,
        ROBOT_GAUGE_Z / 2.0,
    );

    front_sweep + gripper_pair_gauges() + vertical_pick_height_gauge()
}

fn gripper_pair_gauges() -> Part {
    let mut gauges = Part::empty("cassette_quarantine_lockout_gripper_pair_gauges");
    for i in 0..GRIPPER_APPROACH_GAUGE_PAIRS {
        let center_x = -390.0 + i as f64 * 390.0;
        let left = centered_cube(
            format!("cassette_quarantine_lockout_gripper_pair_{i}_left_gauge"),
            10.0,
            190.0,
            42.0,
        )
        .translate(
            center_x - GRIPPER_LANE_GAP / 2.0,
            -DECK_Y / 2.0 + 78.0,
            DECK_Z / 2.0 + 21.0,
        );
        let right = centered_cube(
            format!("cassette_quarantine_lockout_gripper_pair_{i}_right_gauge"),
            10.0,
            190.0,
            42.0,
        )
        .translate(
            center_x + GRIPPER_LANE_GAP / 2.0,
            -DECK_Y / 2.0 + 78.0,
            DECK_Z / 2.0 + 21.0,
        );
        gauges = gauges + left + right;
    }
    gauges
}

fn vertical_pick_height_gauge() -> Part {
    centered_cube(
        "cassette_quarantine_lockout_robot_vertical_pick_height_gauge",
        CASSETTE_DOCK_X + LID_RECEIVER_X + 120.0,
        12.0,
        10.0,
    )
    .translate(
        -95.0,
        CASSETTE_DOCK_POS.1 - CASSETTE_DOCK_Y / 2.0 - 24.0,
        DECK_Z / 2.0 + CASSETTE_DOCK_Z + 48.0,
    )
}

fn service_keepouts() -> Part {
    let left = centered_cube(
        "cassette_quarantine_lockout_left_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        -DECK_X / 2.0 - SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let right = centered_cube(
        "cassette_quarantine_lockout_right_service_keepout",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        DECK_X / 2.0 + SERVICE_KEEPOUT_X / 2.0,
        0.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let rear = centered_cube(
        "cassette_quarantine_lockout_rear_service_keepout",
        REAR_SERVICE_KEEPOUT_X,
        REAR_SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEPOUT_Y / 2.0,
        SERVICE_KEEPOUT_Z / 2.0,
    );
    let top = centered_cube(
        "cassette_quarantine_lockout_top_service_clearance_keepout",
        DECK_X - 180.0,
        DECK_Y - 160.0,
        12.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);

    left + right + rear + top
}

fn station_assembly() -> Part {
    containment_deck()
        + suspect_cassette_dock()
        + quarantine_lid_receiver()
        + lockout_pin_gate_witness()
        + tamper_evident_seal_lands()
        + barcode_rfid_custody_lands()
        + pressure_decay_leak_witness_ports()
        + wipe_contact_coupon_pockets()
        + release_hold_reject_lanes()
        + camera_evidence_bridge()
        + robot_gripper_approach_gauges()
        + service_keepouts()
}

fn datum_positions() -> [(f64, f64); DATUM_BOSSES] {
    [
        (-560.0, -370.0),
        (-185.0, -370.0),
        (185.0, -370.0),
        (560.0, -370.0),
        (-560.0, 370.0),
        (-185.0, 370.0),
        (185.0, 370.0),
        (560.0, 370.0),
    ]
}

fn cassette_pin_offsets() -> [(f64, f64); CASSETTE_DATUM_PINS] {
    [
        (-142.0, -70.0),
        (142.0, -70.0),
        (-142.0, 70.0),
        (142.0, 70.0),
    ]
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn recess_z(module_height: f64, recess_depth: f64) -> f64 {
    DECK_Z / 2.0 + module_height - recess_depth / 2.0 + 0.4
}

fn rim_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn pressure_port_span() -> f64 {
    (PRESSURE_DECAY_PORTS as f64 - 1.0) * PRESSURE_PORT_PITCH + PRESSURE_PORT_COLLAR_D
}

fn contact_coupon_span() -> f64 {
    (CONTACT_COUPON_POCKETS as f64 - 1.0) * 45.0 + CONTACT_COUPON_D
}

fn camera_bridge_clearance() -> f64 {
    CAMERA_BRIDGE_Z - CASSETTE_DOCK_Z.max(LID_RECEIVER_Z).max(LOCKOUT_GATE_Z)
}

fn gate_witness_name(index: usize) -> &'static str {
    match index {
        0 => "pin_present",
        1 => "gate_closed",
        2 => "robot_locked_out",
        _ => panic!("unknown gate witness index"),
    }
}

fn disposition_lane_name(index: usize) -> &'static str {
    match index {
        RELEASE_LANE_INDEX => "release",
        HOLD_LANE_INDEX => "hold",
        REJECT_LANE_INDEX => "reject",
        _ => panic!("unknown disposition lane index"),
    }
}

fn module_rects() -> [Rect; 8] {
    [
        cassette_dock_rect(),
        lid_receiver_rect(),
        lockout_gate_rect(),
        pressure_bar_rect(),
        coupon_bank_rect(),
        custody_panel_rect(),
        tamper_panel_rect(),
        lane_bank_rect(),
    ]
}

fn cassette_dock_rect() -> Rect {
    Rect {
        x: CASSETTE_DOCK_POS.0,
        y: CASSETTE_DOCK_POS.1,
        w: CASSETTE_DOCK_X,
        h: CASSETTE_DOCK_Y,
    }
}

fn lid_receiver_rect() -> Rect {
    Rect {
        x: LID_RECEIVER_POS.0,
        y: LID_RECEIVER_POS.1,
        w: LID_RECEIVER_X,
        h: LID_RECEIVER_Y,
    }
}

fn lockout_gate_rect() -> Rect {
    Rect {
        x: LOCKOUT_GATE_POS.0,
        y: LOCKOUT_GATE_POS.1,
        w: LOCKOUT_GATE_X,
        h: LOCKOUT_GATE_Y,
    }
}

fn pressure_bar_rect() -> Rect {
    Rect {
        x: PRESSURE_BAR_POS.0,
        y: PRESSURE_BAR_POS.1,
        w: PRESSURE_BAR_X,
        h: PRESSURE_BAR_Y,
    }
}

fn coupon_bank_rect() -> Rect {
    Rect {
        x: COUPON_BANK_POS.0,
        y: COUPON_BANK_POS.1,
        w: COUPON_BANK_X,
        h: COUPON_BANK_Y,
    }
}

fn custody_panel_rect() -> Rect {
    Rect {
        x: CUSTODY_PANEL_POS.0,
        y: CUSTODY_PANEL_POS.1,
        w: CUSTODY_PANEL_X,
        h: CUSTODY_PANEL_Y,
    }
}

fn tamper_panel_rect() -> Rect {
    Rect {
        x: TAMPER_PANEL_POS.0,
        y: TAMPER_PANEL_POS.1,
        w: TAMPER_PANEL_X,
        h: TAMPER_PANEL_Y,
    }
}

fn lane_bank_rect() -> Rect {
    Rect {
        x: LANE_BANK_POS.0,
        y: LANE_BANK_POS.1,
        w: LANE_BANK_X,
        h: LANE_BANK_Y,
    }
}

fn rect_fits_on_deck(rect: Rect, margin: f64) -> bool {
    rect.x - rect.w / 2.0 > -DECK_X / 2.0 + margin
        && rect.x + rect.w / 2.0 < DECK_X / 2.0 - margin
        && rect.y - rect.h / 2.0 > -DECK_Y / 2.0 + margin
        && rect.y + rect.h / 2.0 < DECK_Y / 2.0 - margin
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

fn vertical_gap(a: Rect, b: Rect) -> f64 {
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    if ay1 < by0 {
        by0 - ay1
    } else if by1 < ay0 {
        ay0 - by1
    } else {
        0.0
    }
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 13, "unexpected output count");
    assert_eq!(DATUM_BOSSES, 8, "datum boss count changed");
    assert!(module_rects()
        .iter()
        .all(|rect| rect_fits_on_deck(*rect, RIM_W + 16.0)));
    assert!(!rects_overlap(cassette_dock_rect(), lid_receiver_rect()));
    assert!(!rects_overlap(cassette_dock_rect(), lockout_gate_rect()));
    assert!(!rects_overlap(lid_receiver_rect(), lockout_gate_rect()));
    assert!(horizontal_gap(cassette_dock_rect(), lid_receiver_rect()) >= 120.0);
    assert!(vertical_gap(cassette_dock_rect(), lockout_gate_rect()) >= 45.0);
    assert!(pressure_port_span() < PRESSURE_BAR_X - 70.0);
    assert!(contact_coupon_span() < COUPON_BANK_X - 34.0);
    assert!(camera_bridge_clearance() >= CAMERA_MIN_CLEARANCE);
    assert_eq!(DISPOSITION_LANES, 3);
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
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
            assert!(
                path.starts_with("output/closed_robotic_cassette_quarantine_lid_lockout_station_")
            );
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_requested_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 23);
        for required in [
            "suspect_cassette_dock",
            "quarantine_lid_receiver",
            "lockout_pin_witness_bores",
            "gate_position_witness_flags",
            "tamper_evident_seal_lands",
            "barcode_lands",
            "rfid_lands",
            "pressure_decay_ports",
            "leak_witness_cups",
            "wipe_coupon_pockets",
            "contact_coupon_pockets",
            "release_lane",
            "hold_lane",
            "reject_lane",
            "camera_evidence_bridge",
            "robot_gripper_approach_gauges",
            "service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&required));
        }
    }

    #[test]
    fn scope_limits_exclude_process_or_acceptance_claims() {
        assert!(SCOPE_LIMITS.contains(&"mechanical_validation_packaging_only"));
        assert!(SCOPE_LIMITS.contains(&"not_a_sterile_process_claim"));
        assert!(SCOPE_LIMITS.contains(&"not_a_pressure_rated_device"));
        assert!(SCOPE_LIMITS.contains(&"not_a_biological_acceptance_criterion"));
        assert!(REQUIRED_FEATURES.contains(&"not_pressure_rated"));
        assert!(REQUIRED_FEATURES.contains(&"not_biological_acceptance_criterion"));
    }

    #[test]
    fn layout_guards_keep_quarantine_modules_segregated() {
        assert_layout();
        assert!(!rects_overlap(cassette_dock_rect(), lid_receiver_rect()));
        assert!(!rects_overlap(cassette_dock_rect(), lockout_gate_rect()));
        assert!(horizontal_gap(cassette_dock_rect(), lid_receiver_rect()) >= 120.0);
        assert!(vertical_gap(cassette_dock_rect(), lockout_gate_rect()) >= 45.0);
    }

    #[test]
    fn witness_counts_cover_custody_leak_coupon_and_disposition_workflow() {
        assert_eq!(CASSETTE_DATUM_PINS, 4);
        assert_eq!(LOCKOUT_PIN_COUNT, 6);
        assert_eq!(GATE_WITNESS_FLAGS, 3);
        assert_eq!(TAMPER_SEAL_LANDS, 8);
        assert_eq!(BARCODE_LANDS + RFID_LANDS + CUSTODY_CARD_SLOTS, 13);
        assert_eq!(PRESSURE_DECAY_PORTS, 4);
        assert_eq!(LEAK_WITNESS_CUPS, 6);
        assert_eq!(WIPE_COUPON_POCKETS, CONTACT_COUPON_POCKETS);
        assert_eq!(DISPOSITION_LANES, 3);
    }

    #[test]
    fn geometry_expectations_fit_robot_and_evidence_envelope() {
        assert!(pressure_port_span() < PRESSURE_BAR_X - 70.0);
        assert!(contact_coupon_span() < COUPON_BANK_X - 34.0);
        assert!(camera_bridge_clearance() >= CAMERA_MIN_CLEARANCE);
        assert!(ROBOT_GAUGE_X > CASSETTE_DOCK_X + LID_RECEIVER_X);
        assert!(GRIPPER_LANE_GAP > 90.0);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + DECK_Z / 2.0);
    }

    #[test]
    fn stable_lane_and_gate_names_match_physical_disposition() {
        assert_eq!(disposition_lane_name(RELEASE_LANE_INDEX), "release");
        assert_eq!(disposition_lane_name(HOLD_LANE_INDEX), "hold");
        assert_eq!(disposition_lane_name(REJECT_LANE_INDEX), "reject");
        assert_eq!(gate_witness_name(0), "pin_present");
        assert_eq!(gate_witness_name(1), "gate_closed");
        assert_eq!(gate_witness_name(2), "robot_locked_out");
    }
}
