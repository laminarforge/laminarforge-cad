use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette waste-port misroute colorimetric witness station.
//
// Standalone validation CAD for checking that an automated workcell routes a
// closed cassette waste port to the intended waste/retain/reject path before
// any live biology is connected. The model packages keyed cassette waste-port
// docks, an automated routing manifold, deliberate misroute challenge tokens,
// colorimetric witness coupon slots, waste/retain split capture, leak/drip
// containment, barcode/COA custody, release/hold/reject disposition gates,
// camera evidence support, and robot/service keepout gauges. It is mechanical
// validation packaging only; chemistry, acceptance criteria, and batch release
// rules live outside this CAD artifact.

const OUTPUT_PREFIX: &str = "closed_cassette_waste_port_misroute_colorimetric_witness_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_base_containment_deck.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_keyed_cassette_waste_port_dock.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_automated_route_selector_manifold.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_misroute_challenge_token_plate.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_colorimetric_witness_coupon_array.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_waste_retain_split_capture.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_leak_drip_containment_tray.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_barcode_coa_custody_panel.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_hold_release_reject_gate_lanes.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_camera_evidence_bridge.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_closed_custody_retain_archive.stl",
    "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "keyed_cassette_waste_port_dock",
    "automated_route_selector_manifold",
    "misroute_challenge_token_plate",
    "colorimetric_witness_coupon_array",
    "waste_retain_split_capture",
    "leak_drip_containment_tray",
    "barcode_coa_custody_panel",
    "hold_release_reject_gate_lanes",
    "camera_evidence_bridge",
    "robot_service_keepout_gauges",
    "closed_custody_retain_archive",
    "scoped_named_stl_outputs",
];

const ROUTE_STATES: [&str; 4] = ["waste", "retain", "reject", "witness"];
const MISROUTE_CHALLENGES: [&str; 5] = [
    "crossed_waste_retain",
    "wrong_cassette_id",
    "missing_coa",
    "reverse_flow",
    "unlatched_port",
];
const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];
const CUSTODY_STATES: [&str; 4] = ["scan_in", "route_lock", "evidence", "coa_lock"];

const DECK_X: f64 = 1660.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const CURB_W: f64 = 18.0;
const CURB_Z: f64 = 48.0;
const SOCKET_Z: f64 = 7.0;
const BASIN_DEPTH: f64 = 9.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MOUNT_HOLES: usize = 8;
const DECK_DATUMS: usize = 6;

const DOCK_POS: (f64, f64) = (-470.0, 215.0);
const DOCK_X: f64 = 570.0;
const DOCK_Y: f64 = 320.0;
const DOCK_Z: f64 = 56.0;
const CASSETTE_ROWS: usize = 2;
const CASSETTE_COLS: usize = 3;
const CASSETTE_SLOTS: usize = CASSETTE_ROWS * CASSETTE_COLS;
const CASSETTE_NEST_X: f64 = 148.0;
const CASSETTE_NEST_Y: f64 = 92.0;
const CASSETTE_NEST_Z: f64 = 18.0;
const CASSETTE_PITCH_X: f64 = 172.0;
const CASSETTE_PITCH_Y: f64 = 118.0;
const WASTE_PORT_D: f64 = 24.0;
const KEYING_RIBS_PER_SLOT: usize = 3;

const ROUTE_POS: (f64, f64) = (250.0, 235.0);
const ROUTE_X: f64 = 610.0;
const ROUTE_Y: f64 = 280.0;
const ROUTE_Z: f64 = 64.0;
const ROUTE_PATHS: usize = CASSETTE_SLOTS;
const VALVES_PER_PATH: usize = ROUTE_STATES.len();
const VALVE_POCKETS: usize = ROUTE_PATHS * VALVES_PER_PATH;
const ROUTE_PITCH_X: f64 = 88.0;
const ROUTE_CHANNEL_W: f64 = 18.0;
const ROUTE_CHANNEL_DEPTH: f64 = 9.0;
const RFID_READBACK_LANDS: usize = ROUTE_PATHS;

const CHALLENGE_POS: (f64, f64) = (-620.0, -130.0);
const CHALLENGE_X: f64 = 310.0;
const CHALLENGE_Y: f64 = 270.0;
const CHALLENGE_Z: f64 = 38.0;
const TOKEN_COLS: usize = MISROUTE_CHALLENGES.len();
const TOKEN_ROWS: usize = 2;
const CHALLENGE_TOKENS: usize = TOKEN_COLS * TOKEN_ROWS;
const TOKEN_X: f64 = 42.0;
const TOKEN_Y: f64 = 54.0;
const TOKEN_Z: f64 = 14.0;
const TOKEN_PITCH_X: f64 = 52.0;
const TOKEN_PITCH_Y: f64 = 82.0;

const WITNESS_POS: (f64, f64) = (-225.0, -155.0);
const WITNESS_X: f64 = 430.0;
const WITNESS_Y: f64 = 240.0;
const WITNESS_Z: f64 = 42.0;
const WITNESS_ROWS: usize = 3;
const WITNESS_COLS: usize = 6;
const WITNESS_COUPONS: usize = WITNESS_ROWS * WITNESS_COLS;
const WITNESS_SLOT_X: f64 = 44.0;
const WITNESS_SLOT_Y: f64 = 24.0;
const WITNESS_PITCH_X: f64 = 60.0;
const WITNESS_PITCH_Y: f64 = 58.0;
const COLOR_REFERENCE_TILES: usize = 6;
const COUPON_CHALLENGE_VOLUME_UL: f64 = 90.0;

const SPLIT_POS: (f64, f64) = (230.0, -165.0);
const SPLIT_X: f64 = 410.0;
const SPLIT_Y: f64 = 240.0;
const SPLIT_Z: f64 = 52.0;
const SPLIT_BRANCHES: usize = 2;
const WASTE_CAPTURE_BAYS: usize = 3;
const RETAIN_CAPTURE_BAYS: usize = 3;
const CAPTURE_BAG_X: f64 = 118.0;
const CAPTURE_BAG_Y: f64 = 72.0;
const CAPTURE_PITCH_X: f64 = 132.0;
const CAPTURE_PITCH_Y: f64 = 86.0;
const RETAIN_VOLUME_ML: f64 = 450.0;
const WASTE_VOLUME_ML: f64 = 900.0;

const DRIP_POS: (f64, f64) = (620.0, -120.0);
const DRIP_X: f64 = 300.0;
const DRIP_Y: f64 = 270.0;
const DRIP_Z: f64 = 36.0;
const DRIP_CHANNELS: usize = ROUTE_PATHS;
const DRIP_CUP_COUNT: usize = 8;
const DRIP_CHANNEL_W: f64 = 16.0;
const DRIP_CUP_D: f64 = 26.0;

const CUSTODY_POS: (f64, f64) = (-520.0, -379.0);
const CUSTODY_X: f64 = 410.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 24.0;
const BARCODE_LANDS: usize = CASSETTE_SLOTS + ROUTE_STATES.len();
const COA_CLIPS: usize = 6;
const CUSTODY_SEAL_SLOTS: usize = CUSTODY_STATES.len();

const GATE_POS: (f64, f64) = (-40.0, -379.0);
const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 150.0;
const GATE_Z: f64 = 32.0;
const GATE_LANES: usize = DISPOSITION_LANES.len();
const GATE_TOKENS_PER_LANE: usize = 5;
const GATE_TOKEN_SLOTS: usize = GATE_LANES * GATE_TOKENS_PER_LANE;

const ARCHIVE_POS: (f64, f64) = (440.0, -379.0);
const ARCHIVE_X: f64 = 430.0;
const ARCHIVE_Y: f64 = 150.0;
const ARCHIVE_Z: f64 = 34.0;
const RETAIN_ARCHIVE_VIALS: usize = ROUTE_PATHS;
const RETAIN_SEAL_WINDOWS: usize = ROUTE_PATHS;

const BRIDGE_X: f64 = 1390.0;
const BRIDGE_Y: f64 = 72.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 54.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_UNDERSIDE_Z: f64 = 252.0;
const CAMERA_PODS: usize = 6;
const LIGHT_BARS: usize = 4;
const EVIDENCE_FIDUCIALS: usize = 8;

const KEEP_OUT_RAIL_W: f64 = 12.0;
const KEEP_OUT_Z: f64 = 96.0;
const ROBOT_FRONT_CLEARANCE: f64 = 380.0;
const SERVICE_REAR_CLEARANCE: f64 = 245.0;
const LEFT_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 235.0;
const ROBOT_Z_CLEARANCE: f64 = 340.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self, margin: f64) -> bool {
        self.center.0.abs() + self.x / 2.0 <= DECK_X / 2.0 - margin
            && self.center.1.abs() + self.y / 2.0 <= DECK_Y / 2.0 - margin
    }

    fn overlaps(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let dock = keyed_cassette_waste_port_dock();
    export(OUTPUTS[1], &dock);

    let route = automated_route_selector_manifold();
    export(OUTPUTS[2], &route);

    let challenge = misroute_challenge_token_plate();
    export(OUTPUTS[3], &challenge);

    let witnesses = colorimetric_witness_coupon_array();
    export(OUTPUTS[4], &witnesses);

    let split = waste_retain_split_capture();
    export(OUTPUTS[5], &split);

    let drip = leak_drip_containment_tray();
    export(OUTPUTS[6], &drip);

    let custody = barcode_coa_custody_panel();
    export(OUTPUTS[7], &custody);

    let gates = hold_release_reject_gate_lanes();
    export(OUTPUTS[8], &gates);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let archive = closed_custody_retain_archive();
    export(OUTPUTS[11], &archive);

    let assembly = deck
        + dock.translate(DOCK_POS.0, DOCK_POS.1, seated_z(DOCK_Z))
        + route.translate(ROUTE_POS.0, ROUTE_POS.1, seated_z(ROUTE_Z))
        + challenge.translate(CHALLENGE_POS.0, CHALLENGE_POS.1, seated_z(CHALLENGE_Z))
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, seated_z(WITNESS_Z))
        + split.translate(SPLIT_POS.0, SPLIT_POS.1, seated_z(SPLIT_Z))
        + drip.translate(DRIP_POS.0, DRIP_POS.1, seated_z(DRIP_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, seated_z(CUSTODY_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, seated_z(GATE_Z))
        + archive.translate(ARCHIVE_POS.0, ARCHIVE_POS.1, seated_z(ARCHIVE_Z))
        + bridge.translate(0.0, 0.0, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!(
        "{OUTPUT_PREFIX}: {:.0}mm x {:.0}mm closed cassette waste-port routing validation station; {CASSETTE_SLOTS} cassette docks, {ROUTE_PATHS} automated route paths, {CHALLENGE_TOKENS} misroute tokens, {WITNESS_COUPONS} colorimetric witness coupons, waste/retain split capture, barcode/COA custody, release/hold/reject gates, camera evidence bridge, and robot/service keepouts.",
        DECK_X, DECK_Y
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn seated_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_Z / 2.0
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 8] {
    [
        rect("keyed_cassette_waste_port_dock", DOCK_POS, DOCK_X, DOCK_Y),
        rect(
            "automated_route_selector_manifold",
            ROUTE_POS,
            ROUTE_X,
            ROUTE_Y,
        ),
        rect(
            "misroute_challenge_token_plate",
            CHALLENGE_POS,
            CHALLENGE_X,
            CHALLENGE_Y,
        ),
        rect(
            "colorimetric_witness_coupon_array",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect("waste_retain_split_capture", SPLIT_POS, SPLIT_X, SPLIT_Y),
        rect("leak_drip_containment_tray", DRIP_POS, DRIP_X, DRIP_Y),
        rect(
            "barcode_coa_custody_panel",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect("hold_release_reject_gate_lanes", GATE_POS, GATE_X, GATE_Y),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(CASSETTE_SLOTS, CASSETTE_ROWS * CASSETTE_COLS);
    assert_eq!(ROUTE_PATHS, CASSETTE_SLOTS);
    assert_eq!(VALVE_POCKETS, ROUTE_PATHS * VALVES_PER_PATH);
    assert_eq!(RFID_READBACK_LANDS, ROUTE_PATHS);
    assert_eq!(CHALLENGE_TOKENS, TOKEN_ROWS * TOKEN_COLS);
    assert_eq!(WITNESS_COUPONS, WITNESS_ROWS * WITNESS_COLS);
    assert!(WITNESS_COUPONS >= ROUTE_PATHS * 3);
    assert_eq!(SPLIT_BRANCHES, 2);
    assert_eq!(WASTE_CAPTURE_BAYS, RETAIN_CAPTURE_BAYS);
    assert!(WASTE_VOLUME_ML > RETAIN_VOLUME_ML);
    assert_eq!(DRIP_CHANNELS, ROUTE_PATHS);
    assert!(containment_freeboard_ml() > maximum_challenge_liquid_ml());
    assert_eq!(BARCODE_LANDS, CASSETTE_SLOTS + ROUTE_STATES.len());
    assert_eq!(COA_CLIPS, CASSETTE_SLOTS);
    assert_eq!(CUSTODY_SEAL_SLOTS, CUSTODY_STATES.len());
    assert_eq!(GATE_LANES, DISPOSITION_LANES.len());
    assert_eq!(GATE_TOKEN_SLOTS, GATE_LANES * GATE_TOKENS_PER_LANE);
    assert_eq!(RETAIN_ARCHIVE_VIALS, ROUTE_PATHS);
    assert_eq!(RETAIN_SEAL_WINDOWS, ROUTE_PATHS);
    assert!(BRIDGE_UNDERSIDE_Z > DECK_Z + ROUTE_Z + 120.0);
    assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
    assert!(ROBOT_FRONT_CLEARANCE >= 360.0);
    assert!(SERVICE_REAR_CLEARANCE >= 220.0);
    assert!(LEFT_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_SERVICE_CLEARANCE >= 220.0);
    assert_eq!(mount_hole_positions().len(), MOUNT_HOLES);
    assert_eq!(deck_datum_positions().len(), DECK_DATUMS);

    let rects = module_rects();
    for item in rects {
        assert!(item.fits_inside_deck(36.0), "{} exceeds deck", item.name);
    }
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b], 18.0),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = DECK_X - 2.0 * CURB_W;
    let inner_y = DECK_Y - 2.0 * CURB_W;
    let freeboard_z = CURB_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn maximum_challenge_liquid_ml() -> f64 {
    let witness_load = WITNESS_COUPONS as f64 * COUPON_CHALLENGE_VOLUME_UL / 1000.0;
    let route_holdup = ROUTE_PATHS as f64 * 18.0;
    let drip_cups = DRIP_CUP_COUNT as f64 * 22.0;
    witness_load + route_holdup + drip_cups
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "waste_port_misroute_base_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "waste_port_misroute_recessed_secondary_basin_cut",
        DECK_X - 2.0 * (CURB_W + 46.0),
        DECK_Y - 2.0 * (CURB_W + 48.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -10.0, DECK_Z - BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        "waste_port_misroute_closed_drain_placeholder",
        10.0,
        62.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 8.0);

    deck - basin - drain - module_sockets() - deck_mount_holes()
        + perimeter_curbs()
        + workflow_route_spines()
        + deck_datum_targets()
        + evidence_anchor_lands()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("waste_port_misroute_module_sockets");
    for item in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("waste_port_misroute_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_Z + 0.5,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_Z / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("waste_port_misroute_deck_mount_holes");
    for (i, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("waste_port_misroute_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("waste_port_misroute_mount_washer_flat_{i}"),
                28.0,
                8.0,
                DECK_Z + 6.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
}

fn perimeter_curbs() -> Part {
    let front = centered_cube(
        "waste_port_misroute_front_low_robot_curb",
        DECK_X,
        CURB_W,
        30.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + CURB_W / 2.0, DECK_Z + 15.0);
    let rear = centered_cube(
        "waste_port_misroute_rear_service_curb",
        DECK_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - CURB_W / 2.0, DECK_Z + CURB_Z / 2.0);
    let left = centered_cube(
        "waste_port_misroute_left_spill_curb",
        CURB_W,
        DECK_Y,
        CURB_Z,
    )
    .translate(-DECK_X / 2.0 + CURB_W / 2.0, 0.0, DECK_Z + CURB_Z / 2.0);
    let right = centered_cube(
        "waste_port_misroute_right_spill_curb",
        CURB_W,
        DECK_Y,
        CURB_Z,
    )
    .translate(DECK_X / 2.0 - CURB_W / 2.0, 0.0, DECK_Z + CURB_Z / 2.0);

    front + rear + left + right
}

fn workflow_route_spines() -> Part {
    let cassette_to_route = centered_cube(
        "waste_port_misroute_cassette_to_route_spine",
        1040.0,
        11.0,
        24.0,
    )
    .translate(-110.0, 64.0, DECK_Z + 12.0);
    let challenge_to_witness = centered_cube(
        "waste_port_misroute_challenge_to_witness_spine",
        650.0,
        10.0,
        22.0,
    )
    .translate(-410.0, -292.0, DECK_Z + 11.0);
    let witness_to_split = centered_cube(
        "waste_port_misroute_witness_to_split_spine",
        650.0,
        10.0,
        22.0,
    )
    .translate(20.0, -292.0, DECK_Z + 11.0);
    let split_to_archive = centered_cube(
        "waste_port_misroute_split_to_archive_spine",
        890.0,
        10.0,
        22.0,
    )
    .translate(150.0, -318.0, DECK_Z + 11.0);

    cassette_to_route + challenge_to_witness + witness_to_split + split_to_archive
}

fn deck_datum_targets() -> Part {
    let mut datums = Part::empty("waste_port_misroute_deck_datum_targets");
    for (i, (x, y)) in deck_datum_positions().into_iter().enumerate() {
        datums = datums
            + fiducial_disc(&format!("waste_port_misroute_deck_datum_{i}")).translate(
                x,
                y,
                DECK_Z + 3.0,
            );
    }
    datums
}

fn deck_datum_positions() -> [(f64, f64); DECK_DATUMS] {
    [
        (-DECK_X / 2.0 + 110.0, -DECK_Y / 2.0 + 100.0),
        (DECK_X / 2.0 - 110.0, -DECK_Y / 2.0 + 100.0),
        (-DECK_X / 2.0 + 110.0, DECK_Y / 2.0 - 100.0),
        (DECK_X / 2.0 - 110.0, DECK_Y / 2.0 - 100.0),
        (-90.0, 385.0),
        (90.0, 385.0),
    ]
}

fn evidence_anchor_lands() -> Part {
    let left = centered_cube(
        "waste_port_misroute_left_evidence_bridge_anchor_land",
        105.0,
        50.0,
        9.0,
    )
    .translate(-BRIDGE_X / 2.0, 360.0, DECK_Z + 4.5);
    let right = centered_cube(
        "waste_port_misroute_right_evidence_bridge_anchor_land",
        105.0,
        50.0,
        9.0,
    )
    .translate(BRIDGE_X / 2.0, 360.0, DECK_Z + 4.5);

    left + right
}

fn keyed_cassette_waste_port_dock() -> Part {
    let base = centered_cube(
        "waste_port_misroute_keyed_cassette_waste_port_dock_base",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let label_ledge = centered_cube(
        "waste_port_misroute_cassette_scan_ledge",
        DOCK_X - 54.0,
        26.0,
        10.0,
    )
    .translate(0.0, DOCK_Y / 2.0 - 36.0, DOCK_Z / 2.0 + 5.0);

    base - cassette_nest_cuts() - waste_port_clearances() - keyed_rib_reliefs()
        + cassette_gasket_frames()
        + port_collar_guard_rings()
        + dock_lock_lugs()
        + label_ledge
        + dock_route_stubs()
}

fn cassette_nest_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_cassette_nest_cuts");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let x = centered_index(col, CASSETTE_COLS, CASSETTE_PITCH_X);
            let y = centered_index(row, CASSETTE_ROWS, CASSETTE_PITCH_Y);
            cuts = cuts
                + centered_cube(
                    format!("waste_port_misroute_cassette_nest_cut_r{row}_c{col}"),
                    CASSETTE_NEST_X,
                    CASSETTE_NEST_Y,
                    CASSETTE_NEST_Z + 1.0,
                )
                .translate(x, y, DOCK_Z / 2.0 - CASSETTE_NEST_Z / 2.0);
        }
    }
    cuts
}

fn waste_port_clearances() -> Part {
    let mut ports = Part::empty("waste_port_misroute_waste_port_clearances");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        ports = ports
            + centered_cylinder(
                format!("waste_port_misroute_closed_waste_port_clearance_{slot}"),
                WASTE_PORT_D / 2.0,
                DOCK_Z + 3.0,
                36,
            )
            .translate(x + 40.0, y - 16.0, 0.0);
    }
    ports
}

fn keyed_rib_reliefs() -> Part {
    let mut reliefs = Part::empty("waste_port_misroute_keyed_rib_reliefs");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        for rib in 0..KEYING_RIBS_PER_SLOT {
            reliefs = reliefs
                + centered_cube(
                    format!("waste_port_misroute_key_relief_slot_{slot}_{rib}"),
                    8.0,
                    34.0,
                    DOCK_Z + 2.0,
                )
                .translate(x - 46.0 + rib as f64 * 16.0, y + 28.0, 0.0);
        }
    }
    reliefs
}

fn cassette_gasket_frames() -> Part {
    let mut frames = Part::empty("waste_port_misroute_cassette_gasket_frames");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        frames = frames
            + frame_rect(
                &format!("waste_port_misroute_cassette_gasket_frame_{slot}"),
                CASSETTE_NEST_X + 18.0,
                CASSETTE_NEST_Y + 18.0,
                8.0,
                7.0,
            )
            .translate(x, y, DOCK_Z / 2.0 + 3.5);
    }
    frames
}

fn port_collar_guard_rings() -> Part {
    let mut rings = Part::empty("waste_port_misroute_port_collar_guard_rings");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        rings = rings
            + donut(
                &format!("waste_port_misroute_waste_port_guard_ring_{slot}"),
                WASTE_PORT_D / 2.0 + 12.0,
                WASTE_PORT_D / 2.0 + 3.0,
                12.0,
            )
            .translate(x + 40.0, y - 16.0, DOCK_Z / 2.0 + 6.0);
    }
    rings
}

fn dock_lock_lugs() -> Part {
    let mut lugs = Part::empty("waste_port_misroute_dock_lock_lugs");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        lugs = lugs
            + centered_cube(
                format!("waste_port_misroute_left_lock_lug_{slot}"),
                16.0,
                34.0,
                22.0,
            )
            .translate(x - 72.0, y, DOCK_Z / 2.0 + 11.0)
            + centered_cube(
                format!("waste_port_misroute_right_lock_lug_{slot}"),
                16.0,
                34.0,
                22.0,
            )
            .translate(x + 72.0, y, DOCK_Z / 2.0 + 11.0);
    }
    lugs
}

fn dock_route_stubs() -> Part {
    let mut stubs = Part::empty("waste_port_misroute_dock_route_stubs");
    for slot in 0..CASSETTE_SLOTS {
        let (x, y) = cassette_slot_xy(slot);
        stubs = stubs
            + centered_cube(
                format!("waste_port_misroute_closed_tube_stub_to_manifold_{slot}"),
                82.0,
                11.0,
                10.0,
            )
            .translate(x + 96.0, y - 16.0, DOCK_Z / 2.0 + 5.0);
    }
    stubs
}

fn cassette_slot_xy(slot: usize) -> (f64, f64) {
    let row = slot / CASSETTE_COLS;
    let col = slot % CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, CASSETTE_PITCH_X),
        centered_index(row, CASSETTE_ROWS, CASSETTE_PITCH_Y),
    )
}

fn automated_route_selector_manifold() -> Part {
    let block = centered_cube(
        "waste_port_misroute_automated_route_selector_manifold_block",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let cover = centered_cube(
        "waste_port_misroute_clear_route_cover_surrogate",
        ROUTE_X - 46.0,
        ROUTE_Y - 44.0,
        8.0,
    )
    .translate(0.0, 0.0, ROUTE_Z / 2.0 + 4.0);

    block - route_channel_cuts() - valve_pocket_cuts() - readback_tag_recesses()
        + cover
        + valve_actuator_bosses()
        + outlet_route_horns()
        + route_state_index_comb()
}

fn route_channel_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_route_channel_cuts");
    for path in 0..ROUTE_PATHS {
        let x = centered_index(path, ROUTE_PATHS, ROUTE_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("waste_port_misroute_primary_route_channel_cut_{path}"),
                ROUTE_CHANNEL_W,
                ROUTE_Y - 42.0,
                ROUTE_CHANNEL_DEPTH,
            )
            .translate(x, 0.0, ROUTE_Z / 2.0 - ROUTE_CHANNEL_DEPTH / 2.0)
            + centered_cube(
                format!("waste_port_misroute_cross_route_sentinel_cut_{path}"),
                ROUTE_CHANNEL_W + 14.0,
                18.0,
                ROUTE_CHANNEL_DEPTH + 1.0,
            )
            .translate(x, -ROUTE_Y / 2.0 + 42.0, ROUTE_Z / 2.0 - 6.0);
    }
    cuts
}

fn valve_pocket_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_valve_pocket_cuts");
    for path in 0..ROUTE_PATHS {
        let x = centered_index(path, ROUTE_PATHS, ROUTE_PITCH_X);
        for state in 0..VALVES_PER_PATH {
            let y = centered_index(state, VALVES_PER_PATH, 54.0);
            cuts = cuts
                + centered_cylinder(
                    format!("waste_port_misroute_valve_pocket_path_{path}_state_{state}"),
                    13.0,
                    ROUTE_Z + 2.0,
                    32,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn readback_tag_recesses() -> Part {
    let mut tags = Part::empty("waste_port_misroute_readback_tag_recesses");
    for path in 0..RFID_READBACK_LANDS {
        let x = centered_index(path, RFID_READBACK_LANDS, ROUTE_PITCH_X);
        tags = tags
            + centered_cube(
                format!("waste_port_misroute_rfid_readback_recess_{path}"),
                34.0,
                20.0,
                6.0,
            )
            .translate(x, ROUTE_Y / 2.0 - 30.0, ROUTE_Z / 2.0 - 3.0);
    }
    tags
}

fn valve_actuator_bosses() -> Part {
    let mut bosses = Part::empty("waste_port_misroute_valve_actuator_bosses");
    for path in 0..ROUTE_PATHS {
        let x = centered_index(path, ROUTE_PATHS, ROUTE_PITCH_X);
        bosses = bosses
            + centered_cylinder(
                format!("waste_port_misroute_robot_actuator_center_boss_{path}"),
                19.0,
                16.0,
                36,
            )
            .translate(x, 0.0, ROUTE_Z / 2.0 + 8.0)
            + centered_cube(
                format!("waste_port_misroute_actuator_key_flat_{path}"),
                34.0,
                8.0,
                12.0,
            )
            .translate(x, 0.0, ROUTE_Z / 2.0 + 20.0);
    }
    bosses
}

fn outlet_route_horns() -> Part {
    let mut horns = Part::empty("waste_port_misroute_outlet_route_horns");
    for state in 0..ROUTE_STATES.len() {
        let y = centered_index(state, ROUTE_STATES.len(), 54.0);
        horns = horns
            + centered_cube(
                format!("waste_port_misroute_{}_outlet_horn", ROUTE_STATES[state]),
                86.0,
                16.0,
                18.0,
            )
            .translate(ROUTE_X / 2.0 - 46.0, y, ROUTE_Z / 2.0 + 9.0);
    }
    horns
}

fn route_state_index_comb() -> Part {
    let mut comb = Part::empty("waste_port_misroute_route_state_index_comb");
    for state in 0..ROUTE_STATES.len() {
        let y = centered_index(state, ROUTE_STATES.len(), 54.0);
        comb = comb
            + centered_cube(
                format!("waste_port_misroute_route_state_index_tooth_{state}"),
                10.0 + state as f64 * 4.0,
                18.0,
                20.0,
            )
            .translate(-ROUTE_X / 2.0 + 28.0, y, ROUTE_Z / 2.0 + 10.0);
    }
    comb
}

fn misroute_challenge_token_plate() -> Part {
    let base = centered_cube(
        "waste_port_misroute_challenge_token_plate_base",
        CHALLENGE_X,
        CHALLENGE_Y,
        CHALLENGE_Z,
    );
    let header = centered_cube(
        "waste_port_misroute_challenge_header_scan_strip",
        CHALLENGE_X - 38.0,
        24.0,
        10.0,
    )
    .translate(0.0, CHALLENGE_Y / 2.0 - 34.0, CHALLENGE_Z / 2.0 + 5.0);

    base - challenge_token_pockets() - challenge_pin_holes()
        + challenge_tokens()
        + challenge_key_profiles()
        + header
}

fn challenge_token_pockets() -> Part {
    let mut pockets = Part::empty("waste_port_misroute_challenge_token_pockets");
    for row in 0..TOKEN_ROWS {
        for col in 0..TOKEN_COLS {
            pockets = pockets
                + centered_cube(
                    format!("waste_port_misroute_challenge_token_pocket_r{row}_c{col}"),
                    TOKEN_X + 7.0,
                    TOKEN_Y + 7.0,
                    TOKEN_Z + 1.0,
                )
                .translate(
                    centered_index(col, TOKEN_COLS, TOKEN_PITCH_X),
                    centered_index(row, TOKEN_ROWS, TOKEN_PITCH_Y) - 16.0,
                    CHALLENGE_Z / 2.0 - TOKEN_Z / 2.0,
                );
        }
    }
    pockets
}

fn challenge_pin_holes() -> Part {
    let mut holes = Part::empty("waste_port_misroute_challenge_pin_holes");
    for col in 0..TOKEN_COLS {
        holes = holes
            + centered_cylinder(
                format!("waste_port_misroute_challenge_pin_clearance_{col}"),
                4.0,
                CHALLENGE_Z + 2.0,
                24,
            )
            .translate(
                centered_index(col, TOKEN_COLS, TOKEN_PITCH_X),
                -CHALLENGE_Y / 2.0 + 28.0,
                0.0,
            );
    }
    holes
}

fn challenge_tokens() -> Part {
    let mut tokens = Part::empty("waste_port_misroute_challenge_tokens");
    for row in 0..TOKEN_ROWS {
        for col in 0..TOKEN_COLS {
            let token = row * TOKEN_COLS + col;
            tokens = tokens
                + centered_cube(
                    format!(
                        "waste_port_misroute_{}_challenge_token_{token}",
                        MISROUTE_CHALLENGES[col]
                    ),
                    TOKEN_X,
                    TOKEN_Y,
                    TOKEN_Z,
                )
                .translate(
                    centered_index(col, TOKEN_COLS, TOKEN_PITCH_X),
                    centered_index(row, TOKEN_ROWS, TOKEN_PITCH_Y) - 16.0,
                    CHALLENGE_Z / 2.0 + TOKEN_Z / 2.0 + 3.0,
                );
        }
    }
    tokens
}

fn challenge_key_profiles() -> Part {
    let mut keys = Part::empty("waste_port_misroute_challenge_key_profiles");
    for col in 0..TOKEN_COLS {
        keys = keys
            + centered_cube(
                format!("waste_port_misroute_challenge_key_profile_{col}"),
                14.0 + col as f64 * 3.0,
                7.0,
                10.0,
            )
            .translate(
                centered_index(col, TOKEN_COLS, TOKEN_PITCH_X),
                CHALLENGE_Y / 2.0 - 64.0,
                CHALLENGE_Z / 2.0 + 5.0,
            );
    }
    keys
}

fn colorimetric_witness_coupon_array() -> Part {
    let base = centered_cube(
        "waste_port_misroute_colorimetric_witness_coupon_array_base",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let shield = centered_cube(
        "waste_port_misroute_witness_camera_shadow_shield",
        WITNESS_X - 42.0,
        18.0,
        28.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 24.0, WITNESS_Z / 2.0 + 14.0);

    base - witness_coupon_slots() - witness_drain_reliefs()
        + witness_coupon_retainer_lips()
        + color_reference_tile_ladder()
        + witness_fiducials()
        + shield
}

fn witness_coupon_slots() -> Part {
    let mut slots = Part::empty("waste_port_misroute_witness_coupon_slots");
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            slots = slots
                + centered_cube(
                    format!("waste_port_misroute_witness_coupon_slot_r{row}_c{col}"),
                    WITNESS_SLOT_X,
                    WITNESS_SLOT_Y,
                    12.0,
                )
                .translate(
                    centered_index(col, WITNESS_COLS, WITNESS_PITCH_X),
                    centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y) - 10.0,
                    WITNESS_Z / 2.0 - 6.0,
                );
        }
    }
    slots
}

fn witness_drain_reliefs() -> Part {
    let mut drains = Part::empty("waste_port_misroute_witness_drain_reliefs");
    for row in 0..WITNESS_ROWS {
        drains = drains
            + centered_cube(
                format!("waste_port_misroute_witness_row_micro_drain_{row}"),
                WITNESS_X - 80.0,
                5.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y) - 35.0,
                WITNESS_Z / 2.0 - 5.0,
            );
    }
    drains
}

fn witness_coupon_retainer_lips() -> Part {
    let mut lips = Part::empty("waste_port_misroute_witness_coupon_retainer_lips");
    for row in 0..WITNESS_ROWS {
        for col in 0..WITNESS_COLS {
            let x = centered_index(col, WITNESS_COLS, WITNESS_PITCH_X);
            let y = centered_index(row, WITNESS_ROWS, WITNESS_PITCH_Y) - 10.0;
            lips =
                lips + centered_cube(
                    format!("waste_port_misroute_witness_left_retainer_lip_r{row}_c{col}"),
                    4.0,
                    WITNESS_SLOT_Y + 10.0,
                    8.0,
                )
                .translate(
                    x - WITNESS_SLOT_X / 2.0 - 5.0,
                    y,
                    WITNESS_Z / 2.0 + 4.0,
                ) + centered_cube(
                    format!("waste_port_misroute_witness_right_retainer_lip_r{row}_c{col}"),
                    4.0,
                    WITNESS_SLOT_Y + 10.0,
                    8.0,
                )
                .translate(
                    x + WITNESS_SLOT_X / 2.0 + 5.0,
                    y,
                    WITNESS_Z / 2.0 + 4.0,
                );
        }
    }
    lips
}

fn color_reference_tile_ladder() -> Part {
    let mut ladder = Part::empty("waste_port_misroute_color_reference_tile_ladder");
    for tile in 0..COLOR_REFERENCE_TILES {
        ladder = ladder
            + centered_cube(
                format!("waste_port_misroute_color_reference_tile_{tile}"),
                28.0,
                18.0,
                8.0,
            )
            .translate(
                -WITNESS_X / 2.0 + 32.0,
                centered_index(tile, COLOR_REFERENCE_TILES, 28.0),
                WITNESS_Z / 2.0 + 4.0,
            );
    }
    ladder
}

fn witness_fiducials() -> Part {
    let mut fiducials = Part::empty("waste_port_misroute_witness_fiducials");
    for (i, (x, y)) in [
        (-WITNESS_X / 2.0 + 34.0, -WITNESS_Y / 2.0 + 30.0),
        (WITNESS_X / 2.0 - 34.0, -WITNESS_Y / 2.0 + 30.0),
        (-WITNESS_X / 2.0 + 34.0, WITNESS_Y / 2.0 - 54.0),
        (WITNESS_X / 2.0 - 34.0, WITNESS_Y / 2.0 - 54.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("waste_port_misroute_witness_fiducial_{i}")).translate(
                x,
                y,
                WITNESS_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn waste_retain_split_capture() -> Part {
    let base = centered_cube(
        "waste_port_misroute_waste_retain_split_capture_base",
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    );
    let splitter = centered_cube(
        "waste_port_misroute_waste_retain_diverter_keel",
        34.0,
        SPLIT_Y - 52.0,
        28.0,
    )
    .translate(0.0, 0.0, SPLIT_Z / 2.0 + 14.0);

    base - capture_bag_nests() - split_branch_ports()
        + splitter
        + split_route_arrows()
        + capture_clamp_fingers()
        + split_balance_wells()
}

fn capture_bag_nests() -> Part {
    let mut nests = Part::empty("waste_port_misroute_capture_bag_nests");
    for row in 0..SPLIT_BRANCHES {
        for col in 0..WASTE_CAPTURE_BAYS {
            nests = nests
                + centered_cube(
                    format!("waste_port_misroute_capture_bag_nest_r{row}_c{col}"),
                    CAPTURE_BAG_X,
                    CAPTURE_BAG_Y,
                    16.0,
                )
                .translate(
                    centered_index(col, WASTE_CAPTURE_BAYS, CAPTURE_PITCH_X),
                    centered_index(row, SPLIT_BRANCHES, CAPTURE_PITCH_Y),
                    SPLIT_Z / 2.0 - 8.0,
                );
        }
    }
    nests
}

fn split_branch_ports() -> Part {
    let mut ports = Part::empty("waste_port_misroute_split_branch_ports");
    for branch in 0..SPLIT_BRANCHES {
        let y = centered_index(branch, SPLIT_BRANCHES, CAPTURE_PITCH_Y);
        ports = ports
            + centered_cylinder(
                format!("waste_port_misroute_split_branch_port_{branch}"),
                11.0,
                SPLIT_Z + 2.0,
                32,
            )
            .translate(-SPLIT_X / 2.0 + 44.0, y, 0.0);
    }
    ports
}

fn split_route_arrows() -> Part {
    let mut arrows = Part::empty("waste_port_misroute_split_route_arrows");
    for branch in 0..SPLIT_BRANCHES {
        let y = centered_index(branch, SPLIT_BRANCHES, CAPTURE_PITCH_Y);
        arrows = arrows
            + centered_cube(
                format!("waste_port_misroute_split_route_spine_{branch}"),
                SPLIT_X - 105.0,
                9.0,
                10.0,
            )
            .translate(24.0, y, SPLIT_Z / 2.0 + 5.0)
            + centered_cube(
                format!("waste_port_misroute_split_arrow_head_{branch}"),
                22.0,
                22.0,
                10.0,
            )
            .translate(SPLIT_X / 2.0 - 55.0, y, SPLIT_Z / 2.0 + 5.0);
    }
    arrows
}

fn capture_clamp_fingers() -> Part {
    let mut fingers = Part::empty("waste_port_misroute_capture_clamp_fingers");
    for row in 0..SPLIT_BRANCHES {
        for col in 0..WASTE_CAPTURE_BAYS {
            let x = centered_index(col, WASTE_CAPTURE_BAYS, CAPTURE_PITCH_X);
            let y = centered_index(row, SPLIT_BRANCHES, CAPTURE_PITCH_Y);
            fingers = fingers
                + centered_cube(
                    format!("waste_port_misroute_capture_left_clamp_r{row}_c{col}"),
                    8.0,
                    CAPTURE_BAG_Y + 18.0,
                    18.0,
                )
                .translate(x - CAPTURE_BAG_X / 2.0 - 8.0, y, SPLIT_Z / 2.0 + 9.0)
                + centered_cube(
                    format!("waste_port_misroute_capture_right_clamp_r{row}_c{col}"),
                    8.0,
                    CAPTURE_BAG_Y + 18.0,
                    18.0,
                )
                .translate(x + CAPTURE_BAG_X / 2.0 + 8.0, y, SPLIT_Z / 2.0 + 9.0);
        }
    }
    fingers
}

fn split_balance_wells() -> Part {
    let mut wells = Part::empty("waste_port_misroute_split_balance_wells");
    for branch in 0..SPLIT_BRANCHES {
        wells = wells
            + centered_cylinder(
                format!("waste_port_misroute_split_balance_well_{branch}"),
                18.0,
                12.0,
                36,
            )
            .translate(
                SPLIT_X / 2.0 - 34.0,
                centered_index(branch, SPLIT_BRANCHES, CAPTURE_PITCH_Y),
                SPLIT_Z / 2.0 + 6.0,
            );
    }
    wells
}

fn leak_drip_containment_tray() -> Part {
    let tray = centered_cube(
        "waste_port_misroute_leak_drip_containment_tray_body",
        DRIP_X,
        DRIP_Y,
        DRIP_Z,
    );
    let inner_basin = centered_cube(
        "waste_port_misroute_leak_drip_inner_basin_cut",
        DRIP_X - 40.0,
        DRIP_Y - 42.0,
        13.0,
    )
    .translate(0.0, 0.0, DRIP_Z / 2.0 - 6.5);

    tray - inner_basin - drip_channel_cuts() - drip_cup_cuts()
        + drip_weir_ribs()
        + capillary_indicator_tabs()
}

fn drip_channel_cuts() -> Part {
    let mut channels = Part::empty("waste_port_misroute_drip_channel_cuts");
    for channel in 0..DRIP_CHANNELS {
        channels = channels
            + centered_cube(
                format!("waste_port_misroute_drip_channel_cut_{channel}"),
                DRIP_CHANNEL_W,
                DRIP_Y - 64.0,
                9.0,
            )
            .translate(
                centered_index(channel, DRIP_CHANNELS, 38.0),
                0.0,
                DRIP_Z / 2.0 - 5.0,
            );
    }
    channels
}

fn drip_cup_cuts() -> Part {
    let mut cups = Part::empty("waste_port_misroute_drip_cup_cuts");
    for cup in 0..DRIP_CUP_COUNT {
        let row = cup / 4;
        let col = cup % 4;
        cups = cups
            + centered_cylinder(
                format!("waste_port_misroute_drip_cup_cut_{cup}"),
                DRIP_CUP_D / 2.0,
                15.0,
                32,
            )
            .translate(
                centered_index(col, 4, 56.0),
                centered_index(row, 2, 78.0),
                DRIP_Z / 2.0 - 8.0,
            );
    }
    cups
}

fn drip_weir_ribs() -> Part {
    let mut ribs = Part::empty("waste_port_misroute_drip_weir_ribs");
    for rib in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("waste_port_misroute_drip_weir_rib_{rib}"),
                DRIP_X - 54.0,
                6.0,
                16.0,
            )
            .translate(0.0, centered_index(rib, 5, 44.0), DRIP_Z / 2.0 + 8.0);
    }
    ribs
}

fn capillary_indicator_tabs() -> Part {
    let mut tabs = Part::empty("waste_port_misroute_capillary_indicator_tabs");
    for tab in 0..DRIP_CHANNELS {
        tabs = tabs
            + centered_cube(
                format!("waste_port_misroute_capillary_indicator_tab_{tab}"),
                20.0,
                12.0,
                7.0,
            )
            .translate(
                centered_index(tab, DRIP_CHANNELS, 38.0),
                DRIP_Y / 2.0 - 30.0,
                DRIP_Z / 2.0 + 3.5,
            );
    }
    tabs
}

fn barcode_coa_custody_panel() -> Part {
    let panel = centered_cube(
        "waste_port_misroute_barcode_coa_custody_panel_base",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    panel - barcode_land_recesses() - coa_clip_recesses()
        + custody_seal_slots()
        + custody_status_rail()
        + custody_scan_fiducials()
}

fn barcode_land_recesses() -> Part {
    let mut lands = Part::empty("waste_port_misroute_barcode_land_recesses");
    for land in 0..BARCODE_LANDS {
        let row = land / 5;
        let col = land % 5;
        lands = lands
            + centered_cube(
                format!("waste_port_misroute_barcode_land_recess_{land}"),
                54.0,
                20.0,
                6.0,
            )
            .translate(
                centered_index(col, 5, 70.0),
                centered_index(row, 2, 48.0) - 18.0,
                CUSTODY_Z / 2.0 - 3.0,
            );
    }
    lands
}

fn coa_clip_recesses() -> Part {
    let mut clips = Part::empty("waste_port_misroute_coa_clip_recesses");
    for clip in 0..COA_CLIPS {
        clips = clips
            + centered_cube(
                format!("waste_port_misroute_coa_clip_recess_{clip}"),
                34.0,
                10.0,
                6.0,
            )
            .translate(
                centered_index(clip, COA_CLIPS, 56.0),
                CUSTODY_Y / 2.0 - 24.0,
                CUSTODY_Z / 2.0 - 3.0,
            );
    }
    clips
}

fn custody_seal_slots() -> Part {
    let mut slots = Part::empty("waste_port_misroute_custody_seal_slots");
    for state in 0..CUSTODY_SEAL_SLOTS {
        slots = slots
            + centered_cube(
                format!("waste_port_misroute_{}_seal_slot", CUSTODY_STATES[state]),
                48.0,
                12.0,
                9.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 52.0 + state as f64 * 60.0,
                -CUSTODY_Y / 2.0 + 20.0,
                CUSTODY_Z / 2.0 + 4.5,
            );
    }
    slots
}

fn custody_status_rail() -> Part {
    centered_cube(
        "waste_port_misroute_custody_status_rail",
        CUSTODY_X - 52.0,
        8.0,
        12.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0 + 6.0)
}

fn custody_scan_fiducials() -> Part {
    fiducial_disc("waste_port_misroute_custody_scan_fiducial_left").translate(
        -CUSTODY_X / 2.0 + 24.0,
        CUSTODY_Y / 2.0 - 24.0,
        CUSTODY_Z / 2.0 + 3.0,
    ) + fiducial_disc("waste_port_misroute_custody_scan_fiducial_right").translate(
        CUSTODY_X / 2.0 - 24.0,
        CUSTODY_Y / 2.0 - 24.0,
        CUSTODY_Z / 2.0 + 3.0,
    )
}

fn hold_release_reject_gate_lanes() -> Part {
    let panel = centered_cube(
        "waste_port_misroute_hold_release_reject_gate_lanes_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );

    panel - gate_lane_recesses() - gate_token_cuts()
        + gate_slider_rails()
        + interlock_bosses()
        + disposition_stop_blocks()
}

fn gate_lane_recesses() -> Part {
    let mut lanes = Part::empty("waste_port_misroute_gate_lane_recesses");
    for lane in 0..GATE_LANES {
        lanes = lanes
            + centered_cube(
                format!(
                    "waste_port_misroute_{}_gate_lane_recess",
                    DISPOSITION_LANES[lane]
                ),
                GATE_X - 70.0,
                32.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(lane, GATE_LANES, 44.0),
                GATE_Z / 2.0 - 4.0,
            );
    }
    lanes
}

fn gate_token_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_gate_token_cuts");
    for lane in 0..GATE_LANES {
        for token in 0..GATE_TOKENS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("waste_port_misroute_gate_token_cut_l{lane}_{token}"),
                    26.0,
                    20.0,
                    9.0,
                )
                .translate(
                    centered_index(token, GATE_TOKENS_PER_LANE, 58.0),
                    centered_index(lane, GATE_LANES, 44.0),
                    GATE_Z / 2.0 - 5.0,
                );
        }
    }
    cuts
}

fn gate_slider_rails() -> Part {
    let mut rails = Part::empty("waste_port_misroute_gate_slider_rails");
    for lane in 0..GATE_LANES {
        let y = centered_index(lane, GATE_LANES, 44.0);
        rails = rails
            + centered_cube(
                format!(
                    "waste_port_misroute_{}_upper_slider_rail",
                    DISPOSITION_LANES[lane]
                ),
                GATE_X - 64.0,
                6.0,
                12.0,
            )
            .translate(0.0, y + 20.0, GATE_Z / 2.0 + 6.0)
            + centered_cube(
                format!(
                    "waste_port_misroute_{}_lower_slider_rail",
                    DISPOSITION_LANES[lane]
                ),
                GATE_X - 64.0,
                6.0,
                12.0,
            )
            .translate(0.0, y - 20.0, GATE_Z / 2.0 + 6.0);
    }
    rails
}

fn interlock_bosses() -> Part {
    let mut bosses = Part::empty("waste_port_misroute_interlock_bosses");
    for lane in 0..GATE_LANES {
        bosses = bosses
            + centered_cylinder(
                format!("waste_port_misroute_interlock_boss_{lane}"),
                12.0,
                12.0,
                32,
            )
            .translate(
                -GATE_X / 2.0 + 38.0,
                centered_index(lane, GATE_LANES, 44.0),
                GATE_Z / 2.0 + 6.0,
            );
    }
    bosses
}

fn disposition_stop_blocks() -> Part {
    let mut stops = Part::empty("waste_port_misroute_disposition_stop_blocks");
    for lane in 0..GATE_LANES {
        stops = stops
            + centered_cube(
                format!("waste_port_misroute_disposition_stop_block_{lane}"),
                18.0,
                34.0,
                16.0,
            )
            .translate(
                GATE_X / 2.0 - 40.0,
                centered_index(lane, GATE_LANES, 44.0),
                GATE_Z / 2.0 + 8.0,
            );
    }
    stops
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "waste_port_misroute_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_X / 2.0, 360.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        "waste_port_misroute_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_X / 2.0, 360.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        "waste_port_misroute_evidence_bridge_beam",
        BRIDGE_X + BRIDGE_POST_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 360.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    left_post
        + right_post
        + beam
        + camera_pods()
        + evidence_light_bars()
        + evidence_fiducial_rulers()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("waste_port_misroute_camera_pods");
    for pod in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("waste_port_misroute_camera_pod_{pod}"),
                46.0,
                36.0,
                24.0,
            )
            .translate(
                centered_index(pod, CAMERA_PODS, 210.0),
                360.0,
                BRIDGE_UNDERSIDE_Z - 12.0,
            ) + centered_cylinder(
                format!("waste_port_misroute_camera_lens_clearance_{pod}"),
                9.0,
                10.0,
                24,
            )
            .translate(
                centered_index(pod, CAMERA_PODS, 210.0),
                360.0,
                BRIDGE_UNDERSIDE_Z - 28.0,
            );
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("waste_port_misroute_evidence_light_bars");
    for bar in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("waste_port_misroute_evidence_light_bar_{bar}"),
                220.0,
                12.0,
                12.0,
            )
            .translate(
                centered_index(bar, LIGHT_BARS, 290.0),
                360.0 - BRIDGE_Y / 2.0 - 10.0,
                BRIDGE_UNDERSIDE_Z - 18.0,
            );
    }
    bars
}

fn evidence_fiducial_rulers() -> Part {
    let mut rulers = Part::empty("waste_port_misroute_evidence_fiducial_rulers");
    for fiducial in 0..EVIDENCE_FIDUCIALS {
        rulers = rulers
            + centered_cube(
                format!("waste_port_misroute_evidence_scale_tick_{fiducial}"),
                6.0,
                34.0,
                10.0,
            )
            .translate(
                centered_index(fiducial, EVIDENCE_FIDUCIALS, 150.0),
                360.0 + BRIDGE_Y / 2.0 + 8.0,
                BRIDGE_UNDERSIDE_Z - 18.0,
            );
    }
    rulers
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "waste_port_misroute_robot_front_keepout_gauge",
        DECK_X - 120.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE, KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        "waste_port_misroute_service_rear_keepout_gauge",
        DECK_X - 120.0,
        KEEP_OUT_RAIL_W,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE, KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "waste_port_misroute_left_service_keepout_gauge",
        KEEP_OUT_RAIL_W,
        DECK_Y - 120.0,
        KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_SERVICE_CLEARANCE,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        "waste_port_misroute_right_service_keepout_gauge",
        KEEP_OUT_RAIL_W,
        DECK_Y - 120.0,
        KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_CLEARANCE,
        0.0,
        KEEP_OUT_Z / 2.0,
    );
    let z_gauge = centered_cube(
        "waste_port_misroute_robot_z_clearance_gauge",
        42.0,
        42.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 150.0,
        -DECK_Y / 2.0 + 135.0,
        ROBOT_Z_CLEARANCE / 2.0,
    );

    front + rear + left + right + z_gauge + keepout_corner_flags()
}

fn keepout_corner_flags() -> Part {
    let mut flags = Part::empty("waste_port_misroute_keepout_corner_flags");
    for (i, (x, y)) in [
        (
            -DECK_X / 2.0 + LEFT_SERVICE_CLEARANCE,
            -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE,
        ),
        (
            DECK_X / 2.0 - RIGHT_SERVICE_CLEARANCE,
            -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE,
        ),
        (
            -DECK_X / 2.0 + LEFT_SERVICE_CLEARANCE,
            DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE,
        ),
        (
            DECK_X / 2.0 - RIGHT_SERVICE_CLEARANCE,
            DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        flags = flags
            + centered_cube(
                format!("waste_port_misroute_keepout_corner_flag_{i}"),
                32.0,
                32.0,
                18.0,
            )
            .translate(x, y, 9.0);
    }
    flags
}

fn closed_custody_retain_archive() -> Part {
    let base = centered_cube(
        "waste_port_misroute_closed_custody_retain_archive_base",
        ARCHIVE_X,
        ARCHIVE_Y,
        ARCHIVE_Z,
    );

    base - retain_archive_vial_cuts() - retain_seal_window_cuts()
        + archive_latch_bar()
        + archive_chain_of_custody_spine()
}

fn retain_archive_vial_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_retain_archive_vial_cuts");
    for vial in 0..RETAIN_ARCHIVE_VIALS {
        let row = vial / 3;
        let col = vial % 3;
        cuts = cuts
            + centered_cylinder(
                format!("waste_port_misroute_retain_archive_vial_cut_{vial}"),
                15.0,
                22.0,
                32,
            )
            .translate(
                centered_index(col, 3, 68.0) - 74.0,
                centered_index(row, 2, 58.0),
                ARCHIVE_Z / 2.0 - 11.0,
            );
    }
    cuts
}

fn retain_seal_window_cuts() -> Part {
    let mut cuts = Part::empty("waste_port_misroute_retain_seal_window_cuts");
    for window in 0..RETAIN_SEAL_WINDOWS {
        let row = window / 3;
        let col = window % 3;
        cuts = cuts
            + centered_cube(
                format!("waste_port_misroute_retain_seal_window_cut_{window}"),
                38.0,
                18.0,
                8.0,
            )
            .translate(
                centered_index(col, 3, 62.0) + 100.0,
                centered_index(row, 2, 58.0),
                ARCHIVE_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn archive_latch_bar() -> Part {
    centered_cube(
        "waste_port_misroute_archive_tamper_latch_bar",
        ARCHIVE_X - 52.0,
        12.0,
        14.0,
    )
    .translate(0.0, ARCHIVE_Y / 2.0 - 28.0, ARCHIVE_Z / 2.0 + 7.0)
}

fn archive_chain_of_custody_spine() -> Part {
    centered_cube(
        "waste_port_misroute_archive_chain_of_custody_spine",
        10.0,
        ARCHIVE_Y - 44.0,
        12.0,
    )
    .translate(0.0, 0.0, ARCHIVE_Z / 2.0 + 6.0)
}

fn frame_rect(name: &str, outer_x: f64, outer_y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn donut(name: &str, outer_radius: f64, inner_radius: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_radius, z, 40)
        - centered_cylinder(format!("{name}_inner_cut"), inner_radius, z + 1.0, 40)
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 12.0, 5.0, 32)
        - centered_cylinder(format!("{name}_center_pip_cut"), 3.5, 6.0, 24)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cassette_waste_port_misroute_colorimetric_witness_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "keyed_cassette_waste_port_dock",
            "automated_route_selector_manifold",
            "misroute_challenge_token_plate",
            "colorimetric_witness_coupon_array",
            "waste_retain_split_capture",
            "leak_drip_containment_tray",
            "barcode_coa_custody_panel",
            "hold_release_reject_gate_lanes",
            "camera_evidence_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
        assert_design_constraints();
        for item in module_rects() {
            assert!(item.fits_inside_deck(36.0), "{}", item.name);
        }
    }

    #[test]
    fn cassette_routing_and_misroute_detection_counts_are_tied() {
        assert_eq!(CASSETTE_SLOTS, 6);
        assert_eq!(ROUTE_PATHS, CASSETTE_SLOTS);
        assert_eq!(ROUTE_STATES, ["waste", "retain", "reject", "witness"]);
        assert_eq!(VALVE_POCKETS, ROUTE_PATHS * ROUTE_STATES.len());
        assert_eq!(RFID_READBACK_LANDS, ROUTE_PATHS);
        assert_eq!(MISROUTE_CHALLENGES.len(), 5);
        assert_eq!(CHALLENGE_TOKENS, MISROUTE_CHALLENGES.len() * 2);
    }

    #[test]
    fn witness_and_containment_capacity_exceeds_challenge_volume() {
        assert_eq!(WITNESS_COUPONS, 18);
        assert!(WITNESS_COUPONS >= ROUTE_PATHS * 3);
        assert_eq!(COLOR_REFERENCE_TILES, 6);
        assert_eq!(DRIP_CHANNELS, ROUTE_PATHS);
        assert_eq!(DRIP_CUP_COUNT, 8);
        assert!(containment_freeboard_ml() > maximum_challenge_liquid_ml());
    }

    #[test]
    fn waste_retain_custody_and_disposition_are_complete() {
        assert_eq!(SPLIT_BRANCHES, 2);
        assert_eq!(WASTE_CAPTURE_BAYS, RETAIN_CAPTURE_BAYS);
        assert!(WASTE_VOLUME_ML > RETAIN_VOLUME_ML);
        assert_eq!(BARCODE_LANDS, CASSETTE_SLOTS + ROUTE_STATES.len());
        assert_eq!(COA_CLIPS, CASSETTE_SLOTS);
        assert_eq!(CUSTODY_SEAL_SLOTS, CUSTODY_STATES.len());
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(
            GATE_TOKEN_SLOTS,
            DISPOSITION_LANES.len() * GATE_TOKENS_PER_LANE
        );
        assert_eq!(RETAIN_ARCHIVE_VIALS, ROUTE_PATHS);
    }

    #[test]
    fn evidence_bridge_and_robot_service_clearances_are_explicit() {
        assert_eq!(CAMERA_PODS, CASSETTE_SLOTS);
        assert_eq!(LIGHT_BARS, 4);
        assert_eq!(EVIDENCE_FIDUCIALS, 8);
        assert!(BRIDGE_UNDERSIDE_Z > DECK_Z + ROUTE_Z + 120.0);
        assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
        assert!(ROBOT_FRONT_CLEARANCE >= 360.0);
        assert!(SERVICE_REAR_CLEARANCE >= 220.0);
        assert!(LEFT_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 220.0);
    }
}
