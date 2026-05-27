use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette flow-path pre-run air-slug displacement validation station.
//
// This generator packages a mechanical validation fixture for exercising a
// cassette surrogate before a run: slug challenge routing, optical witness
// windows, purge capture, custody surfaces, disposition gates, and evidence
// camera/robot access markers. It is mechanical validation packaging only. It
// is not a sterile-process claim, biological SOP, pressure-rated device, or an
// acceptance criterion for a process or product.

#[cfg(test)]
const OUTPUT_PREFIX: &str =
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_containment_deck.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_cassette_surrogate_dock.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_inlet_outlet_slug_challenge_manifold.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_optical_bubble_witness_windows.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_staged_prime_token_rail.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_low_point_purge_pockets.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_pressure_flow_tap_bosses.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_waste_retain_split_capture.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_route_barcode_rfid_custody_lands.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_release_hold_reject_gates.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_camera_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_cassette_flow_path_pre_run_air_slug_displacement_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 26] = [
    "cassette_surrogate_dock",
    "cassette_datum_rails",
    "cassette_orientation_keys",
    "inlet_slug_challenge_ports",
    "outlet_slug_challenge_ports",
    "slug_displacement_lanes",
    "optical_bubble_witness_windows",
    "witness_backlight_slot",
    "staged_prime_token_rail",
    "prime_stage_tokens",
    "low_point_purge_pockets",
    "purge_route_troughs",
    "pressure_tap_bosses",
    "flow_tap_bosses",
    "waste_split_capture_cells",
    "retain_split_capture_wells",
    "route_barcode_lands",
    "rfid_custody_lands",
    "custody_tamper_lands",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "camera_evidence_bridge",
    "evidence_fiducials",
    "robot_keepouts",
    "service_keepouts",
];

#[cfg(test)]
const OUT_OF_SCOPE_CLAIMS: [&str; 5] = [
    "sterile_process_claim",
    "biological_standard_operating_procedure",
    "pressure_rated_device",
    "process_acceptance_criterion",
    "product_release_criterion",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 24.0;
const RIM_Z: f64 = 52.0;
const SUMP_X: f64 = 1150.0;
const SUMP_Y: f64 = 710.0;
const SUMP_Z: f64 = 5.0;
const DRAIN_D: f64 = 18.0;
const DATUM_BOSSES: usize = 10;

const CASSETTE_DOCK_X: f64 = 350.0;
const CASSETTE_DOCK_Y: f64 = 245.0;
const CASSETTE_DOCK_Z: f64 = 48.0;
const CASSETTE_DOCK_POS: (f64, f64) = (-445.0, 250.0);
const CASSETTE_RECESS_X: f64 = 278.0;
const CASSETTE_RECESS_Y: f64 = 172.0;
const CASSETTE_RECESS_Z: f64 = 22.0;
const CASSETTE_DATUM_PINS: usize = 6;
const CASSETTE_ORIENTATION_KEYS: usize = 4;
const GRIPPER_RELIEFS: usize = 4;

const MANIFOLD_X: f64 = 500.0;
const MANIFOLD_Y: f64 = 190.0;
const MANIFOLD_Z: f64 = 46.0;
const MANIFOLD_POS: (f64, f64) = (0.0, 255.0);
const SLUG_LANES: usize = 6;
const SLUG_LANE_PITCH: f64 = 66.0;
const SLUG_LANE_D: f64 = 9.0;
const INLET_PORTS: usize = 3;
const OUTLET_PORTS: usize = 3;
const CHALLENGE_PORT_D: f64 = 28.0;

const WITNESS_X: f64 = 500.0;
const WITNESS_Y: f64 = 136.0;
const WITNESS_Z: f64 = 30.0;
const WITNESS_POS: (f64, f64) = (-40.0, 45.0);
const BUBBLE_WINDOWS: usize = 7;
const WINDOW_PITCH: f64 = 58.0;
const WINDOW_D: f64 = 28.0;
const OPTICAL_FIDUCIALS: usize = 8;

const TOKEN_RAIL_X: f64 = 425.0;
const TOKEN_RAIL_Y: f64 = 96.0;
const TOKEN_RAIL_Z: f64 = 32.0;
const TOKEN_RAIL_POS: (f64, f64) = (395.0, 100.0);
const PRIME_STAGE_TOKENS: usize = 6;
const PRIME_TOKEN_D: f64 = 28.0;
const PRIME_TOKEN_PITCH: f64 = 58.0;
const PRIME_STEP_MARKERS: usize = PRIME_STAGE_TOKENS;

const PURGE_X: f64 = 400.0;
const PURGE_Y: f64 = 145.0;
const PURGE_Z: f64 = 38.0;
const PURGE_POS: (f64, f64) = (390.0, -100.0);
const LOW_POINT_PURGE_POCKETS: usize = 5;
const PURGE_POCKET_D: f64 = 33.0;
const PURGE_POCKET_PITCH: f64 = 61.0;
const PURGE_TROUGHS: usize = 4;

const TAP_BAR_X: f64 = 420.0;
const TAP_BAR_Y: f64 = 116.0;
const TAP_BAR_Z: f64 = 42.0;
const TAP_BAR_POS: (f64, f64) = (-400.0, -40.0);
const PRESSURE_TAPS: usize = 4;
const FLOW_TAPS: usize = 3;
const TAP_BOSS_D: f64 = 34.0;
const TAP_BORE_D: f64 = 7.0;
const TAP_PITCH: f64 = 62.0;

const CAPTURE_X: f64 = 390.0;
const CAPTURE_Y: f64 = 205.0;
const CAPTURE_Z: f64 = 50.0;
const CAPTURE_POS: (f64, f64) = (-425.0, -285.0);
const WASTE_CAPTURE_CELLS: usize = 4;
const RETAIN_CAPTURE_WELLS: usize = 6;
const CAPTURE_CELL_X: f64 = 66.0;
const CAPTURE_CELL_Y: f64 = 52.0;
const RETAIN_WELL_D: f64 = 24.0;

const CUSTODY_X: f64 = 330.0;
const CUSTODY_Y: f64 = 122.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (450.0, 300.0);
const BARCODE_LANDS: usize = 6;
const RFID_LANDS: usize = 4;
const TAMPER_LANDS: usize = 5;

const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 110.0;
const GATE_Z: f64 = 38.0;
const GATE_POS: (f64, f64) = (80.0, -350.0);
const DISPOSITION_GATES: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const RELEASE_GATE_INDEX: usize = 0;
const HOLD_GATE_INDEX: usize = 1;
const REJECT_GATE_INDEX: usize = 2;

const CAMERA_BRIDGE_X: f64 = 980.0;
const CAMERA_BRIDGE_Y: f64 = 44.0;
const CAMERA_BRIDGE_Z: f64 = 220.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -18.0);
const CAMERA_MOUNTS: usize = 5;
const EVIDENCE_FIDUCIALS: usize = 10;
const ROBOT_KEEPOUT_X: f64 = 1160.0;
const ROBOT_KEEPOUT_Y: f64 = 92.0;
const ROBOT_KEEPOUT_Z: f64 = 78.0;
const SERVICE_KEEPOUT_X: f64 = 102.0;
const SERVICE_KEEPOUT_Y: f64 = 700.0;
const SERVICE_KEEPOUT_Z: f64 = 96.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 305.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    write_part(containment_deck(), OUTPUTS[0]);
    write_part(cassette_surrogate_dock(), OUTPUTS[1]);
    write_part(inlet_outlet_slug_challenge_manifold(), OUTPUTS[2]);
    write_part(optical_bubble_witness_windows(), OUTPUTS[3]);
    write_part(staged_prime_token_rail(), OUTPUTS[4]);
    write_part(low_point_purge_pockets(), OUTPUTS[5]);
    write_part(pressure_flow_tap_bosses(), OUTPUTS[6]);
    write_part(waste_retain_split_capture(), OUTPUTS[7]);
    write_part(route_barcode_rfid_custody_lands(), OUTPUTS[8]);
    write_part(release_hold_reject_gates(), OUTPUTS[9]);
    write_part(camera_evidence_bridge_robot_service_keepouts(), OUTPUTS[10]);
    write_part(station_assembly(), OUTPUTS[11]);

    println!(
        "Closed cassette pre-run air-slug displacement station: {:.0}mm x {:.0}mm deck, {} slug lanes, {} bubble witness windows.",
        DECK_X, DECK_Y, SLUG_LANES, BUBBLE_WINDOWS
    );
    println!(
        "Validation packaging features: {} prime tokens, {} low-point purge pockets, {} pressure taps, {} flow taps.",
        PRIME_STAGE_TOKENS, LOW_POINT_PURGE_POCKETS, PRESSURE_TAPS, FLOW_TAPS
    );
    println!(
        "Custody and evidence: {} barcode lands, {} RFID lands, release/hold/reject gates, {} camera mounts, robot/service keepouts.",
        BARCODE_LANDS, RFID_LANDS, CAMERA_MOUNTS
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_cassette_air_slug_station_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let sump = centered_cube(
        "closed_cassette_air_slug_station_shallow_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "closed_cassette_air_slug_station_captured_drain",
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 80.0, -DECK_Y / 2.0 - 2.0, -1.0);

    deck - sump - drain + containment_rim() + deck_datums() + station_landing_pockets()
}

fn containment_rim() -> Part {
    let left = centered_cube(
        "closed_cassette_air_slug_station_left_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, rim_center_z());
    let right = centered_cube(
        "closed_cassette_air_slug_station_right_containment_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, rim_center_z());
    let rear = centered_cube(
        "closed_cassette_air_slug_station_rear_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, rim_center_z());
    let front = centered_cube(
        "closed_cassette_air_slug_station_front_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z * 0.65,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z * 0.325,
    );

    left + right + rear + front
}

fn deck_datums() -> Part {
    let mut datums = Part::empty("closed_cassette_air_slug_station_deck_datums");
    for i in 0..DATUM_BOSSES {
        let x = -DECK_X / 2.0 + 78.0 + i as f64 * ((DECK_X - 156.0) / 9.0);
        let y = if i % 2 == 0 {
            DECK_Y / 2.0 - 70.0
        } else {
            -DECK_Y / 2.0 + 70.0
        };
        let boss = centered_cylinder(
            format!("closed_cassette_air_slug_station_datum_boss_{i}"),
            13.0,
            6.0,
            36,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("closed_cassette_air_slug_station_datum_bore_{i}"),
            3.0,
            8.0,
            24,
        )
        .translate(x, y, DECK_Z / 2.0 + 3.0);
        datums = datums + (boss - bore);
    }
    datums
}

fn station_landing_pockets() -> Part {
    landing_pocket(
        "cassette_surrogate_dock_land",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_POS,
    ) + landing_pocket(
        "slug_challenge_manifold_land",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_POS,
    ) + landing_pocket(
        "optical_witness_window_land",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_POS,
    ) + landing_pocket(
        "staged_prime_token_rail_land",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_POS,
    ) + landing_pocket("low_point_purge_land", PURGE_X, PURGE_Y, PURGE_POS)
        + landing_pocket("pressure_flow_tap_land", TAP_BAR_X, TAP_BAR_Y, TAP_BAR_POS)
        + landing_pocket(
            "waste_retain_split_capture_land",
            CAPTURE_X,
            CAPTURE_Y,
            CAPTURE_POS,
        )
        + landing_pocket("route_custody_land", CUSTODY_X, CUSTODY_Y, CUSTODY_POS)
        + landing_pocket("disposition_gate_land", GATE_X, GATE_Y, GATE_POS)
}

fn landing_pocket(name: &str, x: f64, y: f64, pos: (f64, f64)) -> Part {
    centered_cube(
        format!("closed_cassette_air_slug_station_{name}"),
        x + 12.0,
        y + 12.0,
        3.0,
    )
    .translate(pos.0, pos.1, DECK_Z / 2.0 + 1.5)
}

fn cassette_surrogate_dock() -> Part {
    let base = centered_cube(
        "closed_cassette_air_slug_station_cassette_surrogate_dock_base",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );
    let recess = centered_cube(
        "closed_cassette_air_slug_station_cassette_surrogate_recess",
        CASSETTE_RECESS_X,
        CASSETTE_RECESS_Y,
        CASSETTE_RECESS_Z,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 - 5.0);
    let gasket_witness = centered_cube(
        "closed_cassette_air_slug_station_cassette_gasket_witness_lip",
        CASSETTE_RECESS_X + 24.0,
        CASSETTE_RECESS_Y + 24.0,
        7.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 3.5);

    base - recess - gripper_relief_slots()
        + gasket_witness
        + cassette_datum_rails()
        + cassette_datum_pins()
        + cassette_orientation_keys()
}

fn cassette_datum_rails() -> Part {
    let left = centered_cube(
        "closed_cassette_air_slug_station_cassette_left_datum_rail",
        14.0,
        CASSETTE_RECESS_Y,
        24.0,
    )
    .translate(
        -CASSETTE_RECESS_X / 2.0 - 13.0,
        0.0,
        CASSETTE_DOCK_Z / 2.0 + 12.0,
    );
    let rear = centered_cube(
        "closed_cassette_air_slug_station_cassette_rear_datum_rail",
        CASSETTE_RECESS_X,
        14.0,
        24.0,
    )
    .translate(
        0.0,
        CASSETTE_RECESS_Y / 2.0 + 13.0,
        CASSETTE_DOCK_Z / 2.0 + 12.0,
    );
    let front_stop = centered_cube(
        "closed_cassette_air_slug_station_cassette_front_sweep_stop",
        CASSETTE_RECESS_X - 60.0,
        10.0,
        18.0,
    )
    .translate(
        18.0,
        -CASSETTE_RECESS_Y / 2.0 - 11.0,
        CASSETTE_DOCK_Z / 2.0 + 9.0,
    );

    left + rear + front_stop
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("closed_cassette_air_slug_station_cassette_datum_pins");
    for i in 0..CASSETTE_DATUM_PINS {
        let col = i % 3;
        let row = i / 3;
        let x = -92.0 + col as f64 * 92.0;
        let y = -45.0 + row as f64 * 90.0;
        let ring = centered_cylinder(
            format!("closed_cassette_air_slug_station_cassette_datum_witness_ring_{i}"),
            12.0,
            3.0,
            36,
        )
        .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 1.5);
        let pin = centered_cylinder(
            format!("closed_cassette_air_slug_station_cassette_datum_pin_{i}"),
            5.5,
            12.0,
            32,
        )
        .translate(x, y, CASSETTE_DOCK_Z / 2.0 + 6.0);
        pins = pins + ring + pin;
    }
    pins
}

fn cassette_orientation_keys() -> Part {
    let mut keys = Part::empty("closed_cassette_air_slug_station_cassette_orientation_keys");
    for i in 0..CASSETTE_ORIENTATION_KEYS {
        let x = -120.0 + i as f64 * 80.0;
        let key = centered_cube(
            format!("closed_cassette_air_slug_station_cassette_orientation_key_{i}"),
            36.0,
            18.0 + i as f64 * 4.0,
            13.0,
        )
        .translate(
            x,
            -CASSETTE_DOCK_Y / 2.0 + 30.0,
            CASSETTE_DOCK_Z / 2.0 + 6.5,
        );
        keys = keys + key;
    }
    keys
}

fn gripper_relief_slots() -> Part {
    let mut slots = Part::empty("closed_cassette_air_slug_station_gripper_relief_slots");
    for i in 0..GRIPPER_RELIEFS {
        let x = if i % 2 == 0 { -1.0 } else { 1.0 } * (CASSETTE_DOCK_X / 2.0 - 52.0);
        let y = if i < 2 { -48.0 } else { 48.0 };
        slots = slots
            + centered_cube(
                format!("closed_cassette_air_slug_station_gripper_relief_slot_{i}"),
                48.0,
                28.0,
                CASSETTE_DOCK_Z + 4.0,
            )
            .translate(x, y, 3.0);
    }
    slots
}

fn inlet_outlet_slug_challenge_manifold() -> Part {
    let base = centered_cube(
        "closed_cassette_air_slug_station_inlet_outlet_slug_challenge_manifold_base",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let main_header = centered_cylinder(
        "closed_cassette_air_slug_station_slug_challenge_main_header",
        SLUG_LANE_D / 2.0 + 4.0,
        MANIFOLD_X - 60.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, MANIFOLD_Z / 2.0 + 8.0);
    let inlet_legend_land = centered_cube(
        "closed_cassette_air_slug_station_inlet_challenge_route_land",
        170.0,
        18.0,
        5.0,
    )
    .translate(-120.0, MANIFOLD_Y / 2.0 - 22.0, MANIFOLD_Z / 2.0 + 4.0);
    let outlet_legend_land = centered_cube(
        "closed_cassette_air_slug_station_outlet_challenge_route_land",
        170.0,
        18.0,
        5.0,
    )
    .translate(120.0, -MANIFOLD_Y / 2.0 + 22.0, MANIFOLD_Z / 2.0 + 4.0);

    base + main_header + inlet_legend_land + outlet_legend_land - slug_lane_bores()
        + slug_lane_witness_ribs()
        + inlet_outlet_challenge_ports()
}

fn slug_lane_bores() -> Part {
    let mut bores = Part::empty("closed_cassette_air_slug_station_slug_lane_bores");
    for i in 0..SLUG_LANES {
        let x = port_x(i, SLUG_LANES, SLUG_LANE_PITCH);
        let bore = centered_cylinder(
            format!("closed_cassette_air_slug_station_slug_displacement_lane_bore_{i}"),
            SLUG_LANE_D / 2.0,
            MANIFOLD_Y + 16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, MANIFOLD_Z / 2.0 + 8.0);
        bores = bores + bore;
    }
    bores
}

fn slug_lane_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_cassette_air_slug_station_slug_lane_witness_ribs");
    for i in 0..SLUG_LANES {
        let x = port_x(i, SLUG_LANES, SLUG_LANE_PITCH);
        let rib = centered_cube(
            format!("closed_cassette_air_slug_station_slug_displacement_lane_witness_rib_{i}"),
            18.0,
            MANIFOLD_Y - 34.0,
            7.0,
        )
        .translate(x, 0.0, MANIFOLD_Z / 2.0 + 4.0);
        ribs = ribs + rib;
    }
    ribs
}

fn inlet_outlet_challenge_ports() -> Part {
    let mut ports = Part::empty("closed_cassette_air_slug_station_inlet_outlet_ports");
    for i in 0..INLET_PORTS {
        let x = port_x(i, INLET_PORTS, 54.0) - 112.0;
        let port = challenge_port("inlet_slug_challenge", i, x, MANIFOLD_Y / 2.0 + 7.0);
        ports = ports + port.0 - port.1;
    }
    for i in 0..OUTLET_PORTS {
        let x = port_x(i, OUTLET_PORTS, 54.0) + 112.0;
        let port = challenge_port("outlet_slug_challenge", i, x, -MANIFOLD_Y / 2.0 - 7.0);
        ports = ports + port.0 - port.1;
    }
    ports
}

fn challenge_port(prefix: &str, index: usize, x: f64, y: f64) -> (Part, Part) {
    let boss = centered_cylinder(
        format!("closed_cassette_air_slug_station_{prefix}_port_boss_{index}"),
        CHALLENGE_PORT_D / 2.0,
        34.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, y, MANIFOLD_Z / 2.0 + 8.0);
    let bore = centered_cylinder(
        format!("closed_cassette_air_slug_station_{prefix}_port_bore_{index}"),
        4.6,
        44.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, y, MANIFOLD_Z / 2.0 + 8.0);
    (boss, bore)
}

fn optical_bubble_witness_windows() -> Part {
    let frame = centered_cube(
        "closed_cassette_air_slug_station_optical_bubble_witness_window_frame",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let backlight_slot = centered_cube(
        "closed_cassette_air_slug_station_witness_backlight_slot",
        WITNESS_X - 70.0,
        22.0,
        18.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0);

    frame - backlight_slot + bubble_window_lens_lands() + optical_fiducials()
}

fn bubble_window_lens_lands() -> Part {
    let mut windows = Part::empty("closed_cassette_air_slug_station_bubble_windows");
    for i in 0..BUBBLE_WINDOWS {
        let x = port_x(i, BUBBLE_WINDOWS, WINDOW_PITCH);
        let cut = centered_cylinder(
            format!("closed_cassette_air_slug_station_optical_bubble_witness_window_{i}"),
            WINDOW_D / 2.0,
            WITNESS_Z + 8.0,
            36,
        )
        .translate(x, 28.0, 6.0);
        let lens_land = centered_cylinder(
            format!("closed_cassette_air_slug_station_optical_bubble_lens_land_{i}"),
            WINDOW_D / 2.0 + 4.0,
            4.0,
            36,
        )
        .translate(x, 28.0, WITNESS_Z / 2.0 + 3.0);
        let route_tick = centered_cube(
            format!("closed_cassette_air_slug_station_window_route_tick_{i}"),
            28.0,
            6.0,
            5.0,
        )
        .translate(x, -WITNESS_Y / 2.0 + 18.0, WITNESS_Z / 2.0 + 3.0);
        windows = windows - cut + lens_land + route_tick;
    }
    windows
}

fn optical_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_cassette_air_slug_station_optical_fiducials");
    for i in 0..OPTICAL_FIDUCIALS {
        let x = -WITNESS_X / 2.0 + 32.0 + (i % 4) as f64 * ((WITNESS_X - 64.0) / 3.0);
        let y = if i < 4 {
            WITNESS_Y / 2.0 - 22.0
        } else {
            -WITNESS_Y / 2.0 + 22.0
        };
        let marker = centered_cylinder(
            format!("closed_cassette_air_slug_station_optical_evidence_fiducial_{i}"),
            5.0,
            4.0,
            24,
        )
        .translate(x, y, WITNESS_Z / 2.0 + 3.0);
        fiducials = fiducials + marker;
    }
    fiducials
}

fn staged_prime_token_rail() -> Part {
    let rail = centered_cube(
        "closed_cassette_air_slug_station_staged_prime_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let sight_slot = centered_cube(
        "closed_cassette_air_slug_station_prime_token_sight_slot",
        TOKEN_RAIL_X - 54.0,
        18.0,
        14.0,
    )
    .translate(0.0, -10.0, TOKEN_RAIL_Z / 2.0);

    rail - sight_slot + prime_stage_token_lands() + prime_step_markers()
}

fn prime_stage_token_lands() -> Part {
    let mut tokens = Part::empty("closed_cassette_air_slug_station_prime_stage_tokens");
    for i in 0..PRIME_STAGE_TOKENS {
        let x = port_x(i, PRIME_STAGE_TOKENS, PRIME_TOKEN_PITCH);
        let pocket = centered_cylinder(
            format!("closed_cassette_air_slug_station_prime_stage_token_pocket_{i}"),
            PRIME_TOKEN_D / 2.0,
            TOKEN_RAIL_Z + 8.0,
            36,
        )
        .translate(x, 22.0, 6.0);
        let token_land = centered_cylinder(
            format!("closed_cassette_air_slug_station_prime_stage_token_land_{i}"),
            PRIME_TOKEN_D / 2.0 - 3.0,
            6.0,
            36,
        )
        .translate(x, 22.0, TOKEN_RAIL_Z / 2.0 + 5.0);
        tokens = tokens - pocket + token_land;
    }
    tokens
}

fn prime_step_markers() -> Part {
    let mut markers = Part::empty("closed_cassette_air_slug_station_prime_step_markers");
    for i in 0..PRIME_STEP_MARKERS {
        let x = port_x(i, PRIME_STEP_MARKERS, PRIME_TOKEN_PITCH);
        let marker = centered_cube(
            format!("closed_cassette_air_slug_station_prime_stage_step_marker_{i}"),
            12.0,
            30.0,
            5.0 + i as f64 * 2.0,
        )
        .translate(
            x,
            -TOKEN_RAIL_Y / 2.0 + 20.0,
            TOKEN_RAIL_Z / 2.0 + 2.5 + i as f64,
        );
        markers = markers + marker;
    }
    markers
}

fn low_point_purge_pockets() -> Part {
    let base = centered_cube(
        "closed_cassette_air_slug_station_low_point_purge_pocket_base",
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    );
    let grade_plane = centered_cube(
        "closed_cassette_air_slug_station_purge_low_slope_plane",
        PURGE_X - 34.0,
        PURGE_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, PURGE_Z / 2.0 - 4.0);

    base - grade_plane - purge_route_troughs() - purge_pocket_cuts()
        + purge_pocket_lips()
        + purge_drain_bosses()
}

fn purge_route_troughs() -> Part {
    let mut troughs = Part::empty("closed_cassette_air_slug_station_purge_route_troughs");
    for i in 0..PURGE_TROUGHS {
        let y = -45.0 + i as f64 * 30.0;
        troughs = troughs
            + centered_cube(
                format!("closed_cassette_air_slug_station_low_point_purge_route_trough_{i}"),
                PURGE_X - 64.0,
                10.0,
                18.0,
            )
            .translate(0.0, y, PURGE_Z / 2.0 - 2.0);
    }
    troughs
}

fn purge_pocket_cuts() -> Part {
    let mut pockets = Part::empty("closed_cassette_air_slug_station_low_point_purge_pocket_cuts");
    for i in 0..LOW_POINT_PURGE_POCKETS {
        let x = port_x(i, LOW_POINT_PURGE_POCKETS, PURGE_POCKET_PITCH);
        pockets = pockets
            + centered_cylinder(
                format!("closed_cassette_air_slug_station_low_point_purge_pocket_{i}"),
                PURGE_POCKET_D / 2.0,
                PURGE_Z + 8.0,
                36,
            )
            .translate(x, 34.0, 8.0);
    }
    pockets
}

fn purge_pocket_lips() -> Part {
    let mut lips = Part::empty("closed_cassette_air_slug_station_purge_pocket_lips");
    for i in 0..LOW_POINT_PURGE_POCKETS {
        let x = port_x(i, LOW_POINT_PURGE_POCKETS, PURGE_POCKET_PITCH);
        lips = lips
            + centered_cylinder(
                format!("closed_cassette_air_slug_station_low_point_purge_pocket_lip_{i}"),
                PURGE_POCKET_D / 2.0 + 4.0,
                4.0,
                36,
            )
            .translate(x, 34.0, PURGE_Z / 2.0 + 3.0);
    }
    lips
}

fn purge_drain_bosses() -> Part {
    let mut bosses = Part::empty("closed_cassette_air_slug_station_purge_drain_bosses");
    for i in 0..2 {
        let x = if i == 0 {
            -PURGE_X / 2.0 + 48.0
        } else {
            PURGE_X / 2.0 - 48.0
        };
        let boss = centered_cylinder(
            format!("closed_cassette_air_slug_station_purge_drain_boss_{i}"),
            12.0,
            24.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -PURGE_Y / 2.0 - 4.0, 4.0);
        bosses = bosses + boss;
    }
    bosses
}

fn pressure_flow_tap_bosses() -> Part {
    let bar = centered_cube(
        "closed_cassette_air_slug_station_pressure_flow_tap_bar",
        TAP_BAR_X,
        TAP_BAR_Y,
        TAP_BAR_Z,
    );
    let route_channel = centered_cylinder(
        "closed_cassette_air_slug_station_pressure_flow_route_channel",
        6.0,
        TAP_BAR_X - 42.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, TAP_BAR_Z / 2.0 + 5.0);

    bar + route_channel + pressure_tap_bosses() + flow_tap_bosses()
}

fn pressure_tap_bosses() -> Part {
    let mut taps = Part::empty("closed_cassette_air_slug_station_pressure_tap_bosses");
    for i in 0..PRESSURE_TAPS {
        let x = port_x(i, PRESSURE_TAPS, TAP_PITCH);
        let tap = tap_boss("pressure", i, x, 25.0);
        taps = taps + tap.0 - tap.1;
    }
    taps
}

fn flow_tap_bosses() -> Part {
    let mut taps = Part::empty("closed_cassette_air_slug_station_flow_tap_bosses");
    for i in 0..FLOW_TAPS {
        let x = port_x(i, FLOW_TAPS, TAP_PITCH + 12.0);
        let tap = tap_boss("flow", i, x, -28.0);
        let paddle = centered_cube(
            format!("closed_cassette_air_slug_station_flow_tap_paddle_witness_{i}"),
            34.0,
            14.0,
            6.0,
        )
        .translate(x, -TAP_BAR_Y / 2.0 + 14.0, TAP_BAR_Z / 2.0 + 5.0);
        taps = taps + tap.0 - tap.1 + paddle;
    }
    taps
}

fn tap_boss(prefix: &str, index: usize, x: f64, y: f64) -> (Part, Part) {
    let boss = centered_cylinder(
        format!("closed_cassette_air_slug_station_{prefix}_tap_boss_{index}"),
        TAP_BOSS_D / 2.0,
        18.0,
        36,
    )
    .translate(x, y, TAP_BAR_Z / 2.0 + 9.0);
    let bore = centered_cylinder(
        format!("closed_cassette_air_slug_station_{prefix}_tap_bore_{index}"),
        TAP_BORE_D / 2.0,
        26.0,
        24,
    )
    .translate(x, y, TAP_BAR_Z / 2.0 + 9.0);
    (boss, bore)
}

fn waste_retain_split_capture() -> Part {
    let tray = centered_cube(
        "closed_cassette_air_slug_station_waste_retain_split_capture_tray",
        CAPTURE_X,
        CAPTURE_Y,
        CAPTURE_Z,
    );
    let basin = centered_cube(
        "closed_cassette_air_slug_station_split_capture_basin",
        CAPTURE_X - 34.0,
        CAPTURE_Y - 34.0,
        28.0,
    )
    .translate(0.0, 0.0, CAPTURE_Z / 2.0 - 4.0);
    let diverter = centered_cube(
        "closed_cassette_air_slug_station_waste_retain_diverter_weir",
        11.0,
        CAPTURE_Y - 28.0,
        30.0,
    )
    .translate(0.0, 0.0, CAPTURE_Z / 2.0 + 10.0);

    tray - basin - waste_capture_cells() - retain_capture_wells()
        + diverter
        + capture_label_lands()
        + retain_well_lips()
}

fn waste_capture_cells() -> Part {
    let mut cells = Part::empty("closed_cassette_air_slug_station_waste_split_capture_cells");
    for i in 0..WASTE_CAPTURE_CELLS {
        let x = port_x(i, WASTE_CAPTURE_CELLS, CAPTURE_CELL_X);
        cells = cells
            + centered_cube(
                format!("closed_cassette_air_slug_station_waste_split_capture_cell_{i}"),
                CAPTURE_CELL_X - 14.0,
                CAPTURE_CELL_Y,
                CAPTURE_Z + 8.0,
            )
            .translate(x - 72.0, 44.0, 8.0);
    }
    cells
}

fn retain_capture_wells() -> Part {
    let mut wells = Part::empty("closed_cassette_air_slug_station_retain_split_capture_wells");
    for i in 0..RETAIN_CAPTURE_WELLS {
        let x = port_x(i, RETAIN_CAPTURE_WELLS, 42.0) + 76.0;
        wells = wells
            + centered_cylinder(
                format!("closed_cassette_air_slug_station_retain_split_capture_well_{i}"),
                RETAIN_WELL_D / 2.0,
                CAPTURE_Z + 8.0,
                32,
            )
            .translate(x, -48.0, 8.0);
    }
    wells
}

fn retain_well_lips() -> Part {
    let mut lips = Part::empty("closed_cassette_air_slug_station_retain_well_lips");
    for i in 0..RETAIN_CAPTURE_WELLS {
        let x = port_x(i, RETAIN_CAPTURE_WELLS, 42.0) + 76.0;
        lips = lips
            + centered_cylinder(
                format!("closed_cassette_air_slug_station_retain_split_capture_well_lip_{i}"),
                RETAIN_WELL_D / 2.0 + 4.0,
                4.0,
                32,
            )
            .translate(x, -48.0, CAPTURE_Z / 2.0 + 3.0);
    }
    lips
}

fn capture_label_lands() -> Part {
    let waste_land = centered_cube(
        "closed_cassette_air_slug_station_waste_capture_label_land",
        150.0,
        16.0,
        4.0,
    )
    .translate(-95.0, CAPTURE_Y / 2.0 - 23.0, CAPTURE_Z / 2.0 + 4.0);
    let retain_land = centered_cube(
        "closed_cassette_air_slug_station_retain_capture_label_land",
        150.0,
        16.0,
        4.0,
    )
    .translate(95.0, -CAPTURE_Y / 2.0 + 23.0, CAPTURE_Z / 2.0 + 4.0);
    waste_land + retain_land
}

fn route_barcode_rfid_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_cassette_air_slug_station_route_barcode_rfid_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    panel + route_barcode_lands() + rfid_custody_lands() + custody_tamper_lands()
}

fn route_barcode_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_air_slug_station_route_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = -112.0 + (i % 3) as f64 * 112.0;
        let y = 30.0 - (i / 3) as f64 * 34.0;
        let land = centered_cube(
            format!("closed_cassette_air_slug_station_route_barcode_land_{i}"),
            72.0,
            22.0,
            4.0,
        )
        .translate(x, y, CUSTODY_Z / 2.0 + 4.0);
        lands = lands + land;
    }
    lands
}

fn rfid_custody_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_air_slug_station_rfid_custody_lands");
    for i in 0..RFID_LANDS {
        let x = port_x(i, RFID_LANDS, 70.0);
        let antenna = centered_cube(
            format!("closed_cassette_air_slug_station_rfid_custody_antenna_land_{i}"),
            52.0,
            26.0,
            4.0,
        )
        .translate(x, -42.0, CUSTODY_Z / 2.0 + 4.0);
        let center_witness = centered_cylinder(
            format!("closed_cassette_air_slug_station_rfid_custody_center_witness_{i}"),
            5.0,
            5.0,
            24,
        )
        .translate(x, -42.0, CUSTODY_Z / 2.0 + 4.0);
        lands = lands + (antenna - center_witness);
    }
    lands
}

fn custody_tamper_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_air_slug_station_custody_tamper_lands");
    for i in 0..TAMPER_LANDS {
        lands = lands
            + centered_cylinder(
                format!("closed_cassette_air_slug_station_custody_tamper_land_{i}"),
                7.0,
                4.0,
                24,
            )
            .translate(
                -CUSTODY_X / 2.0 + 25.0 + i as f64 * 28.0,
                -CUSTODY_Y / 2.0 + 18.0,
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "closed_cassette_air_slug_station_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );

    base + disposition_gate_sliders() - disposition_token_slots()
}

fn disposition_gate_sliders() -> Part {
    let mut sliders = Part::empty("closed_cassette_air_slug_station_disposition_gate_sliders");
    for i in 0..DISPOSITION_GATES {
        let name = disposition_gate_name(i);
        let x = port_x(i, DISPOSITION_GATES, 128.0);
        let slider = centered_cube(
            format!("closed_cassette_air_slug_station_{name}_gate_slider"),
            84.0,
            54.0,
            12.0,
        )
        .translate(x, 7.0, GATE_Z / 2.0 + 8.0);
        let flag = centered_cube(
            format!("closed_cassette_air_slug_station_{name}_gate_flag_land"),
            58.0,
            12.0,
            28.0,
        )
        .translate(x, -GATE_Y / 2.0 + 16.0, GATE_Z / 2.0 + 16.0);
        sliders = sliders + slider + flag;
    }
    sliders
}

fn disposition_token_slots() -> Part {
    let mut slots = Part::empty("closed_cassette_air_slug_station_disposition_token_slots");
    for i in 0..GATE_TOKEN_SLOTS {
        let slot = centered_cube(
            format!("closed_cassette_air_slug_station_disposition_token_slot_{i}"),
            34.0,
            16.0,
            GATE_Z + 8.0,
        )
        .translate(port_x(i, GATE_TOKEN_SLOTS, 55.0), GATE_Y / 2.0 - 24.0, 6.0);
        slots = slots + slot;
    }
    slots
}

fn camera_evidence_bridge_robot_service_keepouts() -> Part {
    camera_bridge() + evidence_fiducials() + robot_keepout_gauges() + service_keepout_gauges()
}

fn camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_cassette_air_slug_station_camera_bridge_left_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "closed_cassette_air_slug_station_camera_bridge_right_post",
        30.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let crossbar = centered_cube(
        "closed_cassette_air_slug_station_camera_evidence_bridge_crossbar",
        CAMERA_BRIDGE_X + 60.0,
        CAMERA_BRIDGE_Y,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);

    left_post + right_post + crossbar + camera_mount_lands()
}

fn camera_mount_lands() -> Part {
    let mut lands = Part::empty("closed_cassette_air_slug_station_camera_mount_lands");
    for i in 0..CAMERA_MOUNTS {
        let x = port_x(i, CAMERA_MOUNTS, 180.0);
        let land = centered_cube(
            format!("closed_cassette_air_slug_station_camera_mount_land_{i}"),
            76.0,
            12.0,
            8.0,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 8.0, CAMERA_BRIDGE_Z - 14.0);
        let bore = centered_cylinder(
            format!("closed_cassette_air_slug_station_camera_mount_bore_{i}"),
            4.0,
            10.0,
            20,
        )
        .translate(x, -CAMERA_BRIDGE_Y / 2.0 - 8.0, CAMERA_BRIDGE_Z - 14.0);
        lands = lands + (land - bore);
    }
    lands
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_cassette_air_slug_station_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIALS {
        let x = -CAMERA_BRIDGE_X / 2.0 + 70.0 + i as f64 * ((CAMERA_BRIDGE_X - 140.0) / 9.0);
        let fid = centered_cylinder(
            format!("closed_cassette_air_slug_station_bridge_evidence_fiducial_{i}"),
            6.0,
            5.0,
            24,
        )
        .translate(x, CAMERA_BRIDGE_Y / 2.0 + 10.0, CAMERA_BRIDGE_Z - 16.0);
        fiducials = fiducials + fid;
    }
    fiducials
}

fn robot_keepout_gauges() -> Part {
    let front = centered_cube(
        "closed_cassette_air_slug_station_robot_front_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 85.0, ROBOT_KEEPOUT_Z / 2.0);
    let rear = centered_cube(
        "closed_cassette_air_slug_station_robot_rear_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y * 0.55,
        ROBOT_KEEPOUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 84.0, ROBOT_KEEPOUT_Z / 2.0);
    front + rear
}

fn service_keepout_gauges() -> Part {
    let left = centered_cube(
        "closed_cassette_air_slug_station_left_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(-DECK_X / 2.0 + 70.0, 0.0, SERVICE_KEEPOUT_Z / 2.0);
    let right = centered_cube(
        "closed_cassette_air_slug_station_right_service_keepout_gauge",
        SERVICE_KEEPOUT_X,
        SERVICE_KEEPOUT_Y,
        SERVICE_KEEPOUT_Z,
    )
    .translate(DECK_X / 2.0 - 70.0, 0.0, SERVICE_KEEPOUT_Z / 2.0);
    let top = centered_cube(
        "closed_cassette_air_slug_station_top_service_clearance_gauge",
        690.0,
        420.0,
        8.0,
    )
    .translate(0.0, 0.0, TOP_SERVICE_CLEARANCE_Z);
    left + right + top
}

fn station_assembly() -> Part {
    containment_deck()
        + cassette_surrogate_dock().translate(
            CASSETTE_DOCK_POS.0,
            CASSETTE_DOCK_POS.1,
            top_z(CASSETTE_DOCK_Z),
        )
        + inlet_outlet_slug_challenge_manifold().translate(
            MANIFOLD_POS.0,
            MANIFOLD_POS.1,
            top_z(MANIFOLD_Z),
        )
        + optical_bubble_witness_windows().translate(WITNESS_POS.0, WITNESS_POS.1, top_z(WITNESS_Z))
        + staged_prime_token_rail().translate(
            TOKEN_RAIL_POS.0,
            TOKEN_RAIL_POS.1,
            top_z(TOKEN_RAIL_Z),
        )
        + low_point_purge_pockets().translate(PURGE_POS.0, PURGE_POS.1, top_z(PURGE_Z))
        + pressure_flow_tap_bosses().translate(TAP_BAR_POS.0, TAP_BAR_POS.1, top_z(TAP_BAR_Z))
        + waste_retain_split_capture().translate(CAPTURE_POS.0, CAPTURE_POS.1, top_z(CAPTURE_Z))
        + route_barcode_rfid_custody_lands().translate(
            CUSTODY_POS.0,
            CUSTODY_POS.1,
            top_z(CUSTODY_Z),
        )
        + release_hold_reject_gates().translate(GATE_POS.0, GATE_POS.1, top_z(GATE_Z))
        + camera_evidence_bridge_robot_service_keepouts().translate(
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            DECK_Z / 2.0,
        )
}

fn top_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn rim_center_z() -> f64 {
    DECK_Z / 2.0 + RIM_Z / 2.0
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn token_span() -> f64 {
    (PRIME_STAGE_TOKENS as f64 - 1.0) * PRIME_TOKEN_PITCH + PRIME_TOKEN_D
}

fn slug_lane_span() -> f64 {
    (SLUG_LANES as f64 - 1.0) * SLUG_LANE_PITCH + SLUG_LANE_D
}

fn witness_window_span() -> f64 {
    (BUBBLE_WINDOWS as f64 - 1.0) * WINDOW_PITCH + WINDOW_D
}

fn purge_pocket_span() -> f64 {
    (LOW_POINT_PURGE_POCKETS as f64 - 1.0) * PURGE_POCKET_PITCH + PURGE_POCKET_D
}

fn tap_span(count: usize) -> f64 {
    (count as f64 - 1.0) * TAP_PITCH + TAP_BOSS_D
}

fn inside_deck(pos: (f64, f64), x: f64, y: f64) -> bool {
    pos.0 - x / 2.0 > -DECK_X / 2.0 + RIM_W
        && pos.0 + x / 2.0 < DECK_X / 2.0 - RIM_W
        && pos.1 - y / 2.0 > -DECK_Y / 2.0 + RIM_W
        && pos.1 + y / 2.0 < DECK_Y / 2.0 - RIM_W
}

fn disposition_gate_name(index: usize) -> &'static str {
    match index {
        RELEASE_GATE_INDEX => "release",
        HOLD_GATE_INDEX => "hold",
        REJECT_GATE_INDEX => "reject",
        _ => panic!("unknown disposition gate index"),
    }
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(DISPOSITION_GATES, 3);
    assert_eq!(RELEASE_GATE_INDEX + HOLD_GATE_INDEX + REJECT_GATE_INDEX, 3);
    assert!(inside_deck(
        CASSETTE_DOCK_POS,
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y
    ));
    assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
    assert!(inside_deck(WITNESS_POS, WITNESS_X, WITNESS_Y));
    assert!(inside_deck(TOKEN_RAIL_POS, TOKEN_RAIL_X, TOKEN_RAIL_Y));
    assert!(inside_deck(PURGE_POS, PURGE_X, PURGE_Y));
    assert!(inside_deck(TAP_BAR_POS, TAP_BAR_X, TAP_BAR_Y));
    assert!(inside_deck(CAPTURE_POS, CAPTURE_X, CAPTURE_Y));
    assert!(inside_deck(CUSTODY_POS, CUSTODY_X, CUSTODY_Y));
    assert!(inside_deck(GATE_POS, GATE_X, GATE_Y));
    assert!(slug_lane_span() < MANIFOLD_X - 70.0);
    assert!(witness_window_span() < WITNESS_X - 80.0);
    assert!(token_span() < TOKEN_RAIL_X - 60.0);
    assert!(purge_pocket_span() < PURGE_X - 80.0);
    assert!(tap_span(PRESSURE_TAPS) < TAP_BAR_X - 120.0);
    assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z + 60.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS.iter().any(|path| path.ends_with("_assembly.stl")));
    }

    #[test]
    fn required_mechanical_validation_features_are_named() {
        assert_eq!(REQUIRED_FEATURES.len(), 26);
        assert!(REQUIRED_FEATURES.contains(&"cassette_surrogate_dock"));
        assert!(REQUIRED_FEATURES.contains(&"inlet_slug_challenge_ports"));
        assert!(REQUIRED_FEATURES.contains(&"outlet_slug_challenge_ports"));
        assert!(REQUIRED_FEATURES.contains(&"slug_displacement_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"optical_bubble_witness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"staged_prime_token_rail"));
        assert!(REQUIRED_FEATURES.contains(&"low_point_purge_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_tap_bosses"));
        assert!(REQUIRED_FEATURES.contains(&"flow_tap_bosses"));
        assert!(REQUIRED_FEATURES.contains(&"waste_split_capture_cells"));
        assert!(REQUIRED_FEATURES.contains(&"retain_split_capture_wells"));
        assert!(REQUIRED_FEATURES.contains(&"route_barcode_lands"));
        assert!(REQUIRED_FEATURES.contains(&"rfid_custody_lands"));
        assert!(REQUIRED_FEATURES.contains(&"release_gate"));
        assert!(REQUIRED_FEATURES.contains(&"hold_gate"));
        assert!(REQUIRED_FEATURES.contains(&"reject_gate"));
        assert!(REQUIRED_FEATURES.contains(&"camera_evidence_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn scope_excludes_process_device_and_acceptance_claims() {
        assert_eq!(OUT_OF_SCOPE_CLAIMS.len(), 5);
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"sterile_process_claim"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"biological_standard_operating_procedure"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"pressure_rated_device"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"process_acceptance_criterion"));
        assert!(OUT_OF_SCOPE_CLAIMS.contains(&"product_release_criterion"));
    }

    #[test]
    fn counts_match_air_slug_displacement_fixture_intent() {
        assert_eq!(SLUG_LANES, INLET_PORTS + OUTLET_PORTS);
        assert!(BUBBLE_WINDOWS >= SLUG_LANES);
        assert_eq!(PRIME_STAGE_TOKENS, 6);
        assert!(LOW_POINT_PURGE_POCKETS >= 5);
        assert!(PRESSURE_TAPS > FLOW_TAPS);
        assert_eq!(WASTE_CAPTURE_CELLS + 2, RETAIN_CAPTURE_WELLS);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(BARCODE_LANDS, SLUG_LANES);
    }

    #[test]
    fn layout_fits_contained_robotic_station() {
        assert_layout();
        assert!(inside_deck(
            CASSETTE_DOCK_POS,
            CASSETTE_DOCK_X,
            CASSETTE_DOCK_Y
        ));
        assert!(inside_deck(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y));
        assert!(inside_deck(CAPTURE_POS, CAPTURE_X, CAPTURE_Y));
        assert!(inside_deck(GATE_POS, GATE_X, GATE_Y));
        assert!(ROBOT_KEEPOUT_X < DECK_X);
        assert!(SERVICE_KEEPOUT_Y < DECK_Y);
        assert!(TOP_SERVICE_CLEARANCE_Z > CAMERA_BRIDGE_Z);
    }

    #[test]
    fn geometry_spans_remain_printable_and_nonoverrunning() {
        assert!(slug_lane_span() < MANIFOLD_X - 70.0);
        assert!(witness_window_span() < WITNESS_X - 80.0);
        assert!(token_span() < TOKEN_RAIL_X - 60.0);
        assert!(purge_pocket_span() < PURGE_X - 80.0);
        assert!(tap_span(PRESSURE_TAPS) < TAP_BAR_X - 120.0);
        assert_eq!(disposition_gate_name(RELEASE_GATE_INDEX), "release");
        assert_eq!(disposition_gate_name(HOLD_GATE_INDEX), "hold");
        assert_eq!(disposition_gate_name(REJECT_GATE_INDEX), "reject");
    }
}
