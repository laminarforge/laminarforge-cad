use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed post-run cassette unload and custody station for tissue-chip automation.
//
// Intent:
// - Receive a sealed post-run cassette without exposing biological internals.
// - Verify connector caps/plugs, retrieve environmental loggers, scan run-record
//   identifiers, capture evidence images, and route the cassette into released,
//   hold, quarantine, or reject disposition lanes.
// - Present archive/sample handoff ports, leak containment, clean/used physical
//   segregation, and explicit robot/service keepout envelopes for workcell layout.
//
// Research assumptions from the Exa pass:
// - Automated cassette/sample systems commonly rely on cassette-level barcode or
//   RFID traceability, audit trails, and automated storage interfaces.
// - Chain-of-custody workflows benefit from identity scan lands, evidence image
//   capture, and physically distinct disposition locations rather than a single
//   operator-dependent drop zone.
// - Sample management platforms emphasize controlled handoff to archive storage
//   and explicit status tracking; this model represents packaging interfaces only.
//
// This is mechanical packaging CAD for custody workflow planning. It is not a
// biological process model, sterility validation, leak validation, or release SOP.

const OUTPUTS: &[&str] = &[
    "output/closed_post_run_cassette_unload_custody_station_leak_tray_base.stl",
    "output/closed_post_run_cassette_unload_custody_station_sealed_cassette_receiver.stl",
    "output/closed_post_run_cassette_unload_custody_station_disposition_lane_array.stl",
    "output/closed_post_run_cassette_unload_custody_station_cap_plug_verification_pockets.stl",
    "output/closed_post_run_cassette_unload_custody_station_environmental_logger_retrieval_nests.stl",
    "output/closed_post_run_cassette_unload_custody_station_sample_archive_handoff_ports.stl",
    "output/closed_post_run_cassette_unload_custody_station_evidence_camera_bridge.stl",
    "output/closed_post_run_cassette_unload_custody_station_barcode_rfid_run_record_lands.stl",
    "output/closed_post_run_cassette_unload_custody_station_clean_used_segregation_barriers.stl",
    "output/closed_post_run_cassette_unload_custody_station_robot_service_keepouts.stl",
    "output/closed_post_run_cassette_unload_custody_station_assembly.stl",
];

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_GUTTER: f64 = 6.0;
const CHIP_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_MARGIN_X: f64 = 42.0;
const CASSETTE_MARGIN_Y: f64 = 46.0;
const CASSETTE_X: f64 = CHIP_ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = REVC_TOTAL_HEIGHT + 22.0;

const DECK_X: f64 = 2040.0;
const DECK_Y: f64 = 1440.0;
const DECK_Z: f64 = 22.0;
const TRAY_CURB_W: f64 = 18.0;
const TRAY_CURB_Z: f64 = 38.0;
const LEAK_SUMP_X: f64 = 1840.0;
const LEAK_SUMP_Y: f64 = 1230.0;
const LEAK_SUMP_DEPTH: f64 = 9.0;
const DRAIN_CHANNEL_W: f64 = 12.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_HOLE_COUNT: usize = 6;

const RECEIVER_X: f64 = CASSETTE_X + 88.0;
const RECEIVER_Y: f64 = CASSETTE_Y + 120.0;
const RECEIVER_Z: f64 = 116.0;
const RECEIVER_SOCKET_DEPTH: f64 = 10.0;
const RECEIVER_GASKET_W: f64 = 12.0;
const RECEIVER_LATCH_COUNT: usize = 4;
const RECEIVER_X_POS: f64 = -560.0;
const RECEIVER_Y_POS: f64 = 250.0;

const DISPOSITION_LANES: usize = 4;
const LANE_CAPACITY: usize = 1;
const DISPOSITION_SLOTS: usize = DISPOSITION_LANES * LANE_CAPACITY;
const LANE_X: f64 = CASSETTE_X + 42.0;
const LANE_Y: f64 = CASSETTE_Y + 72.0;
const LANE_Z: f64 = 54.0;
const LANE_PITCH_X: f64 = 398.0;
const LANE_START_X: f64 = -594.0;
const LANE_Y_POS: f64 = -265.0;
const LANE_DIVIDER_Z: f64 = 122.0;
const STATUS_PADDLE_X: f64 = 118.0;
const STATUS_PADDLE_Y: f64 = 10.0;
const STATUS_PADDLE_Z: f64 = 54.0;

const CAP_PLUG_ROWS: usize = 2;
const CAP_PLUG_COLS: usize = 8;
const CAP_PLUG_POCKETS: usize = CAP_PLUG_ROWS * CAP_PLUG_COLS;
const CAP_PLUG_BLOCK_X: f64 = 390.0;
const CAP_PLUG_BLOCK_Y: f64 = 118.0;
const CAP_PLUG_BLOCK_Z: f64 = 34.0;
const CAP_POCKET_D: f64 = 13.0;
const PLUG_POCKET_D: f64 = 9.0;
const CAP_PLUG_PITCH_X: f64 = 42.0;
const CAP_PLUG_PITCH_Y: f64 = 42.0;
const CAP_PLUG_X_POS: f64 = 210.0;
const CAP_PLUG_Y_POS: f64 = 360.0;

const LOGGER_NESTS: usize = 6;
const LOGGER_NEST_BLOCK_X: f64 = 330.0;
const LOGGER_NEST_BLOCK_Y: f64 = 116.0;
const LOGGER_NEST_BLOCK_Z: f64 = 38.0;
const LOGGER_SLOT_X: f64 = 42.0;
const LOGGER_SLOT_Y: f64 = 68.0;
const LOGGER_SLOT_Z: f64 = 10.0;
const LOGGER_X_POS: f64 = 680.0;
const LOGGER_Y_POS: f64 = 360.0;

const ARCHIVE_PORTS: usize = 8;
const ARCHIVE_PORT_BLOCK_X: f64 = 470.0;
const ARCHIVE_PORT_BLOCK_Y: f64 = 128.0;
const ARCHIVE_PORT_BLOCK_Z: f64 = 48.0;
const ARCHIVE_PORT_D: f64 = 16.0;
const ARCHIVE_SEAL_D: f64 = 24.0;
const ARCHIVE_PORT_PITCH: f64 = 52.0;
const ARCHIVE_X_POS: f64 = 415.0;
const ARCHIVE_Y_POS: f64 = 320.0;

const CAMERA_BRIDGE_X: f64 = 1040.0;
const CAMERA_BRIDGE_Y: f64 = 94.0;
const CAMERA_BRIDGE_Z: f64 = 320.0;
const CAMERA_POST_X: f64 = 34.0;
const CAMERA_POST_Y: f64 = 54.0;
const CAMERA_BEAM_Z: f64 = 32.0;
const CAMERA_PODS: usize = 3;
const CAMERA_POD_X: f64 = 112.0;
const CAMERA_POD_Y: f64 = 58.0;
const CAMERA_POD_Z: f64 = 44.0;
const CAMERA_X_POS: f64 = -160.0;
const CAMERA_Y_POS: f64 = 500.0;

const ID_LANDS: usize = 14;
const BARCODE_LAND_X: f64 = 86.0;
const BARCODE_LAND_Y: f64 = 36.0;
const RFID_LAND_X: f64 = 104.0;
const RFID_LAND_Y: f64 = 54.0;
const RUN_RECORD_LAND_X: f64 = 168.0;
const RUN_RECORD_LAND_Y: f64 = 62.0;
const ID_BAR_X: f64 = 1560.0;
const ID_BAR_Y: f64 = 88.0;
const ID_BAR_Z: f64 = 14.0;
const ID_BAR_Y_POS: f64 = -620.0;

const CLEAN_USED_BARRIER_X: f64 = 1930.0;
const CLEAN_USED_BARRIER_Y: f64 = 20.0;
const CLEAN_USED_BARRIER_Z: f64 = 142.0;
const CLEAN_USED_BARRIER_Y_POS: f64 = 26.0;
const CLEAN_SUPPLY_KEEP_AWAY_Y: f64 = 220.0;
const USED_CUSTODY_KEEP_AWAY_Y: f64 = 250.0;

const ROBOT_CLEARANCE_X: f64 = DECK_X + 250.0;
const ROBOT_CLEARANCE_Y: f64 = DECK_Y + 210.0;
const ROBOT_CLEARANCE_Z: f64 = 390.0;
const FRONT_SERVICE_CLEARANCE_Y: f64 = 390.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 280.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 240.0;
const CAMERA_SERVICE_CLEARANCE_Z: f64 = 420.0;
const KEEPOUT_COUNT: usize = 5;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(&base, OUTPUTS[0]);

    let receiver = sealed_cassette_receiver();
    export(&receiver, OUTPUTS[1]);

    let lanes = disposition_lane_array();
    export(&lanes, OUTPUTS[2]);

    let cap_plug = cap_plug_verification_pockets();
    export(&cap_plug, OUTPUTS[3]);

    let logger = environmental_logger_retrieval_nests();
    export(&logger, OUTPUTS[4]);

    let archive = sample_archive_handoff_ports();
    export(&archive, OUTPUTS[5]);

    let camera = evidence_camera_bridge();
    export(&camera, OUTPUTS[6]);

    let ids = barcode_rfid_run_record_lands();
    export(&ids, OUTPUTS[7]);

    let barriers = clean_used_segregation_barriers();
    export(&barriers, OUTPUTS[8]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[9]);

    let assembly =
        base + receiver + lanes + cap_plug + logger + archive + camera + ids + barriers + keepouts;
    export(&assembly, OUTPUTS[10]);

    println!();
    println!("Closed post-run cassette unload and custody station:");
    println!("  Deck/leak tray:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm with {LEAK_SUMP_DEPTH:.0}mm recessed sump");
    println!("  Cassette envelope:           {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm x {CASSETTE_Z:.1}mm for {CHIP_COUNT} Rev C chips");
    println!("  Receiver:                    sealed socket {RECEIVER_X:.0}mm x {RECEIVER_Y:.0}mm, {RECEIVER_LATCH_COUNT} latch bosses, gasket witness frame");
    println!("  Disposition lanes:           released/hold/quarantine/reject, {DISPOSITION_SLOTS} cassette slots");
    println!("  Cap/plug verification:       {CAP_PLUG_POCKETS} pockets across cap and plug rows");
    println!("  Logger/archive handoff:      {LOGGER_NESTS} logger nests and {ARCHIVE_PORTS} sealed archive ports");
    println!("  Evidence capture:            {CAMERA_PODS} camera pods on {CAMERA_BRIDGE_Z:.0}mm bridge over receiver and scan lands");
    println!("  Traceability:                {ID_LANDS} barcode/RFID/run-record lands");
    println!("  Keepouts:                    robot {ROBOT_CLEARANCE_X:.0}mm x {ROBOT_CLEARANCE_Y:.0}mm x {ROBOT_CLEARANCE_Z:.0}mm, front {FRONT_SERVICE_CLEARANCE_Y:.0}mm, rear {REAR_SERVICE_CLEARANCE_Y:.0}mm");
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(CHIP_COUNT, 20);
    assert_eq!(DISPOSITION_LANES, 4);
    assert_eq!(DISPOSITION_SLOTS, 4);
    assert_eq!(CAP_PLUG_POCKETS, 16);
    assert!(RECEIVER_Z > CASSETTE_Z);
    assert!(receiver_left_edge() > -DECK_X / 2.0 + TRAY_CURB_W + 20.0);
    assert!(receiver_rear_edge() < DECK_Y / 2.0 - TRAY_CURB_W - 20.0);
    assert!(lane_array_span_x() < DECK_X - 2.0 * TRAY_CURB_W);
    assert!(lane_front_edge() > -DECK_Y / 2.0 + 120.0);
    assert!(clean_used_barrier_clearance() >= CLEAN_SUPPLY_KEEP_AWAY_Y);
    assert!(USED_CUSTODY_KEEP_AWAY_Y >= 240.0);
    assert_eq!(KEEPOUT_COUNT, 5);
    assert!(archive_port_x(0).abs() < ARCHIVE_PORT_BLOCK_X / 2.0 - 36.0);
    assert!(archive_port_x(ARCHIVE_PORTS - 1).abs() < ARCHIVE_PORT_BLOCK_X / 2.0 - 36.0);
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "post_run_custody_leak_tray_cleanable_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        "post_run_custody_recessed_leak_sump",
        LEAK_SUMP_X,
        LEAK_SUMP_Y,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -18.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0 + 0.4);
    let drain_channel = centered_cube(
        "post_run_custody_front_drain_channel",
        LEAK_SUMP_X - 120.0,
        DRAIN_CHANNEL_W,
        LEAK_SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 70.0, DECK_Z - LEAK_SUMP_DEPTH / 2.0);
    let drain_port = centered_cylinder(
        "post_run_custody_bulkhead_drain_port",
        DRAIN_PORT_D / 2.0,
        TRAY_CURB_W + 20.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 28.0, DECK_Z - 9.0);

    deck - sump - drain_channel - drain_port + tray_curbs() + mount_holes_and_datums()
}

fn tray_curbs() -> Part {
    let left = centered_cube(
        "post_run_custody_left_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 40.0,
        TRAY_CURB_Z,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_CURB_W / 2.0,
        8.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "post_run_custody_right_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 40.0,
        TRAY_CURB_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_CURB_W / 2.0,
        8.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let rear = centered_cube(
        "post_run_custody_rear_spill_curb",
        DECK_X - 36.0,
        TRAY_CURB_W,
        TRAY_CURB_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let front_left = centered_cube(
        "post_run_custody_front_low_lip_clean_side",
        740.0,
        12.0,
        20.0,
    )
    .translate(-430.0, -DECK_Y / 2.0 + 18.0, DECK_Z + 10.0);
    let front_right = centered_cube(
        "post_run_custody_front_low_lip_used_side",
        740.0,
        12.0,
        20.0,
    )
    .translate(430.0, -DECK_Y / 2.0 + 18.0, DECK_Z + 10.0);

    left + right + rear + front_left + front_right
}

fn mount_holes_and_datums() -> Part {
    let mut holes = Part::empty("post_run_custody_mount_holes");
    for (i, (x, y)) in mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("post_run_custody_m6_clearance_hole_{i}"),
                6.6 / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }

    let rear_datum = centered_cube(
        "post_run_custody_rear_robot_datum_rail",
        DECK_X - 180.0,
        14.0,
        10.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 74.0, 5.0);
    let left_datum = centered_cube(
        "post_run_custody_left_robot_datum_rail",
        14.0,
        DECK_Y - 180.0,
        10.0,
    )
    .translate(-DECK_X / 2.0 + 78.0, 0.0, 5.0);
    let right_datum = centered_cube(
        "post_run_custody_right_robot_datum_rail",
        14.0,
        DECK_Y - 180.0,
        10.0,
    )
    .translate(DECK_X / 2.0 - 78.0, 0.0, 5.0);

    rear_datum + left_datum + right_datum - holes
}

fn sealed_cassette_receiver() -> Part {
    let plinth = centered_cube(
        "post_run_custody_sealed_receiver_plinth",
        RECEIVER_X,
        RECEIVER_Y,
        28.0,
    )
    .translate(RECEIVER_X_POS, RECEIVER_Y_POS, DECK_Z + 14.0);
    let cassette_shadow = centered_cube(
        "post_run_custody_sealed_receiver_cassette_shadow_recess",
        CASSETTE_X + 12.0,
        CASSETTE_Y + 12.0,
        RECEIVER_SOCKET_DEPTH + 1.0,
    )
    .translate(
        RECEIVER_X_POS,
        RECEIVER_Y_POS,
        DECK_Z + 28.0 - RECEIVER_SOCKET_DEPTH / 2.0,
    );
    let rear_stop = centered_cube(
        "post_run_custody_receiver_rear_hard_stop",
        RECEIVER_X - 46.0,
        16.0,
        76.0,
    )
    .translate(
        RECEIVER_X_POS,
        RECEIVER_Y_POS + RECEIVER_Y / 2.0 - 26.0,
        DECK_Z + 28.0 + 38.0,
    );
    let left_rail = centered_cube(
        "post_run_custody_receiver_left_datum_rail",
        18.0,
        RECEIVER_Y - 72.0,
        68.0,
    )
    .translate(
        RECEIVER_X_POS - RECEIVER_X / 2.0 + 34.0,
        RECEIVER_Y_POS,
        DECK_Z + 28.0 + 34.0,
    );
    let right_rail = centered_cube(
        "post_run_custody_receiver_right_datum_rail",
        18.0,
        RECEIVER_Y - 72.0,
        68.0,
    )
    .translate(
        RECEIVER_X_POS + RECEIVER_X / 2.0 - 34.0,
        RECEIVER_Y_POS,
        DECK_Z + 28.0 + 34.0,
    );
    let gasket = receiver_gasket_witness_frame();
    let latches = receiver_latch_bosses();
    let door_witness = centered_cube(
        "post_run_custody_receiver_closed_door_witness_land",
        186.0,
        12.0,
        48.0,
    )
    .translate(
        RECEIVER_X_POS,
        RECEIVER_Y_POS - RECEIVER_Y / 2.0 - 8.0,
        DECK_Z + 74.0,
    );

    plinth - cassette_shadow + rear_stop + left_rail + right_rail + gasket + latches + door_witness
}

fn receiver_gasket_witness_frame() -> Part {
    let top = centered_cube(
        "post_run_custody_receiver_rear_gasket_witness",
        CASSETTE_X + 46.0,
        RECEIVER_GASKET_W,
        12.0,
    )
    .translate(
        RECEIVER_X_POS,
        RECEIVER_Y_POS + CASSETTE_Y / 2.0 + RECEIVER_GASKET_W / 2.0,
        DECK_Z + 35.0,
    );
    let bottom = centered_cube(
        "post_run_custody_receiver_front_gasket_witness",
        CASSETTE_X + 46.0,
        RECEIVER_GASKET_W,
        12.0,
    )
    .translate(
        RECEIVER_X_POS,
        RECEIVER_Y_POS - CASSETTE_Y / 2.0 - RECEIVER_GASKET_W / 2.0,
        DECK_Z + 35.0,
    );
    let left = centered_cube(
        "post_run_custody_receiver_left_gasket_witness",
        RECEIVER_GASKET_W,
        CASSETTE_Y + 46.0,
        12.0,
    )
    .translate(
        RECEIVER_X_POS - CASSETTE_X / 2.0 - RECEIVER_GASKET_W / 2.0,
        RECEIVER_Y_POS,
        DECK_Z + 35.0,
    );
    let right = centered_cube(
        "post_run_custody_receiver_right_gasket_witness",
        RECEIVER_GASKET_W,
        CASSETTE_Y + 46.0,
        12.0,
    )
    .translate(
        RECEIVER_X_POS + CASSETTE_X / 2.0 + RECEIVER_GASKET_W / 2.0,
        RECEIVER_Y_POS,
        DECK_Z + 35.0,
    );

    top + bottom + left + right
}

fn receiver_latch_bosses() -> Part {
    let mut bosses = Part::empty("post_run_custody_receiver_latch_bosses");
    for (i, (x, y)) in receiver_latch_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("post_run_custody_receiver_latch_boss_{i}"),
            18.0,
            16.0,
            32,
        )
        .translate(x, y, DECK_Z + 50.0);
        let pilot = centered_cylinder(
            format!("post_run_custody_receiver_latch_m4_pilot_{i}"),
            2.3,
            18.0,
            18,
        )
        .translate(x, y, DECK_Z + 50.0);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn disposition_lane_array() -> Part {
    disposition_lane("released", 0, "qa_release_green")
        + disposition_lane("hold", 1, "qa_hold_yellow")
        + disposition_lane("quarantine", 2, "deviation_quarantine_orange")
        + disposition_lane("reject", 3, "reject_red")
        + disposition_lane_dividers()
}

fn disposition_lane(name: &str, lane: usize, label: &str) -> Part {
    let x = lane_center_x(lane);
    let platform = centered_cube(
        format!("post_run_custody_{name}_lane_platform"),
        LANE_X,
        LANE_Y,
        LANE_Z,
    )
    .translate(x, LANE_Y_POS, DECK_Z + LANE_Z / 2.0);
    let cassette_cut = centered_cube(
        format!("post_run_custody_{name}_lane_cassette_shadow_recess"),
        CASSETTE_X + 8.0,
        CASSETTE_Y + 8.0,
        8.0,
    )
    .translate(x, LANE_Y_POS, DECK_Z + LANE_Z - 4.0);
    let rear_stop = centered_cube(
        format!("post_run_custody_{name}_lane_rear_stop"),
        LANE_X - 32.0,
        14.0,
        42.0,
    )
    .translate(x, LANE_Y_POS + LANE_Y / 2.0 - 18.0, DECK_Z + LANE_Z + 21.0);
    let front_status = centered_cube(
        format!("post_run_custody_{label}_status_paddle"),
        STATUS_PADDLE_X,
        STATUS_PADDLE_Y,
        STATUS_PADDLE_Z,
    )
    .translate(
        x,
        LANE_Y_POS - LANE_Y / 2.0 - STATUS_PADDLE_Y / 2.0 - 8.0,
        DECK_Z + LANE_Z + STATUS_PADDLE_Z / 2.0,
    );
    let left_datum = centered_cube(
        format!("post_run_custody_{name}_left_cassette_edge_datum"),
        8.0,
        CASSETTE_Y + 22.0,
        24.0,
    )
    .translate(
        x - CASSETTE_X / 2.0 - 10.0,
        LANE_Y_POS,
        DECK_Z + LANE_Z + 12.0,
    );
    let right_datum = centered_cube(
        format!("post_run_custody_{name}_right_cassette_edge_datum"),
        8.0,
        CASSETTE_Y + 22.0,
        24.0,
    )
    .translate(
        x + CASSETTE_X / 2.0 + 10.0,
        LANE_Y_POS,
        DECK_Z + LANE_Z + 12.0,
    );

    platform - cassette_cut + rear_stop + front_status + left_datum + right_datum
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty("post_run_custody_disposition_lane_hard_dividers");
    for i in 0..DISPOSITION_LANES - 1 {
        let divider_x = (lane_center_x(i) + lane_center_x(i + 1)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("post_run_custody_lane_divider_{i}"),
                20.0,
                LANE_Y + 70.0,
                LANE_DIVIDER_Z,
            )
            .translate(divider_x, LANE_Y_POS, DECK_Z + LANE_DIVIDER_Z / 2.0);
    }
    dividers
}

fn cap_plug_verification_pockets() -> Part {
    let body = centered_cube(
        "post_run_custody_cap_plug_verification_block",
        CAP_PLUG_BLOCK_X,
        CAP_PLUG_BLOCK_Y,
        CAP_PLUG_BLOCK_Z,
    )
    .translate(
        CAP_PLUG_X_POS,
        CAP_PLUG_Y_POS,
        DECK_Z + CAP_PLUG_BLOCK_Z / 2.0,
    );
    let mut cuts = Part::empty("post_run_custody_cap_plug_pocket_cuts");
    let mut labels = Part::empty("post_run_custody_cap_plug_position_labels");
    for row in 0..CAP_PLUG_ROWS {
        for col in 0..CAP_PLUG_COLS {
            let x = CAP_PLUG_X_POS + cap_plug_x(col);
            let y = CAP_PLUG_Y_POS + cap_plug_y(row);
            let diameter = if row == 0 {
                CAP_POCKET_D
            } else {
                PLUG_POCKET_D
            };
            cuts = cuts
                + centered_cylinder(
                    format!("post_run_custody_cap_plug_pocket_r{row}_c{col}"),
                    diameter / 2.0,
                    CAP_PLUG_BLOCK_Z + 2.0,
                    24,
                )
                .translate(x, y, DECK_Z + CAP_PLUG_BLOCK_Z / 2.0);
            labels = labels
                + centered_cube(
                    format!("post_run_custody_cap_plug_vision_label_r{row}_c{col}"),
                    24.0,
                    4.0,
                    2.0,
                )
                .translate(
                    x,
                    y - diameter / 2.0 - 8.0,
                    DECK_Z + CAP_PLUG_BLOCK_Z + 1.0,
                );
        }
    }
    let row_label = centered_cube(
        "post_run_custody_cap_row_and_plug_row_scan_land",
        CAP_PLUG_BLOCK_X - 42.0,
        10.0,
        8.0,
    )
    .translate(
        CAP_PLUG_X_POS,
        CAP_PLUG_Y_POS + CAP_PLUG_BLOCK_Y / 2.0 + 12.0,
        DECK_Z + CAP_PLUG_BLOCK_Z + 4.0,
    );

    body - cuts + labels + row_label
}

fn environmental_logger_retrieval_nests() -> Part {
    let block = centered_cube(
        "post_run_custody_environmental_logger_retrieval_block",
        LOGGER_NEST_BLOCK_X,
        LOGGER_NEST_BLOCK_Y,
        LOGGER_NEST_BLOCK_Z,
    )
    .translate(
        LOGGER_X_POS,
        LOGGER_Y_POS,
        DECK_Z + LOGGER_NEST_BLOCK_Z / 2.0,
    );
    let mut cuts = Part::empty("post_run_custody_logger_slot_cuts");
    let mut retainers = Part::empty("post_run_custody_logger_retainer_springs");
    for i in 0..LOGGER_NESTS {
        let x = LOGGER_X_POS + logger_nest_x(i);
        let slot = centered_cube(
            format!("post_run_custody_logger_retrieval_nest_{i}"),
            LOGGER_SLOT_X,
            LOGGER_SLOT_Y,
            LOGGER_SLOT_Z + 1.0,
        )
        .translate(
            x,
            LOGGER_Y_POS,
            DECK_Z + LOGGER_NEST_BLOCK_Z - LOGGER_SLOT_Z / 2.0,
        );
        let retainer = centered_cube(
            format!("post_run_custody_logger_nest_{i}_spring_clip_witness"),
            LOGGER_SLOT_X - 8.0,
            5.0,
            10.0,
        )
        .translate(
            x,
            LOGGER_Y_POS - LOGGER_SLOT_Y / 2.0 - 6.0,
            DECK_Z + LOGGER_NEST_BLOCK_Z + 5.0,
        );
        cuts = cuts + slot;
        retainers = retainers + retainer;
    }
    let custody_flag = centered_cube(
        "post_run_custody_logger_chain_of_custody_flag_land",
        LOGGER_NEST_BLOCK_X - 40.0,
        8.0,
        16.0,
    )
    .translate(
        LOGGER_X_POS,
        LOGGER_Y_POS + LOGGER_NEST_BLOCK_Y / 2.0 + 8.0,
        DECK_Z + LOGGER_NEST_BLOCK_Z + 8.0,
    );

    block - cuts + retainers + custody_flag
}

fn sample_archive_handoff_ports() -> Part {
    let block = centered_cube(
        "post_run_custody_sample_archive_handoff_port_block",
        ARCHIVE_PORT_BLOCK_X,
        ARCHIVE_PORT_BLOCK_Y,
        ARCHIVE_PORT_BLOCK_Z,
    )
    .translate(
        ARCHIVE_X_POS,
        ARCHIVE_Y_POS,
        DECK_Z + ARCHIVE_PORT_BLOCK_Z / 2.0,
    );
    let mut bores = Part::empty("post_run_custody_archive_port_bores");
    let mut seal_lands = Part::empty("post_run_custody_archive_port_seal_lands");
    for i in 0..ARCHIVE_PORTS {
        let x = ARCHIVE_X_POS + archive_port_x(i);
        let bore = centered_cylinder(
            format!("post_run_custody_archive_handoff_bore_{i}"),
            ARCHIVE_PORT_D / 2.0,
            ARCHIVE_PORT_BLOCK_Z + 2.0,
            28,
        )
        .translate(x, ARCHIVE_Y_POS, DECK_Z + ARCHIVE_PORT_BLOCK_Z / 2.0);
        let seal = centered_cylinder(
            format!("post_run_custody_archive_handoff_seal_land_{i}"),
            ARCHIVE_SEAL_D / 2.0,
            4.0,
            28,
        )
        .translate(x, ARCHIVE_Y_POS, DECK_Z + ARCHIVE_PORT_BLOCK_Z + 2.0);
        let seal_open = centered_cylinder(
            format!("post_run_custody_archive_handoff_seal_opening_{i}"),
            ARCHIVE_PORT_D / 2.0 + 0.5,
            5.0,
            28,
        )
        .translate(x, ARCHIVE_Y_POS, DECK_Z + ARCHIVE_PORT_BLOCK_Z + 2.0);
        bores = bores + bore;
        seal_lands = seal_lands + (seal - seal_open);
    }
    let cold_archive_datum = centered_cube(
        "post_run_custody_archive_transfer_tote_datum_face",
        ARCHIVE_PORT_BLOCK_X - 48.0,
        12.0,
        38.0,
    )
    .translate(
        ARCHIVE_X_POS,
        ARCHIVE_Y_POS + ARCHIVE_PORT_BLOCK_Y / 2.0 + 12.0,
        DECK_Z + ARCHIVE_PORT_BLOCK_Z / 2.0,
    );

    block - bores + seal_lands + cold_archive_datum
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "post_run_custody_evidence_camera_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_X_POS - CAMERA_BRIDGE_X / 2.0 + CAMERA_POST_X / 2.0,
        CAMERA_Y_POS,
        DECK_Z + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "post_run_custody_evidence_camera_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        CAMERA_X_POS + CAMERA_BRIDGE_X / 2.0 - CAMERA_POST_X / 2.0,
        CAMERA_Y_POS,
        DECK_Z + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "post_run_custody_evidence_camera_overhead_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_X_POS,
        CAMERA_Y_POS,
        DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );
    let mut pods = Part::empty("post_run_custody_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = CAMERA_X_POS + camera_pod_x(i);
        let pod = centered_cube(
            format!("post_run_custody_evidence_camera_pod_{i}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            CAMERA_Y_POS,
            DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("post_run_custody_evidence_camera_lens_bore_{i}"),
            12.0,
            CAMERA_POD_Y + 2.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            CAMERA_Y_POS,
            DECK_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - CAMERA_POD_Z / 2.0,
        );
        pods = pods + (pod - lens);
    }

    left_post + right_post + beam + pods
}

fn barcode_rfid_run_record_lands() -> Part {
    let bar = centered_cube(
        "post_run_custody_traceability_scan_bar",
        ID_BAR_X,
        ID_BAR_Y,
        ID_BAR_Z,
    )
    .translate(0.0, ID_BAR_Y_POS, DECK_Z + ID_BAR_Z / 2.0);
    let mut lands = Part::empty("post_run_custody_barcode_rfid_run_record_lands");
    for i in 0..ID_LANDS {
        let x = id_land_x(i);
        let (w, d, name) = if i % 7 == 0 {
            (RUN_RECORD_LAND_X, RUN_RECORD_LAND_Y, "run_record")
        } else if i % 2 == 0 {
            (RFID_LAND_X, RFID_LAND_Y, "rfid")
        } else {
            (BARCODE_LAND_X, BARCODE_LAND_Y, "barcode")
        };
        lands = lands
            + centered_cube(
                format!("post_run_custody_{name}_identity_land_{i}"),
                w,
                d,
                4.0,
            )
            .translate(x, ID_BAR_Y_POS, DECK_Z + ID_BAR_Z + 2.0);
    }
    let witness_strip = centered_cube(
        "post_run_custody_run_record_witness_strip",
        ID_BAR_X - 80.0,
        6.0,
        10.0,
    )
    .translate(
        0.0,
        ID_BAR_Y_POS + ID_BAR_Y / 2.0 + 8.0,
        DECK_Z + ID_BAR_Z + 5.0,
    );

    bar + lands + witness_strip
}

fn clean_used_segregation_barriers() -> Part {
    let main = centered_cube(
        "post_run_custody_clean_used_hard_segregation_barrier",
        CLEAN_USED_BARRIER_X,
        CLEAN_USED_BARRIER_Y,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(
        0.0,
        CLEAN_USED_BARRIER_Y_POS,
        DECK_Z + CLEAN_USED_BARRIER_Z / 2.0,
    );
    let clean_label = centered_cube(
        "post_run_custody_clean_side_archive_only_label_land",
        210.0,
        8.0,
        44.0,
    )
    .translate(
        DECK_X / 2.0 - 180.0,
        CLEAN_USED_BARRIER_Y_POS + CLEAN_USED_BARRIER_Y / 2.0 + 6.0,
        DECK_Z + 86.0,
    );
    let used_label = centered_cube(
        "post_run_custody_used_side_cassette_only_label_land",
        210.0,
        8.0,
        44.0,
    )
    .translate(
        -DECK_X / 2.0 + 180.0,
        CLEAN_USED_BARRIER_Y_POS - CLEAN_USED_BARRIER_Y / 2.0 - 6.0,
        DECK_Z + 86.0,
    );
    let pass_under = centered_cube(
        "post_run_custody_low_archive_handoff_pass_under_window",
        540.0,
        CLEAN_USED_BARRIER_Y + 2.0,
        46.0,
    )
    .translate(ARCHIVE_X_POS, CLEAN_USED_BARRIER_Y_POS, DECK_Z + 23.0);

    main - pass_under + clean_label + used_label
}

fn robot_service_keepouts() -> Part {
    let robot = centered_cube(
        "post_run_custody_robot_swept_volume_keepout",
        ROBOT_CLEARANCE_X,
        ROBOT_CLEARANCE_Y,
        ROBOT_CLEARANCE_Z,
    )
    .translate(0.0, 0.0, DECK_Z + ROBOT_CLEARANCE_Z / 2.0);
    let front = centered_cube(
        "post_run_custody_front_service_pull_keepout",
        DECK_X,
        FRONT_SERVICE_CLEARANCE_Y,
        96.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_SERVICE_CLEARANCE_Y / 2.0,
        DECK_Z + 48.0,
    );
    let rear = centered_cube(
        "post_run_custody_rear_camera_service_keepout",
        DECK_X,
        REAR_SERVICE_CLEARANCE_Y,
        CAMERA_SERVICE_CLEARANCE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE_Y / 2.0,
        DECK_Z + CAMERA_SERVICE_CLEARANCE_Z / 2.0,
    );
    let left = centered_cube(
        "post_run_custody_left_lane_service_keepout",
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y,
        112.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE_X / 2.0,
        0.0,
        DECK_Z + 56.0,
    );
    let right = centered_cube(
        "post_run_custody_right_archive_service_keepout",
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y,
        112.0,
    )
    .translate(
        DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE_X / 2.0,
        0.0,
        DECK_Z + 56.0,
    );

    robot + front + rear + left + right
}

fn mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-DECK_X / 2.0 + 48.0, -DECK_Y / 2.0 + 46.0),
        (DECK_X / 2.0 - 48.0, -DECK_Y / 2.0 + 46.0),
        (-DECK_X / 2.0 + 48.0, DECK_Y / 2.0 - 46.0),
        (DECK_X / 2.0 - 48.0, DECK_Y / 2.0 - 46.0),
        (0.0, -DECK_Y / 2.0 + 46.0),
        (0.0, DECK_Y / 2.0 - 46.0),
    ]
}

fn receiver_latch_points() -> [(f64, f64); RECEIVER_LATCH_COUNT] {
    [
        (
            RECEIVER_X_POS - RECEIVER_X / 2.0 + 62.0,
            RECEIVER_Y_POS - RECEIVER_Y / 2.0 + 54.0,
        ),
        (
            RECEIVER_X_POS + RECEIVER_X / 2.0 - 62.0,
            RECEIVER_Y_POS - RECEIVER_Y / 2.0 + 54.0,
        ),
        (
            RECEIVER_X_POS - RECEIVER_X / 2.0 + 62.0,
            RECEIVER_Y_POS + RECEIVER_Y / 2.0 - 54.0,
        ),
        (
            RECEIVER_X_POS + RECEIVER_X / 2.0 - 62.0,
            RECEIVER_Y_POS + RECEIVER_Y / 2.0 - 54.0,
        ),
    ]
}

fn lane_center_x(lane: usize) -> f64 {
    LANE_START_X + lane as f64 * LANE_PITCH_X
}

fn cap_plug_x(col: usize) -> f64 {
    (col as f64 - (CAP_PLUG_COLS as f64 - 1.0) / 2.0) * CAP_PLUG_PITCH_X
}

fn cap_plug_y(row: usize) -> f64 {
    (row as f64 - (CAP_PLUG_ROWS as f64 - 1.0) / 2.0) * CAP_PLUG_PITCH_Y
}

fn logger_nest_x(index: usize) -> f64 {
    (index as f64 - (LOGGER_NESTS as f64 - 1.0) / 2.0) * 50.0
}

fn archive_port_x(index: usize) -> f64 {
    (index as f64 - (ARCHIVE_PORTS as f64 - 1.0) / 2.0) * ARCHIVE_PORT_PITCH
}

fn camera_pod_x(index: usize) -> f64 {
    (index as f64 - (CAMERA_PODS as f64 - 1.0) / 2.0) * 310.0
}

fn id_land_x(index: usize) -> f64 {
    (index as f64 - (ID_LANDS as f64 - 1.0) / 2.0) * 112.0
}

fn receiver_left_edge() -> f64 {
    RECEIVER_X_POS - RECEIVER_X / 2.0
}

fn receiver_rear_edge() -> f64 {
    RECEIVER_Y_POS + RECEIVER_Y / 2.0
}

fn lane_array_span_x() -> f64 {
    (lane_center_x(DISPOSITION_LANES - 1) + LANE_X / 2.0) - (lane_center_x(0) - LANE_X / 2.0)
}

fn lane_front_edge() -> f64 {
    LANE_Y_POS - LANE_Y / 2.0
}

fn clean_used_barrier_clearance() -> f64 {
    (ARCHIVE_Y_POS - ARCHIVE_PORT_BLOCK_Y / 2.0) - CLEAN_USED_BARRIER_Y_POS
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_post_run_cassette_unload_custody_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cassette_receiver_geometry_matches_rev_c_array() {
        assert_eq!(CHIP_COUNT, 20);
        assert!(CASSETTE_X > CHIP_ARRAY_X);
        assert!(CASSETTE_Y > CHIP_ARRAY_Y);
        assert!(CASSETTE_Z > REVC_TOTAL_HEIGHT);
        assert!(RECEIVER_Z > CASSETTE_Z);
        assert!(RECEIVER_X > CASSETTE_X + 2.0 * RECEIVER_GASKET_W);
        assert!(RECEIVER_Y > CASSETTE_Y + 2.0 * RECEIVER_GASKET_W);
        assert_eq!(receiver_latch_points().len(), RECEIVER_LATCH_COUNT);
    }

    #[test]
    fn disposition_lanes_cover_required_statuses_and_fit() {
        assert_eq!(DISPOSITION_LANES, 4);
        assert_eq!(DISPOSITION_SLOTS, 4);
        assert!(lane_center_x(0) < lane_center_x(1));
        assert!(lane_center_x(1) < lane_center_x(2));
        assert!(lane_center_x(2) < lane_center_x(3));
        assert!(lane_array_span_x() < DECK_X - 100.0);
        assert!(lane_front_edge() > -DECK_Y / 2.0 + 100.0);
    }

    #[test]
    fn custody_accessories_have_expected_counts() {
        assert_eq!(CAP_PLUG_ROWS, 2);
        assert_eq!(CAP_PLUG_POCKETS, 16);
        assert_eq!(LOGGER_NESTS, 6);
        assert_eq!(ARCHIVE_PORTS, 8);
        assert_eq!(CAMERA_PODS, 3);
        assert_eq!(ID_LANDS, 14);
        assert!(CAP_POCKET_D > PLUG_POCKET_D);
        assert!(ARCHIVE_SEAL_D > ARCHIVE_PORT_D);
    }

    #[test]
    fn array_coordinates_are_centered_and_clear() {
        assert!((cap_plug_x(0) + cap_plug_x(CAP_PLUG_COLS - 1)).abs() < 1.0e-9);
        assert!((cap_plug_y(0) + cap_plug_y(CAP_PLUG_ROWS - 1)).abs() < 1.0e-9);
        assert!((logger_nest_x(0) + logger_nest_x(LOGGER_NESTS - 1)).abs() < 1.0e-9);
        assert!((archive_port_x(0) + archive_port_x(ARCHIVE_PORTS - 1)).abs() < 1.0e-9);
        assert!(archive_port_x(ARCHIVE_PORTS - 1) < ARCHIVE_PORT_BLOCK_X / 2.0 - 34.0);
        assert!(id_land_x(ID_LANDS - 1) < ID_BAR_X / 2.0 - 40.0);
    }

    #[test]
    fn leak_tray_segregation_and_keepouts_are_explicit() {
        assert_eq!(mount_points().len(), MOUNT_HOLE_COUNT);
        assert_eq!(KEEPOUT_COUNT, 5);
        assert!(LEAK_SUMP_X < DECK_X - 2.0 * TRAY_CURB_W);
        assert!(LEAK_SUMP_Y < DECK_Y - 2.0 * TRAY_CURB_W);
        assert!(clean_used_barrier_clearance() >= CLEAN_SUPPLY_KEEP_AWAY_Y);
        assert!(USED_CUSTODY_KEEP_AWAY_Y >= 240.0);
        assert!(ROBOT_CLEARANCE_Z >= CAMERA_BRIDGE_Z);
        assert!(CAMERA_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
    }
}
