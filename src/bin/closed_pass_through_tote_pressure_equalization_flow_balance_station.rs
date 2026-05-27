use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed pass-through tote pressure equalization and purge-flow balance station.
//
// Intent:
// - Validate a sealed pass-through tote receiver before it is allowed to move
//   closed materials between workcell environments.
// - Represent pressure equalization, HEPA purge balance, decay tokens, release
//   routing, and evidence capture as mechanical CAD surrogates only.
// - Keep robot access and service withdrawal envelopes visible in the exported
//   assembly so the fixture can be checked against adjacent station concepts.

const OUTPUT_PREFIX: &str =
    "output/closed_pass_through_tote_pressure_equalization_flow_balance_station";
const OUTPUTS: [&str; 12] = [
    "base_service_deck",
    "sealed_tote_receiver",
    "dual_pressure_port_panel",
    "hepa_purge_duct_surrogate",
    "flow_witness_vanes",
    "door_seal_coupon_array",
    "pressure_decay_token_lane",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
    "assembly",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_tote_receiver",
    "dual_pressure_port_panel",
    "hepa_purge_duct_surrogate",
    "flow_witness_vanes",
    "door_seal_coupons",
    "pressure_decay_token_lane",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const BASE_X: f64 = 1380.0;
const BASE_Y: f64 = 930.0;
const BASE_Z: f64 = 22.0;
const BASE_RAIL_Z: f64 = 30.0;
const BASE_RAIL_W: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.6;

const TOTE_INNER_X: f64 = 560.0;
const TOTE_INNER_Y: f64 = 410.0;
const TOTE_INNER_Z: f64 = 258.0;
const TOTE_WALL: f64 = 22.0;
const TOTE_OUTER_X: f64 = TOTE_INNER_X + 2.0 * TOTE_WALL;
const TOTE_OUTER_Y: f64 = TOTE_INNER_Y + 2.0 * TOTE_WALL;
const TOTE_OUTER_Z: f64 = TOTE_INNER_Z + 56.0;
const RECEIVER_CLEARANCE: f64 = 8.0;
const RECEIVER_RAIL_W: f64 = 28.0;
const RECEIVER_RAIL_Z: f64 = 42.0;
const LOCATOR_COUNT: usize = 4;
const LATCH_COUNT: usize = 4;

const PANEL_X: f64 = 960.0;
const PANEL_Y: f64 = 30.0;
const PANEL_Z: f64 = 360.0;
const PANEL_BASE_Y: f64 = BASE_Y / 2.0 - 56.0;
const PRESSURE_BANKS: usize = 2;
const PORTS_PER_BANK: usize = 6;
const PRESSURE_PORTS: usize = PRESSURE_BANKS * PORTS_PER_BANK;
const PORT_D: f64 = 10.4;
const GAUGE_D: f64 = 44.0;

const DUCT_X: f64 = 690.0;
const DUCT_Y: f64 = 178.0;
const DUCT_Z: f64 = 116.0;
const DUCT_CHANNEL_X: f64 = 608.0;
const DUCT_CHANNEL_Z: f64 = 58.0;
const HEPA_CASSETTE_X: f64 = 300.0;
const HEPA_CASSETTE_Y: f64 = 84.0;
const HEPA_CASSETTE_Z: f64 = 132.0;
const DIFFUSER_HOLES_X: usize = 8;
const DIFFUSER_HOLES_Z: usize = 3;

const WITNESS_X: f64 = 740.0;
const WITNESS_Y: f64 = 188.0;
const WITNESS_Z: f64 = 18.0;
const WITNESS_ROWS: usize = 2;
const VANES_PER_ROW: usize = 7;
const WITNESS_VANES: usize = WITNESS_ROWS * VANES_PER_ROW;
const VANE_X: f64 = 6.0;
const VANE_Y: f64 = 56.0;
const VANE_Z: f64 = 46.0;

const COUPON_COUNT: usize = 6;
const COUPON_X: f64 = 122.0;
const COUPON_Y: f64 = 64.0;
const COUPON_Z: f64 = 9.0;
const COUPON_TRAY_X: f64 = 470.0;
const COUPON_TRAY_Y: f64 = 190.0;
const COUPON_TRAY_Z: f64 = 14.0;

const TOKEN_COUNT: usize = 8;
const TOKEN_LANE_X: f64 = 640.0;
const TOKEN_LANE_Y: f64 = 128.0;
const TOKEN_LANE_Z: f64 = 16.0;
const TOKEN_D: f64 = 42.0;
const TOKEN_Z: f64 = 8.0;

const BARCODE_LANDS: usize = 5;
const CERTIFICATE_LANDS: usize = 2;
const RECORD_X: f64 = 560.0;
const RECORD_Y: f64 = 156.0;
const RECORD_Z: f64 = 10.0;

const STATUS_LANES: usize = 3;
const STATUS_LANE_X: f64 = 238.0;
const STATUS_LANE_Y: f64 = 158.0;
const STATUS_LANE_Z: f64 = 12.0;
const STATUS_RAIL_Z: f64 = 34.0;

const BRIDGE_X: f64 = 690.0;
const BRIDGE_Y: f64 = 176.0;
const BRIDGE_Z: f64 = 285.0;
const BRIDGE_RAIL: f64 = 18.0;

const KEEP_OUT_RAIL: f64 = 12.0;
const ROBOT_KEEP_OUT_Z: f64 = 250.0;
const DOOR_SWING_KEEP_OUT_Y: f64 = 390.0;
const FILTER_PULL_KEEP_OUT_Y: f64 = 260.0;
const SERVICE_KEEP_OUT_X: f64 = 250.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_service_deck();
    export(OUTPUTS[0], &base);

    let receiver = sealed_tote_receiver();
    export(OUTPUTS[1], &receiver);

    let panel = dual_pressure_port_panel();
    export(OUTPUTS[2], &panel);

    let duct = hepa_purge_duct_surrogate();
    export(OUTPUTS[3], &duct);

    let vanes = flow_witness_vanes();
    export(OUTPUTS[4], &vanes);

    let coupons = door_seal_coupon_array();
    export(OUTPUTS[5], &coupons);

    let tokens = pressure_decay_token_lane();
    export(OUTPUTS[6], &tokens);

    let records = barcode_certificate_lands();
    export(OUTPUTS[7], &records);

    let status_lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &status_lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + receiver.translate(-220.0, -40.0, BASE_Z)
        + panel.translate(0.0, PANEL_BASE_Y, BASE_Z)
        + duct.translate(-210.0, PANEL_BASE_Y - 86.0, BASE_Z + PANEL_Z - 92.0)
        + vanes.translate(-220.0, -BASE_Y / 2.0 + 258.0, BASE_Z)
        + coupons.translate(BASE_X / 2.0 - 300.0, -BASE_Y / 2.0 + 250.0, BASE_Z)
        + tokens.translate(BASE_X / 2.0 - 370.0, 92.0, BASE_Z)
        + records.translate(-BASE_X / 2.0 + 360.0, -BASE_Y / 2.0 + 92.0, BASE_Z)
        + status_lanes.translate(138.0, -BASE_Y / 2.0 + 98.0, BASE_Z)
        + bridge.translate(
            -BASE_X / 2.0 + 360.0,
            -BASE_Y / 2.0 + 92.0,
            BASE_Z + RECORD_Z,
        )
        + keepouts.translate(0.0, 0.0, BASE_Z);

    export(OUTPUTS[11], &assembly);

    println!(
        "Closed pass-through tote equalization station: {:.0} x {:.0} mm deck, {:.0} x {:.0} x {:.0} mm sealed tote envelope, {} pressure ports, {} HEPA diffuser holes, {} flow witness vanes, {} seal coupons, {} decay tokens, {} barcode/certificate lands, {} release/hold/reject lanes, {} receiver latch pockets, and {} requested feature groups.",
        BASE_X,
        BASE_Y,
        TOTE_OUTER_X,
        TOTE_OUTER_Y,
        TOTE_OUTER_Z,
        PRESSURE_PORTS,
        DIFFUSER_HOLES_X * DIFFUSER_HOLES_Z,
        WITNESS_VANES,
        COUPON_COUNT,
        TOKEN_COUNT,
        BARCODE_LANDS + CERTIFICATE_LANDS,
        STATUS_LANES,
        LATCH_COUNT,
        REQUIRED_FEATURES.len()
    );
}

fn export(name: &str, part: &Part) {
    let path = format!("{OUTPUT_PREFIX}_{name}.stl");
    part.write_stl(&path).unwrap();
    println!("Exported: {path}");
}

fn base_service_deck() -> Part {
    let deck = centered_cube(
        "closed_pass_tote_equalization_base_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_workfield = centered_cube(
        "closed_pass_tote_equalization_recessed_workfield_cut",
        BASE_X - 132.0,
        BASE_Y - 136.0,
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z - 3.0);
    let receiver_socket = centered_cube(
        "closed_pass_tote_equalization_receiver_socket_cut",
        TOTE_OUTER_X + 162.0,
        TOTE_OUTER_Y + 138.0,
        7.0,
    )
    .translate(-220.0, -40.0, BASE_Z - 2.0);
    let panel_socket = centered_cube(
        "closed_pass_tote_equalization_pressure_panel_socket_cut",
        PANEL_X + 34.0,
        16.0,
        8.0,
    )
    .translate(0.0, PANEL_BASE_Y, BASE_Z - 2.0);
    let purge_sump = centered_cube(
        "closed_pass_tote_equalization_front_purge_condensate_sump_cut",
        BASE_X - 170.0,
        76.0,
        9.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 70.0, BASE_Z - 3.0);
    let drain = centered_cylinder(
        "closed_pass_tote_equalization_sump_drain_cut",
        8.4 / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 92.0, -BASE_Y / 2.0 + 35.0, BASE_Z / 2.0);

    deck - recessed_workfield - receiver_socket - panel_socket - purge_sump - drain
        + base_perimeter_rails()
        + base_mounting_bosses()
        + robot_fiducials()
        + panel_gussets()
        + purge_drain_guard()
}

fn base_perimeter_rails() -> Part {
    let left = centered_cube(
        "closed_pass_tote_equalization_left_base_rail",
        BASE_RAIL_W,
        BASE_Y - 62.0,
        BASE_RAIL_Z,
    )
    .translate(-BASE_X / 2.0 + 34.0, 0.0, BASE_Z + BASE_RAIL_Z / 2.0);
    let right = centered_cube(
        "closed_pass_tote_equalization_right_base_rail",
        BASE_RAIL_W,
        BASE_Y - 62.0,
        BASE_RAIL_Z,
    )
    .translate(BASE_X / 2.0 - 34.0, 0.0, BASE_Z + BASE_RAIL_Z / 2.0);
    let rear = centered_cube(
        "closed_pass_tote_equalization_rear_base_rail",
        BASE_X - 80.0,
        BASE_RAIL_W,
        BASE_RAIL_Z,
    )
    .translate(0.0, BASE_Y / 2.0 - 34.0, BASE_Z + BASE_RAIL_Z / 2.0);
    let front_low_lip = centered_cube(
        "closed_pass_tote_equalization_front_low_purge_lip",
        BASE_X - 130.0,
        14.0,
        20.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 28.0, BASE_Z + 10.0);

    left + right + rear + front_low_lip
}

fn base_mounting_bosses() -> Part {
    let mut bosses = Part::empty("closed_pass_tote_equalization_base_mounting_bosses");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_pass_tote_equalization_mount_boss_{i}"),
            18.0,
            8.0,
            32,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        let hole = centered_cylinder(
            format!("closed_pass_tote_equalization_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            12.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        let slot = centered_cube(
            format!("closed_pass_tote_equalization_m6_slot_{i}"),
            25.0,
            MOUNT_HOLE_D + 0.4,
            12.0,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        bosses = bosses + (boss - hole - slot);
    }
    bosses
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_pass_tote_equalization_robot_fiducials");
    for (i, (x, y)) in [
        (-BASE_X / 2.0 + 78.0, BASE_Y / 2.0 - 78.0),
        (BASE_X / 2.0 - 78.0, BASE_Y / 2.0 - 78.0),
        (-BASE_X / 2.0 + 78.0, -BASE_Y / 2.0 + 78.0),
        (BASE_X / 2.0 - 78.0, -BASE_Y / 2.0 + 78.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("closed_pass_tote_equalization_base_fiducial_{i}"))
                .translate(*x, *y, BASE_Z + 2.0);
    }
    fiducials
}

fn panel_gussets() -> Part {
    let mut gussets = Part::empty("closed_pass_tote_equalization_panel_gussets");
    for (i, x) in [-420.0, -280.0, -140.0, 0.0, 140.0, 280.0, 420.0]
        .iter()
        .enumerate()
    {
        let web = centered_cube(
            format!("closed_pass_tote_equalization_panel_gusset_web_{i}"),
            12.0,
            78.0,
            86.0,
        )
        .translate(*x, PANEL_BASE_Y - 32.0, BASE_Z + 43.0);
        let foot = centered_cube(
            format!("closed_pass_tote_equalization_panel_gusset_foot_{i}"),
            48.0,
            70.0,
            10.0,
        )
        .translate(*x, PANEL_BASE_Y - 34.0, BASE_Z + 5.0);
        let screw = centered_cylinder(
            format!("closed_pass_tote_equalization_panel_gusset_screw_{i}"),
            5.4 / 2.0,
            14.0,
            24,
        )
        .translate(*x, PANEL_BASE_Y - 56.0, BASE_Z + 5.0);
        gussets = gussets + (web + foot - screw);
    }
    gussets
}

fn purge_drain_guard() -> Part {
    let guard = centered_cube(
        "closed_pass_tote_equalization_purge_drain_guard",
        118.0,
        50.0,
        22.0,
    )
    .translate(BASE_X / 2.0 - 92.0, -BASE_Y / 2.0 + 52.0, BASE_Z + 11.0);
    let sight = centered_cylinder(
        "closed_pass_tote_equalization_purge_drain_sight_cut",
        15.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 92.0, -BASE_Y / 2.0 + 52.0, BASE_Z + 11.0);
    guard - sight
}

fn sealed_tote_receiver() -> Part {
    let receiver_x = TOTE_OUTER_X + 2.0 * RECEIVER_CLEARANCE;
    let receiver_y = TOTE_OUTER_Y + 2.0 * RECEIVER_CLEARANCE;

    let footprint = rectangular_frame_xy(
        "closed_pass_tote_receiver_footprint_witness_ring",
        receiver_x + 92.0,
        receiver_y + 86.0,
        12.0,
        receiver_x + 16.0,
        receiver_y + 14.0,
    )
    .translate(0.0, 0.0, 6.0);
    let rear_stop = centered_cube(
        "closed_pass_tote_receiver_rear_y_datum_stop",
        receiver_x + 58.0,
        RECEIVER_RAIL_W,
        RECEIVER_RAIL_Z,
    )
    .translate(
        0.0,
        receiver_y / 2.0 + RECEIVER_RAIL_W / 2.0,
        RECEIVER_RAIL_Z / 2.0,
    );
    let left_datum = centered_cube(
        "closed_pass_tote_receiver_left_x_datum_rail",
        RECEIVER_RAIL_W,
        receiver_y + 44.0,
        RECEIVER_RAIL_Z,
    )
    .translate(
        -(receiver_x / 2.0 + RECEIVER_RAIL_W / 2.0),
        0.0,
        RECEIVER_RAIL_Z / 2.0,
    );
    let right_soft_rail = centered_cube(
        "closed_pass_tote_receiver_right_soft_capture_rail",
        RECEIVER_RAIL_W,
        receiver_y + 44.0,
        RECEIVER_RAIL_Z * 0.72,
    )
    .translate(
        receiver_x / 2.0 + RECEIVER_RAIL_W / 2.0,
        0.0,
        RECEIVER_RAIL_Z * 0.36,
    );
    let front_low_lip = centered_cube(
        "closed_pass_tote_receiver_front_low_loading_lip",
        receiver_x + 58.0,
        16.0,
        RECEIVER_RAIL_Z * 0.55,
    )
    .translate(0.0, -(receiver_y / 2.0 + 10.0), RECEIVER_RAIL_Z * 0.275);
    let pass_through_frame = rectangular_frame_xz(
        "closed_pass_tote_receiver_pass_through_seal_frame",
        TOTE_OUTER_X + 118.0,
        46.0,
        TOTE_OUTER_Z + 96.0,
        TOTE_INNER_X + 42.0,
        TOTE_INNER_Z + 44.0,
    )
    .translate(
        0.0,
        receiver_y / 2.0 + 74.0,
        RECEIVER_RAIL_Z + (TOTE_OUTER_Z + 96.0) / 2.0,
    );
    let equalization_gasket_land = rectangular_frame_xz(
        "closed_pass_tote_receiver_equalization_gasket_land",
        TOTE_OUTER_X + 86.0,
        10.0,
        TOTE_OUTER_Z + 66.0,
        TOTE_INNER_X + 72.0,
        TOTE_INNER_Z + 68.0,
    )
    .translate(
        0.0,
        receiver_y / 2.0 + 45.0,
        RECEIVER_RAIL_Z + (TOTE_OUTER_Z + 70.0) / 2.0,
    );

    footprint
        + rear_stop
        + left_datum
        + right_soft_rail
        + front_low_lip
        + pass_through_frame
        + equalization_gasket_land
        + tote_locator_cups()
        + receiver_latch_pockets()
        + receiver_tote_shadow_gauge()
}

fn tote_locator_cups() -> Part {
    let mut cups = Part::empty("closed_pass_tote_receiver_locator_cups");
    for (i, (x, y)) in receiver_locator_points().iter().enumerate() {
        let cup = centered_cylinder(
            format!("closed_pass_tote_receiver_locator_cup_{i}"),
            16.0,
            RECEIVER_RAIL_Z,
            40,
        )
        .translate(*x, *y, RECEIVER_RAIL_Z / 2.0);
        let pin_clearance = centered_cylinder(
            format!("closed_pass_tote_receiver_locator_pin_clearance_{i}"),
            8.5,
            RECEIVER_RAIL_Z + 4.0,
            32,
        )
        .translate(*x, *y, RECEIVER_RAIL_Z / 2.0);
        cups = cups + (cup - pin_clearance);
    }
    cups
}

fn receiver_latch_pockets() -> Part {
    let mut pockets = Part::empty("closed_pass_tote_receiver_latch_pockets");
    let y_front = -(TOTE_OUTER_Y / 2.0 + 58.0);
    let y_rear = TOTE_OUTER_Y / 2.0 + 58.0;
    for (i, (x, y)) in [
        (-(TOTE_OUTER_X / 2.0 - 90.0), y_front),
        (TOTE_OUTER_X / 2.0 - 90.0, y_front),
        (-(TOTE_OUTER_X / 2.0 - 90.0), y_rear),
        (TOTE_OUTER_X / 2.0 - 90.0, y_rear),
    ]
    .iter()
    .enumerate()
    {
        let pocket = centered_cube(
            format!("closed_pass_tote_receiver_latch_pocket_{i}"),
            74.0,
            40.0,
            26.0,
        )
        .translate(*x, *y, 13.0);
        let hook_relief = centered_cube(
            format!("closed_pass_tote_receiver_latch_hook_relief_{i}"),
            44.0,
            22.0,
            28.0,
        )
        .translate(*x, *y, 15.0);
        let witness_flag = centered_cube(
            format!("closed_pass_tote_receiver_latch_witness_flag_{i}"),
            16.0,
            12.0,
            38.0,
        )
        .translate(*x + 34.0, *y, 19.0);
        pockets = pockets + (pocket - hook_relief) + witness_flag;
    }
    pockets
}

fn receiver_tote_shadow_gauge() -> Part {
    let floor_shadow = rectangular_frame_xy(
        "closed_pass_tote_receiver_tote_shadow_gauge",
        TOTE_OUTER_X + 18.0,
        TOTE_OUTER_Y + 18.0,
        5.0,
        TOTE_OUTER_X - 18.0,
        TOTE_OUTER_Y - 18.0,
    )
    .translate(0.0, 0.0, RECEIVER_RAIL_Z + 5.0);
    let rear_door_sweep = centered_cube(
        "closed_pass_tote_receiver_door_sweep_witness_bar",
        TOTE_OUTER_X + 82.0,
        10.0,
        24.0,
    )
    .translate(0.0, TOTE_OUTER_Y / 2.0 + 24.0, RECEIVER_RAIL_Z + 12.0);
    floor_shadow + rear_door_sweep
}

fn dual_pressure_port_panel() -> Part {
    let plate = centered_cube(
        "closed_pass_tote_equalization_dual_pressure_panel_plate",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(0.0, 0.0, PANEL_Z / 2.0);

    let mut cuts = Part::empty("closed_pass_tote_equalization_pressure_port_cuts");
    let mut rings = Part::empty("closed_pass_tote_equalization_pressure_port_face_rings");
    for bank in 0..PRESSURE_BANKS {
        for port in 0..PORTS_PER_BANK {
            let (x, z) = pressure_port_pose(bank, port);
            let cut = centered_cylinder(
                format!("closed_pass_tote_equalization_pressure_port_cut_{bank}_{port}"),
                PORT_D / 2.0,
                PANEL_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, z);
            let ring = centered_cylinder(
                format!("closed_pass_tote_equalization_pressure_port_ring_{bank}_{port}"),
                15.0,
                8.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -PANEL_Y / 2.0 - 4.0, z);
            let inner = centered_cylinder(
                format!("closed_pass_tote_equalization_pressure_port_ring_bore_{bank}_{port}"),
                PORT_D / 2.0,
                10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -PANEL_Y / 2.0 - 4.0, z);
            cuts = cuts + cut;
            rings = rings + (ring - inner);
        }
    }

    plate - cuts
        + rings
        + pressure_gauge_cluster()
        + equalization_balance_manifold()
        + port_bank_label_lands()
        + panel_relief_valves()
}

fn pressure_gauge_cluster() -> Part {
    let mut gauges = Part::empty("closed_pass_tote_equalization_pressure_gauge_cluster");
    for (i, (x, z)) in [
        (-370.0, 286.0),
        (-248.0, 286.0),
        (248.0, 286.0),
        (370.0, 286.0),
    ]
    .iter()
    .enumerate()
    {
        let bezel = centered_cylinder(
            format!("closed_pass_tote_equalization_pressure_gauge_bezel_{i}"),
            GAUGE_D / 2.0,
            10.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -PANEL_Y / 2.0 - 5.0, *z);
        let face = centered_cylinder(
            format!("closed_pass_tote_equalization_pressure_gauge_face_recess_{i}"),
            GAUGE_D / 2.0 - 5.0,
            12.0,
            48,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -PANEL_Y / 2.0 - 5.0, *z);
        let needle = centered_cube(
            format!("closed_pass_tote_equalization_pressure_gauge_needle_{i}"),
            GAUGE_D * 0.58,
            4.0,
            3.0,
        )
        .rotate(0.0, 0.0, if i % 2 == 0 { -18.0 } else { 18.0 })
        .translate(*x, -PANEL_Y / 2.0 - 12.0, *z);
        gauges = gauges + (bezel - face) + needle;
    }
    gauges
}

fn equalization_balance_manifold() -> Part {
    let horizontal_header = centered_cube(
        "closed_pass_tote_equalization_balance_header_land",
        PANEL_X - 170.0,
        12.0,
        22.0,
    )
    .translate(0.0, -PANEL_Y / 2.0 - 6.0, 122.0);
    let cross_balance_spool = centered_cylinder(
        "closed_pass_tote_equalization_cross_balance_spool",
        15.0,
        250.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -PANEL_Y / 2.0 - 12.0, 210.0);
    let center_selector = centered_cube(
        "closed_pass_tote_equalization_center_selector_valve_placeholder",
        74.0,
        24.0,
        64.0,
    )
    .translate(0.0, -PANEL_Y / 2.0 - 14.0, 182.0);
    let bypass_token_socket = centered_cube(
        "closed_pass_tote_equalization_bypass_token_socket",
        132.0,
        16.0,
        38.0,
    )
    .translate(0.0, -PANEL_Y / 2.0 - 13.0, 78.0);
    horizontal_header + cross_balance_spool + center_selector + bypass_token_socket
}

fn port_bank_label_lands() -> Part {
    let left_land = port_label_land("left_bank", -310.0);
    let right_land = port_label_land("right_bank", 310.0);
    let center_land = centered_cube(
        "closed_pass_tote_equalization_delta_pressure_label_land",
        154.0,
        8.0,
        28.0,
    )
    .translate(0.0, -PANEL_Y / 2.0 - 6.0, 254.0);
    left_land + right_land + center_land
}

fn port_label_land(name: &str, x: f64) -> Part {
    let land = centered_cube(
        format!("closed_pass_tote_equalization_{name}_label_land"),
        186.0,
        8.0,
        34.0,
    )
    .translate(x, -PANEL_Y / 2.0 - 6.0, 38.0);
    let scanner_recess = centered_cube(
        format!("closed_pass_tote_equalization_{name}_barcode_recess"),
        126.0,
        10.0,
        11.0,
    )
    .translate(x, -PANEL_Y / 2.0 - 6.0, 38.0);
    land - scanner_recess
}

fn panel_relief_valves() -> Part {
    let mut valves = Part::empty("closed_pass_tote_equalization_panel_relief_valves");
    for (i, x) in [-86.0, 86.0].iter().enumerate() {
        let base = centered_cylinder(
            format!("closed_pass_tote_equalization_relief_valve_base_{i}"),
            21.0,
            18.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -PANEL_Y / 2.0 - 9.0, 286.0);
        let cap = centered_cylinder(
            format!("closed_pass_tote_equalization_relief_valve_cap_{i}"),
            12.0,
            22.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -PANEL_Y / 2.0 - 29.0, 286.0);
        valves = valves + base + cap;
    }
    valves
}

fn hepa_purge_duct_surrogate() -> Part {
    let duct_shell = centered_cube(
        "closed_pass_tote_equalization_hepa_purge_duct_outer_shell",
        DUCT_X,
        DUCT_Y,
        DUCT_Z,
    )
    .translate(0.0, 0.0, DUCT_Z / 2.0);
    let duct_channel = centered_cube(
        "closed_pass_tote_equalization_hepa_purge_duct_channel_cut",
        DUCT_CHANNEL_X,
        DUCT_Y + 8.0,
        DUCT_CHANNEL_Z,
    )
    .translate(0.0, 0.0, DUCT_Z / 2.0);
    let hepa_frame = rectangular_frame_xz(
        "closed_pass_tote_equalization_hepa_cassette_frame",
        HEPA_CASSETTE_X,
        HEPA_CASSETTE_Y,
        HEPA_CASSETTE_Z,
        HEPA_CASSETTE_X - 56.0,
        HEPA_CASSETTE_Z - 54.0,
    )
    .translate(0.0, -DUCT_Y / 2.0 - HEPA_CASSETTE_Y / 2.0, DUCT_Z / 2.0);
    let left_collar = duct_collar("left", -DUCT_X / 2.0 - 32.0);
    let right_collar = duct_collar("right", DUCT_X / 2.0 + 32.0);
    let diffuser_plate = diffuser_plate();
    let static_mixer_baffles = purge_balance_baffles();
    let filter_pull_tabs = hepa_filter_pull_tabs();

    duct_shell - duct_channel
        + hepa_frame
        + left_collar
        + right_collar
        + diffuser_plate
        + static_mixer_baffles
        + filter_pull_tabs
}

fn duct_collar(name: &str, x: f64) -> Part {
    let outer = centered_cylinder(
        format!("closed_pass_tote_equalization_{name}_purge_duct_collar_outer"),
        36.0,
        64.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, 0.0, DUCT_Z / 2.0);
    let inner = centered_cylinder(
        format!("closed_pass_tote_equalization_{name}_purge_duct_collar_bore"),
        24.0,
        68.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, 0.0, DUCT_Z / 2.0);
    outer - inner
}

fn diffuser_plate() -> Part {
    let plate = centered_cube(
        "closed_pass_tote_equalization_hepa_diffuser_plate",
        DUCT_X - 110.0,
        10.0,
        DUCT_Z - 34.0,
    )
    .translate(0.0, DUCT_Y / 2.0 + 5.0, DUCT_Z / 2.0);
    let mut holes = Part::empty("closed_pass_tote_equalization_hepa_diffuser_holes");
    for ix in 0..DIFFUSER_HOLES_X {
        for iz in 0..DIFFUSER_HOLES_Z {
            let x = -((DIFFUSER_HOLES_X as f64 - 1.0) * 62.0) / 2.0 + ix as f64 * 62.0;
            let z = DUCT_Z / 2.0 - 28.0 + iz as f64 * 28.0;
            holes = holes
                + centered_cylinder(
                    format!("closed_pass_tote_equalization_diffuser_orifice_{ix}_{iz}"),
                    7.5,
                    14.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, DUCT_Y / 2.0 + 5.0, z);
        }
    }
    plate - holes
}

fn purge_balance_baffles() -> Part {
    let mut baffles = Part::empty("closed_pass_tote_equalization_purge_balance_baffles");
    for (i, x) in [-245.0, -147.0, -49.0, 49.0, 147.0, 245.0]
        .iter()
        .enumerate()
    {
        let baffle = centered_cube(
            format!("closed_pass_tote_equalization_purge_baffle_{i}"),
            8.0,
            DUCT_Y - 40.0,
            DUCT_Z - 48.0,
        )
        .rotate(0.0, 0.0, if i % 2 == 0 { 14.0 } else { -14.0 })
        .translate(*x, 0.0, DUCT_Z / 2.0);
        baffles = baffles + baffle;
    }
    baffles
}

fn hepa_filter_pull_tabs() -> Part {
    let left = centered_cube(
        "closed_pass_tote_equalization_hepa_left_pull_tab",
        32.0,
        18.0,
        82.0,
    )
    .translate(
        -HEPA_CASSETTE_X / 2.0 - 22.0,
        -DUCT_Y / 2.0 - HEPA_CASSETTE_Y,
        DUCT_Z / 2.0,
    );
    let right = centered_cube(
        "closed_pass_tote_equalization_hepa_right_pull_tab",
        32.0,
        18.0,
        82.0,
    )
    .translate(
        HEPA_CASSETTE_X / 2.0 + 22.0,
        -DUCT_Y / 2.0 - HEPA_CASSETTE_Y,
        DUCT_Z / 2.0,
    );
    left + right
}

fn flow_witness_vanes() -> Part {
    let base = centered_cube(
        "closed_pass_tote_equalization_flow_witness_base",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);
    let channel = centered_cube(
        "closed_pass_tote_equalization_flow_witness_channel_cut",
        WITNESS_X - 78.0,
        WITNESS_Y - 52.0,
        WITNESS_Z + 4.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);
    let rails = witness_side_rails();
    let vanes = vane_rows();
    let scale_ticks = vane_scale_ticks();
    let balance_ports = witness_balance_ports();

    base - channel + rails + vanes + scale_ticks + balance_ports
}

fn witness_side_rails() -> Part {
    let left = centered_cube(
        "closed_pass_tote_equalization_flow_witness_left_rail",
        WITNESS_X,
        12.0,
        34.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 18.0, WITNESS_Z + 17.0);
    let right = centered_cube(
        "closed_pass_tote_equalization_flow_witness_right_rail",
        WITNESS_X,
        12.0,
        34.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 18.0, WITNESS_Z + 17.0);
    let inlet = centered_cube(
        "closed_pass_tote_equalization_flow_witness_inlet_stop",
        16.0,
        WITNESS_Y - 36.0,
        28.0,
    )
    .translate(-WITNESS_X / 2.0 + 24.0, 0.0, WITNESS_Z + 14.0);
    let outlet = centered_cube(
        "closed_pass_tote_equalization_flow_witness_outlet_stop",
        16.0,
        WITNESS_Y - 36.0,
        28.0,
    )
    .translate(WITNESS_X / 2.0 - 24.0, 0.0, WITNESS_Z + 14.0);
    left + right + inlet + outlet
}

fn vane_rows() -> Part {
    let mut rows = Part::empty("closed_pass_tote_equalization_flow_witness_vanes");
    for row in 0..WITNESS_ROWS {
        for index in 0..VANES_PER_ROW {
            let x = vane_x(index);
            let y = if row == 0 { -34.0 } else { 34.0 };
            let angle = if (row + index) % 2 == 0 { -11.0 } else { 11.0 };
            rows = rows + witness_vane(row, index, angle).translate(x, y, 0.0);
        }
    }
    rows
}

fn witness_vane(row: usize, index: usize, angle: f64) -> Part {
    let blade = centered_cube(
        format!("closed_pass_tote_equalization_vane_blade_{row}_{index}"),
        VANE_X,
        VANE_Y,
        VANE_Z,
    )
    .rotate(0.0, 0.0, angle)
    .translate(0.0, 0.0, WITNESS_Z + VANE_Z / 2.0);
    let pivot = centered_cylinder(
        format!("closed_pass_tote_equalization_vane_pivot_{row}_{index}"),
        4.0,
        VANE_Y + 18.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, WITNESS_Z + VANE_Z + 4.0);
    let flag = centered_cube(
        format!("closed_pass_tote_equalization_vane_tip_flag_{row}_{index}"),
        18.0,
        4.0,
        8.0,
    )
    .rotate(0.0, 0.0, angle)
    .translate(0.0, VANE_Y / 2.0 + 10.0, WITNESS_Z + VANE_Z + 4.0);
    blade + pivot + flag
}

fn vane_scale_ticks() -> Part {
    let mut ticks = Part::empty("closed_pass_tote_equalization_flow_witness_scale_ticks");
    for row in 0..WITNESS_ROWS {
        let y = if row == 0 {
            -WITNESS_Y / 2.0 + 36.0
        } else {
            WITNESS_Y / 2.0 - 36.0
        };
        for tick in 0..=VANES_PER_ROW {
            let x = -((VANES_PER_ROW as f64) * 82.0) / 2.0 + tick as f64 * 82.0;
            ticks = ticks
                + centered_cube(
                    format!("closed_pass_tote_equalization_vane_scale_tick_{row}_{tick}"),
                    4.0,
                    16.0,
                    6.0,
                )
                .translate(x, y, WITNESS_Z + 3.0);
        }
    }
    ticks
}

fn witness_balance_ports() -> Part {
    let inlet = centered_cylinder(
        "closed_pass_tote_equalization_witness_inlet_balance_port",
        19.0,
        34.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-WITNESS_X / 2.0 - 10.0, 0.0, WITNESS_Z + 22.0);
    let outlet = centered_cylinder(
        "closed_pass_tote_equalization_witness_outlet_balance_port",
        19.0,
        34.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(WITNESS_X / 2.0 + 10.0, 0.0, WITNESS_Z + 22.0);
    inlet + outlet
}

fn door_seal_coupon_array() -> Part {
    let tray = centered_cube(
        "closed_pass_tote_equalization_door_seal_coupon_carrier",
        COUPON_TRAY_X,
        COUPON_TRAY_Y,
        COUPON_TRAY_Z,
    )
    .translate(0.0, 0.0, COUPON_TRAY_Z / 2.0);
    let mut recesses = Part::empty("closed_pass_tote_equalization_door_seal_coupon_recesses");
    let mut coupons = Part::empty("closed_pass_tote_equalization_door_seal_coupons");
    for i in 0..COUPON_COUNT {
        let (x, y) = coupon_pose(i);
        let recess = centered_cube(
            format!("closed_pass_tote_equalization_seal_coupon_recess_{i}"),
            COUPON_X + 8.0,
            COUPON_Y + 8.0,
            COUPON_TRAY_Z + 2.0,
        )
        .translate(x, y, COUPON_TRAY_Z / 2.0 + 2.0);
        let coupon = door_seal_coupon(i).translate(x, y, COUPON_TRAY_Z + COUPON_Z / 2.0 + 2.0);
        recesses = recesses + recess;
        coupons = coupons + coupon;
    }
    tray - recesses + coupons + coupon_clamp_bar() + seal_compression_datum_posts()
}

fn door_seal_coupon(index: usize) -> Part {
    let body = centered_cube(
        format!("closed_pass_tote_equalization_door_seal_coupon_body_{index}"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let groove = rectangular_frame_xy(
        format!("closed_pass_tote_equalization_door_seal_coupon_groove_{index}"),
        COUPON_X - 24.0,
        COUPON_Y - 18.0,
        COUPON_Z + 2.0,
        COUPON_X - 46.0,
        COUPON_Y - 38.0,
    );
    let raised_witness = rectangular_frame_xy(
        format!("closed_pass_tote_equalization_door_seal_coupon_witness_bead_{index}"),
        COUPON_X - 42.0,
        COUPON_Y - 36.0,
        3.0,
        COUPON_X - 66.0,
        COUPON_Y - 52.0,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0 + 1.5);
    let clamp_hole_l = centered_cylinder(
        format!("closed_pass_tote_equalization_door_seal_coupon_left_clamp_hole_{index}"),
        3.2,
        COUPON_Z + 3.0,
        20,
    )
    .translate(-COUPON_X / 2.0 + 16.0, 0.0, 0.0);
    let clamp_hole_r = centered_cylinder(
        format!("closed_pass_tote_equalization_door_seal_coupon_right_clamp_hole_{index}"),
        3.2,
        COUPON_Z + 3.0,
        20,
    )
    .translate(COUPON_X / 2.0 - 16.0, 0.0, 0.0);
    body - groove - clamp_hole_l - clamp_hole_r + raised_witness
}

fn coupon_clamp_bar() -> Part {
    let front = centered_cube(
        "closed_pass_tote_equalization_door_coupon_front_clamp_bar",
        COUPON_TRAY_X - 36.0,
        12.0,
        18.0,
    )
    .translate(0.0, -COUPON_TRAY_Y / 2.0 + 20.0, COUPON_TRAY_Z + 9.0);
    let rear = centered_cube(
        "closed_pass_tote_equalization_door_coupon_rear_clamp_bar",
        COUPON_TRAY_X - 36.0,
        12.0,
        18.0,
    )
    .translate(0.0, COUPON_TRAY_Y / 2.0 - 20.0, COUPON_TRAY_Z + 9.0);
    front + rear
}

fn seal_compression_datum_posts() -> Part {
    let mut posts = Part::empty("closed_pass_tote_equalization_seal_compression_datum_posts");
    for (i, (x, y)) in [
        (-COUPON_TRAY_X / 2.0 + 26.0, -COUPON_TRAY_Y / 2.0 + 26.0),
        (COUPON_TRAY_X / 2.0 - 26.0, -COUPON_TRAY_Y / 2.0 + 26.0),
        (-COUPON_TRAY_X / 2.0 + 26.0, COUPON_TRAY_Y / 2.0 - 26.0),
        (COUPON_TRAY_X / 2.0 - 26.0, COUPON_TRAY_Y / 2.0 - 26.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_pass_tote_equalization_seal_compression_post_{i}"),
                9.0,
                30.0,
                32,
            )
            .translate(*x, *y, COUPON_TRAY_Z + 15.0);
    }
    posts
}

fn pressure_decay_token_lane() -> Part {
    let lane = centered_cube(
        "closed_pass_tote_equalization_pressure_decay_token_lane_base",
        TOKEN_LANE_X,
        TOKEN_LANE_Y,
        TOKEN_LANE_Z,
    )
    .translate(0.0, 0.0, TOKEN_LANE_Z / 2.0);
    let center_track = centered_cube(
        "closed_pass_tote_equalization_pressure_decay_token_center_track_cut",
        TOKEN_LANE_X - 78.0,
        TOKEN_D + 20.0,
        TOKEN_LANE_Z + 3.0,
    )
    .translate(0.0, 0.0, TOKEN_LANE_Z / 2.0 + 1.0);
    let rails = token_lane_rails();
    let tokens = decay_reference_tokens();
    let gates = token_lane_gates();
    let manifold = token_lane_manifold();

    lane - center_track + rails + tokens + gates + manifold
}

fn token_lane_rails() -> Part {
    let front = centered_cube(
        "closed_pass_tote_equalization_token_lane_front_rail",
        TOKEN_LANE_X,
        12.0,
        32.0,
    )
    .translate(0.0, -TOKEN_LANE_Y / 2.0 + 12.0, TOKEN_LANE_Z + 16.0);
    let rear = centered_cube(
        "closed_pass_tote_equalization_token_lane_rear_rail",
        TOKEN_LANE_X,
        12.0,
        32.0,
    )
    .translate(0.0, TOKEN_LANE_Y / 2.0 - 12.0, TOKEN_LANE_Z + 16.0);
    front + rear
}

fn decay_reference_tokens() -> Part {
    let mut tokens = Part::empty("closed_pass_tote_equalization_pressure_decay_tokens");
    for i in 0..TOKEN_COUNT {
        let x = token_x(i);
        tokens = tokens + decay_reference_token(i).translate(x, 0.0, TOKEN_LANE_Z + TOKEN_Z / 2.0);
    }
    tokens
}

fn decay_reference_token(index: usize) -> Part {
    let body = centered_cylinder(
        format!("closed_pass_tote_equalization_decay_reference_token_{index}"),
        TOKEN_D / 2.0,
        TOKEN_Z,
        44,
    );
    let metered_bore = centered_cylinder(
        format!("closed_pass_tote_equalization_decay_reference_token_bore_{index}"),
        1.2 + index as f64 * 0.16,
        TOKEN_Z + 2.0,
        18,
    );
    let clocking_flat = centered_cube(
        format!("closed_pass_tote_equalization_decay_token_clocking_flat_{index}"),
        TOKEN_D,
        7.0,
        TOKEN_Z + 2.0,
    )
    .translate(0.0, TOKEN_D / 2.0 - 3.0, 0.0);
    let witness_notch = centered_cube(
        format!("closed_pass_tote_equalization_decay_token_witness_notch_{index}"),
        8.0 + index as f64,
        5.0,
        3.0,
    )
    .translate(0.0, -TOKEN_D / 2.0 + 4.0, TOKEN_Z / 2.0 + 1.5);
    body - metered_bore - clocking_flat + witness_notch
}

fn token_lane_gates() -> Part {
    let inlet_gate = centered_cube(
        "closed_pass_tote_equalization_token_lane_inlet_gate",
        18.0,
        TOKEN_LANE_Y - 18.0,
        44.0,
    )
    .translate(-TOKEN_LANE_X / 2.0 + 22.0, 0.0, TOKEN_LANE_Z + 22.0);
    let outlet_gate = centered_cube(
        "closed_pass_tote_equalization_token_lane_outlet_gate",
        18.0,
        TOKEN_LANE_Y - 18.0,
        44.0,
    )
    .translate(TOKEN_LANE_X / 2.0 - 22.0, 0.0, TOKEN_LANE_Z + 22.0);
    inlet_gate + outlet_gate
}

fn token_lane_manifold() -> Part {
    let header = centered_cylinder(
        "closed_pass_tote_equalization_token_lane_pressure_header",
        7.0,
        TOKEN_LANE_X - 102.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, TOKEN_LANE_Y / 2.0 + 22.0, TOKEN_LANE_Z + 24.0);
    let mut taps = Part::empty("closed_pass_tote_equalization_token_lane_taps");
    for i in 0..TOKEN_COUNT {
        taps = taps
            + centered_cylinder(
                format!("closed_pass_tote_equalization_token_lane_tap_{i}"),
                3.0,
                38.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(token_x(i), TOKEN_LANE_Y / 2.0 + 5.0, TOKEN_LANE_Z + 24.0);
    }
    header + taps
}

fn barcode_certificate_lands() -> Part {
    let carrier = centered_cube(
        "closed_pass_tote_equalization_barcode_certificate_carrier",
        RECORD_X,
        RECORD_Y,
        RECORD_Z,
    )
    .translate(0.0, 0.0, RECORD_Z / 2.0);
    let barcodes = barcode_lands();
    let certificates = certificate_lands();
    let signature_key = centered_cube(
        "closed_pass_tote_equalization_certificate_signature_key_land",
        124.0,
        58.0,
        7.0,
    )
    .translate(RECORD_X / 2.0 - 82.0, 0.0, RECORD_Z + 3.5);
    let signature_recess = centered_cube(
        "closed_pass_tote_equalization_certificate_signature_key_recess",
        86.0,
        14.0,
        8.0,
    )
    .translate(RECORD_X / 2.0 - 82.0, -20.0, RECORD_Z + 3.5);

    carrier + barcodes + certificates + (signature_key - signature_recess)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_pass_tote_equalization_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = -RECORD_X / 2.0 + 58.0 + i as f64 * 78.0;
        let land = centered_cube(
            format!("closed_pass_tote_equalization_barcode_land_{i}"),
            66.0,
            52.0,
            6.0,
        )
        .translate(x, -RECORD_Y / 2.0 + 44.0, RECORD_Z + 3.0);
        let scan_relief = centered_cube(
            format!("closed_pass_tote_equalization_barcode_scan_relief_{i}"),
            48.0,
            10.0,
            8.0,
        )
        .translate(x, -RECORD_Y / 2.0 + 28.0, RECORD_Z + 3.0);
        let lead_tick = centered_cube(
            format!("closed_pass_tote_equalization_barcode_lead_tick_{i}"),
            4.0,
            54.0,
            7.0,
        )
        .translate(x - 36.0, -RECORD_Y / 2.0 + 44.0, RECORD_Z + 3.5);
        lands = lands + (land - scan_relief) + lead_tick;
    }
    lands
}

fn certificate_lands() -> Part {
    let mut certs = Part::empty("closed_pass_tote_equalization_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = -RECORD_X / 2.0 + 120.0 + i as f64 * 188.0;
        let cert = centered_cube(
            format!("closed_pass_tote_equalization_certificate_land_{i}"),
            154.0,
            58.0,
            6.0,
        )
        .translate(x, RECORD_Y / 2.0 - 44.0, RECORD_Z + 3.0);
        let clip_a = centered_cube(
            format!("closed_pass_tote_equalization_certificate_clip_a_{i}"),
            14.0,
            58.0,
            16.0,
        )
        .translate(x - 86.0, RECORD_Y / 2.0 - 44.0, RECORD_Z + 8.0);
        let clip_b = centered_cube(
            format!("closed_pass_tote_equalization_certificate_clip_b_{i}"),
            14.0,
            58.0,
            16.0,
        )
        .translate(x + 86.0, RECORD_Y / 2.0 - 44.0, RECORD_Z + 8.0);
        certs = certs + cert + clip_a + clip_b;
    }
    certs
}

fn release_hold_reject_lanes() -> Part {
    status_lane("release", -STATUS_LANE_X - 28.0, 0, 1.0)
        + status_lane("hold", 0.0, 1, 0.65)
        + status_lane("reject", STATUS_LANE_X + 28.0, 2, 0.35)
        + status_lane_cross_bar()
}

fn status_lane(name: &str, x: f64, index: usize, post_scale: f64) -> Part {
    let deck = centered_cube(
        format!("closed_pass_tote_equalization_{name}_lane_floor"),
        STATUS_LANE_X,
        STATUS_LANE_Y,
        STATUS_LANE_Z,
    )
    .translate(x, 0.0, STATUS_LANE_Z / 2.0);
    let recess = centered_cube(
        format!("closed_pass_tote_equalization_{name}_lane_tote_card_recess"),
        STATUS_LANE_X - 50.0,
        STATUS_LANE_Y - 48.0,
        STATUS_LANE_Z + 3.0,
    )
    .translate(x, 0.0, STATUS_LANE_Z / 2.0 + 1.0);
    let left_rail = centered_cube(
        format!("closed_pass_tote_equalization_{name}_lane_left_rail"),
        12.0,
        STATUS_LANE_Y,
        STATUS_RAIL_Z,
    )
    .translate(
        x - STATUS_LANE_X / 2.0 + 12.0,
        0.0,
        STATUS_LANE_Z + STATUS_RAIL_Z / 2.0,
    );
    let right_rail = centered_cube(
        format!("closed_pass_tote_equalization_{name}_lane_right_rail"),
        12.0,
        STATUS_LANE_Y,
        STATUS_RAIL_Z,
    )
    .translate(
        x + STATUS_LANE_X / 2.0 - 12.0,
        0.0,
        STATUS_LANE_Z + STATUS_RAIL_Z / 2.0,
    );
    let rear_gate = centered_cube(
        format!("closed_pass_tote_equalization_{name}_lane_rear_gate"),
        STATUS_LANE_X,
        12.0,
        STATUS_RAIL_Z,
    )
    .translate(
        x,
        STATUS_LANE_Y / 2.0 - 12.0,
        STATUS_LANE_Z + STATUS_RAIL_Z / 2.0,
    );
    let status_post_h = 60.0 * post_scale + 18.0;
    let status_post = centered_cylinder(
        format!("closed_pass_tote_equalization_{name}_lane_status_post_{index}"),
        13.0,
        status_post_h,
        32,
    )
    .translate(
        x + STATUS_LANE_X / 2.0 - 36.0,
        -STATUS_LANE_Y / 2.0 + 34.0,
        STATUS_LANE_Z + status_post_h / 2.0,
    );
    let token_pocket = centered_cylinder(
        format!("closed_pass_tote_equalization_{name}_lane_certificate_token_pocket_{index}"),
        22.0,
        7.0,
        36,
    )
    .translate(
        x - STATUS_LANE_X / 2.0 + 44.0,
        -STATUS_LANE_Y / 2.0 + 34.0,
        STATUS_LANE_Z + 3.5,
    );
    deck - recess + left_rail + right_rail + rear_gate + status_post + token_pocket
}

fn status_lane_cross_bar() -> Part {
    centered_cube(
        "closed_pass_tote_equalization_release_hold_reject_lane_alignment_bar",
        3.0 * STATUS_LANE_X + 72.0,
        14.0,
        16.0,
    )
    .translate(0.0, -STATUS_LANE_Y / 2.0 - 18.0, STATUS_LANE_Z + 8.0)
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_left_post",
        BRIDGE_RAIL,
        BRIDGE_RAIL,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0, -BRIDGE_Y / 2.0, BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_right_post",
        BRIDGE_RAIL,
        BRIDGE_RAIL,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0, -BRIDGE_Y / 2.0, BRIDGE_Z / 2.0);
    let rear_left_post = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_rear_left_post",
        BRIDGE_RAIL,
        BRIDGE_RAIL,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0, BRIDGE_Y / 2.0, BRIDGE_Z / 2.0);
    let rear_right_post = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_rear_right_post",
        BRIDGE_RAIL,
        BRIDGE_RAIL,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0, BRIDGE_Y / 2.0, BRIDGE_Z / 2.0);
    let top_beam = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_top_beam",
        BRIDGE_X + BRIDGE_RAIL,
        BRIDGE_RAIL,
        26.0,
    )
    .translate(0.0, -BRIDGE_Y / 2.0, BRIDGE_Z + 13.0);
    let rear_beam = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_rear_beam",
        BRIDGE_X + BRIDGE_RAIL,
        BRIDGE_RAIL,
        26.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0, BRIDGE_Z + 13.0);
    let scanner_bar = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_scanner_bar",
        BRIDGE_X - 90.0,
        28.0,
        24.0,
    )
    .translate(0.0, 0.0, BRIDGE_Z - 44.0);
    let bridge_cameras = evidence_camera_pods();
    let cable_tray = evidence_bridge_cable_tray();
    let token_drop_chute = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_token_drop_chute",
        82.0,
        64.0,
        118.0,
    )
    .translate(BRIDGE_X / 2.0 - 82.0, 0.0, BRIDGE_Z / 2.0 + 18.0);

    left_post
        + right_post
        + rear_left_post
        + rear_right_post
        + top_beam
        + rear_beam
        + scanner_bar
        + bridge_cameras
        + cable_tray
        + token_drop_chute
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("closed_pass_tote_equalization_evidence_camera_pods");
    for (i, x) in [-230.0, 0.0, 230.0].iter().enumerate() {
        let pod = centered_cube(
            format!("closed_pass_tote_equalization_evidence_camera_pod_{i}"),
            82.0,
            48.0,
            36.0,
        )
        .translate(*x, -BRIDGE_Y / 2.0 - 18.0, BRIDGE_Z - 78.0);
        let lens = centered_cylinder(
            format!("closed_pass_tote_equalization_evidence_camera_lens_{i}"),
            12.0,
            10.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -BRIDGE_Y / 2.0 - 46.0, BRIDGE_Z - 78.0);
        pods = pods + pod + lens;
    }
    pods
}

fn evidence_bridge_cable_tray() -> Part {
    let tray = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_cable_tray",
        BRIDGE_X - 70.0,
        34.0,
        16.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0 + 20.0, BRIDGE_Z - 22.0);
    let recess = centered_cube(
        "closed_pass_tote_equalization_evidence_bridge_cable_tray_recess",
        BRIDGE_X - 112.0,
        16.0,
        18.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0 + 20.0, BRIDGE_Z - 20.0);
    tray - recess
}

fn robot_service_keepouts() -> Part {
    let robot_approach = keepout_frame(
        "front_robot_tote_approach",
        TOTE_OUTER_X + 240.0,
        320.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(-220.0, -BASE_Y / 2.0 - 114.0, ROBOT_KEEP_OUT_Z / 2.0);
    let door_swing = keepout_frame(
        "rear_pass_through_door_swing",
        TOTE_OUTER_X + 190.0,
        DOOR_SWING_KEEP_OUT_Y,
        TOTE_OUTER_Z + 170.0,
    )
    .translate(
        -220.0,
        PANEL_BASE_Y + DOOR_SWING_KEEP_OUT_Y / 2.0,
        (TOTE_OUTER_Z + 170.0) / 2.0,
    );
    let hepa_pull = keepout_frame(
        "rear_hepa_filter_pull",
        HEPA_CASSETTE_X + 120.0,
        FILTER_PULL_KEEP_OUT_Y,
        HEPA_CASSETTE_Z + 84.0,
    )
    .translate(
        -210.0,
        PANEL_BASE_Y - 210.0,
        PANEL_Z - 34.0 + (HEPA_CASSETTE_Z + 84.0) / 2.0,
    );
    let left_service = keepout_frame(
        "left_pressure_panel_service",
        SERVICE_KEEP_OUT_X,
        BASE_Y - 160.0,
        190.0,
    )
    .translate(-BASE_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 + 48.0, 0.0, 95.0);
    let right_service = keepout_frame(
        "right_certificate_lane_service",
        SERVICE_KEEP_OUT_X,
        BASE_Y - 190.0,
        172.0,
    )
    .translate(BASE_X / 2.0 + SERVICE_KEEP_OUT_X / 2.0 - 48.0, -22.0, 86.0);
    let overhead_lift = keepout_frame(
        "overhead_tote_lift_clearance",
        TOTE_OUTER_X + 178.0,
        TOTE_OUTER_Y + 178.0,
        590.0,
    )
    .translate(-220.0, -40.0, 295.0);

    robot_approach + door_swing + hepa_pull + left_service + right_service + overhead_lift
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let bottom = rectangular_frame_xy(
        format!("closed_pass_tote_equalization_{name}_keepout_bottom"),
        x,
        y,
        KEEP_OUT_RAIL,
        x - 2.0 * KEEP_OUT_RAIL,
        y - 2.0 * KEEP_OUT_RAIL,
    )
    .translate(0.0, 0.0, -z / 2.0);
    let top = rectangular_frame_xy(
        format!("closed_pass_tote_equalization_{name}_keepout_top"),
        x,
        y,
        KEEP_OUT_RAIL,
        x - 2.0 * KEEP_OUT_RAIL,
        y - 2.0 * KEEP_OUT_RAIL,
    )
    .translate(0.0, 0.0, z / 2.0);
    let mut posts = Part::empty(format!(
        "closed_pass_tote_equalization_{name}_keepout_corner_posts"
    ));
    for (i, (px, py)) in [
        (
            -x / 2.0 + KEEP_OUT_RAIL / 2.0,
            -y / 2.0 + KEEP_OUT_RAIL / 2.0,
        ),
        (
            x / 2.0 - KEEP_OUT_RAIL / 2.0,
            -y / 2.0 + KEEP_OUT_RAIL / 2.0,
        ),
        (
            -x / 2.0 + KEEP_OUT_RAIL / 2.0,
            y / 2.0 - KEEP_OUT_RAIL / 2.0,
        ),
        (x / 2.0 - KEEP_OUT_RAIL / 2.0, y / 2.0 - KEEP_OUT_RAIL / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_pass_tote_equalization_{name}_keepout_post_{i}"),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                z,
            )
            .translate(*px, *py, 0.0);
    }
    bottom + top + posts
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    height_z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, height_z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        inner_x,
        inner_y,
        height_z + 2.0,
    );
    outer - inner
}

fn rectangular_frame_xz(
    name: impl Into<String>,
    outer_x: f64,
    depth_y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, depth_y, outer_z);
    let inner = centered_cube(format!("{name}_inner_cut"), inner_x, depth_y + 2.0, inner_z);
    outer - inner
}

fn fiducial_target(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 8.0, 3.0, 36);
    let bore = centered_cylinder(format!("{name}_center_bore"), 1.8, 4.0, 18);
    disc - bore
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-BASE_X / 2.0 + 56.0, -BASE_Y / 2.0 + 56.0),
        (BASE_X / 2.0 - 56.0, -BASE_Y / 2.0 + 56.0),
        (-BASE_X / 2.0 + 56.0, BASE_Y / 2.0 - 56.0),
        (BASE_X / 2.0 - 56.0, BASE_Y / 2.0 - 56.0),
        (0.0, -BASE_Y / 2.0 + 56.0),
        (0.0, BASE_Y / 2.0 - 56.0),
        (-BASE_X / 2.0 + 56.0, 0.0),
        (BASE_X / 2.0 - 56.0, 0.0),
    ]
}

fn receiver_locator_points() -> [(f64, f64); LOCATOR_COUNT] {
    [
        (-(TOTE_OUTER_X / 2.0 - 72.0), -(TOTE_OUTER_Y / 2.0 - 72.0)),
        (TOTE_OUTER_X / 2.0 - 72.0, -(TOTE_OUTER_Y / 2.0 - 72.0)),
        (-(TOTE_OUTER_X / 2.0 - 72.0), TOTE_OUTER_Y / 2.0 - 72.0),
        (TOTE_OUTER_X / 2.0 - 72.0, TOTE_OUTER_Y / 2.0 - 72.0),
    ]
}

fn pressure_port_pose(bank: usize, port: usize) -> (f64, f64) {
    let bank_center = if bank == 0 { -285.0 } else { 285.0 };
    let col = port % 2;
    let row = port / 2;
    let x = bank_center + if col == 0 { -34.0 } else { 34.0 };
    let z = 86.0 + row as f64 * 48.0;
    (x, z)
}

fn vane_x(index: usize) -> f64 {
    -((VANES_PER_ROW as f64 - 1.0) * 82.0) / 2.0 + index as f64 * 82.0
}

fn coupon_pose(index: usize) -> (f64, f64) {
    let col = index % 3;
    let row = index / 3;
    let x = -((3.0 - 1.0) * 146.0) / 2.0 + col as f64 * 146.0;
    let y = if row == 0 { -44.0 } else { 44.0 };
    (x, y)
}

fn token_x(index: usize) -> f64 {
    -((TOKEN_COUNT as f64 - 1.0) * 66.0) / 2.0 + index as f64 * 66.0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for suffix in OUTPUTS {
            let path = format!("{OUTPUT_PREFIX}_{suffix}.stl");
            assert!(path.starts_with(
                "output/closed_pass_through_tote_pressure_equalization_flow_balance_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn covers_requested_feature_groups() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"sealed_tote_receiver"));
        assert!(REQUIRED_FEATURES.contains(&"dual_pressure_port_panel"));
        assert!(REQUIRED_FEATURES.contains(&"hepa_purge_duct_surrogate"));
        assert!(REQUIRED_FEATURES.contains(&"flow_witness_vanes"));
        assert!(REQUIRED_FEATURES.contains(&"door_seal_coupons"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_token_lane"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn pressure_and_purge_counts_are_balanced() {
        assert_eq!(PRESSURE_BANKS, 2);
        assert_eq!(PRESSURE_PORTS, 12);
        assert_eq!(DIFFUSER_HOLES_X * DIFFUSER_HOLES_Z, 24);
        assert_eq!(WITNESS_ROWS, 2);
        assert_eq!(WITNESS_VANES, 14);
    }

    #[test]
    fn tote_receiver_fits_on_deck_with_service_clearance() {
        assert!(TOTE_OUTER_X + 2.0 * RECEIVER_RAIL_W < BASE_X - 240.0);
        assert!(TOTE_OUTER_Y + 2.0 * RECEIVER_RAIL_W < BASE_Y - 260.0);
        assert!(TOTE_INNER_X < TOTE_OUTER_X);
        assert!(TOTE_INNER_Z < TOTE_OUTER_Z);
    }

    #[test]
    fn evidence_and_status_lanes_cover_release_decision() {
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(TOKEN_COUNT, 8);
        assert_eq!(COUPON_COUNT, 6);
        assert_eq!(BARCODE_LANDS + CERTIFICATE_LANDS, 7);
        assert!(BRIDGE_X > RECORD_X);
    }

    #[test]
    fn latch_and_locator_counts_match_receiver_datum_plan() {
        assert_eq!(LOCATOR_COUNT, 4);
        assert_eq!(LATCH_COUNT, 4);
        assert_eq!(receiver_locator_points().len(), LOCATOR_COUNT);
    }
}
