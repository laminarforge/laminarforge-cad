use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media feed/harvest bag lot mix-up prevention station.
//
// Design assumptions encoded from research:
// - GS1 pharmaceutical chain-of-custody guidance emphasizes captured identifiers
//   such as GTIN, batch/lot, serial or SSCC, read point, business location, and
//   event time. The scan bridge therefore has separate barcode/RFID lands for
//   feed and harvest bags plus custody-card/event lands, not a single shared
//   scan spot that can hide route ambiguity.
// - BPSA single-use integrity practice treats unpacking, handling,
//   installation, and use as risk-managed steps for bags/assemblies, with leak
//   and visual controls as practical end-user checks. The station provides
//   leak-tray containment, load-cell nests, port/cap custody parks, and
//   quarantine lanes without claiming release criteria.
// - Commercial single-use aseptic connectors use repeatable sterile connection
//   operations, gendered or genderless couplings, validated integrity, and
//   process-designed flow direction/keying. The connector panel uses asymmetric
//   feed/harvest mechanical keys so physically plausible wrong-route attempts
//   are blocked before cap removal.
//
// This is validation fixture CAD only. It does not define a sterile connection
// SOP, electronic batch-record implementation, load-cell electronics, release
// criteria, or bioprocess acceptance limits.

const BIN_NAME: &str = "closed_media_feed_harvest_bag_lot_mixup_prevention_station";
#[cfg(test)]
const OUTPUT_PREFIX: &str = "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_base_leak_tray.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_feed_load_cell_bag_nest.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_harvest_load_cell_bag_nest.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_barcode_rfid_scan_bridge.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_sterile_connector_keying_panel.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_cap_port_custody_parks.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_route_confirmation_token_rail.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_quarantine_release_lanes.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_temperature_logger_pockets.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_leak_drain_splash_guards.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_custody_evidence_panel.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_robot_service_keepouts.stl",
    "output/closed_media_feed_harvest_bag_lot_mixup_prevention_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "separated_feed_zone",
    "separated_harvest_zone",
    "barcode_rfid_scan_bridge",
    "load_cell_bag_nests",
    "sterile_connector_keying",
    "cap_port_custody_parks",
    "route_confirmation_token_rail",
    "quarantine_release_lanes",
    "cold_room_temp_logger_pockets",
    "leak_tray",
    "robot_service_keepouts",
    "chain_of_custody_evidence_panel",
];

#[cfg(test)]
const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_fixture_only",
    "no_sterile_connection_sop",
    "no_electronic_batch_record_implementation",
    "no_load_cell_electronics_design",
    "no_bioprocess_release_criteria",
    "no_microbiological_integrity_claim",
];

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const EDGE_MARGIN: f64 = 16.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MAIN_DRAIN_D: f64 = 18.0;

const FEED_NEST_POS: (f64, f64) = (-455.0, 145.0);
const HARVEST_NEST_POS: (f64, f64) = (455.0, 145.0);
const BAG_NEST_X: f64 = 420.0;
const BAG_NEST_Y: f64 = 310.0;
const BAG_NEST_Z: f64 = 62.0;
const BAG_POCKET_X: f64 = 322.0;
const BAG_POCKET_Y: f64 = 210.0;
const BAG_POCKET_DEPTH: f64 = 24.0;
const LOAD_CELLS_PER_NEST: usize = 4;
const LOAD_CELL_PAD_X: f64 = 72.0;
const LOAD_CELL_PAD_Y: f64 = 52.0;
const BAG_SADDLE_RIBS: usize = 8;
const BAG_DATUM_PINS: usize = 6;
const TUBE_STRAIN_RELIEFS_PER_NEST: usize = 5;
const ZONE_KEY_FINS_PER_NEST: usize = 3;
const FEED_KEY_HEIGHT: f64 = 28.0;
const HARVEST_KEY_HEIGHT: f64 = 42.0;

const SCAN_BRIDGE_POS: (f64, f64) = (0.0, 390.0);
const SCAN_BRIDGE_X: f64 = 620.0;
const SCAN_BRIDGE_Y: f64 = 90.0;
const SCAN_BRIDGE_Z: f64 = 205.0;
const SCAN_POST_W: f64 = 30.0;
const SCAN_BEAM_Z: f64 = 28.0;
const SCAN_UNDERSIDE_CLEARANCE: f64 = 150.0;
const BARCODE_WINDOWS: usize = 4;
const RFID_LANDS: usize = 4;
const SCAN_CONFIRM_LIGHT_PIPS: usize = 8;
const LOT_EVENT_CARD_LANDS: usize = 4;

const CONNECTOR_PANEL_POS: (f64, f64) = (0.0, 85.0);
const CONNECTOR_PANEL_X: f64 = 340.0;
const CONNECTOR_PANEL_Y: f64 = 225.0;
const CONNECTOR_PANEL_Z: f64 = 56.0;
const CONNECTOR_PAIRS: usize = 4;
#[cfg(test)]
const FEED_CONNECTOR_KEYS: usize = CONNECTOR_PAIRS;
#[cfg(test)]
const HARVEST_CONNECTOR_KEYS: usize = CONNECTOR_PAIRS;
const CONNECTOR_PITCH_Y: f64 = 43.0;
const CONNECTOR_SOCKET_D: f64 = 30.0;
const MISROUTE_BLOCKER_FINS: usize = 8;
const MEMBRANE_PULL_TAB_GAUGES: usize = 4;

const CAP_PARKS_POS: (f64, f64) = (-455.0, -225.0);
const CAP_PARKS_X: f64 = 390.0;
const CAP_PARKS_Y: f64 = 185.0;
const CAP_PARKS_Z: f64 = 44.0;
const CAP_WELLS_PER_ZONE: usize = 8;
const PORT_CUPS_PER_ZONE: usize = 6;
const CAP_WELL_D: f64 = 24.0;
const PORT_CUP_D: f64 = 31.0;
const CUSTODY_SEAL_SLOTS: usize = 6;

const TOKEN_RAIL_POS: (f64, f64) = (0.0, -260.0);
const TOKEN_RAIL_X: f64 = 430.0;
const TOKEN_RAIL_Y: f64 = 150.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const TOKEN_LANES: usize = 4;
const TOKENS_PER_LANE: usize = 5;
const ROUTE_TOKEN_SLOTS: usize = TOKEN_LANES * TOKENS_PER_LANE;
const TOKEN_SLOT_X: f64 = 42.0;
const TOKEN_SLOT_Y: f64 = 24.0;
const TOKEN_LANE_PITCH_Y: f64 = 31.0;
const TOKEN_SLOT_PITCH_X: f64 = 62.0;

const LANE_POS: (f64, f64) = (455.0, -225.0);
const LANE_X: f64 = 390.0;
const LANE_Y: f64 = 185.0;
const LANE_Z: f64 = 40.0;
const DISPOSITION_LANES: usize = 3;
const LANE_SLOTS_PER_STATUS: usize = 4;
const QUARANTINE_SLOTS: usize = LANE_SLOTS_PER_STATUS;
const HOLD_SLOTS: usize = LANE_SLOTS_PER_STATUS;
const RELEASE_SLOTS: usize = LANE_SLOTS_PER_STATUS;
const LANE_SLOT_X: f64 = 72.0;
const LANE_SLOT_Y: f64 = 31.0;
const LANE_PITCH_Y: f64 = 47.0;
const LANE_DIVIDER_Z: f64 = 72.0;

const LOGGER_POS: (f64, f64) = (0.0, -404.0);
const LOGGER_X: f64 = 430.0;
const LOGGER_Y: f64 = 82.0;
const LOGGER_Z: f64 = 46.0;
const COLD_LOGGER_POCKETS: usize = 2;
const ROOM_LOGGER_POCKETS: usize = 2;
const LOGGER_POCKETS: usize = COLD_LOGGER_POCKETS + ROOM_LOGGER_POCKETS;
const LOGGER_POCKET_X: f64 = 78.0;
const LOGGER_POCKET_Y: f64 = 42.0;
const LOGGER_POCKET_DEPTH: f64 = 24.0;
const LOGGER_THERMAL_GAP: f64 = 58.0;

const LEAK_GUARD_POS: (f64, f64) = (0.0, -55.0);
const LEAK_GUARD_X: f64 = 1210.0;
const LEAK_GUARD_Y: f64 = 120.0;
const LEAK_GUARD_Z: f64 = 38.0;
const SPLASH_GUARD_POSTS: usize = 6;
const SAMPLE_WELL_COUNT: usize = 6;
const LEAK_TROUGH_DRAINS: usize = 4;

const EVIDENCE_POS: (f64, f64) = (0.0, -105.0);
const EVIDENCE_X: f64 = 330.0;
const EVIDENCE_Y: f64 = 98.0;
const EVIDENCE_Z: f64 = 16.0;
const LOT_CARD_SLOTS: usize = 4;
const EVENT_TIME_LANDS: usize = 3;
const READ_POINT_LANDS: usize = 3;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 905.0;
const KEEP_OUT_RAIL: f64 = 8.0;
const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const LEFT_FEED_LOAD_CLEARANCE: f64 = 250.0;
const RIGHT_HARVEST_LOAD_CLEARANCE: f64 = 250.0;
const OVERHEAD_SCAN_CLEARANCE: f64 = 260.0;
const OVERHEAD_BAG_LIFT_CLEARANCE: f64 = 360.0;
const KEEP_OUT_GROUPS: usize = 6;

const MIN_ZONE_GAP: f64 = 260.0;
const MIN_COMPONENT_GAP: f64 = 24.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BagZone {
    Feed,
    Harvest,
}

impl BagZone {
    fn label(self) -> &'static str {
        match self {
            BagZone::Feed => "feed",
            BagZone::Harvest => "harvest",
        }
    }

    fn sign(self) -> f64 {
        match self {
            BagZone::Feed => -1.0,
            BagZone::Harvest => 1.0,
        }
    }

    fn key_height(self) -> f64 {
        match self {
            BagZone::Feed => FEED_KEY_HEIGHT,
            BagZone::Harvest => HARVEST_KEY_HEIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - EDGE_MARGIN;
        let usable_y = STATION_Y / 2.0 - RIM_W - EDGE_MARGIN;

        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }

    fn horizontal_gap(self, other: Rect) -> f64 {
        let left_a = self.center.0 - self.x / 2.0;
        let right_a = self.center.0 + self.x / 2.0;
        let left_b = other.center.0 - other.x / 2.0;
        let right_b = other.center.0 + other.x / 2.0;

        if right_a < left_b {
            left_b - right_a
        } else if right_b < left_a {
            left_a - right_b
        } else {
            0.0
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let feed_nest = load_cell_bag_nest(BagZone::Feed);
    export(OUTPUTS[1], &feed_nest);

    let harvest_nest = load_cell_bag_nest(BagZone::Harvest);
    export(OUTPUTS[2], &harvest_nest);

    let scan_bridge = barcode_rfid_scan_bridge();
    export(OUTPUTS[3], &scan_bridge);

    let connector_panel = sterile_connector_keying_panel();
    export(OUTPUTS[4], &connector_panel);

    let cap_parks = cap_port_custody_parks();
    export(OUTPUTS[5], &cap_parks);

    let token_rail = route_confirmation_token_rail();
    export(OUTPUTS[6], &token_rail);

    let lanes = quarantine_release_lanes();
    export(OUTPUTS[7], &lanes);

    let loggers = temperature_logger_pockets();
    export(OUTPUTS[8], &loggers);

    let leak_guards = leak_drain_splash_guards();
    export(OUTPUTS[9], &leak_guards);

    let evidence = custody_evidence_panel();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + feed_nest.translate(FEED_NEST_POS.0, FEED_NEST_POS.1, on_deck_z(BAG_NEST_Z))
        + harvest_nest.translate(
            HARVEST_NEST_POS.0,
            HARVEST_NEST_POS.1,
            on_deck_z(BAG_NEST_Z),
        )
        + scan_bridge.translate(
            SCAN_BRIDGE_POS.0,
            SCAN_BRIDGE_POS.1,
            on_deck_z(SCAN_BRIDGE_Z),
        )
        + connector_panel.translate(
            CONNECTOR_PANEL_POS.0,
            CONNECTOR_PANEL_POS.1,
            on_deck_z(CONNECTOR_PANEL_Z),
        )
        + cap_parks.translate(CAP_PARKS_POS.0, CAP_PARKS_POS.1, on_deck_z(CAP_PARKS_Z))
        + token_rail.translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, on_deck_z(TOKEN_RAIL_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, on_deck_z(LANE_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_deck_z(LOGGER_Z))
        + leak_guards.translate(LEAK_GUARD_POS.0, LEAK_GUARD_POS.1, on_deck_z(LEAK_GUARD_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_deck_z(EVIDENCE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_RAIL / 2.0 + 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media feed/harvest bag lot mix-up prevention station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Bag zones:              separated feed and harvest load-cell nests, {LOAD_CELLS_PER_NEST} load-cell pads each, {MIN_ZONE_GAP:.0}mm minimum nominal zone gap"
    );
    println!(
        "  Identity capture:       {BARCODE_WINDOWS} barcode windows, {RFID_LANDS} RFID lands, {LOT_EVENT_CARD_LANDS} lot/event card lands on scan bridge"
    );
    println!(
        "  Connector controls:     {CONNECTOR_PAIRS} keyed connector pairs, {MISROUTE_BLOCKER_FINS} misroute blocker fins, {MEMBRANE_PULL_TAB_GAUGES} membrane pull-tab gauges"
    );
    println!(
        "  Custody controls:       {} cap wells, {} port cups, {CUSTODY_SEAL_SLOTS} seal slots, {ROUTE_TOKEN_SLOTS} route token slots",
        CAP_WELLS_PER_ZONE * 2,
        PORT_CUPS_PER_ZONE * 2
    );
    println!(
        "  Disposition/logging:    {QUARANTINE_SLOTS} quarantine, {HOLD_SLOTS} hold, {RELEASE_SLOTS} release slots; {COLD_LOGGER_POCKETS} cold and {ROOM_LOGGER_POCKETS} room-temperature logger pockets"
    );
    println!(
        "  Keepouts:               front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, left/right bag load {LEFT_FEED_LOAD_CLEARANCE:.0}/{RIGHT_HARVEST_LOAD_CLEARANCE:.0}mm, overhead scan {OVERHEAD_SCAN_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    let components = component_rects();
    for component in components {
        assert!(
            component.fits_inside_station(),
            "{} exceeds station envelope",
            component.name
        );
    }

    for i in 0..components.len() {
        for j in i + 1..components.len() {
            assert!(
                !components[i].overlaps_with_clearance(components[j], MIN_COMPONENT_GAP),
                "{} overlaps {} with required clearance",
                components[i].name,
                components[j].name
            );
        }
    }

    assert!(
        feed_rect().horizontal_gap(harvest_rect()) >= MIN_ZONE_GAP,
        "feed and harvest nests are too close"
    );
}

fn component_rects() -> [Rect; 9] {
    [
        feed_rect(),
        harvest_rect(),
        Rect {
            name: "barcode_rfid_scan_bridge",
            center: SCAN_BRIDGE_POS,
            x: SCAN_BRIDGE_X,
            y: SCAN_BRIDGE_Y,
        },
        Rect {
            name: "sterile_connector_keying_panel",
            center: CONNECTOR_PANEL_POS,
            x: CONNECTOR_PANEL_X,
            y: CONNECTOR_PANEL_Y,
        },
        Rect {
            name: "cap_port_custody_parks",
            center: CAP_PARKS_POS,
            x: CAP_PARKS_X,
            y: CAP_PARKS_Y,
        },
        Rect {
            name: "route_confirmation_token_rail",
            center: TOKEN_RAIL_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Rect {
            name: "quarantine_release_lanes",
            center: LANE_POS,
            x: LANE_X,
            y: LANE_Y,
        },
        Rect {
            name: "temperature_logger_pockets",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "custody_evidence_panel",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

fn feed_rect() -> Rect {
    Rect {
        name: "feed_load_cell_bag_nest",
        center: FEED_NEST_POS,
        x: BAG_NEST_X,
        y: BAG_NEST_Y,
    }
}

fn harvest_rect() -> Rect {
    Rect {
        name: "harvest_load_cell_bag_nest",
        center: HARVEST_NEST_POS,
        x: BAG_NEST_X,
        y: BAG_NEST_Y,
    }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(name("base_leak_tray_deck"), STATION_X, STATION_Y, BASE_Z);
    let tray_recess = centered_cube(
        name("base_leak_tray_recess"),
        STATION_X - 2.0 * (RIM_W + 10.0),
        STATION_Y - 2.0 * (RIM_W + 10.0),
        SOCKET_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 1.0);
    let feed_socket = centered_cube(
        name("base_feed_zone_socket"),
        BAG_NEST_X + 36.0,
        BAG_NEST_Y + 36.0,
        SOCKET_DEPTH + 2.0,
    )
    .translate(
        FEED_NEST_POS.0,
        FEED_NEST_POS.1,
        BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
    );
    let harvest_socket = centered_cube(
        name("base_harvest_zone_socket"),
        BAG_NEST_X + 36.0,
        BAG_NEST_Y + 36.0,
        SOCKET_DEPTH + 2.0,
    )
    .translate(
        HARVEST_NEST_POS.0,
        HARVEST_NEST_POS.1,
        BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
    );
    let connector_socket = centered_cube(
        name("base_connector_panel_socket"),
        CONNECTOR_PANEL_X + 28.0,
        CONNECTOR_PANEL_Y + 28.0,
        SOCKET_DEPTH + 2.0,
    )
    .translate(
        CONNECTOR_PANEL_POS.0,
        CONNECTOR_PANEL_POS.1,
        BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
    );
    let drains = base_drain_ports();

    deck - tray_recess - feed_socket - harvest_socket - connector_socket - drains
        + perimeter_rim()
        + base_zone_divider()
        + base_datum_pucks()
}

fn perimeter_rim() -> Part {
    let front = centered_cube(name("base_front_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("base_rear_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(name("base_left_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(name("base_right_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn base_zone_divider() -> Part {
    let fore_aft = centered_cube(name("base_center_feed_harvest_divider"), 16.0, 570.0, 28.0)
        .translate(0.0, 55.0, BASE_Z / 2.0 + 14.0);
    let scan_gate = centered_cube(
        name("base_scan_bridge_approach_gap_marker"),
        210.0,
        14.0,
        20.0,
    )
    .translate(0.0, 225.0, BASE_Z / 2.0 + 10.0);
    fore_aft + scan_gate
}

fn base_drain_ports() -> Part {
    let mut drains = Part::empty(name("base_leak_tray_drain_ports"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 105.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 105.0, -STATION_Y / 2.0 + 92.0),
        (-STATION_X / 2.0 + 105.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 105.0, STATION_Y / 2.0 - 92.0),
    ]
    .iter()
    .enumerate()
    {
        drains = drains
            + centered_cylinder(
                name(&format!("base_leak_tray_drain_port_{i}")),
                MAIN_DRAIN_D / 2.0,
                BASE_Z + 10.0,
                32,
            )
            .translate(*x, *y, 0.0);
    }
    drains
}

fn base_datum_pucks() -> Part {
    let mut pucks = Part::empty(name("base_datum_mount_pucks"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 70.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 70.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 70.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 70.0),
    ]
    .iter()
    .enumerate()
    {
        let puck = centered_cylinder(name(&format!("base_mount_puck_{i}")), 18.0, 6.0, 32)
            - centered_cylinder(
                name(&format!("base_mount_hole_{i}")),
                MOUNT_HOLE_D / 2.0,
                10.0,
                24,
            );
        pucks = pucks + puck.translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    pucks
}

fn load_cell_bag_nest(zone: BagZone) -> Part {
    let label = zone.label();
    let block = centered_cube(
        name(&format!("{label}_load_cell_bag_nest_block")),
        BAG_NEST_X,
        BAG_NEST_Y,
        BAG_NEST_Z,
    );
    let bag_pocket = centered_cube(
        name(&format!("{label}_bag_recess_cut")),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH,
    )
    .translate(0.0, 0.0, BAG_NEST_Z / 2.0 - BAG_POCKET_DEPTH / 2.0 + 1.0);

    block - bag_pocket - load_cell_reliefs(label)
        + load_cell_pads(label)
        + saddle_ribs(label)
        + datum_pins(label)
        + tubing_strain_relief_comb(zone)
        + zone_key_fins(zone)
        + bag_zone_barcode_land(zone)
}

fn load_cell_reliefs(label: &str) -> Part {
    let mut cuts = Part::empty(name(&format!("{label}_load_cell_relief_cuts")));
    for (i, (x, y)) in load_cell_points().iter().enumerate() {
        cuts = cuts
            + centered_cube(
                name(&format!("{label}_load_cell_relief_cut_{i}")),
                LOAD_CELL_PAD_X + 16.0,
                LOAD_CELL_PAD_Y + 16.0,
                12.0,
            )
            .translate(*x, *y, BAG_NEST_Z / 2.0 - 5.0);
    }
    cuts
}

fn load_cell_pads(label: &str) -> Part {
    let mut pads = Part::empty(name(&format!("{label}_load_cell_pad_set")));
    for (i, (x, y)) in load_cell_points().iter().enumerate() {
        pads = pads
            + centered_cube(
                name(&format!("{label}_load_cell_pad_{i}")),
                LOAD_CELL_PAD_X,
                LOAD_CELL_PAD_Y,
                8.0,
            )
            .translate(*x, *y, BAG_NEST_Z / 2.0 + 4.0);
    }
    pads
}

fn saddle_ribs(label: &str) -> Part {
    let mut ribs = Part::empty(name(&format!("{label}_bag_saddle_ribs")));
    for i in 0..BAG_SADDLE_RIBS {
        let x = centered_index(
            i,
            BAG_SADDLE_RIBS,
            BAG_POCKET_X / (BAG_SADDLE_RIBS as f64 + 1.0),
        );
        ribs = ribs
            + centered_cube(
                name(&format!("{label}_bag_saddle_rib_{i}")),
                9.0,
                186.0,
                12.0,
            )
            .translate(x, 0.0, BAG_NEST_Z / 2.0 + 6.0);
    }
    ribs
}

fn datum_pins(label: &str) -> Part {
    let mut pins = Part::empty(name(&format!("{label}_bag_datum_pins")));
    for i in 0..BAG_DATUM_PINS {
        let x = centered_index(i % 3, 3, 118.0);
        let y = if i < 3 {
            -BAG_POCKET_Y / 2.0 - 20.0
        } else {
            BAG_POCKET_Y / 2.0 + 20.0
        };
        pins = pins
            + centered_cylinder(name(&format!("{label}_bag_datum_pin_{i}")), 7.0, 22.0, 24)
                .translate(x, y, BAG_NEST_Z / 2.0 + 11.0);
    }
    pins
}

fn tubing_strain_relief_comb(zone: BagZone) -> Part {
    let label = zone.label();
    let mut comb = centered_cube(
        name(&format!("{label}_tube_strain_relief_comb_backbone")),
        250.0,
        28.0,
        24.0,
    )
    .translate(0.0, -BAG_NEST_Y / 2.0 + 26.0, BAG_NEST_Z / 2.0 + 12.0);
    for i in 0..TUBE_STRAIN_RELIEFS_PER_NEST {
        let x = centered_index(i, TUBE_STRAIN_RELIEFS_PER_NEST, 46.0);
        let channel = centered_cylinder(
            name(&format!("{label}_tube_strain_relief_channel_{i}")),
            9.0,
            34.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BAG_NEST_Y / 2.0 + 25.0, BAG_NEST_Z / 2.0 + 16.0);
        comb = comb - channel;
    }
    comb
}

fn zone_key_fins(zone: BagZone) -> Part {
    let label = zone.label();
    let mut fins = Part::empty(name(&format!("{label}_zone_key_fins")));
    for i in 0..ZONE_KEY_FINS_PER_NEST {
        let y = centered_index(i, ZONE_KEY_FINS_PER_NEST, 44.0);
        fins = fins
            + centered_cube(
                name(&format!("{label}_zone_key_fin_{i}")),
                18.0,
                26.0,
                zone.key_height(),
            )
            .translate(
                zone.sign() * (BAG_NEST_X / 2.0 - 27.0),
                y,
                BAG_NEST_Z / 2.0 + zone.key_height() / 2.0,
            );
    }
    fins
}

fn bag_zone_barcode_land(zone: BagZone) -> Part {
    let label = zone.label();
    barcode_land(&format!("{label}_bag_zone_2d_code_land"), 112.0, 32.0, 11).translate(
        0.0,
        BAG_NEST_Y / 2.0 - 32.0,
        BAG_NEST_Z / 2.0 + 4.0,
    )
}

fn load_cell_points() -> [(f64, f64); LOAD_CELLS_PER_NEST] {
    [
        (-118.0, -78.0),
        (118.0, -78.0),
        (-118.0, 78.0),
        (118.0, 78.0),
    ]
}

fn barcode_rfid_scan_bridge() -> Part {
    let left_post = centered_cube(
        name("scan_bridge_left_post"),
        SCAN_POST_W,
        SCAN_BRIDGE_Y,
        SCAN_BRIDGE_Z,
    )
    .translate(-SCAN_BRIDGE_X / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        name("scan_bridge_right_post"),
        SCAN_POST_W,
        SCAN_BRIDGE_Y,
        SCAN_BRIDGE_Z,
    )
    .translate(SCAN_BRIDGE_X / 2.0, 0.0, 0.0);
    let beam = centered_cube(
        name("scan_bridge_overhead_beam"),
        SCAN_BRIDGE_X + SCAN_POST_W,
        42.0,
        SCAN_BEAM_Z,
    )
    .translate(0.0, 0.0, SCAN_BRIDGE_Z / 2.0 - SCAN_BEAM_Z / 2.0);
    let underside_gauge = centered_cube(
        name("scan_bridge_underside_clearance_gauge"),
        260.0,
        9.0,
        8.0,
    )
    .translate(
        0.0,
        -SCAN_BRIDGE_Y / 2.0 + 24.0,
        -SCAN_BRIDGE_Z / 2.0 + SCAN_UNDERSIDE_CLEARANCE,
    );

    left_post
        + right_post
        + beam
        + underside_gauge
        + barcode_windows()
        + rfid_lands()
        + scan_confirm_light_pips()
        + lot_event_card_lands()
}

fn barcode_windows() -> Part {
    let mut windows = Part::empty(name("scan_bridge_barcode_windows"));
    for i in 0..BARCODE_WINDOWS {
        let x = centered_index(i, BARCODE_WINDOWS, 132.0);
        windows =
            windows
                + barcode_land(&format!("scan_bridge_barcode_window_{i}"), 78.0, 28.0, 9)
                    .translate(x, SCAN_BRIDGE_Y / 2.0 - 24.0, SCAN_BRIDGE_Z / 2.0 - 48.0);
    }
    windows
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty(name("scan_bridge_rfid_lands"));
    for i in 0..RFID_LANDS {
        let x = centered_index(i, RFID_LANDS, 128.0);
        let land = centered_cube(name(&format!("scan_bridge_rfid_land_{i}")), 74.0, 36.0, 5.0)
            + centered_cube(
                name(&format!("scan_bridge_rfid_antenna_trace_x_{i}")),
                64.0,
                3.0,
                4.0,
            )
            .translate(0.0, 0.0, 4.0)
            + centered_cube(
                name(&format!("scan_bridge_rfid_antenna_trace_y_{i}")),
                3.0,
                28.0,
                4.0,
            )
            .translate(0.0, 0.0, 4.0);
        lands = lands + land.translate(x, -SCAN_BRIDGE_Y / 2.0 + 28.0, SCAN_BRIDGE_Z / 2.0 - 70.0);
    }
    lands
}

fn scan_confirm_light_pips() -> Part {
    let mut pips = Part::empty(name("scan_bridge_confirm_light_pips"));
    for i in 0..SCAN_CONFIRM_LIGHT_PIPS {
        let x = centered_index(i, SCAN_CONFIRM_LIGHT_PIPS, 48.0);
        pips = pips
            + centered_cylinder(
                name(&format!("scan_bridge_confirm_light_pip_{i}")),
                5.0,
                5.0,
                18,
            )
            .translate(x, 0.0, SCAN_BRIDGE_Z / 2.0 - 23.0);
    }
    pips
}

fn lot_event_card_lands() -> Part {
    let mut lands = Part::empty(name("scan_bridge_lot_event_card_lands"));
    for i in 0..LOT_EVENT_CARD_LANDS {
        let x = centered_index(i, LOT_EVENT_CARD_LANDS, 110.0);
        lands = lands
            + raised_label_land(
                &format!("scan_bridge_lot_event_card_land_{i}"),
                82.0,
                24.0,
                4,
            )
            .translate(x, 0.0, -SCAN_BRIDGE_Z / 2.0 + 14.0);
    }
    lands
}

fn sterile_connector_keying_panel() -> Part {
    let base = centered_cube(
        name("connector_keying_panel_base"),
        CONNECTOR_PANEL_X,
        CONNECTOR_PANEL_Y,
        CONNECTOR_PANEL_Z,
    );
    let center_barrier = centered_cube(
        name("connector_keying_center_no_cross_route_barrier"),
        18.0,
        CONNECTOR_PANEL_Y + 28.0,
        CONNECTOR_PANEL_Z + 38.0,
    )
    .translate(0.0, 0.0, 19.0);

    base - connector_socket_cuts()
        + center_barrier
        + connector_key_lugs()
        + membrane_tab_gauges()
        + connector_flow_direction_flags()
}

fn connector_socket_cuts() -> Part {
    let mut cuts = Part::empty(name("connector_socket_cuts"));
    for i in 0..CONNECTOR_PAIRS {
        let y = centered_index(i, CONNECTOR_PAIRS, CONNECTOR_PITCH_Y);
        for zone in [BagZone::Feed, BagZone::Harvest] {
            let x = zone.sign() * 78.0;
            cuts = cuts
                + centered_cylinder(
                    name(&format!("{}_connector_socket_cut_{i}", zone.label())),
                    CONNECTOR_SOCKET_D / 2.0,
                    CONNECTOR_PANEL_Z + 4.0,
                    32,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn connector_key_lugs() -> Part {
    let mut lugs = Part::empty(name("connector_key_lugs"));
    for i in 0..CONNECTOR_PAIRS {
        let y = centered_index(i, CONNECTOR_PAIRS, CONNECTOR_PITCH_Y);
        lugs = lugs
            + centered_cube(
                name(&format!("feed_connector_key_lug_{i}")),
                38.0,
                10.0,
                20.0,
            )
            .translate(-78.0, y - 18.0, CONNECTOR_PANEL_Z / 2.0 + 10.0)
            + centered_cube(
                name(&format!("harvest_connector_key_lug_{i}")),
                10.0,
                38.0,
                30.0,
            )
            .translate(78.0 + 18.0, y, CONNECTOR_PANEL_Z / 2.0 + 15.0);
    }

    for i in 0..MISROUTE_BLOCKER_FINS {
        let x = if i % 2 == 0 { -18.0 } else { 18.0 };
        let y = centered_index(i / 2, CONNECTOR_PAIRS, CONNECTOR_PITCH_Y);
        lugs = lugs
            + centered_cube(
                name(&format!("connector_misroute_blocker_fin_{i}")),
                12.0,
                28.0,
                34.0,
            )
            .translate(x, y, CONNECTOR_PANEL_Z / 2.0 + 17.0);
    }
    lugs
}

fn membrane_tab_gauges() -> Part {
    let mut gauges = Part::empty(name("connector_membrane_pull_tab_gauges"));
    for i in 0..MEMBRANE_PULL_TAB_GAUGES {
        let x = centered_index(i, MEMBRANE_PULL_TAB_GAUGES, 68.0);
        gauges = gauges
            + centered_cube(
                name(&format!("connector_membrane_pull_tab_gauge_{i}")),
                48.0,
                12.0,
                10.0,
            )
            .translate(
                x,
                CONNECTOR_PANEL_Y / 2.0 - 25.0,
                CONNECTOR_PANEL_Z / 2.0 + 5.0,
            );
    }
    gauges
}

fn connector_flow_direction_flags() -> Part {
    let feed_flag = raised_label_land("connector_feed_flow_in_flag", 92.0, 28.0, 3).translate(
        -CONNECTOR_PANEL_X / 2.0 + 72.0,
        -CONNECTOR_PANEL_Y / 2.0 + 24.0,
        CONNECTOR_PANEL_Z / 2.0 + 4.0,
    );
    let harvest_flag = raised_label_land("connector_harvest_flow_out_flag", 92.0, 28.0, 5)
        .translate(
            CONNECTOR_PANEL_X / 2.0 - 72.0,
            -CONNECTOR_PANEL_Y / 2.0 + 24.0,
            CONNECTOR_PANEL_Z / 2.0 + 4.0,
        );
    feed_flag + harvest_flag
}

fn cap_port_custody_parks() -> Part {
    let block = centered_cube(
        name("cap_port_custody_parks_block"),
        CAP_PARKS_X,
        CAP_PARKS_Y,
        CAP_PARKS_Z,
    );
    let center_barrier = centered_cube(
        name("cap_port_custody_feed_harvest_barrier"),
        16.0,
        CAP_PARKS_Y,
        46.0,
    )
    .translate(0.0, 0.0, CAP_PARKS_Z / 2.0 + 23.0);

    block - cap_port_cup_cuts()
        + center_barrier
        + cap_well_rims()
        + custody_seal_slots()
        + cap_port_zone_flags()
}

fn cap_port_cup_cuts() -> Part {
    let mut cuts = Part::empty(name("cap_port_custody_cup_cuts"));
    for zone in [BagZone::Feed, BagZone::Harvest] {
        for i in 0..CAP_WELLS_PER_ZONE {
            let x = zone.sign() * 98.0 + centered_index(i % 4, 4, 34.0);
            let y = 36.0 + centered_index(i / 4, 2, 42.0);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("{}_cap_well_cut_{i}", zone.label())),
                    CAP_WELL_D / 2.0,
                    CAP_PARKS_Z,
                    24,
                )
                .translate(x, y, CAP_PARKS_Z / 2.0 - 13.0);
        }
        for i in 0..PORT_CUPS_PER_ZONE {
            let x = zone.sign() * 98.0 + centered_index(i % 3, 3, 42.0);
            let y = -45.0 + centered_index(i / 3, 2, 42.0);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("{}_open_port_cup_cut_{i}", zone.label())),
                    PORT_CUP_D / 2.0,
                    CAP_PARKS_Z,
                    28,
                )
                .translate(x, y, CAP_PARKS_Z / 2.0 - 13.0);
        }
    }
    cuts
}

fn cap_well_rims() -> Part {
    let mut rims = Part::empty(name("cap_port_custody_well_rims"));
    for zone in [BagZone::Feed, BagZone::Harvest] {
        for i in 0..CAP_WELLS_PER_ZONE {
            let x = zone.sign() * 98.0 + centered_index(i % 4, 4, 34.0);
            let y = 36.0 + centered_index(i / 4, 2, 42.0);
            rims = rims
                + centered_cylinder(
                    name(&format!("{}_cap_well_rim_{i}", zone.label())),
                    CAP_WELL_D / 2.0 + 4.0,
                    5.0,
                    24,
                )
                .translate(x, y, CAP_PARKS_Z / 2.0 + 2.5);
        }
    }
    rims
}

fn custody_seal_slots() -> Part {
    let mut slots = Part::empty(name("cap_port_custody_tamper_seal_slots"));
    for i in 0..CUSTODY_SEAL_SLOTS {
        let x = centered_index(i, CUSTODY_SEAL_SLOTS, 48.0);
        slots = slots
            + centered_cube(
                name(&format!("cap_port_custody_tamper_seal_slot_{i}")),
                32.0,
                13.0,
                8.0,
            )
            .translate(x, -CAP_PARKS_Y / 2.0 + 18.0, CAP_PARKS_Z / 2.0 + 4.0);
    }
    slots
}

fn cap_port_zone_flags() -> Part {
    raised_label_land("cap_port_feed_clean_side_flag", 88.0, 22.0, 3).translate(
        -CAP_PARKS_X / 2.0 + 58.0,
        CAP_PARKS_Y / 2.0 - 18.0,
        CAP_PARKS_Z / 2.0 + 4.0,
    ) + raised_label_land("cap_port_harvest_clean_side_flag", 88.0, 22.0, 5).translate(
        CAP_PARKS_X / 2.0 - 58.0,
        CAP_PARKS_Y / 2.0 - 18.0,
        CAP_PARKS_Z / 2.0 + 4.0,
    )
}

fn route_confirmation_token_rail() -> Part {
    let rail = centered_cube(
        name("route_confirmation_token_rail_block"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let lane_dividers = token_lane_dividers();

    rail - route_token_slot_cuts() + lane_dividers + token_presence_flags()
}

fn route_token_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("route_confirmation_token_slot_cuts"));
    for lane in 0..TOKEN_LANES {
        let y = centered_index(lane, TOKEN_LANES, TOKEN_LANE_PITCH_Y);
        for slot in 0..TOKENS_PER_LANE {
            let x = centered_index(slot, TOKENS_PER_LANE, TOKEN_SLOT_PITCH_X);
            cuts = cuts
                + centered_cube(
                    name(&format!(
                        "route_confirmation_lane_{lane}_token_slot_cut_{slot}"
                    )),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_RAIL_Z,
                )
                .translate(x, y, TOKEN_RAIL_Z / 2.0 - 12.0);
        }
    }
    cuts
}

fn token_lane_dividers() -> Part {
    let mut dividers = Part::empty(name("route_confirmation_token_lane_dividers"));
    for i in 0..TOKEN_LANES + 1 {
        let y = -(TOKEN_LANES as f64 * TOKEN_LANE_PITCH_Y) / 2.0 + i as f64 * TOKEN_LANE_PITCH_Y
            - TOKEN_LANE_PITCH_Y / 2.0;
        dividers = dividers
            + centered_cube(
                name(&format!("route_confirmation_token_lane_divider_{i}")),
                TOKEN_RAIL_X - 36.0,
                5.0,
                16.0,
            )
            .translate(0.0, y, TOKEN_RAIL_Z / 2.0 + 8.0);
    }
    dividers
}

fn token_presence_flags() -> Part {
    let mut flags = Part::empty(name("route_confirmation_token_presence_flags"));
    for lane in 0..TOKEN_LANES {
        let y = centered_index(lane, TOKEN_LANES, TOKEN_LANE_PITCH_Y);
        flags = flags
            + raised_label_land(
                &format!("route_confirmation_lane_{lane}_presence_flag"),
                48.0,
                20.0,
                lane + 2,
            )
            .translate(-TOKEN_RAIL_X / 2.0 + 42.0, y, TOKEN_RAIL_Z / 2.0 + 4.0);
    }
    flags
}

fn quarantine_release_lanes() -> Part {
    let lane_block = centered_cube(
        name("quarantine_release_lane_block"),
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    lane_block - disposition_slot_cuts() + disposition_lane_dividers() + status_gate_flags()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty(name("quarantine_release_disposition_slot_cuts"));
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, LANE_PITCH_Y);
        for slot in 0..LANE_SLOTS_PER_STATUS {
            let x = centered_index(slot, LANE_SLOTS_PER_STATUS, 78.0);
            cuts = cuts
                + centered_cube(
                    name(&format!("quarantine_release_lane_{lane}_slot_cut_{slot}")),
                    LANE_SLOT_X,
                    LANE_SLOT_Y,
                    LANE_Z,
                )
                .translate(x, y, LANE_Z / 2.0 - 12.0);
        }
    }
    cuts
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty(name("quarantine_release_lane_dividers"));
    for i in 0..DISPOSITION_LANES + 1 {
        let y = -(DISPOSITION_LANES as f64 * LANE_PITCH_Y) / 2.0 + i as f64 * LANE_PITCH_Y
            - LANE_PITCH_Y / 2.0;
        dividers = dividers
            + centered_cube(
                name(&format!("quarantine_release_lane_divider_{i}")),
                LANE_X - 26.0,
                8.0,
                LANE_DIVIDER_Z,
            )
            .translate(0.0, y, LANE_Z / 2.0 + LANE_DIVIDER_Z / 2.0);
    }
    dividers
}

fn status_gate_flags() -> Part {
    let mut flags = Part::empty(name("quarantine_release_status_gate_flags"));
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, LANE_PITCH_Y);
        flags = flags
            + raised_label_land(
                &format!("quarantine_release_status_gate_flag_{lane}"),
                58.0,
                20.0,
                lane + 3,
            )
            .translate(-LANE_X / 2.0 + 44.0, y, LANE_Z / 2.0 + 5.0);
    }
    flags
}

fn temperature_logger_pockets() -> Part {
    let block = centered_cube(
        name("temperature_logger_pocket_block"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    block - logger_pocket_cuts() + logger_zone_barrier() + logger_sensor_cable_combs()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty(name("temperature_logger_pocket_cuts"));
    for i in 0..LOGGER_POCKETS {
        let x = if i < COLD_LOGGER_POCKETS {
            -LOGGER_THERMAL_GAP - centered_index(i, COLD_LOGGER_POCKETS, 92.0).abs()
        } else {
            LOGGER_THERMAL_GAP
                + centered_index(i - COLD_LOGGER_POCKETS, ROOM_LOGGER_POCKETS, 92.0).abs()
        };
        cuts = cuts
            + centered_cube(
                name(&format!("temperature_logger_pocket_cut_{i}")),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH,
            )
            .translate(x, 0.0, LOGGER_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0 + 1.0);
    }
    cuts
}

fn logger_zone_barrier() -> Part {
    centered_cube(
        name("temperature_logger_cold_room_temp_isolation_barrier"),
        18.0,
        LOGGER_Y + 20.0,
        LOGGER_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0)
}

fn logger_sensor_cable_combs() -> Part {
    let mut combs = Part::empty(name("temperature_logger_sensor_cable_combs"));
    for i in 0..LOGGER_POCKETS {
        let x = centered_index(i, LOGGER_POCKETS, 88.0);
        combs = combs
            + centered_cube(
                name(&format!("temperature_logger_sensor_cable_comb_{i}")),
                44.0,
                8.0,
                9.0,
            )
            .translate(x, -LOGGER_Y / 2.0 + 14.0, LOGGER_Z / 2.0 + 4.5);
    }
    combs
}

fn leak_drain_splash_guards() -> Part {
    let trough = centered_cube(
        name("leak_drain_splash_guard_trough"),
        LEAK_GUARD_X,
        LEAK_GUARD_Y,
        LEAK_GUARD_Z,
    ) - centered_cube(
        name("leak_drain_splash_guard_channel_cut"),
        LEAK_GUARD_X - 52.0,
        LEAK_GUARD_Y - 34.0,
        LEAK_GUARD_Z,
    )
    .translate(0.0, 0.0, 8.0);

    trough + splash_guard_posts() + sample_wells() - leak_trough_drain_cuts()
}

fn splash_guard_posts() -> Part {
    let mut posts = Part::empty(name("leak_drain_splash_guard_posts"));
    for i in 0..SPLASH_GUARD_POSTS {
        let x = centered_index(
            i,
            SPLASH_GUARD_POSTS,
            LEAK_GUARD_X / (SPLASH_GUARD_POSTS as f64),
        );
        posts = posts
            + centered_cube(
                name(&format!("leak_drain_splash_guard_post_{i}")),
                16.0,
                12.0,
                72.0,
            )
            .translate(x, LEAK_GUARD_Y / 2.0 - 12.0, LEAK_GUARD_Z / 2.0 + 36.0);
    }
    posts
}

fn sample_wells() -> Part {
    let mut wells = Part::empty(name("leak_drain_sample_wells"));
    for i in 0..SAMPLE_WELL_COUNT {
        let x = centered_index(i, SAMPLE_WELL_COUNT, 80.0);
        let well = centered_cylinder(
            name(&format!("leak_drain_sample_well_rim_{i}")),
            18.0,
            8.0,
            24,
        ) - centered_cylinder(
            name(&format!("leak_drain_sample_well_cut_{i}")),
            12.0,
            10.0,
            24,
        );
        wells = wells + well.translate(x, -LEAK_GUARD_Y / 2.0 + 18.0, LEAK_GUARD_Z / 2.0 + 4.0);
    }
    wells
}

fn leak_trough_drain_cuts() -> Part {
    let mut drains = Part::empty(name("leak_trough_drain_cuts"));
    for i in 0..LEAK_TROUGH_DRAINS {
        let x = centered_index(i, LEAK_TROUGH_DRAINS, 260.0);
        drains = drains
            + centered_cylinder(name(&format!("leak_trough_drain_cut_{i}")), 9.0, 52.0, 24)
                .translate(x, 0.0, 0.0);
    }
    drains
}

fn custody_evidence_panel() -> Part {
    let panel = centered_cube(
        name("custody_evidence_panel_base"),
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    );
    panel + lot_card_slots() + event_read_point_lands() + custody_mismatch_flag_rack()
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty(name("custody_evidence_lot_card_slots"));
    for i in 0..LOT_CARD_SLOTS {
        let x = centered_index(i, LOT_CARD_SLOTS, 72.0);
        slots = slots
            + centered_cube(
                name(&format!("custody_evidence_lot_card_slot_{i}")),
                58.0,
                15.0,
                14.0,
            )
            .translate(x, -EVIDENCE_Y / 2.0 + 18.0, EVIDENCE_Z / 2.0 + 7.0);
    }
    slots
}

fn event_read_point_lands() -> Part {
    let mut lands = Part::empty(name("custody_evidence_event_read_point_lands"));
    for i in 0..EVENT_TIME_LANDS {
        let x = -80.0 + centered_index(i, EVENT_TIME_LANDS, 46.0);
        lands = lands
            + raised_label_land(
                &format!("custody_evidence_event_time_land_{i}"),
                36.0,
                20.0,
                2,
            )
            .translate(x, EVIDENCE_Y / 2.0 - 18.0, EVIDENCE_Z / 2.0 + 4.0);
    }
    for i in 0..READ_POINT_LANDS {
        let x = 80.0 + centered_index(i, READ_POINT_LANDS, 46.0);
        lands = lands
            + raised_label_land(
                &format!("custody_evidence_read_point_land_{i}"),
                36.0,
                20.0,
                3,
            )
            .translate(x, EVIDENCE_Y / 2.0 - 18.0, EVIDENCE_Z / 2.0 + 4.0);
    }
    lands
}

fn custody_mismatch_flag_rack() -> Part {
    let mut rack = Part::empty(name("custody_evidence_mismatch_flag_rack"));
    for i in 0..4 {
        rack = rack
            + centered_cube(
                name(&format!("custody_evidence_mismatch_flag_{i}")),
                28.0,
                9.0,
                24.0,
            )
            .translate(centered_index(i, 4, 42.0), 0.0, EVIDENCE_Z / 2.0 + 12.0);
    }
    rack
}

fn robot_service_keepouts() -> Part {
    let front = clearance_box(
        "robot_service_front_robot_approach_keepout",
        KEEP_OUT_X,
        42.0,
        FRONT_ROBOT_CLEARANCE,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, FRONT_ROBOT_CLEARANCE / 2.0);
    let rear = clearance_box(
        "robot_service_rear_service_keepout",
        KEEP_OUT_X,
        42.0,
        REAR_SERVICE_CLEARANCE,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, REAR_SERVICE_CLEARANCE / 2.0);
    let left = clearance_box(
        "robot_service_left_feed_bag_load_keepout",
        42.0,
        KEEP_OUT_Y,
        LEFT_FEED_LOAD_CLEARANCE,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, LEFT_FEED_LOAD_CLEARANCE / 2.0);
    let right = clearance_box(
        "robot_service_right_harvest_bag_load_keepout",
        42.0,
        KEEP_OUT_Y,
        RIGHT_HARVEST_LOAD_CLEARANCE,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, RIGHT_HARVEST_LOAD_CLEARANCE / 2.0);
    let overhead_scan = centered_cylinder(
        name("robot_service_overhead_scan_clearance_post"),
        8.0,
        OVERHEAD_SCAN_CLEARANCE,
        24,
    )
    .translate(
        SCAN_BRIDGE_POS.0,
        SCAN_BRIDGE_POS.1,
        OVERHEAD_SCAN_CLEARANCE / 2.0,
    );
    let overhead_bag = centered_cylinder(
        name("robot_service_overhead_bag_lift_clearance_post"),
        8.0,
        OVERHEAD_BAG_LIFT_CLEARANCE,
        24,
    )
    .translate(0.0, 110.0, OVERHEAD_BAG_LIFT_CLEARANCE / 2.0);

    front + rear + left + right + overhead_scan + overhead_bag + keepout_group_tags()
}

fn clearance_box(id: &str, x: f64, y: f64, z: f64) -> Part {
    let mut box_part = Part::empty(name(&format!("{id}_rails")));
    for (i, dx) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            box_part = box_part
                + centered_cube(
                    name(&format!("{id}_vertical_post_{i}_{j}")),
                    KEEP_OUT_RAIL,
                    KEEP_OUT_RAIL,
                    z,
                )
                .translate(dx * x / 2.0, dy * y / 2.0, 0.0);
        }
    }
    for (i, dz) in [-1.0, 1.0].iter().enumerate() {
        for (j, dy) in [-1.0, 1.0].iter().enumerate() {
            box_part = box_part
                + centered_cube(
                    name(&format!("{id}_x_rail_{i}_{j}")),
                    x,
                    KEEP_OUT_RAIL,
                    KEEP_OUT_RAIL,
                )
                .translate(0.0, dy * y / 2.0, dz * z / 2.0);
        }
        for (j, dx) in [-1.0, 1.0].iter().enumerate() {
            box_part = box_part
                + centered_cube(
                    name(&format!("{id}_y_rail_{i}_{j}")),
                    KEEP_OUT_RAIL,
                    y,
                    KEEP_OUT_RAIL,
                )
                .translate(dx * x / 2.0, 0.0, dz * z / 2.0);
        }
    }
    box_part
}

fn keepout_group_tags() -> Part {
    let mut tags = Part::empty(name("robot_service_keepout_group_tags"));
    for i in 0..KEEP_OUT_GROUPS {
        let x = centered_index(i, KEEP_OUT_GROUPS, 110.0);
        tags = tags
            + raised_label_land(
                &format!("robot_service_keepout_group_tag_{i}"),
                72.0,
                22.0,
                i + 2,
            )
            .translate(x, -KEEP_OUT_Y / 2.0 + 26.0, KEEP_OUT_RAIL / 2.0 + 4.0);
    }
    tags
}

fn barcode_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        let bar_w = if i % 2 == 0 { 2.0 } else { 4.0 };
        land = land
            + centered_cube(
                name(&format!("{id}_barcode_bar_{i}")),
                bar_w,
                depth - 7.0,
                3.0,
            )
            .translate(x, 0.0, 3.5);
    }
    land
}

fn raised_label_land(id: &str, width: f64, depth: f64, bars: usize) -> Part {
    let mut land = centered_cube(name(id), width, depth, 4.0);
    for i in 0..bars {
        let x = centered_index(i, bars, width / (bars as f64 + 1.0));
        land = land
            + centered_cube(name(&format!("{id}_raised_bar_{i}")), 3.0, depth - 8.0, 3.0)
                .translate(x, 0.0, 3.5);
    }
    land
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(suffix: &str) -> String {
    format!("{BIN_NAME}_{suffix}")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_lot_mixup_prevention_request() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for feature in [
            "separated_feed_zone",
            "separated_harvest_zone",
            "barcode_rfid_scan_bridge",
            "load_cell_bag_nests",
            "sterile_connector_keying",
            "cap_port_custody_parks",
            "route_confirmation_token_rail",
            "quarantine_release_lanes",
            "cold_room_temp_logger_pockets",
            "leak_tray",
            "robot_service_keepouts",
            "chain_of_custody_evidence_panel",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert!(LIMITATIONS.contains(&"no_sterile_connection_sop"));
    }

    #[test]
    fn major_components_fit_without_unplanned_overlap() {
        assert_layout();
        for component in component_rects() {
            assert!(component.fits_inside_station());
        }
    }

    #[test]
    fn feed_and_harvest_zones_are_physically_separated_and_distinctly_keyed() {
        assert!(feed_rect().center.0 < 0.0);
        assert!(harvest_rect().center.0 > 0.0);
        assert!(feed_rect().horizontal_gap(harvest_rect()) >= MIN_ZONE_GAP);
        assert_eq!(LOAD_CELLS_PER_NEST, 4);
        assert_eq!(load_cell_points().len(), LOAD_CELLS_PER_NEST);
        assert_eq!(ZONE_KEY_FINS_PER_NEST, 3);
        assert_ne!(BagZone::Feed.key_height(), BagZone::Harvest.key_height());
        assert!(BagZone::Harvest.key_height() > BagZone::Feed.key_height());
    }

    #[test]
    fn scan_bridge_and_evidence_counts_lock_identity_capture() {
        assert_eq!(BARCODE_WINDOWS, 4);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(LOT_EVENT_CARD_LANDS, 4);
        assert_eq!(SCAN_CONFIRM_LIGHT_PIPS, 8);
        assert_eq!(LOT_CARD_SLOTS, 4);
        assert_eq!(EVENT_TIME_LANDS + READ_POINT_LANDS, 6);
        assert!(SCAN_UNDERSIDE_CLEARANCE > BAG_NEST_Z + 80.0);
    }

    #[test]
    fn connector_keying_and_custody_capacity_match_two_bag_routes() {
        assert_eq!(CONNECTOR_PAIRS, 4);
        assert_eq!(FEED_CONNECTOR_KEYS, CONNECTOR_PAIRS);
        assert_eq!(HARVEST_CONNECTOR_KEYS, CONNECTOR_PAIRS);
        assert_eq!(MISROUTE_BLOCKER_FINS, CONNECTOR_PAIRS * 2);
        assert_eq!(CAP_WELLS_PER_ZONE * 2, 16);
        assert_eq!(PORT_CUPS_PER_ZONE * 2, 12);
        assert_eq!(CUSTODY_SEAL_SLOTS, 6);
        assert!(CONNECTOR_PANEL_X < feed_rect().horizontal_gap(harvest_rect()));
    }

    #[test]
    fn route_tokens_disposition_lanes_and_loggers_are_counted() {
        assert_eq!(TOKEN_LANES, 4);
        assert_eq!(TOKENS_PER_LANE, 5);
        assert_eq!(ROUTE_TOKEN_SLOTS, 20);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(QUARANTINE_SLOTS + HOLD_SLOTS + RELEASE_SLOTS, 12);
        assert_eq!(LOGGER_POCKETS, 4);
        assert_eq!(COLD_LOGGER_POCKETS, 2);
        assert_eq!(ROOM_LOGGER_POCKETS, 2);
        assert!(LOGGER_THERMAL_GAP > LOGGER_POCKET_X / 2.0);
    }

    #[test]
    fn leak_tray_and_keepouts_are_explicit() {
        assert_eq!(LEAK_TROUGH_DRAINS, 4);
        assert_eq!(SAMPLE_WELL_COUNT, 6);
        assert_eq!(SPLASH_GUARD_POSTS, 6);
        assert_eq!(KEEP_OUT_GROUPS, 6);
        assert!(FRONT_ROBOT_CLEARANCE >= 420.0);
        assert!(REAR_SERVICE_CLEARANCE >= 260.0);
        assert!(LEFT_FEED_LOAD_CLEARANCE >= 240.0);
        assert!(RIGHT_HARVEST_LOAD_CLEARANCE >= 240.0);
        assert!(OVERHEAD_BAG_LIFT_CLEARANCE > OVERHEAD_SCAN_CLEARANCE);
    }
}
