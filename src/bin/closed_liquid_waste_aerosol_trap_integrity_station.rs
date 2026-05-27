use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed liquid-waste aerosol-trap integrity validation station.
//
// Intent:
// - Datum a sealed liquid-waste bottle and aerosol trap without opening the
//   waste path during pre-use or post-run integrity checks.
// - Package a hydrophobic vent filter holder, aerosol challenge bulkhead,
//   condensate sight path, leak basin, backpressure gauge bracket, and
//   quick-connect misroute guards into one mechanically keyed validation deck.
// - Make witness, service, and misroute-prevention features visible as CAD
//   geometry for downstream review. Purchased bottles, filters, gauges,
//   aerosol sources, sensors, and validation methods remain external.
//
// This is product-concept mechanical CAD only. It is not a validated aerosol
// challenge protocol, sterile barrier definition, or waste-treatment method.

const OUTPUTS: [&str; 10] = [
    "output/closed_liquid_waste_aerosol_trap_integrity_station_leak_basin_deck.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_closed_waste_bottle_nest.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_hydrophobic_vent_filter_holder.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_aerosol_challenge_port_panel.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_condensate_sight_path.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_backpressure_gauge_bracket.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_quick_connect_misroute_guards.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_tubing_witness_routes.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_robot_service_keepouts.stl",
    "output/closed_liquid_waste_aerosol_trap_integrity_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 7] = [
    "closed_waste_bottle_nest",
    "hydrophobic_vent_filter_holder",
    "aerosol_challenge_port",
    "condensate_sight_path",
    "leak_basin",
    "backpressure_gauge_bracket",
    "quick_connect_misroute_guards",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 740.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 52.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 16.0;

const BASIN_X: f64 = 980.0;
const BASIN_Y: f64 = 590.0;
const BASIN_DEPTH: f64 = 10.0;
const LEAK_SENSOR_WELLS: usize = 5;
const SPLASH_BAFFLES: usize = 5;

const BOTTLE_NEST_X: f64 = 390.0;
const BOTTLE_NEST_Y: f64 = 300.0;
const BOTTLE_NEST_Z: f64 = 112.0;
const BOTTLE_NEST_POS: (f64, f64) = (-300.0, 82.0);
const WASTE_BOTTLE_D: f64 = 154.0;
const WASTE_BOTTLE_SHADOW_Z: f64 = 232.0;
const BOTTLE_STRAPS: usize = 3;
const BOTTLE_LOCATOR_COUNT: usize = 4;

const FILTER_HOLDER_X: f64 = 382.0;
const FILTER_HOLDER_Y: f64 = 118.0;
const FILTER_HOLDER_Z: f64 = 158.0;
const FILTER_HOLDER_POS: (f64, f64) = (80.0, 226.0);
const FILTER_OD: f64 = 32.0;
const FILTER_LENGTH: f64 = 150.0;
const FILTER_CLIP_COUNT: usize = 3;

const CHALLENGE_PANEL_X: f64 = 340.0;
const CHALLENGE_PANEL_Y: f64 = 54.0;
const CHALLENGE_PANEL_Z: f64 = 172.0;
const CHALLENGE_PANEL_POS: (f64, f64) = (348.0, 214.0);
const CHALLENGE_PORTS: usize = 3;
const CHALLENGE_PORT_D: f64 = 18.0;

const SIGHT_PATH_X: f64 = 468.0;
const SIGHT_PATH_Y: f64 = 112.0;
const SIGHT_PATH_Z: f64 = 78.0;
const SIGHT_PATH_POS: (f64, f64) = (-160.0, -218.0);
const SIGHT_TICKS: usize = 9;
const CONDENSATE_WINDOW_X: f64 = 334.0;
const CONDENSATE_WINDOW_Z: f64 = 38.0;

const GAUGE_BRACKET_X: f64 = 214.0;
const GAUGE_BRACKET_Y: f64 = 118.0;
const GAUGE_BRACKET_Z: f64 = 192.0;
const GAUGE_BRACKET_POS: (f64, f64) = (372.0, -72.0);
const GAUGE_FACE_D: f64 = 82.0;
const GAUGE_GUARD_D: f64 = 118.0;
const GAUGE_TAP_COUNT: usize = 3;

const QC_GUARD_X: f64 = 412.0;
const QC_GUARD_Y: f64 = 142.0;
const QC_GUARD_Z: f64 = 102.0;
const QC_GUARD_POS: (f64, f64) = (270.0, -260.0);
const QC_PORTS: usize = 4;
const QC_PORT_D: f64 = 19.0;
const QC_PORT_PITCH: f64 = 86.0;
const QC_KEY_PROFILES: [(f64, f64, f64); QC_PORTS] = [
    (-16.0, 20.0, 0.0),
    (-6.0, 30.0, 90.0),
    (8.0, 42.0, -90.0),
    (18.0, 54.0, 180.0),
];

const ROUTE_X: f64 = 880.0;
const ROUTE_Y: f64 = 72.0;
const ROUTE_Z: f64 = 34.0;
const ROUTE_POS: (f64, f64) = (-22.0, -296.0);
const ROUTE_CHANNELS: usize = 6;
const TUBE_BORE_D: f64 = 6.4;

const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_OPERATOR_CLEARANCE: f64 = 300.0;
const REAR_AEROSOL_CART_CLEARANCE: f64 = 245.0;
const RIGHT_GAUGE_SERVICE_CLEARANCE: f64 = 210.0;
const TOP_BOTTLE_LIFT_CLEARANCE: f64 = 340.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let basin = leak_basin_deck();
    export(OUTPUTS[0], &basin);

    let bottle_nest = closed_waste_bottle_nest();
    export(OUTPUTS[1], &bottle_nest);

    let filter_holder = hydrophobic_vent_filter_holder();
    export(OUTPUTS[2], &filter_holder);

    let challenge_panel = aerosol_challenge_port_panel();
    export(OUTPUTS[3], &challenge_panel);

    let sight_path = condensate_sight_path();
    export(OUTPUTS[4], &sight_path);

    let gauge = backpressure_gauge_bracket();
    export(OUTPUTS[5], &gauge);

    let qc_guards = quick_connect_misroute_guards();
    export(OUTPUTS[6], &qc_guards);

    let routes = tubing_witness_routes();
    export(OUTPUTS[7], &routes);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[8], &keepouts);

    let assembly = basin
        + bottle_nest.translate(
            BOTTLE_NEST_POS.0,
            BOTTLE_NEST_POS.1,
            deck_mount_z(BOTTLE_NEST_Z),
        )
        + filter_holder.translate(
            FILTER_HOLDER_POS.0,
            FILTER_HOLDER_POS.1,
            deck_mount_z(FILTER_HOLDER_Z),
        )
        + challenge_panel.translate(
            CHALLENGE_PANEL_POS.0,
            CHALLENGE_PANEL_POS.1,
            deck_mount_z(CHALLENGE_PANEL_Z),
        )
        + sight_path.translate(
            SIGHT_PATH_POS.0,
            SIGHT_PATH_POS.1,
            deck_mount_z(SIGHT_PATH_Z),
        )
        + gauge.translate(
            GAUGE_BRACKET_POS.0,
            GAUGE_BRACKET_POS.1,
            deck_mount_z(GAUGE_BRACKET_Z),
        )
        + qc_guards.translate(QC_GUARD_POS.0, QC_GUARD_POS.1, deck_mount_z(QC_GUARD_Z))
        + routes.translate(ROUTE_POS.0, ROUTE_POS.1, deck_mount_z(ROUTE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + 3.0);
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed liquid-waste aerosol-trap integrity station:");
    println!(
        "  Containment deck:       {STATION_X:.0}mm x {STATION_Y:.0}mm, {BASIN_X:.0}mm x {BASIN_Y:.0}mm leak basin, {LEAK_SENSOR_WELLS} leak sensor wells, {DRAIN_PORT_D:.0}mm low-point drain"
    );
    println!(
        "  Bottle datum:           {WASTE_BOTTLE_D:.0}mm closed waste bottle shadow, {BOTTLE_STRAPS} strap bridges, {BOTTLE_LOCATOR_COUNT} locator bosses, lift envelope {WASTE_BOTTLE_SHADOW_Z:.0}mm"
    );
    println!(
        "  Aerosol trap checks:    hydrophobic vent filter holder with {FILTER_CLIP_COUNT} clips, {CHALLENGE_PORTS} challenge/sample/purge bulkhead ports, {SIGHT_TICKS} condensate sight ticks"
    );
    println!(
        "  Pressure controls:      {GAUGE_FACE_D:.0}mm backpressure gauge face, {GAUGE_TAP_COUNT} gauge/manifold taps, guarded overpressure relief pocket"
    );
    println!(
        "  Misroute prevention:    {QC_PORTS} quick-connect ports with unique keyed guards, {ROUTE_CHANNELS} witnessed tubing routes, separate waste/vent/challenge/drain corridors"
    );
    println!(
        "  Service envelopes:      {KEEP_OUT_ZONE_COUNT} keepout zones, {FRONT_OPERATOR_CLEARANCE:.0}mm front operator, {REAR_AEROSOL_CART_CLEARANCE:.0}mm rear aerosol cart, {RIGHT_GAUGE_SERVICE_CLEARANCE:.0}mm gauge service, {TOP_BOTTLE_LIFT_CLEARANCE:.0}mm bottle lift"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_mount_z(height: f64) -> f64 {
    BASE_Z + 4.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    assert_eq!(QC_KEY_PROFILES.len(), QC_PORTS);
    assert_eq!(CHALLENGE_PORTS, 3);
    assert!(ROUTE_CHANNELS >= QC_PORTS + 2);
    assert!(SIGHT_TICKS >= 7);
    assert!(FILTER_LENGTH > WASTE_BOTTLE_D * 0.85);

    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 7] {
    [
        (
            "closed_waste_bottle_nest",
            BOTTLE_NEST_POS,
            BOTTLE_NEST_X,
            BOTTLE_NEST_Y,
        ),
        (
            "hydrophobic_vent_filter_holder",
            FILTER_HOLDER_POS,
            FILTER_HOLDER_X,
            FILTER_HOLDER_Y,
        ),
        (
            "aerosol_challenge_port_panel",
            CHALLENGE_PANEL_POS,
            CHALLENGE_PANEL_X,
            CHALLENGE_PANEL_Y,
        ),
        (
            "condensate_sight_path",
            SIGHT_PATH_POS,
            SIGHT_PATH_X,
            SIGHT_PATH_Y,
        ),
        (
            "backpressure_gauge_bracket",
            GAUGE_BRACKET_POS,
            GAUGE_BRACKET_X,
            GAUGE_BRACKET_Y,
        ),
        (
            "quick_connect_misroute_guards",
            QC_GUARD_POS,
            QC_GUARD_X,
            QC_GUARD_Y,
        ),
        ("tubing_witness_routes", ROUTE_POS, ROUTE_X, ROUTE_Y),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 10.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 10.0
}

fn leak_basin_deck() -> Part {
    let deck = centered_cube(
        "liquid_waste_aerosol_trap_integrity_basin_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin_recess = centered_cube(
        "liquid_waste_aerosol_trap_integrity_leak_basin_recess",
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, -4.0, BASE_Z - BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_low_point_drain",
        DRAIN_PORT_D / 2.0,
        50.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 90.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 9.0);

    deck - basin_recess - drain - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + splash_baffles()
        + leak_sensor_wells()
        + datum_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("liquid_waste_aerosol_trap_integrity_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("liquid_waste_aerosol_trap_integrity_{name}_registration_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("liquid_waste_aerosol_trap_integrity_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 56.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 56.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 56.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-(STATION_X / 2.0 - 56.0), 0.0),
        (STATION_X / 2.0 - 56.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        let round = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_m6_slot_round_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 8.0,
            28,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_m6_slot_bridge_{i}"),
            26.0,
            MOUNT_HOLE_D + 0.8,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, BASE_Z / 2.0);
        slots = slots + round + slot;
    }
    slots
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "liquid_waste_aerosol_trap_integrity_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "liquid_waste_aerosol_trap_integrity_rear_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "liquid_waste_aerosol_trap_integrity_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "liquid_waste_aerosol_trap_integrity_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear_low_notch = centered_cube(
        "liquid_waste_aerosol_trap_integrity_aerosol_cart_low_notch",
        330.0,
        RIM_W + 2.0,
        24.0,
    )
    .translate(350.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z - 12.0);

    front + rear + left + right - rear_low_notch
}

fn splash_baffles() -> Part {
    let mut baffles = Part::empty("liquid_waste_aerosol_trap_integrity_splash_baffles");
    for i in 0..SPLASH_BAFFLES {
        let y = centered_index(i, SPLASH_BAFFLES, 92.0) - 4.0;
        let rib = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_basin_splash_baffle_{i}"),
            BASIN_X - 154.0,
            8.0,
            18.0,
        )
        .translate(0.0, y, BASE_Z + 9.0);
        let drain_gap = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_baffle_drain_gap_{i}"),
            84.0,
            10.0,
            20.0,
        )
        .translate(STATION_X / 2.0 - 154.0, y, BASE_Z + 9.0);
        baffles = baffles + (rib - drain_gap);
    }
    baffles
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("liquid_waste_aerosol_trap_integrity_leak_sensor_wells");
    for (i, (x, y)) in [
        (-420.0, -252.0),
        (-60.0, -284.0),
        (318.0, -254.0),
        (-428.0, 250.0),
        (424.0, 256.0),
    ]
    .iter()
    .enumerate()
    {
        let pocket = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_leak_sensor_pocket_{i}"),
            72.0,
            36.0,
            7.0,
        )
        .translate(*x, *y, BASE_Z + 3.5);
        let wire_chase = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_leak_sensor_wire_chase_{i}"),
            96.0,
            7.0,
            7.0,
        )
        .translate(*x, y + 28.0, BASE_Z + 3.5);
        wells = wells + pocket + wire_chase;
    }
    wells
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("liquid_waste_aerosol_trap_integrity_robot_fiducials");
    for (i, (x, y)) in [
        (-472.0, -286.0),
        (472.0, -286.0),
        (-472.0, 286.0),
        (472.0, 286.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!(
                "liquid_waste_aerosol_trap_integrity_deck_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z + 3.0);
    }
    fiducials
}

fn closed_waste_bottle_nest() -> Part {
    let tray = centered_cube(
        "liquid_waste_aerosol_trap_integrity_bottle_nest_tray",
        BOTTLE_NEST_X,
        BOTTLE_NEST_Y,
        BOTTLE_NEST_Z,
    );
    let bottle_socket = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_bottle_bottom_socket",
        WASTE_BOTTLE_D / 2.0 + 2.0,
        BOTTLE_NEST_Z + 8.0,
        72,
    )
    .translate(0.0, -16.0, 18.0);
    let drain_scallop = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_bottle_socket_drain_scallop",
        20.0,
        BOTTLE_NEST_X + 8.0,
        30,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        0.0,
        -BOTTLE_NEST_Y / 2.0 + 36.0,
        -BOTTLE_NEST_Z / 2.0 + 20.0,
    );
    let body = tray - bottle_socket - drain_scallop - bottle_nest_hand_slots();

    body + bottle_side_cradles()
        + bottle_strap_bridges()
        + bottle_locator_bosses()
        + bottle_neck_capture()
        + bottle_shadow_envelope()
}

fn bottle_nest_hand_slots() -> Part {
    let left = centered_cube(
        "liquid_waste_aerosol_trap_integrity_left_bottle_lift_relief",
        52.0,
        32.0,
        64.0,
    )
    .translate(
        -(WASTE_BOTTLE_D / 2.0 + 40.0),
        -18.0,
        BOTTLE_NEST_Z / 2.0 - 20.0,
    );
    let right = centered_cube(
        "liquid_waste_aerosol_trap_integrity_right_bottle_lift_relief",
        52.0,
        32.0,
        64.0,
    )
    .translate(
        WASTE_BOTTLE_D / 2.0 + 40.0,
        -18.0,
        BOTTLE_NEST_Z / 2.0 - 20.0,
    );
    left + right
}

fn bottle_side_cradles() -> Part {
    let left = centered_cube(
        "liquid_waste_aerosol_trap_integrity_left_bottle_hard_datum_rail",
        20.0,
        BOTTLE_NEST_Y - 78.0,
        96.0,
    )
    .translate(-(WASTE_BOTTLE_D / 2.0 + 24.0), -2.0, 16.0);
    let right = centered_cube(
        "liquid_waste_aerosol_trap_integrity_right_bottle_spring_datum_rail",
        20.0,
        BOTTLE_NEST_Y - 78.0,
        96.0,
    )
    .translate(WASTE_BOTTLE_D / 2.0 + 24.0, -2.0, 16.0);
    let rear_stop = centered_cube(
        "liquid_waste_aerosol_trap_integrity_rear_bottle_stop",
        WASTE_BOTTLE_D + 86.0,
        20.0,
        108.0,
    )
    .translate(0.0, WASTE_BOTTLE_D / 2.0 + 36.0, 24.0);
    let front_low_stop = centered_cube(
        "liquid_waste_aerosol_trap_integrity_front_low_bottle_stop",
        WASTE_BOTTLE_D + 72.0,
        16.0,
        46.0,
    )
    .translate(0.0, -(WASTE_BOTTLE_D / 2.0 + 50.0), -8.0);

    left + right + rear_stop + front_low_stop
}

fn bottle_strap_bridges() -> Part {
    let mut straps = Part::empty("liquid_waste_aerosol_trap_integrity_bottle_strap_bridges");
    for i in 0..BOTTLE_STRAPS {
        let z = -22.0 + i as f64 * 48.0;
        let bridge = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_bottle_strap_bridge_{i}"),
            WASTE_BOTTLE_D + 74.0,
            12.0,
            12.0,
        )
        .translate(0.0, -(WASTE_BOTTLE_D / 2.0 + 48.0), z);
        let latch_boss = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_bottle_strap_latch_boss_{i}"),
            28.0,
            20.0,
            18.0,
        )
        .translate(
            WASTE_BOTTLE_D / 2.0 + 52.0,
            -(WASTE_BOTTLE_D / 2.0 + 48.0),
            z,
        );
        straps = straps + bridge + latch_boss;
    }
    straps
}

fn bottle_locator_bosses() -> Part {
    let mut bosses = Part::empty("liquid_waste_aerosol_trap_integrity_bottle_locator_bosses");
    for (i, (x, y)) in [
        (-(WASTE_BOTTLE_D / 2.0 + 34.0), -88.0),
        (WASTE_BOTTLE_D / 2.0 + 34.0, -88.0),
        (-(WASTE_BOTTLE_D / 2.0 + 34.0), 76.0),
        (WASTE_BOTTLE_D / 2.0 + 34.0, 76.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_bottle_locator_boss_{i}"),
            13.0,
            14.0,
            28,
        )
        .translate(*x, *y, BOTTLE_NEST_Z / 2.0 + 7.0);
        let dowel = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_bottle_locator_dowel_hole_{i}"),
            3.2,
            18.0,
            20,
        )
        .translate(*x, *y, BOTTLE_NEST_Z / 2.0 + 7.0);
        bosses = bosses + (boss - dowel);
    }
    bosses
}

fn bottle_neck_capture() -> Part {
    let mast_l = centered_cube(
        "liquid_waste_aerosol_trap_integrity_left_bottle_neck_mast",
        22.0,
        24.0,
        122.0,
    )
    .translate(-58.0, 108.0, 74.0);
    let mast_r = centered_cube(
        "liquid_waste_aerosol_trap_integrity_right_bottle_neck_mast",
        22.0,
        24.0,
        122.0,
    )
    .translate(58.0, 108.0, 74.0);
    let yoke = ring(
        "liquid_waste_aerosol_trap_integrity_bottle_cap_yoke",
        92.0,
        49.0,
        18.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 108.0, 126.0);
    let cap_flat = centered_cube(
        "liquid_waste_aerosol_trap_integrity_bottle_cap_flat_key_relief",
        78.0,
        24.0,
        20.0,
    )
    .translate(0.0, 108.0, 126.0);

    mast_l + mast_r + (yoke - cap_flat)
}

fn bottle_shadow_envelope() -> Part {
    let hoop_bottom = ring(
        "liquid_waste_aerosol_trap_integrity_bottle_shadow_lower_hoop",
        WASTE_BOTTLE_D + 18.0,
        WASTE_BOTTLE_D + 8.0,
        8.0,
        72,
    )
    .translate(0.0, -16.0, 36.0);
    let hoop_top = ring(
        "liquid_waste_aerosol_trap_integrity_bottle_shadow_upper_hoop",
        WASTE_BOTTLE_D + 18.0,
        WASTE_BOTTLE_D + 8.0,
        8.0,
        72,
    )
    .translate(0.0, -16.0, WASTE_BOTTLE_SHADOW_Z - 30.0);
    let mut posts = Part::empty("liquid_waste_aerosol_trap_integrity_bottle_shadow_posts");
    for (i, (x, y)) in [
        (-(WASTE_BOTTLE_D / 2.0 + 9.0), -16.0),
        (WASTE_BOTTLE_D / 2.0 + 9.0, -16.0),
        (0.0, -(WASTE_BOTTLE_D / 2.0 + 25.0)),
        (0.0, WASTE_BOTTLE_D / 2.0 - 3.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("liquid_waste_aerosol_trap_integrity_bottle_shadow_post_{i}"),
                8.0,
                8.0,
                WASTE_BOTTLE_SHADOW_Z - 66.0,
            )
            .translate(*x, *y, (WASTE_BOTTLE_SHADOW_Z - 30.0) / 2.0 + 16.0);
    }
    hoop_bottom + hoop_top + posts
}

fn hydrophobic_vent_filter_holder() -> Part {
    let backplate = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_holder_backplate",
        FILTER_HOLDER_X,
        18.0,
        FILTER_HOLDER_Z,
    )
    .translate(0.0, FILTER_HOLDER_Y / 2.0 - 9.0, 0.0);
    let base_foot = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_holder_base_foot",
        FILTER_HOLDER_X,
        FILTER_HOLDER_Y,
        18.0,
    )
    .translate(0.0, 0.0, -FILTER_HOLDER_Z / 2.0 + 9.0);
    let cradle = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_holder_cradle_block",
        FILTER_LENGTH + 58.0,
        58.0,
        64.0,
    )
    .translate(0.0, 4.0, 12.0);
    let filter_channel = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_filter_holder_filter_channel",
        FILTER_OD / 2.0 + 1.5,
        FILTER_LENGTH + 72.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 4.0, 24.0);
    let snap_opening = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_holder_snap_opening",
        FILTER_LENGTH + 76.0,
        62.0,
        30.0,
    )
    .translate(0.0, -18.0, 48.0);

    let clips = filter_snap_clips();
    let ports = filter_end_ports();
    let drip_cup = filter_drip_cup();
    let witness_ribs = filter_orientation_witness_ribs();

    backplate
        + base_foot
        + (cradle - filter_channel - snap_opening)
        + clips
        + ports
        + drip_cup
        + witness_ribs
}

fn filter_snap_clips() -> Part {
    let mut clips = Part::empty("liquid_waste_aerosol_trap_integrity_filter_snap_clips");
    for i in 0..FILTER_CLIP_COUNT {
        let x = centered_index(i, FILTER_CLIP_COUNT, 74.0);
        let clip = ring(
            &format!("liquid_waste_aerosol_trap_integrity_filter_clip_ring_{i}"),
            FILTER_OD + 14.0,
            FILTER_OD + 3.0,
            12.0,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 4.0, 24.0);
        let snap_gap = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_filter_clip_snap_gap_{i}"),
            14.0,
            54.0,
            18.0,
        )
        .translate(x, -22.0, 44.0);
        clips = clips + (clip - snap_gap);
    }
    clips
}

fn filter_end_ports() -> Part {
    let inlet = tube_bulkhead(
        "liquid_waste_aerosol_trap_integrity_filter_inlet_bulkhead",
        -FILTER_LENGTH / 2.0 - 44.0,
        4.0,
        24.0,
        17.0,
    );
    let outlet = tube_bulkhead(
        "liquid_waste_aerosol_trap_integrity_filter_outlet_bulkhead",
        FILTER_LENGTH / 2.0 + 44.0,
        4.0,
        24.0,
        17.0,
    );
    let hydrophobic_membrane_disc = ring(
        "liquid_waste_aerosol_trap_integrity_filter_membrane_face",
        FILTER_OD + 18.0,
        FILTER_OD - 4.0,
        6.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(FILTER_LENGTH / 2.0 + 18.0, 4.0, 24.0);

    inlet + outlet + hydrophobic_membrane_disc
}

fn filter_drip_cup() -> Part {
    let cup = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_condensate_drip_cup",
        FILTER_LENGTH + 64.0,
        42.0,
        26.0,
    )
    .translate(0.0, -28.0, -30.0);
    let trough = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_condensate_trough",
        FILTER_LENGTH + 34.0,
        24.0,
        16.0,
    )
    .translate(0.0, -28.0, -22.0);
    let drain = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_filter_drip_cup_drain",
        5.0,
        46.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(FILTER_LENGTH / 2.0 + 18.0, -48.0, -31.0);
    cup - trough - drain
}

fn filter_orientation_witness_ribs() -> Part {
    let arrow_stem = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_flow_arrow_stem",
        128.0,
        7.0,
        8.0,
    )
    .translate(-18.0, -52.0, 66.0);
    let arrow_head = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_flow_arrow_head",
        32.0,
        18.0,
        8.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(66.0, -52.0, 66.0);
    let upstream_stop = centered_cube(
        "liquid_waste_aerosol_trap_integrity_filter_upstream_stop_land",
        18.0,
        18.0,
        68.0,
    )
    .translate(-FILTER_LENGTH / 2.0 - 38.0, -46.0, 16.0);

    arrow_stem + arrow_head + upstream_stop
}

fn aerosol_challenge_port_panel() -> Part {
    let panel = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_bulkhead_panel",
        CHALLENGE_PANEL_X,
        CHALLENGE_PANEL_Y,
        CHALLENGE_PANEL_Z,
    );
    let base_foot = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_panel_base_foot",
        CHALLENGE_PANEL_X,
        92.0,
        18.0,
    )
    .translate(0.0, -18.0, -CHALLENGE_PANEL_Z / 2.0 + 9.0);
    let holes = challenge_port_holes();
    let collars = challenge_port_collars();
    let cap_parks = challenge_cap_parks();
    let interlock = challenge_interlock_flag();
    let drip_shield = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_panel_drip_shield",
        CHALLENGE_PANEL_X - 36.0,
        16.0,
        16.0,
    )
    .translate(0.0, -CHALLENGE_PANEL_Y / 2.0 - 10.0, 44.0);

    (panel - holes) + base_foot + collars + cap_parks + interlock + drip_shield
}

fn challenge_port_holes() -> Part {
    let mut holes = Part::empty("liquid_waste_aerosol_trap_integrity_challenge_port_holes");
    for i in 0..CHALLENGE_PORTS {
        let x = centered_index(i, CHALLENGE_PORTS, 86.0);
        let d = if i == 0 {
            CHALLENGE_PORT_D
        } else if i == 1 {
            12.0
        } else {
            9.5
        };
        holes = holes
            + centered_cylinder(
                format!("liquid_waste_aerosol_trap_integrity_challenge_port_bore_{i}"),
                d / 2.0,
                CHALLENGE_PANEL_Y + 12.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 28.0);
    }
    holes
}

fn challenge_port_collars() -> Part {
    let mut collars = Part::empty("liquid_waste_aerosol_trap_integrity_challenge_port_collars");
    for i in 0..CHALLENGE_PORTS {
        let x = centered_index(i, CHALLENGE_PORTS, 86.0);
        let od = match i {
            0 => 42.0,
            1 => 34.0,
            _ => 28.0,
        };
        let id = match i {
            0 => CHALLENGE_PORT_D,
            1 => 12.0,
            _ => 9.5,
        };
        let collar = ring(
            &format!("liquid_waste_aerosol_trap_integrity_challenge_port_collar_{i}"),
            od,
            id + 1.0,
            10.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -CHALLENGE_PANEL_Y / 2.0 - 6.0, 28.0);
        let keyed_flat = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_challenge_port_key_flat_{i}"),
            14.0 + i as f64 * 4.0,
            12.0,
            6.0,
        )
        .translate(x, -CHALLENGE_PANEL_Y / 2.0 - 12.0, 28.0 + od / 2.0 - 6.0);
        collars = collars + collar + keyed_flat;
    }
    collars
}

fn challenge_cap_parks() -> Part {
    let mut parks = Part::empty("liquid_waste_aerosol_trap_integrity_challenge_cap_parks");
    for i in 0..CHALLENGE_PORTS {
        let x = centered_index(i, CHALLENGE_PORTS, 86.0);
        let pocket = ring(
            &format!("liquid_waste_aerosol_trap_integrity_challenge_cap_park_{i}"),
            30.0,
            18.0,
            9.0,
            32,
        )
        .translate(x, -8.0, 74.0);
        let tether_slot = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_challenge_cap_tether_slot_{i}"),
            5.0,
            48.0,
            9.0,
        )
        .translate(x + 22.0, -14.0, 74.0);
        parks = parks + pocket + tether_slot;
    }
    parks
}

fn challenge_interlock_flag() -> Part {
    let tower = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_interlock_tower",
        34.0,
        32.0,
        116.0,
    )
    .translate(CHALLENGE_PANEL_X / 2.0 - 34.0, -12.0, 4.0);
    let flag = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_interlock_flag",
        54.0,
        12.0,
        26.0,
    )
    .translate(CHALLENGE_PANEL_X / 2.0 - 55.0, -34.0, 62.0);
    let witness_slot = centered_cube(
        "liquid_waste_aerosol_trap_integrity_challenge_interlock_witness_slot",
        18.0,
        14.0,
        58.0,
    )
    .translate(CHALLENGE_PANEL_X / 2.0 - 34.0, -30.0, 10.0);

    tower + flag - witness_slot
}

fn condensate_sight_path() -> Part {
    let frame = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_sight_frame",
        SIGHT_PATH_X,
        SIGHT_PATH_Y,
        SIGHT_PATH_Z,
    );
    let window = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_sight_window_cutout",
        CONDENSATE_WINDOW_X,
        SIGHT_PATH_Y + 8.0,
        CONDENSATE_WINDOW_Z,
    )
    .translate(-26.0, 0.0, 9.0);
    let flow_channel = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_condensate_clear_tube_channel",
        10.0,
        SIGHT_PATH_X - 84.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-22.0, 0.0, -18.0);
    let low_point_cup = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_low_point_cup",
        72.0,
        72.0,
        34.0,
    )
    .translate(SIGHT_PATH_X / 2.0 - 52.0, 0.0, -30.0);
    let low_point_recess = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_condensate_low_point_recess",
        24.0,
        38.0,
        36,
    )
    .translate(SIGHT_PATH_X / 2.0 - 52.0, 0.0, -21.0);
    let drain = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_condensate_drain_to_basin",
        4.8,
        86.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SIGHT_PATH_X / 2.0 - 52.0, -SIGHT_PATH_Y / 2.0 - 8.0, -30.0);

    (frame - window - flow_channel)
        + sight_tick_marks()
        + (low_point_cup - low_point_recess - drain)
        + sight_path_guard_posts()
}

fn sight_tick_marks() -> Part {
    let mut ticks = Part::empty("liquid_waste_aerosol_trap_integrity_condensate_sight_ticks");
    for i in 0..SIGHT_TICKS {
        let x = -CONDENSATE_WINDOW_X / 2.0 + i as f64 * (CONDENSATE_WINDOW_X / 8.0) - 26.0;
        let tick_h = if i % 2 == 0 { 24.0 } else { 14.0 };
        ticks = ticks
            + centered_cube(
                format!("liquid_waste_aerosol_trap_integrity_condensate_tick_{i}"),
                4.0,
                8.0,
                tick_h,
            )
            .translate(x, -SIGHT_PATH_Y / 2.0 - 4.0, 30.0 - tick_h / 2.0);
    }
    ticks
}

fn sight_path_guard_posts() -> Part {
    let left = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_left_guard_post",
        18.0,
        22.0,
        SIGHT_PATH_Z + 32.0,
    )
    .translate(
        -(SIGHT_PATH_X / 2.0 - 38.0),
        -SIGHT_PATH_Y / 2.0 - 4.0,
        16.0,
    );
    let right = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_right_guard_post",
        18.0,
        22.0,
        SIGHT_PATH_Z + 32.0,
    )
    .translate(SIGHT_PATH_X / 2.0 - 38.0, -SIGHT_PATH_Y / 2.0 - 4.0, 16.0);
    let guard_rail = centered_cube(
        "liquid_waste_aerosol_trap_integrity_condensate_front_guard_rail",
        SIGHT_PATH_X - 84.0,
        14.0,
        12.0,
    )
    .translate(0.0, -SIGHT_PATH_Y / 2.0 - 4.0, 54.0);

    left + right + guard_rail
}

fn backpressure_gauge_bracket() -> Part {
    let foot = centered_cube(
        "liquid_waste_aerosol_trap_integrity_gauge_bracket_foot",
        GAUGE_BRACKET_X,
        GAUGE_BRACKET_Y,
        20.0,
    )
    .translate(0.0, 0.0, -GAUGE_BRACKET_Z / 2.0 + 10.0);
    let upright = centered_cube(
        "liquid_waste_aerosol_trap_integrity_gauge_bracket_upright",
        GAUGE_BRACKET_X - 50.0,
        18.0,
        GAUGE_BRACKET_Z,
    )
    .translate(0.0, GAUGE_BRACKET_Y / 2.0 - 16.0, 0.0);
    let gauge_cut = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_gauge_body_clearance",
        GAUGE_FACE_D / 2.0,
        28.0,
        56,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, GAUGE_BRACKET_Y / 2.0 - 18.0, 28.0);
    let gauge_guard = ring(
        "liquid_waste_aerosol_trap_integrity_gauge_front_guard_ring",
        GAUGE_GUARD_D,
        GAUGE_FACE_D + 6.0,
        12.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -GAUGE_BRACKET_Y / 2.0 - 8.0, 28.0);
    let gauge_face = ring(
        "liquid_waste_aerosol_trap_integrity_gauge_face_shadow",
        GAUGE_FACE_D,
        GAUGE_FACE_D - 10.0,
        5.0,
        56,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -GAUGE_BRACKET_Y / 2.0 - 16.0, 28.0);
    let manifold = gauge_tap_manifold();
    let relief = overpressure_relief_pocket();

    foot + (upright - gauge_cut) + gauge_guard + gauge_face + manifold + relief
}

fn gauge_tap_manifold() -> Part {
    let rail = centered_cube(
        "liquid_waste_aerosol_trap_integrity_gauge_tap_manifold_rail",
        168.0,
        28.0,
        32.0,
    )
    .translate(0.0, -24.0, -54.0);
    let mut taps = Part::empty("liquid_waste_aerosol_trap_integrity_gauge_taps");
    for i in 0..GAUGE_TAP_COUNT {
        let x = centered_index(i, GAUGE_TAP_COUNT, 58.0);
        let boss = tube_bulkhead(
            &format!("liquid_waste_aerosol_trap_integrity_gauge_tap_boss_{i}"),
            x,
            -44.0,
            -54.0,
            15.0,
        );
        let valve_land = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_gauge_tap_valve_land_{i}"),
            34.0,
            18.0,
            14.0,
        )
        .translate(x, -50.0, -28.0);
        taps = taps + boss + valve_land;
    }
    rail + taps
}

fn overpressure_relief_pocket() -> Part {
    let pocket_block = centered_cube(
        "liquid_waste_aerosol_trap_integrity_overpressure_relief_pocket_block",
        76.0,
        48.0,
        42.0,
    )
    .translate(GAUGE_BRACKET_X / 2.0 - 52.0, -24.0, 72.0);
    let pocket = centered_cylinder(
        "liquid_waste_aerosol_trap_integrity_overpressure_relief_socket",
        13.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(GAUGE_BRACKET_X / 2.0 - 52.0, -24.0, 72.0);
    let exhaust_slot = centered_cube(
        "liquid_waste_aerosol_trap_integrity_overpressure_exhaust_slot",
        62.0,
        54.0,
        10.0,
    )
    .translate(GAUGE_BRACKET_X / 2.0 - 52.0, -26.0, 94.0);

    pocket_block - pocket - exhaust_slot
}

fn quick_connect_misroute_guards() -> Part {
    let panel = centered_cube(
        "liquid_waste_aerosol_trap_integrity_qc_guard_panel",
        QC_GUARD_X,
        QC_GUARD_Y,
        QC_GUARD_Z,
    );
    let front_cut = centered_cube(
        "liquid_waste_aerosol_trap_integrity_qc_guard_front_hand_clearance",
        QC_GUARD_X - 54.0,
        28.0,
        QC_GUARD_Z - 26.0,
    )
    .translate(0.0, -QC_GUARD_Y / 2.0 + 14.0, 14.0);
    let ports = quick_connect_port_holes();
    let guards = quick_connect_guard_collars();
    let key_blocks = quick_connect_key_blocks();
    let barriers = quick_connect_cross_route_barriers();
    let latch_pockets = quick_connect_latch_pockets();

    (panel - front_cut - ports) + guards + key_blocks + barriers + latch_pockets
}

fn quick_connect_port_holes() -> Part {
    let mut holes = Part::empty("liquid_waste_aerosol_trap_integrity_qc_port_holes");
    for i in 0..QC_PORTS {
        let x = centered_index(i, QC_PORTS, QC_PORT_PITCH);
        let port = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_qc_port_bore_{i}"),
            QC_PORT_D / 2.0,
            QC_GUARD_Y + 10.0,
            34,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 12.0);
        let flat = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_qc_port_key_flat_cut_{i}"),
            8.0 + i as f64 * 3.0,
            QC_GUARD_Y + 12.0,
            10.0,
        )
        .rotate(0.0, 0.0, QC_KEY_PROFILES[i].2)
        .translate(
            x + QC_KEY_PROFILES[i].0,
            0.0,
            12.0 + QC_KEY_PROFILES[i].1 / 4.0,
        );
        holes = holes + port + flat;
    }
    holes
}

fn quick_connect_guard_collars() -> Part {
    let mut collars = Part::empty("liquid_waste_aerosol_trap_integrity_qc_guard_collars");
    for i in 0..QC_PORTS {
        let x = centered_index(i, QC_PORTS, QC_PORT_PITCH);
        let od = 42.0 + i as f64 * 5.0;
        let collar = ring(
            &format!("liquid_waste_aerosol_trap_integrity_qc_guard_collar_{i}"),
            od,
            QC_PORT_D + 1.0,
            12.0,
            42,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -QC_GUARD_Y / 2.0 - 8.0, 12.0);
        let finger_shield = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_qc_guard_finger_shield_{i}"),
            od + 18.0,
            10.0,
            16.0,
        )
        .translate(x, -QC_GUARD_Y / 2.0 - 14.0, -20.0);
        collars = collars + collar + finger_shield;
    }
    collars
}

fn quick_connect_key_blocks() -> Part {
    let mut keys = Part::empty("liquid_waste_aerosol_trap_integrity_qc_key_blocks");
    for i in 0..QC_PORTS {
        let x = centered_index(i, QC_PORTS, QC_PORT_PITCH);
        let (_, height, rot) = QC_KEY_PROFILES[i];
        let key = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_qc_unique_key_block_{i}"),
            16.0 + i as f64 * 4.0,
            22.0,
            height,
        )
        .rotate(0.0, 0.0, rot)
        .translate(x, -QC_GUARD_Y / 2.0 - 20.0, 44.0);
        let pin = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_qc_misroute_pin_{i}"),
            4.0,
            28.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x + 26.0 - i as f64 * 7.0,
            -QC_GUARD_Y / 2.0 - 20.0,
            -2.0 + i as f64 * 9.0,
        );
        keys = keys + key + pin;
    }
    keys
}

fn quick_connect_cross_route_barriers() -> Part {
    let mut barriers = Part::empty("liquid_waste_aerosol_trap_integrity_qc_cross_route_barriers");
    for i in 0..(QC_PORTS - 1) {
        let x = (centered_index(i, QC_PORTS, QC_PORT_PITCH)
            + centered_index(i + 1, QC_PORTS, QC_PORT_PITCH))
            / 2.0;
        barriers = barriers
            + centered_cube(
                format!("liquid_waste_aerosol_trap_integrity_qc_between_port_barrier_{i}"),
                12.0,
                QC_GUARD_Y + 36.0,
                QC_GUARD_Z + 18.0,
            )
            .translate(x, -6.0, 8.0);
    }
    barriers
}

fn quick_connect_latch_pockets() -> Part {
    let mut pockets = Part::empty("liquid_waste_aerosol_trap_integrity_qc_latch_pockets");
    for i in 0..QC_PORTS {
        let x = centered_index(i, QC_PORTS, QC_PORT_PITCH);
        pockets = pockets
            + centered_cube(
                format!("liquid_waste_aerosol_trap_integrity_qc_latch_keeper_{i}"),
                46.0,
                12.0,
                16.0,
            )
            .translate(x, QC_GUARD_Y / 2.0 + 2.0, 44.0);
    }
    pockets
}

fn tubing_witness_routes() -> Part {
    let base = centered_cube(
        "liquid_waste_aerosol_trap_integrity_tubing_route_witness_base",
        ROUTE_X,
        ROUTE_Y,
        ROUTE_Z,
    );
    let mut channels = Part::empty("liquid_waste_aerosol_trap_integrity_tubing_route_channels");
    let mut raised_keys = Part::empty("liquid_waste_aerosol_trap_integrity_tubing_route_keys");
    for i in 0..ROUTE_CHANNELS {
        let x = centered_index(i, ROUTE_CHANNELS, 132.0);
        let bore = centered_cylinder(
            format!("liquid_waste_aerosol_trap_integrity_tubing_route_bore_{i}"),
            TUBE_BORE_D / 2.0,
            ROUTE_Y + 10.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 7.0);
        let open_slot = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_tubing_route_open_slot_{i}"),
            TUBE_BORE_D + 2.0,
            ROUTE_Y + 12.0,
            20.0,
        )
        .translate(x, 0.0, 19.0);
        let route_key = centered_cube(
            format!("liquid_waste_aerosol_trap_integrity_tubing_route_unique_height_key_{i}"),
            52.0,
            18.0,
            8.0 + i as f64 * 3.0,
        )
        .translate(
            x,
            -ROUTE_Y / 2.0 + 18.0,
            ROUTE_Z / 2.0 + 4.0 + i as f64 * 1.5,
        );
        channels = channels + bore + open_slot;
        raised_keys = raised_keys + route_key;
    }
    let split_barrier = centered_cube(
        "liquid_waste_aerosol_trap_integrity_tubing_waste_vent_challenge_segregation_wall",
        ROUTE_X - 80.0,
        10.0,
        52.0,
    )
    .translate(0.0, 0.0, ROUTE_Z / 2.0 + 26.0);
    let drip_gutter = centered_cube(
        "liquid_waste_aerosol_trap_integrity_tubing_route_front_drip_gutter",
        ROUTE_X - 62.0,
        18.0,
        12.0,
    )
    .translate(0.0, -ROUTE_Y / 2.0 + 11.0, -ROUTE_Z / 2.0 + 7.0);

    (base - channels) + raised_keys + split_barrier + drip_gutter
}

fn robot_service_keepouts() -> Part {
    let front = keepout_frame(
        "liquid_waste_aerosol_trap_integrity_front_operator_keepout",
        STATION_X - 120.0,
        FRONT_OPERATOR_CLEARANCE,
        122.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_OPERATOR_CLEARANCE / 2.0, 60.0);
    let rear = keepout_frame(
        "liquid_waste_aerosol_trap_integrity_rear_aerosol_cart_keepout",
        STATION_X - 280.0,
        REAR_AEROSOL_CART_CLEARANCE,
        156.0,
    )
    .translate(
        88.0,
        STATION_Y / 2.0 + REAR_AEROSOL_CART_CLEARANCE / 2.0,
        78.0,
    );
    let gauge = keepout_frame(
        "liquid_waste_aerosol_trap_integrity_right_gauge_service_keepout",
        RIGHT_GAUGE_SERVICE_CLEARANCE,
        330.0,
        170.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_GAUGE_SERVICE_CLEARANCE / 2.0,
        -70.0,
        86.0,
    );
    let bottle_lift = keepout_frame(
        "liquid_waste_aerosol_trap_integrity_top_bottle_lift_keepout",
        310.0,
        300.0,
        TOP_BOTTLE_LIFT_CLEARANCE,
    )
    .translate(
        BOTTLE_NEST_POS.0,
        BOTTLE_NEST_POS.1,
        BASE_Z + TOP_BOTTLE_LIFT_CLEARANCE / 2.0,
    );
    let filter_service = keepout_frame(
        "liquid_waste_aerosol_trap_integrity_filter_pull_service_keepout",
        430.0,
        150.0,
        146.0,
    )
    .translate(FILTER_HOLDER_POS.0, FILTER_HOLDER_POS.1 + 118.0, 78.0);

    front + rear + gauge + bottle_lift + filter_service
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let rail = 7.0;
    let bottom =
        centered_cube(format!("{name}_bottom_frame"), x, y, rail).translate(0.0, 0.0, -z / 2.0);
    let top = centered_cube(format!("{name}_top_frame"), x, y, rail).translate(0.0, 0.0, z / 2.0);
    let left =
        centered_cube(format!("{name}_left_frame"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right =
        centered_cube(format!("{name}_right_frame"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    let front =
        centered_cube(format!("{name}_front_frame"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_frame"), x, rail, z).translate(0.0, y / 2.0, 0.0);

    bottom + top + left + right + front + rear
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 14.0, 3.0, 36);
    let inner = centered_cylinder(format!("{name}_inner_dot"), 5.0, 4.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 34.0, 3.2, 3.2);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.2, 34.0, 3.2);
    outer + inner + cross_x + cross_y
}

fn ring(name: &str, outer_d: f64, inner_d: f64, height: f64, segments: u32) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, height, segments)
        - centered_cylinder(
            format!("{name}_inner"),
            inner_d / 2.0,
            height + 2.0,
            segments,
        )
}

fn tube_bulkhead(name: &str, x: f64, y: f64, z: f64, od: f64) -> Part {
    let boss = ring(&format!("{name}_boss"), od + 14.0, od, 12.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z);
    let wrench_flat = centered_cube(format!("{name}_wrench_flat"), od + 18.0, 6.0, 9.0).translate(
        x,
        y - 8.0,
        z + od / 2.0,
    );
    boss + wrench_flat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_covers_station_features() {
        assert_eq!(OUTPUTS.len(), 10);
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "{feature} missing from output manifest"
            );
        }
    }

    #[test]
    fn layout_constraints_hold() {
        assert_layout();
    }

    #[test]
    fn quick_connect_keys_are_one_to_one_with_ports() {
        assert_eq!(QC_KEY_PROFILES.len(), QC_PORTS);
        assert_eq!(QC_PORTS, 4);
    }

    #[test]
    fn service_keepout_manifest_matches_geometry_count() {
        assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
    }
}
