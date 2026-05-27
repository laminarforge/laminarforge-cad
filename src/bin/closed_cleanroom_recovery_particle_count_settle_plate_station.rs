use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleanroom recovery particle-count / settle-plate validation station.
//
// Intent:
// - Stage recovery evidence after door, material-transfer, or personnel events
//   without opening the closed cell-culture boundary.
// - Keep settle/contact plate cassettes, particle-counter probe handling,
//   timed exposure tokens, pressure-cascade event markers, event coupons,
//   custody lands, and evidence imaging in one deterministic fixture.
// - Represent validation workflow geometry only. Particle limits, settle-plate
//   exposure durations, incubation conditions, and clinical acceptance criteria
//   remain external protocol controls and are not encoded here.

const OUTPUT_PREFIX: &str = "closed_cleanroom_recovery_particle_count_settle_plate_station";
const GEOMETRY_REVISION: &str = "closed-cleanroom-recovery-v1";
const REPRODUCIBILITY_MODE: &str = "deterministic-parametric-mm-no-randomness";
const COORDINATE_SYSTEM: &str = "millimeters-deck-center-origin-z-up";
const CYLINDER_SEGMENTS_STANDARD: u32 = 48;

const OUTPUTS: [&str; 11] = [
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_base_recovery_deck.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_settle_contact_plate_cassette_nests.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_particle_counter_probe_dock.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_timed_exposure_token_lanes.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_pressure_cascade_event_markers.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_transfer_door_event_coupon_rail.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_recovery_state_status_gates.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_barcode_custody_surfaces.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_evidence_camera_bridge.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_robot_service_keepouts.stl",
    "output/closed_cleanroom_recovery_particle_count_settle_plate_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "settle_contact_plate_cassette_nests",
    "particle_counter_probe_dock",
    "timed_exposure_token_lanes",
    "pressure_cascade_event_markers",
    "transfer_door_event_coupon_rail",
    "recovery_state_status_gates",
    "barcode_custody_surfaces",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "assembly",
];

const DECK_X: f64 = 1560.0;
const DECK_Y: f64 = 930.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const DECK_MOUNT_HOLES: usize = 8;
const DATUM_TARGETS: usize = 4;

const PLATE_NEST_X: f64 = 400.0;
const PLATE_NEST_Y: f64 = 230.0;
const PLATE_NEST_Z: f64 = 56.0;
const PLATE_NEST_POS: (f64, f64) = (-525.0, 260.0);
const SETTLE_PLATE_CASSETTE_NESTS: usize = 4;
const CONTACT_PLATE_CASSETTE_NESTS: usize = 6;
const SETTLE_CASSETTE_CLEARANCE_X: f64 = 76.0;
const SETTLE_CASSETTE_CLEARANCE_Y: f64 = 82.0;
const CONTACT_CASSETTE_CLEARANCE_X: f64 = 62.0;
const CONTACT_CASSETTE_CLEARANCE_Y: f64 = 42.0;
const PLATE_LID_PARKS: usize = SETTLE_PLATE_CASSETTE_NESTS;

const PROBE_DOCK_X: f64 = 330.0;
const PROBE_DOCK_Y: f64 = 230.0;
const PROBE_DOCK_Z: f64 = 70.0;
const PROBE_DOCK_POS: (f64, f64) = (-95.0, 260.0);
const PROBE_CRADLE_COUNT: usize = 4;
const PROBE_SLEEVE_D: f64 = 19.0;
const PARTICLE_COUNTER_PORTS: usize = 5;
const ZERO_FILTER_CAP_PARKS: usize = 2;
const SAMPLE_TUBE_OD_MM: f64 = 6.35;
const SAMPLE_TUBE_CLEARANCE_D: f64 = SAMPLE_TUBE_OD_MM + 1.4;

const TOKEN_LANE_X: f64 = 360.0;
const TOKEN_LANE_Y: f64 = 170.0;
const TOKEN_LANE_Z: f64 = 32.0;
const TOKEN_LANE_POS: (f64, f64) = (365.0, 260.0);
const EXPOSURE_TOKEN_LANES: usize = 3;
const TOKENS_PER_LANE: usize = 6;
const EXPOSURE_TOKEN_SLOTS: usize = EXPOSURE_TOKEN_LANES * TOKENS_PER_LANE;
const TOKEN_SLOT_X: f64 = 36.0;
const TOKEN_SLOT_Y: f64 = 24.0;
const TOKEN_PITCH_X: f64 = 48.0;
const TOKEN_LANE_PITCH_Y: f64 = 46.0;

const PRESSURE_MARKER_X: f64 = 400.0;
const PRESSURE_MARKER_Y: f64 = 190.0;
const PRESSURE_MARKER_Z: f64 = 42.0;
const PRESSURE_MARKER_POS: (f64, f64) = (-525.0, 0.0);
const CASCADE_ZONE_MARKERS: usize = 4;
const EVENT_CLASSES: usize = 3;
const PRESSURE_CASCADE_EVENT_MARKERS: usize = CASCADE_ZONE_MARKERS * EVENT_CLASSES;
const CASCADE_DIRECTION_TABS: usize = CASCADE_ZONE_MARKERS - 1;

const DOOR_COUPON_X: f64 = 330.0;
const DOOR_COUPON_Y: f64 = 190.0;
const DOOR_COUPON_Z: f64 = 40.0;
const DOOR_COUPON_POS: (f64, f64) = (-95.0, 0.0);
const TRANSFER_DOOR_EVENT_COUPONS: usize = 10;
const TRANSFER_COUPON_X: f64 = 42.0;
const TRANSFER_COUPON_Y: f64 = 28.0;
const DOOR_EVENT_LATCH_SHIMS: usize = 4;
const TRANSFER_DOOR_LEAF_MARKERS: usize = 2;

const STATUS_GATE_X: f64 = 330.0;
const STATUS_GATE_Y: f64 = 230.0;
const STATUS_GATE_Z: f64 = 60.0;
const STATUS_GATE_POS: (f64, f64) = (-95.0, -270.0);
const RECOVERY_STATUS_LANES: usize = 3;
const STATUS_GATES_PER_LANE: usize = 4;
const STATUS_GATE_SLOTS: usize = RECOVERY_STATUS_LANES * STATUS_GATES_PER_LANE;
const STATUS_GATE_SLOT_X: f64 = 56.0;
const STATUS_GATE_SLOT_Y: f64 = 36.0;

const CUSTODY_PANEL_X: f64 = 360.0;
const CUSTODY_PANEL_Y: f64 = 190.0;
const CUSTODY_PANEL_Z: f64 = 12.0;
const CUSTODY_PANEL_POS: (f64, f64) = (365.0, 0.0);
const BARCODE_LANDS: usize = 12;
const CUSTODY_SEAL_LANDS: usize = 6;
const RUN_RECORD_CARD_LANDS: usize = 3;
const BARCODE_LAND_X: f64 = 82.0;
const BARCODE_LAND_Y: f64 = 23.0;

const BRIDGE_SPAN_X: f64 = 1310.0;
const BRIDGE_Y: f64 = 176.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_POS: (f64, f64) = (0.0, 230.0);
const BRIDGE_UNDERSIDE_Z: f64 = 245.0;
const BRIDGE_POST_X: f64 = 34.0;
const CAMERA_PODS: usize = 5;
const EVIDENCE_LIGHT_BARS: usize = 2;

const FRONT_ROBOT_APPROACH: f64 = 430.0;
const REAR_SERVICE_ACCESS: f64 = 300.0;
const RIGHT_TRANSFER_DOOR_SERVICE: f64 = 260.0;
const ROBOT_Z_CLEARANCE: f64 = 340.0;
const KEEP_OUT_STRIP_Z: f64 = 8.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = DECK_X / 2.0 - RIM_W - 14.0;
        let half_y = DECK_Y / 2.0 - RIM_W - 14.0;
        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
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

    let base = base_recovery_deck();
    export(OUTPUTS[0], &base);

    let plate_nests = settle_contact_plate_cassette_nests();
    export(OUTPUTS[1], &plate_nests);

    let probe_dock = particle_counter_probe_dock();
    export(OUTPUTS[2], &probe_dock);

    let token_lanes = timed_exposure_token_lanes();
    export(OUTPUTS[3], &token_lanes);

    let pressure_markers = pressure_cascade_event_markers();
    export(OUTPUTS[4], &pressure_markers);

    let transfer_rail = transfer_door_event_coupon_rail();
    export(OUTPUTS[5], &transfer_rail);

    let status_gates = recovery_state_status_gates();
    export(OUTPUTS[6], &status_gates);

    let custody = barcode_custody_surfaces();
    export(OUTPUTS[7], &custody);

    let camera_bridge = evidence_camera_bridge();
    export(OUTPUTS[8], &camera_bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + plate_nests.translate(PLATE_NEST_POS.0, PLATE_NEST_POS.1, insert_z(PLATE_NEST_Z))
        + probe_dock.translate(PROBE_DOCK_POS.0, PROBE_DOCK_POS.1, insert_z(PROBE_DOCK_Z))
        + token_lanes.translate(TOKEN_LANE_POS.0, TOKEN_LANE_POS.1, insert_z(TOKEN_LANE_Z))
        + pressure_markers.translate(
            PRESSURE_MARKER_POS.0,
            PRESSURE_MARKER_POS.1,
            insert_z(PRESSURE_MARKER_Z),
        )
        + transfer_rail.translate(
            DOOR_COUPON_POS.0,
            DOOR_COUPON_POS.1,
            insert_z(DOOR_COUPON_Z),
        )
        + status_gates.translate(
            STATUS_GATE_POS.0,
            STATUS_GATE_POS.1,
            insert_z(STATUS_GATE_Z),
        )
        + custody.translate(
            CUSTODY_PANEL_POS.0,
            CUSTODY_PANEL_POS.1,
            insert_z(CUSTODY_PANEL_Z),
        )
        + camera_bridge
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cleanroom recovery particle-count / settle-plate station:");
    println!("  Revision:                   {GEOMETRY_REVISION} ({REPRODUCIBILITY_MODE})");
    println!("  Coordinate system:          {COORDINATE_SYSTEM}");
    println!("  Output prefix:              {OUTPUT_PREFIX}");
    println!(
        "  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm recovery deck with {DECK_MOUNT_HOLES} mount holes and {DATUM_TARGETS} datum targets"
    );
    println!(
        "  Plate evidence:             {SETTLE_PLATE_CASSETTE_NESTS} settle cassette nests, {CONTACT_PLATE_CASSETTE_NESTS} contact cassette nests, and {PLATE_LID_PARKS} lid/cassette parks"
    );
    println!(
        "  Particle counter dock:      {PROBE_CRADLE_COUNT} probe cradles, {PARTICLE_COUNTER_PORTS} inlet/zero/return ports, {ZERO_FILTER_CAP_PARKS} zero-filter cap parks, {SAMPLE_TUBE_OD_MM:.2}mm sample tube OD assumption"
    );
    println!(
        "  Event recovery tracking:    {EXPOSURE_TOKEN_LANES} timed token lanes with {EXPOSURE_TOKEN_SLOTS} slots, {PRESSURE_CASCADE_EVENT_MARKERS} pressure-cascade event markers, {TRANSFER_DOOR_EVENT_COUPONS} transfer-door coupons"
    );
    println!(
        "  Custody/status/imaging:     {STATUS_GATE_SLOTS} status-gate positions, {BARCODE_LANDS} barcode lands, {CUSTODY_SEAL_LANDS} custody seal lands, {CAMERA_PODS} evidence camera pods"
    );
    println!(
        "  Keepouts:                   {FRONT_ROBOT_APPROACH:.0}mm front robot approach, {REAR_SERVICE_ACCESS:.0}mm rear service, {RIGHT_TRANSFER_DOOR_SERVICE:.0}mm transfer-door side service, {ROBOT_Z_CLEARANCE:.0}mm Z clearance"
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
            rect.fits_inside_deck(),
            "{} exceeds station deck envelope",
            rect.name
        );
    }

    let rects = layout_rects();
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }

    assert!(SETTLE_CASSETTE_CLEARANCE_X > 70.0);
    assert!(CONTACT_CASSETTE_CLEARANCE_X > 55.0);
    assert!(SAMPLE_TUBE_CLEARANCE_D > SAMPLE_TUBE_OD_MM);
    assert!(BRIDGE_UNDERSIDE_Z > tallest_deck_module_z() + 120.0);
    assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
    assert_eq!(PRESSURE_CASCADE_EVENT_MARKERS, 12);
    assert_eq!(STATUS_GATE_SLOTS, 12);
}

fn layout_rects() -> [Rect; 7] {
    [
        rect(
            "settle_contact_plate_cassette_nests",
            PLATE_NEST_POS,
            PLATE_NEST_X,
            PLATE_NEST_Y,
        ),
        rect(
            "particle_counter_probe_dock",
            PROBE_DOCK_POS,
            PROBE_DOCK_X,
            PROBE_DOCK_Y,
        ),
        rect(
            "timed_exposure_token_lanes",
            TOKEN_LANE_POS,
            TOKEN_LANE_X,
            TOKEN_LANE_Y,
        ),
        rect(
            "pressure_cascade_event_markers",
            PRESSURE_MARKER_POS,
            PRESSURE_MARKER_X,
            PRESSURE_MARKER_Y,
        ),
        rect(
            "transfer_door_event_coupon_rail",
            DOOR_COUPON_POS,
            DOOR_COUPON_X,
            DOOR_COUPON_Y,
        ),
        rect(
            "recovery_state_status_gates",
            STATUS_GATE_POS,
            STATUS_GATE_X,
            STATUS_GATE_Y,
        ),
        rect(
            "barcode_custody_surfaces",
            CUSTODY_PANEL_POS,
            CUSTODY_PANEL_X,
            CUSTODY_PANEL_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn tallest_deck_module_z() -> f64 {
    [
        PLATE_NEST_Z,
        PROBE_DOCK_Z,
        TOKEN_LANE_Z,
        PRESSURE_MARKER_Z,
        DOOR_COUPON_Z,
        STATUS_GATE_Z,
        CUSTODY_PANEL_Z,
    ]
    .into_iter()
    .fold(0.0, f64::max)
}

fn base_recovery_deck() -> Part {
    let deck = centered_cube(
        "cleanroom_recovery_particle_count_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let shallow_pan = centered_cube(
        "cleanroom_recovery_particle_count_deck_recessed_cleanable_pan",
        DECK_X - 2.0 * (RIM_W + 46.0),
        DECK_Y - 2.0 * (RIM_W + 44.0),
        8.0,
    )
    .translate(0.0, 0.0, DECK_Z - 4.0);
    let front_drain = centered_cube(
        "cleanroom_recovery_particle_count_front_wipe_drain_channel",
        DECK_X - 150.0,
        24.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 70.0, DECK_Z - 4.0);

    deck - shallow_pan - front_drain - insert_sockets() - deck_mount_holes()
        + perimeter_rim()
        + datum_targets()
        + clean_used_zone_spines()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("cleanroom_recovery_insert_registration_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("cleanroom_recovery_socket_{}", rect.name),
                rect.x + 16.0,
                rect.y + 16.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("cleanroom_recovery_deck_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -DECK_Y / 2.0 + 56.0),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-DECK_X / 2.0 + 56.0, 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cleanroom_recovery_m6_mount_hole_{i}"),
                3.4,
                DECK_Z + 3.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube("cleanroom_recovery_front_spill_rim", DECK_X, RIM_W, RIM_Z)
        .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube("cleanroom_recovery_rear_spill_rim", DECK_X, RIM_W, RIM_Z).translate(
        0.0,
        DECK_Y / 2.0 - RIM_W / 2.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let left = centered_cube("cleanroom_recovery_left_spill_rim", RIM_W, DECK_Y, RIM_Z).translate(
        -DECK_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("cleanroom_recovery_right_spill_rim", RIM_W, DECK_Y, RIM_Z)
        .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("cleanroom_recovery_camera_robot_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 112.0, DECK_Y / 2.0 - 112.0),
        (DECK_X / 2.0 - 112.0, DECK_Y / 2.0 - 112.0),
        (-DECK_X / 2.0 + 112.0, -DECK_Y / 2.0 + 112.0),
        (DECK_X / 2.0 - 112.0, -DECK_Y / 2.0 + 112.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("cleanroom_recovery_datum_target_outer_ring_{i}"),
                18.0,
                3.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .translate(x, y, DECK_Z + 1.5)
            - centered_cylinder(
                format!("cleanroom_recovery_datum_target_inner_dot_{i}"),
                7.0,
                4.0,
                32,
            )
            .translate(x, y, DECK_Z + 1.5);
    }
    targets
}

fn clean_used_zone_spines() -> Part {
    let clean_to_exposed = centered_cube(
        "cleanroom_recovery_clean_to_exposed_zone_spine",
        DECK_X - 170.0,
        8.0,
        20.0,
    )
    .translate(0.0, 132.0, DECK_Z + 10.0);
    let exposed_to_recovered = centered_cube(
        "cleanroom_recovery_exposed_to_recovered_zone_spine",
        DECK_X - 170.0,
        8.0,
        20.0,
    )
    .translate(0.0, -132.0, DECK_Z + 10.0);
    clean_to_exposed + exposed_to_recovered
}

fn settle_contact_plate_cassette_nests() -> Part {
    let tray = centered_cube(
        "cleanroom_recovery_settle_contact_cassette_nest_tray",
        PLATE_NEST_X,
        PLATE_NEST_Y,
        PLATE_NEST_Z,
    );
    let inner_wipe_pan = centered_cube(
        "cleanroom_recovery_plate_nest_wipeable_inner_pan_cut",
        PLATE_NEST_X - 36.0,
        PLATE_NEST_Y - 34.0,
        PLATE_NEST_Z + 2.0,
    )
    .translate(0.0, 0.0, 10.0);

    tray - inner_wipe_pan - settle_cassette_clearances() - contact_cassette_clearances()
        + plate_nest_index_ribs()
        + settle_lid_parks()
        + plate_cassette_gate_tabs()
}

fn settle_cassette_clearances() -> Part {
    let mut clearances = Part::empty("cleanroom_recovery_settle_plate_cassette_clearances");
    for i in 0..SETTLE_PLATE_CASSETTE_NESTS {
        let x = (i as f64 - (SETTLE_PLATE_CASSETTE_NESTS as f64 - 1.0) / 2.0) * 88.0;
        clearances = clearances
            + centered_cube(
                format!("cleanroom_recovery_settle_plate_cassette_nest_clearance_{i}"),
                SETTLE_CASSETTE_CLEARANCE_X,
                SETTLE_CASSETTE_CLEARANCE_Y,
                PLATE_NEST_Z + 3.0,
            )
            .translate(x, 48.0, 0.0);
    }
    clearances
}

fn contact_cassette_clearances() -> Part {
    let mut clearances = Part::empty("cleanroom_recovery_contact_plate_cassette_clearances");
    for i in 0..CONTACT_PLATE_CASSETTE_NESTS {
        let col = i % 3;
        let row = i / 3;
        let x = (col as f64 - 1.0) * 96.0;
        let y = -62.0 + row as f64 * 48.0;
        clearances = clearances
            + centered_cube(
                format!("cleanroom_recovery_contact_plate_cassette_nest_clearance_{i}"),
                CONTACT_CASSETTE_CLEARANCE_X,
                CONTACT_CASSETTE_CLEARANCE_Y,
                PLATE_NEST_Z + 3.0,
            )
            .translate(x, y, 0.0);
    }
    clearances
}

fn plate_nest_index_ribs() -> Part {
    let settle_center_rib = centered_cube(
        "cleanroom_recovery_settle_cassette_index_rib",
        PLATE_NEST_X - 52.0,
        6.0,
        18.0,
    )
    .translate(0.0, 6.0, PLATE_NEST_Z / 2.0 + 9.0);
    let contact_row_rib = centered_cube(
        "cleanroom_recovery_contact_cassette_row_index_rib",
        PLATE_NEST_X - 72.0,
        5.0,
        14.0,
    )
    .translate(0.0, -38.0, PLATE_NEST_Z / 2.0 + 7.0);
    let clean_dirty_divider = centered_cube(
        "cleanroom_recovery_plate_cassette_clean_exposed_divider",
        8.0,
        PLATE_NEST_Y - 44.0,
        24.0,
    )
    .translate(-PLATE_NEST_X / 2.0 + 32.0, 0.0, PLATE_NEST_Z / 2.0 + 12.0);
    settle_center_rib + contact_row_rib + clean_dirty_divider
}

fn settle_lid_parks() -> Part {
    let mut parks = Part::empty("cleanroom_recovery_settle_lid_parking_lips");
    for i in 0..PLATE_LID_PARKS {
        let x = (i as f64 - (PLATE_LID_PARKS as f64 - 1.0) / 2.0) * 88.0;
        parks = parks
            + centered_cube(
                format!("cleanroom_recovery_settle_lid_park_lip_{i}"),
                SETTLE_CASSETTE_CLEARANCE_X + 10.0,
                7.0,
                8.0,
            )
            .translate(x, 96.0, PLATE_NEST_Z / 2.0 + 4.0);
    }
    parks
}

fn plate_cassette_gate_tabs() -> Part {
    let mut tabs = Part::empty("cleanroom_recovery_plate_cassette_gate_tabs");
    for i in 0..SETTLE_PLATE_CASSETTE_NESTS {
        let x = (i as f64 - (SETTLE_PLATE_CASSETTE_NESTS as f64 - 1.0) / 2.0) * 88.0;
        tabs = tabs
            + centered_cube(
                format!("cleanroom_recovery_settle_cassette_gate_tab_{i}"),
                18.0,
                30.0,
                32.0,
            )
            .translate(x, -2.0, PLATE_NEST_Z / 2.0 + 16.0);
    }
    tabs
}

fn particle_counter_probe_dock() -> Part {
    let body = centered_cube(
        "cleanroom_recovery_particle_counter_probe_dock_body",
        PROBE_DOCK_X,
        PROBE_DOCK_Y,
        PROBE_DOCK_Z,
    );
    let upper_channel = centered_cube(
        "cleanroom_recovery_probe_dock_longitudinal_wipe_channel",
        PROBE_DOCK_X - 48.0,
        58.0,
        PROBE_DOCK_Z + 3.0,
    )
    .translate(0.0, 30.0, 0.0);
    let cable_trough = centered_cube(
        "cleanroom_recovery_probe_dock_sample_tube_trough",
        PROBE_DOCK_X - 72.0,
        24.0,
        PROBE_DOCK_Z + 3.0,
    )
    .translate(0.0, -PROBE_DOCK_Y / 2.0 + 44.0, 0.0);

    body - upper_channel - cable_trough - probe_sleeve_cuts() - particle_port_cuts()
        + probe_sleeve_collars()
        + zero_filter_cap_parks()
        + probe_dock_keyed_datums()
}

fn probe_sleeve_cuts() -> Part {
    let mut cuts = Part::empty("cleanroom_recovery_probe_sleeve_axis_cuts");
    for i in 0..PROBE_CRADLE_COUNT {
        let x = (i as f64 - (PROBE_CRADLE_COUNT as f64 - 1.0) / 2.0) * 66.0;
        cuts = cuts
            + centered_cylinder(
                format!("cleanroom_recovery_probe_sleeve_clearance_{i}"),
                PROBE_SLEEVE_D / 2.0,
                PROBE_DOCK_Y + 12.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 30.0, 2.0);
    }
    cuts
}

fn probe_sleeve_collars() -> Part {
    let mut collars = Part::empty("cleanroom_recovery_probe_sleeve_collars");
    for i in 0..PROBE_CRADLE_COUNT {
        let x = (i as f64 - (PROBE_CRADLE_COUNT as f64 - 1.0) / 2.0) * 66.0;
        collars = collars
            + centered_cylinder(
                format!("cleanroom_recovery_probe_sleeve_front_collar_{i}"),
                18.0,
                8.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -PROBE_DOCK_Y / 2.0 + 20.0, 2.0)
            + centered_cylinder(
                format!("cleanroom_recovery_probe_sleeve_rear_collar_{i}"),
                18.0,
                8.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PROBE_DOCK_Y / 2.0 - 20.0, 2.0);
    }
    collars
}

fn particle_port_cuts() -> Part {
    let mut ports = Part::empty("cleanroom_recovery_particle_counter_port_cuts");
    for i in 0..PARTICLE_COUNTER_PORTS {
        let x = (i as f64 - (PARTICLE_COUNTER_PORTS as f64 - 1.0) / 2.0) * 48.0;
        ports = ports
            + centered_cylinder(
                format!("cleanroom_recovery_particle_counter_sample_port_{i}"),
                SAMPLE_TUBE_CLEARANCE_D / 2.0,
                PROBE_DOCK_Z + 5.0,
                32,
            )
            .translate(x, -64.0, 0.0);
    }
    ports
}

fn zero_filter_cap_parks() -> Part {
    let mut parks = Part::empty("cleanroom_recovery_zero_filter_cap_parks");
    for i in 0..ZERO_FILTER_CAP_PARKS {
        let x = -118.0 + i as f64 * 236.0;
        parks = parks
            + centered_cylinder(
                format!("cleanroom_recovery_zero_filter_cap_park_lip_{i}"),
                23.0,
                7.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .translate(x, -78.0, PROBE_DOCK_Z / 2.0 + 3.5)
            - centered_cylinder(
                format!("cleanroom_recovery_zero_filter_cap_park_clearance_{i}"),
                17.0,
                8.0,
                CYLINDER_SEGMENTS_STANDARD,
            )
            .translate(x, -78.0, PROBE_DOCK_Z / 2.0 + 3.5);
    }
    parks
}

fn probe_dock_keyed_datums() -> Part {
    let left = centered_cube(
        "cleanroom_recovery_probe_dock_left_keyed_datum",
        30.0,
        16.0,
        18.0,
    )
    .translate(-PROBE_DOCK_X / 2.0 + 32.0, 84.0, PROBE_DOCK_Z / 2.0 + 9.0);
    let right = centered_cube(
        "cleanroom_recovery_probe_dock_right_keyed_datum",
        18.0,
        30.0,
        18.0,
    )
    .translate(PROBE_DOCK_X / 2.0 - 32.0, 84.0, PROBE_DOCK_Z / 2.0 + 9.0);
    left + right
}

fn timed_exposure_token_lanes() -> Part {
    let tray = centered_cube(
        "cleanroom_recovery_timed_exposure_token_lane_tray",
        TOKEN_LANE_X,
        TOKEN_LANE_Y,
        TOKEN_LANE_Z,
    );
    tray - exposure_token_slot_cuts() + exposure_lane_dividers() + exposure_start_finish_gates()
}

fn exposure_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("cleanroom_recovery_exposure_token_slot_cuts");
    for lane in 0..EXPOSURE_TOKEN_LANES {
        for slot in 0..TOKENS_PER_LANE {
            let x = (slot as f64 - (TOKENS_PER_LANE as f64 - 1.0) / 2.0) * TOKEN_PITCH_X;
            let y = (lane as f64 - (EXPOSURE_TOKEN_LANES as f64 - 1.0) / 2.0) * TOKEN_LANE_PITCH_Y;
            cuts = cuts
                + centered_cube(
                    format!("cleanroom_recovery_exposure_lane_{lane}_token_slot_{slot}"),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_LANE_Z + 3.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn exposure_lane_dividers() -> Part {
    let mut dividers = Part::empty("cleanroom_recovery_exposure_token_lane_dividers");
    for gap in 0..(EXPOSURE_TOKEN_LANES - 1) {
        let y = -TOKEN_LANE_PITCH_Y / 2.0 + gap as f64 * TOKEN_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("cleanroom_recovery_exposure_lane_divider_{gap}"),
                TOKEN_LANE_X - 44.0,
                6.0,
                18.0,
            )
            .translate(0.0, y, TOKEN_LANE_Z / 2.0 + 9.0);
    }
    dividers
}

fn exposure_start_finish_gates() -> Part {
    let start = centered_cube(
        "cleanroom_recovery_exposure_token_start_gate",
        12.0,
        TOKEN_LANE_Y - 38.0,
        28.0,
    )
    .translate(-TOKEN_LANE_X / 2.0 + 28.0, 0.0, TOKEN_LANE_Z / 2.0 + 14.0);
    let finish = centered_cube(
        "cleanroom_recovery_exposure_token_finish_gate",
        12.0,
        TOKEN_LANE_Y - 38.0,
        28.0,
    )
    .translate(TOKEN_LANE_X / 2.0 - 28.0, 0.0, TOKEN_LANE_Z / 2.0 + 14.0);
    start + finish
}

fn pressure_cascade_event_markers() -> Part {
    let panel = centered_cube(
        "cleanroom_recovery_pressure_cascade_event_marker_panel",
        PRESSURE_MARKER_X,
        PRESSURE_MARKER_Y,
        PRESSURE_MARKER_Z,
    );
    panel - pressure_event_marker_cuts()
        + cascade_zone_steps()
        + cascade_direction_tabs()
        + event_class_separator_ribs()
}

fn pressure_event_marker_cuts() -> Part {
    let mut cuts = Part::empty("cleanroom_recovery_pressure_cascade_event_marker_cuts");
    for zone in 0..CASCADE_ZONE_MARKERS {
        for event_class in 0..EVENT_CLASSES {
            let x = (zone as f64 - (CASCADE_ZONE_MARKERS as f64 - 1.0) / 2.0) * 78.0;
            let y = (event_class as f64 - (EVENT_CLASSES as f64 - 1.0) / 2.0) * 50.0;
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "cleanroom_recovery_pressure_zone_{zone}_event_{event_class}_marker_socket"
                    ),
                    15.0,
                    PRESSURE_MARKER_Z + 3.0,
                    CYLINDER_SEGMENTS_STANDARD,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn cascade_zone_steps() -> Part {
    let mut steps = Part::empty("cleanroom_recovery_pressure_cascade_zone_step_blocks");
    for zone in 0..CASCADE_ZONE_MARKERS {
        let x = (zone as f64 - (CASCADE_ZONE_MARKERS as f64 - 1.0) / 2.0) * 78.0;
        let z = 8.0 + zone as f64 * 5.0;
        steps = steps
            + centered_cube(
                format!("cleanroom_recovery_pressure_cascade_zone_step_{zone}"),
                44.0,
                14.0,
                z,
            )
            .translate(
                x,
                PRESSURE_MARKER_Y / 2.0 - 24.0,
                PRESSURE_MARKER_Z / 2.0 + z / 2.0,
            );
    }
    steps
}

fn cascade_direction_tabs() -> Part {
    let mut tabs = Part::empty("cleanroom_recovery_pressure_cascade_direction_tabs");
    for i in 0..CASCADE_DIRECTION_TABS {
        let x = -78.0 + i as f64 * 78.0;
        tabs = tabs
            + centered_cube(
                format!("cleanroom_recovery_pressure_cascade_direction_tab_{i}"),
                34.0,
                12.0,
                14.0,
            )
            .translate(
                x + 39.0,
                -PRESSURE_MARKER_Y / 2.0 + 24.0,
                PRESSURE_MARKER_Z / 2.0 + 7.0,
            );
    }
    tabs
}

fn event_class_separator_ribs() -> Part {
    let mut ribs = Part::empty("cleanroom_recovery_pressure_event_class_separator_ribs");
    for gap in 0..(EVENT_CLASSES - 1) {
        let y = -25.0 + gap as f64 * 50.0;
        ribs = ribs
            + centered_cube(
                format!("cleanroom_recovery_pressure_event_class_separator_{gap}"),
                PRESSURE_MARKER_X - 48.0,
                5.0,
                12.0,
            )
            .translate(0.0, y, PRESSURE_MARKER_Z / 2.0 + 6.0);
    }
    ribs
}

fn transfer_door_event_coupon_rail() -> Part {
    let rail = centered_cube(
        "cleanroom_recovery_transfer_door_event_coupon_rail_body",
        DOOR_COUPON_X,
        DOOR_COUPON_Y,
        DOOR_COUPON_Z,
    );
    let coupon_channel = centered_cube(
        "cleanroom_recovery_transfer_door_coupon_return_channel_cut",
        DOOR_COUPON_X - 44.0,
        42.0,
        DOOR_COUPON_Z + 3.0,
    )
    .translate(0.0, -DOOR_COUPON_Y / 2.0 + 40.0, 0.0);

    rail - coupon_channel - transfer_coupon_slot_cuts()
        + transfer_door_leaf_markers()
        + transfer_coupon_latch_shims()
        + transfer_coupon_end_stops()
}

fn transfer_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("cleanroom_recovery_transfer_door_event_coupon_slot_cuts");
    for i in 0..TRANSFER_DOOR_EVENT_COUPONS {
        let x = (i as f64 - (TRANSFER_DOOR_EVENT_COUPONS as f64 - 1.0) / 2.0) * 28.0;
        cuts = cuts
            + centered_cube(
                format!("cleanroom_recovery_transfer_door_event_coupon_slot_{i}"),
                TRANSFER_COUPON_X,
                TRANSFER_COUPON_Y,
                DOOR_COUPON_Z + 3.0,
            )
            .translate(x, 30.0, 0.0);
    }
    cuts
}

fn transfer_door_leaf_markers() -> Part {
    let mut markers = Part::empty("cleanroom_recovery_transfer_door_leaf_markers");
    for i in 0..TRANSFER_DOOR_LEAF_MARKERS {
        let x = -64.0 + i as f64 * 128.0;
        markers = markers
            + centered_cube(
                format!("cleanroom_recovery_transfer_door_leaf_marker_{i}"),
                82.0,
                12.0,
                52.0,
            )
            .translate(x, -DOOR_COUPON_Y / 2.0 + 20.0, DOOR_COUPON_Z / 2.0 + 26.0)
            + centered_cylinder(
                format!("cleanroom_recovery_transfer_door_hinge_pin_marker_{i}"),
                7.0,
                74.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x - 46.0,
                -DOOR_COUPON_Y / 2.0 + 20.0,
                DOOR_COUPON_Z / 2.0 + 26.0,
            );
    }
    markers
}

fn transfer_coupon_latch_shims() -> Part {
    let mut shims = Part::empty("cleanroom_recovery_transfer_door_latch_event_shims");
    for i in 0..DOOR_EVENT_LATCH_SHIMS {
        let x = (i as f64 - (DOOR_EVENT_LATCH_SHIMS as f64 - 1.0) / 2.0) * 58.0;
        shims = shims
            + centered_cube(
                format!("cleanroom_recovery_transfer_door_latch_event_shim_{i}"),
                36.0,
                13.0,
                9.0,
            )
            .translate(x, DOOR_COUPON_Y / 2.0 - 22.0, DOOR_COUPON_Z / 2.0 + 4.5);
    }
    shims
}

fn transfer_coupon_end_stops() -> Part {
    let left = centered_cube(
        "cleanroom_recovery_transfer_coupon_left_end_stop",
        12.0,
        DOOR_COUPON_Y - 42.0,
        28.0,
    )
    .translate(-DOOR_COUPON_X / 2.0 + 24.0, 0.0, DOOR_COUPON_Z / 2.0 + 14.0);
    let right = centered_cube(
        "cleanroom_recovery_transfer_coupon_right_end_stop",
        12.0,
        DOOR_COUPON_Y - 42.0,
        28.0,
    )
    .translate(DOOR_COUPON_X / 2.0 - 24.0, 0.0, DOOR_COUPON_Z / 2.0 + 14.0);
    left + right
}

fn recovery_state_status_gates() -> Part {
    let tray = centered_cube(
        "cleanroom_recovery_state_status_gate_tray",
        STATUS_GATE_X,
        STATUS_GATE_Y,
        STATUS_GATE_Z,
    );
    tray - recovery_status_slot_cuts() + recovery_status_lane_gates() + status_gate_lock_bar()
}

fn recovery_status_slot_cuts() -> Part {
    let mut cuts = Part::empty("cleanroom_recovery_state_status_slot_cuts");
    for lane in 0..RECOVERY_STATUS_LANES {
        for slot in 0..STATUS_GATES_PER_LANE {
            let x = (slot as f64 - (STATUS_GATES_PER_LANE as f64 - 1.0) / 2.0) * 66.0;
            let y = (lane as f64 - (RECOVERY_STATUS_LANES as f64 - 1.0) / 2.0) * 62.0;
            cuts = cuts
                + centered_cube(
                    format!("cleanroom_recovery_status_lane_{lane}_gate_slot_{slot}"),
                    STATUS_GATE_SLOT_X,
                    STATUS_GATE_SLOT_Y,
                    STATUS_GATE_Z + 3.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn recovery_status_lane_gates() -> Part {
    let mut gates = Part::empty("cleanroom_recovery_status_lane_gate_flags");
    for lane in 0..RECOVERY_STATUS_LANES {
        let y = (lane as f64 - (RECOVERY_STATUS_LANES as f64 - 1.0) / 2.0) * 62.0;
        gates = gates
            + centered_cube(
                format!("cleanroom_recovery_status_lane_{lane}_hinged_gate_flag"),
                STATUS_GATE_X - 46.0,
                8.0,
                38.0,
            )
            .translate(0.0, y + 26.0, STATUS_GATE_Z / 2.0 + 19.0);
    }
    gates
}

fn status_gate_lock_bar() -> Part {
    centered_cube(
        "cleanroom_recovery_status_gate_custody_lock_bar",
        STATUS_GATE_X - 54.0,
        12.0,
        34.0,
    )
    .translate(0.0, -STATUS_GATE_Y / 2.0 + 28.0, STATUS_GATE_Z / 2.0 + 17.0)
}

fn barcode_custody_surfaces() -> Part {
    let panel = centered_cube(
        "cleanroom_recovery_barcode_custody_surface_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    panel + barcode_lands() + custody_seal_lands() + run_record_card_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("cleanroom_recovery_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 4;
        let row = i / 4;
        let x = -126.0 + col as f64 * 84.0;
        let y = 54.0 - row as f64 * 34.0;
        lands = lands
            + centered_cube(
                format!("cleanroom_recovery_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + 1.5);
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut lands = Part::empty("cleanroom_recovery_custody_seal_lands");
    for i in 0..CUSTODY_SEAL_LANDS {
        let x = -130.0 + i as f64 * 52.0;
        lands = lands
            + centered_cube(
                format!("cleanroom_recovery_tamper_custody_seal_land_{i}"),
                42.0,
                30.0,
                2.6,
            )
            .translate(x, -64.0, CUSTODY_PANEL_Z / 2.0 + 1.3);
    }
    lands
}

fn run_record_card_lands() -> Part {
    let mut cards = Part::empty("cleanroom_recovery_run_record_card_lands");
    for i in 0..RUN_RECORD_CARD_LANDS {
        let x = -110.0 + i as f64 * 110.0;
        cards = cards
            + centered_cube(
                format!("cleanroom_recovery_run_record_card_land_{i}"),
                86.0,
                28.0,
                2.4,
            )
            .translate(x, CUSTODY_PANEL_Y / 2.0 - 22.0, CUSTODY_PANEL_Z / 2.0 + 1.2);
    }
    cards
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "cleanroom_recovery_evidence_camera_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_SPAN_X / 2.0 + BRIDGE_POST_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "cleanroom_recovery_evidence_camera_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "cleanroom_recovery_evidence_camera_bridge_beam",
        BRIDGE_SPAN_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        DECK_Z + BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    left_post + right_post + beam + evidence_camera_pods() + evidence_light_bars()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("cleanroom_recovery_evidence_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = BRIDGE_POS.0
            + (i as f64 - (CAMERA_PODS as f64 - 1.0) / 2.0) * (BRIDGE_SPAN_X - 270.0)
                / (CAMERA_PODS as f64 - 1.0);
        let pod_z = DECK_Z + BRIDGE_UNDERSIDE_Z - 26.0;
        let lens_y = BRIDGE_POS.1 - BRIDGE_Y / 2.0 - 10.0;
        pods = pods
            + centered_cube(
                format!("cleanroom_recovery_evidence_camera_pod_{i}"),
                62.0,
                44.0,
                38.0,
            )
            .translate(x, BRIDGE_POS.1 - 6.0, pod_z)
            + centered_cylinder(
                format!("cleanroom_recovery_evidence_camera_lens_{i}"),
                11.0,
                16.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, lens_y, pod_z)
            + centered_cylinder(
                format!("cleanroom_recovery_evidence_camera_focus_ring_{i}"),
                17.0,
                5.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, lens_y - 10.0, pod_z);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("cleanroom_recovery_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BARS {
        let side = if i == 0 { -1.0 } else { 1.0 };
        bars = bars
            + centered_cube(
                format!("cleanroom_recovery_evidence_light_bar_{i}"),
                BRIDGE_SPAN_X - 230.0,
                12.0,
                12.0,
            )
            .translate(
                BRIDGE_POS.0,
                BRIDGE_POS.1 + side * 26.0,
                DECK_Z + BRIDGE_UNDERSIDE_Z - 48.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let front_keepout = centered_cube(
        "cleanroom_recovery_front_robot_approach_keepout_strip",
        DECK_X - 150.0,
        22.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        DECK_Z + KEEP_OUT_STRIP_Z / 2.0,
    );
    let rear_keepout = centered_cube(
        "cleanroom_recovery_rear_service_access_keepout_strip",
        DECK_X - 190.0,
        22.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_ACCESS / 2.0,
        DECK_Z + KEEP_OUT_STRIP_Z / 2.0,
    );
    let right_transfer_keepout = centered_cube(
        "cleanroom_recovery_right_transfer_door_service_keepout",
        22.0,
        DECK_Y - 170.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_TRANSFER_DOOR_SERVICE / 2.0,
        0.0,
        DECK_Z + KEEP_OUT_STRIP_Z / 2.0,
    );
    let z_clearance_gauge = centered_cube(
        "cleanroom_recovery_robot_z_clearance_gauge",
        74.0,
        74.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 82.0,
        DECK_Y / 2.0 - 82.0,
        DECK_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    let service_probe_withdrawal = centered_cube(
        "cleanroom_recovery_particle_probe_service_withdrawal_keepout",
        220.0,
        18.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(
        PROBE_DOCK_POS.0,
        DECK_Y / 2.0 + 54.0,
        DECK_Z + KEEP_OUT_STRIP_Z / 2.0,
    );

    front_keepout
        + rear_keepout
        + right_transfer_keepout
        + z_clearance_gauge
        + service_probe_withdrawal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_counts_match_validation_intent() {
        assert_eq!(SETTLE_PLATE_CASSETTE_NESTS, 4);
        assert_eq!(CONTACT_PLATE_CASSETTE_NESTS, 6);
        assert_eq!(PROBE_CRADLE_COUNT, 4);
        assert_eq!(PARTICLE_COUNTER_PORTS, 5);
        assert_eq!(EXPOSURE_TOKEN_SLOTS, 18);
        assert_eq!(PRESSURE_CASCADE_EVENT_MARKERS, 12);
        assert_eq!(TRANSFER_DOOR_EVENT_COUPONS, 10);
        assert_eq!(STATUS_GATE_SLOTS, 12);
        assert_eq!(
            BARCODE_LANDS + CUSTODY_SEAL_LANDS + RUN_RECORD_CARD_LANDS,
            21
        );
        assert_eq!(CAMERA_PODS, 5);
    }

    #[test]
    fn station_bounds_and_modules_do_not_overlap() {
        assert_layout_constraints();
    }

    #[test]
    fn bridge_and_keepouts_clear_service_paths() {
        assert!(BRIDGE_UNDERSIDE_Z > tallest_deck_module_z() + 120.0);
        assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(REAR_SERVICE_ACCESS >= 280.0);
        assert!(RIGHT_TRANSFER_DOOR_SERVICE >= 240.0);
        assert_eq!(EVIDENCE_LIGHT_BARS, 2);
    }

    #[test]
    fn output_manifest_is_stable_and_scoped() {
        let expected = [
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_base_recovery_deck.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_settle_contact_plate_cassette_nests.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_particle_counter_probe_dock.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_timed_exposure_token_lanes.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_pressure_cascade_event_markers.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_transfer_door_event_coupon_rail.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_recovery_state_status_gates.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_barcode_custody_surfaces.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_evidence_camera_bridge.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_robot_service_keepouts.stl",
            "output/closed_cleanroom_recovery_particle_count_settle_plate_station_assembly.stl",
        ];
        assert_eq!(OUTPUTS, expected);
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn reproducibility_controls_are_explicit() {
        assert_eq!(GEOMETRY_REVISION, "closed-cleanroom-recovery-v1");
        assert_eq!(
            REPRODUCIBILITY_MODE,
            "deterministic-parametric-mm-no-randomness"
        );
        assert_eq!(COORDINATE_SYSTEM, "millimeters-deck-center-origin-z-up");
        assert_eq!(CYLINDER_SEGMENTS_STANDARD, 48);
        assert_eq!(DATUM_TARGETS, 4);
        assert_eq!(DECK_MOUNT_HOLES, 8);
    }

    #[test]
    fn no_clinical_acceptance_thresholds_are_encoded() {
        assert_eq!(EVENT_CLASSES, 3);
        assert_eq!(CASCADE_ZONE_MARKERS, 4);
        assert_eq!(CASCADE_DIRECTION_TABS, 3);
        assert!(SAMPLE_TUBE_CLEARANCE_D > SAMPLE_TUBE_OD_MM);
    }
}
