use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental monitoring plate/coupon exposure and recovery station.
//
// Intent:
// - Package settle plates, contact plates, surface coupons, wipe evidence, and
//   exposure-timer tokens as traceable isolator-support consumables.
// - Keep clean incoming plates/coupons physically separated from exposed,
//   recovered, suspect, rejected, and waste materials.
// - Provide sealed cassette docking, robot pick datums, evidence imaging,
//   barcode/RFID/run-record lands, and service keepouts without loose manual
//   staging around the closed culture workcell.
//
// Product concept CAD only. This does not define an environmental monitoring
// sampling plan, incubation condition, acceptance criterion, or sterility claim.

const OUTPUTS: [&str; 13] = [
    "output/closed_environmental_monitoring_plate_coupon_station_base_leak_tray.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_sealed_plate_cassette_dock.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_settle_plate_exposure_grid.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_contact_plate_recovery_nests.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_surface_coupon_carrier.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_barcode_rfid_lands.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_exposure_timer_token_slots.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_released_hold_reject_lanes.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_contamination_suspect_isolation_cover.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_waste_offcut_tray.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_evidence_photo_bridge.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_clean_used_robot_keepouts.stl",
    "output/closed_environmental_monitoring_plate_coupon_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_plate_cassette_dock",
    "settle_plate_exposure_grid",
    "contact_plate_recovery_nests",
    "surface_coupon_carrier",
    "barcode_rfid_lands",
    "exposure_timer_token_slots",
    "released_hold_reject_lanes",
    "contamination_suspect_isolation_cover",
    "waste_offcut_tray",
    "evidence_photo_bridge",
    "clean_used_robot_keepouts",
];

const DECK_X: f64 = 1480.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;

const CASSETTE_DOCK_X: f64 = 420.0;
const CASSETTE_DOCK_Y: f64 = 250.0;
const CASSETTE_DOCK_Z: f64 = 58.0;
const CASSETTE_POS: (f64, f64) = (-500.0, 258.0);
const PLATE_CASSETTE_COUNT: usize = 2;
const CASSETTE_PLATE_SLOTS_PER_SIDE: usize = 6;
const CASSETTE_GATE_Z: f64 = 132.0;
const CASSETTE_SEAL_GROOVE_W: f64 = 7.0;

const SETTLE_GRID_X: f64 = 540.0;
const SETTLE_GRID_Y: f64 = 342.0;
const SETTLE_GRID_Z: f64 = 42.0;
const SETTLE_POS: (f64, f64) = (10.0, 238.0);
const SETTLE_ROWS: usize = 2;
const SETTLE_COLS: usize = 4;
const SETTLE_PLATE_COUNT: usize = SETTLE_ROWS * SETTLE_COLS;
const SETTLE_PLATE_D: f64 = 90.0;
const SETTLE_CLEARANCE_D: f64 = 94.0;
const SETTLE_LID_PARK_COUNT: usize = SETTLE_PLATE_COUNT;
const SETTLE_PITCH_X: f64 = 118.0;
const SETTLE_PITCH_Y: f64 = 142.0;

const CONTACT_NEST_X: f64 = 430.0;
const CONTACT_NEST_Y: f64 = 276.0;
const CONTACT_NEST_Z: f64 = 44.0;
const CONTACT_POS: (f64, f64) = (-466.0, -84.0);
const CONTACT_ROWS: usize = 3;
const CONTACT_COLS: usize = 4;
const CONTACT_PLATE_COUNT: usize = CONTACT_ROWS * CONTACT_COLS;
const CONTACT_PLATE_D: f64 = 55.0;
const CONTACT_CLEARANCE_D: f64 = 61.0;
const CONTACT_PITCH_X: f64 = 84.0;
const CONTACT_PITCH_Y: f64 = 72.0;

const COUPON_CARRIER_X: f64 = 394.0;
const COUPON_CARRIER_Y: f64 = 286.0;
const COUPON_CARRIER_Z: f64 = 40.0;
const COUPON_POS: (f64, f64) = (-12.0, -55.0);
const COUPON_ROWS: usize = 4;
const COUPON_COLS: usize = 5;
const COUPON_COUNT: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 52.0;
const COUPON_SLOT_Y: f64 = 30.0;
const COUPON_PITCH_X: f64 = 66.0;
const COUPON_PITCH_Y: f64 = 54.0;
const WIPE_EVIDENCE_SLOTS: usize = 8;

const TRACE_PANEL_X: f64 = 420.0;
const TRACE_PANEL_Y: f64 = 144.0;
const TRACE_PANEL_Z: f64 = 8.0;
const TRACE_POS: (f64, f64) = (506.0, 40.0);
const BARCODE_LANDS: usize = 16;
const RFID_LANDS: usize = 8;
const BARCODE_LAND_X: f64 = 82.0;
const BARCODE_LAND_Y: f64 = 23.0;
const RFID_LAND_X: f64 = 42.0;
const RFID_LAND_Y: f64 = 32.0;

const TOKEN_PANEL_X: f64 = 336.0;
const TOKEN_PANEL_Y: f64 = 134.0;
const TOKEN_PANEL_Z: f64 = 32.0;
const TOKEN_POS: (f64, f64) = (530.0, 276.0);
const TIMER_TOKEN_SLOTS: usize = 12;
const TOKEN_SLOT_X: f64 = 36.0;
const TOKEN_SLOT_Y: f64 = 24.0;
const TOKEN_PITCH_X: f64 = 48.0;
const TOKEN_PITCH_Y: f64 = 44.0;

const STATUS_X: f64 = 388.0;
const STATUS_Y: f64 = 260.0;
const STATUS_Z: f64 = 46.0;
const STATUS_POS: (f64, f64) = (504.0, -270.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 5;
const STATUS_SLOT_X: f64 = 96.0;
const STATUS_SLOT_Y: f64 = 42.0;
const STATUS_LANE_PITCH_Y: f64 = 74.0;

const ISOLATION_X: f64 = 344.0;
const ISOLATION_Y: f64 = 244.0;
const ISOLATION_Z: f64 = 210.0;
const ISOLATION_POS: (f64, f64) = (-70.0, -330.0);
const ISOLATION_SAMPLE_COUNT: usize = 4;
const ISOLATION_GASKET_W: f64 = 10.0;

const WASTE_TRAY_X: f64 = 326.0;
const WASTE_TRAY_Y: f64 = 118.0;
const WASTE_TRAY_Z: f64 = 34.0;
const WASTE_POS: (f64, f64) = (-512.0, -348.0);
const OFFCUT_WELL_COUNT: usize = 10;

const PHOTO_BRIDGE_X: f64 = 1060.0;
const PHOTO_BRIDGE_Y: f64 = 210.0;
const PHOTO_BRIDGE_Z: f64 = 28.0;
const PHOTO_BRIDGE_POS: (f64, f64) = (22.0, 340.0);
const PHOTO_BRIDGE_UNDERSIDE_Z: f64 = 238.0;
const CAMERA_PODS: usize = 4;
const LED_BAR_COUNT: usize = 2;

const CLEAN_USED_SPINE_X: f64 = 1110.0;
const CLEAN_USED_SPINE_Y: f64 = 18.0;
const CLEAN_USED_SPINE_Z: f64 = 74.0;
const CLEAN_USED_SPINE_POS: (f64, f64) = (-6.0, -4.0);

const FRONT_ROBOT_APPROACH: f64 = 430.0;
const REAR_SERVICE_ACCESS: f64 = 270.0;
const SIDE_CASSETTE_LOAD_ACCESS: f64 = 260.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        let half_x = deck_x / 2.0 - RIM_W;
        let half_y = deck_y / 2.0 - RIM_W;
        (self.center.0 - self.x / 2.0) >= -half_x
            && (self.center.0 + self.x / 2.0) <= half_x
            && (self.center.1 - self.y / 2.0) >= -half_y
            && (self.center.1 + self.y / 2.0) <= half_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let cassette_dock = sealed_plate_cassette_dock();
    export(OUTPUTS[1], &cassette_dock);

    let settle_grid = settle_plate_exposure_grid();
    export(OUTPUTS[2], &settle_grid);

    let contact_nests = contact_plate_recovery_nests();
    export(OUTPUTS[3], &contact_nests);

    let coupon_carrier = surface_coupon_carrier();
    export(OUTPUTS[4], &coupon_carrier);

    let traceability = barcode_rfid_lands();
    export(OUTPUTS[5], &traceability);

    let timer_tokens = exposure_timer_token_slots();
    export(OUTPUTS[6], &timer_tokens);

    let status_lanes = released_hold_reject_lanes();
    export(OUTPUTS[7], &status_lanes);

    let isolation = contamination_suspect_isolation_cover();
    export(OUTPUTS[8], &isolation);

    let waste = waste_offcut_tray();
    export(OUTPUTS[9], &waste);

    let photo_bridge = evidence_photo_bridge();
    export(OUTPUTS[10], &photo_bridge);

    let keepouts = clean_used_robot_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + cassette_dock.translate(CASSETTE_POS.0, CASSETTE_POS.1, insert_z(CASSETTE_DOCK_Z))
        + settle_grid.translate(SETTLE_POS.0, SETTLE_POS.1, insert_z(SETTLE_GRID_Z))
        + contact_nests.translate(CONTACT_POS.0, CONTACT_POS.1, insert_z(CONTACT_NEST_Z))
        + coupon_carrier.translate(COUPON_POS.0, COUPON_POS.1, insert_z(COUPON_CARRIER_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z))
        + timer_tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_PANEL_Z))
        + status_lanes.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + isolation.translate(ISOLATION_POS.0, ISOLATION_POS.1, insert_z(ISOLATION_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_TRAY_Z))
        + photo_bridge
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed environmental monitoring plate/coupon station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm leak-tray deck");
    println!(
        "  Plate handling:             {PLATE_CASSETTE_COUNT} sealed cassette lanes, {} cassette plate slots, {SETTLE_PLATE_COUNT} settle plate exposure positions, {SETTLE_LID_PARK_COUNT} lid parks, and {CONTACT_PLATE_COUNT} contact plate recovery nests",
        PLATE_CASSETTE_COUNT * CASSETTE_PLATE_SLOTS_PER_SIDE
    );
    println!(
        "  Coupon/evidence handling:   {COUPON_COUNT} surface coupon slots, {WIPE_EVIDENCE_SLOTS} wipe evidence pockets, {TIMER_TOKEN_SLOTS} exposure timer token slots, {OFFCUT_WELL_COUNT} waste/offcut wells"
    );
    println!(
        "  Traceability:               {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {STATUS_LANES} released/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each, {ISOLATION_SAMPLE_COUNT} suspect isolation positions"
    );
    println!(
        "  Imaging and keepouts:       {CAMERA_PODS} camera pods, {LED_BAR_COUNT} LED bars, {PHOTO_BRIDGE_UNDERSIDE_Z:.0}mm photo-bridge underside, {FRONT_ROBOT_APPROACH:.0}mm robot approach, {REAR_SERVICE_ACCESS:.0}mm rear service, {SIDE_CASSETTE_LOAD_ACCESS:.0}mm cassette-load access, {ROBOT_Z_CLEARANCE:.0}mm Z clearance"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn assert_layout_constraints() {
    for rect in layout_rects() {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds station deck envelope",
            rect.name
        );
    }

    for (a, b) in non_overlap_pairs() {
        assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
    }

    assert_eq!(SETTLE_PLATE_COUNT, 8);
    assert_eq!(CONTACT_PLATE_COUNT, 12);
    assert_eq!(COUPON_COUNT, 20);
    assert!(SETTLE_CLEARANCE_D > SETTLE_PLATE_D + 2.0);
    assert!(CONTACT_CLEARANCE_D > CONTACT_PLATE_D + 2.0);
    assert!(PHOTO_BRIDGE_UNDERSIDE_Z > ISOLATION_Z + DECK_Z);
    assert!(ROBOT_Z_CLEARANCE > PHOTO_BRIDGE_UNDERSIDE_Z);
}

fn layout_rects() -> [Rect; 10] {
    [
        rect(
            "sealed_plate_cassette_dock",
            CASSETTE_POS,
            CASSETTE_DOCK_X,
            CASSETTE_DOCK_Y,
        ),
        rect(
            "settle_plate_exposure_grid",
            SETTLE_POS,
            SETTLE_GRID_X,
            SETTLE_GRID_Y,
        ),
        rect(
            "contact_plate_recovery_nests",
            CONTACT_POS,
            CONTACT_NEST_X,
            CONTACT_NEST_Y,
        ),
        rect(
            "surface_coupon_carrier",
            COUPON_POS,
            COUPON_CARRIER_X,
            COUPON_CARRIER_Y,
        ),
        rect(
            "barcode_rfid_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        rect(
            "exposure_timer_token_slots",
            TOKEN_POS,
            TOKEN_PANEL_X,
            TOKEN_PANEL_Y,
        ),
        rect("released_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        rect(
            "contamination_suspect_isolation_cover",
            ISOLATION_POS,
            ISOLATION_X,
            ISOLATION_Y,
        ),
        rect("waste_offcut_tray", WASTE_POS, WASTE_TRAY_X, WASTE_TRAY_Y),
        rect(
            "evidence_photo_bridge",
            PHOTO_BRIDGE_POS,
            PHOTO_BRIDGE_X,
            PHOTO_BRIDGE_Y,
        ),
    ]
}

fn non_overlap_pairs() -> [(Rect, Rect); 10] {
    [
        (layout_rects()[0], layout_rects()[1]),
        (layout_rects()[0], layout_rects()[2]),
        (layout_rects()[1], layout_rects()[4]),
        (layout_rects()[1], layout_rects()[5]),
        (layout_rects()[2], layout_rects()[3]),
        (layout_rects()[2], layout_rects()[8]),
        (layout_rects()[3], layout_rects()[6]),
        (layout_rects()[4], layout_rects()[5]),
        (layout_rects()[6], layout_rects()[7]),
        (layout_rects()[7], layout_rects()[8]),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "environmental_monitoring_plate_coupon_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let recessed_pan = centered_cube(
        "environmental_monitoring_plate_coupon_recessed_leak_pan",
        DECK_X - 2.0 * (RIM_W + 42.0),
        DECK_Y - 2.0 * (RIM_W + 46.0),
        8.0,
    )
    .translate(0.0, -8.0, DECK_Z - 4.0);
    let front_drain_channel = centered_cube(
        "environmental_monitoring_plate_coupon_front_low_point_channel",
        DECK_X - 148.0,
        24.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 72.0, DECK_Z - 4.0);
    let drain_port = centered_cylinder(
        "environmental_monitoring_plate_coupon_closed_drain_port",
        9.0,
        48.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 36.0, DECK_Z - 8.0);

    deck - recessed_pan - front_drain_channel - drain_port - insert_sockets() - deck_mount_holes()
        + perimeter_rim()
        + zone_label_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("environmental_monitoring_insert_registration_sockets");
    for rect in layout_rects().iter().take(8) {
        sockets = sockets
            + centered_cube(
                format!("environmental_monitoring_socket_{}", rect.name),
                rect.x + 16.0,
                rect.y + 16.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "environmental_monitoring_front_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "environmental_monitoring_rear_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "environmental_monitoring_left_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "environmental_monitoring_right_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("environmental_monitoring_base_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
        (0.0, -DECK_Y / 2.0 + 54.0),
        (0.0, DECK_Y / 2.0 - 54.0),
        (-DECK_X / 2.0 + 54.0, 0.0),
        (DECK_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("environmental_monitoring_m6_mount_hole_{i}"),
                3.4,
                DECK_Z + 3.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn zone_label_lands() -> Part {
    let clean = centered_cube(
        "environmental_monitoring_clean_incoming_zone_label_land",
        250.0,
        28.0,
        2.0,
    )
    .translate(-452.0, 98.0, DECK_Z + 1.0);
    let exposure = centered_cube(
        "environmental_monitoring_exposure_zone_label_land",
        250.0,
        28.0,
        2.0,
    )
    .translate(112.0, 18.0, DECK_Z + 1.0);
    let recovered = centered_cube(
        "environmental_monitoring_recovered_used_zone_label_land",
        250.0,
        28.0,
        2.0,
    )
    .translate(455.0, -104.0, DECK_Z + 1.0);
    clean + exposure + recovered
}

fn sealed_plate_cassette_dock() -> Part {
    let body = centered_cube(
        "environmental_monitoring_sealed_plate_cassette_dock_body",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let seal_recess = centered_cube(
        "environmental_monitoring_cassette_dock_gasket_recess",
        CASSETTE_DOCK_X - 56.0,
        CASSETTE_DOCK_Y - 54.0,
        CASSETTE_DOCK_Z + 2.0,
    )
    .translate(0.0, 0.0, 8.0);
    let docking_throat = centered_cube(
        "environmental_monitoring_cassette_dock_sealed_transfer_throat",
        CASSETTE_DOCK_X - 110.0,
        54.0,
        CASSETTE_DOCK_Z + 8.0,
    )
    .translate(0.0, CASSETTE_DOCK_Y / 2.0 - 38.0, 0.0);
    let gate = centered_cube(
        "environmental_monitoring_cassette_vertical_gate_envelope",
        CASSETTE_DOCK_X - 78.0,
        22.0,
        CASSETTE_GATE_Z,
    )
    .translate(0.0, -CASSETTE_DOCK_Y / 2.0 + 22.0, CASSETTE_GATE_Z / 2.0);
    let gasket_outer = centered_cube(
        "environmental_monitoring_cassette_outer_gasket_land",
        CASSETTE_DOCK_X - 34.0,
        CASSETTE_DOCK_Y - 32.0,
        6.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 4.0);
    let gasket_inner = centered_cube(
        "environmental_monitoring_cassette_inner_gasket_cut",
        CASSETTE_DOCK_X - 34.0 - 2.0 * CASSETTE_SEAL_GROOVE_W,
        CASSETTE_DOCK_Y - 32.0 - 2.0 * CASSETTE_SEAL_GROOVE_W,
        8.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 4.0);

    body - seal_recess - docking_throat - cassette_plate_slots()
        + gate
        + (gasket_outer - gasket_inner)
}

fn cassette_plate_slots() -> Part {
    let mut slots = Part::empty("environmental_monitoring_cassette_plate_slot_stack");
    let lane_pitch = 148.0;
    let slot_pitch = 28.0;
    for lane in 0..PLATE_CASSETTE_COUNT {
        let x = (lane as f64 - 0.5) * lane_pitch;
        for slot in 0..CASSETTE_PLATE_SLOTS_PER_SIDE {
            let y = -62.0 + slot as f64 * slot_pitch;
            slots = slots
                + centered_cube(
                    format!("environmental_monitoring_plate_cassette_lane_{lane}_slot_{slot}"),
                    112.0,
                    16.0,
                    CASSETTE_DOCK_Z + 5.0,
                )
                .translate(x, y, 0.0);
        }
    }
    slots
}

fn settle_plate_exposure_grid() -> Part {
    let tray = centered_cube(
        "environmental_monitoring_settle_plate_exposure_grid_tray",
        SETTLE_GRID_X,
        SETTLE_GRID_Y,
        SETTLE_GRID_Z,
    );
    let rim_cut = centered_cube(
        "environmental_monitoring_settle_plate_grid_inner_spill_cut",
        SETTLE_GRID_X - 42.0,
        SETTLE_GRID_Y - 42.0,
        SETTLE_GRID_Z + 3.0,
    )
    .translate(0.0, 0.0, 8.0);
    let lid_parks = settle_lid_parks();
    tray - settle_plate_recesses() - rim_cut
        + settle_grid_ribs()
        + lid_parks
        + settle_exposure_lift_posts()
}

fn settle_plate_recesses() -> Part {
    let mut recesses = Part::empty("environmental_monitoring_settle_plate_recesses");
    for row in 0..SETTLE_ROWS {
        for col in 0..SETTLE_COLS {
            let (x, y) = settle_plate_xy(row, col);
            recesses = recesses
                + centered_cylinder(
                    format!("environmental_monitoring_settle_plate_recess_r{row}_c{col}"),
                    SETTLE_CLEARANCE_D / 2.0,
                    SETTLE_GRID_Z + 4.0,
                    64,
                )
                .translate(x, y, 0.0);
        }
    }
    recesses
}

fn settle_grid_ribs() -> Part {
    let mut ribs = Part::empty("environmental_monitoring_settle_grid_index_ribs");
    for col in 0..=SETTLE_COLS {
        let x = -SETTLE_PITCH_X * SETTLE_COLS as f64 / 2.0 + col as f64 * SETTLE_PITCH_X;
        ribs = ribs
            + centered_cube(
                format!("environmental_monitoring_settle_grid_x_rib_{col}"),
                6.0,
                SETTLE_GRID_Y - 56.0,
                14.0,
            )
            .translate(x, 0.0, SETTLE_GRID_Z / 2.0 + 7.0);
    }
    for row in 0..=SETTLE_ROWS {
        let y = -SETTLE_PITCH_Y * SETTLE_ROWS as f64 / 2.0 + row as f64 * SETTLE_PITCH_Y;
        ribs = ribs
            + centered_cube(
                format!("environmental_monitoring_settle_grid_y_rib_{row}"),
                SETTLE_GRID_X - 56.0,
                6.0,
                14.0,
            )
            .translate(0.0, y, SETTLE_GRID_Z / 2.0 + 7.0);
    }
    ribs
}

fn settle_lid_parks() -> Part {
    let mut parks = Part::empty("environmental_monitoring_settle_lid_park_lips");
    for row in 0..SETTLE_ROWS {
        for col in 0..SETTLE_COLS {
            let (x, y) = settle_plate_xy(row, col);
            parks = parks
                + centered_cylinder(
                    format!("environmental_monitoring_settle_lid_park_lip_r{row}_c{col}"),
                    (SETTLE_CLEARANCE_D + 8.0) / 2.0,
                    5.0,
                    64,
                )
                .translate(x, y, SETTLE_GRID_Z / 2.0 + 6.0);
            parks = parks
                - centered_cylinder(
                    format!(
                        "environmental_monitoring_settle_lid_park_inner_clearance_r{row}_c{col}"
                    ),
                    (SETTLE_CLEARANCE_D + 1.0) / 2.0,
                    7.0,
                    64,
                )
                .translate(x, y, SETTLE_GRID_Z / 2.0 + 6.0);
        }
    }
    parks
}

fn settle_exposure_lift_posts() -> Part {
    let mut posts = Part::empty("environmental_monitoring_settle_lid_lift_post_pairs");
    for row in 0..SETTLE_ROWS {
        for col in 0..SETTLE_COLS {
            let (x, y) = settle_plate_xy(row, col);
            for dx in [-38.0, 38.0] {
                posts = posts
                    + centered_cylinder(
                        format!("environmental_monitoring_settle_lid_lift_post_r{row}_c{col}_{dx}"),
                        3.5,
                        22.0,
                        18,
                    )
                    .translate(x + dx, y + 48.0, SETTLE_GRID_Z / 2.0 + 11.0);
            }
        }
    }
    posts
}

fn settle_plate_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (SETTLE_COLS as f64 - 1.0) / 2.0) * SETTLE_PITCH_X,
        (row as f64 - (SETTLE_ROWS as f64 - 1.0) / 2.0) * SETTLE_PITCH_Y,
    )
}

fn contact_plate_recovery_nests() -> Part {
    let tray = centered_cube(
        "environmental_monitoring_contact_plate_recovery_nest_tray",
        CONTACT_NEST_X,
        CONTACT_NEST_Y,
        CONTACT_NEST_Z,
    );
    let palm_clearance = centered_cube(
        "environmental_monitoring_contact_plate_robot_palm_clearance",
        CONTACT_NEST_X - 60.0,
        42.0,
        CONTACT_NEST_Z + 3.0,
    )
    .translate(0.0, CONTACT_NEST_Y / 2.0 - 44.0, 0.0);
    tray - contact_plate_recesses() - palm_clearance + contact_gripper_relief_lands()
}

fn contact_plate_recesses() -> Part {
    let mut recesses = Part::empty("environmental_monitoring_contact_plate_recesses");
    for row in 0..CONTACT_ROWS {
        for col in 0..CONTACT_COLS {
            let x = (col as f64 - (CONTACT_COLS as f64 - 1.0) / 2.0) * CONTACT_PITCH_X;
            let y = (row as f64 - (CONTACT_ROWS as f64 - 1.0) / 2.0) * CONTACT_PITCH_Y;
            recesses = recesses
                + centered_cylinder(
                    format!("environmental_monitoring_contact_plate_recovery_recess_r{row}_c{col}"),
                    CONTACT_CLEARANCE_D / 2.0,
                    CONTACT_NEST_Z + 4.0,
                    48,
                )
                .translate(x, y, 0.0);
        }
    }
    recesses
}

fn contact_gripper_relief_lands() -> Part {
    let mut lands = Part::empty("environmental_monitoring_contact_plate_gripper_relief_lands");
    for row in 0..CONTACT_ROWS {
        let y = (row as f64 - (CONTACT_ROWS as f64 - 1.0) / 2.0) * CONTACT_PITCH_Y;
        lands = lands
            + centered_cube(
                format!("environmental_monitoring_contact_plate_row_{row}_gripper_relief"),
                CONTACT_NEST_X - 72.0,
                9.0,
                8.0,
            )
            .translate(0.0, y + 35.0, CONTACT_NEST_Z / 2.0 + 4.0);
    }
    lands
}

fn surface_coupon_carrier() -> Part {
    let carrier = centered_cube(
        "environmental_monitoring_surface_coupon_carrier",
        COUPON_CARRIER_X,
        COUPON_CARRIER_Y,
        COUPON_CARRIER_Z,
    );
    let wipe_lane = centered_cube(
        "environmental_monitoring_wipe_evidence_lane_cut",
        COUPON_CARRIER_X - 48.0,
        58.0,
        COUPON_CARRIER_Z + 3.0,
    )
    .translate(0.0, -COUPON_CARRIER_Y / 2.0 + 48.0, 0.0);
    carrier - coupon_slots() - wipe_evidence_pockets() - wipe_lane + coupon_barriers()
}

fn coupon_slots() -> Part {
    let mut slots = Part::empty("environmental_monitoring_surface_coupon_slots");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let x = (col as f64 - (COUPON_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X;
            let y = 42.0 + (row as f64 - (COUPON_ROWS as f64 - 1.0) / 2.0) * COUPON_PITCH_Y;
            slots = slots
                + centered_cube(
                    format!("environmental_monitoring_surface_coupon_slot_r{row}_c{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_CARRIER_Z + 2.0,
                )
                .translate(x, y, 0.0);
        }
    }
    slots
}

fn wipe_evidence_pockets() -> Part {
    let mut pockets = Part::empty("environmental_monitoring_wipe_evidence_pockets");
    for i in 0..WIPE_EVIDENCE_SLOTS {
        let x = (i as f64 - (WIPE_EVIDENCE_SLOTS as f64 - 1.0) / 2.0) * 42.0;
        pockets = pockets
            + centered_cube(
                format!("environmental_monitoring_wipe_evidence_pocket_{i}"),
                32.0,
                36.0,
                COUPON_CARRIER_Z + 2.0,
            )
            .translate(x, -COUPON_CARRIER_Y / 2.0 + 46.0, 0.0);
    }
    pockets
}

fn coupon_barriers() -> Part {
    let row_barrier = centered_cube(
        "environmental_monitoring_coupon_clean_used_row_barrier",
        COUPON_CARRIER_X - 42.0,
        6.0,
        20.0,
    )
    .translate(0.0, -16.0, COUPON_CARRIER_Z / 2.0 + 10.0);
    let side_guides = centered_cube(
        "environmental_monitoring_coupon_left_robot_rail",
        8.0,
        COUPON_CARRIER_Y - 42.0,
        22.0,
    )
    .translate(
        -COUPON_CARRIER_X / 2.0 + 30.0,
        8.0,
        COUPON_CARRIER_Z / 2.0 + 11.0,
    ) + centered_cube(
        "environmental_monitoring_coupon_right_robot_rail",
        8.0,
        COUPON_CARRIER_Y - 42.0,
        22.0,
    )
    .translate(
        COUPON_CARRIER_X / 2.0 - 30.0,
        8.0,
        COUPON_CARRIER_Z / 2.0 + 11.0,
    );
    row_barrier + side_guides
}

fn barcode_rfid_lands() -> Part {
    let panel = centered_cube(
        "environmental_monitoring_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = panel;
    for i in 0..BARCODE_LANDS {
        let col = i % 4;
        let row = i / 4;
        let x = -150.0 + col as f64 * 96.0;
        let y = 46.0 - row as f64 * 32.0;
        lands = lands
            + centered_cube(
                format!("environmental_monitoring_barcode_lot_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..RFID_LANDS {
        let x = -168.0 + i as f64 * 48.0;
        lands = lands
            + centered_cube(
                format!("environmental_monitoring_rfid_inlay_land_{i}"),
                RFID_LAND_X,
                RFID_LAND_Y,
                2.6,
            )
            .translate(x, -62.0, TRACE_PANEL_Z / 2.0 + 1.3);
    }
    lands
}

fn exposure_timer_token_slots() -> Part {
    let panel = centered_cube(
        "environmental_monitoring_timer_token_panel",
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    );
    let mut token_cuts = Part::empty("environmental_monitoring_timer_token_cuts");
    for i in 0..TIMER_TOKEN_SLOTS {
        let col = i % 6;
        let row = i / 6;
        let x = (col as f64 - 2.5) * TOKEN_PITCH_X;
        let y = (row as f64 - 0.5) * TOKEN_PITCH_Y;
        token_cuts = token_cuts
            + centered_cube(
                format!("environmental_monitoring_exposure_timer_token_slot_{i}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_PANEL_Z + 2.0,
            )
            .translate(x, y, 0.0);
    }
    panel - token_cuts + exposure_window_dials()
}

fn exposure_window_dials() -> Part {
    let mut dials = Part::empty("environmental_monitoring_exposure_window_dials");
    for i in 0..3 {
        dials = dials
            + centered_cylinder(
                format!("environmental_monitoring_exposure_elapsed_window_dial_{i}"),
                13.0,
                4.0,
                36,
            )
            .translate(
                -132.0 + i as f64 * 38.0,
                TOKEN_PANEL_Y / 2.0 - 26.0,
                TOKEN_PANEL_Z / 2.0 + 2.0,
            );
    }
    dials
}

fn released_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "environmental_monitoring_release_hold_reject_tray",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut cuts = Part::empty("environmental_monitoring_release_status_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = (slot as f64 - (STATUS_SLOTS_PER_LANE as f64 - 1.0) / 2.0) * 70.0;
            let y = (lane as f64 - (STATUS_LANES as f64 - 1.0) / 2.0) * STATUS_LANE_PITCH_Y;
            cuts = cuts
                + centered_cube(
                    format!("environmental_monitoring_status_lane_{lane}_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_Z + 2.0,
                )
                .translate(x, y, 0.0);
        }
    }
    tray - cuts + status_lane_dividers()
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("environmental_monitoring_status_lane_dividers");
    for lane_gap in 0..2 {
        let y = -STATUS_LANE_PITCH_Y / 2.0 + lane_gap as f64 * STATUS_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("environmental_monitoring_status_lane_divider_{lane_gap}"),
                STATUS_X - 36.0,
                7.0,
                22.0,
            )
            .translate(0.0, y, STATUS_Z / 2.0 + 11.0);
    }
    dividers
}

fn contamination_suspect_isolation_cover() -> Part {
    let base_flange = centered_cube(
        "environmental_monitoring_suspect_isolation_base_flange",
        ISOLATION_X,
        ISOLATION_Y,
        18.0,
    );
    let cover_outer = centered_cube(
        "environmental_monitoring_suspect_isolation_clear_cover_envelope",
        ISOLATION_X - 42.0,
        ISOLATION_Y - 42.0,
        ISOLATION_Z,
    )
    .translate(0.0, 0.0, ISOLATION_Z / 2.0);
    let cover_inner = centered_cube(
        "environmental_monitoring_suspect_isolation_inner_clearance",
        ISOLATION_X - 80.0,
        ISOLATION_Y - 80.0,
        ISOLATION_Z - 28.0,
    )
    .translate(0.0, 0.0, ISOLATION_Z / 2.0 + 10.0);
    let gasket_outer = centered_cube(
        "environmental_monitoring_suspect_isolation_gasket_land",
        ISOLATION_X - 20.0,
        ISOLATION_Y - 20.0,
        6.0,
    )
    .translate(0.0, 0.0, 14.0);
    let gasket_inner = centered_cube(
        "environmental_monitoring_suspect_isolation_gasket_center_cut",
        ISOLATION_X - 20.0 - 2.0 * ISOLATION_GASKET_W,
        ISOLATION_Y - 20.0 - 2.0 * ISOLATION_GASKET_W,
        8.0,
    )
    .translate(0.0, 0.0, 14.0);
    base_flange
        + (cover_outer - cover_inner)
        + (gasket_outer - gasket_inner)
        + isolation_sample_wells()
}

fn isolation_sample_wells() -> Part {
    let mut wells = Part::empty("environmental_monitoring_isolation_suspect_sample_wells");
    for i in 0..ISOLATION_SAMPLE_COUNT {
        let x = (i as f64 - (ISOLATION_SAMPLE_COUNT as f64 - 1.0) / 2.0) * 58.0;
        wells = wells
            + centered_cylinder(
                format!("environmental_monitoring_suspect_sample_well_lip_{i}"),
                26.0,
                7.0,
                36,
            )
            .translate(x, -44.0, 20.0)
            - centered_cylinder(
                format!("environmental_monitoring_suspect_sample_well_clearance_{i}"),
                20.0,
                9.0,
                36,
            )
            .translate(x, -44.0, 20.0);
    }
    wells
}

fn waste_offcut_tray() -> Part {
    let tray = centered_cube(
        "environmental_monitoring_waste_offcut_secondary_containment_tray",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    );
    let inner = centered_cube(
        "environmental_monitoring_waste_offcut_removable_insert_cut",
        WASTE_TRAY_X - 42.0,
        WASTE_TRAY_Y - 38.0,
        WASTE_TRAY_Z + 3.0,
    )
    .translate(0.0, 0.0, 8.0);
    tray - inner + offcut_wells() + sharps_guard_lip()
}

fn offcut_wells() -> Part {
    let mut wells = Part::empty("environmental_monitoring_offcut_well_lips");
    for i in 0..OFFCUT_WELL_COUNT {
        let x = (i as f64 - (OFFCUT_WELL_COUNT as f64 - 1.0) / 2.0) * 26.0;
        wells = wells
            + centered_cylinder(
                format!("environmental_monitoring_waste_offcut_well_lip_{i}"),
                10.0,
                5.0,
                24,
            )
            .translate(x, 0.0, WASTE_TRAY_Z / 2.0 + 2.5);
    }
    wells
}

fn sharps_guard_lip() -> Part {
    centered_cube(
        "environmental_monitoring_waste_offcut_guard_lip",
        WASTE_TRAY_X - 32.0,
        8.0,
        32.0,
    )
    .translate(0.0, WASTE_TRAY_Y / 2.0 - 12.0, WASTE_TRAY_Z / 2.0 + 16.0)
}

fn evidence_photo_bridge() -> Part {
    let left_post = centered_cube(
        "environmental_monitoring_photo_bridge_left_post",
        42.0,
        PHOTO_BRIDGE_Y,
        PHOTO_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        PHOTO_BRIDGE_POS.0 - PHOTO_BRIDGE_X / 2.0 + 42.0,
        PHOTO_BRIDGE_POS.1,
        DECK_Z + PHOTO_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "environmental_monitoring_photo_bridge_right_post",
        42.0,
        PHOTO_BRIDGE_Y,
        PHOTO_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        PHOTO_BRIDGE_POS.0 + PHOTO_BRIDGE_X / 2.0 - 42.0,
        PHOTO_BRIDGE_POS.1,
        DECK_Z + PHOTO_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "environmental_monitoring_photo_bridge_camera_beam",
        PHOTO_BRIDGE_X,
        PHOTO_BRIDGE_Y,
        PHOTO_BRIDGE_Z,
    )
    .translate(
        PHOTO_BRIDGE_POS.0,
        PHOTO_BRIDGE_POS.1,
        DECK_Z + PHOTO_BRIDGE_UNDERSIDE_Z + PHOTO_BRIDGE_Z / 2.0,
    );
    left_post + right_post + beam + camera_pods() + led_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("environmental_monitoring_photo_bridge_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = PHOTO_BRIDGE_POS.0
            + (i as f64 - (CAMERA_PODS as f64 - 1.0) / 2.0) * (PHOTO_BRIDGE_X - 260.0)
                / (CAMERA_PODS as f64 - 1.0);
        let pod_z = DECK_Z + PHOTO_BRIDGE_UNDERSIDE_Z - 24.0;
        let lens_y = PHOTO_BRIDGE_POS.1 - PHOTO_BRIDGE_Y / 2.0 - 10.0;
        pods = pods
            + centered_cube(
                format!("environmental_monitoring_evidence_camera_pod_{i}"),
                62.0,
                44.0,
                38.0,
            )
            .translate(x, PHOTO_BRIDGE_POS.1 - 5.0, pod_z)
            + centered_cylinder(
                format!("environmental_monitoring_evidence_camera_lens_{i}"),
                11.0,
                16.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, lens_y, pod_z)
            + centered_cylinder(
                format!("environmental_monitoring_evidence_camera_focus_ring_{i}"),
                17.0,
                5.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, lens_y - 10.0, pod_z);
    }
    pods
}

fn led_bars() -> Part {
    let mut bars = Part::empty("environmental_monitoring_photo_bridge_led_bars");
    for side in [-1.0, 1.0] {
        bars = bars
            + centered_cube(
                format!("environmental_monitoring_photo_bridge_led_bar_{side}"),
                PHOTO_BRIDGE_X - 210.0,
                12.0,
                12.0,
            )
            .translate(
                PHOTO_BRIDGE_POS.0,
                PHOTO_BRIDGE_POS.1 + side * 18.0,
                DECK_Z + PHOTO_BRIDGE_UNDERSIDE_Z - 44.0,
            );
    }
    bars
}

fn clean_used_robot_keepouts() -> Part {
    let spine = centered_cube(
        "environmental_monitoring_clean_used_separation_spine",
        CLEAN_USED_SPINE_X,
        CLEAN_USED_SPINE_Y,
        CLEAN_USED_SPINE_Z,
    )
    .translate(
        CLEAN_USED_SPINE_POS.0,
        CLEAN_USED_SPINE_POS.1,
        DECK_Z + CLEAN_USED_SPINE_Z / 2.0,
    );
    let front_keepout = centered_cube(
        "environmental_monitoring_front_robot_approach_keepout",
        DECK_X - 140.0,
        20.0,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + 4.0,
    );
    let rear_keepout = centered_cube(
        "environmental_monitoring_rear_service_keepout",
        DECK_X - 180.0,
        20.0,
        8.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_SERVICE_ACCESS / 2.0, DECK_Z + 4.0);
    let side_keepout = centered_cube(
        "environmental_monitoring_left_cassette_load_keepout",
        20.0,
        DECK_Y - 190.0,
        8.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_CASSETTE_LOAD_ACCESS / 2.0,
        0.0,
        DECK_Z + 4.0,
    );
    let z_gauge = centered_cube(
        "environmental_monitoring_robot_z_clearance_gauge",
        74.0,
        74.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 78.0,
        DECK_Y / 2.0 - 82.0,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    spine + front_keepout + rear_keepout + side_keepout + z_gauge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_features_fit_without_overlap() {
        assert_layout_constraints();
    }

    #[test]
    fn monitoring_capacity_matches_workflow_assumptions() {
        assert_eq!(SETTLE_PLATE_COUNT, 8);
        assert_eq!(CONTACT_PLATE_COUNT, 12);
        assert_eq!(COUPON_COUNT, 20);
        assert_eq!(WIPE_EVIDENCE_SLOTS, 8);
        assert_eq!(TIMER_TOKEN_SLOTS, 12);
    }

    #[test]
    fn traceability_is_not_under_specified() {
        assert!(BARCODE_LANDS >= SETTLE_PLATE_COUNT + CONTACT_PLATE_COUNT / 2);
        assert!(RFID_LANDS >= PLATE_CASSETTE_COUNT + STATUS_LANES);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 15);
    }

    #[test]
    fn keepouts_clear_highest_recovery_feature() {
        assert!(PHOTO_BRIDGE_UNDERSIDE_Z > ISOLATION_Z + DECK_Z);
        assert!(ROBOT_Z_CLEARANCE > PHOTO_BRIDGE_UNDERSIDE_Z + PHOTO_BRIDGE_Z);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(SIDE_CASSETTE_LOAD_ACCESS >= 240.0);
    }

    #[test]
    fn output_manifest_covers_every_exported_part() {
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }
}
