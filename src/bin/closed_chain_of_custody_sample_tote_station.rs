use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sample/material tote chain-of-custody station.
//
// Design intent:
// - Receive sealed totes from incubator and isolator workflows without opening
//   the sample/material path at the bench.
// - Make identity capture, environmental evidence, tamper seals, release/hold/
//   reject disposition, QC sampling, and archive handoff physically explicit.
// - Reserve robot and service keepouts so the tote, scanner, bridge camera, and
//   pass-through handoff tongue cannot quietly fight for the same envelope.
//
// This is product-concept interface CAD. It is not a custody SOP, sterility
// guarantee, release decision, or regulatory compliance claim.

const OUTPUTS: [&str; 13] = [
    "output/closed_chain_of_custody_sample_tote_station_leak_drip_tray_base.stl",
    "output/closed_chain_of_custody_sample_tote_station_sealed_tote_receiver.stl",
    "output/closed_chain_of_custody_sample_tote_station_workflow_handoff_ports.stl",
    "output/closed_chain_of_custody_sample_tote_station_rfid_barcode_scan_lands.stl",
    "output/closed_chain_of_custody_sample_tote_station_environmental_logger_pocket.stl",
    "output/closed_chain_of_custody_sample_tote_station_tamper_evident_seal_pockets.stl",
    "output/closed_chain_of_custody_sample_tote_station_released_hold_reject_lanes.stl",
    "output/closed_chain_of_custody_sample_tote_station_custody_card_vial_holders.stl",
    "output/closed_chain_of_custody_sample_tote_station_pass_through_handoff_tongue.stl",
    "output/closed_chain_of_custody_sample_tote_station_camera_evidence_bridge.stl",
    "output/closed_chain_of_custody_sample_tote_station_robot_service_keepouts.stl",
    "output/closed_chain_of_custody_sample_tote_station_tote_latch_and_datum_sensors.stl",
    "output/closed_chain_of_custody_sample_tote_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_tote_receiver",
    "rfid_barcode_scan_lands",
    "environmental_logger_pocket",
    "tamper_evident_seal_pockets",
    "released_hold_reject_lanes",
    "chain_of_custody_card_vial_holders",
    "leak_drip_tray",
    "pass_through_handoff_tongue",
    "camera_evidence_bridge",
    "robot_service_keepouts",
    "incubator_isolator_qc_archive_workflow_ports",
];

const WORKFLOW_NAMES: [&str; 4] = ["incubator", "isolator", "qc", "archive"];
const STATUS_LANES: [&str; 3] = ["released", "hold", "reject"];

const STATION_X: f64 = 1360.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const LEAK_RECESS_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.6;

const RECEIVER_CENTER: (f64, f64) = (-390.0, 145.0);
const RECEIVER_X: f64 = 390.0;
const RECEIVER_Y: f64 = 290.0;
const RECEIVER_Z: f64 = 62.0;
const RECEIVER_LOCATOR_COUNT: usize = 4;
const TOTE_PRESENT_SENSORS: usize = 4;
const LATCH_SENSORS: usize = 4;
const DATUM_DISC_COUNT: usize = 4;

const WORKFLOW_PORT_CENTER: (f64, f64) = (0.0, 410.0);
const WORKFLOW_PANEL_X: f64 = 980.0;
const WORKFLOW_PANEL_Y: f64 = 84.0;
const WORKFLOW_PANEL_Z: f64 = 44.0;
const WORKFLOW_PORT_PITCH: f64 = 230.0;

const SCAN_CENTER: (f64, f64) = (120.0, 250.0);
const SCAN_PANEL_X: f64 = 400.0;
const SCAN_PANEL_Y: f64 = 128.0;
const SCAN_PANEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 6;
const STATUS_TOKEN_LANDS: usize = 6;

const LOGGER_CENTER: (f64, f64) = (500.0, 255.0);
const LOGGER_X: f64 = 220.0;
const LOGGER_Y: f64 = 128.0;
const LOGGER_Z: f64 = 50.0;
const LOGGER_POCKETS: usize = 2;
const LOGGER_CABLE_PORTS: usize = 2;

const SEAL_CENTER: (f64, f64) = (505.0, 105.0);
const SEAL_X: f64 = 220.0;
const SEAL_Y: f64 = 142.0;
const SEAL_Z: f64 = 40.0;
const SEAL_POCKET_COUNT: usize = 8;
const SEAL_POCKET_PITCH_X: f64 = 46.0;
const SEAL_POCKET_PITCH_Y: f64 = 44.0;

const LANES_CENTER: (f64, f64) = (260.0, -120.0);
const LANES_X: f64 = 520.0;
const LANES_Y: f64 = 230.0;
const LANES_Z: f64 = 48.0;
const LANE_SLOTS_PER_STATUS: usize = 4;
const STATUS_SLOT_X: f64 = 86.0;
const STATUS_SLOT_Y: f64 = 44.0;
const STATUS_LANE_PITCH_X: f64 = 148.0;
#[cfg(test)]
const STATUS_LANE_GAP_MIN: f64 = 34.0;

const CUSTODY_CENTER: (f64, f64) = (-380.0, -170.0);
const CUSTODY_X: f64 = 410.0;
const CUSTODY_Y: f64 = 210.0;
const CUSTODY_Z: f64 = 54.0;
const CUSTODY_CARD_SLOTS: usize = 6;
const VIAL_HOLDERS: usize = 12;
const CARD_SLOT_PITCH: f64 = 52.0;
const VIAL_COLS: usize = 6;
const VIAL_PITCH_X: f64 = 42.0;
const VIAL_PITCH_Y: f64 = 40.0;

const TONGUE_CENTER: (f64, f64) = (0.0, -380.0);
const TONGUE_X: f64 = 820.0;
const TONGUE_Y: f64 = 145.0;
const TONGUE_Z: f64 = 32.0;
const TONGUE_RAIL_PITCH_X: f64 = 640.0;
const TONGUE_RAIL_W: f64 = 24.0;
const TONGUE_RAIL_Z: f64 = 30.0;
const TONGUE_DATUM_PINS: usize = 4;

const CAMERA_BRIDGE_CENTER: (f64, f64) = (0.0, 25.0);
const CAMERA_BRIDGE_SPAN_X: f64 = 1160.0;
const CAMERA_BRIDGE_POST_X: f64 = 30.0;
const CAMERA_BRIDGE_POST_Y: f64 = 40.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 230.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_BRIDGE_TOTAL_Z: f64 = CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z;
const CAMERA_COUNT: usize = 3;
const LED_STRIP_COUNT: usize = 8;

const KEEP_OUT_Z: f64 = 90.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 360.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 210.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 220.0;
const OVERHEAD_CAMERA_KEEP_OUT_Z: f64 = 290.0;
const SERVICE_KEEP_OUTS: usize = 4;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_drip_tray_base();
    export(OUTPUTS[0], &base);

    let receiver = sealed_tote_receiver();
    export(OUTPUTS[1], &receiver);

    let workflow_ports = workflow_handoff_ports();
    export(OUTPUTS[2], &workflow_ports);

    let scan_lands = rfid_barcode_scan_lands();
    export(OUTPUTS[3], &scan_lands);

    let logger = environmental_logger_pocket();
    export(OUTPUTS[4], &logger);

    let seals = tamper_evident_seal_pockets();
    export(OUTPUTS[5], &seals);

    let lanes = released_hold_reject_lanes();
    export(OUTPUTS[6], &lanes);

    let custody = custody_card_vial_holders();
    export(OUTPUTS[7], &custody);

    let tongue = pass_through_handoff_tongue();
    export(OUTPUTS[8], &tongue);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let sensors = tote_latch_and_datum_sensors();
    export(OUTPUTS[11], &sensors);

    let assembly =
        base + receiver.translate(
            RECEIVER_CENTER.0,
            RECEIVER_CENTER.1,
            deck_insert_z(RECEIVER_Z),
        ) + workflow_ports.translate(
            WORKFLOW_PORT_CENTER.0,
            WORKFLOW_PORT_CENTER.1,
            deck_insert_z(WORKFLOW_PANEL_Z),
        ) + scan_lands.translate(SCAN_CENTER.0, SCAN_CENTER.1, deck_insert_z(SCAN_PANEL_Z))
            + logger.translate(LOGGER_CENTER.0, LOGGER_CENTER.1, deck_insert_z(LOGGER_Z))
            + seals.translate(SEAL_CENTER.0, SEAL_CENTER.1, deck_insert_z(SEAL_Z))
            + lanes.translate(LANES_CENTER.0, LANES_CENTER.1, deck_insert_z(LANES_Z))
            + custody.translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, deck_insert_z(CUSTODY_Z))
            + tongue.translate(TONGUE_CENTER.0, TONGUE_CENTER.1, deck_insert_z(TONGUE_Z))
            + bridge.translate(
                CAMERA_BRIDGE_CENTER.0,
                CAMERA_BRIDGE_CENTER.1,
                deck_insert_z(CAMERA_BRIDGE_TOTAL_Z),
            )
            + keepouts.translate(0.0, 0.0, deck_insert_z(KEEP_OUT_Z))
            + sensors.translate(
                RECEIVER_CENTER.0,
                RECEIVER_CENTER.1,
                deck_insert_z(RECEIVER_Z + 18.0),
            );
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed chain-of-custody sample/material tote station:");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm leak/drip tray");
    println!(
        "  Workflow ports:              {} closed handoff ports for incubator, isolator, QC, and archive routes",
        WORKFLOW_NAMES.len()
    );
    println!(
        "  Tote receiver:               {RECEIVER_X:.0}mm x {RECEIVER_Y:.0}mm sealed receiver, {RECEIVER_LOCATOR_COUNT} locator sockets, {TOTE_PRESENT_SENSORS} tote-present sensors, {LATCH_SENSORS} latch sensors"
    );
    println!(
        "  Traceability:                {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {STATUS_TOKEN_LANDS} status token lands, {CUSTODY_CARD_SLOTS} custody card slots, {VIAL_HOLDERS} vial holders"
    );
    println!(
        "  Evidence/environment:        {LOGGER_POCKETS} logger pockets, {SEAL_POCKET_COUNT} tamper-evident seal pockets, {CAMERA_COUNT} camera pods, {LED_STRIP_COUNT} bridge light segments"
    );
    println!(
        "  Disposition lanes:           released/hold/reject with {LANE_SLOTS_PER_STATUS} tote positions per lane"
    );
    println!(
        "  Robot/service clearances:    front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, side service {SIDE_SERVICE_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_CAMERA_KEEP_OUT_Z:.0}mm"
    );
    println!(
        "  Datum/keepout controls:      {DATUM_DISC_COUNT} receiver datum discs, {TONGUE_DATUM_PINS} tongue datum pins, {SERVICE_KEEP_OUTS} named keepout zones"
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
    for (name, pos, width, depth) in component_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn component_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        (
            "sealed_tote_receiver",
            RECEIVER_CENTER,
            RECEIVER_X,
            RECEIVER_Y,
        ),
        (
            "workflow_handoff_ports",
            WORKFLOW_PORT_CENTER,
            WORKFLOW_PANEL_X,
            WORKFLOW_PANEL_Y,
        ),
        (
            "rfid_barcode_scan_lands",
            SCAN_CENTER,
            SCAN_PANEL_X,
            SCAN_PANEL_Y,
        ),
        (
            "environmental_logger_pocket",
            LOGGER_CENTER,
            LOGGER_X,
            LOGGER_Y,
        ),
        ("tamper_evident_seal_pockets", SEAL_CENTER, SEAL_X, SEAL_Y),
        ("released_hold_reject_lanes", LANES_CENTER, LANES_X, LANES_Y),
        (
            "custody_card_vial_holders",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        (
            "pass_through_handoff_tongue",
            TONGUE_CENTER,
            TONGUE_X,
            TONGUE_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn leak_drip_tray_base() -> Part {
    let deck = centered_cube(
        "closed_chain_of_custody_sample_tote_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let shallow_basin = centered_cube(
        "closed_chain_of_custody_sample_tote_station_leak_basin_recess",
        STATION_X - 112.0,
        STATION_Y - 118.0,
        LEAK_RECESS_DEPTH,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - LEAK_RECESS_DEPTH / 2.0 + 0.2);
    let front_drip_channel = centered_cube(
        "closed_chain_of_custody_sample_tote_station_front_drip_channel",
        STATION_X - 190.0,
        30.0,
        LEAK_RECESS_DEPTH + 4.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 66.0, BASE_Z / 2.0 - 4.0);
    let sump = centered_cube(
        "closed_chain_of_custody_sample_tote_station_sump_recess",
        120.0,
        78.0,
        LEAK_RECESS_DEPTH + 6.0,
    )
    .translate(
        STATION_X / 2.0 - 116.0,
        -STATION_Y / 2.0 + 88.0,
        BASE_Z / 2.0 - 4.0,
    );
    let drain = centered_cylinder(
        "closed_chain_of_custody_sample_tote_station_drain_port",
        DRAIN_PORT_D / 2.0,
        56.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 116.0,
        -STATION_Y / 2.0 + 32.0,
        BASE_Z / 2.0 - 4.0,
    );

    deck - shallow_basin - front_drip_channel - sump - drain - insert_sockets() - mount_holes()
        + perimeter_rims()
        + drip_sump_flag()
        + workflow_route_rails()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_chain_of_custody_sample_tote_station_component_sockets");
    for (name, pos, width, depth) in component_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                5.0,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - 2.4);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("closed_chain_of_custody_sample_tote_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 48.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 48.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, -STATION_Y / 2.0 + 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let front = centered_cube(
        "closed_chain_of_custody_sample_tote_station_front_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(
        "closed_chain_of_custody_sample_tote_station_rear_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        "closed_chain_of_custody_sample_tote_station_left_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "closed_chain_of_custody_sample_tote_station_right_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn drip_sump_flag() -> Part {
    centered_cube(
        "closed_chain_of_custody_sample_tote_station_drip_sump_visual_flag",
        92.0,
        18.0,
        18.0,
    )
    .translate(
        STATION_X / 2.0 - 116.0,
        -STATION_Y / 2.0 + 145.0,
        BASE_Z / 2.0 + 9.0,
    )
}

fn workflow_route_rails() -> Part {
    let incubator_to_receiver = centered_cube(
        "closed_chain_of_custody_sample_tote_station_incubator_to_receiver_route_rail",
        460.0,
        10.0,
        18.0,
    )
    .translate(-235.0, 322.0, BASE_Z / 2.0 + 9.0);
    let receiver_to_qc = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_to_qc_route_rail",
        660.0,
        10.0,
        18.0,
    )
    .translate(-10.0, 62.0, BASE_Z / 2.0 + 9.0);
    let qc_to_archive = centered_cube(
        "closed_chain_of_custody_sample_tote_station_qc_to_archive_route_rail",
        480.0,
        10.0,
        18.0,
    )
    .translate(255.0, -275.0, BASE_Z / 2.0 + 9.0);
    incubator_to_receiver + receiver_to_qc + qc_to_archive
}

fn sealed_tote_receiver() -> Part {
    let body = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_body",
        RECEIVER_X,
        RECEIVER_Y,
        RECEIVER_Z,
    );
    let tote_clearance = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_tote_clearance",
        RECEIVER_X - 72.0,
        RECEIVER_Y - 62.0,
        RECEIVER_Z + 2.0,
    )
    .translate(0.0, 0.0, 7.0);
    let front_loading_throat = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_front_loading_throat",
        RECEIVER_X - 132.0,
        66.0,
        RECEIVER_Z + 4.0,
    )
    .translate(0.0, -RECEIVER_Y / 2.0 + 28.0, 7.0);
    let gasket_land = rectangular_frame_xy(
        "closed_chain_of_custody_sample_tote_station_receiver_gasket_land",
        RECEIVER_X - 36.0,
        RECEIVER_Y - 30.0,
        8.0,
        RECEIVER_X - 92.0,
        RECEIVER_Y - 84.0,
    )
    .translate(0.0, 0.0, RECEIVER_Z / 2.0 + 4.0);
    let rear_stop = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_rear_datum_stop",
        RECEIVER_X - 44.0,
        18.0,
        38.0,
    )
    .translate(0.0, RECEIVER_Y / 2.0 + 9.0, 0.0);
    let left_rail = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_left_x_datum",
        18.0,
        RECEIVER_Y - 28.0,
        36.0,
    )
    .translate(-RECEIVER_X / 2.0 - 9.0, 0.0, -2.0);
    let right_compliance_rail = centered_cube(
        "closed_chain_of_custody_sample_tote_station_receiver_right_soft_datum",
        18.0,
        RECEIVER_Y - 28.0,
        28.0,
    )
    .translate(RECEIVER_X / 2.0 + 9.0, 0.0, -6.0);

    body - tote_clearance - front_loading_throat
        + gasket_land
        + rear_stop
        + left_rail
        + right_compliance_rail
        + receiver_locator_sockets()
        + receiver_latch_pockets()
}

fn receiver_locator_sockets() -> Part {
    let mut locators =
        Part::empty("closed_chain_of_custody_sample_tote_station_receiver_locator_sockets");
    for (i, (x, y)) in receiver_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_chain_of_custody_sample_tote_station_receiver_locator_boss_{i}"),
            16.0,
            12.0,
            36,
        )
        .translate(*x, *y, RECEIVER_Z / 2.0 + 6.0);
        let socket = centered_cylinder(
            format!("closed_chain_of_custody_sample_tote_station_receiver_locator_socket_{i}"),
            7.0,
            14.0,
            28,
        )
        .translate(*x, *y, RECEIVER_Z / 2.0 + 6.0);
        locators = locators + (boss - socket);
    }
    locators
}

fn receiver_latch_pockets() -> Part {
    let mut pockets =
        Part::empty("closed_chain_of_custody_sample_tote_station_receiver_latch_pockets");
    for (i, (x, y)) in [
        (-RECEIVER_X / 2.0 + 54.0, -RECEIVER_Y / 2.0 + 38.0),
        (RECEIVER_X / 2.0 - 54.0, -RECEIVER_Y / 2.0 + 38.0),
        (-RECEIVER_X / 2.0 + 54.0, RECEIVER_Y / 2.0 - 38.0),
        (RECEIVER_X / 2.0 - 54.0, RECEIVER_Y / 2.0 - 38.0),
    ]
    .iter()
    .enumerate()
    {
        let pocket = centered_cube(
            format!("closed_chain_of_custody_sample_tote_station_receiver_latch_pocket_{i}"),
            58.0,
            26.0,
            24.0,
        )
        .translate(*x, *y, RECEIVER_Z / 2.0 + 12.0);
        let relief = centered_cube(
            format!("closed_chain_of_custody_sample_tote_station_receiver_latch_hook_relief_{i}"),
            32.0,
            12.0,
            26.0,
        )
        .translate(*x, *y, RECEIVER_Z / 2.0 + 14.0);
        pockets = pockets + (pocket - relief);
    }
    pockets
}

fn workflow_handoff_ports() -> Part {
    let panel = centered_cube(
        "closed_chain_of_custody_sample_tote_station_workflow_port_backplane",
        WORKFLOW_PANEL_X,
        WORKFLOW_PANEL_Y,
        WORKFLOW_PANEL_Z,
    );
    let mut door_windows =
        Part::empty("closed_chain_of_custody_sample_tote_station_workflow_door_windows");
    let mut port_faces =
        Part::empty("closed_chain_of_custody_sample_tote_station_workflow_port_faces");

    for (i, workflow) in WORKFLOW_NAMES.iter().enumerate() {
        let x = centered_index(i, WORKFLOW_NAMES.len(), WORKFLOW_PORT_PITCH);
        door_windows = door_windows
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_{workflow}_handoff_window"),
                126.0,
                WORKFLOW_PANEL_Y + 4.0,
                22.0,
            )
            .translate(x, 0.0, 4.0);
        port_faces = port_faces
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_{workflow}_port_shutter"),
                158.0,
                14.0,
                38.0,
            )
            .translate(x, -WORKFLOW_PANEL_Y / 2.0 - 7.0, 0.0)
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_{workflow}_route_key"),
                62.0,
                18.0,
                12.0,
            )
            .translate(
                x,
                WORKFLOW_PANEL_Y / 2.0 + 9.0,
                -WORKFLOW_PANEL_Z / 2.0 + 6.0,
            );
    }

    panel - door_windows + port_faces
}

fn rfid_barcode_scan_lands() -> Part {
    let panel = centered_cube(
        "closed_chain_of_custody_sample_tote_station_scan_land_panel",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    );
    let mut barcode_recesses =
        Part::empty("closed_chain_of_custody_sample_tote_station_barcode_recesses");
    for i in 0..BARCODE_LANDS {
        let row = i / 5;
        let col = i % 5;
        barcode_recesses = barcode_recesses
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_barcode_land_{i}"),
                58.0,
                18.0,
                SCAN_PANEL_Z + 2.0,
            )
            .translate(
                centered_index(col, 5, 70.0),
                centered_index(row, 2, 42.0),
                3.0,
            );
    }

    let mut rfid_pads = Part::empty("closed_chain_of_custody_sample_tote_station_rfid_pads");
    for i in 0..RFID_LANDS {
        let row = i / 3;
        let col = i % 3;
        rfid_pads = rfid_pads
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_rfid_chip_land_{i}"),
                28.0,
                28.0,
                5.0,
            )
            .translate(
                centered_index(col, 3, 58.0),
                centered_index(row, 2, 70.0),
                SCAN_PANEL_Z / 2.0 + 2.5,
            );
    }

    let mut status_tokens =
        Part::empty("closed_chain_of_custody_sample_tote_station_status_token_lands");
    for i in 0..STATUS_TOKEN_LANDS {
        status_tokens = status_tokens
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_status_token_land_{i}"),
                9.0,
                4.0,
                28,
            )
            .translate(
                -SCAN_PANEL_X / 2.0 + 34.0,
                centered_index(i, STATUS_TOKEN_LANDS, 18.0),
                SCAN_PANEL_Z / 2.0 + 2.0,
            );
    }

    panel - barcode_recesses + rfid_pads + status_tokens
}

fn environmental_logger_pocket() -> Part {
    let block = centered_cube(
        "closed_chain_of_custody_sample_tote_station_environmental_logger_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut pockets =
        Part::empty("closed_chain_of_custody_sample_tote_station_environmental_logger_pockets");
    for i in 0..LOGGER_POCKETS {
        pockets = pockets
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_logger_pocket_{i}"),
                78.0,
                58.0,
                LOGGER_Z + 4.0,
            )
            .translate(centered_index(i, LOGGER_POCKETS, 92.0), 0.0, 8.0);
    }

    let mut cable_ports =
        Part::empty("closed_chain_of_custody_sample_tote_station_logger_cable_ports");
    for i in 0..LOGGER_CABLE_PORTS {
        cable_ports = cable_ports
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_logger_cable_port_{i}"),
                4.0,
                LOGGER_Y + 6.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, LOGGER_CABLE_PORTS, 92.0), 0.0, -2.0);
    }

    let pull_tabs = centered_cube(
        "closed_chain_of_custody_sample_tote_station_logger_pull_tab_land",
        LOGGER_X - 34.0,
        12.0,
        12.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 12.0, LOGGER_Z / 2.0 + 6.0);

    block - pockets - cable_ports + pull_tabs
}

fn tamper_evident_seal_pockets() -> Part {
    let block = centered_cube(
        "closed_chain_of_custody_sample_tote_station_tamper_seal_block",
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let mut seal_pockets =
        Part::empty("closed_chain_of_custody_sample_tote_station_tamper_seal_pockets");
    for i in 0..SEAL_POCKET_COUNT {
        let row = i / 4;
        let col = i % 4;
        seal_pockets = seal_pockets
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_tamper_seal_pocket_{i}"),
                30.0,
                30.0,
                SEAL_Z + 4.0,
            )
            .translate(
                centered_index(col, 4, SEAL_POCKET_PITCH_X),
                centered_index(row, 2, SEAL_POCKET_PITCH_Y),
                8.0,
            );
    }
    let serialized_tab = centered_cube(
        "closed_chain_of_custody_sample_tote_station_tamper_seal_serial_card_land",
        SEAL_X - 34.0,
        18.0,
        8.0,
    )
    .translate(0.0, SEAL_Y / 2.0 - 19.0, SEAL_Z / 2.0 + 4.0);

    block - seal_pockets + serialized_tab
}

fn released_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_chain_of_custody_sample_tote_station_release_hold_reject_lane_panel",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    let mut slots = Part::empty("closed_chain_of_custody_sample_tote_station_status_lane_slots");
    let mut flags = Part::empty("closed_chain_of_custody_sample_tote_station_status_lane_flags");

    for (lane, status) in STATUS_LANES.iter().enumerate() {
        let x = status_lane_x(lane);
        let lane_backstop = centered_cube(
            format!("closed_chain_of_custody_sample_tote_station_{status}_lane_backstop"),
            118.0,
            12.0,
            30.0,
        )
        .translate(x, LANES_Y / 2.0 - 28.0, LANES_Z / 2.0 + 15.0);
        let status_flag = centered_cube(
            format!("closed_chain_of_custody_sample_tote_station_{status}_status_flag"),
            84.0,
            12.0,
            16.0,
        )
        .translate(x, -LANES_Y / 2.0 + 18.0, LANES_Z / 2.0 + 8.0);
        flags = flags + lane_backstop + status_flag;

        for slot in 0..LANE_SLOTS_PER_STATUS {
            slots = slots
                + centered_cube(
                    format!(
                        "closed_chain_of_custody_sample_tote_station_{status}_tote_slot_{slot}"
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    LANES_Z + 4.0,
                )
                .translate(
                    x,
                    centered_index(slot, LANE_SLOTS_PER_STATUS, 48.0),
                    8.0,
                );
        }
    }

    panel - slots + lane_dividers() + flags
}

fn lane_dividers() -> Part {
    let mut dividers =
        Part::empty("closed_chain_of_custody_sample_tote_station_status_lane_dividers");
    for (i, x) in [-STATUS_LANE_PITCH_X / 2.0, STATUS_LANE_PITCH_X / 2.0]
        .iter()
        .enumerate()
    {
        dividers = dividers
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_status_lane_divider_{i}"),
                12.0,
                LANES_Y - 32.0,
                LANES_Z + 22.0,
            )
            .translate(*x, 0.0, 11.0);
    }
    dividers
}

fn custody_card_vial_holders() -> Part {
    let block = centered_cube(
        "closed_chain_of_custody_sample_tote_station_custody_holder_block",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    let mut card_slots =
        Part::empty("closed_chain_of_custody_sample_tote_station_custody_card_slots");
    for i in 0..CUSTODY_CARD_SLOTS {
        card_slots = card_slots
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_custody_card_slot_{i}"),
                4.0,
                82.0,
                CUSTODY_Z + 4.0,
            )
            .rotate(0.0, 0.0, -8.0)
            .translate(
                centered_index(i, CUSTODY_CARD_SLOTS, CARD_SLOT_PITCH),
                50.0,
                8.0,
            );
    }

    let mut vial_wells =
        Part::empty("closed_chain_of_custody_sample_tote_station_vial_holder_wells");
    for i in 0..VIAL_HOLDERS {
        let row = i / VIAL_COLS;
        let col = i % VIAL_COLS;
        vial_wells = vial_wells
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_custody_vial_well_{i}"),
                8.2,
                CUSTODY_Z + 4.0,
                28,
            )
            .translate(
                centered_index(col, VIAL_COLS, VIAL_PITCH_X),
                -50.0 + centered_index(row, 2, VIAL_PITCH_Y),
                8.0,
            );
    }

    let front_card_fence = centered_cube(
        "closed_chain_of_custody_sample_tote_station_custody_card_front_fence",
        CUSTODY_X - 42.0,
        12.0,
        26.0,
    )
    .translate(0.0, 104.0 - CUSTODY_Y / 2.0, CUSTODY_Z / 2.0 + 13.0);
    let vial_label_rail = centered_cube(
        "closed_chain_of_custody_sample_tote_station_vial_label_scan_rail",
        CUSTODY_X - 56.0,
        16.0,
        10.0,
    )
    .translate(0.0, -CUSTODY_Y / 2.0 + 24.0, CUSTODY_Z / 2.0 + 5.0);

    block - card_slots - vial_wells + front_card_fence + vial_label_rail
}

fn pass_through_handoff_tongue() -> Part {
    let base = centered_cube(
        "closed_chain_of_custody_sample_tote_station_pass_through_tongue_base",
        TONGUE_X,
        TONGUE_Y,
        TONGUE_Z,
    );
    let center_washdown_trough = centered_cube(
        "closed_chain_of_custody_sample_tote_station_pass_through_tongue_washdown_trough",
        TONGUE_X - 160.0,
        24.0,
        TONGUE_Z + 4.0,
    )
    .translate(0.0, 0.0, 8.0);
    let mut rails = Part::empty("closed_chain_of_custody_sample_tote_station_tongue_rails");
    for (i, x) in [-TONGUE_RAIL_PITCH_X / 2.0, TONGUE_RAIL_PITCH_X / 2.0]
        .iter()
        .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_handoff_slide_rail_{i}"),
                TONGUE_RAIL_W,
                TONGUE_Y - 28.0,
                TONGUE_RAIL_Z,
            )
            .translate(*x, 0.0, TONGUE_Z / 2.0 + TONGUE_RAIL_Z / 2.0);
    }

    let rear_gate_key = centered_cube(
        "closed_chain_of_custody_sample_tote_station_handoff_rear_pass_through_gate_key",
        TONGUE_X - 220.0,
        18.0,
        32.0,
    )
    .translate(0.0, TONGUE_Y / 2.0 - 20.0, TONGUE_Z / 2.0 + 16.0);
    let robot_pull_slot = centered_cube(
        "closed_chain_of_custody_sample_tote_station_handoff_robot_pull_slot",
        120.0,
        18.0,
        TONGUE_Z + 6.0,
    )
    .translate(0.0, -TONGUE_Y / 2.0 + 24.0, 6.0);

    base - center_washdown_trough - robot_pull_slot + rails + rear_gate_key + tongue_datum_pins()
}

fn tongue_datum_pins() -> Part {
    let mut datums = Part::empty("closed_chain_of_custody_sample_tote_station_handoff_datum_pins");
    for (i, (x, y)) in [
        (-TONGUE_X / 2.0 + 82.0, -TONGUE_Y / 2.0 + 34.0),
        (TONGUE_X / 2.0 - 82.0, -TONGUE_Y / 2.0 + 34.0),
        (-TONGUE_X / 2.0 + 82.0, TONGUE_Y / 2.0 - 34.0),
        (TONGUE_X / 2.0 - 82.0, TONGUE_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_tongue_datum_pin_{i}"),
                7.0,
                10.0,
                28,
            )
            .translate(*x, *y, TONGUE_Z / 2.0 + 5.0);
    }
    datums
}

fn camera_evidence_bridge() -> Part {
    let bottom_z = -CAMERA_BRIDGE_TOTAL_Z / 2.0;
    let post_center_z = bottom_z + CAMERA_BRIDGE_UNDERSIDE_Z / 2.0;
    let beam_center_z = bottom_z + CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0;
    let mut bridge = Part::empty("closed_chain_of_custody_sample_tote_station_camera_bridge");

    for (i, (x, y)) in [
        (
            -CAMERA_BRIDGE_SPAN_X / 2.0 + CAMERA_BRIDGE_POST_X / 2.0,
            -CAMERA_BRIDGE_POST_Y / 2.0,
        ),
        (
            CAMERA_BRIDGE_SPAN_X / 2.0 - CAMERA_BRIDGE_POST_X / 2.0,
            -CAMERA_BRIDGE_POST_Y / 2.0,
        ),
        (
            -CAMERA_BRIDGE_SPAN_X / 2.0 + CAMERA_BRIDGE_POST_X / 2.0,
            CAMERA_BRIDGE_POST_Y / 2.0,
        ),
        (
            CAMERA_BRIDGE_SPAN_X / 2.0 - CAMERA_BRIDGE_POST_X / 2.0,
            CAMERA_BRIDGE_POST_Y / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_camera_bridge_post_{i}"),
                CAMERA_BRIDGE_POST_X,
                CAMERA_BRIDGE_POST_Y,
                CAMERA_BRIDGE_UNDERSIDE_Z,
            )
            .translate(*x, *y, post_center_z);
    }

    let beam = centered_cube(
        "closed_chain_of_custody_sample_tote_station_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, beam_center_z);
    let camera_pods = evidence_camera_pods(bottom_z);
    let led_segments = evidence_led_segments(bottom_z);

    bridge + beam + camera_pods + led_segments
}

fn evidence_camera_pods(bottom_z: f64) -> Part {
    let mut pods = Part::empty("closed_chain_of_custody_sample_tote_station_evidence_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 280.0);
        let pod = centered_cube(
            format!("closed_chain_of_custody_sample_tote_station_evidence_camera_pod_{i}"),
            64.0,
            38.0,
            28.0,
        )
        .translate(x, -CAMERA_BRIDGE_POST_Y / 2.0 - 20.0, bottom_z + 205.0);
        let lens = centered_cylinder(
            format!("closed_chain_of_custody_sample_tote_station_evidence_camera_lens_{i}"),
            9.0,
            18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -CAMERA_BRIDGE_POST_Y / 2.0 - 42.0, bottom_z + 199.0);
        pods = pods + pod + lens;
    }
    pods
}

fn evidence_led_segments(bottom_z: f64) -> Part {
    let mut strips =
        Part::empty("closed_chain_of_custody_sample_tote_station_evidence_led_segments");
    for i in 0..LED_STRIP_COUNT {
        strips = strips
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_evidence_led_segment_{i}"),
                92.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(i, LED_STRIP_COUNT, 125.0),
                CAMERA_BRIDGE_POST_Y / 2.0 + 7.0,
                bottom_z + 202.0,
            );
    }
    strips
}

fn robot_service_keepouts() -> Part {
    let base_z = -KEEP_OUT_Z / 2.0 + 10.0;
    let front_robot = centered_cube(
        "closed_chain_of_custody_sample_tote_station_front_robot_keepout_bar",
        STATION_X - 140.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, base_z);
    let rear_service = centered_cube(
        "closed_chain_of_custody_sample_tote_station_rear_service_keepout_bar",
        STATION_X - 160.0,
        12.0,
        20.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y, base_z);
    let left_service = centered_cube(
        "closed_chain_of_custody_sample_tote_station_left_service_keepout_bar",
        12.0,
        STATION_Y - 180.0,
        20.0,
    )
    .translate(-STATION_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X, 0.0, base_z);
    let right_service = centered_cube(
        "closed_chain_of_custody_sample_tote_station_right_service_keepout_bar",
        12.0,
        STATION_Y - 180.0,
        20.0,
    )
    .translate(STATION_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X, 0.0, base_z);
    let overhead_gauge = centered_cube(
        "closed_chain_of_custody_sample_tote_station_overhead_camera_clearance_gauge",
        CAMERA_BRIDGE_SPAN_X - 120.0,
        16.0,
        18.0,
    )
    .translate(
        CAMERA_BRIDGE_CENTER.0,
        CAMERA_BRIDGE_CENTER.1 + 72.0,
        -KEEP_OUT_Z / 2.0 + OVERHEAD_CAMERA_KEEP_OUT_Z.min(KEEP_OUT_Z + 220.0),
    );

    front_robot + rear_service + left_service + right_service + overhead_gauge
}

fn tote_latch_and_datum_sensors() -> Part {
    let mut sensors =
        Part::empty("closed_chain_of_custody_sample_tote_station_tote_latch_and_datum_sensors");

    for (i, (x, y)) in receiver_locator_points().iter().enumerate() {
        sensors = sensors
            + centered_cylinder(
                format!("closed_chain_of_custody_sample_tote_station_datum_disc_{i}"),
                11.0,
                4.0,
                32,
            )
            .translate(*x, *y, -8.0);
    }

    for (i, (x, y)) in [
        (-RECEIVER_X / 2.0 + 38.0, 0.0),
        (RECEIVER_X / 2.0 - 38.0, 0.0),
        (0.0, -RECEIVER_Y / 2.0 + 34.0),
        (0.0, RECEIVER_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        sensors = sensors
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_tote_present_sensor_{i}"),
                32.0,
                16.0,
                14.0,
            )
            .translate(*x, *y, 0.0);
    }

    for (i, (x, y)) in [
        (-RECEIVER_X / 2.0 + 58.0, -RECEIVER_Y / 2.0 + 26.0),
        (RECEIVER_X / 2.0 - 58.0, -RECEIVER_Y / 2.0 + 26.0),
        (-RECEIVER_X / 2.0 + 58.0, RECEIVER_Y / 2.0 - 26.0),
        (RECEIVER_X / 2.0 - 58.0, RECEIVER_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        sensors = sensors
            + centered_cube(
                format!("closed_chain_of_custody_sample_tote_station_latch_sensor_flag_{i}"),
                24.0,
                14.0,
                24.0,
            )
            .translate(*x, *y, 5.0);
    }

    sensors
}

fn receiver_locator_points() -> [(f64, f64); RECEIVER_LOCATOR_COUNT] {
    [
        (-RECEIVER_X / 2.0 + 48.0, -RECEIVER_Y / 2.0 + 48.0),
        (RECEIVER_X / 2.0 - 48.0, -RECEIVER_Y / 2.0 + 48.0),
        (-RECEIVER_X / 2.0 + 48.0, RECEIVER_Y / 2.0 - 48.0),
        (RECEIVER_X / 2.0 - 48.0, RECEIVER_Y / 2.0 - 48.0),
    ]
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES.len(), STATUS_LANE_PITCH_X)
}

fn rectangular_frame_xy(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    height: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    centered_cube(format!("{name}_outer"), outer_x, outer_y, height)
        - centered_cube(format!("{name}_inner_cut"), inner_x, inner_y, height + 2.0)
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[cfg(test)]
fn rect(center: (f64, f64), width: f64, height: f64) -> Rect {
    Rect {
        x: center.0,
        y: center.1,
        w: width,
        h: height,
    }
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_chain_of_custody_sample_tote_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_match_chain_of_custody_station_request() {
        for feature in [
            "sealed_tote_receiver",
            "rfid_barcode_scan_lands",
            "environmental_logger_pocket",
            "tamper_evident_seal_pockets",
            "released_hold_reject_lanes",
            "chain_of_custody_card_vial_holders",
            "leak_drip_tray",
            "pass_through_handoff_tongue",
            "camera_evidence_bridge",
            "robot_service_keepouts",
            "incubator_isolator_qc_archive_workflow_ports",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn main_modules_fit_station_envelope() {
        assert_layout();
        assert!(component_specs()
            .iter()
            .all(|(_, pos, width, depth)| fits_on_station(*pos, *width, *depth)));
    }

    #[test]
    fn workflow_ports_cover_incubator_isolator_qc_and_archive() {
        assert_eq!(WORKFLOW_NAMES, ["incubator", "isolator", "qc", "archive"]);
        assert_eq!(WORKFLOW_NAMES.len(), 4);
        assert!(WORKFLOW_PORT_PITCH * (WORKFLOW_NAMES.len() as f64 - 1.0) < WORKFLOW_PANEL_X);
        assert!(
            vertical_gap(
                rect(WORKFLOW_PORT_CENTER, WORKFLOW_PANEL_X, WORKFLOW_PANEL_Y),
                rect(RECEIVER_CENTER, RECEIVER_X, RECEIVER_Y)
            ) >= 70.0
        );
    }

    #[test]
    fn disposition_lanes_are_balanced_and_physically_distinct() {
        assert_eq!(STATUS_LANES, ["released", "hold", "reject"]);
        assert_eq!(STATUS_LANES.len(), 3);
        assert_eq!(LANE_SLOTS_PER_STATUS, 4);
        assert_eq!(LANE_SLOTS_PER_STATUS * STATUS_LANES.len(), 12);
        assert!(STATUS_LANE_PITCH_X - STATUS_SLOT_X >= STATUS_LANE_GAP_MIN);
        assert!(STATUS_LANE_PITCH_X * 2.0 + STATUS_SLOT_X < LANES_X);
    }

    #[test]
    fn custody_traceability_and_environment_counts_are_explicit() {
        assert_eq!(BARCODE_LANDS, 10);
        assert_eq!(RFID_LANDS, 6);
        assert_eq!(STATUS_TOKEN_LANDS, 6);
        assert_eq!(LOGGER_POCKETS, 2);
        assert_eq!(LOGGER_CABLE_PORTS, 2);
        assert_eq!(SEAL_POCKET_COUNT, 8);
        assert_eq!(CUSTODY_CARD_SLOTS, 6);
        assert_eq!(VIAL_HOLDERS, 12);
        assert_eq!(VIAL_HOLDERS, VIAL_COLS * 2);
    }

    #[test]
    fn handoff_tongue_camera_bridge_and_keepouts_are_reserved() {
        assert!(TONGUE_X > RECEIVER_X + CUSTODY_X / 2.0);
        assert_eq!(TONGUE_DATUM_PINS, 4);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LED_STRIP_COUNT, 8);
        assert!(CAMERA_BRIDGE_SPAN_X > RECEIVER_X + LANES_X);
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z > RECEIVER_Z + LOGGER_Z);
        assert_eq!(SERVICE_KEEP_OUTS, 4);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 340.0);
        assert!(OVERHEAD_CAMERA_KEEP_OUT_Z >= CAMERA_BRIDGE_UNDERSIDE_Z);
    }

    #[test]
    fn receiver_sensor_and_leak_design_counts_are_locked() {
        assert_eq!(receiver_locator_points().len(), RECEIVER_LOCATOR_COUNT);
        assert_eq!(RECEIVER_LOCATOR_COUNT, 4);
        assert_eq!(DATUM_DISC_COUNT, RECEIVER_LOCATOR_COUNT);
        assert_eq!(TOTE_PRESENT_SENSORS, 4);
        assert_eq!(LATCH_SENSORS, 4);
        assert!(LEAK_RECESS_DEPTH >= 8.0);
        assert!(DRAIN_PORT_D >= 10.0);
        assert!(
            horizontal_gap(
                rect(RECEIVER_CENTER, RECEIVER_X, RECEIVER_Y),
                rect(SCAN_CENTER, SCAN_PANEL_X, SCAN_PANEL_Y)
            ) >= 110.0
        );
    }
}
