use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sampling carryover/flush validation station.
//
// Intent:
// - Validate sample-loop and analyzer-interface carryover before live culture
//   or analyte claims are made.
// - Keep high/low concentration standards alternating and physically separate
//   from released, hold, and reject custody lanes.
// - Provide closed cartridge nests, flush/waste routing, analyzer docking,
//   traceability lands, timed dwell/flush tokens, leak containment, camera
//   evidence capture, and robot/service keepout envelopes.
//
// This is mechanical architecture CAD for validation workflow planning. It is
// not an analytical method, acceptance criterion, or claim of carryover
// performance.

const OUTPUTS: &[&str] = &[
    "output/closed_sampling_carryover_flush_validation_station_base_leak_tray.stl",
    "output/closed_sampling_carryover_flush_validation_station_closed_sample_loop_cartridge_nests.stl",
    "output/closed_sampling_carryover_flush_validation_station_high_low_standard_rack.stl",
    "output/closed_sampling_carryover_flush_validation_station_flush_waste_routing_manifold.stl",
    "output/closed_sampling_carryover_flush_validation_station_analyzer_dock_placeholder.stl",
    "output/closed_sampling_carryover_flush_validation_station_barcode_certificate_lands.stl",
    "output/closed_sampling_carryover_flush_validation_station_timed_dwell_flush_token_bank.stl",
    "output/closed_sampling_carryover_flush_validation_station_released_lane_caddy.stl",
    "output/closed_sampling_carryover_flush_validation_station_hold_lane_caddy.stl",
    "output/closed_sampling_carryover_flush_validation_station_reject_lane_caddy.stl",
    "output/closed_sampling_carryover_flush_validation_station_evidence_camera_bridge.stl",
    "output/closed_sampling_carryover_flush_validation_station_robot_service_keepouts.stl",
    "output/closed_sampling_carryover_flush_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "closed_sample_loop_cartridge_nests",
    "alternating_high_low_concentration_standard_positions",
    "flush_waste_routing",
    "analyzer_dock_placeholder",
    "barcode_certificate_lands",
    "timed_dwell_flush_tokens",
    "released_hold_reject_lanes",
    "leak_tray",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "status_custody_separation",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_BASIN_X: f64 = STATION_X - 112.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 104.0;
const LEAK_SENSOR_WELLS: usize = 5;

const LOOP_NEST_X: f64 = 540.0;
const LOOP_NEST_Y: f64 = 160.0;
const LOOP_NEST_Z: f64 = 54.0;
const SAMPLE_LOOP_CARTRIDGES: usize = 8;
const LOOP_PITCH_X: f64 = 66.0;
const LOOP_POCKET_X: f64 = 44.0;
const LOOP_POCKET_Y: f64 = 82.0;
const LOOP_POCKET_Z: f64 = 34.0;
const LOOP_TUBE_BORE_D: f64 = 5.8;
const LOOP_SEAL_LATCHES: usize = 8;

const STANDARD_RACK_X: f64 = 520.0;
const STANDARD_RACK_Y: f64 = 138.0;
const STANDARD_RACK_Z: f64 = 48.0;
const STANDARD_POSITIONS: usize = 12;
const STANDARD_PITCH_X: f64 = 40.0;
const HIGH_STANDARD_D: f64 = 19.0;
const LOW_STANDARD_D: f64 = 16.0;

const ROUTING_X: f64 = 880.0;
const ROUTING_Y: f64 = 92.0;
const ROUTING_Z: f64 = 56.0;
const FLUSH_PORTS: usize = SAMPLE_LOOP_CARTRIDGES;
const WASTE_PORTS: usize = SAMPLE_LOOP_CARTRIDGES;
const ROUTING_PORT_PITCH_X: f64 = 82.0;
const FLUSH_BORE_D: f64 = 5.5;
const WASTE_BORE_D: f64 = 8.4;
const WASTE_CHANNEL_MIN_GAP: f64 = 34.0;

const ANALYZER_DOCK_X: f64 = 260.0;
const ANALYZER_DOCK_Y: f64 = 200.0;
const ANALYZER_DOCK_Z: f64 = 132.0;
const ANALYZER_ENVELOPE_X: f64 = 210.0;
const ANALYZER_ENVELOPE_Y: f64 = 146.0;
const ANALYZER_ENVELOPE_Z: f64 = 96.0;
const ANALYZER_INTERFACE_PORTS: usize = 4;

const TRACE_LAND_X: f64 = 400.0;
const TRACE_LAND_Y: f64 = 120.0;
const TRACE_LAND_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 4;

const TOKEN_BANK_X: f64 = 380.0;
const TOKEN_BANK_Y: f64 = 112.0;
const TOKEN_BANK_Z: f64 = 20.0;
const DWELL_TOKENS: usize = 6;
const FLUSH_TOKENS: usize = 6;
const TOKEN_PITCH_X: f64 = 48.0;

const STATUS_LANE_X: f64 = 230.0;
const STATUS_LANE_Y: f64 = 124.0;
const STATUS_LANE_Z: f64 = 42.0;
const STATUS_SLOTS: usize = 4;
const STATUS_SLOT_PITCH_X: f64 = 46.0;
const STATUS_LANE_GAP_MIN: f64 = 80.0;
const STATUS_CUSTODY_GAP_MIN: f64 = 16.0;

const CAMERA_BRIDGE_X: f64 = 1020.0;
const CAMERA_BRIDGE_Y: f64 = 72.0;
const CAMERA_BRIDGE_Z: f64 = 210.0;
const CAMERA_COUNT: usize = 3;
const LIGHT_BAR_COUNT: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 172.0;

const ROBOT_KEEP_OUT_Z: f64 = 210.0;
const ROBOT_KEEP_OUT_ZONES: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 240.0;
const ANALYZER_SERVICE_CLEARANCE: f64 = 190.0;

const LOOP_NEST_POS: (f64, f64) = (-280.0, 100.0);
const STANDARD_RACK_POS: (f64, f64) = (300.0, 130.0);
const ROUTING_POS: (f64, f64) = (0.0, -40.0);
const ANALYZER_POS: (f64, f64) = (420.0, -220.0);
const TRACE_POS: (f64, f64) = (-350.0, -240.0);
const TOKEN_POS: (f64, f64) = (80.0, -250.0);
const RELEASED_POS: (f64, f64) = (-345.0, 280.0);
const HOLD_POS: (f64, f64) = (0.0, 280.0);
const REJECT_POS: (f64, f64) = (345.0, 280.0);
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, 30.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let loops = closed_sample_loop_cartridge_nests();
    export(&loops, OUTPUTS[1]);

    let standards = high_low_standard_rack();
    export(&standards, OUTPUTS[2]);

    let routing = flush_waste_routing_manifold();
    export(&routing, OUTPUTS[3]);

    let analyzer = analyzer_dock_placeholder();
    export(&analyzer, OUTPUTS[4]);

    let trace = barcode_certificate_lands();
    export(&trace, OUTPUTS[5]);

    let tokens = timed_dwell_flush_token_bank();
    export(&tokens, OUTPUTS[6]);

    let released = status_lane_caddy("released", 46.0);
    export(&released, OUTPUTS[7]);

    let hold = status_lane_caddy("hold", 58.0);
    export(&hold, OUTPUTS[8]);

    let reject = status_lane_caddy("reject", 78.0);
    export(&reject, OUTPUTS[9]);

    let camera_bridge = evidence_camera_bridge();
    export(&camera_bridge, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = base
        + loops.translate(LOOP_NEST_POS.0, LOOP_NEST_POS.1, insert_z(LOOP_NEST_Z))
        + standards.translate(
            STANDARD_RACK_POS.0,
            STANDARD_RACK_POS.1,
            insert_z(STANDARD_RACK_Z),
        )
        + routing.translate(ROUTING_POS.0, ROUTING_POS.1, insert_z(ROUTING_Z))
        + analyzer.translate(ANALYZER_POS.0, ANALYZER_POS.1, insert_z(ANALYZER_DOCK_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_LAND_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_BANK_Z))
        + released.translate(RELEASED_POS.0, RELEASED_POS.1, insert_z(STATUS_LANE_Z))
        + hold.translate(HOLD_POS.0, HOLD_POS.1, insert_z(STATUS_LANE_Z))
        + reject.translate(REJECT_POS.0, REJECT_POS.1, insert_z(STATUS_LANE_Z))
        + camera_bridge.translate(
            CAMERA_BRIDGE_POS.0,
            CAMERA_BRIDGE_POS.1,
            insert_z(CAMERA_BRIDGE_Z),
        )
        + closed_tube_route_placeholders()
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed sampling carryover/flush validation station: {:.0}mm x {:.0}mm leak-tray deck, {} closed sample-loop cartridge nests, {} alternating high/low standard positions, {} flush ports, {} waste ports, {} analyzer interface ports, {} barcode/certificate lands, {} timed dwell/flush tokens, released/hold/reject lanes, {} leak sensor wells, {} evidence cameras, and {} robot/service keepout envelopes.",
        STATION_X,
        STATION_Y,
        SAMPLE_LOOP_CARTRIDGES,
        STANDARD_POSITIONS,
        FLUSH_PORTS,
        WASTE_PORTS,
        ANALYZER_INTERFACE_PORTS,
        BARCODE_LANDS + CERTIFICATE_LANDS,
        DWELL_TOKENS + FLUSH_TOKENS,
        LEAK_SENSOR_WELLS,
        CAMERA_COUNT,
        ROBOT_KEEP_OUT_ZONES
    );
    println!(
        "Validation workflow controls: standard rack alternates high/low, wet routing is isolated from custody lanes by {:.0}mm, status lanes have {:.0}mm minimum gap, and camera clearance is {:.0}mm above the deck.",
        wet_to_status_gap(),
        status_lane_gap(),
        CAMERA_CLEARANCE_Z
    );
    println!(
        "Required feature groups covered: {}",
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    for (name, center, width, depth) in module_specs() {
        assert!(
            fits_on_station(center, width, depth, 24.0),
            "{name} exceeds station envelope"
        );
    }

    let loop_nests = rect(LOOP_NEST_POS, LOOP_NEST_X, LOOP_NEST_Y);
    let standards = rect(STANDARD_RACK_POS, STANDARD_RACK_X, STANDARD_RACK_Y);
    let routing = rect(ROUTING_POS, ROUTING_X, ROUTING_Y);
    let analyzer = rect(ANALYZER_POS, ANALYZER_DOCK_X, ANALYZER_DOCK_Y);
    let trace = rect(TRACE_POS, TRACE_LAND_X, TRACE_LAND_Y);
    let tokens = rect(TOKEN_POS, TOKEN_BANK_X, TOKEN_BANK_Y);

    assert!(!rects_overlap(loop_nests, standards));
    assert!(!rects_overlap(loop_nests, routing));
    assert!(!rects_overlap(standards, routing));
    assert!(!rects_overlap(routing, analyzer));
    assert!(!rects_overlap(trace, tokens));
    assert!(!rects_overlap(tokens, analyzer));
    assert!(wet_to_status_gap() >= STATUS_CUSTODY_GAP_MIN);
    assert!(status_lane_gap() >= STATUS_LANE_GAP_MIN);
    assert!((24.0 - -24.0) - (WASTE_BORE_D + FLUSH_BORE_D) / 2.0 >= WASTE_CHANNEL_MIN_GAP);
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 10] {
    [
        (
            "closed_sample_loop_cartridge_nests",
            LOOP_NEST_POS,
            LOOP_NEST_X,
            LOOP_NEST_Y,
        ),
        (
            "high_low_standard_rack",
            STANDARD_RACK_POS,
            STANDARD_RACK_X,
            STANDARD_RACK_Y,
        ),
        (
            "flush_waste_routing_manifold",
            ROUTING_POS,
            ROUTING_X,
            ROUTING_Y,
        ),
        (
            "analyzer_dock_placeholder",
            ANALYZER_POS,
            ANALYZER_DOCK_X,
            ANALYZER_DOCK_Y,
        ),
        (
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_LAND_X,
            TRACE_LAND_Y,
        ),
        (
            "timed_dwell_flush_token_bank",
            TOKEN_POS,
            TOKEN_BANK_X,
            TOKEN_BANK_Y,
        ),
        (
            "released_lane_caddy",
            RELEASED_POS,
            STATUS_LANE_X,
            STATUS_LANE_Y,
        ),
        ("hold_lane_caddy", HOLD_POS, STATUS_LANE_X, STATUS_LANE_Y),
        (
            "reject_lane_caddy",
            REJECT_POS,
            STATUS_LANE_X,
            STATUS_LANE_Y,
        ),
        (
            "evidence_camera_bridge",
            CAMERA_BRIDGE_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_sampling_carryover_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_sampling_carryover_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder(
        "closed_sampling_carryover_station_leak_tray_drain",
        14.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 78.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - basin - drain - insert_sockets() - mount_slots()
        + tray_rims()
        + custody_lane_dividers()
        + leak_sensor_wells()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_sampling_carryover_station_insert_sockets");
    for (name, center, width, depth) in module_specs().iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!("closed_sampling_carryover_station_{name}_socket"),
                width + 10.0,
                depth + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(center.0, center.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_sampling_carryover_station_mount_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 48.0), -(STATION_Y / 2.0 - 46.0)),
        (STATION_X / 2.0 - 48.0, -(STATION_Y / 2.0 - 46.0)),
        (-(STATION_X / 2.0 - 48.0), STATION_Y / 2.0 - 46.0),
        (STATION_X / 2.0 - 48.0, STATION_Y / 2.0 - 46.0),
        (0.0, -(STATION_Y / 2.0 - 46.0)),
        (0.0, STATION_Y / 2.0 - 46.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_sampling_carryover_station_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let relief = centered_cube(
            format!("closed_sampling_carryover_station_mount_slot_relief_{i}"),
            26.0,
            7.2,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + relief;
    }
    slots
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "closed_sampling_carryover_station_left_leak_tray_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_sampling_carryover_station_right_leak_tray_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_sampling_carryover_station_rear_leak_tray_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_sampling_carryover_station_front_low_leak_lip",
        STATION_X - 160.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z / 2.0 + 12.0);

    left + right + rear + front_low_lip
}

fn custody_lane_dividers() -> Part {
    let wet_to_custody = centered_cube(
        "closed_sampling_carryover_station_wet_to_custody_divider",
        STATION_X - 144.0,
        10.0,
        30.0,
    )
    .translate(0.0, 214.0, BASE_Z / 2.0 + 15.0);
    let left_lane_split = centered_cube(
        "closed_sampling_carryover_station_released_hold_lane_split",
        10.0,
        150.0,
        38.0,
    )
    .translate(-172.5, 280.0, BASE_Z / 2.0 + 19.0);
    let right_lane_split = centered_cube(
        "closed_sampling_carryover_station_hold_reject_lane_split",
        10.0,
        150.0,
        38.0,
    )
    .translate(172.5, 280.0, BASE_Z / 2.0 + 19.0);
    let dry_trace_divider = centered_cube(
        "closed_sampling_carryover_station_traceability_dry_zone_divider",
        STATION_X - 170.0,
        8.0,
        24.0,
    )
    .translate(0.0, -162.0, BASE_Z / 2.0 + 12.0);

    wet_to_custody + left_lane_split + right_lane_split + dry_trace_divider
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_sampling_carryover_station_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 180.0);
        let boss = centered_cylinder(
            format!("closed_sampling_carryover_station_leak_sensor_boss_{i}"),
            15.0,
            5.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 62.0, BASE_Z / 2.0 + 2.5);
        let pocket = centered_cylinder(
            format!("closed_sampling_carryover_station_leak_sensor_pocket_{i}"),
            7.0,
            7.0,
            28,
        )
        .translate(x, -STATION_Y / 2.0 + 62.0, BASE_Z / 2.0 + 3.0);
        wells = wells + (boss - pocket);
    }
    wells
}

fn robot_fiducials() -> Part {
    let mut targets = Part::empty("closed_sampling_carryover_station_robot_fiducials");
    for (i, (x, y)) in [
        (-530.0, 332.0),
        (-210.0, 332.0),
        (210.0, 332.0),
        (530.0, 332.0),
        (-530.0, -332.0),
        (530.0, -332.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(&format!(
                "closed_sampling_carryover_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    targets
}

fn closed_sample_loop_cartridge_nests() -> Part {
    let body = centered_cube(
        "closed_sampling_carryover_station_loop_nest_body",
        LOOP_NEST_X,
        LOOP_NEST_Y,
        LOOP_NEST_Z,
    );
    let rear_hinge = centered_cube(
        "closed_sampling_carryover_station_loop_nest_rear_closed_hinge_bar",
        LOOP_NEST_X,
        14.0,
        28.0,
    )
    .translate(0.0, LOOP_NEST_Y / 2.0 - 7.0, LOOP_NEST_Z / 2.0 + 14.0);
    let front_latch = centered_cube(
        "closed_sampling_carryover_station_loop_nest_front_seal_latch_bar",
        LOOP_NEST_X,
        14.0,
        24.0,
    )
    .translate(0.0, -(LOOP_NEST_Y / 2.0 - 7.0), LOOP_NEST_Z / 2.0 + 12.0);
    let gasket_land = centered_cube(
        "closed_sampling_carryover_station_loop_nest_gasket_compression_land",
        LOOP_NEST_X - 48.0,
        LOOP_NEST_Y - 36.0,
        8.0,
    )
    .translate(0.0, 0.0, LOOP_NEST_Z / 2.0 + 4.0);

    body - loop_cartridge_pockets() - loop_tube_bores()
        + rear_hinge
        + front_latch
        + gasket_land
        + loop_latch_tabs()
        + loop_datum_posts()
}

fn loop_cartridge_pockets() -> Part {
    let mut pockets = Part::empty("closed_sampling_carryover_station_loop_cartridge_pockets");
    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = centered_index(i, SAMPLE_LOOP_CARTRIDGES, LOOP_PITCH_X);
        let pocket = centered_cube(
            format!("closed_sampling_carryover_station_closed_loop_cartridge_pocket_{i}"),
            LOOP_POCKET_X,
            LOOP_POCKET_Y,
            LOOP_POCKET_Z,
        )
        .translate(x, 0.0, LOOP_NEST_Z / 2.0 - LOOP_POCKET_Z / 2.0 + 2.0);
        let thumb_relief = centered_cube(
            format!("closed_sampling_carryover_station_loop_cartridge_thumb_relief_{i}"),
            LOOP_POCKET_X - 14.0,
            12.0,
            LOOP_POCKET_Z + 4.0,
        )
        .translate(x, -(LOOP_POCKET_Y / 2.0 - 8.0), LOOP_NEST_Z / 2.0 - 8.0);
        pockets = pockets + pocket + thumb_relief;
    }
    pockets
}

fn loop_tube_bores() -> Part {
    let mut bores = Part::empty("closed_sampling_carryover_station_loop_tube_bores");
    for i in 0..SAMPLE_LOOP_CARTRIDGES {
        let x = centered_index(i, SAMPLE_LOOP_CARTRIDGES, LOOP_PITCH_X);
        let inlet = centered_cylinder(
            format!("closed_sampling_carryover_station_loop_inlet_bore_{i}"),
            LOOP_TUBE_BORE_D / 2.0,
            LOOP_NEST_Y + 18.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 10.0, 0.0, 2.0);
        let outlet = centered_cylinder(
            format!("closed_sampling_carryover_station_loop_outlet_bore_{i}"),
            LOOP_TUBE_BORE_D / 2.0,
            LOOP_NEST_Y + 18.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 10.0, 0.0, 2.0);
        bores = bores + inlet + outlet;
    }
    bores
}

fn loop_latch_tabs() -> Part {
    let mut tabs = Part::empty("closed_sampling_carryover_station_loop_latch_tabs");
    for i in 0..LOOP_SEAL_LATCHES {
        let x = centered_index(i, LOOP_SEAL_LATCHES, LOOP_PITCH_X);
        tabs = tabs
            + centered_cube(
                format!("closed_sampling_carryover_station_loop_latch_tab_{i}"),
                28.0,
                16.0,
                18.0,
            )
            .translate(x, -(LOOP_NEST_Y / 2.0 + 9.0), LOOP_NEST_Z / 2.0 + 9.0);
    }
    tabs
}

fn loop_datum_posts() -> Part {
    let mut posts = Part::empty("closed_sampling_carryover_station_loop_datum_posts");
    for (i, (x, y)) in [
        (-(LOOP_NEST_X / 2.0 - 34.0), -(LOOP_NEST_Y / 2.0 - 28.0)),
        (LOOP_NEST_X / 2.0 - 34.0, -(LOOP_NEST_Y / 2.0 - 28.0)),
        (-(LOOP_NEST_X / 2.0 - 34.0), LOOP_NEST_Y / 2.0 - 28.0),
        (LOOP_NEST_X / 2.0 - 34.0, LOOP_NEST_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_sampling_carryover_station_loop_datum_post_{i}"),
                8.0,
                14.0,
                24,
            )
            .translate(*x, *y, LOOP_NEST_Z / 2.0 + 7.0);
    }
    posts
}

fn high_low_standard_rack() -> Part {
    let body = centered_cube(
        "closed_sampling_carryover_station_high_low_standard_rack_body",
        STANDARD_RACK_X,
        STANDARD_RACK_Y,
        STANDARD_RACK_Z,
    );
    let rear_fence = centered_cube(
        "closed_sampling_carryover_station_standard_rack_rear_fence",
        STANDARD_RACK_X,
        12.0,
        42.0,
    )
    .translate(
        0.0,
        STANDARD_RACK_Y / 2.0 - 6.0,
        STANDARD_RACK_Z / 2.0 + 21.0,
    );
    let split_rail = centered_cube(
        "closed_sampling_carryover_station_high_low_standard_split_rail",
        STANDARD_RACK_X - 36.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, STANDARD_RACK_Z / 2.0 + 12.0);

    body - standard_position_pockets()
        + rear_fence
        + split_rail
        + high_low_standard_position_tags()
        + standard_certificate_clip_slots()
}

fn standard_position_pockets() -> Part {
    let mut pockets = Part::empty("closed_sampling_carryover_station_standard_position_pockets");
    for i in 0..STANDARD_POSITIONS {
        let x = centered_index(i, STANDARD_POSITIONS, STANDARD_PITCH_X);
        let y = if standard_level(i) == StandardLevel::High {
            28.0
        } else {
            -28.0
        };
        let diameter = if standard_level(i) == StandardLevel::High {
            HIGH_STANDARD_D
        } else {
            LOW_STANDARD_D
        };
        pockets = pockets
            + centered_cylinder(
                format!(
                    "closed_sampling_carryover_station_{:?}_standard_well_{i}",
                    standard_level(i)
                ),
                diameter / 2.0,
                STANDARD_RACK_Z + 8.0,
                30,
            )
            .translate(x, y, 0.0);
    }
    pockets
}

fn high_low_standard_position_tags() -> Part {
    let mut tags = Part::empty("closed_sampling_carryover_station_high_low_standard_tags");
    for i in 0..STANDARD_POSITIONS {
        let x = centered_index(i, STANDARD_POSITIONS, STANDARD_PITCH_X);
        let y = if standard_level(i) == StandardLevel::High {
            52.0
        } else {
            -52.0
        };
        let label = if standard_level(i) == StandardLevel::High {
            "high_concentration_standard_land"
        } else {
            "low_concentration_standard_land"
        };
        tags = tags
            + centered_cube(
                format!("closed_sampling_carryover_station_{label}_{i}"),
                30.0,
                14.0,
                5.0,
            )
            .translate(x, y, STANDARD_RACK_Z / 2.0 + 2.5);
    }
    tags
}

fn standard_certificate_clip_slots() -> Part {
    let mut clips = Part::empty("closed_sampling_carryover_station_standard_certificate_clips");
    for i in 0..4 {
        clips = clips
            + centered_cube(
                format!("closed_sampling_carryover_station_standard_certificate_clip_{i}"),
                68.0,
                10.0,
                16.0,
            )
            .translate(
                centered_index(i, 4, 112.0),
                STANDARD_RACK_Y / 2.0 + 8.0,
                14.0,
            );
    }
    clips
}

fn flush_waste_routing_manifold() -> Part {
    let body = centered_cube(
        "closed_sampling_carryover_station_flush_waste_routing_body",
        ROUTING_X,
        ROUTING_Y,
        ROUTING_Z,
    );
    body - flush_waste_port_bores() - flush_waste_headers()
        + flush_port_lands()
        + waste_sump_lands()
        + purge_valve_actuator_pads()
}

fn flush_waste_port_bores() -> Part {
    let mut bores = Part::empty("closed_sampling_carryover_station_flush_waste_port_bores");
    for i in 0..FLUSH_PORTS {
        let x = centered_index(i, FLUSH_PORTS, ROUTING_PORT_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("closed_sampling_carryover_station_flush_port_bore_{i}"),
                FLUSH_BORE_D / 2.0,
                ROUTING_Z + 8.0,
                24,
            )
            .translate(x, 24.0, 0.0)
            + centered_cylinder(
                format!("closed_sampling_carryover_station_waste_port_bore_{i}"),
                WASTE_BORE_D / 2.0,
                ROUTING_Z + 8.0,
                28,
            )
            .translate(x, -24.0, 0.0);
    }
    bores
}

fn flush_waste_headers() -> Part {
    let flush_header = centered_cylinder(
        "closed_sampling_carryover_station_flush_header_bore",
        FLUSH_BORE_D / 2.0,
        ROUTING_X - 80.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 24.0, 4.0);
    let waste_header = centered_cylinder(
        "closed_sampling_carryover_station_waste_header_bore",
        WASTE_BORE_D / 2.0,
        ROUTING_X - 80.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -24.0, 4.0);
    let waste_outlet = centered_cylinder(
        "closed_sampling_carryover_station_waste_outlet_bore",
        12.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(ROUTING_X / 2.0 - 42.0, -ROUTING_Y / 2.0 - 2.0, 4.0);

    flush_header + waste_header + waste_outlet
}

fn flush_port_lands() -> Part {
    let mut lands = Part::empty("closed_sampling_carryover_station_flush_port_lands");
    for i in 0..FLUSH_PORTS {
        let x = centered_index(i, FLUSH_PORTS, ROUTING_PORT_PITCH_X);
        lands = lands
            + centered_cube(
                format!("closed_sampling_carryover_station_flush_port_land_{i}"),
                46.0,
                26.0,
                8.0,
            )
            .translate(x, 24.0, ROUTING_Z / 2.0 + 4.0);
    }
    lands
}

fn waste_sump_lands() -> Part {
    let mut lands = Part::empty("closed_sampling_carryover_station_waste_sump_lands");
    for i in 0..WASTE_PORTS {
        let x = centered_index(i, WASTE_PORTS, ROUTING_PORT_PITCH_X);
        lands = lands
            + centered_cube(
                format!("closed_sampling_carryover_station_waste_sump_land_{i}"),
                52.0,
                28.0,
                9.0,
            )
            .translate(x, -24.0, ROUTING_Z / 2.0 + 4.5);
    }
    lands
}

fn purge_valve_actuator_pads() -> Part {
    let mut pads = Part::empty("closed_sampling_carryover_station_purge_valve_actuator_pads");
    for i in 0..4 {
        let x = centered_index(i, 4, 150.0);
        let pad = centered_cube(
            format!("closed_sampling_carryover_station_purge_valve_pad_{i}"),
            62.0,
            22.0,
            24.0,
        )
        .translate(x, 0.0, ROUTING_Z / 2.0 + 12.0);
        let slot = centered_cube(
            format!("closed_sampling_carryover_station_purge_valve_slot_{i}"),
            34.0,
            8.0,
            26.0,
        )
        .translate(x, 0.0, ROUTING_Z / 2.0 + 12.0);
        pads = pads + (pad - slot);
    }
    pads
}

fn analyzer_dock_placeholder() -> Part {
    let dock = centered_cube(
        "closed_sampling_carryover_station_analyzer_dock_placeholder_body",
        ANALYZER_DOCK_X,
        ANALYZER_DOCK_Y,
        ANALYZER_DOCK_Z,
    );
    let analyzer_envelope = centered_cube(
        "closed_sampling_carryover_station_analyzer_envelope_clearance",
        ANALYZER_ENVELOPE_X,
        ANALYZER_ENVELOPE_Y,
        ANALYZER_ENVELOPE_Z,
    )
    .translate(0.0, -10.0, 10.0);
    let front_window = centered_cube(
        "closed_sampling_carryover_station_analyzer_front_window",
        ANALYZER_ENVELOPE_X - 30.0,
        18.0,
        ANALYZER_ENVELOPE_Z - 20.0,
    )
    .translate(0.0, -(ANALYZER_DOCK_Y / 2.0 - 10.0), 10.0);

    dock - analyzer_envelope - front_window - analyzer_interface_bores()
        + analyzer_alignment_rails()
        + analyzer_latch_posts()
        + analyzer_cable_guard()
}

fn analyzer_interface_bores() -> Part {
    let mut bores = Part::empty("closed_sampling_carryover_station_analyzer_interface_bores");
    for i in 0..ANALYZER_INTERFACE_PORTS {
        bores = bores
            + centered_cylinder(
                format!("closed_sampling_carryover_station_analyzer_interface_port_{i}"),
                FLUSH_BORE_D / 2.0,
                ANALYZER_DOCK_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, ANALYZER_INTERFACE_PORTS, 34.0),
                0.0,
                -28.0,
            );
    }
    bores
}

fn analyzer_alignment_rails() -> Part {
    let left = centered_cube(
        "closed_sampling_carryover_station_analyzer_left_alignment_rail",
        18.0,
        ANALYZER_DOCK_Y - 26.0,
        18.0,
    )
    .translate(
        -(ANALYZER_DOCK_X / 2.0 - 34.0),
        0.0,
        ANALYZER_DOCK_Z / 2.0 + 9.0,
    );
    let right = centered_cube(
        "closed_sampling_carryover_station_analyzer_right_alignment_rail",
        18.0,
        ANALYZER_DOCK_Y - 26.0,
        18.0,
    )
    .translate(
        ANALYZER_DOCK_X / 2.0 - 34.0,
        0.0,
        ANALYZER_DOCK_Z / 2.0 + 9.0,
    );
    left + right
}

fn analyzer_latch_posts() -> Part {
    let mut posts = Part::empty("closed_sampling_carryover_station_analyzer_latch_posts");
    for (i, x) in [-74.0, 74.0].iter().enumerate() {
        posts = posts
            + centered_cylinder(
                format!("closed_sampling_carryover_station_analyzer_latch_post_{i}"),
                11.0,
                34.0,
                28,
            )
            .translate(
                *x,
                ANALYZER_DOCK_Y / 2.0 - 28.0,
                ANALYZER_DOCK_Z / 2.0 + 17.0,
            );
    }
    posts
}

fn analyzer_cable_guard() -> Part {
    centered_cube(
        "closed_sampling_carryover_station_analyzer_cable_guard",
        ANALYZER_DOCK_X - 68.0,
        18.0,
        38.0,
    )
    .translate(
        0.0,
        ANALYZER_DOCK_Y / 2.0 + 9.0,
        -ANALYZER_DOCK_Z / 2.0 + 34.0,
    )
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_sampling_carryover_station_barcode_certificate_plate",
        TRACE_LAND_X,
        TRACE_LAND_Y,
        TRACE_LAND_Z,
    );
    plate - certificate_recesses() + barcode_lands() + certificate_clip_lands() + trace_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_sampling_carryover_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i, BARCODE_LANDS, 44.0);
        lands = lands
            + centered_cube(
                format!("closed_sampling_carryover_station_barcode_land_{i}"),
                34.0,
                18.0,
                4.0,
            )
            .translate(x, TRACE_LAND_Y / 2.0 - 28.0, TRACE_LAND_Z / 2.0 + 2.0);
    }
    lands
}

fn certificate_recesses() -> Part {
    let mut recesses = Part::empty("closed_sampling_carryover_station_certificate_recesses");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 82.0);
        recesses = recesses
            + centered_cube(
                format!("closed_sampling_carryover_station_certificate_recess_{i}"),
                66.0,
                42.0,
                7.0,
            )
            .translate(x, -22.0, TRACE_LAND_Z / 2.0 - 2.0);
    }
    recesses
}

fn certificate_clip_lands() -> Part {
    let mut clips = Part::empty("closed_sampling_carryover_station_certificate_clip_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 82.0);
        clips = clips
            + centered_cube(
                format!("closed_sampling_carryover_station_certificate_clip_land_{i}"),
                58.0,
                8.0,
                8.0,
            )
            .translate(x, -48.0, TRACE_LAND_Z / 2.0 + 4.0);
    }
    clips
}

fn trace_fiducials() -> Part {
    fiducial_target("closed_sampling_carryover_station_trace_fiducial_left").translate(
        -(TRACE_LAND_X / 2.0 - 28.0),
        TRACE_LAND_Y / 2.0 - 28.0,
        TRACE_LAND_Z / 2.0 + 2.0,
    ) + fiducial_target("closed_sampling_carryover_station_trace_fiducial_right").translate(
        TRACE_LAND_X / 2.0 - 28.0,
        TRACE_LAND_Y / 2.0 - 28.0,
        TRACE_LAND_Z / 2.0 + 2.0,
    )
}

fn timed_dwell_flush_token_bank() -> Part {
    let plate = centered_cube(
        "closed_sampling_carryover_station_timed_dwell_flush_token_plate",
        TOKEN_BANK_X,
        TOKEN_BANK_Y,
        TOKEN_BANK_Z,
    );
    plate - timed_token_recesses() + dwell_tokens() + flush_tokens() + timer_reference_rail()
}

fn timed_token_recesses() -> Part {
    let mut recesses = Part::empty("closed_sampling_carryover_station_timed_token_recesses");
    for i in 0..DWELL_TOKENS {
        let x = centered_index(i, DWELL_TOKENS, TOKEN_PITCH_X);
        recesses = recesses
            + centered_cylinder(
                format!("closed_sampling_carryover_station_dwell_token_recess_{i}"),
                17.0,
                TOKEN_BANK_Z + 4.0,
                32,
            )
            .translate(x, 26.0, 0.0)
            + centered_cylinder(
                format!("closed_sampling_carryover_station_flush_token_recess_{i}"),
                17.0,
                TOKEN_BANK_Z + 4.0,
                32,
            )
            .translate(x, -26.0, 0.0);
    }
    recesses
}

fn dwell_tokens() -> Part {
    let mut tokens = Part::empty("closed_sampling_carryover_station_dwell_tokens");
    for i in 0..DWELL_TOKENS {
        let x = centered_index(i, DWELL_TOKENS, TOKEN_PITCH_X);
        tokens = tokens
            + centered_cylinder(
                format!("closed_sampling_carryover_station_timed_dwell_token_{i}"),
                13.5,
                6.0,
                32,
            )
            .translate(x, 26.0, TOKEN_BANK_Z / 2.0 + 3.0);
    }
    tokens
}

fn flush_tokens() -> Part {
    let mut tokens = Part::empty("closed_sampling_carryover_station_flush_tokens");
    for i in 0..FLUSH_TOKENS {
        let x = centered_index(i, FLUSH_TOKENS, TOKEN_PITCH_X);
        tokens = tokens
            + centered_cube(
                format!("closed_sampling_carryover_station_timed_flush_token_{i}"),
                25.0,
                25.0,
                6.0,
            )
            .translate(x, -26.0, TOKEN_BANK_Z / 2.0 + 3.0);
    }
    tokens
}

fn timer_reference_rail() -> Part {
    centered_cube(
        "closed_sampling_carryover_station_timer_reference_rail",
        TOKEN_BANK_X - 42.0,
        8.0,
        18.0,
    )
    .translate(0.0, 0.0, TOKEN_BANK_Z / 2.0 + 9.0)
}

fn status_lane_caddy(status: &str, wall_z: f64) -> Part {
    let body = centered_cube(
        format!("closed_sampling_carryover_station_{status}_lane_caddy_body"),
        STATUS_LANE_X,
        STATUS_LANE_Y,
        STATUS_LANE_Z,
    );
    let rear_wall = centered_cube(
        format!("closed_sampling_carryover_station_{status}_lane_rear_wall"),
        STATUS_LANE_X,
        12.0,
        wall_z,
    )
    .translate(
        0.0,
        STATUS_LANE_Y / 2.0 - 6.0,
        STATUS_LANE_Z / 2.0 + wall_z / 2.0,
    );
    let status_key = centered_cube(
        format!("closed_sampling_carryover_station_{status}_lane_status_key"),
        68.0,
        14.0,
        12.0,
    )
    .translate(
        0.0,
        -(STATUS_LANE_Y / 2.0 - 11.0),
        STATUS_LANE_Z / 2.0 + 6.0,
    );
    let caddy =
        body - status_lane_slots(status) + rear_wall + status_key + lane_side_handles(status);

    if status == "reject" {
        caddy
            + centered_cube(
                "closed_sampling_carryover_station_reject_lane_high_wall_flag",
                STATUS_LANE_X - 28.0,
                10.0,
                34.0,
            )
            .translate(0.0, -8.0, STATUS_LANE_Z / 2.0 + 17.0)
    } else {
        caddy
    }
}

fn status_lane_slots(status: &str) -> Part {
    let mut slots = Part::empty(format!(
        "closed_sampling_carryover_station_{status}_lane_slots"
    ));
    for i in 0..STATUS_SLOTS {
        let x = centered_index(i, STATUS_SLOTS, STATUS_SLOT_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_sampling_carryover_station_{status}_lane_loop_slot_{i}"),
                34.0,
                58.0,
                STATUS_LANE_Z + 6.0,
            )
            .translate(x, -8.0, 4.0)
            + centered_cube(
                format!("closed_sampling_carryover_station_{status}_lane_record_card_slot_{i}"),
                30.0,
                8.0,
                22.0,
            )
            .translate(x, 39.0, STATUS_LANE_Z / 2.0 - 6.0);
    }
    slots
}

fn lane_side_handles(status: &str) -> Part {
    let left = centered_cube(
        format!("closed_sampling_carryover_station_{status}_lane_left_handle"),
        16.0,
        60.0,
        26.0,
    )
    .translate(
        -(STATUS_LANE_X / 2.0 + 8.0),
        0.0,
        STATUS_LANE_Z / 2.0 + 13.0,
    );
    let right = centered_cube(
        format!("closed_sampling_carryover_station_{status}_lane_right_handle"),
        16.0,
        60.0,
        26.0,
    )
    .translate(STATUS_LANE_X / 2.0 + 8.0, 0.0, STATUS_LANE_Z / 2.0 + 13.0);
    left + right
}

fn evidence_camera_bridge() -> Part {
    let beam = centered_cube(
        "closed_sampling_carryover_station_evidence_camera_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        58.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 29.0);
    let left_leg = centered_cube(
        "closed_sampling_carryover_station_evidence_camera_bridge_left_leg",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-(CAMERA_BRIDGE_X / 2.0 - 40.0), 0.0, 0.0);
    let right_leg = centered_cube(
        "closed_sampling_carryover_station_evidence_camera_bridge_right_leg",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 40.0, 0.0, 0.0);
    let front_window = centered_cube(
        "closed_sampling_carryover_station_camera_bridge_evidence_window",
        CAMERA_BRIDGE_X - 150.0,
        CAMERA_BRIDGE_Y + 6.0,
        96.0,
    )
    .translate(0.0, 0.0, -16.0);

    beam + left_leg + right_leg - front_window + camera_pods() + evidence_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("closed_sampling_carryover_station_evidence_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 310.0);
        let pod = centered_cube(
            format!("closed_sampling_carryover_station_evidence_camera_pod_{i}"),
            72.0,
            44.0,
            34.0,
        )
        .translate(x, -(CAMERA_BRIDGE_Y / 2.0 + 20.0), CAMERA_CLEARANCE_Z);
        let lens = centered_cylinder(
            format!("closed_sampling_carryover_station_evidence_camera_lens_{i}"),
            15.0,
            12.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(CAMERA_BRIDGE_Y / 2.0 + 42.0), CAMERA_CLEARANCE_Z);
        pods = pods + pod + lens;
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("closed_sampling_carryover_station_evidence_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        let x = centered_index(i, LIGHT_BAR_COUNT, 230.0);
        bars = bars
            + centered_cube(
                format!("closed_sampling_carryover_station_evidence_led_bar_{i}"),
                120.0,
                10.0,
                14.0,
            )
            .translate(x, CAMERA_BRIDGE_Y / 2.0 + 5.0, CAMERA_CLEARANCE_Z - 34.0);
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let robot_sweep = keepout_box(
        "closed_sampling_carryover_station_robot_loop_load_keepout",
        860.0,
        240.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(-60.0, 100.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    let camera_focus = keepout_box(
        "closed_sampling_carryover_station_camera_focus_keepout",
        CAMERA_BRIDGE_X - 110.0,
        156.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(0.0, 30.0, BASE_Z + CAMERA_CLEARANCE_Z / 2.0);
    let front_service = keepout_box(
        "closed_sampling_carryover_station_front_waste_service_keepout",
        STATION_X - 170.0,
        FRONT_SERVICE_CLEARANCE,
        88.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        BASE_Z + 44.0,
    );
    let rear_service = keepout_box(
        "closed_sampling_carryover_station_rear_standard_service_keepout",
        STATION_X - 190.0,
        REAR_SERVICE_CLEARANCE,
        104.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 52.0,
    );
    let analyzer_service = keepout_box(
        "closed_sampling_carryover_station_right_analyzer_service_keepout",
        ANALYZER_SERVICE_CLEARANCE,
        STATION_Y - 150.0,
        132.0,
    )
    .translate(
        STATION_X / 2.0 + ANALYZER_SERVICE_CLEARANCE / 2.0,
        -38.0,
        BASE_Z + 66.0,
    );

    robot_sweep + camera_focus + front_service + rear_service + analyzer_service
}

fn closed_tube_route_placeholders() -> Part {
    let loop_to_routing = tube_span_y(
        "closed_sampling_carryover_station_loop_to_flush_routing_bundle",
        LOOP_NEST_POS.1 - ROUTING_POS.1 - LOOP_NEST_Y / 2.0 - ROUTING_Y / 2.0 + 16.0,
    )
    .translate(-300.0, 42.0, BASE_Z + LOOP_NEST_Z + 18.0);
    let standards_to_loops = tube_span_x(
        "closed_sampling_carryover_station_standard_to_loop_high_low_bundle",
        STANDARD_RACK_POS.0 - LOOP_NEST_POS.0 - LOOP_NEST_X / 2.0 - 42.0,
    )
    .translate(40.0, 142.0, BASE_Z + STANDARD_RACK_Z + 20.0);
    let routing_to_analyzer = tube_span_x(
        "closed_sampling_carryover_station_routing_to_analyzer_interface_bundle",
        ANALYZER_POS.0 - ROUTING_POS.0 - ANALYZER_DOCK_X / 2.0,
    )
    .translate(226.0, -78.0, BASE_Z + ROUTING_Z + 18.0);
    let waste_drop = tube_span_y(
        "closed_sampling_carryover_station_waste_drop_to_leak_tray_bundle",
        190.0,
    )
    .translate(380.0, -176.0, BASE_Z + ROUTING_Z + 16.0);

    loop_to_routing + standards_to_loops + routing_to_analyzer + waste_drop
}

fn tube_span_x(name: &str, length: f64) -> Part {
    centered_cylinder(name, 3.2, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_span_y(name: &str, length: f64) -> Part {
    centered_cylinder(name, 3.2, length, 24).rotate(90.0, 0.0, 0.0)
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let envelope = centered_cube(format!("{name}_envelope"), x, y, z);
    let relief = centered_cube(
        format!("{name}_interior_relief"),
        x - 18.0,
        y - 18.0,
        z + 4.0,
    );
    envelope - relief
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 14.0, 3.0, 40);
    let inner = centered_cylinder(format!("{name}_inner"), 6.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 34.0, 2.6, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.6, 34.0, 4.0);
    outer - inner + cross_x + cross_y
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn wet_to_status_gap() -> f64 {
    RELEASED_POS.1 - STATUS_LANE_Y / 2.0 - (STANDARD_RACK_POS.1 + STANDARD_RACK_Y / 2.0)
}

fn status_lane_gap() -> f64 {
    HOLD_POS.0 - RELEASED_POS.0 - STATUS_LANE_X
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StandardLevel {
    High,
    Low,
}

fn standard_level(index: usize) -> StandardLevel {
    if index % 2 == 0 {
        StandardLevel::High
    } else {
        StandardLevel::Low
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> Rect {
    Rect {
        x0: center.0 - width / 2.0,
        x1: center.0 + width / 2.0,
        y0: center.1 - depth / 2.0,
        y1: center.1 + depth / 2.0,
    }
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.x0 < b.x1 && a.x1 > b.x0 && a.y0 < b.y1 && a.y1 > b.y0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_include_assembly() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sampling_carryover_flush_validation_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS
            .contains(&"output/closed_sampling_carryover_flush_validation_station_assembly.stl"));
    }

    #[test]
    fn required_physical_features_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"closed_sample_loop_cartridge_nests"));
        assert!(
            REQUIRED_FEATURES.contains(&"alternating_high_low_concentration_standard_positions")
        );
        assert!(REQUIRED_FEATURES.contains(&"flush_waste_routing"));
        assert!(REQUIRED_FEATURES.contains(&"analyzer_dock_placeholder"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"timed_dwell_flush_tokens"));
        assert!(REQUIRED_FEATURES.contains(&"released_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"status_custody_separation"));
    }

    #[test]
    fn basic_station_dimensions_and_counts_match_validation_scope() {
        assert!(STATION_X <= 1200.0);
        assert!(STATION_Y <= 800.0);
        assert!(LEAK_BASIN_X < STATION_X);
        assert!(LEAK_BASIN_Y < STATION_Y);
        assert_eq!(SAMPLE_LOOP_CARTRIDGES, 8);
        assert_eq!(FLUSH_PORTS, SAMPLE_LOOP_CARTRIDGES);
        assert_eq!(WASTE_PORTS, SAMPLE_LOOP_CARTRIDGES);
        assert_eq!(STANDARD_POSITIONS, 12);
        assert_eq!(BARCODE_LANDS + CERTIFICATE_LANDS, STANDARD_POSITIONS);
        assert_eq!(DWELL_TOKENS, FLUSH_TOKENS);
        assert!(ANALYZER_ENVELOPE_X < ANALYZER_DOCK_X);
        assert!(ANALYZER_ENVELOPE_Y < ANALYZER_DOCK_Y);
        assert!(CAMERA_CLEARANCE_Z > BASE_Z + LOOP_NEST_Z);
    }

    #[test]
    fn high_low_standard_positions_strictly_alternate() {
        let mut high_count = 0;
        let mut low_count = 0;
        for i in 0..STANDARD_POSITIONS {
            if standard_level(i) == StandardLevel::High {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if i > 0 {
                assert_ne!(standard_level(i), standard_level(i - 1));
            }
        }
        assert_eq!(high_count, low_count);
        assert_eq!(high_count + low_count, STANDARD_POSITIONS);
    }

    #[test]
    fn major_modules_fit_and_do_not_collide() {
        assert_layout();
    }

    #[test]
    fn status_and_custody_paths_are_physically_separated() {
        assert_eq!(RELEASED_POS.1, HOLD_POS.1);
        assert_eq!(HOLD_POS.1, REJECT_POS.1);
        assert!(RELEASED_POS.0 < HOLD_POS.0);
        assert!(HOLD_POS.0 < REJECT_POS.0);
        assert!(status_lane_gap() >= STATUS_LANE_GAP_MIN);
        assert!(REJECT_POS.0 - HOLD_POS.0 - STATUS_LANE_X >= STATUS_LANE_GAP_MIN);
        assert!(wet_to_status_gap() >= STATUS_CUSTODY_GAP_MIN);
        assert!(STATUS_LANE_Z < CAMERA_CLEARANCE_Z);
    }

    #[test]
    fn flush_and_waste_paths_remain_separate_until_waste_sump() {
        assert_eq!(FLUSH_PORTS, WASTE_PORTS);
        assert!(WASTE_BORE_D > FLUSH_BORE_D);
        assert!((24.0 - -24.0) - (WASTE_BORE_D + FLUSH_BORE_D) / 2.0 >= WASTE_CHANNEL_MIN_GAP);
        assert!(ROUTING_PORT_PITCH_X > LOOP_PITCH_X);
    }

    #[test]
    fn robot_service_keepouts_are_visible_and_named() {
        assert_eq!(ROBOT_KEEP_OUT_ZONES, 4);
        assert!(ROBOT_KEEP_OUT_Z >= 200.0);
        assert!(FRONT_SERVICE_CLEARANCE >= 340.0);
        assert!(REAR_SERVICE_CLEARANCE >= 220.0);
        assert!(ANALYZER_SERVICE_CLEARANCE >= 180.0);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LIGHT_BAR_COUNT, 4);
    }
}
