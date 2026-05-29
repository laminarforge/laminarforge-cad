use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterile-filter integrity and vent-management qualification station.
//
// Intent:
// - Package purchased sterile filter cartridges, hydrophobic vent filters,
//   low-pressure sources, and electronic readers into one qualification deck.
// - Keep dry gas handling physically separated from wet/bubble-point handoff
//   features, with leak containment, condensate capture, clean/used flow lanes,
//   released/hold/reject disposition lanes, and traceability lands.
// - Represent service/robot access envelopes and transfer interfaces for culture
//   modules without modeling pressure-rated internals or prescribing a validated
//   integrity-test method.

const OUTPUTS: [&str; 12] = [
    "output/closed_sterile_filter_vent_integrity_station_leak_tray_base.stl",
    "output/closed_sterile_filter_vent_integrity_station_filter_cartridge_nest_matrix.stl",
    "output/closed_sterile_filter_vent_integrity_station_vent_filter_docks.stl",
    "output/closed_sterile_filter_vent_integrity_station_pressure_decay_handoff_ports.stl",
    "output/closed_sterile_filter_vent_integrity_station_bubble_point_wet_handoff.stl",
    "output/closed_sterile_filter_vent_integrity_station_wet_dry_segregation_barrier.stl",
    "output/closed_sterile_filter_vent_integrity_station_condensate_trap_pockets.stl",
    "output/closed_sterile_filter_vent_integrity_station_traceability_lands.stl",
    "output/closed_sterile_filter_vent_integrity_station_disposition_lanes.stl",
    "output/closed_sterile_filter_vent_integrity_station_clean_used_segregation.stl",
    "output/closed_sterile_filter_vent_integrity_station_robot_service_keepouts.stl",
    "output/closed_sterile_filter_vent_integrity_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "filter_cartridge_nest_matrix",
    "vent_filter_docks",
    "pressure_decay_handoff_ports",
    "bubble_point_wet_handoff_ports",
    "wet_dry_side_segregation",
    "condensate_trap_pockets",
    "barcode_rfid_certificate_lands",
    "released_hold_reject_lanes",
    "leak_tray",
    "clean_used_segregation",
    "robot_keepouts",
    "service_keepouts",
    "assembly_export",
];

const FILTER_ROWS: usize = 3;
const FILTER_COLS: usize = 4;
const FILTER_NEST_COUNT: usize = FILTER_ROWS * FILTER_COLS;
const VENT_FILTER_DOCKS: usize = 8;
const PRESSURE_DECAY_PORTS: usize = FILTER_NEST_COUNT;
const BUBBLE_POINT_PORTS: usize = FILTER_NEST_COUNT;
const CONDENSATE_TRAPS: usize = FILTER_ROWS + 1;
const TRACEABILITY_LANDS: usize = FILTER_NEST_COUNT + 6;
const DISPOSITION_LANES: usize = 3;
const ROBOT_KEEP_OUT_ZONES: usize = 4;

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 20.0;
const LEAK_BASIN_X: f64 = 1410.0;
const LEAK_BASIN_Y: f64 = 850.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const TRAY_CURB_W: f64 = 18.0;
const TRAY_CURB_Z: f64 = 42.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const DRAIN_PORT_D: f64 = 16.0;

const FILTER_NEST_X: f64 = 132.0;
const FILTER_NEST_Y: f64 = 74.0;
const FILTER_NEST_Z: f64 = 34.0;
const FILTER_SOCKET_X: f64 = 104.0;
const FILTER_SOCKET_Y: f64 = 44.0;
const FILTER_SOCKET_DEPTH: f64 = 13.0;
const FILTER_PITCH_X: f64 = 172.0;
const FILTER_PITCH_Y: f64 = 124.0;
const FILTER_MATRIX_X: f64 = FILTER_COLS as f64 * FILTER_PITCH_X + 46.0;
const FILTER_MATRIX_Y: f64 = FILTER_ROWS as f64 * FILTER_PITCH_Y + 60.0;
const FILTER_MATRIX_Z: f64 = 46.0;
const FILTER_MATRIX_CENTER_X: f64 = -250.0;
const FILTER_MATRIX_CENTER_Y: f64 = 34.0;

const VENT_DOCK_X: f64 = 620.0;
const VENT_DOCK_Y: f64 = 150.0;
const VENT_DOCK_Z: f64 = 58.0;
const VENT_FILTER_OD: f64 = 32.0;
const VENT_FILTER_LENGTH: f64 = 78.0;

const PRESSURE_BAR_X: f64 = 1110.0;
const PRESSURE_BAR_Y: f64 = 112.0;
const PRESSURE_BAR_Z: f64 = 52.0;
const PRESSURE_PORT_D: f64 = 8.0;
const GAUGE_POCKET_X: f64 = 56.0;
const GAUGE_POCKET_Y: f64 = 38.0;
const GAUGE_POCKET_Z: f64 = 18.0;

const WET_HANDOFF_X: f64 = 1040.0;
const WET_HANDOFF_Y: f64 = 164.0;
const WET_HANDOFF_Z: f64 = 58.0;
const WET_CUP_D: f64 = 30.0;
const WET_CUP_DEPTH: f64 = 26.0;

const SEG_BARRIER_X: f64 = 1340.0;
const SEG_BARRIER_Y: f64 = 26.0;
const SEG_BARRIER_Z: f64 = 118.0;
const DRY_SIDE_Y: f64 = 244.0;
const WET_SIDE_Y: f64 = -246.0;

const TRAP_BANK_X: f64 = 610.0;
const TRAP_BANK_Y: f64 = 116.0;
const TRAP_BANK_Z: f64 = 54.0;
const TRAP_POCKET_D: f64 = 42.0;

const TRACE_PANEL_X: f64 = 600.0;
const TRACE_PANEL_Y: f64 = 128.0;
const TRACE_PANEL_Z: f64 = 14.0;
const BARCODE_LAND_X: f64 = 76.0;
const BARCODE_LAND_Y: f64 = 38.0;
const RFID_LAND_D: f64 = 44.0;
const CERT_LAND_X: f64 = 146.0;
const CERT_LAND_Y: f64 = 48.0;

const LANE_X: f64 = 318.0;
const LANE_Y: f64 = 248.0;
const LANE_Z: f64 = 38.0;
const LANE_PITCH_X: f64 = 360.0;
const LANE_DIVIDER_W: f64 = 14.0;
const LANE_SOCKET_COUNT: usize = 4;

const CLEAN_USED_PANEL_X: f64 = 1380.0;
const CLEAN_USED_PANEL_Y: f64 = 86.0;
const CLEAN_USED_PANEL_Z: f64 = 44.0;
const CLEAN_USED_DIVIDER_Z: f64 = 92.0;
const BAG_DOCK_COUNT: usize = 6;

const ROBOT_KEEP_OUT_X: f64 = DECK_X + 250.0;
const ROBOT_KEEP_OUT_Y: f64 = DECK_Y + 220.0;
const ROBOT_KEEP_OUT_Z: f64 = 330.0;
const SERVICE_KEEP_OUT_X: f64 = 390.0;
const SERVICE_KEEP_OUT_Y: f64 = 280.0;
const SERVICE_KEEP_OUT_Z: f64 = 220.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let filter_matrix = filter_cartridge_nest_matrix();
    export(OUTPUTS[1], &filter_matrix);

    let vent_docks = vent_filter_docks();
    export(OUTPUTS[2], &vent_docks);

    let pressure = pressure_decay_handoff_ports();
    export(OUTPUTS[3], &pressure);

    let wet_handoff = bubble_point_wet_handoff();
    export(OUTPUTS[4], &wet_handoff);

    let barrier = wet_dry_segregation_barrier();
    export(OUTPUTS[5], &barrier);

    let traps = condensate_trap_pockets();
    export(OUTPUTS[6], &traps);

    let trace = traceability_lands();
    export(OUTPUTS[7], &trace);

    let lanes = disposition_lanes();
    export(OUTPUTS[8], &lanes);

    let clean_used = clean_used_segregation();
    export(OUTPUTS[9], &clean_used);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + filter_matrix.translate(
            FILTER_MATRIX_CENTER_X,
            FILTER_MATRIX_CENTER_Y,
            DECK_Z + FILTER_MATRIX_Z / 2.0,
        ) + vent_docks.translate(390.0, DRY_SIDE_Y, DECK_Z + VENT_DOCK_Z / 2.0)
            + pressure.translate(0.0, DRY_SIDE_Y - 128.0, DECK_Z + PRESSURE_BAR_Z / 2.0)
            + wet_handoff.translate(0.0, WET_SIDE_Y, DECK_Z + WET_HANDOFF_Z / 2.0)
            + barrier.translate(0.0, 0.0, DECK_Z + SEG_BARRIER_Z / 2.0)
            + traps.translate(398.0, WET_SIDE_Y + 170.0, DECK_Z + TRAP_BANK_Z / 2.0)
            + trace.translate(-446.0, -DECK_Y / 2.0 + 92.0, DECK_Z + TRACE_PANEL_Z / 2.0)
            + lanes.translate(0.0, -DECK_Y / 2.0 + 214.0, DECK_Z + LANE_Z / 2.0)
            + clean_used.translate(0.0, DECK_Y / 2.0 - 88.0, DECK_Z + CLEAN_USED_PANEL_Z / 2.0)
            + keepouts.translate(0.0, 0.0, DECK_Z + ROBOT_KEEP_OUT_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed sterile-filter vent integrity station:");
    println!("  Deck/leak tray:              {DECK_X:.0}mm x {DECK_Y:.0}mm with cleanable basin and drain");
    println!(
        "  Filter nests:                {FILTER_NEST_COUNT} cartridge positions in a {FILTER_ROWS}x{FILTER_COLS} matrix"
    );
    println!(
        "  Vent management:             {VENT_FILTER_DOCKS} hydrophobic vent-filter docks plus {CONDENSATE_TRAPS} condensate trap pockets"
    );
    println!(
        "  Integrity handoff:           {PRESSURE_DECAY_PORTS} dry pressure-decay ports and {BUBBLE_POINT_PORTS} wet bubble-point handoff cups"
    );
    println!(
        "  Traceability/disposition:    {TRACEABILITY_LANDS} barcode/RFID/certificate lands and {DISPOSITION_LANES} released/hold/reject lanes"
    );
    println!(
        "  Segregation:                 dry side y={DRY_SIDE_Y:.0}mm, wet side y={WET_SIDE_Y:.0}mm, clean/used transfer strip"
    );
    println!(
        "  Keepouts:                    robot {ROBOT_KEEP_OUT_X:.0}mm x {ROBOT_KEEP_OUT_Y:.0}mm x {ROBOT_KEEP_OUT_Z:.0}mm plus {ROBOT_KEEP_OUT_ZONES} service/robot zones"
    );
    println!("  Required feature groups:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(FILTER_NEST_COUNT, 12);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(TRACEABILITY_LANDS, 18);
    assert!(FILTER_NEST_Z > FILTER_SOCKET_DEPTH + 18.0);
    assert!(filter_matrix_span_x() < DECK_X - 520.0);
    assert!(filter_matrix_span_y() < DECK_Y - 390.0);
    assert!(dry_wet_gap() >= 450.0);
    assert!(VENT_FILTER_DOCKS >= FILTER_ROWS * 2);
    assert!(CONDENSATE_TRAPS >= FILTER_ROWS + 1);
    assert!(ROBOT_KEEP_OUT_X > DECK_X);
    assert!(ROBOT_KEEP_OUT_Y > DECK_Y);
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "closed_filter_integrity_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "closed_filter_integrity_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - LEAK_BASIN_DEPTH / 2.0);
    let drain_channel = centered_cube(
        "closed_filter_integrity_front_drain_channel",
        LEAK_BASIN_X - 120.0,
        DRAIN_CHANNEL_W,
        LEAK_BASIN_DEPTH + 4.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 54.0, DECK_Z - LEAK_BASIN_DEPTH / 2.0);
    let drain_port = centered_cylinder(
        "closed_filter_integrity_leak_tray_bulkhead_drain",
        DRAIN_PORT_D / 2.0,
        TRAY_CURB_W + 20.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 24.0, DECK_Z - 8.0);

    deck - basin - drain_channel - drain_port + tray_curbs() + deck_mount_features()
}

fn tray_curbs() -> Part {
    let left = centered_cube(
        "closed_filter_integrity_left_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 46.0,
        TRAY_CURB_Z,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_CURB_W / 2.0,
        0.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let right = centered_cube(
        "closed_filter_integrity_right_spill_curb",
        TRAY_CURB_W,
        DECK_Y - 46.0,
        TRAY_CURB_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_CURB_W / 2.0,
        0.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_filter_integrity_rear_spill_curb",
        DECK_X - 36.0,
        TRAY_CURB_W,
        TRAY_CURB_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z / 2.0,
    );
    let front_lip = centered_cube(
        "closed_filter_integrity_front_low_spill_lip",
        DECK_X - 190.0,
        TRAY_CURB_W,
        TRAY_CURB_Z * 0.62,
    )
    .translate(
        -50.0,
        -DECK_Y / 2.0 + TRAY_CURB_W / 2.0,
        DECK_Z + TRAY_CURB_Z * 0.31,
    );

    left + right + rear + front_lip
}

fn deck_mount_features() -> Part {
    let mut features = Part::empty("closed_filter_integrity_deck_mount_features");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let bore = centered_cylinder(
            format!("closed_filter_integrity_m6_mount_bore_{i}"),
            6.6 / 2.0,
            DECK_Z + 8.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let counterbore = centered_cylinder(
            format!("closed_filter_integrity_m6_counterbore_{i}"),
            13.0 / 2.0,
            5.0,
            28,
        )
        .translate(*x, *y, DECK_Z - 2.0);
        features = features + bore + counterbore;
    }

    let dry_datum = centered_cube(
        "closed_filter_integrity_dry_side_datum_rail",
        DECK_X - 190.0,
        8.0,
        14.0,
    )
    .translate(0.0, DRY_SIDE_Y + 116.0, DECK_Z + 7.0);
    let wet_datum = centered_cube(
        "closed_filter_integrity_wet_side_datum_rail",
        DECK_X - 190.0,
        8.0,
        14.0,
    )
    .translate(0.0, WET_SIDE_Y - 116.0, DECK_Z + 7.0);

    dry_datum + wet_datum - features
}

fn filter_cartridge_nest_matrix() -> Part {
    let backing = centered_cube(
        "closed_filter_integrity_filter_nest_matrix_backing_plate",
        FILTER_MATRIX_X,
        FILTER_MATRIX_Y,
        FILTER_MATRIX_Z,
    );
    let mut sockets = Part::empty("closed_filter_integrity_filter_cartridge_socket_cutouts");
    let mut rails = Part::empty("closed_filter_integrity_filter_cartridge_soft_capture_rails");
    let mut latch_tabs = Part::empty("closed_filter_integrity_filter_cartridge_latch_tabs");

    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let x = filter_col_x(col);
            let y = filter_row_y(row);
            let socket = centered_cube(
                format!("closed_filter_integrity_cartridge_socket_{idx}"),
                FILTER_SOCKET_X,
                FILTER_SOCKET_Y,
                FILTER_SOCKET_DEPTH,
            )
            .translate(
                x,
                y,
                FILTER_MATRIX_Z / 2.0 - FILTER_SOCKET_DEPTH / 2.0 + 1.0,
            );
            let inlet_keepaway = centered_cylinder(
                format!("closed_filter_integrity_cartridge_luer_clearance_a_{idx}"),
                8.0 / 2.0,
                FILTER_NEST_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - FILTER_SOCKET_X / 2.0 - 12.0, y, 4.0);
            let outlet_keepaway = centered_cylinder(
                format!("closed_filter_integrity_cartridge_luer_clearance_b_{idx}"),
                8.0 / 2.0,
                FILTER_NEST_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x + FILTER_SOCKET_X / 2.0 + 12.0, y, 4.0);
            sockets = sockets + socket + inlet_keepaway + outlet_keepaway;

            let left_rail = centered_cube(
                format!("closed_filter_integrity_cartridge_left_capture_rail_{idx}"),
                FILTER_NEST_X,
                7.0,
                15.0,
            )
            .translate(x, y - FILTER_NEST_Y / 2.0, FILTER_MATRIX_Z / 2.0 + 7.5);
            let right_rail = centered_cube(
                format!("closed_filter_integrity_cartridge_right_capture_rail_{idx}"),
                FILTER_NEST_X,
                7.0,
                15.0,
            )
            .translate(x, y + FILTER_NEST_Y / 2.0, FILTER_MATRIX_Z / 2.0 + 7.5);
            rails = rails + left_rail + right_rail;

            let latch = centered_cube(
                format!("closed_filter_integrity_cartridge_release_latch_land_{idx}"),
                34.0,
                12.0,
                18.0,
            )
            .translate(
                x + FILTER_NEST_X / 2.0 - 22.0,
                y,
                FILTER_MATRIX_Z / 2.0 + 9.0,
            );
            latch_tabs = latch_tabs + latch;
        }
    }

    backing - sockets + rails + latch_tabs + row_lane_markers()
}

fn row_lane_markers() -> Part {
    let mut markers = Part::empty("closed_filter_integrity_filter_row_lane_markers");
    for row in 0..FILTER_ROWS {
        let y = filter_row_y(row);
        let marker = centered_cube(
            format!("closed_filter_integrity_filter_row_status_strip_{row}"),
            FILTER_MATRIX_X - 48.0,
            5.0,
            9.0,
        )
        .translate(
            0.0,
            y + FILTER_PITCH_Y / 2.0 - 24.0,
            FILTER_MATRIX_Z / 2.0 + 4.5,
        );
        markers = markers + marker;
    }
    markers
}

fn vent_filter_docks() -> Part {
    let bank = centered_cube(
        "closed_filter_integrity_vent_filter_dock_bank",
        VENT_DOCK_X,
        VENT_DOCK_Y,
        VENT_DOCK_Z,
    );
    let mut cutouts = Part::empty("closed_filter_integrity_vent_filter_dock_cutouts");
    let mut clips = Part::empty("closed_filter_integrity_vent_filter_retention_clips");

    for i in 0..VENT_FILTER_DOCKS {
        let x = -VENT_DOCK_X / 2.0 + 72.0 + i as f64 * ((VENT_DOCK_X - 144.0) / 7.0);
        let bore = centered_cylinder(
            format!("closed_filter_integrity_hydrophobic_vent_filter_body_clearance_{i}"),
            VENT_FILTER_OD / 2.0,
            VENT_FILTER_LENGTH,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 6.0);
        let dock_slot = centered_cube(
            format!("closed_filter_integrity_vent_filter_flat_slot_{i}"),
            VENT_FILTER_OD + 8.0,
            VENT_FILTER_LENGTH + 18.0,
            16.0,
        )
        .translate(x, 0.0, 4.0);
        cutouts = cutouts + bore + dock_slot;

        let clip_a = centered_cube(
            format!("closed_filter_integrity_vent_filter_clip_front_{i}"),
            VENT_FILTER_OD + 16.0,
            6.0,
            24.0,
        )
        .translate(
            x,
            -VENT_FILTER_LENGTH / 2.0 - 10.0,
            VENT_DOCK_Z / 2.0 + 12.0,
        );
        let clip_b = centered_cube(
            format!("closed_filter_integrity_vent_filter_clip_rear_{i}"),
            VENT_FILTER_OD + 16.0,
            6.0,
            24.0,
        )
        .translate(x, VENT_FILTER_LENGTH / 2.0 + 10.0, VENT_DOCK_Z / 2.0 + 12.0);
        clips = clips + clip_a + clip_b;
    }

    bank - cutouts + clips + vent_bank_manifold_keepaways()
}

fn vent_bank_manifold_keepaways() -> Part {
    let dry_header = centered_cylinder(
        "closed_filter_integrity_dry_vent_header_keepaway",
        9.0 / 2.0,
        VENT_DOCK_X - 68.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, VENT_DOCK_Y / 2.0 - 26.0, 6.0);
    let exhaust_header = centered_cylinder(
        "closed_filter_integrity_exhaust_vent_header_keepaway",
        9.0 / 2.0,
        VENT_DOCK_X - 68.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -VENT_DOCK_Y / 2.0 + 26.0, 6.0);
    Part::empty("closed_filter_integrity_vent_header_keepaway_solid") - dry_header - exhaust_header
}

fn pressure_decay_handoff_ports() -> Part {
    let bar = centered_cube(
        "closed_filter_integrity_pressure_decay_reader_bar",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    );
    let mut ports = Part::empty("closed_filter_integrity_pressure_decay_port_cutouts");
    let mut readers = Part::empty("closed_filter_integrity_pressure_reader_pockets");

    for i in 0..PRESSURE_DECAY_PORTS {
        let col = i % FILTER_COLS;
        let row = i / FILTER_COLS;
        let x = -PRESSURE_BAR_X / 2.0 + 104.0 + col as f64 * 154.0 + row as f64 * 30.0;
        let y = if row % 2 == 0 { -22.0 } else { 22.0 };
        let port = centered_cylinder(
            format!("closed_filter_integrity_pressure_decay_quick_connect_bore_{i}"),
            PRESSURE_PORT_D / 2.0,
            PRESSURE_BAR_Z + 8.0,
            28,
        )
        .translate(x, y, 0.0);
        let reader = centered_cube(
            format!("closed_filter_integrity_pressure_reader_pocket_{i}"),
            GAUGE_POCKET_X,
            GAUGE_POCKET_Y,
            GAUGE_POCKET_Z,
        )
        .translate(x, -PRESSURE_BAR_Y / 2.0 + 26.0, PRESSURE_BAR_Z / 2.0 - 8.0);
        ports = ports + port;
        readers = readers + reader;
    }

    let source_envelope = centered_cube(
        "closed_filter_integrity_purchased_low_pressure_source_envelope",
        208.0,
        84.0,
        86.0,
    )
    .translate(
        PRESSURE_BAR_X / 2.0 - 136.0,
        0.0,
        PRESSURE_BAR_Z / 2.0 + 43.0,
    );
    let regulator_land = centered_cube(
        "closed_filter_integrity_regulator_and_reference_reader_land",
        190.0,
        24.0,
        42.0,
    )
    .translate(
        -PRESSURE_BAR_X / 2.0 + 126.0,
        PRESSURE_BAR_Y / 2.0 + 12.0,
        21.0,
    );

    bar - ports - readers + source_envelope + regulator_land
}

fn bubble_point_wet_handoff() -> Part {
    let tray = centered_cube(
        "closed_filter_integrity_bubble_point_wet_handoff_tray",
        WET_HANDOFF_X,
        WET_HANDOFF_Y,
        WET_HANDOFF_Z,
    );
    let mut cups = Part::empty("closed_filter_integrity_wet_handoff_cup_cutouts");
    let mut splash_shields = Part::empty("closed_filter_integrity_bubble_point_splash_shields");

    for i in 0..BUBBLE_POINT_PORTS {
        let col = i % FILTER_COLS;
        let row = i / FILTER_COLS;
        let x = -WET_HANDOFF_X / 2.0 + 94.0 + col as f64 * 154.0 + row as f64 * 26.0;
        let y = if row % 2 == 0 { -34.0 } else { 34.0 };
        let cup = centered_cylinder(
            format!("closed_filter_integrity_bubble_point_capture_cup_{i}"),
            WET_CUP_D / 2.0,
            WET_CUP_DEPTH,
            36,
        )
        .translate(x, y, WET_HANDOFF_Z / 2.0 - WET_CUP_DEPTH / 2.0 + 1.0);
        let drain = centered_cylinder(
            format!("closed_filter_integrity_bubble_point_cup_drain_{i}"),
            5.0 / 2.0,
            WET_HANDOFF_Z + 5.0,
            20,
        )
        .translate(x, y, 0.0);
        cups = cups + cup + drain;

        let shield = centered_cube(
            format!("closed_filter_integrity_bubble_point_local_splash_shield_{i}"),
            WET_CUP_D + 18.0,
            4.0,
            34.0,
        )
        .translate(x, y + 25.0, WET_HANDOFF_Z / 2.0 + 17.0);
        splash_shields = splash_shields + shield;
    }

    let wet_reader_land = centered_cube(
        "closed_filter_integrity_wet_side_reader_and_wit_adapter_land",
        238.0,
        82.0,
        28.0,
    )
    .translate(WET_HANDOFF_X / 2.0 - 144.0, 0.0, WET_HANDOFF_Z / 2.0 + 14.0);

    tray - cups + splash_shields + wet_reader_land
}

fn wet_dry_segregation_barrier() -> Part {
    let wall = centered_cube(
        "closed_filter_integrity_wet_dry_hard_segregation_wall",
        SEG_BARRIER_X,
        SEG_BARRIER_Y,
        SEG_BARRIER_Z,
    );
    let pass_through_count = FILTER_ROWS;
    let mut pass_throughs = Part::empty("closed_filter_integrity_controlled_handoff_windows");
    for i in 0..pass_through_count {
        let x = -SEG_BARRIER_X / 2.0 + 250.0 + i as f64 * 270.0;
        let window = centered_cube(
            format!("closed_filter_integrity_controlled_tube_handoff_window_{i}"),
            120.0,
            SEG_BARRIER_Y + 4.0,
            42.0,
        )
        .translate(x, 0.0, 6.0);
        pass_throughs = pass_throughs + window;
    }
    let signage = centered_cube(
        "closed_filter_integrity_dry_wet_side_status_signage_lands",
        SEG_BARRIER_X - 120.0,
        5.0,
        26.0,
    )
    .translate(0.0, -SEG_BARRIER_Y / 2.0 - 3.0, SEG_BARRIER_Z / 2.0 + 13.0);

    wall - pass_throughs + signage
}

fn condensate_trap_pockets() -> Part {
    let bank = centered_cube(
        "closed_filter_integrity_condensate_trap_bank",
        TRAP_BANK_X,
        TRAP_BANK_Y,
        TRAP_BANK_Z,
    );
    let mut pockets = Part::empty("closed_filter_integrity_condensate_trap_pocket_cutouts");
    let mut witness_lands = Part::empty("closed_filter_integrity_condensate_witness_lands");
    for i in 0..CONDENSATE_TRAPS {
        let x = -TRAP_BANK_X / 2.0 + 76.0 + i as f64 * ((TRAP_BANK_X - 152.0) / 3.0);
        let pocket = centered_cylinder(
            format!("closed_filter_integrity_condensate_trap_round_pocket_{i}"),
            TRAP_POCKET_D / 2.0,
            TRAP_BANK_Z + 4.0,
            36,
        )
        .translate(x, 0.0, 7.0);
        let drain_slot = centered_cube(
            format!("closed_filter_integrity_condensate_trap_drain_slot_{i}"),
            TRAP_POCKET_D * 0.56,
            TRAP_BANK_Y + 4.0,
            8.0,
        )
        .translate(x, 0.0, -TRAP_BANK_Z / 2.0 + 9.0);
        pockets = pockets + pocket + drain_slot;

        let witness = centered_cube(
            format!("closed_filter_integrity_condensate_visual_witness_land_{i}"),
            52.0,
            20.0,
            10.0,
        )
        .translate(x, -TRAP_BANK_Y / 2.0 - 10.0, TRAP_BANK_Z / 2.0 + 5.0);
        witness_lands = witness_lands + witness;
    }

    bank - pockets + witness_lands
}

fn traceability_lands() -> Part {
    let panel = centered_cube(
        "closed_filter_integrity_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_filter_integrity_barcode_rfid_certificate_lands");

    for i in 0..FILTER_NEST_COUNT {
        let col = i % 6;
        let row = i / 6;
        let land = centered_cube(
            format!("closed_filter_integrity_filter_lot_barcode_land_{i}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            4.0,
        )
        .translate(
            -TRACE_PANEL_X / 2.0 + 60.0 + col as f64 * 90.0,
            -TRACE_PANEL_Y / 2.0 + 32.0 + row as f64 * 48.0,
            TRACE_PANEL_Z / 2.0 + 2.0,
        );
        lands = lands + land;
    }

    for i in 0..4 {
        let rfid = centered_cylinder(
            format!("closed_filter_integrity_rfid_puck_land_{i}"),
            RFID_LAND_D / 2.0,
            5.0,
            32,
        )
        .translate(
            -TRACE_PANEL_X / 2.0 + 72.0 + i as f64 * 92.0,
            48.0,
            TRACE_PANEL_Z / 2.0 + 2.5,
        );
        lands = lands + rfid;
    }

    for i in 0..2 {
        let cert = centered_cube(
            format!("closed_filter_integrity_certificate_scan_land_{i}"),
            CERT_LAND_X,
            CERT_LAND_Y,
            5.0,
        )
        .translate(
            TRACE_PANEL_X / 2.0 - 92.0,
            -34.0 + i as f64 * 68.0,
            TRACE_PANEL_Z / 2.0 + 2.5,
        );
        lands = lands + cert;
    }

    panel + lands
}

fn disposition_lanes() -> Part {
    let mut lanes = Part::empty("closed_filter_integrity_released_hold_reject_lanes");
    for lane in 0..DISPOSITION_LANES {
        let x = -LANE_PITCH_X + lane as f64 * LANE_PITCH_X;
        let tray = centered_cube(
            format!("closed_filter_integrity_disposition_lane_tray_{lane}"),
            LANE_X,
            LANE_Y,
            LANE_Z,
        )
        .translate(x, 0.0, 0.0);
        let recesses = lane_recesses(lane).translate(x, 0.0, 0.0);
        let dividers = centered_cube(
            format!("closed_filter_integrity_disposition_lane_side_divider_{lane}"),
            LANE_X + 28.0,
            LANE_DIVIDER_W,
            LANE_Z + 24.0,
        )
        .translate(x, LANE_Y / 2.0 + LANE_DIVIDER_W / 2.0, 12.0);
        let placard = centered_cube(
            format!("closed_filter_integrity_disposition_lane_status_placard_{lane}"),
            LANE_X - 44.0,
            8.0,
            44.0,
        )
        .translate(x, -LANE_Y / 2.0 - 10.0, LANE_Z / 2.0 + 22.0);
        lanes = lanes + (tray - recesses) + dividers + placard;
    }
    lanes
}

fn lane_recesses(lane: usize) -> Part {
    let mut recesses = Part::empty(format!(
        "closed_filter_integrity_disposition_lane_recesses_{lane}"
    ));
    for i in 0..LANE_SOCKET_COUNT {
        let socket = centered_cube(
            format!("closed_filter_integrity_disposition_lane_{lane}_socket_{i}"),
            54.0,
            78.0,
            12.0,
        )
        .translate(
            -LANE_X / 2.0 + 52.0 + i as f64 * 70.0,
            18.0,
            LANE_Z / 2.0 - 5.0,
        );
        recesses = recesses + socket;
    }
    recesses
}

fn clean_used_segregation() -> Part {
    let strip = centered_cube(
        "closed_filter_integrity_clean_used_transfer_strip",
        CLEAN_USED_PANEL_X,
        CLEAN_USED_PANEL_Y,
        CLEAN_USED_PANEL_Z,
    );
    let divider = centered_cube(
        "closed_filter_integrity_clean_used_hard_divider",
        CLEAN_USED_PANEL_X - 80.0,
        12.0,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(
        0.0,
        0.0,
        CLEAN_USED_PANEL_Z / 2.0 + CLEAN_USED_DIVIDER_Z / 2.0,
    );
    let mut docks = Part::empty("closed_filter_integrity_clean_used_bag_dock_cutouts");
    for i in 0..BAG_DOCK_COUNT {
        let x = -CLEAN_USED_PANEL_X / 2.0 + 135.0 + i as f64 * 224.0;
        let clean = centered_cube(
            format!("closed_filter_integrity_clean_bag_dock_cutout_{i}"),
            112.0,
            28.0,
            18.0,
        )
        .translate(x, CLEAN_USED_PANEL_Y / 4.0, CLEAN_USED_PANEL_Z / 2.0 - 7.0);
        let used = centered_cube(
            format!("closed_filter_integrity_used_bag_dock_cutout_{i}"),
            112.0,
            28.0,
            18.0,
        )
        .translate(x, -CLEAN_USED_PANEL_Y / 4.0, CLEAN_USED_PANEL_Z / 2.0 - 7.0);
        docks = docks + clean + used;
    }

    strip - docks + divider
}

fn robot_service_keepouts() -> Part {
    let robot = centered_cube(
        "closed_filter_integrity_robot_access_keepout_envelope",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    );
    let front_service = centered_cube(
        "closed_filter_integrity_front_reader_service_keepout",
        DECK_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 - SERVICE_KEEP_OUT_Y / 2.0, -55.0);
    let rear_service = centered_cube(
        "closed_filter_integrity_rear_filter_loading_service_keepout",
        DECK_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 + SERVICE_KEEP_OUT_Y / 2.0, -55.0);
    let left_service = centered_cube(
        "closed_filter_integrity_left_wet_side_service_keepout",
        SERVICE_KEEP_OUT_X,
        DECK_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0, 0.0, -55.0);
    let right_service = centered_cube(
        "closed_filter_integrity_right_pressure_source_service_keepout",
        SERVICE_KEEP_OUT_X,
        DECK_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 + SERVICE_KEEP_OUT_X / 2.0, 0.0, -55.0);

    robot + front_service + rear_service + left_service + right_service
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 88.0, -DECK_Y / 2.0 + 82.0),
        (DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 82.0),
        (-DECK_X / 2.0 + 88.0, DECK_Y / 2.0 - 82.0),
        (DECK_X / 2.0 - 88.0, DECK_Y / 2.0 - 82.0),
        (-DECK_X / 2.0 + 88.0, 0.0),
        (DECK_X / 2.0 - 88.0, 0.0),
        (-160.0, DECK_Y / 2.0 - 82.0),
        (160.0, DECK_Y / 2.0 - 82.0),
    ]
}

fn filter_col_x(col: usize) -> f64 {
    -((FILTER_COLS - 1) as f64 * FILTER_PITCH_X) / 2.0 + col as f64 * FILTER_PITCH_X
}

fn filter_row_y(row: usize) -> f64 {
    -((FILTER_ROWS - 1) as f64 * FILTER_PITCH_Y) / 2.0 + row as f64 * FILTER_PITCH_Y
}

fn filter_matrix_span_x() -> f64 {
    (FILTER_COLS - 1) as f64 * FILTER_PITCH_X + FILTER_NEST_X
}

fn filter_matrix_span_y() -> f64 {
    (FILTER_ROWS - 1) as f64 * FILTER_PITCH_Y + FILTER_NEST_Y
}

fn dry_wet_gap() -> f64 {
    DRY_SIDE_Y - WET_SIDE_Y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_has_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS
            .last()
            .unwrap()
            .ends_with("closed_sterile_filter_vent_integrity_station_assembly.stl"));
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_sterile_filter_vent_integrity_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn filter_and_port_counts_match_matrix() {
        assert_eq!(FILTER_ROWS, 3);
        assert_eq!(FILTER_COLS, 4);
        assert_eq!(FILTER_NEST_COUNT, 12);
        assert_eq!(PRESSURE_DECAY_PORTS, FILTER_NEST_COUNT);
        assert_eq!(BUBBLE_POINT_PORTS, FILTER_NEST_COUNT);
        assert_eq!(VENT_FILTER_DOCKS, 8);
    }

    #[test]
    fn geometry_constants_leave_deck_margin() {
        assert_eq!(DECK_X, 1540.0);
        assert_eq!(DECK_Y, 980.0);
        assert!(filter_matrix_span_x() < DECK_X - 520.0);
        assert!(filter_matrix_span_y() < DECK_Y - 390.0);
        assert!(LEAK_BASIN_X < DECK_X - 2.0 * TRAY_CURB_W);
        assert!(LEAK_BASIN_Y < DECK_Y - 2.0 * TRAY_CURB_W);
    }

    #[test]
    fn wet_dry_and_clean_used_segregation_are_explicit() {
        assert!(DRY_SIDE_Y > 0.0);
        assert!(WET_SIDE_Y < 0.0);
        assert!(dry_wet_gap() >= 450.0);
        assert!(SEG_BARRIER_Z > WET_HANDOFF_Z);
        assert_eq!(BAG_DOCK_COUNT, 6);
        assert!(CLEAN_USED_DIVIDER_Z > CLEAN_USED_PANEL_Z);
    }

    #[test]
    fn traceability_and_disposition_capacity_are_sized() {
        assert_eq!(TRACEABILITY_LANDS, FILTER_NEST_COUNT + 6);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(LANE_SOCKET_COUNT * DISPOSITION_LANES, FILTER_NEST_COUNT);
        assert!(CERT_LAND_X > BARCODE_LAND_X);
        assert!(RFID_LAND_D > 40.0);
    }

    #[test]
    fn keepouts_exceed_operating_footprint() {
        assert_eq!(ROBOT_KEEP_OUT_ZONES, 4);
        assert!(ROBOT_KEEP_OUT_X > DECK_X);
        assert!(ROBOT_KEEP_OUT_Y > DECK_Y);
        assert!(ROBOT_KEEP_OUT_Z > SEG_BARRIER_Z + 180.0);
        assert!(SERVICE_KEEP_OUT_X >= 390.0);
        assert!(SERVICE_KEEP_OUT_Y >= 280.0);
    }
}
