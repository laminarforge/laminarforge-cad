use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed feed/harvest line reversal prevention station.
//
// Intent:
// - Validate that an automated closed fluid path cannot swap feed and harvest
//   lines during setup, robot service, bag changeover, or cassette docking.
// - Make reversal prevention physical first: asymmetric keyed port nests,
//   separated strain-relief combs, one-way check-valve witness geometry,
//   wrong-route challenge coupons, and interlocked clamps before route release.
// - Keep custody/evidence surfaces visible so the line identity decision is
//   captured at the same station that blocks the wrong mechanical route.
//
// Research assumptions encoded in geometry:
// - Closed single-use bioprocess paths rely on pre-use inspection, connector
//   integrity, route identity, and in-process leak/pressure checks; this model
//   represents a validation fixture for those controls, not the sterile SOP.
// - Human-factors and automation practice favors poka-yoke asymmetry over
//   labels alone. Feed and harvest features therefore differ in key height,
//   port pitch, datum side, and challenge-coupon geometry.
// - Organ/tissue-chip reproducibility depends on avoiding unintended reverse
//   flow, stagnant dead legs, and contamination-prone manual correction. The
//   fixture forces wrong-route attempts into a reject/hold workflow before any
//   culture-facing path is opened.
//
// This is validation CAD only. It is not a pressure-rated manifold, sterile
// barrier validation, wetted material specification, controller, or release SOP.

const PREFIX: &str = "closed_feed_harvest_line_reversal_prevention_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_feed_harvest_line_reversal_prevention_station_base_leak_tray.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_feed_source_keyed_port_nest.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_harvest_return_keyed_port_nest.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_unidirectional_check_manifold.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_reversal_challenge_coupon_sled.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_pressure_flow_witness_panel.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_barcode_route_authority_bridge.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_tubing_strain_relief_comb.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_clamp_occlusion_interlock_rail.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_reject_hold_release_gate.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_robot_service_keepout_gauges.stl",
    "output/closed_feed_harvest_line_reversal_prevention_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "asymmetric_feed_port_keys",
    "asymmetric_harvest_port_keys",
    "one_way_check_valve_witnesses",
    "wrong_route_challenge_coupon",
    "pressure_flow_witness_panel",
    "barcode_route_authority_bridge",
    "separated_tubing_strain_relief",
    "clamp_occlusion_interlock",
    "reject_hold_release_gate",
    "leak_tray",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1340.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 13.0;

const FEED_POS: (f64, f64) = (-450.0, 218.0);
const HARVEST_POS: (f64, f64) = (450.0, 218.0);
const PORT_NEST_X: f64 = 330.0;
const PORT_NEST_Y: f64 = 232.0;
const PORT_NEST_Z: f64 = 64.0;
const PORT_COUNT_PER_SIDE: usize = 4;
const PORT_SOCKET_D: f64 = 30.0;
const FEED_PORT_PITCH: f64 = 58.0;
const HARVEST_PORT_PITCH: f64 = 72.0;
const FEED_KEY_HEIGHT: f64 = 26.0;
const HARVEST_KEY_HEIGHT: f64 = 43.0;
const FEED_KEY_OFFSET_Y: f64 = -42.0;
const HARVEST_KEY_OFFSET_Y: f64 = 48.0;
const DATUM_PIN_D: f64 = 8.0;
const DATUM_PIN_COUNT_PER_NEST: usize = 3;

const MANIFOLD_POS: (f64, f64) = (0.0, 218.0);
const MANIFOLD_X: f64 = 374.0;
const MANIFOLD_Y: f64 = 232.0;
const MANIFOLD_Z: f64 = 70.0;
const FLOW_LANES: usize = PORT_COUNT_PER_SIDE;
const MANIFOLD_LANE_PITCH_Y: f64 = 42.0;
const TUBE_BORE_D: f64 = 7.0;
const CHECK_VALVES_PER_LANE: usize = 2;
const CHECK_VALVE_WITNESSES: usize = FLOW_LANES * CHECK_VALVES_PER_LANE;
const ARROW_BAR_COUNT: usize = FLOW_LANES * 3;
const DEAD_LEG_WITNESS_WELLS: usize = FLOW_LANES;

const COUPON_POS: (f64, f64) = (0.0, -47.0);
const COUPON_X: f64 = 442.0;
const COUPON_Y: f64 = 208.0;
const COUPON_Z: f64 = 44.0;
const CHALLENGE_COUPONS: usize = 6;
const WRONG_ROUTE_STOP_COUNT: usize = CHALLENGE_COUPONS;
const COUPON_SLOT_X: f64 = 52.0;
const COUPON_SLOT_Y: f64 = 34.0;
const COUPON_PITCH_X: f64 = 64.0;
const ASYMMETRY_GAUGE_COUNT: usize = 4;

const WITNESS_POS: (f64, f64) = (-452.0, -52.0);
const WITNESS_X: f64 = 332.0;
const WITNESS_Y: f64 = 218.0;
const WITNESS_Z: f64 = 54.0;
const PRESSURE_SENSOR_POCKETS: usize = FLOW_LANES;
const FLOW_SENSOR_WINDOWS: usize = FLOW_LANES;
const BUBBLE_WITNESS_WINDOWS: usize = FLOW_LANES;
const WITNESS_CHANNEL_PITCH_Y: f64 = 42.0;

const SCAN_POS: (f64, f64) = (452.0, -52.0);
const SCAN_X: f64 = 332.0;
const SCAN_Y: f64 = 218.0;
const SCAN_Z: f64 = 190.0;
const SCAN_POST_W: f64 = 24.0;
const SCAN_BEAM_Z: f64 = 24.0;
const ROUTE_BARCODE_LANDS: usize = 6;
const LOT_EVENT_LANDS: usize = 4;
const AUTHORITY_TOKEN_DOCKS: usize = 8;

const COMB_POS: (f64, f64) = (-452.0, -316.0);
const COMB_X: f64 = 332.0;
const COMB_Y: f64 = 154.0;
const COMB_Z: f64 = 38.0;
const COMB_FEED_SLOTS: usize = FLOW_LANES;
const COMB_HARVEST_SLOTS: usize = FLOW_LANES;
const COMB_SLOT_X: f64 = 20.0;
const COMB_SLOT_Y: f64 = 64.0;
const COMB_SLOT_PITCH_X: f64 = 34.0;
const LINE_SEPARATOR_RIBS: usize = 5;

const CLAMP_POS: (f64, f64) = (0.0, -316.0);
const CLAMP_X: f64 = 382.0;
const CLAMP_Y: f64 = 154.0;
const CLAMP_Z: f64 = 48.0;
const INTERLOCK_CLAMPS: usize = FLOW_LANES * 2;
const CLAMP_PITCH_X: f64 = 38.0;
const CLAMP_OCCLUSION_WINDOWS: usize = INTERLOCK_CLAMPS;
const NO_BYPASS_GUARDS: usize = 3;

const GATE_POS: (f64, f64) = (452.0, -316.0);
const GATE_X: f64 = 332.0;
const GATE_Y: f64 = 154.0;
const GATE_Z: f64 = 42.0;
const DISPOSITION_LANES: usize = 3;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SLOT_X: f64 = 58.0;
const GATE_SLOT_Y: f64 = 26.0;
const GATE_PITCH_Y: f64 = 44.0;

const KEEP_OUT_FLAGS: usize = 6;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 170.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 128.0;
const LEFT_LINE_LOAD_KEEP_OUT_X: f64 = 155.0;
const RIGHT_LINE_LOAD_KEEP_OUT_X: f64 = 155.0;
const OVERHEAD_BAG_LIFT_KEEP_OUT_Z: f64 = 310.0;

const MIN_COMPONENT_GAP: f64 = 22.0;
const MIN_FEED_HARVEST_ZONE_GAP: f64 = 570.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 14.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 14.0;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RouteSide {
    Feed,
    Harvest,
}

impl RouteSide {
    fn slug(self) -> &'static str {
        match self {
            RouteSide::Feed => "feed",
            RouteSide::Harvest => "harvest",
        }
    }

    fn key_height(self) -> f64 {
        match self {
            RouteSide::Feed => FEED_KEY_HEIGHT,
            RouteSide::Harvest => HARVEST_KEY_HEIGHT,
        }
    }

    fn key_offset_y(self) -> f64 {
        match self {
            RouteSide::Feed => FEED_KEY_OFFSET_Y,
            RouteSide::Harvest => HARVEST_KEY_OFFSET_Y,
        }
    }

    fn port_pitch(self) -> f64 {
        match self {
            RouteSide::Feed => FEED_PORT_PITCH,
            RouteSide::Harvest => HARVEST_PORT_PITCH,
        }
    }

    fn datum_sign(self) -> f64 {
        match self {
            RouteSide::Feed => -1.0,
            RouteSide::Harvest => 1.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let feed_nest = keyed_port_nest(RouteSide::Feed);
    export(OUTPUTS[1], &feed_nest);

    let harvest_nest = keyed_port_nest(RouteSide::Harvest);
    export(OUTPUTS[2], &harvest_nest);

    let manifold = unidirectional_check_manifold();
    export(OUTPUTS[3], &manifold);

    let coupon = reversal_challenge_coupon_sled();
    export(OUTPUTS[4], &coupon);

    let witness = pressure_flow_witness_panel();
    export(OUTPUTS[5], &witness);

    let scan = barcode_route_authority_bridge();
    export(OUTPUTS[6], &scan);

    let comb = tubing_strain_relief_comb();
    export(OUTPUTS[7], &comb);

    let clamps = clamp_occlusion_interlock_rail();
    export(OUTPUTS[8], &clamps);

    let gate = reject_hold_release_gate();
    export(OUTPUTS[9], &gate);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + feed_nest.translate(FEED_POS.0, FEED_POS.1, on_deck_z(PORT_NEST_Z))
        + harvest_nest.translate(HARVEST_POS.0, HARVEST_POS.1, on_deck_z(PORT_NEST_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_deck_z(MANIFOLD_Z))
        + coupon.translate(COUPON_POS.0, COUPON_POS.1, on_deck_z(COUPON_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + scan.translate(SCAN_POS.0, SCAN_POS.1, on_deck_z(SCAN_Z))
        + comb.translate(COMB_POS.0, COMB_POS.1, on_deck_z(COMB_Z))
        + clamps.translate(CLAMP_POS.0, CLAMP_POS.1, on_deck_z(CLAMP_Z))
        + gate.translate(GATE_POS.0, GATE_POS.1, on_deck_z(GATE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + 8.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed feed/harvest line reversal prevention station:");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained leak tray");
    println!(
        "  Route endpoints:        {PORT_COUNT_PER_SIDE} feed keyed sockets and {PORT_COUNT_PER_SIDE} harvest keyed sockets with non-matching pitch/key height"
    );
    println!(
        "  One-way evidence:       {FLOW_LANES} flow lanes, {CHECK_VALVE_WITNESSES} check-valve witnesses, {DEAD_LEG_WITNESS_WELLS} dead-leg witness wells"
    );
    println!(
        "  Reversal challenge:     {CHALLENGE_COUPONS} challenge coupons, {WRONG_ROUTE_STOP_COUNT} wrong-route stops, {ASYMMETRY_GAUGE_COUNT} asymmetry gauges"
    );
    println!(
        "  Measurement surfaces:   {PRESSURE_SENSOR_POCKETS} pressure pockets, {FLOW_SENSOR_WINDOWS} flow windows, {BUBBLE_WITNESS_WINDOWS} bubble windows"
    );
    println!(
        "  Route release controls: {ROUTE_BARCODE_LANDS} barcode lands, {AUTHORITY_TOKEN_DOCKS} authority docks, {INTERLOCK_CLAMPS} interlocked clamps"
    );
    println!(
        "  Keepouts:               front robot {FRONT_ROBOT_KEEP_OUT_Y:.0}mm, rear service {REAR_SERVICE_KEEP_OUT_Y:.0}mm, left/right line loading {LEFT_LINE_LOAD_KEEP_OUT_X:.0}/{RIGHT_LINE_LOAD_KEEP_OUT_X:.0}mm, overhead {OVERHEAD_BAG_LIFT_KEEP_OUT_Z:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0 + 4.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "export count changed");
    assert_eq!(CHECK_VALVE_WITNESSES, FLOW_LANES * CHECK_VALVES_PER_LANE);
    assert_eq!(FLOW_LANES, PORT_COUNT_PER_SIDE);
    assert_eq!(PRESSURE_SENSOR_POCKETS, FLOW_LANES);
    assert_eq!(FLOW_SENSOR_WINDOWS, FLOW_LANES);
    assert_eq!(BUBBLE_WITNESS_WINDOWS, FLOW_LANES);
    assert_ne!(FEED_KEY_HEIGHT, HARVEST_KEY_HEIGHT);
    assert_ne!(FEED_KEY_OFFSET_Y, HARVEST_KEY_OFFSET_Y);
    assert_ne!(FEED_PORT_PITCH, HARVEST_PORT_PITCH);
    assert!(HARVEST_KEY_HEIGHT > FEED_KEY_HEIGHT + 12.0);
    assert!(MIN_FEED_HARVEST_ZONE_GAP <= feed_rect().horizontal_gap(harvest_rect()));
    assert!(INTERLOCK_CLAMPS >= FLOW_LANES * 2);
    assert!(AUTHORITY_TOKEN_DOCKS >= ROUTE_BARCODE_LANDS);
    assert!(WRONG_ROUTE_STOP_COUNT >= CHALLENGE_COUPONS);

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
}

fn component_rects() -> [Rect; 9] {
    [
        feed_rect(),
        harvest_rect(),
        Rect {
            name: "unidirectional_check_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Rect {
            name: "reversal_challenge_coupon_sled",
            center: COUPON_POS,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "pressure_flow_witness_panel",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "barcode_route_authority_bridge",
            center: SCAN_POS,
            x: SCAN_X,
            y: SCAN_Y,
        },
        Rect {
            name: "tubing_strain_relief_comb",
            center: COMB_POS,
            x: COMB_X,
            y: COMB_Y,
        },
        Rect {
            name: "clamp_occlusion_interlock_rail",
            center: CLAMP_POS,
            x: CLAMP_X,
            y: CLAMP_Y,
        },
        Rect {
            name: "reject_hold_release_gate",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn feed_rect() -> Rect {
    Rect {
        name: "feed_source_keyed_port_nest",
        center: FEED_POS,
        x: PORT_NEST_X,
        y: PORT_NEST_Y,
    }
}

fn harvest_rect() -> Rect {
    Rect {
        name: "harvest_return_keyed_port_nest",
        center: HARVEST_POS,
        x: PORT_NEST_X,
        y: PORT_NEST_Y,
    }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(name("base_leak_tray_deck"), STATION_X, STATION_Y, BASE_Z);
    let sump = centered_cube(
        name("base_shallow_containment_sump"),
        STATION_X - 118.0,
        STATION_Y - 108.0,
        SOCKET_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.5);

    deck - sump - base_insert_sockets() - base_drain_ports() - base_mount_holes()
        + perimeter_rim()
        + base_center_route_fences()
        + base_robot_fiducials()
        + base_leak_witness_ribs()
}

fn base_insert_sockets() -> Part {
    let mut sockets = Part::empty(name("base_insert_sockets"));
    for rect in component_rects() {
        sockets = sockets
            + centered_cube(
                name(&format!("base_{}_socket", rect.name)),
                rect.x + 12.0,
                rect.y + 12.0,
                SOCKET_DEPTH + 1.0,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn base_drain_ports() -> Part {
    let mut drains = Part::empty(name("base_drain_ports"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 84.0, -STATION_Y / 2.0 + 78.0),
        (STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 + 78.0),
        (-STATION_X / 2.0 + 84.0, STATION_Y / 2.0 - 78.0),
        (STATION_X / 2.0 - 84.0, STATION_Y / 2.0 - 78.0),
        (0.0, -STATION_Y / 2.0 + 68.0),
    ]
    .iter()
    .enumerate()
    {
        drains = drains
            + centered_cylinder(
                name(&format!("base_drain_port_{index}")),
                DRAIN_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    drains
}

fn base_mount_holes() -> Part {
    let mut holes = Part::empty(name("base_mount_holes"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 52.0, -STATION_Y / 2.0 + 50.0),
        (STATION_X / 2.0 - 52.0, -STATION_Y / 2.0 + 50.0),
        (-STATION_X / 2.0 + 52.0, STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 52.0, STATION_Y / 2.0 - 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
        (0.0, -STATION_Y / 2.0 + 50.0),
    ]
    .iter()
    .enumerate()
    {
        let round = centered_cylinder(
            name(&format!("base_m6_mount_hole_{index}")),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 8.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            name(&format!("base_m6_mount_slot_relief_{index}")),
            28.0,
            MOUNT_HOLE_D + 0.4,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + round + slot;
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        name("base_front_low_containment_lip"),
        STATION_X,
        14.0,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(name("base_rear_service_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(name("base_left_containment_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(name("base_right_containment_rim"), RIM_W, STATION_Y, RIM_Z)
        .translate(
            STATION_X / 2.0 - RIM_W / 2.0,
            0.0,
            BASE_Z / 2.0 + RIM_Z / 2.0,
        );
    front + rear + left + right
}

fn base_center_route_fences() -> Part {
    let vertical = centered_cube(
        name("base_feed_harvest_route_center_fence"),
        18.0,
        515.0,
        26.0,
    )
    .translate(0.0, 50.0, BASE_Z / 2.0 + 13.0);
    let feed_entry = centered_cube(name("base_feed_entry_direction_fence"), 208.0, 10.0, 20.0)
        .translate(-312.0, 88.0, BASE_Z / 2.0 + 10.0);
    let harvest_entry = centered_cube(
        name("base_harvest_entry_direction_fence"),
        208.0,
        10.0,
        20.0,
    )
    .translate(312.0, 348.0, BASE_Z / 2.0 + 10.0);
    let no_crossing_bar = centered_cube(name("base_no_crossing_route_bar"), 290.0, 12.0, 22.0)
        .translate(0.0, 88.0, BASE_Z / 2.0 + 11.0);
    vertical + feed_entry + harvest_entry + no_crossing_bar
}

fn base_robot_fiducials() -> Part {
    let mut fiducials = Part::empty(name("base_robot_fiducials"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 76.0, -STATION_Y / 2.0 + 76.0),
        (STATION_X / 2.0 - 76.0, -STATION_Y / 2.0 + 76.0),
        (-STATION_X / 2.0 + 76.0, STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 76.0, STATION_Y / 2.0 - 76.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            name(&format!("base_robot_fiducial_disc_{index}")),
            13.0,
            5.0,
            36,
        ) - centered_cylinder(
            name(&format!("base_robot_fiducial_center_dot_{index}")),
            3.4,
            7.0,
            24,
        );
        fiducials = fiducials + disc.translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    fiducials
}

fn base_leak_witness_ribs() -> Part {
    let mut ribs = Part::empty(name("base_leak_witness_ribs"));
    for index in 0..9 {
        ribs = ribs
            + centered_cube(
                name(&format!("base_front_leak_witness_rib_{index}")),
                72.0,
                5.0,
                6.0,
            )
            .translate(centered_index(index, 9, 84.0), -392.0, BASE_Z / 2.0 + 3.0);
    }
    ribs
}

fn keyed_port_nest(side: RouteSide) -> Part {
    let slug = side.slug();
    let body = centered_cube(
        name(&format!("{slug}_keyed_port_nest_body")),
        PORT_NEST_X,
        PORT_NEST_Y,
        PORT_NEST_Z,
    );
    let rear_datum = centered_cube(
        name(&format!("{slug}_keyed_port_rear_datum_fence")),
        PORT_NEST_X,
        13.0,
        PORT_NEST_Z + 30.0,
    )
    .translate(0.0, PORT_NEST_Y / 2.0 - 6.5, 15.0);
    let side_datum = centered_cube(
        name(&format!("{slug}_keyed_port_side_specific_datum_fence")),
        12.0,
        PORT_NEST_Y - 42.0,
        PORT_NEST_Z + 18.0,
    )
    .translate(side.datum_sign() * (PORT_NEST_X / 2.0 - 22.0), -10.0, 9.0);

    body + rear_datum + side_datum - port_socket_cuts(side) - datum_pin_cuts(side)
        + asymmetric_key_towers(side)
        + port_latch_lands(side)
        + route_identity_bars(side)
}

fn port_socket_cuts(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut cuts = Part::empty(name(&format!("{slug}_port_socket_cuts")));
    for index in 0..PORT_COUNT_PER_SIDE {
        let y = port_y(side, index);
        let bore = centered_cylinder(
            name(&format!("{slug}_port_socket_bore_{index}")),
            PORT_SOCKET_D / 2.0,
            PORT_NEST_X + 24.0,
            40,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, PORT_NEST_Z / 2.0 - 15.0);
        let flat = centered_cube(
            name(&format!("{slug}_port_socket_key_flat_{index}")),
            34.0,
            8.0 + index as f64 * 1.5,
            22.0,
        )
        .translate(
            side.datum_sign() * 44.0,
            y + side.key_offset_y() / 6.0,
            PORT_NEST_Z / 2.0 - 13.0,
        );
        let latch_window = centered_cube(
            name(&format!("{slug}_port_latch_window_relief_{index}")),
            26.0,
            16.0,
            22.0,
        )
        .translate(
            -side.datum_sign() * 66.0,
            y - side.key_offset_y() / 8.0,
            PORT_NEST_Z / 2.0 - 10.0,
        );
        cuts = cuts + bore + flat + latch_window;
    }
    cuts
}

fn datum_pin_cuts(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut cuts = Part::empty(name(&format!("{slug}_datum_pin_cuts")));
    for index in 0..DATUM_PIN_COUNT_PER_NEST {
        let x = side.datum_sign() * (112.0 - index as f64 * 34.0);
        let y = -PORT_NEST_Y / 2.0 + 24.0 + index as f64 * 34.0;
        cuts = cuts
            + centered_cylinder(
                name(&format!("{slug}_datum_pin_cut_{index}")),
                DATUM_PIN_D / 2.0,
                PORT_NEST_Z + 8.0,
                24,
            )
            .translate(x, y, 0.0);
    }
    cuts
}

fn asymmetric_key_towers(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut towers = Part::empty(name(&format!("{slug}_asymmetric_key_towers")));
    for index in 0..PORT_COUNT_PER_SIDE {
        let tower = centered_cube(
            name(&format!("{slug}_key_tower_{index}")),
            18.0 + index as f64 * 2.0,
            24.0,
            side.key_height(),
        )
        .translate(
            side.datum_sign() * (102.0 - index as f64 * 16.0),
            port_y(side, index) + side.key_offset_y() / 3.0,
            PORT_NEST_Z / 2.0 + side.key_height() / 2.0,
        );
        towers = towers + tower;
    }
    towers
}

fn port_latch_lands(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut lands = Part::empty(name(&format!("{slug}_port_latch_lands")));
    for index in 0..PORT_COUNT_PER_SIDE {
        lands = lands
            + centered_cube(
                name(&format!("{slug}_port_latch_land_{index}")),
                46.0,
                10.0,
                7.0,
            )
            .translate(
                -side.datum_sign() * 96.0,
                port_y(side, index),
                PORT_NEST_Z / 2.0 + 3.5,
            );
    }
    lands
}

fn route_identity_bars(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut bars = Part::empty(name(&format!("{slug}_route_identity_bars")));
    let count = match side {
        RouteSide::Feed => 2,
        RouteSide::Harvest => 3,
    };
    for index in 0..count {
        bars = bars
            + centered_cube(
                name(&format!("{slug}_raised_route_identity_bar_{index}")),
                74.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(index, count, 18.0),
                -PORT_NEST_Y / 2.0 + 22.0,
                PORT_NEST_Z / 2.0 + 4.0,
            );
    }
    bars
}

fn port_y(side: RouteSide, index: usize) -> f64 {
    centered_index(index, PORT_COUNT_PER_SIDE, side.port_pitch())
}

fn unidirectional_check_manifold() -> Part {
    let body = centered_cube(
        name("unidirectional_check_manifold_body"),
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let rear_guard = centered_cube(
        name("unidirectional_check_manifold_rear_no_reverse_guard"),
        MANIFOLD_X,
        14.0,
        MANIFOLD_Z + 22.0,
    )
    .translate(0.0, MANIFOLD_Y / 2.0 - 7.0, 11.0);

    body + rear_guard - manifold_flow_bores() - dead_leg_witness_wells()
        + check_valve_witness_bosses()
        + route_direction_arrow_bars()
        + manifold_no_cross_datum_wall()
}

fn manifold_flow_bores() -> Part {
    let mut bores = Part::empty(name("unidirectional_check_manifold_flow_bores"));
    for lane in 0..FLOW_LANES {
        let y = centered_index(lane, FLOW_LANES, MANIFOLD_LANE_PITCH_Y);
        let bore = centered_cylinder(
            name(&format!(
                "unidirectional_check_manifold_lane_{lane}_through_bore"
            )),
            TUBE_BORE_D / 2.0,
            MANIFOLD_X + 28.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, MANIFOLD_Z / 2.0 - 18.0);
        let inlet_counterbore = centered_cylinder(
            name(&format!(
                "unidirectional_check_manifold_lane_{lane}_feed_counterbore"
            )),
            8.0,
            30.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-MANIFOLD_X / 2.0 + 12.0, y, MANIFOLD_Z / 2.0 - 18.0);
        let outlet_counterbore = centered_cylinder(
            name(&format!(
                "unidirectional_check_manifold_lane_{lane}_harvest_counterbore"
            )),
            6.0,
            30.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(MANIFOLD_X / 2.0 - 12.0, y, MANIFOLD_Z / 2.0 - 18.0);
        bores = bores + bore + inlet_counterbore + outlet_counterbore;
    }
    bores
}

fn dead_leg_witness_wells() -> Part {
    let mut wells = Part::empty(name("unidirectional_check_manifold_dead_leg_witness_wells"));
    for lane in 0..DEAD_LEG_WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                name(&format!(
                    "unidirectional_check_manifold_dead_leg_well_{lane}"
                )),
                10.0,
                18.0,
                28,
            )
            .translate(
                0.0,
                centered_index(lane, DEAD_LEG_WITNESS_WELLS, MANIFOLD_LANE_PITCH_Y),
                MANIFOLD_Z / 2.0 - 7.0,
            );
    }
    wells
}

fn check_valve_witness_bosses() -> Part {
    let mut bosses = Part::empty(name("unidirectional_check_valve_witness_bosses"));
    for lane in 0..FLOW_LANES {
        let y = centered_index(lane, FLOW_LANES, MANIFOLD_LANE_PITCH_Y);
        for side in 0..CHECK_VALVES_PER_LANE {
            let x = if side == 0 { -72.0 } else { 72.0 };
            let boss = centered_cylinder(
                name(&format!(
                    "unidirectional_check_lane_{lane}_check_witness_boss_{side}"
                )),
                16.0,
                10.0,
                32,
            )
            .translate(x, y, MANIFOLD_Z / 2.0 + 5.0);
            let arrow_nib = centered_cube(
                name(&format!(
                    "unidirectional_check_lane_{lane}_check_direction_nib_{side}"
                )),
                18.0,
                8.0,
                8.0,
            )
            .translate(x + 20.0, y, MANIFOLD_Z / 2.0 + 9.0);
            bosses = bosses + boss + arrow_nib;
        }
    }
    bosses
}

fn route_direction_arrow_bars() -> Part {
    let mut bars = Part::empty(name("unidirectional_route_direction_arrow_bars"));
    for index in 0..ARROW_BAR_COUNT {
        let lane = index / 3;
        let step = index % 3;
        bars = bars
            + centered_cube(
                name(&format!("unidirectional_route_arrow_bar_{index}")),
                26.0 + step as f64 * 9.0,
                5.0,
                6.0,
            )
            .translate(
                -120.0 + step as f64 * 64.0,
                centered_index(lane, FLOW_LANES, MANIFOLD_LANE_PITCH_Y) + 16.0,
                MANIFOLD_Z / 2.0 + 3.0,
            );
    }
    bars
}

fn manifold_no_cross_datum_wall() -> Part {
    centered_cube(
        name("unidirectional_check_manifold_no_cross_datum_wall"),
        14.0,
        MANIFOLD_Y - 38.0,
        18.0,
    )
    .translate(0.0, 0.0, MANIFOLD_Z / 2.0 + 9.0)
}

fn reversal_challenge_coupon_sled() -> Part {
    let body = centered_cube(
        name("reversal_challenge_coupon_sled_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let rear_fence = centered_cube(
        name("reversal_challenge_coupon_rear_datum_fence"),
        COUPON_X,
        12.0,
        COUPON_Z + 20.0,
    )
    .translate(0.0, COUPON_Y / 2.0 - 6.0, 10.0);

    body + rear_fence - coupon_slots()
        + wrong_route_stop_blocks()
        + asymmetry_gauges()
        + coupon_identity_rails()
}

fn coupon_slots() -> Part {
    let mut slots = Part::empty(name("reversal_challenge_coupon_slots"));
    for index in 0..CHALLENGE_COUPONS {
        slots = slots
            + centered_cube(
                name(&format!("reversal_challenge_coupon_slot_{index}")),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                COUPON_Z + 4.0,
            )
            .translate(
                centered_index(index, CHALLENGE_COUPONS, COUPON_PITCH_X),
                -24.0 + if index % 2 == 0 { -11.0 } else { 11.0 },
                0.0,
            );
    }
    slots
}

fn wrong_route_stop_blocks() -> Part {
    let mut stops = Part::empty(name("reversal_challenge_wrong_route_stop_blocks"));
    for index in 0..WRONG_ROUTE_STOP_COUNT {
        stops = stops
            + centered_cube(
                name(&format!("reversal_challenge_wrong_route_stop_{index}")),
                20.0,
                18.0 + index as f64,
                28.0,
            )
            .translate(
                centered_index(index, WRONG_ROUTE_STOP_COUNT, COUPON_PITCH_X),
                48.0,
                COUPON_Z / 2.0 + 14.0,
            );
    }
    stops
}

fn asymmetry_gauges() -> Part {
    let mut gauges = Part::empty(name("reversal_challenge_asymmetry_gauges"));
    for index in 0..ASYMMETRY_GAUGE_COUNT {
        gauges = gauges
            + centered_cube(
                name(&format!(
                    "reversal_challenge_asymmetry_height_gauge_{index}"
                )),
                44.0,
                8.0,
                10.0 + index as f64 * 5.0,
            )
            .translate(
                -150.0 + index as f64 * 100.0,
                -COUPON_Y / 2.0 + 26.0,
                COUPON_Z / 2.0 + (10.0 + index as f64 * 5.0) / 2.0,
            );
    }
    gauges
}

fn coupon_identity_rails() -> Part {
    let feed = centered_cube(
        name("reversal_challenge_feed_coupon_identity_rail"),
        168.0,
        7.0,
        7.0,
    )
    .translate(-102.0, -78.0, COUPON_Z / 2.0 + 3.5);
    let harvest = centered_cube(
        name("reversal_challenge_harvest_coupon_identity_rail"),
        220.0,
        7.0,
        12.0,
    )
    .translate(86.0, -64.0, COUPON_Z / 2.0 + 6.0);
    feed + harvest
}

fn pressure_flow_witness_panel() -> Part {
    let body = centered_cube(
        name("pressure_flow_witness_panel_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let splash_lip = centered_cube(
        name("pressure_flow_witness_panel_splash_lip"),
        WITNESS_X,
        12.0,
        WITNESS_Z + 18.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 6.0, 9.0);

    body + splash_lip - pressure_sensor_pockets() - flow_sensor_windows() - bubble_windows()
        + witness_channel_ribs()
        + witness_sample_wells()
}

fn pressure_sensor_pockets() -> Part {
    let mut pockets = Part::empty(name("pressure_flow_pressure_sensor_pockets"));
    for lane in 0..PRESSURE_SENSOR_POCKETS {
        pockets = pockets
            + centered_cube(
                name(&format!("pressure_flow_pressure_sensor_pocket_{lane}")),
                58.0,
                28.0,
                24.0,
            )
            .translate(
                -96.0,
                centered_index(lane, PRESSURE_SENSOR_POCKETS, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 - 9.0,
            );
    }
    pockets
}

fn flow_sensor_windows() -> Part {
    let mut windows = Part::empty(name("pressure_flow_sensor_windows"));
    for lane in 0..FLOW_SENSOR_WINDOWS {
        windows = windows
            + centered_cube(
                name(&format!("pressure_flow_flow_sensor_window_{lane}")),
                56.0,
                18.0,
                18.0,
            )
            .translate(
                0.0,
                centered_index(lane, FLOW_SENSOR_WINDOWS, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 - 8.0,
            );
    }
    windows
}

fn bubble_windows() -> Part {
    let mut windows = Part::empty(name("pressure_flow_bubble_witness_windows"));
    for lane in 0..BUBBLE_WITNESS_WINDOWS {
        windows = windows
            + centered_cube(
                name(&format!("pressure_flow_bubble_window_{lane}")),
                48.0,
                24.0,
                18.0,
            )
            .translate(
                94.0,
                centered_index(lane, BUBBLE_WITNESS_WINDOWS, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 - 8.0,
            );
    }
    windows
}

fn witness_channel_ribs() -> Part {
    let mut ribs = Part::empty(name("pressure_flow_witness_channel_ribs"));
    for lane in 0..FLOW_LANES {
        ribs = ribs
            + centered_cube(
                name(&format!("pressure_flow_witness_channel_rib_{lane}")),
                WITNESS_X - 34.0,
                5.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(lane, FLOW_LANES, WITNESS_CHANNEL_PITCH_Y) + 18.0,
                WITNESS_Z / 2.0 + 4.0,
            );
    }
    ribs
}

fn witness_sample_wells() -> Part {
    let mut wells = Part::empty(name("pressure_flow_sample_witness_wells"));
    for lane in 0..FLOW_LANES {
        wells = wells
            + centered_cylinder(
                name(&format!("pressure_flow_sample_well_{lane}")),
                9.0,
                8.0,
                28,
            )
            .translate(
                144.0,
                centered_index(lane, FLOW_LANES, WITNESS_CHANNEL_PITCH_Y),
                WITNESS_Z / 2.0 + 4.0,
            );
    }
    wells
}

fn barcode_route_authority_bridge() -> Part {
    let left_post = centered_cube(
        name("barcode_route_authority_bridge_left_post"),
        SCAN_POST_W,
        SCAN_Y,
        SCAN_Z,
    )
    .translate(-SCAN_X / 2.0 + SCAN_POST_W / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        name("barcode_route_authority_bridge_right_post"),
        SCAN_POST_W,
        SCAN_Y,
        SCAN_Z,
    )
    .translate(SCAN_X / 2.0 - SCAN_POST_W / 2.0, 0.0, 0.0);
    let beam = centered_cube(
        name("barcode_route_authority_bridge_top_beam"),
        SCAN_X,
        SCAN_Y,
        SCAN_BEAM_Z,
    )
    .translate(0.0, 0.0, SCAN_Z / 2.0 - SCAN_BEAM_Z / 2.0);
    left_post
        + right_post
        + beam
        + barcode_lands()
        + route_authority_token_docks()
        + lot_event_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(name("barcode_route_authority_barcode_lands"));
    for index in 0..ROUTE_BARCODE_LANDS {
        lands = lands
            + centered_cube(
                name(&format!("barcode_route_authority_land_{index}")),
                50.0,
                18.0,
                6.0,
            )
            .translate(
                centered_index(index, ROUTE_BARCODE_LANDS, 44.0),
                -SCAN_Y / 2.0 + 24.0,
                -SCAN_Z / 2.0 + 18.0,
            );
    }
    lands
}

fn route_authority_token_docks() -> Part {
    let mut docks = Part::empty(name("barcode_route_authority_token_docks"));
    for index in 0..AUTHORITY_TOKEN_DOCKS {
        docks = docks
            + centered_cylinder(
                name(&format!("barcode_route_authority_token_dock_{index}")),
                10.0,
                7.0,
                28,
            )
            .translate(
                centered_index(index % 4, 4, 54.0),
                18.0 + (index / 4) as f64 * 38.0,
                -SCAN_Z / 2.0 + 20.0,
            );
    }
    docks
}

fn lot_event_lands() -> Part {
    let mut lands = Part::empty(name("barcode_route_authority_lot_event_lands"));
    for index in 0..LOT_EVENT_LANDS {
        lands = lands
            + centered_cube(
                name(&format!("barcode_route_authority_lot_event_land_{index}")),
                58.0,
                16.0,
                6.0,
            )
            .translate(
                centered_index(index, LOT_EVENT_LANDS, 66.0),
                SCAN_Y / 2.0 - 24.0,
                -SCAN_Z / 2.0 + 18.0,
            );
    }
    lands
}

fn tubing_strain_relief_comb() -> Part {
    let body = centered_cube(
        name("tubing_strain_relief_comb_body"),
        COMB_X,
        COMB_Y,
        COMB_Z,
    );
    let rear_fence = centered_cube(
        name("tubing_strain_relief_rear_feed_harvest_separator"),
        COMB_X,
        10.0,
        COMB_Z + 28.0,
    )
    .translate(0.0, COMB_Y / 2.0 - 5.0, 14.0);

    body + rear_fence - comb_slots(RouteSide::Feed) - comb_slots(RouteSide::Harvest)
        + line_separator_ribs()
        + comb_route_markers()
}

fn comb_slots(side: RouteSide) -> Part {
    let slug = side.slug();
    let mut slots = Part::empty(name(&format!("tubing_strain_relief_{slug}_slots")));
    let y = match side {
        RouteSide::Feed => -31.0,
        RouteSide::Harvest => 31.0,
    };
    let count = match side {
        RouteSide::Feed => COMB_FEED_SLOTS,
        RouteSide::Harvest => COMB_HARVEST_SLOTS,
    };
    for index in 0..count {
        slots = slots
            + centered_cube(
                name(&format!("tubing_strain_relief_{slug}_slot_{index}")),
                COMB_SLOT_X,
                COMB_SLOT_Y,
                COMB_Z + 4.0,
            )
            .translate(centered_index(index, count, COMB_SLOT_PITCH_X), y, 0.0);
    }
    slots
}

fn line_separator_ribs() -> Part {
    let mut ribs = Part::empty(name("tubing_strain_relief_line_separator_ribs"));
    for index in 0..LINE_SEPARATOR_RIBS {
        ribs = ribs
            + centered_cube(
                name(&format!("tubing_strain_relief_separator_rib_{index}")),
                7.0,
                COMB_Y - 28.0,
                10.0,
            )
            .translate(
                centered_index(index, LINE_SEPARATOR_RIBS, 66.0),
                0.0,
                COMB_Z / 2.0 + 5.0,
            );
    }
    ribs
}

fn comb_route_markers() -> Part {
    let feed = centered_cube(
        name("tubing_strain_relief_feed_marker_double_bar"),
        104.0,
        8.0,
        8.0,
    )
    .translate(-72.0, -COMB_Y / 2.0 + 21.0, COMB_Z / 2.0 + 4.0);
    let harvest = centered_cube(
        name("tubing_strain_relief_harvest_marker_triple_bar"),
        148.0,
        8.0,
        12.0,
    )
    .translate(72.0, COMB_Y / 2.0 - 21.0, COMB_Z / 2.0 + 6.0);
    feed + harvest
}

fn clamp_occlusion_interlock_rail() -> Part {
    let body = centered_cube(
        name("clamp_occlusion_interlock_rail_body"),
        CLAMP_X,
        CLAMP_Y,
        CLAMP_Z,
    );
    let guard = centered_cube(
        name("clamp_occlusion_no_manual_bypass_guard"),
        CLAMP_X,
        12.0,
        CLAMP_Z + 24.0,
    )
    .translate(0.0, CLAMP_Y / 2.0 - 6.0, 12.0);

    body + guard - clamp_occlusion_windows() + clamp_interlock_pins() + no_bypass_guard_tabs()
}

fn clamp_occlusion_windows() -> Part {
    let mut windows = Part::empty(name("clamp_occlusion_interlock_windows"));
    for index in 0..CLAMP_OCCLUSION_WINDOWS {
        windows = windows
            + centered_cube(
                name(&format!("clamp_occlusion_window_{index}")),
                23.0,
                32.0,
                CLAMP_Z + 4.0,
            )
            .translate(
                centered_index(index, CLAMP_OCCLUSION_WINDOWS, CLAMP_PITCH_X),
                0.0,
                0.0,
            );
    }
    windows
}

fn clamp_interlock_pins() -> Part {
    let mut pins = Part::empty(name("clamp_occlusion_interlock_pins"));
    for index in 0..INTERLOCK_CLAMPS {
        pins = pins
            + centered_cylinder(
                name(&format!("clamp_occlusion_interlock_pin_{index}")),
                6.0,
                16.0,
                24,
            )
            .translate(
                centered_index(index, INTERLOCK_CLAMPS, CLAMP_PITCH_X),
                if index % 2 == 0 { -45.0 } else { 45.0 },
                CLAMP_Z / 2.0 + 8.0,
            );
    }
    pins
}

fn no_bypass_guard_tabs() -> Part {
    let mut tabs = Part::empty(name("clamp_occlusion_no_bypass_guard_tabs"));
    for index in 0..NO_BYPASS_GUARDS {
        tabs = tabs
            + centered_cube(
                name(&format!("clamp_occlusion_no_bypass_guard_tab_{index}")),
                66.0,
                10.0,
                18.0,
            )
            .translate(
                centered_index(index, NO_BYPASS_GUARDS, 118.0),
                -CLAMP_Y / 2.0 + 20.0,
                CLAMP_Z / 2.0 + 9.0,
            );
    }
    tabs
}

fn reject_hold_release_gate() -> Part {
    let body = centered_cube(
        name("reject_hold_release_gate_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let dividers = disposition_dividers();

    body - gate_slots() + dividers + gate_state_markers()
}

fn gate_slots() -> Part {
    let mut slots = Part::empty(name("reject_hold_release_gate_slots"));
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..GATE_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    name(&format!("reject_hold_release_gate_lane_{lane}_slot_{slot}")),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    GATE_Z + 4.0,
                )
                .translate(
                    centered_index(slot, GATE_SLOTS_PER_LANE, 64.0),
                    centered_index(lane, DISPOSITION_LANES, GATE_PITCH_Y),
                    0.0,
                );
        }
    }
    slots
}

fn disposition_dividers() -> Part {
    let mut dividers = Part::empty(name("reject_hold_release_gate_dividers"));
    for lane in 0..=DISPOSITION_LANES {
        dividers = dividers
            + centered_cube(
                name(&format!("reject_hold_release_gate_divider_{lane}")),
                GATE_X - 28.0,
                5.0,
                12.0,
            )
            .translate(
                0.0,
                -GATE_Y / 2.0 + 22.0 + lane as f64 * GATE_PITCH_Y,
                GATE_Z / 2.0 + 6.0,
            );
    }
    dividers
}

fn gate_state_markers() -> Part {
    let reject = centered_cube(name("reject_hold_release_reject_marker"), 70.0, 8.0, 14.0)
        .translate(-108.0, -GATE_Y / 2.0 + 18.0, GATE_Z / 2.0 + 7.0);
    let hold = centered_cube(name("reject_hold_release_hold_marker"), 70.0, 8.0, 10.0).translate(
        0.0,
        -GATE_Y / 2.0 + 18.0,
        GATE_Z / 2.0 + 5.0,
    );
    let release = centered_cube(name("reject_hold_release_release_marker"), 70.0, 8.0, 6.0)
        .translate(108.0, -GATE_Y / 2.0 + 18.0, GATE_Z / 2.0 + 3.0);
    reject + hold + release
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        name("robot_service_front_robot_sweep_keepout_gauge"),
        STATION_X - 90.0,
        8.0,
        10.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, 0.0);
    let rear = centered_cube(
        name("robot_service_rear_service_keepout_gauge"),
        STATION_X - 90.0,
        8.0,
        10.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y, 0.0);
    let left = centered_cube(
        name("robot_service_left_line_load_keepout_gauge"),
        8.0,
        STATION_Y - 136.0,
        10.0,
    )
    .translate(-STATION_X / 2.0 + LEFT_LINE_LOAD_KEEP_OUT_X, 0.0, 0.0);
    let right = centered_cube(
        name("robot_service_right_line_load_keepout_gauge"),
        8.0,
        STATION_Y - 136.0,
        10.0,
    )
    .translate(STATION_X / 2.0 - RIGHT_LINE_LOAD_KEEP_OUT_X, 0.0, 0.0);

    front + rear + left + right + overhead_keepout_flags()
}

fn overhead_keepout_flags() -> Part {
    let mut flags = Part::empty(name("robot_service_overhead_keepout_flags"));
    for index in 0..KEEP_OUT_FLAGS {
        flags = flags
            + centered_cube(
                name(&format!("robot_service_overhead_bag_lift_flag_{index}")),
                28.0,
                8.0,
                64.0,
            )
            .translate(
                centered_index(index, KEEP_OUT_FLAGS, 148.0),
                STATION_Y / 2.0 - 74.0,
                OVERHEAD_BAG_LIFT_KEEP_OUT_Z / 5.0,
            );
    }
    flags
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn name(suffix: &str) -> String {
    format!("{PREFIX}_{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declares_expected_outputs_and_features() {
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn feed_and_harvest_keys_are_not_interchangeable() {
        assert_ne!(
            RouteSide::Feed.key_height(),
            RouteSide::Harvest.key_height()
        );
        assert_ne!(
            RouteSide::Feed.key_offset_y(),
            RouteSide::Harvest.key_offset_y()
        );
        assert_ne!(
            RouteSide::Feed.port_pitch(),
            RouteSide::Harvest.port_pitch()
        );
        assert!(RouteSide::Harvest.key_height() > RouteSide::Feed.key_height());
    }

    #[test]
    fn route_controls_cover_each_flow_lane() {
        assert_eq!(FLOW_LANES, PORT_COUNT_PER_SIDE);
        assert_eq!(CHECK_VALVE_WITNESSES, FLOW_LANES * CHECK_VALVES_PER_LANE);
        assert!(INTERLOCK_CLAMPS >= FLOW_LANES * 2);
        assert_eq!(PRESSURE_SENSOR_POCKETS, FLOW_LANES);
        assert_eq!(FLOW_SENSOR_WINDOWS, FLOW_LANES);
        assert_eq!(BUBBLE_WITNESS_WINDOWS, FLOW_LANES);
    }

    #[test]
    fn layout_is_inside_station_and_non_overlapping() {
        assert_design_constraints();
    }

    #[test]
    fn challenge_workflow_has_fail_closed_capacity() {
        assert!(WRONG_ROUTE_STOP_COUNT >= CHALLENGE_COUPONS);
        assert_eq!(DISPOSITION_LANES, 3);
        assert!(GATE_SLOTS_PER_LANE >= FLOW_LANES);
        assert!(AUTHORITY_TOKEN_DOCKS >= ROUTE_BARCODE_LANDS);
    }

    #[test]
    fn feed_and_harvest_zones_have_large_physical_gap() {
        assert!(feed_rect().horizontal_gap(harvest_rect()) >= MIN_FEED_HARVEST_ZONE_GAP);
    }
}
