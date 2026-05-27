use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module gasket lot incoming inspection and compression-set station.
//
// Intent:
// - Package incoming gasket lots, traceability evidence, bought measurement
//   tools, imaging, status disposition, retain samples, and leak-containment
//   interfaces as a station-level CAD concept.
// - Represent interfaces around purchased thickness/compression gauges, barcode
//   readers, RFID pads, defect imaging, and COA custody without modeling final
//   seal-qualification internals.
// - Keep released, hold, reject, clean, used, robot, and service zones visibly
//   separated for closed-module gasket receiving workflow planning.
//
// Research assumptions used for placeholder geometry:
// - Incoming elastomer gasket inspection commonly checks lot identity,
//   certificate evidence, visual defects, thickness, compression-set evidence,
//   and retained samples before release to production.
// - Digital thickness gauges, benchtop compression-test/comparator fixtures,
//   barcode/RFID readers, and machine-vision bridges are modeled here as
//   envelope placeholders plus mechanical datums, not as metrology internals.

const OUTPUTS: &[&str] = &[
    "output/closed_gasket_lot_incoming_inspection_station_base_leak_tray.stl",
    "output/closed_gasket_lot_incoming_inspection_station_gasket_lot_trays.stl",
    "output/closed_gasket_lot_incoming_inspection_station_thickness_gauge_pockets.stl",
    "output/closed_gasket_lot_incoming_inspection_station_compression_gauge_pockets.stl",
    "output/closed_gasket_lot_incoming_inspection_station_defect_imaging_bridge.stl",
    "output/closed_gasket_lot_incoming_inspection_station_barcode_rfid_coa_lands.stl",
    "output/closed_gasket_lot_incoming_inspection_station_release_hold_reject_lanes.stl",
    "output/closed_gasket_lot_incoming_inspection_station_sample_retain_pockets.stl",
    "output/closed_gasket_lot_incoming_inspection_station_clean_used_segregation.stl",
    "output/closed_gasket_lot_incoming_inspection_station_robot_service_keepouts.stl",
    "output/closed_gasket_lot_incoming_inspection_station_tooling_datums.stl",
    "output/closed_gasket_lot_incoming_inspection_station_assembly.stl",
];

const BASE_X: f64 = 1260.0;
const BASE_Y: f64 = 840.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;

const LOT_TRAY_COUNT: usize = 6;
const LOT_TRAY_COLS: usize = 3;
const LOT_TRAY_ROWS: usize = 2;
const LOT_TRAY_X: f64 = 390.0;
const LOT_TRAY_Y: f64 = 278.0;
const LOT_TRAY_Z: f64 = 44.0;
const LOT_CENTER_X: f64 = -392.0;
const LOT_CENTER_Y: f64 = 134.0;
const GASKET_NEST_D: f64 = 62.0;
const LOT_PITCH_X: f64 = 116.0;
const LOT_PITCH_Y: f64 = 106.0;

const THICKNESS_BLOCK_X: f64 = 286.0;
const THICKNESS_BLOCK_Y: f64 = 220.0;
const THICKNESS_BLOCK_Z: f64 = 48.0;
const THICKNESS_CENTER_X: f64 = -52.0;
const THICKNESS_CENTER_Y: f64 = 144.0;
const THICKNESS_GAUGE_ENV_X: f64 = 142.0;
const THICKNESS_GAUGE_ENV_Y: f64 = 92.0;
const THICKNESS_GAUGE_ENV_Z: f64 = 210.0;
const THICKNESS_ANVIL_D: f64 = 42.0;
const THICKNESS_REFERENCE_SHIMS: usize = 8;

const COMPRESSION_BLOCK_X: f64 = 314.0;
const COMPRESSION_BLOCK_Y: f64 = 232.0;
const COMPRESSION_BLOCK_Z: f64 = 54.0;
const COMPRESSION_CENTER_X: f64 = 302.0;
const COMPRESSION_CENTER_Y: f64 = 142.0;
const COMPRESSION_GAUGE_ENV_X: f64 = 186.0;
const COMPRESSION_GAUGE_ENV_Y: f64 = 136.0;
const COMPRESSION_GAUGE_ENV_Z: f64 = 245.0;
const COMPRESSION_PLATEN_D: f64 = 78.0;
const COMPRESSION_SPACER_COUNT: usize = 6;

const IMAGING_WINDOW_X: f64 = 720.0;
const IMAGING_WINDOW_Y: f64 = 240.0;
const IMAGING_CENTER_X: f64 = -72.0;
const IMAGING_CENTER_Y: f64 = -60.0;
const CAMERA_BRIDGE_SPAN_X: f64 = 820.0;
const CAMERA_BRIDGE_POST_Y: f64 = 42.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 196.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 30.0;
const DEFECT_REFERENCE_COUPONS: usize = 10;

const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 6;
const COA_LANDS: usize = 4;
const LABEL_LAND_X: f64 = 96.0;
const LABEL_LAND_Y: f64 = 28.0;
const LABEL_LAND_Z: f64 = 5.0;
const TRACEABILITY_CENTER_X: f64 = -404.0;
const TRACEABILITY_CENTER_Y: f64 = -284.0;

const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 5;
const STATUS_X: f64 = 356.0;
const STATUS_Y: f64 = 240.0;
const STATUS_Z: f64 = 46.0;
const STATUS_CENTER_X: f64 = 396.0;
const STATUS_CENTER_Y: f64 = -244.0;
const STATUS_LANE_PITCH: f64 = 76.0;

const RETAIN_BLOCK_X: f64 = 310.0;
const RETAIN_BLOCK_Y: f64 = 188.0;
const RETAIN_BLOCK_Z: f64 = 48.0;
const RETAIN_CENTER_X: f64 = -62.0;
const RETAIN_CENTER_Y: f64 = -284.0;
const RETAIN_COLS: usize = 5;
const RETAIN_ROWS: usize = 3;
const RETAIN_POCKETS: usize = RETAIN_COLS * RETAIN_ROWS;
const RETAIN_PITCH_X: f64 = 48.0;
const RETAIN_PITCH_Y: f64 = 44.0;

const SEGREGATION_SPINE_X: f64 = 1088.0;
const SEGREGATION_SPINE_Y: f64 = 28.0;
const SEGREGATION_SPINE_Z: f64 = 72.0;
const CLEAN_LANE_X: f64 = 470.0;
const USED_LANE_X: f64 = 378.0;
const REJECT_BIN_X: f64 = 202.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 118.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 104.0;
const LEFT_LOT_CART_KEEP_OUT_X: f64 = 112.0;
const RIGHT_GAUGE_SERVICE_KEEP_OUT_X: f64 = 132.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 168.0;
const GAUGE_SERVICE_CLEARANCE_Z: f64 = 292.0;

const TOOL_DATUM_PINS: usize = 8;
const TOOLING_RAILS: usize = 4;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let lot_trays = gasket_lot_trays();
    export(&lot_trays, OUTPUTS[1]);

    let thickness = thickness_gauge_pockets();
    export(&thickness, OUTPUTS[2]);

    let compression = compression_gauge_pockets();
    export(&compression, OUTPUTS[3]);

    let imaging = defect_imaging_bridge();
    export(&imaging, OUTPUTS[4]);

    let traceability = barcode_rfid_coa_lands();
    export(&traceability, OUTPUTS[5]);

    let status = release_hold_reject_lanes();
    export(&status, OUTPUTS[6]);

    let retain = sample_retain_pockets();
    export(&retain, OUTPUTS[7]);

    let segregation = clean_used_segregation();
    export(&segregation, OUTPUTS[8]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[9]);

    let datums = tooling_datums();
    export(&datums, OUTPUTS[10]);

    let assembly = base
        + lot_trays.translate(LOT_CENTER_X, LOT_CENTER_Y, DECK_Z / 2.0)
        + thickness.translate(THICKNESS_CENTER_X, THICKNESS_CENTER_Y, DECK_Z / 2.0)
        + compression.translate(COMPRESSION_CENTER_X, COMPRESSION_CENTER_Y, DECK_Z / 2.0)
        + imaging
        + traceability.translate(TRACEABILITY_CENTER_X, TRACEABILITY_CENTER_Y, DECK_Z / 2.0)
        + status.translate(STATUS_CENTER_X, STATUS_CENTER_Y, DECK_Z / 2.0)
        + retain.translate(RETAIN_CENTER_X, RETAIN_CENTER_Y, DECK_Z / 2.0)
        + segregation
        + keepouts
        + datums;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed gasket incoming station: {:.0} x {:.0}mm leak-tray deck, {} lot tray nests, {:.0} x {:.0} x {:.0}mm thickness gauge envelope, {:.0} x {:.0} x {:.0}mm compression gauge envelope, {} defect reference coupons, {} retain pockets.",
        BASE_X,
        BASE_Y,
        LOT_TRAY_COUNT,
        THICKNESS_GAUGE_ENV_X,
        THICKNESS_GAUGE_ENV_Y,
        THICKNESS_GAUGE_ENV_Z,
        COMPRESSION_GAUGE_ENV_X,
        COMPRESSION_GAUGE_ENV_Y,
        COMPRESSION_GAUGE_ENV_Z,
        DEFECT_REFERENCE_COUPONS,
        RETAIN_POCKETS
    );
    println!(
        "Traceability and disposition: {} barcode lands, {} RFID lands, {} COA lands, {} release/hold/reject lanes with {} slots each.",
        BARCODE_LANDS,
        RFID_LANDS,
        COA_LANDS,
        STATUS_LANES,
        STATUS_SLOTS_PER_LANE
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    for rect in [
        lot_rect(),
        thickness_rect(),
        compression_rect(),
        imaging_rect(),
        traceability_rect(),
        status_rect(),
        retain_rect(),
    ] {
        assert!(
            rect.fits_inside(BASE_X, BASE_Y),
            "{} exceeds closed gasket inspection deck footprint",
            rect.name
        );
    }

    assert!(!lot_rect().overlaps(thickness_rect()));
    assert!(!thickness_rect().overlaps(compression_rect()));
    assert!(!traceability_rect().overlaps(retain_rect()));
    assert!(!retain_rect().overlaps(status_rect()));
    assert!(CAMERA_BRIDGE_UNDERSIDE_Z > LOT_TRAY_Z + 120.0);
    assert!(ROBOT_PICK_CLEARANCE_Z > STATUS_Z + 90.0);
    assert!(GAUGE_SERVICE_CLEARANCE_Z > COMPRESSION_GAUGE_ENV_Z + 30.0);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("closed_gasket_station_base_pan", BASE_X, BASE_Y, DECK_Z);
    let recessed_basin = centered_cube(
        "closed_gasket_station_recessed_leak_basin",
        BASE_X - 2.0 * (RIM_W + 42.0),
        BASE_Y - 2.0 * (RIM_W + 44.0),
        9.0,
    )
    .translate(0.0, -10.0, DECK_Z / 2.0 - 3.0);
    let front_sump = centered_cube(
        "closed_gasket_station_front_leak_sump",
        450.0,
        54.0,
        DECK_Z + 4.0,
    )
    .translate(96.0, -BASE_Y / 2.0 + 60.0, 0.0);
    let drain = centered_cylinder("closed_gasket_station_leak_tray_drain", 7.0 / 2.0, 46.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(438.0, -BASE_Y / 2.0 + 24.0, -2.0);

    deck - recessed_basin - front_sump - drain - deck_mount_holes()
        + base_rim()
        + zone_floor_lands()
}

fn base_rim() -> Part {
    let front = centered_cube("closed_gasket_station_front_leak_lip", BASE_X, RIM_W, RIM_Z)
        .translate(0.0, -BASE_Y / 2.0 + RIM_W / 2.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_gasket_station_rear_service_lip",
        BASE_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, BASE_Y / 2.0 - RIM_W / 2.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_gasket_station_left_lot_cart_lip",
        RIM_W,
        BASE_Y,
        RIM_Z,
    )
    .translate(-BASE_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_gasket_station_right_gauge_lip",
        RIM_W,
        BASE_Y,
        RIM_Z,
    )
    .translate(BASE_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    front + rear + left + right
}

fn zone_floor_lands() -> Part {
    let lot_land = centered_cube(
        "closed_gasket_station_lot_receiving_zone_land",
        348.0,
        6.0,
        5.0,
    )
    .translate(
        LOT_CENTER_X,
        LOT_CENTER_Y - LOT_TRAY_Y / 2.0 - 18.0,
        DECK_Z / 2.0 + 2.5,
    );
    let measure_land = centered_cube(
        "closed_gasket_station_measurement_flow_land",
        640.0,
        6.0,
        5.0,
    )
    .translate(116.0, 2.0, DECK_Z / 2.0 + 2.5);
    let disposition_land = centered_cube(
        "closed_gasket_station_disposition_flow_land",
        384.0,
        6.0,
        5.0,
    )
    .rotate(0.0, 0.0, -22.0)
    .translate(268.0, -142.0, DECK_Z / 2.0 + 2.5);
    lot_land + measure_land + disposition_land
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_gasket_station_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("closed_gasket_station_m6_clearance_{i}"),
            6.8 / 2.0,
            DECK_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_gasket_station_mount_slot_relief_{i}"),
            26.0,
            7.2,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + hole + slot;
    }
    holes
}

fn gasket_lot_trays() -> Part {
    let tray = centered_cube(
        "closed_gasket_station_lot_tray_block",
        LOT_TRAY_X,
        LOT_TRAY_Y,
        LOT_TRAY_Z,
    )
    .translate(0.0, 0.0, LOT_TRAY_Z / 2.0);
    let basin = centered_cube(
        "closed_gasket_station_lot_tray_nested_basin",
        LOT_TRAY_X - 34.0,
        LOT_TRAY_Y - 30.0,
        12.0,
    )
    .translate(0.0, 0.0, LOT_TRAY_Z - 5.0);
    let front_robot_access = centered_cube(
        "closed_gasket_station_lot_tray_front_robot_access",
        LOT_TRAY_X - 72.0,
        30.0,
        LOT_TRAY_Z + 4.0,
    )
    .translate(0.0, -LOT_TRAY_Y / 2.0 + 16.0, LOT_TRAY_Z / 2.0);

    tray - basin - front_robot_access - gasket_lot_pocket_cuts()
        + lot_tray_dividers()
        + lot_tray_label_flags()
        + lot_tray_cover_latches()
}

fn gasket_lot_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_gasket_station_lot_pocket_cuts");
    for i in 0..LOT_TRAY_COUNT {
        let (x, y) = lot_position(i);
        let nest = centered_cylinder(
            format!("closed_gasket_station_gasket_lot_nest_{i}"),
            GASKET_NEST_D / 2.0,
            LOT_TRAY_Z + 4.0,
            48,
        )
        .translate(x, y, LOT_TRAY_Z / 2.0 + 3.0);
        let package_corner = centered_cube(
            format!("closed_gasket_station_packaging_corner_relief_{i}"),
            78.0,
            26.0,
            LOT_TRAY_Z + 4.0,
        )
        .translate(x, y - 34.0, LOT_TRAY_Z / 2.0 + 3.0);
        cuts = cuts + nest + package_corner;
    }
    cuts
}

fn lot_tray_dividers() -> Part {
    let vertical_a = centered_cube(
        "closed_gasket_station_lot_tray_vertical_divider_a",
        8.0,
        LOT_TRAY_Y - 46.0,
        24.0,
    )
    .translate(-LOT_PITCH_X / 2.0, 0.0, LOT_TRAY_Z + 12.0);
    let vertical_b = centered_cube(
        "closed_gasket_station_lot_tray_vertical_divider_b",
        8.0,
        LOT_TRAY_Y - 46.0,
        24.0,
    )
    .translate(LOT_PITCH_X / 2.0, 0.0, LOT_TRAY_Z + 12.0);
    let horizontal = centered_cube(
        "closed_gasket_station_lot_tray_horizontal_divider",
        LOT_TRAY_X - 52.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, LOT_TRAY_Z + 12.0);
    vertical_a + vertical_b + horizontal
}

fn lot_tray_label_flags() -> Part {
    let mut flags = Part::empty("closed_gasket_station_lot_label_flags");
    for i in 0..LOT_TRAY_COUNT {
        let (x, y) = lot_position(i);
        let label = centered_cube(
            format!("closed_gasket_station_lot_label_land_{i}"),
            70.0,
            18.0,
            5.0,
        )
        .translate(x, y + 42.0, LOT_TRAY_Z + 2.5);
        let rfid = centered_cylinder(
            format!("closed_gasket_station_lot_rfid_disc_land_{i}"),
            7.0,
            4.0,
            24,
        )
        .translate(x - 45.0, y + 42.0, LOT_TRAY_Z + 2.0);
        flags = flags + label + rfid;
    }
    flags
}

fn lot_tray_cover_latches() -> Part {
    let rear = centered_cube(
        "closed_gasket_station_lot_tray_rear_lid_latch_rail",
        LOT_TRAY_X - 58.0,
        10.0,
        20.0,
    )
    .translate(0.0, LOT_TRAY_Y / 2.0 - 18.0, LOT_TRAY_Z + 10.0);
    let front = centered_cube(
        "closed_gasket_station_lot_tray_front_lid_latch_rail",
        LOT_TRAY_X - 112.0,
        8.0,
        18.0,
    )
    .translate(0.0, -LOT_TRAY_Y / 2.0 + 42.0, LOT_TRAY_Z + 9.0);
    rear + front
}

fn thickness_gauge_pockets() -> Part {
    let dock = centered_cube(
        "closed_gasket_station_thickness_gauge_dock",
        THICKNESS_BLOCK_X,
        THICKNESS_BLOCK_Y,
        THICKNESS_BLOCK_Z,
    )
    .translate(0.0, 0.0, THICKNESS_BLOCK_Z / 2.0);
    let gauge_foot = centered_cube(
        "closed_gasket_station_thickness_gauge_foot_recess",
        THICKNESS_GAUGE_ENV_X + 26.0,
        THICKNESS_GAUGE_ENV_Y + 24.0,
        10.0,
    )
    .translate(0.0, 12.0, THICKNESS_BLOCK_Z - 4.0);
    let anvil = centered_cylinder(
        "closed_gasket_station_thickness_anvil_pocket",
        THICKNESS_ANVIL_D / 2.0,
        THICKNESS_BLOCK_Z + 4.0,
        48,
    )
    .translate(-72.0, -66.0, THICKNESS_BLOCK_Z / 2.0 + 3.0);
    let probe_slot = centered_cube(
        "closed_gasket_station_thickness_probe_access_slot",
        118.0,
        24.0,
        THICKNESS_BLOCK_Z + 4.0,
    )
    .translate(-6.0, -60.0, THICKNESS_BLOCK_Z / 2.0 + 3.0);

    dock - gauge_foot - anvil - probe_slot
        + thickness_gauge_envelope()
        + thickness_shim_magazine()
        + thickness_part_stop_comb()
}

fn thickness_gauge_envelope() -> Part {
    let body = centered_cube(
        "closed_gasket_station_thickness_gauge_envelope_placeholder",
        THICKNESS_GAUGE_ENV_X,
        THICKNESS_GAUGE_ENV_Y,
        THICKNESS_GAUGE_ENV_Z,
    )
    .translate(0.0, 12.0, THICKNESS_BLOCK_Z + THICKNESS_GAUGE_ENV_Z / 2.0);
    let display_witness = centered_cube(
        "closed_gasket_station_thickness_gauge_display_witness_cut",
        78.0,
        7.0,
        42.0,
    )
    .translate(
        0.0,
        -THICKNESS_GAUGE_ENV_Y / 2.0 + 7.0,
        THICKNESS_BLOCK_Z + 146.0,
    );
    let data_port_witness = centered_cube(
        "closed_gasket_station_thickness_gauge_data_port_witness",
        34.0,
        7.0,
        18.0,
    )
    .translate(
        52.0,
        -THICKNESS_GAUGE_ENV_Y / 2.0 + 7.0,
        THICKNESS_BLOCK_Z + 78.0,
    );
    body - display_witness - data_port_witness
}

fn thickness_shim_magazine() -> Part {
    let magazine = centered_cube(
        "closed_gasket_station_thickness_reference_shim_magazine",
        168.0,
        34.0,
        64.0,
    )
    .translate(
        50.0,
        THICKNESS_BLOCK_Y / 2.0 - 38.0,
        THICKNESS_BLOCK_Z + 32.0,
    );

    let mut slots = Part::empty("closed_gasket_station_thickness_reference_shim_slots");
    for i in 0..THICKNESS_REFERENCE_SHIMS {
        slots = slots
            + centered_cube(
                format!("closed_gasket_station_thickness_reference_shim_slot_{i}"),
                5.0,
                26.0,
                42.0,
            )
            .translate(
                -18.0 + i as f64 * 16.0,
                THICKNESS_BLOCK_Y / 2.0 - 38.0,
                THICKNESS_BLOCK_Z + 30.0,
            );
    }
    magazine - slots
}

fn thickness_part_stop_comb() -> Part {
    let fence = centered_cube(
        "closed_gasket_station_thickness_gasket_stop_comb",
        212.0,
        9.0,
        28.0,
    )
    .translate(
        -16.0,
        -THICKNESS_BLOCK_Y / 2.0 + 28.0,
        THICKNESS_BLOCK_Z + 14.0,
    );
    let mut gaps = Part::empty("closed_gasket_station_thickness_stop_comb_gaps");
    for i in 0..LOT_TRAY_COUNT {
        gaps = gaps
            + centered_cube(
                format!("closed_gasket_station_thickness_stop_comb_gap_{i}"),
                18.0,
                12.0,
                18.0,
            )
            .translate(
                -90.0 + i as f64 * 36.0,
                -THICKNESS_BLOCK_Y / 2.0 + 28.0,
                THICKNESS_BLOCK_Z + 14.0,
            );
    }
    fence - gaps
}

fn compression_gauge_pockets() -> Part {
    let dock = centered_cube(
        "closed_gasket_station_compression_gauge_dock",
        COMPRESSION_BLOCK_X,
        COMPRESSION_BLOCK_Y,
        COMPRESSION_BLOCK_Z,
    )
    .translate(0.0, 0.0, COMPRESSION_BLOCK_Z / 2.0);
    let fixture_recess = centered_cube(
        "closed_gasket_station_compression_fixture_recess",
        COMPRESSION_GAUGE_ENV_X + 34.0,
        COMPRESSION_GAUGE_ENV_Y + 34.0,
        12.0,
    )
    .translate(0.0, 8.0, COMPRESSION_BLOCK_Z - 5.0);
    let platen_socket = centered_cylinder(
        "closed_gasket_station_compression_lower_platen_socket",
        COMPRESSION_PLATEN_D / 2.0,
        COMPRESSION_BLOCK_Z + 4.0,
        56,
    )
    .translate(-54.0, -54.0, COMPRESSION_BLOCK_Z / 2.0 + 3.0);
    let release_notch = centered_cube(
        "closed_gasket_station_compression_front_release_notch",
        128.0,
        28.0,
        COMPRESSION_BLOCK_Z + 4.0,
    )
    .translate(
        -34.0,
        -COMPRESSION_BLOCK_Y / 2.0 + 18.0,
        COMPRESSION_BLOCK_Z / 2.0,
    );

    dock - fixture_recess - platen_socket - release_notch
        + compression_gauge_envelope()
        + compression_spacer_rack()
        + compression_guard_posts()
}

fn compression_gauge_envelope() -> Part {
    let body = centered_cube(
        "closed_gasket_station_compression_gauge_envelope_placeholder",
        COMPRESSION_GAUGE_ENV_X,
        COMPRESSION_GAUGE_ENV_Y,
        COMPRESSION_GAUGE_ENV_Z,
    )
    .translate(
        0.0,
        8.0,
        COMPRESSION_BLOCK_Z + COMPRESSION_GAUGE_ENV_Z / 2.0,
    );
    let front_window = centered_cube(
        "closed_gasket_station_compression_gauge_front_window_witness",
        108.0,
        8.0,
        62.0,
    )
    .translate(
        0.0,
        -COMPRESSION_GAUGE_ENV_Y / 2.0 + 4.0,
        COMPRESSION_BLOCK_Z + 132.0,
    );
    let handle_clearance = centered_cube(
        "closed_gasket_station_compression_gauge_handle_clearance_witness",
        42.0,
        10.0,
        116.0,
    )
    .translate(
        72.0,
        -COMPRESSION_GAUGE_ENV_Y / 2.0 + 3.0,
        COMPRESSION_BLOCK_Z + 156.0,
    );
    body - front_window - handle_clearance
}

fn compression_spacer_rack() -> Part {
    let rack = centered_cube(
        "closed_gasket_station_compression_spacer_rack",
        76.0,
        174.0,
        54.0,
    )
    .translate(
        COMPRESSION_BLOCK_X / 2.0 - 50.0,
        -8.0,
        COMPRESSION_BLOCK_Z + 27.0,
    );
    let mut cuts = Part::empty("closed_gasket_station_compression_spacer_slot_cuts");
    for i in 0..COMPRESSION_SPACER_COUNT {
        cuts = cuts
            + centered_cube(
                format!("closed_gasket_station_compression_spacer_slot_{i}"),
                54.0,
                7.0,
                36.0,
            )
            .translate(
                COMPRESSION_BLOCK_X / 2.0 - 50.0,
                -70.0 + i as f64 * 28.0,
                COMPRESSION_BLOCK_Z + 27.0,
            );
    }
    rack - cuts
}

fn compression_guard_posts() -> Part {
    let mut posts = Part::empty("closed_gasket_station_compression_guard_posts");
    for (i, (x, y)) in [
        (-116.0, -78.0),
        (106.0, -78.0),
        (-116.0, 92.0),
        (106.0, 92.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_gasket_station_compression_guard_post_{i}"),
                8.0,
                116.0,
                28,
            )
            .translate(*x, *y, COMPRESSION_BLOCK_Z + 58.0);
    }
    posts
}

fn defect_imaging_bridge() -> Part {
    let stage = centered_cube(
        "closed_gasket_station_defect_imaging_stage_window",
        IMAGING_WINDOW_X,
        IMAGING_WINDOW_Y,
        16.0,
    )
    .translate(IMAGING_CENTER_X, IMAGING_CENTER_Y, DECK_Z / 2.0 + 8.0);
    let glass_recess = centered_cube(
        "closed_gasket_station_defect_imaging_diffuser_recess",
        IMAGING_WINDOW_X - 58.0,
        IMAGING_WINDOW_Y - 44.0,
        8.0,
    )
    .translate(IMAGING_CENTER_X, IMAGING_CENTER_Y, DECK_Z / 2.0 + 11.0);
    let coupon_strip = defect_reference_coupon_strip();
    let bridge = camera_bridge();
    stage - glass_recess + coupon_strip + bridge
}

fn defect_reference_coupon_strip() -> Part {
    let rail = centered_cube(
        "closed_gasket_station_defect_reference_coupon_strip",
        IMAGING_WINDOW_X - 94.0,
        34.0,
        24.0,
    )
    .translate(
        IMAGING_CENTER_X,
        IMAGING_CENTER_Y + IMAGING_WINDOW_Y / 2.0 - 28.0,
        DECK_Z / 2.0 + 28.0,
    );
    let mut cuts = Part::empty("closed_gasket_station_defect_reference_coupon_cuts");
    for i in 0..DEFECT_REFERENCE_COUPONS {
        cuts = cuts
            + centered_cube(
                format!("closed_gasket_station_defect_coupon_slot_{i}"),
                42.0,
                18.0,
                20.0,
            )
            .translate(
                IMAGING_CENTER_X - 270.0 + i as f64 * 60.0,
                IMAGING_CENTER_Y + IMAGING_WINDOW_Y / 2.0 - 28.0,
                DECK_Z / 2.0 + 29.0,
            );
    }
    rail - cuts
}

fn camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_gasket_station_imaging_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        IMAGING_CENTER_X - CAMERA_BRIDGE_SPAN_X / 2.0,
        IMAGING_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "closed_gasket_station_imaging_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        IMAGING_CENTER_X + CAMERA_BRIDGE_SPAN_X / 2.0,
        IMAGING_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "closed_gasket_station_imaging_bridge_camera_beam",
        CAMERA_BRIDGE_SPAN_X + 68.0,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        IMAGING_CENTER_X,
        IMAGING_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let camera_pod = centered_cube(
        "closed_gasket_station_defect_camera_pod_envelope",
        118.0,
        72.0,
        58.0,
    )
    .translate(
        IMAGING_CENTER_X,
        IMAGING_CENTER_Y,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z + 29.0,
    );
    let light_bar = centered_cube(
        "closed_gasket_station_grazing_light_bar_placeholder",
        620.0,
        18.0,
        18.0,
    )
    .translate(
        IMAGING_CENTER_X,
        IMAGING_CENTER_Y - IMAGING_WINDOW_Y / 2.0 + 26.0,
        DECK_Z / 2.0 + 54.0,
    );
    left_post + right_post + beam + camera_pod + light_bar
}

fn barcode_rfid_coa_lands() -> Part {
    let mut lands = Part::empty("closed_gasket_station_traceability_lands");
    for i in 0..BARCODE_LANDS {
        let x = -156.0 + (i % 2) as f64 * 152.0;
        let y = 82.0 - (i / 2) as f64 * 32.0;
        let label = centered_cube(
            format!("closed_gasket_station_barcode_land_{i}"),
            LABEL_LAND_X,
            LABEL_LAND_Y,
            LABEL_LAND_Z,
        )
        .translate(x, y, LABEL_LAND_Z / 2.0);
        let fiducial = centered_cylinder(
            format!("closed_gasket_station_barcode_scan_fiducial_{i}"),
            4.0,
            4.0,
            18,
        )
        .translate(x - LABEL_LAND_X / 2.0 + 10.0, y, LABEL_LAND_Z + 2.0);
        lands = lands + label + fiducial;
    }
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("closed_gasket_station_rfid_disc_land_{i}"),
                15.0,
                5.0,
                32,
            )
            .translate(
                82.0 + (i % 2) as f64 * 54.0,
                78.0 - (i / 2) as f64 * 50.0,
                2.5,
            );
    }
    for i in 0..COA_LANDS {
        let coa = centered_cube(
            format!("closed_gasket_station_coa_document_land_{i}"),
            142.0,
            34.0,
            5.0,
        )
        .translate(
            -70.0 + (i % 2) as f64 * 156.0,
            -118.0 - (i / 2) as f64 * 42.0,
            2.5,
        );
        let clip = centered_cube(
            format!("closed_gasket_station_coa_retention_clip_{i}"),
            118.0,
            6.0,
            16.0,
        )
        .translate(
            -70.0 + (i % 2) as f64 * 156.0,
            -100.0 - (i / 2) as f64 * 42.0,
            8.0,
        );
        lands = lands + coa + clip;
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "closed_gasket_station_release_hold_reject_lane_tray",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let basin = centered_cube(
        "closed_gasket_station_status_lane_basin",
        STATUS_X - 34.0,
        STATUS_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, STATUS_Z - 4.0);
    tray - basin - status_slot_cuts() + status_lane_dividers() + status_label_lands()
}

fn status_slot_cuts() -> Part {
    let mut cuts = Part::empty("closed_gasket_station_status_slot_cuts");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("closed_gasket_station_status_lane_{lane}_slot_{slot}"),
                    52.0,
                    24.0,
                    STATUS_Z + 4.0,
                )
                .translate(x, -70.0 + slot as f64 * 35.0, STATUS_Z / 2.0 + 3.0);
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let release_hold = centered_cube(
        "closed_gasket_station_release_hold_divider",
        9.0,
        STATUS_Y - 42.0,
        28.0,
    )
    .translate(-STATUS_LANE_PITCH / 2.0, 0.0, STATUS_Z + 14.0);
    let hold_reject = centered_cube(
        "closed_gasket_station_hold_reject_divider",
        9.0,
        STATUS_Y - 42.0,
        28.0,
    )
    .translate(STATUS_LANE_PITCH / 2.0, 0.0, STATUS_Z + 14.0);
    let rear_stop = centered_cube(
        "closed_gasket_station_status_lane_rear_hard_stop",
        STATUS_X - 54.0,
        9.0,
        26.0,
    )
    .translate(0.0, STATUS_Y / 2.0 - 24.0, STATUS_Z + 13.0);
    release_hold + hold_reject + rear_stop
}

fn status_label_lands() -> Part {
    let released = centered_cube(
        "closed_gasket_station_released_lane_label_land",
        72.0,
        20.0,
        5.0,
    )
    .translate(status_lane_x(0), STATUS_Y / 2.0 - 50.0, STATUS_Z + 2.5);
    let hold = centered_cube(
        "closed_gasket_station_hold_lane_label_land",
        72.0,
        20.0,
        5.0,
    )
    .translate(status_lane_x(1), STATUS_Y / 2.0 - 50.0, STATUS_Z + 2.5);
    let reject = centered_cube(
        "closed_gasket_station_reject_lane_label_land",
        72.0,
        20.0,
        5.0,
    )
    .translate(status_lane_x(2), STATUS_Y / 2.0 - 50.0, STATUS_Z + 2.5);
    released + hold + reject
}

fn sample_retain_pockets() -> Part {
    let block = centered_cube(
        "closed_gasket_station_sample_retain_block",
        RETAIN_BLOCK_X,
        RETAIN_BLOCK_Y,
        RETAIN_BLOCK_Z,
    )
    .translate(0.0, 0.0, RETAIN_BLOCK_Z / 2.0);
    let lid_recess = centered_cube(
        "closed_gasket_station_retain_lid_recess",
        RETAIN_BLOCK_X - 34.0,
        RETAIN_BLOCK_Y - 30.0,
        9.0,
    )
    .translate(0.0, 0.0, RETAIN_BLOCK_Z - 4.0);
    block - lid_recess - retain_pocket_cuts() + retain_label_lands() + retain_tamper_bridge()
}

fn retain_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_gasket_station_retain_pocket_cuts");
    for i in 0..RETAIN_POCKETS {
        let col = i % RETAIN_COLS;
        let row = i / RETAIN_COLS;
        let x = -(RETAIN_COLS as f64 - 1.0) * RETAIN_PITCH_X / 2.0 + col as f64 * RETAIN_PITCH_X;
        let y = -(RETAIN_ROWS as f64 - 1.0) * RETAIN_PITCH_Y / 2.0 + row as f64 * RETAIN_PITCH_Y;
        cuts = cuts
            + centered_cylinder(
                format!("closed_gasket_station_retain_gasket_pocket_{i}"),
                14.0,
                RETAIN_BLOCK_Z + 4.0,
                32,
            )
            .translate(x, y, RETAIN_BLOCK_Z / 2.0 + 3.0);
    }
    cuts
}

fn retain_label_lands() -> Part {
    let front = centered_cube(
        "closed_gasket_station_retain_chain_of_custody_label_land",
        RETAIN_BLOCK_X - 84.0,
        20.0,
        5.0,
    )
    .translate(0.0, -RETAIN_BLOCK_Y / 2.0 + 22.0, RETAIN_BLOCK_Z + 2.5);
    let rear = centered_cube(
        "closed_gasket_station_retain_expiry_lot_label_land",
        RETAIN_BLOCK_X - 118.0,
        18.0,
        5.0,
    )
    .translate(0.0, RETAIN_BLOCK_Y / 2.0 - 24.0, RETAIN_BLOCK_Z + 2.5);
    front + rear
}

fn retain_tamper_bridge() -> Part {
    centered_cube(
        "closed_gasket_station_retain_tamper_evidence_bridge",
        RETAIN_BLOCK_X - 70.0,
        10.0,
        24.0,
    )
    .translate(0.0, RETAIN_BLOCK_Y / 2.0 - 44.0, RETAIN_BLOCK_Z + 12.0)
}

fn clean_used_segregation() -> Part {
    let spine = centered_cube(
        "closed_gasket_station_clean_used_status_segregation_spine",
        SEGREGATION_SPINE_X,
        SEGREGATION_SPINE_Y,
        SEGREGATION_SPINE_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + 156.0,
        DECK_Z / 2.0 + SEGREGATION_SPINE_Z / 2.0,
    );
    let clean_lane = centered_cube(
        "closed_gasket_station_clean_incoming_lane_land",
        CLEAN_LANE_X,
        34.0,
        10.0,
    )
    .translate(-272.0, -BASE_Y / 2.0 + 104.0, DECK_Z / 2.0 + 5.0);
    let used_lane = centered_cube(
        "closed_gasket_station_used_tooling_lane_land",
        USED_LANE_X,
        34.0,
        10.0,
    )
    .translate(122.0, -BASE_Y / 2.0 + 104.0, DECK_Z / 2.0 + 5.0);
    let reject_bin = centered_cube(
        "closed_gasket_station_reject_quarantine_bin_land",
        REJECT_BIN_X,
        72.0,
        28.0,
    )
    .translate(452.0, -BASE_Y / 2.0 + 92.0, DECK_Z / 2.0 + 14.0);
    spine + clean_lane + used_lane + reject_bin + segregation_witness_posts()
}

fn segregation_witness_posts() -> Part {
    let mut posts = Part::empty("closed_gasket_station_segregation_witness_posts");
    for (i, x) in [-512.0, -256.0, 0.0, 256.0, 512.0].iter().enumerate() {
        posts = posts
            + centered_cylinder(
                format!("closed_gasket_station_segregation_post_{i}"),
                5.0,
                46.0,
                20,
            )
            .translate(*x, -BASE_Y / 2.0 + 156.0, DECK_Z / 2.0 + 23.0);
    }
    posts
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_gasket_station_front_robot_keepout_volume",
        BASE_X - 130.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0,
        ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let rear_service = centered_cube(
        "closed_gasket_station_rear_service_keepout_volume",
        BASE_X - 150.0,
        REAR_SERVICE_KEEP_OUT_Y,
        GAUGE_SERVICE_CLEARANCE_Z,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0,
        GAUGE_SERVICE_CLEARANCE_Z / 2.0,
    );
    let left_cart = centered_cube(
        "closed_gasket_station_left_lot_cart_keepout_volume",
        LEFT_LOT_CART_KEEP_OUT_X,
        BASE_Y - 160.0,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        -BASE_X / 2.0 - LEFT_LOT_CART_KEEP_OUT_X / 2.0,
        0.0,
        ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let right_gauge = centered_cube(
        "closed_gasket_station_right_gauge_service_keepout_volume",
        RIGHT_GAUGE_SERVICE_KEEP_OUT_X,
        BASE_Y - 120.0,
        GAUGE_SERVICE_CLEARANCE_Z,
    )
    .translate(
        BASE_X / 2.0 + RIGHT_GAUGE_SERVICE_KEEP_OUT_X / 2.0,
        30.0,
        GAUGE_SERVICE_CLEARANCE_Z / 2.0,
    );
    front_robot + rear_service + left_cart + right_gauge
}

fn tooling_datums() -> Part {
    let mut datums = Part::empty("closed_gasket_station_tooling_datums");
    for (i, (x, y)) in tooling_pin_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_gasket_station_tooling_datum_boss_{i}"),
            8.0,
            10.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        let socket = centered_cylinder(
            format!("closed_gasket_station_tooling_datum_pin_socket_{i}"),
            3.2 / 2.0,
            12.0,
            18,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 5.0);
        datums = datums + (boss - socket);
    }
    for i in 0..TOOLING_RAILS {
        let rail = centered_cube(
            format!("closed_gasket_station_tooling_slide_rail_{i}"),
            134.0,
            8.0,
            12.0,
        )
        .translate(-432.0 + i as f64 * 284.0, 322.0, DECK_Z / 2.0 + 6.0);
        datums = datums + rail;
    }
    datums
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 44.0), -(BASE_Y / 2.0 - 44.0)),
        (BASE_X / 2.0 - 44.0, -(BASE_Y / 2.0 - 44.0)),
        (-(BASE_X / 2.0 - 44.0), BASE_Y / 2.0 - 44.0),
        (BASE_X / 2.0 - 44.0, BASE_Y / 2.0 - 44.0),
        (0.0, -(BASE_Y / 2.0 - 44.0)),
        (0.0, BASE_Y / 2.0 - 44.0),
        (-(BASE_X / 2.0 - 44.0), 0.0),
        (BASE_X / 2.0 - 44.0, 0.0),
    ]
}

fn tooling_pin_points() -> [(f64, f64); TOOL_DATUM_PINS] {
    [
        (
            LOT_CENTER_X - LOT_TRAY_X / 2.0 + 28.0,
            LOT_CENTER_Y - LOT_TRAY_Y / 2.0 + 28.0,
        ),
        (
            LOT_CENTER_X + LOT_TRAY_X / 2.0 - 28.0,
            LOT_CENTER_Y + LOT_TRAY_Y / 2.0 - 28.0,
        ),
        (
            THICKNESS_CENTER_X - THICKNESS_BLOCK_X / 2.0 + 28.0,
            THICKNESS_CENTER_Y - THICKNESS_BLOCK_Y / 2.0 + 28.0,
        ),
        (
            THICKNESS_CENTER_X + THICKNESS_BLOCK_X / 2.0 - 28.0,
            THICKNESS_CENTER_Y + THICKNESS_BLOCK_Y / 2.0 - 28.0,
        ),
        (
            COMPRESSION_CENTER_X - COMPRESSION_BLOCK_X / 2.0 + 28.0,
            COMPRESSION_CENTER_Y - COMPRESSION_BLOCK_Y / 2.0 + 28.0,
        ),
        (
            COMPRESSION_CENTER_X + COMPRESSION_BLOCK_X / 2.0 - 28.0,
            COMPRESSION_CENTER_Y + COMPRESSION_BLOCK_Y / 2.0 - 28.0,
        ),
        (
            STATUS_CENTER_X - STATUS_X / 2.0 + 28.0,
            STATUS_CENTER_Y - STATUS_Y / 2.0 + 28.0,
        ),
        (
            RETAIN_CENTER_X + RETAIN_BLOCK_X / 2.0 - 28.0,
            RETAIN_CENTER_Y + RETAIN_BLOCK_Y / 2.0 - 28.0,
        ),
    ]
}

fn lot_position(index: usize) -> (f64, f64) {
    let col = index % LOT_TRAY_COLS;
    let row = index / LOT_TRAY_COLS;
    (
        -(LOT_TRAY_COLS as f64 - 1.0) * LOT_PITCH_X / 2.0 + col as f64 * LOT_PITCH_X,
        -(LOT_TRAY_ROWS as f64 - 1.0) * LOT_PITCH_Y / 2.0 + row as f64 * LOT_PITCH_Y,
    )
}

fn status_lane_x(lane: usize) -> f64 {
    (lane as f64 - 1.0) * STATUS_LANE_PITCH
}

fn lot_rect() -> Rect {
    Rect::new(
        "lot trays",
        LOT_CENTER_X,
        LOT_CENTER_Y,
        LOT_TRAY_X,
        LOT_TRAY_Y,
    )
}

fn thickness_rect() -> Rect {
    Rect::new(
        "thickness gauge dock",
        THICKNESS_CENTER_X,
        THICKNESS_CENTER_Y,
        THICKNESS_BLOCK_X,
        THICKNESS_BLOCK_Y,
    )
}

fn compression_rect() -> Rect {
    Rect::new(
        "compression gauge dock",
        COMPRESSION_CENTER_X,
        COMPRESSION_CENTER_Y,
        COMPRESSION_BLOCK_X,
        COMPRESSION_BLOCK_Y,
    )
}

fn imaging_rect() -> Rect {
    Rect::new(
        "defect imaging stage",
        IMAGING_CENTER_X,
        IMAGING_CENTER_Y,
        IMAGING_WINDOW_X,
        IMAGING_WINDOW_Y,
    )
}

fn traceability_rect() -> Rect {
    Rect::new(
        "barcode RFID COA lands",
        TRACEABILITY_CENTER_X,
        TRACEABILITY_CENTER_Y,
        360.0,
        224.0,
    )
}

fn status_rect() -> Rect {
    Rect::new(
        "status lanes",
        STATUS_CENTER_X,
        STATUS_CENTER_Y,
        STATUS_X,
        STATUS_Y,
    )
}

fn retain_rect() -> Rect {
    Rect::new(
        "sample retain pockets",
        RETAIN_CENTER_X,
        RETAIN_CENTER_Y,
        RETAIN_BLOCK_X,
        RETAIN_BLOCK_Y,
    )
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    cx: f64,
    cy: f64,
    x: f64,
    y: f64,
}

impl Rect {
    fn new(name: &'static str, cx: f64, cy: f64, x: f64, y: f64) -> Self {
        Self { name, cx, cy, x, y }
    }

    fn left(self) -> f64 {
        self.cx - self.x / 2.0
    }

    fn right(self) -> f64 {
        self.cx + self.x / 2.0
    }

    fn bottom(self) -> f64 {
        self.cy - self.y / 2.0
    }

    fn top(self) -> f64 {
        self.cy + self.y / 2.0
    }

    fn fits_inside(self, max_x: f64, max_y: f64) -> bool {
        self.left() >= -max_x / 2.0
            && self.right() <= max_x / 2.0
            && self.bottom() >= -max_y / 2.0
            && self.top() <= max_y / 2.0
    }

    fn overlaps(self, other: Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.bottom() < other.top()
            && self.top() > other.bottom()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_plan_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| {
            path.starts_with("output/closed_gasket_lot_incoming_inspection_station_")
                && path.ends_with(".stl")
        }));
    }

    #[test]
    fn station_geometry_constants_match_incoming_inspection_intent() {
        assert!(BASE_X >= 1200.0);
        assert!(BASE_Y >= 800.0);
        assert_eq!(LOT_TRAY_COUNT, LOT_TRAY_COLS * LOT_TRAY_ROWS);
        assert_eq!(RETAIN_POCKETS, RETAIN_COLS * RETAIN_ROWS);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, 5);
        assert!(THICKNESS_GAUGE_ENV_Z > THICKNESS_BLOCK_Z * 3.0);
        assert!(COMPRESSION_GAUGE_ENV_Z > COMPRESSION_BLOCK_Z * 4.0);
    }

    #[test]
    fn traceability_and_retain_capacity_are_explicit() {
        assert!(BARCODE_LANDS >= LOT_TRAY_COUNT);
        assert_eq!(RFID_LANDS, LOT_TRAY_COUNT);
        assert!(COA_LANDS >= 4);
        assert!(RETAIN_POCKETS >= LOT_TRAY_COUNT * 2);
        assert!(DEFECT_REFERENCE_COUPONS >= 10);
    }

    #[test]
    fn purchased_tool_envelopes_and_keepouts_clear_workflow() {
        assert!(THICKNESS_GAUGE_ENV_X < THICKNESS_BLOCK_X);
        assert!(COMPRESSION_GAUGE_ENV_X < COMPRESSION_BLOCK_X);
        assert!(CAMERA_BRIDGE_SPAN_X > IMAGING_WINDOW_X);
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z > LOT_TRAY_Z + 120.0);
        assert!(GAUGE_SERVICE_CLEARANCE_Z > COMPRESSION_GAUGE_ENV_Z + 30.0);
    }

    #[test]
    fn layout_rectangles_fit_and_core_modules_do_not_overlap() {
        assert_layout();
        assert!(traceability_rect().right() < retain_rect().left());
        assert!(retain_rect().right() < status_rect().left());
        assert!(lot_rect().right() < compression_rect().left());
    }
}
