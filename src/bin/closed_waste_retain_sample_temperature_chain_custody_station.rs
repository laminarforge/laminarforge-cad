use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste retain-sample temperature chain-of-custody station.
//
// Design intent:
// - Validate post-run custody for a closed waste/retain split without opening
//   contaminated fluid paths at the bench.
// - Make retain vial sealing, temperature logger custody, barcode/RFID
//   identity capture, chain-of-custody tokens, hold/release/reject disposition,
//   leak/backflow witness features, quarantine overbagging, camera evidence,
//   and robot/service keepouts physically reviewable as CAD geometry.
// - Keep the model as validation fixture packaging only. Purchased wetted
//   components, custody SOPs, thermal qualification, and release/reject
//   decisions remain external to this deterministic CAD artifact.
//
// Exa research cues used for this concept:
// - Automated bioprocess sample archives emphasize closed sampling, retention
//   sample traceability, and secure sample handling.
// - Biorepository and stability-sample workflows commonly combine barcode/RFID
//   identity, environmental logging, and custody event records.
// - Chain-of-custody failure modes include missing handoff records, lost
//   temperature evidence, and unclear quarantine/release disposition.

const OUTPUT_PREFIX: &str = "output/closed_waste_retain_sample_temperature_chain_custody_station";
const PART_PREFIX: &str = "closed_waste_retain_sample_temperature_chain_custody_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_waste_retain_sample_temperature_chain_custody_station_secondary_containment_deck.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_waste_retain_split_receiver.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_sealed_retain_vial_nests.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_temperature_logger_pockets.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_barcode_rfid_custody_lands.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_chain_of_custody_token_rail.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_hold_release_reject_gates.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_leak_backflow_witness_features.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_quarantine_overbag_interface.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_camera_evidence_bridge.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_robot_service_keepouts.stl",
    "output/closed_waste_retain_sample_temperature_chain_custody_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "waste_retain_split_receiver",
    "sealed_retain_vial_nests",
    "temperature_logger_pockets",
    "barcode_rfid_custody_lands",
    "chain_of_custody_token_rail",
    "hold_release_reject_gates",
    "leak_backflow_witness_features",
    "quarantine_overbag_interface",
    "camera_evidence_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "not_a_waste_release_rule",
    "no_thermal_acceptance_claim",
    "not_a_sterile_barrier_design",
    "purchased_wetted_components_external",
];

const STATUS_NAMES: [&str; 3] = ["hold", "release", "reject"];
const CUSTODY_STEP_NAMES: [&str; 6] = [
    "run_end",
    "split",
    "retain_seal",
    "temperature_read",
    "overbag",
    "final_disposition",
];

const STATION_X: f64 = 1460.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_Z: f64 = 5.5;
const LEAK_BASIN_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.6;

const SPLIT_POS: (f64, f64) = (-440.0, 200.0);
const SPLIT_X: f64 = 430.0;
const SPLIT_Y: f64 = 230.0;
const SPLIT_Z: f64 = 72.0;
const SPLIT_LANES: usize = 8;
const SPLIT_BRANCHES_PER_LANE: usize = 3;
const SPLIT_OUTPUT_PORTS: usize = SPLIT_LANES * SPLIT_BRANCHES_PER_LANE;
const SPLIT_LANE_PITCH: f64 = 46.0;
const WASTE_INLET_PORT_D: f64 = 13.0;
const RETAIN_PORT_D: f64 = 7.2;
const BULKHEAD_LATCHES: usize = 6;

const VIAL_POS: (f64, f64) = (-440.0, -130.0);
const VIAL_NEST_X: f64 = 430.0;
const VIAL_NEST_Y: f64 = 260.0;
const VIAL_NEST_Z: f64 = 58.0;
const RETAIN_VIAL_ROWS: usize = 4;
const RETAIN_VIAL_COLS: usize = 6;
const RETAIN_VIALS: usize = RETAIN_VIAL_ROWS * RETAIN_VIAL_COLS;
const VIAL_PITCH_X: f64 = 54.0;
const VIAL_PITCH_Y: f64 = 48.0;
const VIAL_BORE_D: f64 = 18.2;
const VIAL_SEAL_RINGS: usize = RETAIN_VIALS;
const VIAL_NEST_LATCHES: usize = 8;

const LOGGER_POS: (f64, f64) = (-430.0, -360.0);
const LOGGER_PANEL_X: f64 = 320.0;
const LOGGER_PANEL_Y: f64 = 116.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_POCKETS: usize = 4;
const LOGGER_SLOT_X: f64 = 52.0;
const LOGGER_SLOT_Y: f64 = 78.0;
const LOGGER_SLOT_DEPTH: f64 = 11.0;
const LOGGER_SLOT_PITCH: f64 = 70.0;
const LOGGER_SEAL_WELLS: usize = 8;
const LOGGER_CONTACTS_PER_POCKET: usize = 3;
const LOGGER_CONTACT_PINS: usize = LOGGER_POCKETS * LOGGER_CONTACTS_PER_POCKET;

const CUSTODY_POS: (f64, f64) = (390.0, 230.0);
const CUSTODY_X: f64 = 450.0;
const CUSTODY_Y: f64 = 180.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 8;
const RUN_CARD_LANDS: usize = 4;
const TAMPER_SEAL_WELLS: usize = 8;

const TOKEN_POS: (f64, f64) = (0.0, 385.0);
const TOKEN_RAIL_X: f64 = 860.0;
const TOKEN_RAIL_Y: f64 = 74.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const TOKEN_SLOTS_PER_STEP: usize = STATUS_NAMES.len();
const CUSTODY_TOKEN_SLOTS: usize = CUSTODY_STEP_NAMES.len() * TOKEN_SLOTS_PER_STEP;
const TOKEN_SLOT_X: f64 = 34.0;
const TOKEN_SLOT_Y: f64 = 18.0;
const TOKEN_STEP_PITCH: f64 = 126.0;
const TOKEN_STATUS_PITCH: f64 = 24.0;
const TOKEN_TICK_MARKS: usize = CUSTODY_STEP_NAMES.len() + 1;

const GATE_POS: (f64, f64) = (410.0, -55.0);
const GATE_PANEL_X: f64 = 480.0;
const GATE_PANEL_Y: f64 = 230.0;
const GATE_PANEL_Z: f64 = 46.0;
const GATE_LANES: usize = STATUS_NAMES.len();
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SLOT_X: f64 = 82.0;
const GATE_SLOT_Y: f64 = 40.0;
const GATE_LANE_PITCH: f64 = 140.0;
const GATE_SLOT_PITCH_Y: f64 = 50.0;
const GATE_LOCK_PINS: usize = GATE_LANES * 2;
const GATE_MIN_GAP: f64 = GATE_LANE_PITCH - GATE_SLOT_X;

const WITNESS_POS: (f64, f64) = (-20.0, 40.0);
const WITNESS_X: f64 = 360.0;
const WITNESS_Y: f64 = 120.0;
const WITNESS_Z: f64 = 32.0;
const BACKFLOW_CHANNELS: usize = SPLIT_LANES;
const LEAK_WITNESS_WINDOWS: usize = 6;
const WITNESS_PAD_COUNT: usize = 10;
const DRY_CONTROL_PADS: usize = 2;

const OVERBAG_POS: (f64, f64) = (385.0, -335.0);
const OVERBAG_X: f64 = 430.0;
const OVERBAG_Y: f64 = 132.0;
const OVERBAG_Z: f64 = 44.0;
const OVERBAG_CLAMPS: usize = 6;
const OVERBAG_SEAM_WITNESS_COUPONS: usize = 8;
const OVERBAG_PURGE_PORTS: usize = 2;

const BRIDGE_POS: (f64, f64) = (0.0, 26.0);
const BRIDGE_SPAN_X: f64 = 1240.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_UNDERSIDE_Z: f64 = 238.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const BRIDGE_TOTAL_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const CAMERA_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 6;

const KEEP_OUT_Z: f64 = 94.0;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 225.0;
const LEFT_SPLIT_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_OVERBAG_SERVICE_CLEARANCE: f64 = 240.0;
const OVERHEAD_CAMERA_SERVICE_Z: f64 = 318.0;
const KEEP_OUT_ZONES: usize = 5;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let split_receiver = waste_retain_split_receiver();
    export(OUTPUTS[1], &split_receiver);

    let vial_nests = sealed_retain_vial_nests();
    export(OUTPUTS[2], &vial_nests);

    let loggers = temperature_logger_pockets();
    export(OUTPUTS[3], &loggers);

    let custody = barcode_rfid_custody_lands();
    export(OUTPUTS[4], &custody);

    let token_rail = chain_of_custody_token_rail();
    export(OUTPUTS[5], &token_rail);

    let gates = hold_release_reject_gates();
    export(OUTPUTS[6], &gates);

    let witness = leak_backflow_witness_features();
    export(OUTPUTS[7], &witness);

    let overbag = quarantine_overbag_interface();
    export(OUTPUTS[8], &overbag);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + split_receiver.translate(SPLIT_POS.0, SPLIT_POS.1, deck_z(SPLIT_Z))
        + vial_nests.translate(VIAL_POS.0, VIAL_POS.1, deck_z(VIAL_NEST_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, deck_z(LOGGER_PANEL_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, deck_z(CUSTODY_Z))
        + token_rail.translate(TOKEN_POS.0, TOKEN_POS.1, deck_z(TOKEN_RAIL_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, deck_z(GATE_PANEL_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, deck_z(WITNESS_Z))
        + overbag.translate(OVERBAG_POS.0, OVERBAG_POS.1, deck_z(OVERBAG_Z))
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, deck_z(BRIDGE_TOTAL_Z))
        + keepouts.translate(0.0, 0.0, deck_z(KEEP_OUT_Z));
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed waste retain-sample temperature chain-of-custody station:");
    println!(
        "  Footprint:                {STATION_X:.0}mm x {STATION_Y:.0}mm secondary-containment deck with {LEAK_BASIN_DEPTH:.0}mm witness basin and {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Waste/retain split:       {SPLIT_LANES} closed split lanes, {SPLIT_OUTPUT_PORTS} output witness ports, {BULKHEAD_LATCHES} bulkhead latch lands"
    );
    println!(
        "  Retain custody:           {RETAIN_VIALS} sealed vial pockets, {VIAL_SEAL_RINGS} seal rings, {VIAL_NEST_LATCHES} nest latches"
    );
    println!(
        "  Temperature evidence:     {LOGGER_POCKETS} logger pockets, {LOGGER_SEAL_WELLS} seal wells, {LOGGER_CONTACT_PINS} contact-pin lands"
    );
    println!(
        "  Identity/custody:         {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {RUN_CARD_LANDS} run-card lands, {CUSTODY_TOKEN_SLOTS} custody token slots"
    );
    println!(
        "  Disposition gates:        {:?} lanes, {GATE_SLOTS_PER_LANE} positions per lane, {GATE_LOCK_PINS} lock-pin lands",
        STATUS_NAMES
    );
    println!(
        "  Leak/backflow/overbag:    {BACKFLOW_CHANNELS} backflow channels, {LEAK_WITNESS_WINDOWS} witness windows, {OVERBAG_CLAMPS} overbag clamps, {OVERBAG_SEAM_WITNESS_COUPONS} seam coupons"
    );
    println!(
        "  Evidence/keepouts:        {CAMERA_COUNT} camera pods, {LIGHT_BAR_COUNT} light bars, {KEEP_OUT_ZONES} robot/service keepout zones, {} required feature groups, {} scope limitations",
        REQUIRED_FEATURES.len(),
        LIMITATIONS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0 + 4.0
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PART_PREFIX}_secondary_containment_deck_floor"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        format!("{PART_PREFIX}_shallow_leak_basin_recess"),
        STATION_X - 116.0,
        STATION_Y - 122.0,
        LEAK_BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -10.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.3);
    let drain_channel = centered_cube(
        format!("{PART_PREFIX}_front_witness_drain_channel"),
        STATION_X - 210.0,
        24.0,
        LEAK_BASIN_DEPTH + 3.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 68.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        format!("{PART_PREFIX}_secondary_containment_drain_port"),
        DRAIN_D / 2.0,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 34.0, 0.0);

    deck - basin - drain_channel - drain - fixture_sockets() - mount_holes()
        + perimeter_rims()
        + deck_route_rails()
        + deck_fiducials()
}

fn fixture_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PART_PREFIX}_fixture_sockets"));
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{PART_PREFIX}_{}_socket", footprint.name),
                footprint.x + 10.0,
                footprint.y + 10.0,
                SOCKET_Z + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_Z / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{PART_PREFIX}_mount_holes"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 52.0),
        (0.0, -STATION_Y / 2.0 + 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (SPLIT_POS.0 - SPLIT_X / 2.0 + 44.0, SPLIT_POS.1),
        (OVERBAG_POS.0 + OVERBAG_X / 2.0 - 44.0, OVERBAG_POS.1),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PART_PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("{PART_PREFIX}_m6_mount_slot_{i}"),
                30.0,
                MOUNT_HOLE_D + 0.8,
                BASE_Z + 8.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let front = centered_cube(
        format!("{PART_PREFIX}_front_spill_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_spill_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_split_service_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_overbag_service_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn deck_route_rails() -> Part {
    let split_to_retain = centered_cube(
        format!("{PART_PREFIX}_closed_split_to_retain_route_rail"),
        18.0,
        214.0,
        12.0,
    )
    .translate(-440.0, 34.0, BASE_Z / 2.0 + 6.0);
    let split_to_witness = centered_cube(
        format!("{PART_PREFIX}_split_to_backflow_witness_route_rail"),
        300.0,
        12.0,
        12.0,
    )
    .translate(-230.0, 104.0, BASE_Z / 2.0 + 6.0);
    let witness_to_gate = centered_cube(
        format!("{PART_PREFIX}_witness_to_disposition_gate_route_rail"),
        410.0,
        12.0,
        12.0,
    )
    .translate(205.0, 10.0, BASE_Z / 2.0 + 6.0);
    let gate_to_overbag = centered_cube(
        format!("{PART_PREFIX}_reject_gate_to_quarantine_overbag_route_rail"),
        18.0,
        210.0,
        12.0,
    )
    .translate(410.0, -205.0, BASE_Z / 2.0 + 6.0);
    split_to_retain + split_to_witness + witness_to_gate + gate_to_overbag
}

fn deck_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PART_PREFIX}_robot_datum_fiducials"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 88.0, STATION_Y / 2.0 - 84.0),
        (STATION_X / 2.0 - 88.0, STATION_Y / 2.0 - 84.0),
        (-STATION_X / 2.0 + 88.0, -STATION_Y / 2.0 + 118.0),
        (STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 + 118.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + (centered_cylinder(
                format!("{PART_PREFIX}_fiducial_outer_ring_{i}"),
                16.0,
                4.0,
                40,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 2.0)
                - centered_cylinder(
                    format!("{PART_PREFIX}_fiducial_center_bore_{i}"),
                    5.0,
                    6.0,
                    24,
                )
                .translate(*x, *y, BASE_Z / 2.0 + 2.0));
    }
    fiducials
}

fn waste_retain_split_receiver() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_waste_retain_split_receiver_body"),
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    );
    let gasket_land = rectangular_frame_xy(
        format!("{PART_PREFIX}_split_receiver_compression_gasket"),
        SPLIT_X - 36.0,
        SPLIT_Y - 32.0,
        8.0,
        SPLIT_X - 86.0,
        SPLIT_Y - 82.0,
    )
    .translate(0.0, 0.0, SPLIT_Z / 2.0 + 4.0);

    let mut cuts = Part::empty(format!("{PART_PREFIX}_split_receiver_port_cuts"));
    let mut features = Part::empty(format!("{PART_PREFIX}_split_receiver_features"));
    for lane in 0..SPLIT_LANES {
        let x = centered_index(lane, SPLIT_LANES, SPLIT_LANE_PITCH);
        cuts = cuts
            + centered_cylinder(
                format!("{PART_PREFIX}_split_lane_{lane}_sealed_waste_inlet_bore"),
                WASTE_INLET_PORT_D / 2.0,
                SPLIT_Y + 10.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -26.0, 8.0);

        for branch in 0..SPLIT_BRANCHES_PER_LANE {
            let y = centered_index(branch, SPLIT_BRANCHES_PER_LANE, 46.0) + 24.0;
            cuts = cuts
                + centered_cylinder(
                    format!("{PART_PREFIX}_split_lane_{lane}_branch_{branch}_retain_waste_port"),
                    RETAIN_PORT_D / 2.0,
                    SPLIT_Y + 8.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, -6.0);
        }

        features = features
            + centered_cube(
                format!("{PART_PREFIX}_split_lane_{lane}_valve_status_land"),
                34.0,
                18.0,
                10.0,
            )
            .translate(x, -SPLIT_Y / 2.0 + 24.0, SPLIT_Z / 2.0 + 5.0)
            + centered_cube(
                format!("{PART_PREFIX}_split_lane_{lane}_retain_branch_label_land"),
                34.0,
                16.0,
                8.0,
            )
            .translate(x, SPLIT_Y / 2.0 - 28.0, SPLIT_Z / 2.0 + 4.0);
    }

    body - cuts + gasket_land + split_receiver_latch_lands() + split_receiver_cap_parks() + features
}

fn split_receiver_latch_lands() -> Part {
    let mut latches = Part::empty(format!("{PART_PREFIX}_split_receiver_latch_lands"));
    for i in 0..BULKHEAD_LATCHES {
        let x = centered_index(i, BULKHEAD_LATCHES, 64.0);
        latches = latches
            + centered_cube(
                format!("{PART_PREFIX}_bulkhead_latch_land_{i}"),
                42.0,
                20.0,
                14.0,
            )
            .translate(x, -SPLIT_Y / 2.0 - 12.0, SPLIT_Z / 2.0 + 7.0);
    }
    latches
}

fn split_receiver_cap_parks() -> Part {
    let mut parks = Part::empty(format!("{PART_PREFIX}_split_receiver_cap_parks"));
    for i in 0..SPLIT_LANES {
        let x = centered_index(i, SPLIT_LANES, SPLIT_LANE_PITCH);
        let post = centered_cylinder(
            format!("{PART_PREFIX}_split_lane_{i}_cap_park_post"),
            8.5,
            22.0,
            28,
        )
        .translate(x, SPLIT_Y / 2.0 + 16.0, SPLIT_Z / 2.0 + 11.0);
        let tether_slot = centered_cube(
            format!("{PART_PREFIX}_split_lane_{i}_cap_tether_slot"),
            4.0,
            18.0,
            9.0,
        )
        .translate(x, SPLIT_Y / 2.0 + 16.0, SPLIT_Z / 2.0 + 11.0);
        parks = parks + (post - tether_slot);
    }
    parks
}

fn sealed_retain_vial_nests() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_sealed_retain_vial_nest_block"),
        VIAL_NEST_X,
        VIAL_NEST_Y,
        VIAL_NEST_Z,
    );
    let mut bores = Part::empty(format!("{PART_PREFIX}_sealed_retain_vial_bores"));
    let mut features = Part::empty(format!("{PART_PREFIX}_sealed_retain_vial_features"));
    for row in 0..RETAIN_VIAL_ROWS {
        for col in 0..RETAIN_VIAL_COLS {
            let i = row * RETAIN_VIAL_COLS + col;
            let x = centered_index(col, RETAIN_VIAL_COLS, VIAL_PITCH_X);
            let y = centered_index(row, RETAIN_VIAL_ROWS, VIAL_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("{PART_PREFIX}_retain_vial_bore_{i}"),
                    VIAL_BORE_D / 2.0,
                    VIAL_NEST_Z + 8.0,
                    32,
                )
                .translate(x, y, 0.0);
            features = features
                + (centered_cylinder(
                    format!("{PART_PREFIX}_retain_vial_seal_ring_{i}"),
                    13.5,
                    6.0,
                    36,
                )
                .translate(x, y, VIAL_NEST_Z / 2.0 + 3.0)
                    - centered_cylinder(
                        format!("{PART_PREFIX}_retain_vial_seal_center_clearance_{i}"),
                        VIAL_BORE_D / 2.0,
                        8.0,
                        32,
                    )
                    .translate(x, y, VIAL_NEST_Z / 2.0 + 3.0));
        }
    }

    body - bores + features + vial_nest_latch_tabs() + vial_nest_row_label_lands()
}

fn vial_nest_latch_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PART_PREFIX}_vial_nest_latch_tabs"));
    for i in 0..VIAL_NEST_LATCHES {
        let x = centered_index(i % 4, 4, 92.0);
        let y = if i < 4 {
            -VIAL_NEST_Y / 2.0 - 10.0
        } else {
            VIAL_NEST_Y / 2.0 + 10.0
        };
        tabs = tabs
            + centered_cube(
                format!("{PART_PREFIX}_vial_nest_latch_tab_{i}"),
                54.0,
                18.0,
                16.0,
            )
            .translate(x, y, VIAL_NEST_Z / 2.0 + 8.0);
    }
    tabs
}

fn vial_nest_row_label_lands() -> Part {
    let mut lands = Part::empty(format!("{PART_PREFIX}_vial_nest_row_label_lands"));
    for row in 0..RETAIN_VIAL_ROWS {
        let y = centered_index(row, RETAIN_VIAL_ROWS, VIAL_PITCH_Y);
        lands = lands
            + centered_cube(
                format!("{PART_PREFIX}_retain_vial_row_{row}_custody_label_land"),
                44.0,
                16.0,
                5.0,
            )
            .translate(-VIAL_NEST_X / 2.0 + 30.0, y, VIAL_NEST_Z / 2.0 + 2.5);
    }
    lands
}

fn temperature_logger_pockets() -> Part {
    let body = centered_cube(
        format!("{PART_PREFIX}_temperature_logger_pocket_panel"),
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{PART_PREFIX}_temperature_logger_pocket_cuts"));
    let mut features = Part::empty(format!("{PART_PREFIX}_temperature_logger_pocket_features"));
    for logger in 0..LOGGER_POCKETS {
        let x = centered_index(logger, LOGGER_POCKETS, LOGGER_SLOT_PITCH);
        cuts = cuts
            + centered_cube(
                format!("{PART_PREFIX}_logger_{logger}_sealed_pocket_cut"),
                LOGGER_SLOT_X,
                LOGGER_SLOT_Y,
                LOGGER_SLOT_DEPTH + 1.0,
            )
            .translate(x, 0.0, LOGGER_PANEL_Z / 2.0 - LOGGER_SLOT_DEPTH / 2.0 + 0.5);

        for pin in 0..LOGGER_CONTACTS_PER_POCKET {
            features = features
                + centered_cylinder(
                    format!("{PART_PREFIX}_logger_{logger}_contact_pin_land_{pin}"),
                    2.2,
                    5.0,
                    18,
                )
                .translate(
                    x + centered_index(pin, LOGGER_CONTACTS_PER_POCKET, 10.0),
                    LOGGER_SLOT_Y / 2.0 + 13.0,
                    LOGGER_PANEL_Z / 2.0 + 2.5,
                );
        }
    }

    body - cuts + features + logger_seal_wells() + logger_pull_tabs()
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_logger_tamper_seal_wells"));
    for i in 0..LOGGER_SEAL_WELLS {
        let x = centered_index(i % 4, 4, LOGGER_SLOT_PITCH);
        let y = if i < 4 {
            -LOGGER_PANEL_Y / 2.0 + 16.0
        } else {
            LOGGER_PANEL_Y / 2.0 - 16.0
        };
        wells = wells
            + (centered_cylinder(format!("{PART_PREFIX}_logger_seal_ring_{i}"), 8.0, 6.0, 24)
                .translate(x, y, LOGGER_PANEL_Z / 2.0 + 3.0)
                - centered_cylinder(format!("{PART_PREFIX}_logger_seal_bore_{i}"), 4.2, 8.0, 20)
                    .translate(x, y, LOGGER_PANEL_Z / 2.0 + 3.0));
    }
    wells
}

fn logger_pull_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PART_PREFIX}_logger_pull_tab_lands"));
    for logger in 0..LOGGER_POCKETS {
        let x = centered_index(logger, LOGGER_POCKETS, LOGGER_SLOT_PITCH);
        tabs = tabs
            + centered_cube(
                format!("{PART_PREFIX}_logger_{logger}_chain_pull_tab_land"),
                44.0,
                12.0,
                8.0,
            )
            .translate(x, -LOGGER_PANEL_Y / 2.0 - 8.0, LOGGER_PANEL_Z / 2.0 + 4.0);
    }
    tabs
}

fn barcode_rfid_custody_lands() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_barcode_rfid_custody_plate"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty(format!("{PART_PREFIX}_barcode_rfid_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("{PART_PREFIX}_barcode_land_{i}"), 74.0, 16.0, 5.0).translate(
                centered_index(i % 6, 6, 68.0),
                -CUSTODY_Y / 2.0 + 30.0 + (i / 6) as f64 * 26.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    for i in 0..RFID_LANDS {
        lands = lands
            + (centered_cube(
                format!("{PART_PREFIX}_rfid_tag_well_ring_{i}"),
                34.0,
                24.0,
                6.0,
            )
            .translate(
                centered_index(i % 4, 4, 78.0),
                CUSTODY_Y / 2.0 - 42.0 - (i / 4) as f64 * 30.0,
                CUSTODY_Z / 2.0 + 3.0,
            ) - centered_cube(
                format!("{PART_PREFIX}_rfid_tag_well_cut_{i}"),
                22.0,
                14.0,
                8.0,
            )
            .translate(
                centered_index(i % 4, 4, 78.0),
                CUSTODY_Y / 2.0 - 42.0 - (i / 4) as f64 * 30.0,
                CUSTODY_Z / 2.0 + 3.0,
            ));
    }
    plate + lands + run_card_lands() + tamper_seal_wells()
}

fn run_card_lands() -> Part {
    let mut cards = Part::empty(format!("{PART_PREFIX}_run_card_lands"));
    for i in 0..RUN_CARD_LANDS {
        cards = cards
            + centered_cube(format!("{PART_PREFIX}_run_card_land_{i}"), 72.0, 38.0, 5.0).translate(
                -CUSTODY_X / 2.0 + 52.0,
                centered_index(i, RUN_CARD_LANDS, 38.0),
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    cards
}

fn tamper_seal_wells() -> Part {
    let mut wells = Part::empty(format!("{PART_PREFIX}_custody_tamper_seal_wells"));
    for i in 0..TAMPER_SEAL_WELLS {
        wells = wells
            + (centered_cylinder(
                format!("{PART_PREFIX}_tamper_seal_well_ring_{i}"),
                9.0,
                5.0,
                24,
            )
            .translate(
                CUSTODY_X / 2.0 - 34.0,
                centered_index(i, TAMPER_SEAL_WELLS, 18.0),
                CUSTODY_Z / 2.0 + 2.5,
            ) - centered_cylinder(
                format!("{PART_PREFIX}_tamper_seal_well_cut_{i}"),
                4.5,
                7.0,
                18,
            )
            .translate(
                CUSTODY_X / 2.0 - 34.0,
                centered_index(i, TAMPER_SEAL_WELLS, 18.0),
                CUSTODY_Z / 2.0 + 2.5,
            ));
    }
    wells
}

fn chain_of_custody_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PART_PREFIX}_chain_of_custody_token_rail_body"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let mut slots = Part::empty(format!("{PART_PREFIX}_chain_of_custody_token_slots"));
    let mut labels = Part::empty(format!("{PART_PREFIX}_chain_of_custody_step_label_lands"));
    for step in 0..CUSTODY_STEP_NAMES.len() {
        let step_x = centered_index(step, CUSTODY_STEP_NAMES.len(), TOKEN_STEP_PITCH);
        labels = labels
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_custody_step_label_land",
                    CUSTODY_STEP_NAMES[step]
                ),
                86.0,
                10.0,
                5.0,
            )
            .translate(step_x, -TOKEN_RAIL_Y / 2.0 - 8.0, TOKEN_RAIL_Z / 2.0 + 2.5);

        for status in 0..STATUS_NAMES.len() {
            let y = centered_index(status, STATUS_NAMES.len(), TOKEN_STATUS_PITCH);
            slots = slots
                + centered_cube(
                    format!(
                        "{PART_PREFIX}_{}_{}_token_slot",
                        CUSTODY_STEP_NAMES[step], STATUS_NAMES[status]
                    ),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    9.0,
                )
                .translate(step_x, y, TOKEN_RAIL_Z / 2.0 - 4.5);
        }
    }
    rail - slots + labels + custody_tick_marks()
}

fn custody_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PART_PREFIX}_custody_timeline_tick_marks"));
    for i in 0..TOKEN_TICK_MARKS {
        ticks = ticks
            + centered_cube(
                format!("{PART_PREFIX}_custody_tick_mark_{i}"),
                6.0,
                TOKEN_RAIL_Y + 16.0,
                7.0,
            )
            .translate(
                centered_index(i, TOKEN_TICK_MARKS, TOKEN_STEP_PITCH),
                0.0,
                TOKEN_RAIL_Z / 2.0 + 3.5,
            );
    }
    ticks
}

fn hold_release_reject_gates() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_hold_release_reject_gate_panel"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{PART_PREFIX}_hold_release_reject_slot_cuts"));
    let mut features = Part::empty(format!("{PART_PREFIX}_hold_release_reject_gate_features"));
    for lane in 0..GATE_LANES {
        let x = status_lane_x(lane);
        for slot in 0..GATE_SLOTS_PER_LANE {
            let y = centered_index(slot, GATE_SLOTS_PER_LANE, GATE_SLOT_PITCH_Y);
            cuts = cuts
                + centered_cube(
                    format!("{PART_PREFIX}_{}_gate_slot_{slot}", STATUS_NAMES[lane]),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    12.0,
                )
                .translate(x, y, GATE_PANEL_Z / 2.0 - 6.0);
        }
        features = features
            + centered_cube(
                format!(
                    "{PART_PREFIX}_{}_sliding_gate_shutter_land",
                    STATUS_NAMES[lane]
                ),
                GATE_SLOT_X + 34.0,
                GATE_PANEL_Y - 26.0,
                10.0,
            )
            .translate(x, 0.0, GATE_PANEL_Z / 2.0 + 5.0)
            + centered_cube(
                format!("{PART_PREFIX}_{}_status_label_land", STATUS_NAMES[lane]),
                92.0,
                18.0,
                8.0,
            )
            .translate(x, GATE_PANEL_Y / 2.0 + 12.0, GATE_PANEL_Z / 2.0 + 4.0);
    }
    panel - cuts + features + gate_lock_pin_lands()
}

fn gate_lock_pin_lands() -> Part {
    let mut pins = Part::empty(format!("{PART_PREFIX}_gate_lock_pin_lands"));
    for i in 0..GATE_LOCK_PINS {
        let lane = i / 2;
        let x = status_lane_x(lane);
        let y = if i % 2 == 0 {
            -GATE_PANEL_Y / 2.0 + 22.0
        } else {
            GATE_PANEL_Y / 2.0 - 22.0
        };
        pins = pins
            + (centered_cylinder(
                format!("{PART_PREFIX}_gate_lock_pin_ring_{i}"),
                10.0,
                6.0,
                28,
            )
            .translate(x, y, GATE_PANEL_Z / 2.0 + 3.0)
                - centered_cylinder(
                    format!("{PART_PREFIX}_gate_lock_pin_bore_{i}"),
                    4.4,
                    8.0,
                    20,
                )
                .translate(x, y, GATE_PANEL_Z / 2.0 + 3.0));
    }
    pins
}

fn leak_backflow_witness_features() -> Part {
    let panel = centered_cube(
        format!("{PART_PREFIX}_leak_backflow_witness_panel"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut channels = Part::empty(format!("{PART_PREFIX}_backflow_witness_channel_cuts"));
    let mut features = Part::empty(format!("{PART_PREFIX}_backflow_witness_features"));
    for channel in 0..BACKFLOW_CHANNELS {
        let x = centered_index(channel, BACKFLOW_CHANNELS, 38.0);
        channels = channels
            + centered_cube(
                format!("{PART_PREFIX}_backflow_channel_{channel}"),
                16.0,
                WITNESS_Y + 8.0,
                9.0,
            )
            .translate(x, 0.0, WITNESS_Z / 2.0 - 4.5);
        features = features
            + centered_cube(
                format!("{PART_PREFIX}_backflow_arrow_land_{channel}"),
                22.0,
                12.0,
                5.0,
            )
            .translate(x, -WITNESS_Y / 2.0 - 8.0, WITNESS_Z / 2.0 + 2.5);
    }
    panel - channels + features + leak_witness_windows() + witness_pads()
}

fn leak_witness_windows() -> Part {
    let mut windows = Part::empty(format!("{PART_PREFIX}_leak_witness_windows"));
    for i in 0..LEAK_WITNESS_WINDOWS {
        windows = windows
            + centered_cube(
                format!("{PART_PREFIX}_leak_witness_window_{i}"),
                38.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(i, LEAK_WITNESS_WINDOWS, 52.0),
                WITNESS_Y / 2.0 - 24.0,
                WITNESS_Z / 2.0 + 3.0,
            );
    }
    windows
}

fn witness_pads() -> Part {
    let mut pads = Part::empty(format!("{PART_PREFIX}_colorimetric_witness_pads"));
    for i in 0..WITNESS_PAD_COUNT {
        let row = i / 5;
        let col = i % 5;
        pads = pads
            + centered_cube(
                format!("{PART_PREFIX}_wet_path_witness_pad_{i}"),
                34.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(col, 5, 48.0),
                -WITNESS_Y / 2.0 + 24.0 + row as f64 * 28.0,
                WITNESS_Z / 2.0 + 2.0,
            );
    }
    for i in 0..DRY_CONTROL_PADS {
        pads = pads
            + centered_cube(
                format!("{PART_PREFIX}_dry_control_witness_pad_{i}"),
                34.0,
                20.0,
                4.0,
            )
            .translate(
                WITNESS_X / 2.0 - 34.0,
                centered_index(i, DRY_CONTROL_PADS, 30.0),
                WITNESS_Z / 2.0 + 2.0,
            );
    }
    pads
}

fn quarantine_overbag_interface() -> Part {
    let plate = centered_cube(
        format!("{PART_PREFIX}_quarantine_overbag_interface_plate"),
        OVERBAG_X,
        OVERBAG_Y,
        OVERBAG_Z,
    );
    let mouth_cut = centered_cube(
        format!("{PART_PREFIX}_overbag_mouth_clearance_cut"),
        OVERBAG_X - 122.0,
        42.0,
        OVERBAG_Z + 4.0,
    )
    .translate(0.0, 0.0, 4.0);
    plate - mouth_cut + overbag_clamps() + overbag_seam_witness_coupons() + overbag_purge_ports()
}

fn overbag_clamps() -> Part {
    let mut clamps = Part::empty(format!("{PART_PREFIX}_overbag_clamp_lands"));
    for i in 0..OVERBAG_CLAMPS {
        clamps = clamps
            + centered_cube(
                format!("{PART_PREFIX}_overbag_clamp_land_{i}"),
                52.0,
                22.0,
                18.0,
            )
            .translate(
                centered_index(i, OVERBAG_CLAMPS, 60.0),
                OVERBAG_Y / 2.0 + 12.0,
                OVERBAG_Z / 2.0 + 9.0,
            )
            + centered_cube(
                format!("{PART_PREFIX}_overbag_rear_clamp_land_{i}"),
                52.0,
                22.0,
                18.0,
            )
            .translate(
                centered_index(i, OVERBAG_CLAMPS, 60.0),
                -OVERBAG_Y / 2.0 - 12.0,
                OVERBAG_Z / 2.0 + 9.0,
            );
    }
    clamps
}

fn overbag_seam_witness_coupons() -> Part {
    let mut coupons = Part::empty(format!("{PART_PREFIX}_overbag_seam_witness_coupons"));
    for i in 0..OVERBAG_SEAM_WITNESS_COUPONS {
        coupons = coupons
            + centered_cube(
                format!("{PART_PREFIX}_overbag_seam_coupon_{i}"),
                30.0,
                18.0,
                6.0,
            )
            .translate(
                -OVERBAG_X / 2.0 + 34.0,
                centered_index(i, OVERBAG_SEAM_WITNESS_COUPONS, 18.0),
                OVERBAG_Z / 2.0 + 3.0,
            );
    }
    coupons
}

fn overbag_purge_ports() -> Part {
    let mut ports = Part::empty(format!("{PART_PREFIX}_overbag_purge_filter_ports"));
    for i in 0..OVERBAG_PURGE_PORTS {
        let x = OVERBAG_X / 2.0 - 54.0;
        let y = centered_index(i, OVERBAG_PURGE_PORTS, 42.0);
        ports = ports
            + (centered_cylinder(
                format!("{PART_PREFIX}_overbag_purge_port_ring_{i}"),
                14.0,
                8.0,
                28,
            )
            .translate(x, y, OVERBAG_Z / 2.0 + 4.0)
                - centered_cylinder(
                    format!("{PART_PREFIX}_overbag_purge_port_bore_{i}"),
                    6.0,
                    10.0,
                    24,
                )
                .translate(x, y, OVERBAG_Z / 2.0 + 4.0));
    }
    ports
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PART_PREFIX}_camera_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let right_post = centered_cube(
        format!("{PART_PREFIX}_camera_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, -BRIDGE_BEAM_Z / 2.0);
    let beam = centered_cube(
        format!("{PART_PREFIX}_camera_bridge_crossbeam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);

    left_post + right_post + beam + camera_pods() + light_bar_lands()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PART_PREFIX}_camera_evidence_pods"));
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 245.0);
        pods = pods
            + (centered_cube(
                format!("{PART_PREFIX}_camera_pod_mount_{i}"),
                70.0,
                42.0,
                14.0,
            )
            .translate(
                x,
                -BRIDGE_POST_Y / 2.0 - 10.0,
                BRIDGE_UNDERSIDE_Z / 2.0 - 10.0,
            ) - centered_cylinder(
                format!("{PART_PREFIX}_camera_lens_clearance_{i}"),
                10.0,
                16.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                -BRIDGE_POST_Y / 2.0 - 10.0,
                BRIDGE_UNDERSIDE_Z / 2.0 - 10.0,
            ));
    }
    pods
}

fn light_bar_lands() -> Part {
    let mut lights = Part::empty(format!("{PART_PREFIX}_camera_bridge_light_bar_lands"));
    for i in 0..LIGHT_BAR_COUNT {
        lights = lights
            + centered_cube(
                format!("{PART_PREFIX}_evidence_light_bar_land_{i}"),
                92.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(i, LIGHT_BAR_COUNT, 150.0),
                BRIDGE_POST_Y / 2.0 + 7.0,
                BRIDGE_UNDERSIDE_Z / 2.0 - 12.0,
            );
    }
    lights
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PART_PREFIX}_front_robot_sweep_keepout"),
        STATION_X - 180.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, 0.0);
    let rear = centered_cube(
        format!("{PART_PREFIX}_rear_service_drawer_keepout"),
        STATION_X - 180.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE, 0.0);
    let left = centered_cube(
        format!("{PART_PREFIX}_left_split_receiver_service_keepout"),
        18.0,
        STATION_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 + LEFT_SPLIT_SERVICE_CLEARANCE, 0.0, 0.0);
    let right = centered_cube(
        format!("{PART_PREFIX}_right_overbag_service_keepout"),
        18.0,
        STATION_Y - 180.0,
        KEEP_OUT_Z,
    )
    .translate(STATION_X / 2.0 - RIGHT_OVERBAG_SERVICE_CLEARANCE, 0.0, 0.0);
    let overhead = centered_cube(
        format!("{PART_PREFIX}_overhead_camera_service_keepout_gauge"),
        BRIDGE_SPAN_X - 120.0,
        18.0,
        16.0,
    )
    .translate(0.0, BRIDGE_POS.1, OVERHEAD_CAMERA_SERVICE_Z);
    front + rear + left + right + overhead
}

fn rectangular_frame_xy(
    name: String,
    outer_x: f64,
    outer_y: f64,
    height: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
        - centered_cube(format!("{name}_inner_cut"), inner_x, inner_y, height + 2.0)
}

fn module_footprints() -> [Footprint; 8] {
    [
        footprint("waste_retain_split_receiver", SPLIT_POS, SPLIT_X, SPLIT_Y),
        (footprint(
            "sealed_retain_vial_nests",
            VIAL_POS,
            VIAL_NEST_X,
            VIAL_NEST_Y,
        )),
        (footprint(
            "temperature_logger_pockets",
            LOGGER_POS,
            LOGGER_PANEL_X,
            LOGGER_PANEL_Y,
        )),
        footprint(
            "barcode_rfid_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        footprint(
            "chain_of_custody_token_rail",
            TOKEN_POS,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
        footprint(
            "hold_release_reject_gates",
            GATE_POS,
            GATE_PANEL_X,
            GATE_PANEL_Y,
        ),
        footprint(
            "leak_backflow_witness_features",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        footprint(
            "quarantine_overbag_interface",
            OVERBAG_POS,
            OVERBAG_X,
            OVERBAG_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn fits_inside_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - RIM_W - 16.0;
    let usable_y = STATION_Y / 2.0 - RIM_W - 16.0;
    footprint.center.0 - footprint.x / 2.0 >= -usable_x
        && footprint.center.0 + footprint.x / 2.0 <= usable_x
        && footprint.center.1 - footprint.y / 2.0 >= -usable_y
        && footprint.center.1 + footprint.y / 2.0 <= usable_y
}

fn overlaps(first: Footprint, second: Footprint, clearance: f64) -> bool {
    let dx = (first.center.0 - second.center.0).abs();
    let dy = (first.center.1 - second.center.1).abs();
    dx < (first.x + second.x) / 2.0 + clearance && dy < (first.y + second.y) / 2.0 + clearance
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, GATE_LANES, GATE_LANE_PITCH)
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(STATUS_NAMES, ["hold", "release", "reject"]);
    assert_eq!(SPLIT_OUTPUT_PORTS, RETAIN_VIALS);
    assert_eq!(VIAL_SEAL_RINGS, RETAIN_VIALS);
    assert_eq!(
        LOGGER_CONTACT_PINS,
        LOGGER_POCKETS * LOGGER_CONTACTS_PER_POCKET
    );
    assert_eq!(
        CUSTODY_TOKEN_SLOTS,
        CUSTODY_STEP_NAMES.len() * TOKEN_SLOTS_PER_STEP
    );
    assert_eq!(TOKEN_SLOTS_PER_STEP, STATUS_NAMES.len());
    assert_eq!(GATE_LANES, STATUS_NAMES.len());
    assert!(GATE_MIN_GAP >= 50.0);
    assert!(BRIDGE_UNDERSIDE_Z > SPLIT_Z + VIAL_NEST_Z + 60.0);
    assert!(OVERHEAD_CAMERA_SERVICE_Z > BRIDGE_UNDERSIDE_Z);

    let unique_outputs: std::collections::BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.ends_with(".stl"), "{path}");
    }

    let footprints = module_footprints();
    for footprint in footprints {
        assert!(
            fits_inside_station(footprint),
            "{} exceeds station footprint",
            footprint.name
        );
    }

    for (index, first) in footprints.iter().enumerate() {
        for second in footprints.iter().skip(index + 1) {
            assert!(
                !overlaps(*first, *second, 8.0),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        assert_layout();
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS[0].contains("secondary_containment_deck"));
        assert!(OUTPUTS[1].contains("waste_retain_split_receiver"));
        assert!(OUTPUTS[2].contains("sealed_retain_vial_nests"));
        assert!(OUTPUTS[3].contains("temperature_logger_pockets"));
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_match_requested_validation_scope() {
        for feature in [
            "waste_retain_split_receiver",
            "sealed_retain_vial_nests",
            "temperature_logger_pockets",
            "barcode_rfid_custody_lands",
            "chain_of_custody_token_rail",
            "hold_release_reject_gates",
            "leak_backflow_witness_features",
            "quarantine_overbag_interface",
            "camera_evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn primary_modules_fit_without_overlap() {
        let footprints = module_footprints();
        for footprint in footprints {
            assert!(fits_inside_station(footprint), "{}", footprint.name);
        }
        for (index, first) in footprints.iter().enumerate() {
            for second in footprints.iter().skip(index + 1) {
                assert!(
                    !overlaps(*first, *second, 8.0),
                    "{} overlaps {}",
                    first.name,
                    second.name
                );
            }
        }
    }

    #[test]
    fn retain_split_capacity_balances_to_audit_vials() {
        assert_eq!(SPLIT_LANES, 8);
        assert_eq!(SPLIT_BRANCHES_PER_LANE, 3);
        assert_eq!(SPLIT_OUTPUT_PORTS, 24);
        assert_eq!(RETAIN_VIAL_ROWS * RETAIN_VIAL_COLS, RETAIN_VIALS);
        assert_eq!(SPLIT_OUTPUT_PORTS, RETAIN_VIALS);
        assert_eq!(VIAL_SEAL_RINGS, RETAIN_VIALS);
        assert!(VIAL_NEST_LATCHES >= RETAIN_VIAL_ROWS * 2);
    }

    #[test]
    fn custody_temperature_and_identity_counts_are_locked() {
        assert_eq!(LOGGER_POCKETS, 4);
        assert_eq!(LOGGER_SEAL_WELLS, 8);
        assert_eq!(LOGGER_CONTACT_PINS, 12);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 8);
        assert_eq!(RUN_CARD_LANDS, 4);
        assert_eq!(TAMPER_SEAL_WELLS, 8);
        assert_eq!(CUSTODY_STEP_NAMES.len(), 6);
        assert_eq!(CUSTODY_TOKEN_SLOTS, 18);
    }

    #[test]
    fn disposition_gates_are_distinct_and_balanced() {
        assert_eq!(STATUS_NAMES, ["hold", "release", "reject"]);
        assert_eq!(GATE_LANES, STATUS_NAMES.len());
        assert_eq!(GATE_SLOTS_PER_LANE, 4);
        assert_eq!(GATE_LOCK_PINS, GATE_LANES * 2);
        assert!(GATE_MIN_GAP >= 50.0);
        assert!(status_lane_x(0) < status_lane_x(1));
        assert!(status_lane_x(1) < status_lane_x(2));
    }

    #[test]
    fn leak_overbag_camera_and_keepout_counts_are_explicit() {
        assert_eq!(BACKFLOW_CHANNELS, SPLIT_LANES);
        assert_eq!(LEAK_WITNESS_WINDOWS, 6);
        assert_eq!(WITNESS_PAD_COUNT, 10);
        assert_eq!(DRY_CONTROL_PADS, 2);
        assert_eq!(OVERBAG_CLAMPS, 6);
        assert_eq!(OVERBAG_SEAM_WITNESS_COUPONS, 8);
        assert_eq!(OVERBAG_PURGE_PORTS, 2);
        assert_eq!(CAMERA_COUNT, 5);
        assert_eq!(LIGHT_BAR_COUNT, 6);
        assert_eq!(KEEP_OUT_ZONES, 5);
    }

    #[test]
    fn centered_index_spacing_is_symmetric_for_repeated_features() {
        let first = centered_index(0, SPLIT_LANES, SPLIT_LANE_PITCH);
        let last = centered_index(SPLIT_LANES - 1, SPLIT_LANES, SPLIT_LANE_PITCH);
        assert!((first + last).abs() < 1e-9);
        assert_eq!(
            centered_index(3, SPLIT_LANES, SPLIT_LANE_PITCH)
                - centered_index(2, SPLIT_LANES, SPLIT_LANE_PITCH),
            SPLIT_LANE_PITCH
        );
    }

    #[test]
    fn limitation_markers_prevent_protocol_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"not_a_waste_release_rule"));
        assert!(LIMITATIONS.contains(&"no_thermal_acceptance_claim"));
        assert!(LIMITATIONS.contains(&"not_a_sterile_barrier_design"));
        assert!(LIMITATIONS.contains(&"purchased_wetted_components_external"));
    }
}
