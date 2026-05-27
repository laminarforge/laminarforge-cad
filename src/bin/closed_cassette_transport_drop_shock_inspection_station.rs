use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette/module transport drop-shock evidence inspection station.
//
// Design intent:
// - Receive a closed cassette or module transport carrier before tissue-chip runs.
// - Preserve evidence handling: shock indicator/token lands, barcode/custody
//   capture, seal/gasket witness pockets, leak witness tray, and camera bridge.
// - Keep release, hold, reject, clean, used, robot, and service zones physically
//   obvious without encoding a drop-test protocol or acceptance threshold.
//
// This is inspection and packaging CAD only. It is not a drop-test method, a
// release criterion, a sterility claim, or a transport qualification protocol.

const OUTPUTS: [&str; 13] = [
    "output/closed_cassette_transport_drop_shock_inspection_station_base_deck.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_cassette_receiving_nest.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_shock_indicator_token_lands.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_corner_edge_inspection_gauges.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_seal_gasket_witness_pockets.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_leak_witness_tray.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_accelerometer_logger_pocket.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_barcode_custody_lands.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_clean_used_segregation.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_evidence_camera_bridge.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_transport_drop_shock_inspection_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_OUTPUT_FEATURES: [&str; 12] = [
    "base_deck",
    "cassette_receiving_nest",
    "shock_indicator_token_lands",
    "corner_edge_inspection_gauges",
    "seal_gasket_witness_pockets",
    "leak_witness_tray",
    "accelerometer_logger_pocket",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATUS_LANES: [&str; 3] = ["release", "hold", "reject"];
const WORKFLOW_PORTS: [&str; 4] = ["incoming", "inspection", "custody", "quarantine"];

const STATION_X: f64 = 1360.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_PORT_D: f64 = 10.0;

const CASSETTE_CENTER: (f64, f64) = (-420.0, 145.0);
const CASSETTE_NEST_X: f64 = 430.0;
const CASSETTE_NEST_Y: f64 = 300.0;
const CASSETTE_NEST_Z: f64 = 62.0;
const CASSETTE_CLEARANCE_X: f64 = REVC_CHIP_LENGTH + 168.0;
const CASSETTE_CLEARANCE_Y: f64 = REVC_CHIP_WIDTH + 126.0;
const CASSETTE_CORNER_LOCATORS: usize = 4;
const CASSETTE_EDGE_DATUMS: usize = 6;

const SHOCK_CENTER: (f64, f64) = (55.0, 165.0);
const SHOCK_PANEL_X: f64 = 390.0;
const SHOCK_PANEL_Y: f64 = 250.0;
const SHOCK_PANEL_Z: f64 = 26.0;
const SHOCK_INDICATOR_LANDS: usize = 8;
const TOKEN_LANDS: usize = 12;
const WITNESS_STRIPS: usize = 4;

const GAUGE_CENTER: (f64, f64) = (470.0, 170.0);
const GAUGE_PANEL_X: f64 = 300.0;
const GAUGE_PANEL_Y: f64 = 275.0;
const GAUGE_PANEL_Z: f64 = 42.0;
const CORNER_GAUGES: usize = 4;
const EDGE_GAUGES: usize = 8;

const SEAL_CENTER: (f64, f64) = (-420.0, -185.0);
const SEAL_BLOCK_X: f64 = 420.0;
const SEAL_BLOCK_Y: f64 = 210.0;
const SEAL_BLOCK_Z: f64 = 42.0;
const GASKET_WITNESS_POCKETS: usize = 6;
const TAMPER_SEAL_WELLS: usize = 10;

const LEAK_CENTER: (f64, f64) = (20.0, -285.0);
const LEAK_TRAY_X: f64 = 360.0;
const LEAK_TRAY_Y: f64 = 180.0;
const LEAK_TRAY_Z: f64 = 36.0;
const LEAK_WITNESS_STRIPS: usize = 6;
const LEAK_RETENTION_WELLS: usize = 4;

const LOGGER_CENTER: (f64, f64) = (400.0, -255.0);
const LOGGER_BLOCK_X: f64 = 260.0;
const LOGGER_BLOCK_Y: f64 = 190.0;
const LOGGER_BLOCK_Z: f64 = 50.0;
const LOGGER_POCKETS: usize = 2;
const LOGGER_CABLE_PORTS: usize = 4;

const CUSTODY_CENTER: (f64, f64) = (-50.0, 365.0);
const CUSTODY_PANEL_X: f64 = 900.0;
const CUSTODY_PANEL_Y: f64 = 90.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 14;
const RFID_LANDS: usize = 8;
const CUSTODY_CARD_SLOTS: usize = 6;

const LANES_CENTER: (f64, f64) = (430.0, -65.0);
const LANES_X: f64 = 300.0;
const LANES_Y: f64 = 180.0;
const LANES_Z: f64 = 42.0;
const LANE_SLOTS_PER_STATUS: usize = 3;
const STATUS_LANE_PITCH_X: f64 = 92.0;
const STATUS_SLOT_X: f64 = 66.0;
const STATUS_SLOT_Y: f64 = 42.0;

const CLEAN_USED_WALL_X: f64 = -175.0;
const CLEAN_USED_WALL_Y: f64 = 600.0;
const CLEAN_USED_WALL_Z: f64 = 86.0;
const CLEAN_USED_AIR_GAP: f64 = 40.0;
const USED_RETURN_BIN_X: f64 = 210.0;
const USED_RETURN_BIN_Y: f64 = 96.0;

const CAMERA_CENTER: (f64, f64) = (0.0, 20.0);
const CAMERA_BRIDGE_SPAN_X: f64 = 1180.0;
const CAMERA_POST_X: f64 = 34.0;
const CAMERA_POST_Y: f64 = 44.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 236.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_BRIDGE_TOTAL_Z: f64 = CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z;
const CAMERA_PODS: usize = 4;
const LED_STRIPS: usize = 8;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 350.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 210.0;
const LEFT_CLEAN_CART_KEEP_OUT_X: f64 = 170.0;
const RIGHT_REJECT_SERVICE_KEEP_OUT_X: f64 = 190.0;
const OVERHEAD_CAMERA_KEEP_OUT_Z: f64 = 310.0;

#[derive(Clone, Copy)]
struct ComponentSpec {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let nest = cassette_receiving_nest();
    export(OUTPUTS[1], &nest);

    let shock = shock_indicator_token_lands();
    export(OUTPUTS[2], &shock);

    let gauges = corner_edge_inspection_gauges();
    export(OUTPUTS[3], &gauges);

    let seals = seal_gasket_witness_pockets();
    export(OUTPUTS[4], &seals);

    let leak = leak_witness_tray();
    export(OUTPUTS[5], &leak);

    let logger = accelerometer_logger_pocket();
    export(OUTPUTS[6], &logger);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[10], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        base + nest.translate(
            CASSETTE_CENTER.0,
            CASSETTE_CENTER.1,
            deck_insert_z(CASSETTE_NEST_Z),
        ) + shock.translate(SHOCK_CENTER.0, SHOCK_CENTER.1, deck_insert_z(SHOCK_PANEL_Z))
            + gauges.translate(GAUGE_CENTER.0, GAUGE_CENTER.1, deck_insert_z(GAUGE_PANEL_Z))
            + seals.translate(SEAL_CENTER.0, SEAL_CENTER.1, deck_insert_z(SEAL_BLOCK_Z))
            + leak.translate(LEAK_CENTER.0, LEAK_CENTER.1, deck_insert_z(LEAK_TRAY_Z))
            + logger.translate(
                LOGGER_CENTER.0,
                LOGGER_CENTER.1,
                deck_insert_z(LOGGER_BLOCK_Z),
            )
            + custody.translate(
                CUSTODY_CENTER.0,
                CUSTODY_CENTER.1,
                deck_insert_z(CUSTODY_PANEL_Z),
            )
            + lanes.translate(LANES_CENTER.0, LANES_CENTER.1, deck_insert_z(LANES_Z))
            + segregation.translate(0.0, 0.0, deck_insert_z(CLEAN_USED_WALL_Z))
            + camera.translate(CAMERA_CENTER.0, CAMERA_CENTER.1, BASE_Z / 2.0)
            + keepouts.translate(0.0, 0.0, BASE_Z / 2.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cassette transport drop/shock evidence inspection station:");
    println!(
        "  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm deck with leak/drip basin"
    );
    println!(
        "  Cassette nest:         {CASSETTE_NEST_X:.0}mm x {CASSETTE_NEST_Y:.0}mm receiver for Rev C chip/cassette envelope ({REVC_CHIP_LENGTH:.2}mm x {REVC_CHIP_WIDTH:.2}mm x {REVC_TOTAL_HEIGHT:.2}mm reference)"
    );
    println!(
        "  Evidence lands:        {SHOCK_INDICATOR_LANDS} shock indicator lands, {TOKEN_LANDS} custody/status token lands, {WITNESS_STRIPS} witness strip lands"
    );
    println!(
        "  Inspection gauges:     {CORNER_GAUGES} corner gauge pockets and {EDGE_GAUGES} edge witness gauges"
    );
    println!(
        "  Witness handling:      {GASKET_WITNESS_POCKETS} seal/gasket witness pockets, {TAMPER_SEAL_WELLS} tamper seal wells, {LEAK_WITNESS_STRIPS} leak witness strips"
    );
    println!(
        "  Traceability:          {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {CUSTODY_CARD_SLOTS} custody card slots, {LOGGER_POCKETS} accelerometer logger pockets"
    );
    println!(
        "  Disposition:           release/hold/reject lanes with {LANE_SLOTS_PER_STATUS} closed cassette positions each"
    );
    println!(
        "  Camera/keepouts:       {CAMERA_PODS} evidence camera pods, {LED_STRIPS} light strips, {CAMERA_BRIDGE_UNDERSIDE_Z:.0}mm bridge underside, {CAMERA_BRIDGE_TOTAL_Z:.0}mm bridge total height, {OVERHEAD_CAMERA_KEEP_OUT_Z:.0}mm overhead gauge"
    );
    println!(
        "  Limitation:            inspection and packaging CAD only; no drop-test protocol or acceptance criterion is encoded."
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn component_specs() -> [ComponentSpec; 8] {
    [
        ComponentSpec {
            name: "cassette_receiving_nest",
            center: CASSETTE_CENTER,
            width: CASSETTE_NEST_X,
            depth: CASSETTE_NEST_Y,
        },
        ComponentSpec {
            name: "shock_indicator_token_lands",
            center: SHOCK_CENTER,
            width: SHOCK_PANEL_X,
            depth: SHOCK_PANEL_Y,
        },
        ComponentSpec {
            name: "corner_edge_inspection_gauges",
            center: GAUGE_CENTER,
            width: GAUGE_PANEL_X,
            depth: GAUGE_PANEL_Y,
        },
        ComponentSpec {
            name: "seal_gasket_witness_pockets",
            center: SEAL_CENTER,
            width: SEAL_BLOCK_X,
            depth: SEAL_BLOCK_Y,
        },
        ComponentSpec {
            name: "leak_witness_tray",
            center: LEAK_CENTER,
            width: LEAK_TRAY_X,
            depth: LEAK_TRAY_Y,
        },
        ComponentSpec {
            name: "accelerometer_logger_pocket",
            center: LOGGER_CENTER,
            width: LOGGER_BLOCK_X,
            depth: LOGGER_BLOCK_Y,
        },
        ComponentSpec {
            name: "barcode_custody_lands",
            center: CUSTODY_CENTER,
            width: CUSTODY_PANEL_X,
            depth: CUSTODY_PANEL_Y,
        },
        ComponentSpec {
            name: "release_hold_reject_lanes",
            center: LANES_CENTER,
            width: LANES_X,
            depth: LANES_Y,
        },
    ]
}

fn assert_layout() {
    for spec in component_specs() {
        assert!(
            fits_on_station(spec.center, spec.width, spec.depth),
            "{} exceeds station footprint",
            spec.name
        );
    }

    let specs = component_specs();
    for left in 0..specs.len() {
        for right in left + 1..specs.len() {
            assert!(
                !overlaps(specs[left], specs[right]),
                "{} overlaps {}",
                specs[left].name,
                specs[right].name
            );
        }
    }

    assert!(CASSETTE_CLEARANCE_X > REVC_CHIP_LENGTH + 150.0);
    assert!(CASSETTE_CLEARANCE_Y > REVC_CHIP_WIDTH + 110.0);
    assert!(CASSETTE_NEST_Z > REVC_TOTAL_HEIGHT + 36.0);
    assert!(CAMERA_BRIDGE_UNDERSIDE_Z > CASSETTE_NEST_Z + 150.0);
    assert!(CAMERA_BRIDGE_UNDERSIDE_Z > LOGGER_BLOCK_Z + 160.0);
    assert!(CLEAN_USED_AIR_GAP >= 36.0);
    assert!(FRONT_ROBOT_KEEP_OUT_Y > 300.0);
    assert!(REAR_SERVICE_KEEP_OUT_Y > 180.0);
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    center.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && center.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn overlaps(a: ComponentSpec, b: ComponentSpec) -> bool {
    let ax_min = a.center.0 - a.width / 2.0;
    let ax_max = a.center.0 + a.width / 2.0;
    let ay_min = a.center.1 - a.depth / 2.0;
    let ay_max = a.center.1 + a.depth / 2.0;
    let bx_min = b.center.0 - b.width / 2.0;
    let bx_max = b.center.0 + b.width / 2.0;
    let by_min = b.center.1 - b.depth / 2.0;
    let by_max = b.center.1 + b.depth / 2.0;

    ax_min < bx_max && ax_max > bx_min && ay_min < by_max && ay_max > by_min
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_base_deck_plate",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_recessed_evidence_basin",
        STATION_X - 118.0,
        STATION_Y - 126.0,
        BASIN_DEPTH,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - BASIN_DEPTH / 2.0 + 0.2);
    let front_witness_channel = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_front_leak_witness_channel",
        STATION_X - 230.0,
        30.0,
        BASIN_DEPTH + 4.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 64.0, BASE_Z / 2.0 - 4.0);
    let drain_sump = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_right_front_drain_sump",
        118.0,
        70.0,
        BASIN_DEPTH + 6.0,
    )
    .translate(
        STATION_X / 2.0 - 118.0,
        -STATION_Y / 2.0 + 88.0,
        BASE_Z / 2.0 - 4.0,
    );
    let drain = centered_cylinder(
        "closed_cassette_transport_drop_shock_inspection_station_drain_port",
        DRAIN_PORT_D / 2.0,
        58.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 118.0,
        -STATION_Y / 2.0 + 32.0,
        BASE_Z / 2.0 - 4.0,
    );

    deck - basin
        - front_witness_channel
        - drain_sump
        - drain
        - component_socket_cuts()
        - mount_holes()
        + perimeter_rims()
        + workflow_route_lands()
}

fn component_socket_cuts() -> Part {
    let mut sockets = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_component_socket_cuts",
    );
    for spec in component_specs() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_{}_deck_socket",
                    spec.name
                ),
                spec.width + 8.0,
                spec.depth + 8.0,
                5.0,
            )
            .translate(spec.center.0, spec.center.1, BASE_Z / 2.0 - 2.4);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 48.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 48.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, -STATION_Y / 2.0 + 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (-STATION_X / 2.0 + 54.0, 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_cassette_transport_drop_shock_inspection_station_m6_mount_{i}"),
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
        "closed_cassette_transport_drop_shock_inspection_station_front_evidence_curb",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, z);
    let rear = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_rear_service_curb",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_left_clean_cart_curb",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_right_reject_service_curb",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn workflow_route_lands() -> Part {
    let incoming = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_incoming_to_nest_route_land",
        430.0,
        8.0,
        6.0,
    )
    .translate(-410.0, 314.0, BASE_Z / 2.0 + 3.0);
    let inspection = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_nest_to_inspection_route_land",
        580.0,
        8.0,
        6.0,
    )
    .translate(-80.0, 38.0, BASE_Z / 2.0 + 3.0);
    let disposition = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_inspection_to_disposition_route_land",
        430.0,
        8.0,
        6.0,
    )
    .rotate(0.0, 0.0, -24.0)
    .translate(245.0, -35.0, BASE_Z / 2.0 + 3.0);

    incoming + inspection + disposition + workflow_port_tabs()
}

fn workflow_port_tabs() -> Part {
    let mut tabs =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_workflow_port_tabs");
    for (i, port) in WORKFLOW_PORTS.iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!(
                "closed_cassette_transport_drop_shock_inspection_station_{port}_workflow_port_tab"
            ),
                92.0,
                18.0,
                8.0,
            )
            .translate(
                centered_index(i, WORKFLOW_PORTS.len(), 190.0),
                STATION_Y / 2.0 - 76.0,
                BASE_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn cassette_receiving_nest() -> Part {
    let body = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_body",
        CASSETTE_NEST_X,
        CASSETTE_NEST_Y,
        CASSETTE_NEST_Z,
    );
    let cassette_clearance = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_cassette_clearance",
        CASSETTE_CLEARANCE_X,
        CASSETTE_CLEARANCE_Y,
        CASSETTE_NEST_Z + 6.0,
    )
    .translate(0.0, 0.0, 8.0);
    let front_loading_throat = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_front_loading_throat",
        CASSETTE_CLEARANCE_X - 70.0,
        70.0,
        CASSETTE_NEST_Z + 8.0,
    )
    .translate(0.0, -CASSETTE_NEST_Y / 2.0 + 28.0, 8.0);
    let rear_datum = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_rear_datum_stop",
        CASSETTE_NEST_X - 76.0,
        18.0,
        36.0,
    )
    .translate(0.0, CASSETTE_NEST_Y / 2.0 + 9.0, 0.0);
    let gasket_land = rectangular_frame_xy(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_gasket_witness_land",
        CASSETTE_NEST_X - 48.0,
        CASSETTE_NEST_Y - 44.0,
        8.0,
        CASSETTE_CLEARANCE_X + 16.0,
        CASSETTE_CLEARANCE_Y + 18.0,
    )
    .translate(0.0, 0.0, CASSETTE_NEST_Z / 2.0 + 4.0);

    body - cassette_clearance - front_loading_throat
        + rear_datum
        + gasket_land
        + receiver_locator_bosses()
        + receiver_edge_datums()
        + receiver_spring_clip_lands()
}

fn receiver_locator_bosses() -> Part {
    let mut locators = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_receiver_locator_bosses",
    );
    for (i, (x, y)) in receiver_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!(
                "closed_cassette_transport_drop_shock_inspection_station_receiver_locator_boss_{i}"
            ),
            16.0,
            12.0,
            36,
        )
        .translate(*x, *y, CASSETTE_NEST_Z / 2.0 + 6.0);
        let pin_socket = centered_cylinder(
            format!(
                "closed_cassette_transport_drop_shock_inspection_station_receiver_locator_socket_{i}"
            ),
            6.8,
            14.0,
            28,
        )
        .translate(*x, *y, CASSETTE_NEST_Z / 2.0 + 6.0);
        locators = locators + (boss - pin_socket);
    }
    locators
}

fn receiver_locator_points() -> [(f64, f64); CASSETTE_CORNER_LOCATORS] {
    [
        (
            -CASSETTE_CLEARANCE_X / 2.0 - 18.0,
            -CASSETTE_CLEARANCE_Y / 2.0 - 18.0,
        ),
        (
            CASSETTE_CLEARANCE_X / 2.0 + 18.0,
            -CASSETTE_CLEARANCE_Y / 2.0 - 18.0,
        ),
        (
            -CASSETTE_CLEARANCE_X / 2.0 - 18.0,
            CASSETTE_CLEARANCE_Y / 2.0 + 18.0,
        ),
        (
            CASSETTE_CLEARANCE_X / 2.0 + 18.0,
            CASSETTE_CLEARANCE_Y / 2.0 + 18.0,
        ),
    ]
}

fn receiver_edge_datums() -> Part {
    let mut datums =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_receiver_edge_datums");
    for i in 0..CASSETTE_EDGE_DATUMS {
        let top_edge = i < CASSETTE_EDGE_DATUMS / 2;
        let local_i = i % (CASSETTE_EDGE_DATUMS / 2);
        datums = datums
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_receiver_edge_datum_{i}"
                ),
                54.0,
                12.0,
                20.0,
            )
            .translate(
                centered_index(local_i, CASSETTE_EDGE_DATUMS / 2, 108.0),
                if top_edge {
                    CASSETTE_CLEARANCE_Y / 2.0 + 34.0
                } else {
                    -CASSETTE_CLEARANCE_Y / 2.0 - 34.0
                },
                CASSETTE_NEST_Z / 2.0 + 10.0,
            );
    }
    datums
}

fn receiver_spring_clip_lands() -> Part {
    let mut clips =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_receiver_clip_lands");
    for (i, x) in [-154.0, -52.0, 52.0, 154.0].iter().enumerate() {
        clips = clips
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_receiver_retention_clip_land_{i}"
                ),
                54.0,
                16.0,
                14.0,
            )
            .translate(*x, -CASSETTE_NEST_Y / 2.0 - 8.0, 0.0);
    }
    clips
}

fn shock_indicator_token_lands() -> Part {
    let panel = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_shock_indicator_panel",
        SHOCK_PANEL_X,
        SHOCK_PANEL_Y,
        SHOCK_PANEL_Z,
    );
    let mut indicator_recesses = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_shock_indicator_recesses",
    );
    let mut raised_tokens =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_token_lands");

    for i in 0..SHOCK_INDICATOR_LANDS {
        let row = i / 4;
        let col = i % 4;
        indicator_recesses = indicator_recesses
            + top_pocket(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_shock_indicator_window_{i}"
                ),
                54.0,
                36.0,
                8.0,
                centered_index(col, 4, 78.0),
                46.0 + centered_index(row, 2, 54.0),
                SHOCK_PANEL_Z,
            );
    }

    for i in 0..TOKEN_LANDS {
        let row = i / 6;
        let col = i % 6;
        raised_tokens = raised_tokens
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_status_token_land_{i}"
                ),
                10.0,
                5.0,
                28,
            )
            .translate(
                centered_index(col, 6, 52.0),
                -56.0 + centered_index(row, 2, 42.0),
                SHOCK_PANEL_Z / 2.0 + 2.5,
            );
    }

    panel - indicator_recesses + raised_tokens + shock_witness_strip_lands()
}

fn shock_witness_strip_lands() -> Part {
    let mut strips =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_shock_witness_strips");
    for i in 0..WITNESS_STRIPS {
        strips = strips
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_shock_witness_strip_land_{i}"
                ),
                72.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(i, WITNESS_STRIPS, 86.0),
                -SHOCK_PANEL_Y / 2.0 + 24.0,
                SHOCK_PANEL_Z / 2.0 + 2.5,
            );
    }
    strips
}

fn corner_edge_inspection_gauges() -> Part {
    let panel = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_corner_edge_gauge_panel",
        GAUGE_PANEL_X,
        GAUGE_PANEL_Y,
        GAUGE_PANEL_Z,
    );
    let edge_scan_basin = top_pocket(
        "closed_cassette_transport_drop_shock_inspection_station_edge_scan_basin",
        GAUGE_PANEL_X - 42.0,
        GAUGE_PANEL_Y - 48.0,
        8.0,
        0.0,
        0.0,
        GAUGE_PANEL_Z,
    );

    panel - edge_scan_basin + corner_gauge_blocks() + edge_gauge_combs() + gauge_reference_steps()
}

fn corner_gauge_blocks() -> Part {
    let mut corners =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_corner_gauges");
    for (i, (x, y)) in [
        (-GAUGE_PANEL_X / 2.0 + 42.0, -GAUGE_PANEL_Y / 2.0 + 42.0),
        (GAUGE_PANEL_X / 2.0 - 42.0, -GAUGE_PANEL_Y / 2.0 + 42.0),
        (-GAUGE_PANEL_X / 2.0 + 42.0, GAUGE_PANEL_Y / 2.0 - 42.0),
        (GAUGE_PANEL_X / 2.0 - 42.0, GAUGE_PANEL_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        let x_leg = centered_cube(
            format!(
                "closed_cassette_transport_drop_shock_inspection_station_corner_gauge_x_leg_{i}"
            ),
            72.0,
            12.0,
            28.0,
        )
        .translate(*x, *y, GAUGE_PANEL_Z / 2.0 + 14.0);
        let y_leg = centered_cube(
            format!(
                "closed_cassette_transport_drop_shock_inspection_station_corner_gauge_y_leg_{i}"
            ),
            12.0,
            72.0,
            28.0,
        )
        .translate(*x, *y, GAUGE_PANEL_Z / 2.0 + 14.0);
        let corner_token = centered_cylinder(
            format!(
                "closed_cassette_transport_drop_shock_inspection_station_corner_photo_token_{i}"
            ),
            8.0,
            6.0,
            24,
        )
        .translate(*x, *y, GAUGE_PANEL_Z / 2.0 + 3.0);
        corners = corners + x_leg + y_leg + corner_token;
    }
    corners
}

fn edge_gauge_combs() -> Part {
    let mut combs =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_edge_gauge_combs");
    for i in 0..EDGE_GAUGES {
        let top_edge = i < EDGE_GAUGES / 2;
        let local_i = i % (EDGE_GAUGES / 2);
        combs = combs
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_edge_witness_gauge_{i}"
                ),
                12.0,
                48.0,
                24.0,
            )
            .translate(
                centered_index(local_i, EDGE_GAUGES / 2, 56.0),
                if top_edge { 66.0 } else { -66.0 },
                GAUGE_PANEL_Z / 2.0 + 12.0,
            );
    }
    combs
}

fn gauge_reference_steps() -> Part {
    let low_step = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_low_edge_reference_step",
        GAUGE_PANEL_X - 88.0,
        14.0,
        7.0,
    )
    .translate(0.0, -8.0, GAUGE_PANEL_Z / 2.0 + 3.5);
    let high_step = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_high_edge_reference_step",
        GAUGE_PANEL_X - 132.0,
        14.0,
        14.0,
    )
    .translate(0.0, 18.0, GAUGE_PANEL_Z / 2.0 + 7.0);
    low_step + high_step
}

fn seal_gasket_witness_pockets() -> Part {
    let block = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_seal_witness_block",
        SEAL_BLOCK_X,
        SEAL_BLOCK_Y,
        SEAL_BLOCK_Z,
    );
    let mut pockets = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_gasket_witness_pocket_cuts",
    );
    for i in 0..GASKET_WITNESS_POCKETS {
        let row = i / 3;
        let col = i % 3;
        pockets = pockets
            + top_pocket(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_gasket_witness_pocket_{i}"
                ),
                96.0,
                34.0,
                10.0,
                centered_index(col, 3, 122.0),
                34.0 + centered_index(row, 2, 58.0),
                SEAL_BLOCK_Z,
            );
    }

    let tamper_wells = tamper_seal_wells();
    let seal_photo_ruler = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_seal_photo_ruler_land",
        SEAL_BLOCK_X - 52.0,
        14.0,
        6.0,
    )
    .translate(0.0, -SEAL_BLOCK_Y / 2.0 + 22.0, SEAL_BLOCK_Z / 2.0 + 3.0);

    block - pockets - tamper_wells + seal_photo_ruler + gasket_frame_lands()
}

fn tamper_seal_wells() -> Part {
    let mut wells =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_tamper_seal_wells");
    for i in 0..TAMPER_SEAL_WELLS {
        wells = wells
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_tamper_seal_well_{i}"
                ),
                8.0,
                SEAL_BLOCK_Z + 4.0,
                24,
            )
            .translate(centered_index(i, TAMPER_SEAL_WELLS, 34.0), -54.0, 4.0);
    }
    wells
}

fn gasket_frame_lands() -> Part {
    let outer = rectangular_frame_xy(
        "closed_cassette_transport_drop_shock_inspection_station_outer_gasket_witness_frame",
        SEAL_BLOCK_X - 50.0,
        SEAL_BLOCK_Y - 34.0,
        6.0,
        SEAL_BLOCK_X - 92.0,
        SEAL_BLOCK_Y - 76.0,
    )
    .translate(0.0, 0.0, SEAL_BLOCK_Z / 2.0 + 3.0);
    let inner = rectangular_frame_xy(
        "closed_cassette_transport_drop_shock_inspection_station_inner_gasket_witness_frame",
        205.0,
        118.0,
        5.0,
        168.0,
        82.0,
    )
    .translate(0.0, 0.0, SEAL_BLOCK_Z / 2.0 + 7.5);
    outer + inner
}

fn leak_witness_tray() -> Part {
    let tray = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_leak_witness_tray_body",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let basin = top_pocket(
        "closed_cassette_transport_drop_shock_inspection_station_leak_witness_absorbent_basin",
        LEAK_TRAY_X - 42.0,
        LEAK_TRAY_Y - 44.0,
        12.0,
        0.0,
        0.0,
        LEAK_TRAY_Z,
    );
    let front_sump = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_leak_witness_front_sump",
        96.0,
        30.0,
        LEAK_TRAY_Z + 4.0,
    )
    .translate(LEAK_TRAY_X / 2.0 - 72.0, -LEAK_TRAY_Y / 2.0 + 26.0, 4.0);

    tray - basin - front_sump + leak_witness_strip_lands() + leak_retention_wells()
}

fn leak_witness_strip_lands() -> Part {
    let mut strips = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_leak_witness_strip_lands",
    );
    for i in 0..LEAK_WITNESS_STRIPS {
        strips = strips
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_leak_witness_strip_{i}"
                ),
                34.0,
                LEAK_TRAY_Y - 62.0,
                4.0,
            )
            .translate(
                centered_index(i, LEAK_WITNESS_STRIPS, 46.0),
                0.0,
                LEAK_TRAY_Z / 2.0 + 2.0,
            );
    }
    strips
}

fn leak_retention_wells() -> Part {
    let mut wells =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_leak_retention_wells");
    for i in 0..LEAK_RETENTION_WELLS {
        wells = wells
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_leak_retention_well_{i}"
                ),
                10.0,
                8.0,
                28,
            )
            .translate(
                centered_index(i, LEAK_RETENTION_WELLS, 54.0),
                -LEAK_TRAY_Y / 2.0 + 52.0,
                LEAK_TRAY_Z / 2.0 + 4.0,
            );
    }
    wells
}

fn accelerometer_logger_pocket() -> Part {
    let block = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_accelerometer_logger_block",
        LOGGER_BLOCK_X,
        LOGGER_BLOCK_Y,
        LOGGER_BLOCK_Z,
    );
    let mut pockets = Part::empty(
        "closed_cassette_transport_drop_shock_inspection_station_accelerometer_logger_pocket_cuts",
    );
    for i in 0..LOGGER_POCKETS {
        pockets = pockets
            + top_pocket(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_accelerometer_logger_pocket_{i}"
                ),
                86.0,
                64.0,
                14.0,
                centered_index(i, LOGGER_POCKETS, 100.0),
                12.0,
                LOGGER_BLOCK_Z,
            );
    }

    block - pockets - logger_cable_ports() + logger_bumper_lands() + logger_serial_card_lands()
}

fn logger_cable_ports() -> Part {
    let mut ports =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_logger_cable_ports");
    for i in 0..LOGGER_CABLE_PORTS {
        ports = ports
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_logger_cable_port_{i}"
                ),
                4.0,
                LOGGER_BLOCK_Y + 6.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, LOGGER_CABLE_PORTS, 54.0), 0.0, 0.0);
    }
    ports
}

fn logger_bumper_lands() -> Part {
    let mut bumpers =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_logger_bumper_lands");
    for (i, (x, y)) in [
        (-LOGGER_BLOCK_X / 2.0 + 28.0, -LOGGER_BLOCK_Y / 2.0 + 28.0),
        (LOGGER_BLOCK_X / 2.0 - 28.0, -LOGGER_BLOCK_Y / 2.0 + 28.0),
        (-LOGGER_BLOCK_X / 2.0 + 28.0, LOGGER_BLOCK_Y / 2.0 - 28.0),
        (LOGGER_BLOCK_X / 2.0 - 28.0, LOGGER_BLOCK_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        bumpers = bumpers
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_logger_bumper_land_{i}"
                ),
                9.0,
                6.0,
                24,
            )
            .translate(*x, *y, LOGGER_BLOCK_Z / 2.0 + 3.0);
    }
    bumpers
}

fn logger_serial_card_lands() -> Part {
    centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_logger_serial_card_land",
        LOGGER_BLOCK_X - 46.0,
        16.0,
        6.0,
    )
    .translate(
        0.0,
        -LOGGER_BLOCK_Y / 2.0 + 22.0,
        LOGGER_BLOCK_Z / 2.0 + 3.0,
    )
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_barcode_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    panel - barcode_recesses() - custody_card_slots() + rfid_lands() + custody_token_lands()
}

fn barcode_recesses() -> Part {
    let mut recesses =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_barcode_recesses");
    for i in 0..BARCODE_LANDS {
        let row = i / 7;
        let col = i % 7;
        recesses = recesses
            + top_pocket(
                format!("closed_cassette_transport_drop_shock_inspection_station_barcode_land_{i}"),
                64.0,
                18.0,
                5.0,
                -238.0 + centered_index(col, 7, 76.0),
                centered_index(row, 2, 34.0),
                CUSTODY_PANEL_Z,
            );
    }
    recesses
}

fn rfid_lands() -> Part {
    let mut lands =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_rfid_lands");
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_cassette_transport_drop_shock_inspection_station_rfid_pad_{i}"),
                28.0,
                28.0,
                5.0,
            )
            .translate(
                258.0 + centered_index(i % 4, 4, 42.0),
                centered_index(i / 4, 2, 34.0),
                CUSTODY_PANEL_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_custody_card_slots");
    for i in 0..CUSTODY_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_custody_card_slot_{i}"
                ),
                4.0,
                62.0,
                CUSTODY_PANEL_Z + 4.0,
            )
            .rotate(0.0, 0.0, -8.0)
            .translate(
                392.0 + centered_index(i, CUSTODY_CARD_SLOTS, 24.0),
                0.0,
                0.0,
            );
    }
    slots
}

fn custody_token_lands() -> Part {
    let incoming = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_custody_incoming_token_land",
        70.0,
        16.0,
        6.0,
    )
    .translate(
        -CUSTODY_PANEL_X / 2.0 + 44.0,
        24.0,
        CUSTODY_PANEL_Z / 2.0 + 3.0,
    );
    let signed = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_custody_signed_token_land",
        70.0,
        16.0,
        6.0,
    )
    .translate(
        -CUSTODY_PANEL_X / 2.0 + 44.0,
        -24.0,
        CUSTODY_PANEL_Z / 2.0 + 3.0,
    );
    incoming + signed
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_release_hold_reject_panel",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    let mut slots =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_lane_slot_cuts");
    let mut flags =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_lane_flags");

    for (lane, status) in STATUS_LANES.iter().enumerate() {
        let x = status_lane_x(lane);
        flags = flags
            + centered_cube(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_{status}_lane_flag"
                ),
                66.0,
                12.0,
                16.0,
            )
            .translate(x, -LANES_Y / 2.0 + 18.0, LANES_Z / 2.0 + 8.0);

        for slot in 0..LANE_SLOTS_PER_STATUS {
            slots = slots
                + centered_cube(
                    format!(
                        "closed_cassette_transport_drop_shock_inspection_station_{status}_lane_slot_{slot}"
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    LANES_Z + 4.0,
                )
                .translate(x, centered_index(slot, LANE_SLOTS_PER_STATUS, 46.0), 8.0);
        }
    }

    panel - slots + lane_dividers() + flags
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES.len(), STATUS_LANE_PITCH_X)
}

fn lane_dividers() -> Part {
    let mut dividers =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_lane_dividers");
    for (i, x) in [-STATUS_LANE_PITCH_X / 2.0, STATUS_LANE_PITCH_X / 2.0]
        .iter()
        .enumerate()
    {
        dividers = dividers
            + centered_cube(
                format!("closed_cassette_transport_drop_shock_inspection_station_lane_divider_{i}"),
                10.0,
                LANES_Y - 28.0,
                LANES_Z + 22.0,
            )
            .translate(*x, 0.0, 11.0);
    }
    dividers
}

fn clean_used_segregation() -> Part {
    let wall = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_clean_used_segregation_wall",
        22.0,
        CLEAN_USED_WALL_Y,
        CLEAN_USED_WALL_Z,
    )
    .translate(CLEAN_USED_WALL_X, -60.0, 0.0);
    let clean_lane = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_clean_incoming_lane_land",
        350.0,
        18.0,
        12.0,
    )
    .translate(-420.0, -390.0, -CLEAN_USED_WALL_Z / 2.0 + 6.0);
    let used_return_bin = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_used_return_bin_envelope",
        USED_RETURN_BIN_X,
        USED_RETURN_BIN_Y,
        40.0,
    )
    .translate(285.0, -380.0, -CLEAN_USED_WALL_Z / 2.0 + 20.0);
    let used_return_pocket = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_used_return_bin_pocket",
        USED_RETURN_BIN_X - 42.0,
        USED_RETURN_BIN_Y - 36.0,
        44.0,
    )
    .translate(285.0, -380.0, -CLEAN_USED_WALL_Z / 2.0 + 24.0);
    let one_way_gate = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_clean_to_used_one_way_gate_land",
        84.0,
        18.0,
        18.0,
    )
    .translate(
        CLEAN_USED_WALL_X + 64.0,
        260.0,
        -CLEAN_USED_WALL_Z / 2.0 + 9.0,
    );

    wall + clean_lane + (used_return_bin - used_return_pocket) + one_way_gate
}

fn evidence_camera_bridge() -> Part {
    let post_z = CAMERA_BRIDGE_UNDERSIDE_Z;
    let left_post = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(-CAMERA_BRIDGE_SPAN_X / 2.0, 0.0, post_z / 2.0);
    let right_post = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(CAMERA_BRIDGE_SPAN_X / 2.0, 0.0, post_z / 2.0);
    let rear_left_post = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_rear_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(-CAMERA_BRIDGE_SPAN_X / 2.0, 170.0, post_z / 2.0);
    let rear_right_post = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_rear_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        post_z,
    )
    .translate(CAMERA_BRIDGE_SPAN_X / 2.0, 170.0, post_z / 2.0);
    let front_beam = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_front_beam",
        CAMERA_BRIDGE_SPAN_X + CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_rear_beam",
        CAMERA_BRIDGE_SPAN_X + CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        170.0,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let cross_beam = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_camera_bridge_cross_beam",
        38.0,
        210.0,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        85.0,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );

    left_post
        + right_post
        + rear_left_post
        + rear_right_post
        + front_beam
        + rear_beam
        + cross_beam
        + camera_pods()
        + led_strip_lands()
}

fn camera_pods() -> Part {
    let mut pods =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_camera_pods");
    for i in 0..CAMERA_PODS {
        pods = pods
            + centered_cube(
                format!("closed_cassette_transport_drop_shock_inspection_station_camera_pod_{i}"),
                58.0,
                44.0,
                28.0,
            )
            .translate(
                centered_index(i, CAMERA_PODS, 250.0),
                -32.0,
                CAMERA_BRIDGE_UNDERSIDE_Z - 14.0,
            )
            + centered_cylinder(
                format!(
                    "closed_cassette_transport_drop_shock_inspection_station_camera_lens_clearance_{i}"
                ),
                11.0,
                10.0,
                28,
            )
            .translate(
                centered_index(i, CAMERA_PODS, 250.0),
                -32.0,
                CAMERA_BRIDGE_UNDERSIDE_Z - 31.0,
            );
    }
    pods
}

fn led_strip_lands() -> Part {
    let mut strips =
        Part::empty("closed_cassette_transport_drop_shock_inspection_station_led_strip_lands");
    for i in 0..LED_STRIPS {
        let rear = i >= LED_STRIPS / 2;
        let local_i = i % (LED_STRIPS / 2);
        strips = strips
            + centered_cube(
                format!("closed_cassette_transport_drop_shock_inspection_station_led_strip_{i}"),
                118.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(local_i, LED_STRIPS / 2, 190.0),
                if rear { 138.0 } else { 32.0 },
                CAMERA_BRIDGE_UNDERSIDE_Z - 18.0,
            );
    }
    strips
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_front_robot_keepout_gauge",
        STATION_X - 120.0,
        16.0,
        56.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, 28.0);
    let rear_service = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_rear_service_keepout_gauge",
        STATION_X - 140.0,
        16.0,
        48.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y, 24.0);
    let left_cart = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_left_clean_cart_keepout_gauge",
        16.0,
        STATION_Y - 150.0,
        48.0,
    )
    .translate(-STATION_X / 2.0 + LEFT_CLEAN_CART_KEEP_OUT_X, 0.0, 24.0);
    let right_service = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_right_reject_service_keepout_gauge",
        16.0,
        STATION_Y - 150.0,
        48.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_REJECT_SERVICE_KEEP_OUT_X,
        0.0,
        24.0,
    );
    let overhead = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_overhead_camera_service_gauge",
        120.0,
        120.0,
        12.0,
    )
    .translate(0.0, 95.0, OVERHEAD_CAMERA_KEEP_OUT_Z);
    let robot_gripper_window = centered_cube(
        "closed_cassette_transport_drop_shock_inspection_station_robot_gripper_clearance_window",
        460.0,
        14.0,
        52.0,
    )
    .translate(170.0, -STATION_Y / 2.0 + 160.0, 26.0);

    front_robot + rear_service + left_cart + right_service + overhead + robot_gripper_window
}

fn rectangular_frame_xy(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let rail_x = (outer_x - inner_x) / 2.0;
    let rail_y = (outer_y - inner_y) / 2.0;
    let front = centered_cube(format!("{name}_front"), outer_x, rail_y, z).translate(
        0.0,
        -outer_y / 2.0 + rail_y / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear"), outer_x, rail_y, z).translate(
        0.0,
        outer_y / 2.0 - rail_y / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left"), rail_x, inner_y, z).translate(
        -outer_x / 2.0 + rail_x / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail_x, inner_y, z).translate(
        outer_x / 2.0 - rail_x / 2.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn top_pocket(
    name: impl Into<String>,
    x: f64,
    y: f64,
    depth: f64,
    tx: f64,
    ty: f64,
    body_z: f64,
) -> Part {
    centered_cube(name, x, y, depth + 0.8).translate(tx, ty, body_z / 2.0 - depth / 2.0 + 0.4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_stable_unique_and_station_scoped() {
        assert_eq!(OUTPUTS.len(), 13);

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());

        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_cassette_transport_drop_shock_inspection_station_")
            );
            assert!(path.ends_with(".stl"));
        }

        assert_eq!(
            OUTPUTS[0],
            "output/closed_cassette_transport_drop_shock_inspection_station_base_deck.stl"
        );
        assert_eq!(
            OUTPUTS[12],
            "output/closed_cassette_transport_drop_shock_inspection_station_assembly.stl"
        );

        for feature in REQUIRED_OUTPUT_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing required output feature: {feature}"
            );
        }
    }

    #[test]
    fn geometry_assumptions_keep_station_clear_and_inspection_only() {
        assert_layout();
        assert_eq!(STATUS_LANES, ["release", "hold", "reject"]);
        assert_eq!(
            WORKFLOW_PORTS,
            ["incoming", "inspection", "custody", "quarantine"]
        );
        assert_eq!(SHOCK_INDICATOR_LANDS, 8);
        assert_eq!(TOKEN_LANDS, 12);
        assert_eq!(GASKET_WITNESS_POCKETS, 6);
        assert_eq!(LEAK_WITNESS_STRIPS, 6);
        assert_eq!(LOGGER_POCKETS, 2);
        assert_eq!(CAMERA_PODS, 4);
        assert!(CAMERA_BRIDGE_TOTAL_Z < OVERHEAD_CAMERA_KEEP_OUT_Z);
        assert!(FRONT_ROBOT_KEEP_OUT_Y > REAR_SERVICE_KEEP_OUT_Y);
    }

    #[test]
    fn cassette_nest_protects_rev_c_envelope_without_acceptance_thresholds() {
        assert!(CASSETTE_CLEARANCE_X > REVC_CHIP_LENGTH + 150.0);
        assert!(CASSETTE_CLEARANCE_Y > REVC_CHIP_WIDTH + 110.0);
        assert!(CASSETTE_NEST_Z > REVC_TOTAL_HEIGHT + 36.0);
        assert!(CASSETTE_EDGE_DATUMS >= 6);
        assert!(CORNER_GAUGES == 4);
        assert!(EDGE_GAUGES >= 8);
    }
}
