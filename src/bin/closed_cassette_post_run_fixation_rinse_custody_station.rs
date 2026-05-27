use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette post-run fixation, rinse, and sample-custody station.
//
// Intent:
// - Keep tissue-chip cassettes docked and sealed after culture runs while
//   fixation, rinse, waste routing, and archive handoff are represented as
//   closed-path validation geometry.
// - Provide deterministic fixture features for traceability, dwell timing,
//   chemical compatibility coupons, evidence capture, and clean/used material
//   segregation without implying an open manual handling workflow.
// - Reserve robot and service keepouts so downstream integration can validate
//   cassette transfer, barcode reads, and post-run custody without shared CAD
//   edits.
//
// This is architecture/fit CAD only. It is not a sterile barrier, wetted-path,
// tissue handling, chemical exposure, or release-test specification.

const OUTPUTS: &[&str] = &[
    "output/closed_cassette_post_run_fixation_rinse_custody_station_base_leak_overflow_tray.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_sealed_cassette_dock.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_fixative_input_port_surrogates.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_rinse_waste_route_manifold.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_timed_dwell_token_lanes.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_sample_archive_handoff_nests.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_barcode_status_lands.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_chemical_compatibility_coupon_pockets.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_clean_used_segregation_barriers.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_evidence_bridge.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_robot_service_keepouts.stl",
    "output/closed_cassette_post_run_fixation_rinse_custody_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "sealed_cassette_dock",
    "fixative_input_port_surrogates",
    "rinse_waste_route_manifold",
    "timed_dwell_token_lanes",
    "leak_overflow_tray",
    "sample_archive_handoff_nests",
    "barcode_status_lands",
    "chemical_compatibility_coupon_pockets",
    "clean_used_segregation",
    "evidence_bridge",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const TRAY_WALL_W: f64 = 18.0;
const TRAY_WALL_Z: f64 = 48.0;
const LEAK_BASIN_X: f64 = STATION_X - 126.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 118.0;
const LEAK_BASIN_Z: f64 = 8.0;
const SOCKET_Z: f64 = 5.2;
const DRAIN_D: f64 = 18.0;
const OVERFLOW_CHANNELS: usize = 4;
const LEAK_SENSOR_WELLS: usize = 6;

const DOCK_X: f64 = 520.0;
const DOCK_Y: f64 = 250.0;
const DOCK_Z: f64 = 56.0;
const CASSETTE_SLOTS: usize = 4;
const CASSETTE_SLOT_X: f64 = 104.0;
const CASSETTE_SLOT_Y: f64 = 178.0;
const CASSETTE_SLOT_Z: f64 = 34.0;
const CASSETTE_PITCH_X: f64 = 118.0;
const SEAL_GASKET_W: f64 = 8.0;
const LATCHES_PER_SLOT: usize = 2;

const FIXATIVE_PANEL_X: f64 = 360.0;
const FIXATIVE_PANEL_Y: f64 = 190.0;
const FIXATIVE_PANEL_Z: f64 = 58.0;
const FIXATIVE_PORTS: usize = 6;
const FIXATIVE_PORT_PITCH_X: f64 = 52.0;
const FIXATIVE_PORT_D: f64 = 15.0;
const VENT_SURROGATE_D: f64 = 8.0;

const MANIFOLD_X: f64 = 560.0;
const MANIFOLD_Y: f64 = 170.0;
const MANIFOLD_Z: f64 = 54.0;
const RINSE_BRANCHES: usize = 4;
const WASTE_BRANCHES: usize = 4;
const ROUTE_PITCH_X: f64 = 112.0;
const RINSE_BORE_D: f64 = 8.0;
const WASTE_BORE_D: f64 = 12.0;
const WASTE_TROUGH_X: f64 = 500.0;
const WASTE_TROUGH_Y: f64 = 32.0;
const WASTE_TROUGH_Z: f64 = 18.0;

const DWELL_BANK_X: f64 = 480.0;
const DWELL_BANK_Y: f64 = 146.0;
const DWELL_BANK_Z: f64 = 28.0;
const DWELL_LANES: usize = 4;
const DWELL_TOKENS_PER_LANE: usize = 3;
const DWELL_LANE_PITCH_X: f64 = 108.0;
const DWELL_TOKEN_PITCH_Y: f64 = 42.0;
const DWELL_TOKEN_D: f64 = 24.0;
const DWELL_LANE_W: f64 = 78.0;

const ARCHIVE_BANK_X: f64 = 430.0;
const ARCHIVE_BANK_Y: f64 = 190.0;
const ARCHIVE_BANK_Z: f64 = 42.0;
const ARCHIVE_NESTS: usize = 6;
const ARCHIVE_NEST_PITCH_X: f64 = 64.0;
const ARCHIVE_NEST_D: f64 = 30.0;
const ARCHIVE_SEAL_RING_D: f64 = 42.0;

const TRACE_PANEL_X: f64 = 285.0;
const TRACE_PANEL_Y: f64 = 112.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const STATUS_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 62.0;
const BARCODE_LAND_Y: f64 = 18.0;
const STATUS_LAND_D: f64 = 20.0;

const COUPON_BANK_X: f64 = 330.0;
const COUPON_BANK_Y: f64 = 165.0;
const COUPON_BANK_Z: f64 = 32.0;
const COUPON_POCKETS: usize = 8;
const COUPON_PITCH_X: f64 = 74.0;
const COUPON_POCKET_X: f64 = 42.0;
const COUPON_POCKET_Y: f64 = 56.0;
const COUPON_POCKET_Z: f64 = 18.0;

const CLEAN_USED_BARRIER_Z: f64 = 44.0;
const EVIDENCE_BRIDGE_X: f64 = 1120.0;
const EVIDENCE_BRIDGE_Y: f64 = 74.0;
const EVIDENCE_POST_X: f64 = 30.0;
const EVIDENCE_POST_Y: f64 = 44.0;
const EVIDENCE_POST_Z: f64 = 220.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CAMERAS: usize = 4;
const EVIDENCE_LIGHT_BARS: usize = 4;

const ROBOT_KEEP_OUT_WINDOWS: usize = 5;
const ROBOT_KEEP_OUT_Z: f64 = 188.0;
const FRONT_SERVICE_CLEARANCE: f64 = 380.0;
const REAR_SERVICE_CLEARANCE: f64 = 245.0;
const LEFT_CLEAN_SERVICE_CLEARANCE: f64 = 170.0;
const RIGHT_WASTE_SERVICE_CLEARANCE: f64 = 210.0;

const DOCK_POS: (f64, f64) = (-300.0, 150.0);
const FIXATIVE_POS: (f64, f64) = (390.0, 160.0);
const MANIFOLD_POS: (f64, f64) = (315.0, -75.0);
const DWELL_POS: (f64, f64) = (-360.0, -110.0);
const ARCHIVE_POS: (f64, f64) = (260.0, -290.0);
const TRACE_POS: (f64, f64) = (-485.0, 348.0);
const COUPON_POS: (f64, f64) = (-365.0, -305.0);
const EVIDENCE_POS: (f64, f64) = (0.0, 34.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_overflow_tray();
    export(&base, OUTPUTS[0]);

    let dock = sealed_cassette_dock();
    export(&dock, OUTPUTS[1]);

    let fixative = fixative_input_port_surrogates();
    export(&fixative, OUTPUTS[2]);

    let manifold = rinse_waste_route_manifold();
    export(&manifold, OUTPUTS[3]);

    let dwell = timed_dwell_token_lanes();
    export(&dwell, OUTPUTS[4]);

    let archive = sample_archive_handoff_nests();
    export(&archive, OUTPUTS[5]);

    let trace = barcode_status_lands();
    export(&trace, OUTPUTS[6]);

    let coupons = chemical_compatibility_coupon_pockets();
    export(&coupons, OUTPUTS[7]);

    let segregation = clean_used_segregation_barriers();
    export(&segregation, OUTPUTS[8]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + dock
        + fixative
        + manifold
        + dwell
        + archive
        + trace
        + coupons
        + segregation
        + bridge
        + keepouts
        + closed_route_placeholders();
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed cassette post-run fixation/rinse custody station: {:.0}mm x {:.0}mm leak/overflow tray, {} sealed cassette docks, {} fixative input/vent surrogates, {} rinse/waste branches, and {} timed dwell token positions.",
        STATION_X,
        STATION_Y,
        CASSETTE_SLOTS,
        FIXATIVE_PORTS * 2,
        RINSE_BRANCHES + WASTE_BRANCHES,
        DWELL_LANES * DWELL_TOKENS_PER_LANE
    );
    println!(
        "Custody geometry: {} archive nests, {} barcode/status lands, {} compatibility coupon pockets, {} leak wells, clean/used barriers, and evidence bridge with {} cameras.",
        ARCHIVE_NESTS,
        BARCODE_LANDS + STATUS_LANDS,
        COUPON_POCKETS,
        LEAK_SENSOR_WELLS,
        EVIDENCE_CAMERAS
    );
    println!(
        "Keepouts: {} robot windows, front {:.0}mm, rear {:.0}mm, left clean {:.0}mm, right waste {:.0}mm; {} required feature groups.",
        ROBOT_KEEP_OUT_WINDOWS,
        FRONT_SERVICE_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        LEFT_CLEAN_SERVICE_CLEARANCE,
        RIGHT_WASTE_SERVICE_CLEARANCE,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(RINSE_BRANCHES, CASSETTE_SLOTS);
    assert_eq!(WASTE_BRANCHES, CASSETTE_SLOTS);
    assert!(ARCHIVE_NESTS >= CASSETTE_SLOTS);
    assert!(COUPON_POCKETS >= 2 * CASSETTE_SLOTS);
    assert!(DWELL_LANES == CASSETTE_SLOTS);

    for (name, center, width, depth) in fixture_specs() {
        assert!(
            fits_on_station(center, width, depth, 24.0),
            "{name} exceeds station envelope"
        );
    }

    let dock = rect(DOCK_POS, DOCK_X, DOCK_Y);
    let fixative = rect(FIXATIVE_POS, FIXATIVE_PANEL_X, FIXATIVE_PANEL_Y);
    let manifold = rect(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y);
    let dwell = rect(DWELL_POS, DWELL_BANK_X, DWELL_BANK_Y);
    let archive = rect(ARCHIVE_POS, ARCHIVE_BANK_X, ARCHIVE_BANK_Y);
    let trace = rect(TRACE_POS, TRACE_PANEL_X, TRACE_PANEL_Y);
    let coupons = rect(COUPON_POS, COUPON_BANK_X, COUPON_BANK_Y);

    for (left_name, left, right_name, right) in [
        ("dock", dock, "fixative", fixative),
        ("dock", dock, "dwell", dwell),
        ("fixative", fixative, "manifold", manifold),
        ("manifold", manifold, "archive", archive),
        ("dwell", dwell, "coupons", coupons),
        ("archive", archive, "coupons", coupons),
        ("trace", trace, "dock", dock),
    ] {
        assert!(
            !rects_overlap(left, right, 8.0),
            "{left_name} overlaps {right_name}"
        );
    }
}

fn fixture_specs() -> [(&'static str, (f64, f64), f64, f64); 7] {
    [
        ("sealed_cassette_dock", DOCK_POS, DOCK_X, DOCK_Y),
        (
            "fixative_input_port_surrogates",
            FIXATIVE_POS,
            FIXATIVE_PANEL_X,
            FIXATIVE_PANEL_Y,
        ),
        (
            "rinse_waste_route_manifold",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        (
            "timed_dwell_token_lanes",
            DWELL_POS,
            DWELL_BANK_X,
            DWELL_BANK_Y,
        ),
        (
            "sample_archive_handoff_nests",
            ARCHIVE_POS,
            ARCHIVE_BANK_X,
            ARCHIVE_BANK_Y,
        ),
        (
            "barcode_status_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "chemical_compatibility_coupon_pockets",
            COUPON_POS,
            COUPON_BANK_X,
            COUPON_BANK_Y,
        ),
    ]
}

fn base_leak_overflow_tray() -> Part {
    let deck = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_Z,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - LEAK_BASIN_Z / 2.0 + 0.4);
    let drain = centered_cylinder(
        "closed_post_run_fixation_rinse_custody_station_front_overflow_drain_surrogate",
        DRAIN_D / 2.0,
        72.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 - 4.0, 0.0);

    deck - basin - drain - fixture_sockets() - mount_slots()
        + tray_rims()
        + overflow_channels()
        + leak_sensor_wells()
        + datum_targets()
}

fn fixture_sockets() -> Part {
    let mut sockets = Part::empty("closed_post_run_fixation_rinse_custody_station_fixture_sockets");
    for (name, center, width, depth) in fixture_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_{name}_socket"),
                width + 10.0,
                depth + 10.0,
                SOCKET_Z + 0.4,
            )
            .translate(center.0, center.1, BASE_Z / 2.0 - SOCKET_Z / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_post_run_fixation_rinse_custody_station_mount_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 56.0), -(STATION_Y / 2.0 - 52.0)),
        (STATION_X / 2.0 - 56.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 56.0), STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 52.0),
        (0.0, -(STATION_Y / 2.0 - 52.0)),
        (0.0, STATION_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_m6_mount_bore_{i}"),
                3.4,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_m6_mount_slot_{i}"),
                28.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_left_tray_wall",
        TRAY_WALL_W,
        STATION_Y,
        TRAY_WALL_Z,
    )
    .translate(
        -STATION_X / 2.0 + TRAY_WALL_W / 2.0,
        0.0,
        z_on_base(TRAY_WALL_Z),
    );
    let right = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_right_tray_wall",
        TRAY_WALL_W,
        STATION_Y,
        TRAY_WALL_Z,
    )
    .translate(
        STATION_X / 2.0 - TRAY_WALL_W / 2.0,
        0.0,
        z_on_base(TRAY_WALL_Z),
    );
    let rear = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_rear_tray_wall",
        STATION_X,
        TRAY_WALL_W,
        TRAY_WALL_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - TRAY_WALL_W / 2.0,
        z_on_base(TRAY_WALL_Z),
    );
    let front_lip = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_front_low_overflow_lip",
        STATION_X - 170.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, z_on_base(24.0));

    left + right + rear + front_lip
}

fn overflow_channels() -> Part {
    let mut channels =
        Part::empty("closed_post_run_fixation_rinse_custody_station_overflow_channels");
    for i in 0..OVERFLOW_CHANNELS {
        let x = centered_index(i, OVERFLOW_CHANNELS, 230.0);
        channels = channels
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_overflow_channel_{i}"),
                150.0,
                12.0,
                6.0,
            )
            .translate(x, -STATION_Y / 2.0 + 66.0, BASE_Z / 2.0 + 3.0);
    }
    channels
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_post_run_fixation_rinse_custody_station_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 170.0);
        let boss = centered_cylinder(
            format!("closed_post_run_fixation_rinse_custody_station_leak_sensor_boss_{i}"),
            15.0,
            5.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 104.0, BASE_Z / 2.0 + 2.5);
        let pocket = centered_cylinder(
            format!("closed_post_run_fixation_rinse_custody_station_leak_sensor_pocket_{i}"),
            7.0,
            7.0,
            28,
        )
        .translate(x, -STATION_Y / 2.0 + 104.0, BASE_Z / 2.0 + 3.0);
        wells = wells + (boss - pocket);
    }
    wells
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_post_run_fixation_rinse_custody_station_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 90.0, STATION_Y / 2.0 - 86.0),
        (STATION_X / 2.0 - 90.0, STATION_Y / 2.0 - 86.0),
        (-STATION_X / 2.0 + 90.0, -STATION_Y / 2.0 + 150.0),
        (STATION_X / 2.0 - 90.0, -STATION_Y / 2.0 + 150.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_robot_datum_target_{i}"),
                16.0,
                3.0,
                40,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5)
            - centered_cylinder(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_robot_datum_cross_bore_{i}"
                ),
                4.0,
                5.0,
                20,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5);
    }
    targets
}

fn sealed_cassette_dock() -> Part {
    let deck = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_sealed_cassette_dock_deck",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, z_on_base(DOCK_Z));
    let mut cutouts = Part::empty("closed_post_run_fixation_rinse_custody_station_dock_cutouts");
    let mut features = Part::empty("closed_post_run_fixation_rinse_custody_station_dock_features");
    for i in 0..CASSETTE_SLOTS {
        let x = DOCK_POS.0 + centered_index(i, CASSETTE_SLOTS, CASSETTE_PITCH_X);
        cutouts = cutouts
            + centered_cube(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_cassette_cradle_pocket_{i}"
                ),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                CASSETTE_SLOT_Z,
            )
            .translate(x, DOCK_POS.1, z_on_base(DOCK_Z) + 8.0);
        features = features
            + gasket_frame(
                format!("closed_post_run_fixation_rinse_custody_station_seal_gasket_frame_{i}"),
                x,
                DOCK_POS.1,
                CASSETTE_SLOT_X + 18.0,
                CASSETTE_SLOT_Y + 18.0,
                SEAL_GASKET_W,
                8.0,
                z_on_base(DOCK_Z) + DOCK_Z / 2.0 + 4.0,
            )
            + latch_pair(i, x);
    }
    deck - cutouts + features + cassette_presence_pins()
}

fn gasket_frame(
    name: String,
    x: f64,
    y: f64,
    width: f64,
    depth: f64,
    rail: f64,
    height: f64,
    z: f64,
) -> Part {
    centered_cube(format!("{name}_left"), rail, depth, height).translate(x - width / 2.0, y, z)
        + centered_cube(format!("{name}_right"), rail, depth, height).translate(
            x + width / 2.0,
            y,
            z,
        )
        + centered_cube(format!("{name}_front"), width, rail, height).translate(
            x,
            y - depth / 2.0,
            z,
        )
        + centered_cube(format!("{name}_rear"), width, rail, height).translate(
            x,
            y + depth / 2.0,
            z,
        )
}

fn latch_pair(slot: usize, x: f64) -> Part {
    let mut latches = Part::empty(format!(
        "closed_post_run_fixation_rinse_custody_station_latch_pair_{slot}"
    ));
    for i in 0..LATCHES_PER_SLOT {
        let y = DOCK_POS.1 + if i == 0 { -112.0 } else { 112.0 };
        latches = latches
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_sealed_latch_{slot}_{i}"),
                64.0,
                16.0,
                18.0,
            )
            .translate(x, y, z_on_base(DOCK_Z) + DOCK_Z / 2.0 + 9.0);
    }
    latches
}

fn cassette_presence_pins() -> Part {
    let mut pins =
        Part::empty("closed_post_run_fixation_rinse_custody_station_cassette_presence_pins");
    for i in 0..CASSETTE_SLOTS {
        let x = DOCK_POS.0 + centered_index(i, CASSETTE_SLOTS, CASSETTE_PITCH_X);
        pins = pins
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_presence_pin_a_{i}"),
                4.0,
                18.0,
                20,
            )
            .translate(x - 34.0, DOCK_POS.1 + 72.0, z_on_base(DOCK_Z) + 9.0)
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_presence_pin_b_{i}"),
                4.0,
                18.0,
                20,
            )
            .translate(x + 34.0, DOCK_POS.1 + 72.0, z_on_base(DOCK_Z) + 9.0);
    }
    pins
}

fn fixative_input_port_surrogates() -> Part {
    let panel = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_fixative_port_panel",
        FIXATIVE_PANEL_X,
        FIXATIVE_PANEL_Y,
        FIXATIVE_PANEL_Z,
    )
    .translate(FIXATIVE_POS.0, FIXATIVE_POS.1, z_on_base(FIXATIVE_PANEL_Z));
    let mut cutouts = Part::empty("closed_post_run_fixation_rinse_custody_station_fixative_bores");
    let mut collars =
        Part::empty("closed_post_run_fixation_rinse_custody_station_fixative_port_collars");
    for i in 0..FIXATIVE_PORTS {
        let x = FIXATIVE_POS.0 + centered_index(i, FIXATIVE_PORTS, FIXATIVE_PORT_PITCH_X);
        let y = FIXATIVE_POS.1 + if i % 2 == 0 { -28.0 } else { 28.0 };
        cutouts = cutouts
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_fixative_input_bore_{i}"),
                FIXATIVE_PORT_D / 2.0,
                FIXATIVE_PANEL_Z + 8.0,
                32,
            )
            .translate(x, y, z_on_base(FIXATIVE_PANEL_Z));
        collars = collars
            + centered_cylinder(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_fixative_collared_port_{i}"
                ),
                16.0,
                12.0,
                36,
            )
            .translate(
                x,
                y,
                z_on_base(FIXATIVE_PANEL_Z) + FIXATIVE_PANEL_Z / 2.0 + 6.0,
            )
            + centered_cylinder(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_fixative_vent_surrogate_{i}"
                ),
                VENT_SURROGATE_D / 2.0,
                12.0,
                24,
            )
            .translate(
                x,
                y + 44.0,
                z_on_base(FIXATIVE_PANEL_Z) + FIXATIVE_PANEL_Z / 2.0 + 6.0,
            );
    }
    panel - cutouts + collars + keyed_fixative_orientation_rails()
}

fn keyed_fixative_orientation_rails() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_fixative_keyed_clean_rail",
        FIXATIVE_PANEL_X - 36.0,
        8.0,
        16.0,
    )
    .translate(
        FIXATIVE_POS.0,
        FIXATIVE_POS.1 - FIXATIVE_PANEL_Y / 2.0 + 24.0,
        z_on_base(FIXATIVE_PANEL_Z) + FIXATIVE_PANEL_Z / 2.0 + 8.0,
    ) + centered_cube(
        "closed_post_run_fixation_rinse_custody_station_fixative_keyed_used_rail",
        FIXATIVE_PANEL_X - 90.0,
        8.0,
        16.0,
    )
    .translate(
        FIXATIVE_POS.0,
        FIXATIVE_POS.1 + FIXATIVE_PANEL_Y / 2.0 - 24.0,
        z_on_base(FIXATIVE_PANEL_Z) + FIXATIVE_PANEL_Z / 2.0 + 8.0,
    )
}

fn rinse_waste_route_manifold() -> Part {
    let body = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_rinse_waste_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(MANIFOLD_POS.0, MANIFOLD_POS.1, z_on_base(MANIFOLD_Z));
    let mut bores = Part::empty("closed_post_run_fixation_rinse_custody_station_manifold_bores");
    let mut bosses =
        Part::empty("closed_post_run_fixation_rinse_custody_station_manifold_route_bosses");
    for i in 0..RINSE_BRANCHES {
        let x = MANIFOLD_POS.0 + centered_index(i, RINSE_BRANCHES, ROUTE_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_rinse_branch_bore_{i}"),
                RINSE_BORE_D / 2.0,
                MANIFOLD_Z + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, MANIFOLD_POS.1 - 24.0, z_on_base(MANIFOLD_Z));
        bores = bores
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_waste_branch_bore_{i}"),
                WASTE_BORE_D / 2.0,
                MANIFOLD_Z + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, MANIFOLD_POS.1 + 36.0, z_on_base(MANIFOLD_Z));
        bosses = bosses
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_rinse_branch_boss_{i}"),
                14.0,
                16.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                MANIFOLD_POS.1 - MANIFOLD_Y / 2.0 - 8.0,
                z_on_base(MANIFOLD_Z),
            )
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_waste_branch_boss_{i}"),
                18.0,
                18.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                MANIFOLD_POS.1 + MANIFOLD_Y / 2.0 + 9.0,
                z_on_base(MANIFOLD_Z),
            );
    }
    body - bores + bosses + waste_trough() + flow_direction_keys()
}

fn waste_trough() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_waste_overflow_trough",
        WASTE_TROUGH_X,
        WASTE_TROUGH_Y,
        WASTE_TROUGH_Z,
    )
    .translate(
        MANIFOLD_POS.0,
        MANIFOLD_POS.1 + MANIFOLD_Y / 2.0 + 38.0,
        z_on_base(WASTE_TROUGH_Z),
    )
}

fn flow_direction_keys() -> Part {
    let mut keys = Part::empty("closed_post_run_fixation_rinse_custody_station_flow_keys");
    for i in 0..RINSE_BRANCHES {
        let x = MANIFOLD_POS.0 + centered_index(i, RINSE_BRANCHES, ROUTE_PITCH_X);
        keys = keys
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_rinse_arrow_key_{i}"),
                46.0,
                8.0,
                8.0,
            )
            .translate(x, MANIFOLD_POS.1 - 70.0, z_on_base(MANIFOLD_Z) + 32.0)
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_waste_arrow_key_{i}"),
                46.0,
                8.0,
                8.0,
            )
            .translate(x, MANIFOLD_POS.1 + 72.0, z_on_base(MANIFOLD_Z) + 32.0);
    }
    keys
}

fn timed_dwell_token_lanes() -> Part {
    let bank = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_dwell_lane_bank",
        DWELL_BANK_X,
        DWELL_BANK_Y,
        DWELL_BANK_Z,
    )
    .translate(DWELL_POS.0, DWELL_POS.1, z_on_base(DWELL_BANK_Z));
    let mut pockets =
        Part::empty("closed_post_run_fixation_rinse_custody_station_dwell_token_pockets");
    let mut rails = Part::empty("closed_post_run_fixation_rinse_custody_station_dwell_lane_rails");
    for lane in 0..DWELL_LANES {
        let x = DWELL_POS.0 + centered_index(lane, DWELL_LANES, DWELL_LANE_PITCH_X);
        rails = rails
            + centered_cube(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_dwell_lane_rail_{lane}_left"
                ),
                6.0,
                DWELL_BANK_Y - 18.0,
                18.0,
            )
            .translate(
                x - DWELL_LANE_W / 2.0,
                DWELL_POS.1,
                z_on_base(DWELL_BANK_Z) + 18.0,
            )
            + centered_cube(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_dwell_lane_rail_{lane}_right"
                ),
                6.0,
                DWELL_BANK_Y - 18.0,
                18.0,
            )
            .translate(
                x + DWELL_LANE_W / 2.0,
                DWELL_POS.1,
                z_on_base(DWELL_BANK_Z) + 18.0,
            );
        for token in 0..DWELL_TOKENS_PER_LANE {
            let y = DWELL_POS.1 + centered_index(token, DWELL_TOKENS_PER_LANE, DWELL_TOKEN_PITCH_Y);
            pockets = pockets
                + centered_cylinder(
                    format!("closed_post_run_fixation_rinse_custody_station_dwell_token_pocket_{lane}_{token}"),
                    DWELL_TOKEN_D / 2.0,
                    12.0,
                    32,
                )
                .translate(x, y, z_on_base(DWELL_BANK_Z) + 10.0);
        }
    }
    bank - pockets + rails + dwell_time_index_blocks()
}

fn dwell_time_index_blocks() -> Part {
    let mut blocks =
        Part::empty("closed_post_run_fixation_rinse_custody_station_dwell_time_index_blocks");
    for i in 0..DWELL_LANES {
        let x = DWELL_POS.0 + centered_index(i, DWELL_LANES, DWELL_LANE_PITCH_X);
        blocks = blocks
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_dwell_time_index_{i}"),
                52.0,
                16.0,
                14.0,
            )
            .translate(
                x,
                DWELL_POS.1 + DWELL_BANK_Y / 2.0 - 18.0,
                z_on_base(DWELL_BANK_Z) + 20.0,
            );
    }
    blocks
}

fn sample_archive_handoff_nests() -> Part {
    let bank = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_archive_handoff_bank",
        ARCHIVE_BANK_X,
        ARCHIVE_BANK_Y,
        ARCHIVE_BANK_Z,
    )
    .translate(ARCHIVE_POS.0, ARCHIVE_POS.1, z_on_base(ARCHIVE_BANK_Z));
    let mut pockets =
        Part::empty("closed_post_run_fixation_rinse_custody_station_archive_nest_pockets");
    let mut rings =
        Part::empty("closed_post_run_fixation_rinse_custody_station_archive_seal_rings");
    for i in 0..ARCHIVE_NESTS {
        let x = ARCHIVE_POS.0 + centered_index(i, ARCHIVE_NESTS, ARCHIVE_NEST_PITCH_X);
        pockets = pockets
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_archive_vial_pocket_{i}"),
                ARCHIVE_NEST_D / 2.0,
                ARCHIVE_BANK_Z + 8.0,
                40,
            )
            .translate(x, ARCHIVE_POS.1, z_on_base(ARCHIVE_BANK_Z) + 4.0);
        rings = rings
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_archive_tamper_ring_{i}"),
                ARCHIVE_SEAL_RING_D / 2.0,
                6.0,
                40,
            )
            .translate(x, ARCHIVE_POS.1, z_on_base(ARCHIVE_BANK_Z) + ARCHIVE_BANK_Z / 2.0 + 3.0)
            - centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_archive_tamper_ring_opening_{i}"),
                ARCHIVE_NEST_D / 2.0 + 3.0,
                8.0,
                40,
            )
            .translate(x, ARCHIVE_POS.1, z_on_base(ARCHIVE_BANK_Z) + ARCHIVE_BANK_Z / 2.0 + 3.0);
    }
    bank - pockets + rings + archive_robot_grip_clearances()
}

fn archive_robot_grip_clearances() -> Part {
    let mut clearances =
        Part::empty("closed_post_run_fixation_rinse_custody_station_archive_grip_clearances");
    for i in 0..ARCHIVE_NESTS {
        let x = ARCHIVE_POS.0 + centered_index(i, ARCHIVE_NESTS, ARCHIVE_NEST_PITCH_X);
        clearances = clearances
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_archive_grip_slot_{i}"),
                12.0,
                ARCHIVE_BANK_Y - 34.0,
                10.0,
            )
            .translate(
                x,
                ARCHIVE_POS.1,
                z_on_base(ARCHIVE_BANK_Z) + ARCHIVE_BANK_Z / 2.0 + 5.0,
            );
    }
    clearances
}

fn barcode_status_lands() -> Part {
    let panel = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, z_on_base(TRACE_PANEL_Z));
    let mut lands = Part::empty("closed_post_run_fixation_rinse_custody_station_trace_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        let x = TRACE_POS.0 + centered_index(col, 4, 68.0);
        let y = TRACE_POS.1 + if row == 0 { -24.0 } else { 18.0 };
        lands = lands
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, z_on_base(TRACE_PANEL_Z) + TRACE_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..STATUS_LANDS {
        let x = TRACE_POS.0 + centered_index(i, STATUS_LANDS, 58.0);
        lands = lands
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_status_indicator_land_{i}"),
                STATUS_LAND_D / 2.0,
                3.0,
                30,
            )
            .translate(
                x,
                TRACE_POS.1 + 48.0,
                z_on_base(TRACE_PANEL_Z) + TRACE_PANEL_Z / 2.0 + 1.5,
            );
    }
    panel + lands
}

fn chemical_compatibility_coupon_pockets() -> Part {
    let bank = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_compatibility_coupon_bank",
        COUPON_BANK_X,
        COUPON_BANK_Y,
        COUPON_BANK_Z,
    )
    .translate(COUPON_POS.0, COUPON_POS.1, z_on_base(COUPON_BANK_Z));
    let mut pockets =
        Part::empty("closed_post_run_fixation_rinse_custody_station_coupon_pocket_cutouts");
    let mut labels =
        Part::empty("closed_post_run_fixation_rinse_custody_station_coupon_label_tabs");
    for i in 0..COUPON_POCKETS {
        let row = i / 4;
        let col = i % 4;
        let x = COUPON_POS.0 + centered_index(col, 4, COUPON_PITCH_X);
        let y = COUPON_POS.1 + if row == 0 { -38.0 } else { 38.0 };
        pockets = pockets
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_coupon_pocket_{i}"),
                COUPON_POCKET_X,
                COUPON_POCKET_Y,
                COUPON_POCKET_Z,
            )
            .translate(x, y, z_on_base(COUPON_BANK_Z) + 5.0);
        labels = labels
            + centered_cube(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_coupon_material_label_land_{i}"
                ),
                COUPON_POCKET_X,
                8.0,
                4.0,
            )
            .translate(
                x,
                y + 36.0,
                z_on_base(COUPON_BANK_Z) + COUPON_BANK_Z / 2.0 + 2.0,
            );
    }
    bank - pockets + labels
}

fn clean_used_segregation_barriers() -> Part {
    let clean_used_center_barrier = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_clean_used_center_barrier",
        12.0,
        690.0,
        CLEAN_USED_BARRIER_Z,
    )
    .translate(-40.0, -30.0, z_on_base(CLEAN_USED_BARRIER_Z));
    let dock_to_trace_barrier = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_clean_trace_barrier",
        450.0,
        10.0,
        34.0,
    )
    .translate(-410.0, 260.0, z_on_base(34.0));
    let manifold_waste_barrier = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_used_waste_barrier",
        620.0,
        12.0,
        42.0,
    )
    .translate(270.0, -190.0, z_on_base(42.0));
    let archive_custody_gate = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_archive_custody_gate",
        450.0,
        12.0,
        52.0,
    )
    .translate(260.0, -410.0, z_on_base(52.0));

    clean_used_center_barrier
        + dock_to_trace_barrier
        + manifold_waste_barrier
        + archive_custody_gate
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_evidence_bridge_left_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_POS.1,
        z_on_base(EVIDENCE_POST_Z),
    );
    let right_post = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_evidence_bridge_right_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_POS.1,
        z_on_base(EVIDENCE_POST_Z),
    );
    let beam = centered_cube(
        "closed_post_run_fixation_rinse_custody_station_evidence_bridge_camera_beam",
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    left_post + right_post + beam + evidence_cameras() + evidence_light_bars()
}

fn evidence_cameras() -> Part {
    let mut cameras =
        Part::empty("closed_post_run_fixation_rinse_custody_station_evidence_cameras");
    for i in 0..EVIDENCE_CAMERAS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_CAMERAS, 270.0);
        cameras = cameras
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_camera_block_{i}"),
                48.0,
                36.0,
                26.0,
            )
            .translate(x, EVIDENCE_POS.1, BASE_Z / 2.0 + EVIDENCE_POST_Z - 18.0)
            + centered_cylinder(
                format!("closed_post_run_fixation_rinse_custody_station_camera_lens_{i}"),
                12.0,
                12.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                EVIDENCE_POS.1 - 24.0,
                BASE_Z / 2.0 + EVIDENCE_POST_Z - 18.0,
            );
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut lights =
        Part::empty("closed_post_run_fixation_rinse_custody_station_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BARS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_LIGHT_BARS, 250.0);
        lights = lights
            + centered_cube(
                format!("closed_post_run_fixation_rinse_custody_station_light_bar_{i}"),
                120.0,
                12.0,
                12.0,
            )
            .translate(
                x,
                EVIDENCE_POS.1 + 32.0,
                BASE_Z / 2.0 + EVIDENCE_POST_Z - 48.0,
            );
    }
    lights
}

fn robot_service_keepouts() -> Part {
    let mut windows =
        Part::empty("closed_post_run_fixation_rinse_custody_station_robot_keepout_windows");
    for i in 0..ROBOT_KEEP_OUT_WINDOWS {
        let x = centered_index(i, ROBOT_KEEP_OUT_WINDOWS, 230.0);
        windows = windows
            + centered_cube(
                format!(
                    "closed_post_run_fixation_rinse_custody_station_robot_swept_volume_window_{i}"
                ),
                120.0,
                16.0,
                ROBOT_KEEP_OUT_Z,
            )
            .translate(x, 20.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);
    }
    windows
        + front_service_gauge()
        + rear_service_gauge()
        + left_clean_service_gauge()
        + right_waste_service_gauge()
}

fn front_service_gauge() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_front_service_clearance_gauge",
        STATION_X - 180.0,
        12.0,
        30.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z / 2.0 + 15.0,
    )
}

fn rear_service_gauge() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_rear_service_clearance_gauge",
        STATION_X - 220.0,
        12.0,
        30.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z / 2.0 + 15.0,
    )
}

fn left_clean_service_gauge() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_left_clean_service_clearance_gauge",
        10.0,
        430.0,
        28.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CLEAN_SERVICE_CLEARANCE / 2.0,
        110.0,
        BASE_Z / 2.0 + 14.0,
    )
}

fn right_waste_service_gauge() -> Part {
    centered_cube(
        "closed_post_run_fixation_rinse_custody_station_right_waste_service_clearance_gauge",
        10.0,
        470.0,
        34.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_WASTE_SERVICE_CLEARANCE / 2.0,
        -145.0,
        BASE_Z / 2.0 + 17.0,
    )
}

fn closed_route_placeholders() -> Part {
    let dock_to_fixative = route_bar(
        "closed_post_run_fixation_rinse_custody_station_dock_to_fixative_route_placeholder",
        366.0,
        8.0,
        -2.0,
        142.0,
    );
    let fixative_to_manifold = route_bar(
        "closed_post_run_fixation_rinse_custody_station_fixative_to_manifold_route_placeholder",
        8.0,
        120.0,
        390.0,
        38.0,
    );
    let dock_to_dwell = route_bar(
        "closed_post_run_fixation_rinse_custody_station_dock_to_dwell_token_route_placeholder",
        8.0,
        148.0,
        -360.0,
        20.0,
    );
    let manifold_to_archive = route_bar(
        "closed_post_run_fixation_rinse_custody_station_manifold_to_archive_closed_handoff_route_placeholder",
        8.0,
        142.0,
        260.0,
        -188.0,
    );
    let dwell_to_coupons = route_bar(
        "closed_post_run_fixation_rinse_custody_station_dwell_to_coupon_witness_route_placeholder",
        8.0,
        116.0,
        -365.0,
        -212.0,
    );
    dock_to_fixative + fixative_to_manifold + dock_to_dwell + manifold_to_archive + dwell_to_coupons
}

fn route_bar(name: &str, width: f64, depth: f64, x: f64, y: f64) -> Part {
    centered_cube(name, width, depth, 6.0).translate(x, y, BASE_Z / 2.0 + 6.0)
}

fn z_on_base(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn rects_overlap(left: (f64, f64, f64, f64), right: (f64, f64, f64, f64), margin: f64) -> bool {
    left.0 < right.1 + margin
        && left.1 + margin > right.0
        && left.2 < right.3 + margin
        && left.3 + margin > right.2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_set_is_complete_and_deterministic() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_cassette_post_run_fixation_rinse_custody_station_")));
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            "output/closed_cassette_post_run_fixation_rinse_custody_station_assembly.stl"
        );
    }

    #[test]
    fn required_feature_groups_cover_ticket_scope() {
        for feature in [
            "sealed_cassette_dock",
            "fixative_input_port_surrogates",
            "rinse_waste_route_manifold",
            "timed_dwell_token_lanes",
            "leak_overflow_tray",
            "sample_archive_handoff_nests",
            "barcode_status_lands",
            "chemical_compatibility_coupon_pockets",
            "clean_used_segregation",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn closed_route_counts_match_cassette_slots() {
        assert_eq!(CASSETTE_SLOTS, DWELL_LANES);
        assert_eq!(CASSETTE_SLOTS, RINSE_BRANCHES);
        assert_eq!(CASSETTE_SLOTS, WASTE_BRANCHES);
        assert!(ARCHIVE_NESTS >= CASSETTE_SLOTS);
    }

    #[test]
    fn layout_assertions_hold() {
        assert_layout();
    }
}
