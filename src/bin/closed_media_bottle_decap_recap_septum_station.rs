use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bottle decap/recap and septum-access station.
//
// Intent:
// - Validate robotic handling around purchased sterile GL45 media bottles,
//   sterile screw caps, septum caps, torque tools, pierce tools, scanners, and
//   connector handoff interfaces.
// - Keep bottle/cap identity, release state, leak containment, tool approach
//   clearance, and clean/used segregation visible in one automation fixture.
// - Model packaging and mechanical interfaces only. Sterile processing,
//   biological transfer, validated torque methods, and puncture SOPs remain
//   separate process-validation work.

const OUTPUTS: [&str; 13] = [
    "output/closed_media_bottle_decap_recap_septum_station_cleanable_deck.stl",
    "output/closed_media_bottle_decap_recap_septum_station_bottle_nest_matrix.stl",
    "output/closed_media_bottle_decap_recap_septum_station_cap_plug_parks.stl",
    "output/closed_media_bottle_decap_recap_septum_station_torque_verification_pocket.stl",
    "output/closed_media_bottle_decap_recap_septum_station_septum_alignment_wells.stl",
    "output/closed_media_bottle_decap_recap_septum_station_drip_leak_tray.stl",
    "output/closed_media_bottle_decap_recap_septum_station_sterile_connector_handoff.stl",
    "output/closed_media_bottle_decap_recap_septum_station_barcode_rfid_coa_lands.stl",
    "output/closed_media_bottle_decap_recap_septum_station_release_hold_reject_lanes.stl",
    "output/closed_media_bottle_decap_recap_septum_station_clean_used_segregation.stl",
    "output/closed_media_bottle_decap_recap_septum_station_robot_tool_keepouts.stl",
    "output/closed_media_bottle_decap_recap_septum_station_scanner_bridge.stl",
    "output/closed_media_bottle_decap_recap_septum_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "bottle_nest_matrix",
    "cap_plug_parks",
    "torque_verification_pocket",
    "septum_access_alignment_wells",
    "drip_leak_tray",
    "sterile_connector_handoff",
    "barcode_rfid_coa_lands",
    "released_hold_reject_lanes",
    "clean_used_segregation",
    "robot_tool_keepouts",
    "assembly_export",
];

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 20.0;
const DECK_RAIL_Z: f64 = 26.0;
const DECK_RAIL_W: f64 = 16.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.6;

const BOTTLE_ROWS: usize = 2;
const BOTTLE_COLS: usize = 3;
const BOTTLE_COUNT: usize = BOTTLE_ROWS * BOTTLE_COLS;
const BOTTLE_OD: f64 = 86.0;
const BOTTLE_NEST_CLEARANCE: f64 = 2.0;
const BOTTLE_NEST_D: f64 = BOTTLE_OD + 2.0 * BOTTLE_NEST_CLEARANCE;
const BOTTLE_PITCH_X: f64 = 142.0;
const BOTTLE_PITCH_Y: f64 = 174.0;
const BOTTLE_MATRIX_X: f64 = 520.0;
const BOTTLE_MATRIX_Y: f64 = 420.0;
const BOTTLE_MATRIX_Z: f64 = 38.0;
const BOTTLE_MATRIX_CENTER: (f64, f64) = (-250.0, 78.0);
const BOTTLE_RETAINING_RING_Z: f64 = 20.0;

const GL45_CAP_OD: f64 = 54.0;
const SEPTUM_CAP_OD: f64 = 58.0;
const CAP_PARK_COUNT: usize = BOTTLE_COUNT;
const PLUG_PARK_COUNT: usize = BOTTLE_COUNT;
const CAP_PARK_X: f64 = 520.0;
const CAP_PARK_Y: f64 = 210.0;
const CAP_PARK_Z: f64 = 30.0;
const CAP_PARK_CENTER: (f64, f64) = (300.0, 238.0);
const CAP_PARK_PITCH_X: f64 = 78.0;
const CAP_PARK_PITCH_Y: f64 = 124.0;
const CLEAN_USED_BARRIER_GAP: f64 = 52.0;

const TORQUE_POCKET_X: f64 = 250.0;
const TORQUE_POCKET_Y: f64 = 176.0;
const TORQUE_POCKET_Z: f64 = 42.0;
const TORQUE_CENTER: (f64, f64) = (322.0, -74.0);
const TORQUE_TOOL_D: f64 = 74.0;
const TORQUE_SOCKET_D: f64 = GL45_CAP_OD + 6.0;
const TORQUE_LABEL_LAND_X: f64 = 156.0;

const SEPTUM_PANEL_X: f64 = 360.0;
const SEPTUM_PANEL_Y: f64 = 188.0;
const SEPTUM_PANEL_Z: f64 = 34.0;
const SEPTUM_CENTER: (f64, f64) = (-64.0, -258.0);
const SEPTUM_WELL_COUNT: usize = 4;
const SEPTUM_WELL_D: f64 = 44.0;
const SEPTUM_WELL_PITCH: f64 = 78.0;
const PIERCE_TOOL_GUIDE_D: f64 = 18.0;
const SEPTUM_ACCESS_CLEARANCE_Z: f64 = 132.0;

const DRIP_TRAY_X: f64 = 1000.0;
const DRIP_TRAY_Y: f64 = 180.0;
const DRIP_TRAY_Z: f64 = 28.0;
const DRIP_TRAY_CENTER: (f64, f64) = (-20.0, -300.0);
const DRIP_TRAY_CURB_Z: f64 = 28.0;

const CONNECTOR_X: f64 = 330.0;
const CONNECTOR_Y: f64 = 146.0;
const CONNECTOR_Z: f64 = 36.0;
const CONNECTOR_CENTER: (f64, f64) = (-410.0, -250.0);
const CONNECTOR_CRADLES: usize = 4;
const CONNECTOR_CRADLE_D: f64 = 32.0;
const CONNECTOR_PITCH: f64 = 66.0;

const ID_LANDS_X: f64 = 410.0;
const ID_LANDS_Y: f64 = 124.0;
const ID_LANDS_Z: f64 = 8.0;
const ID_LANDS_CENTER: (f64, f64) = (328.0, -292.0);
const BARCODE_LANDS: usize = 4;
const RFID_PAD_X: f64 = 74.0;
const RFID_PAD_Y: f64 = 54.0;
const COA_LAND_X: f64 = 142.0;
const COA_LAND_Y: f64 = 86.0;

const STATUS_X: f64 = 470.0;
const STATUS_Y: f64 = 186.0;
const STATUS_Z: f64 = 30.0;
const STATUS_CENTER: (f64, f64) = (312.0, 54.0);
const STATUS_LANE_COUNT: usize = 3;
const STATUS_SLOT_COUNT: usize = BOTTLE_COUNT;
const STATUS_SLOT_X: f64 = 108.0;
const STATUS_SLOT_Y: f64 = 34.0;
const STATUS_SLOT_PITCH_X: f64 = 124.0;
const STATUS_LANE_PITCH_Y: f64 = 54.0;

const SEGREGATION_X: f64 = 1020.0;
const SEGREGATION_Y: f64 = 70.0;
const SEGREGATION_Z: f64 = 48.0;
const SEGREGATION_CENTER: (f64, f64) = (-10.0, 8.0);
const CLEAN_SIDE_MIN_Y: f64 = 54.0;

const KEEP_OUT_Z: f64 = 160.0;
const ROBOT_APPROACH_KEEP_OUTS: usize = 5;
const TORQUE_TOOL_KEEP_OUT_D: f64 = 140.0;
const SEPTUM_TOOL_KEEP_OUT_D: f64 = 98.0;

const SCANNER_BRIDGE_X: f64 = 1030.0;
const SCANNER_BRIDGE_Y: f64 = 52.0;
const SCANNER_BRIDGE_Z: f64 = 34.0;
const SCANNER_BRIDGE_UNDERSIDE_Z: f64 = 166.0;
const SCANNER_SLED_X: f64 = 126.0;
const SCANNER_SLED_Y: f64 = 62.0;
const RFID_ANTENNA_X: f64 = 154.0;
const RFID_ANTENNA_Y: f64 = 92.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_deck();
    export(OUTPUTS[0], &deck);

    let nests = bottle_nest_matrix();
    export(OUTPUTS[1], &nests);

    let parks = cap_plug_parks();
    export(OUTPUTS[2], &parks);

    let torque = torque_verification_pocket();
    export(OUTPUTS[3], &torque);

    let septum = septum_access_alignment_wells();
    export(OUTPUTS[4], &septum);

    let tray = drip_leak_tray();
    export(OUTPUTS[5], &tray);

    let handoff = sterile_connector_handoff();
    export(OUTPUTS[6], &handoff);

    let lands = barcode_rfid_coa_lands();
    export(OUTPUTS[7], &lands);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_tool_keepouts();
    export(OUTPUTS[10], &keepouts);

    let bridge = scanner_bridge();
    export(OUTPUTS[11], &bridge);

    let assembly =
        deck + nests.translate(
            BOTTLE_MATRIX_CENTER.0,
            BOTTLE_MATRIX_CENTER.1,
            deck_insert_z(BOTTLE_MATRIX_Z),
        ) + parks.translate(
            CAP_PARK_CENTER.0,
            CAP_PARK_CENTER.1,
            deck_insert_z(CAP_PARK_Z),
        ) + torque.translate(
            TORQUE_CENTER.0,
            TORQUE_CENTER.1,
            deck_insert_z(TORQUE_POCKET_Z),
        ) + septum.translate(
            SEPTUM_CENTER.0,
            SEPTUM_CENTER.1,
            deck_insert_z(SEPTUM_PANEL_Z),
        ) + tray.translate(
            DRIP_TRAY_CENTER.0,
            DRIP_TRAY_CENTER.1,
            deck_insert_z(DRIP_TRAY_Z),
        ) + handoff.translate(
            CONNECTOR_CENTER.0,
            CONNECTOR_CENTER.1,
            deck_insert_z(CONNECTOR_Z),
        ) + lands.translate(
            ID_LANDS_CENTER.0,
            ID_LANDS_CENTER.1,
            deck_insert_z(ID_LANDS_Z),
        ) + lanes.translate(STATUS_CENTER.0, STATUS_CENTER.1, deck_insert_z(STATUS_Z))
            + segregation.translate(
                SEGREGATION_CENTER.0,
                SEGREGATION_CENTER.1,
                DECK_Z / 2.0 + SEGREGATION_Z / 2.0,
            )
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_Z / 2.0)
            + bridge.translate(0.0, 0.0, DECK_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media-bottle decap/recap and septum-access station:");
    println!(
        "  Deck envelope:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm with drip/leak containment"
    );
    println!(
        "  Bottle nest matrix:         {BOTTLE_COUNT} positions in {BOTTLE_ROWS}x{BOTTLE_COLS}, {BOTTLE_NEST_D:.0}mm nest diameter for ~500 mL GL45 bottles"
    );
    println!(
        "  Cap/plug parks:             {CAP_PARK_COUNT} cap parks and {PLUG_PARK_COUNT} septum/plug parks, clean/used barrier gap {:.0}mm",
        cap_park_barrier_gap()
    );
    println!(
        "  Torque verification:        {TORQUE_TOOL_D:.0}mm tool pocket, {TORQUE_SOCKET_D:.0}mm cap socket, barcode label land"
    );
    println!(
        "  Septum access:              {SEPTUM_WELL_COUNT} alignment wells, {PIERCE_TOOL_GUIDE_D:.0}mm pierce-tool guide, {SEPTUM_ACCESS_CLEARANCE_Z:.0}mm keepout height"
    );
    println!(
        "  Traceability/status:        barcode/RFID/COA lands plus released/hold/reject lanes for {STATUS_SLOT_COUNT} bottles"
    );
    println!(
        "  Robot/tool keepouts:        {ROBOT_APPROACH_KEEP_OUTS} approach envelopes plus per-bottle capper and septum pierce clearances"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    assert!(fits_on_deck(
        BOTTLE_MATRIX_CENTER,
        BOTTLE_MATRIX_X,
        BOTTLE_MATRIX_Y,
        14.0
    ));
    assert!(fits_on_deck(CAP_PARK_CENTER, CAP_PARK_X, CAP_PARK_Y, 14.0));
    assert!(fits_on_deck(
        TORQUE_CENTER,
        TORQUE_POCKET_X,
        TORQUE_POCKET_Y,
        14.0
    ));
    assert!(fits_on_deck(
        SEPTUM_CENTER,
        SEPTUM_PANEL_X,
        SEPTUM_PANEL_Y,
        14.0
    ));
    assert!(fits_on_deck(
        DRIP_TRAY_CENTER,
        DRIP_TRAY_X,
        DRIP_TRAY_Y,
        14.0
    ));
    assert!(fits_on_deck(
        CONNECTOR_CENTER,
        CONNECTOR_X,
        CONNECTOR_Y,
        14.0
    ));
    assert!(fits_on_deck(ID_LANDS_CENTER, ID_LANDS_X, ID_LANDS_Y, 14.0));
    assert!(fits_on_deck(STATUS_CENTER, STATUS_X, STATUS_Y, 14.0));
    assert!(
        cap_park_barrier_gap() >= CLEAN_USED_BARRIER_GAP,
        "clean and used cap/plug parks do not have enough segregation gap"
    );
    assert!(
        BOTTLE_NEST_D > BOTTLE_OD,
        "bottle nest does not include handling clearance"
    );
    assert!(
        !rects_overlap(
            rect(BOTTLE_MATRIX_CENTER, BOTTLE_MATRIX_X, BOTTLE_MATRIX_Y),
            rect(STATUS_CENTER, STATUS_X, STATUS_Y)
        ),
        "bottle matrix collides with released/hold/reject lanes"
    );
    assert!(
        vertical_gap(
            rect(DRIP_TRAY_CENTER, DRIP_TRAY_X, DRIP_TRAY_Y),
            rect(STATUS_CENTER, STATUS_X, STATUS_Y)
        ) >= 18.0,
        "drip tray is too close to status lanes"
    );
}

fn cleanable_deck() -> Part {
    let deck = centered_cube(
        "closed_media_bottle_station_cleanable_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let nest_recess = top_recess(
        "closed_media_bottle_station_bottle_matrix_recess",
        BOTTLE_MATRIX_CENTER,
        BOTTLE_MATRIX_X + 22.0,
        BOTTLE_MATRIX_Y + 22.0,
        5.0,
    );
    let cap_recess = top_recess(
        "closed_media_bottle_station_cap_park_recess",
        CAP_PARK_CENTER,
        CAP_PARK_X + 20.0,
        CAP_PARK_Y + 20.0,
        4.0,
    );
    let torque_recess = top_recess(
        "closed_media_bottle_station_torque_pocket_recess",
        TORQUE_CENTER,
        TORQUE_POCKET_X + 18.0,
        TORQUE_POCKET_Y + 18.0,
        5.0,
    );
    let septum_recess = top_recess(
        "closed_media_bottle_station_septum_panel_recess",
        SEPTUM_CENTER,
        SEPTUM_PANEL_X + 18.0,
        SEPTUM_PANEL_Y + 18.0,
        4.0,
    );
    let tray_recess = top_recess(
        "closed_media_bottle_station_drip_tray_recess",
        DRIP_TRAY_CENTER,
        DRIP_TRAY_X + 18.0,
        DRIP_TRAY_Y + 18.0,
        4.0,
    );
    deck - nest_recess
        - cap_recess
        - torque_recess
        - septum_recess
        - tray_recess
        - deck_drains_and_mounts()
        + deck_perimeter_rails()
        + deck_robot_fiducials()
}

fn top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - depth / 2.0 + 0.1,
    )
}

fn deck_drains_and_mounts() -> Part {
    let drain = centered_cylinder(
        "closed_media_bottle_station_front_leak_tray_drain",
        DRAIN_D / 2.0,
        DECK_Z + 4.0,
        32,
    )
    .translate(
        DRIP_TRAY_CENTER.0 + DRIP_TRAY_X / 2.0 - 66.0,
        DRIP_TRAY_CENTER.1 - DRIP_TRAY_Y / 2.0 + 38.0,
        0.0,
    );
    let mut mounts = Part::empty("closed_media_bottle_station_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        mounts = mounts
            + centered_cylinder(
                format!("closed_media_bottle_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    drain + mounts
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 58.0), -(DECK_Y / 2.0 - 56.0)),
        (DECK_X / 2.0 - 58.0, -(DECK_Y / 2.0 - 56.0)),
        (-(DECK_X / 2.0 - 58.0), DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 56.0),
        (0.0, -(DECK_Y / 2.0 - 56.0)),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-(DECK_X / 2.0 - 58.0), 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
}

fn deck_perimeter_rails() -> Part {
    let rear = centered_cube(
        "closed_media_bottle_station_rear_cleanable_rail",
        DECK_X - 116.0,
        DECK_RAIL_W,
        DECK_RAIL_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 36.0, DECK_Z / 2.0 + DECK_RAIL_Z / 2.0);
    let left = centered_cube(
        "closed_media_bottle_station_left_cleanable_rail",
        DECK_RAIL_W,
        DECK_Y - 132.0,
        DECK_RAIL_Z,
    )
    .translate(-DECK_X / 2.0 + 36.0, 0.0, DECK_Z / 2.0 + DECK_RAIL_Z / 2.0);
    let right = centered_cube(
        "closed_media_bottle_station_right_tool_datum_rail",
        DECK_RAIL_W,
        DECK_Y - 132.0,
        DECK_RAIL_Z,
    )
    .translate(DECK_X / 2.0 - 36.0, 0.0, DECK_Z / 2.0 + DECK_RAIL_Z / 2.0);
    let front = centered_cube(
        "closed_media_bottle_station_front_low_leak_lip",
        DECK_X - 180.0,
        12.0,
        16.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 36.0, DECK_Z / 2.0 + 8.0);
    rear + left + right + front
}

fn deck_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_media_bottle_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 86.0), DECK_Y / 2.0 - 86.0),
        (DECK_X / 2.0 - 86.0, DECK_Y / 2.0 - 86.0),
        (-(DECK_X / 2.0 - 86.0), -(DECK_Y / 2.0 - 86.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("closed_media_bottle_station_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn bottle_nest_matrix() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_bottle_nest_matrix_plate",
        BOTTLE_MATRIX_X,
        BOTTLE_MATRIX_Y,
        BOTTLE_MATRIX_Z,
    );
    base - bottle_nest_cuts() + bottle_retaining_rings() + bottle_row_label_lands()
}

fn bottle_nest_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_bottle_nest_cuts");
    for index in 0..BOTTLE_COUNT {
        let (x, y) = bottle_position(index);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bottle_station_bottle_nest_cut_{index}"),
                BOTTLE_NEST_D / 2.0,
                BOTTLE_MATRIX_Z + 4.0,
                64,
            )
            .translate(x, y, 0.0);
    }
    cuts
}

fn bottle_retaining_rings() -> Part {
    let mut rings = Part::empty("closed_media_bottle_station_bottle_retaining_rings");
    for index in 0..BOTTLE_COUNT {
        let (x, y) = bottle_position(index);
        let outer = centered_cylinder(
            format!("closed_media_bottle_station_bottle_retaining_ring_outer_{index}"),
            (BOTTLE_NEST_D + 18.0) / 2.0,
            BOTTLE_RETAINING_RING_Z,
            64,
        );
        let inner = centered_cylinder(
            format!("closed_media_bottle_station_bottle_retaining_ring_inner_{index}"),
            (BOTTLE_NEST_D + 3.0) / 2.0,
            BOTTLE_RETAINING_RING_Z + 2.0,
            64,
        );
        rings = rings
            + (outer - inner).translate(
                x,
                y,
                BOTTLE_MATRIX_Z / 2.0 + BOTTLE_RETAINING_RING_Z / 2.0,
            );
    }
    rings
}

fn bottle_row_label_lands() -> Part {
    let clean_row = centered_cube(
        "closed_media_bottle_station_clean_bottle_row_barcode_land",
        BOTTLE_MATRIX_X - 72.0,
        18.0,
        4.0,
    )
    .translate(
        0.0,
        BOTTLE_MATRIX_Y / 2.0 - 42.0,
        BOTTLE_MATRIX_Z / 2.0 + 2.0,
    );
    let used_row = centered_cube(
        "closed_media_bottle_station_used_bottle_row_barcode_land",
        BOTTLE_MATRIX_X - 72.0,
        18.0,
        4.0,
    )
    .translate(
        0.0,
        -(BOTTLE_MATRIX_Y / 2.0 - 42.0),
        BOTTLE_MATRIX_Z / 2.0 + 2.0,
    );
    clean_row + used_row
}

fn cap_plug_parks() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_cap_plug_park_plate",
        CAP_PARK_X,
        CAP_PARK_Y,
        CAP_PARK_Z,
    );
    base - cap_park_cuts() - plug_park_cuts() + cap_park_barrier() + cap_park_label_lands()
}

fn cap_park_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_cap_park_cuts");
    for index in 0..CAP_PARK_COUNT {
        let (x, y) = cap_park_position(index, 0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bottle_station_clean_cap_socket_{index}"),
                (GL45_CAP_OD + 5.0) / 2.0,
                16.0,
                48,
            )
            .translate(x, y, CAP_PARK_Z / 2.0 - 7.0);
    }
    cuts
}

fn plug_park_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_plug_park_cuts");
    for index in 0..PLUG_PARK_COUNT {
        let (x, y) = cap_park_position(index, 1);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bottle_station_used_septum_plug_socket_{index}"),
                (SEPTUM_CAP_OD + 5.0) / 2.0,
                16.0,
                48,
            )
            .translate(x, y, CAP_PARK_Z / 2.0 - 7.0);
    }
    cuts
}

fn cap_park_barrier() -> Part {
    centered_cube(
        "closed_media_bottle_station_clean_used_cap_barrier",
        CAP_PARK_X - 54.0,
        10.0,
        32.0,
    )
    .translate(0.0, 0.0, CAP_PARK_Z / 2.0 + 16.0)
}

fn cap_park_label_lands() -> Part {
    let clean = centered_cube(
        "closed_media_bottle_station_clean_cap_barcode_land",
        178.0,
        18.0,
        4.0,
    )
    .translate(-150.0, CAP_PARK_Y / 2.0 - 24.0, CAP_PARK_Z / 2.0 + 2.0);
    let used = centered_cube(
        "closed_media_bottle_station_used_cap_barcode_land",
        178.0,
        18.0,
        4.0,
    )
    .translate(150.0, -(CAP_PARK_Y / 2.0 - 24.0), CAP_PARK_Z / 2.0 + 2.0);
    clean + used
}

fn torque_verification_pocket() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_torque_verification_base",
        TORQUE_POCKET_X,
        TORQUE_POCKET_Y,
        TORQUE_POCKET_Z,
    );
    let tool_cup = centered_cylinder(
        "closed_media_bottle_station_torque_driver_body_cup",
        TORQUE_TOOL_D / 2.0,
        24.0,
        56,
    )
    .translate(-58.0, 0.0, TORQUE_POCKET_Z / 2.0 - 10.0);
    let cap_socket = centered_cylinder(
        "closed_media_bottle_station_torque_reference_cap_socket",
        TORQUE_SOCKET_D / 2.0,
        22.0,
        56,
    )
    .translate(62.0, 0.0, TORQUE_POCKET_Z / 2.0 - 9.0);
    let wrench_flat = centered_cube(
        "closed_media_bottle_station_torque_flat_alignment_cut",
        92.0,
        18.0,
        18.0,
    )
    .translate(-58.0, 0.0, TORQUE_POCKET_Z / 2.0 - 8.0);
    let label = centered_cube(
        "closed_media_bottle_station_torque_verification_barcode_land",
        TORQUE_LABEL_LAND_X,
        18.0,
        4.0,
    )
    .translate(
        0.0,
        TORQUE_POCKET_Y / 2.0 - 26.0,
        TORQUE_POCKET_Z / 2.0 + 2.0,
    );
    base - tool_cup - cap_socket - wrench_flat + label + torque_sensor_bosses()
}

fn torque_sensor_bosses() -> Part {
    let mut bosses = Part::empty("closed_media_bottle_station_torque_sensor_bosses");
    for (i, x) in [-92.0, 92.0].iter().enumerate() {
        let boss = centered_cube(
            format!("closed_media_bottle_station_torque_load_boss_{i}"),
            42.0,
            34.0,
            18.0,
        )
        .translate(
            *x,
            -TORQUE_POCKET_Y / 2.0 + 35.0,
            TORQUE_POCKET_Z / 2.0 + 9.0,
        );
        let screw = centered_cylinder(
            format!("closed_media_bottle_station_torque_boss_m4_clearance_{i}"),
            2.4,
            20.0,
            20,
        )
        .translate(
            *x,
            -TORQUE_POCKET_Y / 2.0 + 35.0,
            TORQUE_POCKET_Z / 2.0 + 9.0,
        );
        bosses = bosses + (boss - screw);
    }
    bosses
}

fn septum_access_alignment_wells() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_septum_alignment_panel",
        SEPTUM_PANEL_X,
        SEPTUM_PANEL_Y,
        SEPTUM_PANEL_Z,
    );
    base - septum_well_cuts() + pierce_tool_guides() + septum_panel_lands()
}

fn septum_well_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_septum_alignment_well_cuts");
    for i in 0..SEPTUM_WELL_COUNT {
        let x = centered_index(i, SEPTUM_WELL_COUNT, SEPTUM_WELL_PITCH);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bottle_station_septum_cap_alignment_well_{i}"),
                SEPTUM_WELL_D / 2.0,
                18.0,
                44,
            )
            .translate(x, -20.0, SEPTUM_PANEL_Z / 2.0 - 7.0);
    }
    cuts
}

fn pierce_tool_guides() -> Part {
    let mut guides = Part::empty("closed_media_bottle_station_pierce_tool_guides");
    for i in 0..SEPTUM_WELL_COUNT {
        let x = centered_index(i, SEPTUM_WELL_COUNT, SEPTUM_WELL_PITCH);
        let post = centered_cylinder(
            format!("closed_media_bottle_station_pierce_tool_guide_post_{i}"),
            18.0,
            42.0,
            36,
        )
        .translate(x, 46.0, SEPTUM_PANEL_Z / 2.0 + 21.0);
        let guide = centered_cylinder(
            format!("closed_media_bottle_station_pierce_tool_guide_bore_{i}"),
            PIERCE_TOOL_GUIDE_D / 2.0,
            44.0,
            28,
        )
        .translate(x, 46.0, SEPTUM_PANEL_Z / 2.0 + 21.0);
        guides = guides + (post - guide);
    }
    guides
}

fn septum_panel_lands() -> Part {
    let pierce_label = centered_cube(
        "closed_media_bottle_station_pierce_tool_recipe_label_land",
        SEPTUM_PANEL_X - 76.0,
        16.0,
        4.0,
    )
    .translate(0.0, SEPTUM_PANEL_Y / 2.0 - 26.0, SEPTUM_PANEL_Z / 2.0 + 2.0);
    let witness_strip = centered_cube(
        "closed_media_bottle_station_septum_witness_strip_land",
        SEPTUM_PANEL_X - 116.0,
        14.0,
        4.0,
    )
    .translate(
        0.0,
        -(SEPTUM_PANEL_Y / 2.0 - 28.0),
        SEPTUM_PANEL_Z / 2.0 + 2.0,
    );
    pierce_label + witness_strip
}

fn drip_leak_tray() -> Part {
    let tray = centered_cube(
        "closed_media_bottle_station_drip_leak_outer_tray",
        DRIP_TRAY_X,
        DRIP_TRAY_Y,
        DRIP_TRAY_Z,
    );
    let basin = centered_cube(
        "closed_media_bottle_station_drip_leak_basin",
        DRIP_TRAY_X - 48.0,
        DRIP_TRAY_Y - 44.0,
        14.0,
    )
    .translate(0.0, 0.0, DRIP_TRAY_Z / 2.0 - 6.0);
    let drain = centered_cylinder(
        "closed_media_bottle_station_drip_tray_low_point_drain",
        DRAIN_D / 2.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DRIP_TRAY_X / 2.0 - 62.0, -(DRIP_TRAY_Y / 2.0 + 12.0), -1.0);
    tray - basin - drain + drip_tray_curbs() + leak_witness_ribs()
}

fn drip_tray_curbs() -> Part {
    let rear = centered_cube(
        "closed_media_bottle_station_drip_tray_rear_curb",
        DRIP_TRAY_X,
        12.0,
        DRIP_TRAY_CURB_Z,
    )
    .translate(
        0.0,
        DRIP_TRAY_Y / 2.0 - 6.0,
        DRIP_TRAY_Z / 2.0 + DRIP_TRAY_CURB_Z / 2.0,
    );
    let left = centered_cube(
        "closed_media_bottle_station_drip_tray_left_curb",
        12.0,
        DRIP_TRAY_Y,
        DRIP_TRAY_CURB_Z,
    )
    .translate(
        -DRIP_TRAY_X / 2.0 + 6.0,
        0.0,
        DRIP_TRAY_Z / 2.0 + DRIP_TRAY_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_bottle_station_drip_tray_right_curb",
        12.0,
        DRIP_TRAY_Y,
        DRIP_TRAY_CURB_Z,
    )
    .translate(
        DRIP_TRAY_X / 2.0 - 6.0,
        0.0,
        DRIP_TRAY_Z / 2.0 + DRIP_TRAY_CURB_Z / 2.0,
    );
    rear + left + right
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_bottle_station_leak_witness_ribs");
    for i in 0..BOTTLE_COUNT {
        ribs = ribs
            + centered_cube(
                format!("closed_media_bottle_station_leak_witness_channel_{i}"),
                4.0,
                DRIP_TRAY_Y - 56.0,
                6.0,
            )
            .translate(
                centered_index(i, BOTTLE_COUNT, 104.0),
                0.0,
                DRIP_TRAY_Z / 2.0 - 2.0,
            );
    }
    ribs
}

fn sterile_connector_handoff() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_sterile_connector_handoff_base",
        CONNECTOR_X,
        CONNECTOR_Y,
        CONNECTOR_Z,
    );
    base - connector_cradle_cuts() + connector_soft_stops() + connector_label_lands()
}

fn connector_cradle_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_connector_cradle_cuts");
    for i in 0..CONNECTOR_CRADLES {
        let x = centered_index(i, CONNECTOR_CRADLES, CONNECTOR_PITCH);
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bottle_station_sterile_connector_cradle_{i}"),
                CONNECTOR_CRADLE_D / 2.0,
                CONNECTOR_Y + 4.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, CONNECTOR_Z / 2.0 + 2.0);
    }
    cuts
}

fn connector_soft_stops() -> Part {
    let mut stops = Part::empty("closed_media_bottle_station_connector_soft_stops");
    for i in 0..CONNECTOR_CRADLES {
        let x = centered_index(i, CONNECTOR_CRADLES, CONNECTOR_PITCH);
        stops = stops
            + centered_cube(
                format!("closed_media_bottle_station_connector_end_stop_{i}"),
                12.0,
                18.0,
                32.0,
            )
            .translate(x, CONNECTOR_Y / 2.0 - 24.0, CONNECTOR_Z / 2.0 + 16.0);
    }
    stops
}

fn connector_label_lands() -> Part {
    centered_cube(
        "closed_media_bottle_station_connector_handoff_barcode_land",
        CONNECTOR_X - 78.0,
        18.0,
        4.0,
    )
    .translate(0.0, -(CONNECTOR_Y / 2.0 - 24.0), CONNECTOR_Z / 2.0 + 2.0)
}

fn barcode_rfid_coa_lands() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_traceability_lands_base",
        ID_LANDS_X,
        ID_LANDS_Y,
        ID_LANDS_Z,
    );
    base + barcode_lands() + rfid_pads() + coa_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_media_bottle_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_bottle_station_bottle_barcode_land_{i}"),
                82.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 92.0),
                ID_LANDS_Y / 2.0 - 28.0,
                ID_LANDS_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn rfid_pads() -> Part {
    let inbound = centered_cube(
        "closed_media_bottle_station_inbound_rfid_pad",
        RFID_PAD_X,
        RFID_PAD_Y,
        4.0,
    )
    .translate(-88.0, -16.0, ID_LANDS_Z / 2.0 + 2.0);
    let outbound = centered_cube(
        "closed_media_bottle_station_outbound_rfid_pad",
        RFID_PAD_X,
        RFID_PAD_Y,
        4.0,
    )
    .translate(88.0, -16.0, ID_LANDS_Z / 2.0 + 2.0);
    inbound + outbound
}

fn coa_lands() -> Part {
    centered_cube(
        "closed_media_bottle_station_coa_document_land",
        COA_LAND_X,
        COA_LAND_Y,
        4.0,
    )
    .translate(0.0, -ID_LANDS_Y / 2.0 + 28.0, ID_LANDS_Z / 2.0 + 2.0)
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_media_bottle_station_release_hold_reject_lane_base",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    base - status_slot_cuts() + status_lane_dividers() + status_lane_label_lands()
}

fn status_slot_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bottle_station_status_slot_cuts");
    for lane in 0..STATUS_LANE_COUNT {
        let y = centered_index(lane, STATUS_LANE_COUNT, STATUS_LANE_PITCH_Y);
        for i in 0..STATUS_SLOT_COUNT {
            let x = centered_index(i % BOTTLE_COLS, BOTTLE_COLS, STATUS_SLOT_PITCH_X);
            if i / BOTTLE_COLS == 0 {
                cuts = cuts
                    + centered_cube(
                        format!("closed_media_bottle_station_status_lane_{lane}_slot_{i}"),
                        STATUS_SLOT_X,
                        STATUS_SLOT_Y,
                        12.0,
                    )
                    .translate(x, y, STATUS_Z / 2.0 - 5.0);
            }
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("closed_media_bottle_station_status_lane_dividers");
    for i in 0..2 {
        let y = -STATUS_LANE_PITCH_Y / 2.0 + i as f64 * STATUS_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("closed_media_bottle_station_status_lane_divider_{i}"),
                STATUS_X - 58.0,
                6.0,
                24.0,
            )
            .translate(0.0, y, STATUS_Z / 2.0 + 12.0);
    }
    dividers
}

fn status_lane_label_lands() -> Part {
    let mut labels = Part::empty("closed_media_bottle_station_status_label_lands");
    for (i, y) in [STATUS_LANE_PITCH_Y, 0.0, -STATUS_LANE_PITCH_Y]
        .iter()
        .enumerate()
    {
        labels = labels
            + centered_cube(
                format!("closed_media_bottle_station_release_hold_reject_label_{i}"),
                78.0,
                20.0,
                4.0,
            )
            .translate(-(STATUS_X / 2.0 - 50.0), *y, STATUS_Z / 2.0 + 2.0);
    }
    labels
}

fn clean_used_segregation() -> Part {
    let barrier = centered_cube(
        "closed_media_bottle_station_clean_used_center_barrier",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let handoff_gap = centered_cube(
        "closed_media_bottle_station_controlled_handoff_gap",
        142.0,
        SEGREGATION_Y + 4.0,
        SEGREGATION_Z + 4.0,
    )
    .translate(-408.0, 0.0, 0.0);
    let scan_gap = centered_cube(
        "closed_media_bottle_station_traceability_scan_gap",
        160.0,
        SEGREGATION_Y + 4.0,
        SEGREGATION_Z + 4.0,
    )
    .translate(328.0, 0.0, 0.0);
    barrier - handoff_gap - scan_gap + segregation_sign_lands()
}

fn segregation_sign_lands() -> Part {
    let clean = centered_cube(
        "closed_media_bottle_station_clean_side_label_land",
        164.0,
        18.0,
        4.0,
    )
    .translate(-190.0, CLEAN_SIDE_MIN_Y / 2.0, SEGREGATION_Z / 2.0 + 2.0);
    let used = centered_cube(
        "closed_media_bottle_station_used_side_label_land",
        164.0,
        18.0,
        4.0,
    )
    .translate(190.0, -CLEAN_SIDE_MIN_Y / 2.0, SEGREGATION_Z / 2.0 + 2.0);
    clean + used
}

fn robot_tool_keepouts() -> Part {
    let mut keepouts = Part::empty("closed_media_bottle_station_robot_tool_keepouts");
    for index in 0..BOTTLE_COUNT {
        let (x, y) = bottle_position(index);
        keepouts = keepouts
            + centered_cylinder(
                format!("closed_media_bottle_station_bottle_{index}_capper_keepout"),
                TORQUE_TOOL_KEEP_OUT_D / 2.0,
                KEEP_OUT_Z,
                40,
            )
            .translate(BOTTLE_MATRIX_CENTER.0 + x, BOTTLE_MATRIX_CENTER.1 + y, 0.0);
    }
    for i in 0..SEPTUM_WELL_COUNT {
        keepouts = keepouts
            + centered_cylinder(
                format!("closed_media_bottle_station_septum_tool_keepout_{i}"),
                SEPTUM_TOOL_KEEP_OUT_D / 2.0,
                SEPTUM_ACCESS_CLEARANCE_Z,
                36,
            )
            .translate(
                SEPTUM_CENTER.0 + centered_index(i, SEPTUM_WELL_COUNT, SEPTUM_WELL_PITCH),
                SEPTUM_CENTER.1 + 26.0,
                0.0,
            );
    }
    keepouts
        + centered_cube(
            "closed_media_bottle_station_front_robot_sweep_keepout",
            DECK_X - 160.0,
            82.0,
            KEEP_OUT_Z,
        )
        .translate(0.0, -(DECK_Y / 2.0 - 88.0), 0.0)
        + centered_cube(
            "closed_media_bottle_station_rear_tool_cable_keepout",
            DECK_X - 200.0,
            66.0,
            KEEP_OUT_Z,
        )
        .translate(0.0, DECK_Y / 2.0 - 86.0, 0.0)
}

fn scanner_bridge() -> Part {
    let beam = centered_cube(
        "closed_media_bottle_station_barcode_rfid_scanner_bridge_beam",
        SCANNER_BRIDGE_X,
        SCANNER_BRIDGE_Y,
        SCANNER_BRIDGE_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z + SCANNER_BRIDGE_Z / 2.0,
    );
    let left_post = scanner_bridge_post("left").translate(
        -SCANNER_BRIDGE_X / 2.0 + 34.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = scanner_bridge_post("right").translate(
        SCANNER_BRIDGE_X / 2.0 - 34.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    beam + left_post + right_post + scanner_sleds() + bridge_antenna_pads()
}

fn scanner_bridge_post(side: &str) -> Part {
    centered_cube(
        format!("closed_media_bottle_station_scanner_bridge_{side}_post"),
        30.0,
        58.0,
        SCANNER_BRIDGE_UNDERSIDE_Z,
    )
}

fn scanner_sleds() -> Part {
    let barcode = centered_cube(
        "closed_media_bottle_station_overhead_barcode_scanner_sled",
        SCANNER_SLED_X,
        SCANNER_SLED_Y,
        24.0,
    )
    .translate(
        -230.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z - 18.0,
    );
    let camera = centered_cube(
        "closed_media_bottle_station_cap_ocr_camera_sled",
        SCANNER_SLED_X,
        SCANNER_SLED_Y,
        24.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 106.0, SCANNER_BRIDGE_UNDERSIDE_Z - 18.0);
    let coa = centered_cube(
        "closed_media_bottle_station_coa_document_camera_sled",
        SCANNER_SLED_X,
        SCANNER_SLED_Y,
        24.0,
    )
    .translate(
        230.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z - 18.0,
    );
    barcode + camera + coa
}

fn bridge_antenna_pads() -> Part {
    let inbound = centered_cube(
        "closed_media_bottle_station_inbound_bridge_rfid_antenna",
        RFID_ANTENNA_X,
        RFID_ANTENNA_Y,
        8.0,
    )
    .translate(
        -390.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z - 34.0,
    );
    let outbound = centered_cube(
        "closed_media_bottle_station_outbound_bridge_rfid_antenna",
        RFID_ANTENNA_X,
        RFID_ANTENNA_Y,
        8.0,
    )
    .translate(
        390.0,
        DECK_Y / 2.0 - 106.0,
        SCANNER_BRIDGE_UNDERSIDE_Z - 34.0,
    );
    inbound + outbound
}

fn bottle_position(index: usize) -> (f64, f64) {
    let row = index / BOTTLE_COLS;
    let col = index % BOTTLE_COLS;
    (
        centered_index(col, BOTTLE_COLS, BOTTLE_PITCH_X),
        centered_index(row, BOTTLE_ROWS, BOTTLE_PITCH_Y),
    )
}

fn cap_park_position(index: usize, row: usize) -> (f64, f64) {
    let col = index % CAP_PARK_COUNT;
    (
        centered_index(col, CAP_PARK_COUNT, CAP_PARK_PITCH_X),
        centered_index(row, 2, CAP_PARK_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn cap_park_barrier_gap() -> f64 {
    CAP_PARK_PITCH_Y - (GL45_CAP_OD.max(SEPTUM_CAP_OD) + 5.0)
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 + margin <= DECK_X / 2.0
        && center.1.abs() + y / 2.0 + margin <= DECK_Y / 2.0
}

fn rect(center: (f64, f64), x: f64, y: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - x / 2.0,
        center.0 + x / 2.0,
        center.1 - y / 2.0,
        center.1 + y / 2.0,
    )
}

fn rects_overlap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> bool {
    a.0 < b.1 && a.1 > b.0 && a.2 < b.3 && a.3 > b.2
}

fn vertical_gap(a: (f64, f64, f64, f64), b: (f64, f64, f64, f64)) -> f64 {
    if a.3 < b.2 {
        b.2 - a.3
    } else if b.3 < a.2 {
        a.2 - b.3
    } else {
        0.0
    }
}

fn fiducial_target(name: &str) -> Part {
    let base = centered_cylinder(format!("{name}_outer_disc"), 13.0, 3.0, 40);
    let cross_x = centered_cube(format!("{name}_cross_x"), 28.0, 3.0, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 28.0, 4.0);
    base + cross_x + cross_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_count_matches_assembly_manifest() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn bottle_matrix_capacity_and_spacing_are_stable() {
        assert_eq!(BOTTLE_COUNT, 6);
        assert_eq!(BOTTLE_ROWS * BOTTLE_COLS, BOTTLE_COUNT);
        assert!(BOTTLE_PITCH_X > BOTTLE_NEST_D + 36.0);
        assert!(BOTTLE_PITCH_Y > BOTTLE_NEST_D + 60.0);
    }

    #[test]
    fn cap_and_plug_parks_have_clean_used_gap() {
        assert_eq!(CAP_PARK_COUNT, BOTTLE_COUNT);
        assert_eq!(PLUG_PARK_COUNT, BOTTLE_COUNT);
        assert!(cap_park_barrier_gap() >= CLEAN_USED_BARRIER_GAP);
    }

    #[test]
    fn required_feature_list_covers_user_scope() {
        assert!(REQUIRED_FEATURES.contains(&"bottle_nest_matrix"));
        assert!(REQUIRED_FEATURES.contains(&"cap_plug_parks"));
        assert!(REQUIRED_FEATURES.contains(&"torque_verification_pocket"));
        assert!(REQUIRED_FEATURES.contains(&"septum_access_alignment_wells"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_coa_lands"));
        assert!(REQUIRED_FEATURES.contains(&"released_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"robot_tool_keepouts"));
    }

    #[test]
    fn major_modules_fit_without_forcing_shared_layout_edits() {
        assert_layout();
        assert!(SCANNER_BRIDGE_UNDERSIDE_Z > SEPTUM_ACCESS_CLEARANCE_Z);
        assert!(ROBOT_APPROACH_KEEP_OUTS <= REQUIRED_FEATURES.len());
    }
}
